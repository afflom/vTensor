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
    C35(u64),
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
                                                            let a496 = match v2.clone() {
                                                                Adt2::C0(h495, _, _, _, _, _, _, _, _, _, _) => {
                                                                    h495
                                                                }
                                                            };
                                                            let a497 = 0u64;
                                                            nat_eq(a496, a497)
                                                        } else {
                                                            false
                                                        } {
                                                            let a499 = match v4.clone() {
                                                                Adt2::C0(h498, _, _, _, _, _, _, _, _, _, _) => {
                                                                    h498
                                                                }
                                                            };
                                                            let a500 = 26u64;
                                                            nat_eq(a499, a500)
                                                        } else {
                                                            false
                                                        } {
                                                            let m501 = v3;
                                                            match m501 {
                                                                None => {
                                                                    Adt10::C1
                                                                }
                                                                Some(v9) => {
                                                                    f53(v2, v9, v4)?
                                                                }
                                                            }
                                                        } else {
                                                            let a502 = v0;
                                                            let a503 = 33u64;
                                                            if if if nat_eq(a502, a503) {
                                                                let a505 = match v4.clone() {
                                                                    Adt2::C0(h504, _, _, _, _, _, _, _, _, _, _) => {
                                                                        h504
                                                                    }
                                                                };
                                                                let a506 = 0u64;
                                                                nat_eq(a505, a506)
                                                            } else {
                                                                false
                                                            } {
                                                                f54(match v2.clone() {
                                                                    Adt2::C0(h507, _, _, _, _, _, _, _, _, _, _) => {
                                                                        h507
                                                                    }
                                                                })
                                                            } else {
                                                                false
                                                            } {
                                                                f55(v2, v3, v4)?
                                                            } else {
                                                                f56(v0, v2, {
                                                                    let m508 = v3;
                                                                    match m508 {
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
                                                        let a509 = v0;
                                                        let a510 = 19u64;
                                                        if nat_eq(a509, a510) {
                                                            f57(v2, v4)?
                                                        } else {
                                                            let a511 = v0;
                                                            let a512 = 21u64;
                                                            if nat_eq(a511, a512) {
                                                                f58(v5, v2, v3, v4)?
                                                            } else {
                                                                let a513 = v0;
                                                                let a514 = 61u64;
                                                                if if if nat_eq(a513, a514) {
                                                                    true
                                                                } else {
                                                                    let a515 = v0;
                                                                    let a516 = 62u64;
                                                                    nat_eq(a515, a516)
                                                                } {
                                                                    true
                                                                } else {
                                                                    let a517 = v0;
                                                                    let a518 = 63u64;
                                                                    nat_eq(a517, a518)
                                                                } {
                                                                    f59(v0, v1, v2, v4)?
                                                                } else {
                                                                    let a519 = v0;
                                                                    let a520 = 4u64;
                                                                    if if nat_eq(a519, a520) {
                                                                        true
                                                                    } else {
                                                                        let a521 = v0;
                                                                        let a522 = 32u64;
                                                                        nat_eq(a521, a522)
                                                                    } {
                                                                        f60(v0, v1, v2, v3, v4)?
                                                                    } else {
                                                                        let a523 = v0;
                                                                        let a524 = 20u64;
                                                                        if nat_eq(a523, a524) {
                                                                            f61(v2, v4)?
                                                                        } else {
                                                                            let a525 = v0;
                                                                            let a526 = 42u64;
                                                                            if nat_eq(a525, a526) {
                                                                                f62(4u64, v2, v4)?
                                                                            } else {
                                                                                let a527 = v0;
                                                                                let a528 = 69u64;
                                                                                if nat_eq(a527, a528) {
                                                                                    let a529 = v5;
                                                                                    let a530 = 4u64;
                                                                                    if nat_lt(a529, a530) {
                                                                                        f62(v5, v2, v4)?
                                                                                    } else {
                                                                                        Adt10::C1
                                                                                    }
                                                                                } else {
                                                                                    let a531 = v0;
                                                                                    let a532 = 70u64;
                                                                                    if nat_eq(a531, a532) {
                                                                                        f63(v5, v2, v4)?
                                                                                    } else {
                                                                                        let a533 = v0;
                                                                                        let a534 = 75u64;
                                                                                        if if nat_eq(a533, a534) {
                                                                                            true
                                                                                        } else {
                                                                                            let a535 = v0;
                                                                                            let a536 = 76u64;
                                                                                            nat_eq(a535, a536)
                                                                                        } {
                                                                                            f64({
                                                                                                let a537 = v0;
                                                                                                let a538 = 76u64;
                                                                                                nat_eq(a537, a538)
                                                                                            }, v1, v2, v4)?
                                                                                        } else {
                                                                                            let a539 = v0;
                                                                                            let a540 = 31u64;
                                                                                            if if if nat_eq(a539, a540) {
                                                                                                true
                                                                                            } else {
                                                                                                let a541 = v0;
                                                                                                let a542 = 47u64;
                                                                                                nat_eq(a541, a542)
                                                                                            } {
                                                                                                true
                                                                                            } else {
                                                                                                let a543 = v0;
                                                                                                let a544 = 68u64;
                                                                                                nat_eq(a543, a544)
                                                                                            } {
                                                                                                f65(v0, v1, v2, v4)?
                                                                                            } else {
                                                                                                let a545 = v0;
                                                                                                let a546 = 15u64;
                                                                                                if nat_eq(a545, a546) {
                                                                                                    f66(v2, v4)?
                                                                                                } else {
                                                                                                    let a547 = v0;
                                                                                                    let a548 = 17u64;
                                                                                                    if nat_eq(a547, a548) {
                                                                                                        f67(v2, v4)?
                                                                                                    } else {
                                                                                                        let a549 = v0;
                                                                                                        let a550 = 18u64;
                                                                                                        if nat_eq(a549, a550) {
                                                                                                            f68(v2, v3, v4)?
                                                                                                        } else {
                                                                                                            let a551 = v0;
                                                                                                            let a552 = 13u64;
                                                                                                            if if if nat_eq(a551, a552) {
                                                                                                                true
                                                                                                            } else {
                                                                                                                let a553 = v0;
                                                                                                                let a554 = 14u64;
                                                                                                                nat_eq(a553, a554)
                                                                                                            } {
                                                                                                                true
                                                                                                            } else {
                                                                                                                let a555 = v0;
                                                                                                                let a556 = 16u64;
                                                                                                                nat_eq(a555, a556)
                                                                                                            } {
                                                                                                                f69(v0, v2, v4)?
                                                                                                            } else {
                                                                                                                let m557 = f70(v0, v5)?;
                                                                                                                match m557 {
                                                                                                                    None => {
                                                                                                                        f71(v0, v5, v2, v3, v4)?
                                                                                                                    }
                                                                                                                    Some(v11) => {
                                                                                                                        f72(v11, v2, v4)?
                                                                                                                    }
                                                                                                                }
                                                                                                            }
                                                                                                        }
                                                                                                    }
                                                                                                }
                                                                                            }
                                                                                        }
                                                                                    }
                                                                                }
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
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
        Adt8::C0(_, h558, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h558
        }
    };
    let v5: u64 = match v0.clone() {
        Adt8::C0(_, _, h559, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h559
        }
    };
    let v6: u64 = {
        let a560 = v3;
        let a561 = v5;
        let a562 = 0u64;
        nat_rem(a560, a561, a562)
    };
    let a563 = v1;
    let a564 = 0u64;
    if nat_eq(a563, a564) {
        let a566 = v3;
        let a567 = match v0.clone() {
            Adt8::C0(_, _, _, h565, _, _, _, _, _, _, _, _, _, _, _) => {
                h565
            }
        };
        if nat_lt(a566, a567) {
            Ok(Adt9::C8(Some(f73(match v0.clone() {
                Adt8::C0(_, _, _, _, _, _, _, _, h568, _, _, _, _, _, _) => {
                    h568
                }
            }, {
                let a572 = v6;
                let a573 = {
                    let m571 = match v0.clone() {
                        Adt8::C0(_, _, _, _, _, _, _, _, h570, _, _, _, _, _, _) => {
                            h570
                        }
                    };
                    match m571 {
                        Adt3::C0(h569, _, _, _, _, _, _, _) => {
                            h569
                        }
                    }
                };
                let a574 = 0u64;
                nat_rem(a572, a573, a574)
            }, {
                let a584 = {
                    let a578 = v6;
                    let a579 = {
                        let m577 = match v0.clone() {
                            Adt8::C0(_, _, _, _, _, _, _, _, h576, _, _, _, _, _, _) => {
                                h576
                            }
                        };
                        match m577 {
                            Adt3::C0(h575, _, _, _, _, _, _, _) => {
                                h575
                            }
                        }
                    };
                    let a580 = 0u64;
                    nat_quot(a578, a579, a580)
                };
                let a585 = {
                    let m583 = match v0.clone() {
                        Adt8::C0(_, _, _, _, _, _, _, _, h582, _, _, _, _, _, _) => {
                            h582
                        }
                    };
                    match m583 {
                        Adt3::C0(_, h581, _, _, _, _, _, _) => {
                            h581
                        }
                    }
                };
                let a586 = 0u64;
                nat_rem(a584, a585, a586)
            }, {
                let a596 = {
                    let a590 = v6;
                    let a591 = {
                        let m589 = match v0.clone() {
                            Adt8::C0(_, _, _, _, _, _, _, _, h588, _, _, _, _, _, _) => {
                                h588
                            }
                        };
                        match m589 {
                            Adt3::C0(h587, _, _, _, _, _, _, _) => {
                                h587
                            }
                        }
                    };
                    let a592 = 0u64;
                    nat_quot(a590, a591, a592)
                };
                let a597 = {
                    let m595 = match v0 {
                        Adt8::C0(_, _, _, _, _, _, _, _, h594, _, _, _, _, _, _) => {
                            h594
                        }
                    };
                    match m595 {
                        Adt3::C0(_, h593, _, _, _, _, _, _) => {
                            h593
                        }
                    }
                };
                let a598 = 0u64;
                nat_quot(a596, a597, a598)
            }, {
                let a599 = v3;
                let a600 = v5;
                let a601 = 0u64;
                nat_quot(a599, a600, a601)
            })?)))
        } else {
            Ok(Adt9::C0(Adt10::C1))
        }
    } else {
        let v7: u64 = {
            let a607 = v2;
            let a608 = {
                let a604 = v3;
                let a605 = {
                    let a602 = v1;
                    let a603 = 1u64;
                    if nat_eq(a602, a603) {
                        v4
                    } else {
                        v5
                    }
                };
                let a606 = 0u64;
                nat_quot(a604, a605, a606)
            };
            nat_add(a607, a608)?
        };
        let a610 = v7;
        let a611 = match v0.clone() {
            Adt8::C0(h609, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h609
            }
        };
        if nat_lt(a610, a611) {
            let v8: u64 = {
                let a615 = v7;
                let a616 = {
                    let m614 = match v0.clone() {
                        Adt8::C0(_, _, _, _, _, _, _, _, _, h613, _, _, _, _, _) => {
                            h613
                        }
                    };
                    match m614 {
                        Adt3::C0(h612, _, _, _, _, _, _, _) => {
                            h612
                        }
                    }
                };
                let a617 = 0u64;
                nat_rem(a615, a616, a617)
            };
            let v9: u64 = {
                let a621 = v7;
                let a622 = {
                    let m620 = match v0.clone() {
                        Adt8::C0(_, _, _, _, _, _, _, _, _, h619, _, _, _, _, _) => {
                            h619
                        }
                    };
                    match m620 {
                        Adt3::C0(h618, _, _, _, _, _, _, _) => {
                            h618
                        }
                    }
                };
                let a623 = 0u64;
                nat_quot(a621, a622, a623)
            };
            let v10: u64 = {
                let a627 = v9;
                let a628 = {
                    let m626 = match v0.clone() {
                        Adt8::C0(_, _, _, _, _, _, _, _, _, h625, _, _, _, _, _) => {
                            h625
                        }
                    };
                    match m626 {
                        Adt3::C0(_, h624, _, _, _, _, _, _) => {
                            h624
                        }
                    }
                };
                let a629 = 0u64;
                nat_rem(a627, a628, a629)
            };
            let v11: u64 = {
                let a633 = v9;
                let a634 = {
                    let m632 = match v0.clone() {
                        Adt8::C0(_, _, _, _, _, _, _, _, _, h631, _, _, _, _, _) => {
                            h631
                        }
                    };
                    match m632 {
                        Adt3::C0(_, h630, _, _, _, _, _, _) => {
                            h630
                        }
                    }
                };
                let a635 = 0u64;
                nat_quot(a633, a634, a635)
            };
            let a636 = v1;
            let a637 = 1u64;
            if nat_eq(a636, a637) {
                Ok(Adt9::C8(Some(f73(match v0 {
                    Adt8::C0(_, _, _, _, _, _, _, _, _, h638, _, _, _, _, _) => {
                        h638
                    }
                }, v8, v10, {
                    let a639 = v3;
                    let a640 = v4;
                    let a641 = 0u64;
                    nat_rem(a639, a640, a641)
                }, v11)?)))
            } else {
                let a642 = v1;
                let a643 = 2u64;
                if nat_eq(a642, a643) {
                    Ok(Adt9::C8(Some({
                        let a689 = f73(match v0.clone() {
                            Adt8::C0(_, _, _, _, _, _, _, _, _, _, h644, _, _, _, _) => {
                                h644
                            }
                        }, {
                            let a656 = {
                                let a648 = v8;
                                let a649 = {
                                    let m647 = match v0.clone() {
                                        Adt8::C0(_, _, _, _, _, _, _, _, _, _, _, h646, _, _, _) => {
                                            h646
                                        }
                                    };
                                    match m647 {
                                        Adt6::C0(_, h645, _, _) => {
                                            h645
                                        }
                                    }
                                };
                                nat_mul(a648, a649)?
                            };
                            let a657 = {
                                let a653 = v6;
                                let a654 = {
                                    let m652 = match v0.clone() {
                                        Adt8::C0(_, _, _, _, _, _, _, _, h651, _, _, _, _, _, _) => {
                                            h651
                                        }
                                    };
                                    match m652 {
                                        Adt3::C0(h650, _, _, _, _, _, _, _) => {
                                            h650
                                        }
                                    }
                                };
                                let a655 = 0u64;
                                nat_rem(a653, a654, a655)
                            };
                            nat_add(a656, a657)?
                        }, {
                            let a675 = {
                                let a661 = v10;
                                let a662 = {
                                    let m660 = match v0.clone() {
                                        Adt8::C0(_, _, _, _, _, _, _, _, _, _, _, h659, _, _, _) => {
                                            h659
                                        }
                                    };
                                    match m660 {
                                        Adt6::C0(_, h658, _, _) => {
                                            h658
                                        }
                                    }
                                };
                                nat_mul(a661, a662)?
                            };
                            let a676 = {
                                let a672 = {
                                    let a666 = v6;
                                    let a667 = {
                                        let m665 = match v0.clone() {
                                            Adt8::C0(_, _, _, _, _, _, _, _, h664, _, _, _, _, _, _) => {
                                                h664
                                            }
                                        };
                                        match m665 {
                                            Adt3::C0(h663, _, _, _, _, _, _, _) => {
                                                h663
                                            }
                                        }
                                    };
                                    let a668 = 0u64;
                                    nat_quot(a666, a667, a668)
                                };
                                let a673 = {
                                    let m671 = match v0.clone() {
                                        Adt8::C0(_, _, _, _, _, _, _, _, h670, _, _, _, _, _, _) => {
                                            h670
                                        }
                                    };
                                    match m671 {
                                        Adt3::C0(_, h669, _, _, _, _, _, _) => {
                                            h669
                                        }
                                    }
                                };
                                let a674 = 0u64;
                                nat_rem(a672, a673, a674)
                            };
                            nat_add(a675, a676)?
                        }, {
                            let a686 = {
                                let a680 = v6;
                                let a681 = {
                                    let m679 = match v0.clone() {
                                        Adt8::C0(_, _, _, _, _, _, _, _, h678, _, _, _, _, _, _) => {
                                            h678
                                        }
                                    };
                                    match m679 {
                                        Adt3::C0(h677, _, _, _, _, _, _, _) => {
                                            h677
                                        }
                                    }
                                };
                                let a682 = 0u64;
                                nat_quot(a680, a681, a682)
                            };
                            let a687 = {
                                let m685 = match v0 {
                                    Adt8::C0(_, _, _, _, _, _, _, _, h684, _, _, _, _, _, _) => {
                                        h684
                                    }
                                };
                                match m685 {
                                    Adt3::C0(_, h683, _, _, _, _, _, _) => {
                                        h683
                                    }
                                }
                            };
                            let a688 = 0u64;
                            nat_quot(a686, a687, a688)
                        }, v11)?;
                        let a690 = 4u64;
                        let a691 = 0u64;
                        nat_quot(a689, a690, a691)
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
    let a692 = v1;
    let a693 = 0u64;
    if nat_eq(a692, a693) {
        let a695 = v3;
        let a696 = match v0.clone() {
            Adt8::C0(_, _, _, h694, _, _, _, _, _, _, _, _, _, _, _) => {
                h694
            }
        };
        if nat_lt(a695, a696) {
            let v4: Adt3 = match v0.clone() {
                Adt8::C0(_, _, _, _, _, _, _, _, h697, _, _, _, _, _, _) => {
                    h697
                }
            };
            let v5: u64 = {
                let a699 = v3;
                let a700 = match v0.clone() {
                    Adt8::C0(_, _, h698, _, _, _, _, _, _, _, _, _, _, _, _) => {
                        h698
                    }
                };
                let a701 = 0u64;
                nat_quot(a699, a700, a701)
            };
            Ok(Adt9::C8(Some(f73(v4.clone(), f74(v5, v4.clone(), 0u64), f74(v5, v4.clone(), 1u64), f74(v5, v4, 2u64), f75(v0.clone(), v5, {
                let a703 = v3;
                let a704 = match v0 {
                    Adt8::C0(_, _, h702, _, _, _, _, _, _, _, _, _, _, _, _) => {
                        h702
                    }
                };
                let a705 = 0u64;
                nat_rem(a703, a704, a705)
            })?)?)))
        } else {
            Ok(Adt9::C0(Adt10::C1))
        }
    } else {
        let v6: Adt3 = match v0.clone() {
            Adt8::C0(_, _, _, _, _, _, _, _, _, _, h706, _, _, _, _) => {
                h706
            }
        };
        let v7: u64 = {
            let a707 = v1;
            let a708 = 1u64;
            if nat_eq(a707, a708) {
                match v0.clone() {
                    Adt8::C0(_, h709, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                        h709
                    }
                }
            } else {
                match v0.clone() {
                    Adt8::C0(_, _, h710, _, _, _, _, _, _, _, _, _, _, _, _) => {
                        h710
                    }
                }
            }
        };
        let v8: u64 = {
            let a714 = v2;
            let a715 = {
                let a711 = v3;
                let a712 = v7;
                let a713 = 0u64;
                nat_quot(a711, a712, a713)
            };
            nat_add(a714, a715)?
        };
        let v9: u64 = {
            let a716 = v3;
            let a717 = v7;
            let a718 = 0u64;
            nat_rem(a716, a717, a718)
        };
        let v10: u64 = {
            let a720 = v8;
            let a721 = match v6.clone() {
                Adt3::C0(h719, _, _, _, _, _, _, _) => {
                    h719
                }
            };
            let a722 = 0u64;
            nat_rem(a720, a721, a722)
        };
        let v11: u64 = {
            let a724 = v8;
            let a725 = match v6.clone() {
                Adt3::C0(h723, _, _, _, _, _, _, _) => {
                    h723
                }
            };
            let a726 = 0u64;
            nat_quot(a724, a725, a726)
        };
        let v12: u64 = {
            let a728 = v11;
            let a729 = match v6.clone() {
                Adt3::C0(_, h727, _, _, _, _, _, _) => {
                    h727
                }
            };
            let a730 = 0u64;
            nat_rem(a728, a729, a730)
        };
        let v13: u64 = {
            let a732 = v11;
            let a733 = match v6.clone() {
                Adt3::C0(_, h731, _, _, _, _, _, _) => {
                    h731
                }
            };
            let a734 = 0u64;
            nat_quot(a732, a733, a734)
        };
        let a736 = v8;
        let a737 = match v0.clone() {
            Adt8::C0(h735, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h735
            }
        };
        if nat_lt(a736, a737) {
            let a738 = v1;
            let a739 = 1u64;
            if nat_eq(a738, a739) {
                let v14: Adt3 = match v0.clone() {
                    Adt8::C0(_, _, _, _, _, _, _, _, h740, _, _, _, _, _, _) => {
                        h740
                    }
                };
                let v15: Adt3 = match v0.clone() {
                    Adt8::C0(_, _, _, _, _, _, _, _, _, h741, _, _, _, _, _) => {
                        h741
                    }
                };
                let m744 = f76(match v0.clone() {
                    Adt8::C0(_, _, _, _, _, _, _, _, _, _, _, h742, _, _, _) => {
                        h742
                    }
                }, match v15.clone() {
                    Adt3::C0(h743, _, _, _, _, _, _, _) => {
                        h743
                    }
                }, v10, f74(v9, v14.clone(), 0u64))?;
                match m744 {
                    None => {
                        Ok(Adt9::C8(None::<u64>))
                    }
                    Some(v16) => {
                        let m747 = f76(match v0.clone() {
                            Adt8::C0(_, _, _, _, _, _, _, _, _, _, _, _, h745, _, _) => {
                                h745
                            }
                        }, match v15 {
                            Adt3::C0(_, h746, _, _, _, _, _, _) => {
                                h746
                            }
                        }, v12, f74(v9, v14, 1u64))?;
                        match m747 {
                            None => {
                                Ok(Adt9::C8(None::<u64>))
                            }
                            Some(v17) => {
                                f77(v0, v9, v13, v16, v17)
                            }
                        }
                    }
                }
            } else {
                let a748 = v1;
                let a749 = 2u64;
                if nat_eq(a748, a749) {
                    Ok(Adt9::C8(Some({
                        let a751 = match v0.clone() {
                            Adt8::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, h750) => {
                                h750
                            }
                        };
                        let a752 = 0u64;
                        if nat_eq(a751, a752) {
                            f73(v6, v10, v12, v9, v13)?
                        } else {
                            f73(v6.clone(), v10, v12, {
                                let a754 = v13;
                                let a755 = match v6.clone() {
                                    Adt3::C0(_, _, h753, _, _, _, _, _) => {
                                        h753
                                    }
                                };
                                let a756 = 0u64;
                                nat_rem(a754, a755, a756)
                            }, {
                                let a764 = {
                                    let a762 = {
                                        let a758 = v13;
                                        let a759 = match v6 {
                                            Adt3::C0(_, _, h757, _, _, _, _, _) => {
                                                h757
                                            }
                                        };
                                        let a760 = 0u64;
                                        nat_quot(a758, a759, a760)
                                    };
                                    let a763 = match v0 {
                                        Adt8::C0(_, _, h761, _, _, _, _, _, _, _, _, _, _, _, _) => {
                                            h761
                                        }
                                    };
                                    nat_mul(a762, a763)?
                                };
                                let a765 = v9;
                                nat_add(a764, a765)?
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
    Ok(f78(v3.clone(), {
        let m766 = v3.clone();
        match m766 {
            Adt4::C0 => {
                f79(v0, v1, v2, v3)?
            }
            Adt4::C1 => {
                f79(v0, v1, v2, v3)?
            }
            Adt4::C2(_, _) => {
                f79(v0, v1, v2, v3)?
            }
            Adt4::C3(_, _, _, _, _) => {
                f79(v0, v1, v2, v3)?
            }
            Adt4::C4(_) => {
                f79(v0, v1, v2, v3)?
            }
            Adt4::C5(_, _, _, _) => {
                f79(v0, v1, v2, v3)?
            }
            Adt4::C6(_, _, _) => {
                f79(v0, v1, v2, v3)?
            }
            Adt4::C7(_) => {
                f79(v0, v1, v2, v3)?
            }
            Adt4::C8(_) => {
                f79(v0, v1, v2, v3)?
            }
            Adt4::C9(_, _) => {
                f79(v0, v1, v2, v3)?
            }
            Adt4::C10(_, _, _) => {
                f79(v0, v1, v2, v3)?
            }
            Adt4::C11(_, _) => {
                f79(v0, v1, v2, v3)?
            }
            Adt4::C12(_, _, _) => {
                f79(v0, v1, v2, v3)?
            }
            Adt4::C13(_, _, _, _) => {
                f79(v0, v1, v2, v3)?
            }
            Adt4::C14(_, _, _) => {
                f79(v0, v1, v2, v3)?
            }
            Adt4::C15 => {
                f79(v0, v1, v2, v3)?
            }
            Adt4::C16(_, _) => {
                f79(v0, v1, v2, v3)?
            }
            Adt4::C17 => {
                f85(v0, v1, v2, v4)?
            }
            Adt4::C18(_, _) => {
                f79(v0, v1, v2, v3)?
            }
            Adt4::C19(v58, v59) => {
                f86(v58, v0, v1, v2, v59, v4)?
            }
            Adt4::C20(v60, v61) => {
                f87(v60, v0, v1, v2, v61, v4)?
            }
            Adt4::C21 => {
                f79(v0, v1, v2, v3)?
            }
            Adt4::C22(v62, v63) => {
                f88(v0, v1, v2, v62, v63, v4)?
            }
            Adt4::C23(v77, v78, v79, v80) => {
                f95(v0, v80, v77, v78, v79, v4)?
            }
            Adt4::C24(v76) => {
                if v5 {
                    f93(Adt16::C0(v76), v0, v1, v2, v4, v6, v7, v8, v9, v10)?
                } else {
                    Adt9::C0(Adt10::C5(65u64))
                }
            }
            Adt4::C25(_, _, _) => {
                f79(v0, v1, v2, v3)?
            }
            Adt4::C26(v72, v73, v74, v75) => {
                let a768 = v72;
                let a769 = 64u64;
                if if if nat_eq(a768, a769) {
                    true
                } else {
                    let a770 = v72;
                    let a771 = 65u64;
                    nat_eq(a770, a771)
                } {
                    true
                } else {
                    let a772 = v72;
                    let a773 = 66u64;
                    nat_eq(a772, a773)
                } {
                    if v5 {
                        f93(Adt16::C1(v72, v73, v74, v75), v0, v1, v2, v4, v6, v7, v8, v9, v10)?
                    } else {
                        Adt9::C0(Adt10::C5(64u64))
                    }
                } else {
                    let a774 = v72;
                    let a775 = 0u64;
                    if if nat_eq(a774, a775) {
                        true
                    } else {
                        let a776 = v72;
                        let a777 = 30u64;
                        nat_eq(a776, a777)
                    } {
                        f94(v0, v1, v2, v72, v73, v74, v75, v4)?
                    } else {
                        Adt9::C0(Adt10::C1)
                    }
                }
            }
            Adt4::C27(v71) => {
                f92(v0, v1, v2, v71)?
            }
            Adt4::C28(v67) => {
                f89(v0, v1, v2, v67, v4)?
            }
            Adt4::C29(v68) => {
                f90(v0, v1, v2, v68)?
            }
            Adt4::C30(v69, _) => {
                f91(v0, v1, v2, v69)?
            }
            Adt4::C31(v33, v34) => {
                f80(v0, v1, v2, v33, v34)?
            }
            Adt4::C32(v35) => {
                f81(v0, match v1 {
                    Adt3::C0(h767, _, _, _, _, _, _, _) => {
                        h767
                    }
                }, v35)?
            }
            Adt4::C33(v36) => {
                f82(v0, v1, v36, v2)?
            }
            Adt4::C34(v38, v39, v40, v41) => {
                f84(v0, v1, v38, v2, v39, v40, v41)?
            }
            Adt4::C35(v37) => {
                f83(v0, v37, v1, v2)?
            }
        }
    }))
}

pub fn f14(v0: Adt4) -> R<Adt9> {
    let m778 = v0;
    match m778 {
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
        Adt4::C25(v54, _, v56) => {
            Ok(Adt9::C6({
                let a783 = {
                    let a779 = 0u64;
                    let a780 = v54;
                    if nat_lt(a779, a780) {
                        1u64
                    } else {
                        0u64
                    }
                };
                let a784 = {
                    let a781 = 0u64;
                    let a782 = v56;
                    if nat_lt(a781, a782) {
                        6u64
                    } else {
                        0u64
                    }
                };
                nat_add(a783, a784)?
            }, 0u64, 0u64, false))
        }
        Adt4::C26(v62, _, _, _) => {
            Ok(Adt9::C6({
                let a785 = v62;
                let a786 = 66u64;
                if nat_eq(a785, a786) {
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
    }
}

pub fn f15(v0: bool, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64, v7: u64, v8: u64, v9: u64, v10: u64, v11: u64, v12: u64) -> Adt9 {
    let a787 = v1;
    let a788 = 5u64;
    if if if if if nat_lt(a787, a788) {
        true
    } else {
        f96(v0, v1, v2, v3, v4)
    } {
        let a789 = v5;
        let a790 = 5u64;
        nat_lt(a789, a790)
    } else {
        false
    } {
        let a791 = v7;
        let a792 = 5u64;
        nat_lt(a791, a792)
    } else {
        false
    } {
        let a793 = v9;
        let a794 = 2u64;
        if nat_lt(a793, a794) {
            true
        } else {
            let a795 = v9;
            let a796 = 3u64;
            nat_eq(a795, a796)
        }
    } else {
        false
    } {
        Adt9::C7(f97(v0, v1, v2, v3, v4), f98(v5, v6), f98(v7, v8), f99({
            let a797 = v9;
            let a798 = 0u64;
            if nat_eq(a797, a798) {
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
    let a810 = {
        let a804 = {
            let a800 = match v0.clone() {
                Adt2::C0(_, h799, _, _, _, _, _, _, _, _, _) => {
                    h799
                }
            };
            let a801 = v1;
            let a802 = 0u64;
            nat_quot(a800, a801, a802)
        };
        let a805 = match v0.clone() {
            Adt2::C0(_, _, h803, _, _, _, _, _, _, _, _) => {
                h803
            }
        };
        nat_mul(a804, a805)?
    };
    let a811 = {
        let a808 = match v0.clone() {
            Adt2::C0(_, _, _, h806, _, _, _, _, _, _, _) => {
                h806
            }
        };
        let a809 = match v0 {
            Adt2::C0(_, _, _, _, h807, _, _, _, _, _, _) => {
                h807
            }
        };
        nat_mul(a808, a809)?
    };
    nat_mul(a810, a811)
}

pub fn f17(v0: u64) -> bool {
    let a812 = v0;
    let a813 = 4294967296u64;
    if nat_lt(a812, a813) {
        let a817 = {
            let a814 = v0;
            let a815 = 2147483648u64;
            let a816 = 0u64;
            nat_rem(a814, a815, a816)
        };
        let a818 = 2139095040u64;
        nat_lt(a817, a818)
    } else {
        false
    }
}

pub fn f18(v0: u64) -> R<u64> {
    let a819 = 4u64;
    let a820 = v0;
    nat_mul(a819, a820)
}

pub fn f19(v0: u64) -> R<u64> {
    let a821 = v0;
    let a822 = 0u64;
    if nat_eq(a821, a822) {
        Ok(0u64)
    } else {
        let a825 = 4u64;
        let a826 = {
            let a823 = 3u64;
            let a824 = v0;
            nat_mul(a823, a824)?
        };
        nat_add(a825, a826)
    }
}

pub fn f20(v0: Adt2, v1: u64) -> Adt3 {
    Adt3::C0({
        let a828 = match v0.clone() {
            Adt2::C0(_, h827, _, _, _, _, _, _, _, _, _) => {
                h827
            }
        };
        let a829 = v1;
        let a830 = 0u64;
        nat_quot(a828, a829, a830)
    }, match v0.clone() {
        Adt2::C0(_, _, h831, _, _, _, _, _, _, _, _) => {
            h831
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, h832, _, _, _, _, _, _, _) => {
            h832
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, h833, _, _, _, _, _, _) => {
            h833
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, _, h834, _, _, _, _, _) => {
            h834
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, _, _, h835, _, _, _, _) => {
            h835
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, _, _, _, h836, _, _, _) => {
            h836
        }
    }, match v0 {
        Adt2::C0(_, _, _, _, _, _, _, _, h837, _, _) => {
            h837
        }
    })
}

pub fn f21(v0: bool, v1: Adt2) -> Adt3 {
    if v0 {
        f20(v1, 1u64)
    } else {
        Adt3::C0(match v1.clone() {
            Adt2::C0(_, h838, _, _, _, _, _, _, _, _, _) => {
                h838
            }
        }, 1u64, match v1.clone() {
            Adt2::C0(_, _, h839, _, _, _, _, _, _, _, _) => {
                h839
            }
        }, match v1.clone() {
            Adt2::C0(_, _, _, h840, _, _, _, _, _, _, _) => {
                h840
            }
        }, match v1.clone() {
            Adt2::C0(_, _, _, _, _, h841, _, _, _, _, _) => {
                h841
            }
        }, 0u64, match v1.clone() {
            Adt2::C0(_, _, _, _, _, _, h842, _, _, _, _) => {
                h842
            }
        }, match v1 {
            Adt2::C0(_, _, _, _, _, _, _, h843, _, _, _) => {
                h843
            }
        })
    }
}

pub fn f22(v0: Adt2) -> R<bool> {
    let a845 = match v0.clone() {
        Adt2::C0(_, h844, _, _, _, _, _, _, _, _, _) => {
            h844
        }
    };
    let a846 = 1u64;
    if if if if nat_eq(a845, a846) {
        true
    } else {
        let a848 = match v0.clone() {
            Adt2::C0(_, _, _, _, _, h847, _, _, _, _, _) => {
                h847
            }
        };
        let a849 = 4u64;
        nat_eq(a848, a849)
    } {
        let a851 = match v0.clone() {
            Adt2::C0(_, _, h850, _, _, _, _, _, _, _, _) => {
                h850
            }
        };
        let a852 = 1u64;
        if nat_eq(a851, a852) {
            true
        } else {
            let a857 = match v0.clone() {
                Adt2::C0(_, _, _, _, _, _, h853, _, _, _, _) => {
                    h853
                }
            };
            let a858 = {
                let a855 = match v0.clone() {
                    Adt2::C0(_, h854, _, _, _, _, _, _, _, _, _) => {
                        h854
                    }
                };
                let a856 = 4u64;
                nat_mul(a855, a856)?
            };
            nat_eq(a857, a858)
        }
    } else {
        false
    } {
        let a860 = match v0.clone() {
            Adt2::C0(_, _, _, h859, _, _, _, _, _, _, _) => {
                h859
            }
        };
        let a861 = 1u64;
        if nat_eq(a860, a861) {
            true
        } else {
            let a869 = match v0.clone() {
                Adt2::C0(_, _, _, _, _, _, _, h862, _, _, _) => {
                    h862
                }
            };
            let a870 = {
                let a867 = {
                    let a865 = match v0.clone() {
                        Adt2::C0(_, h863, _, _, _, _, _, _, _, _, _) => {
                            h863
                        }
                    };
                    let a866 = match v0.clone() {
                        Adt2::C0(_, _, h864, _, _, _, _, _, _, _, _) => {
                            h864
                        }
                    };
                    nat_mul(a865, a866)?
                };
                let a868 = 4u64;
                nat_mul(a867, a868)?
            };
            nat_eq(a869, a870)
        }
    } else {
        false
    } {
        let a872 = match v0.clone() {
            Adt2::C0(_, _, _, _, h871, _, _, _, _, _, _) => {
                h871
            }
        };
        let a873 = 1u64;
        if nat_eq(a872, a873) {
            Ok(true)
        } else {
            let a884 = match v0.clone() {
                Adt2::C0(_, _, _, _, _, _, _, _, h874, _, _) => {
                    h874
                }
            };
            let a885 = {
                let a882 = {
                    let a880 = {
                        let a877 = match v0.clone() {
                            Adt2::C0(_, h875, _, _, _, _, _, _, _, _, _) => {
                                h875
                            }
                        };
                        let a878 = match v0.clone() {
                            Adt2::C0(_, _, h876, _, _, _, _, _, _, _, _) => {
                                h876
                            }
                        };
                        nat_mul(a877, a878)?
                    };
                    let a881 = match v0 {
                        Adt2::C0(_, _, _, h879, _, _, _, _, _, _, _) => {
                            h879
                        }
                    };
                    nat_mul(a880, a881)?
                };
                let a883 = 4u64;
                nat_mul(a882, a883)?
            };
            Ok(nat_eq(a884, a885))
        }
    } else {
        Ok(false)
    }
}

pub fn f23(v0: Adt2, v1: Adt2) -> R<bool> {
    let a887 = match v0.clone() {
        Adt2::C0(_, _, _, _, _, _, _, _, _, h886, _) => {
            h886
        }
    };
    let a888 = 0u64;
    if nat_eq(a887, a888) {
        Ok(true)
    } else {
        let a890 = match v1.clone() {
            Adt2::C0(_, _, _, _, _, _, _, _, _, h889, _) => {
                h889
            }
        };
        let a891 = 0u64;
        if nat_eq(a890, a891) {
            Ok(true)
        } else {
            let a893 = match v0.clone() {
                Adt2::C0(_, _, _, _, _, _, _, _, _, _, h892) => {
                    h892
                }
            };
            let a894 = 0u64;
            if nat_eq(a893, a894) {
                Ok(true)
            } else {
                let a896 = match v1.clone() {
                    Adt2::C0(_, _, _, _, _, _, _, _, _, _, h895) => {
                        h895
                    }
                };
                let a897 = 0u64;
                if nat_eq(a896, a897) {
                    Ok(true)
                } else {
                    let a903 = {
                        let a900 = match v0.clone() {
                            Adt2::C0(_, _, _, _, _, _, _, _, _, _, h898) => {
                                h898
                            }
                        };
                        let a901 = match v0.clone() {
                            Adt2::C0(_, _, _, _, _, _, _, _, _, h899, _) => {
                                h899
                            }
                        };
                        nat_add(a900, a901)?
                    };
                    let a904 = match v1.clone() {
                        Adt2::C0(_, _, _, _, _, _, _, _, _, _, h902) => {
                            h902
                        }
                    };
                    if nat_le(a903, a904) {
                        Ok(true)
                    } else {
                        let a910 = {
                            let a907 = match v1.clone() {
                                Adt2::C0(_, _, _, _, _, _, _, _, _, _, h905) => {
                                    h905
                                }
                            };
                            let a908 = match v1 {
                                Adt2::C0(_, _, _, _, _, _, _, _, _, h906, _) => {
                                    h906
                                }
                            };
                            nat_add(a907, a908)?
                        };
                        let a911 = match v0 {
                            Adt2::C0(_, _, _, _, _, _, _, _, _, _, h909) => {
                                h909
                            }
                        };
                        Ok(nat_le(a910, a911))
                    }
                }
            }
        }
    }
}

pub fn f24(v0: Adt6, v1: u64) -> R<u64> {
    let a913 = match v0.clone() {
        Adt6::C0(h912, _, _, _) => {
            h912
        }
    };
    let a914 = 0u64;
    if nat_eq(a913, a914) {
        f100(Adt6::C0(1u64, match v0.clone() {
            Adt6::C0(_, h915, _, _) => {
                h915
            }
        }, match v0.clone() {
            Adt6::C0(_, _, h916, _) => {
                h916
            }
        }, match v0.clone() {
            Adt6::C0(_, _, _, h917) => {
                h917
            }
        }), {
            let a919 = v1;
            let a920 = match v0 {
                Adt6::C0(_, _, _, h918) => {
                    h918
                }
            };
            nat_add(a919, a920)?
        })
    } else {
        f100(v0, v1)
    }
}

pub fn f25(v0: Adt2, v1: Adt2) -> bool {
    let a923 = match v0.clone() {
        Adt2::C0(_, h921, _, _, _, _, _, _, _, _, _) => {
            h921
        }
    };
    let a924 = match v1.clone() {
        Adt2::C0(_, h922, _, _, _, _, _, _, _, _, _) => {
            h922
        }
    };
    if if if nat_eq(a923, a924) {
        let a927 = match v0.clone() {
            Adt2::C0(_, _, h925, _, _, _, _, _, _, _, _) => {
                h925
            }
        };
        let a928 = match v1.clone() {
            Adt2::C0(_, _, h926, _, _, _, _, _, _, _, _) => {
                h926
            }
        };
        nat_eq(a927, a928)
    } else {
        false
    } {
        let a931 = match v0.clone() {
            Adt2::C0(_, _, _, h929, _, _, _, _, _, _, _) => {
                h929
            }
        };
        let a932 = match v1.clone() {
            Adt2::C0(_, _, _, h930, _, _, _, _, _, _, _) => {
                h930
            }
        };
        nat_eq(a931, a932)
    } else {
        false
    } {
        let a935 = match v0 {
            Adt2::C0(_, _, _, _, h933, _, _, _, _, _, _) => {
                h933
            }
        };
        let a936 = match v1 {
            Adt2::C0(_, _, _, _, h934, _, _, _, _, _, _) => {
                h934
            }
        };
        nat_eq(a935, a936)
    } else {
        false
    }
}

pub fn f26(v0: u64, v1: u64, v2: u64, v3: u64) -> R<u64> {
    let a941 = {
        let a937 = v0;
        let a938 = v1;
        nat_mul(a937, a938)?
    };
    let a942 = {
        let a939 = v2;
        let a940 = f101(v3);
        nat_mul(a939, a940)?
    };
    nat_mul(a941, a942)
}

pub fn f27(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let a945 = v0;
    let a946 = {
        let a943 = f102(v1)?;
        let a944 = v2;
        nat_mul(a943, a944)?
    };
    nat_add(a945, a946)
}

pub fn f28(v0: Adt2) -> Adt14 {
    Adt14::C0(match v0.clone() {
        Adt2::C0(h947, _, _, _, _, _, _, _, _, _, _) => {
            h947
        }
    }, match v0.clone() {
        Adt2::C0(_, h948, _, _, _, _, _, _, _, _, _) => {
            h948
        }
    }, match v0.clone() {
        Adt2::C0(_, _, h949, _, _, _, _, _, _, _, _) => {
            h949
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, h950, _, _, _, _, _, _, _) => {
            h950
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, h951, _, _, _, _, _, _) => {
            h951
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, _, h952, _, _, _, _, _) => {
            h952
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, _, _, h953, _, _, _, _) => {
            h953
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, _, _, _, h954, _, _, _) => {
            h954
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, _, _, _, _, h955, _, _) => {
            h955
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, _, _, _, _, _, h956, _) => {
            h956
        }
    }, match v0 {
        Adt2::C0(_, _, _, _, _, _, _, _, _, _, h957) => {
            h957
        }
    })
}

pub fn f29(v0: u64, v1: u64, v2: u64, v3: bool, v4: Adt14, v5: Adt14, v6: Adt14) -> R<Adt15> {
    let a958 = v0;
    let a959 = 23u64;
    if nat_eq(a958, a959) {
        f103(v1, v4, v6)
    } else {
        let a960 = v0;
        let a961 = 26u64;
        if nat_eq(a960, a961) {
            f104(v1, v2, v4, v6)
        } else {
            let a962 = v0;
            let a963 = 25u64;
            if nat_eq(a962, a963) {
                f105(v1, v3, v4, v5, v6)
            } else {
                Ok(Adt15::C0)
            }
        }
    }
}

pub fn f30(v0: u64) -> u64 {
    let a964 = v0;
    let a965 = 30u64;
    if nat_eq(a964, a965) {
        2u64
    } else {
        4u64
    }
}

pub fn f31(v0: u64) -> bool {
    let a966 = v0;
    let a967 = 2139095040u64;
    if nat_le(a966, a967) {
        true
    } else {
        let a968 = v0;
        let a969 = 2147483648u64;
        nat_eq(a968, a969)
    }
}

pub fn f32(v0: Adt2, v1: u64) -> bool {
    let a971 = match v0.clone() {
        Adt2::C0(_, _, _, _, _, h970, _, _, _, _, _) => {
            h970
        }
    };
    let a972 = v1;
    if if nat_eq(a971, a972) {
        let a977 = {
            let a974 = match v0.clone() {
                Adt2::C0(_, _, _, _, _, _, _, _, _, _, h973) => {
                    h973
                }
            };
            let a975 = v1;
            let a976 = 0u64;
            nat_rem(a974, a975, a976)
        };
        let a978 = 0u64;
        nat_eq(a977, a978)
    } else {
        false
    } {
        f106(match v0.clone() {
            Adt2::C0(_, _, h979, _, _, _, _, _, _, _, _) => {
                h979
            }
        }, match v0.clone() {
            Adt2::C0(_, _, _, h980, _, _, _, _, _, _, _) => {
                h980
            }
        }, match v0.clone() {
            Adt2::C0(_, _, _, _, h981, _, _, _, _, _, _) => {
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
        }, v1)
    } else {
        false
    }
}

pub fn f33(v0: Adt2, v1: Adt2) -> bool {
    if if if if if if f25(v0.clone(), v1.clone()) {
        let a987 = match v0.clone() {
            Adt2::C0(h985, _, _, _, _, _, _, _, _, _, _) => {
                h985
            }
        };
        let a988 = match v1.clone() {
            Adt2::C0(h986, _, _, _, _, _, _, _, _, _, _) => {
                h986
            }
        };
        nat_eq(a987, a988)
    } else {
        false
    } {
        let a991 = match v0.clone() {
            Adt2::C0(_, _, _, _, _, _, _, _, _, _, h989) => {
                h989
            }
        };
        let a992 = match v1.clone() {
            Adt2::C0(_, _, _, _, _, _, _, _, _, _, h990) => {
                h990
            }
        };
        nat_eq(a991, a992)
    } else {
        false
    } {
        let a995 = match v0.clone() {
            Adt2::C0(_, _, _, _, _, h993, _, _, _, _, _) => {
                h993
            }
        };
        let a996 = match v1.clone() {
            Adt2::C0(_, _, _, _, _, h994, _, _, _, _, _) => {
                h994
            }
        };
        nat_eq(a995, a996)
    } else {
        false
    } {
        let a999 = match v0.clone() {
            Adt2::C0(_, _, _, _, _, _, h997, _, _, _, _) => {
                h997
            }
        };
        let a1000 = match v1.clone() {
            Adt2::C0(_, _, _, _, _, _, h998, _, _, _, _) => {
                h998
            }
        };
        nat_eq(a999, a1000)
    } else {
        false
    } {
        let a1003 = match v0.clone() {
            Adt2::C0(_, _, _, _, _, _, _, h1001, _, _, _) => {
                h1001
            }
        };
        let a1004 = match v1.clone() {
            Adt2::C0(_, _, _, _, _, _, _, h1002, _, _, _) => {
                h1002
            }
        };
        nat_eq(a1003, a1004)
    } else {
        false
    } {
        let a1007 = match v0 {
            Adt2::C0(_, _, _, _, _, _, _, _, h1005, _, _) => {
                h1005
            }
        };
        let a1008 = match v1 {
            Adt2::C0(_, _, _, _, _, _, _, _, h1006, _, _) => {
                h1006
            }
        };
        nat_eq(a1007, a1008)
    } else {
        false
    }
}

pub fn f34(v0: u64, v1: u64, v2: u64, v3: u64) -> bool {
    let a1009 = v0;
    let a1010 = 0u64;
    if if if nat_eq(a1009, a1010) {
        true
    } else {
        let a1011 = v1;
        let a1012 = 0u64;
        nat_eq(a1011, a1012)
    } {
        true
    } else {
        let a1013 = v2;
        let a1014 = 0u64;
        nat_eq(a1013, a1014)
    } {
        true
    } else {
        let a1015 = v3;
        let a1016 = 0u64;
        nat_eq(a1015, a1016)
    }
}

pub fn f35(v0: Adt2, v1: u64) -> R<bool> {
    f107({
        let a1018 = match v0.clone() {
            Adt2::C0(_, h1017, _, _, _, _, _, _, _, _, _) => {
                h1017
            }
        };
        let a1019 = v1;
        nat_mul(a1018, a1019)?
    }, match v0.clone() {
        Adt2::C0(_, _, h1020, _, _, _, _, _, _, _, _) => {
            h1020
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, h1021, _, _, _, _, _, _, _) => {
            h1021
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, h1022, _, _, _, _, _, _) => {
            h1022
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, _, _, h1023, _, _, _, _) => {
            h1023
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, _, _, _, h1024, _, _, _) => {
            h1024
        }
    }, match v0 {
        Adt2::C0(_, _, _, _, _, _, _, _, h1025, _, _) => {
            h1025
        }
    })
}

pub fn f36(v0: u64, v1: u64, v2: u64, v3: u64) -> R<u64> {
    let a1026 = v1;
    let a1027 = 0u64;
    if if nat_eq(a1026, a1027) {
        true
    } else {
        let a1028 = v2;
        let a1029 = 0u64;
        nat_eq(a1028, a1029)
    } {
        Ok(0u64)
    } else {
        let a1036 = {
            let a1030 = f108(v1, v2, v3)?;
            let a1031 = f109(v0, v1, v2)?;
            nat_add(a1030, a1031)?
        };
        let a1037 = {
            let a1034 = f110(v0);
            let a1035 = {
                let a1032 = v1;
                let a1033 = v2;
                nat_mul(a1032, a1033)?
            };
            nat_mul(a1034, a1035)?
        };
        nat_add(a1036, a1037)
    }
}

pub fn f37(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let a1038 = v0;
    let a1039 = 0u64;
    if if nat_eq(a1038, a1039) {
        true
    } else {
        let a1040 = v1;
        let a1041 = 0u64;
        nat_eq(a1040, a1041)
    } {
        Ok(3u64)
    } else {
        let a1044 = {
            let a1042 = 3u64;
            let a1043 = v1;
            nat_add(a1042, a1043)?
        };
        let a1045 = v2;
        nat_add(a1044, a1045)
    }
}

pub fn f38(v0: u64) -> bool {
    let a1046 = v0;
    let a1047 = 2139095040u64;
    if nat_le(a1046, a1047) {
        true
    } else {
        let a1048 = v0;
        let a1049 = 2147483648u64;
        nat_eq(a1048, a1049)
    }
}

pub fn f39(v0: u64, v1: u64) -> R<u64> {
    let a1050 = v0;
    let a1051 = 0u64;
    if nat_eq(a1050, a1051) {
        Ok(0u64)
    } else {
        let a1052 = v1;
        let a1053 = f111(v0)?;
        nat_mul(a1052, a1053)
    }
}

pub fn f40(v0: u64, v1: u64, v2: u64, v3: u64) -> R<u64> {
    let a1054 = v0;
    let a1055 = 0u64;
    if if if if nat_eq(a1054, a1055) {
        true
    } else {
        let a1056 = v1;
        let a1057 = 0u64;
        nat_eq(a1056, a1057)
    } {
        true
    } else {
        let a1058 = v2;
        let a1059 = 0u64;
        nat_eq(a1058, a1059)
    } {
        true
    } else {
        let a1060 = v3;
        let a1061 = 0u64;
        nat_eq(a1060, a1061)
    } {
        Ok(0u64)
    } else {
        let a1074 = {
            let a1064 = 2u64;
            let a1065 = {
                let a1062 = v1;
                let a1063 = v2;
                nat_mul(a1062, a1063)?
            };
            nat_mul(a1064, a1065)?
        };
        let a1075 = {
            let a1072 = {
                let a1068 = {
                    let a1066 = v1;
                    let a1067 = v2;
                    nat_mul(a1066, a1067)?
                };
                let a1069 = v3;
                nat_mul(a1068, a1069)?
            };
            let a1073 = {
                let a1070 = 1u64;
                let a1071 = v0;
                nat_add(a1070, a1071)?
            };
            nat_mul(a1072, a1073)?
        };
        nat_add(a1074, a1075)
    }
}

pub fn f41(v0: Adt1, v1: Adt2, v2: Adt2, v3: Adt2) -> R<Adt10> {
    let v4: Adt6 = Adt6::C0(match v1.clone() {
        Adt2::C0(_, h1076, _, _, _, _, _, _, _, _, _) => {
            h1076
        }
    }, match v0.clone() {
        Adt1::C0(h1077, _, _, _, _, _, _, _, _, _, _, _) => {
            h1077
        }
    }, match v0.clone() {
        Adt1::C0(_, _, h1078, _, _, _, _, _, _, _, _, _) => {
            h1078
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, _, h1079, _, _, _, _, _, _, _) => {
            h1079
        }
    });
    let v5: Adt6 = Adt6::C0(match v1.clone() {
        Adt2::C0(_, _, h1080, _, _, _, _, _, _, _, _) => {
            h1080
        }
    }, match v0.clone() {
        Adt1::C0(_, h1081, _, _, _, _, _, _, _, _, _, _) => {
            h1081
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, h1082, _, _, _, _, _, _, _, _) => {
            h1082
        }
    }, match v0 {
        Adt1::C0(_, _, _, _, _, h1083, _, _, _, _, _, _) => {
            h1083
        }
    });
    let v6: u64 = match v1.clone() {
        Adt2::C0(h1084, _, _, _, _, _, _, _, _, _, _) => {
            h1084
        }
    };
    let v7: bool = {
        let a1087 = match v2.clone() {
            Adt2::C0(_, _, _, _, _, _, _, h1085, _, _, _) => {
                h1085
            }
        };
        let a1088 = match v2.clone() {
            Adt2::C0(_, _, _, _, _, h1086, _, _, _, _, _) => {
                h1086
            }
        };
        nat_lt(a1087, a1088)
    };
    let a1089 = v6;
    let a1090 = 0u64;
    if if if if if if if if if if if if if if if nat_eq(a1089, a1090) {
        true
    } else {
        let a1091 = v6;
        let a1092 = 1u64;
        nat_eq(a1091, a1092)
    } {
        let a1094 = match v2.clone() {
            Adt2::C0(h1093, _, _, _, _, _, _, _, _, _, _) => {
                h1093
            }
        };
        let a1095 = 0u64;
        nat_eq(a1094, a1095)
    } else {
        false
    } {
        let a1097 = match v3.clone() {
            Adt2::C0(h1096, _, _, _, _, _, _, _, _, _, _) => {
                h1096
            }
        };
        let a1098 = 0u64;
        nat_eq(a1097, a1098)
    } else {
        false
    } {
        let a1100 = match v1.clone() {
            Adt2::C0(_, _, _, h1099, _, _, _, _, _, _, _) => {
                h1099
            }
        };
        let a1101 = 1u64;
        nat_eq(a1100, a1101)
    } else {
        false
    } {
        let a1104 = match v1.clone() {
            Adt2::C0(_, _, _, _, h1102, _, _, _, _, _, _) => {
                h1102
            }
        };
        let a1105 = match v2.clone() {
            Adt2::C0(_, _, _, h1103, _, _, _, _, _, _, _) => {
                h1103
            }
        };
        nat_eq(a1104, a1105)
    } else {
        false
    } {
        let a1108 = match v3.clone() {
            Adt2::C0(_, _, _, h1106, _, _, _, _, _, _, _) => {
                h1106
            }
        };
        let a1109 = match v2.clone() {
            Adt2::C0(_, _, _, h1107, _, _, _, _, _, _, _) => {
                h1107
            }
        };
        nat_eq(a1108, a1109)
    } else {
        false
    } {
        let a1112 = match v3.clone() {
            Adt2::C0(_, _, _, _, h1110, _, _, _, _, _, _) => {
                h1110
            }
        };
        let a1113 = match v2.clone() {
            Adt2::C0(_, _, _, _, h1111, _, _, _, _, _, _) => {
                h1111
            }
        };
        nat_eq(a1112, a1113)
    } else {
        false
    } {
        let a1115 = 0u64;
        let a1116 = match v4.clone() {
            Adt6::C0(h1114, _, _, _) => {
                h1114
            }
        };
        if if if if if nat_lt(a1115, a1116) {
            let a1118 = 0u64;
            let a1119 = match v4.clone() {
                Adt6::C0(_, h1117, _, _) => {
                    h1117
                }
            };
            nat_lt(a1118, a1119)
        } else {
            false
        } {
            let a1121 = match v4.clone() {
                Adt6::C0(_, h1120, _, _) => {
                    h1120
                }
            };
            let a1122 = 2147483648u64;
            nat_lt(a1121, a1122)
        } else {
            false
        } {
            let a1124 = 0u64;
            let a1125 = match v4.clone() {
                Adt6::C0(_, _, _, h1123) => {
                    h1123
                }
            };
            nat_lt(a1124, a1125)
        } else {
            false
        } {
            let a1127 = match v4.clone() {
                Adt6::C0(_, _, _, h1126) => {
                    h1126
                }
            };
            let a1128 = 2147483648u64;
            nat_lt(a1127, a1128)
        } else {
            false
        } {
            let a1131 = match v3.clone() {
                Adt2::C0(_, h1129, _, _, _, _, _, _, _, _, _) => {
                    h1129
                }
            };
            let a1132 = f100(v4.clone(), match v2.clone() {
                Adt2::C0(_, h1130, _, _, _, _, _, _, _, _, _) => {
                    h1130
                }
            })?;
            nat_eq(a1131, a1132)
        } else {
            false
        }
    } else {
        false
    } {
        let a1134 = 0u64;
        let a1135 = match v5.clone() {
            Adt6::C0(h1133, _, _, _) => {
                h1133
            }
        };
        if if if if if nat_lt(a1134, a1135) {
            let a1137 = 0u64;
            let a1138 = match v5.clone() {
                Adt6::C0(_, h1136, _, _) => {
                    h1136
                }
            };
            nat_lt(a1137, a1138)
        } else {
            false
        } {
            let a1140 = match v5.clone() {
                Adt6::C0(_, h1139, _, _) => {
                    h1139
                }
            };
            let a1141 = 2147483648u64;
            nat_lt(a1140, a1141)
        } else {
            false
        } {
            let a1143 = 0u64;
            let a1144 = match v5.clone() {
                Adt6::C0(_, _, _, h1142) => {
                    h1142
                }
            };
            nat_lt(a1143, a1144)
        } else {
            false
        } {
            let a1146 = match v5.clone() {
                Adt6::C0(_, _, _, h1145) => {
                    h1145
                }
            };
            let a1147 = 2147483648u64;
            nat_lt(a1146, a1147)
        } else {
            false
        } {
            let a1150 = match v3.clone() {
                Adt2::C0(_, _, h1148, _, _, _, _, _, _, _, _) => {
                    h1148
                }
            };
            let a1151 = f100(v5.clone(), match v2.clone() {
                Adt2::C0(_, _, h1149, _, _, _, _, _, _, _, _) => {
                    h1149
                }
            })?;
            nat_eq(a1150, a1151)
        } else {
            false
        }
    } else {
        false
    } {
        f112(v2.clone(), 4u64, v7, false)?
    } else {
        false
    } {
        f112(v3.clone(), 4u64, v7, false)?
    } else {
        false
    } {
        f112(v1.clone(), {
            let a1152 = v6;
            let a1153 = 0u64;
            if nat_eq(a1152, a1153) {
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
            let a1162 = {
                let a1160 = {
                    let a1158 = 3u64;
                    let a1159 = {
                        let a1156 = match v4.clone() {
                            Adt6::C0(h1154, _, _, _) => {
                                h1154
                            }
                        };
                        let a1157 = match v5.clone() {
                            Adt6::C0(h1155, _, _, _) => {
                                h1155
                            }
                        };
                        nat_mul(a1156, a1157)?
                    };
                    nat_mul(a1158, a1159)?
                };
                let a1161 = 2u64;
                nat_add(a1160, a1161)?
            };
            let a1163 = f16(v3.clone(), 1u64)?;
            nat_mul(a1162, a1163)?
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
        f113(v0.clone())?
    } else {
        false
    } {
        f113(v1.clone())?
    } else {
        false
    } {
        f113(v2.clone())?
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
            let a1164 = 6u64;
            let a1165 = f16(v0.clone(), 1u64)?;
            nat_mul(a1164, a1165)?
        }, 2u64, f20(v0, 1u64), f20(v2, 1u64), Adt4::C33(f20(v1, 1u64))))
    } else {
        Ok(Adt10::C1)
    }
}

pub fn f43(v0: Adt1, v1: Adt2, v2: Option<Adt2>, v3: Adt2) -> R<Adt10> {
    let m1166 = v2;
    match m1166 {
        None => {
            Ok(Adt10::C1)
        }
        Some(v4) => {
            let v5: u64 = match v1.clone() {
                Adt2::C0(_, h1167, _, _, _, _, _, _, _, _, _) => {
                    h1167
                }
            };
            let v6: u64 = {
                let a1173 = match v1.clone() {
                    Adt2::C0(_, _, h1168, _, _, _, _, _, _, _, _) => {
                        h1168
                    }
                };
                let a1174 = {
                    let a1171 = match v1.clone() {
                        Adt2::C0(_, _, _, h1169, _, _, _, _, _, _, _) => {
                            h1169
                        }
                    };
                    let a1172 = match v1.clone() {
                        Adt2::C0(_, _, _, _, h1170, _, _, _, _, _, _) => {
                            h1170
                        }
                    };
                    nat_mul(a1171, a1172)?
                };
                nat_mul(a1173, a1174)?
            };
            if if if if if if if if if if f25(v1.clone(), v4.clone()) {
                f25(v1.clone(), v3.clone())
            } else {
                false
            } {
                f114(v1.clone(), v6)?
            } else {
                false
            } {
                f114(v4.clone(), v6)?
            } else {
                false
            } {
                f114(v3.clone(), v6)?
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
                let a1175 = v5;
                let a1176 = 2147483647u64;
                nat_le(a1175, a1176)
            } else {
                false
            } {
                let a1177 = v6;
                let a1178 = 2147483647u64;
                nat_le(a1177, a1178)
            } else {
                false
            } {
                let a1180 = match v0.clone() {
                    Adt1::C0(h1179, _, _, _, _, _, _, _, _, _, _, _) => {
                        h1179
                    }
                };
                let a1181 = 4294967295u64;
                if nat_le(a1180, a1181) {
                    let a1183 = match v0.clone() {
                        Adt1::C0(_, h1182, _, _, _, _, _, _, _, _, _, _) => {
                            h1182
                        }
                    };
                    let a1184 = 4294967295u64;
                    nat_le(a1183, a1184)
                } else {
                    false
                }
            } else {
                false
            } {
                Ok(Adt10::C4({
                    let a1185 = v5;
                    let a1186 = 0u64;
                    if nat_eq(a1185, a1186) {
                        0u64
                    } else {
                        let a1187 = v6;
                        let a1188 = f115(v5)?;
                        nat_mul(a1187, a1188)?
                    }
                }, 4u64, f20(v1, 1u64), f20(v3, 1u64), Adt4::C32(match v0 {
                    Adt1::C0(h1189, _, _, _, _, _, _, _, _, _, _, _) => {
                        h1189
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
        Adt1::C0(h1190, _, _, _, _, _, _, _, _, _, _, _) => {
            h1190
        }
    };
    let v4: u64 = match v0.clone() {
        Adt1::C0(_, h1191, _, _, _, _, _, _, _, _, _, _) => {
            h1191
        }
    };
    let v5: u64 = match v0 {
        Adt1::C0(_, _, h1192, _, _, _, _, _, _, _, _, _) => {
            h1192
        }
    };
    let v6: u64 = {
        let a1194 = match v1.clone() {
            Adt2::C0(_, h1193, _, _, _, _, _, _, _, _, _) => {
                h1193
            }
        };
        let a1195 = v4;
        let a1196 = 0u64;
        nat_quot(a1194, a1195, a1196)
    };
    let v7: u64 = {
        let a1198 = match v1.clone() {
            Adt2::C0(h1197, _, _, _, _, _, _, _, _, _, _) => {
                h1197
            }
        };
        let a1199 = 0u64;
        if nat_eq(a1198, a1199) {
            4u64
        } else {
            2u64
        }
    };
    if if if if if if if if if if if if if if if f116(match v1.clone() {
        Adt2::C0(h1200, _, _, _, _, _, _, _, _, _, _) => {
            h1200
        }
    }) {
        let a1203 = match v1.clone() {
            Adt2::C0(h1201, _, _, _, _, _, _, _, _, _, _) => {
                h1201
            }
        };
        let a1204 = match v2.clone() {
            Adt2::C0(h1202, _, _, _, _, _, _, _, _, _, _) => {
                h1202
            }
        };
        nat_eq(a1203, a1204)
    } else {
        false
    } {
        let a1205 = 0u64;
        let a1206 = v3;
        nat_lt(a1205, a1206)
    } else {
        false
    } {
        let a1207 = v3;
        let a1208 = 2147483648u64;
        nat_lt(a1207, a1208)
    } else {
        false
    } {
        let a1209 = 0u64;
        let a1210 = v4;
        nat_lt(a1209, a1210)
    } else {
        false
    } {
        let a1211 = v4;
        let a1212 = 2147483648u64;
        nat_lt(a1211, a1212)
    } else {
        false
    } {
        let a1213 = v5;
        let a1214 = 2147483648u64;
        nat_lt(a1213, a1214)
    } else {
        false
    } {
        let a1215 = 0u64;
        let a1216 = v6;
        nat_lt(a1215, a1216)
    } else {
        false
    } {
        let a1218 = 0u64;
        let a1219 = match v2.clone() {
            Adt2::C0(_, h1217, _, _, _, _, _, _, _, _, _) => {
                h1217
            }
        };
        nat_lt(a1218, a1219)
    } else {
        false
    } {
        let a1223 = match v1.clone() {
            Adt2::C0(_, h1220, _, _, _, _, _, _, _, _, _) => {
                h1220
            }
        };
        let a1224 = {
            let a1221 = v6;
            let a1222 = v4;
            nat_mul(a1221, a1222)?
        };
        nat_eq(a1223, a1224)
    } else {
        false
    } {
        let a1226 = match v2.clone() {
            Adt2::C0(_, _, h1225, _, _, _, _, _, _, _, _) => {
                h1225
            }
        };
        let a1227 = v4;
        nat_eq(a1226, a1227)
    } else {
        false
    } {
        let a1240 = {
            let a1233 = {
                let a1231 = match v2.clone() {
                    Adt2::C0(_, h1228, _, _, _, _, _, _, _, _, _) => {
                        h1228
                    }
                };
                let a1232 = {
                    let a1229 = v5;
                    let a1230 = 2u64;
                    nat_mul(a1229, a1230)?
                };
                nat_add(a1231, a1232)?
            };
            let a1234 = v3;
            nat_add(a1233, a1234)?
        };
        let a1241 = {
            let a1238 = {
                let a1236 = match v1.clone() {
                    Adt2::C0(_, _, h1235, _, _, _, _, _, _, _, _) => {
                        h1235
                    }
                };
                let a1237 = v3;
                nat_mul(a1236, a1237)?
            };
            let a1239 = v6;
            nat_add(a1238, a1239)?
        };
        nat_eq(a1240, a1241)
    } else {
        false
    } {
        f117(v1.clone(), v7)?
    } else {
        false
    } {
        f117(v2.clone(), v7)?
    } else {
        false
    } {
        f23(v1.clone(), v2.clone())?
    } else {
        false
    } {
        let v8: Adt6 = Adt6::C0(v6, v3, v5, 1u64);
        Ok(Adt10::C4({
            let a1242 = f16(v2.clone(), 1u64)?;
            let a1243 = f118(v8.clone())?;
            nat_mul(a1242, a1243)?
        }, v7, f20(v1.clone(), 1u64), f20(v2, 1u64), Adt4::C30(v8, match v1 {
            Adt2::C0(h1244, _, _, _, _, _, _, _, _, _, _) => {
                h1244
            }
        })))
    } else {
        Ok(Adt10::C1)
    }
}

pub fn f45(v0: Adt2, v1: Option<Adt2>, v2: Adt2) -> R<Adt10> {
    let m1245 = v1;
    match m1245 {
        None => {
            Ok(Adt10::C1)
        }
        Some(v3) => {
            let a1247 = match v0.clone() {
                Adt2::C0(h1246, _, _, _, _, _, _, _, _, _, _) => {
                    h1246
                }
            };
            let a1248 = 0u64;
            if if if if if if if if if if if if if if if if if if if nat_eq(a1247, a1248) {
                let a1250 = match v3.clone() {
                    Adt2::C0(h1249, _, _, _, _, _, _, _, _, _, _) => {
                        h1249
                    }
                };
                let a1251 = 0u64;
                nat_eq(a1250, a1251)
            } else {
                false
            } {
                let a1253 = match v2.clone() {
                    Adt2::C0(h1252, _, _, _, _, _, _, _, _, _, _) => {
                        h1252
                    }
                };
                let a1254 = 0u64;
                nat_eq(a1253, a1254)
            } else {
                false
            } {
                let a1256 = match v0.clone() {
                    Adt2::C0(_, _, _, _, h1255, _, _, _, _, _, _) => {
                        h1255
                    }
                };
                let a1257 = 1u64;
                nat_eq(a1256, a1257)
            } else {
                false
            } {
                let a1259 = match v3.clone() {
                    Adt2::C0(_, _, _, h1258, _, _, _, _, _, _, _) => {
                        h1258
                    }
                };
                let a1260 = 1u64;
                nat_eq(a1259, a1260)
            } else {
                false
            } {
                let a1262 = match v3.clone() {
                    Adt2::C0(_, _, _, _, h1261, _, _, _, _, _, _) => {
                        h1261
                    }
                };
                let a1263 = 1u64;
                nat_eq(a1262, a1263)
            } else {
                false
            } {
                let a1266 = match v0.clone() {
                    Adt2::C0(_, _, h1264, _, _, _, _, _, _, _, _) => {
                        h1264
                    }
                };
                let a1267 = match v3.clone() {
                    Adt2::C0(_, _, h1265, _, _, _, _, _, _, _, _) => {
                        h1265
                    }
                };
                nat_eq(a1266, a1267)
            } else {
                false
            } {
                let a1272 = match v3.clone() {
                    Adt2::C0(_, h1268, _, _, _, _, _, _, _, _, _) => {
                        h1268
                    }
                };
                let a1273 = {
                    let a1270 = match v0.clone() {
                        Adt2::C0(_, h1269, _, _, _, _, _, _, _, _, _) => {
                            h1269
                        }
                    };
                    let a1271 = 1u64;
                    nat_add(a1270, a1271)?
                };
                nat_le(a1272, a1273)
            } else {
                false
            } {
                let a1276 = match v2.clone() {
                    Adt2::C0(_, h1274, _, _, _, _, _, _, _, _, _) => {
                        h1274
                    }
                };
                let a1277 = match v0.clone() {
                    Adt2::C0(_, _, h1275, _, _, _, _, _, _, _, _) => {
                        h1275
                    }
                };
                nat_eq(a1276, a1277)
            } else {
                false
            } {
                let a1285 = match v2.clone() {
                    Adt2::C0(_, _, h1278, _, _, _, _, _, _, _, _) => {
                        h1278
                    }
                };
                let a1286 = {
                    let a1283 = {
                        let a1280 = match v0.clone() {
                            Adt2::C0(_, h1279, _, _, _, _, _, _, _, _, _) => {
                                h1279
                            }
                        };
                        let a1281 = 1u64;
                        nat_add(a1280, a1281)?
                    };
                    let a1284 = match v3.clone() {
                        Adt2::C0(_, h1282, _, _, _, _, _, _, _, _, _) => {
                            h1282
                        }
                    };
                    nat_sub(a1283, a1284)
                };
                nat_eq(a1285, a1286)
            } else {
                false
            } {
                let a1289 = match v2.clone() {
                    Adt2::C0(_, _, _, h1287, _, _, _, _, _, _, _) => {
                        h1287
                    }
                };
                let a1290 = match v0.clone() {
                    Adt2::C0(_, _, _, h1288, _, _, _, _, _, _, _) => {
                        h1288
                    }
                };
                nat_eq(a1289, a1290)
            } else {
                false
            } {
                let a1292 = match v2.clone() {
                    Adt2::C0(_, _, _, _, h1291, _, _, _, _, _, _) => {
                        h1291
                    }
                };
                let a1293 = 1u64;
                nat_eq(a1292, a1293)
            } else {
                false
            } {
                f22(v2.clone())?
            } else {
                false
            } {
                let a1295 = match v0.clone() {
                    Adt2::C0(_, _, _, _, _, h1294, _, _, _, _, _) => {
                        h1294
                    }
                };
                let a1296 = 4u64;
                nat_eq(a1295, a1296)
            } else {
                false
            } {
                let a1298 = match v3.clone() {
                    Adt2::C0(_, _, _, _, _, h1297, _, _, _, _, _) => {
                        h1297
                    }
                };
                let a1299 = 4u64;
                nat_eq(a1298, a1299)
            } else {
                false
            } {
                let a1304 = match v0.clone() {
                    Adt2::C0(_, _, _, _, _, _, h1300, _, _, _, _) => {
                        h1300
                    }
                };
                let a1305 = {
                    let a1302 = match v0.clone() {
                        Adt2::C0(_, h1301, _, _, _, _, _, _, _, _, _) => {
                            h1301
                        }
                    };
                    let a1303 = 4u64;
                    nat_mul(a1302, a1303)?
                };
                nat_eq(a1304, a1305)
            } else {
                false
            } {
                let a1310 = match v3.clone() {
                    Adt2::C0(_, _, _, _, _, _, h1306, _, _, _, _) => {
                        h1306
                    }
                };
                let a1311 = {
                    let a1308 = match v3.clone() {
                        Adt2::C0(_, h1307, _, _, _, _, _, _, _, _, _) => {
                            h1307
                        }
                    };
                    let a1309 = 4u64;
                    nat_mul(a1308, a1309)?
                };
                nat_eq(a1310, a1311)
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
                    let a1313 = f16(v2.clone(), 1u64)?;
                    let a1314 = f119(match v3.clone() {
                        Adt2::C0(_, h1312, _, _, _, _, _, _, _, _, _) => {
                            h1312
                        }
                    })?;
                    nat_mul(a1313, a1314)?
                }, 4u64, f20(v0, 1u64), f20(v2, 1u64), Adt4::C29(f20(v3, 1u64))))
            } else {
                Ok(Adt10::C1)
            }
        }
    }
}

pub fn f46(v0: bool, v1: Adt1, v2: Adt2, v3: Adt2) -> R<Adt10> {
    let v4: Adt6 = Adt6::C0(match v1.clone() {
        Adt1::C0(_, h1315, _, _, _, _, _, _, _, _, _, _) => {
            h1315
        }
    }, if v0 {
        match v1.clone() {
            Adt1::C0(_, _, _, h1316, _, _, _, _, _, _, _, _) => {
                h1316
            }
        }
    } else {
        match v1.clone() {
            Adt1::C0(_, _, h1317, _, _, _, _, _, _, _, _, _) => {
                h1317
            }
        }
    }, if v0 {
        match v1.clone() {
            Adt1::C0(_, _, _, _, _, h1318, _, _, _, _, _, _) => {
                h1318
            }
        }
    } else {
        match v1.clone() {
            Adt1::C0(_, _, _, h1319, _, _, _, _, _, _, _, _) => {
                h1319
            }
        }
    }, 1u64);
    let v5: Adt6 = if v0 {
        Adt6::C0(match v1.clone() {
            Adt1::C0(_, _, h1320, _, _, _, _, _, _, _, _, _) => {
                h1320
            }
        }, match v1.clone() {
            Adt1::C0(_, _, _, _, h1321, _, _, _, _, _, _, _) => {
                h1321
            }
        }, match v1.clone() {
            Adt1::C0(_, _, _, _, _, _, h1322, _, _, _, _, _) => {
                h1322
            }
        }, 1u64)
    } else {
        Adt6::C0(1u64, 1u64, 0u64, 1u64)
    };
    let a1324 = match v1.clone() {
        Adt1::C0(h1323, _, _, _, _, _, _, _, _, _, _, _) => {
            h1323
        }
    };
    let a1325 = 1u64;
    if if if if if if if if if if if nat_le(a1324, a1325) {
        let a1327 = match v2.clone() {
            Adt2::C0(h1326, _, _, _, _, _, _, _, _, _, _) => {
                h1326
            }
        };
        let a1328 = 1u64;
        nat_le(a1327, a1328)
    } else {
        false
    } {
        let a1330 = match v3.clone() {
            Adt2::C0(h1329, _, _, _, _, _, _, _, _, _, _) => {
                h1329
            }
        };
        let a1331 = 0u64;
        nat_eq(a1330, a1331)
    } else {
        false
    } {
        let a1336 = match v2.clone() {
            Adt2::C0(_, _, _, _, _, h1332, _, _, _, _, _) => {
                h1332
            }
        };
        let a1337 = {
            let a1334 = match v2.clone() {
                Adt2::C0(h1333, _, _, _, _, _, _, _, _, _, _) => {
                    h1333
                }
            };
            let a1335 = 0u64;
            if nat_eq(a1334, a1335) {
                4u64
            } else {
                2u64
            }
        };
        nat_eq(a1336, a1337)
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
        f120(v4.clone(), match v2.clone() {
            Adt2::C0(_, h1338, _, _, _, _, _, _, _, _, _) => {
                h1338
            }
        }, match v3.clone() {
            Adt2::C0(_, h1339, _, _, _, _, _, _, _, _, _) => {
                h1339
            }
        })?
    } else {
        false
    } {
        if v0 {
            f120(v5.clone(), match v2.clone() {
                Adt2::C0(_, _, h1340, _, _, _, _, _, _, _, _) => {
                    h1340
                }
            }, match v3.clone() {
                Adt2::C0(_, _, h1341, _, _, _, _, _, _, _, _) => {
                    h1341
                }
            })?
        } else {
            let a1344 = match v2.clone() {
                Adt2::C0(_, _, h1342, _, _, _, _, _, _, _, _) => {
                    h1342
                }
            };
            let a1345 = match v3.clone() {
                Adt2::C0(_, _, h1343, _, _, _, _, _, _, _, _) => {
                    h1343
                }
            };
            nat_eq(a1344, a1345)
        }
    } else {
        false
    } {
        let a1348 = match v2.clone() {
            Adt2::C0(_, _, _, h1346, _, _, _, _, _, _, _) => {
                h1346
            }
        };
        let a1349 = match v3.clone() {
            Adt2::C0(_, _, _, h1347, _, _, _, _, _, _, _) => {
                h1347
            }
        };
        nat_eq(a1348, a1349)
    } else {
        false
    } {
        let a1352 = match v2.clone() {
            Adt2::C0(_, _, _, _, h1350, _, _, _, _, _, _) => {
                h1350
            }
        };
        let a1353 = match v3.clone() {
            Adt2::C0(_, _, _, _, h1351, _, _, _, _, _, _) => {
                h1351
            }
        };
        nat_eq(a1352, a1353)
    } else {
        false
    } {
        let a1358 = {
            let a1356 = match v4.clone() {
                Adt6::C0(h1354, _, _, _) => {
                    h1354
                }
            };
            let a1357 = match v5.clone() {
                Adt6::C0(h1355, _, _, _) => {
                    h1355
                }
            };
            nat_mul(a1356, a1357)?
        };
        let a1359 = 2147483648u64;
        nat_lt(a1358, a1359)
    } else {
        false
    } {
        let v6: Adt7 = Adt7::C0(v4, v5, v0, {
            let a1361 = match v1 {
                Adt1::C0(h1360, _, _, _, _, _, _, _, _, _, _, _) => {
                    h1360
                }
            };
            let a1362 = 0u64;
            nat_eq(a1361, a1362)
        }, match v2.clone() {
            Adt2::C0(h1363, _, _, _, _, _, _, _, _, _, _) => {
                h1363
            }
        });
        Ok(Adt10::C4({
            let a1364 = f16(v3.clone(), 1u64)?;
            let a1365 = f121(v6.clone())?;
            nat_mul(a1364, a1365)?
        }, 4u64, f20(v2, 1u64), f20(v3, 1u64), Adt4::C28(v6)))
    } else {
        Ok(Adt10::C1)
    }
}

pub fn f47(v0: Adt1, v1: Adt2, v2: Option<Adt2>, v3: Adt2) -> R<Adt10> {
    let m1366 = v2;
    match m1366 {
        None => {
            Ok(Adt10::C1)
        }
        Some(v4) => {
            let v5: bool = {
                let a1368 = match v0.clone() {
                    Adt1::C0(_, _, _, _, _, _, h1367, _, _, _, _, _) => {
                        h1367
                    }
                };
                let a1369 = 1u64;
                nat_eq(a1368, a1369)
            };
            let v6: Adt6 = Adt6::C0(match v1.clone() {
                Adt2::C0(_, h1370, _, _, _, _, _, _, _, _, _) => {
                    h1370
                }
            }, match v0.clone() {
                Adt1::C0(h1371, _, _, _, _, _, _, _, _, _, _, _) => {
                    h1371
                }
            }, match v0.clone() {
                Adt1::C0(_, _, h1372, _, _, _, _, _, _, _, _, _) => {
                    h1372
                }
            }, match v0.clone() {
                Adt1::C0(_, _, _, _, h1373, _, _, _, _, _, _, _) => {
                    h1373
                }
            });
            let v7: Adt6 = if v5 {
                Adt6::C0(match v1.clone() {
                    Adt2::C0(_, _, h1374, _, _, _, _, _, _, _, _) => {
                        h1374
                    }
                }, match v0.clone() {
                    Adt1::C0(_, h1375, _, _, _, _, _, _, _, _, _, _) => {
                        h1375
                    }
                }, match v0.clone() {
                    Adt1::C0(_, _, _, h1376, _, _, _, _, _, _, _, _) => {
                        h1376
                    }
                }, match v0.clone() {
                    Adt1::C0(_, _, _, _, _, h1377, _, _, _, _, _, _) => {
                        h1377
                    }
                })
            } else {
                Adt6::C0(1u64, 1u64, 0u64, 1u64)
            };
            let v8: Adt6 = Adt6::C0(1u64, 1u64, 0u64, 1u64);
            let v9: u64 = if v5 {
                match v4.clone() {
                    Adt2::C0(_, _, _, h1378, _, _, _, _, _, _, _) => {
                        h1378
                    }
                }
            } else {
                match v4.clone() {
                    Adt2::C0(_, _, h1379, _, _, _, _, _, _, _, _) => {
                        h1379
                    }
                }
            };
            let v10: u64 = if v5 {
                match v4.clone() {
                    Adt2::C0(_, _, _, _, h1380, _, _, _, _, _, _) => {
                        h1380
                    }
                }
            } else {
                match v4.clone() {
                    Adt2::C0(_, _, _, h1381, _, _, _, _, _, _, _) => {
                        h1381
                    }
                }
            };
            let v11: u64 = if v5 {
                match v4.clone() {
                    Adt2::C0(_, _, h1382, _, _, _, _, _, _, _, _) => {
                        h1382
                    }
                }
            } else {
                1u64
            };
            let v12: u64 = match v4.clone() {
                Adt2::C0(h1383, _, _, _, _, _, _, _, _, _, _) => {
                    h1383
                }
            };
            let v13: u64 = match v3.clone() {
                Adt2::C0(h1384, _, _, _, _, _, _, _, _, _, _) => {
                    h1384
                }
            };
            let v14: u64 = {
                let a1385 = v12;
                let a1386 = 0u64;
                if nat_eq(a1385, a1386) {
                    4u64
                } else {
                    2u64
                }
            };
            let v15: u64 = {
                let a1388 = match v4.clone() {
                    Adt2::C0(_, h1387, _, _, _, _, _, _, _, _, _) => {
                        h1387
                    }
                };
                let a1389 = v14;
                nat_mul(a1388, a1389)?
            };
            let v16: u64 = if v5 {
                match v4.clone() {
                    Adt2::C0(_, _, _, _, _, _, _, h1390, _, _, _) => {
                        h1390
                    }
                }
            } else {
                match v4.clone() {
                    Adt2::C0(_, _, _, _, _, _, h1391, _, _, _, _) => {
                        h1391
                    }
                }
            };
            let v17: u64 = if v5 {
                match v4.clone() {
                    Adt2::C0(_, _, _, _, _, _, _, _, h1392, _, _) => {
                        h1392
                    }
                }
            } else {
                match v4.clone() {
                    Adt2::C0(_, _, _, _, _, _, _, h1393, _, _, _) => {
                        h1393
                    }
                }
            };
            let a1395 = match v0 {
                Adt1::C0(_, _, _, _, _, _, h1394, _, _, _, _, _) => {
                    h1394
                }
            };
            let a1396 = 1u64;
            if if if if if if if if if if nat_le(a1395, a1396) {
                let a1397 = v12;
                let a1398 = 0u64;
                if nat_eq(a1397, a1398) {
                    true
                } else {
                    let a1399 = v12;
                    let a1400 = 1u64;
                    if nat_eq(a1399, a1400) {
                        let a1401 = v13;
                        let a1402 = 1u64;
                        nat_eq(a1401, a1402)
                    } else {
                        false
                    }
                }
            } else {
                false
            } {
                let a1403 = v13;
                let a1404 = 0u64;
                if nat_eq(a1403, a1404) {
                    true
                } else {
                    let a1405 = v13;
                    let a1406 = 1u64;
                    nat_eq(a1405, a1406)
                }
            } else {
                false
            } {
                let a1408 = match v4.clone() {
                    Adt2::C0(_, _, _, _, _, h1407, _, _, _, _, _) => {
                        h1407
                    }
                };
                let a1409 = v14;
                nat_eq(a1408, a1409)
            } else {
                false
            } {
                let a1417 = match v3.clone() {
                    Adt2::C0(_, h1410, _, _, _, _, _, _, _, _, _) => {
                        h1410
                    }
                };
                let a1418 = {
                    let a1415 = v9;
                    let a1416 = {
                        let a1413 = match v6.clone() {
                            Adt6::C0(h1411, _, _, _) => {
                                h1411
                            }
                        };
                        let a1414 = match v7.clone() {
                            Adt6::C0(h1412, _, _, _) => {
                                h1412
                            }
                        };
                        nat_mul(a1413, a1414)?
                    };
                    nat_mul(a1415, a1416)?
                };
                nat_eq(a1417, a1418)
            } else {
                false
            } {
                let a1421 = match v3.clone() {
                    Adt2::C0(_, _, h1419, _, _, _, _, _, _, _, _) => {
                        h1419
                    }
                };
                let a1422 = f100(v6.clone(), match v4.clone() {
                    Adt2::C0(_, h1420, _, _, _, _, _, _, _, _, _) => {
                        h1420
                    }
                })?;
                nat_eq(a1421, a1422)
            } else {
                false
            } {
                let a1424 = 0u64;
                let a1425 = match v3.clone() {
                    Adt2::C0(_, _, h1423, _, _, _, _, _, _, _, _) => {
                        h1423
                    }
                };
                nat_lt(a1424, a1425)
            } else {
                false
            } {
                if v5 {
                    let a1427 = match v1 {
                        Adt2::C0(_, _, _, h1426, _, _, _, _, _, _, _) => {
                            h1426
                        }
                    };
                    let a1428 = v9;
                    if if if nat_eq(a1427, a1428) {
                        let a1430 = match v3.clone() {
                            Adt2::C0(_, _, _, h1429, _, _, _, _, _, _, _) => {
                                h1429
                            }
                        };
                        let a1431 = f100(v7.clone(), v11)?;
                        nat_eq(a1430, a1431)
                    } else {
                        false
                    } {
                        let a1433 = match v3.clone() {
                            Adt2::C0(_, _, _, _, h1432, _, _, _, _, _, _) => {
                                h1432
                            }
                        };
                        let a1434 = v10;
                        nat_eq(a1433, a1434)
                    } else {
                        false
                    } {
                        let a1436 = 0u64;
                        let a1437 = match v3.clone() {
                            Adt2::C0(_, _, _, h1435, _, _, _, _, _, _, _) => {
                                h1435
                            }
                        };
                        nat_lt(a1436, a1437)
                    } else {
                        false
                    }
                } else {
                    let a1439 = match v1 {
                        Adt2::C0(_, _, h1438, _, _, _, _, _, _, _, _) => {
                            h1438
                        }
                    };
                    let a1440 = v9;
                    if if if nat_eq(a1439, a1440) {
                        let a1442 = match v4.clone() {
                            Adt2::C0(_, _, _, _, h1441, _, _, _, _, _, _) => {
                                h1441
                            }
                        };
                        let a1443 = 1u64;
                        nat_eq(a1442, a1443)
                    } else {
                        false
                    } {
                        let a1445 = match v3.clone() {
                            Adt2::C0(_, _, _, h1444, _, _, _, _, _, _, _) => {
                                h1444
                            }
                        };
                        let a1446 = v10;
                        nat_eq(a1445, a1446)
                    } else {
                        false
                    } {
                        let a1448 = match v3.clone() {
                            Adt2::C0(_, _, _, _, h1447, _, _, _, _, _, _) => {
                                h1447
                            }
                        };
                        let a1449 = 1u64;
                        nat_eq(a1448, a1449)
                    } else {
                        false
                    }
                }
            } else {
                false
            } {
                if f34(match v4.clone() {
                    Adt2::C0(_, h1450, _, _, _, _, _, _, _, _, _) => {
                        h1450
                    }
                }, v11, v9, v10) {
                    true
                } else {
                    let a1470 = {
                        let a1467 = {
                            let a1465 = {
                                let a1459 = {
                                    let a1453 = {
                                        let a1451 = v10;
                                        let a1452 = 1u64;
                                        nat_sub(a1451, a1452)
                                    };
                                    let a1454 = v17;
                                    nat_mul(a1453, a1454)?
                                };
                                let a1460 = {
                                    let a1457 = {
                                        let a1455 = v9;
                                        let a1456 = 1u64;
                                        nat_sub(a1455, a1456)
                                    };
                                    let a1458 = v16;
                                    nat_mul(a1457, a1458)?
                                };
                                nat_add(a1459, a1460)?
                            };
                            let a1466 = {
                                let a1463 = {
                                    let a1461 = v11;
                                    let a1462 = 1u64;
                                    nat_sub(a1461, a1462)
                                };
                                let a1464 = v15;
                                nat_mul(a1463, a1464)?
                            };
                            nat_add(a1465, a1466)?
                        };
                        let a1468 = v15;
                        nat_add(a1467, a1468)?
                    };
                    let a1471 = match v4.clone() {
                        Adt2::C0(_, _, _, _, _, _, _, _, _, h1469, _) => {
                            h1469
                        }
                    };
                    nat_le(a1470, a1471)
                }
            } else {
                false
            } {
                f23(v4.clone(), v3.clone())?
            } else {
                false
            } {
                Ok(Adt10::C4(f16(v3.clone(), 1u64)?, {
                    let a1472 = v13;
                    let a1473 = 0u64;
                    if nat_eq(a1472, a1473) {
                        4u64
                    } else {
                        2u64
                    }
                }, Adt3::C0(match v4 {
                    Adt2::C0(_, h1474, _, _, _, _, _, _, _, _, _) => {
                        h1474
                    }
                }, v11, 1u64, v10, v14, v15, 0u64, 0u64), if v5 {
                    f20(v3, 1u64)
                } else {
                    Adt3::C0(match v3.clone() {
                        Adt2::C0(_, h1475, _, _, _, _, _, _, _, _, _) => {
                            h1475
                        }
                    }, match v3.clone() {
                        Adt2::C0(_, _, h1476, _, _, _, _, _, _, _, _) => {
                            h1476
                        }
                    }, 1u64, match v3.clone() {
                        Adt2::C0(_, _, _, h1477, _, _, _, _, _, _, _) => {
                            h1477
                        }
                    }, match v3.clone() {
                        Adt2::C0(_, _, _, _, _, h1478, _, _, _, _, _) => {
                            h1478
                        }
                    }, match v3.clone() {
                        Adt2::C0(_, _, _, _, _, _, h1479, _, _, _, _) => {
                            h1479
                        }
                    }, 0u64, match v3 {
                        Adt2::C0(_, _, _, _, _, _, _, h1480, _, _, _) => {
                            h1480
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
    let m1481 = v2;
    match m1481 {
        None => {
            Ok(Adt10::C1)
        }
        Some(v4) => {
            let v5: Adt6 = Adt6::C0(match v1.clone() {
                Adt2::C0(_, h1482, _, _, _, _, _, _, _, _, _) => {
                    h1482
                }
            }, match v0.clone() {
                Adt1::C0(h1483, _, _, _, _, _, _, _, _, _, _, _) => {
                    h1483
                }
            }, match v0.clone() {
                Adt1::C0(_, _, _, h1484, _, _, _, _, _, _, _, _) => {
                    h1484
                }
            }, match v0.clone() {
                Adt1::C0(_, _, _, _, _, _, h1485, _, _, _, _, _) => {
                    h1485
                }
            });
            let v6: Adt6 = Adt6::C0(match v1.clone() {
                Adt2::C0(_, _, h1486, _, _, _, _, _, _, _, _) => {
                    h1486
                }
            }, match v0.clone() {
                Adt1::C0(_, h1487, _, _, _, _, _, _, _, _, _, _) => {
                    h1487
                }
            }, match v0.clone() {
                Adt1::C0(_, _, _, _, h1488, _, _, _, _, _, _, _) => {
                    h1488
                }
            }, match v0.clone() {
                Adt1::C0(_, _, _, _, _, _, _, h1489, _, _, _, _) => {
                    h1489
                }
            });
            let v7: Adt6 = Adt6::C0(match v1 {
                Adt2::C0(_, _, _, h1490, _, _, _, _, _, _, _) => {
                    h1490
                }
            }, match v0.clone() {
                Adt1::C0(_, _, h1491, _, _, _, _, _, _, _, _, _) => {
                    h1491
                }
            }, match v0.clone() {
                Adt1::C0(_, _, _, _, _, h1492, _, _, _, _, _, _) => {
                    h1492
                }
            }, match v0.clone() {
                Adt1::C0(_, _, _, _, _, _, _, _, h1493, _, _, _) => {
                    h1493
                }
            });
            let v8: u64 = match v0 {
                Adt1::C0(_, _, _, _, _, _, _, _, _, h1494, _, _) => {
                    h1494
                }
            };
            let v9: u64 = f100(v7.clone(), match v4.clone() {
                Adt2::C0(_, _, _, h1495, _, _, _, _, _, _, _) => {
                    h1495
                }
            })?;
            let v10: u64 = match v3.clone() {
                Adt2::C0(h1496, _, _, _, _, _, _, _, _, _, _) => {
                    h1496
                }
            };
            let v11: u64 = {
                let a1504 = v8;
                let a1505 = {
                    let a1502 = match v5.clone() {
                        Adt6::C0(h1497, _, _, _) => {
                            h1497
                        }
                    };
                    let a1503 = {
                        let a1500 = match v6.clone() {
                            Adt6::C0(h1498, _, _, _) => {
                                h1498
                            }
                        };
                        let a1501 = match v7.clone() {
                            Adt6::C0(h1499, _, _, _) => {
                                h1499
                            }
                        };
                        nat_mul(a1500, a1501)?
                    };
                    nat_mul(a1502, a1503)?
                };
                nat_mul(a1504, a1505)?
            };
            let a1507 = match v4.clone() {
                Adt2::C0(h1506, _, _, _, _, _, _, _, _, _, _) => {
                    h1506
                }
            };
            let a1508 = 0u64;
            if if if if if if if if if if if if if nat_eq(a1507, a1508) {
                let a1509 = v10;
                let a1510 = 0u64;
                if nat_eq(a1509, a1510) {
                    true
                } else {
                    let a1511 = v10;
                    let a1512 = 1u64;
                    nat_eq(a1511, a1512)
                }
            } else {
                false
            } {
                let a1513 = 0u64;
                let a1514 = v8;
                nat_lt(a1513, a1514)
            } else {
                false
            } {
                let a1515 = v8;
                let a1516 = 2147483648u64;
                nat_lt(a1515, a1516)
            } else {
                false
            } {
                let a1518 = match v4.clone() {
                    Adt2::C0(_, _, _, _, _, h1517, _, _, _, _, _) => {
                        h1517
                    }
                };
                let a1519 = 4u64;
                nat_eq(a1518, a1519)
            } else {
                false
            } {
                let a1520 = 0u64;
                let a1521 = v9;
                nat_lt(a1520, a1521)
            } else {
                false
            } {
                let a1523 = 0u64;
                let a1524 = match v3.clone() {
                    Adt2::C0(_, _, h1522, _, _, _, _, _, _, _, _) => {
                        h1522
                    }
                };
                nat_lt(a1523, a1524)
            } else {
                false
            } {
                let a1526 = 0u64;
                let a1527 = match v3.clone() {
                    Adt2::C0(_, _, _, h1525, _, _, _, _, _, _, _) => {
                        h1525
                    }
                };
                nat_lt(a1526, a1527)
            } else {
                false
            } {
                let a1529 = match v3.clone() {
                    Adt2::C0(_, h1528, _, _, _, _, _, _, _, _, _) => {
                        h1528
                    }
                };
                let a1530 = v11;
                nat_eq(a1529, a1530)
            } else {
                false
            } {
                let a1533 = match v3.clone() {
                    Adt2::C0(_, _, h1531, _, _, _, _, _, _, _, _) => {
                        h1531
                    }
                };
                let a1534 = f100(v5.clone(), match v4.clone() {
                    Adt2::C0(_, h1532, _, _, _, _, _, _, _, _, _) => {
                        h1532
                    }
                })?;
                nat_eq(a1533, a1534)
            } else {
                false
            } {
                let a1537 = match v3.clone() {
                    Adt2::C0(_, _, _, h1535, _, _, _, _, _, _, _) => {
                        h1535
                    }
                };
                let a1538 = f100(v6.clone(), match v4.clone() {
                    Adt2::C0(_, _, h1536, _, _, _, _, _, _, _, _) => {
                        h1536
                    }
                })?;
                nat_eq(a1537, a1538)
            } else {
                false
            } {
                let a1546 = match v3.clone() {
                    Adt2::C0(_, _, _, _, h1539, _, _, _, _, _, _) => {
                        h1539
                    }
                };
                let a1547 = {
                    let a1544 = v9;
                    let a1545 = {
                        let a1541 = match v4.clone() {
                            Adt2::C0(_, _, _, _, h1540, _, _, _, _, _, _) => {
                                h1540
                            }
                        };
                        let a1542 = v8;
                        let a1543 = 0u64;
                        nat_quot(a1541, a1542, a1543)
                    };
                    nat_mul(a1544, a1545)?
                };
                nat_eq(a1546, a1547)
            } else {
                false
            } {
                f23(v4.clone(), v3.clone())?
            } else {
                false
            } {
                Ok(Adt10::C4(f16(v3.clone(), 1u64)?, {
                    let a1548 = v10;
                    let a1549 = 0u64;
                    if nat_eq(a1548, a1549) {
                        4u64
                    } else {
                        2u64
                    }
                }, f20(v4.clone(), 1u64), f20(v3, 1u64), Adt4::C27(Adt5::C0(v5, v6, v7, match v4.clone() {
                    Adt2::C0(_, _, _, _, _, _, _, _, h1550, _, _) => {
                        h1550
                    }
                }, {
                    let a1552 = v8;
                    let a1553 = match v4 {
                        Adt2::C0(_, _, _, _, _, _, _, _, h1551, _, _) => {
                            h1551
                        }
                    };
                    nat_mul(a1552, a1553)?
                }, 0u64, v9, v10))))
            } else {
                Ok(Adt10::C1)
            }
        }
    }
}

pub fn f49(v0: Adt2, v1: Option<Adt2>, v2: Adt2) -> R<Adt10> {
    let m1554 = v1;
    match m1554 {
        None => {
            Ok(Adt10::C1)
        }
        Some(v3) => {
            f122(v0, v3, v2)
        }
    }
}

pub fn f50(v0: Adt2, v1: Adt2) -> R<Adt10> {
    let a1556 = match v0.clone() {
        Adt2::C0(h1555, _, _, _, _, _, _, _, _, _, _) => {
            h1555
        }
    };
    let a1557 = 1u64;
    if if nat_eq(a1556, a1557) {
        true
    } else {
        let a1559 = match v0.clone() {
            Adt2::C0(h1558, _, _, _, _, _, _, _, _, _, _) => {
                h1558
            }
        };
        let a1560 = 30u64;
        nat_eq(a1559, a1560)
    } {
        let a1562 = match v1.clone() {
            Adt2::C0(h1561, _, _, _, _, _, _, _, _, _, _) => {
                h1561
            }
        };
        let a1563 = 1u64;
        if if if if if if if if if if if nat_eq(a1562, a1563) {
            let a1566 = match v0.clone() {
                Adt2::C0(_, h1564, _, _, _, _, _, _, _, _, _) => {
                    h1564
                }
            };
            let a1567 = match v1.clone() {
                Adt2::C0(_, h1565, _, _, _, _, _, _, _, _, _) => {
                    h1565
                }
            };
            nat_eq(a1566, a1567)
        } else {
            false
        } {
            let a1569 = 0u64;
            let a1570 = match v1.clone() {
                Adt2::C0(_, _, h1568, _, _, _, _, _, _, _, _) => {
                    h1568
                }
            };
            nat_lt(a1569, a1570)
        } else {
            false
        } {
            let a1573 = match v1.clone() {
                Adt2::C0(_, _, h1571, _, _, _, _, _, _, _, _) => {
                    h1571
                }
            };
            let a1574 = match v1.clone() {
                Adt2::C0(_, _, _, h1572, _, _, _, _, _, _, _) => {
                    h1572
                }
            };
            nat_eq(a1573, a1574)
        } else {
            false
        } {
            let a1576 = match v1.clone() {
                Adt2::C0(_, _, _, _, h1575, _, _, _, _, _, _) => {
                    h1575
                }
            };
            let a1577 = 1u64;
            nat_eq(a1576, a1577)
        } else {
            false
        } {
            let a1585 = match v0.clone() {
                Adt2::C0(_, _, h1578, _, _, _, _, _, _, _, _) => {
                    h1578
                }
            };
            let a1586 = {
                let a1583 = {
                    let a1581 = match v1.clone() {
                        Adt2::C0(_, _, h1579, _, _, _, _, _, _, _, _) => {
                            h1579
                        }
                    };
                    let a1582 = match v1.clone() {
                        Adt2::C0(_, _, h1580, _, _, _, _, _, _, _, _) => {
                            h1580
                        }
                    };
                    nat_add(a1581, a1582)?
                };
                let a1584 = 1u64;
                nat_sub(a1583, a1584)
            };
            nat_eq(a1585, a1586)
        } else {
            false
        } {
            let a1588 = 0u64;
            let a1589 = match v0.clone() {
                Adt2::C0(_, _, _, h1587, _, _, _, _, _, _, _) => {
                    h1587
                }
            };
            nat_lt(a1588, a1589)
        } else {
            false
        } {
            let a1591 = 0u64;
            let a1592 = match v0.clone() {
                Adt2::C0(_, _, _, _, h1590, _, _, _, _, _, _) => {
                    h1590
                }
            };
            nat_lt(a1591, a1592)
        } else {
            false
        } {
            let a1594 = 2u64;
            let a1595 = match v0.clone() {
                Adt2::C0(_, _, _, _, _, h1593, _, _, _, _, _) => {
                    h1593
                }
            };
            nat_le(a1594, a1595)
        } else {
            false
        } {
            let a1597 = 2u64;
            let a1598 = match v1.clone() {
                Adt2::C0(_, _, _, _, _, h1596, _, _, _, _, _) => {
                    h1596
                }
            };
            nat_le(a1597, a1598)
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
    let m1599 = v1;
    match m1599 {
        None => {
            Ok(Adt10::C1)
        }
        Some(v3) => {
            f123(v0, v3, v2)
        }
    }
}

pub fn f52(v0: u64) -> bool {
    let a1600 = v0;
    let a1601 = 1u64;
    if if nat_eq(a1600, a1601) {
        true
    } else {
        let a1602 = v0;
        let a1603 = 33u64;
        nat_eq(a1602, a1603)
    } {
        true
    } else {
        let a1604 = v0;
        let a1605 = 34u64;
        nat_eq(a1604, a1605)
    }
}

pub fn f53(v0: Adt2, v1: Adt2, v2: Adt2) -> R<Adt10> {
    let v3: u64 = f16(v0.clone(), 1u64)?;
    if if if if if f33(v1, v2.clone()) {
        let a1606 = v3;
        let a1607 = f16(v2.clone(), 1u64)?;
        nat_eq(a1606, a1607)
    } else {
        false
    } {
        f124(v0.clone(), v3)?
    } else {
        false
    } {
        f124(v2.clone(), v3)?
    } else {
        false
    } {
        let a1608 = v3;
        let a1609 = 0u64;
        if nat_eq(a1608, a1609) {
            true
        } else {
            let a1611 = 4u64;
            let a1612 = match v2.clone() {
                Adt2::C0(_, _, _, _, _, h1610, _, _, _, _, _) => {
                    h1610
                }
            };
            nat_le(a1611, a1612)
        }
    } else {
        false
    } {
        Ok(Adt10::C4({
            let a1613 = 2u64;
            let a1614 = v3;
            nat_mul(a1613, a1614)?
        }, 4u64, f20(v0, 1u64), f20(v2, 1u64), Adt4::C35(v3)))
    } else {
        Ok(Adt10::C1)
    }
}

pub fn f54(v0: u64) -> bool {
    if f125(v0) {
        !f126(v0)
    } else {
        false
    }
}

pub fn f55(v0: Adt2, v1: Option<Adt2>, v2: Adt2) -> R<Adt10> {
    let m1615 = v1;
    match m1615 {
        None => {
            Ok(Adt10::C1)
        }
        Some(v3) => {
            let m1617 = f127(match v0.clone() {
                Adt2::C0(h1616, _, _, _, _, _, _, _, _, _, _) => {
                    h1616
                }
            });
            match m1617 {
                Adt17::C0 => {
                    Ok(Adt10::C0)
                }
                Adt17::C1(v4) => {
                    let v5: u64 = match v4.clone() {
                        Adt18::C0(_, h1618, _) => {
                            h1618
                        }
                    };
                    if if if if if if f33(v3, v2.clone()) {
                        f23(v0.clone(), v2.clone())?
                    } else {
                        false
                    } {
                        let a1619 = f16(v0.clone(), 1u64)?;
                        let a1620 = f16(v2.clone(), 1u64)?;
                        nat_eq(a1619, a1620)
                    } else {
                        false
                    } {
                        let a1625 = {
                            let a1622 = match v0.clone() {
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
                            let a1629 = match v4 {
                                Adt18::C0(_, _, h1627) => {
                                    h1627
                                }
                            };
                            let a1630 = match v0.clone() {
                                Adt2::C0(_, _, _, _, _, h1628, _, _, _, _, _) => {
                                    h1628
                                }
                            };
                            nat_le(a1629, a1630)
                        } else {
                            false
                        }
                    } else {
                        false
                    } {
                        let a1631 = v5;
                        let a1632 = 1u64;
                        if nat_eq(a1631, a1632) {
                            let a1634 = 4u64;
                            let a1635 = match v2.clone() {
                                Adt2::C0(_, _, _, _, _, h1633, _, _, _, _, _) => {
                                    h1633
                                }
                            };
                            nat_le(a1634, a1635)
                        } else {
                            let a1637 = match v2.clone() {
                                Adt2::C0(_, _, _, _, _, h1636, _, _, _, _, _) => {
                                    h1636
                                }
                            };
                            let a1638 = 4u64;
                            nat_eq(a1637, a1638)
                        }
                    } else {
                        false
                    } {
                        let a1643 = {
                            let a1640 = match v2.clone() {
                                Adt2::C0(_, h1639, _, _, _, _, _, _, _, _, _) => {
                                    h1639
                                }
                            };
                            let a1641 = v5;
                            let a1642 = 1u64;
                            nat_rem(a1640, a1641, a1642)
                        };
                        let a1644 = 0u64;
                        if nat_eq(a1643, a1644) {
                            true
                        } else {
                            f22(v2.clone())?
                        }
                    } else {
                        false
                    } {
                        Ok(Adt10::C4(f16(v0.clone(), v5)?, 4u64, f20(v0.clone(), v5), f20(v2, 1u64), Adt4::C11(match v0 {
                            Adt2::C0(h1645, _, _, _, _, _, _, _, _, _, _) => {
                                h1645
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

pub fn f56(v0: u64, v1: Adt2, v2: bool, v3: Adt2) -> R<Adt10> {
    let a1648 = match v1.clone() {
        Adt2::C0(h1646, _, _, _, _, _, _, _, _, _, _) => {
            h1646
        }
    };
    let a1649 = match v3.clone() {
        Adt2::C0(h1647, _, _, _, _, _, _, _, _, _, _) => {
            h1647
        }
    };
    if nat_eq(a1648, a1649) {
        f128(v0, v1, v2, v3)
    } else {
        if if if if f126(match v1.clone() {
            Adt2::C0(h1650, _, _, _, _, _, _, _, _, _, _) => {
                h1650
            }
        }) {
            f126(match v3.clone() {
                Adt2::C0(h1651, _, _, _, _, _, _, _, _, _, _) => {
                    h1651
                }
            })
        } else {
            false
        } {
            let a1652 = v0;
            let a1653 = 33u64;
            if nat_eq(a1652, a1653) {
                v2
            } else {
                true
            }
        } else {
            false
        } {
            let a1654 = f16(v1.clone(), 1u64)?;
            let a1655 = f16(v3.clone(), 1u64)?;
            nat_eq(a1654, a1655)
        } else {
            false
        } {
            let m1657 = f127(match v3.clone() {
                Adt2::C0(h1656, _, _, _, _, _, _, _, _, _, _) => {
                    h1656
                }
            });
            match m1657 {
                Adt17::C0 => {
                    Ok(Adt10::C0)
                }
                Adt17::C1(v4) => {
                    let v5: u64 = match v1.clone() {
                        Adt2::C0(h1658, _, _, _, _, _, _, _, _, _, _) => {
                            h1658
                        }
                    };
                    let v6: u64 = match v3.clone() {
                        Adt2::C0(h1659, _, _, _, _, _, _, _, _, _, _) => {
                            h1659
                        }
                    };
                    let v7: bool = {
                        let a1660 = v5;
                        let a1661 = 43u64;
                        if nat_eq(a1660, a1661) {
                            true
                        } else {
                            let a1662 = v6;
                            let a1663 = 43u64;
                            nat_eq(a1662, a1663)
                        }
                    };
                    Ok(Adt10::C4({
                        let a1664 = f16(v3.clone(), 1u64)?;
                        let a1665 = if v7 {
                            f129(v5, v6)
                        } else {
                            1u64
                        };
                        nat_mul(a1664, a1665)?
                    }, match v4 {
                        Adt18::C0(_, _, h1666) => {
                            h1666
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

pub fn f57(v0: Adt2, v1: Adt2) -> R<Adt10> {
    let a1668 = match v0.clone() {
        Adt2::C0(h1667, _, _, _, _, _, _, _, _, _, _) => {
            h1667
        }
    };
    let a1669 = 0u64;
    if if if if nat_eq(a1668, a1669) {
        true
    } else {
        let a1671 = match v0.clone() {
            Adt2::C0(h1670, _, _, _, _, _, _, _, _, _, _) => {
                h1670
            }
        };
        let a1672 = 1u64;
        nat_eq(a1671, a1672)
    } {
        true
    } else {
        let a1674 = match v0.clone() {
            Adt2::C0(h1673, _, _, _, _, _, _, _, _, _, _) => {
                h1673
            }
        };
        let a1675 = 25u64;
        if nat_eq(a1674, a1675) {
            true
        } else {
            let a1677 = match v0.clone() {
                Adt2::C0(h1676, _, _, _, _, _, _, _, _, _, _) => {
                    h1676
                }
            };
            let a1678 = 26u64;
            nat_eq(a1677, a1678)
        }
    } {
        true
    } else {
        let a1680 = match v0.clone() {
            Adt2::C0(h1679, _, _, _, _, _, _, _, _, _, _) => {
                h1679
            }
        };
        let a1681 = 30u64;
        nat_eq(a1680, a1681)
    } {
        let a1684 = match v0.clone() {
            Adt2::C0(h1682, _, _, _, _, _, _, _, _, _, _) => {
                h1682
            }
        };
        let a1685 = match v1.clone() {
            Adt2::C0(h1683, _, _, _, _, _, _, _, _, _, _) => {
                h1683
            }
        };
        if if nat_eq(a1684, a1685) {
            f130(match v1.clone() {
                Adt2::C0(_, h1686, _, _, _, _, _, _, _, _, _) => {
                    h1686
                }
            }, match v1.clone() {
                Adt2::C0(_, _, h1687, _, _, _, _, _, _, _, _) => {
                    h1687
                }
            }, match v1.clone() {
                Adt2::C0(_, _, _, h1688, _, _, _, _, _, _, _) => {
                    h1688
                }
            }, match v1.clone() {
                Adt2::C0(_, _, _, _, h1689, _, _, _, _, _, _) => {
                    h1689
                }
            }, match v0.clone() {
                Adt2::C0(_, h1690, _, _, _, _, _, _, _, _, _) => {
                    h1690
                }
            }, match v0.clone() {
                Adt2::C0(_, _, h1691, _, _, _, _, _, _, _, _) => {
                    h1691
                }
            }, match v0.clone() {
                Adt2::C0(_, _, _, h1692, _, _, _, _, _, _, _) => {
                    h1692
                }
            }, match v0.clone() {
                Adt2::C0(_, _, _, _, h1693, _, _, _, _, _, _) => {
                    h1693
                }
            })
        } else {
            false
        } {
            let m1695 = f127(match v0.clone() {
                Adt2::C0(h1694, _, _, _, _, _, _, _, _, _, _) => {
                    h1694
                }
            });
            match m1695 {
                Adt17::C0 => {
                    Ok(Adt10::C0)
                }
                Adt17::C1(v2) => {
                    Ok(Adt10::C4(f16(v1.clone(), 1u64)?, match v2 {
                        Adt18::C0(_, _, h1696) => {
                            h1696
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

pub fn f58(v0: u64, v1: Adt2, v2: Option<Adt2>, v3: Adt2) -> R<Adt10> {
    let m1697 = v2;
    match m1697 {
        None => {
            Ok(Adt10::C1)
        }
        Some(v4) => {
            let a1698 = v0;
            let a1699 = 4u64;
            if if if nat_lt(a1698, a1699) {
                let a1702 = match v1.clone() {
                    Adt2::C0(h1700, _, _, _, _, _, _, _, _, _, _) => {
                        h1700
                    }
                };
                let a1703 = match v4.clone() {
                    Adt2::C0(h1701, _, _, _, _, _, _, _, _, _, _) => {
                        h1701
                    }
                };
                if nat_eq(a1702, a1703) {
                    let a1706 = match v1.clone() {
                        Adt2::C0(h1704, _, _, _, _, _, _, _, _, _, _) => {
                            h1704
                        }
                    };
                    let a1707 = match v3.clone() {
                        Adt2::C0(h1705, _, _, _, _, _, _, _, _, _, _) => {
                            h1705
                        }
                    };
                    nat_eq(a1706, a1707)
                } else {
                    false
                }
            } else {
                false
            } {
                if if if f131(v0, 0u64, match v1.clone() {
                    Adt2::C0(_, h1708, _, _, _, _, _, _, _, _, _) => {
                        h1708
                    }
                }, match v4.clone() {
                    Adt2::C0(_, h1709, _, _, _, _, _, _, _, _, _) => {
                        h1709
                    }
                }, match v3.clone() {
                    Adt2::C0(_, h1710, _, _, _, _, _, _, _, _, _) => {
                        h1710
                    }
                })? {
                    f131(v0, 1u64, match v1.clone() {
                        Adt2::C0(_, _, h1711, _, _, _, _, _, _, _, _) => {
                            h1711
                        }
                    }, match v4.clone() {
                        Adt2::C0(_, _, h1712, _, _, _, _, _, _, _, _) => {
                            h1712
                        }
                    }, match v3.clone() {
                        Adt2::C0(_, _, h1713, _, _, _, _, _, _, _, _) => {
                            h1713
                        }
                    })?
                } else {
                    false
                } {
                    f131(v0, 2u64, match v1.clone() {
                        Adt2::C0(_, _, _, h1714, _, _, _, _, _, _, _) => {
                            h1714
                        }
                    }, match v4.clone() {
                        Adt2::C0(_, _, _, h1715, _, _, _, _, _, _, _) => {
                            h1715
                        }
                    }, match v3.clone() {
                        Adt2::C0(_, _, _, h1716, _, _, _, _, _, _, _) => {
                            h1716
                        }
                    })?
                } else {
                    false
                } {
                    f131(v0, 3u64, match v1.clone() {
                        Adt2::C0(_, _, _, _, h1717, _, _, _, _, _, _) => {
                            h1717
                        }
                    }, match v4.clone() {
                        Adt2::C0(_, _, _, _, h1718, _, _, _, _, _, _) => {
                            h1718
                        }
                    }, match v3.clone() {
                        Adt2::C0(_, _, _, _, h1719, _, _, _, _, _, _) => {
                            h1719
                        }
                    })?
                } else {
                    false
                }
            } else {
                false
            } {
                let m1721 = f127(match v1.clone() {
                    Adt2::C0(h1720, _, _, _, _, _, _, _, _, _, _) => {
                        h1720
                    }
                });
                match m1721 {
                    Adt17::C0 => {
                        Ok(Adt10::C0)
                    }
                    Adt17::C1(v5) => {
                        let v6: u64 = match v5.clone() {
                            Adt18::C0(_, h1722, _) => {
                                h1722
                            }
                        };
                        let a1727 = {
                            let a1724 = match v1.clone() {
                                Adt2::C0(_, h1723, _, _, _, _, _, _, _, _, _) => {
                                    h1723
                                }
                            };
                            let a1725 = v6;
                            let a1726 = 1u64;
                            nat_rem(a1724, a1725, a1726)
                        };
                        let a1728 = 0u64;
                        if if if nat_eq(a1727, a1728) {
                            let a1733 = {
                                let a1730 = match v4.clone() {
                                    Adt2::C0(_, h1729, _, _, _, _, _, _, _, _, _) => {
                                        h1729
                                    }
                                };
                                let a1731 = v6;
                                let a1732 = 1u64;
                                nat_rem(a1730, a1731, a1732)
                            };
                            let a1734 = 0u64;
                            nat_eq(a1733, a1734)
                        } else {
                            false
                        } {
                            let a1739 = {
                                let a1736 = match v3.clone() {
                                    Adt2::C0(_, h1735, _, _, _, _, _, _, _, _, _) => {
                                        h1735
                                    }
                                };
                                let a1737 = v6;
                                let a1738 = 1u64;
                                nat_rem(a1736, a1737, a1738)
                            };
                            let a1740 = 0u64;
                            nat_eq(a1739, a1740)
                        } else {
                            false
                        } {
                            Ok(Adt10::C4(f16(v3.clone(), v6)?, match v5 {
                                Adt18::C0(_, _, h1741) => {
                                    h1741
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

pub fn f59(v0: u64, v1: Adt1, v2: Adt2, v3: Adt2) -> R<Adt10> {
    let a1743 = match v2.clone() {
        Adt2::C0(h1742, _, _, _, _, _, _, _, _, _, _) => {
            h1742
        }
    };
    let a1744 = 0u64;
    if if nat_eq(a1743, a1744) {
        let a1746 = match v3.clone() {
            Adt2::C0(h1745, _, _, _, _, _, _, _, _, _, _) => {
                h1745
            }
        };
        let a1747 = 0u64;
        nat_eq(a1746, a1747)
    } else {
        false
    } {
        let a1748 = v0;
        let a1749 = 61u64;
        if nat_eq(a1748, a1749) {
            let v4: bool = {
                let a1751 = 0u64;
                let a1752 = match v1.clone() {
                    Adt1::C0(_, _, _, _, _, _, _, _, h1750, _, _, _) => {
                        h1750
                    }
                };
                nat_lt(a1751, a1752)
            };
            if if if if if if f132(match v2.clone() {
                Adt2::C0(_, h1753, _, _, _, _, _, _, _, _, _) => {
                    h1753
                }
            }, match v1.clone() {
                Adt1::C0(h1754, _, _, _, _, _, _, _, _, _, _, _) => {
                    h1754
                }
            }, match v1.clone() {
                Adt1::C0(_, h1755, _, _, _, _, _, _, _, _, _, _) => {
                    h1755
                }
            }, match v3.clone() {
                Adt2::C0(_, h1756, _, _, _, _, _, _, _, _, _) => {
                    h1756
                }
            })? {
                f132(match v2.clone() {
                    Adt2::C0(_, _, h1757, _, _, _, _, _, _, _, _) => {
                        h1757
                    }
                }, match v1.clone() {
                    Adt1::C0(_, _, h1758, _, _, _, _, _, _, _, _, _) => {
                        h1758
                    }
                }, match v1.clone() {
                    Adt1::C0(_, _, _, h1759, _, _, _, _, _, _, _, _) => {
                        h1759
                    }
                }, match v3.clone() {
                    Adt2::C0(_, _, h1760, _, _, _, _, _, _, _, _) => {
                        h1760
                    }
                })?
            } else {
                false
            } {
                f132(match v2.clone() {
                    Adt2::C0(_, _, _, h1761, _, _, _, _, _, _, _) => {
                        h1761
                    }
                }, match v1.clone() {
                    Adt1::C0(_, _, _, _, h1762, _, _, _, _, _, _, _) => {
                        h1762
                    }
                }, match v1.clone() {
                    Adt1::C0(_, _, _, _, _, h1763, _, _, _, _, _, _) => {
                        h1763
                    }
                }, match v3.clone() {
                    Adt2::C0(_, _, _, h1764, _, _, _, _, _, _, _) => {
                        h1764
                    }
                })?
            } else {
                false
            } {
                f132(match v2.clone() {
                    Adt2::C0(_, _, _, _, h1765, _, _, _, _, _, _) => {
                        h1765
                    }
                }, match v1.clone() {
                    Adt1::C0(_, _, _, _, _, _, h1766, _, _, _, _, _) => {
                        h1766
                    }
                }, match v1.clone() {
                    Adt1::C0(_, _, _, _, _, _, _, h1767, _, _, _, _) => {
                        h1767
                    }
                }, match v3.clone() {
                    Adt2::C0(_, _, _, _, h1768, _, _, _, _, _, _) => {
                        h1768
                    }
                })?
            } else {
                false
            } {
                let a1770 = match v1.clone() {
                    Adt1::C0(_, _, _, _, _, _, _, _, h1769, _, _, _) => {
                        h1769
                    }
                };
                let a1771 = 4294967295u64;
                nat_le(a1770, a1771)
            } else {
                false
            } {
                if v4 {
                    !f34(match v2.clone() {
                        Adt2::C0(_, h1772, _, _, _, _, _, _, _, _, _) => {
                            h1772
                        }
                    }, match v2.clone() {
                        Adt2::C0(_, _, h1773, _, _, _, _, _, _, _, _) => {
                            h1773
                        }
                    }, match v2.clone() {
                        Adt2::C0(_, _, _, h1774, _, _, _, _, _, _, _) => {
                            h1774
                        }
                    }, match v2.clone() {
                        Adt2::C0(_, _, _, _, h1775, _, _, _, _, _, _) => {
                            h1775
                        }
                    })
                } else {
                    true
                }
            } else {
                false
            } {
                Ok(Adt10::C4(f16(v3.clone(), 1u64)?, 4u64, f20(v2, 1u64), f20(v3, 1u64), Adt4::C3(match v1.clone() {
                    Adt1::C0(h1776, _, _, _, _, _, _, _, _, _, _, _) => {
                        h1776
                    }
                }, match v1.clone() {
                    Adt1::C0(_, _, h1777, _, _, _, _, _, _, _, _, _) => {
                        h1777
                    }
                }, match v1.clone() {
                    Adt1::C0(_, _, _, _, h1778, _, _, _, _, _, _, _) => {
                        h1778
                    }
                }, match v1 {
                    Adt1::C0(_, _, _, _, _, _, h1779, _, _, _, _, _) => {
                        h1779
                    }
                }, v4)))
            } else {
                Ok(Adt10::C1)
            }
        } else {
            let a1780 = v0;
            let a1781 = 62u64;
            if nat_eq(a1780, a1781) {
                let a1783 = match v1.clone() {
                    Adt1::C0(h1782, _, _, _, _, _, _, _, _, _, _, _) => {
                        h1782
                    }
                };
                let a1784 = 2147483648u64;
                if if if if if nat_lt(a1783, a1784) {
                    let a1786 = match v1.clone() {
                        Adt1::C0(_, h1785, _, _, _, _, _, _, _, _, _, _) => {
                            h1785
                        }
                    };
                    let a1787 = 2147483648u64;
                    nat_lt(a1786, a1787)
                } else {
                    false
                } {
                    let a1790 = match v1.clone() {
                        Adt1::C0(h1788, _, _, _, _, _, _, _, _, _, _, _) => {
                            h1788
                        }
                    };
                    let a1791 = match v2.clone() {
                        Adt2::C0(_, h1789, _, _, _, _, _, _, _, _, _) => {
                            h1789
                        }
                    };
                    nat_lt(a1790, a1791)
                } else {
                    false
                } {
                    let a1794 = match v1.clone() {
                        Adt1::C0(_, h1792, _, _, _, _, _, _, _, _, _, _) => {
                            h1792
                        }
                    };
                    let a1795 = match v2.clone() {
                        Adt2::C0(_, h1793, _, _, _, _, _, _, _, _, _) => {
                            h1793
                        }
                    };
                    nat_lt(a1794, a1795)
                } else {
                    false
                } {
                    let a1798 = match v2.clone() {
                        Adt2::C0(_, _, h1796, _, _, _, _, _, _, _, _) => {
                            h1796
                        }
                    };
                    let a1799 = match v3.clone() {
                        Adt2::C0(_, _, h1797, _, _, _, _, _, _, _, _) => {
                            h1797
                        }
                    };
                    if if if nat_eq(a1798, a1799) {
                        let a1802 = match v2.clone() {
                            Adt2::C0(_, _, _, h1800, _, _, _, _, _, _, _) => {
                                h1800
                            }
                        };
                        let a1803 = match v3.clone() {
                            Adt2::C0(_, _, _, h1801, _, _, _, _, _, _, _) => {
                                h1801
                            }
                        };
                        nat_eq(a1802, a1803)
                    } else {
                        false
                    } {
                        let a1806 = match v2.clone() {
                            Adt2::C0(_, _, _, _, h1804, _, _, _, _, _, _) => {
                                h1804
                            }
                        };
                        let a1807 = match v3.clone() {
                            Adt2::C0(_, _, _, _, h1805, _, _, _, _, _, _) => {
                                h1805
                            }
                        };
                        nat_eq(a1806, a1807)
                    } else {
                        false
                    } {
                        let a1816 = match v3.clone() {
                            Adt2::C0(_, h1808, _, _, _, _, _, _, _, _, _) => {
                                h1808
                            }
                        };
                        let a1817 = {
                            let a1814 = {
                                let a1811 = match v2.clone() {
                                    Adt2::C0(_, h1809, _, _, _, _, _, _, _, _, _) => {
                                        h1809
                                    }
                                };
                                let a1812 = match v1.clone() {
                                    Adt1::C0(h1810, _, _, _, _, _, _, _, _, _, _, _) => {
                                        h1810
                                    }
                                };
                                nat_add(a1811, a1812)?
                            };
                            let a1815 = match v1.clone() {
                                Adt1::C0(_, h1813, _, _, _, _, _, _, _, _, _, _) => {
                                    h1813
                                }
                            };
                            nat_add(a1814, a1815)?
                        };
                        nat_eq(a1816, a1817)
                    } else {
                        false
                    }
                } else {
                    false
                } {
                    Ok(Adt10::C4(f16(v3.clone(), 1u64)?, 4u64, f20(v2, 1u64), f20(v3, 1u64), Adt4::C4(match v1 {
                        Adt1::C0(h1818, _, _, _, _, _, _, _, _, _, _, _) => {
                            h1818
                        }
                    })))
                } else {
                    Ok(Adt10::C1)
                }
            } else {
                let a1819 = v0;
                let a1820 = 63u64;
                if nat_eq(a1819, a1820) {
                    if if if if if f133(match v2.clone() {
                        Adt2::C0(_, h1821, _, _, _, _, _, _, _, _, _) => {
                            h1821
                        }
                    }, match v1.clone() {
                        Adt1::C0(h1822, _, _, _, _, _, _, _, _, _, _, _) => {
                            h1822
                        }
                    })? {
                        f133(match v2.clone() {
                            Adt2::C0(_, _, h1823, _, _, _, _, _, _, _, _) => {
                                h1823
                            }
                        }, match v1.clone() {
                            Adt1::C0(_, h1824, _, _, _, _, _, _, _, _, _, _) => {
                                h1824
                            }
                        })?
                    } else {
                        false
                    } {
                        f133(match v2.clone() {
                            Adt2::C0(_, _, _, h1825, _, _, _, _, _, _, _) => {
                                h1825
                            }
                        }, match v1.clone() {
                            Adt1::C0(_, _, h1826, _, _, _, _, _, _, _, _, _) => {
                                h1826
                            }
                        })?
                    } else {
                        false
                    } {
                        f133(match v2.clone() {
                            Adt2::C0(_, _, _, _, h1827, _, _, _, _, _, _) => {
                                h1827
                            }
                        }, match v1.clone() {
                            Adt1::C0(_, _, _, h1828, _, _, _, _, _, _, _, _) => {
                                h1828
                            }
                        })?
                    } else {
                        false
                    } {
                        let a1831 = match v2.clone() {
                            Adt2::C0(_, h1829, _, _, _, _, _, _, _, _, _) => {
                                h1829
                            }
                        };
                        let a1832 = match v3.clone() {
                            Adt2::C0(_, h1830, _, _, _, _, _, _, _, _, _) => {
                                h1830
                            }
                        };
                        if if if nat_eq(a1831, a1832) {
                            let a1835 = match v2.clone() {
                                Adt2::C0(_, _, h1833, _, _, _, _, _, _, _, _) => {
                                    h1833
                                }
                            };
                            let a1836 = match v3.clone() {
                                Adt2::C0(_, _, h1834, _, _, _, _, _, _, _, _) => {
                                    h1834
                                }
                            };
                            nat_eq(a1835, a1836)
                        } else {
                            false
                        } {
                            let a1839 = match v2.clone() {
                                Adt2::C0(_, _, _, h1837, _, _, _, _, _, _, _) => {
                                    h1837
                                }
                            };
                            let a1840 = match v3.clone() {
                                Adt2::C0(_, _, _, h1838, _, _, _, _, _, _, _) => {
                                    h1838
                                }
                            };
                            nat_eq(a1839, a1840)
                        } else {
                            false
                        } {
                            let a1843 = match v2.clone() {
                                Adt2::C0(_, _, _, _, h1841, _, _, _, _, _, _) => {
                                    h1841
                                }
                            };
                            let a1844 = match v3.clone() {
                                Adt2::C0(_, _, _, _, h1842, _, _, _, _, _, _) => {
                                    h1842
                                }
                            };
                            nat_eq(a1843, a1844)
                        } else {
                            false
                        }
                    } else {
                        false
                    } {
                        Ok(Adt10::C4(f16(v3.clone(), 1u64)?, 4u64, f20(v2, 1u64), f20(v3, 1u64), Adt4::C5(match v1.clone() {
                            Adt1::C0(h1845, _, _, _, _, _, _, _, _, _, _, _) => {
                                h1845
                            }
                        }, match v1.clone() {
                            Adt1::C0(_, h1846, _, _, _, _, _, _, _, _, _, _) => {
                                h1846
                            }
                        }, match v1.clone() {
                            Adt1::C0(_, _, h1847, _, _, _, _, _, _, _, _, _) => {
                                h1847
                            }
                        }, match v1 {
                            Adt1::C0(_, _, _, h1848, _, _, _, _, _, _, _, _) => {
                                h1848
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

pub fn f60(v0: u64, v1: Adt1, v2: Adt2, v3: Option<Adt2>, v4: Adt2) -> R<Adt10> {
    let m1849 = v3;
    match m1849 {
        None => {
            Ok(Adt10::C1)
        }
        Some(v5) => {
            let v6: bool = {
                let a1850 = v0;
                let a1851 = 4u64;
                nat_eq(a1850, a1851)
            };
            let v7: bool = {
                let a1853 = 0u64;
                let a1854 = match v1.clone() {
                    Adt1::C0(_, _, _, _, h1852, _, _, _, _, _, _, _) => {
                        h1852
                    }
                };
                nat_lt(a1853, a1854)
            };
            let v8: u64 = f16(v5.clone(), 1u64)?;
            let v9: Adt3 = Adt3::C0(match v5.clone() {
                Adt2::C0(_, h1855, _, _, _, _, _, _, _, _, _) => {
                    h1855
                }
            }, match v5.clone() {
                Adt2::C0(_, _, h1856, _, _, _, _, _, _, _, _) => {
                    h1856
                }
            }, match v5.clone() {
                Adt2::C0(_, _, _, h1857, _, _, _, _, _, _, _) => {
                    h1857
                }
            }, match v5.clone() {
                Adt2::C0(_, _, _, _, h1858, _, _, _, _, _, _) => {
                    h1858
                }
            }, 4u64, match v1.clone() {
                Adt1::C0(h1859, _, _, _, _, _, _, _, _, _, _, _) => {
                    h1859
                }
            }, match v1.clone() {
                Adt1::C0(_, h1860, _, _, _, _, _, _, _, _, _, _) => {
                    h1860
                }
            }, match v1.clone() {
                Adt1::C0(_, _, h1861, _, _, _, _, _, _, _, _, _) => {
                    h1861
                }
            });
            if if if if if if if if if if if if v6 {
                let a1863 = match v2.clone() {
                    Adt2::C0(h1862, _, _, _, _, _, _, _, _, _, _) => {
                        h1862
                    }
                };
                let a1864 = 0u64;
                if nat_eq(a1863, a1864) {
                    let a1866 = match v5.clone() {
                        Adt2::C0(h1865, _, _, _, _, _, _, _, _, _, _) => {
                            h1865
                        }
                    };
                    let a1867 = 0u64;
                    nat_eq(a1866, a1867)
                } else {
                    false
                }
            } else {
                let a1869 = match v2.clone() {
                    Adt2::C0(h1868, _, _, _, _, _, _, _, _, _, _) => {
                        h1868
                    }
                };
                let a1870 = 0u64;
                if if nat_eq(a1869, a1870) {
                    true
                } else {
                    let a1872 = match v2.clone() {
                        Adt2::C0(h1871, _, _, _, _, _, _, _, _, _, _) => {
                            h1871
                        }
                    };
                    let a1873 = 26u64;
                    nat_eq(a1872, a1873)
                } {
                    let a1875 = match v5.clone() {
                        Adt2::C0(h1874, _, _, _, _, _, _, _, _, _, _) => {
                            h1874
                        }
                    };
                    let a1876 = 0u64;
                    if nat_eq(a1875, a1876) {
                        true
                    } else {
                        let a1878 = match v5.clone() {
                            Adt2::C0(h1877, _, _, _, _, _, _, _, _, _, _) => {
                                h1877
                            }
                        };
                        let a1879 = 26u64;
                        nat_eq(a1878, a1879)
                    }
                } else {
                    false
                }
            } {
                let a1882 = match v2.clone() {
                    Adt2::C0(h1880, _, _, _, _, _, _, _, _, _, _) => {
                        h1880
                    }
                };
                let a1883 = match v4.clone() {
                    Adt2::C0(h1881, _, _, _, _, _, _, _, _, _, _) => {
                        h1881
                    }
                };
                nat_eq(a1882, a1883)
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
                let a1885 = match v5.clone() {
                    Adt2::C0(_, _, _, _, _, h1884, _, _, _, _, _) => {
                        h1884
                    }
                };
                let a1886 = 4u64;
                nat_eq(a1885, a1886)
            } else {
                false
            } {
                let a1887 = v8;
                let a1888 = f16(v2.clone(), 1u64)?;
                nat_le(a1887, a1888)
            } else {
                false
            } {
                let a1890 = match v1.clone() {
                    Adt1::C0(_, _, _, h1889, _, _, _, _, _, _, _, _) => {
                        h1889
                    }
                };
                let a1891 = 1073741824u64;
                nat_lt(a1890, a1891)
            } else {
                false
            } {
                let a1893 = match v1.clone() {
                    Adt1::C0(h1892, _, _, _, _, _, _, _, _, _, _, _) => {
                        h1892
                    }
                };
                let a1894 = 4294967295u64;
                if if if if nat_le(a1893, a1894) {
                    let a1896 = match v1.clone() {
                        Adt1::C0(_, h1895, _, _, _, _, _, _, _, _, _, _) => {
                            h1895
                        }
                    };
                    let a1897 = 4294967295u64;
                    nat_le(a1896, a1897)
                } else {
                    false
                } {
                    let a1899 = match v1.clone() {
                        Adt1::C0(_, _, h1898, _, _, _, _, _, _, _, _, _) => {
                            h1898
                        }
                    };
                    let a1900 = 4294967295u64;
                    nat_le(a1899, a1900)
                } else {
                    false
                } {
                    let a1902 = match v1.clone() {
                        Adt1::C0(_, _, _, h1901, _, _, _, _, _, _, _, _) => {
                            h1901
                        }
                    };
                    let a1903 = 4294967295u64;
                    nat_le(a1902, a1903)
                } else {
                    false
                } {
                    let a1905 = match v1.clone() {
                        Adt1::C0(_, _, _, _, h1904, _, _, _, _, _, _, _) => {
                            h1904
                        }
                    };
                    let a1906 = 4294967295u64;
                    nat_le(a1905, a1906)
                } else {
                    false
                }
            } else {
                false
            } {
                if v7 {
                    let a1909 = match v2.clone() {
                        Adt2::C0(_, _, _, _, _, _, _, _, _, _, h1907) => {
                            h1907
                        }
                    };
                    let a1910 = match v4.clone() {
                        Adt2::C0(_, _, _, _, _, _, _, _, _, _, h1908) => {
                            h1908
                        }
                    };
                    nat_eq(a1909, a1910)
                } else {
                    true
                }
            } else {
                false
            } {
                f134(v9.clone(), match v1.clone() {
                    Adt1::C0(_, _, _, h1911, _, _, _, _, _, _, _, _) => {
                        h1911
                    }
                }, match v4 {
                    Adt2::C0(_, _, _, _, _, _, _, _, _, h1912, _) => {
                        h1912
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
                    let a1913 = v10;
                    let a1914 = v8;
                    nat_add(a1913, a1914)?
                }, 4u64, f20(v5, 1u64), v9, Adt4::C6(v10, match v1 {
                    Adt1::C0(_, _, _, h1915, _, _, _, _, _, _, _, _) => {
                        h1915
                    }
                }, v6)))
            } else {
                Ok(Adt10::C1)
            }
        }
    }
}

pub fn f61(v0: Adt2, v1: Adt2) -> R<Adt10> {
    let a1917 = match v0.clone() {
        Adt2::C0(h1916, _, _, _, _, _, _, _, _, _, _) => {
            h1916
        }
    };
    let a1918 = 0u64;
    if if if nat_eq(a1917, a1918) {
        let a1920 = match v1.clone() {
            Adt2::C0(h1919, _, _, _, _, _, _, _, _, _, _) => {
                h1919
            }
        };
        let a1921 = 0u64;
        nat_eq(a1920, a1921)
    } else {
        false
    } {
        f130(match v0.clone() {
            Adt2::C0(_, h1922, _, _, _, _, _, _, _, _, _) => {
                h1922
            }
        }, match v0.clone() {
            Adt2::C0(_, _, h1923, _, _, _, _, _, _, _, _) => {
                h1923
            }
        }, match v0.clone() {
            Adt2::C0(_, _, _, h1924, _, _, _, _, _, _, _) => {
                h1924
            }
        }, match v0.clone() {
            Adt2::C0(_, _, _, _, h1925, _, _, _, _, _, _) => {
                h1925
            }
        }, match v1.clone() {
            Adt2::C0(_, h1926, _, _, _, _, _, _, _, _, _) => {
                h1926
            }
        }, match v1.clone() {
            Adt2::C0(_, _, h1927, _, _, _, _, _, _, _, _) => {
                h1927
            }
        }, match v1.clone() {
            Adt2::C0(_, _, _, h1928, _, _, _, _, _, _, _) => {
                h1928
            }
        }, match v1.clone() {
            Adt2::C0(_, _, _, _, h1929, _, _, _, _, _, _) => {
                h1929
            }
        })
    } else {
        false
    } {
        let v2: u64 = f16(v1.clone(), 1u64)?;
        Ok(Adt10::C4({
            let a1930 = v2;
            let a1931 = f16(v0.clone(), 1u64)?;
            nat_add(a1930, a1931)?
        }, 4u64, f20(v0, 1u64), f20(v1, 1u64), Adt4::C7(v2)))
    } else {
        Ok(Adt10::C1)
    }
}

pub fn f62(v0: u64, v1: Adt2, v2: Adt2) -> R<Adt10> {
    let a1933 = match v1.clone() {
        Adt2::C0(h1932, _, _, _, _, _, _, _, _, _, _) => {
            h1932
        }
    };
    let a1934 = 0u64;
    if if if nat_eq(a1933, a1934) {
        true
    } else {
        let a1936 = match v1.clone() {
            Adt2::C0(h1935, _, _, _, _, _, _, _, _, _, _) => {
                h1935
            }
        };
        let a1937 = 1u64;
        nat_eq(a1936, a1937)
    } {
        true
    } else {
        let a1939 = match v1.clone() {
            Adt2::C0(h1938, _, _, _, _, _, _, _, _, _, _) => {
                h1938
            }
        };
        let a1940 = 30u64;
        nat_eq(a1939, a1940)
    } {
        let a1943 = match v1.clone() {
            Adt2::C0(h1941, _, _, _, _, _, _, _, _, _, _) => {
                h1941
            }
        };
        let a1944 = match v2.clone() {
            Adt2::C0(h1942, _, _, _, _, _, _, _, _, _, _) => {
                h1942
            }
        };
        if if if if if nat_eq(a1943, a1944) {
            let a1947 = match v1.clone() {
                Adt2::C0(_, h1945, _, _, _, _, _, _, _, _, _) => {
                    h1945
                }
            };
            let a1948 = match v2.clone() {
                Adt2::C0(_, h1946, _, _, _, _, _, _, _, _, _) => {
                    h1946
                }
            };
            nat_eq(a1947, a1948)
        } else {
            false
        } {
            let a1951 = match v1.clone() {
                Adt2::C0(_, _, _, h1949, _, _, _, _, _, _, _) => {
                    h1949
                }
            };
            let a1952 = match v2.clone() {
                Adt2::C0(_, _, _, h1950, _, _, _, _, _, _, _) => {
                    h1950
                }
            };
            nat_eq(a1951, a1952)
        } else {
            false
        } {
            let a1955 = match v1.clone() {
                Adt2::C0(_, _, _, _, h1953, _, _, _, _, _, _) => {
                    h1953
                }
            };
            let a1956 = match v2.clone() {
                Adt2::C0(_, _, _, _, h1954, _, _, _, _, _, _) => {
                    h1954
                }
            };
            nat_eq(a1955, a1956)
        } else {
            false
        } {
            let a1957 = v0;
            let a1958 = 4u64;
            if nat_eq(a1957, a1958) {
                let a1960 = match v1.clone() {
                    Adt2::C0(_, _, h1959, _, _, _, _, _, _, _, _) => {
                        h1959
                    }
                };
                let a1961 = 1u64;
                if nat_eq(a1960, a1961) {
                    let a1964 = match v1.clone() {
                        Adt2::C0(_, h1962, _, _, _, _, _, _, _, _, _) => {
                            h1962
                        }
                    };
                    let a1965 = match v2.clone() {
                        Adt2::C0(_, _, h1963, _, _, _, _, _, _, _, _) => {
                            h1963
                        }
                    };
                    nat_eq(a1964, a1965)
                } else {
                    false
                }
            } else {
                let a1966 = v0;
                let a1967 = 4u64;
                if if nat_lt(a1966, a1967) {
                    let a1970 = match v1.clone() {
                        Adt2::C0(_, h1968, _, _, _, _, _, _, _, _, _) => {
                            h1968
                        }
                    };
                    let a1971 = match v1.clone() {
                        Adt2::C0(_, _, h1969, _, _, _, _, _, _, _, _) => {
                            h1969
                        }
                    };
                    nat_eq(a1970, a1971)
                } else {
                    false
                } {
                    let a1974 = match v1.clone() {
                        Adt2::C0(_, _, h1972, _, _, _, _, _, _, _, _) => {
                            h1972
                        }
                    };
                    let a1975 = match v2.clone() {
                        Adt2::C0(_, _, h1973, _, _, _, _, _, _, _, _) => {
                            h1973
                        }
                    };
                    nat_eq(a1974, a1975)
                } else {
                    false
                }
            }
        } else {
            false
        } {
            let m1977 = f127(match v1.clone() {
                Adt2::C0(h1976, _, _, _, _, _, _, _, _, _, _) => {
                    h1976
                }
            });
            match m1977 {
                Adt17::C0 => {
                    Ok(Adt10::C0)
                }
                Adt17::C1(v3) => {
                    Ok(Adt10::C4(f16(v2.clone(), 1u64)?, match v3 {
                        Adt18::C0(_, _, h1978) => {
                            h1978
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

pub fn f63(v0: u64, v1: Adt2, v2: Adt2) -> R<Adt10> {
    if if if f135(match v1.clone() {
        Adt2::C0(h1979, _, _, _, _, _, _, _, _, _, _) => {
            h1979
        }
    }, v0) {
        let a1982 = match v1.clone() {
            Adt2::C0(h1980, _, _, _, _, _, _, _, _, _, _) => {
                h1980
            }
        };
        let a1983 = match v2.clone() {
            Adt2::C0(h1981, _, _, _, _, _, _, _, _, _, _) => {
                h1981
            }
        };
        nat_eq(a1982, a1983)
    } else {
        false
    } {
        let a1986 = match v1.clone() {
            Adt2::C0(_, h1984, _, _, _, _, _, _, _, _, _) => {
                h1984
            }
        };
        let a1987 = match v2.clone() {
            Adt2::C0(_, h1985, _, _, _, _, _, _, _, _, _) => {
                h1985
            }
        };
        if if if nat_eq(a1986, a1987) {
            let a1990 = match v1.clone() {
                Adt2::C0(_, _, h1988, _, _, _, _, _, _, _, _) => {
                    h1988
                }
            };
            let a1991 = match v2.clone() {
                Adt2::C0(_, _, h1989, _, _, _, _, _, _, _, _) => {
                    h1989
                }
            };
            nat_eq(a1990, a1991)
        } else {
            false
        } {
            let a1994 = match v1.clone() {
                Adt2::C0(_, _, _, h1992, _, _, _, _, _, _, _) => {
                    h1992
                }
            };
            let a1995 = match v2.clone() {
                Adt2::C0(_, _, _, h1993, _, _, _, _, _, _, _) => {
                    h1993
                }
            };
            nat_eq(a1994, a1995)
        } else {
            false
        } {
            let a1998 = match v1.clone() {
                Adt2::C0(_, _, _, _, h1996, _, _, _, _, _, _) => {
                    h1996
                }
            };
            let a1999 = match v2.clone() {
                Adt2::C0(_, _, _, _, h1997, _, _, _, _, _, _) => {
                    h1997
                }
            };
            nat_eq(a1998, a1999)
        } else {
            false
        }
    } else {
        false
    } {
        let m2001 = f127(match v1.clone() {
            Adt2::C0(h2000, _, _, _, _, _, _, _, _, _, _) => {
                h2000
            }
        });
        match m2001 {
            Adt17::C0 => {
                Ok(Adt10::C0)
            }
            Adt17::C1(v3) => {
                Ok(Adt10::C4(f16(v2.clone(), 1u64)?, match v3 {
                    Adt18::C0(_, _, h2002) => {
                        h2002
                    }
                }, f20(v1.clone(), 1u64), f20(v2, 1u64), Adt4::C9(match v1 {
                    Adt2::C0(h2003, _, _, _, _, _, _, _, _, _, _) => {
                        h2003
                    }
                }, v0)))
            }
        }
    } else {
        Ok(Adt10::C1)
    }
}

pub fn f64(v0: bool, v1: Adt1, v2: Adt2, v3: Adt2) -> R<Adt10> {
    let v4: u64 = if v0 {
        match v1.clone() {
            Adt1::C0(h2004, _, _, _, _, _, _, _, _, _, _, _) => {
                h2004
            }
        }
    } else {
        match v1.clone() {
            Adt1::C0(_, _, h2005, _, _, _, _, _, _, _, _, _) => {
                h2005
            }
        }
    };
    let a2006 = 0u64;
    let a2007 = v4;
    if if nat_lt(a2006, a2007) {
        let a2008 = v4;
        let a2009 = 2147483648u64;
        nat_lt(a2008, a2009)
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
        let v7: u64 = f136(match v5.clone() {
            Adt2::C0(_, _, h2010, _, _, _, _, _, _, _, _) => {
                h2010
            }
        }, v4)?;
        let v8: u64 = f136(match v5.clone() {
            Adt2::C0(_, _, _, h2011, _, _, _, _, _, _, _) => {
                h2011
            }
        }, v4)?;
        let a2013 = match v5.clone() {
            Adt2::C0(h2012, _, _, _, _, _, _, _, _, _, _) => {
                h2012
            }
        };
        let a2014 = 0u64;
        if if if if if nat_eq(a2013, a2014) {
            let a2016 = match v6.clone() {
                Adt2::C0(h2015, _, _, _, _, _, _, _, _, _, _) => {
                    h2015
                }
            };
            let a2017 = 0u64;
            nat_eq(a2016, a2017)
        } else {
            false
        } {
            let a2019 = match v5.clone() {
                Adt2::C0(_, _, _, _, h2018, _, _, _, _, _, _) => {
                    h2018
                }
            };
            let a2020 = 1u64;
            nat_eq(a2019, a2020)
        } else {
            false
        } {
            let a2023 = match v5.clone() {
                Adt2::C0(_, h2021, _, _, _, _, _, _, _, _, _) => {
                    h2021
                }
            };
            let a2024 = match v6.clone() {
                Adt2::C0(_, h2022, _, _, _, _, _, _, _, _, _) => {
                    h2022
                }
            };
            nat_eq(a2023, a2024)
        } else {
            false
        } {
            if v0 {
                let a2026 = match v5.clone() {
                    Adt2::C0(_, _, h2025, _, _, _, _, _, _, _, _) => {
                        h2025
                    }
                };
                let a2027 = 2147483648u64;
                if if if nat_lt(a2026, a2027) {
                    let a2029 = match v5.clone() {
                        Adt2::C0(_, _, _, h2028, _, _, _, _, _, _, _) => {
                            h2028
                        }
                    };
                    let a2030 = 2147483648u64;
                    nat_lt(a2029, a2030)
                } else {
                    false
                } {
                    let a2033 = {
                        let a2031 = v7;
                        let a2032 = v8;
                        nat_mul(a2031, a2032)?
                    };
                    let a2034 = 2147483648u64;
                    nat_le(a2033, a2034)
                } else {
                    false
                } {
                    f137(v5, v6, v4, v7)?
                } else {
                    false
                }
            } else {
                let a2036 = match v1.clone() {
                    Adt1::C0(h2035, _, _, _, _, _, _, _, _, _, _, _) => {
                        h2035
                    }
                };
                let a2037 = v7;
                if if if if nat_eq(a2036, a2037) {
                    let a2039 = match v1 {
                        Adt1::C0(_, h2038, _, _, _, _, _, _, _, _, _, _) => {
                            h2038
                        }
                    };
                    let a2040 = v8;
                    nat_eq(a2039, a2040)
                } else {
                    false
                } {
                    f138(v7, v8, v4)?
                } else {
                    false
                } {
                    let a2042 = match v6.clone() {
                        Adt2::C0(_, _, h2041, _, _, _, _, _, _, _, _) => {
                            h2041
                        }
                    };
                    let a2043 = v4;
                    if nat_eq(a2042, a2043) {
                        let a2045 = match v6.clone() {
                            Adt2::C0(_, _, _, h2044, _, _, _, _, _, _, _) => {
                                h2044
                            }
                        };
                        let a2046 = v4;
                        nat_eq(a2045, a2046)
                    } else {
                        false
                    }
                } else {
                    false
                } {
                    let a2050 = match v6 {
                        Adt2::C0(_, _, _, _, h2047, _, _, _, _, _, _) => {
                            h2047
                        }
                    };
                    let a2051 = {
                        let a2048 = v7;
                        let a2049 = v8;
                        nat_mul(a2048, a2049)?
                    };
                    nat_eq(a2050, a2051)
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

pub fn f65(v0: u64, v1: Adt1, v2: Adt2, v3: Adt2) -> R<Adt10> {
    let a2053 = match v2.clone() {
        Adt2::C0(h2052, _, _, _, _, _, _, _, _, _, _) => {
            h2052
        }
    };
    let a2054 = 0u64;
    if if if if if nat_eq(a2053, a2054) {
        true
    } else {
        let a2056 = match v2.clone() {
            Adt2::C0(h2055, _, _, _, _, _, _, _, _, _, _) => {
                h2055
            }
        };
        let a2057 = 1u64;
        nat_eq(a2056, a2057)
    } {
        true
    } else {
        let a2059 = match v2.clone() {
            Adt2::C0(h2058, _, _, _, _, _, _, _, _, _, _) => {
                h2058
            }
        };
        let a2060 = 30u64;
        nat_eq(a2059, a2060)
    } {
        let a2063 = match v2.clone() {
            Adt2::C0(h2061, _, _, _, _, _, _, _, _, _, _) => {
                h2061
            }
        };
        let a2064 = match v3.clone() {
            Adt2::C0(h2062, _, _, _, _, _, _, _, _, _, _) => {
                h2062
            }
        };
        nat_eq(a2063, a2064)
    } else {
        false
    } {
        if if f25(v2.clone(), v3.clone()) {
            let a2066 = match v1.clone() {
                Adt1::C0(h2065, _, _, _, _, _, _, _, _, _, _, _) => {
                    h2065
                }
            };
            let a2067 = 4294967296u64;
            nat_lt(a2066, a2067)
        } else {
            false
        } {
            let a2069 = match v1.clone() {
                Adt1::C0(_, h2068, _, _, _, _, _, _, _, _, _, _) => {
                    h2068
                }
            };
            let a2070 = 4294967296u64;
            nat_lt(a2069, a2070)
        } else {
            false
        }
    } else {
        false
    } {
        Ok(Adt10::C4(f16(v3.clone(), 1u64)?, {
            let a2072 = match v2.clone() {
                Adt2::C0(h2071, _, _, _, _, _, _, _, _, _, _) => {
                    h2071
                }
            };
            let a2073 = 0u64;
            if nat_eq(a2072, a2073) {
                4u64
            } else {
                2u64
            }
        }, f20(v2.clone(), 1u64), f20(v3, 1u64), Adt4::C13({
            let a2074 = v0;
            let a2075 = 31u64;
            if nat_eq(a2074, a2075) {
                0u64
            } else {
                let a2076 = v0;
                let a2077 = 47u64;
                if nat_eq(a2076, a2077) {
                    1u64
                } else {
                    2u64
                }
            }
        }, match v2 {
            Adt2::C0(h2078, _, _, _, _, _, _, _, _, _, _) => {
                h2078
            }
        }, match v1.clone() {
            Adt1::C0(h2079, _, _, _, _, _, _, _, _, _, _, _) => {
                h2079
            }
        }, match v1 {
            Adt1::C0(_, h2080, _, _, _, _, _, _, _, _, _, _) => {
                h2080
            }
        })))
    } else {
        Ok(Adt10::C1)
    }
}

pub fn f66(v0: Adt2, v1: Adt2) -> R<Adt10> {
    let a2082 = match v0.clone() {
        Adt2::C0(h2081, _, _, _, _, _, _, _, _, _, _) => {
            h2081
        }
    };
    let a2083 = 0u64;
    if if if nat_eq(a2082, a2083) {
        let a2085 = match v1.clone() {
            Adt2::C0(h2084, _, _, _, _, _, _, _, _, _, _) => {
                h2084
            }
        };
        let a2086 = 0u64;
        nat_eq(a2085, a2086)
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

pub fn f67(v0: Adt2, v1: Adt2) -> R<Adt10> {
    let a2088 = match v0.clone() {
        Adt2::C0(h2087, _, _, _, _, _, _, _, _, _, _) => {
            h2087
        }
    };
    let a2089 = 0u64;
    if if nat_eq(a2088, a2089) {
        let a2091 = match v1.clone() {
            Adt2::C0(h2090, _, _, _, _, _, _, _, _, _, _) => {
                h2090
            }
        };
        let a2092 = 26u64;
        nat_eq(a2091, a2092)
    } else {
        false
    } {
        let a2094 = match v0.clone() {
            Adt2::C0(_, _, _, h2093, _, _, _, _, _, _, _) => {
                h2093
            }
        };
        let a2095 = 1u64;
        if if if if if if if nat_eq(a2094, a2095) {
            let a2097 = match v0.clone() {
                Adt2::C0(_, _, _, _, h2096, _, _, _, _, _, _) => {
                    h2096
                }
            };
            let a2098 = 1u64;
            nat_eq(a2097, a2098)
        } else {
            false
        } {
            let a2100 = match v0.clone() {
                Adt2::C0(_, h2099, _, _, _, _, _, _, _, _, _) => {
                    h2099
                }
            };
            let a2101 = 2147483647u64;
            nat_le(a2100, a2101)
        } else {
            false
        } {
            let a2104 = match v1.clone() {
                Adt2::C0(_, h2102, _, _, _, _, _, _, _, _, _) => {
                    h2102
                }
            };
            let a2105 = match v0.clone() {
                Adt2::C0(_, _, h2103, _, _, _, _, _, _, _, _) => {
                    h2103
                }
            };
            nat_eq(a2104, a2105)
        } else {
            false
        } {
            let a2107 = match v1.clone() {
                Adt2::C0(_, _, h2106, _, _, _, _, _, _, _, _) => {
                    h2106
                }
            };
            let a2108 = 1u64;
            nat_eq(a2107, a2108)
        } else {
            false
        } {
            let a2110 = match v1.clone() {
                Adt2::C0(_, _, _, h2109, _, _, _, _, _, _, _) => {
                    h2109
                }
            };
            let a2111 = 1u64;
            nat_eq(a2110, a2111)
        } else {
            false
        } {
            let a2113 = match v1.clone() {
                Adt2::C0(_, _, _, _, h2112, _, _, _, _, _, _) => {
                    h2112
                }
            };
            let a2114 = 1u64;
            nat_eq(a2113, a2114)
        } else {
            false
        } {
            Ok(Adt10::C4({
                let a2117 = match v0.clone() {
                    Adt2::C0(_, _, h2115, _, _, _, _, _, _, _, _) => {
                        h2115
                    }
                };
                let a2118 = f139(match v0.clone() {
                    Adt2::C0(_, h2116, _, _, _, _, _, _, _, _, _) => {
                        h2116
                    }
                })?;
                nat_mul(a2117, a2118)?
            }, 4u64, f20(v0, 1u64), f20(v1, 1u64), Adt4::C17))
        } else {
            Ok(Adt10::C1)
        }
    } else {
        Ok(Adt10::C0)
    }
}

pub fn f68(v0: Adt2, v1: Option<Adt2>, v2: Adt2) -> R<Adt10> {
    let m2119 = v1;
    match m2119 {
        None => {
            Ok(Adt10::C1)
        }
        Some(v3) => {
            let v4: u64 = f140(f20(v0.clone(), 1u64))?;
            let a2121 = match v0.clone() {
                Adt2::C0(h2120, _, _, _, _, _, _, _, _, _, _) => {
                    h2120
                }
            };
            let a2122 = 26u64;
            if if if nat_eq(a2121, a2122) {
                let a2124 = match v3.clone() {
                    Adt2::C0(h2123, _, _, _, _, _, _, _, _, _, _) => {
                        h2123
                    }
                };
                let a2125 = 26u64;
                nat_eq(a2124, a2125)
            } else {
                false
            } {
                let a2127 = match v2.clone() {
                    Adt2::C0(h2126, _, _, _, _, _, _, _, _, _, _) => {
                        h2126
                    }
                };
                let a2128 = 27u64;
                nat_eq(a2127, a2128)
            } else {
                false
            } {
                if if if if if if f25(v0.clone(), v3.clone()) {
                    let a2130 = match v2.clone() {
                        Adt2::C0(_, h2129, _, _, _, _, _, _, _, _, _) => {
                            h2129
                        }
                    };
                    let a2131 = 1u64;
                    nat_eq(a2130, a2131)
                } else {
                    false
                } {
                    let a2133 = match v2.clone() {
                        Adt2::C0(_, _, h2132, _, _, _, _, _, _, _, _) => {
                            h2132
                        }
                    };
                    let a2134 = 1u64;
                    nat_eq(a2133, a2134)
                } else {
                    false
                } {
                    let a2136 = match v2.clone() {
                        Adt2::C0(_, _, _, h2135, _, _, _, _, _, _, _) => {
                            h2135
                        }
                    };
                    let a2137 = 1u64;
                    nat_eq(a2136, a2137)
                } else {
                    false
                } {
                    let a2139 = match v2.clone() {
                        Adt2::C0(_, _, _, _, h2138, _, _, _, _, _, _) => {
                            h2138
                        }
                    };
                    let a2140 = 1u64;
                    nat_eq(a2139, a2140)
                } else {
                    false
                } {
                    let a2141 = v4;
                    let a2142 = 9223372036854775808u64;
                    nat_lt(a2141, a2142)
                } else {
                    false
                } {
                    Ok(Adt10::C4({
                        let a2145 = 2u64;
                        let a2146 = {
                            let a2143 = v4;
                            let a2144 = 2u64;
                            nat_mul(a2143, a2144)?
                        };
                        nat_add(a2145, a2146)?
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

pub fn f69(v0: u64, v1: Adt2, v2: Adt2) -> R<Adt10> {
    let v3: u64 = match v1.clone() {
        Adt2::C0(h2147, _, _, _, _, _, _, _, _, _, _) => {
            h2147
        }
    };
    let a2148 = v3;
    let a2149 = 0u64;
    if if nat_eq(a2148, a2149) {
        true
    } else {
        let a2150 = v0;
        let a2151 = 13u64;
        if nat_eq(a2150, a2151) {
            let a2152 = v3;
            let a2153 = 1u64;
            if nat_eq(a2152, a2153) {
                true
            } else {
                let a2154 = v3;
                let a2155 = 30u64;
                nat_eq(a2154, a2155)
            }
        } else {
            false
        }
    } {
        let a2157 = v3;
        let a2158 = match v2.clone() {
            Adt2::C0(h2156, _, _, _, _, _, _, _, _, _, _) => {
                h2156
            }
        };
        if if nat_eq(a2157, a2158) {
            let a2159 = v0;
            let a2160 = 13u64;
            if nat_eq(a2159, a2160) {
                let a2162 = match v2.clone() {
                    Adt2::C0(_, h2161, _, _, _, _, _, _, _, _, _) => {
                        h2161
                    }
                };
                let a2163 = 1u64;
                if if if nat_eq(a2162, a2163) {
                    let a2165 = match v2.clone() {
                        Adt2::C0(_, _, h2164, _, _, _, _, _, _, _, _) => {
                            h2164
                        }
                    };
                    let a2166 = 1u64;
                    nat_eq(a2165, a2166)
                } else {
                    false
                } {
                    let a2168 = match v2.clone() {
                        Adt2::C0(_, _, _, h2167, _, _, _, _, _, _, _) => {
                            h2167
                        }
                    };
                    let a2169 = 1u64;
                    nat_eq(a2168, a2169)
                } else {
                    false
                } {
                    let a2171 = match v2.clone() {
                        Adt2::C0(_, _, _, _, h2170, _, _, _, _, _, _) => {
                            h2170
                        }
                    };
                    let a2172 = 1u64;
                    nat_eq(a2171, a2172)
                } else {
                    false
                }
            } else {
                let a2174 = match v2.clone() {
                    Adt2::C0(_, h2173, _, _, _, _, _, _, _, _, _) => {
                        h2173
                    }
                };
                let a2175 = 1u64;
                if if if nat_eq(a2174, a2175) {
                    let a2178 = match v1.clone() {
                        Adt2::C0(_, _, h2176, _, _, _, _, _, _, _, _) => {
                            h2176
                        }
                    };
                    let a2179 = match v2.clone() {
                        Adt2::C0(_, _, h2177, _, _, _, _, _, _, _, _) => {
                            h2177
                        }
                    };
                    nat_eq(a2178, a2179)
                } else {
                    false
                } {
                    let a2182 = match v1.clone() {
                        Adt2::C0(_, _, _, h2180, _, _, _, _, _, _, _) => {
                            h2180
                        }
                    };
                    let a2183 = match v2.clone() {
                        Adt2::C0(_, _, _, h2181, _, _, _, _, _, _, _) => {
                            h2181
                        }
                    };
                    nat_eq(a2182, a2183)
                } else {
                    false
                } {
                    let a2186 = match v1.clone() {
                        Adt2::C0(_, _, _, _, h2184, _, _, _, _, _, _) => {
                            h2184
                        }
                    };
                    let a2187 = match v2.clone() {
                        Adt2::C0(_, _, _, _, h2185, _, _, _, _, _, _) => {
                            h2185
                        }
                    };
                    nat_eq(a2186, a2187)
                } else {
                    false
                }
            }
        } else {
            false
        } {
            let v4: Adt3 = f20(v1, 1u64);
            Ok(Adt10::C4(f141(v4.clone(), v0, v3)?, {
                let a2188 = v3;
                let a2189 = 0u64;
                if nat_eq(a2188, a2189) {
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

pub fn f70(v0: u64, v1: u64) -> R<Option<u64>> {
    let a2190 = v0;
    let a2191 = 89u64;
    if if nat_eq(a2190, a2191) {
        let a2192 = v1;
        let a2193 = 14u64;
        nat_eq(a2192, a2193)
    } else {
        false
    } {
        Ok(Some(23u64))
    } else {
        let a2194 = v0;
        let a2195 = 89u64;
        if if nat_eq(a2194, a2195) {
            let a2196 = v1;
            let a2197 = 15u64;
            nat_eq(a2196, a2197)
        } else {
            false
        } {
            Ok(Some(22u64))
        } else {
            let a2198 = v0;
            let a2199 = 89u64;
            if if nat_eq(a2198, a2199) {
                let a2200 = 8u64;
                let a2201 = v1;
                if nat_le(a2200, a2201) {
                    let a2202 = v1;
                    let a2203 = 9u64;
                    nat_le(a2202, a2203)
                } else {
                    false
                }
            } else {
                false
            } {
                Ok(Some({
                    let a2204 = v1;
                    let a2205 = 12u64;
                    nat_add(a2204, a2205)?
                }))
            } else {
                let a2206 = v0;
                let a2207 = 89u64;
                if if nat_eq(a2206, a2207) {
                    let a2208 = v1;
                    let a2209 = 4u64;
                    nat_eq(a2208, a2209)
                } else {
                    false
                } {
                    Ok(Some(19u64))
                } else {
                    let a2210 = 10u64;
                    let a2211 = v0;
                    if if nat_le(a2210, a2211) {
                        let a2212 = v0;
                        let a2213 = 12u64;
                        nat_le(a2212, a2213)
                    } else {
                        false
                    } {
                        Ok(Some({
                            let a2214 = v0;
                            let a2215 = 3u64;
                            nat_add(a2214, a2215)?
                        }))
                    } else {
                        let a2216 = v0;
                        let a2217 = 89u64;
                        if if nat_eq(a2216, a2217) {
                            let a2218 = v1;
                            let a2219 = 7u64;
                            nat_eq(a2218, a2219)
                        } else {
                            false
                        } {
                            Ok(Some(17u64))
                        } else {
                            let a2220 = v0;
                            let a2221 = 89u64;
                            if if nat_eq(a2220, a2221) {
                                let a2222 = v1;
                                let a2223 = 10u64;
                                nat_eq(a2222, a2223)
                            } else {
                                false
                            } {
                                Ok(Some(18u64))
                            } else {
                                let a2224 = v0;
                                let a2225 = 89u64;
                                if if nat_eq(a2224, a2225) {
                                    let a2226 = v1;
                                    let a2227 = 13u64;
                                    nat_eq(a2226, a2227)
                                } else {
                                    false
                                } {
                                    Ok(Some(16u64))
                                } else {
                                    let a2228 = v0;
                                    let a2229 = 8u64;
                                    if nat_eq(a2228, a2229) {
                                        Ok(Some(5u64))
                                    } else {
                                        let a2230 = v0;
                                        let a2231 = 9u64;
                                        if nat_eq(a2230, a2231) {
                                            Ok(Some(6u64))
                                        } else {
                                            let a2232 = v0;
                                            let a2233 = 89u64;
                                            if nat_eq(a2232, a2233) {
                                                let a2234 = v1;
                                                let a2235 = 0u64;
                                                if nat_eq(a2234, a2235) {
                                                    Ok(Some(0u64))
                                                } else {
                                                    let a2236 = v1;
                                                    let a2237 = 1u64;
                                                    if nat_eq(a2236, a2237) {
                                                        Ok(Some(1u64))
                                                    } else {
                                                        let a2238 = v1;
                                                        let a2239 = 2u64;
                                                        if nat_eq(a2238, a2239) {
                                                            Ok(Some(2u64))
                                                        } else {
                                                            let a2240 = v1;
                                                            let a2241 = 3u64;
                                                            if nat_eq(a2240, a2241) {
                                                                Ok(Some(3u64))
                                                            } else {
                                                                let a2242 = v1;
                                                                let a2243 = 6u64;
                                                                if nat_eq(a2242, a2243) {
                                                                    Ok(Some(4u64))
                                                                } else {
                                                                    let a2244 = v1;
                                                                    let a2245 = 12u64;
                                                                    if nat_eq(a2244, a2245) {
                                                                        Ok(Some(7u64))
                                                                    } else {
                                                                        let a2246 = v1;
                                                                        let a2247 = 11u64;
                                                                        if nat_eq(a2246, a2247) {
                                                                            Ok(Some(8u64))
                                                                        } else {
                                                                            let a2248 = v1;
                                                                            let a2249 = 18u64;
                                                                            if nat_eq(a2248, a2249) {
                                                                                Ok(Some(9u64))
                                                                            } else {
                                                                                let a2250 = v1;
                                                                                let a2251 = 19u64;
                                                                                if nat_eq(a2250, a2251) {
                                                                                    Ok(Some(10u64))
                                                                                } else {
                                                                                    let a2252 = v1;
                                                                                    let a2253 = 20u64;
                                                                                    if nat_eq(a2252, a2253) {
                                                                                        Ok(Some(11u64))
                                                                                    } else {
                                                                                        let a2254 = v1;
                                                                                        let a2255 = 21u64;
                                                                                        if nat_eq(a2254, a2255) {
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

pub fn f71(v0: u64, v1: u64, v2: Adt2, v3: Option<Adt2>, v4: Adt2) -> R<Adt10> {
    let m2256 = f142(v0, v1, v2.clone(), v3.clone(), v4.clone())?;
    match m2256 {
        Adt10::C0 => {
            Ok(Adt10::C0)
        }
        Adt10::C1 => {
            let a2257 = v0;
            let a2258 = 2u64;
            if if if if nat_eq(a2257, a2258) {
                true
            } else {
                let a2259 = v0;
                let a2260 = 5u64;
                nat_eq(a2259, a2260)
            } {
                true
            } else {
                let a2261 = v0;
                let a2262 = 6u64;
                nat_eq(a2261, a2262)
            } {
                true
            } else {
                let a2263 = v0;
                let a2264 = 7u64;
                nat_eq(a2263, a2264)
            } {
                Ok(Adt10::C5(67u64))
            } else {
                Ok(Adt10::C1)
            }
        }
        Adt10::C2(v7, v8, _, _, _, _, v13) => {
            if v8 {
                Ok(Adt10::C4(v13, 4u64, f20(v2.clone(), 1u64), f20(v4, 1u64), Adt4::C10(v7, v8, {
                    let m2265 = v3;
                    match m2265 {
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

pub fn f72(v0: u64, v1: Adt2, v2: Adt2) -> R<Adt10> {
    let a2267 = match v1.clone() {
        Adt2::C0(h2266, _, _, _, _, _, _, _, _, _, _) => {
            h2266
        }
    };
    let a2268 = 0u64;
    if if if if if nat_eq(a2267, a2268) {
        true
    } else {
        let a2270 = match v1.clone() {
            Adt2::C0(h2269, _, _, _, _, _, _, _, _, _, _) => {
                h2269
            }
        };
        let a2271 = 1u64;
        nat_eq(a2270, a2271)
    } {
        true
    } else {
        let a2273 = match v1.clone() {
            Adt2::C0(h2272, _, _, _, _, _, _, _, _, _, _) => {
                h2272
            }
        };
        let a2274 = 30u64;
        nat_eq(a2273, a2274)
    } {
        let a2277 = match v1.clone() {
            Adt2::C0(h2275, _, _, _, _, _, _, _, _, _, _) => {
                h2275
            }
        };
        let a2278 = match v2.clone() {
            Adt2::C0(h2276, _, _, _, _, _, _, _, _, _, _) => {
                h2276
            }
        };
        if nat_eq(a2277, a2278) {
            true
        } else {
            let a2280 = match v2.clone() {
                Adt2::C0(h2279, _, _, _, _, _, _, _, _, _, _) => {
                    h2279
                }
            };
            let a2281 = 0u64;
            nat_eq(a2280, a2281)
        }
    } else {
        false
    } {
        f25(v1.clone(), v2.clone())
    } else {
        false
    } {
        Ok(Adt10::C4({
            let a2283 = f16(v2.clone(), 1u64)?;
            let a2284 = f143(v0, match v2.clone() {
                Adt2::C0(h2282, _, _, _, _, _, _, _, _, _, _) => {
                    h2282
                }
            });
            nat_mul(a2283, a2284)?
        }, {
            let a2286 = match v2.clone() {
                Adt2::C0(h2285, _, _, _, _, _, _, _, _, _, _) => {
                    h2285
                }
            };
            let a2287 = 0u64;
            if nat_eq(a2286, a2287) {
                4u64
            } else {
                2u64
            }
        }, f20(v1.clone(), 1u64), f20(v2.clone(), 1u64), Adt4::C14(v0, match v1 {
            Adt2::C0(h2288, _, _, _, _, _, _, _, _, _, _) => {
                h2288
            }
        }, match v2 {
            Adt2::C0(h2289, _, _, _, _, _, _, _, _, _, _) => {
                h2289
            }
        })))
    } else {
        Ok(Adt10::C1)
    }
}

pub fn f73(v0: Adt3, v1: u64, v2: u64, v3: u64, v4: u64) -> R<u64> {
    f144(v1, v2, v3, v4, match v0.clone() {
        Adt3::C0(_, _, _, _, h2290, _, _, _) => {
            h2290
        }
    }, match v0.clone() {
        Adt3::C0(_, _, _, _, _, h2291, _, _) => {
            h2291
        }
    }, match v0.clone() {
        Adt3::C0(_, _, _, _, _, _, h2292, _) => {
            h2292
        }
    }, match v0 {
        Adt3::C0(_, _, _, _, _, _, _, h2293) => {
            h2293
        }
    })
}

pub fn f74(v0: u64, v1: Adt3, v2: u64) -> u64 {
    let v3: u64 = {
        let a2295 = v0;
        let a2296 = match v1.clone() {
            Adt3::C0(h2294, _, _, _, _, _, _, _) => {
                h2294
            }
        };
        let a2297 = 0u64;
        nat_quot(a2295, a2296, a2297)
    };
    let v4: u64 = {
        let a2299 = v3;
        let a2300 = match v1.clone() {
            Adt3::C0(_, h2298, _, _, _, _, _, _) => {
                h2298
            }
        };
        let a2301 = 0u64;
        nat_quot(a2299, a2300, a2301)
    };
    let a2302 = v2;
    let a2303 = 0u64;
    if nat_eq(a2302, a2303) {
        let a2305 = v0;
        let a2306 = match v1 {
            Adt3::C0(h2304, _, _, _, _, _, _, _) => {
                h2304
            }
        };
        let a2307 = 0u64;
        nat_rem(a2305, a2306, a2307)
    } else {
        let a2308 = v2;
        let a2309 = 1u64;
        if nat_eq(a2308, a2309) {
            let a2311 = v3;
            let a2312 = match v1 {
                Adt3::C0(_, h2310, _, _, _, _, _, _) => {
                    h2310
                }
            };
            let a2313 = 0u64;
            nat_rem(a2311, a2312, a2313)
        } else {
            let a2314 = v2;
            let a2315 = 2u64;
            if nat_eq(a2314, a2315) {
                let a2317 = v4;
                let a2318 = match v1 {
                    Adt3::C0(_, _, h2316, _, _, _, _, _) => {
                        h2316
                    }
                };
                let a2319 = 0u64;
                nat_rem(a2317, a2318, a2319)
            } else {
                let a2321 = v4;
                let a2322 = match v1 {
                    Adt3::C0(_, _, h2320, _, _, _, _, _) => {
                        h2320
                    }
                };
                let a2323 = 0u64;
                nat_quot(a2321, a2322, a2323)
            }
        }
    }
}

pub fn f75(v0: Adt8, v1: u64, v2: u64) -> R<u64> {
    let a2325 = match v0.clone() {
        Adt8::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, h2324) => {
            h2324
        }
    };
    let a2326 = 0u64;
    if nat_eq(a2325, a2326) {
        Ok(v2)
    } else {
        let a2331 = {
            let a2328 = v2;
            let a2329 = match v0.clone() {
                Adt8::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, h2327) => {
                    h2327
                }
            };
            nat_mul(a2328, a2329)?
        };
        let a2332 = f74(v1, match v0 {
            Adt8::C0(_, _, _, _, _, _, _, _, h2330, _, _, _, _, _, _) => {
                h2330
            }
        }, 3u64);
        nat_add(a2331, a2332)
    }
}

pub fn f76(v0: Adt6, v1: u64, v2: u64, v3: u64) -> R<Option<u64>> {
    let v4: u64 = {
        let a2342 = {
            let a2339 = {
                let a2334 = v2;
                let a2335 = match v0.clone() {
                    Adt6::C0(_, h2333, _, _) => {
                        h2333
                    }
                };
                nat_mul(a2334, a2335)?
            };
            let a2340 = {
                let a2337 = v3;
                let a2338 = match v0.clone() {
                    Adt6::C0(_, _, _, h2336) => {
                        h2336
                    }
                };
                nat_mul(a2337, a2338)?
            };
            nat_add(a2339, a2340)?
        };
        let a2343 = f145(match v0.clone() {
            Adt6::C0(_, _, h2341, _) => {
                h2341
            }
        });
        nat_add(a2342, a2343)?
    };
    let v5: u64 = f146(match v0 {
        Adt6::C0(_, _, h2344, _) => {
            h2344
        }
    });
    let a2345 = v5;
    let a2346 = v4;
    if if nat_le(a2345, a2346) {
        let a2349 = {
            let a2347 = v4;
            let a2348 = v5;
            nat_sub(a2347, a2348)
        };
        let a2350 = v1;
        nat_lt(a2349, a2350)
    } else {
        false
    } {
        Ok(Some({
            let a2351 = v4;
            let a2352 = v5;
            nat_sub(a2351, a2352)
        }))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f77(v0: Adt8, v1: u64, v2: u64, v3: u64, v4: u64) -> R<Adt9> {
    let v5: Adt3 = match v0.clone() {
        Adt8::C0(_, _, _, _, _, _, _, _, _, h2353, _, _, _, _, _) => {
            h2353
        }
    };
    let v6: Adt3 = match v0.clone() {
        Adt8::C0(_, _, _, _, _, _, _, _, h2354, _, _, _, _, _, _) => {
            h2354
        }
    };
    let v7: u64 = f74(v1, v6.clone(), 2u64);
    let a2356 = match v0.clone() {
        Adt8::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, h2355) => {
            h2355
        }
    };
    let a2357 = 0u64;
    if nat_eq(a2356, a2357) {
        Ok(Adt9::C8(Some(f73(v5, v3, v4, v7, v2)?)))
    } else {
        let v8: u64 = {
            let m2360 = match v0.clone() {
                Adt8::C0(_, _, _, _, _, _, _, _, _, _, h2359, _, _, _, _) => {
                    h2359
                }
            };
            match m2360 {
                Adt3::C0(_, _, h2358, _, _, _, _, _) => {
                    h2358
                }
            }
        };
        let m2366 = f76(match v0.clone() {
            Adt8::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, h2361, _) => {
                h2361
            }
        }, match v5.clone() {
            Adt3::C0(_, _, h2362, _, _, _, _, _) => {
                h2362
            }
        }, {
            let a2363 = v2;
            let a2364 = v8;
            let a2365 = 0u64;
            nat_rem(a2363, a2364, a2365)
        }, v7)?;
        match m2366 {
            None => {
                Ok(Adt9::C8(None::<u64>))
            }
            Some(v9) => {
                let v10: u64 = {
                    let a2373 = {
                        let a2371 = {
                            let a2367 = v2;
                            let a2368 = v8;
                            let a2369 = 0u64;
                            nat_quot(a2367, a2368, a2369)
                        };
                        let a2372 = match v0 {
                            Adt8::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, h2370) => {
                                h2370
                            }
                        };
                        nat_mul(a2371, a2372)?
                    };
                    let a2374 = f74(v1, v6, 3u64);
                    nat_add(a2373, a2374)?
                };
                Ok(Adt9::C8(Some(f73(v5, v3, v4, v9, v10)?)))
            }
        }
    }
}

pub fn f78(v0: Adt4, v1: Adt9) -> Adt9 {
    let m2375 = v1;
    match m2375 {
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

pub fn f79(v0: u64, v1: Adt3, v2: Adt3, v3: Adt4) -> R<Adt9> {
    let m2376 = v3;
    match m2376 {
        Adt4::C0 => {
            Ok(Adt9::C1(false, f152(v0, v1)?, f152(v0, v2)?))
        }
        Adt4::C1 => {
            Ok(Adt9::C1(false, f154(v0, v1, v2.clone())?, f152(v0, v2)?))
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
            Ok(Adt9::C1(true, 0u64, f152(v0, v2)?))
        }
        Adt4::C10(v25, v26, v27) => {
            f161(v0, v1, v2, v25, v26, v27)
        }
        Adt4::C11(v37, v38) => {
            f162(v0, v1, v2, v37, v38)
        }
        Adt4::C12(v39, v40, v41) => {
            f163(v0, v1, v2, v39, v40, v41)
        }
        Adt4::C13(_, v43, _, _) => {
            Ok(Adt9::C3(v43, v43, f152(v0, v1)?, f152(v0, v2)?))
        }
        Adt4::C14(v46, v47, v48) => {
            let a2379 = v46;
            let a2380 = 23u64;
            if nat_eq(a2379, a2380) {
                f164(v0, v1, v2, v47, v48)
            } else {
                Ok(Adt9::C3(v47, v48, f152(v0, v1)?, f152(v0, v2)?))
            }
        }
        Adt4::C15 => {
            f165(v0, v1, v2)
        }
        Adt4::C16(v49, v50) => {
            f166(v0, v1, v2, v49, v50)
        }
        Adt4::C17 => {
            Ok(Adt9::C0(Adt10::C1))
        }
        Adt4::C18(v51, v52) => {
            f167(v0, v1, v52, v51)
        }
        Adt4::C19(_, _) => {
            Ok(Adt9::C0(Adt10::C1))
        }
        Adt4::C20(_, _) => {
            Ok(Adt9::C0(Adt10::C1))
        }
        Adt4::C21 => {
            Ok(Adt9::C1(false, f152(f153(v0, match v2.clone() {
                Adt3::C0(h2377, _, _, _, _, _, _, _) => {
                    h2377
                }
            }, match v2.clone() {
                Adt3::C0(_, h2378, _, _, _, _, _, _) => {
                    h2378
                }
            })?, v1)?, f152(v0, v2)?))
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
        Adt4::C25(v57, v58, v59) => {
            f168(v0, v57, v58, v59)
        }
        Adt4::C26(v65, _, _, _) => {
            let a2381 = v65;
            let a2382 = 64u64;
            if if if nat_eq(a2381, a2382) {
                true
            } else {
                let a2383 = v65;
                let a2384 = 65u64;
                nat_eq(a2383, a2384)
            } {
                true
            } else {
                let a2385 = v65;
                let a2386 = 66u64;
                nat_eq(a2385, a2386)
            } {
                Ok(Adt9::C0(Adt10::C5(64u64)))
            } else {
                Ok(Adt9::C0(Adt10::C1))
            }
        }
        Adt4::C27(v64) => {
            f92(v0, v1, v2, v64)
        }
        Adt4::C28(v60) => {
            f89(v0, v1, v2, v60, 0u64)
        }
        Adt4::C29(v61) => {
            f90(v0, v1, v2, v61)
        }
        Adt4::C30(v62, _) => {
            f91(v0, v1, v2, v62)
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
    }
}

pub fn f80(v0: u64, v1: Adt3, v2: Adt3, v3: u64, v4: u64) -> R<Adt9> {
    let v5: u64 = f129(v3, v4);
    let v6: u64 = {
        let a2387 = v0;
        let a2388 = v5;
        let a2389 = 0u64;
        nat_quot(a2387, a2388, a2389)
    };
    let a2390 = v5;
    let a2391 = 1u64;
    if nat_eq(a2390, a2391) {
        Ok(f147(f169(v3, v4), v3, v4, 0u64, f152(v6, v1)?, 2u64, 0u64, 3u64, f152(v6, v2)?))
    } else {
        let a2395 = {
            let a2392 = v0;
            let a2393 = 2u64;
            let a2394 = 0u64;
            nat_rem(a2392, a2393, a2394)
        };
        let a2396 = 0u64;
        if nat_eq(a2395, a2396) {
            Ok(f147(f169(v3, 0u64), v3, 0u64, 0u64, f152(v6, v1)?, 2u64, 0u64, 1u64, 0u64))
        } else {
            Ok(f147(f169(0u64, v4), 0u64, v4, 1u64, 0u64, 2u64, 0u64, 3u64, f152(v6, v2)?))
        }
    }
}

pub fn f81(v0: u64, v1: u64, v2: u64) -> R<Adt9> {
    let v3: u64 = f115(v1)?;
    let v4: u64 = {
        let a2397 = v0;
        let a2398 = v3;
        let a2399 = 0u64;
        nat_rem(a2397, a2398, a2399)
    };
    let v5: u64 = {
        let a2402 = {
            let a2400 = v1;
            let a2401 = 3u64;
            nat_mul(a2400, a2401)?
        };
        let a2403 = 1u64;
        nat_add(a2402, a2403)?
    };
    let a2404 = v4;
    let a2405 = 0u64;
    if nat_eq(a2404, a2405) {
        Ok(Adt9::C7(Adt11::C0(0u64), Adt12::C2(0u64), Adt12::C2(0u64), Adt13::C0(0u64), 8u64, 8u64))
    } else {
        let a2406 = v4;
        let a2407 = v5;
        if nat_eq(a2406, a2407) {
            Ok(Adt9::C7(Adt11::C0(2u64), Adt12::C1(0u64), Adt12::C2(0u64), Adt13::C0(0u64), 8u64, 8u64))
        } else {
            let v6: u64 = {
                let a2412 = v4;
                let a2413 = {
                    let a2408 = v4;
                    let a2409 = v5;
                    if nat_lt(a2408, a2409) {
                        1u64
                    } else {
                        let a2410 = v5;
                        let a2411 = 1u64;
                        nat_add(a2410, a2411)?
                    }
                };
                nat_sub(a2412, a2413)
            };
            let v7: u64 = {
                let a2414 = v6;
                let a2415 = 3u64;
                let a2416 = 0u64;
                nat_rem(a2414, a2415, a2416)
            };
            let v8: u64 = {
                let a2427 = 4u64;
                let a2428 = {
                    let a2425 = {
                        let a2420 = {
                            let a2417 = v0;
                            let a2418 = v3;
                            let a2419 = 0u64;
                            nat_quot(a2417, a2418, a2419)
                        };
                        let a2421 = v1;
                        nat_mul(a2420, a2421)?
                    };
                    let a2426 = {
                        let a2422 = v6;
                        let a2423 = 3u64;
                        let a2424 = 0u64;
                        nat_quot(a2422, a2423, a2424)
                    };
                    nat_add(a2425, a2426)?
                };
                nat_mul(a2427, a2428)?
            };
            let a2429 = v4;
            let a2430 = v5;
            if nat_lt(a2429, a2430) {
                let a2431 = v7;
                let a2432 = 0u64;
                if nat_eq(a2431, a2432) {
                    Ok(Adt9::C7(Adt11::C1(2u64), Adt12::C0(1u64, v8), Adt12::C0(0u64, v8), Adt13::C0(1u64), 4u64, 4u64))
                } else {
                    let a2433 = v7;
                    let a2434 = 1u64;
                    if nat_eq(a2433, a2434) {
                        Ok(Adt9::C7(Adt11::C0(1u64), Adt12::C1(1u64), Adt12::C2(0u64), Adt13::C0(1u64), 4u64, 4u64))
                    } else {
                        Ok(Adt9::C7(Adt11::C0(0u64), Adt12::C1(0u64), Adt12::C1(1u64), Adt13::C0(0u64), 8u64, 8u64))
                    }
                }
            } else {
                let a2435 = v7;
                let a2436 = 0u64;
                if nat_eq(a2435, a2436) {
                    Ok(Adt9::C7(Adt11::C1(1u64), Adt12::C0(0u64, v8), Adt12::C1(0u64), Adt13::C0(1u64), 4u64, 4u64))
                } else {
                    let a2437 = v7;
                    let a2438 = 1u64;
                    if nat_eq(a2437, a2438) {
                        Ok(Adt9::C7(Adt11::C1(2u64), Adt12::C2(v2), Adt12::C1(1u64), Adt13::C0(1u64), 4u64, 4u64))
                    } else {
                        Ok(Adt9::C7(Adt11::C1(2u64), Adt12::C1(1u64), Adt12::C0(1u64, v8), Adt13::C1(v8), 4u64, 4u64))
                    }
                }
            }
        }
    }
}

pub fn f82(v0: u64, v1: Adt3, v2: Adt3, v3: Adt3) -> R<Adt9> {
    let v4: u64 = {
        let a2439 = v0;
        let a2440 = 6u64;
        let a2441 = 0u64;
        nat_quot(a2439, a2440, a2441)
    };
    let v5: u64 = {
        let a2442 = v0;
        let a2443 = 6u64;
        let a2444 = 0u64;
        nat_rem(a2442, a2443, a2444)
    };
    let a2445 = v5;
    let a2446 = 0u64;
    if nat_eq(a2445, a2446) {
        Ok(Adt9::C7(Adt11::C7(17u64, 0u64, 0u64), Adt12::C0(1u64, f152(v4, v2)?), Adt12::C2(0u64), Adt13::C0(0u64), 4u64, 4u64))
    } else {
        let a2447 = v5;
        let a2448 = 1u64;
        if nat_eq(a2447, a2448) {
            Ok(Adt9::C7(Adt11::C1(1u64), Adt12::C2(1065353216u64), Adt12::C1(0u64), Adt13::C0(1u64), 4u64, 4u64))
        } else {
            let a2449 = v5;
            let a2450 = 2u64;
            if nat_eq(a2449, a2450) {
                Ok(Adt9::C7(Adt11::C1(2u64), Adt12::C0(1u64, f152(v4, v2)?), Adt12::C1(1u64), Adt13::C0(1u64), 4u64, 4u64))
            } else {
                let a2451 = v5;
                let a2452 = 3u64;
                if nat_eq(a2451, a2452) {
                    Ok(Adt9::C7(Adt11::C1(0u64), Adt12::C2(1065353216u64), Adt12::C1(1u64), Adt13::C0(1u64), 4u64, 4u64))
                } else {
                    let a2453 = v5;
                    let a2454 = 4u64;
                    if nat_eq(a2453, a2454) {
                        Ok(Adt9::C7(Adt11::C1(2u64), Adt12::C0(0u64, f152(v4, v1)?), Adt12::C1(0u64), Adt13::C0(0u64), 4u64, 4u64))
                    } else {
                        Ok(Adt9::C7(Adt11::C1(2u64), Adt12::C1(0u64), Adt12::C1(1u64), Adt13::C1(f152(v4, v3)?), 4u64, 4u64))
                    }
                }
            }
        }
    }
}

pub fn f83(v0: u64, v1: u64, v2: Adt3, v3: Adt3) -> R<Adt9> {
    let a2455 = v0;
    let a2456 = v1;
    if nat_lt(a2455, a2456) {
        Ok(Adt9::C7(Adt11::C9, Adt12::C0(0u64, f152(v0, v2)?), Adt12::C2(0u64), Adt13::C0(v0), 4u64, 4u64))
    } else {
        let v4: u64 = {
            let a2457 = v0;
            let a2458 = v1;
            nat_sub(a2457, a2458)
        };
        Ok(Adt9::C7(Adt11::C5(3u64), Adt12::C1(v4), Adt12::C2(0u64), Adt13::C1(f152(v4, v3)?), 4u64, 4u64))
    }
}

pub fn f84(v0: u64, v1: Adt3, v2: Adt3, v3: Adt3, v4: Adt6, v5: Adt6, v6: u64) -> R<Adt9> {
    let v7: u64 = {
        let a2465 = {
            let a2463 = 3u64;
            let a2464 = {
                let a2461 = match v4.clone() {
                    Adt6::C0(h2459, _, _, _) => {
                        h2459
                    }
                };
                let a2462 = match v5.clone() {
                    Adt6::C0(h2460, _, _, _) => {
                        h2460
                    }
                };
                nat_mul(a2461, a2462)?
            };
            nat_mul(a2463, a2464)?
        };
        let a2466 = 2u64;
        nat_add(a2465, a2466)?
    };
    let v8: u64 = {
        let a2467 = v0;
        let a2468 = v7;
        let a2469 = 0u64;
        nat_quot(a2467, a2468, a2469)
    };
    let v9: u64 = {
        let a2470 = v0;
        let a2471 = v7;
        let a2472 = 0u64;
        nat_rem(a2470, a2471, a2472)
    };
    let a2473 = v9;
    let a2474 = 0u64;
    if nat_eq(a2473, a2474) {
        Ok(Adt9::C7(Adt11::C2(0u64, 0u64, false), Adt12::C2(0u64), Adt12::C2(0u64), Adt13::C0(0u64), 4u64, 4u64))
    } else {
        let a2477 = {
            let a2475 = v9;
            let a2476 = 1u64;
            nat_add(a2475, a2476)?
        };
        let a2478 = v7;
        if nat_eq(a2477, a2478) {
            Ok(Adt9::C7(Adt11::C2(0u64, 0u64, false), Adt12::C1(0u64), Adt12::C2(0u64), Adt13::C1(f152(v8, v3)?), 4u64, 4u64))
        } else {
            let v10: u64 = {
                let a2479 = v9;
                let a2480 = 1u64;
                nat_sub(a2479, a2480)
            };
            let v11: u64 = {
                let a2481 = v10;
                let a2482 = 3u64;
                let a2483 = 0u64;
                nat_quot(a2481, a2482, a2483)
            };
            let v12: u64 = {
                let a2485 = v11;
                let a2486 = match v4.clone() {
                    Adt6::C0(h2484, _, _, _) => {
                        h2484
                    }
                };
                let a2487 = 0u64;
                nat_rem(a2485, a2486, a2487)
            };
            let v13: u64 = {
                let a2489 = v11;
                let a2490 = match v4.clone() {
                    Adt6::C0(h2488, _, _, _) => {
                        h2488
                    }
                };
                let a2491 = 0u64;
                nat_quot(a2489, a2490, a2491)
            };
            let v14: u64 = {
                let a2492 = v10;
                let a2493 = 3u64;
                let a2494 = 0u64;
                nat_rem(a2492, a2493, a2494)
            };
            let v15: u64 = f74(v8, v3.clone(), 2u64);
            let m2496 = f76(v4, match v2.clone() {
                Adt3::C0(h2495, _, _, _, _, _, _, _) => {
                    h2495
                }
            }, f74(v8, v3.clone(), 0u64), v12)?;
            match m2496 {
                None => {
                    Ok(Adt9::C7(Adt11::C2(0u64, 0u64, false), Adt12::C1(0u64), Adt12::C2(0u64), Adt13::C0(0u64), 4u64, 4u64))
                }
                Some(v16) => {
                    let m2498 = f76(v5, match v2.clone() {
                        Adt3::C0(_, h2497, _, _, _, _, _, _) => {
                            h2497
                        }
                    }, f74(v8, v3.clone(), 1u64), v13)?;
                    match m2498 {
                        None => {
                            Ok(Adt9::C7(Adt11::C2(0u64, 0u64, false), Adt12::C1(0u64), Adt12::C2(0u64), Adt13::C0(0u64), 4u64, 4u64))
                        }
                        Some(v17) => {
                            let a2499 = v14;
                            let a2500 = 0u64;
                            if nat_eq(a2499, a2500) {
                                Ok(Adt9::C7(Adt11::C2(v6, 0u64, false), Adt12::C0(0u64, f73(v1, v12, v13, 0u64, v15)?), Adt12::C2(0u64), Adt13::C0(1u64), f170(v6), 4u64))
                            } else {
                                let a2501 = v14;
                                let a2502 = 1u64;
                                if nat_eq(a2501, a2502) {
                                    Ok(Adt9::C7(Adt11::C1(2u64), Adt12::C1(1u64), Adt12::C0(1u64, f73(v2, v16, v17, v15, f74(v8, v3, 3u64))?), Adt13::C0(1u64), 4u64, 4u64))
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

pub fn f85(v0: u64, v1: Adt3, v2: Adt3, v3: u64) -> R<Adt9> {
    let v4: u64 = f139(match v1.clone() {
        Adt3::C0(h2503, _, _, _, _, _, _, _) => {
            h2503
        }
    })?;
    let v5: u64 = {
        let a2504 = v0;
        let a2505 = v4;
        let a2506 = 0u64;
        nat_quot(a2504, a2505, a2506)
    };
    let v6: u64 = {
        let a2507 = v0;
        let a2508 = v4;
        let a2509 = 0u64;
        nat_rem(a2507, a2508, a2509)
    };
    let a2510 = v6;
    let a2511 = 0u64;
    if nat_eq(a2510, a2511) {
        Ok(Adt9::C4(19u64, 2u64, 4286578688u64, 2u64, 0u64, 1u64, 0u64))
    } else {
        let a2512 = v6;
        let a2513 = 1u64;
        if nat_eq(a2512, a2513) {
            Ok(Adt9::C4(19u64, 2u64, 0u64, 2u64, 0u64, 1u64, 1u64))
        } else {
            let a2516 = {
                let a2514 = v6;
                let a2515 = 1u64;
                nat_add(a2514, a2515)?
            };
            let a2517 = v4;
            if nat_eq(a2516, a2517) {
                Ok(Adt9::C4(19u64, 1u64, 1u64, 2u64, 0u64, 3u64, f152(v5, v2)?))
            } else {
                let v7: u64 = {
                    let a2518 = v6;
                    let a2519 = 2u64;
                    nat_sub(a2518, a2519)
                };
                let v8: u64 = {
                    let a2520 = v7;
                    let a2521 = 3u64;
                    let a2522 = 0u64;
                    nat_quot(a2520, a2521, a2522)
                };
                let v9: u64 = {
                    let a2523 = v7;
                    let a2524 = 3u64;
                    let a2525 = 0u64;
                    nat_rem(a2523, a2524, a2525)
                };
                let v10: u64 = f152({
                    let a2529 = {
                        let a2527 = v5;
                        let a2528 = match v1.clone() {
                            Adt3::C0(h2526, _, _, _, _, _, _, _) => {
                                h2526
                            }
                        };
                        nat_mul(a2527, a2528)?
                    };
                    let a2530 = v8;
                    nat_add(a2529, a2530)?
                }, v1)?;
                let a2531 = v9;
                let a2532 = 0u64;
                if nat_eq(a2531, a2532) {
                    Ok(Adt9::C4(16u64, 1u64, 0u64, 0u64, v10, 1u64, 2u64))
                } else {
                    let a2533 = v9;
                    let a2534 = 1u64;
                    if nat_eq(a2533, a2534) {
                        let a2535 = v3;
                        let a2536 = 2u64;
                        if nat_eq(a2535, a2536) {
                            Ok(Adt9::C4(19u64, 1u64, 0u64, 2u64, 0u64, 1u64, 0u64))
                        } else {
                            Ok(Adt9::C4(19u64, 0u64, v10, 2u64, 0u64, 1u64, 0u64))
                        }
                    } else {
                        let a2537 = v3;
                        let a2538 = 2u64;
                        if if nat_eq(a2537, a2538) {
                            true
                        } else {
                            let a2539 = v3;
                            let a2540 = 3u64;
                            nat_eq(a2539, a2540)
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

pub fn f86(v0: u64, v1: u64, v2: Adt3, v3: Adt3, v4: Adt3, v5: u64) -> R<Adt9> {
    let v6: u64 = {
        let a2542 = 2u64;
        let a2543 = match v2.clone() {
            Adt3::C0(_, h2541, _, _, _, _, _, _) => {
                h2541
            }
        };
        nat_mul(a2542, a2543)?
    };
    let a2544 = v1;
    let a2545 = v6;
    if nat_lt(a2544, a2545) {
        let a2549 = {
            let a2546 = v1;
            let a2547 = 2u64;
            let a2548 = 0u64;
            nat_rem(a2546, a2547, a2548)
        };
        let a2550 = 0u64;
        if nat_eq(a2549, a2550) {
            Ok(Adt9::C4(19u64, 4u64, f152({
                let a2551 = v1;
                let a2552 = 2u64;
                let a2553 = 0u64;
                nat_quot(a2551, a2552, a2553)
            }, v4)?, 2u64, 0u64, 1u64, 2u64))
        } else {
            let a2554 = v5;
            let a2555 = 2147483648u64;
            if if nat_lt(a2554, a2555) {
                let a2557 = v5;
                let a2558 = match v3 {
                    Adt3::C0(_, h2556, _, _, _, _, _, _) => {
                        h2556
                    }
                };
                nat_lt(a2557, a2558)
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
            let a2561 = match v3.clone() {
                Adt3::C0(h2559, _, _, _, _, _, _, _) => {
                    h2559
                }
            };
            let a2562 = match v3.clone() {
                Adt3::C0(_, h2560, _, _, _, _, _, _) => {
                    h2560
                }
            };
            nat_mul(a2561, a2562)?
        };
        let a2565 = {
            let a2563 = v1;
            let a2564 = v6;
            nat_sub(a2563, a2564)
        };
        let a2566 = v7;
        if nat_lt(a2565, a2566) {
            Ok(Adt9::C4(19u64, 2u64, 0u64, 2u64, 0u64, 3u64, f152({
                let a2567 = v1;
                let a2568 = v6;
                nat_sub(a2567, a2568)
            }, v3)?))
        } else {
            let v8: u64 = {
                let a2571 = {
                    let a2569 = v1;
                    let a2570 = v6;
                    nat_sub(a2569, a2570)
                };
                let a2572 = v7;
                nat_sub(a2571, a2572)
            };
            let v9: u64 = {
                let a2574 = 1u64;
                let a2575 = f171(v0, match v2.clone() {
                    Adt3::C0(h2573, _, _, _, _, _, _, _) => {
                        h2573
                    }
                })?;
                nat_add(a2574, a2575)?
            };
            let v10: u64 = {
                let a2576 = v8;
                let a2577 = v9;
                let a2578 = 0u64;
                nat_quot(a2576, a2577, a2578)
            };
            let v11: u64 = {
                let a2579 = v8;
                let a2580 = v9;
                let a2581 = 0u64;
                nat_rem(a2579, a2580, a2581)
            };
            let a2582 = v11;
            let a2583 = 0u64;
            if nat_eq(a2582, a2583) {
                Ok(Adt9::C4(19u64, 4u64, f152(v10, v4)?, 2u64, 0u64, 1u64, 2u64))
            } else {
                if f116(v0) {
                    let a2587 = {
                        let a2584 = v11;
                        let a2585 = 2u64;
                        let a2586 = 0u64;
                        nat_rem(a2584, a2585, a2586)
                    };
                    let a2588 = 1u64;
                    if nat_eq(a2587, a2588) {
                        Ok(Adt9::C4(6u64, 0u64, f152({
                            let a2597 = {
                                let a2590 = v10;
                                let a2591 = match v2.clone() {
                                    Adt3::C0(h2589, _, _, _, _, _, _, _) => {
                                        h2589
                                    }
                                };
                                nat_mul(a2590, a2591)?
                            };
                            let a2598 = {
                                let a2594 = {
                                    let a2592 = v11;
                                    let a2593 = 1u64;
                                    nat_sub(a2592, a2593)
                                };
                                let a2595 = 2u64;
                                let a2596 = 0u64;
                                nat_quot(a2594, a2595, a2596)
                            };
                            nat_add(a2597, a2598)?
                        }, v2)?, 2u64, 0u64, 1u64, 0u64))
                    } else {
                        Ok(Adt9::C4(4u64, 3u64, f152({
                            let a2607 = {
                                let a2600 = v5;
                                let a2601 = match v2.clone() {
                                    Adt3::C0(h2599, _, _, _, _, _, _, _) => {
                                        h2599
                                    }
                                };
                                nat_mul(a2600, a2601)?
                            };
                            let a2608 = {
                                let a2604 = {
                                    let a2602 = v11;
                                    let a2603 = 1u64;
                                    nat_sub(a2602, a2603)
                                };
                                let a2605 = 2u64;
                                let a2606 = 0u64;
                                nat_quot(a2604, a2605, a2606)
                            };
                            nat_add(a2607, a2608)?
                        }, v3.clone())?, 1u64, 0u64, 3u64, f152({
                            let a2617 = {
                                let a2610 = v5;
                                let a2611 = match v2 {
                                    Adt3::C0(h2609, _, _, _, _, _, _, _) => {
                                        h2609
                                    }
                                };
                                nat_mul(a2610, a2611)?
                            };
                            let a2618 = {
                                let a2614 = {
                                    let a2612 = v11;
                                    let a2613 = 1u64;
                                    nat_sub(a2612, a2613)
                                };
                                let a2615 = 2u64;
                                let a2616 = 0u64;
                                nat_quot(a2614, a2615, a2616)
                            };
                            nat_add(a2617, a2618)?
                        }, v3)?))
                    }
                } else {
                    f172(v0, v10, {
                        let a2619 = v11;
                        let a2620 = 1u64;
                        nat_sub(a2619, a2620)
                    }, v5, v2, v3)
                }
            }
        }
    }
}

pub fn f87(v0: u64, v1: u64, v2: Adt3, v3: Adt3, v4: Adt3, v5: u64) -> R<Adt9> {
    let a2621 = v1;
    let a2622 = f173(v2.clone(), v4.clone())?;
    if nat_lt(a2621, a2622) {
        let a2625 = v1;
        let a2626 = {
            let a2623 = 2u64;
            let a2624 = f140(v4.clone())?;
            nat_mul(a2623, a2624)?
        };
        if nat_lt(a2625, a2626) {
            let a2630 = {
                let a2627 = v1;
                let a2628 = 2u64;
                let a2629 = 0u64;
                nat_rem(a2627, a2628, a2629)
            };
            let a2631 = 0u64;
            if nat_eq(a2630, a2631) {
                f174({
                    let a2632 = v1;
                    let a2633 = 2u64;
                    let a2634 = 0u64;
                    nat_quot(a2632, a2633, a2634)
                }, v4)
            } else {
                if f175(match v2 {
                    Adt3::C0(_, h2635, _, _, _, _, _, _) => {
                        h2635
                    }
                }, v5) {
                    Ok(Adt9::C4(19u64, 1u64, 2u64, 2u64, 0u64, 1u64, 2u64))
                } else {
                    Ok(Adt9::C0(Adt10::C1))
                }
            }
        } else {
            let v6: u64 = {
                let a2638 = v1;
                let a2639 = {
                    let a2636 = 2u64;
                    let a2637 = f140(v4.clone())?;
                    nat_mul(a2636, a2637)?
                };
                nat_sub(a2638, a2639)
            };
            let v7: u64 = {
                let a2643 = v6;
                let a2644 = {
                    let a2641 = 1u64;
                    let a2642 = match v2.clone() {
                        Adt3::C0(h2640, _, _, _, _, _, _, _) => {
                            h2640
                        }
                    };
                    nat_add(a2641, a2642)?
                };
                let a2645 = 0u64;
                nat_quot(a2643, a2644, a2645)
            };
            let v8: u64 = {
                let a2649 = v6;
                let a2650 = {
                    let a2647 = 1u64;
                    let a2648 = match v2.clone() {
                        Adt3::C0(h2646, _, _, _, _, _, _, _) => {
                            h2646
                        }
                    };
                    nat_add(a2647, a2648)?
                };
                let a2651 = 0u64;
                nat_rem(a2649, a2650, a2651)
            };
            let a2652 = v8;
            let a2653 = 0u64;
            if nat_eq(a2652, a2653) {
                f174(v7, v4)
            } else {
                f176(v0, v7, {
                    let a2654 = v8;
                    let a2655 = 1u64;
                    nat_sub(a2654, a2655)
                }, v2, v3, v4, v5)
            }
        }
    } else {
        Ok(Adt9::C0(Adt10::C1))
    }
}

pub fn f88(v0: u64, v1: Adt3, v2: Adt3, v3: Adt3, v4: Adt3, v5: u64) -> R<Adt9> {
    let v6: u64 = {
        let a2658 = match v4.clone() {
            Adt3::C0(h2656, _, _, _, _, _, _, _) => {
                h2656
            }
        };
        let a2659 = match v4.clone() {
            Adt3::C0(_, h2657, _, _, _, _, _, _) => {
                h2657
            }
        };
        nat_mul(a2658, a2659)?
    };
    let v7: u64 = {
        let a2660 = 2u64;
        let a2661 = v6;
        nat_mul(a2660, a2661)?
    };
    let v8: u64 = f177(match v3.clone() {
        Adt3::C0(_, h2662, _, _, _, _, _, _) => {
            h2662
        }
    }, match v3.clone() {
        Adt3::C0(_, _, h2663, _, _, _, _, _) => {
            h2663
        }
    }, match v3.clone() {
        Adt3::C0(_, _, _, h2664, _, _, _, _) => {
            h2664
        }
    });
    let a2665 = v0;
    let a2666 = v7;
    if nat_lt(a2665, a2666) {
        let a2670 = {
            let a2667 = v0;
            let a2668 = 2u64;
            let a2669 = 0u64;
            nat_rem(a2667, a2668, a2669)
        };
        let a2671 = 0u64;
        if nat_eq(a2670, a2671) {
            Ok(Adt9::C4(19u64, 5u64, f152({
                let a2672 = v0;
                let a2673 = 2u64;
                let a2674 = 0u64;
                nat_quot(a2672, a2673, a2674)
            }, v4)?, 2u64, 0u64, 1u64, 2u64))
        } else {
            if f178(v5, v8) {
                Ok(Adt9::C4(19u64, 1u64, 2u64, 2u64, 0u64, 1u64, 2u64))
            } else {
                Ok(Adt9::C0(Adt10::C1))
            }
        }
    } else {
        let v9: u64 = {
            let a2675 = v0;
            let a2676 = v7;
            nat_sub(a2675, a2676)
        };
        let v10: u64 = {
            let a2680 = v9;
            let a2681 = {
                let a2678 = 1u64;
                let a2679 = match v1.clone() {
                    Adt3::C0(h2677, _, _, _, _, _, _, _) => {
                        h2677
                    }
                };
                nat_add(a2678, a2679)?
            };
            let a2682 = 0u64;
            nat_quot(a2680, a2681, a2682)
        };
        let v11: u64 = {
            let a2686 = v9;
            let a2687 = {
                let a2684 = 1u64;
                let a2685 = match v1.clone() {
                    Adt3::C0(h2683, _, _, _, _, _, _, _) => {
                        h2683
                    }
                };
                nat_add(a2684, a2685)?
            };
            let a2688 = 0u64;
            nat_rem(a2686, a2687, a2688)
        };
        let a2689 = v11;
        let a2690 = 0u64;
        if nat_eq(a2689, a2690) {
            Ok(Adt9::C4(19u64, 5u64, f152({
                let a2691 = v10;
                let a2692 = v6;
                let a2693 = 0u64;
                nat_rem(a2691, a2692, a2693)
            }, v4)?, 2u64, 0u64, 1u64, 2u64))
        } else {
            if f178(v5, v8) {
                let v12: u64 = {
                    let a2699 = {
                        let a2695 = v10;
                        let a2696 = match v1.clone() {
                            Adt3::C0(h2694, _, _, _, _, _, _, _) => {
                                h2694
                            }
                        };
                        nat_mul(a2695, a2696)?
                    };
                    let a2700 = {
                        let a2697 = v11;
                        let a2698 = 1u64;
                        nat_sub(a2697, a2698)
                    };
                    nat_add(a2699, a2700)?
                };
                let a2701 = v5;
                let a2702 = 4294967295u64;
                if nat_eq(a2701, a2702) {
                    Ok(Adt9::C1(false, f152(v12, v1)?, f152(v12, v2)?))
                } else {
                    Ok(Adt9::C4(8u64, 0u64, f152(v12, v1)?, 4u64, f152({
                        let a2708 = {
                            let a2704 = v5;
                            let a2705 = match v3.clone() {
                                Adt3::C0(h2703, _, _, _, _, _, _, _) => {
                                    h2703
                                }
                            };
                            nat_mul(a2704, a2705)?
                        };
                        let a2709 = {
                            let a2706 = v11;
                            let a2707 = 1u64;
                            nat_sub(a2706, a2707)
                        };
                        nat_add(a2708, a2709)?
                    }, v3)?, 3u64, f152(v12, v2)?))
                }
            } else {
                Ok(Adt9::C0(Adt10::C1))
            }
        }
    }
}

pub fn f89(v0: u64, v1: Adt3, v2: Adt3, v3: Adt7, v4: u64) -> R<Adt9> {
    let v5: u64 = f121(v3.clone())?;
    let v6: u64 = {
        let a2710 = v0;
        let a2711 = v5;
        let a2712 = 0u64;
        nat_quot(a2710, a2711, a2712)
    };
    let v7: u64 = {
        let a2713 = v0;
        let a2714 = v5;
        let a2715 = 0u64;
        nat_rem(a2713, a2714, a2715)
    };
    let v8: u64 = f179(v3.clone())?;
    let a2716 = v7;
    let a2717 = 0u64;
    if nat_eq(a2716, a2717) {
        Ok(Adt9::C4(19u64, 2u64, if match v3 {
            Adt7::C0(_, _, _, h2718, _) => {
                h2718
            }
        } {
            4286578687u64
        } else {
            0u64
        }, 2u64, 0u64, 1u64, 0u64))
    } else {
        let a2722 = v5;
        let a2723 = {
            let a2720 = v7;
            let a2721 = if match v3.clone() {
                Adt7::C0(_, _, _, h2719, _) => {
                    h2719
                }
            } {
                1u64
            } else {
                2u64
            };
            nat_add(a2720, a2721)?
        };
        if nat_le(a2722, a2723) {
            if match v3.clone() {
                Adt7::C0(_, _, _, h2724, _) => {
                    h2724
                }
            } {
                Ok(Adt9::C4(19u64, 1u64, 0u64, 2u64, 0u64, 3u64, f152(v6, v2)?))
            } else {
                let a2727 = {
                    let a2725 = v7;
                    let a2726 = 2u64;
                    nat_add(a2725, a2726)?
                };
                let a2728 = v5;
                if nat_eq(a2727, a2728) {
                    let v9: u64 = if match v3.clone() {
                        Adt7::C0(_, _, h2729, _, _) => {
                            h2729
                        }
                    } {
                        let a2736 = {
                            let m2732 = match v3.clone() {
                                Adt7::C0(h2731, _, _, _, _) => {
                                    h2731
                                }
                            };
                            match m2732 {
                                Adt6::C0(h2730, _, _, _) => {
                                    h2730
                                }
                            }
                        };
                        let a2737 = {
                            let m2735 = match v3 {
                                Adt7::C0(_, h2734, _, _, _) => {
                                    h2734
                                }
                            };
                            match m2735 {
                                Adt6::C0(h2733, _, _, _) => {
                                    h2733
                                }
                            }
                        };
                        nat_mul(a2736, a2737)?
                    } else {
                        f180(match v3 {
                            Adt7::C0(h2738, _, _, _, _) => {
                                h2738
                            }
                        }, match v1 {
                            Adt3::C0(h2739, _, _, _, _, _, _, _) => {
                                h2739
                            }
                        }, f74(v6, v2, 0u64))?
                    };
                    Ok(Adt9::C4(3u64, 2u64, {
                        let a2740 = v9;
                        let a2741 = 0u64;
                        if nat_eq(a2740, a2741) {
                            1u64
                        } else {
                            v9
                        }
                    }, 2u64, 0u64, 1u64, 1u64))
                } else {
                    Ok(Adt9::C4(5u64, 1u64, 0u64, 1u64, 1u64, 3u64, f152(v6, v2)?))
                }
            }
        } else {
            let v10: u64 = {
                let a2744 = {
                    let a2742 = v7;
                    let a2743 = 1u64;
                    nat_sub(a2742, a2743)
                };
                let a2745 = v8;
                let a2746 = 0u64;
                nat_rem(a2744, a2745, a2746)
            };
            let m2752 = f181(v3.clone(), v1, v2, v6, {
                let a2749 = {
                    let a2747 = v7;
                    let a2748 = 1u64;
                    nat_sub(a2747, a2748)
                };
                let a2750 = v8;
                let a2751 = 0u64;
                nat_quot(a2749, a2750, a2751)
            })?;
            match m2752 {
                None => {
                    Ok(Adt9::C4(19u64, 1u64, 0u64, 2u64, 0u64, 1u64, 0u64))
                }
                Some(v11) => {
                    let a2754 = match v3.clone() {
                        Adt7::C0(_, _, _, _, h2753) => {
                            h2753
                        }
                    };
                    let a2755 = 1u64;
                    if if nat_eq(a2754, a2755) {
                        let a2756 = v10;
                        let a2757 = 0u64;
                        nat_eq(a2756, a2757)
                    } else {
                        false
                    } {
                        Ok(Adt9::C4(6u64, 0u64, v11, 2u64, 0u64, 1u64, 1u64))
                    } else {
                        if match v3.clone() {
                            Adt7::C0(_, _, _, h2758, _) => {
                                h2758
                            }
                        } {
                            let a2761 = {
                                let a2759 = v10;
                                let a2760 = 1u64;
                                nat_add(a2759, a2760)?
                            };
                            let a2762 = v8;
                            if nat_eq(a2761, a2762) {
                                let a2763 = v4;
                                let a2764 = 2u64;
                                if nat_eq(a2763, a2764) {
                                    Ok(Adt9::C4(19u64, 1u64, 0u64, 2u64, 0u64, 1u64, 0u64))
                                } else {
                                    Ok(Adt9::C4(19u64, {
                                        let a2766 = match v3.clone() {
                                            Adt7::C0(_, _, _, _, h2765) => {
                                                h2765
                                            }
                                        };
                                        let a2767 = 1u64;
                                        if nat_eq(a2766, a2767) {
                                            1u64
                                        } else {
                                            0u64
                                        }
                                    }, {
                                        let a2769 = match v3 {
                                            Adt7::C0(_, _, _, _, h2768) => {
                                                h2768
                                            }
                                        };
                                        let a2770 = 1u64;
                                        if nat_eq(a2769, a2770) {
                                            1u64
                                        } else {
                                            v11
                                        }
                                    }, 2u64, 0u64, 1u64, 0u64))
                                }
                            } else {
                                Ok(Adt9::C4(16u64, 1u64, 0u64, {
                                    let a2772 = match v3.clone() {
                                        Adt7::C0(_, _, _, _, h2771) => {
                                            h2771
                                        }
                                    };
                                    let a2773 = 1u64;
                                    if nat_eq(a2772, a2773) {
                                        1u64
                                    } else {
                                        0u64
                                    }
                                }, {
                                    let a2775 = match v3 {
                                        Adt7::C0(_, _, _, _, h2774) => {
                                            h2774
                                        }
                                    };
                                    let a2776 = 1u64;
                                    if nat_eq(a2775, a2776) {
                                        1u64
                                    } else {
                                        v11
                                    }
                                }, 1u64, 2u64))
                            }
                        } else {
                            Ok(Adt9::C4(4u64, 1u64, 0u64, {
                                let a2778 = match v3.clone() {
                                    Adt7::C0(_, _, _, _, h2777) => {
                                        h2777
                                    }
                                };
                                let a2779 = 1u64;
                                if nat_eq(a2778, a2779) {
                                    1u64
                                } else {
                                    0u64
                                }
                            }, {
                                let a2781 = match v3 {
                                    Adt7::C0(_, _, _, _, h2780) => {
                                        h2780
                                    }
                                };
                                let a2782 = 1u64;
                                if nat_eq(a2781, a2782) {
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

pub fn f90(v0: u64, v1: Adt3, v2: Adt3, v3: Adt3) -> R<Adt9> {
    let v4: u64 = f119(match v3.clone() {
        Adt3::C0(h2783, _, _, _, _, _, _, _) => {
            h2783
        }
    })?;
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
            Ok(Adt9::C4(19u64, 1u64, 0u64, 2u64, 0u64, 3u64, f152(v6, v2)?))
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
                    let a2803 = {
                        let a2801 = v5;
                        let a2802 = 1u64;
                        nat_sub(a2801, a2802)
                    };
                    let a2804 = 2u64;
                    let a2805 = 0u64;
                    nat_quot(a2803, a2804, a2805)
                };
                let v8: u64 = f74(v6, v2.clone(), 0u64);
                let v9: u64 = f74(v6, v2.clone(), 1u64);
                let v10: u64 = f74(v6, v2, 2u64);
                Ok(Adt9::C4(10u64, 0u64, f73(v1, {
                    let a2806 = v9;
                    let a2807 = v7;
                    nat_add(a2806, a2807)?
                }, v8, v10, 0u64)?, 4u64, f73(v3, v7, v8, 0u64, 0u64)?, 1u64, 1u64))
            }
        }
    }
}

pub fn f91(v0: u64, v1: Adt3, v2: Adt3, v3: Adt6) -> R<Adt9> {
    let v4: u64 = f118(v3.clone())?;
    let v5: u64 = {
        let a2808 = v0;
        let a2809 = v4;
        let a2810 = 0u64;
        nat_rem(a2808, a2809, a2810)
    };
    let v6: u64 = {
        let a2811 = v0;
        let a2812 = v4;
        let a2813 = 0u64;
        nat_quot(a2811, a2812, a2813)
    };
    let a2814 = v5;
    let a2815 = 0u64;
    if nat_eq(a2814, a2815) {
        Ok(Adt9::C4(19u64, 2u64, 0u64, 2u64, 0u64, 1u64, 0u64))
    } else {
        let a2818 = {
            let a2816 = v5;
            let a2817 = 1u64;
            nat_add(a2816, a2817)?
        };
        let a2819 = v4;
        if nat_eq(a2818, a2819) {
            Ok(Adt9::C4(7u64, 1u64, 0u64, 2u64, 0u64, 3u64, f152(v6, v2)?))
        } else {
            let a2823 = {
                let a2820 = v5;
                let a2821 = 2u64;
                let a2822 = 0u64;
                nat_rem(a2820, a2821, a2822)
            };
            let a2824 = 0u64;
            if nat_eq(a2823, a2824) {
                Ok(Adt9::C4(4u64, 1u64, 0u64, 1u64, 1u64, 1u64, 0u64))
            } else {
                let v7: u64 = {
                    let a2826 = f74(v6, v2.clone(), 0u64);
                    let a2827 = match v3.clone() {
                        Adt6::C0(_, _, h2825, _) => {
                            h2825
                        }
                    };
                    nat_add(a2826, a2827)?
                };
                let v8: u64 = {
                    let a2835 = {
                        let a2832 = {
                            let a2829 = v7;
                            let a2830 = match v3.clone() {
                                Adt6::C0(_, h2828, _, _) => {
                                    h2828
                                }
                            };
                            nat_add(a2829, a2830)?
                        };
                        let a2833 = match v3.clone() {
                            Adt6::C0(h2831, _, _, _) => {
                                h2831
                            }
                        };
                        nat_sub(a2832, a2833)
                    };
                    let a2836 = match v3.clone() {
                        Adt6::C0(_, h2834, _, _) => {
                            h2834
                        }
                    };
                    let a2837 = 0u64;
                    nat_quot(a2835, a2836, a2837)
                };
                let v9: u64 = {
                    let a2843 = v8;
                    let a2844 = {
                        let a2840 = {
                            let a2838 = v5;
                            let a2839 = 1u64;
                            nat_sub(a2838, a2839)
                        };
                        let a2841 = 2u64;
                        let a2842 = 0u64;
                        nat_quot(a2840, a2841, a2842)
                    };
                    nat_add(a2843, a2844)?
                };
                let v10: u64 = {
                    let a2846 = v9;
                    let a2847 = match v3.clone() {
                        Adt6::C0(_, h2845, _, _) => {
                            h2845
                        }
                    };
                    nat_mul(a2846, a2847)?
                };
                let v11: u64 = {
                    let a2848 = v7;
                    let a2849 = v10;
                    nat_sub(a2848, a2849)
                };
                let a2851 = v9;
                let a2852 = match v1.clone() {
                    Adt3::C0(_, h2850, _, _, _, _, _, _) => {
                        h2850
                    }
                };
                if if if nat_lt(a2851, a2852) {
                    let a2853 = v10;
                    let a2854 = v7;
                    nat_le(a2853, a2854)
                } else {
                    false
                } {
                    let a2856 = v11;
                    let a2857 = match v3.clone() {
                        Adt6::C0(h2855, _, _, _) => {
                            h2855
                        }
                    };
                    nat_lt(a2856, a2857)
                } else {
                    false
                } {
                    Ok(Adt9::C4(6u64, 0u64, f73(v1, {
                        let a2861 = {
                            let a2859 = f74(v6, v2, 1u64);
                            let a2860 = match v3 {
                                Adt6::C0(h2858, _, _, _) => {
                                    h2858
                                }
                            };
                            nat_mul(a2859, a2860)?
                        };
                        let a2862 = v11;
                        nat_add(a2861, a2862)?
                    }, v9, 0u64, 0u64)?, 2u64, 0u64, 1u64, 1u64))
                } else {
                    Ok(Adt9::C4(19u64, 2u64, 0u64, 2u64, 0u64, 1u64, 1u64))
                }
            }
        }
    }
}

pub fn f92(v0: u64, v1: Adt3, v2: Adt3, v3: Adt5) -> R<Adt9> {
    let v4: u64 = f74(v0, v2.clone(), 0u64);
    let v5: u64 = f74(v0, v2.clone(), 3u64);
    let v6: u64 = {
        let a2864 = v5;
        let a2865 = match v3.clone() {
            Adt5::C0(_, _, _, _, _, _, h2863, _) => {
                h2863
            }
        };
        let a2866 = 0u64;
        nat_rem(a2864, a2865, a2866)
    };
    let v7: u64 = {
        let a2868 = v5;
        let a2869 = match v3.clone() {
            Adt5::C0(_, _, _, _, _, _, h2867, _) => {
                h2867
            }
        };
        let a2870 = 0u64;
        nat_quot(a2868, a2869, a2870)
    };
    let v8: Option<u64> = f76(match v3.clone() {
        Adt5::C0(h2871, _, _, _, _, _, _, _) => {
            h2871
        }
    }, match v1.clone() {
        Adt3::C0(h2872, _, _, _, _, _, _, _) => {
            h2872
        }
    }, f74(v0, v2.clone(), 1u64), {
        let a2876 = v4;
        let a2877 = {
            let m2875 = match v3.clone() {
                Adt5::C0(h2874, _, _, _, _, _, _, _) => {
                    h2874
                }
            };
            match m2875 {
                Adt6::C0(h2873, _, _, _) => {
                    h2873
                }
            }
        };
        let a2878 = 0u64;
        nat_rem(a2876, a2877, a2878)
    })?;
    let v9: u64 = {
        let a2882 = v4;
        let a2883 = {
            let m2881 = match v3.clone() {
                Adt5::C0(h2880, _, _, _, _, _, _, _) => {
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
        nat_quot(a2882, a2883, a2884)
    };
    let v10: Option<u64> = f76(match v3.clone() {
        Adt5::C0(_, h2885, _, _, _, _, _, _) => {
            h2885
        }
    }, match v1.clone() {
        Adt3::C0(_, h2886, _, _, _, _, _, _) => {
            h2886
        }
    }, f74(v0, v2.clone(), 2u64), {
        let a2890 = v9;
        let a2891 = {
            let m2889 = match v3.clone() {
                Adt5::C0(_, h2888, _, _, _, _, _, _) => {
                    h2888
                }
            };
            match m2889 {
                Adt6::C0(h2887, _, _, _) => {
                    h2887
                }
            }
        };
        let a2892 = 0u64;
        nat_rem(a2890, a2891, a2892)
    })?;
    let v11: u64 = {
        let a2896 = v9;
        let a2897 = {
            let m2895 = match v3.clone() {
                Adt5::C0(_, h2894, _, _, _, _, _, _) => {
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
        nat_quot(a2896, a2897, a2898)
    };
    let v12: Option<u64> = f76(match v3.clone() {
        Adt5::C0(_, _, h2899, _, _, _, _, _) => {
            h2899
        }
    }, match v1.clone() {
        Adt3::C0(_, _, h2900, _, _, _, _, _) => {
            h2900
        }
    }, v6, {
        let a2904 = v11;
        let a2905 = {
            let m2903 = match v3.clone() {
                Adt5::C0(_, _, h2902, _, _, _, _, _) => {
                    h2902
                }
            };
            match m2903 {
                Adt6::C0(h2901, _, _, _) => {
                    h2901
                }
            }
        };
        let a2906 = 0u64;
        nat_rem(a2904, a2905, a2906)
    })?;
    let v13: u64 = {
        let a2910 = v11;
        let a2911 = {
            let m2909 = match v3.clone() {
                Adt5::C0(_, _, h2908, _, _, _, _, _) => {
                    h2908
                }
            };
            match m2909 {
                Adt6::C0(h2907, _, _, _) => {
                    h2907
                }
            }
        };
        let a2912 = 0u64;
        nat_quot(a2910, a2911, a2912)
    };
    let v17: Adt12 = {
        let m2913 = v8;
        match m2913 {
            None => {
                Adt12::C2(0u64)
            }
            Some(v14) => {
                let m2914 = v10;
                match m2914 {
                    None => {
                        Adt12::C2(0u64)
                    }
                    Some(v15) => {
                        let m2915 = v12;
                        match m2915 {
                            None => {
                                Adt12::C2(0u64)
                            }
                            Some(v16) => {
                                Adt12::C0(1u64, {
                                    let a2924 = {
                                        let a2919 = f73(v1, v14, v15, v16, 0u64)?;
                                        let a2920 = {
                                            let a2917 = v7;
                                            let a2918 = match v3.clone() {
                                                Adt5::C0(_, _, _, _, h2916, _, _, _) => {
                                                    h2916
                                                }
                                            };
                                            nat_mul(a2917, a2918)?
                                        };
                                        nat_add(a2919, a2920)?
                                    };
                                    let a2925 = {
                                        let a2922 = v13;
                                        let a2923 = match v3.clone() {
                                            Adt5::C0(_, _, _, h2921, _, _, _, _) => {
                                                h2921
                                            }
                                        };
                                        nat_mul(a2922, a2923)?
                                    };
                                    nat_add(a2924, a2925)?
                                })
                            }
                        }
                    }
                }
            }
        }
    };
    Ok(Adt9::C7(Adt11::C2(match v3.clone() {
        Adt5::C0(_, _, _, _, _, h2926, _, _) => {
            h2926
        }
    }, match v3.clone() {
        Adt5::C0(_, _, _, _, _, _, _, h2927) => {
            h2927
        }
    }, true), v17, Adt12::C2(0u64), Adt13::C1(f152(v0, v2)?), {
        let a2929 = match v3.clone() {
            Adt5::C0(_, _, _, _, _, h2928, _, _) => {
                h2928
            }
        };
        let a2930 = 0u64;
        if nat_eq(a2929, a2930) {
            4u64
        } else {
            2u64
        }
    }, {
        let a2932 = match v3 {
            Adt5::C0(_, _, _, _, _, _, _, h2931) => {
                h2931
            }
        };
        let a2933 = 0u64;
        if nat_eq(a2932, a2933) {
            4u64
        } else {
            2u64
        }
    }))
}

pub fn f93(v0: Adt16, v1: u64, v2: Adt3, v3: Adt3, v4: u64, v5: u64, v6: u64, v7: u64, v8: u64, v9: u64) -> R<Adt9> {
    let m2946 = f182(v0, v1, match v2.clone() {
        Adt3::C0(h2934, _, _, _, _, _, _, _) => {
            h2934
        }
    }, match v2.clone() {
        Adt3::C0(_, h2935, _, _, _, _, _, _) => {
            h2935
        }
    }, match v2.clone() {
        Adt3::C0(_, _, h2936, _, _, _, _, _) => {
            h2936
        }
    }, match v2.clone() {
        Adt3::C0(_, _, _, h2937, _, _, _, _) => {
            h2937
        }
    }, match v2.clone() {
        Adt3::C0(_, _, _, _, h2938, _, _, _) => {
            h2938
        }
    }, match v2.clone() {
        Adt3::C0(_, _, _, _, _, h2939, _, _) => {
            h2939
        }
    }, match v2.clone() {
        Adt3::C0(_, _, _, _, _, _, h2940, _) => {
            h2940
        }
    }, match v2 {
        Adt3::C0(_, _, _, _, _, _, _, h2941) => {
            h2941
        }
    }, v6, v7, v8, v9, match v3.clone() {
        Adt3::C0(_, _, _, _, h2942, _, _, _) => {
            h2942
        }
    }, match v3.clone() {
        Adt3::C0(_, _, _, _, _, h2943, _, _) => {
            h2943
        }
    }, match v3.clone() {
        Adt3::C0(_, _, _, _, _, _, h2944, _) => {
            h2944
        }
    }, match v3 {
        Adt3::C0(_, _, _, _, _, _, _, h2945) => {
            h2945
        }
    }, v4)?;
    match m2946 {
        Adt19::C0 => {
            Ok(Adt9::C0(Adt10::C1))
        }
        Adt19::C1(v10, v11, v12, v13, v14, v15, v16, v17) => {
            Ok(f15(false, v10, v11, 0u64, 0u64, v12, v13, v14, v15, v16, v17, v5, v5))
        }
    }
}

pub fn f94(v0: u64, v1: Adt3, v2: Adt3, v3: u64, v4: u64, v5: u64, v6: u64, v7: u64) -> R<Adt9> {
    let v8: u64 = {
        let a2949 = v6;
        let a2950 = {
            let a2947 = 3u64;
            let a2948 = v5;
            nat_add(a2947, a2948)?
        };
        nat_sub(a2949, a2950)
    };
    let a2952 = v0;
    let a2953 = f36(v3, match v1.clone() {
        Adt3::C0(h2951, _, _, _, _, _, _, _) => {
            h2951
        }
    }, v5, v8)?;
    if nat_lt(a2952, a2953) {
        let v9: u64 = f108(match v1.clone() {
            Adt3::C0(h2954, _, _, _, _, _, _, _) => {
                h2954
            }
        }, v5, v8)?;
        let v10: u64 = f109(v3, match v1.clone() {
            Adt3::C0(h2955, _, _, _, _, _, _, _) => {
                h2955
            }
        }, v5)?;
        let a2956 = v0;
        let a2957 = v9;
        if nat_lt(a2956, a2957) {
            f183(v0, v2, f30(v3), {
                let a2958 = 3u64;
                let a2959 = v5;
                nat_add(a2958, a2959)?
            }, v7)
        } else {
            let a2962 = v0;
            let a2963 = {
                let a2960 = v9;
                let a2961 = v10;
                nat_add(a2960, a2961)?
            };
            if nat_lt(a2962, a2963) {
                let v11: u64 = {
                    let a2964 = v0;
                    let a2965 = v9;
                    nat_sub(a2964, a2965)
                };
                let v12: u64 = f184(v3, match v1.clone() {
                    Adt3::C0(h2966, _, _, _, _, _, _, _) => {
                        h2966
                    }
                })?;
                f185({
                    let a2967 = v11;
                    let a2968 = v12;
                    let a2969 = 0u64;
                    nat_rem(a2967, a2968, a2969)
                }, {
                    let a2970 = v11;
                    let a2971 = v12;
                    let a2972 = 0u64;
                    nat_quot(a2970, a2971, a2972)
                }, v1, v3, v4, v7)
            } else {
                f186({
                    let a2975 = v0;
                    let a2976 = {
                        let a2973 = v9;
                        let a2974 = v10;
                        nat_add(a2973, a2974)?
                    };
                    nat_sub(a2975, a2976)
                }, v1, v2, v3)
            }
        }
    } else {
        Ok(Adt9::C0(Adt10::C1))
    }
}

pub fn f95(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64) -> R<Adt9> {
    let a2977 = v0;
    let a2978 = f19(v1)?;
    if nat_lt(a2977, a2978) {
        let a2979 = v0;
        let a2980 = 4u64;
        if nat_lt(a2979, a2980) {
            let a2981 = v0;
            let a2982 = 0u64;
            if nat_eq(a2981, a2982) {
                Ok(Adt9::C4(16u64, 2u64, v2, 2u64, v3, 1u64, 2u64))
            } else {
                let a2983 = v0;
                let a2984 = 1u64;
                if nat_eq(a2983, a2984) {
                    let a2985 = v5;
                    let a2986 = 0u64;
                    if nat_eq(a2985, a2986) {
                        Ok(Adt9::C4(9u64, 2u64, v3, 2u64, v2, 1u64, 2u64))
                    } else {
                        Ok(Adt9::C0(Adt10::C1))
                    }
                } else {
                    let a2987 = v0;
                    let a2988 = 2u64;
                    if nat_eq(a2987, a2988) {
                        Ok(Adt9::C4(11u64, 1u64, 2u64, 2u64, v4, 1u64, 2u64))
                    } else {
                        Ok(Adt9::C4(20u64, 1u64, 2u64, 2u64, 10u64, 1u64, 2u64))
                    }
                }
            }
        } else {
            let a2989 = v0;
            let a2990 = 4u64;
            if if nat_eq(a2989, a2990) {
                let a2991 = f187(v5)?;
                let a2992 = v1;
                !nat_eq(a2991, a2992)
            } else {
                false
            } {
                Ok(Adt9::C0(Adt10::C1))
            } else {
                let v6: u64 = {
                    let a2995 = {
                        let a2993 = v0;
                        let a2994 = 4u64;
                        nat_sub(a2993, a2994)
                    };
                    let a2996 = 3u64;
                    let a2997 = 0u64;
                    nat_quot(a2995, a2996, a2997)
                };
                let v7: u64 = {
                    let a3000 = {
                        let a2998 = v0;
                        let a2999 = 4u64;
                        nat_sub(a2998, a2999)
                    };
                    let a3001 = 3u64;
                    let a3002 = 0u64;
                    nat_rem(a3000, a3001, a3002)
                };
                let a3003 = v7;
                let a3004 = 0u64;
                if nat_eq(a3003, a3004) {
                    Ok(Adt9::C4(3u64, 2u64, v6, 2u64, 0u64, 1u64, 0u64))
                } else {
                    let a3005 = v7;
                    let a3006 = 1u64;
                    if nat_eq(a3005, a3006) {
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

pub fn f96(v0: bool, v1: u64, v2: u64, v3: u64, v4: u64) -> bool {
    if if if if v0 {
        let a3007 = v1;
        let a3008 = 5u64;
        nat_eq(a3007, a3008)
    } else {
        false
    } {
        let a3009 = v2;
        let a3010 = 16u64;
        nat_eq(a3009, a3010)
    } else {
        false
    } {
        let a3011 = v3;
        let a3012 = 0u64;
        nat_eq(a3011, a3012)
    } else {
        false
    } {
        let a3013 = v4;
        let a3014 = 0u64;
        nat_eq(a3013, a3014)
    } else {
        false
    }
}

pub fn f97(v0: bool, v1: u64, v2: u64, v3: u64, v4: u64) -> Adt11 {
    let a3015 = v1;
    let a3016 = 0u64;
    if nat_eq(a3015, a3016) {
        if v0 {
            let a3017 = v3;
            let a3018 = 0u64;
            if if if nat_eq(a3017, a3018) {
                let a3019 = v4;
                let a3020 = 43u64;
                nat_eq(a3019, a3020)
            } else {
                false
            } {
                true
            } else {
                let a3021 = v3;
                let a3022 = 43u64;
                if nat_eq(a3021, a3022) {
                    let a3023 = v4;
                    let a3024 = 0u64;
                    nat_eq(a3023, a3024)
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
        let a3025 = v1;
        let a3026 = 1u64;
        if nat_eq(a3025, a3026) {
            Adt11::C1(v2)
        } else {
            let a3027 = v1;
            let a3028 = 2u64;
            if nat_eq(a3027, a3028) {
                Adt11::C5(v2)
            } else {
                let a3029 = v1;
                let a3030 = 3u64;
                if nat_eq(a3029, a3030) {
                    Adt11::C3(v2, v2, false)
                } else {
                    let a3031 = v1;
                    let a3032 = 4u64;
                    if nat_eq(a3031, a3032) {
                        Adt11::C2(v3, v4, false)
                    } else {
                        if f96(v0, v1, v2, v3, v4) {
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

pub fn f98(v0: u64, v1: u64) -> Adt12 {
    let a3033 = v0;
    let a3034 = 0u64;
    if nat_eq(a3033, a3034) {
        Adt12::C0(0u64, v1)
    } else {
        let a3035 = v0;
        let a3036 = 1u64;
        if nat_eq(a3035, a3036) {
            Adt12::C1(v1)
        } else {
            let a3037 = v0;
            let a3038 = 2u64;
            if nat_eq(a3037, a3038) {
                Adt12::C2(v1)
            } else {
                let a3039 = v0;
                let a3040 = 3u64;
                if nat_eq(a3039, a3040) {
                    Adt12::C3(v1)
                } else {
                    let a3041 = v0;
                    let a3042 = 13u64;
                    if nat_lt(a3041, a3042) {
                        Adt12::C0({
                            let a3043 = v0;
                            let a3044 = 3u64;
                            nat_sub(a3043, a3044)
                        }, v1)
                    } else {
                        Adt12::C4
                    }
                }
            }
        }
    }
}

pub fn f99(v0: u64, v1: u64) -> Adt13 {
    let a3045 = v0;
    let a3046 = 1u64;
    if nat_eq(a3045, a3046) {
        Adt13::C0(v1)
    } else {
        let a3047 = v0;
        let a3048 = 3u64;
        if if nat_eq(a3047, a3048) {
            true
        } else {
            let a3049 = v0;
            let a3050 = 6u64;
            nat_eq(a3049, a3050)
        } {
            Adt13::C1(v1)
        } else {
            Adt13::C2
        }
    }
}

pub fn f100(v0: Adt6, v1: u64) -> R<u64> {
    let a3052 = 0u64;
    let a3053 = match v0.clone() {
        Adt6::C0(h3051, _, _, _) => {
            h3051
        }
    };
    if if if if if nat_lt(a3052, a3053) {
        let a3055 = 0u64;
        let a3056 = match v0.clone() {
            Adt6::C0(_, h3054, _, _) => {
                h3054
            }
        };
        nat_lt(a3055, a3056)
    } else {
        false
    } {
        let a3058 = 0u64;
        let a3059 = match v0.clone() {
            Adt6::C0(_, _, _, h3057) => {
                h3057
            }
        };
        nat_lt(a3058, a3059)
    } else {
        false
    } {
        let a3061 = match v0.clone() {
            Adt6::C0(_, h3060, _, _) => {
                h3060
            }
        };
        let a3062 = 2147483648u64;
        nat_lt(a3061, a3062)
    } else {
        false
    } {
        let a3064 = match v0.clone() {
            Adt6::C0(_, _, _, h3063) => {
                h3063
            }
        };
        let a3065 = 2147483648u64;
        nat_lt(a3064, a3065)
    } else {
        false
    } {
        let v2: u64 = {
            let a3069 = v1;
            let a3070 = {
                let a3067 = 2u64;
                let a3068 = f146(match v0.clone() {
                    Adt6::C0(_, _, h3066, _) => {
                        h3066
                    }
                });
                nat_mul(a3067, a3068)?
            };
            nat_add(a3069, a3070)?
        };
        let v3: u64 = {
            let a3082 = {
                let a3077 = 1u64;
                let a3078 = {
                    let a3075 = match v0.clone() {
                        Adt6::C0(_, _, _, h3071) => {
                            h3071
                        }
                    };
                    let a3076 = {
                        let a3073 = match v0.clone() {
                            Adt6::C0(h3072, _, _, _) => {
                                h3072
                            }
                        };
                        let a3074 = 1u64;
                        nat_sub(a3073, a3074)
                    };
                    nat_mul(a3075, a3076)?
                };
                nat_add(a3077, a3078)?
            };
            let a3083 = {
                let a3080 = 2u64;
                let a3081 = f145(match v0.clone() {
                    Adt6::C0(_, _, h3079, _) => {
                        h3079
                    }
                });
                nat_mul(a3080, a3081)?
            };
            nat_add(a3082, a3083)?
        };
        let a3084 = v3;
        let a3085 = v2;
        if nat_le(a3084, a3085) {
            let a3092 = 1u64;
            let a3093 = {
                let a3089 = {
                    let a3086 = v2;
                    let a3087 = v3;
                    nat_sub(a3086, a3087)
                };
                let a3090 = match v0 {
                    Adt6::C0(_, h3088, _, _) => {
                        h3088
                    }
                };
                let a3091 = 0u64;
                nat_quot(a3089, a3090, a3091)
            };
            nat_add(a3092, a3093)
        } else {
            let a3097 = {
                let a3094 = v3;
                let a3095 = v2;
                nat_sub(a3094, a3095)
            };
            let a3098 = match v0 {
                Adt6::C0(_, h3096, _, _) => {
                    h3096
                }
            };
            if nat_lt(a3097, a3098) {
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
        let a3099 = v0;
        let a3100 = 4294967296u64;
        let a3101 = 0u64;
        nat_rem(a3099, a3100, a3101)
    };
    let a3102 = v1;
    let a3103 = 2147483648u64;
    if nat_lt(a3102, a3103) {
        v1
    } else {
        0u64
    }
}

pub fn f102(v0: u64) -> R<u64> {
    let a3106 = 2u64;
    let a3107 = {
        let a3104 = 2u64;
        let a3105 = v0;
        nat_mul(a3104, a3105)?
    };
    nat_add(a3106, a3107)
}

pub fn f103(v0: u64, v1: Adt14, v2: Adt14) -> R<Adt15> {
    let a3109 = match v1.clone() {
        Adt14::C0(h3108, _, _, _, _, _, _, _, _, _, _) => {
            h3108
        }
    };
    let a3110 = 0u64;
    if if if if nat_eq(a3109, a3110) {
        let a3112 = match v2.clone() {
            Adt14::C0(h3111, _, _, _, _, _, _, _, _, _, _) => {
                h3111
            }
        };
        let a3113 = 0u64;
        nat_eq(a3112, a3113)
    } else {
        false
    } {
        f188(v1.clone(), v2.clone())
    } else {
        false
    } {
        if f189(match v1.clone() {
            Adt14::C0(_, h3114, _, _, _, _, _, _, _, _, _) => {
                h3114
            }
        }, match v1.clone() {
            Adt14::C0(_, _, h3115, _, _, _, _, _, _, _, _) => {
                h3115
            }
        }, match v1.clone() {
            Adt14::C0(_, _, _, h3116, _, _, _, _, _, _, _) => {
                h3116
            }
        }, match v1.clone() {
            Adt14::C0(_, _, _, _, h3117, _, _, _, _, _, _) => {
                h3117
            }
        }) {
            true
        } else {
            if if if if f31(v0) {
                f190(v1.clone(), v2.clone())
            } else {
                false
            } {
                f191(v1.clone())
            } else {
                false
            } {
                f191(v2.clone())
            } else {
                false
            } {
                if f192(v1.clone(), v2.clone())? {
                    true
                } else {
                    f193(v1.clone(), v2.clone())
                }
            } else {
                false
            }
        }
    } else {
        false
    } {
        let v3: u64 = if f189(match v1.clone() {
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
            0u64
        } else {
            let a3127 = match v1.clone() {
                Adt14::C0(_, _, h3122, _, _, _, _, _, _, _, _) => {
                    h3122
                }
            };
            let a3128 = {
                let a3125 = match v1.clone() {
                    Adt14::C0(_, _, _, h3123, _, _, _, _, _, _, _) => {
                        h3123
                    }
                };
                let a3126 = match v1.clone() {
                    Adt14::C0(_, _, _, _, h3124, _, _, _, _, _, _) => {
                        h3124
                    }
                };
                nat_mul(a3125, a3126)?
            };
            nat_mul(a3127, a3128)?
        };
        let v4: u64 = {
            let a3129 = v3;
            let a3130 = 0u64;
            if if nat_eq(a3129, a3130) {
                true
            } else {
                f194(v2.clone())?
            } {
                0u64
            } else {
                let a3132 = match v2.clone() {
                    Adt14::C0(_, _, _, _, _, _, _, _, _, h3131, _) => {
                        h3131
                    }
                };
                let a3133 = 4u64;
                let a3134 = 0u64;
                nat_quot(a3132, a3133, a3134)
            }
        };
        Ok(Adt15::C1(f195(f196(match v1.clone() {
            Adt14::C0(_, _, _, _, _, h3135, _, _, _, _, _) => {
                h3135
            }
        }, match v2 {
            Adt14::C0(_, _, _, _, _, h3136, _, _, _, _, _) => {
                h3136
            }
        }), match v1 {
            Adt14::C0(_, h3137, _, _, _, _, _, _, _, _, _) => {
                h3137
            }
        }, v3, v4)?, 4u64, 64u64, v3, f197(v4)?, v0))
    } else {
        Ok(Adt15::C0)
    }
}

pub fn f104(v0: u64, v1: u64, v2: Adt14, v3: Adt14) -> R<Adt15> {
    let v4: Adt14 = f198(v3.clone());
    let a3139 = match v2.clone() {
        Adt14::C0(h3138, _, _, _, _, _, _, _, _, _, _) => {
            h3138
        }
    };
    let a3140 = 0u64;
    if if if if nat_eq(a3139, a3140) {
        let a3142 = match v3.clone() {
            Adt14::C0(h3141, _, _, _, _, _, _, _, _, _, _) => {
                h3141
            }
        };
        let a3143 = 0u64;
        nat_eq(a3142, a3143)
    } else {
        false
    } {
        f188(v2.clone(), v3)
    } else {
        false
    } {
        if f189(match v2.clone() {
            Adt14::C0(_, h3144, _, _, _, _, _, _, _, _, _) => {
                h3144
            }
        }, match v2.clone() {
            Adt14::C0(_, _, h3145, _, _, _, _, _, _, _, _) => {
                h3145
            }
        }, match v2.clone() {
            Adt14::C0(_, _, _, h3146, _, _, _, _, _, _, _) => {
                h3146
            }
        }, match v2.clone() {
            Adt14::C0(_, _, _, _, h3147, _, _, _, _, _, _) => {
                h3147
            }
        }) {
            true
        } else {
            let a3149 = match v2.clone() {
                Adt14::C0(_, _, _, _, _, h3148, _, _, _, _, _) => {
                    h3148
                }
            };
            let a3150 = 4u64;
            if if if if if nat_eq(a3149, a3150) {
                f199(match v2.clone() {
                    Adt14::C0(_, h3151, _, _, _, _, _, _, _, _, _) => {
                        h3151
                    }
                }, match v2.clone() {
                    Adt14::C0(_, _, h3152, _, _, _, _, _, _, _, _) => {
                        h3152
                    }
                }, match v2.clone() {
                    Adt14::C0(_, _, _, h3153, _, _, _, _, _, _, _) => {
                        h3153
                    }
                }, v0)?
            } else {
                false
            } {
                f191(v2.clone())
            } else {
                false
            } {
                f191(v4.clone())
            } else {
                false
            } {
                let a3155 = f200(v4.clone())?;
                let a3156 = match v4.clone() {
                    Adt14::C0(_, _, _, _, _, _, _, _, _, h3154, _) => {
                        h3154
                    }
                };
                nat_le(a3155, a3156)
            } else {
                false
            } {
                if f192(v2.clone(), v4.clone())? {
                    true
                } else {
                    f193(v2.clone(), v4.clone())
                }
            } else {
                false
            }
        }
    } else {
        false
    } {
        let v5: u64 = if if f189(match v2.clone() {
            Adt14::C0(_, h3157, _, _, _, _, _, _, _, _, _) => {
                h3157
            }
        }, match v2.clone() {
            Adt14::C0(_, _, h3158, _, _, _, _, _, _, _, _) => {
                h3158
            }
        }, match v2.clone() {
            Adt14::C0(_, _, _, h3159, _, _, _, _, _, _, _) => {
                h3159
            }
        }, match v2.clone() {
            Adt14::C0(_, _, _, _, h3160, _, _, _, _, _, _) => {
                h3160
            }
        }) {
            true
        } else {
            f194(v4.clone())?
        } {
            0u64
        } else {
            let a3162 = match v4 {
                Adt14::C0(_, _, _, _, _, _, _, _, _, h3161, _) => {
                    h3161
                }
            };
            let a3163 = 4u64;
            let a3164 = 0u64;
            nat_quot(a3162, a3163, a3164)
        };
        Ok(Adt15::C1(f201(match v2.clone() {
            Adt14::C0(_, h3165, _, _, _, _, _, _, _, _, _) => {
                h3165
            }
        }, match v2.clone() {
            Adt14::C0(_, _, h3166, _, _, _, _, _, _, _, _) => {
                h3166
            }
        }, match v2.clone() {
            Adt14::C0(_, _, _, h3167, _, _, _, _, _, _, _) => {
                h3167
            }
        }, match v2 {
            Adt14::C0(_, _, _, _, h3168, _, _, _, _, _, _) => {
                h3168
            }
        }, v0, v5)?, 4u64, 65u64, v0, f197(v5)?, v1))
    } else {
        Ok(Adt15::C0)
    }
}

pub fn f105(v0: u64, v1: bool, v2: Adt14, v3: Adt14, v4: Adt14) -> R<Adt15> {
    let v5: Adt14 = f198(v4.clone());
    if if if if if if if v1 {
        let a3170 = match v2.clone() {
            Adt14::C0(h3169, _, _, _, _, _, _, _, _, _, _) => {
                h3169
            }
        };
        let a3171 = 0u64;
        nat_eq(a3170, a3171)
    } else {
        false
    } {
        let a3173 = match v3.clone() {
            Adt14::C0(h3172, _, _, _, _, _, _, _, _, _, _) => {
                h3172
            }
        };
        let a3174 = 0u64;
        nat_eq(a3173, a3174)
    } else {
        false
    } {
        let a3176 = match v4.clone() {
            Adt14::C0(h3175, _, _, _, _, _, _, _, _, _, _) => {
                h3175
            }
        };
        let a3177 = 0u64;
        nat_eq(a3176, a3177)
    } else {
        false
    } {
        f188(v2.clone(), v3.clone())
    } else {
        false
    } {
        f188(v2.clone(), v4)
    } else {
        false
    } {
        if f189(match v2.clone() {
            Adt14::C0(_, h3178, _, _, _, _, _, _, _, _, _) => {
                h3178
            }
        }, match v2.clone() {
            Adt14::C0(_, _, h3179, _, _, _, _, _, _, _, _) => {
                h3179
            }
        }, match v2.clone() {
            Adt14::C0(_, _, _, h3180, _, _, _, _, _, _, _) => {
                h3180
            }
        }, match v2.clone() {
            Adt14::C0(_, _, _, _, h3181, _, _, _, _, _, _) => {
                h3181
            }
        }) {
            true
        } else {
            let a3183 = match v2.clone() {
                Adt14::C0(_, _, _, _, _, h3182, _, _, _, _, _) => {
                    h3182
                }
            };
            let a3184 = 4u64;
            if if if if if if if nat_eq(a3183, a3184) {
                let a3186 = match v3.clone() {
                    Adt14::C0(_, _, _, _, _, h3185, _, _, _, _, _) => {
                        h3185
                    }
                };
                let a3187 = 4u64;
                nat_eq(a3186, a3187)
            } else {
                false
            } {
                f191(v2.clone())
            } else {
                false
            } {
                f191(v3.clone())
            } else {
                false
            } {
                f191(v5.clone())
            } else {
                false
            } {
                let a3189 = f200(v5.clone())?;
                let a3190 = match v5.clone() {
                    Adt14::C0(_, _, _, _, _, _, _, _, _, h3188, _) => {
                        h3188
                    }
                };
                nat_le(a3189, a3190)
            } else {
                false
            } {
                if f192(v2.clone(), v5.clone())? {
                    true
                } else {
                    f193(v2.clone(), v5.clone())
                }
            } else {
                false
            } {
                if f192(v3.clone(), v5.clone())? {
                    true
                } else {
                    f193(v3, v5.clone())
                }
            } else {
                false
            }
        }
    } else {
        false
    } {
        let v6: u64 = if f189(match v2.clone() {
            Adt14::C0(_, h3191, _, _, _, _, _, _, _, _, _) => {
                h3191
            }
        }, match v2.clone() {
            Adt14::C0(_, _, h3192, _, _, _, _, _, _, _, _) => {
                h3192
            }
        }, match v2.clone() {
            Adt14::C0(_, _, _, h3193, _, _, _, _, _, _, _) => {
                h3193
            }
        }, match v2.clone() {
            Adt14::C0(_, _, _, _, h3194, _, _, _, _, _, _) => {
                h3194
            }
        }) {
            0u64
        } else {
            let a3200 = match v2.clone() {
                Adt14::C0(_, _, h3195, _, _, _, _, _, _, _, _) => {
                    h3195
                }
            };
            let a3201 = {
                let a3198 = match v2.clone() {
                    Adt14::C0(_, _, _, h3196, _, _, _, _, _, _, _) => {
                        h3196
                    }
                };
                let a3199 = match v2.clone() {
                    Adt14::C0(_, _, _, _, h3197, _, _, _, _, _, _) => {
                        h3197
                    }
                };
                nat_mul(a3198, a3199)?
            };
            nat_mul(a3200, a3201)?
        };
        let v7: u64 = {
            let a3202 = v6;
            let a3203 = 0u64;
            if if nat_eq(a3202, a3203) {
                true
            } else {
                f194(v5.clone())?
            } {
                0u64
            } else {
                let a3205 = match v5 {
                    Adt14::C0(_, _, _, _, _, _, _, _, _, h3204, _) => {
                        h3204
                    }
                };
                let a3206 = 4u64;
                let a3207 = 0u64;
                nat_quot(a3205, a3206, a3207)
            }
        };
        Ok(Adt15::C1(f202(match v2 {
            Adt14::C0(_, h3208, _, _, _, _, _, _, _, _, _) => {
                h3208
            }
        }, v6, v7)?, 4u64, 66u64, v6, f197(v7)?, v0))
    } else {
        Ok(Adt15::C0)
    }
}

pub fn f106(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64) -> bool {
    if if f203(v0, v3, v6) {
        f203(v1, v4, v6)
    } else {
        false
    } {
        f203(v2, v5, v6)
    } else {
        false
    }
}

pub fn f107(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64) -> R<bool> {
    if if if if if f204(v0, v1, v4, v2, v5, v3, v6)? {
        true
    } else {
        f204(v0, v1, v4, v3, v6, v2, v5)?
    } {
        true
    } else {
        f204(v0, v2, v5, v1, v4, v3, v6)?
    } {
        true
    } else {
        f204(v0, v2, v5, v3, v6, v1, v4)?
    } {
        true
    } else {
        f204(v0, v3, v6, v1, v4, v2, v5)?
    } {
        Ok(true)
    } else {
        f204(v0, v3, v6, v2, v5, v1, v4)
    }
}

pub fn f108(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let a3209 = v2;
    let a3210 = 0u64;
    if nat_eq(a3209, a3210) {
        Ok(0u64)
    } else {
        let a3213 = 2u64;
        let a3214 = {
            let a3211 = v0;
            let a3212 = v1;
            nat_mul(a3211, a3212)?
        };
        nat_mul(a3213, a3214)
    }
}

pub fn f109(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let a3215 = v2;
    let a3216 = f184(v0, v1)?;
    nat_mul(a3215, a3216)
}

pub fn f110(v0: u64) -> u64 {
    let a3217 = v0;
    let a3218 = 30u64;
    if nat_eq(a3217, a3218) {
        3u64
    } else {
        1u64
    }
}

pub fn f111(v0: u64) -> R<u64> {
    let a3221 = 5u64;
    let a3222 = {
        let a3219 = 4u64;
        let a3220 = v0;
        nat_mul(a3219, a3220)?
    };
    nat_add(a3221, a3222)
}

pub fn f112(v0: Adt2, v1: u64, v2: bool, v3: bool) -> R<bool> {
    if v2 {
        if v3 {
            let a3224 = match v0.clone() {
                Adt2::C0(_, _, _, _, h3223, _, _, _, _, _, _) => {
                    h3223
                }
            };
            let a3225 = 1u64;
            if if if if if nat_le(a3224, a3225) {
                true
            } else {
                let a3227 = match v0.clone() {
                    Adt2::C0(_, _, _, _, _, _, _, _, h3226, _, _) => {
                        h3226
                    }
                };
                let a3228 = v1;
                nat_eq(a3227, a3228)
            } {
                let a3230 = match v0.clone() {
                    Adt2::C0(_, h3229, _, _, _, _, _, _, _, _, _) => {
                        h3229
                    }
                };
                let a3231 = 1u64;
                if nat_le(a3230, a3231) {
                    true
                } else {
                    let a3236 = match v0.clone() {
                        Adt2::C0(_, _, _, _, _, h3232, _, _, _, _, _) => {
                            h3232
                        }
                    };
                    let a3237 = {
                        let a3234 = v1;
                        let a3235 = match v0.clone() {
                            Adt2::C0(_, _, _, _, h3233, _, _, _, _, _, _) => {
                                h3233
                            }
                        };
                        nat_mul(a3234, a3235)?
                    };
                    nat_eq(a3236, a3237)
                }
            } else {
                false
            } {
                let a3239 = match v0.clone() {
                    Adt2::C0(_, _, h3238, _, _, _, _, _, _, _, _) => {
                        h3238
                    }
                };
                let a3240 = 1u64;
                if nat_le(a3239, a3240) {
                    true
                } else {
                    let a3248 = match v0.clone() {
                        Adt2::C0(_, _, _, _, _, _, h3241, _, _, _, _) => {
                            h3241
                        }
                    };
                    let a3249 = {
                        let a3246 = {
                            let a3243 = v1;
                            let a3244 = match v0.clone() {
                                Adt2::C0(_, _, _, _, h3242, _, _, _, _, _, _) => {
                                    h3242
                                }
                            };
                            nat_mul(a3243, a3244)?
                        };
                        let a3247 = match v0.clone() {
                            Adt2::C0(_, h3245, _, _, _, _, _, _, _, _, _) => {
                                h3245
                            }
                        };
                        nat_mul(a3246, a3247)?
                    };
                    nat_eq(a3248, a3249)
                }
            } else {
                false
            } {
                let a3251 = match v0.clone() {
                    Adt2::C0(_, _, _, h3250, _, _, _, _, _, _, _) => {
                        h3250
                    }
                };
                let a3252 = 1u64;
                if nat_le(a3251, a3252) {
                    true
                } else {
                    let a3263 = match v0.clone() {
                        Adt2::C0(_, _, _, _, _, _, _, h3253, _, _, _) => {
                            h3253
                        }
                    };
                    let a3264 = {
                        let a3261 = {
                            let a3258 = {
                                let a3255 = v1;
                                let a3256 = match v0.clone() {
                                    Adt2::C0(_, _, _, _, h3254, _, _, _, _, _, _) => {
                                        h3254
                                    }
                                };
                                nat_mul(a3255, a3256)?
                            };
                            let a3259 = match v0.clone() {
                                Adt2::C0(_, h3257, _, _, _, _, _, _, _, _, _) => {
                                    h3257
                                }
                            };
                            nat_mul(a3258, a3259)?
                        };
                        let a3262 = match v0.clone() {
                            Adt2::C0(_, _, h3260, _, _, _, _, _, _, _, _) => {
                                h3260
                            }
                        };
                        nat_mul(a3261, a3262)?
                    };
                    nat_eq(a3263, a3264)
                }
            } else {
                false
            } {
                let a3278 = {
                    let a3275 = {
                        let a3272 = {
                            let a3269 = {
                                let a3266 = v1;
                                let a3267 = match v0.clone() {
                                    Adt2::C0(_, _, _, _, h3265, _, _, _, _, _, _) => {
                                        h3265
                                    }
                                };
                                nat_mul(a3266, a3267)?
                            };
                            let a3270 = match v0.clone() {
                                Adt2::C0(_, h3268, _, _, _, _, _, _, _, _, _) => {
                                    h3268
                                }
                            };
                            nat_mul(a3269, a3270)?
                        };
                        let a3273 = match v0.clone() {
                            Adt2::C0(_, _, h3271, _, _, _, _, _, _, _, _) => {
                                h3271
                            }
                        };
                        nat_mul(a3272, a3273)?
                    };
                    let a3276 = match v0.clone() {
                        Adt2::C0(_, _, _, h3274, _, _, _, _, _, _, _) => {
                            h3274
                        }
                    };
                    nat_mul(a3275, a3276)?
                };
                let a3279 = match v0 {
                    Adt2::C0(_, _, _, _, _, _, _, _, _, h3277, _) => {
                        h3277
                    }
                };
                Ok(nat_le(a3278, a3279))
            } else {
                Ok(false)
            }
        } else {
            let a3281 = match v0.clone() {
                Adt2::C0(_, _, _, h3280, _, _, _, _, _, _, _) => {
                    h3280
                }
            };
            let a3282 = 1u64;
            if if if if if nat_le(a3281, a3282) {
                true
            } else {
                let a3284 = match v0.clone() {
                    Adt2::C0(_, _, _, _, _, _, _, h3283, _, _, _) => {
                        h3283
                    }
                };
                let a3285 = v1;
                nat_eq(a3284, a3285)
            } {
                let a3287 = match v0.clone() {
                    Adt2::C0(_, h3286, _, _, _, _, _, _, _, _, _) => {
                        h3286
                    }
                };
                let a3288 = 1u64;
                if nat_le(a3287, a3288) {
                    true
                } else {
                    let a3293 = match v0.clone() {
                        Adt2::C0(_, _, _, _, _, h3289, _, _, _, _, _) => {
                            h3289
                        }
                    };
                    let a3294 = {
                        let a3291 = v1;
                        let a3292 = match v0.clone() {
                            Adt2::C0(_, _, _, h3290, _, _, _, _, _, _, _) => {
                                h3290
                            }
                        };
                        nat_mul(a3291, a3292)?
                    };
                    nat_eq(a3293, a3294)
                }
            } else {
                false
            } {
                let a3296 = match v0.clone() {
                    Adt2::C0(_, _, h3295, _, _, _, _, _, _, _, _) => {
                        h3295
                    }
                };
                let a3297 = 1u64;
                if nat_le(a3296, a3297) {
                    true
                } else {
                    let a3305 = match v0.clone() {
                        Adt2::C0(_, _, _, _, _, _, h3298, _, _, _, _) => {
                            h3298
                        }
                    };
                    let a3306 = {
                        let a3303 = {
                            let a3300 = v1;
                            let a3301 = match v0.clone() {
                                Adt2::C0(_, _, _, h3299, _, _, _, _, _, _, _) => {
                                    h3299
                                }
                            };
                            nat_mul(a3300, a3301)?
                        };
                        let a3304 = match v0.clone() {
                            Adt2::C0(_, h3302, _, _, _, _, _, _, _, _, _) => {
                                h3302
                            }
                        };
                        nat_mul(a3303, a3304)?
                    };
                    nat_eq(a3305, a3306)
                }
            } else {
                false
            } {
                let a3308 = match v0.clone() {
                    Adt2::C0(_, _, _, _, h3307, _, _, _, _, _, _) => {
                        h3307
                    }
                };
                let a3309 = 1u64;
                if nat_le(a3308, a3309) {
                    true
                } else {
                    let a3320 = match v0.clone() {
                        Adt2::C0(_, _, _, _, _, _, _, _, h3310, _, _) => {
                            h3310
                        }
                    };
                    let a3321 = {
                        let a3318 = {
                            let a3315 = {
                                let a3312 = v1;
                                let a3313 = match v0.clone() {
                                    Adt2::C0(_, _, _, h3311, _, _, _, _, _, _, _) => {
                                        h3311
                                    }
                                };
                                nat_mul(a3312, a3313)?
                            };
                            let a3316 = match v0.clone() {
                                Adt2::C0(_, h3314, _, _, _, _, _, _, _, _, _) => {
                                    h3314
                                }
                            };
                            nat_mul(a3315, a3316)?
                        };
                        let a3319 = match v0.clone() {
                            Adt2::C0(_, _, h3317, _, _, _, _, _, _, _, _) => {
                                h3317
                            }
                        };
                        nat_mul(a3318, a3319)?
                    };
                    nat_eq(a3320, a3321)
                }
            } else {
                false
            } {
                let a3335 = {
                    let a3332 = {
                        let a3329 = {
                            let a3326 = {
                                let a3323 = v1;
                                let a3324 = match v0.clone() {
                                    Adt2::C0(_, _, _, h3322, _, _, _, _, _, _, _) => {
                                        h3322
                                    }
                                };
                                nat_mul(a3323, a3324)?
                            };
                            let a3327 = match v0.clone() {
                                Adt2::C0(_, h3325, _, _, _, _, _, _, _, _, _) => {
                                    h3325
                                }
                            };
                            nat_mul(a3326, a3327)?
                        };
                        let a3330 = match v0.clone() {
                            Adt2::C0(_, _, h3328, _, _, _, _, _, _, _, _) => {
                                h3328
                            }
                        };
                        nat_mul(a3329, a3330)?
                    };
                    let a3333 = match v0.clone() {
                        Adt2::C0(_, _, _, _, h3331, _, _, _, _, _, _) => {
                            h3331
                        }
                    };
                    nat_mul(a3332, a3333)?
                };
                let a3336 = match v0 {
                    Adt2::C0(_, _, _, _, _, _, _, _, _, h3334, _) => {
                        h3334
                    }
                };
                Ok(nat_le(a3335, a3336))
            } else {
                Ok(false)
            }
        }
    } else {
        let a3338 = match v0.clone() {
            Adt2::C0(_, h3337, _, _, _, _, _, _, _, _, _) => {
                h3337
            }
        };
        let a3339 = 1u64;
        if if if if if nat_le(a3338, a3339) {
            true
        } else {
            let a3341 = match v0.clone() {
                Adt2::C0(_, _, _, _, _, h3340, _, _, _, _, _) => {
                    h3340
                }
            };
            let a3342 = v1;
            nat_eq(a3341, a3342)
        } {
            let a3344 = match v0.clone() {
                Adt2::C0(_, _, h3343, _, _, _, _, _, _, _, _) => {
                    h3343
                }
            };
            let a3345 = 1u64;
            if nat_le(a3344, a3345) {
                true
            } else {
                let a3350 = match v0.clone() {
                    Adt2::C0(_, _, _, _, _, _, h3346, _, _, _, _) => {
                        h3346
                    }
                };
                let a3351 = {
                    let a3348 = v1;
                    let a3349 = match v0.clone() {
                        Adt2::C0(_, h3347, _, _, _, _, _, _, _, _, _) => {
                            h3347
                        }
                    };
                    nat_mul(a3348, a3349)?
                };
                nat_eq(a3350, a3351)
            }
        } else {
            false
        } {
            let a3353 = match v0.clone() {
                Adt2::C0(_, _, _, h3352, _, _, _, _, _, _, _) => {
                    h3352
                }
            };
            let a3354 = 1u64;
            if nat_le(a3353, a3354) {
                true
            } else {
                let a3362 = match v0.clone() {
                    Adt2::C0(_, _, _, _, _, _, _, h3355, _, _, _) => {
                        h3355
                    }
                };
                let a3363 = {
                    let a3360 = {
                        let a3357 = v1;
                        let a3358 = match v0.clone() {
                            Adt2::C0(_, h3356, _, _, _, _, _, _, _, _, _) => {
                                h3356
                            }
                        };
                        nat_mul(a3357, a3358)?
                    };
                    let a3361 = match v0.clone() {
                        Adt2::C0(_, _, h3359, _, _, _, _, _, _, _, _) => {
                            h3359
                        }
                    };
                    nat_mul(a3360, a3361)?
                };
                nat_eq(a3362, a3363)
            }
        } else {
            false
        } {
            let a3365 = match v0.clone() {
                Adt2::C0(_, _, _, _, h3364, _, _, _, _, _, _) => {
                    h3364
                }
            };
            let a3366 = 1u64;
            if nat_le(a3365, a3366) {
                true
            } else {
                let a3377 = match v0.clone() {
                    Adt2::C0(_, _, _, _, _, _, _, _, h3367, _, _) => {
                        h3367
                    }
                };
                let a3378 = {
                    let a3375 = {
                        let a3372 = {
                            let a3369 = v1;
                            let a3370 = match v0.clone() {
                                Adt2::C0(_, h3368, _, _, _, _, _, _, _, _, _) => {
                                    h3368
                                }
                            };
                            nat_mul(a3369, a3370)?
                        };
                        let a3373 = match v0.clone() {
                            Adt2::C0(_, _, h3371, _, _, _, _, _, _, _, _) => {
                                h3371
                            }
                        };
                        nat_mul(a3372, a3373)?
                    };
                    let a3376 = match v0.clone() {
                        Adt2::C0(_, _, _, h3374, _, _, _, _, _, _, _) => {
                            h3374
                        }
                    };
                    nat_mul(a3375, a3376)?
                };
                nat_eq(a3377, a3378)
            }
        } else {
            false
        } {
            let a3392 = {
                let a3389 = {
                    let a3386 = {
                        let a3383 = {
                            let a3380 = v1;
                            let a3381 = match v0.clone() {
                                Adt2::C0(_, h3379, _, _, _, _, _, _, _, _, _) => {
                                    h3379
                                }
                            };
                            nat_mul(a3380, a3381)?
                        };
                        let a3384 = match v0.clone() {
                            Adt2::C0(_, _, h3382, _, _, _, _, _, _, _, _) => {
                                h3382
                            }
                        };
                        nat_mul(a3383, a3384)?
                    };
                    let a3387 = match v0.clone() {
                        Adt2::C0(_, _, _, h3385, _, _, _, _, _, _, _) => {
                            h3385
                        }
                    };
                    nat_mul(a3386, a3387)?
                };
                let a3390 = match v0.clone() {
                    Adt2::C0(_, _, _, _, h3388, _, _, _, _, _, _) => {
                        h3388
                    }
                };
                nat_mul(a3389, a3390)?
            };
            let a3393 = match v0 {
                Adt2::C0(_, _, _, _, _, _, _, _, _, h3391, _) => {
                    h3391
                }
            };
            Ok(nat_le(a3392, a3393))
        } else {
            Ok(false)
        }
    }
}

pub fn f113(v0: Adt2) -> R<bool> {
    let v1: u64 = {
        let a3399 = match v0.clone() {
            Adt2::C0(_, _, h3394, _, _, _, _, _, _, _, _) => {
                h3394
            }
        };
        let a3400 = {
            let a3397 = match v0.clone() {
                Adt2::C0(_, _, _, h3395, _, _, _, _, _, _, _) => {
                    h3395
                }
            };
            let a3398 = match v0.clone() {
                Adt2::C0(_, _, _, _, h3396, _, _, _, _, _, _) => {
                    h3396
                }
            };
            nat_mul(a3397, a3398)?
        };
        nat_mul(a3399, a3400)?
    };
    let v2: u64 = f16(v0.clone(), 1u64)?;
    let a3402 = match v0.clone() {
        Adt2::C0(h3401, _, _, _, _, _, _, _, _, _, _) => {
            h3401
        }
    };
    let a3403 = 0u64;
    if if if if if if if nat_eq(a3402, a3403) {
        let a3405 = match v0.clone() {
            Adt2::C0(_, h3404, _, _, _, _, _, _, _, _, _) => {
                h3404
            }
        };
        let a3406 = 2147483647u64;
        nat_le(a3405, a3406)
    } else {
        false
    } {
        let a3407 = v1;
        let a3408 = 2147483647u64;
        nat_le(a3407, a3408)
    } else {
        false
    } {
        let a3410 = match v0.clone() {
            Adt2::C0(_, h3409, _, _, _, _, _, _, _, _, _) => {
                h3409
            }
        };
        let a3411 = 1u64;
        if nat_eq(a3410, a3411) {
            true
        } else {
            let a3413 = match v0.clone() {
                Adt2::C0(_, _, _, _, _, h3412, _, _, _, _, _) => {
                    h3412
                }
            };
            let a3414 = 4u64;
            nat_eq(a3413, a3414)
        }
    } else {
        false
    } {
        let a3415 = v1;
        let a3416 = 1u64;
        if nat_le(a3415, a3416) {
            true
        } else {
            let a3421 = {
                let a3418 = match v0.clone() {
                    Adt2::C0(_, h3417, _, _, _, _, _, _, _, _, _) => {
                        h3417
                    }
                };
                let a3419 = 4u64;
                nat_mul(a3418, a3419)?
            };
            let a3422 = match v0.clone() {
                Adt2::C0(_, _, _, _, _, _, h3420, _, _, _, _) => {
                    h3420
                }
            };
            nat_le(a3421, a3422)
        }
    } else {
        false
    } {
        let a3424 = match v0.clone() {
            Adt2::C0(_, _, _, h3423, _, _, _, _, _, _, _) => {
                h3423
            }
        };
        let a3425 = 1u64;
        if nat_eq(a3424, a3425) {
            true
        } else {
            let a3431 = match v0.clone() {
                Adt2::C0(_, _, _, _, _, _, _, h3426, _, _, _) => {
                    h3426
                }
            };
            let a3432 = {
                let a3429 = match v0.clone() {
                    Adt2::C0(_, _, h3427, _, _, _, _, _, _, _, _) => {
                        h3427
                    }
                };
                let a3430 = match v0.clone() {
                    Adt2::C0(_, _, _, _, _, _, h3428, _, _, _, _) => {
                        h3428
                    }
                };
                nat_mul(a3429, a3430)?
            };
            nat_eq(a3431, a3432)
        }
    } else {
        false
    } {
        let a3434 = match v0.clone() {
            Adt2::C0(_, _, _, _, h3433, _, _, _, _, _, _) => {
                h3433
            }
        };
        let a3435 = 1u64;
        if nat_eq(a3434, a3435) {
            true
        } else {
            let a3444 = match v0.clone() {
                Adt2::C0(_, _, _, _, _, _, _, _, h3436, _, _) => {
                    h3436
                }
            };
            let a3445 = {
                let a3442 = match v0.clone() {
                    Adt2::C0(_, _, _, h3437, _, _, _, _, _, _, _) => {
                        h3437
                    }
                };
                let a3443 = {
                    let a3440 = match v0.clone() {
                        Adt2::C0(_, _, h3438, _, _, _, _, _, _, _, _) => {
                            h3438
                        }
                    };
                    let a3441 = match v0.clone() {
                        Adt2::C0(_, _, _, _, _, _, h3439, _, _, _, _) => {
                            h3439
                        }
                    };
                    nat_mul(a3440, a3441)?
                };
                nat_mul(a3442, a3443)?
            };
            nat_eq(a3444, a3445)
        }
    } else {
        false
    } {
        let a3446 = v2;
        let a3447 = 0u64;
        if nat_eq(a3446, a3447) {
            Ok(true)
        } else {
            let a3453 = {
                let a3450 = f152({
                    let a3448 = v2;
                    let a3449 = 1u64;
                    nat_sub(a3448, a3449)
                }, f20(v0.clone(), 1u64))?;
                let a3451 = 4u64;
                nat_add(a3450, a3451)?
            };
            let a3454 = match v0 {
                Adt2::C0(_, _, _, _, _, _, _, _, _, h3452, _) => {
                    h3452
                }
            };
            Ok(nat_le(a3453, a3454))
        }
    } else {
        Ok(false)
    }
}

pub fn f114(v0: Adt2, v1: u64) -> R<bool> {
    let a3456 = match v0.clone() {
        Adt2::C0(h3455, _, _, _, _, _, _, _, _, _, _) => {
            h3455
        }
    };
    let a3457 = 0u64;
    if if nat_eq(a3456, a3457) {
        f22(v0.clone())?
    } else {
        false
    } {
        let a3458 = v1;
        let a3459 = 1u64;
        if nat_le(a3458, a3459) {
            Ok(true)
        } else {
            let a3464 = match v0.clone() {
                Adt2::C0(_, _, _, _, _, _, h3460, _, _, _, _) => {
                    h3460
                }
            };
            let a3465 = {
                let a3462 = 4u64;
                let a3463 = match v0 {
                    Adt2::C0(_, h3461, _, _, _, _, _, _, _, _, _) => {
                        h3461
                    }
                };
                nat_mul(a3462, a3463)?
            };
            Ok(nat_eq(a3464, a3465))
        }
    } else {
        Ok(false)
    }
}

pub fn f115(v0: u64) -> R<u64> {
    let a3468 = 2u64;
    let a3469 = {
        let a3466 = 6u64;
        let a3467 = v0;
        nat_mul(a3466, a3467)?
    };
    nat_add(a3468, a3469)
}

pub fn f116(v0: u64) -> bool {
    let a3470 = v0;
    let a3471 = 0u64;
    if if nat_eq(a3470, a3471) {
        true
    } else {
        let a3472 = v0;
        let a3473 = 1u64;
        nat_eq(a3472, a3473)
    } {
        true
    } else {
        let a3474 = v0;
        let a3475 = 30u64;
        nat_eq(a3474, a3475)
    }
}

pub fn f117(v0: Adt2, v1: u64) -> R<bool> {
    let a3477 = match v0.clone() {
        Adt2::C0(_, _, _, h3476, _, _, _, _, _, _, _) => {
            h3476
        }
    };
    let a3478 = 1u64;
    if if if nat_eq(a3477, a3478) {
        let a3480 = match v0.clone() {
            Adt2::C0(_, _, _, _, h3479, _, _, _, _, _, _) => {
                h3479
            }
        };
        let a3481 = 1u64;
        nat_eq(a3480, a3481)
    } else {
        false
    } {
        let a3483 = match v0.clone() {
            Adt2::C0(_, _, _, _, _, h3482, _, _, _, _, _) => {
                h3482
            }
        };
        let a3484 = v1;
        nat_eq(a3483, a3484)
    } else {
        false
    } {
        let a3486 = match v0.clone() {
            Adt2::C0(_, _, h3485, _, _, _, _, _, _, _, _) => {
                h3485
            }
        };
        let a3487 = 1u64;
        if nat_eq(a3486, a3487) {
            Ok(true)
        } else {
            let a3492 = match v0.clone() {
                Adt2::C0(_, _, _, _, _, _, h3488, _, _, _, _) => {
                    h3488
                }
            };
            let a3493 = {
                let a3490 = match v0 {
                    Adt2::C0(_, h3489, _, _, _, _, _, _, _, _, _) => {
                        h3489
                    }
                };
                let a3491 = v1;
                nat_mul(a3490, a3491)?
            };
            Ok(nat_eq(a3492, a3493))
        }
    } else {
        Ok(false)
    }
}

pub fn f118(v0: Adt6) -> R<u64> {
    let a3506 = 2u64;
    let a3507 = {
        let a3504 = 2u64;
        let a3505 = {
            let a3501 = {
                let a3498 = match v0.clone() {
                    Adt6::C0(h3494, _, _, _) => {
                        h3494
                    }
                };
                let a3499 = {
                    let a3496 = match v0.clone() {
                        Adt6::C0(_, h3495, _, _) => {
                            h3495
                        }
                    };
                    let a3497 = 1u64;
                    nat_sub(a3496, a3497)
                };
                nat_add(a3498, a3499)?
            };
            let a3502 = match v0 {
                Adt6::C0(_, h3500, _, _) => {
                    h3500
                }
            };
            let a3503 = 0u64;
            nat_quot(a3501, a3502, a3503)
        };
        nat_mul(a3504, a3505)?
    };
    nat_add(a3506, a3507)
}

pub fn f119(v0: u64) -> R<u64> {
    let a3510 = 2u64;
    let a3511 = {
        let a3508 = v0;
        let a3509 = 2u64;
        nat_mul(a3508, a3509)?
    };
    nat_add(a3510, a3511)
}

pub fn f120(v0: Adt6, v1: u64, v2: u64) -> R<bool> {
    let v3: u64 = {
        let a3515 = {
            let a3512 = v2;
            let a3513 = 1u64;
            nat_sub(a3512, a3513)
        };
        let a3516 = match v0.clone() {
            Adt6::C0(_, h3514, _, _) => {
                h3514
            }
        };
        nat_mul(a3515, a3516)?
    };
    let a3518 = 0u64;
    let a3519 = match v0.clone() {
        Adt6::C0(h3517, _, _, _) => {
            h3517
        }
    };
    if if if if if if if if nat_lt(a3518, a3519) {
        let a3521 = match v0.clone() {
            Adt6::C0(h3520, _, _, _) => {
                h3520
            }
        };
        let a3522 = 2147483648u64;
        nat_lt(a3521, a3522)
    } else {
        false
    } {
        let a3524 = 0u64;
        let a3525 = match v0.clone() {
            Adt6::C0(_, h3523, _, _) => {
                h3523
            }
        };
        nat_lt(a3524, a3525)
    } else {
        false
    } {
        let a3527 = match v0.clone() {
            Adt6::C0(_, h3526, _, _) => {
                h3526
            }
        };
        let a3528 = 2147483648u64;
        nat_lt(a3527, a3528)
    } else {
        false
    } {
        let a3529 = v1;
        let a3530 = 2147483648u64;
        nat_lt(a3529, a3530)
    } else {
        false
    } {
        let a3531 = 0u64;
        let a3532 = v2;
        nat_lt(a3531, a3532)
    } else {
        false
    } {
        let a3533 = v3;
        let a3534 = 2147483648u64;
        nat_lt(a3533, a3534)
    } else {
        false
    } {
        let a3546 = {
            let a3541 = {
                let a3538 = v3;
                let a3539 = {
                    let a3536 = match v0.clone() {
                        Adt6::C0(h3535, _, _, _) => {
                            h3535
                        }
                    };
                    let a3537 = 1u64;
                    nat_sub(a3536, a3537)
                };
                nat_add(a3538, a3539)?
            };
            let a3542 = f145(match v0.clone() {
                Adt6::C0(_, _, h3540, _) => {
                    h3540
                }
            });
            nat_add(a3541, a3542)?
        };
        let a3547 = {
            let a3544 = 2147483647u64;
            let a3545 = f146(match v0.clone() {
                Adt6::C0(_, _, h3543, _) => {
                    h3543
                }
            });
            nat_add(a3544, a3545)?
        };
        nat_le(a3546, a3547)
    } else {
        false
    } {
        let a3549 = match v0 {
            Adt6::C0(_, _, h3548, _) => {
                h3548
            }
        };
        let a3550 = 2147483648u64;
        Ok(!nat_eq(a3549, a3550))
    } else {
        Ok(false)
    }
}

pub fn f121(v0: Adt7) -> R<u64> {
    let a3562 = if match v0.clone() {
        Adt7::C0(_, _, _, h3551, _) => {
            h3551
        }
    } {
        2u64
    } else {
        3u64
    };
    let a3563 = {
        let a3560 = {
            let a3558 = {
                let m3554 = match v0.clone() {
                    Adt7::C0(h3553, _, _, _, _) => {
                        h3553
                    }
                };
                match m3554 {
                    Adt6::C0(h3552, _, _, _) => {
                        h3552
                    }
                }
            };
            let a3559 = {
                let m3557 = match v0.clone() {
                    Adt7::C0(_, h3556, _, _, _) => {
                        h3556
                    }
                };
                match m3557 {
                    Adt6::C0(h3555, _, _, _) => {
                        h3555
                    }
                }
            };
            nat_mul(a3558, a3559)?
        };
        let a3561 = f179(v0)?;
        nat_mul(a3560, a3561)?
    };
    nat_add(a3562, a3563)
}

pub fn f122(v0: Adt2, v1: Adt2, v2: Adt2) -> R<Adt10> {
    if f205(match v0.clone() {
        Adt2::C0(h3564, _, _, _, _, _, _, _, _, _, _) => {
            h3564
        }
    }, match v0.clone() {
        Adt2::C0(_, h3565, _, _, _, _, _, _, _, _, _) => {
            h3565
        }
    }, match v0.clone() {
        Adt2::C0(_, _, h3566, _, _, _, _, _, _, _, _) => {
            h3566
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, h3567, _, _, _, _, _, _, _) => {
            h3567
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, h3568, _, _, _, _, _, _) => {
            h3568
        }
    }, match v1.clone() {
        Adt2::C0(h3569, _, _, _, _, _, _, _, _, _, _) => {
            h3569
        }
    }, match v1.clone() {
        Adt2::C0(_, h3570, _, _, _, _, _, _, _, _, _) => {
            h3570
        }
    }, match v1.clone() {
        Adt2::C0(_, _, h3571, _, _, _, _, _, _, _, _) => {
            h3571
        }
    }, match v1.clone() {
        Adt2::C0(_, _, _, h3572, _, _, _, _, _, _, _) => {
            h3572
        }
    }, match v1.clone() {
        Adt2::C0(_, _, _, _, h3573, _, _, _, _, _, _) => {
            h3573
        }
    }, match v2.clone() {
        Adt2::C0(h3574, _, _, _, _, _, _, _, _, _, _) => {
            h3574
        }
    }, match v2.clone() {
        Adt2::C0(_, h3575, _, _, _, _, _, _, _, _, _) => {
            h3575
        }
    }, match v2.clone() {
        Adt2::C0(_, _, h3576, _, _, _, _, _, _, _, _) => {
            h3576
        }
    }, match v2.clone() {
        Adt2::C0(_, _, _, h3577, _, _, _, _, _, _, _) => {
            h3577
        }
    }, match v2.clone() {
        Adt2::C0(_, _, _, _, h3578, _, _, _, _, _, _) => {
            h3578
        }
    }) {
        let m3580 = f127(match v0.clone() {
            Adt2::C0(h3579, _, _, _, _, _, _, _, _, _, _) => {
                h3579
            }
        });
        match m3580 {
            Adt17::C0 => {
                Ok(Adt10::C0)
            }
            Adt17::C1(v3) => {
                let a3586 = {
                    let a3583 = match v0.clone() {
                        Adt2::C0(_, h3581, _, _, _, _, _, _, _, _, _) => {
                            h3581
                        }
                    };
                    let a3584 = match v3.clone() {
                        Adt18::C0(_, h3582, _) => {
                            h3582
                        }
                    };
                    let a3585 = 0u64;
                    nat_rem(a3583, a3584, a3585)
                };
                let a3587 = 0u64;
                if if nat_eq(a3586, a3587) {
                    if f206(match v0.clone() {
                        Adt2::C0(_, h3588, _, _, _, _, _, _, _, _, _) => {
                            h3588
                        }
                    }, match v1.clone() {
                        Adt2::C0(_, h3589, _, _, _, _, _, _, _, _, _) => {
                            h3589
                        }
                    }, match v1.clone() {
                        Adt2::C0(_, _, h3590, _, _, _, _, _, _, _, _) => {
                            h3590
                        }
                    }, match v1.clone() {
                        Adt2::C0(_, _, _, h3591, _, _, _, _, _, _, _) => {
                            h3591
                        }
                    }) {
                        true
                    } else {
                        let a3593 = 4u64;
                        let a3594 = match v2.clone() {
                            Adt2::C0(_, _, _, _, _, h3592, _, _, _, _, _) => {
                                h3592
                            }
                        };
                        if if nat_le(a3593, a3594) {
                            let a3596 = match v3.clone() {
                                Adt18::C0(_, h3595, _) => {
                                    h3595
                                }
                            };
                            let a3597 = 1u64;
                            if nat_eq(a3596, a3597) {
                                true
                            } else {
                                let a3600 = match v3.clone() {
                                    Adt18::C0(_, _, h3598) => {
                                        h3598
                                    }
                                };
                                let a3601 = match v0.clone() {
                                    Adt2::C0(_, _, _, _, _, h3599, _, _, _, _, _) => {
                                        h3599
                                    }
                                };
                                nat_le(a3600, a3601)
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
                        Adt18::C0(_, h3602, _) => {
                            h3602
                        }
                    });
                    Ok(Adt10::C4(if f206(match v0.clone() {
                        Adt2::C0(_, h3603, _, _, _, _, _, _, _, _, _) => {
                            h3603
                        }
                    }, match v1.clone() {
                        Adt2::C0(_, h3604, _, _, _, _, _, _, _, _, _) => {
                            h3604
                        }
                    }, match v1.clone() {
                        Adt2::C0(_, _, h3605, _, _, _, _, _, _, _, _) => {
                            h3605
                        }
                    }, match v1.clone() {
                        Adt2::C0(_, _, _, h3606, _, _, _, _, _, _, _) => {
                            h3606
                        }
                    }) {
                        0u64
                    } else {
                        f173(v4.clone(), f20(v1.clone(), 1u64))?
                    }, 4u64, v4, f20(v2, 1u64), Adt4::C20(match v0 {
                        Adt2::C0(h3607, _, _, _, _, _, _, _, _, _, _) => {
                            h3607
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

pub fn f123(v0: Adt2, v1: Adt2, v2: Adt2) -> R<Adt10> {
    if if if if if if if if if if if if if f125(match v0.clone() {
        Adt2::C0(h3608, _, _, _, _, _, _, _, _, _, _) => {
            h3608
        }
    }) {
        let a3614 = {
            let a3611 = match v0.clone() {
                Adt2::C0(_, h3609, _, _, _, _, _, _, _, _, _) => {
                    h3609
                }
            };
            let a3612 = f207(match v0.clone() {
                Adt2::C0(h3610, _, _, _, _, _, _, _, _, _, _) => {
                    h3610
                }
            });
            let a3613 = 1u64;
            nat_rem(a3611, a3612, a3613)
        };
        let a3615 = 0u64;
        nat_eq(a3614, a3615)
    } else {
        false
    } {
        let a3617 = match v1.clone() {
            Adt2::C0(h3616, _, _, _, _, _, _, _, _, _, _) => {
                h3616
            }
        };
        let a3618 = 26u64;
        nat_eq(a3617, a3618)
    } else {
        false
    } {
        let a3620 = match v2.clone() {
            Adt2::C0(h3619, _, _, _, _, _, _, _, _, _, _) => {
                h3619
            }
        };
        let a3621 = 0u64;
        nat_eq(a3620, a3621)
    } else {
        false
    } {
        let a3624 = match v0.clone() {
            Adt2::C0(_, h3622, _, _, _, _, _, _, _, _, _) => {
                h3622
            }
        };
        let a3625 = match v2.clone() {
            Adt2::C0(_, h3623, _, _, _, _, _, _, _, _, _) => {
                h3623
            }
        };
        nat_eq(a3624, a3625)
    } else {
        false
    } {
        let a3628 = match v1.clone() {
            Adt2::C0(_, h3626, _, _, _, _, _, _, _, _, _) => {
                h3626
            }
        };
        let a3629 = match v0.clone() {
            Adt2::C0(_, _, h3627, _, _, _, _, _, _, _, _) => {
                h3627
            }
        };
        nat_le(a3628, a3629)
    } else {
        false
    } {
        let a3631 = match v0.clone() {
            Adt2::C0(_, _, _, h3630, _, _, _, _, _, _, _) => {
                h3630
            }
        };
        let a3632 = 1u64;
        nat_eq(a3631, a3632)
    } else {
        false
    } {
        let a3634 = match v0.clone() {
            Adt2::C0(_, _, _, _, h3633, _, _, _, _, _, _) => {
                h3633
            }
        };
        let a3635 = 1u64;
        nat_eq(a3634, a3635)
    } else {
        false
    } {
        let a3637 = match v2.clone() {
            Adt2::C0(_, _, _, h3636, _, _, _, _, _, _, _) => {
                h3636
            }
        };
        let a3638 = 1u64;
        nat_eq(a3637, a3638)
    } else {
        false
    } {
        let a3640 = match v2.clone() {
            Adt2::C0(_, _, _, _, h3639, _, _, _, _, _, _) => {
                h3639
            }
        };
        let a3641 = 1u64;
        nat_eq(a3640, a3641)
    } else {
        false
    } {
        let a3643 = match v1.clone() {
            Adt2::C0(_, _, h3642, _, _, _, _, _, _, _, _) => {
                h3642
            }
        };
        let a3644 = 1u64;
        nat_eq(a3643, a3644)
    } else {
        false
    } {
        let a3646 = match v1.clone() {
            Adt2::C0(_, _, _, h3645, _, _, _, _, _, _, _) => {
                h3645
            }
        };
        let a3647 = 1u64;
        nat_eq(a3646, a3647)
    } else {
        false
    } {
        let a3649 = match v1.clone() {
            Adt2::C0(_, _, _, _, h3648, _, _, _, _, _, _) => {
                h3648
            }
        };
        let a3650 = 1u64;
        nat_eq(a3649, a3650)
    } else {
        false
    } {
        if f34(match v2.clone() {
            Adt2::C0(_, h3651, _, _, _, _, _, _, _, _, _) => {
                h3651
            }
        }, match v2.clone() {
            Adt2::C0(_, _, h3652, _, _, _, _, _, _, _, _) => {
                h3652
            }
        }, match v2.clone() {
            Adt2::C0(_, _, _, h3653, _, _, _, _, _, _, _) => {
                h3653
            }
        }, match v2.clone() {
            Adt2::C0(_, _, _, _, h3654, _, _, _, _, _, _) => {
                h3654
            }
        }) {
            Ok(Adt10::C4(0u64, 4u64, f20(v0, 1u64), f20(v2, 1u64), Adt4::C9(0u64, 0u64)))
        } else {
            let a3656 = match v1.clone() {
                Adt2::C0(_, h3655, _, _, _, _, _, _, _, _, _) => {
                    h3655
                }
            };
            let a3657 = 0u64;
            if nat_eq(a3656, a3657) {
                Ok(Adt10::C4(f16(v2.clone(), 1u64)?, 4u64, f20(v0, 1u64), f20(v2, 1u64), Adt4::C9(0u64, 0u64)))
            } else {
                let a3659 = match v0.clone() {
                    Adt2::C0(_, h3658, _, _, _, _, _, _, _, _, _) => {
                        h3658
                    }
                };
                let a3660 = 2147483647u64;
                if if if if if if nat_le(a3659, a3660) {
                    let a3662 = match v1.clone() {
                        Adt2::C0(_, h3661, _, _, _, _, _, _, _, _, _) => {
                            h3661
                        }
                    };
                    let a3663 = 2147483647u64;
                    nat_le(a3662, a3663)
                } else {
                    false
                } {
                    let a3665 = 4u64;
                    let a3666 = match v2.clone() {
                        Adt2::C0(_, _, _, _, _, h3664, _, _, _, _, _) => {
                            h3664
                        }
                    };
                    nat_le(a3665, a3666)
                } else {
                    false
                } {
                    let a3672 = {
                        let a3669 = match v2.clone() {
                            Adt2::C0(_, h3667, _, _, _, _, _, _, _, _, _) => {
                                h3667
                            }
                        };
                        let a3670 = match v2.clone() {
                            Adt2::C0(_, _, _, _, _, h3668, _, _, _, _, _) => {
                                h3668
                            }
                        };
                        nat_mul(a3669, a3670)?
                    };
                    let a3673 = match v2.clone() {
                        Adt2::C0(_, _, _, _, _, _, h3671, _, _, _, _) => {
                            h3671
                        }
                    };
                    nat_le(a3672, a3673)
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
                        let a3686 = {
                            let a3677 = {
                                let a3675 = 2u64;
                                let a3676 = match v1.clone() {
                                    Adt2::C0(_, h3674, _, _, _, _, _, _, _, _, _) => {
                                        h3674
                                    }
                                };
                                nat_mul(a3675, a3676)?
                            };
                            let a3678 = f16(v2.clone(), 1u64)?;
                            nat_add(a3677, a3678)?
                        };
                        let a3687 = {
                            let a3684 = match v1.clone() {
                                Adt2::C0(_, h3679, _, _, _, _, _, _, _, _, _) => {
                                    h3679
                                }
                            };
                            let a3685 = {
                                let a3682 = 1u64;
                                let a3683 = f171(match v0.clone() {
                                    Adt2::C0(h3680, _, _, _, _, _, _, _, _, _, _) => {
                                        h3680
                                    }
                                }, match v0.clone() {
                                    Adt2::C0(_, h3681, _, _, _, _, _, _, _, _, _) => {
                                        h3681
                                    }
                                })?;
                                nat_add(a3682, a3683)?
                            };
                            nat_mul(a3684, a3685)?
                        };
                        nat_add(a3686, a3687)?
                    }, 4u64, Adt3::C0(match v0.clone() {
                        Adt2::C0(_, h3688, _, _, _, _, _, _, _, _, _) => {
                            h3688
                        }
                    }, match v1.clone() {
                        Adt2::C0(_, h3689, _, _, _, _, _, _, _, _, _) => {
                            h3689
                        }
                    }, 1u64, 1u64, match v0.clone() {
                        Adt2::C0(_, _, _, _, _, h3690, _, _, _, _, _) => {
                            h3690
                        }
                    }, match v0.clone() {
                        Adt2::C0(_, _, _, _, _, _, h3691, _, _, _, _) => {
                            h3691
                        }
                    }, match v0.clone() {
                        Adt2::C0(_, _, _, _, _, _, _, h3692, _, _, _) => {
                            h3692
                        }
                    }, match v0.clone() {
                        Adt2::C0(_, _, _, _, _, _, _, _, h3693, _, _) => {
                            h3693
                        }
                    }), f20(v2, 1u64), Adt4::C19(match v0 {
                        Adt2::C0(h3694, _, _, _, _, _, _, _, _, _, _) => {
                            h3694
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

pub fn f124(v0: Adt2, v1: u64) -> R<bool> {
    let a3695 = v1;
    let a3696 = 0u64;
    if nat_eq(a3695, a3696) {
        Ok(true)
    } else {
        let a3702 = {
            let a3699 = f152({
                let a3697 = v1;
                let a3698 = 1u64;
                nat_sub(a3697, a3698)
            }, f20(v0.clone(), 1u64))?;
            let a3700 = 4u64;
            nat_add(a3699, a3700)?
        };
        let a3703 = match v0 {
            Adt2::C0(_, _, _, _, _, _, _, _, _, h3701, _) => {
                h3701
            }
        };
        Ok(nat_le(a3702, a3703))
    }
}

pub fn f125(v0: u64) -> bool {
    let a3704 = v0;
    let a3705 = 0u64;
    if if if if if if if if if if if if if if if if if if if if if if if if if if if if if nat_eq(a3704, a3705) {
        true
    } else {
        let a3706 = v0;
        let a3707 = 1u64;
        nat_eq(a3706, a3707)
    } {
        true
    } else {
        let a3708 = v0;
        let a3709 = 2u64;
        nat_eq(a3708, a3709)
    } {
        true
    } else {
        let a3710 = v0;
        let a3711 = 3u64;
        nat_eq(a3710, a3711)
    } {
        true
    } else {
        let a3712 = v0;
        let a3713 = 6u64;
        nat_eq(a3712, a3713)
    } {
        true
    } else {
        let a3714 = v0;
        let a3715 = 7u64;
        nat_eq(a3714, a3715)
    } {
        true
    } else {
        let a3716 = v0;
        let a3717 = 8u64;
        nat_eq(a3716, a3717)
    } {
        true
    } else {
        let a3718 = v0;
        let a3719 = 10u64;
        nat_eq(a3718, a3719)
    } {
        true
    } else {
        let a3720 = v0;
        let a3721 = 11u64;
        nat_eq(a3720, a3721)
    } {
        true
    } else {
        let a3722 = v0;
        let a3723 = 12u64;
        nat_eq(a3722, a3723)
    } {
        true
    } else {
        let a3724 = v0;
        let a3725 = 13u64;
        nat_eq(a3724, a3725)
    } {
        true
    } else {
        let a3726 = v0;
        let a3727 = 14u64;
        nat_eq(a3726, a3727)
    } {
        true
    } else {
        let a3728 = v0;
        let a3729 = 16u64;
        nat_eq(a3728, a3729)
    } {
        true
    } else {
        let a3730 = v0;
        let a3731 = 17u64;
        nat_eq(a3730, a3731)
    } {
        true
    } else {
        let a3732 = v0;
        let a3733 = 18u64;
        nat_eq(a3732, a3733)
    } {
        true
    } else {
        let a3734 = v0;
        let a3735 = 19u64;
        nat_eq(a3734, a3735)
    } {
        true
    } else {
        let a3736 = v0;
        let a3737 = 20u64;
        nat_eq(a3736, a3737)
    } {
        true
    } else {
        let a3738 = v0;
        let a3739 = 21u64;
        nat_eq(a3738, a3739)
    } {
        true
    } else {
        let a3740 = v0;
        let a3741 = 22u64;
        nat_eq(a3740, a3741)
    } {
        true
    } else {
        let a3742 = v0;
        let a3743 = 23u64;
        nat_eq(a3742, a3743)
    } {
        true
    } else {
        let a3744 = v0;
        let a3745 = 26u64;
        nat_eq(a3744, a3745)
    } {
        true
    } else {
        let a3746 = v0;
        let a3747 = 29u64;
        nat_eq(a3746, a3747)
    } {
        true
    } else {
        let a3748 = v0;
        let a3749 = 30u64;
        nat_eq(a3748, a3749)
    } {
        true
    } else {
        let a3750 = v0;
        let a3751 = 34u64;
        nat_eq(a3750, a3751)
    } {
        true
    } else {
        let a3752 = v0;
        let a3753 = 35u64;
        nat_eq(a3752, a3753)
    } {
        true
    } else {
        let a3754 = v0;
        let a3755 = 39u64;
        nat_eq(a3754, a3755)
    } {
        true
    } else {
        let a3756 = v0;
        let a3757 = 40u64;
        nat_eq(a3756, a3757)
    } {
        true
    } else {
        let a3758 = v0;
        let a3759 = 41u64;
        nat_eq(a3758, a3759)
    } {
        true
    } else {
        let a3760 = v0;
        let a3761 = 42u64;
        nat_eq(a3760, a3761)
    } {
        true
    } else {
        let a3762 = v0;
        let a3763 = 43u64;
        nat_eq(a3762, a3763)
    }
}

pub fn f126(v0: u64) -> bool {
    if f116(v0) {
        true
    } else {
        let a3764 = v0;
        let a3765 = 43u64;
        nat_eq(a3764, a3765)
    }
}

pub fn f127(v0: u64) -> Adt17 {
    let a3766 = v0;
    let a3767 = 0u64;
    if nat_eq(a3766, a3767) {
        Adt17::C1(Adt18::C0(0u64, 1u64, 4u64))
    } else {
        let a3768 = v0;
        let a3769 = 1u64;
        if nat_eq(a3768, a3769) {
            Adt17::C1(Adt18::C0(1u64, 1u64, 2u64))
        } else {
            let a3770 = v0;
            let a3771 = 2u64;
            if nat_eq(a3770, a3771) {
                Adt17::C1(Adt18::C0(2u64, 32u64, 18u64))
            } else {
                let a3772 = v0;
                let a3773 = 3u64;
                if nat_eq(a3772, a3773) {
                    Adt17::C1(Adt18::C0(3u64, 32u64, 20u64))
                } else {
                    let a3774 = v0;
                    let a3775 = 6u64;
                    if nat_eq(a3774, a3775) {
                        Adt17::C1(Adt18::C0(6u64, 32u64, 22u64))
                    } else {
                        let a3776 = v0;
                        let a3777 = 7u64;
                        if nat_eq(a3776, a3777) {
                            Adt17::C1(Adt18::C0(7u64, 32u64, 24u64))
                        } else {
                            let a3778 = v0;
                            let a3779 = 8u64;
                            if nat_eq(a3778, a3779) {
                                Adt17::C1(Adt18::C0(8u64, 32u64, 34u64))
                            } else {
                                let a3780 = v0;
                                let a3781 = 9u64;
                                if nat_eq(a3780, a3781) {
                                    Adt17::C1(Adt18::C0(9u64, 32u64, 36u64))
                                } else {
                                    let a3782 = v0;
                                    let a3783 = 10u64;
                                    if nat_eq(a3782, a3783) {
                                        Adt17::C1(Adt18::C0(10u64, 256u64, 84u64))
                                    } else {
                                        let a3784 = v0;
                                        let a3785 = 11u64;
                                        if nat_eq(a3784, a3785) {
                                            Adt17::C1(Adt18::C0(11u64, 256u64, 110u64))
                                        } else {
                                            let a3786 = v0;
                                            let a3787 = 12u64;
                                            if nat_eq(a3786, a3787) {
                                                Adt17::C1(Adt18::C0(12u64, 256u64, 144u64))
                                            } else {
                                                let a3788 = v0;
                                                let a3789 = 13u64;
                                                if nat_eq(a3788, a3789) {
                                                    Adt17::C1(Adt18::C0(13u64, 256u64, 176u64))
                                                } else {
                                                    let a3790 = v0;
                                                    let a3791 = 14u64;
                                                    if nat_eq(a3790, a3791) {
                                                        Adt17::C1(Adt18::C0(14u64, 256u64, 210u64))
                                                    } else {
                                                        let a3792 = v0;
                                                        let a3793 = 15u64;
                                                        if nat_eq(a3792, a3793) {
                                                            Adt17::C1(Adt18::C0(15u64, 256u64, 292u64))
                                                        } else {
                                                            let a3794 = v0;
                                                            let a3795 = 16u64;
                                                            if nat_eq(a3794, a3795) {
                                                                Adt17::C1(Adt18::C0(16u64, 256u64, 66u64))
                                                            } else {
                                                                let a3796 = v0;
                                                                let a3797 = 17u64;
                                                                if nat_eq(a3796, a3797) {
                                                                    Adt17::C1(Adt18::C0(17u64, 256u64, 74u64))
                                                                } else {
                                                                    let a3798 = v0;
                                                                    let a3799 = 18u64;
                                                                    if nat_eq(a3798, a3799) {
                                                                        Adt17::C1(Adt18::C0(18u64, 256u64, 98u64))
                                                                    } else {
                                                                        let a3800 = v0;
                                                                        let a3801 = 19u64;
                                                                        if nat_eq(a3800, a3801) {
                                                                            Adt17::C1(Adt18::C0(19u64, 256u64, 50u64))
                                                                        } else {
                                                                            let a3802 = v0;
                                                                            let a3803 = 20u64;
                                                                            if nat_eq(a3802, a3803) {
                                                                                Adt17::C1(Adt18::C0(20u64, 32u64, 18u64))
                                                                            } else {
                                                                                let a3804 = v0;
                                                                                let a3805 = 21u64;
                                                                                if nat_eq(a3804, a3805) {
                                                                                    Adt17::C1(Adt18::C0(21u64, 256u64, 110u64))
                                                                                } else {
                                                                                    let a3806 = v0;
                                                                                    let a3807 = 22u64;
                                                                                    if nat_eq(a3806, a3807) {
                                                                                        Adt17::C1(Adt18::C0(22u64, 256u64, 82u64))
                                                                                    } else {
                                                                                        let a3808 = v0;
                                                                                        let a3809 = 23u64;
                                                                                        if nat_eq(a3808, a3809) {
                                                                                            Adt17::C1(Adt18::C0(23u64, 256u64, 136u64))
                                                                                        } else {
                                                                                            let a3810 = v0;
                                                                                            let a3811 = 24u64;
                                                                                            if nat_eq(a3810, a3811) {
                                                                                                Adt17::C1(Adt18::C0(24u64, 1u64, 1u64))
                                                                                            } else {
                                                                                                let a3812 = v0;
                                                                                                let a3813 = 25u64;
                                                                                                if nat_eq(a3812, a3813) {
                                                                                                    Adt17::C1(Adt18::C0(25u64, 1u64, 2u64))
                                                                                                } else {
                                                                                                    let a3814 = v0;
                                                                                                    let a3815 = 26u64;
                                                                                                    if nat_eq(a3814, a3815) {
                                                                                                        Adt17::C1(Adt18::C0(26u64, 1u64, 4u64))
                                                                                                    } else {
                                                                                                        let a3816 = v0;
                                                                                                        let a3817 = 27u64;
                                                                                                        if nat_eq(a3816, a3817) {
                                                                                                            Adt17::C1(Adt18::C0(27u64, 1u64, 8u64))
                                                                                                        } else {
                                                                                                            let a3818 = v0;
                                                                                                            let a3819 = 28u64;
                                                                                                            if nat_eq(a3818, a3819) {
                                                                                                                Adt17::C1(Adt18::C0(28u64, 1u64, 8u64))
                                                                                                            } else {
                                                                                                                let a3820 = v0;
                                                                                                                let a3821 = 29u64;
                                                                                                                if nat_eq(a3820, a3821) {
                                                                                                                    Adt17::C1(Adt18::C0(29u64, 256u64, 56u64))
                                                                                                                } else {
                                                                                                                    let a3822 = v0;
                                                                                                                    let a3823 = 30u64;
                                                                                                                    if nat_eq(a3822, a3823) {
                                                                                                                        Adt17::C1(Adt18::C0(30u64, 1u64, 2u64))
                                                                                                                    } else {
                                                                                                                        let a3824 = v0;
                                                                                                                        let a3825 = 34u64;
                                                                                                                        if nat_eq(a3824, a3825) {
                                                                                                                            Adt17::C1(Adt18::C0(34u64, 256u64, 54u64))
                                                                                                                        } else {
                                                                                                                            let a3826 = v0;
                                                                                                                            let a3827 = 35u64;
                                                                                                                            if nat_eq(a3826, a3827) {
                                                                                                                                Adt17::C1(Adt18::C0(35u64, 256u64, 66u64))
                                                                                                                            } else {
                                                                                                                                let a3828 = v0;
                                                                                                                                let a3829 = 39u64;
                                                                                                                                if nat_eq(a3828, a3829) {
                                                                                                                                    Adt17::C1(Adt18::C0(39u64, 32u64, 17u64))
                                                                                                                                } else {
                                                                                                                                    let a3830 = v0;
                                                                                                                                    let a3831 = 40u64;
                                                                                                                                    if nat_eq(a3830, a3831) {
                                                                                                                                        Adt17::C1(Adt18::C0(40u64, 64u64, 36u64))
                                                                                                                                    } else {
                                                                                                                                        let a3832 = v0;
                                                                                                                                        let a3833 = 41u64;
                                                                                                                                        if nat_eq(a3832, a3833) {
                                                                                                                                            Adt17::C1(Adt18::C0(41u64, 128u64, 18u64))
                                                                                                                                        } else {
                                                                                                                                            let a3834 = v0;
                                                                                                                                            let a3835 = 42u64;
                                                                                                                                            if nat_eq(a3834, a3835) {
                                                                                                                                                Adt17::C1(Adt18::C0(42u64, 64u64, 18u64))
                                                                                                                                            } else {
                                                                                                                                                let a3836 = v0;
                                                                                                                                                let a3837 = 43u64;
                                                                                                                                                if nat_eq(a3836, a3837) {
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

pub fn f128(v0: u64, v1: Adt2, v2: bool, v3: Adt2) -> R<Adt10> {
    let a3840 = match v1.clone() {
        Adt2::C0(h3838, _, _, _, _, _, _, _, _, _, _) => {
            h3838
        }
    };
    let a3841 = match v3.clone() {
        Adt2::C0(h3839, _, _, _, _, _, _, _, _, _, _) => {
            h3839
        }
    };
    if if nat_eq(a3840, a3841) {
        let a3842 = v0;
        let a3843 = 33u64;
        if nat_eq(a3842, a3843) {
            v2
        } else {
            true
        }
    } else {
        false
    } {
        let m3845 = f127(match v1.clone() {
            Adt2::C0(h3844, _, _, _, _, _, _, _, _, _, _) => {
                h3844
            }
        });
        match m3845 {
            Adt17::C0 => {
                Ok(Adt10::C0)
            }
            Adt17::C1(v4) => {
                let v5: u64 = match v4.clone() {
                    Adt18::C0(_, h3846, _) => {
                        h3846
                    }
                };
                let v6: u64 = f16(v1.clone(), v5)?;
                let a3851 = {
                    let a3848 = match v1.clone() {
                        Adt2::C0(_, h3847, _, _, _, _, _, _, _, _, _) => {
                            h3847
                        }
                    };
                    let a3849 = v5;
                    let a3850 = 1u64;
                    nat_rem(a3848, a3849, a3850)
                };
                let a3852 = 0u64;
                if if if nat_eq(a3851, a3852) {
                    let a3857 = {
                        let a3854 = match v3.clone() {
                            Adt2::C0(_, h3853, _, _, _, _, _, _, _, _, _) => {
                                h3853
                            }
                        };
                        let a3855 = v5;
                        let a3856 = 1u64;
                        nat_rem(a3854, a3855, a3856)
                    };
                    let a3858 = 0u64;
                    nat_eq(a3857, a3858)
                } else {
                    false
                } {
                    let a3859 = v6;
                    let a3860 = f16(v3.clone(), v5)?;
                    nat_eq(a3859, a3860)
                } else {
                    false
                } {
                    Ok(Adt10::C4(v6, match v4 {
                        Adt18::C0(_, _, h3861) => {
                            h3861
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

pub fn f129(v0: u64, v1: u64) -> u64 {
    let a3862 = v0;
    let a3863 = 0u64;
    if if nat_eq(a3862, a3863) {
        true
    } else {
        let a3864 = v1;
        let a3865 = 0u64;
        nat_eq(a3864, a3865)
    } {
        1u64
    } else {
        2u64
    }
}

pub fn f130(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64, v7: u64) -> bool {
    if f34(v4, v5, v6, v7) {
        f34(v0, v1, v2, v3)
    } else {
        let a3869 = {
            let a3866 = v0;
            let a3867 = v4;
            let a3868 = 1u64;
            nat_rem(a3866, a3867, a3868)
        };
        let a3870 = 0u64;
        if if if nat_eq(a3869, a3870) {
            let a3874 = {
                let a3871 = v1;
                let a3872 = v5;
                let a3873 = 1u64;
                nat_rem(a3871, a3872, a3873)
            };
            let a3875 = 0u64;
            nat_eq(a3874, a3875)
        } else {
            false
        } {
            let a3879 = {
                let a3876 = v2;
                let a3877 = v6;
                let a3878 = 1u64;
                nat_rem(a3876, a3877, a3878)
            };
            let a3880 = 0u64;
            nat_eq(a3879, a3880)
        } else {
            false
        } {
            let a3884 = {
                let a3881 = v3;
                let a3882 = v7;
                let a3883 = 1u64;
                nat_rem(a3881, a3882, a3883)
            };
            let a3885 = 0u64;
            nat_eq(a3884, a3885)
        } else {
            false
        }
    }
}

pub fn f131(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64) -> R<bool> {
    let a3886 = v0;
    let a3887 = v1;
    if nat_eq(a3886, a3887) {
        let a3890 = v4;
        let a3891 = {
            let a3888 = v2;
            let a3889 = v3;
            nat_add(a3888, a3889)?
        };
        Ok(nat_eq(a3890, a3891))
    } else {
        let a3892 = v2;
        let a3893 = v3;
        if nat_eq(a3892, a3893) {
            let a3894 = v4;
            let a3895 = v2;
            Ok(nat_eq(a3894, a3895))
        } else {
            Ok(false)
        }
    }
}

pub fn f132(v0: u64, v1: u64, v2: u64, v3: u64) -> R<bool> {
    let v4: u64 = {
        let a3898 = {
            let a3896 = v0;
            let a3897 = f146(v1);
            nat_add(a3896, a3897)?
        };
        let a3899 = f146(v2);
        nat_add(a3898, a3899)?
    };
    let v5: u64 = {
        let a3900 = f145(v1);
        let a3901 = f145(v2);
        nat_add(a3900, a3901)?
    };
    let a3902 = v1;
    let a3903 = 4294967295u64;
    if if nat_le(a3902, a3903) {
        let a3904 = v2;
        let a3905 = 4294967295u64;
        nat_le(a3904, a3905)
    } else {
        false
    } {
        let a3906 = v5;
        let a3907 = v4;
        if nat_le(a3906, a3907) {
            let a3910 = v3;
            let a3911 = {
                let a3908 = v4;
                let a3909 = v5;
                nat_sub(a3908, a3909)
            };
            Ok(nat_eq(a3910, a3911))
        } else {
            Ok(false)
        }
    } else {
        Ok(false)
    }
}

pub fn f133(v0: u64, v1: u64) -> R<bool> {
    let a3912 = v1;
    let a3913 = 4294967295u64;
    if nat_le(a3912, a3913) {
        let a3916 = {
            let a3914 = f146(v1);
            let a3915 = f145(v1);
            nat_add(a3914, a3915)?
        };
        let a3917 = v0;
        Ok(nat_lt(a3916, a3917))
    } else {
        Ok(false)
    }
}

pub fn f134(v0: Adt3, v1: u64, v2: u64, v3: u64, v4: bool) -> R<bool> {
    let v5: u64 = f208(v0.clone(), false)?;
    let v6: u64 = f208(v0, true)?;
    let a3918 = v6;
    let a3919 = v1;
    if if nat_le(a3918, a3919) {
        let a3926 = {
            let a3924 = {
                let a3920 = v1;
                let a3921 = v5;
                nat_add(a3920, a3921)?
            };
            let a3925 = {
                let a3922 = v3;
                let a3923 = 0u64;
                if nat_eq(a3922, a3923) {
                    0u64
                } else {
                    4u64
                }
            };
            nat_add(a3924, a3925)?
        };
        let a3927 = v2;
        nat_le(a3926, a3927)
    } else {
        false
    } {
        if v4 {
            let a3932 = {
                let a3930 = {
                    let a3928 = v1;
                    let a3929 = v5;
                    nat_add(a3928, a3929)?
                };
                let a3931 = v6;
                nat_sub(a3930, a3931)
            };
            let a3933 = v2;
            Ok(nat_lt(a3932, a3933))
        } else {
            let a3938 = {
                let a3936 = {
                    let a3934 = v1;
                    let a3935 = v5;
                    nat_add(a3934, a3935)?
                };
                let a3937 = v6;
                nat_sub(a3936, a3937)
            };
            let a3939 = v2;
            Ok(nat_le(a3938, a3939))
        }
    } else {
        Ok(false)
    }
}

pub fn f135(v0: u64, v1: u64) -> bool {
    let a3940 = v0;
    let a3941 = 0u64;
    if if nat_eq(a3940, a3941) {
        true
    } else {
        let a3942 = v0;
        let a3943 = 1u64;
        nat_eq(a3942, a3943)
    } {
        let a3944 = v1;
        let a3945 = 4294967296u64;
        nat_lt(a3944, a3945)
    } else {
        false
    }
}

pub fn f136(v0: u64, v1: u64) -> R<u64> {
    let a3950 = {
        let a3948 = v0;
        let a3949 = {
            let a3946 = v1;
            let a3947 = 1u64;
            nat_sub(a3946, a3947)
        };
        nat_add(a3948, a3949)?
    };
    let a3951 = v1;
    let a3952 = 0u64;
    Ok(nat_quot(a3950, a3951, a3952))
}

pub fn f137(v0: Adt2, v1: Adt2, v2: u64, v3: u64) -> R<bool> {
    if f34(match v0.clone() {
        Adt2::C0(_, h3953, _, _, _, _, _, _, _, _, _) => {
            h3953
        }
    }, match v0.clone() {
        Adt2::C0(_, _, h3954, _, _, _, _, _, _, _, _) => {
            h3954
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, h3955, _, _, _, _, _, _, _) => {
            h3955
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, h3956, _, _, _, _, _, _) => {
            h3956
        }
    }) {
        Ok(true)
    } else {
        let v4: u64 = {
            let a3959 = match v1.clone() {
                Adt2::C0(_, h3957, _, _, _, _, _, _, _, _, _) => {
                    h3957
                }
            };
            let a3960 = match v1.clone() {
                Adt2::C0(_, _, h3958, _, _, _, _, _, _, _, _) => {
                    h3958
                }
            };
            nat_mul(a3959, a3960)?
        };
        let v5: u64 = {
            let a3962 = v4;
            let a3963 = match v1.clone() {
                Adt2::C0(_, _, _, h3961, _, _, _, _, _, _, _) => {
                    h3961
                }
            };
            nat_mul(a3962, a3963)?
        };
        let a3976 = {
            let a3974 = {
                let a3965 = match v0.clone() {
                    Adt2::C0(_, h3964, _, _, _, _, _, _, _, _, _) => {
                        h3964
                    }
                };
                let a3966 = 1u64;
                nat_sub(a3965, a3966)
            };
            let a3975 = {
                let a3972 = f209(match v0.clone() {
                    Adt2::C0(_, _, h3967, _, _, _, _, _, _, _, _) => {
                        h3967
                    }
                }, v2, v5, match v1.clone() {
                    Adt2::C0(_, h3968, _, _, _, _, _, _, _, _, _) => {
                        h3968
                    }
                })?;
                let a3973 = f209(match v0 {
                    Adt2::C0(_, _, _, h3969, _, _, _, _, _, _, _) => {
                        h3969
                    }
                }, v2, {
                    let a3970 = v3;
                    let a3971 = v5;
                    nat_mul(a3970, a3971)?
                }, v4)?;
                nat_add(a3972, a3973)?
            };
            nat_add(a3974, a3975)?
        };
        let a3977 = f16(v1, 1u64)?;
        Ok(nat_lt(a3976, a3977))
    }
}

pub fn f138(v0: u64, v1: u64, v2: u64) -> R<bool> {
    let a3978 = v0;
    let a3979 = 2147483648u64;
    if if if if nat_lt(a3978, a3979) {
        let a3980 = v1;
        let a3981 = 2147483648u64;
        nat_lt(a3980, a3981)
    } else {
        false
    } {
        let a3984 = {
            let a3982 = v0;
            let a3983 = v1;
            nat_mul(a3982, a3983)?
        };
        let a3985 = 2147483648u64;
        nat_lt(a3984, a3985)
    } else {
        false
    } {
        let a3990 = {
            let a3988 = {
                let a3986 = v0;
                let a3987 = 1u64;
                nat_sub(a3986, a3987)
            };
            let a3989 = v2;
            nat_mul(a3988, a3989)?
        };
        let a3991 = 2147483648u64;
        nat_lt(a3990, a3991)
    } else {
        false
    } {
        let a3996 = {
            let a3994 = {
                let a3992 = v1;
                let a3993 = 1u64;
                nat_sub(a3992, a3993)
            };
            let a3995 = v2;
            nat_mul(a3994, a3995)?
        };
        let a3997 = 2147483648u64;
        Ok(nat_lt(a3996, a3997))
    } else {
        Ok(false)
    }
}

pub fn f139(v0: u64) -> R<u64> {
    let a4000 = 3u64;
    let a4001 = {
        let a3998 = v0;
        let a3999 = 3u64;
        nat_mul(a3998, a3999)?
    };
    nat_add(a4000, a4001)
}

pub fn f140(v0: Adt3) -> R<u64> {
    if f34(match v0.clone() {
        Adt3::C0(h4002, _, _, _, _, _, _, _) => {
            h4002
        }
    }, match v0.clone() {
        Adt3::C0(_, h4003, _, _, _, _, _, _) => {
            h4003
        }
    }, match v0.clone() {
        Adt3::C0(_, _, h4004, _, _, _, _, _) => {
            h4004
        }
    }, match v0.clone() {
        Adt3::C0(_, _, _, h4005, _, _, _, _) => {
            h4005
        }
    }) {
        Ok(0u64)
    } else {
        let a4014 = match v0.clone() {
            Adt3::C0(h4006, _, _, _, _, _, _, _) => {
                h4006
            }
        };
        let a4015 = {
            let a4012 = match v0.clone() {
                Adt3::C0(_, h4007, _, _, _, _, _, _) => {
                    h4007
                }
            };
            let a4013 = {
                let a4010 = match v0.clone() {
                    Adt3::C0(_, _, h4008, _, _, _, _, _) => {
                        h4008
                    }
                };
                let a4011 = match v0 {
                    Adt3::C0(_, _, _, h4009, _, _, _, _) => {
                        h4009
                    }
                };
                nat_mul(a4010, a4011)?
            };
            nat_mul(a4012, a4013)?
        };
        nat_mul(a4014, a4015)
    }
}

pub fn f141(v0: Adt3, v1: u64, v2: u64) -> R<u64> {
    let a4016 = v1;
    let a4017 = 13u64;
    if if nat_eq(a4016, a4017) {
        f34(match v0.clone() {
            Adt3::C0(h4018, _, _, _, _, _, _, _) => {
                h4018
            }
        }, match v0.clone() {
            Adt3::C0(_, h4019, _, _, _, _, _, _) => {
                h4019
            }
        }, match v0.clone() {
            Adt3::C0(_, _, h4020, _, _, _, _, _) => {
                h4020
            }
        }, match v0.clone() {
            Adt3::C0(_, _, _, h4021, _, _, _, _) => {
                h4021
            }
        })
    } else {
        false
    } {
        Ok(2u64)
    } else {
        let v3: u64 = {
            let a4027 = match v0.clone() {
                Adt3::C0(_, h4022, _, _, _, _, _, _) => {
                    h4022
                }
            };
            let a4028 = {
                let a4025 = match v0.clone() {
                    Adt3::C0(_, _, h4023, _, _, _, _, _) => {
                        h4023
                    }
                };
                let a4026 = match v0.clone() {
                    Adt3::C0(_, _, _, h4024, _, _, _, _) => {
                        h4024
                    }
                };
                nat_mul(a4025, a4026)?
            };
            nat_mul(a4027, a4028)?
        };
        let a4029 = v1;
        let a4030 = 13u64;
        if nat_eq(a4029, a4030) {
            let a4039 = 2u64;
            let a4040 = if f210(v0.clone(), v2) {
                let a4032 = v3;
                let a4033 = f211(match v0 {
                    Adt3::C0(h4031, _, _, _, _, _, _, _) => {
                        h4031
                    }
                }, false)?;
                nat_mul(a4032, a4033)?
            } else {
                let a4037 = v3;
                let a4038 = {
                    let a4035 = match v0 {
                        Adt3::C0(h4034, _, _, _, _, _, _, _) => {
                            h4034
                        }
                    };
                    let a4036 = 2u64;
                    nat_mul(a4035, a4036)?
                };
                nat_mul(a4037, a4038)?
            };
            nat_add(a4039, a4040)
        } else {
            let a4044 = v3;
            let a4045 = f211(match v0 {
                Adt3::C0(h4041, _, _, _, _, _, _, _) => {
                    h4041
                }
            }, {
                let a4042 = v1;
                let a4043 = 16u64;
                nat_eq(a4042, a4043)
            })?;
            nat_mul(a4044, a4045)
        }
    }
}

pub fn f142(v0: u64, v1: u64, v2: Adt2, v3: Option<Adt2>, v4: Adt2) -> R<Adt10> {
    let a4046 = v0;
    let a4047 = 30u64;
    if if nat_eq(a4046, a4047) {
        let a4050 = match v2.clone() {
            Adt2::C0(_, _, _, _, _, _, h4048, _, _, _, _) => {
                h4048
            }
        };
        let a4051 = match v2.clone() {
            Adt2::C0(_, _, _, _, _, h4049, _, _, _, _, _) => {
                h4049
            }
        };
        nat_lt(a4050, a4051)
    } else {
        false
    } {
        Ok(Adt10::C1)
    } else {
        let m4052 = v3;
        match m4052 {
            None => {
                f212(v0, v1, match v2.clone() {
                    Adt2::C0(h4053, _, _, _, _, _, _, _, _, _, _) => {
                        h4053
                    }
                }, 0u64, match v4.clone() {
                    Adt2::C0(h4054, _, _, _, _, _, _, _, _, _, _) => {
                        h4054
                    }
                }, false, match v2.clone() {
                    Adt2::C0(_, h4055, _, _, _, _, _, _, _, _, _) => {
                        h4055
                    }
                }, match v2.clone() {
                    Adt2::C0(_, _, h4056, _, _, _, _, _, _, _, _) => {
                        h4056
                    }
                }, match v2.clone() {
                    Adt2::C0(_, _, _, h4057, _, _, _, _, _, _, _) => {
                        h4057
                    }
                }, match v2 {
                    Adt2::C0(_, _, _, _, h4058, _, _, _, _, _, _) => {
                        h4058
                    }
                }, 1u64, 1u64, 1u64, 1u64, match v4.clone() {
                    Adt2::C0(_, h4059, _, _, _, _, _, _, _, _, _) => {
                        h4059
                    }
                }, match v4.clone() {
                    Adt2::C0(_, _, h4060, _, _, _, _, _, _, _, _) => {
                        h4060
                    }
                }, match v4.clone() {
                    Adt2::C0(_, _, _, h4061, _, _, _, _, _, _, _) => {
                        h4061
                    }
                }, match v4 {
                    Adt2::C0(_, _, _, _, h4062, _, _, _, _, _, _) => {
                        h4062
                    }
                })
            }
            Some(v5) => {
                f212(v0, v1, match v2.clone() {
                    Adt2::C0(h4063, _, _, _, _, _, _, _, _, _, _) => {
                        h4063
                    }
                }, match v5.clone() {
                    Adt2::C0(h4064, _, _, _, _, _, _, _, _, _, _) => {
                        h4064
                    }
                }, match v4.clone() {
                    Adt2::C0(h4065, _, _, _, _, _, _, _, _, _, _) => {
                        h4065
                    }
                }, true, match v2.clone() {
                    Adt2::C0(_, h4066, _, _, _, _, _, _, _, _, _) => {
                        h4066
                    }
                }, match v2.clone() {
                    Adt2::C0(_, _, h4067, _, _, _, _, _, _, _, _) => {
                        h4067
                    }
                }, match v2.clone() {
                    Adt2::C0(_, _, _, h4068, _, _, _, _, _, _, _) => {
                        h4068
                    }
                }, match v2 {
                    Adt2::C0(_, _, _, _, h4069, _, _, _, _, _, _) => {
                        h4069
                    }
                }, match v5.clone() {
                    Adt2::C0(_, h4070, _, _, _, _, _, _, _, _, _) => {
                        h4070
                    }
                }, match v5.clone() {
                    Adt2::C0(_, _, h4071, _, _, _, _, _, _, _, _) => {
                        h4071
                    }
                }, match v5.clone() {
                    Adt2::C0(_, _, _, h4072, _, _, _, _, _, _, _) => {
                        h4072
                    }
                }, match v5 {
                    Adt2::C0(_, _, _, _, h4073, _, _, _, _, _, _) => {
                        h4073
                    }
                }, match v4.clone() {
                    Adt2::C0(_, h4074, _, _, _, _, _, _, _, _, _) => {
                        h4074
                    }
                }, match v4.clone() {
                    Adt2::C0(_, _, h4075, _, _, _, _, _, _, _, _) => {
                        h4075
                    }
                }, match v4.clone() {
                    Adt2::C0(_, _, _, h4076, _, _, _, _, _, _, _) => {
                        h4076
                    }
                }, match v4 {
                    Adt2::C0(_, _, _, _, h4077, _, _, _, _, _, _) => {
                        h4077
                    }
                })
            }
        }
    }
}

pub fn f143(v0: u64, v1: u64) -> u64 {
    let a4078 = v0;
    let a4079 = 23u64;
    if nat_eq(a4078, a4079) {
        let a4080 = v1;
        let a4081 = 0u64;
        if nat_eq(a4080, a4081) {
            2u64
        } else {
            3u64
        }
    } else {
        1u64
    }
}

pub fn f144(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64, v7: u64) -> R<u64> {
    let a4094 = {
        let a4090 = {
            let a4086 = {
                let a4082 = v0;
                let a4083 = v4;
                nat_mul(a4082, a4083)?
            };
            let a4087 = {
                let a4084 = v1;
                let a4085 = v5;
                nat_mul(a4084, a4085)?
            };
            nat_add(a4086, a4087)?
        };
        let a4091 = {
            let a4088 = v2;
            let a4089 = v6;
            nat_mul(a4088, a4089)?
        };
        nat_add(a4090, a4091)?
    };
    let a4095 = {
        let a4092 = v3;
        let a4093 = v7;
        nat_mul(a4092, a4093)?
    };
    nat_add(a4094, a4095)
}

pub fn f145(v0: u64) -> u64 {
    let a4096 = v0;
    let a4097 = 2147483648u64;
    if nat_lt(a4096, a4097) {
        0u64
    } else {
        let a4098 = 4294967296u64;
        let a4099 = v0;
        nat_sub(a4098, a4099)
    }
}

pub fn f146(v0: u64) -> u64 {
    let a4100 = v0;
    let a4101 = 2147483648u64;
    if nat_lt(a4100, a4101) {
        v0
    } else {
        0u64
    }
}

pub fn f147(v0: Adt11, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64, v7: u64, v8: u64) -> Adt9 {
    let v9: u64 = {
        let a4102 = v7;
        let a4103 = 6u64;
        if nat_eq(a4102, a4103) {
            27u64
        } else {
            v2
        }
    };
    Adt9::C7(v0, f98(v3, v4), f98(v5, v6), f99(v7, v8), f170(v1), f170(v9))
}

pub fn f148(v0: Adt4, v1: u64, v2: u64) -> Adt11 {
    let m4104 = v0;
    match m4104 {
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
        Adt4::C13(v39, v40, v41, v42) => {
            Adt11::C4(v39, v40, v41, v42)
        }
        Adt4::C14(v43, _, _) => {
            let a4105 = 13u64;
            let a4106 = v43;
            if nat_le(a4105, a4106) {
                Adt11::C7(v43, v1, v2)
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
        Adt4::C27(v63) => {
            Adt11::C2(match v63.clone() {
                Adt5::C0(_, _, _, _, _, h4107, _, _) => {
                    h4107
                }
            }, match v63 {
                Adt5::C0(_, _, _, _, _, _, _, h4108) => {
                    h4108
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
    }
}

pub fn f149(v0: Adt4) -> u64 {
    let m4109 = v0;
    match m4109 {
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
        Adt4::C14(v41, _, _) => {
            v41
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
    }
}

pub fn f150(v0: Adt4, v1: u64) -> u64 {
    let m4110 = v0;
    match m4110 {
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
        Adt4::C16(_, v46) => {
            v46
        }
        Adt4::C17 => {
            0u64
        }
        Adt4::C18(_, _) => {
            0u64
        }
        Adt4::C19(v49, _) => {
            let a4111 = v1;
            let a4112 = 6u64;
            if nat_eq(a4111, a4112) {
                v49
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
        Adt4::C26(v63, _, _, _) => {
            let a4121 = v1;
            let a4122 = 6u64;
            if if if nat_eq(a4121, a4122) {
                true
            } else {
                let a4123 = v1;
                let a4124 = 7u64;
                nat_eq(a4123, a4124)
            } {
                let a4125 = v63;
                let a4126 = 30u64;
                nat_eq(a4125, a4126)
            } else {
                false
            } {
                30u64
            } else {
                0u64
            }
        }
        Adt4::C27(v62) => {
            match v62 {
                Adt5::C0(_, _, _, _, _, _, _, h4120) => {
                    h4120
                }
            }
        }
        Adt4::C28(v58) => {
            let a4113 = v1;
            let a4114 = 6u64;
            if nat_eq(a4113, a4114) {
                match v58 {
                    Adt7::C0(_, _, _, _, h4115) => {
                        h4115
                    }
                }
            } else {
                0u64
            }
        }
        Adt4::C29(_) => {
            0u64
        }
        Adt4::C30(_, v61) => {
            let a4116 = v1;
            let a4117 = 6u64;
            if if nat_eq(a4116, a4117) {
                true
            } else {
                let a4118 = v1;
                let a4119 = 7u64;
                nat_eq(a4118, a4119)
            } {
                v61
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
    }
}

pub fn f151(v0: Adt4, v1: u64) -> Adt11 {
    let a4127 = v1;
    let a4128 = 4u64;
    if nat_lt(a4127, a4128) {
        Adt11::C0(v1)
    } else {
        let a4129 = v1;
        let a4130 = 4u64;
        if nat_eq(a4129, a4130) {
            Adt11::C1(0u64)
        } else {
            let a4131 = v1;
            let a4132 = 5u64;
            if nat_eq(a4131, a4132) {
                Adt11::C1(3u64)
            } else {
                let a4133 = v1;
                let a4134 = 6u64;
                if nat_eq(a4133, a4134) {
                    Adt11::C2(f150(v0, v1), 0u64, false)
                } else {
                    let a4135 = v1;
                    let a4136 = 7u64;
                    if nat_eq(a4135, a4136) {
                        Adt11::C2(0u64, f150(v0, v1), false)
                    } else {
                        let a4137 = v1;
                        let a4138 = 16u64;
                        if nat_lt(a4137, a4138) {
                            Adt11::C1({
                                let a4139 = v1;
                                let a4140 = 8u64;
                                nat_sub(a4139, a4140)
                            })
                        } else {
                            let a4141 = v1;
                            let a4142 = 20u64;
                            if nat_lt(a4141, a4142) {
                                Adt11::C5({
                                    let a4143 = v1;
                                    let a4144 = 16u64;
                                    nat_sub(a4143, a4144)
                                })
                            } else {
                                let a4145 = v1;
                                let a4146 = 20u64;
                                if nat_eq(a4145, a4146) {
                                    Adt11::C3(0u64, 0u64, false)
                                } else {
                                    let a4147 = v1;
                                    let a4148 = 21u64;
                                    if nat_eq(a4147, a4148) {
                                        Adt11::C0(4u64)
                                    } else {
                                        let a4149 = v1;
                                        let a4150 = 22u64;
                                        if nat_eq(a4149, a4150) {
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

pub fn f152(v0: u64, v1: Adt3) -> R<u64> {
    let v2: u64 = {
        let a4152 = v0;
        let a4153 = match v1.clone() {
            Adt3::C0(h4151, _, _, _, _, _, _, _) => {
                h4151
            }
        };
        let a4154 = 0u64;
        nat_quot(a4152, a4153, a4154)
    };
    let v3: u64 = {
        let a4156 = v2;
        let a4157 = match v1.clone() {
            Adt3::C0(_, h4155, _, _, _, _, _, _) => {
                h4155
            }
        };
        let a4158 = 0u64;
        nat_quot(a4156, a4157, a4158)
    };
    f144({
        let a4160 = v0;
        let a4161 = match v1.clone() {
            Adt3::C0(h4159, _, _, _, _, _, _, _) => {
                h4159
            }
        };
        let a4162 = 0u64;
        nat_rem(a4160, a4161, a4162)
    }, {
        let a4164 = v2;
        let a4165 = match v1.clone() {
            Adt3::C0(_, h4163, _, _, _, _, _, _) => {
                h4163
            }
        };
        let a4166 = 0u64;
        nat_rem(a4164, a4165, a4166)
    }, {
        let a4168 = v3;
        let a4169 = match v1.clone() {
            Adt3::C0(_, _, h4167, _, _, _, _, _) => {
                h4167
            }
        };
        let a4170 = 0u64;
        nat_rem(a4168, a4169, a4170)
    }, {
        let a4172 = v3;
        let a4173 = match v1.clone() {
            Adt3::C0(_, _, h4171, _, _, _, _, _) => {
                h4171
            }
        };
        let a4174 = 0u64;
        nat_quot(a4172, a4173, a4174)
    }, match v1.clone() {
        Adt3::C0(_, _, _, _, h4175, _, _, _) => {
            h4175
        }
    }, match v1.clone() {
        Adt3::C0(_, _, _, _, _, h4176, _, _) => {
            h4176
        }
    }, match v1.clone() {
        Adt3::C0(_, _, _, _, _, _, h4177, _) => {
            h4177
        }
    }, match v1 {
        Adt3::C0(_, _, _, _, _, _, _, h4178) => {
            h4178
        }
    })
}

pub fn f153(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let a4196 = {
        let a4191 = f213(v2, {
            let a4182 = {
                let a4179 = v0;
                let a4180 = v1;
                let a4181 = 0u64;
                nat_quot(a4179, a4180, a4181)
            };
            let a4183 = v2;
            let a4184 = 0u64;
            nat_rem(a4182, a4183, a4184)
        }, {
            let a4188 = {
                let a4185 = v0;
                let a4186 = v1;
                let a4187 = 0u64;
                nat_quot(a4185, a4186, a4187)
            };
            let a4189 = v2;
            let a4190 = 0u64;
            nat_quot(a4188, a4189, a4190)
        })?;
        let a4192 = v1;
        nat_mul(a4191, a4192)?
    };
    let a4197 = {
        let a4193 = v0;
        let a4194 = v1;
        let a4195 = 0u64;
        nat_rem(a4193, a4194, a4195)
    };
    nat_add(a4196, a4197)
}

pub fn f154(v0: u64, v1: Adt3, v2: Adt3) -> R<u64> {
    let v3: u64 = {
        let a4199 = v0;
        let a4200 = match v2.clone() {
            Adt3::C0(h4198, _, _, _, _, _, _, _) => {
                h4198
            }
        };
        let a4201 = 0u64;
        nat_quot(a4199, a4200, a4201)
    };
    let v4: u64 = {
        let a4203 = v3;
        let a4204 = match v2.clone() {
            Adt3::C0(_, h4202, _, _, _, _, _, _) => {
                h4202
            }
        };
        let a4205 = 0u64;
        nat_quot(a4203, a4204, a4205)
    };
    f214({
        let a4207 = v0;
        let a4208 = match v2.clone() {
            Adt3::C0(h4206, _, _, _, _, _, _, _) => {
                h4206
            }
        };
        let a4209 = 0u64;
        nat_rem(a4207, a4208, a4209)
    }, {
        let a4211 = v3;
        let a4212 = match v2.clone() {
            Adt3::C0(_, h4210, _, _, _, _, _, _) => {
                h4210
            }
        };
        let a4213 = 0u64;
        nat_rem(a4211, a4212, a4213)
    }, {
        let a4215 = v4;
        let a4216 = match v2.clone() {
            Adt3::C0(_, _, h4214, _, _, _, _, _) => {
                h4214
            }
        };
        let a4217 = 0u64;
        nat_rem(a4215, a4216, a4217)
    }, {
        let a4219 = v4;
        let a4220 = match v2 {
            Adt3::C0(_, _, h4218, _, _, _, _, _) => {
                h4218
            }
        };
        let a4221 = 0u64;
        nat_quot(a4219, a4220, a4221)
    }, match v1.clone() {
        Adt3::C0(h4222, _, _, _, _, _, _, _) => {
            h4222
        }
    }, match v1.clone() {
        Adt3::C0(_, h4223, _, _, _, _, _, _) => {
            h4223
        }
    }, match v1.clone() {
        Adt3::C0(_, _, h4224, _, _, _, _, _) => {
            h4224
        }
    }, match v1.clone() {
        Adt3::C0(_, _, _, h4225, _, _, _, _) => {
            h4225
        }
    }, match v1.clone() {
        Adt3::C0(_, _, _, _, h4226, _, _, _) => {
            h4226
        }
    }, match v1.clone() {
        Adt3::C0(_, _, _, _, _, h4227, _, _) => {
            h4227
        }
    }, match v1.clone() {
        Adt3::C0(_, _, _, _, _, _, h4228, _) => {
            h4228
        }
    }, match v1 {
        Adt3::C0(_, _, _, _, _, _, _, h4229) => {
            h4229
        }
    })
}

pub fn f155(v0: u64, v1: Adt3, v2: Adt3, v3: Adt3, v4: u64) -> R<Adt9> {
    let v5: u64 = {
        let a4231 = v0;
        let a4232 = match v3.clone() {
            Adt3::C0(h4230, _, _, _, _, _, _, _) => {
                h4230
            }
        };
        let a4233 = 0u64;
        nat_quot(a4231, a4232, a4233)
    };
    let v6: u64 = {
        let a4235 = v5;
        let a4236 = match v3.clone() {
            Adt3::C0(_, h4234, _, _, _, _, _, _) => {
                h4234
            }
        };
        let a4237 = 0u64;
        nat_quot(a4235, a4236, a4237)
    };
    let v7: u64 = {
        let a4239 = v0;
        let a4240 = match v3.clone() {
            Adt3::C0(h4238, _, _, _, _, _, _, _) => {
                h4238
            }
        };
        let a4241 = 0u64;
        nat_rem(a4239, a4240, a4241)
    };
    let v8: u64 = {
        let a4243 = v5;
        let a4244 = match v3.clone() {
            Adt3::C0(_, h4242, _, _, _, _, _, _) => {
                h4242
            }
        };
        let a4245 = 0u64;
        nat_rem(a4243, a4244, a4245)
    };
    let v9: u64 = {
        let a4247 = v6;
        let a4248 = match v3.clone() {
            Adt3::C0(_, _, h4246, _, _, _, _, _) => {
                h4246
            }
        };
        let a4249 = 0u64;
        nat_rem(a4247, a4248, a4249)
    };
    let v10: u64 = {
        let a4251 = v6;
        let a4252 = match v3.clone() {
            Adt3::C0(_, _, h4250, _, _, _, _, _) => {
                h4250
            }
        };
        let a4253 = 0u64;
        nat_quot(a4251, a4252, a4253)
    };
    let v11: u64 = {
        let a4254 = v4;
        let a4255 = 0u64;
        if nat_eq(a4254, a4255) {
            match v1.clone() {
                Adt3::C0(h4256, _, _, _, _, _, _, _) => {
                    h4256
                }
            }
        } else {
            let a4257 = v4;
            let a4258 = 1u64;
            if nat_eq(a4257, a4258) {
                match v1.clone() {
                    Adt3::C0(_, h4259, _, _, _, _, _, _) => {
                        h4259
                    }
                }
            } else {
                let a4260 = v4;
                let a4261 = 2u64;
                if nat_eq(a4260, a4261) {
                    match v1.clone() {
                        Adt3::C0(_, _, h4262, _, _, _, _, _) => {
                            h4262
                        }
                    }
                } else {
                    match v1.clone() {
                        Adt3::C0(_, _, _, h4263, _, _, _, _) => {
                            h4263
                        }
                    }
                }
            }
        }
    };
    let v12: bool = {
        let a4270 = v11;
        let a4271 = {
            let a4264 = v4;
            let a4265 = 0u64;
            if nat_eq(a4264, a4265) {
                v7
            } else {
                let a4266 = v4;
                let a4267 = 1u64;
                if nat_eq(a4266, a4267) {
                    v8
                } else {
                    let a4268 = v4;
                    let a4269 = 2u64;
                    if nat_eq(a4268, a4269) {
                        v9
                    } else {
                        v10
                    }
                }
            }
        };
        nat_le(a4270, a4271)
    };
    let v13: Adt3 = if v12 {
        v2
    } else {
        v1
    };
    Ok(Adt9::C1(v12, f144(if if v12 {
        let a4272 = v4;
        let a4273 = 0u64;
        nat_eq(a4272, a4273)
    } else {
        false
    } {
        let a4274 = v7;
        let a4275 = v11;
        nat_sub(a4274, a4275)
    } else {
        v7
    }, if if v12 {
        let a4276 = v4;
        let a4277 = 1u64;
        nat_eq(a4276, a4277)
    } else {
        false
    } {
        let a4278 = v8;
        let a4279 = v11;
        nat_sub(a4278, a4279)
    } else {
        v8
    }, if if v12 {
        let a4280 = v4;
        let a4281 = 2u64;
        nat_eq(a4280, a4281)
    } else {
        false
    } {
        let a4282 = v9;
        let a4283 = v11;
        nat_sub(a4282, a4283)
    } else {
        v9
    }, if if v12 {
        let a4284 = v4;
        let a4285 = 3u64;
        nat_eq(a4284, a4285)
    } else {
        false
    } {
        let a4286 = v10;
        let a4287 = v11;
        nat_sub(a4286, a4287)
    } else {
        v10
    }, match v13.clone() {
        Adt3::C0(_, _, _, _, h4288, _, _, _) => {
            h4288
        }
    }, match v13.clone() {
        Adt3::C0(_, _, _, _, _, h4289, _, _) => {
            h4289
        }
    }, match v13.clone() {
        Adt3::C0(_, _, _, _, _, _, h4290, _) => {
            h4290
        }
    }, match v13 {
        Adt3::C0(_, _, _, _, _, _, _, h4291) => {
            h4291
        }
    })?, f152(v0, v3)?))
}

pub fn f156(v0: u64, v1: Adt3, v2: Adt3, v3: u64, v4: u64, v5: u64, v6: u64, v7: bool) -> R<Adt9> {
    let v8: u64 = f74(v0, v2.clone(), 0u64);
    let v9: u64 = f74(v0, v2.clone(), 1u64);
    let v10: u64 = f74(v0, v2.clone(), 2u64);
    let v11: u64 = f74(v0, v2.clone(), 3u64);
    if v7 {
        Ok(Adt9::C1(false, f73(v1.clone(), f215(v8, v3, match v1.clone() {
            Adt3::C0(h4292, _, _, _, _, _, _, _) => {
                h4292
            }
        })?, f215(v9, v4, match v1.clone() {
            Adt3::C0(_, h4293, _, _, _, _, _, _) => {
                h4293
            }
        })?, f215(v10, v5, match v1.clone() {
            Adt3::C0(_, _, h4294, _, _, _, _, _) => {
                h4294
            }
        })?, f215(v11, v6, match v1 {
            Adt3::C0(_, _, _, h4295, _, _, _, _) => {
                h4295
            }
        })?)?, f152(v0, v2)?))
    } else {
        if if if if f216(v8, v3, match v1.clone() {
            Adt3::C0(h4296, _, _, _, _, _, _, _) => {
                h4296
            }
        })? {
            f216(v9, v4, match v1.clone() {
                Adt3::C0(_, h4297, _, _, _, _, _, _) => {
                    h4297
                }
            })?
        } else {
            false
        } {
            f216(v10, v5, match v1.clone() {
                Adt3::C0(_, _, h4298, _, _, _, _, _) => {
                    h4298
                }
            })?
        } else {
            false
        } {
            f216(v11, v6, match v1.clone() {
                Adt3::C0(_, _, _, h4299, _, _, _, _) => {
                    h4299
                }
            })?
        } else {
            false
        } {
            Ok(Adt9::C1(false, f73(v1, f217(v8, v3)?, f217(v9, v4)?, f217(v10, v5)?, f217(v11, v6)?)?, f152(v0, v2)?))
        } else {
            Ok(Adt9::C1(true, 0u64, f152(v0, v2)?))
        }
    }
}

pub fn f157(v0: u64, v1: Adt3, v2: Adt3, v3: u64) -> R<Adt9> {
    let v4: u64 = f74(v0, v2.clone(), 0u64);
    let v5: u64 = f74(v0, v2.clone(), 1u64);
    let v6: u64 = f74(v0, v2.clone(), 2u64);
    let v7: u64 = f74(v0, v2.clone(), 3u64);
    Ok(Adt9::C1(false, f73(v1.clone(), f218(v4, v3, match v1 {
        Adt3::C0(h4300, _, _, _, _, _, _, _) => {
            h4300
        }
    })?, v5, v6, v7)?, f152(v0, v2)?))
}

pub fn f158(v0: u64, v1: Adt3, v2: Adt3, v3: u64, v4: u64, v5: bool) -> R<Adt9> {
    let a4301 = v0;
    let a4302 = v3;
    if nat_lt(a4301, a4302) {
        Ok(Adt9::C1(false, {
            let a4303 = v0;
            let a4304 = 4u64;
            nat_mul(a4303, a4304)?
        }, {
            let a4305 = v0;
            let a4306 = 4u64;
            nat_mul(a4305, a4306)?
        }))
    } else {
        let v6: u64 = {
            let a4307 = v0;
            let a4308 = v3;
            nat_sub(a4307, a4308)
        };
        let v7: u64 = f152(v6, v1)?;
        let v8: u64 = f219(v6, v2, v4)?;
        if v5 {
            Ok(Adt9::C4(8u64, 0u64, v8, 4u64, v7, 3u64, v8))
        } else {
            Ok(Adt9::C1(true, v7, v8))
        }
    }
}

pub fn f159(v0: u64, v1: Adt3, v2: Adt3, v3: u64) -> R<Adt9> {
    let a4309 = v0;
    let a4310 = v3;
    if nat_lt(a4309, a4310) {
        Ok(Adt9::C1(true, 0u64, f152(v0, v2)?))
    } else {
        let v4: u64 = {
            let a4311 = v0;
            let a4312 = v3;
            nat_sub(a4311, a4312)
        };
        let v5: u64 = f154(v4, v2, v1.clone())?;
        Ok(Adt9::C4(8u64, 3u64, v5, 0u64, f152(v4, v1)?, 3u64, v5))
    }
}

pub fn f160(v0: u64, v1: u64, v2: Adt3, v3: Adt3) -> R<Adt9> {
    let v4: u64 = {
        let a4314 = v1;
        let a4315 = match v3.clone() {
            Adt3::C0(h4313, _, _, _, _, _, _, _) => {
                h4313
            }
        };
        let a4316 = 0u64;
        nat_rem(a4314, a4315, a4316)
    };
    let v5: u64 = {
        let a4322 = {
            let a4318 = v1;
            let a4319 = match v3.clone() {
                Adt3::C0(h4317, _, _, _, _, _, _, _) => {
                    h4317
                }
            };
            let a4320 = 0u64;
            nat_quot(a4318, a4319, a4320)
        };
        let a4323 = match v3.clone() {
            Adt3::C0(_, h4321, _, _, _, _, _, _) => {
                h4321
            }
        };
        let a4324 = 0u64;
        nat_rem(a4322, a4323, a4324)
    };
    let v6: bool = f220(v0, v4, v5);
    Ok(Adt9::C1(!v6, if v6 {
        f154(v1, v2, v3.clone())?
    } else {
        0u64
    }, f152(v1, v3)?))
}

pub fn f161(v0: u64, v1: Adt3, v2: Adt3, v3: u64, v4: bool, v5: Adt3) -> R<Adt9> {
    Ok(Adt9::C4({
        let a4325 = v3;
        let a4326 = 8u64;
        nat_add(a4325, a4326)?
    }, 0u64, f152(v0, v1)?, if v4 {
        5u64
    } else {
        4u64
    }, if v4 {
        0u64
    } else {
        f154(v0, v5, v2.clone())?
    }, 3u64, f152(v0, v2)?))
}

pub fn f162(v0: u64, v1: Adt3, v2: Adt3, v3: u64, v4: u64) -> R<Adt9> {
    let a4327 = v4;
    let a4328 = 0u64;
    if if nat_eq(a4327, a4328) {
        f54(v3)
    } else {
        false
    } {
        let m4329 = f127(v3);
        match m4329 {
            Adt17::C0 => {
                Ok(Adt9::C0(Adt10::C1))
            }
            Adt17::C1(v5) => {
                let v6: u64 = match v5.clone() {
                    Adt18::C0(_, h4330, _) => {
                        h4330
                    }
                };
                Ok(Adt9::C5(v3, f152(v0, v1)?, match v5 {
                    Adt18::C0(_, _, h4331) => {
                        h4331
                    }
                }, f152({
                    let a4332 = v0;
                    let a4333 = v6;
                    nat_mul(a4332, a4333)?
                }, v2.clone())?, {
                    let a4339 = {
                        let a4337 = {
                            let a4334 = v6;
                            let a4335 = 1u64;
                            nat_sub(a4334, a4335)
                        };
                        let a4338 = match v2.clone() {
                            Adt3::C0(_, _, _, _, h4336, _, _, _) => {
                                h4336
                            }
                        };
                        nat_mul(a4337, a4338)?
                    };
                    let a4340 = 4u64;
                    nat_add(a4339, a4340)?
                }, match v2 {
                    Adt3::C0(_, _, _, _, h4341, _, _, _) => {
                        h4341
                    }
                }))
            }
        }
    } else {
        Ok(Adt9::C3(v3, v4, f152(v0, v1)?, f152(v0, v2)?))
    }
}

pub fn f163(v0: u64, v1: Adt3, v2: Adt3, v3: bool, v4: u64, v5: u64) -> R<Adt9> {
    let v6: u64 = {
        let a4343 = v0;
        let a4344 = match v2.clone() {
            Adt3::C0(h4342, _, _, _, _, _, _, _) => {
                h4342
            }
        };
        let a4345 = 0u64;
        nat_rem(a4343, a4344, a4345)
    };
    let v7: u64 = {
        let a4347 = v0;
        let a4348 = match v2.clone() {
            Adt3::C0(h4346, _, _, _, _, _, _, _) => {
                h4346
            }
        };
        let a4349 = 0u64;
        nat_quot(a4347, a4348, a4349)
    };
    let v8: u64 = {
        let a4351 = v7;
        let a4352 = match v2.clone() {
            Adt3::C0(_, h4350, _, _, _, _, _, _) => {
                h4350
            }
        };
        let a4353 = 0u64;
        nat_rem(a4351, a4352, a4353)
    };
    let v9: u64 = {
        let a4355 = v7;
        let a4356 = match v2.clone() {
            Adt3::C0(_, h4354, _, _, _, _, _, _) => {
                h4354
            }
        };
        let a4357 = 0u64;
        nat_quot(a4355, a4356, a4357)
    };
    let v10: u64 = {
        let a4359 = v9;
        let a4360 = match v2.clone() {
            Adt3::C0(_, _, h4358, _, _, _, _, _) => {
                h4358
            }
        };
        let a4361 = 0u64;
        nat_rem(a4359, a4360, a4361)
    };
    let v11: u64 = {
        let a4363 = v9;
        let a4364 = match v2.clone() {
            Adt3::C0(_, _, h4362, _, _, _, _, _) => {
                h4362
            }
        };
        let a4365 = 0u64;
        nat_quot(a4363, a4364, a4365)
    };
    let v12: u64 = f152(v0, v2)?;
    if v3 {
        let v13: u64 = {
            let a4374 = {
                let a4369 = {
                    let a4366 = v10;
                    let a4367 = v4;
                    let a4368 = 0u64;
                    nat_quot(a4366, a4367, a4368)
                };
                let a4370 = v5;
                nat_mul(a4369, a4370)?
            };
            let a4375 = {
                let a4371 = v8;
                let a4372 = v4;
                let a4373 = 0u64;
                nat_quot(a4371, a4372, a4373)
            };
            nat_add(a4374, a4375)?
        };
        let v14: u64 = {
            let a4395 = v6;
            let a4396 = {
                let a4393 = match v1.clone() {
                    Adt3::C0(h4376, _, _, _, _, _, _, _) => {
                        h4376
                    }
                };
                let a4394 = {
                    let a4391 = {
                        let a4377 = v8;
                        let a4378 = v4;
                        let a4379 = 0u64;
                        nat_rem(a4377, a4378, a4379)
                    };
                    let a4392 = {
                        let a4389 = match v1.clone() {
                            Adt3::C0(_, h4380, _, _, _, _, _, _) => {
                                h4380
                            }
                        };
                        let a4390 = {
                            let a4387 = {
                                let a4381 = v10;
                                let a4382 = v4;
                                let a4383 = 0u64;
                                nat_rem(a4381, a4382, a4383)
                            };
                            let a4388 = {
                                let a4385 = match v1.clone() {
                                    Adt3::C0(_, _, h4384, _, _, _, _, _) => {
                                        h4384
                                    }
                                };
                                let a4386 = v13;
                                nat_mul(a4385, a4386)?
                            };
                            nat_add(a4387, a4388)?
                        };
                        nat_mul(a4389, a4390)?
                    };
                    nat_add(a4391, a4392)?
                };
                nat_mul(a4393, a4394)?
            };
            nat_add(a4395, a4396)?
        };
        Ok(Adt9::C1(false, f152(v14, v1)?, v12))
    } else {
        let v15: u64 = {
            let a4402 = {
                let a4400 = {
                    let a4397 = v11;
                    let a4398 = v5;
                    let a4399 = 0u64;
                    nat_rem(a4397, a4398, a4399)
                };
                let a4401 = v4;
                nat_mul(a4400, a4401)?
            };
            let a4403 = v8;
            nat_add(a4402, a4403)?
        };
        let v16: u64 = {
            let a4409 = {
                let a4407 = {
                    let a4404 = v11;
                    let a4405 = v5;
                    let a4406 = 0u64;
                    nat_quot(a4404, a4405, a4406)
                };
                let a4408 = v4;
                nat_mul(a4407, a4408)?
            };
            let a4410 = v10;
            nat_add(a4409, a4410)?
        };
        let a4412 = v15;
        let a4413 = match v1.clone() {
            Adt3::C0(_, h4411, _, _, _, _, _, _) => {
                h4411
            }
        };
        if if nat_lt(a4412, a4413) {
            let a4415 = v16;
            let a4416 = match v1.clone() {
                Adt3::C0(_, _, h4414, _, _, _, _, _) => {
                    h4414
                }
            };
            nat_lt(a4415, a4416)
        } else {
            false
        } {
            Ok(Adt9::C1(false, f152({
                let a4425 = v6;
                let a4426 = {
                    let a4423 = match v1.clone() {
                        Adt3::C0(h4417, _, _, _, _, _, _, _) => {
                            h4417
                        }
                    };
                    let a4424 = {
                        let a4421 = v15;
                        let a4422 = {
                            let a4419 = match v1.clone() {
                                Adt3::C0(_, h4418, _, _, _, _, _, _) => {
                                    h4418
                                }
                            };
                            let a4420 = v16;
                            nat_mul(a4419, a4420)?
                        };
                        nat_add(a4421, a4422)?
                    };
                    nat_mul(a4423, a4424)?
                };
                nat_add(a4425, a4426)?
            }, v1)?, v12))
        } else {
            Ok(Adt9::C1(true, 0u64, v12))
        }
    }
}

pub fn f164(v0: u64, v1: Adt3, v2: Adt3, v3: u64, v4: u64) -> R<Adt9> {
    let v5: u64 = f143(23u64, v4);
    let v6: u64 = {
        let a4427 = v0;
        let a4428 = v5;
        let a4429 = 0u64;
        nat_rem(a4427, a4428, a4429)
    };
    let v7: u64 = {
        let a4430 = v0;
        let a4431 = v5;
        let a4432 = 0u64;
        nat_quot(a4430, a4431, a4432)
    };
    let v8: Adt13 = Adt13::C1(f152(v7, v2)?);
    let a4433 = v6;
    let a4434 = 0u64;
    if nat_eq(a4433, a4434) {
        Ok(Adt9::C7(Adt11::C7(16u64, v3, 0u64), Adt12::C0(0u64, f152(v7, v1)?), Adt12::C2(0u64), Adt13::C0(0u64), f170(v3), 4u64))
    } else {
        let a4435 = v6;
        let a4436 = 1u64;
        if nat_eq(a4435, a4436) {
            Ok(Adt9::C7(Adt11::C1(1u64), Adt12::C1(0u64), Adt12::C2(1065353216u64), {
                let a4437 = v4;
                let a4438 = 0u64;
                if nat_eq(a4437, a4438) {
                    v8
                } else {
                    Adt13::C0(0u64)
                }
            }, 4u64, 4u64))
        } else {
            Ok(Adt9::C7(Adt11::C2(0u64, v4, true), Adt12::C1(0u64), Adt12::C2(0u64), v8, 4u64, f170(v4)))
        }
    }
}

pub fn f165(v0: u64, v1: Adt3, v2: Adt3) -> R<Adt9> {
    let v3: u64 = f152(v0, v2.clone())?;
    let v4: u64 = f152(v0, v1.clone())?;
    let a4443 = {
        let a4440 = v0;
        let a4441 = match v1 {
            Adt3::C0(h4439, _, _, _, _, _, _, _) => {
                h4439
            }
        };
        let a4442 = 0u64;
        nat_rem(a4440, a4441, a4442)
    };
    let a4444 = 0u64;
    if nat_eq(a4443, a4444) {
        Ok(Adt9::C1(false, v4, v3))
    } else {
        Ok(Adt9::C4(8u64, 3u64, f152({
            let a4445 = v0;
            let a4446 = 1u64;
            nat_sub(a4445, a4446)
        }, v2)?, 0u64, v4, 3u64, v3))
    }
}

pub fn f166(v0: u64, v1: Adt3, v2: Adt3, v3: u64, v4: u64) -> R<Adt9> {
    let a4447 = v3;
    let a4448 = 13u64;
    if nat_eq(a4447, a4448) {
        f221(v0, v1, v4)
    } else {
        let v5: u64 = f211(match v1.clone() {
            Adt3::C0(h4449, _, _, _, _, _, _, _) => {
                h4449
            }
        }, {
            let a4450 = v3;
            let a4451 = 16u64;
            nat_eq(a4450, a4451)
        })?;
        let v6: u64 = {
            let a4452 = v0;
            let a4453 = v5;
            let a4454 = 0u64;
            nat_quot(a4452, a4453, a4454)
        };
        let v7: u64 = {
            let a4455 = v0;
            let a4456 = v5;
            let a4457 = 0u64;
            nat_rem(a4455, a4456, a4457)
        };
        let v8: u64 = {
            let a4461 = 1u64;
            let a4462 = {
                let a4459 = match v1.clone() {
                    Adt3::C0(h4458, _, _, _, _, _, _, _) => {
                        h4458
                    }
                };
                let a4460 = 2u64;
                nat_mul(a4459, a4460)?
            };
            nat_add(a4461, a4462)?
        };
        let a4463 = v7;
        let a4464 = v8;
        if nat_lt(a4463, a4464) {
            f222(v7, v6, v1, 0u64)
        } else {
            let v9: u64 = f152(v6, v2)?;
            let a4465 = v3;
            let a4466 = 14u64;
            if nat_eq(a4465, a4466) {
                Ok(Adt9::C4(2u64, 1u64, 1u64, 2u64, 0u64, 3u64, v9))
            } else {
                let a4467 = v7;
                let a4468 = v8;
                if nat_eq(a4467, a4468) {
                    Ok(Adt9::C4(2u64, 1u64, 1u64, 2u64, 0u64, 1u64, 1u64))
                } else {
                    let a4471 = v7;
                    let a4472 = {
                        let a4469 = v8;
                        let a4470 = 1u64;
                        nat_add(a4469, a4470)?
                    };
                    if nat_eq(a4471, a4472) {
                        Ok(Adt9::C4(3u64, 2u64, match v1 {
                            Adt3::C0(h4473, _, _, _, _, _, _, _) => {
                                h4473
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
    let a4474 = v0;
    let a4475 = 0u64;
    if nat_eq(a4474, a4475) {
        Ok(Adt9::C4(19u64, 2u64, 0u64, 2u64, 0u64, 1u64, 0u64))
    } else {
        let a4482 = {
            let a4476 = v0;
            let a4477 = 1u64;
            nat_add(a4476, a4477)?
        };
        let a4483 = {
            let a4480 = 2u64;
            let a4481 = {
                let a4478 = v3;
                let a4479 = 2u64;
                nat_mul(a4478, a4479)?
            };
            nat_add(a4480, a4481)?
        };
        if nat_eq(a4482, a4483) {
            Ok(Adt9::C4(19u64, 1u64, 0u64, 2u64, 0u64, 6u64, 0u64))
        } else {
            let v4: u64 = {
                let a4484 = v0;
                let a4485 = 1u64;
                nat_sub(a4484, a4485)
            };
            let v5: u64 = {
                let a4486 = v4;
                let a4487 = 2u64;
                let a4488 = 0u64;
                nat_quot(a4486, a4487, a4488)
            };
            let a4492 = {
                let a4489 = v4;
                let a4490 = 2u64;
                let a4491 = 0u64;
                nat_rem(a4489, a4490, a4491)
            };
            let a4493 = 0u64;
            if nat_eq(a4492, a4493) {
                Ok(Adt9::C4(17u64, 0u64, f152(v5, v1)?, 4u64, f152(v5, v2)?, 1u64, 2u64))
            } else {
                Ok(Adt9::C4(18u64, 1u64, 0u64, 1u64, 2u64, 1u64, 0u64))
            }
        }
    }
}

pub fn f168(v0: u64, v1: u64, v2: u64, v3: u64) -> R<Adt9> {
    let a4494 = v0;
    let a4495 = f27(v1, v2, v3)?;
    if nat_lt(a4494, a4495) {
        let a4496 = v0;
        let a4497 = v1;
        if nat_lt(a4496, a4497) {
            Ok(Adt9::C1(false, {
                let a4498 = 4u64;
                let a4499 = v0;
                nat_mul(a4498, a4499)?
            }, {
                let a4500 = 4u64;
                let a4501 = v0;
                nat_mul(a4500, a4501)?
            }))
        } else {
            let v4: u64 = {
                let a4502 = v0;
                let a4503 = v1;
                nat_sub(a4502, a4503)
            };
            let v5: u64 = f223(v4, v2)?;
            let a4504 = v5;
            let a4505 = 2u64;
            if nat_lt(a4504, a4505) {
                Ok(Adt9::C4(6u64, {
                    let a4506 = v5;
                    let a4507 = 0u64;
                    if nat_eq(a4506, a4507) {
                        4u64
                    } else {
                        5u64
                    }
                }, f224(v4, v2)?, 2u64, 0u64, 1u64, v5))
            } else {
                let v6: u64 = f225(v4, v2)?;
                Ok(Adt9::C4(8u64, 3u64, v6, 1u64, if f226(v4, v2)? {
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

pub fn f169(v0: u64, v1: u64) -> Adt11 {
    let a4508 = v0;
    let a4509 = 43u64;
    if nat_eq(a4508, a4509) {
        Adt11::C8(3u64)
    } else {
        let a4510 = v1;
        let a4511 = 43u64;
        if nat_eq(a4510, a4511) {
            Adt11::C8(4u64)
        } else {
            Adt11::C2(v0, v1, true)
        }
    }
}

pub fn f170(v0: u64) -> u64 {
    let m4512 = f127(v0);
    match m4512 {
        Adt17::C0 => {
            0u64
        }
        Adt17::C1(v1) => {
            let a4514 = match v1.clone() {
                Adt18::C0(_, h4513, _) => {
                    h4513
                }
            };
            let a4515 = 1u64;
            if if nat_eq(a4514, a4515) {
                let a4517 = match v1.clone() {
                    Adt18::C0(_, _, h4516) => {
                        h4516
                    }
                };
                let a4518 = 1u64;
                if if if nat_eq(a4517, a4518) {
                    true
                } else {
                    let a4520 = match v1.clone() {
                        Adt18::C0(_, _, h4519) => {
                            h4519
                        }
                    };
                    let a4521 = 2u64;
                    nat_eq(a4520, a4521)
                } {
                    true
                } else {
                    let a4523 = match v1.clone() {
                        Adt18::C0(_, _, h4522) => {
                            h4522
                        }
                    };
                    let a4524 = 4u64;
                    nat_eq(a4523, a4524)
                } {
                    true
                } else {
                    let a4526 = match v1.clone() {
                        Adt18::C0(_, _, h4525) => {
                            h4525
                        }
                    };
                    let a4527 = 8u64;
                    nat_eq(a4526, a4527)
                }
            } else {
                false
            } {
                match v1 {
                    Adt18::C0(_, _, h4528) => {
                        h4528
                    }
                }
            } else {
                0u64
            }
        }
    }
}

pub fn f171(v0: u64, v1: u64) -> R<u64> {
    if f116(v0) {
        let a4529 = 2u64;
        let a4530 = v1;
        nat_mul(a4529, a4530)
    } else {
        let a4531 = v1;
        let a4532 = f207(v0);
        let a4533 = 0u64;
        Ok(nat_quot(a4531, a4532, a4533))
    }
}

pub fn f172(v0: u64, v1: u64, v2: u64, v3: u64, v4: Adt3, v5: Adt3) -> R<Adt9> {
    let m4534 = f127(v0);
    match m4534 {
        Adt17::C0 => {
            Ok(Adt9::C0(Adt10::C1))
        }
        Adt17::C1(v6) => {
            Ok(Adt9::C10(v0, f144(v2, v1, 0u64, 0u64, match v4.clone() {
                Adt3::C0(_, _, _, _, h4535, _, _, _) => {
                    h4535
                }
            }, match v4.clone() {
                Adt3::C0(_, _, _, _, _, h4536, _, _) => {
                    h4536
                }
            }, match v4.clone() {
                Adt3::C0(_, _, _, _, _, _, h4537, _) => {
                    h4537
                }
            }, match v4 {
                Adt3::C0(_, _, _, _, _, _, _, h4538) => {
                    h4538
                }
            })?, match v6.clone() {
                Adt18::C0(_, _, h4539) => {
                    h4539
                }
            }, f144({
                let a4541 = v2;
                let a4542 = match v6.clone() {
                    Adt18::C0(_, h4540, _) => {
                        h4540
                    }
                };
                nat_mul(a4541, a4542)?
            }, v3, 0u64, 0u64, match v5.clone() {
                Adt3::C0(_, _, _, _, h4543, _, _, _) => {
                    h4543
                }
            }, match v5.clone() {
                Adt3::C0(_, _, _, _, _, h4544, _, _) => {
                    h4544
                }
            }, match v5.clone() {
                Adt3::C0(_, _, _, _, _, _, h4545, _) => {
                    h4545
                }
            }, match v5.clone() {
                Adt3::C0(_, _, _, _, _, _, _, h4546) => {
                    h4546
                }
            })?, match v6 {
                Adt18::C0(_, h4547, _) => {
                    h4547
                }
            }, match v5 {
                Adt3::C0(_, _, _, _, h4548, _, _, _) => {
                    h4548
                }
            }))
        }
    }
}

pub fn f173(v0: Adt3, v1: Adt3) -> R<u64> {
    let a4556 = {
        let a4549 = 2u64;
        let a4550 = f140(v1.clone())?;
        nat_mul(a4549, a4550)?
    };
    let a4557 = {
        let a4554 = f140(v1)?;
        let a4555 = {
            let a4552 = 1u64;
            let a4553 = match v0 {
                Adt3::C0(h4551, _, _, _, _, _, _, _) => {
                    h4551
                }
            };
            nat_add(a4552, a4553)?
        };
        nat_mul(a4554, a4555)?
    };
    nat_add(a4556, a4557)
}

pub fn f174(v0: u64, v1: Adt3) -> R<Adt9> {
    Ok(Adt9::C4(19u64, 4u64, f152(v0, v1)?, 2u64, 0u64, 1u64, 2u64))
}

pub fn f175(v0: u64, v1: u64) -> bool {
    let a4558 = v1;
    let a4559 = 2147483647u64;
    if nat_le(a4558, a4559) {
        let a4560 = v1;
        let a4561 = v0;
        nat_lt(a4560, a4561)
    } else {
        false
    }
}

pub fn f176(v0: u64, v1: u64, v2: u64, v3: Adt3, v4: Adt3, v5: Adt3, v6: u64) -> R<Adt9> {
    let m4562 = f127(v0);
    match m4562 {
        Adt17::C0 => {
            Ok(Adt9::C0(Adt10::C1))
        }
        Adt17::C1(v7) => {
            let v8: u64 = {
                let a4564 = v2;
                let a4565 = match v7.clone() {
                    Adt18::C0(_, h4563, _) => {
                        h4563
                    }
                };
                nat_mul(a4564, a4565)?
            };
            let m4578 = f227(v0, v8, v6, v1, match v5.clone() {
                Adt3::C0(h4566, _, _, _, _, _, _, _) => {
                    h4566
                }
            }, match v5.clone() {
                Adt3::C0(_, h4567, _, _, _, _, _, _) => {
                    h4567
                }
            }, match v5 {
                Adt3::C0(_, _, h4568, _, _, _, _, _) => {
                    h4568
                }
            }, {
                let a4571 = match v3.clone() {
                    Adt3::C0(h4569, _, _, _, _, _, _, _) => {
                        h4569
                    }
                };
                let a4572 = match v7.clone() {
                    Adt18::C0(_, h4570, _) => {
                        h4570
                    }
                };
                nat_mul(a4571, a4572)?
            }, match v3.clone() {
                Adt3::C0(_, h4573, _, _, _, _, _, _) => {
                    h4573
                }
            }, match v3.clone() {
                Adt3::C0(_, _, _, _, h4574, _, _, _) => {
                    h4574
                }
            }, match v3.clone() {
                Adt3::C0(_, _, _, _, _, h4575, _, _) => {
                    h4575
                }
            }, match v3.clone() {
                Adt3::C0(_, _, _, _, _, _, h4576, _) => {
                    h4576
                }
            }, match v3 {
                Adt3::C0(_, _, _, _, _, _, _, h4577) => {
                    h4577
                }
            })?;
            match m4578 {
                None => {
                    Ok(Adt9::C0(Adt10::C1))
                }
                Some(v9) => {
                    let a4579 = v0;
                    let a4580 = 0u64;
                    if if nat_eq(a4579, a4580) {
                        true
                    } else {
                        let a4581 = v0;
                        let a4582 = 26u64;
                        nat_eq(a4581, a4582)
                    } {
                        Ok(Adt9::C1(false, v9, f152({
                            let a4586 = {
                                let a4584 = v1;
                                let a4585 = match v4.clone() {
                                    Adt3::C0(h4583, _, _, _, _, _, _, _) => {
                                        h4583
                                    }
                                };
                                nat_mul(a4584, a4585)?
                            };
                            let a4587 = v8;
                            nat_add(a4586, a4587)?
                        }, v4)?))
                    } else {
                        Ok(Adt9::C5(v0, v9, match v7.clone() {
                            Adt18::C0(_, _, h4588) => {
                                h4588
                            }
                        }, f152({
                            let a4592 = {
                                let a4590 = v1;
                                let a4591 = match v4.clone() {
                                    Adt3::C0(h4589, _, _, _, _, _, _, _) => {
                                        h4589
                                    }
                                };
                                nat_mul(a4590, a4591)?
                            };
                            let a4593 = v8;
                            nat_add(a4592, a4593)?
                        }, v4.clone())?, {
                            let a4600 = {
                                let a4598 = {
                                    let a4595 = match v7 {
                                        Adt18::C0(_, h4594, _) => {
                                            h4594
                                        }
                                    };
                                    let a4596 = 1u64;
                                    nat_sub(a4595, a4596)
                                };
                                let a4599 = match v4.clone() {
                                    Adt3::C0(_, _, _, _, h4597, _, _, _) => {
                                        h4597
                                    }
                                };
                                nat_mul(a4598, a4599)?
                            };
                            let a4601 = 4u64;
                            nat_add(a4600, a4601)?
                        }, match v4 {
                            Adt3::C0(_, _, _, _, h4602, _, _, _) => {
                                h4602
                            }
                        }))
                    }
                }
            }
        }
    }
}

pub fn f177(v0: u64, v1: u64, v2: u64) -> u64 {
    let a4603 = v1;
    let a4604 = 0u64;
    if if nat_eq(a4603, a4604) {
        true
    } else {
        let a4605 = v2;
        let a4606 = 0u64;
        nat_eq(a4605, a4606)
    } {
        0u64
    } else {
        v0
    }
}

pub fn f178(v0: u64, v1: u64) -> bool {
    let a4607 = v0;
    let a4608 = 4294967295u64;
    if nat_eq(a4607, a4608) {
        true
    } else {
        let a4609 = v0;
        let a4610 = 2147483648u64;
        if nat_lt(a4609, a4610) {
            let a4611 = v0;
            let a4612 = v1;
            nat_lt(a4611, a4612)
        } else {
            false
        }
    }
}

pub fn f179(v0: Adt7) -> R<u64> {
    let a4617 = if match v0.clone() {
        Adt7::C0(_, _, _, h4613, _) => {
            h4613
        }
    } {
        2u64
    } else {
        1u64
    };
    let a4618 = {
        let a4615 = match v0 {
            Adt7::C0(_, _, _, _, h4614) => {
                h4614
            }
        };
        let a4616 = 1u64;
        if nat_eq(a4615, a4616) {
            1u64
        } else {
            0u64
        }
    };
    nat_add(a4617, a4618)
}

pub fn f180(v0: Adt6, v1: u64, v2: u64) -> R<u64> {
    let v3: u64 = {
        let a4623 = {
            let a4620 = v2;
            let a4621 = match v0.clone() {
                Adt6::C0(_, h4619, _, _) => {
                    h4619
                }
            };
            nat_mul(a4620, a4621)?
        };
        let a4624 = f145(match v0.clone() {
            Adt6::C0(_, _, h4622, _) => {
                h4622
            }
        });
        nat_add(a4623, a4624)?
    };
    let v4: u64 = f146(match v0.clone() {
        Adt6::C0(_, _, h4625, _) => {
            h4625
        }
    });
    let v5: u64 = {
        let a4629 = {
            let a4627 = v3;
            let a4628 = match v0 {
                Adt6::C0(h4626, _, _, _) => {
                    h4626
                }
            };
            nat_add(a4627, a4628)?
        };
        let a4630 = v4;
        nat_sub(a4629, a4630)
    };
    let a4635 = {
        let a4631 = v5;
        let a4632 = v1;
        if nat_lt(a4631, a4632) {
            v5
        } else {
            v1
        }
    };
    let a4636 = {
        let a4633 = v3;
        let a4634 = v4;
        nat_sub(a4633, a4634)
    };
    Ok(nat_sub(a4635, a4636))
}

pub fn f181(v0: Adt7, v1: Adt3, v2: Adt3, v3: u64, v4: u64) -> R<Option<u64>> {
    let m4645 = f76(match v0.clone() {
        Adt7::C0(h4637, _, _, _, _) => {
            h4637
        }
    }, match v1.clone() {
        Adt3::C0(h4638, _, _, _, _, _, _, _) => {
            h4638
        }
    }, f74(v3, v2.clone(), 0u64), {
        let a4642 = v4;
        let a4643 = {
            let m4641 = match v0.clone() {
                Adt7::C0(h4640, _, _, _, _) => {
                    h4640
                }
            };
            match m4641 {
                Adt6::C0(h4639, _, _, _) => {
                    h4639
                }
            }
        };
        let a4644 = 0u64;
        nat_rem(a4642, a4643, a4644)
    })?;
    match m4645 {
        None => {
            Ok(None::<u64>)
        }
        Some(v5) => {
            let m4654 = f76(match v0.clone() {
                Adt7::C0(_, h4646, _, _, _) => {
                    h4646
                }
            }, match v1.clone() {
                Adt3::C0(_, h4647, _, _, _, _, _, _) => {
                    h4647
                }
            }, f74(v3, v2.clone(), 1u64), {
                let a4651 = v4;
                let a4652 = {
                    let m4650 = match v0 {
                        Adt7::C0(h4649, _, _, _, _) => {
                            h4649
                        }
                    };
                    match m4650 {
                        Adt6::C0(h4648, _, _, _) => {
                            h4648
                        }
                    }
                };
                let a4653 = 0u64;
                nat_quot(a4651, a4652, a4653)
            })?;
            match m4654 {
                None => {
                    Ok(None::<u64>)
                }
                Some(v6) => {
                    Ok(Some(f73(v1, v5, v6, f74(v3, v2.clone(), 2u64), f74(v3, v2, 3u64))?))
                }
            }
        }
    }
}

pub fn f182(v0: Adt16, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64, v7: u64, v8: u64, v9: u64, v10: u64, v11: u64, v12: u64, v13: u64, v14: u64, v15: u64, v16: u64, v17: u64, v18: u64) -> R<Adt19> {
    let m4655 = v0;
    match m4655 {
        Adt16::C0(v19) => {
            f228(v1, v2, v3, v4, v5, v6, v7, v8, v9, v14, v15, v16, v17, v19, v18)
        }
        Adt16::C1(v20, v21, v22, v23) => {
            let a4656 = v20;
            let a4657 = 66u64;
            if nat_eq(a4656, a4657) {
                f229(v1, v2, v3, v4, v5, v6, v7, v8, v9, v10, v11, v12, v13, v14, v15, v16, v17, v21, v22, v23, v18)
            } else {
                f230(v20, v1, v2, v3, v4, v5, v6, v7, v8, v9, v14, v15, v16, v17, v21, v22, v23, v18)
            }
        }
    }
}

pub fn f183(v0: u64, v1: Adt3, v2: u64, v3: u64, v4: u64) -> R<Adt9> {
    let v5: u64 = {
        let a4664 = v3;
        let a4665 = {
            let a4661 = f152({
                let a4658 = v0;
                let a4659 = 2u64;
                let a4660 = 0u64;
                nat_quot(a4658, a4659, a4660)
            }, v1)?;
            let a4662 = v2;
            let a4663 = 0u64;
            nat_quot(a4661, a4662, a4663)
        };
        nat_add(a4664, a4665)?
    };
    let a4669 = {
        let a4666 = v0;
        let a4667 = 2u64;
        let a4668 = 0u64;
        nat_rem(a4666, a4667, a4668)
    };
    let a4670 = 0u64;
    if nat_eq(a4669, a4670) {
        Ok(Adt9::C4(19u64, 1u64, v5, 2u64, 0u64, 1u64, 2u64))
    } else {
        let a4671 = v4;
        let a4672 = 0u64;
        if nat_eq(a4671, a4672) {
            Ok(Adt9::C4(19u64, 2u64, 1u64, 2u64, 0u64, 1u64, v5))
        } else {
            Ok(Adt9::C0(Adt10::C1))
        }
    }
}

pub fn f184(v0: u64, v1: u64) -> R<u64> {
    let a4675 = 8u64;
    let a4676 = {
        let a4673 = f231(v0);
        let a4674 = v1;
        nat_mul(a4673, a4674)?
    };
    nat_add(a4675, a4676)
}

pub fn f185(v0: u64, v1: u64, v2: Adt3, v3: u64, v4: u64, v5: u64) -> R<Adt9> {
    let v6: u64 = match v2.clone() {
        Adt3::C0(h4677, _, _, _, _, _, _, _) => {
            h4677
        }
    };
    let v7: u64 = f231(v3);
    let v8: u64 = {
        let a4680 = 1u64;
        let a4681 = {
            let a4678 = v6;
            let a4679 = v7;
            nat_mul(a4678, a4679)?
        };
        nat_add(a4680, a4681)?
    };
    let a4682 = v0;
    let a4683 = 0u64;
    if nat_eq(a4682, a4683) {
        Ok(f232(0u64))
    } else {
        let a4684 = v0;
        let a4685 = v8;
        if nat_lt(a4684, a4685) {
            let v9: u64 = {
                let a4686 = v0;
                let a4687 = 1u64;
                nat_sub(a4686, a4687)
            };
            let v10: u64 = {
                let a4688 = v9;
                let a4689 = v7;
                let a4690 = 0u64;
                nat_rem(a4688, a4689, a4690)
            };
            let v11: u64 = f152({
                let a4696 = {
                    let a4691 = v1;
                    let a4692 = v6;
                    nat_mul(a4691, a4692)?
                };
                let a4697 = {
                    let a4693 = v9;
                    let a4694 = v7;
                    let a4695 = 0u64;
                    nat_quot(a4693, a4694, a4695)
                };
                nat_add(a4696, a4697)?
            }, v2)?;
            let a4698 = v3;
            let a4699 = 30u64;
            if nat_eq(a4698, a4699) {
                let a4700 = v10;
                let a4701 = 0u64;
                if nat_eq(a4700, a4701) {
                    Ok(Adt9::C4(6u64, 0u64, v11, 2u64, 0u64, 1u64, 1u64))
                } else {
                    let a4702 = v10;
                    let a4703 = 1u64;
                    if nat_eq(a4702, a4703) {
                        Ok(Adt9::C4(10u64, 1u64, 1u64, 1u64, 1u64, 1u64, 1u64))
                    } else {
                        let a4704 = v10;
                        let a4705 = 2u64;
                        if nat_eq(a4704, a4705) {
                            Ok(Adt9::C4(1u64, 1u64, 1u64, 2u64, 0u64, 1u64, 1u64))
                        } else {
                            Ok(Adt9::C4(0u64, 1u64, 0u64, 1u64, 1u64, 1u64, 0u64))
                        }
                    }
                }
            } else {
                let a4706 = v10;
                let a4707 = 0u64;
                if nat_eq(a4706, a4707) {
                    Ok(Adt9::C4(10u64, 0u64, v11, 0u64, v11, 1u64, 1u64))
                } else {
                    let a4708 = v10;
                    let a4709 = 1u64;
                    if nat_eq(a4708, a4709) {
                        Ok(Adt9::C4(1u64, 1u64, 1u64, 2u64, 0u64, 1u64, 1u64))
                    } else {
                        Ok(Adt9::C4(0u64, 1u64, 0u64, 1u64, 1u64, 1u64, 0u64))
                    }
                }
            }
        } else {
            let v12: u64 = {
                let a4710 = v0;
                let a4711 = v8;
                nat_sub(a4710, a4711)
            };
            let a4712 = v12;
            let a4713 = 0u64;
            if nat_eq(a4712, a4713) {
                Ok(Adt9::C4(22u64, 2u64, v6, 2u64, 0u64, 1u64, 1u64))
            } else {
                let a4714 = v12;
                let a4715 = 1u64;
                if nat_eq(a4714, a4715) {
                    Ok(Adt9::C4(21u64, 1u64, 0u64, 1u64, 1u64, 1u64, 1u64))
                } else {
                    let a4716 = v12;
                    let a4717 = 2u64;
                    if nat_eq(a4716, a4717) {
                        Ok(Adt9::C4(2u64, 1u64, 1u64, 2u64, 0u64, 1u64, 1u64))
                    } else {
                        let a4718 = v12;
                        let a4719 = 3u64;
                        if nat_eq(a4718, a4719) {
                            Ok(Adt9::C4(4u64, 1u64, 1u64, 2u64, v4, 1u64, 1u64))
                        } else {
                            let a4720 = v12;
                            let a4721 = 4u64;
                            if nat_eq(a4720, a4721) {
                                Ok(Adt9::C4(12u64, 1u64, 1u64, 2u64, 0u64, 1u64, 1u64))
                            } else {
                                let a4722 = v12;
                                let a4723 = 5u64;
                                if nat_eq(a4722, a4723) {
                                    Ok(Adt9::C4(11u64, 2u64, 1065353216u64, 1u64, 1u64, 1u64, 2u64))
                                } else {
                                    let a4724 = v12;
                                    let a4725 = 6u64;
                                    if nat_eq(a4724, a4725) {
                                        if f233(v5) {
                                            Ok(Adt9::C4(19u64, 1u64, 2u64, 2u64, 0u64, 1u64, {
                                                let a4726 = 3u64;
                                                let a4727 = v1;
                                                nat_add(a4726, a4727)?
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

pub fn f186(v0: u64, v1: Adt3, v2: Adt3, v3: u64) -> R<Adt9> {
    let v4: u64 = {
        let a4728 = v0;
        let a4729 = f110(v3);
        let a4730 = 0u64;
        nat_quot(a4728, a4729, a4730)
    };
    let v5: u64 = {
        let a4735 = 3u64;
        let a4736 = {
            let a4732 = v4;
            let a4733 = match v1.clone() {
                Adt3::C0(h4731, _, _, _, _, _, _, _) => {
                    h4731
                }
            };
            let a4734 = 0u64;
            nat_quot(a4732, a4733, a4734)
        };
        nat_add(a4735, a4736)?
    };
    let v6: u64 = f152(v4, v1)?;
    let v7: u64 = f152(v4, v2)?;
    let a4737 = v3;
    let a4738 = 30u64;
    if nat_eq(a4737, a4738) {
        let a4742 = {
            let a4739 = v0;
            let a4740 = 3u64;
            let a4741 = 0u64;
            nat_rem(a4739, a4740, a4741)
        };
        let a4743 = 0u64;
        if nat_eq(a4742, a4743) {
            Ok(Adt9::C4(6u64, 0u64, v6, 2u64, 0u64, 1u64, 1u64))
        } else {
            let a4747 = {
                let a4744 = v0;
                let a4745 = 3u64;
                let a4746 = 0u64;
                nat_rem(a4744, a4745, a4746)
            };
            let a4748 = 1u64;
            if nat_eq(a4747, a4748) {
                Ok(Adt9::C4(10u64, 1u64, 1u64, 1u64, v5, 1u64, 1u64))
            } else {
                Ok(Adt9::C4(7u64, 1u64, 1u64, 2u64, 0u64, 3u64, v7))
            }
        }
    } else {
        Ok(Adt9::C4(10u64, 0u64, v6, 1u64, v5, 3u64, v7))
    }
}

pub fn f187(v0: u64) -> R<u64> {
    let v1: u64 = {
        let a4749 = v0;
        let a4750 = 8388608u64;
        let a4751 = 0u64;
        nat_quot(a4749, a4750, a4751)
    };
    let a4752 = v0;
    let a4753 = 2147483648u64;
    if if if nat_lt(a4752, a4753) {
        let a4754 = 127u64;
        let a4755 = v1;
        nat_le(a4754, a4755)
    } else {
        false
    } {
        let a4756 = v1;
        let a4757 = 190u64;
        nat_lt(a4756, a4757)
    } else {
        false
    } {
        let v2: u64 = {
            let a4761 = 8388608u64;
            let a4762 = {
                let a4758 = v0;
                let a4759 = 8388608u64;
                let a4760 = 0u64;
                nat_rem(a4758, a4759, a4760)
            };
            nat_add(a4761, a4762)?
        };
        let a4763 = v1;
        let a4764 = 150u64;
        if nat_lt(a4763, a4764) {
            let v3: u64 = f234({
                let a4765 = 150u64;
                let a4766 = v1;
                nat_sub(a4765, a4766)
            });
            let a4770 = {
                let a4767 = v2;
                let a4768 = v3;
                let a4769 = 0u64;
                nat_rem(a4767, a4768, a4769)
            };
            let a4771 = 0u64;
            if nat_eq(a4770, a4771) {
                let a4772 = v2;
                let a4773 = v3;
                let a4774 = 0u64;
                Ok(nat_quot(a4772, a4773, a4774))
            } else {
                Ok(0u64)
            }
        } else {
            let a4777 = f234({
                let a4775 = v1;
                let a4776 = 150u64;
                nat_sub(a4775, a4776)
            });
            let a4778 = v2;
            nat_mul(a4777, a4778)
        }
    } else {
        Ok(0u64)
    }
}

pub fn f188(v0: Adt14, v1: Adt14) -> bool {
    let a4781 = match v0.clone() {
        Adt14::C0(_, h4779, _, _, _, _, _, _, _, _, _) => {
            h4779
        }
    };
    let a4782 = match v1.clone() {
        Adt14::C0(_, h4780, _, _, _, _, _, _, _, _, _) => {
            h4780
        }
    };
    if if if nat_eq(a4781, a4782) {
        let a4785 = match v0.clone() {
            Adt14::C0(_, _, h4783, _, _, _, _, _, _, _, _) => {
                h4783
            }
        };
        let a4786 = match v1.clone() {
            Adt14::C0(_, _, h4784, _, _, _, _, _, _, _, _) => {
                h4784
            }
        };
        nat_eq(a4785, a4786)
    } else {
        false
    } {
        let a4789 = match v0.clone() {
            Adt14::C0(_, _, _, h4787, _, _, _, _, _, _, _) => {
                h4787
            }
        };
        let a4790 = match v1.clone() {
            Adt14::C0(_, _, _, h4788, _, _, _, _, _, _, _) => {
                h4788
            }
        };
        nat_eq(a4789, a4790)
    } else {
        false
    } {
        let a4793 = match v0 {
            Adt14::C0(_, _, _, _, h4791, _, _, _, _, _, _) => {
                h4791
            }
        };
        let a4794 = match v1 {
            Adt14::C0(_, _, _, _, h4792, _, _, _, _, _, _) => {
                h4792
            }
        };
        nat_eq(a4793, a4794)
    } else {
        false
    }
}

pub fn f189(v0: u64, v1: u64, v2: u64, v3: u64) -> bool {
    let a4795 = v0;
    let a4796 = 0u64;
    if if if nat_eq(a4795, a4796) {
        true
    } else {
        let a4797 = v1;
        let a4798 = 0u64;
        nat_eq(a4797, a4798)
    } {
        true
    } else {
        let a4799 = v2;
        let a4800 = 0u64;
        nat_eq(a4799, a4800)
    } {
        true
    } else {
        let a4801 = v3;
        let a4802 = 0u64;
        nat_eq(a4801, a4802)
    }
}

pub fn f190(v0: Adt14, v1: Adt14) -> bool {
    if f196(match v0.clone() {
        Adt14::C0(_, _, _, _, _, h4803, _, _, _, _, _) => {
            h4803
        }
    }, match v1 {
        Adt14::C0(_, _, _, _, _, h4804, _, _, _, _, _) => {
            h4804
        }
    }) {
        let a4806 = match v0 {
            Adt14::C0(_, h4805, _, _, _, _, _, _, _, _, _) => {
                h4805
            }
        };
        let a4807 = 2147483647u64;
        nat_le(a4806, a4807)
    } else {
        true
    }
}

pub fn f191(v0: Adt14) -> bool {
    let a4812 = {
        let a4809 = match v0.clone() {
            Adt14::C0(_, _, _, _, _, _, _, _, _, _, h4808) => {
                h4808
            }
        };
        let a4810 = 4u64;
        let a4811 = 0u64;
        nat_rem(a4809, a4810, a4811)
    };
    let a4813 = 0u64;
    if if nat_eq(a4812, a4813) {
        f235(match v0.clone() {
            Adt14::C0(_, h4814, _, _, _, _, _, _, _, _, _) => {
                h4814
            }
        }, match v0.clone() {
            Adt14::C0(_, _, _, _, _, h4815, _, _, _, _, _) => {
                h4815
            }
        })
    } else {
        false
    } {
        f106(match v0.clone() {
            Adt14::C0(_, _, h4816, _, _, _, _, _, _, _, _) => {
                h4816
            }
        }, match v0.clone() {
            Adt14::C0(_, _, _, h4817, _, _, _, _, _, _, _) => {
                h4817
            }
        }, match v0.clone() {
            Adt14::C0(_, _, _, _, h4818, _, _, _, _, _, _) => {
                h4818
            }
        }, match v0.clone() {
            Adt14::C0(_, _, _, _, _, _, h4819, _, _, _, _) => {
                h4819
            }
        }, match v0.clone() {
            Adt14::C0(_, _, _, _, _, _, _, h4820, _, _, _) => {
                h4820
            }
        }, match v0 {
            Adt14::C0(_, _, _, _, _, _, _, _, h4821, _, _) => {
                h4821
            }
        }, 4u64)
    } else {
        false
    }
}

pub fn f192(v0: Adt14, v1: Adt14) -> R<bool> {
    let a4823 = match v0.clone() {
        Adt14::C0(_, _, _, _, _, _, _, _, _, h4822, _) => {
            h4822
        }
    };
    let a4824 = 0u64;
    if nat_eq(a4823, a4824) {
        Ok(true)
    } else {
        let a4826 = match v1.clone() {
            Adt14::C0(_, _, _, _, _, _, _, _, _, h4825, _) => {
                h4825
            }
        };
        let a4827 = 0u64;
        if nat_eq(a4826, a4827) {
            Ok(true)
        } else {
            let a4829 = match v0.clone() {
                Adt14::C0(_, _, _, _, _, _, _, _, _, _, h4828) => {
                    h4828
                }
            };
            let a4830 = 0u64;
            if nat_eq(a4829, a4830) {
                Ok(true)
            } else {
                let a4832 = match v1.clone() {
                    Adt14::C0(_, _, _, _, _, _, _, _, _, _, h4831) => {
                        h4831
                    }
                };
                let a4833 = 0u64;
                if nat_eq(a4832, a4833) {
                    Ok(true)
                } else {
                    let a4839 = {
                        let a4836 = match v0.clone() {
                            Adt14::C0(_, _, _, _, _, _, _, _, _, _, h4834) => {
                                h4834
                            }
                        };
                        let a4837 = match v0.clone() {
                            Adt14::C0(_, _, _, _, _, _, _, _, _, h4835, _) => {
                                h4835
                            }
                        };
                        nat_add(a4836, a4837)?
                    };
                    let a4840 = match v1.clone() {
                        Adt14::C0(_, _, _, _, _, _, _, _, _, _, h4838) => {
                            h4838
                        }
                    };
                    if nat_le(a4839, a4840) {
                        Ok(true)
                    } else {
                        let a4846 = {
                            let a4843 = match v1.clone() {
                                Adt14::C0(_, _, _, _, _, _, _, _, _, _, h4841) => {
                                    h4841
                                }
                            };
                            let a4844 = match v1 {
                                Adt14::C0(_, _, _, _, _, _, _, _, _, h4842, _) => {
                                    h4842
                                }
                            };
                            nat_add(a4843, a4844)?
                        };
                        let a4847 = match v0 {
                            Adt14::C0(_, _, _, _, _, _, _, _, _, _, h4845) => {
                                h4845
                            }
                        };
                        Ok(nat_le(a4846, a4847))
                    }
                }
            }
        }
    }
}

pub fn f193(v0: Adt14, v1: Adt14) -> bool {
    if if if if if if f188(v0.clone(), v1.clone()) {
        let a4850 = match v0.clone() {
            Adt14::C0(h4848, _, _, _, _, _, _, _, _, _, _) => {
                h4848
            }
        };
        let a4851 = match v1.clone() {
            Adt14::C0(h4849, _, _, _, _, _, _, _, _, _, _) => {
                h4849
            }
        };
        nat_eq(a4850, a4851)
    } else {
        false
    } {
        let a4854 = match v0.clone() {
            Adt14::C0(_, _, _, _, _, _, _, _, _, _, h4852) => {
                h4852
            }
        };
        let a4855 = match v1.clone() {
            Adt14::C0(_, _, _, _, _, _, _, _, _, _, h4853) => {
                h4853
            }
        };
        nat_eq(a4854, a4855)
    } else {
        false
    } {
        let a4858 = match v0.clone() {
            Adt14::C0(_, _, _, _, _, h4856, _, _, _, _, _) => {
                h4856
            }
        };
        let a4859 = match v1.clone() {
            Adt14::C0(_, _, _, _, _, h4857, _, _, _, _, _) => {
                h4857
            }
        };
        nat_eq(a4858, a4859)
    } else {
        false
    } {
        let a4862 = match v0.clone() {
            Adt14::C0(_, _, _, _, _, _, h4860, _, _, _, _) => {
                h4860
            }
        };
        let a4863 = match v1.clone() {
            Adt14::C0(_, _, _, _, _, _, h4861, _, _, _, _) => {
                h4861
            }
        };
        nat_eq(a4862, a4863)
    } else {
        false
    } {
        let a4866 = match v0.clone() {
            Adt14::C0(_, _, _, _, _, _, _, h4864, _, _, _) => {
                h4864
            }
        };
        let a4867 = match v1.clone() {
            Adt14::C0(_, _, _, _, _, _, _, h4865, _, _, _) => {
                h4865
            }
        };
        nat_eq(a4866, a4867)
    } else {
        false
    } {
        let a4870 = match v0 {
            Adt14::C0(_, _, _, _, _, _, _, _, h4868, _, _) => {
                h4868
            }
        };
        let a4871 = match v1 {
            Adt14::C0(_, _, _, _, _, _, _, _, h4869, _, _) => {
                h4869
            }
        };
        nat_eq(a4870, a4871)
    } else {
        false
    }
}

pub fn f194(v0: Adt14) -> R<bool> {
    if f236(match v0.clone() {
        Adt14::C0(_, h4872, _, _, _, _, _, _, _, _, _) => {
            h4872
        }
    }, match v0.clone() {
        Adt14::C0(_, _, _, _, _, h4873, _, _, _, _, _) => {
            h4873
        }
    }) {
        f107(f237(match v0.clone() {
            Adt14::C0(_, h4874, _, _, _, _, _, _, _, _, _) => {
                h4874
            }
        }, match v0.clone() {
            Adt14::C0(_, _, _, _, _, h4875, _, _, _, _, _) => {
                h4875
            }
        })?, match v0.clone() {
            Adt14::C0(_, _, h4876, _, _, _, _, _, _, _, _) => {
                h4876
            }
        }, match v0.clone() {
            Adt14::C0(_, _, _, h4877, _, _, _, _, _, _, _) => {
                h4877
            }
        }, match v0.clone() {
            Adt14::C0(_, _, _, _, h4878, _, _, _, _, _, _) => {
                h4878
            }
        }, match v0.clone() {
            Adt14::C0(_, _, _, _, _, _, h4879, _, _, _, _) => {
                h4879
            }
        }, match v0.clone() {
            Adt14::C0(_, _, _, _, _, _, _, h4880, _, _, _) => {
                h4880
            }
        }, match v0 {
            Adt14::C0(_, _, _, _, _, _, _, _, h4881, _, _) => {
                h4881
            }
        })
    } else {
        Ok(false)
    }
}

pub fn f195(v0: bool, v1: u64, v2: u64, v3: u64) -> R<u64> {
    let a4882 = v1;
    let a4883 = 0u64;
    if if nat_eq(a4882, a4883) {
        true
    } else {
        let a4884 = v2;
        let a4885 = 0u64;
        nat_eq(a4884, a4885)
    } {
        Ok(0u64)
    } else {
        let a4888 = f108(v1, v2, v3)?;
        let a4889 = {
            let a4886 = v2;
            let a4887 = f238(v0, v1)?;
            nat_mul(a4886, a4887)?
        };
        nat_add(a4888, a4889)
    }
}

pub fn f196(v0: u64, v1: u64) -> bool {
    let a4890 = v0;
    let a4891 = 4u64;
    if nat_eq(a4890, a4891) {
        let a4892 = v1;
        let a4893 = 4u64;
        nat_eq(a4892, a4893)
    } else {
        false
    }
}

pub fn f197(v0: u64) -> R<u64> {
    let a4894 = 6u64;
    let a4895 = v0;
    nat_add(a4894, a4895)
}

pub fn f198(v0: Adt14) -> Adt14 {
    Adt14::C0(match v0.clone() {
        Adt14::C0(h4896, _, _, _, _, _, _, _, _, _, _) => {
            h4896
        }
    }, match v0.clone() {
        Adt14::C0(_, h4897, _, _, _, _, _, _, _, _, _) => {
            h4897
        }
    }, match v0.clone() {
        Adt14::C0(_, _, h4898, _, _, _, _, _, _, _, _) => {
            h4898
        }
    }, match v0.clone() {
        Adt14::C0(_, _, _, h4899, _, _, _, _, _, _, _) => {
            h4899
        }
    }, match v0.clone() {
        Adt14::C0(_, _, _, _, h4900, _, _, _, _, _, _) => {
            h4900
        }
    }, 4u64, match v0.clone() {
        Adt14::C0(_, _, _, _, _, _, h4901, _, _, _, _) => {
            h4901
        }
    }, match v0.clone() {
        Adt14::C0(_, _, _, _, _, _, _, h4902, _, _, _) => {
            h4902
        }
    }, match v0.clone() {
        Adt14::C0(_, _, _, _, _, _, _, _, h4903, _, _) => {
            h4903
        }
    }, match v0.clone() {
        Adt14::C0(_, _, _, _, _, _, _, _, _, h4904, _) => {
            h4904
        }
    }, match v0 {
        Adt14::C0(_, _, _, _, _, _, _, _, _, _, h4905) => {
            h4905
        }
    })
}

pub fn f199(v0: u64, v1: u64, v2: u64, v3: u64) -> R<bool> {
    let a4906 = v0;
    let a4907 = 2147483647u64;
    if if if nat_le(a4906, a4907) {
        let a4908 = 0u64;
        let a4909 = v3;
        if nat_lt(a4908, a4909) {
            let a4910 = v3;
            let a4911 = 2147483647u64;
            nat_le(a4910, a4911)
        } else {
            false
        }
    } else {
        false
    } {
        let a4916 = {
            let a4914 = {
                let a4912 = v2;
                let a4913 = v3;
                nat_add(a4912, a4913)?
            };
            let a4915 = 1u64;
            nat_sub(a4914, a4915)
        };
        let a4917 = 2147483647u64;
        nat_le(a4916, a4917)
    } else {
        false
    } {
        let a4922 = {
            let a4920 = {
                let a4918 = v0;
                let a4919 = v1;
                nat_mul(a4918, a4919)?
            };
            let a4921 = f239(v2, v3)?;
            nat_mul(a4920, a4921)?
        };
        let a4923 = 9223372036854775807u64;
        Ok(nat_le(a4922, a4923))
    } else {
        Ok(false)
    }
}

pub fn f200(v0: Adt14) -> R<u64> {
    let a4954 = {
        let a4946 = {
            let a4938 = {
                let a4930 = 4u64;
                let a4931 = {
                    let a4928 = {
                        let a4925 = match v0.clone() {
                            Adt14::C0(_, h4924, _, _, _, _, _, _, _, _, _) => {
                                h4924
                            }
                        };
                        let a4926 = 1u64;
                        nat_sub(a4925, a4926)
                    };
                    let a4929 = match v0.clone() {
                        Adt14::C0(_, _, _, _, _, h4927, _, _, _, _, _) => {
                            h4927
                        }
                    };
                    nat_mul(a4928, a4929)?
                };
                nat_add(a4930, a4931)?
            };
            let a4939 = {
                let a4936 = {
                    let a4933 = match v0.clone() {
                        Adt14::C0(_, _, h4932, _, _, _, _, _, _, _, _) => {
                            h4932
                        }
                    };
                    let a4934 = 1u64;
                    nat_sub(a4933, a4934)
                };
                let a4937 = match v0.clone() {
                    Adt14::C0(_, _, _, _, _, _, h4935, _, _, _, _) => {
                        h4935
                    }
                };
                nat_mul(a4936, a4937)?
            };
            nat_add(a4938, a4939)?
        };
        let a4947 = {
            let a4944 = {
                let a4941 = match v0.clone() {
                    Adt14::C0(_, _, _, h4940, _, _, _, _, _, _, _) => {
                        h4940
                    }
                };
                let a4942 = 1u64;
                nat_sub(a4941, a4942)
            };
            let a4945 = match v0.clone() {
                Adt14::C0(_, _, _, _, _, _, _, h4943, _, _, _) => {
                    h4943
                }
            };
            nat_mul(a4944, a4945)?
        };
        nat_add(a4946, a4947)?
    };
    let a4955 = {
        let a4952 = {
            let a4949 = match v0.clone() {
                Adt14::C0(_, _, _, _, h4948, _, _, _, _, _, _) => {
                    h4948
                }
            };
            let a4950 = 1u64;
            nat_sub(a4949, a4950)
        };
        let a4953 = match v0 {
            Adt14::C0(_, _, _, _, _, _, _, _, h4951, _, _) => {
                h4951
            }
        };
        nat_mul(a4952, a4953)?
    };
    nat_add(a4954, a4955)
}

pub fn f201(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64) -> R<u64> {
    let a4956 = v0;
    let a4957 = 0u64;
    if if if if nat_eq(a4956, a4957) {
        true
    } else {
        let a4958 = v1;
        let a4959 = 0u64;
        nat_eq(a4958, a4959)
    } {
        true
    } else {
        let a4960 = v2;
        let a4961 = 0u64;
        nat_eq(a4960, a4961)
    } {
        true
    } else {
        let a4962 = v3;
        let a4963 = 0u64;
        nat_eq(a4962, a4963)
    } {
        Ok(0u64)
    } else {
        let a4964 = f240(v0, v1, v2, v3, v5)?;
        let a4965 = f241(v0, v1, v2, v3, v4)?;
        nat_add(a4964, a4965)
    }
}

pub fn f202(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let a4966 = v0;
    let a4967 = 0u64;
    if if nat_eq(a4966, a4967) {
        true
    } else {
        let a4968 = v1;
        let a4969 = 0u64;
        nat_eq(a4968, a4969)
    } {
        Ok(0u64)
    } else {
        let a4972 = f108(v0, v1, v2)?;
        let a4973 = {
            let a4970 = v1;
            let a4971 = f242(v0)?;
            nat_mul(a4970, a4971)?
        };
        nat_add(a4972, a4973)
    }
}

pub fn f203(v0: u64, v1: u64, v2: u64) -> bool {
    let a4974 = v0;
    let a4975 = 1u64;
    if nat_le(a4974, a4975) {
        true
    } else {
        let a4979 = {
            let a4976 = v1;
            let a4977 = v2;
            let a4978 = 1u64;
            nat_rem(a4976, a4977, a4978)
        };
        let a4980 = 0u64;
        nat_eq(a4979, a4980)
    }
}

pub fn f204(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64) -> R<bool> {
    let a4981 = v1;
    let a4982 = 1u64;
    if if if nat_le(a4981, a4982) {
        true
    } else {
        let a4983 = v0;
        let a4984 = v2;
        nat_le(a4983, a4984)
    } {
        let a4985 = v3;
        let a4986 = 1u64;
        if nat_le(a4985, a4986) {
            true
        } else {
            let a4993 = {
                let a4991 = v0;
                let a4992 = {
                    let a4989 = {
                        let a4987 = v1;
                        let a4988 = 1u64;
                        nat_sub(a4987, a4988)
                    };
                    let a4990 = v2;
                    nat_mul(a4989, a4990)?
                };
                nat_add(a4991, a4992)?
            };
            let a4994 = v4;
            nat_le(a4993, a4994)
        }
    } else {
        false
    } {
        let a4995 = v5;
        let a4996 = 1u64;
        if nat_le(a4995, a4996) {
            Ok(true)
        } else {
            let a5009 = {
                let a5007 = {
                    let a5001 = v0;
                    let a5002 = {
                        let a4999 = {
                            let a4997 = v1;
                            let a4998 = 1u64;
                            nat_sub(a4997, a4998)
                        };
                        let a5000 = v2;
                        nat_mul(a4999, a5000)?
                    };
                    nat_add(a5001, a5002)?
                };
                let a5008 = {
                    let a5005 = {
                        let a5003 = v3;
                        let a5004 = 1u64;
                        nat_sub(a5003, a5004)
                    };
                    let a5006 = v4;
                    nat_mul(a5005, a5006)?
                };
                nat_add(a5007, a5008)?
            };
            let a5010 = v6;
            Ok(nat_le(a5009, a5010))
        }
    } else {
        Ok(false)
    }
}

pub fn f205(v0: u64, v1: u64, _: u64, v3: u64, v4: u64, v5: u64, v6: u64, v7: u64, v8: u64, v9: u64, v10: u64, v11: u64, v12: u64, v13: u64, v14: u64) -> bool {
    if if if if if if if if if f243(v0, v1, v6, v7, v8) {
        let a5011 = v5;
        let a5012 = 26u64;
        nat_eq(a5011, a5012)
    } else {
        false
    } {
        let a5013 = v3;
        let a5014 = v7;
        nat_eq(a5013, a5014)
    } else {
        false
    } {
        let a5015 = v4;
        let a5016 = v8;
        nat_eq(a5015, a5016)
    } else {
        false
    } {
        let a5017 = v9;
        let a5018 = 1u64;
        nat_eq(a5017, a5018)
    } else {
        false
    } {
        let a5019 = v10;
        let a5020 = f244(v0);
        nat_eq(a5019, a5020)
    } else {
        false
    } {
        let a5021 = v11;
        let a5022 = v1;
        nat_eq(a5021, a5022)
    } else {
        false
    } {
        let a5023 = v12;
        let a5024 = v6;
        nat_eq(a5023, a5024)
    } else {
        false
    } {
        let a5025 = v13;
        let a5026 = v7;
        nat_eq(a5025, a5026)
    } else {
        false
    } {
        let a5027 = v14;
        let a5028 = v8;
        nat_eq(a5027, a5028)
    } else {
        false
    }
}

pub fn f206(v0: u64, v1: u64, v2: u64, v3: u64) -> bool {
    let a5029 = v0;
    let a5030 = 0u64;
    if if if nat_eq(a5029, a5030) {
        true
    } else {
        let a5031 = v1;
        let a5032 = 0u64;
        nat_eq(a5031, a5032)
    } {
        true
    } else {
        let a5033 = v2;
        let a5034 = 0u64;
        nat_eq(a5033, a5034)
    } {
        true
    } else {
        let a5035 = v3;
        let a5036 = 0u64;
        nat_eq(a5035, a5036)
    }
}

pub fn f207(v0: u64) -> u64 {
    let m5037 = f127(v0);
    match m5037 {
        Adt17::C0 => {
            0u64
        }
        Adt17::C1(v1) => {
            match v1 {
                Adt18::C0(_, h5038, _) => {
                    h5038
                }
            }
        }
    }
}

pub fn f208(v0: Adt3, v1: bool) -> R<u64> {
    f245(v0.clone(), {
        let a5040 = match v0.clone() {
            Adt3::C0(h5039, _, _, _, _, _, _, _) => {
                h5039
            }
        };
        let a5041 = 1u64;
        nat_sub(a5040, a5041)
    }, {
        let a5043 = match v0.clone() {
            Adt3::C0(_, h5042, _, _, _, _, _, _) => {
                h5042
            }
        };
        let a5044 = 1u64;
        nat_sub(a5043, a5044)
    }, {
        let a5046 = match v0.clone() {
            Adt3::C0(_, _, h5045, _, _, _, _, _) => {
                h5045
            }
        };
        let a5047 = 1u64;
        nat_sub(a5046, a5047)
    }, {
        let a5049 = match v0 {
            Adt3::C0(_, _, _, h5048, _, _, _, _) => {
                h5048
            }
        };
        let a5050 = 1u64;
        nat_sub(a5049, a5050)
    }, v1)
}

pub fn f209(v0: u64, v1: u64, v2: u64, v3: u64) -> R<u64> {
    let a5051 = v0;
    let a5052 = 0u64;
    if nat_eq(a5051, a5052) {
        Ok(0u64)
    } else {
        let v4: u64 = {
            let a5053 = v0;
            let a5054 = 1u64;
            nat_sub(a5053, a5054)
        };
        let v5: u64 = {
            let a5055 = v4;
            let a5056 = v1;
            let a5057 = 0u64;
            nat_quot(a5055, a5056, a5057)
        };
        let v6: u64 = {
            let a5065 = {
                let a5058 = v5;
                let a5059 = v2;
                nat_mul(a5058, a5059)?
            };
            let a5066 = {
                let a5063 = {
                    let a5060 = v4;
                    let a5061 = v1;
                    let a5062 = 0u64;
                    nat_rem(a5060, a5061, a5062)
                };
                let a5064 = v3;
                nat_mul(a5063, a5064)?
            };
            nat_add(a5065, a5066)?
        };
        let a5067 = v5;
        let a5068 = 0u64;
        if nat_eq(a5067, a5068) {
            Ok(v6)
        } else {
            let v7: u64 = {
                let a5077 = {
                    let a5071 = {
                        let a5069 = v5;
                        let a5070 = 1u64;
                        nat_sub(a5069, a5070)
                    };
                    let a5072 = v2;
                    nat_mul(a5071, a5072)?
                };
                let a5078 = {
                    let a5075 = {
                        let a5073 = v1;
                        let a5074 = 1u64;
                        nat_sub(a5073, a5074)
                    };
                    let a5076 = v3;
                    nat_mul(a5075, a5076)?
                };
                nat_add(a5077, a5078)?
            };
            let a5079 = v6;
            let a5080 = v7;
            if nat_lt(a5079, a5080) {
                Ok(v7)
            } else {
                Ok(v6)
            }
        }
    }
}

pub fn f210(v0: Adt3, v1: u64) -> bool {
    let a5081 = v1;
    let a5082 = 0u64;
    if !nat_eq(a5081, a5082) {
        true
    } else {
        let a5084 = match v0 {
            Adt3::C0(_, _, _, _, h5083, _, _, _) => {
                h5083
            }
        };
        let a5085 = 4u64;
        nat_eq(a5084, a5085)
    }
}

pub fn f211(v0: u64, v1: bool) -> R<u64> {
    let a5088 = {
        let a5086 = v0;
        let a5087 = 2u64;
        nat_mul(a5086, a5087)?
    };
    let a5089 = if v1 {
        4u64
    } else {
        2u64
    };
    nat_add(a5088, a5089)
}

pub fn f212(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: bool, v6: u64, v7: u64, v8: u64, v9: u64, v10: u64, v11: u64, v12: u64, v13: u64, v14: u64, v15: u64, v16: u64, v17: u64) -> R<Adt10> {
    let m5090 = f246(v0, v1);
    match m5090 {
        None => {
            if v5 {
                let m5091 = f247(v0, v2, v3, v4, v6, v7, v8, v9, v10, v11, v12, v13, v14, v15, v16, v17)?;
                match m5091 {
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
                let a5092 = 4u64;
                let a5093 = v28;
                nat_le(a5092, a5093)
            };
            let a5094 = v2;
            let a5095 = 0u64;
            if if if if if nat_eq(a5094, a5095) {
                let a5096 = v4;
                let a5097 = 0u64;
                nat_eq(a5096, a5097)
            } else {
                false
            } {
                let a5098 = v6;
                let a5099 = v14;
                if nat_eq(a5098, a5099) {
                    let a5100 = v7;
                    let a5101 = v15;
                    nat_eq(a5100, a5101)
                } else {
                    false
                }
            } else {
                false
            } {
                let a5102 = v8;
                let a5103 = v16;
                if nat_eq(a5102, a5103) {
                    let a5104 = v9;
                    let a5105 = v17;
                    nat_eq(a5104, a5105)
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
                        let a5106 = v3;
                        let a5107 = 0u64;
                        nat_eq(a5106, a5107)
                    } else {
                        false
                    } {
                        f130(v6, v7, v8, v9, v10, v11, v12, v13)
                    } else {
                        false
                    }
                }
            } else {
                false
            } {
                Ok(Adt10::C2(v28, v29, v6, v7, v8, v9, {
                    let a5112 = {
                        let a5108 = v6;
                        let a5109 = v7;
                        nat_mul(a5108, a5109)?
                    };
                    let a5113 = {
                        let a5110 = v8;
                        let a5111 = v9;
                        nat_mul(a5110, a5111)?
                    };
                    nat_mul(a5112, a5113)?
                }))
            } else {
                Ok(Adt10::C1)
            }
        }
    }
}

pub fn f213(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let a5118 = {
        let a5116 = {
            let a5114 = v0;
            let a5115 = v1;
            nat_sub(a5114, a5115)
        };
        let a5117 = 1u64;
        nat_sub(a5116, a5117)
    };
    let a5119 = v2;
    nat_add(a5118, a5119)
}

pub fn f214(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64, v7: u64, v8: u64, v9: u64, v10: u64, v11: u64) -> R<u64> {
    f144({
        let a5120 = v0;
        let a5121 = v4;
        let a5122 = 0u64;
        nat_rem(a5120, a5121, a5122)
    }, {
        let a5123 = v1;
        let a5124 = v5;
        let a5125 = 0u64;
        nat_rem(a5123, a5124, a5125)
    }, {
        let a5126 = v2;
        let a5127 = v6;
        let a5128 = 0u64;
        nat_rem(a5126, a5127, a5128)
    }, {
        let a5129 = v3;
        let a5130 = v7;
        let a5131 = 0u64;
        nat_rem(a5129, a5130, a5131)
    }, v8, v9, v10, v11)
}

pub fn f215(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let a5132 = v1;
    let a5133 = 2147483648u64;
    if nat_lt(a5132, a5133) {
        let a5144 = {
            let a5142 = {
                let a5137 = {
                    let a5134 = v0;
                    let a5135 = v2;
                    let a5136 = 0u64;
                    nat_rem(a5134, a5135, a5136)
                };
                let a5138 = v2;
                nat_add(a5137, a5138)?
            };
            let a5143 = {
                let a5139 = v1;
                let a5140 = v2;
                let a5141 = 0u64;
                nat_rem(a5139, a5140, a5141)
            };
            nat_sub(a5142, a5143)
        };
        let a5145 = v2;
        let a5146 = 0u64;
        Ok(nat_rem(a5144, a5145, a5146))
    } else {
        let a5149 = {
            let a5147 = v0;
            let a5148 = f145(v1);
            nat_add(a5147, a5148)?
        };
        let a5150 = v2;
        let a5151 = 0u64;
        Ok(nat_rem(a5149, a5150, a5151))
    }
}

pub fn f216(v0: u64, v1: u64, v2: u64) -> R<bool> {
    let a5154 = f146(v1);
    let a5155 = {
        let a5152 = v0;
        let a5153 = f145(v1);
        nat_add(a5152, a5153)?
    };
    if nat_le(a5154, a5155) {
        let a5156 = f217(v0, v1)?;
        let a5157 = v2;
        Ok(nat_lt(a5156, a5157))
    } else {
        Ok(false)
    }
}

pub fn f217(v0: u64, v1: u64) -> R<u64> {
    let a5160 = {
        let a5158 = v0;
        let a5159 = f145(v1);
        nat_add(a5158, a5159)?
    };
    let a5161 = f146(v1);
    Ok(nat_sub(a5160, a5161))
}

pub fn f218(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let a5162 = v0;
    let a5163 = v1;
    if nat_lt(a5162, a5163) {
        let a5164 = v1;
        let a5165 = v0;
        Ok(nat_sub(a5164, a5165))
    } else {
        let a5168 = v0;
        let a5169 = {
            let a5166 = v1;
            let a5167 = v2;
            nat_add(a5166, a5167)?
        };
        if nat_lt(a5168, a5169) {
            let a5170 = v0;
            let a5171 = v1;
            Ok(nat_sub(a5170, a5171))
        } else {
            let a5178 = {
                let a5174 = 2u64;
                let a5175 = {
                    let a5172 = v2;
                    let a5173 = 1u64;
                    nat_sub(a5172, a5173)
                };
                nat_mul(a5174, a5175)?
            };
            let a5179 = {
                let a5176 = v0;
                let a5177 = v1;
                nat_sub(a5176, a5177)
            };
            Ok(nat_sub(a5178, a5179))
        }
    }
}

pub fn f219(v0: u64, v1: Adt3, v2: u64) -> R<u64> {
    let a5182 = {
        let a5180 = v2;
        let a5181 = f245(v1.clone(), f74(v0, v1.clone(), 0u64), f74(v0, v1.clone(), 1u64), f74(v0, v1.clone(), 2u64), f74(v0, v1.clone(), 3u64), false)?;
        nat_add(a5180, a5181)?
    };
    let a5183 = f245(v1.clone(), f74(v0, v1.clone(), 0u64), f74(v0, v1.clone(), 1u64), f74(v0, v1.clone(), 2u64), f74(v0, v1, 3u64), true)?;
    Ok(nat_sub(a5182, a5183))
}

pub fn f220(v0: u64, v1: u64, v2: u64) -> bool {
    let a5184 = v0;
    let a5185 = 0u64;
    if nat_eq(a5184, a5185) {
        let a5186 = v2;
        let a5187 = v1;
        nat_le(a5186, a5187)
    } else {
        let a5188 = v0;
        let a5189 = 1u64;
        if nat_eq(a5188, a5189) {
            let a5190 = v2;
            let a5191 = v1;
            nat_lt(a5190, a5191)
        } else {
            let a5192 = v0;
            let a5193 = 2u64;
            if nat_eq(a5192, a5193) {
                let a5194 = v1;
                let a5195 = v2;
                nat_le(a5194, a5195)
            } else {
                let a5196 = v0;
                let a5197 = 3u64;
                if nat_eq(a5196, a5197) {
                    let a5198 = v1;
                    let a5199 = v2;
                    nat_lt(a5198, a5199)
                } else {
                    let a5200 = v1;
                    let a5201 = v2;
                    nat_eq(a5200, a5201)
                }
            }
        }
    }
}

pub fn f221(v0: u64, v1: Adt3, v2: u64) -> R<Adt9> {
    let a5202 = v0;
    let a5203 = 0u64;
    if nat_eq(a5202, a5203) {
        Ok(f232(0u64))
    } else {
        let a5206 = {
            let a5204 = v0;
            let a5205 = 1u64;
            nat_add(a5204, a5205)?
        };
        let a5207 = f141(v1.clone(), 13u64, v2)?;
        if nat_eq(a5206, a5207) {
            Ok(Adt9::C4({
                let a5208 = v2;
                let a5209 = 0u64;
                if nat_eq(a5208, a5209) {
                    2u64
                } else {
                    7u64
                }
            }, 1u64, 0u64, 2u64, 0u64, 3u64, 0u64))
        } else {
            let v3: u64 = {
                let a5210 = v0;
                let a5211 = 1u64;
                nat_sub(a5210, a5211)
            };
            if f210(v1.clone(), v2) {
                let v4: u64 = f211(match v1.clone() {
                    Adt3::C0(h5212, _, _, _, _, _, _, _) => {
                        h5212
                    }
                }, false)?;
                let v5: u64 = {
                    let a5213 = v3;
                    let a5214 = v4;
                    let a5215 = 0u64;
                    nat_quot(a5213, a5214, a5215)
                };
                let v6: u64 = {
                    let a5216 = v3;
                    let a5217 = v4;
                    let a5218 = 0u64;
                    nat_rem(a5216, a5217, a5218)
                };
                let a5221 = {
                    let a5219 = v6;
                    let a5220 = 1u64;
                    nat_add(a5219, a5220)?
                };
                let a5222 = v4;
                if nat_eq(a5221, a5222) {
                    Ok(Adt9::C4({
                        let a5223 = v2;
                        let a5224 = 0u64;
                        if nat_eq(a5223, a5224) {
                            0u64
                        } else {
                            4u64
                        }
                    }, 1u64, 0u64, 1u64, 1u64, 1u64, 0u64))
                } else {
                    f222(v6, v5, v1, v2)
                }
            } else {
                f248({
                    let a5225 = v3;
                    let a5226 = 2u64;
                    let a5227 = 0u64;
                    nat_quot(a5225, a5226, a5227)
                }, {
                    let a5228 = v3;
                    let a5229 = 2u64;
                    let a5230 = 0u64;
                    nat_rem(a5228, a5229, a5230)
                }, v1, v2, 0u64)
            }
        }
    }
}

pub fn f222(v0: u64, v1: u64, v2: Adt3, v3: u64) -> R<Adt9> {
    let a5231 = v0;
    let a5232 = 0u64;
    if nat_eq(a5231, a5232) {
        Ok(f232(1u64))
    } else {
        f248({
            let a5241 = {
                let a5234 = v1;
                let a5235 = match v2.clone() {
                    Adt3::C0(h5233, _, _, _, _, _, _, _) => {
                        h5233
                    }
                };
                nat_mul(a5234, a5235)?
            };
            let a5242 = {
                let a5238 = {
                    let a5236 = v0;
                    let a5237 = 1u64;
                    nat_sub(a5236, a5237)
                };
                let a5239 = 2u64;
                let a5240 = 0u64;
                nat_quot(a5238, a5239, a5240)
            };
            nat_add(a5241, a5242)?
        }, {
            let a5245 = {
                let a5243 = v0;
                let a5244 = 1u64;
                nat_sub(a5243, a5244)
            };
            let a5246 = 2u64;
            let a5247 = 0u64;
            nat_rem(a5245, a5246, a5247)
        }, v2, v3, 1u64)
    }
}

pub fn f223(v0: u64, v1: u64) -> R<u64> {
    let a5248 = v0;
    let a5249 = f102(v1)?;
    let a5250 = 0u64;
    Ok(nat_rem(a5248, a5249, a5250))
}

pub fn f224(v0: u64, v1: u64) -> R<u64> {
    let a5251 = 4u64;
    let a5252 = f249(v0, v1)?;
    nat_mul(a5251, a5252)
}

pub fn f225(v0: u64, v1: u64) -> R<u64> {
    let a5253 = 4u64;
    let a5254 = f250(v0, v1)?;
    nat_mul(a5253, a5254)
}

pub fn f226(v0: u64, v1: u64) -> R<bool> {
    let a5260 = {
        let a5257 = {
            let a5255 = f223(v0, v1)?;
            let a5256 = 2u64;
            nat_sub(a5255, a5256)
        };
        let a5258 = 2u64;
        let a5259 = 0u64;
        nat_rem(a5257, a5258, a5259)
    };
    let a5261 = 0u64;
    Ok(nat_eq(a5260, a5261))
}

pub fn f227(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64, v7: u64, v8: u64, v9: u64, v10: u64, v11: u64, v12: u64) -> R<Option<u64>> {
    let m5262 = f127(v0);
    match m5262 {
        Adt17::C0 => {
            Ok(None::<u64>)
        }
        Adt17::C1(v13) => {
            if if if if if f125(v0) {
                let a5263 = v1;
                let a5264 = v7;
                nat_lt(a5263, a5264)
            } else {
                false
            } {
                f175(v8, v2)
            } else {
                false
            } {
                let a5265 = v3;
                let a5266 = f251(v4, v5, v6)?;
                nat_lt(a5265, a5266)
            } else {
                false
            } {
                let a5271 = {
                    let a5268 = v7;
                    let a5269 = match v13.clone() {
                        Adt18::C0(_, h5267, _) => {
                            h5267
                        }
                    };
                    let a5270 = 0u64;
                    nat_rem(a5268, a5269, a5270)
                };
                let a5272 = 0u64;
                nat_eq(a5271, a5272)
            } else {
                false
            } {
                Ok(Some({
                    let a5301 = {
                        let a5291 = {
                            let a5281 = {
                                let a5277 = {
                                    let a5274 = v1;
                                    let a5275 = match v13 {
                                        Adt18::C0(_, h5273, _) => {
                                            h5273
                                        }
                                    };
                                    let a5276 = 0u64;
                                    nat_quot(a5274, a5275, a5276)
                                };
                                let a5278 = v9;
                                nat_mul(a5277, a5278)?
                            };
                            let a5282 = {
                                let a5279 = v2;
                                let a5280 = v10;
                                nat_mul(a5279, a5280)?
                            };
                            nat_add(a5281, a5282)?
                        };
                        let a5292 = {
                            let a5289 = {
                                let a5286 = {
                                    let a5283 = v3;
                                    let a5284 = v4;
                                    let a5285 = 0u64;
                                    nat_quot(a5283, a5284, a5285)
                                };
                                let a5287 = v5;
                                let a5288 = 0u64;
                                nat_rem(a5286, a5287, a5288)
                            };
                            let a5290 = v11;
                            nat_mul(a5289, a5290)?
                        };
                        nat_add(a5291, a5292)?
                    };
                    let a5302 = {
                        let a5299 = {
                            let a5296 = {
                                let a5293 = v3;
                                let a5294 = v4;
                                let a5295 = 0u64;
                                nat_quot(a5293, a5294, a5295)
                            };
                            let a5297 = v5;
                            let a5298 = 0u64;
                            nat_quot(a5296, a5297, a5298)
                        };
                        let a5300 = v12;
                        nat_mul(a5299, a5300)?
                    };
                    nat_add(a5301, a5302)?
                }))
            } else {
                Ok(None::<u64>)
            }
        }
    }
}

pub fn f228(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64, v7: u64, v8: u64, v9: u64, v10: u64, v11: u64, v12: u64, v13: u64, v14: u64) -> R<Adt19> {
    f252(v0, Adt20::C0(v1, v2, v3, v4, v5, v6, v7, v8), Adt20::C0(v1, v2, v3, v4, v9, v10, v11, v12), v13, v14)
}

pub fn f229(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64, v7: u64, v8: u64, v9: u64, v10: u64, v11: u64, v12: u64, v13: u64, v14: u64, v15: u64, v16: u64, v17: u64, v18: u64, v19: u64, v20: u64) -> R<Adt19> {
    f253(v0, Adt20::C0(v1, v2, v3, v4, v5, v6, v7, v8), Adt20::C0(v1, v2, v3, v4, v9, v10, v11, v12), Adt20::C0(v1, v2, v3, v4, v13, v14, v15, v16), v17, v18, v19, v20)
}

pub fn f230(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64, v7: u64, v8: u64, v9: u64, v10: u64, v11: u64, v12: u64, v13: u64, v14: u64, v15: u64, v16: u64, v17: u64) -> R<Adt19> {
    let a5303 = v0;
    let a5304 = 64u64;
    if nat_eq(a5303, a5304) {
        f254(v1, Adt20::C0(v2, v3, v4, v5, v6, v7, v8, v9), Adt20::C0(v2, v3, v4, v5, v10, v11, v12, v13), v14, v15, v16, v17)
    } else {
        let a5305 = v0;
        let a5306 = 65u64;
        if nat_eq(a5305, a5306) {
            f255(v1, Adt20::C0(v2, v3, v4, v5, v6, v7, v8, v9), Adt20::C0(v2, v3, v4, v5, v10, v11, v12, v13), v14, v15, v16, v17)
        } else {
            Ok(Adt19::C0)
        }
    }
}

pub fn f231(v0: u64) -> u64 {
    let a5307 = v0;
    let a5308 = 30u64;
    if nat_eq(a5307, a5308) {
        4u64
    } else {
        3u64
    }
}

pub fn f232(v0: u64) -> Adt9 {
    Adt9::C4(0u64, 2u64, 0u64, 2u64, 0u64, 1u64, v0)
}

pub fn f233(v0: u64) -> bool {
    let a5309 = 0u64;
    let a5310 = v0;
    if nat_lt(a5309, a5310) {
        let a5311 = v0;
        let a5312 = 2139095040u64;
        nat_le(a5311, a5312)
    } else {
        false
    }
}

pub fn f234(v0: u64) -> u64 {
    let a5313 = v0;
    let a5314 = 40u64;
    if nat_lt(a5313, a5314) {
        let a5315 = v0;
        let a5316 = 20u64;
        if nat_lt(a5315, a5316) {
            let a5317 = v0;
            let a5318 = 10u64;
            if nat_lt(a5317, a5318) {
                let a5319 = v0;
                let a5320 = 5u64;
                if nat_lt(a5319, a5320) {
                    let a5321 = v0;
                    let a5322 = 2u64;
                    if nat_lt(a5321, a5322) {
                        let a5323 = v0;
                        let a5324 = 1u64;
                        if nat_lt(a5323, a5324) {
                            1u64
                        } else {
                            2u64
                        }
                    } else {
                        let a5325 = v0;
                        let a5326 = 3u64;
                        if nat_lt(a5325, a5326) {
                            4u64
                        } else {
                            let a5327 = v0;
                            let a5328 = 4u64;
                            if nat_lt(a5327, a5328) {
                                8u64
                            } else {
                                16u64
                            }
                        }
                    }
                } else {
                    let a5329 = v0;
                    let a5330 = 7u64;
                    if nat_lt(a5329, a5330) {
                        let a5331 = v0;
                        let a5332 = 6u64;
                        if nat_lt(a5331, a5332) {
                            32u64
                        } else {
                            64u64
                        }
                    } else {
                        let a5333 = v0;
                        let a5334 = 8u64;
                        if nat_lt(a5333, a5334) {
                            128u64
                        } else {
                            let a5335 = v0;
                            let a5336 = 9u64;
                            if nat_lt(a5335, a5336) {
                                256u64
                            } else {
                                512u64
                            }
                        }
                    }
                }
            } else {
                let a5337 = v0;
                let a5338 = 15u64;
                if nat_lt(a5337, a5338) {
                    let a5339 = v0;
                    let a5340 = 12u64;
                    if nat_lt(a5339, a5340) {
                        let a5341 = v0;
                        let a5342 = 11u64;
                        if nat_lt(a5341, a5342) {
                            1024u64
                        } else {
                            2048u64
                        }
                    } else {
                        let a5343 = v0;
                        let a5344 = 13u64;
                        if nat_lt(a5343, a5344) {
                            4096u64
                        } else {
                            let a5345 = v0;
                            let a5346 = 14u64;
                            if nat_lt(a5345, a5346) {
                                8192u64
                            } else {
                                16384u64
                            }
                        }
                    }
                } else {
                    let a5347 = v0;
                    let a5348 = 17u64;
                    if nat_lt(a5347, a5348) {
                        let a5349 = v0;
                        let a5350 = 16u64;
                        if nat_lt(a5349, a5350) {
                            32768u64
                        } else {
                            65536u64
                        }
                    } else {
                        let a5351 = v0;
                        let a5352 = 18u64;
                        if nat_lt(a5351, a5352) {
                            131072u64
                        } else {
                            let a5353 = v0;
                            let a5354 = 19u64;
                            if nat_lt(a5353, a5354) {
                                262144u64
                            } else {
                                524288u64
                            }
                        }
                    }
                }
            }
        } else {
            let a5355 = v0;
            let a5356 = 30u64;
            if nat_lt(a5355, a5356) {
                let a5357 = v0;
                let a5358 = 25u64;
                if nat_lt(a5357, a5358) {
                    let a5359 = v0;
                    let a5360 = 22u64;
                    if nat_lt(a5359, a5360) {
                        let a5361 = v0;
                        let a5362 = 21u64;
                        if nat_lt(a5361, a5362) {
                            1048576u64
                        } else {
                            2097152u64
                        }
                    } else {
                        let a5363 = v0;
                        let a5364 = 23u64;
                        if nat_lt(a5363, a5364) {
                            4194304u64
                        } else {
                            let a5365 = v0;
                            let a5366 = 24u64;
                            if nat_lt(a5365, a5366) {
                                8388608u64
                            } else {
                                16777216u64
                            }
                        }
                    }
                } else {
                    let a5367 = v0;
                    let a5368 = 27u64;
                    if nat_lt(a5367, a5368) {
                        let a5369 = v0;
                        let a5370 = 26u64;
                        if nat_lt(a5369, a5370) {
                            33554432u64
                        } else {
                            67108864u64
                        }
                    } else {
                        let a5371 = v0;
                        let a5372 = 28u64;
                        if nat_lt(a5371, a5372) {
                            134217728u64
                        } else {
                            let a5373 = v0;
                            let a5374 = 29u64;
                            if nat_lt(a5373, a5374) {
                                268435456u64
                            } else {
                                536870912u64
                            }
                        }
                    }
                }
            } else {
                let a5375 = v0;
                let a5376 = 35u64;
                if nat_lt(a5375, a5376) {
                    let a5377 = v0;
                    let a5378 = 32u64;
                    if nat_lt(a5377, a5378) {
                        let a5379 = v0;
                        let a5380 = 31u64;
                        if nat_lt(a5379, a5380) {
                            1073741824u64
                        } else {
                            2147483648u64
                        }
                    } else {
                        let a5381 = v0;
                        let a5382 = 33u64;
                        if nat_lt(a5381, a5382) {
                            4294967296u64
                        } else {
                            let a5383 = v0;
                            let a5384 = 34u64;
                            if nat_lt(a5383, a5384) {
                                8589934592u64
                            } else {
                                17179869184u64
                            }
                        }
                    }
                } else {
                    let a5385 = v0;
                    let a5386 = 37u64;
                    if nat_lt(a5385, a5386) {
                        let a5387 = v0;
                        let a5388 = 36u64;
                        if nat_lt(a5387, a5388) {
                            34359738368u64
                        } else {
                            68719476736u64
                        }
                    } else {
                        let a5389 = v0;
                        let a5390 = 38u64;
                        if nat_lt(a5389, a5390) {
                            137438953472u64
                        } else {
                            let a5391 = v0;
                            let a5392 = 39u64;
                            if nat_lt(a5391, a5392) {
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

pub fn f235(v0: u64, v1: u64) -> bool {
    let a5393 = v0;
    let a5394 = 1u64;
    if nat_le(a5393, a5394) {
        true
    } else {
        let a5398 = {
            let a5395 = v1;
            let a5396 = 4u64;
            let a5397 = 0u64;
            nat_rem(a5395, a5396, a5397)
        };
        let a5399 = 0u64;
        nat_eq(a5398, a5399)
    }
}

pub fn f236(v0: u64, v1: u64) -> bool {
    let a5400 = v0;
    let a5401 = 1u64;
    if nat_le(a5400, a5401) {
        true
    } else {
        let a5402 = 4u64;
        let a5403 = v1;
        nat_le(a5402, a5403)
    }
}

pub fn f237(v0: u64, v1: u64) -> R<u64> {
    let a5404 = v0;
    let a5405 = 0u64;
    if nat_eq(a5404, a5405) {
        Ok(0u64)
    } else {
        let a5410 = 4u64;
        let a5411 = {
            let a5408 = {
                let a5406 = v0;
                let a5407 = 1u64;
                nat_sub(a5406, a5407)
            };
            let a5409 = v1;
            nat_mul(a5408, a5409)?
        };
        nat_add(a5410, a5411)
    }
}

pub fn f238(v0: bool, v1: u64) -> R<u64> {
    let a5416 = {
        let a5414 = {
            let a5412 = f256(v0, v1)?;
            let a5413 = f257(v0, v1)?;
            nat_add(a5412, a5413)?
        };
        let a5415 = f258(v0);
        nat_add(a5414, a5415)?
    };
    let a5417 = v1;
    nat_add(a5416, a5417)
}

pub fn f239(v0: u64, v1: u64) -> R<u64> {
    let v2: u64 = f259(v0, v1)?;
    let v3: u64 = {
        let a5422 = {
            let a5420 = {
                let a5418 = v1;
                let a5419 = 1u64;
                nat_sub(a5418, a5419)
            };
            let a5421 = v2;
            nat_mul(a5420, a5421)?
        };
        let a5423 = v0;
        nat_sub(a5422, a5423)
    };
    let a5424 = v2;
    let a5425 = v3;
    if nat_lt(a5424, a5425) {
        Ok(v3)
    } else {
        Ok(v2)
    }
}

pub fn f240(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64) -> R<u64> {
    let a5426 = v4;
    let a5427 = 0u64;
    if nat_eq(a5426, a5427) {
        Ok(0u64)
    } else {
        let a5434 = 2u64;
        let a5435 = {
            let a5432 = {
                let a5428 = v0;
                let a5429 = v1;
                nat_mul(a5428, a5429)?
            };
            let a5433 = {
                let a5430 = v2;
                let a5431 = v3;
                nat_mul(a5430, a5431)?
            };
            nat_mul(a5432, a5433)?
        };
        nat_mul(a5434, a5435)
    }
}

pub fn f241(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64) -> R<u64> {
    let a5440 = v3;
    let a5441 = {
        let a5438 = {
            let a5436 = f260(v2, v4)?;
            let a5437 = f261(v0, v1, f259(v2, v4)?)?;
            nat_mul(a5436, a5437)?
        };
        let a5439 = f261(v0, v1, f262(v2, v4)?)?;
        nat_add(a5438, a5439)?
    };
    nat_mul(a5440, a5441)
}

pub fn f242(v0: u64) -> R<u64> {
    let a5444 = 13u64;
    let a5445 = {
        let a5442 = 9u64;
        let a5443 = v0;
        nat_mul(a5442, a5443)?
    };
    nat_add(a5444, a5445)
}

pub fn f243(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64) -> bool {
    if f206(v1, v2, v3, v4) {
        f263(v0)
    } else {
        f125(v0)
    }
}

pub fn f244(v0: u64) -> u64 {
    let a5446 = v0;
    let a5447 = 26u64;
    if nat_eq(a5446, a5447) {
        26u64
    } else {
        0u64
    }
}

pub fn f245(v0: Adt3, v1: u64, v2: u64, v3: u64, v4: u64, v5: bool) -> R<u64> {
    let a5468 = {
        let a5462 = {
            let a5456 = {
                let a5450 = v1;
                let a5451 = if v5 {
                    f145(match v0.clone() {
                        Adt3::C0(_, _, _, _, h5448, _, _, _) => {
                            h5448
                        }
                    })
                } else {
                    f146(match v0.clone() {
                        Adt3::C0(_, _, _, _, h5449, _, _, _) => {
                            h5449
                        }
                    })
                };
                nat_mul(a5450, a5451)?
            };
            let a5457 = {
                let a5454 = v2;
                let a5455 = if v5 {
                    f145(match v0.clone() {
                        Adt3::C0(_, _, _, _, _, h5452, _, _) => {
                            h5452
                        }
                    })
                } else {
                    f146(match v0.clone() {
                        Adt3::C0(_, _, _, _, _, h5453, _, _) => {
                            h5453
                        }
                    })
                };
                nat_mul(a5454, a5455)?
            };
            nat_add(a5456, a5457)?
        };
        let a5463 = {
            let a5460 = v3;
            let a5461 = if v5 {
                f145(match v0.clone() {
                    Adt3::C0(_, _, _, _, _, _, h5458, _) => {
                        h5458
                    }
                })
            } else {
                f146(match v0.clone() {
                    Adt3::C0(_, _, _, _, _, _, h5459, _) => {
                        h5459
                    }
                })
            };
            nat_mul(a5460, a5461)?
        };
        nat_add(a5462, a5463)?
    };
    let a5469 = {
        let a5466 = v4;
        let a5467 = if v5 {
            f145(match v0 {
                Adt3::C0(_, _, _, _, _, _, _, h5464) => {
                    h5464
                }
            })
        } else {
            f146(match v0 {
                Adt3::C0(_, _, _, _, _, _, _, h5465) => {
                    h5465
                }
            })
        };
        nat_mul(a5466, a5467)?
    };
    nat_add(a5468, a5469)
}

pub fn f246(v0: u64, v1: u64) -> Option<u64> {
    let a5470 = v0;
    let a5471 = 2u64;
    if nat_eq(a5470, a5471) {
        Some(0u64)
    } else {
        let a5472 = v0;
        let a5473 = 5u64;
        if nat_eq(a5472, a5473) {
            Some(1u64)
        } else {
            let a5474 = v0;
            let a5475 = 6u64;
            if nat_eq(a5474, a5475) {
                Some(2u64)
            } else {
                let a5476 = v0;
                let a5477 = 7u64;
                if nat_eq(a5476, a5477) {
                    Some(3u64)
                } else {
                    let a5478 = v0;
                    let a5479 = 9u64;
                    if nat_eq(a5478, a5479) {
                        Some(4u64)
                    } else {
                        let a5480 = v0;
                        let a5481 = 8u64;
                        if nat_eq(a5480, a5481) {
                            Some(5u64)
                        } else {
                            let a5482 = v0;
                            let a5483 = 89u64;
                            if nat_eq(a5482, a5483) {
                                let a5484 = v1;
                                let a5485 = 2u64;
                                if nat_eq(a5484, a5485) {
                                    Some(6u64)
                                } else {
                                    let a5486 = v1;
                                    let a5487 = 0u64;
                                    if nat_eq(a5486, a5487) {
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

pub fn f247(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64, v7: u64, v8: u64, v9: u64, v10: u64, v11: u64, v12: u64, v13: u64, v14: u64, v15: u64) -> R<Adt21> {
    let a5488 = v0;
    let a5489 = 30u64;
    if nat_eq(a5488, a5489) {
        let a5490 = v2;
        let a5491 = 0u64;
        if if if nat_eq(a5490, a5491) {
            true
        } else {
            let a5492 = v2;
            let a5493 = 1u64;
            nat_eq(a5492, a5493)
        } {
            let a5494 = v1;
            let a5495 = 0u64;
            if nat_eq(a5494, a5495) {
                true
            } else {
                f264(v1, v4)
            }
        } else {
            false
        } {
            f265(28u64, 0u64, 0u64, v3, v5, v4, v6, v7, v9, v8, v10, v11, v12, v13, v14, v15)
        } else {
            Ok(Adt21::C0)
        }
    } else {
        if if if if f266(v1) {
            f266(v2)
        } else {
            false
        } {
            let a5496 = v1;
            let a5497 = 0u64;
            if if nat_eq(a5496, a5497) {
                true
            } else {
                let a5498 = v2;
                let a5499 = 0u64;
                nat_eq(a5498, a5499)
            } {
                true
            } else {
                let a5500 = v1;
                let a5501 = v2;
                nat_eq(a5500, a5501)
            }
        } else {
            false
        } {
            true
        } else {
            let a5502 = v1;
            let a5503 = 43u64;
            if nat_eq(a5502, a5503) {
                f266(v2)
            } else {
                false
            }
        } {
            f265(v0, 0u64, 0u64, v3, v4, v5, v6, v7, v8, v9, v10, v11, v12, v13, v14, v15)
        } else {
            Ok(Adt21::C0)
        }
    }
}

pub fn f248(v0: u64, v1: u64, v2: Adt3, v3: u64, v4: u64) -> R<Adt9> {
    let a5504 = v1;
    let a5505 = 0u64;
    if nat_eq(a5504, a5505) {
        Ok(Adt9::C4({
            let a5506 = v3;
            let a5507 = 0u64;
            if nat_eq(a5506, a5507) {
                1u64
            } else {
                6u64
            }
        }, 0u64, f152(v0, v2)?, 2u64, 0u64, 1u64, 2u64))
    } else {
        Ok(Adt9::C4({
            let a5508 = v3;
            let a5509 = 0u64;
            if nat_eq(a5508, a5509) {
                0u64
            } else {
                4u64
            }
        }, 1u64, v4, 1u64, 2u64, 1u64, v4))
    }
}

pub fn f249(v0: u64, v1: u64) -> R<u64> {
    let a5510 = v0;
    let a5511 = f102(v1)?;
    let a5512 = 0u64;
    Ok(nat_quot(a5510, a5511, a5512))
}

pub fn f250(v0: u64, v1: u64) -> R<u64> {
    let v2: u64 = f249(v0, v1)?;
    let v3: u64 = {
        let a5515 = {
            let a5513 = f223(v0, v1)?;
            let a5514 = 2u64;
            nat_sub(a5513, a5514)
        };
        let a5516 = 2u64;
        let a5517 = 0u64;
        nat_quot(a5515, a5516, a5517)
    };
    let v4: u64 = {
        let a5518 = v2;
        let a5519 = v1;
        nat_mul(a5518, a5519)?
    };
    if f226(v0, v1)? {
        let a5520 = v4;
        let a5521 = v3;
        nat_add(a5520, a5521)
    } else {
        let a5533 = {
            let a5529 = v4;
            let a5530 = {
                let a5527 = {
                    let a5522 = v1;
                    let a5523 = 1u64;
                    nat_sub(a5522, a5523)
                };
                let a5528 = {
                    let a5524 = v2;
                    let a5525 = v1;
                    let a5526 = 0u64;
                    nat_rem(a5524, a5525, a5526)
                };
                nat_mul(a5527, a5528)?
            };
            nat_sub(a5529, a5530)
        };
        let a5534 = {
            let a5531 = v3;
            let a5532 = v1;
            nat_mul(a5531, a5532)?
        };
        nat_add(a5533, a5534)
    }
}

pub fn f251(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let a5535 = v0;
    let a5536 = 0u64;
    if if if nat_eq(a5535, a5536) {
        true
    } else {
        let a5537 = v1;
        let a5538 = 0u64;
        nat_eq(a5537, a5538)
    } {
        true
    } else {
        let a5539 = v2;
        let a5540 = 0u64;
        nat_eq(a5539, a5540)
    } {
        Ok(0u64)
    } else {
        let a5543 = {
            let a5541 = v0;
            let a5542 = v1;
            nat_mul(a5541, a5542)?
        };
        let a5544 = v2;
        nat_mul(a5543, a5544)
    }
}

pub fn f252(v0: u64, v1: Adt20, v2: Adt20, v3: u64, v4: u64) -> R<Adt19> {
    let v5: u64 = f111(match v1.clone() {
        Adt20::C0(h5545, _, _, _, _, _, _, _) => {
            h5545
        }
    })?;
    let v6: u64 = {
        let a5551 = match v1.clone() {
            Adt20::C0(_, h5546, _, _, _, _, _, _) => {
                h5546
            }
        };
        let a5552 = {
            let a5549 = match v1.clone() {
                Adt20::C0(_, _, h5547, _, _, _, _, _) => {
                    h5547
                }
            };
            let a5550 = match v1.clone() {
                Adt20::C0(_, _, _, h5548, _, _, _, _) => {
                    h5548
                }
            };
            nat_mul(a5549, a5550)?
        };
        nat_mul(a5551, a5552)?
    };
    let a5554 = v0;
    let a5555 = f39(match v1.clone() {
        Adt20::C0(h5553, _, _, _, _, _, _, _) => {
            h5553
        }
    }, v6)?;
    if nat_lt(a5554, a5555) {
        f267({
            let a5556 = v0;
            let a5557 = v5;
            let a5558 = 0u64;
            nat_rem(a5556, a5557, a5558)
        }, {
            let a5559 = v0;
            let a5560 = v5;
            let a5561 = 0u64;
            nat_quot(a5559, a5560, a5561)
        }, v1, v2, v3, v4)
    } else {
        Ok(Adt19::C0)
    }
}

pub fn f253(v0: u64, v1: Adt20, v2: Adt20, v3: Adt20, v4: u64, v5: u64, v6: u64, v7: u64) -> R<Adt19> {
    let v8: u64 = {
        let a5562 = v6;
        let a5563 = 6u64;
        nat_sub(a5562, a5563)
    };
    let v9: Adt20 = f268(v3);
    let a5565 = v0;
    let a5566 = f202(match v1.clone() {
        Adt20::C0(h5564, _, _, _, _, _, _, _) => {
            h5564
        }
    }, v5, v8)?;
    if nat_lt(a5565, a5566) {
        let v10: u64 = f108(match v1.clone() {
            Adt20::C0(h5567, _, _, _, _, _, _, _) => {
                h5567
            }
        }, v5, v8)?;
        let a5568 = v0;
        let a5569 = v10;
        if nat_lt(a5568, a5569) {
            f269(v0, v9, 4u64, 6u64, v7)
        } else {
            let v11: u64 = {
                let a5570 = v0;
                let a5571 = v10;
                nat_sub(a5570, a5571)
            };
            let v12: u64 = f242(match v1.clone() {
                Adt20::C0(h5572, _, _, _, _, _, _, _) => {
                    h5572
                }
            })?;
            f270({
                let a5573 = v11;
                let a5574 = v12;
                let a5575 = 0u64;
                nat_rem(a5573, a5574, a5575)
            }, {
                let a5576 = v11;
                let a5577 = v12;
                let a5578 = 0u64;
                nat_quot(a5576, a5577, a5578)
            }, v1, v2, v9, v4)
        }
    } else {
        Ok(Adt19::C0)
    }
}

pub fn f254(v0: u64, v1: Adt20, v2: Adt20, v3: u64, v4: u64, v5: u64, v6: u64) -> R<Adt19> {
    let v7: u64 = {
        let a5579 = v5;
        let a5580 = 6u64;
        nat_sub(a5579, a5580)
    };
    let v8: bool = f196(match v1.clone() {
        Adt20::C0(_, _, _, _, h5581, _, _, _) => {
            h5581
        }
    }, match v2.clone() {
        Adt20::C0(_, _, _, _, h5582, _, _, _) => {
            h5582
        }
    });
    let a5584 = v0;
    let a5585 = f195(v8, match v1.clone() {
        Adt20::C0(h5583, _, _, _, _, _, _, _) => {
            h5583
        }
    }, v4, v7)?;
    if nat_lt(a5584, a5585) {
        let v9: u64 = f108(match v1.clone() {
            Adt20::C0(h5586, _, _, _, _, _, _, _) => {
                h5586
            }
        }, v4, v7)?;
        let a5587 = v0;
        let a5588 = v9;
        if nat_lt(a5587, a5588) {
            f269(v0, v2, 4u64, 6u64, v6)
        } else {
            let v10: u64 = {
                let a5589 = v0;
                let a5590 = v9;
                nat_sub(a5589, a5590)
            };
            let v11: u64 = f238(v8, match v1.clone() {
                Adt20::C0(h5591, _, _, _, _, _, _, _) => {
                    h5591
                }
            })?;
            f271({
                let a5592 = v10;
                let a5593 = v11;
                let a5594 = 0u64;
                nat_rem(a5592, a5593, a5594)
            }, {
                let a5595 = v10;
                let a5596 = v11;
                let a5597 = 0u64;
                nat_quot(a5595, a5596, a5597)
            }, v1, v2, v3, v8)
        }
    } else {
        Ok(Adt19::C0)
    }
}

pub fn f255(v0: u64, v1: Adt20, v2: Adt20, v3: u64, v4: u64, v5: u64, v6: u64) -> R<Adt19> {
    let v7: u64 = {
        let a5598 = v5;
        let a5599 = 6u64;
        nat_sub(a5598, a5599)
    };
    let v8: Adt20 = f268(v2);
    let v9: u64 = f240(match v1.clone() {
        Adt20::C0(h5600, _, _, _, _, _, _, _) => {
            h5600
        }
    }, match v1.clone() {
        Adt20::C0(_, h5601, _, _, _, _, _, _) => {
            h5601
        }
    }, match v1.clone() {
        Adt20::C0(_, _, h5602, _, _, _, _, _) => {
            h5602
        }
    }, match v1.clone() {
        Adt20::C0(_, _, _, h5603, _, _, _, _) => {
            h5603
        }
    }, v7)?;
    let a5608 = v0;
    let a5609 = f201(match v1.clone() {
        Adt20::C0(h5604, _, _, _, _, _, _, _) => {
            h5604
        }
    }, match v1.clone() {
        Adt20::C0(_, h5605, _, _, _, _, _, _) => {
            h5605
        }
    }, match v1.clone() {
        Adt20::C0(_, _, h5606, _, _, _, _, _) => {
            h5606
        }
    }, match v1.clone() {
        Adt20::C0(_, _, _, h5607, _, _, _, _) => {
            h5607
        }
    }, v4, v7)?;
    if nat_lt(a5608, a5609) {
        let a5610 = v0;
        let a5611 = v9;
        if nat_lt(a5610, a5611) {
            f269(v0, v8, 4u64, 6u64, v6)
        } else {
            let v10: Adt22 = f272({
                let a5612 = v0;
                let a5613 = v9;
                nat_sub(a5612, a5613)
            }, match v1.clone() {
                Adt20::C0(h5614, _, _, _, _, _, _, _) => {
                    h5614
                }
            }, match v1.clone() {
                Adt20::C0(_, h5615, _, _, _, _, _, _) => {
                    h5615
                }
            }, match v1.clone() {
                Adt20::C0(_, _, h5616, _, _, _, _, _) => {
                    h5616
                }
            }, match v1.clone() {
                Adt20::C0(_, _, _, h5617, _, _, _, _) => {
                    h5617
                }
            }, v4)?;
            f273(v10, v1, v8, v3)
        }
    } else {
        Ok(Adt19::C0)
    }
}

pub fn f256(v0: bool, v1: u64) -> R<u64> {
    let a5620 = if v0 {
        5u64
    } else {
        4u64
    };
    let a5621 = {
        let a5618 = f274(v0);
        let a5619 = v1;
        nat_mul(a5618, a5619)?
    };
    nat_add(a5620, a5621)
}

pub fn f257(v0: bool, v1: u64) -> R<u64> {
    if v0 {
        let a5629 = {
            let a5622 = 13u64;
            let a5623 = f275(v1);
            nat_mul(a5622, a5623)?
        };
        let a5630 = {
            let a5627 = 4u64;
            let a5628 = {
                let a5624 = v1;
                let a5625 = 4u64;
                let a5626 = 0u64;
                nat_rem(a5624, a5625, a5626)
            };
            nat_mul(a5627, a5628)?
        };
        nat_add(a5629, a5630)
    } else {
        let a5631 = 3u64;
        let a5632 = v1;
        nat_mul(a5631, a5632)
    }
}

pub fn f258(v0: bool) -> u64 {
    if v0 {
        6u64
    } else {
        5u64
    }
}

pub fn f259(v0: u64, v1: u64) -> R<u64> {
    let a5637 = {
        let a5635 = {
            let a5633 = v0;
            let a5634 = v1;
            nat_add(a5633, a5634)?
        };
        let a5636 = 1u64;
        nat_sub(a5635, a5636)
    };
    let a5638 = v1;
    let a5639 = 0u64;
    Ok(nat_quot(a5637, a5638, a5639))
}

pub fn f260(v0: u64, v1: u64) -> R<u64> {
    let a5640 = v0;
    let a5641 = f259(v0, v1)?;
    let a5642 = 0u64;
    Ok(nat_quot(a5640, a5641, a5642))
}

pub fn f261(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let a5643 = v2;
    let a5644 = 0u64;
    if nat_eq(a5643, a5644) {
        Ok(0u64)
    } else {
        let a5649 = 11u64;
        let a5650 = {
            let a5647 = {
                let a5645 = v1;
                let a5646 = v2;
                nat_mul(a5645, a5646)?
            };
            let a5648 = f276(v0)?;
            nat_mul(a5647, a5648)?
        };
        nat_add(a5649, a5650)
    }
}

pub fn f262(v0: u64, v1: u64) -> R<u64> {
    let a5651 = v0;
    let a5652 = f259(v0, v1)?;
    let a5653 = 0u64;
    Ok(nat_rem(a5651, a5652, a5653))
}

pub fn f263(v0: u64) -> bool {
    let m5654 = f127(v0);
    match m5654 {
        Adt17::C0 => {
            false
        }
        Adt17::C1(_) => {
            true
        }
    }
}

pub fn f264(v0: u64, v1: u64) -> bool {
    let a5655 = v0;
    let a5656 = 1u64;
    if nat_eq(a5655, a5656) {
        true
    } else {
        let v2: u64 = f277(v0);
        let a5657 = 0u64;
        let a5658 = v2;
        if nat_lt(a5657, a5658) {
            let a5662 = {
                let a5659 = v1;
                let a5660 = v2;
                let a5661 = 1u64;
                nat_rem(a5659, a5660, a5661)
            };
            let a5663 = 0u64;
            nat_eq(a5662, a5663)
        } else {
            false
        }
    }
}

pub fn f265(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64, v7: u64, v8: u64, v9: u64, v10: u64, v11: u64, v12: u64, v13: u64, v14: u64, v15: u64) -> R<Adt21> {
    let a5664 = v0;
    let a5665 = 28u64;
    if if if if nat_eq(a5664, a5665) {
        let a5666 = v1;
        let a5667 = 0u64;
        nat_eq(a5666, a5667)
    } else {
        false
    } {
        let a5668 = v2;
        let a5669 = 0u64;
        nat_eq(a5668, a5669)
    } else {
        false
    } {
        let a5670 = v3;
        let a5671 = 0u64;
        nat_eq(a5670, a5671)
    } else {
        false
    } {
        let a5672 = v4;
        let a5673 = v8;
        if if if if if if if if nat_eq(a5672, a5673) {
            let a5674 = v5;
            let a5675 = v12;
            nat_eq(a5674, a5675)
        } else {
            false
        } {
            let a5676 = v9;
            let a5677 = v13;
            nat_eq(a5676, a5677)
        } else {
            false
        } {
            let a5678 = v10;
            let a5679 = v14;
            nat_eq(a5678, a5679)
        } else {
            false
        } {
            let a5680 = v11;
            let a5681 = v15;
            nat_eq(a5680, a5681)
        } else {
            false
        } {
            let a5682 = 0u64;
            let a5683 = v6;
            nat_lt(a5682, a5683)
        } else {
            false
        } {
            let a5684 = 0u64;
            let a5685 = v7;
            nat_lt(a5684, a5685)
        } else {
            false
        } {
            let a5689 = {
                let a5686 = v10;
                let a5687 = v6;
                let a5688 = 1u64;
                nat_rem(a5686, a5687, a5688)
            };
            let a5690 = 0u64;
            if nat_eq(a5689, a5690) {
                let a5694 = {
                    let a5691 = v11;
                    let a5692 = v7;
                    let a5693 = 1u64;
                    nat_rem(a5691, a5692, a5693)
                };
                let a5695 = 0u64;
                nat_eq(a5694, a5695)
            } else {
                false
            }
        } else {
            false
        } {
            Ok(Adt21::C2(v5, v4, v9, v10, v11, {
                let a5696 = v10;
                let a5697 = v6;
                let a5698 = 0u64;
                nat_quot(a5696, a5697, a5698)
            }, {
                let a5699 = v11;
                let a5700 = v7;
                let a5701 = 0u64;
                nat_quot(a5699, a5700, a5701)
            }, {
                let a5702 = v5;
                let a5703 = v4;
                nat_mul(a5702, a5703)?
            }, {
                let a5704 = v4;
                let a5705 = v9;
                nat_mul(a5704, a5705)?
            }, {
                let a5706 = v5;
                let a5707 = v9;
                nat_mul(a5706, a5707)?
            }))
        } else {
            Ok(Adt21::C1)
        }
    } else {
        Ok(Adt21::C0)
    }
}

pub fn f266(v0: u64) -> bool {
    let a5708 = v0;
    let a5709 = 0u64;
    if if nat_eq(a5708, a5709) {
        true
    } else {
        let a5710 = v0;
        let a5711 = 1u64;
        nat_eq(a5710, a5711)
    } {
        true
    } else {
        let a5712 = v0;
        let a5713 = 30u64;
        nat_eq(a5712, a5713)
    }
}

pub fn f267(v0: u64, v1: u64, v2: Adt20, v3: Adt20, v4: u64, v5: u64) -> R<Adt19> {
    let v6: u64 = match v2.clone() {
        Adt20::C0(h5714, _, _, _, _, _, _, _) => {
            h5714
        }
    };
    let v7: u64 = {
        let a5717 = 1u64;
        let a5718 = {
            let a5715 = 3u64;
            let a5716 = v6;
            nat_mul(a5715, a5716)?
        };
        nat_add(a5717, a5718)?
    };
    let a5719 = v0;
    let a5720 = 0u64;
    if nat_eq(a5719, a5720) {
        Ok(f278(0u64))
    } else {
        let a5721 = v0;
        let a5722 = v7;
        if nat_lt(a5721, a5722) {
            let v8: u64 = {
                let a5723 = v0;
                let a5724 = 1u64;
                nat_sub(a5723, a5724)
            };
            let v9: u64 = {
                let a5730 = {
                    let a5725 = v1;
                    let a5726 = v6;
                    nat_mul(a5725, a5726)?
                };
                let a5731 = {
                    let a5727 = v8;
                    let a5728 = 3u64;
                    let a5729 = 0u64;
                    nat_quot(a5727, a5728, a5729)
                };
                nat_add(a5730, a5731)?
            };
            let v10: u64 = f279(v9, v2)?;
            let v11: u64 = {
                let a5732 = v8;
                let a5733 = 3u64;
                let a5734 = 0u64;
                nat_rem(a5732, a5733, a5734)
            };
            let a5735 = v11;
            let a5736 = 0u64;
            if nat_eq(a5735, a5736) {
                Ok(f280(10u64, 0u64, v10, 0u64, v10, 1u64, 1u64))
            } else {
                let a5737 = v11;
                let a5738 = 1u64;
                if nat_eq(a5737, a5738) {
                    Ok(f280(1u64, 1u64, 1u64, 2u64, 0u64, 1u64, 1u64))
                } else {
                    Ok(f280(0u64, 1u64, 0u64, 1u64, 1u64, 1u64, 0u64))
                }
            }
        } else {
            let a5739 = v0;
            let a5740 = v7;
            if nat_eq(a5739, a5740) {
                Ok(f280(2u64, 1u64, 0u64, 2u64, 0u64, 1u64, 1u64))
            } else {
                let a5743 = v0;
                let a5744 = {
                    let a5741 = v7;
                    let a5742 = 1u64;
                    nat_add(a5741, a5742)?
                };
                if nat_eq(a5743, a5744) {
                    Ok(f280(12u64, 1u64, 1u64, 2u64, 0u64, 1u64, 1u64))
                } else {
                    let a5747 = v0;
                    let a5748 = {
                        let a5745 = v7;
                        let a5746 = 2u64;
                        nat_add(a5745, a5746)?
                    };
                    if nat_eq(a5747, a5748) {
                        Ok(f280(16u64, 1u64, 1u64, 2u64, v4, 1u64, 2u64))
                    } else {
                        let a5751 = v0;
                        let a5752 = {
                            let a5749 = v7;
                            let a5750 = 3u64;
                            nat_add(a5749, a5750)?
                        };
                        if nat_eq(a5751, a5752) {
                            let a5753 = v5;
                            let a5754 = 1u64;
                            if if nat_eq(a5753, a5754) {
                                true
                            } else {
                                let a5755 = v5;
                                let a5756 = 2u64;
                                nat_eq(a5755, a5756)
                            } {
                                Ok(f280(11u64, 2u64, 1065353216u64, 1u64, 1u64, 1u64, 2u64))
                            } else {
                                Ok(f280(11u64, 2u64, 1065353216u64, 2u64, v4, 1u64, 2u64))
                            }
                        } else {
                            let v12: u64 = {
                                let a5763 = {
                                    let a5757 = v1;
                                    let a5758 = v6;
                                    nat_mul(a5757, a5758)?
                                };
                                let a5764 = {
                                    let a5761 = v0;
                                    let a5762 = {
                                        let a5759 = v7;
                                        let a5760 = 4u64;
                                        nat_add(a5759, a5760)?
                                    };
                                    nat_sub(a5761, a5762)
                                };
                                nat_add(a5763, a5764)?
                            };
                            Ok(f280(10u64, 0u64, f279(v12, v2)?, 1u64, 2u64, 3u64, f279(v12, v3)?))
                        }
                    }
                }
            }
        }
    }
}

pub fn f268(v0: Adt20) -> Adt20 {
    Adt20::C0(match v0.clone() {
        Adt20::C0(h5765, _, _, _, _, _, _, _) => {
            h5765
        }
    }, match v0.clone() {
        Adt20::C0(_, h5766, _, _, _, _, _, _) => {
            h5766
        }
    }, match v0.clone() {
        Adt20::C0(_, _, h5767, _, _, _, _, _) => {
            h5767
        }
    }, match v0.clone() {
        Adt20::C0(_, _, _, h5768, _, _, _, _) => {
            h5768
        }
    }, 4u64, match v0.clone() {
        Adt20::C0(_, _, _, _, _, h5769, _, _) => {
            h5769
        }
    }, match v0.clone() {
        Adt20::C0(_, _, _, _, _, _, h5770, _) => {
            h5770
        }
    }, match v0 {
        Adt20::C0(_, _, _, _, _, _, _, h5771) => {
            h5771
        }
    })
}

pub fn f269(v0: u64, v1: Adt20, v2: u64, v3: u64, v4: u64) -> R<Adt19> {
    let v5: u64 = {
        let a5778 = v3;
        let a5779 = {
            let a5775 = f279({
                let a5772 = v0;
                let a5773 = 2u64;
                let a5774 = 0u64;
                nat_quot(a5772, a5773, a5774)
            }, v1)?;
            let a5776 = v2;
            let a5777 = 0u64;
            nat_quot(a5775, a5776, a5777)
        };
        nat_add(a5778, a5779)?
    };
    let a5783 = {
        let a5780 = v0;
        let a5781 = 2u64;
        let a5782 = 0u64;
        nat_rem(a5780, a5781, a5782)
    };
    let a5784 = 0u64;
    if nat_eq(a5783, a5784) {
        Ok(f280(19u64, 1u64, v5, 2u64, 0u64, 1u64, 2u64))
    } else {
        let a5785 = v4;
        let a5786 = 0u64;
        if nat_eq(a5785, a5786) {
            Ok(f280(19u64, 2u64, 1u64, 2u64, 0u64, 1u64, v5))
        } else {
            Ok(Adt19::C0)
        }
    }
}

pub fn f270(v0: u64, v1: u64, v2: Adt20, v3: Adt20, v4: Adt20, v5: u64) -> R<Adt19> {
    let v6: u64 = f281(match v2.clone() {
        Adt20::C0(h5787, _, _, _, _, _, _, _) => {
            h5787
        }
    })?;
    let v7: u64 = f282(match v2.clone() {
        Adt20::C0(h5788, _, _, _, _, _, _, _) => {
            h5788
        }
    })?;
    let a5789 = v0;
    let a5790 = 2u64;
    if nat_lt(a5789, a5790) {
        Ok(f278(v0))
    } else {
        let a5791 = v0;
        let a5792 = v6;
        if nat_lt(a5791, a5792) {
            f283({
                let a5793 = v0;
                let a5794 = 2u64;
                nat_sub(a5793, a5794)
            }, v1, v2, v3)
        } else {
            let a5795 = v0;
            let a5796 = v7;
            if nat_lt(a5795, a5796) {
                Ok(f284({
                    let a5797 = v0;
                    let a5798 = v6;
                    nat_sub(a5797, a5798)
                }, match v2 {
                    Adt20::C0(h5799, _, _, _, _, _, _, _) => {
                        h5799
                    }
                }, v5))
            } else {
                let a5801 = v0;
                let a5802 = f242(match v2.clone() {
                    Adt20::C0(h5800, _, _, _, _, _, _, _) => {
                        h5800
                    }
                })?;
                if nat_lt(a5801, a5802) {
                    f285({
                        let a5803 = v0;
                        let a5804 = v7;
                        nat_sub(a5803, a5804)
                    }, v1, v2, v3, v4)
                } else {
                    Ok(Adt19::C0)
                }
            }
        }
    }
}

pub fn f271(v0: u64, v1: u64, v2: Adt20, v3: Adt20, v4: u64, v5: bool) -> R<Adt19> {
    let v6: u64 = f256(v5, match v2.clone() {
        Adt20::C0(h5805, _, _, _, _, _, _, _) => {
            h5805
        }
    })?;
    let v7: u64 = {
        let a5807 = v6;
        let a5808 = f257(v5, match v2.clone() {
            Adt20::C0(h5806, _, _, _, _, _, _, _) => {
                h5806
            }
        })?;
        nat_add(a5807, a5808)?
    };
    let v8: u64 = {
        let a5809 = v7;
        let a5810 = f258(v5);
        nat_add(a5809, a5810)?
    };
    let a5811 = v0;
    let a5812 = v6;
    if nat_lt(a5811, a5812) {
        f286(v0, v1, v2, v5)
    } else {
        let a5813 = v0;
        let a5814 = v7;
        if nat_lt(a5813, a5814) {
            f287({
                let a5815 = v0;
                let a5816 = v6;
                nat_sub(a5815, a5816)
            }, v1, v2, v3, v5)
        } else {
            let a5817 = v0;
            let a5818 = v8;
            if nat_lt(a5817, a5818) {
                Ok(f288({
                    let a5819 = v0;
                    let a5820 = v7;
                    nat_sub(a5819, a5820)
                }, match v2 {
                    Adt20::C0(h5821, _, _, _, _, _, _, _) => {
                        h5821
                    }
                }, v4, v5))
            } else {
                let a5825 = v0;
                let a5826 = {
                    let a5823 = v8;
                    let a5824 = match v2.clone() {
                        Adt20::C0(h5822, _, _, _, _, _, _, _) => {
                            h5822
                        }
                    };
                    nat_add(a5823, a5824)?
                };
                if nat_lt(a5825, a5826) {
                    let v9: u64 = f279({
                        let a5832 = {
                            let a5828 = v1;
                            let a5829 = match v2 {
                                Adt20::C0(h5827, _, _, _, _, _, _, _) => {
                                    h5827
                                }
                            };
                            nat_mul(a5828, a5829)?
                        };
                        let a5833 = {
                            let a5830 = v0;
                            let a5831 = v8;
                            nat_sub(a5830, a5831)
                        };
                        nat_add(a5832, a5833)?
                    }, v3)?;
                    Ok(f280(10u64, 3u64, v9, 1u64, 1u64, 3u64, v9))
                } else {
                    Ok(Adt19::C0)
                }
            }
        }
    }
}

pub fn f272(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64) -> R<Adt22> {
    let v6: u64 = f259(v3, v5)?;
    let v7: u64 = f261(v1, v2, v6)?;
    let v8: u64 = {
        let a5836 = {
            let a5834 = f260(v3, v5)?;
            let a5835 = v4;
            nat_mul(a5834, a5835)?
        };
        let a5837 = v7;
        nat_mul(a5836, a5837)?
    };
    let a5838 = v0;
    let a5839 = v8;
    if nat_lt(a5838, a5839) {
        let v9: u64 = {
            let a5840 = v0;
            let a5841 = v7;
            let a5842 = 0u64;
            nat_quot(a5840, a5841, a5842)
        };
        Ok(Adt22::C0({
            let a5846 = {
                let a5843 = v9;
                let a5844 = v4;
                let a5845 = 0u64;
                nat_quot(a5843, a5844, a5845)
            };
            let a5847 = v6;
            nat_mul(a5846, a5847)?
        }, v6, {
            let a5848 = v9;
            let a5849 = v4;
            let a5850 = 0u64;
            nat_rem(a5848, a5849, a5850)
        }, {
            let a5851 = v0;
            let a5852 = v7;
            let a5853 = 0u64;
            nat_rem(a5851, a5852, a5853)
        }))
    } else {
        let v10: u64 = {
            let a5854 = v0;
            let a5855 = v8;
            nat_sub(a5854, a5855)
        };
        let v11: u64 = f261(v1, v2, f262(v3, v5)?)?;
        Ok(Adt22::C0({
            let a5856 = f260(v3, v5)?;
            let a5857 = v6;
            nat_mul(a5856, a5857)?
        }, f262(v3, v5)?, {
            let a5858 = v10;
            let a5859 = v11;
            let a5860 = 0u64;
            nat_quot(a5858, a5859, a5860)
        }, {
            let a5861 = v10;
            let a5862 = v11;
            let a5863 = 0u64;
            nat_rem(a5861, a5862, a5863)
        }))
    }
}

pub fn f273(v0: Adt22, v1: Adt20, v2: Adt20, v3: u64) -> R<Adt19> {
    let v4: u64 = match v0.clone() {
        Adt22::C0(_, _, _, h5864) => {
            h5864
        }
    };
    let v5: u64 = match v1.clone() {
        Adt20::C0(h5865, _, _, _, _, _, _, _) => {
            h5865
        }
    };
    let v6: u64 = {
        let a5868 = match v0.clone() {
            Adt22::C0(_, h5866, _, _) => {
                h5866
            }
        };
        let a5869 = match v1.clone() {
            Adt20::C0(_, h5867, _, _, _, _, _, _) => {
                h5867
            }
        };
        nat_mul(a5868, a5869)?
    };
    let v7: u64 = {
        let a5870 = v6;
        let a5871 = v5;
        nat_mul(a5870, a5871)?
    };
    let v8: u64 = {
        let a5874 = {
            let a5872 = 2u64;
            let a5873 = v5;
            nat_mul(a5872, a5873)?
        };
        let a5875 = 2u64;
        nat_add(a5874, a5875)?
    };
    let v9: u64 = {
        let a5878 = {
            let a5876 = 4u64;
            let a5877 = v5;
            nat_mul(a5876, a5877)?
        };
        let a5879 = 2u64;
        nat_add(a5878, a5879)?
    };
    let v10: u64 = {
        let a5882 = 1u64;
        let a5883 = {
            let a5880 = v6;
            let a5881 = v8;
            nat_mul(a5880, a5881)?
        };
        nat_add(a5882, a5883)?
    };
    let v11: u64 = {
        let a5884 = v10;
        let a5885 = 4u64;
        nat_add(a5884, a5885)?
    };
    let v12: u64 = {
        let a5888 = v11;
        let a5889 = {
            let a5886 = v6;
            let a5887 = v9;
            nat_mul(a5886, a5887)?
        };
        nat_add(a5888, a5889)?
    };
    let v13: u64 = {
        let a5890 = v12;
        let a5891 = 6u64;
        nat_add(a5890, a5891)?
    };
    let a5892 = v4;
    let a5893 = 0u64;
    if nat_eq(a5892, a5893) {
        Ok(f278(0u64))
    } else {
        let a5894 = v4;
        let a5895 = v10;
        if nat_lt(a5894, a5895) {
            let v14: u64 = {
                let a5896 = v4;
                let a5897 = 1u64;
                nat_sub(a5896, a5897)
            };
            f289(false, {
                let a5898 = v14;
                let a5899 = v8;
                let a5900 = 0u64;
                nat_rem(a5898, a5899, a5900)
            }, {
                let a5901 = v14;
                let a5902 = v8;
                let a5903 = 0u64;
                nat_quot(a5901, a5902, a5903)
            }, v0, v1, v2)
        } else {
            let a5904 = v4;
            let a5905 = v11;
            if nat_lt(a5904, a5905) {
                Ok(f290({
                    let a5906 = v4;
                    let a5907 = v10;
                    nat_sub(a5906, a5907)
                }, v7))
            } else {
                let a5908 = v4;
                let a5909 = v12;
                if nat_lt(a5908, a5909) {
                    let v15: u64 = {
                        let a5910 = v4;
                        let a5911 = v11;
                        nat_sub(a5910, a5911)
                    };
                    f289(true, {
                        let a5912 = v15;
                        let a5913 = v9;
                        let a5914 = 0u64;
                        nat_rem(a5912, a5913, a5914)
                    }, {
                        let a5915 = v15;
                        let a5916 = v9;
                        let a5917 = 0u64;
                        nat_quot(a5915, a5916, a5917)
                    }, v0, v1, v2)
                } else {
                    let a5918 = v4;
                    let a5919 = v13;
                    if nat_lt(a5918, a5919) {
                        Ok(f291({
                            let a5920 = v4;
                            let a5921 = v12;
                            nat_sub(a5920, a5921)
                        }, v7, v3))
                    } else {
                        let a5924 = v4;
                        let a5925 = {
                            let a5922 = v13;
                            let a5923 = v7;
                            nat_add(a5922, a5923)?
                        };
                        if nat_lt(a5924, a5925) {
                            let v16: u64 = {
                                let a5926 = v4;
                                let a5927 = v13;
                                nat_sub(a5926, a5927)
                            };
                            let v17: u64 = {
                                let a5928 = v16;
                                let a5929 = v5;
                                let a5930 = 0u64;
                                nat_quot(a5928, a5929, a5930)
                            };
                            let v18: u64 = {
                                let a5936 = {
                                    let a5931 = f292(v17, v0, v1)?;
                                    let a5932 = v5;
                                    nat_mul(a5931, a5932)?
                                };
                                let a5937 = {
                                    let a5933 = v16;
                                    let a5934 = v5;
                                    let a5935 = 0u64;
                                    nat_rem(a5933, a5934, a5935)
                                };
                                nat_add(a5936, a5937)?
                            };
                            let v19: u64 = f279(v18, v2)?;
                            Ok(f280(10u64, 3u64, v19, 1u64, 1u64, 3u64, v19))
                        } else {
                            Ok(Adt19::C0)
                        }
                    }
                }
            }
        }
    }
}

pub fn f274(v0: bool) -> u64 {
    if v0 {
        2u64
    } else {
        1u64
    }
}

pub fn f275(v0: u64) -> u64 {
    let a5938 = v0;
    let a5939 = 4u64;
    let a5940 = 0u64;
    nat_quot(a5938, a5939, a5940)
}

pub fn f276(v0: u64) -> R<u64> {
    let a5943 = {
        let a5941 = 7u64;
        let a5942 = v0;
        nat_mul(a5941, a5942)?
    };
    let a5944 = 4u64;
    nat_add(a5943, a5944)
}

pub fn f277(v0: u64) -> u64 {
    if f125(v0) {
        let m5945 = f127(v0);
        match m5945 {
            Adt17::C0 => {
                0u64
            }
            Adt17::C1(v1) => {
                let a5947 = 1u64;
                let a5948 = match v1.clone() {
                    Adt18::C0(_, h5946, _) => {
                        h5946
                    }
                };
                if if nat_lt(a5947, a5948) {
                    true
                } else {
                    let a5949 = v0;
                    let a5950 = 43u64;
                    nat_eq(a5949, a5950)
                } {
                    match v1 {
                        Adt18::C0(_, h5951, _) => {
                            h5951
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

pub fn f278(v0: u64) -> Adt19 {
    f280(0u64, 2u64, 0u64, 2u64, 0u64, 1u64, v0)
}

pub fn f279(v0: u64, v1: Adt20) -> R<u64> {
    let v2: u64 = {
        let a5953 = v0;
        let a5954 = match v1.clone() {
            Adt20::C0(h5952, _, _, _, _, _, _, _) => {
                h5952
            }
        };
        let a5955 = 0u64;
        nat_quot(a5953, a5954, a5955)
    };
    let v3: u64 = {
        let a5957 = v2;
        let a5958 = match v1.clone() {
            Adt20::C0(_, h5956, _, _, _, _, _, _) => {
                h5956
            }
        };
        let a5959 = 0u64;
        nat_quot(a5957, a5958, a5959)
    };
    f144({
        let a5961 = v0;
        let a5962 = match v1.clone() {
            Adt20::C0(h5960, _, _, _, _, _, _, _) => {
                h5960
            }
        };
        let a5963 = 0u64;
        nat_rem(a5961, a5962, a5963)
    }, {
        let a5965 = v2;
        let a5966 = match v1.clone() {
            Adt20::C0(_, h5964, _, _, _, _, _, _) => {
                h5964
            }
        };
        let a5967 = 0u64;
        nat_rem(a5965, a5966, a5967)
    }, {
        let a5969 = v3;
        let a5970 = match v1.clone() {
            Adt20::C0(_, _, h5968, _, _, _, _, _) => {
                h5968
            }
        };
        let a5971 = 0u64;
        nat_rem(a5969, a5970, a5971)
    }, {
        let a5973 = v3;
        let a5974 = match v1.clone() {
            Adt20::C0(_, _, h5972, _, _, _, _, _) => {
                h5972
            }
        };
        let a5975 = 0u64;
        nat_quot(a5973, a5974, a5975)
    }, match v1.clone() {
        Adt20::C0(_, _, _, _, h5976, _, _, _) => {
            h5976
        }
    }, match v1.clone() {
        Adt20::C0(_, _, _, _, _, h5977, _, _) => {
            h5977
        }
    }, match v1.clone() {
        Adt20::C0(_, _, _, _, _, _, h5978, _) => {
            h5978
        }
    }, match v1 {
        Adt20::C0(_, _, _, _, _, _, _, h5979) => {
            h5979
        }
    })
}

pub fn f280(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64) -> Adt19 {
    let a5980 = v0;
    let a5981 = 4u64;
    if if if nat_le(a5980, a5981) {
        true
    } else {
        let a5982 = 9u64;
        let a5983 = v0;
        if nat_le(a5982, a5983) {
            let a5984 = v0;
            let a5985 = 12u64;
            nat_le(a5984, a5985)
        } else {
            false
        }
    } {
        true
    } else {
        let a5986 = v0;
        let a5987 = 16u64;
        if if nat_eq(a5986, a5987) {
            true
        } else {
            let a5988 = v0;
            let a5989 = 19u64;
            nat_eq(a5988, a5989)
        } {
            true
        } else {
            let a5990 = v0;
            let a5991 = 21u64;
            if nat_eq(a5990, a5991) {
                true
            } else {
                let a5992 = v0;
                let a5993 = 22u64;
                nat_eq(a5992, a5993)
            }
        }
    } {
        Adt19::C1({
            let a5994 = v0;
            let a5995 = 4u64;
            if if nat_lt(a5994, a5995) {
                true
            } else {
                let a5996 = 21u64;
                let a5997 = v0;
                nat_le(a5996, a5997)
            } {
                0u64
            } else {
                let a5998 = v0;
                let a5999 = 16u64;
                if if nat_eq(a5998, a5999) {
                    true
                } else {
                    let a6000 = v0;
                    let a6001 = 19u64;
                    nat_eq(a6000, a6001)
                } {
                    2u64
                } else {
                    1u64
                }
            }
        }, {
            let a6002 = v0;
            let a6003 = 4u64;
            if nat_lt(a6002, a6003) {
                v0
            } else {
                let a6004 = v0;
                let a6005 = 4u64;
                if if nat_eq(a6004, a6005) {
                    true
                } else {
                    let a6006 = v0;
                    let a6007 = 16u64;
                    nat_eq(a6006, a6007)
                } {
                    0u64
                } else {
                    let a6008 = v0;
                    let a6009 = 19u64;
                    if nat_eq(a6008, a6009) {
                        3u64
                    } else {
                        let a6010 = 21u64;
                        let a6011 = v0;
                        if nat_le(a6010, a6011) {
                            let a6012 = v0;
                            let a6013 = 17u64;
                            nat_sub(a6012, a6013)
                        } else {
                            let a6014 = v0;
                            let a6015 = 8u64;
                            nat_sub(a6014, a6015)
                        }
                    }
                }
            }
        }, v1, v2, v3, v4, v5, v6)
    } else {
        Adt19::C0
    }
}

pub fn f281(v0: u64) -> R<u64> {
    let a6018 = 2u64;
    let a6019 = {
        let a6016 = 6u64;
        let a6017 = v0;
        nat_mul(a6016, a6017)?
    };
    nat_add(a6018, a6019)
}

pub fn f282(v0: u64) -> R<u64> {
    let a6020 = f281(v0)?;
    let a6021 = 11u64;
    nat_add(a6020, a6021)
}

pub fn f283(v0: u64, v1: u64, v2: Adt20, v3: Adt20) -> R<Adt19> {
    let v4: u64 = {
        let a6022 = v0;
        let a6023 = 6u64;
        let a6024 = 0u64;
        nat_rem(a6022, a6023, a6024)
    };
    let v5: u64 = {
        let a6031 = {
            let a6026 = v1;
            let a6027 = match v2.clone() {
                Adt20::C0(h6025, _, _, _, _, _, _, _) => {
                    h6025
                }
            };
            nat_mul(a6026, a6027)?
        };
        let a6032 = {
            let a6028 = v0;
            let a6029 = 6u64;
            let a6030 = 0u64;
            nat_quot(a6028, a6029, a6030)
        };
        nat_add(a6031, a6032)?
    };
    let v6: u64 = f279(v5, v3)?;
    let v7: u64 = f279(v5, v2)?;
    let a6033 = v4;
    let a6034 = 0u64;
    if nat_eq(a6033, a6034) {
        Ok(f280(10u64, 4u64, v6, 4u64, v6, 1u64, 2u64))
    } else {
        let a6035 = v4;
        let a6036 = 1u64;
        if nat_eq(a6035, a6036) {
            Ok(f280(1u64, 1u64, 2u64, 2u64, 0u64, 1u64, 2u64))
        } else {
            let a6037 = v4;
            let a6038 = 2u64;
            if nat_eq(a6037, a6038) {
                Ok(f280(0u64, 1u64, 0u64, 1u64, 2u64, 1u64, 0u64))
            } else {
                let a6039 = v4;
                let a6040 = 3u64;
                if nat_eq(a6039, a6040) {
                    Ok(f280(10u64, 4u64, v6, 0u64, v7, 1u64, 2u64))
                } else {
                    let a6041 = v4;
                    let a6042 = 4u64;
                    if nat_eq(a6041, a6042) {
                        Ok(f280(1u64, 1u64, 2u64, 2u64, 0u64, 1u64, 2u64))
                    } else {
                        Ok(f280(0u64, 1u64, 1u64, 1u64, 2u64, 1u64, 1u64))
                    }
                }
            }
        }
    }
}

pub fn f284(v0: u64, v1: u64, v2: u64) -> Adt19 {
    let a6043 = v0;
    let a6044 = 0u64;
    if nat_eq(a6043, a6044) {
        f280(2u64, 1u64, 0u64, 2u64, 0u64, 1u64, 3u64)
    } else {
        let a6045 = v0;
        let a6046 = 1u64;
        if nat_eq(a6045, a6046) {
            f280(3u64, 2u64, v1, 2u64, 0u64, 1u64, 2u64)
        } else {
            let a6047 = v0;
            let a6048 = 2u64;
            if nat_eq(a6047, a6048) {
                f280(11u64, 1u64, 3u64, 1u64, 2u64, 1u64, 4u64)
            } else {
                let a6049 = v0;
                let a6050 = 3u64;
                if nat_eq(a6049, a6050) {
                    f280(4u64, 1u64, 4u64, 2u64, v2, 1u64, 4u64)
                } else {
                    let a6051 = v0;
                    let a6052 = 4u64;
                    if nat_eq(a6051, a6052) {
                        f280(12u64, 1u64, 4u64, 2u64, 0u64, 1u64, 4u64)
                    } else {
                        let a6053 = v0;
                        let a6054 = 5u64;
                        if nat_eq(a6053, a6054) {
                            f280(11u64, 2u64, 1065353216u64, 1u64, 4u64, 1u64, 4u64)
                        } else {
                            let a6055 = v0;
                            let a6056 = 6u64;
                            if nat_eq(a6055, a6056) {
                                f280(10u64, 2u64, v2, 1u64, 2u64, 1u64, 5u64)
                            } else {
                                let a6057 = v0;
                                let a6058 = 7u64;
                                if nat_eq(a6057, a6058) {
                                    f280(4u64, 1u64, 3u64, 1u64, 5u64, 1u64, 5u64)
                                } else {
                                    let a6059 = v0;
                                    let a6060 = 8u64;
                                    if nat_eq(a6059, a6060) {
                                        f280(2u64, 1u64, 1u64, 2u64, 0u64, 1u64, 2u64)
                                    } else {
                                        let a6061 = v0;
                                        let a6062 = 9u64;
                                        if nat_eq(a6061, a6062) {
                                            Adt19::C1(3u64, 0u64, 1u64, 2u64, 2u64, 2u64, 1u64, 2u64)
                                        } else {
                                            let a6063 = v0;
                                            let a6064 = 10u64;
                                            if nat_eq(a6063, a6064) {
                                                f280(11u64, 1u64, 2u64, 1u64, 5u64, 1u64, 5u64)
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

pub fn f285(v0: u64, v1: u64, v2: Adt20, v3: Adt20, v4: Adt20) -> R<Adt19> {
    let v5: u64 = {
        let a6065 = v0;
        let a6066 = 3u64;
        let a6067 = 0u64;
        nat_rem(a6065, a6066, a6067)
    };
    let v6: u64 = {
        let a6074 = {
            let a6069 = v1;
            let a6070 = match v2.clone() {
                Adt20::C0(h6068, _, _, _, _, _, _, _) => {
                    h6068
                }
            };
            nat_mul(a6069, a6070)?
        };
        let a6075 = {
            let a6071 = v0;
            let a6072 = 3u64;
            let a6073 = 0u64;
            nat_quot(a6071, a6072, a6073)
        };
        nat_add(a6074, a6075)?
    };
    let v7: u64 = f279(v6, v3)?;
    let v8: u64 = f279(v6, v2)?;
    let v9: u64 = f279(v6, v4)?;
    let a6076 = v5;
    let a6077 = 0u64;
    if nat_eq(a6076, a6077) {
        Ok(f280(10u64, 4u64, v7, 1u64, 5u64, 1u64, 2u64))
    } else {
        let a6078 = v5;
        let a6079 = 1u64;
        if nat_eq(a6078, a6079) {
            Ok(f280(4u64, 0u64, v8, 1u64, 2u64, 1u64, 2u64))
        } else {
            Ok(f280(10u64, 1u64, 2u64, 1u64, 4u64, 3u64, v9))
        }
    }
}

pub fn f286(v0: u64, v1: u64, v2: Adt20, v3: bool) -> R<Adt19> {
    let v4: u64 = match v2.clone() {
        Adt20::C0(h6080, _, _, _, _, _, _, _) => {
            h6080
        }
    };
    let v5: u64 = {
        let a6083 = 1u64;
        let a6084 = {
            let a6081 = v4;
            let a6082 = f274(v3);
            nat_mul(a6081, a6082)?
        };
        nat_add(a6083, a6084)?
    };
    let a6085 = v0;
    let a6086 = 0u64;
    if nat_eq(a6085, a6086) {
        Ok(f278(0u64))
    } else {
        let a6087 = v0;
        let a6088 = v5;
        if nat_lt(a6087, a6088) {
            let v6: u64 = {
                let a6089 = v0;
                let a6090 = 1u64;
                nat_sub(a6089, a6090)
            };
            let v7: u64 = f279({
                let a6096 = {
                    let a6091 = v1;
                    let a6092 = v4;
                    nat_mul(a6091, a6092)?
                };
                let a6097 = {
                    let a6093 = v6;
                    let a6094 = f274(v3);
                    let a6095 = 0u64;
                    nat_quot(a6093, a6094, a6095)
                };
                nat_add(a6096, a6097)?
            }, v2)?;
            if v3 {
                let a6101 = {
                    let a6098 = v6;
                    let a6099 = 2u64;
                    let a6100 = 0u64;
                    nat_rem(a6098, a6099, a6100)
                };
                let a6102 = 0u64;
                if nat_eq(a6101, a6102) {
                    Ok(f280(1u64, 0u64, v7, 2u64, 0u64, 1u64, 1u64))
                } else {
                    Ok(f280(0u64, 1u64, 0u64, 1u64, 1u64, 1u64, 0u64))
                }
            } else {
                Ok(f280(4u64, 1u64, 0u64, 0u64, v7, 1u64, 0u64))
            }
        } else {
            let v8: u64 = {
                let a6103 = v0;
                let a6104 = v5;
                nat_sub(a6103, a6104)
            };
            let a6105 = v8;
            let a6106 = 0u64;
            if nat_eq(a6105, a6106) {
                Ok(f280(3u64, 2u64, v4, 2u64, 0u64, 1u64, 1u64))
            } else {
                if if v3 {
                    let a6107 = v8;
                    let a6108 = 1u64;
                    nat_eq(a6107, a6108)
                } else {
                    false
                } {
                    Ok(f280(2u64, 1u64, 0u64, 2u64, 0u64, 1u64, 0u64))
                } else {
                    let a6109 = v8;
                    let a6110 = if v3 {
                        2u64
                    } else {
                        1u64
                    };
                    if nat_eq(a6109, a6110) {
                        Ok(f280(11u64, 1u64, 0u64, 1u64, 1u64, 1u64, 3u64))
                    } else {
                        let a6111 = v8;
                        let a6112 = if v3 {
                            3u64
                        } else {
                            2u64
                        };
                        if nat_eq(a6111, a6112) {
                            Ok(f278(0u64))
                        } else {
                            Ok(Adt19::C0)
                        }
                    }
                }
            }
        }
    }
}

pub fn f287(v0: u64, v1: u64, v2: Adt20, v3: Adt20, v4: bool) -> R<Adt19> {
    let v5: u64 = if v4 {
        let a6114 = 13u64;
        let a6115 = f275(match v2.clone() {
            Adt20::C0(h6113, _, _, _, _, _, _, _) => {
                h6113
            }
        });
        nat_mul(a6114, a6115)?
    } else {
        0u64
    };
    let a6116 = v0;
    let a6117 = v5;
    if nat_lt(a6116, a6117) {
        f293({
            let a6118 = v0;
            let a6119 = 13u64;
            let a6120 = 0u64;
            nat_rem(a6118, a6119, a6120)
        }, v1, {
            let a6121 = v0;
            let a6122 = 13u64;
            let a6123 = 0u64;
            nat_quot(a6121, a6122, a6123)
        }, v2, v3)
    } else {
        let v6: u64 = {
            let a6124 = v0;
            let a6125 = v5;
            nat_sub(a6124, a6125)
        };
        let v7: u64 = if v4 {
            4u64
        } else {
            3u64
        };
        f294({
            let a6126 = v6;
            let a6127 = v7;
            let a6128 = 0u64;
            nat_rem(a6126, a6127, a6128)
        }, {
            let a6138 = {
                let a6133 = {
                    let a6130 = v1;
                    let a6131 = match v2.clone() {
                        Adt20::C0(h6129, _, _, _, _, _, _, _) => {
                            h6129
                        }
                    };
                    nat_mul(a6130, a6131)?
                };
                let a6134 = if v4 {
                    f295(match v2.clone() {
                        Adt20::C0(h6132, _, _, _, _, _, _, _) => {
                            h6132
                        }
                    })?
                } else {
                    0u64
                };
                nat_add(a6133, a6134)?
            };
            let a6139 = {
                let a6135 = v6;
                let a6136 = v7;
                let a6137 = 0u64;
                nat_quot(a6135, a6136, a6137)
            };
            nat_add(a6138, a6139)?
        }, v2, v3, v4)
    }
}

pub fn f288(v0: u64, v1: u64, v2: u64, v3: bool) -> Adt19 {
    let a6140 = v0;
    let a6141 = 0u64;
    if nat_eq(a6140, a6141) {
        f280(if v3 {
            22u64
        } else {
            3u64
        }, 2u64, v1, 2u64, 0u64, 1u64, 1u64)
    } else {
        let a6142 = v0;
        let a6143 = 1u64;
        if nat_eq(a6142, a6143) {
            f280(if v3 {
                21u64
            } else {
                11u64
            }, 1u64, 0u64, 1u64, 1u64, 1u64, 1u64)
        } else {
            if if v3 {
                let a6144 = v0;
                let a6145 = 2u64;
                nat_eq(a6144, a6145)
            } else {
                false
            } {
                f280(2u64, 1u64, 1u64, 2u64, 0u64, 1u64, 1u64)
            } else {
                let a6146 = v0;
                let a6147 = if v3 {
                    3u64
                } else {
                    2u64
                };
                if nat_eq(a6146, a6147) {
                    f280(4u64, 1u64, 1u64, 2u64, v2, 1u64, 1u64)
                } else {
                    let a6148 = v0;
                    let a6149 = if v3 {
                        4u64
                    } else {
                        3u64
                    };
                    if nat_eq(a6148, a6149) {
                        f280(12u64, 1u64, 1u64, 2u64, 0u64, 1u64, 1u64)
                    } else {
                        let a6150 = v0;
                        let a6151 = if v3 {
                            5u64
                        } else {
                            4u64
                        };
                        if nat_eq(a6150, a6151) {
                            f280(11u64, 2u64, 1065353216u64, 1u64, 1u64, 1u64, 1u64)
                        } else {
                            Adt19::C0
                        }
                    }
                }
            }
        }
    }
}

pub fn f289(v0: bool, v1: u64, v2: u64, v3: Adt22, v4: Adt20, v5: Adt20) -> R<Adt19> {
    let v6: u64 = if v0 {
        4u64
    } else {
        2u64
    };
    let v7: u64 = {
        let a6155 = 1u64;
        let a6156 = {
            let a6153 = match v4.clone() {
                Adt20::C0(h6152, _, _, _, _, _, _, _) => {
                    h6152
                }
            };
            let a6154 = v6;
            nat_mul(a6153, a6154)?
        };
        nat_add(a6155, a6156)?
    };
    let a6157 = v1;
    let a6158 = 0u64;
    if nat_eq(a6157, a6158) {
        Ok(f278(1u64))
    } else {
        let a6159 = v1;
        let a6160 = v7;
        if nat_lt(a6159, a6160) {
            let v8: u64 = {
                let a6161 = v1;
                let a6162 = 1u64;
                nat_sub(a6161, a6162)
            };
            let v9: u64 = {
                let a6169 = {
                    let a6164 = f292(v2, v3, v4.clone())?;
                    let a6165 = match v4.clone() {
                        Adt20::C0(h6163, _, _, _, _, _, _, _) => {
                            h6163
                        }
                    };
                    nat_mul(a6164, a6165)?
                };
                let a6170 = {
                    let a6166 = v8;
                    let a6167 = v6;
                    let a6168 = 0u64;
                    nat_quot(a6166, a6167, a6168)
                };
                nat_add(a6169, a6170)?
            };
            let v10: u64 = f279(v9, v4)?;
            let v11: u64 = f279(v9, v5)?;
            let v12: u64 = {
                let a6171 = v8;
                let a6172 = v6;
                let a6173 = 0u64;
                nat_rem(a6171, a6172, a6173)
            };
            if v0 {
                let a6174 = v12;
                let a6175 = 0u64;
                if nat_eq(a6174, a6175) {
                    Ok(f280(9u64, 0u64, v10, 1u64, 3u64, 3u64, v11))
                } else {
                    let a6176 = v12;
                    let a6177 = 1u64;
                    if nat_eq(a6176, a6177) {
                        Ok(f280(10u64, 3u64, v11, 3u64, v11, 1u64, 2u64))
                    } else {
                        let a6178 = v12;
                        let a6179 = 2u64;
                        if nat_eq(a6178, a6179) {
                            Ok(f280(1u64, 1u64, 2u64, 2u64, 0u64, 1u64, 2u64))
                        } else {
                            Ok(f280(0u64, 1u64, 1u64, 1u64, 2u64, 1u64, 1u64))
                        }
                    }
                }
            } else {
                let a6180 = v12;
                let a6181 = 0u64;
                if nat_eq(a6180, a6181) {
                    Ok(f280(1u64, 0u64, v10, 2u64, 0u64, 1u64, 2u64))
                } else {
                    Ok(f280(0u64, 1u64, 1u64, 1u64, 2u64, 1u64, 1u64))
                }
            }
        } else {
            let a6182 = v1;
            let a6183 = v7;
            if nat_eq(a6182, a6183) {
                Ok(f280(0u64, 1u64, 0u64, 1u64, 1u64, 1u64, 0u64))
            } else {
                Ok(Adt19::C0)
            }
        }
    }
}

pub fn f290(v0: u64, v1: u64) -> Adt19 {
    let a6184 = v0;
    let a6185 = 0u64;
    if nat_eq(a6184, a6185) {
        f280(22u64, 2u64, v1, 2u64, 0u64, 1u64, 2u64)
    } else {
        let a6186 = v0;
        let a6187 = 1u64;
        if nat_eq(a6186, a6187) {
            f280(21u64, 1u64, 0u64, 1u64, 2u64, 1u64, 3u64)
        } else {
            let a6188 = v0;
            let a6189 = 2u64;
            if nat_eq(a6188, a6189) {
                f280(2u64, 1u64, 3u64, 2u64, 0u64, 1u64, 3u64)
            } else {
                let a6190 = v0;
                let a6191 = 3u64;
                if nat_eq(a6190, a6191) {
                    f278(0u64)
                } else {
                    Adt19::C0
                }
            }
        }
    }
}

pub fn f291(v0: u64, v1: u64, v2: u64) -> Adt19 {
    let a6192 = v0;
    let a6193 = 0u64;
    if nat_eq(a6192, a6193) {
        f280(22u64, 2u64, v1, 2u64, 0u64, 1u64, 2u64)
    } else {
        let a6194 = v0;
        let a6195 = 1u64;
        if nat_eq(a6194, a6195) {
            f280(21u64, 1u64, 0u64, 1u64, 2u64, 1u64, 2u64)
        } else {
            let a6196 = v0;
            let a6197 = 2u64;
            if nat_eq(a6196, a6197) {
                f280(2u64, 1u64, 2u64, 2u64, 0u64, 1u64, 2u64)
            } else {
                let a6198 = v0;
                let a6199 = 3u64;
                if nat_eq(a6198, a6199) {
                    f280(4u64, 1u64, 2u64, 2u64, v2, 1u64, 1u64)
                } else {
                    let a6200 = v0;
                    let a6201 = 4u64;
                    if nat_eq(a6200, a6201) {
                        f280(12u64, 1u64, 1u64, 2u64, 0u64, 1u64, 1u64)
                    } else {
                        let a6202 = v0;
                        let a6203 = 5u64;
                        if nat_eq(a6202, a6203) {
                            f280(11u64, 2u64, 1065353216u64, 1u64, 1u64, 1u64, 1u64)
                        } else {
                            Adt19::C0
                        }
                    }
                }
            }
        }
    }
}

pub fn f292(v0: u64, v1: Adt22, v2: Adt20) -> R<u64> {
    let a6214 = {
        let a6212 = {
            let a6209 = {
                let a6206 = match v1.clone() {
                    Adt22::C0(_, _, h6204, _) => {
                        h6204
                    }
                };
                let a6207 = match v2.clone() {
                    Adt20::C0(_, _, h6205, _, _, _, _, _) => {
                        h6205
                    }
                };
                nat_mul(a6206, a6207)?
            };
            let a6210 = match v1 {
                Adt22::C0(h6208, _, _, _) => {
                    h6208
                }
            };
            nat_add(a6209, a6210)?
        };
        let a6213 = match v2 {
            Adt20::C0(_, h6211, _, _, _, _, _, _) => {
                h6211
            }
        };
        nat_mul(a6212, a6213)?
    };
    let a6215 = v0;
    nat_add(a6214, a6215)
}

pub fn f293(v0: u64, v1: u64, v2: u64, v3: Adt20, v4: Adt20) -> R<Adt19> {
    let v5: u64 = {
        let a6223 = {
            let a6221 = {
                let a6217 = v1;
                let a6218 = match v3.clone() {
                    Adt20::C0(h6216, _, _, _, _, _, _, _) => {
                        h6216
                    }
                };
                nat_mul(a6217, a6218)?
            };
            let a6222 = {
                let a6219 = v2;
                let a6220 = 4u64;
                nat_mul(a6219, a6220)?
            };
            nat_add(a6221, a6222)?
        };
        let a6224 = f296(v0);
        nat_add(a6223, a6224)?
    };
    let v6: u64 = f279(v5, v3)?;
    let v7: u64 = f279(v5, v4)?;
    let a6225 = v0;
    let a6226 = 0u64;
    if if if if nat_eq(a6225, a6226) {
        true
    } else {
        let a6227 = v0;
        let a6228 = 2u64;
        nat_eq(a6227, a6228)
    } {
        true
    } else {
        let a6229 = v0;
        let a6230 = 5u64;
        nat_eq(a6229, a6230)
    } {
        true
    } else {
        let a6231 = v0;
        let a6232 = 7u64;
        nat_eq(a6231, a6232)
    } {
        Ok(f280(9u64, 0u64, v6, 1u64, 3u64, 3u64, v7))
    } else {
        let a6233 = v0;
        let a6234 = 1u64;
        if if if if nat_eq(a6233, a6234) {
            true
        } else {
            let a6235 = v0;
            let a6236 = 3u64;
            nat_eq(a6235, a6236)
        } {
            true
        } else {
            let a6237 = v0;
            let a6238 = 6u64;
            nat_eq(a6237, a6238)
        } {
            true
        } else {
            let a6239 = v0;
            let a6240 = 8u64;
            nat_eq(a6239, a6240)
        } {
            Ok(f280(10u64, 3u64, v7, 3u64, v7, 1u64, {
                let a6241 = v0;
                let a6242 = 1u64;
                if nat_eq(a6241, a6242) {
                    4u64
                } else {
                    let a6243 = v0;
                    let a6244 = 6u64;
                    if nat_eq(a6243, a6244) {
                        5u64
                    } else {
                        1u64
                    }
                }
            }))
        } else {
            let a6245 = v0;
            let a6246 = 4u64;
            if nat_eq(a6245, a6246) {
                Ok(f280(4u64, 1u64, 4u64, 1u64, 1u64, 1u64, 4u64))
            } else {
                let a6247 = v0;
                let a6248 = 9u64;
                if nat_eq(a6247, a6248) {
                    Ok(f280(4u64, 1u64, 5u64, 1u64, 1u64, 1u64, 5u64))
                } else {
                    let a6249 = v0;
                    let a6250 = 10u64;
                    if nat_eq(a6249, a6250) {
                        Ok(f280(4u64, 1u64, 4u64, 1u64, 5u64, 1u64, 1u64))
                    } else {
                        let a6251 = v0;
                        let a6252 = 11u64;
                        if nat_eq(a6251, a6252) {
                            Ok(f280(1u64, 1u64, 1u64, 2u64, 0u64, 1u64, 1u64))
                        } else {
                            let a6253 = v0;
                            let a6254 = 12u64;
                            if nat_eq(a6253, a6254) {
                                Ok(f280(0u64, 1u64, 0u64, 1u64, 1u64, 1u64, 0u64))
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

pub fn f294(v0: u64, v1: u64, v2: Adt20, v3: Adt20, v4: bool) -> R<Adt19> {
    let v5: u64 = f279(v1, v2)?;
    let v6: u64 = f279(v1, v3)?;
    let a6255 = v0;
    let a6256 = 0u64;
    if nat_eq(a6255, a6256) {
        Ok(f280(9u64, 0u64, v5, 1u64, 3u64, 3u64, v6))
    } else {
        let a6257 = v0;
        let a6258 = 1u64;
        if nat_eq(a6257, a6258) {
            Ok(f280(10u64, 3u64, v6, 3u64, v6, 1u64, 1u64))
        } else {
            if v4 {
                let a6259 = v0;
                let a6260 = 2u64;
                if nat_eq(a6259, a6260) {
                    Ok(f280(1u64, 1u64, 1u64, 2u64, 0u64, 1u64, 1u64))
                } else {
                    let a6261 = v0;
                    let a6262 = 3u64;
                    if nat_eq(a6261, a6262) {
                        Ok(f280(0u64, 1u64, 0u64, 1u64, 1u64, 1u64, 0u64))
                    } else {
                        Ok(Adt19::C0)
                    }
                }
            } else {
                let a6263 = v0;
                let a6264 = 2u64;
                if nat_eq(a6263, a6264) {
                    Ok(f280(4u64, 1u64, 0u64, 1u64, 1u64, 1u64, 0u64))
                } else {
                    Ok(Adt19::C0)
                }
            }
        }
    }
}

pub fn f295(v0: u64) -> R<u64> {
    let a6265 = 4u64;
    let a6266 = f275(v0);
    nat_mul(a6265, a6266)
}

pub fn f296(v0: u64) -> u64 {
    let a6267 = v0;
    let a6268 = 5u64;
    if nat_lt(a6267, a6268) {
        let a6269 = v0;
        let a6270 = 2u64;
        if nat_lt(a6269, a6270) {
            0u64
        } else {
            1u64
        }
    } else {
        let a6271 = v0;
        let a6272 = 7u64;
        if nat_lt(a6271, a6272) {
            2u64
        } else {
            3u64
        }
    }
}
