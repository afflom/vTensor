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
            let a3 = 87u64;
            if nat_eq(a2, a3) {
                Ok(Adt9::C0(f1(v3, v4, v5, v6, v7, v8)?))
            } else {
                let a4 = v1;
                let a5 = 64u64;
                if nat_eq(a4, a5) {
                    Ok(Adt9::C0(f2(v2, v8)?))
                } else {
                    let m6 = v3;
                    match m6 {
                        None => {
                            Ok(Adt9::C0(Adt10::C1))
                        }
                        Some(v9) => {
                            let a7 = v1;
                            let a8 = 48u64;
                            if if nat_eq(a7, a8) {
                                true
                            } else {
                                let a9 = v1;
                                let a10 = 56u64;
                                nat_eq(a9, a10)
                            } {
                                Ok(Adt9::C0(f3({
                                    let a11 = v1;
                                    let a12 = 56u64;
                                    nat_eq(a11, a12)
                                }, v2, v9, v4, v8)?))
                            } else {
                                let a13 = v1;
                                let a14 = 53u64;
                                if if nat_eq(a13, a14) {
                                    true
                                } else {
                                    let a15 = v1;
                                    let a16 = 54u64;
                                    nat_eq(a15, a16)
                                } {
                                    Ok(Adt9::C0(f4({
                                        let a17 = v1;
                                        let a18 = 54u64;
                                        nat_eq(a17, a18)
                                    }, v2, v9, v4, v8)?))
                                } else {
                                    let a19 = v1;
                                    let a20 = 78u64;
                                    if nat_eq(a19, a20) {
                                        Ok(Adt9::C0(f5(v2, v9, v4, v5, v8)?))
                                    } else {
                                        let a21 = v1;
                                        let a22 = 23u64;
                                        if if if nat_eq(a21, a22) {
                                            true
                                        } else {
                                            let a23 = v1;
                                            let a24 = 26u64;
                                            nat_eq(a23, a24)
                                        } {
                                            true
                                        } else {
                                            let a25 = v1;
                                            let a26 = 25u64;
                                            nat_eq(a25, a26)
                                        } {
                                            Ok(Adt9::C0(f6(v1, v2, v9, v4, v8)?))
                                        } else {
                                            let a27 = v1;
                                            let a28 = 24u64;
                                            if nat_eq(a27, a28) {
                                                Ok(Adt9::C0(f7(v2, v9, v8)?))
                                            } else {
                                                let a29 = v1;
                                                let a30 = 27u64;
                                                if nat_eq(a29, a30) {
                                                    Ok(Adt9::C0(f8(v2, v9, v8)?))
                                                } else {
                                                    let a31 = v1;
                                                    let a32 = 3u64;
                                                    if nat_eq(a31, a32) {
                                                        Ok(Adt9::C0(f9(v9, v4, v5, v8)?))
                                                    } else {
                                                        f10(v1, v2, v9, v4, v8)
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        Adt0::C1(v15, v16, v17, v18, v19) => {
            f13(v15, v16, v17, v18, v19)
        }
        Adt0::C2(v30) => {
            f15(v30)
        }
        Adt0::C3(v31, v32, v33, v34, v35, v36, v37, v38, v39, v40, v41, v42, v43) => {
            Ok(f16(v31, v32, v33, v34, v35, v36, v37, v38, v39, v40, v41, v42, v43))
        }
        Adt0::C4(v10, v11, v12, v13) => {
            let a34 = 0u64;
            let a35 = match v10.clone() {
                Adt8::C0(_, _, _, _, _, _, _, h33, _, _, _, _, _, _, _) => {
                    h33
                }
            };
            if nat_lt(a34, a35) {
                f11(v10, v11, v12, v13)
            } else {
                f12(v10, v11, v12, v13)
            }
        }
        Adt0::C5(v14) => {
            Ok(Adt9::C9({
                let a36 = 3240099840u64;
                let a37 = v14;
                if if nat_le(a36, a37) {
                    let a38 = v14;
                    let a39 = 4286578688u64;
                    nat_le(a38, a39)
                } else {
                    false
                } {
                    Some(0u64)
                } else {
                    let a40 = 1092616192u64;
                    let a41 = v14;
                    if if nat_le(a40, a41) {
                        let a42 = v14;
                        let a43 = 2139095040u64;
                        nat_le(a42, a43)
                    } else {
                        false
                    } {
                        Some(v14)
                    } else {
                        None::<u64>
                    }
                }
            }))
        }
        Adt0::C6(v20, v21, v22, v23, v24, v25, v26, v27, v28, v29) => {
            f14(v20, v21, v22, v23, v24, true, v25, v26, v27, v28, v29)
        }
    }
}

pub fn f1(v0: Option<Adt2>, v1: Option<Adt2>, v2: Option<Adt2>, v3: Option<Adt2>, v4: bool, v5: Adt2) -> R<Adt10> {
    let v7: Adt2 = {
        let m44 = v0.clone();
        match m44 {
            None => {
                v5.clone()
            }
            Some(v6) => {
                v6
            }
        }
    };
    let v9: Adt2 = {
        let m45 = v1.clone();
        match m45 {
            None => {
                v5.clone()
            }
            Some(v8) => {
                v8
            }
        }
    };
    let v11: Adt2 = {
        let m46 = v2.clone();
        match m46 {
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
        let m47 = v0;
        match m47 {
            None => {
                false
            }
            Some(_) => {
                let m48 = v1;
                match m48 {
                    None => {
                        false
                    }
                    Some(_) => {
                        let m49 = v2;
                        match m49 {
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
        let a51 = match v7.clone() {
            Adt2::C0(h50, _, _, _, _, _, _, _, _, _, _) => {
                h50
            }
        };
        let a52 = 0u64;
        if if if if if if if if if if if if if if nat_eq(a51, a52) {
            let a54 = match v7.clone() {
                Adt2::C0(_, _, _, _, h53, _, _, _, _, _, _) => {
                    h53
                }
            };
            let a55 = 1u64;
            nat_eq(a54, a55)
        } else {
            false
        } {
            f17(v7.clone(), f18(v7.clone(), 1u64)?)?
        } else {
            false
        } {
            f19(v7.clone(), v5.clone())?
        } else {
            false
        } {
            let a57 = match v9.clone() {
                Adt2::C0(h56, _, _, _, _, _, _, _, _, _, _) => {
                    h56
                }
            };
            let a58 = 0u64;
            if if if nat_eq(a57, a58) {
                let a60 = match v9.clone() {
                    Adt2::C0(_, _, _, _, h59, _, _, _, _, _, _) => {
                        h59
                    }
                };
                let a61 = 1u64;
                nat_eq(a60, a61)
            } else {
                false
            } {
                f17(v9.clone(), f18(v9.clone(), 1u64)?)?
            } else {
                false
            } {
                f19(v9.clone(), v5.clone())?
            } else {
                false
            }
        } else {
            false
        } {
            let a63 = match v11.clone() {
                Adt2::C0(h62, _, _, _, _, _, _, _, _, _, _) => {
                    h62
                }
            };
            let a64 = 0u64;
            if if if nat_eq(a63, a64) {
                let a66 = match v11.clone() {
                    Adt2::C0(_, _, _, _, h65, _, _, _, _, _, _) => {
                        h65
                    }
                };
                let a67 = 1u64;
                nat_eq(a66, a67)
            } else {
                false
            } {
                f17(v11.clone(), f18(v11.clone(), 1u64)?)?
            } else {
                false
            } {
                f19(v11.clone(), v5.clone())?
            } else {
                false
            }
        } else {
            false
        } {
            let a69 = match v7.clone() {
                Adt2::C0(_, _, _, h68, _, _, _, _, _, _, _) => {
                    h68
                }
            };
            let a70 = 1u64;
            nat_eq(a69, a70)
        } else {
            false
        } {
            let a72 = match v11.clone() {
                Adt2::C0(_, _, _, h71, _, _, _, _, _, _, _) => {
                    h71
                }
            };
            let a73 = 1u64;
            nat_eq(a72, a73)
        } else {
            false
        } {
            let a76 = match v9.clone() {
                Adt2::C0(_, h74, _, _, _, _, _, _, _, _, _) => {
                    h74
                }
            };
            let a77 = match v7.clone() {
                Adt2::C0(_, h75, _, _, _, _, _, _, _, _, _) => {
                    h75
                }
            };
            nat_eq(a76, a77)
        } else {
            false
        } {
            let a80 = match v9.clone() {
                Adt2::C0(_, _, _, h78, _, _, _, _, _, _, _) => {
                    h78
                }
            };
            let a81 = match v7.clone() {
                Adt2::C0(_, _, h79, _, _, _, _, _, _, _, _) => {
                    h79
                }
            };
            nat_eq(a80, a81)
        } else {
            false
        } {
            let a84 = match v11.clone() {
                Adt2::C0(_, h82, _, _, _, _, _, _, _, _, _) => {
                    h82
                }
            };
            let a85 = match v9.clone() {
                Adt2::C0(_, _, h83, _, _, _, _, _, _, _, _) => {
                    h83
                }
            };
            nat_eq(a84, a85)
        } else {
            false
        } {
            let a88 = match v11.clone() {
                Adt2::C0(_, _, h86, _, _, _, _, _, _, _, _) => {
                    h86
                }
            };
            let a89 = match v7.clone() {
                Adt2::C0(_, _, h87, _, _, _, _, _, _, _, _) => {
                    h87
                }
            };
            nat_eq(a88, a89)
        } else {
            false
        } {
            let a91 = 0u64;
            let a92 = match v9.clone() {
                Adt2::C0(_, _, h90, _, _, _, _, _, _, _, _) => {
                    h90
                }
            };
            if nat_lt(a91, a92) {
                f20(v9.clone(), v5.clone())
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
            let m93 = v3.clone();
            match m93 {
                None => {
                    true
                }
                Some(v15) => {
                    let a95 = match v15.clone() {
                        Adt2::C0(h94, _, _, _, _, _, _, _, _, _, _) => {
                            h94
                        }
                    };
                    let a96 = 0u64;
                    if if if if if if nat_eq(a95, a96) {
                        let a98 = match v15.clone() {
                            Adt2::C0(_, _, _, _, h97, _, _, _, _, _, _) => {
                                h97
                            }
                        };
                        let a99 = 1u64;
                        nat_eq(a98, a99)
                    } else {
                        false
                    } {
                        f17(v15.clone(), f18(v15.clone(), 1u64)?)?
                    } else {
                        false
                    } {
                        f19(v15.clone(), v5.clone())?
                    } else {
                        false
                    } {
                        let a102 = match v15.clone() {
                            Adt2::C0(_, h100, _, _, _, _, _, _, _, _, _) => {
                                h100
                            }
                        };
                        let a103 = match v9.clone() {
                            Adt2::C0(_, _, h101, _, _, _, _, _, _, _, _) => {
                                h101
                            }
                        };
                        nat_eq(a102, a103)
                    } else {
                        false
                    } {
                        let a106 = match v15.clone() {
                            Adt2::C0(_, _, h104, _, _, _, _, _, _, _, _) => {
                                h104
                            }
                        };
                        let a107 = match v9.clone() {
                            Adt2::C0(_, _, h105, _, _, _, _, _, _, _, _) => {
                                h105
                            }
                        };
                        nat_eq(a106, a107)
                    } else {
                        false
                    } {
                        let a110 = match v15 {
                            Adt2::C0(_, _, _, h108, _, _, _, _, _, _, _) => {
                                h108
                            }
                        };
                        let a111 = match v7.clone() {
                            Adt2::C0(_, _, h109, _, _, _, _, _, _, _, _) => {
                                h109
                            }
                        };
                        nat_eq(a110, a111)
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
            let m112 = v3;
            match m112 {
                None => {
                    None::<Adt3>
                }
                Some(v16) => {
                    Some(f22(v16, 1u64))
                }
            }
        };
        Ok(Adt10::C4({
            let a114 = f18(v5.clone(), 1u64)?;
            let a115 = f23(match v9.clone() {
                Adt2::C0(_, _, h113, _, _, _, _, _, _, _, _) => {
                    h113
                }
            }, v17.clone())?;
            nat_mul(a114, a115)?
        }, 4u64, f22(v7, 1u64), f22(v5, 1u64), Adt4::C37(f22(v9, 1u64), f22(v11, 1u64), v17)))
    } else {
        Ok(Adt10::C1)
    }
}

pub fn f2(v0: Adt1, v1: Adt2) -> R<Adt10> {
    let v2: u64 = f18(v1.clone(), 1u64)?;
    let a117 = match v1.clone() {
        Adt2::C0(h116, _, _, _, _, _, _, _, _, _, _) => {
            h116
        }
    };
    let a118 = 0u64;
    if if if if if if if if nat_eq(a117, a118) {
        let a120 = match v1.clone() {
            Adt2::C0(_, _, _, _, _, h119, _, _, _, _, _) => {
                h119
            }
        };
        let a121 = 4u64;
        nat_eq(a120, a121)
    } else {
        false
    } {
        f24(match v0.clone() {
            Adt1::C0(h122, _, _, _, _, _, _, _, _, _, _, _) => {
                h122
            }
        })
    } else {
        false
    } {
        f24(match v0.clone() {
            Adt1::C0(_, h123, _, _, _, _, _, _, _, _, _, _) => {
                h123
            }
        })
    } else {
        false
    } {
        let a125 = 0u64;
        let a126 = match v0.clone() {
            Adt1::C0(_, _, h124, _, _, _, _, _, _, _, _, _) => {
                h124
            }
        };
        nat_lt(a125, a126)
    } else {
        false
    } {
        let a128 = match v0.clone() {
            Adt1::C0(_, _, h127, _, _, _, _, _, _, _, _, _) => {
                h127
            }
        };
        let a129 = 2139095040u64;
        nat_lt(a128, a129)
    } else {
        false
    } {
        let a130 = v2;
        let a131 = 9223372036854775808u64;
        nat_lt(a130, a131)
    } else {
        false
    } {
        let a133 = f25(v2)?;
        let a134 = match v1.clone() {
            Adt2::C0(_, _, _, _, _, _, _, _, _, h132, _) => {
                h132
            }
        };
        nat_le(a133, a134)
    } else {
        false
    } {
        Ok(Adt10::C4(f26(v2)?, 4u64, f22(v1.clone(), 1u64), f22(v1, 1u64), Adt4::C23(match v0.clone() {
            Adt1::C0(h135, _, _, _, _, _, _, _, _, _, _, _) => {
                h135
            }
        }, match v0.clone() {
            Adt1::C0(_, h136, _, _, _, _, _, _, _, _, _, _) => {
                h136
            }
        }, match v0 {
            Adt1::C0(_, _, h137, _, _, _, _, _, _, _, _, _) => {
                h137
            }
        }, v2)))
    } else {
        Ok(Adt10::C1)
    }
}

pub fn f3(v0: bool, v1: Adt1, v2: Adt2, v3: Option<Adt2>, v4: Adt2) -> R<Adt10> {
    let m138 = v3;
    match m138 {
        None => {
            Ok(Adt10::C1)
        }
        Some(v5) => {
            let v6: Adt3 = f27(v0, v2.clone());
            let v7: Adt3 = f27(v0, v5.clone());
            let v8: Adt3 = f27(v0, v4.clone());
            let v9: u64 = match v7.clone() {
                Adt3::C0(h139, _, _, _, _, _, _, _) => {
                    h139
                }
            };
            let v10: u64 = match v7.clone() {
                Adt3::C0(_, h140, _, _, _, _, _, _) => {
                    h140
                }
            };
            let v11: u64 = match v7.clone() {
                Adt3::C0(_, _, h141, _, _, _, _, _) => {
                    h141
                }
            };
            let v12: u64 = match v7.clone() {
                Adt3::C0(_, _, _, h142, _, _, _, _) => {
                    h142
                }
            };
            let v13: u64 = match v6.clone() {
                Adt3::C0(h143, _, _, _, _, _, _, _) => {
                    h143
                }
            };
            let v14: u64 = match v6.clone() {
                Adt3::C0(_, h144, _, _, _, _, _, _) => {
                    h144
                }
            };
            let v15: u64 = match v6.clone() {
                Adt3::C0(_, _, h145, _, _, _, _, _) => {
                    h145
                }
            };
            let v16: u64 = match v1.clone() {
                Adt1::C0(h146, _, _, _, _, _, _, _, _, _, _, _) => {
                    h146
                }
            };
            let v17: u64 = {
                let a151 = v13;
                let a152 = {
                    let a149 = {
                        let a147 = v9;
                        let a148 = 1u64;
                        nat_sub(a147, a148)
                    };
                    let a150 = v16;
                    nat_mul(a149, a150)?
                };
                nat_add(a151, a152)?
            };
            let v18: u64 = {
                let a157 = v14;
                let a158 = {
                    let a155 = {
                        let a153 = v10;
                        let a154 = 1u64;
                        nat_sub(a153, a154)
                    };
                    let a156 = v16;
                    nat_mul(a155, a156)?
                };
                nat_add(a157, a158)?
            };
            let v19: u64 = {
                let a161 = {
                    let a159 = v9;
                    let a160 = v10;
                    nat_mul(a159, a160)?
                };
                let a162 = v12;
                nat_mul(a161, a162)?
            };
            let v20: u64 = {
                let a165 = {
                    let a163 = v13;
                    let a164 = v14;
                    nat_mul(a163, a164)?
                };
                let a166 = v15;
                nat_mul(a165, a166)?
            };
            let v21: u64 = {
                let a167 = v19;
                let a168 = 32u64;
                if nat_lt(a167, a168) {
                    v19
                } else {
                    32u64
                }
            };
            let a170 = match v2.clone() {
                Adt2::C0(h169, _, _, _, _, _, _, _, _, _, _) => {
                    h169
                }
            };
            let a171 = 0u64;
            if if if if if if if if if if if if if if if if if if if if if if if if nat_eq(a170, a171) {
                true
            } else {
                let a173 = match v2.clone() {
                    Adt2::C0(h172, _, _, _, _, _, _, _, _, _, _) => {
                        h172
                    }
                };
                let a174 = 1u64;
                nat_eq(a173, a174)
            } {
                let a176 = match v5.clone() {
                    Adt2::C0(h175, _, _, _, _, _, _, _, _, _, _) => {
                        h175
                    }
                };
                let a177 = 0u64;
                nat_eq(a176, a177)
            } else {
                false
            } {
                let a179 = match v4.clone() {
                    Adt2::C0(h178, _, _, _, _, _, _, _, _, _, _) => {
                        h178
                    }
                };
                let a180 = 0u64;
                nat_eq(a179, a180)
            } else {
                false
            } {
                let a181 = v16;
                let a182 = 2147483648u64;
                nat_lt(a181, a182)
            } else {
                false
            } {
                let a183 = 0u64;
                let a184 = v9;
                nat_lt(a183, a184)
            } else {
                false
            } {
                let a185 = 0u64;
                let a186 = v10;
                nat_lt(a185, a186)
            } else {
                false
            } {
                let a187 = 0u64;
                let a188 = v11;
                nat_lt(a187, a188)
            } else {
                false
            } {
                let a189 = 0u64;
                let a190 = v12;
                nat_lt(a189, a190)
            } else {
                false
            } {
                let a191 = 0u64;
                let a192 = v13;
                nat_lt(a191, a192)
            } else {
                false
            } {
                let a193 = 0u64;
                let a194 = v14;
                nat_lt(a193, a194)
            } else {
                false
            } {
                let a195 = 0u64;
                let a196 = v15;
                nat_lt(a195, a196)
            } else {
                false
            } {
                let a197 = 0u64;
                let a198 = v16;
                nat_lt(a197, a198)
            } else {
                false
            } {
                let a200 = match v6.clone() {
                    Adt3::C0(_, _, _, h199, _, _, _, _) => {
                        h199
                    }
                };
                let a201 = v11;
                nat_eq(a200, a201)
            } else {
                false
            } {
                let a203 = match v8.clone() {
                    Adt3::C0(h202, _, _, _, _, _, _, _) => {
                        h202
                    }
                };
                let a204 = v17;
                nat_eq(a203, a204)
            } else {
                false
            } {
                let a206 = match v8.clone() {
                    Adt3::C0(_, h205, _, _, _, _, _, _) => {
                        h205
                    }
                };
                let a207 = v18;
                nat_eq(a206, a207)
            } else {
                false
            } {
                let a209 = match v8.clone() {
                    Adt3::C0(_, _, h208, _, _, _, _, _) => {
                        h208
                    }
                };
                let a210 = v15;
                nat_eq(a209, a210)
            } else {
                false
            } {
                let a212 = match v8.clone() {
                    Adt3::C0(_, _, _, h211, _, _, _, _) => {
                        h211
                    }
                };
                let a213 = v12;
                nat_eq(a212, a213)
            } else {
                false
            } {
                f28(v4.clone())?
            } else {
                false
            } {
                let a218 = match v2.clone() {
                    Adt2::C0(_, _, _, _, _, h214, _, _, _, _, _) => {
                        h214
                    }
                };
                let a219 = {
                    let a216 = match v2.clone() {
                        Adt2::C0(h215, _, _, _, _, _, _, _, _, _, _) => {
                            h215
                        }
                    };
                    let a217 = 1u64;
                    if nat_eq(a216, a217) {
                        2u64
                    } else {
                        4u64
                    }
                };
                nat_eq(a218, a219)
            } else {
                false
            } {
                let a221 = match v5.clone() {
                    Adt2::C0(_, _, _, _, _, h220, _, _, _, _, _) => {
                        h220
                    }
                };
                let a222 = 4u64;
                nat_eq(a221, a222)
            } else {
                false
            } {
                f19(v2.clone(), v4.clone())?
            } else {
                false
            } {
                f19(v5.clone(), v4.clone())?
            } else {
                false
            } {
                if v0 {
                    let a227 = match v2.clone() {
                        Adt2::C0(_, _, _, _, _, _, h223, _, _, _, _) => {
                            h223
                        }
                    };
                    let a228 = {
                        let a225 = v13;
                        let a226 = match v2 {
                            Adt2::C0(_, _, _, _, _, h224, _, _, _, _, _) => {
                                h224
                            }
                        };
                        nat_mul(a225, a226)?
                    };
                    nat_eq(a227, a228)
                } else {
                    let a230 = match v1.clone() {
                        Adt1::C0(_, h229, _, _, _, _, _, _, _, _, _, _) => {
                            h229
                        }
                    };
                    let a231 = 0u64;
                    if if if if if if nat_eq(a230, a231) {
                        let a233 = match v1 {
                            Adt1::C0(_, _, h232, _, _, _, _, _, _, _, _, _) => {
                                h232
                            }
                        };
                        let a234 = 1u64;
                        nat_eq(a233, a234)
                    } else {
                        false
                    } {
                        let a236 = match v2 {
                            Adt2::C0(_, _, _, _, h235, _, _, _, _, _, _) => {
                                h235
                            }
                        };
                        let a237 = 1u64;
                        nat_eq(a236, a237)
                    } else {
                        false
                    } {
                        let a239 = match v5.clone() {
                            Adt2::C0(_, _, _, h238, _, _, _, _, _, _, _) => {
                                h238
                            }
                        };
                        let a240 = 1u64;
                        nat_eq(a239, a240)
                    } else {
                        false
                    } {
                        let a242 = match v5 {
                            Adt2::C0(_, _, _, _, h241, _, _, _, _, _, _) => {
                                h241
                            }
                        };
                        let a243 = 1u64;
                        nat_eq(a242, a243)
                    } else {
                        false
                    } {
                        let a245 = match v4.clone() {
                            Adt2::C0(_, _, _, h244, _, _, _, _, _, _, _) => {
                                h244
                            }
                        };
                        let a246 = 1u64;
                        nat_eq(a245, a246)
                    } else {
                        false
                    } {
                        let a248 = match v4 {
                            Adt2::C0(_, _, _, _, h247, _, _, _, _, _, _) => {
                                h247
                            }
                        };
                        let a249 = 1u64;
                        nat_eq(a248, a249)
                    } else {
                        false
                    }
                }
            } else {
                false
            } {
                Ok(Adt10::C6(Adt8::C0(v19, v11, v20, {
                    let a250 = v11;
                    let a251 = v20;
                    nat_mul(a250, a251)?
                }, v21, {
                    let a252 = v21;
                    let a253 = v11;
                    nat_mul(a252, a253)?
                }, {
                    let a254 = v21;
                    let a255 = v20;
                    nat_mul(a254, a255)?
                }, {
                    let a260 = {
                        let a256 = v17;
                        let a257 = v18;
                        nat_mul(a256, a257)?
                    };
                    let a261 = {
                        let a258 = v15;
                        let a259 = v12;
                        nat_mul(a258, a259)?
                    };
                    nat_mul(a260, a261)?
                }, v6, v7, v8, Adt6::C0(v13, v16, 0u64, 1u64), Adt6::C0(v14, v16, 0u64, 1u64), Adt6::C0(1u64, 1u64, 0u64, 1u64), 0u64)))
            } else {
                Ok(Adt10::C1)
            }
        }
    }
}

pub fn f4(v0: bool, v1: Adt1, v2: Adt2, v3: Option<Adt2>, v4: Adt2) -> R<Adt10> {
    let m262 = v3;
    match m262 {
        None => {
            Ok(Adt10::C1)
        }
        Some(v5) => {
            let v6: Adt6 = Adt6::C0(match v2.clone() {
                Adt2::C0(_, h263, _, _, _, _, _, _, _, _, _) => {
                    h263
                }
            }, match v1.clone() {
                Adt1::C0(h264, _, _, _, _, _, _, _, _, _, _, _) => {
                    h264
                }
            }, if v0 {
                match v1.clone() {
                    Adt1::C0(_, _, _, h265, _, _, _, _, _, _, _, _) => {
                        h265
                    }
                }
            } else {
                match v1.clone() {
                    Adt1::C0(_, _, h266, _, _, _, _, _, _, _, _, _) => {
                        h266
                    }
                }
            }, if v0 {
                match v1.clone() {
                    Adt1::C0(_, _, _, _, _, _, h267, _, _, _, _, _) => {
                        h267
                    }
                }
            } else {
                match v1.clone() {
                    Adt1::C0(_, _, _, _, h268, _, _, _, _, _, _, _) => {
                        h268
                    }
                }
            });
            let v7: Adt6 = Adt6::C0(match v2.clone() {
                Adt2::C0(_, _, h269, _, _, _, _, _, _, _, _) => {
                    h269
                }
            }, match v1.clone() {
                Adt1::C0(_, h270, _, _, _, _, _, _, _, _, _, _) => {
                    h270
                }
            }, if v0 {
                match v1.clone() {
                    Adt1::C0(_, _, _, _, h271, _, _, _, _, _, _, _) => {
                        h271
                    }
                }
            } else {
                match v1.clone() {
                    Adt1::C0(_, _, _, h272, _, _, _, _, _, _, _, _) => {
                        h272
                    }
                }
            }, if v0 {
                match v1.clone() {
                    Adt1::C0(_, _, _, _, _, _, _, h273, _, _, _, _) => {
                        h273
                    }
                }
            } else {
                match v1.clone() {
                    Adt1::C0(_, _, _, _, _, h274, _, _, _, _, _, _) => {
                        h274
                    }
                }
            });
            let v8: Adt6 = if v0 {
                Adt6::C0(match v2.clone() {
                    Adt2::C0(_, _, _, h275, _, _, _, _, _, _, _) => {
                        h275
                    }
                }, match v1.clone() {
                    Adt1::C0(_, _, h276, _, _, _, _, _, _, _, _, _) => {
                        h276
                    }
                }, match v1.clone() {
                    Adt1::C0(_, _, _, _, _, h277, _, _, _, _, _, _) => {
                        h277
                    }
                }, match v1.clone() {
                    Adt1::C0(_, _, _, _, _, _, _, _, h278, _, _, _) => {
                        h278
                    }
                })
            } else {
                Adt6::C0(1u64, 1u64, 0u64, 1u64)
            };
            let v9: u64 = f29(v6.clone(), match v5.clone() {
                Adt2::C0(_, h279, _, _, _, _, _, _, _, _, _) => {
                    h279
                }
            })?;
            let v10: u64 = f29(v7.clone(), match v5.clone() {
                Adt2::C0(_, _, h280, _, _, _, _, _, _, _, _) => {
                    h280
                }
            })?;
            let v11: u64 = if v0 {
                f29(v8.clone(), match v5.clone() {
                    Adt2::C0(_, _, _, h281, _, _, _, _, _, _, _) => {
                        h281
                    }
                })?
            } else {
                1u64
            };
            let v12: u64 = if v0 {
                match v1.clone() {
                    Adt1::C0(_, _, _, _, _, _, _, _, _, h282, _, _) => {
                        h282
                    }
                }
            } else {
                match v5.clone() {
                    Adt2::C0(_, _, _, h283, _, _, _, _, _, _, _) => {
                        h283
                    }
                }
            };
            let v13: u64 = if v0 {
                match v1.clone() {
                    Adt1::C0(_, _, _, _, _, _, _, _, _, _, h284, _) => {
                        h284
                    }
                }
            } else {
                match v5.clone() {
                    Adt2::C0(_, _, _, _, h285, _, _, _, _, _, _) => {
                        h285
                    }
                }
            };
            let v14: u64 = if v0 {
                match v1 {
                    Adt1::C0(_, _, _, _, _, _, _, _, _, _, _, h286) => {
                        h286
                    }
                }
            } else {
                match v2.clone() {
                    Adt2::C0(_, _, _, _, h287, _, _, _, _, _, _) => {
                        h287
                    }
                }
            };
            let v15: u64 = {
                let a292 = {
                    let a288 = v9;
                    let a289 = v10;
                    nat_mul(a288, a289)?
                };
                let a293 = {
                    let a290 = v11;
                    let a291 = v13;
                    nat_mul(a290, a291)?
                };
                nat_mul(a292, a293)?
            };
            let v16: u64 = {
                let a301 = {
                    let a296 = match v6.clone() {
                        Adt6::C0(h294, _, _, _) => {
                            h294
                        }
                    };
                    let a297 = match v7.clone() {
                        Adt6::C0(h295, _, _, _) => {
                            h295
                        }
                    };
                    nat_mul(a296, a297)?
                };
                let a302 = {
                    let a299 = match v8.clone() {
                        Adt6::C0(h298, _, _, _) => {
                            h298
                        }
                    };
                    let a300 = v12;
                    nat_mul(a299, a300)?
                };
                nat_mul(a301, a302)?
            };
            let v17: u64 = {
                let a303 = v15;
                let a304 = 32u64;
                if nat_lt(a303, a304) {
                    v15
                } else {
                    32u64
                }
            };
            let a306 = match v2.clone() {
                Adt2::C0(h305, _, _, _, _, _, _, _, _, _, _) => {
                    h305
                }
            };
            let a307 = 0u64;
            if if if if if if if if if if if nat_eq(a306, a307) {
                true
            } else {
                let a309 = match v2.clone() {
                    Adt2::C0(h308, _, _, _, _, _, _, _, _, _, _) => {
                        h308
                    }
                };
                let a310 = 1u64;
                nat_eq(a309, a310)
            } {
                let a312 = match v5.clone() {
                    Adt2::C0(h311, _, _, _, _, _, _, _, _, _, _) => {
                        h311
                    }
                };
                let a313 = 0u64;
                nat_eq(a312, a313)
            } else {
                false
            } {
                let a315 = match v4.clone() {
                    Adt2::C0(h314, _, _, _, _, _, _, _, _, _, _) => {
                        h314
                    }
                };
                let a316 = 0u64;
                nat_eq(a315, a316)
            } else {
                false
            } {
                let a318 = match v4.clone() {
                    Adt2::C0(_, h317, _, _, _, _, _, _, _, _, _) => {
                        h317
                    }
                };
                let a319 = v9;
                nat_eq(a318, a319)
            } else {
                false
            } {
                let a321 = match v4.clone() {
                    Adt2::C0(_, _, h320, _, _, _, _, _, _, _, _) => {
                        h320
                    }
                };
                let a322 = v10;
                nat_eq(a321, a322)
            } else {
                false
            } {
                if v0 {
                    let a323 = 0u64;
                    let a324 = v12;
                    if if if if if if if nat_lt(a323, a324) {
                        let a325 = v12;
                        let a326 = 2147483648u64;
                        nat_lt(a325, a326)
                    } else {
                        false
                    } {
                        let a327 = v14;
                        let a328 = 2147483648u64;
                        nat_lt(a327, a328)
                    } else {
                        false
                    } {
                        let a329 = v13;
                        let a330 = 2147483648u64;
                        nat_lt(a329, a330)
                    } else {
                        false
                    } {
                        let a334 = match v2.clone() {
                            Adt2::C0(_, _, _, _, h331, _, _, _, _, _, _) => {
                                h331
                            }
                        };
                        let a335 = {
                            let a332 = v12;
                            let a333 = v14;
                            nat_mul(a332, a333)?
                        };
                        nat_eq(a334, a335)
                    } else {
                        false
                    } {
                        let a339 = match v5.clone() {
                            Adt2::C0(_, _, _, _, h336, _, _, _, _, _, _) => {
                                h336
                            }
                        };
                        let a340 = {
                            let a337 = v12;
                            let a338 = v13;
                            nat_mul(a337, a338)?
                        };
                        nat_eq(a339, a340)
                    } else {
                        false
                    } {
                        let a342 = match v4.clone() {
                            Adt2::C0(_, _, _, h341, _, _, _, _, _, _, _) => {
                                h341
                            }
                        };
                        let a343 = v11;
                        nat_eq(a342, a343)
                    } else {
                        false
                    } {
                        let a347 = match v4.clone() {
                            Adt2::C0(_, _, _, _, h344, _, _, _, _, _, _) => {
                                h344
                            }
                        };
                        let a348 = {
                            let a345 = v14;
                            let a346 = v13;
                            nat_mul(a345, a346)?
                        };
                        nat_eq(a347, a348)
                    } else {
                        false
                    }
                } else {
                    let a350 = match v2.clone() {
                        Adt2::C0(_, _, _, h349, _, _, _, _, _, _, _) => {
                            h349
                        }
                    };
                    let a351 = v12;
                    if if if nat_eq(a350, a351) {
                        let a353 = match v4.clone() {
                            Adt2::C0(_, _, _, h352, _, _, _, _, _, _, _) => {
                                h352
                            }
                        };
                        let a354 = v14;
                        nat_eq(a353, a354)
                    } else {
                        false
                    } {
                        let a356 = match v4.clone() {
                            Adt2::C0(_, _, _, _, h355, _, _, _, _, _, _) => {
                                h355
                            }
                        };
                        let a357 = v13;
                        nat_eq(a356, a357)
                    } else {
                        false
                    } {
                        let a358 = 0u64;
                        let a359 = v16;
                        nat_lt(a358, a359)
                    } else {
                        false
                    }
                }
            } else {
                false
            } {
                let a360 = 0u64;
                let a361 = v14;
                nat_lt(a360, a361)
            } else {
                false
            } {
                let a362 = 0u64;
                let a363 = v15;
                nat_lt(a362, a363)
            } else {
                false
            } {
                f19(v2.clone(), v4.clone())?
            } else {
                false
            } {
                f19(v5.clone(), v4.clone())?
            } else {
                false
            } {
                Ok(Adt10::C6(Adt8::C0(v15, v16, v14, {
                    let a364 = v16;
                    let a365 = v14;
                    nat_mul(a364, a365)?
                }, v17, {
                    let a366 = v17;
                    let a367 = v16;
                    nat_mul(a366, a367)?
                }, {
                    let a368 = v17;
                    let a369 = v14;
                    nat_mul(a368, a369)?
                }, 0u64, f22(v2, 1u64), f22(v5, 1u64), f22(v4, 1u64), v6, v7, v8, if v0 {
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

pub fn f5(v0: Adt1, v1: Adt2, v2: Option<Adt2>, v3: Option<Adt2>, v4: Adt2) -> R<Adt10> {
    let m370 = v2;
    match m370 {
        None => {
            Ok(Adt10::C1)
        }
        Some(v5) => {
            let m371 = v3;
            match m371 {
                None => {
                    Ok(Adt10::C1)
                }
                Some(v6) => {
                    let a373 = match v1.clone() {
                        Adt2::C0(h372, _, _, _, _, _, _, _, _, _, _) => {
                            h372
                        }
                    };
                    let a374 = 0u64;
                    if if if if if if if if if if if if if if if nat_eq(a373, a374) {
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
                        let a379 = match v6.clone() {
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
                        f20(v5.clone(), v6.clone())
                    } else {
                        false
                    } {
                        f20(v1.clone(), v4.clone())
                    } else {
                        false
                    } {
                        f28(v1.clone())?
                    } else {
                        false
                    } {
                        f28(v5.clone())?
                    } else {
                        false
                    } {
                        f28(v6)?
                    } else {
                        false
                    } {
                        f28(v4.clone())?
                    } else {
                        false
                    } {
                        let a389 = match v1.clone() {
                            Adt2::C0(_, h384, _, _, _, _, _, _, _, _, _) => {
                                h384
                            }
                        };
                        let a390 = {
                            let a387 = match v5.clone() {
                                Adt2::C0(_, h385, _, _, _, _, _, _, _, _, _) => {
                                    h385
                                }
                            };
                            let a388 = match v5.clone() {
                                Adt2::C0(_, h386, _, _, _, _, _, _, _, _, _) => {
                                    h386
                                }
                            };
                            nat_mul(a387, a388)?
                        };
                        nat_eq(a389, a390)
                    } else {
                        false
                    } {
                        let a396 = match v1.clone() {
                            Adt2::C0(_, _, h391, _, _, _, _, _, _, _, _) => {
                                h391
                            }
                        };
                        let a397 = {
                            let a394 = match v5.clone() {
                                Adt2::C0(_, _, h392, _, _, _, _, _, _, _, _) => {
                                    h392
                                }
                            };
                            let a395 = match v5.clone() {
                                Adt2::C0(_, _, _, h393, _, _, _, _, _, _, _) => {
                                    h393
                                }
                            };
                            nat_mul(a394, a395)?
                        };
                        nat_eq(a396, a397)
                    } else {
                        false
                    } {
                        let a400 = match v1.clone() {
                            Adt2::C0(_, _, _, h398, _, _, _, _, _, _, _) => {
                                h398
                            }
                        };
                        let a401 = match v5.clone() {
                            Adt2::C0(_, _, _, _, h399, _, _, _, _, _, _) => {
                                h399
                            }
                        };
                        nat_eq(a400, a401)
                    } else {
                        false
                    } {
                        let a403 = match v0.clone() {
                            Adt1::C0(h402, _, _, _, _, _, _, _, _, _, _, _) => {
                                h402
                            }
                        };
                        let a404 = 4294967296u64;
                        nat_lt(a403, a404)
                    } else {
                        false
                    } {
                        let a406 = match v0.clone() {
                            Adt1::C0(h405, _, _, _, _, _, _, _, _, _, _, _) => {
                                h405
                            }
                        };
                        let a407 = 0u64;
                        if nat_eq(a406, a407) {
                            if f19(v1.clone(), v4.clone())? {
                                true
                            } else {
                                let a410 = match v1.clone() {
                                    Adt2::C0(_, _, _, _, _, _, _, _, _, _, h408) => {
                                        h408
                                    }
                                };
                                let a411 = match v4.clone() {
                                    Adt2::C0(_, _, _, _, _, _, _, _, _, _, h409) => {
                                        h409
                                    }
                                };
                                nat_eq(a410, a411)
                            }
                        } else {
                            let a414 = match v1.clone() {
                                Adt2::C0(_, _, _, _, _, _, _, _, _, _, h412) => {
                                    h412
                                }
                            };
                            let a415 = match v4.clone() {
                                Adt2::C0(_, _, _, _, _, _, _, _, _, _, h413) => {
                                    h413
                                }
                            };
                            nat_eq(a414, a415)
                        }
                    } else {
                        false
                    } {
                        let v7: bool = {
                            let a417 = match v4.clone() {
                                Adt2::C0(_, h416, _, _, _, _, _, _, _, _, _) => {
                                    h416
                                }
                            };
                            let a418 = 0u64;
                            if if if nat_eq(a417, a418) {
                                true
                            } else {
                                let a420 = match v4.clone() {
                                    Adt2::C0(_, _, h419, _, _, _, _, _, _, _, _) => {
                                        h419
                                    }
                                };
                                let a421 = 0u64;
                                nat_eq(a420, a421)
                            } {
                                true
                            } else {
                                let a423 = match v4.clone() {
                                    Adt2::C0(_, _, _, h422, _, _, _, _, _, _, _) => {
                                        h422
                                    }
                                };
                                let a424 = 0u64;
                                nat_eq(a423, a424)
                            } {
                                true
                            } else {
                                let a426 = match v4.clone() {
                                    Adt2::C0(_, _, _, _, h425, _, _, _, _, _, _) => {
                                        h425
                                    }
                                };
                                let a427 = 0u64;
                                nat_eq(a426, a427)
                            }
                        };
                        let v8: u64 = match v5.clone() {
                            Adt2::C0(_, h428, _, _, _, _, _, _, _, _, _) => {
                                h428
                            }
                        };
                        let v9: u64 = if v7 {
                            0u64
                        } else {
                            let a430 = match v0 {
                                Adt1::C0(h429, _, _, _, _, _, _, _, _, _, _, _) => {
                                    h429
                                }
                            };
                            let a431 = 0u64;
                            if nat_eq(a430, a431) {
                                f18(v1.clone(), 1u64)?
                            } else {
                                0u64
                            }
                        };
                        let v10: u64 = if v7 {
                            0u64
                        } else {
                            f30(v8, match v5.clone() {
                                Adt2::C0(_, _, h432, _, _, _, _, _, _, _, _) => {
                                    h432
                                }
                            }, match v5.clone() {
                                Adt2::C0(_, _, _, h433, _, _, _, _, _, _, _) => {
                                    h433
                                }
                            }, match v5 {
                                Adt2::C0(_, _, _, _, h434, _, _, _, _, _, _) => {
                                    h434
                                }
                            })?
                        };
                        Ok(Adt10::C4(f31(v9, v8, v10)?, 4u64, f22(v1, 1u64), f22(v4, 1u64), Adt4::C25(v9, v8, v10)))
                    } else {
                        Ok(Adt10::C1)
                    }
                }
            }
        }
    }
}

pub fn f6(v0: u64, v1: Adt1, v2: Adt2, v3: Option<Adt2>, v4: Adt2) -> R<Adt10> {
    let v5: Adt14 = f32(v2.clone());
    let v6: Adt14 = f32(v4.clone());
    let v8: Adt15 = {
        let m435 = v3;
        match m435 {
            None => {
                f33(v0, match v1.clone() {
                    Adt1::C0(h436, _, _, _, _, _, _, _, _, _, _, _) => {
                        h436
                    }
                }, match v1 {
                    Adt1::C0(_, h437, _, _, _, _, _, _, _, _, _, _) => {
                        h437
                    }
                }, false, v5.clone(), v5, v6)?
            }
            Some(v7) => {
                f33(v0, match v1.clone() {
                    Adt1::C0(h438, _, _, _, _, _, _, _, _, _, _, _) => {
                        h438
                    }
                }, match v1 {
                    Adt1::C0(_, h439, _, _, _, _, _, _, _, _, _, _) => {
                        h439
                    }
                }, true, v5, f32(v7), v6)?
            }
        }
    };
    let m440 = v8;
    match m440 {
        Adt15::C0 => {
            Ok(Adt10::C1)
        }
        Adt15::C1(v9, v10, v11, v12, v13, v14) => {
            Ok(Adt10::C4(v9, v10, f22(v2, 1u64), f22(v4, 1u64), Adt4::C26(v11, v14, v12, v13)))
        }
    }
}

pub fn f7(v0: Adt1, v1: Adt2, v2: Adt2) -> R<Adt10> {
    let v3: u64 = match v1.clone() {
        Adt2::C0(h441, _, _, _, _, _, _, _, _, _, _) => {
            h441
        }
    };
    let v4: u64 = f34(v3);
    let a443 = match v1.clone() {
        Adt2::C0(h442, _, _, _, _, _, _, _, _, _, _) => {
            h442
        }
    };
    let a444 = 0u64;
    if if if if if if if if nat_eq(a443, a444) {
        true
    } else {
        let a446 = match v1.clone() {
            Adt2::C0(h445, _, _, _, _, _, _, _, _, _, _) => {
                h445
            }
        };
        let a447 = 30u64;
        nat_eq(a446, a447)
    } {
        let a450 = match v1.clone() {
            Adt2::C0(h448, _, _, _, _, _, _, _, _, _, _) => {
                h448
            }
        };
        let a451 = match v2.clone() {
            Adt2::C0(h449, _, _, _, _, _, _, _, _, _, _) => {
                h449
            }
        };
        nat_eq(a450, a451)
    } else {
        false
    } {
        f20(v1.clone(), v2.clone())
    } else {
        false
    } {
        f35(match v0.clone() {
            Adt1::C0(h452, _, _, _, _, _, _, _, _, _, _, _) => {
                h452
            }
        })
    } else {
        false
    } {
        f36(v1.clone(), v4)
    } else {
        false
    } {
        f36(v2.clone(), v4)
    } else {
        false
    } {
        if f19(v1.clone(), v2.clone())? {
            true
        } else {
            f37(v1.clone(), v2.clone())
        }
    } else {
        false
    } {
        let v5: u64 = if f38(match v1.clone() {
            Adt2::C0(_, h453, _, _, _, _, _, _, _, _, _) => {
                h453
            }
        }, match v1.clone() {
            Adt2::C0(_, _, h454, _, _, _, _, _, _, _, _) => {
                h454
            }
        }, match v1.clone() {
            Adt2::C0(_, _, _, h455, _, _, _, _, _, _, _) => {
                h455
            }
        }, match v1.clone() {
            Adt2::C0(_, _, _, _, h456, _, _, _, _, _, _) => {
                h456
            }
        }) {
            0u64
        } else {
            let a462 = match v1.clone() {
                Adt2::C0(_, _, h457, _, _, _, _, _, _, _, _) => {
                    h457
                }
            };
            let a463 = {
                let a460 = match v1.clone() {
                    Adt2::C0(_, _, _, h458, _, _, _, _, _, _, _) => {
                        h458
                    }
                };
                let a461 = match v1.clone() {
                    Adt2::C0(_, _, _, _, h459, _, _, _, _, _, _) => {
                        h459
                    }
                };
                nat_mul(a460, a461)?
            };
            nat_mul(a462, a463)?
        };
        let v6: u64 = {
            let a464 = v5;
            let a465 = 0u64;
            if if nat_eq(a464, a465) {
                true
            } else {
                f39(v2.clone(), v4)?
            } {
                0u64
            } else {
                let a467 = match v2.clone() {
                    Adt2::C0(_, _, _, _, _, _, _, _, _, h466, _) => {
                        h466
                    }
                };
                let a468 = v4;
                let a469 = 0u64;
                nat_quot(a467, a468, a469)
            }
        };
        Ok(Adt10::C4(f40(v3, match v1.clone() {
            Adt2::C0(_, h470, _, _, _, _, _, _, _, _, _) => {
                h470
            }
        }, v5, v6)?, v4, f22(v1.clone(), 1u64), f22(v2, 1u64), Adt4::C26(v3, match v0 {
            Adt1::C0(h471, _, _, _, _, _, _, _, _, _, _, _) => {
                h471
            }
        }, v5, f41(match v1 {
            Adt2::C0(_, h472, _, _, _, _, _, _, _, _, _) => {
                h472
            }
        }, v5, v6)?)))
    } else {
        Ok(Adt10::C1)
    }
}

pub fn f8(v0: Adt1, v1: Adt2, v2: Adt2) -> R<Adt10> {
    let a474 = match v1.clone() {
        Adt2::C0(h473, _, _, _, _, _, _, _, _, _, _) => {
            h473
        }
    };
    let a475 = 0u64;
    if if if if if nat_eq(a474, a475) {
        let a477 = match v2.clone() {
            Adt2::C0(h476, _, _, _, _, _, _, _, _, _, _) => {
                h476
            }
        };
        let a478 = 0u64;
        nat_eq(a477, a478)
    } else {
        false
    } {
        f20(v1.clone(), v2.clone())
    } else {
        false
    } {
        f42(match v0.clone() {
            Adt1::C0(h479, _, _, _, _, _, _, _, _, _, _, _) => {
                h479
            }
        })
    } else {
        false
    } {
        if f19(v1.clone(), v2.clone())? {
            true
        } else {
            f37(v1.clone(), v2.clone())
        }
    } else {
        false
    } {
        let v3: Adt3 = f22(v1, 1u64);
        let v4: u64 = {
            let a485 = match v3.clone() {
                Adt3::C0(_, h480, _, _, _, _, _, _) => {
                    h480
                }
            };
            let a486 = {
                let a483 = match v3.clone() {
                    Adt3::C0(_, _, h481, _, _, _, _, _) => {
                        h481
                    }
                };
                let a484 = match v3.clone() {
                    Adt3::C0(_, _, _, h482, _, _, _, _) => {
                        h482
                    }
                };
                nat_mul(a483, a484)?
            };
            nat_mul(a485, a486)?
        };
        Ok(Adt10::C4(f43(match v3.clone() {
            Adt3::C0(h487, _, _, _, _, _, _, _) => {
                h487
            }
        }, v4)?, 4u64, v3, f22(v2, 1u64), Adt4::C24(match v0 {
            Adt1::C0(h488, _, _, _, _, _, _, _, _, _, _, _) => {
                h488
            }
        })))
    } else {
        Ok(Adt10::C1)
    }
}

pub fn f9(v0: Adt2, v1: Option<Adt2>, v2: Option<Adt2>, v3: Adt2) -> R<Adt10> {
    let m489 = v1;
    match m489 {
        None => {
            Ok(Adt10::C1)
        }
        Some(v4) => {
            let m490 = v2;
            match m490 {
                None => {
                    Ok(Adt10::C1)
                }
                Some(v5) => {
                    let a492 = match v0.clone() {
                        Adt2::C0(h491, _, _, _, _, _, _, _, _, _, _) => {
                            h491
                        }
                    };
                    let a493 = 0u64;
                    if if if if if if if if if if if if if if if if if nat_eq(a492, a493) {
                        let a495 = match v4.clone() {
                            Adt2::C0(h494, _, _, _, _, _, _, _, _, _, _) => {
                                h494
                            }
                        };
                        let a496 = 0u64;
                        nat_eq(a495, a496)
                    } else {
                        false
                    } {
                        let a498 = match v5.clone() {
                            Adt2::C0(h497, _, _, _, _, _, _, _, _, _, _) => {
                                h497
                            }
                        };
                        let a499 = 26u64;
                        nat_eq(a498, a499)
                    } else {
                        false
                    } {
                        let a501 = match v3.clone() {
                            Adt2::C0(h500, _, _, _, _, _, _, _, _, _, _) => {
                                h500
                            }
                        };
                        let a502 = 0u64;
                        nat_eq(a501, a502)
                    } else {
                        false
                    } {
                        f20(v0.clone(), v3.clone())
                    } else {
                        false
                    } {
                        let a505 = match v0.clone() {
                            Adt2::C0(_, h503, _, _, _, _, _, _, _, _, _) => {
                                h503
                            }
                        };
                        let a506 = match v4.clone() {
                            Adt2::C0(_, h504, _, _, _, _, _, _, _, _, _) => {
                                h504
                            }
                        };
                        nat_eq(a505, a506)
                    } else {
                        false
                    } {
                        let a509 = match v0.clone() {
                            Adt2::C0(_, _, h507, _, _, _, _, _, _, _, _) => {
                                h507
                            }
                        };
                        let a510 = match v5.clone() {
                            Adt2::C0(_, h508, _, _, _, _, _, _, _, _, _) => {
                                h508
                            }
                        };
                        nat_eq(a509, a510)
                    } else {
                        false
                    } {
                        let a513 = match v0.clone() {
                            Adt2::C0(_, _, _, h511, _, _, _, _, _, _, _) => {
                                h511
                            }
                        };
                        let a514 = match v5.clone() {
                            Adt2::C0(_, _, h512, _, _, _, _, _, _, _, _) => {
                                h512
                            }
                        };
                        nat_eq(a513, a514)
                    } else {
                        false
                    } {
                        let a516 = 0u64;
                        let a517 = match v5.clone() {
                            Adt2::C0(_, _, _, h515, _, _, _, _, _, _, _) => {
                                h515
                            }
                        };
                        nat_lt(a516, a517)
                    } else {
                        false
                    } {
                        let a519 = 0u64;
                        let a520 = match v5.clone() {
                            Adt2::C0(_, _, _, _, h518, _, _, _, _, _, _) => {
                                h518
                            }
                        };
                        nat_lt(a519, a520)
                    } else {
                        false
                    } {
                        let a522 = 4u64;
                        let a523 = match v0.clone() {
                            Adt2::C0(_, _, _, _, _, h521, _, _, _, _, _) => {
                                h521
                            }
                        };
                        nat_le(a522, a523)
                    } else {
                        false
                    } {
                        let a525 = 4u64;
                        let a526 = match v4.clone() {
                            Adt2::C0(_, _, _, _, _, h524, _, _, _, _, _) => {
                                h524
                            }
                        };
                        nat_le(a525, a526)
                    } else {
                        false
                    } {
                        let a528 = 4u64;
                        let a529 = match v5.clone() {
                            Adt2::C0(_, _, _, _, _, h527, _, _, _, _, _) => {
                                h527
                            }
                        };
                        nat_le(a528, a529)
                    } else {
                        false
                    } {
                        let a531 = 4u64;
                        let a532 = match v3.clone() {
                            Adt2::C0(_, _, _, _, _, h530, _, _, _, _, _) => {
                                h530
                            }
                        };
                        nat_le(a531, a532)
                    } else {
                        false
                    } {
                        f19(v0.clone(), v3.clone())?
                    } else {
                        false
                    } {
                        f19(v4.clone(), v3.clone())?
                    } else {
                        false
                    } {
                        f19(v5.clone(), v3.clone())?
                    } else {
                        false
                    } {
                        Ok(Adt10::C4(f44(match v0.clone() {
                            Adt2::C0(_, h533, _, _, _, _, _, _, _, _, _) => {
                                h533
                            }
                        }, match v0.clone() {
                            Adt2::C0(_, _, h534, _, _, _, _, _, _, _, _) => {
                                h534
                            }
                        }, match v0.clone() {
                            Adt2::C0(_, _, _, h535, _, _, _, _, _, _, _) => {
                                h535
                            }
                        }, match v0.clone() {
                            Adt2::C0(_, _, _, _, h536, _, _, _, _, _, _) => {
                                h536
                            }
                        })?, 4u64, f22(v0, 1u64), f22(v3, 1u64), Adt4::C22(f22(v4, 1u64), f22(v5, 1u64))))
                    } else {
                        Ok(Adt10::C1)
                    }
                }
            }
        }
    }
}

pub fn f10(v0: u64, v1: Adt1, v2: Adt2, v3: Option<Adt2>, v4: Adt2) -> R<Adt9> {
    let a537 = v0;
    let a538 = 28u64;
    if if if nat_eq(a537, a538) {
        true
    } else {
        let a539 = v0;
        let a540 = 29u64;
        nat_eq(a539, a540)
    } {
        let a542 = 0u64;
        let a543 = match v1.clone() {
            Adt1::C0(h541, _, _, _, _, _, _, _, _, _, _, _) => {
                h541
            }
        };
        if nat_lt(a542, a543) {
            true
        } else {
            let a545 = 0u64;
            let a546 = match v1.clone() {
                Adt1::C0(_, _, _, h544, _, _, _, _, _, _, _, _) => {
                    h544
                }
            };
            nat_lt(a545, a546)
        }
    } else {
        false
    } {
        let m550 = f45(match v2.clone() {
            Adt2::C0(h547, _, _, _, _, _, _, _, _, _, _) => {
                h547
            }
        }, match v1.clone() {
            Adt1::C0(h548, _, _, _, _, _, _, _, _, _, _, _) => {
                h548
            }
        }, match v1.clone() {
            Adt1::C0(_, _, _, h549, _, _, _, _, _, _, _, _) => {
                h549
            }
        });
        match m550 {
            None => {
                Ok(Adt9::C0(Adt10::C8))
            }
            Some(v5) => {
                if v5 {
                    Ok(Adt9::C0(Adt10::C7))
                } else {
                    f46(v0, v1, v2, v3, v4)
                }
            }
        }
    } else {
        f46(v0, v1, v2, v3, v4)
    }
}

pub fn f11(v0: Adt8, v1: u64, v2: u64, v3: u64) -> R<Adt9> {
    let v4: u64 = match v0.clone() {
        Adt8::C0(_, h551, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h551
        }
    };
    let v5: u64 = match v0.clone() {
        Adt8::C0(_, _, h552, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h552
        }
    };
    let v6: u64 = {
        let a553 = v3;
        let a554 = v5;
        let a555 = 0u64;
        nat_rem(a553, a554, a555)
    };
    let a556 = v1;
    let a557 = 0u64;
    if nat_eq(a556, a557) {
        let a559 = v3;
        let a560 = match v0.clone() {
            Adt8::C0(_, _, _, h558, _, _, _, _, _, _, _, _, _, _, _) => {
                h558
            }
        };
        if nat_lt(a559, a560) {
            Ok(Adt9::C8(Some(f47(match v0.clone() {
                Adt8::C0(_, _, _, _, _, _, _, _, h561, _, _, _, _, _, _) => {
                    h561
                }
            }, {
                let a565 = v6;
                let a566 = {
                    let m564 = match v0.clone() {
                        Adt8::C0(_, _, _, _, _, _, _, _, h563, _, _, _, _, _, _) => {
                            h563
                        }
                    };
                    match m564 {
                        Adt3::C0(h562, _, _, _, _, _, _, _) => {
                            h562
                        }
                    }
                };
                let a567 = 0u64;
                nat_rem(a565, a566, a567)
            }, {
                let a577 = {
                    let a571 = v6;
                    let a572 = {
                        let m570 = match v0.clone() {
                            Adt8::C0(_, _, _, _, _, _, _, _, h569, _, _, _, _, _, _) => {
                                h569
                            }
                        };
                        match m570 {
                            Adt3::C0(h568, _, _, _, _, _, _, _) => {
                                h568
                            }
                        }
                    };
                    let a573 = 0u64;
                    nat_quot(a571, a572, a573)
                };
                let a578 = {
                    let m576 = match v0.clone() {
                        Adt8::C0(_, _, _, _, _, _, _, _, h575, _, _, _, _, _, _) => {
                            h575
                        }
                    };
                    match m576 {
                        Adt3::C0(_, h574, _, _, _, _, _, _) => {
                            h574
                        }
                    }
                };
                let a579 = 0u64;
                nat_rem(a577, a578, a579)
            }, {
                let a589 = {
                    let a583 = v6;
                    let a584 = {
                        let m582 = match v0.clone() {
                            Adt8::C0(_, _, _, _, _, _, _, _, h581, _, _, _, _, _, _) => {
                                h581
                            }
                        };
                        match m582 {
                            Adt3::C0(h580, _, _, _, _, _, _, _) => {
                                h580
                            }
                        }
                    };
                    let a585 = 0u64;
                    nat_quot(a583, a584, a585)
                };
                let a590 = {
                    let m588 = match v0 {
                        Adt8::C0(_, _, _, _, _, _, _, _, h587, _, _, _, _, _, _) => {
                            h587
                        }
                    };
                    match m588 {
                        Adt3::C0(_, h586, _, _, _, _, _, _) => {
                            h586
                        }
                    }
                };
                let a591 = 0u64;
                nat_quot(a589, a590, a591)
            }, {
                let a592 = v3;
                let a593 = v5;
                let a594 = 0u64;
                nat_quot(a592, a593, a594)
            })?)))
        } else {
            Ok(Adt9::C0(Adt10::C1))
        }
    } else {
        let v7: u64 = {
            let a600 = v2;
            let a601 = {
                let a597 = v3;
                let a598 = {
                    let a595 = v1;
                    let a596 = 1u64;
                    if nat_eq(a595, a596) {
                        v4
                    } else {
                        v5
                    }
                };
                let a599 = 0u64;
                nat_quot(a597, a598, a599)
            };
            nat_add(a600, a601)?
        };
        let a603 = v7;
        let a604 = match v0.clone() {
            Adt8::C0(h602, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h602
            }
        };
        if nat_lt(a603, a604) {
            let v8: u64 = {
                let a608 = v7;
                let a609 = {
                    let m607 = match v0.clone() {
                        Adt8::C0(_, _, _, _, _, _, _, _, _, h606, _, _, _, _, _) => {
                            h606
                        }
                    };
                    match m607 {
                        Adt3::C0(h605, _, _, _, _, _, _, _) => {
                            h605
                        }
                    }
                };
                let a610 = 0u64;
                nat_rem(a608, a609, a610)
            };
            let v9: u64 = {
                let a614 = v7;
                let a615 = {
                    let m613 = match v0.clone() {
                        Adt8::C0(_, _, _, _, _, _, _, _, _, h612, _, _, _, _, _) => {
                            h612
                        }
                    };
                    match m613 {
                        Adt3::C0(h611, _, _, _, _, _, _, _) => {
                            h611
                        }
                    }
                };
                let a616 = 0u64;
                nat_quot(a614, a615, a616)
            };
            let v10: u64 = {
                let a620 = v9;
                let a621 = {
                    let m619 = match v0.clone() {
                        Adt8::C0(_, _, _, _, _, _, _, _, _, h618, _, _, _, _, _) => {
                            h618
                        }
                    };
                    match m619 {
                        Adt3::C0(_, h617, _, _, _, _, _, _) => {
                            h617
                        }
                    }
                };
                let a622 = 0u64;
                nat_rem(a620, a621, a622)
            };
            let v11: u64 = {
                let a626 = v9;
                let a627 = {
                    let m625 = match v0.clone() {
                        Adt8::C0(_, _, _, _, _, _, _, _, _, h624, _, _, _, _, _) => {
                            h624
                        }
                    };
                    match m625 {
                        Adt3::C0(_, h623, _, _, _, _, _, _) => {
                            h623
                        }
                    }
                };
                let a628 = 0u64;
                nat_quot(a626, a627, a628)
            };
            let a629 = v1;
            let a630 = 1u64;
            if nat_eq(a629, a630) {
                Ok(Adt9::C8(Some(f47(match v0 {
                    Adt8::C0(_, _, _, _, _, _, _, _, _, h631, _, _, _, _, _) => {
                        h631
                    }
                }, v8, v10, {
                    let a632 = v3;
                    let a633 = v4;
                    let a634 = 0u64;
                    nat_rem(a632, a633, a634)
                }, v11)?)))
            } else {
                let a635 = v1;
                let a636 = 2u64;
                if nat_eq(a635, a636) {
                    Ok(Adt9::C8(Some({
                        let a682 = f47(match v0.clone() {
                            Adt8::C0(_, _, _, _, _, _, _, _, _, _, h637, _, _, _, _) => {
                                h637
                            }
                        }, {
                            let a649 = {
                                let a641 = v8;
                                let a642 = {
                                    let m640 = match v0.clone() {
                                        Adt8::C0(_, _, _, _, _, _, _, _, _, _, _, h639, _, _, _) => {
                                            h639
                                        }
                                    };
                                    match m640 {
                                        Adt6::C0(_, h638, _, _) => {
                                            h638
                                        }
                                    }
                                };
                                nat_mul(a641, a642)?
                            };
                            let a650 = {
                                let a646 = v6;
                                let a647 = {
                                    let m645 = match v0.clone() {
                                        Adt8::C0(_, _, _, _, _, _, _, _, h644, _, _, _, _, _, _) => {
                                            h644
                                        }
                                    };
                                    match m645 {
                                        Adt3::C0(h643, _, _, _, _, _, _, _) => {
                                            h643
                                        }
                                    }
                                };
                                let a648 = 0u64;
                                nat_rem(a646, a647, a648)
                            };
                            nat_add(a649, a650)?
                        }, {
                            let a668 = {
                                let a654 = v10;
                                let a655 = {
                                    let m653 = match v0.clone() {
                                        Adt8::C0(_, _, _, _, _, _, _, _, _, _, _, h652, _, _, _) => {
                                            h652
                                        }
                                    };
                                    match m653 {
                                        Adt6::C0(_, h651, _, _) => {
                                            h651
                                        }
                                    }
                                };
                                nat_mul(a654, a655)?
                            };
                            let a669 = {
                                let a665 = {
                                    let a659 = v6;
                                    let a660 = {
                                        let m658 = match v0.clone() {
                                            Adt8::C0(_, _, _, _, _, _, _, _, h657, _, _, _, _, _, _) => {
                                                h657
                                            }
                                        };
                                        match m658 {
                                            Adt3::C0(h656, _, _, _, _, _, _, _) => {
                                                h656
                                            }
                                        }
                                    };
                                    let a661 = 0u64;
                                    nat_quot(a659, a660, a661)
                                };
                                let a666 = {
                                    let m664 = match v0.clone() {
                                        Adt8::C0(_, _, _, _, _, _, _, _, h663, _, _, _, _, _, _) => {
                                            h663
                                        }
                                    };
                                    match m664 {
                                        Adt3::C0(_, h662, _, _, _, _, _, _) => {
                                            h662
                                        }
                                    }
                                };
                                let a667 = 0u64;
                                nat_rem(a665, a666, a667)
                            };
                            nat_add(a668, a669)?
                        }, {
                            let a679 = {
                                let a673 = v6;
                                let a674 = {
                                    let m672 = match v0.clone() {
                                        Adt8::C0(_, _, _, _, _, _, _, _, h671, _, _, _, _, _, _) => {
                                            h671
                                        }
                                    };
                                    match m672 {
                                        Adt3::C0(h670, _, _, _, _, _, _, _) => {
                                            h670
                                        }
                                    }
                                };
                                let a675 = 0u64;
                                nat_quot(a673, a674, a675)
                            };
                            let a680 = {
                                let m678 = match v0 {
                                    Adt8::C0(_, _, _, _, _, _, _, _, h677, _, _, _, _, _, _) => {
                                        h677
                                    }
                                };
                                match m678 {
                                    Adt3::C0(_, h676, _, _, _, _, _, _) => {
                                        h676
                                    }
                                }
                            };
                            let a681 = 0u64;
                            nat_quot(a679, a680, a681)
                        }, v11)?;
                        let a683 = 4u64;
                        let a684 = 0u64;
                        nat_quot(a682, a683, a684)
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

pub fn f12(v0: Adt8, v1: u64, v2: u64, v3: u64) -> R<Adt9> {
    let a685 = v1;
    let a686 = 0u64;
    if nat_eq(a685, a686) {
        let a688 = v3;
        let a689 = match v0.clone() {
            Adt8::C0(_, _, _, h687, _, _, _, _, _, _, _, _, _, _, _) => {
                h687
            }
        };
        if nat_lt(a688, a689) {
            let v4: Adt3 = match v0.clone() {
                Adt8::C0(_, _, _, _, _, _, _, _, h690, _, _, _, _, _, _) => {
                    h690
                }
            };
            let v5: u64 = {
                let a692 = v3;
                let a693 = match v0.clone() {
                    Adt8::C0(_, _, h691, _, _, _, _, _, _, _, _, _, _, _, _) => {
                        h691
                    }
                };
                let a694 = 0u64;
                nat_quot(a692, a693, a694)
            };
            Ok(Adt9::C8(Some(f47(v4.clone(), f48(v5, v4.clone(), 0u64), f48(v5, v4.clone(), 1u64), f48(v5, v4, 2u64), f49(v0.clone(), v5, {
                let a696 = v3;
                let a697 = match v0 {
                    Adt8::C0(_, _, h695, _, _, _, _, _, _, _, _, _, _, _, _) => {
                        h695
                    }
                };
                let a698 = 0u64;
                nat_rem(a696, a697, a698)
            })?)?)))
        } else {
            Ok(Adt9::C0(Adt10::C1))
        }
    } else {
        let v6: Adt3 = match v0.clone() {
            Adt8::C0(_, _, _, _, _, _, _, _, _, _, h699, _, _, _, _) => {
                h699
            }
        };
        let v7: u64 = {
            let a700 = v1;
            let a701 = 1u64;
            if nat_eq(a700, a701) {
                match v0.clone() {
                    Adt8::C0(_, h702, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                        h702
                    }
                }
            } else {
                match v0.clone() {
                    Adt8::C0(_, _, h703, _, _, _, _, _, _, _, _, _, _, _, _) => {
                        h703
                    }
                }
            }
        };
        let v8: u64 = {
            let a707 = v2;
            let a708 = {
                let a704 = v3;
                let a705 = v7;
                let a706 = 0u64;
                nat_quot(a704, a705, a706)
            };
            nat_add(a707, a708)?
        };
        let v9: u64 = {
            let a709 = v3;
            let a710 = v7;
            let a711 = 0u64;
            nat_rem(a709, a710, a711)
        };
        let v10: u64 = {
            let a713 = v8;
            let a714 = match v6.clone() {
                Adt3::C0(h712, _, _, _, _, _, _, _) => {
                    h712
                }
            };
            let a715 = 0u64;
            nat_rem(a713, a714, a715)
        };
        let v11: u64 = {
            let a717 = v8;
            let a718 = match v6.clone() {
                Adt3::C0(h716, _, _, _, _, _, _, _) => {
                    h716
                }
            };
            let a719 = 0u64;
            nat_quot(a717, a718, a719)
        };
        let v12: u64 = {
            let a721 = v11;
            let a722 = match v6.clone() {
                Adt3::C0(_, h720, _, _, _, _, _, _) => {
                    h720
                }
            };
            let a723 = 0u64;
            nat_rem(a721, a722, a723)
        };
        let v13: u64 = {
            let a725 = v11;
            let a726 = match v6.clone() {
                Adt3::C0(_, h724, _, _, _, _, _, _) => {
                    h724
                }
            };
            let a727 = 0u64;
            nat_quot(a725, a726, a727)
        };
        let a729 = v8;
        let a730 = match v0.clone() {
            Adt8::C0(h728, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h728
            }
        };
        if nat_lt(a729, a730) {
            let a731 = v1;
            let a732 = 1u64;
            if nat_eq(a731, a732) {
                let v14: Adt3 = match v0.clone() {
                    Adt8::C0(_, _, _, _, _, _, _, _, h733, _, _, _, _, _, _) => {
                        h733
                    }
                };
                let v15: Adt3 = match v0.clone() {
                    Adt8::C0(_, _, _, _, _, _, _, _, _, h734, _, _, _, _, _) => {
                        h734
                    }
                };
                let m737 = f50(match v0.clone() {
                    Adt8::C0(_, _, _, _, _, _, _, _, _, _, _, h735, _, _, _) => {
                        h735
                    }
                }, match v15.clone() {
                    Adt3::C0(h736, _, _, _, _, _, _, _) => {
                        h736
                    }
                }, v10, f48(v9, v14.clone(), 0u64))?;
                match m737 {
                    None => {
                        Ok(Adt9::C8(None::<u64>))
                    }
                    Some(v16) => {
                        let m740 = f50(match v0.clone() {
                            Adt8::C0(_, _, _, _, _, _, _, _, _, _, _, _, h738, _, _) => {
                                h738
                            }
                        }, match v15 {
                            Adt3::C0(_, h739, _, _, _, _, _, _) => {
                                h739
                            }
                        }, v12, f48(v9, v14, 1u64))?;
                        match m740 {
                            None => {
                                Ok(Adt9::C8(None::<u64>))
                            }
                            Some(v17) => {
                                f51(v0, v9, v13, v16, v17)
                            }
                        }
                    }
                }
            } else {
                let a741 = v1;
                let a742 = 2u64;
                if nat_eq(a741, a742) {
                    Ok(Adt9::C8(Some({
                        let a744 = match v0.clone() {
                            Adt8::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, h743) => {
                                h743
                            }
                        };
                        let a745 = 0u64;
                        if nat_eq(a744, a745) {
                            f47(v6, v10, v12, v9, v13)?
                        } else {
                            f47(v6.clone(), v10, v12, {
                                let a747 = v13;
                                let a748 = match v6.clone() {
                                    Adt3::C0(_, _, h746, _, _, _, _, _) => {
                                        h746
                                    }
                                };
                                let a749 = 0u64;
                                nat_rem(a747, a748, a749)
                            }, {
                                let a757 = {
                                    let a755 = {
                                        let a751 = v13;
                                        let a752 = match v6 {
                                            Adt3::C0(_, _, h750, _, _, _, _, _) => {
                                                h750
                                            }
                                        };
                                        let a753 = 0u64;
                                        nat_quot(a751, a752, a753)
                                    };
                                    let a756 = match v0 {
                                        Adt8::C0(_, _, h754, _, _, _, _, _, _, _, _, _, _, _, _) => {
                                            h754
                                        }
                                    };
                                    nat_mul(a755, a756)?
                                };
                                let a758 = v9;
                                nat_add(a757, a758)?
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

pub fn f13(v0: u64, v1: Adt3, v2: Adt3, v3: Adt4, v4: u64) -> R<Adt9> {
    f14(v0, v1, v2, v3, v4, false, 0u64, 0u64, 0u64, 0u64, 0u64)
}

pub fn f14(v0: u64, v1: Adt3, v2: Adt3, v3: Adt4, v4: u64, v5: bool, v6: u64, v7: u64, v8: u64, v9: u64, v10: u64) -> R<Adt9> {
    Ok(f52(v3.clone(), {
        let m759 = v3.clone();
        match m759 {
            Adt4::C0 => {
                f53(v0, v1, v2, v3)?
            }
            Adt4::C1 => {
                f53(v0, v1, v2, v3)?
            }
            Adt4::C2(_, _) => {
                f53(v0, v1, v2, v3)?
            }
            Adt4::C3(_, _, _, _, _) => {
                f53(v0, v1, v2, v3)?
            }
            Adt4::C4(_) => {
                f53(v0, v1, v2, v3)?
            }
            Adt4::C5(_, _, _, _) => {
                f53(v0, v1, v2, v3)?
            }
            Adt4::C6(_, _, _) => {
                f53(v0, v1, v2, v3)?
            }
            Adt4::C7(_) => {
                f53(v0, v1, v2, v3)?
            }
            Adt4::C8(_) => {
                f53(v0, v1, v2, v3)?
            }
            Adt4::C9(_, _) => {
                f53(v0, v1, v2, v3)?
            }
            Adt4::C10(_, _, _) => {
                f53(v0, v1, v2, v3)?
            }
            Adt4::C11(_, _) => {
                f53(v0, v1, v2, v3)?
            }
            Adt4::C12(_, _, _) => {
                f53(v0, v1, v2, v3)?
            }
            Adt4::C13(_, _, _, _) => {
                f53(v0, v1, v2, v3)?
            }
            Adt4::C14(_, _, _) => {
                f53(v0, v1, v2, v3)?
            }
            Adt4::C15 => {
                f53(v0, v1, v2, v3)?
            }
            Adt4::C16(_, _) => {
                f53(v0, v1, v2, v3)?
            }
            Adt4::C17 => {
                f61(v0, v1, v2, v4)?
            }
            Adt4::C18(_, _) => {
                f53(v0, v1, v2, v3)?
            }
            Adt4::C19(v64, v65) => {
                f62(v64, v0, v1, v2, v65, v4)?
            }
            Adt4::C20(v66, v67) => {
                f63(v66, v0, v1, v2, v67, v4)?
            }
            Adt4::C21 => {
                f53(v0, v1, v2, v3)?
            }
            Adt4::C22(v68, v69) => {
                f64(v0, v1, v2, v68, v69, v4)?
            }
            Adt4::C23(v83, v84, v85, v86) => {
                f71(v0, v86, v83, v84, v85, v4)?
            }
            Adt4::C24(v82) => {
                if v5 {
                    f69(Adt16::C0(v82), v0, v1, v2, v4, v6, v7, v8, v9, v10)?
                } else {
                    Adt9::C0(Adt10::C5(65u64))
                }
            }
            Adt4::C25(_, _, _) => {
                f53(v0, v1, v2, v3)?
            }
            Adt4::C26(v78, v79, v80, v81) => {
                let a761 = v78;
                let a762 = 64u64;
                if if if nat_eq(a761, a762) {
                    true
                } else {
                    let a763 = v78;
                    let a764 = 65u64;
                    nat_eq(a763, a764)
                } {
                    true
                } else {
                    let a765 = v78;
                    let a766 = 66u64;
                    nat_eq(a765, a766)
                } {
                    if v5 {
                        f69(Adt16::C1(v78, v79, v80, v81), v0, v1, v2, v4, v6, v7, v8, v9, v10)?
                    } else {
                        Adt9::C0(Adt10::C5(64u64))
                    }
                } else {
                    let a767 = v78;
                    let a768 = 0u64;
                    if if nat_eq(a767, a768) {
                        true
                    } else {
                        let a769 = v78;
                        let a770 = 30u64;
                        nat_eq(a769, a770)
                    } {
                        f70(v0, v1, v2, v78, v79, v80, v81, v4)?
                    } else {
                        Adt9::C0(Adt10::C1)
                    }
                }
            }
            Adt4::C27(v77) => {
                f68(v0, v1, v2, v77)?
            }
            Adt4::C28(v73) => {
                f65(v0, v1, v2, v73, v4)?
            }
            Adt4::C29(v74) => {
                f66(v0, v1, v2, v74)?
            }
            Adt4::C30(v75, _) => {
                f67(v0, v1, v2, v75)?
            }
            Adt4::C31(v33, v34) => {
                f54(v0, v1, v2, v33, v34)?
            }
            Adt4::C32(v35) => {
                f55(v0, match v1 {
                    Adt3::C0(h760, _, _, _, _, _, _, _) => {
                        h760
                    }
                }, v35)?
            }
            Adt4::C33(v36) => {
                f56(v0, v1, v36, v2)?
            }
            Adt4::C34(v44, v45, v46, v47) => {
                f60(v0, v1, v44, v2, v45, v46, v47)?
            }
            Adt4::C35(v43) => {
                f59(v0, v43, v1, v2)?
            }
            Adt4::C36(v37, v38, v39) => {
                f57(v0, v1, v37, v2, v38, v39)?
            }
            Adt4::C37(v40, v41, v42) => {
                f58(v0, v1, v40, v41, v42, v2)?
            }
        }
    }))
}

pub fn f15(v0: Adt4) -> R<Adt9> {
    let m771 = v0;
    match m771 {
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
        Adt4::C25(v61, _, v63) => {
            Ok(Adt9::C6({
                let a777 = {
                    let a773 = 0u64;
                    let a774 = v61;
                    if nat_lt(a773, a774) {
                        1u64
                    } else {
                        0u64
                    }
                };
                let a778 = {
                    let a775 = 0u64;
                    let a776 = v63;
                    if nat_lt(a775, a776) {
                        6u64
                    } else {
                        0u64
                    }
                };
                nat_add(a777, a778)?
            }, 0u64, 0u64, false))
        }
        Adt4::C26(v69, _, _, _) => {
            Ok(Adt9::C6({
                let a779 = v69;
                let a780 = 66u64;
                if nat_eq(a779, a780) {
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
                let m772 = v32;
                match m772 {
                    None => {
                        7u64
                    }
                    Some(_) => {
                        15u64
                    }
                }
            }, 0u64, 0u64, false))
        }
    }
}

pub fn f16(v0: bool, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64, v7: u64, v8: u64, v9: u64, v10: u64, v11: u64, v12: u64) -> Adt9 {
    let a781 = v1;
    let a782 = 5u64;
    if if if if if nat_lt(a781, a782) {
        true
    } else {
        f72(v0, v1, v2, v3, v4)
    } {
        let a783 = v5;
        let a784 = 5u64;
        nat_lt(a783, a784)
    } else {
        false
    } {
        let a785 = v7;
        let a786 = 5u64;
        nat_lt(a785, a786)
    } else {
        false
    } {
        let a787 = v9;
        let a788 = 2u64;
        if nat_lt(a787, a788) {
            true
        } else {
            let a789 = v9;
            let a790 = 3u64;
            nat_eq(a789, a790)
        }
    } else {
        false
    } {
        Adt9::C7(f73(v0, v1, v2, v3, v4), f74(v5, v6), f74(v7, v8), f75({
            let a791 = v9;
            let a792 = 0u64;
            if nat_eq(a791, a792) {
                1u64
            } else {
                v9
            }
        }, v10), v11, v12)
    } else {
        Adt9::C0(Adt10::C1)
    }
}

pub fn f17(v0: Adt2, v1: u64) -> R<bool> {
    let a793 = v1;
    let a794 = 0u64;
    if nat_eq(a793, a794) {
        Ok(true)
    } else {
        let a800 = {
            let a797 = f76({
                let a795 = v1;
                let a796 = 1u64;
                nat_sub(a795, a796)
            }, f22(v0.clone(), 1u64))?;
            let a798 = 4u64;
            nat_add(a797, a798)?
        };
        let a801 = match v0 {
            Adt2::C0(_, _, _, _, _, _, _, _, _, h799, _) => {
                h799
            }
        };
        Ok(nat_le(a800, a801))
    }
}

pub fn f18(v0: Adt2, v1: u64) -> R<u64> {
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

pub fn f19(v0: Adt2, v1: Adt2) -> R<bool> {
    let a816 = match v0.clone() {
        Adt2::C0(_, _, _, _, _, _, _, _, _, h815, _) => {
            h815
        }
    };
    let a817 = 0u64;
    if nat_eq(a816, a817) {
        Ok(true)
    } else {
        let a819 = match v1.clone() {
            Adt2::C0(_, _, _, _, _, _, _, _, _, h818, _) => {
                h818
            }
        };
        let a820 = 0u64;
        if nat_eq(a819, a820) {
            Ok(true)
        } else {
            let a822 = match v0.clone() {
                Adt2::C0(_, _, _, _, _, _, _, _, _, _, h821) => {
                    h821
                }
            };
            let a823 = 0u64;
            if nat_eq(a822, a823) {
                Ok(true)
            } else {
                let a825 = match v1.clone() {
                    Adt2::C0(_, _, _, _, _, _, _, _, _, _, h824) => {
                        h824
                    }
                };
                let a826 = 0u64;
                if nat_eq(a825, a826) {
                    Ok(true)
                } else {
                    let a832 = {
                        let a829 = match v0.clone() {
                            Adt2::C0(_, _, _, _, _, _, _, _, _, _, h827) => {
                                h827
                            }
                        };
                        let a830 = match v0.clone() {
                            Adt2::C0(_, _, _, _, _, _, _, _, _, h828, _) => {
                                h828
                            }
                        };
                        nat_add(a829, a830)?
                    };
                    let a833 = match v1.clone() {
                        Adt2::C0(_, _, _, _, _, _, _, _, _, _, h831) => {
                            h831
                        }
                    };
                    if nat_le(a832, a833) {
                        Ok(true)
                    } else {
                        let a839 = {
                            let a836 = match v1.clone() {
                                Adt2::C0(_, _, _, _, _, _, _, _, _, _, h834) => {
                                    h834
                                }
                            };
                            let a837 = match v1 {
                                Adt2::C0(_, _, _, _, _, _, _, _, _, h835, _) => {
                                    h835
                                }
                            };
                            nat_add(a836, a837)?
                        };
                        let a840 = match v0 {
                            Adt2::C0(_, _, _, _, _, _, _, _, _, _, h838) => {
                                h838
                            }
                        };
                        Ok(nat_le(a839, a840))
                    }
                }
            }
        }
    }
}

pub fn f20(v0: Adt2, v1: Adt2) -> bool {
    let a843 = match v0.clone() {
        Adt2::C0(_, h841, _, _, _, _, _, _, _, _, _) => {
            h841
        }
    };
    let a844 = match v1.clone() {
        Adt2::C0(_, h842, _, _, _, _, _, _, _, _, _) => {
            h842
        }
    };
    if if if nat_eq(a843, a844) {
        let a847 = match v0.clone() {
            Adt2::C0(_, _, h845, _, _, _, _, _, _, _, _) => {
                h845
            }
        };
        let a848 = match v1.clone() {
            Adt2::C0(_, _, h846, _, _, _, _, _, _, _, _) => {
                h846
            }
        };
        nat_eq(a847, a848)
    } else {
        false
    } {
        let a851 = match v0.clone() {
            Adt2::C0(_, _, _, h849, _, _, _, _, _, _, _) => {
                h849
            }
        };
        let a852 = match v1.clone() {
            Adt2::C0(_, _, _, h850, _, _, _, _, _, _, _) => {
                h850
            }
        };
        nat_eq(a851, a852)
    } else {
        false
    } {
        let a855 = match v0 {
            Adt2::C0(_, _, _, _, h853, _, _, _, _, _, _) => {
                h853
            }
        };
        let a856 = match v1 {
            Adt2::C0(_, _, _, _, h854, _, _, _, _, _, _) => {
                h854
            }
        };
        nat_eq(a855, a856)
    } else {
        false
    }
}

pub fn f21(v0: Adt2) -> R<bool> {
    let v1: u64 = {
        let a862 = match v0.clone() {
            Adt2::C0(_, _, h857, _, _, _, _, _, _, _, _) => {
                h857
            }
        };
        let a863 = {
            let a860 = match v0.clone() {
                Adt2::C0(_, _, _, h858, _, _, _, _, _, _, _) => {
                    h858
                }
            };
            let a861 = match v0.clone() {
                Adt2::C0(_, _, _, _, h859, _, _, _, _, _, _) => {
                    h859
                }
            };
            nat_mul(a860, a861)?
        };
        nat_mul(a862, a863)?
    };
    let v2: u64 = f18(v0.clone(), 1u64)?;
    let a865 = match v0.clone() {
        Adt2::C0(h864, _, _, _, _, _, _, _, _, _, _) => {
            h864
        }
    };
    let a866 = 0u64;
    if if if if if if if nat_eq(a865, a866) {
        let a868 = match v0.clone() {
            Adt2::C0(_, h867, _, _, _, _, _, _, _, _, _) => {
                h867
            }
        };
        let a869 = 2147483647u64;
        nat_le(a868, a869)
    } else {
        false
    } {
        let a870 = v1;
        let a871 = 2147483647u64;
        nat_le(a870, a871)
    } else {
        false
    } {
        let a873 = match v0.clone() {
            Adt2::C0(_, h872, _, _, _, _, _, _, _, _, _) => {
                h872
            }
        };
        let a874 = 1u64;
        if nat_eq(a873, a874) {
            true
        } else {
            let a876 = match v0.clone() {
                Adt2::C0(_, _, _, _, _, h875, _, _, _, _, _) => {
                    h875
                }
            };
            let a877 = 4u64;
            nat_eq(a876, a877)
        }
    } else {
        false
    } {
        let a878 = v1;
        let a879 = 1u64;
        if nat_le(a878, a879) {
            true
        } else {
            let a884 = {
                let a881 = match v0.clone() {
                    Adt2::C0(_, h880, _, _, _, _, _, _, _, _, _) => {
                        h880
                    }
                };
                let a882 = 4u64;
                nat_mul(a881, a882)?
            };
            let a885 = match v0.clone() {
                Adt2::C0(_, _, _, _, _, _, h883, _, _, _, _) => {
                    h883
                }
            };
            nat_le(a884, a885)
        }
    } else {
        false
    } {
        let a887 = match v0.clone() {
            Adt2::C0(_, _, _, h886, _, _, _, _, _, _, _) => {
                h886
            }
        };
        let a888 = 1u64;
        if nat_eq(a887, a888) {
            true
        } else {
            let a894 = match v0.clone() {
                Adt2::C0(_, _, _, _, _, _, _, h889, _, _, _) => {
                    h889
                }
            };
            let a895 = {
                let a892 = match v0.clone() {
                    Adt2::C0(_, _, h890, _, _, _, _, _, _, _, _) => {
                        h890
                    }
                };
                let a893 = match v0.clone() {
                    Adt2::C0(_, _, _, _, _, _, h891, _, _, _, _) => {
                        h891
                    }
                };
                nat_mul(a892, a893)?
            };
            nat_eq(a894, a895)
        }
    } else {
        false
    } {
        let a897 = match v0.clone() {
            Adt2::C0(_, _, _, _, h896, _, _, _, _, _, _) => {
                h896
            }
        };
        let a898 = 1u64;
        if nat_eq(a897, a898) {
            true
        } else {
            let a907 = match v0.clone() {
                Adt2::C0(_, _, _, _, _, _, _, _, h899, _, _) => {
                    h899
                }
            };
            let a908 = {
                let a905 = match v0.clone() {
                    Adt2::C0(_, _, _, h900, _, _, _, _, _, _, _) => {
                        h900
                    }
                };
                let a906 = {
                    let a903 = match v0.clone() {
                        Adt2::C0(_, _, h901, _, _, _, _, _, _, _, _) => {
                            h901
                        }
                    };
                    let a904 = match v0.clone() {
                        Adt2::C0(_, _, _, _, _, _, h902, _, _, _, _) => {
                            h902
                        }
                    };
                    nat_mul(a903, a904)?
                };
                nat_mul(a905, a906)?
            };
            nat_eq(a907, a908)
        }
    } else {
        false
    } {
        let a909 = v2;
        let a910 = 0u64;
        if nat_eq(a909, a910) {
            Ok(true)
        } else {
            let a916 = {
                let a913 = f76({
                    let a911 = v2;
                    let a912 = 1u64;
                    nat_sub(a911, a912)
                }, f22(v0.clone(), 1u64))?;
                let a914 = 4u64;
                nat_add(a913, a914)?
            };
            let a917 = match v0 {
                Adt2::C0(_, _, _, _, _, _, _, _, _, h915, _) => {
                    h915
                }
            };
            Ok(nat_le(a916, a917))
        }
    } else {
        Ok(false)
    }
}

pub fn f22(v0: Adt2, v1: u64) -> Adt3 {
    Adt3::C0({
        let a919 = match v0.clone() {
            Adt2::C0(_, h918, _, _, _, _, _, _, _, _, _) => {
                h918
            }
        };
        let a920 = v1;
        let a921 = 0u64;
        nat_quot(a919, a920, a921)
    }, match v0.clone() {
        Adt2::C0(_, _, h922, _, _, _, _, _, _, _, _) => {
            h922
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, h923, _, _, _, _, _, _, _) => {
            h923
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, h924, _, _, _, _, _, _) => {
            h924
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, _, h925, _, _, _, _, _) => {
            h925
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, _, _, h926, _, _, _, _) => {
            h926
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, _, _, _, h927, _, _, _) => {
            h927
        }
    }, match v0 {
        Adt2::C0(_, _, _, _, _, _, _, _, h928, _, _) => {
            h928
        }
    })
}

pub fn f23(v0: u64, v1: Option<Adt3>) -> R<u64> {
    let m929 = v1;
    match m929 {
        None => {
            Ok(2u64)
        }
        Some(_) => {
            let a932 = 1u64;
            let a933 = {
                let a930 = 2u64;
                let a931 = v0;
                nat_mul(a930, a931)?
            };
            nat_add(a932, a933)
        }
    }
}

pub fn f24(v0: u64) -> bool {
    let a934 = v0;
    let a935 = 4294967296u64;
    if nat_lt(a934, a935) {
        let a939 = {
            let a936 = v0;
            let a937 = 2147483648u64;
            let a938 = 0u64;
            nat_rem(a936, a937, a938)
        };
        let a940 = 2139095040u64;
        nat_lt(a939, a940)
    } else {
        false
    }
}

pub fn f25(v0: u64) -> R<u64> {
    let a941 = 4u64;
    let a942 = v0;
    nat_mul(a941, a942)
}

pub fn f26(v0: u64) -> R<u64> {
    let a943 = v0;
    let a944 = 0u64;
    if nat_eq(a943, a944) {
        Ok(0u64)
    } else {
        let a947 = 4u64;
        let a948 = {
            let a945 = 3u64;
            let a946 = v0;
            nat_mul(a945, a946)?
        };
        nat_add(a947, a948)
    }
}

pub fn f27(v0: bool, v1: Adt2) -> Adt3 {
    if v0 {
        f22(v1, 1u64)
    } else {
        Adt3::C0(match v1.clone() {
            Adt2::C0(_, h949, _, _, _, _, _, _, _, _, _) => {
                h949
            }
        }, 1u64, match v1.clone() {
            Adt2::C0(_, _, h950, _, _, _, _, _, _, _, _) => {
                h950
            }
        }, match v1.clone() {
            Adt2::C0(_, _, _, h951, _, _, _, _, _, _, _) => {
                h951
            }
        }, match v1.clone() {
            Adt2::C0(_, _, _, _, _, h952, _, _, _, _, _) => {
                h952
            }
        }, 0u64, match v1.clone() {
            Adt2::C0(_, _, _, _, _, _, h953, _, _, _, _) => {
                h953
            }
        }, match v1 {
            Adt2::C0(_, _, _, _, _, _, _, h954, _, _, _) => {
                h954
            }
        })
    }
}

pub fn f28(v0: Adt2) -> R<bool> {
    let a956 = match v0.clone() {
        Adt2::C0(_, h955, _, _, _, _, _, _, _, _, _) => {
            h955
        }
    };
    let a957 = 1u64;
    if if if if nat_eq(a956, a957) {
        true
    } else {
        let a959 = match v0.clone() {
            Adt2::C0(_, _, _, _, _, h958, _, _, _, _, _) => {
                h958
            }
        };
        let a960 = 4u64;
        nat_eq(a959, a960)
    } {
        let a962 = match v0.clone() {
            Adt2::C0(_, _, h961, _, _, _, _, _, _, _, _) => {
                h961
            }
        };
        let a963 = 1u64;
        if nat_eq(a962, a963) {
            true
        } else {
            let a968 = match v0.clone() {
                Adt2::C0(_, _, _, _, _, _, h964, _, _, _, _) => {
                    h964
                }
            };
            let a969 = {
                let a966 = match v0.clone() {
                    Adt2::C0(_, h965, _, _, _, _, _, _, _, _, _) => {
                        h965
                    }
                };
                let a967 = 4u64;
                nat_mul(a966, a967)?
            };
            nat_eq(a968, a969)
        }
    } else {
        false
    } {
        let a971 = match v0.clone() {
            Adt2::C0(_, _, _, h970, _, _, _, _, _, _, _) => {
                h970
            }
        };
        let a972 = 1u64;
        if nat_eq(a971, a972) {
            true
        } else {
            let a980 = match v0.clone() {
                Adt2::C0(_, _, _, _, _, _, _, h973, _, _, _) => {
                    h973
                }
            };
            let a981 = {
                let a978 = {
                    let a976 = match v0.clone() {
                        Adt2::C0(_, h974, _, _, _, _, _, _, _, _, _) => {
                            h974
                        }
                    };
                    let a977 = match v0.clone() {
                        Adt2::C0(_, _, h975, _, _, _, _, _, _, _, _) => {
                            h975
                        }
                    };
                    nat_mul(a976, a977)?
                };
                let a979 = 4u64;
                nat_mul(a978, a979)?
            };
            nat_eq(a980, a981)
        }
    } else {
        false
    } {
        let a983 = match v0.clone() {
            Adt2::C0(_, _, _, _, h982, _, _, _, _, _, _) => {
                h982
            }
        };
        let a984 = 1u64;
        if nat_eq(a983, a984) {
            Ok(true)
        } else {
            let a995 = match v0.clone() {
                Adt2::C0(_, _, _, _, _, _, _, _, h985, _, _) => {
                    h985
                }
            };
            let a996 = {
                let a993 = {
                    let a991 = {
                        let a988 = match v0.clone() {
                            Adt2::C0(_, h986, _, _, _, _, _, _, _, _, _) => {
                                h986
                            }
                        };
                        let a989 = match v0.clone() {
                            Adt2::C0(_, _, h987, _, _, _, _, _, _, _, _) => {
                                h987
                            }
                        };
                        nat_mul(a988, a989)?
                    };
                    let a992 = match v0 {
                        Adt2::C0(_, _, _, h990, _, _, _, _, _, _, _) => {
                            h990
                        }
                    };
                    nat_mul(a991, a992)?
                };
                let a994 = 4u64;
                nat_mul(a993, a994)?
            };
            Ok(nat_eq(a995, a996))
        }
    } else {
        Ok(false)
    }
}

pub fn f29(v0: Adt6, v1: u64) -> R<u64> {
    let a998 = match v0.clone() {
        Adt6::C0(h997, _, _, _) => {
            h997
        }
    };
    let a999 = 0u64;
    if nat_eq(a998, a999) {
        f77(Adt6::C0(1u64, match v0.clone() {
            Adt6::C0(_, h1000, _, _) => {
                h1000
            }
        }, match v0.clone() {
            Adt6::C0(_, _, h1001, _) => {
                h1001
            }
        }, match v0.clone() {
            Adt6::C0(_, _, _, h1002) => {
                h1002
            }
        }), {
            let a1004 = v1;
            let a1005 = match v0 {
                Adt6::C0(_, _, _, h1003) => {
                    h1003
                }
            };
            nat_add(a1004, a1005)?
        })
    } else {
        f77(v0, v1)
    }
}

pub fn f30(v0: u64, v1: u64, v2: u64, v3: u64) -> R<u64> {
    let a1010 = {
        let a1006 = v0;
        let a1007 = v1;
        nat_mul(a1006, a1007)?
    };
    let a1011 = {
        let a1008 = v2;
        let a1009 = f78(v3);
        nat_mul(a1008, a1009)?
    };
    nat_mul(a1010, a1011)
}

pub fn f31(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let a1014 = v0;
    let a1015 = {
        let a1012 = f79(v1)?;
        let a1013 = v2;
        nat_mul(a1012, a1013)?
    };
    nat_add(a1014, a1015)
}

pub fn f32(v0: Adt2) -> Adt14 {
    Adt14::C0(match v0.clone() {
        Adt2::C0(h1016, _, _, _, _, _, _, _, _, _, _) => {
            h1016
        }
    }, match v0.clone() {
        Adt2::C0(_, h1017, _, _, _, _, _, _, _, _, _) => {
            h1017
        }
    }, match v0.clone() {
        Adt2::C0(_, _, h1018, _, _, _, _, _, _, _, _) => {
            h1018
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, h1019, _, _, _, _, _, _, _) => {
            h1019
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, h1020, _, _, _, _, _, _) => {
            h1020
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, _, h1021, _, _, _, _, _) => {
            h1021
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, _, _, h1022, _, _, _, _) => {
            h1022
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, _, _, _, h1023, _, _, _) => {
            h1023
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, _, _, _, _, h1024, _, _) => {
            h1024
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, _, _, _, _, _, h1025, _) => {
            h1025
        }
    }, match v0 {
        Adt2::C0(_, _, _, _, _, _, _, _, _, _, h1026) => {
            h1026
        }
    })
}

pub fn f33(v0: u64, v1: u64, v2: u64, v3: bool, v4: Adt14, v5: Adt14, v6: Adt14) -> R<Adt15> {
    let a1027 = v0;
    let a1028 = 23u64;
    if nat_eq(a1027, a1028) {
        f80(v1, v4, v6)
    } else {
        let a1029 = v0;
        let a1030 = 26u64;
        if nat_eq(a1029, a1030) {
            f81(v1, v2, v4, v6)
        } else {
            let a1031 = v0;
            let a1032 = 25u64;
            if nat_eq(a1031, a1032) {
                f82(v1, v3, v4, v5, v6)
            } else {
                Ok(Adt15::C0)
            }
        }
    }
}

pub fn f34(v0: u64) -> u64 {
    let a1033 = v0;
    let a1034 = 30u64;
    if nat_eq(a1033, a1034) {
        2u64
    } else {
        4u64
    }
}

pub fn f35(v0: u64) -> bool {
    let a1035 = v0;
    let a1036 = 2139095040u64;
    if nat_le(a1035, a1036) {
        true
    } else {
        let a1037 = v0;
        let a1038 = 2147483648u64;
        nat_eq(a1037, a1038)
    }
}

pub fn f36(v0: Adt2, v1: u64) -> bool {
    let a1040 = match v0.clone() {
        Adt2::C0(_, _, _, _, _, h1039, _, _, _, _, _) => {
            h1039
        }
    };
    let a1041 = v1;
    if if nat_eq(a1040, a1041) {
        let a1046 = {
            let a1043 = match v0.clone() {
                Adt2::C0(_, _, _, _, _, _, _, _, _, _, h1042) => {
                    h1042
                }
            };
            let a1044 = v1;
            let a1045 = 0u64;
            nat_rem(a1043, a1044, a1045)
        };
        let a1047 = 0u64;
        nat_eq(a1046, a1047)
    } else {
        false
    } {
        f83(match v0.clone() {
            Adt2::C0(_, _, h1048, _, _, _, _, _, _, _, _) => {
                h1048
            }
        }, match v0.clone() {
            Adt2::C0(_, _, _, h1049, _, _, _, _, _, _, _) => {
                h1049
            }
        }, match v0.clone() {
            Adt2::C0(_, _, _, _, h1050, _, _, _, _, _, _) => {
                h1050
            }
        }, match v0.clone() {
            Adt2::C0(_, _, _, _, _, _, h1051, _, _, _, _) => {
                h1051
            }
        }, match v0.clone() {
            Adt2::C0(_, _, _, _, _, _, _, h1052, _, _, _) => {
                h1052
            }
        }, match v0 {
            Adt2::C0(_, _, _, _, _, _, _, _, h1053, _, _) => {
                h1053
            }
        }, v1)
    } else {
        false
    }
}

pub fn f37(v0: Adt2, v1: Adt2) -> bool {
    if if if if if if f20(v0.clone(), v1.clone()) {
        let a1056 = match v0.clone() {
            Adt2::C0(h1054, _, _, _, _, _, _, _, _, _, _) => {
                h1054
            }
        };
        let a1057 = match v1.clone() {
            Adt2::C0(h1055, _, _, _, _, _, _, _, _, _, _) => {
                h1055
            }
        };
        nat_eq(a1056, a1057)
    } else {
        false
    } {
        let a1060 = match v0.clone() {
            Adt2::C0(_, _, _, _, _, _, _, _, _, _, h1058) => {
                h1058
            }
        };
        let a1061 = match v1.clone() {
            Adt2::C0(_, _, _, _, _, _, _, _, _, _, h1059) => {
                h1059
            }
        };
        nat_eq(a1060, a1061)
    } else {
        false
    } {
        let a1064 = match v0.clone() {
            Adt2::C0(_, _, _, _, _, h1062, _, _, _, _, _) => {
                h1062
            }
        };
        let a1065 = match v1.clone() {
            Adt2::C0(_, _, _, _, _, h1063, _, _, _, _, _) => {
                h1063
            }
        };
        nat_eq(a1064, a1065)
    } else {
        false
    } {
        let a1068 = match v0.clone() {
            Adt2::C0(_, _, _, _, _, _, h1066, _, _, _, _) => {
                h1066
            }
        };
        let a1069 = match v1.clone() {
            Adt2::C0(_, _, _, _, _, _, h1067, _, _, _, _) => {
                h1067
            }
        };
        nat_eq(a1068, a1069)
    } else {
        false
    } {
        let a1072 = match v0.clone() {
            Adt2::C0(_, _, _, _, _, _, _, h1070, _, _, _) => {
                h1070
            }
        };
        let a1073 = match v1.clone() {
            Adt2::C0(_, _, _, _, _, _, _, h1071, _, _, _) => {
                h1071
            }
        };
        nat_eq(a1072, a1073)
    } else {
        false
    } {
        let a1076 = match v0 {
            Adt2::C0(_, _, _, _, _, _, _, _, h1074, _, _) => {
                h1074
            }
        };
        let a1077 = match v1 {
            Adt2::C0(_, _, _, _, _, _, _, _, h1075, _, _) => {
                h1075
            }
        };
        nat_eq(a1076, a1077)
    } else {
        false
    }
}

pub fn f38(v0: u64, v1: u64, v2: u64, v3: u64) -> bool {
    let a1078 = v0;
    let a1079 = 0u64;
    if if if nat_eq(a1078, a1079) {
        true
    } else {
        let a1080 = v1;
        let a1081 = 0u64;
        nat_eq(a1080, a1081)
    } {
        true
    } else {
        let a1082 = v2;
        let a1083 = 0u64;
        nat_eq(a1082, a1083)
    } {
        true
    } else {
        let a1084 = v3;
        let a1085 = 0u64;
        nat_eq(a1084, a1085)
    }
}

pub fn f39(v0: Adt2, v1: u64) -> R<bool> {
    f84({
        let a1087 = match v0.clone() {
            Adt2::C0(_, h1086, _, _, _, _, _, _, _, _, _) => {
                h1086
            }
        };
        let a1088 = v1;
        nat_mul(a1087, a1088)?
    }, match v0.clone() {
        Adt2::C0(_, _, h1089, _, _, _, _, _, _, _, _) => {
            h1089
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, h1090, _, _, _, _, _, _, _) => {
            h1090
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, h1091, _, _, _, _, _, _) => {
            h1091
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, _, _, h1092, _, _, _, _) => {
            h1092
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, _, _, _, h1093, _, _, _) => {
            h1093
        }
    }, match v0 {
        Adt2::C0(_, _, _, _, _, _, _, _, h1094, _, _) => {
            h1094
        }
    })
}

pub fn f40(v0: u64, v1: u64, v2: u64, v3: u64) -> R<u64> {
    let a1095 = v1;
    let a1096 = 0u64;
    if if nat_eq(a1095, a1096) {
        true
    } else {
        let a1097 = v2;
        let a1098 = 0u64;
        nat_eq(a1097, a1098)
    } {
        Ok(0u64)
    } else {
        let a1105 = {
            let a1099 = f85(v1, v2, v3)?;
            let a1100 = f86(v0, v1, v2)?;
            nat_add(a1099, a1100)?
        };
        let a1106 = {
            let a1103 = f87(v0);
            let a1104 = {
                let a1101 = v1;
                let a1102 = v2;
                nat_mul(a1101, a1102)?
            };
            nat_mul(a1103, a1104)?
        };
        nat_add(a1105, a1106)
    }
}

pub fn f41(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let a1107 = v0;
    let a1108 = 0u64;
    if if nat_eq(a1107, a1108) {
        true
    } else {
        let a1109 = v1;
        let a1110 = 0u64;
        nat_eq(a1109, a1110)
    } {
        Ok(3u64)
    } else {
        let a1113 = {
            let a1111 = 3u64;
            let a1112 = v1;
            nat_add(a1111, a1112)?
        };
        let a1114 = v2;
        nat_add(a1113, a1114)
    }
}

pub fn f42(v0: u64) -> bool {
    let a1115 = v0;
    let a1116 = 2139095040u64;
    if nat_le(a1115, a1116) {
        true
    } else {
        let a1117 = v0;
        let a1118 = 2147483648u64;
        nat_eq(a1117, a1118)
    }
}

pub fn f43(v0: u64, v1: u64) -> R<u64> {
    let a1119 = v0;
    let a1120 = 0u64;
    if nat_eq(a1119, a1120) {
        Ok(0u64)
    } else {
        let a1121 = v1;
        let a1122 = f88(v0)?;
        nat_mul(a1121, a1122)
    }
}

pub fn f44(v0: u64, v1: u64, v2: u64, v3: u64) -> R<u64> {
    let a1123 = v0;
    let a1124 = 0u64;
    if if if if nat_eq(a1123, a1124) {
        true
    } else {
        let a1125 = v1;
        let a1126 = 0u64;
        nat_eq(a1125, a1126)
    } {
        true
    } else {
        let a1127 = v2;
        let a1128 = 0u64;
        nat_eq(a1127, a1128)
    } {
        true
    } else {
        let a1129 = v3;
        let a1130 = 0u64;
        nat_eq(a1129, a1130)
    } {
        Ok(0u64)
    } else {
        let a1143 = {
            let a1133 = 2u64;
            let a1134 = {
                let a1131 = v1;
                let a1132 = v2;
                nat_mul(a1131, a1132)?
            };
            nat_mul(a1133, a1134)?
        };
        let a1144 = {
            let a1141 = {
                let a1137 = {
                    let a1135 = v1;
                    let a1136 = v2;
                    nat_mul(a1135, a1136)?
                };
                let a1138 = v3;
                nat_mul(a1137, a1138)?
            };
            let a1142 = {
                let a1139 = 1u64;
                let a1140 = v0;
                nat_add(a1139, a1140)?
            };
            nat_mul(a1141, a1142)?
        };
        nat_add(a1143, a1144)
    }
}

pub fn f45(v0: u64, v1: u64, v2: u64) -> Option<bool> {
    let v3: bool = {
        let a1145 = v1;
        let a1146 = 0u64;
        if if if nat_eq(a1145, a1146) {
            true
        } else {
            let a1147 = v1;
            let a1148 = 10u64;
            nat_eq(a1147, a1148)
        } {
            true
        } else {
            let a1149 = v1;
            let a1150 = 15u64;
            nat_eq(a1149, a1150)
        } {
            true
        } else {
            let a1151 = v1;
            let a1152 = 20u64;
            nat_eq(a1151, a1152)
        }
    };
    let v4: bool = {
        let a1153 = v2;
        let a1154 = 0u64;
        if if if if if nat_eq(a1153, a1154) {
            true
        } else {
            let a1155 = v2;
            let a1156 = 10u64;
            nat_eq(a1155, a1156)
        } {
            true
        } else {
            let a1157 = v2;
            let a1158 = 15u64;
            nat_eq(a1157, a1158)
        } {
            true
        } else {
            let a1159 = v2;
            let a1160 = 20u64;
            nat_eq(a1159, a1160)
        } {
            true
        } else {
            let a1161 = v2;
            let a1162 = 30u64;
            nat_eq(a1161, a1162)
        } {
            true
        } else {
            let a1163 = v2;
            let a1164 = 40u64;
            nat_eq(a1163, a1164)
        }
    };
    if if v3 {
        v4
    } else {
        false
    } {
        Some({
            let a1165 = v0;
            let a1166 = 30u64;
            if if nat_eq(a1165, a1166) {
                let a1167 = v2;
                let a1168 = 10u64;
                nat_eq(a1167, a1168)
            } else {
                false
            } {
                true
            } else {
                if f89(v0) {
                    let a1169 = v2;
                    let a1170 = 10u64;
                    if if nat_eq(a1169, a1170) {
                        true
                    } else {
                        let a1171 = v2;
                        let a1172 = 15u64;
                        nat_eq(a1171, a1172)
                    } {
                        true
                    } else {
                        let a1173 = v2;
                        let a1174 = 20u64;
                        nat_eq(a1173, a1174)
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

pub fn f46(v0: u64, v1: Adt1, v2: Adt2, v3: Option<Adt2>, v4: Adt2) -> R<Adt9> {
    let v5: u64 = match v1.clone() {
        Adt1::C0(h1175, _, _, _, _, _, _, _, _, _, _, _) => {
            h1175
        }
    };
    Ok(Adt9::C0({
        let a1176 = v0;
        let a1177 = 86u64;
        if nat_eq(a1176, a1177) {
            let m1178 = v3;
            match m1178 {
                None => {
                    Adt10::C1
                }
                Some(v6) => {
                    f90(v1, v2, v6, v4)?
                }
            }
        } else {
            let a1179 = v0;
            let a1180 = 55u64;
            if nat_eq(a1179, a1180) {
                let m1181 = v3;
                match m1181 {
                    None => {
                        Adt10::C1
                    }
                    Some(v7) => {
                        f91(v1, v2, v7, v4)?
                    }
                }
            } else {
                let a1182 = v0;
                let a1183 = 22u64;
                if nat_eq(a1182, a1183) {
                    let m1184 = v3;
                    match m1184 {
                        None => {
                            Adt10::C1
                        }
                        Some(v8) => {
                            f92(v2, v8, v4)?
                        }
                    }
                } else {
                    let a1185 = v0;
                    let a1186 = 44u64;
                    if nat_eq(a1185, a1186) {
                        f93(v1, v2, v3, v4)?
                    } else {
                        let a1187 = v0;
                        let a1188 = 52u64;
                        if nat_eq(a1187, a1188) {
                            f94(v1, v2, v4)?
                        } else {
                            let a1189 = v0;
                            let a1190 = 73u64;
                            if nat_eq(a1189, a1190) {
                                f95(v2, v3, v4)?
                            } else {
                                let a1191 = v0;
                                let a1192 = 57u64;
                                if if nat_eq(a1191, a1192) {
                                    true
                                } else {
                                    let a1193 = v0;
                                    let a1194 = 58u64;
                                    nat_eq(a1193, a1194)
                                } {
                                    let m1195 = v3;
                                    match m1195 {
                                        None => {
                                            f96({
                                                let a1196 = v0;
                                                let a1197 = 58u64;
                                                nat_eq(a1196, a1197)
                                            }, v1, v2, v4)?
                                        }
                                        Some(_) => {
                                            Adt10::C1
                                        }
                                    }
                                } else {
                                    let a1198 = v0;
                                    let a1199 = 49u64;
                                    if nat_eq(a1198, a1199) {
                                        f97(v1, v2, v3, v4)?
                                    } else {
                                        let a1200 = v0;
                                        let a1201 = 51u64;
                                        if nat_eq(a1200, a1201) {
                                            f98(v1, v2, v3, v4)?
                                        } else {
                                            let a1202 = v0;
                                            let a1203 = 39u64;
                                            if nat_eq(a1202, a1203) {
                                                f99(v2, v3, v4)?
                                            } else {
                                                let a1204 = v0;
                                                let a1205 = 77u64;
                                                if nat_eq(a1204, a1205) {
                                                    f100(v2, v4)?
                                                } else {
                                                    let a1206 = v0;
                                                    let a1207 = 40u64;
                                                    if nat_eq(a1206, a1207) {
                                                        f101(v2, v3, v4)?
                                                    } else {
                                                        if f102(v0) {
                                                            let a1208 = v0;
                                                            let a1209 = 33u64;
                                                            if if if nat_eq(a1208, a1209) {
                                                                let a1211 = match v2.clone() {
                                                                    Adt2::C0(h1210, _, _, _, _, _, _, _, _, _, _) => {
                                                                        h1210
                                                                    }
                                                                };
                                                                let a1212 = 0u64;
                                                                nat_eq(a1211, a1212)
                                                            } else {
                                                                false
                                                            } {
                                                                let a1214 = match v4.clone() {
                                                                    Adt2::C0(h1213, _, _, _, _, _, _, _, _, _, _) => {
                                                                        h1213
                                                                    }
                                                                };
                                                                let a1215 = 26u64;
                                                                nat_eq(a1214, a1215)
                                                            } else {
                                                                false
                                                            } {
                                                                let m1216 = v3;
                                                                match m1216 {
                                                                    None => {
                                                                        Adt10::C1
                                                                    }
                                                                    Some(v10) => {
                                                                        f103(v2, v10, v4)?
                                                                    }
                                                                }
                                                            } else {
                                                                let a1217 = v0;
                                                                let a1218 = 33u64;
                                                                if if if nat_eq(a1217, a1218) {
                                                                    let a1220 = match v4.clone() {
                                                                        Adt2::C0(h1219, _, _, _, _, _, _, _, _, _, _) => {
                                                                            h1219
                                                                        }
                                                                    };
                                                                    let a1221 = 0u64;
                                                                    nat_eq(a1220, a1221)
                                                                } else {
                                                                    false
                                                                } {
                                                                    f104(match v2.clone() {
                                                                        Adt2::C0(h1222, _, _, _, _, _, _, _, _, _, _) => {
                                                                            h1222
                                                                        }
                                                                    })
                                                                } else {
                                                                    false
                                                                } {
                                                                    f105(v2, v3, v4)?
                                                                } else {
                                                                    f106(v0, v2, {
                                                                        let m1223 = v3;
                                                                        match m1223 {
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
                                                            let a1224 = v0;
                                                            let a1225 = 19u64;
                                                            if nat_eq(a1224, a1225) {
                                                                f107(v2, v4)?
                                                            } else {
                                                                let a1226 = v0;
                                                                let a1227 = 21u64;
                                                                if nat_eq(a1226, a1227) {
                                                                    f108(v5, v2, v3, v4)?
                                                                } else {
                                                                    let a1228 = v0;
                                                                    let a1229 = 61u64;
                                                                    if if if nat_eq(a1228, a1229) {
                                                                        true
                                                                    } else {
                                                                        let a1230 = v0;
                                                                        let a1231 = 62u64;
                                                                        nat_eq(a1230, a1231)
                                                                    } {
                                                                        true
                                                                    } else {
                                                                        let a1232 = v0;
                                                                        let a1233 = 63u64;
                                                                        nat_eq(a1232, a1233)
                                                                    } {
                                                                        f109(v0, v1, v2, v4)?
                                                                    } else {
                                                                        let a1234 = v0;
                                                                        let a1235 = 4u64;
                                                                        if if nat_eq(a1234, a1235) {
                                                                            true
                                                                        } else {
                                                                            let a1236 = v0;
                                                                            let a1237 = 32u64;
                                                                            nat_eq(a1236, a1237)
                                                                        } {
                                                                            f110(v0, v1, v2, v3, v4)?
                                                                        } else {
                                                                            let a1238 = v0;
                                                                            let a1239 = 20u64;
                                                                            if nat_eq(a1238, a1239) {
                                                                                f111(v2, v4)?
                                                                            } else {
                                                                                let a1240 = v0;
                                                                                let a1241 = 42u64;
                                                                                if nat_eq(a1240, a1241) {
                                                                                    f112(4u64, v2, v4)?
                                                                                } else {
                                                                                    let a1242 = v0;
                                                                                    let a1243 = 69u64;
                                                                                    if nat_eq(a1242, a1243) {
                                                                                        let a1244 = v5;
                                                                                        let a1245 = 4u64;
                                                                                        if nat_lt(a1244, a1245) {
                                                                                            f112(v5, v2, v4)?
                                                                                        } else {
                                                                                            Adt10::C1
                                                                                        }
                                                                                    } else {
                                                                                        let a1246 = v0;
                                                                                        let a1247 = 70u64;
                                                                                        if nat_eq(a1246, a1247) {
                                                                                            f113(v5, v2, v4)?
                                                                                        } else {
                                                                                            let a1248 = v0;
                                                                                            let a1249 = 75u64;
                                                                                            if if nat_eq(a1248, a1249) {
                                                                                                true
                                                                                            } else {
                                                                                                let a1250 = v0;
                                                                                                let a1251 = 76u64;
                                                                                                nat_eq(a1250, a1251)
                                                                                            } {
                                                                                                f114({
                                                                                                    let a1252 = v0;
                                                                                                    let a1253 = 76u64;
                                                                                                    nat_eq(a1252, a1253)
                                                                                                }, v1, v2, v4)?
                                                                                            } else {
                                                                                                let a1254 = v0;
                                                                                                let a1255 = 31u64;
                                                                                                if if if nat_eq(a1254, a1255) {
                                                                                                    true
                                                                                                } else {
                                                                                                    let a1256 = v0;
                                                                                                    let a1257 = 47u64;
                                                                                                    nat_eq(a1256, a1257)
                                                                                                } {
                                                                                                    true
                                                                                                } else {
                                                                                                    let a1258 = v0;
                                                                                                    let a1259 = 68u64;
                                                                                                    nat_eq(a1258, a1259)
                                                                                                } {
                                                                                                    f115(v0, v1, v2, v4)?
                                                                                                } else {
                                                                                                    let a1260 = v0;
                                                                                                    let a1261 = 15u64;
                                                                                                    if nat_eq(a1260, a1261) {
                                                                                                        f116(v2, v4)?
                                                                                                    } else {
                                                                                                        let a1262 = v0;
                                                                                                        let a1263 = 17u64;
                                                                                                        if nat_eq(a1262, a1263) {
                                                                                                            f117(v2, v4)?
                                                                                                        } else {
                                                                                                            let a1264 = v0;
                                                                                                            let a1265 = 18u64;
                                                                                                            if nat_eq(a1264, a1265) {
                                                                                                                f118(v2, v3, v4)?
                                                                                                            } else {
                                                                                                                let a1266 = v0;
                                                                                                                let a1267 = 13u64;
                                                                                                                if if if nat_eq(a1266, a1267) {
                                                                                                                    true
                                                                                                                } else {
                                                                                                                    let a1268 = v0;
                                                                                                                    let a1269 = 14u64;
                                                                                                                    nat_eq(a1268, a1269)
                                                                                                                } {
                                                                                                                    true
                                                                                                                } else {
                                                                                                                    let a1270 = v0;
                                                                                                                    let a1271 = 16u64;
                                                                                                                    nat_eq(a1270, a1271)
                                                                                                                } {
                                                                                                                    f119(v0, v2, v4)?
                                                                                                                } else {
                                                                                                                    let m1272 = f120(v0, v5)?;
                                                                                                                    match m1272 {
                                                                                                                        None => {
                                                                                                                            f121(v0, v5, v2, v3, v4)?
                                                                                                                        }
                                                                                                                        Some(v12) => {
                                                                                                                            f122(v12, v2, v4)?
                                                                                                                        }
                                                                                                                    }
                                                                                                                }
                                                                                                            }
                                                                                                        }
                                                                                                    }
                                                                                                }
                                                                                            }
                                                                                        }
                                                                                    }
                                                                                }
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
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

pub fn f47(v0: Adt3, v1: u64, v2: u64, v3: u64, v4: u64) -> R<u64> {
    f123(v1, v2, v3, v4, match v0.clone() {
        Adt3::C0(_, _, _, _, h1273, _, _, _) => {
            h1273
        }
    }, match v0.clone() {
        Adt3::C0(_, _, _, _, _, h1274, _, _) => {
            h1274
        }
    }, match v0.clone() {
        Adt3::C0(_, _, _, _, _, _, h1275, _) => {
            h1275
        }
    }, match v0 {
        Adt3::C0(_, _, _, _, _, _, _, h1276) => {
            h1276
        }
    })
}

pub fn f48(v0: u64, v1: Adt3, v2: u64) -> u64 {
    let v3: u64 = {
        let a1278 = v0;
        let a1279 = match v1.clone() {
            Adt3::C0(h1277, _, _, _, _, _, _, _) => {
                h1277
            }
        };
        let a1280 = 0u64;
        nat_quot(a1278, a1279, a1280)
    };
    let v4: u64 = {
        let a1282 = v3;
        let a1283 = match v1.clone() {
            Adt3::C0(_, h1281, _, _, _, _, _, _) => {
                h1281
            }
        };
        let a1284 = 0u64;
        nat_quot(a1282, a1283, a1284)
    };
    let a1285 = v2;
    let a1286 = 0u64;
    if nat_eq(a1285, a1286) {
        let a1288 = v0;
        let a1289 = match v1 {
            Adt3::C0(h1287, _, _, _, _, _, _, _) => {
                h1287
            }
        };
        let a1290 = 0u64;
        nat_rem(a1288, a1289, a1290)
    } else {
        let a1291 = v2;
        let a1292 = 1u64;
        if nat_eq(a1291, a1292) {
            let a1294 = v3;
            let a1295 = match v1 {
                Adt3::C0(_, h1293, _, _, _, _, _, _) => {
                    h1293
                }
            };
            let a1296 = 0u64;
            nat_rem(a1294, a1295, a1296)
        } else {
            let a1297 = v2;
            let a1298 = 2u64;
            if nat_eq(a1297, a1298) {
                let a1300 = v4;
                let a1301 = match v1 {
                    Adt3::C0(_, _, h1299, _, _, _, _, _) => {
                        h1299
                    }
                };
                let a1302 = 0u64;
                nat_rem(a1300, a1301, a1302)
            } else {
                let a1304 = v4;
                let a1305 = match v1 {
                    Adt3::C0(_, _, h1303, _, _, _, _, _) => {
                        h1303
                    }
                };
                let a1306 = 0u64;
                nat_quot(a1304, a1305, a1306)
            }
        }
    }
}

pub fn f49(v0: Adt8, v1: u64, v2: u64) -> R<u64> {
    let a1308 = match v0.clone() {
        Adt8::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, h1307) => {
            h1307
        }
    };
    let a1309 = 0u64;
    if nat_eq(a1308, a1309) {
        Ok(v2)
    } else {
        let a1314 = {
            let a1311 = v2;
            let a1312 = match v0.clone() {
                Adt8::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, h1310) => {
                    h1310
                }
            };
            nat_mul(a1311, a1312)?
        };
        let a1315 = f48(v1, match v0 {
            Adt8::C0(_, _, _, _, _, _, _, _, h1313, _, _, _, _, _, _) => {
                h1313
            }
        }, 3u64);
        nat_add(a1314, a1315)
    }
}

pub fn f50(v0: Adt6, v1: u64, v2: u64, v3: u64) -> R<Option<u64>> {
    let v4: u64 = {
        let a1325 = {
            let a1322 = {
                let a1317 = v2;
                let a1318 = match v0.clone() {
                    Adt6::C0(_, h1316, _, _) => {
                        h1316
                    }
                };
                nat_mul(a1317, a1318)?
            };
            let a1323 = {
                let a1320 = v3;
                let a1321 = match v0.clone() {
                    Adt6::C0(_, _, _, h1319) => {
                        h1319
                    }
                };
                nat_mul(a1320, a1321)?
            };
            nat_add(a1322, a1323)?
        };
        let a1326 = f124(match v0.clone() {
            Adt6::C0(_, _, h1324, _) => {
                h1324
            }
        });
        nat_add(a1325, a1326)?
    };
    let v5: u64 = f125(match v0 {
        Adt6::C0(_, _, h1327, _) => {
            h1327
        }
    });
    let a1328 = v5;
    let a1329 = v4;
    if if nat_le(a1328, a1329) {
        let a1332 = {
            let a1330 = v4;
            let a1331 = v5;
            nat_sub(a1330, a1331)
        };
        let a1333 = v1;
        nat_lt(a1332, a1333)
    } else {
        false
    } {
        Ok(Some({
            let a1334 = v4;
            let a1335 = v5;
            nat_sub(a1334, a1335)
        }))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f51(v0: Adt8, v1: u64, v2: u64, v3: u64, v4: u64) -> R<Adt9> {
    let v5: Adt3 = match v0.clone() {
        Adt8::C0(_, _, _, _, _, _, _, _, _, h1336, _, _, _, _, _) => {
            h1336
        }
    };
    let v6: Adt3 = match v0.clone() {
        Adt8::C0(_, _, _, _, _, _, _, _, h1337, _, _, _, _, _, _) => {
            h1337
        }
    };
    let v7: u64 = f48(v1, v6.clone(), 2u64);
    let a1339 = match v0.clone() {
        Adt8::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, h1338) => {
            h1338
        }
    };
    let a1340 = 0u64;
    if nat_eq(a1339, a1340) {
        Ok(Adt9::C8(Some(f47(v5, v3, v4, v7, v2)?)))
    } else {
        let v8: u64 = {
            let m1343 = match v0.clone() {
                Adt8::C0(_, _, _, _, _, _, _, _, _, _, h1342, _, _, _, _) => {
                    h1342
                }
            };
            match m1343 {
                Adt3::C0(_, _, h1341, _, _, _, _, _) => {
                    h1341
                }
            }
        };
        let m1349 = f50(match v0.clone() {
            Adt8::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, h1344, _) => {
                h1344
            }
        }, match v5.clone() {
            Adt3::C0(_, _, h1345, _, _, _, _, _) => {
                h1345
            }
        }, {
            let a1346 = v2;
            let a1347 = v8;
            let a1348 = 0u64;
            nat_rem(a1346, a1347, a1348)
        }, v7)?;
        match m1349 {
            None => {
                Ok(Adt9::C8(None::<u64>))
            }
            Some(v9) => {
                let v10: u64 = {
                    let a1356 = {
                        let a1354 = {
                            let a1350 = v2;
                            let a1351 = v8;
                            let a1352 = 0u64;
                            nat_quot(a1350, a1351, a1352)
                        };
                        let a1355 = match v0 {
                            Adt8::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, h1353) => {
                                h1353
                            }
                        };
                        nat_mul(a1354, a1355)?
                    };
                    let a1357 = f48(v1, v6, 3u64);
                    nat_add(a1356, a1357)?
                };
                Ok(Adt9::C8(Some(f47(v5, v3, v4, v9, v10)?)))
            }
        }
    }
}

pub fn f52(v0: Adt4, v1: Adt9) -> Adt9 {
    let m1358 = v1;
    match m1358 {
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
            f126(f127(v0.clone(), v20, v21), v20, v21, 0u64, v22, 2u64, f128(v0), 3u64, v23)
        }
        Adt9::C4(v24, v25, v26, v27, v28, v29, v30) => {
            let v31: u64 = f129(v0.clone(), v24);
            f126(f130(v0, v24), v31, v31, v25, v26, v27, v28, v29, v30)
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

pub fn f53(v0: u64, v1: Adt3, v2: Adt3, v3: Adt4) -> R<Adt9> {
    let m1359 = v3;
    match m1359 {
        Adt4::C0 => {
            Ok(Adt9::C1(false, f76(v0, v1)?, f76(v0, v2)?))
        }
        Adt4::C1 => {
            Ok(Adt9::C1(false, f132(v0, v1, v2.clone())?, f76(v0, v2)?))
        }
        Adt4::C2(v6, v7) => {
            f133(v0, v1, v7, v2, v6)
        }
        Adt4::C3(v8, v9, v10, v11, v12) => {
            f134(v0, v1, v2, v8, v9, v10, v11, v12)
        }
        Adt4::C4(v13) => {
            f135(v0, v1, v2, v13)
        }
        Adt4::C5(v14, v15, v16, v17) => {
            f134(v0, v1, v2, v14, v15, v16, v17, true)
        }
        Adt4::C6(v18, v19, v20) => {
            f136(v0, v1, v2, v18, v19, v20)
        }
        Adt4::C7(v21) => {
            f137(v0, v1, v2, v21)
        }
        Adt4::C8(v22) => {
            f138(v22, v0, v1, v2)
        }
        Adt4::C9(_, _) => {
            Ok(Adt9::C1(true, 0u64, f76(v0, v2)?))
        }
        Adt4::C10(v25, v26, v27) => {
            f139(v0, v1, v2, v25, v26, v27)
        }
        Adt4::C11(v43, v44) => {
            f140(v0, v1, v2, v43, v44)
        }
        Adt4::C12(v45, v46, v47) => {
            f141(v0, v1, v2, v45, v46, v47)
        }
        Adt4::C13(_, v49, _, _) => {
            Ok(Adt9::C3(v49, v49, f76(v0, v1)?, f76(v0, v2)?))
        }
        Adt4::C14(v52, v53, v54) => {
            let a1362 = v52;
            let a1363 = 23u64;
            if nat_eq(a1362, a1363) {
                f142(v0, v1, v2, v53, v54)
            } else {
                Ok(Adt9::C3(v53, v54, f76(v0, v1)?, f76(v0, v2)?))
            }
        }
        Adt4::C15 => {
            f143(v0, v1, v2)
        }
        Adt4::C16(v55, v56) => {
            f144(v0, v1, v2, v55, v56)
        }
        Adt4::C17 => {
            Ok(Adt9::C0(Adt10::C1))
        }
        Adt4::C18(v57, v58) => {
            f145(v0, v1, v58, v57)
        }
        Adt4::C19(_, _) => {
            Ok(Adt9::C0(Adt10::C1))
        }
        Adt4::C20(_, _) => {
            Ok(Adt9::C0(Adt10::C1))
        }
        Adt4::C21 => {
            Ok(Adt9::C1(false, f76(f131(v0, match v2.clone() {
                Adt3::C0(h1360, _, _, _, _, _, _, _) => {
                    h1360
                }
            }, match v2.clone() {
                Adt3::C0(_, h1361, _, _, _, _, _, _) => {
                    h1361
                }
            })?, v1)?, f76(v0, v2)?))
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
        Adt4::C25(v63, v64, v65) => {
            f146(v0, v63, v64, v65)
        }
        Adt4::C26(v71, _, _, _) => {
            let a1364 = v71;
            let a1365 = 64u64;
            if if if nat_eq(a1364, a1365) {
                true
            } else {
                let a1366 = v71;
                let a1367 = 65u64;
                nat_eq(a1366, a1367)
            } {
                true
            } else {
                let a1368 = v71;
                let a1369 = 66u64;
                nat_eq(a1368, a1369)
            } {
                Ok(Adt9::C0(Adt10::C5(64u64)))
            } else {
                Ok(Adt9::C0(Adt10::C1))
            }
        }
        Adt4::C27(v70) => {
            f68(v0, v1, v2, v70)
        }
        Adt4::C28(v66) => {
            f65(v0, v1, v2, v66, 0u64)
        }
        Adt4::C29(v67) => {
            f66(v0, v1, v2, v67)
        }
        Adt4::C30(v68, _) => {
            f67(v0, v1, v2, v68)
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
    }
}

pub fn f54(v0: u64, v1: Adt3, v2: Adt3, v3: u64, v4: u64) -> R<Adt9> {
    let v5: u64 = f147(v3, v4);
    let v6: u64 = {
        let a1370 = v0;
        let a1371 = v5;
        let a1372 = 0u64;
        nat_quot(a1370, a1371, a1372)
    };
    let a1373 = v5;
    let a1374 = 1u64;
    if nat_eq(a1373, a1374) {
        Ok(f126(f148(v3, v4), v3, v4, 0u64, f76(v6, v1)?, 2u64, 0u64, 3u64, f76(v6, v2)?))
    } else {
        let a1378 = {
            let a1375 = v0;
            let a1376 = 2u64;
            let a1377 = 0u64;
            nat_rem(a1375, a1376, a1377)
        };
        let a1379 = 0u64;
        if nat_eq(a1378, a1379) {
            Ok(f126(f148(v3, 0u64), v3, 0u64, 0u64, f76(v6, v1)?, 2u64, 0u64, 1u64, 0u64))
        } else {
            Ok(f126(f148(0u64, v4), 0u64, v4, 1u64, 0u64, 2u64, 0u64, 3u64, f76(v6, v2)?))
        }
    }
}

pub fn f55(v0: u64, v1: u64, v2: u64) -> R<Adt9> {
    let v3: u64 = f149(v1)?;
    let v4: u64 = {
        let a1380 = v0;
        let a1381 = v3;
        let a1382 = 0u64;
        nat_rem(a1380, a1381, a1382)
    };
    let v5: u64 = {
        let a1385 = {
            let a1383 = v1;
            let a1384 = 3u64;
            nat_mul(a1383, a1384)?
        };
        let a1386 = 1u64;
        nat_add(a1385, a1386)?
    };
    let a1387 = v4;
    let a1388 = 0u64;
    if nat_eq(a1387, a1388) {
        Ok(Adt9::C7(Adt11::C0(0u64), Adt12::C2(0u64), Adt12::C2(0u64), Adt13::C0(0u64), 8u64, 8u64))
    } else {
        let a1389 = v4;
        let a1390 = v5;
        if nat_eq(a1389, a1390) {
            Ok(Adt9::C7(Adt11::C0(2u64), Adt12::C1(0u64), Adt12::C2(0u64), Adt13::C0(0u64), 8u64, 8u64))
        } else {
            let v6: u64 = {
                let a1395 = v4;
                let a1396 = {
                    let a1391 = v4;
                    let a1392 = v5;
                    if nat_lt(a1391, a1392) {
                        1u64
                    } else {
                        let a1393 = v5;
                        let a1394 = 1u64;
                        nat_add(a1393, a1394)?
                    }
                };
                nat_sub(a1395, a1396)
            };
            let v7: u64 = {
                let a1397 = v6;
                let a1398 = 3u64;
                let a1399 = 0u64;
                nat_rem(a1397, a1398, a1399)
            };
            let v8: u64 = {
                let a1410 = 4u64;
                let a1411 = {
                    let a1408 = {
                        let a1403 = {
                            let a1400 = v0;
                            let a1401 = v3;
                            let a1402 = 0u64;
                            nat_quot(a1400, a1401, a1402)
                        };
                        let a1404 = v1;
                        nat_mul(a1403, a1404)?
                    };
                    let a1409 = {
                        let a1405 = v6;
                        let a1406 = 3u64;
                        let a1407 = 0u64;
                        nat_quot(a1405, a1406, a1407)
                    };
                    nat_add(a1408, a1409)?
                };
                nat_mul(a1410, a1411)?
            };
            let a1412 = v4;
            let a1413 = v5;
            if nat_lt(a1412, a1413) {
                let a1414 = v7;
                let a1415 = 0u64;
                if nat_eq(a1414, a1415) {
                    Ok(Adt9::C7(Adt11::C1(2u64), Adt12::C0(1u64, v8), Adt12::C0(0u64, v8), Adt13::C0(1u64), 4u64, 4u64))
                } else {
                    let a1416 = v7;
                    let a1417 = 1u64;
                    if nat_eq(a1416, a1417) {
                        Ok(Adt9::C7(Adt11::C0(1u64), Adt12::C1(1u64), Adt12::C2(0u64), Adt13::C0(1u64), 4u64, 4u64))
                    } else {
                        Ok(Adt9::C7(Adt11::C0(0u64), Adt12::C1(0u64), Adt12::C1(1u64), Adt13::C0(0u64), 8u64, 8u64))
                    }
                }
            } else {
                let a1418 = v7;
                let a1419 = 0u64;
                if nat_eq(a1418, a1419) {
                    Ok(Adt9::C7(Adt11::C1(1u64), Adt12::C0(0u64, v8), Adt12::C1(0u64), Adt13::C0(1u64), 4u64, 4u64))
                } else {
                    let a1420 = v7;
                    let a1421 = 1u64;
                    if nat_eq(a1420, a1421) {
                        Ok(Adt9::C7(Adt11::C1(2u64), Adt12::C2(v2), Adt12::C1(1u64), Adt13::C0(1u64), 4u64, 4u64))
                    } else {
                        Ok(Adt9::C7(Adt11::C1(2u64), Adt12::C1(1u64), Adt12::C0(1u64, v8), Adt13::C1(v8), 4u64, 4u64))
                    }
                }
            }
        }
    }
}

pub fn f56(v0: u64, v1: Adt3, v2: Adt3, v3: Adt3) -> R<Adt9> {
    let v4: u64 = {
        let a1422 = v0;
        let a1423 = 6u64;
        let a1424 = 0u64;
        nat_quot(a1422, a1423, a1424)
    };
    let v5: u64 = {
        let a1425 = v0;
        let a1426 = 6u64;
        let a1427 = 0u64;
        nat_rem(a1425, a1426, a1427)
    };
    let a1428 = v5;
    let a1429 = 0u64;
    if nat_eq(a1428, a1429) {
        Ok(Adt9::C7(Adt11::C7(17u64, 0u64, 0u64), Adt12::C0(1u64, f76(v4, v2)?), Adt12::C2(0u64), Adt13::C0(0u64), 4u64, 4u64))
    } else {
        let a1430 = v5;
        let a1431 = 1u64;
        if nat_eq(a1430, a1431) {
            Ok(Adt9::C7(Adt11::C1(1u64), Adt12::C2(1065353216u64), Adt12::C1(0u64), Adt13::C0(1u64), 4u64, 4u64))
        } else {
            let a1432 = v5;
            let a1433 = 2u64;
            if nat_eq(a1432, a1433) {
                Ok(Adt9::C7(Adt11::C1(2u64), Adt12::C0(1u64, f76(v4, v2)?), Adt12::C1(1u64), Adt13::C0(1u64), 4u64, 4u64))
            } else {
                let a1434 = v5;
                let a1435 = 3u64;
                if nat_eq(a1434, a1435) {
                    Ok(Adt9::C7(Adt11::C1(0u64), Adt12::C2(1065353216u64), Adt12::C1(1u64), Adt13::C0(1u64), 4u64, 4u64))
                } else {
                    let a1436 = v5;
                    let a1437 = 4u64;
                    if nat_eq(a1436, a1437) {
                        Ok(Adt9::C7(Adt11::C1(2u64), Adt12::C0(0u64, f76(v4, v1)?), Adt12::C1(0u64), Adt13::C0(0u64), 4u64, 4u64))
                    } else {
                        Ok(Adt9::C7(Adt11::C1(2u64), Adt12::C1(0u64), Adt12::C1(1u64), Adt13::C1(f76(v4, v3)?), 4u64, 4u64))
                    }
                }
            }
        }
    }
}

pub fn f57(v0: u64, v1: Adt3, v2: Adt3, v3: Adt3, v4: bool, v5: u64) -> R<Adt9> {
    let v6: u64 = if v4 {
        3u64
    } else {
        2u64
    };
    let v7: u64 = f150(match v1.clone() {
        Adt3::C0(_, h1438, _, _, _, _, _, _) => {
            h1438
        }
    }, v4)?;
    let v8: u64 = {
        let a1439 = v0;
        let a1440 = v7;
        let a1441 = 0u64;
        nat_rem(a1439, a1440, a1441)
    };
    let v9: u64 = {
        let a1442 = v0;
        let a1443 = v7;
        let a1444 = 0u64;
        nat_quot(a1442, a1443, a1444)
    };
    let a1447 = {
        let a1445 = v8;
        let a1446 = 1u64;
        nat_add(a1445, a1446)?
    };
    let a1448 = v7;
    if nat_eq(a1447, a1448) {
        Ok(Adt9::C7(Adt11::C1(2u64), Adt12::C2(v5), Adt12::C1(0u64), Adt13::C1(f76(v9, v3)?), 4u64, 4u64))
    } else {
        let v10: u64 = {
            let a1449 = v8;
            let a1450 = v6;
            let a1451 = 0u64;
            nat_quot(a1449, a1450, a1451)
        };
        let v11: u64 = {
            let a1452 = v8;
            let a1453 = v6;
            let a1454 = 0u64;
            nat_rem(a1452, a1453, a1454)
        };
        let v12: u64 = {
            let a1456 = v9;
            let a1457 = match v1.clone() {
                Adt3::C0(h1455, _, _, _, _, _, _, _) => {
                    h1455
                }
            };
            let a1458 = 0u64;
            nat_rem(a1456, a1457, a1458)
        };
        let v13: u64 = {
            let a1460 = v9;
            let a1461 = match v1.clone() {
                Adt3::C0(h1459, _, _, _, _, _, _, _) => {
                    h1459
                }
            };
            let a1462 = 0u64;
            nat_quot(a1460, a1461, a1462)
        };
        let v14: u64 = if v4 {
            let a1474 = {
                let a1469 = {
                    let a1464 = match v2.clone() {
                        Adt3::C0(_, _, _, _, h1463, _, _, _) => {
                            h1463
                        }
                    };
                    let a1465 = v12;
                    nat_mul(a1464, a1465)?
                };
                let a1470 = {
                    let a1467 = match v2.clone() {
                        Adt3::C0(_, _, _, _, _, h1466, _, _) => {
                            h1466
                        }
                    };
                    let a1468 = v10;
                    nat_mul(a1467, a1468)?
                };
                nat_add(a1469, a1470)?
            };
            let a1475 = {
                let a1472 = match v2 {
                    Adt3::C0(_, _, _, _, _, _, h1471, _) => {
                        h1471
                    }
                };
                let a1473 = v13;
                nat_mul(a1472, a1473)?
            };
            nat_add(a1474, a1475)?
        } else {
            let a1482 = {
                let a1477 = match v2.clone() {
                    Adt3::C0(_, _, _, _, h1476, _, _, _) => {
                        h1476
                    }
                };
                let a1478 = v10;
                nat_mul(a1477, a1478)?
            };
            let a1483 = {
                let a1480 = match v2 {
                    Adt3::C0(_, _, _, _, _, h1479, _, _) => {
                        h1479
                    }
                };
                let a1481 = v13;
                nat_mul(a1480, a1481)?
            };
            nat_add(a1482, a1483)?
        };
        if if v4 {
            let a1484 = v11;
            let a1485 = 0u64;
            nat_eq(a1484, a1485)
        } else {
            false
        } {
            Ok(Adt9::C7(Adt11::C7(17u64, 0u64, 0u64), Adt12::C0(1u64, v14), Adt12::C2(0u64), Adt13::C0(1u64), 4u64, 4u64))
        } else {
            let a1488 = {
                let a1486 = v11;
                let a1487 = 1u64;
                nat_add(a1486, a1487)?
            };
            let a1489 = v6;
            if nat_eq(a1488, a1489) {
                Ok(Adt9::C7(Adt11::C1(0u64), {
                    let a1490 = v10;
                    let a1491 = 0u64;
                    if nat_eq(a1490, a1491) {
                        Adt12::C2(0u64)
                    } else {
                        Adt12::C1(0u64)
                    }
                }, Adt12::C1(1u64), Adt13::C0(0u64), 4u64, 4u64))
            } else {
                Ok(Adt9::C7(Adt11::C1(2u64), Adt12::C0(0u64, {
                    let a1503 = {
                        let a1498 = {
                            let a1493 = match v1.clone() {
                                Adt3::C0(_, _, _, _, h1492, _, _, _) => {
                                    h1492
                                }
                            };
                            let a1494 = v12;
                            nat_mul(a1493, a1494)?
                        };
                        let a1499 = {
                            let a1496 = match v1.clone() {
                                Adt3::C0(_, _, _, _, _, h1495, _, _) => {
                                    h1495
                                }
                            };
                            let a1497 = v10;
                            nat_mul(a1496, a1497)?
                        };
                        nat_add(a1498, a1499)?
                    };
                    let a1504 = {
                        let a1501 = match v1 {
                            Adt3::C0(_, _, _, _, _, _, h1500, _) => {
                                h1500
                            }
                        };
                        let a1502 = v13;
                        nat_mul(a1501, a1502)?
                    };
                    nat_add(a1503, a1504)?
                }), if v4 {
                    Adt12::C1(1u64)
                } else {
                    Adt12::C0(1u64, v14)
                }, Adt13::C0(1u64), 4u64, 4u64))
            }
        }
    }
}

pub fn f58(v0: u64, v1: Adt3, v2: Adt3, v3: Adt3, v4: Option<Adt3>, v5: Adt3) -> R<Adt9> {
    let v6: u64 = f23(match v2.clone() {
        Adt3::C0(_, h1505, _, _, _, _, _, _) => {
            h1505
        }
    }, v4.clone())?;
    let v7: u64 = {
        let a1506 = v0;
        let a1507 = v6;
        let a1508 = 0u64;
        nat_rem(a1506, a1507, a1508)
    };
    let v8: u64 = {
        let a1509 = v0;
        let a1510 = v6;
        let a1511 = 0u64;
        nat_quot(a1509, a1510, a1511)
    };
    let v9: u64 = {
        let a1513 = v8;
        let a1514 = match v5.clone() {
            Adt3::C0(h1512, _, _, _, _, _, _, _) => {
                h1512
            }
        };
        let a1515 = 0u64;
        nat_rem(a1513, a1514, a1515)
    };
    let v10: u64 = {
        let a1517 = v8;
        let a1518 = match v5.clone() {
            Adt3::C0(h1516, _, _, _, _, _, _, _) => {
                h1516
            }
        };
        let a1519 = 0u64;
        nat_quot(a1517, a1518, a1519)
    };
    let v11: u64 = {
        let a1521 = v10;
        let a1522 = match v2.clone() {
            Adt3::C0(_, h1520, _, _, _, _, _, _) => {
                h1520
            }
        };
        let a1523 = 0u64;
        nat_rem(a1521, a1522, a1523)
    };
    let v12: u64 = {
        let a1525 = v10;
        let a1526 = match v2.clone() {
            Adt3::C0(_, h1524, _, _, _, _, _, _) => {
                h1524
            }
        };
        let a1527 = 0u64;
        nat_quot(a1525, a1526, a1527)
    };
    let a1528 = v7;
    let a1529 = 0u64;
    if nat_eq(a1528, a1529) {
        Ok(Adt9::C7(Adt11::C1(2u64), Adt12::C0(0u64, {
            let a1536 = {
                let a1531 = match v1.clone() {
                    Adt3::C0(_, _, _, _, h1530, _, _, _) => {
                        h1530
                    }
                };
                let a1532 = v9;
                nat_mul(a1531, a1532)?
            };
            let a1537 = {
                let a1534 = match v1 {
                    Adt3::C0(_, _, _, _, _, h1533, _, _) => {
                        h1533
                    }
                };
                let a1535 = v12;
                nat_mul(a1534, a1535)?
            };
            nat_add(a1536, a1537)?
        }), Adt12::C0(2u64, {
            let a1544 = {
                let a1539 = match v3.clone() {
                    Adt3::C0(_, _, _, _, h1538, _, _, _) => {
                        h1538
                    }
                };
                let a1540 = v11;
                nat_mul(a1539, a1540)?
            };
            let a1545 = {
                let a1542 = match v3 {
                    Adt3::C0(_, _, _, _, _, h1541, _, _) => {
                        h1541
                    }
                };
                let a1543 = v12;
                nat_mul(a1542, a1543)?
            };
            nat_add(a1544, a1545)?
        }), Adt13::C0(0u64), 4u64, 4u64))
    } else {
        let m1546 = v4;
        match m1546 {
            None => {
                Ok(Adt9::C7(Adt11::C1(0u64), Adt12::C1(0u64), Adt12::C0(1u64, {
                    let a1558 = {
                        let a1553 = {
                            let a1548 = match v2.clone() {
                                Adt3::C0(_, _, _, _, h1547, _, _, _) => {
                                    h1547
                                }
                            };
                            let a1549 = v9;
                            nat_mul(a1548, a1549)?
                        };
                        let a1554 = {
                            let a1551 = match v2.clone() {
                                Adt3::C0(_, _, _, _, _, h1550, _, _) => {
                                    h1550
                                }
                            };
                            let a1552 = v11;
                            nat_mul(a1551, a1552)?
                        };
                        nat_add(a1553, a1554)?
                    };
                    let a1559 = {
                        let a1556 = match v2 {
                            Adt3::C0(_, _, _, _, _, _, h1555, _) => {
                                h1555
                            }
                        };
                        let a1557 = v12;
                        nat_mul(a1556, a1557)?
                    };
                    nat_add(a1558, a1559)?
                }), Adt13::C1(f76(v8, v5)?), 4u64, 4u64))
            }
            Some(v13) => {
                let v14: u64 = {
                    let a1560 = v7;
                    let a1561 = 1u64;
                    nat_sub(a1560, a1561)
                };
                let v15: u64 = {
                    let a1562 = v14;
                    let a1563 = 2u64;
                    let a1564 = 0u64;
                    nat_quot(a1562, a1563, a1564)
                };
                let a1568 = {
                    let a1565 = v14;
                    let a1566 = 2u64;
                    let a1567 = 0u64;
                    nat_rem(a1565, a1566, a1567)
                };
                let a1569 = 0u64;
                if nat_eq(a1568, a1569) {
                    Ok(Adt9::C7(Adt11::C1(2u64), Adt12::C0(1u64, {
                        let a1581 = {
                            let a1576 = {
                                let a1571 = match v2.clone() {
                                    Adt3::C0(_, _, _, _, h1570, _, _, _) => {
                                        h1570
                                    }
                                };
                                let a1572 = v9;
                                nat_mul(a1571, a1572)?
                            };
                            let a1577 = {
                                let a1574 = match v2.clone() {
                                    Adt3::C0(_, _, _, _, _, h1573, _, _) => {
                                        h1573
                                    }
                                };
                                let a1575 = v15;
                                nat_mul(a1574, a1575)?
                            };
                            nat_add(a1576, a1577)?
                        };
                        let a1582 = {
                            let a1579 = match v2 {
                                Adt3::C0(_, _, _, _, _, _, h1578, _) => {
                                    h1578
                                }
                            };
                            let a1580 = v12;
                            nat_mul(a1579, a1580)?
                        };
                        nat_add(a1581, a1582)?
                    }), Adt12::C0(3u64, {
                        let a1594 = {
                            let a1589 = {
                                let a1584 = match v13.clone() {
                                    Adt3::C0(_, _, _, _, h1583, _, _, _) => {
                                        h1583
                                    }
                                };
                                let a1585 = v11;
                                nat_mul(a1584, a1585)?
                            };
                            let a1590 = {
                                let a1587 = match v13.clone() {
                                    Adt3::C0(_, _, _, _, _, h1586, _, _) => {
                                        h1586
                                    }
                                };
                                let a1588 = v15;
                                nat_mul(a1587, a1588)?
                            };
                            nat_add(a1589, a1590)?
                        };
                        let a1595 = {
                            let a1592 = match v13 {
                                Adt3::C0(_, _, _, _, _, _, h1591, _) => {
                                    h1591
                                }
                            };
                            let a1593 = v12;
                            nat_mul(a1592, a1593)?
                        };
                        nat_add(a1594, a1595)?
                    }), Adt13::C0(1u64), 4u64, 4u64))
                } else {
                    Ok(Adt9::C7(Adt11::C1(0u64), Adt12::C1(0u64), Adt12::C1(1u64), {
                        let a1598 = {
                            let a1596 = v7;
                            let a1597 = 1u64;
                            nat_add(a1596, a1597)?
                        };
                        let a1599 = v6;
                        if nat_eq(a1598, a1599) {
                            Adt13::C1(f76(v8, v5)?)
                        } else {
                            Adt13::C0(0u64)
                        }
                    }, 4u64, 4u64))
                }
            }
        }
    }
}

pub fn f59(v0: u64, v1: u64, v2: Adt3, v3: Adt3) -> R<Adt9> {
    let a1600 = v0;
    let a1601 = v1;
    if nat_lt(a1600, a1601) {
        Ok(Adt9::C7(Adt11::C9, Adt12::C0(0u64, f76(v0, v2)?), Adt12::C2(0u64), Adt13::C0(v0), 4u64, 4u64))
    } else {
        let v4: u64 = {
            let a1602 = v0;
            let a1603 = v1;
            nat_sub(a1602, a1603)
        };
        Ok(Adt9::C7(Adt11::C5(3u64), Adt12::C1(v4), Adt12::C2(0u64), Adt13::C1(f76(v4, v3)?), 4u64, 4u64))
    }
}

pub fn f60(v0: u64, v1: Adt3, v2: Adt3, v3: Adt3, v4: Adt6, v5: Adt6, v6: u64) -> R<Adt9> {
    let v7: u64 = {
        let a1610 = {
            let a1608 = 3u64;
            let a1609 = {
                let a1606 = match v4.clone() {
                    Adt6::C0(h1604, _, _, _) => {
                        h1604
                    }
                };
                let a1607 = match v5.clone() {
                    Adt6::C0(h1605, _, _, _) => {
                        h1605
                    }
                };
                nat_mul(a1606, a1607)?
            };
            nat_mul(a1608, a1609)?
        };
        let a1611 = 2u64;
        nat_add(a1610, a1611)?
    };
    let v8: u64 = {
        let a1612 = v0;
        let a1613 = v7;
        let a1614 = 0u64;
        nat_quot(a1612, a1613, a1614)
    };
    let v9: u64 = {
        let a1615 = v0;
        let a1616 = v7;
        let a1617 = 0u64;
        nat_rem(a1615, a1616, a1617)
    };
    let a1618 = v9;
    let a1619 = 0u64;
    if nat_eq(a1618, a1619) {
        Ok(Adt9::C7(Adt11::C2(0u64, 0u64, false), Adt12::C2(0u64), Adt12::C2(0u64), Adt13::C0(0u64), 4u64, 4u64))
    } else {
        let a1622 = {
            let a1620 = v9;
            let a1621 = 1u64;
            nat_add(a1620, a1621)?
        };
        let a1623 = v7;
        if nat_eq(a1622, a1623) {
            Ok(Adt9::C7(Adt11::C2(0u64, 0u64, false), Adt12::C1(0u64), Adt12::C2(0u64), Adt13::C1(f76(v8, v3)?), 4u64, 4u64))
        } else {
            let v10: u64 = {
                let a1624 = v9;
                let a1625 = 1u64;
                nat_sub(a1624, a1625)
            };
            let v11: u64 = {
                let a1626 = v10;
                let a1627 = 3u64;
                let a1628 = 0u64;
                nat_quot(a1626, a1627, a1628)
            };
            let v12: u64 = {
                let a1630 = v11;
                let a1631 = match v4.clone() {
                    Adt6::C0(h1629, _, _, _) => {
                        h1629
                    }
                };
                let a1632 = 0u64;
                nat_rem(a1630, a1631, a1632)
            };
            let v13: u64 = {
                let a1634 = v11;
                let a1635 = match v4.clone() {
                    Adt6::C0(h1633, _, _, _) => {
                        h1633
                    }
                };
                let a1636 = 0u64;
                nat_quot(a1634, a1635, a1636)
            };
            let v14: u64 = {
                let a1637 = v10;
                let a1638 = 3u64;
                let a1639 = 0u64;
                nat_rem(a1637, a1638, a1639)
            };
            let v15: u64 = f48(v8, v3.clone(), 2u64);
            let m1641 = f50(v4, match v2.clone() {
                Adt3::C0(h1640, _, _, _, _, _, _, _) => {
                    h1640
                }
            }, f48(v8, v3.clone(), 0u64), v12)?;
            match m1641 {
                None => {
                    Ok(Adt9::C7(Adt11::C2(0u64, 0u64, false), Adt12::C1(0u64), Adt12::C2(0u64), Adt13::C0(0u64), 4u64, 4u64))
                }
                Some(v16) => {
                    let m1643 = f50(v5, match v2.clone() {
                        Adt3::C0(_, h1642, _, _, _, _, _, _) => {
                            h1642
                        }
                    }, f48(v8, v3.clone(), 1u64), v13)?;
                    match m1643 {
                        None => {
                            Ok(Adt9::C7(Adt11::C2(0u64, 0u64, false), Adt12::C1(0u64), Adt12::C2(0u64), Adt13::C0(0u64), 4u64, 4u64))
                        }
                        Some(v17) => {
                            let a1644 = v14;
                            let a1645 = 0u64;
                            if nat_eq(a1644, a1645) {
                                Ok(Adt9::C7(Adt11::C2(v6, 0u64, false), Adt12::C0(0u64, f47(v1, v12, v13, 0u64, v15)?), Adt12::C2(0u64), Adt13::C0(1u64), f151(v6), 4u64))
                            } else {
                                let a1646 = v14;
                                let a1647 = 1u64;
                                if nat_eq(a1646, a1647) {
                                    Ok(Adt9::C7(Adt11::C1(2u64), Adt12::C1(1u64), Adt12::C0(1u64, f47(v2, v16, v17, v15, f48(v8, v3, 3u64))?), Adt13::C0(1u64), 4u64, 4u64))
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

pub fn f61(v0: u64, v1: Adt3, v2: Adt3, v3: u64) -> R<Adt9> {
    let v4: u64 = f152(match v1.clone() {
        Adt3::C0(h1648, _, _, _, _, _, _, _) => {
            h1648
        }
    })?;
    let v5: u64 = {
        let a1649 = v0;
        let a1650 = v4;
        let a1651 = 0u64;
        nat_quot(a1649, a1650, a1651)
    };
    let v6: u64 = {
        let a1652 = v0;
        let a1653 = v4;
        let a1654 = 0u64;
        nat_rem(a1652, a1653, a1654)
    };
    let a1655 = v6;
    let a1656 = 0u64;
    if nat_eq(a1655, a1656) {
        Ok(Adt9::C4(19u64, 2u64, 4286578688u64, 2u64, 0u64, 1u64, 0u64))
    } else {
        let a1657 = v6;
        let a1658 = 1u64;
        if nat_eq(a1657, a1658) {
            Ok(Adt9::C4(19u64, 2u64, 0u64, 2u64, 0u64, 1u64, 1u64))
        } else {
            let a1661 = {
                let a1659 = v6;
                let a1660 = 1u64;
                nat_add(a1659, a1660)?
            };
            let a1662 = v4;
            if nat_eq(a1661, a1662) {
                Ok(Adt9::C4(19u64, 1u64, 1u64, 2u64, 0u64, 3u64, f76(v5, v2)?))
            } else {
                let v7: u64 = {
                    let a1663 = v6;
                    let a1664 = 2u64;
                    nat_sub(a1663, a1664)
                };
                let v8: u64 = {
                    let a1665 = v7;
                    let a1666 = 3u64;
                    let a1667 = 0u64;
                    nat_quot(a1665, a1666, a1667)
                };
                let v9: u64 = {
                    let a1668 = v7;
                    let a1669 = 3u64;
                    let a1670 = 0u64;
                    nat_rem(a1668, a1669, a1670)
                };
                let v10: u64 = f76({
                    let a1674 = {
                        let a1672 = v5;
                        let a1673 = match v1.clone() {
                            Adt3::C0(h1671, _, _, _, _, _, _, _) => {
                                h1671
                            }
                        };
                        nat_mul(a1672, a1673)?
                    };
                    let a1675 = v8;
                    nat_add(a1674, a1675)?
                }, v1)?;
                let a1676 = v9;
                let a1677 = 0u64;
                if nat_eq(a1676, a1677) {
                    Ok(Adt9::C4(16u64, 1u64, 0u64, 0u64, v10, 1u64, 2u64))
                } else {
                    let a1678 = v9;
                    let a1679 = 1u64;
                    if nat_eq(a1678, a1679) {
                        let a1680 = v3;
                        let a1681 = 2u64;
                        if nat_eq(a1680, a1681) {
                            Ok(Adt9::C4(19u64, 1u64, 0u64, 2u64, 0u64, 1u64, 0u64))
                        } else {
                            Ok(Adt9::C4(19u64, 0u64, v10, 2u64, 0u64, 1u64, 0u64))
                        }
                    } else {
                        let a1682 = v3;
                        let a1683 = 2u64;
                        if if nat_eq(a1682, a1683) {
                            true
                        } else {
                            let a1684 = v3;
                            let a1685 = 3u64;
                            nat_eq(a1684, a1685)
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

pub fn f62(v0: u64, v1: u64, v2: Adt3, v3: Adt3, v4: Adt3, v5: u64) -> R<Adt9> {
    let v6: u64 = {
        let a1687 = 2u64;
        let a1688 = match v2.clone() {
            Adt3::C0(_, h1686, _, _, _, _, _, _) => {
                h1686
            }
        };
        nat_mul(a1687, a1688)?
    };
    let a1689 = v1;
    let a1690 = v6;
    if nat_lt(a1689, a1690) {
        let a1694 = {
            let a1691 = v1;
            let a1692 = 2u64;
            let a1693 = 0u64;
            nat_rem(a1691, a1692, a1693)
        };
        let a1695 = 0u64;
        if nat_eq(a1694, a1695) {
            Ok(Adt9::C4(19u64, 4u64, f76({
                let a1696 = v1;
                let a1697 = 2u64;
                let a1698 = 0u64;
                nat_quot(a1696, a1697, a1698)
            }, v4)?, 2u64, 0u64, 1u64, 2u64))
        } else {
            let a1699 = v5;
            let a1700 = 2147483648u64;
            if if nat_lt(a1699, a1700) {
                let a1702 = v5;
                let a1703 = match v3 {
                    Adt3::C0(_, h1701, _, _, _, _, _, _) => {
                        h1701
                    }
                };
                nat_lt(a1702, a1703)
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
            let a1706 = match v3.clone() {
                Adt3::C0(h1704, _, _, _, _, _, _, _) => {
                    h1704
                }
            };
            let a1707 = match v3.clone() {
                Adt3::C0(_, h1705, _, _, _, _, _, _) => {
                    h1705
                }
            };
            nat_mul(a1706, a1707)?
        };
        let a1710 = {
            let a1708 = v1;
            let a1709 = v6;
            nat_sub(a1708, a1709)
        };
        let a1711 = v7;
        if nat_lt(a1710, a1711) {
            Ok(Adt9::C4(19u64, 2u64, 0u64, 2u64, 0u64, 3u64, f76({
                let a1712 = v1;
                let a1713 = v6;
                nat_sub(a1712, a1713)
            }, v3)?))
        } else {
            let v8: u64 = {
                let a1716 = {
                    let a1714 = v1;
                    let a1715 = v6;
                    nat_sub(a1714, a1715)
                };
                let a1717 = v7;
                nat_sub(a1716, a1717)
            };
            let v9: u64 = {
                let a1719 = 1u64;
                let a1720 = f153(v0, match v2.clone() {
                    Adt3::C0(h1718, _, _, _, _, _, _, _) => {
                        h1718
                    }
                })?;
                nat_add(a1719, a1720)?
            };
            let v10: u64 = {
                let a1721 = v8;
                let a1722 = v9;
                let a1723 = 0u64;
                nat_quot(a1721, a1722, a1723)
            };
            let v11: u64 = {
                let a1724 = v8;
                let a1725 = v9;
                let a1726 = 0u64;
                nat_rem(a1724, a1725, a1726)
            };
            let a1727 = v11;
            let a1728 = 0u64;
            if nat_eq(a1727, a1728) {
                Ok(Adt9::C4(19u64, 4u64, f76(v10, v4)?, 2u64, 0u64, 1u64, 2u64))
            } else {
                if f154(v0) {
                    let a1732 = {
                        let a1729 = v11;
                        let a1730 = 2u64;
                        let a1731 = 0u64;
                        nat_rem(a1729, a1730, a1731)
                    };
                    let a1733 = 1u64;
                    if nat_eq(a1732, a1733) {
                        Ok(Adt9::C4(6u64, 0u64, f76({
                            let a1742 = {
                                let a1735 = v10;
                                let a1736 = match v2.clone() {
                                    Adt3::C0(h1734, _, _, _, _, _, _, _) => {
                                        h1734
                                    }
                                };
                                nat_mul(a1735, a1736)?
                            };
                            let a1743 = {
                                let a1739 = {
                                    let a1737 = v11;
                                    let a1738 = 1u64;
                                    nat_sub(a1737, a1738)
                                };
                                let a1740 = 2u64;
                                let a1741 = 0u64;
                                nat_quot(a1739, a1740, a1741)
                            };
                            nat_add(a1742, a1743)?
                        }, v2)?, 2u64, 0u64, 1u64, 0u64))
                    } else {
                        Ok(Adt9::C4(4u64, 3u64, f76({
                            let a1752 = {
                                let a1745 = v5;
                                let a1746 = match v2.clone() {
                                    Adt3::C0(h1744, _, _, _, _, _, _, _) => {
                                        h1744
                                    }
                                };
                                nat_mul(a1745, a1746)?
                            };
                            let a1753 = {
                                let a1749 = {
                                    let a1747 = v11;
                                    let a1748 = 1u64;
                                    nat_sub(a1747, a1748)
                                };
                                let a1750 = 2u64;
                                let a1751 = 0u64;
                                nat_quot(a1749, a1750, a1751)
                            };
                            nat_add(a1752, a1753)?
                        }, v3.clone())?, 1u64, 0u64, 3u64, f76({
                            let a1762 = {
                                let a1755 = v5;
                                let a1756 = match v2 {
                                    Adt3::C0(h1754, _, _, _, _, _, _, _) => {
                                        h1754
                                    }
                                };
                                nat_mul(a1755, a1756)?
                            };
                            let a1763 = {
                                let a1759 = {
                                    let a1757 = v11;
                                    let a1758 = 1u64;
                                    nat_sub(a1757, a1758)
                                };
                                let a1760 = 2u64;
                                let a1761 = 0u64;
                                nat_quot(a1759, a1760, a1761)
                            };
                            nat_add(a1762, a1763)?
                        }, v3)?))
                    }
                } else {
                    f155(v0, v10, {
                        let a1764 = v11;
                        let a1765 = 1u64;
                        nat_sub(a1764, a1765)
                    }, v5, v2, v3)
                }
            }
        }
    }
}

pub fn f63(v0: u64, v1: u64, v2: Adt3, v3: Adt3, v4: Adt3, v5: u64) -> R<Adt9> {
    let a1766 = v1;
    let a1767 = f156(v2.clone(), v4.clone())?;
    if nat_lt(a1766, a1767) {
        let a1770 = v1;
        let a1771 = {
            let a1768 = 2u64;
            let a1769 = f157(v4.clone())?;
            nat_mul(a1768, a1769)?
        };
        if nat_lt(a1770, a1771) {
            let a1775 = {
                let a1772 = v1;
                let a1773 = 2u64;
                let a1774 = 0u64;
                nat_rem(a1772, a1773, a1774)
            };
            let a1776 = 0u64;
            if nat_eq(a1775, a1776) {
                f158({
                    let a1777 = v1;
                    let a1778 = 2u64;
                    let a1779 = 0u64;
                    nat_quot(a1777, a1778, a1779)
                }, v4)
            } else {
                if f159(match v2 {
                    Adt3::C0(_, h1780, _, _, _, _, _, _) => {
                        h1780
                    }
                }, v5) {
                    Ok(Adt9::C4(19u64, 1u64, 2u64, 2u64, 0u64, 1u64, 2u64))
                } else {
                    Ok(Adt9::C0(Adt10::C1))
                }
            }
        } else {
            let v6: u64 = {
                let a1783 = v1;
                let a1784 = {
                    let a1781 = 2u64;
                    let a1782 = f157(v4.clone())?;
                    nat_mul(a1781, a1782)?
                };
                nat_sub(a1783, a1784)
            };
            let v7: u64 = {
                let a1788 = v6;
                let a1789 = {
                    let a1786 = 1u64;
                    let a1787 = match v2.clone() {
                        Adt3::C0(h1785, _, _, _, _, _, _, _) => {
                            h1785
                        }
                    };
                    nat_add(a1786, a1787)?
                };
                let a1790 = 0u64;
                nat_quot(a1788, a1789, a1790)
            };
            let v8: u64 = {
                let a1794 = v6;
                let a1795 = {
                    let a1792 = 1u64;
                    let a1793 = match v2.clone() {
                        Adt3::C0(h1791, _, _, _, _, _, _, _) => {
                            h1791
                        }
                    };
                    nat_add(a1792, a1793)?
                };
                let a1796 = 0u64;
                nat_rem(a1794, a1795, a1796)
            };
            let a1797 = v8;
            let a1798 = 0u64;
            if nat_eq(a1797, a1798) {
                f158(v7, v4)
            } else {
                f160(v0, v7, {
                    let a1799 = v8;
                    let a1800 = 1u64;
                    nat_sub(a1799, a1800)
                }, v2, v3, v4, v5)
            }
        }
    } else {
        Ok(Adt9::C0(Adt10::C1))
    }
}

pub fn f64(v0: u64, v1: Adt3, v2: Adt3, v3: Adt3, v4: Adt3, v5: u64) -> R<Adt9> {
    let v6: u64 = {
        let a1803 = match v4.clone() {
            Adt3::C0(h1801, _, _, _, _, _, _, _) => {
                h1801
            }
        };
        let a1804 = match v4.clone() {
            Adt3::C0(_, h1802, _, _, _, _, _, _) => {
                h1802
            }
        };
        nat_mul(a1803, a1804)?
    };
    let v7: u64 = {
        let a1805 = 2u64;
        let a1806 = v6;
        nat_mul(a1805, a1806)?
    };
    let v8: u64 = f161(match v3.clone() {
        Adt3::C0(_, h1807, _, _, _, _, _, _) => {
            h1807
        }
    }, match v3.clone() {
        Adt3::C0(_, _, h1808, _, _, _, _, _) => {
            h1808
        }
    }, match v3.clone() {
        Adt3::C0(_, _, _, h1809, _, _, _, _) => {
            h1809
        }
    });
    let a1810 = v0;
    let a1811 = v7;
    if nat_lt(a1810, a1811) {
        let a1815 = {
            let a1812 = v0;
            let a1813 = 2u64;
            let a1814 = 0u64;
            nat_rem(a1812, a1813, a1814)
        };
        let a1816 = 0u64;
        if nat_eq(a1815, a1816) {
            Ok(Adt9::C4(19u64, 5u64, f76({
                let a1817 = v0;
                let a1818 = 2u64;
                let a1819 = 0u64;
                nat_quot(a1817, a1818, a1819)
            }, v4)?, 2u64, 0u64, 1u64, 2u64))
        } else {
            if f162(v5, v8) {
                Ok(Adt9::C4(19u64, 1u64, 2u64, 2u64, 0u64, 1u64, 2u64))
            } else {
                Ok(Adt9::C0(Adt10::C1))
            }
        }
    } else {
        let v9: u64 = {
            let a1820 = v0;
            let a1821 = v7;
            nat_sub(a1820, a1821)
        };
        let v10: u64 = {
            let a1825 = v9;
            let a1826 = {
                let a1823 = 1u64;
                let a1824 = match v1.clone() {
                    Adt3::C0(h1822, _, _, _, _, _, _, _) => {
                        h1822
                    }
                };
                nat_add(a1823, a1824)?
            };
            let a1827 = 0u64;
            nat_quot(a1825, a1826, a1827)
        };
        let v11: u64 = {
            let a1831 = v9;
            let a1832 = {
                let a1829 = 1u64;
                let a1830 = match v1.clone() {
                    Adt3::C0(h1828, _, _, _, _, _, _, _) => {
                        h1828
                    }
                };
                nat_add(a1829, a1830)?
            };
            let a1833 = 0u64;
            nat_rem(a1831, a1832, a1833)
        };
        let a1834 = v11;
        let a1835 = 0u64;
        if nat_eq(a1834, a1835) {
            Ok(Adt9::C4(19u64, 5u64, f76({
                let a1836 = v10;
                let a1837 = v6;
                let a1838 = 0u64;
                nat_rem(a1836, a1837, a1838)
            }, v4)?, 2u64, 0u64, 1u64, 2u64))
        } else {
            if f162(v5, v8) {
                let v12: u64 = {
                    let a1844 = {
                        let a1840 = v10;
                        let a1841 = match v1.clone() {
                            Adt3::C0(h1839, _, _, _, _, _, _, _) => {
                                h1839
                            }
                        };
                        nat_mul(a1840, a1841)?
                    };
                    let a1845 = {
                        let a1842 = v11;
                        let a1843 = 1u64;
                        nat_sub(a1842, a1843)
                    };
                    nat_add(a1844, a1845)?
                };
                let a1846 = v5;
                let a1847 = 4294967295u64;
                if nat_eq(a1846, a1847) {
                    Ok(Adt9::C1(false, f76(v12, v1)?, f76(v12, v2)?))
                } else {
                    Ok(Adt9::C4(8u64, 0u64, f76(v12, v1)?, 4u64, f76({
                        let a1853 = {
                            let a1849 = v5;
                            let a1850 = match v3.clone() {
                                Adt3::C0(h1848, _, _, _, _, _, _, _) => {
                                    h1848
                                }
                            };
                            nat_mul(a1849, a1850)?
                        };
                        let a1854 = {
                            let a1851 = v11;
                            let a1852 = 1u64;
                            nat_sub(a1851, a1852)
                        };
                        nat_add(a1853, a1854)?
                    }, v3)?, 3u64, f76(v12, v2)?))
                }
            } else {
                Ok(Adt9::C0(Adt10::C1))
            }
        }
    }
}

pub fn f65(v0: u64, v1: Adt3, v2: Adt3, v3: Adt7, v4: u64) -> R<Adt9> {
    let v5: u64 = f163(v3.clone())?;
    let v6: u64 = {
        let a1855 = v0;
        let a1856 = v5;
        let a1857 = 0u64;
        nat_quot(a1855, a1856, a1857)
    };
    let v7: u64 = {
        let a1858 = v0;
        let a1859 = v5;
        let a1860 = 0u64;
        nat_rem(a1858, a1859, a1860)
    };
    let v8: u64 = f164(v3.clone())?;
    let a1861 = v7;
    let a1862 = 0u64;
    if nat_eq(a1861, a1862) {
        Ok(Adt9::C4(19u64, 2u64, if match v3 {
            Adt7::C0(_, _, _, h1863, _) => {
                h1863
            }
        } {
            4286578687u64
        } else {
            0u64
        }, 2u64, 0u64, 1u64, 0u64))
    } else {
        let a1867 = v5;
        let a1868 = {
            let a1865 = v7;
            let a1866 = if match v3.clone() {
                Adt7::C0(_, _, _, h1864, _) => {
                    h1864
                }
            } {
                1u64
            } else {
                2u64
            };
            nat_add(a1865, a1866)?
        };
        if nat_le(a1867, a1868) {
            if match v3.clone() {
                Adt7::C0(_, _, _, h1869, _) => {
                    h1869
                }
            } {
                Ok(Adt9::C4(19u64, 1u64, 0u64, 2u64, 0u64, 3u64, f76(v6, v2)?))
            } else {
                let a1872 = {
                    let a1870 = v7;
                    let a1871 = 2u64;
                    nat_add(a1870, a1871)?
                };
                let a1873 = v5;
                if nat_eq(a1872, a1873) {
                    let v9: u64 = if match v3.clone() {
                        Adt7::C0(_, _, h1874, _, _) => {
                            h1874
                        }
                    } {
                        let a1881 = {
                            let m1877 = match v3.clone() {
                                Adt7::C0(h1876, _, _, _, _) => {
                                    h1876
                                }
                            };
                            match m1877 {
                                Adt6::C0(h1875, _, _, _) => {
                                    h1875
                                }
                            }
                        };
                        let a1882 = {
                            let m1880 = match v3 {
                                Adt7::C0(_, h1879, _, _, _) => {
                                    h1879
                                }
                            };
                            match m1880 {
                                Adt6::C0(h1878, _, _, _) => {
                                    h1878
                                }
                            }
                        };
                        nat_mul(a1881, a1882)?
                    } else {
                        f165(match v3 {
                            Adt7::C0(h1883, _, _, _, _) => {
                                h1883
                            }
                        }, match v1 {
                            Adt3::C0(h1884, _, _, _, _, _, _, _) => {
                                h1884
                            }
                        }, f48(v6, v2, 0u64))?
                    };
                    Ok(Adt9::C4(3u64, 2u64, {
                        let a1885 = v9;
                        let a1886 = 0u64;
                        if nat_eq(a1885, a1886) {
                            1u64
                        } else {
                            v9
                        }
                    }, 2u64, 0u64, 1u64, 1u64))
                } else {
                    Ok(Adt9::C4(5u64, 1u64, 0u64, 1u64, 1u64, 3u64, f76(v6, v2)?))
                }
            }
        } else {
            let v10: u64 = {
                let a1889 = {
                    let a1887 = v7;
                    let a1888 = 1u64;
                    nat_sub(a1887, a1888)
                };
                let a1890 = v8;
                let a1891 = 0u64;
                nat_rem(a1889, a1890, a1891)
            };
            let m1897 = f166(v3.clone(), v1, v2, v6, {
                let a1894 = {
                    let a1892 = v7;
                    let a1893 = 1u64;
                    nat_sub(a1892, a1893)
                };
                let a1895 = v8;
                let a1896 = 0u64;
                nat_quot(a1894, a1895, a1896)
            })?;
            match m1897 {
                None => {
                    Ok(Adt9::C4(19u64, 1u64, 0u64, 2u64, 0u64, 1u64, 0u64))
                }
                Some(v11) => {
                    let a1899 = match v3.clone() {
                        Adt7::C0(_, _, _, _, h1898) => {
                            h1898
                        }
                    };
                    let a1900 = 1u64;
                    if if nat_eq(a1899, a1900) {
                        let a1901 = v10;
                        let a1902 = 0u64;
                        nat_eq(a1901, a1902)
                    } else {
                        false
                    } {
                        Ok(Adt9::C4(6u64, 0u64, v11, 2u64, 0u64, 1u64, 1u64))
                    } else {
                        if match v3.clone() {
                            Adt7::C0(_, _, _, h1903, _) => {
                                h1903
                            }
                        } {
                            let a1906 = {
                                let a1904 = v10;
                                let a1905 = 1u64;
                                nat_add(a1904, a1905)?
                            };
                            let a1907 = v8;
                            if nat_eq(a1906, a1907) {
                                let a1908 = v4;
                                let a1909 = 2u64;
                                if nat_eq(a1908, a1909) {
                                    Ok(Adt9::C4(19u64, 1u64, 0u64, 2u64, 0u64, 1u64, 0u64))
                                } else {
                                    Ok(Adt9::C4(19u64, {
                                        let a1911 = match v3.clone() {
                                            Adt7::C0(_, _, _, _, h1910) => {
                                                h1910
                                            }
                                        };
                                        let a1912 = 1u64;
                                        if nat_eq(a1911, a1912) {
                                            1u64
                                        } else {
                                            0u64
                                        }
                                    }, {
                                        let a1914 = match v3 {
                                            Adt7::C0(_, _, _, _, h1913) => {
                                                h1913
                                            }
                                        };
                                        let a1915 = 1u64;
                                        if nat_eq(a1914, a1915) {
                                            1u64
                                        } else {
                                            v11
                                        }
                                    }, 2u64, 0u64, 1u64, 0u64))
                                }
                            } else {
                                Ok(Adt9::C4(16u64, 1u64, 0u64, {
                                    let a1917 = match v3.clone() {
                                        Adt7::C0(_, _, _, _, h1916) => {
                                            h1916
                                        }
                                    };
                                    let a1918 = 1u64;
                                    if nat_eq(a1917, a1918) {
                                        1u64
                                    } else {
                                        0u64
                                    }
                                }, {
                                    let a1920 = match v3 {
                                        Adt7::C0(_, _, _, _, h1919) => {
                                            h1919
                                        }
                                    };
                                    let a1921 = 1u64;
                                    if nat_eq(a1920, a1921) {
                                        1u64
                                    } else {
                                        v11
                                    }
                                }, 1u64, 2u64))
                            }
                        } else {
                            Ok(Adt9::C4(4u64, 1u64, 0u64, {
                                let a1923 = match v3.clone() {
                                    Adt7::C0(_, _, _, _, h1922) => {
                                        h1922
                                    }
                                };
                                let a1924 = 1u64;
                                if nat_eq(a1923, a1924) {
                                    1u64
                                } else {
                                    0u64
                                }
                            }, {
                                let a1926 = match v3 {
                                    Adt7::C0(_, _, _, _, h1925) => {
                                        h1925
                                    }
                                };
                                let a1927 = 1u64;
                                if nat_eq(a1926, a1927) {
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

pub fn f66(v0: u64, v1: Adt3, v2: Adt3, v3: Adt3) -> R<Adt9> {
    let v4: u64 = f167(match v3.clone() {
        Adt3::C0(h1928, _, _, _, _, _, _, _) => {
            h1928
        }
    })?;
    let v5: u64 = {
        let a1929 = v0;
        let a1930 = v4;
        let a1931 = 0u64;
        nat_rem(a1929, a1930, a1931)
    };
    let v6: u64 = {
        let a1932 = v0;
        let a1933 = v4;
        let a1934 = 0u64;
        nat_quot(a1932, a1933, a1934)
    };
    let a1935 = v5;
    let a1936 = 0u64;
    if nat_eq(a1935, a1936) {
        Ok(Adt9::C4(19u64, 2u64, 0u64, 2u64, 0u64, 1u64, 0u64))
    } else {
        let a1939 = {
            let a1937 = v5;
            let a1938 = 1u64;
            nat_add(a1937, a1938)?
        };
        let a1940 = v4;
        if nat_eq(a1939, a1940) {
            Ok(Adt9::C4(19u64, 1u64, 0u64, 2u64, 0u64, 3u64, f76(v6, v2)?))
        } else {
            let a1944 = {
                let a1941 = v5;
                let a1942 = 2u64;
                let a1943 = 0u64;
                nat_rem(a1941, a1942, a1943)
            };
            let a1945 = 0u64;
            if nat_eq(a1944, a1945) {
                Ok(Adt9::C4(4u64, 1u64, 0u64, 1u64, 1u64, 1u64, 0u64))
            } else {
                let v7: u64 = {
                    let a1948 = {
                        let a1946 = v5;
                        let a1947 = 1u64;
                        nat_sub(a1946, a1947)
                    };
                    let a1949 = 2u64;
                    let a1950 = 0u64;
                    nat_quot(a1948, a1949, a1950)
                };
                let v8: u64 = f48(v6, v2.clone(), 0u64);
                let v9: u64 = f48(v6, v2.clone(), 1u64);
                let v10: u64 = f48(v6, v2, 2u64);
                Ok(Adt9::C4(10u64, 0u64, f47(v1, {
                    let a1951 = v9;
                    let a1952 = v7;
                    nat_add(a1951, a1952)?
                }, v8, v10, 0u64)?, 4u64, f47(v3, v7, v8, 0u64, 0u64)?, 1u64, 1u64))
            }
        }
    }
}

pub fn f67(v0: u64, v1: Adt3, v2: Adt3, v3: Adt6) -> R<Adt9> {
    let v4: u64 = f168(v3.clone())?;
    let v5: u64 = {
        let a1953 = v0;
        let a1954 = v4;
        let a1955 = 0u64;
        nat_rem(a1953, a1954, a1955)
    };
    let v6: u64 = {
        let a1956 = v0;
        let a1957 = v4;
        let a1958 = 0u64;
        nat_quot(a1956, a1957, a1958)
    };
    let a1959 = v5;
    let a1960 = 0u64;
    if nat_eq(a1959, a1960) {
        Ok(Adt9::C4(19u64, 2u64, 0u64, 2u64, 0u64, 1u64, 0u64))
    } else {
        let a1963 = {
            let a1961 = v5;
            let a1962 = 1u64;
            nat_add(a1961, a1962)?
        };
        let a1964 = v4;
        if nat_eq(a1963, a1964) {
            Ok(Adt9::C4(7u64, 1u64, 0u64, 2u64, 0u64, 3u64, f76(v6, v2)?))
        } else {
            let a1968 = {
                let a1965 = v5;
                let a1966 = 2u64;
                let a1967 = 0u64;
                nat_rem(a1965, a1966, a1967)
            };
            let a1969 = 0u64;
            if nat_eq(a1968, a1969) {
                Ok(Adt9::C4(4u64, 1u64, 0u64, 1u64, 1u64, 1u64, 0u64))
            } else {
                let v7: u64 = {
                    let a1971 = f48(v6, v2.clone(), 0u64);
                    let a1972 = match v3.clone() {
                        Adt6::C0(_, _, h1970, _) => {
                            h1970
                        }
                    };
                    nat_add(a1971, a1972)?
                };
                let v8: u64 = {
                    let a1980 = {
                        let a1977 = {
                            let a1974 = v7;
                            let a1975 = match v3.clone() {
                                Adt6::C0(_, h1973, _, _) => {
                                    h1973
                                }
                            };
                            nat_add(a1974, a1975)?
                        };
                        let a1978 = match v3.clone() {
                            Adt6::C0(h1976, _, _, _) => {
                                h1976
                            }
                        };
                        nat_sub(a1977, a1978)
                    };
                    let a1981 = match v3.clone() {
                        Adt6::C0(_, h1979, _, _) => {
                            h1979
                        }
                    };
                    let a1982 = 0u64;
                    nat_quot(a1980, a1981, a1982)
                };
                let v9: u64 = {
                    let a1988 = v8;
                    let a1989 = {
                        let a1985 = {
                            let a1983 = v5;
                            let a1984 = 1u64;
                            nat_sub(a1983, a1984)
                        };
                        let a1986 = 2u64;
                        let a1987 = 0u64;
                        nat_quot(a1985, a1986, a1987)
                    };
                    nat_add(a1988, a1989)?
                };
                let v10: u64 = {
                    let a1991 = v9;
                    let a1992 = match v3.clone() {
                        Adt6::C0(_, h1990, _, _) => {
                            h1990
                        }
                    };
                    nat_mul(a1991, a1992)?
                };
                let v11: u64 = {
                    let a1993 = v7;
                    let a1994 = v10;
                    nat_sub(a1993, a1994)
                };
                let a1996 = v9;
                let a1997 = match v1.clone() {
                    Adt3::C0(_, h1995, _, _, _, _, _, _) => {
                        h1995
                    }
                };
                if if if nat_lt(a1996, a1997) {
                    let a1998 = v10;
                    let a1999 = v7;
                    nat_le(a1998, a1999)
                } else {
                    false
                } {
                    let a2001 = v11;
                    let a2002 = match v3.clone() {
                        Adt6::C0(h2000, _, _, _) => {
                            h2000
                        }
                    };
                    nat_lt(a2001, a2002)
                } else {
                    false
                } {
                    Ok(Adt9::C4(6u64, 0u64, f47(v1, {
                        let a2006 = {
                            let a2004 = f48(v6, v2, 1u64);
                            let a2005 = match v3 {
                                Adt6::C0(h2003, _, _, _) => {
                                    h2003
                                }
                            };
                            nat_mul(a2004, a2005)?
                        };
                        let a2007 = v11;
                        nat_add(a2006, a2007)?
                    }, v9, 0u64, 0u64)?, 2u64, 0u64, 1u64, 1u64))
                } else {
                    Ok(Adt9::C4(19u64, 2u64, 0u64, 2u64, 0u64, 1u64, 1u64))
                }
            }
        }
    }
}

pub fn f68(v0: u64, v1: Adt3, v2: Adt3, v3: Adt5) -> R<Adt9> {
    let v4: u64 = f48(v0, v2.clone(), 0u64);
    let v5: u64 = f48(v0, v2.clone(), 3u64);
    let v6: u64 = {
        let a2009 = v5;
        let a2010 = match v3.clone() {
            Adt5::C0(_, _, _, _, _, _, h2008, _) => {
                h2008
            }
        };
        let a2011 = 0u64;
        nat_rem(a2009, a2010, a2011)
    };
    let v7: u64 = {
        let a2013 = v5;
        let a2014 = match v3.clone() {
            Adt5::C0(_, _, _, _, _, _, h2012, _) => {
                h2012
            }
        };
        let a2015 = 0u64;
        nat_quot(a2013, a2014, a2015)
    };
    let v8: Option<u64> = f50(match v3.clone() {
        Adt5::C0(h2016, _, _, _, _, _, _, _) => {
            h2016
        }
    }, match v1.clone() {
        Adt3::C0(h2017, _, _, _, _, _, _, _) => {
            h2017
        }
    }, f48(v0, v2.clone(), 1u64), {
        let a2021 = v4;
        let a2022 = {
            let m2020 = match v3.clone() {
                Adt5::C0(h2019, _, _, _, _, _, _, _) => {
                    h2019
                }
            };
            match m2020 {
                Adt6::C0(h2018, _, _, _) => {
                    h2018
                }
            }
        };
        let a2023 = 0u64;
        nat_rem(a2021, a2022, a2023)
    })?;
    let v9: u64 = {
        let a2027 = v4;
        let a2028 = {
            let m2026 = match v3.clone() {
                Adt5::C0(h2025, _, _, _, _, _, _, _) => {
                    h2025
                }
            };
            match m2026 {
                Adt6::C0(h2024, _, _, _) => {
                    h2024
                }
            }
        };
        let a2029 = 0u64;
        nat_quot(a2027, a2028, a2029)
    };
    let v10: Option<u64> = f50(match v3.clone() {
        Adt5::C0(_, h2030, _, _, _, _, _, _) => {
            h2030
        }
    }, match v1.clone() {
        Adt3::C0(_, h2031, _, _, _, _, _, _) => {
            h2031
        }
    }, f48(v0, v2.clone(), 2u64), {
        let a2035 = v9;
        let a2036 = {
            let m2034 = match v3.clone() {
                Adt5::C0(_, h2033, _, _, _, _, _, _) => {
                    h2033
                }
            };
            match m2034 {
                Adt6::C0(h2032, _, _, _) => {
                    h2032
                }
            }
        };
        let a2037 = 0u64;
        nat_rem(a2035, a2036, a2037)
    })?;
    let v11: u64 = {
        let a2041 = v9;
        let a2042 = {
            let m2040 = match v3.clone() {
                Adt5::C0(_, h2039, _, _, _, _, _, _) => {
                    h2039
                }
            };
            match m2040 {
                Adt6::C0(h2038, _, _, _) => {
                    h2038
                }
            }
        };
        let a2043 = 0u64;
        nat_quot(a2041, a2042, a2043)
    };
    let v12: Option<u64> = f50(match v3.clone() {
        Adt5::C0(_, _, h2044, _, _, _, _, _) => {
            h2044
        }
    }, match v1.clone() {
        Adt3::C0(_, _, h2045, _, _, _, _, _) => {
            h2045
        }
    }, v6, {
        let a2049 = v11;
        let a2050 = {
            let m2048 = match v3.clone() {
                Adt5::C0(_, _, h2047, _, _, _, _, _) => {
                    h2047
                }
            };
            match m2048 {
                Adt6::C0(h2046, _, _, _) => {
                    h2046
                }
            }
        };
        let a2051 = 0u64;
        nat_rem(a2049, a2050, a2051)
    })?;
    let v13: u64 = {
        let a2055 = v11;
        let a2056 = {
            let m2054 = match v3.clone() {
                Adt5::C0(_, _, h2053, _, _, _, _, _) => {
                    h2053
                }
            };
            match m2054 {
                Adt6::C0(h2052, _, _, _) => {
                    h2052
                }
            }
        };
        let a2057 = 0u64;
        nat_quot(a2055, a2056, a2057)
    };
    let v17: Adt12 = {
        let m2058 = v8;
        match m2058 {
            None => {
                Adt12::C2(0u64)
            }
            Some(v14) => {
                let m2059 = v10;
                match m2059 {
                    None => {
                        Adt12::C2(0u64)
                    }
                    Some(v15) => {
                        let m2060 = v12;
                        match m2060 {
                            None => {
                                Adt12::C2(0u64)
                            }
                            Some(v16) => {
                                Adt12::C0(1u64, {
                                    let a2069 = {
                                        let a2064 = f47(v1, v14, v15, v16, 0u64)?;
                                        let a2065 = {
                                            let a2062 = v7;
                                            let a2063 = match v3.clone() {
                                                Adt5::C0(_, _, _, _, h2061, _, _, _) => {
                                                    h2061
                                                }
                                            };
                                            nat_mul(a2062, a2063)?
                                        };
                                        nat_add(a2064, a2065)?
                                    };
                                    let a2070 = {
                                        let a2067 = v13;
                                        let a2068 = match v3.clone() {
                                            Adt5::C0(_, _, _, h2066, _, _, _, _) => {
                                                h2066
                                            }
                                        };
                                        nat_mul(a2067, a2068)?
                                    };
                                    nat_add(a2069, a2070)?
                                })
                            }
                        }
                    }
                }
            }
        }
    };
    Ok(Adt9::C7(Adt11::C2(match v3.clone() {
        Adt5::C0(_, _, _, _, _, h2071, _, _) => {
            h2071
        }
    }, match v3.clone() {
        Adt5::C0(_, _, _, _, _, _, _, h2072) => {
            h2072
        }
    }, true), v17, Adt12::C2(0u64), Adt13::C1(f76(v0, v2)?), {
        let a2074 = match v3.clone() {
            Adt5::C0(_, _, _, _, _, h2073, _, _) => {
                h2073
            }
        };
        let a2075 = 0u64;
        if nat_eq(a2074, a2075) {
            4u64
        } else {
            2u64
        }
    }, {
        let a2077 = match v3 {
            Adt5::C0(_, _, _, _, _, _, _, h2076) => {
                h2076
            }
        };
        let a2078 = 0u64;
        if nat_eq(a2077, a2078) {
            4u64
        } else {
            2u64
        }
    }))
}

pub fn f69(v0: Adt16, v1: u64, v2: Adt3, v3: Adt3, v4: u64, v5: u64, v6: u64, v7: u64, v8: u64, v9: u64) -> R<Adt9> {
    let m2091 = f169(v0, v1, match v2.clone() {
        Adt3::C0(h2079, _, _, _, _, _, _, _) => {
            h2079
        }
    }, match v2.clone() {
        Adt3::C0(_, h2080, _, _, _, _, _, _) => {
            h2080
        }
    }, match v2.clone() {
        Adt3::C0(_, _, h2081, _, _, _, _, _) => {
            h2081
        }
    }, match v2.clone() {
        Adt3::C0(_, _, _, h2082, _, _, _, _) => {
            h2082
        }
    }, match v2.clone() {
        Adt3::C0(_, _, _, _, h2083, _, _, _) => {
            h2083
        }
    }, match v2.clone() {
        Adt3::C0(_, _, _, _, _, h2084, _, _) => {
            h2084
        }
    }, match v2.clone() {
        Adt3::C0(_, _, _, _, _, _, h2085, _) => {
            h2085
        }
    }, match v2 {
        Adt3::C0(_, _, _, _, _, _, _, h2086) => {
            h2086
        }
    }, v6, v7, v8, v9, match v3.clone() {
        Adt3::C0(_, _, _, _, h2087, _, _, _) => {
            h2087
        }
    }, match v3.clone() {
        Adt3::C0(_, _, _, _, _, h2088, _, _) => {
            h2088
        }
    }, match v3.clone() {
        Adt3::C0(_, _, _, _, _, _, h2089, _) => {
            h2089
        }
    }, match v3 {
        Adt3::C0(_, _, _, _, _, _, _, h2090) => {
            h2090
        }
    }, v4)?;
    match m2091 {
        Adt17::C0 => {
            Ok(Adt9::C0(Adt10::C1))
        }
        Adt17::C1(v10, v11, v12, v13, v14, v15, v16, v17) => {
            Ok(f16(false, v10, v11, 0u64, 0u64, v12, v13, v14, v15, v16, v17, v5, v5))
        }
    }
}

pub fn f70(v0: u64, v1: Adt3, v2: Adt3, v3: u64, v4: u64, v5: u64, v6: u64, v7: u64) -> R<Adt9> {
    let v8: u64 = {
        let a2094 = v6;
        let a2095 = {
            let a2092 = 3u64;
            let a2093 = v5;
            nat_add(a2092, a2093)?
        };
        nat_sub(a2094, a2095)
    };
    let a2097 = v0;
    let a2098 = f40(v3, match v1.clone() {
        Adt3::C0(h2096, _, _, _, _, _, _, _) => {
            h2096
        }
    }, v5, v8)?;
    if nat_lt(a2097, a2098) {
        let v9: u64 = f85(match v1.clone() {
            Adt3::C0(h2099, _, _, _, _, _, _, _) => {
                h2099
            }
        }, v5, v8)?;
        let v10: u64 = f86(v3, match v1.clone() {
            Adt3::C0(h2100, _, _, _, _, _, _, _) => {
                h2100
            }
        }, v5)?;
        let a2101 = v0;
        let a2102 = v9;
        if nat_lt(a2101, a2102) {
            f170(v0, v2, f34(v3), {
                let a2103 = 3u64;
                let a2104 = v5;
                nat_add(a2103, a2104)?
            }, v7)
        } else {
            let a2107 = v0;
            let a2108 = {
                let a2105 = v9;
                let a2106 = v10;
                nat_add(a2105, a2106)?
            };
            if nat_lt(a2107, a2108) {
                let v11: u64 = {
                    let a2109 = v0;
                    let a2110 = v9;
                    nat_sub(a2109, a2110)
                };
                let v12: u64 = f171(v3, match v1.clone() {
                    Adt3::C0(h2111, _, _, _, _, _, _, _) => {
                        h2111
                    }
                })?;
                f172({
                    let a2112 = v11;
                    let a2113 = v12;
                    let a2114 = 0u64;
                    nat_rem(a2112, a2113, a2114)
                }, {
                    let a2115 = v11;
                    let a2116 = v12;
                    let a2117 = 0u64;
                    nat_quot(a2115, a2116, a2117)
                }, v1, v3, v4, v7)
            } else {
                f173({
                    let a2120 = v0;
                    let a2121 = {
                        let a2118 = v9;
                        let a2119 = v10;
                        nat_add(a2118, a2119)?
                    };
                    nat_sub(a2120, a2121)
                }, v1, v2, v3)
            }
        }
    } else {
        Ok(Adt9::C0(Adt10::C1))
    }
}

pub fn f71(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64) -> R<Adt9> {
    let a2122 = v0;
    let a2123 = f26(v1)?;
    if nat_lt(a2122, a2123) {
        let a2124 = v0;
        let a2125 = 4u64;
        if nat_lt(a2124, a2125) {
            let a2126 = v0;
            let a2127 = 0u64;
            if nat_eq(a2126, a2127) {
                Ok(Adt9::C4(16u64, 2u64, v2, 2u64, v3, 1u64, 2u64))
            } else {
                let a2128 = v0;
                let a2129 = 1u64;
                if nat_eq(a2128, a2129) {
                    let a2130 = v5;
                    let a2131 = 0u64;
                    if nat_eq(a2130, a2131) {
                        Ok(Adt9::C4(9u64, 2u64, v3, 2u64, v2, 1u64, 2u64))
                    } else {
                        Ok(Adt9::C0(Adt10::C1))
                    }
                } else {
                    let a2132 = v0;
                    let a2133 = 2u64;
                    if nat_eq(a2132, a2133) {
                        Ok(Adt9::C4(11u64, 1u64, 2u64, 2u64, v4, 1u64, 2u64))
                    } else {
                        Ok(Adt9::C4(20u64, 1u64, 2u64, 2u64, 10u64, 1u64, 2u64))
                    }
                }
            }
        } else {
            let a2134 = v0;
            let a2135 = 4u64;
            if if nat_eq(a2134, a2135) {
                let a2136 = f174(v5)?;
                let a2137 = v1;
                !nat_eq(a2136, a2137)
            } else {
                false
            } {
                Ok(Adt9::C0(Adt10::C1))
            } else {
                let v6: u64 = {
                    let a2140 = {
                        let a2138 = v0;
                        let a2139 = 4u64;
                        nat_sub(a2138, a2139)
                    };
                    let a2141 = 3u64;
                    let a2142 = 0u64;
                    nat_quot(a2140, a2141, a2142)
                };
                let v7: u64 = {
                    let a2145 = {
                        let a2143 = v0;
                        let a2144 = 4u64;
                        nat_sub(a2143, a2144)
                    };
                    let a2146 = 3u64;
                    let a2147 = 0u64;
                    nat_rem(a2145, a2146, a2147)
                };
                let a2148 = v7;
                let a2149 = 0u64;
                if nat_eq(a2148, a2149) {
                    Ok(Adt9::C4(3u64, 2u64, v6, 2u64, 0u64, 1u64, 0u64))
                } else {
                    let a2150 = v7;
                    let a2151 = 1u64;
                    if nat_eq(a2150, a2151) {
                        Ok(Adt9::C4(10u64, 2u64, v4, 1u64, 0u64, 1u64, 0u64))
                    } else {
                        Ok(Adt9::C4(8u64, 2u64, v2, 1u64, 0u64, 3u64, f25(v6)?))
                    }
                }
            }
        }
    } else {
        Ok(Adt9::C0(Adt10::C1))
    }
}

pub fn f72(v0: bool, v1: u64, v2: u64, v3: u64, v4: u64) -> bool {
    if if if if v0 {
        let a2152 = v1;
        let a2153 = 5u64;
        nat_eq(a2152, a2153)
    } else {
        false
    } {
        let a2154 = v2;
        let a2155 = 16u64;
        nat_eq(a2154, a2155)
    } else {
        false
    } {
        let a2156 = v3;
        let a2157 = 0u64;
        nat_eq(a2156, a2157)
    } else {
        false
    } {
        let a2158 = v4;
        let a2159 = 0u64;
        nat_eq(a2158, a2159)
    } else {
        false
    }
}

pub fn f73(v0: bool, v1: u64, v2: u64, v3: u64, v4: u64) -> Adt11 {
    let a2160 = v1;
    let a2161 = 0u64;
    if nat_eq(a2160, a2161) {
        if v0 {
            let a2162 = v3;
            let a2163 = 0u64;
            if if if nat_eq(a2162, a2163) {
                let a2164 = v4;
                let a2165 = 43u64;
                nat_eq(a2164, a2165)
            } else {
                false
            } {
                true
            } else {
                let a2166 = v3;
                let a2167 = 43u64;
                if nat_eq(a2166, a2167) {
                    let a2168 = v4;
                    let a2169 = 0u64;
                    nat_eq(a2168, a2169)
                } else {
                    false
                }
            } {
                f148(v3, v4)
            } else {
                Adt11::C2(v3, v4, false)
            }
        } else {
            Adt11::C0(v2)
        }
    } else {
        let a2170 = v1;
        let a2171 = 1u64;
        if nat_eq(a2170, a2171) {
            Adt11::C1(v2)
        } else {
            let a2172 = v1;
            let a2173 = 2u64;
            if nat_eq(a2172, a2173) {
                Adt11::C5(v2)
            } else {
                let a2174 = v1;
                let a2175 = 3u64;
                if nat_eq(a2174, a2175) {
                    Adt11::C3(v2, v2, false)
                } else {
                    let a2176 = v1;
                    let a2177 = 4u64;
                    if nat_eq(a2176, a2177) {
                        Adt11::C2(v3, v4, false)
                    } else {
                        if f72(v0, v1, v2, v3, v4) {
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

pub fn f74(v0: u64, v1: u64) -> Adt12 {
    let a2178 = v0;
    let a2179 = 0u64;
    if nat_eq(a2178, a2179) {
        Adt12::C0(0u64, v1)
    } else {
        let a2180 = v0;
        let a2181 = 1u64;
        if nat_eq(a2180, a2181) {
            Adt12::C1(v1)
        } else {
            let a2182 = v0;
            let a2183 = 2u64;
            if nat_eq(a2182, a2183) {
                Adt12::C2(v1)
            } else {
                let a2184 = v0;
                let a2185 = 3u64;
                if nat_eq(a2184, a2185) {
                    Adt12::C3(v1)
                } else {
                    let a2186 = v0;
                    let a2187 = 13u64;
                    if nat_lt(a2186, a2187) {
                        Adt12::C0({
                            let a2188 = v0;
                            let a2189 = 3u64;
                            nat_sub(a2188, a2189)
                        }, v1)
                    } else {
                        Adt12::C4
                    }
                }
            }
        }
    }
}

pub fn f75(v0: u64, v1: u64) -> Adt13 {
    let a2190 = v0;
    let a2191 = 1u64;
    if nat_eq(a2190, a2191) {
        Adt13::C0(v1)
    } else {
        let a2192 = v0;
        let a2193 = 3u64;
        if if nat_eq(a2192, a2193) {
            true
        } else {
            let a2194 = v0;
            let a2195 = 6u64;
            nat_eq(a2194, a2195)
        } {
            Adt13::C1(v1)
        } else {
            Adt13::C2
        }
    }
}

pub fn f76(v0: u64, v1: Adt3) -> R<u64> {
    let v2: u64 = {
        let a2197 = v0;
        let a2198 = match v1.clone() {
            Adt3::C0(h2196, _, _, _, _, _, _, _) => {
                h2196
            }
        };
        let a2199 = 0u64;
        nat_quot(a2197, a2198, a2199)
    };
    let v3: u64 = {
        let a2201 = v2;
        let a2202 = match v1.clone() {
            Adt3::C0(_, h2200, _, _, _, _, _, _) => {
                h2200
            }
        };
        let a2203 = 0u64;
        nat_quot(a2201, a2202, a2203)
    };
    f123({
        let a2205 = v0;
        let a2206 = match v1.clone() {
            Adt3::C0(h2204, _, _, _, _, _, _, _) => {
                h2204
            }
        };
        let a2207 = 0u64;
        nat_rem(a2205, a2206, a2207)
    }, {
        let a2209 = v2;
        let a2210 = match v1.clone() {
            Adt3::C0(_, h2208, _, _, _, _, _, _) => {
                h2208
            }
        };
        let a2211 = 0u64;
        nat_rem(a2209, a2210, a2211)
    }, {
        let a2213 = v3;
        let a2214 = match v1.clone() {
            Adt3::C0(_, _, h2212, _, _, _, _, _) => {
                h2212
            }
        };
        let a2215 = 0u64;
        nat_rem(a2213, a2214, a2215)
    }, {
        let a2217 = v3;
        let a2218 = match v1.clone() {
            Adt3::C0(_, _, h2216, _, _, _, _, _) => {
                h2216
            }
        };
        let a2219 = 0u64;
        nat_quot(a2217, a2218, a2219)
    }, match v1.clone() {
        Adt3::C0(_, _, _, _, h2220, _, _, _) => {
            h2220
        }
    }, match v1.clone() {
        Adt3::C0(_, _, _, _, _, h2221, _, _) => {
            h2221
        }
    }, match v1.clone() {
        Adt3::C0(_, _, _, _, _, _, h2222, _) => {
            h2222
        }
    }, match v1 {
        Adt3::C0(_, _, _, _, _, _, _, h2223) => {
            h2223
        }
    })
}

pub fn f77(v0: Adt6, v1: u64) -> R<u64> {
    let a2225 = 0u64;
    let a2226 = match v0.clone() {
        Adt6::C0(h2224, _, _, _) => {
            h2224
        }
    };
    if if if if if nat_lt(a2225, a2226) {
        let a2228 = 0u64;
        let a2229 = match v0.clone() {
            Adt6::C0(_, h2227, _, _) => {
                h2227
            }
        };
        nat_lt(a2228, a2229)
    } else {
        false
    } {
        let a2231 = 0u64;
        let a2232 = match v0.clone() {
            Adt6::C0(_, _, _, h2230) => {
                h2230
            }
        };
        nat_lt(a2231, a2232)
    } else {
        false
    } {
        let a2234 = match v0.clone() {
            Adt6::C0(_, h2233, _, _) => {
                h2233
            }
        };
        let a2235 = 2147483648u64;
        nat_lt(a2234, a2235)
    } else {
        false
    } {
        let a2237 = match v0.clone() {
            Adt6::C0(_, _, _, h2236) => {
                h2236
            }
        };
        let a2238 = 2147483648u64;
        nat_lt(a2237, a2238)
    } else {
        false
    } {
        let v2: u64 = {
            let a2242 = v1;
            let a2243 = {
                let a2240 = 2u64;
                let a2241 = f125(match v0.clone() {
                    Adt6::C0(_, _, h2239, _) => {
                        h2239
                    }
                });
                nat_mul(a2240, a2241)?
            };
            nat_add(a2242, a2243)?
        };
        let v3: u64 = {
            let a2255 = {
                let a2250 = 1u64;
                let a2251 = {
                    let a2248 = match v0.clone() {
                        Adt6::C0(_, _, _, h2244) => {
                            h2244
                        }
                    };
                    let a2249 = {
                        let a2246 = match v0.clone() {
                            Adt6::C0(h2245, _, _, _) => {
                                h2245
                            }
                        };
                        let a2247 = 1u64;
                        nat_sub(a2246, a2247)
                    };
                    nat_mul(a2248, a2249)?
                };
                nat_add(a2250, a2251)?
            };
            let a2256 = {
                let a2253 = 2u64;
                let a2254 = f124(match v0.clone() {
                    Adt6::C0(_, _, h2252, _) => {
                        h2252
                    }
                });
                nat_mul(a2253, a2254)?
            };
            nat_add(a2255, a2256)?
        };
        let a2257 = v3;
        let a2258 = v2;
        if nat_le(a2257, a2258) {
            let a2265 = 1u64;
            let a2266 = {
                let a2262 = {
                    let a2259 = v2;
                    let a2260 = v3;
                    nat_sub(a2259, a2260)
                };
                let a2263 = match v0 {
                    Adt6::C0(_, h2261, _, _) => {
                        h2261
                    }
                };
                let a2264 = 0u64;
                nat_quot(a2262, a2263, a2264)
            };
            nat_add(a2265, a2266)
        } else {
            let a2270 = {
                let a2267 = v3;
                let a2268 = v2;
                nat_sub(a2267, a2268)
            };
            let a2271 = match v0 {
                Adt6::C0(_, h2269, _, _) => {
                    h2269
                }
            };
            if nat_lt(a2270, a2271) {
                Ok(1u64)
            } else {
                Ok(0u64)
            }
        }
    } else {
        Ok(0u64)
    }
}

pub fn f78(v0: u64) -> u64 {
    let v1: u64 = {
        let a2272 = v0;
        let a2273 = 4294967296u64;
        let a2274 = 0u64;
        nat_rem(a2272, a2273, a2274)
    };
    let a2275 = v1;
    let a2276 = 2147483648u64;
    if nat_lt(a2275, a2276) {
        v1
    } else {
        0u64
    }
}

pub fn f79(v0: u64) -> R<u64> {
    let a2279 = 2u64;
    let a2280 = {
        let a2277 = 2u64;
        let a2278 = v0;
        nat_mul(a2277, a2278)?
    };
    nat_add(a2279, a2280)
}

pub fn f80(v0: u64, v1: Adt14, v2: Adt14) -> R<Adt15> {
    let a2282 = match v1.clone() {
        Adt14::C0(h2281, _, _, _, _, _, _, _, _, _, _) => {
            h2281
        }
    };
    let a2283 = 0u64;
    if if if if nat_eq(a2282, a2283) {
        let a2285 = match v2.clone() {
            Adt14::C0(h2284, _, _, _, _, _, _, _, _, _, _) => {
                h2284
            }
        };
        let a2286 = 0u64;
        nat_eq(a2285, a2286)
    } else {
        false
    } {
        f175(v1.clone(), v2.clone())
    } else {
        false
    } {
        if f176(match v1.clone() {
            Adt14::C0(_, h2287, _, _, _, _, _, _, _, _, _) => {
                h2287
            }
        }, match v1.clone() {
            Adt14::C0(_, _, h2288, _, _, _, _, _, _, _, _) => {
                h2288
            }
        }, match v1.clone() {
            Adt14::C0(_, _, _, h2289, _, _, _, _, _, _, _) => {
                h2289
            }
        }, match v1.clone() {
            Adt14::C0(_, _, _, _, h2290, _, _, _, _, _, _) => {
                h2290
            }
        }) {
            true
        } else {
            if if if if f35(v0) {
                f177(v1.clone(), v2.clone())
            } else {
                false
            } {
                f178(v1.clone())
            } else {
                false
            } {
                f178(v2.clone())
            } else {
                false
            } {
                if f179(v1.clone(), v2.clone())? {
                    true
                } else {
                    f180(v1.clone(), v2.clone())
                }
            } else {
                false
            }
        }
    } else {
        false
    } {
        let v3: u64 = if f176(match v1.clone() {
            Adt14::C0(_, h2291, _, _, _, _, _, _, _, _, _) => {
                h2291
            }
        }, match v1.clone() {
            Adt14::C0(_, _, h2292, _, _, _, _, _, _, _, _) => {
                h2292
            }
        }, match v1.clone() {
            Adt14::C0(_, _, _, h2293, _, _, _, _, _, _, _) => {
                h2293
            }
        }, match v1.clone() {
            Adt14::C0(_, _, _, _, h2294, _, _, _, _, _, _) => {
                h2294
            }
        }) {
            0u64
        } else {
            let a2300 = match v1.clone() {
                Adt14::C0(_, _, h2295, _, _, _, _, _, _, _, _) => {
                    h2295
                }
            };
            let a2301 = {
                let a2298 = match v1.clone() {
                    Adt14::C0(_, _, _, h2296, _, _, _, _, _, _, _) => {
                        h2296
                    }
                };
                let a2299 = match v1.clone() {
                    Adt14::C0(_, _, _, _, h2297, _, _, _, _, _, _) => {
                        h2297
                    }
                };
                nat_mul(a2298, a2299)?
            };
            nat_mul(a2300, a2301)?
        };
        let v4: u64 = {
            let a2302 = v3;
            let a2303 = 0u64;
            if if nat_eq(a2302, a2303) {
                true
            } else {
                f181(v2.clone())?
            } {
                0u64
            } else {
                let a2305 = match v2.clone() {
                    Adt14::C0(_, _, _, _, _, _, _, _, _, h2304, _) => {
                        h2304
                    }
                };
                let a2306 = 4u64;
                let a2307 = 0u64;
                nat_quot(a2305, a2306, a2307)
            }
        };
        Ok(Adt15::C1(f182(f183(match v1.clone() {
            Adt14::C0(_, _, _, _, _, h2308, _, _, _, _, _) => {
                h2308
            }
        }, match v2 {
            Adt14::C0(_, _, _, _, _, h2309, _, _, _, _, _) => {
                h2309
            }
        }), match v1 {
            Adt14::C0(_, h2310, _, _, _, _, _, _, _, _, _) => {
                h2310
            }
        }, v3, v4)?, 4u64, 64u64, v3, f184(v4)?, v0))
    } else {
        Ok(Adt15::C0)
    }
}

pub fn f81(v0: u64, v1: u64, v2: Adt14, v3: Adt14) -> R<Adt15> {
    let v4: Adt14 = f185(v3.clone());
    let a2312 = match v2.clone() {
        Adt14::C0(h2311, _, _, _, _, _, _, _, _, _, _) => {
            h2311
        }
    };
    let a2313 = 0u64;
    if if if if nat_eq(a2312, a2313) {
        let a2315 = match v3.clone() {
            Adt14::C0(h2314, _, _, _, _, _, _, _, _, _, _) => {
                h2314
            }
        };
        let a2316 = 0u64;
        nat_eq(a2315, a2316)
    } else {
        false
    } {
        f175(v2.clone(), v3)
    } else {
        false
    } {
        if f176(match v2.clone() {
            Adt14::C0(_, h2317, _, _, _, _, _, _, _, _, _) => {
                h2317
            }
        }, match v2.clone() {
            Adt14::C0(_, _, h2318, _, _, _, _, _, _, _, _) => {
                h2318
            }
        }, match v2.clone() {
            Adt14::C0(_, _, _, h2319, _, _, _, _, _, _, _) => {
                h2319
            }
        }, match v2.clone() {
            Adt14::C0(_, _, _, _, h2320, _, _, _, _, _, _) => {
                h2320
            }
        }) {
            true
        } else {
            let a2322 = match v2.clone() {
                Adt14::C0(_, _, _, _, _, h2321, _, _, _, _, _) => {
                    h2321
                }
            };
            let a2323 = 4u64;
            if if if if if nat_eq(a2322, a2323) {
                f186(match v2.clone() {
                    Adt14::C0(_, h2324, _, _, _, _, _, _, _, _, _) => {
                        h2324
                    }
                }, match v2.clone() {
                    Adt14::C0(_, _, h2325, _, _, _, _, _, _, _, _) => {
                        h2325
                    }
                }, match v2.clone() {
                    Adt14::C0(_, _, _, h2326, _, _, _, _, _, _, _) => {
                        h2326
                    }
                }, v0)?
            } else {
                false
            } {
                f178(v2.clone())
            } else {
                false
            } {
                f178(v4.clone())
            } else {
                false
            } {
                let a2328 = f187(v4.clone())?;
                let a2329 = match v4.clone() {
                    Adt14::C0(_, _, _, _, _, _, _, _, _, h2327, _) => {
                        h2327
                    }
                };
                nat_le(a2328, a2329)
            } else {
                false
            } {
                if f179(v2.clone(), v4.clone())? {
                    true
                } else {
                    f180(v2.clone(), v4.clone())
                }
            } else {
                false
            }
        }
    } else {
        false
    } {
        let v5: u64 = if if f176(match v2.clone() {
            Adt14::C0(_, h2330, _, _, _, _, _, _, _, _, _) => {
                h2330
            }
        }, match v2.clone() {
            Adt14::C0(_, _, h2331, _, _, _, _, _, _, _, _) => {
                h2331
            }
        }, match v2.clone() {
            Adt14::C0(_, _, _, h2332, _, _, _, _, _, _, _) => {
                h2332
            }
        }, match v2.clone() {
            Adt14::C0(_, _, _, _, h2333, _, _, _, _, _, _) => {
                h2333
            }
        }) {
            true
        } else {
            f181(v4.clone())?
        } {
            0u64
        } else {
            let a2335 = match v4 {
                Adt14::C0(_, _, _, _, _, _, _, _, _, h2334, _) => {
                    h2334
                }
            };
            let a2336 = 4u64;
            let a2337 = 0u64;
            nat_quot(a2335, a2336, a2337)
        };
        Ok(Adt15::C1(f188(match v2.clone() {
            Adt14::C0(_, h2338, _, _, _, _, _, _, _, _, _) => {
                h2338
            }
        }, match v2.clone() {
            Adt14::C0(_, _, h2339, _, _, _, _, _, _, _, _) => {
                h2339
            }
        }, match v2.clone() {
            Adt14::C0(_, _, _, h2340, _, _, _, _, _, _, _) => {
                h2340
            }
        }, match v2 {
            Adt14::C0(_, _, _, _, h2341, _, _, _, _, _, _) => {
                h2341
            }
        }, v0, v5)?, 4u64, 65u64, v0, f184(v5)?, v1))
    } else {
        Ok(Adt15::C0)
    }
}

pub fn f82(v0: u64, v1: bool, v2: Adt14, v3: Adt14, v4: Adt14) -> R<Adt15> {
    let v5: Adt14 = f185(v4.clone());
    if if if if if if if v1 {
        let a2343 = match v2.clone() {
            Adt14::C0(h2342, _, _, _, _, _, _, _, _, _, _) => {
                h2342
            }
        };
        let a2344 = 0u64;
        nat_eq(a2343, a2344)
    } else {
        false
    } {
        let a2346 = match v3.clone() {
            Adt14::C0(h2345, _, _, _, _, _, _, _, _, _, _) => {
                h2345
            }
        };
        let a2347 = 0u64;
        nat_eq(a2346, a2347)
    } else {
        false
    } {
        let a2349 = match v4.clone() {
            Adt14::C0(h2348, _, _, _, _, _, _, _, _, _, _) => {
                h2348
            }
        };
        let a2350 = 0u64;
        nat_eq(a2349, a2350)
    } else {
        false
    } {
        f175(v2.clone(), v3.clone())
    } else {
        false
    } {
        f175(v2.clone(), v4)
    } else {
        false
    } {
        if f176(match v2.clone() {
            Adt14::C0(_, h2351, _, _, _, _, _, _, _, _, _) => {
                h2351
            }
        }, match v2.clone() {
            Adt14::C0(_, _, h2352, _, _, _, _, _, _, _, _) => {
                h2352
            }
        }, match v2.clone() {
            Adt14::C0(_, _, _, h2353, _, _, _, _, _, _, _) => {
                h2353
            }
        }, match v2.clone() {
            Adt14::C0(_, _, _, _, h2354, _, _, _, _, _, _) => {
                h2354
            }
        }) {
            true
        } else {
            let a2356 = match v2.clone() {
                Adt14::C0(_, _, _, _, _, h2355, _, _, _, _, _) => {
                    h2355
                }
            };
            let a2357 = 4u64;
            if if if if if if if nat_eq(a2356, a2357) {
                let a2359 = match v3.clone() {
                    Adt14::C0(_, _, _, _, _, h2358, _, _, _, _, _) => {
                        h2358
                    }
                };
                let a2360 = 4u64;
                nat_eq(a2359, a2360)
            } else {
                false
            } {
                f178(v2.clone())
            } else {
                false
            } {
                f178(v3.clone())
            } else {
                false
            } {
                f178(v5.clone())
            } else {
                false
            } {
                let a2362 = f187(v5.clone())?;
                let a2363 = match v5.clone() {
                    Adt14::C0(_, _, _, _, _, _, _, _, _, h2361, _) => {
                        h2361
                    }
                };
                nat_le(a2362, a2363)
            } else {
                false
            } {
                if f179(v2.clone(), v5.clone())? {
                    true
                } else {
                    f180(v2.clone(), v5.clone())
                }
            } else {
                false
            } {
                if f179(v3.clone(), v5.clone())? {
                    true
                } else {
                    f180(v3, v5.clone())
                }
            } else {
                false
            }
        }
    } else {
        false
    } {
        let v6: u64 = if f176(match v2.clone() {
            Adt14::C0(_, h2364, _, _, _, _, _, _, _, _, _) => {
                h2364
            }
        }, match v2.clone() {
            Adt14::C0(_, _, h2365, _, _, _, _, _, _, _, _) => {
                h2365
            }
        }, match v2.clone() {
            Adt14::C0(_, _, _, h2366, _, _, _, _, _, _, _) => {
                h2366
            }
        }, match v2.clone() {
            Adt14::C0(_, _, _, _, h2367, _, _, _, _, _, _) => {
                h2367
            }
        }) {
            0u64
        } else {
            let a2373 = match v2.clone() {
                Adt14::C0(_, _, h2368, _, _, _, _, _, _, _, _) => {
                    h2368
                }
            };
            let a2374 = {
                let a2371 = match v2.clone() {
                    Adt14::C0(_, _, _, h2369, _, _, _, _, _, _, _) => {
                        h2369
                    }
                };
                let a2372 = match v2.clone() {
                    Adt14::C0(_, _, _, _, h2370, _, _, _, _, _, _) => {
                        h2370
                    }
                };
                nat_mul(a2371, a2372)?
            };
            nat_mul(a2373, a2374)?
        };
        let v7: u64 = {
            let a2375 = v6;
            let a2376 = 0u64;
            if if nat_eq(a2375, a2376) {
                true
            } else {
                f181(v5.clone())?
            } {
                0u64
            } else {
                let a2378 = match v5 {
                    Adt14::C0(_, _, _, _, _, _, _, _, _, h2377, _) => {
                        h2377
                    }
                };
                let a2379 = 4u64;
                let a2380 = 0u64;
                nat_quot(a2378, a2379, a2380)
            }
        };
        Ok(Adt15::C1(f189(match v2 {
            Adt14::C0(_, h2381, _, _, _, _, _, _, _, _, _) => {
                h2381
            }
        }, v6, v7)?, 4u64, 66u64, v6, f184(v7)?, v0))
    } else {
        Ok(Adt15::C0)
    }
}

pub fn f83(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64) -> bool {
    if if f190(v0, v3, v6) {
        f190(v1, v4, v6)
    } else {
        false
    } {
        f190(v2, v5, v6)
    } else {
        false
    }
}

pub fn f84(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64) -> R<bool> {
    if if if if if f191(v0, v1, v4, v2, v5, v3, v6)? {
        true
    } else {
        f191(v0, v1, v4, v3, v6, v2, v5)?
    } {
        true
    } else {
        f191(v0, v2, v5, v1, v4, v3, v6)?
    } {
        true
    } else {
        f191(v0, v2, v5, v3, v6, v1, v4)?
    } {
        true
    } else {
        f191(v0, v3, v6, v1, v4, v2, v5)?
    } {
        Ok(true)
    } else {
        f191(v0, v3, v6, v2, v5, v1, v4)
    }
}

pub fn f85(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let a2382 = v2;
    let a2383 = 0u64;
    if nat_eq(a2382, a2383) {
        Ok(0u64)
    } else {
        let a2386 = 2u64;
        let a2387 = {
            let a2384 = v0;
            let a2385 = v1;
            nat_mul(a2384, a2385)?
        };
        nat_mul(a2386, a2387)
    }
}

pub fn f86(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let a2388 = v2;
    let a2389 = f171(v0, v1)?;
    nat_mul(a2388, a2389)
}

pub fn f87(v0: u64) -> u64 {
    let a2390 = v0;
    let a2391 = 30u64;
    if nat_eq(a2390, a2391) {
        3u64
    } else {
        1u64
    }
}

pub fn f88(v0: u64) -> R<u64> {
    let a2394 = 5u64;
    let a2395 = {
        let a2392 = 4u64;
        let a2393 = v0;
        nat_mul(a2392, a2393)?
    };
    nat_add(a2394, a2395)
}

pub fn f89(v0: u64) -> bool {
    let m2396 = f192(v0);
    match m2396 {
        Adt18::C0 => {
            false
        }
        Adt18::C1(v1) => {
            let a2398 = 1u64;
            let a2399 = match v1 {
                Adt19::C0(_, h2397, _) => {
                    h2397
                }
            };
            nat_lt(a2398, a2399)
        }
    }
}

pub fn f90(v0: Adt1, v1: Adt2, v2: Adt2, v3: Adt2) -> R<Adt10> {
    let v4: bool = {
        let a2401 = 0u64;
        let a2402 = match v0.clone() {
            Adt1::C0(_, h2400, _, _, _, _, _, _, _, _, _, _) => {
                h2400
            }
        };
        nat_lt(a2401, a2402)
    };
    let a2404 = match v1.clone() {
        Adt2::C0(h2403, _, _, _, _, _, _, _, _, _, _) => {
            h2403
        }
    };
    let a2405 = 0u64;
    if if if if if if if if if if if if if if if if nat_eq(a2404, a2405) {
        let a2407 = match v2.clone() {
            Adt2::C0(h2406, _, _, _, _, _, _, _, _, _, _) => {
                h2406
            }
        };
        let a2408 = 0u64;
        nat_eq(a2407, a2408)
    } else {
        false
    } {
        let a2410 = match v3.clone() {
            Adt2::C0(h2409, _, _, _, _, _, _, _, _, _, _) => {
                h2409
            }
        };
        let a2411 = 0u64;
        nat_eq(a2410, a2411)
    } else {
        false
    } {
        let a2413 = match v1.clone() {
            Adt2::C0(_, _, _, _, h2412, _, _, _, _, _, _) => {
                h2412
            }
        };
        let a2414 = 1u64;
        nat_eq(a2413, a2414)
    } else {
        false
    } {
        let a2416 = match v2.clone() {
            Adt2::C0(_, _, _, _, h2415, _, _, _, _, _, _) => {
                h2415
            }
        };
        let a2417 = 1u64;
        nat_eq(a2416, a2417)
    } else {
        false
    } {
        let a2419 = match v3.clone() {
            Adt2::C0(_, _, _, _, h2418, _, _, _, _, _, _) => {
                h2418
            }
        };
        let a2420 = 1u64;
        nat_eq(a2419, a2420)
    } else {
        false
    } {
        let a2422 = 0u64;
        let a2423 = match v1.clone() {
            Adt2::C0(_, _, h2421, _, _, _, _, _, _, _, _) => {
                h2421
            }
        };
        nat_lt(a2422, a2423)
    } else {
        false
    } {
        let a2425 = match v3.clone() {
            Adt2::C0(_, _, _, h2424, _, _, _, _, _, _, _) => {
                h2424
            }
        };
        let a2426 = 1u64;
        nat_eq(a2425, a2426)
    } else {
        false
    } {
        let a2429 = match v3.clone() {
            Adt2::C0(_, h2427, _, _, _, _, _, _, _, _, _) => {
                h2427
            }
        };
        let a2430 = match v1.clone() {
            Adt2::C0(_, h2428, _, _, _, _, _, _, _, _, _) => {
                h2428
            }
        };
        nat_eq(a2429, a2430)
    } else {
        false
    } {
        let a2433 = match v3.clone() {
            Adt2::C0(_, _, h2431, _, _, _, _, _, _, _, _) => {
                h2431
            }
        };
        let a2434 = match v1.clone() {
            Adt2::C0(_, _, _, h2432, _, _, _, _, _, _, _) => {
                h2432
            }
        };
        nat_eq(a2433, a2434)
    } else {
        false
    } {
        if v4 {
            f20(v1.clone(), v2.clone())
        } else {
            let a2437 = match v2.clone() {
                Adt2::C0(_, h2435, _, _, _, _, _, _, _, _, _) => {
                    h2435
                }
            };
            let a2438 = match v1.clone() {
                Adt2::C0(_, _, h2436, _, _, _, _, _, _, _, _) => {
                    h2436
                }
            };
            if if nat_eq(a2437, a2438) {
                let a2441 = match v2.clone() {
                    Adt2::C0(_, _, h2439, _, _, _, _, _, _, _, _) => {
                        h2439
                    }
                };
                let a2442 = match v1.clone() {
                    Adt2::C0(_, _, _, h2440, _, _, _, _, _, _, _) => {
                        h2440
                    }
                };
                nat_eq(a2441, a2442)
            } else {
                false
            } {
                let a2444 = match v2.clone() {
                    Adt2::C0(_, _, _, h2443, _, _, _, _, _, _, _) => {
                        h2443
                    }
                };
                let a2445 = 1u64;
                nat_eq(a2444, a2445)
            } else {
                false
            }
        }
    } else {
        false
    } {
        f17(v1.clone(), f18(v1.clone(), 1u64)?)?
    } else {
        false
    } {
        f17(v2.clone(), f18(v2.clone(), 1u64)?)?
    } else {
        false
    } {
        f21(v3.clone())?
    } else {
        false
    } {
        f19(v1.clone(), v3.clone())?
    } else {
        false
    } {
        f19(v2.clone(), v3.clone())?
    } else {
        false
    } {
        Ok(Adt10::C4({
            let a2447 = f18(v3.clone(), 1u64)?;
            let a2448 = f150(match v1.clone() {
                Adt2::C0(_, _, h2446, _, _, _, _, _, _, _, _) => {
                    h2446
                }
            }, v4)?;
            nat_mul(a2447, a2448)?
        }, 4u64, f22(v1, 1u64), f22(v3, 1u64), Adt4::C36(f22(v2, 1u64), v4, match v0 {
            Adt1::C0(h2449, _, _, _, _, _, _, _, _, _, _, _) => {
                h2449
            }
        })))
    } else {
        Ok(Adt10::C1)
    }
}

pub fn f91(v0: Adt1, v1: Adt2, v2: Adt2, v3: Adt2) -> R<Adt10> {
    let v4: Adt6 = Adt6::C0(match v1.clone() {
        Adt2::C0(_, h2450, _, _, _, _, _, _, _, _, _) => {
            h2450
        }
    }, match v0.clone() {
        Adt1::C0(h2451, _, _, _, _, _, _, _, _, _, _, _) => {
            h2451
        }
    }, match v0.clone() {
        Adt1::C0(_, _, h2452, _, _, _, _, _, _, _, _, _) => {
            h2452
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, _, h2453, _, _, _, _, _, _, _) => {
            h2453
        }
    });
    let v5: Adt6 = Adt6::C0(match v1.clone() {
        Adt2::C0(_, _, h2454, _, _, _, _, _, _, _, _) => {
            h2454
        }
    }, match v0.clone() {
        Adt1::C0(_, h2455, _, _, _, _, _, _, _, _, _, _) => {
            h2455
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, h2456, _, _, _, _, _, _, _, _) => {
            h2456
        }
    }, match v0 {
        Adt1::C0(_, _, _, _, _, h2457, _, _, _, _, _, _) => {
            h2457
        }
    });
    let v6: u64 = match v1.clone() {
        Adt2::C0(h2458, _, _, _, _, _, _, _, _, _, _) => {
            h2458
        }
    };
    let v7: bool = {
        let a2461 = match v2.clone() {
            Adt2::C0(_, _, _, _, _, _, _, h2459, _, _, _) => {
                h2459
            }
        };
        let a2462 = match v2.clone() {
            Adt2::C0(_, _, _, _, _, h2460, _, _, _, _, _) => {
                h2460
            }
        };
        nat_lt(a2461, a2462)
    };
    let a2463 = v6;
    let a2464 = 0u64;
    if if if if if if if if if if if if if if if nat_eq(a2463, a2464) {
        true
    } else {
        let a2465 = v6;
        let a2466 = 1u64;
        nat_eq(a2465, a2466)
    } {
        let a2468 = match v2.clone() {
            Adt2::C0(h2467, _, _, _, _, _, _, _, _, _, _) => {
                h2467
            }
        };
        let a2469 = 0u64;
        nat_eq(a2468, a2469)
    } else {
        false
    } {
        let a2471 = match v3.clone() {
            Adt2::C0(h2470, _, _, _, _, _, _, _, _, _, _) => {
                h2470
            }
        };
        let a2472 = 0u64;
        nat_eq(a2471, a2472)
    } else {
        false
    } {
        let a2474 = match v1.clone() {
            Adt2::C0(_, _, _, h2473, _, _, _, _, _, _, _) => {
                h2473
            }
        };
        let a2475 = 1u64;
        nat_eq(a2474, a2475)
    } else {
        false
    } {
        let a2478 = match v1.clone() {
            Adt2::C0(_, _, _, _, h2476, _, _, _, _, _, _) => {
                h2476
            }
        };
        let a2479 = match v2.clone() {
            Adt2::C0(_, _, _, h2477, _, _, _, _, _, _, _) => {
                h2477
            }
        };
        nat_eq(a2478, a2479)
    } else {
        false
    } {
        let a2482 = match v3.clone() {
            Adt2::C0(_, _, _, h2480, _, _, _, _, _, _, _) => {
                h2480
            }
        };
        let a2483 = match v2.clone() {
            Adt2::C0(_, _, _, h2481, _, _, _, _, _, _, _) => {
                h2481
            }
        };
        nat_eq(a2482, a2483)
    } else {
        false
    } {
        let a2486 = match v3.clone() {
            Adt2::C0(_, _, _, _, h2484, _, _, _, _, _, _) => {
                h2484
            }
        };
        let a2487 = match v2.clone() {
            Adt2::C0(_, _, _, _, h2485, _, _, _, _, _, _) => {
                h2485
            }
        };
        nat_eq(a2486, a2487)
    } else {
        false
    } {
        let a2489 = 0u64;
        let a2490 = match v4.clone() {
            Adt6::C0(h2488, _, _, _) => {
                h2488
            }
        };
        if if if if if nat_lt(a2489, a2490) {
            let a2492 = 0u64;
            let a2493 = match v4.clone() {
                Adt6::C0(_, h2491, _, _) => {
                    h2491
                }
            };
            nat_lt(a2492, a2493)
        } else {
            false
        } {
            let a2495 = match v4.clone() {
                Adt6::C0(_, h2494, _, _) => {
                    h2494
                }
            };
            let a2496 = 2147483648u64;
            nat_lt(a2495, a2496)
        } else {
            false
        } {
            let a2498 = 0u64;
            let a2499 = match v4.clone() {
                Adt6::C0(_, _, _, h2497) => {
                    h2497
                }
            };
            nat_lt(a2498, a2499)
        } else {
            false
        } {
            let a2501 = match v4.clone() {
                Adt6::C0(_, _, _, h2500) => {
                    h2500
                }
            };
            let a2502 = 2147483648u64;
            nat_lt(a2501, a2502)
        } else {
            false
        } {
            let a2505 = match v3.clone() {
                Adt2::C0(_, h2503, _, _, _, _, _, _, _, _, _) => {
                    h2503
                }
            };
            let a2506 = f77(v4.clone(), match v2.clone() {
                Adt2::C0(_, h2504, _, _, _, _, _, _, _, _, _) => {
                    h2504
                }
            })?;
            nat_eq(a2505, a2506)
        } else {
            false
        }
    } else {
        false
    } {
        let a2508 = 0u64;
        let a2509 = match v5.clone() {
            Adt6::C0(h2507, _, _, _) => {
                h2507
            }
        };
        if if if if if nat_lt(a2508, a2509) {
            let a2511 = 0u64;
            let a2512 = match v5.clone() {
                Adt6::C0(_, h2510, _, _) => {
                    h2510
                }
            };
            nat_lt(a2511, a2512)
        } else {
            false
        } {
            let a2514 = match v5.clone() {
                Adt6::C0(_, h2513, _, _) => {
                    h2513
                }
            };
            let a2515 = 2147483648u64;
            nat_lt(a2514, a2515)
        } else {
            false
        } {
            let a2517 = 0u64;
            let a2518 = match v5.clone() {
                Adt6::C0(_, _, _, h2516) => {
                    h2516
                }
            };
            nat_lt(a2517, a2518)
        } else {
            false
        } {
            let a2520 = match v5.clone() {
                Adt6::C0(_, _, _, h2519) => {
                    h2519
                }
            };
            let a2521 = 2147483648u64;
            nat_lt(a2520, a2521)
        } else {
            false
        } {
            let a2524 = match v3.clone() {
                Adt2::C0(_, _, h2522, _, _, _, _, _, _, _, _) => {
                    h2522
                }
            };
            let a2525 = f77(v5.clone(), match v2.clone() {
                Adt2::C0(_, _, h2523, _, _, _, _, _, _, _, _) => {
                    h2523
                }
            })?;
            nat_eq(a2524, a2525)
        } else {
            false
        }
    } else {
        false
    } {
        f193(v2.clone(), 4u64, v7, false)?
    } else {
        false
    } {
        f193(v3.clone(), 4u64, v7, false)?
    } else {
        false
    } {
        f193(v1.clone(), {
            let a2526 = v6;
            let a2527 = 0u64;
            if nat_eq(a2526, a2527) {
                4u64
            } else {
                2u64
            }
        }, v7, true)?
    } else {
        false
    } {
        f19(v1.clone(), v3.clone())?
    } else {
        false
    } {
        f19(v2.clone(), v3.clone())?
    } else {
        false
    } {
        Ok(Adt10::C4({
            let a2536 = {
                let a2534 = {
                    let a2532 = 3u64;
                    let a2533 = {
                        let a2530 = match v4.clone() {
                            Adt6::C0(h2528, _, _, _) => {
                                h2528
                            }
                        };
                        let a2531 = match v5.clone() {
                            Adt6::C0(h2529, _, _, _) => {
                                h2529
                            }
                        };
                        nat_mul(a2530, a2531)?
                    };
                    nat_mul(a2532, a2533)?
                };
                let a2535 = 2u64;
                nat_add(a2534, a2535)?
            };
            let a2537 = f18(v3.clone(), 1u64)?;
            nat_mul(a2536, a2537)?
        }, 4u64, f22(v1, 1u64), f22(v3, 1u64), Adt4::C34(f22(v2, 1u64), v4, v5, v6)))
    } else {
        Ok(Adt10::C1)
    }
}

pub fn f92(v0: Adt2, v1: Adt2, v2: Adt2) -> R<Adt10> {
    if if if if if if if f20(v0.clone(), v1.clone()) {
        f20(v0.clone(), v2.clone())
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
        if f19(v0.clone(), v2.clone())? {
            true
        } else {
            f37(v0.clone(), v2.clone())
        }
    } else {
        false
    } {
        if f19(v1.clone(), v2.clone())? {
            true
        } else {
            f37(v1.clone(), v2.clone())
        }
    } else {
        false
    } {
        Ok(Adt10::C4({
            let a2538 = 6u64;
            let a2539 = f18(v0.clone(), 1u64)?;
            nat_mul(a2538, a2539)?
        }, 2u64, f22(v0, 1u64), f22(v2, 1u64), Adt4::C33(f22(v1, 1u64))))
    } else {
        Ok(Adt10::C1)
    }
}

pub fn f93(v0: Adt1, v1: Adt2, v2: Option<Adt2>, v3: Adt2) -> R<Adt10> {
    let m2540 = v2;
    match m2540 {
        None => {
            Ok(Adt10::C1)
        }
        Some(v4) => {
            let v5: u64 = match v1.clone() {
                Adt2::C0(_, h2541, _, _, _, _, _, _, _, _, _) => {
                    h2541
                }
            };
            let v6: u64 = {
                let a2547 = match v1.clone() {
                    Adt2::C0(_, _, h2542, _, _, _, _, _, _, _, _) => {
                        h2542
                    }
                };
                let a2548 = {
                    let a2545 = match v1.clone() {
                        Adt2::C0(_, _, _, h2543, _, _, _, _, _, _, _) => {
                            h2543
                        }
                    };
                    let a2546 = match v1.clone() {
                        Adt2::C0(_, _, _, _, h2544, _, _, _, _, _, _) => {
                            h2544
                        }
                    };
                    nat_mul(a2545, a2546)?
                };
                nat_mul(a2547, a2548)?
            };
            if if if if if if if if if if f20(v1.clone(), v4.clone()) {
                f20(v1.clone(), v3.clone())
            } else {
                false
            } {
                f194(v1.clone(), v6)?
            } else {
                false
            } {
                f194(v4.clone(), v6)?
            } else {
                false
            } {
                f194(v3.clone(), v6)?
            } else {
                false
            } {
                if f19(v1.clone(), v3.clone())? {
                    true
                } else {
                    f37(v1.clone(), v3.clone())
                }
            } else {
                false
            } {
                if f19(v4.clone(), v3.clone())? {
                    true
                } else {
                    f37(v4, v3.clone())
                }
            } else {
                false
            } {
                let a2549 = v5;
                let a2550 = 2147483647u64;
                nat_le(a2549, a2550)
            } else {
                false
            } {
                let a2551 = v6;
                let a2552 = 2147483647u64;
                nat_le(a2551, a2552)
            } else {
                false
            } {
                let a2554 = match v0.clone() {
                    Adt1::C0(h2553, _, _, _, _, _, _, _, _, _, _, _) => {
                        h2553
                    }
                };
                let a2555 = 4294967295u64;
                if nat_le(a2554, a2555) {
                    let a2557 = match v0.clone() {
                        Adt1::C0(_, h2556, _, _, _, _, _, _, _, _, _, _) => {
                            h2556
                        }
                    };
                    let a2558 = 4294967295u64;
                    nat_le(a2557, a2558)
                } else {
                    false
                }
            } else {
                false
            } {
                Ok(Adt10::C4({
                    let a2559 = v5;
                    let a2560 = 0u64;
                    if nat_eq(a2559, a2560) {
                        0u64
                    } else {
                        let a2561 = v6;
                        let a2562 = f149(v5)?;
                        nat_mul(a2561, a2562)?
                    }
                }, 4u64, f22(v1, 1u64), f22(v3, 1u64), Adt4::C32(match v0 {
                    Adt1::C0(h2563, _, _, _, _, _, _, _, _, _, _, _) => {
                        h2563
                    }
                })))
            } else {
                Ok(Adt10::C1)
            }
        }
    }
}

pub fn f94(v0: Adt1, v1: Adt2, v2: Adt2) -> R<Adt10> {
    let v3: u64 = match v0.clone() {
        Adt1::C0(h2564, _, _, _, _, _, _, _, _, _, _, _) => {
            h2564
        }
    };
    let v4: u64 = match v0.clone() {
        Adt1::C0(_, h2565, _, _, _, _, _, _, _, _, _, _) => {
            h2565
        }
    };
    let v5: u64 = match v0 {
        Adt1::C0(_, _, h2566, _, _, _, _, _, _, _, _, _) => {
            h2566
        }
    };
    let v6: u64 = {
        let a2568 = match v1.clone() {
            Adt2::C0(_, h2567, _, _, _, _, _, _, _, _, _) => {
                h2567
            }
        };
        let a2569 = v4;
        let a2570 = 0u64;
        nat_quot(a2568, a2569, a2570)
    };
    let v7: u64 = {
        let a2572 = match v1.clone() {
            Adt2::C0(h2571, _, _, _, _, _, _, _, _, _, _) => {
                h2571
            }
        };
        let a2573 = 0u64;
        if nat_eq(a2572, a2573) {
            4u64
        } else {
            2u64
        }
    };
    if if if if if if if if if if if if if if if f154(match v1.clone() {
        Adt2::C0(h2574, _, _, _, _, _, _, _, _, _, _) => {
            h2574
        }
    }) {
        let a2577 = match v1.clone() {
            Adt2::C0(h2575, _, _, _, _, _, _, _, _, _, _) => {
                h2575
            }
        };
        let a2578 = match v2.clone() {
            Adt2::C0(h2576, _, _, _, _, _, _, _, _, _, _) => {
                h2576
            }
        };
        nat_eq(a2577, a2578)
    } else {
        false
    } {
        let a2579 = 0u64;
        let a2580 = v3;
        nat_lt(a2579, a2580)
    } else {
        false
    } {
        let a2581 = v3;
        let a2582 = 2147483648u64;
        nat_lt(a2581, a2582)
    } else {
        false
    } {
        let a2583 = 0u64;
        let a2584 = v4;
        nat_lt(a2583, a2584)
    } else {
        false
    } {
        let a2585 = v4;
        let a2586 = 2147483648u64;
        nat_lt(a2585, a2586)
    } else {
        false
    } {
        let a2587 = v5;
        let a2588 = 2147483648u64;
        nat_lt(a2587, a2588)
    } else {
        false
    } {
        let a2589 = 0u64;
        let a2590 = v6;
        nat_lt(a2589, a2590)
    } else {
        false
    } {
        let a2592 = 0u64;
        let a2593 = match v2.clone() {
            Adt2::C0(_, h2591, _, _, _, _, _, _, _, _, _) => {
                h2591
            }
        };
        nat_lt(a2592, a2593)
    } else {
        false
    } {
        let a2597 = match v1.clone() {
            Adt2::C0(_, h2594, _, _, _, _, _, _, _, _, _) => {
                h2594
            }
        };
        let a2598 = {
            let a2595 = v6;
            let a2596 = v4;
            nat_mul(a2595, a2596)?
        };
        nat_eq(a2597, a2598)
    } else {
        false
    } {
        let a2600 = match v2.clone() {
            Adt2::C0(_, _, h2599, _, _, _, _, _, _, _, _) => {
                h2599
            }
        };
        let a2601 = v4;
        nat_eq(a2600, a2601)
    } else {
        false
    } {
        let a2614 = {
            let a2607 = {
                let a2605 = match v2.clone() {
                    Adt2::C0(_, h2602, _, _, _, _, _, _, _, _, _) => {
                        h2602
                    }
                };
                let a2606 = {
                    let a2603 = v5;
                    let a2604 = 2u64;
                    nat_mul(a2603, a2604)?
                };
                nat_add(a2605, a2606)?
            };
            let a2608 = v3;
            nat_add(a2607, a2608)?
        };
        let a2615 = {
            let a2612 = {
                let a2610 = match v1.clone() {
                    Adt2::C0(_, _, h2609, _, _, _, _, _, _, _, _) => {
                        h2609
                    }
                };
                let a2611 = v3;
                nat_mul(a2610, a2611)?
            };
            let a2613 = v6;
            nat_add(a2612, a2613)?
        };
        nat_eq(a2614, a2615)
    } else {
        false
    } {
        f195(v1.clone(), v7)?
    } else {
        false
    } {
        f195(v2.clone(), v7)?
    } else {
        false
    } {
        f19(v1.clone(), v2.clone())?
    } else {
        false
    } {
        let v8: Adt6 = Adt6::C0(v6, v3, v5, 1u64);
        Ok(Adt10::C4({
            let a2616 = f18(v2.clone(), 1u64)?;
            let a2617 = f168(v8.clone())?;
            nat_mul(a2616, a2617)?
        }, v7, f22(v1.clone(), 1u64), f22(v2, 1u64), Adt4::C30(v8, match v1 {
            Adt2::C0(h2618, _, _, _, _, _, _, _, _, _, _) => {
                h2618
            }
        })))
    } else {
        Ok(Adt10::C1)
    }
}

pub fn f95(v0: Adt2, v1: Option<Adt2>, v2: Adt2) -> R<Adt10> {
    let m2619 = v1;
    match m2619 {
        None => {
            Ok(Adt10::C1)
        }
        Some(v3) => {
            let a2621 = match v0.clone() {
                Adt2::C0(h2620, _, _, _, _, _, _, _, _, _, _) => {
                    h2620
                }
            };
            let a2622 = 0u64;
            if if if if if if if if if if if if if if if if if if if nat_eq(a2621, a2622) {
                let a2624 = match v3.clone() {
                    Adt2::C0(h2623, _, _, _, _, _, _, _, _, _, _) => {
                        h2623
                    }
                };
                let a2625 = 0u64;
                nat_eq(a2624, a2625)
            } else {
                false
            } {
                let a2627 = match v2.clone() {
                    Adt2::C0(h2626, _, _, _, _, _, _, _, _, _, _) => {
                        h2626
                    }
                };
                let a2628 = 0u64;
                nat_eq(a2627, a2628)
            } else {
                false
            } {
                let a2630 = match v0.clone() {
                    Adt2::C0(_, _, _, _, h2629, _, _, _, _, _, _) => {
                        h2629
                    }
                };
                let a2631 = 1u64;
                nat_eq(a2630, a2631)
            } else {
                false
            } {
                let a2633 = match v3.clone() {
                    Adt2::C0(_, _, _, h2632, _, _, _, _, _, _, _) => {
                        h2632
                    }
                };
                let a2634 = 1u64;
                nat_eq(a2633, a2634)
            } else {
                false
            } {
                let a2636 = match v3.clone() {
                    Adt2::C0(_, _, _, _, h2635, _, _, _, _, _, _) => {
                        h2635
                    }
                };
                let a2637 = 1u64;
                nat_eq(a2636, a2637)
            } else {
                false
            } {
                let a2640 = match v0.clone() {
                    Adt2::C0(_, _, h2638, _, _, _, _, _, _, _, _) => {
                        h2638
                    }
                };
                let a2641 = match v3.clone() {
                    Adt2::C0(_, _, h2639, _, _, _, _, _, _, _, _) => {
                        h2639
                    }
                };
                nat_eq(a2640, a2641)
            } else {
                false
            } {
                let a2646 = match v3.clone() {
                    Adt2::C0(_, h2642, _, _, _, _, _, _, _, _, _) => {
                        h2642
                    }
                };
                let a2647 = {
                    let a2644 = match v0.clone() {
                        Adt2::C0(_, h2643, _, _, _, _, _, _, _, _, _) => {
                            h2643
                        }
                    };
                    let a2645 = 1u64;
                    nat_add(a2644, a2645)?
                };
                nat_le(a2646, a2647)
            } else {
                false
            } {
                let a2650 = match v2.clone() {
                    Adt2::C0(_, h2648, _, _, _, _, _, _, _, _, _) => {
                        h2648
                    }
                };
                let a2651 = match v0.clone() {
                    Adt2::C0(_, _, h2649, _, _, _, _, _, _, _, _) => {
                        h2649
                    }
                };
                nat_eq(a2650, a2651)
            } else {
                false
            } {
                let a2659 = match v2.clone() {
                    Adt2::C0(_, _, h2652, _, _, _, _, _, _, _, _) => {
                        h2652
                    }
                };
                let a2660 = {
                    let a2657 = {
                        let a2654 = match v0.clone() {
                            Adt2::C0(_, h2653, _, _, _, _, _, _, _, _, _) => {
                                h2653
                            }
                        };
                        let a2655 = 1u64;
                        nat_add(a2654, a2655)?
                    };
                    let a2658 = match v3.clone() {
                        Adt2::C0(_, h2656, _, _, _, _, _, _, _, _, _) => {
                            h2656
                        }
                    };
                    nat_sub(a2657, a2658)
                };
                nat_eq(a2659, a2660)
            } else {
                false
            } {
                let a2663 = match v2.clone() {
                    Adt2::C0(_, _, _, h2661, _, _, _, _, _, _, _) => {
                        h2661
                    }
                };
                let a2664 = match v0.clone() {
                    Adt2::C0(_, _, _, h2662, _, _, _, _, _, _, _) => {
                        h2662
                    }
                };
                nat_eq(a2663, a2664)
            } else {
                false
            } {
                let a2666 = match v2.clone() {
                    Adt2::C0(_, _, _, _, h2665, _, _, _, _, _, _) => {
                        h2665
                    }
                };
                let a2667 = 1u64;
                nat_eq(a2666, a2667)
            } else {
                false
            } {
                f28(v2.clone())?
            } else {
                false
            } {
                let a2669 = match v0.clone() {
                    Adt2::C0(_, _, _, _, _, h2668, _, _, _, _, _) => {
                        h2668
                    }
                };
                let a2670 = 4u64;
                nat_eq(a2669, a2670)
            } else {
                false
            } {
                let a2672 = match v3.clone() {
                    Adt2::C0(_, _, _, _, _, h2671, _, _, _, _, _) => {
                        h2671
                    }
                };
                let a2673 = 4u64;
                nat_eq(a2672, a2673)
            } else {
                false
            } {
                let a2678 = match v0.clone() {
                    Adt2::C0(_, _, _, _, _, _, h2674, _, _, _, _) => {
                        h2674
                    }
                };
                let a2679 = {
                    let a2676 = match v0.clone() {
                        Adt2::C0(_, h2675, _, _, _, _, _, _, _, _, _) => {
                            h2675
                        }
                    };
                    let a2677 = 4u64;
                    nat_mul(a2676, a2677)?
                };
                nat_eq(a2678, a2679)
            } else {
                false
            } {
                let a2684 = match v3.clone() {
                    Adt2::C0(_, _, _, _, _, _, h2680, _, _, _, _) => {
                        h2680
                    }
                };
                let a2685 = {
                    let a2682 = match v3.clone() {
                        Adt2::C0(_, h2681, _, _, _, _, _, _, _, _, _) => {
                            h2681
                        }
                    };
                    let a2683 = 4u64;
                    nat_mul(a2682, a2683)?
                };
                nat_eq(a2684, a2685)
            } else {
                false
            } {
                f19(v0.clone(), v2.clone())?
            } else {
                false
            } {
                f19(v3.clone(), v2.clone())?
            } else {
                false
            } {
                Ok(Adt10::C4({
                    let a2687 = f18(v2.clone(), 1u64)?;
                    let a2688 = f167(match v3.clone() {
                        Adt2::C0(_, h2686, _, _, _, _, _, _, _, _, _) => {
                            h2686
                        }
                    })?;
                    nat_mul(a2687, a2688)?
                }, 4u64, f22(v0, 1u64), f22(v2, 1u64), Adt4::C29(f22(v3, 1u64))))
            } else {
                Ok(Adt10::C1)
            }
        }
    }
}

pub fn f96(v0: bool, v1: Adt1, v2: Adt2, v3: Adt2) -> R<Adt10> {
    let v4: Adt6 = Adt6::C0(match v1.clone() {
        Adt1::C0(_, h2689, _, _, _, _, _, _, _, _, _, _) => {
            h2689
        }
    }, if v0 {
        match v1.clone() {
            Adt1::C0(_, _, _, h2690, _, _, _, _, _, _, _, _) => {
                h2690
            }
        }
    } else {
        match v1.clone() {
            Adt1::C0(_, _, h2691, _, _, _, _, _, _, _, _, _) => {
                h2691
            }
        }
    }, if v0 {
        match v1.clone() {
            Adt1::C0(_, _, _, _, _, h2692, _, _, _, _, _, _) => {
                h2692
            }
        }
    } else {
        match v1.clone() {
            Adt1::C0(_, _, _, h2693, _, _, _, _, _, _, _, _) => {
                h2693
            }
        }
    }, 1u64);
    let v5: Adt6 = if v0 {
        Adt6::C0(match v1.clone() {
            Adt1::C0(_, _, h2694, _, _, _, _, _, _, _, _, _) => {
                h2694
            }
        }, match v1.clone() {
            Adt1::C0(_, _, _, _, h2695, _, _, _, _, _, _, _) => {
                h2695
            }
        }, match v1.clone() {
            Adt1::C0(_, _, _, _, _, _, h2696, _, _, _, _, _) => {
                h2696
            }
        }, 1u64)
    } else {
        Adt6::C0(1u64, 1u64, 0u64, 1u64)
    };
    let a2698 = match v1.clone() {
        Adt1::C0(h2697, _, _, _, _, _, _, _, _, _, _, _) => {
            h2697
        }
    };
    let a2699 = 1u64;
    if if if if if if if if if if if nat_le(a2698, a2699) {
        let a2701 = match v2.clone() {
            Adt2::C0(h2700, _, _, _, _, _, _, _, _, _, _) => {
                h2700
            }
        };
        let a2702 = 1u64;
        nat_le(a2701, a2702)
    } else {
        false
    } {
        let a2704 = match v3.clone() {
            Adt2::C0(h2703, _, _, _, _, _, _, _, _, _, _) => {
                h2703
            }
        };
        let a2705 = 0u64;
        nat_eq(a2704, a2705)
    } else {
        false
    } {
        let a2710 = match v2.clone() {
            Adt2::C0(_, _, _, _, _, h2706, _, _, _, _, _) => {
                h2706
            }
        };
        let a2711 = {
            let a2708 = match v2.clone() {
                Adt2::C0(h2707, _, _, _, _, _, _, _, _, _, _) => {
                    h2707
                }
            };
            let a2709 = 0u64;
            if nat_eq(a2708, a2709) {
                4u64
            } else {
                2u64
            }
        };
        nat_eq(a2710, a2711)
    } else {
        false
    } {
        f28(v3.clone())?
    } else {
        false
    } {
        f19(v2.clone(), v3.clone())?
    } else {
        false
    } {
        f196(v4.clone(), match v2.clone() {
            Adt2::C0(_, h2712, _, _, _, _, _, _, _, _, _) => {
                h2712
            }
        }, match v3.clone() {
            Adt2::C0(_, h2713, _, _, _, _, _, _, _, _, _) => {
                h2713
            }
        })?
    } else {
        false
    } {
        if v0 {
            f196(v5.clone(), match v2.clone() {
                Adt2::C0(_, _, h2714, _, _, _, _, _, _, _, _) => {
                    h2714
                }
            }, match v3.clone() {
                Adt2::C0(_, _, h2715, _, _, _, _, _, _, _, _) => {
                    h2715
                }
            })?
        } else {
            let a2718 = match v2.clone() {
                Adt2::C0(_, _, h2716, _, _, _, _, _, _, _, _) => {
                    h2716
                }
            };
            let a2719 = match v3.clone() {
                Adt2::C0(_, _, h2717, _, _, _, _, _, _, _, _) => {
                    h2717
                }
            };
            nat_eq(a2718, a2719)
        }
    } else {
        false
    } {
        let a2722 = match v2.clone() {
            Adt2::C0(_, _, _, h2720, _, _, _, _, _, _, _) => {
                h2720
            }
        };
        let a2723 = match v3.clone() {
            Adt2::C0(_, _, _, h2721, _, _, _, _, _, _, _) => {
                h2721
            }
        };
        nat_eq(a2722, a2723)
    } else {
        false
    } {
        let a2726 = match v2.clone() {
            Adt2::C0(_, _, _, _, h2724, _, _, _, _, _, _) => {
                h2724
            }
        };
        let a2727 = match v3.clone() {
            Adt2::C0(_, _, _, _, h2725, _, _, _, _, _, _) => {
                h2725
            }
        };
        nat_eq(a2726, a2727)
    } else {
        false
    } {
        let a2732 = {
            let a2730 = match v4.clone() {
                Adt6::C0(h2728, _, _, _) => {
                    h2728
                }
            };
            let a2731 = match v5.clone() {
                Adt6::C0(h2729, _, _, _) => {
                    h2729
                }
            };
            nat_mul(a2730, a2731)?
        };
        let a2733 = 2147483648u64;
        nat_lt(a2732, a2733)
    } else {
        false
    } {
        let v6: Adt7 = Adt7::C0(v4, v5, v0, {
            let a2735 = match v1 {
                Adt1::C0(h2734, _, _, _, _, _, _, _, _, _, _, _) => {
                    h2734
                }
            };
            let a2736 = 0u64;
            nat_eq(a2735, a2736)
        }, match v2.clone() {
            Adt2::C0(h2737, _, _, _, _, _, _, _, _, _, _) => {
                h2737
            }
        });
        Ok(Adt10::C4({
            let a2738 = f18(v3.clone(), 1u64)?;
            let a2739 = f163(v6.clone())?;
            nat_mul(a2738, a2739)?
        }, 4u64, f22(v2, 1u64), f22(v3, 1u64), Adt4::C28(v6)))
    } else {
        Ok(Adt10::C1)
    }
}

pub fn f97(v0: Adt1, v1: Adt2, v2: Option<Adt2>, v3: Adt2) -> R<Adt10> {
    let m2740 = v2;
    match m2740 {
        None => {
            Ok(Adt10::C1)
        }
        Some(v4) => {
            let v5: bool = {
                let a2742 = match v0.clone() {
                    Adt1::C0(_, _, _, _, _, _, h2741, _, _, _, _, _) => {
                        h2741
                    }
                };
                let a2743 = 1u64;
                nat_eq(a2742, a2743)
            };
            let v6: Adt6 = Adt6::C0(match v1.clone() {
                Adt2::C0(_, h2744, _, _, _, _, _, _, _, _, _) => {
                    h2744
                }
            }, match v0.clone() {
                Adt1::C0(h2745, _, _, _, _, _, _, _, _, _, _, _) => {
                    h2745
                }
            }, match v0.clone() {
                Adt1::C0(_, _, h2746, _, _, _, _, _, _, _, _, _) => {
                    h2746
                }
            }, match v0.clone() {
                Adt1::C0(_, _, _, _, h2747, _, _, _, _, _, _, _) => {
                    h2747
                }
            });
            let v7: Adt6 = if v5 {
                Adt6::C0(match v1.clone() {
                    Adt2::C0(_, _, h2748, _, _, _, _, _, _, _, _) => {
                        h2748
                    }
                }, match v0.clone() {
                    Adt1::C0(_, h2749, _, _, _, _, _, _, _, _, _, _) => {
                        h2749
                    }
                }, match v0.clone() {
                    Adt1::C0(_, _, _, h2750, _, _, _, _, _, _, _, _) => {
                        h2750
                    }
                }, match v0.clone() {
                    Adt1::C0(_, _, _, _, _, h2751, _, _, _, _, _, _) => {
                        h2751
                    }
                })
            } else {
                Adt6::C0(1u64, 1u64, 0u64, 1u64)
            };
            let v8: Adt6 = Adt6::C0(1u64, 1u64, 0u64, 1u64);
            let v9: u64 = if v5 {
                match v4.clone() {
                    Adt2::C0(_, _, _, h2752, _, _, _, _, _, _, _) => {
                        h2752
                    }
                }
            } else {
                match v4.clone() {
                    Adt2::C0(_, _, h2753, _, _, _, _, _, _, _, _) => {
                        h2753
                    }
                }
            };
            let v10: u64 = if v5 {
                match v4.clone() {
                    Adt2::C0(_, _, _, _, h2754, _, _, _, _, _, _) => {
                        h2754
                    }
                }
            } else {
                match v4.clone() {
                    Adt2::C0(_, _, _, h2755, _, _, _, _, _, _, _) => {
                        h2755
                    }
                }
            };
            let v11: u64 = if v5 {
                match v4.clone() {
                    Adt2::C0(_, _, h2756, _, _, _, _, _, _, _, _) => {
                        h2756
                    }
                }
            } else {
                1u64
            };
            let v12: u64 = match v4.clone() {
                Adt2::C0(h2757, _, _, _, _, _, _, _, _, _, _) => {
                    h2757
                }
            };
            let v13: u64 = match v3.clone() {
                Adt2::C0(h2758, _, _, _, _, _, _, _, _, _, _) => {
                    h2758
                }
            };
            let v14: u64 = {
                let a2759 = v12;
                let a2760 = 0u64;
                if nat_eq(a2759, a2760) {
                    4u64
                } else {
                    2u64
                }
            };
            let v15: u64 = {
                let a2762 = match v4.clone() {
                    Adt2::C0(_, h2761, _, _, _, _, _, _, _, _, _) => {
                        h2761
                    }
                };
                let a2763 = v14;
                nat_mul(a2762, a2763)?
            };
            let v16: u64 = if v5 {
                match v4.clone() {
                    Adt2::C0(_, _, _, _, _, _, _, h2764, _, _, _) => {
                        h2764
                    }
                }
            } else {
                match v4.clone() {
                    Adt2::C0(_, _, _, _, _, _, h2765, _, _, _, _) => {
                        h2765
                    }
                }
            };
            let v17: u64 = if v5 {
                match v4.clone() {
                    Adt2::C0(_, _, _, _, _, _, _, _, h2766, _, _) => {
                        h2766
                    }
                }
            } else {
                match v4.clone() {
                    Adt2::C0(_, _, _, _, _, _, _, h2767, _, _, _) => {
                        h2767
                    }
                }
            };
            let a2769 = match v0 {
                Adt1::C0(_, _, _, _, _, _, h2768, _, _, _, _, _) => {
                    h2768
                }
            };
            let a2770 = 1u64;
            if if if if if if if if if if nat_le(a2769, a2770) {
                let a2771 = v12;
                let a2772 = 0u64;
                if nat_eq(a2771, a2772) {
                    true
                } else {
                    let a2773 = v12;
                    let a2774 = 1u64;
                    if nat_eq(a2773, a2774) {
                        let a2775 = v13;
                        let a2776 = 1u64;
                        nat_eq(a2775, a2776)
                    } else {
                        false
                    }
                }
            } else {
                false
            } {
                let a2777 = v13;
                let a2778 = 0u64;
                if nat_eq(a2777, a2778) {
                    true
                } else {
                    let a2779 = v13;
                    let a2780 = 1u64;
                    nat_eq(a2779, a2780)
                }
            } else {
                false
            } {
                let a2782 = match v4.clone() {
                    Adt2::C0(_, _, _, _, _, h2781, _, _, _, _, _) => {
                        h2781
                    }
                };
                let a2783 = v14;
                nat_eq(a2782, a2783)
            } else {
                false
            } {
                let a2791 = match v3.clone() {
                    Adt2::C0(_, h2784, _, _, _, _, _, _, _, _, _) => {
                        h2784
                    }
                };
                let a2792 = {
                    let a2789 = v9;
                    let a2790 = {
                        let a2787 = match v6.clone() {
                            Adt6::C0(h2785, _, _, _) => {
                                h2785
                            }
                        };
                        let a2788 = match v7.clone() {
                            Adt6::C0(h2786, _, _, _) => {
                                h2786
                            }
                        };
                        nat_mul(a2787, a2788)?
                    };
                    nat_mul(a2789, a2790)?
                };
                nat_eq(a2791, a2792)
            } else {
                false
            } {
                let a2795 = match v3.clone() {
                    Adt2::C0(_, _, h2793, _, _, _, _, _, _, _, _) => {
                        h2793
                    }
                };
                let a2796 = f77(v6.clone(), match v4.clone() {
                    Adt2::C0(_, h2794, _, _, _, _, _, _, _, _, _) => {
                        h2794
                    }
                })?;
                nat_eq(a2795, a2796)
            } else {
                false
            } {
                let a2798 = 0u64;
                let a2799 = match v3.clone() {
                    Adt2::C0(_, _, h2797, _, _, _, _, _, _, _, _) => {
                        h2797
                    }
                };
                nat_lt(a2798, a2799)
            } else {
                false
            } {
                if v5 {
                    let a2801 = match v1 {
                        Adt2::C0(_, _, _, h2800, _, _, _, _, _, _, _) => {
                            h2800
                        }
                    };
                    let a2802 = v9;
                    if if if nat_eq(a2801, a2802) {
                        let a2804 = match v3.clone() {
                            Adt2::C0(_, _, _, h2803, _, _, _, _, _, _, _) => {
                                h2803
                            }
                        };
                        let a2805 = f77(v7.clone(), v11)?;
                        nat_eq(a2804, a2805)
                    } else {
                        false
                    } {
                        let a2807 = match v3.clone() {
                            Adt2::C0(_, _, _, _, h2806, _, _, _, _, _, _) => {
                                h2806
                            }
                        };
                        let a2808 = v10;
                        nat_eq(a2807, a2808)
                    } else {
                        false
                    } {
                        let a2810 = 0u64;
                        let a2811 = match v3.clone() {
                            Adt2::C0(_, _, _, h2809, _, _, _, _, _, _, _) => {
                                h2809
                            }
                        };
                        nat_lt(a2810, a2811)
                    } else {
                        false
                    }
                } else {
                    let a2813 = match v1 {
                        Adt2::C0(_, _, h2812, _, _, _, _, _, _, _, _) => {
                            h2812
                        }
                    };
                    let a2814 = v9;
                    if if if nat_eq(a2813, a2814) {
                        let a2816 = match v4.clone() {
                            Adt2::C0(_, _, _, _, h2815, _, _, _, _, _, _) => {
                                h2815
                            }
                        };
                        let a2817 = 1u64;
                        nat_eq(a2816, a2817)
                    } else {
                        false
                    } {
                        let a2819 = match v3.clone() {
                            Adt2::C0(_, _, _, h2818, _, _, _, _, _, _, _) => {
                                h2818
                            }
                        };
                        let a2820 = v10;
                        nat_eq(a2819, a2820)
                    } else {
                        false
                    } {
                        let a2822 = match v3.clone() {
                            Adt2::C0(_, _, _, _, h2821, _, _, _, _, _, _) => {
                                h2821
                            }
                        };
                        let a2823 = 1u64;
                        nat_eq(a2822, a2823)
                    } else {
                        false
                    }
                }
            } else {
                false
            } {
                if f38(match v4.clone() {
                    Adt2::C0(_, h2824, _, _, _, _, _, _, _, _, _) => {
                        h2824
                    }
                }, v11, v9, v10) {
                    true
                } else {
                    let a2844 = {
                        let a2841 = {
                            let a2839 = {
                                let a2833 = {
                                    let a2827 = {
                                        let a2825 = v10;
                                        let a2826 = 1u64;
                                        nat_sub(a2825, a2826)
                                    };
                                    let a2828 = v17;
                                    nat_mul(a2827, a2828)?
                                };
                                let a2834 = {
                                    let a2831 = {
                                        let a2829 = v9;
                                        let a2830 = 1u64;
                                        nat_sub(a2829, a2830)
                                    };
                                    let a2832 = v16;
                                    nat_mul(a2831, a2832)?
                                };
                                nat_add(a2833, a2834)?
                            };
                            let a2840 = {
                                let a2837 = {
                                    let a2835 = v11;
                                    let a2836 = 1u64;
                                    nat_sub(a2835, a2836)
                                };
                                let a2838 = v15;
                                nat_mul(a2837, a2838)?
                            };
                            nat_add(a2839, a2840)?
                        };
                        let a2842 = v15;
                        nat_add(a2841, a2842)?
                    };
                    let a2845 = match v4.clone() {
                        Adt2::C0(_, _, _, _, _, _, _, _, _, h2843, _) => {
                            h2843
                        }
                    };
                    nat_le(a2844, a2845)
                }
            } else {
                false
            } {
                f19(v4.clone(), v3.clone())?
            } else {
                false
            } {
                Ok(Adt10::C4(f18(v3.clone(), 1u64)?, {
                    let a2846 = v13;
                    let a2847 = 0u64;
                    if nat_eq(a2846, a2847) {
                        4u64
                    } else {
                        2u64
                    }
                }, Adt3::C0(match v4 {
                    Adt2::C0(_, h2848, _, _, _, _, _, _, _, _, _) => {
                        h2848
                    }
                }, v11, 1u64, v10, v14, v15, 0u64, 0u64), if v5 {
                    f22(v3, 1u64)
                } else {
                    Adt3::C0(match v3.clone() {
                        Adt2::C0(_, h2849, _, _, _, _, _, _, _, _, _) => {
                            h2849
                        }
                    }, match v3.clone() {
                        Adt2::C0(_, _, h2850, _, _, _, _, _, _, _, _) => {
                            h2850
                        }
                    }, 1u64, match v3.clone() {
                        Adt2::C0(_, _, _, h2851, _, _, _, _, _, _, _) => {
                            h2851
                        }
                    }, match v3.clone() {
                        Adt2::C0(_, _, _, _, _, h2852, _, _, _, _, _) => {
                            h2852
                        }
                    }, match v3.clone() {
                        Adt2::C0(_, _, _, _, _, _, h2853, _, _, _, _) => {
                            h2853
                        }
                    }, 0u64, match v3 {
                        Adt2::C0(_, _, _, _, _, _, _, h2854, _, _, _) => {
                            h2854
                        }
                    })
                }, Adt4::C27(Adt5::C0(v6, v7, v8, v16, v17, v12, 1u64, v13))))
            } else {
                Ok(Adt10::C1)
            }
        }
    }
}

pub fn f98(v0: Adt1, v1: Adt2, v2: Option<Adt2>, v3: Adt2) -> R<Adt10> {
    let m2855 = v2;
    match m2855 {
        None => {
            Ok(Adt10::C1)
        }
        Some(v4) => {
            let v5: Adt6 = Adt6::C0(match v1.clone() {
                Adt2::C0(_, h2856, _, _, _, _, _, _, _, _, _) => {
                    h2856
                }
            }, match v0.clone() {
                Adt1::C0(h2857, _, _, _, _, _, _, _, _, _, _, _) => {
                    h2857
                }
            }, match v0.clone() {
                Adt1::C0(_, _, _, h2858, _, _, _, _, _, _, _, _) => {
                    h2858
                }
            }, match v0.clone() {
                Adt1::C0(_, _, _, _, _, _, h2859, _, _, _, _, _) => {
                    h2859
                }
            });
            let v6: Adt6 = Adt6::C0(match v1.clone() {
                Adt2::C0(_, _, h2860, _, _, _, _, _, _, _, _) => {
                    h2860
                }
            }, match v0.clone() {
                Adt1::C0(_, h2861, _, _, _, _, _, _, _, _, _, _) => {
                    h2861
                }
            }, match v0.clone() {
                Adt1::C0(_, _, _, _, h2862, _, _, _, _, _, _, _) => {
                    h2862
                }
            }, match v0.clone() {
                Adt1::C0(_, _, _, _, _, _, _, h2863, _, _, _, _) => {
                    h2863
                }
            });
            let v7: Adt6 = Adt6::C0(match v1 {
                Adt2::C0(_, _, _, h2864, _, _, _, _, _, _, _) => {
                    h2864
                }
            }, match v0.clone() {
                Adt1::C0(_, _, h2865, _, _, _, _, _, _, _, _, _) => {
                    h2865
                }
            }, match v0.clone() {
                Adt1::C0(_, _, _, _, _, h2866, _, _, _, _, _, _) => {
                    h2866
                }
            }, match v0.clone() {
                Adt1::C0(_, _, _, _, _, _, _, _, h2867, _, _, _) => {
                    h2867
                }
            });
            let v8: u64 = match v0 {
                Adt1::C0(_, _, _, _, _, _, _, _, _, h2868, _, _) => {
                    h2868
                }
            };
            let v9: u64 = f77(v7.clone(), match v4.clone() {
                Adt2::C0(_, _, _, h2869, _, _, _, _, _, _, _) => {
                    h2869
                }
            })?;
            let v10: u64 = match v3.clone() {
                Adt2::C0(h2870, _, _, _, _, _, _, _, _, _, _) => {
                    h2870
                }
            };
            let v11: u64 = {
                let a2878 = v8;
                let a2879 = {
                    let a2876 = match v5.clone() {
                        Adt6::C0(h2871, _, _, _) => {
                            h2871
                        }
                    };
                    let a2877 = {
                        let a2874 = match v6.clone() {
                            Adt6::C0(h2872, _, _, _) => {
                                h2872
                            }
                        };
                        let a2875 = match v7.clone() {
                            Adt6::C0(h2873, _, _, _) => {
                                h2873
                            }
                        };
                        nat_mul(a2874, a2875)?
                    };
                    nat_mul(a2876, a2877)?
                };
                nat_mul(a2878, a2879)?
            };
            let a2881 = match v4.clone() {
                Adt2::C0(h2880, _, _, _, _, _, _, _, _, _, _) => {
                    h2880
                }
            };
            let a2882 = 0u64;
            if if if if if if if if if if if if if nat_eq(a2881, a2882) {
                let a2883 = v10;
                let a2884 = 0u64;
                if nat_eq(a2883, a2884) {
                    true
                } else {
                    let a2885 = v10;
                    let a2886 = 1u64;
                    nat_eq(a2885, a2886)
                }
            } else {
                false
            } {
                let a2887 = 0u64;
                let a2888 = v8;
                nat_lt(a2887, a2888)
            } else {
                false
            } {
                let a2889 = v8;
                let a2890 = 2147483648u64;
                nat_lt(a2889, a2890)
            } else {
                false
            } {
                let a2892 = match v4.clone() {
                    Adt2::C0(_, _, _, _, _, h2891, _, _, _, _, _) => {
                        h2891
                    }
                };
                let a2893 = 4u64;
                nat_eq(a2892, a2893)
            } else {
                false
            } {
                let a2894 = 0u64;
                let a2895 = v9;
                nat_lt(a2894, a2895)
            } else {
                false
            } {
                let a2897 = 0u64;
                let a2898 = match v3.clone() {
                    Adt2::C0(_, _, h2896, _, _, _, _, _, _, _, _) => {
                        h2896
                    }
                };
                nat_lt(a2897, a2898)
            } else {
                false
            } {
                let a2900 = 0u64;
                let a2901 = match v3.clone() {
                    Adt2::C0(_, _, _, h2899, _, _, _, _, _, _, _) => {
                        h2899
                    }
                };
                nat_lt(a2900, a2901)
            } else {
                false
            } {
                let a2903 = match v3.clone() {
                    Adt2::C0(_, h2902, _, _, _, _, _, _, _, _, _) => {
                        h2902
                    }
                };
                let a2904 = v11;
                nat_eq(a2903, a2904)
            } else {
                false
            } {
                let a2907 = match v3.clone() {
                    Adt2::C0(_, _, h2905, _, _, _, _, _, _, _, _) => {
                        h2905
                    }
                };
                let a2908 = f77(v5.clone(), match v4.clone() {
                    Adt2::C0(_, h2906, _, _, _, _, _, _, _, _, _) => {
                        h2906
                    }
                })?;
                nat_eq(a2907, a2908)
            } else {
                false
            } {
                let a2911 = match v3.clone() {
                    Adt2::C0(_, _, _, h2909, _, _, _, _, _, _, _) => {
                        h2909
                    }
                };
                let a2912 = f77(v6.clone(), match v4.clone() {
                    Adt2::C0(_, _, h2910, _, _, _, _, _, _, _, _) => {
                        h2910
                    }
                })?;
                nat_eq(a2911, a2912)
            } else {
                false
            } {
                let a2920 = match v3.clone() {
                    Adt2::C0(_, _, _, _, h2913, _, _, _, _, _, _) => {
                        h2913
                    }
                };
                let a2921 = {
                    let a2918 = v9;
                    let a2919 = {
                        let a2915 = match v4.clone() {
                            Adt2::C0(_, _, _, _, h2914, _, _, _, _, _, _) => {
                                h2914
                            }
                        };
                        let a2916 = v8;
                        let a2917 = 0u64;
                        nat_quot(a2915, a2916, a2917)
                    };
                    nat_mul(a2918, a2919)?
                };
                nat_eq(a2920, a2921)
            } else {
                false
            } {
                f19(v4.clone(), v3.clone())?
            } else {
                false
            } {
                Ok(Adt10::C4(f18(v3.clone(), 1u64)?, {
                    let a2922 = v10;
                    let a2923 = 0u64;
                    if nat_eq(a2922, a2923) {
                        4u64
                    } else {
                        2u64
                    }
                }, f22(v4.clone(), 1u64), f22(v3, 1u64), Adt4::C27(Adt5::C0(v5, v6, v7, match v4.clone() {
                    Adt2::C0(_, _, _, _, _, _, _, _, h2924, _, _) => {
                        h2924
                    }
                }, {
                    let a2926 = v8;
                    let a2927 = match v4 {
                        Adt2::C0(_, _, _, _, _, _, _, _, h2925, _, _) => {
                            h2925
                        }
                    };
                    nat_mul(a2926, a2927)?
                }, 0u64, v9, v10))))
            } else {
                Ok(Adt10::C1)
            }
        }
    }
}

pub fn f99(v0: Adt2, v1: Option<Adt2>, v2: Adt2) -> R<Adt10> {
    let m2928 = v1;
    match m2928 {
        None => {
            Ok(Adt10::C1)
        }
        Some(v3) => {
            f197(v0, v3, v2)
        }
    }
}

pub fn f100(v0: Adt2, v1: Adt2) -> R<Adt10> {
    let a2930 = match v0.clone() {
        Adt2::C0(h2929, _, _, _, _, _, _, _, _, _, _) => {
            h2929
        }
    };
    let a2931 = 1u64;
    if if nat_eq(a2930, a2931) {
        true
    } else {
        let a2933 = match v0.clone() {
            Adt2::C0(h2932, _, _, _, _, _, _, _, _, _, _) => {
                h2932
            }
        };
        let a2934 = 30u64;
        nat_eq(a2933, a2934)
    } {
        let a2936 = match v1.clone() {
            Adt2::C0(h2935, _, _, _, _, _, _, _, _, _, _) => {
                h2935
            }
        };
        let a2937 = 1u64;
        if if if if if if if if if if if nat_eq(a2936, a2937) {
            let a2940 = match v0.clone() {
                Adt2::C0(_, h2938, _, _, _, _, _, _, _, _, _) => {
                    h2938
                }
            };
            let a2941 = match v1.clone() {
                Adt2::C0(_, h2939, _, _, _, _, _, _, _, _, _) => {
                    h2939
                }
            };
            nat_eq(a2940, a2941)
        } else {
            false
        } {
            let a2943 = 0u64;
            let a2944 = match v1.clone() {
                Adt2::C0(_, _, h2942, _, _, _, _, _, _, _, _) => {
                    h2942
                }
            };
            nat_lt(a2943, a2944)
        } else {
            false
        } {
            let a2947 = match v1.clone() {
                Adt2::C0(_, _, h2945, _, _, _, _, _, _, _, _) => {
                    h2945
                }
            };
            let a2948 = match v1.clone() {
                Adt2::C0(_, _, _, h2946, _, _, _, _, _, _, _) => {
                    h2946
                }
            };
            nat_eq(a2947, a2948)
        } else {
            false
        } {
            let a2950 = match v1.clone() {
                Adt2::C0(_, _, _, _, h2949, _, _, _, _, _, _) => {
                    h2949
                }
            };
            let a2951 = 1u64;
            nat_eq(a2950, a2951)
        } else {
            false
        } {
            let a2959 = match v0.clone() {
                Adt2::C0(_, _, h2952, _, _, _, _, _, _, _, _) => {
                    h2952
                }
            };
            let a2960 = {
                let a2957 = {
                    let a2955 = match v1.clone() {
                        Adt2::C0(_, _, h2953, _, _, _, _, _, _, _, _) => {
                            h2953
                        }
                    };
                    let a2956 = match v1.clone() {
                        Adt2::C0(_, _, h2954, _, _, _, _, _, _, _, _) => {
                            h2954
                        }
                    };
                    nat_add(a2955, a2956)?
                };
                let a2958 = 1u64;
                nat_sub(a2957, a2958)
            };
            nat_eq(a2959, a2960)
        } else {
            false
        } {
            let a2962 = 0u64;
            let a2963 = match v0.clone() {
                Adt2::C0(_, _, _, h2961, _, _, _, _, _, _, _) => {
                    h2961
                }
            };
            nat_lt(a2962, a2963)
        } else {
            false
        } {
            let a2965 = 0u64;
            let a2966 = match v0.clone() {
                Adt2::C0(_, _, _, _, h2964, _, _, _, _, _, _) => {
                    h2964
                }
            };
            nat_lt(a2965, a2966)
        } else {
            false
        } {
            let a2968 = 2u64;
            let a2969 = match v0.clone() {
                Adt2::C0(_, _, _, _, _, h2967, _, _, _, _, _) => {
                    h2967
                }
            };
            nat_le(a2968, a2969)
        } else {
            false
        } {
            let a2971 = 2u64;
            let a2972 = match v1.clone() {
                Adt2::C0(_, _, _, _, _, h2970, _, _, _, _, _) => {
                    h2970
                }
            };
            nat_le(a2971, a2972)
        } else {
            false
        } {
            f19(v0.clone(), v1.clone())?
        } else {
            false
        } {
            Ok(Adt10::C4(f18(v1.clone(), 1u64)?, 2u64, f22(v0, 1u64), f22(v1, 1u64), Adt4::C21))
        } else {
            Ok(Adt10::C1)
        }
    } else {
        Ok(Adt10::C0)
    }
}

pub fn f101(v0: Adt2, v1: Option<Adt2>, v2: Adt2) -> R<Adt10> {
    let m2973 = v1;
    match m2973 {
        None => {
            Ok(Adt10::C1)
        }
        Some(v3) => {
            f198(v0, v3, v2)
        }
    }
}

pub fn f102(v0: u64) -> bool {
    let a2974 = v0;
    let a2975 = 1u64;
    if if nat_eq(a2974, a2975) {
        true
    } else {
        let a2976 = v0;
        let a2977 = 33u64;
        nat_eq(a2976, a2977)
    } {
        true
    } else {
        let a2978 = v0;
        let a2979 = 34u64;
        nat_eq(a2978, a2979)
    }
}

pub fn f103(v0: Adt2, v1: Adt2, v2: Adt2) -> R<Adt10> {
    let v3: u64 = f18(v0.clone(), 1u64)?;
    if if if if if f37(v1, v2.clone()) {
        let a2980 = v3;
        let a2981 = f18(v2.clone(), 1u64)?;
        nat_eq(a2980, a2981)
    } else {
        false
    } {
        f17(v0.clone(), v3)?
    } else {
        false
    } {
        f17(v2.clone(), v3)?
    } else {
        false
    } {
        let a2982 = v3;
        let a2983 = 0u64;
        if nat_eq(a2982, a2983) {
            true
        } else {
            let a2985 = 4u64;
            let a2986 = match v2.clone() {
                Adt2::C0(_, _, _, _, _, h2984, _, _, _, _, _) => {
                    h2984
                }
            };
            nat_le(a2985, a2986)
        }
    } else {
        false
    } {
        Ok(Adt10::C4({
            let a2987 = 2u64;
            let a2988 = v3;
            nat_mul(a2987, a2988)?
        }, 4u64, f22(v0, 1u64), f22(v2, 1u64), Adt4::C35(v3)))
    } else {
        Ok(Adt10::C1)
    }
}

pub fn f104(v0: u64) -> bool {
    if f199(v0) {
        !f200(v0)
    } else {
        false
    }
}

pub fn f105(v0: Adt2, v1: Option<Adt2>, v2: Adt2) -> R<Adt10> {
    let m2989 = v1;
    match m2989 {
        None => {
            Ok(Adt10::C1)
        }
        Some(v3) => {
            let m2991 = f192(match v0.clone() {
                Adt2::C0(h2990, _, _, _, _, _, _, _, _, _, _) => {
                    h2990
                }
            });
            match m2991 {
                Adt18::C0 => {
                    Ok(Adt10::C0)
                }
                Adt18::C1(v4) => {
                    let v5: u64 = match v4.clone() {
                        Adt19::C0(_, h2992, _) => {
                            h2992
                        }
                    };
                    if if if if if if f37(v3, v2.clone()) {
                        f19(v0.clone(), v2.clone())?
                    } else {
                        false
                    } {
                        let a2993 = f18(v0.clone(), 1u64)?;
                        let a2994 = f18(v2.clone(), 1u64)?;
                        nat_eq(a2993, a2994)
                    } else {
                        false
                    } {
                        let a2999 = {
                            let a2996 = match v0.clone() {
                                Adt2::C0(_, h2995, _, _, _, _, _, _, _, _, _) => {
                                    h2995
                                }
                            };
                            let a2997 = v5;
                            let a2998 = 1u64;
                            nat_rem(a2996, a2997, a2998)
                        };
                        let a3000 = 0u64;
                        if nat_eq(a2999, a3000) {
                            let a3003 = match v4 {
                                Adt19::C0(_, _, h3001) => {
                                    h3001
                                }
                            };
                            let a3004 = match v0.clone() {
                                Adt2::C0(_, _, _, _, _, h3002, _, _, _, _, _) => {
                                    h3002
                                }
                            };
                            nat_le(a3003, a3004)
                        } else {
                            false
                        }
                    } else {
                        false
                    } {
                        let a3005 = v5;
                        let a3006 = 1u64;
                        if nat_eq(a3005, a3006) {
                            let a3008 = 4u64;
                            let a3009 = match v2.clone() {
                                Adt2::C0(_, _, _, _, _, h3007, _, _, _, _, _) => {
                                    h3007
                                }
                            };
                            nat_le(a3008, a3009)
                        } else {
                            let a3011 = match v2.clone() {
                                Adt2::C0(_, _, _, _, _, h3010, _, _, _, _, _) => {
                                    h3010
                                }
                            };
                            let a3012 = 4u64;
                            nat_eq(a3011, a3012)
                        }
                    } else {
                        false
                    } {
                        let a3017 = {
                            let a3014 = match v2.clone() {
                                Adt2::C0(_, h3013, _, _, _, _, _, _, _, _, _) => {
                                    h3013
                                }
                            };
                            let a3015 = v5;
                            let a3016 = 1u64;
                            nat_rem(a3014, a3015, a3016)
                        };
                        let a3018 = 0u64;
                        if nat_eq(a3017, a3018) {
                            true
                        } else {
                            f28(v2.clone())?
                        }
                    } else {
                        false
                    } {
                        Ok(Adt10::C4(f18(v0.clone(), v5)?, 4u64, f22(v0.clone(), v5), f22(v2, 1u64), Adt4::C11(match v0 {
                            Adt2::C0(h3019, _, _, _, _, _, _, _, _, _, _) => {
                                h3019
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

pub fn f106(v0: u64, v1: Adt2, v2: bool, v3: Adt2) -> R<Adt10> {
    let a3022 = match v1.clone() {
        Adt2::C0(h3020, _, _, _, _, _, _, _, _, _, _) => {
            h3020
        }
    };
    let a3023 = match v3.clone() {
        Adt2::C0(h3021, _, _, _, _, _, _, _, _, _, _) => {
            h3021
        }
    };
    if nat_eq(a3022, a3023) {
        f201(v0, v1, v2, v3)
    } else {
        if if if if f200(match v1.clone() {
            Adt2::C0(h3024, _, _, _, _, _, _, _, _, _, _) => {
                h3024
            }
        }) {
            f200(match v3.clone() {
                Adt2::C0(h3025, _, _, _, _, _, _, _, _, _, _) => {
                    h3025
                }
            })
        } else {
            false
        } {
            let a3026 = v0;
            let a3027 = 33u64;
            if nat_eq(a3026, a3027) {
                v2
            } else {
                true
            }
        } else {
            false
        } {
            let a3028 = f18(v1.clone(), 1u64)?;
            let a3029 = f18(v3.clone(), 1u64)?;
            nat_eq(a3028, a3029)
        } else {
            false
        } {
            let m3031 = f192(match v3.clone() {
                Adt2::C0(h3030, _, _, _, _, _, _, _, _, _, _) => {
                    h3030
                }
            });
            match m3031 {
                Adt18::C0 => {
                    Ok(Adt10::C0)
                }
                Adt18::C1(v4) => {
                    let v5: u64 = match v1.clone() {
                        Adt2::C0(h3032, _, _, _, _, _, _, _, _, _, _) => {
                            h3032
                        }
                    };
                    let v6: u64 = match v3.clone() {
                        Adt2::C0(h3033, _, _, _, _, _, _, _, _, _, _) => {
                            h3033
                        }
                    };
                    let v7: bool = {
                        let a3034 = v5;
                        let a3035 = 43u64;
                        if nat_eq(a3034, a3035) {
                            true
                        } else {
                            let a3036 = v6;
                            let a3037 = 43u64;
                            nat_eq(a3036, a3037)
                        }
                    };
                    Ok(Adt10::C4({
                        let a3038 = f18(v3.clone(), 1u64)?;
                        let a3039 = if v7 {
                            f147(v5, v6)
                        } else {
                            1u64
                        };
                        nat_mul(a3038, a3039)?
                    }, match v4 {
                        Adt19::C0(_, _, h3040) => {
                            h3040
                        }
                    }, f22(v1, 1u64), f22(v3, 1u64), if v7 {
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

pub fn f107(v0: Adt2, v1: Adt2) -> R<Adt10> {
    let a3042 = match v0.clone() {
        Adt2::C0(h3041, _, _, _, _, _, _, _, _, _, _) => {
            h3041
        }
    };
    let a3043 = 0u64;
    if if if if nat_eq(a3042, a3043) {
        true
    } else {
        let a3045 = match v0.clone() {
            Adt2::C0(h3044, _, _, _, _, _, _, _, _, _, _) => {
                h3044
            }
        };
        let a3046 = 1u64;
        nat_eq(a3045, a3046)
    } {
        true
    } else {
        let a3048 = match v0.clone() {
            Adt2::C0(h3047, _, _, _, _, _, _, _, _, _, _) => {
                h3047
            }
        };
        let a3049 = 25u64;
        if nat_eq(a3048, a3049) {
            true
        } else {
            let a3051 = match v0.clone() {
                Adt2::C0(h3050, _, _, _, _, _, _, _, _, _, _) => {
                    h3050
                }
            };
            let a3052 = 26u64;
            nat_eq(a3051, a3052)
        }
    } {
        true
    } else {
        let a3054 = match v0.clone() {
            Adt2::C0(h3053, _, _, _, _, _, _, _, _, _, _) => {
                h3053
            }
        };
        let a3055 = 30u64;
        nat_eq(a3054, a3055)
    } {
        let a3058 = match v0.clone() {
            Adt2::C0(h3056, _, _, _, _, _, _, _, _, _, _) => {
                h3056
            }
        };
        let a3059 = match v1.clone() {
            Adt2::C0(h3057, _, _, _, _, _, _, _, _, _, _) => {
                h3057
            }
        };
        if if nat_eq(a3058, a3059) {
            f202(match v1.clone() {
                Adt2::C0(_, h3060, _, _, _, _, _, _, _, _, _) => {
                    h3060
                }
            }, match v1.clone() {
                Adt2::C0(_, _, h3061, _, _, _, _, _, _, _, _) => {
                    h3061
                }
            }, match v1.clone() {
                Adt2::C0(_, _, _, h3062, _, _, _, _, _, _, _) => {
                    h3062
                }
            }, match v1.clone() {
                Adt2::C0(_, _, _, _, h3063, _, _, _, _, _, _) => {
                    h3063
                }
            }, match v0.clone() {
                Adt2::C0(_, h3064, _, _, _, _, _, _, _, _, _) => {
                    h3064
                }
            }, match v0.clone() {
                Adt2::C0(_, _, h3065, _, _, _, _, _, _, _, _) => {
                    h3065
                }
            }, match v0.clone() {
                Adt2::C0(_, _, _, h3066, _, _, _, _, _, _, _) => {
                    h3066
                }
            }, match v0.clone() {
                Adt2::C0(_, _, _, _, h3067, _, _, _, _, _, _) => {
                    h3067
                }
            })
        } else {
            false
        } {
            let m3069 = f192(match v0.clone() {
                Adt2::C0(h3068, _, _, _, _, _, _, _, _, _, _) => {
                    h3068
                }
            });
            match m3069 {
                Adt18::C0 => {
                    Ok(Adt10::C0)
                }
                Adt18::C1(v2) => {
                    Ok(Adt10::C4(f18(v1.clone(), 1u64)?, match v2 {
                        Adt19::C0(_, _, h3070) => {
                            h3070
                        }
                    }, f22(v0, 1u64), f22(v1, 1u64), Adt4::C1))
                }
            }
        } else {
            Ok(Adt10::C1)
        }
    } else {
        Ok(Adt10::C0)
    }
}

pub fn f108(v0: u64, v1: Adt2, v2: Option<Adt2>, v3: Adt2) -> R<Adt10> {
    let m3071 = v2;
    match m3071 {
        None => {
            Ok(Adt10::C1)
        }
        Some(v4) => {
            let a3072 = v0;
            let a3073 = 4u64;
            if if if nat_lt(a3072, a3073) {
                let a3076 = match v1.clone() {
                    Adt2::C0(h3074, _, _, _, _, _, _, _, _, _, _) => {
                        h3074
                    }
                };
                let a3077 = match v4.clone() {
                    Adt2::C0(h3075, _, _, _, _, _, _, _, _, _, _) => {
                        h3075
                    }
                };
                if nat_eq(a3076, a3077) {
                    let a3080 = match v1.clone() {
                        Adt2::C0(h3078, _, _, _, _, _, _, _, _, _, _) => {
                            h3078
                        }
                    };
                    let a3081 = match v3.clone() {
                        Adt2::C0(h3079, _, _, _, _, _, _, _, _, _, _) => {
                            h3079
                        }
                    };
                    nat_eq(a3080, a3081)
                } else {
                    false
                }
            } else {
                false
            } {
                if if if f203(v0, 0u64, match v1.clone() {
                    Adt2::C0(_, h3082, _, _, _, _, _, _, _, _, _) => {
                        h3082
                    }
                }, match v4.clone() {
                    Adt2::C0(_, h3083, _, _, _, _, _, _, _, _, _) => {
                        h3083
                    }
                }, match v3.clone() {
                    Adt2::C0(_, h3084, _, _, _, _, _, _, _, _, _) => {
                        h3084
                    }
                })? {
                    f203(v0, 1u64, match v1.clone() {
                        Adt2::C0(_, _, h3085, _, _, _, _, _, _, _, _) => {
                            h3085
                        }
                    }, match v4.clone() {
                        Adt2::C0(_, _, h3086, _, _, _, _, _, _, _, _) => {
                            h3086
                        }
                    }, match v3.clone() {
                        Adt2::C0(_, _, h3087, _, _, _, _, _, _, _, _) => {
                            h3087
                        }
                    })?
                } else {
                    false
                } {
                    f203(v0, 2u64, match v1.clone() {
                        Adt2::C0(_, _, _, h3088, _, _, _, _, _, _, _) => {
                            h3088
                        }
                    }, match v4.clone() {
                        Adt2::C0(_, _, _, h3089, _, _, _, _, _, _, _) => {
                            h3089
                        }
                    }, match v3.clone() {
                        Adt2::C0(_, _, _, h3090, _, _, _, _, _, _, _) => {
                            h3090
                        }
                    })?
                } else {
                    false
                } {
                    f203(v0, 3u64, match v1.clone() {
                        Adt2::C0(_, _, _, _, h3091, _, _, _, _, _, _) => {
                            h3091
                        }
                    }, match v4.clone() {
                        Adt2::C0(_, _, _, _, h3092, _, _, _, _, _, _) => {
                            h3092
                        }
                    }, match v3.clone() {
                        Adt2::C0(_, _, _, _, h3093, _, _, _, _, _, _) => {
                            h3093
                        }
                    })?
                } else {
                    false
                }
            } else {
                false
            } {
                let m3095 = f192(match v1.clone() {
                    Adt2::C0(h3094, _, _, _, _, _, _, _, _, _, _) => {
                        h3094
                    }
                });
                match m3095 {
                    Adt18::C0 => {
                        Ok(Adt10::C0)
                    }
                    Adt18::C1(v5) => {
                        let v6: u64 = match v5.clone() {
                            Adt19::C0(_, h3096, _) => {
                                h3096
                            }
                        };
                        let a3101 = {
                            let a3098 = match v1.clone() {
                                Adt2::C0(_, h3097, _, _, _, _, _, _, _, _, _) => {
                                    h3097
                                }
                            };
                            let a3099 = v6;
                            let a3100 = 1u64;
                            nat_rem(a3098, a3099, a3100)
                        };
                        let a3102 = 0u64;
                        if if if nat_eq(a3101, a3102) {
                            let a3107 = {
                                let a3104 = match v4.clone() {
                                    Adt2::C0(_, h3103, _, _, _, _, _, _, _, _, _) => {
                                        h3103
                                    }
                                };
                                let a3105 = v6;
                                let a3106 = 1u64;
                                nat_rem(a3104, a3105, a3106)
                            };
                            let a3108 = 0u64;
                            nat_eq(a3107, a3108)
                        } else {
                            false
                        } {
                            let a3113 = {
                                let a3110 = match v3.clone() {
                                    Adt2::C0(_, h3109, _, _, _, _, _, _, _, _, _) => {
                                        h3109
                                    }
                                };
                                let a3111 = v6;
                                let a3112 = 1u64;
                                nat_rem(a3110, a3111, a3112)
                            };
                            let a3114 = 0u64;
                            nat_eq(a3113, a3114)
                        } else {
                            false
                        } {
                            Ok(Adt10::C4(f18(v3.clone(), v6)?, match v5 {
                                Adt19::C0(_, _, h3115) => {
                                    h3115
                                }
                            }, f22(v1, v6), f22(v3, v6), Adt4::C2(v0, f22(v4, v6))))
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

pub fn f109(v0: u64, v1: Adt1, v2: Adt2, v3: Adt2) -> R<Adt10> {
    let a3117 = match v2.clone() {
        Adt2::C0(h3116, _, _, _, _, _, _, _, _, _, _) => {
            h3116
        }
    };
    let a3118 = 0u64;
    if if nat_eq(a3117, a3118) {
        let a3120 = match v3.clone() {
            Adt2::C0(h3119, _, _, _, _, _, _, _, _, _, _) => {
                h3119
            }
        };
        let a3121 = 0u64;
        nat_eq(a3120, a3121)
    } else {
        false
    } {
        let a3122 = v0;
        let a3123 = 61u64;
        if nat_eq(a3122, a3123) {
            let v4: bool = {
                let a3125 = 0u64;
                let a3126 = match v1.clone() {
                    Adt1::C0(_, _, _, _, _, _, _, _, h3124, _, _, _) => {
                        h3124
                    }
                };
                nat_lt(a3125, a3126)
            };
            if if if if if if f204(match v2.clone() {
                Adt2::C0(_, h3127, _, _, _, _, _, _, _, _, _) => {
                    h3127
                }
            }, match v1.clone() {
                Adt1::C0(h3128, _, _, _, _, _, _, _, _, _, _, _) => {
                    h3128
                }
            }, match v1.clone() {
                Adt1::C0(_, h3129, _, _, _, _, _, _, _, _, _, _) => {
                    h3129
                }
            }, match v3.clone() {
                Adt2::C0(_, h3130, _, _, _, _, _, _, _, _, _) => {
                    h3130
                }
            })? {
                f204(match v2.clone() {
                    Adt2::C0(_, _, h3131, _, _, _, _, _, _, _, _) => {
                        h3131
                    }
                }, match v1.clone() {
                    Adt1::C0(_, _, h3132, _, _, _, _, _, _, _, _, _) => {
                        h3132
                    }
                }, match v1.clone() {
                    Adt1::C0(_, _, _, h3133, _, _, _, _, _, _, _, _) => {
                        h3133
                    }
                }, match v3.clone() {
                    Adt2::C0(_, _, h3134, _, _, _, _, _, _, _, _) => {
                        h3134
                    }
                })?
            } else {
                false
            } {
                f204(match v2.clone() {
                    Adt2::C0(_, _, _, h3135, _, _, _, _, _, _, _) => {
                        h3135
                    }
                }, match v1.clone() {
                    Adt1::C0(_, _, _, _, h3136, _, _, _, _, _, _, _) => {
                        h3136
                    }
                }, match v1.clone() {
                    Adt1::C0(_, _, _, _, _, h3137, _, _, _, _, _, _) => {
                        h3137
                    }
                }, match v3.clone() {
                    Adt2::C0(_, _, _, h3138, _, _, _, _, _, _, _) => {
                        h3138
                    }
                })?
            } else {
                false
            } {
                f204(match v2.clone() {
                    Adt2::C0(_, _, _, _, h3139, _, _, _, _, _, _) => {
                        h3139
                    }
                }, match v1.clone() {
                    Adt1::C0(_, _, _, _, _, _, h3140, _, _, _, _, _) => {
                        h3140
                    }
                }, match v1.clone() {
                    Adt1::C0(_, _, _, _, _, _, _, h3141, _, _, _, _) => {
                        h3141
                    }
                }, match v3.clone() {
                    Adt2::C0(_, _, _, _, h3142, _, _, _, _, _, _) => {
                        h3142
                    }
                })?
            } else {
                false
            } {
                let a3144 = match v1.clone() {
                    Adt1::C0(_, _, _, _, _, _, _, _, h3143, _, _, _) => {
                        h3143
                    }
                };
                let a3145 = 4294967295u64;
                nat_le(a3144, a3145)
            } else {
                false
            } {
                if v4 {
                    !f38(match v2.clone() {
                        Adt2::C0(_, h3146, _, _, _, _, _, _, _, _, _) => {
                            h3146
                        }
                    }, match v2.clone() {
                        Adt2::C0(_, _, h3147, _, _, _, _, _, _, _, _) => {
                            h3147
                        }
                    }, match v2.clone() {
                        Adt2::C0(_, _, _, h3148, _, _, _, _, _, _, _) => {
                            h3148
                        }
                    }, match v2.clone() {
                        Adt2::C0(_, _, _, _, h3149, _, _, _, _, _, _) => {
                            h3149
                        }
                    })
                } else {
                    true
                }
            } else {
                false
            } {
                Ok(Adt10::C4(f18(v3.clone(), 1u64)?, 4u64, f22(v2, 1u64), f22(v3, 1u64), Adt4::C3(match v1.clone() {
                    Adt1::C0(h3150, _, _, _, _, _, _, _, _, _, _, _) => {
                        h3150
                    }
                }, match v1.clone() {
                    Adt1::C0(_, _, h3151, _, _, _, _, _, _, _, _, _) => {
                        h3151
                    }
                }, match v1.clone() {
                    Adt1::C0(_, _, _, _, h3152, _, _, _, _, _, _, _) => {
                        h3152
                    }
                }, match v1 {
                    Adt1::C0(_, _, _, _, _, _, h3153, _, _, _, _, _) => {
                        h3153
                    }
                }, v4)))
            } else {
                Ok(Adt10::C1)
            }
        } else {
            let a3154 = v0;
            let a3155 = 62u64;
            if nat_eq(a3154, a3155) {
                let a3157 = match v1.clone() {
                    Adt1::C0(h3156, _, _, _, _, _, _, _, _, _, _, _) => {
                        h3156
                    }
                };
                let a3158 = 2147483648u64;
                if if if if if nat_lt(a3157, a3158) {
                    let a3160 = match v1.clone() {
                        Adt1::C0(_, h3159, _, _, _, _, _, _, _, _, _, _) => {
                            h3159
                        }
                    };
                    let a3161 = 2147483648u64;
                    nat_lt(a3160, a3161)
                } else {
                    false
                } {
                    let a3164 = match v1.clone() {
                        Adt1::C0(h3162, _, _, _, _, _, _, _, _, _, _, _) => {
                            h3162
                        }
                    };
                    let a3165 = match v2.clone() {
                        Adt2::C0(_, h3163, _, _, _, _, _, _, _, _, _) => {
                            h3163
                        }
                    };
                    nat_lt(a3164, a3165)
                } else {
                    false
                } {
                    let a3168 = match v1.clone() {
                        Adt1::C0(_, h3166, _, _, _, _, _, _, _, _, _, _) => {
                            h3166
                        }
                    };
                    let a3169 = match v2.clone() {
                        Adt2::C0(_, h3167, _, _, _, _, _, _, _, _, _) => {
                            h3167
                        }
                    };
                    nat_lt(a3168, a3169)
                } else {
                    false
                } {
                    let a3172 = match v2.clone() {
                        Adt2::C0(_, _, h3170, _, _, _, _, _, _, _, _) => {
                            h3170
                        }
                    };
                    let a3173 = match v3.clone() {
                        Adt2::C0(_, _, h3171, _, _, _, _, _, _, _, _) => {
                            h3171
                        }
                    };
                    if if if nat_eq(a3172, a3173) {
                        let a3176 = match v2.clone() {
                            Adt2::C0(_, _, _, h3174, _, _, _, _, _, _, _) => {
                                h3174
                            }
                        };
                        let a3177 = match v3.clone() {
                            Adt2::C0(_, _, _, h3175, _, _, _, _, _, _, _) => {
                                h3175
                            }
                        };
                        nat_eq(a3176, a3177)
                    } else {
                        false
                    } {
                        let a3180 = match v2.clone() {
                            Adt2::C0(_, _, _, _, h3178, _, _, _, _, _, _) => {
                                h3178
                            }
                        };
                        let a3181 = match v3.clone() {
                            Adt2::C0(_, _, _, _, h3179, _, _, _, _, _, _) => {
                                h3179
                            }
                        };
                        nat_eq(a3180, a3181)
                    } else {
                        false
                    } {
                        let a3190 = match v3.clone() {
                            Adt2::C0(_, h3182, _, _, _, _, _, _, _, _, _) => {
                                h3182
                            }
                        };
                        let a3191 = {
                            let a3188 = {
                                let a3185 = match v2.clone() {
                                    Adt2::C0(_, h3183, _, _, _, _, _, _, _, _, _) => {
                                        h3183
                                    }
                                };
                                let a3186 = match v1.clone() {
                                    Adt1::C0(h3184, _, _, _, _, _, _, _, _, _, _, _) => {
                                        h3184
                                    }
                                };
                                nat_add(a3185, a3186)?
                            };
                            let a3189 = match v1.clone() {
                                Adt1::C0(_, h3187, _, _, _, _, _, _, _, _, _, _) => {
                                    h3187
                                }
                            };
                            nat_add(a3188, a3189)?
                        };
                        nat_eq(a3190, a3191)
                    } else {
                        false
                    }
                } else {
                    false
                } {
                    Ok(Adt10::C4(f18(v3.clone(), 1u64)?, 4u64, f22(v2, 1u64), f22(v3, 1u64), Adt4::C4(match v1 {
                        Adt1::C0(h3192, _, _, _, _, _, _, _, _, _, _, _) => {
                            h3192
                        }
                    })))
                } else {
                    Ok(Adt10::C1)
                }
            } else {
                let a3193 = v0;
                let a3194 = 63u64;
                if nat_eq(a3193, a3194) {
                    if if if if if f205(match v2.clone() {
                        Adt2::C0(_, h3195, _, _, _, _, _, _, _, _, _) => {
                            h3195
                        }
                    }, match v1.clone() {
                        Adt1::C0(h3196, _, _, _, _, _, _, _, _, _, _, _) => {
                            h3196
                        }
                    })? {
                        f205(match v2.clone() {
                            Adt2::C0(_, _, h3197, _, _, _, _, _, _, _, _) => {
                                h3197
                            }
                        }, match v1.clone() {
                            Adt1::C0(_, h3198, _, _, _, _, _, _, _, _, _, _) => {
                                h3198
                            }
                        })?
                    } else {
                        false
                    } {
                        f205(match v2.clone() {
                            Adt2::C0(_, _, _, h3199, _, _, _, _, _, _, _) => {
                                h3199
                            }
                        }, match v1.clone() {
                            Adt1::C0(_, _, h3200, _, _, _, _, _, _, _, _, _) => {
                                h3200
                            }
                        })?
                    } else {
                        false
                    } {
                        f205(match v2.clone() {
                            Adt2::C0(_, _, _, _, h3201, _, _, _, _, _, _) => {
                                h3201
                            }
                        }, match v1.clone() {
                            Adt1::C0(_, _, _, h3202, _, _, _, _, _, _, _, _) => {
                                h3202
                            }
                        })?
                    } else {
                        false
                    } {
                        let a3205 = match v2.clone() {
                            Adt2::C0(_, h3203, _, _, _, _, _, _, _, _, _) => {
                                h3203
                            }
                        };
                        let a3206 = match v3.clone() {
                            Adt2::C0(_, h3204, _, _, _, _, _, _, _, _, _) => {
                                h3204
                            }
                        };
                        if if if nat_eq(a3205, a3206) {
                            let a3209 = match v2.clone() {
                                Adt2::C0(_, _, h3207, _, _, _, _, _, _, _, _) => {
                                    h3207
                                }
                            };
                            let a3210 = match v3.clone() {
                                Adt2::C0(_, _, h3208, _, _, _, _, _, _, _, _) => {
                                    h3208
                                }
                            };
                            nat_eq(a3209, a3210)
                        } else {
                            false
                        } {
                            let a3213 = match v2.clone() {
                                Adt2::C0(_, _, _, h3211, _, _, _, _, _, _, _) => {
                                    h3211
                                }
                            };
                            let a3214 = match v3.clone() {
                                Adt2::C0(_, _, _, h3212, _, _, _, _, _, _, _) => {
                                    h3212
                                }
                            };
                            nat_eq(a3213, a3214)
                        } else {
                            false
                        } {
                            let a3217 = match v2.clone() {
                                Adt2::C0(_, _, _, _, h3215, _, _, _, _, _, _) => {
                                    h3215
                                }
                            };
                            let a3218 = match v3.clone() {
                                Adt2::C0(_, _, _, _, h3216, _, _, _, _, _, _) => {
                                    h3216
                                }
                            };
                            nat_eq(a3217, a3218)
                        } else {
                            false
                        }
                    } else {
                        false
                    } {
                        Ok(Adt10::C4(f18(v3.clone(), 1u64)?, 4u64, f22(v2, 1u64), f22(v3, 1u64), Adt4::C5(match v1.clone() {
                            Adt1::C0(h3219, _, _, _, _, _, _, _, _, _, _, _) => {
                                h3219
                            }
                        }, match v1.clone() {
                            Adt1::C0(_, h3220, _, _, _, _, _, _, _, _, _, _) => {
                                h3220
                            }
                        }, match v1.clone() {
                            Adt1::C0(_, _, h3221, _, _, _, _, _, _, _, _, _) => {
                                h3221
                            }
                        }, match v1 {
                            Adt1::C0(_, _, _, h3222, _, _, _, _, _, _, _, _) => {
                                h3222
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

pub fn f110(v0: u64, v1: Adt1, v2: Adt2, v3: Option<Adt2>, v4: Adt2) -> R<Adt10> {
    let m3223 = v3;
    match m3223 {
        None => {
            Ok(Adt10::C1)
        }
        Some(v5) => {
            let v6: bool = {
                let a3224 = v0;
                let a3225 = 4u64;
                nat_eq(a3224, a3225)
            };
            let v7: bool = {
                let a3227 = 0u64;
                let a3228 = match v1.clone() {
                    Adt1::C0(_, _, _, _, h3226, _, _, _, _, _, _, _) => {
                        h3226
                    }
                };
                nat_lt(a3227, a3228)
            };
            let v8: u64 = f18(v5.clone(), 1u64)?;
            let v9: Adt3 = Adt3::C0(match v5.clone() {
                Adt2::C0(_, h3229, _, _, _, _, _, _, _, _, _) => {
                    h3229
                }
            }, match v5.clone() {
                Adt2::C0(_, _, h3230, _, _, _, _, _, _, _, _) => {
                    h3230
                }
            }, match v5.clone() {
                Adt2::C0(_, _, _, h3231, _, _, _, _, _, _, _) => {
                    h3231
                }
            }, match v5.clone() {
                Adt2::C0(_, _, _, _, h3232, _, _, _, _, _, _) => {
                    h3232
                }
            }, 4u64, match v1.clone() {
                Adt1::C0(h3233, _, _, _, _, _, _, _, _, _, _, _) => {
                    h3233
                }
            }, match v1.clone() {
                Adt1::C0(_, h3234, _, _, _, _, _, _, _, _, _, _) => {
                    h3234
                }
            }, match v1.clone() {
                Adt1::C0(_, _, h3235, _, _, _, _, _, _, _, _, _) => {
                    h3235
                }
            });
            if if if if if if if if if if if if v6 {
                let a3237 = match v2.clone() {
                    Adt2::C0(h3236, _, _, _, _, _, _, _, _, _, _) => {
                        h3236
                    }
                };
                let a3238 = 0u64;
                if nat_eq(a3237, a3238) {
                    let a3240 = match v5.clone() {
                        Adt2::C0(h3239, _, _, _, _, _, _, _, _, _, _) => {
                            h3239
                        }
                    };
                    let a3241 = 0u64;
                    nat_eq(a3240, a3241)
                } else {
                    false
                }
            } else {
                let a3243 = match v2.clone() {
                    Adt2::C0(h3242, _, _, _, _, _, _, _, _, _, _) => {
                        h3242
                    }
                };
                let a3244 = 0u64;
                if if nat_eq(a3243, a3244) {
                    true
                } else {
                    let a3246 = match v2.clone() {
                        Adt2::C0(h3245, _, _, _, _, _, _, _, _, _, _) => {
                            h3245
                        }
                    };
                    let a3247 = 26u64;
                    nat_eq(a3246, a3247)
                } {
                    let a3249 = match v5.clone() {
                        Adt2::C0(h3248, _, _, _, _, _, _, _, _, _, _) => {
                            h3248
                        }
                    };
                    let a3250 = 0u64;
                    if nat_eq(a3249, a3250) {
                        true
                    } else {
                        let a3252 = match v5.clone() {
                            Adt2::C0(h3251, _, _, _, _, _, _, _, _, _, _) => {
                                h3251
                            }
                        };
                        let a3253 = 26u64;
                        nat_eq(a3252, a3253)
                    }
                } else {
                    false
                }
            } {
                let a3256 = match v2.clone() {
                    Adt2::C0(h3254, _, _, _, _, _, _, _, _, _, _) => {
                        h3254
                    }
                };
                let a3257 = match v4.clone() {
                    Adt2::C0(h3255, _, _, _, _, _, _, _, _, _, _) => {
                        h3255
                    }
                };
                nat_eq(a3256, a3257)
            } else {
                false
            } {
                f20(v2.clone(), v4.clone())
            } else {
                false
            } {
                f28(v2.clone())?
            } else {
                false
            } {
                f28(v4.clone())?
            } else {
                false
            } {
                let a3259 = match v5.clone() {
                    Adt2::C0(_, _, _, _, _, h3258, _, _, _, _, _) => {
                        h3258
                    }
                };
                let a3260 = 4u64;
                nat_eq(a3259, a3260)
            } else {
                false
            } {
                let a3261 = v8;
                let a3262 = f18(v2.clone(), 1u64)?;
                nat_le(a3261, a3262)
            } else {
                false
            } {
                let a3264 = match v1.clone() {
                    Adt1::C0(_, _, _, h3263, _, _, _, _, _, _, _, _) => {
                        h3263
                    }
                };
                let a3265 = 1073741824u64;
                nat_lt(a3264, a3265)
            } else {
                false
            } {
                let a3267 = match v1.clone() {
                    Adt1::C0(h3266, _, _, _, _, _, _, _, _, _, _, _) => {
                        h3266
                    }
                };
                let a3268 = 4294967295u64;
                if if if if nat_le(a3267, a3268) {
                    let a3270 = match v1.clone() {
                        Adt1::C0(_, h3269, _, _, _, _, _, _, _, _, _, _) => {
                            h3269
                        }
                    };
                    let a3271 = 4294967295u64;
                    nat_le(a3270, a3271)
                } else {
                    false
                } {
                    let a3273 = match v1.clone() {
                        Adt1::C0(_, _, h3272, _, _, _, _, _, _, _, _, _) => {
                            h3272
                        }
                    };
                    let a3274 = 4294967295u64;
                    nat_le(a3273, a3274)
                } else {
                    false
                } {
                    let a3276 = match v1.clone() {
                        Adt1::C0(_, _, _, h3275, _, _, _, _, _, _, _, _) => {
                            h3275
                        }
                    };
                    let a3277 = 4294967295u64;
                    nat_le(a3276, a3277)
                } else {
                    false
                } {
                    let a3279 = match v1.clone() {
                        Adt1::C0(_, _, _, _, h3278, _, _, _, _, _, _, _) => {
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
                if v7 {
                    let a3283 = match v2.clone() {
                        Adt2::C0(_, _, _, _, _, _, _, _, _, _, h3281) => {
                            h3281
                        }
                    };
                    let a3284 = match v4.clone() {
                        Adt2::C0(_, _, _, _, _, _, _, _, _, _, h3282) => {
                            h3282
                        }
                    };
                    nat_eq(a3283, a3284)
                } else {
                    true
                }
            } else {
                false
            } {
                f206(v9.clone(), match v1.clone() {
                    Adt1::C0(_, _, _, h3285, _, _, _, _, _, _, _, _) => {
                        h3285
                    }
                }, match v4 {
                    Adt2::C0(_, _, _, _, _, _, _, _, _, h3286, _) => {
                        h3286
                    }
                }, v8, v6)?
            } else {
                false
            } {
                let v10: u64 = if v7 {
                    0u64
                } else {
                    f18(v2, 1u64)?
                };
                Ok(Adt10::C4({
                    let a3287 = v10;
                    let a3288 = v8;
                    nat_add(a3287, a3288)?
                }, 4u64, f22(v5, 1u64), v9, Adt4::C6(v10, match v1 {
                    Adt1::C0(_, _, _, h3289, _, _, _, _, _, _, _, _) => {
                        h3289
                    }
                }, v6)))
            } else {
                Ok(Adt10::C1)
            }
        }
    }
}

pub fn f111(v0: Adt2, v1: Adt2) -> R<Adt10> {
    let a3291 = match v0.clone() {
        Adt2::C0(h3290, _, _, _, _, _, _, _, _, _, _) => {
            h3290
        }
    };
    let a3292 = 0u64;
    if if if nat_eq(a3291, a3292) {
        let a3294 = match v1.clone() {
            Adt2::C0(h3293, _, _, _, _, _, _, _, _, _, _) => {
                h3293
            }
        };
        let a3295 = 0u64;
        nat_eq(a3294, a3295)
    } else {
        false
    } {
        f202(match v0.clone() {
            Adt2::C0(_, h3296, _, _, _, _, _, _, _, _, _) => {
                h3296
            }
        }, match v0.clone() {
            Adt2::C0(_, _, h3297, _, _, _, _, _, _, _, _) => {
                h3297
            }
        }, match v0.clone() {
            Adt2::C0(_, _, _, h3298, _, _, _, _, _, _, _) => {
                h3298
            }
        }, match v0.clone() {
            Adt2::C0(_, _, _, _, h3299, _, _, _, _, _, _) => {
                h3299
            }
        }, match v1.clone() {
            Adt2::C0(_, h3300, _, _, _, _, _, _, _, _, _) => {
                h3300
            }
        }, match v1.clone() {
            Adt2::C0(_, _, h3301, _, _, _, _, _, _, _, _) => {
                h3301
            }
        }, match v1.clone() {
            Adt2::C0(_, _, _, h3302, _, _, _, _, _, _, _) => {
                h3302
            }
        }, match v1.clone() {
            Adt2::C0(_, _, _, _, h3303, _, _, _, _, _, _) => {
                h3303
            }
        })
    } else {
        false
    } {
        let v2: u64 = f18(v1.clone(), 1u64)?;
        Ok(Adt10::C4({
            let a3304 = v2;
            let a3305 = f18(v0.clone(), 1u64)?;
            nat_add(a3304, a3305)?
        }, 4u64, f22(v0, 1u64), f22(v1, 1u64), Adt4::C7(v2)))
    } else {
        Ok(Adt10::C1)
    }
}

pub fn f112(v0: u64, v1: Adt2, v2: Adt2) -> R<Adt10> {
    let a3307 = match v1.clone() {
        Adt2::C0(h3306, _, _, _, _, _, _, _, _, _, _) => {
            h3306
        }
    };
    let a3308 = 0u64;
    if if if nat_eq(a3307, a3308) {
        true
    } else {
        let a3310 = match v1.clone() {
            Adt2::C0(h3309, _, _, _, _, _, _, _, _, _, _) => {
                h3309
            }
        };
        let a3311 = 1u64;
        nat_eq(a3310, a3311)
    } {
        true
    } else {
        let a3313 = match v1.clone() {
            Adt2::C0(h3312, _, _, _, _, _, _, _, _, _, _) => {
                h3312
            }
        };
        let a3314 = 30u64;
        nat_eq(a3313, a3314)
    } {
        let a3317 = match v1.clone() {
            Adt2::C0(h3315, _, _, _, _, _, _, _, _, _, _) => {
                h3315
            }
        };
        let a3318 = match v2.clone() {
            Adt2::C0(h3316, _, _, _, _, _, _, _, _, _, _) => {
                h3316
            }
        };
        if if if if if nat_eq(a3317, a3318) {
            let a3321 = match v1.clone() {
                Adt2::C0(_, h3319, _, _, _, _, _, _, _, _, _) => {
                    h3319
                }
            };
            let a3322 = match v2.clone() {
                Adt2::C0(_, h3320, _, _, _, _, _, _, _, _, _) => {
                    h3320
                }
            };
            nat_eq(a3321, a3322)
        } else {
            false
        } {
            let a3325 = match v1.clone() {
                Adt2::C0(_, _, _, h3323, _, _, _, _, _, _, _) => {
                    h3323
                }
            };
            let a3326 = match v2.clone() {
                Adt2::C0(_, _, _, h3324, _, _, _, _, _, _, _) => {
                    h3324
                }
            };
            nat_eq(a3325, a3326)
        } else {
            false
        } {
            let a3329 = match v1.clone() {
                Adt2::C0(_, _, _, _, h3327, _, _, _, _, _, _) => {
                    h3327
                }
            };
            let a3330 = match v2.clone() {
                Adt2::C0(_, _, _, _, h3328, _, _, _, _, _, _) => {
                    h3328
                }
            };
            nat_eq(a3329, a3330)
        } else {
            false
        } {
            let a3331 = v0;
            let a3332 = 4u64;
            if nat_eq(a3331, a3332) {
                let a3334 = match v1.clone() {
                    Adt2::C0(_, _, h3333, _, _, _, _, _, _, _, _) => {
                        h3333
                    }
                };
                let a3335 = 1u64;
                if nat_eq(a3334, a3335) {
                    let a3338 = match v1.clone() {
                        Adt2::C0(_, h3336, _, _, _, _, _, _, _, _, _) => {
                            h3336
                        }
                    };
                    let a3339 = match v2.clone() {
                        Adt2::C0(_, _, h3337, _, _, _, _, _, _, _, _) => {
                            h3337
                        }
                    };
                    nat_eq(a3338, a3339)
                } else {
                    false
                }
            } else {
                let a3340 = v0;
                let a3341 = 4u64;
                if if nat_lt(a3340, a3341) {
                    let a3344 = match v1.clone() {
                        Adt2::C0(_, h3342, _, _, _, _, _, _, _, _, _) => {
                            h3342
                        }
                    };
                    let a3345 = match v1.clone() {
                        Adt2::C0(_, _, h3343, _, _, _, _, _, _, _, _) => {
                            h3343
                        }
                    };
                    nat_eq(a3344, a3345)
                } else {
                    false
                } {
                    let a3348 = match v1.clone() {
                        Adt2::C0(_, _, h3346, _, _, _, _, _, _, _, _) => {
                            h3346
                        }
                    };
                    let a3349 = match v2.clone() {
                        Adt2::C0(_, _, h3347, _, _, _, _, _, _, _, _) => {
                            h3347
                        }
                    };
                    nat_eq(a3348, a3349)
                } else {
                    false
                }
            }
        } else {
            false
        } {
            let m3351 = f192(match v1.clone() {
                Adt2::C0(h3350, _, _, _, _, _, _, _, _, _, _) => {
                    h3350
                }
            });
            match m3351 {
                Adt18::C0 => {
                    Ok(Adt10::C0)
                }
                Adt18::C1(v3) => {
                    Ok(Adt10::C4(f18(v2.clone(), 1u64)?, match v3 {
                        Adt19::C0(_, _, h3352) => {
                            h3352
                        }
                    }, f22(v1, 1u64), f22(v2, 1u64), Adt4::C8(v0)))
                }
            }
        } else {
            Ok(Adt10::C1)
        }
    } else {
        Ok(Adt10::C0)
    }
}

pub fn f113(v0: u64, v1: Adt2, v2: Adt2) -> R<Adt10> {
    if if if f207(match v1.clone() {
        Adt2::C0(h3353, _, _, _, _, _, _, _, _, _, _) => {
            h3353
        }
    }, v0) {
        let a3356 = match v1.clone() {
            Adt2::C0(h3354, _, _, _, _, _, _, _, _, _, _) => {
                h3354
            }
        };
        let a3357 = match v2.clone() {
            Adt2::C0(h3355, _, _, _, _, _, _, _, _, _, _) => {
                h3355
            }
        };
        nat_eq(a3356, a3357)
    } else {
        false
    } {
        let a3360 = match v1.clone() {
            Adt2::C0(_, h3358, _, _, _, _, _, _, _, _, _) => {
                h3358
            }
        };
        let a3361 = match v2.clone() {
            Adt2::C0(_, h3359, _, _, _, _, _, _, _, _, _) => {
                h3359
            }
        };
        if if if nat_eq(a3360, a3361) {
            let a3364 = match v1.clone() {
                Adt2::C0(_, _, h3362, _, _, _, _, _, _, _, _) => {
                    h3362
                }
            };
            let a3365 = match v2.clone() {
                Adt2::C0(_, _, h3363, _, _, _, _, _, _, _, _) => {
                    h3363
                }
            };
            nat_eq(a3364, a3365)
        } else {
            false
        } {
            let a3368 = match v1.clone() {
                Adt2::C0(_, _, _, h3366, _, _, _, _, _, _, _) => {
                    h3366
                }
            };
            let a3369 = match v2.clone() {
                Adt2::C0(_, _, _, h3367, _, _, _, _, _, _, _) => {
                    h3367
                }
            };
            nat_eq(a3368, a3369)
        } else {
            false
        } {
            let a3372 = match v1.clone() {
                Adt2::C0(_, _, _, _, h3370, _, _, _, _, _, _) => {
                    h3370
                }
            };
            let a3373 = match v2.clone() {
                Adt2::C0(_, _, _, _, h3371, _, _, _, _, _, _) => {
                    h3371
                }
            };
            nat_eq(a3372, a3373)
        } else {
            false
        }
    } else {
        false
    } {
        let m3375 = f192(match v1.clone() {
            Adt2::C0(h3374, _, _, _, _, _, _, _, _, _, _) => {
                h3374
            }
        });
        match m3375 {
            Adt18::C0 => {
                Ok(Adt10::C0)
            }
            Adt18::C1(v3) => {
                Ok(Adt10::C4(f18(v2.clone(), 1u64)?, match v3 {
                    Adt19::C0(_, _, h3376) => {
                        h3376
                    }
                }, f22(v1.clone(), 1u64), f22(v2, 1u64), Adt4::C9(match v1 {
                    Adt2::C0(h3377, _, _, _, _, _, _, _, _, _, _) => {
                        h3377
                    }
                }, v0)))
            }
        }
    } else {
        Ok(Adt10::C1)
    }
}

pub fn f114(v0: bool, v1: Adt1, v2: Adt2, v3: Adt2) -> R<Adt10> {
    let v4: u64 = if v0 {
        match v1.clone() {
            Adt1::C0(h3378, _, _, _, _, _, _, _, _, _, _, _) => {
                h3378
            }
        }
    } else {
        match v1.clone() {
            Adt1::C0(_, _, h3379, _, _, _, _, _, _, _, _, _) => {
                h3379
            }
        }
    };
    let a3380 = 0u64;
    let a3381 = v4;
    if if nat_lt(a3380, a3381) {
        let a3382 = v4;
        let a3383 = 2147483648u64;
        nat_lt(a3382, a3383)
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
        let v7: u64 = f208(match v5.clone() {
            Adt2::C0(_, _, h3384, _, _, _, _, _, _, _, _) => {
                h3384
            }
        }, v4)?;
        let v8: u64 = f208(match v5.clone() {
            Adt2::C0(_, _, _, h3385, _, _, _, _, _, _, _) => {
                h3385
            }
        }, v4)?;
        let a3387 = match v5.clone() {
            Adt2::C0(h3386, _, _, _, _, _, _, _, _, _, _) => {
                h3386
            }
        };
        let a3388 = 0u64;
        if if if if if nat_eq(a3387, a3388) {
            let a3390 = match v6.clone() {
                Adt2::C0(h3389, _, _, _, _, _, _, _, _, _, _) => {
                    h3389
                }
            };
            let a3391 = 0u64;
            nat_eq(a3390, a3391)
        } else {
            false
        } {
            let a3393 = match v5.clone() {
                Adt2::C0(_, _, _, _, h3392, _, _, _, _, _, _) => {
                    h3392
                }
            };
            let a3394 = 1u64;
            nat_eq(a3393, a3394)
        } else {
            false
        } {
            let a3397 = match v5.clone() {
                Adt2::C0(_, h3395, _, _, _, _, _, _, _, _, _) => {
                    h3395
                }
            };
            let a3398 = match v6.clone() {
                Adt2::C0(_, h3396, _, _, _, _, _, _, _, _, _) => {
                    h3396
                }
            };
            nat_eq(a3397, a3398)
        } else {
            false
        } {
            if v0 {
                let a3400 = match v5.clone() {
                    Adt2::C0(_, _, h3399, _, _, _, _, _, _, _, _) => {
                        h3399
                    }
                };
                let a3401 = 2147483648u64;
                if if if nat_lt(a3400, a3401) {
                    let a3403 = match v5.clone() {
                        Adt2::C0(_, _, _, h3402, _, _, _, _, _, _, _) => {
                            h3402
                        }
                    };
                    let a3404 = 2147483648u64;
                    nat_lt(a3403, a3404)
                } else {
                    false
                } {
                    let a3407 = {
                        let a3405 = v7;
                        let a3406 = v8;
                        nat_mul(a3405, a3406)?
                    };
                    let a3408 = 2147483648u64;
                    nat_le(a3407, a3408)
                } else {
                    false
                } {
                    f209(v5, v6, v4, v7)?
                } else {
                    false
                }
            } else {
                let a3410 = match v1.clone() {
                    Adt1::C0(h3409, _, _, _, _, _, _, _, _, _, _, _) => {
                        h3409
                    }
                };
                let a3411 = v7;
                if if if if nat_eq(a3410, a3411) {
                    let a3413 = match v1 {
                        Adt1::C0(_, h3412, _, _, _, _, _, _, _, _, _, _) => {
                            h3412
                        }
                    };
                    let a3414 = v8;
                    nat_eq(a3413, a3414)
                } else {
                    false
                } {
                    f210(v7, v8, v4)?
                } else {
                    false
                } {
                    let a3416 = match v6.clone() {
                        Adt2::C0(_, _, h3415, _, _, _, _, _, _, _, _) => {
                            h3415
                        }
                    };
                    let a3417 = v4;
                    if nat_eq(a3416, a3417) {
                        let a3419 = match v6.clone() {
                            Adt2::C0(_, _, _, h3418, _, _, _, _, _, _, _) => {
                                h3418
                            }
                        };
                        let a3420 = v4;
                        nat_eq(a3419, a3420)
                    } else {
                        false
                    }
                } else {
                    false
                } {
                    let a3424 = match v6 {
                        Adt2::C0(_, _, _, _, h3421, _, _, _, _, _, _) => {
                            h3421
                        }
                    };
                    let a3425 = {
                        let a3422 = v7;
                        let a3423 = v8;
                        nat_mul(a3422, a3423)?
                    };
                    nat_eq(a3424, a3425)
                } else {
                    false
                }
            }
        } else {
            false
        } {
            Ok(Adt10::C4(f18(v3.clone(), 1u64)?, 4u64, f22(v2, 1u64), f22(v3, 1u64), Adt4::C12(v0, v4, v7)))
        } else {
            Ok(Adt10::C1)
        }
    } else {
        Ok(Adt10::C1)
    }
}

pub fn f115(v0: u64, v1: Adt1, v2: Adt2, v3: Adt2) -> R<Adt10> {
    let a3427 = match v2.clone() {
        Adt2::C0(h3426, _, _, _, _, _, _, _, _, _, _) => {
            h3426
        }
    };
    let a3428 = 0u64;
    if if if if if nat_eq(a3427, a3428) {
        true
    } else {
        let a3430 = match v2.clone() {
            Adt2::C0(h3429, _, _, _, _, _, _, _, _, _, _) => {
                h3429
            }
        };
        let a3431 = 1u64;
        nat_eq(a3430, a3431)
    } {
        true
    } else {
        let a3433 = match v2.clone() {
            Adt2::C0(h3432, _, _, _, _, _, _, _, _, _, _) => {
                h3432
            }
        };
        let a3434 = 30u64;
        nat_eq(a3433, a3434)
    } {
        let a3437 = match v2.clone() {
            Adt2::C0(h3435, _, _, _, _, _, _, _, _, _, _) => {
                h3435
            }
        };
        let a3438 = match v3.clone() {
            Adt2::C0(h3436, _, _, _, _, _, _, _, _, _, _) => {
                h3436
            }
        };
        nat_eq(a3437, a3438)
    } else {
        false
    } {
        if if f20(v2.clone(), v3.clone()) {
            let a3440 = match v1.clone() {
                Adt1::C0(h3439, _, _, _, _, _, _, _, _, _, _, _) => {
                    h3439
                }
            };
            let a3441 = 4294967296u64;
            nat_lt(a3440, a3441)
        } else {
            false
        } {
            let a3443 = match v1.clone() {
                Adt1::C0(_, h3442, _, _, _, _, _, _, _, _, _, _) => {
                    h3442
                }
            };
            let a3444 = 4294967296u64;
            nat_lt(a3443, a3444)
        } else {
            false
        }
    } else {
        false
    } {
        Ok(Adt10::C4(f18(v3.clone(), 1u64)?, {
            let a3446 = match v2.clone() {
                Adt2::C0(h3445, _, _, _, _, _, _, _, _, _, _) => {
                    h3445
                }
            };
            let a3447 = 0u64;
            if nat_eq(a3446, a3447) {
                4u64
            } else {
                2u64
            }
        }, f22(v2.clone(), 1u64), f22(v3, 1u64), Adt4::C13({
            let a3448 = v0;
            let a3449 = 31u64;
            if nat_eq(a3448, a3449) {
                0u64
            } else {
                let a3450 = v0;
                let a3451 = 47u64;
                if nat_eq(a3450, a3451) {
                    1u64
                } else {
                    2u64
                }
            }
        }, match v2 {
            Adt2::C0(h3452, _, _, _, _, _, _, _, _, _, _) => {
                h3452
            }
        }, match v1.clone() {
            Adt1::C0(h3453, _, _, _, _, _, _, _, _, _, _, _) => {
                h3453
            }
        }, match v1 {
            Adt1::C0(_, h3454, _, _, _, _, _, _, _, _, _, _) => {
                h3454
            }
        })))
    } else {
        Ok(Adt10::C1)
    }
}

pub fn f116(v0: Adt2, v1: Adt2) -> R<Adt10> {
    let a3456 = match v0.clone() {
        Adt2::C0(h3455, _, _, _, _, _, _, _, _, _, _) => {
            h3455
        }
    };
    let a3457 = 0u64;
    if if if nat_eq(a3456, a3457) {
        let a3459 = match v1.clone() {
            Adt2::C0(h3458, _, _, _, _, _, _, _, _, _, _) => {
                h3458
            }
        };
        let a3460 = 0u64;
        nat_eq(a3459, a3460)
    } else {
        false
    } {
        f20(v0.clone(), v1.clone())
    } else {
        false
    } {
        Ok(Adt10::C4(f18(v1.clone(), 1u64)?, 4u64, f22(v0, 1u64), f22(v1, 1u64), Adt4::C15))
    } else {
        Ok(Adt10::C1)
    }
}

pub fn f117(v0: Adt2, v1: Adt2) -> R<Adt10> {
    let a3462 = match v0.clone() {
        Adt2::C0(h3461, _, _, _, _, _, _, _, _, _, _) => {
            h3461
        }
    };
    let a3463 = 0u64;
    if if nat_eq(a3462, a3463) {
        let a3465 = match v1.clone() {
            Adt2::C0(h3464, _, _, _, _, _, _, _, _, _, _) => {
                h3464
            }
        };
        let a3466 = 26u64;
        nat_eq(a3465, a3466)
    } else {
        false
    } {
        let a3468 = match v0.clone() {
            Adt2::C0(_, _, _, h3467, _, _, _, _, _, _, _) => {
                h3467
            }
        };
        let a3469 = 1u64;
        if if if if if if if nat_eq(a3468, a3469) {
            let a3471 = match v0.clone() {
                Adt2::C0(_, _, _, _, h3470, _, _, _, _, _, _) => {
                    h3470
                }
            };
            let a3472 = 1u64;
            nat_eq(a3471, a3472)
        } else {
            false
        } {
            let a3474 = match v0.clone() {
                Adt2::C0(_, h3473, _, _, _, _, _, _, _, _, _) => {
                    h3473
                }
            };
            let a3475 = 2147483647u64;
            nat_le(a3474, a3475)
        } else {
            false
        } {
            let a3478 = match v1.clone() {
                Adt2::C0(_, h3476, _, _, _, _, _, _, _, _, _) => {
                    h3476
                }
            };
            let a3479 = match v0.clone() {
                Adt2::C0(_, _, h3477, _, _, _, _, _, _, _, _) => {
                    h3477
                }
            };
            nat_eq(a3478, a3479)
        } else {
            false
        } {
            let a3481 = match v1.clone() {
                Adt2::C0(_, _, h3480, _, _, _, _, _, _, _, _) => {
                    h3480
                }
            };
            let a3482 = 1u64;
            nat_eq(a3481, a3482)
        } else {
            false
        } {
            let a3484 = match v1.clone() {
                Adt2::C0(_, _, _, h3483, _, _, _, _, _, _, _) => {
                    h3483
                }
            };
            let a3485 = 1u64;
            nat_eq(a3484, a3485)
        } else {
            false
        } {
            let a3487 = match v1.clone() {
                Adt2::C0(_, _, _, _, h3486, _, _, _, _, _, _) => {
                    h3486
                }
            };
            let a3488 = 1u64;
            nat_eq(a3487, a3488)
        } else {
            false
        } {
            Ok(Adt10::C4({
                let a3491 = match v0.clone() {
                    Adt2::C0(_, _, h3489, _, _, _, _, _, _, _, _) => {
                        h3489
                    }
                };
                let a3492 = f152(match v0.clone() {
                    Adt2::C0(_, h3490, _, _, _, _, _, _, _, _, _) => {
                        h3490
                    }
                })?;
                nat_mul(a3491, a3492)?
            }, 4u64, f22(v0, 1u64), f22(v1, 1u64), Adt4::C17))
        } else {
            Ok(Adt10::C1)
        }
    } else {
        Ok(Adt10::C0)
    }
}

pub fn f118(v0: Adt2, v1: Option<Adt2>, v2: Adt2) -> R<Adt10> {
    let m3493 = v1;
    match m3493 {
        None => {
            Ok(Adt10::C1)
        }
        Some(v3) => {
            let v4: u64 = f157(f22(v0.clone(), 1u64))?;
            let a3495 = match v0.clone() {
                Adt2::C0(h3494, _, _, _, _, _, _, _, _, _, _) => {
                    h3494
                }
            };
            let a3496 = 26u64;
            if if if nat_eq(a3495, a3496) {
                let a3498 = match v3.clone() {
                    Adt2::C0(h3497, _, _, _, _, _, _, _, _, _, _) => {
                        h3497
                    }
                };
                let a3499 = 26u64;
                nat_eq(a3498, a3499)
            } else {
                false
            } {
                let a3501 = match v2.clone() {
                    Adt2::C0(h3500, _, _, _, _, _, _, _, _, _, _) => {
                        h3500
                    }
                };
                let a3502 = 27u64;
                nat_eq(a3501, a3502)
            } else {
                false
            } {
                if if if if if if f20(v0.clone(), v3.clone()) {
                    let a3504 = match v2.clone() {
                        Adt2::C0(_, h3503, _, _, _, _, _, _, _, _, _) => {
                            h3503
                        }
                    };
                    let a3505 = 1u64;
                    nat_eq(a3504, a3505)
                } else {
                    false
                } {
                    let a3507 = match v2.clone() {
                        Adt2::C0(_, _, h3506, _, _, _, _, _, _, _, _) => {
                            h3506
                        }
                    };
                    let a3508 = 1u64;
                    nat_eq(a3507, a3508)
                } else {
                    false
                } {
                    let a3510 = match v2.clone() {
                        Adt2::C0(_, _, _, h3509, _, _, _, _, _, _, _) => {
                            h3509
                        }
                    };
                    let a3511 = 1u64;
                    nat_eq(a3510, a3511)
                } else {
                    false
                } {
                    let a3513 = match v2.clone() {
                        Adt2::C0(_, _, _, _, h3512, _, _, _, _, _, _) => {
                            h3512
                        }
                    };
                    let a3514 = 1u64;
                    nat_eq(a3513, a3514)
                } else {
                    false
                } {
                    let a3515 = v4;
                    let a3516 = 9223372036854775808u64;
                    nat_lt(a3515, a3516)
                } else {
                    false
                } {
                    Ok(Adt10::C4({
                        let a3519 = 2u64;
                        let a3520 = {
                            let a3517 = v4;
                            let a3518 = 2u64;
                            nat_mul(a3517, a3518)?
                        };
                        nat_add(a3519, a3520)?
                    }, 8u64, f22(v0, 1u64), f22(v2, 1u64), Adt4::C18(v4, f22(v3, 1u64))))
                } else {
                    Ok(Adt10::C1)
                }
            } else {
                Ok(Adt10::C0)
            }
        }
    }
}

pub fn f119(v0: u64, v1: Adt2, v2: Adt2) -> R<Adt10> {
    let v3: u64 = match v1.clone() {
        Adt2::C0(h3521, _, _, _, _, _, _, _, _, _, _) => {
            h3521
        }
    };
    let a3522 = v3;
    let a3523 = 0u64;
    if if nat_eq(a3522, a3523) {
        true
    } else {
        let a3524 = v0;
        let a3525 = 13u64;
        if nat_eq(a3524, a3525) {
            let a3526 = v3;
            let a3527 = 1u64;
            if nat_eq(a3526, a3527) {
                true
            } else {
                let a3528 = v3;
                let a3529 = 30u64;
                nat_eq(a3528, a3529)
            }
        } else {
            false
        }
    } {
        let a3531 = v3;
        let a3532 = match v2.clone() {
            Adt2::C0(h3530, _, _, _, _, _, _, _, _, _, _) => {
                h3530
            }
        };
        if if nat_eq(a3531, a3532) {
            let a3533 = v0;
            let a3534 = 13u64;
            if nat_eq(a3533, a3534) {
                let a3536 = match v2.clone() {
                    Adt2::C0(_, h3535, _, _, _, _, _, _, _, _, _) => {
                        h3535
                    }
                };
                let a3537 = 1u64;
                if if if nat_eq(a3536, a3537) {
                    let a3539 = match v2.clone() {
                        Adt2::C0(_, _, h3538, _, _, _, _, _, _, _, _) => {
                            h3538
                        }
                    };
                    let a3540 = 1u64;
                    nat_eq(a3539, a3540)
                } else {
                    false
                } {
                    let a3542 = match v2.clone() {
                        Adt2::C0(_, _, _, h3541, _, _, _, _, _, _, _) => {
                            h3541
                        }
                    };
                    let a3543 = 1u64;
                    nat_eq(a3542, a3543)
                } else {
                    false
                } {
                    let a3545 = match v2.clone() {
                        Adt2::C0(_, _, _, _, h3544, _, _, _, _, _, _) => {
                            h3544
                        }
                    };
                    let a3546 = 1u64;
                    nat_eq(a3545, a3546)
                } else {
                    false
                }
            } else {
                let a3548 = match v2.clone() {
                    Adt2::C0(_, h3547, _, _, _, _, _, _, _, _, _) => {
                        h3547
                    }
                };
                let a3549 = 1u64;
                if if if nat_eq(a3548, a3549) {
                    let a3552 = match v1.clone() {
                        Adt2::C0(_, _, h3550, _, _, _, _, _, _, _, _) => {
                            h3550
                        }
                    };
                    let a3553 = match v2.clone() {
                        Adt2::C0(_, _, h3551, _, _, _, _, _, _, _, _) => {
                            h3551
                        }
                    };
                    nat_eq(a3552, a3553)
                } else {
                    false
                } {
                    let a3556 = match v1.clone() {
                        Adt2::C0(_, _, _, h3554, _, _, _, _, _, _, _) => {
                            h3554
                        }
                    };
                    let a3557 = match v2.clone() {
                        Adt2::C0(_, _, _, h3555, _, _, _, _, _, _, _) => {
                            h3555
                        }
                    };
                    nat_eq(a3556, a3557)
                } else {
                    false
                } {
                    let a3560 = match v1.clone() {
                        Adt2::C0(_, _, _, _, h3558, _, _, _, _, _, _) => {
                            h3558
                        }
                    };
                    let a3561 = match v2.clone() {
                        Adt2::C0(_, _, _, _, h3559, _, _, _, _, _, _) => {
                            h3559
                        }
                    };
                    nat_eq(a3560, a3561)
                } else {
                    false
                }
            }
        } else {
            false
        } {
            let v4: Adt3 = f22(v1, 1u64);
            Ok(Adt10::C4(f211(v4.clone(), v0, v3)?, {
                let a3562 = v3;
                let a3563 = 0u64;
                if nat_eq(a3562, a3563) {
                    4u64
                } else {
                    2u64
                }
            }, v4, f22(v2, 1u64), Adt4::C16(v0, v3)))
        } else {
            Ok(Adt10::C1)
        }
    } else {
        Ok(Adt10::C0)
    }
}

pub fn f120(v0: u64, v1: u64) -> R<Option<u64>> {
    let a3564 = v0;
    let a3565 = 89u64;
    if if nat_eq(a3564, a3565) {
        let a3566 = v1;
        let a3567 = 14u64;
        nat_eq(a3566, a3567)
    } else {
        false
    } {
        Ok(Some(23u64))
    } else {
        let a3568 = v0;
        let a3569 = 89u64;
        if if nat_eq(a3568, a3569) {
            let a3570 = v1;
            let a3571 = 15u64;
            nat_eq(a3570, a3571)
        } else {
            false
        } {
            Ok(Some(22u64))
        } else {
            let a3572 = v0;
            let a3573 = 89u64;
            if if nat_eq(a3572, a3573) {
                let a3574 = 8u64;
                let a3575 = v1;
                if nat_le(a3574, a3575) {
                    let a3576 = v1;
                    let a3577 = 9u64;
                    nat_le(a3576, a3577)
                } else {
                    false
                }
            } else {
                false
            } {
                Ok(Some({
                    let a3578 = v1;
                    let a3579 = 12u64;
                    nat_add(a3578, a3579)?
                }))
            } else {
                let a3580 = v0;
                let a3581 = 89u64;
                if if nat_eq(a3580, a3581) {
                    let a3582 = v1;
                    let a3583 = 4u64;
                    nat_eq(a3582, a3583)
                } else {
                    false
                } {
                    Ok(Some(19u64))
                } else {
                    let a3584 = 10u64;
                    let a3585 = v0;
                    if if nat_le(a3584, a3585) {
                        let a3586 = v0;
                        let a3587 = 12u64;
                        nat_le(a3586, a3587)
                    } else {
                        false
                    } {
                        Ok(Some({
                            let a3588 = v0;
                            let a3589 = 3u64;
                            nat_add(a3588, a3589)?
                        }))
                    } else {
                        let a3590 = v0;
                        let a3591 = 89u64;
                        if if nat_eq(a3590, a3591) {
                            let a3592 = v1;
                            let a3593 = 7u64;
                            nat_eq(a3592, a3593)
                        } else {
                            false
                        } {
                            Ok(Some(17u64))
                        } else {
                            let a3594 = v0;
                            let a3595 = 89u64;
                            if if nat_eq(a3594, a3595) {
                                let a3596 = v1;
                                let a3597 = 10u64;
                                nat_eq(a3596, a3597)
                            } else {
                                false
                            } {
                                Ok(Some(18u64))
                            } else {
                                let a3598 = v0;
                                let a3599 = 89u64;
                                if if nat_eq(a3598, a3599) {
                                    let a3600 = v1;
                                    let a3601 = 13u64;
                                    nat_eq(a3600, a3601)
                                } else {
                                    false
                                } {
                                    Ok(Some(16u64))
                                } else {
                                    let a3602 = v0;
                                    let a3603 = 8u64;
                                    if nat_eq(a3602, a3603) {
                                        Ok(Some(5u64))
                                    } else {
                                        let a3604 = v0;
                                        let a3605 = 9u64;
                                        if nat_eq(a3604, a3605) {
                                            Ok(Some(6u64))
                                        } else {
                                            let a3606 = v0;
                                            let a3607 = 89u64;
                                            if nat_eq(a3606, a3607) {
                                                let a3608 = v1;
                                                let a3609 = 0u64;
                                                if nat_eq(a3608, a3609) {
                                                    Ok(Some(0u64))
                                                } else {
                                                    let a3610 = v1;
                                                    let a3611 = 1u64;
                                                    if nat_eq(a3610, a3611) {
                                                        Ok(Some(1u64))
                                                    } else {
                                                        let a3612 = v1;
                                                        let a3613 = 2u64;
                                                        if nat_eq(a3612, a3613) {
                                                            Ok(Some(2u64))
                                                        } else {
                                                            let a3614 = v1;
                                                            let a3615 = 3u64;
                                                            if nat_eq(a3614, a3615) {
                                                                Ok(Some(3u64))
                                                            } else {
                                                                let a3616 = v1;
                                                                let a3617 = 6u64;
                                                                if nat_eq(a3616, a3617) {
                                                                    Ok(Some(4u64))
                                                                } else {
                                                                    let a3618 = v1;
                                                                    let a3619 = 12u64;
                                                                    if nat_eq(a3618, a3619) {
                                                                        Ok(Some(7u64))
                                                                    } else {
                                                                        let a3620 = v1;
                                                                        let a3621 = 11u64;
                                                                        if nat_eq(a3620, a3621) {
                                                                            Ok(Some(8u64))
                                                                        } else {
                                                                            let a3622 = v1;
                                                                            let a3623 = 18u64;
                                                                            if nat_eq(a3622, a3623) {
                                                                                Ok(Some(9u64))
                                                                            } else {
                                                                                let a3624 = v1;
                                                                                let a3625 = 19u64;
                                                                                if nat_eq(a3624, a3625) {
                                                                                    Ok(Some(10u64))
                                                                                } else {
                                                                                    let a3626 = v1;
                                                                                    let a3627 = 20u64;
                                                                                    if nat_eq(a3626, a3627) {
                                                                                        Ok(Some(11u64))
                                                                                    } else {
                                                                                        let a3628 = v1;
                                                                                        let a3629 = 21u64;
                                                                                        if nat_eq(a3628, a3629) {
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

pub fn f121(v0: u64, v1: u64, v2: Adt2, v3: Option<Adt2>, v4: Adt2) -> R<Adt10> {
    let m3630 = f212(v0, v1, v2.clone(), v3.clone(), v4.clone())?;
    match m3630 {
        Adt10::C0 => {
            Ok(Adt10::C0)
        }
        Adt10::C1 => {
            let a3631 = v0;
            let a3632 = 2u64;
            if if if if nat_eq(a3631, a3632) {
                true
            } else {
                let a3633 = v0;
                let a3634 = 5u64;
                nat_eq(a3633, a3634)
            } {
                true
            } else {
                let a3635 = v0;
                let a3636 = 6u64;
                nat_eq(a3635, a3636)
            } {
                true
            } else {
                let a3637 = v0;
                let a3638 = 7u64;
                nat_eq(a3637, a3638)
            } {
                Ok(Adt10::C5(67u64))
            } else {
                Ok(Adt10::C1)
            }
        }
        Adt10::C2(v7, v8, _, _, _, _, v13) => {
            if v8 {
                Ok(Adt10::C4(v13, 4u64, f22(v2.clone(), 1u64), f22(v4, 1u64), Adt4::C10(v7, v8, {
                    let m3639 = v3;
                    match m3639 {
                        None => {
                            f22(v2, 1u64)
                        }
                        Some(v14) => {
                            f22(v14, 1u64)
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

pub fn f122(v0: u64, v1: Adt2, v2: Adt2) -> R<Adt10> {
    let a3641 = match v1.clone() {
        Adt2::C0(h3640, _, _, _, _, _, _, _, _, _, _) => {
            h3640
        }
    };
    let a3642 = 0u64;
    if if if if if nat_eq(a3641, a3642) {
        true
    } else {
        let a3644 = match v1.clone() {
            Adt2::C0(h3643, _, _, _, _, _, _, _, _, _, _) => {
                h3643
            }
        };
        let a3645 = 1u64;
        nat_eq(a3644, a3645)
    } {
        true
    } else {
        let a3647 = match v1.clone() {
            Adt2::C0(h3646, _, _, _, _, _, _, _, _, _, _) => {
                h3646
            }
        };
        let a3648 = 30u64;
        nat_eq(a3647, a3648)
    } {
        let a3651 = match v1.clone() {
            Adt2::C0(h3649, _, _, _, _, _, _, _, _, _, _) => {
                h3649
            }
        };
        let a3652 = match v2.clone() {
            Adt2::C0(h3650, _, _, _, _, _, _, _, _, _, _) => {
                h3650
            }
        };
        if nat_eq(a3651, a3652) {
            true
        } else {
            let a3654 = match v2.clone() {
                Adt2::C0(h3653, _, _, _, _, _, _, _, _, _, _) => {
                    h3653
                }
            };
            let a3655 = 0u64;
            nat_eq(a3654, a3655)
        }
    } else {
        false
    } {
        f20(v1.clone(), v2.clone())
    } else {
        false
    } {
        Ok(Adt10::C4({
            let a3657 = f18(v2.clone(), 1u64)?;
            let a3658 = f213(v0, match v2.clone() {
                Adt2::C0(h3656, _, _, _, _, _, _, _, _, _, _) => {
                    h3656
                }
            });
            nat_mul(a3657, a3658)?
        }, {
            let a3660 = match v2.clone() {
                Adt2::C0(h3659, _, _, _, _, _, _, _, _, _, _) => {
                    h3659
                }
            };
            let a3661 = 0u64;
            if nat_eq(a3660, a3661) {
                4u64
            } else {
                2u64
            }
        }, f22(v1.clone(), 1u64), f22(v2.clone(), 1u64), Adt4::C14(v0, match v1 {
            Adt2::C0(h3662, _, _, _, _, _, _, _, _, _, _) => {
                h3662
            }
        }, match v2 {
            Adt2::C0(h3663, _, _, _, _, _, _, _, _, _, _) => {
                h3663
            }
        })))
    } else {
        Ok(Adt10::C1)
    }
}

pub fn f123(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64, v7: u64) -> R<u64> {
    let a3676 = {
        let a3672 = {
            let a3668 = {
                let a3664 = v0;
                let a3665 = v4;
                nat_mul(a3664, a3665)?
            };
            let a3669 = {
                let a3666 = v1;
                let a3667 = v5;
                nat_mul(a3666, a3667)?
            };
            nat_add(a3668, a3669)?
        };
        let a3673 = {
            let a3670 = v2;
            let a3671 = v6;
            nat_mul(a3670, a3671)?
        };
        nat_add(a3672, a3673)?
    };
    let a3677 = {
        let a3674 = v3;
        let a3675 = v7;
        nat_mul(a3674, a3675)?
    };
    nat_add(a3676, a3677)
}

pub fn f124(v0: u64) -> u64 {
    let a3678 = v0;
    let a3679 = 2147483648u64;
    if nat_lt(a3678, a3679) {
        0u64
    } else {
        let a3680 = 4294967296u64;
        let a3681 = v0;
        nat_sub(a3680, a3681)
    }
}

pub fn f125(v0: u64) -> u64 {
    let a3682 = v0;
    let a3683 = 2147483648u64;
    if nat_lt(a3682, a3683) {
        v0
    } else {
        0u64
    }
}

pub fn f126(v0: Adt11, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64, v7: u64, v8: u64) -> Adt9 {
    let v9: u64 = {
        let a3684 = v7;
        let a3685 = 6u64;
        if nat_eq(a3684, a3685) {
            27u64
        } else {
            v2
        }
    };
    Adt9::C7(v0, f74(v3, v4), f74(v5, v6), f75(v7, v8), f151(v1), f151(v9))
}

pub fn f127(v0: Adt4, v1: u64, v2: u64) -> Adt11 {
    let m3686 = v0;
    match m3686 {
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
        Adt4::C13(v45, v46, v47, v48) => {
            Adt11::C4(v45, v46, v47, v48)
        }
        Adt4::C14(v49, _, _) => {
            let a3687 = 13u64;
            let a3688 = v49;
            if nat_le(a3687, a3688) {
                Adt11::C7(v49, v1, v2)
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
        Adt4::C27(v69) => {
            Adt11::C2(match v69.clone() {
                Adt5::C0(_, _, _, _, _, h3689, _, _) => {
                    h3689
                }
            }, match v69 {
                Adt5::C0(_, _, _, _, _, _, _, h3690) => {
                    h3690
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
    }
}

pub fn f128(v0: Adt4) -> u64 {
    let m3691 = v0;
    match m3691 {
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
        Adt4::C14(v47, _, _) => {
            v47
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
    }
}

pub fn f129(v0: Adt4, v1: u64) -> u64 {
    let m3692 = v0;
    match m3692 {
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
        Adt4::C16(_, v52) => {
            v52
        }
        Adt4::C17 => {
            0u64
        }
        Adt4::C18(_, _) => {
            0u64
        }
        Adt4::C19(v55, _) => {
            let a3693 = v1;
            let a3694 = 6u64;
            if nat_eq(a3693, a3694) {
                v55
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
        Adt4::C26(v69, _, _, _) => {
            let a3703 = v1;
            let a3704 = 6u64;
            if if if nat_eq(a3703, a3704) {
                true
            } else {
                let a3705 = v1;
                let a3706 = 7u64;
                nat_eq(a3705, a3706)
            } {
                let a3707 = v69;
                let a3708 = 30u64;
                nat_eq(a3707, a3708)
            } else {
                false
            } {
                30u64
            } else {
                0u64
            }
        }
        Adt4::C27(v68) => {
            match v68 {
                Adt5::C0(_, _, _, _, _, _, _, h3702) => {
                    h3702
                }
            }
        }
        Adt4::C28(v64) => {
            let a3695 = v1;
            let a3696 = 6u64;
            if nat_eq(a3695, a3696) {
                match v64 {
                    Adt7::C0(_, _, _, _, h3697) => {
                        h3697
                    }
                }
            } else {
                0u64
            }
        }
        Adt4::C29(_) => {
            0u64
        }
        Adt4::C30(_, v67) => {
            let a3698 = v1;
            let a3699 = 6u64;
            if if nat_eq(a3698, a3699) {
                true
            } else {
                let a3700 = v1;
                let a3701 = 7u64;
                nat_eq(a3700, a3701)
            } {
                v67
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
    }
}

pub fn f130(v0: Adt4, v1: u64) -> Adt11 {
    let a3709 = v1;
    let a3710 = 4u64;
    if nat_lt(a3709, a3710) {
        Adt11::C0(v1)
    } else {
        let a3711 = v1;
        let a3712 = 4u64;
        if nat_eq(a3711, a3712) {
            Adt11::C1(0u64)
        } else {
            let a3713 = v1;
            let a3714 = 5u64;
            if nat_eq(a3713, a3714) {
                Adt11::C1(3u64)
            } else {
                let a3715 = v1;
                let a3716 = 6u64;
                if nat_eq(a3715, a3716) {
                    Adt11::C2(f129(v0, v1), 0u64, false)
                } else {
                    let a3717 = v1;
                    let a3718 = 7u64;
                    if nat_eq(a3717, a3718) {
                        Adt11::C2(0u64, f129(v0, v1), false)
                    } else {
                        let a3719 = v1;
                        let a3720 = 16u64;
                        if nat_lt(a3719, a3720) {
                            Adt11::C1({
                                let a3721 = v1;
                                let a3722 = 8u64;
                                nat_sub(a3721, a3722)
                            })
                        } else {
                            let a3723 = v1;
                            let a3724 = 20u64;
                            if nat_lt(a3723, a3724) {
                                Adt11::C5({
                                    let a3725 = v1;
                                    let a3726 = 16u64;
                                    nat_sub(a3725, a3726)
                                })
                            } else {
                                let a3727 = v1;
                                let a3728 = 20u64;
                                if nat_eq(a3727, a3728) {
                                    Adt11::C3(0u64, 0u64, false)
                                } else {
                                    let a3729 = v1;
                                    let a3730 = 21u64;
                                    if nat_eq(a3729, a3730) {
                                        Adt11::C0(4u64)
                                    } else {
                                        let a3731 = v1;
                                        let a3732 = 22u64;
                                        if nat_eq(a3731, a3732) {
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

pub fn f131(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let a3750 = {
        let a3745 = f214(v2, {
            let a3736 = {
                let a3733 = v0;
                let a3734 = v1;
                let a3735 = 0u64;
                nat_quot(a3733, a3734, a3735)
            };
            let a3737 = v2;
            let a3738 = 0u64;
            nat_rem(a3736, a3737, a3738)
        }, {
            let a3742 = {
                let a3739 = v0;
                let a3740 = v1;
                let a3741 = 0u64;
                nat_quot(a3739, a3740, a3741)
            };
            let a3743 = v2;
            let a3744 = 0u64;
            nat_quot(a3742, a3743, a3744)
        })?;
        let a3746 = v1;
        nat_mul(a3745, a3746)?
    };
    let a3751 = {
        let a3747 = v0;
        let a3748 = v1;
        let a3749 = 0u64;
        nat_rem(a3747, a3748, a3749)
    };
    nat_add(a3750, a3751)
}

pub fn f132(v0: u64, v1: Adt3, v2: Adt3) -> R<u64> {
    let v3: u64 = {
        let a3753 = v0;
        let a3754 = match v2.clone() {
            Adt3::C0(h3752, _, _, _, _, _, _, _) => {
                h3752
            }
        };
        let a3755 = 0u64;
        nat_quot(a3753, a3754, a3755)
    };
    let v4: u64 = {
        let a3757 = v3;
        let a3758 = match v2.clone() {
            Adt3::C0(_, h3756, _, _, _, _, _, _) => {
                h3756
            }
        };
        let a3759 = 0u64;
        nat_quot(a3757, a3758, a3759)
    };
    f215({
        let a3761 = v0;
        let a3762 = match v2.clone() {
            Adt3::C0(h3760, _, _, _, _, _, _, _) => {
                h3760
            }
        };
        let a3763 = 0u64;
        nat_rem(a3761, a3762, a3763)
    }, {
        let a3765 = v3;
        let a3766 = match v2.clone() {
            Adt3::C0(_, h3764, _, _, _, _, _, _) => {
                h3764
            }
        };
        let a3767 = 0u64;
        nat_rem(a3765, a3766, a3767)
    }, {
        let a3769 = v4;
        let a3770 = match v2.clone() {
            Adt3::C0(_, _, h3768, _, _, _, _, _) => {
                h3768
            }
        };
        let a3771 = 0u64;
        nat_rem(a3769, a3770, a3771)
    }, {
        let a3773 = v4;
        let a3774 = match v2 {
            Adt3::C0(_, _, h3772, _, _, _, _, _) => {
                h3772
            }
        };
        let a3775 = 0u64;
        nat_quot(a3773, a3774, a3775)
    }, match v1.clone() {
        Adt3::C0(h3776, _, _, _, _, _, _, _) => {
            h3776
        }
    }, match v1.clone() {
        Adt3::C0(_, h3777, _, _, _, _, _, _) => {
            h3777
        }
    }, match v1.clone() {
        Adt3::C0(_, _, h3778, _, _, _, _, _) => {
            h3778
        }
    }, match v1.clone() {
        Adt3::C0(_, _, _, h3779, _, _, _, _) => {
            h3779
        }
    }, match v1.clone() {
        Adt3::C0(_, _, _, _, h3780, _, _, _) => {
            h3780
        }
    }, match v1.clone() {
        Adt3::C0(_, _, _, _, _, h3781, _, _) => {
            h3781
        }
    }, match v1.clone() {
        Adt3::C0(_, _, _, _, _, _, h3782, _) => {
            h3782
        }
    }, match v1 {
        Adt3::C0(_, _, _, _, _, _, _, h3783) => {
            h3783
        }
    })
}

pub fn f133(v0: u64, v1: Adt3, v2: Adt3, v3: Adt3, v4: u64) -> R<Adt9> {
    let v5: u64 = {
        let a3785 = v0;
        let a3786 = match v3.clone() {
            Adt3::C0(h3784, _, _, _, _, _, _, _) => {
                h3784
            }
        };
        let a3787 = 0u64;
        nat_quot(a3785, a3786, a3787)
    };
    let v6: u64 = {
        let a3789 = v5;
        let a3790 = match v3.clone() {
            Adt3::C0(_, h3788, _, _, _, _, _, _) => {
                h3788
            }
        };
        let a3791 = 0u64;
        nat_quot(a3789, a3790, a3791)
    };
    let v7: u64 = {
        let a3793 = v0;
        let a3794 = match v3.clone() {
            Adt3::C0(h3792, _, _, _, _, _, _, _) => {
                h3792
            }
        };
        let a3795 = 0u64;
        nat_rem(a3793, a3794, a3795)
    };
    let v8: u64 = {
        let a3797 = v5;
        let a3798 = match v3.clone() {
            Adt3::C0(_, h3796, _, _, _, _, _, _) => {
                h3796
            }
        };
        let a3799 = 0u64;
        nat_rem(a3797, a3798, a3799)
    };
    let v9: u64 = {
        let a3801 = v6;
        let a3802 = match v3.clone() {
            Adt3::C0(_, _, h3800, _, _, _, _, _) => {
                h3800
            }
        };
        let a3803 = 0u64;
        nat_rem(a3801, a3802, a3803)
    };
    let v10: u64 = {
        let a3805 = v6;
        let a3806 = match v3.clone() {
            Adt3::C0(_, _, h3804, _, _, _, _, _) => {
                h3804
            }
        };
        let a3807 = 0u64;
        nat_quot(a3805, a3806, a3807)
    };
    let v11: u64 = {
        let a3808 = v4;
        let a3809 = 0u64;
        if nat_eq(a3808, a3809) {
            match v1.clone() {
                Adt3::C0(h3810, _, _, _, _, _, _, _) => {
                    h3810
                }
            }
        } else {
            let a3811 = v4;
            let a3812 = 1u64;
            if nat_eq(a3811, a3812) {
                match v1.clone() {
                    Adt3::C0(_, h3813, _, _, _, _, _, _) => {
                        h3813
                    }
                }
            } else {
                let a3814 = v4;
                let a3815 = 2u64;
                if nat_eq(a3814, a3815) {
                    match v1.clone() {
                        Adt3::C0(_, _, h3816, _, _, _, _, _) => {
                            h3816
                        }
                    }
                } else {
                    match v1.clone() {
                        Adt3::C0(_, _, _, h3817, _, _, _, _) => {
                            h3817
                        }
                    }
                }
            }
        }
    };
    let v12: bool = {
        let a3824 = v11;
        let a3825 = {
            let a3818 = v4;
            let a3819 = 0u64;
            if nat_eq(a3818, a3819) {
                v7
            } else {
                let a3820 = v4;
                let a3821 = 1u64;
                if nat_eq(a3820, a3821) {
                    v8
                } else {
                    let a3822 = v4;
                    let a3823 = 2u64;
                    if nat_eq(a3822, a3823) {
                        v9
                    } else {
                        v10
                    }
                }
            }
        };
        nat_le(a3824, a3825)
    };
    let v13: Adt3 = if v12 {
        v2
    } else {
        v1
    };
    Ok(Adt9::C1(v12, f123(if if v12 {
        let a3826 = v4;
        let a3827 = 0u64;
        nat_eq(a3826, a3827)
    } else {
        false
    } {
        let a3828 = v7;
        let a3829 = v11;
        nat_sub(a3828, a3829)
    } else {
        v7
    }, if if v12 {
        let a3830 = v4;
        let a3831 = 1u64;
        nat_eq(a3830, a3831)
    } else {
        false
    } {
        let a3832 = v8;
        let a3833 = v11;
        nat_sub(a3832, a3833)
    } else {
        v8
    }, if if v12 {
        let a3834 = v4;
        let a3835 = 2u64;
        nat_eq(a3834, a3835)
    } else {
        false
    } {
        let a3836 = v9;
        let a3837 = v11;
        nat_sub(a3836, a3837)
    } else {
        v9
    }, if if v12 {
        let a3838 = v4;
        let a3839 = 3u64;
        nat_eq(a3838, a3839)
    } else {
        false
    } {
        let a3840 = v10;
        let a3841 = v11;
        nat_sub(a3840, a3841)
    } else {
        v10
    }, match v13.clone() {
        Adt3::C0(_, _, _, _, h3842, _, _, _) => {
            h3842
        }
    }, match v13.clone() {
        Adt3::C0(_, _, _, _, _, h3843, _, _) => {
            h3843
        }
    }, match v13.clone() {
        Adt3::C0(_, _, _, _, _, _, h3844, _) => {
            h3844
        }
    }, match v13 {
        Adt3::C0(_, _, _, _, _, _, _, h3845) => {
            h3845
        }
    })?, f76(v0, v3)?))
}

pub fn f134(v0: u64, v1: Adt3, v2: Adt3, v3: u64, v4: u64, v5: u64, v6: u64, v7: bool) -> R<Adt9> {
    let v8: u64 = f48(v0, v2.clone(), 0u64);
    let v9: u64 = f48(v0, v2.clone(), 1u64);
    let v10: u64 = f48(v0, v2.clone(), 2u64);
    let v11: u64 = f48(v0, v2.clone(), 3u64);
    if v7 {
        Ok(Adt9::C1(false, f47(v1.clone(), f216(v8, v3, match v1.clone() {
            Adt3::C0(h3846, _, _, _, _, _, _, _) => {
                h3846
            }
        })?, f216(v9, v4, match v1.clone() {
            Adt3::C0(_, h3847, _, _, _, _, _, _) => {
                h3847
            }
        })?, f216(v10, v5, match v1.clone() {
            Adt3::C0(_, _, h3848, _, _, _, _, _) => {
                h3848
            }
        })?, f216(v11, v6, match v1 {
            Adt3::C0(_, _, _, h3849, _, _, _, _) => {
                h3849
            }
        })?)?, f76(v0, v2)?))
    } else {
        if if if if f217(v8, v3, match v1.clone() {
            Adt3::C0(h3850, _, _, _, _, _, _, _) => {
                h3850
            }
        })? {
            f217(v9, v4, match v1.clone() {
                Adt3::C0(_, h3851, _, _, _, _, _, _) => {
                    h3851
                }
            })?
        } else {
            false
        } {
            f217(v10, v5, match v1.clone() {
                Adt3::C0(_, _, h3852, _, _, _, _, _) => {
                    h3852
                }
            })?
        } else {
            false
        } {
            f217(v11, v6, match v1.clone() {
                Adt3::C0(_, _, _, h3853, _, _, _, _) => {
                    h3853
                }
            })?
        } else {
            false
        } {
            Ok(Adt9::C1(false, f47(v1, f218(v8, v3)?, f218(v9, v4)?, f218(v10, v5)?, f218(v11, v6)?)?, f76(v0, v2)?))
        } else {
            Ok(Adt9::C1(true, 0u64, f76(v0, v2)?))
        }
    }
}

pub fn f135(v0: u64, v1: Adt3, v2: Adt3, v3: u64) -> R<Adt9> {
    let v4: u64 = f48(v0, v2.clone(), 0u64);
    let v5: u64 = f48(v0, v2.clone(), 1u64);
    let v6: u64 = f48(v0, v2.clone(), 2u64);
    let v7: u64 = f48(v0, v2.clone(), 3u64);
    Ok(Adt9::C1(false, f47(v1.clone(), f219(v4, v3, match v1 {
        Adt3::C0(h3854, _, _, _, _, _, _, _) => {
            h3854
        }
    })?, v5, v6, v7)?, f76(v0, v2)?))
}

pub fn f136(v0: u64, v1: Adt3, v2: Adt3, v3: u64, v4: u64, v5: bool) -> R<Adt9> {
    let a3855 = v0;
    let a3856 = v3;
    if nat_lt(a3855, a3856) {
        Ok(Adt9::C1(false, {
            let a3857 = v0;
            let a3858 = 4u64;
            nat_mul(a3857, a3858)?
        }, {
            let a3859 = v0;
            let a3860 = 4u64;
            nat_mul(a3859, a3860)?
        }))
    } else {
        let v6: u64 = {
            let a3861 = v0;
            let a3862 = v3;
            nat_sub(a3861, a3862)
        };
        let v7: u64 = f76(v6, v1)?;
        let v8: u64 = f220(v6, v2, v4)?;
        if v5 {
            Ok(Adt9::C4(8u64, 0u64, v8, 4u64, v7, 3u64, v8))
        } else {
            Ok(Adt9::C1(true, v7, v8))
        }
    }
}

pub fn f137(v0: u64, v1: Adt3, v2: Adt3, v3: u64) -> R<Adt9> {
    let a3863 = v0;
    let a3864 = v3;
    if nat_lt(a3863, a3864) {
        Ok(Adt9::C1(true, 0u64, f76(v0, v2)?))
    } else {
        let v4: u64 = {
            let a3865 = v0;
            let a3866 = v3;
            nat_sub(a3865, a3866)
        };
        let v5: u64 = f132(v4, v2, v1.clone())?;
        Ok(Adt9::C4(8u64, 3u64, v5, 0u64, f76(v4, v1)?, 3u64, v5))
    }
}

pub fn f138(v0: u64, v1: u64, v2: Adt3, v3: Adt3) -> R<Adt9> {
    let v4: u64 = {
        let a3868 = v1;
        let a3869 = match v3.clone() {
            Adt3::C0(h3867, _, _, _, _, _, _, _) => {
                h3867
            }
        };
        let a3870 = 0u64;
        nat_rem(a3868, a3869, a3870)
    };
    let v5: u64 = {
        let a3876 = {
            let a3872 = v1;
            let a3873 = match v3.clone() {
                Adt3::C0(h3871, _, _, _, _, _, _, _) => {
                    h3871
                }
            };
            let a3874 = 0u64;
            nat_quot(a3872, a3873, a3874)
        };
        let a3877 = match v3.clone() {
            Adt3::C0(_, h3875, _, _, _, _, _, _) => {
                h3875
            }
        };
        let a3878 = 0u64;
        nat_rem(a3876, a3877, a3878)
    };
    let v6: bool = f221(v0, v4, v5);
    Ok(Adt9::C1(!v6, if v6 {
        f132(v1, v2, v3.clone())?
    } else {
        0u64
    }, f76(v1, v3)?))
}

pub fn f139(v0: u64, v1: Adt3, v2: Adt3, v3: u64, v4: bool, v5: Adt3) -> R<Adt9> {
    Ok(Adt9::C4({
        let a3879 = v3;
        let a3880 = 8u64;
        nat_add(a3879, a3880)?
    }, 0u64, f76(v0, v1)?, if v4 {
        5u64
    } else {
        4u64
    }, if v4 {
        0u64
    } else {
        f132(v0, v5, v2.clone())?
    }, 3u64, f76(v0, v2)?))
}

pub fn f140(v0: u64, v1: Adt3, v2: Adt3, v3: u64, v4: u64) -> R<Adt9> {
    let a3881 = v4;
    let a3882 = 0u64;
    if if nat_eq(a3881, a3882) {
        f104(v3)
    } else {
        false
    } {
        let m3883 = f192(v3);
        match m3883 {
            Adt18::C0 => {
                Ok(Adt9::C0(Adt10::C1))
            }
            Adt18::C1(v5) => {
                let v6: u64 = match v5.clone() {
                    Adt19::C0(_, h3884, _) => {
                        h3884
                    }
                };
                Ok(Adt9::C5(v3, f76(v0, v1)?, match v5 {
                    Adt19::C0(_, _, h3885) => {
                        h3885
                    }
                }, f76({
                    let a3886 = v0;
                    let a3887 = v6;
                    nat_mul(a3886, a3887)?
                }, v2.clone())?, {
                    let a3893 = {
                        let a3891 = {
                            let a3888 = v6;
                            let a3889 = 1u64;
                            nat_sub(a3888, a3889)
                        };
                        let a3892 = match v2.clone() {
                            Adt3::C0(_, _, _, _, h3890, _, _, _) => {
                                h3890
                            }
                        };
                        nat_mul(a3891, a3892)?
                    };
                    let a3894 = 4u64;
                    nat_add(a3893, a3894)?
                }, match v2 {
                    Adt3::C0(_, _, _, _, h3895, _, _, _) => {
                        h3895
                    }
                }))
            }
        }
    } else {
        Ok(Adt9::C3(v3, v4, f76(v0, v1)?, f76(v0, v2)?))
    }
}

pub fn f141(v0: u64, v1: Adt3, v2: Adt3, v3: bool, v4: u64, v5: u64) -> R<Adt9> {
    let v6: u64 = {
        let a3897 = v0;
        let a3898 = match v2.clone() {
            Adt3::C0(h3896, _, _, _, _, _, _, _) => {
                h3896
            }
        };
        let a3899 = 0u64;
        nat_rem(a3897, a3898, a3899)
    };
    let v7: u64 = {
        let a3901 = v0;
        let a3902 = match v2.clone() {
            Adt3::C0(h3900, _, _, _, _, _, _, _) => {
                h3900
            }
        };
        let a3903 = 0u64;
        nat_quot(a3901, a3902, a3903)
    };
    let v8: u64 = {
        let a3905 = v7;
        let a3906 = match v2.clone() {
            Adt3::C0(_, h3904, _, _, _, _, _, _) => {
                h3904
            }
        };
        let a3907 = 0u64;
        nat_rem(a3905, a3906, a3907)
    };
    let v9: u64 = {
        let a3909 = v7;
        let a3910 = match v2.clone() {
            Adt3::C0(_, h3908, _, _, _, _, _, _) => {
                h3908
            }
        };
        let a3911 = 0u64;
        nat_quot(a3909, a3910, a3911)
    };
    let v10: u64 = {
        let a3913 = v9;
        let a3914 = match v2.clone() {
            Adt3::C0(_, _, h3912, _, _, _, _, _) => {
                h3912
            }
        };
        let a3915 = 0u64;
        nat_rem(a3913, a3914, a3915)
    };
    let v11: u64 = {
        let a3917 = v9;
        let a3918 = match v2.clone() {
            Adt3::C0(_, _, h3916, _, _, _, _, _) => {
                h3916
            }
        };
        let a3919 = 0u64;
        nat_quot(a3917, a3918, a3919)
    };
    let v12: u64 = f76(v0, v2)?;
    if v3 {
        let v13: u64 = {
            let a3928 = {
                let a3923 = {
                    let a3920 = v10;
                    let a3921 = v4;
                    let a3922 = 0u64;
                    nat_quot(a3920, a3921, a3922)
                };
                let a3924 = v5;
                nat_mul(a3923, a3924)?
            };
            let a3929 = {
                let a3925 = v8;
                let a3926 = v4;
                let a3927 = 0u64;
                nat_quot(a3925, a3926, a3927)
            };
            nat_add(a3928, a3929)?
        };
        let v14: u64 = {
            let a3949 = v6;
            let a3950 = {
                let a3947 = match v1.clone() {
                    Adt3::C0(h3930, _, _, _, _, _, _, _) => {
                        h3930
                    }
                };
                let a3948 = {
                    let a3945 = {
                        let a3931 = v8;
                        let a3932 = v4;
                        let a3933 = 0u64;
                        nat_rem(a3931, a3932, a3933)
                    };
                    let a3946 = {
                        let a3943 = match v1.clone() {
                            Adt3::C0(_, h3934, _, _, _, _, _, _) => {
                                h3934
                            }
                        };
                        let a3944 = {
                            let a3941 = {
                                let a3935 = v10;
                                let a3936 = v4;
                                let a3937 = 0u64;
                                nat_rem(a3935, a3936, a3937)
                            };
                            let a3942 = {
                                let a3939 = match v1.clone() {
                                    Adt3::C0(_, _, h3938, _, _, _, _, _) => {
                                        h3938
                                    }
                                };
                                let a3940 = v13;
                                nat_mul(a3939, a3940)?
                            };
                            nat_add(a3941, a3942)?
                        };
                        nat_mul(a3943, a3944)?
                    };
                    nat_add(a3945, a3946)?
                };
                nat_mul(a3947, a3948)?
            };
            nat_add(a3949, a3950)?
        };
        Ok(Adt9::C1(false, f76(v14, v1)?, v12))
    } else {
        let v15: u64 = {
            let a3956 = {
                let a3954 = {
                    let a3951 = v11;
                    let a3952 = v5;
                    let a3953 = 0u64;
                    nat_rem(a3951, a3952, a3953)
                };
                let a3955 = v4;
                nat_mul(a3954, a3955)?
            };
            let a3957 = v8;
            nat_add(a3956, a3957)?
        };
        let v16: u64 = {
            let a3963 = {
                let a3961 = {
                    let a3958 = v11;
                    let a3959 = v5;
                    let a3960 = 0u64;
                    nat_quot(a3958, a3959, a3960)
                };
                let a3962 = v4;
                nat_mul(a3961, a3962)?
            };
            let a3964 = v10;
            nat_add(a3963, a3964)?
        };
        let a3966 = v15;
        let a3967 = match v1.clone() {
            Adt3::C0(_, h3965, _, _, _, _, _, _) => {
                h3965
            }
        };
        if if nat_lt(a3966, a3967) {
            let a3969 = v16;
            let a3970 = match v1.clone() {
                Adt3::C0(_, _, h3968, _, _, _, _, _) => {
                    h3968
                }
            };
            nat_lt(a3969, a3970)
        } else {
            false
        } {
            Ok(Adt9::C1(false, f76({
                let a3979 = v6;
                let a3980 = {
                    let a3977 = match v1.clone() {
                        Adt3::C0(h3971, _, _, _, _, _, _, _) => {
                            h3971
                        }
                    };
                    let a3978 = {
                        let a3975 = v15;
                        let a3976 = {
                            let a3973 = match v1.clone() {
                                Adt3::C0(_, h3972, _, _, _, _, _, _) => {
                                    h3972
                                }
                            };
                            let a3974 = v16;
                            nat_mul(a3973, a3974)?
                        };
                        nat_add(a3975, a3976)?
                    };
                    nat_mul(a3977, a3978)?
                };
                nat_add(a3979, a3980)?
            }, v1)?, v12))
        } else {
            Ok(Adt9::C1(true, 0u64, v12))
        }
    }
}

pub fn f142(v0: u64, v1: Adt3, v2: Adt3, v3: u64, v4: u64) -> R<Adt9> {
    let v5: u64 = f213(23u64, v4);
    let v6: u64 = {
        let a3981 = v0;
        let a3982 = v5;
        let a3983 = 0u64;
        nat_rem(a3981, a3982, a3983)
    };
    let v7: u64 = {
        let a3984 = v0;
        let a3985 = v5;
        let a3986 = 0u64;
        nat_quot(a3984, a3985, a3986)
    };
    let v8: Adt13 = Adt13::C1(f76(v7, v2)?);
    let a3987 = v6;
    let a3988 = 0u64;
    if nat_eq(a3987, a3988) {
        Ok(Adt9::C7(Adt11::C7(16u64, v3, 0u64), Adt12::C0(0u64, f76(v7, v1)?), Adt12::C2(0u64), Adt13::C0(0u64), f151(v3), 4u64))
    } else {
        let a3989 = v6;
        let a3990 = 1u64;
        if nat_eq(a3989, a3990) {
            Ok(Adt9::C7(Adt11::C1(1u64), Adt12::C1(0u64), Adt12::C2(1065353216u64), {
                let a3991 = v4;
                let a3992 = 0u64;
                if nat_eq(a3991, a3992) {
                    v8
                } else {
                    Adt13::C0(0u64)
                }
            }, 4u64, 4u64))
        } else {
            Ok(Adt9::C7(Adt11::C2(0u64, v4, true), Adt12::C1(0u64), Adt12::C2(0u64), v8, 4u64, f151(v4)))
        }
    }
}

pub fn f143(v0: u64, v1: Adt3, v2: Adt3) -> R<Adt9> {
    let v3: u64 = f76(v0, v2.clone())?;
    let v4: u64 = f76(v0, v1.clone())?;
    let a3997 = {
        let a3994 = v0;
        let a3995 = match v1 {
            Adt3::C0(h3993, _, _, _, _, _, _, _) => {
                h3993
            }
        };
        let a3996 = 0u64;
        nat_rem(a3994, a3995, a3996)
    };
    let a3998 = 0u64;
    if nat_eq(a3997, a3998) {
        Ok(Adt9::C1(false, v4, v3))
    } else {
        Ok(Adt9::C4(8u64, 3u64, f76({
            let a3999 = v0;
            let a4000 = 1u64;
            nat_sub(a3999, a4000)
        }, v2)?, 0u64, v4, 3u64, v3))
    }
}

pub fn f144(v0: u64, v1: Adt3, v2: Adt3, v3: u64, v4: u64) -> R<Adt9> {
    let a4001 = v3;
    let a4002 = 13u64;
    if nat_eq(a4001, a4002) {
        f222(v0, v1, v4)
    } else {
        let v5: u64 = f223(match v1.clone() {
            Adt3::C0(h4003, _, _, _, _, _, _, _) => {
                h4003
            }
        }, {
            let a4004 = v3;
            let a4005 = 16u64;
            nat_eq(a4004, a4005)
        })?;
        let v6: u64 = {
            let a4006 = v0;
            let a4007 = v5;
            let a4008 = 0u64;
            nat_quot(a4006, a4007, a4008)
        };
        let v7: u64 = {
            let a4009 = v0;
            let a4010 = v5;
            let a4011 = 0u64;
            nat_rem(a4009, a4010, a4011)
        };
        let v8: u64 = {
            let a4015 = 1u64;
            let a4016 = {
                let a4013 = match v1.clone() {
                    Adt3::C0(h4012, _, _, _, _, _, _, _) => {
                        h4012
                    }
                };
                let a4014 = 2u64;
                nat_mul(a4013, a4014)?
            };
            nat_add(a4015, a4016)?
        };
        let a4017 = v7;
        let a4018 = v8;
        if nat_lt(a4017, a4018) {
            f224(v7, v6, v1, 0u64)
        } else {
            let v9: u64 = f76(v6, v2)?;
            let a4019 = v3;
            let a4020 = 14u64;
            if nat_eq(a4019, a4020) {
                Ok(Adt9::C4(2u64, 1u64, 1u64, 2u64, 0u64, 3u64, v9))
            } else {
                let a4021 = v7;
                let a4022 = v8;
                if nat_eq(a4021, a4022) {
                    Ok(Adt9::C4(2u64, 1u64, 1u64, 2u64, 0u64, 1u64, 1u64))
                } else {
                    let a4025 = v7;
                    let a4026 = {
                        let a4023 = v8;
                        let a4024 = 1u64;
                        nat_add(a4023, a4024)?
                    };
                    if nat_eq(a4025, a4026) {
                        Ok(Adt9::C4(3u64, 2u64, match v1 {
                            Adt3::C0(h4027, _, _, _, _, _, _, _) => {
                                h4027
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

pub fn f145(v0: u64, v1: Adt3, v2: Adt3, v3: u64) -> R<Adt9> {
    let a4028 = v0;
    let a4029 = 0u64;
    if nat_eq(a4028, a4029) {
        Ok(Adt9::C4(19u64, 2u64, 0u64, 2u64, 0u64, 1u64, 0u64))
    } else {
        let a4036 = {
            let a4030 = v0;
            let a4031 = 1u64;
            nat_add(a4030, a4031)?
        };
        let a4037 = {
            let a4034 = 2u64;
            let a4035 = {
                let a4032 = v3;
                let a4033 = 2u64;
                nat_mul(a4032, a4033)?
            };
            nat_add(a4034, a4035)?
        };
        if nat_eq(a4036, a4037) {
            Ok(Adt9::C4(19u64, 1u64, 0u64, 2u64, 0u64, 6u64, 0u64))
        } else {
            let v4: u64 = {
                let a4038 = v0;
                let a4039 = 1u64;
                nat_sub(a4038, a4039)
            };
            let v5: u64 = {
                let a4040 = v4;
                let a4041 = 2u64;
                let a4042 = 0u64;
                nat_quot(a4040, a4041, a4042)
            };
            let a4046 = {
                let a4043 = v4;
                let a4044 = 2u64;
                let a4045 = 0u64;
                nat_rem(a4043, a4044, a4045)
            };
            let a4047 = 0u64;
            if nat_eq(a4046, a4047) {
                Ok(Adt9::C4(17u64, 0u64, f76(v5, v1)?, 4u64, f76(v5, v2)?, 1u64, 2u64))
            } else {
                Ok(Adt9::C4(18u64, 1u64, 0u64, 1u64, 2u64, 1u64, 0u64))
            }
        }
    }
}

pub fn f146(v0: u64, v1: u64, v2: u64, v3: u64) -> R<Adt9> {
    let a4048 = v0;
    let a4049 = f31(v1, v2, v3)?;
    if nat_lt(a4048, a4049) {
        let a4050 = v0;
        let a4051 = v1;
        if nat_lt(a4050, a4051) {
            Ok(Adt9::C1(false, {
                let a4052 = 4u64;
                let a4053 = v0;
                nat_mul(a4052, a4053)?
            }, {
                let a4054 = 4u64;
                let a4055 = v0;
                nat_mul(a4054, a4055)?
            }))
        } else {
            let v4: u64 = {
                let a4056 = v0;
                let a4057 = v1;
                nat_sub(a4056, a4057)
            };
            let v5: u64 = f225(v4, v2)?;
            let a4058 = v5;
            let a4059 = 2u64;
            if nat_lt(a4058, a4059) {
                Ok(Adt9::C4(6u64, {
                    let a4060 = v5;
                    let a4061 = 0u64;
                    if nat_eq(a4060, a4061) {
                        4u64
                    } else {
                        5u64
                    }
                }, f226(v4, v2)?, 2u64, 0u64, 1u64, v5))
            } else {
                let v6: u64 = f227(v4, v2)?;
                Ok(Adt9::C4(8u64, 3u64, v6, 1u64, if f228(v4, v2)? {
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

pub fn f147(v0: u64, v1: u64) -> u64 {
    let a4062 = v0;
    let a4063 = 0u64;
    if if nat_eq(a4062, a4063) {
        true
    } else {
        let a4064 = v1;
        let a4065 = 0u64;
        nat_eq(a4064, a4065)
    } {
        1u64
    } else {
        2u64
    }
}

pub fn f148(v0: u64, v1: u64) -> Adt11 {
    let a4066 = v0;
    let a4067 = 43u64;
    if nat_eq(a4066, a4067) {
        Adt11::C8(3u64)
    } else {
        let a4068 = v1;
        let a4069 = 43u64;
        if nat_eq(a4068, a4069) {
            Adt11::C8(4u64)
        } else {
            Adt11::C2(v0, v1, true)
        }
    }
}

pub fn f149(v0: u64) -> R<u64> {
    let a4072 = 2u64;
    let a4073 = {
        let a4070 = 6u64;
        let a4071 = v0;
        nat_mul(a4070, a4071)?
    };
    nat_add(a4072, a4073)
}

pub fn f150(v0: u64, v1: bool) -> R<u64> {
    let a4076 = {
        let a4074 = v0;
        let a4075 = if v1 {
            3u64
        } else {
            2u64
        };
        nat_mul(a4074, a4075)?
    };
    let a4077 = 1u64;
    nat_add(a4076, a4077)
}

pub fn f151(v0: u64) -> u64 {
    let m4078 = f192(v0);
    match m4078 {
        Adt18::C0 => {
            0u64
        }
        Adt18::C1(v1) => {
            let a4080 = match v1.clone() {
                Adt19::C0(_, h4079, _) => {
                    h4079
                }
            };
            let a4081 = 1u64;
            if if nat_eq(a4080, a4081) {
                let a4083 = match v1.clone() {
                    Adt19::C0(_, _, h4082) => {
                        h4082
                    }
                };
                let a4084 = 1u64;
                if if if nat_eq(a4083, a4084) {
                    true
                } else {
                    let a4086 = match v1.clone() {
                        Adt19::C0(_, _, h4085) => {
                            h4085
                        }
                    };
                    let a4087 = 2u64;
                    nat_eq(a4086, a4087)
                } {
                    true
                } else {
                    let a4089 = match v1.clone() {
                        Adt19::C0(_, _, h4088) => {
                            h4088
                        }
                    };
                    let a4090 = 4u64;
                    nat_eq(a4089, a4090)
                } {
                    true
                } else {
                    let a4092 = match v1.clone() {
                        Adt19::C0(_, _, h4091) => {
                            h4091
                        }
                    };
                    let a4093 = 8u64;
                    nat_eq(a4092, a4093)
                }
            } else {
                false
            } {
                match v1 {
                    Adt19::C0(_, _, h4094) => {
                        h4094
                    }
                }
            } else {
                0u64
            }
        }
    }
}

pub fn f152(v0: u64) -> R<u64> {
    let a4097 = 3u64;
    let a4098 = {
        let a4095 = v0;
        let a4096 = 3u64;
        nat_mul(a4095, a4096)?
    };
    nat_add(a4097, a4098)
}

pub fn f153(v0: u64, v1: u64) -> R<u64> {
    if f154(v0) {
        let a4099 = 2u64;
        let a4100 = v1;
        nat_mul(a4099, a4100)
    } else {
        let a4101 = v1;
        let a4102 = f229(v0);
        let a4103 = 0u64;
        Ok(nat_quot(a4101, a4102, a4103))
    }
}

pub fn f154(v0: u64) -> bool {
    let a4104 = v0;
    let a4105 = 0u64;
    if if nat_eq(a4104, a4105) {
        true
    } else {
        let a4106 = v0;
        let a4107 = 1u64;
        nat_eq(a4106, a4107)
    } {
        true
    } else {
        let a4108 = v0;
        let a4109 = 30u64;
        nat_eq(a4108, a4109)
    }
}

pub fn f155(v0: u64, v1: u64, v2: u64, v3: u64, v4: Adt3, v5: Adt3) -> R<Adt9> {
    let m4110 = f192(v0);
    match m4110 {
        Adt18::C0 => {
            Ok(Adt9::C0(Adt10::C1))
        }
        Adt18::C1(v6) => {
            Ok(Adt9::C10(v0, f123(v2, v1, 0u64, 0u64, match v4.clone() {
                Adt3::C0(_, _, _, _, h4111, _, _, _) => {
                    h4111
                }
            }, match v4.clone() {
                Adt3::C0(_, _, _, _, _, h4112, _, _) => {
                    h4112
                }
            }, match v4.clone() {
                Adt3::C0(_, _, _, _, _, _, h4113, _) => {
                    h4113
                }
            }, match v4 {
                Adt3::C0(_, _, _, _, _, _, _, h4114) => {
                    h4114
                }
            })?, match v6.clone() {
                Adt19::C0(_, _, h4115) => {
                    h4115
                }
            }, f123({
                let a4117 = v2;
                let a4118 = match v6.clone() {
                    Adt19::C0(_, h4116, _) => {
                        h4116
                    }
                };
                nat_mul(a4117, a4118)?
            }, v3, 0u64, 0u64, match v5.clone() {
                Adt3::C0(_, _, _, _, h4119, _, _, _) => {
                    h4119
                }
            }, match v5.clone() {
                Adt3::C0(_, _, _, _, _, h4120, _, _) => {
                    h4120
                }
            }, match v5.clone() {
                Adt3::C0(_, _, _, _, _, _, h4121, _) => {
                    h4121
                }
            }, match v5.clone() {
                Adt3::C0(_, _, _, _, _, _, _, h4122) => {
                    h4122
                }
            })?, match v6 {
                Adt19::C0(_, h4123, _) => {
                    h4123
                }
            }, match v5 {
                Adt3::C0(_, _, _, _, h4124, _, _, _) => {
                    h4124
                }
            }))
        }
    }
}

pub fn f156(v0: Adt3, v1: Adt3) -> R<u64> {
    let a4132 = {
        let a4125 = 2u64;
        let a4126 = f157(v1.clone())?;
        nat_mul(a4125, a4126)?
    };
    let a4133 = {
        let a4130 = f157(v1)?;
        let a4131 = {
            let a4128 = 1u64;
            let a4129 = match v0 {
                Adt3::C0(h4127, _, _, _, _, _, _, _) => {
                    h4127
                }
            };
            nat_add(a4128, a4129)?
        };
        nat_mul(a4130, a4131)?
    };
    nat_add(a4132, a4133)
}

pub fn f157(v0: Adt3) -> R<u64> {
    if f38(match v0.clone() {
        Adt3::C0(h4134, _, _, _, _, _, _, _) => {
            h4134
        }
    }, match v0.clone() {
        Adt3::C0(_, h4135, _, _, _, _, _, _) => {
            h4135
        }
    }, match v0.clone() {
        Adt3::C0(_, _, h4136, _, _, _, _, _) => {
            h4136
        }
    }, match v0.clone() {
        Adt3::C0(_, _, _, h4137, _, _, _, _) => {
            h4137
        }
    }) {
        Ok(0u64)
    } else {
        let a4146 = match v0.clone() {
            Adt3::C0(h4138, _, _, _, _, _, _, _) => {
                h4138
            }
        };
        let a4147 = {
            let a4144 = match v0.clone() {
                Adt3::C0(_, h4139, _, _, _, _, _, _) => {
                    h4139
                }
            };
            let a4145 = {
                let a4142 = match v0.clone() {
                    Adt3::C0(_, _, h4140, _, _, _, _, _) => {
                        h4140
                    }
                };
                let a4143 = match v0 {
                    Adt3::C0(_, _, _, h4141, _, _, _, _) => {
                        h4141
                    }
                };
                nat_mul(a4142, a4143)?
            };
            nat_mul(a4144, a4145)?
        };
        nat_mul(a4146, a4147)
    }
}

pub fn f158(v0: u64, v1: Adt3) -> R<Adt9> {
    Ok(Adt9::C4(19u64, 4u64, f76(v0, v1)?, 2u64, 0u64, 1u64, 2u64))
}

pub fn f159(v0: u64, v1: u64) -> bool {
    let a4148 = v1;
    let a4149 = 2147483647u64;
    if nat_le(a4148, a4149) {
        let a4150 = v1;
        let a4151 = v0;
        nat_lt(a4150, a4151)
    } else {
        false
    }
}

pub fn f160(v0: u64, v1: u64, v2: u64, v3: Adt3, v4: Adt3, v5: Adt3, v6: u64) -> R<Adt9> {
    let m4152 = f192(v0);
    match m4152 {
        Adt18::C0 => {
            Ok(Adt9::C0(Adt10::C1))
        }
        Adt18::C1(v7) => {
            let v8: u64 = {
                let a4154 = v2;
                let a4155 = match v7.clone() {
                    Adt19::C0(_, h4153, _) => {
                        h4153
                    }
                };
                nat_mul(a4154, a4155)?
            };
            let m4168 = f230(v0, v8, v6, v1, match v5.clone() {
                Adt3::C0(h4156, _, _, _, _, _, _, _) => {
                    h4156
                }
            }, match v5.clone() {
                Adt3::C0(_, h4157, _, _, _, _, _, _) => {
                    h4157
                }
            }, match v5 {
                Adt3::C0(_, _, h4158, _, _, _, _, _) => {
                    h4158
                }
            }, {
                let a4161 = match v3.clone() {
                    Adt3::C0(h4159, _, _, _, _, _, _, _) => {
                        h4159
                    }
                };
                let a4162 = match v7.clone() {
                    Adt19::C0(_, h4160, _) => {
                        h4160
                    }
                };
                nat_mul(a4161, a4162)?
            }, match v3.clone() {
                Adt3::C0(_, h4163, _, _, _, _, _, _) => {
                    h4163
                }
            }, match v3.clone() {
                Adt3::C0(_, _, _, _, h4164, _, _, _) => {
                    h4164
                }
            }, match v3.clone() {
                Adt3::C0(_, _, _, _, _, h4165, _, _) => {
                    h4165
                }
            }, match v3.clone() {
                Adt3::C0(_, _, _, _, _, _, h4166, _) => {
                    h4166
                }
            }, match v3 {
                Adt3::C0(_, _, _, _, _, _, _, h4167) => {
                    h4167
                }
            })?;
            match m4168 {
                None => {
                    Ok(Adt9::C0(Adt10::C1))
                }
                Some(v9) => {
                    let a4169 = v0;
                    let a4170 = 0u64;
                    if if nat_eq(a4169, a4170) {
                        true
                    } else {
                        let a4171 = v0;
                        let a4172 = 26u64;
                        nat_eq(a4171, a4172)
                    } {
                        Ok(Adt9::C1(false, v9, f76({
                            let a4176 = {
                                let a4174 = v1;
                                let a4175 = match v4.clone() {
                                    Adt3::C0(h4173, _, _, _, _, _, _, _) => {
                                        h4173
                                    }
                                };
                                nat_mul(a4174, a4175)?
                            };
                            let a4177 = v8;
                            nat_add(a4176, a4177)?
                        }, v4)?))
                    } else {
                        Ok(Adt9::C5(v0, v9, match v7.clone() {
                            Adt19::C0(_, _, h4178) => {
                                h4178
                            }
                        }, f76({
                            let a4182 = {
                                let a4180 = v1;
                                let a4181 = match v4.clone() {
                                    Adt3::C0(h4179, _, _, _, _, _, _, _) => {
                                        h4179
                                    }
                                };
                                nat_mul(a4180, a4181)?
                            };
                            let a4183 = v8;
                            nat_add(a4182, a4183)?
                        }, v4.clone())?, {
                            let a4190 = {
                                let a4188 = {
                                    let a4185 = match v7 {
                                        Adt19::C0(_, h4184, _) => {
                                            h4184
                                        }
                                    };
                                    let a4186 = 1u64;
                                    nat_sub(a4185, a4186)
                                };
                                let a4189 = match v4.clone() {
                                    Adt3::C0(_, _, _, _, h4187, _, _, _) => {
                                        h4187
                                    }
                                };
                                nat_mul(a4188, a4189)?
                            };
                            let a4191 = 4u64;
                            nat_add(a4190, a4191)?
                        }, match v4 {
                            Adt3::C0(_, _, _, _, h4192, _, _, _) => {
                                h4192
                            }
                        }))
                    }
                }
            }
        }
    }
}

pub fn f161(v0: u64, v1: u64, v2: u64) -> u64 {
    let a4193 = v1;
    let a4194 = 0u64;
    if if nat_eq(a4193, a4194) {
        true
    } else {
        let a4195 = v2;
        let a4196 = 0u64;
        nat_eq(a4195, a4196)
    } {
        0u64
    } else {
        v0
    }
}

pub fn f162(v0: u64, v1: u64) -> bool {
    let a4197 = v0;
    let a4198 = 4294967295u64;
    if nat_eq(a4197, a4198) {
        true
    } else {
        let a4199 = v0;
        let a4200 = 2147483648u64;
        if nat_lt(a4199, a4200) {
            let a4201 = v0;
            let a4202 = v1;
            nat_lt(a4201, a4202)
        } else {
            false
        }
    }
}

pub fn f163(v0: Adt7) -> R<u64> {
    let a4214 = if match v0.clone() {
        Adt7::C0(_, _, _, h4203, _) => {
            h4203
        }
    } {
        2u64
    } else {
        3u64
    };
    let a4215 = {
        let a4212 = {
            let a4210 = {
                let m4206 = match v0.clone() {
                    Adt7::C0(h4205, _, _, _, _) => {
                        h4205
                    }
                };
                match m4206 {
                    Adt6::C0(h4204, _, _, _) => {
                        h4204
                    }
                }
            };
            let a4211 = {
                let m4209 = match v0.clone() {
                    Adt7::C0(_, h4208, _, _, _) => {
                        h4208
                    }
                };
                match m4209 {
                    Adt6::C0(h4207, _, _, _) => {
                        h4207
                    }
                }
            };
            nat_mul(a4210, a4211)?
        };
        let a4213 = f164(v0)?;
        nat_mul(a4212, a4213)?
    };
    nat_add(a4214, a4215)
}

pub fn f164(v0: Adt7) -> R<u64> {
    let a4220 = if match v0.clone() {
        Adt7::C0(_, _, _, h4216, _) => {
            h4216
        }
    } {
        2u64
    } else {
        1u64
    };
    let a4221 = {
        let a4218 = match v0 {
            Adt7::C0(_, _, _, _, h4217) => {
                h4217
            }
        };
        let a4219 = 1u64;
        if nat_eq(a4218, a4219) {
            1u64
        } else {
            0u64
        }
    };
    nat_add(a4220, a4221)
}

pub fn f165(v0: Adt6, v1: u64, v2: u64) -> R<u64> {
    let v3: u64 = {
        let a4226 = {
            let a4223 = v2;
            let a4224 = match v0.clone() {
                Adt6::C0(_, h4222, _, _) => {
                    h4222
                }
            };
            nat_mul(a4223, a4224)?
        };
        let a4227 = f124(match v0.clone() {
            Adt6::C0(_, _, h4225, _) => {
                h4225
            }
        });
        nat_add(a4226, a4227)?
    };
    let v4: u64 = f125(match v0.clone() {
        Adt6::C0(_, _, h4228, _) => {
            h4228
        }
    });
    let v5: u64 = {
        let a4232 = {
            let a4230 = v3;
            let a4231 = match v0 {
                Adt6::C0(h4229, _, _, _) => {
                    h4229
                }
            };
            nat_add(a4230, a4231)?
        };
        let a4233 = v4;
        nat_sub(a4232, a4233)
    };
    let a4238 = {
        let a4234 = v5;
        let a4235 = v1;
        if nat_lt(a4234, a4235) {
            v5
        } else {
            v1
        }
    };
    let a4239 = {
        let a4236 = v3;
        let a4237 = v4;
        nat_sub(a4236, a4237)
    };
    Ok(nat_sub(a4238, a4239))
}

pub fn f166(v0: Adt7, v1: Adt3, v2: Adt3, v3: u64, v4: u64) -> R<Option<u64>> {
    let m4248 = f50(match v0.clone() {
        Adt7::C0(h4240, _, _, _, _) => {
            h4240
        }
    }, match v1.clone() {
        Adt3::C0(h4241, _, _, _, _, _, _, _) => {
            h4241
        }
    }, f48(v3, v2.clone(), 0u64), {
        let a4245 = v4;
        let a4246 = {
            let m4244 = match v0.clone() {
                Adt7::C0(h4243, _, _, _, _) => {
                    h4243
                }
            };
            match m4244 {
                Adt6::C0(h4242, _, _, _) => {
                    h4242
                }
            }
        };
        let a4247 = 0u64;
        nat_rem(a4245, a4246, a4247)
    })?;
    match m4248 {
        None => {
            Ok(None::<u64>)
        }
        Some(v5) => {
            let m4257 = f50(match v0.clone() {
                Adt7::C0(_, h4249, _, _, _) => {
                    h4249
                }
            }, match v1.clone() {
                Adt3::C0(_, h4250, _, _, _, _, _, _) => {
                    h4250
                }
            }, f48(v3, v2.clone(), 1u64), {
                let a4254 = v4;
                let a4255 = {
                    let m4253 = match v0 {
                        Adt7::C0(h4252, _, _, _, _) => {
                            h4252
                        }
                    };
                    match m4253 {
                        Adt6::C0(h4251, _, _, _) => {
                            h4251
                        }
                    }
                };
                let a4256 = 0u64;
                nat_quot(a4254, a4255, a4256)
            })?;
            match m4257 {
                None => {
                    Ok(None::<u64>)
                }
                Some(v6) => {
                    Ok(Some(f47(v1, v5, v6, f48(v3, v2.clone(), 2u64), f48(v3, v2, 3u64))?))
                }
            }
        }
    }
}

pub fn f167(v0: u64) -> R<u64> {
    let a4260 = 2u64;
    let a4261 = {
        let a4258 = v0;
        let a4259 = 2u64;
        nat_mul(a4258, a4259)?
    };
    nat_add(a4260, a4261)
}

pub fn f168(v0: Adt6) -> R<u64> {
    let a4274 = 2u64;
    let a4275 = {
        let a4272 = 2u64;
        let a4273 = {
            let a4269 = {
                let a4266 = match v0.clone() {
                    Adt6::C0(h4262, _, _, _) => {
                        h4262
                    }
                };
                let a4267 = {
                    let a4264 = match v0.clone() {
                        Adt6::C0(_, h4263, _, _) => {
                            h4263
                        }
                    };
                    let a4265 = 1u64;
                    nat_sub(a4264, a4265)
                };
                nat_add(a4266, a4267)?
            };
            let a4270 = match v0 {
                Adt6::C0(_, h4268, _, _) => {
                    h4268
                }
            };
            let a4271 = 0u64;
            nat_quot(a4269, a4270, a4271)
        };
        nat_mul(a4272, a4273)?
    };
    nat_add(a4274, a4275)
}

pub fn f169(v0: Adt16, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64, v7: u64, v8: u64, v9: u64, v10: u64, v11: u64, v12: u64, v13: u64, v14: u64, v15: u64, v16: u64, v17: u64, v18: u64) -> R<Adt17> {
    let m4276 = v0;
    match m4276 {
        Adt16::C0(v19) => {
            f231(v1, v2, v3, v4, v5, v6, v7, v8, v9, v14, v15, v16, v17, v19, v18)
        }
        Adt16::C1(v20, v21, v22, v23) => {
            let a4277 = v20;
            let a4278 = 66u64;
            if nat_eq(a4277, a4278) {
                f232(v1, v2, v3, v4, v5, v6, v7, v8, v9, v10, v11, v12, v13, v14, v15, v16, v17, v21, v22, v23, v18)
            } else {
                f233(v20, v1, v2, v3, v4, v5, v6, v7, v8, v9, v14, v15, v16, v17, v21, v22, v23, v18)
            }
        }
    }
}

pub fn f170(v0: u64, v1: Adt3, v2: u64, v3: u64, v4: u64) -> R<Adt9> {
    let v5: u64 = {
        let a4285 = v3;
        let a4286 = {
            let a4282 = f76({
                let a4279 = v0;
                let a4280 = 2u64;
                let a4281 = 0u64;
                nat_quot(a4279, a4280, a4281)
            }, v1)?;
            let a4283 = v2;
            let a4284 = 0u64;
            nat_quot(a4282, a4283, a4284)
        };
        nat_add(a4285, a4286)?
    };
    let a4290 = {
        let a4287 = v0;
        let a4288 = 2u64;
        let a4289 = 0u64;
        nat_rem(a4287, a4288, a4289)
    };
    let a4291 = 0u64;
    if nat_eq(a4290, a4291) {
        Ok(Adt9::C4(19u64, 1u64, v5, 2u64, 0u64, 1u64, 2u64))
    } else {
        let a4292 = v4;
        let a4293 = 0u64;
        if nat_eq(a4292, a4293) {
            Ok(Adt9::C4(19u64, 2u64, 1u64, 2u64, 0u64, 1u64, v5))
        } else {
            Ok(Adt9::C0(Adt10::C1))
        }
    }
}

pub fn f171(v0: u64, v1: u64) -> R<u64> {
    let a4296 = 8u64;
    let a4297 = {
        let a4294 = f234(v0);
        let a4295 = v1;
        nat_mul(a4294, a4295)?
    };
    nat_add(a4296, a4297)
}

pub fn f172(v0: u64, v1: u64, v2: Adt3, v3: u64, v4: u64, v5: u64) -> R<Adt9> {
    let v6: u64 = match v2.clone() {
        Adt3::C0(h4298, _, _, _, _, _, _, _) => {
            h4298
        }
    };
    let v7: u64 = f234(v3);
    let v8: u64 = {
        let a4301 = 1u64;
        let a4302 = {
            let a4299 = v6;
            let a4300 = v7;
            nat_mul(a4299, a4300)?
        };
        nat_add(a4301, a4302)?
    };
    let a4303 = v0;
    let a4304 = 0u64;
    if nat_eq(a4303, a4304) {
        Ok(f235(0u64))
    } else {
        let a4305 = v0;
        let a4306 = v8;
        if nat_lt(a4305, a4306) {
            let v9: u64 = {
                let a4307 = v0;
                let a4308 = 1u64;
                nat_sub(a4307, a4308)
            };
            let v10: u64 = {
                let a4309 = v9;
                let a4310 = v7;
                let a4311 = 0u64;
                nat_rem(a4309, a4310, a4311)
            };
            let v11: u64 = f76({
                let a4317 = {
                    let a4312 = v1;
                    let a4313 = v6;
                    nat_mul(a4312, a4313)?
                };
                let a4318 = {
                    let a4314 = v9;
                    let a4315 = v7;
                    let a4316 = 0u64;
                    nat_quot(a4314, a4315, a4316)
                };
                nat_add(a4317, a4318)?
            }, v2)?;
            let a4319 = v3;
            let a4320 = 30u64;
            if nat_eq(a4319, a4320) {
                let a4321 = v10;
                let a4322 = 0u64;
                if nat_eq(a4321, a4322) {
                    Ok(Adt9::C4(6u64, 0u64, v11, 2u64, 0u64, 1u64, 1u64))
                } else {
                    let a4323 = v10;
                    let a4324 = 1u64;
                    if nat_eq(a4323, a4324) {
                        Ok(Adt9::C4(10u64, 1u64, 1u64, 1u64, 1u64, 1u64, 1u64))
                    } else {
                        let a4325 = v10;
                        let a4326 = 2u64;
                        if nat_eq(a4325, a4326) {
                            Ok(Adt9::C4(1u64, 1u64, 1u64, 2u64, 0u64, 1u64, 1u64))
                        } else {
                            Ok(Adt9::C4(0u64, 1u64, 0u64, 1u64, 1u64, 1u64, 0u64))
                        }
                    }
                }
            } else {
                let a4327 = v10;
                let a4328 = 0u64;
                if nat_eq(a4327, a4328) {
                    Ok(Adt9::C4(10u64, 0u64, v11, 0u64, v11, 1u64, 1u64))
                } else {
                    let a4329 = v10;
                    let a4330 = 1u64;
                    if nat_eq(a4329, a4330) {
                        Ok(Adt9::C4(1u64, 1u64, 1u64, 2u64, 0u64, 1u64, 1u64))
                    } else {
                        Ok(Adt9::C4(0u64, 1u64, 0u64, 1u64, 1u64, 1u64, 0u64))
                    }
                }
            }
        } else {
            let v12: u64 = {
                let a4331 = v0;
                let a4332 = v8;
                nat_sub(a4331, a4332)
            };
            let a4333 = v12;
            let a4334 = 0u64;
            if nat_eq(a4333, a4334) {
                Ok(Adt9::C4(22u64, 2u64, v6, 2u64, 0u64, 1u64, 1u64))
            } else {
                let a4335 = v12;
                let a4336 = 1u64;
                if nat_eq(a4335, a4336) {
                    Ok(Adt9::C4(21u64, 1u64, 0u64, 1u64, 1u64, 1u64, 1u64))
                } else {
                    let a4337 = v12;
                    let a4338 = 2u64;
                    if nat_eq(a4337, a4338) {
                        Ok(Adt9::C4(2u64, 1u64, 1u64, 2u64, 0u64, 1u64, 1u64))
                    } else {
                        let a4339 = v12;
                        let a4340 = 3u64;
                        if nat_eq(a4339, a4340) {
                            Ok(Adt9::C4(4u64, 1u64, 1u64, 2u64, v4, 1u64, 1u64))
                        } else {
                            let a4341 = v12;
                            let a4342 = 4u64;
                            if nat_eq(a4341, a4342) {
                                Ok(Adt9::C4(12u64, 1u64, 1u64, 2u64, 0u64, 1u64, 1u64))
                            } else {
                                let a4343 = v12;
                                let a4344 = 5u64;
                                if nat_eq(a4343, a4344) {
                                    Ok(Adt9::C4(11u64, 2u64, 1065353216u64, 1u64, 1u64, 1u64, 2u64))
                                } else {
                                    let a4345 = v12;
                                    let a4346 = 6u64;
                                    if nat_eq(a4345, a4346) {
                                        if f236(v5) {
                                            Ok(Adt9::C4(19u64, 1u64, 2u64, 2u64, 0u64, 1u64, {
                                                let a4347 = 3u64;
                                                let a4348 = v1;
                                                nat_add(a4347, a4348)?
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

pub fn f173(v0: u64, v1: Adt3, v2: Adt3, v3: u64) -> R<Adt9> {
    let v4: u64 = {
        let a4349 = v0;
        let a4350 = f87(v3);
        let a4351 = 0u64;
        nat_quot(a4349, a4350, a4351)
    };
    let v5: u64 = {
        let a4356 = 3u64;
        let a4357 = {
            let a4353 = v4;
            let a4354 = match v1.clone() {
                Adt3::C0(h4352, _, _, _, _, _, _, _) => {
                    h4352
                }
            };
            let a4355 = 0u64;
            nat_quot(a4353, a4354, a4355)
        };
        nat_add(a4356, a4357)?
    };
    let v6: u64 = f76(v4, v1)?;
    let v7: u64 = f76(v4, v2)?;
    let a4358 = v3;
    let a4359 = 30u64;
    if nat_eq(a4358, a4359) {
        let a4363 = {
            let a4360 = v0;
            let a4361 = 3u64;
            let a4362 = 0u64;
            nat_rem(a4360, a4361, a4362)
        };
        let a4364 = 0u64;
        if nat_eq(a4363, a4364) {
            Ok(Adt9::C4(6u64, 0u64, v6, 2u64, 0u64, 1u64, 1u64))
        } else {
            let a4368 = {
                let a4365 = v0;
                let a4366 = 3u64;
                let a4367 = 0u64;
                nat_rem(a4365, a4366, a4367)
            };
            let a4369 = 1u64;
            if nat_eq(a4368, a4369) {
                Ok(Adt9::C4(10u64, 1u64, 1u64, 1u64, v5, 1u64, 1u64))
            } else {
                Ok(Adt9::C4(7u64, 1u64, 1u64, 2u64, 0u64, 3u64, v7))
            }
        }
    } else {
        Ok(Adt9::C4(10u64, 0u64, v6, 1u64, v5, 3u64, v7))
    }
}

pub fn f174(v0: u64) -> R<u64> {
    let v1: u64 = {
        let a4370 = v0;
        let a4371 = 8388608u64;
        let a4372 = 0u64;
        nat_quot(a4370, a4371, a4372)
    };
    let a4373 = v0;
    let a4374 = 2147483648u64;
    if if if nat_lt(a4373, a4374) {
        let a4375 = 127u64;
        let a4376 = v1;
        nat_le(a4375, a4376)
    } else {
        false
    } {
        let a4377 = v1;
        let a4378 = 190u64;
        nat_lt(a4377, a4378)
    } else {
        false
    } {
        let v2: u64 = {
            let a4382 = 8388608u64;
            let a4383 = {
                let a4379 = v0;
                let a4380 = 8388608u64;
                let a4381 = 0u64;
                nat_rem(a4379, a4380, a4381)
            };
            nat_add(a4382, a4383)?
        };
        let a4384 = v1;
        let a4385 = 150u64;
        if nat_lt(a4384, a4385) {
            let v3: u64 = f237({
                let a4386 = 150u64;
                let a4387 = v1;
                nat_sub(a4386, a4387)
            });
            let a4391 = {
                let a4388 = v2;
                let a4389 = v3;
                let a4390 = 0u64;
                nat_rem(a4388, a4389, a4390)
            };
            let a4392 = 0u64;
            if nat_eq(a4391, a4392) {
                let a4393 = v2;
                let a4394 = v3;
                let a4395 = 0u64;
                Ok(nat_quot(a4393, a4394, a4395))
            } else {
                Ok(0u64)
            }
        } else {
            let a4398 = f237({
                let a4396 = v1;
                let a4397 = 150u64;
                nat_sub(a4396, a4397)
            });
            let a4399 = v2;
            nat_mul(a4398, a4399)
        }
    } else {
        Ok(0u64)
    }
}

pub fn f175(v0: Adt14, v1: Adt14) -> bool {
    let a4402 = match v0.clone() {
        Adt14::C0(_, h4400, _, _, _, _, _, _, _, _, _) => {
            h4400
        }
    };
    let a4403 = match v1.clone() {
        Adt14::C0(_, h4401, _, _, _, _, _, _, _, _, _) => {
            h4401
        }
    };
    if if if nat_eq(a4402, a4403) {
        let a4406 = match v0.clone() {
            Adt14::C0(_, _, h4404, _, _, _, _, _, _, _, _) => {
                h4404
            }
        };
        let a4407 = match v1.clone() {
            Adt14::C0(_, _, h4405, _, _, _, _, _, _, _, _) => {
                h4405
            }
        };
        nat_eq(a4406, a4407)
    } else {
        false
    } {
        let a4410 = match v0.clone() {
            Adt14::C0(_, _, _, h4408, _, _, _, _, _, _, _) => {
                h4408
            }
        };
        let a4411 = match v1.clone() {
            Adt14::C0(_, _, _, h4409, _, _, _, _, _, _, _) => {
                h4409
            }
        };
        nat_eq(a4410, a4411)
    } else {
        false
    } {
        let a4414 = match v0 {
            Adt14::C0(_, _, _, _, h4412, _, _, _, _, _, _) => {
                h4412
            }
        };
        let a4415 = match v1 {
            Adt14::C0(_, _, _, _, h4413, _, _, _, _, _, _) => {
                h4413
            }
        };
        nat_eq(a4414, a4415)
    } else {
        false
    }
}

pub fn f176(v0: u64, v1: u64, v2: u64, v3: u64) -> bool {
    let a4416 = v0;
    let a4417 = 0u64;
    if if if nat_eq(a4416, a4417) {
        true
    } else {
        let a4418 = v1;
        let a4419 = 0u64;
        nat_eq(a4418, a4419)
    } {
        true
    } else {
        let a4420 = v2;
        let a4421 = 0u64;
        nat_eq(a4420, a4421)
    } {
        true
    } else {
        let a4422 = v3;
        let a4423 = 0u64;
        nat_eq(a4422, a4423)
    }
}

pub fn f177(v0: Adt14, v1: Adt14) -> bool {
    if f183(match v0.clone() {
        Adt14::C0(_, _, _, _, _, h4424, _, _, _, _, _) => {
            h4424
        }
    }, match v1 {
        Adt14::C0(_, _, _, _, _, h4425, _, _, _, _, _) => {
            h4425
        }
    }) {
        let a4427 = match v0 {
            Adt14::C0(_, h4426, _, _, _, _, _, _, _, _, _) => {
                h4426
            }
        };
        let a4428 = 2147483647u64;
        nat_le(a4427, a4428)
    } else {
        true
    }
}

pub fn f178(v0: Adt14) -> bool {
    let a4433 = {
        let a4430 = match v0.clone() {
            Adt14::C0(_, _, _, _, _, _, _, _, _, _, h4429) => {
                h4429
            }
        };
        let a4431 = 4u64;
        let a4432 = 0u64;
        nat_rem(a4430, a4431, a4432)
    };
    let a4434 = 0u64;
    if if nat_eq(a4433, a4434) {
        f238(match v0.clone() {
            Adt14::C0(_, h4435, _, _, _, _, _, _, _, _, _) => {
                h4435
            }
        }, match v0.clone() {
            Adt14::C0(_, _, _, _, _, h4436, _, _, _, _, _) => {
                h4436
            }
        })
    } else {
        false
    } {
        f83(match v0.clone() {
            Adt14::C0(_, _, h4437, _, _, _, _, _, _, _, _) => {
                h4437
            }
        }, match v0.clone() {
            Adt14::C0(_, _, _, h4438, _, _, _, _, _, _, _) => {
                h4438
            }
        }, match v0.clone() {
            Adt14::C0(_, _, _, _, h4439, _, _, _, _, _, _) => {
                h4439
            }
        }, match v0.clone() {
            Adt14::C0(_, _, _, _, _, _, h4440, _, _, _, _) => {
                h4440
            }
        }, match v0.clone() {
            Adt14::C0(_, _, _, _, _, _, _, h4441, _, _, _) => {
                h4441
            }
        }, match v0 {
            Adt14::C0(_, _, _, _, _, _, _, _, h4442, _, _) => {
                h4442
            }
        }, 4u64)
    } else {
        false
    }
}

pub fn f179(v0: Adt14, v1: Adt14) -> R<bool> {
    let a4444 = match v0.clone() {
        Adt14::C0(_, _, _, _, _, _, _, _, _, h4443, _) => {
            h4443
        }
    };
    let a4445 = 0u64;
    if nat_eq(a4444, a4445) {
        Ok(true)
    } else {
        let a4447 = match v1.clone() {
            Adt14::C0(_, _, _, _, _, _, _, _, _, h4446, _) => {
                h4446
            }
        };
        let a4448 = 0u64;
        if nat_eq(a4447, a4448) {
            Ok(true)
        } else {
            let a4450 = match v0.clone() {
                Adt14::C0(_, _, _, _, _, _, _, _, _, _, h4449) => {
                    h4449
                }
            };
            let a4451 = 0u64;
            if nat_eq(a4450, a4451) {
                Ok(true)
            } else {
                let a4453 = match v1.clone() {
                    Adt14::C0(_, _, _, _, _, _, _, _, _, _, h4452) => {
                        h4452
                    }
                };
                let a4454 = 0u64;
                if nat_eq(a4453, a4454) {
                    Ok(true)
                } else {
                    let a4460 = {
                        let a4457 = match v0.clone() {
                            Adt14::C0(_, _, _, _, _, _, _, _, _, _, h4455) => {
                                h4455
                            }
                        };
                        let a4458 = match v0.clone() {
                            Adt14::C0(_, _, _, _, _, _, _, _, _, h4456, _) => {
                                h4456
                            }
                        };
                        nat_add(a4457, a4458)?
                    };
                    let a4461 = match v1.clone() {
                        Adt14::C0(_, _, _, _, _, _, _, _, _, _, h4459) => {
                            h4459
                        }
                    };
                    if nat_le(a4460, a4461) {
                        Ok(true)
                    } else {
                        let a4467 = {
                            let a4464 = match v1.clone() {
                                Adt14::C0(_, _, _, _, _, _, _, _, _, _, h4462) => {
                                    h4462
                                }
                            };
                            let a4465 = match v1 {
                                Adt14::C0(_, _, _, _, _, _, _, _, _, h4463, _) => {
                                    h4463
                                }
                            };
                            nat_add(a4464, a4465)?
                        };
                        let a4468 = match v0 {
                            Adt14::C0(_, _, _, _, _, _, _, _, _, _, h4466) => {
                                h4466
                            }
                        };
                        Ok(nat_le(a4467, a4468))
                    }
                }
            }
        }
    }
}

pub fn f180(v0: Adt14, v1: Adt14) -> bool {
    if if if if if if f175(v0.clone(), v1.clone()) {
        let a4471 = match v0.clone() {
            Adt14::C0(h4469, _, _, _, _, _, _, _, _, _, _) => {
                h4469
            }
        };
        let a4472 = match v1.clone() {
            Adt14::C0(h4470, _, _, _, _, _, _, _, _, _, _) => {
                h4470
            }
        };
        nat_eq(a4471, a4472)
    } else {
        false
    } {
        let a4475 = match v0.clone() {
            Adt14::C0(_, _, _, _, _, _, _, _, _, _, h4473) => {
                h4473
            }
        };
        let a4476 = match v1.clone() {
            Adt14::C0(_, _, _, _, _, _, _, _, _, _, h4474) => {
                h4474
            }
        };
        nat_eq(a4475, a4476)
    } else {
        false
    } {
        let a4479 = match v0.clone() {
            Adt14::C0(_, _, _, _, _, h4477, _, _, _, _, _) => {
                h4477
            }
        };
        let a4480 = match v1.clone() {
            Adt14::C0(_, _, _, _, _, h4478, _, _, _, _, _) => {
                h4478
            }
        };
        nat_eq(a4479, a4480)
    } else {
        false
    } {
        let a4483 = match v0.clone() {
            Adt14::C0(_, _, _, _, _, _, h4481, _, _, _, _) => {
                h4481
            }
        };
        let a4484 = match v1.clone() {
            Adt14::C0(_, _, _, _, _, _, h4482, _, _, _, _) => {
                h4482
            }
        };
        nat_eq(a4483, a4484)
    } else {
        false
    } {
        let a4487 = match v0.clone() {
            Adt14::C0(_, _, _, _, _, _, _, h4485, _, _, _) => {
                h4485
            }
        };
        let a4488 = match v1.clone() {
            Adt14::C0(_, _, _, _, _, _, _, h4486, _, _, _) => {
                h4486
            }
        };
        nat_eq(a4487, a4488)
    } else {
        false
    } {
        let a4491 = match v0 {
            Adt14::C0(_, _, _, _, _, _, _, _, h4489, _, _) => {
                h4489
            }
        };
        let a4492 = match v1 {
            Adt14::C0(_, _, _, _, _, _, _, _, h4490, _, _) => {
                h4490
            }
        };
        nat_eq(a4491, a4492)
    } else {
        false
    }
}

pub fn f181(v0: Adt14) -> R<bool> {
    if f239(match v0.clone() {
        Adt14::C0(_, h4493, _, _, _, _, _, _, _, _, _) => {
            h4493
        }
    }, match v0.clone() {
        Adt14::C0(_, _, _, _, _, h4494, _, _, _, _, _) => {
            h4494
        }
    }) {
        f84(f240(match v0.clone() {
            Adt14::C0(_, h4495, _, _, _, _, _, _, _, _, _) => {
                h4495
            }
        }, match v0.clone() {
            Adt14::C0(_, _, _, _, _, h4496, _, _, _, _, _) => {
                h4496
            }
        })?, match v0.clone() {
            Adt14::C0(_, _, h4497, _, _, _, _, _, _, _, _) => {
                h4497
            }
        }, match v0.clone() {
            Adt14::C0(_, _, _, h4498, _, _, _, _, _, _, _) => {
                h4498
            }
        }, match v0.clone() {
            Adt14::C0(_, _, _, _, h4499, _, _, _, _, _, _) => {
                h4499
            }
        }, match v0.clone() {
            Adt14::C0(_, _, _, _, _, _, h4500, _, _, _, _) => {
                h4500
            }
        }, match v0.clone() {
            Adt14::C0(_, _, _, _, _, _, _, h4501, _, _, _) => {
                h4501
            }
        }, match v0 {
            Adt14::C0(_, _, _, _, _, _, _, _, h4502, _, _) => {
                h4502
            }
        })
    } else {
        Ok(false)
    }
}

pub fn f182(v0: bool, v1: u64, v2: u64, v3: u64) -> R<u64> {
    let a4503 = v1;
    let a4504 = 0u64;
    if if nat_eq(a4503, a4504) {
        true
    } else {
        let a4505 = v2;
        let a4506 = 0u64;
        nat_eq(a4505, a4506)
    } {
        Ok(0u64)
    } else {
        let a4509 = f85(v1, v2, v3)?;
        let a4510 = {
            let a4507 = v2;
            let a4508 = f241(v0, v1)?;
            nat_mul(a4507, a4508)?
        };
        nat_add(a4509, a4510)
    }
}

pub fn f183(v0: u64, v1: u64) -> bool {
    let a4511 = v0;
    let a4512 = 4u64;
    if nat_eq(a4511, a4512) {
        let a4513 = v1;
        let a4514 = 4u64;
        nat_eq(a4513, a4514)
    } else {
        false
    }
}

pub fn f184(v0: u64) -> R<u64> {
    let a4515 = 6u64;
    let a4516 = v0;
    nat_add(a4515, a4516)
}

pub fn f185(v0: Adt14) -> Adt14 {
    Adt14::C0(match v0.clone() {
        Adt14::C0(h4517, _, _, _, _, _, _, _, _, _, _) => {
            h4517
        }
    }, match v0.clone() {
        Adt14::C0(_, h4518, _, _, _, _, _, _, _, _, _) => {
            h4518
        }
    }, match v0.clone() {
        Adt14::C0(_, _, h4519, _, _, _, _, _, _, _, _) => {
            h4519
        }
    }, match v0.clone() {
        Adt14::C0(_, _, _, h4520, _, _, _, _, _, _, _) => {
            h4520
        }
    }, match v0.clone() {
        Adt14::C0(_, _, _, _, h4521, _, _, _, _, _, _) => {
            h4521
        }
    }, 4u64, match v0.clone() {
        Adt14::C0(_, _, _, _, _, _, h4522, _, _, _, _) => {
            h4522
        }
    }, match v0.clone() {
        Adt14::C0(_, _, _, _, _, _, _, h4523, _, _, _) => {
            h4523
        }
    }, match v0.clone() {
        Adt14::C0(_, _, _, _, _, _, _, _, h4524, _, _) => {
            h4524
        }
    }, match v0.clone() {
        Adt14::C0(_, _, _, _, _, _, _, _, _, h4525, _) => {
            h4525
        }
    }, match v0 {
        Adt14::C0(_, _, _, _, _, _, _, _, _, _, h4526) => {
            h4526
        }
    })
}

pub fn f186(v0: u64, v1: u64, v2: u64, v3: u64) -> R<bool> {
    let a4527 = v0;
    let a4528 = 2147483647u64;
    if if if nat_le(a4527, a4528) {
        let a4529 = 0u64;
        let a4530 = v3;
        if nat_lt(a4529, a4530) {
            let a4531 = v3;
            let a4532 = 2147483647u64;
            nat_le(a4531, a4532)
        } else {
            false
        }
    } else {
        false
    } {
        let a4537 = {
            let a4535 = {
                let a4533 = v2;
                let a4534 = v3;
                nat_add(a4533, a4534)?
            };
            let a4536 = 1u64;
            nat_sub(a4535, a4536)
        };
        let a4538 = 2147483647u64;
        nat_le(a4537, a4538)
    } else {
        false
    } {
        let a4543 = {
            let a4541 = {
                let a4539 = v0;
                let a4540 = v1;
                nat_mul(a4539, a4540)?
            };
            let a4542 = f242(v2, v3)?;
            nat_mul(a4541, a4542)?
        };
        let a4544 = 9223372036854775807u64;
        Ok(nat_le(a4543, a4544))
    } else {
        Ok(false)
    }
}

pub fn f187(v0: Adt14) -> R<u64> {
    let a4575 = {
        let a4567 = {
            let a4559 = {
                let a4551 = 4u64;
                let a4552 = {
                    let a4549 = {
                        let a4546 = match v0.clone() {
                            Adt14::C0(_, h4545, _, _, _, _, _, _, _, _, _) => {
                                h4545
                            }
                        };
                        let a4547 = 1u64;
                        nat_sub(a4546, a4547)
                    };
                    let a4550 = match v0.clone() {
                        Adt14::C0(_, _, _, _, _, h4548, _, _, _, _, _) => {
                            h4548
                        }
                    };
                    nat_mul(a4549, a4550)?
                };
                nat_add(a4551, a4552)?
            };
            let a4560 = {
                let a4557 = {
                    let a4554 = match v0.clone() {
                        Adt14::C0(_, _, h4553, _, _, _, _, _, _, _, _) => {
                            h4553
                        }
                    };
                    let a4555 = 1u64;
                    nat_sub(a4554, a4555)
                };
                let a4558 = match v0.clone() {
                    Adt14::C0(_, _, _, _, _, _, h4556, _, _, _, _) => {
                        h4556
                    }
                };
                nat_mul(a4557, a4558)?
            };
            nat_add(a4559, a4560)?
        };
        let a4568 = {
            let a4565 = {
                let a4562 = match v0.clone() {
                    Adt14::C0(_, _, _, h4561, _, _, _, _, _, _, _) => {
                        h4561
                    }
                };
                let a4563 = 1u64;
                nat_sub(a4562, a4563)
            };
            let a4566 = match v0.clone() {
                Adt14::C0(_, _, _, _, _, _, _, h4564, _, _, _) => {
                    h4564
                }
            };
            nat_mul(a4565, a4566)?
        };
        nat_add(a4567, a4568)?
    };
    let a4576 = {
        let a4573 = {
            let a4570 = match v0.clone() {
                Adt14::C0(_, _, _, _, h4569, _, _, _, _, _, _) => {
                    h4569
                }
            };
            let a4571 = 1u64;
            nat_sub(a4570, a4571)
        };
        let a4574 = match v0 {
            Adt14::C0(_, _, _, _, _, _, _, _, h4572, _, _) => {
                h4572
            }
        };
        nat_mul(a4573, a4574)?
    };
    nat_add(a4575, a4576)
}

pub fn f188(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64) -> R<u64> {
    let a4577 = v0;
    let a4578 = 0u64;
    if if if if nat_eq(a4577, a4578) {
        true
    } else {
        let a4579 = v1;
        let a4580 = 0u64;
        nat_eq(a4579, a4580)
    } {
        true
    } else {
        let a4581 = v2;
        let a4582 = 0u64;
        nat_eq(a4581, a4582)
    } {
        true
    } else {
        let a4583 = v3;
        let a4584 = 0u64;
        nat_eq(a4583, a4584)
    } {
        Ok(0u64)
    } else {
        let a4585 = f243(v0, v1, v2, v3, v5)?;
        let a4586 = f244(v0, v1, v2, v3, v4)?;
        nat_add(a4585, a4586)
    }
}

pub fn f189(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let a4587 = v0;
    let a4588 = 0u64;
    if if nat_eq(a4587, a4588) {
        true
    } else {
        let a4589 = v1;
        let a4590 = 0u64;
        nat_eq(a4589, a4590)
    } {
        Ok(0u64)
    } else {
        let a4593 = f85(v0, v1, v2)?;
        let a4594 = {
            let a4591 = v1;
            let a4592 = f245(v0)?;
            nat_mul(a4591, a4592)?
        };
        nat_add(a4593, a4594)
    }
}

pub fn f190(v0: u64, v1: u64, v2: u64) -> bool {
    let a4595 = v0;
    let a4596 = 1u64;
    if nat_le(a4595, a4596) {
        true
    } else {
        let a4600 = {
            let a4597 = v1;
            let a4598 = v2;
            let a4599 = 1u64;
            nat_rem(a4597, a4598, a4599)
        };
        let a4601 = 0u64;
        nat_eq(a4600, a4601)
    }
}

pub fn f191(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64) -> R<bool> {
    let a4602 = v1;
    let a4603 = 1u64;
    if if if nat_le(a4602, a4603) {
        true
    } else {
        let a4604 = v0;
        let a4605 = v2;
        nat_le(a4604, a4605)
    } {
        let a4606 = v3;
        let a4607 = 1u64;
        if nat_le(a4606, a4607) {
            true
        } else {
            let a4614 = {
                let a4612 = v0;
                let a4613 = {
                    let a4610 = {
                        let a4608 = v1;
                        let a4609 = 1u64;
                        nat_sub(a4608, a4609)
                    };
                    let a4611 = v2;
                    nat_mul(a4610, a4611)?
                };
                nat_add(a4612, a4613)?
            };
            let a4615 = v4;
            nat_le(a4614, a4615)
        }
    } else {
        false
    } {
        let a4616 = v5;
        let a4617 = 1u64;
        if nat_le(a4616, a4617) {
            Ok(true)
        } else {
            let a4630 = {
                let a4628 = {
                    let a4622 = v0;
                    let a4623 = {
                        let a4620 = {
                            let a4618 = v1;
                            let a4619 = 1u64;
                            nat_sub(a4618, a4619)
                        };
                        let a4621 = v2;
                        nat_mul(a4620, a4621)?
                    };
                    nat_add(a4622, a4623)?
                };
                let a4629 = {
                    let a4626 = {
                        let a4624 = v3;
                        let a4625 = 1u64;
                        nat_sub(a4624, a4625)
                    };
                    let a4627 = v4;
                    nat_mul(a4626, a4627)?
                };
                nat_add(a4628, a4629)?
            };
            let a4631 = v6;
            Ok(nat_le(a4630, a4631))
        }
    } else {
        Ok(false)
    }
}

pub fn f192(v0: u64) -> Adt18 {
    let a4632 = v0;
    let a4633 = 0u64;
    if nat_eq(a4632, a4633) {
        Adt18::C1(Adt19::C0(0u64, 1u64, 4u64))
    } else {
        let a4634 = v0;
        let a4635 = 1u64;
        if nat_eq(a4634, a4635) {
            Adt18::C1(Adt19::C0(1u64, 1u64, 2u64))
        } else {
            let a4636 = v0;
            let a4637 = 2u64;
            if nat_eq(a4636, a4637) {
                Adt18::C1(Adt19::C0(2u64, 32u64, 18u64))
            } else {
                let a4638 = v0;
                let a4639 = 3u64;
                if nat_eq(a4638, a4639) {
                    Adt18::C1(Adt19::C0(3u64, 32u64, 20u64))
                } else {
                    let a4640 = v0;
                    let a4641 = 6u64;
                    if nat_eq(a4640, a4641) {
                        Adt18::C1(Adt19::C0(6u64, 32u64, 22u64))
                    } else {
                        let a4642 = v0;
                        let a4643 = 7u64;
                        if nat_eq(a4642, a4643) {
                            Adt18::C1(Adt19::C0(7u64, 32u64, 24u64))
                        } else {
                            let a4644 = v0;
                            let a4645 = 8u64;
                            if nat_eq(a4644, a4645) {
                                Adt18::C1(Adt19::C0(8u64, 32u64, 34u64))
                            } else {
                                let a4646 = v0;
                                let a4647 = 9u64;
                                if nat_eq(a4646, a4647) {
                                    Adt18::C1(Adt19::C0(9u64, 32u64, 36u64))
                                } else {
                                    let a4648 = v0;
                                    let a4649 = 10u64;
                                    if nat_eq(a4648, a4649) {
                                        Adt18::C1(Adt19::C0(10u64, 256u64, 84u64))
                                    } else {
                                        let a4650 = v0;
                                        let a4651 = 11u64;
                                        if nat_eq(a4650, a4651) {
                                            Adt18::C1(Adt19::C0(11u64, 256u64, 110u64))
                                        } else {
                                            let a4652 = v0;
                                            let a4653 = 12u64;
                                            if nat_eq(a4652, a4653) {
                                                Adt18::C1(Adt19::C0(12u64, 256u64, 144u64))
                                            } else {
                                                let a4654 = v0;
                                                let a4655 = 13u64;
                                                if nat_eq(a4654, a4655) {
                                                    Adt18::C1(Adt19::C0(13u64, 256u64, 176u64))
                                                } else {
                                                    let a4656 = v0;
                                                    let a4657 = 14u64;
                                                    if nat_eq(a4656, a4657) {
                                                        Adt18::C1(Adt19::C0(14u64, 256u64, 210u64))
                                                    } else {
                                                        let a4658 = v0;
                                                        let a4659 = 15u64;
                                                        if nat_eq(a4658, a4659) {
                                                            Adt18::C1(Adt19::C0(15u64, 256u64, 292u64))
                                                        } else {
                                                            let a4660 = v0;
                                                            let a4661 = 16u64;
                                                            if nat_eq(a4660, a4661) {
                                                                Adt18::C1(Adt19::C0(16u64, 256u64, 66u64))
                                                            } else {
                                                                let a4662 = v0;
                                                                let a4663 = 17u64;
                                                                if nat_eq(a4662, a4663) {
                                                                    Adt18::C1(Adt19::C0(17u64, 256u64, 74u64))
                                                                } else {
                                                                    let a4664 = v0;
                                                                    let a4665 = 18u64;
                                                                    if nat_eq(a4664, a4665) {
                                                                        Adt18::C1(Adt19::C0(18u64, 256u64, 98u64))
                                                                    } else {
                                                                        let a4666 = v0;
                                                                        let a4667 = 19u64;
                                                                        if nat_eq(a4666, a4667) {
                                                                            Adt18::C1(Adt19::C0(19u64, 256u64, 50u64))
                                                                        } else {
                                                                            let a4668 = v0;
                                                                            let a4669 = 20u64;
                                                                            if nat_eq(a4668, a4669) {
                                                                                Adt18::C1(Adt19::C0(20u64, 32u64, 18u64))
                                                                            } else {
                                                                                let a4670 = v0;
                                                                                let a4671 = 21u64;
                                                                                if nat_eq(a4670, a4671) {
                                                                                    Adt18::C1(Adt19::C0(21u64, 256u64, 110u64))
                                                                                } else {
                                                                                    let a4672 = v0;
                                                                                    let a4673 = 22u64;
                                                                                    if nat_eq(a4672, a4673) {
                                                                                        Adt18::C1(Adt19::C0(22u64, 256u64, 82u64))
                                                                                    } else {
                                                                                        let a4674 = v0;
                                                                                        let a4675 = 23u64;
                                                                                        if nat_eq(a4674, a4675) {
                                                                                            Adt18::C1(Adt19::C0(23u64, 256u64, 136u64))
                                                                                        } else {
                                                                                            let a4676 = v0;
                                                                                            let a4677 = 24u64;
                                                                                            if nat_eq(a4676, a4677) {
                                                                                                Adt18::C1(Adt19::C0(24u64, 1u64, 1u64))
                                                                                            } else {
                                                                                                let a4678 = v0;
                                                                                                let a4679 = 25u64;
                                                                                                if nat_eq(a4678, a4679) {
                                                                                                    Adt18::C1(Adt19::C0(25u64, 1u64, 2u64))
                                                                                                } else {
                                                                                                    let a4680 = v0;
                                                                                                    let a4681 = 26u64;
                                                                                                    if nat_eq(a4680, a4681) {
                                                                                                        Adt18::C1(Adt19::C0(26u64, 1u64, 4u64))
                                                                                                    } else {
                                                                                                        let a4682 = v0;
                                                                                                        let a4683 = 27u64;
                                                                                                        if nat_eq(a4682, a4683) {
                                                                                                            Adt18::C1(Adt19::C0(27u64, 1u64, 8u64))
                                                                                                        } else {
                                                                                                            let a4684 = v0;
                                                                                                            let a4685 = 28u64;
                                                                                                            if nat_eq(a4684, a4685) {
                                                                                                                Adt18::C1(Adt19::C0(28u64, 1u64, 8u64))
                                                                                                            } else {
                                                                                                                let a4686 = v0;
                                                                                                                let a4687 = 29u64;
                                                                                                                if nat_eq(a4686, a4687) {
                                                                                                                    Adt18::C1(Adt19::C0(29u64, 256u64, 56u64))
                                                                                                                } else {
                                                                                                                    let a4688 = v0;
                                                                                                                    let a4689 = 30u64;
                                                                                                                    if nat_eq(a4688, a4689) {
                                                                                                                        Adt18::C1(Adt19::C0(30u64, 1u64, 2u64))
                                                                                                                    } else {
                                                                                                                        let a4690 = v0;
                                                                                                                        let a4691 = 34u64;
                                                                                                                        if nat_eq(a4690, a4691) {
                                                                                                                            Adt18::C1(Adt19::C0(34u64, 256u64, 54u64))
                                                                                                                        } else {
                                                                                                                            let a4692 = v0;
                                                                                                                            let a4693 = 35u64;
                                                                                                                            if nat_eq(a4692, a4693) {
                                                                                                                                Adt18::C1(Adt19::C0(35u64, 256u64, 66u64))
                                                                                                                            } else {
                                                                                                                                let a4694 = v0;
                                                                                                                                let a4695 = 39u64;
                                                                                                                                if nat_eq(a4694, a4695) {
                                                                                                                                    Adt18::C1(Adt19::C0(39u64, 32u64, 17u64))
                                                                                                                                } else {
                                                                                                                                    let a4696 = v0;
                                                                                                                                    let a4697 = 40u64;
                                                                                                                                    if nat_eq(a4696, a4697) {
                                                                                                                                        Adt18::C1(Adt19::C0(40u64, 64u64, 36u64))
                                                                                                                                    } else {
                                                                                                                                        let a4698 = v0;
                                                                                                                                        let a4699 = 41u64;
                                                                                                                                        if nat_eq(a4698, a4699) {
                                                                                                                                            Adt18::C1(Adt19::C0(41u64, 128u64, 18u64))
                                                                                                                                        } else {
                                                                                                                                            let a4700 = v0;
                                                                                                                                            let a4701 = 42u64;
                                                                                                                                            if nat_eq(a4700, a4701) {
                                                                                                                                                Adt18::C1(Adt19::C0(42u64, 64u64, 18u64))
                                                                                                                                            } else {
                                                                                                                                                let a4702 = v0;
                                                                                                                                                let a4703 = 43u64;
                                                                                                                                                if nat_eq(a4702, a4703) {
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

pub fn f193(v0: Adt2, v1: u64, v2: bool, v3: bool) -> R<bool> {
    if v2 {
        if v3 {
            let a4705 = match v0.clone() {
                Adt2::C0(_, _, _, _, h4704, _, _, _, _, _, _) => {
                    h4704
                }
            };
            let a4706 = 1u64;
            if if if if if nat_le(a4705, a4706) {
                true
            } else {
                let a4708 = match v0.clone() {
                    Adt2::C0(_, _, _, _, _, _, _, _, h4707, _, _) => {
                        h4707
                    }
                };
                let a4709 = v1;
                nat_eq(a4708, a4709)
            } {
                let a4711 = match v0.clone() {
                    Adt2::C0(_, h4710, _, _, _, _, _, _, _, _, _) => {
                        h4710
                    }
                };
                let a4712 = 1u64;
                if nat_le(a4711, a4712) {
                    true
                } else {
                    let a4717 = match v0.clone() {
                        Adt2::C0(_, _, _, _, _, h4713, _, _, _, _, _) => {
                            h4713
                        }
                    };
                    let a4718 = {
                        let a4715 = v1;
                        let a4716 = match v0.clone() {
                            Adt2::C0(_, _, _, _, h4714, _, _, _, _, _, _) => {
                                h4714
                            }
                        };
                        nat_mul(a4715, a4716)?
                    };
                    nat_eq(a4717, a4718)
                }
            } else {
                false
            } {
                let a4720 = match v0.clone() {
                    Adt2::C0(_, _, h4719, _, _, _, _, _, _, _, _) => {
                        h4719
                    }
                };
                let a4721 = 1u64;
                if nat_le(a4720, a4721) {
                    true
                } else {
                    let a4729 = match v0.clone() {
                        Adt2::C0(_, _, _, _, _, _, h4722, _, _, _, _) => {
                            h4722
                        }
                    };
                    let a4730 = {
                        let a4727 = {
                            let a4724 = v1;
                            let a4725 = match v0.clone() {
                                Adt2::C0(_, _, _, _, h4723, _, _, _, _, _, _) => {
                                    h4723
                                }
                            };
                            nat_mul(a4724, a4725)?
                        };
                        let a4728 = match v0.clone() {
                            Adt2::C0(_, h4726, _, _, _, _, _, _, _, _, _) => {
                                h4726
                            }
                        };
                        nat_mul(a4727, a4728)?
                    };
                    nat_eq(a4729, a4730)
                }
            } else {
                false
            } {
                let a4732 = match v0.clone() {
                    Adt2::C0(_, _, _, h4731, _, _, _, _, _, _, _) => {
                        h4731
                    }
                };
                let a4733 = 1u64;
                if nat_le(a4732, a4733) {
                    true
                } else {
                    let a4744 = match v0.clone() {
                        Adt2::C0(_, _, _, _, _, _, _, h4734, _, _, _) => {
                            h4734
                        }
                    };
                    let a4745 = {
                        let a4742 = {
                            let a4739 = {
                                let a4736 = v1;
                                let a4737 = match v0.clone() {
                                    Adt2::C0(_, _, _, _, h4735, _, _, _, _, _, _) => {
                                        h4735
                                    }
                                };
                                nat_mul(a4736, a4737)?
                            };
                            let a4740 = match v0.clone() {
                                Adt2::C0(_, h4738, _, _, _, _, _, _, _, _, _) => {
                                    h4738
                                }
                            };
                            nat_mul(a4739, a4740)?
                        };
                        let a4743 = match v0.clone() {
                            Adt2::C0(_, _, h4741, _, _, _, _, _, _, _, _) => {
                                h4741
                            }
                        };
                        nat_mul(a4742, a4743)?
                    };
                    nat_eq(a4744, a4745)
                }
            } else {
                false
            } {
                let a4759 = {
                    let a4756 = {
                        let a4753 = {
                            let a4750 = {
                                let a4747 = v1;
                                let a4748 = match v0.clone() {
                                    Adt2::C0(_, _, _, _, h4746, _, _, _, _, _, _) => {
                                        h4746
                                    }
                                };
                                nat_mul(a4747, a4748)?
                            };
                            let a4751 = match v0.clone() {
                                Adt2::C0(_, h4749, _, _, _, _, _, _, _, _, _) => {
                                    h4749
                                }
                            };
                            nat_mul(a4750, a4751)?
                        };
                        let a4754 = match v0.clone() {
                            Adt2::C0(_, _, h4752, _, _, _, _, _, _, _, _) => {
                                h4752
                            }
                        };
                        nat_mul(a4753, a4754)?
                    };
                    let a4757 = match v0.clone() {
                        Adt2::C0(_, _, _, h4755, _, _, _, _, _, _, _) => {
                            h4755
                        }
                    };
                    nat_mul(a4756, a4757)?
                };
                let a4760 = match v0 {
                    Adt2::C0(_, _, _, _, _, _, _, _, _, h4758, _) => {
                        h4758
                    }
                };
                Ok(nat_le(a4759, a4760))
            } else {
                Ok(false)
            }
        } else {
            let a4762 = match v0.clone() {
                Adt2::C0(_, _, _, h4761, _, _, _, _, _, _, _) => {
                    h4761
                }
            };
            let a4763 = 1u64;
            if if if if if nat_le(a4762, a4763) {
                true
            } else {
                let a4765 = match v0.clone() {
                    Adt2::C0(_, _, _, _, _, _, _, h4764, _, _, _) => {
                        h4764
                    }
                };
                let a4766 = v1;
                nat_eq(a4765, a4766)
            } {
                let a4768 = match v0.clone() {
                    Adt2::C0(_, h4767, _, _, _, _, _, _, _, _, _) => {
                        h4767
                    }
                };
                let a4769 = 1u64;
                if nat_le(a4768, a4769) {
                    true
                } else {
                    let a4774 = match v0.clone() {
                        Adt2::C0(_, _, _, _, _, h4770, _, _, _, _, _) => {
                            h4770
                        }
                    };
                    let a4775 = {
                        let a4772 = v1;
                        let a4773 = match v0.clone() {
                            Adt2::C0(_, _, _, h4771, _, _, _, _, _, _, _) => {
                                h4771
                            }
                        };
                        nat_mul(a4772, a4773)?
                    };
                    nat_eq(a4774, a4775)
                }
            } else {
                false
            } {
                let a4777 = match v0.clone() {
                    Adt2::C0(_, _, h4776, _, _, _, _, _, _, _, _) => {
                        h4776
                    }
                };
                let a4778 = 1u64;
                if nat_le(a4777, a4778) {
                    true
                } else {
                    let a4786 = match v0.clone() {
                        Adt2::C0(_, _, _, _, _, _, h4779, _, _, _, _) => {
                            h4779
                        }
                    };
                    let a4787 = {
                        let a4784 = {
                            let a4781 = v1;
                            let a4782 = match v0.clone() {
                                Adt2::C0(_, _, _, h4780, _, _, _, _, _, _, _) => {
                                    h4780
                                }
                            };
                            nat_mul(a4781, a4782)?
                        };
                        let a4785 = match v0.clone() {
                            Adt2::C0(_, h4783, _, _, _, _, _, _, _, _, _) => {
                                h4783
                            }
                        };
                        nat_mul(a4784, a4785)?
                    };
                    nat_eq(a4786, a4787)
                }
            } else {
                false
            } {
                let a4789 = match v0.clone() {
                    Adt2::C0(_, _, _, _, h4788, _, _, _, _, _, _) => {
                        h4788
                    }
                };
                let a4790 = 1u64;
                if nat_le(a4789, a4790) {
                    true
                } else {
                    let a4801 = match v0.clone() {
                        Adt2::C0(_, _, _, _, _, _, _, _, h4791, _, _) => {
                            h4791
                        }
                    };
                    let a4802 = {
                        let a4799 = {
                            let a4796 = {
                                let a4793 = v1;
                                let a4794 = match v0.clone() {
                                    Adt2::C0(_, _, _, h4792, _, _, _, _, _, _, _) => {
                                        h4792
                                    }
                                };
                                nat_mul(a4793, a4794)?
                            };
                            let a4797 = match v0.clone() {
                                Adt2::C0(_, h4795, _, _, _, _, _, _, _, _, _) => {
                                    h4795
                                }
                            };
                            nat_mul(a4796, a4797)?
                        };
                        let a4800 = match v0.clone() {
                            Adt2::C0(_, _, h4798, _, _, _, _, _, _, _, _) => {
                                h4798
                            }
                        };
                        nat_mul(a4799, a4800)?
                    };
                    nat_eq(a4801, a4802)
                }
            } else {
                false
            } {
                let a4816 = {
                    let a4813 = {
                        let a4810 = {
                            let a4807 = {
                                let a4804 = v1;
                                let a4805 = match v0.clone() {
                                    Adt2::C0(_, _, _, h4803, _, _, _, _, _, _, _) => {
                                        h4803
                                    }
                                };
                                nat_mul(a4804, a4805)?
                            };
                            let a4808 = match v0.clone() {
                                Adt2::C0(_, h4806, _, _, _, _, _, _, _, _, _) => {
                                    h4806
                                }
                            };
                            nat_mul(a4807, a4808)?
                        };
                        let a4811 = match v0.clone() {
                            Adt2::C0(_, _, h4809, _, _, _, _, _, _, _, _) => {
                                h4809
                            }
                        };
                        nat_mul(a4810, a4811)?
                    };
                    let a4814 = match v0.clone() {
                        Adt2::C0(_, _, _, _, h4812, _, _, _, _, _, _) => {
                            h4812
                        }
                    };
                    nat_mul(a4813, a4814)?
                };
                let a4817 = match v0 {
                    Adt2::C0(_, _, _, _, _, _, _, _, _, h4815, _) => {
                        h4815
                    }
                };
                Ok(nat_le(a4816, a4817))
            } else {
                Ok(false)
            }
        }
    } else {
        let a4819 = match v0.clone() {
            Adt2::C0(_, h4818, _, _, _, _, _, _, _, _, _) => {
                h4818
            }
        };
        let a4820 = 1u64;
        if if if if if nat_le(a4819, a4820) {
            true
        } else {
            let a4822 = match v0.clone() {
                Adt2::C0(_, _, _, _, _, h4821, _, _, _, _, _) => {
                    h4821
                }
            };
            let a4823 = v1;
            nat_eq(a4822, a4823)
        } {
            let a4825 = match v0.clone() {
                Adt2::C0(_, _, h4824, _, _, _, _, _, _, _, _) => {
                    h4824
                }
            };
            let a4826 = 1u64;
            if nat_le(a4825, a4826) {
                true
            } else {
                let a4831 = match v0.clone() {
                    Adt2::C0(_, _, _, _, _, _, h4827, _, _, _, _) => {
                        h4827
                    }
                };
                let a4832 = {
                    let a4829 = v1;
                    let a4830 = match v0.clone() {
                        Adt2::C0(_, h4828, _, _, _, _, _, _, _, _, _) => {
                            h4828
                        }
                    };
                    nat_mul(a4829, a4830)?
                };
                nat_eq(a4831, a4832)
            }
        } else {
            false
        } {
            let a4834 = match v0.clone() {
                Adt2::C0(_, _, _, h4833, _, _, _, _, _, _, _) => {
                    h4833
                }
            };
            let a4835 = 1u64;
            if nat_le(a4834, a4835) {
                true
            } else {
                let a4843 = match v0.clone() {
                    Adt2::C0(_, _, _, _, _, _, _, h4836, _, _, _) => {
                        h4836
                    }
                };
                let a4844 = {
                    let a4841 = {
                        let a4838 = v1;
                        let a4839 = match v0.clone() {
                            Adt2::C0(_, h4837, _, _, _, _, _, _, _, _, _) => {
                                h4837
                            }
                        };
                        nat_mul(a4838, a4839)?
                    };
                    let a4842 = match v0.clone() {
                        Adt2::C0(_, _, h4840, _, _, _, _, _, _, _, _) => {
                            h4840
                        }
                    };
                    nat_mul(a4841, a4842)?
                };
                nat_eq(a4843, a4844)
            }
        } else {
            false
        } {
            let a4846 = match v0.clone() {
                Adt2::C0(_, _, _, _, h4845, _, _, _, _, _, _) => {
                    h4845
                }
            };
            let a4847 = 1u64;
            if nat_le(a4846, a4847) {
                true
            } else {
                let a4858 = match v0.clone() {
                    Adt2::C0(_, _, _, _, _, _, _, _, h4848, _, _) => {
                        h4848
                    }
                };
                let a4859 = {
                    let a4856 = {
                        let a4853 = {
                            let a4850 = v1;
                            let a4851 = match v0.clone() {
                                Adt2::C0(_, h4849, _, _, _, _, _, _, _, _, _) => {
                                    h4849
                                }
                            };
                            nat_mul(a4850, a4851)?
                        };
                        let a4854 = match v0.clone() {
                            Adt2::C0(_, _, h4852, _, _, _, _, _, _, _, _) => {
                                h4852
                            }
                        };
                        nat_mul(a4853, a4854)?
                    };
                    let a4857 = match v0.clone() {
                        Adt2::C0(_, _, _, h4855, _, _, _, _, _, _, _) => {
                            h4855
                        }
                    };
                    nat_mul(a4856, a4857)?
                };
                nat_eq(a4858, a4859)
            }
        } else {
            false
        } {
            let a4873 = {
                let a4870 = {
                    let a4867 = {
                        let a4864 = {
                            let a4861 = v1;
                            let a4862 = match v0.clone() {
                                Adt2::C0(_, h4860, _, _, _, _, _, _, _, _, _) => {
                                    h4860
                                }
                            };
                            nat_mul(a4861, a4862)?
                        };
                        let a4865 = match v0.clone() {
                            Adt2::C0(_, _, h4863, _, _, _, _, _, _, _, _) => {
                                h4863
                            }
                        };
                        nat_mul(a4864, a4865)?
                    };
                    let a4868 = match v0.clone() {
                        Adt2::C0(_, _, _, h4866, _, _, _, _, _, _, _) => {
                            h4866
                        }
                    };
                    nat_mul(a4867, a4868)?
                };
                let a4871 = match v0.clone() {
                    Adt2::C0(_, _, _, _, h4869, _, _, _, _, _, _) => {
                        h4869
                    }
                };
                nat_mul(a4870, a4871)?
            };
            let a4874 = match v0 {
                Adt2::C0(_, _, _, _, _, _, _, _, _, h4872, _) => {
                    h4872
                }
            };
            Ok(nat_le(a4873, a4874))
        } else {
            Ok(false)
        }
    }
}

pub fn f194(v0: Adt2, v1: u64) -> R<bool> {
    let a4876 = match v0.clone() {
        Adt2::C0(h4875, _, _, _, _, _, _, _, _, _, _) => {
            h4875
        }
    };
    let a4877 = 0u64;
    if if nat_eq(a4876, a4877) {
        f28(v0.clone())?
    } else {
        false
    } {
        let a4878 = v1;
        let a4879 = 1u64;
        if nat_le(a4878, a4879) {
            Ok(true)
        } else {
            let a4884 = match v0.clone() {
                Adt2::C0(_, _, _, _, _, _, h4880, _, _, _, _) => {
                    h4880
                }
            };
            let a4885 = {
                let a4882 = 4u64;
                let a4883 = match v0 {
                    Adt2::C0(_, h4881, _, _, _, _, _, _, _, _, _) => {
                        h4881
                    }
                };
                nat_mul(a4882, a4883)?
            };
            Ok(nat_eq(a4884, a4885))
        }
    } else {
        Ok(false)
    }
}

pub fn f195(v0: Adt2, v1: u64) -> R<bool> {
    let a4887 = match v0.clone() {
        Adt2::C0(_, _, _, h4886, _, _, _, _, _, _, _) => {
            h4886
        }
    };
    let a4888 = 1u64;
    if if if nat_eq(a4887, a4888) {
        let a4890 = match v0.clone() {
            Adt2::C0(_, _, _, _, h4889, _, _, _, _, _, _) => {
                h4889
            }
        };
        let a4891 = 1u64;
        nat_eq(a4890, a4891)
    } else {
        false
    } {
        let a4893 = match v0.clone() {
            Adt2::C0(_, _, _, _, _, h4892, _, _, _, _, _) => {
                h4892
            }
        };
        let a4894 = v1;
        nat_eq(a4893, a4894)
    } else {
        false
    } {
        let a4896 = match v0.clone() {
            Adt2::C0(_, _, h4895, _, _, _, _, _, _, _, _) => {
                h4895
            }
        };
        let a4897 = 1u64;
        if nat_eq(a4896, a4897) {
            Ok(true)
        } else {
            let a4902 = match v0.clone() {
                Adt2::C0(_, _, _, _, _, _, h4898, _, _, _, _) => {
                    h4898
                }
            };
            let a4903 = {
                let a4900 = match v0 {
                    Adt2::C0(_, h4899, _, _, _, _, _, _, _, _, _) => {
                        h4899
                    }
                };
                let a4901 = v1;
                nat_mul(a4900, a4901)?
            };
            Ok(nat_eq(a4902, a4903))
        }
    } else {
        Ok(false)
    }
}

pub fn f196(v0: Adt6, v1: u64, v2: u64) -> R<bool> {
    let v3: u64 = {
        let a4907 = {
            let a4904 = v2;
            let a4905 = 1u64;
            nat_sub(a4904, a4905)
        };
        let a4908 = match v0.clone() {
            Adt6::C0(_, h4906, _, _) => {
                h4906
            }
        };
        nat_mul(a4907, a4908)?
    };
    let a4910 = 0u64;
    let a4911 = match v0.clone() {
        Adt6::C0(h4909, _, _, _) => {
            h4909
        }
    };
    if if if if if if if if nat_lt(a4910, a4911) {
        let a4913 = match v0.clone() {
            Adt6::C0(h4912, _, _, _) => {
                h4912
            }
        };
        let a4914 = 2147483648u64;
        nat_lt(a4913, a4914)
    } else {
        false
    } {
        let a4916 = 0u64;
        let a4917 = match v0.clone() {
            Adt6::C0(_, h4915, _, _) => {
                h4915
            }
        };
        nat_lt(a4916, a4917)
    } else {
        false
    } {
        let a4919 = match v0.clone() {
            Adt6::C0(_, h4918, _, _) => {
                h4918
            }
        };
        let a4920 = 2147483648u64;
        nat_lt(a4919, a4920)
    } else {
        false
    } {
        let a4921 = v1;
        let a4922 = 2147483648u64;
        nat_lt(a4921, a4922)
    } else {
        false
    } {
        let a4923 = 0u64;
        let a4924 = v2;
        nat_lt(a4923, a4924)
    } else {
        false
    } {
        let a4925 = v3;
        let a4926 = 2147483648u64;
        nat_lt(a4925, a4926)
    } else {
        false
    } {
        let a4938 = {
            let a4933 = {
                let a4930 = v3;
                let a4931 = {
                    let a4928 = match v0.clone() {
                        Adt6::C0(h4927, _, _, _) => {
                            h4927
                        }
                    };
                    let a4929 = 1u64;
                    nat_sub(a4928, a4929)
                };
                nat_add(a4930, a4931)?
            };
            let a4934 = f124(match v0.clone() {
                Adt6::C0(_, _, h4932, _) => {
                    h4932
                }
            });
            nat_add(a4933, a4934)?
        };
        let a4939 = {
            let a4936 = 2147483647u64;
            let a4937 = f125(match v0.clone() {
                Adt6::C0(_, _, h4935, _) => {
                    h4935
                }
            });
            nat_add(a4936, a4937)?
        };
        nat_le(a4938, a4939)
    } else {
        false
    } {
        let a4941 = match v0 {
            Adt6::C0(_, _, h4940, _) => {
                h4940
            }
        };
        let a4942 = 2147483648u64;
        Ok(!nat_eq(a4941, a4942))
    } else {
        Ok(false)
    }
}

pub fn f197(v0: Adt2, v1: Adt2, v2: Adt2) -> R<Adt10> {
    if f246(match v0.clone() {
        Adt2::C0(h4943, _, _, _, _, _, _, _, _, _, _) => {
            h4943
        }
    }, match v0.clone() {
        Adt2::C0(_, h4944, _, _, _, _, _, _, _, _, _) => {
            h4944
        }
    }, match v0.clone() {
        Adt2::C0(_, _, h4945, _, _, _, _, _, _, _, _) => {
            h4945
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, h4946, _, _, _, _, _, _, _) => {
            h4946
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, h4947, _, _, _, _, _, _) => {
            h4947
        }
    }, match v1.clone() {
        Adt2::C0(h4948, _, _, _, _, _, _, _, _, _, _) => {
            h4948
        }
    }, match v1.clone() {
        Adt2::C0(_, h4949, _, _, _, _, _, _, _, _, _) => {
            h4949
        }
    }, match v1.clone() {
        Adt2::C0(_, _, h4950, _, _, _, _, _, _, _, _) => {
            h4950
        }
    }, match v1.clone() {
        Adt2::C0(_, _, _, h4951, _, _, _, _, _, _, _) => {
            h4951
        }
    }, match v1.clone() {
        Adt2::C0(_, _, _, _, h4952, _, _, _, _, _, _) => {
            h4952
        }
    }, match v2.clone() {
        Adt2::C0(h4953, _, _, _, _, _, _, _, _, _, _) => {
            h4953
        }
    }, match v2.clone() {
        Adt2::C0(_, h4954, _, _, _, _, _, _, _, _, _) => {
            h4954
        }
    }, match v2.clone() {
        Adt2::C0(_, _, h4955, _, _, _, _, _, _, _, _) => {
            h4955
        }
    }, match v2.clone() {
        Adt2::C0(_, _, _, h4956, _, _, _, _, _, _, _) => {
            h4956
        }
    }, match v2.clone() {
        Adt2::C0(_, _, _, _, h4957, _, _, _, _, _, _) => {
            h4957
        }
    }) {
        let m4959 = f192(match v0.clone() {
            Adt2::C0(h4958, _, _, _, _, _, _, _, _, _, _) => {
                h4958
            }
        });
        match m4959 {
            Adt18::C0 => {
                Ok(Adt10::C0)
            }
            Adt18::C1(v3) => {
                let a4965 = {
                    let a4962 = match v0.clone() {
                        Adt2::C0(_, h4960, _, _, _, _, _, _, _, _, _) => {
                            h4960
                        }
                    };
                    let a4963 = match v3.clone() {
                        Adt19::C0(_, h4961, _) => {
                            h4961
                        }
                    };
                    let a4964 = 0u64;
                    nat_rem(a4962, a4963, a4964)
                };
                let a4966 = 0u64;
                if if nat_eq(a4965, a4966) {
                    if f247(match v0.clone() {
                        Adt2::C0(_, h4967, _, _, _, _, _, _, _, _, _) => {
                            h4967
                        }
                    }, match v1.clone() {
                        Adt2::C0(_, h4968, _, _, _, _, _, _, _, _, _) => {
                            h4968
                        }
                    }, match v1.clone() {
                        Adt2::C0(_, _, h4969, _, _, _, _, _, _, _, _) => {
                            h4969
                        }
                    }, match v1.clone() {
                        Adt2::C0(_, _, _, h4970, _, _, _, _, _, _, _) => {
                            h4970
                        }
                    }) {
                        true
                    } else {
                        let a4972 = 4u64;
                        let a4973 = match v2.clone() {
                            Adt2::C0(_, _, _, _, _, h4971, _, _, _, _, _) => {
                                h4971
                            }
                        };
                        if if nat_le(a4972, a4973) {
                            let a4975 = match v3.clone() {
                                Adt19::C0(_, h4974, _) => {
                                    h4974
                                }
                            };
                            let a4976 = 1u64;
                            if nat_eq(a4975, a4976) {
                                true
                            } else {
                                let a4979 = match v3.clone() {
                                    Adt19::C0(_, _, h4977) => {
                                        h4977
                                    }
                                };
                                let a4980 = match v0.clone() {
                                    Adt2::C0(_, _, _, _, _, h4978, _, _, _, _, _) => {
                                        h4978
                                    }
                                };
                                nat_le(a4979, a4980)
                            }
                        } else {
                            false
                        } {
                            if f19(v0.clone(), v2.clone())? {
                                f19(v1.clone(), v2.clone())?
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
                    let v4: Adt3 = f22(v0.clone(), match v3 {
                        Adt19::C0(_, h4981, _) => {
                            h4981
                        }
                    });
                    Ok(Adt10::C4(if f247(match v0.clone() {
                        Adt2::C0(_, h4982, _, _, _, _, _, _, _, _, _) => {
                            h4982
                        }
                    }, match v1.clone() {
                        Adt2::C0(_, h4983, _, _, _, _, _, _, _, _, _) => {
                            h4983
                        }
                    }, match v1.clone() {
                        Adt2::C0(_, _, h4984, _, _, _, _, _, _, _, _) => {
                            h4984
                        }
                    }, match v1.clone() {
                        Adt2::C0(_, _, _, h4985, _, _, _, _, _, _, _) => {
                            h4985
                        }
                    }) {
                        0u64
                    } else {
                        f156(v4.clone(), f22(v1.clone(), 1u64))?
                    }, 4u64, v4, f22(v2, 1u64), Adt4::C20(match v0 {
                        Adt2::C0(h4986, _, _, _, _, _, _, _, _, _, _) => {
                            h4986
                        }
                    }, f22(v1, 1u64))))
                } else {
                    Ok(Adt10::C1)
                }
            }
        }
    } else {
        Ok(Adt10::C1)
    }
}

pub fn f198(v0: Adt2, v1: Adt2, v2: Adt2) -> R<Adt10> {
    if if if if if if if if if if if if if f199(match v0.clone() {
        Adt2::C0(h4987, _, _, _, _, _, _, _, _, _, _) => {
            h4987
        }
    }) {
        let a4993 = {
            let a4990 = match v0.clone() {
                Adt2::C0(_, h4988, _, _, _, _, _, _, _, _, _) => {
                    h4988
                }
            };
            let a4991 = f229(match v0.clone() {
                Adt2::C0(h4989, _, _, _, _, _, _, _, _, _, _) => {
                    h4989
                }
            });
            let a4992 = 1u64;
            nat_rem(a4990, a4991, a4992)
        };
        let a4994 = 0u64;
        nat_eq(a4993, a4994)
    } else {
        false
    } {
        let a4996 = match v1.clone() {
            Adt2::C0(h4995, _, _, _, _, _, _, _, _, _, _) => {
                h4995
            }
        };
        let a4997 = 26u64;
        nat_eq(a4996, a4997)
    } else {
        false
    } {
        let a4999 = match v2.clone() {
            Adt2::C0(h4998, _, _, _, _, _, _, _, _, _, _) => {
                h4998
            }
        };
        let a5000 = 0u64;
        nat_eq(a4999, a5000)
    } else {
        false
    } {
        let a5003 = match v0.clone() {
            Adt2::C0(_, h5001, _, _, _, _, _, _, _, _, _) => {
                h5001
            }
        };
        let a5004 = match v2.clone() {
            Adt2::C0(_, h5002, _, _, _, _, _, _, _, _, _) => {
                h5002
            }
        };
        nat_eq(a5003, a5004)
    } else {
        false
    } {
        let a5007 = match v1.clone() {
            Adt2::C0(_, h5005, _, _, _, _, _, _, _, _, _) => {
                h5005
            }
        };
        let a5008 = match v0.clone() {
            Adt2::C0(_, _, h5006, _, _, _, _, _, _, _, _) => {
                h5006
            }
        };
        nat_le(a5007, a5008)
    } else {
        false
    } {
        let a5010 = match v0.clone() {
            Adt2::C0(_, _, _, h5009, _, _, _, _, _, _, _) => {
                h5009
            }
        };
        let a5011 = 1u64;
        nat_eq(a5010, a5011)
    } else {
        false
    } {
        let a5013 = match v0.clone() {
            Adt2::C0(_, _, _, _, h5012, _, _, _, _, _, _) => {
                h5012
            }
        };
        let a5014 = 1u64;
        nat_eq(a5013, a5014)
    } else {
        false
    } {
        let a5016 = match v2.clone() {
            Adt2::C0(_, _, _, h5015, _, _, _, _, _, _, _) => {
                h5015
            }
        };
        let a5017 = 1u64;
        nat_eq(a5016, a5017)
    } else {
        false
    } {
        let a5019 = match v2.clone() {
            Adt2::C0(_, _, _, _, h5018, _, _, _, _, _, _) => {
                h5018
            }
        };
        let a5020 = 1u64;
        nat_eq(a5019, a5020)
    } else {
        false
    } {
        let a5022 = match v1.clone() {
            Adt2::C0(_, _, h5021, _, _, _, _, _, _, _, _) => {
                h5021
            }
        };
        let a5023 = 1u64;
        nat_eq(a5022, a5023)
    } else {
        false
    } {
        let a5025 = match v1.clone() {
            Adt2::C0(_, _, _, h5024, _, _, _, _, _, _, _) => {
                h5024
            }
        };
        let a5026 = 1u64;
        nat_eq(a5025, a5026)
    } else {
        false
    } {
        let a5028 = match v1.clone() {
            Adt2::C0(_, _, _, _, h5027, _, _, _, _, _, _) => {
                h5027
            }
        };
        let a5029 = 1u64;
        nat_eq(a5028, a5029)
    } else {
        false
    } {
        if f38(match v2.clone() {
            Adt2::C0(_, h5030, _, _, _, _, _, _, _, _, _) => {
                h5030
            }
        }, match v2.clone() {
            Adt2::C0(_, _, h5031, _, _, _, _, _, _, _, _) => {
                h5031
            }
        }, match v2.clone() {
            Adt2::C0(_, _, _, h5032, _, _, _, _, _, _, _) => {
                h5032
            }
        }, match v2.clone() {
            Adt2::C0(_, _, _, _, h5033, _, _, _, _, _, _) => {
                h5033
            }
        }) {
            Ok(Adt10::C4(0u64, 4u64, f22(v0, 1u64), f22(v2, 1u64), Adt4::C9(0u64, 0u64)))
        } else {
            let a5035 = match v1.clone() {
                Adt2::C0(_, h5034, _, _, _, _, _, _, _, _, _) => {
                    h5034
                }
            };
            let a5036 = 0u64;
            if nat_eq(a5035, a5036) {
                Ok(Adt10::C4(f18(v2.clone(), 1u64)?, 4u64, f22(v0, 1u64), f22(v2, 1u64), Adt4::C9(0u64, 0u64)))
            } else {
                let a5038 = match v0.clone() {
                    Adt2::C0(_, h5037, _, _, _, _, _, _, _, _, _) => {
                        h5037
                    }
                };
                let a5039 = 2147483647u64;
                if if if if if if nat_le(a5038, a5039) {
                    let a5041 = match v1.clone() {
                        Adt2::C0(_, h5040, _, _, _, _, _, _, _, _, _) => {
                            h5040
                        }
                    };
                    let a5042 = 2147483647u64;
                    nat_le(a5041, a5042)
                } else {
                    false
                } {
                    let a5044 = 4u64;
                    let a5045 = match v2.clone() {
                        Adt2::C0(_, _, _, _, _, h5043, _, _, _, _, _) => {
                            h5043
                        }
                    };
                    nat_le(a5044, a5045)
                } else {
                    false
                } {
                    let a5051 = {
                        let a5048 = match v2.clone() {
                            Adt2::C0(_, h5046, _, _, _, _, _, _, _, _, _) => {
                                h5046
                            }
                        };
                        let a5049 = match v2.clone() {
                            Adt2::C0(_, _, _, _, _, h5047, _, _, _, _, _) => {
                                h5047
                            }
                        };
                        nat_mul(a5048, a5049)?
                    };
                    let a5052 = match v2.clone() {
                        Adt2::C0(_, _, _, _, _, _, h5050, _, _, _, _) => {
                            h5050
                        }
                    };
                    nat_le(a5051, a5052)
                } else {
                    false
                } {
                    f19(v0.clone(), v2.clone())?
                } else {
                    false
                } {
                    f19(v1.clone(), v2.clone())?
                } else {
                    false
                } {
                    Ok(Adt10::C4({
                        let a5065 = {
                            let a5056 = {
                                let a5054 = 2u64;
                                let a5055 = match v1.clone() {
                                    Adt2::C0(_, h5053, _, _, _, _, _, _, _, _, _) => {
                                        h5053
                                    }
                                };
                                nat_mul(a5054, a5055)?
                            };
                            let a5057 = f18(v2.clone(), 1u64)?;
                            nat_add(a5056, a5057)?
                        };
                        let a5066 = {
                            let a5063 = match v1.clone() {
                                Adt2::C0(_, h5058, _, _, _, _, _, _, _, _, _) => {
                                    h5058
                                }
                            };
                            let a5064 = {
                                let a5061 = 1u64;
                                let a5062 = f153(match v0.clone() {
                                    Adt2::C0(h5059, _, _, _, _, _, _, _, _, _, _) => {
                                        h5059
                                    }
                                }, match v0.clone() {
                                    Adt2::C0(_, h5060, _, _, _, _, _, _, _, _, _) => {
                                        h5060
                                    }
                                })?;
                                nat_add(a5061, a5062)?
                            };
                            nat_mul(a5063, a5064)?
                        };
                        nat_add(a5065, a5066)?
                    }, 4u64, Adt3::C0(match v0.clone() {
                        Adt2::C0(_, h5067, _, _, _, _, _, _, _, _, _) => {
                            h5067
                        }
                    }, match v1.clone() {
                        Adt2::C0(_, h5068, _, _, _, _, _, _, _, _, _) => {
                            h5068
                        }
                    }, 1u64, 1u64, match v0.clone() {
                        Adt2::C0(_, _, _, _, _, h5069, _, _, _, _, _) => {
                            h5069
                        }
                    }, match v0.clone() {
                        Adt2::C0(_, _, _, _, _, _, h5070, _, _, _, _) => {
                            h5070
                        }
                    }, match v0.clone() {
                        Adt2::C0(_, _, _, _, _, _, _, h5071, _, _, _) => {
                            h5071
                        }
                    }, match v0.clone() {
                        Adt2::C0(_, _, _, _, _, _, _, _, h5072, _, _) => {
                            h5072
                        }
                    }), f22(v2, 1u64), Adt4::C19(match v0 {
                        Adt2::C0(h5073, _, _, _, _, _, _, _, _, _, _) => {
                            h5073
                        }
                    }, f22(v1, 1u64))))
                } else {
                    Ok(Adt10::C1)
                }
            }
        }
    } else {
        Ok(Adt10::C1)
    }
}

pub fn f199(v0: u64) -> bool {
    let a5074 = v0;
    let a5075 = 0u64;
    if if if if if if if if if if if if if if if if if if if if if if if if if if if if if nat_eq(a5074, a5075) {
        true
    } else {
        let a5076 = v0;
        let a5077 = 1u64;
        nat_eq(a5076, a5077)
    } {
        true
    } else {
        let a5078 = v0;
        let a5079 = 2u64;
        nat_eq(a5078, a5079)
    } {
        true
    } else {
        let a5080 = v0;
        let a5081 = 3u64;
        nat_eq(a5080, a5081)
    } {
        true
    } else {
        let a5082 = v0;
        let a5083 = 6u64;
        nat_eq(a5082, a5083)
    } {
        true
    } else {
        let a5084 = v0;
        let a5085 = 7u64;
        nat_eq(a5084, a5085)
    } {
        true
    } else {
        let a5086 = v0;
        let a5087 = 8u64;
        nat_eq(a5086, a5087)
    } {
        true
    } else {
        let a5088 = v0;
        let a5089 = 10u64;
        nat_eq(a5088, a5089)
    } {
        true
    } else {
        let a5090 = v0;
        let a5091 = 11u64;
        nat_eq(a5090, a5091)
    } {
        true
    } else {
        let a5092 = v0;
        let a5093 = 12u64;
        nat_eq(a5092, a5093)
    } {
        true
    } else {
        let a5094 = v0;
        let a5095 = 13u64;
        nat_eq(a5094, a5095)
    } {
        true
    } else {
        let a5096 = v0;
        let a5097 = 14u64;
        nat_eq(a5096, a5097)
    } {
        true
    } else {
        let a5098 = v0;
        let a5099 = 16u64;
        nat_eq(a5098, a5099)
    } {
        true
    } else {
        let a5100 = v0;
        let a5101 = 17u64;
        nat_eq(a5100, a5101)
    } {
        true
    } else {
        let a5102 = v0;
        let a5103 = 18u64;
        nat_eq(a5102, a5103)
    } {
        true
    } else {
        let a5104 = v0;
        let a5105 = 19u64;
        nat_eq(a5104, a5105)
    } {
        true
    } else {
        let a5106 = v0;
        let a5107 = 20u64;
        nat_eq(a5106, a5107)
    } {
        true
    } else {
        let a5108 = v0;
        let a5109 = 21u64;
        nat_eq(a5108, a5109)
    } {
        true
    } else {
        let a5110 = v0;
        let a5111 = 22u64;
        nat_eq(a5110, a5111)
    } {
        true
    } else {
        let a5112 = v0;
        let a5113 = 23u64;
        nat_eq(a5112, a5113)
    } {
        true
    } else {
        let a5114 = v0;
        let a5115 = 26u64;
        nat_eq(a5114, a5115)
    } {
        true
    } else {
        let a5116 = v0;
        let a5117 = 29u64;
        nat_eq(a5116, a5117)
    } {
        true
    } else {
        let a5118 = v0;
        let a5119 = 30u64;
        nat_eq(a5118, a5119)
    } {
        true
    } else {
        let a5120 = v0;
        let a5121 = 34u64;
        nat_eq(a5120, a5121)
    } {
        true
    } else {
        let a5122 = v0;
        let a5123 = 35u64;
        nat_eq(a5122, a5123)
    } {
        true
    } else {
        let a5124 = v0;
        let a5125 = 39u64;
        nat_eq(a5124, a5125)
    } {
        true
    } else {
        let a5126 = v0;
        let a5127 = 40u64;
        nat_eq(a5126, a5127)
    } {
        true
    } else {
        let a5128 = v0;
        let a5129 = 41u64;
        nat_eq(a5128, a5129)
    } {
        true
    } else {
        let a5130 = v0;
        let a5131 = 42u64;
        nat_eq(a5130, a5131)
    } {
        true
    } else {
        let a5132 = v0;
        let a5133 = 43u64;
        nat_eq(a5132, a5133)
    }
}

pub fn f200(v0: u64) -> bool {
    if f154(v0) {
        true
    } else {
        let a5134 = v0;
        let a5135 = 43u64;
        nat_eq(a5134, a5135)
    }
}

pub fn f201(v0: u64, v1: Adt2, v2: bool, v3: Adt2) -> R<Adt10> {
    let a5138 = match v1.clone() {
        Adt2::C0(h5136, _, _, _, _, _, _, _, _, _, _) => {
            h5136
        }
    };
    let a5139 = match v3.clone() {
        Adt2::C0(h5137, _, _, _, _, _, _, _, _, _, _) => {
            h5137
        }
    };
    if if nat_eq(a5138, a5139) {
        let a5140 = v0;
        let a5141 = 33u64;
        if nat_eq(a5140, a5141) {
            v2
        } else {
            true
        }
    } else {
        false
    } {
        let m5143 = f192(match v1.clone() {
            Adt2::C0(h5142, _, _, _, _, _, _, _, _, _, _) => {
                h5142
            }
        });
        match m5143 {
            Adt18::C0 => {
                Ok(Adt10::C0)
            }
            Adt18::C1(v4) => {
                let v5: u64 = match v4.clone() {
                    Adt19::C0(_, h5144, _) => {
                        h5144
                    }
                };
                let v6: u64 = f18(v1.clone(), v5)?;
                let a5149 = {
                    let a5146 = match v1.clone() {
                        Adt2::C0(_, h5145, _, _, _, _, _, _, _, _, _) => {
                            h5145
                        }
                    };
                    let a5147 = v5;
                    let a5148 = 1u64;
                    nat_rem(a5146, a5147, a5148)
                };
                let a5150 = 0u64;
                if if if nat_eq(a5149, a5150) {
                    let a5155 = {
                        let a5152 = match v3.clone() {
                            Adt2::C0(_, h5151, _, _, _, _, _, _, _, _, _) => {
                                h5151
                            }
                        };
                        let a5153 = v5;
                        let a5154 = 1u64;
                        nat_rem(a5152, a5153, a5154)
                    };
                    let a5156 = 0u64;
                    nat_eq(a5155, a5156)
                } else {
                    false
                } {
                    let a5157 = v6;
                    let a5158 = f18(v3.clone(), v5)?;
                    nat_eq(a5157, a5158)
                } else {
                    false
                } {
                    Ok(Adt10::C4(v6, match v4 {
                        Adt19::C0(_, _, h5159) => {
                            h5159
                        }
                    }, f22(v1, v5), f22(v3, v5), Adt4::C0))
                } else {
                    Ok(Adt10::C1)
                }
            }
        }
    } else {
        Ok(Adt10::C1)
    }
}

pub fn f202(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64, v7: u64) -> bool {
    if f38(v4, v5, v6, v7) {
        f38(v0, v1, v2, v3)
    } else {
        let a5163 = {
            let a5160 = v0;
            let a5161 = v4;
            let a5162 = 1u64;
            nat_rem(a5160, a5161, a5162)
        };
        let a5164 = 0u64;
        if if if nat_eq(a5163, a5164) {
            let a5168 = {
                let a5165 = v1;
                let a5166 = v5;
                let a5167 = 1u64;
                nat_rem(a5165, a5166, a5167)
            };
            let a5169 = 0u64;
            nat_eq(a5168, a5169)
        } else {
            false
        } {
            let a5173 = {
                let a5170 = v2;
                let a5171 = v6;
                let a5172 = 1u64;
                nat_rem(a5170, a5171, a5172)
            };
            let a5174 = 0u64;
            nat_eq(a5173, a5174)
        } else {
            false
        } {
            let a5178 = {
                let a5175 = v3;
                let a5176 = v7;
                let a5177 = 1u64;
                nat_rem(a5175, a5176, a5177)
            };
            let a5179 = 0u64;
            nat_eq(a5178, a5179)
        } else {
            false
        }
    }
}

pub fn f203(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64) -> R<bool> {
    let a5180 = v0;
    let a5181 = v1;
    if nat_eq(a5180, a5181) {
        let a5184 = v4;
        let a5185 = {
            let a5182 = v2;
            let a5183 = v3;
            nat_add(a5182, a5183)?
        };
        Ok(nat_eq(a5184, a5185))
    } else {
        let a5186 = v2;
        let a5187 = v3;
        if nat_eq(a5186, a5187) {
            let a5188 = v4;
            let a5189 = v2;
            Ok(nat_eq(a5188, a5189))
        } else {
            Ok(false)
        }
    }
}

pub fn f204(v0: u64, v1: u64, v2: u64, v3: u64) -> R<bool> {
    let v4: u64 = {
        let a5192 = {
            let a5190 = v0;
            let a5191 = f125(v1);
            nat_add(a5190, a5191)?
        };
        let a5193 = f125(v2);
        nat_add(a5192, a5193)?
    };
    let v5: u64 = {
        let a5194 = f124(v1);
        let a5195 = f124(v2);
        nat_add(a5194, a5195)?
    };
    let a5196 = v1;
    let a5197 = 4294967295u64;
    if if nat_le(a5196, a5197) {
        let a5198 = v2;
        let a5199 = 4294967295u64;
        nat_le(a5198, a5199)
    } else {
        false
    } {
        let a5200 = v5;
        let a5201 = v4;
        if nat_le(a5200, a5201) {
            let a5204 = v3;
            let a5205 = {
                let a5202 = v4;
                let a5203 = v5;
                nat_sub(a5202, a5203)
            };
            Ok(nat_eq(a5204, a5205))
        } else {
            Ok(false)
        }
    } else {
        Ok(false)
    }
}

pub fn f205(v0: u64, v1: u64) -> R<bool> {
    let a5206 = v1;
    let a5207 = 4294967295u64;
    if nat_le(a5206, a5207) {
        let a5210 = {
            let a5208 = f125(v1);
            let a5209 = f124(v1);
            nat_add(a5208, a5209)?
        };
        let a5211 = v0;
        Ok(nat_lt(a5210, a5211))
    } else {
        Ok(false)
    }
}

pub fn f206(v0: Adt3, v1: u64, v2: u64, v3: u64, v4: bool) -> R<bool> {
    let v5: u64 = f248(v0.clone(), false)?;
    let v6: u64 = f248(v0, true)?;
    let a5212 = v6;
    let a5213 = v1;
    if if nat_le(a5212, a5213) {
        let a5220 = {
            let a5218 = {
                let a5214 = v1;
                let a5215 = v5;
                nat_add(a5214, a5215)?
            };
            let a5219 = {
                let a5216 = v3;
                let a5217 = 0u64;
                if nat_eq(a5216, a5217) {
                    0u64
                } else {
                    4u64
                }
            };
            nat_add(a5218, a5219)?
        };
        let a5221 = v2;
        nat_le(a5220, a5221)
    } else {
        false
    } {
        if v4 {
            let a5226 = {
                let a5224 = {
                    let a5222 = v1;
                    let a5223 = v5;
                    nat_add(a5222, a5223)?
                };
                let a5225 = v6;
                nat_sub(a5224, a5225)
            };
            let a5227 = v2;
            Ok(nat_lt(a5226, a5227))
        } else {
            let a5232 = {
                let a5230 = {
                    let a5228 = v1;
                    let a5229 = v5;
                    nat_add(a5228, a5229)?
                };
                let a5231 = v6;
                nat_sub(a5230, a5231)
            };
            let a5233 = v2;
            Ok(nat_le(a5232, a5233))
        }
    } else {
        Ok(false)
    }
}

pub fn f207(v0: u64, v1: u64) -> bool {
    let a5234 = v0;
    let a5235 = 0u64;
    if if nat_eq(a5234, a5235) {
        true
    } else {
        let a5236 = v0;
        let a5237 = 1u64;
        nat_eq(a5236, a5237)
    } {
        let a5238 = v1;
        let a5239 = 4294967296u64;
        nat_lt(a5238, a5239)
    } else {
        false
    }
}

pub fn f208(v0: u64, v1: u64) -> R<u64> {
    let a5244 = {
        let a5242 = v0;
        let a5243 = {
            let a5240 = v1;
            let a5241 = 1u64;
            nat_sub(a5240, a5241)
        };
        nat_add(a5242, a5243)?
    };
    let a5245 = v1;
    let a5246 = 0u64;
    Ok(nat_quot(a5244, a5245, a5246))
}

pub fn f209(v0: Adt2, v1: Adt2, v2: u64, v3: u64) -> R<bool> {
    if f38(match v0.clone() {
        Adt2::C0(_, h5247, _, _, _, _, _, _, _, _, _) => {
            h5247
        }
    }, match v0.clone() {
        Adt2::C0(_, _, h5248, _, _, _, _, _, _, _, _) => {
            h5248
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, h5249, _, _, _, _, _, _, _) => {
            h5249
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, h5250, _, _, _, _, _, _) => {
            h5250
        }
    }) {
        Ok(true)
    } else {
        let v4: u64 = {
            let a5253 = match v1.clone() {
                Adt2::C0(_, h5251, _, _, _, _, _, _, _, _, _) => {
                    h5251
                }
            };
            let a5254 = match v1.clone() {
                Adt2::C0(_, _, h5252, _, _, _, _, _, _, _, _) => {
                    h5252
                }
            };
            nat_mul(a5253, a5254)?
        };
        let v5: u64 = {
            let a5256 = v4;
            let a5257 = match v1.clone() {
                Adt2::C0(_, _, _, h5255, _, _, _, _, _, _, _) => {
                    h5255
                }
            };
            nat_mul(a5256, a5257)?
        };
        let a5270 = {
            let a5268 = {
                let a5259 = match v0.clone() {
                    Adt2::C0(_, h5258, _, _, _, _, _, _, _, _, _) => {
                        h5258
                    }
                };
                let a5260 = 1u64;
                nat_sub(a5259, a5260)
            };
            let a5269 = {
                let a5266 = f249(match v0.clone() {
                    Adt2::C0(_, _, h5261, _, _, _, _, _, _, _, _) => {
                        h5261
                    }
                }, v2, v5, match v1.clone() {
                    Adt2::C0(_, h5262, _, _, _, _, _, _, _, _, _) => {
                        h5262
                    }
                })?;
                let a5267 = f249(match v0 {
                    Adt2::C0(_, _, _, h5263, _, _, _, _, _, _, _) => {
                        h5263
                    }
                }, v2, {
                    let a5264 = v3;
                    let a5265 = v5;
                    nat_mul(a5264, a5265)?
                }, v4)?;
                nat_add(a5266, a5267)?
            };
            nat_add(a5268, a5269)?
        };
        let a5271 = f18(v1, 1u64)?;
        Ok(nat_lt(a5270, a5271))
    }
}

pub fn f210(v0: u64, v1: u64, v2: u64) -> R<bool> {
    let a5272 = v0;
    let a5273 = 2147483648u64;
    if if if if nat_lt(a5272, a5273) {
        let a5274 = v1;
        let a5275 = 2147483648u64;
        nat_lt(a5274, a5275)
    } else {
        false
    } {
        let a5278 = {
            let a5276 = v0;
            let a5277 = v1;
            nat_mul(a5276, a5277)?
        };
        let a5279 = 2147483648u64;
        nat_lt(a5278, a5279)
    } else {
        false
    } {
        let a5284 = {
            let a5282 = {
                let a5280 = v0;
                let a5281 = 1u64;
                nat_sub(a5280, a5281)
            };
            let a5283 = v2;
            nat_mul(a5282, a5283)?
        };
        let a5285 = 2147483648u64;
        nat_lt(a5284, a5285)
    } else {
        false
    } {
        let a5290 = {
            let a5288 = {
                let a5286 = v1;
                let a5287 = 1u64;
                nat_sub(a5286, a5287)
            };
            let a5289 = v2;
            nat_mul(a5288, a5289)?
        };
        let a5291 = 2147483648u64;
        Ok(nat_lt(a5290, a5291))
    } else {
        Ok(false)
    }
}

pub fn f211(v0: Adt3, v1: u64, v2: u64) -> R<u64> {
    let a5292 = v1;
    let a5293 = 13u64;
    if if nat_eq(a5292, a5293) {
        f38(match v0.clone() {
            Adt3::C0(h5294, _, _, _, _, _, _, _) => {
                h5294
            }
        }, match v0.clone() {
            Adt3::C0(_, h5295, _, _, _, _, _, _) => {
                h5295
            }
        }, match v0.clone() {
            Adt3::C0(_, _, h5296, _, _, _, _, _) => {
                h5296
            }
        }, match v0.clone() {
            Adt3::C0(_, _, _, h5297, _, _, _, _) => {
                h5297
            }
        })
    } else {
        false
    } {
        Ok(2u64)
    } else {
        let v3: u64 = {
            let a5303 = match v0.clone() {
                Adt3::C0(_, h5298, _, _, _, _, _, _) => {
                    h5298
                }
            };
            let a5304 = {
                let a5301 = match v0.clone() {
                    Adt3::C0(_, _, h5299, _, _, _, _, _) => {
                        h5299
                    }
                };
                let a5302 = match v0.clone() {
                    Adt3::C0(_, _, _, h5300, _, _, _, _) => {
                        h5300
                    }
                };
                nat_mul(a5301, a5302)?
            };
            nat_mul(a5303, a5304)?
        };
        let a5305 = v1;
        let a5306 = 13u64;
        if nat_eq(a5305, a5306) {
            let a5315 = 2u64;
            let a5316 = if f250(v0.clone(), v2) {
                let a5308 = v3;
                let a5309 = f223(match v0 {
                    Adt3::C0(h5307, _, _, _, _, _, _, _) => {
                        h5307
                    }
                }, false)?;
                nat_mul(a5308, a5309)?
            } else {
                let a5313 = v3;
                let a5314 = {
                    let a5311 = match v0 {
                        Adt3::C0(h5310, _, _, _, _, _, _, _) => {
                            h5310
                        }
                    };
                    let a5312 = 2u64;
                    nat_mul(a5311, a5312)?
                };
                nat_mul(a5313, a5314)?
            };
            nat_add(a5315, a5316)
        } else {
            let a5320 = v3;
            let a5321 = f223(match v0 {
                Adt3::C0(h5317, _, _, _, _, _, _, _) => {
                    h5317
                }
            }, {
                let a5318 = v1;
                let a5319 = 16u64;
                nat_eq(a5318, a5319)
            })?;
            nat_mul(a5320, a5321)
        }
    }
}

pub fn f212(v0: u64, v1: u64, v2: Adt2, v3: Option<Adt2>, v4: Adt2) -> R<Adt10> {
    let a5322 = v0;
    let a5323 = 30u64;
    if if nat_eq(a5322, a5323) {
        let a5326 = match v2.clone() {
            Adt2::C0(_, _, _, _, _, _, h5324, _, _, _, _) => {
                h5324
            }
        };
        let a5327 = match v2.clone() {
            Adt2::C0(_, _, _, _, _, h5325, _, _, _, _, _) => {
                h5325
            }
        };
        nat_lt(a5326, a5327)
    } else {
        false
    } {
        Ok(Adt10::C1)
    } else {
        let m5328 = v3;
        match m5328 {
            None => {
                f251(v0, v1, match v2.clone() {
                    Adt2::C0(h5329, _, _, _, _, _, _, _, _, _, _) => {
                        h5329
                    }
                }, 0u64, match v4.clone() {
                    Adt2::C0(h5330, _, _, _, _, _, _, _, _, _, _) => {
                        h5330
                    }
                }, false, match v2.clone() {
                    Adt2::C0(_, h5331, _, _, _, _, _, _, _, _, _) => {
                        h5331
                    }
                }, match v2.clone() {
                    Adt2::C0(_, _, h5332, _, _, _, _, _, _, _, _) => {
                        h5332
                    }
                }, match v2.clone() {
                    Adt2::C0(_, _, _, h5333, _, _, _, _, _, _, _) => {
                        h5333
                    }
                }, match v2 {
                    Adt2::C0(_, _, _, _, h5334, _, _, _, _, _, _) => {
                        h5334
                    }
                }, 1u64, 1u64, 1u64, 1u64, match v4.clone() {
                    Adt2::C0(_, h5335, _, _, _, _, _, _, _, _, _) => {
                        h5335
                    }
                }, match v4.clone() {
                    Adt2::C0(_, _, h5336, _, _, _, _, _, _, _, _) => {
                        h5336
                    }
                }, match v4.clone() {
                    Adt2::C0(_, _, _, h5337, _, _, _, _, _, _, _) => {
                        h5337
                    }
                }, match v4 {
                    Adt2::C0(_, _, _, _, h5338, _, _, _, _, _, _) => {
                        h5338
                    }
                })
            }
            Some(v5) => {
                f251(v0, v1, match v2.clone() {
                    Adt2::C0(h5339, _, _, _, _, _, _, _, _, _, _) => {
                        h5339
                    }
                }, match v5.clone() {
                    Adt2::C0(h5340, _, _, _, _, _, _, _, _, _, _) => {
                        h5340
                    }
                }, match v4.clone() {
                    Adt2::C0(h5341, _, _, _, _, _, _, _, _, _, _) => {
                        h5341
                    }
                }, true, match v2.clone() {
                    Adt2::C0(_, h5342, _, _, _, _, _, _, _, _, _) => {
                        h5342
                    }
                }, match v2.clone() {
                    Adt2::C0(_, _, h5343, _, _, _, _, _, _, _, _) => {
                        h5343
                    }
                }, match v2.clone() {
                    Adt2::C0(_, _, _, h5344, _, _, _, _, _, _, _) => {
                        h5344
                    }
                }, match v2 {
                    Adt2::C0(_, _, _, _, h5345, _, _, _, _, _, _) => {
                        h5345
                    }
                }, match v5.clone() {
                    Adt2::C0(_, h5346, _, _, _, _, _, _, _, _, _) => {
                        h5346
                    }
                }, match v5.clone() {
                    Adt2::C0(_, _, h5347, _, _, _, _, _, _, _, _) => {
                        h5347
                    }
                }, match v5.clone() {
                    Adt2::C0(_, _, _, h5348, _, _, _, _, _, _, _) => {
                        h5348
                    }
                }, match v5 {
                    Adt2::C0(_, _, _, _, h5349, _, _, _, _, _, _) => {
                        h5349
                    }
                }, match v4.clone() {
                    Adt2::C0(_, h5350, _, _, _, _, _, _, _, _, _) => {
                        h5350
                    }
                }, match v4.clone() {
                    Adt2::C0(_, _, h5351, _, _, _, _, _, _, _, _) => {
                        h5351
                    }
                }, match v4.clone() {
                    Adt2::C0(_, _, _, h5352, _, _, _, _, _, _, _) => {
                        h5352
                    }
                }, match v4 {
                    Adt2::C0(_, _, _, _, h5353, _, _, _, _, _, _) => {
                        h5353
                    }
                })
            }
        }
    }
}

pub fn f213(v0: u64, v1: u64) -> u64 {
    let a5354 = v0;
    let a5355 = 23u64;
    if nat_eq(a5354, a5355) {
        let a5356 = v1;
        let a5357 = 0u64;
        if nat_eq(a5356, a5357) {
            2u64
        } else {
            3u64
        }
    } else {
        1u64
    }
}

pub fn f214(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let a5362 = {
        let a5360 = {
            let a5358 = v0;
            let a5359 = v1;
            nat_sub(a5358, a5359)
        };
        let a5361 = 1u64;
        nat_sub(a5360, a5361)
    };
    let a5363 = v2;
    nat_add(a5362, a5363)
}

pub fn f215(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64, v7: u64, v8: u64, v9: u64, v10: u64, v11: u64) -> R<u64> {
    f123({
        let a5364 = v0;
        let a5365 = v4;
        let a5366 = 0u64;
        nat_rem(a5364, a5365, a5366)
    }, {
        let a5367 = v1;
        let a5368 = v5;
        let a5369 = 0u64;
        nat_rem(a5367, a5368, a5369)
    }, {
        let a5370 = v2;
        let a5371 = v6;
        let a5372 = 0u64;
        nat_rem(a5370, a5371, a5372)
    }, {
        let a5373 = v3;
        let a5374 = v7;
        let a5375 = 0u64;
        nat_rem(a5373, a5374, a5375)
    }, v8, v9, v10, v11)
}

pub fn f216(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let a5376 = v1;
    let a5377 = 2147483648u64;
    if nat_lt(a5376, a5377) {
        let a5388 = {
            let a5386 = {
                let a5381 = {
                    let a5378 = v0;
                    let a5379 = v2;
                    let a5380 = 0u64;
                    nat_rem(a5378, a5379, a5380)
                };
                let a5382 = v2;
                nat_add(a5381, a5382)?
            };
            let a5387 = {
                let a5383 = v1;
                let a5384 = v2;
                let a5385 = 0u64;
                nat_rem(a5383, a5384, a5385)
            };
            nat_sub(a5386, a5387)
        };
        let a5389 = v2;
        let a5390 = 0u64;
        Ok(nat_rem(a5388, a5389, a5390))
    } else {
        let a5393 = {
            let a5391 = v0;
            let a5392 = f124(v1);
            nat_add(a5391, a5392)?
        };
        let a5394 = v2;
        let a5395 = 0u64;
        Ok(nat_rem(a5393, a5394, a5395))
    }
}

pub fn f217(v0: u64, v1: u64, v2: u64) -> R<bool> {
    let a5398 = f125(v1);
    let a5399 = {
        let a5396 = v0;
        let a5397 = f124(v1);
        nat_add(a5396, a5397)?
    };
    if nat_le(a5398, a5399) {
        let a5400 = f218(v0, v1)?;
        let a5401 = v2;
        Ok(nat_lt(a5400, a5401))
    } else {
        Ok(false)
    }
}

pub fn f218(v0: u64, v1: u64) -> R<u64> {
    let a5404 = {
        let a5402 = v0;
        let a5403 = f124(v1);
        nat_add(a5402, a5403)?
    };
    let a5405 = f125(v1);
    Ok(nat_sub(a5404, a5405))
}

pub fn f219(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let a5406 = v0;
    let a5407 = v1;
    if nat_lt(a5406, a5407) {
        let a5408 = v1;
        let a5409 = v0;
        Ok(nat_sub(a5408, a5409))
    } else {
        let a5412 = v0;
        let a5413 = {
            let a5410 = v1;
            let a5411 = v2;
            nat_add(a5410, a5411)?
        };
        if nat_lt(a5412, a5413) {
            let a5414 = v0;
            let a5415 = v1;
            Ok(nat_sub(a5414, a5415))
        } else {
            let a5422 = {
                let a5418 = 2u64;
                let a5419 = {
                    let a5416 = v2;
                    let a5417 = 1u64;
                    nat_sub(a5416, a5417)
                };
                nat_mul(a5418, a5419)?
            };
            let a5423 = {
                let a5420 = v0;
                let a5421 = v1;
                nat_sub(a5420, a5421)
            };
            Ok(nat_sub(a5422, a5423))
        }
    }
}

pub fn f220(v0: u64, v1: Adt3, v2: u64) -> R<u64> {
    let a5426 = {
        let a5424 = v2;
        let a5425 = f252(v1.clone(), f48(v0, v1.clone(), 0u64), f48(v0, v1.clone(), 1u64), f48(v0, v1.clone(), 2u64), f48(v0, v1.clone(), 3u64), false)?;
        nat_add(a5424, a5425)?
    };
    let a5427 = f252(v1.clone(), f48(v0, v1.clone(), 0u64), f48(v0, v1.clone(), 1u64), f48(v0, v1.clone(), 2u64), f48(v0, v1, 3u64), true)?;
    Ok(nat_sub(a5426, a5427))
}

pub fn f221(v0: u64, v1: u64, v2: u64) -> bool {
    let a5428 = v0;
    let a5429 = 0u64;
    if nat_eq(a5428, a5429) {
        let a5430 = v2;
        let a5431 = v1;
        nat_le(a5430, a5431)
    } else {
        let a5432 = v0;
        let a5433 = 1u64;
        if nat_eq(a5432, a5433) {
            let a5434 = v2;
            let a5435 = v1;
            nat_lt(a5434, a5435)
        } else {
            let a5436 = v0;
            let a5437 = 2u64;
            if nat_eq(a5436, a5437) {
                let a5438 = v1;
                let a5439 = v2;
                nat_le(a5438, a5439)
            } else {
                let a5440 = v0;
                let a5441 = 3u64;
                if nat_eq(a5440, a5441) {
                    let a5442 = v1;
                    let a5443 = v2;
                    nat_lt(a5442, a5443)
                } else {
                    let a5444 = v1;
                    let a5445 = v2;
                    nat_eq(a5444, a5445)
                }
            }
        }
    }
}

pub fn f222(v0: u64, v1: Adt3, v2: u64) -> R<Adt9> {
    let a5446 = v0;
    let a5447 = 0u64;
    if nat_eq(a5446, a5447) {
        Ok(f235(0u64))
    } else {
        let a5450 = {
            let a5448 = v0;
            let a5449 = 1u64;
            nat_add(a5448, a5449)?
        };
        let a5451 = f211(v1.clone(), 13u64, v2)?;
        if nat_eq(a5450, a5451) {
            Ok(Adt9::C4({
                let a5452 = v2;
                let a5453 = 0u64;
                if nat_eq(a5452, a5453) {
                    2u64
                } else {
                    7u64
                }
            }, 1u64, 0u64, 2u64, 0u64, 3u64, 0u64))
        } else {
            let v3: u64 = {
                let a5454 = v0;
                let a5455 = 1u64;
                nat_sub(a5454, a5455)
            };
            if f250(v1.clone(), v2) {
                let v4: u64 = f223(match v1.clone() {
                    Adt3::C0(h5456, _, _, _, _, _, _, _) => {
                        h5456
                    }
                }, false)?;
                let v5: u64 = {
                    let a5457 = v3;
                    let a5458 = v4;
                    let a5459 = 0u64;
                    nat_quot(a5457, a5458, a5459)
                };
                let v6: u64 = {
                    let a5460 = v3;
                    let a5461 = v4;
                    let a5462 = 0u64;
                    nat_rem(a5460, a5461, a5462)
                };
                let a5465 = {
                    let a5463 = v6;
                    let a5464 = 1u64;
                    nat_add(a5463, a5464)?
                };
                let a5466 = v4;
                if nat_eq(a5465, a5466) {
                    Ok(Adt9::C4({
                        let a5467 = v2;
                        let a5468 = 0u64;
                        if nat_eq(a5467, a5468) {
                            0u64
                        } else {
                            4u64
                        }
                    }, 1u64, 0u64, 1u64, 1u64, 1u64, 0u64))
                } else {
                    f224(v6, v5, v1, v2)
                }
            } else {
                f253({
                    let a5469 = v3;
                    let a5470 = 2u64;
                    let a5471 = 0u64;
                    nat_quot(a5469, a5470, a5471)
                }, {
                    let a5472 = v3;
                    let a5473 = 2u64;
                    let a5474 = 0u64;
                    nat_rem(a5472, a5473, a5474)
                }, v1, v2, 0u64)
            }
        }
    }
}

pub fn f223(v0: u64, v1: bool) -> R<u64> {
    let a5477 = {
        let a5475 = v0;
        let a5476 = 2u64;
        nat_mul(a5475, a5476)?
    };
    let a5478 = if v1 {
        4u64
    } else {
        2u64
    };
    nat_add(a5477, a5478)
}

pub fn f224(v0: u64, v1: u64, v2: Adt3, v3: u64) -> R<Adt9> {
    let a5479 = v0;
    let a5480 = 0u64;
    if nat_eq(a5479, a5480) {
        Ok(f235(1u64))
    } else {
        f253({
            let a5489 = {
                let a5482 = v1;
                let a5483 = match v2.clone() {
                    Adt3::C0(h5481, _, _, _, _, _, _, _) => {
                        h5481
                    }
                };
                nat_mul(a5482, a5483)?
            };
            let a5490 = {
                let a5486 = {
                    let a5484 = v0;
                    let a5485 = 1u64;
                    nat_sub(a5484, a5485)
                };
                let a5487 = 2u64;
                let a5488 = 0u64;
                nat_quot(a5486, a5487, a5488)
            };
            nat_add(a5489, a5490)?
        }, {
            let a5493 = {
                let a5491 = v0;
                let a5492 = 1u64;
                nat_sub(a5491, a5492)
            };
            let a5494 = 2u64;
            let a5495 = 0u64;
            nat_rem(a5493, a5494, a5495)
        }, v2, v3, 1u64)
    }
}

pub fn f225(v0: u64, v1: u64) -> R<u64> {
    let a5496 = v0;
    let a5497 = f79(v1)?;
    let a5498 = 0u64;
    Ok(nat_rem(a5496, a5497, a5498))
}

pub fn f226(v0: u64, v1: u64) -> R<u64> {
    let a5499 = 4u64;
    let a5500 = f254(v0, v1)?;
    nat_mul(a5499, a5500)
}

pub fn f227(v0: u64, v1: u64) -> R<u64> {
    let a5501 = 4u64;
    let a5502 = f255(v0, v1)?;
    nat_mul(a5501, a5502)
}

pub fn f228(v0: u64, v1: u64) -> R<bool> {
    let a5508 = {
        let a5505 = {
            let a5503 = f225(v0, v1)?;
            let a5504 = 2u64;
            nat_sub(a5503, a5504)
        };
        let a5506 = 2u64;
        let a5507 = 0u64;
        nat_rem(a5505, a5506, a5507)
    };
    let a5509 = 0u64;
    Ok(nat_eq(a5508, a5509))
}

pub fn f229(v0: u64) -> u64 {
    let m5510 = f192(v0);
    match m5510 {
        Adt18::C0 => {
            0u64
        }
        Adt18::C1(v1) => {
            match v1 {
                Adt19::C0(_, h5511, _) => {
                    h5511
                }
            }
        }
    }
}

pub fn f230(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64, v7: u64, v8: u64, v9: u64, v10: u64, v11: u64, v12: u64) -> R<Option<u64>> {
    let m5512 = f192(v0);
    match m5512 {
        Adt18::C0 => {
            Ok(None::<u64>)
        }
        Adt18::C1(v13) => {
            if if if if if f199(v0) {
                let a5513 = v1;
                let a5514 = v7;
                nat_lt(a5513, a5514)
            } else {
                false
            } {
                f159(v8, v2)
            } else {
                false
            } {
                let a5515 = v3;
                let a5516 = f256(v4, v5, v6)?;
                nat_lt(a5515, a5516)
            } else {
                false
            } {
                let a5521 = {
                    let a5518 = v7;
                    let a5519 = match v13.clone() {
                        Adt19::C0(_, h5517, _) => {
                            h5517
                        }
                    };
                    let a5520 = 0u64;
                    nat_rem(a5518, a5519, a5520)
                };
                let a5522 = 0u64;
                nat_eq(a5521, a5522)
            } else {
                false
            } {
                Ok(Some({
                    let a5551 = {
                        let a5541 = {
                            let a5531 = {
                                let a5527 = {
                                    let a5524 = v1;
                                    let a5525 = match v13 {
                                        Adt19::C0(_, h5523, _) => {
                                            h5523
                                        }
                                    };
                                    let a5526 = 0u64;
                                    nat_quot(a5524, a5525, a5526)
                                };
                                let a5528 = v9;
                                nat_mul(a5527, a5528)?
                            };
                            let a5532 = {
                                let a5529 = v2;
                                let a5530 = v10;
                                nat_mul(a5529, a5530)?
                            };
                            nat_add(a5531, a5532)?
                        };
                        let a5542 = {
                            let a5539 = {
                                let a5536 = {
                                    let a5533 = v3;
                                    let a5534 = v4;
                                    let a5535 = 0u64;
                                    nat_quot(a5533, a5534, a5535)
                                };
                                let a5537 = v5;
                                let a5538 = 0u64;
                                nat_rem(a5536, a5537, a5538)
                            };
                            let a5540 = v11;
                            nat_mul(a5539, a5540)?
                        };
                        nat_add(a5541, a5542)?
                    };
                    let a5552 = {
                        let a5549 = {
                            let a5546 = {
                                let a5543 = v3;
                                let a5544 = v4;
                                let a5545 = 0u64;
                                nat_quot(a5543, a5544, a5545)
                            };
                            let a5547 = v5;
                            let a5548 = 0u64;
                            nat_quot(a5546, a5547, a5548)
                        };
                        let a5550 = v12;
                        nat_mul(a5549, a5550)?
                    };
                    nat_add(a5551, a5552)?
                }))
            } else {
                Ok(None::<u64>)
            }
        }
    }
}

pub fn f231(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64, v7: u64, v8: u64, v9: u64, v10: u64, v11: u64, v12: u64, v13: u64, v14: u64) -> R<Adt17> {
    f257(v0, Adt20::C0(v1, v2, v3, v4, v5, v6, v7, v8), Adt20::C0(v1, v2, v3, v4, v9, v10, v11, v12), v13, v14)
}

pub fn f232(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64, v7: u64, v8: u64, v9: u64, v10: u64, v11: u64, v12: u64, v13: u64, v14: u64, v15: u64, v16: u64, v17: u64, v18: u64, v19: u64, v20: u64) -> R<Adt17> {
    f258(v0, Adt20::C0(v1, v2, v3, v4, v5, v6, v7, v8), Adt20::C0(v1, v2, v3, v4, v9, v10, v11, v12), Adt20::C0(v1, v2, v3, v4, v13, v14, v15, v16), v17, v18, v19, v20)
}

pub fn f233(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64, v7: u64, v8: u64, v9: u64, v10: u64, v11: u64, v12: u64, v13: u64, v14: u64, v15: u64, v16: u64, v17: u64) -> R<Adt17> {
    let a5553 = v0;
    let a5554 = 64u64;
    if nat_eq(a5553, a5554) {
        f259(v1, Adt20::C0(v2, v3, v4, v5, v6, v7, v8, v9), Adt20::C0(v2, v3, v4, v5, v10, v11, v12, v13), v14, v15, v16, v17)
    } else {
        let a5555 = v0;
        let a5556 = 65u64;
        if nat_eq(a5555, a5556) {
            f260(v1, Adt20::C0(v2, v3, v4, v5, v6, v7, v8, v9), Adt20::C0(v2, v3, v4, v5, v10, v11, v12, v13), v14, v15, v16, v17)
        } else {
            Ok(Adt17::C0)
        }
    }
}

pub fn f234(v0: u64) -> u64 {
    let a5557 = v0;
    let a5558 = 30u64;
    if nat_eq(a5557, a5558) {
        4u64
    } else {
        3u64
    }
}

pub fn f235(v0: u64) -> Adt9 {
    Adt9::C4(0u64, 2u64, 0u64, 2u64, 0u64, 1u64, v0)
}

pub fn f236(v0: u64) -> bool {
    let a5559 = 0u64;
    let a5560 = v0;
    if nat_lt(a5559, a5560) {
        let a5561 = v0;
        let a5562 = 2139095040u64;
        nat_le(a5561, a5562)
    } else {
        false
    }
}

pub fn f237(v0: u64) -> u64 {
    let a5563 = v0;
    let a5564 = 40u64;
    if nat_lt(a5563, a5564) {
        let a5565 = v0;
        let a5566 = 20u64;
        if nat_lt(a5565, a5566) {
            let a5567 = v0;
            let a5568 = 10u64;
            if nat_lt(a5567, a5568) {
                let a5569 = v0;
                let a5570 = 5u64;
                if nat_lt(a5569, a5570) {
                    let a5571 = v0;
                    let a5572 = 2u64;
                    if nat_lt(a5571, a5572) {
                        let a5573 = v0;
                        let a5574 = 1u64;
                        if nat_lt(a5573, a5574) {
                            1u64
                        } else {
                            2u64
                        }
                    } else {
                        let a5575 = v0;
                        let a5576 = 3u64;
                        if nat_lt(a5575, a5576) {
                            4u64
                        } else {
                            let a5577 = v0;
                            let a5578 = 4u64;
                            if nat_lt(a5577, a5578) {
                                8u64
                            } else {
                                16u64
                            }
                        }
                    }
                } else {
                    let a5579 = v0;
                    let a5580 = 7u64;
                    if nat_lt(a5579, a5580) {
                        let a5581 = v0;
                        let a5582 = 6u64;
                        if nat_lt(a5581, a5582) {
                            32u64
                        } else {
                            64u64
                        }
                    } else {
                        let a5583 = v0;
                        let a5584 = 8u64;
                        if nat_lt(a5583, a5584) {
                            128u64
                        } else {
                            let a5585 = v0;
                            let a5586 = 9u64;
                            if nat_lt(a5585, a5586) {
                                256u64
                            } else {
                                512u64
                            }
                        }
                    }
                }
            } else {
                let a5587 = v0;
                let a5588 = 15u64;
                if nat_lt(a5587, a5588) {
                    let a5589 = v0;
                    let a5590 = 12u64;
                    if nat_lt(a5589, a5590) {
                        let a5591 = v0;
                        let a5592 = 11u64;
                        if nat_lt(a5591, a5592) {
                            1024u64
                        } else {
                            2048u64
                        }
                    } else {
                        let a5593 = v0;
                        let a5594 = 13u64;
                        if nat_lt(a5593, a5594) {
                            4096u64
                        } else {
                            let a5595 = v0;
                            let a5596 = 14u64;
                            if nat_lt(a5595, a5596) {
                                8192u64
                            } else {
                                16384u64
                            }
                        }
                    }
                } else {
                    let a5597 = v0;
                    let a5598 = 17u64;
                    if nat_lt(a5597, a5598) {
                        let a5599 = v0;
                        let a5600 = 16u64;
                        if nat_lt(a5599, a5600) {
                            32768u64
                        } else {
                            65536u64
                        }
                    } else {
                        let a5601 = v0;
                        let a5602 = 18u64;
                        if nat_lt(a5601, a5602) {
                            131072u64
                        } else {
                            let a5603 = v0;
                            let a5604 = 19u64;
                            if nat_lt(a5603, a5604) {
                                262144u64
                            } else {
                                524288u64
                            }
                        }
                    }
                }
            }
        } else {
            let a5605 = v0;
            let a5606 = 30u64;
            if nat_lt(a5605, a5606) {
                let a5607 = v0;
                let a5608 = 25u64;
                if nat_lt(a5607, a5608) {
                    let a5609 = v0;
                    let a5610 = 22u64;
                    if nat_lt(a5609, a5610) {
                        let a5611 = v0;
                        let a5612 = 21u64;
                        if nat_lt(a5611, a5612) {
                            1048576u64
                        } else {
                            2097152u64
                        }
                    } else {
                        let a5613 = v0;
                        let a5614 = 23u64;
                        if nat_lt(a5613, a5614) {
                            4194304u64
                        } else {
                            let a5615 = v0;
                            let a5616 = 24u64;
                            if nat_lt(a5615, a5616) {
                                8388608u64
                            } else {
                                16777216u64
                            }
                        }
                    }
                } else {
                    let a5617 = v0;
                    let a5618 = 27u64;
                    if nat_lt(a5617, a5618) {
                        let a5619 = v0;
                        let a5620 = 26u64;
                        if nat_lt(a5619, a5620) {
                            33554432u64
                        } else {
                            67108864u64
                        }
                    } else {
                        let a5621 = v0;
                        let a5622 = 28u64;
                        if nat_lt(a5621, a5622) {
                            134217728u64
                        } else {
                            let a5623 = v0;
                            let a5624 = 29u64;
                            if nat_lt(a5623, a5624) {
                                268435456u64
                            } else {
                                536870912u64
                            }
                        }
                    }
                }
            } else {
                let a5625 = v0;
                let a5626 = 35u64;
                if nat_lt(a5625, a5626) {
                    let a5627 = v0;
                    let a5628 = 32u64;
                    if nat_lt(a5627, a5628) {
                        let a5629 = v0;
                        let a5630 = 31u64;
                        if nat_lt(a5629, a5630) {
                            1073741824u64
                        } else {
                            2147483648u64
                        }
                    } else {
                        let a5631 = v0;
                        let a5632 = 33u64;
                        if nat_lt(a5631, a5632) {
                            4294967296u64
                        } else {
                            let a5633 = v0;
                            let a5634 = 34u64;
                            if nat_lt(a5633, a5634) {
                                8589934592u64
                            } else {
                                17179869184u64
                            }
                        }
                    }
                } else {
                    let a5635 = v0;
                    let a5636 = 37u64;
                    if nat_lt(a5635, a5636) {
                        let a5637 = v0;
                        let a5638 = 36u64;
                        if nat_lt(a5637, a5638) {
                            34359738368u64
                        } else {
                            68719476736u64
                        }
                    } else {
                        let a5639 = v0;
                        let a5640 = 38u64;
                        if nat_lt(a5639, a5640) {
                            137438953472u64
                        } else {
                            let a5641 = v0;
                            let a5642 = 39u64;
                            if nat_lt(a5641, a5642) {
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
    let a5643 = v0;
    let a5644 = 1u64;
    if nat_le(a5643, a5644) {
        true
    } else {
        let a5648 = {
            let a5645 = v1;
            let a5646 = 4u64;
            let a5647 = 0u64;
            nat_rem(a5645, a5646, a5647)
        };
        let a5649 = 0u64;
        nat_eq(a5648, a5649)
    }
}

pub fn f239(v0: u64, v1: u64) -> bool {
    let a5650 = v0;
    let a5651 = 1u64;
    if nat_le(a5650, a5651) {
        true
    } else {
        let a5652 = 4u64;
        let a5653 = v1;
        nat_le(a5652, a5653)
    }
}

pub fn f240(v0: u64, v1: u64) -> R<u64> {
    let a5654 = v0;
    let a5655 = 0u64;
    if nat_eq(a5654, a5655) {
        Ok(0u64)
    } else {
        let a5660 = 4u64;
        let a5661 = {
            let a5658 = {
                let a5656 = v0;
                let a5657 = 1u64;
                nat_sub(a5656, a5657)
            };
            let a5659 = v1;
            nat_mul(a5658, a5659)?
        };
        nat_add(a5660, a5661)
    }
}

pub fn f241(v0: bool, v1: u64) -> R<u64> {
    let a5666 = {
        let a5664 = {
            let a5662 = f261(v0, v1)?;
            let a5663 = f262(v0, v1)?;
            nat_add(a5662, a5663)?
        };
        let a5665 = f263(v0);
        nat_add(a5664, a5665)?
    };
    let a5667 = v1;
    nat_add(a5666, a5667)
}

pub fn f242(v0: u64, v1: u64) -> R<u64> {
    let v2: u64 = f264(v0, v1)?;
    let v3: u64 = {
        let a5672 = {
            let a5670 = {
                let a5668 = v1;
                let a5669 = 1u64;
                nat_sub(a5668, a5669)
            };
            let a5671 = v2;
            nat_mul(a5670, a5671)?
        };
        let a5673 = v0;
        nat_sub(a5672, a5673)
    };
    let a5674 = v2;
    let a5675 = v3;
    if nat_lt(a5674, a5675) {
        Ok(v3)
    } else {
        Ok(v2)
    }
}

pub fn f243(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64) -> R<u64> {
    let a5676 = v4;
    let a5677 = 0u64;
    if nat_eq(a5676, a5677) {
        Ok(0u64)
    } else {
        let a5684 = 2u64;
        let a5685 = {
            let a5682 = {
                let a5678 = v0;
                let a5679 = v1;
                nat_mul(a5678, a5679)?
            };
            let a5683 = {
                let a5680 = v2;
                let a5681 = v3;
                nat_mul(a5680, a5681)?
            };
            nat_mul(a5682, a5683)?
        };
        nat_mul(a5684, a5685)
    }
}

pub fn f244(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64) -> R<u64> {
    let a5690 = v3;
    let a5691 = {
        let a5688 = {
            let a5686 = f265(v2, v4)?;
            let a5687 = f266(v0, v1, f264(v2, v4)?)?;
            nat_mul(a5686, a5687)?
        };
        let a5689 = f266(v0, v1, f267(v2, v4)?)?;
        nat_add(a5688, a5689)?
    };
    nat_mul(a5690, a5691)
}

pub fn f245(v0: u64) -> R<u64> {
    let a5694 = 13u64;
    let a5695 = {
        let a5692 = 9u64;
        let a5693 = v0;
        nat_mul(a5692, a5693)?
    };
    nat_add(a5694, a5695)
}

pub fn f246(v0: u64, v1: u64, _: u64, v3: u64, v4: u64, v5: u64, v6: u64, v7: u64, v8: u64, v9: u64, v10: u64, v11: u64, v12: u64, v13: u64, v14: u64) -> bool {
    if if if if if if if if if f268(v0, v1, v6, v7, v8) {
        let a5696 = v5;
        let a5697 = 26u64;
        nat_eq(a5696, a5697)
    } else {
        false
    } {
        let a5698 = v3;
        let a5699 = v7;
        nat_eq(a5698, a5699)
    } else {
        false
    } {
        let a5700 = v4;
        let a5701 = v8;
        nat_eq(a5700, a5701)
    } else {
        false
    } {
        let a5702 = v9;
        let a5703 = 1u64;
        nat_eq(a5702, a5703)
    } else {
        false
    } {
        let a5704 = v10;
        let a5705 = f269(v0);
        nat_eq(a5704, a5705)
    } else {
        false
    } {
        let a5706 = v11;
        let a5707 = v1;
        nat_eq(a5706, a5707)
    } else {
        false
    } {
        let a5708 = v12;
        let a5709 = v6;
        nat_eq(a5708, a5709)
    } else {
        false
    } {
        let a5710 = v13;
        let a5711 = v7;
        nat_eq(a5710, a5711)
    } else {
        false
    } {
        let a5712 = v14;
        let a5713 = v8;
        nat_eq(a5712, a5713)
    } else {
        false
    }
}

pub fn f247(v0: u64, v1: u64, v2: u64, v3: u64) -> bool {
    let a5714 = v0;
    let a5715 = 0u64;
    if if if nat_eq(a5714, a5715) {
        true
    } else {
        let a5716 = v1;
        let a5717 = 0u64;
        nat_eq(a5716, a5717)
    } {
        true
    } else {
        let a5718 = v2;
        let a5719 = 0u64;
        nat_eq(a5718, a5719)
    } {
        true
    } else {
        let a5720 = v3;
        let a5721 = 0u64;
        nat_eq(a5720, a5721)
    }
}

pub fn f248(v0: Adt3, v1: bool) -> R<u64> {
    f252(v0.clone(), {
        let a5723 = match v0.clone() {
            Adt3::C0(h5722, _, _, _, _, _, _, _) => {
                h5722
            }
        };
        let a5724 = 1u64;
        nat_sub(a5723, a5724)
    }, {
        let a5726 = match v0.clone() {
            Adt3::C0(_, h5725, _, _, _, _, _, _) => {
                h5725
            }
        };
        let a5727 = 1u64;
        nat_sub(a5726, a5727)
    }, {
        let a5729 = match v0.clone() {
            Adt3::C0(_, _, h5728, _, _, _, _, _) => {
                h5728
            }
        };
        let a5730 = 1u64;
        nat_sub(a5729, a5730)
    }, {
        let a5732 = match v0 {
            Adt3::C0(_, _, _, h5731, _, _, _, _) => {
                h5731
            }
        };
        let a5733 = 1u64;
        nat_sub(a5732, a5733)
    }, v1)
}

pub fn f249(v0: u64, v1: u64, v2: u64, v3: u64) -> R<u64> {
    let a5734 = v0;
    let a5735 = 0u64;
    if nat_eq(a5734, a5735) {
        Ok(0u64)
    } else {
        let v4: u64 = {
            let a5736 = v0;
            let a5737 = 1u64;
            nat_sub(a5736, a5737)
        };
        let v5: u64 = {
            let a5738 = v4;
            let a5739 = v1;
            let a5740 = 0u64;
            nat_quot(a5738, a5739, a5740)
        };
        let v6: u64 = {
            let a5748 = {
                let a5741 = v5;
                let a5742 = v2;
                nat_mul(a5741, a5742)?
            };
            let a5749 = {
                let a5746 = {
                    let a5743 = v4;
                    let a5744 = v1;
                    let a5745 = 0u64;
                    nat_rem(a5743, a5744, a5745)
                };
                let a5747 = v3;
                nat_mul(a5746, a5747)?
            };
            nat_add(a5748, a5749)?
        };
        let a5750 = v5;
        let a5751 = 0u64;
        if nat_eq(a5750, a5751) {
            Ok(v6)
        } else {
            let v7: u64 = {
                let a5760 = {
                    let a5754 = {
                        let a5752 = v5;
                        let a5753 = 1u64;
                        nat_sub(a5752, a5753)
                    };
                    let a5755 = v2;
                    nat_mul(a5754, a5755)?
                };
                let a5761 = {
                    let a5758 = {
                        let a5756 = v1;
                        let a5757 = 1u64;
                        nat_sub(a5756, a5757)
                    };
                    let a5759 = v3;
                    nat_mul(a5758, a5759)?
                };
                nat_add(a5760, a5761)?
            };
            let a5762 = v6;
            let a5763 = v7;
            if nat_lt(a5762, a5763) {
                Ok(v7)
            } else {
                Ok(v6)
            }
        }
    }
}

pub fn f250(v0: Adt3, v1: u64) -> bool {
    let a5764 = v1;
    let a5765 = 0u64;
    if !nat_eq(a5764, a5765) {
        true
    } else {
        let a5767 = match v0 {
            Adt3::C0(_, _, _, _, h5766, _, _, _) => {
                h5766
            }
        };
        let a5768 = 4u64;
        nat_eq(a5767, a5768)
    }
}

pub fn f251(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: bool, v6: u64, v7: u64, v8: u64, v9: u64, v10: u64, v11: u64, v12: u64, v13: u64, v14: u64, v15: u64, v16: u64, v17: u64) -> R<Adt10> {
    let m5769 = f270(v0, v1);
    match m5769 {
        None => {
            if v5 {
                let m5770 = f271(v0, v2, v3, v4, v6, v7, v8, v9, v10, v11, v12, v13, v14, v15, v16, v17)?;
                match m5770 {
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
                let a5771 = 4u64;
                let a5772 = v28;
                nat_le(a5771, a5772)
            };
            let a5773 = v2;
            let a5774 = 0u64;
            if if if if if nat_eq(a5773, a5774) {
                let a5775 = v4;
                let a5776 = 0u64;
                nat_eq(a5775, a5776)
            } else {
                false
            } {
                let a5777 = v6;
                let a5778 = v14;
                if nat_eq(a5777, a5778) {
                    let a5779 = v7;
                    let a5780 = v15;
                    nat_eq(a5779, a5780)
                } else {
                    false
                }
            } else {
                false
            } {
                let a5781 = v8;
                let a5782 = v16;
                if nat_eq(a5781, a5782) {
                    let a5783 = v9;
                    let a5784 = v17;
                    nat_eq(a5783, a5784)
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
                        let a5785 = v3;
                        let a5786 = 0u64;
                        nat_eq(a5785, a5786)
                    } else {
                        false
                    } {
                        f202(v6, v7, v8, v9, v10, v11, v12, v13)
                    } else {
                        false
                    }
                }
            } else {
                false
            } {
                Ok(Adt10::C2(v28, v29, v6, v7, v8, v9, {
                    let a5791 = {
                        let a5787 = v6;
                        let a5788 = v7;
                        nat_mul(a5787, a5788)?
                    };
                    let a5792 = {
                        let a5789 = v8;
                        let a5790 = v9;
                        nat_mul(a5789, a5790)?
                    };
                    nat_mul(a5791, a5792)?
                }))
            } else {
                Ok(Adt10::C1)
            }
        }
    }
}

pub fn f252(v0: Adt3, v1: u64, v2: u64, v3: u64, v4: u64, v5: bool) -> R<u64> {
    let a5813 = {
        let a5807 = {
            let a5801 = {
                let a5795 = v1;
                let a5796 = if v5 {
                    f124(match v0.clone() {
                        Adt3::C0(_, _, _, _, h5793, _, _, _) => {
                            h5793
                        }
                    })
                } else {
                    f125(match v0.clone() {
                        Adt3::C0(_, _, _, _, h5794, _, _, _) => {
                            h5794
                        }
                    })
                };
                nat_mul(a5795, a5796)?
            };
            let a5802 = {
                let a5799 = v2;
                let a5800 = if v5 {
                    f124(match v0.clone() {
                        Adt3::C0(_, _, _, _, _, h5797, _, _) => {
                            h5797
                        }
                    })
                } else {
                    f125(match v0.clone() {
                        Adt3::C0(_, _, _, _, _, h5798, _, _) => {
                            h5798
                        }
                    })
                };
                nat_mul(a5799, a5800)?
            };
            nat_add(a5801, a5802)?
        };
        let a5808 = {
            let a5805 = v3;
            let a5806 = if v5 {
                f124(match v0.clone() {
                    Adt3::C0(_, _, _, _, _, _, h5803, _) => {
                        h5803
                    }
                })
            } else {
                f125(match v0.clone() {
                    Adt3::C0(_, _, _, _, _, _, h5804, _) => {
                        h5804
                    }
                })
            };
            nat_mul(a5805, a5806)?
        };
        nat_add(a5807, a5808)?
    };
    let a5814 = {
        let a5811 = v4;
        let a5812 = if v5 {
            f124(match v0 {
                Adt3::C0(_, _, _, _, _, _, _, h5809) => {
                    h5809
                }
            })
        } else {
            f125(match v0 {
                Adt3::C0(_, _, _, _, _, _, _, h5810) => {
                    h5810
                }
            })
        };
        nat_mul(a5811, a5812)?
    };
    nat_add(a5813, a5814)
}

pub fn f253(v0: u64, v1: u64, v2: Adt3, v3: u64, v4: u64) -> R<Adt9> {
    let a5815 = v1;
    let a5816 = 0u64;
    if nat_eq(a5815, a5816) {
        Ok(Adt9::C4({
            let a5817 = v3;
            let a5818 = 0u64;
            if nat_eq(a5817, a5818) {
                1u64
            } else {
                6u64
            }
        }, 0u64, f76(v0, v2)?, 2u64, 0u64, 1u64, 2u64))
    } else {
        Ok(Adt9::C4({
            let a5819 = v3;
            let a5820 = 0u64;
            if nat_eq(a5819, a5820) {
                0u64
            } else {
                4u64
            }
        }, 1u64, v4, 1u64, 2u64, 1u64, v4))
    }
}

pub fn f254(v0: u64, v1: u64) -> R<u64> {
    let a5821 = v0;
    let a5822 = f79(v1)?;
    let a5823 = 0u64;
    Ok(nat_quot(a5821, a5822, a5823))
}

pub fn f255(v0: u64, v1: u64) -> R<u64> {
    let v2: u64 = f254(v0, v1)?;
    let v3: u64 = {
        let a5826 = {
            let a5824 = f225(v0, v1)?;
            let a5825 = 2u64;
            nat_sub(a5824, a5825)
        };
        let a5827 = 2u64;
        let a5828 = 0u64;
        nat_quot(a5826, a5827, a5828)
    };
    let v4: u64 = {
        let a5829 = v2;
        let a5830 = v1;
        nat_mul(a5829, a5830)?
    };
    if f228(v0, v1)? {
        let a5831 = v4;
        let a5832 = v3;
        nat_add(a5831, a5832)
    } else {
        let a5844 = {
            let a5840 = v4;
            let a5841 = {
                let a5838 = {
                    let a5833 = v1;
                    let a5834 = 1u64;
                    nat_sub(a5833, a5834)
                };
                let a5839 = {
                    let a5835 = v2;
                    let a5836 = v1;
                    let a5837 = 0u64;
                    nat_rem(a5835, a5836, a5837)
                };
                nat_mul(a5838, a5839)?
            };
            nat_sub(a5840, a5841)
        };
        let a5845 = {
            let a5842 = v3;
            let a5843 = v1;
            nat_mul(a5842, a5843)?
        };
        nat_add(a5844, a5845)
    }
}

pub fn f256(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let a5846 = v0;
    let a5847 = 0u64;
    if if if nat_eq(a5846, a5847) {
        true
    } else {
        let a5848 = v1;
        let a5849 = 0u64;
        nat_eq(a5848, a5849)
    } {
        true
    } else {
        let a5850 = v2;
        let a5851 = 0u64;
        nat_eq(a5850, a5851)
    } {
        Ok(0u64)
    } else {
        let a5854 = {
            let a5852 = v0;
            let a5853 = v1;
            nat_mul(a5852, a5853)?
        };
        let a5855 = v2;
        nat_mul(a5854, a5855)
    }
}

pub fn f257(v0: u64, v1: Adt20, v2: Adt20, v3: u64, v4: u64) -> R<Adt17> {
    let v5: u64 = f88(match v1.clone() {
        Adt20::C0(h5856, _, _, _, _, _, _, _) => {
            h5856
        }
    })?;
    let v6: u64 = {
        let a5862 = match v1.clone() {
            Adt20::C0(_, h5857, _, _, _, _, _, _) => {
                h5857
            }
        };
        let a5863 = {
            let a5860 = match v1.clone() {
                Adt20::C0(_, _, h5858, _, _, _, _, _) => {
                    h5858
                }
            };
            let a5861 = match v1.clone() {
                Adt20::C0(_, _, _, h5859, _, _, _, _) => {
                    h5859
                }
            };
            nat_mul(a5860, a5861)?
        };
        nat_mul(a5862, a5863)?
    };
    let a5865 = v0;
    let a5866 = f43(match v1.clone() {
        Adt20::C0(h5864, _, _, _, _, _, _, _) => {
            h5864
        }
    }, v6)?;
    if nat_lt(a5865, a5866) {
        f272({
            let a5867 = v0;
            let a5868 = v5;
            let a5869 = 0u64;
            nat_rem(a5867, a5868, a5869)
        }, {
            let a5870 = v0;
            let a5871 = v5;
            let a5872 = 0u64;
            nat_quot(a5870, a5871, a5872)
        }, v1, v2, v3, v4)
    } else {
        Ok(Adt17::C0)
    }
}

pub fn f258(v0: u64, v1: Adt20, v2: Adt20, v3: Adt20, v4: u64, v5: u64, v6: u64, v7: u64) -> R<Adt17> {
    let v8: u64 = {
        let a5873 = v6;
        let a5874 = 6u64;
        nat_sub(a5873, a5874)
    };
    let v9: Adt20 = f273(v3);
    let a5876 = v0;
    let a5877 = f189(match v1.clone() {
        Adt20::C0(h5875, _, _, _, _, _, _, _) => {
            h5875
        }
    }, v5, v8)?;
    if nat_lt(a5876, a5877) {
        let v10: u64 = f85(match v1.clone() {
            Adt20::C0(h5878, _, _, _, _, _, _, _) => {
                h5878
            }
        }, v5, v8)?;
        let a5879 = v0;
        let a5880 = v10;
        if nat_lt(a5879, a5880) {
            f274(v0, v9, 4u64, 6u64, v7)
        } else {
            let v11: u64 = {
                let a5881 = v0;
                let a5882 = v10;
                nat_sub(a5881, a5882)
            };
            let v12: u64 = f245(match v1.clone() {
                Adt20::C0(h5883, _, _, _, _, _, _, _) => {
                    h5883
                }
            })?;
            f275({
                let a5884 = v11;
                let a5885 = v12;
                let a5886 = 0u64;
                nat_rem(a5884, a5885, a5886)
            }, {
                let a5887 = v11;
                let a5888 = v12;
                let a5889 = 0u64;
                nat_quot(a5887, a5888, a5889)
            }, v1, v2, v9, v4)
        }
    } else {
        Ok(Adt17::C0)
    }
}

pub fn f259(v0: u64, v1: Adt20, v2: Adt20, v3: u64, v4: u64, v5: u64, v6: u64) -> R<Adt17> {
    let v7: u64 = {
        let a5890 = v5;
        let a5891 = 6u64;
        nat_sub(a5890, a5891)
    };
    let v8: bool = f183(match v1.clone() {
        Adt20::C0(_, _, _, _, h5892, _, _, _) => {
            h5892
        }
    }, match v2.clone() {
        Adt20::C0(_, _, _, _, h5893, _, _, _) => {
            h5893
        }
    });
    let a5895 = v0;
    let a5896 = f182(v8, match v1.clone() {
        Adt20::C0(h5894, _, _, _, _, _, _, _) => {
            h5894
        }
    }, v4, v7)?;
    if nat_lt(a5895, a5896) {
        let v9: u64 = f85(match v1.clone() {
            Adt20::C0(h5897, _, _, _, _, _, _, _) => {
                h5897
            }
        }, v4, v7)?;
        let a5898 = v0;
        let a5899 = v9;
        if nat_lt(a5898, a5899) {
            f274(v0, v2, 4u64, 6u64, v6)
        } else {
            let v10: u64 = {
                let a5900 = v0;
                let a5901 = v9;
                nat_sub(a5900, a5901)
            };
            let v11: u64 = f241(v8, match v1.clone() {
                Adt20::C0(h5902, _, _, _, _, _, _, _) => {
                    h5902
                }
            })?;
            f276({
                let a5903 = v10;
                let a5904 = v11;
                let a5905 = 0u64;
                nat_rem(a5903, a5904, a5905)
            }, {
                let a5906 = v10;
                let a5907 = v11;
                let a5908 = 0u64;
                nat_quot(a5906, a5907, a5908)
            }, v1, v2, v3, v8)
        }
    } else {
        Ok(Adt17::C0)
    }
}

pub fn f260(v0: u64, v1: Adt20, v2: Adt20, v3: u64, v4: u64, v5: u64, v6: u64) -> R<Adt17> {
    let v7: u64 = {
        let a5909 = v5;
        let a5910 = 6u64;
        nat_sub(a5909, a5910)
    };
    let v8: Adt20 = f273(v2);
    let v9: u64 = f243(match v1.clone() {
        Adt20::C0(h5911, _, _, _, _, _, _, _) => {
            h5911
        }
    }, match v1.clone() {
        Adt20::C0(_, h5912, _, _, _, _, _, _) => {
            h5912
        }
    }, match v1.clone() {
        Adt20::C0(_, _, h5913, _, _, _, _, _) => {
            h5913
        }
    }, match v1.clone() {
        Adt20::C0(_, _, _, h5914, _, _, _, _) => {
            h5914
        }
    }, v7)?;
    let a5919 = v0;
    let a5920 = f188(match v1.clone() {
        Adt20::C0(h5915, _, _, _, _, _, _, _) => {
            h5915
        }
    }, match v1.clone() {
        Adt20::C0(_, h5916, _, _, _, _, _, _) => {
            h5916
        }
    }, match v1.clone() {
        Adt20::C0(_, _, h5917, _, _, _, _, _) => {
            h5917
        }
    }, match v1.clone() {
        Adt20::C0(_, _, _, h5918, _, _, _, _) => {
            h5918
        }
    }, v4, v7)?;
    if nat_lt(a5919, a5920) {
        let a5921 = v0;
        let a5922 = v9;
        if nat_lt(a5921, a5922) {
            f274(v0, v8, 4u64, 6u64, v6)
        } else {
            let v10: Adt21 = f277({
                let a5923 = v0;
                let a5924 = v9;
                nat_sub(a5923, a5924)
            }, match v1.clone() {
                Adt20::C0(h5925, _, _, _, _, _, _, _) => {
                    h5925
                }
            }, match v1.clone() {
                Adt20::C0(_, h5926, _, _, _, _, _, _) => {
                    h5926
                }
            }, match v1.clone() {
                Adt20::C0(_, _, h5927, _, _, _, _, _) => {
                    h5927
                }
            }, match v1.clone() {
                Adt20::C0(_, _, _, h5928, _, _, _, _) => {
                    h5928
                }
            }, v4)?;
            f278(v10, v1, v8, v3)
        }
    } else {
        Ok(Adt17::C0)
    }
}

pub fn f261(v0: bool, v1: u64) -> R<u64> {
    let a5931 = if v0 {
        5u64
    } else {
        4u64
    };
    let a5932 = {
        let a5929 = f279(v0);
        let a5930 = v1;
        nat_mul(a5929, a5930)?
    };
    nat_add(a5931, a5932)
}

pub fn f262(v0: bool, v1: u64) -> R<u64> {
    if v0 {
        let a5940 = {
            let a5933 = 13u64;
            let a5934 = f280(v1);
            nat_mul(a5933, a5934)?
        };
        let a5941 = {
            let a5938 = 4u64;
            let a5939 = {
                let a5935 = v1;
                let a5936 = 4u64;
                let a5937 = 0u64;
                nat_rem(a5935, a5936, a5937)
            };
            nat_mul(a5938, a5939)?
        };
        nat_add(a5940, a5941)
    } else {
        let a5942 = 3u64;
        let a5943 = v1;
        nat_mul(a5942, a5943)
    }
}

pub fn f263(v0: bool) -> u64 {
    if v0 {
        6u64
    } else {
        5u64
    }
}

pub fn f264(v0: u64, v1: u64) -> R<u64> {
    let a5948 = {
        let a5946 = {
            let a5944 = v0;
            let a5945 = v1;
            nat_add(a5944, a5945)?
        };
        let a5947 = 1u64;
        nat_sub(a5946, a5947)
    };
    let a5949 = v1;
    let a5950 = 0u64;
    Ok(nat_quot(a5948, a5949, a5950))
}

pub fn f265(v0: u64, v1: u64) -> R<u64> {
    let a5951 = v0;
    let a5952 = f264(v0, v1)?;
    let a5953 = 0u64;
    Ok(nat_quot(a5951, a5952, a5953))
}

pub fn f266(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let a5954 = v2;
    let a5955 = 0u64;
    if nat_eq(a5954, a5955) {
        Ok(0u64)
    } else {
        let a5960 = 11u64;
        let a5961 = {
            let a5958 = {
                let a5956 = v1;
                let a5957 = v2;
                nat_mul(a5956, a5957)?
            };
            let a5959 = f281(v0)?;
            nat_mul(a5958, a5959)?
        };
        nat_add(a5960, a5961)
    }
}

pub fn f267(v0: u64, v1: u64) -> R<u64> {
    let a5962 = v0;
    let a5963 = f264(v0, v1)?;
    let a5964 = 0u64;
    Ok(nat_rem(a5962, a5963, a5964))
}

pub fn f268(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64) -> bool {
    if f247(v1, v2, v3, v4) {
        f282(v0)
    } else {
        f199(v0)
    }
}

pub fn f269(v0: u64) -> u64 {
    let a5965 = v0;
    let a5966 = 26u64;
    if nat_eq(a5965, a5966) {
        26u64
    } else {
        0u64
    }
}

pub fn f270(v0: u64, v1: u64) -> Option<u64> {
    let a5967 = v0;
    let a5968 = 2u64;
    if nat_eq(a5967, a5968) {
        Some(0u64)
    } else {
        let a5969 = v0;
        let a5970 = 5u64;
        if nat_eq(a5969, a5970) {
            Some(1u64)
        } else {
            let a5971 = v0;
            let a5972 = 6u64;
            if nat_eq(a5971, a5972) {
                Some(2u64)
            } else {
                let a5973 = v0;
                let a5974 = 7u64;
                if nat_eq(a5973, a5974) {
                    Some(3u64)
                } else {
                    let a5975 = v0;
                    let a5976 = 9u64;
                    if nat_eq(a5975, a5976) {
                        Some(4u64)
                    } else {
                        let a5977 = v0;
                        let a5978 = 8u64;
                        if nat_eq(a5977, a5978) {
                            Some(5u64)
                        } else {
                            let a5979 = v0;
                            let a5980 = 89u64;
                            if nat_eq(a5979, a5980) {
                                let a5981 = v1;
                                let a5982 = 2u64;
                                if nat_eq(a5981, a5982) {
                                    Some(6u64)
                                } else {
                                    let a5983 = v1;
                                    let a5984 = 0u64;
                                    if nat_eq(a5983, a5984) {
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

pub fn f271(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64, v7: u64, v8: u64, v9: u64, v10: u64, v11: u64, v12: u64, v13: u64, v14: u64, v15: u64) -> R<Adt22> {
    let a5985 = v0;
    let a5986 = 30u64;
    if nat_eq(a5985, a5986) {
        let a5987 = v2;
        let a5988 = 0u64;
        if if if nat_eq(a5987, a5988) {
            true
        } else {
            let a5989 = v2;
            let a5990 = 1u64;
            nat_eq(a5989, a5990)
        } {
            let a5991 = v1;
            let a5992 = 0u64;
            if nat_eq(a5991, a5992) {
                true
            } else {
                f283(v1, v4)
            }
        } else {
            false
        } {
            f284(28u64, 0u64, 0u64, v3, v5, v4, v6, v7, v9, v8, v10, v11, v12, v13, v14, v15)
        } else {
            Ok(Adt22::C0)
        }
    } else {
        if if if if f285(v1) {
            f285(v2)
        } else {
            false
        } {
            let a5993 = v1;
            let a5994 = 0u64;
            if if nat_eq(a5993, a5994) {
                true
            } else {
                let a5995 = v2;
                let a5996 = 0u64;
                nat_eq(a5995, a5996)
            } {
                true
            } else {
                let a5997 = v1;
                let a5998 = v2;
                nat_eq(a5997, a5998)
            }
        } else {
            false
        } {
            true
        } else {
            let a5999 = v1;
            let a6000 = 43u64;
            if nat_eq(a5999, a6000) {
                f285(v2)
            } else {
                false
            }
        } {
            f284(v0, 0u64, 0u64, v3, v4, v5, v6, v7, v8, v9, v10, v11, v12, v13, v14, v15)
        } else {
            Ok(Adt22::C0)
        }
    }
}

pub fn f272(v0: u64, v1: u64, v2: Adt20, v3: Adt20, v4: u64, v5: u64) -> R<Adt17> {
    let v6: u64 = match v2.clone() {
        Adt20::C0(h6001, _, _, _, _, _, _, _) => {
            h6001
        }
    };
    let v7: u64 = {
        let a6004 = 1u64;
        let a6005 = {
            let a6002 = 3u64;
            let a6003 = v6;
            nat_mul(a6002, a6003)?
        };
        nat_add(a6004, a6005)?
    };
    let a6006 = v0;
    let a6007 = 0u64;
    if nat_eq(a6006, a6007) {
        Ok(f286(0u64))
    } else {
        let a6008 = v0;
        let a6009 = v7;
        if nat_lt(a6008, a6009) {
            let v8: u64 = {
                let a6010 = v0;
                let a6011 = 1u64;
                nat_sub(a6010, a6011)
            };
            let v9: u64 = {
                let a6017 = {
                    let a6012 = v1;
                    let a6013 = v6;
                    nat_mul(a6012, a6013)?
                };
                let a6018 = {
                    let a6014 = v8;
                    let a6015 = 3u64;
                    let a6016 = 0u64;
                    nat_quot(a6014, a6015, a6016)
                };
                nat_add(a6017, a6018)?
            };
            let v10: u64 = f287(v9, v2)?;
            let v11: u64 = {
                let a6019 = v8;
                let a6020 = 3u64;
                let a6021 = 0u64;
                nat_rem(a6019, a6020, a6021)
            };
            let a6022 = v11;
            let a6023 = 0u64;
            if nat_eq(a6022, a6023) {
                Ok(f288(10u64, 0u64, v10, 0u64, v10, 1u64, 1u64))
            } else {
                let a6024 = v11;
                let a6025 = 1u64;
                if nat_eq(a6024, a6025) {
                    Ok(f288(1u64, 1u64, 1u64, 2u64, 0u64, 1u64, 1u64))
                } else {
                    Ok(f288(0u64, 1u64, 0u64, 1u64, 1u64, 1u64, 0u64))
                }
            }
        } else {
            let a6026 = v0;
            let a6027 = v7;
            if nat_eq(a6026, a6027) {
                Ok(f288(2u64, 1u64, 0u64, 2u64, 0u64, 1u64, 1u64))
            } else {
                let a6030 = v0;
                let a6031 = {
                    let a6028 = v7;
                    let a6029 = 1u64;
                    nat_add(a6028, a6029)?
                };
                if nat_eq(a6030, a6031) {
                    Ok(f288(12u64, 1u64, 1u64, 2u64, 0u64, 1u64, 1u64))
                } else {
                    let a6034 = v0;
                    let a6035 = {
                        let a6032 = v7;
                        let a6033 = 2u64;
                        nat_add(a6032, a6033)?
                    };
                    if nat_eq(a6034, a6035) {
                        Ok(f288(16u64, 1u64, 1u64, 2u64, v4, 1u64, 2u64))
                    } else {
                        let a6038 = v0;
                        let a6039 = {
                            let a6036 = v7;
                            let a6037 = 3u64;
                            nat_add(a6036, a6037)?
                        };
                        if nat_eq(a6038, a6039) {
                            let a6040 = v5;
                            let a6041 = 1u64;
                            if if nat_eq(a6040, a6041) {
                                true
                            } else {
                                let a6042 = v5;
                                let a6043 = 2u64;
                                nat_eq(a6042, a6043)
                            } {
                                Ok(f288(11u64, 2u64, 1065353216u64, 1u64, 1u64, 1u64, 2u64))
                            } else {
                                Ok(f288(11u64, 2u64, 1065353216u64, 2u64, v4, 1u64, 2u64))
                            }
                        } else {
                            let v12: u64 = {
                                let a6050 = {
                                    let a6044 = v1;
                                    let a6045 = v6;
                                    nat_mul(a6044, a6045)?
                                };
                                let a6051 = {
                                    let a6048 = v0;
                                    let a6049 = {
                                        let a6046 = v7;
                                        let a6047 = 4u64;
                                        nat_add(a6046, a6047)?
                                    };
                                    nat_sub(a6048, a6049)
                                };
                                nat_add(a6050, a6051)?
                            };
                            Ok(f288(10u64, 0u64, f287(v12, v2)?, 1u64, 2u64, 3u64, f287(v12, v3)?))
                        }
                    }
                }
            }
        }
    }
}

pub fn f273(v0: Adt20) -> Adt20 {
    Adt20::C0(match v0.clone() {
        Adt20::C0(h6052, _, _, _, _, _, _, _) => {
            h6052
        }
    }, match v0.clone() {
        Adt20::C0(_, h6053, _, _, _, _, _, _) => {
            h6053
        }
    }, match v0.clone() {
        Adt20::C0(_, _, h6054, _, _, _, _, _) => {
            h6054
        }
    }, match v0.clone() {
        Adt20::C0(_, _, _, h6055, _, _, _, _) => {
            h6055
        }
    }, 4u64, match v0.clone() {
        Adt20::C0(_, _, _, _, _, h6056, _, _) => {
            h6056
        }
    }, match v0.clone() {
        Adt20::C0(_, _, _, _, _, _, h6057, _) => {
            h6057
        }
    }, match v0 {
        Adt20::C0(_, _, _, _, _, _, _, h6058) => {
            h6058
        }
    })
}

pub fn f274(v0: u64, v1: Adt20, v2: u64, v3: u64, v4: u64) -> R<Adt17> {
    let v5: u64 = {
        let a6065 = v3;
        let a6066 = {
            let a6062 = f287({
                let a6059 = v0;
                let a6060 = 2u64;
                let a6061 = 0u64;
                nat_quot(a6059, a6060, a6061)
            }, v1)?;
            let a6063 = v2;
            let a6064 = 0u64;
            nat_quot(a6062, a6063, a6064)
        };
        nat_add(a6065, a6066)?
    };
    let a6070 = {
        let a6067 = v0;
        let a6068 = 2u64;
        let a6069 = 0u64;
        nat_rem(a6067, a6068, a6069)
    };
    let a6071 = 0u64;
    if nat_eq(a6070, a6071) {
        Ok(f288(19u64, 1u64, v5, 2u64, 0u64, 1u64, 2u64))
    } else {
        let a6072 = v4;
        let a6073 = 0u64;
        if nat_eq(a6072, a6073) {
            Ok(f288(19u64, 2u64, 1u64, 2u64, 0u64, 1u64, v5))
        } else {
            Ok(Adt17::C0)
        }
    }
}

pub fn f275(v0: u64, v1: u64, v2: Adt20, v3: Adt20, v4: Adt20, v5: u64) -> R<Adt17> {
    let v6: u64 = f289(match v2.clone() {
        Adt20::C0(h6074, _, _, _, _, _, _, _) => {
            h6074
        }
    })?;
    let v7: u64 = f290(match v2.clone() {
        Adt20::C0(h6075, _, _, _, _, _, _, _) => {
            h6075
        }
    })?;
    let a6076 = v0;
    let a6077 = 2u64;
    if nat_lt(a6076, a6077) {
        Ok(f286(v0))
    } else {
        let a6078 = v0;
        let a6079 = v6;
        if nat_lt(a6078, a6079) {
            f291({
                let a6080 = v0;
                let a6081 = 2u64;
                nat_sub(a6080, a6081)
            }, v1, v2, v3)
        } else {
            let a6082 = v0;
            let a6083 = v7;
            if nat_lt(a6082, a6083) {
                Ok(f292({
                    let a6084 = v0;
                    let a6085 = v6;
                    nat_sub(a6084, a6085)
                }, match v2 {
                    Adt20::C0(h6086, _, _, _, _, _, _, _) => {
                        h6086
                    }
                }, v5))
            } else {
                let a6088 = v0;
                let a6089 = f245(match v2.clone() {
                    Adt20::C0(h6087, _, _, _, _, _, _, _) => {
                        h6087
                    }
                })?;
                if nat_lt(a6088, a6089) {
                    f293({
                        let a6090 = v0;
                        let a6091 = v7;
                        nat_sub(a6090, a6091)
                    }, v1, v2, v3, v4)
                } else {
                    Ok(Adt17::C0)
                }
            }
        }
    }
}

pub fn f276(v0: u64, v1: u64, v2: Adt20, v3: Adt20, v4: u64, v5: bool) -> R<Adt17> {
    let v6: u64 = f261(v5, match v2.clone() {
        Adt20::C0(h6092, _, _, _, _, _, _, _) => {
            h6092
        }
    })?;
    let v7: u64 = {
        let a6094 = v6;
        let a6095 = f262(v5, match v2.clone() {
            Adt20::C0(h6093, _, _, _, _, _, _, _) => {
                h6093
            }
        })?;
        nat_add(a6094, a6095)?
    };
    let v8: u64 = {
        let a6096 = v7;
        let a6097 = f263(v5);
        nat_add(a6096, a6097)?
    };
    let a6098 = v0;
    let a6099 = v6;
    if nat_lt(a6098, a6099) {
        f294(v0, v1, v2, v5)
    } else {
        let a6100 = v0;
        let a6101 = v7;
        if nat_lt(a6100, a6101) {
            f295({
                let a6102 = v0;
                let a6103 = v6;
                nat_sub(a6102, a6103)
            }, v1, v2, v3, v5)
        } else {
            let a6104 = v0;
            let a6105 = v8;
            if nat_lt(a6104, a6105) {
                Ok(f296({
                    let a6106 = v0;
                    let a6107 = v7;
                    nat_sub(a6106, a6107)
                }, match v2 {
                    Adt20::C0(h6108, _, _, _, _, _, _, _) => {
                        h6108
                    }
                }, v4, v5))
            } else {
                let a6112 = v0;
                let a6113 = {
                    let a6110 = v8;
                    let a6111 = match v2.clone() {
                        Adt20::C0(h6109, _, _, _, _, _, _, _) => {
                            h6109
                        }
                    };
                    nat_add(a6110, a6111)?
                };
                if nat_lt(a6112, a6113) {
                    let v9: u64 = f287({
                        let a6119 = {
                            let a6115 = v1;
                            let a6116 = match v2 {
                                Adt20::C0(h6114, _, _, _, _, _, _, _) => {
                                    h6114
                                }
                            };
                            nat_mul(a6115, a6116)?
                        };
                        let a6120 = {
                            let a6117 = v0;
                            let a6118 = v8;
                            nat_sub(a6117, a6118)
                        };
                        nat_add(a6119, a6120)?
                    }, v3)?;
                    Ok(f288(10u64, 3u64, v9, 1u64, 1u64, 3u64, v9))
                } else {
                    Ok(Adt17::C0)
                }
            }
        }
    }
}

pub fn f277(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64) -> R<Adt21> {
    let v6: u64 = f264(v3, v5)?;
    let v7: u64 = f266(v1, v2, v6)?;
    let v8: u64 = {
        let a6123 = {
            let a6121 = f265(v3, v5)?;
            let a6122 = v4;
            nat_mul(a6121, a6122)?
        };
        let a6124 = v7;
        nat_mul(a6123, a6124)?
    };
    let a6125 = v0;
    let a6126 = v8;
    if nat_lt(a6125, a6126) {
        let v9: u64 = {
            let a6127 = v0;
            let a6128 = v7;
            let a6129 = 0u64;
            nat_quot(a6127, a6128, a6129)
        };
        Ok(Adt21::C0({
            let a6133 = {
                let a6130 = v9;
                let a6131 = v4;
                let a6132 = 0u64;
                nat_quot(a6130, a6131, a6132)
            };
            let a6134 = v6;
            nat_mul(a6133, a6134)?
        }, v6, {
            let a6135 = v9;
            let a6136 = v4;
            let a6137 = 0u64;
            nat_rem(a6135, a6136, a6137)
        }, {
            let a6138 = v0;
            let a6139 = v7;
            let a6140 = 0u64;
            nat_rem(a6138, a6139, a6140)
        }))
    } else {
        let v10: u64 = {
            let a6141 = v0;
            let a6142 = v8;
            nat_sub(a6141, a6142)
        };
        let v11: u64 = f266(v1, v2, f267(v3, v5)?)?;
        Ok(Adt21::C0({
            let a6143 = f265(v3, v5)?;
            let a6144 = v6;
            nat_mul(a6143, a6144)?
        }, f267(v3, v5)?, {
            let a6145 = v10;
            let a6146 = v11;
            let a6147 = 0u64;
            nat_quot(a6145, a6146, a6147)
        }, {
            let a6148 = v10;
            let a6149 = v11;
            let a6150 = 0u64;
            nat_rem(a6148, a6149, a6150)
        }))
    }
}

pub fn f278(v0: Adt21, v1: Adt20, v2: Adt20, v3: u64) -> R<Adt17> {
    let v4: u64 = match v0.clone() {
        Adt21::C0(_, _, _, h6151) => {
            h6151
        }
    };
    let v5: u64 = match v1.clone() {
        Adt20::C0(h6152, _, _, _, _, _, _, _) => {
            h6152
        }
    };
    let v6: u64 = {
        let a6155 = match v0.clone() {
            Adt21::C0(_, h6153, _, _) => {
                h6153
            }
        };
        let a6156 = match v1.clone() {
            Adt20::C0(_, h6154, _, _, _, _, _, _) => {
                h6154
            }
        };
        nat_mul(a6155, a6156)?
    };
    let v7: u64 = {
        let a6157 = v6;
        let a6158 = v5;
        nat_mul(a6157, a6158)?
    };
    let v8: u64 = {
        let a6161 = {
            let a6159 = 2u64;
            let a6160 = v5;
            nat_mul(a6159, a6160)?
        };
        let a6162 = 2u64;
        nat_add(a6161, a6162)?
    };
    let v9: u64 = {
        let a6165 = {
            let a6163 = 4u64;
            let a6164 = v5;
            nat_mul(a6163, a6164)?
        };
        let a6166 = 2u64;
        nat_add(a6165, a6166)?
    };
    let v10: u64 = {
        let a6169 = 1u64;
        let a6170 = {
            let a6167 = v6;
            let a6168 = v8;
            nat_mul(a6167, a6168)?
        };
        nat_add(a6169, a6170)?
    };
    let v11: u64 = {
        let a6171 = v10;
        let a6172 = 4u64;
        nat_add(a6171, a6172)?
    };
    let v12: u64 = {
        let a6175 = v11;
        let a6176 = {
            let a6173 = v6;
            let a6174 = v9;
            nat_mul(a6173, a6174)?
        };
        nat_add(a6175, a6176)?
    };
    let v13: u64 = {
        let a6177 = v12;
        let a6178 = 6u64;
        nat_add(a6177, a6178)?
    };
    let a6179 = v4;
    let a6180 = 0u64;
    if nat_eq(a6179, a6180) {
        Ok(f286(0u64))
    } else {
        let a6181 = v4;
        let a6182 = v10;
        if nat_lt(a6181, a6182) {
            let v14: u64 = {
                let a6183 = v4;
                let a6184 = 1u64;
                nat_sub(a6183, a6184)
            };
            f297(false, {
                let a6185 = v14;
                let a6186 = v8;
                let a6187 = 0u64;
                nat_rem(a6185, a6186, a6187)
            }, {
                let a6188 = v14;
                let a6189 = v8;
                let a6190 = 0u64;
                nat_quot(a6188, a6189, a6190)
            }, v0, v1, v2)
        } else {
            let a6191 = v4;
            let a6192 = v11;
            if nat_lt(a6191, a6192) {
                Ok(f298({
                    let a6193 = v4;
                    let a6194 = v10;
                    nat_sub(a6193, a6194)
                }, v7))
            } else {
                let a6195 = v4;
                let a6196 = v12;
                if nat_lt(a6195, a6196) {
                    let v15: u64 = {
                        let a6197 = v4;
                        let a6198 = v11;
                        nat_sub(a6197, a6198)
                    };
                    f297(true, {
                        let a6199 = v15;
                        let a6200 = v9;
                        let a6201 = 0u64;
                        nat_rem(a6199, a6200, a6201)
                    }, {
                        let a6202 = v15;
                        let a6203 = v9;
                        let a6204 = 0u64;
                        nat_quot(a6202, a6203, a6204)
                    }, v0, v1, v2)
                } else {
                    let a6205 = v4;
                    let a6206 = v13;
                    if nat_lt(a6205, a6206) {
                        Ok(f299({
                            let a6207 = v4;
                            let a6208 = v12;
                            nat_sub(a6207, a6208)
                        }, v7, v3))
                    } else {
                        let a6211 = v4;
                        let a6212 = {
                            let a6209 = v13;
                            let a6210 = v7;
                            nat_add(a6209, a6210)?
                        };
                        if nat_lt(a6211, a6212) {
                            let v16: u64 = {
                                let a6213 = v4;
                                let a6214 = v13;
                                nat_sub(a6213, a6214)
                            };
                            let v17: u64 = {
                                let a6215 = v16;
                                let a6216 = v5;
                                let a6217 = 0u64;
                                nat_quot(a6215, a6216, a6217)
                            };
                            let v18: u64 = {
                                let a6223 = {
                                    let a6218 = f300(v17, v0, v1)?;
                                    let a6219 = v5;
                                    nat_mul(a6218, a6219)?
                                };
                                let a6224 = {
                                    let a6220 = v16;
                                    let a6221 = v5;
                                    let a6222 = 0u64;
                                    nat_rem(a6220, a6221, a6222)
                                };
                                nat_add(a6223, a6224)?
                            };
                            let v19: u64 = f287(v18, v2)?;
                            Ok(f288(10u64, 3u64, v19, 1u64, 1u64, 3u64, v19))
                        } else {
                            Ok(Adt17::C0)
                        }
                    }
                }
            }
        }
    }
}

pub fn f279(v0: bool) -> u64 {
    if v0 {
        2u64
    } else {
        1u64
    }
}

pub fn f280(v0: u64) -> u64 {
    let a6225 = v0;
    let a6226 = 4u64;
    let a6227 = 0u64;
    nat_quot(a6225, a6226, a6227)
}

pub fn f281(v0: u64) -> R<u64> {
    let a6230 = {
        let a6228 = 7u64;
        let a6229 = v0;
        nat_mul(a6228, a6229)?
    };
    let a6231 = 4u64;
    nat_add(a6230, a6231)
}

pub fn f282(v0: u64) -> bool {
    let m6232 = f192(v0);
    match m6232 {
        Adt18::C0 => {
            false
        }
        Adt18::C1(_) => {
            true
        }
    }
}

pub fn f283(v0: u64, v1: u64) -> bool {
    let a6233 = v0;
    let a6234 = 1u64;
    if nat_eq(a6233, a6234) {
        true
    } else {
        let v2: u64 = f301(v0);
        let a6235 = 0u64;
        let a6236 = v2;
        if nat_lt(a6235, a6236) {
            let a6240 = {
                let a6237 = v1;
                let a6238 = v2;
                let a6239 = 1u64;
                nat_rem(a6237, a6238, a6239)
            };
            let a6241 = 0u64;
            nat_eq(a6240, a6241)
        } else {
            false
        }
    }
}

pub fn f284(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64, v7: u64, v8: u64, v9: u64, v10: u64, v11: u64, v12: u64, v13: u64, v14: u64, v15: u64) -> R<Adt22> {
    let a6242 = v0;
    let a6243 = 28u64;
    if if if if nat_eq(a6242, a6243) {
        let a6244 = v1;
        let a6245 = 0u64;
        nat_eq(a6244, a6245)
    } else {
        false
    } {
        let a6246 = v2;
        let a6247 = 0u64;
        nat_eq(a6246, a6247)
    } else {
        false
    } {
        let a6248 = v3;
        let a6249 = 0u64;
        nat_eq(a6248, a6249)
    } else {
        false
    } {
        let a6250 = v4;
        let a6251 = v8;
        if if if if if if if if nat_eq(a6250, a6251) {
            let a6252 = v5;
            let a6253 = v12;
            nat_eq(a6252, a6253)
        } else {
            false
        } {
            let a6254 = v9;
            let a6255 = v13;
            nat_eq(a6254, a6255)
        } else {
            false
        } {
            let a6256 = v10;
            let a6257 = v14;
            nat_eq(a6256, a6257)
        } else {
            false
        } {
            let a6258 = v11;
            let a6259 = v15;
            nat_eq(a6258, a6259)
        } else {
            false
        } {
            let a6260 = 0u64;
            let a6261 = v6;
            nat_lt(a6260, a6261)
        } else {
            false
        } {
            let a6262 = 0u64;
            let a6263 = v7;
            nat_lt(a6262, a6263)
        } else {
            false
        } {
            let a6267 = {
                let a6264 = v10;
                let a6265 = v6;
                let a6266 = 1u64;
                nat_rem(a6264, a6265, a6266)
            };
            let a6268 = 0u64;
            if nat_eq(a6267, a6268) {
                let a6272 = {
                    let a6269 = v11;
                    let a6270 = v7;
                    let a6271 = 1u64;
                    nat_rem(a6269, a6270, a6271)
                };
                let a6273 = 0u64;
                nat_eq(a6272, a6273)
            } else {
                false
            }
        } else {
            false
        } {
            Ok(Adt22::C2(v5, v4, v9, v10, v11, {
                let a6274 = v10;
                let a6275 = v6;
                let a6276 = 0u64;
                nat_quot(a6274, a6275, a6276)
            }, {
                let a6277 = v11;
                let a6278 = v7;
                let a6279 = 0u64;
                nat_quot(a6277, a6278, a6279)
            }, {
                let a6280 = v5;
                let a6281 = v4;
                nat_mul(a6280, a6281)?
            }, {
                let a6282 = v4;
                let a6283 = v9;
                nat_mul(a6282, a6283)?
            }, {
                let a6284 = v5;
                let a6285 = v9;
                nat_mul(a6284, a6285)?
            }))
        } else {
            Ok(Adt22::C1)
        }
    } else {
        Ok(Adt22::C0)
    }
}

pub fn f285(v0: u64) -> bool {
    let a6286 = v0;
    let a6287 = 0u64;
    if if nat_eq(a6286, a6287) {
        true
    } else {
        let a6288 = v0;
        let a6289 = 1u64;
        nat_eq(a6288, a6289)
    } {
        true
    } else {
        let a6290 = v0;
        let a6291 = 30u64;
        nat_eq(a6290, a6291)
    }
}

pub fn f286(v0: u64) -> Adt17 {
    f288(0u64, 2u64, 0u64, 2u64, 0u64, 1u64, v0)
}

pub fn f287(v0: u64, v1: Adt20) -> R<u64> {
    let v2: u64 = {
        let a6293 = v0;
        let a6294 = match v1.clone() {
            Adt20::C0(h6292, _, _, _, _, _, _, _) => {
                h6292
            }
        };
        let a6295 = 0u64;
        nat_quot(a6293, a6294, a6295)
    };
    let v3: u64 = {
        let a6297 = v2;
        let a6298 = match v1.clone() {
            Adt20::C0(_, h6296, _, _, _, _, _, _) => {
                h6296
            }
        };
        let a6299 = 0u64;
        nat_quot(a6297, a6298, a6299)
    };
    f123({
        let a6301 = v0;
        let a6302 = match v1.clone() {
            Adt20::C0(h6300, _, _, _, _, _, _, _) => {
                h6300
            }
        };
        let a6303 = 0u64;
        nat_rem(a6301, a6302, a6303)
    }, {
        let a6305 = v2;
        let a6306 = match v1.clone() {
            Adt20::C0(_, h6304, _, _, _, _, _, _) => {
                h6304
            }
        };
        let a6307 = 0u64;
        nat_rem(a6305, a6306, a6307)
    }, {
        let a6309 = v3;
        let a6310 = match v1.clone() {
            Adt20::C0(_, _, h6308, _, _, _, _, _) => {
                h6308
            }
        };
        let a6311 = 0u64;
        nat_rem(a6309, a6310, a6311)
    }, {
        let a6313 = v3;
        let a6314 = match v1.clone() {
            Adt20::C0(_, _, h6312, _, _, _, _, _) => {
                h6312
            }
        };
        let a6315 = 0u64;
        nat_quot(a6313, a6314, a6315)
    }, match v1.clone() {
        Adt20::C0(_, _, _, _, h6316, _, _, _) => {
            h6316
        }
    }, match v1.clone() {
        Adt20::C0(_, _, _, _, _, h6317, _, _) => {
            h6317
        }
    }, match v1.clone() {
        Adt20::C0(_, _, _, _, _, _, h6318, _) => {
            h6318
        }
    }, match v1 {
        Adt20::C0(_, _, _, _, _, _, _, h6319) => {
            h6319
        }
    })
}

pub fn f288(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64) -> Adt17 {
    let a6320 = v0;
    let a6321 = 4u64;
    if if if nat_le(a6320, a6321) {
        true
    } else {
        let a6322 = 9u64;
        let a6323 = v0;
        if nat_le(a6322, a6323) {
            let a6324 = v0;
            let a6325 = 12u64;
            nat_le(a6324, a6325)
        } else {
            false
        }
    } {
        true
    } else {
        let a6326 = v0;
        let a6327 = 16u64;
        if if nat_eq(a6326, a6327) {
            true
        } else {
            let a6328 = v0;
            let a6329 = 19u64;
            nat_eq(a6328, a6329)
        } {
            true
        } else {
            let a6330 = v0;
            let a6331 = 21u64;
            if nat_eq(a6330, a6331) {
                true
            } else {
                let a6332 = v0;
                let a6333 = 22u64;
                nat_eq(a6332, a6333)
            }
        }
    } {
        Adt17::C1({
            let a6334 = v0;
            let a6335 = 4u64;
            if if nat_lt(a6334, a6335) {
                true
            } else {
                let a6336 = 21u64;
                let a6337 = v0;
                nat_le(a6336, a6337)
            } {
                0u64
            } else {
                let a6338 = v0;
                let a6339 = 16u64;
                if if nat_eq(a6338, a6339) {
                    true
                } else {
                    let a6340 = v0;
                    let a6341 = 19u64;
                    nat_eq(a6340, a6341)
                } {
                    2u64
                } else {
                    1u64
                }
            }
        }, {
            let a6342 = v0;
            let a6343 = 4u64;
            if nat_lt(a6342, a6343) {
                v0
            } else {
                let a6344 = v0;
                let a6345 = 4u64;
                if if nat_eq(a6344, a6345) {
                    true
                } else {
                    let a6346 = v0;
                    let a6347 = 16u64;
                    nat_eq(a6346, a6347)
                } {
                    0u64
                } else {
                    let a6348 = v0;
                    let a6349 = 19u64;
                    if nat_eq(a6348, a6349) {
                        3u64
                    } else {
                        let a6350 = 21u64;
                        let a6351 = v0;
                        if nat_le(a6350, a6351) {
                            let a6352 = v0;
                            let a6353 = 17u64;
                            nat_sub(a6352, a6353)
                        } else {
                            let a6354 = v0;
                            let a6355 = 8u64;
                            nat_sub(a6354, a6355)
                        }
                    }
                }
            }
        }, v1, v2, v3, v4, v5, v6)
    } else {
        Adt17::C0
    }
}

pub fn f289(v0: u64) -> R<u64> {
    let a6358 = 2u64;
    let a6359 = {
        let a6356 = 6u64;
        let a6357 = v0;
        nat_mul(a6356, a6357)?
    };
    nat_add(a6358, a6359)
}

pub fn f290(v0: u64) -> R<u64> {
    let a6360 = f289(v0)?;
    let a6361 = 11u64;
    nat_add(a6360, a6361)
}

pub fn f291(v0: u64, v1: u64, v2: Adt20, v3: Adt20) -> R<Adt17> {
    let v4: u64 = {
        let a6362 = v0;
        let a6363 = 6u64;
        let a6364 = 0u64;
        nat_rem(a6362, a6363, a6364)
    };
    let v5: u64 = {
        let a6371 = {
            let a6366 = v1;
            let a6367 = match v2.clone() {
                Adt20::C0(h6365, _, _, _, _, _, _, _) => {
                    h6365
                }
            };
            nat_mul(a6366, a6367)?
        };
        let a6372 = {
            let a6368 = v0;
            let a6369 = 6u64;
            let a6370 = 0u64;
            nat_quot(a6368, a6369, a6370)
        };
        nat_add(a6371, a6372)?
    };
    let v6: u64 = f287(v5, v3)?;
    let v7: u64 = f287(v5, v2)?;
    let a6373 = v4;
    let a6374 = 0u64;
    if nat_eq(a6373, a6374) {
        Ok(f288(10u64, 4u64, v6, 4u64, v6, 1u64, 2u64))
    } else {
        let a6375 = v4;
        let a6376 = 1u64;
        if nat_eq(a6375, a6376) {
            Ok(f288(1u64, 1u64, 2u64, 2u64, 0u64, 1u64, 2u64))
        } else {
            let a6377 = v4;
            let a6378 = 2u64;
            if nat_eq(a6377, a6378) {
                Ok(f288(0u64, 1u64, 0u64, 1u64, 2u64, 1u64, 0u64))
            } else {
                let a6379 = v4;
                let a6380 = 3u64;
                if nat_eq(a6379, a6380) {
                    Ok(f288(10u64, 4u64, v6, 0u64, v7, 1u64, 2u64))
                } else {
                    let a6381 = v4;
                    let a6382 = 4u64;
                    if nat_eq(a6381, a6382) {
                        Ok(f288(1u64, 1u64, 2u64, 2u64, 0u64, 1u64, 2u64))
                    } else {
                        Ok(f288(0u64, 1u64, 1u64, 1u64, 2u64, 1u64, 1u64))
                    }
                }
            }
        }
    }
}

pub fn f292(v0: u64, v1: u64, v2: u64) -> Adt17 {
    let a6383 = v0;
    let a6384 = 0u64;
    if nat_eq(a6383, a6384) {
        f288(2u64, 1u64, 0u64, 2u64, 0u64, 1u64, 3u64)
    } else {
        let a6385 = v0;
        let a6386 = 1u64;
        if nat_eq(a6385, a6386) {
            f288(3u64, 2u64, v1, 2u64, 0u64, 1u64, 2u64)
        } else {
            let a6387 = v0;
            let a6388 = 2u64;
            if nat_eq(a6387, a6388) {
                f288(11u64, 1u64, 3u64, 1u64, 2u64, 1u64, 4u64)
            } else {
                let a6389 = v0;
                let a6390 = 3u64;
                if nat_eq(a6389, a6390) {
                    f288(4u64, 1u64, 4u64, 2u64, v2, 1u64, 4u64)
                } else {
                    let a6391 = v0;
                    let a6392 = 4u64;
                    if nat_eq(a6391, a6392) {
                        f288(12u64, 1u64, 4u64, 2u64, 0u64, 1u64, 4u64)
                    } else {
                        let a6393 = v0;
                        let a6394 = 5u64;
                        if nat_eq(a6393, a6394) {
                            f288(11u64, 2u64, 1065353216u64, 1u64, 4u64, 1u64, 4u64)
                        } else {
                            let a6395 = v0;
                            let a6396 = 6u64;
                            if nat_eq(a6395, a6396) {
                                f288(10u64, 2u64, v2, 1u64, 2u64, 1u64, 5u64)
                            } else {
                                let a6397 = v0;
                                let a6398 = 7u64;
                                if nat_eq(a6397, a6398) {
                                    f288(4u64, 1u64, 3u64, 1u64, 5u64, 1u64, 5u64)
                                } else {
                                    let a6399 = v0;
                                    let a6400 = 8u64;
                                    if nat_eq(a6399, a6400) {
                                        f288(2u64, 1u64, 1u64, 2u64, 0u64, 1u64, 2u64)
                                    } else {
                                        let a6401 = v0;
                                        let a6402 = 9u64;
                                        if nat_eq(a6401, a6402) {
                                            Adt17::C1(3u64, 0u64, 1u64, 2u64, 2u64, 2u64, 1u64, 2u64)
                                        } else {
                                            let a6403 = v0;
                                            let a6404 = 10u64;
                                            if nat_eq(a6403, a6404) {
                                                f288(11u64, 1u64, 2u64, 1u64, 5u64, 1u64, 5u64)
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

pub fn f293(v0: u64, v1: u64, v2: Adt20, v3: Adt20, v4: Adt20) -> R<Adt17> {
    let v5: u64 = {
        let a6405 = v0;
        let a6406 = 3u64;
        let a6407 = 0u64;
        nat_rem(a6405, a6406, a6407)
    };
    let v6: u64 = {
        let a6414 = {
            let a6409 = v1;
            let a6410 = match v2.clone() {
                Adt20::C0(h6408, _, _, _, _, _, _, _) => {
                    h6408
                }
            };
            nat_mul(a6409, a6410)?
        };
        let a6415 = {
            let a6411 = v0;
            let a6412 = 3u64;
            let a6413 = 0u64;
            nat_quot(a6411, a6412, a6413)
        };
        nat_add(a6414, a6415)?
    };
    let v7: u64 = f287(v6, v3)?;
    let v8: u64 = f287(v6, v2)?;
    let v9: u64 = f287(v6, v4)?;
    let a6416 = v5;
    let a6417 = 0u64;
    if nat_eq(a6416, a6417) {
        Ok(f288(10u64, 4u64, v7, 1u64, 5u64, 1u64, 2u64))
    } else {
        let a6418 = v5;
        let a6419 = 1u64;
        if nat_eq(a6418, a6419) {
            Ok(f288(4u64, 0u64, v8, 1u64, 2u64, 1u64, 2u64))
        } else {
            Ok(f288(10u64, 1u64, 2u64, 1u64, 4u64, 3u64, v9))
        }
    }
}

pub fn f294(v0: u64, v1: u64, v2: Adt20, v3: bool) -> R<Adt17> {
    let v4: u64 = match v2.clone() {
        Adt20::C0(h6420, _, _, _, _, _, _, _) => {
            h6420
        }
    };
    let v5: u64 = {
        let a6423 = 1u64;
        let a6424 = {
            let a6421 = v4;
            let a6422 = f279(v3);
            nat_mul(a6421, a6422)?
        };
        nat_add(a6423, a6424)?
    };
    let a6425 = v0;
    let a6426 = 0u64;
    if nat_eq(a6425, a6426) {
        Ok(f286(0u64))
    } else {
        let a6427 = v0;
        let a6428 = v5;
        if nat_lt(a6427, a6428) {
            let v6: u64 = {
                let a6429 = v0;
                let a6430 = 1u64;
                nat_sub(a6429, a6430)
            };
            let v7: u64 = f287({
                let a6436 = {
                    let a6431 = v1;
                    let a6432 = v4;
                    nat_mul(a6431, a6432)?
                };
                let a6437 = {
                    let a6433 = v6;
                    let a6434 = f279(v3);
                    let a6435 = 0u64;
                    nat_quot(a6433, a6434, a6435)
                };
                nat_add(a6436, a6437)?
            }, v2)?;
            if v3 {
                let a6441 = {
                    let a6438 = v6;
                    let a6439 = 2u64;
                    let a6440 = 0u64;
                    nat_rem(a6438, a6439, a6440)
                };
                let a6442 = 0u64;
                if nat_eq(a6441, a6442) {
                    Ok(f288(1u64, 0u64, v7, 2u64, 0u64, 1u64, 1u64))
                } else {
                    Ok(f288(0u64, 1u64, 0u64, 1u64, 1u64, 1u64, 0u64))
                }
            } else {
                Ok(f288(4u64, 1u64, 0u64, 0u64, v7, 1u64, 0u64))
            }
        } else {
            let v8: u64 = {
                let a6443 = v0;
                let a6444 = v5;
                nat_sub(a6443, a6444)
            };
            let a6445 = v8;
            let a6446 = 0u64;
            if nat_eq(a6445, a6446) {
                Ok(f288(3u64, 2u64, v4, 2u64, 0u64, 1u64, 1u64))
            } else {
                if if v3 {
                    let a6447 = v8;
                    let a6448 = 1u64;
                    nat_eq(a6447, a6448)
                } else {
                    false
                } {
                    Ok(f288(2u64, 1u64, 0u64, 2u64, 0u64, 1u64, 0u64))
                } else {
                    let a6449 = v8;
                    let a6450 = if v3 {
                        2u64
                    } else {
                        1u64
                    };
                    if nat_eq(a6449, a6450) {
                        Ok(f288(11u64, 1u64, 0u64, 1u64, 1u64, 1u64, 3u64))
                    } else {
                        let a6451 = v8;
                        let a6452 = if v3 {
                            3u64
                        } else {
                            2u64
                        };
                        if nat_eq(a6451, a6452) {
                            Ok(f286(0u64))
                        } else {
                            Ok(Adt17::C0)
                        }
                    }
                }
            }
        }
    }
}

pub fn f295(v0: u64, v1: u64, v2: Adt20, v3: Adt20, v4: bool) -> R<Adt17> {
    let v5: u64 = if v4 {
        let a6454 = 13u64;
        let a6455 = f280(match v2.clone() {
            Adt20::C0(h6453, _, _, _, _, _, _, _) => {
                h6453
            }
        });
        nat_mul(a6454, a6455)?
    } else {
        0u64
    };
    let a6456 = v0;
    let a6457 = v5;
    if nat_lt(a6456, a6457) {
        f302({
            let a6458 = v0;
            let a6459 = 13u64;
            let a6460 = 0u64;
            nat_rem(a6458, a6459, a6460)
        }, v1, {
            let a6461 = v0;
            let a6462 = 13u64;
            let a6463 = 0u64;
            nat_quot(a6461, a6462, a6463)
        }, v2, v3)
    } else {
        let v6: u64 = {
            let a6464 = v0;
            let a6465 = v5;
            nat_sub(a6464, a6465)
        };
        let v7: u64 = if v4 {
            4u64
        } else {
            3u64
        };
        f303({
            let a6466 = v6;
            let a6467 = v7;
            let a6468 = 0u64;
            nat_rem(a6466, a6467, a6468)
        }, {
            let a6478 = {
                let a6473 = {
                    let a6470 = v1;
                    let a6471 = match v2.clone() {
                        Adt20::C0(h6469, _, _, _, _, _, _, _) => {
                            h6469
                        }
                    };
                    nat_mul(a6470, a6471)?
                };
                let a6474 = if v4 {
                    f304(match v2.clone() {
                        Adt20::C0(h6472, _, _, _, _, _, _, _) => {
                            h6472
                        }
                    })?
                } else {
                    0u64
                };
                nat_add(a6473, a6474)?
            };
            let a6479 = {
                let a6475 = v6;
                let a6476 = v7;
                let a6477 = 0u64;
                nat_quot(a6475, a6476, a6477)
            };
            nat_add(a6478, a6479)?
        }, v2, v3, v4)
    }
}

pub fn f296(v0: u64, v1: u64, v2: u64, v3: bool) -> Adt17 {
    let a6480 = v0;
    let a6481 = 0u64;
    if nat_eq(a6480, a6481) {
        f288(if v3 {
            22u64
        } else {
            3u64
        }, 2u64, v1, 2u64, 0u64, 1u64, 1u64)
    } else {
        let a6482 = v0;
        let a6483 = 1u64;
        if nat_eq(a6482, a6483) {
            f288(if v3 {
                21u64
            } else {
                11u64
            }, 1u64, 0u64, 1u64, 1u64, 1u64, 1u64)
        } else {
            if if v3 {
                let a6484 = v0;
                let a6485 = 2u64;
                nat_eq(a6484, a6485)
            } else {
                false
            } {
                f288(2u64, 1u64, 1u64, 2u64, 0u64, 1u64, 1u64)
            } else {
                let a6486 = v0;
                let a6487 = if v3 {
                    3u64
                } else {
                    2u64
                };
                if nat_eq(a6486, a6487) {
                    f288(4u64, 1u64, 1u64, 2u64, v2, 1u64, 1u64)
                } else {
                    let a6488 = v0;
                    let a6489 = if v3 {
                        4u64
                    } else {
                        3u64
                    };
                    if nat_eq(a6488, a6489) {
                        f288(12u64, 1u64, 1u64, 2u64, 0u64, 1u64, 1u64)
                    } else {
                        let a6490 = v0;
                        let a6491 = if v3 {
                            5u64
                        } else {
                            4u64
                        };
                        if nat_eq(a6490, a6491) {
                            f288(11u64, 2u64, 1065353216u64, 1u64, 1u64, 1u64, 1u64)
                        } else {
                            Adt17::C0
                        }
                    }
                }
            }
        }
    }
}

pub fn f297(v0: bool, v1: u64, v2: u64, v3: Adt21, v4: Adt20, v5: Adt20) -> R<Adt17> {
    let v6: u64 = if v0 {
        4u64
    } else {
        2u64
    };
    let v7: u64 = {
        let a6495 = 1u64;
        let a6496 = {
            let a6493 = match v4.clone() {
                Adt20::C0(h6492, _, _, _, _, _, _, _) => {
                    h6492
                }
            };
            let a6494 = v6;
            nat_mul(a6493, a6494)?
        };
        nat_add(a6495, a6496)?
    };
    let a6497 = v1;
    let a6498 = 0u64;
    if nat_eq(a6497, a6498) {
        Ok(f286(1u64))
    } else {
        let a6499 = v1;
        let a6500 = v7;
        if nat_lt(a6499, a6500) {
            let v8: u64 = {
                let a6501 = v1;
                let a6502 = 1u64;
                nat_sub(a6501, a6502)
            };
            let v9: u64 = {
                let a6509 = {
                    let a6504 = f300(v2, v3, v4.clone())?;
                    let a6505 = match v4.clone() {
                        Adt20::C0(h6503, _, _, _, _, _, _, _) => {
                            h6503
                        }
                    };
                    nat_mul(a6504, a6505)?
                };
                let a6510 = {
                    let a6506 = v8;
                    let a6507 = v6;
                    let a6508 = 0u64;
                    nat_quot(a6506, a6507, a6508)
                };
                nat_add(a6509, a6510)?
            };
            let v10: u64 = f287(v9, v4)?;
            let v11: u64 = f287(v9, v5)?;
            let v12: u64 = {
                let a6511 = v8;
                let a6512 = v6;
                let a6513 = 0u64;
                nat_rem(a6511, a6512, a6513)
            };
            if v0 {
                let a6514 = v12;
                let a6515 = 0u64;
                if nat_eq(a6514, a6515) {
                    Ok(f288(9u64, 0u64, v10, 1u64, 3u64, 3u64, v11))
                } else {
                    let a6516 = v12;
                    let a6517 = 1u64;
                    if nat_eq(a6516, a6517) {
                        Ok(f288(10u64, 3u64, v11, 3u64, v11, 1u64, 2u64))
                    } else {
                        let a6518 = v12;
                        let a6519 = 2u64;
                        if nat_eq(a6518, a6519) {
                            Ok(f288(1u64, 1u64, 2u64, 2u64, 0u64, 1u64, 2u64))
                        } else {
                            Ok(f288(0u64, 1u64, 1u64, 1u64, 2u64, 1u64, 1u64))
                        }
                    }
                }
            } else {
                let a6520 = v12;
                let a6521 = 0u64;
                if nat_eq(a6520, a6521) {
                    Ok(f288(1u64, 0u64, v10, 2u64, 0u64, 1u64, 2u64))
                } else {
                    Ok(f288(0u64, 1u64, 1u64, 1u64, 2u64, 1u64, 1u64))
                }
            }
        } else {
            let a6522 = v1;
            let a6523 = v7;
            if nat_eq(a6522, a6523) {
                Ok(f288(0u64, 1u64, 0u64, 1u64, 1u64, 1u64, 0u64))
            } else {
                Ok(Adt17::C0)
            }
        }
    }
}

pub fn f298(v0: u64, v1: u64) -> Adt17 {
    let a6524 = v0;
    let a6525 = 0u64;
    if nat_eq(a6524, a6525) {
        f288(22u64, 2u64, v1, 2u64, 0u64, 1u64, 2u64)
    } else {
        let a6526 = v0;
        let a6527 = 1u64;
        if nat_eq(a6526, a6527) {
            f288(21u64, 1u64, 0u64, 1u64, 2u64, 1u64, 3u64)
        } else {
            let a6528 = v0;
            let a6529 = 2u64;
            if nat_eq(a6528, a6529) {
                f288(2u64, 1u64, 3u64, 2u64, 0u64, 1u64, 3u64)
            } else {
                let a6530 = v0;
                let a6531 = 3u64;
                if nat_eq(a6530, a6531) {
                    f286(0u64)
                } else {
                    Adt17::C0
                }
            }
        }
    }
}

pub fn f299(v0: u64, v1: u64, v2: u64) -> Adt17 {
    let a6532 = v0;
    let a6533 = 0u64;
    if nat_eq(a6532, a6533) {
        f288(22u64, 2u64, v1, 2u64, 0u64, 1u64, 2u64)
    } else {
        let a6534 = v0;
        let a6535 = 1u64;
        if nat_eq(a6534, a6535) {
            f288(21u64, 1u64, 0u64, 1u64, 2u64, 1u64, 2u64)
        } else {
            let a6536 = v0;
            let a6537 = 2u64;
            if nat_eq(a6536, a6537) {
                f288(2u64, 1u64, 2u64, 2u64, 0u64, 1u64, 2u64)
            } else {
                let a6538 = v0;
                let a6539 = 3u64;
                if nat_eq(a6538, a6539) {
                    f288(4u64, 1u64, 2u64, 2u64, v2, 1u64, 1u64)
                } else {
                    let a6540 = v0;
                    let a6541 = 4u64;
                    if nat_eq(a6540, a6541) {
                        f288(12u64, 1u64, 1u64, 2u64, 0u64, 1u64, 1u64)
                    } else {
                        let a6542 = v0;
                        let a6543 = 5u64;
                        if nat_eq(a6542, a6543) {
                            f288(11u64, 2u64, 1065353216u64, 1u64, 1u64, 1u64, 1u64)
                        } else {
                            Adt17::C0
                        }
                    }
                }
            }
        }
    }
}

pub fn f300(v0: u64, v1: Adt21, v2: Adt20) -> R<u64> {
    let a6554 = {
        let a6552 = {
            let a6549 = {
                let a6546 = match v1.clone() {
                    Adt21::C0(_, _, h6544, _) => {
                        h6544
                    }
                };
                let a6547 = match v2.clone() {
                    Adt20::C0(_, _, h6545, _, _, _, _, _) => {
                        h6545
                    }
                };
                nat_mul(a6546, a6547)?
            };
            let a6550 = match v1 {
                Adt21::C0(h6548, _, _, _) => {
                    h6548
                }
            };
            nat_add(a6549, a6550)?
        };
        let a6553 = match v2 {
            Adt20::C0(_, h6551, _, _, _, _, _, _) => {
                h6551
            }
        };
        nat_mul(a6552, a6553)?
    };
    let a6555 = v0;
    nat_add(a6554, a6555)
}

pub fn f301(v0: u64) -> u64 {
    if f199(v0) {
        let m6556 = f192(v0);
        match m6556 {
            Adt18::C0 => {
                0u64
            }
            Adt18::C1(v1) => {
                let a6558 = 1u64;
                let a6559 = match v1.clone() {
                    Adt19::C0(_, h6557, _) => {
                        h6557
                    }
                };
                if if nat_lt(a6558, a6559) {
                    true
                } else {
                    let a6560 = v0;
                    let a6561 = 43u64;
                    nat_eq(a6560, a6561)
                } {
                    match v1 {
                        Adt19::C0(_, h6562, _) => {
                            h6562
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

pub fn f302(v0: u64, v1: u64, v2: u64, v3: Adt20, v4: Adt20) -> R<Adt17> {
    let v5: u64 = {
        let a6570 = {
            let a6568 = {
                let a6564 = v1;
                let a6565 = match v3.clone() {
                    Adt20::C0(h6563, _, _, _, _, _, _, _) => {
                        h6563
                    }
                };
                nat_mul(a6564, a6565)?
            };
            let a6569 = {
                let a6566 = v2;
                let a6567 = 4u64;
                nat_mul(a6566, a6567)?
            };
            nat_add(a6568, a6569)?
        };
        let a6571 = f305(v0);
        nat_add(a6570, a6571)?
    };
    let v6: u64 = f287(v5, v3)?;
    let v7: u64 = f287(v5, v4)?;
    let a6572 = v0;
    let a6573 = 0u64;
    if if if if nat_eq(a6572, a6573) {
        true
    } else {
        let a6574 = v0;
        let a6575 = 2u64;
        nat_eq(a6574, a6575)
    } {
        true
    } else {
        let a6576 = v0;
        let a6577 = 5u64;
        nat_eq(a6576, a6577)
    } {
        true
    } else {
        let a6578 = v0;
        let a6579 = 7u64;
        nat_eq(a6578, a6579)
    } {
        Ok(f288(9u64, 0u64, v6, 1u64, 3u64, 3u64, v7))
    } else {
        let a6580 = v0;
        let a6581 = 1u64;
        if if if if nat_eq(a6580, a6581) {
            true
        } else {
            let a6582 = v0;
            let a6583 = 3u64;
            nat_eq(a6582, a6583)
        } {
            true
        } else {
            let a6584 = v0;
            let a6585 = 6u64;
            nat_eq(a6584, a6585)
        } {
            true
        } else {
            let a6586 = v0;
            let a6587 = 8u64;
            nat_eq(a6586, a6587)
        } {
            Ok(f288(10u64, 3u64, v7, 3u64, v7, 1u64, {
                let a6588 = v0;
                let a6589 = 1u64;
                if nat_eq(a6588, a6589) {
                    4u64
                } else {
                    let a6590 = v0;
                    let a6591 = 6u64;
                    if nat_eq(a6590, a6591) {
                        5u64
                    } else {
                        1u64
                    }
                }
            }))
        } else {
            let a6592 = v0;
            let a6593 = 4u64;
            if nat_eq(a6592, a6593) {
                Ok(f288(4u64, 1u64, 4u64, 1u64, 1u64, 1u64, 4u64))
            } else {
                let a6594 = v0;
                let a6595 = 9u64;
                if nat_eq(a6594, a6595) {
                    Ok(f288(4u64, 1u64, 5u64, 1u64, 1u64, 1u64, 5u64))
                } else {
                    let a6596 = v0;
                    let a6597 = 10u64;
                    if nat_eq(a6596, a6597) {
                        Ok(f288(4u64, 1u64, 4u64, 1u64, 5u64, 1u64, 1u64))
                    } else {
                        let a6598 = v0;
                        let a6599 = 11u64;
                        if nat_eq(a6598, a6599) {
                            Ok(f288(1u64, 1u64, 1u64, 2u64, 0u64, 1u64, 1u64))
                        } else {
                            let a6600 = v0;
                            let a6601 = 12u64;
                            if nat_eq(a6600, a6601) {
                                Ok(f288(0u64, 1u64, 0u64, 1u64, 1u64, 1u64, 0u64))
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

pub fn f303(v0: u64, v1: u64, v2: Adt20, v3: Adt20, v4: bool) -> R<Adt17> {
    let v5: u64 = f287(v1, v2)?;
    let v6: u64 = f287(v1, v3)?;
    let a6602 = v0;
    let a6603 = 0u64;
    if nat_eq(a6602, a6603) {
        Ok(f288(9u64, 0u64, v5, 1u64, 3u64, 3u64, v6))
    } else {
        let a6604 = v0;
        let a6605 = 1u64;
        if nat_eq(a6604, a6605) {
            Ok(f288(10u64, 3u64, v6, 3u64, v6, 1u64, 1u64))
        } else {
            if v4 {
                let a6606 = v0;
                let a6607 = 2u64;
                if nat_eq(a6606, a6607) {
                    Ok(f288(1u64, 1u64, 1u64, 2u64, 0u64, 1u64, 1u64))
                } else {
                    let a6608 = v0;
                    let a6609 = 3u64;
                    if nat_eq(a6608, a6609) {
                        Ok(f288(0u64, 1u64, 0u64, 1u64, 1u64, 1u64, 0u64))
                    } else {
                        Ok(Adt17::C0)
                    }
                }
            } else {
                let a6610 = v0;
                let a6611 = 2u64;
                if nat_eq(a6610, a6611) {
                    Ok(f288(4u64, 1u64, 0u64, 1u64, 1u64, 1u64, 0u64))
                } else {
                    Ok(Adt17::C0)
                }
            }
        }
    }
}

pub fn f304(v0: u64) -> R<u64> {
    let a6612 = 4u64;
    let a6613 = f280(v0);
    nat_mul(a6612, a6613)
}

pub fn f305(v0: u64) -> u64 {
    let a6614 = v0;
    let a6615 = 5u64;
    if nat_lt(a6614, a6615) {
        let a6616 = v0;
        let a6617 = 2u64;
        if nat_lt(a6616, a6617) {
            0u64
        } else {
            1u64
        }
    } else {
        let a6618 = v0;
        let a6619 = 7u64;
        if nat_lt(a6618, a6619) {
            2u64
        } else {
            3u64
        }
    }
}

pub fn run(p0: Adt0) -> R<Adt9> {
    f0(p0)
}
