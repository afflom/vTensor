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
    C36(Adt3, bool, u64),
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
        let a465 = 86u64;
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
            let a468 = 55u64;
            if nat_eq(a467, a468) {
                let m469 = v3;
                match m469 {
                    None => {
                        Adt10::C1
                    }
                    Some(v7) => {
                        f42(v1, v2, v7, v4)?
                    }
                }
            } else {
                let a470 = v0;
                let a471 = 22u64;
                if nat_eq(a470, a471) {
                    let m472 = v3;
                    match m472 {
                        None => {
                            Adt10::C1
                        }
                        Some(v8) => {
                            f43(v2, v8, v4)?
                        }
                    }
                } else {
                    let a473 = v0;
                    let a474 = 44u64;
                    if nat_eq(a473, a474) {
                        f44(v1, v2, v3, v4)?
                    } else {
                        let a475 = v0;
                        let a476 = 52u64;
                        if nat_eq(a475, a476) {
                            f45(v1, v2, v4)?
                        } else {
                            let a477 = v0;
                            let a478 = 73u64;
                            if nat_eq(a477, a478) {
                                f46(v2, v3, v4)?
                            } else {
                                let a479 = v0;
                                let a480 = 57u64;
                                if if nat_eq(a479, a480) {
                                    true
                                } else {
                                    let a481 = v0;
                                    let a482 = 58u64;
                                    nat_eq(a481, a482)
                                } {
                                    let m483 = v3;
                                    match m483 {
                                        None => {
                                            f47({
                                                let a484 = v0;
                                                let a485 = 58u64;
                                                nat_eq(a484, a485)
                                            }, v1, v2, v4)?
                                        }
                                        Some(_) => {
                                            Adt10::C1
                                        }
                                    }
                                } else {
                                    let a486 = v0;
                                    let a487 = 49u64;
                                    if nat_eq(a486, a487) {
                                        f48(v1, v2, v3, v4)?
                                    } else {
                                        let a488 = v0;
                                        let a489 = 51u64;
                                        if nat_eq(a488, a489) {
                                            f49(v1, v2, v3, v4)?
                                        } else {
                                            let a490 = v0;
                                            let a491 = 39u64;
                                            if nat_eq(a490, a491) {
                                                f50(v2, v3, v4)?
                                            } else {
                                                let a492 = v0;
                                                let a493 = 77u64;
                                                if nat_eq(a492, a493) {
                                                    f51(v2, v4)?
                                                } else {
                                                    let a494 = v0;
                                                    let a495 = 40u64;
                                                    if nat_eq(a494, a495) {
                                                        f52(v2, v3, v4)?
                                                    } else {
                                                        if f53(v0) {
                                                            let a496 = v0;
                                                            let a497 = 33u64;
                                                            if if if nat_eq(a496, a497) {
                                                                let a499 = match v2.clone() {
                                                                    Adt2::C0(h498, _, _, _, _, _, _, _, _, _, _) => {
                                                                        h498
                                                                    }
                                                                };
                                                                let a500 = 0u64;
                                                                nat_eq(a499, a500)
                                                            } else {
                                                                false
                                                            } {
                                                                let a502 = match v4.clone() {
                                                                    Adt2::C0(h501, _, _, _, _, _, _, _, _, _, _) => {
                                                                        h501
                                                                    }
                                                                };
                                                                let a503 = 26u64;
                                                                nat_eq(a502, a503)
                                                            } else {
                                                                false
                                                            } {
                                                                let m504 = v3;
                                                                match m504 {
                                                                    None => {
                                                                        Adt10::C1
                                                                    }
                                                                    Some(v10) => {
                                                                        f54(v2, v10, v4)?
                                                                    }
                                                                }
                                                            } else {
                                                                let a505 = v0;
                                                                let a506 = 33u64;
                                                                if if if nat_eq(a505, a506) {
                                                                    let a508 = match v4.clone() {
                                                                        Adt2::C0(h507, _, _, _, _, _, _, _, _, _, _) => {
                                                                            h507
                                                                        }
                                                                    };
                                                                    let a509 = 0u64;
                                                                    nat_eq(a508, a509)
                                                                } else {
                                                                    false
                                                                } {
                                                                    f55(match v2.clone() {
                                                                        Adt2::C0(h510, _, _, _, _, _, _, _, _, _, _) => {
                                                                            h510
                                                                        }
                                                                    })
                                                                } else {
                                                                    false
                                                                } {
                                                                    f56(v2, v3, v4)?
                                                                } else {
                                                                    f57(v0, v2, {
                                                                        let m511 = v3;
                                                                        match m511 {
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
                                                            let a512 = v0;
                                                            let a513 = 19u64;
                                                            if nat_eq(a512, a513) {
                                                                f58(v2, v4)?
                                                            } else {
                                                                let a514 = v0;
                                                                let a515 = 21u64;
                                                                if nat_eq(a514, a515) {
                                                                    f59(v5, v2, v3, v4)?
                                                                } else {
                                                                    let a516 = v0;
                                                                    let a517 = 61u64;
                                                                    if if if nat_eq(a516, a517) {
                                                                        true
                                                                    } else {
                                                                        let a518 = v0;
                                                                        let a519 = 62u64;
                                                                        nat_eq(a518, a519)
                                                                    } {
                                                                        true
                                                                    } else {
                                                                        let a520 = v0;
                                                                        let a521 = 63u64;
                                                                        nat_eq(a520, a521)
                                                                    } {
                                                                        f60(v0, v1, v2, v4)?
                                                                    } else {
                                                                        let a522 = v0;
                                                                        let a523 = 4u64;
                                                                        if if nat_eq(a522, a523) {
                                                                            true
                                                                        } else {
                                                                            let a524 = v0;
                                                                            let a525 = 32u64;
                                                                            nat_eq(a524, a525)
                                                                        } {
                                                                            f61(v0, v1, v2, v3, v4)?
                                                                        } else {
                                                                            let a526 = v0;
                                                                            let a527 = 20u64;
                                                                            if nat_eq(a526, a527) {
                                                                                f62(v2, v4)?
                                                                            } else {
                                                                                let a528 = v0;
                                                                                let a529 = 42u64;
                                                                                if nat_eq(a528, a529) {
                                                                                    f63(4u64, v2, v4)?
                                                                                } else {
                                                                                    let a530 = v0;
                                                                                    let a531 = 69u64;
                                                                                    if nat_eq(a530, a531) {
                                                                                        let a532 = v5;
                                                                                        let a533 = 4u64;
                                                                                        if nat_lt(a532, a533) {
                                                                                            f63(v5, v2, v4)?
                                                                                        } else {
                                                                                            Adt10::C1
                                                                                        }
                                                                                    } else {
                                                                                        let a534 = v0;
                                                                                        let a535 = 70u64;
                                                                                        if nat_eq(a534, a535) {
                                                                                            f64(v5, v2, v4)?
                                                                                        } else {
                                                                                            let a536 = v0;
                                                                                            let a537 = 75u64;
                                                                                            if if nat_eq(a536, a537) {
                                                                                                true
                                                                                            } else {
                                                                                                let a538 = v0;
                                                                                                let a539 = 76u64;
                                                                                                nat_eq(a538, a539)
                                                                                            } {
                                                                                                f65({
                                                                                                    let a540 = v0;
                                                                                                    let a541 = 76u64;
                                                                                                    nat_eq(a540, a541)
                                                                                                }, v1, v2, v4)?
                                                                                            } else {
                                                                                                let a542 = v0;
                                                                                                let a543 = 31u64;
                                                                                                if if if nat_eq(a542, a543) {
                                                                                                    true
                                                                                                } else {
                                                                                                    let a544 = v0;
                                                                                                    let a545 = 47u64;
                                                                                                    nat_eq(a544, a545)
                                                                                                } {
                                                                                                    true
                                                                                                } else {
                                                                                                    let a546 = v0;
                                                                                                    let a547 = 68u64;
                                                                                                    nat_eq(a546, a547)
                                                                                                } {
                                                                                                    f66(v0, v1, v2, v4)?
                                                                                                } else {
                                                                                                    let a548 = v0;
                                                                                                    let a549 = 15u64;
                                                                                                    if nat_eq(a548, a549) {
                                                                                                        f67(v2, v4)?
                                                                                                    } else {
                                                                                                        let a550 = v0;
                                                                                                        let a551 = 17u64;
                                                                                                        if nat_eq(a550, a551) {
                                                                                                            f68(v2, v4)?
                                                                                                        } else {
                                                                                                            let a552 = v0;
                                                                                                            let a553 = 18u64;
                                                                                                            if nat_eq(a552, a553) {
                                                                                                                f69(v2, v3, v4)?
                                                                                                            } else {
                                                                                                                let a554 = v0;
                                                                                                                let a555 = 13u64;
                                                                                                                if if if nat_eq(a554, a555) {
                                                                                                                    true
                                                                                                                } else {
                                                                                                                    let a556 = v0;
                                                                                                                    let a557 = 14u64;
                                                                                                                    nat_eq(a556, a557)
                                                                                                                } {
                                                                                                                    true
                                                                                                                } else {
                                                                                                                    let a558 = v0;
                                                                                                                    let a559 = 16u64;
                                                                                                                    nat_eq(a558, a559)
                                                                                                                } {
                                                                                                                    f70(v0, v2, v4)?
                                                                                                                } else {
                                                                                                                    let m560 = f71(v0, v5)?;
                                                                                                                    match m560 {
                                                                                                                        None => {
                                                                                                                            f72(v0, v5, v2, v3, v4)?
                                                                                                                        }
                                                                                                                        Some(v12) => {
                                                                                                                            f73(v12, v2, v4)?
                                                                                                                        }
                                                                                                                    }
                                                                                                                }
                                                                                                            }
                                                                                                        }
                                                                                                    }
                                                                                                }
                                                                                            }
                                                                                        }
                                                                                    }
                                                                                }
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
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
        Adt8::C0(_, h561, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h561
        }
    };
    let v5: u64 = match v0.clone() {
        Adt8::C0(_, _, h562, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h562
        }
    };
    let v6: u64 = {
        let a563 = v3;
        let a564 = v5;
        let a565 = 0u64;
        nat_rem(a563, a564, a565)
    };
    let a566 = v1;
    let a567 = 0u64;
    if nat_eq(a566, a567) {
        let a569 = v3;
        let a570 = match v0.clone() {
            Adt8::C0(_, _, _, h568, _, _, _, _, _, _, _, _, _, _, _) => {
                h568
            }
        };
        if nat_lt(a569, a570) {
            Ok(Adt9::C8(Some(f74(match v0.clone() {
                Adt8::C0(_, _, _, _, _, _, _, _, h571, _, _, _, _, _, _) => {
                    h571
                }
            }, {
                let a575 = v6;
                let a576 = {
                    let m574 = match v0.clone() {
                        Adt8::C0(_, _, _, _, _, _, _, _, h573, _, _, _, _, _, _) => {
                            h573
                        }
                    };
                    match m574 {
                        Adt3::C0(h572, _, _, _, _, _, _, _) => {
                            h572
                        }
                    }
                };
                let a577 = 0u64;
                nat_rem(a575, a576, a577)
            }, {
                let a587 = {
                    let a581 = v6;
                    let a582 = {
                        let m580 = match v0.clone() {
                            Adt8::C0(_, _, _, _, _, _, _, _, h579, _, _, _, _, _, _) => {
                                h579
                            }
                        };
                        match m580 {
                            Adt3::C0(h578, _, _, _, _, _, _, _) => {
                                h578
                            }
                        }
                    };
                    let a583 = 0u64;
                    nat_quot(a581, a582, a583)
                };
                let a588 = {
                    let m586 = match v0.clone() {
                        Adt8::C0(_, _, _, _, _, _, _, _, h585, _, _, _, _, _, _) => {
                            h585
                        }
                    };
                    match m586 {
                        Adt3::C0(_, h584, _, _, _, _, _, _) => {
                            h584
                        }
                    }
                };
                let a589 = 0u64;
                nat_rem(a587, a588, a589)
            }, {
                let a599 = {
                    let a593 = v6;
                    let a594 = {
                        let m592 = match v0.clone() {
                            Adt8::C0(_, _, _, _, _, _, _, _, h591, _, _, _, _, _, _) => {
                                h591
                            }
                        };
                        match m592 {
                            Adt3::C0(h590, _, _, _, _, _, _, _) => {
                                h590
                            }
                        }
                    };
                    let a595 = 0u64;
                    nat_quot(a593, a594, a595)
                };
                let a600 = {
                    let m598 = match v0 {
                        Adt8::C0(_, _, _, _, _, _, _, _, h597, _, _, _, _, _, _) => {
                            h597
                        }
                    };
                    match m598 {
                        Adt3::C0(_, h596, _, _, _, _, _, _) => {
                            h596
                        }
                    }
                };
                let a601 = 0u64;
                nat_quot(a599, a600, a601)
            }, {
                let a602 = v3;
                let a603 = v5;
                let a604 = 0u64;
                nat_quot(a602, a603, a604)
            })?)))
        } else {
            Ok(Adt9::C0(Adt10::C1))
        }
    } else {
        let v7: u64 = {
            let a610 = v2;
            let a611 = {
                let a607 = v3;
                let a608 = {
                    let a605 = v1;
                    let a606 = 1u64;
                    if nat_eq(a605, a606) {
                        v4
                    } else {
                        v5
                    }
                };
                let a609 = 0u64;
                nat_quot(a607, a608, a609)
            };
            nat_add(a610, a611)?
        };
        let a613 = v7;
        let a614 = match v0.clone() {
            Adt8::C0(h612, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h612
            }
        };
        if nat_lt(a613, a614) {
            let v8: u64 = {
                let a618 = v7;
                let a619 = {
                    let m617 = match v0.clone() {
                        Adt8::C0(_, _, _, _, _, _, _, _, _, h616, _, _, _, _, _) => {
                            h616
                        }
                    };
                    match m617 {
                        Adt3::C0(h615, _, _, _, _, _, _, _) => {
                            h615
                        }
                    }
                };
                let a620 = 0u64;
                nat_rem(a618, a619, a620)
            };
            let v9: u64 = {
                let a624 = v7;
                let a625 = {
                    let m623 = match v0.clone() {
                        Adt8::C0(_, _, _, _, _, _, _, _, _, h622, _, _, _, _, _) => {
                            h622
                        }
                    };
                    match m623 {
                        Adt3::C0(h621, _, _, _, _, _, _, _) => {
                            h621
                        }
                    }
                };
                let a626 = 0u64;
                nat_quot(a624, a625, a626)
            };
            let v10: u64 = {
                let a630 = v9;
                let a631 = {
                    let m629 = match v0.clone() {
                        Adt8::C0(_, _, _, _, _, _, _, _, _, h628, _, _, _, _, _) => {
                            h628
                        }
                    };
                    match m629 {
                        Adt3::C0(_, h627, _, _, _, _, _, _) => {
                            h627
                        }
                    }
                };
                let a632 = 0u64;
                nat_rem(a630, a631, a632)
            };
            let v11: u64 = {
                let a636 = v9;
                let a637 = {
                    let m635 = match v0.clone() {
                        Adt8::C0(_, _, _, _, _, _, _, _, _, h634, _, _, _, _, _) => {
                            h634
                        }
                    };
                    match m635 {
                        Adt3::C0(_, h633, _, _, _, _, _, _) => {
                            h633
                        }
                    }
                };
                let a638 = 0u64;
                nat_quot(a636, a637, a638)
            };
            let a639 = v1;
            let a640 = 1u64;
            if nat_eq(a639, a640) {
                Ok(Adt9::C8(Some(f74(match v0 {
                    Adt8::C0(_, _, _, _, _, _, _, _, _, h641, _, _, _, _, _) => {
                        h641
                    }
                }, v8, v10, {
                    let a642 = v3;
                    let a643 = v4;
                    let a644 = 0u64;
                    nat_rem(a642, a643, a644)
                }, v11)?)))
            } else {
                let a645 = v1;
                let a646 = 2u64;
                if nat_eq(a645, a646) {
                    Ok(Adt9::C8(Some({
                        let a692 = f74(match v0.clone() {
                            Adt8::C0(_, _, _, _, _, _, _, _, _, _, h647, _, _, _, _) => {
                                h647
                            }
                        }, {
                            let a659 = {
                                let a651 = v8;
                                let a652 = {
                                    let m650 = match v0.clone() {
                                        Adt8::C0(_, _, _, _, _, _, _, _, _, _, _, h649, _, _, _) => {
                                            h649
                                        }
                                    };
                                    match m650 {
                                        Adt6::C0(_, h648, _, _) => {
                                            h648
                                        }
                                    }
                                };
                                nat_mul(a651, a652)?
                            };
                            let a660 = {
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
                                nat_rem(a656, a657, a658)
                            };
                            nat_add(a659, a660)?
                        }, {
                            let a678 = {
                                let a664 = v10;
                                let a665 = {
                                    let m663 = match v0.clone() {
                                        Adt8::C0(_, _, _, _, _, _, _, _, _, _, _, h662, _, _, _) => {
                                            h662
                                        }
                                    };
                                    match m663 {
                                        Adt6::C0(_, h661, _, _) => {
                                            h661
                                        }
                                    }
                                };
                                nat_mul(a664, a665)?
                            };
                            let a679 = {
                                let a675 = {
                                    let a669 = v6;
                                    let a670 = {
                                        let m668 = match v0.clone() {
                                            Adt8::C0(_, _, _, _, _, _, _, _, h667, _, _, _, _, _, _) => {
                                                h667
                                            }
                                        };
                                        match m668 {
                                            Adt3::C0(h666, _, _, _, _, _, _, _) => {
                                                h666
                                            }
                                        }
                                    };
                                    let a671 = 0u64;
                                    nat_quot(a669, a670, a671)
                                };
                                let a676 = {
                                    let m674 = match v0.clone() {
                                        Adt8::C0(_, _, _, _, _, _, _, _, h673, _, _, _, _, _, _) => {
                                            h673
                                        }
                                    };
                                    match m674 {
                                        Adt3::C0(_, h672, _, _, _, _, _, _) => {
                                            h672
                                        }
                                    }
                                };
                                let a677 = 0u64;
                                nat_rem(a675, a676, a677)
                            };
                            nat_add(a678, a679)?
                        }, {
                            let a689 = {
                                let a683 = v6;
                                let a684 = {
                                    let m682 = match v0.clone() {
                                        Adt8::C0(_, _, _, _, _, _, _, _, h681, _, _, _, _, _, _) => {
                                            h681
                                        }
                                    };
                                    match m682 {
                                        Adt3::C0(h680, _, _, _, _, _, _, _) => {
                                            h680
                                        }
                                    }
                                };
                                let a685 = 0u64;
                                nat_quot(a683, a684, a685)
                            };
                            let a690 = {
                                let m688 = match v0 {
                                    Adt8::C0(_, _, _, _, _, _, _, _, h687, _, _, _, _, _, _) => {
                                        h687
                                    }
                                };
                                match m688 {
                                    Adt3::C0(_, h686, _, _, _, _, _, _) => {
                                        h686
                                    }
                                }
                            };
                            let a691 = 0u64;
                            nat_quot(a689, a690, a691)
                        }, v11)?;
                        let a693 = 4u64;
                        let a694 = 0u64;
                        nat_quot(a692, a693, a694)
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
    let a695 = v1;
    let a696 = 0u64;
    if nat_eq(a695, a696) {
        let a698 = v3;
        let a699 = match v0.clone() {
            Adt8::C0(_, _, _, h697, _, _, _, _, _, _, _, _, _, _, _) => {
                h697
            }
        };
        if nat_lt(a698, a699) {
            let v4: Adt3 = match v0.clone() {
                Adt8::C0(_, _, _, _, _, _, _, _, h700, _, _, _, _, _, _) => {
                    h700
                }
            };
            let v5: u64 = {
                let a702 = v3;
                let a703 = match v0.clone() {
                    Adt8::C0(_, _, h701, _, _, _, _, _, _, _, _, _, _, _, _) => {
                        h701
                    }
                };
                let a704 = 0u64;
                nat_quot(a702, a703, a704)
            };
            Ok(Adt9::C8(Some(f74(v4.clone(), f75(v5, v4.clone(), 0u64), f75(v5, v4.clone(), 1u64), f75(v5, v4, 2u64), f76(v0.clone(), v5, {
                let a706 = v3;
                let a707 = match v0 {
                    Adt8::C0(_, _, h705, _, _, _, _, _, _, _, _, _, _, _, _) => {
                        h705
                    }
                };
                let a708 = 0u64;
                nat_rem(a706, a707, a708)
            })?)?)))
        } else {
            Ok(Adt9::C0(Adt10::C1))
        }
    } else {
        let v6: Adt3 = match v0.clone() {
            Adt8::C0(_, _, _, _, _, _, _, _, _, _, h709, _, _, _, _) => {
                h709
            }
        };
        let v7: u64 = {
            let a710 = v1;
            let a711 = 1u64;
            if nat_eq(a710, a711) {
                match v0.clone() {
                    Adt8::C0(_, h712, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                        h712
                    }
                }
            } else {
                match v0.clone() {
                    Adt8::C0(_, _, h713, _, _, _, _, _, _, _, _, _, _, _, _) => {
                        h713
                    }
                }
            }
        };
        let v8: u64 = {
            let a717 = v2;
            let a718 = {
                let a714 = v3;
                let a715 = v7;
                let a716 = 0u64;
                nat_quot(a714, a715, a716)
            };
            nat_add(a717, a718)?
        };
        let v9: u64 = {
            let a719 = v3;
            let a720 = v7;
            let a721 = 0u64;
            nat_rem(a719, a720, a721)
        };
        let v10: u64 = {
            let a723 = v8;
            let a724 = match v6.clone() {
                Adt3::C0(h722, _, _, _, _, _, _, _) => {
                    h722
                }
            };
            let a725 = 0u64;
            nat_rem(a723, a724, a725)
        };
        let v11: u64 = {
            let a727 = v8;
            let a728 = match v6.clone() {
                Adt3::C0(h726, _, _, _, _, _, _, _) => {
                    h726
                }
            };
            let a729 = 0u64;
            nat_quot(a727, a728, a729)
        };
        let v12: u64 = {
            let a731 = v11;
            let a732 = match v6.clone() {
                Adt3::C0(_, h730, _, _, _, _, _, _) => {
                    h730
                }
            };
            let a733 = 0u64;
            nat_rem(a731, a732, a733)
        };
        let v13: u64 = {
            let a735 = v11;
            let a736 = match v6.clone() {
                Adt3::C0(_, h734, _, _, _, _, _, _) => {
                    h734
                }
            };
            let a737 = 0u64;
            nat_quot(a735, a736, a737)
        };
        let a739 = v8;
        let a740 = match v0.clone() {
            Adt8::C0(h738, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h738
            }
        };
        if nat_lt(a739, a740) {
            let a741 = v1;
            let a742 = 1u64;
            if nat_eq(a741, a742) {
                let v14: Adt3 = match v0.clone() {
                    Adt8::C0(_, _, _, _, _, _, _, _, h743, _, _, _, _, _, _) => {
                        h743
                    }
                };
                let v15: Adt3 = match v0.clone() {
                    Adt8::C0(_, _, _, _, _, _, _, _, _, h744, _, _, _, _, _) => {
                        h744
                    }
                };
                let m747 = f77(match v0.clone() {
                    Adt8::C0(_, _, _, _, _, _, _, _, _, _, _, h745, _, _, _) => {
                        h745
                    }
                }, match v15.clone() {
                    Adt3::C0(h746, _, _, _, _, _, _, _) => {
                        h746
                    }
                }, v10, f75(v9, v14.clone(), 0u64))?;
                match m747 {
                    None => {
                        Ok(Adt9::C8(None::<u64>))
                    }
                    Some(v16) => {
                        let m750 = f77(match v0.clone() {
                            Adt8::C0(_, _, _, _, _, _, _, _, _, _, _, _, h748, _, _) => {
                                h748
                            }
                        }, match v15 {
                            Adt3::C0(_, h749, _, _, _, _, _, _) => {
                                h749
                            }
                        }, v12, f75(v9, v14, 1u64))?;
                        match m750 {
                            None => {
                                Ok(Adt9::C8(None::<u64>))
                            }
                            Some(v17) => {
                                f78(v0, v9, v13, v16, v17)
                            }
                        }
                    }
                }
            } else {
                let a751 = v1;
                let a752 = 2u64;
                if nat_eq(a751, a752) {
                    Ok(Adt9::C8(Some({
                        let a754 = match v0.clone() {
                            Adt8::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, h753) => {
                                h753
                            }
                        };
                        let a755 = 0u64;
                        if nat_eq(a754, a755) {
                            f74(v6, v10, v12, v9, v13)?
                        } else {
                            f74(v6.clone(), v10, v12, {
                                let a757 = v13;
                                let a758 = match v6.clone() {
                                    Adt3::C0(_, _, h756, _, _, _, _, _) => {
                                        h756
                                    }
                                };
                                let a759 = 0u64;
                                nat_rem(a757, a758, a759)
                            }, {
                                let a767 = {
                                    let a765 = {
                                        let a761 = v13;
                                        let a762 = match v6 {
                                            Adt3::C0(_, _, h760, _, _, _, _, _) => {
                                                h760
                                            }
                                        };
                                        let a763 = 0u64;
                                        nat_quot(a761, a762, a763)
                                    };
                                    let a766 = match v0 {
                                        Adt8::C0(_, _, h764, _, _, _, _, _, _, _, _, _, _, _, _) => {
                                            h764
                                        }
                                    };
                                    nat_mul(a765, a766)?
                                };
                                let a768 = v9;
                                nat_add(a767, a768)?
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
    Ok(f79(v3.clone(), {
        let m769 = v3.clone();
        match m769 {
            Adt4::C0 => {
                f80(v0, v1, v2, v3)?
            }
            Adt4::C1 => {
                f80(v0, v1, v2, v3)?
            }
            Adt4::C2(_, _) => {
                f80(v0, v1, v2, v3)?
            }
            Adt4::C3(_, _, _, _, _) => {
                f80(v0, v1, v2, v3)?
            }
            Adt4::C4(_) => {
                f80(v0, v1, v2, v3)?
            }
            Adt4::C5(_, _, _, _) => {
                f80(v0, v1, v2, v3)?
            }
            Adt4::C6(_, _, _) => {
                f80(v0, v1, v2, v3)?
            }
            Adt4::C7(_) => {
                f80(v0, v1, v2, v3)?
            }
            Adt4::C8(_) => {
                f80(v0, v1, v2, v3)?
            }
            Adt4::C9(_, _) => {
                f80(v0, v1, v2, v3)?
            }
            Adt4::C10(_, _, _) => {
                f80(v0, v1, v2, v3)?
            }
            Adt4::C11(_, _) => {
                f80(v0, v1, v2, v3)?
            }
            Adt4::C12(_, _, _) => {
                f80(v0, v1, v2, v3)?
            }
            Adt4::C13(_, _, _, _) => {
                f80(v0, v1, v2, v3)?
            }
            Adt4::C14(_, _, _) => {
                f80(v0, v1, v2, v3)?
            }
            Adt4::C15 => {
                f80(v0, v1, v2, v3)?
            }
            Adt4::C16(_, _) => {
                f80(v0, v1, v2, v3)?
            }
            Adt4::C17 => {
                f87(v0, v1, v2, v4)?
            }
            Adt4::C18(_, _) => {
                f80(v0, v1, v2, v3)?
            }
            Adt4::C19(v61, v62) => {
                f88(v61, v0, v1, v2, v62, v4)?
            }
            Adt4::C20(v63, v64) => {
                f89(v63, v0, v1, v2, v64, v4)?
            }
            Adt4::C21 => {
                f80(v0, v1, v2, v3)?
            }
            Adt4::C22(v65, v66) => {
                f90(v0, v1, v2, v65, v66, v4)?
            }
            Adt4::C23(v80, v81, v82, v83) => {
                f97(v0, v83, v80, v81, v82, v4)?
            }
            Adt4::C24(v79) => {
                if v5 {
                    f95(Adt16::C0(v79), v0, v1, v2, v4, v6, v7, v8, v9, v10)?
                } else {
                    Adt9::C0(Adt10::C5(65u64))
                }
            }
            Adt4::C25(_, _, _) => {
                f80(v0, v1, v2, v3)?
            }
            Adt4::C26(v75, v76, v77, v78) => {
                let a771 = v75;
                let a772 = 64u64;
                if if if nat_eq(a771, a772) {
                    true
                } else {
                    let a773 = v75;
                    let a774 = 65u64;
                    nat_eq(a773, a774)
                } {
                    true
                } else {
                    let a775 = v75;
                    let a776 = 66u64;
                    nat_eq(a775, a776)
                } {
                    if v5 {
                        f95(Adt16::C1(v75, v76, v77, v78), v0, v1, v2, v4, v6, v7, v8, v9, v10)?
                    } else {
                        Adt9::C0(Adt10::C5(64u64))
                    }
                } else {
                    let a777 = v75;
                    let a778 = 0u64;
                    if if nat_eq(a777, a778) {
                        true
                    } else {
                        let a779 = v75;
                        let a780 = 30u64;
                        nat_eq(a779, a780)
                    } {
                        f96(v0, v1, v2, v75, v76, v77, v78, v4)?
                    } else {
                        Adt9::C0(Adt10::C1)
                    }
                }
            }
            Adt4::C27(v74) => {
                f94(v0, v1, v2, v74)?
            }
            Adt4::C28(v70) => {
                f91(v0, v1, v2, v70, v4)?
            }
            Adt4::C29(v71) => {
                f92(v0, v1, v2, v71)?
            }
            Adt4::C30(v72, _) => {
                f93(v0, v1, v2, v72)?
            }
            Adt4::C31(v33, v34) => {
                f81(v0, v1, v2, v33, v34)?
            }
            Adt4::C32(v35) => {
                f82(v0, match v1 {
                    Adt3::C0(h770, _, _, _, _, _, _, _) => {
                        h770
                    }
                }, v35)?
            }
            Adt4::C33(v36) => {
                f83(v0, v1, v36, v2)?
            }
            Adt4::C34(v41, v42, v43, v44) => {
                f86(v0, v1, v41, v2, v42, v43, v44)?
            }
            Adt4::C35(v40) => {
                f85(v0, v40, v1, v2)?
            }
            Adt4::C36(v37, v38, v39) => {
                f84(v0, v1, v37, v2, v38, v39)?
            }
        }
    }))
}

pub fn f14(v0: Adt4) -> R<Adt9> {
    let m781 = v0;
    match m781 {
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
        Adt4::C25(v57, _, v59) => {
            Ok(Adt9::C6({
                let a786 = {
                    let a782 = 0u64;
                    let a783 = v57;
                    if nat_lt(a782, a783) {
                        1u64
                    } else {
                        0u64
                    }
                };
                let a787 = {
                    let a784 = 0u64;
                    let a785 = v59;
                    if nat_lt(a784, a785) {
                        6u64
                    } else {
                        0u64
                    }
                };
                nat_add(a786, a787)?
            }, 0u64, 0u64, false))
        }
        Adt4::C26(v65, _, _, _) => {
            Ok(Adt9::C6({
                let a788 = v65;
                let a789 = 66u64;
                if nat_eq(a788, a789) {
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
    }
}

pub fn f15(v0: bool, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64, v7: u64, v8: u64, v9: u64, v10: u64, v11: u64, v12: u64) -> Adt9 {
    let a790 = v1;
    let a791 = 5u64;
    if if if if if nat_lt(a790, a791) {
        true
    } else {
        f98(v0, v1, v2, v3, v4)
    } {
        let a792 = v5;
        let a793 = 5u64;
        nat_lt(a792, a793)
    } else {
        false
    } {
        let a794 = v7;
        let a795 = 5u64;
        nat_lt(a794, a795)
    } else {
        false
    } {
        let a796 = v9;
        let a797 = 2u64;
        if nat_lt(a796, a797) {
            true
        } else {
            let a798 = v9;
            let a799 = 3u64;
            nat_eq(a798, a799)
        }
    } else {
        false
    } {
        Adt9::C7(f99(v0, v1, v2, v3, v4), f100(v5, v6), f100(v7, v8), f101({
            let a800 = v9;
            let a801 = 0u64;
            if nat_eq(a800, a801) {
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
    let a813 = {
        let a807 = {
            let a803 = match v0.clone() {
                Adt2::C0(_, h802, _, _, _, _, _, _, _, _, _) => {
                    h802
                }
            };
            let a804 = v1;
            let a805 = 0u64;
            nat_quot(a803, a804, a805)
        };
        let a808 = match v0.clone() {
            Adt2::C0(_, _, h806, _, _, _, _, _, _, _, _) => {
                h806
            }
        };
        nat_mul(a807, a808)?
    };
    let a814 = {
        let a811 = match v0.clone() {
            Adt2::C0(_, _, _, h809, _, _, _, _, _, _, _) => {
                h809
            }
        };
        let a812 = match v0 {
            Adt2::C0(_, _, _, _, h810, _, _, _, _, _, _) => {
                h810
            }
        };
        nat_mul(a811, a812)?
    };
    nat_mul(a813, a814)
}

pub fn f17(v0: u64) -> bool {
    let a815 = v0;
    let a816 = 4294967296u64;
    if nat_lt(a815, a816) {
        let a820 = {
            let a817 = v0;
            let a818 = 2147483648u64;
            let a819 = 0u64;
            nat_rem(a817, a818, a819)
        };
        let a821 = 2139095040u64;
        nat_lt(a820, a821)
    } else {
        false
    }
}

pub fn f18(v0: u64) -> R<u64> {
    let a822 = 4u64;
    let a823 = v0;
    nat_mul(a822, a823)
}

pub fn f19(v0: u64) -> R<u64> {
    let a824 = v0;
    let a825 = 0u64;
    if nat_eq(a824, a825) {
        Ok(0u64)
    } else {
        let a828 = 4u64;
        let a829 = {
            let a826 = 3u64;
            let a827 = v0;
            nat_mul(a826, a827)?
        };
        nat_add(a828, a829)
    }
}

pub fn f20(v0: Adt2, v1: u64) -> Adt3 {
    Adt3::C0({
        let a831 = match v0.clone() {
            Adt2::C0(_, h830, _, _, _, _, _, _, _, _, _) => {
                h830
            }
        };
        let a832 = v1;
        let a833 = 0u64;
        nat_quot(a831, a832, a833)
    }, match v0.clone() {
        Adt2::C0(_, _, h834, _, _, _, _, _, _, _, _) => {
            h834
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, h835, _, _, _, _, _, _, _) => {
            h835
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, h836, _, _, _, _, _, _) => {
            h836
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, _, h837, _, _, _, _, _) => {
            h837
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, _, _, h838, _, _, _, _) => {
            h838
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, _, _, _, h839, _, _, _) => {
            h839
        }
    }, match v0 {
        Adt2::C0(_, _, _, _, _, _, _, _, h840, _, _) => {
            h840
        }
    })
}

pub fn f21(v0: bool, v1: Adt2) -> Adt3 {
    if v0 {
        f20(v1, 1u64)
    } else {
        Adt3::C0(match v1.clone() {
            Adt2::C0(_, h841, _, _, _, _, _, _, _, _, _) => {
                h841
            }
        }, 1u64, match v1.clone() {
            Adt2::C0(_, _, h842, _, _, _, _, _, _, _, _) => {
                h842
            }
        }, match v1.clone() {
            Adt2::C0(_, _, _, h843, _, _, _, _, _, _, _) => {
                h843
            }
        }, match v1.clone() {
            Adt2::C0(_, _, _, _, _, h844, _, _, _, _, _) => {
                h844
            }
        }, 0u64, match v1.clone() {
            Adt2::C0(_, _, _, _, _, _, h845, _, _, _, _) => {
                h845
            }
        }, match v1 {
            Adt2::C0(_, _, _, _, _, _, _, h846, _, _, _) => {
                h846
            }
        })
    }
}

pub fn f22(v0: Adt2) -> R<bool> {
    let a848 = match v0.clone() {
        Adt2::C0(_, h847, _, _, _, _, _, _, _, _, _) => {
            h847
        }
    };
    let a849 = 1u64;
    if if if if nat_eq(a848, a849) {
        true
    } else {
        let a851 = match v0.clone() {
            Adt2::C0(_, _, _, _, _, h850, _, _, _, _, _) => {
                h850
            }
        };
        let a852 = 4u64;
        nat_eq(a851, a852)
    } {
        let a854 = match v0.clone() {
            Adt2::C0(_, _, h853, _, _, _, _, _, _, _, _) => {
                h853
            }
        };
        let a855 = 1u64;
        if nat_eq(a854, a855) {
            true
        } else {
            let a860 = match v0.clone() {
                Adt2::C0(_, _, _, _, _, _, h856, _, _, _, _) => {
                    h856
                }
            };
            let a861 = {
                let a858 = match v0.clone() {
                    Adt2::C0(_, h857, _, _, _, _, _, _, _, _, _) => {
                        h857
                    }
                };
                let a859 = 4u64;
                nat_mul(a858, a859)?
            };
            nat_eq(a860, a861)
        }
    } else {
        false
    } {
        let a863 = match v0.clone() {
            Adt2::C0(_, _, _, h862, _, _, _, _, _, _, _) => {
                h862
            }
        };
        let a864 = 1u64;
        if nat_eq(a863, a864) {
            true
        } else {
            let a872 = match v0.clone() {
                Adt2::C0(_, _, _, _, _, _, _, h865, _, _, _) => {
                    h865
                }
            };
            let a873 = {
                let a870 = {
                    let a868 = match v0.clone() {
                        Adt2::C0(_, h866, _, _, _, _, _, _, _, _, _) => {
                            h866
                        }
                    };
                    let a869 = match v0.clone() {
                        Adt2::C0(_, _, h867, _, _, _, _, _, _, _, _) => {
                            h867
                        }
                    };
                    nat_mul(a868, a869)?
                };
                let a871 = 4u64;
                nat_mul(a870, a871)?
            };
            nat_eq(a872, a873)
        }
    } else {
        false
    } {
        let a875 = match v0.clone() {
            Adt2::C0(_, _, _, _, h874, _, _, _, _, _, _) => {
                h874
            }
        };
        let a876 = 1u64;
        if nat_eq(a875, a876) {
            Ok(true)
        } else {
            let a887 = match v0.clone() {
                Adt2::C0(_, _, _, _, _, _, _, _, h877, _, _) => {
                    h877
                }
            };
            let a888 = {
                let a885 = {
                    let a883 = {
                        let a880 = match v0.clone() {
                            Adt2::C0(_, h878, _, _, _, _, _, _, _, _, _) => {
                                h878
                            }
                        };
                        let a881 = match v0.clone() {
                            Adt2::C0(_, _, h879, _, _, _, _, _, _, _, _) => {
                                h879
                            }
                        };
                        nat_mul(a880, a881)?
                    };
                    let a884 = match v0 {
                        Adt2::C0(_, _, _, h882, _, _, _, _, _, _, _) => {
                            h882
                        }
                    };
                    nat_mul(a883, a884)?
                };
                let a886 = 4u64;
                nat_mul(a885, a886)?
            };
            Ok(nat_eq(a887, a888))
        }
    } else {
        Ok(false)
    }
}

pub fn f23(v0: Adt2, v1: Adt2) -> R<bool> {
    let a890 = match v0.clone() {
        Adt2::C0(_, _, _, _, _, _, _, _, _, h889, _) => {
            h889
        }
    };
    let a891 = 0u64;
    if nat_eq(a890, a891) {
        Ok(true)
    } else {
        let a893 = match v1.clone() {
            Adt2::C0(_, _, _, _, _, _, _, _, _, h892, _) => {
                h892
            }
        };
        let a894 = 0u64;
        if nat_eq(a893, a894) {
            Ok(true)
        } else {
            let a896 = match v0.clone() {
                Adt2::C0(_, _, _, _, _, _, _, _, _, _, h895) => {
                    h895
                }
            };
            let a897 = 0u64;
            if nat_eq(a896, a897) {
                Ok(true)
            } else {
                let a899 = match v1.clone() {
                    Adt2::C0(_, _, _, _, _, _, _, _, _, _, h898) => {
                        h898
                    }
                };
                let a900 = 0u64;
                if nat_eq(a899, a900) {
                    Ok(true)
                } else {
                    let a906 = {
                        let a903 = match v0.clone() {
                            Adt2::C0(_, _, _, _, _, _, _, _, _, _, h901) => {
                                h901
                            }
                        };
                        let a904 = match v0.clone() {
                            Adt2::C0(_, _, _, _, _, _, _, _, _, h902, _) => {
                                h902
                            }
                        };
                        nat_add(a903, a904)?
                    };
                    let a907 = match v1.clone() {
                        Adt2::C0(_, _, _, _, _, _, _, _, _, _, h905) => {
                            h905
                        }
                    };
                    if nat_le(a906, a907) {
                        Ok(true)
                    } else {
                        let a913 = {
                            let a910 = match v1.clone() {
                                Adt2::C0(_, _, _, _, _, _, _, _, _, _, h908) => {
                                    h908
                                }
                            };
                            let a911 = match v1 {
                                Adt2::C0(_, _, _, _, _, _, _, _, _, h909, _) => {
                                    h909
                                }
                            };
                            nat_add(a910, a911)?
                        };
                        let a914 = match v0 {
                            Adt2::C0(_, _, _, _, _, _, _, _, _, _, h912) => {
                                h912
                            }
                        };
                        Ok(nat_le(a913, a914))
                    }
                }
            }
        }
    }
}

pub fn f24(v0: Adt6, v1: u64) -> R<u64> {
    let a916 = match v0.clone() {
        Adt6::C0(h915, _, _, _) => {
            h915
        }
    };
    let a917 = 0u64;
    if nat_eq(a916, a917) {
        f102(Adt6::C0(1u64, match v0.clone() {
            Adt6::C0(_, h918, _, _) => {
                h918
            }
        }, match v0.clone() {
            Adt6::C0(_, _, h919, _) => {
                h919
            }
        }, match v0.clone() {
            Adt6::C0(_, _, _, h920) => {
                h920
            }
        }), {
            let a922 = v1;
            let a923 = match v0 {
                Adt6::C0(_, _, _, h921) => {
                    h921
                }
            };
            nat_add(a922, a923)?
        })
    } else {
        f102(v0, v1)
    }
}

pub fn f25(v0: Adt2, v1: Adt2) -> bool {
    let a926 = match v0.clone() {
        Adt2::C0(_, h924, _, _, _, _, _, _, _, _, _) => {
            h924
        }
    };
    let a927 = match v1.clone() {
        Adt2::C0(_, h925, _, _, _, _, _, _, _, _, _) => {
            h925
        }
    };
    if if if nat_eq(a926, a927) {
        let a930 = match v0.clone() {
            Adt2::C0(_, _, h928, _, _, _, _, _, _, _, _) => {
                h928
            }
        };
        let a931 = match v1.clone() {
            Adt2::C0(_, _, h929, _, _, _, _, _, _, _, _) => {
                h929
            }
        };
        nat_eq(a930, a931)
    } else {
        false
    } {
        let a934 = match v0.clone() {
            Adt2::C0(_, _, _, h932, _, _, _, _, _, _, _) => {
                h932
            }
        };
        let a935 = match v1.clone() {
            Adt2::C0(_, _, _, h933, _, _, _, _, _, _, _) => {
                h933
            }
        };
        nat_eq(a934, a935)
    } else {
        false
    } {
        let a938 = match v0 {
            Adt2::C0(_, _, _, _, h936, _, _, _, _, _, _) => {
                h936
            }
        };
        let a939 = match v1 {
            Adt2::C0(_, _, _, _, h937, _, _, _, _, _, _) => {
                h937
            }
        };
        nat_eq(a938, a939)
    } else {
        false
    }
}

pub fn f26(v0: u64, v1: u64, v2: u64, v3: u64) -> R<u64> {
    let a944 = {
        let a940 = v0;
        let a941 = v1;
        nat_mul(a940, a941)?
    };
    let a945 = {
        let a942 = v2;
        let a943 = f103(v3);
        nat_mul(a942, a943)?
    };
    nat_mul(a944, a945)
}

pub fn f27(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let a948 = v0;
    let a949 = {
        let a946 = f104(v1)?;
        let a947 = v2;
        nat_mul(a946, a947)?
    };
    nat_add(a948, a949)
}

pub fn f28(v0: Adt2) -> Adt14 {
    Adt14::C0(match v0.clone() {
        Adt2::C0(h950, _, _, _, _, _, _, _, _, _, _) => {
            h950
        }
    }, match v0.clone() {
        Adt2::C0(_, h951, _, _, _, _, _, _, _, _, _) => {
            h951
        }
    }, match v0.clone() {
        Adt2::C0(_, _, h952, _, _, _, _, _, _, _, _) => {
            h952
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, h953, _, _, _, _, _, _, _) => {
            h953
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, h954, _, _, _, _, _, _) => {
            h954
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, _, h955, _, _, _, _, _) => {
            h955
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, _, _, h956, _, _, _, _) => {
            h956
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, _, _, _, h957, _, _, _) => {
            h957
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, _, _, _, _, h958, _, _) => {
            h958
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, _, _, _, _, _, h959, _) => {
            h959
        }
    }, match v0 {
        Adt2::C0(_, _, _, _, _, _, _, _, _, _, h960) => {
            h960
        }
    })
}

pub fn f29(v0: u64, v1: u64, v2: u64, v3: bool, v4: Adt14, v5: Adt14, v6: Adt14) -> R<Adt15> {
    let a961 = v0;
    let a962 = 23u64;
    if nat_eq(a961, a962) {
        f105(v1, v4, v6)
    } else {
        let a963 = v0;
        let a964 = 26u64;
        if nat_eq(a963, a964) {
            f106(v1, v2, v4, v6)
        } else {
            let a965 = v0;
            let a966 = 25u64;
            if nat_eq(a965, a966) {
                f107(v1, v3, v4, v5, v6)
            } else {
                Ok(Adt15::C0)
            }
        }
    }
}

pub fn f30(v0: u64) -> u64 {
    let a967 = v0;
    let a968 = 30u64;
    if nat_eq(a967, a968) {
        2u64
    } else {
        4u64
    }
}

pub fn f31(v0: u64) -> bool {
    let a969 = v0;
    let a970 = 2139095040u64;
    if nat_le(a969, a970) {
        true
    } else {
        let a971 = v0;
        let a972 = 2147483648u64;
        nat_eq(a971, a972)
    }
}

pub fn f32(v0: Adt2, v1: u64) -> bool {
    let a974 = match v0.clone() {
        Adt2::C0(_, _, _, _, _, h973, _, _, _, _, _) => {
            h973
        }
    };
    let a975 = v1;
    if if nat_eq(a974, a975) {
        let a980 = {
            let a977 = match v0.clone() {
                Adt2::C0(_, _, _, _, _, _, _, _, _, _, h976) => {
                    h976
                }
            };
            let a978 = v1;
            let a979 = 0u64;
            nat_rem(a977, a978, a979)
        };
        let a981 = 0u64;
        nat_eq(a980, a981)
    } else {
        false
    } {
        f108(match v0.clone() {
            Adt2::C0(_, _, h982, _, _, _, _, _, _, _, _) => {
                h982
            }
        }, match v0.clone() {
            Adt2::C0(_, _, _, h983, _, _, _, _, _, _, _) => {
                h983
            }
        }, match v0.clone() {
            Adt2::C0(_, _, _, _, h984, _, _, _, _, _, _) => {
                h984
            }
        }, match v0.clone() {
            Adt2::C0(_, _, _, _, _, _, h985, _, _, _, _) => {
                h985
            }
        }, match v0.clone() {
            Adt2::C0(_, _, _, _, _, _, _, h986, _, _, _) => {
                h986
            }
        }, match v0 {
            Adt2::C0(_, _, _, _, _, _, _, _, h987, _, _) => {
                h987
            }
        }, v1)
    } else {
        false
    }
}

pub fn f33(v0: Adt2, v1: Adt2) -> bool {
    if if if if if if f25(v0.clone(), v1.clone()) {
        let a990 = match v0.clone() {
            Adt2::C0(h988, _, _, _, _, _, _, _, _, _, _) => {
                h988
            }
        };
        let a991 = match v1.clone() {
            Adt2::C0(h989, _, _, _, _, _, _, _, _, _, _) => {
                h989
            }
        };
        nat_eq(a990, a991)
    } else {
        false
    } {
        let a994 = match v0.clone() {
            Adt2::C0(_, _, _, _, _, _, _, _, _, _, h992) => {
                h992
            }
        };
        let a995 = match v1.clone() {
            Adt2::C0(_, _, _, _, _, _, _, _, _, _, h993) => {
                h993
            }
        };
        nat_eq(a994, a995)
    } else {
        false
    } {
        let a998 = match v0.clone() {
            Adt2::C0(_, _, _, _, _, h996, _, _, _, _, _) => {
                h996
            }
        };
        let a999 = match v1.clone() {
            Adt2::C0(_, _, _, _, _, h997, _, _, _, _, _) => {
                h997
            }
        };
        nat_eq(a998, a999)
    } else {
        false
    } {
        let a1002 = match v0.clone() {
            Adt2::C0(_, _, _, _, _, _, h1000, _, _, _, _) => {
                h1000
            }
        };
        let a1003 = match v1.clone() {
            Adt2::C0(_, _, _, _, _, _, h1001, _, _, _, _) => {
                h1001
            }
        };
        nat_eq(a1002, a1003)
    } else {
        false
    } {
        let a1006 = match v0.clone() {
            Adt2::C0(_, _, _, _, _, _, _, h1004, _, _, _) => {
                h1004
            }
        };
        let a1007 = match v1.clone() {
            Adt2::C0(_, _, _, _, _, _, _, h1005, _, _, _) => {
                h1005
            }
        };
        nat_eq(a1006, a1007)
    } else {
        false
    } {
        let a1010 = match v0 {
            Adt2::C0(_, _, _, _, _, _, _, _, h1008, _, _) => {
                h1008
            }
        };
        let a1011 = match v1 {
            Adt2::C0(_, _, _, _, _, _, _, _, h1009, _, _) => {
                h1009
            }
        };
        nat_eq(a1010, a1011)
    } else {
        false
    }
}

pub fn f34(v0: u64, v1: u64, v2: u64, v3: u64) -> bool {
    let a1012 = v0;
    let a1013 = 0u64;
    if if if nat_eq(a1012, a1013) {
        true
    } else {
        let a1014 = v1;
        let a1015 = 0u64;
        nat_eq(a1014, a1015)
    } {
        true
    } else {
        let a1016 = v2;
        let a1017 = 0u64;
        nat_eq(a1016, a1017)
    } {
        true
    } else {
        let a1018 = v3;
        let a1019 = 0u64;
        nat_eq(a1018, a1019)
    }
}

pub fn f35(v0: Adt2, v1: u64) -> R<bool> {
    f109({
        let a1021 = match v0.clone() {
            Adt2::C0(_, h1020, _, _, _, _, _, _, _, _, _) => {
                h1020
            }
        };
        let a1022 = v1;
        nat_mul(a1021, a1022)?
    }, match v0.clone() {
        Adt2::C0(_, _, h1023, _, _, _, _, _, _, _, _) => {
            h1023
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, h1024, _, _, _, _, _, _, _) => {
            h1024
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, h1025, _, _, _, _, _, _) => {
            h1025
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, _, _, h1026, _, _, _, _) => {
            h1026
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, _, _, _, h1027, _, _, _) => {
            h1027
        }
    }, match v0 {
        Adt2::C0(_, _, _, _, _, _, _, _, h1028, _, _) => {
            h1028
        }
    })
}

pub fn f36(v0: u64, v1: u64, v2: u64, v3: u64) -> R<u64> {
    let a1029 = v1;
    let a1030 = 0u64;
    if if nat_eq(a1029, a1030) {
        true
    } else {
        let a1031 = v2;
        let a1032 = 0u64;
        nat_eq(a1031, a1032)
    } {
        Ok(0u64)
    } else {
        let a1039 = {
            let a1033 = f110(v1, v2, v3)?;
            let a1034 = f111(v0, v1, v2)?;
            nat_add(a1033, a1034)?
        };
        let a1040 = {
            let a1037 = f112(v0);
            let a1038 = {
                let a1035 = v1;
                let a1036 = v2;
                nat_mul(a1035, a1036)?
            };
            nat_mul(a1037, a1038)?
        };
        nat_add(a1039, a1040)
    }
}

pub fn f37(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let a1041 = v0;
    let a1042 = 0u64;
    if if nat_eq(a1041, a1042) {
        true
    } else {
        let a1043 = v1;
        let a1044 = 0u64;
        nat_eq(a1043, a1044)
    } {
        Ok(3u64)
    } else {
        let a1047 = {
            let a1045 = 3u64;
            let a1046 = v1;
            nat_add(a1045, a1046)?
        };
        let a1048 = v2;
        nat_add(a1047, a1048)
    }
}

pub fn f38(v0: u64) -> bool {
    let a1049 = v0;
    let a1050 = 2139095040u64;
    if nat_le(a1049, a1050) {
        true
    } else {
        let a1051 = v0;
        let a1052 = 2147483648u64;
        nat_eq(a1051, a1052)
    }
}

pub fn f39(v0: u64, v1: u64) -> R<u64> {
    let a1053 = v0;
    let a1054 = 0u64;
    if nat_eq(a1053, a1054) {
        Ok(0u64)
    } else {
        let a1055 = v1;
        let a1056 = f113(v0)?;
        nat_mul(a1055, a1056)
    }
}

pub fn f40(v0: u64, v1: u64, v2: u64, v3: u64) -> R<u64> {
    let a1057 = v0;
    let a1058 = 0u64;
    if if if if nat_eq(a1057, a1058) {
        true
    } else {
        let a1059 = v1;
        let a1060 = 0u64;
        nat_eq(a1059, a1060)
    } {
        true
    } else {
        let a1061 = v2;
        let a1062 = 0u64;
        nat_eq(a1061, a1062)
    } {
        true
    } else {
        let a1063 = v3;
        let a1064 = 0u64;
        nat_eq(a1063, a1064)
    } {
        Ok(0u64)
    } else {
        let a1077 = {
            let a1067 = 2u64;
            let a1068 = {
                let a1065 = v1;
                let a1066 = v2;
                nat_mul(a1065, a1066)?
            };
            nat_mul(a1067, a1068)?
        };
        let a1078 = {
            let a1075 = {
                let a1071 = {
                    let a1069 = v1;
                    let a1070 = v2;
                    nat_mul(a1069, a1070)?
                };
                let a1072 = v3;
                nat_mul(a1071, a1072)?
            };
            let a1076 = {
                let a1073 = 1u64;
                let a1074 = v0;
                nat_add(a1073, a1074)?
            };
            nat_mul(a1075, a1076)?
        };
        nat_add(a1077, a1078)
    }
}

pub fn f41(v0: Adt1, v1: Adt2, v2: Adt2, v3: Adt2) -> R<Adt10> {
    let v4: bool = {
        let a1080 = 0u64;
        let a1081 = match v0.clone() {
            Adt1::C0(_, h1079, _, _, _, _, _, _, _, _, _, _) => {
                h1079
            }
        };
        nat_lt(a1080, a1081)
    };
    let a1083 = match v1.clone() {
        Adt2::C0(h1082, _, _, _, _, _, _, _, _, _, _) => {
            h1082
        }
    };
    let a1084 = 0u64;
    if if if if if if if if if if if if if if if if nat_eq(a1083, a1084) {
        let a1086 = match v2.clone() {
            Adt2::C0(h1085, _, _, _, _, _, _, _, _, _, _) => {
                h1085
            }
        };
        let a1087 = 0u64;
        nat_eq(a1086, a1087)
    } else {
        false
    } {
        let a1089 = match v3.clone() {
            Adt2::C0(h1088, _, _, _, _, _, _, _, _, _, _) => {
                h1088
            }
        };
        let a1090 = 0u64;
        nat_eq(a1089, a1090)
    } else {
        false
    } {
        let a1092 = match v1.clone() {
            Adt2::C0(_, _, _, _, h1091, _, _, _, _, _, _) => {
                h1091
            }
        };
        let a1093 = 1u64;
        nat_eq(a1092, a1093)
    } else {
        false
    } {
        let a1095 = match v2.clone() {
            Adt2::C0(_, _, _, _, h1094, _, _, _, _, _, _) => {
                h1094
            }
        };
        let a1096 = 1u64;
        nat_eq(a1095, a1096)
    } else {
        false
    } {
        let a1098 = match v3.clone() {
            Adt2::C0(_, _, _, _, h1097, _, _, _, _, _, _) => {
                h1097
            }
        };
        let a1099 = 1u64;
        nat_eq(a1098, a1099)
    } else {
        false
    } {
        let a1101 = 0u64;
        let a1102 = match v1.clone() {
            Adt2::C0(_, _, h1100, _, _, _, _, _, _, _, _) => {
                h1100
            }
        };
        nat_lt(a1101, a1102)
    } else {
        false
    } {
        let a1104 = match v3.clone() {
            Adt2::C0(_, _, _, h1103, _, _, _, _, _, _, _) => {
                h1103
            }
        };
        let a1105 = 1u64;
        nat_eq(a1104, a1105)
    } else {
        false
    } {
        let a1108 = match v3.clone() {
            Adt2::C0(_, h1106, _, _, _, _, _, _, _, _, _) => {
                h1106
            }
        };
        let a1109 = match v1.clone() {
            Adt2::C0(_, h1107, _, _, _, _, _, _, _, _, _) => {
                h1107
            }
        };
        nat_eq(a1108, a1109)
    } else {
        false
    } {
        let a1112 = match v3.clone() {
            Adt2::C0(_, _, h1110, _, _, _, _, _, _, _, _) => {
                h1110
            }
        };
        let a1113 = match v1.clone() {
            Adt2::C0(_, _, _, h1111, _, _, _, _, _, _, _) => {
                h1111
            }
        };
        nat_eq(a1112, a1113)
    } else {
        false
    } {
        if v4 {
            f25(v1.clone(), v2.clone())
        } else {
            let a1116 = match v2.clone() {
                Adt2::C0(_, h1114, _, _, _, _, _, _, _, _, _) => {
                    h1114
                }
            };
            let a1117 = match v1.clone() {
                Adt2::C0(_, _, h1115, _, _, _, _, _, _, _, _) => {
                    h1115
                }
            };
            if if nat_eq(a1116, a1117) {
                let a1120 = match v2.clone() {
                    Adt2::C0(_, _, h1118, _, _, _, _, _, _, _, _) => {
                        h1118
                    }
                };
                let a1121 = match v1.clone() {
                    Adt2::C0(_, _, _, h1119, _, _, _, _, _, _, _) => {
                        h1119
                    }
                };
                nat_eq(a1120, a1121)
            } else {
                false
            } {
                let a1123 = match v2.clone() {
                    Adt2::C0(_, _, _, h1122, _, _, _, _, _, _, _) => {
                        h1122
                    }
                };
                let a1124 = 1u64;
                nat_eq(a1123, a1124)
            } else {
                false
            }
        }
    } else {
        false
    } {
        f114(v1.clone(), f16(v1.clone(), 1u64)?)?
    } else {
        false
    } {
        f114(v2.clone(), f16(v2.clone(), 1u64)?)?
    } else {
        false
    } {
        f115(v3.clone())?
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
            let a1126 = f16(v3.clone(), 1u64)?;
            let a1127 = f116(match v1.clone() {
                Adt2::C0(_, _, h1125, _, _, _, _, _, _, _, _) => {
                    h1125
                }
            }, v4)?;
            nat_mul(a1126, a1127)?
        }, 4u64, f20(v1, 1u64), f20(v3, 1u64), Adt4::C36(f20(v2, 1u64), v4, match v0 {
            Adt1::C0(h1128, _, _, _, _, _, _, _, _, _, _, _) => {
                h1128
            }
        })))
    } else {
        Ok(Adt10::C1)
    }
}

pub fn f42(v0: Adt1, v1: Adt2, v2: Adt2, v3: Adt2) -> R<Adt10> {
    let v4: Adt6 = Adt6::C0(match v1.clone() {
        Adt2::C0(_, h1129, _, _, _, _, _, _, _, _, _) => {
            h1129
        }
    }, match v0.clone() {
        Adt1::C0(h1130, _, _, _, _, _, _, _, _, _, _, _) => {
            h1130
        }
    }, match v0.clone() {
        Adt1::C0(_, _, h1131, _, _, _, _, _, _, _, _, _) => {
            h1131
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, _, h1132, _, _, _, _, _, _, _) => {
            h1132
        }
    });
    let v5: Adt6 = Adt6::C0(match v1.clone() {
        Adt2::C0(_, _, h1133, _, _, _, _, _, _, _, _) => {
            h1133
        }
    }, match v0.clone() {
        Adt1::C0(_, h1134, _, _, _, _, _, _, _, _, _, _) => {
            h1134
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, h1135, _, _, _, _, _, _, _, _) => {
            h1135
        }
    }, match v0 {
        Adt1::C0(_, _, _, _, _, h1136, _, _, _, _, _, _) => {
            h1136
        }
    });
    let v6: u64 = match v1.clone() {
        Adt2::C0(h1137, _, _, _, _, _, _, _, _, _, _) => {
            h1137
        }
    };
    let v7: bool = {
        let a1140 = match v2.clone() {
            Adt2::C0(_, _, _, _, _, _, _, h1138, _, _, _) => {
                h1138
            }
        };
        let a1141 = match v2.clone() {
            Adt2::C0(_, _, _, _, _, h1139, _, _, _, _, _) => {
                h1139
            }
        };
        nat_lt(a1140, a1141)
    };
    let a1142 = v6;
    let a1143 = 0u64;
    if if if if if if if if if if if if if if if nat_eq(a1142, a1143) {
        true
    } else {
        let a1144 = v6;
        let a1145 = 1u64;
        nat_eq(a1144, a1145)
    } {
        let a1147 = match v2.clone() {
            Adt2::C0(h1146, _, _, _, _, _, _, _, _, _, _) => {
                h1146
            }
        };
        let a1148 = 0u64;
        nat_eq(a1147, a1148)
    } else {
        false
    } {
        let a1150 = match v3.clone() {
            Adt2::C0(h1149, _, _, _, _, _, _, _, _, _, _) => {
                h1149
            }
        };
        let a1151 = 0u64;
        nat_eq(a1150, a1151)
    } else {
        false
    } {
        let a1153 = match v1.clone() {
            Adt2::C0(_, _, _, h1152, _, _, _, _, _, _, _) => {
                h1152
            }
        };
        let a1154 = 1u64;
        nat_eq(a1153, a1154)
    } else {
        false
    } {
        let a1157 = match v1.clone() {
            Adt2::C0(_, _, _, _, h1155, _, _, _, _, _, _) => {
                h1155
            }
        };
        let a1158 = match v2.clone() {
            Adt2::C0(_, _, _, h1156, _, _, _, _, _, _, _) => {
                h1156
            }
        };
        nat_eq(a1157, a1158)
    } else {
        false
    } {
        let a1161 = match v3.clone() {
            Adt2::C0(_, _, _, h1159, _, _, _, _, _, _, _) => {
                h1159
            }
        };
        let a1162 = match v2.clone() {
            Adt2::C0(_, _, _, h1160, _, _, _, _, _, _, _) => {
                h1160
            }
        };
        nat_eq(a1161, a1162)
    } else {
        false
    } {
        let a1165 = match v3.clone() {
            Adt2::C0(_, _, _, _, h1163, _, _, _, _, _, _) => {
                h1163
            }
        };
        let a1166 = match v2.clone() {
            Adt2::C0(_, _, _, _, h1164, _, _, _, _, _, _) => {
                h1164
            }
        };
        nat_eq(a1165, a1166)
    } else {
        false
    } {
        let a1168 = 0u64;
        let a1169 = match v4.clone() {
            Adt6::C0(h1167, _, _, _) => {
                h1167
            }
        };
        if if if if if nat_lt(a1168, a1169) {
            let a1171 = 0u64;
            let a1172 = match v4.clone() {
                Adt6::C0(_, h1170, _, _) => {
                    h1170
                }
            };
            nat_lt(a1171, a1172)
        } else {
            false
        } {
            let a1174 = match v4.clone() {
                Adt6::C0(_, h1173, _, _) => {
                    h1173
                }
            };
            let a1175 = 2147483648u64;
            nat_lt(a1174, a1175)
        } else {
            false
        } {
            let a1177 = 0u64;
            let a1178 = match v4.clone() {
                Adt6::C0(_, _, _, h1176) => {
                    h1176
                }
            };
            nat_lt(a1177, a1178)
        } else {
            false
        } {
            let a1180 = match v4.clone() {
                Adt6::C0(_, _, _, h1179) => {
                    h1179
                }
            };
            let a1181 = 2147483648u64;
            nat_lt(a1180, a1181)
        } else {
            false
        } {
            let a1184 = match v3.clone() {
                Adt2::C0(_, h1182, _, _, _, _, _, _, _, _, _) => {
                    h1182
                }
            };
            let a1185 = f102(v4.clone(), match v2.clone() {
                Adt2::C0(_, h1183, _, _, _, _, _, _, _, _, _) => {
                    h1183
                }
            })?;
            nat_eq(a1184, a1185)
        } else {
            false
        }
    } else {
        false
    } {
        let a1187 = 0u64;
        let a1188 = match v5.clone() {
            Adt6::C0(h1186, _, _, _) => {
                h1186
            }
        };
        if if if if if nat_lt(a1187, a1188) {
            let a1190 = 0u64;
            let a1191 = match v5.clone() {
                Adt6::C0(_, h1189, _, _) => {
                    h1189
                }
            };
            nat_lt(a1190, a1191)
        } else {
            false
        } {
            let a1193 = match v5.clone() {
                Adt6::C0(_, h1192, _, _) => {
                    h1192
                }
            };
            let a1194 = 2147483648u64;
            nat_lt(a1193, a1194)
        } else {
            false
        } {
            let a1196 = 0u64;
            let a1197 = match v5.clone() {
                Adt6::C0(_, _, _, h1195) => {
                    h1195
                }
            };
            nat_lt(a1196, a1197)
        } else {
            false
        } {
            let a1199 = match v5.clone() {
                Adt6::C0(_, _, _, h1198) => {
                    h1198
                }
            };
            let a1200 = 2147483648u64;
            nat_lt(a1199, a1200)
        } else {
            false
        } {
            let a1203 = match v3.clone() {
                Adt2::C0(_, _, h1201, _, _, _, _, _, _, _, _) => {
                    h1201
                }
            };
            let a1204 = f102(v5.clone(), match v2.clone() {
                Adt2::C0(_, _, h1202, _, _, _, _, _, _, _, _) => {
                    h1202
                }
            })?;
            nat_eq(a1203, a1204)
        } else {
            false
        }
    } else {
        false
    } {
        f117(v2.clone(), 4u64, v7, false)?
    } else {
        false
    } {
        f117(v3.clone(), 4u64, v7, false)?
    } else {
        false
    } {
        f117(v1.clone(), {
            let a1205 = v6;
            let a1206 = 0u64;
            if nat_eq(a1205, a1206) {
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
            let a1215 = {
                let a1213 = {
                    let a1211 = 3u64;
                    let a1212 = {
                        let a1209 = match v4.clone() {
                            Adt6::C0(h1207, _, _, _) => {
                                h1207
                            }
                        };
                        let a1210 = match v5.clone() {
                            Adt6::C0(h1208, _, _, _) => {
                                h1208
                            }
                        };
                        nat_mul(a1209, a1210)?
                    };
                    nat_mul(a1211, a1212)?
                };
                let a1214 = 2u64;
                nat_add(a1213, a1214)?
            };
            let a1216 = f16(v3.clone(), 1u64)?;
            nat_mul(a1215, a1216)?
        }, 4u64, f20(v1, 1u64), f20(v3, 1u64), Adt4::C34(f20(v2, 1u64), v4, v5, v6)))
    } else {
        Ok(Adt10::C1)
    }
}

pub fn f43(v0: Adt2, v1: Adt2, v2: Adt2) -> R<Adt10> {
    if if if if if if if f25(v0.clone(), v1.clone()) {
        f25(v0.clone(), v2.clone())
    } else {
        false
    } {
        f115(v0.clone())?
    } else {
        false
    } {
        f115(v1.clone())?
    } else {
        false
    } {
        f115(v2.clone())?
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
            let a1217 = 6u64;
            let a1218 = f16(v0.clone(), 1u64)?;
            nat_mul(a1217, a1218)?
        }, 2u64, f20(v0, 1u64), f20(v2, 1u64), Adt4::C33(f20(v1, 1u64))))
    } else {
        Ok(Adt10::C1)
    }
}

pub fn f44(v0: Adt1, v1: Adt2, v2: Option<Adt2>, v3: Adt2) -> R<Adt10> {
    let m1219 = v2;
    match m1219 {
        None => {
            Ok(Adt10::C1)
        }
        Some(v4) => {
            let v5: u64 = match v1.clone() {
                Adt2::C0(_, h1220, _, _, _, _, _, _, _, _, _) => {
                    h1220
                }
            };
            let v6: u64 = {
                let a1226 = match v1.clone() {
                    Adt2::C0(_, _, h1221, _, _, _, _, _, _, _, _) => {
                        h1221
                    }
                };
                let a1227 = {
                    let a1224 = match v1.clone() {
                        Adt2::C0(_, _, _, h1222, _, _, _, _, _, _, _) => {
                            h1222
                        }
                    };
                    let a1225 = match v1.clone() {
                        Adt2::C0(_, _, _, _, h1223, _, _, _, _, _, _) => {
                            h1223
                        }
                    };
                    nat_mul(a1224, a1225)?
                };
                nat_mul(a1226, a1227)?
            };
            if if if if if if if if if if f25(v1.clone(), v4.clone()) {
                f25(v1.clone(), v3.clone())
            } else {
                false
            } {
                f118(v1.clone(), v6)?
            } else {
                false
            } {
                f118(v4.clone(), v6)?
            } else {
                false
            } {
                f118(v3.clone(), v6)?
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
                let a1228 = v5;
                let a1229 = 2147483647u64;
                nat_le(a1228, a1229)
            } else {
                false
            } {
                let a1230 = v6;
                let a1231 = 2147483647u64;
                nat_le(a1230, a1231)
            } else {
                false
            } {
                let a1233 = match v0.clone() {
                    Adt1::C0(h1232, _, _, _, _, _, _, _, _, _, _, _) => {
                        h1232
                    }
                };
                let a1234 = 4294967295u64;
                if nat_le(a1233, a1234) {
                    let a1236 = match v0.clone() {
                        Adt1::C0(_, h1235, _, _, _, _, _, _, _, _, _, _) => {
                            h1235
                        }
                    };
                    let a1237 = 4294967295u64;
                    nat_le(a1236, a1237)
                } else {
                    false
                }
            } else {
                false
            } {
                Ok(Adt10::C4({
                    let a1238 = v5;
                    let a1239 = 0u64;
                    if nat_eq(a1238, a1239) {
                        0u64
                    } else {
                        let a1240 = v6;
                        let a1241 = f119(v5)?;
                        nat_mul(a1240, a1241)?
                    }
                }, 4u64, f20(v1, 1u64), f20(v3, 1u64), Adt4::C32(match v0 {
                    Adt1::C0(h1242, _, _, _, _, _, _, _, _, _, _, _) => {
                        h1242
                    }
                })))
            } else {
                Ok(Adt10::C1)
            }
        }
    }
}

pub fn f45(v0: Adt1, v1: Adt2, v2: Adt2) -> R<Adt10> {
    let v3: u64 = match v0.clone() {
        Adt1::C0(h1243, _, _, _, _, _, _, _, _, _, _, _) => {
            h1243
        }
    };
    let v4: u64 = match v0.clone() {
        Adt1::C0(_, h1244, _, _, _, _, _, _, _, _, _, _) => {
            h1244
        }
    };
    let v5: u64 = match v0 {
        Adt1::C0(_, _, h1245, _, _, _, _, _, _, _, _, _) => {
            h1245
        }
    };
    let v6: u64 = {
        let a1247 = match v1.clone() {
            Adt2::C0(_, h1246, _, _, _, _, _, _, _, _, _) => {
                h1246
            }
        };
        let a1248 = v4;
        let a1249 = 0u64;
        nat_quot(a1247, a1248, a1249)
    };
    let v7: u64 = {
        let a1251 = match v1.clone() {
            Adt2::C0(h1250, _, _, _, _, _, _, _, _, _, _) => {
                h1250
            }
        };
        let a1252 = 0u64;
        if nat_eq(a1251, a1252) {
            4u64
        } else {
            2u64
        }
    };
    if if if if if if if if if if if if if if if f120(match v1.clone() {
        Adt2::C0(h1253, _, _, _, _, _, _, _, _, _, _) => {
            h1253
        }
    }) {
        let a1256 = match v1.clone() {
            Adt2::C0(h1254, _, _, _, _, _, _, _, _, _, _) => {
                h1254
            }
        };
        let a1257 = match v2.clone() {
            Adt2::C0(h1255, _, _, _, _, _, _, _, _, _, _) => {
                h1255
            }
        };
        nat_eq(a1256, a1257)
    } else {
        false
    } {
        let a1258 = 0u64;
        let a1259 = v3;
        nat_lt(a1258, a1259)
    } else {
        false
    } {
        let a1260 = v3;
        let a1261 = 2147483648u64;
        nat_lt(a1260, a1261)
    } else {
        false
    } {
        let a1262 = 0u64;
        let a1263 = v4;
        nat_lt(a1262, a1263)
    } else {
        false
    } {
        let a1264 = v4;
        let a1265 = 2147483648u64;
        nat_lt(a1264, a1265)
    } else {
        false
    } {
        let a1266 = v5;
        let a1267 = 2147483648u64;
        nat_lt(a1266, a1267)
    } else {
        false
    } {
        let a1268 = 0u64;
        let a1269 = v6;
        nat_lt(a1268, a1269)
    } else {
        false
    } {
        let a1271 = 0u64;
        let a1272 = match v2.clone() {
            Adt2::C0(_, h1270, _, _, _, _, _, _, _, _, _) => {
                h1270
            }
        };
        nat_lt(a1271, a1272)
    } else {
        false
    } {
        let a1276 = match v1.clone() {
            Adt2::C0(_, h1273, _, _, _, _, _, _, _, _, _) => {
                h1273
            }
        };
        let a1277 = {
            let a1274 = v6;
            let a1275 = v4;
            nat_mul(a1274, a1275)?
        };
        nat_eq(a1276, a1277)
    } else {
        false
    } {
        let a1279 = match v2.clone() {
            Adt2::C0(_, _, h1278, _, _, _, _, _, _, _, _) => {
                h1278
            }
        };
        let a1280 = v4;
        nat_eq(a1279, a1280)
    } else {
        false
    } {
        let a1293 = {
            let a1286 = {
                let a1284 = match v2.clone() {
                    Adt2::C0(_, h1281, _, _, _, _, _, _, _, _, _) => {
                        h1281
                    }
                };
                let a1285 = {
                    let a1282 = v5;
                    let a1283 = 2u64;
                    nat_mul(a1282, a1283)?
                };
                nat_add(a1284, a1285)?
            };
            let a1287 = v3;
            nat_add(a1286, a1287)?
        };
        let a1294 = {
            let a1291 = {
                let a1289 = match v1.clone() {
                    Adt2::C0(_, _, h1288, _, _, _, _, _, _, _, _) => {
                        h1288
                    }
                };
                let a1290 = v3;
                nat_mul(a1289, a1290)?
            };
            let a1292 = v6;
            nat_add(a1291, a1292)?
        };
        nat_eq(a1293, a1294)
    } else {
        false
    } {
        f121(v1.clone(), v7)?
    } else {
        false
    } {
        f121(v2.clone(), v7)?
    } else {
        false
    } {
        f23(v1.clone(), v2.clone())?
    } else {
        false
    } {
        let v8: Adt6 = Adt6::C0(v6, v3, v5, 1u64);
        Ok(Adt10::C4({
            let a1295 = f16(v2.clone(), 1u64)?;
            let a1296 = f122(v8.clone())?;
            nat_mul(a1295, a1296)?
        }, v7, f20(v1.clone(), 1u64), f20(v2, 1u64), Adt4::C30(v8, match v1 {
            Adt2::C0(h1297, _, _, _, _, _, _, _, _, _, _) => {
                h1297
            }
        })))
    } else {
        Ok(Adt10::C1)
    }
}

pub fn f46(v0: Adt2, v1: Option<Adt2>, v2: Adt2) -> R<Adt10> {
    let m1298 = v1;
    match m1298 {
        None => {
            Ok(Adt10::C1)
        }
        Some(v3) => {
            let a1300 = match v0.clone() {
                Adt2::C0(h1299, _, _, _, _, _, _, _, _, _, _) => {
                    h1299
                }
            };
            let a1301 = 0u64;
            if if if if if if if if if if if if if if if if if if if nat_eq(a1300, a1301) {
                let a1303 = match v3.clone() {
                    Adt2::C0(h1302, _, _, _, _, _, _, _, _, _, _) => {
                        h1302
                    }
                };
                let a1304 = 0u64;
                nat_eq(a1303, a1304)
            } else {
                false
            } {
                let a1306 = match v2.clone() {
                    Adt2::C0(h1305, _, _, _, _, _, _, _, _, _, _) => {
                        h1305
                    }
                };
                let a1307 = 0u64;
                nat_eq(a1306, a1307)
            } else {
                false
            } {
                let a1309 = match v0.clone() {
                    Adt2::C0(_, _, _, _, h1308, _, _, _, _, _, _) => {
                        h1308
                    }
                };
                let a1310 = 1u64;
                nat_eq(a1309, a1310)
            } else {
                false
            } {
                let a1312 = match v3.clone() {
                    Adt2::C0(_, _, _, h1311, _, _, _, _, _, _, _) => {
                        h1311
                    }
                };
                let a1313 = 1u64;
                nat_eq(a1312, a1313)
            } else {
                false
            } {
                let a1315 = match v3.clone() {
                    Adt2::C0(_, _, _, _, h1314, _, _, _, _, _, _) => {
                        h1314
                    }
                };
                let a1316 = 1u64;
                nat_eq(a1315, a1316)
            } else {
                false
            } {
                let a1319 = match v0.clone() {
                    Adt2::C0(_, _, h1317, _, _, _, _, _, _, _, _) => {
                        h1317
                    }
                };
                let a1320 = match v3.clone() {
                    Adt2::C0(_, _, h1318, _, _, _, _, _, _, _, _) => {
                        h1318
                    }
                };
                nat_eq(a1319, a1320)
            } else {
                false
            } {
                let a1325 = match v3.clone() {
                    Adt2::C0(_, h1321, _, _, _, _, _, _, _, _, _) => {
                        h1321
                    }
                };
                let a1326 = {
                    let a1323 = match v0.clone() {
                        Adt2::C0(_, h1322, _, _, _, _, _, _, _, _, _) => {
                            h1322
                        }
                    };
                    let a1324 = 1u64;
                    nat_add(a1323, a1324)?
                };
                nat_le(a1325, a1326)
            } else {
                false
            } {
                let a1329 = match v2.clone() {
                    Adt2::C0(_, h1327, _, _, _, _, _, _, _, _, _) => {
                        h1327
                    }
                };
                let a1330 = match v0.clone() {
                    Adt2::C0(_, _, h1328, _, _, _, _, _, _, _, _) => {
                        h1328
                    }
                };
                nat_eq(a1329, a1330)
            } else {
                false
            } {
                let a1338 = match v2.clone() {
                    Adt2::C0(_, _, h1331, _, _, _, _, _, _, _, _) => {
                        h1331
                    }
                };
                let a1339 = {
                    let a1336 = {
                        let a1333 = match v0.clone() {
                            Adt2::C0(_, h1332, _, _, _, _, _, _, _, _, _) => {
                                h1332
                            }
                        };
                        let a1334 = 1u64;
                        nat_add(a1333, a1334)?
                    };
                    let a1337 = match v3.clone() {
                        Adt2::C0(_, h1335, _, _, _, _, _, _, _, _, _) => {
                            h1335
                        }
                    };
                    nat_sub(a1336, a1337)
                };
                nat_eq(a1338, a1339)
            } else {
                false
            } {
                let a1342 = match v2.clone() {
                    Adt2::C0(_, _, _, h1340, _, _, _, _, _, _, _) => {
                        h1340
                    }
                };
                let a1343 = match v0.clone() {
                    Adt2::C0(_, _, _, h1341, _, _, _, _, _, _, _) => {
                        h1341
                    }
                };
                nat_eq(a1342, a1343)
            } else {
                false
            } {
                let a1345 = match v2.clone() {
                    Adt2::C0(_, _, _, _, h1344, _, _, _, _, _, _) => {
                        h1344
                    }
                };
                let a1346 = 1u64;
                nat_eq(a1345, a1346)
            } else {
                false
            } {
                f22(v2.clone())?
            } else {
                false
            } {
                let a1348 = match v0.clone() {
                    Adt2::C0(_, _, _, _, _, h1347, _, _, _, _, _) => {
                        h1347
                    }
                };
                let a1349 = 4u64;
                nat_eq(a1348, a1349)
            } else {
                false
            } {
                let a1351 = match v3.clone() {
                    Adt2::C0(_, _, _, _, _, h1350, _, _, _, _, _) => {
                        h1350
                    }
                };
                let a1352 = 4u64;
                nat_eq(a1351, a1352)
            } else {
                false
            } {
                let a1357 = match v0.clone() {
                    Adt2::C0(_, _, _, _, _, _, h1353, _, _, _, _) => {
                        h1353
                    }
                };
                let a1358 = {
                    let a1355 = match v0.clone() {
                        Adt2::C0(_, h1354, _, _, _, _, _, _, _, _, _) => {
                            h1354
                        }
                    };
                    let a1356 = 4u64;
                    nat_mul(a1355, a1356)?
                };
                nat_eq(a1357, a1358)
            } else {
                false
            } {
                let a1363 = match v3.clone() {
                    Adt2::C0(_, _, _, _, _, _, h1359, _, _, _, _) => {
                        h1359
                    }
                };
                let a1364 = {
                    let a1361 = match v3.clone() {
                        Adt2::C0(_, h1360, _, _, _, _, _, _, _, _, _) => {
                            h1360
                        }
                    };
                    let a1362 = 4u64;
                    nat_mul(a1361, a1362)?
                };
                nat_eq(a1363, a1364)
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
                    let a1366 = f16(v2.clone(), 1u64)?;
                    let a1367 = f123(match v3.clone() {
                        Adt2::C0(_, h1365, _, _, _, _, _, _, _, _, _) => {
                            h1365
                        }
                    })?;
                    nat_mul(a1366, a1367)?
                }, 4u64, f20(v0, 1u64), f20(v2, 1u64), Adt4::C29(f20(v3, 1u64))))
            } else {
                Ok(Adt10::C1)
            }
        }
    }
}

pub fn f47(v0: bool, v1: Adt1, v2: Adt2, v3: Adt2) -> R<Adt10> {
    let v4: Adt6 = Adt6::C0(match v1.clone() {
        Adt1::C0(_, h1368, _, _, _, _, _, _, _, _, _, _) => {
            h1368
        }
    }, if v0 {
        match v1.clone() {
            Adt1::C0(_, _, _, h1369, _, _, _, _, _, _, _, _) => {
                h1369
            }
        }
    } else {
        match v1.clone() {
            Adt1::C0(_, _, h1370, _, _, _, _, _, _, _, _, _) => {
                h1370
            }
        }
    }, if v0 {
        match v1.clone() {
            Adt1::C0(_, _, _, _, _, h1371, _, _, _, _, _, _) => {
                h1371
            }
        }
    } else {
        match v1.clone() {
            Adt1::C0(_, _, _, h1372, _, _, _, _, _, _, _, _) => {
                h1372
            }
        }
    }, 1u64);
    let v5: Adt6 = if v0 {
        Adt6::C0(match v1.clone() {
            Adt1::C0(_, _, h1373, _, _, _, _, _, _, _, _, _) => {
                h1373
            }
        }, match v1.clone() {
            Adt1::C0(_, _, _, _, h1374, _, _, _, _, _, _, _) => {
                h1374
            }
        }, match v1.clone() {
            Adt1::C0(_, _, _, _, _, _, h1375, _, _, _, _, _) => {
                h1375
            }
        }, 1u64)
    } else {
        Adt6::C0(1u64, 1u64, 0u64, 1u64)
    };
    let a1377 = match v1.clone() {
        Adt1::C0(h1376, _, _, _, _, _, _, _, _, _, _, _) => {
            h1376
        }
    };
    let a1378 = 1u64;
    if if if if if if if if if if if nat_le(a1377, a1378) {
        let a1380 = match v2.clone() {
            Adt2::C0(h1379, _, _, _, _, _, _, _, _, _, _) => {
                h1379
            }
        };
        let a1381 = 1u64;
        nat_le(a1380, a1381)
    } else {
        false
    } {
        let a1383 = match v3.clone() {
            Adt2::C0(h1382, _, _, _, _, _, _, _, _, _, _) => {
                h1382
            }
        };
        let a1384 = 0u64;
        nat_eq(a1383, a1384)
    } else {
        false
    } {
        let a1389 = match v2.clone() {
            Adt2::C0(_, _, _, _, _, h1385, _, _, _, _, _) => {
                h1385
            }
        };
        let a1390 = {
            let a1387 = match v2.clone() {
                Adt2::C0(h1386, _, _, _, _, _, _, _, _, _, _) => {
                    h1386
                }
            };
            let a1388 = 0u64;
            if nat_eq(a1387, a1388) {
                4u64
            } else {
                2u64
            }
        };
        nat_eq(a1389, a1390)
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
        f124(v4.clone(), match v2.clone() {
            Adt2::C0(_, h1391, _, _, _, _, _, _, _, _, _) => {
                h1391
            }
        }, match v3.clone() {
            Adt2::C0(_, h1392, _, _, _, _, _, _, _, _, _) => {
                h1392
            }
        })?
    } else {
        false
    } {
        if v0 {
            f124(v5.clone(), match v2.clone() {
                Adt2::C0(_, _, h1393, _, _, _, _, _, _, _, _) => {
                    h1393
                }
            }, match v3.clone() {
                Adt2::C0(_, _, h1394, _, _, _, _, _, _, _, _) => {
                    h1394
                }
            })?
        } else {
            let a1397 = match v2.clone() {
                Adt2::C0(_, _, h1395, _, _, _, _, _, _, _, _) => {
                    h1395
                }
            };
            let a1398 = match v3.clone() {
                Adt2::C0(_, _, h1396, _, _, _, _, _, _, _, _) => {
                    h1396
                }
            };
            nat_eq(a1397, a1398)
        }
    } else {
        false
    } {
        let a1401 = match v2.clone() {
            Adt2::C0(_, _, _, h1399, _, _, _, _, _, _, _) => {
                h1399
            }
        };
        let a1402 = match v3.clone() {
            Adt2::C0(_, _, _, h1400, _, _, _, _, _, _, _) => {
                h1400
            }
        };
        nat_eq(a1401, a1402)
    } else {
        false
    } {
        let a1405 = match v2.clone() {
            Adt2::C0(_, _, _, _, h1403, _, _, _, _, _, _) => {
                h1403
            }
        };
        let a1406 = match v3.clone() {
            Adt2::C0(_, _, _, _, h1404, _, _, _, _, _, _) => {
                h1404
            }
        };
        nat_eq(a1405, a1406)
    } else {
        false
    } {
        let a1411 = {
            let a1409 = match v4.clone() {
                Adt6::C0(h1407, _, _, _) => {
                    h1407
                }
            };
            let a1410 = match v5.clone() {
                Adt6::C0(h1408, _, _, _) => {
                    h1408
                }
            };
            nat_mul(a1409, a1410)?
        };
        let a1412 = 2147483648u64;
        nat_lt(a1411, a1412)
    } else {
        false
    } {
        let v6: Adt7 = Adt7::C0(v4, v5, v0, {
            let a1414 = match v1 {
                Adt1::C0(h1413, _, _, _, _, _, _, _, _, _, _, _) => {
                    h1413
                }
            };
            let a1415 = 0u64;
            nat_eq(a1414, a1415)
        }, match v2.clone() {
            Adt2::C0(h1416, _, _, _, _, _, _, _, _, _, _) => {
                h1416
            }
        });
        Ok(Adt10::C4({
            let a1417 = f16(v3.clone(), 1u64)?;
            let a1418 = f125(v6.clone())?;
            nat_mul(a1417, a1418)?
        }, 4u64, f20(v2, 1u64), f20(v3, 1u64), Adt4::C28(v6)))
    } else {
        Ok(Adt10::C1)
    }
}

pub fn f48(v0: Adt1, v1: Adt2, v2: Option<Adt2>, v3: Adt2) -> R<Adt10> {
    let m1419 = v2;
    match m1419 {
        None => {
            Ok(Adt10::C1)
        }
        Some(v4) => {
            let v5: bool = {
                let a1421 = match v0.clone() {
                    Adt1::C0(_, _, _, _, _, _, h1420, _, _, _, _, _) => {
                        h1420
                    }
                };
                let a1422 = 1u64;
                nat_eq(a1421, a1422)
            };
            let v6: Adt6 = Adt6::C0(match v1.clone() {
                Adt2::C0(_, h1423, _, _, _, _, _, _, _, _, _) => {
                    h1423
                }
            }, match v0.clone() {
                Adt1::C0(h1424, _, _, _, _, _, _, _, _, _, _, _) => {
                    h1424
                }
            }, match v0.clone() {
                Adt1::C0(_, _, h1425, _, _, _, _, _, _, _, _, _) => {
                    h1425
                }
            }, match v0.clone() {
                Adt1::C0(_, _, _, _, h1426, _, _, _, _, _, _, _) => {
                    h1426
                }
            });
            let v7: Adt6 = if v5 {
                Adt6::C0(match v1.clone() {
                    Adt2::C0(_, _, h1427, _, _, _, _, _, _, _, _) => {
                        h1427
                    }
                }, match v0.clone() {
                    Adt1::C0(_, h1428, _, _, _, _, _, _, _, _, _, _) => {
                        h1428
                    }
                }, match v0.clone() {
                    Adt1::C0(_, _, _, h1429, _, _, _, _, _, _, _, _) => {
                        h1429
                    }
                }, match v0.clone() {
                    Adt1::C0(_, _, _, _, _, h1430, _, _, _, _, _, _) => {
                        h1430
                    }
                })
            } else {
                Adt6::C0(1u64, 1u64, 0u64, 1u64)
            };
            let v8: Adt6 = Adt6::C0(1u64, 1u64, 0u64, 1u64);
            let v9: u64 = if v5 {
                match v4.clone() {
                    Adt2::C0(_, _, _, h1431, _, _, _, _, _, _, _) => {
                        h1431
                    }
                }
            } else {
                match v4.clone() {
                    Adt2::C0(_, _, h1432, _, _, _, _, _, _, _, _) => {
                        h1432
                    }
                }
            };
            let v10: u64 = if v5 {
                match v4.clone() {
                    Adt2::C0(_, _, _, _, h1433, _, _, _, _, _, _) => {
                        h1433
                    }
                }
            } else {
                match v4.clone() {
                    Adt2::C0(_, _, _, h1434, _, _, _, _, _, _, _) => {
                        h1434
                    }
                }
            };
            let v11: u64 = if v5 {
                match v4.clone() {
                    Adt2::C0(_, _, h1435, _, _, _, _, _, _, _, _) => {
                        h1435
                    }
                }
            } else {
                1u64
            };
            let v12: u64 = match v4.clone() {
                Adt2::C0(h1436, _, _, _, _, _, _, _, _, _, _) => {
                    h1436
                }
            };
            let v13: u64 = match v3.clone() {
                Adt2::C0(h1437, _, _, _, _, _, _, _, _, _, _) => {
                    h1437
                }
            };
            let v14: u64 = {
                let a1438 = v12;
                let a1439 = 0u64;
                if nat_eq(a1438, a1439) {
                    4u64
                } else {
                    2u64
                }
            };
            let v15: u64 = {
                let a1441 = match v4.clone() {
                    Adt2::C0(_, h1440, _, _, _, _, _, _, _, _, _) => {
                        h1440
                    }
                };
                let a1442 = v14;
                nat_mul(a1441, a1442)?
            };
            let v16: u64 = if v5 {
                match v4.clone() {
                    Adt2::C0(_, _, _, _, _, _, _, h1443, _, _, _) => {
                        h1443
                    }
                }
            } else {
                match v4.clone() {
                    Adt2::C0(_, _, _, _, _, _, h1444, _, _, _, _) => {
                        h1444
                    }
                }
            };
            let v17: u64 = if v5 {
                match v4.clone() {
                    Adt2::C0(_, _, _, _, _, _, _, _, h1445, _, _) => {
                        h1445
                    }
                }
            } else {
                match v4.clone() {
                    Adt2::C0(_, _, _, _, _, _, _, h1446, _, _, _) => {
                        h1446
                    }
                }
            };
            let a1448 = match v0 {
                Adt1::C0(_, _, _, _, _, _, h1447, _, _, _, _, _) => {
                    h1447
                }
            };
            let a1449 = 1u64;
            if if if if if if if if if if nat_le(a1448, a1449) {
                let a1450 = v12;
                let a1451 = 0u64;
                if nat_eq(a1450, a1451) {
                    true
                } else {
                    let a1452 = v12;
                    let a1453 = 1u64;
                    if nat_eq(a1452, a1453) {
                        let a1454 = v13;
                        let a1455 = 1u64;
                        nat_eq(a1454, a1455)
                    } else {
                        false
                    }
                }
            } else {
                false
            } {
                let a1456 = v13;
                let a1457 = 0u64;
                if nat_eq(a1456, a1457) {
                    true
                } else {
                    let a1458 = v13;
                    let a1459 = 1u64;
                    nat_eq(a1458, a1459)
                }
            } else {
                false
            } {
                let a1461 = match v4.clone() {
                    Adt2::C0(_, _, _, _, _, h1460, _, _, _, _, _) => {
                        h1460
                    }
                };
                let a1462 = v14;
                nat_eq(a1461, a1462)
            } else {
                false
            } {
                let a1470 = match v3.clone() {
                    Adt2::C0(_, h1463, _, _, _, _, _, _, _, _, _) => {
                        h1463
                    }
                };
                let a1471 = {
                    let a1468 = v9;
                    let a1469 = {
                        let a1466 = match v6.clone() {
                            Adt6::C0(h1464, _, _, _) => {
                                h1464
                            }
                        };
                        let a1467 = match v7.clone() {
                            Adt6::C0(h1465, _, _, _) => {
                                h1465
                            }
                        };
                        nat_mul(a1466, a1467)?
                    };
                    nat_mul(a1468, a1469)?
                };
                nat_eq(a1470, a1471)
            } else {
                false
            } {
                let a1474 = match v3.clone() {
                    Adt2::C0(_, _, h1472, _, _, _, _, _, _, _, _) => {
                        h1472
                    }
                };
                let a1475 = f102(v6.clone(), match v4.clone() {
                    Adt2::C0(_, h1473, _, _, _, _, _, _, _, _, _) => {
                        h1473
                    }
                })?;
                nat_eq(a1474, a1475)
            } else {
                false
            } {
                let a1477 = 0u64;
                let a1478 = match v3.clone() {
                    Adt2::C0(_, _, h1476, _, _, _, _, _, _, _, _) => {
                        h1476
                    }
                };
                nat_lt(a1477, a1478)
            } else {
                false
            } {
                if v5 {
                    let a1480 = match v1 {
                        Adt2::C0(_, _, _, h1479, _, _, _, _, _, _, _) => {
                            h1479
                        }
                    };
                    let a1481 = v9;
                    if if if nat_eq(a1480, a1481) {
                        let a1483 = match v3.clone() {
                            Adt2::C0(_, _, _, h1482, _, _, _, _, _, _, _) => {
                                h1482
                            }
                        };
                        let a1484 = f102(v7.clone(), v11)?;
                        nat_eq(a1483, a1484)
                    } else {
                        false
                    } {
                        let a1486 = match v3.clone() {
                            Adt2::C0(_, _, _, _, h1485, _, _, _, _, _, _) => {
                                h1485
                            }
                        };
                        let a1487 = v10;
                        nat_eq(a1486, a1487)
                    } else {
                        false
                    } {
                        let a1489 = 0u64;
                        let a1490 = match v3.clone() {
                            Adt2::C0(_, _, _, h1488, _, _, _, _, _, _, _) => {
                                h1488
                            }
                        };
                        nat_lt(a1489, a1490)
                    } else {
                        false
                    }
                } else {
                    let a1492 = match v1 {
                        Adt2::C0(_, _, h1491, _, _, _, _, _, _, _, _) => {
                            h1491
                        }
                    };
                    let a1493 = v9;
                    if if if nat_eq(a1492, a1493) {
                        let a1495 = match v4.clone() {
                            Adt2::C0(_, _, _, _, h1494, _, _, _, _, _, _) => {
                                h1494
                            }
                        };
                        let a1496 = 1u64;
                        nat_eq(a1495, a1496)
                    } else {
                        false
                    } {
                        let a1498 = match v3.clone() {
                            Adt2::C0(_, _, _, h1497, _, _, _, _, _, _, _) => {
                                h1497
                            }
                        };
                        let a1499 = v10;
                        nat_eq(a1498, a1499)
                    } else {
                        false
                    } {
                        let a1501 = match v3.clone() {
                            Adt2::C0(_, _, _, _, h1500, _, _, _, _, _, _) => {
                                h1500
                            }
                        };
                        let a1502 = 1u64;
                        nat_eq(a1501, a1502)
                    } else {
                        false
                    }
                }
            } else {
                false
            } {
                if f34(match v4.clone() {
                    Adt2::C0(_, h1503, _, _, _, _, _, _, _, _, _) => {
                        h1503
                    }
                }, v11, v9, v10) {
                    true
                } else {
                    let a1523 = {
                        let a1520 = {
                            let a1518 = {
                                let a1512 = {
                                    let a1506 = {
                                        let a1504 = v10;
                                        let a1505 = 1u64;
                                        nat_sub(a1504, a1505)
                                    };
                                    let a1507 = v17;
                                    nat_mul(a1506, a1507)?
                                };
                                let a1513 = {
                                    let a1510 = {
                                        let a1508 = v9;
                                        let a1509 = 1u64;
                                        nat_sub(a1508, a1509)
                                    };
                                    let a1511 = v16;
                                    nat_mul(a1510, a1511)?
                                };
                                nat_add(a1512, a1513)?
                            };
                            let a1519 = {
                                let a1516 = {
                                    let a1514 = v11;
                                    let a1515 = 1u64;
                                    nat_sub(a1514, a1515)
                                };
                                let a1517 = v15;
                                nat_mul(a1516, a1517)?
                            };
                            nat_add(a1518, a1519)?
                        };
                        let a1521 = v15;
                        nat_add(a1520, a1521)?
                    };
                    let a1524 = match v4.clone() {
                        Adt2::C0(_, _, _, _, _, _, _, _, _, h1522, _) => {
                            h1522
                        }
                    };
                    nat_le(a1523, a1524)
                }
            } else {
                false
            } {
                f23(v4.clone(), v3.clone())?
            } else {
                false
            } {
                Ok(Adt10::C4(f16(v3.clone(), 1u64)?, {
                    let a1525 = v13;
                    let a1526 = 0u64;
                    if nat_eq(a1525, a1526) {
                        4u64
                    } else {
                        2u64
                    }
                }, Adt3::C0(match v4 {
                    Adt2::C0(_, h1527, _, _, _, _, _, _, _, _, _) => {
                        h1527
                    }
                }, v11, 1u64, v10, v14, v15, 0u64, 0u64), if v5 {
                    f20(v3, 1u64)
                } else {
                    Adt3::C0(match v3.clone() {
                        Adt2::C0(_, h1528, _, _, _, _, _, _, _, _, _) => {
                            h1528
                        }
                    }, match v3.clone() {
                        Adt2::C0(_, _, h1529, _, _, _, _, _, _, _, _) => {
                            h1529
                        }
                    }, 1u64, match v3.clone() {
                        Adt2::C0(_, _, _, h1530, _, _, _, _, _, _, _) => {
                            h1530
                        }
                    }, match v3.clone() {
                        Adt2::C0(_, _, _, _, _, h1531, _, _, _, _, _) => {
                            h1531
                        }
                    }, match v3.clone() {
                        Adt2::C0(_, _, _, _, _, _, h1532, _, _, _, _) => {
                            h1532
                        }
                    }, 0u64, match v3 {
                        Adt2::C0(_, _, _, _, _, _, _, h1533, _, _, _) => {
                            h1533
                        }
                    })
                }, Adt4::C27(Adt5::C0(v6, v7, v8, v16, v17, v12, 1u64, v13))))
            } else {
                Ok(Adt10::C1)
            }
        }
    }
}

pub fn f49(v0: Adt1, v1: Adt2, v2: Option<Adt2>, v3: Adt2) -> R<Adt10> {
    let m1534 = v2;
    match m1534 {
        None => {
            Ok(Adt10::C1)
        }
        Some(v4) => {
            let v5: Adt6 = Adt6::C0(match v1.clone() {
                Adt2::C0(_, h1535, _, _, _, _, _, _, _, _, _) => {
                    h1535
                }
            }, match v0.clone() {
                Adt1::C0(h1536, _, _, _, _, _, _, _, _, _, _, _) => {
                    h1536
                }
            }, match v0.clone() {
                Adt1::C0(_, _, _, h1537, _, _, _, _, _, _, _, _) => {
                    h1537
                }
            }, match v0.clone() {
                Adt1::C0(_, _, _, _, _, _, h1538, _, _, _, _, _) => {
                    h1538
                }
            });
            let v6: Adt6 = Adt6::C0(match v1.clone() {
                Adt2::C0(_, _, h1539, _, _, _, _, _, _, _, _) => {
                    h1539
                }
            }, match v0.clone() {
                Adt1::C0(_, h1540, _, _, _, _, _, _, _, _, _, _) => {
                    h1540
                }
            }, match v0.clone() {
                Adt1::C0(_, _, _, _, h1541, _, _, _, _, _, _, _) => {
                    h1541
                }
            }, match v0.clone() {
                Adt1::C0(_, _, _, _, _, _, _, h1542, _, _, _, _) => {
                    h1542
                }
            });
            let v7: Adt6 = Adt6::C0(match v1 {
                Adt2::C0(_, _, _, h1543, _, _, _, _, _, _, _) => {
                    h1543
                }
            }, match v0.clone() {
                Adt1::C0(_, _, h1544, _, _, _, _, _, _, _, _, _) => {
                    h1544
                }
            }, match v0.clone() {
                Adt1::C0(_, _, _, _, _, h1545, _, _, _, _, _, _) => {
                    h1545
                }
            }, match v0.clone() {
                Adt1::C0(_, _, _, _, _, _, _, _, h1546, _, _, _) => {
                    h1546
                }
            });
            let v8: u64 = match v0 {
                Adt1::C0(_, _, _, _, _, _, _, _, _, h1547, _, _) => {
                    h1547
                }
            };
            let v9: u64 = f102(v7.clone(), match v4.clone() {
                Adt2::C0(_, _, _, h1548, _, _, _, _, _, _, _) => {
                    h1548
                }
            })?;
            let v10: u64 = match v3.clone() {
                Adt2::C0(h1549, _, _, _, _, _, _, _, _, _, _) => {
                    h1549
                }
            };
            let v11: u64 = {
                let a1557 = v8;
                let a1558 = {
                    let a1555 = match v5.clone() {
                        Adt6::C0(h1550, _, _, _) => {
                            h1550
                        }
                    };
                    let a1556 = {
                        let a1553 = match v6.clone() {
                            Adt6::C0(h1551, _, _, _) => {
                                h1551
                            }
                        };
                        let a1554 = match v7.clone() {
                            Adt6::C0(h1552, _, _, _) => {
                                h1552
                            }
                        };
                        nat_mul(a1553, a1554)?
                    };
                    nat_mul(a1555, a1556)?
                };
                nat_mul(a1557, a1558)?
            };
            let a1560 = match v4.clone() {
                Adt2::C0(h1559, _, _, _, _, _, _, _, _, _, _) => {
                    h1559
                }
            };
            let a1561 = 0u64;
            if if if if if if if if if if if if if nat_eq(a1560, a1561) {
                let a1562 = v10;
                let a1563 = 0u64;
                if nat_eq(a1562, a1563) {
                    true
                } else {
                    let a1564 = v10;
                    let a1565 = 1u64;
                    nat_eq(a1564, a1565)
                }
            } else {
                false
            } {
                let a1566 = 0u64;
                let a1567 = v8;
                nat_lt(a1566, a1567)
            } else {
                false
            } {
                let a1568 = v8;
                let a1569 = 2147483648u64;
                nat_lt(a1568, a1569)
            } else {
                false
            } {
                let a1571 = match v4.clone() {
                    Adt2::C0(_, _, _, _, _, h1570, _, _, _, _, _) => {
                        h1570
                    }
                };
                let a1572 = 4u64;
                nat_eq(a1571, a1572)
            } else {
                false
            } {
                let a1573 = 0u64;
                let a1574 = v9;
                nat_lt(a1573, a1574)
            } else {
                false
            } {
                let a1576 = 0u64;
                let a1577 = match v3.clone() {
                    Adt2::C0(_, _, h1575, _, _, _, _, _, _, _, _) => {
                        h1575
                    }
                };
                nat_lt(a1576, a1577)
            } else {
                false
            } {
                let a1579 = 0u64;
                let a1580 = match v3.clone() {
                    Adt2::C0(_, _, _, h1578, _, _, _, _, _, _, _) => {
                        h1578
                    }
                };
                nat_lt(a1579, a1580)
            } else {
                false
            } {
                let a1582 = match v3.clone() {
                    Adt2::C0(_, h1581, _, _, _, _, _, _, _, _, _) => {
                        h1581
                    }
                };
                let a1583 = v11;
                nat_eq(a1582, a1583)
            } else {
                false
            } {
                let a1586 = match v3.clone() {
                    Adt2::C0(_, _, h1584, _, _, _, _, _, _, _, _) => {
                        h1584
                    }
                };
                let a1587 = f102(v5.clone(), match v4.clone() {
                    Adt2::C0(_, h1585, _, _, _, _, _, _, _, _, _) => {
                        h1585
                    }
                })?;
                nat_eq(a1586, a1587)
            } else {
                false
            } {
                let a1590 = match v3.clone() {
                    Adt2::C0(_, _, _, h1588, _, _, _, _, _, _, _) => {
                        h1588
                    }
                };
                let a1591 = f102(v6.clone(), match v4.clone() {
                    Adt2::C0(_, _, h1589, _, _, _, _, _, _, _, _) => {
                        h1589
                    }
                })?;
                nat_eq(a1590, a1591)
            } else {
                false
            } {
                let a1599 = match v3.clone() {
                    Adt2::C0(_, _, _, _, h1592, _, _, _, _, _, _) => {
                        h1592
                    }
                };
                let a1600 = {
                    let a1597 = v9;
                    let a1598 = {
                        let a1594 = match v4.clone() {
                            Adt2::C0(_, _, _, _, h1593, _, _, _, _, _, _) => {
                                h1593
                            }
                        };
                        let a1595 = v8;
                        let a1596 = 0u64;
                        nat_quot(a1594, a1595, a1596)
                    };
                    nat_mul(a1597, a1598)?
                };
                nat_eq(a1599, a1600)
            } else {
                false
            } {
                f23(v4.clone(), v3.clone())?
            } else {
                false
            } {
                Ok(Adt10::C4(f16(v3.clone(), 1u64)?, {
                    let a1601 = v10;
                    let a1602 = 0u64;
                    if nat_eq(a1601, a1602) {
                        4u64
                    } else {
                        2u64
                    }
                }, f20(v4.clone(), 1u64), f20(v3, 1u64), Adt4::C27(Adt5::C0(v5, v6, v7, match v4.clone() {
                    Adt2::C0(_, _, _, _, _, _, _, _, h1603, _, _) => {
                        h1603
                    }
                }, {
                    let a1605 = v8;
                    let a1606 = match v4 {
                        Adt2::C0(_, _, _, _, _, _, _, _, h1604, _, _) => {
                            h1604
                        }
                    };
                    nat_mul(a1605, a1606)?
                }, 0u64, v9, v10))))
            } else {
                Ok(Adt10::C1)
            }
        }
    }
}

pub fn f50(v0: Adt2, v1: Option<Adt2>, v2: Adt2) -> R<Adt10> {
    let m1607 = v1;
    match m1607 {
        None => {
            Ok(Adt10::C1)
        }
        Some(v3) => {
            f126(v0, v3, v2)
        }
    }
}

pub fn f51(v0: Adt2, v1: Adt2) -> R<Adt10> {
    let a1609 = match v0.clone() {
        Adt2::C0(h1608, _, _, _, _, _, _, _, _, _, _) => {
            h1608
        }
    };
    let a1610 = 1u64;
    if if nat_eq(a1609, a1610) {
        true
    } else {
        let a1612 = match v0.clone() {
            Adt2::C0(h1611, _, _, _, _, _, _, _, _, _, _) => {
                h1611
            }
        };
        let a1613 = 30u64;
        nat_eq(a1612, a1613)
    } {
        let a1615 = match v1.clone() {
            Adt2::C0(h1614, _, _, _, _, _, _, _, _, _, _) => {
                h1614
            }
        };
        let a1616 = 1u64;
        if if if if if if if if if if if nat_eq(a1615, a1616) {
            let a1619 = match v0.clone() {
                Adt2::C0(_, h1617, _, _, _, _, _, _, _, _, _) => {
                    h1617
                }
            };
            let a1620 = match v1.clone() {
                Adt2::C0(_, h1618, _, _, _, _, _, _, _, _, _) => {
                    h1618
                }
            };
            nat_eq(a1619, a1620)
        } else {
            false
        } {
            let a1622 = 0u64;
            let a1623 = match v1.clone() {
                Adt2::C0(_, _, h1621, _, _, _, _, _, _, _, _) => {
                    h1621
                }
            };
            nat_lt(a1622, a1623)
        } else {
            false
        } {
            let a1626 = match v1.clone() {
                Adt2::C0(_, _, h1624, _, _, _, _, _, _, _, _) => {
                    h1624
                }
            };
            let a1627 = match v1.clone() {
                Adt2::C0(_, _, _, h1625, _, _, _, _, _, _, _) => {
                    h1625
                }
            };
            nat_eq(a1626, a1627)
        } else {
            false
        } {
            let a1629 = match v1.clone() {
                Adt2::C0(_, _, _, _, h1628, _, _, _, _, _, _) => {
                    h1628
                }
            };
            let a1630 = 1u64;
            nat_eq(a1629, a1630)
        } else {
            false
        } {
            let a1638 = match v0.clone() {
                Adt2::C0(_, _, h1631, _, _, _, _, _, _, _, _) => {
                    h1631
                }
            };
            let a1639 = {
                let a1636 = {
                    let a1634 = match v1.clone() {
                        Adt2::C0(_, _, h1632, _, _, _, _, _, _, _, _) => {
                            h1632
                        }
                    };
                    let a1635 = match v1.clone() {
                        Adt2::C0(_, _, h1633, _, _, _, _, _, _, _, _) => {
                            h1633
                        }
                    };
                    nat_add(a1634, a1635)?
                };
                let a1637 = 1u64;
                nat_sub(a1636, a1637)
            };
            nat_eq(a1638, a1639)
        } else {
            false
        } {
            let a1641 = 0u64;
            let a1642 = match v0.clone() {
                Adt2::C0(_, _, _, h1640, _, _, _, _, _, _, _) => {
                    h1640
                }
            };
            nat_lt(a1641, a1642)
        } else {
            false
        } {
            let a1644 = 0u64;
            let a1645 = match v0.clone() {
                Adt2::C0(_, _, _, _, h1643, _, _, _, _, _, _) => {
                    h1643
                }
            };
            nat_lt(a1644, a1645)
        } else {
            false
        } {
            let a1647 = 2u64;
            let a1648 = match v0.clone() {
                Adt2::C0(_, _, _, _, _, h1646, _, _, _, _, _) => {
                    h1646
                }
            };
            nat_le(a1647, a1648)
        } else {
            false
        } {
            let a1650 = 2u64;
            let a1651 = match v1.clone() {
                Adt2::C0(_, _, _, _, _, h1649, _, _, _, _, _) => {
                    h1649
                }
            };
            nat_le(a1650, a1651)
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

pub fn f52(v0: Adt2, v1: Option<Adt2>, v2: Adt2) -> R<Adt10> {
    let m1652 = v1;
    match m1652 {
        None => {
            Ok(Adt10::C1)
        }
        Some(v3) => {
            f127(v0, v3, v2)
        }
    }
}

pub fn f53(v0: u64) -> bool {
    let a1653 = v0;
    let a1654 = 1u64;
    if if nat_eq(a1653, a1654) {
        true
    } else {
        let a1655 = v0;
        let a1656 = 33u64;
        nat_eq(a1655, a1656)
    } {
        true
    } else {
        let a1657 = v0;
        let a1658 = 34u64;
        nat_eq(a1657, a1658)
    }
}

pub fn f54(v0: Adt2, v1: Adt2, v2: Adt2) -> R<Adt10> {
    let v3: u64 = f16(v0.clone(), 1u64)?;
    if if if if if f33(v1, v2.clone()) {
        let a1659 = v3;
        let a1660 = f16(v2.clone(), 1u64)?;
        nat_eq(a1659, a1660)
    } else {
        false
    } {
        f114(v0.clone(), v3)?
    } else {
        false
    } {
        f114(v2.clone(), v3)?
    } else {
        false
    } {
        let a1661 = v3;
        let a1662 = 0u64;
        if nat_eq(a1661, a1662) {
            true
        } else {
            let a1664 = 4u64;
            let a1665 = match v2.clone() {
                Adt2::C0(_, _, _, _, _, h1663, _, _, _, _, _) => {
                    h1663
                }
            };
            nat_le(a1664, a1665)
        }
    } else {
        false
    } {
        Ok(Adt10::C4({
            let a1666 = 2u64;
            let a1667 = v3;
            nat_mul(a1666, a1667)?
        }, 4u64, f20(v0, 1u64), f20(v2, 1u64), Adt4::C35(v3)))
    } else {
        Ok(Adt10::C1)
    }
}

pub fn f55(v0: u64) -> bool {
    if f128(v0) {
        !f129(v0)
    } else {
        false
    }
}

pub fn f56(v0: Adt2, v1: Option<Adt2>, v2: Adt2) -> R<Adt10> {
    let m1668 = v1;
    match m1668 {
        None => {
            Ok(Adt10::C1)
        }
        Some(v3) => {
            let m1670 = f130(match v0.clone() {
                Adt2::C0(h1669, _, _, _, _, _, _, _, _, _, _) => {
                    h1669
                }
            });
            match m1670 {
                Adt17::C0 => {
                    Ok(Adt10::C0)
                }
                Adt17::C1(v4) => {
                    let v5: u64 = match v4.clone() {
                        Adt18::C0(_, h1671, _) => {
                            h1671
                        }
                    };
                    if if if if if if f33(v3, v2.clone()) {
                        f23(v0.clone(), v2.clone())?
                    } else {
                        false
                    } {
                        let a1672 = f16(v0.clone(), 1u64)?;
                        let a1673 = f16(v2.clone(), 1u64)?;
                        nat_eq(a1672, a1673)
                    } else {
                        false
                    } {
                        let a1678 = {
                            let a1675 = match v0.clone() {
                                Adt2::C0(_, h1674, _, _, _, _, _, _, _, _, _) => {
                                    h1674
                                }
                            };
                            let a1676 = v5;
                            let a1677 = 1u64;
                            nat_rem(a1675, a1676, a1677)
                        };
                        let a1679 = 0u64;
                        if nat_eq(a1678, a1679) {
                            let a1682 = match v4 {
                                Adt18::C0(_, _, h1680) => {
                                    h1680
                                }
                            };
                            let a1683 = match v0.clone() {
                                Adt2::C0(_, _, _, _, _, h1681, _, _, _, _, _) => {
                                    h1681
                                }
                            };
                            nat_le(a1682, a1683)
                        } else {
                            false
                        }
                    } else {
                        false
                    } {
                        let a1684 = v5;
                        let a1685 = 1u64;
                        if nat_eq(a1684, a1685) {
                            let a1687 = 4u64;
                            let a1688 = match v2.clone() {
                                Adt2::C0(_, _, _, _, _, h1686, _, _, _, _, _) => {
                                    h1686
                                }
                            };
                            nat_le(a1687, a1688)
                        } else {
                            let a1690 = match v2.clone() {
                                Adt2::C0(_, _, _, _, _, h1689, _, _, _, _, _) => {
                                    h1689
                                }
                            };
                            let a1691 = 4u64;
                            nat_eq(a1690, a1691)
                        }
                    } else {
                        false
                    } {
                        let a1696 = {
                            let a1693 = match v2.clone() {
                                Adt2::C0(_, h1692, _, _, _, _, _, _, _, _, _) => {
                                    h1692
                                }
                            };
                            let a1694 = v5;
                            let a1695 = 1u64;
                            nat_rem(a1693, a1694, a1695)
                        };
                        let a1697 = 0u64;
                        if nat_eq(a1696, a1697) {
                            true
                        } else {
                            f22(v2.clone())?
                        }
                    } else {
                        false
                    } {
                        Ok(Adt10::C4(f16(v0.clone(), v5)?, 4u64, f20(v0.clone(), v5), f20(v2, 1u64), Adt4::C11(match v0 {
                            Adt2::C0(h1698, _, _, _, _, _, _, _, _, _, _) => {
                                h1698
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

pub fn f57(v0: u64, v1: Adt2, v2: bool, v3: Adt2) -> R<Adt10> {
    let a1701 = match v1.clone() {
        Adt2::C0(h1699, _, _, _, _, _, _, _, _, _, _) => {
            h1699
        }
    };
    let a1702 = match v3.clone() {
        Adt2::C0(h1700, _, _, _, _, _, _, _, _, _, _) => {
            h1700
        }
    };
    if nat_eq(a1701, a1702) {
        f131(v0, v1, v2, v3)
    } else {
        if if if if f129(match v1.clone() {
            Adt2::C0(h1703, _, _, _, _, _, _, _, _, _, _) => {
                h1703
            }
        }) {
            f129(match v3.clone() {
                Adt2::C0(h1704, _, _, _, _, _, _, _, _, _, _) => {
                    h1704
                }
            })
        } else {
            false
        } {
            let a1705 = v0;
            let a1706 = 33u64;
            if nat_eq(a1705, a1706) {
                v2
            } else {
                true
            }
        } else {
            false
        } {
            let a1707 = f16(v1.clone(), 1u64)?;
            let a1708 = f16(v3.clone(), 1u64)?;
            nat_eq(a1707, a1708)
        } else {
            false
        } {
            let m1710 = f130(match v3.clone() {
                Adt2::C0(h1709, _, _, _, _, _, _, _, _, _, _) => {
                    h1709
                }
            });
            match m1710 {
                Adt17::C0 => {
                    Ok(Adt10::C0)
                }
                Adt17::C1(v4) => {
                    let v5: u64 = match v1.clone() {
                        Adt2::C0(h1711, _, _, _, _, _, _, _, _, _, _) => {
                            h1711
                        }
                    };
                    let v6: u64 = match v3.clone() {
                        Adt2::C0(h1712, _, _, _, _, _, _, _, _, _, _) => {
                            h1712
                        }
                    };
                    let v7: bool = {
                        let a1713 = v5;
                        let a1714 = 43u64;
                        if nat_eq(a1713, a1714) {
                            true
                        } else {
                            let a1715 = v6;
                            let a1716 = 43u64;
                            nat_eq(a1715, a1716)
                        }
                    };
                    Ok(Adt10::C4({
                        let a1717 = f16(v3.clone(), 1u64)?;
                        let a1718 = if v7 {
                            f132(v5, v6)
                        } else {
                            1u64
                        };
                        nat_mul(a1717, a1718)?
                    }, match v4 {
                        Adt18::C0(_, _, h1719) => {
                            h1719
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

pub fn f58(v0: Adt2, v1: Adt2) -> R<Adt10> {
    let a1721 = match v0.clone() {
        Adt2::C0(h1720, _, _, _, _, _, _, _, _, _, _) => {
            h1720
        }
    };
    let a1722 = 0u64;
    if if if if nat_eq(a1721, a1722) {
        true
    } else {
        let a1724 = match v0.clone() {
            Adt2::C0(h1723, _, _, _, _, _, _, _, _, _, _) => {
                h1723
            }
        };
        let a1725 = 1u64;
        nat_eq(a1724, a1725)
    } {
        true
    } else {
        let a1727 = match v0.clone() {
            Adt2::C0(h1726, _, _, _, _, _, _, _, _, _, _) => {
                h1726
            }
        };
        let a1728 = 25u64;
        if nat_eq(a1727, a1728) {
            true
        } else {
            let a1730 = match v0.clone() {
                Adt2::C0(h1729, _, _, _, _, _, _, _, _, _, _) => {
                    h1729
                }
            };
            let a1731 = 26u64;
            nat_eq(a1730, a1731)
        }
    } {
        true
    } else {
        let a1733 = match v0.clone() {
            Adt2::C0(h1732, _, _, _, _, _, _, _, _, _, _) => {
                h1732
            }
        };
        let a1734 = 30u64;
        nat_eq(a1733, a1734)
    } {
        let a1737 = match v0.clone() {
            Adt2::C0(h1735, _, _, _, _, _, _, _, _, _, _) => {
                h1735
            }
        };
        let a1738 = match v1.clone() {
            Adt2::C0(h1736, _, _, _, _, _, _, _, _, _, _) => {
                h1736
            }
        };
        if if nat_eq(a1737, a1738) {
            f133(match v1.clone() {
                Adt2::C0(_, h1739, _, _, _, _, _, _, _, _, _) => {
                    h1739
                }
            }, match v1.clone() {
                Adt2::C0(_, _, h1740, _, _, _, _, _, _, _, _) => {
                    h1740
                }
            }, match v1.clone() {
                Adt2::C0(_, _, _, h1741, _, _, _, _, _, _, _) => {
                    h1741
                }
            }, match v1.clone() {
                Adt2::C0(_, _, _, _, h1742, _, _, _, _, _, _) => {
                    h1742
                }
            }, match v0.clone() {
                Adt2::C0(_, h1743, _, _, _, _, _, _, _, _, _) => {
                    h1743
                }
            }, match v0.clone() {
                Adt2::C0(_, _, h1744, _, _, _, _, _, _, _, _) => {
                    h1744
                }
            }, match v0.clone() {
                Adt2::C0(_, _, _, h1745, _, _, _, _, _, _, _) => {
                    h1745
                }
            }, match v0.clone() {
                Adt2::C0(_, _, _, _, h1746, _, _, _, _, _, _) => {
                    h1746
                }
            })
        } else {
            false
        } {
            let m1748 = f130(match v0.clone() {
                Adt2::C0(h1747, _, _, _, _, _, _, _, _, _, _) => {
                    h1747
                }
            });
            match m1748 {
                Adt17::C0 => {
                    Ok(Adt10::C0)
                }
                Adt17::C1(v2) => {
                    Ok(Adt10::C4(f16(v1.clone(), 1u64)?, match v2 {
                        Adt18::C0(_, _, h1749) => {
                            h1749
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

pub fn f59(v0: u64, v1: Adt2, v2: Option<Adt2>, v3: Adt2) -> R<Adt10> {
    let m1750 = v2;
    match m1750 {
        None => {
            Ok(Adt10::C1)
        }
        Some(v4) => {
            let a1751 = v0;
            let a1752 = 4u64;
            if if if nat_lt(a1751, a1752) {
                let a1755 = match v1.clone() {
                    Adt2::C0(h1753, _, _, _, _, _, _, _, _, _, _) => {
                        h1753
                    }
                };
                let a1756 = match v4.clone() {
                    Adt2::C0(h1754, _, _, _, _, _, _, _, _, _, _) => {
                        h1754
                    }
                };
                if nat_eq(a1755, a1756) {
                    let a1759 = match v1.clone() {
                        Adt2::C0(h1757, _, _, _, _, _, _, _, _, _, _) => {
                            h1757
                        }
                    };
                    let a1760 = match v3.clone() {
                        Adt2::C0(h1758, _, _, _, _, _, _, _, _, _, _) => {
                            h1758
                        }
                    };
                    nat_eq(a1759, a1760)
                } else {
                    false
                }
            } else {
                false
            } {
                if if if f134(v0, 0u64, match v1.clone() {
                    Adt2::C0(_, h1761, _, _, _, _, _, _, _, _, _) => {
                        h1761
                    }
                }, match v4.clone() {
                    Adt2::C0(_, h1762, _, _, _, _, _, _, _, _, _) => {
                        h1762
                    }
                }, match v3.clone() {
                    Adt2::C0(_, h1763, _, _, _, _, _, _, _, _, _) => {
                        h1763
                    }
                })? {
                    f134(v0, 1u64, match v1.clone() {
                        Adt2::C0(_, _, h1764, _, _, _, _, _, _, _, _) => {
                            h1764
                        }
                    }, match v4.clone() {
                        Adt2::C0(_, _, h1765, _, _, _, _, _, _, _, _) => {
                            h1765
                        }
                    }, match v3.clone() {
                        Adt2::C0(_, _, h1766, _, _, _, _, _, _, _, _) => {
                            h1766
                        }
                    })?
                } else {
                    false
                } {
                    f134(v0, 2u64, match v1.clone() {
                        Adt2::C0(_, _, _, h1767, _, _, _, _, _, _, _) => {
                            h1767
                        }
                    }, match v4.clone() {
                        Adt2::C0(_, _, _, h1768, _, _, _, _, _, _, _) => {
                            h1768
                        }
                    }, match v3.clone() {
                        Adt2::C0(_, _, _, h1769, _, _, _, _, _, _, _) => {
                            h1769
                        }
                    })?
                } else {
                    false
                } {
                    f134(v0, 3u64, match v1.clone() {
                        Adt2::C0(_, _, _, _, h1770, _, _, _, _, _, _) => {
                            h1770
                        }
                    }, match v4.clone() {
                        Adt2::C0(_, _, _, _, h1771, _, _, _, _, _, _) => {
                            h1771
                        }
                    }, match v3.clone() {
                        Adt2::C0(_, _, _, _, h1772, _, _, _, _, _, _) => {
                            h1772
                        }
                    })?
                } else {
                    false
                }
            } else {
                false
            } {
                let m1774 = f130(match v1.clone() {
                    Adt2::C0(h1773, _, _, _, _, _, _, _, _, _, _) => {
                        h1773
                    }
                });
                match m1774 {
                    Adt17::C0 => {
                        Ok(Adt10::C0)
                    }
                    Adt17::C1(v5) => {
                        let v6: u64 = match v5.clone() {
                            Adt18::C0(_, h1775, _) => {
                                h1775
                            }
                        };
                        let a1780 = {
                            let a1777 = match v1.clone() {
                                Adt2::C0(_, h1776, _, _, _, _, _, _, _, _, _) => {
                                    h1776
                                }
                            };
                            let a1778 = v6;
                            let a1779 = 1u64;
                            nat_rem(a1777, a1778, a1779)
                        };
                        let a1781 = 0u64;
                        if if if nat_eq(a1780, a1781) {
                            let a1786 = {
                                let a1783 = match v4.clone() {
                                    Adt2::C0(_, h1782, _, _, _, _, _, _, _, _, _) => {
                                        h1782
                                    }
                                };
                                let a1784 = v6;
                                let a1785 = 1u64;
                                nat_rem(a1783, a1784, a1785)
                            };
                            let a1787 = 0u64;
                            nat_eq(a1786, a1787)
                        } else {
                            false
                        } {
                            let a1792 = {
                                let a1789 = match v3.clone() {
                                    Adt2::C0(_, h1788, _, _, _, _, _, _, _, _, _) => {
                                        h1788
                                    }
                                };
                                let a1790 = v6;
                                let a1791 = 1u64;
                                nat_rem(a1789, a1790, a1791)
                            };
                            let a1793 = 0u64;
                            nat_eq(a1792, a1793)
                        } else {
                            false
                        } {
                            Ok(Adt10::C4(f16(v3.clone(), v6)?, match v5 {
                                Adt18::C0(_, _, h1794) => {
                                    h1794
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

pub fn f60(v0: u64, v1: Adt1, v2: Adt2, v3: Adt2) -> R<Adt10> {
    let a1796 = match v2.clone() {
        Adt2::C0(h1795, _, _, _, _, _, _, _, _, _, _) => {
            h1795
        }
    };
    let a1797 = 0u64;
    if if nat_eq(a1796, a1797) {
        let a1799 = match v3.clone() {
            Adt2::C0(h1798, _, _, _, _, _, _, _, _, _, _) => {
                h1798
            }
        };
        let a1800 = 0u64;
        nat_eq(a1799, a1800)
    } else {
        false
    } {
        let a1801 = v0;
        let a1802 = 61u64;
        if nat_eq(a1801, a1802) {
            let v4: bool = {
                let a1804 = 0u64;
                let a1805 = match v1.clone() {
                    Adt1::C0(_, _, _, _, _, _, _, _, h1803, _, _, _) => {
                        h1803
                    }
                };
                nat_lt(a1804, a1805)
            };
            if if if if if if f135(match v2.clone() {
                Adt2::C0(_, h1806, _, _, _, _, _, _, _, _, _) => {
                    h1806
                }
            }, match v1.clone() {
                Adt1::C0(h1807, _, _, _, _, _, _, _, _, _, _, _) => {
                    h1807
                }
            }, match v1.clone() {
                Adt1::C0(_, h1808, _, _, _, _, _, _, _, _, _, _) => {
                    h1808
                }
            }, match v3.clone() {
                Adt2::C0(_, h1809, _, _, _, _, _, _, _, _, _) => {
                    h1809
                }
            })? {
                f135(match v2.clone() {
                    Adt2::C0(_, _, h1810, _, _, _, _, _, _, _, _) => {
                        h1810
                    }
                }, match v1.clone() {
                    Adt1::C0(_, _, h1811, _, _, _, _, _, _, _, _, _) => {
                        h1811
                    }
                }, match v1.clone() {
                    Adt1::C0(_, _, _, h1812, _, _, _, _, _, _, _, _) => {
                        h1812
                    }
                }, match v3.clone() {
                    Adt2::C0(_, _, h1813, _, _, _, _, _, _, _, _) => {
                        h1813
                    }
                })?
            } else {
                false
            } {
                f135(match v2.clone() {
                    Adt2::C0(_, _, _, h1814, _, _, _, _, _, _, _) => {
                        h1814
                    }
                }, match v1.clone() {
                    Adt1::C0(_, _, _, _, h1815, _, _, _, _, _, _, _) => {
                        h1815
                    }
                }, match v1.clone() {
                    Adt1::C0(_, _, _, _, _, h1816, _, _, _, _, _, _) => {
                        h1816
                    }
                }, match v3.clone() {
                    Adt2::C0(_, _, _, h1817, _, _, _, _, _, _, _) => {
                        h1817
                    }
                })?
            } else {
                false
            } {
                f135(match v2.clone() {
                    Adt2::C0(_, _, _, _, h1818, _, _, _, _, _, _) => {
                        h1818
                    }
                }, match v1.clone() {
                    Adt1::C0(_, _, _, _, _, _, h1819, _, _, _, _, _) => {
                        h1819
                    }
                }, match v1.clone() {
                    Adt1::C0(_, _, _, _, _, _, _, h1820, _, _, _, _) => {
                        h1820
                    }
                }, match v3.clone() {
                    Adt2::C0(_, _, _, _, h1821, _, _, _, _, _, _) => {
                        h1821
                    }
                })?
            } else {
                false
            } {
                let a1823 = match v1.clone() {
                    Adt1::C0(_, _, _, _, _, _, _, _, h1822, _, _, _) => {
                        h1822
                    }
                };
                let a1824 = 4294967295u64;
                nat_le(a1823, a1824)
            } else {
                false
            } {
                if v4 {
                    !f34(match v2.clone() {
                        Adt2::C0(_, h1825, _, _, _, _, _, _, _, _, _) => {
                            h1825
                        }
                    }, match v2.clone() {
                        Adt2::C0(_, _, h1826, _, _, _, _, _, _, _, _) => {
                            h1826
                        }
                    }, match v2.clone() {
                        Adt2::C0(_, _, _, h1827, _, _, _, _, _, _, _) => {
                            h1827
                        }
                    }, match v2.clone() {
                        Adt2::C0(_, _, _, _, h1828, _, _, _, _, _, _) => {
                            h1828
                        }
                    })
                } else {
                    true
                }
            } else {
                false
            } {
                Ok(Adt10::C4(f16(v3.clone(), 1u64)?, 4u64, f20(v2, 1u64), f20(v3, 1u64), Adt4::C3(match v1.clone() {
                    Adt1::C0(h1829, _, _, _, _, _, _, _, _, _, _, _) => {
                        h1829
                    }
                }, match v1.clone() {
                    Adt1::C0(_, _, h1830, _, _, _, _, _, _, _, _, _) => {
                        h1830
                    }
                }, match v1.clone() {
                    Adt1::C0(_, _, _, _, h1831, _, _, _, _, _, _, _) => {
                        h1831
                    }
                }, match v1 {
                    Adt1::C0(_, _, _, _, _, _, h1832, _, _, _, _, _) => {
                        h1832
                    }
                }, v4)))
            } else {
                Ok(Adt10::C1)
            }
        } else {
            let a1833 = v0;
            let a1834 = 62u64;
            if nat_eq(a1833, a1834) {
                let a1836 = match v1.clone() {
                    Adt1::C0(h1835, _, _, _, _, _, _, _, _, _, _, _) => {
                        h1835
                    }
                };
                let a1837 = 2147483648u64;
                if if if if if nat_lt(a1836, a1837) {
                    let a1839 = match v1.clone() {
                        Adt1::C0(_, h1838, _, _, _, _, _, _, _, _, _, _) => {
                            h1838
                        }
                    };
                    let a1840 = 2147483648u64;
                    nat_lt(a1839, a1840)
                } else {
                    false
                } {
                    let a1843 = match v1.clone() {
                        Adt1::C0(h1841, _, _, _, _, _, _, _, _, _, _, _) => {
                            h1841
                        }
                    };
                    let a1844 = match v2.clone() {
                        Adt2::C0(_, h1842, _, _, _, _, _, _, _, _, _) => {
                            h1842
                        }
                    };
                    nat_lt(a1843, a1844)
                } else {
                    false
                } {
                    let a1847 = match v1.clone() {
                        Adt1::C0(_, h1845, _, _, _, _, _, _, _, _, _, _) => {
                            h1845
                        }
                    };
                    let a1848 = match v2.clone() {
                        Adt2::C0(_, h1846, _, _, _, _, _, _, _, _, _) => {
                            h1846
                        }
                    };
                    nat_lt(a1847, a1848)
                } else {
                    false
                } {
                    let a1851 = match v2.clone() {
                        Adt2::C0(_, _, h1849, _, _, _, _, _, _, _, _) => {
                            h1849
                        }
                    };
                    let a1852 = match v3.clone() {
                        Adt2::C0(_, _, h1850, _, _, _, _, _, _, _, _) => {
                            h1850
                        }
                    };
                    if if if nat_eq(a1851, a1852) {
                        let a1855 = match v2.clone() {
                            Adt2::C0(_, _, _, h1853, _, _, _, _, _, _, _) => {
                                h1853
                            }
                        };
                        let a1856 = match v3.clone() {
                            Adt2::C0(_, _, _, h1854, _, _, _, _, _, _, _) => {
                                h1854
                            }
                        };
                        nat_eq(a1855, a1856)
                    } else {
                        false
                    } {
                        let a1859 = match v2.clone() {
                            Adt2::C0(_, _, _, _, h1857, _, _, _, _, _, _) => {
                                h1857
                            }
                        };
                        let a1860 = match v3.clone() {
                            Adt2::C0(_, _, _, _, h1858, _, _, _, _, _, _) => {
                                h1858
                            }
                        };
                        nat_eq(a1859, a1860)
                    } else {
                        false
                    } {
                        let a1869 = match v3.clone() {
                            Adt2::C0(_, h1861, _, _, _, _, _, _, _, _, _) => {
                                h1861
                            }
                        };
                        let a1870 = {
                            let a1867 = {
                                let a1864 = match v2.clone() {
                                    Adt2::C0(_, h1862, _, _, _, _, _, _, _, _, _) => {
                                        h1862
                                    }
                                };
                                let a1865 = match v1.clone() {
                                    Adt1::C0(h1863, _, _, _, _, _, _, _, _, _, _, _) => {
                                        h1863
                                    }
                                };
                                nat_add(a1864, a1865)?
                            };
                            let a1868 = match v1.clone() {
                                Adt1::C0(_, h1866, _, _, _, _, _, _, _, _, _, _) => {
                                    h1866
                                }
                            };
                            nat_add(a1867, a1868)?
                        };
                        nat_eq(a1869, a1870)
                    } else {
                        false
                    }
                } else {
                    false
                } {
                    Ok(Adt10::C4(f16(v3.clone(), 1u64)?, 4u64, f20(v2, 1u64), f20(v3, 1u64), Adt4::C4(match v1 {
                        Adt1::C0(h1871, _, _, _, _, _, _, _, _, _, _, _) => {
                            h1871
                        }
                    })))
                } else {
                    Ok(Adt10::C1)
                }
            } else {
                let a1872 = v0;
                let a1873 = 63u64;
                if nat_eq(a1872, a1873) {
                    if if if if if f136(match v2.clone() {
                        Adt2::C0(_, h1874, _, _, _, _, _, _, _, _, _) => {
                            h1874
                        }
                    }, match v1.clone() {
                        Adt1::C0(h1875, _, _, _, _, _, _, _, _, _, _, _) => {
                            h1875
                        }
                    })? {
                        f136(match v2.clone() {
                            Adt2::C0(_, _, h1876, _, _, _, _, _, _, _, _) => {
                                h1876
                            }
                        }, match v1.clone() {
                            Adt1::C0(_, h1877, _, _, _, _, _, _, _, _, _, _) => {
                                h1877
                            }
                        })?
                    } else {
                        false
                    } {
                        f136(match v2.clone() {
                            Adt2::C0(_, _, _, h1878, _, _, _, _, _, _, _) => {
                                h1878
                            }
                        }, match v1.clone() {
                            Adt1::C0(_, _, h1879, _, _, _, _, _, _, _, _, _) => {
                                h1879
                            }
                        })?
                    } else {
                        false
                    } {
                        f136(match v2.clone() {
                            Adt2::C0(_, _, _, _, h1880, _, _, _, _, _, _) => {
                                h1880
                            }
                        }, match v1.clone() {
                            Adt1::C0(_, _, _, h1881, _, _, _, _, _, _, _, _) => {
                                h1881
                            }
                        })?
                    } else {
                        false
                    } {
                        let a1884 = match v2.clone() {
                            Adt2::C0(_, h1882, _, _, _, _, _, _, _, _, _) => {
                                h1882
                            }
                        };
                        let a1885 = match v3.clone() {
                            Adt2::C0(_, h1883, _, _, _, _, _, _, _, _, _) => {
                                h1883
                            }
                        };
                        if if if nat_eq(a1884, a1885) {
                            let a1888 = match v2.clone() {
                                Adt2::C0(_, _, h1886, _, _, _, _, _, _, _, _) => {
                                    h1886
                                }
                            };
                            let a1889 = match v3.clone() {
                                Adt2::C0(_, _, h1887, _, _, _, _, _, _, _, _) => {
                                    h1887
                                }
                            };
                            nat_eq(a1888, a1889)
                        } else {
                            false
                        } {
                            let a1892 = match v2.clone() {
                                Adt2::C0(_, _, _, h1890, _, _, _, _, _, _, _) => {
                                    h1890
                                }
                            };
                            let a1893 = match v3.clone() {
                                Adt2::C0(_, _, _, h1891, _, _, _, _, _, _, _) => {
                                    h1891
                                }
                            };
                            nat_eq(a1892, a1893)
                        } else {
                            false
                        } {
                            let a1896 = match v2.clone() {
                                Adt2::C0(_, _, _, _, h1894, _, _, _, _, _, _) => {
                                    h1894
                                }
                            };
                            let a1897 = match v3.clone() {
                                Adt2::C0(_, _, _, _, h1895, _, _, _, _, _, _) => {
                                    h1895
                                }
                            };
                            nat_eq(a1896, a1897)
                        } else {
                            false
                        }
                    } else {
                        false
                    } {
                        Ok(Adt10::C4(f16(v3.clone(), 1u64)?, 4u64, f20(v2, 1u64), f20(v3, 1u64), Adt4::C5(match v1.clone() {
                            Adt1::C0(h1898, _, _, _, _, _, _, _, _, _, _, _) => {
                                h1898
                            }
                        }, match v1.clone() {
                            Adt1::C0(_, h1899, _, _, _, _, _, _, _, _, _, _) => {
                                h1899
                            }
                        }, match v1.clone() {
                            Adt1::C0(_, _, h1900, _, _, _, _, _, _, _, _, _) => {
                                h1900
                            }
                        }, match v1 {
                            Adt1::C0(_, _, _, h1901, _, _, _, _, _, _, _, _) => {
                                h1901
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

pub fn f61(v0: u64, v1: Adt1, v2: Adt2, v3: Option<Adt2>, v4: Adt2) -> R<Adt10> {
    let m1902 = v3;
    match m1902 {
        None => {
            Ok(Adt10::C1)
        }
        Some(v5) => {
            let v6: bool = {
                let a1903 = v0;
                let a1904 = 4u64;
                nat_eq(a1903, a1904)
            };
            let v7: bool = {
                let a1906 = 0u64;
                let a1907 = match v1.clone() {
                    Adt1::C0(_, _, _, _, h1905, _, _, _, _, _, _, _) => {
                        h1905
                    }
                };
                nat_lt(a1906, a1907)
            };
            let v8: u64 = f16(v5.clone(), 1u64)?;
            let v9: Adt3 = Adt3::C0(match v5.clone() {
                Adt2::C0(_, h1908, _, _, _, _, _, _, _, _, _) => {
                    h1908
                }
            }, match v5.clone() {
                Adt2::C0(_, _, h1909, _, _, _, _, _, _, _, _) => {
                    h1909
                }
            }, match v5.clone() {
                Adt2::C0(_, _, _, h1910, _, _, _, _, _, _, _) => {
                    h1910
                }
            }, match v5.clone() {
                Adt2::C0(_, _, _, _, h1911, _, _, _, _, _, _) => {
                    h1911
                }
            }, 4u64, match v1.clone() {
                Adt1::C0(h1912, _, _, _, _, _, _, _, _, _, _, _) => {
                    h1912
                }
            }, match v1.clone() {
                Adt1::C0(_, h1913, _, _, _, _, _, _, _, _, _, _) => {
                    h1913
                }
            }, match v1.clone() {
                Adt1::C0(_, _, h1914, _, _, _, _, _, _, _, _, _) => {
                    h1914
                }
            });
            if if if if if if if if if if if if v6 {
                let a1916 = match v2.clone() {
                    Adt2::C0(h1915, _, _, _, _, _, _, _, _, _, _) => {
                        h1915
                    }
                };
                let a1917 = 0u64;
                if nat_eq(a1916, a1917) {
                    let a1919 = match v5.clone() {
                        Adt2::C0(h1918, _, _, _, _, _, _, _, _, _, _) => {
                            h1918
                        }
                    };
                    let a1920 = 0u64;
                    nat_eq(a1919, a1920)
                } else {
                    false
                }
            } else {
                let a1922 = match v2.clone() {
                    Adt2::C0(h1921, _, _, _, _, _, _, _, _, _, _) => {
                        h1921
                    }
                };
                let a1923 = 0u64;
                if if nat_eq(a1922, a1923) {
                    true
                } else {
                    let a1925 = match v2.clone() {
                        Adt2::C0(h1924, _, _, _, _, _, _, _, _, _, _) => {
                            h1924
                        }
                    };
                    let a1926 = 26u64;
                    nat_eq(a1925, a1926)
                } {
                    let a1928 = match v5.clone() {
                        Adt2::C0(h1927, _, _, _, _, _, _, _, _, _, _) => {
                            h1927
                        }
                    };
                    let a1929 = 0u64;
                    if nat_eq(a1928, a1929) {
                        true
                    } else {
                        let a1931 = match v5.clone() {
                            Adt2::C0(h1930, _, _, _, _, _, _, _, _, _, _) => {
                                h1930
                            }
                        };
                        let a1932 = 26u64;
                        nat_eq(a1931, a1932)
                    }
                } else {
                    false
                }
            } {
                let a1935 = match v2.clone() {
                    Adt2::C0(h1933, _, _, _, _, _, _, _, _, _, _) => {
                        h1933
                    }
                };
                let a1936 = match v4.clone() {
                    Adt2::C0(h1934, _, _, _, _, _, _, _, _, _, _) => {
                        h1934
                    }
                };
                nat_eq(a1935, a1936)
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
                let a1938 = match v5.clone() {
                    Adt2::C0(_, _, _, _, _, h1937, _, _, _, _, _) => {
                        h1937
                    }
                };
                let a1939 = 4u64;
                nat_eq(a1938, a1939)
            } else {
                false
            } {
                let a1940 = v8;
                let a1941 = f16(v2.clone(), 1u64)?;
                nat_le(a1940, a1941)
            } else {
                false
            } {
                let a1943 = match v1.clone() {
                    Adt1::C0(_, _, _, h1942, _, _, _, _, _, _, _, _) => {
                        h1942
                    }
                };
                let a1944 = 1073741824u64;
                nat_lt(a1943, a1944)
            } else {
                false
            } {
                let a1946 = match v1.clone() {
                    Adt1::C0(h1945, _, _, _, _, _, _, _, _, _, _, _) => {
                        h1945
                    }
                };
                let a1947 = 4294967295u64;
                if if if if nat_le(a1946, a1947) {
                    let a1949 = match v1.clone() {
                        Adt1::C0(_, h1948, _, _, _, _, _, _, _, _, _, _) => {
                            h1948
                        }
                    };
                    let a1950 = 4294967295u64;
                    nat_le(a1949, a1950)
                } else {
                    false
                } {
                    let a1952 = match v1.clone() {
                        Adt1::C0(_, _, h1951, _, _, _, _, _, _, _, _, _) => {
                            h1951
                        }
                    };
                    let a1953 = 4294967295u64;
                    nat_le(a1952, a1953)
                } else {
                    false
                } {
                    let a1955 = match v1.clone() {
                        Adt1::C0(_, _, _, h1954, _, _, _, _, _, _, _, _) => {
                            h1954
                        }
                    };
                    let a1956 = 4294967295u64;
                    nat_le(a1955, a1956)
                } else {
                    false
                } {
                    let a1958 = match v1.clone() {
                        Adt1::C0(_, _, _, _, h1957, _, _, _, _, _, _, _) => {
                            h1957
                        }
                    };
                    let a1959 = 4294967295u64;
                    nat_le(a1958, a1959)
                } else {
                    false
                }
            } else {
                false
            } {
                if v7 {
                    let a1962 = match v2.clone() {
                        Adt2::C0(_, _, _, _, _, _, _, _, _, _, h1960) => {
                            h1960
                        }
                    };
                    let a1963 = match v4.clone() {
                        Adt2::C0(_, _, _, _, _, _, _, _, _, _, h1961) => {
                            h1961
                        }
                    };
                    nat_eq(a1962, a1963)
                } else {
                    true
                }
            } else {
                false
            } {
                f137(v9.clone(), match v1.clone() {
                    Adt1::C0(_, _, _, h1964, _, _, _, _, _, _, _, _) => {
                        h1964
                    }
                }, match v4 {
                    Adt2::C0(_, _, _, _, _, _, _, _, _, h1965, _) => {
                        h1965
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
                    let a1966 = v10;
                    let a1967 = v8;
                    nat_add(a1966, a1967)?
                }, 4u64, f20(v5, 1u64), v9, Adt4::C6(v10, match v1 {
                    Adt1::C0(_, _, _, h1968, _, _, _, _, _, _, _, _) => {
                        h1968
                    }
                }, v6)))
            } else {
                Ok(Adt10::C1)
            }
        }
    }
}

pub fn f62(v0: Adt2, v1: Adt2) -> R<Adt10> {
    let a1970 = match v0.clone() {
        Adt2::C0(h1969, _, _, _, _, _, _, _, _, _, _) => {
            h1969
        }
    };
    let a1971 = 0u64;
    if if if nat_eq(a1970, a1971) {
        let a1973 = match v1.clone() {
            Adt2::C0(h1972, _, _, _, _, _, _, _, _, _, _) => {
                h1972
            }
        };
        let a1974 = 0u64;
        nat_eq(a1973, a1974)
    } else {
        false
    } {
        f133(match v0.clone() {
            Adt2::C0(_, h1975, _, _, _, _, _, _, _, _, _) => {
                h1975
            }
        }, match v0.clone() {
            Adt2::C0(_, _, h1976, _, _, _, _, _, _, _, _) => {
                h1976
            }
        }, match v0.clone() {
            Adt2::C0(_, _, _, h1977, _, _, _, _, _, _, _) => {
                h1977
            }
        }, match v0.clone() {
            Adt2::C0(_, _, _, _, h1978, _, _, _, _, _, _) => {
                h1978
            }
        }, match v1.clone() {
            Adt2::C0(_, h1979, _, _, _, _, _, _, _, _, _) => {
                h1979
            }
        }, match v1.clone() {
            Adt2::C0(_, _, h1980, _, _, _, _, _, _, _, _) => {
                h1980
            }
        }, match v1.clone() {
            Adt2::C0(_, _, _, h1981, _, _, _, _, _, _, _) => {
                h1981
            }
        }, match v1.clone() {
            Adt2::C0(_, _, _, _, h1982, _, _, _, _, _, _) => {
                h1982
            }
        })
    } else {
        false
    } {
        let v2: u64 = f16(v1.clone(), 1u64)?;
        Ok(Adt10::C4({
            let a1983 = v2;
            let a1984 = f16(v0.clone(), 1u64)?;
            nat_add(a1983, a1984)?
        }, 4u64, f20(v0, 1u64), f20(v1, 1u64), Adt4::C7(v2)))
    } else {
        Ok(Adt10::C1)
    }
}

pub fn f63(v0: u64, v1: Adt2, v2: Adt2) -> R<Adt10> {
    let a1986 = match v1.clone() {
        Adt2::C0(h1985, _, _, _, _, _, _, _, _, _, _) => {
            h1985
        }
    };
    let a1987 = 0u64;
    if if if nat_eq(a1986, a1987) {
        true
    } else {
        let a1989 = match v1.clone() {
            Adt2::C0(h1988, _, _, _, _, _, _, _, _, _, _) => {
                h1988
            }
        };
        let a1990 = 1u64;
        nat_eq(a1989, a1990)
    } {
        true
    } else {
        let a1992 = match v1.clone() {
            Adt2::C0(h1991, _, _, _, _, _, _, _, _, _, _) => {
                h1991
            }
        };
        let a1993 = 30u64;
        nat_eq(a1992, a1993)
    } {
        let a1996 = match v1.clone() {
            Adt2::C0(h1994, _, _, _, _, _, _, _, _, _, _) => {
                h1994
            }
        };
        let a1997 = match v2.clone() {
            Adt2::C0(h1995, _, _, _, _, _, _, _, _, _, _) => {
                h1995
            }
        };
        if if if if if nat_eq(a1996, a1997) {
            let a2000 = match v1.clone() {
                Adt2::C0(_, h1998, _, _, _, _, _, _, _, _, _) => {
                    h1998
                }
            };
            let a2001 = match v2.clone() {
                Adt2::C0(_, h1999, _, _, _, _, _, _, _, _, _) => {
                    h1999
                }
            };
            nat_eq(a2000, a2001)
        } else {
            false
        } {
            let a2004 = match v1.clone() {
                Adt2::C0(_, _, _, h2002, _, _, _, _, _, _, _) => {
                    h2002
                }
            };
            let a2005 = match v2.clone() {
                Adt2::C0(_, _, _, h2003, _, _, _, _, _, _, _) => {
                    h2003
                }
            };
            nat_eq(a2004, a2005)
        } else {
            false
        } {
            let a2008 = match v1.clone() {
                Adt2::C0(_, _, _, _, h2006, _, _, _, _, _, _) => {
                    h2006
                }
            };
            let a2009 = match v2.clone() {
                Adt2::C0(_, _, _, _, h2007, _, _, _, _, _, _) => {
                    h2007
                }
            };
            nat_eq(a2008, a2009)
        } else {
            false
        } {
            let a2010 = v0;
            let a2011 = 4u64;
            if nat_eq(a2010, a2011) {
                let a2013 = match v1.clone() {
                    Adt2::C0(_, _, h2012, _, _, _, _, _, _, _, _) => {
                        h2012
                    }
                };
                let a2014 = 1u64;
                if nat_eq(a2013, a2014) {
                    let a2017 = match v1.clone() {
                        Adt2::C0(_, h2015, _, _, _, _, _, _, _, _, _) => {
                            h2015
                        }
                    };
                    let a2018 = match v2.clone() {
                        Adt2::C0(_, _, h2016, _, _, _, _, _, _, _, _) => {
                            h2016
                        }
                    };
                    nat_eq(a2017, a2018)
                } else {
                    false
                }
            } else {
                let a2019 = v0;
                let a2020 = 4u64;
                if if nat_lt(a2019, a2020) {
                    let a2023 = match v1.clone() {
                        Adt2::C0(_, h2021, _, _, _, _, _, _, _, _, _) => {
                            h2021
                        }
                    };
                    let a2024 = match v1.clone() {
                        Adt2::C0(_, _, h2022, _, _, _, _, _, _, _, _) => {
                            h2022
                        }
                    };
                    nat_eq(a2023, a2024)
                } else {
                    false
                } {
                    let a2027 = match v1.clone() {
                        Adt2::C0(_, _, h2025, _, _, _, _, _, _, _, _) => {
                            h2025
                        }
                    };
                    let a2028 = match v2.clone() {
                        Adt2::C0(_, _, h2026, _, _, _, _, _, _, _, _) => {
                            h2026
                        }
                    };
                    nat_eq(a2027, a2028)
                } else {
                    false
                }
            }
        } else {
            false
        } {
            let m2030 = f130(match v1.clone() {
                Adt2::C0(h2029, _, _, _, _, _, _, _, _, _, _) => {
                    h2029
                }
            });
            match m2030 {
                Adt17::C0 => {
                    Ok(Adt10::C0)
                }
                Adt17::C1(v3) => {
                    Ok(Adt10::C4(f16(v2.clone(), 1u64)?, match v3 {
                        Adt18::C0(_, _, h2031) => {
                            h2031
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

pub fn f64(v0: u64, v1: Adt2, v2: Adt2) -> R<Adt10> {
    if if if f138(match v1.clone() {
        Adt2::C0(h2032, _, _, _, _, _, _, _, _, _, _) => {
            h2032
        }
    }, v0) {
        let a2035 = match v1.clone() {
            Adt2::C0(h2033, _, _, _, _, _, _, _, _, _, _) => {
                h2033
            }
        };
        let a2036 = match v2.clone() {
            Adt2::C0(h2034, _, _, _, _, _, _, _, _, _, _) => {
                h2034
            }
        };
        nat_eq(a2035, a2036)
    } else {
        false
    } {
        let a2039 = match v1.clone() {
            Adt2::C0(_, h2037, _, _, _, _, _, _, _, _, _) => {
                h2037
            }
        };
        let a2040 = match v2.clone() {
            Adt2::C0(_, h2038, _, _, _, _, _, _, _, _, _) => {
                h2038
            }
        };
        if if if nat_eq(a2039, a2040) {
            let a2043 = match v1.clone() {
                Adt2::C0(_, _, h2041, _, _, _, _, _, _, _, _) => {
                    h2041
                }
            };
            let a2044 = match v2.clone() {
                Adt2::C0(_, _, h2042, _, _, _, _, _, _, _, _) => {
                    h2042
                }
            };
            nat_eq(a2043, a2044)
        } else {
            false
        } {
            let a2047 = match v1.clone() {
                Adt2::C0(_, _, _, h2045, _, _, _, _, _, _, _) => {
                    h2045
                }
            };
            let a2048 = match v2.clone() {
                Adt2::C0(_, _, _, h2046, _, _, _, _, _, _, _) => {
                    h2046
                }
            };
            nat_eq(a2047, a2048)
        } else {
            false
        } {
            let a2051 = match v1.clone() {
                Adt2::C0(_, _, _, _, h2049, _, _, _, _, _, _) => {
                    h2049
                }
            };
            let a2052 = match v2.clone() {
                Adt2::C0(_, _, _, _, h2050, _, _, _, _, _, _) => {
                    h2050
                }
            };
            nat_eq(a2051, a2052)
        } else {
            false
        }
    } else {
        false
    } {
        let m2054 = f130(match v1.clone() {
            Adt2::C0(h2053, _, _, _, _, _, _, _, _, _, _) => {
                h2053
            }
        });
        match m2054 {
            Adt17::C0 => {
                Ok(Adt10::C0)
            }
            Adt17::C1(v3) => {
                Ok(Adt10::C4(f16(v2.clone(), 1u64)?, match v3 {
                    Adt18::C0(_, _, h2055) => {
                        h2055
                    }
                }, f20(v1.clone(), 1u64), f20(v2, 1u64), Adt4::C9(match v1 {
                    Adt2::C0(h2056, _, _, _, _, _, _, _, _, _, _) => {
                        h2056
                    }
                }, v0)))
            }
        }
    } else {
        Ok(Adt10::C1)
    }
}

pub fn f65(v0: bool, v1: Adt1, v2: Adt2, v3: Adt2) -> R<Adt10> {
    let v4: u64 = if v0 {
        match v1.clone() {
            Adt1::C0(h2057, _, _, _, _, _, _, _, _, _, _, _) => {
                h2057
            }
        }
    } else {
        match v1.clone() {
            Adt1::C0(_, _, h2058, _, _, _, _, _, _, _, _, _) => {
                h2058
            }
        }
    };
    let a2059 = 0u64;
    let a2060 = v4;
    if if nat_lt(a2059, a2060) {
        let a2061 = v4;
        let a2062 = 2147483648u64;
        nat_lt(a2061, a2062)
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
        let v7: u64 = f139(match v5.clone() {
            Adt2::C0(_, _, h2063, _, _, _, _, _, _, _, _) => {
                h2063
            }
        }, v4)?;
        let v8: u64 = f139(match v5.clone() {
            Adt2::C0(_, _, _, h2064, _, _, _, _, _, _, _) => {
                h2064
            }
        }, v4)?;
        let a2066 = match v5.clone() {
            Adt2::C0(h2065, _, _, _, _, _, _, _, _, _, _) => {
                h2065
            }
        };
        let a2067 = 0u64;
        if if if if if nat_eq(a2066, a2067) {
            let a2069 = match v6.clone() {
                Adt2::C0(h2068, _, _, _, _, _, _, _, _, _, _) => {
                    h2068
                }
            };
            let a2070 = 0u64;
            nat_eq(a2069, a2070)
        } else {
            false
        } {
            let a2072 = match v5.clone() {
                Adt2::C0(_, _, _, _, h2071, _, _, _, _, _, _) => {
                    h2071
                }
            };
            let a2073 = 1u64;
            nat_eq(a2072, a2073)
        } else {
            false
        } {
            let a2076 = match v5.clone() {
                Adt2::C0(_, h2074, _, _, _, _, _, _, _, _, _) => {
                    h2074
                }
            };
            let a2077 = match v6.clone() {
                Adt2::C0(_, h2075, _, _, _, _, _, _, _, _, _) => {
                    h2075
                }
            };
            nat_eq(a2076, a2077)
        } else {
            false
        } {
            if v0 {
                let a2079 = match v5.clone() {
                    Adt2::C0(_, _, h2078, _, _, _, _, _, _, _, _) => {
                        h2078
                    }
                };
                let a2080 = 2147483648u64;
                if if if nat_lt(a2079, a2080) {
                    let a2082 = match v5.clone() {
                        Adt2::C0(_, _, _, h2081, _, _, _, _, _, _, _) => {
                            h2081
                        }
                    };
                    let a2083 = 2147483648u64;
                    nat_lt(a2082, a2083)
                } else {
                    false
                } {
                    let a2086 = {
                        let a2084 = v7;
                        let a2085 = v8;
                        nat_mul(a2084, a2085)?
                    };
                    let a2087 = 2147483648u64;
                    nat_le(a2086, a2087)
                } else {
                    false
                } {
                    f140(v5, v6, v4, v7)?
                } else {
                    false
                }
            } else {
                let a2089 = match v1.clone() {
                    Adt1::C0(h2088, _, _, _, _, _, _, _, _, _, _, _) => {
                        h2088
                    }
                };
                let a2090 = v7;
                if if if if nat_eq(a2089, a2090) {
                    let a2092 = match v1 {
                        Adt1::C0(_, h2091, _, _, _, _, _, _, _, _, _, _) => {
                            h2091
                        }
                    };
                    let a2093 = v8;
                    nat_eq(a2092, a2093)
                } else {
                    false
                } {
                    f141(v7, v8, v4)?
                } else {
                    false
                } {
                    let a2095 = match v6.clone() {
                        Adt2::C0(_, _, h2094, _, _, _, _, _, _, _, _) => {
                            h2094
                        }
                    };
                    let a2096 = v4;
                    if nat_eq(a2095, a2096) {
                        let a2098 = match v6.clone() {
                            Adt2::C0(_, _, _, h2097, _, _, _, _, _, _, _) => {
                                h2097
                            }
                        };
                        let a2099 = v4;
                        nat_eq(a2098, a2099)
                    } else {
                        false
                    }
                } else {
                    false
                } {
                    let a2103 = match v6 {
                        Adt2::C0(_, _, _, _, h2100, _, _, _, _, _, _) => {
                            h2100
                        }
                    };
                    let a2104 = {
                        let a2101 = v7;
                        let a2102 = v8;
                        nat_mul(a2101, a2102)?
                    };
                    nat_eq(a2103, a2104)
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

pub fn f66(v0: u64, v1: Adt1, v2: Adt2, v3: Adt2) -> R<Adt10> {
    let a2106 = match v2.clone() {
        Adt2::C0(h2105, _, _, _, _, _, _, _, _, _, _) => {
            h2105
        }
    };
    let a2107 = 0u64;
    if if if if if nat_eq(a2106, a2107) {
        true
    } else {
        let a2109 = match v2.clone() {
            Adt2::C0(h2108, _, _, _, _, _, _, _, _, _, _) => {
                h2108
            }
        };
        let a2110 = 1u64;
        nat_eq(a2109, a2110)
    } {
        true
    } else {
        let a2112 = match v2.clone() {
            Adt2::C0(h2111, _, _, _, _, _, _, _, _, _, _) => {
                h2111
            }
        };
        let a2113 = 30u64;
        nat_eq(a2112, a2113)
    } {
        let a2116 = match v2.clone() {
            Adt2::C0(h2114, _, _, _, _, _, _, _, _, _, _) => {
                h2114
            }
        };
        let a2117 = match v3.clone() {
            Adt2::C0(h2115, _, _, _, _, _, _, _, _, _, _) => {
                h2115
            }
        };
        nat_eq(a2116, a2117)
    } else {
        false
    } {
        if if f25(v2.clone(), v3.clone()) {
            let a2119 = match v1.clone() {
                Adt1::C0(h2118, _, _, _, _, _, _, _, _, _, _, _) => {
                    h2118
                }
            };
            let a2120 = 4294967296u64;
            nat_lt(a2119, a2120)
        } else {
            false
        } {
            let a2122 = match v1.clone() {
                Adt1::C0(_, h2121, _, _, _, _, _, _, _, _, _, _) => {
                    h2121
                }
            };
            let a2123 = 4294967296u64;
            nat_lt(a2122, a2123)
        } else {
            false
        }
    } else {
        false
    } {
        Ok(Adt10::C4(f16(v3.clone(), 1u64)?, {
            let a2125 = match v2.clone() {
                Adt2::C0(h2124, _, _, _, _, _, _, _, _, _, _) => {
                    h2124
                }
            };
            let a2126 = 0u64;
            if nat_eq(a2125, a2126) {
                4u64
            } else {
                2u64
            }
        }, f20(v2.clone(), 1u64), f20(v3, 1u64), Adt4::C13({
            let a2127 = v0;
            let a2128 = 31u64;
            if nat_eq(a2127, a2128) {
                0u64
            } else {
                let a2129 = v0;
                let a2130 = 47u64;
                if nat_eq(a2129, a2130) {
                    1u64
                } else {
                    2u64
                }
            }
        }, match v2 {
            Adt2::C0(h2131, _, _, _, _, _, _, _, _, _, _) => {
                h2131
            }
        }, match v1.clone() {
            Adt1::C0(h2132, _, _, _, _, _, _, _, _, _, _, _) => {
                h2132
            }
        }, match v1 {
            Adt1::C0(_, h2133, _, _, _, _, _, _, _, _, _, _) => {
                h2133
            }
        })))
    } else {
        Ok(Adt10::C1)
    }
}

pub fn f67(v0: Adt2, v1: Adt2) -> R<Adt10> {
    let a2135 = match v0.clone() {
        Adt2::C0(h2134, _, _, _, _, _, _, _, _, _, _) => {
            h2134
        }
    };
    let a2136 = 0u64;
    if if if nat_eq(a2135, a2136) {
        let a2138 = match v1.clone() {
            Adt2::C0(h2137, _, _, _, _, _, _, _, _, _, _) => {
                h2137
            }
        };
        let a2139 = 0u64;
        nat_eq(a2138, a2139)
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

pub fn f68(v0: Adt2, v1: Adt2) -> R<Adt10> {
    let a2141 = match v0.clone() {
        Adt2::C0(h2140, _, _, _, _, _, _, _, _, _, _) => {
            h2140
        }
    };
    let a2142 = 0u64;
    if if nat_eq(a2141, a2142) {
        let a2144 = match v1.clone() {
            Adt2::C0(h2143, _, _, _, _, _, _, _, _, _, _) => {
                h2143
            }
        };
        let a2145 = 26u64;
        nat_eq(a2144, a2145)
    } else {
        false
    } {
        let a2147 = match v0.clone() {
            Adt2::C0(_, _, _, h2146, _, _, _, _, _, _, _) => {
                h2146
            }
        };
        let a2148 = 1u64;
        if if if if if if if nat_eq(a2147, a2148) {
            let a2150 = match v0.clone() {
                Adt2::C0(_, _, _, _, h2149, _, _, _, _, _, _) => {
                    h2149
                }
            };
            let a2151 = 1u64;
            nat_eq(a2150, a2151)
        } else {
            false
        } {
            let a2153 = match v0.clone() {
                Adt2::C0(_, h2152, _, _, _, _, _, _, _, _, _) => {
                    h2152
                }
            };
            let a2154 = 2147483647u64;
            nat_le(a2153, a2154)
        } else {
            false
        } {
            let a2157 = match v1.clone() {
                Adt2::C0(_, h2155, _, _, _, _, _, _, _, _, _) => {
                    h2155
                }
            };
            let a2158 = match v0.clone() {
                Adt2::C0(_, _, h2156, _, _, _, _, _, _, _, _) => {
                    h2156
                }
            };
            nat_eq(a2157, a2158)
        } else {
            false
        } {
            let a2160 = match v1.clone() {
                Adt2::C0(_, _, h2159, _, _, _, _, _, _, _, _) => {
                    h2159
                }
            };
            let a2161 = 1u64;
            nat_eq(a2160, a2161)
        } else {
            false
        } {
            let a2163 = match v1.clone() {
                Adt2::C0(_, _, _, h2162, _, _, _, _, _, _, _) => {
                    h2162
                }
            };
            let a2164 = 1u64;
            nat_eq(a2163, a2164)
        } else {
            false
        } {
            let a2166 = match v1.clone() {
                Adt2::C0(_, _, _, _, h2165, _, _, _, _, _, _) => {
                    h2165
                }
            };
            let a2167 = 1u64;
            nat_eq(a2166, a2167)
        } else {
            false
        } {
            Ok(Adt10::C4({
                let a2170 = match v0.clone() {
                    Adt2::C0(_, _, h2168, _, _, _, _, _, _, _, _) => {
                        h2168
                    }
                };
                let a2171 = f142(match v0.clone() {
                    Adt2::C0(_, h2169, _, _, _, _, _, _, _, _, _) => {
                        h2169
                    }
                })?;
                nat_mul(a2170, a2171)?
            }, 4u64, f20(v0, 1u64), f20(v1, 1u64), Adt4::C17))
        } else {
            Ok(Adt10::C1)
        }
    } else {
        Ok(Adt10::C0)
    }
}

pub fn f69(v0: Adt2, v1: Option<Adt2>, v2: Adt2) -> R<Adt10> {
    let m2172 = v1;
    match m2172 {
        None => {
            Ok(Adt10::C1)
        }
        Some(v3) => {
            let v4: u64 = f143(f20(v0.clone(), 1u64))?;
            let a2174 = match v0.clone() {
                Adt2::C0(h2173, _, _, _, _, _, _, _, _, _, _) => {
                    h2173
                }
            };
            let a2175 = 26u64;
            if if if nat_eq(a2174, a2175) {
                let a2177 = match v3.clone() {
                    Adt2::C0(h2176, _, _, _, _, _, _, _, _, _, _) => {
                        h2176
                    }
                };
                let a2178 = 26u64;
                nat_eq(a2177, a2178)
            } else {
                false
            } {
                let a2180 = match v2.clone() {
                    Adt2::C0(h2179, _, _, _, _, _, _, _, _, _, _) => {
                        h2179
                    }
                };
                let a2181 = 27u64;
                nat_eq(a2180, a2181)
            } else {
                false
            } {
                if if if if if if f25(v0.clone(), v3.clone()) {
                    let a2183 = match v2.clone() {
                        Adt2::C0(_, h2182, _, _, _, _, _, _, _, _, _) => {
                            h2182
                        }
                    };
                    let a2184 = 1u64;
                    nat_eq(a2183, a2184)
                } else {
                    false
                } {
                    let a2186 = match v2.clone() {
                        Adt2::C0(_, _, h2185, _, _, _, _, _, _, _, _) => {
                            h2185
                        }
                    };
                    let a2187 = 1u64;
                    nat_eq(a2186, a2187)
                } else {
                    false
                } {
                    let a2189 = match v2.clone() {
                        Adt2::C0(_, _, _, h2188, _, _, _, _, _, _, _) => {
                            h2188
                        }
                    };
                    let a2190 = 1u64;
                    nat_eq(a2189, a2190)
                } else {
                    false
                } {
                    let a2192 = match v2.clone() {
                        Adt2::C0(_, _, _, _, h2191, _, _, _, _, _, _) => {
                            h2191
                        }
                    };
                    let a2193 = 1u64;
                    nat_eq(a2192, a2193)
                } else {
                    false
                } {
                    let a2194 = v4;
                    let a2195 = 9223372036854775808u64;
                    nat_lt(a2194, a2195)
                } else {
                    false
                } {
                    Ok(Adt10::C4({
                        let a2198 = 2u64;
                        let a2199 = {
                            let a2196 = v4;
                            let a2197 = 2u64;
                            nat_mul(a2196, a2197)?
                        };
                        nat_add(a2198, a2199)?
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

pub fn f70(v0: u64, v1: Adt2, v2: Adt2) -> R<Adt10> {
    let v3: u64 = match v1.clone() {
        Adt2::C0(h2200, _, _, _, _, _, _, _, _, _, _) => {
            h2200
        }
    };
    let a2201 = v3;
    let a2202 = 0u64;
    if if nat_eq(a2201, a2202) {
        true
    } else {
        let a2203 = v0;
        let a2204 = 13u64;
        if nat_eq(a2203, a2204) {
            let a2205 = v3;
            let a2206 = 1u64;
            if nat_eq(a2205, a2206) {
                true
            } else {
                let a2207 = v3;
                let a2208 = 30u64;
                nat_eq(a2207, a2208)
            }
        } else {
            false
        }
    } {
        let a2210 = v3;
        let a2211 = match v2.clone() {
            Adt2::C0(h2209, _, _, _, _, _, _, _, _, _, _) => {
                h2209
            }
        };
        if if nat_eq(a2210, a2211) {
            let a2212 = v0;
            let a2213 = 13u64;
            if nat_eq(a2212, a2213) {
                let a2215 = match v2.clone() {
                    Adt2::C0(_, h2214, _, _, _, _, _, _, _, _, _) => {
                        h2214
                    }
                };
                let a2216 = 1u64;
                if if if nat_eq(a2215, a2216) {
                    let a2218 = match v2.clone() {
                        Adt2::C0(_, _, h2217, _, _, _, _, _, _, _, _) => {
                            h2217
                        }
                    };
                    let a2219 = 1u64;
                    nat_eq(a2218, a2219)
                } else {
                    false
                } {
                    let a2221 = match v2.clone() {
                        Adt2::C0(_, _, _, h2220, _, _, _, _, _, _, _) => {
                            h2220
                        }
                    };
                    let a2222 = 1u64;
                    nat_eq(a2221, a2222)
                } else {
                    false
                } {
                    let a2224 = match v2.clone() {
                        Adt2::C0(_, _, _, _, h2223, _, _, _, _, _, _) => {
                            h2223
                        }
                    };
                    let a2225 = 1u64;
                    nat_eq(a2224, a2225)
                } else {
                    false
                }
            } else {
                let a2227 = match v2.clone() {
                    Adt2::C0(_, h2226, _, _, _, _, _, _, _, _, _) => {
                        h2226
                    }
                };
                let a2228 = 1u64;
                if if if nat_eq(a2227, a2228) {
                    let a2231 = match v1.clone() {
                        Adt2::C0(_, _, h2229, _, _, _, _, _, _, _, _) => {
                            h2229
                        }
                    };
                    let a2232 = match v2.clone() {
                        Adt2::C0(_, _, h2230, _, _, _, _, _, _, _, _) => {
                            h2230
                        }
                    };
                    nat_eq(a2231, a2232)
                } else {
                    false
                } {
                    let a2235 = match v1.clone() {
                        Adt2::C0(_, _, _, h2233, _, _, _, _, _, _, _) => {
                            h2233
                        }
                    };
                    let a2236 = match v2.clone() {
                        Adt2::C0(_, _, _, h2234, _, _, _, _, _, _, _) => {
                            h2234
                        }
                    };
                    nat_eq(a2235, a2236)
                } else {
                    false
                } {
                    let a2239 = match v1.clone() {
                        Adt2::C0(_, _, _, _, h2237, _, _, _, _, _, _) => {
                            h2237
                        }
                    };
                    let a2240 = match v2.clone() {
                        Adt2::C0(_, _, _, _, h2238, _, _, _, _, _, _) => {
                            h2238
                        }
                    };
                    nat_eq(a2239, a2240)
                } else {
                    false
                }
            }
        } else {
            false
        } {
            let v4: Adt3 = f20(v1, 1u64);
            Ok(Adt10::C4(f144(v4.clone(), v0, v3)?, {
                let a2241 = v3;
                let a2242 = 0u64;
                if nat_eq(a2241, a2242) {
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

pub fn f71(v0: u64, v1: u64) -> R<Option<u64>> {
    let a2243 = v0;
    let a2244 = 89u64;
    if if nat_eq(a2243, a2244) {
        let a2245 = v1;
        let a2246 = 14u64;
        nat_eq(a2245, a2246)
    } else {
        false
    } {
        Ok(Some(23u64))
    } else {
        let a2247 = v0;
        let a2248 = 89u64;
        if if nat_eq(a2247, a2248) {
            let a2249 = v1;
            let a2250 = 15u64;
            nat_eq(a2249, a2250)
        } else {
            false
        } {
            Ok(Some(22u64))
        } else {
            let a2251 = v0;
            let a2252 = 89u64;
            if if nat_eq(a2251, a2252) {
                let a2253 = 8u64;
                let a2254 = v1;
                if nat_le(a2253, a2254) {
                    let a2255 = v1;
                    let a2256 = 9u64;
                    nat_le(a2255, a2256)
                } else {
                    false
                }
            } else {
                false
            } {
                Ok(Some({
                    let a2257 = v1;
                    let a2258 = 12u64;
                    nat_add(a2257, a2258)?
                }))
            } else {
                let a2259 = v0;
                let a2260 = 89u64;
                if if nat_eq(a2259, a2260) {
                    let a2261 = v1;
                    let a2262 = 4u64;
                    nat_eq(a2261, a2262)
                } else {
                    false
                } {
                    Ok(Some(19u64))
                } else {
                    let a2263 = 10u64;
                    let a2264 = v0;
                    if if nat_le(a2263, a2264) {
                        let a2265 = v0;
                        let a2266 = 12u64;
                        nat_le(a2265, a2266)
                    } else {
                        false
                    } {
                        Ok(Some({
                            let a2267 = v0;
                            let a2268 = 3u64;
                            nat_add(a2267, a2268)?
                        }))
                    } else {
                        let a2269 = v0;
                        let a2270 = 89u64;
                        if if nat_eq(a2269, a2270) {
                            let a2271 = v1;
                            let a2272 = 7u64;
                            nat_eq(a2271, a2272)
                        } else {
                            false
                        } {
                            Ok(Some(17u64))
                        } else {
                            let a2273 = v0;
                            let a2274 = 89u64;
                            if if nat_eq(a2273, a2274) {
                                let a2275 = v1;
                                let a2276 = 10u64;
                                nat_eq(a2275, a2276)
                            } else {
                                false
                            } {
                                Ok(Some(18u64))
                            } else {
                                let a2277 = v0;
                                let a2278 = 89u64;
                                if if nat_eq(a2277, a2278) {
                                    let a2279 = v1;
                                    let a2280 = 13u64;
                                    nat_eq(a2279, a2280)
                                } else {
                                    false
                                } {
                                    Ok(Some(16u64))
                                } else {
                                    let a2281 = v0;
                                    let a2282 = 8u64;
                                    if nat_eq(a2281, a2282) {
                                        Ok(Some(5u64))
                                    } else {
                                        let a2283 = v0;
                                        let a2284 = 9u64;
                                        if nat_eq(a2283, a2284) {
                                            Ok(Some(6u64))
                                        } else {
                                            let a2285 = v0;
                                            let a2286 = 89u64;
                                            if nat_eq(a2285, a2286) {
                                                let a2287 = v1;
                                                let a2288 = 0u64;
                                                if nat_eq(a2287, a2288) {
                                                    Ok(Some(0u64))
                                                } else {
                                                    let a2289 = v1;
                                                    let a2290 = 1u64;
                                                    if nat_eq(a2289, a2290) {
                                                        Ok(Some(1u64))
                                                    } else {
                                                        let a2291 = v1;
                                                        let a2292 = 2u64;
                                                        if nat_eq(a2291, a2292) {
                                                            Ok(Some(2u64))
                                                        } else {
                                                            let a2293 = v1;
                                                            let a2294 = 3u64;
                                                            if nat_eq(a2293, a2294) {
                                                                Ok(Some(3u64))
                                                            } else {
                                                                let a2295 = v1;
                                                                let a2296 = 6u64;
                                                                if nat_eq(a2295, a2296) {
                                                                    Ok(Some(4u64))
                                                                } else {
                                                                    let a2297 = v1;
                                                                    let a2298 = 12u64;
                                                                    if nat_eq(a2297, a2298) {
                                                                        Ok(Some(7u64))
                                                                    } else {
                                                                        let a2299 = v1;
                                                                        let a2300 = 11u64;
                                                                        if nat_eq(a2299, a2300) {
                                                                            Ok(Some(8u64))
                                                                        } else {
                                                                            let a2301 = v1;
                                                                            let a2302 = 18u64;
                                                                            if nat_eq(a2301, a2302) {
                                                                                Ok(Some(9u64))
                                                                            } else {
                                                                                let a2303 = v1;
                                                                                let a2304 = 19u64;
                                                                                if nat_eq(a2303, a2304) {
                                                                                    Ok(Some(10u64))
                                                                                } else {
                                                                                    let a2305 = v1;
                                                                                    let a2306 = 20u64;
                                                                                    if nat_eq(a2305, a2306) {
                                                                                        Ok(Some(11u64))
                                                                                    } else {
                                                                                        let a2307 = v1;
                                                                                        let a2308 = 21u64;
                                                                                        if nat_eq(a2307, a2308) {
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

pub fn f72(v0: u64, v1: u64, v2: Adt2, v3: Option<Adt2>, v4: Adt2) -> R<Adt10> {
    let m2309 = f145(v0, v1, v2.clone(), v3.clone(), v4.clone())?;
    match m2309 {
        Adt10::C0 => {
            Ok(Adt10::C0)
        }
        Adt10::C1 => {
            let a2310 = v0;
            let a2311 = 2u64;
            if if if if nat_eq(a2310, a2311) {
                true
            } else {
                let a2312 = v0;
                let a2313 = 5u64;
                nat_eq(a2312, a2313)
            } {
                true
            } else {
                let a2314 = v0;
                let a2315 = 6u64;
                nat_eq(a2314, a2315)
            } {
                true
            } else {
                let a2316 = v0;
                let a2317 = 7u64;
                nat_eq(a2316, a2317)
            } {
                Ok(Adt10::C5(67u64))
            } else {
                Ok(Adt10::C1)
            }
        }
        Adt10::C2(v7, v8, _, _, _, _, v13) => {
            if v8 {
                Ok(Adt10::C4(v13, 4u64, f20(v2.clone(), 1u64), f20(v4, 1u64), Adt4::C10(v7, v8, {
                    let m2318 = v3;
                    match m2318 {
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

pub fn f73(v0: u64, v1: Adt2, v2: Adt2) -> R<Adt10> {
    let a2320 = match v1.clone() {
        Adt2::C0(h2319, _, _, _, _, _, _, _, _, _, _) => {
            h2319
        }
    };
    let a2321 = 0u64;
    if if if if if nat_eq(a2320, a2321) {
        true
    } else {
        let a2323 = match v1.clone() {
            Adt2::C0(h2322, _, _, _, _, _, _, _, _, _, _) => {
                h2322
            }
        };
        let a2324 = 1u64;
        nat_eq(a2323, a2324)
    } {
        true
    } else {
        let a2326 = match v1.clone() {
            Adt2::C0(h2325, _, _, _, _, _, _, _, _, _, _) => {
                h2325
            }
        };
        let a2327 = 30u64;
        nat_eq(a2326, a2327)
    } {
        let a2330 = match v1.clone() {
            Adt2::C0(h2328, _, _, _, _, _, _, _, _, _, _) => {
                h2328
            }
        };
        let a2331 = match v2.clone() {
            Adt2::C0(h2329, _, _, _, _, _, _, _, _, _, _) => {
                h2329
            }
        };
        if nat_eq(a2330, a2331) {
            true
        } else {
            let a2333 = match v2.clone() {
                Adt2::C0(h2332, _, _, _, _, _, _, _, _, _, _) => {
                    h2332
                }
            };
            let a2334 = 0u64;
            nat_eq(a2333, a2334)
        }
    } else {
        false
    } {
        f25(v1.clone(), v2.clone())
    } else {
        false
    } {
        Ok(Adt10::C4({
            let a2336 = f16(v2.clone(), 1u64)?;
            let a2337 = f146(v0, match v2.clone() {
                Adt2::C0(h2335, _, _, _, _, _, _, _, _, _, _) => {
                    h2335
                }
            });
            nat_mul(a2336, a2337)?
        }, {
            let a2339 = match v2.clone() {
                Adt2::C0(h2338, _, _, _, _, _, _, _, _, _, _) => {
                    h2338
                }
            };
            let a2340 = 0u64;
            if nat_eq(a2339, a2340) {
                4u64
            } else {
                2u64
            }
        }, f20(v1.clone(), 1u64), f20(v2.clone(), 1u64), Adt4::C14(v0, match v1 {
            Adt2::C0(h2341, _, _, _, _, _, _, _, _, _, _) => {
                h2341
            }
        }, match v2 {
            Adt2::C0(h2342, _, _, _, _, _, _, _, _, _, _) => {
                h2342
            }
        })))
    } else {
        Ok(Adt10::C1)
    }
}

pub fn f74(v0: Adt3, v1: u64, v2: u64, v3: u64, v4: u64) -> R<u64> {
    f147(v1, v2, v3, v4, match v0.clone() {
        Adt3::C0(_, _, _, _, h2343, _, _, _) => {
            h2343
        }
    }, match v0.clone() {
        Adt3::C0(_, _, _, _, _, h2344, _, _) => {
            h2344
        }
    }, match v0.clone() {
        Adt3::C0(_, _, _, _, _, _, h2345, _) => {
            h2345
        }
    }, match v0 {
        Adt3::C0(_, _, _, _, _, _, _, h2346) => {
            h2346
        }
    })
}

pub fn f75(v0: u64, v1: Adt3, v2: u64) -> u64 {
    let v3: u64 = {
        let a2348 = v0;
        let a2349 = match v1.clone() {
            Adt3::C0(h2347, _, _, _, _, _, _, _) => {
                h2347
            }
        };
        let a2350 = 0u64;
        nat_quot(a2348, a2349, a2350)
    };
    let v4: u64 = {
        let a2352 = v3;
        let a2353 = match v1.clone() {
            Adt3::C0(_, h2351, _, _, _, _, _, _) => {
                h2351
            }
        };
        let a2354 = 0u64;
        nat_quot(a2352, a2353, a2354)
    };
    let a2355 = v2;
    let a2356 = 0u64;
    if nat_eq(a2355, a2356) {
        let a2358 = v0;
        let a2359 = match v1 {
            Adt3::C0(h2357, _, _, _, _, _, _, _) => {
                h2357
            }
        };
        let a2360 = 0u64;
        nat_rem(a2358, a2359, a2360)
    } else {
        let a2361 = v2;
        let a2362 = 1u64;
        if nat_eq(a2361, a2362) {
            let a2364 = v3;
            let a2365 = match v1 {
                Adt3::C0(_, h2363, _, _, _, _, _, _) => {
                    h2363
                }
            };
            let a2366 = 0u64;
            nat_rem(a2364, a2365, a2366)
        } else {
            let a2367 = v2;
            let a2368 = 2u64;
            if nat_eq(a2367, a2368) {
                let a2370 = v4;
                let a2371 = match v1 {
                    Adt3::C0(_, _, h2369, _, _, _, _, _) => {
                        h2369
                    }
                };
                let a2372 = 0u64;
                nat_rem(a2370, a2371, a2372)
            } else {
                let a2374 = v4;
                let a2375 = match v1 {
                    Adt3::C0(_, _, h2373, _, _, _, _, _) => {
                        h2373
                    }
                };
                let a2376 = 0u64;
                nat_quot(a2374, a2375, a2376)
            }
        }
    }
}

pub fn f76(v0: Adt8, v1: u64, v2: u64) -> R<u64> {
    let a2378 = match v0.clone() {
        Adt8::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, h2377) => {
            h2377
        }
    };
    let a2379 = 0u64;
    if nat_eq(a2378, a2379) {
        Ok(v2)
    } else {
        let a2384 = {
            let a2381 = v2;
            let a2382 = match v0.clone() {
                Adt8::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, h2380) => {
                    h2380
                }
            };
            nat_mul(a2381, a2382)?
        };
        let a2385 = f75(v1, match v0 {
            Adt8::C0(_, _, _, _, _, _, _, _, h2383, _, _, _, _, _, _) => {
                h2383
            }
        }, 3u64);
        nat_add(a2384, a2385)
    }
}

pub fn f77(v0: Adt6, v1: u64, v2: u64, v3: u64) -> R<Option<u64>> {
    let v4: u64 = {
        let a2395 = {
            let a2392 = {
                let a2387 = v2;
                let a2388 = match v0.clone() {
                    Adt6::C0(_, h2386, _, _) => {
                        h2386
                    }
                };
                nat_mul(a2387, a2388)?
            };
            let a2393 = {
                let a2390 = v3;
                let a2391 = match v0.clone() {
                    Adt6::C0(_, _, _, h2389) => {
                        h2389
                    }
                };
                nat_mul(a2390, a2391)?
            };
            nat_add(a2392, a2393)?
        };
        let a2396 = f148(match v0.clone() {
            Adt6::C0(_, _, h2394, _) => {
                h2394
            }
        });
        nat_add(a2395, a2396)?
    };
    let v5: u64 = f149(match v0 {
        Adt6::C0(_, _, h2397, _) => {
            h2397
        }
    });
    let a2398 = v5;
    let a2399 = v4;
    if if nat_le(a2398, a2399) {
        let a2402 = {
            let a2400 = v4;
            let a2401 = v5;
            nat_sub(a2400, a2401)
        };
        let a2403 = v1;
        nat_lt(a2402, a2403)
    } else {
        false
    } {
        Ok(Some({
            let a2404 = v4;
            let a2405 = v5;
            nat_sub(a2404, a2405)
        }))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f78(v0: Adt8, v1: u64, v2: u64, v3: u64, v4: u64) -> R<Adt9> {
    let v5: Adt3 = match v0.clone() {
        Adt8::C0(_, _, _, _, _, _, _, _, _, h2406, _, _, _, _, _) => {
            h2406
        }
    };
    let v6: Adt3 = match v0.clone() {
        Adt8::C0(_, _, _, _, _, _, _, _, h2407, _, _, _, _, _, _) => {
            h2407
        }
    };
    let v7: u64 = f75(v1, v6.clone(), 2u64);
    let a2409 = match v0.clone() {
        Adt8::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, h2408) => {
            h2408
        }
    };
    let a2410 = 0u64;
    if nat_eq(a2409, a2410) {
        Ok(Adt9::C8(Some(f74(v5, v3, v4, v7, v2)?)))
    } else {
        let v8: u64 = {
            let m2413 = match v0.clone() {
                Adt8::C0(_, _, _, _, _, _, _, _, _, _, h2412, _, _, _, _) => {
                    h2412
                }
            };
            match m2413 {
                Adt3::C0(_, _, h2411, _, _, _, _, _) => {
                    h2411
                }
            }
        };
        let m2419 = f77(match v0.clone() {
            Adt8::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, h2414, _) => {
                h2414
            }
        }, match v5.clone() {
            Adt3::C0(_, _, h2415, _, _, _, _, _) => {
                h2415
            }
        }, {
            let a2416 = v2;
            let a2417 = v8;
            let a2418 = 0u64;
            nat_rem(a2416, a2417, a2418)
        }, v7)?;
        match m2419 {
            None => {
                Ok(Adt9::C8(None::<u64>))
            }
            Some(v9) => {
                let v10: u64 = {
                    let a2426 = {
                        let a2424 = {
                            let a2420 = v2;
                            let a2421 = v8;
                            let a2422 = 0u64;
                            nat_quot(a2420, a2421, a2422)
                        };
                        let a2425 = match v0 {
                            Adt8::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, h2423) => {
                                h2423
                            }
                        };
                        nat_mul(a2424, a2425)?
                    };
                    let a2427 = f75(v1, v6, 3u64);
                    nat_add(a2426, a2427)?
                };
                Ok(Adt9::C8(Some(f74(v5, v3, v4, v9, v10)?)))
            }
        }
    }
}

pub fn f79(v0: Adt4, v1: Adt9) -> Adt9 {
    let m2428 = v1;
    match m2428 {
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
            f150(f151(v0.clone(), v20, v21), v20, v21, 0u64, v22, 2u64, f152(v0), 3u64, v23)
        }
        Adt9::C4(v24, v25, v26, v27, v28, v29, v30) => {
            let v31: u64 = f153(v0.clone(), v24);
            f150(f154(v0, v24), v31, v31, v25, v26, v27, v28, v29, v30)
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

pub fn f80(v0: u64, v1: Adt3, v2: Adt3, v3: Adt4) -> R<Adt9> {
    let m2429 = v3;
    match m2429 {
        Adt4::C0 => {
            Ok(Adt9::C1(false, f155(v0, v1)?, f155(v0, v2)?))
        }
        Adt4::C1 => {
            Ok(Adt9::C1(false, f157(v0, v1, v2.clone())?, f155(v0, v2)?))
        }
        Adt4::C2(v6, v7) => {
            f158(v0, v1, v7, v2, v6)
        }
        Adt4::C3(v8, v9, v10, v11, v12) => {
            f159(v0, v1, v2, v8, v9, v10, v11, v12)
        }
        Adt4::C4(v13) => {
            f160(v0, v1, v2, v13)
        }
        Adt4::C5(v14, v15, v16, v17) => {
            f159(v0, v1, v2, v14, v15, v16, v17, true)
        }
        Adt4::C6(v18, v19, v20) => {
            f161(v0, v1, v2, v18, v19, v20)
        }
        Adt4::C7(v21) => {
            f162(v0, v1, v2, v21)
        }
        Adt4::C8(v22) => {
            f163(v22, v0, v1, v2)
        }
        Adt4::C9(_, _) => {
            Ok(Adt9::C1(true, 0u64, f155(v0, v2)?))
        }
        Adt4::C10(v25, v26, v27) => {
            f164(v0, v1, v2, v25, v26, v27)
        }
        Adt4::C11(v40, v41) => {
            f165(v0, v1, v2, v40, v41)
        }
        Adt4::C12(v42, v43, v44) => {
            f166(v0, v1, v2, v42, v43, v44)
        }
        Adt4::C13(_, v46, _, _) => {
            Ok(Adt9::C3(v46, v46, f155(v0, v1)?, f155(v0, v2)?))
        }
        Adt4::C14(v49, v50, v51) => {
            let a2432 = v49;
            let a2433 = 23u64;
            if nat_eq(a2432, a2433) {
                f167(v0, v1, v2, v50, v51)
            } else {
                Ok(Adt9::C3(v50, v51, f155(v0, v1)?, f155(v0, v2)?))
            }
        }
        Adt4::C15 => {
            f168(v0, v1, v2)
        }
        Adt4::C16(v52, v53) => {
            f169(v0, v1, v2, v52, v53)
        }
        Adt4::C17 => {
            Ok(Adt9::C0(Adt10::C1))
        }
        Adt4::C18(v54, v55) => {
            f170(v0, v1, v55, v54)
        }
        Adt4::C19(_, _) => {
            Ok(Adt9::C0(Adt10::C1))
        }
        Adt4::C20(_, _) => {
            Ok(Adt9::C0(Adt10::C1))
        }
        Adt4::C21 => {
            Ok(Adt9::C1(false, f155(f156(v0, match v2.clone() {
                Adt3::C0(h2430, _, _, _, _, _, _, _) => {
                    h2430
                }
            }, match v2.clone() {
                Adt3::C0(_, h2431, _, _, _, _, _, _) => {
                    h2431
                }
            })?, v1)?, f155(v0, v2)?))
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
        Adt4::C25(v60, v61, v62) => {
            f171(v0, v60, v61, v62)
        }
        Adt4::C26(v68, _, _, _) => {
            let a2434 = v68;
            let a2435 = 64u64;
            if if if nat_eq(a2434, a2435) {
                true
            } else {
                let a2436 = v68;
                let a2437 = 65u64;
                nat_eq(a2436, a2437)
            } {
                true
            } else {
                let a2438 = v68;
                let a2439 = 66u64;
                nat_eq(a2438, a2439)
            } {
                Ok(Adt9::C0(Adt10::C5(64u64)))
            } else {
                Ok(Adt9::C0(Adt10::C1))
            }
        }
        Adt4::C27(v67) => {
            f94(v0, v1, v2, v67)
        }
        Adt4::C28(v63) => {
            f91(v0, v1, v2, v63, 0u64)
        }
        Adt4::C29(v64) => {
            f92(v0, v1, v2, v64)
        }
        Adt4::C30(v65, _) => {
            f93(v0, v1, v2, v65)
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
    }
}

pub fn f81(v0: u64, v1: Adt3, v2: Adt3, v3: u64, v4: u64) -> R<Adt9> {
    let v5: u64 = f132(v3, v4);
    let v6: u64 = {
        let a2440 = v0;
        let a2441 = v5;
        let a2442 = 0u64;
        nat_quot(a2440, a2441, a2442)
    };
    let a2443 = v5;
    let a2444 = 1u64;
    if nat_eq(a2443, a2444) {
        Ok(f150(f172(v3, v4), v3, v4, 0u64, f155(v6, v1)?, 2u64, 0u64, 3u64, f155(v6, v2)?))
    } else {
        let a2448 = {
            let a2445 = v0;
            let a2446 = 2u64;
            let a2447 = 0u64;
            nat_rem(a2445, a2446, a2447)
        };
        let a2449 = 0u64;
        if nat_eq(a2448, a2449) {
            Ok(f150(f172(v3, 0u64), v3, 0u64, 0u64, f155(v6, v1)?, 2u64, 0u64, 1u64, 0u64))
        } else {
            Ok(f150(f172(0u64, v4), 0u64, v4, 1u64, 0u64, 2u64, 0u64, 3u64, f155(v6, v2)?))
        }
    }
}

pub fn f82(v0: u64, v1: u64, v2: u64) -> R<Adt9> {
    let v3: u64 = f119(v1)?;
    let v4: u64 = {
        let a2450 = v0;
        let a2451 = v3;
        let a2452 = 0u64;
        nat_rem(a2450, a2451, a2452)
    };
    let v5: u64 = {
        let a2455 = {
            let a2453 = v1;
            let a2454 = 3u64;
            nat_mul(a2453, a2454)?
        };
        let a2456 = 1u64;
        nat_add(a2455, a2456)?
    };
    let a2457 = v4;
    let a2458 = 0u64;
    if nat_eq(a2457, a2458) {
        Ok(Adt9::C7(Adt11::C0(0u64), Adt12::C2(0u64), Adt12::C2(0u64), Adt13::C0(0u64), 8u64, 8u64))
    } else {
        let a2459 = v4;
        let a2460 = v5;
        if nat_eq(a2459, a2460) {
            Ok(Adt9::C7(Adt11::C0(2u64), Adt12::C1(0u64), Adt12::C2(0u64), Adt13::C0(0u64), 8u64, 8u64))
        } else {
            let v6: u64 = {
                let a2465 = v4;
                let a2466 = {
                    let a2461 = v4;
                    let a2462 = v5;
                    if nat_lt(a2461, a2462) {
                        1u64
                    } else {
                        let a2463 = v5;
                        let a2464 = 1u64;
                        nat_add(a2463, a2464)?
                    }
                };
                nat_sub(a2465, a2466)
            };
            let v7: u64 = {
                let a2467 = v6;
                let a2468 = 3u64;
                let a2469 = 0u64;
                nat_rem(a2467, a2468, a2469)
            };
            let v8: u64 = {
                let a2480 = 4u64;
                let a2481 = {
                    let a2478 = {
                        let a2473 = {
                            let a2470 = v0;
                            let a2471 = v3;
                            let a2472 = 0u64;
                            nat_quot(a2470, a2471, a2472)
                        };
                        let a2474 = v1;
                        nat_mul(a2473, a2474)?
                    };
                    let a2479 = {
                        let a2475 = v6;
                        let a2476 = 3u64;
                        let a2477 = 0u64;
                        nat_quot(a2475, a2476, a2477)
                    };
                    nat_add(a2478, a2479)?
                };
                nat_mul(a2480, a2481)?
            };
            let a2482 = v4;
            let a2483 = v5;
            if nat_lt(a2482, a2483) {
                let a2484 = v7;
                let a2485 = 0u64;
                if nat_eq(a2484, a2485) {
                    Ok(Adt9::C7(Adt11::C1(2u64), Adt12::C0(1u64, v8), Adt12::C0(0u64, v8), Adt13::C0(1u64), 4u64, 4u64))
                } else {
                    let a2486 = v7;
                    let a2487 = 1u64;
                    if nat_eq(a2486, a2487) {
                        Ok(Adt9::C7(Adt11::C0(1u64), Adt12::C1(1u64), Adt12::C2(0u64), Adt13::C0(1u64), 4u64, 4u64))
                    } else {
                        Ok(Adt9::C7(Adt11::C0(0u64), Adt12::C1(0u64), Adt12::C1(1u64), Adt13::C0(0u64), 8u64, 8u64))
                    }
                }
            } else {
                let a2488 = v7;
                let a2489 = 0u64;
                if nat_eq(a2488, a2489) {
                    Ok(Adt9::C7(Adt11::C1(1u64), Adt12::C0(0u64, v8), Adt12::C1(0u64), Adt13::C0(1u64), 4u64, 4u64))
                } else {
                    let a2490 = v7;
                    let a2491 = 1u64;
                    if nat_eq(a2490, a2491) {
                        Ok(Adt9::C7(Adt11::C1(2u64), Adt12::C2(v2), Adt12::C1(1u64), Adt13::C0(1u64), 4u64, 4u64))
                    } else {
                        Ok(Adt9::C7(Adt11::C1(2u64), Adt12::C1(1u64), Adt12::C0(1u64, v8), Adt13::C1(v8), 4u64, 4u64))
                    }
                }
            }
        }
    }
}

pub fn f83(v0: u64, v1: Adt3, v2: Adt3, v3: Adt3) -> R<Adt9> {
    let v4: u64 = {
        let a2492 = v0;
        let a2493 = 6u64;
        let a2494 = 0u64;
        nat_quot(a2492, a2493, a2494)
    };
    let v5: u64 = {
        let a2495 = v0;
        let a2496 = 6u64;
        let a2497 = 0u64;
        nat_rem(a2495, a2496, a2497)
    };
    let a2498 = v5;
    let a2499 = 0u64;
    if nat_eq(a2498, a2499) {
        Ok(Adt9::C7(Adt11::C7(17u64, 0u64, 0u64), Adt12::C0(1u64, f155(v4, v2)?), Adt12::C2(0u64), Adt13::C0(0u64), 4u64, 4u64))
    } else {
        let a2500 = v5;
        let a2501 = 1u64;
        if nat_eq(a2500, a2501) {
            Ok(Adt9::C7(Adt11::C1(1u64), Adt12::C2(1065353216u64), Adt12::C1(0u64), Adt13::C0(1u64), 4u64, 4u64))
        } else {
            let a2502 = v5;
            let a2503 = 2u64;
            if nat_eq(a2502, a2503) {
                Ok(Adt9::C7(Adt11::C1(2u64), Adt12::C0(1u64, f155(v4, v2)?), Adt12::C1(1u64), Adt13::C0(1u64), 4u64, 4u64))
            } else {
                let a2504 = v5;
                let a2505 = 3u64;
                if nat_eq(a2504, a2505) {
                    Ok(Adt9::C7(Adt11::C1(0u64), Adt12::C2(1065353216u64), Adt12::C1(1u64), Adt13::C0(1u64), 4u64, 4u64))
                } else {
                    let a2506 = v5;
                    let a2507 = 4u64;
                    if nat_eq(a2506, a2507) {
                        Ok(Adt9::C7(Adt11::C1(2u64), Adt12::C0(0u64, f155(v4, v1)?), Adt12::C1(0u64), Adt13::C0(0u64), 4u64, 4u64))
                    } else {
                        Ok(Adt9::C7(Adt11::C1(2u64), Adt12::C1(0u64), Adt12::C1(1u64), Adt13::C1(f155(v4, v3)?), 4u64, 4u64))
                    }
                }
            }
        }
    }
}

pub fn f84(v0: u64, v1: Adt3, v2: Adt3, v3: Adt3, v4: bool, v5: u64) -> R<Adt9> {
    let v6: u64 = if v4 {
        3u64
    } else {
        2u64
    };
    let v7: u64 = f116(match v1.clone() {
        Adt3::C0(_, h2508, _, _, _, _, _, _) => {
            h2508
        }
    }, v4)?;
    let v8: u64 = {
        let a2509 = v0;
        let a2510 = v7;
        let a2511 = 0u64;
        nat_rem(a2509, a2510, a2511)
    };
    let v9: u64 = {
        let a2512 = v0;
        let a2513 = v7;
        let a2514 = 0u64;
        nat_quot(a2512, a2513, a2514)
    };
    let a2517 = {
        let a2515 = v8;
        let a2516 = 1u64;
        nat_add(a2515, a2516)?
    };
    let a2518 = v7;
    if nat_eq(a2517, a2518) {
        Ok(Adt9::C7(Adt11::C1(2u64), Adt12::C2(v5), Adt12::C1(0u64), Adt13::C1(f155(v9, v3)?), 4u64, 4u64))
    } else {
        let v10: u64 = {
            let a2519 = v8;
            let a2520 = v6;
            let a2521 = 0u64;
            nat_quot(a2519, a2520, a2521)
        };
        let v11: u64 = {
            let a2522 = v8;
            let a2523 = v6;
            let a2524 = 0u64;
            nat_rem(a2522, a2523, a2524)
        };
        let v12: u64 = {
            let a2526 = v9;
            let a2527 = match v1.clone() {
                Adt3::C0(h2525, _, _, _, _, _, _, _) => {
                    h2525
                }
            };
            let a2528 = 0u64;
            nat_rem(a2526, a2527, a2528)
        };
        let v13: u64 = {
            let a2530 = v9;
            let a2531 = match v1.clone() {
                Adt3::C0(h2529, _, _, _, _, _, _, _) => {
                    h2529
                }
            };
            let a2532 = 0u64;
            nat_quot(a2530, a2531, a2532)
        };
        let v14: u64 = if v4 {
            let a2544 = {
                let a2539 = {
                    let a2534 = match v2.clone() {
                        Adt3::C0(_, _, _, _, h2533, _, _, _) => {
                            h2533
                        }
                    };
                    let a2535 = v12;
                    nat_mul(a2534, a2535)?
                };
                let a2540 = {
                    let a2537 = match v2.clone() {
                        Adt3::C0(_, _, _, _, _, h2536, _, _) => {
                            h2536
                        }
                    };
                    let a2538 = v10;
                    nat_mul(a2537, a2538)?
                };
                nat_add(a2539, a2540)?
            };
            let a2545 = {
                let a2542 = match v2 {
                    Adt3::C0(_, _, _, _, _, _, h2541, _) => {
                        h2541
                    }
                };
                let a2543 = v13;
                nat_mul(a2542, a2543)?
            };
            nat_add(a2544, a2545)?
        } else {
            let a2552 = {
                let a2547 = match v2.clone() {
                    Adt3::C0(_, _, _, _, h2546, _, _, _) => {
                        h2546
                    }
                };
                let a2548 = v10;
                nat_mul(a2547, a2548)?
            };
            let a2553 = {
                let a2550 = match v2 {
                    Adt3::C0(_, _, _, _, _, h2549, _, _) => {
                        h2549
                    }
                };
                let a2551 = v13;
                nat_mul(a2550, a2551)?
            };
            nat_add(a2552, a2553)?
        };
        if if v4 {
            let a2554 = v11;
            let a2555 = 0u64;
            nat_eq(a2554, a2555)
        } else {
            false
        } {
            Ok(Adt9::C7(Adt11::C7(17u64, 0u64, 0u64), Adt12::C0(1u64, v14), Adt12::C2(0u64), Adt13::C0(1u64), 4u64, 4u64))
        } else {
            let a2558 = {
                let a2556 = v11;
                let a2557 = 1u64;
                nat_add(a2556, a2557)?
            };
            let a2559 = v6;
            if nat_eq(a2558, a2559) {
                Ok(Adt9::C7(Adt11::C1(0u64), {
                    let a2560 = v10;
                    let a2561 = 0u64;
                    if nat_eq(a2560, a2561) {
                        Adt12::C2(0u64)
                    } else {
                        Adt12::C1(0u64)
                    }
                }, Adt12::C1(1u64), Adt13::C0(0u64), 4u64, 4u64))
            } else {
                Ok(Adt9::C7(Adt11::C1(2u64), Adt12::C0(0u64, {
                    let a2573 = {
                        let a2568 = {
                            let a2563 = match v1.clone() {
                                Adt3::C0(_, _, _, _, h2562, _, _, _) => {
                                    h2562
                                }
                            };
                            let a2564 = v12;
                            nat_mul(a2563, a2564)?
                        };
                        let a2569 = {
                            let a2566 = match v1.clone() {
                                Adt3::C0(_, _, _, _, _, h2565, _, _) => {
                                    h2565
                                }
                            };
                            let a2567 = v10;
                            nat_mul(a2566, a2567)?
                        };
                        nat_add(a2568, a2569)?
                    };
                    let a2574 = {
                        let a2571 = match v1 {
                            Adt3::C0(_, _, _, _, _, _, h2570, _) => {
                                h2570
                            }
                        };
                        let a2572 = v13;
                        nat_mul(a2571, a2572)?
                    };
                    nat_add(a2573, a2574)?
                }), if v4 {
                    Adt12::C1(1u64)
                } else {
                    Adt12::C0(1u64, v14)
                }, Adt13::C0(1u64), 4u64, 4u64))
            }
        }
    }
}

pub fn f85(v0: u64, v1: u64, v2: Adt3, v3: Adt3) -> R<Adt9> {
    let a2575 = v0;
    let a2576 = v1;
    if nat_lt(a2575, a2576) {
        Ok(Adt9::C7(Adt11::C9, Adt12::C0(0u64, f155(v0, v2)?), Adt12::C2(0u64), Adt13::C0(v0), 4u64, 4u64))
    } else {
        let v4: u64 = {
            let a2577 = v0;
            let a2578 = v1;
            nat_sub(a2577, a2578)
        };
        Ok(Adt9::C7(Adt11::C5(3u64), Adt12::C1(v4), Adt12::C2(0u64), Adt13::C1(f155(v4, v3)?), 4u64, 4u64))
    }
}

pub fn f86(v0: u64, v1: Adt3, v2: Adt3, v3: Adt3, v4: Adt6, v5: Adt6, v6: u64) -> R<Adt9> {
    let v7: u64 = {
        let a2585 = {
            let a2583 = 3u64;
            let a2584 = {
                let a2581 = match v4.clone() {
                    Adt6::C0(h2579, _, _, _) => {
                        h2579
                    }
                };
                let a2582 = match v5.clone() {
                    Adt6::C0(h2580, _, _, _) => {
                        h2580
                    }
                };
                nat_mul(a2581, a2582)?
            };
            nat_mul(a2583, a2584)?
        };
        let a2586 = 2u64;
        nat_add(a2585, a2586)?
    };
    let v8: u64 = {
        let a2587 = v0;
        let a2588 = v7;
        let a2589 = 0u64;
        nat_quot(a2587, a2588, a2589)
    };
    let v9: u64 = {
        let a2590 = v0;
        let a2591 = v7;
        let a2592 = 0u64;
        nat_rem(a2590, a2591, a2592)
    };
    let a2593 = v9;
    let a2594 = 0u64;
    if nat_eq(a2593, a2594) {
        Ok(Adt9::C7(Adt11::C2(0u64, 0u64, false), Adt12::C2(0u64), Adt12::C2(0u64), Adt13::C0(0u64), 4u64, 4u64))
    } else {
        let a2597 = {
            let a2595 = v9;
            let a2596 = 1u64;
            nat_add(a2595, a2596)?
        };
        let a2598 = v7;
        if nat_eq(a2597, a2598) {
            Ok(Adt9::C7(Adt11::C2(0u64, 0u64, false), Adt12::C1(0u64), Adt12::C2(0u64), Adt13::C1(f155(v8, v3)?), 4u64, 4u64))
        } else {
            let v10: u64 = {
                let a2599 = v9;
                let a2600 = 1u64;
                nat_sub(a2599, a2600)
            };
            let v11: u64 = {
                let a2601 = v10;
                let a2602 = 3u64;
                let a2603 = 0u64;
                nat_quot(a2601, a2602, a2603)
            };
            let v12: u64 = {
                let a2605 = v11;
                let a2606 = match v4.clone() {
                    Adt6::C0(h2604, _, _, _) => {
                        h2604
                    }
                };
                let a2607 = 0u64;
                nat_rem(a2605, a2606, a2607)
            };
            let v13: u64 = {
                let a2609 = v11;
                let a2610 = match v4.clone() {
                    Adt6::C0(h2608, _, _, _) => {
                        h2608
                    }
                };
                let a2611 = 0u64;
                nat_quot(a2609, a2610, a2611)
            };
            let v14: u64 = {
                let a2612 = v10;
                let a2613 = 3u64;
                let a2614 = 0u64;
                nat_rem(a2612, a2613, a2614)
            };
            let v15: u64 = f75(v8, v3.clone(), 2u64);
            let m2616 = f77(v4, match v2.clone() {
                Adt3::C0(h2615, _, _, _, _, _, _, _) => {
                    h2615
                }
            }, f75(v8, v3.clone(), 0u64), v12)?;
            match m2616 {
                None => {
                    Ok(Adt9::C7(Adt11::C2(0u64, 0u64, false), Adt12::C1(0u64), Adt12::C2(0u64), Adt13::C0(0u64), 4u64, 4u64))
                }
                Some(v16) => {
                    let m2618 = f77(v5, match v2.clone() {
                        Adt3::C0(_, h2617, _, _, _, _, _, _) => {
                            h2617
                        }
                    }, f75(v8, v3.clone(), 1u64), v13)?;
                    match m2618 {
                        None => {
                            Ok(Adt9::C7(Adt11::C2(0u64, 0u64, false), Adt12::C1(0u64), Adt12::C2(0u64), Adt13::C0(0u64), 4u64, 4u64))
                        }
                        Some(v17) => {
                            let a2619 = v14;
                            let a2620 = 0u64;
                            if nat_eq(a2619, a2620) {
                                Ok(Adt9::C7(Adt11::C2(v6, 0u64, false), Adt12::C0(0u64, f74(v1, v12, v13, 0u64, v15)?), Adt12::C2(0u64), Adt13::C0(1u64), f173(v6), 4u64))
                            } else {
                                let a2621 = v14;
                                let a2622 = 1u64;
                                if nat_eq(a2621, a2622) {
                                    Ok(Adt9::C7(Adt11::C1(2u64), Adt12::C1(1u64), Adt12::C0(1u64, f74(v2, v16, v17, v15, f75(v8, v3, 3u64))?), Adt13::C0(1u64), 4u64, 4u64))
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

pub fn f87(v0: u64, v1: Adt3, v2: Adt3, v3: u64) -> R<Adt9> {
    let v4: u64 = f142(match v1.clone() {
        Adt3::C0(h2623, _, _, _, _, _, _, _) => {
            h2623
        }
    })?;
    let v5: u64 = {
        let a2624 = v0;
        let a2625 = v4;
        let a2626 = 0u64;
        nat_quot(a2624, a2625, a2626)
    };
    let v6: u64 = {
        let a2627 = v0;
        let a2628 = v4;
        let a2629 = 0u64;
        nat_rem(a2627, a2628, a2629)
    };
    let a2630 = v6;
    let a2631 = 0u64;
    if nat_eq(a2630, a2631) {
        Ok(Adt9::C4(19u64, 2u64, 4286578688u64, 2u64, 0u64, 1u64, 0u64))
    } else {
        let a2632 = v6;
        let a2633 = 1u64;
        if nat_eq(a2632, a2633) {
            Ok(Adt9::C4(19u64, 2u64, 0u64, 2u64, 0u64, 1u64, 1u64))
        } else {
            let a2636 = {
                let a2634 = v6;
                let a2635 = 1u64;
                nat_add(a2634, a2635)?
            };
            let a2637 = v4;
            if nat_eq(a2636, a2637) {
                Ok(Adt9::C4(19u64, 1u64, 1u64, 2u64, 0u64, 3u64, f155(v5, v2)?))
            } else {
                let v7: u64 = {
                    let a2638 = v6;
                    let a2639 = 2u64;
                    nat_sub(a2638, a2639)
                };
                let v8: u64 = {
                    let a2640 = v7;
                    let a2641 = 3u64;
                    let a2642 = 0u64;
                    nat_quot(a2640, a2641, a2642)
                };
                let v9: u64 = {
                    let a2643 = v7;
                    let a2644 = 3u64;
                    let a2645 = 0u64;
                    nat_rem(a2643, a2644, a2645)
                };
                let v10: u64 = f155({
                    let a2649 = {
                        let a2647 = v5;
                        let a2648 = match v1.clone() {
                            Adt3::C0(h2646, _, _, _, _, _, _, _) => {
                                h2646
                            }
                        };
                        nat_mul(a2647, a2648)?
                    };
                    let a2650 = v8;
                    nat_add(a2649, a2650)?
                }, v1)?;
                let a2651 = v9;
                let a2652 = 0u64;
                if nat_eq(a2651, a2652) {
                    Ok(Adt9::C4(16u64, 1u64, 0u64, 0u64, v10, 1u64, 2u64))
                } else {
                    let a2653 = v9;
                    let a2654 = 1u64;
                    if nat_eq(a2653, a2654) {
                        let a2655 = v3;
                        let a2656 = 2u64;
                        if nat_eq(a2655, a2656) {
                            Ok(Adt9::C4(19u64, 1u64, 0u64, 2u64, 0u64, 1u64, 0u64))
                        } else {
                            Ok(Adt9::C4(19u64, 0u64, v10, 2u64, 0u64, 1u64, 0u64))
                        }
                    } else {
                        let a2657 = v3;
                        let a2658 = 2u64;
                        if if nat_eq(a2657, a2658) {
                            true
                        } else {
                            let a2659 = v3;
                            let a2660 = 3u64;
                            nat_eq(a2659, a2660)
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

pub fn f88(v0: u64, v1: u64, v2: Adt3, v3: Adt3, v4: Adt3, v5: u64) -> R<Adt9> {
    let v6: u64 = {
        let a2662 = 2u64;
        let a2663 = match v2.clone() {
            Adt3::C0(_, h2661, _, _, _, _, _, _) => {
                h2661
            }
        };
        nat_mul(a2662, a2663)?
    };
    let a2664 = v1;
    let a2665 = v6;
    if nat_lt(a2664, a2665) {
        let a2669 = {
            let a2666 = v1;
            let a2667 = 2u64;
            let a2668 = 0u64;
            nat_rem(a2666, a2667, a2668)
        };
        let a2670 = 0u64;
        if nat_eq(a2669, a2670) {
            Ok(Adt9::C4(19u64, 4u64, f155({
                let a2671 = v1;
                let a2672 = 2u64;
                let a2673 = 0u64;
                nat_quot(a2671, a2672, a2673)
            }, v4)?, 2u64, 0u64, 1u64, 2u64))
        } else {
            let a2674 = v5;
            let a2675 = 2147483648u64;
            if if nat_lt(a2674, a2675) {
                let a2677 = v5;
                let a2678 = match v3 {
                    Adt3::C0(_, h2676, _, _, _, _, _, _) => {
                        h2676
                    }
                };
                nat_lt(a2677, a2678)
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
            let a2681 = match v3.clone() {
                Adt3::C0(h2679, _, _, _, _, _, _, _) => {
                    h2679
                }
            };
            let a2682 = match v3.clone() {
                Adt3::C0(_, h2680, _, _, _, _, _, _) => {
                    h2680
                }
            };
            nat_mul(a2681, a2682)?
        };
        let a2685 = {
            let a2683 = v1;
            let a2684 = v6;
            nat_sub(a2683, a2684)
        };
        let a2686 = v7;
        if nat_lt(a2685, a2686) {
            Ok(Adt9::C4(19u64, 2u64, 0u64, 2u64, 0u64, 3u64, f155({
                let a2687 = v1;
                let a2688 = v6;
                nat_sub(a2687, a2688)
            }, v3)?))
        } else {
            let v8: u64 = {
                let a2691 = {
                    let a2689 = v1;
                    let a2690 = v6;
                    nat_sub(a2689, a2690)
                };
                let a2692 = v7;
                nat_sub(a2691, a2692)
            };
            let v9: u64 = {
                let a2694 = 1u64;
                let a2695 = f174(v0, match v2.clone() {
                    Adt3::C0(h2693, _, _, _, _, _, _, _) => {
                        h2693
                    }
                })?;
                nat_add(a2694, a2695)?
            };
            let v10: u64 = {
                let a2696 = v8;
                let a2697 = v9;
                let a2698 = 0u64;
                nat_quot(a2696, a2697, a2698)
            };
            let v11: u64 = {
                let a2699 = v8;
                let a2700 = v9;
                let a2701 = 0u64;
                nat_rem(a2699, a2700, a2701)
            };
            let a2702 = v11;
            let a2703 = 0u64;
            if nat_eq(a2702, a2703) {
                Ok(Adt9::C4(19u64, 4u64, f155(v10, v4)?, 2u64, 0u64, 1u64, 2u64))
            } else {
                if f120(v0) {
                    let a2707 = {
                        let a2704 = v11;
                        let a2705 = 2u64;
                        let a2706 = 0u64;
                        nat_rem(a2704, a2705, a2706)
                    };
                    let a2708 = 1u64;
                    if nat_eq(a2707, a2708) {
                        Ok(Adt9::C4(6u64, 0u64, f155({
                            let a2717 = {
                                let a2710 = v10;
                                let a2711 = match v2.clone() {
                                    Adt3::C0(h2709, _, _, _, _, _, _, _) => {
                                        h2709
                                    }
                                };
                                nat_mul(a2710, a2711)?
                            };
                            let a2718 = {
                                let a2714 = {
                                    let a2712 = v11;
                                    let a2713 = 1u64;
                                    nat_sub(a2712, a2713)
                                };
                                let a2715 = 2u64;
                                let a2716 = 0u64;
                                nat_quot(a2714, a2715, a2716)
                            };
                            nat_add(a2717, a2718)?
                        }, v2)?, 2u64, 0u64, 1u64, 0u64))
                    } else {
                        Ok(Adt9::C4(4u64, 3u64, f155({
                            let a2727 = {
                                let a2720 = v5;
                                let a2721 = match v2.clone() {
                                    Adt3::C0(h2719, _, _, _, _, _, _, _) => {
                                        h2719
                                    }
                                };
                                nat_mul(a2720, a2721)?
                            };
                            let a2728 = {
                                let a2724 = {
                                    let a2722 = v11;
                                    let a2723 = 1u64;
                                    nat_sub(a2722, a2723)
                                };
                                let a2725 = 2u64;
                                let a2726 = 0u64;
                                nat_quot(a2724, a2725, a2726)
                            };
                            nat_add(a2727, a2728)?
                        }, v3.clone())?, 1u64, 0u64, 3u64, f155({
                            let a2737 = {
                                let a2730 = v5;
                                let a2731 = match v2 {
                                    Adt3::C0(h2729, _, _, _, _, _, _, _) => {
                                        h2729
                                    }
                                };
                                nat_mul(a2730, a2731)?
                            };
                            let a2738 = {
                                let a2734 = {
                                    let a2732 = v11;
                                    let a2733 = 1u64;
                                    nat_sub(a2732, a2733)
                                };
                                let a2735 = 2u64;
                                let a2736 = 0u64;
                                nat_quot(a2734, a2735, a2736)
                            };
                            nat_add(a2737, a2738)?
                        }, v3)?))
                    }
                } else {
                    f175(v0, v10, {
                        let a2739 = v11;
                        let a2740 = 1u64;
                        nat_sub(a2739, a2740)
                    }, v5, v2, v3)
                }
            }
        }
    }
}

pub fn f89(v0: u64, v1: u64, v2: Adt3, v3: Adt3, v4: Adt3, v5: u64) -> R<Adt9> {
    let a2741 = v1;
    let a2742 = f176(v2.clone(), v4.clone())?;
    if nat_lt(a2741, a2742) {
        let a2745 = v1;
        let a2746 = {
            let a2743 = 2u64;
            let a2744 = f143(v4.clone())?;
            nat_mul(a2743, a2744)?
        };
        if nat_lt(a2745, a2746) {
            let a2750 = {
                let a2747 = v1;
                let a2748 = 2u64;
                let a2749 = 0u64;
                nat_rem(a2747, a2748, a2749)
            };
            let a2751 = 0u64;
            if nat_eq(a2750, a2751) {
                f177({
                    let a2752 = v1;
                    let a2753 = 2u64;
                    let a2754 = 0u64;
                    nat_quot(a2752, a2753, a2754)
                }, v4)
            } else {
                if f178(match v2 {
                    Adt3::C0(_, h2755, _, _, _, _, _, _) => {
                        h2755
                    }
                }, v5) {
                    Ok(Adt9::C4(19u64, 1u64, 2u64, 2u64, 0u64, 1u64, 2u64))
                } else {
                    Ok(Adt9::C0(Adt10::C1))
                }
            }
        } else {
            let v6: u64 = {
                let a2758 = v1;
                let a2759 = {
                    let a2756 = 2u64;
                    let a2757 = f143(v4.clone())?;
                    nat_mul(a2756, a2757)?
                };
                nat_sub(a2758, a2759)
            };
            let v7: u64 = {
                let a2763 = v6;
                let a2764 = {
                    let a2761 = 1u64;
                    let a2762 = match v2.clone() {
                        Adt3::C0(h2760, _, _, _, _, _, _, _) => {
                            h2760
                        }
                    };
                    nat_add(a2761, a2762)?
                };
                let a2765 = 0u64;
                nat_quot(a2763, a2764, a2765)
            };
            let v8: u64 = {
                let a2769 = v6;
                let a2770 = {
                    let a2767 = 1u64;
                    let a2768 = match v2.clone() {
                        Adt3::C0(h2766, _, _, _, _, _, _, _) => {
                            h2766
                        }
                    };
                    nat_add(a2767, a2768)?
                };
                let a2771 = 0u64;
                nat_rem(a2769, a2770, a2771)
            };
            let a2772 = v8;
            let a2773 = 0u64;
            if nat_eq(a2772, a2773) {
                f177(v7, v4)
            } else {
                f179(v0, v7, {
                    let a2774 = v8;
                    let a2775 = 1u64;
                    nat_sub(a2774, a2775)
                }, v2, v3, v4, v5)
            }
        }
    } else {
        Ok(Adt9::C0(Adt10::C1))
    }
}

pub fn f90(v0: u64, v1: Adt3, v2: Adt3, v3: Adt3, v4: Adt3, v5: u64) -> R<Adt9> {
    let v6: u64 = {
        let a2778 = match v4.clone() {
            Adt3::C0(h2776, _, _, _, _, _, _, _) => {
                h2776
            }
        };
        let a2779 = match v4.clone() {
            Adt3::C0(_, h2777, _, _, _, _, _, _) => {
                h2777
            }
        };
        nat_mul(a2778, a2779)?
    };
    let v7: u64 = {
        let a2780 = 2u64;
        let a2781 = v6;
        nat_mul(a2780, a2781)?
    };
    let v8: u64 = f180(match v3.clone() {
        Adt3::C0(_, h2782, _, _, _, _, _, _) => {
            h2782
        }
    }, match v3.clone() {
        Adt3::C0(_, _, h2783, _, _, _, _, _) => {
            h2783
        }
    }, match v3.clone() {
        Adt3::C0(_, _, _, h2784, _, _, _, _) => {
            h2784
        }
    });
    let a2785 = v0;
    let a2786 = v7;
    if nat_lt(a2785, a2786) {
        let a2790 = {
            let a2787 = v0;
            let a2788 = 2u64;
            let a2789 = 0u64;
            nat_rem(a2787, a2788, a2789)
        };
        let a2791 = 0u64;
        if nat_eq(a2790, a2791) {
            Ok(Adt9::C4(19u64, 5u64, f155({
                let a2792 = v0;
                let a2793 = 2u64;
                let a2794 = 0u64;
                nat_quot(a2792, a2793, a2794)
            }, v4)?, 2u64, 0u64, 1u64, 2u64))
        } else {
            if f181(v5, v8) {
                Ok(Adt9::C4(19u64, 1u64, 2u64, 2u64, 0u64, 1u64, 2u64))
            } else {
                Ok(Adt9::C0(Adt10::C1))
            }
        }
    } else {
        let v9: u64 = {
            let a2795 = v0;
            let a2796 = v7;
            nat_sub(a2795, a2796)
        };
        let v10: u64 = {
            let a2800 = v9;
            let a2801 = {
                let a2798 = 1u64;
                let a2799 = match v1.clone() {
                    Adt3::C0(h2797, _, _, _, _, _, _, _) => {
                        h2797
                    }
                };
                nat_add(a2798, a2799)?
            };
            let a2802 = 0u64;
            nat_quot(a2800, a2801, a2802)
        };
        let v11: u64 = {
            let a2806 = v9;
            let a2807 = {
                let a2804 = 1u64;
                let a2805 = match v1.clone() {
                    Adt3::C0(h2803, _, _, _, _, _, _, _) => {
                        h2803
                    }
                };
                nat_add(a2804, a2805)?
            };
            let a2808 = 0u64;
            nat_rem(a2806, a2807, a2808)
        };
        let a2809 = v11;
        let a2810 = 0u64;
        if nat_eq(a2809, a2810) {
            Ok(Adt9::C4(19u64, 5u64, f155({
                let a2811 = v10;
                let a2812 = v6;
                let a2813 = 0u64;
                nat_rem(a2811, a2812, a2813)
            }, v4)?, 2u64, 0u64, 1u64, 2u64))
        } else {
            if f181(v5, v8) {
                let v12: u64 = {
                    let a2819 = {
                        let a2815 = v10;
                        let a2816 = match v1.clone() {
                            Adt3::C0(h2814, _, _, _, _, _, _, _) => {
                                h2814
                            }
                        };
                        nat_mul(a2815, a2816)?
                    };
                    let a2820 = {
                        let a2817 = v11;
                        let a2818 = 1u64;
                        nat_sub(a2817, a2818)
                    };
                    nat_add(a2819, a2820)?
                };
                let a2821 = v5;
                let a2822 = 4294967295u64;
                if nat_eq(a2821, a2822) {
                    Ok(Adt9::C1(false, f155(v12, v1)?, f155(v12, v2)?))
                } else {
                    Ok(Adt9::C4(8u64, 0u64, f155(v12, v1)?, 4u64, f155({
                        let a2828 = {
                            let a2824 = v5;
                            let a2825 = match v3.clone() {
                                Adt3::C0(h2823, _, _, _, _, _, _, _) => {
                                    h2823
                                }
                            };
                            nat_mul(a2824, a2825)?
                        };
                        let a2829 = {
                            let a2826 = v11;
                            let a2827 = 1u64;
                            nat_sub(a2826, a2827)
                        };
                        nat_add(a2828, a2829)?
                    }, v3)?, 3u64, f155(v12, v2)?))
                }
            } else {
                Ok(Adt9::C0(Adt10::C1))
            }
        }
    }
}

pub fn f91(v0: u64, v1: Adt3, v2: Adt3, v3: Adt7, v4: u64) -> R<Adt9> {
    let v5: u64 = f125(v3.clone())?;
    let v6: u64 = {
        let a2830 = v0;
        let a2831 = v5;
        let a2832 = 0u64;
        nat_quot(a2830, a2831, a2832)
    };
    let v7: u64 = {
        let a2833 = v0;
        let a2834 = v5;
        let a2835 = 0u64;
        nat_rem(a2833, a2834, a2835)
    };
    let v8: u64 = f182(v3.clone())?;
    let a2836 = v7;
    let a2837 = 0u64;
    if nat_eq(a2836, a2837) {
        Ok(Adt9::C4(19u64, 2u64, if match v3 {
            Adt7::C0(_, _, _, h2838, _) => {
                h2838
            }
        } {
            4286578687u64
        } else {
            0u64
        }, 2u64, 0u64, 1u64, 0u64))
    } else {
        let a2842 = v5;
        let a2843 = {
            let a2840 = v7;
            let a2841 = if match v3.clone() {
                Adt7::C0(_, _, _, h2839, _) => {
                    h2839
                }
            } {
                1u64
            } else {
                2u64
            };
            nat_add(a2840, a2841)?
        };
        if nat_le(a2842, a2843) {
            if match v3.clone() {
                Adt7::C0(_, _, _, h2844, _) => {
                    h2844
                }
            } {
                Ok(Adt9::C4(19u64, 1u64, 0u64, 2u64, 0u64, 3u64, f155(v6, v2)?))
            } else {
                let a2847 = {
                    let a2845 = v7;
                    let a2846 = 2u64;
                    nat_add(a2845, a2846)?
                };
                let a2848 = v5;
                if nat_eq(a2847, a2848) {
                    let v9: u64 = if match v3.clone() {
                        Adt7::C0(_, _, h2849, _, _) => {
                            h2849
                        }
                    } {
                        let a2856 = {
                            let m2852 = match v3.clone() {
                                Adt7::C0(h2851, _, _, _, _) => {
                                    h2851
                                }
                            };
                            match m2852 {
                                Adt6::C0(h2850, _, _, _) => {
                                    h2850
                                }
                            }
                        };
                        let a2857 = {
                            let m2855 = match v3 {
                                Adt7::C0(_, h2854, _, _, _) => {
                                    h2854
                                }
                            };
                            match m2855 {
                                Adt6::C0(h2853, _, _, _) => {
                                    h2853
                                }
                            }
                        };
                        nat_mul(a2856, a2857)?
                    } else {
                        f183(match v3 {
                            Adt7::C0(h2858, _, _, _, _) => {
                                h2858
                            }
                        }, match v1 {
                            Adt3::C0(h2859, _, _, _, _, _, _, _) => {
                                h2859
                            }
                        }, f75(v6, v2, 0u64))?
                    };
                    Ok(Adt9::C4(3u64, 2u64, {
                        let a2860 = v9;
                        let a2861 = 0u64;
                        if nat_eq(a2860, a2861) {
                            1u64
                        } else {
                            v9
                        }
                    }, 2u64, 0u64, 1u64, 1u64))
                } else {
                    Ok(Adt9::C4(5u64, 1u64, 0u64, 1u64, 1u64, 3u64, f155(v6, v2)?))
                }
            }
        } else {
            let v10: u64 = {
                let a2864 = {
                    let a2862 = v7;
                    let a2863 = 1u64;
                    nat_sub(a2862, a2863)
                };
                let a2865 = v8;
                let a2866 = 0u64;
                nat_rem(a2864, a2865, a2866)
            };
            let m2872 = f184(v3.clone(), v1, v2, v6, {
                let a2869 = {
                    let a2867 = v7;
                    let a2868 = 1u64;
                    nat_sub(a2867, a2868)
                };
                let a2870 = v8;
                let a2871 = 0u64;
                nat_quot(a2869, a2870, a2871)
            })?;
            match m2872 {
                None => {
                    Ok(Adt9::C4(19u64, 1u64, 0u64, 2u64, 0u64, 1u64, 0u64))
                }
                Some(v11) => {
                    let a2874 = match v3.clone() {
                        Adt7::C0(_, _, _, _, h2873) => {
                            h2873
                        }
                    };
                    let a2875 = 1u64;
                    if if nat_eq(a2874, a2875) {
                        let a2876 = v10;
                        let a2877 = 0u64;
                        nat_eq(a2876, a2877)
                    } else {
                        false
                    } {
                        Ok(Adt9::C4(6u64, 0u64, v11, 2u64, 0u64, 1u64, 1u64))
                    } else {
                        if match v3.clone() {
                            Adt7::C0(_, _, _, h2878, _) => {
                                h2878
                            }
                        } {
                            let a2881 = {
                                let a2879 = v10;
                                let a2880 = 1u64;
                                nat_add(a2879, a2880)?
                            };
                            let a2882 = v8;
                            if nat_eq(a2881, a2882) {
                                let a2883 = v4;
                                let a2884 = 2u64;
                                if nat_eq(a2883, a2884) {
                                    Ok(Adt9::C4(19u64, 1u64, 0u64, 2u64, 0u64, 1u64, 0u64))
                                } else {
                                    Ok(Adt9::C4(19u64, {
                                        let a2886 = match v3.clone() {
                                            Adt7::C0(_, _, _, _, h2885) => {
                                                h2885
                                            }
                                        };
                                        let a2887 = 1u64;
                                        if nat_eq(a2886, a2887) {
                                            1u64
                                        } else {
                                            0u64
                                        }
                                    }, {
                                        let a2889 = match v3 {
                                            Adt7::C0(_, _, _, _, h2888) => {
                                                h2888
                                            }
                                        };
                                        let a2890 = 1u64;
                                        if nat_eq(a2889, a2890) {
                                            1u64
                                        } else {
                                            v11
                                        }
                                    }, 2u64, 0u64, 1u64, 0u64))
                                }
                            } else {
                                Ok(Adt9::C4(16u64, 1u64, 0u64, {
                                    let a2892 = match v3.clone() {
                                        Adt7::C0(_, _, _, _, h2891) => {
                                            h2891
                                        }
                                    };
                                    let a2893 = 1u64;
                                    if nat_eq(a2892, a2893) {
                                        1u64
                                    } else {
                                        0u64
                                    }
                                }, {
                                    let a2895 = match v3 {
                                        Adt7::C0(_, _, _, _, h2894) => {
                                            h2894
                                        }
                                    };
                                    let a2896 = 1u64;
                                    if nat_eq(a2895, a2896) {
                                        1u64
                                    } else {
                                        v11
                                    }
                                }, 1u64, 2u64))
                            }
                        } else {
                            Ok(Adt9::C4(4u64, 1u64, 0u64, {
                                let a2898 = match v3.clone() {
                                    Adt7::C0(_, _, _, _, h2897) => {
                                        h2897
                                    }
                                };
                                let a2899 = 1u64;
                                if nat_eq(a2898, a2899) {
                                    1u64
                                } else {
                                    0u64
                                }
                            }, {
                                let a2901 = match v3 {
                                    Adt7::C0(_, _, _, _, h2900) => {
                                        h2900
                                    }
                                };
                                let a2902 = 1u64;
                                if nat_eq(a2901, a2902) {
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

pub fn f92(v0: u64, v1: Adt3, v2: Adt3, v3: Adt3) -> R<Adt9> {
    let v4: u64 = f123(match v3.clone() {
        Adt3::C0(h2903, _, _, _, _, _, _, _) => {
            h2903
        }
    })?;
    let v5: u64 = {
        let a2904 = v0;
        let a2905 = v4;
        let a2906 = 0u64;
        nat_rem(a2904, a2905, a2906)
    };
    let v6: u64 = {
        let a2907 = v0;
        let a2908 = v4;
        let a2909 = 0u64;
        nat_quot(a2907, a2908, a2909)
    };
    let a2910 = v5;
    let a2911 = 0u64;
    if nat_eq(a2910, a2911) {
        Ok(Adt9::C4(19u64, 2u64, 0u64, 2u64, 0u64, 1u64, 0u64))
    } else {
        let a2914 = {
            let a2912 = v5;
            let a2913 = 1u64;
            nat_add(a2912, a2913)?
        };
        let a2915 = v4;
        if nat_eq(a2914, a2915) {
            Ok(Adt9::C4(19u64, 1u64, 0u64, 2u64, 0u64, 3u64, f155(v6, v2)?))
        } else {
            let a2919 = {
                let a2916 = v5;
                let a2917 = 2u64;
                let a2918 = 0u64;
                nat_rem(a2916, a2917, a2918)
            };
            let a2920 = 0u64;
            if nat_eq(a2919, a2920) {
                Ok(Adt9::C4(4u64, 1u64, 0u64, 1u64, 1u64, 1u64, 0u64))
            } else {
                let v7: u64 = {
                    let a2923 = {
                        let a2921 = v5;
                        let a2922 = 1u64;
                        nat_sub(a2921, a2922)
                    };
                    let a2924 = 2u64;
                    let a2925 = 0u64;
                    nat_quot(a2923, a2924, a2925)
                };
                let v8: u64 = f75(v6, v2.clone(), 0u64);
                let v9: u64 = f75(v6, v2.clone(), 1u64);
                let v10: u64 = f75(v6, v2, 2u64);
                Ok(Adt9::C4(10u64, 0u64, f74(v1, {
                    let a2926 = v9;
                    let a2927 = v7;
                    nat_add(a2926, a2927)?
                }, v8, v10, 0u64)?, 4u64, f74(v3, v7, v8, 0u64, 0u64)?, 1u64, 1u64))
            }
        }
    }
}

pub fn f93(v0: u64, v1: Adt3, v2: Adt3, v3: Adt6) -> R<Adt9> {
    let v4: u64 = f122(v3.clone())?;
    let v5: u64 = {
        let a2928 = v0;
        let a2929 = v4;
        let a2930 = 0u64;
        nat_rem(a2928, a2929, a2930)
    };
    let v6: u64 = {
        let a2931 = v0;
        let a2932 = v4;
        let a2933 = 0u64;
        nat_quot(a2931, a2932, a2933)
    };
    let a2934 = v5;
    let a2935 = 0u64;
    if nat_eq(a2934, a2935) {
        Ok(Adt9::C4(19u64, 2u64, 0u64, 2u64, 0u64, 1u64, 0u64))
    } else {
        let a2938 = {
            let a2936 = v5;
            let a2937 = 1u64;
            nat_add(a2936, a2937)?
        };
        let a2939 = v4;
        if nat_eq(a2938, a2939) {
            Ok(Adt9::C4(7u64, 1u64, 0u64, 2u64, 0u64, 3u64, f155(v6, v2)?))
        } else {
            let a2943 = {
                let a2940 = v5;
                let a2941 = 2u64;
                let a2942 = 0u64;
                nat_rem(a2940, a2941, a2942)
            };
            let a2944 = 0u64;
            if nat_eq(a2943, a2944) {
                Ok(Adt9::C4(4u64, 1u64, 0u64, 1u64, 1u64, 1u64, 0u64))
            } else {
                let v7: u64 = {
                    let a2946 = f75(v6, v2.clone(), 0u64);
                    let a2947 = match v3.clone() {
                        Adt6::C0(_, _, h2945, _) => {
                            h2945
                        }
                    };
                    nat_add(a2946, a2947)?
                };
                let v8: u64 = {
                    let a2955 = {
                        let a2952 = {
                            let a2949 = v7;
                            let a2950 = match v3.clone() {
                                Adt6::C0(_, h2948, _, _) => {
                                    h2948
                                }
                            };
                            nat_add(a2949, a2950)?
                        };
                        let a2953 = match v3.clone() {
                            Adt6::C0(h2951, _, _, _) => {
                                h2951
                            }
                        };
                        nat_sub(a2952, a2953)
                    };
                    let a2956 = match v3.clone() {
                        Adt6::C0(_, h2954, _, _) => {
                            h2954
                        }
                    };
                    let a2957 = 0u64;
                    nat_quot(a2955, a2956, a2957)
                };
                let v9: u64 = {
                    let a2963 = v8;
                    let a2964 = {
                        let a2960 = {
                            let a2958 = v5;
                            let a2959 = 1u64;
                            nat_sub(a2958, a2959)
                        };
                        let a2961 = 2u64;
                        let a2962 = 0u64;
                        nat_quot(a2960, a2961, a2962)
                    };
                    nat_add(a2963, a2964)?
                };
                let v10: u64 = {
                    let a2966 = v9;
                    let a2967 = match v3.clone() {
                        Adt6::C0(_, h2965, _, _) => {
                            h2965
                        }
                    };
                    nat_mul(a2966, a2967)?
                };
                let v11: u64 = {
                    let a2968 = v7;
                    let a2969 = v10;
                    nat_sub(a2968, a2969)
                };
                let a2971 = v9;
                let a2972 = match v1.clone() {
                    Adt3::C0(_, h2970, _, _, _, _, _, _) => {
                        h2970
                    }
                };
                if if if nat_lt(a2971, a2972) {
                    let a2973 = v10;
                    let a2974 = v7;
                    nat_le(a2973, a2974)
                } else {
                    false
                } {
                    let a2976 = v11;
                    let a2977 = match v3.clone() {
                        Adt6::C0(h2975, _, _, _) => {
                            h2975
                        }
                    };
                    nat_lt(a2976, a2977)
                } else {
                    false
                } {
                    Ok(Adt9::C4(6u64, 0u64, f74(v1, {
                        let a2981 = {
                            let a2979 = f75(v6, v2, 1u64);
                            let a2980 = match v3 {
                                Adt6::C0(h2978, _, _, _) => {
                                    h2978
                                }
                            };
                            nat_mul(a2979, a2980)?
                        };
                        let a2982 = v11;
                        nat_add(a2981, a2982)?
                    }, v9, 0u64, 0u64)?, 2u64, 0u64, 1u64, 1u64))
                } else {
                    Ok(Adt9::C4(19u64, 2u64, 0u64, 2u64, 0u64, 1u64, 1u64))
                }
            }
        }
    }
}

pub fn f94(v0: u64, v1: Adt3, v2: Adt3, v3: Adt5) -> R<Adt9> {
    let v4: u64 = f75(v0, v2.clone(), 0u64);
    let v5: u64 = f75(v0, v2.clone(), 3u64);
    let v6: u64 = {
        let a2984 = v5;
        let a2985 = match v3.clone() {
            Adt5::C0(_, _, _, _, _, _, h2983, _) => {
                h2983
            }
        };
        let a2986 = 0u64;
        nat_rem(a2984, a2985, a2986)
    };
    let v7: u64 = {
        let a2988 = v5;
        let a2989 = match v3.clone() {
            Adt5::C0(_, _, _, _, _, _, h2987, _) => {
                h2987
            }
        };
        let a2990 = 0u64;
        nat_quot(a2988, a2989, a2990)
    };
    let v8: Option<u64> = f77(match v3.clone() {
        Adt5::C0(h2991, _, _, _, _, _, _, _) => {
            h2991
        }
    }, match v1.clone() {
        Adt3::C0(h2992, _, _, _, _, _, _, _) => {
            h2992
        }
    }, f75(v0, v2.clone(), 1u64), {
        let a2996 = v4;
        let a2997 = {
            let m2995 = match v3.clone() {
                Adt5::C0(h2994, _, _, _, _, _, _, _) => {
                    h2994
                }
            };
            match m2995 {
                Adt6::C0(h2993, _, _, _) => {
                    h2993
                }
            }
        };
        let a2998 = 0u64;
        nat_rem(a2996, a2997, a2998)
    })?;
    let v9: u64 = {
        let a3002 = v4;
        let a3003 = {
            let m3001 = match v3.clone() {
                Adt5::C0(h3000, _, _, _, _, _, _, _) => {
                    h3000
                }
            };
            match m3001 {
                Adt6::C0(h2999, _, _, _) => {
                    h2999
                }
            }
        };
        let a3004 = 0u64;
        nat_quot(a3002, a3003, a3004)
    };
    let v10: Option<u64> = f77(match v3.clone() {
        Adt5::C0(_, h3005, _, _, _, _, _, _) => {
            h3005
        }
    }, match v1.clone() {
        Adt3::C0(_, h3006, _, _, _, _, _, _) => {
            h3006
        }
    }, f75(v0, v2.clone(), 2u64), {
        let a3010 = v9;
        let a3011 = {
            let m3009 = match v3.clone() {
                Adt5::C0(_, h3008, _, _, _, _, _, _) => {
                    h3008
                }
            };
            match m3009 {
                Adt6::C0(h3007, _, _, _) => {
                    h3007
                }
            }
        };
        let a3012 = 0u64;
        nat_rem(a3010, a3011, a3012)
    })?;
    let v11: u64 = {
        let a3016 = v9;
        let a3017 = {
            let m3015 = match v3.clone() {
                Adt5::C0(_, h3014, _, _, _, _, _, _) => {
                    h3014
                }
            };
            match m3015 {
                Adt6::C0(h3013, _, _, _) => {
                    h3013
                }
            }
        };
        let a3018 = 0u64;
        nat_quot(a3016, a3017, a3018)
    };
    let v12: Option<u64> = f77(match v3.clone() {
        Adt5::C0(_, _, h3019, _, _, _, _, _) => {
            h3019
        }
    }, match v1.clone() {
        Adt3::C0(_, _, h3020, _, _, _, _, _) => {
            h3020
        }
    }, v6, {
        let a3024 = v11;
        let a3025 = {
            let m3023 = match v3.clone() {
                Adt5::C0(_, _, h3022, _, _, _, _, _) => {
                    h3022
                }
            };
            match m3023 {
                Adt6::C0(h3021, _, _, _) => {
                    h3021
                }
            }
        };
        let a3026 = 0u64;
        nat_rem(a3024, a3025, a3026)
    })?;
    let v13: u64 = {
        let a3030 = v11;
        let a3031 = {
            let m3029 = match v3.clone() {
                Adt5::C0(_, _, h3028, _, _, _, _, _) => {
                    h3028
                }
            };
            match m3029 {
                Adt6::C0(h3027, _, _, _) => {
                    h3027
                }
            }
        };
        let a3032 = 0u64;
        nat_quot(a3030, a3031, a3032)
    };
    let v17: Adt12 = {
        let m3033 = v8;
        match m3033 {
            None => {
                Adt12::C2(0u64)
            }
            Some(v14) => {
                let m3034 = v10;
                match m3034 {
                    None => {
                        Adt12::C2(0u64)
                    }
                    Some(v15) => {
                        let m3035 = v12;
                        match m3035 {
                            None => {
                                Adt12::C2(0u64)
                            }
                            Some(v16) => {
                                Adt12::C0(1u64, {
                                    let a3044 = {
                                        let a3039 = f74(v1, v14, v15, v16, 0u64)?;
                                        let a3040 = {
                                            let a3037 = v7;
                                            let a3038 = match v3.clone() {
                                                Adt5::C0(_, _, _, _, h3036, _, _, _) => {
                                                    h3036
                                                }
                                            };
                                            nat_mul(a3037, a3038)?
                                        };
                                        nat_add(a3039, a3040)?
                                    };
                                    let a3045 = {
                                        let a3042 = v13;
                                        let a3043 = match v3.clone() {
                                            Adt5::C0(_, _, _, h3041, _, _, _, _) => {
                                                h3041
                                            }
                                        };
                                        nat_mul(a3042, a3043)?
                                    };
                                    nat_add(a3044, a3045)?
                                })
                            }
                        }
                    }
                }
            }
        }
    };
    Ok(Adt9::C7(Adt11::C2(match v3.clone() {
        Adt5::C0(_, _, _, _, _, h3046, _, _) => {
            h3046
        }
    }, match v3.clone() {
        Adt5::C0(_, _, _, _, _, _, _, h3047) => {
            h3047
        }
    }, true), v17, Adt12::C2(0u64), Adt13::C1(f155(v0, v2)?), {
        let a3049 = match v3.clone() {
            Adt5::C0(_, _, _, _, _, h3048, _, _) => {
                h3048
            }
        };
        let a3050 = 0u64;
        if nat_eq(a3049, a3050) {
            4u64
        } else {
            2u64
        }
    }, {
        let a3052 = match v3 {
            Adt5::C0(_, _, _, _, _, _, _, h3051) => {
                h3051
            }
        };
        let a3053 = 0u64;
        if nat_eq(a3052, a3053) {
            4u64
        } else {
            2u64
        }
    }))
}

pub fn f95(v0: Adt16, v1: u64, v2: Adt3, v3: Adt3, v4: u64, v5: u64, v6: u64, v7: u64, v8: u64, v9: u64) -> R<Adt9> {
    let m3066 = f185(v0, v1, match v2.clone() {
        Adt3::C0(h3054, _, _, _, _, _, _, _) => {
            h3054
        }
    }, match v2.clone() {
        Adt3::C0(_, h3055, _, _, _, _, _, _) => {
            h3055
        }
    }, match v2.clone() {
        Adt3::C0(_, _, h3056, _, _, _, _, _) => {
            h3056
        }
    }, match v2.clone() {
        Adt3::C0(_, _, _, h3057, _, _, _, _) => {
            h3057
        }
    }, match v2.clone() {
        Adt3::C0(_, _, _, _, h3058, _, _, _) => {
            h3058
        }
    }, match v2.clone() {
        Adt3::C0(_, _, _, _, _, h3059, _, _) => {
            h3059
        }
    }, match v2.clone() {
        Adt3::C0(_, _, _, _, _, _, h3060, _) => {
            h3060
        }
    }, match v2 {
        Adt3::C0(_, _, _, _, _, _, _, h3061) => {
            h3061
        }
    }, v6, v7, v8, v9, match v3.clone() {
        Adt3::C0(_, _, _, _, h3062, _, _, _) => {
            h3062
        }
    }, match v3.clone() {
        Adt3::C0(_, _, _, _, _, h3063, _, _) => {
            h3063
        }
    }, match v3.clone() {
        Adt3::C0(_, _, _, _, _, _, h3064, _) => {
            h3064
        }
    }, match v3 {
        Adt3::C0(_, _, _, _, _, _, _, h3065) => {
            h3065
        }
    }, v4)?;
    match m3066 {
        Adt19::C0 => {
            Ok(Adt9::C0(Adt10::C1))
        }
        Adt19::C1(v10, v11, v12, v13, v14, v15, v16, v17) => {
            Ok(f15(false, v10, v11, 0u64, 0u64, v12, v13, v14, v15, v16, v17, v5, v5))
        }
    }
}

pub fn f96(v0: u64, v1: Adt3, v2: Adt3, v3: u64, v4: u64, v5: u64, v6: u64, v7: u64) -> R<Adt9> {
    let v8: u64 = {
        let a3069 = v6;
        let a3070 = {
            let a3067 = 3u64;
            let a3068 = v5;
            nat_add(a3067, a3068)?
        };
        nat_sub(a3069, a3070)
    };
    let a3072 = v0;
    let a3073 = f36(v3, match v1.clone() {
        Adt3::C0(h3071, _, _, _, _, _, _, _) => {
            h3071
        }
    }, v5, v8)?;
    if nat_lt(a3072, a3073) {
        let v9: u64 = f110(match v1.clone() {
            Adt3::C0(h3074, _, _, _, _, _, _, _) => {
                h3074
            }
        }, v5, v8)?;
        let v10: u64 = f111(v3, match v1.clone() {
            Adt3::C0(h3075, _, _, _, _, _, _, _) => {
                h3075
            }
        }, v5)?;
        let a3076 = v0;
        let a3077 = v9;
        if nat_lt(a3076, a3077) {
            f186(v0, v2, f30(v3), {
                let a3078 = 3u64;
                let a3079 = v5;
                nat_add(a3078, a3079)?
            }, v7)
        } else {
            let a3082 = v0;
            let a3083 = {
                let a3080 = v9;
                let a3081 = v10;
                nat_add(a3080, a3081)?
            };
            if nat_lt(a3082, a3083) {
                let v11: u64 = {
                    let a3084 = v0;
                    let a3085 = v9;
                    nat_sub(a3084, a3085)
                };
                let v12: u64 = f187(v3, match v1.clone() {
                    Adt3::C0(h3086, _, _, _, _, _, _, _) => {
                        h3086
                    }
                })?;
                f188({
                    let a3087 = v11;
                    let a3088 = v12;
                    let a3089 = 0u64;
                    nat_rem(a3087, a3088, a3089)
                }, {
                    let a3090 = v11;
                    let a3091 = v12;
                    let a3092 = 0u64;
                    nat_quot(a3090, a3091, a3092)
                }, v1, v3, v4, v7)
            } else {
                f189({
                    let a3095 = v0;
                    let a3096 = {
                        let a3093 = v9;
                        let a3094 = v10;
                        nat_add(a3093, a3094)?
                    };
                    nat_sub(a3095, a3096)
                }, v1, v2, v3)
            }
        }
    } else {
        Ok(Adt9::C0(Adt10::C1))
    }
}

pub fn f97(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64) -> R<Adt9> {
    let a3097 = v0;
    let a3098 = f19(v1)?;
    if nat_lt(a3097, a3098) {
        let a3099 = v0;
        let a3100 = 4u64;
        if nat_lt(a3099, a3100) {
            let a3101 = v0;
            let a3102 = 0u64;
            if nat_eq(a3101, a3102) {
                Ok(Adt9::C4(16u64, 2u64, v2, 2u64, v3, 1u64, 2u64))
            } else {
                let a3103 = v0;
                let a3104 = 1u64;
                if nat_eq(a3103, a3104) {
                    let a3105 = v5;
                    let a3106 = 0u64;
                    if nat_eq(a3105, a3106) {
                        Ok(Adt9::C4(9u64, 2u64, v3, 2u64, v2, 1u64, 2u64))
                    } else {
                        Ok(Adt9::C0(Adt10::C1))
                    }
                } else {
                    let a3107 = v0;
                    let a3108 = 2u64;
                    if nat_eq(a3107, a3108) {
                        Ok(Adt9::C4(11u64, 1u64, 2u64, 2u64, v4, 1u64, 2u64))
                    } else {
                        Ok(Adt9::C4(20u64, 1u64, 2u64, 2u64, 10u64, 1u64, 2u64))
                    }
                }
            }
        } else {
            let a3109 = v0;
            let a3110 = 4u64;
            if if nat_eq(a3109, a3110) {
                let a3111 = f190(v5)?;
                let a3112 = v1;
                !nat_eq(a3111, a3112)
            } else {
                false
            } {
                Ok(Adt9::C0(Adt10::C1))
            } else {
                let v6: u64 = {
                    let a3115 = {
                        let a3113 = v0;
                        let a3114 = 4u64;
                        nat_sub(a3113, a3114)
                    };
                    let a3116 = 3u64;
                    let a3117 = 0u64;
                    nat_quot(a3115, a3116, a3117)
                };
                let v7: u64 = {
                    let a3120 = {
                        let a3118 = v0;
                        let a3119 = 4u64;
                        nat_sub(a3118, a3119)
                    };
                    let a3121 = 3u64;
                    let a3122 = 0u64;
                    nat_rem(a3120, a3121, a3122)
                };
                let a3123 = v7;
                let a3124 = 0u64;
                if nat_eq(a3123, a3124) {
                    Ok(Adt9::C4(3u64, 2u64, v6, 2u64, 0u64, 1u64, 0u64))
                } else {
                    let a3125 = v7;
                    let a3126 = 1u64;
                    if nat_eq(a3125, a3126) {
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

pub fn f98(v0: bool, v1: u64, v2: u64, v3: u64, v4: u64) -> bool {
    if if if if v0 {
        let a3127 = v1;
        let a3128 = 5u64;
        nat_eq(a3127, a3128)
    } else {
        false
    } {
        let a3129 = v2;
        let a3130 = 16u64;
        nat_eq(a3129, a3130)
    } else {
        false
    } {
        let a3131 = v3;
        let a3132 = 0u64;
        nat_eq(a3131, a3132)
    } else {
        false
    } {
        let a3133 = v4;
        let a3134 = 0u64;
        nat_eq(a3133, a3134)
    } else {
        false
    }
}

pub fn f99(v0: bool, v1: u64, v2: u64, v3: u64, v4: u64) -> Adt11 {
    let a3135 = v1;
    let a3136 = 0u64;
    if nat_eq(a3135, a3136) {
        if v0 {
            let a3137 = v3;
            let a3138 = 0u64;
            if if if nat_eq(a3137, a3138) {
                let a3139 = v4;
                let a3140 = 43u64;
                nat_eq(a3139, a3140)
            } else {
                false
            } {
                true
            } else {
                let a3141 = v3;
                let a3142 = 43u64;
                if nat_eq(a3141, a3142) {
                    let a3143 = v4;
                    let a3144 = 0u64;
                    nat_eq(a3143, a3144)
                } else {
                    false
                }
            } {
                f172(v3, v4)
            } else {
                Adt11::C2(v3, v4, false)
            }
        } else {
            Adt11::C0(v2)
        }
    } else {
        let a3145 = v1;
        let a3146 = 1u64;
        if nat_eq(a3145, a3146) {
            Adt11::C1(v2)
        } else {
            let a3147 = v1;
            let a3148 = 2u64;
            if nat_eq(a3147, a3148) {
                Adt11::C5(v2)
            } else {
                let a3149 = v1;
                let a3150 = 3u64;
                if nat_eq(a3149, a3150) {
                    Adt11::C3(v2, v2, false)
                } else {
                    let a3151 = v1;
                    let a3152 = 4u64;
                    if nat_eq(a3151, a3152) {
                        Adt11::C2(v3, v4, false)
                    } else {
                        if f98(v0, v1, v2, v3, v4) {
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

pub fn f100(v0: u64, v1: u64) -> Adt12 {
    let a3153 = v0;
    let a3154 = 0u64;
    if nat_eq(a3153, a3154) {
        Adt12::C0(0u64, v1)
    } else {
        let a3155 = v0;
        let a3156 = 1u64;
        if nat_eq(a3155, a3156) {
            Adt12::C1(v1)
        } else {
            let a3157 = v0;
            let a3158 = 2u64;
            if nat_eq(a3157, a3158) {
                Adt12::C2(v1)
            } else {
                let a3159 = v0;
                let a3160 = 3u64;
                if nat_eq(a3159, a3160) {
                    Adt12::C3(v1)
                } else {
                    let a3161 = v0;
                    let a3162 = 13u64;
                    if nat_lt(a3161, a3162) {
                        Adt12::C0({
                            let a3163 = v0;
                            let a3164 = 3u64;
                            nat_sub(a3163, a3164)
                        }, v1)
                    } else {
                        Adt12::C4
                    }
                }
            }
        }
    }
}

pub fn f101(v0: u64, v1: u64) -> Adt13 {
    let a3165 = v0;
    let a3166 = 1u64;
    if nat_eq(a3165, a3166) {
        Adt13::C0(v1)
    } else {
        let a3167 = v0;
        let a3168 = 3u64;
        if if nat_eq(a3167, a3168) {
            true
        } else {
            let a3169 = v0;
            let a3170 = 6u64;
            nat_eq(a3169, a3170)
        } {
            Adt13::C1(v1)
        } else {
            Adt13::C2
        }
    }
}

pub fn f102(v0: Adt6, v1: u64) -> R<u64> {
    let a3172 = 0u64;
    let a3173 = match v0.clone() {
        Adt6::C0(h3171, _, _, _) => {
            h3171
        }
    };
    if if if if if nat_lt(a3172, a3173) {
        let a3175 = 0u64;
        let a3176 = match v0.clone() {
            Adt6::C0(_, h3174, _, _) => {
                h3174
            }
        };
        nat_lt(a3175, a3176)
    } else {
        false
    } {
        let a3178 = 0u64;
        let a3179 = match v0.clone() {
            Adt6::C0(_, _, _, h3177) => {
                h3177
            }
        };
        nat_lt(a3178, a3179)
    } else {
        false
    } {
        let a3181 = match v0.clone() {
            Adt6::C0(_, h3180, _, _) => {
                h3180
            }
        };
        let a3182 = 2147483648u64;
        nat_lt(a3181, a3182)
    } else {
        false
    } {
        let a3184 = match v0.clone() {
            Adt6::C0(_, _, _, h3183) => {
                h3183
            }
        };
        let a3185 = 2147483648u64;
        nat_lt(a3184, a3185)
    } else {
        false
    } {
        let v2: u64 = {
            let a3189 = v1;
            let a3190 = {
                let a3187 = 2u64;
                let a3188 = f149(match v0.clone() {
                    Adt6::C0(_, _, h3186, _) => {
                        h3186
                    }
                });
                nat_mul(a3187, a3188)?
            };
            nat_add(a3189, a3190)?
        };
        let v3: u64 = {
            let a3202 = {
                let a3197 = 1u64;
                let a3198 = {
                    let a3195 = match v0.clone() {
                        Adt6::C0(_, _, _, h3191) => {
                            h3191
                        }
                    };
                    let a3196 = {
                        let a3193 = match v0.clone() {
                            Adt6::C0(h3192, _, _, _) => {
                                h3192
                            }
                        };
                        let a3194 = 1u64;
                        nat_sub(a3193, a3194)
                    };
                    nat_mul(a3195, a3196)?
                };
                nat_add(a3197, a3198)?
            };
            let a3203 = {
                let a3200 = 2u64;
                let a3201 = f148(match v0.clone() {
                    Adt6::C0(_, _, h3199, _) => {
                        h3199
                    }
                });
                nat_mul(a3200, a3201)?
            };
            nat_add(a3202, a3203)?
        };
        let a3204 = v3;
        let a3205 = v2;
        if nat_le(a3204, a3205) {
            let a3212 = 1u64;
            let a3213 = {
                let a3209 = {
                    let a3206 = v2;
                    let a3207 = v3;
                    nat_sub(a3206, a3207)
                };
                let a3210 = match v0 {
                    Adt6::C0(_, h3208, _, _) => {
                        h3208
                    }
                };
                let a3211 = 0u64;
                nat_quot(a3209, a3210, a3211)
            };
            nat_add(a3212, a3213)
        } else {
            let a3217 = {
                let a3214 = v3;
                let a3215 = v2;
                nat_sub(a3214, a3215)
            };
            let a3218 = match v0 {
                Adt6::C0(_, h3216, _, _) => {
                    h3216
                }
            };
            if nat_lt(a3217, a3218) {
                Ok(1u64)
            } else {
                Ok(0u64)
            }
        }
    } else {
        Ok(0u64)
    }
}

pub fn f103(v0: u64) -> u64 {
    let v1: u64 = {
        let a3219 = v0;
        let a3220 = 4294967296u64;
        let a3221 = 0u64;
        nat_rem(a3219, a3220, a3221)
    };
    let a3222 = v1;
    let a3223 = 2147483648u64;
    if nat_lt(a3222, a3223) {
        v1
    } else {
        0u64
    }
}

pub fn f104(v0: u64) -> R<u64> {
    let a3226 = 2u64;
    let a3227 = {
        let a3224 = 2u64;
        let a3225 = v0;
        nat_mul(a3224, a3225)?
    };
    nat_add(a3226, a3227)
}

pub fn f105(v0: u64, v1: Adt14, v2: Adt14) -> R<Adt15> {
    let a3229 = match v1.clone() {
        Adt14::C0(h3228, _, _, _, _, _, _, _, _, _, _) => {
            h3228
        }
    };
    let a3230 = 0u64;
    if if if if nat_eq(a3229, a3230) {
        let a3232 = match v2.clone() {
            Adt14::C0(h3231, _, _, _, _, _, _, _, _, _, _) => {
                h3231
            }
        };
        let a3233 = 0u64;
        nat_eq(a3232, a3233)
    } else {
        false
    } {
        f191(v1.clone(), v2.clone())
    } else {
        false
    } {
        if f192(match v1.clone() {
            Adt14::C0(_, h3234, _, _, _, _, _, _, _, _, _) => {
                h3234
            }
        }, match v1.clone() {
            Adt14::C0(_, _, h3235, _, _, _, _, _, _, _, _) => {
                h3235
            }
        }, match v1.clone() {
            Adt14::C0(_, _, _, h3236, _, _, _, _, _, _, _) => {
                h3236
            }
        }, match v1.clone() {
            Adt14::C0(_, _, _, _, h3237, _, _, _, _, _, _) => {
                h3237
            }
        }) {
            true
        } else {
            if if if if f31(v0) {
                f193(v1.clone(), v2.clone())
            } else {
                false
            } {
                f194(v1.clone())
            } else {
                false
            } {
                f194(v2.clone())
            } else {
                false
            } {
                if f195(v1.clone(), v2.clone())? {
                    true
                } else {
                    f196(v1.clone(), v2.clone())
                }
            } else {
                false
            }
        }
    } else {
        false
    } {
        let v3: u64 = if f192(match v1.clone() {
            Adt14::C0(_, h3238, _, _, _, _, _, _, _, _, _) => {
                h3238
            }
        }, match v1.clone() {
            Adt14::C0(_, _, h3239, _, _, _, _, _, _, _, _) => {
                h3239
            }
        }, match v1.clone() {
            Adt14::C0(_, _, _, h3240, _, _, _, _, _, _, _) => {
                h3240
            }
        }, match v1.clone() {
            Adt14::C0(_, _, _, _, h3241, _, _, _, _, _, _) => {
                h3241
            }
        }) {
            0u64
        } else {
            let a3247 = match v1.clone() {
                Adt14::C0(_, _, h3242, _, _, _, _, _, _, _, _) => {
                    h3242
                }
            };
            let a3248 = {
                let a3245 = match v1.clone() {
                    Adt14::C0(_, _, _, h3243, _, _, _, _, _, _, _) => {
                        h3243
                    }
                };
                let a3246 = match v1.clone() {
                    Adt14::C0(_, _, _, _, h3244, _, _, _, _, _, _) => {
                        h3244
                    }
                };
                nat_mul(a3245, a3246)?
            };
            nat_mul(a3247, a3248)?
        };
        let v4: u64 = {
            let a3249 = v3;
            let a3250 = 0u64;
            if if nat_eq(a3249, a3250) {
                true
            } else {
                f197(v2.clone())?
            } {
                0u64
            } else {
                let a3252 = match v2.clone() {
                    Adt14::C0(_, _, _, _, _, _, _, _, _, h3251, _) => {
                        h3251
                    }
                };
                let a3253 = 4u64;
                let a3254 = 0u64;
                nat_quot(a3252, a3253, a3254)
            }
        };
        Ok(Adt15::C1(f198(f199(match v1.clone() {
            Adt14::C0(_, _, _, _, _, h3255, _, _, _, _, _) => {
                h3255
            }
        }, match v2 {
            Adt14::C0(_, _, _, _, _, h3256, _, _, _, _, _) => {
                h3256
            }
        }), match v1 {
            Adt14::C0(_, h3257, _, _, _, _, _, _, _, _, _) => {
                h3257
            }
        }, v3, v4)?, 4u64, 64u64, v3, f200(v4)?, v0))
    } else {
        Ok(Adt15::C0)
    }
}

pub fn f106(v0: u64, v1: u64, v2: Adt14, v3: Adt14) -> R<Adt15> {
    let v4: Adt14 = f201(v3.clone());
    let a3259 = match v2.clone() {
        Adt14::C0(h3258, _, _, _, _, _, _, _, _, _, _) => {
            h3258
        }
    };
    let a3260 = 0u64;
    if if if if nat_eq(a3259, a3260) {
        let a3262 = match v3.clone() {
            Adt14::C0(h3261, _, _, _, _, _, _, _, _, _, _) => {
                h3261
            }
        };
        let a3263 = 0u64;
        nat_eq(a3262, a3263)
    } else {
        false
    } {
        f191(v2.clone(), v3)
    } else {
        false
    } {
        if f192(match v2.clone() {
            Adt14::C0(_, h3264, _, _, _, _, _, _, _, _, _) => {
                h3264
            }
        }, match v2.clone() {
            Adt14::C0(_, _, h3265, _, _, _, _, _, _, _, _) => {
                h3265
            }
        }, match v2.clone() {
            Adt14::C0(_, _, _, h3266, _, _, _, _, _, _, _) => {
                h3266
            }
        }, match v2.clone() {
            Adt14::C0(_, _, _, _, h3267, _, _, _, _, _, _) => {
                h3267
            }
        }) {
            true
        } else {
            let a3269 = match v2.clone() {
                Adt14::C0(_, _, _, _, _, h3268, _, _, _, _, _) => {
                    h3268
                }
            };
            let a3270 = 4u64;
            if if if if if nat_eq(a3269, a3270) {
                f202(match v2.clone() {
                    Adt14::C0(_, h3271, _, _, _, _, _, _, _, _, _) => {
                        h3271
                    }
                }, match v2.clone() {
                    Adt14::C0(_, _, h3272, _, _, _, _, _, _, _, _) => {
                        h3272
                    }
                }, match v2.clone() {
                    Adt14::C0(_, _, _, h3273, _, _, _, _, _, _, _) => {
                        h3273
                    }
                }, v0)?
            } else {
                false
            } {
                f194(v2.clone())
            } else {
                false
            } {
                f194(v4.clone())
            } else {
                false
            } {
                let a3275 = f203(v4.clone())?;
                let a3276 = match v4.clone() {
                    Adt14::C0(_, _, _, _, _, _, _, _, _, h3274, _) => {
                        h3274
                    }
                };
                nat_le(a3275, a3276)
            } else {
                false
            } {
                if f195(v2.clone(), v4.clone())? {
                    true
                } else {
                    f196(v2.clone(), v4.clone())
                }
            } else {
                false
            }
        }
    } else {
        false
    } {
        let v5: u64 = if if f192(match v2.clone() {
            Adt14::C0(_, h3277, _, _, _, _, _, _, _, _, _) => {
                h3277
            }
        }, match v2.clone() {
            Adt14::C0(_, _, h3278, _, _, _, _, _, _, _, _) => {
                h3278
            }
        }, match v2.clone() {
            Adt14::C0(_, _, _, h3279, _, _, _, _, _, _, _) => {
                h3279
            }
        }, match v2.clone() {
            Adt14::C0(_, _, _, _, h3280, _, _, _, _, _, _) => {
                h3280
            }
        }) {
            true
        } else {
            f197(v4.clone())?
        } {
            0u64
        } else {
            let a3282 = match v4 {
                Adt14::C0(_, _, _, _, _, _, _, _, _, h3281, _) => {
                    h3281
                }
            };
            let a3283 = 4u64;
            let a3284 = 0u64;
            nat_quot(a3282, a3283, a3284)
        };
        Ok(Adt15::C1(f204(match v2.clone() {
            Adt14::C0(_, h3285, _, _, _, _, _, _, _, _, _) => {
                h3285
            }
        }, match v2.clone() {
            Adt14::C0(_, _, h3286, _, _, _, _, _, _, _, _) => {
                h3286
            }
        }, match v2.clone() {
            Adt14::C0(_, _, _, h3287, _, _, _, _, _, _, _) => {
                h3287
            }
        }, match v2 {
            Adt14::C0(_, _, _, _, h3288, _, _, _, _, _, _) => {
                h3288
            }
        }, v0, v5)?, 4u64, 65u64, v0, f200(v5)?, v1))
    } else {
        Ok(Adt15::C0)
    }
}

pub fn f107(v0: u64, v1: bool, v2: Adt14, v3: Adt14, v4: Adt14) -> R<Adt15> {
    let v5: Adt14 = f201(v4.clone());
    if if if if if if if v1 {
        let a3290 = match v2.clone() {
            Adt14::C0(h3289, _, _, _, _, _, _, _, _, _, _) => {
                h3289
            }
        };
        let a3291 = 0u64;
        nat_eq(a3290, a3291)
    } else {
        false
    } {
        let a3293 = match v3.clone() {
            Adt14::C0(h3292, _, _, _, _, _, _, _, _, _, _) => {
                h3292
            }
        };
        let a3294 = 0u64;
        nat_eq(a3293, a3294)
    } else {
        false
    } {
        let a3296 = match v4.clone() {
            Adt14::C0(h3295, _, _, _, _, _, _, _, _, _, _) => {
                h3295
            }
        };
        let a3297 = 0u64;
        nat_eq(a3296, a3297)
    } else {
        false
    } {
        f191(v2.clone(), v3.clone())
    } else {
        false
    } {
        f191(v2.clone(), v4)
    } else {
        false
    } {
        if f192(match v2.clone() {
            Adt14::C0(_, h3298, _, _, _, _, _, _, _, _, _) => {
                h3298
            }
        }, match v2.clone() {
            Adt14::C0(_, _, h3299, _, _, _, _, _, _, _, _) => {
                h3299
            }
        }, match v2.clone() {
            Adt14::C0(_, _, _, h3300, _, _, _, _, _, _, _) => {
                h3300
            }
        }, match v2.clone() {
            Adt14::C0(_, _, _, _, h3301, _, _, _, _, _, _) => {
                h3301
            }
        }) {
            true
        } else {
            let a3303 = match v2.clone() {
                Adt14::C0(_, _, _, _, _, h3302, _, _, _, _, _) => {
                    h3302
                }
            };
            let a3304 = 4u64;
            if if if if if if if nat_eq(a3303, a3304) {
                let a3306 = match v3.clone() {
                    Adt14::C0(_, _, _, _, _, h3305, _, _, _, _, _) => {
                        h3305
                    }
                };
                let a3307 = 4u64;
                nat_eq(a3306, a3307)
            } else {
                false
            } {
                f194(v2.clone())
            } else {
                false
            } {
                f194(v3.clone())
            } else {
                false
            } {
                f194(v5.clone())
            } else {
                false
            } {
                let a3309 = f203(v5.clone())?;
                let a3310 = match v5.clone() {
                    Adt14::C0(_, _, _, _, _, _, _, _, _, h3308, _) => {
                        h3308
                    }
                };
                nat_le(a3309, a3310)
            } else {
                false
            } {
                if f195(v2.clone(), v5.clone())? {
                    true
                } else {
                    f196(v2.clone(), v5.clone())
                }
            } else {
                false
            } {
                if f195(v3.clone(), v5.clone())? {
                    true
                } else {
                    f196(v3, v5.clone())
                }
            } else {
                false
            }
        }
    } else {
        false
    } {
        let v6: u64 = if f192(match v2.clone() {
            Adt14::C0(_, h3311, _, _, _, _, _, _, _, _, _) => {
                h3311
            }
        }, match v2.clone() {
            Adt14::C0(_, _, h3312, _, _, _, _, _, _, _, _) => {
                h3312
            }
        }, match v2.clone() {
            Adt14::C0(_, _, _, h3313, _, _, _, _, _, _, _) => {
                h3313
            }
        }, match v2.clone() {
            Adt14::C0(_, _, _, _, h3314, _, _, _, _, _, _) => {
                h3314
            }
        }) {
            0u64
        } else {
            let a3320 = match v2.clone() {
                Adt14::C0(_, _, h3315, _, _, _, _, _, _, _, _) => {
                    h3315
                }
            };
            let a3321 = {
                let a3318 = match v2.clone() {
                    Adt14::C0(_, _, _, h3316, _, _, _, _, _, _, _) => {
                        h3316
                    }
                };
                let a3319 = match v2.clone() {
                    Adt14::C0(_, _, _, _, h3317, _, _, _, _, _, _) => {
                        h3317
                    }
                };
                nat_mul(a3318, a3319)?
            };
            nat_mul(a3320, a3321)?
        };
        let v7: u64 = {
            let a3322 = v6;
            let a3323 = 0u64;
            if if nat_eq(a3322, a3323) {
                true
            } else {
                f197(v5.clone())?
            } {
                0u64
            } else {
                let a3325 = match v5 {
                    Adt14::C0(_, _, _, _, _, _, _, _, _, h3324, _) => {
                        h3324
                    }
                };
                let a3326 = 4u64;
                let a3327 = 0u64;
                nat_quot(a3325, a3326, a3327)
            }
        };
        Ok(Adt15::C1(f205(match v2 {
            Adt14::C0(_, h3328, _, _, _, _, _, _, _, _, _) => {
                h3328
            }
        }, v6, v7)?, 4u64, 66u64, v6, f200(v7)?, v0))
    } else {
        Ok(Adt15::C0)
    }
}

pub fn f108(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64) -> bool {
    if if f206(v0, v3, v6) {
        f206(v1, v4, v6)
    } else {
        false
    } {
        f206(v2, v5, v6)
    } else {
        false
    }
}

pub fn f109(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64) -> R<bool> {
    if if if if if f207(v0, v1, v4, v2, v5, v3, v6)? {
        true
    } else {
        f207(v0, v1, v4, v3, v6, v2, v5)?
    } {
        true
    } else {
        f207(v0, v2, v5, v1, v4, v3, v6)?
    } {
        true
    } else {
        f207(v0, v2, v5, v3, v6, v1, v4)?
    } {
        true
    } else {
        f207(v0, v3, v6, v1, v4, v2, v5)?
    } {
        Ok(true)
    } else {
        f207(v0, v3, v6, v2, v5, v1, v4)
    }
}

pub fn f110(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let a3329 = v2;
    let a3330 = 0u64;
    if nat_eq(a3329, a3330) {
        Ok(0u64)
    } else {
        let a3333 = 2u64;
        let a3334 = {
            let a3331 = v0;
            let a3332 = v1;
            nat_mul(a3331, a3332)?
        };
        nat_mul(a3333, a3334)
    }
}

pub fn f111(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let a3335 = v2;
    let a3336 = f187(v0, v1)?;
    nat_mul(a3335, a3336)
}

pub fn f112(v0: u64) -> u64 {
    let a3337 = v0;
    let a3338 = 30u64;
    if nat_eq(a3337, a3338) {
        3u64
    } else {
        1u64
    }
}

pub fn f113(v0: u64) -> R<u64> {
    let a3341 = 5u64;
    let a3342 = {
        let a3339 = 4u64;
        let a3340 = v0;
        nat_mul(a3339, a3340)?
    };
    nat_add(a3341, a3342)
}

pub fn f114(v0: Adt2, v1: u64) -> R<bool> {
    let a3343 = v1;
    let a3344 = 0u64;
    if nat_eq(a3343, a3344) {
        Ok(true)
    } else {
        let a3350 = {
            let a3347 = f155({
                let a3345 = v1;
                let a3346 = 1u64;
                nat_sub(a3345, a3346)
            }, f20(v0.clone(), 1u64))?;
            let a3348 = 4u64;
            nat_add(a3347, a3348)?
        };
        let a3351 = match v0 {
            Adt2::C0(_, _, _, _, _, _, _, _, _, h3349, _) => {
                h3349
            }
        };
        Ok(nat_le(a3350, a3351))
    }
}

pub fn f115(v0: Adt2) -> R<bool> {
    let v1: u64 = {
        let a3357 = match v0.clone() {
            Adt2::C0(_, _, h3352, _, _, _, _, _, _, _, _) => {
                h3352
            }
        };
        let a3358 = {
            let a3355 = match v0.clone() {
                Adt2::C0(_, _, _, h3353, _, _, _, _, _, _, _) => {
                    h3353
                }
            };
            let a3356 = match v0.clone() {
                Adt2::C0(_, _, _, _, h3354, _, _, _, _, _, _) => {
                    h3354
                }
            };
            nat_mul(a3355, a3356)?
        };
        nat_mul(a3357, a3358)?
    };
    let v2: u64 = f16(v0.clone(), 1u64)?;
    let a3360 = match v0.clone() {
        Adt2::C0(h3359, _, _, _, _, _, _, _, _, _, _) => {
            h3359
        }
    };
    let a3361 = 0u64;
    if if if if if if if nat_eq(a3360, a3361) {
        let a3363 = match v0.clone() {
            Adt2::C0(_, h3362, _, _, _, _, _, _, _, _, _) => {
                h3362
            }
        };
        let a3364 = 2147483647u64;
        nat_le(a3363, a3364)
    } else {
        false
    } {
        let a3365 = v1;
        let a3366 = 2147483647u64;
        nat_le(a3365, a3366)
    } else {
        false
    } {
        let a3368 = match v0.clone() {
            Adt2::C0(_, h3367, _, _, _, _, _, _, _, _, _) => {
                h3367
            }
        };
        let a3369 = 1u64;
        if nat_eq(a3368, a3369) {
            true
        } else {
            let a3371 = match v0.clone() {
                Adt2::C0(_, _, _, _, _, h3370, _, _, _, _, _) => {
                    h3370
                }
            };
            let a3372 = 4u64;
            nat_eq(a3371, a3372)
        }
    } else {
        false
    } {
        let a3373 = v1;
        let a3374 = 1u64;
        if nat_le(a3373, a3374) {
            true
        } else {
            let a3379 = {
                let a3376 = match v0.clone() {
                    Adt2::C0(_, h3375, _, _, _, _, _, _, _, _, _) => {
                        h3375
                    }
                };
                let a3377 = 4u64;
                nat_mul(a3376, a3377)?
            };
            let a3380 = match v0.clone() {
                Adt2::C0(_, _, _, _, _, _, h3378, _, _, _, _) => {
                    h3378
                }
            };
            nat_le(a3379, a3380)
        }
    } else {
        false
    } {
        let a3382 = match v0.clone() {
            Adt2::C0(_, _, _, h3381, _, _, _, _, _, _, _) => {
                h3381
            }
        };
        let a3383 = 1u64;
        if nat_eq(a3382, a3383) {
            true
        } else {
            let a3389 = match v0.clone() {
                Adt2::C0(_, _, _, _, _, _, _, h3384, _, _, _) => {
                    h3384
                }
            };
            let a3390 = {
                let a3387 = match v0.clone() {
                    Adt2::C0(_, _, h3385, _, _, _, _, _, _, _, _) => {
                        h3385
                    }
                };
                let a3388 = match v0.clone() {
                    Adt2::C0(_, _, _, _, _, _, h3386, _, _, _, _) => {
                        h3386
                    }
                };
                nat_mul(a3387, a3388)?
            };
            nat_eq(a3389, a3390)
        }
    } else {
        false
    } {
        let a3392 = match v0.clone() {
            Adt2::C0(_, _, _, _, h3391, _, _, _, _, _, _) => {
                h3391
            }
        };
        let a3393 = 1u64;
        if nat_eq(a3392, a3393) {
            true
        } else {
            let a3402 = match v0.clone() {
                Adt2::C0(_, _, _, _, _, _, _, _, h3394, _, _) => {
                    h3394
                }
            };
            let a3403 = {
                let a3400 = match v0.clone() {
                    Adt2::C0(_, _, _, h3395, _, _, _, _, _, _, _) => {
                        h3395
                    }
                };
                let a3401 = {
                    let a3398 = match v0.clone() {
                        Adt2::C0(_, _, h3396, _, _, _, _, _, _, _, _) => {
                            h3396
                        }
                    };
                    let a3399 = match v0.clone() {
                        Adt2::C0(_, _, _, _, _, _, h3397, _, _, _, _) => {
                            h3397
                        }
                    };
                    nat_mul(a3398, a3399)?
                };
                nat_mul(a3400, a3401)?
            };
            nat_eq(a3402, a3403)
        }
    } else {
        false
    } {
        let a3404 = v2;
        let a3405 = 0u64;
        if nat_eq(a3404, a3405) {
            Ok(true)
        } else {
            let a3411 = {
                let a3408 = f155({
                    let a3406 = v2;
                    let a3407 = 1u64;
                    nat_sub(a3406, a3407)
                }, f20(v0.clone(), 1u64))?;
                let a3409 = 4u64;
                nat_add(a3408, a3409)?
            };
            let a3412 = match v0 {
                Adt2::C0(_, _, _, _, _, _, _, _, _, h3410, _) => {
                    h3410
                }
            };
            Ok(nat_le(a3411, a3412))
        }
    } else {
        Ok(false)
    }
}

pub fn f116(v0: u64, v1: bool) -> R<u64> {
    let a3415 = {
        let a3413 = v0;
        let a3414 = if v1 {
            3u64
        } else {
            2u64
        };
        nat_mul(a3413, a3414)?
    };
    let a3416 = 1u64;
    nat_add(a3415, a3416)
}

pub fn f117(v0: Adt2, v1: u64, v2: bool, v3: bool) -> R<bool> {
    if v2 {
        if v3 {
            let a3418 = match v0.clone() {
                Adt2::C0(_, _, _, _, h3417, _, _, _, _, _, _) => {
                    h3417
                }
            };
            let a3419 = 1u64;
            if if if if if nat_le(a3418, a3419) {
                true
            } else {
                let a3421 = match v0.clone() {
                    Adt2::C0(_, _, _, _, _, _, _, _, h3420, _, _) => {
                        h3420
                    }
                };
                let a3422 = v1;
                nat_eq(a3421, a3422)
            } {
                let a3424 = match v0.clone() {
                    Adt2::C0(_, h3423, _, _, _, _, _, _, _, _, _) => {
                        h3423
                    }
                };
                let a3425 = 1u64;
                if nat_le(a3424, a3425) {
                    true
                } else {
                    let a3430 = match v0.clone() {
                        Adt2::C0(_, _, _, _, _, h3426, _, _, _, _, _) => {
                            h3426
                        }
                    };
                    let a3431 = {
                        let a3428 = v1;
                        let a3429 = match v0.clone() {
                            Adt2::C0(_, _, _, _, h3427, _, _, _, _, _, _) => {
                                h3427
                            }
                        };
                        nat_mul(a3428, a3429)?
                    };
                    nat_eq(a3430, a3431)
                }
            } else {
                false
            } {
                let a3433 = match v0.clone() {
                    Adt2::C0(_, _, h3432, _, _, _, _, _, _, _, _) => {
                        h3432
                    }
                };
                let a3434 = 1u64;
                if nat_le(a3433, a3434) {
                    true
                } else {
                    let a3442 = match v0.clone() {
                        Adt2::C0(_, _, _, _, _, _, h3435, _, _, _, _) => {
                            h3435
                        }
                    };
                    let a3443 = {
                        let a3440 = {
                            let a3437 = v1;
                            let a3438 = match v0.clone() {
                                Adt2::C0(_, _, _, _, h3436, _, _, _, _, _, _) => {
                                    h3436
                                }
                            };
                            nat_mul(a3437, a3438)?
                        };
                        let a3441 = match v0.clone() {
                            Adt2::C0(_, h3439, _, _, _, _, _, _, _, _, _) => {
                                h3439
                            }
                        };
                        nat_mul(a3440, a3441)?
                    };
                    nat_eq(a3442, a3443)
                }
            } else {
                false
            } {
                let a3445 = match v0.clone() {
                    Adt2::C0(_, _, _, h3444, _, _, _, _, _, _, _) => {
                        h3444
                    }
                };
                let a3446 = 1u64;
                if nat_le(a3445, a3446) {
                    true
                } else {
                    let a3457 = match v0.clone() {
                        Adt2::C0(_, _, _, _, _, _, _, h3447, _, _, _) => {
                            h3447
                        }
                    };
                    let a3458 = {
                        let a3455 = {
                            let a3452 = {
                                let a3449 = v1;
                                let a3450 = match v0.clone() {
                                    Adt2::C0(_, _, _, _, h3448, _, _, _, _, _, _) => {
                                        h3448
                                    }
                                };
                                nat_mul(a3449, a3450)?
                            };
                            let a3453 = match v0.clone() {
                                Adt2::C0(_, h3451, _, _, _, _, _, _, _, _, _) => {
                                    h3451
                                }
                            };
                            nat_mul(a3452, a3453)?
                        };
                        let a3456 = match v0.clone() {
                            Adt2::C0(_, _, h3454, _, _, _, _, _, _, _, _) => {
                                h3454
                            }
                        };
                        nat_mul(a3455, a3456)?
                    };
                    nat_eq(a3457, a3458)
                }
            } else {
                false
            } {
                let a3472 = {
                    let a3469 = {
                        let a3466 = {
                            let a3463 = {
                                let a3460 = v1;
                                let a3461 = match v0.clone() {
                                    Adt2::C0(_, _, _, _, h3459, _, _, _, _, _, _) => {
                                        h3459
                                    }
                                };
                                nat_mul(a3460, a3461)?
                            };
                            let a3464 = match v0.clone() {
                                Adt2::C0(_, h3462, _, _, _, _, _, _, _, _, _) => {
                                    h3462
                                }
                            };
                            nat_mul(a3463, a3464)?
                        };
                        let a3467 = match v0.clone() {
                            Adt2::C0(_, _, h3465, _, _, _, _, _, _, _, _) => {
                                h3465
                            }
                        };
                        nat_mul(a3466, a3467)?
                    };
                    let a3470 = match v0.clone() {
                        Adt2::C0(_, _, _, h3468, _, _, _, _, _, _, _) => {
                            h3468
                        }
                    };
                    nat_mul(a3469, a3470)?
                };
                let a3473 = match v0 {
                    Adt2::C0(_, _, _, _, _, _, _, _, _, h3471, _) => {
                        h3471
                    }
                };
                Ok(nat_le(a3472, a3473))
            } else {
                Ok(false)
            }
        } else {
            let a3475 = match v0.clone() {
                Adt2::C0(_, _, _, h3474, _, _, _, _, _, _, _) => {
                    h3474
                }
            };
            let a3476 = 1u64;
            if if if if if nat_le(a3475, a3476) {
                true
            } else {
                let a3478 = match v0.clone() {
                    Adt2::C0(_, _, _, _, _, _, _, h3477, _, _, _) => {
                        h3477
                    }
                };
                let a3479 = v1;
                nat_eq(a3478, a3479)
            } {
                let a3481 = match v0.clone() {
                    Adt2::C0(_, h3480, _, _, _, _, _, _, _, _, _) => {
                        h3480
                    }
                };
                let a3482 = 1u64;
                if nat_le(a3481, a3482) {
                    true
                } else {
                    let a3487 = match v0.clone() {
                        Adt2::C0(_, _, _, _, _, h3483, _, _, _, _, _) => {
                            h3483
                        }
                    };
                    let a3488 = {
                        let a3485 = v1;
                        let a3486 = match v0.clone() {
                            Adt2::C0(_, _, _, h3484, _, _, _, _, _, _, _) => {
                                h3484
                            }
                        };
                        nat_mul(a3485, a3486)?
                    };
                    nat_eq(a3487, a3488)
                }
            } else {
                false
            } {
                let a3490 = match v0.clone() {
                    Adt2::C0(_, _, h3489, _, _, _, _, _, _, _, _) => {
                        h3489
                    }
                };
                let a3491 = 1u64;
                if nat_le(a3490, a3491) {
                    true
                } else {
                    let a3499 = match v0.clone() {
                        Adt2::C0(_, _, _, _, _, _, h3492, _, _, _, _) => {
                            h3492
                        }
                    };
                    let a3500 = {
                        let a3497 = {
                            let a3494 = v1;
                            let a3495 = match v0.clone() {
                                Adt2::C0(_, _, _, h3493, _, _, _, _, _, _, _) => {
                                    h3493
                                }
                            };
                            nat_mul(a3494, a3495)?
                        };
                        let a3498 = match v0.clone() {
                            Adt2::C0(_, h3496, _, _, _, _, _, _, _, _, _) => {
                                h3496
                            }
                        };
                        nat_mul(a3497, a3498)?
                    };
                    nat_eq(a3499, a3500)
                }
            } else {
                false
            } {
                let a3502 = match v0.clone() {
                    Adt2::C0(_, _, _, _, h3501, _, _, _, _, _, _) => {
                        h3501
                    }
                };
                let a3503 = 1u64;
                if nat_le(a3502, a3503) {
                    true
                } else {
                    let a3514 = match v0.clone() {
                        Adt2::C0(_, _, _, _, _, _, _, _, h3504, _, _) => {
                            h3504
                        }
                    };
                    let a3515 = {
                        let a3512 = {
                            let a3509 = {
                                let a3506 = v1;
                                let a3507 = match v0.clone() {
                                    Adt2::C0(_, _, _, h3505, _, _, _, _, _, _, _) => {
                                        h3505
                                    }
                                };
                                nat_mul(a3506, a3507)?
                            };
                            let a3510 = match v0.clone() {
                                Adt2::C0(_, h3508, _, _, _, _, _, _, _, _, _) => {
                                    h3508
                                }
                            };
                            nat_mul(a3509, a3510)?
                        };
                        let a3513 = match v0.clone() {
                            Adt2::C0(_, _, h3511, _, _, _, _, _, _, _, _) => {
                                h3511
                            }
                        };
                        nat_mul(a3512, a3513)?
                    };
                    nat_eq(a3514, a3515)
                }
            } else {
                false
            } {
                let a3529 = {
                    let a3526 = {
                        let a3523 = {
                            let a3520 = {
                                let a3517 = v1;
                                let a3518 = match v0.clone() {
                                    Adt2::C0(_, _, _, h3516, _, _, _, _, _, _, _) => {
                                        h3516
                                    }
                                };
                                nat_mul(a3517, a3518)?
                            };
                            let a3521 = match v0.clone() {
                                Adt2::C0(_, h3519, _, _, _, _, _, _, _, _, _) => {
                                    h3519
                                }
                            };
                            nat_mul(a3520, a3521)?
                        };
                        let a3524 = match v0.clone() {
                            Adt2::C0(_, _, h3522, _, _, _, _, _, _, _, _) => {
                                h3522
                            }
                        };
                        nat_mul(a3523, a3524)?
                    };
                    let a3527 = match v0.clone() {
                        Adt2::C0(_, _, _, _, h3525, _, _, _, _, _, _) => {
                            h3525
                        }
                    };
                    nat_mul(a3526, a3527)?
                };
                let a3530 = match v0 {
                    Adt2::C0(_, _, _, _, _, _, _, _, _, h3528, _) => {
                        h3528
                    }
                };
                Ok(nat_le(a3529, a3530))
            } else {
                Ok(false)
            }
        }
    } else {
        let a3532 = match v0.clone() {
            Adt2::C0(_, h3531, _, _, _, _, _, _, _, _, _) => {
                h3531
            }
        };
        let a3533 = 1u64;
        if if if if if nat_le(a3532, a3533) {
            true
        } else {
            let a3535 = match v0.clone() {
                Adt2::C0(_, _, _, _, _, h3534, _, _, _, _, _) => {
                    h3534
                }
            };
            let a3536 = v1;
            nat_eq(a3535, a3536)
        } {
            let a3538 = match v0.clone() {
                Adt2::C0(_, _, h3537, _, _, _, _, _, _, _, _) => {
                    h3537
                }
            };
            let a3539 = 1u64;
            if nat_le(a3538, a3539) {
                true
            } else {
                let a3544 = match v0.clone() {
                    Adt2::C0(_, _, _, _, _, _, h3540, _, _, _, _) => {
                        h3540
                    }
                };
                let a3545 = {
                    let a3542 = v1;
                    let a3543 = match v0.clone() {
                        Adt2::C0(_, h3541, _, _, _, _, _, _, _, _, _) => {
                            h3541
                        }
                    };
                    nat_mul(a3542, a3543)?
                };
                nat_eq(a3544, a3545)
            }
        } else {
            false
        } {
            let a3547 = match v0.clone() {
                Adt2::C0(_, _, _, h3546, _, _, _, _, _, _, _) => {
                    h3546
                }
            };
            let a3548 = 1u64;
            if nat_le(a3547, a3548) {
                true
            } else {
                let a3556 = match v0.clone() {
                    Adt2::C0(_, _, _, _, _, _, _, h3549, _, _, _) => {
                        h3549
                    }
                };
                let a3557 = {
                    let a3554 = {
                        let a3551 = v1;
                        let a3552 = match v0.clone() {
                            Adt2::C0(_, h3550, _, _, _, _, _, _, _, _, _) => {
                                h3550
                            }
                        };
                        nat_mul(a3551, a3552)?
                    };
                    let a3555 = match v0.clone() {
                        Adt2::C0(_, _, h3553, _, _, _, _, _, _, _, _) => {
                            h3553
                        }
                    };
                    nat_mul(a3554, a3555)?
                };
                nat_eq(a3556, a3557)
            }
        } else {
            false
        } {
            let a3559 = match v0.clone() {
                Adt2::C0(_, _, _, _, h3558, _, _, _, _, _, _) => {
                    h3558
                }
            };
            let a3560 = 1u64;
            if nat_le(a3559, a3560) {
                true
            } else {
                let a3571 = match v0.clone() {
                    Adt2::C0(_, _, _, _, _, _, _, _, h3561, _, _) => {
                        h3561
                    }
                };
                let a3572 = {
                    let a3569 = {
                        let a3566 = {
                            let a3563 = v1;
                            let a3564 = match v0.clone() {
                                Adt2::C0(_, h3562, _, _, _, _, _, _, _, _, _) => {
                                    h3562
                                }
                            };
                            nat_mul(a3563, a3564)?
                        };
                        let a3567 = match v0.clone() {
                            Adt2::C0(_, _, h3565, _, _, _, _, _, _, _, _) => {
                                h3565
                            }
                        };
                        nat_mul(a3566, a3567)?
                    };
                    let a3570 = match v0.clone() {
                        Adt2::C0(_, _, _, h3568, _, _, _, _, _, _, _) => {
                            h3568
                        }
                    };
                    nat_mul(a3569, a3570)?
                };
                nat_eq(a3571, a3572)
            }
        } else {
            false
        } {
            let a3586 = {
                let a3583 = {
                    let a3580 = {
                        let a3577 = {
                            let a3574 = v1;
                            let a3575 = match v0.clone() {
                                Adt2::C0(_, h3573, _, _, _, _, _, _, _, _, _) => {
                                    h3573
                                }
                            };
                            nat_mul(a3574, a3575)?
                        };
                        let a3578 = match v0.clone() {
                            Adt2::C0(_, _, h3576, _, _, _, _, _, _, _, _) => {
                                h3576
                            }
                        };
                        nat_mul(a3577, a3578)?
                    };
                    let a3581 = match v0.clone() {
                        Adt2::C0(_, _, _, h3579, _, _, _, _, _, _, _) => {
                            h3579
                        }
                    };
                    nat_mul(a3580, a3581)?
                };
                let a3584 = match v0.clone() {
                    Adt2::C0(_, _, _, _, h3582, _, _, _, _, _, _) => {
                        h3582
                    }
                };
                nat_mul(a3583, a3584)?
            };
            let a3587 = match v0 {
                Adt2::C0(_, _, _, _, _, _, _, _, _, h3585, _) => {
                    h3585
                }
            };
            Ok(nat_le(a3586, a3587))
        } else {
            Ok(false)
        }
    }
}

pub fn f118(v0: Adt2, v1: u64) -> R<bool> {
    let a3589 = match v0.clone() {
        Adt2::C0(h3588, _, _, _, _, _, _, _, _, _, _) => {
            h3588
        }
    };
    let a3590 = 0u64;
    if if nat_eq(a3589, a3590) {
        f22(v0.clone())?
    } else {
        false
    } {
        let a3591 = v1;
        let a3592 = 1u64;
        if nat_le(a3591, a3592) {
            Ok(true)
        } else {
            let a3597 = match v0.clone() {
                Adt2::C0(_, _, _, _, _, _, h3593, _, _, _, _) => {
                    h3593
                }
            };
            let a3598 = {
                let a3595 = 4u64;
                let a3596 = match v0 {
                    Adt2::C0(_, h3594, _, _, _, _, _, _, _, _, _) => {
                        h3594
                    }
                };
                nat_mul(a3595, a3596)?
            };
            Ok(nat_eq(a3597, a3598))
        }
    } else {
        Ok(false)
    }
}

pub fn f119(v0: u64) -> R<u64> {
    let a3601 = 2u64;
    let a3602 = {
        let a3599 = 6u64;
        let a3600 = v0;
        nat_mul(a3599, a3600)?
    };
    nat_add(a3601, a3602)
}

pub fn f120(v0: u64) -> bool {
    let a3603 = v0;
    let a3604 = 0u64;
    if if nat_eq(a3603, a3604) {
        true
    } else {
        let a3605 = v0;
        let a3606 = 1u64;
        nat_eq(a3605, a3606)
    } {
        true
    } else {
        let a3607 = v0;
        let a3608 = 30u64;
        nat_eq(a3607, a3608)
    }
}

pub fn f121(v0: Adt2, v1: u64) -> R<bool> {
    let a3610 = match v0.clone() {
        Adt2::C0(_, _, _, h3609, _, _, _, _, _, _, _) => {
            h3609
        }
    };
    let a3611 = 1u64;
    if if if nat_eq(a3610, a3611) {
        let a3613 = match v0.clone() {
            Adt2::C0(_, _, _, _, h3612, _, _, _, _, _, _) => {
                h3612
            }
        };
        let a3614 = 1u64;
        nat_eq(a3613, a3614)
    } else {
        false
    } {
        let a3616 = match v0.clone() {
            Adt2::C0(_, _, _, _, _, h3615, _, _, _, _, _) => {
                h3615
            }
        };
        let a3617 = v1;
        nat_eq(a3616, a3617)
    } else {
        false
    } {
        let a3619 = match v0.clone() {
            Adt2::C0(_, _, h3618, _, _, _, _, _, _, _, _) => {
                h3618
            }
        };
        let a3620 = 1u64;
        if nat_eq(a3619, a3620) {
            Ok(true)
        } else {
            let a3625 = match v0.clone() {
                Adt2::C0(_, _, _, _, _, _, h3621, _, _, _, _) => {
                    h3621
                }
            };
            let a3626 = {
                let a3623 = match v0 {
                    Adt2::C0(_, h3622, _, _, _, _, _, _, _, _, _) => {
                        h3622
                    }
                };
                let a3624 = v1;
                nat_mul(a3623, a3624)?
            };
            Ok(nat_eq(a3625, a3626))
        }
    } else {
        Ok(false)
    }
}

pub fn f122(v0: Adt6) -> R<u64> {
    let a3639 = 2u64;
    let a3640 = {
        let a3637 = 2u64;
        let a3638 = {
            let a3634 = {
                let a3631 = match v0.clone() {
                    Adt6::C0(h3627, _, _, _) => {
                        h3627
                    }
                };
                let a3632 = {
                    let a3629 = match v0.clone() {
                        Adt6::C0(_, h3628, _, _) => {
                            h3628
                        }
                    };
                    let a3630 = 1u64;
                    nat_sub(a3629, a3630)
                };
                nat_add(a3631, a3632)?
            };
            let a3635 = match v0 {
                Adt6::C0(_, h3633, _, _) => {
                    h3633
                }
            };
            let a3636 = 0u64;
            nat_quot(a3634, a3635, a3636)
        };
        nat_mul(a3637, a3638)?
    };
    nat_add(a3639, a3640)
}

pub fn f123(v0: u64) -> R<u64> {
    let a3643 = 2u64;
    let a3644 = {
        let a3641 = v0;
        let a3642 = 2u64;
        nat_mul(a3641, a3642)?
    };
    nat_add(a3643, a3644)
}

pub fn f124(v0: Adt6, v1: u64, v2: u64) -> R<bool> {
    let v3: u64 = {
        let a3648 = {
            let a3645 = v2;
            let a3646 = 1u64;
            nat_sub(a3645, a3646)
        };
        let a3649 = match v0.clone() {
            Adt6::C0(_, h3647, _, _) => {
                h3647
            }
        };
        nat_mul(a3648, a3649)?
    };
    let a3651 = 0u64;
    let a3652 = match v0.clone() {
        Adt6::C0(h3650, _, _, _) => {
            h3650
        }
    };
    if if if if if if if if nat_lt(a3651, a3652) {
        let a3654 = match v0.clone() {
            Adt6::C0(h3653, _, _, _) => {
                h3653
            }
        };
        let a3655 = 2147483648u64;
        nat_lt(a3654, a3655)
    } else {
        false
    } {
        let a3657 = 0u64;
        let a3658 = match v0.clone() {
            Adt6::C0(_, h3656, _, _) => {
                h3656
            }
        };
        nat_lt(a3657, a3658)
    } else {
        false
    } {
        let a3660 = match v0.clone() {
            Adt6::C0(_, h3659, _, _) => {
                h3659
            }
        };
        let a3661 = 2147483648u64;
        nat_lt(a3660, a3661)
    } else {
        false
    } {
        let a3662 = v1;
        let a3663 = 2147483648u64;
        nat_lt(a3662, a3663)
    } else {
        false
    } {
        let a3664 = 0u64;
        let a3665 = v2;
        nat_lt(a3664, a3665)
    } else {
        false
    } {
        let a3666 = v3;
        let a3667 = 2147483648u64;
        nat_lt(a3666, a3667)
    } else {
        false
    } {
        let a3679 = {
            let a3674 = {
                let a3671 = v3;
                let a3672 = {
                    let a3669 = match v0.clone() {
                        Adt6::C0(h3668, _, _, _) => {
                            h3668
                        }
                    };
                    let a3670 = 1u64;
                    nat_sub(a3669, a3670)
                };
                nat_add(a3671, a3672)?
            };
            let a3675 = f148(match v0.clone() {
                Adt6::C0(_, _, h3673, _) => {
                    h3673
                }
            });
            nat_add(a3674, a3675)?
        };
        let a3680 = {
            let a3677 = 2147483647u64;
            let a3678 = f149(match v0.clone() {
                Adt6::C0(_, _, h3676, _) => {
                    h3676
                }
            });
            nat_add(a3677, a3678)?
        };
        nat_le(a3679, a3680)
    } else {
        false
    } {
        let a3682 = match v0 {
            Adt6::C0(_, _, h3681, _) => {
                h3681
            }
        };
        let a3683 = 2147483648u64;
        Ok(!nat_eq(a3682, a3683))
    } else {
        Ok(false)
    }
}

pub fn f125(v0: Adt7) -> R<u64> {
    let a3695 = if match v0.clone() {
        Adt7::C0(_, _, _, h3684, _) => {
            h3684
        }
    } {
        2u64
    } else {
        3u64
    };
    let a3696 = {
        let a3693 = {
            let a3691 = {
                let m3687 = match v0.clone() {
                    Adt7::C0(h3686, _, _, _, _) => {
                        h3686
                    }
                };
                match m3687 {
                    Adt6::C0(h3685, _, _, _) => {
                        h3685
                    }
                }
            };
            let a3692 = {
                let m3690 = match v0.clone() {
                    Adt7::C0(_, h3689, _, _, _) => {
                        h3689
                    }
                };
                match m3690 {
                    Adt6::C0(h3688, _, _, _) => {
                        h3688
                    }
                }
            };
            nat_mul(a3691, a3692)?
        };
        let a3694 = f182(v0)?;
        nat_mul(a3693, a3694)?
    };
    nat_add(a3695, a3696)
}

pub fn f126(v0: Adt2, v1: Adt2, v2: Adt2) -> R<Adt10> {
    if f208(match v0.clone() {
        Adt2::C0(h3697, _, _, _, _, _, _, _, _, _, _) => {
            h3697
        }
    }, match v0.clone() {
        Adt2::C0(_, h3698, _, _, _, _, _, _, _, _, _) => {
            h3698
        }
    }, match v0.clone() {
        Adt2::C0(_, _, h3699, _, _, _, _, _, _, _, _) => {
            h3699
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, h3700, _, _, _, _, _, _, _) => {
            h3700
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, h3701, _, _, _, _, _, _) => {
            h3701
        }
    }, match v1.clone() {
        Adt2::C0(h3702, _, _, _, _, _, _, _, _, _, _) => {
            h3702
        }
    }, match v1.clone() {
        Adt2::C0(_, h3703, _, _, _, _, _, _, _, _, _) => {
            h3703
        }
    }, match v1.clone() {
        Adt2::C0(_, _, h3704, _, _, _, _, _, _, _, _) => {
            h3704
        }
    }, match v1.clone() {
        Adt2::C0(_, _, _, h3705, _, _, _, _, _, _, _) => {
            h3705
        }
    }, match v1.clone() {
        Adt2::C0(_, _, _, _, h3706, _, _, _, _, _, _) => {
            h3706
        }
    }, match v2.clone() {
        Adt2::C0(h3707, _, _, _, _, _, _, _, _, _, _) => {
            h3707
        }
    }, match v2.clone() {
        Adt2::C0(_, h3708, _, _, _, _, _, _, _, _, _) => {
            h3708
        }
    }, match v2.clone() {
        Adt2::C0(_, _, h3709, _, _, _, _, _, _, _, _) => {
            h3709
        }
    }, match v2.clone() {
        Adt2::C0(_, _, _, h3710, _, _, _, _, _, _, _) => {
            h3710
        }
    }, match v2.clone() {
        Adt2::C0(_, _, _, _, h3711, _, _, _, _, _, _) => {
            h3711
        }
    }) {
        let m3713 = f130(match v0.clone() {
            Adt2::C0(h3712, _, _, _, _, _, _, _, _, _, _) => {
                h3712
            }
        });
        match m3713 {
            Adt17::C0 => {
                Ok(Adt10::C0)
            }
            Adt17::C1(v3) => {
                let a3719 = {
                    let a3716 = match v0.clone() {
                        Adt2::C0(_, h3714, _, _, _, _, _, _, _, _, _) => {
                            h3714
                        }
                    };
                    let a3717 = match v3.clone() {
                        Adt18::C0(_, h3715, _) => {
                            h3715
                        }
                    };
                    let a3718 = 0u64;
                    nat_rem(a3716, a3717, a3718)
                };
                let a3720 = 0u64;
                if if nat_eq(a3719, a3720) {
                    if f209(match v0.clone() {
                        Adt2::C0(_, h3721, _, _, _, _, _, _, _, _, _) => {
                            h3721
                        }
                    }, match v1.clone() {
                        Adt2::C0(_, h3722, _, _, _, _, _, _, _, _, _) => {
                            h3722
                        }
                    }, match v1.clone() {
                        Adt2::C0(_, _, h3723, _, _, _, _, _, _, _, _) => {
                            h3723
                        }
                    }, match v1.clone() {
                        Adt2::C0(_, _, _, h3724, _, _, _, _, _, _, _) => {
                            h3724
                        }
                    }) {
                        true
                    } else {
                        let a3726 = 4u64;
                        let a3727 = match v2.clone() {
                            Adt2::C0(_, _, _, _, _, h3725, _, _, _, _, _) => {
                                h3725
                            }
                        };
                        if if nat_le(a3726, a3727) {
                            let a3729 = match v3.clone() {
                                Adt18::C0(_, h3728, _) => {
                                    h3728
                                }
                            };
                            let a3730 = 1u64;
                            if nat_eq(a3729, a3730) {
                                true
                            } else {
                                let a3733 = match v3.clone() {
                                    Adt18::C0(_, _, h3731) => {
                                        h3731
                                    }
                                };
                                let a3734 = match v0.clone() {
                                    Adt2::C0(_, _, _, _, _, h3732, _, _, _, _, _) => {
                                        h3732
                                    }
                                };
                                nat_le(a3733, a3734)
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
                        Adt18::C0(_, h3735, _) => {
                            h3735
                        }
                    });
                    Ok(Adt10::C4(if f209(match v0.clone() {
                        Adt2::C0(_, h3736, _, _, _, _, _, _, _, _, _) => {
                            h3736
                        }
                    }, match v1.clone() {
                        Adt2::C0(_, h3737, _, _, _, _, _, _, _, _, _) => {
                            h3737
                        }
                    }, match v1.clone() {
                        Adt2::C0(_, _, h3738, _, _, _, _, _, _, _, _) => {
                            h3738
                        }
                    }, match v1.clone() {
                        Adt2::C0(_, _, _, h3739, _, _, _, _, _, _, _) => {
                            h3739
                        }
                    }) {
                        0u64
                    } else {
                        f176(v4.clone(), f20(v1.clone(), 1u64))?
                    }, 4u64, v4, f20(v2, 1u64), Adt4::C20(match v0 {
                        Adt2::C0(h3740, _, _, _, _, _, _, _, _, _, _) => {
                            h3740
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

pub fn f127(v0: Adt2, v1: Adt2, v2: Adt2) -> R<Adt10> {
    if if if if if if if if if if if if if f128(match v0.clone() {
        Adt2::C0(h3741, _, _, _, _, _, _, _, _, _, _) => {
            h3741
        }
    }) {
        let a3747 = {
            let a3744 = match v0.clone() {
                Adt2::C0(_, h3742, _, _, _, _, _, _, _, _, _) => {
                    h3742
                }
            };
            let a3745 = f210(match v0.clone() {
                Adt2::C0(h3743, _, _, _, _, _, _, _, _, _, _) => {
                    h3743
                }
            });
            let a3746 = 1u64;
            nat_rem(a3744, a3745, a3746)
        };
        let a3748 = 0u64;
        nat_eq(a3747, a3748)
    } else {
        false
    } {
        let a3750 = match v1.clone() {
            Adt2::C0(h3749, _, _, _, _, _, _, _, _, _, _) => {
                h3749
            }
        };
        let a3751 = 26u64;
        nat_eq(a3750, a3751)
    } else {
        false
    } {
        let a3753 = match v2.clone() {
            Adt2::C0(h3752, _, _, _, _, _, _, _, _, _, _) => {
                h3752
            }
        };
        let a3754 = 0u64;
        nat_eq(a3753, a3754)
    } else {
        false
    } {
        let a3757 = match v0.clone() {
            Adt2::C0(_, h3755, _, _, _, _, _, _, _, _, _) => {
                h3755
            }
        };
        let a3758 = match v2.clone() {
            Adt2::C0(_, h3756, _, _, _, _, _, _, _, _, _) => {
                h3756
            }
        };
        nat_eq(a3757, a3758)
    } else {
        false
    } {
        let a3761 = match v1.clone() {
            Adt2::C0(_, h3759, _, _, _, _, _, _, _, _, _) => {
                h3759
            }
        };
        let a3762 = match v0.clone() {
            Adt2::C0(_, _, h3760, _, _, _, _, _, _, _, _) => {
                h3760
            }
        };
        nat_le(a3761, a3762)
    } else {
        false
    } {
        let a3764 = match v0.clone() {
            Adt2::C0(_, _, _, h3763, _, _, _, _, _, _, _) => {
                h3763
            }
        };
        let a3765 = 1u64;
        nat_eq(a3764, a3765)
    } else {
        false
    } {
        let a3767 = match v0.clone() {
            Adt2::C0(_, _, _, _, h3766, _, _, _, _, _, _) => {
                h3766
            }
        };
        let a3768 = 1u64;
        nat_eq(a3767, a3768)
    } else {
        false
    } {
        let a3770 = match v2.clone() {
            Adt2::C0(_, _, _, h3769, _, _, _, _, _, _, _) => {
                h3769
            }
        };
        let a3771 = 1u64;
        nat_eq(a3770, a3771)
    } else {
        false
    } {
        let a3773 = match v2.clone() {
            Adt2::C0(_, _, _, _, h3772, _, _, _, _, _, _) => {
                h3772
            }
        };
        let a3774 = 1u64;
        nat_eq(a3773, a3774)
    } else {
        false
    } {
        let a3776 = match v1.clone() {
            Adt2::C0(_, _, h3775, _, _, _, _, _, _, _, _) => {
                h3775
            }
        };
        let a3777 = 1u64;
        nat_eq(a3776, a3777)
    } else {
        false
    } {
        let a3779 = match v1.clone() {
            Adt2::C0(_, _, _, h3778, _, _, _, _, _, _, _) => {
                h3778
            }
        };
        let a3780 = 1u64;
        nat_eq(a3779, a3780)
    } else {
        false
    } {
        let a3782 = match v1.clone() {
            Adt2::C0(_, _, _, _, h3781, _, _, _, _, _, _) => {
                h3781
            }
        };
        let a3783 = 1u64;
        nat_eq(a3782, a3783)
    } else {
        false
    } {
        if f34(match v2.clone() {
            Adt2::C0(_, h3784, _, _, _, _, _, _, _, _, _) => {
                h3784
            }
        }, match v2.clone() {
            Adt2::C0(_, _, h3785, _, _, _, _, _, _, _, _) => {
                h3785
            }
        }, match v2.clone() {
            Adt2::C0(_, _, _, h3786, _, _, _, _, _, _, _) => {
                h3786
            }
        }, match v2.clone() {
            Adt2::C0(_, _, _, _, h3787, _, _, _, _, _, _) => {
                h3787
            }
        }) {
            Ok(Adt10::C4(0u64, 4u64, f20(v0, 1u64), f20(v2, 1u64), Adt4::C9(0u64, 0u64)))
        } else {
            let a3789 = match v1.clone() {
                Adt2::C0(_, h3788, _, _, _, _, _, _, _, _, _) => {
                    h3788
                }
            };
            let a3790 = 0u64;
            if nat_eq(a3789, a3790) {
                Ok(Adt10::C4(f16(v2.clone(), 1u64)?, 4u64, f20(v0, 1u64), f20(v2, 1u64), Adt4::C9(0u64, 0u64)))
            } else {
                let a3792 = match v0.clone() {
                    Adt2::C0(_, h3791, _, _, _, _, _, _, _, _, _) => {
                        h3791
                    }
                };
                let a3793 = 2147483647u64;
                if if if if if if nat_le(a3792, a3793) {
                    let a3795 = match v1.clone() {
                        Adt2::C0(_, h3794, _, _, _, _, _, _, _, _, _) => {
                            h3794
                        }
                    };
                    let a3796 = 2147483647u64;
                    nat_le(a3795, a3796)
                } else {
                    false
                } {
                    let a3798 = 4u64;
                    let a3799 = match v2.clone() {
                        Adt2::C0(_, _, _, _, _, h3797, _, _, _, _, _) => {
                            h3797
                        }
                    };
                    nat_le(a3798, a3799)
                } else {
                    false
                } {
                    let a3805 = {
                        let a3802 = match v2.clone() {
                            Adt2::C0(_, h3800, _, _, _, _, _, _, _, _, _) => {
                                h3800
                            }
                        };
                        let a3803 = match v2.clone() {
                            Adt2::C0(_, _, _, _, _, h3801, _, _, _, _, _) => {
                                h3801
                            }
                        };
                        nat_mul(a3802, a3803)?
                    };
                    let a3806 = match v2.clone() {
                        Adt2::C0(_, _, _, _, _, _, h3804, _, _, _, _) => {
                            h3804
                        }
                    };
                    nat_le(a3805, a3806)
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
                        let a3819 = {
                            let a3810 = {
                                let a3808 = 2u64;
                                let a3809 = match v1.clone() {
                                    Adt2::C0(_, h3807, _, _, _, _, _, _, _, _, _) => {
                                        h3807
                                    }
                                };
                                nat_mul(a3808, a3809)?
                            };
                            let a3811 = f16(v2.clone(), 1u64)?;
                            nat_add(a3810, a3811)?
                        };
                        let a3820 = {
                            let a3817 = match v1.clone() {
                                Adt2::C0(_, h3812, _, _, _, _, _, _, _, _, _) => {
                                    h3812
                                }
                            };
                            let a3818 = {
                                let a3815 = 1u64;
                                let a3816 = f174(match v0.clone() {
                                    Adt2::C0(h3813, _, _, _, _, _, _, _, _, _, _) => {
                                        h3813
                                    }
                                }, match v0.clone() {
                                    Adt2::C0(_, h3814, _, _, _, _, _, _, _, _, _) => {
                                        h3814
                                    }
                                })?;
                                nat_add(a3815, a3816)?
                            };
                            nat_mul(a3817, a3818)?
                        };
                        nat_add(a3819, a3820)?
                    }, 4u64, Adt3::C0(match v0.clone() {
                        Adt2::C0(_, h3821, _, _, _, _, _, _, _, _, _) => {
                            h3821
                        }
                    }, match v1.clone() {
                        Adt2::C0(_, h3822, _, _, _, _, _, _, _, _, _) => {
                            h3822
                        }
                    }, 1u64, 1u64, match v0.clone() {
                        Adt2::C0(_, _, _, _, _, h3823, _, _, _, _, _) => {
                            h3823
                        }
                    }, match v0.clone() {
                        Adt2::C0(_, _, _, _, _, _, h3824, _, _, _, _) => {
                            h3824
                        }
                    }, match v0.clone() {
                        Adt2::C0(_, _, _, _, _, _, _, h3825, _, _, _) => {
                            h3825
                        }
                    }, match v0.clone() {
                        Adt2::C0(_, _, _, _, _, _, _, _, h3826, _, _) => {
                            h3826
                        }
                    }), f20(v2, 1u64), Adt4::C19(match v0 {
                        Adt2::C0(h3827, _, _, _, _, _, _, _, _, _, _) => {
                            h3827
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

pub fn f128(v0: u64) -> bool {
    let a3828 = v0;
    let a3829 = 0u64;
    if if if if if if if if if if if if if if if if if if if if if if if if if if if if if nat_eq(a3828, a3829) {
        true
    } else {
        let a3830 = v0;
        let a3831 = 1u64;
        nat_eq(a3830, a3831)
    } {
        true
    } else {
        let a3832 = v0;
        let a3833 = 2u64;
        nat_eq(a3832, a3833)
    } {
        true
    } else {
        let a3834 = v0;
        let a3835 = 3u64;
        nat_eq(a3834, a3835)
    } {
        true
    } else {
        let a3836 = v0;
        let a3837 = 6u64;
        nat_eq(a3836, a3837)
    } {
        true
    } else {
        let a3838 = v0;
        let a3839 = 7u64;
        nat_eq(a3838, a3839)
    } {
        true
    } else {
        let a3840 = v0;
        let a3841 = 8u64;
        nat_eq(a3840, a3841)
    } {
        true
    } else {
        let a3842 = v0;
        let a3843 = 10u64;
        nat_eq(a3842, a3843)
    } {
        true
    } else {
        let a3844 = v0;
        let a3845 = 11u64;
        nat_eq(a3844, a3845)
    } {
        true
    } else {
        let a3846 = v0;
        let a3847 = 12u64;
        nat_eq(a3846, a3847)
    } {
        true
    } else {
        let a3848 = v0;
        let a3849 = 13u64;
        nat_eq(a3848, a3849)
    } {
        true
    } else {
        let a3850 = v0;
        let a3851 = 14u64;
        nat_eq(a3850, a3851)
    } {
        true
    } else {
        let a3852 = v0;
        let a3853 = 16u64;
        nat_eq(a3852, a3853)
    } {
        true
    } else {
        let a3854 = v0;
        let a3855 = 17u64;
        nat_eq(a3854, a3855)
    } {
        true
    } else {
        let a3856 = v0;
        let a3857 = 18u64;
        nat_eq(a3856, a3857)
    } {
        true
    } else {
        let a3858 = v0;
        let a3859 = 19u64;
        nat_eq(a3858, a3859)
    } {
        true
    } else {
        let a3860 = v0;
        let a3861 = 20u64;
        nat_eq(a3860, a3861)
    } {
        true
    } else {
        let a3862 = v0;
        let a3863 = 21u64;
        nat_eq(a3862, a3863)
    } {
        true
    } else {
        let a3864 = v0;
        let a3865 = 22u64;
        nat_eq(a3864, a3865)
    } {
        true
    } else {
        let a3866 = v0;
        let a3867 = 23u64;
        nat_eq(a3866, a3867)
    } {
        true
    } else {
        let a3868 = v0;
        let a3869 = 26u64;
        nat_eq(a3868, a3869)
    } {
        true
    } else {
        let a3870 = v0;
        let a3871 = 29u64;
        nat_eq(a3870, a3871)
    } {
        true
    } else {
        let a3872 = v0;
        let a3873 = 30u64;
        nat_eq(a3872, a3873)
    } {
        true
    } else {
        let a3874 = v0;
        let a3875 = 34u64;
        nat_eq(a3874, a3875)
    } {
        true
    } else {
        let a3876 = v0;
        let a3877 = 35u64;
        nat_eq(a3876, a3877)
    } {
        true
    } else {
        let a3878 = v0;
        let a3879 = 39u64;
        nat_eq(a3878, a3879)
    } {
        true
    } else {
        let a3880 = v0;
        let a3881 = 40u64;
        nat_eq(a3880, a3881)
    } {
        true
    } else {
        let a3882 = v0;
        let a3883 = 41u64;
        nat_eq(a3882, a3883)
    } {
        true
    } else {
        let a3884 = v0;
        let a3885 = 42u64;
        nat_eq(a3884, a3885)
    } {
        true
    } else {
        let a3886 = v0;
        let a3887 = 43u64;
        nat_eq(a3886, a3887)
    }
}

pub fn f129(v0: u64) -> bool {
    if f120(v0) {
        true
    } else {
        let a3888 = v0;
        let a3889 = 43u64;
        nat_eq(a3888, a3889)
    }
}

pub fn f130(v0: u64) -> Adt17 {
    let a3890 = v0;
    let a3891 = 0u64;
    if nat_eq(a3890, a3891) {
        Adt17::C1(Adt18::C0(0u64, 1u64, 4u64))
    } else {
        let a3892 = v0;
        let a3893 = 1u64;
        if nat_eq(a3892, a3893) {
            Adt17::C1(Adt18::C0(1u64, 1u64, 2u64))
        } else {
            let a3894 = v0;
            let a3895 = 2u64;
            if nat_eq(a3894, a3895) {
                Adt17::C1(Adt18::C0(2u64, 32u64, 18u64))
            } else {
                let a3896 = v0;
                let a3897 = 3u64;
                if nat_eq(a3896, a3897) {
                    Adt17::C1(Adt18::C0(3u64, 32u64, 20u64))
                } else {
                    let a3898 = v0;
                    let a3899 = 6u64;
                    if nat_eq(a3898, a3899) {
                        Adt17::C1(Adt18::C0(6u64, 32u64, 22u64))
                    } else {
                        let a3900 = v0;
                        let a3901 = 7u64;
                        if nat_eq(a3900, a3901) {
                            Adt17::C1(Adt18::C0(7u64, 32u64, 24u64))
                        } else {
                            let a3902 = v0;
                            let a3903 = 8u64;
                            if nat_eq(a3902, a3903) {
                                Adt17::C1(Adt18::C0(8u64, 32u64, 34u64))
                            } else {
                                let a3904 = v0;
                                let a3905 = 9u64;
                                if nat_eq(a3904, a3905) {
                                    Adt17::C1(Adt18::C0(9u64, 32u64, 36u64))
                                } else {
                                    let a3906 = v0;
                                    let a3907 = 10u64;
                                    if nat_eq(a3906, a3907) {
                                        Adt17::C1(Adt18::C0(10u64, 256u64, 84u64))
                                    } else {
                                        let a3908 = v0;
                                        let a3909 = 11u64;
                                        if nat_eq(a3908, a3909) {
                                            Adt17::C1(Adt18::C0(11u64, 256u64, 110u64))
                                        } else {
                                            let a3910 = v0;
                                            let a3911 = 12u64;
                                            if nat_eq(a3910, a3911) {
                                                Adt17::C1(Adt18::C0(12u64, 256u64, 144u64))
                                            } else {
                                                let a3912 = v0;
                                                let a3913 = 13u64;
                                                if nat_eq(a3912, a3913) {
                                                    Adt17::C1(Adt18::C0(13u64, 256u64, 176u64))
                                                } else {
                                                    let a3914 = v0;
                                                    let a3915 = 14u64;
                                                    if nat_eq(a3914, a3915) {
                                                        Adt17::C1(Adt18::C0(14u64, 256u64, 210u64))
                                                    } else {
                                                        let a3916 = v0;
                                                        let a3917 = 15u64;
                                                        if nat_eq(a3916, a3917) {
                                                            Adt17::C1(Adt18::C0(15u64, 256u64, 292u64))
                                                        } else {
                                                            let a3918 = v0;
                                                            let a3919 = 16u64;
                                                            if nat_eq(a3918, a3919) {
                                                                Adt17::C1(Adt18::C0(16u64, 256u64, 66u64))
                                                            } else {
                                                                let a3920 = v0;
                                                                let a3921 = 17u64;
                                                                if nat_eq(a3920, a3921) {
                                                                    Adt17::C1(Adt18::C0(17u64, 256u64, 74u64))
                                                                } else {
                                                                    let a3922 = v0;
                                                                    let a3923 = 18u64;
                                                                    if nat_eq(a3922, a3923) {
                                                                        Adt17::C1(Adt18::C0(18u64, 256u64, 98u64))
                                                                    } else {
                                                                        let a3924 = v0;
                                                                        let a3925 = 19u64;
                                                                        if nat_eq(a3924, a3925) {
                                                                            Adt17::C1(Adt18::C0(19u64, 256u64, 50u64))
                                                                        } else {
                                                                            let a3926 = v0;
                                                                            let a3927 = 20u64;
                                                                            if nat_eq(a3926, a3927) {
                                                                                Adt17::C1(Adt18::C0(20u64, 32u64, 18u64))
                                                                            } else {
                                                                                let a3928 = v0;
                                                                                let a3929 = 21u64;
                                                                                if nat_eq(a3928, a3929) {
                                                                                    Adt17::C1(Adt18::C0(21u64, 256u64, 110u64))
                                                                                } else {
                                                                                    let a3930 = v0;
                                                                                    let a3931 = 22u64;
                                                                                    if nat_eq(a3930, a3931) {
                                                                                        Adt17::C1(Adt18::C0(22u64, 256u64, 82u64))
                                                                                    } else {
                                                                                        let a3932 = v0;
                                                                                        let a3933 = 23u64;
                                                                                        if nat_eq(a3932, a3933) {
                                                                                            Adt17::C1(Adt18::C0(23u64, 256u64, 136u64))
                                                                                        } else {
                                                                                            let a3934 = v0;
                                                                                            let a3935 = 24u64;
                                                                                            if nat_eq(a3934, a3935) {
                                                                                                Adt17::C1(Adt18::C0(24u64, 1u64, 1u64))
                                                                                            } else {
                                                                                                let a3936 = v0;
                                                                                                let a3937 = 25u64;
                                                                                                if nat_eq(a3936, a3937) {
                                                                                                    Adt17::C1(Adt18::C0(25u64, 1u64, 2u64))
                                                                                                } else {
                                                                                                    let a3938 = v0;
                                                                                                    let a3939 = 26u64;
                                                                                                    if nat_eq(a3938, a3939) {
                                                                                                        Adt17::C1(Adt18::C0(26u64, 1u64, 4u64))
                                                                                                    } else {
                                                                                                        let a3940 = v0;
                                                                                                        let a3941 = 27u64;
                                                                                                        if nat_eq(a3940, a3941) {
                                                                                                            Adt17::C1(Adt18::C0(27u64, 1u64, 8u64))
                                                                                                        } else {
                                                                                                            let a3942 = v0;
                                                                                                            let a3943 = 28u64;
                                                                                                            if nat_eq(a3942, a3943) {
                                                                                                                Adt17::C1(Adt18::C0(28u64, 1u64, 8u64))
                                                                                                            } else {
                                                                                                                let a3944 = v0;
                                                                                                                let a3945 = 29u64;
                                                                                                                if nat_eq(a3944, a3945) {
                                                                                                                    Adt17::C1(Adt18::C0(29u64, 256u64, 56u64))
                                                                                                                } else {
                                                                                                                    let a3946 = v0;
                                                                                                                    let a3947 = 30u64;
                                                                                                                    if nat_eq(a3946, a3947) {
                                                                                                                        Adt17::C1(Adt18::C0(30u64, 1u64, 2u64))
                                                                                                                    } else {
                                                                                                                        let a3948 = v0;
                                                                                                                        let a3949 = 34u64;
                                                                                                                        if nat_eq(a3948, a3949) {
                                                                                                                            Adt17::C1(Adt18::C0(34u64, 256u64, 54u64))
                                                                                                                        } else {
                                                                                                                            let a3950 = v0;
                                                                                                                            let a3951 = 35u64;
                                                                                                                            if nat_eq(a3950, a3951) {
                                                                                                                                Adt17::C1(Adt18::C0(35u64, 256u64, 66u64))
                                                                                                                            } else {
                                                                                                                                let a3952 = v0;
                                                                                                                                let a3953 = 39u64;
                                                                                                                                if nat_eq(a3952, a3953) {
                                                                                                                                    Adt17::C1(Adt18::C0(39u64, 32u64, 17u64))
                                                                                                                                } else {
                                                                                                                                    let a3954 = v0;
                                                                                                                                    let a3955 = 40u64;
                                                                                                                                    if nat_eq(a3954, a3955) {
                                                                                                                                        Adt17::C1(Adt18::C0(40u64, 64u64, 36u64))
                                                                                                                                    } else {
                                                                                                                                        let a3956 = v0;
                                                                                                                                        let a3957 = 41u64;
                                                                                                                                        if nat_eq(a3956, a3957) {
                                                                                                                                            Adt17::C1(Adt18::C0(41u64, 128u64, 18u64))
                                                                                                                                        } else {
                                                                                                                                            let a3958 = v0;
                                                                                                                                            let a3959 = 42u64;
                                                                                                                                            if nat_eq(a3958, a3959) {
                                                                                                                                                Adt17::C1(Adt18::C0(42u64, 64u64, 18u64))
                                                                                                                                            } else {
                                                                                                                                                let a3960 = v0;
                                                                                                                                                let a3961 = 43u64;
                                                                                                                                                if nat_eq(a3960, a3961) {
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

pub fn f131(v0: u64, v1: Adt2, v2: bool, v3: Adt2) -> R<Adt10> {
    let a3964 = match v1.clone() {
        Adt2::C0(h3962, _, _, _, _, _, _, _, _, _, _) => {
            h3962
        }
    };
    let a3965 = match v3.clone() {
        Adt2::C0(h3963, _, _, _, _, _, _, _, _, _, _) => {
            h3963
        }
    };
    if if nat_eq(a3964, a3965) {
        let a3966 = v0;
        let a3967 = 33u64;
        if nat_eq(a3966, a3967) {
            v2
        } else {
            true
        }
    } else {
        false
    } {
        let m3969 = f130(match v1.clone() {
            Adt2::C0(h3968, _, _, _, _, _, _, _, _, _, _) => {
                h3968
            }
        });
        match m3969 {
            Adt17::C0 => {
                Ok(Adt10::C0)
            }
            Adt17::C1(v4) => {
                let v5: u64 = match v4.clone() {
                    Adt18::C0(_, h3970, _) => {
                        h3970
                    }
                };
                let v6: u64 = f16(v1.clone(), v5)?;
                let a3975 = {
                    let a3972 = match v1.clone() {
                        Adt2::C0(_, h3971, _, _, _, _, _, _, _, _, _) => {
                            h3971
                        }
                    };
                    let a3973 = v5;
                    let a3974 = 1u64;
                    nat_rem(a3972, a3973, a3974)
                };
                let a3976 = 0u64;
                if if if nat_eq(a3975, a3976) {
                    let a3981 = {
                        let a3978 = match v3.clone() {
                            Adt2::C0(_, h3977, _, _, _, _, _, _, _, _, _) => {
                                h3977
                            }
                        };
                        let a3979 = v5;
                        let a3980 = 1u64;
                        nat_rem(a3978, a3979, a3980)
                    };
                    let a3982 = 0u64;
                    nat_eq(a3981, a3982)
                } else {
                    false
                } {
                    let a3983 = v6;
                    let a3984 = f16(v3.clone(), v5)?;
                    nat_eq(a3983, a3984)
                } else {
                    false
                } {
                    Ok(Adt10::C4(v6, match v4 {
                        Adt18::C0(_, _, h3985) => {
                            h3985
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

pub fn f132(v0: u64, v1: u64) -> u64 {
    let a3986 = v0;
    let a3987 = 0u64;
    if if nat_eq(a3986, a3987) {
        true
    } else {
        let a3988 = v1;
        let a3989 = 0u64;
        nat_eq(a3988, a3989)
    } {
        1u64
    } else {
        2u64
    }
}

pub fn f133(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64, v7: u64) -> bool {
    if f34(v4, v5, v6, v7) {
        f34(v0, v1, v2, v3)
    } else {
        let a3993 = {
            let a3990 = v0;
            let a3991 = v4;
            let a3992 = 1u64;
            nat_rem(a3990, a3991, a3992)
        };
        let a3994 = 0u64;
        if if if nat_eq(a3993, a3994) {
            let a3998 = {
                let a3995 = v1;
                let a3996 = v5;
                let a3997 = 1u64;
                nat_rem(a3995, a3996, a3997)
            };
            let a3999 = 0u64;
            nat_eq(a3998, a3999)
        } else {
            false
        } {
            let a4003 = {
                let a4000 = v2;
                let a4001 = v6;
                let a4002 = 1u64;
                nat_rem(a4000, a4001, a4002)
            };
            let a4004 = 0u64;
            nat_eq(a4003, a4004)
        } else {
            false
        } {
            let a4008 = {
                let a4005 = v3;
                let a4006 = v7;
                let a4007 = 1u64;
                nat_rem(a4005, a4006, a4007)
            };
            let a4009 = 0u64;
            nat_eq(a4008, a4009)
        } else {
            false
        }
    }
}

pub fn f134(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64) -> R<bool> {
    let a4010 = v0;
    let a4011 = v1;
    if nat_eq(a4010, a4011) {
        let a4014 = v4;
        let a4015 = {
            let a4012 = v2;
            let a4013 = v3;
            nat_add(a4012, a4013)?
        };
        Ok(nat_eq(a4014, a4015))
    } else {
        let a4016 = v2;
        let a4017 = v3;
        if nat_eq(a4016, a4017) {
            let a4018 = v4;
            let a4019 = v2;
            Ok(nat_eq(a4018, a4019))
        } else {
            Ok(false)
        }
    }
}

pub fn f135(v0: u64, v1: u64, v2: u64, v3: u64) -> R<bool> {
    let v4: u64 = {
        let a4022 = {
            let a4020 = v0;
            let a4021 = f149(v1);
            nat_add(a4020, a4021)?
        };
        let a4023 = f149(v2);
        nat_add(a4022, a4023)?
    };
    let v5: u64 = {
        let a4024 = f148(v1);
        let a4025 = f148(v2);
        nat_add(a4024, a4025)?
    };
    let a4026 = v1;
    let a4027 = 4294967295u64;
    if if nat_le(a4026, a4027) {
        let a4028 = v2;
        let a4029 = 4294967295u64;
        nat_le(a4028, a4029)
    } else {
        false
    } {
        let a4030 = v5;
        let a4031 = v4;
        if nat_le(a4030, a4031) {
            let a4034 = v3;
            let a4035 = {
                let a4032 = v4;
                let a4033 = v5;
                nat_sub(a4032, a4033)
            };
            Ok(nat_eq(a4034, a4035))
        } else {
            Ok(false)
        }
    } else {
        Ok(false)
    }
}

pub fn f136(v0: u64, v1: u64) -> R<bool> {
    let a4036 = v1;
    let a4037 = 4294967295u64;
    if nat_le(a4036, a4037) {
        let a4040 = {
            let a4038 = f149(v1);
            let a4039 = f148(v1);
            nat_add(a4038, a4039)?
        };
        let a4041 = v0;
        Ok(nat_lt(a4040, a4041))
    } else {
        Ok(false)
    }
}

pub fn f137(v0: Adt3, v1: u64, v2: u64, v3: u64, v4: bool) -> R<bool> {
    let v5: u64 = f211(v0.clone(), false)?;
    let v6: u64 = f211(v0, true)?;
    let a4042 = v6;
    let a4043 = v1;
    if if nat_le(a4042, a4043) {
        let a4050 = {
            let a4048 = {
                let a4044 = v1;
                let a4045 = v5;
                nat_add(a4044, a4045)?
            };
            let a4049 = {
                let a4046 = v3;
                let a4047 = 0u64;
                if nat_eq(a4046, a4047) {
                    0u64
                } else {
                    4u64
                }
            };
            nat_add(a4048, a4049)?
        };
        let a4051 = v2;
        nat_le(a4050, a4051)
    } else {
        false
    } {
        if v4 {
            let a4056 = {
                let a4054 = {
                    let a4052 = v1;
                    let a4053 = v5;
                    nat_add(a4052, a4053)?
                };
                let a4055 = v6;
                nat_sub(a4054, a4055)
            };
            let a4057 = v2;
            Ok(nat_lt(a4056, a4057))
        } else {
            let a4062 = {
                let a4060 = {
                    let a4058 = v1;
                    let a4059 = v5;
                    nat_add(a4058, a4059)?
                };
                let a4061 = v6;
                nat_sub(a4060, a4061)
            };
            let a4063 = v2;
            Ok(nat_le(a4062, a4063))
        }
    } else {
        Ok(false)
    }
}

pub fn f138(v0: u64, v1: u64) -> bool {
    let a4064 = v0;
    let a4065 = 0u64;
    if if nat_eq(a4064, a4065) {
        true
    } else {
        let a4066 = v0;
        let a4067 = 1u64;
        nat_eq(a4066, a4067)
    } {
        let a4068 = v1;
        let a4069 = 4294967296u64;
        nat_lt(a4068, a4069)
    } else {
        false
    }
}

pub fn f139(v0: u64, v1: u64) -> R<u64> {
    let a4074 = {
        let a4072 = v0;
        let a4073 = {
            let a4070 = v1;
            let a4071 = 1u64;
            nat_sub(a4070, a4071)
        };
        nat_add(a4072, a4073)?
    };
    let a4075 = v1;
    let a4076 = 0u64;
    Ok(nat_quot(a4074, a4075, a4076))
}

pub fn f140(v0: Adt2, v1: Adt2, v2: u64, v3: u64) -> R<bool> {
    if f34(match v0.clone() {
        Adt2::C0(_, h4077, _, _, _, _, _, _, _, _, _) => {
            h4077
        }
    }, match v0.clone() {
        Adt2::C0(_, _, h4078, _, _, _, _, _, _, _, _) => {
            h4078
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, h4079, _, _, _, _, _, _, _) => {
            h4079
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, h4080, _, _, _, _, _, _) => {
            h4080
        }
    }) {
        Ok(true)
    } else {
        let v4: u64 = {
            let a4083 = match v1.clone() {
                Adt2::C0(_, h4081, _, _, _, _, _, _, _, _, _) => {
                    h4081
                }
            };
            let a4084 = match v1.clone() {
                Adt2::C0(_, _, h4082, _, _, _, _, _, _, _, _) => {
                    h4082
                }
            };
            nat_mul(a4083, a4084)?
        };
        let v5: u64 = {
            let a4086 = v4;
            let a4087 = match v1.clone() {
                Adt2::C0(_, _, _, h4085, _, _, _, _, _, _, _) => {
                    h4085
                }
            };
            nat_mul(a4086, a4087)?
        };
        let a4100 = {
            let a4098 = {
                let a4089 = match v0.clone() {
                    Adt2::C0(_, h4088, _, _, _, _, _, _, _, _, _) => {
                        h4088
                    }
                };
                let a4090 = 1u64;
                nat_sub(a4089, a4090)
            };
            let a4099 = {
                let a4096 = f212(match v0.clone() {
                    Adt2::C0(_, _, h4091, _, _, _, _, _, _, _, _) => {
                        h4091
                    }
                }, v2, v5, match v1.clone() {
                    Adt2::C0(_, h4092, _, _, _, _, _, _, _, _, _) => {
                        h4092
                    }
                })?;
                let a4097 = f212(match v0 {
                    Adt2::C0(_, _, _, h4093, _, _, _, _, _, _, _) => {
                        h4093
                    }
                }, v2, {
                    let a4094 = v3;
                    let a4095 = v5;
                    nat_mul(a4094, a4095)?
                }, v4)?;
                nat_add(a4096, a4097)?
            };
            nat_add(a4098, a4099)?
        };
        let a4101 = f16(v1, 1u64)?;
        Ok(nat_lt(a4100, a4101))
    }
}

pub fn f141(v0: u64, v1: u64, v2: u64) -> R<bool> {
    let a4102 = v0;
    let a4103 = 2147483648u64;
    if if if if nat_lt(a4102, a4103) {
        let a4104 = v1;
        let a4105 = 2147483648u64;
        nat_lt(a4104, a4105)
    } else {
        false
    } {
        let a4108 = {
            let a4106 = v0;
            let a4107 = v1;
            nat_mul(a4106, a4107)?
        };
        let a4109 = 2147483648u64;
        nat_lt(a4108, a4109)
    } else {
        false
    } {
        let a4114 = {
            let a4112 = {
                let a4110 = v0;
                let a4111 = 1u64;
                nat_sub(a4110, a4111)
            };
            let a4113 = v2;
            nat_mul(a4112, a4113)?
        };
        let a4115 = 2147483648u64;
        nat_lt(a4114, a4115)
    } else {
        false
    } {
        let a4120 = {
            let a4118 = {
                let a4116 = v1;
                let a4117 = 1u64;
                nat_sub(a4116, a4117)
            };
            let a4119 = v2;
            nat_mul(a4118, a4119)?
        };
        let a4121 = 2147483648u64;
        Ok(nat_lt(a4120, a4121))
    } else {
        Ok(false)
    }
}

pub fn f142(v0: u64) -> R<u64> {
    let a4124 = 3u64;
    let a4125 = {
        let a4122 = v0;
        let a4123 = 3u64;
        nat_mul(a4122, a4123)?
    };
    nat_add(a4124, a4125)
}

pub fn f143(v0: Adt3) -> R<u64> {
    if f34(match v0.clone() {
        Adt3::C0(h4126, _, _, _, _, _, _, _) => {
            h4126
        }
    }, match v0.clone() {
        Adt3::C0(_, h4127, _, _, _, _, _, _) => {
            h4127
        }
    }, match v0.clone() {
        Adt3::C0(_, _, h4128, _, _, _, _, _) => {
            h4128
        }
    }, match v0.clone() {
        Adt3::C0(_, _, _, h4129, _, _, _, _) => {
            h4129
        }
    }) {
        Ok(0u64)
    } else {
        let a4138 = match v0.clone() {
            Adt3::C0(h4130, _, _, _, _, _, _, _) => {
                h4130
            }
        };
        let a4139 = {
            let a4136 = match v0.clone() {
                Adt3::C0(_, h4131, _, _, _, _, _, _) => {
                    h4131
                }
            };
            let a4137 = {
                let a4134 = match v0.clone() {
                    Adt3::C0(_, _, h4132, _, _, _, _, _) => {
                        h4132
                    }
                };
                let a4135 = match v0 {
                    Adt3::C0(_, _, _, h4133, _, _, _, _) => {
                        h4133
                    }
                };
                nat_mul(a4134, a4135)?
            };
            nat_mul(a4136, a4137)?
        };
        nat_mul(a4138, a4139)
    }
}

pub fn f144(v0: Adt3, v1: u64, v2: u64) -> R<u64> {
    let a4140 = v1;
    let a4141 = 13u64;
    if if nat_eq(a4140, a4141) {
        f34(match v0.clone() {
            Adt3::C0(h4142, _, _, _, _, _, _, _) => {
                h4142
            }
        }, match v0.clone() {
            Adt3::C0(_, h4143, _, _, _, _, _, _) => {
                h4143
            }
        }, match v0.clone() {
            Adt3::C0(_, _, h4144, _, _, _, _, _) => {
                h4144
            }
        }, match v0.clone() {
            Adt3::C0(_, _, _, h4145, _, _, _, _) => {
                h4145
            }
        })
    } else {
        false
    } {
        Ok(2u64)
    } else {
        let v3: u64 = {
            let a4151 = match v0.clone() {
                Adt3::C0(_, h4146, _, _, _, _, _, _) => {
                    h4146
                }
            };
            let a4152 = {
                let a4149 = match v0.clone() {
                    Adt3::C0(_, _, h4147, _, _, _, _, _) => {
                        h4147
                    }
                };
                let a4150 = match v0.clone() {
                    Adt3::C0(_, _, _, h4148, _, _, _, _) => {
                        h4148
                    }
                };
                nat_mul(a4149, a4150)?
            };
            nat_mul(a4151, a4152)?
        };
        let a4153 = v1;
        let a4154 = 13u64;
        if nat_eq(a4153, a4154) {
            let a4163 = 2u64;
            let a4164 = if f213(v0.clone(), v2) {
                let a4156 = v3;
                let a4157 = f214(match v0 {
                    Adt3::C0(h4155, _, _, _, _, _, _, _) => {
                        h4155
                    }
                }, false)?;
                nat_mul(a4156, a4157)?
            } else {
                let a4161 = v3;
                let a4162 = {
                    let a4159 = match v0 {
                        Adt3::C0(h4158, _, _, _, _, _, _, _) => {
                            h4158
                        }
                    };
                    let a4160 = 2u64;
                    nat_mul(a4159, a4160)?
                };
                nat_mul(a4161, a4162)?
            };
            nat_add(a4163, a4164)
        } else {
            let a4168 = v3;
            let a4169 = f214(match v0 {
                Adt3::C0(h4165, _, _, _, _, _, _, _) => {
                    h4165
                }
            }, {
                let a4166 = v1;
                let a4167 = 16u64;
                nat_eq(a4166, a4167)
            })?;
            nat_mul(a4168, a4169)
        }
    }
}

pub fn f145(v0: u64, v1: u64, v2: Adt2, v3: Option<Adt2>, v4: Adt2) -> R<Adt10> {
    let a4170 = v0;
    let a4171 = 30u64;
    if if nat_eq(a4170, a4171) {
        let a4174 = match v2.clone() {
            Adt2::C0(_, _, _, _, _, _, h4172, _, _, _, _) => {
                h4172
            }
        };
        let a4175 = match v2.clone() {
            Adt2::C0(_, _, _, _, _, h4173, _, _, _, _, _) => {
                h4173
            }
        };
        nat_lt(a4174, a4175)
    } else {
        false
    } {
        Ok(Adt10::C1)
    } else {
        let m4176 = v3;
        match m4176 {
            None => {
                f215(v0, v1, match v2.clone() {
                    Adt2::C0(h4177, _, _, _, _, _, _, _, _, _, _) => {
                        h4177
                    }
                }, 0u64, match v4.clone() {
                    Adt2::C0(h4178, _, _, _, _, _, _, _, _, _, _) => {
                        h4178
                    }
                }, false, match v2.clone() {
                    Adt2::C0(_, h4179, _, _, _, _, _, _, _, _, _) => {
                        h4179
                    }
                }, match v2.clone() {
                    Adt2::C0(_, _, h4180, _, _, _, _, _, _, _, _) => {
                        h4180
                    }
                }, match v2.clone() {
                    Adt2::C0(_, _, _, h4181, _, _, _, _, _, _, _) => {
                        h4181
                    }
                }, match v2 {
                    Adt2::C0(_, _, _, _, h4182, _, _, _, _, _, _) => {
                        h4182
                    }
                }, 1u64, 1u64, 1u64, 1u64, match v4.clone() {
                    Adt2::C0(_, h4183, _, _, _, _, _, _, _, _, _) => {
                        h4183
                    }
                }, match v4.clone() {
                    Adt2::C0(_, _, h4184, _, _, _, _, _, _, _, _) => {
                        h4184
                    }
                }, match v4.clone() {
                    Adt2::C0(_, _, _, h4185, _, _, _, _, _, _, _) => {
                        h4185
                    }
                }, match v4 {
                    Adt2::C0(_, _, _, _, h4186, _, _, _, _, _, _) => {
                        h4186
                    }
                })
            }
            Some(v5) => {
                f215(v0, v1, match v2.clone() {
                    Adt2::C0(h4187, _, _, _, _, _, _, _, _, _, _) => {
                        h4187
                    }
                }, match v5.clone() {
                    Adt2::C0(h4188, _, _, _, _, _, _, _, _, _, _) => {
                        h4188
                    }
                }, match v4.clone() {
                    Adt2::C0(h4189, _, _, _, _, _, _, _, _, _, _) => {
                        h4189
                    }
                }, true, match v2.clone() {
                    Adt2::C0(_, h4190, _, _, _, _, _, _, _, _, _) => {
                        h4190
                    }
                }, match v2.clone() {
                    Adt2::C0(_, _, h4191, _, _, _, _, _, _, _, _) => {
                        h4191
                    }
                }, match v2.clone() {
                    Adt2::C0(_, _, _, h4192, _, _, _, _, _, _, _) => {
                        h4192
                    }
                }, match v2 {
                    Adt2::C0(_, _, _, _, h4193, _, _, _, _, _, _) => {
                        h4193
                    }
                }, match v5.clone() {
                    Adt2::C0(_, h4194, _, _, _, _, _, _, _, _, _) => {
                        h4194
                    }
                }, match v5.clone() {
                    Adt2::C0(_, _, h4195, _, _, _, _, _, _, _, _) => {
                        h4195
                    }
                }, match v5.clone() {
                    Adt2::C0(_, _, _, h4196, _, _, _, _, _, _, _) => {
                        h4196
                    }
                }, match v5 {
                    Adt2::C0(_, _, _, _, h4197, _, _, _, _, _, _) => {
                        h4197
                    }
                }, match v4.clone() {
                    Adt2::C0(_, h4198, _, _, _, _, _, _, _, _, _) => {
                        h4198
                    }
                }, match v4.clone() {
                    Adt2::C0(_, _, h4199, _, _, _, _, _, _, _, _) => {
                        h4199
                    }
                }, match v4.clone() {
                    Adt2::C0(_, _, _, h4200, _, _, _, _, _, _, _) => {
                        h4200
                    }
                }, match v4 {
                    Adt2::C0(_, _, _, _, h4201, _, _, _, _, _, _) => {
                        h4201
                    }
                })
            }
        }
    }
}

pub fn f146(v0: u64, v1: u64) -> u64 {
    let a4202 = v0;
    let a4203 = 23u64;
    if nat_eq(a4202, a4203) {
        let a4204 = v1;
        let a4205 = 0u64;
        if nat_eq(a4204, a4205) {
            2u64
        } else {
            3u64
        }
    } else {
        1u64
    }
}

pub fn f147(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64, v7: u64) -> R<u64> {
    let a4218 = {
        let a4214 = {
            let a4210 = {
                let a4206 = v0;
                let a4207 = v4;
                nat_mul(a4206, a4207)?
            };
            let a4211 = {
                let a4208 = v1;
                let a4209 = v5;
                nat_mul(a4208, a4209)?
            };
            nat_add(a4210, a4211)?
        };
        let a4215 = {
            let a4212 = v2;
            let a4213 = v6;
            nat_mul(a4212, a4213)?
        };
        nat_add(a4214, a4215)?
    };
    let a4219 = {
        let a4216 = v3;
        let a4217 = v7;
        nat_mul(a4216, a4217)?
    };
    nat_add(a4218, a4219)
}

pub fn f148(v0: u64) -> u64 {
    let a4220 = v0;
    let a4221 = 2147483648u64;
    if nat_lt(a4220, a4221) {
        0u64
    } else {
        let a4222 = 4294967296u64;
        let a4223 = v0;
        nat_sub(a4222, a4223)
    }
}

pub fn f149(v0: u64) -> u64 {
    let a4224 = v0;
    let a4225 = 2147483648u64;
    if nat_lt(a4224, a4225) {
        v0
    } else {
        0u64
    }
}

pub fn f150(v0: Adt11, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64, v7: u64, v8: u64) -> Adt9 {
    let v9: u64 = {
        let a4226 = v7;
        let a4227 = 6u64;
        if nat_eq(a4226, a4227) {
            27u64
        } else {
            v2
        }
    };
    Adt9::C7(v0, f100(v3, v4), f100(v5, v6), f101(v7, v8), f173(v1), f173(v9))
}

pub fn f151(v0: Adt4, v1: u64, v2: u64) -> Adt11 {
    let m4228 = v0;
    match m4228 {
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
        Adt4::C13(v42, v43, v44, v45) => {
            Adt11::C4(v42, v43, v44, v45)
        }
        Adt4::C14(v46, _, _) => {
            let a4229 = 13u64;
            let a4230 = v46;
            if nat_le(a4229, a4230) {
                Adt11::C7(v46, v1, v2)
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
        Adt4::C27(v66) => {
            Adt11::C2(match v66.clone() {
                Adt5::C0(_, _, _, _, _, h4231, _, _) => {
                    h4231
                }
            }, match v66 {
                Adt5::C0(_, _, _, _, _, _, _, h4232) => {
                    h4232
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
    }
}

pub fn f152(v0: Adt4) -> u64 {
    let m4233 = v0;
    match m4233 {
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
        Adt4::C14(v44, _, _) => {
            v44
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
    }
}

pub fn f153(v0: Adt4, v1: u64) -> u64 {
    let m4234 = v0;
    match m4234 {
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
        Adt4::C16(_, v49) => {
            v49
        }
        Adt4::C17 => {
            0u64
        }
        Adt4::C18(_, _) => {
            0u64
        }
        Adt4::C19(v52, _) => {
            let a4235 = v1;
            let a4236 = 6u64;
            if nat_eq(a4235, a4236) {
                v52
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
        Adt4::C26(v66, _, _, _) => {
            let a4245 = v1;
            let a4246 = 6u64;
            if if if nat_eq(a4245, a4246) {
                true
            } else {
                let a4247 = v1;
                let a4248 = 7u64;
                nat_eq(a4247, a4248)
            } {
                let a4249 = v66;
                let a4250 = 30u64;
                nat_eq(a4249, a4250)
            } else {
                false
            } {
                30u64
            } else {
                0u64
            }
        }
        Adt4::C27(v65) => {
            match v65 {
                Adt5::C0(_, _, _, _, _, _, _, h4244) => {
                    h4244
                }
            }
        }
        Adt4::C28(v61) => {
            let a4237 = v1;
            let a4238 = 6u64;
            if nat_eq(a4237, a4238) {
                match v61 {
                    Adt7::C0(_, _, _, _, h4239) => {
                        h4239
                    }
                }
            } else {
                0u64
            }
        }
        Adt4::C29(_) => {
            0u64
        }
        Adt4::C30(_, v64) => {
            let a4240 = v1;
            let a4241 = 6u64;
            if if nat_eq(a4240, a4241) {
                true
            } else {
                let a4242 = v1;
                let a4243 = 7u64;
                nat_eq(a4242, a4243)
            } {
                v64
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
    }
}

pub fn f154(v0: Adt4, v1: u64) -> Adt11 {
    let a4251 = v1;
    let a4252 = 4u64;
    if nat_lt(a4251, a4252) {
        Adt11::C0(v1)
    } else {
        let a4253 = v1;
        let a4254 = 4u64;
        if nat_eq(a4253, a4254) {
            Adt11::C1(0u64)
        } else {
            let a4255 = v1;
            let a4256 = 5u64;
            if nat_eq(a4255, a4256) {
                Adt11::C1(3u64)
            } else {
                let a4257 = v1;
                let a4258 = 6u64;
                if nat_eq(a4257, a4258) {
                    Adt11::C2(f153(v0, v1), 0u64, false)
                } else {
                    let a4259 = v1;
                    let a4260 = 7u64;
                    if nat_eq(a4259, a4260) {
                        Adt11::C2(0u64, f153(v0, v1), false)
                    } else {
                        let a4261 = v1;
                        let a4262 = 16u64;
                        if nat_lt(a4261, a4262) {
                            Adt11::C1({
                                let a4263 = v1;
                                let a4264 = 8u64;
                                nat_sub(a4263, a4264)
                            })
                        } else {
                            let a4265 = v1;
                            let a4266 = 20u64;
                            if nat_lt(a4265, a4266) {
                                Adt11::C5({
                                    let a4267 = v1;
                                    let a4268 = 16u64;
                                    nat_sub(a4267, a4268)
                                })
                            } else {
                                let a4269 = v1;
                                let a4270 = 20u64;
                                if nat_eq(a4269, a4270) {
                                    Adt11::C3(0u64, 0u64, false)
                                } else {
                                    let a4271 = v1;
                                    let a4272 = 21u64;
                                    if nat_eq(a4271, a4272) {
                                        Adt11::C0(4u64)
                                    } else {
                                        let a4273 = v1;
                                        let a4274 = 22u64;
                                        if nat_eq(a4273, a4274) {
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

pub fn f155(v0: u64, v1: Adt3) -> R<u64> {
    let v2: u64 = {
        let a4276 = v0;
        let a4277 = match v1.clone() {
            Adt3::C0(h4275, _, _, _, _, _, _, _) => {
                h4275
            }
        };
        let a4278 = 0u64;
        nat_quot(a4276, a4277, a4278)
    };
    let v3: u64 = {
        let a4280 = v2;
        let a4281 = match v1.clone() {
            Adt3::C0(_, h4279, _, _, _, _, _, _) => {
                h4279
            }
        };
        let a4282 = 0u64;
        nat_quot(a4280, a4281, a4282)
    };
    f147({
        let a4284 = v0;
        let a4285 = match v1.clone() {
            Adt3::C0(h4283, _, _, _, _, _, _, _) => {
                h4283
            }
        };
        let a4286 = 0u64;
        nat_rem(a4284, a4285, a4286)
    }, {
        let a4288 = v2;
        let a4289 = match v1.clone() {
            Adt3::C0(_, h4287, _, _, _, _, _, _) => {
                h4287
            }
        };
        let a4290 = 0u64;
        nat_rem(a4288, a4289, a4290)
    }, {
        let a4292 = v3;
        let a4293 = match v1.clone() {
            Adt3::C0(_, _, h4291, _, _, _, _, _) => {
                h4291
            }
        };
        let a4294 = 0u64;
        nat_rem(a4292, a4293, a4294)
    }, {
        let a4296 = v3;
        let a4297 = match v1.clone() {
            Adt3::C0(_, _, h4295, _, _, _, _, _) => {
                h4295
            }
        };
        let a4298 = 0u64;
        nat_quot(a4296, a4297, a4298)
    }, match v1.clone() {
        Adt3::C0(_, _, _, _, h4299, _, _, _) => {
            h4299
        }
    }, match v1.clone() {
        Adt3::C0(_, _, _, _, _, h4300, _, _) => {
            h4300
        }
    }, match v1.clone() {
        Adt3::C0(_, _, _, _, _, _, h4301, _) => {
            h4301
        }
    }, match v1 {
        Adt3::C0(_, _, _, _, _, _, _, h4302) => {
            h4302
        }
    })
}

pub fn f156(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let a4320 = {
        let a4315 = f216(v2, {
            let a4306 = {
                let a4303 = v0;
                let a4304 = v1;
                let a4305 = 0u64;
                nat_quot(a4303, a4304, a4305)
            };
            let a4307 = v2;
            let a4308 = 0u64;
            nat_rem(a4306, a4307, a4308)
        }, {
            let a4312 = {
                let a4309 = v0;
                let a4310 = v1;
                let a4311 = 0u64;
                nat_quot(a4309, a4310, a4311)
            };
            let a4313 = v2;
            let a4314 = 0u64;
            nat_quot(a4312, a4313, a4314)
        })?;
        let a4316 = v1;
        nat_mul(a4315, a4316)?
    };
    let a4321 = {
        let a4317 = v0;
        let a4318 = v1;
        let a4319 = 0u64;
        nat_rem(a4317, a4318, a4319)
    };
    nat_add(a4320, a4321)
}

pub fn f157(v0: u64, v1: Adt3, v2: Adt3) -> R<u64> {
    let v3: u64 = {
        let a4323 = v0;
        let a4324 = match v2.clone() {
            Adt3::C0(h4322, _, _, _, _, _, _, _) => {
                h4322
            }
        };
        let a4325 = 0u64;
        nat_quot(a4323, a4324, a4325)
    };
    let v4: u64 = {
        let a4327 = v3;
        let a4328 = match v2.clone() {
            Adt3::C0(_, h4326, _, _, _, _, _, _) => {
                h4326
            }
        };
        let a4329 = 0u64;
        nat_quot(a4327, a4328, a4329)
    };
    f217({
        let a4331 = v0;
        let a4332 = match v2.clone() {
            Adt3::C0(h4330, _, _, _, _, _, _, _) => {
                h4330
            }
        };
        let a4333 = 0u64;
        nat_rem(a4331, a4332, a4333)
    }, {
        let a4335 = v3;
        let a4336 = match v2.clone() {
            Adt3::C0(_, h4334, _, _, _, _, _, _) => {
                h4334
            }
        };
        let a4337 = 0u64;
        nat_rem(a4335, a4336, a4337)
    }, {
        let a4339 = v4;
        let a4340 = match v2.clone() {
            Adt3::C0(_, _, h4338, _, _, _, _, _) => {
                h4338
            }
        };
        let a4341 = 0u64;
        nat_rem(a4339, a4340, a4341)
    }, {
        let a4343 = v4;
        let a4344 = match v2 {
            Adt3::C0(_, _, h4342, _, _, _, _, _) => {
                h4342
            }
        };
        let a4345 = 0u64;
        nat_quot(a4343, a4344, a4345)
    }, match v1.clone() {
        Adt3::C0(h4346, _, _, _, _, _, _, _) => {
            h4346
        }
    }, match v1.clone() {
        Adt3::C0(_, h4347, _, _, _, _, _, _) => {
            h4347
        }
    }, match v1.clone() {
        Adt3::C0(_, _, h4348, _, _, _, _, _) => {
            h4348
        }
    }, match v1.clone() {
        Adt3::C0(_, _, _, h4349, _, _, _, _) => {
            h4349
        }
    }, match v1.clone() {
        Adt3::C0(_, _, _, _, h4350, _, _, _) => {
            h4350
        }
    }, match v1.clone() {
        Adt3::C0(_, _, _, _, _, h4351, _, _) => {
            h4351
        }
    }, match v1.clone() {
        Adt3::C0(_, _, _, _, _, _, h4352, _) => {
            h4352
        }
    }, match v1 {
        Adt3::C0(_, _, _, _, _, _, _, h4353) => {
            h4353
        }
    })
}

pub fn f158(v0: u64, v1: Adt3, v2: Adt3, v3: Adt3, v4: u64) -> R<Adt9> {
    let v5: u64 = {
        let a4355 = v0;
        let a4356 = match v3.clone() {
            Adt3::C0(h4354, _, _, _, _, _, _, _) => {
                h4354
            }
        };
        let a4357 = 0u64;
        nat_quot(a4355, a4356, a4357)
    };
    let v6: u64 = {
        let a4359 = v5;
        let a4360 = match v3.clone() {
            Adt3::C0(_, h4358, _, _, _, _, _, _) => {
                h4358
            }
        };
        let a4361 = 0u64;
        nat_quot(a4359, a4360, a4361)
    };
    let v7: u64 = {
        let a4363 = v0;
        let a4364 = match v3.clone() {
            Adt3::C0(h4362, _, _, _, _, _, _, _) => {
                h4362
            }
        };
        let a4365 = 0u64;
        nat_rem(a4363, a4364, a4365)
    };
    let v8: u64 = {
        let a4367 = v5;
        let a4368 = match v3.clone() {
            Adt3::C0(_, h4366, _, _, _, _, _, _) => {
                h4366
            }
        };
        let a4369 = 0u64;
        nat_rem(a4367, a4368, a4369)
    };
    let v9: u64 = {
        let a4371 = v6;
        let a4372 = match v3.clone() {
            Adt3::C0(_, _, h4370, _, _, _, _, _) => {
                h4370
            }
        };
        let a4373 = 0u64;
        nat_rem(a4371, a4372, a4373)
    };
    let v10: u64 = {
        let a4375 = v6;
        let a4376 = match v3.clone() {
            Adt3::C0(_, _, h4374, _, _, _, _, _) => {
                h4374
            }
        };
        let a4377 = 0u64;
        nat_quot(a4375, a4376, a4377)
    };
    let v11: u64 = {
        let a4378 = v4;
        let a4379 = 0u64;
        if nat_eq(a4378, a4379) {
            match v1.clone() {
                Adt3::C0(h4380, _, _, _, _, _, _, _) => {
                    h4380
                }
            }
        } else {
            let a4381 = v4;
            let a4382 = 1u64;
            if nat_eq(a4381, a4382) {
                match v1.clone() {
                    Adt3::C0(_, h4383, _, _, _, _, _, _) => {
                        h4383
                    }
                }
            } else {
                let a4384 = v4;
                let a4385 = 2u64;
                if nat_eq(a4384, a4385) {
                    match v1.clone() {
                        Adt3::C0(_, _, h4386, _, _, _, _, _) => {
                            h4386
                        }
                    }
                } else {
                    match v1.clone() {
                        Adt3::C0(_, _, _, h4387, _, _, _, _) => {
                            h4387
                        }
                    }
                }
            }
        }
    };
    let v12: bool = {
        let a4394 = v11;
        let a4395 = {
            let a4388 = v4;
            let a4389 = 0u64;
            if nat_eq(a4388, a4389) {
                v7
            } else {
                let a4390 = v4;
                let a4391 = 1u64;
                if nat_eq(a4390, a4391) {
                    v8
                } else {
                    let a4392 = v4;
                    let a4393 = 2u64;
                    if nat_eq(a4392, a4393) {
                        v9
                    } else {
                        v10
                    }
                }
            }
        };
        nat_le(a4394, a4395)
    };
    let v13: Adt3 = if v12 {
        v2
    } else {
        v1
    };
    Ok(Adt9::C1(v12, f147(if if v12 {
        let a4396 = v4;
        let a4397 = 0u64;
        nat_eq(a4396, a4397)
    } else {
        false
    } {
        let a4398 = v7;
        let a4399 = v11;
        nat_sub(a4398, a4399)
    } else {
        v7
    }, if if v12 {
        let a4400 = v4;
        let a4401 = 1u64;
        nat_eq(a4400, a4401)
    } else {
        false
    } {
        let a4402 = v8;
        let a4403 = v11;
        nat_sub(a4402, a4403)
    } else {
        v8
    }, if if v12 {
        let a4404 = v4;
        let a4405 = 2u64;
        nat_eq(a4404, a4405)
    } else {
        false
    } {
        let a4406 = v9;
        let a4407 = v11;
        nat_sub(a4406, a4407)
    } else {
        v9
    }, if if v12 {
        let a4408 = v4;
        let a4409 = 3u64;
        nat_eq(a4408, a4409)
    } else {
        false
    } {
        let a4410 = v10;
        let a4411 = v11;
        nat_sub(a4410, a4411)
    } else {
        v10
    }, match v13.clone() {
        Adt3::C0(_, _, _, _, h4412, _, _, _) => {
            h4412
        }
    }, match v13.clone() {
        Adt3::C0(_, _, _, _, _, h4413, _, _) => {
            h4413
        }
    }, match v13.clone() {
        Adt3::C0(_, _, _, _, _, _, h4414, _) => {
            h4414
        }
    }, match v13 {
        Adt3::C0(_, _, _, _, _, _, _, h4415) => {
            h4415
        }
    })?, f155(v0, v3)?))
}

pub fn f159(v0: u64, v1: Adt3, v2: Adt3, v3: u64, v4: u64, v5: u64, v6: u64, v7: bool) -> R<Adt9> {
    let v8: u64 = f75(v0, v2.clone(), 0u64);
    let v9: u64 = f75(v0, v2.clone(), 1u64);
    let v10: u64 = f75(v0, v2.clone(), 2u64);
    let v11: u64 = f75(v0, v2.clone(), 3u64);
    if v7 {
        Ok(Adt9::C1(false, f74(v1.clone(), f218(v8, v3, match v1.clone() {
            Adt3::C0(h4416, _, _, _, _, _, _, _) => {
                h4416
            }
        })?, f218(v9, v4, match v1.clone() {
            Adt3::C0(_, h4417, _, _, _, _, _, _) => {
                h4417
            }
        })?, f218(v10, v5, match v1.clone() {
            Adt3::C0(_, _, h4418, _, _, _, _, _) => {
                h4418
            }
        })?, f218(v11, v6, match v1 {
            Adt3::C0(_, _, _, h4419, _, _, _, _) => {
                h4419
            }
        })?)?, f155(v0, v2)?))
    } else {
        if if if if f219(v8, v3, match v1.clone() {
            Adt3::C0(h4420, _, _, _, _, _, _, _) => {
                h4420
            }
        })? {
            f219(v9, v4, match v1.clone() {
                Adt3::C0(_, h4421, _, _, _, _, _, _) => {
                    h4421
                }
            })?
        } else {
            false
        } {
            f219(v10, v5, match v1.clone() {
                Adt3::C0(_, _, h4422, _, _, _, _, _) => {
                    h4422
                }
            })?
        } else {
            false
        } {
            f219(v11, v6, match v1.clone() {
                Adt3::C0(_, _, _, h4423, _, _, _, _) => {
                    h4423
                }
            })?
        } else {
            false
        } {
            Ok(Adt9::C1(false, f74(v1, f220(v8, v3)?, f220(v9, v4)?, f220(v10, v5)?, f220(v11, v6)?)?, f155(v0, v2)?))
        } else {
            Ok(Adt9::C1(true, 0u64, f155(v0, v2)?))
        }
    }
}

pub fn f160(v0: u64, v1: Adt3, v2: Adt3, v3: u64) -> R<Adt9> {
    let v4: u64 = f75(v0, v2.clone(), 0u64);
    let v5: u64 = f75(v0, v2.clone(), 1u64);
    let v6: u64 = f75(v0, v2.clone(), 2u64);
    let v7: u64 = f75(v0, v2.clone(), 3u64);
    Ok(Adt9::C1(false, f74(v1.clone(), f221(v4, v3, match v1 {
        Adt3::C0(h4424, _, _, _, _, _, _, _) => {
            h4424
        }
    })?, v5, v6, v7)?, f155(v0, v2)?))
}

pub fn f161(v0: u64, v1: Adt3, v2: Adt3, v3: u64, v4: u64, v5: bool) -> R<Adt9> {
    let a4425 = v0;
    let a4426 = v3;
    if nat_lt(a4425, a4426) {
        Ok(Adt9::C1(false, {
            let a4427 = v0;
            let a4428 = 4u64;
            nat_mul(a4427, a4428)?
        }, {
            let a4429 = v0;
            let a4430 = 4u64;
            nat_mul(a4429, a4430)?
        }))
    } else {
        let v6: u64 = {
            let a4431 = v0;
            let a4432 = v3;
            nat_sub(a4431, a4432)
        };
        let v7: u64 = f155(v6, v1)?;
        let v8: u64 = f222(v6, v2, v4)?;
        if v5 {
            Ok(Adt9::C4(8u64, 0u64, v8, 4u64, v7, 3u64, v8))
        } else {
            Ok(Adt9::C1(true, v7, v8))
        }
    }
}

pub fn f162(v0: u64, v1: Adt3, v2: Adt3, v3: u64) -> R<Adt9> {
    let a4433 = v0;
    let a4434 = v3;
    if nat_lt(a4433, a4434) {
        Ok(Adt9::C1(true, 0u64, f155(v0, v2)?))
    } else {
        let v4: u64 = {
            let a4435 = v0;
            let a4436 = v3;
            nat_sub(a4435, a4436)
        };
        let v5: u64 = f157(v4, v2, v1.clone())?;
        Ok(Adt9::C4(8u64, 3u64, v5, 0u64, f155(v4, v1)?, 3u64, v5))
    }
}

pub fn f163(v0: u64, v1: u64, v2: Adt3, v3: Adt3) -> R<Adt9> {
    let v4: u64 = {
        let a4438 = v1;
        let a4439 = match v3.clone() {
            Adt3::C0(h4437, _, _, _, _, _, _, _) => {
                h4437
            }
        };
        let a4440 = 0u64;
        nat_rem(a4438, a4439, a4440)
    };
    let v5: u64 = {
        let a4446 = {
            let a4442 = v1;
            let a4443 = match v3.clone() {
                Adt3::C0(h4441, _, _, _, _, _, _, _) => {
                    h4441
                }
            };
            let a4444 = 0u64;
            nat_quot(a4442, a4443, a4444)
        };
        let a4447 = match v3.clone() {
            Adt3::C0(_, h4445, _, _, _, _, _, _) => {
                h4445
            }
        };
        let a4448 = 0u64;
        nat_rem(a4446, a4447, a4448)
    };
    let v6: bool = f223(v0, v4, v5);
    Ok(Adt9::C1(!v6, if v6 {
        f157(v1, v2, v3.clone())?
    } else {
        0u64
    }, f155(v1, v3)?))
}

pub fn f164(v0: u64, v1: Adt3, v2: Adt3, v3: u64, v4: bool, v5: Adt3) -> R<Adt9> {
    Ok(Adt9::C4({
        let a4449 = v3;
        let a4450 = 8u64;
        nat_add(a4449, a4450)?
    }, 0u64, f155(v0, v1)?, if v4 {
        5u64
    } else {
        4u64
    }, if v4 {
        0u64
    } else {
        f157(v0, v5, v2.clone())?
    }, 3u64, f155(v0, v2)?))
}

pub fn f165(v0: u64, v1: Adt3, v2: Adt3, v3: u64, v4: u64) -> R<Adt9> {
    let a4451 = v4;
    let a4452 = 0u64;
    if if nat_eq(a4451, a4452) {
        f55(v3)
    } else {
        false
    } {
        let m4453 = f130(v3);
        match m4453 {
            Adt17::C0 => {
                Ok(Adt9::C0(Adt10::C1))
            }
            Adt17::C1(v5) => {
                let v6: u64 = match v5.clone() {
                    Adt18::C0(_, h4454, _) => {
                        h4454
                    }
                };
                Ok(Adt9::C5(v3, f155(v0, v1)?, match v5 {
                    Adt18::C0(_, _, h4455) => {
                        h4455
                    }
                }, f155({
                    let a4456 = v0;
                    let a4457 = v6;
                    nat_mul(a4456, a4457)?
                }, v2.clone())?, {
                    let a4463 = {
                        let a4461 = {
                            let a4458 = v6;
                            let a4459 = 1u64;
                            nat_sub(a4458, a4459)
                        };
                        let a4462 = match v2.clone() {
                            Adt3::C0(_, _, _, _, h4460, _, _, _) => {
                                h4460
                            }
                        };
                        nat_mul(a4461, a4462)?
                    };
                    let a4464 = 4u64;
                    nat_add(a4463, a4464)?
                }, match v2 {
                    Adt3::C0(_, _, _, _, h4465, _, _, _) => {
                        h4465
                    }
                }))
            }
        }
    } else {
        Ok(Adt9::C3(v3, v4, f155(v0, v1)?, f155(v0, v2)?))
    }
}

pub fn f166(v0: u64, v1: Adt3, v2: Adt3, v3: bool, v4: u64, v5: u64) -> R<Adt9> {
    let v6: u64 = {
        let a4467 = v0;
        let a4468 = match v2.clone() {
            Adt3::C0(h4466, _, _, _, _, _, _, _) => {
                h4466
            }
        };
        let a4469 = 0u64;
        nat_rem(a4467, a4468, a4469)
    };
    let v7: u64 = {
        let a4471 = v0;
        let a4472 = match v2.clone() {
            Adt3::C0(h4470, _, _, _, _, _, _, _) => {
                h4470
            }
        };
        let a4473 = 0u64;
        nat_quot(a4471, a4472, a4473)
    };
    let v8: u64 = {
        let a4475 = v7;
        let a4476 = match v2.clone() {
            Adt3::C0(_, h4474, _, _, _, _, _, _) => {
                h4474
            }
        };
        let a4477 = 0u64;
        nat_rem(a4475, a4476, a4477)
    };
    let v9: u64 = {
        let a4479 = v7;
        let a4480 = match v2.clone() {
            Adt3::C0(_, h4478, _, _, _, _, _, _) => {
                h4478
            }
        };
        let a4481 = 0u64;
        nat_quot(a4479, a4480, a4481)
    };
    let v10: u64 = {
        let a4483 = v9;
        let a4484 = match v2.clone() {
            Adt3::C0(_, _, h4482, _, _, _, _, _) => {
                h4482
            }
        };
        let a4485 = 0u64;
        nat_rem(a4483, a4484, a4485)
    };
    let v11: u64 = {
        let a4487 = v9;
        let a4488 = match v2.clone() {
            Adt3::C0(_, _, h4486, _, _, _, _, _) => {
                h4486
            }
        };
        let a4489 = 0u64;
        nat_quot(a4487, a4488, a4489)
    };
    let v12: u64 = f155(v0, v2)?;
    if v3 {
        let v13: u64 = {
            let a4498 = {
                let a4493 = {
                    let a4490 = v10;
                    let a4491 = v4;
                    let a4492 = 0u64;
                    nat_quot(a4490, a4491, a4492)
                };
                let a4494 = v5;
                nat_mul(a4493, a4494)?
            };
            let a4499 = {
                let a4495 = v8;
                let a4496 = v4;
                let a4497 = 0u64;
                nat_quot(a4495, a4496, a4497)
            };
            nat_add(a4498, a4499)?
        };
        let v14: u64 = {
            let a4519 = v6;
            let a4520 = {
                let a4517 = match v1.clone() {
                    Adt3::C0(h4500, _, _, _, _, _, _, _) => {
                        h4500
                    }
                };
                let a4518 = {
                    let a4515 = {
                        let a4501 = v8;
                        let a4502 = v4;
                        let a4503 = 0u64;
                        nat_rem(a4501, a4502, a4503)
                    };
                    let a4516 = {
                        let a4513 = match v1.clone() {
                            Adt3::C0(_, h4504, _, _, _, _, _, _) => {
                                h4504
                            }
                        };
                        let a4514 = {
                            let a4511 = {
                                let a4505 = v10;
                                let a4506 = v4;
                                let a4507 = 0u64;
                                nat_rem(a4505, a4506, a4507)
                            };
                            let a4512 = {
                                let a4509 = match v1.clone() {
                                    Adt3::C0(_, _, h4508, _, _, _, _, _) => {
                                        h4508
                                    }
                                };
                                let a4510 = v13;
                                nat_mul(a4509, a4510)?
                            };
                            nat_add(a4511, a4512)?
                        };
                        nat_mul(a4513, a4514)?
                    };
                    nat_add(a4515, a4516)?
                };
                nat_mul(a4517, a4518)?
            };
            nat_add(a4519, a4520)?
        };
        Ok(Adt9::C1(false, f155(v14, v1)?, v12))
    } else {
        let v15: u64 = {
            let a4526 = {
                let a4524 = {
                    let a4521 = v11;
                    let a4522 = v5;
                    let a4523 = 0u64;
                    nat_rem(a4521, a4522, a4523)
                };
                let a4525 = v4;
                nat_mul(a4524, a4525)?
            };
            let a4527 = v8;
            nat_add(a4526, a4527)?
        };
        let v16: u64 = {
            let a4533 = {
                let a4531 = {
                    let a4528 = v11;
                    let a4529 = v5;
                    let a4530 = 0u64;
                    nat_quot(a4528, a4529, a4530)
                };
                let a4532 = v4;
                nat_mul(a4531, a4532)?
            };
            let a4534 = v10;
            nat_add(a4533, a4534)?
        };
        let a4536 = v15;
        let a4537 = match v1.clone() {
            Adt3::C0(_, h4535, _, _, _, _, _, _) => {
                h4535
            }
        };
        if if nat_lt(a4536, a4537) {
            let a4539 = v16;
            let a4540 = match v1.clone() {
                Adt3::C0(_, _, h4538, _, _, _, _, _) => {
                    h4538
                }
            };
            nat_lt(a4539, a4540)
        } else {
            false
        } {
            Ok(Adt9::C1(false, f155({
                let a4549 = v6;
                let a4550 = {
                    let a4547 = match v1.clone() {
                        Adt3::C0(h4541, _, _, _, _, _, _, _) => {
                            h4541
                        }
                    };
                    let a4548 = {
                        let a4545 = v15;
                        let a4546 = {
                            let a4543 = match v1.clone() {
                                Adt3::C0(_, h4542, _, _, _, _, _, _) => {
                                    h4542
                                }
                            };
                            let a4544 = v16;
                            nat_mul(a4543, a4544)?
                        };
                        nat_add(a4545, a4546)?
                    };
                    nat_mul(a4547, a4548)?
                };
                nat_add(a4549, a4550)?
            }, v1)?, v12))
        } else {
            Ok(Adt9::C1(true, 0u64, v12))
        }
    }
}

pub fn f167(v0: u64, v1: Adt3, v2: Adt3, v3: u64, v4: u64) -> R<Adt9> {
    let v5: u64 = f146(23u64, v4);
    let v6: u64 = {
        let a4551 = v0;
        let a4552 = v5;
        let a4553 = 0u64;
        nat_rem(a4551, a4552, a4553)
    };
    let v7: u64 = {
        let a4554 = v0;
        let a4555 = v5;
        let a4556 = 0u64;
        nat_quot(a4554, a4555, a4556)
    };
    let v8: Adt13 = Adt13::C1(f155(v7, v2)?);
    let a4557 = v6;
    let a4558 = 0u64;
    if nat_eq(a4557, a4558) {
        Ok(Adt9::C7(Adt11::C7(16u64, v3, 0u64), Adt12::C0(0u64, f155(v7, v1)?), Adt12::C2(0u64), Adt13::C0(0u64), f173(v3), 4u64))
    } else {
        let a4559 = v6;
        let a4560 = 1u64;
        if nat_eq(a4559, a4560) {
            Ok(Adt9::C7(Adt11::C1(1u64), Adt12::C1(0u64), Adt12::C2(1065353216u64), {
                let a4561 = v4;
                let a4562 = 0u64;
                if nat_eq(a4561, a4562) {
                    v8
                } else {
                    Adt13::C0(0u64)
                }
            }, 4u64, 4u64))
        } else {
            Ok(Adt9::C7(Adt11::C2(0u64, v4, true), Adt12::C1(0u64), Adt12::C2(0u64), v8, 4u64, f173(v4)))
        }
    }
}

pub fn f168(v0: u64, v1: Adt3, v2: Adt3) -> R<Adt9> {
    let v3: u64 = f155(v0, v2.clone())?;
    let v4: u64 = f155(v0, v1.clone())?;
    let a4567 = {
        let a4564 = v0;
        let a4565 = match v1 {
            Adt3::C0(h4563, _, _, _, _, _, _, _) => {
                h4563
            }
        };
        let a4566 = 0u64;
        nat_rem(a4564, a4565, a4566)
    };
    let a4568 = 0u64;
    if nat_eq(a4567, a4568) {
        Ok(Adt9::C1(false, v4, v3))
    } else {
        Ok(Adt9::C4(8u64, 3u64, f155({
            let a4569 = v0;
            let a4570 = 1u64;
            nat_sub(a4569, a4570)
        }, v2)?, 0u64, v4, 3u64, v3))
    }
}

pub fn f169(v0: u64, v1: Adt3, v2: Adt3, v3: u64, v4: u64) -> R<Adt9> {
    let a4571 = v3;
    let a4572 = 13u64;
    if nat_eq(a4571, a4572) {
        f224(v0, v1, v4)
    } else {
        let v5: u64 = f214(match v1.clone() {
            Adt3::C0(h4573, _, _, _, _, _, _, _) => {
                h4573
            }
        }, {
            let a4574 = v3;
            let a4575 = 16u64;
            nat_eq(a4574, a4575)
        })?;
        let v6: u64 = {
            let a4576 = v0;
            let a4577 = v5;
            let a4578 = 0u64;
            nat_quot(a4576, a4577, a4578)
        };
        let v7: u64 = {
            let a4579 = v0;
            let a4580 = v5;
            let a4581 = 0u64;
            nat_rem(a4579, a4580, a4581)
        };
        let v8: u64 = {
            let a4585 = 1u64;
            let a4586 = {
                let a4583 = match v1.clone() {
                    Adt3::C0(h4582, _, _, _, _, _, _, _) => {
                        h4582
                    }
                };
                let a4584 = 2u64;
                nat_mul(a4583, a4584)?
            };
            nat_add(a4585, a4586)?
        };
        let a4587 = v7;
        let a4588 = v8;
        if nat_lt(a4587, a4588) {
            f225(v7, v6, v1, 0u64)
        } else {
            let v9: u64 = f155(v6, v2)?;
            let a4589 = v3;
            let a4590 = 14u64;
            if nat_eq(a4589, a4590) {
                Ok(Adt9::C4(2u64, 1u64, 1u64, 2u64, 0u64, 3u64, v9))
            } else {
                let a4591 = v7;
                let a4592 = v8;
                if nat_eq(a4591, a4592) {
                    Ok(Adt9::C4(2u64, 1u64, 1u64, 2u64, 0u64, 1u64, 1u64))
                } else {
                    let a4595 = v7;
                    let a4596 = {
                        let a4593 = v8;
                        let a4594 = 1u64;
                        nat_add(a4593, a4594)?
                    };
                    if nat_eq(a4595, a4596) {
                        Ok(Adt9::C4(3u64, 2u64, match v1 {
                            Adt3::C0(h4597, _, _, _, _, _, _, _) => {
                                h4597
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

pub fn f170(v0: u64, v1: Adt3, v2: Adt3, v3: u64) -> R<Adt9> {
    let a4598 = v0;
    let a4599 = 0u64;
    if nat_eq(a4598, a4599) {
        Ok(Adt9::C4(19u64, 2u64, 0u64, 2u64, 0u64, 1u64, 0u64))
    } else {
        let a4606 = {
            let a4600 = v0;
            let a4601 = 1u64;
            nat_add(a4600, a4601)?
        };
        let a4607 = {
            let a4604 = 2u64;
            let a4605 = {
                let a4602 = v3;
                let a4603 = 2u64;
                nat_mul(a4602, a4603)?
            };
            nat_add(a4604, a4605)?
        };
        if nat_eq(a4606, a4607) {
            Ok(Adt9::C4(19u64, 1u64, 0u64, 2u64, 0u64, 6u64, 0u64))
        } else {
            let v4: u64 = {
                let a4608 = v0;
                let a4609 = 1u64;
                nat_sub(a4608, a4609)
            };
            let v5: u64 = {
                let a4610 = v4;
                let a4611 = 2u64;
                let a4612 = 0u64;
                nat_quot(a4610, a4611, a4612)
            };
            let a4616 = {
                let a4613 = v4;
                let a4614 = 2u64;
                let a4615 = 0u64;
                nat_rem(a4613, a4614, a4615)
            };
            let a4617 = 0u64;
            if nat_eq(a4616, a4617) {
                Ok(Adt9::C4(17u64, 0u64, f155(v5, v1)?, 4u64, f155(v5, v2)?, 1u64, 2u64))
            } else {
                Ok(Adt9::C4(18u64, 1u64, 0u64, 1u64, 2u64, 1u64, 0u64))
            }
        }
    }
}

pub fn f171(v0: u64, v1: u64, v2: u64, v3: u64) -> R<Adt9> {
    let a4618 = v0;
    let a4619 = f27(v1, v2, v3)?;
    if nat_lt(a4618, a4619) {
        let a4620 = v0;
        let a4621 = v1;
        if nat_lt(a4620, a4621) {
            Ok(Adt9::C1(false, {
                let a4622 = 4u64;
                let a4623 = v0;
                nat_mul(a4622, a4623)?
            }, {
                let a4624 = 4u64;
                let a4625 = v0;
                nat_mul(a4624, a4625)?
            }))
        } else {
            let v4: u64 = {
                let a4626 = v0;
                let a4627 = v1;
                nat_sub(a4626, a4627)
            };
            let v5: u64 = f226(v4, v2)?;
            let a4628 = v5;
            let a4629 = 2u64;
            if nat_lt(a4628, a4629) {
                Ok(Adt9::C4(6u64, {
                    let a4630 = v5;
                    let a4631 = 0u64;
                    if nat_eq(a4630, a4631) {
                        4u64
                    } else {
                        5u64
                    }
                }, f227(v4, v2)?, 2u64, 0u64, 1u64, v5))
            } else {
                let v6: u64 = f228(v4, v2)?;
                Ok(Adt9::C4(8u64, 3u64, v6, 1u64, if f229(v4, v2)? {
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

pub fn f172(v0: u64, v1: u64) -> Adt11 {
    let a4632 = v0;
    let a4633 = 43u64;
    if nat_eq(a4632, a4633) {
        Adt11::C8(3u64)
    } else {
        let a4634 = v1;
        let a4635 = 43u64;
        if nat_eq(a4634, a4635) {
            Adt11::C8(4u64)
        } else {
            Adt11::C2(v0, v1, true)
        }
    }
}

pub fn f173(v0: u64) -> u64 {
    let m4636 = f130(v0);
    match m4636 {
        Adt17::C0 => {
            0u64
        }
        Adt17::C1(v1) => {
            let a4638 = match v1.clone() {
                Adt18::C0(_, h4637, _) => {
                    h4637
                }
            };
            let a4639 = 1u64;
            if if nat_eq(a4638, a4639) {
                let a4641 = match v1.clone() {
                    Adt18::C0(_, _, h4640) => {
                        h4640
                    }
                };
                let a4642 = 1u64;
                if if if nat_eq(a4641, a4642) {
                    true
                } else {
                    let a4644 = match v1.clone() {
                        Adt18::C0(_, _, h4643) => {
                            h4643
                        }
                    };
                    let a4645 = 2u64;
                    nat_eq(a4644, a4645)
                } {
                    true
                } else {
                    let a4647 = match v1.clone() {
                        Adt18::C0(_, _, h4646) => {
                            h4646
                        }
                    };
                    let a4648 = 4u64;
                    nat_eq(a4647, a4648)
                } {
                    true
                } else {
                    let a4650 = match v1.clone() {
                        Adt18::C0(_, _, h4649) => {
                            h4649
                        }
                    };
                    let a4651 = 8u64;
                    nat_eq(a4650, a4651)
                }
            } else {
                false
            } {
                match v1 {
                    Adt18::C0(_, _, h4652) => {
                        h4652
                    }
                }
            } else {
                0u64
            }
        }
    }
}

pub fn f174(v0: u64, v1: u64) -> R<u64> {
    if f120(v0) {
        let a4653 = 2u64;
        let a4654 = v1;
        nat_mul(a4653, a4654)
    } else {
        let a4655 = v1;
        let a4656 = f210(v0);
        let a4657 = 0u64;
        Ok(nat_quot(a4655, a4656, a4657))
    }
}

pub fn f175(v0: u64, v1: u64, v2: u64, v3: u64, v4: Adt3, v5: Adt3) -> R<Adt9> {
    let m4658 = f130(v0);
    match m4658 {
        Adt17::C0 => {
            Ok(Adt9::C0(Adt10::C1))
        }
        Adt17::C1(v6) => {
            Ok(Adt9::C10(v0, f147(v2, v1, 0u64, 0u64, match v4.clone() {
                Adt3::C0(_, _, _, _, h4659, _, _, _) => {
                    h4659
                }
            }, match v4.clone() {
                Adt3::C0(_, _, _, _, _, h4660, _, _) => {
                    h4660
                }
            }, match v4.clone() {
                Adt3::C0(_, _, _, _, _, _, h4661, _) => {
                    h4661
                }
            }, match v4 {
                Adt3::C0(_, _, _, _, _, _, _, h4662) => {
                    h4662
                }
            })?, match v6.clone() {
                Adt18::C0(_, _, h4663) => {
                    h4663
                }
            }, f147({
                let a4665 = v2;
                let a4666 = match v6.clone() {
                    Adt18::C0(_, h4664, _) => {
                        h4664
                    }
                };
                nat_mul(a4665, a4666)?
            }, v3, 0u64, 0u64, match v5.clone() {
                Adt3::C0(_, _, _, _, h4667, _, _, _) => {
                    h4667
                }
            }, match v5.clone() {
                Adt3::C0(_, _, _, _, _, h4668, _, _) => {
                    h4668
                }
            }, match v5.clone() {
                Adt3::C0(_, _, _, _, _, _, h4669, _) => {
                    h4669
                }
            }, match v5.clone() {
                Adt3::C0(_, _, _, _, _, _, _, h4670) => {
                    h4670
                }
            })?, match v6 {
                Adt18::C0(_, h4671, _) => {
                    h4671
                }
            }, match v5 {
                Adt3::C0(_, _, _, _, h4672, _, _, _) => {
                    h4672
                }
            }))
        }
    }
}

pub fn f176(v0: Adt3, v1: Adt3) -> R<u64> {
    let a4680 = {
        let a4673 = 2u64;
        let a4674 = f143(v1.clone())?;
        nat_mul(a4673, a4674)?
    };
    let a4681 = {
        let a4678 = f143(v1)?;
        let a4679 = {
            let a4676 = 1u64;
            let a4677 = match v0 {
                Adt3::C0(h4675, _, _, _, _, _, _, _) => {
                    h4675
                }
            };
            nat_add(a4676, a4677)?
        };
        nat_mul(a4678, a4679)?
    };
    nat_add(a4680, a4681)
}

pub fn f177(v0: u64, v1: Adt3) -> R<Adt9> {
    Ok(Adt9::C4(19u64, 4u64, f155(v0, v1)?, 2u64, 0u64, 1u64, 2u64))
}

pub fn f178(v0: u64, v1: u64) -> bool {
    let a4682 = v1;
    let a4683 = 2147483647u64;
    if nat_le(a4682, a4683) {
        let a4684 = v1;
        let a4685 = v0;
        nat_lt(a4684, a4685)
    } else {
        false
    }
}

pub fn f179(v0: u64, v1: u64, v2: u64, v3: Adt3, v4: Adt3, v5: Adt3, v6: u64) -> R<Adt9> {
    let m4686 = f130(v0);
    match m4686 {
        Adt17::C0 => {
            Ok(Adt9::C0(Adt10::C1))
        }
        Adt17::C1(v7) => {
            let v8: u64 = {
                let a4688 = v2;
                let a4689 = match v7.clone() {
                    Adt18::C0(_, h4687, _) => {
                        h4687
                    }
                };
                nat_mul(a4688, a4689)?
            };
            let m4702 = f230(v0, v8, v6, v1, match v5.clone() {
                Adt3::C0(h4690, _, _, _, _, _, _, _) => {
                    h4690
                }
            }, match v5.clone() {
                Adt3::C0(_, h4691, _, _, _, _, _, _) => {
                    h4691
                }
            }, match v5 {
                Adt3::C0(_, _, h4692, _, _, _, _, _) => {
                    h4692
                }
            }, {
                let a4695 = match v3.clone() {
                    Adt3::C0(h4693, _, _, _, _, _, _, _) => {
                        h4693
                    }
                };
                let a4696 = match v7.clone() {
                    Adt18::C0(_, h4694, _) => {
                        h4694
                    }
                };
                nat_mul(a4695, a4696)?
            }, match v3.clone() {
                Adt3::C0(_, h4697, _, _, _, _, _, _) => {
                    h4697
                }
            }, match v3.clone() {
                Adt3::C0(_, _, _, _, h4698, _, _, _) => {
                    h4698
                }
            }, match v3.clone() {
                Adt3::C0(_, _, _, _, _, h4699, _, _) => {
                    h4699
                }
            }, match v3.clone() {
                Adt3::C0(_, _, _, _, _, _, h4700, _) => {
                    h4700
                }
            }, match v3 {
                Adt3::C0(_, _, _, _, _, _, _, h4701) => {
                    h4701
                }
            })?;
            match m4702 {
                None => {
                    Ok(Adt9::C0(Adt10::C1))
                }
                Some(v9) => {
                    let a4703 = v0;
                    let a4704 = 0u64;
                    if if nat_eq(a4703, a4704) {
                        true
                    } else {
                        let a4705 = v0;
                        let a4706 = 26u64;
                        nat_eq(a4705, a4706)
                    } {
                        Ok(Adt9::C1(false, v9, f155({
                            let a4710 = {
                                let a4708 = v1;
                                let a4709 = match v4.clone() {
                                    Adt3::C0(h4707, _, _, _, _, _, _, _) => {
                                        h4707
                                    }
                                };
                                nat_mul(a4708, a4709)?
                            };
                            let a4711 = v8;
                            nat_add(a4710, a4711)?
                        }, v4)?))
                    } else {
                        Ok(Adt9::C5(v0, v9, match v7.clone() {
                            Adt18::C0(_, _, h4712) => {
                                h4712
                            }
                        }, f155({
                            let a4716 = {
                                let a4714 = v1;
                                let a4715 = match v4.clone() {
                                    Adt3::C0(h4713, _, _, _, _, _, _, _) => {
                                        h4713
                                    }
                                };
                                nat_mul(a4714, a4715)?
                            };
                            let a4717 = v8;
                            nat_add(a4716, a4717)?
                        }, v4.clone())?, {
                            let a4724 = {
                                let a4722 = {
                                    let a4719 = match v7 {
                                        Adt18::C0(_, h4718, _) => {
                                            h4718
                                        }
                                    };
                                    let a4720 = 1u64;
                                    nat_sub(a4719, a4720)
                                };
                                let a4723 = match v4.clone() {
                                    Adt3::C0(_, _, _, _, h4721, _, _, _) => {
                                        h4721
                                    }
                                };
                                nat_mul(a4722, a4723)?
                            };
                            let a4725 = 4u64;
                            nat_add(a4724, a4725)?
                        }, match v4 {
                            Adt3::C0(_, _, _, _, h4726, _, _, _) => {
                                h4726
                            }
                        }))
                    }
                }
            }
        }
    }
}

pub fn f180(v0: u64, v1: u64, v2: u64) -> u64 {
    let a4727 = v1;
    let a4728 = 0u64;
    if if nat_eq(a4727, a4728) {
        true
    } else {
        let a4729 = v2;
        let a4730 = 0u64;
        nat_eq(a4729, a4730)
    } {
        0u64
    } else {
        v0
    }
}

pub fn f181(v0: u64, v1: u64) -> bool {
    let a4731 = v0;
    let a4732 = 4294967295u64;
    if nat_eq(a4731, a4732) {
        true
    } else {
        let a4733 = v0;
        let a4734 = 2147483648u64;
        if nat_lt(a4733, a4734) {
            let a4735 = v0;
            let a4736 = v1;
            nat_lt(a4735, a4736)
        } else {
            false
        }
    }
}

pub fn f182(v0: Adt7) -> R<u64> {
    let a4741 = if match v0.clone() {
        Adt7::C0(_, _, _, h4737, _) => {
            h4737
        }
    } {
        2u64
    } else {
        1u64
    };
    let a4742 = {
        let a4739 = match v0 {
            Adt7::C0(_, _, _, _, h4738) => {
                h4738
            }
        };
        let a4740 = 1u64;
        if nat_eq(a4739, a4740) {
            1u64
        } else {
            0u64
        }
    };
    nat_add(a4741, a4742)
}

pub fn f183(v0: Adt6, v1: u64, v2: u64) -> R<u64> {
    let v3: u64 = {
        let a4747 = {
            let a4744 = v2;
            let a4745 = match v0.clone() {
                Adt6::C0(_, h4743, _, _) => {
                    h4743
                }
            };
            nat_mul(a4744, a4745)?
        };
        let a4748 = f148(match v0.clone() {
            Adt6::C0(_, _, h4746, _) => {
                h4746
            }
        });
        nat_add(a4747, a4748)?
    };
    let v4: u64 = f149(match v0.clone() {
        Adt6::C0(_, _, h4749, _) => {
            h4749
        }
    });
    let v5: u64 = {
        let a4753 = {
            let a4751 = v3;
            let a4752 = match v0 {
                Adt6::C0(h4750, _, _, _) => {
                    h4750
                }
            };
            nat_add(a4751, a4752)?
        };
        let a4754 = v4;
        nat_sub(a4753, a4754)
    };
    let a4759 = {
        let a4755 = v5;
        let a4756 = v1;
        if nat_lt(a4755, a4756) {
            v5
        } else {
            v1
        }
    };
    let a4760 = {
        let a4757 = v3;
        let a4758 = v4;
        nat_sub(a4757, a4758)
    };
    Ok(nat_sub(a4759, a4760))
}

pub fn f184(v0: Adt7, v1: Adt3, v2: Adt3, v3: u64, v4: u64) -> R<Option<u64>> {
    let m4769 = f77(match v0.clone() {
        Adt7::C0(h4761, _, _, _, _) => {
            h4761
        }
    }, match v1.clone() {
        Adt3::C0(h4762, _, _, _, _, _, _, _) => {
            h4762
        }
    }, f75(v3, v2.clone(), 0u64), {
        let a4766 = v4;
        let a4767 = {
            let m4765 = match v0.clone() {
                Adt7::C0(h4764, _, _, _, _) => {
                    h4764
                }
            };
            match m4765 {
                Adt6::C0(h4763, _, _, _) => {
                    h4763
                }
            }
        };
        let a4768 = 0u64;
        nat_rem(a4766, a4767, a4768)
    })?;
    match m4769 {
        None => {
            Ok(None::<u64>)
        }
        Some(v5) => {
            let m4778 = f77(match v0.clone() {
                Adt7::C0(_, h4770, _, _, _) => {
                    h4770
                }
            }, match v1.clone() {
                Adt3::C0(_, h4771, _, _, _, _, _, _) => {
                    h4771
                }
            }, f75(v3, v2.clone(), 1u64), {
                let a4775 = v4;
                let a4776 = {
                    let m4774 = match v0 {
                        Adt7::C0(h4773, _, _, _, _) => {
                            h4773
                        }
                    };
                    match m4774 {
                        Adt6::C0(h4772, _, _, _) => {
                            h4772
                        }
                    }
                };
                let a4777 = 0u64;
                nat_quot(a4775, a4776, a4777)
            })?;
            match m4778 {
                None => {
                    Ok(None::<u64>)
                }
                Some(v6) => {
                    Ok(Some(f74(v1, v5, v6, f75(v3, v2.clone(), 2u64), f75(v3, v2, 3u64))?))
                }
            }
        }
    }
}

pub fn f185(v0: Adt16, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64, v7: u64, v8: u64, v9: u64, v10: u64, v11: u64, v12: u64, v13: u64, v14: u64, v15: u64, v16: u64, v17: u64, v18: u64) -> R<Adt19> {
    let m4779 = v0;
    match m4779 {
        Adt16::C0(v19) => {
            f231(v1, v2, v3, v4, v5, v6, v7, v8, v9, v14, v15, v16, v17, v19, v18)
        }
        Adt16::C1(v20, v21, v22, v23) => {
            let a4780 = v20;
            let a4781 = 66u64;
            if nat_eq(a4780, a4781) {
                f232(v1, v2, v3, v4, v5, v6, v7, v8, v9, v10, v11, v12, v13, v14, v15, v16, v17, v21, v22, v23, v18)
            } else {
                f233(v20, v1, v2, v3, v4, v5, v6, v7, v8, v9, v14, v15, v16, v17, v21, v22, v23, v18)
            }
        }
    }
}

pub fn f186(v0: u64, v1: Adt3, v2: u64, v3: u64, v4: u64) -> R<Adt9> {
    let v5: u64 = {
        let a4788 = v3;
        let a4789 = {
            let a4785 = f155({
                let a4782 = v0;
                let a4783 = 2u64;
                let a4784 = 0u64;
                nat_quot(a4782, a4783, a4784)
            }, v1)?;
            let a4786 = v2;
            let a4787 = 0u64;
            nat_quot(a4785, a4786, a4787)
        };
        nat_add(a4788, a4789)?
    };
    let a4793 = {
        let a4790 = v0;
        let a4791 = 2u64;
        let a4792 = 0u64;
        nat_rem(a4790, a4791, a4792)
    };
    let a4794 = 0u64;
    if nat_eq(a4793, a4794) {
        Ok(Adt9::C4(19u64, 1u64, v5, 2u64, 0u64, 1u64, 2u64))
    } else {
        let a4795 = v4;
        let a4796 = 0u64;
        if nat_eq(a4795, a4796) {
            Ok(Adt9::C4(19u64, 2u64, 1u64, 2u64, 0u64, 1u64, v5))
        } else {
            Ok(Adt9::C0(Adt10::C1))
        }
    }
}

pub fn f187(v0: u64, v1: u64) -> R<u64> {
    let a4799 = 8u64;
    let a4800 = {
        let a4797 = f234(v0);
        let a4798 = v1;
        nat_mul(a4797, a4798)?
    };
    nat_add(a4799, a4800)
}

pub fn f188(v0: u64, v1: u64, v2: Adt3, v3: u64, v4: u64, v5: u64) -> R<Adt9> {
    let v6: u64 = match v2.clone() {
        Adt3::C0(h4801, _, _, _, _, _, _, _) => {
            h4801
        }
    };
    let v7: u64 = f234(v3);
    let v8: u64 = {
        let a4804 = 1u64;
        let a4805 = {
            let a4802 = v6;
            let a4803 = v7;
            nat_mul(a4802, a4803)?
        };
        nat_add(a4804, a4805)?
    };
    let a4806 = v0;
    let a4807 = 0u64;
    if nat_eq(a4806, a4807) {
        Ok(f235(0u64))
    } else {
        let a4808 = v0;
        let a4809 = v8;
        if nat_lt(a4808, a4809) {
            let v9: u64 = {
                let a4810 = v0;
                let a4811 = 1u64;
                nat_sub(a4810, a4811)
            };
            let v10: u64 = {
                let a4812 = v9;
                let a4813 = v7;
                let a4814 = 0u64;
                nat_rem(a4812, a4813, a4814)
            };
            let v11: u64 = f155({
                let a4820 = {
                    let a4815 = v1;
                    let a4816 = v6;
                    nat_mul(a4815, a4816)?
                };
                let a4821 = {
                    let a4817 = v9;
                    let a4818 = v7;
                    let a4819 = 0u64;
                    nat_quot(a4817, a4818, a4819)
                };
                nat_add(a4820, a4821)?
            }, v2)?;
            let a4822 = v3;
            let a4823 = 30u64;
            if nat_eq(a4822, a4823) {
                let a4824 = v10;
                let a4825 = 0u64;
                if nat_eq(a4824, a4825) {
                    Ok(Adt9::C4(6u64, 0u64, v11, 2u64, 0u64, 1u64, 1u64))
                } else {
                    let a4826 = v10;
                    let a4827 = 1u64;
                    if nat_eq(a4826, a4827) {
                        Ok(Adt9::C4(10u64, 1u64, 1u64, 1u64, 1u64, 1u64, 1u64))
                    } else {
                        let a4828 = v10;
                        let a4829 = 2u64;
                        if nat_eq(a4828, a4829) {
                            Ok(Adt9::C4(1u64, 1u64, 1u64, 2u64, 0u64, 1u64, 1u64))
                        } else {
                            Ok(Adt9::C4(0u64, 1u64, 0u64, 1u64, 1u64, 1u64, 0u64))
                        }
                    }
                }
            } else {
                let a4830 = v10;
                let a4831 = 0u64;
                if nat_eq(a4830, a4831) {
                    Ok(Adt9::C4(10u64, 0u64, v11, 0u64, v11, 1u64, 1u64))
                } else {
                    let a4832 = v10;
                    let a4833 = 1u64;
                    if nat_eq(a4832, a4833) {
                        Ok(Adt9::C4(1u64, 1u64, 1u64, 2u64, 0u64, 1u64, 1u64))
                    } else {
                        Ok(Adt9::C4(0u64, 1u64, 0u64, 1u64, 1u64, 1u64, 0u64))
                    }
                }
            }
        } else {
            let v12: u64 = {
                let a4834 = v0;
                let a4835 = v8;
                nat_sub(a4834, a4835)
            };
            let a4836 = v12;
            let a4837 = 0u64;
            if nat_eq(a4836, a4837) {
                Ok(Adt9::C4(22u64, 2u64, v6, 2u64, 0u64, 1u64, 1u64))
            } else {
                let a4838 = v12;
                let a4839 = 1u64;
                if nat_eq(a4838, a4839) {
                    Ok(Adt9::C4(21u64, 1u64, 0u64, 1u64, 1u64, 1u64, 1u64))
                } else {
                    let a4840 = v12;
                    let a4841 = 2u64;
                    if nat_eq(a4840, a4841) {
                        Ok(Adt9::C4(2u64, 1u64, 1u64, 2u64, 0u64, 1u64, 1u64))
                    } else {
                        let a4842 = v12;
                        let a4843 = 3u64;
                        if nat_eq(a4842, a4843) {
                            Ok(Adt9::C4(4u64, 1u64, 1u64, 2u64, v4, 1u64, 1u64))
                        } else {
                            let a4844 = v12;
                            let a4845 = 4u64;
                            if nat_eq(a4844, a4845) {
                                Ok(Adt9::C4(12u64, 1u64, 1u64, 2u64, 0u64, 1u64, 1u64))
                            } else {
                                let a4846 = v12;
                                let a4847 = 5u64;
                                if nat_eq(a4846, a4847) {
                                    Ok(Adt9::C4(11u64, 2u64, 1065353216u64, 1u64, 1u64, 1u64, 2u64))
                                } else {
                                    let a4848 = v12;
                                    let a4849 = 6u64;
                                    if nat_eq(a4848, a4849) {
                                        if f236(v5) {
                                            Ok(Adt9::C4(19u64, 1u64, 2u64, 2u64, 0u64, 1u64, {
                                                let a4850 = 3u64;
                                                let a4851 = v1;
                                                nat_add(a4850, a4851)?
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

pub fn f189(v0: u64, v1: Adt3, v2: Adt3, v3: u64) -> R<Adt9> {
    let v4: u64 = {
        let a4852 = v0;
        let a4853 = f112(v3);
        let a4854 = 0u64;
        nat_quot(a4852, a4853, a4854)
    };
    let v5: u64 = {
        let a4859 = 3u64;
        let a4860 = {
            let a4856 = v4;
            let a4857 = match v1.clone() {
                Adt3::C0(h4855, _, _, _, _, _, _, _) => {
                    h4855
                }
            };
            let a4858 = 0u64;
            nat_quot(a4856, a4857, a4858)
        };
        nat_add(a4859, a4860)?
    };
    let v6: u64 = f155(v4, v1)?;
    let v7: u64 = f155(v4, v2)?;
    let a4861 = v3;
    let a4862 = 30u64;
    if nat_eq(a4861, a4862) {
        let a4866 = {
            let a4863 = v0;
            let a4864 = 3u64;
            let a4865 = 0u64;
            nat_rem(a4863, a4864, a4865)
        };
        let a4867 = 0u64;
        if nat_eq(a4866, a4867) {
            Ok(Adt9::C4(6u64, 0u64, v6, 2u64, 0u64, 1u64, 1u64))
        } else {
            let a4871 = {
                let a4868 = v0;
                let a4869 = 3u64;
                let a4870 = 0u64;
                nat_rem(a4868, a4869, a4870)
            };
            let a4872 = 1u64;
            if nat_eq(a4871, a4872) {
                Ok(Adt9::C4(10u64, 1u64, 1u64, 1u64, v5, 1u64, 1u64))
            } else {
                Ok(Adt9::C4(7u64, 1u64, 1u64, 2u64, 0u64, 3u64, v7))
            }
        }
    } else {
        Ok(Adt9::C4(10u64, 0u64, v6, 1u64, v5, 3u64, v7))
    }
}

pub fn f190(v0: u64) -> R<u64> {
    let v1: u64 = {
        let a4873 = v0;
        let a4874 = 8388608u64;
        let a4875 = 0u64;
        nat_quot(a4873, a4874, a4875)
    };
    let a4876 = v0;
    let a4877 = 2147483648u64;
    if if if nat_lt(a4876, a4877) {
        let a4878 = 127u64;
        let a4879 = v1;
        nat_le(a4878, a4879)
    } else {
        false
    } {
        let a4880 = v1;
        let a4881 = 190u64;
        nat_lt(a4880, a4881)
    } else {
        false
    } {
        let v2: u64 = {
            let a4885 = 8388608u64;
            let a4886 = {
                let a4882 = v0;
                let a4883 = 8388608u64;
                let a4884 = 0u64;
                nat_rem(a4882, a4883, a4884)
            };
            nat_add(a4885, a4886)?
        };
        let a4887 = v1;
        let a4888 = 150u64;
        if nat_lt(a4887, a4888) {
            let v3: u64 = f237({
                let a4889 = 150u64;
                let a4890 = v1;
                nat_sub(a4889, a4890)
            });
            let a4894 = {
                let a4891 = v2;
                let a4892 = v3;
                let a4893 = 0u64;
                nat_rem(a4891, a4892, a4893)
            };
            let a4895 = 0u64;
            if nat_eq(a4894, a4895) {
                let a4896 = v2;
                let a4897 = v3;
                let a4898 = 0u64;
                Ok(nat_quot(a4896, a4897, a4898))
            } else {
                Ok(0u64)
            }
        } else {
            let a4901 = f237({
                let a4899 = v1;
                let a4900 = 150u64;
                nat_sub(a4899, a4900)
            });
            let a4902 = v2;
            nat_mul(a4901, a4902)
        }
    } else {
        Ok(0u64)
    }
}

pub fn f191(v0: Adt14, v1: Adt14) -> bool {
    let a4905 = match v0.clone() {
        Adt14::C0(_, h4903, _, _, _, _, _, _, _, _, _) => {
            h4903
        }
    };
    let a4906 = match v1.clone() {
        Adt14::C0(_, h4904, _, _, _, _, _, _, _, _, _) => {
            h4904
        }
    };
    if if if nat_eq(a4905, a4906) {
        let a4909 = match v0.clone() {
            Adt14::C0(_, _, h4907, _, _, _, _, _, _, _, _) => {
                h4907
            }
        };
        let a4910 = match v1.clone() {
            Adt14::C0(_, _, h4908, _, _, _, _, _, _, _, _) => {
                h4908
            }
        };
        nat_eq(a4909, a4910)
    } else {
        false
    } {
        let a4913 = match v0.clone() {
            Adt14::C0(_, _, _, h4911, _, _, _, _, _, _, _) => {
                h4911
            }
        };
        let a4914 = match v1.clone() {
            Adt14::C0(_, _, _, h4912, _, _, _, _, _, _, _) => {
                h4912
            }
        };
        nat_eq(a4913, a4914)
    } else {
        false
    } {
        let a4917 = match v0 {
            Adt14::C0(_, _, _, _, h4915, _, _, _, _, _, _) => {
                h4915
            }
        };
        let a4918 = match v1 {
            Adt14::C0(_, _, _, _, h4916, _, _, _, _, _, _) => {
                h4916
            }
        };
        nat_eq(a4917, a4918)
    } else {
        false
    }
}

pub fn f192(v0: u64, v1: u64, v2: u64, v3: u64) -> bool {
    let a4919 = v0;
    let a4920 = 0u64;
    if if if nat_eq(a4919, a4920) {
        true
    } else {
        let a4921 = v1;
        let a4922 = 0u64;
        nat_eq(a4921, a4922)
    } {
        true
    } else {
        let a4923 = v2;
        let a4924 = 0u64;
        nat_eq(a4923, a4924)
    } {
        true
    } else {
        let a4925 = v3;
        let a4926 = 0u64;
        nat_eq(a4925, a4926)
    }
}

pub fn f193(v0: Adt14, v1: Adt14) -> bool {
    if f199(match v0.clone() {
        Adt14::C0(_, _, _, _, _, h4927, _, _, _, _, _) => {
            h4927
        }
    }, match v1 {
        Adt14::C0(_, _, _, _, _, h4928, _, _, _, _, _) => {
            h4928
        }
    }) {
        let a4930 = match v0 {
            Adt14::C0(_, h4929, _, _, _, _, _, _, _, _, _) => {
                h4929
            }
        };
        let a4931 = 2147483647u64;
        nat_le(a4930, a4931)
    } else {
        true
    }
}

pub fn f194(v0: Adt14) -> bool {
    let a4936 = {
        let a4933 = match v0.clone() {
            Adt14::C0(_, _, _, _, _, _, _, _, _, _, h4932) => {
                h4932
            }
        };
        let a4934 = 4u64;
        let a4935 = 0u64;
        nat_rem(a4933, a4934, a4935)
    };
    let a4937 = 0u64;
    if if nat_eq(a4936, a4937) {
        f238(match v0.clone() {
            Adt14::C0(_, h4938, _, _, _, _, _, _, _, _, _) => {
                h4938
            }
        }, match v0.clone() {
            Adt14::C0(_, _, _, _, _, h4939, _, _, _, _, _) => {
                h4939
            }
        })
    } else {
        false
    } {
        f108(match v0.clone() {
            Adt14::C0(_, _, h4940, _, _, _, _, _, _, _, _) => {
                h4940
            }
        }, match v0.clone() {
            Adt14::C0(_, _, _, h4941, _, _, _, _, _, _, _) => {
                h4941
            }
        }, match v0.clone() {
            Adt14::C0(_, _, _, _, h4942, _, _, _, _, _, _) => {
                h4942
            }
        }, match v0.clone() {
            Adt14::C0(_, _, _, _, _, _, h4943, _, _, _, _) => {
                h4943
            }
        }, match v0.clone() {
            Adt14::C0(_, _, _, _, _, _, _, h4944, _, _, _) => {
                h4944
            }
        }, match v0 {
            Adt14::C0(_, _, _, _, _, _, _, _, h4945, _, _) => {
                h4945
            }
        }, 4u64)
    } else {
        false
    }
}

pub fn f195(v0: Adt14, v1: Adt14) -> R<bool> {
    let a4947 = match v0.clone() {
        Adt14::C0(_, _, _, _, _, _, _, _, _, h4946, _) => {
            h4946
        }
    };
    let a4948 = 0u64;
    if nat_eq(a4947, a4948) {
        Ok(true)
    } else {
        let a4950 = match v1.clone() {
            Adt14::C0(_, _, _, _, _, _, _, _, _, h4949, _) => {
                h4949
            }
        };
        let a4951 = 0u64;
        if nat_eq(a4950, a4951) {
            Ok(true)
        } else {
            let a4953 = match v0.clone() {
                Adt14::C0(_, _, _, _, _, _, _, _, _, _, h4952) => {
                    h4952
                }
            };
            let a4954 = 0u64;
            if nat_eq(a4953, a4954) {
                Ok(true)
            } else {
                let a4956 = match v1.clone() {
                    Adt14::C0(_, _, _, _, _, _, _, _, _, _, h4955) => {
                        h4955
                    }
                };
                let a4957 = 0u64;
                if nat_eq(a4956, a4957) {
                    Ok(true)
                } else {
                    let a4963 = {
                        let a4960 = match v0.clone() {
                            Adt14::C0(_, _, _, _, _, _, _, _, _, _, h4958) => {
                                h4958
                            }
                        };
                        let a4961 = match v0.clone() {
                            Adt14::C0(_, _, _, _, _, _, _, _, _, h4959, _) => {
                                h4959
                            }
                        };
                        nat_add(a4960, a4961)?
                    };
                    let a4964 = match v1.clone() {
                        Adt14::C0(_, _, _, _, _, _, _, _, _, _, h4962) => {
                            h4962
                        }
                    };
                    if nat_le(a4963, a4964) {
                        Ok(true)
                    } else {
                        let a4970 = {
                            let a4967 = match v1.clone() {
                                Adt14::C0(_, _, _, _, _, _, _, _, _, _, h4965) => {
                                    h4965
                                }
                            };
                            let a4968 = match v1 {
                                Adt14::C0(_, _, _, _, _, _, _, _, _, h4966, _) => {
                                    h4966
                                }
                            };
                            nat_add(a4967, a4968)?
                        };
                        let a4971 = match v0 {
                            Adt14::C0(_, _, _, _, _, _, _, _, _, _, h4969) => {
                                h4969
                            }
                        };
                        Ok(nat_le(a4970, a4971))
                    }
                }
            }
        }
    }
}

pub fn f196(v0: Adt14, v1: Adt14) -> bool {
    if if if if if if f191(v0.clone(), v1.clone()) {
        let a4974 = match v0.clone() {
            Adt14::C0(h4972, _, _, _, _, _, _, _, _, _, _) => {
                h4972
            }
        };
        let a4975 = match v1.clone() {
            Adt14::C0(h4973, _, _, _, _, _, _, _, _, _, _) => {
                h4973
            }
        };
        nat_eq(a4974, a4975)
    } else {
        false
    } {
        let a4978 = match v0.clone() {
            Adt14::C0(_, _, _, _, _, _, _, _, _, _, h4976) => {
                h4976
            }
        };
        let a4979 = match v1.clone() {
            Adt14::C0(_, _, _, _, _, _, _, _, _, _, h4977) => {
                h4977
            }
        };
        nat_eq(a4978, a4979)
    } else {
        false
    } {
        let a4982 = match v0.clone() {
            Adt14::C0(_, _, _, _, _, h4980, _, _, _, _, _) => {
                h4980
            }
        };
        let a4983 = match v1.clone() {
            Adt14::C0(_, _, _, _, _, h4981, _, _, _, _, _) => {
                h4981
            }
        };
        nat_eq(a4982, a4983)
    } else {
        false
    } {
        let a4986 = match v0.clone() {
            Adt14::C0(_, _, _, _, _, _, h4984, _, _, _, _) => {
                h4984
            }
        };
        let a4987 = match v1.clone() {
            Adt14::C0(_, _, _, _, _, _, h4985, _, _, _, _) => {
                h4985
            }
        };
        nat_eq(a4986, a4987)
    } else {
        false
    } {
        let a4990 = match v0.clone() {
            Adt14::C0(_, _, _, _, _, _, _, h4988, _, _, _) => {
                h4988
            }
        };
        let a4991 = match v1.clone() {
            Adt14::C0(_, _, _, _, _, _, _, h4989, _, _, _) => {
                h4989
            }
        };
        nat_eq(a4990, a4991)
    } else {
        false
    } {
        let a4994 = match v0 {
            Adt14::C0(_, _, _, _, _, _, _, _, h4992, _, _) => {
                h4992
            }
        };
        let a4995 = match v1 {
            Adt14::C0(_, _, _, _, _, _, _, _, h4993, _, _) => {
                h4993
            }
        };
        nat_eq(a4994, a4995)
    } else {
        false
    }
}

pub fn f197(v0: Adt14) -> R<bool> {
    if f239(match v0.clone() {
        Adt14::C0(_, h4996, _, _, _, _, _, _, _, _, _) => {
            h4996
        }
    }, match v0.clone() {
        Adt14::C0(_, _, _, _, _, h4997, _, _, _, _, _) => {
            h4997
        }
    }) {
        f109(f240(match v0.clone() {
            Adt14::C0(_, h4998, _, _, _, _, _, _, _, _, _) => {
                h4998
            }
        }, match v0.clone() {
            Adt14::C0(_, _, _, _, _, h4999, _, _, _, _, _) => {
                h4999
            }
        })?, match v0.clone() {
            Adt14::C0(_, _, h5000, _, _, _, _, _, _, _, _) => {
                h5000
            }
        }, match v0.clone() {
            Adt14::C0(_, _, _, h5001, _, _, _, _, _, _, _) => {
                h5001
            }
        }, match v0.clone() {
            Adt14::C0(_, _, _, _, h5002, _, _, _, _, _, _) => {
                h5002
            }
        }, match v0.clone() {
            Adt14::C0(_, _, _, _, _, _, h5003, _, _, _, _) => {
                h5003
            }
        }, match v0.clone() {
            Adt14::C0(_, _, _, _, _, _, _, h5004, _, _, _) => {
                h5004
            }
        }, match v0 {
            Adt14::C0(_, _, _, _, _, _, _, _, h5005, _, _) => {
                h5005
            }
        })
    } else {
        Ok(false)
    }
}

pub fn f198(v0: bool, v1: u64, v2: u64, v3: u64) -> R<u64> {
    let a5006 = v1;
    let a5007 = 0u64;
    if if nat_eq(a5006, a5007) {
        true
    } else {
        let a5008 = v2;
        let a5009 = 0u64;
        nat_eq(a5008, a5009)
    } {
        Ok(0u64)
    } else {
        let a5012 = f110(v1, v2, v3)?;
        let a5013 = {
            let a5010 = v2;
            let a5011 = f241(v0, v1)?;
            nat_mul(a5010, a5011)?
        };
        nat_add(a5012, a5013)
    }
}

pub fn f199(v0: u64, v1: u64) -> bool {
    let a5014 = v0;
    let a5015 = 4u64;
    if nat_eq(a5014, a5015) {
        let a5016 = v1;
        let a5017 = 4u64;
        nat_eq(a5016, a5017)
    } else {
        false
    }
}

pub fn f200(v0: u64) -> R<u64> {
    let a5018 = 6u64;
    let a5019 = v0;
    nat_add(a5018, a5019)
}

pub fn f201(v0: Adt14) -> Adt14 {
    Adt14::C0(match v0.clone() {
        Adt14::C0(h5020, _, _, _, _, _, _, _, _, _, _) => {
            h5020
        }
    }, match v0.clone() {
        Adt14::C0(_, h5021, _, _, _, _, _, _, _, _, _) => {
            h5021
        }
    }, match v0.clone() {
        Adt14::C0(_, _, h5022, _, _, _, _, _, _, _, _) => {
            h5022
        }
    }, match v0.clone() {
        Adt14::C0(_, _, _, h5023, _, _, _, _, _, _, _) => {
            h5023
        }
    }, match v0.clone() {
        Adt14::C0(_, _, _, _, h5024, _, _, _, _, _, _) => {
            h5024
        }
    }, 4u64, match v0.clone() {
        Adt14::C0(_, _, _, _, _, _, h5025, _, _, _, _) => {
            h5025
        }
    }, match v0.clone() {
        Adt14::C0(_, _, _, _, _, _, _, h5026, _, _, _) => {
            h5026
        }
    }, match v0.clone() {
        Adt14::C0(_, _, _, _, _, _, _, _, h5027, _, _) => {
            h5027
        }
    }, match v0.clone() {
        Adt14::C0(_, _, _, _, _, _, _, _, _, h5028, _) => {
            h5028
        }
    }, match v0 {
        Adt14::C0(_, _, _, _, _, _, _, _, _, _, h5029) => {
            h5029
        }
    })
}

pub fn f202(v0: u64, v1: u64, v2: u64, v3: u64) -> R<bool> {
    let a5030 = v0;
    let a5031 = 2147483647u64;
    if if if nat_le(a5030, a5031) {
        let a5032 = 0u64;
        let a5033 = v3;
        if nat_lt(a5032, a5033) {
            let a5034 = v3;
            let a5035 = 2147483647u64;
            nat_le(a5034, a5035)
        } else {
            false
        }
    } else {
        false
    } {
        let a5040 = {
            let a5038 = {
                let a5036 = v2;
                let a5037 = v3;
                nat_add(a5036, a5037)?
            };
            let a5039 = 1u64;
            nat_sub(a5038, a5039)
        };
        let a5041 = 2147483647u64;
        nat_le(a5040, a5041)
    } else {
        false
    } {
        let a5046 = {
            let a5044 = {
                let a5042 = v0;
                let a5043 = v1;
                nat_mul(a5042, a5043)?
            };
            let a5045 = f242(v2, v3)?;
            nat_mul(a5044, a5045)?
        };
        let a5047 = 9223372036854775807u64;
        Ok(nat_le(a5046, a5047))
    } else {
        Ok(false)
    }
}

pub fn f203(v0: Adt14) -> R<u64> {
    let a5078 = {
        let a5070 = {
            let a5062 = {
                let a5054 = 4u64;
                let a5055 = {
                    let a5052 = {
                        let a5049 = match v0.clone() {
                            Adt14::C0(_, h5048, _, _, _, _, _, _, _, _, _) => {
                                h5048
                            }
                        };
                        let a5050 = 1u64;
                        nat_sub(a5049, a5050)
                    };
                    let a5053 = match v0.clone() {
                        Adt14::C0(_, _, _, _, _, h5051, _, _, _, _, _) => {
                            h5051
                        }
                    };
                    nat_mul(a5052, a5053)?
                };
                nat_add(a5054, a5055)?
            };
            let a5063 = {
                let a5060 = {
                    let a5057 = match v0.clone() {
                        Adt14::C0(_, _, h5056, _, _, _, _, _, _, _, _) => {
                            h5056
                        }
                    };
                    let a5058 = 1u64;
                    nat_sub(a5057, a5058)
                };
                let a5061 = match v0.clone() {
                    Adt14::C0(_, _, _, _, _, _, h5059, _, _, _, _) => {
                        h5059
                    }
                };
                nat_mul(a5060, a5061)?
            };
            nat_add(a5062, a5063)?
        };
        let a5071 = {
            let a5068 = {
                let a5065 = match v0.clone() {
                    Adt14::C0(_, _, _, h5064, _, _, _, _, _, _, _) => {
                        h5064
                    }
                };
                let a5066 = 1u64;
                nat_sub(a5065, a5066)
            };
            let a5069 = match v0.clone() {
                Adt14::C0(_, _, _, _, _, _, _, h5067, _, _, _) => {
                    h5067
                }
            };
            nat_mul(a5068, a5069)?
        };
        nat_add(a5070, a5071)?
    };
    let a5079 = {
        let a5076 = {
            let a5073 = match v0.clone() {
                Adt14::C0(_, _, _, _, h5072, _, _, _, _, _, _) => {
                    h5072
                }
            };
            let a5074 = 1u64;
            nat_sub(a5073, a5074)
        };
        let a5077 = match v0 {
            Adt14::C0(_, _, _, _, _, _, _, _, h5075, _, _) => {
                h5075
            }
        };
        nat_mul(a5076, a5077)?
    };
    nat_add(a5078, a5079)
}

pub fn f204(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64) -> R<u64> {
    let a5080 = v0;
    let a5081 = 0u64;
    if if if if nat_eq(a5080, a5081) {
        true
    } else {
        let a5082 = v1;
        let a5083 = 0u64;
        nat_eq(a5082, a5083)
    } {
        true
    } else {
        let a5084 = v2;
        let a5085 = 0u64;
        nat_eq(a5084, a5085)
    } {
        true
    } else {
        let a5086 = v3;
        let a5087 = 0u64;
        nat_eq(a5086, a5087)
    } {
        Ok(0u64)
    } else {
        let a5088 = f243(v0, v1, v2, v3, v5)?;
        let a5089 = f244(v0, v1, v2, v3, v4)?;
        nat_add(a5088, a5089)
    }
}

pub fn f205(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let a5090 = v0;
    let a5091 = 0u64;
    if if nat_eq(a5090, a5091) {
        true
    } else {
        let a5092 = v1;
        let a5093 = 0u64;
        nat_eq(a5092, a5093)
    } {
        Ok(0u64)
    } else {
        let a5096 = f110(v0, v1, v2)?;
        let a5097 = {
            let a5094 = v1;
            let a5095 = f245(v0)?;
            nat_mul(a5094, a5095)?
        };
        nat_add(a5096, a5097)
    }
}

pub fn f206(v0: u64, v1: u64, v2: u64) -> bool {
    let a5098 = v0;
    let a5099 = 1u64;
    if nat_le(a5098, a5099) {
        true
    } else {
        let a5103 = {
            let a5100 = v1;
            let a5101 = v2;
            let a5102 = 1u64;
            nat_rem(a5100, a5101, a5102)
        };
        let a5104 = 0u64;
        nat_eq(a5103, a5104)
    }
}

pub fn f207(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64) -> R<bool> {
    let a5105 = v1;
    let a5106 = 1u64;
    if if if nat_le(a5105, a5106) {
        true
    } else {
        let a5107 = v0;
        let a5108 = v2;
        nat_le(a5107, a5108)
    } {
        let a5109 = v3;
        let a5110 = 1u64;
        if nat_le(a5109, a5110) {
            true
        } else {
            let a5117 = {
                let a5115 = v0;
                let a5116 = {
                    let a5113 = {
                        let a5111 = v1;
                        let a5112 = 1u64;
                        nat_sub(a5111, a5112)
                    };
                    let a5114 = v2;
                    nat_mul(a5113, a5114)?
                };
                nat_add(a5115, a5116)?
            };
            let a5118 = v4;
            nat_le(a5117, a5118)
        }
    } else {
        false
    } {
        let a5119 = v5;
        let a5120 = 1u64;
        if nat_le(a5119, a5120) {
            Ok(true)
        } else {
            let a5133 = {
                let a5131 = {
                    let a5125 = v0;
                    let a5126 = {
                        let a5123 = {
                            let a5121 = v1;
                            let a5122 = 1u64;
                            nat_sub(a5121, a5122)
                        };
                        let a5124 = v2;
                        nat_mul(a5123, a5124)?
                    };
                    nat_add(a5125, a5126)?
                };
                let a5132 = {
                    let a5129 = {
                        let a5127 = v3;
                        let a5128 = 1u64;
                        nat_sub(a5127, a5128)
                    };
                    let a5130 = v4;
                    nat_mul(a5129, a5130)?
                };
                nat_add(a5131, a5132)?
            };
            let a5134 = v6;
            Ok(nat_le(a5133, a5134))
        }
    } else {
        Ok(false)
    }
}

pub fn f208(v0: u64, v1: u64, _: u64, v3: u64, v4: u64, v5: u64, v6: u64, v7: u64, v8: u64, v9: u64, v10: u64, v11: u64, v12: u64, v13: u64, v14: u64) -> bool {
    if if if if if if if if if f246(v0, v1, v6, v7, v8) {
        let a5135 = v5;
        let a5136 = 26u64;
        nat_eq(a5135, a5136)
    } else {
        false
    } {
        let a5137 = v3;
        let a5138 = v7;
        nat_eq(a5137, a5138)
    } else {
        false
    } {
        let a5139 = v4;
        let a5140 = v8;
        nat_eq(a5139, a5140)
    } else {
        false
    } {
        let a5141 = v9;
        let a5142 = 1u64;
        nat_eq(a5141, a5142)
    } else {
        false
    } {
        let a5143 = v10;
        let a5144 = f247(v0);
        nat_eq(a5143, a5144)
    } else {
        false
    } {
        let a5145 = v11;
        let a5146 = v1;
        nat_eq(a5145, a5146)
    } else {
        false
    } {
        let a5147 = v12;
        let a5148 = v6;
        nat_eq(a5147, a5148)
    } else {
        false
    } {
        let a5149 = v13;
        let a5150 = v7;
        nat_eq(a5149, a5150)
    } else {
        false
    } {
        let a5151 = v14;
        let a5152 = v8;
        nat_eq(a5151, a5152)
    } else {
        false
    }
}

pub fn f209(v0: u64, v1: u64, v2: u64, v3: u64) -> bool {
    let a5153 = v0;
    let a5154 = 0u64;
    if if if nat_eq(a5153, a5154) {
        true
    } else {
        let a5155 = v1;
        let a5156 = 0u64;
        nat_eq(a5155, a5156)
    } {
        true
    } else {
        let a5157 = v2;
        let a5158 = 0u64;
        nat_eq(a5157, a5158)
    } {
        true
    } else {
        let a5159 = v3;
        let a5160 = 0u64;
        nat_eq(a5159, a5160)
    }
}

pub fn f210(v0: u64) -> u64 {
    let m5161 = f130(v0);
    match m5161 {
        Adt17::C0 => {
            0u64
        }
        Adt17::C1(v1) => {
            match v1 {
                Adt18::C0(_, h5162, _) => {
                    h5162
                }
            }
        }
    }
}

pub fn f211(v0: Adt3, v1: bool) -> R<u64> {
    f248(v0.clone(), {
        let a5164 = match v0.clone() {
            Adt3::C0(h5163, _, _, _, _, _, _, _) => {
                h5163
            }
        };
        let a5165 = 1u64;
        nat_sub(a5164, a5165)
    }, {
        let a5167 = match v0.clone() {
            Adt3::C0(_, h5166, _, _, _, _, _, _) => {
                h5166
            }
        };
        let a5168 = 1u64;
        nat_sub(a5167, a5168)
    }, {
        let a5170 = match v0.clone() {
            Adt3::C0(_, _, h5169, _, _, _, _, _) => {
                h5169
            }
        };
        let a5171 = 1u64;
        nat_sub(a5170, a5171)
    }, {
        let a5173 = match v0 {
            Adt3::C0(_, _, _, h5172, _, _, _, _) => {
                h5172
            }
        };
        let a5174 = 1u64;
        nat_sub(a5173, a5174)
    }, v1)
}

pub fn f212(v0: u64, v1: u64, v2: u64, v3: u64) -> R<u64> {
    let a5175 = v0;
    let a5176 = 0u64;
    if nat_eq(a5175, a5176) {
        Ok(0u64)
    } else {
        let v4: u64 = {
            let a5177 = v0;
            let a5178 = 1u64;
            nat_sub(a5177, a5178)
        };
        let v5: u64 = {
            let a5179 = v4;
            let a5180 = v1;
            let a5181 = 0u64;
            nat_quot(a5179, a5180, a5181)
        };
        let v6: u64 = {
            let a5189 = {
                let a5182 = v5;
                let a5183 = v2;
                nat_mul(a5182, a5183)?
            };
            let a5190 = {
                let a5187 = {
                    let a5184 = v4;
                    let a5185 = v1;
                    let a5186 = 0u64;
                    nat_rem(a5184, a5185, a5186)
                };
                let a5188 = v3;
                nat_mul(a5187, a5188)?
            };
            nat_add(a5189, a5190)?
        };
        let a5191 = v5;
        let a5192 = 0u64;
        if nat_eq(a5191, a5192) {
            Ok(v6)
        } else {
            let v7: u64 = {
                let a5201 = {
                    let a5195 = {
                        let a5193 = v5;
                        let a5194 = 1u64;
                        nat_sub(a5193, a5194)
                    };
                    let a5196 = v2;
                    nat_mul(a5195, a5196)?
                };
                let a5202 = {
                    let a5199 = {
                        let a5197 = v1;
                        let a5198 = 1u64;
                        nat_sub(a5197, a5198)
                    };
                    let a5200 = v3;
                    nat_mul(a5199, a5200)?
                };
                nat_add(a5201, a5202)?
            };
            let a5203 = v6;
            let a5204 = v7;
            if nat_lt(a5203, a5204) {
                Ok(v7)
            } else {
                Ok(v6)
            }
        }
    }
}

pub fn f213(v0: Adt3, v1: u64) -> bool {
    let a5205 = v1;
    let a5206 = 0u64;
    if !nat_eq(a5205, a5206) {
        true
    } else {
        let a5208 = match v0 {
            Adt3::C0(_, _, _, _, h5207, _, _, _) => {
                h5207
            }
        };
        let a5209 = 4u64;
        nat_eq(a5208, a5209)
    }
}

pub fn f214(v0: u64, v1: bool) -> R<u64> {
    let a5212 = {
        let a5210 = v0;
        let a5211 = 2u64;
        nat_mul(a5210, a5211)?
    };
    let a5213 = if v1 {
        4u64
    } else {
        2u64
    };
    nat_add(a5212, a5213)
}

pub fn f215(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: bool, v6: u64, v7: u64, v8: u64, v9: u64, v10: u64, v11: u64, v12: u64, v13: u64, v14: u64, v15: u64, v16: u64, v17: u64) -> R<Adt10> {
    let m5214 = f249(v0, v1);
    match m5214 {
        None => {
            if v5 {
                let m5215 = f250(v0, v2, v3, v4, v6, v7, v8, v9, v10, v11, v12, v13, v14, v15, v16, v17)?;
                match m5215 {
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
                let a5216 = 4u64;
                let a5217 = v28;
                nat_le(a5216, a5217)
            };
            let a5218 = v2;
            let a5219 = 0u64;
            if if if if if nat_eq(a5218, a5219) {
                let a5220 = v4;
                let a5221 = 0u64;
                nat_eq(a5220, a5221)
            } else {
                false
            } {
                let a5222 = v6;
                let a5223 = v14;
                if nat_eq(a5222, a5223) {
                    let a5224 = v7;
                    let a5225 = v15;
                    nat_eq(a5224, a5225)
                } else {
                    false
                }
            } else {
                false
            } {
                let a5226 = v8;
                let a5227 = v16;
                if nat_eq(a5226, a5227) {
                    let a5228 = v9;
                    let a5229 = v17;
                    nat_eq(a5228, a5229)
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
                        let a5230 = v3;
                        let a5231 = 0u64;
                        nat_eq(a5230, a5231)
                    } else {
                        false
                    } {
                        f133(v6, v7, v8, v9, v10, v11, v12, v13)
                    } else {
                        false
                    }
                }
            } else {
                false
            } {
                Ok(Adt10::C2(v28, v29, v6, v7, v8, v9, {
                    let a5236 = {
                        let a5232 = v6;
                        let a5233 = v7;
                        nat_mul(a5232, a5233)?
                    };
                    let a5237 = {
                        let a5234 = v8;
                        let a5235 = v9;
                        nat_mul(a5234, a5235)?
                    };
                    nat_mul(a5236, a5237)?
                }))
            } else {
                Ok(Adt10::C1)
            }
        }
    }
}

pub fn f216(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let a5242 = {
        let a5240 = {
            let a5238 = v0;
            let a5239 = v1;
            nat_sub(a5238, a5239)
        };
        let a5241 = 1u64;
        nat_sub(a5240, a5241)
    };
    let a5243 = v2;
    nat_add(a5242, a5243)
}

pub fn f217(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64, v7: u64, v8: u64, v9: u64, v10: u64, v11: u64) -> R<u64> {
    f147({
        let a5244 = v0;
        let a5245 = v4;
        let a5246 = 0u64;
        nat_rem(a5244, a5245, a5246)
    }, {
        let a5247 = v1;
        let a5248 = v5;
        let a5249 = 0u64;
        nat_rem(a5247, a5248, a5249)
    }, {
        let a5250 = v2;
        let a5251 = v6;
        let a5252 = 0u64;
        nat_rem(a5250, a5251, a5252)
    }, {
        let a5253 = v3;
        let a5254 = v7;
        let a5255 = 0u64;
        nat_rem(a5253, a5254, a5255)
    }, v8, v9, v10, v11)
}

pub fn f218(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let a5256 = v1;
    let a5257 = 2147483648u64;
    if nat_lt(a5256, a5257) {
        let a5268 = {
            let a5266 = {
                let a5261 = {
                    let a5258 = v0;
                    let a5259 = v2;
                    let a5260 = 0u64;
                    nat_rem(a5258, a5259, a5260)
                };
                let a5262 = v2;
                nat_add(a5261, a5262)?
            };
            let a5267 = {
                let a5263 = v1;
                let a5264 = v2;
                let a5265 = 0u64;
                nat_rem(a5263, a5264, a5265)
            };
            nat_sub(a5266, a5267)
        };
        let a5269 = v2;
        let a5270 = 0u64;
        Ok(nat_rem(a5268, a5269, a5270))
    } else {
        let a5273 = {
            let a5271 = v0;
            let a5272 = f148(v1);
            nat_add(a5271, a5272)?
        };
        let a5274 = v2;
        let a5275 = 0u64;
        Ok(nat_rem(a5273, a5274, a5275))
    }
}

pub fn f219(v0: u64, v1: u64, v2: u64) -> R<bool> {
    let a5278 = f149(v1);
    let a5279 = {
        let a5276 = v0;
        let a5277 = f148(v1);
        nat_add(a5276, a5277)?
    };
    if nat_le(a5278, a5279) {
        let a5280 = f220(v0, v1)?;
        let a5281 = v2;
        Ok(nat_lt(a5280, a5281))
    } else {
        Ok(false)
    }
}

pub fn f220(v0: u64, v1: u64) -> R<u64> {
    let a5284 = {
        let a5282 = v0;
        let a5283 = f148(v1);
        nat_add(a5282, a5283)?
    };
    let a5285 = f149(v1);
    Ok(nat_sub(a5284, a5285))
}

pub fn f221(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let a5286 = v0;
    let a5287 = v1;
    if nat_lt(a5286, a5287) {
        let a5288 = v1;
        let a5289 = v0;
        Ok(nat_sub(a5288, a5289))
    } else {
        let a5292 = v0;
        let a5293 = {
            let a5290 = v1;
            let a5291 = v2;
            nat_add(a5290, a5291)?
        };
        if nat_lt(a5292, a5293) {
            let a5294 = v0;
            let a5295 = v1;
            Ok(nat_sub(a5294, a5295))
        } else {
            let a5302 = {
                let a5298 = 2u64;
                let a5299 = {
                    let a5296 = v2;
                    let a5297 = 1u64;
                    nat_sub(a5296, a5297)
                };
                nat_mul(a5298, a5299)?
            };
            let a5303 = {
                let a5300 = v0;
                let a5301 = v1;
                nat_sub(a5300, a5301)
            };
            Ok(nat_sub(a5302, a5303))
        }
    }
}

pub fn f222(v0: u64, v1: Adt3, v2: u64) -> R<u64> {
    let a5306 = {
        let a5304 = v2;
        let a5305 = f248(v1.clone(), f75(v0, v1.clone(), 0u64), f75(v0, v1.clone(), 1u64), f75(v0, v1.clone(), 2u64), f75(v0, v1.clone(), 3u64), false)?;
        nat_add(a5304, a5305)?
    };
    let a5307 = f248(v1.clone(), f75(v0, v1.clone(), 0u64), f75(v0, v1.clone(), 1u64), f75(v0, v1.clone(), 2u64), f75(v0, v1, 3u64), true)?;
    Ok(nat_sub(a5306, a5307))
}

pub fn f223(v0: u64, v1: u64, v2: u64) -> bool {
    let a5308 = v0;
    let a5309 = 0u64;
    if nat_eq(a5308, a5309) {
        let a5310 = v2;
        let a5311 = v1;
        nat_le(a5310, a5311)
    } else {
        let a5312 = v0;
        let a5313 = 1u64;
        if nat_eq(a5312, a5313) {
            let a5314 = v2;
            let a5315 = v1;
            nat_lt(a5314, a5315)
        } else {
            let a5316 = v0;
            let a5317 = 2u64;
            if nat_eq(a5316, a5317) {
                let a5318 = v1;
                let a5319 = v2;
                nat_le(a5318, a5319)
            } else {
                let a5320 = v0;
                let a5321 = 3u64;
                if nat_eq(a5320, a5321) {
                    let a5322 = v1;
                    let a5323 = v2;
                    nat_lt(a5322, a5323)
                } else {
                    let a5324 = v1;
                    let a5325 = v2;
                    nat_eq(a5324, a5325)
                }
            }
        }
    }
}

pub fn f224(v0: u64, v1: Adt3, v2: u64) -> R<Adt9> {
    let a5326 = v0;
    let a5327 = 0u64;
    if nat_eq(a5326, a5327) {
        Ok(f235(0u64))
    } else {
        let a5330 = {
            let a5328 = v0;
            let a5329 = 1u64;
            nat_add(a5328, a5329)?
        };
        let a5331 = f144(v1.clone(), 13u64, v2)?;
        if nat_eq(a5330, a5331) {
            Ok(Adt9::C4({
                let a5332 = v2;
                let a5333 = 0u64;
                if nat_eq(a5332, a5333) {
                    2u64
                } else {
                    7u64
                }
            }, 1u64, 0u64, 2u64, 0u64, 3u64, 0u64))
        } else {
            let v3: u64 = {
                let a5334 = v0;
                let a5335 = 1u64;
                nat_sub(a5334, a5335)
            };
            if f213(v1.clone(), v2) {
                let v4: u64 = f214(match v1.clone() {
                    Adt3::C0(h5336, _, _, _, _, _, _, _) => {
                        h5336
                    }
                }, false)?;
                let v5: u64 = {
                    let a5337 = v3;
                    let a5338 = v4;
                    let a5339 = 0u64;
                    nat_quot(a5337, a5338, a5339)
                };
                let v6: u64 = {
                    let a5340 = v3;
                    let a5341 = v4;
                    let a5342 = 0u64;
                    nat_rem(a5340, a5341, a5342)
                };
                let a5345 = {
                    let a5343 = v6;
                    let a5344 = 1u64;
                    nat_add(a5343, a5344)?
                };
                let a5346 = v4;
                if nat_eq(a5345, a5346) {
                    Ok(Adt9::C4({
                        let a5347 = v2;
                        let a5348 = 0u64;
                        if nat_eq(a5347, a5348) {
                            0u64
                        } else {
                            4u64
                        }
                    }, 1u64, 0u64, 1u64, 1u64, 1u64, 0u64))
                } else {
                    f225(v6, v5, v1, v2)
                }
            } else {
                f251({
                    let a5349 = v3;
                    let a5350 = 2u64;
                    let a5351 = 0u64;
                    nat_quot(a5349, a5350, a5351)
                }, {
                    let a5352 = v3;
                    let a5353 = 2u64;
                    let a5354 = 0u64;
                    nat_rem(a5352, a5353, a5354)
                }, v1, v2, 0u64)
            }
        }
    }
}

pub fn f225(v0: u64, v1: u64, v2: Adt3, v3: u64) -> R<Adt9> {
    let a5355 = v0;
    let a5356 = 0u64;
    if nat_eq(a5355, a5356) {
        Ok(f235(1u64))
    } else {
        f251({
            let a5365 = {
                let a5358 = v1;
                let a5359 = match v2.clone() {
                    Adt3::C0(h5357, _, _, _, _, _, _, _) => {
                        h5357
                    }
                };
                nat_mul(a5358, a5359)?
            };
            let a5366 = {
                let a5362 = {
                    let a5360 = v0;
                    let a5361 = 1u64;
                    nat_sub(a5360, a5361)
                };
                let a5363 = 2u64;
                let a5364 = 0u64;
                nat_quot(a5362, a5363, a5364)
            };
            nat_add(a5365, a5366)?
        }, {
            let a5369 = {
                let a5367 = v0;
                let a5368 = 1u64;
                nat_sub(a5367, a5368)
            };
            let a5370 = 2u64;
            let a5371 = 0u64;
            nat_rem(a5369, a5370, a5371)
        }, v2, v3, 1u64)
    }
}

pub fn f226(v0: u64, v1: u64) -> R<u64> {
    let a5372 = v0;
    let a5373 = f104(v1)?;
    let a5374 = 0u64;
    Ok(nat_rem(a5372, a5373, a5374))
}

pub fn f227(v0: u64, v1: u64) -> R<u64> {
    let a5375 = 4u64;
    let a5376 = f252(v0, v1)?;
    nat_mul(a5375, a5376)
}

pub fn f228(v0: u64, v1: u64) -> R<u64> {
    let a5377 = 4u64;
    let a5378 = f253(v0, v1)?;
    nat_mul(a5377, a5378)
}

pub fn f229(v0: u64, v1: u64) -> R<bool> {
    let a5384 = {
        let a5381 = {
            let a5379 = f226(v0, v1)?;
            let a5380 = 2u64;
            nat_sub(a5379, a5380)
        };
        let a5382 = 2u64;
        let a5383 = 0u64;
        nat_rem(a5381, a5382, a5383)
    };
    let a5385 = 0u64;
    Ok(nat_eq(a5384, a5385))
}

pub fn f230(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64, v7: u64, v8: u64, v9: u64, v10: u64, v11: u64, v12: u64) -> R<Option<u64>> {
    let m5386 = f130(v0);
    match m5386 {
        Adt17::C0 => {
            Ok(None::<u64>)
        }
        Adt17::C1(v13) => {
            if if if if if f128(v0) {
                let a5387 = v1;
                let a5388 = v7;
                nat_lt(a5387, a5388)
            } else {
                false
            } {
                f178(v8, v2)
            } else {
                false
            } {
                let a5389 = v3;
                let a5390 = f254(v4, v5, v6)?;
                nat_lt(a5389, a5390)
            } else {
                false
            } {
                let a5395 = {
                    let a5392 = v7;
                    let a5393 = match v13.clone() {
                        Adt18::C0(_, h5391, _) => {
                            h5391
                        }
                    };
                    let a5394 = 0u64;
                    nat_rem(a5392, a5393, a5394)
                };
                let a5396 = 0u64;
                nat_eq(a5395, a5396)
            } else {
                false
            } {
                Ok(Some({
                    let a5425 = {
                        let a5415 = {
                            let a5405 = {
                                let a5401 = {
                                    let a5398 = v1;
                                    let a5399 = match v13 {
                                        Adt18::C0(_, h5397, _) => {
                                            h5397
                                        }
                                    };
                                    let a5400 = 0u64;
                                    nat_quot(a5398, a5399, a5400)
                                };
                                let a5402 = v9;
                                nat_mul(a5401, a5402)?
                            };
                            let a5406 = {
                                let a5403 = v2;
                                let a5404 = v10;
                                nat_mul(a5403, a5404)?
                            };
                            nat_add(a5405, a5406)?
                        };
                        let a5416 = {
                            let a5413 = {
                                let a5410 = {
                                    let a5407 = v3;
                                    let a5408 = v4;
                                    let a5409 = 0u64;
                                    nat_quot(a5407, a5408, a5409)
                                };
                                let a5411 = v5;
                                let a5412 = 0u64;
                                nat_rem(a5410, a5411, a5412)
                            };
                            let a5414 = v11;
                            nat_mul(a5413, a5414)?
                        };
                        nat_add(a5415, a5416)?
                    };
                    let a5426 = {
                        let a5423 = {
                            let a5420 = {
                                let a5417 = v3;
                                let a5418 = v4;
                                let a5419 = 0u64;
                                nat_quot(a5417, a5418, a5419)
                            };
                            let a5421 = v5;
                            let a5422 = 0u64;
                            nat_quot(a5420, a5421, a5422)
                        };
                        let a5424 = v12;
                        nat_mul(a5423, a5424)?
                    };
                    nat_add(a5425, a5426)?
                }))
            } else {
                Ok(None::<u64>)
            }
        }
    }
}

pub fn f231(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64, v7: u64, v8: u64, v9: u64, v10: u64, v11: u64, v12: u64, v13: u64, v14: u64) -> R<Adt19> {
    f255(v0, Adt20::C0(v1, v2, v3, v4, v5, v6, v7, v8), Adt20::C0(v1, v2, v3, v4, v9, v10, v11, v12), v13, v14)
}

pub fn f232(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64, v7: u64, v8: u64, v9: u64, v10: u64, v11: u64, v12: u64, v13: u64, v14: u64, v15: u64, v16: u64, v17: u64, v18: u64, v19: u64, v20: u64) -> R<Adt19> {
    f256(v0, Adt20::C0(v1, v2, v3, v4, v5, v6, v7, v8), Adt20::C0(v1, v2, v3, v4, v9, v10, v11, v12), Adt20::C0(v1, v2, v3, v4, v13, v14, v15, v16), v17, v18, v19, v20)
}

pub fn f233(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64, v7: u64, v8: u64, v9: u64, v10: u64, v11: u64, v12: u64, v13: u64, v14: u64, v15: u64, v16: u64, v17: u64) -> R<Adt19> {
    let a5427 = v0;
    let a5428 = 64u64;
    if nat_eq(a5427, a5428) {
        f257(v1, Adt20::C0(v2, v3, v4, v5, v6, v7, v8, v9), Adt20::C0(v2, v3, v4, v5, v10, v11, v12, v13), v14, v15, v16, v17)
    } else {
        let a5429 = v0;
        let a5430 = 65u64;
        if nat_eq(a5429, a5430) {
            f258(v1, Adt20::C0(v2, v3, v4, v5, v6, v7, v8, v9), Adt20::C0(v2, v3, v4, v5, v10, v11, v12, v13), v14, v15, v16, v17)
        } else {
            Ok(Adt19::C0)
        }
    }
}

pub fn f234(v0: u64) -> u64 {
    let a5431 = v0;
    let a5432 = 30u64;
    if nat_eq(a5431, a5432) {
        4u64
    } else {
        3u64
    }
}

pub fn f235(v0: u64) -> Adt9 {
    Adt9::C4(0u64, 2u64, 0u64, 2u64, 0u64, 1u64, v0)
}

pub fn f236(v0: u64) -> bool {
    let a5433 = 0u64;
    let a5434 = v0;
    if nat_lt(a5433, a5434) {
        let a5435 = v0;
        let a5436 = 2139095040u64;
        nat_le(a5435, a5436)
    } else {
        false
    }
}

pub fn f237(v0: u64) -> u64 {
    let a5437 = v0;
    let a5438 = 40u64;
    if nat_lt(a5437, a5438) {
        let a5439 = v0;
        let a5440 = 20u64;
        if nat_lt(a5439, a5440) {
            let a5441 = v0;
            let a5442 = 10u64;
            if nat_lt(a5441, a5442) {
                let a5443 = v0;
                let a5444 = 5u64;
                if nat_lt(a5443, a5444) {
                    let a5445 = v0;
                    let a5446 = 2u64;
                    if nat_lt(a5445, a5446) {
                        let a5447 = v0;
                        let a5448 = 1u64;
                        if nat_lt(a5447, a5448) {
                            1u64
                        } else {
                            2u64
                        }
                    } else {
                        let a5449 = v0;
                        let a5450 = 3u64;
                        if nat_lt(a5449, a5450) {
                            4u64
                        } else {
                            let a5451 = v0;
                            let a5452 = 4u64;
                            if nat_lt(a5451, a5452) {
                                8u64
                            } else {
                                16u64
                            }
                        }
                    }
                } else {
                    let a5453 = v0;
                    let a5454 = 7u64;
                    if nat_lt(a5453, a5454) {
                        let a5455 = v0;
                        let a5456 = 6u64;
                        if nat_lt(a5455, a5456) {
                            32u64
                        } else {
                            64u64
                        }
                    } else {
                        let a5457 = v0;
                        let a5458 = 8u64;
                        if nat_lt(a5457, a5458) {
                            128u64
                        } else {
                            let a5459 = v0;
                            let a5460 = 9u64;
                            if nat_lt(a5459, a5460) {
                                256u64
                            } else {
                                512u64
                            }
                        }
                    }
                }
            } else {
                let a5461 = v0;
                let a5462 = 15u64;
                if nat_lt(a5461, a5462) {
                    let a5463 = v0;
                    let a5464 = 12u64;
                    if nat_lt(a5463, a5464) {
                        let a5465 = v0;
                        let a5466 = 11u64;
                        if nat_lt(a5465, a5466) {
                            1024u64
                        } else {
                            2048u64
                        }
                    } else {
                        let a5467 = v0;
                        let a5468 = 13u64;
                        if nat_lt(a5467, a5468) {
                            4096u64
                        } else {
                            let a5469 = v0;
                            let a5470 = 14u64;
                            if nat_lt(a5469, a5470) {
                                8192u64
                            } else {
                                16384u64
                            }
                        }
                    }
                } else {
                    let a5471 = v0;
                    let a5472 = 17u64;
                    if nat_lt(a5471, a5472) {
                        let a5473 = v0;
                        let a5474 = 16u64;
                        if nat_lt(a5473, a5474) {
                            32768u64
                        } else {
                            65536u64
                        }
                    } else {
                        let a5475 = v0;
                        let a5476 = 18u64;
                        if nat_lt(a5475, a5476) {
                            131072u64
                        } else {
                            let a5477 = v0;
                            let a5478 = 19u64;
                            if nat_lt(a5477, a5478) {
                                262144u64
                            } else {
                                524288u64
                            }
                        }
                    }
                }
            }
        } else {
            let a5479 = v0;
            let a5480 = 30u64;
            if nat_lt(a5479, a5480) {
                let a5481 = v0;
                let a5482 = 25u64;
                if nat_lt(a5481, a5482) {
                    let a5483 = v0;
                    let a5484 = 22u64;
                    if nat_lt(a5483, a5484) {
                        let a5485 = v0;
                        let a5486 = 21u64;
                        if nat_lt(a5485, a5486) {
                            1048576u64
                        } else {
                            2097152u64
                        }
                    } else {
                        let a5487 = v0;
                        let a5488 = 23u64;
                        if nat_lt(a5487, a5488) {
                            4194304u64
                        } else {
                            let a5489 = v0;
                            let a5490 = 24u64;
                            if nat_lt(a5489, a5490) {
                                8388608u64
                            } else {
                                16777216u64
                            }
                        }
                    }
                } else {
                    let a5491 = v0;
                    let a5492 = 27u64;
                    if nat_lt(a5491, a5492) {
                        let a5493 = v0;
                        let a5494 = 26u64;
                        if nat_lt(a5493, a5494) {
                            33554432u64
                        } else {
                            67108864u64
                        }
                    } else {
                        let a5495 = v0;
                        let a5496 = 28u64;
                        if nat_lt(a5495, a5496) {
                            134217728u64
                        } else {
                            let a5497 = v0;
                            let a5498 = 29u64;
                            if nat_lt(a5497, a5498) {
                                268435456u64
                            } else {
                                536870912u64
                            }
                        }
                    }
                }
            } else {
                let a5499 = v0;
                let a5500 = 35u64;
                if nat_lt(a5499, a5500) {
                    let a5501 = v0;
                    let a5502 = 32u64;
                    if nat_lt(a5501, a5502) {
                        let a5503 = v0;
                        let a5504 = 31u64;
                        if nat_lt(a5503, a5504) {
                            1073741824u64
                        } else {
                            2147483648u64
                        }
                    } else {
                        let a5505 = v0;
                        let a5506 = 33u64;
                        if nat_lt(a5505, a5506) {
                            4294967296u64
                        } else {
                            let a5507 = v0;
                            let a5508 = 34u64;
                            if nat_lt(a5507, a5508) {
                                8589934592u64
                            } else {
                                17179869184u64
                            }
                        }
                    }
                } else {
                    let a5509 = v0;
                    let a5510 = 37u64;
                    if nat_lt(a5509, a5510) {
                        let a5511 = v0;
                        let a5512 = 36u64;
                        if nat_lt(a5511, a5512) {
                            34359738368u64
                        } else {
                            68719476736u64
                        }
                    } else {
                        let a5513 = v0;
                        let a5514 = 38u64;
                        if nat_lt(a5513, a5514) {
                            137438953472u64
                        } else {
                            let a5515 = v0;
                            let a5516 = 39u64;
                            if nat_lt(a5515, a5516) {
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

pub fn f238(v0: u64, v1: u64) -> bool {
    let a5517 = v0;
    let a5518 = 1u64;
    if nat_le(a5517, a5518) {
        true
    } else {
        let a5522 = {
            let a5519 = v1;
            let a5520 = 4u64;
            let a5521 = 0u64;
            nat_rem(a5519, a5520, a5521)
        };
        let a5523 = 0u64;
        nat_eq(a5522, a5523)
    }
}

pub fn f239(v0: u64, v1: u64) -> bool {
    let a5524 = v0;
    let a5525 = 1u64;
    if nat_le(a5524, a5525) {
        true
    } else {
        let a5526 = 4u64;
        let a5527 = v1;
        nat_le(a5526, a5527)
    }
}

pub fn f240(v0: u64, v1: u64) -> R<u64> {
    let a5528 = v0;
    let a5529 = 0u64;
    if nat_eq(a5528, a5529) {
        Ok(0u64)
    } else {
        let a5534 = 4u64;
        let a5535 = {
            let a5532 = {
                let a5530 = v0;
                let a5531 = 1u64;
                nat_sub(a5530, a5531)
            };
            let a5533 = v1;
            nat_mul(a5532, a5533)?
        };
        nat_add(a5534, a5535)
    }
}

pub fn f241(v0: bool, v1: u64) -> R<u64> {
    let a5540 = {
        let a5538 = {
            let a5536 = f259(v0, v1)?;
            let a5537 = f260(v0, v1)?;
            nat_add(a5536, a5537)?
        };
        let a5539 = f261(v0);
        nat_add(a5538, a5539)?
    };
    let a5541 = v1;
    nat_add(a5540, a5541)
}

pub fn f242(v0: u64, v1: u64) -> R<u64> {
    let v2: u64 = f262(v0, v1)?;
    let v3: u64 = {
        let a5546 = {
            let a5544 = {
                let a5542 = v1;
                let a5543 = 1u64;
                nat_sub(a5542, a5543)
            };
            let a5545 = v2;
            nat_mul(a5544, a5545)?
        };
        let a5547 = v0;
        nat_sub(a5546, a5547)
    };
    let a5548 = v2;
    let a5549 = v3;
    if nat_lt(a5548, a5549) {
        Ok(v3)
    } else {
        Ok(v2)
    }
}

pub fn f243(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64) -> R<u64> {
    let a5550 = v4;
    let a5551 = 0u64;
    if nat_eq(a5550, a5551) {
        Ok(0u64)
    } else {
        let a5558 = 2u64;
        let a5559 = {
            let a5556 = {
                let a5552 = v0;
                let a5553 = v1;
                nat_mul(a5552, a5553)?
            };
            let a5557 = {
                let a5554 = v2;
                let a5555 = v3;
                nat_mul(a5554, a5555)?
            };
            nat_mul(a5556, a5557)?
        };
        nat_mul(a5558, a5559)
    }
}

pub fn f244(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64) -> R<u64> {
    let a5564 = v3;
    let a5565 = {
        let a5562 = {
            let a5560 = f263(v2, v4)?;
            let a5561 = f264(v0, v1, f262(v2, v4)?)?;
            nat_mul(a5560, a5561)?
        };
        let a5563 = f264(v0, v1, f265(v2, v4)?)?;
        nat_add(a5562, a5563)?
    };
    nat_mul(a5564, a5565)
}

pub fn f245(v0: u64) -> R<u64> {
    let a5568 = 13u64;
    let a5569 = {
        let a5566 = 9u64;
        let a5567 = v0;
        nat_mul(a5566, a5567)?
    };
    nat_add(a5568, a5569)
}

pub fn f246(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64) -> bool {
    if f209(v1, v2, v3, v4) {
        f266(v0)
    } else {
        f128(v0)
    }
}

pub fn f247(v0: u64) -> u64 {
    let a5570 = v0;
    let a5571 = 26u64;
    if nat_eq(a5570, a5571) {
        26u64
    } else {
        0u64
    }
}

pub fn f248(v0: Adt3, v1: u64, v2: u64, v3: u64, v4: u64, v5: bool) -> R<u64> {
    let a5592 = {
        let a5586 = {
            let a5580 = {
                let a5574 = v1;
                let a5575 = if v5 {
                    f148(match v0.clone() {
                        Adt3::C0(_, _, _, _, h5572, _, _, _) => {
                            h5572
                        }
                    })
                } else {
                    f149(match v0.clone() {
                        Adt3::C0(_, _, _, _, h5573, _, _, _) => {
                            h5573
                        }
                    })
                };
                nat_mul(a5574, a5575)?
            };
            let a5581 = {
                let a5578 = v2;
                let a5579 = if v5 {
                    f148(match v0.clone() {
                        Adt3::C0(_, _, _, _, _, h5576, _, _) => {
                            h5576
                        }
                    })
                } else {
                    f149(match v0.clone() {
                        Adt3::C0(_, _, _, _, _, h5577, _, _) => {
                            h5577
                        }
                    })
                };
                nat_mul(a5578, a5579)?
            };
            nat_add(a5580, a5581)?
        };
        let a5587 = {
            let a5584 = v3;
            let a5585 = if v5 {
                f148(match v0.clone() {
                    Adt3::C0(_, _, _, _, _, _, h5582, _) => {
                        h5582
                    }
                })
            } else {
                f149(match v0.clone() {
                    Adt3::C0(_, _, _, _, _, _, h5583, _) => {
                        h5583
                    }
                })
            };
            nat_mul(a5584, a5585)?
        };
        nat_add(a5586, a5587)?
    };
    let a5593 = {
        let a5590 = v4;
        let a5591 = if v5 {
            f148(match v0 {
                Adt3::C0(_, _, _, _, _, _, _, h5588) => {
                    h5588
                }
            })
        } else {
            f149(match v0 {
                Adt3::C0(_, _, _, _, _, _, _, h5589) => {
                    h5589
                }
            })
        };
        nat_mul(a5590, a5591)?
    };
    nat_add(a5592, a5593)
}

pub fn f249(v0: u64, v1: u64) -> Option<u64> {
    let a5594 = v0;
    let a5595 = 2u64;
    if nat_eq(a5594, a5595) {
        Some(0u64)
    } else {
        let a5596 = v0;
        let a5597 = 5u64;
        if nat_eq(a5596, a5597) {
            Some(1u64)
        } else {
            let a5598 = v0;
            let a5599 = 6u64;
            if nat_eq(a5598, a5599) {
                Some(2u64)
            } else {
                let a5600 = v0;
                let a5601 = 7u64;
                if nat_eq(a5600, a5601) {
                    Some(3u64)
                } else {
                    let a5602 = v0;
                    let a5603 = 9u64;
                    if nat_eq(a5602, a5603) {
                        Some(4u64)
                    } else {
                        let a5604 = v0;
                        let a5605 = 8u64;
                        if nat_eq(a5604, a5605) {
                            Some(5u64)
                        } else {
                            let a5606 = v0;
                            let a5607 = 89u64;
                            if nat_eq(a5606, a5607) {
                                let a5608 = v1;
                                let a5609 = 2u64;
                                if nat_eq(a5608, a5609) {
                                    Some(6u64)
                                } else {
                                    let a5610 = v1;
                                    let a5611 = 0u64;
                                    if nat_eq(a5610, a5611) {
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

pub fn f250(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64, v7: u64, v8: u64, v9: u64, v10: u64, v11: u64, v12: u64, v13: u64, v14: u64, v15: u64) -> R<Adt21> {
    let a5612 = v0;
    let a5613 = 30u64;
    if nat_eq(a5612, a5613) {
        let a5614 = v2;
        let a5615 = 0u64;
        if if if nat_eq(a5614, a5615) {
            true
        } else {
            let a5616 = v2;
            let a5617 = 1u64;
            nat_eq(a5616, a5617)
        } {
            let a5618 = v1;
            let a5619 = 0u64;
            if nat_eq(a5618, a5619) {
                true
            } else {
                f267(v1, v4)
            }
        } else {
            false
        } {
            f268(28u64, 0u64, 0u64, v3, v5, v4, v6, v7, v9, v8, v10, v11, v12, v13, v14, v15)
        } else {
            Ok(Adt21::C0)
        }
    } else {
        if if if if f269(v1) {
            f269(v2)
        } else {
            false
        } {
            let a5620 = v1;
            let a5621 = 0u64;
            if if nat_eq(a5620, a5621) {
                true
            } else {
                let a5622 = v2;
                let a5623 = 0u64;
                nat_eq(a5622, a5623)
            } {
                true
            } else {
                let a5624 = v1;
                let a5625 = v2;
                nat_eq(a5624, a5625)
            }
        } else {
            false
        } {
            true
        } else {
            let a5626 = v1;
            let a5627 = 43u64;
            if nat_eq(a5626, a5627) {
                f269(v2)
            } else {
                false
            }
        } {
            f268(v0, 0u64, 0u64, v3, v4, v5, v6, v7, v8, v9, v10, v11, v12, v13, v14, v15)
        } else {
            Ok(Adt21::C0)
        }
    }
}

pub fn f251(v0: u64, v1: u64, v2: Adt3, v3: u64, v4: u64) -> R<Adt9> {
    let a5628 = v1;
    let a5629 = 0u64;
    if nat_eq(a5628, a5629) {
        Ok(Adt9::C4({
            let a5630 = v3;
            let a5631 = 0u64;
            if nat_eq(a5630, a5631) {
                1u64
            } else {
                6u64
            }
        }, 0u64, f155(v0, v2)?, 2u64, 0u64, 1u64, 2u64))
    } else {
        Ok(Adt9::C4({
            let a5632 = v3;
            let a5633 = 0u64;
            if nat_eq(a5632, a5633) {
                0u64
            } else {
                4u64
            }
        }, 1u64, v4, 1u64, 2u64, 1u64, v4))
    }
}

pub fn f252(v0: u64, v1: u64) -> R<u64> {
    let a5634 = v0;
    let a5635 = f104(v1)?;
    let a5636 = 0u64;
    Ok(nat_quot(a5634, a5635, a5636))
}

pub fn f253(v0: u64, v1: u64) -> R<u64> {
    let v2: u64 = f252(v0, v1)?;
    let v3: u64 = {
        let a5639 = {
            let a5637 = f226(v0, v1)?;
            let a5638 = 2u64;
            nat_sub(a5637, a5638)
        };
        let a5640 = 2u64;
        let a5641 = 0u64;
        nat_quot(a5639, a5640, a5641)
    };
    let v4: u64 = {
        let a5642 = v2;
        let a5643 = v1;
        nat_mul(a5642, a5643)?
    };
    if f229(v0, v1)? {
        let a5644 = v4;
        let a5645 = v3;
        nat_add(a5644, a5645)
    } else {
        let a5657 = {
            let a5653 = v4;
            let a5654 = {
                let a5651 = {
                    let a5646 = v1;
                    let a5647 = 1u64;
                    nat_sub(a5646, a5647)
                };
                let a5652 = {
                    let a5648 = v2;
                    let a5649 = v1;
                    let a5650 = 0u64;
                    nat_rem(a5648, a5649, a5650)
                };
                nat_mul(a5651, a5652)?
            };
            nat_sub(a5653, a5654)
        };
        let a5658 = {
            let a5655 = v3;
            let a5656 = v1;
            nat_mul(a5655, a5656)?
        };
        nat_add(a5657, a5658)
    }
}

pub fn f254(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let a5659 = v0;
    let a5660 = 0u64;
    if if if nat_eq(a5659, a5660) {
        true
    } else {
        let a5661 = v1;
        let a5662 = 0u64;
        nat_eq(a5661, a5662)
    } {
        true
    } else {
        let a5663 = v2;
        let a5664 = 0u64;
        nat_eq(a5663, a5664)
    } {
        Ok(0u64)
    } else {
        let a5667 = {
            let a5665 = v0;
            let a5666 = v1;
            nat_mul(a5665, a5666)?
        };
        let a5668 = v2;
        nat_mul(a5667, a5668)
    }
}

pub fn f255(v0: u64, v1: Adt20, v2: Adt20, v3: u64, v4: u64) -> R<Adt19> {
    let v5: u64 = f113(match v1.clone() {
        Adt20::C0(h5669, _, _, _, _, _, _, _) => {
            h5669
        }
    })?;
    let v6: u64 = {
        let a5675 = match v1.clone() {
            Adt20::C0(_, h5670, _, _, _, _, _, _) => {
                h5670
            }
        };
        let a5676 = {
            let a5673 = match v1.clone() {
                Adt20::C0(_, _, h5671, _, _, _, _, _) => {
                    h5671
                }
            };
            let a5674 = match v1.clone() {
                Adt20::C0(_, _, _, h5672, _, _, _, _) => {
                    h5672
                }
            };
            nat_mul(a5673, a5674)?
        };
        nat_mul(a5675, a5676)?
    };
    let a5678 = v0;
    let a5679 = f39(match v1.clone() {
        Adt20::C0(h5677, _, _, _, _, _, _, _) => {
            h5677
        }
    }, v6)?;
    if nat_lt(a5678, a5679) {
        f270({
            let a5680 = v0;
            let a5681 = v5;
            let a5682 = 0u64;
            nat_rem(a5680, a5681, a5682)
        }, {
            let a5683 = v0;
            let a5684 = v5;
            let a5685 = 0u64;
            nat_quot(a5683, a5684, a5685)
        }, v1, v2, v3, v4)
    } else {
        Ok(Adt19::C0)
    }
}

pub fn f256(v0: u64, v1: Adt20, v2: Adt20, v3: Adt20, v4: u64, v5: u64, v6: u64, v7: u64) -> R<Adt19> {
    let v8: u64 = {
        let a5686 = v6;
        let a5687 = 6u64;
        nat_sub(a5686, a5687)
    };
    let v9: Adt20 = f271(v3);
    let a5689 = v0;
    let a5690 = f205(match v1.clone() {
        Adt20::C0(h5688, _, _, _, _, _, _, _) => {
            h5688
        }
    }, v5, v8)?;
    if nat_lt(a5689, a5690) {
        let v10: u64 = f110(match v1.clone() {
            Adt20::C0(h5691, _, _, _, _, _, _, _) => {
                h5691
            }
        }, v5, v8)?;
        let a5692 = v0;
        let a5693 = v10;
        if nat_lt(a5692, a5693) {
            f272(v0, v9, 4u64, 6u64, v7)
        } else {
            let v11: u64 = {
                let a5694 = v0;
                let a5695 = v10;
                nat_sub(a5694, a5695)
            };
            let v12: u64 = f245(match v1.clone() {
                Adt20::C0(h5696, _, _, _, _, _, _, _) => {
                    h5696
                }
            })?;
            f273({
                let a5697 = v11;
                let a5698 = v12;
                let a5699 = 0u64;
                nat_rem(a5697, a5698, a5699)
            }, {
                let a5700 = v11;
                let a5701 = v12;
                let a5702 = 0u64;
                nat_quot(a5700, a5701, a5702)
            }, v1, v2, v9, v4)
        }
    } else {
        Ok(Adt19::C0)
    }
}

pub fn f257(v0: u64, v1: Adt20, v2: Adt20, v3: u64, v4: u64, v5: u64, v6: u64) -> R<Adt19> {
    let v7: u64 = {
        let a5703 = v5;
        let a5704 = 6u64;
        nat_sub(a5703, a5704)
    };
    let v8: bool = f199(match v1.clone() {
        Adt20::C0(_, _, _, _, h5705, _, _, _) => {
            h5705
        }
    }, match v2.clone() {
        Adt20::C0(_, _, _, _, h5706, _, _, _) => {
            h5706
        }
    });
    let a5708 = v0;
    let a5709 = f198(v8, match v1.clone() {
        Adt20::C0(h5707, _, _, _, _, _, _, _) => {
            h5707
        }
    }, v4, v7)?;
    if nat_lt(a5708, a5709) {
        let v9: u64 = f110(match v1.clone() {
            Adt20::C0(h5710, _, _, _, _, _, _, _) => {
                h5710
            }
        }, v4, v7)?;
        let a5711 = v0;
        let a5712 = v9;
        if nat_lt(a5711, a5712) {
            f272(v0, v2, 4u64, 6u64, v6)
        } else {
            let v10: u64 = {
                let a5713 = v0;
                let a5714 = v9;
                nat_sub(a5713, a5714)
            };
            let v11: u64 = f241(v8, match v1.clone() {
                Adt20::C0(h5715, _, _, _, _, _, _, _) => {
                    h5715
                }
            })?;
            f274({
                let a5716 = v10;
                let a5717 = v11;
                let a5718 = 0u64;
                nat_rem(a5716, a5717, a5718)
            }, {
                let a5719 = v10;
                let a5720 = v11;
                let a5721 = 0u64;
                nat_quot(a5719, a5720, a5721)
            }, v1, v2, v3, v8)
        }
    } else {
        Ok(Adt19::C0)
    }
}

pub fn f258(v0: u64, v1: Adt20, v2: Adt20, v3: u64, v4: u64, v5: u64, v6: u64) -> R<Adt19> {
    let v7: u64 = {
        let a5722 = v5;
        let a5723 = 6u64;
        nat_sub(a5722, a5723)
    };
    let v8: Adt20 = f271(v2);
    let v9: u64 = f243(match v1.clone() {
        Adt20::C0(h5724, _, _, _, _, _, _, _) => {
            h5724
        }
    }, match v1.clone() {
        Adt20::C0(_, h5725, _, _, _, _, _, _) => {
            h5725
        }
    }, match v1.clone() {
        Adt20::C0(_, _, h5726, _, _, _, _, _) => {
            h5726
        }
    }, match v1.clone() {
        Adt20::C0(_, _, _, h5727, _, _, _, _) => {
            h5727
        }
    }, v7)?;
    let a5732 = v0;
    let a5733 = f204(match v1.clone() {
        Adt20::C0(h5728, _, _, _, _, _, _, _) => {
            h5728
        }
    }, match v1.clone() {
        Adt20::C0(_, h5729, _, _, _, _, _, _) => {
            h5729
        }
    }, match v1.clone() {
        Adt20::C0(_, _, h5730, _, _, _, _, _) => {
            h5730
        }
    }, match v1.clone() {
        Adt20::C0(_, _, _, h5731, _, _, _, _) => {
            h5731
        }
    }, v4, v7)?;
    if nat_lt(a5732, a5733) {
        let a5734 = v0;
        let a5735 = v9;
        if nat_lt(a5734, a5735) {
            f272(v0, v8, 4u64, 6u64, v6)
        } else {
            let v10: Adt22 = f275({
                let a5736 = v0;
                let a5737 = v9;
                nat_sub(a5736, a5737)
            }, match v1.clone() {
                Adt20::C0(h5738, _, _, _, _, _, _, _) => {
                    h5738
                }
            }, match v1.clone() {
                Adt20::C0(_, h5739, _, _, _, _, _, _) => {
                    h5739
                }
            }, match v1.clone() {
                Adt20::C0(_, _, h5740, _, _, _, _, _) => {
                    h5740
                }
            }, match v1.clone() {
                Adt20::C0(_, _, _, h5741, _, _, _, _) => {
                    h5741
                }
            }, v4)?;
            f276(v10, v1, v8, v3)
        }
    } else {
        Ok(Adt19::C0)
    }
}

pub fn f259(v0: bool, v1: u64) -> R<u64> {
    let a5744 = if v0 {
        5u64
    } else {
        4u64
    };
    let a5745 = {
        let a5742 = f277(v0);
        let a5743 = v1;
        nat_mul(a5742, a5743)?
    };
    nat_add(a5744, a5745)
}

pub fn f260(v0: bool, v1: u64) -> R<u64> {
    if v0 {
        let a5753 = {
            let a5746 = 13u64;
            let a5747 = f278(v1);
            nat_mul(a5746, a5747)?
        };
        let a5754 = {
            let a5751 = 4u64;
            let a5752 = {
                let a5748 = v1;
                let a5749 = 4u64;
                let a5750 = 0u64;
                nat_rem(a5748, a5749, a5750)
            };
            nat_mul(a5751, a5752)?
        };
        nat_add(a5753, a5754)
    } else {
        let a5755 = 3u64;
        let a5756 = v1;
        nat_mul(a5755, a5756)
    }
}

pub fn f261(v0: bool) -> u64 {
    if v0 {
        6u64
    } else {
        5u64
    }
}

pub fn f262(v0: u64, v1: u64) -> R<u64> {
    let a5761 = {
        let a5759 = {
            let a5757 = v0;
            let a5758 = v1;
            nat_add(a5757, a5758)?
        };
        let a5760 = 1u64;
        nat_sub(a5759, a5760)
    };
    let a5762 = v1;
    let a5763 = 0u64;
    Ok(nat_quot(a5761, a5762, a5763))
}

pub fn f263(v0: u64, v1: u64) -> R<u64> {
    let a5764 = v0;
    let a5765 = f262(v0, v1)?;
    let a5766 = 0u64;
    Ok(nat_quot(a5764, a5765, a5766))
}

pub fn f264(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let a5767 = v2;
    let a5768 = 0u64;
    if nat_eq(a5767, a5768) {
        Ok(0u64)
    } else {
        let a5773 = 11u64;
        let a5774 = {
            let a5771 = {
                let a5769 = v1;
                let a5770 = v2;
                nat_mul(a5769, a5770)?
            };
            let a5772 = f279(v0)?;
            nat_mul(a5771, a5772)?
        };
        nat_add(a5773, a5774)
    }
}

pub fn f265(v0: u64, v1: u64) -> R<u64> {
    let a5775 = v0;
    let a5776 = f262(v0, v1)?;
    let a5777 = 0u64;
    Ok(nat_rem(a5775, a5776, a5777))
}

pub fn f266(v0: u64) -> bool {
    let m5778 = f130(v0);
    match m5778 {
        Adt17::C0 => {
            false
        }
        Adt17::C1(_) => {
            true
        }
    }
}

pub fn f267(v0: u64, v1: u64) -> bool {
    let a5779 = v0;
    let a5780 = 1u64;
    if nat_eq(a5779, a5780) {
        true
    } else {
        let v2: u64 = f280(v0);
        let a5781 = 0u64;
        let a5782 = v2;
        if nat_lt(a5781, a5782) {
            let a5786 = {
                let a5783 = v1;
                let a5784 = v2;
                let a5785 = 1u64;
                nat_rem(a5783, a5784, a5785)
            };
            let a5787 = 0u64;
            nat_eq(a5786, a5787)
        } else {
            false
        }
    }
}

pub fn f268(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64, v7: u64, v8: u64, v9: u64, v10: u64, v11: u64, v12: u64, v13: u64, v14: u64, v15: u64) -> R<Adt21> {
    let a5788 = v0;
    let a5789 = 28u64;
    if if if if nat_eq(a5788, a5789) {
        let a5790 = v1;
        let a5791 = 0u64;
        nat_eq(a5790, a5791)
    } else {
        false
    } {
        let a5792 = v2;
        let a5793 = 0u64;
        nat_eq(a5792, a5793)
    } else {
        false
    } {
        let a5794 = v3;
        let a5795 = 0u64;
        nat_eq(a5794, a5795)
    } else {
        false
    } {
        let a5796 = v4;
        let a5797 = v8;
        if if if if if if if if nat_eq(a5796, a5797) {
            let a5798 = v5;
            let a5799 = v12;
            nat_eq(a5798, a5799)
        } else {
            false
        } {
            let a5800 = v9;
            let a5801 = v13;
            nat_eq(a5800, a5801)
        } else {
            false
        } {
            let a5802 = v10;
            let a5803 = v14;
            nat_eq(a5802, a5803)
        } else {
            false
        } {
            let a5804 = v11;
            let a5805 = v15;
            nat_eq(a5804, a5805)
        } else {
            false
        } {
            let a5806 = 0u64;
            let a5807 = v6;
            nat_lt(a5806, a5807)
        } else {
            false
        } {
            let a5808 = 0u64;
            let a5809 = v7;
            nat_lt(a5808, a5809)
        } else {
            false
        } {
            let a5813 = {
                let a5810 = v10;
                let a5811 = v6;
                let a5812 = 1u64;
                nat_rem(a5810, a5811, a5812)
            };
            let a5814 = 0u64;
            if nat_eq(a5813, a5814) {
                let a5818 = {
                    let a5815 = v11;
                    let a5816 = v7;
                    let a5817 = 1u64;
                    nat_rem(a5815, a5816, a5817)
                };
                let a5819 = 0u64;
                nat_eq(a5818, a5819)
            } else {
                false
            }
        } else {
            false
        } {
            Ok(Adt21::C2(v5, v4, v9, v10, v11, {
                let a5820 = v10;
                let a5821 = v6;
                let a5822 = 0u64;
                nat_quot(a5820, a5821, a5822)
            }, {
                let a5823 = v11;
                let a5824 = v7;
                let a5825 = 0u64;
                nat_quot(a5823, a5824, a5825)
            }, {
                let a5826 = v5;
                let a5827 = v4;
                nat_mul(a5826, a5827)?
            }, {
                let a5828 = v4;
                let a5829 = v9;
                nat_mul(a5828, a5829)?
            }, {
                let a5830 = v5;
                let a5831 = v9;
                nat_mul(a5830, a5831)?
            }))
        } else {
            Ok(Adt21::C1)
        }
    } else {
        Ok(Adt21::C0)
    }
}

pub fn f269(v0: u64) -> bool {
    let a5832 = v0;
    let a5833 = 0u64;
    if if nat_eq(a5832, a5833) {
        true
    } else {
        let a5834 = v0;
        let a5835 = 1u64;
        nat_eq(a5834, a5835)
    } {
        true
    } else {
        let a5836 = v0;
        let a5837 = 30u64;
        nat_eq(a5836, a5837)
    }
}

pub fn f270(v0: u64, v1: u64, v2: Adt20, v3: Adt20, v4: u64, v5: u64) -> R<Adt19> {
    let v6: u64 = match v2.clone() {
        Adt20::C0(h5838, _, _, _, _, _, _, _) => {
            h5838
        }
    };
    let v7: u64 = {
        let a5841 = 1u64;
        let a5842 = {
            let a5839 = 3u64;
            let a5840 = v6;
            nat_mul(a5839, a5840)?
        };
        nat_add(a5841, a5842)?
    };
    let a5843 = v0;
    let a5844 = 0u64;
    if nat_eq(a5843, a5844) {
        Ok(f281(0u64))
    } else {
        let a5845 = v0;
        let a5846 = v7;
        if nat_lt(a5845, a5846) {
            let v8: u64 = {
                let a5847 = v0;
                let a5848 = 1u64;
                nat_sub(a5847, a5848)
            };
            let v9: u64 = {
                let a5854 = {
                    let a5849 = v1;
                    let a5850 = v6;
                    nat_mul(a5849, a5850)?
                };
                let a5855 = {
                    let a5851 = v8;
                    let a5852 = 3u64;
                    let a5853 = 0u64;
                    nat_quot(a5851, a5852, a5853)
                };
                nat_add(a5854, a5855)?
            };
            let v10: u64 = f282(v9, v2)?;
            let v11: u64 = {
                let a5856 = v8;
                let a5857 = 3u64;
                let a5858 = 0u64;
                nat_rem(a5856, a5857, a5858)
            };
            let a5859 = v11;
            let a5860 = 0u64;
            if nat_eq(a5859, a5860) {
                Ok(f283(10u64, 0u64, v10, 0u64, v10, 1u64, 1u64))
            } else {
                let a5861 = v11;
                let a5862 = 1u64;
                if nat_eq(a5861, a5862) {
                    Ok(f283(1u64, 1u64, 1u64, 2u64, 0u64, 1u64, 1u64))
                } else {
                    Ok(f283(0u64, 1u64, 0u64, 1u64, 1u64, 1u64, 0u64))
                }
            }
        } else {
            let a5863 = v0;
            let a5864 = v7;
            if nat_eq(a5863, a5864) {
                Ok(f283(2u64, 1u64, 0u64, 2u64, 0u64, 1u64, 1u64))
            } else {
                let a5867 = v0;
                let a5868 = {
                    let a5865 = v7;
                    let a5866 = 1u64;
                    nat_add(a5865, a5866)?
                };
                if nat_eq(a5867, a5868) {
                    Ok(f283(12u64, 1u64, 1u64, 2u64, 0u64, 1u64, 1u64))
                } else {
                    let a5871 = v0;
                    let a5872 = {
                        let a5869 = v7;
                        let a5870 = 2u64;
                        nat_add(a5869, a5870)?
                    };
                    if nat_eq(a5871, a5872) {
                        Ok(f283(16u64, 1u64, 1u64, 2u64, v4, 1u64, 2u64))
                    } else {
                        let a5875 = v0;
                        let a5876 = {
                            let a5873 = v7;
                            let a5874 = 3u64;
                            nat_add(a5873, a5874)?
                        };
                        if nat_eq(a5875, a5876) {
                            let a5877 = v5;
                            let a5878 = 1u64;
                            if if nat_eq(a5877, a5878) {
                                true
                            } else {
                                let a5879 = v5;
                                let a5880 = 2u64;
                                nat_eq(a5879, a5880)
                            } {
                                Ok(f283(11u64, 2u64, 1065353216u64, 1u64, 1u64, 1u64, 2u64))
                            } else {
                                Ok(f283(11u64, 2u64, 1065353216u64, 2u64, v4, 1u64, 2u64))
                            }
                        } else {
                            let v12: u64 = {
                                let a5887 = {
                                    let a5881 = v1;
                                    let a5882 = v6;
                                    nat_mul(a5881, a5882)?
                                };
                                let a5888 = {
                                    let a5885 = v0;
                                    let a5886 = {
                                        let a5883 = v7;
                                        let a5884 = 4u64;
                                        nat_add(a5883, a5884)?
                                    };
                                    nat_sub(a5885, a5886)
                                };
                                nat_add(a5887, a5888)?
                            };
                            Ok(f283(10u64, 0u64, f282(v12, v2)?, 1u64, 2u64, 3u64, f282(v12, v3)?))
                        }
                    }
                }
            }
        }
    }
}

pub fn f271(v0: Adt20) -> Adt20 {
    Adt20::C0(match v0.clone() {
        Adt20::C0(h5889, _, _, _, _, _, _, _) => {
            h5889
        }
    }, match v0.clone() {
        Adt20::C0(_, h5890, _, _, _, _, _, _) => {
            h5890
        }
    }, match v0.clone() {
        Adt20::C0(_, _, h5891, _, _, _, _, _) => {
            h5891
        }
    }, match v0.clone() {
        Adt20::C0(_, _, _, h5892, _, _, _, _) => {
            h5892
        }
    }, 4u64, match v0.clone() {
        Adt20::C0(_, _, _, _, _, h5893, _, _) => {
            h5893
        }
    }, match v0.clone() {
        Adt20::C0(_, _, _, _, _, _, h5894, _) => {
            h5894
        }
    }, match v0 {
        Adt20::C0(_, _, _, _, _, _, _, h5895) => {
            h5895
        }
    })
}

pub fn f272(v0: u64, v1: Adt20, v2: u64, v3: u64, v4: u64) -> R<Adt19> {
    let v5: u64 = {
        let a5902 = v3;
        let a5903 = {
            let a5899 = f282({
                let a5896 = v0;
                let a5897 = 2u64;
                let a5898 = 0u64;
                nat_quot(a5896, a5897, a5898)
            }, v1)?;
            let a5900 = v2;
            let a5901 = 0u64;
            nat_quot(a5899, a5900, a5901)
        };
        nat_add(a5902, a5903)?
    };
    let a5907 = {
        let a5904 = v0;
        let a5905 = 2u64;
        let a5906 = 0u64;
        nat_rem(a5904, a5905, a5906)
    };
    let a5908 = 0u64;
    if nat_eq(a5907, a5908) {
        Ok(f283(19u64, 1u64, v5, 2u64, 0u64, 1u64, 2u64))
    } else {
        let a5909 = v4;
        let a5910 = 0u64;
        if nat_eq(a5909, a5910) {
            Ok(f283(19u64, 2u64, 1u64, 2u64, 0u64, 1u64, v5))
        } else {
            Ok(Adt19::C0)
        }
    }
}

pub fn f273(v0: u64, v1: u64, v2: Adt20, v3: Adt20, v4: Adt20, v5: u64) -> R<Adt19> {
    let v6: u64 = f284(match v2.clone() {
        Adt20::C0(h5911, _, _, _, _, _, _, _) => {
            h5911
        }
    })?;
    let v7: u64 = f285(match v2.clone() {
        Adt20::C0(h5912, _, _, _, _, _, _, _) => {
            h5912
        }
    })?;
    let a5913 = v0;
    let a5914 = 2u64;
    if nat_lt(a5913, a5914) {
        Ok(f281(v0))
    } else {
        let a5915 = v0;
        let a5916 = v6;
        if nat_lt(a5915, a5916) {
            f286({
                let a5917 = v0;
                let a5918 = 2u64;
                nat_sub(a5917, a5918)
            }, v1, v2, v3)
        } else {
            let a5919 = v0;
            let a5920 = v7;
            if nat_lt(a5919, a5920) {
                Ok(f287({
                    let a5921 = v0;
                    let a5922 = v6;
                    nat_sub(a5921, a5922)
                }, match v2 {
                    Adt20::C0(h5923, _, _, _, _, _, _, _) => {
                        h5923
                    }
                }, v5))
            } else {
                let a5925 = v0;
                let a5926 = f245(match v2.clone() {
                    Adt20::C0(h5924, _, _, _, _, _, _, _) => {
                        h5924
                    }
                })?;
                if nat_lt(a5925, a5926) {
                    f288({
                        let a5927 = v0;
                        let a5928 = v7;
                        nat_sub(a5927, a5928)
                    }, v1, v2, v3, v4)
                } else {
                    Ok(Adt19::C0)
                }
            }
        }
    }
}

pub fn f274(v0: u64, v1: u64, v2: Adt20, v3: Adt20, v4: u64, v5: bool) -> R<Adt19> {
    let v6: u64 = f259(v5, match v2.clone() {
        Adt20::C0(h5929, _, _, _, _, _, _, _) => {
            h5929
        }
    })?;
    let v7: u64 = {
        let a5931 = v6;
        let a5932 = f260(v5, match v2.clone() {
            Adt20::C0(h5930, _, _, _, _, _, _, _) => {
                h5930
            }
        })?;
        nat_add(a5931, a5932)?
    };
    let v8: u64 = {
        let a5933 = v7;
        let a5934 = f261(v5);
        nat_add(a5933, a5934)?
    };
    let a5935 = v0;
    let a5936 = v6;
    if nat_lt(a5935, a5936) {
        f289(v0, v1, v2, v5)
    } else {
        let a5937 = v0;
        let a5938 = v7;
        if nat_lt(a5937, a5938) {
            f290({
                let a5939 = v0;
                let a5940 = v6;
                nat_sub(a5939, a5940)
            }, v1, v2, v3, v5)
        } else {
            let a5941 = v0;
            let a5942 = v8;
            if nat_lt(a5941, a5942) {
                Ok(f291({
                    let a5943 = v0;
                    let a5944 = v7;
                    nat_sub(a5943, a5944)
                }, match v2 {
                    Adt20::C0(h5945, _, _, _, _, _, _, _) => {
                        h5945
                    }
                }, v4, v5))
            } else {
                let a5949 = v0;
                let a5950 = {
                    let a5947 = v8;
                    let a5948 = match v2.clone() {
                        Adt20::C0(h5946, _, _, _, _, _, _, _) => {
                            h5946
                        }
                    };
                    nat_add(a5947, a5948)?
                };
                if nat_lt(a5949, a5950) {
                    let v9: u64 = f282({
                        let a5956 = {
                            let a5952 = v1;
                            let a5953 = match v2 {
                                Adt20::C0(h5951, _, _, _, _, _, _, _) => {
                                    h5951
                                }
                            };
                            nat_mul(a5952, a5953)?
                        };
                        let a5957 = {
                            let a5954 = v0;
                            let a5955 = v8;
                            nat_sub(a5954, a5955)
                        };
                        nat_add(a5956, a5957)?
                    }, v3)?;
                    Ok(f283(10u64, 3u64, v9, 1u64, 1u64, 3u64, v9))
                } else {
                    Ok(Adt19::C0)
                }
            }
        }
    }
}

pub fn f275(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64) -> R<Adt22> {
    let v6: u64 = f262(v3, v5)?;
    let v7: u64 = f264(v1, v2, v6)?;
    let v8: u64 = {
        let a5960 = {
            let a5958 = f263(v3, v5)?;
            let a5959 = v4;
            nat_mul(a5958, a5959)?
        };
        let a5961 = v7;
        nat_mul(a5960, a5961)?
    };
    let a5962 = v0;
    let a5963 = v8;
    if nat_lt(a5962, a5963) {
        let v9: u64 = {
            let a5964 = v0;
            let a5965 = v7;
            let a5966 = 0u64;
            nat_quot(a5964, a5965, a5966)
        };
        Ok(Adt22::C0({
            let a5970 = {
                let a5967 = v9;
                let a5968 = v4;
                let a5969 = 0u64;
                nat_quot(a5967, a5968, a5969)
            };
            let a5971 = v6;
            nat_mul(a5970, a5971)?
        }, v6, {
            let a5972 = v9;
            let a5973 = v4;
            let a5974 = 0u64;
            nat_rem(a5972, a5973, a5974)
        }, {
            let a5975 = v0;
            let a5976 = v7;
            let a5977 = 0u64;
            nat_rem(a5975, a5976, a5977)
        }))
    } else {
        let v10: u64 = {
            let a5978 = v0;
            let a5979 = v8;
            nat_sub(a5978, a5979)
        };
        let v11: u64 = f264(v1, v2, f265(v3, v5)?)?;
        Ok(Adt22::C0({
            let a5980 = f263(v3, v5)?;
            let a5981 = v6;
            nat_mul(a5980, a5981)?
        }, f265(v3, v5)?, {
            let a5982 = v10;
            let a5983 = v11;
            let a5984 = 0u64;
            nat_quot(a5982, a5983, a5984)
        }, {
            let a5985 = v10;
            let a5986 = v11;
            let a5987 = 0u64;
            nat_rem(a5985, a5986, a5987)
        }))
    }
}

pub fn f276(v0: Adt22, v1: Adt20, v2: Adt20, v3: u64) -> R<Adt19> {
    let v4: u64 = match v0.clone() {
        Adt22::C0(_, _, _, h5988) => {
            h5988
        }
    };
    let v5: u64 = match v1.clone() {
        Adt20::C0(h5989, _, _, _, _, _, _, _) => {
            h5989
        }
    };
    let v6: u64 = {
        let a5992 = match v0.clone() {
            Adt22::C0(_, h5990, _, _) => {
                h5990
            }
        };
        let a5993 = match v1.clone() {
            Adt20::C0(_, h5991, _, _, _, _, _, _) => {
                h5991
            }
        };
        nat_mul(a5992, a5993)?
    };
    let v7: u64 = {
        let a5994 = v6;
        let a5995 = v5;
        nat_mul(a5994, a5995)?
    };
    let v8: u64 = {
        let a5998 = {
            let a5996 = 2u64;
            let a5997 = v5;
            nat_mul(a5996, a5997)?
        };
        let a5999 = 2u64;
        nat_add(a5998, a5999)?
    };
    let v9: u64 = {
        let a6002 = {
            let a6000 = 4u64;
            let a6001 = v5;
            nat_mul(a6000, a6001)?
        };
        let a6003 = 2u64;
        nat_add(a6002, a6003)?
    };
    let v10: u64 = {
        let a6006 = 1u64;
        let a6007 = {
            let a6004 = v6;
            let a6005 = v8;
            nat_mul(a6004, a6005)?
        };
        nat_add(a6006, a6007)?
    };
    let v11: u64 = {
        let a6008 = v10;
        let a6009 = 4u64;
        nat_add(a6008, a6009)?
    };
    let v12: u64 = {
        let a6012 = v11;
        let a6013 = {
            let a6010 = v6;
            let a6011 = v9;
            nat_mul(a6010, a6011)?
        };
        nat_add(a6012, a6013)?
    };
    let v13: u64 = {
        let a6014 = v12;
        let a6015 = 6u64;
        nat_add(a6014, a6015)?
    };
    let a6016 = v4;
    let a6017 = 0u64;
    if nat_eq(a6016, a6017) {
        Ok(f281(0u64))
    } else {
        let a6018 = v4;
        let a6019 = v10;
        if nat_lt(a6018, a6019) {
            let v14: u64 = {
                let a6020 = v4;
                let a6021 = 1u64;
                nat_sub(a6020, a6021)
            };
            f292(false, {
                let a6022 = v14;
                let a6023 = v8;
                let a6024 = 0u64;
                nat_rem(a6022, a6023, a6024)
            }, {
                let a6025 = v14;
                let a6026 = v8;
                let a6027 = 0u64;
                nat_quot(a6025, a6026, a6027)
            }, v0, v1, v2)
        } else {
            let a6028 = v4;
            let a6029 = v11;
            if nat_lt(a6028, a6029) {
                Ok(f293({
                    let a6030 = v4;
                    let a6031 = v10;
                    nat_sub(a6030, a6031)
                }, v7))
            } else {
                let a6032 = v4;
                let a6033 = v12;
                if nat_lt(a6032, a6033) {
                    let v15: u64 = {
                        let a6034 = v4;
                        let a6035 = v11;
                        nat_sub(a6034, a6035)
                    };
                    f292(true, {
                        let a6036 = v15;
                        let a6037 = v9;
                        let a6038 = 0u64;
                        nat_rem(a6036, a6037, a6038)
                    }, {
                        let a6039 = v15;
                        let a6040 = v9;
                        let a6041 = 0u64;
                        nat_quot(a6039, a6040, a6041)
                    }, v0, v1, v2)
                } else {
                    let a6042 = v4;
                    let a6043 = v13;
                    if nat_lt(a6042, a6043) {
                        Ok(f294({
                            let a6044 = v4;
                            let a6045 = v12;
                            nat_sub(a6044, a6045)
                        }, v7, v3))
                    } else {
                        let a6048 = v4;
                        let a6049 = {
                            let a6046 = v13;
                            let a6047 = v7;
                            nat_add(a6046, a6047)?
                        };
                        if nat_lt(a6048, a6049) {
                            let v16: u64 = {
                                let a6050 = v4;
                                let a6051 = v13;
                                nat_sub(a6050, a6051)
                            };
                            let v17: u64 = {
                                let a6052 = v16;
                                let a6053 = v5;
                                let a6054 = 0u64;
                                nat_quot(a6052, a6053, a6054)
                            };
                            let v18: u64 = {
                                let a6060 = {
                                    let a6055 = f295(v17, v0, v1)?;
                                    let a6056 = v5;
                                    nat_mul(a6055, a6056)?
                                };
                                let a6061 = {
                                    let a6057 = v16;
                                    let a6058 = v5;
                                    let a6059 = 0u64;
                                    nat_rem(a6057, a6058, a6059)
                                };
                                nat_add(a6060, a6061)?
                            };
                            let v19: u64 = f282(v18, v2)?;
                            Ok(f283(10u64, 3u64, v19, 1u64, 1u64, 3u64, v19))
                        } else {
                            Ok(Adt19::C0)
                        }
                    }
                }
            }
        }
    }
}

pub fn f277(v0: bool) -> u64 {
    if v0 {
        2u64
    } else {
        1u64
    }
}

pub fn f278(v0: u64) -> u64 {
    let a6062 = v0;
    let a6063 = 4u64;
    let a6064 = 0u64;
    nat_quot(a6062, a6063, a6064)
}

pub fn f279(v0: u64) -> R<u64> {
    let a6067 = {
        let a6065 = 7u64;
        let a6066 = v0;
        nat_mul(a6065, a6066)?
    };
    let a6068 = 4u64;
    nat_add(a6067, a6068)
}

pub fn f280(v0: u64) -> u64 {
    if f128(v0) {
        let m6069 = f130(v0);
        match m6069 {
            Adt17::C0 => {
                0u64
            }
            Adt17::C1(v1) => {
                let a6071 = 1u64;
                let a6072 = match v1.clone() {
                    Adt18::C0(_, h6070, _) => {
                        h6070
                    }
                };
                if if nat_lt(a6071, a6072) {
                    true
                } else {
                    let a6073 = v0;
                    let a6074 = 43u64;
                    nat_eq(a6073, a6074)
                } {
                    match v1 {
                        Adt18::C0(_, h6075, _) => {
                            h6075
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

pub fn f281(v0: u64) -> Adt19 {
    f283(0u64, 2u64, 0u64, 2u64, 0u64, 1u64, v0)
}

pub fn f282(v0: u64, v1: Adt20) -> R<u64> {
    let v2: u64 = {
        let a6077 = v0;
        let a6078 = match v1.clone() {
            Adt20::C0(h6076, _, _, _, _, _, _, _) => {
                h6076
            }
        };
        let a6079 = 0u64;
        nat_quot(a6077, a6078, a6079)
    };
    let v3: u64 = {
        let a6081 = v2;
        let a6082 = match v1.clone() {
            Adt20::C0(_, h6080, _, _, _, _, _, _) => {
                h6080
            }
        };
        let a6083 = 0u64;
        nat_quot(a6081, a6082, a6083)
    };
    f147({
        let a6085 = v0;
        let a6086 = match v1.clone() {
            Adt20::C0(h6084, _, _, _, _, _, _, _) => {
                h6084
            }
        };
        let a6087 = 0u64;
        nat_rem(a6085, a6086, a6087)
    }, {
        let a6089 = v2;
        let a6090 = match v1.clone() {
            Adt20::C0(_, h6088, _, _, _, _, _, _) => {
                h6088
            }
        };
        let a6091 = 0u64;
        nat_rem(a6089, a6090, a6091)
    }, {
        let a6093 = v3;
        let a6094 = match v1.clone() {
            Adt20::C0(_, _, h6092, _, _, _, _, _) => {
                h6092
            }
        };
        let a6095 = 0u64;
        nat_rem(a6093, a6094, a6095)
    }, {
        let a6097 = v3;
        let a6098 = match v1.clone() {
            Adt20::C0(_, _, h6096, _, _, _, _, _) => {
                h6096
            }
        };
        let a6099 = 0u64;
        nat_quot(a6097, a6098, a6099)
    }, match v1.clone() {
        Adt20::C0(_, _, _, _, h6100, _, _, _) => {
            h6100
        }
    }, match v1.clone() {
        Adt20::C0(_, _, _, _, _, h6101, _, _) => {
            h6101
        }
    }, match v1.clone() {
        Adt20::C0(_, _, _, _, _, _, h6102, _) => {
            h6102
        }
    }, match v1 {
        Adt20::C0(_, _, _, _, _, _, _, h6103) => {
            h6103
        }
    })
}

pub fn f283(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64) -> Adt19 {
    let a6104 = v0;
    let a6105 = 4u64;
    if if if nat_le(a6104, a6105) {
        true
    } else {
        let a6106 = 9u64;
        let a6107 = v0;
        if nat_le(a6106, a6107) {
            let a6108 = v0;
            let a6109 = 12u64;
            nat_le(a6108, a6109)
        } else {
            false
        }
    } {
        true
    } else {
        let a6110 = v0;
        let a6111 = 16u64;
        if if nat_eq(a6110, a6111) {
            true
        } else {
            let a6112 = v0;
            let a6113 = 19u64;
            nat_eq(a6112, a6113)
        } {
            true
        } else {
            let a6114 = v0;
            let a6115 = 21u64;
            if nat_eq(a6114, a6115) {
                true
            } else {
                let a6116 = v0;
                let a6117 = 22u64;
                nat_eq(a6116, a6117)
            }
        }
    } {
        Adt19::C1({
            let a6118 = v0;
            let a6119 = 4u64;
            if if nat_lt(a6118, a6119) {
                true
            } else {
                let a6120 = 21u64;
                let a6121 = v0;
                nat_le(a6120, a6121)
            } {
                0u64
            } else {
                let a6122 = v0;
                let a6123 = 16u64;
                if if nat_eq(a6122, a6123) {
                    true
                } else {
                    let a6124 = v0;
                    let a6125 = 19u64;
                    nat_eq(a6124, a6125)
                } {
                    2u64
                } else {
                    1u64
                }
            }
        }, {
            let a6126 = v0;
            let a6127 = 4u64;
            if nat_lt(a6126, a6127) {
                v0
            } else {
                let a6128 = v0;
                let a6129 = 4u64;
                if if nat_eq(a6128, a6129) {
                    true
                } else {
                    let a6130 = v0;
                    let a6131 = 16u64;
                    nat_eq(a6130, a6131)
                } {
                    0u64
                } else {
                    let a6132 = v0;
                    let a6133 = 19u64;
                    if nat_eq(a6132, a6133) {
                        3u64
                    } else {
                        let a6134 = 21u64;
                        let a6135 = v0;
                        if nat_le(a6134, a6135) {
                            let a6136 = v0;
                            let a6137 = 17u64;
                            nat_sub(a6136, a6137)
                        } else {
                            let a6138 = v0;
                            let a6139 = 8u64;
                            nat_sub(a6138, a6139)
                        }
                    }
                }
            }
        }, v1, v2, v3, v4, v5, v6)
    } else {
        Adt19::C0
    }
}

pub fn f284(v0: u64) -> R<u64> {
    let a6142 = 2u64;
    let a6143 = {
        let a6140 = 6u64;
        let a6141 = v0;
        nat_mul(a6140, a6141)?
    };
    nat_add(a6142, a6143)
}

pub fn f285(v0: u64) -> R<u64> {
    let a6144 = f284(v0)?;
    let a6145 = 11u64;
    nat_add(a6144, a6145)
}

pub fn f286(v0: u64, v1: u64, v2: Adt20, v3: Adt20) -> R<Adt19> {
    let v4: u64 = {
        let a6146 = v0;
        let a6147 = 6u64;
        let a6148 = 0u64;
        nat_rem(a6146, a6147, a6148)
    };
    let v5: u64 = {
        let a6155 = {
            let a6150 = v1;
            let a6151 = match v2.clone() {
                Adt20::C0(h6149, _, _, _, _, _, _, _) => {
                    h6149
                }
            };
            nat_mul(a6150, a6151)?
        };
        let a6156 = {
            let a6152 = v0;
            let a6153 = 6u64;
            let a6154 = 0u64;
            nat_quot(a6152, a6153, a6154)
        };
        nat_add(a6155, a6156)?
    };
    let v6: u64 = f282(v5, v3)?;
    let v7: u64 = f282(v5, v2)?;
    let a6157 = v4;
    let a6158 = 0u64;
    if nat_eq(a6157, a6158) {
        Ok(f283(10u64, 4u64, v6, 4u64, v6, 1u64, 2u64))
    } else {
        let a6159 = v4;
        let a6160 = 1u64;
        if nat_eq(a6159, a6160) {
            Ok(f283(1u64, 1u64, 2u64, 2u64, 0u64, 1u64, 2u64))
        } else {
            let a6161 = v4;
            let a6162 = 2u64;
            if nat_eq(a6161, a6162) {
                Ok(f283(0u64, 1u64, 0u64, 1u64, 2u64, 1u64, 0u64))
            } else {
                let a6163 = v4;
                let a6164 = 3u64;
                if nat_eq(a6163, a6164) {
                    Ok(f283(10u64, 4u64, v6, 0u64, v7, 1u64, 2u64))
                } else {
                    let a6165 = v4;
                    let a6166 = 4u64;
                    if nat_eq(a6165, a6166) {
                        Ok(f283(1u64, 1u64, 2u64, 2u64, 0u64, 1u64, 2u64))
                    } else {
                        Ok(f283(0u64, 1u64, 1u64, 1u64, 2u64, 1u64, 1u64))
                    }
                }
            }
        }
    }
}

pub fn f287(v0: u64, v1: u64, v2: u64) -> Adt19 {
    let a6167 = v0;
    let a6168 = 0u64;
    if nat_eq(a6167, a6168) {
        f283(2u64, 1u64, 0u64, 2u64, 0u64, 1u64, 3u64)
    } else {
        let a6169 = v0;
        let a6170 = 1u64;
        if nat_eq(a6169, a6170) {
            f283(3u64, 2u64, v1, 2u64, 0u64, 1u64, 2u64)
        } else {
            let a6171 = v0;
            let a6172 = 2u64;
            if nat_eq(a6171, a6172) {
                f283(11u64, 1u64, 3u64, 1u64, 2u64, 1u64, 4u64)
            } else {
                let a6173 = v0;
                let a6174 = 3u64;
                if nat_eq(a6173, a6174) {
                    f283(4u64, 1u64, 4u64, 2u64, v2, 1u64, 4u64)
                } else {
                    let a6175 = v0;
                    let a6176 = 4u64;
                    if nat_eq(a6175, a6176) {
                        f283(12u64, 1u64, 4u64, 2u64, 0u64, 1u64, 4u64)
                    } else {
                        let a6177 = v0;
                        let a6178 = 5u64;
                        if nat_eq(a6177, a6178) {
                            f283(11u64, 2u64, 1065353216u64, 1u64, 4u64, 1u64, 4u64)
                        } else {
                            let a6179 = v0;
                            let a6180 = 6u64;
                            if nat_eq(a6179, a6180) {
                                f283(10u64, 2u64, v2, 1u64, 2u64, 1u64, 5u64)
                            } else {
                                let a6181 = v0;
                                let a6182 = 7u64;
                                if nat_eq(a6181, a6182) {
                                    f283(4u64, 1u64, 3u64, 1u64, 5u64, 1u64, 5u64)
                                } else {
                                    let a6183 = v0;
                                    let a6184 = 8u64;
                                    if nat_eq(a6183, a6184) {
                                        f283(2u64, 1u64, 1u64, 2u64, 0u64, 1u64, 2u64)
                                    } else {
                                        let a6185 = v0;
                                        let a6186 = 9u64;
                                        if nat_eq(a6185, a6186) {
                                            Adt19::C1(3u64, 0u64, 1u64, 2u64, 2u64, 2u64, 1u64, 2u64)
                                        } else {
                                            let a6187 = v0;
                                            let a6188 = 10u64;
                                            if nat_eq(a6187, a6188) {
                                                f283(11u64, 1u64, 2u64, 1u64, 5u64, 1u64, 5u64)
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

pub fn f288(v0: u64, v1: u64, v2: Adt20, v3: Adt20, v4: Adt20) -> R<Adt19> {
    let v5: u64 = {
        let a6189 = v0;
        let a6190 = 3u64;
        let a6191 = 0u64;
        nat_rem(a6189, a6190, a6191)
    };
    let v6: u64 = {
        let a6198 = {
            let a6193 = v1;
            let a6194 = match v2.clone() {
                Adt20::C0(h6192, _, _, _, _, _, _, _) => {
                    h6192
                }
            };
            nat_mul(a6193, a6194)?
        };
        let a6199 = {
            let a6195 = v0;
            let a6196 = 3u64;
            let a6197 = 0u64;
            nat_quot(a6195, a6196, a6197)
        };
        nat_add(a6198, a6199)?
    };
    let v7: u64 = f282(v6, v3)?;
    let v8: u64 = f282(v6, v2)?;
    let v9: u64 = f282(v6, v4)?;
    let a6200 = v5;
    let a6201 = 0u64;
    if nat_eq(a6200, a6201) {
        Ok(f283(10u64, 4u64, v7, 1u64, 5u64, 1u64, 2u64))
    } else {
        let a6202 = v5;
        let a6203 = 1u64;
        if nat_eq(a6202, a6203) {
            Ok(f283(4u64, 0u64, v8, 1u64, 2u64, 1u64, 2u64))
        } else {
            Ok(f283(10u64, 1u64, 2u64, 1u64, 4u64, 3u64, v9))
        }
    }
}

pub fn f289(v0: u64, v1: u64, v2: Adt20, v3: bool) -> R<Adt19> {
    let v4: u64 = match v2.clone() {
        Adt20::C0(h6204, _, _, _, _, _, _, _) => {
            h6204
        }
    };
    let v5: u64 = {
        let a6207 = 1u64;
        let a6208 = {
            let a6205 = v4;
            let a6206 = f277(v3);
            nat_mul(a6205, a6206)?
        };
        nat_add(a6207, a6208)?
    };
    let a6209 = v0;
    let a6210 = 0u64;
    if nat_eq(a6209, a6210) {
        Ok(f281(0u64))
    } else {
        let a6211 = v0;
        let a6212 = v5;
        if nat_lt(a6211, a6212) {
            let v6: u64 = {
                let a6213 = v0;
                let a6214 = 1u64;
                nat_sub(a6213, a6214)
            };
            let v7: u64 = f282({
                let a6220 = {
                    let a6215 = v1;
                    let a6216 = v4;
                    nat_mul(a6215, a6216)?
                };
                let a6221 = {
                    let a6217 = v6;
                    let a6218 = f277(v3);
                    let a6219 = 0u64;
                    nat_quot(a6217, a6218, a6219)
                };
                nat_add(a6220, a6221)?
            }, v2)?;
            if v3 {
                let a6225 = {
                    let a6222 = v6;
                    let a6223 = 2u64;
                    let a6224 = 0u64;
                    nat_rem(a6222, a6223, a6224)
                };
                let a6226 = 0u64;
                if nat_eq(a6225, a6226) {
                    Ok(f283(1u64, 0u64, v7, 2u64, 0u64, 1u64, 1u64))
                } else {
                    Ok(f283(0u64, 1u64, 0u64, 1u64, 1u64, 1u64, 0u64))
                }
            } else {
                Ok(f283(4u64, 1u64, 0u64, 0u64, v7, 1u64, 0u64))
            }
        } else {
            let v8: u64 = {
                let a6227 = v0;
                let a6228 = v5;
                nat_sub(a6227, a6228)
            };
            let a6229 = v8;
            let a6230 = 0u64;
            if nat_eq(a6229, a6230) {
                Ok(f283(3u64, 2u64, v4, 2u64, 0u64, 1u64, 1u64))
            } else {
                if if v3 {
                    let a6231 = v8;
                    let a6232 = 1u64;
                    nat_eq(a6231, a6232)
                } else {
                    false
                } {
                    Ok(f283(2u64, 1u64, 0u64, 2u64, 0u64, 1u64, 0u64))
                } else {
                    let a6233 = v8;
                    let a6234 = if v3 {
                        2u64
                    } else {
                        1u64
                    };
                    if nat_eq(a6233, a6234) {
                        Ok(f283(11u64, 1u64, 0u64, 1u64, 1u64, 1u64, 3u64))
                    } else {
                        let a6235 = v8;
                        let a6236 = if v3 {
                            3u64
                        } else {
                            2u64
                        };
                        if nat_eq(a6235, a6236) {
                            Ok(f281(0u64))
                        } else {
                            Ok(Adt19::C0)
                        }
                    }
                }
            }
        }
    }
}

pub fn f290(v0: u64, v1: u64, v2: Adt20, v3: Adt20, v4: bool) -> R<Adt19> {
    let v5: u64 = if v4 {
        let a6238 = 13u64;
        let a6239 = f278(match v2.clone() {
            Adt20::C0(h6237, _, _, _, _, _, _, _) => {
                h6237
            }
        });
        nat_mul(a6238, a6239)?
    } else {
        0u64
    };
    let a6240 = v0;
    let a6241 = v5;
    if nat_lt(a6240, a6241) {
        f296({
            let a6242 = v0;
            let a6243 = 13u64;
            let a6244 = 0u64;
            nat_rem(a6242, a6243, a6244)
        }, v1, {
            let a6245 = v0;
            let a6246 = 13u64;
            let a6247 = 0u64;
            nat_quot(a6245, a6246, a6247)
        }, v2, v3)
    } else {
        let v6: u64 = {
            let a6248 = v0;
            let a6249 = v5;
            nat_sub(a6248, a6249)
        };
        let v7: u64 = if v4 {
            4u64
        } else {
            3u64
        };
        f297({
            let a6250 = v6;
            let a6251 = v7;
            let a6252 = 0u64;
            nat_rem(a6250, a6251, a6252)
        }, {
            let a6262 = {
                let a6257 = {
                    let a6254 = v1;
                    let a6255 = match v2.clone() {
                        Adt20::C0(h6253, _, _, _, _, _, _, _) => {
                            h6253
                        }
                    };
                    nat_mul(a6254, a6255)?
                };
                let a6258 = if v4 {
                    f298(match v2.clone() {
                        Adt20::C0(h6256, _, _, _, _, _, _, _) => {
                            h6256
                        }
                    })?
                } else {
                    0u64
                };
                nat_add(a6257, a6258)?
            };
            let a6263 = {
                let a6259 = v6;
                let a6260 = v7;
                let a6261 = 0u64;
                nat_quot(a6259, a6260, a6261)
            };
            nat_add(a6262, a6263)?
        }, v2, v3, v4)
    }
}

pub fn f291(v0: u64, v1: u64, v2: u64, v3: bool) -> Adt19 {
    let a6264 = v0;
    let a6265 = 0u64;
    if nat_eq(a6264, a6265) {
        f283(if v3 {
            22u64
        } else {
            3u64
        }, 2u64, v1, 2u64, 0u64, 1u64, 1u64)
    } else {
        let a6266 = v0;
        let a6267 = 1u64;
        if nat_eq(a6266, a6267) {
            f283(if v3 {
                21u64
            } else {
                11u64
            }, 1u64, 0u64, 1u64, 1u64, 1u64, 1u64)
        } else {
            if if v3 {
                let a6268 = v0;
                let a6269 = 2u64;
                nat_eq(a6268, a6269)
            } else {
                false
            } {
                f283(2u64, 1u64, 1u64, 2u64, 0u64, 1u64, 1u64)
            } else {
                let a6270 = v0;
                let a6271 = if v3 {
                    3u64
                } else {
                    2u64
                };
                if nat_eq(a6270, a6271) {
                    f283(4u64, 1u64, 1u64, 2u64, v2, 1u64, 1u64)
                } else {
                    let a6272 = v0;
                    let a6273 = if v3 {
                        4u64
                    } else {
                        3u64
                    };
                    if nat_eq(a6272, a6273) {
                        f283(12u64, 1u64, 1u64, 2u64, 0u64, 1u64, 1u64)
                    } else {
                        let a6274 = v0;
                        let a6275 = if v3 {
                            5u64
                        } else {
                            4u64
                        };
                        if nat_eq(a6274, a6275) {
                            f283(11u64, 2u64, 1065353216u64, 1u64, 1u64, 1u64, 1u64)
                        } else {
                            Adt19::C0
                        }
                    }
                }
            }
        }
    }
}

pub fn f292(v0: bool, v1: u64, v2: u64, v3: Adt22, v4: Adt20, v5: Adt20) -> R<Adt19> {
    let v6: u64 = if v0 {
        4u64
    } else {
        2u64
    };
    let v7: u64 = {
        let a6279 = 1u64;
        let a6280 = {
            let a6277 = match v4.clone() {
                Adt20::C0(h6276, _, _, _, _, _, _, _) => {
                    h6276
                }
            };
            let a6278 = v6;
            nat_mul(a6277, a6278)?
        };
        nat_add(a6279, a6280)?
    };
    let a6281 = v1;
    let a6282 = 0u64;
    if nat_eq(a6281, a6282) {
        Ok(f281(1u64))
    } else {
        let a6283 = v1;
        let a6284 = v7;
        if nat_lt(a6283, a6284) {
            let v8: u64 = {
                let a6285 = v1;
                let a6286 = 1u64;
                nat_sub(a6285, a6286)
            };
            let v9: u64 = {
                let a6293 = {
                    let a6288 = f295(v2, v3, v4.clone())?;
                    let a6289 = match v4.clone() {
                        Adt20::C0(h6287, _, _, _, _, _, _, _) => {
                            h6287
                        }
                    };
                    nat_mul(a6288, a6289)?
                };
                let a6294 = {
                    let a6290 = v8;
                    let a6291 = v6;
                    let a6292 = 0u64;
                    nat_quot(a6290, a6291, a6292)
                };
                nat_add(a6293, a6294)?
            };
            let v10: u64 = f282(v9, v4)?;
            let v11: u64 = f282(v9, v5)?;
            let v12: u64 = {
                let a6295 = v8;
                let a6296 = v6;
                let a6297 = 0u64;
                nat_rem(a6295, a6296, a6297)
            };
            if v0 {
                let a6298 = v12;
                let a6299 = 0u64;
                if nat_eq(a6298, a6299) {
                    Ok(f283(9u64, 0u64, v10, 1u64, 3u64, 3u64, v11))
                } else {
                    let a6300 = v12;
                    let a6301 = 1u64;
                    if nat_eq(a6300, a6301) {
                        Ok(f283(10u64, 3u64, v11, 3u64, v11, 1u64, 2u64))
                    } else {
                        let a6302 = v12;
                        let a6303 = 2u64;
                        if nat_eq(a6302, a6303) {
                            Ok(f283(1u64, 1u64, 2u64, 2u64, 0u64, 1u64, 2u64))
                        } else {
                            Ok(f283(0u64, 1u64, 1u64, 1u64, 2u64, 1u64, 1u64))
                        }
                    }
                }
            } else {
                let a6304 = v12;
                let a6305 = 0u64;
                if nat_eq(a6304, a6305) {
                    Ok(f283(1u64, 0u64, v10, 2u64, 0u64, 1u64, 2u64))
                } else {
                    Ok(f283(0u64, 1u64, 1u64, 1u64, 2u64, 1u64, 1u64))
                }
            }
        } else {
            let a6306 = v1;
            let a6307 = v7;
            if nat_eq(a6306, a6307) {
                Ok(f283(0u64, 1u64, 0u64, 1u64, 1u64, 1u64, 0u64))
            } else {
                Ok(Adt19::C0)
            }
        }
    }
}

pub fn f293(v0: u64, v1: u64) -> Adt19 {
    let a6308 = v0;
    let a6309 = 0u64;
    if nat_eq(a6308, a6309) {
        f283(22u64, 2u64, v1, 2u64, 0u64, 1u64, 2u64)
    } else {
        let a6310 = v0;
        let a6311 = 1u64;
        if nat_eq(a6310, a6311) {
            f283(21u64, 1u64, 0u64, 1u64, 2u64, 1u64, 3u64)
        } else {
            let a6312 = v0;
            let a6313 = 2u64;
            if nat_eq(a6312, a6313) {
                f283(2u64, 1u64, 3u64, 2u64, 0u64, 1u64, 3u64)
            } else {
                let a6314 = v0;
                let a6315 = 3u64;
                if nat_eq(a6314, a6315) {
                    f281(0u64)
                } else {
                    Adt19::C0
                }
            }
        }
    }
}

pub fn f294(v0: u64, v1: u64, v2: u64) -> Adt19 {
    let a6316 = v0;
    let a6317 = 0u64;
    if nat_eq(a6316, a6317) {
        f283(22u64, 2u64, v1, 2u64, 0u64, 1u64, 2u64)
    } else {
        let a6318 = v0;
        let a6319 = 1u64;
        if nat_eq(a6318, a6319) {
            f283(21u64, 1u64, 0u64, 1u64, 2u64, 1u64, 2u64)
        } else {
            let a6320 = v0;
            let a6321 = 2u64;
            if nat_eq(a6320, a6321) {
                f283(2u64, 1u64, 2u64, 2u64, 0u64, 1u64, 2u64)
            } else {
                let a6322 = v0;
                let a6323 = 3u64;
                if nat_eq(a6322, a6323) {
                    f283(4u64, 1u64, 2u64, 2u64, v2, 1u64, 1u64)
                } else {
                    let a6324 = v0;
                    let a6325 = 4u64;
                    if nat_eq(a6324, a6325) {
                        f283(12u64, 1u64, 1u64, 2u64, 0u64, 1u64, 1u64)
                    } else {
                        let a6326 = v0;
                        let a6327 = 5u64;
                        if nat_eq(a6326, a6327) {
                            f283(11u64, 2u64, 1065353216u64, 1u64, 1u64, 1u64, 1u64)
                        } else {
                            Adt19::C0
                        }
                    }
                }
            }
        }
    }
}

pub fn f295(v0: u64, v1: Adt22, v2: Adt20) -> R<u64> {
    let a6338 = {
        let a6336 = {
            let a6333 = {
                let a6330 = match v1.clone() {
                    Adt22::C0(_, _, h6328, _) => {
                        h6328
                    }
                };
                let a6331 = match v2.clone() {
                    Adt20::C0(_, _, h6329, _, _, _, _, _) => {
                        h6329
                    }
                };
                nat_mul(a6330, a6331)?
            };
            let a6334 = match v1 {
                Adt22::C0(h6332, _, _, _) => {
                    h6332
                }
            };
            nat_add(a6333, a6334)?
        };
        let a6337 = match v2 {
            Adt20::C0(_, h6335, _, _, _, _, _, _) => {
                h6335
            }
        };
        nat_mul(a6336, a6337)?
    };
    let a6339 = v0;
    nat_add(a6338, a6339)
}

pub fn f296(v0: u64, v1: u64, v2: u64, v3: Adt20, v4: Adt20) -> R<Adt19> {
    let v5: u64 = {
        let a6347 = {
            let a6345 = {
                let a6341 = v1;
                let a6342 = match v3.clone() {
                    Adt20::C0(h6340, _, _, _, _, _, _, _) => {
                        h6340
                    }
                };
                nat_mul(a6341, a6342)?
            };
            let a6346 = {
                let a6343 = v2;
                let a6344 = 4u64;
                nat_mul(a6343, a6344)?
            };
            nat_add(a6345, a6346)?
        };
        let a6348 = f299(v0);
        nat_add(a6347, a6348)?
    };
    let v6: u64 = f282(v5, v3)?;
    let v7: u64 = f282(v5, v4)?;
    let a6349 = v0;
    let a6350 = 0u64;
    if if if if nat_eq(a6349, a6350) {
        true
    } else {
        let a6351 = v0;
        let a6352 = 2u64;
        nat_eq(a6351, a6352)
    } {
        true
    } else {
        let a6353 = v0;
        let a6354 = 5u64;
        nat_eq(a6353, a6354)
    } {
        true
    } else {
        let a6355 = v0;
        let a6356 = 7u64;
        nat_eq(a6355, a6356)
    } {
        Ok(f283(9u64, 0u64, v6, 1u64, 3u64, 3u64, v7))
    } else {
        let a6357 = v0;
        let a6358 = 1u64;
        if if if if nat_eq(a6357, a6358) {
            true
        } else {
            let a6359 = v0;
            let a6360 = 3u64;
            nat_eq(a6359, a6360)
        } {
            true
        } else {
            let a6361 = v0;
            let a6362 = 6u64;
            nat_eq(a6361, a6362)
        } {
            true
        } else {
            let a6363 = v0;
            let a6364 = 8u64;
            nat_eq(a6363, a6364)
        } {
            Ok(f283(10u64, 3u64, v7, 3u64, v7, 1u64, {
                let a6365 = v0;
                let a6366 = 1u64;
                if nat_eq(a6365, a6366) {
                    4u64
                } else {
                    let a6367 = v0;
                    let a6368 = 6u64;
                    if nat_eq(a6367, a6368) {
                        5u64
                    } else {
                        1u64
                    }
                }
            }))
        } else {
            let a6369 = v0;
            let a6370 = 4u64;
            if nat_eq(a6369, a6370) {
                Ok(f283(4u64, 1u64, 4u64, 1u64, 1u64, 1u64, 4u64))
            } else {
                let a6371 = v0;
                let a6372 = 9u64;
                if nat_eq(a6371, a6372) {
                    Ok(f283(4u64, 1u64, 5u64, 1u64, 1u64, 1u64, 5u64))
                } else {
                    let a6373 = v0;
                    let a6374 = 10u64;
                    if nat_eq(a6373, a6374) {
                        Ok(f283(4u64, 1u64, 4u64, 1u64, 5u64, 1u64, 1u64))
                    } else {
                        let a6375 = v0;
                        let a6376 = 11u64;
                        if nat_eq(a6375, a6376) {
                            Ok(f283(1u64, 1u64, 1u64, 2u64, 0u64, 1u64, 1u64))
                        } else {
                            let a6377 = v0;
                            let a6378 = 12u64;
                            if nat_eq(a6377, a6378) {
                                Ok(f283(0u64, 1u64, 0u64, 1u64, 1u64, 1u64, 0u64))
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

pub fn f297(v0: u64, v1: u64, v2: Adt20, v3: Adt20, v4: bool) -> R<Adt19> {
    let v5: u64 = f282(v1, v2)?;
    let v6: u64 = f282(v1, v3)?;
    let a6379 = v0;
    let a6380 = 0u64;
    if nat_eq(a6379, a6380) {
        Ok(f283(9u64, 0u64, v5, 1u64, 3u64, 3u64, v6))
    } else {
        let a6381 = v0;
        let a6382 = 1u64;
        if nat_eq(a6381, a6382) {
            Ok(f283(10u64, 3u64, v6, 3u64, v6, 1u64, 1u64))
        } else {
            if v4 {
                let a6383 = v0;
                let a6384 = 2u64;
                if nat_eq(a6383, a6384) {
                    Ok(f283(1u64, 1u64, 1u64, 2u64, 0u64, 1u64, 1u64))
                } else {
                    let a6385 = v0;
                    let a6386 = 3u64;
                    if nat_eq(a6385, a6386) {
                        Ok(f283(0u64, 1u64, 0u64, 1u64, 1u64, 1u64, 0u64))
                    } else {
                        Ok(Adt19::C0)
                    }
                }
            } else {
                let a6387 = v0;
                let a6388 = 2u64;
                if nat_eq(a6387, a6388) {
                    Ok(f283(4u64, 1u64, 0u64, 1u64, 1u64, 1u64, 0u64))
                } else {
                    Ok(Adt19::C0)
                }
            }
        }
    }
}

pub fn f298(v0: u64) -> R<u64> {
    let a6389 = 4u64;
    let a6390 = f278(v0);
    nat_mul(a6389, a6390)
}

pub fn f299(v0: u64) -> u64 {
    let a6391 = v0;
    let a6392 = 5u64;
    if nat_lt(a6391, a6392) {
        let a6393 = v0;
        let a6394 = 2u64;
        if nat_lt(a6393, a6394) {
            0u64
        } else {
            1u64
        }
    } else {
        let a6395 = v0;
        let a6396 = 7u64;
        if nat_lt(a6395, a6396) {
            2u64
        } else {
            3u64
        }
    }
}

pub fn run(p0: Adt0) -> R<Adt9> {
    f0(p0)
}
