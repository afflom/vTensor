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
    let a463 = v0;
    let a464 = 28u64;
    if if if nat_eq(a463, a464) {
        true
    } else {
        let a465 = v0;
        let a466 = 29u64;
        nat_eq(a465, a466)
    } {
        let a468 = 0u64;
        let a469 = match v1.clone() {
            Adt1::C0(h467, _, _, _, _, _, _, _, _, _, _, _) => {
                h467
            }
        };
        if nat_lt(a468, a469) {
            true
        } else {
            let a471 = 0u64;
            let a472 = match v1.clone() {
                Adt1::C0(_, _, _, h470, _, _, _, _, _, _, _, _) => {
                    h470
                }
            };
            nat_lt(a471, a472)
        }
    } else {
        false
    } {
        let m476 = f41(match v2.clone() {
            Adt2::C0(h473, _, _, _, _, _, _, _, _, _, _) => {
                h473
            }
        }, match v1.clone() {
            Adt1::C0(h474, _, _, _, _, _, _, _, _, _, _, _) => {
                h474
            }
        }, match v1.clone() {
            Adt1::C0(_, _, _, h475, _, _, _, _, _, _, _, _) => {
                h475
            }
        });
        match m476 {
            None => {
                Ok(Adt9::C0(Adt10::C8))
            }
            Some(v5) => {
                if v5 {
                    Ok(Adt9::C0(Adt10::C7))
                } else {
                    f42(v0, v1, v2, v3, v4)
                }
            }
        }
    } else {
        f42(v0, v1, v2, v3, v4)
    }
}

pub fn f10(v0: Adt8, v1: u64, v2: u64, v3: u64) -> R<Adt9> {
    let v4: u64 = match v0.clone() {
        Adt8::C0(_, h477, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h477
        }
    };
    let v5: u64 = match v0.clone() {
        Adt8::C0(_, _, h478, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h478
        }
    };
    let v6: u64 = {
        let a479 = v3;
        let a480 = v5;
        let a481 = 0u64;
        nat_rem(a479, a480, a481)
    };
    let a482 = v1;
    let a483 = 0u64;
    if nat_eq(a482, a483) {
        let a485 = v3;
        let a486 = match v0.clone() {
            Adt8::C0(_, _, _, h484, _, _, _, _, _, _, _, _, _, _, _) => {
                h484
            }
        };
        if nat_lt(a485, a486) {
            Ok(Adt9::C8(Some(f43(match v0.clone() {
                Adt8::C0(_, _, _, _, _, _, _, _, h487, _, _, _, _, _, _) => {
                    h487
                }
            }, {
                let a491 = v6;
                let a492 = {
                    let m490 = match v0.clone() {
                        Adt8::C0(_, _, _, _, _, _, _, _, h489, _, _, _, _, _, _) => {
                            h489
                        }
                    };
                    match m490 {
                        Adt3::C0(h488, _, _, _, _, _, _, _) => {
                            h488
                        }
                    }
                };
                let a493 = 0u64;
                nat_rem(a491, a492, a493)
            }, {
                let a503 = {
                    let a497 = v6;
                    let a498 = {
                        let m496 = match v0.clone() {
                            Adt8::C0(_, _, _, _, _, _, _, _, h495, _, _, _, _, _, _) => {
                                h495
                            }
                        };
                        match m496 {
                            Adt3::C0(h494, _, _, _, _, _, _, _) => {
                                h494
                            }
                        }
                    };
                    let a499 = 0u64;
                    nat_quot(a497, a498, a499)
                };
                let a504 = {
                    let m502 = match v0.clone() {
                        Adt8::C0(_, _, _, _, _, _, _, _, h501, _, _, _, _, _, _) => {
                            h501
                        }
                    };
                    match m502 {
                        Adt3::C0(_, h500, _, _, _, _, _, _) => {
                            h500
                        }
                    }
                };
                let a505 = 0u64;
                nat_rem(a503, a504, a505)
            }, {
                let a515 = {
                    let a509 = v6;
                    let a510 = {
                        let m508 = match v0.clone() {
                            Adt8::C0(_, _, _, _, _, _, _, _, h507, _, _, _, _, _, _) => {
                                h507
                            }
                        };
                        match m508 {
                            Adt3::C0(h506, _, _, _, _, _, _, _) => {
                                h506
                            }
                        }
                    };
                    let a511 = 0u64;
                    nat_quot(a509, a510, a511)
                };
                let a516 = {
                    let m514 = match v0 {
                        Adt8::C0(_, _, _, _, _, _, _, _, h513, _, _, _, _, _, _) => {
                            h513
                        }
                    };
                    match m514 {
                        Adt3::C0(_, h512, _, _, _, _, _, _) => {
                            h512
                        }
                    }
                };
                let a517 = 0u64;
                nat_quot(a515, a516, a517)
            }, {
                let a518 = v3;
                let a519 = v5;
                let a520 = 0u64;
                nat_quot(a518, a519, a520)
            })?)))
        } else {
            Ok(Adt9::C0(Adt10::C1))
        }
    } else {
        let v7: u64 = {
            let a526 = v2;
            let a527 = {
                let a523 = v3;
                let a524 = {
                    let a521 = v1;
                    let a522 = 1u64;
                    if nat_eq(a521, a522) {
                        v4
                    } else {
                        v5
                    }
                };
                let a525 = 0u64;
                nat_quot(a523, a524, a525)
            };
            nat_add(a526, a527)?
        };
        let a529 = v7;
        let a530 = match v0.clone() {
            Adt8::C0(h528, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h528
            }
        };
        if nat_lt(a529, a530) {
            let v8: u64 = {
                let a534 = v7;
                let a535 = {
                    let m533 = match v0.clone() {
                        Adt8::C0(_, _, _, _, _, _, _, _, _, h532, _, _, _, _, _) => {
                            h532
                        }
                    };
                    match m533 {
                        Adt3::C0(h531, _, _, _, _, _, _, _) => {
                            h531
                        }
                    }
                };
                let a536 = 0u64;
                nat_rem(a534, a535, a536)
            };
            let v9: u64 = {
                let a540 = v7;
                let a541 = {
                    let m539 = match v0.clone() {
                        Adt8::C0(_, _, _, _, _, _, _, _, _, h538, _, _, _, _, _) => {
                            h538
                        }
                    };
                    match m539 {
                        Adt3::C0(h537, _, _, _, _, _, _, _) => {
                            h537
                        }
                    }
                };
                let a542 = 0u64;
                nat_quot(a540, a541, a542)
            };
            let v10: u64 = {
                let a546 = v9;
                let a547 = {
                    let m545 = match v0.clone() {
                        Adt8::C0(_, _, _, _, _, _, _, _, _, h544, _, _, _, _, _) => {
                            h544
                        }
                    };
                    match m545 {
                        Adt3::C0(_, h543, _, _, _, _, _, _) => {
                            h543
                        }
                    }
                };
                let a548 = 0u64;
                nat_rem(a546, a547, a548)
            };
            let v11: u64 = {
                let a552 = v9;
                let a553 = {
                    let m551 = match v0.clone() {
                        Adt8::C0(_, _, _, _, _, _, _, _, _, h550, _, _, _, _, _) => {
                            h550
                        }
                    };
                    match m551 {
                        Adt3::C0(_, h549, _, _, _, _, _, _) => {
                            h549
                        }
                    }
                };
                let a554 = 0u64;
                nat_quot(a552, a553, a554)
            };
            let a555 = v1;
            let a556 = 1u64;
            if nat_eq(a555, a556) {
                Ok(Adt9::C8(Some(f43(match v0 {
                    Adt8::C0(_, _, _, _, _, _, _, _, _, h557, _, _, _, _, _) => {
                        h557
                    }
                }, v8, v10, {
                    let a558 = v3;
                    let a559 = v4;
                    let a560 = 0u64;
                    nat_rem(a558, a559, a560)
                }, v11)?)))
            } else {
                let a561 = v1;
                let a562 = 2u64;
                if nat_eq(a561, a562) {
                    Ok(Adt9::C8(Some({
                        let a608 = f43(match v0.clone() {
                            Adt8::C0(_, _, _, _, _, _, _, _, _, _, h563, _, _, _, _) => {
                                h563
                            }
                        }, {
                            let a575 = {
                                let a567 = v8;
                                let a568 = {
                                    let m566 = match v0.clone() {
                                        Adt8::C0(_, _, _, _, _, _, _, _, _, _, _, h565, _, _, _) => {
                                            h565
                                        }
                                    };
                                    match m566 {
                                        Adt6::C0(_, h564, _, _) => {
                                            h564
                                        }
                                    }
                                };
                                nat_mul(a567, a568)?
                            };
                            let a576 = {
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
                            };
                            nat_add(a575, a576)?
                        }, {
                            let a594 = {
                                let a580 = v10;
                                let a581 = {
                                    let m579 = match v0.clone() {
                                        Adt8::C0(_, _, _, _, _, _, _, _, _, _, _, h578, _, _, _) => {
                                            h578
                                        }
                                    };
                                    match m579 {
                                        Adt6::C0(_, h577, _, _) => {
                                            h577
                                        }
                                    }
                                };
                                nat_mul(a580, a581)?
                            };
                            let a595 = {
                                let a591 = {
                                    let a585 = v6;
                                    let a586 = {
                                        let m584 = match v0.clone() {
                                            Adt8::C0(_, _, _, _, _, _, _, _, h583, _, _, _, _, _, _) => {
                                                h583
                                            }
                                        };
                                        match m584 {
                                            Adt3::C0(h582, _, _, _, _, _, _, _) => {
                                                h582
                                            }
                                        }
                                    };
                                    let a587 = 0u64;
                                    nat_quot(a585, a586, a587)
                                };
                                let a592 = {
                                    let m590 = match v0.clone() {
                                        Adt8::C0(_, _, _, _, _, _, _, _, h589, _, _, _, _, _, _) => {
                                            h589
                                        }
                                    };
                                    match m590 {
                                        Adt3::C0(_, h588, _, _, _, _, _, _) => {
                                            h588
                                        }
                                    }
                                };
                                let a593 = 0u64;
                                nat_rem(a591, a592, a593)
                            };
                            nat_add(a594, a595)?
                        }, {
                            let a605 = {
                                let a599 = v6;
                                let a600 = {
                                    let m598 = match v0.clone() {
                                        Adt8::C0(_, _, _, _, _, _, _, _, h597, _, _, _, _, _, _) => {
                                            h597
                                        }
                                    };
                                    match m598 {
                                        Adt3::C0(h596, _, _, _, _, _, _, _) => {
                                            h596
                                        }
                                    }
                                };
                                let a601 = 0u64;
                                nat_quot(a599, a600, a601)
                            };
                            let a606 = {
                                let m604 = match v0 {
                                    Adt8::C0(_, _, _, _, _, _, _, _, h603, _, _, _, _, _, _) => {
                                        h603
                                    }
                                };
                                match m604 {
                                    Adt3::C0(_, h602, _, _, _, _, _, _) => {
                                        h602
                                    }
                                }
                            };
                            let a607 = 0u64;
                            nat_quot(a605, a606, a607)
                        }, v11)?;
                        let a609 = 4u64;
                        let a610 = 0u64;
                        nat_quot(a608, a609, a610)
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
    let a611 = v1;
    let a612 = 0u64;
    if nat_eq(a611, a612) {
        let a614 = v3;
        let a615 = match v0.clone() {
            Adt8::C0(_, _, _, h613, _, _, _, _, _, _, _, _, _, _, _) => {
                h613
            }
        };
        if nat_lt(a614, a615) {
            let v4: Adt3 = match v0.clone() {
                Adt8::C0(_, _, _, _, _, _, _, _, h616, _, _, _, _, _, _) => {
                    h616
                }
            };
            let v5: u64 = {
                let a618 = v3;
                let a619 = match v0.clone() {
                    Adt8::C0(_, _, h617, _, _, _, _, _, _, _, _, _, _, _, _) => {
                        h617
                    }
                };
                let a620 = 0u64;
                nat_quot(a618, a619, a620)
            };
            Ok(Adt9::C8(Some(f43(v4.clone(), f44(v5, v4.clone(), 0u64), f44(v5, v4.clone(), 1u64), f44(v5, v4, 2u64), f45(v0.clone(), v5, {
                let a622 = v3;
                let a623 = match v0 {
                    Adt8::C0(_, _, h621, _, _, _, _, _, _, _, _, _, _, _, _) => {
                        h621
                    }
                };
                let a624 = 0u64;
                nat_rem(a622, a623, a624)
            })?)?)))
        } else {
            Ok(Adt9::C0(Adt10::C1))
        }
    } else {
        let v6: Adt3 = match v0.clone() {
            Adt8::C0(_, _, _, _, _, _, _, _, _, _, h625, _, _, _, _) => {
                h625
            }
        };
        let v7: u64 = {
            let a626 = v1;
            let a627 = 1u64;
            if nat_eq(a626, a627) {
                match v0.clone() {
                    Adt8::C0(_, h628, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                        h628
                    }
                }
            } else {
                match v0.clone() {
                    Adt8::C0(_, _, h629, _, _, _, _, _, _, _, _, _, _, _, _) => {
                        h629
                    }
                }
            }
        };
        let v8: u64 = {
            let a633 = v2;
            let a634 = {
                let a630 = v3;
                let a631 = v7;
                let a632 = 0u64;
                nat_quot(a630, a631, a632)
            };
            nat_add(a633, a634)?
        };
        let v9: u64 = {
            let a635 = v3;
            let a636 = v7;
            let a637 = 0u64;
            nat_rem(a635, a636, a637)
        };
        let v10: u64 = {
            let a639 = v8;
            let a640 = match v6.clone() {
                Adt3::C0(h638, _, _, _, _, _, _, _) => {
                    h638
                }
            };
            let a641 = 0u64;
            nat_rem(a639, a640, a641)
        };
        let v11: u64 = {
            let a643 = v8;
            let a644 = match v6.clone() {
                Adt3::C0(h642, _, _, _, _, _, _, _) => {
                    h642
                }
            };
            let a645 = 0u64;
            nat_quot(a643, a644, a645)
        };
        let v12: u64 = {
            let a647 = v11;
            let a648 = match v6.clone() {
                Adt3::C0(_, h646, _, _, _, _, _, _) => {
                    h646
                }
            };
            let a649 = 0u64;
            nat_rem(a647, a648, a649)
        };
        let v13: u64 = {
            let a651 = v11;
            let a652 = match v6.clone() {
                Adt3::C0(_, h650, _, _, _, _, _, _) => {
                    h650
                }
            };
            let a653 = 0u64;
            nat_quot(a651, a652, a653)
        };
        let a655 = v8;
        let a656 = match v0.clone() {
            Adt8::C0(h654, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h654
            }
        };
        if nat_lt(a655, a656) {
            let a657 = v1;
            let a658 = 1u64;
            if nat_eq(a657, a658) {
                let v14: Adt3 = match v0.clone() {
                    Adt8::C0(_, _, _, _, _, _, _, _, h659, _, _, _, _, _, _) => {
                        h659
                    }
                };
                let v15: Adt3 = match v0.clone() {
                    Adt8::C0(_, _, _, _, _, _, _, _, _, h660, _, _, _, _, _) => {
                        h660
                    }
                };
                let m663 = f46(match v0.clone() {
                    Adt8::C0(_, _, _, _, _, _, _, _, _, _, _, h661, _, _, _) => {
                        h661
                    }
                }, match v15.clone() {
                    Adt3::C0(h662, _, _, _, _, _, _, _) => {
                        h662
                    }
                }, v10, f44(v9, v14.clone(), 0u64))?;
                match m663 {
                    None => {
                        Ok(Adt9::C8(None::<u64>))
                    }
                    Some(v16) => {
                        let m666 = f46(match v0.clone() {
                            Adt8::C0(_, _, _, _, _, _, _, _, _, _, _, _, h664, _, _) => {
                                h664
                            }
                        }, match v15 {
                            Adt3::C0(_, h665, _, _, _, _, _, _) => {
                                h665
                            }
                        }, v12, f44(v9, v14, 1u64))?;
                        match m666 {
                            None => {
                                Ok(Adt9::C8(None::<u64>))
                            }
                            Some(v17) => {
                                f47(v0, v9, v13, v16, v17)
                            }
                        }
                    }
                }
            } else {
                let a667 = v1;
                let a668 = 2u64;
                if nat_eq(a667, a668) {
                    Ok(Adt9::C8(Some({
                        let a670 = match v0.clone() {
                            Adt8::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, h669) => {
                                h669
                            }
                        };
                        let a671 = 0u64;
                        if nat_eq(a670, a671) {
                            f43(v6, v10, v12, v9, v13)?
                        } else {
                            f43(v6.clone(), v10, v12, {
                                let a673 = v13;
                                let a674 = match v6.clone() {
                                    Adt3::C0(_, _, h672, _, _, _, _, _) => {
                                        h672
                                    }
                                };
                                let a675 = 0u64;
                                nat_rem(a673, a674, a675)
                            }, {
                                let a683 = {
                                    let a681 = {
                                        let a677 = v13;
                                        let a678 = match v6 {
                                            Adt3::C0(_, _, h676, _, _, _, _, _) => {
                                                h676
                                            }
                                        };
                                        let a679 = 0u64;
                                        nat_quot(a677, a678, a679)
                                    };
                                    let a682 = match v0 {
                                        Adt8::C0(_, _, h680, _, _, _, _, _, _, _, _, _, _, _, _) => {
                                            h680
                                        }
                                    };
                                    nat_mul(a681, a682)?
                                };
                                let a684 = v9;
                                nat_add(a683, a684)?
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
    Ok(f48(v3.clone(), {
        let m685 = v3.clone();
        match m685 {
            Adt4::C0 => {
                f49(v0, v1, v2, v3)?
            }
            Adt4::C1 => {
                f49(v0, v1, v2, v3)?
            }
            Adt4::C2(_, _) => {
                f49(v0, v1, v2, v3)?
            }
            Adt4::C3(_, _, _, _, _) => {
                f49(v0, v1, v2, v3)?
            }
            Adt4::C4(_) => {
                f49(v0, v1, v2, v3)?
            }
            Adt4::C5(_, _, _, _) => {
                f49(v0, v1, v2, v3)?
            }
            Adt4::C6(_, _, _) => {
                f49(v0, v1, v2, v3)?
            }
            Adt4::C7(_) => {
                f49(v0, v1, v2, v3)?
            }
            Adt4::C8(_) => {
                f49(v0, v1, v2, v3)?
            }
            Adt4::C9(_, _) => {
                f49(v0, v1, v2, v3)?
            }
            Adt4::C10(_, _, _) => {
                f49(v0, v1, v2, v3)?
            }
            Adt4::C11(_, _) => {
                f49(v0, v1, v2, v3)?
            }
            Adt4::C12(_, _, _) => {
                f49(v0, v1, v2, v3)?
            }
            Adt4::C13(_, _, _, _) => {
                f49(v0, v1, v2, v3)?
            }
            Adt4::C14(_, _, _) => {
                f49(v0, v1, v2, v3)?
            }
            Adt4::C15 => {
                f49(v0, v1, v2, v3)?
            }
            Adt4::C16(_, _) => {
                f49(v0, v1, v2, v3)?
            }
            Adt4::C17 => {
                f56(v0, v1, v2, v4)?
            }
            Adt4::C18(_, _) => {
                f49(v0, v1, v2, v3)?
            }
            Adt4::C19(v61, v62) => {
                f57(v61, v0, v1, v2, v62, v4)?
            }
            Adt4::C20(v63, v64) => {
                f58(v63, v0, v1, v2, v64, v4)?
            }
            Adt4::C21 => {
                f49(v0, v1, v2, v3)?
            }
            Adt4::C22(v65, v66) => {
                f59(v0, v1, v2, v65, v66, v4)?
            }
            Adt4::C23(v80, v81, v82, v83) => {
                f66(v0, v83, v80, v81, v82, v4)?
            }
            Adt4::C24(v79) => {
                if v5 {
                    f64(Adt16::C0(v79), v0, v1, v2, v4, v6, v7, v8, v9, v10)?
                } else {
                    Adt9::C0(Adt10::C5(65u64))
                }
            }
            Adt4::C25(_, _, _) => {
                f49(v0, v1, v2, v3)?
            }
            Adt4::C26(v75, v76, v77, v78) => {
                let a687 = v75;
                let a688 = 64u64;
                if if if nat_eq(a687, a688) {
                    true
                } else {
                    let a689 = v75;
                    let a690 = 65u64;
                    nat_eq(a689, a690)
                } {
                    true
                } else {
                    let a691 = v75;
                    let a692 = 66u64;
                    nat_eq(a691, a692)
                } {
                    if v5 {
                        f64(Adt16::C1(v75, v76, v77, v78), v0, v1, v2, v4, v6, v7, v8, v9, v10)?
                    } else {
                        Adt9::C0(Adt10::C5(64u64))
                    }
                } else {
                    let a693 = v75;
                    let a694 = 0u64;
                    if if nat_eq(a693, a694) {
                        true
                    } else {
                        let a695 = v75;
                        let a696 = 30u64;
                        nat_eq(a695, a696)
                    } {
                        f65(v0, v1, v2, v75, v76, v77, v78, v4)?
                    } else {
                        Adt9::C0(Adt10::C1)
                    }
                }
            }
            Adt4::C27(v74) => {
                f63(v0, v1, v2, v74)?
            }
            Adt4::C28(v70) => {
                f60(v0, v1, v2, v70, v4)?
            }
            Adt4::C29(v71) => {
                f61(v0, v1, v2, v71)?
            }
            Adt4::C30(v72, _) => {
                f62(v0, v1, v2, v72)?
            }
            Adt4::C31(v33, v34) => {
                f50(v0, v1, v2, v33, v34)?
            }
            Adt4::C32(v35) => {
                f51(v0, match v1 {
                    Adt3::C0(h686, _, _, _, _, _, _, _) => {
                        h686
                    }
                }, v35)?
            }
            Adt4::C33(v36) => {
                f52(v0, v1, v36, v2)?
            }
            Adt4::C34(v41, v42, v43, v44) => {
                f55(v0, v1, v41, v2, v42, v43, v44)?
            }
            Adt4::C35(v40) => {
                f54(v0, v40, v1, v2)?
            }
            Adt4::C36(v37, v38, v39) => {
                f53(v0, v1, v37, v2, v38, v39)?
            }
        }
    }))
}

pub fn f14(v0: Adt4) -> R<Adt9> {
    let m697 = v0;
    match m697 {
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
                let a702 = {
                    let a698 = 0u64;
                    let a699 = v57;
                    if nat_lt(a698, a699) {
                        1u64
                    } else {
                        0u64
                    }
                };
                let a703 = {
                    let a700 = 0u64;
                    let a701 = v59;
                    if nat_lt(a700, a701) {
                        6u64
                    } else {
                        0u64
                    }
                };
                nat_add(a702, a703)?
            }, 0u64, 0u64, false))
        }
        Adt4::C26(v65, _, _, _) => {
            Ok(Adt9::C6({
                let a704 = v65;
                let a705 = 66u64;
                if nat_eq(a704, a705) {
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
    let a706 = v1;
    let a707 = 5u64;
    if if if if if nat_lt(a706, a707) {
        true
    } else {
        f67(v0, v1, v2, v3, v4)
    } {
        let a708 = v5;
        let a709 = 5u64;
        nat_lt(a708, a709)
    } else {
        false
    } {
        let a710 = v7;
        let a711 = 5u64;
        nat_lt(a710, a711)
    } else {
        false
    } {
        let a712 = v9;
        let a713 = 2u64;
        if nat_lt(a712, a713) {
            true
        } else {
            let a714 = v9;
            let a715 = 3u64;
            nat_eq(a714, a715)
        }
    } else {
        false
    } {
        Adt9::C7(f68(v0, v1, v2, v3, v4), f69(v5, v6), f69(v7, v8), f70({
            let a716 = v9;
            let a717 = 0u64;
            if nat_eq(a716, a717) {
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
    let a729 = {
        let a723 = {
            let a719 = match v0.clone() {
                Adt2::C0(_, h718, _, _, _, _, _, _, _, _, _) => {
                    h718
                }
            };
            let a720 = v1;
            let a721 = 0u64;
            nat_quot(a719, a720, a721)
        };
        let a724 = match v0.clone() {
            Adt2::C0(_, _, h722, _, _, _, _, _, _, _, _) => {
                h722
            }
        };
        nat_mul(a723, a724)?
    };
    let a730 = {
        let a727 = match v0.clone() {
            Adt2::C0(_, _, _, h725, _, _, _, _, _, _, _) => {
                h725
            }
        };
        let a728 = match v0 {
            Adt2::C0(_, _, _, _, h726, _, _, _, _, _, _) => {
                h726
            }
        };
        nat_mul(a727, a728)?
    };
    nat_mul(a729, a730)
}

pub fn f17(v0: u64) -> bool {
    let a731 = v0;
    let a732 = 4294967296u64;
    if nat_lt(a731, a732) {
        let a736 = {
            let a733 = v0;
            let a734 = 2147483648u64;
            let a735 = 0u64;
            nat_rem(a733, a734, a735)
        };
        let a737 = 2139095040u64;
        nat_lt(a736, a737)
    } else {
        false
    }
}

pub fn f18(v0: u64) -> R<u64> {
    let a738 = 4u64;
    let a739 = v0;
    nat_mul(a738, a739)
}

pub fn f19(v0: u64) -> R<u64> {
    let a740 = v0;
    let a741 = 0u64;
    if nat_eq(a740, a741) {
        Ok(0u64)
    } else {
        let a744 = 4u64;
        let a745 = {
            let a742 = 3u64;
            let a743 = v0;
            nat_mul(a742, a743)?
        };
        nat_add(a744, a745)
    }
}

pub fn f20(v0: Adt2, v1: u64) -> Adt3 {
    Adt3::C0({
        let a747 = match v0.clone() {
            Adt2::C0(_, h746, _, _, _, _, _, _, _, _, _) => {
                h746
            }
        };
        let a748 = v1;
        let a749 = 0u64;
        nat_quot(a747, a748, a749)
    }, match v0.clone() {
        Adt2::C0(_, _, h750, _, _, _, _, _, _, _, _) => {
            h750
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, h751, _, _, _, _, _, _, _) => {
            h751
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, h752, _, _, _, _, _, _) => {
            h752
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, _, h753, _, _, _, _, _) => {
            h753
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, _, _, h754, _, _, _, _) => {
            h754
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, _, _, _, h755, _, _, _) => {
            h755
        }
    }, match v0 {
        Adt2::C0(_, _, _, _, _, _, _, _, h756, _, _) => {
            h756
        }
    })
}

pub fn f21(v0: bool, v1: Adt2) -> Adt3 {
    if v0 {
        f20(v1, 1u64)
    } else {
        Adt3::C0(match v1.clone() {
            Adt2::C0(_, h757, _, _, _, _, _, _, _, _, _) => {
                h757
            }
        }, 1u64, match v1.clone() {
            Adt2::C0(_, _, h758, _, _, _, _, _, _, _, _) => {
                h758
            }
        }, match v1.clone() {
            Adt2::C0(_, _, _, h759, _, _, _, _, _, _, _) => {
                h759
            }
        }, match v1.clone() {
            Adt2::C0(_, _, _, _, _, h760, _, _, _, _, _) => {
                h760
            }
        }, 0u64, match v1.clone() {
            Adt2::C0(_, _, _, _, _, _, h761, _, _, _, _) => {
                h761
            }
        }, match v1 {
            Adt2::C0(_, _, _, _, _, _, _, h762, _, _, _) => {
                h762
            }
        })
    }
}

pub fn f22(v0: Adt2) -> R<bool> {
    let a764 = match v0.clone() {
        Adt2::C0(_, h763, _, _, _, _, _, _, _, _, _) => {
            h763
        }
    };
    let a765 = 1u64;
    if if if if nat_eq(a764, a765) {
        true
    } else {
        let a767 = match v0.clone() {
            Adt2::C0(_, _, _, _, _, h766, _, _, _, _, _) => {
                h766
            }
        };
        let a768 = 4u64;
        nat_eq(a767, a768)
    } {
        let a770 = match v0.clone() {
            Adt2::C0(_, _, h769, _, _, _, _, _, _, _, _) => {
                h769
            }
        };
        let a771 = 1u64;
        if nat_eq(a770, a771) {
            true
        } else {
            let a776 = match v0.clone() {
                Adt2::C0(_, _, _, _, _, _, h772, _, _, _, _) => {
                    h772
                }
            };
            let a777 = {
                let a774 = match v0.clone() {
                    Adt2::C0(_, h773, _, _, _, _, _, _, _, _, _) => {
                        h773
                    }
                };
                let a775 = 4u64;
                nat_mul(a774, a775)?
            };
            nat_eq(a776, a777)
        }
    } else {
        false
    } {
        let a779 = match v0.clone() {
            Adt2::C0(_, _, _, h778, _, _, _, _, _, _, _) => {
                h778
            }
        };
        let a780 = 1u64;
        if nat_eq(a779, a780) {
            true
        } else {
            let a788 = match v0.clone() {
                Adt2::C0(_, _, _, _, _, _, _, h781, _, _, _) => {
                    h781
                }
            };
            let a789 = {
                let a786 = {
                    let a784 = match v0.clone() {
                        Adt2::C0(_, h782, _, _, _, _, _, _, _, _, _) => {
                            h782
                        }
                    };
                    let a785 = match v0.clone() {
                        Adt2::C0(_, _, h783, _, _, _, _, _, _, _, _) => {
                            h783
                        }
                    };
                    nat_mul(a784, a785)?
                };
                let a787 = 4u64;
                nat_mul(a786, a787)?
            };
            nat_eq(a788, a789)
        }
    } else {
        false
    } {
        let a791 = match v0.clone() {
            Adt2::C0(_, _, _, _, h790, _, _, _, _, _, _) => {
                h790
            }
        };
        let a792 = 1u64;
        if nat_eq(a791, a792) {
            Ok(true)
        } else {
            let a803 = match v0.clone() {
                Adt2::C0(_, _, _, _, _, _, _, _, h793, _, _) => {
                    h793
                }
            };
            let a804 = {
                let a801 = {
                    let a799 = {
                        let a796 = match v0.clone() {
                            Adt2::C0(_, h794, _, _, _, _, _, _, _, _, _) => {
                                h794
                            }
                        };
                        let a797 = match v0.clone() {
                            Adt2::C0(_, _, h795, _, _, _, _, _, _, _, _) => {
                                h795
                            }
                        };
                        nat_mul(a796, a797)?
                    };
                    let a800 = match v0 {
                        Adt2::C0(_, _, _, h798, _, _, _, _, _, _, _) => {
                            h798
                        }
                    };
                    nat_mul(a799, a800)?
                };
                let a802 = 4u64;
                nat_mul(a801, a802)?
            };
            Ok(nat_eq(a803, a804))
        }
    } else {
        Ok(false)
    }
}

pub fn f23(v0: Adt2, v1: Adt2) -> R<bool> {
    let a806 = match v0.clone() {
        Adt2::C0(_, _, _, _, _, _, _, _, _, h805, _) => {
            h805
        }
    };
    let a807 = 0u64;
    if nat_eq(a806, a807) {
        Ok(true)
    } else {
        let a809 = match v1.clone() {
            Adt2::C0(_, _, _, _, _, _, _, _, _, h808, _) => {
                h808
            }
        };
        let a810 = 0u64;
        if nat_eq(a809, a810) {
            Ok(true)
        } else {
            let a812 = match v0.clone() {
                Adt2::C0(_, _, _, _, _, _, _, _, _, _, h811) => {
                    h811
                }
            };
            let a813 = 0u64;
            if nat_eq(a812, a813) {
                Ok(true)
            } else {
                let a815 = match v1.clone() {
                    Adt2::C0(_, _, _, _, _, _, _, _, _, _, h814) => {
                        h814
                    }
                };
                let a816 = 0u64;
                if nat_eq(a815, a816) {
                    Ok(true)
                } else {
                    let a822 = {
                        let a819 = match v0.clone() {
                            Adt2::C0(_, _, _, _, _, _, _, _, _, _, h817) => {
                                h817
                            }
                        };
                        let a820 = match v0.clone() {
                            Adt2::C0(_, _, _, _, _, _, _, _, _, h818, _) => {
                                h818
                            }
                        };
                        nat_add(a819, a820)?
                    };
                    let a823 = match v1.clone() {
                        Adt2::C0(_, _, _, _, _, _, _, _, _, _, h821) => {
                            h821
                        }
                    };
                    if nat_le(a822, a823) {
                        Ok(true)
                    } else {
                        let a829 = {
                            let a826 = match v1.clone() {
                                Adt2::C0(_, _, _, _, _, _, _, _, _, _, h824) => {
                                    h824
                                }
                            };
                            let a827 = match v1 {
                                Adt2::C0(_, _, _, _, _, _, _, _, _, h825, _) => {
                                    h825
                                }
                            };
                            nat_add(a826, a827)?
                        };
                        let a830 = match v0 {
                            Adt2::C0(_, _, _, _, _, _, _, _, _, _, h828) => {
                                h828
                            }
                        };
                        Ok(nat_le(a829, a830))
                    }
                }
            }
        }
    }
}

pub fn f24(v0: Adt6, v1: u64) -> R<u64> {
    let a832 = match v0.clone() {
        Adt6::C0(h831, _, _, _) => {
            h831
        }
    };
    let a833 = 0u64;
    if nat_eq(a832, a833) {
        f71(Adt6::C0(1u64, match v0.clone() {
            Adt6::C0(_, h834, _, _) => {
                h834
            }
        }, match v0.clone() {
            Adt6::C0(_, _, h835, _) => {
                h835
            }
        }, match v0.clone() {
            Adt6::C0(_, _, _, h836) => {
                h836
            }
        }), {
            let a838 = v1;
            let a839 = match v0 {
                Adt6::C0(_, _, _, h837) => {
                    h837
                }
            };
            nat_add(a838, a839)?
        })
    } else {
        f71(v0, v1)
    }
}

pub fn f25(v0: Adt2, v1: Adt2) -> bool {
    let a842 = match v0.clone() {
        Adt2::C0(_, h840, _, _, _, _, _, _, _, _, _) => {
            h840
        }
    };
    let a843 = match v1.clone() {
        Adt2::C0(_, h841, _, _, _, _, _, _, _, _, _) => {
            h841
        }
    };
    if if if nat_eq(a842, a843) {
        let a846 = match v0.clone() {
            Adt2::C0(_, _, h844, _, _, _, _, _, _, _, _) => {
                h844
            }
        };
        let a847 = match v1.clone() {
            Adt2::C0(_, _, h845, _, _, _, _, _, _, _, _) => {
                h845
            }
        };
        nat_eq(a846, a847)
    } else {
        false
    } {
        let a850 = match v0.clone() {
            Adt2::C0(_, _, _, h848, _, _, _, _, _, _, _) => {
                h848
            }
        };
        let a851 = match v1.clone() {
            Adt2::C0(_, _, _, h849, _, _, _, _, _, _, _) => {
                h849
            }
        };
        nat_eq(a850, a851)
    } else {
        false
    } {
        let a854 = match v0 {
            Adt2::C0(_, _, _, _, h852, _, _, _, _, _, _) => {
                h852
            }
        };
        let a855 = match v1 {
            Adt2::C0(_, _, _, _, h853, _, _, _, _, _, _) => {
                h853
            }
        };
        nat_eq(a854, a855)
    } else {
        false
    }
}

pub fn f26(v0: u64, v1: u64, v2: u64, v3: u64) -> R<u64> {
    let a860 = {
        let a856 = v0;
        let a857 = v1;
        nat_mul(a856, a857)?
    };
    let a861 = {
        let a858 = v2;
        let a859 = f72(v3);
        nat_mul(a858, a859)?
    };
    nat_mul(a860, a861)
}

pub fn f27(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let a864 = v0;
    let a865 = {
        let a862 = f73(v1)?;
        let a863 = v2;
        nat_mul(a862, a863)?
    };
    nat_add(a864, a865)
}

pub fn f28(v0: Adt2) -> Adt14 {
    Adt14::C0(match v0.clone() {
        Adt2::C0(h866, _, _, _, _, _, _, _, _, _, _) => {
            h866
        }
    }, match v0.clone() {
        Adt2::C0(_, h867, _, _, _, _, _, _, _, _, _) => {
            h867
        }
    }, match v0.clone() {
        Adt2::C0(_, _, h868, _, _, _, _, _, _, _, _) => {
            h868
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, h869, _, _, _, _, _, _, _) => {
            h869
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, h870, _, _, _, _, _, _) => {
            h870
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, _, h871, _, _, _, _, _) => {
            h871
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, _, _, h872, _, _, _, _) => {
            h872
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, _, _, _, h873, _, _, _) => {
            h873
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, _, _, _, _, h874, _, _) => {
            h874
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, _, _, _, _, _, h875, _) => {
            h875
        }
    }, match v0 {
        Adt2::C0(_, _, _, _, _, _, _, _, _, _, h876) => {
            h876
        }
    })
}

pub fn f29(v0: u64, v1: u64, v2: u64, v3: bool, v4: Adt14, v5: Adt14, v6: Adt14) -> R<Adt15> {
    let a877 = v0;
    let a878 = 23u64;
    if nat_eq(a877, a878) {
        f74(v1, v4, v6)
    } else {
        let a879 = v0;
        let a880 = 26u64;
        if nat_eq(a879, a880) {
            f75(v1, v2, v4, v6)
        } else {
            let a881 = v0;
            let a882 = 25u64;
            if nat_eq(a881, a882) {
                f76(v1, v3, v4, v5, v6)
            } else {
                Ok(Adt15::C0)
            }
        }
    }
}

pub fn f30(v0: u64) -> u64 {
    let a883 = v0;
    let a884 = 30u64;
    if nat_eq(a883, a884) {
        2u64
    } else {
        4u64
    }
}

pub fn f31(v0: u64) -> bool {
    let a885 = v0;
    let a886 = 2139095040u64;
    if nat_le(a885, a886) {
        true
    } else {
        let a887 = v0;
        let a888 = 2147483648u64;
        nat_eq(a887, a888)
    }
}

pub fn f32(v0: Adt2, v1: u64) -> bool {
    let a890 = match v0.clone() {
        Adt2::C0(_, _, _, _, _, h889, _, _, _, _, _) => {
            h889
        }
    };
    let a891 = v1;
    if if nat_eq(a890, a891) {
        let a896 = {
            let a893 = match v0.clone() {
                Adt2::C0(_, _, _, _, _, _, _, _, _, _, h892) => {
                    h892
                }
            };
            let a894 = v1;
            let a895 = 0u64;
            nat_rem(a893, a894, a895)
        };
        let a897 = 0u64;
        nat_eq(a896, a897)
    } else {
        false
    } {
        f77(match v0.clone() {
            Adt2::C0(_, _, h898, _, _, _, _, _, _, _, _) => {
                h898
            }
        }, match v0.clone() {
            Adt2::C0(_, _, _, h899, _, _, _, _, _, _, _) => {
                h899
            }
        }, match v0.clone() {
            Adt2::C0(_, _, _, _, h900, _, _, _, _, _, _) => {
                h900
            }
        }, match v0.clone() {
            Adt2::C0(_, _, _, _, _, _, h901, _, _, _, _) => {
                h901
            }
        }, match v0.clone() {
            Adt2::C0(_, _, _, _, _, _, _, h902, _, _, _) => {
                h902
            }
        }, match v0 {
            Adt2::C0(_, _, _, _, _, _, _, _, h903, _, _) => {
                h903
            }
        }, v1)
    } else {
        false
    }
}

pub fn f33(v0: Adt2, v1: Adt2) -> bool {
    if if if if if if f25(v0.clone(), v1.clone()) {
        let a906 = match v0.clone() {
            Adt2::C0(h904, _, _, _, _, _, _, _, _, _, _) => {
                h904
            }
        };
        let a907 = match v1.clone() {
            Adt2::C0(h905, _, _, _, _, _, _, _, _, _, _) => {
                h905
            }
        };
        nat_eq(a906, a907)
    } else {
        false
    } {
        let a910 = match v0.clone() {
            Adt2::C0(_, _, _, _, _, _, _, _, _, _, h908) => {
                h908
            }
        };
        let a911 = match v1.clone() {
            Adt2::C0(_, _, _, _, _, _, _, _, _, _, h909) => {
                h909
            }
        };
        nat_eq(a910, a911)
    } else {
        false
    } {
        let a914 = match v0.clone() {
            Adt2::C0(_, _, _, _, _, h912, _, _, _, _, _) => {
                h912
            }
        };
        let a915 = match v1.clone() {
            Adt2::C0(_, _, _, _, _, h913, _, _, _, _, _) => {
                h913
            }
        };
        nat_eq(a914, a915)
    } else {
        false
    } {
        let a918 = match v0.clone() {
            Adt2::C0(_, _, _, _, _, _, h916, _, _, _, _) => {
                h916
            }
        };
        let a919 = match v1.clone() {
            Adt2::C0(_, _, _, _, _, _, h917, _, _, _, _) => {
                h917
            }
        };
        nat_eq(a918, a919)
    } else {
        false
    } {
        let a922 = match v0.clone() {
            Adt2::C0(_, _, _, _, _, _, _, h920, _, _, _) => {
                h920
            }
        };
        let a923 = match v1.clone() {
            Adt2::C0(_, _, _, _, _, _, _, h921, _, _, _) => {
                h921
            }
        };
        nat_eq(a922, a923)
    } else {
        false
    } {
        let a926 = match v0 {
            Adt2::C0(_, _, _, _, _, _, _, _, h924, _, _) => {
                h924
            }
        };
        let a927 = match v1 {
            Adt2::C0(_, _, _, _, _, _, _, _, h925, _, _) => {
                h925
            }
        };
        nat_eq(a926, a927)
    } else {
        false
    }
}

pub fn f34(v0: u64, v1: u64, v2: u64, v3: u64) -> bool {
    let a928 = v0;
    let a929 = 0u64;
    if if if nat_eq(a928, a929) {
        true
    } else {
        let a930 = v1;
        let a931 = 0u64;
        nat_eq(a930, a931)
    } {
        true
    } else {
        let a932 = v2;
        let a933 = 0u64;
        nat_eq(a932, a933)
    } {
        true
    } else {
        let a934 = v3;
        let a935 = 0u64;
        nat_eq(a934, a935)
    }
}

pub fn f35(v0: Adt2, v1: u64) -> R<bool> {
    f78({
        let a937 = match v0.clone() {
            Adt2::C0(_, h936, _, _, _, _, _, _, _, _, _) => {
                h936
            }
        };
        let a938 = v1;
        nat_mul(a937, a938)?
    }, match v0.clone() {
        Adt2::C0(_, _, h939, _, _, _, _, _, _, _, _) => {
            h939
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, h940, _, _, _, _, _, _, _) => {
            h940
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, h941, _, _, _, _, _, _) => {
            h941
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, _, _, h942, _, _, _, _) => {
            h942
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, _, _, _, h943, _, _, _) => {
            h943
        }
    }, match v0 {
        Adt2::C0(_, _, _, _, _, _, _, _, h944, _, _) => {
            h944
        }
    })
}

pub fn f36(v0: u64, v1: u64, v2: u64, v3: u64) -> R<u64> {
    let a945 = v1;
    let a946 = 0u64;
    if if nat_eq(a945, a946) {
        true
    } else {
        let a947 = v2;
        let a948 = 0u64;
        nat_eq(a947, a948)
    } {
        Ok(0u64)
    } else {
        let a955 = {
            let a949 = f79(v1, v2, v3)?;
            let a950 = f80(v0, v1, v2)?;
            nat_add(a949, a950)?
        };
        let a956 = {
            let a953 = f81(v0);
            let a954 = {
                let a951 = v1;
                let a952 = v2;
                nat_mul(a951, a952)?
            };
            nat_mul(a953, a954)?
        };
        nat_add(a955, a956)
    }
}

pub fn f37(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let a957 = v0;
    let a958 = 0u64;
    if if nat_eq(a957, a958) {
        true
    } else {
        let a959 = v1;
        let a960 = 0u64;
        nat_eq(a959, a960)
    } {
        Ok(3u64)
    } else {
        let a963 = {
            let a961 = 3u64;
            let a962 = v1;
            nat_add(a961, a962)?
        };
        let a964 = v2;
        nat_add(a963, a964)
    }
}

pub fn f38(v0: u64) -> bool {
    let a965 = v0;
    let a966 = 2139095040u64;
    if nat_le(a965, a966) {
        true
    } else {
        let a967 = v0;
        let a968 = 2147483648u64;
        nat_eq(a967, a968)
    }
}

pub fn f39(v0: u64, v1: u64) -> R<u64> {
    let a969 = v0;
    let a970 = 0u64;
    if nat_eq(a969, a970) {
        Ok(0u64)
    } else {
        let a971 = v1;
        let a972 = f82(v0)?;
        nat_mul(a971, a972)
    }
}

pub fn f40(v0: u64, v1: u64, v2: u64, v3: u64) -> R<u64> {
    let a973 = v0;
    let a974 = 0u64;
    if if if if nat_eq(a973, a974) {
        true
    } else {
        let a975 = v1;
        let a976 = 0u64;
        nat_eq(a975, a976)
    } {
        true
    } else {
        let a977 = v2;
        let a978 = 0u64;
        nat_eq(a977, a978)
    } {
        true
    } else {
        let a979 = v3;
        let a980 = 0u64;
        nat_eq(a979, a980)
    } {
        Ok(0u64)
    } else {
        let a993 = {
            let a983 = 2u64;
            let a984 = {
                let a981 = v1;
                let a982 = v2;
                nat_mul(a981, a982)?
            };
            nat_mul(a983, a984)?
        };
        let a994 = {
            let a991 = {
                let a987 = {
                    let a985 = v1;
                    let a986 = v2;
                    nat_mul(a985, a986)?
                };
                let a988 = v3;
                nat_mul(a987, a988)?
            };
            let a992 = {
                let a989 = 1u64;
                let a990 = v0;
                nat_add(a989, a990)?
            };
            nat_mul(a991, a992)?
        };
        nat_add(a993, a994)
    }
}

pub fn f41(v0: u64, v1: u64, v2: u64) -> Option<bool> {
    let v3: bool = {
        let a995 = v1;
        let a996 = 0u64;
        if if if nat_eq(a995, a996) {
            true
        } else {
            let a997 = v1;
            let a998 = 10u64;
            nat_eq(a997, a998)
        } {
            true
        } else {
            let a999 = v1;
            let a1000 = 15u64;
            nat_eq(a999, a1000)
        } {
            true
        } else {
            let a1001 = v1;
            let a1002 = 20u64;
            nat_eq(a1001, a1002)
        }
    };
    let v4: bool = {
        let a1003 = v2;
        let a1004 = 0u64;
        if if if if if nat_eq(a1003, a1004) {
            true
        } else {
            let a1005 = v2;
            let a1006 = 10u64;
            nat_eq(a1005, a1006)
        } {
            true
        } else {
            let a1007 = v2;
            let a1008 = 15u64;
            nat_eq(a1007, a1008)
        } {
            true
        } else {
            let a1009 = v2;
            let a1010 = 20u64;
            nat_eq(a1009, a1010)
        } {
            true
        } else {
            let a1011 = v2;
            let a1012 = 30u64;
            nat_eq(a1011, a1012)
        } {
            true
        } else {
            let a1013 = v2;
            let a1014 = 40u64;
            nat_eq(a1013, a1014)
        }
    };
    if if v3 {
        v4
    } else {
        false
    } {
        Some({
            let a1015 = v0;
            let a1016 = 30u64;
            if if nat_eq(a1015, a1016) {
                let a1017 = v2;
                let a1018 = 10u64;
                nat_eq(a1017, a1018)
            } else {
                false
            } {
                true
            } else {
                if f83(v0) {
                    let a1019 = v2;
                    let a1020 = 10u64;
                    if if nat_eq(a1019, a1020) {
                        true
                    } else {
                        let a1021 = v2;
                        let a1022 = 15u64;
                        nat_eq(a1021, a1022)
                    } {
                        true
                    } else {
                        let a1023 = v2;
                        let a1024 = 20u64;
                        nat_eq(a1023, a1024)
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

pub fn f42(v0: u64, v1: Adt1, v2: Adt2, v3: Option<Adt2>, v4: Adt2) -> R<Adt9> {
    let v5: u64 = match v1.clone() {
        Adt1::C0(h1025, _, _, _, _, _, _, _, _, _, _, _) => {
            h1025
        }
    };
    Ok(Adt9::C0({
        let a1026 = v0;
        let a1027 = 86u64;
        if nat_eq(a1026, a1027) {
            let m1028 = v3;
            match m1028 {
                None => {
                    Adt10::C1
                }
                Some(v6) => {
                    f84(v1, v2, v6, v4)?
                }
            }
        } else {
            let a1029 = v0;
            let a1030 = 55u64;
            if nat_eq(a1029, a1030) {
                let m1031 = v3;
                match m1031 {
                    None => {
                        Adt10::C1
                    }
                    Some(v7) => {
                        f85(v1, v2, v7, v4)?
                    }
                }
            } else {
                let a1032 = v0;
                let a1033 = 22u64;
                if nat_eq(a1032, a1033) {
                    let m1034 = v3;
                    match m1034 {
                        None => {
                            Adt10::C1
                        }
                        Some(v8) => {
                            f86(v2, v8, v4)?
                        }
                    }
                } else {
                    let a1035 = v0;
                    let a1036 = 44u64;
                    if nat_eq(a1035, a1036) {
                        f87(v1, v2, v3, v4)?
                    } else {
                        let a1037 = v0;
                        let a1038 = 52u64;
                        if nat_eq(a1037, a1038) {
                            f88(v1, v2, v4)?
                        } else {
                            let a1039 = v0;
                            let a1040 = 73u64;
                            if nat_eq(a1039, a1040) {
                                f89(v2, v3, v4)?
                            } else {
                                let a1041 = v0;
                                let a1042 = 57u64;
                                if if nat_eq(a1041, a1042) {
                                    true
                                } else {
                                    let a1043 = v0;
                                    let a1044 = 58u64;
                                    nat_eq(a1043, a1044)
                                } {
                                    let m1045 = v3;
                                    match m1045 {
                                        None => {
                                            f90({
                                                let a1046 = v0;
                                                let a1047 = 58u64;
                                                nat_eq(a1046, a1047)
                                            }, v1, v2, v4)?
                                        }
                                        Some(_) => {
                                            Adt10::C1
                                        }
                                    }
                                } else {
                                    let a1048 = v0;
                                    let a1049 = 49u64;
                                    if nat_eq(a1048, a1049) {
                                        f91(v1, v2, v3, v4)?
                                    } else {
                                        let a1050 = v0;
                                        let a1051 = 51u64;
                                        if nat_eq(a1050, a1051) {
                                            f92(v1, v2, v3, v4)?
                                        } else {
                                            let a1052 = v0;
                                            let a1053 = 39u64;
                                            if nat_eq(a1052, a1053) {
                                                f93(v2, v3, v4)?
                                            } else {
                                                let a1054 = v0;
                                                let a1055 = 77u64;
                                                if nat_eq(a1054, a1055) {
                                                    f94(v2, v4)?
                                                } else {
                                                    let a1056 = v0;
                                                    let a1057 = 40u64;
                                                    if nat_eq(a1056, a1057) {
                                                        f95(v2, v3, v4)?
                                                    } else {
                                                        if f96(v0) {
                                                            let a1058 = v0;
                                                            let a1059 = 33u64;
                                                            if if if nat_eq(a1058, a1059) {
                                                                let a1061 = match v2.clone() {
                                                                    Adt2::C0(h1060, _, _, _, _, _, _, _, _, _, _) => {
                                                                        h1060
                                                                    }
                                                                };
                                                                let a1062 = 0u64;
                                                                nat_eq(a1061, a1062)
                                                            } else {
                                                                false
                                                            } {
                                                                let a1064 = match v4.clone() {
                                                                    Adt2::C0(h1063, _, _, _, _, _, _, _, _, _, _) => {
                                                                        h1063
                                                                    }
                                                                };
                                                                let a1065 = 26u64;
                                                                nat_eq(a1064, a1065)
                                                            } else {
                                                                false
                                                            } {
                                                                let m1066 = v3;
                                                                match m1066 {
                                                                    None => {
                                                                        Adt10::C1
                                                                    }
                                                                    Some(v10) => {
                                                                        f97(v2, v10, v4)?
                                                                    }
                                                                }
                                                            } else {
                                                                let a1067 = v0;
                                                                let a1068 = 33u64;
                                                                if if if nat_eq(a1067, a1068) {
                                                                    let a1070 = match v4.clone() {
                                                                        Adt2::C0(h1069, _, _, _, _, _, _, _, _, _, _) => {
                                                                            h1069
                                                                        }
                                                                    };
                                                                    let a1071 = 0u64;
                                                                    nat_eq(a1070, a1071)
                                                                } else {
                                                                    false
                                                                } {
                                                                    f98(match v2.clone() {
                                                                        Adt2::C0(h1072, _, _, _, _, _, _, _, _, _, _) => {
                                                                            h1072
                                                                        }
                                                                    })
                                                                } else {
                                                                    false
                                                                } {
                                                                    f99(v2, v3, v4)?
                                                                } else {
                                                                    f100(v0, v2, {
                                                                        let m1073 = v3;
                                                                        match m1073 {
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
                                                            let a1074 = v0;
                                                            let a1075 = 19u64;
                                                            if nat_eq(a1074, a1075) {
                                                                f101(v2, v4)?
                                                            } else {
                                                                let a1076 = v0;
                                                                let a1077 = 21u64;
                                                                if nat_eq(a1076, a1077) {
                                                                    f102(v5, v2, v3, v4)?
                                                                } else {
                                                                    let a1078 = v0;
                                                                    let a1079 = 61u64;
                                                                    if if if nat_eq(a1078, a1079) {
                                                                        true
                                                                    } else {
                                                                        let a1080 = v0;
                                                                        let a1081 = 62u64;
                                                                        nat_eq(a1080, a1081)
                                                                    } {
                                                                        true
                                                                    } else {
                                                                        let a1082 = v0;
                                                                        let a1083 = 63u64;
                                                                        nat_eq(a1082, a1083)
                                                                    } {
                                                                        f103(v0, v1, v2, v4)?
                                                                    } else {
                                                                        let a1084 = v0;
                                                                        let a1085 = 4u64;
                                                                        if if nat_eq(a1084, a1085) {
                                                                            true
                                                                        } else {
                                                                            let a1086 = v0;
                                                                            let a1087 = 32u64;
                                                                            nat_eq(a1086, a1087)
                                                                        } {
                                                                            f104(v0, v1, v2, v3, v4)?
                                                                        } else {
                                                                            let a1088 = v0;
                                                                            let a1089 = 20u64;
                                                                            if nat_eq(a1088, a1089) {
                                                                                f105(v2, v4)?
                                                                            } else {
                                                                                let a1090 = v0;
                                                                                let a1091 = 42u64;
                                                                                if nat_eq(a1090, a1091) {
                                                                                    f106(4u64, v2, v4)?
                                                                                } else {
                                                                                    let a1092 = v0;
                                                                                    let a1093 = 69u64;
                                                                                    if nat_eq(a1092, a1093) {
                                                                                        let a1094 = v5;
                                                                                        let a1095 = 4u64;
                                                                                        if nat_lt(a1094, a1095) {
                                                                                            f106(v5, v2, v4)?
                                                                                        } else {
                                                                                            Adt10::C1
                                                                                        }
                                                                                    } else {
                                                                                        let a1096 = v0;
                                                                                        let a1097 = 70u64;
                                                                                        if nat_eq(a1096, a1097) {
                                                                                            f107(v5, v2, v4)?
                                                                                        } else {
                                                                                            let a1098 = v0;
                                                                                            let a1099 = 75u64;
                                                                                            if if nat_eq(a1098, a1099) {
                                                                                                true
                                                                                            } else {
                                                                                                let a1100 = v0;
                                                                                                let a1101 = 76u64;
                                                                                                nat_eq(a1100, a1101)
                                                                                            } {
                                                                                                f108({
                                                                                                    let a1102 = v0;
                                                                                                    let a1103 = 76u64;
                                                                                                    nat_eq(a1102, a1103)
                                                                                                }, v1, v2, v4)?
                                                                                            } else {
                                                                                                let a1104 = v0;
                                                                                                let a1105 = 31u64;
                                                                                                if if if nat_eq(a1104, a1105) {
                                                                                                    true
                                                                                                } else {
                                                                                                    let a1106 = v0;
                                                                                                    let a1107 = 47u64;
                                                                                                    nat_eq(a1106, a1107)
                                                                                                } {
                                                                                                    true
                                                                                                } else {
                                                                                                    let a1108 = v0;
                                                                                                    let a1109 = 68u64;
                                                                                                    nat_eq(a1108, a1109)
                                                                                                } {
                                                                                                    f109(v0, v1, v2, v4)?
                                                                                                } else {
                                                                                                    let a1110 = v0;
                                                                                                    let a1111 = 15u64;
                                                                                                    if nat_eq(a1110, a1111) {
                                                                                                        f110(v2, v4)?
                                                                                                    } else {
                                                                                                        let a1112 = v0;
                                                                                                        let a1113 = 17u64;
                                                                                                        if nat_eq(a1112, a1113) {
                                                                                                            f111(v2, v4)?
                                                                                                        } else {
                                                                                                            let a1114 = v0;
                                                                                                            let a1115 = 18u64;
                                                                                                            if nat_eq(a1114, a1115) {
                                                                                                                f112(v2, v3, v4)?
                                                                                                            } else {
                                                                                                                let a1116 = v0;
                                                                                                                let a1117 = 13u64;
                                                                                                                if if if nat_eq(a1116, a1117) {
                                                                                                                    true
                                                                                                                } else {
                                                                                                                    let a1118 = v0;
                                                                                                                    let a1119 = 14u64;
                                                                                                                    nat_eq(a1118, a1119)
                                                                                                                } {
                                                                                                                    true
                                                                                                                } else {
                                                                                                                    let a1120 = v0;
                                                                                                                    let a1121 = 16u64;
                                                                                                                    nat_eq(a1120, a1121)
                                                                                                                } {
                                                                                                                    f113(v0, v2, v4)?
                                                                                                                } else {
                                                                                                                    let m1122 = f114(v0, v5)?;
                                                                                                                    match m1122 {
                                                                                                                        None => {
                                                                                                                            f115(v0, v5, v2, v3, v4)?
                                                                                                                        }
                                                                                                                        Some(v12) => {
                                                                                                                            f116(v12, v2, v4)?
                                                                                                                        }
                                                                                                                    }
                                                                                                                }
                                                                                                            }
                                                                                                        }
                                                                                                    }
                                                                                                }
                                                                                            }
                                                                                        }
                                                                                    }
                                                                                }
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
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

pub fn f43(v0: Adt3, v1: u64, v2: u64, v3: u64, v4: u64) -> R<u64> {
    f117(v1, v2, v3, v4, match v0.clone() {
        Adt3::C0(_, _, _, _, h1123, _, _, _) => {
            h1123
        }
    }, match v0.clone() {
        Adt3::C0(_, _, _, _, _, h1124, _, _) => {
            h1124
        }
    }, match v0.clone() {
        Adt3::C0(_, _, _, _, _, _, h1125, _) => {
            h1125
        }
    }, match v0 {
        Adt3::C0(_, _, _, _, _, _, _, h1126) => {
            h1126
        }
    })
}

pub fn f44(v0: u64, v1: Adt3, v2: u64) -> u64 {
    let v3: u64 = {
        let a1128 = v0;
        let a1129 = match v1.clone() {
            Adt3::C0(h1127, _, _, _, _, _, _, _) => {
                h1127
            }
        };
        let a1130 = 0u64;
        nat_quot(a1128, a1129, a1130)
    };
    let v4: u64 = {
        let a1132 = v3;
        let a1133 = match v1.clone() {
            Adt3::C0(_, h1131, _, _, _, _, _, _) => {
                h1131
            }
        };
        let a1134 = 0u64;
        nat_quot(a1132, a1133, a1134)
    };
    let a1135 = v2;
    let a1136 = 0u64;
    if nat_eq(a1135, a1136) {
        let a1138 = v0;
        let a1139 = match v1 {
            Adt3::C0(h1137, _, _, _, _, _, _, _) => {
                h1137
            }
        };
        let a1140 = 0u64;
        nat_rem(a1138, a1139, a1140)
    } else {
        let a1141 = v2;
        let a1142 = 1u64;
        if nat_eq(a1141, a1142) {
            let a1144 = v3;
            let a1145 = match v1 {
                Adt3::C0(_, h1143, _, _, _, _, _, _) => {
                    h1143
                }
            };
            let a1146 = 0u64;
            nat_rem(a1144, a1145, a1146)
        } else {
            let a1147 = v2;
            let a1148 = 2u64;
            if nat_eq(a1147, a1148) {
                let a1150 = v4;
                let a1151 = match v1 {
                    Adt3::C0(_, _, h1149, _, _, _, _, _) => {
                        h1149
                    }
                };
                let a1152 = 0u64;
                nat_rem(a1150, a1151, a1152)
            } else {
                let a1154 = v4;
                let a1155 = match v1 {
                    Adt3::C0(_, _, h1153, _, _, _, _, _) => {
                        h1153
                    }
                };
                let a1156 = 0u64;
                nat_quot(a1154, a1155, a1156)
            }
        }
    }
}

pub fn f45(v0: Adt8, v1: u64, v2: u64) -> R<u64> {
    let a1158 = match v0.clone() {
        Adt8::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, h1157) => {
            h1157
        }
    };
    let a1159 = 0u64;
    if nat_eq(a1158, a1159) {
        Ok(v2)
    } else {
        let a1164 = {
            let a1161 = v2;
            let a1162 = match v0.clone() {
                Adt8::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, h1160) => {
                    h1160
                }
            };
            nat_mul(a1161, a1162)?
        };
        let a1165 = f44(v1, match v0 {
            Adt8::C0(_, _, _, _, _, _, _, _, h1163, _, _, _, _, _, _) => {
                h1163
            }
        }, 3u64);
        nat_add(a1164, a1165)
    }
}

pub fn f46(v0: Adt6, v1: u64, v2: u64, v3: u64) -> R<Option<u64>> {
    let v4: u64 = {
        let a1175 = {
            let a1172 = {
                let a1167 = v2;
                let a1168 = match v0.clone() {
                    Adt6::C0(_, h1166, _, _) => {
                        h1166
                    }
                };
                nat_mul(a1167, a1168)?
            };
            let a1173 = {
                let a1170 = v3;
                let a1171 = match v0.clone() {
                    Adt6::C0(_, _, _, h1169) => {
                        h1169
                    }
                };
                nat_mul(a1170, a1171)?
            };
            nat_add(a1172, a1173)?
        };
        let a1176 = f118(match v0.clone() {
            Adt6::C0(_, _, h1174, _) => {
                h1174
            }
        });
        nat_add(a1175, a1176)?
    };
    let v5: u64 = f119(match v0 {
        Adt6::C0(_, _, h1177, _) => {
            h1177
        }
    });
    let a1178 = v5;
    let a1179 = v4;
    if if nat_le(a1178, a1179) {
        let a1182 = {
            let a1180 = v4;
            let a1181 = v5;
            nat_sub(a1180, a1181)
        };
        let a1183 = v1;
        nat_lt(a1182, a1183)
    } else {
        false
    } {
        Ok(Some({
            let a1184 = v4;
            let a1185 = v5;
            nat_sub(a1184, a1185)
        }))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f47(v0: Adt8, v1: u64, v2: u64, v3: u64, v4: u64) -> R<Adt9> {
    let v5: Adt3 = match v0.clone() {
        Adt8::C0(_, _, _, _, _, _, _, _, _, h1186, _, _, _, _, _) => {
            h1186
        }
    };
    let v6: Adt3 = match v0.clone() {
        Adt8::C0(_, _, _, _, _, _, _, _, h1187, _, _, _, _, _, _) => {
            h1187
        }
    };
    let v7: u64 = f44(v1, v6.clone(), 2u64);
    let a1189 = match v0.clone() {
        Adt8::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, h1188) => {
            h1188
        }
    };
    let a1190 = 0u64;
    if nat_eq(a1189, a1190) {
        Ok(Adt9::C8(Some(f43(v5, v3, v4, v7, v2)?)))
    } else {
        let v8: u64 = {
            let m1193 = match v0.clone() {
                Adt8::C0(_, _, _, _, _, _, _, _, _, _, h1192, _, _, _, _) => {
                    h1192
                }
            };
            match m1193 {
                Adt3::C0(_, _, h1191, _, _, _, _, _) => {
                    h1191
                }
            }
        };
        let m1199 = f46(match v0.clone() {
            Adt8::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, h1194, _) => {
                h1194
            }
        }, match v5.clone() {
            Adt3::C0(_, _, h1195, _, _, _, _, _) => {
                h1195
            }
        }, {
            let a1196 = v2;
            let a1197 = v8;
            let a1198 = 0u64;
            nat_rem(a1196, a1197, a1198)
        }, v7)?;
        match m1199 {
            None => {
                Ok(Adt9::C8(None::<u64>))
            }
            Some(v9) => {
                let v10: u64 = {
                    let a1206 = {
                        let a1204 = {
                            let a1200 = v2;
                            let a1201 = v8;
                            let a1202 = 0u64;
                            nat_quot(a1200, a1201, a1202)
                        };
                        let a1205 = match v0 {
                            Adt8::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, h1203) => {
                                h1203
                            }
                        };
                        nat_mul(a1204, a1205)?
                    };
                    let a1207 = f44(v1, v6, 3u64);
                    nat_add(a1206, a1207)?
                };
                Ok(Adt9::C8(Some(f43(v5, v3, v4, v9, v10)?)))
            }
        }
    }
}

pub fn f48(v0: Adt4, v1: Adt9) -> Adt9 {
    let m1208 = v1;
    match m1208 {
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
            f120(f121(v0.clone(), v20, v21), v20, v21, 0u64, v22, 2u64, f122(v0), 3u64, v23)
        }
        Adt9::C4(v24, v25, v26, v27, v28, v29, v30) => {
            let v31: u64 = f123(v0.clone(), v24);
            f120(f124(v0, v24), v31, v31, v25, v26, v27, v28, v29, v30)
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

pub fn f49(v0: u64, v1: Adt3, v2: Adt3, v3: Adt4) -> R<Adt9> {
    let m1209 = v3;
    match m1209 {
        Adt4::C0 => {
            Ok(Adt9::C1(false, f125(v0, v1)?, f125(v0, v2)?))
        }
        Adt4::C1 => {
            Ok(Adt9::C1(false, f127(v0, v1, v2.clone())?, f125(v0, v2)?))
        }
        Adt4::C2(v6, v7) => {
            f128(v0, v1, v7, v2, v6)
        }
        Adt4::C3(v8, v9, v10, v11, v12) => {
            f129(v0, v1, v2, v8, v9, v10, v11, v12)
        }
        Adt4::C4(v13) => {
            f130(v0, v1, v2, v13)
        }
        Adt4::C5(v14, v15, v16, v17) => {
            f129(v0, v1, v2, v14, v15, v16, v17, true)
        }
        Adt4::C6(v18, v19, v20) => {
            f131(v0, v1, v2, v18, v19, v20)
        }
        Adt4::C7(v21) => {
            f132(v0, v1, v2, v21)
        }
        Adt4::C8(v22) => {
            f133(v22, v0, v1, v2)
        }
        Adt4::C9(_, _) => {
            Ok(Adt9::C1(true, 0u64, f125(v0, v2)?))
        }
        Adt4::C10(v25, v26, v27) => {
            f134(v0, v1, v2, v25, v26, v27)
        }
        Adt4::C11(v40, v41) => {
            f135(v0, v1, v2, v40, v41)
        }
        Adt4::C12(v42, v43, v44) => {
            f136(v0, v1, v2, v42, v43, v44)
        }
        Adt4::C13(_, v46, _, _) => {
            Ok(Adt9::C3(v46, v46, f125(v0, v1)?, f125(v0, v2)?))
        }
        Adt4::C14(v49, v50, v51) => {
            let a1212 = v49;
            let a1213 = 23u64;
            if nat_eq(a1212, a1213) {
                f137(v0, v1, v2, v50, v51)
            } else {
                Ok(Adt9::C3(v50, v51, f125(v0, v1)?, f125(v0, v2)?))
            }
        }
        Adt4::C15 => {
            f138(v0, v1, v2)
        }
        Adt4::C16(v52, v53) => {
            f139(v0, v1, v2, v52, v53)
        }
        Adt4::C17 => {
            Ok(Adt9::C0(Adt10::C1))
        }
        Adt4::C18(v54, v55) => {
            f140(v0, v1, v55, v54)
        }
        Adt4::C19(_, _) => {
            Ok(Adt9::C0(Adt10::C1))
        }
        Adt4::C20(_, _) => {
            Ok(Adt9::C0(Adt10::C1))
        }
        Adt4::C21 => {
            Ok(Adt9::C1(false, f125(f126(v0, match v2.clone() {
                Adt3::C0(h1210, _, _, _, _, _, _, _) => {
                    h1210
                }
            }, match v2.clone() {
                Adt3::C0(_, h1211, _, _, _, _, _, _) => {
                    h1211
                }
            })?, v1)?, f125(v0, v2)?))
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
            f141(v0, v60, v61, v62)
        }
        Adt4::C26(v68, _, _, _) => {
            let a1214 = v68;
            let a1215 = 64u64;
            if if if nat_eq(a1214, a1215) {
                true
            } else {
                let a1216 = v68;
                let a1217 = 65u64;
                nat_eq(a1216, a1217)
            } {
                true
            } else {
                let a1218 = v68;
                let a1219 = 66u64;
                nat_eq(a1218, a1219)
            } {
                Ok(Adt9::C0(Adt10::C5(64u64)))
            } else {
                Ok(Adt9::C0(Adt10::C1))
            }
        }
        Adt4::C27(v67) => {
            f63(v0, v1, v2, v67)
        }
        Adt4::C28(v63) => {
            f60(v0, v1, v2, v63, 0u64)
        }
        Adt4::C29(v64) => {
            f61(v0, v1, v2, v64)
        }
        Adt4::C30(v65, _) => {
            f62(v0, v1, v2, v65)
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

pub fn f50(v0: u64, v1: Adt3, v2: Adt3, v3: u64, v4: u64) -> R<Adt9> {
    let v5: u64 = f142(v3, v4);
    let v6: u64 = {
        let a1220 = v0;
        let a1221 = v5;
        let a1222 = 0u64;
        nat_quot(a1220, a1221, a1222)
    };
    let a1223 = v5;
    let a1224 = 1u64;
    if nat_eq(a1223, a1224) {
        Ok(f120(f143(v3, v4), v3, v4, 0u64, f125(v6, v1)?, 2u64, 0u64, 3u64, f125(v6, v2)?))
    } else {
        let a1228 = {
            let a1225 = v0;
            let a1226 = 2u64;
            let a1227 = 0u64;
            nat_rem(a1225, a1226, a1227)
        };
        let a1229 = 0u64;
        if nat_eq(a1228, a1229) {
            Ok(f120(f143(v3, 0u64), v3, 0u64, 0u64, f125(v6, v1)?, 2u64, 0u64, 1u64, 0u64))
        } else {
            Ok(f120(f143(0u64, v4), 0u64, v4, 1u64, 0u64, 2u64, 0u64, 3u64, f125(v6, v2)?))
        }
    }
}

pub fn f51(v0: u64, v1: u64, v2: u64) -> R<Adt9> {
    let v3: u64 = f144(v1)?;
    let v4: u64 = {
        let a1230 = v0;
        let a1231 = v3;
        let a1232 = 0u64;
        nat_rem(a1230, a1231, a1232)
    };
    let v5: u64 = {
        let a1235 = {
            let a1233 = v1;
            let a1234 = 3u64;
            nat_mul(a1233, a1234)?
        };
        let a1236 = 1u64;
        nat_add(a1235, a1236)?
    };
    let a1237 = v4;
    let a1238 = 0u64;
    if nat_eq(a1237, a1238) {
        Ok(Adt9::C7(Adt11::C0(0u64), Adt12::C2(0u64), Adt12::C2(0u64), Adt13::C0(0u64), 8u64, 8u64))
    } else {
        let a1239 = v4;
        let a1240 = v5;
        if nat_eq(a1239, a1240) {
            Ok(Adt9::C7(Adt11::C0(2u64), Adt12::C1(0u64), Adt12::C2(0u64), Adt13::C0(0u64), 8u64, 8u64))
        } else {
            let v6: u64 = {
                let a1245 = v4;
                let a1246 = {
                    let a1241 = v4;
                    let a1242 = v5;
                    if nat_lt(a1241, a1242) {
                        1u64
                    } else {
                        let a1243 = v5;
                        let a1244 = 1u64;
                        nat_add(a1243, a1244)?
                    }
                };
                nat_sub(a1245, a1246)
            };
            let v7: u64 = {
                let a1247 = v6;
                let a1248 = 3u64;
                let a1249 = 0u64;
                nat_rem(a1247, a1248, a1249)
            };
            let v8: u64 = {
                let a1260 = 4u64;
                let a1261 = {
                    let a1258 = {
                        let a1253 = {
                            let a1250 = v0;
                            let a1251 = v3;
                            let a1252 = 0u64;
                            nat_quot(a1250, a1251, a1252)
                        };
                        let a1254 = v1;
                        nat_mul(a1253, a1254)?
                    };
                    let a1259 = {
                        let a1255 = v6;
                        let a1256 = 3u64;
                        let a1257 = 0u64;
                        nat_quot(a1255, a1256, a1257)
                    };
                    nat_add(a1258, a1259)?
                };
                nat_mul(a1260, a1261)?
            };
            let a1262 = v4;
            let a1263 = v5;
            if nat_lt(a1262, a1263) {
                let a1264 = v7;
                let a1265 = 0u64;
                if nat_eq(a1264, a1265) {
                    Ok(Adt9::C7(Adt11::C1(2u64), Adt12::C0(1u64, v8), Adt12::C0(0u64, v8), Adt13::C0(1u64), 4u64, 4u64))
                } else {
                    let a1266 = v7;
                    let a1267 = 1u64;
                    if nat_eq(a1266, a1267) {
                        Ok(Adt9::C7(Adt11::C0(1u64), Adt12::C1(1u64), Adt12::C2(0u64), Adt13::C0(1u64), 4u64, 4u64))
                    } else {
                        Ok(Adt9::C7(Adt11::C0(0u64), Adt12::C1(0u64), Adt12::C1(1u64), Adt13::C0(0u64), 8u64, 8u64))
                    }
                }
            } else {
                let a1268 = v7;
                let a1269 = 0u64;
                if nat_eq(a1268, a1269) {
                    Ok(Adt9::C7(Adt11::C1(1u64), Adt12::C0(0u64, v8), Adt12::C1(0u64), Adt13::C0(1u64), 4u64, 4u64))
                } else {
                    let a1270 = v7;
                    let a1271 = 1u64;
                    if nat_eq(a1270, a1271) {
                        Ok(Adt9::C7(Adt11::C1(2u64), Adt12::C2(v2), Adt12::C1(1u64), Adt13::C0(1u64), 4u64, 4u64))
                    } else {
                        Ok(Adt9::C7(Adt11::C1(2u64), Adt12::C1(1u64), Adt12::C0(1u64, v8), Adt13::C1(v8), 4u64, 4u64))
                    }
                }
            }
        }
    }
}

pub fn f52(v0: u64, v1: Adt3, v2: Adt3, v3: Adt3) -> R<Adt9> {
    let v4: u64 = {
        let a1272 = v0;
        let a1273 = 6u64;
        let a1274 = 0u64;
        nat_quot(a1272, a1273, a1274)
    };
    let v5: u64 = {
        let a1275 = v0;
        let a1276 = 6u64;
        let a1277 = 0u64;
        nat_rem(a1275, a1276, a1277)
    };
    let a1278 = v5;
    let a1279 = 0u64;
    if nat_eq(a1278, a1279) {
        Ok(Adt9::C7(Adt11::C7(17u64, 0u64, 0u64), Adt12::C0(1u64, f125(v4, v2)?), Adt12::C2(0u64), Adt13::C0(0u64), 4u64, 4u64))
    } else {
        let a1280 = v5;
        let a1281 = 1u64;
        if nat_eq(a1280, a1281) {
            Ok(Adt9::C7(Adt11::C1(1u64), Adt12::C2(1065353216u64), Adt12::C1(0u64), Adt13::C0(1u64), 4u64, 4u64))
        } else {
            let a1282 = v5;
            let a1283 = 2u64;
            if nat_eq(a1282, a1283) {
                Ok(Adt9::C7(Adt11::C1(2u64), Adt12::C0(1u64, f125(v4, v2)?), Adt12::C1(1u64), Adt13::C0(1u64), 4u64, 4u64))
            } else {
                let a1284 = v5;
                let a1285 = 3u64;
                if nat_eq(a1284, a1285) {
                    Ok(Adt9::C7(Adt11::C1(0u64), Adt12::C2(1065353216u64), Adt12::C1(1u64), Adt13::C0(1u64), 4u64, 4u64))
                } else {
                    let a1286 = v5;
                    let a1287 = 4u64;
                    if nat_eq(a1286, a1287) {
                        Ok(Adt9::C7(Adt11::C1(2u64), Adt12::C0(0u64, f125(v4, v1)?), Adt12::C1(0u64), Adt13::C0(0u64), 4u64, 4u64))
                    } else {
                        Ok(Adt9::C7(Adt11::C1(2u64), Adt12::C1(0u64), Adt12::C1(1u64), Adt13::C1(f125(v4, v3)?), 4u64, 4u64))
                    }
                }
            }
        }
    }
}

pub fn f53(v0: u64, v1: Adt3, v2: Adt3, v3: Adt3, v4: bool, v5: u64) -> R<Adt9> {
    let v6: u64 = if v4 {
        3u64
    } else {
        2u64
    };
    let v7: u64 = f145(match v1.clone() {
        Adt3::C0(_, h1288, _, _, _, _, _, _) => {
            h1288
        }
    }, v4)?;
    let v8: u64 = {
        let a1289 = v0;
        let a1290 = v7;
        let a1291 = 0u64;
        nat_rem(a1289, a1290, a1291)
    };
    let v9: u64 = {
        let a1292 = v0;
        let a1293 = v7;
        let a1294 = 0u64;
        nat_quot(a1292, a1293, a1294)
    };
    let a1297 = {
        let a1295 = v8;
        let a1296 = 1u64;
        nat_add(a1295, a1296)?
    };
    let a1298 = v7;
    if nat_eq(a1297, a1298) {
        Ok(Adt9::C7(Adt11::C1(2u64), Adt12::C2(v5), Adt12::C1(0u64), Adt13::C1(f125(v9, v3)?), 4u64, 4u64))
    } else {
        let v10: u64 = {
            let a1299 = v8;
            let a1300 = v6;
            let a1301 = 0u64;
            nat_quot(a1299, a1300, a1301)
        };
        let v11: u64 = {
            let a1302 = v8;
            let a1303 = v6;
            let a1304 = 0u64;
            nat_rem(a1302, a1303, a1304)
        };
        let v12: u64 = {
            let a1306 = v9;
            let a1307 = match v1.clone() {
                Adt3::C0(h1305, _, _, _, _, _, _, _) => {
                    h1305
                }
            };
            let a1308 = 0u64;
            nat_rem(a1306, a1307, a1308)
        };
        let v13: u64 = {
            let a1310 = v9;
            let a1311 = match v1.clone() {
                Adt3::C0(h1309, _, _, _, _, _, _, _) => {
                    h1309
                }
            };
            let a1312 = 0u64;
            nat_quot(a1310, a1311, a1312)
        };
        let v14: u64 = if v4 {
            let a1324 = {
                let a1319 = {
                    let a1314 = match v2.clone() {
                        Adt3::C0(_, _, _, _, h1313, _, _, _) => {
                            h1313
                        }
                    };
                    let a1315 = v12;
                    nat_mul(a1314, a1315)?
                };
                let a1320 = {
                    let a1317 = match v2.clone() {
                        Adt3::C0(_, _, _, _, _, h1316, _, _) => {
                            h1316
                        }
                    };
                    let a1318 = v10;
                    nat_mul(a1317, a1318)?
                };
                nat_add(a1319, a1320)?
            };
            let a1325 = {
                let a1322 = match v2 {
                    Adt3::C0(_, _, _, _, _, _, h1321, _) => {
                        h1321
                    }
                };
                let a1323 = v13;
                nat_mul(a1322, a1323)?
            };
            nat_add(a1324, a1325)?
        } else {
            let a1332 = {
                let a1327 = match v2.clone() {
                    Adt3::C0(_, _, _, _, h1326, _, _, _) => {
                        h1326
                    }
                };
                let a1328 = v10;
                nat_mul(a1327, a1328)?
            };
            let a1333 = {
                let a1330 = match v2 {
                    Adt3::C0(_, _, _, _, _, h1329, _, _) => {
                        h1329
                    }
                };
                let a1331 = v13;
                nat_mul(a1330, a1331)?
            };
            nat_add(a1332, a1333)?
        };
        if if v4 {
            let a1334 = v11;
            let a1335 = 0u64;
            nat_eq(a1334, a1335)
        } else {
            false
        } {
            Ok(Adt9::C7(Adt11::C7(17u64, 0u64, 0u64), Adt12::C0(1u64, v14), Adt12::C2(0u64), Adt13::C0(1u64), 4u64, 4u64))
        } else {
            let a1338 = {
                let a1336 = v11;
                let a1337 = 1u64;
                nat_add(a1336, a1337)?
            };
            let a1339 = v6;
            if nat_eq(a1338, a1339) {
                Ok(Adt9::C7(Adt11::C1(0u64), {
                    let a1340 = v10;
                    let a1341 = 0u64;
                    if nat_eq(a1340, a1341) {
                        Adt12::C2(0u64)
                    } else {
                        Adt12::C1(0u64)
                    }
                }, Adt12::C1(1u64), Adt13::C0(0u64), 4u64, 4u64))
            } else {
                Ok(Adt9::C7(Adt11::C1(2u64), Adt12::C0(0u64, {
                    let a1353 = {
                        let a1348 = {
                            let a1343 = match v1.clone() {
                                Adt3::C0(_, _, _, _, h1342, _, _, _) => {
                                    h1342
                                }
                            };
                            let a1344 = v12;
                            nat_mul(a1343, a1344)?
                        };
                        let a1349 = {
                            let a1346 = match v1.clone() {
                                Adt3::C0(_, _, _, _, _, h1345, _, _) => {
                                    h1345
                                }
                            };
                            let a1347 = v10;
                            nat_mul(a1346, a1347)?
                        };
                        nat_add(a1348, a1349)?
                    };
                    let a1354 = {
                        let a1351 = match v1 {
                            Adt3::C0(_, _, _, _, _, _, h1350, _) => {
                                h1350
                            }
                        };
                        let a1352 = v13;
                        nat_mul(a1351, a1352)?
                    };
                    nat_add(a1353, a1354)?
                }), if v4 {
                    Adt12::C1(1u64)
                } else {
                    Adt12::C0(1u64, v14)
                }, Adt13::C0(1u64), 4u64, 4u64))
            }
        }
    }
}

pub fn f54(v0: u64, v1: u64, v2: Adt3, v3: Adt3) -> R<Adt9> {
    let a1355 = v0;
    let a1356 = v1;
    if nat_lt(a1355, a1356) {
        Ok(Adt9::C7(Adt11::C9, Adt12::C0(0u64, f125(v0, v2)?), Adt12::C2(0u64), Adt13::C0(v0), 4u64, 4u64))
    } else {
        let v4: u64 = {
            let a1357 = v0;
            let a1358 = v1;
            nat_sub(a1357, a1358)
        };
        Ok(Adt9::C7(Adt11::C5(3u64), Adt12::C1(v4), Adt12::C2(0u64), Adt13::C1(f125(v4, v3)?), 4u64, 4u64))
    }
}

pub fn f55(v0: u64, v1: Adt3, v2: Adt3, v3: Adt3, v4: Adt6, v5: Adt6, v6: u64) -> R<Adt9> {
    let v7: u64 = {
        let a1365 = {
            let a1363 = 3u64;
            let a1364 = {
                let a1361 = match v4.clone() {
                    Adt6::C0(h1359, _, _, _) => {
                        h1359
                    }
                };
                let a1362 = match v5.clone() {
                    Adt6::C0(h1360, _, _, _) => {
                        h1360
                    }
                };
                nat_mul(a1361, a1362)?
            };
            nat_mul(a1363, a1364)?
        };
        let a1366 = 2u64;
        nat_add(a1365, a1366)?
    };
    let v8: u64 = {
        let a1367 = v0;
        let a1368 = v7;
        let a1369 = 0u64;
        nat_quot(a1367, a1368, a1369)
    };
    let v9: u64 = {
        let a1370 = v0;
        let a1371 = v7;
        let a1372 = 0u64;
        nat_rem(a1370, a1371, a1372)
    };
    let a1373 = v9;
    let a1374 = 0u64;
    if nat_eq(a1373, a1374) {
        Ok(Adt9::C7(Adt11::C2(0u64, 0u64, false), Adt12::C2(0u64), Adt12::C2(0u64), Adt13::C0(0u64), 4u64, 4u64))
    } else {
        let a1377 = {
            let a1375 = v9;
            let a1376 = 1u64;
            nat_add(a1375, a1376)?
        };
        let a1378 = v7;
        if nat_eq(a1377, a1378) {
            Ok(Adt9::C7(Adt11::C2(0u64, 0u64, false), Adt12::C1(0u64), Adt12::C2(0u64), Adt13::C1(f125(v8, v3)?), 4u64, 4u64))
        } else {
            let v10: u64 = {
                let a1379 = v9;
                let a1380 = 1u64;
                nat_sub(a1379, a1380)
            };
            let v11: u64 = {
                let a1381 = v10;
                let a1382 = 3u64;
                let a1383 = 0u64;
                nat_quot(a1381, a1382, a1383)
            };
            let v12: u64 = {
                let a1385 = v11;
                let a1386 = match v4.clone() {
                    Adt6::C0(h1384, _, _, _) => {
                        h1384
                    }
                };
                let a1387 = 0u64;
                nat_rem(a1385, a1386, a1387)
            };
            let v13: u64 = {
                let a1389 = v11;
                let a1390 = match v4.clone() {
                    Adt6::C0(h1388, _, _, _) => {
                        h1388
                    }
                };
                let a1391 = 0u64;
                nat_quot(a1389, a1390, a1391)
            };
            let v14: u64 = {
                let a1392 = v10;
                let a1393 = 3u64;
                let a1394 = 0u64;
                nat_rem(a1392, a1393, a1394)
            };
            let v15: u64 = f44(v8, v3.clone(), 2u64);
            let m1396 = f46(v4, match v2.clone() {
                Adt3::C0(h1395, _, _, _, _, _, _, _) => {
                    h1395
                }
            }, f44(v8, v3.clone(), 0u64), v12)?;
            match m1396 {
                None => {
                    Ok(Adt9::C7(Adt11::C2(0u64, 0u64, false), Adt12::C1(0u64), Adt12::C2(0u64), Adt13::C0(0u64), 4u64, 4u64))
                }
                Some(v16) => {
                    let m1398 = f46(v5, match v2.clone() {
                        Adt3::C0(_, h1397, _, _, _, _, _, _) => {
                            h1397
                        }
                    }, f44(v8, v3.clone(), 1u64), v13)?;
                    match m1398 {
                        None => {
                            Ok(Adt9::C7(Adt11::C2(0u64, 0u64, false), Adt12::C1(0u64), Adt12::C2(0u64), Adt13::C0(0u64), 4u64, 4u64))
                        }
                        Some(v17) => {
                            let a1399 = v14;
                            let a1400 = 0u64;
                            if nat_eq(a1399, a1400) {
                                Ok(Adt9::C7(Adt11::C2(v6, 0u64, false), Adt12::C0(0u64, f43(v1, v12, v13, 0u64, v15)?), Adt12::C2(0u64), Adt13::C0(1u64), f146(v6), 4u64))
                            } else {
                                let a1401 = v14;
                                let a1402 = 1u64;
                                if nat_eq(a1401, a1402) {
                                    Ok(Adt9::C7(Adt11::C1(2u64), Adt12::C1(1u64), Adt12::C0(1u64, f43(v2, v16, v17, v15, f44(v8, v3, 3u64))?), Adt13::C0(1u64), 4u64, 4u64))
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

pub fn f56(v0: u64, v1: Adt3, v2: Adt3, v3: u64) -> R<Adt9> {
    let v4: u64 = f147(match v1.clone() {
        Adt3::C0(h1403, _, _, _, _, _, _, _) => {
            h1403
        }
    })?;
    let v5: u64 = {
        let a1404 = v0;
        let a1405 = v4;
        let a1406 = 0u64;
        nat_quot(a1404, a1405, a1406)
    };
    let v6: u64 = {
        let a1407 = v0;
        let a1408 = v4;
        let a1409 = 0u64;
        nat_rem(a1407, a1408, a1409)
    };
    let a1410 = v6;
    let a1411 = 0u64;
    if nat_eq(a1410, a1411) {
        Ok(Adt9::C4(19u64, 2u64, 4286578688u64, 2u64, 0u64, 1u64, 0u64))
    } else {
        let a1412 = v6;
        let a1413 = 1u64;
        if nat_eq(a1412, a1413) {
            Ok(Adt9::C4(19u64, 2u64, 0u64, 2u64, 0u64, 1u64, 1u64))
        } else {
            let a1416 = {
                let a1414 = v6;
                let a1415 = 1u64;
                nat_add(a1414, a1415)?
            };
            let a1417 = v4;
            if nat_eq(a1416, a1417) {
                Ok(Adt9::C4(19u64, 1u64, 1u64, 2u64, 0u64, 3u64, f125(v5, v2)?))
            } else {
                let v7: u64 = {
                    let a1418 = v6;
                    let a1419 = 2u64;
                    nat_sub(a1418, a1419)
                };
                let v8: u64 = {
                    let a1420 = v7;
                    let a1421 = 3u64;
                    let a1422 = 0u64;
                    nat_quot(a1420, a1421, a1422)
                };
                let v9: u64 = {
                    let a1423 = v7;
                    let a1424 = 3u64;
                    let a1425 = 0u64;
                    nat_rem(a1423, a1424, a1425)
                };
                let v10: u64 = f125({
                    let a1429 = {
                        let a1427 = v5;
                        let a1428 = match v1.clone() {
                            Adt3::C0(h1426, _, _, _, _, _, _, _) => {
                                h1426
                            }
                        };
                        nat_mul(a1427, a1428)?
                    };
                    let a1430 = v8;
                    nat_add(a1429, a1430)?
                }, v1)?;
                let a1431 = v9;
                let a1432 = 0u64;
                if nat_eq(a1431, a1432) {
                    Ok(Adt9::C4(16u64, 1u64, 0u64, 0u64, v10, 1u64, 2u64))
                } else {
                    let a1433 = v9;
                    let a1434 = 1u64;
                    if nat_eq(a1433, a1434) {
                        let a1435 = v3;
                        let a1436 = 2u64;
                        if nat_eq(a1435, a1436) {
                            Ok(Adt9::C4(19u64, 1u64, 0u64, 2u64, 0u64, 1u64, 0u64))
                        } else {
                            Ok(Adt9::C4(19u64, 0u64, v10, 2u64, 0u64, 1u64, 0u64))
                        }
                    } else {
                        let a1437 = v3;
                        let a1438 = 2u64;
                        if if nat_eq(a1437, a1438) {
                            true
                        } else {
                            let a1439 = v3;
                            let a1440 = 3u64;
                            nat_eq(a1439, a1440)
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

pub fn f57(v0: u64, v1: u64, v2: Adt3, v3: Adt3, v4: Adt3, v5: u64) -> R<Adt9> {
    let v6: u64 = {
        let a1442 = 2u64;
        let a1443 = match v2.clone() {
            Adt3::C0(_, h1441, _, _, _, _, _, _) => {
                h1441
            }
        };
        nat_mul(a1442, a1443)?
    };
    let a1444 = v1;
    let a1445 = v6;
    if nat_lt(a1444, a1445) {
        let a1449 = {
            let a1446 = v1;
            let a1447 = 2u64;
            let a1448 = 0u64;
            nat_rem(a1446, a1447, a1448)
        };
        let a1450 = 0u64;
        if nat_eq(a1449, a1450) {
            Ok(Adt9::C4(19u64, 4u64, f125({
                let a1451 = v1;
                let a1452 = 2u64;
                let a1453 = 0u64;
                nat_quot(a1451, a1452, a1453)
            }, v4)?, 2u64, 0u64, 1u64, 2u64))
        } else {
            let a1454 = v5;
            let a1455 = 2147483648u64;
            if if nat_lt(a1454, a1455) {
                let a1457 = v5;
                let a1458 = match v3 {
                    Adt3::C0(_, h1456, _, _, _, _, _, _) => {
                        h1456
                    }
                };
                nat_lt(a1457, a1458)
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
            let a1461 = match v3.clone() {
                Adt3::C0(h1459, _, _, _, _, _, _, _) => {
                    h1459
                }
            };
            let a1462 = match v3.clone() {
                Adt3::C0(_, h1460, _, _, _, _, _, _) => {
                    h1460
                }
            };
            nat_mul(a1461, a1462)?
        };
        let a1465 = {
            let a1463 = v1;
            let a1464 = v6;
            nat_sub(a1463, a1464)
        };
        let a1466 = v7;
        if nat_lt(a1465, a1466) {
            Ok(Adt9::C4(19u64, 2u64, 0u64, 2u64, 0u64, 3u64, f125({
                let a1467 = v1;
                let a1468 = v6;
                nat_sub(a1467, a1468)
            }, v3)?))
        } else {
            let v8: u64 = {
                let a1471 = {
                    let a1469 = v1;
                    let a1470 = v6;
                    nat_sub(a1469, a1470)
                };
                let a1472 = v7;
                nat_sub(a1471, a1472)
            };
            let v9: u64 = {
                let a1474 = 1u64;
                let a1475 = f148(v0, match v2.clone() {
                    Adt3::C0(h1473, _, _, _, _, _, _, _) => {
                        h1473
                    }
                })?;
                nat_add(a1474, a1475)?
            };
            let v10: u64 = {
                let a1476 = v8;
                let a1477 = v9;
                let a1478 = 0u64;
                nat_quot(a1476, a1477, a1478)
            };
            let v11: u64 = {
                let a1479 = v8;
                let a1480 = v9;
                let a1481 = 0u64;
                nat_rem(a1479, a1480, a1481)
            };
            let a1482 = v11;
            let a1483 = 0u64;
            if nat_eq(a1482, a1483) {
                Ok(Adt9::C4(19u64, 4u64, f125(v10, v4)?, 2u64, 0u64, 1u64, 2u64))
            } else {
                if f149(v0) {
                    let a1487 = {
                        let a1484 = v11;
                        let a1485 = 2u64;
                        let a1486 = 0u64;
                        nat_rem(a1484, a1485, a1486)
                    };
                    let a1488 = 1u64;
                    if nat_eq(a1487, a1488) {
                        Ok(Adt9::C4(6u64, 0u64, f125({
                            let a1497 = {
                                let a1490 = v10;
                                let a1491 = match v2.clone() {
                                    Adt3::C0(h1489, _, _, _, _, _, _, _) => {
                                        h1489
                                    }
                                };
                                nat_mul(a1490, a1491)?
                            };
                            let a1498 = {
                                let a1494 = {
                                    let a1492 = v11;
                                    let a1493 = 1u64;
                                    nat_sub(a1492, a1493)
                                };
                                let a1495 = 2u64;
                                let a1496 = 0u64;
                                nat_quot(a1494, a1495, a1496)
                            };
                            nat_add(a1497, a1498)?
                        }, v2)?, 2u64, 0u64, 1u64, 0u64))
                    } else {
                        Ok(Adt9::C4(4u64, 3u64, f125({
                            let a1507 = {
                                let a1500 = v5;
                                let a1501 = match v2.clone() {
                                    Adt3::C0(h1499, _, _, _, _, _, _, _) => {
                                        h1499
                                    }
                                };
                                nat_mul(a1500, a1501)?
                            };
                            let a1508 = {
                                let a1504 = {
                                    let a1502 = v11;
                                    let a1503 = 1u64;
                                    nat_sub(a1502, a1503)
                                };
                                let a1505 = 2u64;
                                let a1506 = 0u64;
                                nat_quot(a1504, a1505, a1506)
                            };
                            nat_add(a1507, a1508)?
                        }, v3.clone())?, 1u64, 0u64, 3u64, f125({
                            let a1517 = {
                                let a1510 = v5;
                                let a1511 = match v2 {
                                    Adt3::C0(h1509, _, _, _, _, _, _, _) => {
                                        h1509
                                    }
                                };
                                nat_mul(a1510, a1511)?
                            };
                            let a1518 = {
                                let a1514 = {
                                    let a1512 = v11;
                                    let a1513 = 1u64;
                                    nat_sub(a1512, a1513)
                                };
                                let a1515 = 2u64;
                                let a1516 = 0u64;
                                nat_quot(a1514, a1515, a1516)
                            };
                            nat_add(a1517, a1518)?
                        }, v3)?))
                    }
                } else {
                    f150(v0, v10, {
                        let a1519 = v11;
                        let a1520 = 1u64;
                        nat_sub(a1519, a1520)
                    }, v5, v2, v3)
                }
            }
        }
    }
}

pub fn f58(v0: u64, v1: u64, v2: Adt3, v3: Adt3, v4: Adt3, v5: u64) -> R<Adt9> {
    let a1521 = v1;
    let a1522 = f151(v2.clone(), v4.clone())?;
    if nat_lt(a1521, a1522) {
        let a1525 = v1;
        let a1526 = {
            let a1523 = 2u64;
            let a1524 = f152(v4.clone())?;
            nat_mul(a1523, a1524)?
        };
        if nat_lt(a1525, a1526) {
            let a1530 = {
                let a1527 = v1;
                let a1528 = 2u64;
                let a1529 = 0u64;
                nat_rem(a1527, a1528, a1529)
            };
            let a1531 = 0u64;
            if nat_eq(a1530, a1531) {
                f153({
                    let a1532 = v1;
                    let a1533 = 2u64;
                    let a1534 = 0u64;
                    nat_quot(a1532, a1533, a1534)
                }, v4)
            } else {
                if f154(match v2 {
                    Adt3::C0(_, h1535, _, _, _, _, _, _) => {
                        h1535
                    }
                }, v5) {
                    Ok(Adt9::C4(19u64, 1u64, 2u64, 2u64, 0u64, 1u64, 2u64))
                } else {
                    Ok(Adt9::C0(Adt10::C1))
                }
            }
        } else {
            let v6: u64 = {
                let a1538 = v1;
                let a1539 = {
                    let a1536 = 2u64;
                    let a1537 = f152(v4.clone())?;
                    nat_mul(a1536, a1537)?
                };
                nat_sub(a1538, a1539)
            };
            let v7: u64 = {
                let a1543 = v6;
                let a1544 = {
                    let a1541 = 1u64;
                    let a1542 = match v2.clone() {
                        Adt3::C0(h1540, _, _, _, _, _, _, _) => {
                            h1540
                        }
                    };
                    nat_add(a1541, a1542)?
                };
                let a1545 = 0u64;
                nat_quot(a1543, a1544, a1545)
            };
            let v8: u64 = {
                let a1549 = v6;
                let a1550 = {
                    let a1547 = 1u64;
                    let a1548 = match v2.clone() {
                        Adt3::C0(h1546, _, _, _, _, _, _, _) => {
                            h1546
                        }
                    };
                    nat_add(a1547, a1548)?
                };
                let a1551 = 0u64;
                nat_rem(a1549, a1550, a1551)
            };
            let a1552 = v8;
            let a1553 = 0u64;
            if nat_eq(a1552, a1553) {
                f153(v7, v4)
            } else {
                f155(v0, v7, {
                    let a1554 = v8;
                    let a1555 = 1u64;
                    nat_sub(a1554, a1555)
                }, v2, v3, v4, v5)
            }
        }
    } else {
        Ok(Adt9::C0(Adt10::C1))
    }
}

pub fn f59(v0: u64, v1: Adt3, v2: Adt3, v3: Adt3, v4: Adt3, v5: u64) -> R<Adt9> {
    let v6: u64 = {
        let a1558 = match v4.clone() {
            Adt3::C0(h1556, _, _, _, _, _, _, _) => {
                h1556
            }
        };
        let a1559 = match v4.clone() {
            Adt3::C0(_, h1557, _, _, _, _, _, _) => {
                h1557
            }
        };
        nat_mul(a1558, a1559)?
    };
    let v7: u64 = {
        let a1560 = 2u64;
        let a1561 = v6;
        nat_mul(a1560, a1561)?
    };
    let v8: u64 = f156(match v3.clone() {
        Adt3::C0(_, h1562, _, _, _, _, _, _) => {
            h1562
        }
    }, match v3.clone() {
        Adt3::C0(_, _, h1563, _, _, _, _, _) => {
            h1563
        }
    }, match v3.clone() {
        Adt3::C0(_, _, _, h1564, _, _, _, _) => {
            h1564
        }
    });
    let a1565 = v0;
    let a1566 = v7;
    if nat_lt(a1565, a1566) {
        let a1570 = {
            let a1567 = v0;
            let a1568 = 2u64;
            let a1569 = 0u64;
            nat_rem(a1567, a1568, a1569)
        };
        let a1571 = 0u64;
        if nat_eq(a1570, a1571) {
            Ok(Adt9::C4(19u64, 5u64, f125({
                let a1572 = v0;
                let a1573 = 2u64;
                let a1574 = 0u64;
                nat_quot(a1572, a1573, a1574)
            }, v4)?, 2u64, 0u64, 1u64, 2u64))
        } else {
            if f157(v5, v8) {
                Ok(Adt9::C4(19u64, 1u64, 2u64, 2u64, 0u64, 1u64, 2u64))
            } else {
                Ok(Adt9::C0(Adt10::C1))
            }
        }
    } else {
        let v9: u64 = {
            let a1575 = v0;
            let a1576 = v7;
            nat_sub(a1575, a1576)
        };
        let v10: u64 = {
            let a1580 = v9;
            let a1581 = {
                let a1578 = 1u64;
                let a1579 = match v1.clone() {
                    Adt3::C0(h1577, _, _, _, _, _, _, _) => {
                        h1577
                    }
                };
                nat_add(a1578, a1579)?
            };
            let a1582 = 0u64;
            nat_quot(a1580, a1581, a1582)
        };
        let v11: u64 = {
            let a1586 = v9;
            let a1587 = {
                let a1584 = 1u64;
                let a1585 = match v1.clone() {
                    Adt3::C0(h1583, _, _, _, _, _, _, _) => {
                        h1583
                    }
                };
                nat_add(a1584, a1585)?
            };
            let a1588 = 0u64;
            nat_rem(a1586, a1587, a1588)
        };
        let a1589 = v11;
        let a1590 = 0u64;
        if nat_eq(a1589, a1590) {
            Ok(Adt9::C4(19u64, 5u64, f125({
                let a1591 = v10;
                let a1592 = v6;
                let a1593 = 0u64;
                nat_rem(a1591, a1592, a1593)
            }, v4)?, 2u64, 0u64, 1u64, 2u64))
        } else {
            if f157(v5, v8) {
                let v12: u64 = {
                    let a1599 = {
                        let a1595 = v10;
                        let a1596 = match v1.clone() {
                            Adt3::C0(h1594, _, _, _, _, _, _, _) => {
                                h1594
                            }
                        };
                        nat_mul(a1595, a1596)?
                    };
                    let a1600 = {
                        let a1597 = v11;
                        let a1598 = 1u64;
                        nat_sub(a1597, a1598)
                    };
                    nat_add(a1599, a1600)?
                };
                let a1601 = v5;
                let a1602 = 4294967295u64;
                if nat_eq(a1601, a1602) {
                    Ok(Adt9::C1(false, f125(v12, v1)?, f125(v12, v2)?))
                } else {
                    Ok(Adt9::C4(8u64, 0u64, f125(v12, v1)?, 4u64, f125({
                        let a1608 = {
                            let a1604 = v5;
                            let a1605 = match v3.clone() {
                                Adt3::C0(h1603, _, _, _, _, _, _, _) => {
                                    h1603
                                }
                            };
                            nat_mul(a1604, a1605)?
                        };
                        let a1609 = {
                            let a1606 = v11;
                            let a1607 = 1u64;
                            nat_sub(a1606, a1607)
                        };
                        nat_add(a1608, a1609)?
                    }, v3)?, 3u64, f125(v12, v2)?))
                }
            } else {
                Ok(Adt9::C0(Adt10::C1))
            }
        }
    }
}

pub fn f60(v0: u64, v1: Adt3, v2: Adt3, v3: Adt7, v4: u64) -> R<Adt9> {
    let v5: u64 = f158(v3.clone())?;
    let v6: u64 = {
        let a1610 = v0;
        let a1611 = v5;
        let a1612 = 0u64;
        nat_quot(a1610, a1611, a1612)
    };
    let v7: u64 = {
        let a1613 = v0;
        let a1614 = v5;
        let a1615 = 0u64;
        nat_rem(a1613, a1614, a1615)
    };
    let v8: u64 = f159(v3.clone())?;
    let a1616 = v7;
    let a1617 = 0u64;
    if nat_eq(a1616, a1617) {
        Ok(Adt9::C4(19u64, 2u64, if match v3 {
            Adt7::C0(_, _, _, h1618, _) => {
                h1618
            }
        } {
            4286578687u64
        } else {
            0u64
        }, 2u64, 0u64, 1u64, 0u64))
    } else {
        let a1622 = v5;
        let a1623 = {
            let a1620 = v7;
            let a1621 = if match v3.clone() {
                Adt7::C0(_, _, _, h1619, _) => {
                    h1619
                }
            } {
                1u64
            } else {
                2u64
            };
            nat_add(a1620, a1621)?
        };
        if nat_le(a1622, a1623) {
            if match v3.clone() {
                Adt7::C0(_, _, _, h1624, _) => {
                    h1624
                }
            } {
                Ok(Adt9::C4(19u64, 1u64, 0u64, 2u64, 0u64, 3u64, f125(v6, v2)?))
            } else {
                let a1627 = {
                    let a1625 = v7;
                    let a1626 = 2u64;
                    nat_add(a1625, a1626)?
                };
                let a1628 = v5;
                if nat_eq(a1627, a1628) {
                    let v9: u64 = if match v3.clone() {
                        Adt7::C0(_, _, h1629, _, _) => {
                            h1629
                        }
                    } {
                        let a1636 = {
                            let m1632 = match v3.clone() {
                                Adt7::C0(h1631, _, _, _, _) => {
                                    h1631
                                }
                            };
                            match m1632 {
                                Adt6::C0(h1630, _, _, _) => {
                                    h1630
                                }
                            }
                        };
                        let a1637 = {
                            let m1635 = match v3 {
                                Adt7::C0(_, h1634, _, _, _) => {
                                    h1634
                                }
                            };
                            match m1635 {
                                Adt6::C0(h1633, _, _, _) => {
                                    h1633
                                }
                            }
                        };
                        nat_mul(a1636, a1637)?
                    } else {
                        f160(match v3 {
                            Adt7::C0(h1638, _, _, _, _) => {
                                h1638
                            }
                        }, match v1 {
                            Adt3::C0(h1639, _, _, _, _, _, _, _) => {
                                h1639
                            }
                        }, f44(v6, v2, 0u64))?
                    };
                    Ok(Adt9::C4(3u64, 2u64, {
                        let a1640 = v9;
                        let a1641 = 0u64;
                        if nat_eq(a1640, a1641) {
                            1u64
                        } else {
                            v9
                        }
                    }, 2u64, 0u64, 1u64, 1u64))
                } else {
                    Ok(Adt9::C4(5u64, 1u64, 0u64, 1u64, 1u64, 3u64, f125(v6, v2)?))
                }
            }
        } else {
            let v10: u64 = {
                let a1644 = {
                    let a1642 = v7;
                    let a1643 = 1u64;
                    nat_sub(a1642, a1643)
                };
                let a1645 = v8;
                let a1646 = 0u64;
                nat_rem(a1644, a1645, a1646)
            };
            let m1652 = f161(v3.clone(), v1, v2, v6, {
                let a1649 = {
                    let a1647 = v7;
                    let a1648 = 1u64;
                    nat_sub(a1647, a1648)
                };
                let a1650 = v8;
                let a1651 = 0u64;
                nat_quot(a1649, a1650, a1651)
            })?;
            match m1652 {
                None => {
                    Ok(Adt9::C4(19u64, 1u64, 0u64, 2u64, 0u64, 1u64, 0u64))
                }
                Some(v11) => {
                    let a1654 = match v3.clone() {
                        Adt7::C0(_, _, _, _, h1653) => {
                            h1653
                        }
                    };
                    let a1655 = 1u64;
                    if if nat_eq(a1654, a1655) {
                        let a1656 = v10;
                        let a1657 = 0u64;
                        nat_eq(a1656, a1657)
                    } else {
                        false
                    } {
                        Ok(Adt9::C4(6u64, 0u64, v11, 2u64, 0u64, 1u64, 1u64))
                    } else {
                        if match v3.clone() {
                            Adt7::C0(_, _, _, h1658, _) => {
                                h1658
                            }
                        } {
                            let a1661 = {
                                let a1659 = v10;
                                let a1660 = 1u64;
                                nat_add(a1659, a1660)?
                            };
                            let a1662 = v8;
                            if nat_eq(a1661, a1662) {
                                let a1663 = v4;
                                let a1664 = 2u64;
                                if nat_eq(a1663, a1664) {
                                    Ok(Adt9::C4(19u64, 1u64, 0u64, 2u64, 0u64, 1u64, 0u64))
                                } else {
                                    Ok(Adt9::C4(19u64, {
                                        let a1666 = match v3.clone() {
                                            Adt7::C0(_, _, _, _, h1665) => {
                                                h1665
                                            }
                                        };
                                        let a1667 = 1u64;
                                        if nat_eq(a1666, a1667) {
                                            1u64
                                        } else {
                                            0u64
                                        }
                                    }, {
                                        let a1669 = match v3 {
                                            Adt7::C0(_, _, _, _, h1668) => {
                                                h1668
                                            }
                                        };
                                        let a1670 = 1u64;
                                        if nat_eq(a1669, a1670) {
                                            1u64
                                        } else {
                                            v11
                                        }
                                    }, 2u64, 0u64, 1u64, 0u64))
                                }
                            } else {
                                Ok(Adt9::C4(16u64, 1u64, 0u64, {
                                    let a1672 = match v3.clone() {
                                        Adt7::C0(_, _, _, _, h1671) => {
                                            h1671
                                        }
                                    };
                                    let a1673 = 1u64;
                                    if nat_eq(a1672, a1673) {
                                        1u64
                                    } else {
                                        0u64
                                    }
                                }, {
                                    let a1675 = match v3 {
                                        Adt7::C0(_, _, _, _, h1674) => {
                                            h1674
                                        }
                                    };
                                    let a1676 = 1u64;
                                    if nat_eq(a1675, a1676) {
                                        1u64
                                    } else {
                                        v11
                                    }
                                }, 1u64, 2u64))
                            }
                        } else {
                            Ok(Adt9::C4(4u64, 1u64, 0u64, {
                                let a1678 = match v3.clone() {
                                    Adt7::C0(_, _, _, _, h1677) => {
                                        h1677
                                    }
                                };
                                let a1679 = 1u64;
                                if nat_eq(a1678, a1679) {
                                    1u64
                                } else {
                                    0u64
                                }
                            }, {
                                let a1681 = match v3 {
                                    Adt7::C0(_, _, _, _, h1680) => {
                                        h1680
                                    }
                                };
                                let a1682 = 1u64;
                                if nat_eq(a1681, a1682) {
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

pub fn f61(v0: u64, v1: Adt3, v2: Adt3, v3: Adt3) -> R<Adt9> {
    let v4: u64 = f162(match v3.clone() {
        Adt3::C0(h1683, _, _, _, _, _, _, _) => {
            h1683
        }
    })?;
    let v5: u64 = {
        let a1684 = v0;
        let a1685 = v4;
        let a1686 = 0u64;
        nat_rem(a1684, a1685, a1686)
    };
    let v6: u64 = {
        let a1687 = v0;
        let a1688 = v4;
        let a1689 = 0u64;
        nat_quot(a1687, a1688, a1689)
    };
    let a1690 = v5;
    let a1691 = 0u64;
    if nat_eq(a1690, a1691) {
        Ok(Adt9::C4(19u64, 2u64, 0u64, 2u64, 0u64, 1u64, 0u64))
    } else {
        let a1694 = {
            let a1692 = v5;
            let a1693 = 1u64;
            nat_add(a1692, a1693)?
        };
        let a1695 = v4;
        if nat_eq(a1694, a1695) {
            Ok(Adt9::C4(19u64, 1u64, 0u64, 2u64, 0u64, 3u64, f125(v6, v2)?))
        } else {
            let a1699 = {
                let a1696 = v5;
                let a1697 = 2u64;
                let a1698 = 0u64;
                nat_rem(a1696, a1697, a1698)
            };
            let a1700 = 0u64;
            if nat_eq(a1699, a1700) {
                Ok(Adt9::C4(4u64, 1u64, 0u64, 1u64, 1u64, 1u64, 0u64))
            } else {
                let v7: u64 = {
                    let a1703 = {
                        let a1701 = v5;
                        let a1702 = 1u64;
                        nat_sub(a1701, a1702)
                    };
                    let a1704 = 2u64;
                    let a1705 = 0u64;
                    nat_quot(a1703, a1704, a1705)
                };
                let v8: u64 = f44(v6, v2.clone(), 0u64);
                let v9: u64 = f44(v6, v2.clone(), 1u64);
                let v10: u64 = f44(v6, v2, 2u64);
                Ok(Adt9::C4(10u64, 0u64, f43(v1, {
                    let a1706 = v9;
                    let a1707 = v7;
                    nat_add(a1706, a1707)?
                }, v8, v10, 0u64)?, 4u64, f43(v3, v7, v8, 0u64, 0u64)?, 1u64, 1u64))
            }
        }
    }
}

pub fn f62(v0: u64, v1: Adt3, v2: Adt3, v3: Adt6) -> R<Adt9> {
    let v4: u64 = f163(v3.clone())?;
    let v5: u64 = {
        let a1708 = v0;
        let a1709 = v4;
        let a1710 = 0u64;
        nat_rem(a1708, a1709, a1710)
    };
    let v6: u64 = {
        let a1711 = v0;
        let a1712 = v4;
        let a1713 = 0u64;
        nat_quot(a1711, a1712, a1713)
    };
    let a1714 = v5;
    let a1715 = 0u64;
    if nat_eq(a1714, a1715) {
        Ok(Adt9::C4(19u64, 2u64, 0u64, 2u64, 0u64, 1u64, 0u64))
    } else {
        let a1718 = {
            let a1716 = v5;
            let a1717 = 1u64;
            nat_add(a1716, a1717)?
        };
        let a1719 = v4;
        if nat_eq(a1718, a1719) {
            Ok(Adt9::C4(7u64, 1u64, 0u64, 2u64, 0u64, 3u64, f125(v6, v2)?))
        } else {
            let a1723 = {
                let a1720 = v5;
                let a1721 = 2u64;
                let a1722 = 0u64;
                nat_rem(a1720, a1721, a1722)
            };
            let a1724 = 0u64;
            if nat_eq(a1723, a1724) {
                Ok(Adt9::C4(4u64, 1u64, 0u64, 1u64, 1u64, 1u64, 0u64))
            } else {
                let v7: u64 = {
                    let a1726 = f44(v6, v2.clone(), 0u64);
                    let a1727 = match v3.clone() {
                        Adt6::C0(_, _, h1725, _) => {
                            h1725
                        }
                    };
                    nat_add(a1726, a1727)?
                };
                let v8: u64 = {
                    let a1735 = {
                        let a1732 = {
                            let a1729 = v7;
                            let a1730 = match v3.clone() {
                                Adt6::C0(_, h1728, _, _) => {
                                    h1728
                                }
                            };
                            nat_add(a1729, a1730)?
                        };
                        let a1733 = match v3.clone() {
                            Adt6::C0(h1731, _, _, _) => {
                                h1731
                            }
                        };
                        nat_sub(a1732, a1733)
                    };
                    let a1736 = match v3.clone() {
                        Adt6::C0(_, h1734, _, _) => {
                            h1734
                        }
                    };
                    let a1737 = 0u64;
                    nat_quot(a1735, a1736, a1737)
                };
                let v9: u64 = {
                    let a1743 = v8;
                    let a1744 = {
                        let a1740 = {
                            let a1738 = v5;
                            let a1739 = 1u64;
                            nat_sub(a1738, a1739)
                        };
                        let a1741 = 2u64;
                        let a1742 = 0u64;
                        nat_quot(a1740, a1741, a1742)
                    };
                    nat_add(a1743, a1744)?
                };
                let v10: u64 = {
                    let a1746 = v9;
                    let a1747 = match v3.clone() {
                        Adt6::C0(_, h1745, _, _) => {
                            h1745
                        }
                    };
                    nat_mul(a1746, a1747)?
                };
                let v11: u64 = {
                    let a1748 = v7;
                    let a1749 = v10;
                    nat_sub(a1748, a1749)
                };
                let a1751 = v9;
                let a1752 = match v1.clone() {
                    Adt3::C0(_, h1750, _, _, _, _, _, _) => {
                        h1750
                    }
                };
                if if if nat_lt(a1751, a1752) {
                    let a1753 = v10;
                    let a1754 = v7;
                    nat_le(a1753, a1754)
                } else {
                    false
                } {
                    let a1756 = v11;
                    let a1757 = match v3.clone() {
                        Adt6::C0(h1755, _, _, _) => {
                            h1755
                        }
                    };
                    nat_lt(a1756, a1757)
                } else {
                    false
                } {
                    Ok(Adt9::C4(6u64, 0u64, f43(v1, {
                        let a1761 = {
                            let a1759 = f44(v6, v2, 1u64);
                            let a1760 = match v3 {
                                Adt6::C0(h1758, _, _, _) => {
                                    h1758
                                }
                            };
                            nat_mul(a1759, a1760)?
                        };
                        let a1762 = v11;
                        nat_add(a1761, a1762)?
                    }, v9, 0u64, 0u64)?, 2u64, 0u64, 1u64, 1u64))
                } else {
                    Ok(Adt9::C4(19u64, 2u64, 0u64, 2u64, 0u64, 1u64, 1u64))
                }
            }
        }
    }
}

pub fn f63(v0: u64, v1: Adt3, v2: Adt3, v3: Adt5) -> R<Adt9> {
    let v4: u64 = f44(v0, v2.clone(), 0u64);
    let v5: u64 = f44(v0, v2.clone(), 3u64);
    let v6: u64 = {
        let a1764 = v5;
        let a1765 = match v3.clone() {
            Adt5::C0(_, _, _, _, _, _, h1763, _) => {
                h1763
            }
        };
        let a1766 = 0u64;
        nat_rem(a1764, a1765, a1766)
    };
    let v7: u64 = {
        let a1768 = v5;
        let a1769 = match v3.clone() {
            Adt5::C0(_, _, _, _, _, _, h1767, _) => {
                h1767
            }
        };
        let a1770 = 0u64;
        nat_quot(a1768, a1769, a1770)
    };
    let v8: Option<u64> = f46(match v3.clone() {
        Adt5::C0(h1771, _, _, _, _, _, _, _) => {
            h1771
        }
    }, match v1.clone() {
        Adt3::C0(h1772, _, _, _, _, _, _, _) => {
            h1772
        }
    }, f44(v0, v2.clone(), 1u64), {
        let a1776 = v4;
        let a1777 = {
            let m1775 = match v3.clone() {
                Adt5::C0(h1774, _, _, _, _, _, _, _) => {
                    h1774
                }
            };
            match m1775 {
                Adt6::C0(h1773, _, _, _) => {
                    h1773
                }
            }
        };
        let a1778 = 0u64;
        nat_rem(a1776, a1777, a1778)
    })?;
    let v9: u64 = {
        let a1782 = v4;
        let a1783 = {
            let m1781 = match v3.clone() {
                Adt5::C0(h1780, _, _, _, _, _, _, _) => {
                    h1780
                }
            };
            match m1781 {
                Adt6::C0(h1779, _, _, _) => {
                    h1779
                }
            }
        };
        let a1784 = 0u64;
        nat_quot(a1782, a1783, a1784)
    };
    let v10: Option<u64> = f46(match v3.clone() {
        Adt5::C0(_, h1785, _, _, _, _, _, _) => {
            h1785
        }
    }, match v1.clone() {
        Adt3::C0(_, h1786, _, _, _, _, _, _) => {
            h1786
        }
    }, f44(v0, v2.clone(), 2u64), {
        let a1790 = v9;
        let a1791 = {
            let m1789 = match v3.clone() {
                Adt5::C0(_, h1788, _, _, _, _, _, _) => {
                    h1788
                }
            };
            match m1789 {
                Adt6::C0(h1787, _, _, _) => {
                    h1787
                }
            }
        };
        let a1792 = 0u64;
        nat_rem(a1790, a1791, a1792)
    })?;
    let v11: u64 = {
        let a1796 = v9;
        let a1797 = {
            let m1795 = match v3.clone() {
                Adt5::C0(_, h1794, _, _, _, _, _, _) => {
                    h1794
                }
            };
            match m1795 {
                Adt6::C0(h1793, _, _, _) => {
                    h1793
                }
            }
        };
        let a1798 = 0u64;
        nat_quot(a1796, a1797, a1798)
    };
    let v12: Option<u64> = f46(match v3.clone() {
        Adt5::C0(_, _, h1799, _, _, _, _, _) => {
            h1799
        }
    }, match v1.clone() {
        Adt3::C0(_, _, h1800, _, _, _, _, _) => {
            h1800
        }
    }, v6, {
        let a1804 = v11;
        let a1805 = {
            let m1803 = match v3.clone() {
                Adt5::C0(_, _, h1802, _, _, _, _, _) => {
                    h1802
                }
            };
            match m1803 {
                Adt6::C0(h1801, _, _, _) => {
                    h1801
                }
            }
        };
        let a1806 = 0u64;
        nat_rem(a1804, a1805, a1806)
    })?;
    let v13: u64 = {
        let a1810 = v11;
        let a1811 = {
            let m1809 = match v3.clone() {
                Adt5::C0(_, _, h1808, _, _, _, _, _) => {
                    h1808
                }
            };
            match m1809 {
                Adt6::C0(h1807, _, _, _) => {
                    h1807
                }
            }
        };
        let a1812 = 0u64;
        nat_quot(a1810, a1811, a1812)
    };
    let v17: Adt12 = {
        let m1813 = v8;
        match m1813 {
            None => {
                Adt12::C2(0u64)
            }
            Some(v14) => {
                let m1814 = v10;
                match m1814 {
                    None => {
                        Adt12::C2(0u64)
                    }
                    Some(v15) => {
                        let m1815 = v12;
                        match m1815 {
                            None => {
                                Adt12::C2(0u64)
                            }
                            Some(v16) => {
                                Adt12::C0(1u64, {
                                    let a1824 = {
                                        let a1819 = f43(v1, v14, v15, v16, 0u64)?;
                                        let a1820 = {
                                            let a1817 = v7;
                                            let a1818 = match v3.clone() {
                                                Adt5::C0(_, _, _, _, h1816, _, _, _) => {
                                                    h1816
                                                }
                                            };
                                            nat_mul(a1817, a1818)?
                                        };
                                        nat_add(a1819, a1820)?
                                    };
                                    let a1825 = {
                                        let a1822 = v13;
                                        let a1823 = match v3.clone() {
                                            Adt5::C0(_, _, _, h1821, _, _, _, _) => {
                                                h1821
                                            }
                                        };
                                        nat_mul(a1822, a1823)?
                                    };
                                    nat_add(a1824, a1825)?
                                })
                            }
                        }
                    }
                }
            }
        }
    };
    Ok(Adt9::C7(Adt11::C2(match v3.clone() {
        Adt5::C0(_, _, _, _, _, h1826, _, _) => {
            h1826
        }
    }, match v3.clone() {
        Adt5::C0(_, _, _, _, _, _, _, h1827) => {
            h1827
        }
    }, true), v17, Adt12::C2(0u64), Adt13::C1(f125(v0, v2)?), {
        let a1829 = match v3.clone() {
            Adt5::C0(_, _, _, _, _, h1828, _, _) => {
                h1828
            }
        };
        let a1830 = 0u64;
        if nat_eq(a1829, a1830) {
            4u64
        } else {
            2u64
        }
    }, {
        let a1832 = match v3 {
            Adt5::C0(_, _, _, _, _, _, _, h1831) => {
                h1831
            }
        };
        let a1833 = 0u64;
        if nat_eq(a1832, a1833) {
            4u64
        } else {
            2u64
        }
    }))
}

pub fn f64(v0: Adt16, v1: u64, v2: Adt3, v3: Adt3, v4: u64, v5: u64, v6: u64, v7: u64, v8: u64, v9: u64) -> R<Adt9> {
    let m1846 = f164(v0, v1, match v2.clone() {
        Adt3::C0(h1834, _, _, _, _, _, _, _) => {
            h1834
        }
    }, match v2.clone() {
        Adt3::C0(_, h1835, _, _, _, _, _, _) => {
            h1835
        }
    }, match v2.clone() {
        Adt3::C0(_, _, h1836, _, _, _, _, _) => {
            h1836
        }
    }, match v2.clone() {
        Adt3::C0(_, _, _, h1837, _, _, _, _) => {
            h1837
        }
    }, match v2.clone() {
        Adt3::C0(_, _, _, _, h1838, _, _, _) => {
            h1838
        }
    }, match v2.clone() {
        Adt3::C0(_, _, _, _, _, h1839, _, _) => {
            h1839
        }
    }, match v2.clone() {
        Adt3::C0(_, _, _, _, _, _, h1840, _) => {
            h1840
        }
    }, match v2 {
        Adt3::C0(_, _, _, _, _, _, _, h1841) => {
            h1841
        }
    }, v6, v7, v8, v9, match v3.clone() {
        Adt3::C0(_, _, _, _, h1842, _, _, _) => {
            h1842
        }
    }, match v3.clone() {
        Adt3::C0(_, _, _, _, _, h1843, _, _) => {
            h1843
        }
    }, match v3.clone() {
        Adt3::C0(_, _, _, _, _, _, h1844, _) => {
            h1844
        }
    }, match v3 {
        Adt3::C0(_, _, _, _, _, _, _, h1845) => {
            h1845
        }
    }, v4)?;
    match m1846 {
        Adt17::C0 => {
            Ok(Adt9::C0(Adt10::C1))
        }
        Adt17::C1(v10, v11, v12, v13, v14, v15, v16, v17) => {
            Ok(f15(false, v10, v11, 0u64, 0u64, v12, v13, v14, v15, v16, v17, v5, v5))
        }
    }
}

pub fn f65(v0: u64, v1: Adt3, v2: Adt3, v3: u64, v4: u64, v5: u64, v6: u64, v7: u64) -> R<Adt9> {
    let v8: u64 = {
        let a1849 = v6;
        let a1850 = {
            let a1847 = 3u64;
            let a1848 = v5;
            nat_add(a1847, a1848)?
        };
        nat_sub(a1849, a1850)
    };
    let a1852 = v0;
    let a1853 = f36(v3, match v1.clone() {
        Adt3::C0(h1851, _, _, _, _, _, _, _) => {
            h1851
        }
    }, v5, v8)?;
    if nat_lt(a1852, a1853) {
        let v9: u64 = f79(match v1.clone() {
            Adt3::C0(h1854, _, _, _, _, _, _, _) => {
                h1854
            }
        }, v5, v8)?;
        let v10: u64 = f80(v3, match v1.clone() {
            Adt3::C0(h1855, _, _, _, _, _, _, _) => {
                h1855
            }
        }, v5)?;
        let a1856 = v0;
        let a1857 = v9;
        if nat_lt(a1856, a1857) {
            f165(v0, v2, f30(v3), {
                let a1858 = 3u64;
                let a1859 = v5;
                nat_add(a1858, a1859)?
            }, v7)
        } else {
            let a1862 = v0;
            let a1863 = {
                let a1860 = v9;
                let a1861 = v10;
                nat_add(a1860, a1861)?
            };
            if nat_lt(a1862, a1863) {
                let v11: u64 = {
                    let a1864 = v0;
                    let a1865 = v9;
                    nat_sub(a1864, a1865)
                };
                let v12: u64 = f166(v3, match v1.clone() {
                    Adt3::C0(h1866, _, _, _, _, _, _, _) => {
                        h1866
                    }
                })?;
                f167({
                    let a1867 = v11;
                    let a1868 = v12;
                    let a1869 = 0u64;
                    nat_rem(a1867, a1868, a1869)
                }, {
                    let a1870 = v11;
                    let a1871 = v12;
                    let a1872 = 0u64;
                    nat_quot(a1870, a1871, a1872)
                }, v1, v3, v4, v7)
            } else {
                f168({
                    let a1875 = v0;
                    let a1876 = {
                        let a1873 = v9;
                        let a1874 = v10;
                        nat_add(a1873, a1874)?
                    };
                    nat_sub(a1875, a1876)
                }, v1, v2, v3)
            }
        }
    } else {
        Ok(Adt9::C0(Adt10::C1))
    }
}

pub fn f66(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64) -> R<Adt9> {
    let a1877 = v0;
    let a1878 = f19(v1)?;
    if nat_lt(a1877, a1878) {
        let a1879 = v0;
        let a1880 = 4u64;
        if nat_lt(a1879, a1880) {
            let a1881 = v0;
            let a1882 = 0u64;
            if nat_eq(a1881, a1882) {
                Ok(Adt9::C4(16u64, 2u64, v2, 2u64, v3, 1u64, 2u64))
            } else {
                let a1883 = v0;
                let a1884 = 1u64;
                if nat_eq(a1883, a1884) {
                    let a1885 = v5;
                    let a1886 = 0u64;
                    if nat_eq(a1885, a1886) {
                        Ok(Adt9::C4(9u64, 2u64, v3, 2u64, v2, 1u64, 2u64))
                    } else {
                        Ok(Adt9::C0(Adt10::C1))
                    }
                } else {
                    let a1887 = v0;
                    let a1888 = 2u64;
                    if nat_eq(a1887, a1888) {
                        Ok(Adt9::C4(11u64, 1u64, 2u64, 2u64, v4, 1u64, 2u64))
                    } else {
                        Ok(Adt9::C4(20u64, 1u64, 2u64, 2u64, 10u64, 1u64, 2u64))
                    }
                }
            }
        } else {
            let a1889 = v0;
            let a1890 = 4u64;
            if if nat_eq(a1889, a1890) {
                let a1891 = f169(v5)?;
                let a1892 = v1;
                !nat_eq(a1891, a1892)
            } else {
                false
            } {
                Ok(Adt9::C0(Adt10::C1))
            } else {
                let v6: u64 = {
                    let a1895 = {
                        let a1893 = v0;
                        let a1894 = 4u64;
                        nat_sub(a1893, a1894)
                    };
                    let a1896 = 3u64;
                    let a1897 = 0u64;
                    nat_quot(a1895, a1896, a1897)
                };
                let v7: u64 = {
                    let a1900 = {
                        let a1898 = v0;
                        let a1899 = 4u64;
                        nat_sub(a1898, a1899)
                    };
                    let a1901 = 3u64;
                    let a1902 = 0u64;
                    nat_rem(a1900, a1901, a1902)
                };
                let a1903 = v7;
                let a1904 = 0u64;
                if nat_eq(a1903, a1904) {
                    Ok(Adt9::C4(3u64, 2u64, v6, 2u64, 0u64, 1u64, 0u64))
                } else {
                    let a1905 = v7;
                    let a1906 = 1u64;
                    if nat_eq(a1905, a1906) {
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

pub fn f67(v0: bool, v1: u64, v2: u64, v3: u64, v4: u64) -> bool {
    if if if if v0 {
        let a1907 = v1;
        let a1908 = 5u64;
        nat_eq(a1907, a1908)
    } else {
        false
    } {
        let a1909 = v2;
        let a1910 = 16u64;
        nat_eq(a1909, a1910)
    } else {
        false
    } {
        let a1911 = v3;
        let a1912 = 0u64;
        nat_eq(a1911, a1912)
    } else {
        false
    } {
        let a1913 = v4;
        let a1914 = 0u64;
        nat_eq(a1913, a1914)
    } else {
        false
    }
}

pub fn f68(v0: bool, v1: u64, v2: u64, v3: u64, v4: u64) -> Adt11 {
    let a1915 = v1;
    let a1916 = 0u64;
    if nat_eq(a1915, a1916) {
        if v0 {
            let a1917 = v3;
            let a1918 = 0u64;
            if if if nat_eq(a1917, a1918) {
                let a1919 = v4;
                let a1920 = 43u64;
                nat_eq(a1919, a1920)
            } else {
                false
            } {
                true
            } else {
                let a1921 = v3;
                let a1922 = 43u64;
                if nat_eq(a1921, a1922) {
                    let a1923 = v4;
                    let a1924 = 0u64;
                    nat_eq(a1923, a1924)
                } else {
                    false
                }
            } {
                f143(v3, v4)
            } else {
                Adt11::C2(v3, v4, false)
            }
        } else {
            Adt11::C0(v2)
        }
    } else {
        let a1925 = v1;
        let a1926 = 1u64;
        if nat_eq(a1925, a1926) {
            Adt11::C1(v2)
        } else {
            let a1927 = v1;
            let a1928 = 2u64;
            if nat_eq(a1927, a1928) {
                Adt11::C5(v2)
            } else {
                let a1929 = v1;
                let a1930 = 3u64;
                if nat_eq(a1929, a1930) {
                    Adt11::C3(v2, v2, false)
                } else {
                    let a1931 = v1;
                    let a1932 = 4u64;
                    if nat_eq(a1931, a1932) {
                        Adt11::C2(v3, v4, false)
                    } else {
                        if f67(v0, v1, v2, v3, v4) {
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

pub fn f69(v0: u64, v1: u64) -> Adt12 {
    let a1933 = v0;
    let a1934 = 0u64;
    if nat_eq(a1933, a1934) {
        Adt12::C0(0u64, v1)
    } else {
        let a1935 = v0;
        let a1936 = 1u64;
        if nat_eq(a1935, a1936) {
            Adt12::C1(v1)
        } else {
            let a1937 = v0;
            let a1938 = 2u64;
            if nat_eq(a1937, a1938) {
                Adt12::C2(v1)
            } else {
                let a1939 = v0;
                let a1940 = 3u64;
                if nat_eq(a1939, a1940) {
                    Adt12::C3(v1)
                } else {
                    let a1941 = v0;
                    let a1942 = 13u64;
                    if nat_lt(a1941, a1942) {
                        Adt12::C0({
                            let a1943 = v0;
                            let a1944 = 3u64;
                            nat_sub(a1943, a1944)
                        }, v1)
                    } else {
                        Adt12::C4
                    }
                }
            }
        }
    }
}

pub fn f70(v0: u64, v1: u64) -> Adt13 {
    let a1945 = v0;
    let a1946 = 1u64;
    if nat_eq(a1945, a1946) {
        Adt13::C0(v1)
    } else {
        let a1947 = v0;
        let a1948 = 3u64;
        if if nat_eq(a1947, a1948) {
            true
        } else {
            let a1949 = v0;
            let a1950 = 6u64;
            nat_eq(a1949, a1950)
        } {
            Adt13::C1(v1)
        } else {
            Adt13::C2
        }
    }
}

pub fn f71(v0: Adt6, v1: u64) -> R<u64> {
    let a1952 = 0u64;
    let a1953 = match v0.clone() {
        Adt6::C0(h1951, _, _, _) => {
            h1951
        }
    };
    if if if if if nat_lt(a1952, a1953) {
        let a1955 = 0u64;
        let a1956 = match v0.clone() {
            Adt6::C0(_, h1954, _, _) => {
                h1954
            }
        };
        nat_lt(a1955, a1956)
    } else {
        false
    } {
        let a1958 = 0u64;
        let a1959 = match v0.clone() {
            Adt6::C0(_, _, _, h1957) => {
                h1957
            }
        };
        nat_lt(a1958, a1959)
    } else {
        false
    } {
        let a1961 = match v0.clone() {
            Adt6::C0(_, h1960, _, _) => {
                h1960
            }
        };
        let a1962 = 2147483648u64;
        nat_lt(a1961, a1962)
    } else {
        false
    } {
        let a1964 = match v0.clone() {
            Adt6::C0(_, _, _, h1963) => {
                h1963
            }
        };
        let a1965 = 2147483648u64;
        nat_lt(a1964, a1965)
    } else {
        false
    } {
        let v2: u64 = {
            let a1969 = v1;
            let a1970 = {
                let a1967 = 2u64;
                let a1968 = f119(match v0.clone() {
                    Adt6::C0(_, _, h1966, _) => {
                        h1966
                    }
                });
                nat_mul(a1967, a1968)?
            };
            nat_add(a1969, a1970)?
        };
        let v3: u64 = {
            let a1982 = {
                let a1977 = 1u64;
                let a1978 = {
                    let a1975 = match v0.clone() {
                        Adt6::C0(_, _, _, h1971) => {
                            h1971
                        }
                    };
                    let a1976 = {
                        let a1973 = match v0.clone() {
                            Adt6::C0(h1972, _, _, _) => {
                                h1972
                            }
                        };
                        let a1974 = 1u64;
                        nat_sub(a1973, a1974)
                    };
                    nat_mul(a1975, a1976)?
                };
                nat_add(a1977, a1978)?
            };
            let a1983 = {
                let a1980 = 2u64;
                let a1981 = f118(match v0.clone() {
                    Adt6::C0(_, _, h1979, _) => {
                        h1979
                    }
                });
                nat_mul(a1980, a1981)?
            };
            nat_add(a1982, a1983)?
        };
        let a1984 = v3;
        let a1985 = v2;
        if nat_le(a1984, a1985) {
            let a1992 = 1u64;
            let a1993 = {
                let a1989 = {
                    let a1986 = v2;
                    let a1987 = v3;
                    nat_sub(a1986, a1987)
                };
                let a1990 = match v0 {
                    Adt6::C0(_, h1988, _, _) => {
                        h1988
                    }
                };
                let a1991 = 0u64;
                nat_quot(a1989, a1990, a1991)
            };
            nat_add(a1992, a1993)
        } else {
            let a1997 = {
                let a1994 = v3;
                let a1995 = v2;
                nat_sub(a1994, a1995)
            };
            let a1998 = match v0 {
                Adt6::C0(_, h1996, _, _) => {
                    h1996
                }
            };
            if nat_lt(a1997, a1998) {
                Ok(1u64)
            } else {
                Ok(0u64)
            }
        }
    } else {
        Ok(0u64)
    }
}

pub fn f72(v0: u64) -> u64 {
    let v1: u64 = {
        let a1999 = v0;
        let a2000 = 4294967296u64;
        let a2001 = 0u64;
        nat_rem(a1999, a2000, a2001)
    };
    let a2002 = v1;
    let a2003 = 2147483648u64;
    if nat_lt(a2002, a2003) {
        v1
    } else {
        0u64
    }
}

pub fn f73(v0: u64) -> R<u64> {
    let a2006 = 2u64;
    let a2007 = {
        let a2004 = 2u64;
        let a2005 = v0;
        nat_mul(a2004, a2005)?
    };
    nat_add(a2006, a2007)
}

pub fn f74(v0: u64, v1: Adt14, v2: Adt14) -> R<Adt15> {
    let a2009 = match v1.clone() {
        Adt14::C0(h2008, _, _, _, _, _, _, _, _, _, _) => {
            h2008
        }
    };
    let a2010 = 0u64;
    if if if if nat_eq(a2009, a2010) {
        let a2012 = match v2.clone() {
            Adt14::C0(h2011, _, _, _, _, _, _, _, _, _, _) => {
                h2011
            }
        };
        let a2013 = 0u64;
        nat_eq(a2012, a2013)
    } else {
        false
    } {
        f170(v1.clone(), v2.clone())
    } else {
        false
    } {
        if f171(match v1.clone() {
            Adt14::C0(_, h2014, _, _, _, _, _, _, _, _, _) => {
                h2014
            }
        }, match v1.clone() {
            Adt14::C0(_, _, h2015, _, _, _, _, _, _, _, _) => {
                h2015
            }
        }, match v1.clone() {
            Adt14::C0(_, _, _, h2016, _, _, _, _, _, _, _) => {
                h2016
            }
        }, match v1.clone() {
            Adt14::C0(_, _, _, _, h2017, _, _, _, _, _, _) => {
                h2017
            }
        }) {
            true
        } else {
            if if if if f31(v0) {
                f172(v1.clone(), v2.clone())
            } else {
                false
            } {
                f173(v1.clone())
            } else {
                false
            } {
                f173(v2.clone())
            } else {
                false
            } {
                if f174(v1.clone(), v2.clone())? {
                    true
                } else {
                    f175(v1.clone(), v2.clone())
                }
            } else {
                false
            }
        }
    } else {
        false
    } {
        let v3: u64 = if f171(match v1.clone() {
            Adt14::C0(_, h2018, _, _, _, _, _, _, _, _, _) => {
                h2018
            }
        }, match v1.clone() {
            Adt14::C0(_, _, h2019, _, _, _, _, _, _, _, _) => {
                h2019
            }
        }, match v1.clone() {
            Adt14::C0(_, _, _, h2020, _, _, _, _, _, _, _) => {
                h2020
            }
        }, match v1.clone() {
            Adt14::C0(_, _, _, _, h2021, _, _, _, _, _, _) => {
                h2021
            }
        }) {
            0u64
        } else {
            let a2027 = match v1.clone() {
                Adt14::C0(_, _, h2022, _, _, _, _, _, _, _, _) => {
                    h2022
                }
            };
            let a2028 = {
                let a2025 = match v1.clone() {
                    Adt14::C0(_, _, _, h2023, _, _, _, _, _, _, _) => {
                        h2023
                    }
                };
                let a2026 = match v1.clone() {
                    Adt14::C0(_, _, _, _, h2024, _, _, _, _, _, _) => {
                        h2024
                    }
                };
                nat_mul(a2025, a2026)?
            };
            nat_mul(a2027, a2028)?
        };
        let v4: u64 = {
            let a2029 = v3;
            let a2030 = 0u64;
            if if nat_eq(a2029, a2030) {
                true
            } else {
                f176(v2.clone())?
            } {
                0u64
            } else {
                let a2032 = match v2.clone() {
                    Adt14::C0(_, _, _, _, _, _, _, _, _, h2031, _) => {
                        h2031
                    }
                };
                let a2033 = 4u64;
                let a2034 = 0u64;
                nat_quot(a2032, a2033, a2034)
            }
        };
        Ok(Adt15::C1(f177(f178(match v1.clone() {
            Adt14::C0(_, _, _, _, _, h2035, _, _, _, _, _) => {
                h2035
            }
        }, match v2 {
            Adt14::C0(_, _, _, _, _, h2036, _, _, _, _, _) => {
                h2036
            }
        }), match v1 {
            Adt14::C0(_, h2037, _, _, _, _, _, _, _, _, _) => {
                h2037
            }
        }, v3, v4)?, 4u64, 64u64, v3, f179(v4)?, v0))
    } else {
        Ok(Adt15::C0)
    }
}

pub fn f75(v0: u64, v1: u64, v2: Adt14, v3: Adt14) -> R<Adt15> {
    let v4: Adt14 = f180(v3.clone());
    let a2039 = match v2.clone() {
        Adt14::C0(h2038, _, _, _, _, _, _, _, _, _, _) => {
            h2038
        }
    };
    let a2040 = 0u64;
    if if if if nat_eq(a2039, a2040) {
        let a2042 = match v3.clone() {
            Adt14::C0(h2041, _, _, _, _, _, _, _, _, _, _) => {
                h2041
            }
        };
        let a2043 = 0u64;
        nat_eq(a2042, a2043)
    } else {
        false
    } {
        f170(v2.clone(), v3)
    } else {
        false
    } {
        if f171(match v2.clone() {
            Adt14::C0(_, h2044, _, _, _, _, _, _, _, _, _) => {
                h2044
            }
        }, match v2.clone() {
            Adt14::C0(_, _, h2045, _, _, _, _, _, _, _, _) => {
                h2045
            }
        }, match v2.clone() {
            Adt14::C0(_, _, _, h2046, _, _, _, _, _, _, _) => {
                h2046
            }
        }, match v2.clone() {
            Adt14::C0(_, _, _, _, h2047, _, _, _, _, _, _) => {
                h2047
            }
        }) {
            true
        } else {
            let a2049 = match v2.clone() {
                Adt14::C0(_, _, _, _, _, h2048, _, _, _, _, _) => {
                    h2048
                }
            };
            let a2050 = 4u64;
            if if if if if nat_eq(a2049, a2050) {
                f181(match v2.clone() {
                    Adt14::C0(_, h2051, _, _, _, _, _, _, _, _, _) => {
                        h2051
                    }
                }, match v2.clone() {
                    Adt14::C0(_, _, h2052, _, _, _, _, _, _, _, _) => {
                        h2052
                    }
                }, match v2.clone() {
                    Adt14::C0(_, _, _, h2053, _, _, _, _, _, _, _) => {
                        h2053
                    }
                }, v0)?
            } else {
                false
            } {
                f173(v2.clone())
            } else {
                false
            } {
                f173(v4.clone())
            } else {
                false
            } {
                let a2055 = f182(v4.clone())?;
                let a2056 = match v4.clone() {
                    Adt14::C0(_, _, _, _, _, _, _, _, _, h2054, _) => {
                        h2054
                    }
                };
                nat_le(a2055, a2056)
            } else {
                false
            } {
                if f174(v2.clone(), v4.clone())? {
                    true
                } else {
                    f175(v2.clone(), v4.clone())
                }
            } else {
                false
            }
        }
    } else {
        false
    } {
        let v5: u64 = if if f171(match v2.clone() {
            Adt14::C0(_, h2057, _, _, _, _, _, _, _, _, _) => {
                h2057
            }
        }, match v2.clone() {
            Adt14::C0(_, _, h2058, _, _, _, _, _, _, _, _) => {
                h2058
            }
        }, match v2.clone() {
            Adt14::C0(_, _, _, h2059, _, _, _, _, _, _, _) => {
                h2059
            }
        }, match v2.clone() {
            Adt14::C0(_, _, _, _, h2060, _, _, _, _, _, _) => {
                h2060
            }
        }) {
            true
        } else {
            f176(v4.clone())?
        } {
            0u64
        } else {
            let a2062 = match v4 {
                Adt14::C0(_, _, _, _, _, _, _, _, _, h2061, _) => {
                    h2061
                }
            };
            let a2063 = 4u64;
            let a2064 = 0u64;
            nat_quot(a2062, a2063, a2064)
        };
        Ok(Adt15::C1(f183(match v2.clone() {
            Adt14::C0(_, h2065, _, _, _, _, _, _, _, _, _) => {
                h2065
            }
        }, match v2.clone() {
            Adt14::C0(_, _, h2066, _, _, _, _, _, _, _, _) => {
                h2066
            }
        }, match v2.clone() {
            Adt14::C0(_, _, _, h2067, _, _, _, _, _, _, _) => {
                h2067
            }
        }, match v2 {
            Adt14::C0(_, _, _, _, h2068, _, _, _, _, _, _) => {
                h2068
            }
        }, v0, v5)?, 4u64, 65u64, v0, f179(v5)?, v1))
    } else {
        Ok(Adt15::C0)
    }
}

pub fn f76(v0: u64, v1: bool, v2: Adt14, v3: Adt14, v4: Adt14) -> R<Adt15> {
    let v5: Adt14 = f180(v4.clone());
    if if if if if if if v1 {
        let a2070 = match v2.clone() {
            Adt14::C0(h2069, _, _, _, _, _, _, _, _, _, _) => {
                h2069
            }
        };
        let a2071 = 0u64;
        nat_eq(a2070, a2071)
    } else {
        false
    } {
        let a2073 = match v3.clone() {
            Adt14::C0(h2072, _, _, _, _, _, _, _, _, _, _) => {
                h2072
            }
        };
        let a2074 = 0u64;
        nat_eq(a2073, a2074)
    } else {
        false
    } {
        let a2076 = match v4.clone() {
            Adt14::C0(h2075, _, _, _, _, _, _, _, _, _, _) => {
                h2075
            }
        };
        let a2077 = 0u64;
        nat_eq(a2076, a2077)
    } else {
        false
    } {
        f170(v2.clone(), v3.clone())
    } else {
        false
    } {
        f170(v2.clone(), v4)
    } else {
        false
    } {
        if f171(match v2.clone() {
            Adt14::C0(_, h2078, _, _, _, _, _, _, _, _, _) => {
                h2078
            }
        }, match v2.clone() {
            Adt14::C0(_, _, h2079, _, _, _, _, _, _, _, _) => {
                h2079
            }
        }, match v2.clone() {
            Adt14::C0(_, _, _, h2080, _, _, _, _, _, _, _) => {
                h2080
            }
        }, match v2.clone() {
            Adt14::C0(_, _, _, _, h2081, _, _, _, _, _, _) => {
                h2081
            }
        }) {
            true
        } else {
            let a2083 = match v2.clone() {
                Adt14::C0(_, _, _, _, _, h2082, _, _, _, _, _) => {
                    h2082
                }
            };
            let a2084 = 4u64;
            if if if if if if if nat_eq(a2083, a2084) {
                let a2086 = match v3.clone() {
                    Adt14::C0(_, _, _, _, _, h2085, _, _, _, _, _) => {
                        h2085
                    }
                };
                let a2087 = 4u64;
                nat_eq(a2086, a2087)
            } else {
                false
            } {
                f173(v2.clone())
            } else {
                false
            } {
                f173(v3.clone())
            } else {
                false
            } {
                f173(v5.clone())
            } else {
                false
            } {
                let a2089 = f182(v5.clone())?;
                let a2090 = match v5.clone() {
                    Adt14::C0(_, _, _, _, _, _, _, _, _, h2088, _) => {
                        h2088
                    }
                };
                nat_le(a2089, a2090)
            } else {
                false
            } {
                if f174(v2.clone(), v5.clone())? {
                    true
                } else {
                    f175(v2.clone(), v5.clone())
                }
            } else {
                false
            } {
                if f174(v3.clone(), v5.clone())? {
                    true
                } else {
                    f175(v3, v5.clone())
                }
            } else {
                false
            }
        }
    } else {
        false
    } {
        let v6: u64 = if f171(match v2.clone() {
            Adt14::C0(_, h2091, _, _, _, _, _, _, _, _, _) => {
                h2091
            }
        }, match v2.clone() {
            Adt14::C0(_, _, h2092, _, _, _, _, _, _, _, _) => {
                h2092
            }
        }, match v2.clone() {
            Adt14::C0(_, _, _, h2093, _, _, _, _, _, _, _) => {
                h2093
            }
        }, match v2.clone() {
            Adt14::C0(_, _, _, _, h2094, _, _, _, _, _, _) => {
                h2094
            }
        }) {
            0u64
        } else {
            let a2100 = match v2.clone() {
                Adt14::C0(_, _, h2095, _, _, _, _, _, _, _, _) => {
                    h2095
                }
            };
            let a2101 = {
                let a2098 = match v2.clone() {
                    Adt14::C0(_, _, _, h2096, _, _, _, _, _, _, _) => {
                        h2096
                    }
                };
                let a2099 = match v2.clone() {
                    Adt14::C0(_, _, _, _, h2097, _, _, _, _, _, _) => {
                        h2097
                    }
                };
                nat_mul(a2098, a2099)?
            };
            nat_mul(a2100, a2101)?
        };
        let v7: u64 = {
            let a2102 = v6;
            let a2103 = 0u64;
            if if nat_eq(a2102, a2103) {
                true
            } else {
                f176(v5.clone())?
            } {
                0u64
            } else {
                let a2105 = match v5 {
                    Adt14::C0(_, _, _, _, _, _, _, _, _, h2104, _) => {
                        h2104
                    }
                };
                let a2106 = 4u64;
                let a2107 = 0u64;
                nat_quot(a2105, a2106, a2107)
            }
        };
        Ok(Adt15::C1(f184(match v2 {
            Adt14::C0(_, h2108, _, _, _, _, _, _, _, _, _) => {
                h2108
            }
        }, v6, v7)?, 4u64, 66u64, v6, f179(v7)?, v0))
    } else {
        Ok(Adt15::C0)
    }
}

pub fn f77(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64) -> bool {
    if if f185(v0, v3, v6) {
        f185(v1, v4, v6)
    } else {
        false
    } {
        f185(v2, v5, v6)
    } else {
        false
    }
}

pub fn f78(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64) -> R<bool> {
    if if if if if f186(v0, v1, v4, v2, v5, v3, v6)? {
        true
    } else {
        f186(v0, v1, v4, v3, v6, v2, v5)?
    } {
        true
    } else {
        f186(v0, v2, v5, v1, v4, v3, v6)?
    } {
        true
    } else {
        f186(v0, v2, v5, v3, v6, v1, v4)?
    } {
        true
    } else {
        f186(v0, v3, v6, v1, v4, v2, v5)?
    } {
        Ok(true)
    } else {
        f186(v0, v3, v6, v2, v5, v1, v4)
    }
}

pub fn f79(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let a2109 = v2;
    let a2110 = 0u64;
    if nat_eq(a2109, a2110) {
        Ok(0u64)
    } else {
        let a2113 = 2u64;
        let a2114 = {
            let a2111 = v0;
            let a2112 = v1;
            nat_mul(a2111, a2112)?
        };
        nat_mul(a2113, a2114)
    }
}

pub fn f80(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let a2115 = v2;
    let a2116 = f166(v0, v1)?;
    nat_mul(a2115, a2116)
}

pub fn f81(v0: u64) -> u64 {
    let a2117 = v0;
    let a2118 = 30u64;
    if nat_eq(a2117, a2118) {
        3u64
    } else {
        1u64
    }
}

pub fn f82(v0: u64) -> R<u64> {
    let a2121 = 5u64;
    let a2122 = {
        let a2119 = 4u64;
        let a2120 = v0;
        nat_mul(a2119, a2120)?
    };
    nat_add(a2121, a2122)
}

pub fn f83(v0: u64) -> bool {
    let m2123 = f187(v0);
    match m2123 {
        Adt18::C0 => {
            false
        }
        Adt18::C1(v1) => {
            let a2125 = 1u64;
            let a2126 = match v1 {
                Adt19::C0(_, h2124, _) => {
                    h2124
                }
            };
            nat_lt(a2125, a2126)
        }
    }
}

pub fn f84(v0: Adt1, v1: Adt2, v2: Adt2, v3: Adt2) -> R<Adt10> {
    let v4: bool = {
        let a2128 = 0u64;
        let a2129 = match v0.clone() {
            Adt1::C0(_, h2127, _, _, _, _, _, _, _, _, _, _) => {
                h2127
            }
        };
        nat_lt(a2128, a2129)
    };
    let a2131 = match v1.clone() {
        Adt2::C0(h2130, _, _, _, _, _, _, _, _, _, _) => {
            h2130
        }
    };
    let a2132 = 0u64;
    if if if if if if if if if if if if if if if if nat_eq(a2131, a2132) {
        let a2134 = match v2.clone() {
            Adt2::C0(h2133, _, _, _, _, _, _, _, _, _, _) => {
                h2133
            }
        };
        let a2135 = 0u64;
        nat_eq(a2134, a2135)
    } else {
        false
    } {
        let a2137 = match v3.clone() {
            Adt2::C0(h2136, _, _, _, _, _, _, _, _, _, _) => {
                h2136
            }
        };
        let a2138 = 0u64;
        nat_eq(a2137, a2138)
    } else {
        false
    } {
        let a2140 = match v1.clone() {
            Adt2::C0(_, _, _, _, h2139, _, _, _, _, _, _) => {
                h2139
            }
        };
        let a2141 = 1u64;
        nat_eq(a2140, a2141)
    } else {
        false
    } {
        let a2143 = match v2.clone() {
            Adt2::C0(_, _, _, _, h2142, _, _, _, _, _, _) => {
                h2142
            }
        };
        let a2144 = 1u64;
        nat_eq(a2143, a2144)
    } else {
        false
    } {
        let a2146 = match v3.clone() {
            Adt2::C0(_, _, _, _, h2145, _, _, _, _, _, _) => {
                h2145
            }
        };
        let a2147 = 1u64;
        nat_eq(a2146, a2147)
    } else {
        false
    } {
        let a2149 = 0u64;
        let a2150 = match v1.clone() {
            Adt2::C0(_, _, h2148, _, _, _, _, _, _, _, _) => {
                h2148
            }
        };
        nat_lt(a2149, a2150)
    } else {
        false
    } {
        let a2152 = match v3.clone() {
            Adt2::C0(_, _, _, h2151, _, _, _, _, _, _, _) => {
                h2151
            }
        };
        let a2153 = 1u64;
        nat_eq(a2152, a2153)
    } else {
        false
    } {
        let a2156 = match v3.clone() {
            Adt2::C0(_, h2154, _, _, _, _, _, _, _, _, _) => {
                h2154
            }
        };
        let a2157 = match v1.clone() {
            Adt2::C0(_, h2155, _, _, _, _, _, _, _, _, _) => {
                h2155
            }
        };
        nat_eq(a2156, a2157)
    } else {
        false
    } {
        let a2160 = match v3.clone() {
            Adt2::C0(_, _, h2158, _, _, _, _, _, _, _, _) => {
                h2158
            }
        };
        let a2161 = match v1.clone() {
            Adt2::C0(_, _, _, h2159, _, _, _, _, _, _, _) => {
                h2159
            }
        };
        nat_eq(a2160, a2161)
    } else {
        false
    } {
        if v4 {
            f25(v1.clone(), v2.clone())
        } else {
            let a2164 = match v2.clone() {
                Adt2::C0(_, h2162, _, _, _, _, _, _, _, _, _) => {
                    h2162
                }
            };
            let a2165 = match v1.clone() {
                Adt2::C0(_, _, h2163, _, _, _, _, _, _, _, _) => {
                    h2163
                }
            };
            if if nat_eq(a2164, a2165) {
                let a2168 = match v2.clone() {
                    Adt2::C0(_, _, h2166, _, _, _, _, _, _, _, _) => {
                        h2166
                    }
                };
                let a2169 = match v1.clone() {
                    Adt2::C0(_, _, _, h2167, _, _, _, _, _, _, _) => {
                        h2167
                    }
                };
                nat_eq(a2168, a2169)
            } else {
                false
            } {
                let a2171 = match v2.clone() {
                    Adt2::C0(_, _, _, h2170, _, _, _, _, _, _, _) => {
                        h2170
                    }
                };
                let a2172 = 1u64;
                nat_eq(a2171, a2172)
            } else {
                false
            }
        }
    } else {
        false
    } {
        f188(v1.clone(), f16(v1.clone(), 1u64)?)?
    } else {
        false
    } {
        f188(v2.clone(), f16(v2.clone(), 1u64)?)?
    } else {
        false
    } {
        f189(v3.clone())?
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
            let a2174 = f16(v3.clone(), 1u64)?;
            let a2175 = f145(match v1.clone() {
                Adt2::C0(_, _, h2173, _, _, _, _, _, _, _, _) => {
                    h2173
                }
            }, v4)?;
            nat_mul(a2174, a2175)?
        }, 4u64, f20(v1, 1u64), f20(v3, 1u64), Adt4::C36(f20(v2, 1u64), v4, match v0 {
            Adt1::C0(h2176, _, _, _, _, _, _, _, _, _, _, _) => {
                h2176
            }
        })))
    } else {
        Ok(Adt10::C1)
    }
}

pub fn f85(v0: Adt1, v1: Adt2, v2: Adt2, v3: Adt2) -> R<Adt10> {
    let v4: Adt6 = Adt6::C0(match v1.clone() {
        Adt2::C0(_, h2177, _, _, _, _, _, _, _, _, _) => {
            h2177
        }
    }, match v0.clone() {
        Adt1::C0(h2178, _, _, _, _, _, _, _, _, _, _, _) => {
            h2178
        }
    }, match v0.clone() {
        Adt1::C0(_, _, h2179, _, _, _, _, _, _, _, _, _) => {
            h2179
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, _, h2180, _, _, _, _, _, _, _) => {
            h2180
        }
    });
    let v5: Adt6 = Adt6::C0(match v1.clone() {
        Adt2::C0(_, _, h2181, _, _, _, _, _, _, _, _) => {
            h2181
        }
    }, match v0.clone() {
        Adt1::C0(_, h2182, _, _, _, _, _, _, _, _, _, _) => {
            h2182
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, h2183, _, _, _, _, _, _, _, _) => {
            h2183
        }
    }, match v0 {
        Adt1::C0(_, _, _, _, _, h2184, _, _, _, _, _, _) => {
            h2184
        }
    });
    let v6: u64 = match v1.clone() {
        Adt2::C0(h2185, _, _, _, _, _, _, _, _, _, _) => {
            h2185
        }
    };
    let v7: bool = {
        let a2188 = match v2.clone() {
            Adt2::C0(_, _, _, _, _, _, _, h2186, _, _, _) => {
                h2186
            }
        };
        let a2189 = match v2.clone() {
            Adt2::C0(_, _, _, _, _, h2187, _, _, _, _, _) => {
                h2187
            }
        };
        nat_lt(a2188, a2189)
    };
    let a2190 = v6;
    let a2191 = 0u64;
    if if if if if if if if if if if if if if if nat_eq(a2190, a2191) {
        true
    } else {
        let a2192 = v6;
        let a2193 = 1u64;
        nat_eq(a2192, a2193)
    } {
        let a2195 = match v2.clone() {
            Adt2::C0(h2194, _, _, _, _, _, _, _, _, _, _) => {
                h2194
            }
        };
        let a2196 = 0u64;
        nat_eq(a2195, a2196)
    } else {
        false
    } {
        let a2198 = match v3.clone() {
            Adt2::C0(h2197, _, _, _, _, _, _, _, _, _, _) => {
                h2197
            }
        };
        let a2199 = 0u64;
        nat_eq(a2198, a2199)
    } else {
        false
    } {
        let a2201 = match v1.clone() {
            Adt2::C0(_, _, _, h2200, _, _, _, _, _, _, _) => {
                h2200
            }
        };
        let a2202 = 1u64;
        nat_eq(a2201, a2202)
    } else {
        false
    } {
        let a2205 = match v1.clone() {
            Adt2::C0(_, _, _, _, h2203, _, _, _, _, _, _) => {
                h2203
            }
        };
        let a2206 = match v2.clone() {
            Adt2::C0(_, _, _, h2204, _, _, _, _, _, _, _) => {
                h2204
            }
        };
        nat_eq(a2205, a2206)
    } else {
        false
    } {
        let a2209 = match v3.clone() {
            Adt2::C0(_, _, _, h2207, _, _, _, _, _, _, _) => {
                h2207
            }
        };
        let a2210 = match v2.clone() {
            Adt2::C0(_, _, _, h2208, _, _, _, _, _, _, _) => {
                h2208
            }
        };
        nat_eq(a2209, a2210)
    } else {
        false
    } {
        let a2213 = match v3.clone() {
            Adt2::C0(_, _, _, _, h2211, _, _, _, _, _, _) => {
                h2211
            }
        };
        let a2214 = match v2.clone() {
            Adt2::C0(_, _, _, _, h2212, _, _, _, _, _, _) => {
                h2212
            }
        };
        nat_eq(a2213, a2214)
    } else {
        false
    } {
        let a2216 = 0u64;
        let a2217 = match v4.clone() {
            Adt6::C0(h2215, _, _, _) => {
                h2215
            }
        };
        if if if if if nat_lt(a2216, a2217) {
            let a2219 = 0u64;
            let a2220 = match v4.clone() {
                Adt6::C0(_, h2218, _, _) => {
                    h2218
                }
            };
            nat_lt(a2219, a2220)
        } else {
            false
        } {
            let a2222 = match v4.clone() {
                Adt6::C0(_, h2221, _, _) => {
                    h2221
                }
            };
            let a2223 = 2147483648u64;
            nat_lt(a2222, a2223)
        } else {
            false
        } {
            let a2225 = 0u64;
            let a2226 = match v4.clone() {
                Adt6::C0(_, _, _, h2224) => {
                    h2224
                }
            };
            nat_lt(a2225, a2226)
        } else {
            false
        } {
            let a2228 = match v4.clone() {
                Adt6::C0(_, _, _, h2227) => {
                    h2227
                }
            };
            let a2229 = 2147483648u64;
            nat_lt(a2228, a2229)
        } else {
            false
        } {
            let a2232 = match v3.clone() {
                Adt2::C0(_, h2230, _, _, _, _, _, _, _, _, _) => {
                    h2230
                }
            };
            let a2233 = f71(v4.clone(), match v2.clone() {
                Adt2::C0(_, h2231, _, _, _, _, _, _, _, _, _) => {
                    h2231
                }
            })?;
            nat_eq(a2232, a2233)
        } else {
            false
        }
    } else {
        false
    } {
        let a2235 = 0u64;
        let a2236 = match v5.clone() {
            Adt6::C0(h2234, _, _, _) => {
                h2234
            }
        };
        if if if if if nat_lt(a2235, a2236) {
            let a2238 = 0u64;
            let a2239 = match v5.clone() {
                Adt6::C0(_, h2237, _, _) => {
                    h2237
                }
            };
            nat_lt(a2238, a2239)
        } else {
            false
        } {
            let a2241 = match v5.clone() {
                Adt6::C0(_, h2240, _, _) => {
                    h2240
                }
            };
            let a2242 = 2147483648u64;
            nat_lt(a2241, a2242)
        } else {
            false
        } {
            let a2244 = 0u64;
            let a2245 = match v5.clone() {
                Adt6::C0(_, _, _, h2243) => {
                    h2243
                }
            };
            nat_lt(a2244, a2245)
        } else {
            false
        } {
            let a2247 = match v5.clone() {
                Adt6::C0(_, _, _, h2246) => {
                    h2246
                }
            };
            let a2248 = 2147483648u64;
            nat_lt(a2247, a2248)
        } else {
            false
        } {
            let a2251 = match v3.clone() {
                Adt2::C0(_, _, h2249, _, _, _, _, _, _, _, _) => {
                    h2249
                }
            };
            let a2252 = f71(v5.clone(), match v2.clone() {
                Adt2::C0(_, _, h2250, _, _, _, _, _, _, _, _) => {
                    h2250
                }
            })?;
            nat_eq(a2251, a2252)
        } else {
            false
        }
    } else {
        false
    } {
        f190(v2.clone(), 4u64, v7, false)?
    } else {
        false
    } {
        f190(v3.clone(), 4u64, v7, false)?
    } else {
        false
    } {
        f190(v1.clone(), {
            let a2253 = v6;
            let a2254 = 0u64;
            if nat_eq(a2253, a2254) {
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
            let a2263 = {
                let a2261 = {
                    let a2259 = 3u64;
                    let a2260 = {
                        let a2257 = match v4.clone() {
                            Adt6::C0(h2255, _, _, _) => {
                                h2255
                            }
                        };
                        let a2258 = match v5.clone() {
                            Adt6::C0(h2256, _, _, _) => {
                                h2256
                            }
                        };
                        nat_mul(a2257, a2258)?
                    };
                    nat_mul(a2259, a2260)?
                };
                let a2262 = 2u64;
                nat_add(a2261, a2262)?
            };
            let a2264 = f16(v3.clone(), 1u64)?;
            nat_mul(a2263, a2264)?
        }, 4u64, f20(v1, 1u64), f20(v3, 1u64), Adt4::C34(f20(v2, 1u64), v4, v5, v6)))
    } else {
        Ok(Adt10::C1)
    }
}

pub fn f86(v0: Adt2, v1: Adt2, v2: Adt2) -> R<Adt10> {
    if if if if if if if f25(v0.clone(), v1.clone()) {
        f25(v0.clone(), v2.clone())
    } else {
        false
    } {
        f189(v0.clone())?
    } else {
        false
    } {
        f189(v1.clone())?
    } else {
        false
    } {
        f189(v2.clone())?
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
            let a2265 = 6u64;
            let a2266 = f16(v0.clone(), 1u64)?;
            nat_mul(a2265, a2266)?
        }, 2u64, f20(v0, 1u64), f20(v2, 1u64), Adt4::C33(f20(v1, 1u64))))
    } else {
        Ok(Adt10::C1)
    }
}

pub fn f87(v0: Adt1, v1: Adt2, v2: Option<Adt2>, v3: Adt2) -> R<Adt10> {
    let m2267 = v2;
    match m2267 {
        None => {
            Ok(Adt10::C1)
        }
        Some(v4) => {
            let v5: u64 = match v1.clone() {
                Adt2::C0(_, h2268, _, _, _, _, _, _, _, _, _) => {
                    h2268
                }
            };
            let v6: u64 = {
                let a2274 = match v1.clone() {
                    Adt2::C0(_, _, h2269, _, _, _, _, _, _, _, _) => {
                        h2269
                    }
                };
                let a2275 = {
                    let a2272 = match v1.clone() {
                        Adt2::C0(_, _, _, h2270, _, _, _, _, _, _, _) => {
                            h2270
                        }
                    };
                    let a2273 = match v1.clone() {
                        Adt2::C0(_, _, _, _, h2271, _, _, _, _, _, _) => {
                            h2271
                        }
                    };
                    nat_mul(a2272, a2273)?
                };
                nat_mul(a2274, a2275)?
            };
            if if if if if if if if if if f25(v1.clone(), v4.clone()) {
                f25(v1.clone(), v3.clone())
            } else {
                false
            } {
                f191(v1.clone(), v6)?
            } else {
                false
            } {
                f191(v4.clone(), v6)?
            } else {
                false
            } {
                f191(v3.clone(), v6)?
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
                let a2276 = v5;
                let a2277 = 2147483647u64;
                nat_le(a2276, a2277)
            } else {
                false
            } {
                let a2278 = v6;
                let a2279 = 2147483647u64;
                nat_le(a2278, a2279)
            } else {
                false
            } {
                let a2281 = match v0.clone() {
                    Adt1::C0(h2280, _, _, _, _, _, _, _, _, _, _, _) => {
                        h2280
                    }
                };
                let a2282 = 4294967295u64;
                if nat_le(a2281, a2282) {
                    let a2284 = match v0.clone() {
                        Adt1::C0(_, h2283, _, _, _, _, _, _, _, _, _, _) => {
                            h2283
                        }
                    };
                    let a2285 = 4294967295u64;
                    nat_le(a2284, a2285)
                } else {
                    false
                }
            } else {
                false
            } {
                Ok(Adt10::C4({
                    let a2286 = v5;
                    let a2287 = 0u64;
                    if nat_eq(a2286, a2287) {
                        0u64
                    } else {
                        let a2288 = v6;
                        let a2289 = f144(v5)?;
                        nat_mul(a2288, a2289)?
                    }
                }, 4u64, f20(v1, 1u64), f20(v3, 1u64), Adt4::C32(match v0 {
                    Adt1::C0(h2290, _, _, _, _, _, _, _, _, _, _, _) => {
                        h2290
                    }
                })))
            } else {
                Ok(Adt10::C1)
            }
        }
    }
}

pub fn f88(v0: Adt1, v1: Adt2, v2: Adt2) -> R<Adt10> {
    let v3: u64 = match v0.clone() {
        Adt1::C0(h2291, _, _, _, _, _, _, _, _, _, _, _) => {
            h2291
        }
    };
    let v4: u64 = match v0.clone() {
        Adt1::C0(_, h2292, _, _, _, _, _, _, _, _, _, _) => {
            h2292
        }
    };
    let v5: u64 = match v0 {
        Adt1::C0(_, _, h2293, _, _, _, _, _, _, _, _, _) => {
            h2293
        }
    };
    let v6: u64 = {
        let a2295 = match v1.clone() {
            Adt2::C0(_, h2294, _, _, _, _, _, _, _, _, _) => {
                h2294
            }
        };
        let a2296 = v4;
        let a2297 = 0u64;
        nat_quot(a2295, a2296, a2297)
    };
    let v7: u64 = {
        let a2299 = match v1.clone() {
            Adt2::C0(h2298, _, _, _, _, _, _, _, _, _, _) => {
                h2298
            }
        };
        let a2300 = 0u64;
        if nat_eq(a2299, a2300) {
            4u64
        } else {
            2u64
        }
    };
    if if if if if if if if if if if if if if if f149(match v1.clone() {
        Adt2::C0(h2301, _, _, _, _, _, _, _, _, _, _) => {
            h2301
        }
    }) {
        let a2304 = match v1.clone() {
            Adt2::C0(h2302, _, _, _, _, _, _, _, _, _, _) => {
                h2302
            }
        };
        let a2305 = match v2.clone() {
            Adt2::C0(h2303, _, _, _, _, _, _, _, _, _, _) => {
                h2303
            }
        };
        nat_eq(a2304, a2305)
    } else {
        false
    } {
        let a2306 = 0u64;
        let a2307 = v3;
        nat_lt(a2306, a2307)
    } else {
        false
    } {
        let a2308 = v3;
        let a2309 = 2147483648u64;
        nat_lt(a2308, a2309)
    } else {
        false
    } {
        let a2310 = 0u64;
        let a2311 = v4;
        nat_lt(a2310, a2311)
    } else {
        false
    } {
        let a2312 = v4;
        let a2313 = 2147483648u64;
        nat_lt(a2312, a2313)
    } else {
        false
    } {
        let a2314 = v5;
        let a2315 = 2147483648u64;
        nat_lt(a2314, a2315)
    } else {
        false
    } {
        let a2316 = 0u64;
        let a2317 = v6;
        nat_lt(a2316, a2317)
    } else {
        false
    } {
        let a2319 = 0u64;
        let a2320 = match v2.clone() {
            Adt2::C0(_, h2318, _, _, _, _, _, _, _, _, _) => {
                h2318
            }
        };
        nat_lt(a2319, a2320)
    } else {
        false
    } {
        let a2324 = match v1.clone() {
            Adt2::C0(_, h2321, _, _, _, _, _, _, _, _, _) => {
                h2321
            }
        };
        let a2325 = {
            let a2322 = v6;
            let a2323 = v4;
            nat_mul(a2322, a2323)?
        };
        nat_eq(a2324, a2325)
    } else {
        false
    } {
        let a2327 = match v2.clone() {
            Adt2::C0(_, _, h2326, _, _, _, _, _, _, _, _) => {
                h2326
            }
        };
        let a2328 = v4;
        nat_eq(a2327, a2328)
    } else {
        false
    } {
        let a2341 = {
            let a2334 = {
                let a2332 = match v2.clone() {
                    Adt2::C0(_, h2329, _, _, _, _, _, _, _, _, _) => {
                        h2329
                    }
                };
                let a2333 = {
                    let a2330 = v5;
                    let a2331 = 2u64;
                    nat_mul(a2330, a2331)?
                };
                nat_add(a2332, a2333)?
            };
            let a2335 = v3;
            nat_add(a2334, a2335)?
        };
        let a2342 = {
            let a2339 = {
                let a2337 = match v1.clone() {
                    Adt2::C0(_, _, h2336, _, _, _, _, _, _, _, _) => {
                        h2336
                    }
                };
                let a2338 = v3;
                nat_mul(a2337, a2338)?
            };
            let a2340 = v6;
            nat_add(a2339, a2340)?
        };
        nat_eq(a2341, a2342)
    } else {
        false
    } {
        f192(v1.clone(), v7)?
    } else {
        false
    } {
        f192(v2.clone(), v7)?
    } else {
        false
    } {
        f23(v1.clone(), v2.clone())?
    } else {
        false
    } {
        let v8: Adt6 = Adt6::C0(v6, v3, v5, 1u64);
        Ok(Adt10::C4({
            let a2343 = f16(v2.clone(), 1u64)?;
            let a2344 = f163(v8.clone())?;
            nat_mul(a2343, a2344)?
        }, v7, f20(v1.clone(), 1u64), f20(v2, 1u64), Adt4::C30(v8, match v1 {
            Adt2::C0(h2345, _, _, _, _, _, _, _, _, _, _) => {
                h2345
            }
        })))
    } else {
        Ok(Adt10::C1)
    }
}

pub fn f89(v0: Adt2, v1: Option<Adt2>, v2: Adt2) -> R<Adt10> {
    let m2346 = v1;
    match m2346 {
        None => {
            Ok(Adt10::C1)
        }
        Some(v3) => {
            let a2348 = match v0.clone() {
                Adt2::C0(h2347, _, _, _, _, _, _, _, _, _, _) => {
                    h2347
                }
            };
            let a2349 = 0u64;
            if if if if if if if if if if if if if if if if if if if nat_eq(a2348, a2349) {
                let a2351 = match v3.clone() {
                    Adt2::C0(h2350, _, _, _, _, _, _, _, _, _, _) => {
                        h2350
                    }
                };
                let a2352 = 0u64;
                nat_eq(a2351, a2352)
            } else {
                false
            } {
                let a2354 = match v2.clone() {
                    Adt2::C0(h2353, _, _, _, _, _, _, _, _, _, _) => {
                        h2353
                    }
                };
                let a2355 = 0u64;
                nat_eq(a2354, a2355)
            } else {
                false
            } {
                let a2357 = match v0.clone() {
                    Adt2::C0(_, _, _, _, h2356, _, _, _, _, _, _) => {
                        h2356
                    }
                };
                let a2358 = 1u64;
                nat_eq(a2357, a2358)
            } else {
                false
            } {
                let a2360 = match v3.clone() {
                    Adt2::C0(_, _, _, h2359, _, _, _, _, _, _, _) => {
                        h2359
                    }
                };
                let a2361 = 1u64;
                nat_eq(a2360, a2361)
            } else {
                false
            } {
                let a2363 = match v3.clone() {
                    Adt2::C0(_, _, _, _, h2362, _, _, _, _, _, _) => {
                        h2362
                    }
                };
                let a2364 = 1u64;
                nat_eq(a2363, a2364)
            } else {
                false
            } {
                let a2367 = match v0.clone() {
                    Adt2::C0(_, _, h2365, _, _, _, _, _, _, _, _) => {
                        h2365
                    }
                };
                let a2368 = match v3.clone() {
                    Adt2::C0(_, _, h2366, _, _, _, _, _, _, _, _) => {
                        h2366
                    }
                };
                nat_eq(a2367, a2368)
            } else {
                false
            } {
                let a2373 = match v3.clone() {
                    Adt2::C0(_, h2369, _, _, _, _, _, _, _, _, _) => {
                        h2369
                    }
                };
                let a2374 = {
                    let a2371 = match v0.clone() {
                        Adt2::C0(_, h2370, _, _, _, _, _, _, _, _, _) => {
                            h2370
                        }
                    };
                    let a2372 = 1u64;
                    nat_add(a2371, a2372)?
                };
                nat_le(a2373, a2374)
            } else {
                false
            } {
                let a2377 = match v2.clone() {
                    Adt2::C0(_, h2375, _, _, _, _, _, _, _, _, _) => {
                        h2375
                    }
                };
                let a2378 = match v0.clone() {
                    Adt2::C0(_, _, h2376, _, _, _, _, _, _, _, _) => {
                        h2376
                    }
                };
                nat_eq(a2377, a2378)
            } else {
                false
            } {
                let a2386 = match v2.clone() {
                    Adt2::C0(_, _, h2379, _, _, _, _, _, _, _, _) => {
                        h2379
                    }
                };
                let a2387 = {
                    let a2384 = {
                        let a2381 = match v0.clone() {
                            Adt2::C0(_, h2380, _, _, _, _, _, _, _, _, _) => {
                                h2380
                            }
                        };
                        let a2382 = 1u64;
                        nat_add(a2381, a2382)?
                    };
                    let a2385 = match v3.clone() {
                        Adt2::C0(_, h2383, _, _, _, _, _, _, _, _, _) => {
                            h2383
                        }
                    };
                    nat_sub(a2384, a2385)
                };
                nat_eq(a2386, a2387)
            } else {
                false
            } {
                let a2390 = match v2.clone() {
                    Adt2::C0(_, _, _, h2388, _, _, _, _, _, _, _) => {
                        h2388
                    }
                };
                let a2391 = match v0.clone() {
                    Adt2::C0(_, _, _, h2389, _, _, _, _, _, _, _) => {
                        h2389
                    }
                };
                nat_eq(a2390, a2391)
            } else {
                false
            } {
                let a2393 = match v2.clone() {
                    Adt2::C0(_, _, _, _, h2392, _, _, _, _, _, _) => {
                        h2392
                    }
                };
                let a2394 = 1u64;
                nat_eq(a2393, a2394)
            } else {
                false
            } {
                f22(v2.clone())?
            } else {
                false
            } {
                let a2396 = match v0.clone() {
                    Adt2::C0(_, _, _, _, _, h2395, _, _, _, _, _) => {
                        h2395
                    }
                };
                let a2397 = 4u64;
                nat_eq(a2396, a2397)
            } else {
                false
            } {
                let a2399 = match v3.clone() {
                    Adt2::C0(_, _, _, _, _, h2398, _, _, _, _, _) => {
                        h2398
                    }
                };
                let a2400 = 4u64;
                nat_eq(a2399, a2400)
            } else {
                false
            } {
                let a2405 = match v0.clone() {
                    Adt2::C0(_, _, _, _, _, _, h2401, _, _, _, _) => {
                        h2401
                    }
                };
                let a2406 = {
                    let a2403 = match v0.clone() {
                        Adt2::C0(_, h2402, _, _, _, _, _, _, _, _, _) => {
                            h2402
                        }
                    };
                    let a2404 = 4u64;
                    nat_mul(a2403, a2404)?
                };
                nat_eq(a2405, a2406)
            } else {
                false
            } {
                let a2411 = match v3.clone() {
                    Adt2::C0(_, _, _, _, _, _, h2407, _, _, _, _) => {
                        h2407
                    }
                };
                let a2412 = {
                    let a2409 = match v3.clone() {
                        Adt2::C0(_, h2408, _, _, _, _, _, _, _, _, _) => {
                            h2408
                        }
                    };
                    let a2410 = 4u64;
                    nat_mul(a2409, a2410)?
                };
                nat_eq(a2411, a2412)
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
                    let a2414 = f16(v2.clone(), 1u64)?;
                    let a2415 = f162(match v3.clone() {
                        Adt2::C0(_, h2413, _, _, _, _, _, _, _, _, _) => {
                            h2413
                        }
                    })?;
                    nat_mul(a2414, a2415)?
                }, 4u64, f20(v0, 1u64), f20(v2, 1u64), Adt4::C29(f20(v3, 1u64))))
            } else {
                Ok(Adt10::C1)
            }
        }
    }
}

pub fn f90(v0: bool, v1: Adt1, v2: Adt2, v3: Adt2) -> R<Adt10> {
    let v4: Adt6 = Adt6::C0(match v1.clone() {
        Adt1::C0(_, h2416, _, _, _, _, _, _, _, _, _, _) => {
            h2416
        }
    }, if v0 {
        match v1.clone() {
            Adt1::C0(_, _, _, h2417, _, _, _, _, _, _, _, _) => {
                h2417
            }
        }
    } else {
        match v1.clone() {
            Adt1::C0(_, _, h2418, _, _, _, _, _, _, _, _, _) => {
                h2418
            }
        }
    }, if v0 {
        match v1.clone() {
            Adt1::C0(_, _, _, _, _, h2419, _, _, _, _, _, _) => {
                h2419
            }
        }
    } else {
        match v1.clone() {
            Adt1::C0(_, _, _, h2420, _, _, _, _, _, _, _, _) => {
                h2420
            }
        }
    }, 1u64);
    let v5: Adt6 = if v0 {
        Adt6::C0(match v1.clone() {
            Adt1::C0(_, _, h2421, _, _, _, _, _, _, _, _, _) => {
                h2421
            }
        }, match v1.clone() {
            Adt1::C0(_, _, _, _, h2422, _, _, _, _, _, _, _) => {
                h2422
            }
        }, match v1.clone() {
            Adt1::C0(_, _, _, _, _, _, h2423, _, _, _, _, _) => {
                h2423
            }
        }, 1u64)
    } else {
        Adt6::C0(1u64, 1u64, 0u64, 1u64)
    };
    let a2425 = match v1.clone() {
        Adt1::C0(h2424, _, _, _, _, _, _, _, _, _, _, _) => {
            h2424
        }
    };
    let a2426 = 1u64;
    if if if if if if if if if if if nat_le(a2425, a2426) {
        let a2428 = match v2.clone() {
            Adt2::C0(h2427, _, _, _, _, _, _, _, _, _, _) => {
                h2427
            }
        };
        let a2429 = 1u64;
        nat_le(a2428, a2429)
    } else {
        false
    } {
        let a2431 = match v3.clone() {
            Adt2::C0(h2430, _, _, _, _, _, _, _, _, _, _) => {
                h2430
            }
        };
        let a2432 = 0u64;
        nat_eq(a2431, a2432)
    } else {
        false
    } {
        let a2437 = match v2.clone() {
            Adt2::C0(_, _, _, _, _, h2433, _, _, _, _, _) => {
                h2433
            }
        };
        let a2438 = {
            let a2435 = match v2.clone() {
                Adt2::C0(h2434, _, _, _, _, _, _, _, _, _, _) => {
                    h2434
                }
            };
            let a2436 = 0u64;
            if nat_eq(a2435, a2436) {
                4u64
            } else {
                2u64
            }
        };
        nat_eq(a2437, a2438)
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
        f193(v4.clone(), match v2.clone() {
            Adt2::C0(_, h2439, _, _, _, _, _, _, _, _, _) => {
                h2439
            }
        }, match v3.clone() {
            Adt2::C0(_, h2440, _, _, _, _, _, _, _, _, _) => {
                h2440
            }
        })?
    } else {
        false
    } {
        if v0 {
            f193(v5.clone(), match v2.clone() {
                Adt2::C0(_, _, h2441, _, _, _, _, _, _, _, _) => {
                    h2441
                }
            }, match v3.clone() {
                Adt2::C0(_, _, h2442, _, _, _, _, _, _, _, _) => {
                    h2442
                }
            })?
        } else {
            let a2445 = match v2.clone() {
                Adt2::C0(_, _, h2443, _, _, _, _, _, _, _, _) => {
                    h2443
                }
            };
            let a2446 = match v3.clone() {
                Adt2::C0(_, _, h2444, _, _, _, _, _, _, _, _) => {
                    h2444
                }
            };
            nat_eq(a2445, a2446)
        }
    } else {
        false
    } {
        let a2449 = match v2.clone() {
            Adt2::C0(_, _, _, h2447, _, _, _, _, _, _, _) => {
                h2447
            }
        };
        let a2450 = match v3.clone() {
            Adt2::C0(_, _, _, h2448, _, _, _, _, _, _, _) => {
                h2448
            }
        };
        nat_eq(a2449, a2450)
    } else {
        false
    } {
        let a2453 = match v2.clone() {
            Adt2::C0(_, _, _, _, h2451, _, _, _, _, _, _) => {
                h2451
            }
        };
        let a2454 = match v3.clone() {
            Adt2::C0(_, _, _, _, h2452, _, _, _, _, _, _) => {
                h2452
            }
        };
        nat_eq(a2453, a2454)
    } else {
        false
    } {
        let a2459 = {
            let a2457 = match v4.clone() {
                Adt6::C0(h2455, _, _, _) => {
                    h2455
                }
            };
            let a2458 = match v5.clone() {
                Adt6::C0(h2456, _, _, _) => {
                    h2456
                }
            };
            nat_mul(a2457, a2458)?
        };
        let a2460 = 2147483648u64;
        nat_lt(a2459, a2460)
    } else {
        false
    } {
        let v6: Adt7 = Adt7::C0(v4, v5, v0, {
            let a2462 = match v1 {
                Adt1::C0(h2461, _, _, _, _, _, _, _, _, _, _, _) => {
                    h2461
                }
            };
            let a2463 = 0u64;
            nat_eq(a2462, a2463)
        }, match v2.clone() {
            Adt2::C0(h2464, _, _, _, _, _, _, _, _, _, _) => {
                h2464
            }
        });
        Ok(Adt10::C4({
            let a2465 = f16(v3.clone(), 1u64)?;
            let a2466 = f158(v6.clone())?;
            nat_mul(a2465, a2466)?
        }, 4u64, f20(v2, 1u64), f20(v3, 1u64), Adt4::C28(v6)))
    } else {
        Ok(Adt10::C1)
    }
}

pub fn f91(v0: Adt1, v1: Adt2, v2: Option<Adt2>, v3: Adt2) -> R<Adt10> {
    let m2467 = v2;
    match m2467 {
        None => {
            Ok(Adt10::C1)
        }
        Some(v4) => {
            let v5: bool = {
                let a2469 = match v0.clone() {
                    Adt1::C0(_, _, _, _, _, _, h2468, _, _, _, _, _) => {
                        h2468
                    }
                };
                let a2470 = 1u64;
                nat_eq(a2469, a2470)
            };
            let v6: Adt6 = Adt6::C0(match v1.clone() {
                Adt2::C0(_, h2471, _, _, _, _, _, _, _, _, _) => {
                    h2471
                }
            }, match v0.clone() {
                Adt1::C0(h2472, _, _, _, _, _, _, _, _, _, _, _) => {
                    h2472
                }
            }, match v0.clone() {
                Adt1::C0(_, _, h2473, _, _, _, _, _, _, _, _, _) => {
                    h2473
                }
            }, match v0.clone() {
                Adt1::C0(_, _, _, _, h2474, _, _, _, _, _, _, _) => {
                    h2474
                }
            });
            let v7: Adt6 = if v5 {
                Adt6::C0(match v1.clone() {
                    Adt2::C0(_, _, h2475, _, _, _, _, _, _, _, _) => {
                        h2475
                    }
                }, match v0.clone() {
                    Adt1::C0(_, h2476, _, _, _, _, _, _, _, _, _, _) => {
                        h2476
                    }
                }, match v0.clone() {
                    Adt1::C0(_, _, _, h2477, _, _, _, _, _, _, _, _) => {
                        h2477
                    }
                }, match v0.clone() {
                    Adt1::C0(_, _, _, _, _, h2478, _, _, _, _, _, _) => {
                        h2478
                    }
                })
            } else {
                Adt6::C0(1u64, 1u64, 0u64, 1u64)
            };
            let v8: Adt6 = Adt6::C0(1u64, 1u64, 0u64, 1u64);
            let v9: u64 = if v5 {
                match v4.clone() {
                    Adt2::C0(_, _, _, h2479, _, _, _, _, _, _, _) => {
                        h2479
                    }
                }
            } else {
                match v4.clone() {
                    Adt2::C0(_, _, h2480, _, _, _, _, _, _, _, _) => {
                        h2480
                    }
                }
            };
            let v10: u64 = if v5 {
                match v4.clone() {
                    Adt2::C0(_, _, _, _, h2481, _, _, _, _, _, _) => {
                        h2481
                    }
                }
            } else {
                match v4.clone() {
                    Adt2::C0(_, _, _, h2482, _, _, _, _, _, _, _) => {
                        h2482
                    }
                }
            };
            let v11: u64 = if v5 {
                match v4.clone() {
                    Adt2::C0(_, _, h2483, _, _, _, _, _, _, _, _) => {
                        h2483
                    }
                }
            } else {
                1u64
            };
            let v12: u64 = match v4.clone() {
                Adt2::C0(h2484, _, _, _, _, _, _, _, _, _, _) => {
                    h2484
                }
            };
            let v13: u64 = match v3.clone() {
                Adt2::C0(h2485, _, _, _, _, _, _, _, _, _, _) => {
                    h2485
                }
            };
            let v14: u64 = {
                let a2486 = v12;
                let a2487 = 0u64;
                if nat_eq(a2486, a2487) {
                    4u64
                } else {
                    2u64
                }
            };
            let v15: u64 = {
                let a2489 = match v4.clone() {
                    Adt2::C0(_, h2488, _, _, _, _, _, _, _, _, _) => {
                        h2488
                    }
                };
                let a2490 = v14;
                nat_mul(a2489, a2490)?
            };
            let v16: u64 = if v5 {
                match v4.clone() {
                    Adt2::C0(_, _, _, _, _, _, _, h2491, _, _, _) => {
                        h2491
                    }
                }
            } else {
                match v4.clone() {
                    Adt2::C0(_, _, _, _, _, _, h2492, _, _, _, _) => {
                        h2492
                    }
                }
            };
            let v17: u64 = if v5 {
                match v4.clone() {
                    Adt2::C0(_, _, _, _, _, _, _, _, h2493, _, _) => {
                        h2493
                    }
                }
            } else {
                match v4.clone() {
                    Adt2::C0(_, _, _, _, _, _, _, h2494, _, _, _) => {
                        h2494
                    }
                }
            };
            let a2496 = match v0 {
                Adt1::C0(_, _, _, _, _, _, h2495, _, _, _, _, _) => {
                    h2495
                }
            };
            let a2497 = 1u64;
            if if if if if if if if if if nat_le(a2496, a2497) {
                let a2498 = v12;
                let a2499 = 0u64;
                if nat_eq(a2498, a2499) {
                    true
                } else {
                    let a2500 = v12;
                    let a2501 = 1u64;
                    if nat_eq(a2500, a2501) {
                        let a2502 = v13;
                        let a2503 = 1u64;
                        nat_eq(a2502, a2503)
                    } else {
                        false
                    }
                }
            } else {
                false
            } {
                let a2504 = v13;
                let a2505 = 0u64;
                if nat_eq(a2504, a2505) {
                    true
                } else {
                    let a2506 = v13;
                    let a2507 = 1u64;
                    nat_eq(a2506, a2507)
                }
            } else {
                false
            } {
                let a2509 = match v4.clone() {
                    Adt2::C0(_, _, _, _, _, h2508, _, _, _, _, _) => {
                        h2508
                    }
                };
                let a2510 = v14;
                nat_eq(a2509, a2510)
            } else {
                false
            } {
                let a2518 = match v3.clone() {
                    Adt2::C0(_, h2511, _, _, _, _, _, _, _, _, _) => {
                        h2511
                    }
                };
                let a2519 = {
                    let a2516 = v9;
                    let a2517 = {
                        let a2514 = match v6.clone() {
                            Adt6::C0(h2512, _, _, _) => {
                                h2512
                            }
                        };
                        let a2515 = match v7.clone() {
                            Adt6::C0(h2513, _, _, _) => {
                                h2513
                            }
                        };
                        nat_mul(a2514, a2515)?
                    };
                    nat_mul(a2516, a2517)?
                };
                nat_eq(a2518, a2519)
            } else {
                false
            } {
                let a2522 = match v3.clone() {
                    Adt2::C0(_, _, h2520, _, _, _, _, _, _, _, _) => {
                        h2520
                    }
                };
                let a2523 = f71(v6.clone(), match v4.clone() {
                    Adt2::C0(_, h2521, _, _, _, _, _, _, _, _, _) => {
                        h2521
                    }
                })?;
                nat_eq(a2522, a2523)
            } else {
                false
            } {
                let a2525 = 0u64;
                let a2526 = match v3.clone() {
                    Adt2::C0(_, _, h2524, _, _, _, _, _, _, _, _) => {
                        h2524
                    }
                };
                nat_lt(a2525, a2526)
            } else {
                false
            } {
                if v5 {
                    let a2528 = match v1 {
                        Adt2::C0(_, _, _, h2527, _, _, _, _, _, _, _) => {
                            h2527
                        }
                    };
                    let a2529 = v9;
                    if if if nat_eq(a2528, a2529) {
                        let a2531 = match v3.clone() {
                            Adt2::C0(_, _, _, h2530, _, _, _, _, _, _, _) => {
                                h2530
                            }
                        };
                        let a2532 = f71(v7.clone(), v11)?;
                        nat_eq(a2531, a2532)
                    } else {
                        false
                    } {
                        let a2534 = match v3.clone() {
                            Adt2::C0(_, _, _, _, h2533, _, _, _, _, _, _) => {
                                h2533
                            }
                        };
                        let a2535 = v10;
                        nat_eq(a2534, a2535)
                    } else {
                        false
                    } {
                        let a2537 = 0u64;
                        let a2538 = match v3.clone() {
                            Adt2::C0(_, _, _, h2536, _, _, _, _, _, _, _) => {
                                h2536
                            }
                        };
                        nat_lt(a2537, a2538)
                    } else {
                        false
                    }
                } else {
                    let a2540 = match v1 {
                        Adt2::C0(_, _, h2539, _, _, _, _, _, _, _, _) => {
                            h2539
                        }
                    };
                    let a2541 = v9;
                    if if if nat_eq(a2540, a2541) {
                        let a2543 = match v4.clone() {
                            Adt2::C0(_, _, _, _, h2542, _, _, _, _, _, _) => {
                                h2542
                            }
                        };
                        let a2544 = 1u64;
                        nat_eq(a2543, a2544)
                    } else {
                        false
                    } {
                        let a2546 = match v3.clone() {
                            Adt2::C0(_, _, _, h2545, _, _, _, _, _, _, _) => {
                                h2545
                            }
                        };
                        let a2547 = v10;
                        nat_eq(a2546, a2547)
                    } else {
                        false
                    } {
                        let a2549 = match v3.clone() {
                            Adt2::C0(_, _, _, _, h2548, _, _, _, _, _, _) => {
                                h2548
                            }
                        };
                        let a2550 = 1u64;
                        nat_eq(a2549, a2550)
                    } else {
                        false
                    }
                }
            } else {
                false
            } {
                if f34(match v4.clone() {
                    Adt2::C0(_, h2551, _, _, _, _, _, _, _, _, _) => {
                        h2551
                    }
                }, v11, v9, v10) {
                    true
                } else {
                    let a2571 = {
                        let a2568 = {
                            let a2566 = {
                                let a2560 = {
                                    let a2554 = {
                                        let a2552 = v10;
                                        let a2553 = 1u64;
                                        nat_sub(a2552, a2553)
                                    };
                                    let a2555 = v17;
                                    nat_mul(a2554, a2555)?
                                };
                                let a2561 = {
                                    let a2558 = {
                                        let a2556 = v9;
                                        let a2557 = 1u64;
                                        nat_sub(a2556, a2557)
                                    };
                                    let a2559 = v16;
                                    nat_mul(a2558, a2559)?
                                };
                                nat_add(a2560, a2561)?
                            };
                            let a2567 = {
                                let a2564 = {
                                    let a2562 = v11;
                                    let a2563 = 1u64;
                                    nat_sub(a2562, a2563)
                                };
                                let a2565 = v15;
                                nat_mul(a2564, a2565)?
                            };
                            nat_add(a2566, a2567)?
                        };
                        let a2569 = v15;
                        nat_add(a2568, a2569)?
                    };
                    let a2572 = match v4.clone() {
                        Adt2::C0(_, _, _, _, _, _, _, _, _, h2570, _) => {
                            h2570
                        }
                    };
                    nat_le(a2571, a2572)
                }
            } else {
                false
            } {
                f23(v4.clone(), v3.clone())?
            } else {
                false
            } {
                Ok(Adt10::C4(f16(v3.clone(), 1u64)?, {
                    let a2573 = v13;
                    let a2574 = 0u64;
                    if nat_eq(a2573, a2574) {
                        4u64
                    } else {
                        2u64
                    }
                }, Adt3::C0(match v4 {
                    Adt2::C0(_, h2575, _, _, _, _, _, _, _, _, _) => {
                        h2575
                    }
                }, v11, 1u64, v10, v14, v15, 0u64, 0u64), if v5 {
                    f20(v3, 1u64)
                } else {
                    Adt3::C0(match v3.clone() {
                        Adt2::C0(_, h2576, _, _, _, _, _, _, _, _, _) => {
                            h2576
                        }
                    }, match v3.clone() {
                        Adt2::C0(_, _, h2577, _, _, _, _, _, _, _, _) => {
                            h2577
                        }
                    }, 1u64, match v3.clone() {
                        Adt2::C0(_, _, _, h2578, _, _, _, _, _, _, _) => {
                            h2578
                        }
                    }, match v3.clone() {
                        Adt2::C0(_, _, _, _, _, h2579, _, _, _, _, _) => {
                            h2579
                        }
                    }, match v3.clone() {
                        Adt2::C0(_, _, _, _, _, _, h2580, _, _, _, _) => {
                            h2580
                        }
                    }, 0u64, match v3 {
                        Adt2::C0(_, _, _, _, _, _, _, h2581, _, _, _) => {
                            h2581
                        }
                    })
                }, Adt4::C27(Adt5::C0(v6, v7, v8, v16, v17, v12, 1u64, v13))))
            } else {
                Ok(Adt10::C1)
            }
        }
    }
}

pub fn f92(v0: Adt1, v1: Adt2, v2: Option<Adt2>, v3: Adt2) -> R<Adt10> {
    let m2582 = v2;
    match m2582 {
        None => {
            Ok(Adt10::C1)
        }
        Some(v4) => {
            let v5: Adt6 = Adt6::C0(match v1.clone() {
                Adt2::C0(_, h2583, _, _, _, _, _, _, _, _, _) => {
                    h2583
                }
            }, match v0.clone() {
                Adt1::C0(h2584, _, _, _, _, _, _, _, _, _, _, _) => {
                    h2584
                }
            }, match v0.clone() {
                Adt1::C0(_, _, _, h2585, _, _, _, _, _, _, _, _) => {
                    h2585
                }
            }, match v0.clone() {
                Adt1::C0(_, _, _, _, _, _, h2586, _, _, _, _, _) => {
                    h2586
                }
            });
            let v6: Adt6 = Adt6::C0(match v1.clone() {
                Adt2::C0(_, _, h2587, _, _, _, _, _, _, _, _) => {
                    h2587
                }
            }, match v0.clone() {
                Adt1::C0(_, h2588, _, _, _, _, _, _, _, _, _, _) => {
                    h2588
                }
            }, match v0.clone() {
                Adt1::C0(_, _, _, _, h2589, _, _, _, _, _, _, _) => {
                    h2589
                }
            }, match v0.clone() {
                Adt1::C0(_, _, _, _, _, _, _, h2590, _, _, _, _) => {
                    h2590
                }
            });
            let v7: Adt6 = Adt6::C0(match v1 {
                Adt2::C0(_, _, _, h2591, _, _, _, _, _, _, _) => {
                    h2591
                }
            }, match v0.clone() {
                Adt1::C0(_, _, h2592, _, _, _, _, _, _, _, _, _) => {
                    h2592
                }
            }, match v0.clone() {
                Adt1::C0(_, _, _, _, _, h2593, _, _, _, _, _, _) => {
                    h2593
                }
            }, match v0.clone() {
                Adt1::C0(_, _, _, _, _, _, _, _, h2594, _, _, _) => {
                    h2594
                }
            });
            let v8: u64 = match v0 {
                Adt1::C0(_, _, _, _, _, _, _, _, _, h2595, _, _) => {
                    h2595
                }
            };
            let v9: u64 = f71(v7.clone(), match v4.clone() {
                Adt2::C0(_, _, _, h2596, _, _, _, _, _, _, _) => {
                    h2596
                }
            })?;
            let v10: u64 = match v3.clone() {
                Adt2::C0(h2597, _, _, _, _, _, _, _, _, _, _) => {
                    h2597
                }
            };
            let v11: u64 = {
                let a2605 = v8;
                let a2606 = {
                    let a2603 = match v5.clone() {
                        Adt6::C0(h2598, _, _, _) => {
                            h2598
                        }
                    };
                    let a2604 = {
                        let a2601 = match v6.clone() {
                            Adt6::C0(h2599, _, _, _) => {
                                h2599
                            }
                        };
                        let a2602 = match v7.clone() {
                            Adt6::C0(h2600, _, _, _) => {
                                h2600
                            }
                        };
                        nat_mul(a2601, a2602)?
                    };
                    nat_mul(a2603, a2604)?
                };
                nat_mul(a2605, a2606)?
            };
            let a2608 = match v4.clone() {
                Adt2::C0(h2607, _, _, _, _, _, _, _, _, _, _) => {
                    h2607
                }
            };
            let a2609 = 0u64;
            if if if if if if if if if if if if if nat_eq(a2608, a2609) {
                let a2610 = v10;
                let a2611 = 0u64;
                if nat_eq(a2610, a2611) {
                    true
                } else {
                    let a2612 = v10;
                    let a2613 = 1u64;
                    nat_eq(a2612, a2613)
                }
            } else {
                false
            } {
                let a2614 = 0u64;
                let a2615 = v8;
                nat_lt(a2614, a2615)
            } else {
                false
            } {
                let a2616 = v8;
                let a2617 = 2147483648u64;
                nat_lt(a2616, a2617)
            } else {
                false
            } {
                let a2619 = match v4.clone() {
                    Adt2::C0(_, _, _, _, _, h2618, _, _, _, _, _) => {
                        h2618
                    }
                };
                let a2620 = 4u64;
                nat_eq(a2619, a2620)
            } else {
                false
            } {
                let a2621 = 0u64;
                let a2622 = v9;
                nat_lt(a2621, a2622)
            } else {
                false
            } {
                let a2624 = 0u64;
                let a2625 = match v3.clone() {
                    Adt2::C0(_, _, h2623, _, _, _, _, _, _, _, _) => {
                        h2623
                    }
                };
                nat_lt(a2624, a2625)
            } else {
                false
            } {
                let a2627 = 0u64;
                let a2628 = match v3.clone() {
                    Adt2::C0(_, _, _, h2626, _, _, _, _, _, _, _) => {
                        h2626
                    }
                };
                nat_lt(a2627, a2628)
            } else {
                false
            } {
                let a2630 = match v3.clone() {
                    Adt2::C0(_, h2629, _, _, _, _, _, _, _, _, _) => {
                        h2629
                    }
                };
                let a2631 = v11;
                nat_eq(a2630, a2631)
            } else {
                false
            } {
                let a2634 = match v3.clone() {
                    Adt2::C0(_, _, h2632, _, _, _, _, _, _, _, _) => {
                        h2632
                    }
                };
                let a2635 = f71(v5.clone(), match v4.clone() {
                    Adt2::C0(_, h2633, _, _, _, _, _, _, _, _, _) => {
                        h2633
                    }
                })?;
                nat_eq(a2634, a2635)
            } else {
                false
            } {
                let a2638 = match v3.clone() {
                    Adt2::C0(_, _, _, h2636, _, _, _, _, _, _, _) => {
                        h2636
                    }
                };
                let a2639 = f71(v6.clone(), match v4.clone() {
                    Adt2::C0(_, _, h2637, _, _, _, _, _, _, _, _) => {
                        h2637
                    }
                })?;
                nat_eq(a2638, a2639)
            } else {
                false
            } {
                let a2647 = match v3.clone() {
                    Adt2::C0(_, _, _, _, h2640, _, _, _, _, _, _) => {
                        h2640
                    }
                };
                let a2648 = {
                    let a2645 = v9;
                    let a2646 = {
                        let a2642 = match v4.clone() {
                            Adt2::C0(_, _, _, _, h2641, _, _, _, _, _, _) => {
                                h2641
                            }
                        };
                        let a2643 = v8;
                        let a2644 = 0u64;
                        nat_quot(a2642, a2643, a2644)
                    };
                    nat_mul(a2645, a2646)?
                };
                nat_eq(a2647, a2648)
            } else {
                false
            } {
                f23(v4.clone(), v3.clone())?
            } else {
                false
            } {
                Ok(Adt10::C4(f16(v3.clone(), 1u64)?, {
                    let a2649 = v10;
                    let a2650 = 0u64;
                    if nat_eq(a2649, a2650) {
                        4u64
                    } else {
                        2u64
                    }
                }, f20(v4.clone(), 1u64), f20(v3, 1u64), Adt4::C27(Adt5::C0(v5, v6, v7, match v4.clone() {
                    Adt2::C0(_, _, _, _, _, _, _, _, h2651, _, _) => {
                        h2651
                    }
                }, {
                    let a2653 = v8;
                    let a2654 = match v4 {
                        Adt2::C0(_, _, _, _, _, _, _, _, h2652, _, _) => {
                            h2652
                        }
                    };
                    nat_mul(a2653, a2654)?
                }, 0u64, v9, v10))))
            } else {
                Ok(Adt10::C1)
            }
        }
    }
}

pub fn f93(v0: Adt2, v1: Option<Adt2>, v2: Adt2) -> R<Adt10> {
    let m2655 = v1;
    match m2655 {
        None => {
            Ok(Adt10::C1)
        }
        Some(v3) => {
            f194(v0, v3, v2)
        }
    }
}

pub fn f94(v0: Adt2, v1: Adt2) -> R<Adt10> {
    let a2657 = match v0.clone() {
        Adt2::C0(h2656, _, _, _, _, _, _, _, _, _, _) => {
            h2656
        }
    };
    let a2658 = 1u64;
    if if nat_eq(a2657, a2658) {
        true
    } else {
        let a2660 = match v0.clone() {
            Adt2::C0(h2659, _, _, _, _, _, _, _, _, _, _) => {
                h2659
            }
        };
        let a2661 = 30u64;
        nat_eq(a2660, a2661)
    } {
        let a2663 = match v1.clone() {
            Adt2::C0(h2662, _, _, _, _, _, _, _, _, _, _) => {
                h2662
            }
        };
        let a2664 = 1u64;
        if if if if if if if if if if if nat_eq(a2663, a2664) {
            let a2667 = match v0.clone() {
                Adt2::C0(_, h2665, _, _, _, _, _, _, _, _, _) => {
                    h2665
                }
            };
            let a2668 = match v1.clone() {
                Adt2::C0(_, h2666, _, _, _, _, _, _, _, _, _) => {
                    h2666
                }
            };
            nat_eq(a2667, a2668)
        } else {
            false
        } {
            let a2670 = 0u64;
            let a2671 = match v1.clone() {
                Adt2::C0(_, _, h2669, _, _, _, _, _, _, _, _) => {
                    h2669
                }
            };
            nat_lt(a2670, a2671)
        } else {
            false
        } {
            let a2674 = match v1.clone() {
                Adt2::C0(_, _, h2672, _, _, _, _, _, _, _, _) => {
                    h2672
                }
            };
            let a2675 = match v1.clone() {
                Adt2::C0(_, _, _, h2673, _, _, _, _, _, _, _) => {
                    h2673
                }
            };
            nat_eq(a2674, a2675)
        } else {
            false
        } {
            let a2677 = match v1.clone() {
                Adt2::C0(_, _, _, _, h2676, _, _, _, _, _, _) => {
                    h2676
                }
            };
            let a2678 = 1u64;
            nat_eq(a2677, a2678)
        } else {
            false
        } {
            let a2686 = match v0.clone() {
                Adt2::C0(_, _, h2679, _, _, _, _, _, _, _, _) => {
                    h2679
                }
            };
            let a2687 = {
                let a2684 = {
                    let a2682 = match v1.clone() {
                        Adt2::C0(_, _, h2680, _, _, _, _, _, _, _, _) => {
                            h2680
                        }
                    };
                    let a2683 = match v1.clone() {
                        Adt2::C0(_, _, h2681, _, _, _, _, _, _, _, _) => {
                            h2681
                        }
                    };
                    nat_add(a2682, a2683)?
                };
                let a2685 = 1u64;
                nat_sub(a2684, a2685)
            };
            nat_eq(a2686, a2687)
        } else {
            false
        } {
            let a2689 = 0u64;
            let a2690 = match v0.clone() {
                Adt2::C0(_, _, _, h2688, _, _, _, _, _, _, _) => {
                    h2688
                }
            };
            nat_lt(a2689, a2690)
        } else {
            false
        } {
            let a2692 = 0u64;
            let a2693 = match v0.clone() {
                Adt2::C0(_, _, _, _, h2691, _, _, _, _, _, _) => {
                    h2691
                }
            };
            nat_lt(a2692, a2693)
        } else {
            false
        } {
            let a2695 = 2u64;
            let a2696 = match v0.clone() {
                Adt2::C0(_, _, _, _, _, h2694, _, _, _, _, _) => {
                    h2694
                }
            };
            nat_le(a2695, a2696)
        } else {
            false
        } {
            let a2698 = 2u64;
            let a2699 = match v1.clone() {
                Adt2::C0(_, _, _, _, _, h2697, _, _, _, _, _) => {
                    h2697
                }
            };
            nat_le(a2698, a2699)
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

pub fn f95(v0: Adt2, v1: Option<Adt2>, v2: Adt2) -> R<Adt10> {
    let m2700 = v1;
    match m2700 {
        None => {
            Ok(Adt10::C1)
        }
        Some(v3) => {
            f195(v0, v3, v2)
        }
    }
}

pub fn f96(v0: u64) -> bool {
    let a2701 = v0;
    let a2702 = 1u64;
    if if nat_eq(a2701, a2702) {
        true
    } else {
        let a2703 = v0;
        let a2704 = 33u64;
        nat_eq(a2703, a2704)
    } {
        true
    } else {
        let a2705 = v0;
        let a2706 = 34u64;
        nat_eq(a2705, a2706)
    }
}

pub fn f97(v0: Adt2, v1: Adt2, v2: Adt2) -> R<Adt10> {
    let v3: u64 = f16(v0.clone(), 1u64)?;
    if if if if if f33(v1, v2.clone()) {
        let a2707 = v3;
        let a2708 = f16(v2.clone(), 1u64)?;
        nat_eq(a2707, a2708)
    } else {
        false
    } {
        f188(v0.clone(), v3)?
    } else {
        false
    } {
        f188(v2.clone(), v3)?
    } else {
        false
    } {
        let a2709 = v3;
        let a2710 = 0u64;
        if nat_eq(a2709, a2710) {
            true
        } else {
            let a2712 = 4u64;
            let a2713 = match v2.clone() {
                Adt2::C0(_, _, _, _, _, h2711, _, _, _, _, _) => {
                    h2711
                }
            };
            nat_le(a2712, a2713)
        }
    } else {
        false
    } {
        Ok(Adt10::C4({
            let a2714 = 2u64;
            let a2715 = v3;
            nat_mul(a2714, a2715)?
        }, 4u64, f20(v0, 1u64), f20(v2, 1u64), Adt4::C35(v3)))
    } else {
        Ok(Adt10::C1)
    }
}

pub fn f98(v0: u64) -> bool {
    if f196(v0) {
        !f197(v0)
    } else {
        false
    }
}

pub fn f99(v0: Adt2, v1: Option<Adt2>, v2: Adt2) -> R<Adt10> {
    let m2716 = v1;
    match m2716 {
        None => {
            Ok(Adt10::C1)
        }
        Some(v3) => {
            let m2718 = f187(match v0.clone() {
                Adt2::C0(h2717, _, _, _, _, _, _, _, _, _, _) => {
                    h2717
                }
            });
            match m2718 {
                Adt18::C0 => {
                    Ok(Adt10::C0)
                }
                Adt18::C1(v4) => {
                    let v5: u64 = match v4.clone() {
                        Adt19::C0(_, h2719, _) => {
                            h2719
                        }
                    };
                    if if if if if if f33(v3, v2.clone()) {
                        f23(v0.clone(), v2.clone())?
                    } else {
                        false
                    } {
                        let a2720 = f16(v0.clone(), 1u64)?;
                        let a2721 = f16(v2.clone(), 1u64)?;
                        nat_eq(a2720, a2721)
                    } else {
                        false
                    } {
                        let a2726 = {
                            let a2723 = match v0.clone() {
                                Adt2::C0(_, h2722, _, _, _, _, _, _, _, _, _) => {
                                    h2722
                                }
                            };
                            let a2724 = v5;
                            let a2725 = 1u64;
                            nat_rem(a2723, a2724, a2725)
                        };
                        let a2727 = 0u64;
                        if nat_eq(a2726, a2727) {
                            let a2730 = match v4 {
                                Adt19::C0(_, _, h2728) => {
                                    h2728
                                }
                            };
                            let a2731 = match v0.clone() {
                                Adt2::C0(_, _, _, _, _, h2729, _, _, _, _, _) => {
                                    h2729
                                }
                            };
                            nat_le(a2730, a2731)
                        } else {
                            false
                        }
                    } else {
                        false
                    } {
                        let a2732 = v5;
                        let a2733 = 1u64;
                        if nat_eq(a2732, a2733) {
                            let a2735 = 4u64;
                            let a2736 = match v2.clone() {
                                Adt2::C0(_, _, _, _, _, h2734, _, _, _, _, _) => {
                                    h2734
                                }
                            };
                            nat_le(a2735, a2736)
                        } else {
                            let a2738 = match v2.clone() {
                                Adt2::C0(_, _, _, _, _, h2737, _, _, _, _, _) => {
                                    h2737
                                }
                            };
                            let a2739 = 4u64;
                            nat_eq(a2738, a2739)
                        }
                    } else {
                        false
                    } {
                        let a2744 = {
                            let a2741 = match v2.clone() {
                                Adt2::C0(_, h2740, _, _, _, _, _, _, _, _, _) => {
                                    h2740
                                }
                            };
                            let a2742 = v5;
                            let a2743 = 1u64;
                            nat_rem(a2741, a2742, a2743)
                        };
                        let a2745 = 0u64;
                        if nat_eq(a2744, a2745) {
                            true
                        } else {
                            f22(v2.clone())?
                        }
                    } else {
                        false
                    } {
                        Ok(Adt10::C4(f16(v0.clone(), v5)?, 4u64, f20(v0.clone(), v5), f20(v2, 1u64), Adt4::C11(match v0 {
                            Adt2::C0(h2746, _, _, _, _, _, _, _, _, _, _) => {
                                h2746
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

pub fn f100(v0: u64, v1: Adt2, v2: bool, v3: Adt2) -> R<Adt10> {
    let a2749 = match v1.clone() {
        Adt2::C0(h2747, _, _, _, _, _, _, _, _, _, _) => {
            h2747
        }
    };
    let a2750 = match v3.clone() {
        Adt2::C0(h2748, _, _, _, _, _, _, _, _, _, _) => {
            h2748
        }
    };
    if nat_eq(a2749, a2750) {
        f198(v0, v1, v2, v3)
    } else {
        if if if if f197(match v1.clone() {
            Adt2::C0(h2751, _, _, _, _, _, _, _, _, _, _) => {
                h2751
            }
        }) {
            f197(match v3.clone() {
                Adt2::C0(h2752, _, _, _, _, _, _, _, _, _, _) => {
                    h2752
                }
            })
        } else {
            false
        } {
            let a2753 = v0;
            let a2754 = 33u64;
            if nat_eq(a2753, a2754) {
                v2
            } else {
                true
            }
        } else {
            false
        } {
            let a2755 = f16(v1.clone(), 1u64)?;
            let a2756 = f16(v3.clone(), 1u64)?;
            nat_eq(a2755, a2756)
        } else {
            false
        } {
            let m2758 = f187(match v3.clone() {
                Adt2::C0(h2757, _, _, _, _, _, _, _, _, _, _) => {
                    h2757
                }
            });
            match m2758 {
                Adt18::C0 => {
                    Ok(Adt10::C0)
                }
                Adt18::C1(v4) => {
                    let v5: u64 = match v1.clone() {
                        Adt2::C0(h2759, _, _, _, _, _, _, _, _, _, _) => {
                            h2759
                        }
                    };
                    let v6: u64 = match v3.clone() {
                        Adt2::C0(h2760, _, _, _, _, _, _, _, _, _, _) => {
                            h2760
                        }
                    };
                    let v7: bool = {
                        let a2761 = v5;
                        let a2762 = 43u64;
                        if nat_eq(a2761, a2762) {
                            true
                        } else {
                            let a2763 = v6;
                            let a2764 = 43u64;
                            nat_eq(a2763, a2764)
                        }
                    };
                    Ok(Adt10::C4({
                        let a2765 = f16(v3.clone(), 1u64)?;
                        let a2766 = if v7 {
                            f142(v5, v6)
                        } else {
                            1u64
                        };
                        nat_mul(a2765, a2766)?
                    }, match v4 {
                        Adt19::C0(_, _, h2767) => {
                            h2767
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

pub fn f101(v0: Adt2, v1: Adt2) -> R<Adt10> {
    let a2769 = match v0.clone() {
        Adt2::C0(h2768, _, _, _, _, _, _, _, _, _, _) => {
            h2768
        }
    };
    let a2770 = 0u64;
    if if if if nat_eq(a2769, a2770) {
        true
    } else {
        let a2772 = match v0.clone() {
            Adt2::C0(h2771, _, _, _, _, _, _, _, _, _, _) => {
                h2771
            }
        };
        let a2773 = 1u64;
        nat_eq(a2772, a2773)
    } {
        true
    } else {
        let a2775 = match v0.clone() {
            Adt2::C0(h2774, _, _, _, _, _, _, _, _, _, _) => {
                h2774
            }
        };
        let a2776 = 25u64;
        if nat_eq(a2775, a2776) {
            true
        } else {
            let a2778 = match v0.clone() {
                Adt2::C0(h2777, _, _, _, _, _, _, _, _, _, _) => {
                    h2777
                }
            };
            let a2779 = 26u64;
            nat_eq(a2778, a2779)
        }
    } {
        true
    } else {
        let a2781 = match v0.clone() {
            Adt2::C0(h2780, _, _, _, _, _, _, _, _, _, _) => {
                h2780
            }
        };
        let a2782 = 30u64;
        nat_eq(a2781, a2782)
    } {
        let a2785 = match v0.clone() {
            Adt2::C0(h2783, _, _, _, _, _, _, _, _, _, _) => {
                h2783
            }
        };
        let a2786 = match v1.clone() {
            Adt2::C0(h2784, _, _, _, _, _, _, _, _, _, _) => {
                h2784
            }
        };
        if if nat_eq(a2785, a2786) {
            f199(match v1.clone() {
                Adt2::C0(_, h2787, _, _, _, _, _, _, _, _, _) => {
                    h2787
                }
            }, match v1.clone() {
                Adt2::C0(_, _, h2788, _, _, _, _, _, _, _, _) => {
                    h2788
                }
            }, match v1.clone() {
                Adt2::C0(_, _, _, h2789, _, _, _, _, _, _, _) => {
                    h2789
                }
            }, match v1.clone() {
                Adt2::C0(_, _, _, _, h2790, _, _, _, _, _, _) => {
                    h2790
                }
            }, match v0.clone() {
                Adt2::C0(_, h2791, _, _, _, _, _, _, _, _, _) => {
                    h2791
                }
            }, match v0.clone() {
                Adt2::C0(_, _, h2792, _, _, _, _, _, _, _, _) => {
                    h2792
                }
            }, match v0.clone() {
                Adt2::C0(_, _, _, h2793, _, _, _, _, _, _, _) => {
                    h2793
                }
            }, match v0.clone() {
                Adt2::C0(_, _, _, _, h2794, _, _, _, _, _, _) => {
                    h2794
                }
            })
        } else {
            false
        } {
            let m2796 = f187(match v0.clone() {
                Adt2::C0(h2795, _, _, _, _, _, _, _, _, _, _) => {
                    h2795
                }
            });
            match m2796 {
                Adt18::C0 => {
                    Ok(Adt10::C0)
                }
                Adt18::C1(v2) => {
                    Ok(Adt10::C4(f16(v1.clone(), 1u64)?, match v2 {
                        Adt19::C0(_, _, h2797) => {
                            h2797
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

pub fn f102(v0: u64, v1: Adt2, v2: Option<Adt2>, v3: Adt2) -> R<Adt10> {
    let m2798 = v2;
    match m2798 {
        None => {
            Ok(Adt10::C1)
        }
        Some(v4) => {
            let a2799 = v0;
            let a2800 = 4u64;
            if if if nat_lt(a2799, a2800) {
                let a2803 = match v1.clone() {
                    Adt2::C0(h2801, _, _, _, _, _, _, _, _, _, _) => {
                        h2801
                    }
                };
                let a2804 = match v4.clone() {
                    Adt2::C0(h2802, _, _, _, _, _, _, _, _, _, _) => {
                        h2802
                    }
                };
                if nat_eq(a2803, a2804) {
                    let a2807 = match v1.clone() {
                        Adt2::C0(h2805, _, _, _, _, _, _, _, _, _, _) => {
                            h2805
                        }
                    };
                    let a2808 = match v3.clone() {
                        Adt2::C0(h2806, _, _, _, _, _, _, _, _, _, _) => {
                            h2806
                        }
                    };
                    nat_eq(a2807, a2808)
                } else {
                    false
                }
            } else {
                false
            } {
                if if if f200(v0, 0u64, match v1.clone() {
                    Adt2::C0(_, h2809, _, _, _, _, _, _, _, _, _) => {
                        h2809
                    }
                }, match v4.clone() {
                    Adt2::C0(_, h2810, _, _, _, _, _, _, _, _, _) => {
                        h2810
                    }
                }, match v3.clone() {
                    Adt2::C0(_, h2811, _, _, _, _, _, _, _, _, _) => {
                        h2811
                    }
                })? {
                    f200(v0, 1u64, match v1.clone() {
                        Adt2::C0(_, _, h2812, _, _, _, _, _, _, _, _) => {
                            h2812
                        }
                    }, match v4.clone() {
                        Adt2::C0(_, _, h2813, _, _, _, _, _, _, _, _) => {
                            h2813
                        }
                    }, match v3.clone() {
                        Adt2::C0(_, _, h2814, _, _, _, _, _, _, _, _) => {
                            h2814
                        }
                    })?
                } else {
                    false
                } {
                    f200(v0, 2u64, match v1.clone() {
                        Adt2::C0(_, _, _, h2815, _, _, _, _, _, _, _) => {
                            h2815
                        }
                    }, match v4.clone() {
                        Adt2::C0(_, _, _, h2816, _, _, _, _, _, _, _) => {
                            h2816
                        }
                    }, match v3.clone() {
                        Adt2::C0(_, _, _, h2817, _, _, _, _, _, _, _) => {
                            h2817
                        }
                    })?
                } else {
                    false
                } {
                    f200(v0, 3u64, match v1.clone() {
                        Adt2::C0(_, _, _, _, h2818, _, _, _, _, _, _) => {
                            h2818
                        }
                    }, match v4.clone() {
                        Adt2::C0(_, _, _, _, h2819, _, _, _, _, _, _) => {
                            h2819
                        }
                    }, match v3.clone() {
                        Adt2::C0(_, _, _, _, h2820, _, _, _, _, _, _) => {
                            h2820
                        }
                    })?
                } else {
                    false
                }
            } else {
                false
            } {
                let m2822 = f187(match v1.clone() {
                    Adt2::C0(h2821, _, _, _, _, _, _, _, _, _, _) => {
                        h2821
                    }
                });
                match m2822 {
                    Adt18::C0 => {
                        Ok(Adt10::C0)
                    }
                    Adt18::C1(v5) => {
                        let v6: u64 = match v5.clone() {
                            Adt19::C0(_, h2823, _) => {
                                h2823
                            }
                        };
                        let a2828 = {
                            let a2825 = match v1.clone() {
                                Adt2::C0(_, h2824, _, _, _, _, _, _, _, _, _) => {
                                    h2824
                                }
                            };
                            let a2826 = v6;
                            let a2827 = 1u64;
                            nat_rem(a2825, a2826, a2827)
                        };
                        let a2829 = 0u64;
                        if if if nat_eq(a2828, a2829) {
                            let a2834 = {
                                let a2831 = match v4.clone() {
                                    Adt2::C0(_, h2830, _, _, _, _, _, _, _, _, _) => {
                                        h2830
                                    }
                                };
                                let a2832 = v6;
                                let a2833 = 1u64;
                                nat_rem(a2831, a2832, a2833)
                            };
                            let a2835 = 0u64;
                            nat_eq(a2834, a2835)
                        } else {
                            false
                        } {
                            let a2840 = {
                                let a2837 = match v3.clone() {
                                    Adt2::C0(_, h2836, _, _, _, _, _, _, _, _, _) => {
                                        h2836
                                    }
                                };
                                let a2838 = v6;
                                let a2839 = 1u64;
                                nat_rem(a2837, a2838, a2839)
                            };
                            let a2841 = 0u64;
                            nat_eq(a2840, a2841)
                        } else {
                            false
                        } {
                            Ok(Adt10::C4(f16(v3.clone(), v6)?, match v5 {
                                Adt19::C0(_, _, h2842) => {
                                    h2842
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

pub fn f103(v0: u64, v1: Adt1, v2: Adt2, v3: Adt2) -> R<Adt10> {
    let a2844 = match v2.clone() {
        Adt2::C0(h2843, _, _, _, _, _, _, _, _, _, _) => {
            h2843
        }
    };
    let a2845 = 0u64;
    if if nat_eq(a2844, a2845) {
        let a2847 = match v3.clone() {
            Adt2::C0(h2846, _, _, _, _, _, _, _, _, _, _) => {
                h2846
            }
        };
        let a2848 = 0u64;
        nat_eq(a2847, a2848)
    } else {
        false
    } {
        let a2849 = v0;
        let a2850 = 61u64;
        if nat_eq(a2849, a2850) {
            let v4: bool = {
                let a2852 = 0u64;
                let a2853 = match v1.clone() {
                    Adt1::C0(_, _, _, _, _, _, _, _, h2851, _, _, _) => {
                        h2851
                    }
                };
                nat_lt(a2852, a2853)
            };
            if if if if if if f201(match v2.clone() {
                Adt2::C0(_, h2854, _, _, _, _, _, _, _, _, _) => {
                    h2854
                }
            }, match v1.clone() {
                Adt1::C0(h2855, _, _, _, _, _, _, _, _, _, _, _) => {
                    h2855
                }
            }, match v1.clone() {
                Adt1::C0(_, h2856, _, _, _, _, _, _, _, _, _, _) => {
                    h2856
                }
            }, match v3.clone() {
                Adt2::C0(_, h2857, _, _, _, _, _, _, _, _, _) => {
                    h2857
                }
            })? {
                f201(match v2.clone() {
                    Adt2::C0(_, _, h2858, _, _, _, _, _, _, _, _) => {
                        h2858
                    }
                }, match v1.clone() {
                    Adt1::C0(_, _, h2859, _, _, _, _, _, _, _, _, _) => {
                        h2859
                    }
                }, match v1.clone() {
                    Adt1::C0(_, _, _, h2860, _, _, _, _, _, _, _, _) => {
                        h2860
                    }
                }, match v3.clone() {
                    Adt2::C0(_, _, h2861, _, _, _, _, _, _, _, _) => {
                        h2861
                    }
                })?
            } else {
                false
            } {
                f201(match v2.clone() {
                    Adt2::C0(_, _, _, h2862, _, _, _, _, _, _, _) => {
                        h2862
                    }
                }, match v1.clone() {
                    Adt1::C0(_, _, _, _, h2863, _, _, _, _, _, _, _) => {
                        h2863
                    }
                }, match v1.clone() {
                    Adt1::C0(_, _, _, _, _, h2864, _, _, _, _, _, _) => {
                        h2864
                    }
                }, match v3.clone() {
                    Adt2::C0(_, _, _, h2865, _, _, _, _, _, _, _) => {
                        h2865
                    }
                })?
            } else {
                false
            } {
                f201(match v2.clone() {
                    Adt2::C0(_, _, _, _, h2866, _, _, _, _, _, _) => {
                        h2866
                    }
                }, match v1.clone() {
                    Adt1::C0(_, _, _, _, _, _, h2867, _, _, _, _, _) => {
                        h2867
                    }
                }, match v1.clone() {
                    Adt1::C0(_, _, _, _, _, _, _, h2868, _, _, _, _) => {
                        h2868
                    }
                }, match v3.clone() {
                    Adt2::C0(_, _, _, _, h2869, _, _, _, _, _, _) => {
                        h2869
                    }
                })?
            } else {
                false
            } {
                let a2871 = match v1.clone() {
                    Adt1::C0(_, _, _, _, _, _, _, _, h2870, _, _, _) => {
                        h2870
                    }
                };
                let a2872 = 4294967295u64;
                nat_le(a2871, a2872)
            } else {
                false
            } {
                if v4 {
                    !f34(match v2.clone() {
                        Adt2::C0(_, h2873, _, _, _, _, _, _, _, _, _) => {
                            h2873
                        }
                    }, match v2.clone() {
                        Adt2::C0(_, _, h2874, _, _, _, _, _, _, _, _) => {
                            h2874
                        }
                    }, match v2.clone() {
                        Adt2::C0(_, _, _, h2875, _, _, _, _, _, _, _) => {
                            h2875
                        }
                    }, match v2.clone() {
                        Adt2::C0(_, _, _, _, h2876, _, _, _, _, _, _) => {
                            h2876
                        }
                    })
                } else {
                    true
                }
            } else {
                false
            } {
                Ok(Adt10::C4(f16(v3.clone(), 1u64)?, 4u64, f20(v2, 1u64), f20(v3, 1u64), Adt4::C3(match v1.clone() {
                    Adt1::C0(h2877, _, _, _, _, _, _, _, _, _, _, _) => {
                        h2877
                    }
                }, match v1.clone() {
                    Adt1::C0(_, _, h2878, _, _, _, _, _, _, _, _, _) => {
                        h2878
                    }
                }, match v1.clone() {
                    Adt1::C0(_, _, _, _, h2879, _, _, _, _, _, _, _) => {
                        h2879
                    }
                }, match v1 {
                    Adt1::C0(_, _, _, _, _, _, h2880, _, _, _, _, _) => {
                        h2880
                    }
                }, v4)))
            } else {
                Ok(Adt10::C1)
            }
        } else {
            let a2881 = v0;
            let a2882 = 62u64;
            if nat_eq(a2881, a2882) {
                let a2884 = match v1.clone() {
                    Adt1::C0(h2883, _, _, _, _, _, _, _, _, _, _, _) => {
                        h2883
                    }
                };
                let a2885 = 2147483648u64;
                if if if if if nat_lt(a2884, a2885) {
                    let a2887 = match v1.clone() {
                        Adt1::C0(_, h2886, _, _, _, _, _, _, _, _, _, _) => {
                            h2886
                        }
                    };
                    let a2888 = 2147483648u64;
                    nat_lt(a2887, a2888)
                } else {
                    false
                } {
                    let a2891 = match v1.clone() {
                        Adt1::C0(h2889, _, _, _, _, _, _, _, _, _, _, _) => {
                            h2889
                        }
                    };
                    let a2892 = match v2.clone() {
                        Adt2::C0(_, h2890, _, _, _, _, _, _, _, _, _) => {
                            h2890
                        }
                    };
                    nat_lt(a2891, a2892)
                } else {
                    false
                } {
                    let a2895 = match v1.clone() {
                        Adt1::C0(_, h2893, _, _, _, _, _, _, _, _, _, _) => {
                            h2893
                        }
                    };
                    let a2896 = match v2.clone() {
                        Adt2::C0(_, h2894, _, _, _, _, _, _, _, _, _) => {
                            h2894
                        }
                    };
                    nat_lt(a2895, a2896)
                } else {
                    false
                } {
                    let a2899 = match v2.clone() {
                        Adt2::C0(_, _, h2897, _, _, _, _, _, _, _, _) => {
                            h2897
                        }
                    };
                    let a2900 = match v3.clone() {
                        Adt2::C0(_, _, h2898, _, _, _, _, _, _, _, _) => {
                            h2898
                        }
                    };
                    if if if nat_eq(a2899, a2900) {
                        let a2903 = match v2.clone() {
                            Adt2::C0(_, _, _, h2901, _, _, _, _, _, _, _) => {
                                h2901
                            }
                        };
                        let a2904 = match v3.clone() {
                            Adt2::C0(_, _, _, h2902, _, _, _, _, _, _, _) => {
                                h2902
                            }
                        };
                        nat_eq(a2903, a2904)
                    } else {
                        false
                    } {
                        let a2907 = match v2.clone() {
                            Adt2::C0(_, _, _, _, h2905, _, _, _, _, _, _) => {
                                h2905
                            }
                        };
                        let a2908 = match v3.clone() {
                            Adt2::C0(_, _, _, _, h2906, _, _, _, _, _, _) => {
                                h2906
                            }
                        };
                        nat_eq(a2907, a2908)
                    } else {
                        false
                    } {
                        let a2917 = match v3.clone() {
                            Adt2::C0(_, h2909, _, _, _, _, _, _, _, _, _) => {
                                h2909
                            }
                        };
                        let a2918 = {
                            let a2915 = {
                                let a2912 = match v2.clone() {
                                    Adt2::C0(_, h2910, _, _, _, _, _, _, _, _, _) => {
                                        h2910
                                    }
                                };
                                let a2913 = match v1.clone() {
                                    Adt1::C0(h2911, _, _, _, _, _, _, _, _, _, _, _) => {
                                        h2911
                                    }
                                };
                                nat_add(a2912, a2913)?
                            };
                            let a2916 = match v1.clone() {
                                Adt1::C0(_, h2914, _, _, _, _, _, _, _, _, _, _) => {
                                    h2914
                                }
                            };
                            nat_add(a2915, a2916)?
                        };
                        nat_eq(a2917, a2918)
                    } else {
                        false
                    }
                } else {
                    false
                } {
                    Ok(Adt10::C4(f16(v3.clone(), 1u64)?, 4u64, f20(v2, 1u64), f20(v3, 1u64), Adt4::C4(match v1 {
                        Adt1::C0(h2919, _, _, _, _, _, _, _, _, _, _, _) => {
                            h2919
                        }
                    })))
                } else {
                    Ok(Adt10::C1)
                }
            } else {
                let a2920 = v0;
                let a2921 = 63u64;
                if nat_eq(a2920, a2921) {
                    if if if if if f202(match v2.clone() {
                        Adt2::C0(_, h2922, _, _, _, _, _, _, _, _, _) => {
                            h2922
                        }
                    }, match v1.clone() {
                        Adt1::C0(h2923, _, _, _, _, _, _, _, _, _, _, _) => {
                            h2923
                        }
                    })? {
                        f202(match v2.clone() {
                            Adt2::C0(_, _, h2924, _, _, _, _, _, _, _, _) => {
                                h2924
                            }
                        }, match v1.clone() {
                            Adt1::C0(_, h2925, _, _, _, _, _, _, _, _, _, _) => {
                                h2925
                            }
                        })?
                    } else {
                        false
                    } {
                        f202(match v2.clone() {
                            Adt2::C0(_, _, _, h2926, _, _, _, _, _, _, _) => {
                                h2926
                            }
                        }, match v1.clone() {
                            Adt1::C0(_, _, h2927, _, _, _, _, _, _, _, _, _) => {
                                h2927
                            }
                        })?
                    } else {
                        false
                    } {
                        f202(match v2.clone() {
                            Adt2::C0(_, _, _, _, h2928, _, _, _, _, _, _) => {
                                h2928
                            }
                        }, match v1.clone() {
                            Adt1::C0(_, _, _, h2929, _, _, _, _, _, _, _, _) => {
                                h2929
                            }
                        })?
                    } else {
                        false
                    } {
                        let a2932 = match v2.clone() {
                            Adt2::C0(_, h2930, _, _, _, _, _, _, _, _, _) => {
                                h2930
                            }
                        };
                        let a2933 = match v3.clone() {
                            Adt2::C0(_, h2931, _, _, _, _, _, _, _, _, _) => {
                                h2931
                            }
                        };
                        if if if nat_eq(a2932, a2933) {
                            let a2936 = match v2.clone() {
                                Adt2::C0(_, _, h2934, _, _, _, _, _, _, _, _) => {
                                    h2934
                                }
                            };
                            let a2937 = match v3.clone() {
                                Adt2::C0(_, _, h2935, _, _, _, _, _, _, _, _) => {
                                    h2935
                                }
                            };
                            nat_eq(a2936, a2937)
                        } else {
                            false
                        } {
                            let a2940 = match v2.clone() {
                                Adt2::C0(_, _, _, h2938, _, _, _, _, _, _, _) => {
                                    h2938
                                }
                            };
                            let a2941 = match v3.clone() {
                                Adt2::C0(_, _, _, h2939, _, _, _, _, _, _, _) => {
                                    h2939
                                }
                            };
                            nat_eq(a2940, a2941)
                        } else {
                            false
                        } {
                            let a2944 = match v2.clone() {
                                Adt2::C0(_, _, _, _, h2942, _, _, _, _, _, _) => {
                                    h2942
                                }
                            };
                            let a2945 = match v3.clone() {
                                Adt2::C0(_, _, _, _, h2943, _, _, _, _, _, _) => {
                                    h2943
                                }
                            };
                            nat_eq(a2944, a2945)
                        } else {
                            false
                        }
                    } else {
                        false
                    } {
                        Ok(Adt10::C4(f16(v3.clone(), 1u64)?, 4u64, f20(v2, 1u64), f20(v3, 1u64), Adt4::C5(match v1.clone() {
                            Adt1::C0(h2946, _, _, _, _, _, _, _, _, _, _, _) => {
                                h2946
                            }
                        }, match v1.clone() {
                            Adt1::C0(_, h2947, _, _, _, _, _, _, _, _, _, _) => {
                                h2947
                            }
                        }, match v1.clone() {
                            Adt1::C0(_, _, h2948, _, _, _, _, _, _, _, _, _) => {
                                h2948
                            }
                        }, match v1 {
                            Adt1::C0(_, _, _, h2949, _, _, _, _, _, _, _, _) => {
                                h2949
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

pub fn f104(v0: u64, v1: Adt1, v2: Adt2, v3: Option<Adt2>, v4: Adt2) -> R<Adt10> {
    let m2950 = v3;
    match m2950 {
        None => {
            Ok(Adt10::C1)
        }
        Some(v5) => {
            let v6: bool = {
                let a2951 = v0;
                let a2952 = 4u64;
                nat_eq(a2951, a2952)
            };
            let v7: bool = {
                let a2954 = 0u64;
                let a2955 = match v1.clone() {
                    Adt1::C0(_, _, _, _, h2953, _, _, _, _, _, _, _) => {
                        h2953
                    }
                };
                nat_lt(a2954, a2955)
            };
            let v8: u64 = f16(v5.clone(), 1u64)?;
            let v9: Adt3 = Adt3::C0(match v5.clone() {
                Adt2::C0(_, h2956, _, _, _, _, _, _, _, _, _) => {
                    h2956
                }
            }, match v5.clone() {
                Adt2::C0(_, _, h2957, _, _, _, _, _, _, _, _) => {
                    h2957
                }
            }, match v5.clone() {
                Adt2::C0(_, _, _, h2958, _, _, _, _, _, _, _) => {
                    h2958
                }
            }, match v5.clone() {
                Adt2::C0(_, _, _, _, h2959, _, _, _, _, _, _) => {
                    h2959
                }
            }, 4u64, match v1.clone() {
                Adt1::C0(h2960, _, _, _, _, _, _, _, _, _, _, _) => {
                    h2960
                }
            }, match v1.clone() {
                Adt1::C0(_, h2961, _, _, _, _, _, _, _, _, _, _) => {
                    h2961
                }
            }, match v1.clone() {
                Adt1::C0(_, _, h2962, _, _, _, _, _, _, _, _, _) => {
                    h2962
                }
            });
            if if if if if if if if if if if if v6 {
                let a2964 = match v2.clone() {
                    Adt2::C0(h2963, _, _, _, _, _, _, _, _, _, _) => {
                        h2963
                    }
                };
                let a2965 = 0u64;
                if nat_eq(a2964, a2965) {
                    let a2967 = match v5.clone() {
                        Adt2::C0(h2966, _, _, _, _, _, _, _, _, _, _) => {
                            h2966
                        }
                    };
                    let a2968 = 0u64;
                    nat_eq(a2967, a2968)
                } else {
                    false
                }
            } else {
                let a2970 = match v2.clone() {
                    Adt2::C0(h2969, _, _, _, _, _, _, _, _, _, _) => {
                        h2969
                    }
                };
                let a2971 = 0u64;
                if if nat_eq(a2970, a2971) {
                    true
                } else {
                    let a2973 = match v2.clone() {
                        Adt2::C0(h2972, _, _, _, _, _, _, _, _, _, _) => {
                            h2972
                        }
                    };
                    let a2974 = 26u64;
                    nat_eq(a2973, a2974)
                } {
                    let a2976 = match v5.clone() {
                        Adt2::C0(h2975, _, _, _, _, _, _, _, _, _, _) => {
                            h2975
                        }
                    };
                    let a2977 = 0u64;
                    if nat_eq(a2976, a2977) {
                        true
                    } else {
                        let a2979 = match v5.clone() {
                            Adt2::C0(h2978, _, _, _, _, _, _, _, _, _, _) => {
                                h2978
                            }
                        };
                        let a2980 = 26u64;
                        nat_eq(a2979, a2980)
                    }
                } else {
                    false
                }
            } {
                let a2983 = match v2.clone() {
                    Adt2::C0(h2981, _, _, _, _, _, _, _, _, _, _) => {
                        h2981
                    }
                };
                let a2984 = match v4.clone() {
                    Adt2::C0(h2982, _, _, _, _, _, _, _, _, _, _) => {
                        h2982
                    }
                };
                nat_eq(a2983, a2984)
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
                let a2986 = match v5.clone() {
                    Adt2::C0(_, _, _, _, _, h2985, _, _, _, _, _) => {
                        h2985
                    }
                };
                let a2987 = 4u64;
                nat_eq(a2986, a2987)
            } else {
                false
            } {
                let a2988 = v8;
                let a2989 = f16(v2.clone(), 1u64)?;
                nat_le(a2988, a2989)
            } else {
                false
            } {
                let a2991 = match v1.clone() {
                    Adt1::C0(_, _, _, h2990, _, _, _, _, _, _, _, _) => {
                        h2990
                    }
                };
                let a2992 = 1073741824u64;
                nat_lt(a2991, a2992)
            } else {
                false
            } {
                let a2994 = match v1.clone() {
                    Adt1::C0(h2993, _, _, _, _, _, _, _, _, _, _, _) => {
                        h2993
                    }
                };
                let a2995 = 4294967295u64;
                if if if if nat_le(a2994, a2995) {
                    let a2997 = match v1.clone() {
                        Adt1::C0(_, h2996, _, _, _, _, _, _, _, _, _, _) => {
                            h2996
                        }
                    };
                    let a2998 = 4294967295u64;
                    nat_le(a2997, a2998)
                } else {
                    false
                } {
                    let a3000 = match v1.clone() {
                        Adt1::C0(_, _, h2999, _, _, _, _, _, _, _, _, _) => {
                            h2999
                        }
                    };
                    let a3001 = 4294967295u64;
                    nat_le(a3000, a3001)
                } else {
                    false
                } {
                    let a3003 = match v1.clone() {
                        Adt1::C0(_, _, _, h3002, _, _, _, _, _, _, _, _) => {
                            h3002
                        }
                    };
                    let a3004 = 4294967295u64;
                    nat_le(a3003, a3004)
                } else {
                    false
                } {
                    let a3006 = match v1.clone() {
                        Adt1::C0(_, _, _, _, h3005, _, _, _, _, _, _, _) => {
                            h3005
                        }
                    };
                    let a3007 = 4294967295u64;
                    nat_le(a3006, a3007)
                } else {
                    false
                }
            } else {
                false
            } {
                if v7 {
                    let a3010 = match v2.clone() {
                        Adt2::C0(_, _, _, _, _, _, _, _, _, _, h3008) => {
                            h3008
                        }
                    };
                    let a3011 = match v4.clone() {
                        Adt2::C0(_, _, _, _, _, _, _, _, _, _, h3009) => {
                            h3009
                        }
                    };
                    nat_eq(a3010, a3011)
                } else {
                    true
                }
            } else {
                false
            } {
                f203(v9.clone(), match v1.clone() {
                    Adt1::C0(_, _, _, h3012, _, _, _, _, _, _, _, _) => {
                        h3012
                    }
                }, match v4 {
                    Adt2::C0(_, _, _, _, _, _, _, _, _, h3013, _) => {
                        h3013
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
                    let a3014 = v10;
                    let a3015 = v8;
                    nat_add(a3014, a3015)?
                }, 4u64, f20(v5, 1u64), v9, Adt4::C6(v10, match v1 {
                    Adt1::C0(_, _, _, h3016, _, _, _, _, _, _, _, _) => {
                        h3016
                    }
                }, v6)))
            } else {
                Ok(Adt10::C1)
            }
        }
    }
}

pub fn f105(v0: Adt2, v1: Adt2) -> R<Adt10> {
    let a3018 = match v0.clone() {
        Adt2::C0(h3017, _, _, _, _, _, _, _, _, _, _) => {
            h3017
        }
    };
    let a3019 = 0u64;
    if if if nat_eq(a3018, a3019) {
        let a3021 = match v1.clone() {
            Adt2::C0(h3020, _, _, _, _, _, _, _, _, _, _) => {
                h3020
            }
        };
        let a3022 = 0u64;
        nat_eq(a3021, a3022)
    } else {
        false
    } {
        f199(match v0.clone() {
            Adt2::C0(_, h3023, _, _, _, _, _, _, _, _, _) => {
                h3023
            }
        }, match v0.clone() {
            Adt2::C0(_, _, h3024, _, _, _, _, _, _, _, _) => {
                h3024
            }
        }, match v0.clone() {
            Adt2::C0(_, _, _, h3025, _, _, _, _, _, _, _) => {
                h3025
            }
        }, match v0.clone() {
            Adt2::C0(_, _, _, _, h3026, _, _, _, _, _, _) => {
                h3026
            }
        }, match v1.clone() {
            Adt2::C0(_, h3027, _, _, _, _, _, _, _, _, _) => {
                h3027
            }
        }, match v1.clone() {
            Adt2::C0(_, _, h3028, _, _, _, _, _, _, _, _) => {
                h3028
            }
        }, match v1.clone() {
            Adt2::C0(_, _, _, h3029, _, _, _, _, _, _, _) => {
                h3029
            }
        }, match v1.clone() {
            Adt2::C0(_, _, _, _, h3030, _, _, _, _, _, _) => {
                h3030
            }
        })
    } else {
        false
    } {
        let v2: u64 = f16(v1.clone(), 1u64)?;
        Ok(Adt10::C4({
            let a3031 = v2;
            let a3032 = f16(v0.clone(), 1u64)?;
            nat_add(a3031, a3032)?
        }, 4u64, f20(v0, 1u64), f20(v1, 1u64), Adt4::C7(v2)))
    } else {
        Ok(Adt10::C1)
    }
}

pub fn f106(v0: u64, v1: Adt2, v2: Adt2) -> R<Adt10> {
    let a3034 = match v1.clone() {
        Adt2::C0(h3033, _, _, _, _, _, _, _, _, _, _) => {
            h3033
        }
    };
    let a3035 = 0u64;
    if if if nat_eq(a3034, a3035) {
        true
    } else {
        let a3037 = match v1.clone() {
            Adt2::C0(h3036, _, _, _, _, _, _, _, _, _, _) => {
                h3036
            }
        };
        let a3038 = 1u64;
        nat_eq(a3037, a3038)
    } {
        true
    } else {
        let a3040 = match v1.clone() {
            Adt2::C0(h3039, _, _, _, _, _, _, _, _, _, _) => {
                h3039
            }
        };
        let a3041 = 30u64;
        nat_eq(a3040, a3041)
    } {
        let a3044 = match v1.clone() {
            Adt2::C0(h3042, _, _, _, _, _, _, _, _, _, _) => {
                h3042
            }
        };
        let a3045 = match v2.clone() {
            Adt2::C0(h3043, _, _, _, _, _, _, _, _, _, _) => {
                h3043
            }
        };
        if if if if if nat_eq(a3044, a3045) {
            let a3048 = match v1.clone() {
                Adt2::C0(_, h3046, _, _, _, _, _, _, _, _, _) => {
                    h3046
                }
            };
            let a3049 = match v2.clone() {
                Adt2::C0(_, h3047, _, _, _, _, _, _, _, _, _) => {
                    h3047
                }
            };
            nat_eq(a3048, a3049)
        } else {
            false
        } {
            let a3052 = match v1.clone() {
                Adt2::C0(_, _, _, h3050, _, _, _, _, _, _, _) => {
                    h3050
                }
            };
            let a3053 = match v2.clone() {
                Adt2::C0(_, _, _, h3051, _, _, _, _, _, _, _) => {
                    h3051
                }
            };
            nat_eq(a3052, a3053)
        } else {
            false
        } {
            let a3056 = match v1.clone() {
                Adt2::C0(_, _, _, _, h3054, _, _, _, _, _, _) => {
                    h3054
                }
            };
            let a3057 = match v2.clone() {
                Adt2::C0(_, _, _, _, h3055, _, _, _, _, _, _) => {
                    h3055
                }
            };
            nat_eq(a3056, a3057)
        } else {
            false
        } {
            let a3058 = v0;
            let a3059 = 4u64;
            if nat_eq(a3058, a3059) {
                let a3061 = match v1.clone() {
                    Adt2::C0(_, _, h3060, _, _, _, _, _, _, _, _) => {
                        h3060
                    }
                };
                let a3062 = 1u64;
                if nat_eq(a3061, a3062) {
                    let a3065 = match v1.clone() {
                        Adt2::C0(_, h3063, _, _, _, _, _, _, _, _, _) => {
                            h3063
                        }
                    };
                    let a3066 = match v2.clone() {
                        Adt2::C0(_, _, h3064, _, _, _, _, _, _, _, _) => {
                            h3064
                        }
                    };
                    nat_eq(a3065, a3066)
                } else {
                    false
                }
            } else {
                let a3067 = v0;
                let a3068 = 4u64;
                if if nat_lt(a3067, a3068) {
                    let a3071 = match v1.clone() {
                        Adt2::C0(_, h3069, _, _, _, _, _, _, _, _, _) => {
                            h3069
                        }
                    };
                    let a3072 = match v1.clone() {
                        Adt2::C0(_, _, h3070, _, _, _, _, _, _, _, _) => {
                            h3070
                        }
                    };
                    nat_eq(a3071, a3072)
                } else {
                    false
                } {
                    let a3075 = match v1.clone() {
                        Adt2::C0(_, _, h3073, _, _, _, _, _, _, _, _) => {
                            h3073
                        }
                    };
                    let a3076 = match v2.clone() {
                        Adt2::C0(_, _, h3074, _, _, _, _, _, _, _, _) => {
                            h3074
                        }
                    };
                    nat_eq(a3075, a3076)
                } else {
                    false
                }
            }
        } else {
            false
        } {
            let m3078 = f187(match v1.clone() {
                Adt2::C0(h3077, _, _, _, _, _, _, _, _, _, _) => {
                    h3077
                }
            });
            match m3078 {
                Adt18::C0 => {
                    Ok(Adt10::C0)
                }
                Adt18::C1(v3) => {
                    Ok(Adt10::C4(f16(v2.clone(), 1u64)?, match v3 {
                        Adt19::C0(_, _, h3079) => {
                            h3079
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

pub fn f107(v0: u64, v1: Adt2, v2: Adt2) -> R<Adt10> {
    if if if f204(match v1.clone() {
        Adt2::C0(h3080, _, _, _, _, _, _, _, _, _, _) => {
            h3080
        }
    }, v0) {
        let a3083 = match v1.clone() {
            Adt2::C0(h3081, _, _, _, _, _, _, _, _, _, _) => {
                h3081
            }
        };
        let a3084 = match v2.clone() {
            Adt2::C0(h3082, _, _, _, _, _, _, _, _, _, _) => {
                h3082
            }
        };
        nat_eq(a3083, a3084)
    } else {
        false
    } {
        let a3087 = match v1.clone() {
            Adt2::C0(_, h3085, _, _, _, _, _, _, _, _, _) => {
                h3085
            }
        };
        let a3088 = match v2.clone() {
            Adt2::C0(_, h3086, _, _, _, _, _, _, _, _, _) => {
                h3086
            }
        };
        if if if nat_eq(a3087, a3088) {
            let a3091 = match v1.clone() {
                Adt2::C0(_, _, h3089, _, _, _, _, _, _, _, _) => {
                    h3089
                }
            };
            let a3092 = match v2.clone() {
                Adt2::C0(_, _, h3090, _, _, _, _, _, _, _, _) => {
                    h3090
                }
            };
            nat_eq(a3091, a3092)
        } else {
            false
        } {
            let a3095 = match v1.clone() {
                Adt2::C0(_, _, _, h3093, _, _, _, _, _, _, _) => {
                    h3093
                }
            };
            let a3096 = match v2.clone() {
                Adt2::C0(_, _, _, h3094, _, _, _, _, _, _, _) => {
                    h3094
                }
            };
            nat_eq(a3095, a3096)
        } else {
            false
        } {
            let a3099 = match v1.clone() {
                Adt2::C0(_, _, _, _, h3097, _, _, _, _, _, _) => {
                    h3097
                }
            };
            let a3100 = match v2.clone() {
                Adt2::C0(_, _, _, _, h3098, _, _, _, _, _, _) => {
                    h3098
                }
            };
            nat_eq(a3099, a3100)
        } else {
            false
        }
    } else {
        false
    } {
        let m3102 = f187(match v1.clone() {
            Adt2::C0(h3101, _, _, _, _, _, _, _, _, _, _) => {
                h3101
            }
        });
        match m3102 {
            Adt18::C0 => {
                Ok(Adt10::C0)
            }
            Adt18::C1(v3) => {
                Ok(Adt10::C4(f16(v2.clone(), 1u64)?, match v3 {
                    Adt19::C0(_, _, h3103) => {
                        h3103
                    }
                }, f20(v1.clone(), 1u64), f20(v2, 1u64), Adt4::C9(match v1 {
                    Adt2::C0(h3104, _, _, _, _, _, _, _, _, _, _) => {
                        h3104
                    }
                }, v0)))
            }
        }
    } else {
        Ok(Adt10::C1)
    }
}

pub fn f108(v0: bool, v1: Adt1, v2: Adt2, v3: Adt2) -> R<Adt10> {
    let v4: u64 = if v0 {
        match v1.clone() {
            Adt1::C0(h3105, _, _, _, _, _, _, _, _, _, _, _) => {
                h3105
            }
        }
    } else {
        match v1.clone() {
            Adt1::C0(_, _, h3106, _, _, _, _, _, _, _, _, _) => {
                h3106
            }
        }
    };
    let a3107 = 0u64;
    let a3108 = v4;
    if if nat_lt(a3107, a3108) {
        let a3109 = v4;
        let a3110 = 2147483648u64;
        nat_lt(a3109, a3110)
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
        let v7: u64 = f205(match v5.clone() {
            Adt2::C0(_, _, h3111, _, _, _, _, _, _, _, _) => {
                h3111
            }
        }, v4)?;
        let v8: u64 = f205(match v5.clone() {
            Adt2::C0(_, _, _, h3112, _, _, _, _, _, _, _) => {
                h3112
            }
        }, v4)?;
        let a3114 = match v5.clone() {
            Adt2::C0(h3113, _, _, _, _, _, _, _, _, _, _) => {
                h3113
            }
        };
        let a3115 = 0u64;
        if if if if if nat_eq(a3114, a3115) {
            let a3117 = match v6.clone() {
                Adt2::C0(h3116, _, _, _, _, _, _, _, _, _, _) => {
                    h3116
                }
            };
            let a3118 = 0u64;
            nat_eq(a3117, a3118)
        } else {
            false
        } {
            let a3120 = match v5.clone() {
                Adt2::C0(_, _, _, _, h3119, _, _, _, _, _, _) => {
                    h3119
                }
            };
            let a3121 = 1u64;
            nat_eq(a3120, a3121)
        } else {
            false
        } {
            let a3124 = match v5.clone() {
                Adt2::C0(_, h3122, _, _, _, _, _, _, _, _, _) => {
                    h3122
                }
            };
            let a3125 = match v6.clone() {
                Adt2::C0(_, h3123, _, _, _, _, _, _, _, _, _) => {
                    h3123
                }
            };
            nat_eq(a3124, a3125)
        } else {
            false
        } {
            if v0 {
                let a3127 = match v5.clone() {
                    Adt2::C0(_, _, h3126, _, _, _, _, _, _, _, _) => {
                        h3126
                    }
                };
                let a3128 = 2147483648u64;
                if if if nat_lt(a3127, a3128) {
                    let a3130 = match v5.clone() {
                        Adt2::C0(_, _, _, h3129, _, _, _, _, _, _, _) => {
                            h3129
                        }
                    };
                    let a3131 = 2147483648u64;
                    nat_lt(a3130, a3131)
                } else {
                    false
                } {
                    let a3134 = {
                        let a3132 = v7;
                        let a3133 = v8;
                        nat_mul(a3132, a3133)?
                    };
                    let a3135 = 2147483648u64;
                    nat_le(a3134, a3135)
                } else {
                    false
                } {
                    f206(v5, v6, v4, v7)?
                } else {
                    false
                }
            } else {
                let a3137 = match v1.clone() {
                    Adt1::C0(h3136, _, _, _, _, _, _, _, _, _, _, _) => {
                        h3136
                    }
                };
                let a3138 = v7;
                if if if if nat_eq(a3137, a3138) {
                    let a3140 = match v1 {
                        Adt1::C0(_, h3139, _, _, _, _, _, _, _, _, _, _) => {
                            h3139
                        }
                    };
                    let a3141 = v8;
                    nat_eq(a3140, a3141)
                } else {
                    false
                } {
                    f207(v7, v8, v4)?
                } else {
                    false
                } {
                    let a3143 = match v6.clone() {
                        Adt2::C0(_, _, h3142, _, _, _, _, _, _, _, _) => {
                            h3142
                        }
                    };
                    let a3144 = v4;
                    if nat_eq(a3143, a3144) {
                        let a3146 = match v6.clone() {
                            Adt2::C0(_, _, _, h3145, _, _, _, _, _, _, _) => {
                                h3145
                            }
                        };
                        let a3147 = v4;
                        nat_eq(a3146, a3147)
                    } else {
                        false
                    }
                } else {
                    false
                } {
                    let a3151 = match v6 {
                        Adt2::C0(_, _, _, _, h3148, _, _, _, _, _, _) => {
                            h3148
                        }
                    };
                    let a3152 = {
                        let a3149 = v7;
                        let a3150 = v8;
                        nat_mul(a3149, a3150)?
                    };
                    nat_eq(a3151, a3152)
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

pub fn f109(v0: u64, v1: Adt1, v2: Adt2, v3: Adt2) -> R<Adt10> {
    let a3154 = match v2.clone() {
        Adt2::C0(h3153, _, _, _, _, _, _, _, _, _, _) => {
            h3153
        }
    };
    let a3155 = 0u64;
    if if if if if nat_eq(a3154, a3155) {
        true
    } else {
        let a3157 = match v2.clone() {
            Adt2::C0(h3156, _, _, _, _, _, _, _, _, _, _) => {
                h3156
            }
        };
        let a3158 = 1u64;
        nat_eq(a3157, a3158)
    } {
        true
    } else {
        let a3160 = match v2.clone() {
            Adt2::C0(h3159, _, _, _, _, _, _, _, _, _, _) => {
                h3159
            }
        };
        let a3161 = 30u64;
        nat_eq(a3160, a3161)
    } {
        let a3164 = match v2.clone() {
            Adt2::C0(h3162, _, _, _, _, _, _, _, _, _, _) => {
                h3162
            }
        };
        let a3165 = match v3.clone() {
            Adt2::C0(h3163, _, _, _, _, _, _, _, _, _, _) => {
                h3163
            }
        };
        nat_eq(a3164, a3165)
    } else {
        false
    } {
        if if f25(v2.clone(), v3.clone()) {
            let a3167 = match v1.clone() {
                Adt1::C0(h3166, _, _, _, _, _, _, _, _, _, _, _) => {
                    h3166
                }
            };
            let a3168 = 4294967296u64;
            nat_lt(a3167, a3168)
        } else {
            false
        } {
            let a3170 = match v1.clone() {
                Adt1::C0(_, h3169, _, _, _, _, _, _, _, _, _, _) => {
                    h3169
                }
            };
            let a3171 = 4294967296u64;
            nat_lt(a3170, a3171)
        } else {
            false
        }
    } else {
        false
    } {
        Ok(Adt10::C4(f16(v3.clone(), 1u64)?, {
            let a3173 = match v2.clone() {
                Adt2::C0(h3172, _, _, _, _, _, _, _, _, _, _) => {
                    h3172
                }
            };
            let a3174 = 0u64;
            if nat_eq(a3173, a3174) {
                4u64
            } else {
                2u64
            }
        }, f20(v2.clone(), 1u64), f20(v3, 1u64), Adt4::C13({
            let a3175 = v0;
            let a3176 = 31u64;
            if nat_eq(a3175, a3176) {
                0u64
            } else {
                let a3177 = v0;
                let a3178 = 47u64;
                if nat_eq(a3177, a3178) {
                    1u64
                } else {
                    2u64
                }
            }
        }, match v2 {
            Adt2::C0(h3179, _, _, _, _, _, _, _, _, _, _) => {
                h3179
            }
        }, match v1.clone() {
            Adt1::C0(h3180, _, _, _, _, _, _, _, _, _, _, _) => {
                h3180
            }
        }, match v1 {
            Adt1::C0(_, h3181, _, _, _, _, _, _, _, _, _, _) => {
                h3181
            }
        })))
    } else {
        Ok(Adt10::C1)
    }
}

pub fn f110(v0: Adt2, v1: Adt2) -> R<Adt10> {
    let a3183 = match v0.clone() {
        Adt2::C0(h3182, _, _, _, _, _, _, _, _, _, _) => {
            h3182
        }
    };
    let a3184 = 0u64;
    if if if nat_eq(a3183, a3184) {
        let a3186 = match v1.clone() {
            Adt2::C0(h3185, _, _, _, _, _, _, _, _, _, _) => {
                h3185
            }
        };
        let a3187 = 0u64;
        nat_eq(a3186, a3187)
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

pub fn f111(v0: Adt2, v1: Adt2) -> R<Adt10> {
    let a3189 = match v0.clone() {
        Adt2::C0(h3188, _, _, _, _, _, _, _, _, _, _) => {
            h3188
        }
    };
    let a3190 = 0u64;
    if if nat_eq(a3189, a3190) {
        let a3192 = match v1.clone() {
            Adt2::C0(h3191, _, _, _, _, _, _, _, _, _, _) => {
                h3191
            }
        };
        let a3193 = 26u64;
        nat_eq(a3192, a3193)
    } else {
        false
    } {
        let a3195 = match v0.clone() {
            Adt2::C0(_, _, _, h3194, _, _, _, _, _, _, _) => {
                h3194
            }
        };
        let a3196 = 1u64;
        if if if if if if if nat_eq(a3195, a3196) {
            let a3198 = match v0.clone() {
                Adt2::C0(_, _, _, _, h3197, _, _, _, _, _, _) => {
                    h3197
                }
            };
            let a3199 = 1u64;
            nat_eq(a3198, a3199)
        } else {
            false
        } {
            let a3201 = match v0.clone() {
                Adt2::C0(_, h3200, _, _, _, _, _, _, _, _, _) => {
                    h3200
                }
            };
            let a3202 = 2147483647u64;
            nat_le(a3201, a3202)
        } else {
            false
        } {
            let a3205 = match v1.clone() {
                Adt2::C0(_, h3203, _, _, _, _, _, _, _, _, _) => {
                    h3203
                }
            };
            let a3206 = match v0.clone() {
                Adt2::C0(_, _, h3204, _, _, _, _, _, _, _, _) => {
                    h3204
                }
            };
            nat_eq(a3205, a3206)
        } else {
            false
        } {
            let a3208 = match v1.clone() {
                Adt2::C0(_, _, h3207, _, _, _, _, _, _, _, _) => {
                    h3207
                }
            };
            let a3209 = 1u64;
            nat_eq(a3208, a3209)
        } else {
            false
        } {
            let a3211 = match v1.clone() {
                Adt2::C0(_, _, _, h3210, _, _, _, _, _, _, _) => {
                    h3210
                }
            };
            let a3212 = 1u64;
            nat_eq(a3211, a3212)
        } else {
            false
        } {
            let a3214 = match v1.clone() {
                Adt2::C0(_, _, _, _, h3213, _, _, _, _, _, _) => {
                    h3213
                }
            };
            let a3215 = 1u64;
            nat_eq(a3214, a3215)
        } else {
            false
        } {
            Ok(Adt10::C4({
                let a3218 = match v0.clone() {
                    Adt2::C0(_, _, h3216, _, _, _, _, _, _, _, _) => {
                        h3216
                    }
                };
                let a3219 = f147(match v0.clone() {
                    Adt2::C0(_, h3217, _, _, _, _, _, _, _, _, _) => {
                        h3217
                    }
                })?;
                nat_mul(a3218, a3219)?
            }, 4u64, f20(v0, 1u64), f20(v1, 1u64), Adt4::C17))
        } else {
            Ok(Adt10::C1)
        }
    } else {
        Ok(Adt10::C0)
    }
}

pub fn f112(v0: Adt2, v1: Option<Adt2>, v2: Adt2) -> R<Adt10> {
    let m3220 = v1;
    match m3220 {
        None => {
            Ok(Adt10::C1)
        }
        Some(v3) => {
            let v4: u64 = f152(f20(v0.clone(), 1u64))?;
            let a3222 = match v0.clone() {
                Adt2::C0(h3221, _, _, _, _, _, _, _, _, _, _) => {
                    h3221
                }
            };
            let a3223 = 26u64;
            if if if nat_eq(a3222, a3223) {
                let a3225 = match v3.clone() {
                    Adt2::C0(h3224, _, _, _, _, _, _, _, _, _, _) => {
                        h3224
                    }
                };
                let a3226 = 26u64;
                nat_eq(a3225, a3226)
            } else {
                false
            } {
                let a3228 = match v2.clone() {
                    Adt2::C0(h3227, _, _, _, _, _, _, _, _, _, _) => {
                        h3227
                    }
                };
                let a3229 = 27u64;
                nat_eq(a3228, a3229)
            } else {
                false
            } {
                if if if if if if f25(v0.clone(), v3.clone()) {
                    let a3231 = match v2.clone() {
                        Adt2::C0(_, h3230, _, _, _, _, _, _, _, _, _) => {
                            h3230
                        }
                    };
                    let a3232 = 1u64;
                    nat_eq(a3231, a3232)
                } else {
                    false
                } {
                    let a3234 = match v2.clone() {
                        Adt2::C0(_, _, h3233, _, _, _, _, _, _, _, _) => {
                            h3233
                        }
                    };
                    let a3235 = 1u64;
                    nat_eq(a3234, a3235)
                } else {
                    false
                } {
                    let a3237 = match v2.clone() {
                        Adt2::C0(_, _, _, h3236, _, _, _, _, _, _, _) => {
                            h3236
                        }
                    };
                    let a3238 = 1u64;
                    nat_eq(a3237, a3238)
                } else {
                    false
                } {
                    let a3240 = match v2.clone() {
                        Adt2::C0(_, _, _, _, h3239, _, _, _, _, _, _) => {
                            h3239
                        }
                    };
                    let a3241 = 1u64;
                    nat_eq(a3240, a3241)
                } else {
                    false
                } {
                    let a3242 = v4;
                    let a3243 = 9223372036854775808u64;
                    nat_lt(a3242, a3243)
                } else {
                    false
                } {
                    Ok(Adt10::C4({
                        let a3246 = 2u64;
                        let a3247 = {
                            let a3244 = v4;
                            let a3245 = 2u64;
                            nat_mul(a3244, a3245)?
                        };
                        nat_add(a3246, a3247)?
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

pub fn f113(v0: u64, v1: Adt2, v2: Adt2) -> R<Adt10> {
    let v3: u64 = match v1.clone() {
        Adt2::C0(h3248, _, _, _, _, _, _, _, _, _, _) => {
            h3248
        }
    };
    let a3249 = v3;
    let a3250 = 0u64;
    if if nat_eq(a3249, a3250) {
        true
    } else {
        let a3251 = v0;
        let a3252 = 13u64;
        if nat_eq(a3251, a3252) {
            let a3253 = v3;
            let a3254 = 1u64;
            if nat_eq(a3253, a3254) {
                true
            } else {
                let a3255 = v3;
                let a3256 = 30u64;
                nat_eq(a3255, a3256)
            }
        } else {
            false
        }
    } {
        let a3258 = v3;
        let a3259 = match v2.clone() {
            Adt2::C0(h3257, _, _, _, _, _, _, _, _, _, _) => {
                h3257
            }
        };
        if if nat_eq(a3258, a3259) {
            let a3260 = v0;
            let a3261 = 13u64;
            if nat_eq(a3260, a3261) {
                let a3263 = match v2.clone() {
                    Adt2::C0(_, h3262, _, _, _, _, _, _, _, _, _) => {
                        h3262
                    }
                };
                let a3264 = 1u64;
                if if if nat_eq(a3263, a3264) {
                    let a3266 = match v2.clone() {
                        Adt2::C0(_, _, h3265, _, _, _, _, _, _, _, _) => {
                            h3265
                        }
                    };
                    let a3267 = 1u64;
                    nat_eq(a3266, a3267)
                } else {
                    false
                } {
                    let a3269 = match v2.clone() {
                        Adt2::C0(_, _, _, h3268, _, _, _, _, _, _, _) => {
                            h3268
                        }
                    };
                    let a3270 = 1u64;
                    nat_eq(a3269, a3270)
                } else {
                    false
                } {
                    let a3272 = match v2.clone() {
                        Adt2::C0(_, _, _, _, h3271, _, _, _, _, _, _) => {
                            h3271
                        }
                    };
                    let a3273 = 1u64;
                    nat_eq(a3272, a3273)
                } else {
                    false
                }
            } else {
                let a3275 = match v2.clone() {
                    Adt2::C0(_, h3274, _, _, _, _, _, _, _, _, _) => {
                        h3274
                    }
                };
                let a3276 = 1u64;
                if if if nat_eq(a3275, a3276) {
                    let a3279 = match v1.clone() {
                        Adt2::C0(_, _, h3277, _, _, _, _, _, _, _, _) => {
                            h3277
                        }
                    };
                    let a3280 = match v2.clone() {
                        Adt2::C0(_, _, h3278, _, _, _, _, _, _, _, _) => {
                            h3278
                        }
                    };
                    nat_eq(a3279, a3280)
                } else {
                    false
                } {
                    let a3283 = match v1.clone() {
                        Adt2::C0(_, _, _, h3281, _, _, _, _, _, _, _) => {
                            h3281
                        }
                    };
                    let a3284 = match v2.clone() {
                        Adt2::C0(_, _, _, h3282, _, _, _, _, _, _, _) => {
                            h3282
                        }
                    };
                    nat_eq(a3283, a3284)
                } else {
                    false
                } {
                    let a3287 = match v1.clone() {
                        Adt2::C0(_, _, _, _, h3285, _, _, _, _, _, _) => {
                            h3285
                        }
                    };
                    let a3288 = match v2.clone() {
                        Adt2::C0(_, _, _, _, h3286, _, _, _, _, _, _) => {
                            h3286
                        }
                    };
                    nat_eq(a3287, a3288)
                } else {
                    false
                }
            }
        } else {
            false
        } {
            let v4: Adt3 = f20(v1, 1u64);
            Ok(Adt10::C4(f208(v4.clone(), v0, v3)?, {
                let a3289 = v3;
                let a3290 = 0u64;
                if nat_eq(a3289, a3290) {
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

pub fn f114(v0: u64, v1: u64) -> R<Option<u64>> {
    let a3291 = v0;
    let a3292 = 89u64;
    if if nat_eq(a3291, a3292) {
        let a3293 = v1;
        let a3294 = 14u64;
        nat_eq(a3293, a3294)
    } else {
        false
    } {
        Ok(Some(23u64))
    } else {
        let a3295 = v0;
        let a3296 = 89u64;
        if if nat_eq(a3295, a3296) {
            let a3297 = v1;
            let a3298 = 15u64;
            nat_eq(a3297, a3298)
        } else {
            false
        } {
            Ok(Some(22u64))
        } else {
            let a3299 = v0;
            let a3300 = 89u64;
            if if nat_eq(a3299, a3300) {
                let a3301 = 8u64;
                let a3302 = v1;
                if nat_le(a3301, a3302) {
                    let a3303 = v1;
                    let a3304 = 9u64;
                    nat_le(a3303, a3304)
                } else {
                    false
                }
            } else {
                false
            } {
                Ok(Some({
                    let a3305 = v1;
                    let a3306 = 12u64;
                    nat_add(a3305, a3306)?
                }))
            } else {
                let a3307 = v0;
                let a3308 = 89u64;
                if if nat_eq(a3307, a3308) {
                    let a3309 = v1;
                    let a3310 = 4u64;
                    nat_eq(a3309, a3310)
                } else {
                    false
                } {
                    Ok(Some(19u64))
                } else {
                    let a3311 = 10u64;
                    let a3312 = v0;
                    if if nat_le(a3311, a3312) {
                        let a3313 = v0;
                        let a3314 = 12u64;
                        nat_le(a3313, a3314)
                    } else {
                        false
                    } {
                        Ok(Some({
                            let a3315 = v0;
                            let a3316 = 3u64;
                            nat_add(a3315, a3316)?
                        }))
                    } else {
                        let a3317 = v0;
                        let a3318 = 89u64;
                        if if nat_eq(a3317, a3318) {
                            let a3319 = v1;
                            let a3320 = 7u64;
                            nat_eq(a3319, a3320)
                        } else {
                            false
                        } {
                            Ok(Some(17u64))
                        } else {
                            let a3321 = v0;
                            let a3322 = 89u64;
                            if if nat_eq(a3321, a3322) {
                                let a3323 = v1;
                                let a3324 = 10u64;
                                nat_eq(a3323, a3324)
                            } else {
                                false
                            } {
                                Ok(Some(18u64))
                            } else {
                                let a3325 = v0;
                                let a3326 = 89u64;
                                if if nat_eq(a3325, a3326) {
                                    let a3327 = v1;
                                    let a3328 = 13u64;
                                    nat_eq(a3327, a3328)
                                } else {
                                    false
                                } {
                                    Ok(Some(16u64))
                                } else {
                                    let a3329 = v0;
                                    let a3330 = 8u64;
                                    if nat_eq(a3329, a3330) {
                                        Ok(Some(5u64))
                                    } else {
                                        let a3331 = v0;
                                        let a3332 = 9u64;
                                        if nat_eq(a3331, a3332) {
                                            Ok(Some(6u64))
                                        } else {
                                            let a3333 = v0;
                                            let a3334 = 89u64;
                                            if nat_eq(a3333, a3334) {
                                                let a3335 = v1;
                                                let a3336 = 0u64;
                                                if nat_eq(a3335, a3336) {
                                                    Ok(Some(0u64))
                                                } else {
                                                    let a3337 = v1;
                                                    let a3338 = 1u64;
                                                    if nat_eq(a3337, a3338) {
                                                        Ok(Some(1u64))
                                                    } else {
                                                        let a3339 = v1;
                                                        let a3340 = 2u64;
                                                        if nat_eq(a3339, a3340) {
                                                            Ok(Some(2u64))
                                                        } else {
                                                            let a3341 = v1;
                                                            let a3342 = 3u64;
                                                            if nat_eq(a3341, a3342) {
                                                                Ok(Some(3u64))
                                                            } else {
                                                                let a3343 = v1;
                                                                let a3344 = 6u64;
                                                                if nat_eq(a3343, a3344) {
                                                                    Ok(Some(4u64))
                                                                } else {
                                                                    let a3345 = v1;
                                                                    let a3346 = 12u64;
                                                                    if nat_eq(a3345, a3346) {
                                                                        Ok(Some(7u64))
                                                                    } else {
                                                                        let a3347 = v1;
                                                                        let a3348 = 11u64;
                                                                        if nat_eq(a3347, a3348) {
                                                                            Ok(Some(8u64))
                                                                        } else {
                                                                            let a3349 = v1;
                                                                            let a3350 = 18u64;
                                                                            if nat_eq(a3349, a3350) {
                                                                                Ok(Some(9u64))
                                                                            } else {
                                                                                let a3351 = v1;
                                                                                let a3352 = 19u64;
                                                                                if nat_eq(a3351, a3352) {
                                                                                    Ok(Some(10u64))
                                                                                } else {
                                                                                    let a3353 = v1;
                                                                                    let a3354 = 20u64;
                                                                                    if nat_eq(a3353, a3354) {
                                                                                        Ok(Some(11u64))
                                                                                    } else {
                                                                                        let a3355 = v1;
                                                                                        let a3356 = 21u64;
                                                                                        if nat_eq(a3355, a3356) {
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

pub fn f115(v0: u64, v1: u64, v2: Adt2, v3: Option<Adt2>, v4: Adt2) -> R<Adt10> {
    let m3357 = f209(v0, v1, v2.clone(), v3.clone(), v4.clone())?;
    match m3357 {
        Adt10::C0 => {
            Ok(Adt10::C0)
        }
        Adt10::C1 => {
            let a3358 = v0;
            let a3359 = 2u64;
            if if if if nat_eq(a3358, a3359) {
                true
            } else {
                let a3360 = v0;
                let a3361 = 5u64;
                nat_eq(a3360, a3361)
            } {
                true
            } else {
                let a3362 = v0;
                let a3363 = 6u64;
                nat_eq(a3362, a3363)
            } {
                true
            } else {
                let a3364 = v0;
                let a3365 = 7u64;
                nat_eq(a3364, a3365)
            } {
                Ok(Adt10::C5(67u64))
            } else {
                Ok(Adt10::C1)
            }
        }
        Adt10::C2(v7, v8, _, _, _, _, v13) => {
            if v8 {
                Ok(Adt10::C4(v13, 4u64, f20(v2.clone(), 1u64), f20(v4, 1u64), Adt4::C10(v7, v8, {
                    let m3366 = v3;
                    match m3366 {
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
        Adt10::C7 => {
            Ok(Adt10::C7)
        }
        Adt10::C8 => {
            Ok(Adt10::C8)
        }
    }
}

pub fn f116(v0: u64, v1: Adt2, v2: Adt2) -> R<Adt10> {
    let a3368 = match v1.clone() {
        Adt2::C0(h3367, _, _, _, _, _, _, _, _, _, _) => {
            h3367
        }
    };
    let a3369 = 0u64;
    if if if if if nat_eq(a3368, a3369) {
        true
    } else {
        let a3371 = match v1.clone() {
            Adt2::C0(h3370, _, _, _, _, _, _, _, _, _, _) => {
                h3370
            }
        };
        let a3372 = 1u64;
        nat_eq(a3371, a3372)
    } {
        true
    } else {
        let a3374 = match v1.clone() {
            Adt2::C0(h3373, _, _, _, _, _, _, _, _, _, _) => {
                h3373
            }
        };
        let a3375 = 30u64;
        nat_eq(a3374, a3375)
    } {
        let a3378 = match v1.clone() {
            Adt2::C0(h3376, _, _, _, _, _, _, _, _, _, _) => {
                h3376
            }
        };
        let a3379 = match v2.clone() {
            Adt2::C0(h3377, _, _, _, _, _, _, _, _, _, _) => {
                h3377
            }
        };
        if nat_eq(a3378, a3379) {
            true
        } else {
            let a3381 = match v2.clone() {
                Adt2::C0(h3380, _, _, _, _, _, _, _, _, _, _) => {
                    h3380
                }
            };
            let a3382 = 0u64;
            nat_eq(a3381, a3382)
        }
    } else {
        false
    } {
        f25(v1.clone(), v2.clone())
    } else {
        false
    } {
        Ok(Adt10::C4({
            let a3384 = f16(v2.clone(), 1u64)?;
            let a3385 = f210(v0, match v2.clone() {
                Adt2::C0(h3383, _, _, _, _, _, _, _, _, _, _) => {
                    h3383
                }
            });
            nat_mul(a3384, a3385)?
        }, {
            let a3387 = match v2.clone() {
                Adt2::C0(h3386, _, _, _, _, _, _, _, _, _, _) => {
                    h3386
                }
            };
            let a3388 = 0u64;
            if nat_eq(a3387, a3388) {
                4u64
            } else {
                2u64
            }
        }, f20(v1.clone(), 1u64), f20(v2.clone(), 1u64), Adt4::C14(v0, match v1 {
            Adt2::C0(h3389, _, _, _, _, _, _, _, _, _, _) => {
                h3389
            }
        }, match v2 {
            Adt2::C0(h3390, _, _, _, _, _, _, _, _, _, _) => {
                h3390
            }
        })))
    } else {
        Ok(Adt10::C1)
    }
}

pub fn f117(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64, v7: u64) -> R<u64> {
    let a3403 = {
        let a3399 = {
            let a3395 = {
                let a3391 = v0;
                let a3392 = v4;
                nat_mul(a3391, a3392)?
            };
            let a3396 = {
                let a3393 = v1;
                let a3394 = v5;
                nat_mul(a3393, a3394)?
            };
            nat_add(a3395, a3396)?
        };
        let a3400 = {
            let a3397 = v2;
            let a3398 = v6;
            nat_mul(a3397, a3398)?
        };
        nat_add(a3399, a3400)?
    };
    let a3404 = {
        let a3401 = v3;
        let a3402 = v7;
        nat_mul(a3401, a3402)?
    };
    nat_add(a3403, a3404)
}

pub fn f118(v0: u64) -> u64 {
    let a3405 = v0;
    let a3406 = 2147483648u64;
    if nat_lt(a3405, a3406) {
        0u64
    } else {
        let a3407 = 4294967296u64;
        let a3408 = v0;
        nat_sub(a3407, a3408)
    }
}

pub fn f119(v0: u64) -> u64 {
    let a3409 = v0;
    let a3410 = 2147483648u64;
    if nat_lt(a3409, a3410) {
        v0
    } else {
        0u64
    }
}

pub fn f120(v0: Adt11, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64, v7: u64, v8: u64) -> Adt9 {
    let v9: u64 = {
        let a3411 = v7;
        let a3412 = 6u64;
        if nat_eq(a3411, a3412) {
            27u64
        } else {
            v2
        }
    };
    Adt9::C7(v0, f69(v3, v4), f69(v5, v6), f70(v7, v8), f146(v1), f146(v9))
}

pub fn f121(v0: Adt4, v1: u64, v2: u64) -> Adt11 {
    let m3413 = v0;
    match m3413 {
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
            let a3414 = 13u64;
            let a3415 = v46;
            if nat_le(a3414, a3415) {
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
                Adt5::C0(_, _, _, _, _, h3416, _, _) => {
                    h3416
                }
            }, match v66 {
                Adt5::C0(_, _, _, _, _, _, _, h3417) => {
                    h3417
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

pub fn f122(v0: Adt4) -> u64 {
    let m3418 = v0;
    match m3418 {
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

pub fn f123(v0: Adt4, v1: u64) -> u64 {
    let m3419 = v0;
    match m3419 {
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
            let a3420 = v1;
            let a3421 = 6u64;
            if nat_eq(a3420, a3421) {
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
            let a3430 = v1;
            let a3431 = 6u64;
            if if if nat_eq(a3430, a3431) {
                true
            } else {
                let a3432 = v1;
                let a3433 = 7u64;
                nat_eq(a3432, a3433)
            } {
                let a3434 = v66;
                let a3435 = 30u64;
                nat_eq(a3434, a3435)
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
                Adt5::C0(_, _, _, _, _, _, _, h3429) => {
                    h3429
                }
            }
        }
        Adt4::C28(v61) => {
            let a3422 = v1;
            let a3423 = 6u64;
            if nat_eq(a3422, a3423) {
                match v61 {
                    Adt7::C0(_, _, _, _, h3424) => {
                        h3424
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
            let a3425 = v1;
            let a3426 = 6u64;
            if if nat_eq(a3425, a3426) {
                true
            } else {
                let a3427 = v1;
                let a3428 = 7u64;
                nat_eq(a3427, a3428)
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

pub fn f124(v0: Adt4, v1: u64) -> Adt11 {
    let a3436 = v1;
    let a3437 = 4u64;
    if nat_lt(a3436, a3437) {
        Adt11::C0(v1)
    } else {
        let a3438 = v1;
        let a3439 = 4u64;
        if nat_eq(a3438, a3439) {
            Adt11::C1(0u64)
        } else {
            let a3440 = v1;
            let a3441 = 5u64;
            if nat_eq(a3440, a3441) {
                Adt11::C1(3u64)
            } else {
                let a3442 = v1;
                let a3443 = 6u64;
                if nat_eq(a3442, a3443) {
                    Adt11::C2(f123(v0, v1), 0u64, false)
                } else {
                    let a3444 = v1;
                    let a3445 = 7u64;
                    if nat_eq(a3444, a3445) {
                        Adt11::C2(0u64, f123(v0, v1), false)
                    } else {
                        let a3446 = v1;
                        let a3447 = 16u64;
                        if nat_lt(a3446, a3447) {
                            Adt11::C1({
                                let a3448 = v1;
                                let a3449 = 8u64;
                                nat_sub(a3448, a3449)
                            })
                        } else {
                            let a3450 = v1;
                            let a3451 = 20u64;
                            if nat_lt(a3450, a3451) {
                                Adt11::C5({
                                    let a3452 = v1;
                                    let a3453 = 16u64;
                                    nat_sub(a3452, a3453)
                                })
                            } else {
                                let a3454 = v1;
                                let a3455 = 20u64;
                                if nat_eq(a3454, a3455) {
                                    Adt11::C3(0u64, 0u64, false)
                                } else {
                                    let a3456 = v1;
                                    let a3457 = 21u64;
                                    if nat_eq(a3456, a3457) {
                                        Adt11::C0(4u64)
                                    } else {
                                        let a3458 = v1;
                                        let a3459 = 22u64;
                                        if nat_eq(a3458, a3459) {
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

pub fn f125(v0: u64, v1: Adt3) -> R<u64> {
    let v2: u64 = {
        let a3461 = v0;
        let a3462 = match v1.clone() {
            Adt3::C0(h3460, _, _, _, _, _, _, _) => {
                h3460
            }
        };
        let a3463 = 0u64;
        nat_quot(a3461, a3462, a3463)
    };
    let v3: u64 = {
        let a3465 = v2;
        let a3466 = match v1.clone() {
            Adt3::C0(_, h3464, _, _, _, _, _, _) => {
                h3464
            }
        };
        let a3467 = 0u64;
        nat_quot(a3465, a3466, a3467)
    };
    f117({
        let a3469 = v0;
        let a3470 = match v1.clone() {
            Adt3::C0(h3468, _, _, _, _, _, _, _) => {
                h3468
            }
        };
        let a3471 = 0u64;
        nat_rem(a3469, a3470, a3471)
    }, {
        let a3473 = v2;
        let a3474 = match v1.clone() {
            Adt3::C0(_, h3472, _, _, _, _, _, _) => {
                h3472
            }
        };
        let a3475 = 0u64;
        nat_rem(a3473, a3474, a3475)
    }, {
        let a3477 = v3;
        let a3478 = match v1.clone() {
            Adt3::C0(_, _, h3476, _, _, _, _, _) => {
                h3476
            }
        };
        let a3479 = 0u64;
        nat_rem(a3477, a3478, a3479)
    }, {
        let a3481 = v3;
        let a3482 = match v1.clone() {
            Adt3::C0(_, _, h3480, _, _, _, _, _) => {
                h3480
            }
        };
        let a3483 = 0u64;
        nat_quot(a3481, a3482, a3483)
    }, match v1.clone() {
        Adt3::C0(_, _, _, _, h3484, _, _, _) => {
            h3484
        }
    }, match v1.clone() {
        Adt3::C0(_, _, _, _, _, h3485, _, _) => {
            h3485
        }
    }, match v1.clone() {
        Adt3::C0(_, _, _, _, _, _, h3486, _) => {
            h3486
        }
    }, match v1 {
        Adt3::C0(_, _, _, _, _, _, _, h3487) => {
            h3487
        }
    })
}

pub fn f126(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let a3505 = {
        let a3500 = f211(v2, {
            let a3491 = {
                let a3488 = v0;
                let a3489 = v1;
                let a3490 = 0u64;
                nat_quot(a3488, a3489, a3490)
            };
            let a3492 = v2;
            let a3493 = 0u64;
            nat_rem(a3491, a3492, a3493)
        }, {
            let a3497 = {
                let a3494 = v0;
                let a3495 = v1;
                let a3496 = 0u64;
                nat_quot(a3494, a3495, a3496)
            };
            let a3498 = v2;
            let a3499 = 0u64;
            nat_quot(a3497, a3498, a3499)
        })?;
        let a3501 = v1;
        nat_mul(a3500, a3501)?
    };
    let a3506 = {
        let a3502 = v0;
        let a3503 = v1;
        let a3504 = 0u64;
        nat_rem(a3502, a3503, a3504)
    };
    nat_add(a3505, a3506)
}

pub fn f127(v0: u64, v1: Adt3, v2: Adt3) -> R<u64> {
    let v3: u64 = {
        let a3508 = v0;
        let a3509 = match v2.clone() {
            Adt3::C0(h3507, _, _, _, _, _, _, _) => {
                h3507
            }
        };
        let a3510 = 0u64;
        nat_quot(a3508, a3509, a3510)
    };
    let v4: u64 = {
        let a3512 = v3;
        let a3513 = match v2.clone() {
            Adt3::C0(_, h3511, _, _, _, _, _, _) => {
                h3511
            }
        };
        let a3514 = 0u64;
        nat_quot(a3512, a3513, a3514)
    };
    f212({
        let a3516 = v0;
        let a3517 = match v2.clone() {
            Adt3::C0(h3515, _, _, _, _, _, _, _) => {
                h3515
            }
        };
        let a3518 = 0u64;
        nat_rem(a3516, a3517, a3518)
    }, {
        let a3520 = v3;
        let a3521 = match v2.clone() {
            Adt3::C0(_, h3519, _, _, _, _, _, _) => {
                h3519
            }
        };
        let a3522 = 0u64;
        nat_rem(a3520, a3521, a3522)
    }, {
        let a3524 = v4;
        let a3525 = match v2.clone() {
            Adt3::C0(_, _, h3523, _, _, _, _, _) => {
                h3523
            }
        };
        let a3526 = 0u64;
        nat_rem(a3524, a3525, a3526)
    }, {
        let a3528 = v4;
        let a3529 = match v2 {
            Adt3::C0(_, _, h3527, _, _, _, _, _) => {
                h3527
            }
        };
        let a3530 = 0u64;
        nat_quot(a3528, a3529, a3530)
    }, match v1.clone() {
        Adt3::C0(h3531, _, _, _, _, _, _, _) => {
            h3531
        }
    }, match v1.clone() {
        Adt3::C0(_, h3532, _, _, _, _, _, _) => {
            h3532
        }
    }, match v1.clone() {
        Adt3::C0(_, _, h3533, _, _, _, _, _) => {
            h3533
        }
    }, match v1.clone() {
        Adt3::C0(_, _, _, h3534, _, _, _, _) => {
            h3534
        }
    }, match v1.clone() {
        Adt3::C0(_, _, _, _, h3535, _, _, _) => {
            h3535
        }
    }, match v1.clone() {
        Adt3::C0(_, _, _, _, _, h3536, _, _) => {
            h3536
        }
    }, match v1.clone() {
        Adt3::C0(_, _, _, _, _, _, h3537, _) => {
            h3537
        }
    }, match v1 {
        Adt3::C0(_, _, _, _, _, _, _, h3538) => {
            h3538
        }
    })
}

pub fn f128(v0: u64, v1: Adt3, v2: Adt3, v3: Adt3, v4: u64) -> R<Adt9> {
    let v5: u64 = {
        let a3540 = v0;
        let a3541 = match v3.clone() {
            Adt3::C0(h3539, _, _, _, _, _, _, _) => {
                h3539
            }
        };
        let a3542 = 0u64;
        nat_quot(a3540, a3541, a3542)
    };
    let v6: u64 = {
        let a3544 = v5;
        let a3545 = match v3.clone() {
            Adt3::C0(_, h3543, _, _, _, _, _, _) => {
                h3543
            }
        };
        let a3546 = 0u64;
        nat_quot(a3544, a3545, a3546)
    };
    let v7: u64 = {
        let a3548 = v0;
        let a3549 = match v3.clone() {
            Adt3::C0(h3547, _, _, _, _, _, _, _) => {
                h3547
            }
        };
        let a3550 = 0u64;
        nat_rem(a3548, a3549, a3550)
    };
    let v8: u64 = {
        let a3552 = v5;
        let a3553 = match v3.clone() {
            Adt3::C0(_, h3551, _, _, _, _, _, _) => {
                h3551
            }
        };
        let a3554 = 0u64;
        nat_rem(a3552, a3553, a3554)
    };
    let v9: u64 = {
        let a3556 = v6;
        let a3557 = match v3.clone() {
            Adt3::C0(_, _, h3555, _, _, _, _, _) => {
                h3555
            }
        };
        let a3558 = 0u64;
        nat_rem(a3556, a3557, a3558)
    };
    let v10: u64 = {
        let a3560 = v6;
        let a3561 = match v3.clone() {
            Adt3::C0(_, _, h3559, _, _, _, _, _) => {
                h3559
            }
        };
        let a3562 = 0u64;
        nat_quot(a3560, a3561, a3562)
    };
    let v11: u64 = {
        let a3563 = v4;
        let a3564 = 0u64;
        if nat_eq(a3563, a3564) {
            match v1.clone() {
                Adt3::C0(h3565, _, _, _, _, _, _, _) => {
                    h3565
                }
            }
        } else {
            let a3566 = v4;
            let a3567 = 1u64;
            if nat_eq(a3566, a3567) {
                match v1.clone() {
                    Adt3::C0(_, h3568, _, _, _, _, _, _) => {
                        h3568
                    }
                }
            } else {
                let a3569 = v4;
                let a3570 = 2u64;
                if nat_eq(a3569, a3570) {
                    match v1.clone() {
                        Adt3::C0(_, _, h3571, _, _, _, _, _) => {
                            h3571
                        }
                    }
                } else {
                    match v1.clone() {
                        Adt3::C0(_, _, _, h3572, _, _, _, _) => {
                            h3572
                        }
                    }
                }
            }
        }
    };
    let v12: bool = {
        let a3579 = v11;
        let a3580 = {
            let a3573 = v4;
            let a3574 = 0u64;
            if nat_eq(a3573, a3574) {
                v7
            } else {
                let a3575 = v4;
                let a3576 = 1u64;
                if nat_eq(a3575, a3576) {
                    v8
                } else {
                    let a3577 = v4;
                    let a3578 = 2u64;
                    if nat_eq(a3577, a3578) {
                        v9
                    } else {
                        v10
                    }
                }
            }
        };
        nat_le(a3579, a3580)
    };
    let v13: Adt3 = if v12 {
        v2
    } else {
        v1
    };
    Ok(Adt9::C1(v12, f117(if if v12 {
        let a3581 = v4;
        let a3582 = 0u64;
        nat_eq(a3581, a3582)
    } else {
        false
    } {
        let a3583 = v7;
        let a3584 = v11;
        nat_sub(a3583, a3584)
    } else {
        v7
    }, if if v12 {
        let a3585 = v4;
        let a3586 = 1u64;
        nat_eq(a3585, a3586)
    } else {
        false
    } {
        let a3587 = v8;
        let a3588 = v11;
        nat_sub(a3587, a3588)
    } else {
        v8
    }, if if v12 {
        let a3589 = v4;
        let a3590 = 2u64;
        nat_eq(a3589, a3590)
    } else {
        false
    } {
        let a3591 = v9;
        let a3592 = v11;
        nat_sub(a3591, a3592)
    } else {
        v9
    }, if if v12 {
        let a3593 = v4;
        let a3594 = 3u64;
        nat_eq(a3593, a3594)
    } else {
        false
    } {
        let a3595 = v10;
        let a3596 = v11;
        nat_sub(a3595, a3596)
    } else {
        v10
    }, match v13.clone() {
        Adt3::C0(_, _, _, _, h3597, _, _, _) => {
            h3597
        }
    }, match v13.clone() {
        Adt3::C0(_, _, _, _, _, h3598, _, _) => {
            h3598
        }
    }, match v13.clone() {
        Adt3::C0(_, _, _, _, _, _, h3599, _) => {
            h3599
        }
    }, match v13 {
        Adt3::C0(_, _, _, _, _, _, _, h3600) => {
            h3600
        }
    })?, f125(v0, v3)?))
}

pub fn f129(v0: u64, v1: Adt3, v2: Adt3, v3: u64, v4: u64, v5: u64, v6: u64, v7: bool) -> R<Adt9> {
    let v8: u64 = f44(v0, v2.clone(), 0u64);
    let v9: u64 = f44(v0, v2.clone(), 1u64);
    let v10: u64 = f44(v0, v2.clone(), 2u64);
    let v11: u64 = f44(v0, v2.clone(), 3u64);
    if v7 {
        Ok(Adt9::C1(false, f43(v1.clone(), f213(v8, v3, match v1.clone() {
            Adt3::C0(h3601, _, _, _, _, _, _, _) => {
                h3601
            }
        })?, f213(v9, v4, match v1.clone() {
            Adt3::C0(_, h3602, _, _, _, _, _, _) => {
                h3602
            }
        })?, f213(v10, v5, match v1.clone() {
            Adt3::C0(_, _, h3603, _, _, _, _, _) => {
                h3603
            }
        })?, f213(v11, v6, match v1 {
            Adt3::C0(_, _, _, h3604, _, _, _, _) => {
                h3604
            }
        })?)?, f125(v0, v2)?))
    } else {
        if if if if f214(v8, v3, match v1.clone() {
            Adt3::C0(h3605, _, _, _, _, _, _, _) => {
                h3605
            }
        })? {
            f214(v9, v4, match v1.clone() {
                Adt3::C0(_, h3606, _, _, _, _, _, _) => {
                    h3606
                }
            })?
        } else {
            false
        } {
            f214(v10, v5, match v1.clone() {
                Adt3::C0(_, _, h3607, _, _, _, _, _) => {
                    h3607
                }
            })?
        } else {
            false
        } {
            f214(v11, v6, match v1.clone() {
                Adt3::C0(_, _, _, h3608, _, _, _, _) => {
                    h3608
                }
            })?
        } else {
            false
        } {
            Ok(Adt9::C1(false, f43(v1, f215(v8, v3)?, f215(v9, v4)?, f215(v10, v5)?, f215(v11, v6)?)?, f125(v0, v2)?))
        } else {
            Ok(Adt9::C1(true, 0u64, f125(v0, v2)?))
        }
    }
}

pub fn f130(v0: u64, v1: Adt3, v2: Adt3, v3: u64) -> R<Adt9> {
    let v4: u64 = f44(v0, v2.clone(), 0u64);
    let v5: u64 = f44(v0, v2.clone(), 1u64);
    let v6: u64 = f44(v0, v2.clone(), 2u64);
    let v7: u64 = f44(v0, v2.clone(), 3u64);
    Ok(Adt9::C1(false, f43(v1.clone(), f216(v4, v3, match v1 {
        Adt3::C0(h3609, _, _, _, _, _, _, _) => {
            h3609
        }
    })?, v5, v6, v7)?, f125(v0, v2)?))
}

pub fn f131(v0: u64, v1: Adt3, v2: Adt3, v3: u64, v4: u64, v5: bool) -> R<Adt9> {
    let a3610 = v0;
    let a3611 = v3;
    if nat_lt(a3610, a3611) {
        Ok(Adt9::C1(false, {
            let a3612 = v0;
            let a3613 = 4u64;
            nat_mul(a3612, a3613)?
        }, {
            let a3614 = v0;
            let a3615 = 4u64;
            nat_mul(a3614, a3615)?
        }))
    } else {
        let v6: u64 = {
            let a3616 = v0;
            let a3617 = v3;
            nat_sub(a3616, a3617)
        };
        let v7: u64 = f125(v6, v1)?;
        let v8: u64 = f217(v6, v2, v4)?;
        if v5 {
            Ok(Adt9::C4(8u64, 0u64, v8, 4u64, v7, 3u64, v8))
        } else {
            Ok(Adt9::C1(true, v7, v8))
        }
    }
}

pub fn f132(v0: u64, v1: Adt3, v2: Adt3, v3: u64) -> R<Adt9> {
    let a3618 = v0;
    let a3619 = v3;
    if nat_lt(a3618, a3619) {
        Ok(Adt9::C1(true, 0u64, f125(v0, v2)?))
    } else {
        let v4: u64 = {
            let a3620 = v0;
            let a3621 = v3;
            nat_sub(a3620, a3621)
        };
        let v5: u64 = f127(v4, v2, v1.clone())?;
        Ok(Adt9::C4(8u64, 3u64, v5, 0u64, f125(v4, v1)?, 3u64, v5))
    }
}

pub fn f133(v0: u64, v1: u64, v2: Adt3, v3: Adt3) -> R<Adt9> {
    let v4: u64 = {
        let a3623 = v1;
        let a3624 = match v3.clone() {
            Adt3::C0(h3622, _, _, _, _, _, _, _) => {
                h3622
            }
        };
        let a3625 = 0u64;
        nat_rem(a3623, a3624, a3625)
    };
    let v5: u64 = {
        let a3631 = {
            let a3627 = v1;
            let a3628 = match v3.clone() {
                Adt3::C0(h3626, _, _, _, _, _, _, _) => {
                    h3626
                }
            };
            let a3629 = 0u64;
            nat_quot(a3627, a3628, a3629)
        };
        let a3632 = match v3.clone() {
            Adt3::C0(_, h3630, _, _, _, _, _, _) => {
                h3630
            }
        };
        let a3633 = 0u64;
        nat_rem(a3631, a3632, a3633)
    };
    let v6: bool = f218(v0, v4, v5);
    Ok(Adt9::C1(!v6, if v6 {
        f127(v1, v2, v3.clone())?
    } else {
        0u64
    }, f125(v1, v3)?))
}

pub fn f134(v0: u64, v1: Adt3, v2: Adt3, v3: u64, v4: bool, v5: Adt3) -> R<Adt9> {
    Ok(Adt9::C4({
        let a3634 = v3;
        let a3635 = 8u64;
        nat_add(a3634, a3635)?
    }, 0u64, f125(v0, v1)?, if v4 {
        5u64
    } else {
        4u64
    }, if v4 {
        0u64
    } else {
        f127(v0, v5, v2.clone())?
    }, 3u64, f125(v0, v2)?))
}

pub fn f135(v0: u64, v1: Adt3, v2: Adt3, v3: u64, v4: u64) -> R<Adt9> {
    let a3636 = v4;
    let a3637 = 0u64;
    if if nat_eq(a3636, a3637) {
        f98(v3)
    } else {
        false
    } {
        let m3638 = f187(v3);
        match m3638 {
            Adt18::C0 => {
                Ok(Adt9::C0(Adt10::C1))
            }
            Adt18::C1(v5) => {
                let v6: u64 = match v5.clone() {
                    Adt19::C0(_, h3639, _) => {
                        h3639
                    }
                };
                Ok(Adt9::C5(v3, f125(v0, v1)?, match v5 {
                    Adt19::C0(_, _, h3640) => {
                        h3640
                    }
                }, f125({
                    let a3641 = v0;
                    let a3642 = v6;
                    nat_mul(a3641, a3642)?
                }, v2.clone())?, {
                    let a3648 = {
                        let a3646 = {
                            let a3643 = v6;
                            let a3644 = 1u64;
                            nat_sub(a3643, a3644)
                        };
                        let a3647 = match v2.clone() {
                            Adt3::C0(_, _, _, _, h3645, _, _, _) => {
                                h3645
                            }
                        };
                        nat_mul(a3646, a3647)?
                    };
                    let a3649 = 4u64;
                    nat_add(a3648, a3649)?
                }, match v2 {
                    Adt3::C0(_, _, _, _, h3650, _, _, _) => {
                        h3650
                    }
                }))
            }
        }
    } else {
        Ok(Adt9::C3(v3, v4, f125(v0, v1)?, f125(v0, v2)?))
    }
}

pub fn f136(v0: u64, v1: Adt3, v2: Adt3, v3: bool, v4: u64, v5: u64) -> R<Adt9> {
    let v6: u64 = {
        let a3652 = v0;
        let a3653 = match v2.clone() {
            Adt3::C0(h3651, _, _, _, _, _, _, _) => {
                h3651
            }
        };
        let a3654 = 0u64;
        nat_rem(a3652, a3653, a3654)
    };
    let v7: u64 = {
        let a3656 = v0;
        let a3657 = match v2.clone() {
            Adt3::C0(h3655, _, _, _, _, _, _, _) => {
                h3655
            }
        };
        let a3658 = 0u64;
        nat_quot(a3656, a3657, a3658)
    };
    let v8: u64 = {
        let a3660 = v7;
        let a3661 = match v2.clone() {
            Adt3::C0(_, h3659, _, _, _, _, _, _) => {
                h3659
            }
        };
        let a3662 = 0u64;
        nat_rem(a3660, a3661, a3662)
    };
    let v9: u64 = {
        let a3664 = v7;
        let a3665 = match v2.clone() {
            Adt3::C0(_, h3663, _, _, _, _, _, _) => {
                h3663
            }
        };
        let a3666 = 0u64;
        nat_quot(a3664, a3665, a3666)
    };
    let v10: u64 = {
        let a3668 = v9;
        let a3669 = match v2.clone() {
            Adt3::C0(_, _, h3667, _, _, _, _, _) => {
                h3667
            }
        };
        let a3670 = 0u64;
        nat_rem(a3668, a3669, a3670)
    };
    let v11: u64 = {
        let a3672 = v9;
        let a3673 = match v2.clone() {
            Adt3::C0(_, _, h3671, _, _, _, _, _) => {
                h3671
            }
        };
        let a3674 = 0u64;
        nat_quot(a3672, a3673, a3674)
    };
    let v12: u64 = f125(v0, v2)?;
    if v3 {
        let v13: u64 = {
            let a3683 = {
                let a3678 = {
                    let a3675 = v10;
                    let a3676 = v4;
                    let a3677 = 0u64;
                    nat_quot(a3675, a3676, a3677)
                };
                let a3679 = v5;
                nat_mul(a3678, a3679)?
            };
            let a3684 = {
                let a3680 = v8;
                let a3681 = v4;
                let a3682 = 0u64;
                nat_quot(a3680, a3681, a3682)
            };
            nat_add(a3683, a3684)?
        };
        let v14: u64 = {
            let a3704 = v6;
            let a3705 = {
                let a3702 = match v1.clone() {
                    Adt3::C0(h3685, _, _, _, _, _, _, _) => {
                        h3685
                    }
                };
                let a3703 = {
                    let a3700 = {
                        let a3686 = v8;
                        let a3687 = v4;
                        let a3688 = 0u64;
                        nat_rem(a3686, a3687, a3688)
                    };
                    let a3701 = {
                        let a3698 = match v1.clone() {
                            Adt3::C0(_, h3689, _, _, _, _, _, _) => {
                                h3689
                            }
                        };
                        let a3699 = {
                            let a3696 = {
                                let a3690 = v10;
                                let a3691 = v4;
                                let a3692 = 0u64;
                                nat_rem(a3690, a3691, a3692)
                            };
                            let a3697 = {
                                let a3694 = match v1.clone() {
                                    Adt3::C0(_, _, h3693, _, _, _, _, _) => {
                                        h3693
                                    }
                                };
                                let a3695 = v13;
                                nat_mul(a3694, a3695)?
                            };
                            nat_add(a3696, a3697)?
                        };
                        nat_mul(a3698, a3699)?
                    };
                    nat_add(a3700, a3701)?
                };
                nat_mul(a3702, a3703)?
            };
            nat_add(a3704, a3705)?
        };
        Ok(Adt9::C1(false, f125(v14, v1)?, v12))
    } else {
        let v15: u64 = {
            let a3711 = {
                let a3709 = {
                    let a3706 = v11;
                    let a3707 = v5;
                    let a3708 = 0u64;
                    nat_rem(a3706, a3707, a3708)
                };
                let a3710 = v4;
                nat_mul(a3709, a3710)?
            };
            let a3712 = v8;
            nat_add(a3711, a3712)?
        };
        let v16: u64 = {
            let a3718 = {
                let a3716 = {
                    let a3713 = v11;
                    let a3714 = v5;
                    let a3715 = 0u64;
                    nat_quot(a3713, a3714, a3715)
                };
                let a3717 = v4;
                nat_mul(a3716, a3717)?
            };
            let a3719 = v10;
            nat_add(a3718, a3719)?
        };
        let a3721 = v15;
        let a3722 = match v1.clone() {
            Adt3::C0(_, h3720, _, _, _, _, _, _) => {
                h3720
            }
        };
        if if nat_lt(a3721, a3722) {
            let a3724 = v16;
            let a3725 = match v1.clone() {
                Adt3::C0(_, _, h3723, _, _, _, _, _) => {
                    h3723
                }
            };
            nat_lt(a3724, a3725)
        } else {
            false
        } {
            Ok(Adt9::C1(false, f125({
                let a3734 = v6;
                let a3735 = {
                    let a3732 = match v1.clone() {
                        Adt3::C0(h3726, _, _, _, _, _, _, _) => {
                            h3726
                        }
                    };
                    let a3733 = {
                        let a3730 = v15;
                        let a3731 = {
                            let a3728 = match v1.clone() {
                                Adt3::C0(_, h3727, _, _, _, _, _, _) => {
                                    h3727
                                }
                            };
                            let a3729 = v16;
                            nat_mul(a3728, a3729)?
                        };
                        nat_add(a3730, a3731)?
                    };
                    nat_mul(a3732, a3733)?
                };
                nat_add(a3734, a3735)?
            }, v1)?, v12))
        } else {
            Ok(Adt9::C1(true, 0u64, v12))
        }
    }
}

pub fn f137(v0: u64, v1: Adt3, v2: Adt3, v3: u64, v4: u64) -> R<Adt9> {
    let v5: u64 = f210(23u64, v4);
    let v6: u64 = {
        let a3736 = v0;
        let a3737 = v5;
        let a3738 = 0u64;
        nat_rem(a3736, a3737, a3738)
    };
    let v7: u64 = {
        let a3739 = v0;
        let a3740 = v5;
        let a3741 = 0u64;
        nat_quot(a3739, a3740, a3741)
    };
    let v8: Adt13 = Adt13::C1(f125(v7, v2)?);
    let a3742 = v6;
    let a3743 = 0u64;
    if nat_eq(a3742, a3743) {
        Ok(Adt9::C7(Adt11::C7(16u64, v3, 0u64), Adt12::C0(0u64, f125(v7, v1)?), Adt12::C2(0u64), Adt13::C0(0u64), f146(v3), 4u64))
    } else {
        let a3744 = v6;
        let a3745 = 1u64;
        if nat_eq(a3744, a3745) {
            Ok(Adt9::C7(Adt11::C1(1u64), Adt12::C1(0u64), Adt12::C2(1065353216u64), {
                let a3746 = v4;
                let a3747 = 0u64;
                if nat_eq(a3746, a3747) {
                    v8
                } else {
                    Adt13::C0(0u64)
                }
            }, 4u64, 4u64))
        } else {
            Ok(Adt9::C7(Adt11::C2(0u64, v4, true), Adt12::C1(0u64), Adt12::C2(0u64), v8, 4u64, f146(v4)))
        }
    }
}

pub fn f138(v0: u64, v1: Adt3, v2: Adt3) -> R<Adt9> {
    let v3: u64 = f125(v0, v2.clone())?;
    let v4: u64 = f125(v0, v1.clone())?;
    let a3752 = {
        let a3749 = v0;
        let a3750 = match v1 {
            Adt3::C0(h3748, _, _, _, _, _, _, _) => {
                h3748
            }
        };
        let a3751 = 0u64;
        nat_rem(a3749, a3750, a3751)
    };
    let a3753 = 0u64;
    if nat_eq(a3752, a3753) {
        Ok(Adt9::C1(false, v4, v3))
    } else {
        Ok(Adt9::C4(8u64, 3u64, f125({
            let a3754 = v0;
            let a3755 = 1u64;
            nat_sub(a3754, a3755)
        }, v2)?, 0u64, v4, 3u64, v3))
    }
}

pub fn f139(v0: u64, v1: Adt3, v2: Adt3, v3: u64, v4: u64) -> R<Adt9> {
    let a3756 = v3;
    let a3757 = 13u64;
    if nat_eq(a3756, a3757) {
        f219(v0, v1, v4)
    } else {
        let v5: u64 = f220(match v1.clone() {
            Adt3::C0(h3758, _, _, _, _, _, _, _) => {
                h3758
            }
        }, {
            let a3759 = v3;
            let a3760 = 16u64;
            nat_eq(a3759, a3760)
        })?;
        let v6: u64 = {
            let a3761 = v0;
            let a3762 = v5;
            let a3763 = 0u64;
            nat_quot(a3761, a3762, a3763)
        };
        let v7: u64 = {
            let a3764 = v0;
            let a3765 = v5;
            let a3766 = 0u64;
            nat_rem(a3764, a3765, a3766)
        };
        let v8: u64 = {
            let a3770 = 1u64;
            let a3771 = {
                let a3768 = match v1.clone() {
                    Adt3::C0(h3767, _, _, _, _, _, _, _) => {
                        h3767
                    }
                };
                let a3769 = 2u64;
                nat_mul(a3768, a3769)?
            };
            nat_add(a3770, a3771)?
        };
        let a3772 = v7;
        let a3773 = v8;
        if nat_lt(a3772, a3773) {
            f221(v7, v6, v1, 0u64)
        } else {
            let v9: u64 = f125(v6, v2)?;
            let a3774 = v3;
            let a3775 = 14u64;
            if nat_eq(a3774, a3775) {
                Ok(Adt9::C4(2u64, 1u64, 1u64, 2u64, 0u64, 3u64, v9))
            } else {
                let a3776 = v7;
                let a3777 = v8;
                if nat_eq(a3776, a3777) {
                    Ok(Adt9::C4(2u64, 1u64, 1u64, 2u64, 0u64, 1u64, 1u64))
                } else {
                    let a3780 = v7;
                    let a3781 = {
                        let a3778 = v8;
                        let a3779 = 1u64;
                        nat_add(a3778, a3779)?
                    };
                    if nat_eq(a3780, a3781) {
                        Ok(Adt9::C4(3u64, 2u64, match v1 {
                            Adt3::C0(h3782, _, _, _, _, _, _, _) => {
                                h3782
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

pub fn f140(v0: u64, v1: Adt3, v2: Adt3, v3: u64) -> R<Adt9> {
    let a3783 = v0;
    let a3784 = 0u64;
    if nat_eq(a3783, a3784) {
        Ok(Adt9::C4(19u64, 2u64, 0u64, 2u64, 0u64, 1u64, 0u64))
    } else {
        let a3791 = {
            let a3785 = v0;
            let a3786 = 1u64;
            nat_add(a3785, a3786)?
        };
        let a3792 = {
            let a3789 = 2u64;
            let a3790 = {
                let a3787 = v3;
                let a3788 = 2u64;
                nat_mul(a3787, a3788)?
            };
            nat_add(a3789, a3790)?
        };
        if nat_eq(a3791, a3792) {
            Ok(Adt9::C4(19u64, 1u64, 0u64, 2u64, 0u64, 6u64, 0u64))
        } else {
            let v4: u64 = {
                let a3793 = v0;
                let a3794 = 1u64;
                nat_sub(a3793, a3794)
            };
            let v5: u64 = {
                let a3795 = v4;
                let a3796 = 2u64;
                let a3797 = 0u64;
                nat_quot(a3795, a3796, a3797)
            };
            let a3801 = {
                let a3798 = v4;
                let a3799 = 2u64;
                let a3800 = 0u64;
                nat_rem(a3798, a3799, a3800)
            };
            let a3802 = 0u64;
            if nat_eq(a3801, a3802) {
                Ok(Adt9::C4(17u64, 0u64, f125(v5, v1)?, 4u64, f125(v5, v2)?, 1u64, 2u64))
            } else {
                Ok(Adt9::C4(18u64, 1u64, 0u64, 1u64, 2u64, 1u64, 0u64))
            }
        }
    }
}

pub fn f141(v0: u64, v1: u64, v2: u64, v3: u64) -> R<Adt9> {
    let a3803 = v0;
    let a3804 = f27(v1, v2, v3)?;
    if nat_lt(a3803, a3804) {
        let a3805 = v0;
        let a3806 = v1;
        if nat_lt(a3805, a3806) {
            Ok(Adt9::C1(false, {
                let a3807 = 4u64;
                let a3808 = v0;
                nat_mul(a3807, a3808)?
            }, {
                let a3809 = 4u64;
                let a3810 = v0;
                nat_mul(a3809, a3810)?
            }))
        } else {
            let v4: u64 = {
                let a3811 = v0;
                let a3812 = v1;
                nat_sub(a3811, a3812)
            };
            let v5: u64 = f222(v4, v2)?;
            let a3813 = v5;
            let a3814 = 2u64;
            if nat_lt(a3813, a3814) {
                Ok(Adt9::C4(6u64, {
                    let a3815 = v5;
                    let a3816 = 0u64;
                    if nat_eq(a3815, a3816) {
                        4u64
                    } else {
                        5u64
                    }
                }, f223(v4, v2)?, 2u64, 0u64, 1u64, v5))
            } else {
                let v6: u64 = f224(v4, v2)?;
                Ok(Adt9::C4(8u64, 3u64, v6, 1u64, if f225(v4, v2)? {
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

pub fn f142(v0: u64, v1: u64) -> u64 {
    let a3817 = v0;
    let a3818 = 0u64;
    if if nat_eq(a3817, a3818) {
        true
    } else {
        let a3819 = v1;
        let a3820 = 0u64;
        nat_eq(a3819, a3820)
    } {
        1u64
    } else {
        2u64
    }
}

pub fn f143(v0: u64, v1: u64) -> Adt11 {
    let a3821 = v0;
    let a3822 = 43u64;
    if nat_eq(a3821, a3822) {
        Adt11::C8(3u64)
    } else {
        let a3823 = v1;
        let a3824 = 43u64;
        if nat_eq(a3823, a3824) {
            Adt11::C8(4u64)
        } else {
            Adt11::C2(v0, v1, true)
        }
    }
}

pub fn f144(v0: u64) -> R<u64> {
    let a3827 = 2u64;
    let a3828 = {
        let a3825 = 6u64;
        let a3826 = v0;
        nat_mul(a3825, a3826)?
    };
    nat_add(a3827, a3828)
}

pub fn f145(v0: u64, v1: bool) -> R<u64> {
    let a3831 = {
        let a3829 = v0;
        let a3830 = if v1 {
            3u64
        } else {
            2u64
        };
        nat_mul(a3829, a3830)?
    };
    let a3832 = 1u64;
    nat_add(a3831, a3832)
}

pub fn f146(v0: u64) -> u64 {
    let m3833 = f187(v0);
    match m3833 {
        Adt18::C0 => {
            0u64
        }
        Adt18::C1(v1) => {
            let a3835 = match v1.clone() {
                Adt19::C0(_, h3834, _) => {
                    h3834
                }
            };
            let a3836 = 1u64;
            if if nat_eq(a3835, a3836) {
                let a3838 = match v1.clone() {
                    Adt19::C0(_, _, h3837) => {
                        h3837
                    }
                };
                let a3839 = 1u64;
                if if if nat_eq(a3838, a3839) {
                    true
                } else {
                    let a3841 = match v1.clone() {
                        Adt19::C0(_, _, h3840) => {
                            h3840
                        }
                    };
                    let a3842 = 2u64;
                    nat_eq(a3841, a3842)
                } {
                    true
                } else {
                    let a3844 = match v1.clone() {
                        Adt19::C0(_, _, h3843) => {
                            h3843
                        }
                    };
                    let a3845 = 4u64;
                    nat_eq(a3844, a3845)
                } {
                    true
                } else {
                    let a3847 = match v1.clone() {
                        Adt19::C0(_, _, h3846) => {
                            h3846
                        }
                    };
                    let a3848 = 8u64;
                    nat_eq(a3847, a3848)
                }
            } else {
                false
            } {
                match v1 {
                    Adt19::C0(_, _, h3849) => {
                        h3849
                    }
                }
            } else {
                0u64
            }
        }
    }
}

pub fn f147(v0: u64) -> R<u64> {
    let a3852 = 3u64;
    let a3853 = {
        let a3850 = v0;
        let a3851 = 3u64;
        nat_mul(a3850, a3851)?
    };
    nat_add(a3852, a3853)
}

pub fn f148(v0: u64, v1: u64) -> R<u64> {
    if f149(v0) {
        let a3854 = 2u64;
        let a3855 = v1;
        nat_mul(a3854, a3855)
    } else {
        let a3856 = v1;
        let a3857 = f226(v0);
        let a3858 = 0u64;
        Ok(nat_quot(a3856, a3857, a3858))
    }
}

pub fn f149(v0: u64) -> bool {
    let a3859 = v0;
    let a3860 = 0u64;
    if if nat_eq(a3859, a3860) {
        true
    } else {
        let a3861 = v0;
        let a3862 = 1u64;
        nat_eq(a3861, a3862)
    } {
        true
    } else {
        let a3863 = v0;
        let a3864 = 30u64;
        nat_eq(a3863, a3864)
    }
}

pub fn f150(v0: u64, v1: u64, v2: u64, v3: u64, v4: Adt3, v5: Adt3) -> R<Adt9> {
    let m3865 = f187(v0);
    match m3865 {
        Adt18::C0 => {
            Ok(Adt9::C0(Adt10::C1))
        }
        Adt18::C1(v6) => {
            Ok(Adt9::C10(v0, f117(v2, v1, 0u64, 0u64, match v4.clone() {
                Adt3::C0(_, _, _, _, h3866, _, _, _) => {
                    h3866
                }
            }, match v4.clone() {
                Adt3::C0(_, _, _, _, _, h3867, _, _) => {
                    h3867
                }
            }, match v4.clone() {
                Adt3::C0(_, _, _, _, _, _, h3868, _) => {
                    h3868
                }
            }, match v4 {
                Adt3::C0(_, _, _, _, _, _, _, h3869) => {
                    h3869
                }
            })?, match v6.clone() {
                Adt19::C0(_, _, h3870) => {
                    h3870
                }
            }, f117({
                let a3872 = v2;
                let a3873 = match v6.clone() {
                    Adt19::C0(_, h3871, _) => {
                        h3871
                    }
                };
                nat_mul(a3872, a3873)?
            }, v3, 0u64, 0u64, match v5.clone() {
                Adt3::C0(_, _, _, _, h3874, _, _, _) => {
                    h3874
                }
            }, match v5.clone() {
                Adt3::C0(_, _, _, _, _, h3875, _, _) => {
                    h3875
                }
            }, match v5.clone() {
                Adt3::C0(_, _, _, _, _, _, h3876, _) => {
                    h3876
                }
            }, match v5.clone() {
                Adt3::C0(_, _, _, _, _, _, _, h3877) => {
                    h3877
                }
            })?, match v6 {
                Adt19::C0(_, h3878, _) => {
                    h3878
                }
            }, match v5 {
                Adt3::C0(_, _, _, _, h3879, _, _, _) => {
                    h3879
                }
            }))
        }
    }
}

pub fn f151(v0: Adt3, v1: Adt3) -> R<u64> {
    let a3887 = {
        let a3880 = 2u64;
        let a3881 = f152(v1.clone())?;
        nat_mul(a3880, a3881)?
    };
    let a3888 = {
        let a3885 = f152(v1)?;
        let a3886 = {
            let a3883 = 1u64;
            let a3884 = match v0 {
                Adt3::C0(h3882, _, _, _, _, _, _, _) => {
                    h3882
                }
            };
            nat_add(a3883, a3884)?
        };
        nat_mul(a3885, a3886)?
    };
    nat_add(a3887, a3888)
}

pub fn f152(v0: Adt3) -> R<u64> {
    if f34(match v0.clone() {
        Adt3::C0(h3889, _, _, _, _, _, _, _) => {
            h3889
        }
    }, match v0.clone() {
        Adt3::C0(_, h3890, _, _, _, _, _, _) => {
            h3890
        }
    }, match v0.clone() {
        Adt3::C0(_, _, h3891, _, _, _, _, _) => {
            h3891
        }
    }, match v0.clone() {
        Adt3::C0(_, _, _, h3892, _, _, _, _) => {
            h3892
        }
    }) {
        Ok(0u64)
    } else {
        let a3901 = match v0.clone() {
            Adt3::C0(h3893, _, _, _, _, _, _, _) => {
                h3893
            }
        };
        let a3902 = {
            let a3899 = match v0.clone() {
                Adt3::C0(_, h3894, _, _, _, _, _, _) => {
                    h3894
                }
            };
            let a3900 = {
                let a3897 = match v0.clone() {
                    Adt3::C0(_, _, h3895, _, _, _, _, _) => {
                        h3895
                    }
                };
                let a3898 = match v0 {
                    Adt3::C0(_, _, _, h3896, _, _, _, _) => {
                        h3896
                    }
                };
                nat_mul(a3897, a3898)?
            };
            nat_mul(a3899, a3900)?
        };
        nat_mul(a3901, a3902)
    }
}

pub fn f153(v0: u64, v1: Adt3) -> R<Adt9> {
    Ok(Adt9::C4(19u64, 4u64, f125(v0, v1)?, 2u64, 0u64, 1u64, 2u64))
}

pub fn f154(v0: u64, v1: u64) -> bool {
    let a3903 = v1;
    let a3904 = 2147483647u64;
    if nat_le(a3903, a3904) {
        let a3905 = v1;
        let a3906 = v0;
        nat_lt(a3905, a3906)
    } else {
        false
    }
}

pub fn f155(v0: u64, v1: u64, v2: u64, v3: Adt3, v4: Adt3, v5: Adt3, v6: u64) -> R<Adt9> {
    let m3907 = f187(v0);
    match m3907 {
        Adt18::C0 => {
            Ok(Adt9::C0(Adt10::C1))
        }
        Adt18::C1(v7) => {
            let v8: u64 = {
                let a3909 = v2;
                let a3910 = match v7.clone() {
                    Adt19::C0(_, h3908, _) => {
                        h3908
                    }
                };
                nat_mul(a3909, a3910)?
            };
            let m3923 = f227(v0, v8, v6, v1, match v5.clone() {
                Adt3::C0(h3911, _, _, _, _, _, _, _) => {
                    h3911
                }
            }, match v5.clone() {
                Adt3::C0(_, h3912, _, _, _, _, _, _) => {
                    h3912
                }
            }, match v5 {
                Adt3::C0(_, _, h3913, _, _, _, _, _) => {
                    h3913
                }
            }, {
                let a3916 = match v3.clone() {
                    Adt3::C0(h3914, _, _, _, _, _, _, _) => {
                        h3914
                    }
                };
                let a3917 = match v7.clone() {
                    Adt19::C0(_, h3915, _) => {
                        h3915
                    }
                };
                nat_mul(a3916, a3917)?
            }, match v3.clone() {
                Adt3::C0(_, h3918, _, _, _, _, _, _) => {
                    h3918
                }
            }, match v3.clone() {
                Adt3::C0(_, _, _, _, h3919, _, _, _) => {
                    h3919
                }
            }, match v3.clone() {
                Adt3::C0(_, _, _, _, _, h3920, _, _) => {
                    h3920
                }
            }, match v3.clone() {
                Adt3::C0(_, _, _, _, _, _, h3921, _) => {
                    h3921
                }
            }, match v3 {
                Adt3::C0(_, _, _, _, _, _, _, h3922) => {
                    h3922
                }
            })?;
            match m3923 {
                None => {
                    Ok(Adt9::C0(Adt10::C1))
                }
                Some(v9) => {
                    let a3924 = v0;
                    let a3925 = 0u64;
                    if if nat_eq(a3924, a3925) {
                        true
                    } else {
                        let a3926 = v0;
                        let a3927 = 26u64;
                        nat_eq(a3926, a3927)
                    } {
                        Ok(Adt9::C1(false, v9, f125({
                            let a3931 = {
                                let a3929 = v1;
                                let a3930 = match v4.clone() {
                                    Adt3::C0(h3928, _, _, _, _, _, _, _) => {
                                        h3928
                                    }
                                };
                                nat_mul(a3929, a3930)?
                            };
                            let a3932 = v8;
                            nat_add(a3931, a3932)?
                        }, v4)?))
                    } else {
                        Ok(Adt9::C5(v0, v9, match v7.clone() {
                            Adt19::C0(_, _, h3933) => {
                                h3933
                            }
                        }, f125({
                            let a3937 = {
                                let a3935 = v1;
                                let a3936 = match v4.clone() {
                                    Adt3::C0(h3934, _, _, _, _, _, _, _) => {
                                        h3934
                                    }
                                };
                                nat_mul(a3935, a3936)?
                            };
                            let a3938 = v8;
                            nat_add(a3937, a3938)?
                        }, v4.clone())?, {
                            let a3945 = {
                                let a3943 = {
                                    let a3940 = match v7 {
                                        Adt19::C0(_, h3939, _) => {
                                            h3939
                                        }
                                    };
                                    let a3941 = 1u64;
                                    nat_sub(a3940, a3941)
                                };
                                let a3944 = match v4.clone() {
                                    Adt3::C0(_, _, _, _, h3942, _, _, _) => {
                                        h3942
                                    }
                                };
                                nat_mul(a3943, a3944)?
                            };
                            let a3946 = 4u64;
                            nat_add(a3945, a3946)?
                        }, match v4 {
                            Adt3::C0(_, _, _, _, h3947, _, _, _) => {
                                h3947
                            }
                        }))
                    }
                }
            }
        }
    }
}

pub fn f156(v0: u64, v1: u64, v2: u64) -> u64 {
    let a3948 = v1;
    let a3949 = 0u64;
    if if nat_eq(a3948, a3949) {
        true
    } else {
        let a3950 = v2;
        let a3951 = 0u64;
        nat_eq(a3950, a3951)
    } {
        0u64
    } else {
        v0
    }
}

pub fn f157(v0: u64, v1: u64) -> bool {
    let a3952 = v0;
    let a3953 = 4294967295u64;
    if nat_eq(a3952, a3953) {
        true
    } else {
        let a3954 = v0;
        let a3955 = 2147483648u64;
        if nat_lt(a3954, a3955) {
            let a3956 = v0;
            let a3957 = v1;
            nat_lt(a3956, a3957)
        } else {
            false
        }
    }
}

pub fn f158(v0: Adt7) -> R<u64> {
    let a3969 = if match v0.clone() {
        Adt7::C0(_, _, _, h3958, _) => {
            h3958
        }
    } {
        2u64
    } else {
        3u64
    };
    let a3970 = {
        let a3967 = {
            let a3965 = {
                let m3961 = match v0.clone() {
                    Adt7::C0(h3960, _, _, _, _) => {
                        h3960
                    }
                };
                match m3961 {
                    Adt6::C0(h3959, _, _, _) => {
                        h3959
                    }
                }
            };
            let a3966 = {
                let m3964 = match v0.clone() {
                    Adt7::C0(_, h3963, _, _, _) => {
                        h3963
                    }
                };
                match m3964 {
                    Adt6::C0(h3962, _, _, _) => {
                        h3962
                    }
                }
            };
            nat_mul(a3965, a3966)?
        };
        let a3968 = f159(v0)?;
        nat_mul(a3967, a3968)?
    };
    nat_add(a3969, a3970)
}

pub fn f159(v0: Adt7) -> R<u64> {
    let a3975 = if match v0.clone() {
        Adt7::C0(_, _, _, h3971, _) => {
            h3971
        }
    } {
        2u64
    } else {
        1u64
    };
    let a3976 = {
        let a3973 = match v0 {
            Adt7::C0(_, _, _, _, h3972) => {
                h3972
            }
        };
        let a3974 = 1u64;
        if nat_eq(a3973, a3974) {
            1u64
        } else {
            0u64
        }
    };
    nat_add(a3975, a3976)
}

pub fn f160(v0: Adt6, v1: u64, v2: u64) -> R<u64> {
    let v3: u64 = {
        let a3981 = {
            let a3978 = v2;
            let a3979 = match v0.clone() {
                Adt6::C0(_, h3977, _, _) => {
                    h3977
                }
            };
            nat_mul(a3978, a3979)?
        };
        let a3982 = f118(match v0.clone() {
            Adt6::C0(_, _, h3980, _) => {
                h3980
            }
        });
        nat_add(a3981, a3982)?
    };
    let v4: u64 = f119(match v0.clone() {
        Adt6::C0(_, _, h3983, _) => {
            h3983
        }
    });
    let v5: u64 = {
        let a3987 = {
            let a3985 = v3;
            let a3986 = match v0 {
                Adt6::C0(h3984, _, _, _) => {
                    h3984
                }
            };
            nat_add(a3985, a3986)?
        };
        let a3988 = v4;
        nat_sub(a3987, a3988)
    };
    let a3993 = {
        let a3989 = v5;
        let a3990 = v1;
        if nat_lt(a3989, a3990) {
            v5
        } else {
            v1
        }
    };
    let a3994 = {
        let a3991 = v3;
        let a3992 = v4;
        nat_sub(a3991, a3992)
    };
    Ok(nat_sub(a3993, a3994))
}

pub fn f161(v0: Adt7, v1: Adt3, v2: Adt3, v3: u64, v4: u64) -> R<Option<u64>> {
    let m4003 = f46(match v0.clone() {
        Adt7::C0(h3995, _, _, _, _) => {
            h3995
        }
    }, match v1.clone() {
        Adt3::C0(h3996, _, _, _, _, _, _, _) => {
            h3996
        }
    }, f44(v3, v2.clone(), 0u64), {
        let a4000 = v4;
        let a4001 = {
            let m3999 = match v0.clone() {
                Adt7::C0(h3998, _, _, _, _) => {
                    h3998
                }
            };
            match m3999 {
                Adt6::C0(h3997, _, _, _) => {
                    h3997
                }
            }
        };
        let a4002 = 0u64;
        nat_rem(a4000, a4001, a4002)
    })?;
    match m4003 {
        None => {
            Ok(None::<u64>)
        }
        Some(v5) => {
            let m4012 = f46(match v0.clone() {
                Adt7::C0(_, h4004, _, _, _) => {
                    h4004
                }
            }, match v1.clone() {
                Adt3::C0(_, h4005, _, _, _, _, _, _) => {
                    h4005
                }
            }, f44(v3, v2.clone(), 1u64), {
                let a4009 = v4;
                let a4010 = {
                    let m4008 = match v0 {
                        Adt7::C0(h4007, _, _, _, _) => {
                            h4007
                        }
                    };
                    match m4008 {
                        Adt6::C0(h4006, _, _, _) => {
                            h4006
                        }
                    }
                };
                let a4011 = 0u64;
                nat_quot(a4009, a4010, a4011)
            })?;
            match m4012 {
                None => {
                    Ok(None::<u64>)
                }
                Some(v6) => {
                    Ok(Some(f43(v1, v5, v6, f44(v3, v2.clone(), 2u64), f44(v3, v2, 3u64))?))
                }
            }
        }
    }
}

pub fn f162(v0: u64) -> R<u64> {
    let a4015 = 2u64;
    let a4016 = {
        let a4013 = v0;
        let a4014 = 2u64;
        nat_mul(a4013, a4014)?
    };
    nat_add(a4015, a4016)
}

pub fn f163(v0: Adt6) -> R<u64> {
    let a4029 = 2u64;
    let a4030 = {
        let a4027 = 2u64;
        let a4028 = {
            let a4024 = {
                let a4021 = match v0.clone() {
                    Adt6::C0(h4017, _, _, _) => {
                        h4017
                    }
                };
                let a4022 = {
                    let a4019 = match v0.clone() {
                        Adt6::C0(_, h4018, _, _) => {
                            h4018
                        }
                    };
                    let a4020 = 1u64;
                    nat_sub(a4019, a4020)
                };
                nat_add(a4021, a4022)?
            };
            let a4025 = match v0 {
                Adt6::C0(_, h4023, _, _) => {
                    h4023
                }
            };
            let a4026 = 0u64;
            nat_quot(a4024, a4025, a4026)
        };
        nat_mul(a4027, a4028)?
    };
    nat_add(a4029, a4030)
}

pub fn f164(v0: Adt16, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64, v7: u64, v8: u64, v9: u64, v10: u64, v11: u64, v12: u64, v13: u64, v14: u64, v15: u64, v16: u64, v17: u64, v18: u64) -> R<Adt17> {
    let m4031 = v0;
    match m4031 {
        Adt16::C0(v19) => {
            f228(v1, v2, v3, v4, v5, v6, v7, v8, v9, v14, v15, v16, v17, v19, v18)
        }
        Adt16::C1(v20, v21, v22, v23) => {
            let a4032 = v20;
            let a4033 = 66u64;
            if nat_eq(a4032, a4033) {
                f229(v1, v2, v3, v4, v5, v6, v7, v8, v9, v10, v11, v12, v13, v14, v15, v16, v17, v21, v22, v23, v18)
            } else {
                f230(v20, v1, v2, v3, v4, v5, v6, v7, v8, v9, v14, v15, v16, v17, v21, v22, v23, v18)
            }
        }
    }
}

pub fn f165(v0: u64, v1: Adt3, v2: u64, v3: u64, v4: u64) -> R<Adt9> {
    let v5: u64 = {
        let a4040 = v3;
        let a4041 = {
            let a4037 = f125({
                let a4034 = v0;
                let a4035 = 2u64;
                let a4036 = 0u64;
                nat_quot(a4034, a4035, a4036)
            }, v1)?;
            let a4038 = v2;
            let a4039 = 0u64;
            nat_quot(a4037, a4038, a4039)
        };
        nat_add(a4040, a4041)?
    };
    let a4045 = {
        let a4042 = v0;
        let a4043 = 2u64;
        let a4044 = 0u64;
        nat_rem(a4042, a4043, a4044)
    };
    let a4046 = 0u64;
    if nat_eq(a4045, a4046) {
        Ok(Adt9::C4(19u64, 1u64, v5, 2u64, 0u64, 1u64, 2u64))
    } else {
        let a4047 = v4;
        let a4048 = 0u64;
        if nat_eq(a4047, a4048) {
            Ok(Adt9::C4(19u64, 2u64, 1u64, 2u64, 0u64, 1u64, v5))
        } else {
            Ok(Adt9::C0(Adt10::C1))
        }
    }
}

pub fn f166(v0: u64, v1: u64) -> R<u64> {
    let a4051 = 8u64;
    let a4052 = {
        let a4049 = f231(v0);
        let a4050 = v1;
        nat_mul(a4049, a4050)?
    };
    nat_add(a4051, a4052)
}

pub fn f167(v0: u64, v1: u64, v2: Adt3, v3: u64, v4: u64, v5: u64) -> R<Adt9> {
    let v6: u64 = match v2.clone() {
        Adt3::C0(h4053, _, _, _, _, _, _, _) => {
            h4053
        }
    };
    let v7: u64 = f231(v3);
    let v8: u64 = {
        let a4056 = 1u64;
        let a4057 = {
            let a4054 = v6;
            let a4055 = v7;
            nat_mul(a4054, a4055)?
        };
        nat_add(a4056, a4057)?
    };
    let a4058 = v0;
    let a4059 = 0u64;
    if nat_eq(a4058, a4059) {
        Ok(f232(0u64))
    } else {
        let a4060 = v0;
        let a4061 = v8;
        if nat_lt(a4060, a4061) {
            let v9: u64 = {
                let a4062 = v0;
                let a4063 = 1u64;
                nat_sub(a4062, a4063)
            };
            let v10: u64 = {
                let a4064 = v9;
                let a4065 = v7;
                let a4066 = 0u64;
                nat_rem(a4064, a4065, a4066)
            };
            let v11: u64 = f125({
                let a4072 = {
                    let a4067 = v1;
                    let a4068 = v6;
                    nat_mul(a4067, a4068)?
                };
                let a4073 = {
                    let a4069 = v9;
                    let a4070 = v7;
                    let a4071 = 0u64;
                    nat_quot(a4069, a4070, a4071)
                };
                nat_add(a4072, a4073)?
            }, v2)?;
            let a4074 = v3;
            let a4075 = 30u64;
            if nat_eq(a4074, a4075) {
                let a4076 = v10;
                let a4077 = 0u64;
                if nat_eq(a4076, a4077) {
                    Ok(Adt9::C4(6u64, 0u64, v11, 2u64, 0u64, 1u64, 1u64))
                } else {
                    let a4078 = v10;
                    let a4079 = 1u64;
                    if nat_eq(a4078, a4079) {
                        Ok(Adt9::C4(10u64, 1u64, 1u64, 1u64, 1u64, 1u64, 1u64))
                    } else {
                        let a4080 = v10;
                        let a4081 = 2u64;
                        if nat_eq(a4080, a4081) {
                            Ok(Adt9::C4(1u64, 1u64, 1u64, 2u64, 0u64, 1u64, 1u64))
                        } else {
                            Ok(Adt9::C4(0u64, 1u64, 0u64, 1u64, 1u64, 1u64, 0u64))
                        }
                    }
                }
            } else {
                let a4082 = v10;
                let a4083 = 0u64;
                if nat_eq(a4082, a4083) {
                    Ok(Adt9::C4(10u64, 0u64, v11, 0u64, v11, 1u64, 1u64))
                } else {
                    let a4084 = v10;
                    let a4085 = 1u64;
                    if nat_eq(a4084, a4085) {
                        Ok(Adt9::C4(1u64, 1u64, 1u64, 2u64, 0u64, 1u64, 1u64))
                    } else {
                        Ok(Adt9::C4(0u64, 1u64, 0u64, 1u64, 1u64, 1u64, 0u64))
                    }
                }
            }
        } else {
            let v12: u64 = {
                let a4086 = v0;
                let a4087 = v8;
                nat_sub(a4086, a4087)
            };
            let a4088 = v12;
            let a4089 = 0u64;
            if nat_eq(a4088, a4089) {
                Ok(Adt9::C4(22u64, 2u64, v6, 2u64, 0u64, 1u64, 1u64))
            } else {
                let a4090 = v12;
                let a4091 = 1u64;
                if nat_eq(a4090, a4091) {
                    Ok(Adt9::C4(21u64, 1u64, 0u64, 1u64, 1u64, 1u64, 1u64))
                } else {
                    let a4092 = v12;
                    let a4093 = 2u64;
                    if nat_eq(a4092, a4093) {
                        Ok(Adt9::C4(2u64, 1u64, 1u64, 2u64, 0u64, 1u64, 1u64))
                    } else {
                        let a4094 = v12;
                        let a4095 = 3u64;
                        if nat_eq(a4094, a4095) {
                            Ok(Adt9::C4(4u64, 1u64, 1u64, 2u64, v4, 1u64, 1u64))
                        } else {
                            let a4096 = v12;
                            let a4097 = 4u64;
                            if nat_eq(a4096, a4097) {
                                Ok(Adt9::C4(12u64, 1u64, 1u64, 2u64, 0u64, 1u64, 1u64))
                            } else {
                                let a4098 = v12;
                                let a4099 = 5u64;
                                if nat_eq(a4098, a4099) {
                                    Ok(Adt9::C4(11u64, 2u64, 1065353216u64, 1u64, 1u64, 1u64, 2u64))
                                } else {
                                    let a4100 = v12;
                                    let a4101 = 6u64;
                                    if nat_eq(a4100, a4101) {
                                        if f233(v5) {
                                            Ok(Adt9::C4(19u64, 1u64, 2u64, 2u64, 0u64, 1u64, {
                                                let a4102 = 3u64;
                                                let a4103 = v1;
                                                nat_add(a4102, a4103)?
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

pub fn f168(v0: u64, v1: Adt3, v2: Adt3, v3: u64) -> R<Adt9> {
    let v4: u64 = {
        let a4104 = v0;
        let a4105 = f81(v3);
        let a4106 = 0u64;
        nat_quot(a4104, a4105, a4106)
    };
    let v5: u64 = {
        let a4111 = 3u64;
        let a4112 = {
            let a4108 = v4;
            let a4109 = match v1.clone() {
                Adt3::C0(h4107, _, _, _, _, _, _, _) => {
                    h4107
                }
            };
            let a4110 = 0u64;
            nat_quot(a4108, a4109, a4110)
        };
        nat_add(a4111, a4112)?
    };
    let v6: u64 = f125(v4, v1)?;
    let v7: u64 = f125(v4, v2)?;
    let a4113 = v3;
    let a4114 = 30u64;
    if nat_eq(a4113, a4114) {
        let a4118 = {
            let a4115 = v0;
            let a4116 = 3u64;
            let a4117 = 0u64;
            nat_rem(a4115, a4116, a4117)
        };
        let a4119 = 0u64;
        if nat_eq(a4118, a4119) {
            Ok(Adt9::C4(6u64, 0u64, v6, 2u64, 0u64, 1u64, 1u64))
        } else {
            let a4123 = {
                let a4120 = v0;
                let a4121 = 3u64;
                let a4122 = 0u64;
                nat_rem(a4120, a4121, a4122)
            };
            let a4124 = 1u64;
            if nat_eq(a4123, a4124) {
                Ok(Adt9::C4(10u64, 1u64, 1u64, 1u64, v5, 1u64, 1u64))
            } else {
                Ok(Adt9::C4(7u64, 1u64, 1u64, 2u64, 0u64, 3u64, v7))
            }
        }
    } else {
        Ok(Adt9::C4(10u64, 0u64, v6, 1u64, v5, 3u64, v7))
    }
}

pub fn f169(v0: u64) -> R<u64> {
    let v1: u64 = {
        let a4125 = v0;
        let a4126 = 8388608u64;
        let a4127 = 0u64;
        nat_quot(a4125, a4126, a4127)
    };
    let a4128 = v0;
    let a4129 = 2147483648u64;
    if if if nat_lt(a4128, a4129) {
        let a4130 = 127u64;
        let a4131 = v1;
        nat_le(a4130, a4131)
    } else {
        false
    } {
        let a4132 = v1;
        let a4133 = 190u64;
        nat_lt(a4132, a4133)
    } else {
        false
    } {
        let v2: u64 = {
            let a4137 = 8388608u64;
            let a4138 = {
                let a4134 = v0;
                let a4135 = 8388608u64;
                let a4136 = 0u64;
                nat_rem(a4134, a4135, a4136)
            };
            nat_add(a4137, a4138)?
        };
        let a4139 = v1;
        let a4140 = 150u64;
        if nat_lt(a4139, a4140) {
            let v3: u64 = f234({
                let a4141 = 150u64;
                let a4142 = v1;
                nat_sub(a4141, a4142)
            });
            let a4146 = {
                let a4143 = v2;
                let a4144 = v3;
                let a4145 = 0u64;
                nat_rem(a4143, a4144, a4145)
            };
            let a4147 = 0u64;
            if nat_eq(a4146, a4147) {
                let a4148 = v2;
                let a4149 = v3;
                let a4150 = 0u64;
                Ok(nat_quot(a4148, a4149, a4150))
            } else {
                Ok(0u64)
            }
        } else {
            let a4153 = f234({
                let a4151 = v1;
                let a4152 = 150u64;
                nat_sub(a4151, a4152)
            });
            let a4154 = v2;
            nat_mul(a4153, a4154)
        }
    } else {
        Ok(0u64)
    }
}

pub fn f170(v0: Adt14, v1: Adt14) -> bool {
    let a4157 = match v0.clone() {
        Adt14::C0(_, h4155, _, _, _, _, _, _, _, _, _) => {
            h4155
        }
    };
    let a4158 = match v1.clone() {
        Adt14::C0(_, h4156, _, _, _, _, _, _, _, _, _) => {
            h4156
        }
    };
    if if if nat_eq(a4157, a4158) {
        let a4161 = match v0.clone() {
            Adt14::C0(_, _, h4159, _, _, _, _, _, _, _, _) => {
                h4159
            }
        };
        let a4162 = match v1.clone() {
            Adt14::C0(_, _, h4160, _, _, _, _, _, _, _, _) => {
                h4160
            }
        };
        nat_eq(a4161, a4162)
    } else {
        false
    } {
        let a4165 = match v0.clone() {
            Adt14::C0(_, _, _, h4163, _, _, _, _, _, _, _) => {
                h4163
            }
        };
        let a4166 = match v1.clone() {
            Adt14::C0(_, _, _, h4164, _, _, _, _, _, _, _) => {
                h4164
            }
        };
        nat_eq(a4165, a4166)
    } else {
        false
    } {
        let a4169 = match v0 {
            Adt14::C0(_, _, _, _, h4167, _, _, _, _, _, _) => {
                h4167
            }
        };
        let a4170 = match v1 {
            Adt14::C0(_, _, _, _, h4168, _, _, _, _, _, _) => {
                h4168
            }
        };
        nat_eq(a4169, a4170)
    } else {
        false
    }
}

pub fn f171(v0: u64, v1: u64, v2: u64, v3: u64) -> bool {
    let a4171 = v0;
    let a4172 = 0u64;
    if if if nat_eq(a4171, a4172) {
        true
    } else {
        let a4173 = v1;
        let a4174 = 0u64;
        nat_eq(a4173, a4174)
    } {
        true
    } else {
        let a4175 = v2;
        let a4176 = 0u64;
        nat_eq(a4175, a4176)
    } {
        true
    } else {
        let a4177 = v3;
        let a4178 = 0u64;
        nat_eq(a4177, a4178)
    }
}

pub fn f172(v0: Adt14, v1: Adt14) -> bool {
    if f178(match v0.clone() {
        Adt14::C0(_, _, _, _, _, h4179, _, _, _, _, _) => {
            h4179
        }
    }, match v1 {
        Adt14::C0(_, _, _, _, _, h4180, _, _, _, _, _) => {
            h4180
        }
    }) {
        let a4182 = match v0 {
            Adt14::C0(_, h4181, _, _, _, _, _, _, _, _, _) => {
                h4181
            }
        };
        let a4183 = 2147483647u64;
        nat_le(a4182, a4183)
    } else {
        true
    }
}

pub fn f173(v0: Adt14) -> bool {
    let a4188 = {
        let a4185 = match v0.clone() {
            Adt14::C0(_, _, _, _, _, _, _, _, _, _, h4184) => {
                h4184
            }
        };
        let a4186 = 4u64;
        let a4187 = 0u64;
        nat_rem(a4185, a4186, a4187)
    };
    let a4189 = 0u64;
    if if nat_eq(a4188, a4189) {
        f235(match v0.clone() {
            Adt14::C0(_, h4190, _, _, _, _, _, _, _, _, _) => {
                h4190
            }
        }, match v0.clone() {
            Adt14::C0(_, _, _, _, _, h4191, _, _, _, _, _) => {
                h4191
            }
        })
    } else {
        false
    } {
        f77(match v0.clone() {
            Adt14::C0(_, _, h4192, _, _, _, _, _, _, _, _) => {
                h4192
            }
        }, match v0.clone() {
            Adt14::C0(_, _, _, h4193, _, _, _, _, _, _, _) => {
                h4193
            }
        }, match v0.clone() {
            Adt14::C0(_, _, _, _, h4194, _, _, _, _, _, _) => {
                h4194
            }
        }, match v0.clone() {
            Adt14::C0(_, _, _, _, _, _, h4195, _, _, _, _) => {
                h4195
            }
        }, match v0.clone() {
            Adt14::C0(_, _, _, _, _, _, _, h4196, _, _, _) => {
                h4196
            }
        }, match v0 {
            Adt14::C0(_, _, _, _, _, _, _, _, h4197, _, _) => {
                h4197
            }
        }, 4u64)
    } else {
        false
    }
}

pub fn f174(v0: Adt14, v1: Adt14) -> R<bool> {
    let a4199 = match v0.clone() {
        Adt14::C0(_, _, _, _, _, _, _, _, _, h4198, _) => {
            h4198
        }
    };
    let a4200 = 0u64;
    if nat_eq(a4199, a4200) {
        Ok(true)
    } else {
        let a4202 = match v1.clone() {
            Adt14::C0(_, _, _, _, _, _, _, _, _, h4201, _) => {
                h4201
            }
        };
        let a4203 = 0u64;
        if nat_eq(a4202, a4203) {
            Ok(true)
        } else {
            let a4205 = match v0.clone() {
                Adt14::C0(_, _, _, _, _, _, _, _, _, _, h4204) => {
                    h4204
                }
            };
            let a4206 = 0u64;
            if nat_eq(a4205, a4206) {
                Ok(true)
            } else {
                let a4208 = match v1.clone() {
                    Adt14::C0(_, _, _, _, _, _, _, _, _, _, h4207) => {
                        h4207
                    }
                };
                let a4209 = 0u64;
                if nat_eq(a4208, a4209) {
                    Ok(true)
                } else {
                    let a4215 = {
                        let a4212 = match v0.clone() {
                            Adt14::C0(_, _, _, _, _, _, _, _, _, _, h4210) => {
                                h4210
                            }
                        };
                        let a4213 = match v0.clone() {
                            Adt14::C0(_, _, _, _, _, _, _, _, _, h4211, _) => {
                                h4211
                            }
                        };
                        nat_add(a4212, a4213)?
                    };
                    let a4216 = match v1.clone() {
                        Adt14::C0(_, _, _, _, _, _, _, _, _, _, h4214) => {
                            h4214
                        }
                    };
                    if nat_le(a4215, a4216) {
                        Ok(true)
                    } else {
                        let a4222 = {
                            let a4219 = match v1.clone() {
                                Adt14::C0(_, _, _, _, _, _, _, _, _, _, h4217) => {
                                    h4217
                                }
                            };
                            let a4220 = match v1 {
                                Adt14::C0(_, _, _, _, _, _, _, _, _, h4218, _) => {
                                    h4218
                                }
                            };
                            nat_add(a4219, a4220)?
                        };
                        let a4223 = match v0 {
                            Adt14::C0(_, _, _, _, _, _, _, _, _, _, h4221) => {
                                h4221
                            }
                        };
                        Ok(nat_le(a4222, a4223))
                    }
                }
            }
        }
    }
}

pub fn f175(v0: Adt14, v1: Adt14) -> bool {
    if if if if if if f170(v0.clone(), v1.clone()) {
        let a4226 = match v0.clone() {
            Adt14::C0(h4224, _, _, _, _, _, _, _, _, _, _) => {
                h4224
            }
        };
        let a4227 = match v1.clone() {
            Adt14::C0(h4225, _, _, _, _, _, _, _, _, _, _) => {
                h4225
            }
        };
        nat_eq(a4226, a4227)
    } else {
        false
    } {
        let a4230 = match v0.clone() {
            Adt14::C0(_, _, _, _, _, _, _, _, _, _, h4228) => {
                h4228
            }
        };
        let a4231 = match v1.clone() {
            Adt14::C0(_, _, _, _, _, _, _, _, _, _, h4229) => {
                h4229
            }
        };
        nat_eq(a4230, a4231)
    } else {
        false
    } {
        let a4234 = match v0.clone() {
            Adt14::C0(_, _, _, _, _, h4232, _, _, _, _, _) => {
                h4232
            }
        };
        let a4235 = match v1.clone() {
            Adt14::C0(_, _, _, _, _, h4233, _, _, _, _, _) => {
                h4233
            }
        };
        nat_eq(a4234, a4235)
    } else {
        false
    } {
        let a4238 = match v0.clone() {
            Adt14::C0(_, _, _, _, _, _, h4236, _, _, _, _) => {
                h4236
            }
        };
        let a4239 = match v1.clone() {
            Adt14::C0(_, _, _, _, _, _, h4237, _, _, _, _) => {
                h4237
            }
        };
        nat_eq(a4238, a4239)
    } else {
        false
    } {
        let a4242 = match v0.clone() {
            Adt14::C0(_, _, _, _, _, _, _, h4240, _, _, _) => {
                h4240
            }
        };
        let a4243 = match v1.clone() {
            Adt14::C0(_, _, _, _, _, _, _, h4241, _, _, _) => {
                h4241
            }
        };
        nat_eq(a4242, a4243)
    } else {
        false
    } {
        let a4246 = match v0 {
            Adt14::C0(_, _, _, _, _, _, _, _, h4244, _, _) => {
                h4244
            }
        };
        let a4247 = match v1 {
            Adt14::C0(_, _, _, _, _, _, _, _, h4245, _, _) => {
                h4245
            }
        };
        nat_eq(a4246, a4247)
    } else {
        false
    }
}

pub fn f176(v0: Adt14) -> R<bool> {
    if f236(match v0.clone() {
        Adt14::C0(_, h4248, _, _, _, _, _, _, _, _, _) => {
            h4248
        }
    }, match v0.clone() {
        Adt14::C0(_, _, _, _, _, h4249, _, _, _, _, _) => {
            h4249
        }
    }) {
        f78(f237(match v0.clone() {
            Adt14::C0(_, h4250, _, _, _, _, _, _, _, _, _) => {
                h4250
            }
        }, match v0.clone() {
            Adt14::C0(_, _, _, _, _, h4251, _, _, _, _, _) => {
                h4251
            }
        })?, match v0.clone() {
            Adt14::C0(_, _, h4252, _, _, _, _, _, _, _, _) => {
                h4252
            }
        }, match v0.clone() {
            Adt14::C0(_, _, _, h4253, _, _, _, _, _, _, _) => {
                h4253
            }
        }, match v0.clone() {
            Adt14::C0(_, _, _, _, h4254, _, _, _, _, _, _) => {
                h4254
            }
        }, match v0.clone() {
            Adt14::C0(_, _, _, _, _, _, h4255, _, _, _, _) => {
                h4255
            }
        }, match v0.clone() {
            Adt14::C0(_, _, _, _, _, _, _, h4256, _, _, _) => {
                h4256
            }
        }, match v0 {
            Adt14::C0(_, _, _, _, _, _, _, _, h4257, _, _) => {
                h4257
            }
        })
    } else {
        Ok(false)
    }
}

pub fn f177(v0: bool, v1: u64, v2: u64, v3: u64) -> R<u64> {
    let a4258 = v1;
    let a4259 = 0u64;
    if if nat_eq(a4258, a4259) {
        true
    } else {
        let a4260 = v2;
        let a4261 = 0u64;
        nat_eq(a4260, a4261)
    } {
        Ok(0u64)
    } else {
        let a4264 = f79(v1, v2, v3)?;
        let a4265 = {
            let a4262 = v2;
            let a4263 = f238(v0, v1)?;
            nat_mul(a4262, a4263)?
        };
        nat_add(a4264, a4265)
    }
}

pub fn f178(v0: u64, v1: u64) -> bool {
    let a4266 = v0;
    let a4267 = 4u64;
    if nat_eq(a4266, a4267) {
        let a4268 = v1;
        let a4269 = 4u64;
        nat_eq(a4268, a4269)
    } else {
        false
    }
}

pub fn f179(v0: u64) -> R<u64> {
    let a4270 = 6u64;
    let a4271 = v0;
    nat_add(a4270, a4271)
}

pub fn f180(v0: Adt14) -> Adt14 {
    Adt14::C0(match v0.clone() {
        Adt14::C0(h4272, _, _, _, _, _, _, _, _, _, _) => {
            h4272
        }
    }, match v0.clone() {
        Adt14::C0(_, h4273, _, _, _, _, _, _, _, _, _) => {
            h4273
        }
    }, match v0.clone() {
        Adt14::C0(_, _, h4274, _, _, _, _, _, _, _, _) => {
            h4274
        }
    }, match v0.clone() {
        Adt14::C0(_, _, _, h4275, _, _, _, _, _, _, _) => {
            h4275
        }
    }, match v0.clone() {
        Adt14::C0(_, _, _, _, h4276, _, _, _, _, _, _) => {
            h4276
        }
    }, 4u64, match v0.clone() {
        Adt14::C0(_, _, _, _, _, _, h4277, _, _, _, _) => {
            h4277
        }
    }, match v0.clone() {
        Adt14::C0(_, _, _, _, _, _, _, h4278, _, _, _) => {
            h4278
        }
    }, match v0.clone() {
        Adt14::C0(_, _, _, _, _, _, _, _, h4279, _, _) => {
            h4279
        }
    }, match v0.clone() {
        Adt14::C0(_, _, _, _, _, _, _, _, _, h4280, _) => {
            h4280
        }
    }, match v0 {
        Adt14::C0(_, _, _, _, _, _, _, _, _, _, h4281) => {
            h4281
        }
    })
}

pub fn f181(v0: u64, v1: u64, v2: u64, v3: u64) -> R<bool> {
    let a4282 = v0;
    let a4283 = 2147483647u64;
    if if if nat_le(a4282, a4283) {
        let a4284 = 0u64;
        let a4285 = v3;
        if nat_lt(a4284, a4285) {
            let a4286 = v3;
            let a4287 = 2147483647u64;
            nat_le(a4286, a4287)
        } else {
            false
        }
    } else {
        false
    } {
        let a4292 = {
            let a4290 = {
                let a4288 = v2;
                let a4289 = v3;
                nat_add(a4288, a4289)?
            };
            let a4291 = 1u64;
            nat_sub(a4290, a4291)
        };
        let a4293 = 2147483647u64;
        nat_le(a4292, a4293)
    } else {
        false
    } {
        let a4298 = {
            let a4296 = {
                let a4294 = v0;
                let a4295 = v1;
                nat_mul(a4294, a4295)?
            };
            let a4297 = f239(v2, v3)?;
            nat_mul(a4296, a4297)?
        };
        let a4299 = 9223372036854775807u64;
        Ok(nat_le(a4298, a4299))
    } else {
        Ok(false)
    }
}

pub fn f182(v0: Adt14) -> R<u64> {
    let a4330 = {
        let a4322 = {
            let a4314 = {
                let a4306 = 4u64;
                let a4307 = {
                    let a4304 = {
                        let a4301 = match v0.clone() {
                            Adt14::C0(_, h4300, _, _, _, _, _, _, _, _, _) => {
                                h4300
                            }
                        };
                        let a4302 = 1u64;
                        nat_sub(a4301, a4302)
                    };
                    let a4305 = match v0.clone() {
                        Adt14::C0(_, _, _, _, _, h4303, _, _, _, _, _) => {
                            h4303
                        }
                    };
                    nat_mul(a4304, a4305)?
                };
                nat_add(a4306, a4307)?
            };
            let a4315 = {
                let a4312 = {
                    let a4309 = match v0.clone() {
                        Adt14::C0(_, _, h4308, _, _, _, _, _, _, _, _) => {
                            h4308
                        }
                    };
                    let a4310 = 1u64;
                    nat_sub(a4309, a4310)
                };
                let a4313 = match v0.clone() {
                    Adt14::C0(_, _, _, _, _, _, h4311, _, _, _, _) => {
                        h4311
                    }
                };
                nat_mul(a4312, a4313)?
            };
            nat_add(a4314, a4315)?
        };
        let a4323 = {
            let a4320 = {
                let a4317 = match v0.clone() {
                    Adt14::C0(_, _, _, h4316, _, _, _, _, _, _, _) => {
                        h4316
                    }
                };
                let a4318 = 1u64;
                nat_sub(a4317, a4318)
            };
            let a4321 = match v0.clone() {
                Adt14::C0(_, _, _, _, _, _, _, h4319, _, _, _) => {
                    h4319
                }
            };
            nat_mul(a4320, a4321)?
        };
        nat_add(a4322, a4323)?
    };
    let a4331 = {
        let a4328 = {
            let a4325 = match v0.clone() {
                Adt14::C0(_, _, _, _, h4324, _, _, _, _, _, _) => {
                    h4324
                }
            };
            let a4326 = 1u64;
            nat_sub(a4325, a4326)
        };
        let a4329 = match v0 {
            Adt14::C0(_, _, _, _, _, _, _, _, h4327, _, _) => {
                h4327
            }
        };
        nat_mul(a4328, a4329)?
    };
    nat_add(a4330, a4331)
}

pub fn f183(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64) -> R<u64> {
    let a4332 = v0;
    let a4333 = 0u64;
    if if if if nat_eq(a4332, a4333) {
        true
    } else {
        let a4334 = v1;
        let a4335 = 0u64;
        nat_eq(a4334, a4335)
    } {
        true
    } else {
        let a4336 = v2;
        let a4337 = 0u64;
        nat_eq(a4336, a4337)
    } {
        true
    } else {
        let a4338 = v3;
        let a4339 = 0u64;
        nat_eq(a4338, a4339)
    } {
        Ok(0u64)
    } else {
        let a4340 = f240(v0, v1, v2, v3, v5)?;
        let a4341 = f241(v0, v1, v2, v3, v4)?;
        nat_add(a4340, a4341)
    }
}

pub fn f184(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let a4342 = v0;
    let a4343 = 0u64;
    if if nat_eq(a4342, a4343) {
        true
    } else {
        let a4344 = v1;
        let a4345 = 0u64;
        nat_eq(a4344, a4345)
    } {
        Ok(0u64)
    } else {
        let a4348 = f79(v0, v1, v2)?;
        let a4349 = {
            let a4346 = v1;
            let a4347 = f242(v0)?;
            nat_mul(a4346, a4347)?
        };
        nat_add(a4348, a4349)
    }
}

pub fn f185(v0: u64, v1: u64, v2: u64) -> bool {
    let a4350 = v0;
    let a4351 = 1u64;
    if nat_le(a4350, a4351) {
        true
    } else {
        let a4355 = {
            let a4352 = v1;
            let a4353 = v2;
            let a4354 = 1u64;
            nat_rem(a4352, a4353, a4354)
        };
        let a4356 = 0u64;
        nat_eq(a4355, a4356)
    }
}

pub fn f186(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64) -> R<bool> {
    let a4357 = v1;
    let a4358 = 1u64;
    if if if nat_le(a4357, a4358) {
        true
    } else {
        let a4359 = v0;
        let a4360 = v2;
        nat_le(a4359, a4360)
    } {
        let a4361 = v3;
        let a4362 = 1u64;
        if nat_le(a4361, a4362) {
            true
        } else {
            let a4369 = {
                let a4367 = v0;
                let a4368 = {
                    let a4365 = {
                        let a4363 = v1;
                        let a4364 = 1u64;
                        nat_sub(a4363, a4364)
                    };
                    let a4366 = v2;
                    nat_mul(a4365, a4366)?
                };
                nat_add(a4367, a4368)?
            };
            let a4370 = v4;
            nat_le(a4369, a4370)
        }
    } else {
        false
    } {
        let a4371 = v5;
        let a4372 = 1u64;
        if nat_le(a4371, a4372) {
            Ok(true)
        } else {
            let a4385 = {
                let a4383 = {
                    let a4377 = v0;
                    let a4378 = {
                        let a4375 = {
                            let a4373 = v1;
                            let a4374 = 1u64;
                            nat_sub(a4373, a4374)
                        };
                        let a4376 = v2;
                        nat_mul(a4375, a4376)?
                    };
                    nat_add(a4377, a4378)?
                };
                let a4384 = {
                    let a4381 = {
                        let a4379 = v3;
                        let a4380 = 1u64;
                        nat_sub(a4379, a4380)
                    };
                    let a4382 = v4;
                    nat_mul(a4381, a4382)?
                };
                nat_add(a4383, a4384)?
            };
            let a4386 = v6;
            Ok(nat_le(a4385, a4386))
        }
    } else {
        Ok(false)
    }
}

pub fn f187(v0: u64) -> Adt18 {
    let a4387 = v0;
    let a4388 = 0u64;
    if nat_eq(a4387, a4388) {
        Adt18::C1(Adt19::C0(0u64, 1u64, 4u64))
    } else {
        let a4389 = v0;
        let a4390 = 1u64;
        if nat_eq(a4389, a4390) {
            Adt18::C1(Adt19::C0(1u64, 1u64, 2u64))
        } else {
            let a4391 = v0;
            let a4392 = 2u64;
            if nat_eq(a4391, a4392) {
                Adt18::C1(Adt19::C0(2u64, 32u64, 18u64))
            } else {
                let a4393 = v0;
                let a4394 = 3u64;
                if nat_eq(a4393, a4394) {
                    Adt18::C1(Adt19::C0(3u64, 32u64, 20u64))
                } else {
                    let a4395 = v0;
                    let a4396 = 6u64;
                    if nat_eq(a4395, a4396) {
                        Adt18::C1(Adt19::C0(6u64, 32u64, 22u64))
                    } else {
                        let a4397 = v0;
                        let a4398 = 7u64;
                        if nat_eq(a4397, a4398) {
                            Adt18::C1(Adt19::C0(7u64, 32u64, 24u64))
                        } else {
                            let a4399 = v0;
                            let a4400 = 8u64;
                            if nat_eq(a4399, a4400) {
                                Adt18::C1(Adt19::C0(8u64, 32u64, 34u64))
                            } else {
                                let a4401 = v0;
                                let a4402 = 9u64;
                                if nat_eq(a4401, a4402) {
                                    Adt18::C1(Adt19::C0(9u64, 32u64, 36u64))
                                } else {
                                    let a4403 = v0;
                                    let a4404 = 10u64;
                                    if nat_eq(a4403, a4404) {
                                        Adt18::C1(Adt19::C0(10u64, 256u64, 84u64))
                                    } else {
                                        let a4405 = v0;
                                        let a4406 = 11u64;
                                        if nat_eq(a4405, a4406) {
                                            Adt18::C1(Adt19::C0(11u64, 256u64, 110u64))
                                        } else {
                                            let a4407 = v0;
                                            let a4408 = 12u64;
                                            if nat_eq(a4407, a4408) {
                                                Adt18::C1(Adt19::C0(12u64, 256u64, 144u64))
                                            } else {
                                                let a4409 = v0;
                                                let a4410 = 13u64;
                                                if nat_eq(a4409, a4410) {
                                                    Adt18::C1(Adt19::C0(13u64, 256u64, 176u64))
                                                } else {
                                                    let a4411 = v0;
                                                    let a4412 = 14u64;
                                                    if nat_eq(a4411, a4412) {
                                                        Adt18::C1(Adt19::C0(14u64, 256u64, 210u64))
                                                    } else {
                                                        let a4413 = v0;
                                                        let a4414 = 15u64;
                                                        if nat_eq(a4413, a4414) {
                                                            Adt18::C1(Adt19::C0(15u64, 256u64, 292u64))
                                                        } else {
                                                            let a4415 = v0;
                                                            let a4416 = 16u64;
                                                            if nat_eq(a4415, a4416) {
                                                                Adt18::C1(Adt19::C0(16u64, 256u64, 66u64))
                                                            } else {
                                                                let a4417 = v0;
                                                                let a4418 = 17u64;
                                                                if nat_eq(a4417, a4418) {
                                                                    Adt18::C1(Adt19::C0(17u64, 256u64, 74u64))
                                                                } else {
                                                                    let a4419 = v0;
                                                                    let a4420 = 18u64;
                                                                    if nat_eq(a4419, a4420) {
                                                                        Adt18::C1(Adt19::C0(18u64, 256u64, 98u64))
                                                                    } else {
                                                                        let a4421 = v0;
                                                                        let a4422 = 19u64;
                                                                        if nat_eq(a4421, a4422) {
                                                                            Adt18::C1(Adt19::C0(19u64, 256u64, 50u64))
                                                                        } else {
                                                                            let a4423 = v0;
                                                                            let a4424 = 20u64;
                                                                            if nat_eq(a4423, a4424) {
                                                                                Adt18::C1(Adt19::C0(20u64, 32u64, 18u64))
                                                                            } else {
                                                                                let a4425 = v0;
                                                                                let a4426 = 21u64;
                                                                                if nat_eq(a4425, a4426) {
                                                                                    Adt18::C1(Adt19::C0(21u64, 256u64, 110u64))
                                                                                } else {
                                                                                    let a4427 = v0;
                                                                                    let a4428 = 22u64;
                                                                                    if nat_eq(a4427, a4428) {
                                                                                        Adt18::C1(Adt19::C0(22u64, 256u64, 82u64))
                                                                                    } else {
                                                                                        let a4429 = v0;
                                                                                        let a4430 = 23u64;
                                                                                        if nat_eq(a4429, a4430) {
                                                                                            Adt18::C1(Adt19::C0(23u64, 256u64, 136u64))
                                                                                        } else {
                                                                                            let a4431 = v0;
                                                                                            let a4432 = 24u64;
                                                                                            if nat_eq(a4431, a4432) {
                                                                                                Adt18::C1(Adt19::C0(24u64, 1u64, 1u64))
                                                                                            } else {
                                                                                                let a4433 = v0;
                                                                                                let a4434 = 25u64;
                                                                                                if nat_eq(a4433, a4434) {
                                                                                                    Adt18::C1(Adt19::C0(25u64, 1u64, 2u64))
                                                                                                } else {
                                                                                                    let a4435 = v0;
                                                                                                    let a4436 = 26u64;
                                                                                                    if nat_eq(a4435, a4436) {
                                                                                                        Adt18::C1(Adt19::C0(26u64, 1u64, 4u64))
                                                                                                    } else {
                                                                                                        let a4437 = v0;
                                                                                                        let a4438 = 27u64;
                                                                                                        if nat_eq(a4437, a4438) {
                                                                                                            Adt18::C1(Adt19::C0(27u64, 1u64, 8u64))
                                                                                                        } else {
                                                                                                            let a4439 = v0;
                                                                                                            let a4440 = 28u64;
                                                                                                            if nat_eq(a4439, a4440) {
                                                                                                                Adt18::C1(Adt19::C0(28u64, 1u64, 8u64))
                                                                                                            } else {
                                                                                                                let a4441 = v0;
                                                                                                                let a4442 = 29u64;
                                                                                                                if nat_eq(a4441, a4442) {
                                                                                                                    Adt18::C1(Adt19::C0(29u64, 256u64, 56u64))
                                                                                                                } else {
                                                                                                                    let a4443 = v0;
                                                                                                                    let a4444 = 30u64;
                                                                                                                    if nat_eq(a4443, a4444) {
                                                                                                                        Adt18::C1(Adt19::C0(30u64, 1u64, 2u64))
                                                                                                                    } else {
                                                                                                                        let a4445 = v0;
                                                                                                                        let a4446 = 34u64;
                                                                                                                        if nat_eq(a4445, a4446) {
                                                                                                                            Adt18::C1(Adt19::C0(34u64, 256u64, 54u64))
                                                                                                                        } else {
                                                                                                                            let a4447 = v0;
                                                                                                                            let a4448 = 35u64;
                                                                                                                            if nat_eq(a4447, a4448) {
                                                                                                                                Adt18::C1(Adt19::C0(35u64, 256u64, 66u64))
                                                                                                                            } else {
                                                                                                                                let a4449 = v0;
                                                                                                                                let a4450 = 39u64;
                                                                                                                                if nat_eq(a4449, a4450) {
                                                                                                                                    Adt18::C1(Adt19::C0(39u64, 32u64, 17u64))
                                                                                                                                } else {
                                                                                                                                    let a4451 = v0;
                                                                                                                                    let a4452 = 40u64;
                                                                                                                                    if nat_eq(a4451, a4452) {
                                                                                                                                        Adt18::C1(Adt19::C0(40u64, 64u64, 36u64))
                                                                                                                                    } else {
                                                                                                                                        let a4453 = v0;
                                                                                                                                        let a4454 = 41u64;
                                                                                                                                        if nat_eq(a4453, a4454) {
                                                                                                                                            Adt18::C1(Adt19::C0(41u64, 128u64, 18u64))
                                                                                                                                        } else {
                                                                                                                                            let a4455 = v0;
                                                                                                                                            let a4456 = 42u64;
                                                                                                                                            if nat_eq(a4455, a4456) {
                                                                                                                                                Adt18::C1(Adt19::C0(42u64, 64u64, 18u64))
                                                                                                                                            } else {
                                                                                                                                                let a4457 = v0;
                                                                                                                                                let a4458 = 43u64;
                                                                                                                                                if nat_eq(a4457, a4458) {
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

pub fn f188(v0: Adt2, v1: u64) -> R<bool> {
    let a4459 = v1;
    let a4460 = 0u64;
    if nat_eq(a4459, a4460) {
        Ok(true)
    } else {
        let a4466 = {
            let a4463 = f125({
                let a4461 = v1;
                let a4462 = 1u64;
                nat_sub(a4461, a4462)
            }, f20(v0.clone(), 1u64))?;
            let a4464 = 4u64;
            nat_add(a4463, a4464)?
        };
        let a4467 = match v0 {
            Adt2::C0(_, _, _, _, _, _, _, _, _, h4465, _) => {
                h4465
            }
        };
        Ok(nat_le(a4466, a4467))
    }
}

pub fn f189(v0: Adt2) -> R<bool> {
    let v1: u64 = {
        let a4473 = match v0.clone() {
            Adt2::C0(_, _, h4468, _, _, _, _, _, _, _, _) => {
                h4468
            }
        };
        let a4474 = {
            let a4471 = match v0.clone() {
                Adt2::C0(_, _, _, h4469, _, _, _, _, _, _, _) => {
                    h4469
                }
            };
            let a4472 = match v0.clone() {
                Adt2::C0(_, _, _, _, h4470, _, _, _, _, _, _) => {
                    h4470
                }
            };
            nat_mul(a4471, a4472)?
        };
        nat_mul(a4473, a4474)?
    };
    let v2: u64 = f16(v0.clone(), 1u64)?;
    let a4476 = match v0.clone() {
        Adt2::C0(h4475, _, _, _, _, _, _, _, _, _, _) => {
            h4475
        }
    };
    let a4477 = 0u64;
    if if if if if if if nat_eq(a4476, a4477) {
        let a4479 = match v0.clone() {
            Adt2::C0(_, h4478, _, _, _, _, _, _, _, _, _) => {
                h4478
            }
        };
        let a4480 = 2147483647u64;
        nat_le(a4479, a4480)
    } else {
        false
    } {
        let a4481 = v1;
        let a4482 = 2147483647u64;
        nat_le(a4481, a4482)
    } else {
        false
    } {
        let a4484 = match v0.clone() {
            Adt2::C0(_, h4483, _, _, _, _, _, _, _, _, _) => {
                h4483
            }
        };
        let a4485 = 1u64;
        if nat_eq(a4484, a4485) {
            true
        } else {
            let a4487 = match v0.clone() {
                Adt2::C0(_, _, _, _, _, h4486, _, _, _, _, _) => {
                    h4486
                }
            };
            let a4488 = 4u64;
            nat_eq(a4487, a4488)
        }
    } else {
        false
    } {
        let a4489 = v1;
        let a4490 = 1u64;
        if nat_le(a4489, a4490) {
            true
        } else {
            let a4495 = {
                let a4492 = match v0.clone() {
                    Adt2::C0(_, h4491, _, _, _, _, _, _, _, _, _) => {
                        h4491
                    }
                };
                let a4493 = 4u64;
                nat_mul(a4492, a4493)?
            };
            let a4496 = match v0.clone() {
                Adt2::C0(_, _, _, _, _, _, h4494, _, _, _, _) => {
                    h4494
                }
            };
            nat_le(a4495, a4496)
        }
    } else {
        false
    } {
        let a4498 = match v0.clone() {
            Adt2::C0(_, _, _, h4497, _, _, _, _, _, _, _) => {
                h4497
            }
        };
        let a4499 = 1u64;
        if nat_eq(a4498, a4499) {
            true
        } else {
            let a4505 = match v0.clone() {
                Adt2::C0(_, _, _, _, _, _, _, h4500, _, _, _) => {
                    h4500
                }
            };
            let a4506 = {
                let a4503 = match v0.clone() {
                    Adt2::C0(_, _, h4501, _, _, _, _, _, _, _, _) => {
                        h4501
                    }
                };
                let a4504 = match v0.clone() {
                    Adt2::C0(_, _, _, _, _, _, h4502, _, _, _, _) => {
                        h4502
                    }
                };
                nat_mul(a4503, a4504)?
            };
            nat_eq(a4505, a4506)
        }
    } else {
        false
    } {
        let a4508 = match v0.clone() {
            Adt2::C0(_, _, _, _, h4507, _, _, _, _, _, _) => {
                h4507
            }
        };
        let a4509 = 1u64;
        if nat_eq(a4508, a4509) {
            true
        } else {
            let a4518 = match v0.clone() {
                Adt2::C0(_, _, _, _, _, _, _, _, h4510, _, _) => {
                    h4510
                }
            };
            let a4519 = {
                let a4516 = match v0.clone() {
                    Adt2::C0(_, _, _, h4511, _, _, _, _, _, _, _) => {
                        h4511
                    }
                };
                let a4517 = {
                    let a4514 = match v0.clone() {
                        Adt2::C0(_, _, h4512, _, _, _, _, _, _, _, _) => {
                            h4512
                        }
                    };
                    let a4515 = match v0.clone() {
                        Adt2::C0(_, _, _, _, _, _, h4513, _, _, _, _) => {
                            h4513
                        }
                    };
                    nat_mul(a4514, a4515)?
                };
                nat_mul(a4516, a4517)?
            };
            nat_eq(a4518, a4519)
        }
    } else {
        false
    } {
        let a4520 = v2;
        let a4521 = 0u64;
        if nat_eq(a4520, a4521) {
            Ok(true)
        } else {
            let a4527 = {
                let a4524 = f125({
                    let a4522 = v2;
                    let a4523 = 1u64;
                    nat_sub(a4522, a4523)
                }, f20(v0.clone(), 1u64))?;
                let a4525 = 4u64;
                nat_add(a4524, a4525)?
            };
            let a4528 = match v0 {
                Adt2::C0(_, _, _, _, _, _, _, _, _, h4526, _) => {
                    h4526
                }
            };
            Ok(nat_le(a4527, a4528))
        }
    } else {
        Ok(false)
    }
}

pub fn f190(v0: Adt2, v1: u64, v2: bool, v3: bool) -> R<bool> {
    if v2 {
        if v3 {
            let a4530 = match v0.clone() {
                Adt2::C0(_, _, _, _, h4529, _, _, _, _, _, _) => {
                    h4529
                }
            };
            let a4531 = 1u64;
            if if if if if nat_le(a4530, a4531) {
                true
            } else {
                let a4533 = match v0.clone() {
                    Adt2::C0(_, _, _, _, _, _, _, _, h4532, _, _) => {
                        h4532
                    }
                };
                let a4534 = v1;
                nat_eq(a4533, a4534)
            } {
                let a4536 = match v0.clone() {
                    Adt2::C0(_, h4535, _, _, _, _, _, _, _, _, _) => {
                        h4535
                    }
                };
                let a4537 = 1u64;
                if nat_le(a4536, a4537) {
                    true
                } else {
                    let a4542 = match v0.clone() {
                        Adt2::C0(_, _, _, _, _, h4538, _, _, _, _, _) => {
                            h4538
                        }
                    };
                    let a4543 = {
                        let a4540 = v1;
                        let a4541 = match v0.clone() {
                            Adt2::C0(_, _, _, _, h4539, _, _, _, _, _, _) => {
                                h4539
                            }
                        };
                        nat_mul(a4540, a4541)?
                    };
                    nat_eq(a4542, a4543)
                }
            } else {
                false
            } {
                let a4545 = match v0.clone() {
                    Adt2::C0(_, _, h4544, _, _, _, _, _, _, _, _) => {
                        h4544
                    }
                };
                let a4546 = 1u64;
                if nat_le(a4545, a4546) {
                    true
                } else {
                    let a4554 = match v0.clone() {
                        Adt2::C0(_, _, _, _, _, _, h4547, _, _, _, _) => {
                            h4547
                        }
                    };
                    let a4555 = {
                        let a4552 = {
                            let a4549 = v1;
                            let a4550 = match v0.clone() {
                                Adt2::C0(_, _, _, _, h4548, _, _, _, _, _, _) => {
                                    h4548
                                }
                            };
                            nat_mul(a4549, a4550)?
                        };
                        let a4553 = match v0.clone() {
                            Adt2::C0(_, h4551, _, _, _, _, _, _, _, _, _) => {
                                h4551
                            }
                        };
                        nat_mul(a4552, a4553)?
                    };
                    nat_eq(a4554, a4555)
                }
            } else {
                false
            } {
                let a4557 = match v0.clone() {
                    Adt2::C0(_, _, _, h4556, _, _, _, _, _, _, _) => {
                        h4556
                    }
                };
                let a4558 = 1u64;
                if nat_le(a4557, a4558) {
                    true
                } else {
                    let a4569 = match v0.clone() {
                        Adt2::C0(_, _, _, _, _, _, _, h4559, _, _, _) => {
                            h4559
                        }
                    };
                    let a4570 = {
                        let a4567 = {
                            let a4564 = {
                                let a4561 = v1;
                                let a4562 = match v0.clone() {
                                    Adt2::C0(_, _, _, _, h4560, _, _, _, _, _, _) => {
                                        h4560
                                    }
                                };
                                nat_mul(a4561, a4562)?
                            };
                            let a4565 = match v0.clone() {
                                Adt2::C0(_, h4563, _, _, _, _, _, _, _, _, _) => {
                                    h4563
                                }
                            };
                            nat_mul(a4564, a4565)?
                        };
                        let a4568 = match v0.clone() {
                            Adt2::C0(_, _, h4566, _, _, _, _, _, _, _, _) => {
                                h4566
                            }
                        };
                        nat_mul(a4567, a4568)?
                    };
                    nat_eq(a4569, a4570)
                }
            } else {
                false
            } {
                let a4584 = {
                    let a4581 = {
                        let a4578 = {
                            let a4575 = {
                                let a4572 = v1;
                                let a4573 = match v0.clone() {
                                    Adt2::C0(_, _, _, _, h4571, _, _, _, _, _, _) => {
                                        h4571
                                    }
                                };
                                nat_mul(a4572, a4573)?
                            };
                            let a4576 = match v0.clone() {
                                Adt2::C0(_, h4574, _, _, _, _, _, _, _, _, _) => {
                                    h4574
                                }
                            };
                            nat_mul(a4575, a4576)?
                        };
                        let a4579 = match v0.clone() {
                            Adt2::C0(_, _, h4577, _, _, _, _, _, _, _, _) => {
                                h4577
                            }
                        };
                        nat_mul(a4578, a4579)?
                    };
                    let a4582 = match v0.clone() {
                        Adt2::C0(_, _, _, h4580, _, _, _, _, _, _, _) => {
                            h4580
                        }
                    };
                    nat_mul(a4581, a4582)?
                };
                let a4585 = match v0 {
                    Adt2::C0(_, _, _, _, _, _, _, _, _, h4583, _) => {
                        h4583
                    }
                };
                Ok(nat_le(a4584, a4585))
            } else {
                Ok(false)
            }
        } else {
            let a4587 = match v0.clone() {
                Adt2::C0(_, _, _, h4586, _, _, _, _, _, _, _) => {
                    h4586
                }
            };
            let a4588 = 1u64;
            if if if if if nat_le(a4587, a4588) {
                true
            } else {
                let a4590 = match v0.clone() {
                    Adt2::C0(_, _, _, _, _, _, _, h4589, _, _, _) => {
                        h4589
                    }
                };
                let a4591 = v1;
                nat_eq(a4590, a4591)
            } {
                let a4593 = match v0.clone() {
                    Adt2::C0(_, h4592, _, _, _, _, _, _, _, _, _) => {
                        h4592
                    }
                };
                let a4594 = 1u64;
                if nat_le(a4593, a4594) {
                    true
                } else {
                    let a4599 = match v0.clone() {
                        Adt2::C0(_, _, _, _, _, h4595, _, _, _, _, _) => {
                            h4595
                        }
                    };
                    let a4600 = {
                        let a4597 = v1;
                        let a4598 = match v0.clone() {
                            Adt2::C0(_, _, _, h4596, _, _, _, _, _, _, _) => {
                                h4596
                            }
                        };
                        nat_mul(a4597, a4598)?
                    };
                    nat_eq(a4599, a4600)
                }
            } else {
                false
            } {
                let a4602 = match v0.clone() {
                    Adt2::C0(_, _, h4601, _, _, _, _, _, _, _, _) => {
                        h4601
                    }
                };
                let a4603 = 1u64;
                if nat_le(a4602, a4603) {
                    true
                } else {
                    let a4611 = match v0.clone() {
                        Adt2::C0(_, _, _, _, _, _, h4604, _, _, _, _) => {
                            h4604
                        }
                    };
                    let a4612 = {
                        let a4609 = {
                            let a4606 = v1;
                            let a4607 = match v0.clone() {
                                Adt2::C0(_, _, _, h4605, _, _, _, _, _, _, _) => {
                                    h4605
                                }
                            };
                            nat_mul(a4606, a4607)?
                        };
                        let a4610 = match v0.clone() {
                            Adt2::C0(_, h4608, _, _, _, _, _, _, _, _, _) => {
                                h4608
                            }
                        };
                        nat_mul(a4609, a4610)?
                    };
                    nat_eq(a4611, a4612)
                }
            } else {
                false
            } {
                let a4614 = match v0.clone() {
                    Adt2::C0(_, _, _, _, h4613, _, _, _, _, _, _) => {
                        h4613
                    }
                };
                let a4615 = 1u64;
                if nat_le(a4614, a4615) {
                    true
                } else {
                    let a4626 = match v0.clone() {
                        Adt2::C0(_, _, _, _, _, _, _, _, h4616, _, _) => {
                            h4616
                        }
                    };
                    let a4627 = {
                        let a4624 = {
                            let a4621 = {
                                let a4618 = v1;
                                let a4619 = match v0.clone() {
                                    Adt2::C0(_, _, _, h4617, _, _, _, _, _, _, _) => {
                                        h4617
                                    }
                                };
                                nat_mul(a4618, a4619)?
                            };
                            let a4622 = match v0.clone() {
                                Adt2::C0(_, h4620, _, _, _, _, _, _, _, _, _) => {
                                    h4620
                                }
                            };
                            nat_mul(a4621, a4622)?
                        };
                        let a4625 = match v0.clone() {
                            Adt2::C0(_, _, h4623, _, _, _, _, _, _, _, _) => {
                                h4623
                            }
                        };
                        nat_mul(a4624, a4625)?
                    };
                    nat_eq(a4626, a4627)
                }
            } else {
                false
            } {
                let a4641 = {
                    let a4638 = {
                        let a4635 = {
                            let a4632 = {
                                let a4629 = v1;
                                let a4630 = match v0.clone() {
                                    Adt2::C0(_, _, _, h4628, _, _, _, _, _, _, _) => {
                                        h4628
                                    }
                                };
                                nat_mul(a4629, a4630)?
                            };
                            let a4633 = match v0.clone() {
                                Adt2::C0(_, h4631, _, _, _, _, _, _, _, _, _) => {
                                    h4631
                                }
                            };
                            nat_mul(a4632, a4633)?
                        };
                        let a4636 = match v0.clone() {
                            Adt2::C0(_, _, h4634, _, _, _, _, _, _, _, _) => {
                                h4634
                            }
                        };
                        nat_mul(a4635, a4636)?
                    };
                    let a4639 = match v0.clone() {
                        Adt2::C0(_, _, _, _, h4637, _, _, _, _, _, _) => {
                            h4637
                        }
                    };
                    nat_mul(a4638, a4639)?
                };
                let a4642 = match v0 {
                    Adt2::C0(_, _, _, _, _, _, _, _, _, h4640, _) => {
                        h4640
                    }
                };
                Ok(nat_le(a4641, a4642))
            } else {
                Ok(false)
            }
        }
    } else {
        let a4644 = match v0.clone() {
            Adt2::C0(_, h4643, _, _, _, _, _, _, _, _, _) => {
                h4643
            }
        };
        let a4645 = 1u64;
        if if if if if nat_le(a4644, a4645) {
            true
        } else {
            let a4647 = match v0.clone() {
                Adt2::C0(_, _, _, _, _, h4646, _, _, _, _, _) => {
                    h4646
                }
            };
            let a4648 = v1;
            nat_eq(a4647, a4648)
        } {
            let a4650 = match v0.clone() {
                Adt2::C0(_, _, h4649, _, _, _, _, _, _, _, _) => {
                    h4649
                }
            };
            let a4651 = 1u64;
            if nat_le(a4650, a4651) {
                true
            } else {
                let a4656 = match v0.clone() {
                    Adt2::C0(_, _, _, _, _, _, h4652, _, _, _, _) => {
                        h4652
                    }
                };
                let a4657 = {
                    let a4654 = v1;
                    let a4655 = match v0.clone() {
                        Adt2::C0(_, h4653, _, _, _, _, _, _, _, _, _) => {
                            h4653
                        }
                    };
                    nat_mul(a4654, a4655)?
                };
                nat_eq(a4656, a4657)
            }
        } else {
            false
        } {
            let a4659 = match v0.clone() {
                Adt2::C0(_, _, _, h4658, _, _, _, _, _, _, _) => {
                    h4658
                }
            };
            let a4660 = 1u64;
            if nat_le(a4659, a4660) {
                true
            } else {
                let a4668 = match v0.clone() {
                    Adt2::C0(_, _, _, _, _, _, _, h4661, _, _, _) => {
                        h4661
                    }
                };
                let a4669 = {
                    let a4666 = {
                        let a4663 = v1;
                        let a4664 = match v0.clone() {
                            Adt2::C0(_, h4662, _, _, _, _, _, _, _, _, _) => {
                                h4662
                            }
                        };
                        nat_mul(a4663, a4664)?
                    };
                    let a4667 = match v0.clone() {
                        Adt2::C0(_, _, h4665, _, _, _, _, _, _, _, _) => {
                            h4665
                        }
                    };
                    nat_mul(a4666, a4667)?
                };
                nat_eq(a4668, a4669)
            }
        } else {
            false
        } {
            let a4671 = match v0.clone() {
                Adt2::C0(_, _, _, _, h4670, _, _, _, _, _, _) => {
                    h4670
                }
            };
            let a4672 = 1u64;
            if nat_le(a4671, a4672) {
                true
            } else {
                let a4683 = match v0.clone() {
                    Adt2::C0(_, _, _, _, _, _, _, _, h4673, _, _) => {
                        h4673
                    }
                };
                let a4684 = {
                    let a4681 = {
                        let a4678 = {
                            let a4675 = v1;
                            let a4676 = match v0.clone() {
                                Adt2::C0(_, h4674, _, _, _, _, _, _, _, _, _) => {
                                    h4674
                                }
                            };
                            nat_mul(a4675, a4676)?
                        };
                        let a4679 = match v0.clone() {
                            Adt2::C0(_, _, h4677, _, _, _, _, _, _, _, _) => {
                                h4677
                            }
                        };
                        nat_mul(a4678, a4679)?
                    };
                    let a4682 = match v0.clone() {
                        Adt2::C0(_, _, _, h4680, _, _, _, _, _, _, _) => {
                            h4680
                        }
                    };
                    nat_mul(a4681, a4682)?
                };
                nat_eq(a4683, a4684)
            }
        } else {
            false
        } {
            let a4698 = {
                let a4695 = {
                    let a4692 = {
                        let a4689 = {
                            let a4686 = v1;
                            let a4687 = match v0.clone() {
                                Adt2::C0(_, h4685, _, _, _, _, _, _, _, _, _) => {
                                    h4685
                                }
                            };
                            nat_mul(a4686, a4687)?
                        };
                        let a4690 = match v0.clone() {
                            Adt2::C0(_, _, h4688, _, _, _, _, _, _, _, _) => {
                                h4688
                            }
                        };
                        nat_mul(a4689, a4690)?
                    };
                    let a4693 = match v0.clone() {
                        Adt2::C0(_, _, _, h4691, _, _, _, _, _, _, _) => {
                            h4691
                        }
                    };
                    nat_mul(a4692, a4693)?
                };
                let a4696 = match v0.clone() {
                    Adt2::C0(_, _, _, _, h4694, _, _, _, _, _, _) => {
                        h4694
                    }
                };
                nat_mul(a4695, a4696)?
            };
            let a4699 = match v0 {
                Adt2::C0(_, _, _, _, _, _, _, _, _, h4697, _) => {
                    h4697
                }
            };
            Ok(nat_le(a4698, a4699))
        } else {
            Ok(false)
        }
    }
}

pub fn f191(v0: Adt2, v1: u64) -> R<bool> {
    let a4701 = match v0.clone() {
        Adt2::C0(h4700, _, _, _, _, _, _, _, _, _, _) => {
            h4700
        }
    };
    let a4702 = 0u64;
    if if nat_eq(a4701, a4702) {
        f22(v0.clone())?
    } else {
        false
    } {
        let a4703 = v1;
        let a4704 = 1u64;
        if nat_le(a4703, a4704) {
            Ok(true)
        } else {
            let a4709 = match v0.clone() {
                Adt2::C0(_, _, _, _, _, _, h4705, _, _, _, _) => {
                    h4705
                }
            };
            let a4710 = {
                let a4707 = 4u64;
                let a4708 = match v0 {
                    Adt2::C0(_, h4706, _, _, _, _, _, _, _, _, _) => {
                        h4706
                    }
                };
                nat_mul(a4707, a4708)?
            };
            Ok(nat_eq(a4709, a4710))
        }
    } else {
        Ok(false)
    }
}

pub fn f192(v0: Adt2, v1: u64) -> R<bool> {
    let a4712 = match v0.clone() {
        Adt2::C0(_, _, _, h4711, _, _, _, _, _, _, _) => {
            h4711
        }
    };
    let a4713 = 1u64;
    if if if nat_eq(a4712, a4713) {
        let a4715 = match v0.clone() {
            Adt2::C0(_, _, _, _, h4714, _, _, _, _, _, _) => {
                h4714
            }
        };
        let a4716 = 1u64;
        nat_eq(a4715, a4716)
    } else {
        false
    } {
        let a4718 = match v0.clone() {
            Adt2::C0(_, _, _, _, _, h4717, _, _, _, _, _) => {
                h4717
            }
        };
        let a4719 = v1;
        nat_eq(a4718, a4719)
    } else {
        false
    } {
        let a4721 = match v0.clone() {
            Adt2::C0(_, _, h4720, _, _, _, _, _, _, _, _) => {
                h4720
            }
        };
        let a4722 = 1u64;
        if nat_eq(a4721, a4722) {
            Ok(true)
        } else {
            let a4727 = match v0.clone() {
                Adt2::C0(_, _, _, _, _, _, h4723, _, _, _, _) => {
                    h4723
                }
            };
            let a4728 = {
                let a4725 = match v0 {
                    Adt2::C0(_, h4724, _, _, _, _, _, _, _, _, _) => {
                        h4724
                    }
                };
                let a4726 = v1;
                nat_mul(a4725, a4726)?
            };
            Ok(nat_eq(a4727, a4728))
        }
    } else {
        Ok(false)
    }
}

pub fn f193(v0: Adt6, v1: u64, v2: u64) -> R<bool> {
    let v3: u64 = {
        let a4732 = {
            let a4729 = v2;
            let a4730 = 1u64;
            nat_sub(a4729, a4730)
        };
        let a4733 = match v0.clone() {
            Adt6::C0(_, h4731, _, _) => {
                h4731
            }
        };
        nat_mul(a4732, a4733)?
    };
    let a4735 = 0u64;
    let a4736 = match v0.clone() {
        Adt6::C0(h4734, _, _, _) => {
            h4734
        }
    };
    if if if if if if if if nat_lt(a4735, a4736) {
        let a4738 = match v0.clone() {
            Adt6::C0(h4737, _, _, _) => {
                h4737
            }
        };
        let a4739 = 2147483648u64;
        nat_lt(a4738, a4739)
    } else {
        false
    } {
        let a4741 = 0u64;
        let a4742 = match v0.clone() {
            Adt6::C0(_, h4740, _, _) => {
                h4740
            }
        };
        nat_lt(a4741, a4742)
    } else {
        false
    } {
        let a4744 = match v0.clone() {
            Adt6::C0(_, h4743, _, _) => {
                h4743
            }
        };
        let a4745 = 2147483648u64;
        nat_lt(a4744, a4745)
    } else {
        false
    } {
        let a4746 = v1;
        let a4747 = 2147483648u64;
        nat_lt(a4746, a4747)
    } else {
        false
    } {
        let a4748 = 0u64;
        let a4749 = v2;
        nat_lt(a4748, a4749)
    } else {
        false
    } {
        let a4750 = v3;
        let a4751 = 2147483648u64;
        nat_lt(a4750, a4751)
    } else {
        false
    } {
        let a4763 = {
            let a4758 = {
                let a4755 = v3;
                let a4756 = {
                    let a4753 = match v0.clone() {
                        Adt6::C0(h4752, _, _, _) => {
                            h4752
                        }
                    };
                    let a4754 = 1u64;
                    nat_sub(a4753, a4754)
                };
                nat_add(a4755, a4756)?
            };
            let a4759 = f118(match v0.clone() {
                Adt6::C0(_, _, h4757, _) => {
                    h4757
                }
            });
            nat_add(a4758, a4759)?
        };
        let a4764 = {
            let a4761 = 2147483647u64;
            let a4762 = f119(match v0.clone() {
                Adt6::C0(_, _, h4760, _) => {
                    h4760
                }
            });
            nat_add(a4761, a4762)?
        };
        nat_le(a4763, a4764)
    } else {
        false
    } {
        let a4766 = match v0 {
            Adt6::C0(_, _, h4765, _) => {
                h4765
            }
        };
        let a4767 = 2147483648u64;
        Ok(!nat_eq(a4766, a4767))
    } else {
        Ok(false)
    }
}

pub fn f194(v0: Adt2, v1: Adt2, v2: Adt2) -> R<Adt10> {
    if f243(match v0.clone() {
        Adt2::C0(h4768, _, _, _, _, _, _, _, _, _, _) => {
            h4768
        }
    }, match v0.clone() {
        Adt2::C0(_, h4769, _, _, _, _, _, _, _, _, _) => {
            h4769
        }
    }, match v0.clone() {
        Adt2::C0(_, _, h4770, _, _, _, _, _, _, _, _) => {
            h4770
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, h4771, _, _, _, _, _, _, _) => {
            h4771
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, h4772, _, _, _, _, _, _) => {
            h4772
        }
    }, match v1.clone() {
        Adt2::C0(h4773, _, _, _, _, _, _, _, _, _, _) => {
            h4773
        }
    }, match v1.clone() {
        Adt2::C0(_, h4774, _, _, _, _, _, _, _, _, _) => {
            h4774
        }
    }, match v1.clone() {
        Adt2::C0(_, _, h4775, _, _, _, _, _, _, _, _) => {
            h4775
        }
    }, match v1.clone() {
        Adt2::C0(_, _, _, h4776, _, _, _, _, _, _, _) => {
            h4776
        }
    }, match v1.clone() {
        Adt2::C0(_, _, _, _, h4777, _, _, _, _, _, _) => {
            h4777
        }
    }, match v2.clone() {
        Adt2::C0(h4778, _, _, _, _, _, _, _, _, _, _) => {
            h4778
        }
    }, match v2.clone() {
        Adt2::C0(_, h4779, _, _, _, _, _, _, _, _, _) => {
            h4779
        }
    }, match v2.clone() {
        Adt2::C0(_, _, h4780, _, _, _, _, _, _, _, _) => {
            h4780
        }
    }, match v2.clone() {
        Adt2::C0(_, _, _, h4781, _, _, _, _, _, _, _) => {
            h4781
        }
    }, match v2.clone() {
        Adt2::C0(_, _, _, _, h4782, _, _, _, _, _, _) => {
            h4782
        }
    }) {
        let m4784 = f187(match v0.clone() {
            Adt2::C0(h4783, _, _, _, _, _, _, _, _, _, _) => {
                h4783
            }
        });
        match m4784 {
            Adt18::C0 => {
                Ok(Adt10::C0)
            }
            Adt18::C1(v3) => {
                let a4790 = {
                    let a4787 = match v0.clone() {
                        Adt2::C0(_, h4785, _, _, _, _, _, _, _, _, _) => {
                            h4785
                        }
                    };
                    let a4788 = match v3.clone() {
                        Adt19::C0(_, h4786, _) => {
                            h4786
                        }
                    };
                    let a4789 = 0u64;
                    nat_rem(a4787, a4788, a4789)
                };
                let a4791 = 0u64;
                if if nat_eq(a4790, a4791) {
                    if f244(match v0.clone() {
                        Adt2::C0(_, h4792, _, _, _, _, _, _, _, _, _) => {
                            h4792
                        }
                    }, match v1.clone() {
                        Adt2::C0(_, h4793, _, _, _, _, _, _, _, _, _) => {
                            h4793
                        }
                    }, match v1.clone() {
                        Adt2::C0(_, _, h4794, _, _, _, _, _, _, _, _) => {
                            h4794
                        }
                    }, match v1.clone() {
                        Adt2::C0(_, _, _, h4795, _, _, _, _, _, _, _) => {
                            h4795
                        }
                    }) {
                        true
                    } else {
                        let a4797 = 4u64;
                        let a4798 = match v2.clone() {
                            Adt2::C0(_, _, _, _, _, h4796, _, _, _, _, _) => {
                                h4796
                            }
                        };
                        if if nat_le(a4797, a4798) {
                            let a4800 = match v3.clone() {
                                Adt19::C0(_, h4799, _) => {
                                    h4799
                                }
                            };
                            let a4801 = 1u64;
                            if nat_eq(a4800, a4801) {
                                true
                            } else {
                                let a4804 = match v3.clone() {
                                    Adt19::C0(_, _, h4802) => {
                                        h4802
                                    }
                                };
                                let a4805 = match v0.clone() {
                                    Adt2::C0(_, _, _, _, _, h4803, _, _, _, _, _) => {
                                        h4803
                                    }
                                };
                                nat_le(a4804, a4805)
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
                        Adt19::C0(_, h4806, _) => {
                            h4806
                        }
                    });
                    Ok(Adt10::C4(if f244(match v0.clone() {
                        Adt2::C0(_, h4807, _, _, _, _, _, _, _, _, _) => {
                            h4807
                        }
                    }, match v1.clone() {
                        Adt2::C0(_, h4808, _, _, _, _, _, _, _, _, _) => {
                            h4808
                        }
                    }, match v1.clone() {
                        Adt2::C0(_, _, h4809, _, _, _, _, _, _, _, _) => {
                            h4809
                        }
                    }, match v1.clone() {
                        Adt2::C0(_, _, _, h4810, _, _, _, _, _, _, _) => {
                            h4810
                        }
                    }) {
                        0u64
                    } else {
                        f151(v4.clone(), f20(v1.clone(), 1u64))?
                    }, 4u64, v4, f20(v2, 1u64), Adt4::C20(match v0 {
                        Adt2::C0(h4811, _, _, _, _, _, _, _, _, _, _) => {
                            h4811
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

pub fn f195(v0: Adt2, v1: Adt2, v2: Adt2) -> R<Adt10> {
    if if if if if if if if if if if if if f196(match v0.clone() {
        Adt2::C0(h4812, _, _, _, _, _, _, _, _, _, _) => {
            h4812
        }
    }) {
        let a4818 = {
            let a4815 = match v0.clone() {
                Adt2::C0(_, h4813, _, _, _, _, _, _, _, _, _) => {
                    h4813
                }
            };
            let a4816 = f226(match v0.clone() {
                Adt2::C0(h4814, _, _, _, _, _, _, _, _, _, _) => {
                    h4814
                }
            });
            let a4817 = 1u64;
            nat_rem(a4815, a4816, a4817)
        };
        let a4819 = 0u64;
        nat_eq(a4818, a4819)
    } else {
        false
    } {
        let a4821 = match v1.clone() {
            Adt2::C0(h4820, _, _, _, _, _, _, _, _, _, _) => {
                h4820
            }
        };
        let a4822 = 26u64;
        nat_eq(a4821, a4822)
    } else {
        false
    } {
        let a4824 = match v2.clone() {
            Adt2::C0(h4823, _, _, _, _, _, _, _, _, _, _) => {
                h4823
            }
        };
        let a4825 = 0u64;
        nat_eq(a4824, a4825)
    } else {
        false
    } {
        let a4828 = match v0.clone() {
            Adt2::C0(_, h4826, _, _, _, _, _, _, _, _, _) => {
                h4826
            }
        };
        let a4829 = match v2.clone() {
            Adt2::C0(_, h4827, _, _, _, _, _, _, _, _, _) => {
                h4827
            }
        };
        nat_eq(a4828, a4829)
    } else {
        false
    } {
        let a4832 = match v1.clone() {
            Adt2::C0(_, h4830, _, _, _, _, _, _, _, _, _) => {
                h4830
            }
        };
        let a4833 = match v0.clone() {
            Adt2::C0(_, _, h4831, _, _, _, _, _, _, _, _) => {
                h4831
            }
        };
        nat_le(a4832, a4833)
    } else {
        false
    } {
        let a4835 = match v0.clone() {
            Adt2::C0(_, _, _, h4834, _, _, _, _, _, _, _) => {
                h4834
            }
        };
        let a4836 = 1u64;
        nat_eq(a4835, a4836)
    } else {
        false
    } {
        let a4838 = match v0.clone() {
            Adt2::C0(_, _, _, _, h4837, _, _, _, _, _, _) => {
                h4837
            }
        };
        let a4839 = 1u64;
        nat_eq(a4838, a4839)
    } else {
        false
    } {
        let a4841 = match v2.clone() {
            Adt2::C0(_, _, _, h4840, _, _, _, _, _, _, _) => {
                h4840
            }
        };
        let a4842 = 1u64;
        nat_eq(a4841, a4842)
    } else {
        false
    } {
        let a4844 = match v2.clone() {
            Adt2::C0(_, _, _, _, h4843, _, _, _, _, _, _) => {
                h4843
            }
        };
        let a4845 = 1u64;
        nat_eq(a4844, a4845)
    } else {
        false
    } {
        let a4847 = match v1.clone() {
            Adt2::C0(_, _, h4846, _, _, _, _, _, _, _, _) => {
                h4846
            }
        };
        let a4848 = 1u64;
        nat_eq(a4847, a4848)
    } else {
        false
    } {
        let a4850 = match v1.clone() {
            Adt2::C0(_, _, _, h4849, _, _, _, _, _, _, _) => {
                h4849
            }
        };
        let a4851 = 1u64;
        nat_eq(a4850, a4851)
    } else {
        false
    } {
        let a4853 = match v1.clone() {
            Adt2::C0(_, _, _, _, h4852, _, _, _, _, _, _) => {
                h4852
            }
        };
        let a4854 = 1u64;
        nat_eq(a4853, a4854)
    } else {
        false
    } {
        if f34(match v2.clone() {
            Adt2::C0(_, h4855, _, _, _, _, _, _, _, _, _) => {
                h4855
            }
        }, match v2.clone() {
            Adt2::C0(_, _, h4856, _, _, _, _, _, _, _, _) => {
                h4856
            }
        }, match v2.clone() {
            Adt2::C0(_, _, _, h4857, _, _, _, _, _, _, _) => {
                h4857
            }
        }, match v2.clone() {
            Adt2::C0(_, _, _, _, h4858, _, _, _, _, _, _) => {
                h4858
            }
        }) {
            Ok(Adt10::C4(0u64, 4u64, f20(v0, 1u64), f20(v2, 1u64), Adt4::C9(0u64, 0u64)))
        } else {
            let a4860 = match v1.clone() {
                Adt2::C0(_, h4859, _, _, _, _, _, _, _, _, _) => {
                    h4859
                }
            };
            let a4861 = 0u64;
            if nat_eq(a4860, a4861) {
                Ok(Adt10::C4(f16(v2.clone(), 1u64)?, 4u64, f20(v0, 1u64), f20(v2, 1u64), Adt4::C9(0u64, 0u64)))
            } else {
                let a4863 = match v0.clone() {
                    Adt2::C0(_, h4862, _, _, _, _, _, _, _, _, _) => {
                        h4862
                    }
                };
                let a4864 = 2147483647u64;
                if if if if if if nat_le(a4863, a4864) {
                    let a4866 = match v1.clone() {
                        Adt2::C0(_, h4865, _, _, _, _, _, _, _, _, _) => {
                            h4865
                        }
                    };
                    let a4867 = 2147483647u64;
                    nat_le(a4866, a4867)
                } else {
                    false
                } {
                    let a4869 = 4u64;
                    let a4870 = match v2.clone() {
                        Adt2::C0(_, _, _, _, _, h4868, _, _, _, _, _) => {
                            h4868
                        }
                    };
                    nat_le(a4869, a4870)
                } else {
                    false
                } {
                    let a4876 = {
                        let a4873 = match v2.clone() {
                            Adt2::C0(_, h4871, _, _, _, _, _, _, _, _, _) => {
                                h4871
                            }
                        };
                        let a4874 = match v2.clone() {
                            Adt2::C0(_, _, _, _, _, h4872, _, _, _, _, _) => {
                                h4872
                            }
                        };
                        nat_mul(a4873, a4874)?
                    };
                    let a4877 = match v2.clone() {
                        Adt2::C0(_, _, _, _, _, _, h4875, _, _, _, _) => {
                            h4875
                        }
                    };
                    nat_le(a4876, a4877)
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
                        let a4890 = {
                            let a4881 = {
                                let a4879 = 2u64;
                                let a4880 = match v1.clone() {
                                    Adt2::C0(_, h4878, _, _, _, _, _, _, _, _, _) => {
                                        h4878
                                    }
                                };
                                nat_mul(a4879, a4880)?
                            };
                            let a4882 = f16(v2.clone(), 1u64)?;
                            nat_add(a4881, a4882)?
                        };
                        let a4891 = {
                            let a4888 = match v1.clone() {
                                Adt2::C0(_, h4883, _, _, _, _, _, _, _, _, _) => {
                                    h4883
                                }
                            };
                            let a4889 = {
                                let a4886 = 1u64;
                                let a4887 = f148(match v0.clone() {
                                    Adt2::C0(h4884, _, _, _, _, _, _, _, _, _, _) => {
                                        h4884
                                    }
                                }, match v0.clone() {
                                    Adt2::C0(_, h4885, _, _, _, _, _, _, _, _, _) => {
                                        h4885
                                    }
                                })?;
                                nat_add(a4886, a4887)?
                            };
                            nat_mul(a4888, a4889)?
                        };
                        nat_add(a4890, a4891)?
                    }, 4u64, Adt3::C0(match v0.clone() {
                        Adt2::C0(_, h4892, _, _, _, _, _, _, _, _, _) => {
                            h4892
                        }
                    }, match v1.clone() {
                        Adt2::C0(_, h4893, _, _, _, _, _, _, _, _, _) => {
                            h4893
                        }
                    }, 1u64, 1u64, match v0.clone() {
                        Adt2::C0(_, _, _, _, _, h4894, _, _, _, _, _) => {
                            h4894
                        }
                    }, match v0.clone() {
                        Adt2::C0(_, _, _, _, _, _, h4895, _, _, _, _) => {
                            h4895
                        }
                    }, match v0.clone() {
                        Adt2::C0(_, _, _, _, _, _, _, h4896, _, _, _) => {
                            h4896
                        }
                    }, match v0.clone() {
                        Adt2::C0(_, _, _, _, _, _, _, _, h4897, _, _) => {
                            h4897
                        }
                    }), f20(v2, 1u64), Adt4::C19(match v0 {
                        Adt2::C0(h4898, _, _, _, _, _, _, _, _, _, _) => {
                            h4898
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

pub fn f196(v0: u64) -> bool {
    let a4899 = v0;
    let a4900 = 0u64;
    if if if if if if if if if if if if if if if if if if if if if if if if if if if if if nat_eq(a4899, a4900) {
        true
    } else {
        let a4901 = v0;
        let a4902 = 1u64;
        nat_eq(a4901, a4902)
    } {
        true
    } else {
        let a4903 = v0;
        let a4904 = 2u64;
        nat_eq(a4903, a4904)
    } {
        true
    } else {
        let a4905 = v0;
        let a4906 = 3u64;
        nat_eq(a4905, a4906)
    } {
        true
    } else {
        let a4907 = v0;
        let a4908 = 6u64;
        nat_eq(a4907, a4908)
    } {
        true
    } else {
        let a4909 = v0;
        let a4910 = 7u64;
        nat_eq(a4909, a4910)
    } {
        true
    } else {
        let a4911 = v0;
        let a4912 = 8u64;
        nat_eq(a4911, a4912)
    } {
        true
    } else {
        let a4913 = v0;
        let a4914 = 10u64;
        nat_eq(a4913, a4914)
    } {
        true
    } else {
        let a4915 = v0;
        let a4916 = 11u64;
        nat_eq(a4915, a4916)
    } {
        true
    } else {
        let a4917 = v0;
        let a4918 = 12u64;
        nat_eq(a4917, a4918)
    } {
        true
    } else {
        let a4919 = v0;
        let a4920 = 13u64;
        nat_eq(a4919, a4920)
    } {
        true
    } else {
        let a4921 = v0;
        let a4922 = 14u64;
        nat_eq(a4921, a4922)
    } {
        true
    } else {
        let a4923 = v0;
        let a4924 = 16u64;
        nat_eq(a4923, a4924)
    } {
        true
    } else {
        let a4925 = v0;
        let a4926 = 17u64;
        nat_eq(a4925, a4926)
    } {
        true
    } else {
        let a4927 = v0;
        let a4928 = 18u64;
        nat_eq(a4927, a4928)
    } {
        true
    } else {
        let a4929 = v0;
        let a4930 = 19u64;
        nat_eq(a4929, a4930)
    } {
        true
    } else {
        let a4931 = v0;
        let a4932 = 20u64;
        nat_eq(a4931, a4932)
    } {
        true
    } else {
        let a4933 = v0;
        let a4934 = 21u64;
        nat_eq(a4933, a4934)
    } {
        true
    } else {
        let a4935 = v0;
        let a4936 = 22u64;
        nat_eq(a4935, a4936)
    } {
        true
    } else {
        let a4937 = v0;
        let a4938 = 23u64;
        nat_eq(a4937, a4938)
    } {
        true
    } else {
        let a4939 = v0;
        let a4940 = 26u64;
        nat_eq(a4939, a4940)
    } {
        true
    } else {
        let a4941 = v0;
        let a4942 = 29u64;
        nat_eq(a4941, a4942)
    } {
        true
    } else {
        let a4943 = v0;
        let a4944 = 30u64;
        nat_eq(a4943, a4944)
    } {
        true
    } else {
        let a4945 = v0;
        let a4946 = 34u64;
        nat_eq(a4945, a4946)
    } {
        true
    } else {
        let a4947 = v0;
        let a4948 = 35u64;
        nat_eq(a4947, a4948)
    } {
        true
    } else {
        let a4949 = v0;
        let a4950 = 39u64;
        nat_eq(a4949, a4950)
    } {
        true
    } else {
        let a4951 = v0;
        let a4952 = 40u64;
        nat_eq(a4951, a4952)
    } {
        true
    } else {
        let a4953 = v0;
        let a4954 = 41u64;
        nat_eq(a4953, a4954)
    } {
        true
    } else {
        let a4955 = v0;
        let a4956 = 42u64;
        nat_eq(a4955, a4956)
    } {
        true
    } else {
        let a4957 = v0;
        let a4958 = 43u64;
        nat_eq(a4957, a4958)
    }
}

pub fn f197(v0: u64) -> bool {
    if f149(v0) {
        true
    } else {
        let a4959 = v0;
        let a4960 = 43u64;
        nat_eq(a4959, a4960)
    }
}

pub fn f198(v0: u64, v1: Adt2, v2: bool, v3: Adt2) -> R<Adt10> {
    let a4963 = match v1.clone() {
        Adt2::C0(h4961, _, _, _, _, _, _, _, _, _, _) => {
            h4961
        }
    };
    let a4964 = match v3.clone() {
        Adt2::C0(h4962, _, _, _, _, _, _, _, _, _, _) => {
            h4962
        }
    };
    if if nat_eq(a4963, a4964) {
        let a4965 = v0;
        let a4966 = 33u64;
        if nat_eq(a4965, a4966) {
            v2
        } else {
            true
        }
    } else {
        false
    } {
        let m4968 = f187(match v1.clone() {
            Adt2::C0(h4967, _, _, _, _, _, _, _, _, _, _) => {
                h4967
            }
        });
        match m4968 {
            Adt18::C0 => {
                Ok(Adt10::C0)
            }
            Adt18::C1(v4) => {
                let v5: u64 = match v4.clone() {
                    Adt19::C0(_, h4969, _) => {
                        h4969
                    }
                };
                let v6: u64 = f16(v1.clone(), v5)?;
                let a4974 = {
                    let a4971 = match v1.clone() {
                        Adt2::C0(_, h4970, _, _, _, _, _, _, _, _, _) => {
                            h4970
                        }
                    };
                    let a4972 = v5;
                    let a4973 = 1u64;
                    nat_rem(a4971, a4972, a4973)
                };
                let a4975 = 0u64;
                if if if nat_eq(a4974, a4975) {
                    let a4980 = {
                        let a4977 = match v3.clone() {
                            Adt2::C0(_, h4976, _, _, _, _, _, _, _, _, _) => {
                                h4976
                            }
                        };
                        let a4978 = v5;
                        let a4979 = 1u64;
                        nat_rem(a4977, a4978, a4979)
                    };
                    let a4981 = 0u64;
                    nat_eq(a4980, a4981)
                } else {
                    false
                } {
                    let a4982 = v6;
                    let a4983 = f16(v3.clone(), v5)?;
                    nat_eq(a4982, a4983)
                } else {
                    false
                } {
                    Ok(Adt10::C4(v6, match v4 {
                        Adt19::C0(_, _, h4984) => {
                            h4984
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

pub fn f199(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64, v7: u64) -> bool {
    if f34(v4, v5, v6, v7) {
        f34(v0, v1, v2, v3)
    } else {
        let a4988 = {
            let a4985 = v0;
            let a4986 = v4;
            let a4987 = 1u64;
            nat_rem(a4985, a4986, a4987)
        };
        let a4989 = 0u64;
        if if if nat_eq(a4988, a4989) {
            let a4993 = {
                let a4990 = v1;
                let a4991 = v5;
                let a4992 = 1u64;
                nat_rem(a4990, a4991, a4992)
            };
            let a4994 = 0u64;
            nat_eq(a4993, a4994)
        } else {
            false
        } {
            let a4998 = {
                let a4995 = v2;
                let a4996 = v6;
                let a4997 = 1u64;
                nat_rem(a4995, a4996, a4997)
            };
            let a4999 = 0u64;
            nat_eq(a4998, a4999)
        } else {
            false
        } {
            let a5003 = {
                let a5000 = v3;
                let a5001 = v7;
                let a5002 = 1u64;
                nat_rem(a5000, a5001, a5002)
            };
            let a5004 = 0u64;
            nat_eq(a5003, a5004)
        } else {
            false
        }
    }
}

pub fn f200(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64) -> R<bool> {
    let a5005 = v0;
    let a5006 = v1;
    if nat_eq(a5005, a5006) {
        let a5009 = v4;
        let a5010 = {
            let a5007 = v2;
            let a5008 = v3;
            nat_add(a5007, a5008)?
        };
        Ok(nat_eq(a5009, a5010))
    } else {
        let a5011 = v2;
        let a5012 = v3;
        if nat_eq(a5011, a5012) {
            let a5013 = v4;
            let a5014 = v2;
            Ok(nat_eq(a5013, a5014))
        } else {
            Ok(false)
        }
    }
}

pub fn f201(v0: u64, v1: u64, v2: u64, v3: u64) -> R<bool> {
    let v4: u64 = {
        let a5017 = {
            let a5015 = v0;
            let a5016 = f119(v1);
            nat_add(a5015, a5016)?
        };
        let a5018 = f119(v2);
        nat_add(a5017, a5018)?
    };
    let v5: u64 = {
        let a5019 = f118(v1);
        let a5020 = f118(v2);
        nat_add(a5019, a5020)?
    };
    let a5021 = v1;
    let a5022 = 4294967295u64;
    if if nat_le(a5021, a5022) {
        let a5023 = v2;
        let a5024 = 4294967295u64;
        nat_le(a5023, a5024)
    } else {
        false
    } {
        let a5025 = v5;
        let a5026 = v4;
        if nat_le(a5025, a5026) {
            let a5029 = v3;
            let a5030 = {
                let a5027 = v4;
                let a5028 = v5;
                nat_sub(a5027, a5028)
            };
            Ok(nat_eq(a5029, a5030))
        } else {
            Ok(false)
        }
    } else {
        Ok(false)
    }
}

pub fn f202(v0: u64, v1: u64) -> R<bool> {
    let a5031 = v1;
    let a5032 = 4294967295u64;
    if nat_le(a5031, a5032) {
        let a5035 = {
            let a5033 = f119(v1);
            let a5034 = f118(v1);
            nat_add(a5033, a5034)?
        };
        let a5036 = v0;
        Ok(nat_lt(a5035, a5036))
    } else {
        Ok(false)
    }
}

pub fn f203(v0: Adt3, v1: u64, v2: u64, v3: u64, v4: bool) -> R<bool> {
    let v5: u64 = f245(v0.clone(), false)?;
    let v6: u64 = f245(v0, true)?;
    let a5037 = v6;
    let a5038 = v1;
    if if nat_le(a5037, a5038) {
        let a5045 = {
            let a5043 = {
                let a5039 = v1;
                let a5040 = v5;
                nat_add(a5039, a5040)?
            };
            let a5044 = {
                let a5041 = v3;
                let a5042 = 0u64;
                if nat_eq(a5041, a5042) {
                    0u64
                } else {
                    4u64
                }
            };
            nat_add(a5043, a5044)?
        };
        let a5046 = v2;
        nat_le(a5045, a5046)
    } else {
        false
    } {
        if v4 {
            let a5051 = {
                let a5049 = {
                    let a5047 = v1;
                    let a5048 = v5;
                    nat_add(a5047, a5048)?
                };
                let a5050 = v6;
                nat_sub(a5049, a5050)
            };
            let a5052 = v2;
            Ok(nat_lt(a5051, a5052))
        } else {
            let a5057 = {
                let a5055 = {
                    let a5053 = v1;
                    let a5054 = v5;
                    nat_add(a5053, a5054)?
                };
                let a5056 = v6;
                nat_sub(a5055, a5056)
            };
            let a5058 = v2;
            Ok(nat_le(a5057, a5058))
        }
    } else {
        Ok(false)
    }
}

pub fn f204(v0: u64, v1: u64) -> bool {
    let a5059 = v0;
    let a5060 = 0u64;
    if if nat_eq(a5059, a5060) {
        true
    } else {
        let a5061 = v0;
        let a5062 = 1u64;
        nat_eq(a5061, a5062)
    } {
        let a5063 = v1;
        let a5064 = 4294967296u64;
        nat_lt(a5063, a5064)
    } else {
        false
    }
}

pub fn f205(v0: u64, v1: u64) -> R<u64> {
    let a5069 = {
        let a5067 = v0;
        let a5068 = {
            let a5065 = v1;
            let a5066 = 1u64;
            nat_sub(a5065, a5066)
        };
        nat_add(a5067, a5068)?
    };
    let a5070 = v1;
    let a5071 = 0u64;
    Ok(nat_quot(a5069, a5070, a5071))
}

pub fn f206(v0: Adt2, v1: Adt2, v2: u64, v3: u64) -> R<bool> {
    if f34(match v0.clone() {
        Adt2::C0(_, h5072, _, _, _, _, _, _, _, _, _) => {
            h5072
        }
    }, match v0.clone() {
        Adt2::C0(_, _, h5073, _, _, _, _, _, _, _, _) => {
            h5073
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, h5074, _, _, _, _, _, _, _) => {
            h5074
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, h5075, _, _, _, _, _, _) => {
            h5075
        }
    }) {
        Ok(true)
    } else {
        let v4: u64 = {
            let a5078 = match v1.clone() {
                Adt2::C0(_, h5076, _, _, _, _, _, _, _, _, _) => {
                    h5076
                }
            };
            let a5079 = match v1.clone() {
                Adt2::C0(_, _, h5077, _, _, _, _, _, _, _, _) => {
                    h5077
                }
            };
            nat_mul(a5078, a5079)?
        };
        let v5: u64 = {
            let a5081 = v4;
            let a5082 = match v1.clone() {
                Adt2::C0(_, _, _, h5080, _, _, _, _, _, _, _) => {
                    h5080
                }
            };
            nat_mul(a5081, a5082)?
        };
        let a5095 = {
            let a5093 = {
                let a5084 = match v0.clone() {
                    Adt2::C0(_, h5083, _, _, _, _, _, _, _, _, _) => {
                        h5083
                    }
                };
                let a5085 = 1u64;
                nat_sub(a5084, a5085)
            };
            let a5094 = {
                let a5091 = f246(match v0.clone() {
                    Adt2::C0(_, _, h5086, _, _, _, _, _, _, _, _) => {
                        h5086
                    }
                }, v2, v5, match v1.clone() {
                    Adt2::C0(_, h5087, _, _, _, _, _, _, _, _, _) => {
                        h5087
                    }
                })?;
                let a5092 = f246(match v0 {
                    Adt2::C0(_, _, _, h5088, _, _, _, _, _, _, _) => {
                        h5088
                    }
                }, v2, {
                    let a5089 = v3;
                    let a5090 = v5;
                    nat_mul(a5089, a5090)?
                }, v4)?;
                nat_add(a5091, a5092)?
            };
            nat_add(a5093, a5094)?
        };
        let a5096 = f16(v1, 1u64)?;
        Ok(nat_lt(a5095, a5096))
    }
}

pub fn f207(v0: u64, v1: u64, v2: u64) -> R<bool> {
    let a5097 = v0;
    let a5098 = 2147483648u64;
    if if if if nat_lt(a5097, a5098) {
        let a5099 = v1;
        let a5100 = 2147483648u64;
        nat_lt(a5099, a5100)
    } else {
        false
    } {
        let a5103 = {
            let a5101 = v0;
            let a5102 = v1;
            nat_mul(a5101, a5102)?
        };
        let a5104 = 2147483648u64;
        nat_lt(a5103, a5104)
    } else {
        false
    } {
        let a5109 = {
            let a5107 = {
                let a5105 = v0;
                let a5106 = 1u64;
                nat_sub(a5105, a5106)
            };
            let a5108 = v2;
            nat_mul(a5107, a5108)?
        };
        let a5110 = 2147483648u64;
        nat_lt(a5109, a5110)
    } else {
        false
    } {
        let a5115 = {
            let a5113 = {
                let a5111 = v1;
                let a5112 = 1u64;
                nat_sub(a5111, a5112)
            };
            let a5114 = v2;
            nat_mul(a5113, a5114)?
        };
        let a5116 = 2147483648u64;
        Ok(nat_lt(a5115, a5116))
    } else {
        Ok(false)
    }
}

pub fn f208(v0: Adt3, v1: u64, v2: u64) -> R<u64> {
    let a5117 = v1;
    let a5118 = 13u64;
    if if nat_eq(a5117, a5118) {
        f34(match v0.clone() {
            Adt3::C0(h5119, _, _, _, _, _, _, _) => {
                h5119
            }
        }, match v0.clone() {
            Adt3::C0(_, h5120, _, _, _, _, _, _) => {
                h5120
            }
        }, match v0.clone() {
            Adt3::C0(_, _, h5121, _, _, _, _, _) => {
                h5121
            }
        }, match v0.clone() {
            Adt3::C0(_, _, _, h5122, _, _, _, _) => {
                h5122
            }
        })
    } else {
        false
    } {
        Ok(2u64)
    } else {
        let v3: u64 = {
            let a5128 = match v0.clone() {
                Adt3::C0(_, h5123, _, _, _, _, _, _) => {
                    h5123
                }
            };
            let a5129 = {
                let a5126 = match v0.clone() {
                    Adt3::C0(_, _, h5124, _, _, _, _, _) => {
                        h5124
                    }
                };
                let a5127 = match v0.clone() {
                    Adt3::C0(_, _, _, h5125, _, _, _, _) => {
                        h5125
                    }
                };
                nat_mul(a5126, a5127)?
            };
            nat_mul(a5128, a5129)?
        };
        let a5130 = v1;
        let a5131 = 13u64;
        if nat_eq(a5130, a5131) {
            let a5140 = 2u64;
            let a5141 = if f247(v0.clone(), v2) {
                let a5133 = v3;
                let a5134 = f220(match v0 {
                    Adt3::C0(h5132, _, _, _, _, _, _, _) => {
                        h5132
                    }
                }, false)?;
                nat_mul(a5133, a5134)?
            } else {
                let a5138 = v3;
                let a5139 = {
                    let a5136 = match v0 {
                        Adt3::C0(h5135, _, _, _, _, _, _, _) => {
                            h5135
                        }
                    };
                    let a5137 = 2u64;
                    nat_mul(a5136, a5137)?
                };
                nat_mul(a5138, a5139)?
            };
            nat_add(a5140, a5141)
        } else {
            let a5145 = v3;
            let a5146 = f220(match v0 {
                Adt3::C0(h5142, _, _, _, _, _, _, _) => {
                    h5142
                }
            }, {
                let a5143 = v1;
                let a5144 = 16u64;
                nat_eq(a5143, a5144)
            })?;
            nat_mul(a5145, a5146)
        }
    }
}

pub fn f209(v0: u64, v1: u64, v2: Adt2, v3: Option<Adt2>, v4: Adt2) -> R<Adt10> {
    let a5147 = v0;
    let a5148 = 30u64;
    if if nat_eq(a5147, a5148) {
        let a5151 = match v2.clone() {
            Adt2::C0(_, _, _, _, _, _, h5149, _, _, _, _) => {
                h5149
            }
        };
        let a5152 = match v2.clone() {
            Adt2::C0(_, _, _, _, _, h5150, _, _, _, _, _) => {
                h5150
            }
        };
        nat_lt(a5151, a5152)
    } else {
        false
    } {
        Ok(Adt10::C1)
    } else {
        let m5153 = v3;
        match m5153 {
            None => {
                f248(v0, v1, match v2.clone() {
                    Adt2::C0(h5154, _, _, _, _, _, _, _, _, _, _) => {
                        h5154
                    }
                }, 0u64, match v4.clone() {
                    Adt2::C0(h5155, _, _, _, _, _, _, _, _, _, _) => {
                        h5155
                    }
                }, false, match v2.clone() {
                    Adt2::C0(_, h5156, _, _, _, _, _, _, _, _, _) => {
                        h5156
                    }
                }, match v2.clone() {
                    Adt2::C0(_, _, h5157, _, _, _, _, _, _, _, _) => {
                        h5157
                    }
                }, match v2.clone() {
                    Adt2::C0(_, _, _, h5158, _, _, _, _, _, _, _) => {
                        h5158
                    }
                }, match v2 {
                    Adt2::C0(_, _, _, _, h5159, _, _, _, _, _, _) => {
                        h5159
                    }
                }, 1u64, 1u64, 1u64, 1u64, match v4.clone() {
                    Adt2::C0(_, h5160, _, _, _, _, _, _, _, _, _) => {
                        h5160
                    }
                }, match v4.clone() {
                    Adt2::C0(_, _, h5161, _, _, _, _, _, _, _, _) => {
                        h5161
                    }
                }, match v4.clone() {
                    Adt2::C0(_, _, _, h5162, _, _, _, _, _, _, _) => {
                        h5162
                    }
                }, match v4 {
                    Adt2::C0(_, _, _, _, h5163, _, _, _, _, _, _) => {
                        h5163
                    }
                })
            }
            Some(v5) => {
                f248(v0, v1, match v2.clone() {
                    Adt2::C0(h5164, _, _, _, _, _, _, _, _, _, _) => {
                        h5164
                    }
                }, match v5.clone() {
                    Adt2::C0(h5165, _, _, _, _, _, _, _, _, _, _) => {
                        h5165
                    }
                }, match v4.clone() {
                    Adt2::C0(h5166, _, _, _, _, _, _, _, _, _, _) => {
                        h5166
                    }
                }, true, match v2.clone() {
                    Adt2::C0(_, h5167, _, _, _, _, _, _, _, _, _) => {
                        h5167
                    }
                }, match v2.clone() {
                    Adt2::C0(_, _, h5168, _, _, _, _, _, _, _, _) => {
                        h5168
                    }
                }, match v2.clone() {
                    Adt2::C0(_, _, _, h5169, _, _, _, _, _, _, _) => {
                        h5169
                    }
                }, match v2 {
                    Adt2::C0(_, _, _, _, h5170, _, _, _, _, _, _) => {
                        h5170
                    }
                }, match v5.clone() {
                    Adt2::C0(_, h5171, _, _, _, _, _, _, _, _, _) => {
                        h5171
                    }
                }, match v5.clone() {
                    Adt2::C0(_, _, h5172, _, _, _, _, _, _, _, _) => {
                        h5172
                    }
                }, match v5.clone() {
                    Adt2::C0(_, _, _, h5173, _, _, _, _, _, _, _) => {
                        h5173
                    }
                }, match v5 {
                    Adt2::C0(_, _, _, _, h5174, _, _, _, _, _, _) => {
                        h5174
                    }
                }, match v4.clone() {
                    Adt2::C0(_, h5175, _, _, _, _, _, _, _, _, _) => {
                        h5175
                    }
                }, match v4.clone() {
                    Adt2::C0(_, _, h5176, _, _, _, _, _, _, _, _) => {
                        h5176
                    }
                }, match v4.clone() {
                    Adt2::C0(_, _, _, h5177, _, _, _, _, _, _, _) => {
                        h5177
                    }
                }, match v4 {
                    Adt2::C0(_, _, _, _, h5178, _, _, _, _, _, _) => {
                        h5178
                    }
                })
            }
        }
    }
}

pub fn f210(v0: u64, v1: u64) -> u64 {
    let a5179 = v0;
    let a5180 = 23u64;
    if nat_eq(a5179, a5180) {
        let a5181 = v1;
        let a5182 = 0u64;
        if nat_eq(a5181, a5182) {
            2u64
        } else {
            3u64
        }
    } else {
        1u64
    }
}

pub fn f211(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let a5187 = {
        let a5185 = {
            let a5183 = v0;
            let a5184 = v1;
            nat_sub(a5183, a5184)
        };
        let a5186 = 1u64;
        nat_sub(a5185, a5186)
    };
    let a5188 = v2;
    nat_add(a5187, a5188)
}

pub fn f212(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64, v7: u64, v8: u64, v9: u64, v10: u64, v11: u64) -> R<u64> {
    f117({
        let a5189 = v0;
        let a5190 = v4;
        let a5191 = 0u64;
        nat_rem(a5189, a5190, a5191)
    }, {
        let a5192 = v1;
        let a5193 = v5;
        let a5194 = 0u64;
        nat_rem(a5192, a5193, a5194)
    }, {
        let a5195 = v2;
        let a5196 = v6;
        let a5197 = 0u64;
        nat_rem(a5195, a5196, a5197)
    }, {
        let a5198 = v3;
        let a5199 = v7;
        let a5200 = 0u64;
        nat_rem(a5198, a5199, a5200)
    }, v8, v9, v10, v11)
}

pub fn f213(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let a5201 = v1;
    let a5202 = 2147483648u64;
    if nat_lt(a5201, a5202) {
        let a5213 = {
            let a5211 = {
                let a5206 = {
                    let a5203 = v0;
                    let a5204 = v2;
                    let a5205 = 0u64;
                    nat_rem(a5203, a5204, a5205)
                };
                let a5207 = v2;
                nat_add(a5206, a5207)?
            };
            let a5212 = {
                let a5208 = v1;
                let a5209 = v2;
                let a5210 = 0u64;
                nat_rem(a5208, a5209, a5210)
            };
            nat_sub(a5211, a5212)
        };
        let a5214 = v2;
        let a5215 = 0u64;
        Ok(nat_rem(a5213, a5214, a5215))
    } else {
        let a5218 = {
            let a5216 = v0;
            let a5217 = f118(v1);
            nat_add(a5216, a5217)?
        };
        let a5219 = v2;
        let a5220 = 0u64;
        Ok(nat_rem(a5218, a5219, a5220))
    }
}

pub fn f214(v0: u64, v1: u64, v2: u64) -> R<bool> {
    let a5223 = f119(v1);
    let a5224 = {
        let a5221 = v0;
        let a5222 = f118(v1);
        nat_add(a5221, a5222)?
    };
    if nat_le(a5223, a5224) {
        let a5225 = f215(v0, v1)?;
        let a5226 = v2;
        Ok(nat_lt(a5225, a5226))
    } else {
        Ok(false)
    }
}

pub fn f215(v0: u64, v1: u64) -> R<u64> {
    let a5229 = {
        let a5227 = v0;
        let a5228 = f118(v1);
        nat_add(a5227, a5228)?
    };
    let a5230 = f119(v1);
    Ok(nat_sub(a5229, a5230))
}

pub fn f216(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let a5231 = v0;
    let a5232 = v1;
    if nat_lt(a5231, a5232) {
        let a5233 = v1;
        let a5234 = v0;
        Ok(nat_sub(a5233, a5234))
    } else {
        let a5237 = v0;
        let a5238 = {
            let a5235 = v1;
            let a5236 = v2;
            nat_add(a5235, a5236)?
        };
        if nat_lt(a5237, a5238) {
            let a5239 = v0;
            let a5240 = v1;
            Ok(nat_sub(a5239, a5240))
        } else {
            let a5247 = {
                let a5243 = 2u64;
                let a5244 = {
                    let a5241 = v2;
                    let a5242 = 1u64;
                    nat_sub(a5241, a5242)
                };
                nat_mul(a5243, a5244)?
            };
            let a5248 = {
                let a5245 = v0;
                let a5246 = v1;
                nat_sub(a5245, a5246)
            };
            Ok(nat_sub(a5247, a5248))
        }
    }
}

pub fn f217(v0: u64, v1: Adt3, v2: u64) -> R<u64> {
    let a5251 = {
        let a5249 = v2;
        let a5250 = f249(v1.clone(), f44(v0, v1.clone(), 0u64), f44(v0, v1.clone(), 1u64), f44(v0, v1.clone(), 2u64), f44(v0, v1.clone(), 3u64), false)?;
        nat_add(a5249, a5250)?
    };
    let a5252 = f249(v1.clone(), f44(v0, v1.clone(), 0u64), f44(v0, v1.clone(), 1u64), f44(v0, v1.clone(), 2u64), f44(v0, v1, 3u64), true)?;
    Ok(nat_sub(a5251, a5252))
}

pub fn f218(v0: u64, v1: u64, v2: u64) -> bool {
    let a5253 = v0;
    let a5254 = 0u64;
    if nat_eq(a5253, a5254) {
        let a5255 = v2;
        let a5256 = v1;
        nat_le(a5255, a5256)
    } else {
        let a5257 = v0;
        let a5258 = 1u64;
        if nat_eq(a5257, a5258) {
            let a5259 = v2;
            let a5260 = v1;
            nat_lt(a5259, a5260)
        } else {
            let a5261 = v0;
            let a5262 = 2u64;
            if nat_eq(a5261, a5262) {
                let a5263 = v1;
                let a5264 = v2;
                nat_le(a5263, a5264)
            } else {
                let a5265 = v0;
                let a5266 = 3u64;
                if nat_eq(a5265, a5266) {
                    let a5267 = v1;
                    let a5268 = v2;
                    nat_lt(a5267, a5268)
                } else {
                    let a5269 = v1;
                    let a5270 = v2;
                    nat_eq(a5269, a5270)
                }
            }
        }
    }
}

pub fn f219(v0: u64, v1: Adt3, v2: u64) -> R<Adt9> {
    let a5271 = v0;
    let a5272 = 0u64;
    if nat_eq(a5271, a5272) {
        Ok(f232(0u64))
    } else {
        let a5275 = {
            let a5273 = v0;
            let a5274 = 1u64;
            nat_add(a5273, a5274)?
        };
        let a5276 = f208(v1.clone(), 13u64, v2)?;
        if nat_eq(a5275, a5276) {
            Ok(Adt9::C4({
                let a5277 = v2;
                let a5278 = 0u64;
                if nat_eq(a5277, a5278) {
                    2u64
                } else {
                    7u64
                }
            }, 1u64, 0u64, 2u64, 0u64, 3u64, 0u64))
        } else {
            let v3: u64 = {
                let a5279 = v0;
                let a5280 = 1u64;
                nat_sub(a5279, a5280)
            };
            if f247(v1.clone(), v2) {
                let v4: u64 = f220(match v1.clone() {
                    Adt3::C0(h5281, _, _, _, _, _, _, _) => {
                        h5281
                    }
                }, false)?;
                let v5: u64 = {
                    let a5282 = v3;
                    let a5283 = v4;
                    let a5284 = 0u64;
                    nat_quot(a5282, a5283, a5284)
                };
                let v6: u64 = {
                    let a5285 = v3;
                    let a5286 = v4;
                    let a5287 = 0u64;
                    nat_rem(a5285, a5286, a5287)
                };
                let a5290 = {
                    let a5288 = v6;
                    let a5289 = 1u64;
                    nat_add(a5288, a5289)?
                };
                let a5291 = v4;
                if nat_eq(a5290, a5291) {
                    Ok(Adt9::C4({
                        let a5292 = v2;
                        let a5293 = 0u64;
                        if nat_eq(a5292, a5293) {
                            0u64
                        } else {
                            4u64
                        }
                    }, 1u64, 0u64, 1u64, 1u64, 1u64, 0u64))
                } else {
                    f221(v6, v5, v1, v2)
                }
            } else {
                f250({
                    let a5294 = v3;
                    let a5295 = 2u64;
                    let a5296 = 0u64;
                    nat_quot(a5294, a5295, a5296)
                }, {
                    let a5297 = v3;
                    let a5298 = 2u64;
                    let a5299 = 0u64;
                    nat_rem(a5297, a5298, a5299)
                }, v1, v2, 0u64)
            }
        }
    }
}

pub fn f220(v0: u64, v1: bool) -> R<u64> {
    let a5302 = {
        let a5300 = v0;
        let a5301 = 2u64;
        nat_mul(a5300, a5301)?
    };
    let a5303 = if v1 {
        4u64
    } else {
        2u64
    };
    nat_add(a5302, a5303)
}

pub fn f221(v0: u64, v1: u64, v2: Adt3, v3: u64) -> R<Adt9> {
    let a5304 = v0;
    let a5305 = 0u64;
    if nat_eq(a5304, a5305) {
        Ok(f232(1u64))
    } else {
        f250({
            let a5314 = {
                let a5307 = v1;
                let a5308 = match v2.clone() {
                    Adt3::C0(h5306, _, _, _, _, _, _, _) => {
                        h5306
                    }
                };
                nat_mul(a5307, a5308)?
            };
            let a5315 = {
                let a5311 = {
                    let a5309 = v0;
                    let a5310 = 1u64;
                    nat_sub(a5309, a5310)
                };
                let a5312 = 2u64;
                let a5313 = 0u64;
                nat_quot(a5311, a5312, a5313)
            };
            nat_add(a5314, a5315)?
        }, {
            let a5318 = {
                let a5316 = v0;
                let a5317 = 1u64;
                nat_sub(a5316, a5317)
            };
            let a5319 = 2u64;
            let a5320 = 0u64;
            nat_rem(a5318, a5319, a5320)
        }, v2, v3, 1u64)
    }
}

pub fn f222(v0: u64, v1: u64) -> R<u64> {
    let a5321 = v0;
    let a5322 = f73(v1)?;
    let a5323 = 0u64;
    Ok(nat_rem(a5321, a5322, a5323))
}

pub fn f223(v0: u64, v1: u64) -> R<u64> {
    let a5324 = 4u64;
    let a5325 = f251(v0, v1)?;
    nat_mul(a5324, a5325)
}

pub fn f224(v0: u64, v1: u64) -> R<u64> {
    let a5326 = 4u64;
    let a5327 = f252(v0, v1)?;
    nat_mul(a5326, a5327)
}

pub fn f225(v0: u64, v1: u64) -> R<bool> {
    let a5333 = {
        let a5330 = {
            let a5328 = f222(v0, v1)?;
            let a5329 = 2u64;
            nat_sub(a5328, a5329)
        };
        let a5331 = 2u64;
        let a5332 = 0u64;
        nat_rem(a5330, a5331, a5332)
    };
    let a5334 = 0u64;
    Ok(nat_eq(a5333, a5334))
}

pub fn f226(v0: u64) -> u64 {
    let m5335 = f187(v0);
    match m5335 {
        Adt18::C0 => {
            0u64
        }
        Adt18::C1(v1) => {
            match v1 {
                Adt19::C0(_, h5336, _) => {
                    h5336
                }
            }
        }
    }
}

pub fn f227(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64, v7: u64, v8: u64, v9: u64, v10: u64, v11: u64, v12: u64) -> R<Option<u64>> {
    let m5337 = f187(v0);
    match m5337 {
        Adt18::C0 => {
            Ok(None::<u64>)
        }
        Adt18::C1(v13) => {
            if if if if if f196(v0) {
                let a5338 = v1;
                let a5339 = v7;
                nat_lt(a5338, a5339)
            } else {
                false
            } {
                f154(v8, v2)
            } else {
                false
            } {
                let a5340 = v3;
                let a5341 = f253(v4, v5, v6)?;
                nat_lt(a5340, a5341)
            } else {
                false
            } {
                let a5346 = {
                    let a5343 = v7;
                    let a5344 = match v13.clone() {
                        Adt19::C0(_, h5342, _) => {
                            h5342
                        }
                    };
                    let a5345 = 0u64;
                    nat_rem(a5343, a5344, a5345)
                };
                let a5347 = 0u64;
                nat_eq(a5346, a5347)
            } else {
                false
            } {
                Ok(Some({
                    let a5376 = {
                        let a5366 = {
                            let a5356 = {
                                let a5352 = {
                                    let a5349 = v1;
                                    let a5350 = match v13 {
                                        Adt19::C0(_, h5348, _) => {
                                            h5348
                                        }
                                    };
                                    let a5351 = 0u64;
                                    nat_quot(a5349, a5350, a5351)
                                };
                                let a5353 = v9;
                                nat_mul(a5352, a5353)?
                            };
                            let a5357 = {
                                let a5354 = v2;
                                let a5355 = v10;
                                nat_mul(a5354, a5355)?
                            };
                            nat_add(a5356, a5357)?
                        };
                        let a5367 = {
                            let a5364 = {
                                let a5361 = {
                                    let a5358 = v3;
                                    let a5359 = v4;
                                    let a5360 = 0u64;
                                    nat_quot(a5358, a5359, a5360)
                                };
                                let a5362 = v5;
                                let a5363 = 0u64;
                                nat_rem(a5361, a5362, a5363)
                            };
                            let a5365 = v11;
                            nat_mul(a5364, a5365)?
                        };
                        nat_add(a5366, a5367)?
                    };
                    let a5377 = {
                        let a5374 = {
                            let a5371 = {
                                let a5368 = v3;
                                let a5369 = v4;
                                let a5370 = 0u64;
                                nat_quot(a5368, a5369, a5370)
                            };
                            let a5372 = v5;
                            let a5373 = 0u64;
                            nat_quot(a5371, a5372, a5373)
                        };
                        let a5375 = v12;
                        nat_mul(a5374, a5375)?
                    };
                    nat_add(a5376, a5377)?
                }))
            } else {
                Ok(None::<u64>)
            }
        }
    }
}

pub fn f228(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64, v7: u64, v8: u64, v9: u64, v10: u64, v11: u64, v12: u64, v13: u64, v14: u64) -> R<Adt17> {
    f254(v0, Adt20::C0(v1, v2, v3, v4, v5, v6, v7, v8), Adt20::C0(v1, v2, v3, v4, v9, v10, v11, v12), v13, v14)
}

pub fn f229(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64, v7: u64, v8: u64, v9: u64, v10: u64, v11: u64, v12: u64, v13: u64, v14: u64, v15: u64, v16: u64, v17: u64, v18: u64, v19: u64, v20: u64) -> R<Adt17> {
    f255(v0, Adt20::C0(v1, v2, v3, v4, v5, v6, v7, v8), Adt20::C0(v1, v2, v3, v4, v9, v10, v11, v12), Adt20::C0(v1, v2, v3, v4, v13, v14, v15, v16), v17, v18, v19, v20)
}

pub fn f230(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64, v7: u64, v8: u64, v9: u64, v10: u64, v11: u64, v12: u64, v13: u64, v14: u64, v15: u64, v16: u64, v17: u64) -> R<Adt17> {
    let a5378 = v0;
    let a5379 = 64u64;
    if nat_eq(a5378, a5379) {
        f256(v1, Adt20::C0(v2, v3, v4, v5, v6, v7, v8, v9), Adt20::C0(v2, v3, v4, v5, v10, v11, v12, v13), v14, v15, v16, v17)
    } else {
        let a5380 = v0;
        let a5381 = 65u64;
        if nat_eq(a5380, a5381) {
            f257(v1, Adt20::C0(v2, v3, v4, v5, v6, v7, v8, v9), Adt20::C0(v2, v3, v4, v5, v10, v11, v12, v13), v14, v15, v16, v17)
        } else {
            Ok(Adt17::C0)
        }
    }
}

pub fn f231(v0: u64) -> u64 {
    let a5382 = v0;
    let a5383 = 30u64;
    if nat_eq(a5382, a5383) {
        4u64
    } else {
        3u64
    }
}

pub fn f232(v0: u64) -> Adt9 {
    Adt9::C4(0u64, 2u64, 0u64, 2u64, 0u64, 1u64, v0)
}

pub fn f233(v0: u64) -> bool {
    let a5384 = 0u64;
    let a5385 = v0;
    if nat_lt(a5384, a5385) {
        let a5386 = v0;
        let a5387 = 2139095040u64;
        nat_le(a5386, a5387)
    } else {
        false
    }
}

pub fn f234(v0: u64) -> u64 {
    let a5388 = v0;
    let a5389 = 40u64;
    if nat_lt(a5388, a5389) {
        let a5390 = v0;
        let a5391 = 20u64;
        if nat_lt(a5390, a5391) {
            let a5392 = v0;
            let a5393 = 10u64;
            if nat_lt(a5392, a5393) {
                let a5394 = v0;
                let a5395 = 5u64;
                if nat_lt(a5394, a5395) {
                    let a5396 = v0;
                    let a5397 = 2u64;
                    if nat_lt(a5396, a5397) {
                        let a5398 = v0;
                        let a5399 = 1u64;
                        if nat_lt(a5398, a5399) {
                            1u64
                        } else {
                            2u64
                        }
                    } else {
                        let a5400 = v0;
                        let a5401 = 3u64;
                        if nat_lt(a5400, a5401) {
                            4u64
                        } else {
                            let a5402 = v0;
                            let a5403 = 4u64;
                            if nat_lt(a5402, a5403) {
                                8u64
                            } else {
                                16u64
                            }
                        }
                    }
                } else {
                    let a5404 = v0;
                    let a5405 = 7u64;
                    if nat_lt(a5404, a5405) {
                        let a5406 = v0;
                        let a5407 = 6u64;
                        if nat_lt(a5406, a5407) {
                            32u64
                        } else {
                            64u64
                        }
                    } else {
                        let a5408 = v0;
                        let a5409 = 8u64;
                        if nat_lt(a5408, a5409) {
                            128u64
                        } else {
                            let a5410 = v0;
                            let a5411 = 9u64;
                            if nat_lt(a5410, a5411) {
                                256u64
                            } else {
                                512u64
                            }
                        }
                    }
                }
            } else {
                let a5412 = v0;
                let a5413 = 15u64;
                if nat_lt(a5412, a5413) {
                    let a5414 = v0;
                    let a5415 = 12u64;
                    if nat_lt(a5414, a5415) {
                        let a5416 = v0;
                        let a5417 = 11u64;
                        if nat_lt(a5416, a5417) {
                            1024u64
                        } else {
                            2048u64
                        }
                    } else {
                        let a5418 = v0;
                        let a5419 = 13u64;
                        if nat_lt(a5418, a5419) {
                            4096u64
                        } else {
                            let a5420 = v0;
                            let a5421 = 14u64;
                            if nat_lt(a5420, a5421) {
                                8192u64
                            } else {
                                16384u64
                            }
                        }
                    }
                } else {
                    let a5422 = v0;
                    let a5423 = 17u64;
                    if nat_lt(a5422, a5423) {
                        let a5424 = v0;
                        let a5425 = 16u64;
                        if nat_lt(a5424, a5425) {
                            32768u64
                        } else {
                            65536u64
                        }
                    } else {
                        let a5426 = v0;
                        let a5427 = 18u64;
                        if nat_lt(a5426, a5427) {
                            131072u64
                        } else {
                            let a5428 = v0;
                            let a5429 = 19u64;
                            if nat_lt(a5428, a5429) {
                                262144u64
                            } else {
                                524288u64
                            }
                        }
                    }
                }
            }
        } else {
            let a5430 = v0;
            let a5431 = 30u64;
            if nat_lt(a5430, a5431) {
                let a5432 = v0;
                let a5433 = 25u64;
                if nat_lt(a5432, a5433) {
                    let a5434 = v0;
                    let a5435 = 22u64;
                    if nat_lt(a5434, a5435) {
                        let a5436 = v0;
                        let a5437 = 21u64;
                        if nat_lt(a5436, a5437) {
                            1048576u64
                        } else {
                            2097152u64
                        }
                    } else {
                        let a5438 = v0;
                        let a5439 = 23u64;
                        if nat_lt(a5438, a5439) {
                            4194304u64
                        } else {
                            let a5440 = v0;
                            let a5441 = 24u64;
                            if nat_lt(a5440, a5441) {
                                8388608u64
                            } else {
                                16777216u64
                            }
                        }
                    }
                } else {
                    let a5442 = v0;
                    let a5443 = 27u64;
                    if nat_lt(a5442, a5443) {
                        let a5444 = v0;
                        let a5445 = 26u64;
                        if nat_lt(a5444, a5445) {
                            33554432u64
                        } else {
                            67108864u64
                        }
                    } else {
                        let a5446 = v0;
                        let a5447 = 28u64;
                        if nat_lt(a5446, a5447) {
                            134217728u64
                        } else {
                            let a5448 = v0;
                            let a5449 = 29u64;
                            if nat_lt(a5448, a5449) {
                                268435456u64
                            } else {
                                536870912u64
                            }
                        }
                    }
                }
            } else {
                let a5450 = v0;
                let a5451 = 35u64;
                if nat_lt(a5450, a5451) {
                    let a5452 = v0;
                    let a5453 = 32u64;
                    if nat_lt(a5452, a5453) {
                        let a5454 = v0;
                        let a5455 = 31u64;
                        if nat_lt(a5454, a5455) {
                            1073741824u64
                        } else {
                            2147483648u64
                        }
                    } else {
                        let a5456 = v0;
                        let a5457 = 33u64;
                        if nat_lt(a5456, a5457) {
                            4294967296u64
                        } else {
                            let a5458 = v0;
                            let a5459 = 34u64;
                            if nat_lt(a5458, a5459) {
                                8589934592u64
                            } else {
                                17179869184u64
                            }
                        }
                    }
                } else {
                    let a5460 = v0;
                    let a5461 = 37u64;
                    if nat_lt(a5460, a5461) {
                        let a5462 = v0;
                        let a5463 = 36u64;
                        if nat_lt(a5462, a5463) {
                            34359738368u64
                        } else {
                            68719476736u64
                        }
                    } else {
                        let a5464 = v0;
                        let a5465 = 38u64;
                        if nat_lt(a5464, a5465) {
                            137438953472u64
                        } else {
                            let a5466 = v0;
                            let a5467 = 39u64;
                            if nat_lt(a5466, a5467) {
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
    let a5468 = v0;
    let a5469 = 1u64;
    if nat_le(a5468, a5469) {
        true
    } else {
        let a5473 = {
            let a5470 = v1;
            let a5471 = 4u64;
            let a5472 = 0u64;
            nat_rem(a5470, a5471, a5472)
        };
        let a5474 = 0u64;
        nat_eq(a5473, a5474)
    }
}

pub fn f236(v0: u64, v1: u64) -> bool {
    let a5475 = v0;
    let a5476 = 1u64;
    if nat_le(a5475, a5476) {
        true
    } else {
        let a5477 = 4u64;
        let a5478 = v1;
        nat_le(a5477, a5478)
    }
}

pub fn f237(v0: u64, v1: u64) -> R<u64> {
    let a5479 = v0;
    let a5480 = 0u64;
    if nat_eq(a5479, a5480) {
        Ok(0u64)
    } else {
        let a5485 = 4u64;
        let a5486 = {
            let a5483 = {
                let a5481 = v0;
                let a5482 = 1u64;
                nat_sub(a5481, a5482)
            };
            let a5484 = v1;
            nat_mul(a5483, a5484)?
        };
        nat_add(a5485, a5486)
    }
}

pub fn f238(v0: bool, v1: u64) -> R<u64> {
    let a5491 = {
        let a5489 = {
            let a5487 = f258(v0, v1)?;
            let a5488 = f259(v0, v1)?;
            nat_add(a5487, a5488)?
        };
        let a5490 = f260(v0);
        nat_add(a5489, a5490)?
    };
    let a5492 = v1;
    nat_add(a5491, a5492)
}

pub fn f239(v0: u64, v1: u64) -> R<u64> {
    let v2: u64 = f261(v0, v1)?;
    let v3: u64 = {
        let a5497 = {
            let a5495 = {
                let a5493 = v1;
                let a5494 = 1u64;
                nat_sub(a5493, a5494)
            };
            let a5496 = v2;
            nat_mul(a5495, a5496)?
        };
        let a5498 = v0;
        nat_sub(a5497, a5498)
    };
    let a5499 = v2;
    let a5500 = v3;
    if nat_lt(a5499, a5500) {
        Ok(v3)
    } else {
        Ok(v2)
    }
}

pub fn f240(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64) -> R<u64> {
    let a5501 = v4;
    let a5502 = 0u64;
    if nat_eq(a5501, a5502) {
        Ok(0u64)
    } else {
        let a5509 = 2u64;
        let a5510 = {
            let a5507 = {
                let a5503 = v0;
                let a5504 = v1;
                nat_mul(a5503, a5504)?
            };
            let a5508 = {
                let a5505 = v2;
                let a5506 = v3;
                nat_mul(a5505, a5506)?
            };
            nat_mul(a5507, a5508)?
        };
        nat_mul(a5509, a5510)
    }
}

pub fn f241(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64) -> R<u64> {
    let a5515 = v3;
    let a5516 = {
        let a5513 = {
            let a5511 = f262(v2, v4)?;
            let a5512 = f263(v0, v1, f261(v2, v4)?)?;
            nat_mul(a5511, a5512)?
        };
        let a5514 = f263(v0, v1, f264(v2, v4)?)?;
        nat_add(a5513, a5514)?
    };
    nat_mul(a5515, a5516)
}

pub fn f242(v0: u64) -> R<u64> {
    let a5519 = 13u64;
    let a5520 = {
        let a5517 = 9u64;
        let a5518 = v0;
        nat_mul(a5517, a5518)?
    };
    nat_add(a5519, a5520)
}

pub fn f243(v0: u64, v1: u64, _: u64, v3: u64, v4: u64, v5: u64, v6: u64, v7: u64, v8: u64, v9: u64, v10: u64, v11: u64, v12: u64, v13: u64, v14: u64) -> bool {
    if if if if if if if if if f265(v0, v1, v6, v7, v8) {
        let a5521 = v5;
        let a5522 = 26u64;
        nat_eq(a5521, a5522)
    } else {
        false
    } {
        let a5523 = v3;
        let a5524 = v7;
        nat_eq(a5523, a5524)
    } else {
        false
    } {
        let a5525 = v4;
        let a5526 = v8;
        nat_eq(a5525, a5526)
    } else {
        false
    } {
        let a5527 = v9;
        let a5528 = 1u64;
        nat_eq(a5527, a5528)
    } else {
        false
    } {
        let a5529 = v10;
        let a5530 = f266(v0);
        nat_eq(a5529, a5530)
    } else {
        false
    } {
        let a5531 = v11;
        let a5532 = v1;
        nat_eq(a5531, a5532)
    } else {
        false
    } {
        let a5533 = v12;
        let a5534 = v6;
        nat_eq(a5533, a5534)
    } else {
        false
    } {
        let a5535 = v13;
        let a5536 = v7;
        nat_eq(a5535, a5536)
    } else {
        false
    } {
        let a5537 = v14;
        let a5538 = v8;
        nat_eq(a5537, a5538)
    } else {
        false
    }
}

pub fn f244(v0: u64, v1: u64, v2: u64, v3: u64) -> bool {
    let a5539 = v0;
    let a5540 = 0u64;
    if if if nat_eq(a5539, a5540) {
        true
    } else {
        let a5541 = v1;
        let a5542 = 0u64;
        nat_eq(a5541, a5542)
    } {
        true
    } else {
        let a5543 = v2;
        let a5544 = 0u64;
        nat_eq(a5543, a5544)
    } {
        true
    } else {
        let a5545 = v3;
        let a5546 = 0u64;
        nat_eq(a5545, a5546)
    }
}

pub fn f245(v0: Adt3, v1: bool) -> R<u64> {
    f249(v0.clone(), {
        let a5548 = match v0.clone() {
            Adt3::C0(h5547, _, _, _, _, _, _, _) => {
                h5547
            }
        };
        let a5549 = 1u64;
        nat_sub(a5548, a5549)
    }, {
        let a5551 = match v0.clone() {
            Adt3::C0(_, h5550, _, _, _, _, _, _) => {
                h5550
            }
        };
        let a5552 = 1u64;
        nat_sub(a5551, a5552)
    }, {
        let a5554 = match v0.clone() {
            Adt3::C0(_, _, h5553, _, _, _, _, _) => {
                h5553
            }
        };
        let a5555 = 1u64;
        nat_sub(a5554, a5555)
    }, {
        let a5557 = match v0 {
            Adt3::C0(_, _, _, h5556, _, _, _, _) => {
                h5556
            }
        };
        let a5558 = 1u64;
        nat_sub(a5557, a5558)
    }, v1)
}

pub fn f246(v0: u64, v1: u64, v2: u64, v3: u64) -> R<u64> {
    let a5559 = v0;
    let a5560 = 0u64;
    if nat_eq(a5559, a5560) {
        Ok(0u64)
    } else {
        let v4: u64 = {
            let a5561 = v0;
            let a5562 = 1u64;
            nat_sub(a5561, a5562)
        };
        let v5: u64 = {
            let a5563 = v4;
            let a5564 = v1;
            let a5565 = 0u64;
            nat_quot(a5563, a5564, a5565)
        };
        let v6: u64 = {
            let a5573 = {
                let a5566 = v5;
                let a5567 = v2;
                nat_mul(a5566, a5567)?
            };
            let a5574 = {
                let a5571 = {
                    let a5568 = v4;
                    let a5569 = v1;
                    let a5570 = 0u64;
                    nat_rem(a5568, a5569, a5570)
                };
                let a5572 = v3;
                nat_mul(a5571, a5572)?
            };
            nat_add(a5573, a5574)?
        };
        let a5575 = v5;
        let a5576 = 0u64;
        if nat_eq(a5575, a5576) {
            Ok(v6)
        } else {
            let v7: u64 = {
                let a5585 = {
                    let a5579 = {
                        let a5577 = v5;
                        let a5578 = 1u64;
                        nat_sub(a5577, a5578)
                    };
                    let a5580 = v2;
                    nat_mul(a5579, a5580)?
                };
                let a5586 = {
                    let a5583 = {
                        let a5581 = v1;
                        let a5582 = 1u64;
                        nat_sub(a5581, a5582)
                    };
                    let a5584 = v3;
                    nat_mul(a5583, a5584)?
                };
                nat_add(a5585, a5586)?
            };
            let a5587 = v6;
            let a5588 = v7;
            if nat_lt(a5587, a5588) {
                Ok(v7)
            } else {
                Ok(v6)
            }
        }
    }
}

pub fn f247(v0: Adt3, v1: u64) -> bool {
    let a5589 = v1;
    let a5590 = 0u64;
    if !nat_eq(a5589, a5590) {
        true
    } else {
        let a5592 = match v0 {
            Adt3::C0(_, _, _, _, h5591, _, _, _) => {
                h5591
            }
        };
        let a5593 = 4u64;
        nat_eq(a5592, a5593)
    }
}

pub fn f248(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: bool, v6: u64, v7: u64, v8: u64, v9: u64, v10: u64, v11: u64, v12: u64, v13: u64, v14: u64, v15: u64, v16: u64, v17: u64) -> R<Adt10> {
    let m5594 = f267(v0, v1);
    match m5594 {
        None => {
            if v5 {
                let m5595 = f268(v0, v2, v3, v4, v6, v7, v8, v9, v10, v11, v12, v13, v14, v15, v16, v17)?;
                match m5595 {
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
                let a5596 = 4u64;
                let a5597 = v28;
                nat_le(a5596, a5597)
            };
            let a5598 = v2;
            let a5599 = 0u64;
            if if if if if nat_eq(a5598, a5599) {
                let a5600 = v4;
                let a5601 = 0u64;
                nat_eq(a5600, a5601)
            } else {
                false
            } {
                let a5602 = v6;
                let a5603 = v14;
                if nat_eq(a5602, a5603) {
                    let a5604 = v7;
                    let a5605 = v15;
                    nat_eq(a5604, a5605)
                } else {
                    false
                }
            } else {
                false
            } {
                let a5606 = v8;
                let a5607 = v16;
                if nat_eq(a5606, a5607) {
                    let a5608 = v9;
                    let a5609 = v17;
                    nat_eq(a5608, a5609)
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
                        let a5610 = v3;
                        let a5611 = 0u64;
                        nat_eq(a5610, a5611)
                    } else {
                        false
                    } {
                        f199(v6, v7, v8, v9, v10, v11, v12, v13)
                    } else {
                        false
                    }
                }
            } else {
                false
            } {
                Ok(Adt10::C2(v28, v29, v6, v7, v8, v9, {
                    let a5616 = {
                        let a5612 = v6;
                        let a5613 = v7;
                        nat_mul(a5612, a5613)?
                    };
                    let a5617 = {
                        let a5614 = v8;
                        let a5615 = v9;
                        nat_mul(a5614, a5615)?
                    };
                    nat_mul(a5616, a5617)?
                }))
            } else {
                Ok(Adt10::C1)
            }
        }
    }
}

pub fn f249(v0: Adt3, v1: u64, v2: u64, v3: u64, v4: u64, v5: bool) -> R<u64> {
    let a5638 = {
        let a5632 = {
            let a5626 = {
                let a5620 = v1;
                let a5621 = if v5 {
                    f118(match v0.clone() {
                        Adt3::C0(_, _, _, _, h5618, _, _, _) => {
                            h5618
                        }
                    })
                } else {
                    f119(match v0.clone() {
                        Adt3::C0(_, _, _, _, h5619, _, _, _) => {
                            h5619
                        }
                    })
                };
                nat_mul(a5620, a5621)?
            };
            let a5627 = {
                let a5624 = v2;
                let a5625 = if v5 {
                    f118(match v0.clone() {
                        Adt3::C0(_, _, _, _, _, h5622, _, _) => {
                            h5622
                        }
                    })
                } else {
                    f119(match v0.clone() {
                        Adt3::C0(_, _, _, _, _, h5623, _, _) => {
                            h5623
                        }
                    })
                };
                nat_mul(a5624, a5625)?
            };
            nat_add(a5626, a5627)?
        };
        let a5633 = {
            let a5630 = v3;
            let a5631 = if v5 {
                f118(match v0.clone() {
                    Adt3::C0(_, _, _, _, _, _, h5628, _) => {
                        h5628
                    }
                })
            } else {
                f119(match v0.clone() {
                    Adt3::C0(_, _, _, _, _, _, h5629, _) => {
                        h5629
                    }
                })
            };
            nat_mul(a5630, a5631)?
        };
        nat_add(a5632, a5633)?
    };
    let a5639 = {
        let a5636 = v4;
        let a5637 = if v5 {
            f118(match v0 {
                Adt3::C0(_, _, _, _, _, _, _, h5634) => {
                    h5634
                }
            })
        } else {
            f119(match v0 {
                Adt3::C0(_, _, _, _, _, _, _, h5635) => {
                    h5635
                }
            })
        };
        nat_mul(a5636, a5637)?
    };
    nat_add(a5638, a5639)
}

pub fn f250(v0: u64, v1: u64, v2: Adt3, v3: u64, v4: u64) -> R<Adt9> {
    let a5640 = v1;
    let a5641 = 0u64;
    if nat_eq(a5640, a5641) {
        Ok(Adt9::C4({
            let a5642 = v3;
            let a5643 = 0u64;
            if nat_eq(a5642, a5643) {
                1u64
            } else {
                6u64
            }
        }, 0u64, f125(v0, v2)?, 2u64, 0u64, 1u64, 2u64))
    } else {
        Ok(Adt9::C4({
            let a5644 = v3;
            let a5645 = 0u64;
            if nat_eq(a5644, a5645) {
                0u64
            } else {
                4u64
            }
        }, 1u64, v4, 1u64, 2u64, 1u64, v4))
    }
}

pub fn f251(v0: u64, v1: u64) -> R<u64> {
    let a5646 = v0;
    let a5647 = f73(v1)?;
    let a5648 = 0u64;
    Ok(nat_quot(a5646, a5647, a5648))
}

pub fn f252(v0: u64, v1: u64) -> R<u64> {
    let v2: u64 = f251(v0, v1)?;
    let v3: u64 = {
        let a5651 = {
            let a5649 = f222(v0, v1)?;
            let a5650 = 2u64;
            nat_sub(a5649, a5650)
        };
        let a5652 = 2u64;
        let a5653 = 0u64;
        nat_quot(a5651, a5652, a5653)
    };
    let v4: u64 = {
        let a5654 = v2;
        let a5655 = v1;
        nat_mul(a5654, a5655)?
    };
    if f225(v0, v1)? {
        let a5656 = v4;
        let a5657 = v3;
        nat_add(a5656, a5657)
    } else {
        let a5669 = {
            let a5665 = v4;
            let a5666 = {
                let a5663 = {
                    let a5658 = v1;
                    let a5659 = 1u64;
                    nat_sub(a5658, a5659)
                };
                let a5664 = {
                    let a5660 = v2;
                    let a5661 = v1;
                    let a5662 = 0u64;
                    nat_rem(a5660, a5661, a5662)
                };
                nat_mul(a5663, a5664)?
            };
            nat_sub(a5665, a5666)
        };
        let a5670 = {
            let a5667 = v3;
            let a5668 = v1;
            nat_mul(a5667, a5668)?
        };
        nat_add(a5669, a5670)
    }
}

pub fn f253(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let a5671 = v0;
    let a5672 = 0u64;
    if if if nat_eq(a5671, a5672) {
        true
    } else {
        let a5673 = v1;
        let a5674 = 0u64;
        nat_eq(a5673, a5674)
    } {
        true
    } else {
        let a5675 = v2;
        let a5676 = 0u64;
        nat_eq(a5675, a5676)
    } {
        Ok(0u64)
    } else {
        let a5679 = {
            let a5677 = v0;
            let a5678 = v1;
            nat_mul(a5677, a5678)?
        };
        let a5680 = v2;
        nat_mul(a5679, a5680)
    }
}

pub fn f254(v0: u64, v1: Adt20, v2: Adt20, v3: u64, v4: u64) -> R<Adt17> {
    let v5: u64 = f82(match v1.clone() {
        Adt20::C0(h5681, _, _, _, _, _, _, _) => {
            h5681
        }
    })?;
    let v6: u64 = {
        let a5687 = match v1.clone() {
            Adt20::C0(_, h5682, _, _, _, _, _, _) => {
                h5682
            }
        };
        let a5688 = {
            let a5685 = match v1.clone() {
                Adt20::C0(_, _, h5683, _, _, _, _, _) => {
                    h5683
                }
            };
            let a5686 = match v1.clone() {
                Adt20::C0(_, _, _, h5684, _, _, _, _) => {
                    h5684
                }
            };
            nat_mul(a5685, a5686)?
        };
        nat_mul(a5687, a5688)?
    };
    let a5690 = v0;
    let a5691 = f39(match v1.clone() {
        Adt20::C0(h5689, _, _, _, _, _, _, _) => {
            h5689
        }
    }, v6)?;
    if nat_lt(a5690, a5691) {
        f269({
            let a5692 = v0;
            let a5693 = v5;
            let a5694 = 0u64;
            nat_rem(a5692, a5693, a5694)
        }, {
            let a5695 = v0;
            let a5696 = v5;
            let a5697 = 0u64;
            nat_quot(a5695, a5696, a5697)
        }, v1, v2, v3, v4)
    } else {
        Ok(Adt17::C0)
    }
}

pub fn f255(v0: u64, v1: Adt20, v2: Adt20, v3: Adt20, v4: u64, v5: u64, v6: u64, v7: u64) -> R<Adt17> {
    let v8: u64 = {
        let a5698 = v6;
        let a5699 = 6u64;
        nat_sub(a5698, a5699)
    };
    let v9: Adt20 = f270(v3);
    let a5701 = v0;
    let a5702 = f184(match v1.clone() {
        Adt20::C0(h5700, _, _, _, _, _, _, _) => {
            h5700
        }
    }, v5, v8)?;
    if nat_lt(a5701, a5702) {
        let v10: u64 = f79(match v1.clone() {
            Adt20::C0(h5703, _, _, _, _, _, _, _) => {
                h5703
            }
        }, v5, v8)?;
        let a5704 = v0;
        let a5705 = v10;
        if nat_lt(a5704, a5705) {
            f271(v0, v9, 4u64, 6u64, v7)
        } else {
            let v11: u64 = {
                let a5706 = v0;
                let a5707 = v10;
                nat_sub(a5706, a5707)
            };
            let v12: u64 = f242(match v1.clone() {
                Adt20::C0(h5708, _, _, _, _, _, _, _) => {
                    h5708
                }
            })?;
            f272({
                let a5709 = v11;
                let a5710 = v12;
                let a5711 = 0u64;
                nat_rem(a5709, a5710, a5711)
            }, {
                let a5712 = v11;
                let a5713 = v12;
                let a5714 = 0u64;
                nat_quot(a5712, a5713, a5714)
            }, v1, v2, v9, v4)
        }
    } else {
        Ok(Adt17::C0)
    }
}

pub fn f256(v0: u64, v1: Adt20, v2: Adt20, v3: u64, v4: u64, v5: u64, v6: u64) -> R<Adt17> {
    let v7: u64 = {
        let a5715 = v5;
        let a5716 = 6u64;
        nat_sub(a5715, a5716)
    };
    let v8: bool = f178(match v1.clone() {
        Adt20::C0(_, _, _, _, h5717, _, _, _) => {
            h5717
        }
    }, match v2.clone() {
        Adt20::C0(_, _, _, _, h5718, _, _, _) => {
            h5718
        }
    });
    let a5720 = v0;
    let a5721 = f177(v8, match v1.clone() {
        Adt20::C0(h5719, _, _, _, _, _, _, _) => {
            h5719
        }
    }, v4, v7)?;
    if nat_lt(a5720, a5721) {
        let v9: u64 = f79(match v1.clone() {
            Adt20::C0(h5722, _, _, _, _, _, _, _) => {
                h5722
            }
        }, v4, v7)?;
        let a5723 = v0;
        let a5724 = v9;
        if nat_lt(a5723, a5724) {
            f271(v0, v2, 4u64, 6u64, v6)
        } else {
            let v10: u64 = {
                let a5725 = v0;
                let a5726 = v9;
                nat_sub(a5725, a5726)
            };
            let v11: u64 = f238(v8, match v1.clone() {
                Adt20::C0(h5727, _, _, _, _, _, _, _) => {
                    h5727
                }
            })?;
            f273({
                let a5728 = v10;
                let a5729 = v11;
                let a5730 = 0u64;
                nat_rem(a5728, a5729, a5730)
            }, {
                let a5731 = v10;
                let a5732 = v11;
                let a5733 = 0u64;
                nat_quot(a5731, a5732, a5733)
            }, v1, v2, v3, v8)
        }
    } else {
        Ok(Adt17::C0)
    }
}

pub fn f257(v0: u64, v1: Adt20, v2: Adt20, v3: u64, v4: u64, v5: u64, v6: u64) -> R<Adt17> {
    let v7: u64 = {
        let a5734 = v5;
        let a5735 = 6u64;
        nat_sub(a5734, a5735)
    };
    let v8: Adt20 = f270(v2);
    let v9: u64 = f240(match v1.clone() {
        Adt20::C0(h5736, _, _, _, _, _, _, _) => {
            h5736
        }
    }, match v1.clone() {
        Adt20::C0(_, h5737, _, _, _, _, _, _) => {
            h5737
        }
    }, match v1.clone() {
        Adt20::C0(_, _, h5738, _, _, _, _, _) => {
            h5738
        }
    }, match v1.clone() {
        Adt20::C0(_, _, _, h5739, _, _, _, _) => {
            h5739
        }
    }, v7)?;
    let a5744 = v0;
    let a5745 = f183(match v1.clone() {
        Adt20::C0(h5740, _, _, _, _, _, _, _) => {
            h5740
        }
    }, match v1.clone() {
        Adt20::C0(_, h5741, _, _, _, _, _, _) => {
            h5741
        }
    }, match v1.clone() {
        Adt20::C0(_, _, h5742, _, _, _, _, _) => {
            h5742
        }
    }, match v1.clone() {
        Adt20::C0(_, _, _, h5743, _, _, _, _) => {
            h5743
        }
    }, v4, v7)?;
    if nat_lt(a5744, a5745) {
        let a5746 = v0;
        let a5747 = v9;
        if nat_lt(a5746, a5747) {
            f271(v0, v8, 4u64, 6u64, v6)
        } else {
            let v10: Adt21 = f274({
                let a5748 = v0;
                let a5749 = v9;
                nat_sub(a5748, a5749)
            }, match v1.clone() {
                Adt20::C0(h5750, _, _, _, _, _, _, _) => {
                    h5750
                }
            }, match v1.clone() {
                Adt20::C0(_, h5751, _, _, _, _, _, _) => {
                    h5751
                }
            }, match v1.clone() {
                Adt20::C0(_, _, h5752, _, _, _, _, _) => {
                    h5752
                }
            }, match v1.clone() {
                Adt20::C0(_, _, _, h5753, _, _, _, _) => {
                    h5753
                }
            }, v4)?;
            f275(v10, v1, v8, v3)
        }
    } else {
        Ok(Adt17::C0)
    }
}

pub fn f258(v0: bool, v1: u64) -> R<u64> {
    let a5756 = if v0 {
        5u64
    } else {
        4u64
    };
    let a5757 = {
        let a5754 = f276(v0);
        let a5755 = v1;
        nat_mul(a5754, a5755)?
    };
    nat_add(a5756, a5757)
}

pub fn f259(v0: bool, v1: u64) -> R<u64> {
    if v0 {
        let a5765 = {
            let a5758 = 13u64;
            let a5759 = f277(v1);
            nat_mul(a5758, a5759)?
        };
        let a5766 = {
            let a5763 = 4u64;
            let a5764 = {
                let a5760 = v1;
                let a5761 = 4u64;
                let a5762 = 0u64;
                nat_rem(a5760, a5761, a5762)
            };
            nat_mul(a5763, a5764)?
        };
        nat_add(a5765, a5766)
    } else {
        let a5767 = 3u64;
        let a5768 = v1;
        nat_mul(a5767, a5768)
    }
}

pub fn f260(v0: bool) -> u64 {
    if v0 {
        6u64
    } else {
        5u64
    }
}

pub fn f261(v0: u64, v1: u64) -> R<u64> {
    let a5773 = {
        let a5771 = {
            let a5769 = v0;
            let a5770 = v1;
            nat_add(a5769, a5770)?
        };
        let a5772 = 1u64;
        nat_sub(a5771, a5772)
    };
    let a5774 = v1;
    let a5775 = 0u64;
    Ok(nat_quot(a5773, a5774, a5775))
}

pub fn f262(v0: u64, v1: u64) -> R<u64> {
    let a5776 = v0;
    let a5777 = f261(v0, v1)?;
    let a5778 = 0u64;
    Ok(nat_quot(a5776, a5777, a5778))
}

pub fn f263(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let a5779 = v2;
    let a5780 = 0u64;
    if nat_eq(a5779, a5780) {
        Ok(0u64)
    } else {
        let a5785 = 11u64;
        let a5786 = {
            let a5783 = {
                let a5781 = v1;
                let a5782 = v2;
                nat_mul(a5781, a5782)?
            };
            let a5784 = f278(v0)?;
            nat_mul(a5783, a5784)?
        };
        nat_add(a5785, a5786)
    }
}

pub fn f264(v0: u64, v1: u64) -> R<u64> {
    let a5787 = v0;
    let a5788 = f261(v0, v1)?;
    let a5789 = 0u64;
    Ok(nat_rem(a5787, a5788, a5789))
}

pub fn f265(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64) -> bool {
    if f244(v1, v2, v3, v4) {
        f279(v0)
    } else {
        f196(v0)
    }
}

pub fn f266(v0: u64) -> u64 {
    let a5790 = v0;
    let a5791 = 26u64;
    if nat_eq(a5790, a5791) {
        26u64
    } else {
        0u64
    }
}

pub fn f267(v0: u64, v1: u64) -> Option<u64> {
    let a5792 = v0;
    let a5793 = 2u64;
    if nat_eq(a5792, a5793) {
        Some(0u64)
    } else {
        let a5794 = v0;
        let a5795 = 5u64;
        if nat_eq(a5794, a5795) {
            Some(1u64)
        } else {
            let a5796 = v0;
            let a5797 = 6u64;
            if nat_eq(a5796, a5797) {
                Some(2u64)
            } else {
                let a5798 = v0;
                let a5799 = 7u64;
                if nat_eq(a5798, a5799) {
                    Some(3u64)
                } else {
                    let a5800 = v0;
                    let a5801 = 9u64;
                    if nat_eq(a5800, a5801) {
                        Some(4u64)
                    } else {
                        let a5802 = v0;
                        let a5803 = 8u64;
                        if nat_eq(a5802, a5803) {
                            Some(5u64)
                        } else {
                            let a5804 = v0;
                            let a5805 = 89u64;
                            if nat_eq(a5804, a5805) {
                                let a5806 = v1;
                                let a5807 = 2u64;
                                if nat_eq(a5806, a5807) {
                                    Some(6u64)
                                } else {
                                    let a5808 = v1;
                                    let a5809 = 0u64;
                                    if nat_eq(a5808, a5809) {
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

pub fn f268(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64, v7: u64, v8: u64, v9: u64, v10: u64, v11: u64, v12: u64, v13: u64, v14: u64, v15: u64) -> R<Adt22> {
    let a5810 = v0;
    let a5811 = 30u64;
    if nat_eq(a5810, a5811) {
        let a5812 = v2;
        let a5813 = 0u64;
        if if if nat_eq(a5812, a5813) {
            true
        } else {
            let a5814 = v2;
            let a5815 = 1u64;
            nat_eq(a5814, a5815)
        } {
            let a5816 = v1;
            let a5817 = 0u64;
            if nat_eq(a5816, a5817) {
                true
            } else {
                f280(v1, v4)
            }
        } else {
            false
        } {
            f281(28u64, 0u64, 0u64, v3, v5, v4, v6, v7, v9, v8, v10, v11, v12, v13, v14, v15)
        } else {
            Ok(Adt22::C0)
        }
    } else {
        if if if if f282(v1) {
            f282(v2)
        } else {
            false
        } {
            let a5818 = v1;
            let a5819 = 0u64;
            if if nat_eq(a5818, a5819) {
                true
            } else {
                let a5820 = v2;
                let a5821 = 0u64;
                nat_eq(a5820, a5821)
            } {
                true
            } else {
                let a5822 = v1;
                let a5823 = v2;
                nat_eq(a5822, a5823)
            }
        } else {
            false
        } {
            true
        } else {
            let a5824 = v1;
            let a5825 = 43u64;
            if nat_eq(a5824, a5825) {
                f282(v2)
            } else {
                false
            }
        } {
            f281(v0, 0u64, 0u64, v3, v4, v5, v6, v7, v8, v9, v10, v11, v12, v13, v14, v15)
        } else {
            Ok(Adt22::C0)
        }
    }
}

pub fn f269(v0: u64, v1: u64, v2: Adt20, v3: Adt20, v4: u64, v5: u64) -> R<Adt17> {
    let v6: u64 = match v2.clone() {
        Adt20::C0(h5826, _, _, _, _, _, _, _) => {
            h5826
        }
    };
    let v7: u64 = {
        let a5829 = 1u64;
        let a5830 = {
            let a5827 = 3u64;
            let a5828 = v6;
            nat_mul(a5827, a5828)?
        };
        nat_add(a5829, a5830)?
    };
    let a5831 = v0;
    let a5832 = 0u64;
    if nat_eq(a5831, a5832) {
        Ok(f283(0u64))
    } else {
        let a5833 = v0;
        let a5834 = v7;
        if nat_lt(a5833, a5834) {
            let v8: u64 = {
                let a5835 = v0;
                let a5836 = 1u64;
                nat_sub(a5835, a5836)
            };
            let v9: u64 = {
                let a5842 = {
                    let a5837 = v1;
                    let a5838 = v6;
                    nat_mul(a5837, a5838)?
                };
                let a5843 = {
                    let a5839 = v8;
                    let a5840 = 3u64;
                    let a5841 = 0u64;
                    nat_quot(a5839, a5840, a5841)
                };
                nat_add(a5842, a5843)?
            };
            let v10: u64 = f284(v9, v2)?;
            let v11: u64 = {
                let a5844 = v8;
                let a5845 = 3u64;
                let a5846 = 0u64;
                nat_rem(a5844, a5845, a5846)
            };
            let a5847 = v11;
            let a5848 = 0u64;
            if nat_eq(a5847, a5848) {
                Ok(f285(10u64, 0u64, v10, 0u64, v10, 1u64, 1u64))
            } else {
                let a5849 = v11;
                let a5850 = 1u64;
                if nat_eq(a5849, a5850) {
                    Ok(f285(1u64, 1u64, 1u64, 2u64, 0u64, 1u64, 1u64))
                } else {
                    Ok(f285(0u64, 1u64, 0u64, 1u64, 1u64, 1u64, 0u64))
                }
            }
        } else {
            let a5851 = v0;
            let a5852 = v7;
            if nat_eq(a5851, a5852) {
                Ok(f285(2u64, 1u64, 0u64, 2u64, 0u64, 1u64, 1u64))
            } else {
                let a5855 = v0;
                let a5856 = {
                    let a5853 = v7;
                    let a5854 = 1u64;
                    nat_add(a5853, a5854)?
                };
                if nat_eq(a5855, a5856) {
                    Ok(f285(12u64, 1u64, 1u64, 2u64, 0u64, 1u64, 1u64))
                } else {
                    let a5859 = v0;
                    let a5860 = {
                        let a5857 = v7;
                        let a5858 = 2u64;
                        nat_add(a5857, a5858)?
                    };
                    if nat_eq(a5859, a5860) {
                        Ok(f285(16u64, 1u64, 1u64, 2u64, v4, 1u64, 2u64))
                    } else {
                        let a5863 = v0;
                        let a5864 = {
                            let a5861 = v7;
                            let a5862 = 3u64;
                            nat_add(a5861, a5862)?
                        };
                        if nat_eq(a5863, a5864) {
                            let a5865 = v5;
                            let a5866 = 1u64;
                            if if nat_eq(a5865, a5866) {
                                true
                            } else {
                                let a5867 = v5;
                                let a5868 = 2u64;
                                nat_eq(a5867, a5868)
                            } {
                                Ok(f285(11u64, 2u64, 1065353216u64, 1u64, 1u64, 1u64, 2u64))
                            } else {
                                Ok(f285(11u64, 2u64, 1065353216u64, 2u64, v4, 1u64, 2u64))
                            }
                        } else {
                            let v12: u64 = {
                                let a5875 = {
                                    let a5869 = v1;
                                    let a5870 = v6;
                                    nat_mul(a5869, a5870)?
                                };
                                let a5876 = {
                                    let a5873 = v0;
                                    let a5874 = {
                                        let a5871 = v7;
                                        let a5872 = 4u64;
                                        nat_add(a5871, a5872)?
                                    };
                                    nat_sub(a5873, a5874)
                                };
                                nat_add(a5875, a5876)?
                            };
                            Ok(f285(10u64, 0u64, f284(v12, v2)?, 1u64, 2u64, 3u64, f284(v12, v3)?))
                        }
                    }
                }
            }
        }
    }
}

pub fn f270(v0: Adt20) -> Adt20 {
    Adt20::C0(match v0.clone() {
        Adt20::C0(h5877, _, _, _, _, _, _, _) => {
            h5877
        }
    }, match v0.clone() {
        Adt20::C0(_, h5878, _, _, _, _, _, _) => {
            h5878
        }
    }, match v0.clone() {
        Adt20::C0(_, _, h5879, _, _, _, _, _) => {
            h5879
        }
    }, match v0.clone() {
        Adt20::C0(_, _, _, h5880, _, _, _, _) => {
            h5880
        }
    }, 4u64, match v0.clone() {
        Adt20::C0(_, _, _, _, _, h5881, _, _) => {
            h5881
        }
    }, match v0.clone() {
        Adt20::C0(_, _, _, _, _, _, h5882, _) => {
            h5882
        }
    }, match v0 {
        Adt20::C0(_, _, _, _, _, _, _, h5883) => {
            h5883
        }
    })
}

pub fn f271(v0: u64, v1: Adt20, v2: u64, v3: u64, v4: u64) -> R<Adt17> {
    let v5: u64 = {
        let a5890 = v3;
        let a5891 = {
            let a5887 = f284({
                let a5884 = v0;
                let a5885 = 2u64;
                let a5886 = 0u64;
                nat_quot(a5884, a5885, a5886)
            }, v1)?;
            let a5888 = v2;
            let a5889 = 0u64;
            nat_quot(a5887, a5888, a5889)
        };
        nat_add(a5890, a5891)?
    };
    let a5895 = {
        let a5892 = v0;
        let a5893 = 2u64;
        let a5894 = 0u64;
        nat_rem(a5892, a5893, a5894)
    };
    let a5896 = 0u64;
    if nat_eq(a5895, a5896) {
        Ok(f285(19u64, 1u64, v5, 2u64, 0u64, 1u64, 2u64))
    } else {
        let a5897 = v4;
        let a5898 = 0u64;
        if nat_eq(a5897, a5898) {
            Ok(f285(19u64, 2u64, 1u64, 2u64, 0u64, 1u64, v5))
        } else {
            Ok(Adt17::C0)
        }
    }
}

pub fn f272(v0: u64, v1: u64, v2: Adt20, v3: Adt20, v4: Adt20, v5: u64) -> R<Adt17> {
    let v6: u64 = f286(match v2.clone() {
        Adt20::C0(h5899, _, _, _, _, _, _, _) => {
            h5899
        }
    })?;
    let v7: u64 = f287(match v2.clone() {
        Adt20::C0(h5900, _, _, _, _, _, _, _) => {
            h5900
        }
    })?;
    let a5901 = v0;
    let a5902 = 2u64;
    if nat_lt(a5901, a5902) {
        Ok(f283(v0))
    } else {
        let a5903 = v0;
        let a5904 = v6;
        if nat_lt(a5903, a5904) {
            f288({
                let a5905 = v0;
                let a5906 = 2u64;
                nat_sub(a5905, a5906)
            }, v1, v2, v3)
        } else {
            let a5907 = v0;
            let a5908 = v7;
            if nat_lt(a5907, a5908) {
                Ok(f289({
                    let a5909 = v0;
                    let a5910 = v6;
                    nat_sub(a5909, a5910)
                }, match v2 {
                    Adt20::C0(h5911, _, _, _, _, _, _, _) => {
                        h5911
                    }
                }, v5))
            } else {
                let a5913 = v0;
                let a5914 = f242(match v2.clone() {
                    Adt20::C0(h5912, _, _, _, _, _, _, _) => {
                        h5912
                    }
                })?;
                if nat_lt(a5913, a5914) {
                    f290({
                        let a5915 = v0;
                        let a5916 = v7;
                        nat_sub(a5915, a5916)
                    }, v1, v2, v3, v4)
                } else {
                    Ok(Adt17::C0)
                }
            }
        }
    }
}

pub fn f273(v0: u64, v1: u64, v2: Adt20, v3: Adt20, v4: u64, v5: bool) -> R<Adt17> {
    let v6: u64 = f258(v5, match v2.clone() {
        Adt20::C0(h5917, _, _, _, _, _, _, _) => {
            h5917
        }
    })?;
    let v7: u64 = {
        let a5919 = v6;
        let a5920 = f259(v5, match v2.clone() {
            Adt20::C0(h5918, _, _, _, _, _, _, _) => {
                h5918
            }
        })?;
        nat_add(a5919, a5920)?
    };
    let v8: u64 = {
        let a5921 = v7;
        let a5922 = f260(v5);
        nat_add(a5921, a5922)?
    };
    let a5923 = v0;
    let a5924 = v6;
    if nat_lt(a5923, a5924) {
        f291(v0, v1, v2, v5)
    } else {
        let a5925 = v0;
        let a5926 = v7;
        if nat_lt(a5925, a5926) {
            f292({
                let a5927 = v0;
                let a5928 = v6;
                nat_sub(a5927, a5928)
            }, v1, v2, v3, v5)
        } else {
            let a5929 = v0;
            let a5930 = v8;
            if nat_lt(a5929, a5930) {
                Ok(f293({
                    let a5931 = v0;
                    let a5932 = v7;
                    nat_sub(a5931, a5932)
                }, match v2 {
                    Adt20::C0(h5933, _, _, _, _, _, _, _) => {
                        h5933
                    }
                }, v4, v5))
            } else {
                let a5937 = v0;
                let a5938 = {
                    let a5935 = v8;
                    let a5936 = match v2.clone() {
                        Adt20::C0(h5934, _, _, _, _, _, _, _) => {
                            h5934
                        }
                    };
                    nat_add(a5935, a5936)?
                };
                if nat_lt(a5937, a5938) {
                    let v9: u64 = f284({
                        let a5944 = {
                            let a5940 = v1;
                            let a5941 = match v2 {
                                Adt20::C0(h5939, _, _, _, _, _, _, _) => {
                                    h5939
                                }
                            };
                            nat_mul(a5940, a5941)?
                        };
                        let a5945 = {
                            let a5942 = v0;
                            let a5943 = v8;
                            nat_sub(a5942, a5943)
                        };
                        nat_add(a5944, a5945)?
                    }, v3)?;
                    Ok(f285(10u64, 3u64, v9, 1u64, 1u64, 3u64, v9))
                } else {
                    Ok(Adt17::C0)
                }
            }
        }
    }
}

pub fn f274(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64) -> R<Adt21> {
    let v6: u64 = f261(v3, v5)?;
    let v7: u64 = f263(v1, v2, v6)?;
    let v8: u64 = {
        let a5948 = {
            let a5946 = f262(v3, v5)?;
            let a5947 = v4;
            nat_mul(a5946, a5947)?
        };
        let a5949 = v7;
        nat_mul(a5948, a5949)?
    };
    let a5950 = v0;
    let a5951 = v8;
    if nat_lt(a5950, a5951) {
        let v9: u64 = {
            let a5952 = v0;
            let a5953 = v7;
            let a5954 = 0u64;
            nat_quot(a5952, a5953, a5954)
        };
        Ok(Adt21::C0({
            let a5958 = {
                let a5955 = v9;
                let a5956 = v4;
                let a5957 = 0u64;
                nat_quot(a5955, a5956, a5957)
            };
            let a5959 = v6;
            nat_mul(a5958, a5959)?
        }, v6, {
            let a5960 = v9;
            let a5961 = v4;
            let a5962 = 0u64;
            nat_rem(a5960, a5961, a5962)
        }, {
            let a5963 = v0;
            let a5964 = v7;
            let a5965 = 0u64;
            nat_rem(a5963, a5964, a5965)
        }))
    } else {
        let v10: u64 = {
            let a5966 = v0;
            let a5967 = v8;
            nat_sub(a5966, a5967)
        };
        let v11: u64 = f263(v1, v2, f264(v3, v5)?)?;
        Ok(Adt21::C0({
            let a5968 = f262(v3, v5)?;
            let a5969 = v6;
            nat_mul(a5968, a5969)?
        }, f264(v3, v5)?, {
            let a5970 = v10;
            let a5971 = v11;
            let a5972 = 0u64;
            nat_quot(a5970, a5971, a5972)
        }, {
            let a5973 = v10;
            let a5974 = v11;
            let a5975 = 0u64;
            nat_rem(a5973, a5974, a5975)
        }))
    }
}

pub fn f275(v0: Adt21, v1: Adt20, v2: Adt20, v3: u64) -> R<Adt17> {
    let v4: u64 = match v0.clone() {
        Adt21::C0(_, _, _, h5976) => {
            h5976
        }
    };
    let v5: u64 = match v1.clone() {
        Adt20::C0(h5977, _, _, _, _, _, _, _) => {
            h5977
        }
    };
    let v6: u64 = {
        let a5980 = match v0.clone() {
            Adt21::C0(_, h5978, _, _) => {
                h5978
            }
        };
        let a5981 = match v1.clone() {
            Adt20::C0(_, h5979, _, _, _, _, _, _) => {
                h5979
            }
        };
        nat_mul(a5980, a5981)?
    };
    let v7: u64 = {
        let a5982 = v6;
        let a5983 = v5;
        nat_mul(a5982, a5983)?
    };
    let v8: u64 = {
        let a5986 = {
            let a5984 = 2u64;
            let a5985 = v5;
            nat_mul(a5984, a5985)?
        };
        let a5987 = 2u64;
        nat_add(a5986, a5987)?
    };
    let v9: u64 = {
        let a5990 = {
            let a5988 = 4u64;
            let a5989 = v5;
            nat_mul(a5988, a5989)?
        };
        let a5991 = 2u64;
        nat_add(a5990, a5991)?
    };
    let v10: u64 = {
        let a5994 = 1u64;
        let a5995 = {
            let a5992 = v6;
            let a5993 = v8;
            nat_mul(a5992, a5993)?
        };
        nat_add(a5994, a5995)?
    };
    let v11: u64 = {
        let a5996 = v10;
        let a5997 = 4u64;
        nat_add(a5996, a5997)?
    };
    let v12: u64 = {
        let a6000 = v11;
        let a6001 = {
            let a5998 = v6;
            let a5999 = v9;
            nat_mul(a5998, a5999)?
        };
        nat_add(a6000, a6001)?
    };
    let v13: u64 = {
        let a6002 = v12;
        let a6003 = 6u64;
        nat_add(a6002, a6003)?
    };
    let a6004 = v4;
    let a6005 = 0u64;
    if nat_eq(a6004, a6005) {
        Ok(f283(0u64))
    } else {
        let a6006 = v4;
        let a6007 = v10;
        if nat_lt(a6006, a6007) {
            let v14: u64 = {
                let a6008 = v4;
                let a6009 = 1u64;
                nat_sub(a6008, a6009)
            };
            f294(false, {
                let a6010 = v14;
                let a6011 = v8;
                let a6012 = 0u64;
                nat_rem(a6010, a6011, a6012)
            }, {
                let a6013 = v14;
                let a6014 = v8;
                let a6015 = 0u64;
                nat_quot(a6013, a6014, a6015)
            }, v0, v1, v2)
        } else {
            let a6016 = v4;
            let a6017 = v11;
            if nat_lt(a6016, a6017) {
                Ok(f295({
                    let a6018 = v4;
                    let a6019 = v10;
                    nat_sub(a6018, a6019)
                }, v7))
            } else {
                let a6020 = v4;
                let a6021 = v12;
                if nat_lt(a6020, a6021) {
                    let v15: u64 = {
                        let a6022 = v4;
                        let a6023 = v11;
                        nat_sub(a6022, a6023)
                    };
                    f294(true, {
                        let a6024 = v15;
                        let a6025 = v9;
                        let a6026 = 0u64;
                        nat_rem(a6024, a6025, a6026)
                    }, {
                        let a6027 = v15;
                        let a6028 = v9;
                        let a6029 = 0u64;
                        nat_quot(a6027, a6028, a6029)
                    }, v0, v1, v2)
                } else {
                    let a6030 = v4;
                    let a6031 = v13;
                    if nat_lt(a6030, a6031) {
                        Ok(f296({
                            let a6032 = v4;
                            let a6033 = v12;
                            nat_sub(a6032, a6033)
                        }, v7, v3))
                    } else {
                        let a6036 = v4;
                        let a6037 = {
                            let a6034 = v13;
                            let a6035 = v7;
                            nat_add(a6034, a6035)?
                        };
                        if nat_lt(a6036, a6037) {
                            let v16: u64 = {
                                let a6038 = v4;
                                let a6039 = v13;
                                nat_sub(a6038, a6039)
                            };
                            let v17: u64 = {
                                let a6040 = v16;
                                let a6041 = v5;
                                let a6042 = 0u64;
                                nat_quot(a6040, a6041, a6042)
                            };
                            let v18: u64 = {
                                let a6048 = {
                                    let a6043 = f297(v17, v0, v1)?;
                                    let a6044 = v5;
                                    nat_mul(a6043, a6044)?
                                };
                                let a6049 = {
                                    let a6045 = v16;
                                    let a6046 = v5;
                                    let a6047 = 0u64;
                                    nat_rem(a6045, a6046, a6047)
                                };
                                nat_add(a6048, a6049)?
                            };
                            let v19: u64 = f284(v18, v2)?;
                            Ok(f285(10u64, 3u64, v19, 1u64, 1u64, 3u64, v19))
                        } else {
                            Ok(Adt17::C0)
                        }
                    }
                }
            }
        }
    }
}

pub fn f276(v0: bool) -> u64 {
    if v0 {
        2u64
    } else {
        1u64
    }
}

pub fn f277(v0: u64) -> u64 {
    let a6050 = v0;
    let a6051 = 4u64;
    let a6052 = 0u64;
    nat_quot(a6050, a6051, a6052)
}

pub fn f278(v0: u64) -> R<u64> {
    let a6055 = {
        let a6053 = 7u64;
        let a6054 = v0;
        nat_mul(a6053, a6054)?
    };
    let a6056 = 4u64;
    nat_add(a6055, a6056)
}

pub fn f279(v0: u64) -> bool {
    let m6057 = f187(v0);
    match m6057 {
        Adt18::C0 => {
            false
        }
        Adt18::C1(_) => {
            true
        }
    }
}

pub fn f280(v0: u64, v1: u64) -> bool {
    let a6058 = v0;
    let a6059 = 1u64;
    if nat_eq(a6058, a6059) {
        true
    } else {
        let v2: u64 = f298(v0);
        let a6060 = 0u64;
        let a6061 = v2;
        if nat_lt(a6060, a6061) {
            let a6065 = {
                let a6062 = v1;
                let a6063 = v2;
                let a6064 = 1u64;
                nat_rem(a6062, a6063, a6064)
            };
            let a6066 = 0u64;
            nat_eq(a6065, a6066)
        } else {
            false
        }
    }
}

pub fn f281(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64, v7: u64, v8: u64, v9: u64, v10: u64, v11: u64, v12: u64, v13: u64, v14: u64, v15: u64) -> R<Adt22> {
    let a6067 = v0;
    let a6068 = 28u64;
    if if if if nat_eq(a6067, a6068) {
        let a6069 = v1;
        let a6070 = 0u64;
        nat_eq(a6069, a6070)
    } else {
        false
    } {
        let a6071 = v2;
        let a6072 = 0u64;
        nat_eq(a6071, a6072)
    } else {
        false
    } {
        let a6073 = v3;
        let a6074 = 0u64;
        nat_eq(a6073, a6074)
    } else {
        false
    } {
        let a6075 = v4;
        let a6076 = v8;
        if if if if if if if if nat_eq(a6075, a6076) {
            let a6077 = v5;
            let a6078 = v12;
            nat_eq(a6077, a6078)
        } else {
            false
        } {
            let a6079 = v9;
            let a6080 = v13;
            nat_eq(a6079, a6080)
        } else {
            false
        } {
            let a6081 = v10;
            let a6082 = v14;
            nat_eq(a6081, a6082)
        } else {
            false
        } {
            let a6083 = v11;
            let a6084 = v15;
            nat_eq(a6083, a6084)
        } else {
            false
        } {
            let a6085 = 0u64;
            let a6086 = v6;
            nat_lt(a6085, a6086)
        } else {
            false
        } {
            let a6087 = 0u64;
            let a6088 = v7;
            nat_lt(a6087, a6088)
        } else {
            false
        } {
            let a6092 = {
                let a6089 = v10;
                let a6090 = v6;
                let a6091 = 1u64;
                nat_rem(a6089, a6090, a6091)
            };
            let a6093 = 0u64;
            if nat_eq(a6092, a6093) {
                let a6097 = {
                    let a6094 = v11;
                    let a6095 = v7;
                    let a6096 = 1u64;
                    nat_rem(a6094, a6095, a6096)
                };
                let a6098 = 0u64;
                nat_eq(a6097, a6098)
            } else {
                false
            }
        } else {
            false
        } {
            Ok(Adt22::C2(v5, v4, v9, v10, v11, {
                let a6099 = v10;
                let a6100 = v6;
                let a6101 = 0u64;
                nat_quot(a6099, a6100, a6101)
            }, {
                let a6102 = v11;
                let a6103 = v7;
                let a6104 = 0u64;
                nat_quot(a6102, a6103, a6104)
            }, {
                let a6105 = v5;
                let a6106 = v4;
                nat_mul(a6105, a6106)?
            }, {
                let a6107 = v4;
                let a6108 = v9;
                nat_mul(a6107, a6108)?
            }, {
                let a6109 = v5;
                let a6110 = v9;
                nat_mul(a6109, a6110)?
            }))
        } else {
            Ok(Adt22::C1)
        }
    } else {
        Ok(Adt22::C0)
    }
}

pub fn f282(v0: u64) -> bool {
    let a6111 = v0;
    let a6112 = 0u64;
    if if nat_eq(a6111, a6112) {
        true
    } else {
        let a6113 = v0;
        let a6114 = 1u64;
        nat_eq(a6113, a6114)
    } {
        true
    } else {
        let a6115 = v0;
        let a6116 = 30u64;
        nat_eq(a6115, a6116)
    }
}

pub fn f283(v0: u64) -> Adt17 {
    f285(0u64, 2u64, 0u64, 2u64, 0u64, 1u64, v0)
}

pub fn f284(v0: u64, v1: Adt20) -> R<u64> {
    let v2: u64 = {
        let a6118 = v0;
        let a6119 = match v1.clone() {
            Adt20::C0(h6117, _, _, _, _, _, _, _) => {
                h6117
            }
        };
        let a6120 = 0u64;
        nat_quot(a6118, a6119, a6120)
    };
    let v3: u64 = {
        let a6122 = v2;
        let a6123 = match v1.clone() {
            Adt20::C0(_, h6121, _, _, _, _, _, _) => {
                h6121
            }
        };
        let a6124 = 0u64;
        nat_quot(a6122, a6123, a6124)
    };
    f117({
        let a6126 = v0;
        let a6127 = match v1.clone() {
            Adt20::C0(h6125, _, _, _, _, _, _, _) => {
                h6125
            }
        };
        let a6128 = 0u64;
        nat_rem(a6126, a6127, a6128)
    }, {
        let a6130 = v2;
        let a6131 = match v1.clone() {
            Adt20::C0(_, h6129, _, _, _, _, _, _) => {
                h6129
            }
        };
        let a6132 = 0u64;
        nat_rem(a6130, a6131, a6132)
    }, {
        let a6134 = v3;
        let a6135 = match v1.clone() {
            Adt20::C0(_, _, h6133, _, _, _, _, _) => {
                h6133
            }
        };
        let a6136 = 0u64;
        nat_rem(a6134, a6135, a6136)
    }, {
        let a6138 = v3;
        let a6139 = match v1.clone() {
            Adt20::C0(_, _, h6137, _, _, _, _, _) => {
                h6137
            }
        };
        let a6140 = 0u64;
        nat_quot(a6138, a6139, a6140)
    }, match v1.clone() {
        Adt20::C0(_, _, _, _, h6141, _, _, _) => {
            h6141
        }
    }, match v1.clone() {
        Adt20::C0(_, _, _, _, _, h6142, _, _) => {
            h6142
        }
    }, match v1.clone() {
        Adt20::C0(_, _, _, _, _, _, h6143, _) => {
            h6143
        }
    }, match v1 {
        Adt20::C0(_, _, _, _, _, _, _, h6144) => {
            h6144
        }
    })
}

pub fn f285(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64) -> Adt17 {
    let a6145 = v0;
    let a6146 = 4u64;
    if if if nat_le(a6145, a6146) {
        true
    } else {
        let a6147 = 9u64;
        let a6148 = v0;
        if nat_le(a6147, a6148) {
            let a6149 = v0;
            let a6150 = 12u64;
            nat_le(a6149, a6150)
        } else {
            false
        }
    } {
        true
    } else {
        let a6151 = v0;
        let a6152 = 16u64;
        if if nat_eq(a6151, a6152) {
            true
        } else {
            let a6153 = v0;
            let a6154 = 19u64;
            nat_eq(a6153, a6154)
        } {
            true
        } else {
            let a6155 = v0;
            let a6156 = 21u64;
            if nat_eq(a6155, a6156) {
                true
            } else {
                let a6157 = v0;
                let a6158 = 22u64;
                nat_eq(a6157, a6158)
            }
        }
    } {
        Adt17::C1({
            let a6159 = v0;
            let a6160 = 4u64;
            if if nat_lt(a6159, a6160) {
                true
            } else {
                let a6161 = 21u64;
                let a6162 = v0;
                nat_le(a6161, a6162)
            } {
                0u64
            } else {
                let a6163 = v0;
                let a6164 = 16u64;
                if if nat_eq(a6163, a6164) {
                    true
                } else {
                    let a6165 = v0;
                    let a6166 = 19u64;
                    nat_eq(a6165, a6166)
                } {
                    2u64
                } else {
                    1u64
                }
            }
        }, {
            let a6167 = v0;
            let a6168 = 4u64;
            if nat_lt(a6167, a6168) {
                v0
            } else {
                let a6169 = v0;
                let a6170 = 4u64;
                if if nat_eq(a6169, a6170) {
                    true
                } else {
                    let a6171 = v0;
                    let a6172 = 16u64;
                    nat_eq(a6171, a6172)
                } {
                    0u64
                } else {
                    let a6173 = v0;
                    let a6174 = 19u64;
                    if nat_eq(a6173, a6174) {
                        3u64
                    } else {
                        let a6175 = 21u64;
                        let a6176 = v0;
                        if nat_le(a6175, a6176) {
                            let a6177 = v0;
                            let a6178 = 17u64;
                            nat_sub(a6177, a6178)
                        } else {
                            let a6179 = v0;
                            let a6180 = 8u64;
                            nat_sub(a6179, a6180)
                        }
                    }
                }
            }
        }, v1, v2, v3, v4, v5, v6)
    } else {
        Adt17::C0
    }
}

pub fn f286(v0: u64) -> R<u64> {
    let a6183 = 2u64;
    let a6184 = {
        let a6181 = 6u64;
        let a6182 = v0;
        nat_mul(a6181, a6182)?
    };
    nat_add(a6183, a6184)
}

pub fn f287(v0: u64) -> R<u64> {
    let a6185 = f286(v0)?;
    let a6186 = 11u64;
    nat_add(a6185, a6186)
}

pub fn f288(v0: u64, v1: u64, v2: Adt20, v3: Adt20) -> R<Adt17> {
    let v4: u64 = {
        let a6187 = v0;
        let a6188 = 6u64;
        let a6189 = 0u64;
        nat_rem(a6187, a6188, a6189)
    };
    let v5: u64 = {
        let a6196 = {
            let a6191 = v1;
            let a6192 = match v2.clone() {
                Adt20::C0(h6190, _, _, _, _, _, _, _) => {
                    h6190
                }
            };
            nat_mul(a6191, a6192)?
        };
        let a6197 = {
            let a6193 = v0;
            let a6194 = 6u64;
            let a6195 = 0u64;
            nat_quot(a6193, a6194, a6195)
        };
        nat_add(a6196, a6197)?
    };
    let v6: u64 = f284(v5, v3)?;
    let v7: u64 = f284(v5, v2)?;
    let a6198 = v4;
    let a6199 = 0u64;
    if nat_eq(a6198, a6199) {
        Ok(f285(10u64, 4u64, v6, 4u64, v6, 1u64, 2u64))
    } else {
        let a6200 = v4;
        let a6201 = 1u64;
        if nat_eq(a6200, a6201) {
            Ok(f285(1u64, 1u64, 2u64, 2u64, 0u64, 1u64, 2u64))
        } else {
            let a6202 = v4;
            let a6203 = 2u64;
            if nat_eq(a6202, a6203) {
                Ok(f285(0u64, 1u64, 0u64, 1u64, 2u64, 1u64, 0u64))
            } else {
                let a6204 = v4;
                let a6205 = 3u64;
                if nat_eq(a6204, a6205) {
                    Ok(f285(10u64, 4u64, v6, 0u64, v7, 1u64, 2u64))
                } else {
                    let a6206 = v4;
                    let a6207 = 4u64;
                    if nat_eq(a6206, a6207) {
                        Ok(f285(1u64, 1u64, 2u64, 2u64, 0u64, 1u64, 2u64))
                    } else {
                        Ok(f285(0u64, 1u64, 1u64, 1u64, 2u64, 1u64, 1u64))
                    }
                }
            }
        }
    }
}

pub fn f289(v0: u64, v1: u64, v2: u64) -> Adt17 {
    let a6208 = v0;
    let a6209 = 0u64;
    if nat_eq(a6208, a6209) {
        f285(2u64, 1u64, 0u64, 2u64, 0u64, 1u64, 3u64)
    } else {
        let a6210 = v0;
        let a6211 = 1u64;
        if nat_eq(a6210, a6211) {
            f285(3u64, 2u64, v1, 2u64, 0u64, 1u64, 2u64)
        } else {
            let a6212 = v0;
            let a6213 = 2u64;
            if nat_eq(a6212, a6213) {
                f285(11u64, 1u64, 3u64, 1u64, 2u64, 1u64, 4u64)
            } else {
                let a6214 = v0;
                let a6215 = 3u64;
                if nat_eq(a6214, a6215) {
                    f285(4u64, 1u64, 4u64, 2u64, v2, 1u64, 4u64)
                } else {
                    let a6216 = v0;
                    let a6217 = 4u64;
                    if nat_eq(a6216, a6217) {
                        f285(12u64, 1u64, 4u64, 2u64, 0u64, 1u64, 4u64)
                    } else {
                        let a6218 = v0;
                        let a6219 = 5u64;
                        if nat_eq(a6218, a6219) {
                            f285(11u64, 2u64, 1065353216u64, 1u64, 4u64, 1u64, 4u64)
                        } else {
                            let a6220 = v0;
                            let a6221 = 6u64;
                            if nat_eq(a6220, a6221) {
                                f285(10u64, 2u64, v2, 1u64, 2u64, 1u64, 5u64)
                            } else {
                                let a6222 = v0;
                                let a6223 = 7u64;
                                if nat_eq(a6222, a6223) {
                                    f285(4u64, 1u64, 3u64, 1u64, 5u64, 1u64, 5u64)
                                } else {
                                    let a6224 = v0;
                                    let a6225 = 8u64;
                                    if nat_eq(a6224, a6225) {
                                        f285(2u64, 1u64, 1u64, 2u64, 0u64, 1u64, 2u64)
                                    } else {
                                        let a6226 = v0;
                                        let a6227 = 9u64;
                                        if nat_eq(a6226, a6227) {
                                            Adt17::C1(3u64, 0u64, 1u64, 2u64, 2u64, 2u64, 1u64, 2u64)
                                        } else {
                                            let a6228 = v0;
                                            let a6229 = 10u64;
                                            if nat_eq(a6228, a6229) {
                                                f285(11u64, 1u64, 2u64, 1u64, 5u64, 1u64, 5u64)
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

pub fn f290(v0: u64, v1: u64, v2: Adt20, v3: Adt20, v4: Adt20) -> R<Adt17> {
    let v5: u64 = {
        let a6230 = v0;
        let a6231 = 3u64;
        let a6232 = 0u64;
        nat_rem(a6230, a6231, a6232)
    };
    let v6: u64 = {
        let a6239 = {
            let a6234 = v1;
            let a6235 = match v2.clone() {
                Adt20::C0(h6233, _, _, _, _, _, _, _) => {
                    h6233
                }
            };
            nat_mul(a6234, a6235)?
        };
        let a6240 = {
            let a6236 = v0;
            let a6237 = 3u64;
            let a6238 = 0u64;
            nat_quot(a6236, a6237, a6238)
        };
        nat_add(a6239, a6240)?
    };
    let v7: u64 = f284(v6, v3)?;
    let v8: u64 = f284(v6, v2)?;
    let v9: u64 = f284(v6, v4)?;
    let a6241 = v5;
    let a6242 = 0u64;
    if nat_eq(a6241, a6242) {
        Ok(f285(10u64, 4u64, v7, 1u64, 5u64, 1u64, 2u64))
    } else {
        let a6243 = v5;
        let a6244 = 1u64;
        if nat_eq(a6243, a6244) {
            Ok(f285(4u64, 0u64, v8, 1u64, 2u64, 1u64, 2u64))
        } else {
            Ok(f285(10u64, 1u64, 2u64, 1u64, 4u64, 3u64, v9))
        }
    }
}

pub fn f291(v0: u64, v1: u64, v2: Adt20, v3: bool) -> R<Adt17> {
    let v4: u64 = match v2.clone() {
        Adt20::C0(h6245, _, _, _, _, _, _, _) => {
            h6245
        }
    };
    let v5: u64 = {
        let a6248 = 1u64;
        let a6249 = {
            let a6246 = v4;
            let a6247 = f276(v3);
            nat_mul(a6246, a6247)?
        };
        nat_add(a6248, a6249)?
    };
    let a6250 = v0;
    let a6251 = 0u64;
    if nat_eq(a6250, a6251) {
        Ok(f283(0u64))
    } else {
        let a6252 = v0;
        let a6253 = v5;
        if nat_lt(a6252, a6253) {
            let v6: u64 = {
                let a6254 = v0;
                let a6255 = 1u64;
                nat_sub(a6254, a6255)
            };
            let v7: u64 = f284({
                let a6261 = {
                    let a6256 = v1;
                    let a6257 = v4;
                    nat_mul(a6256, a6257)?
                };
                let a6262 = {
                    let a6258 = v6;
                    let a6259 = f276(v3);
                    let a6260 = 0u64;
                    nat_quot(a6258, a6259, a6260)
                };
                nat_add(a6261, a6262)?
            }, v2)?;
            if v3 {
                let a6266 = {
                    let a6263 = v6;
                    let a6264 = 2u64;
                    let a6265 = 0u64;
                    nat_rem(a6263, a6264, a6265)
                };
                let a6267 = 0u64;
                if nat_eq(a6266, a6267) {
                    Ok(f285(1u64, 0u64, v7, 2u64, 0u64, 1u64, 1u64))
                } else {
                    Ok(f285(0u64, 1u64, 0u64, 1u64, 1u64, 1u64, 0u64))
                }
            } else {
                Ok(f285(4u64, 1u64, 0u64, 0u64, v7, 1u64, 0u64))
            }
        } else {
            let v8: u64 = {
                let a6268 = v0;
                let a6269 = v5;
                nat_sub(a6268, a6269)
            };
            let a6270 = v8;
            let a6271 = 0u64;
            if nat_eq(a6270, a6271) {
                Ok(f285(3u64, 2u64, v4, 2u64, 0u64, 1u64, 1u64))
            } else {
                if if v3 {
                    let a6272 = v8;
                    let a6273 = 1u64;
                    nat_eq(a6272, a6273)
                } else {
                    false
                } {
                    Ok(f285(2u64, 1u64, 0u64, 2u64, 0u64, 1u64, 0u64))
                } else {
                    let a6274 = v8;
                    let a6275 = if v3 {
                        2u64
                    } else {
                        1u64
                    };
                    if nat_eq(a6274, a6275) {
                        Ok(f285(11u64, 1u64, 0u64, 1u64, 1u64, 1u64, 3u64))
                    } else {
                        let a6276 = v8;
                        let a6277 = if v3 {
                            3u64
                        } else {
                            2u64
                        };
                        if nat_eq(a6276, a6277) {
                            Ok(f283(0u64))
                        } else {
                            Ok(Adt17::C0)
                        }
                    }
                }
            }
        }
    }
}

pub fn f292(v0: u64, v1: u64, v2: Adt20, v3: Adt20, v4: bool) -> R<Adt17> {
    let v5: u64 = if v4 {
        let a6279 = 13u64;
        let a6280 = f277(match v2.clone() {
            Adt20::C0(h6278, _, _, _, _, _, _, _) => {
                h6278
            }
        });
        nat_mul(a6279, a6280)?
    } else {
        0u64
    };
    let a6281 = v0;
    let a6282 = v5;
    if nat_lt(a6281, a6282) {
        f299({
            let a6283 = v0;
            let a6284 = 13u64;
            let a6285 = 0u64;
            nat_rem(a6283, a6284, a6285)
        }, v1, {
            let a6286 = v0;
            let a6287 = 13u64;
            let a6288 = 0u64;
            nat_quot(a6286, a6287, a6288)
        }, v2, v3)
    } else {
        let v6: u64 = {
            let a6289 = v0;
            let a6290 = v5;
            nat_sub(a6289, a6290)
        };
        let v7: u64 = if v4 {
            4u64
        } else {
            3u64
        };
        f300({
            let a6291 = v6;
            let a6292 = v7;
            let a6293 = 0u64;
            nat_rem(a6291, a6292, a6293)
        }, {
            let a6303 = {
                let a6298 = {
                    let a6295 = v1;
                    let a6296 = match v2.clone() {
                        Adt20::C0(h6294, _, _, _, _, _, _, _) => {
                            h6294
                        }
                    };
                    nat_mul(a6295, a6296)?
                };
                let a6299 = if v4 {
                    f301(match v2.clone() {
                        Adt20::C0(h6297, _, _, _, _, _, _, _) => {
                            h6297
                        }
                    })?
                } else {
                    0u64
                };
                nat_add(a6298, a6299)?
            };
            let a6304 = {
                let a6300 = v6;
                let a6301 = v7;
                let a6302 = 0u64;
                nat_quot(a6300, a6301, a6302)
            };
            nat_add(a6303, a6304)?
        }, v2, v3, v4)
    }
}

pub fn f293(v0: u64, v1: u64, v2: u64, v3: bool) -> Adt17 {
    let a6305 = v0;
    let a6306 = 0u64;
    if nat_eq(a6305, a6306) {
        f285(if v3 {
            22u64
        } else {
            3u64
        }, 2u64, v1, 2u64, 0u64, 1u64, 1u64)
    } else {
        let a6307 = v0;
        let a6308 = 1u64;
        if nat_eq(a6307, a6308) {
            f285(if v3 {
                21u64
            } else {
                11u64
            }, 1u64, 0u64, 1u64, 1u64, 1u64, 1u64)
        } else {
            if if v3 {
                let a6309 = v0;
                let a6310 = 2u64;
                nat_eq(a6309, a6310)
            } else {
                false
            } {
                f285(2u64, 1u64, 1u64, 2u64, 0u64, 1u64, 1u64)
            } else {
                let a6311 = v0;
                let a6312 = if v3 {
                    3u64
                } else {
                    2u64
                };
                if nat_eq(a6311, a6312) {
                    f285(4u64, 1u64, 1u64, 2u64, v2, 1u64, 1u64)
                } else {
                    let a6313 = v0;
                    let a6314 = if v3 {
                        4u64
                    } else {
                        3u64
                    };
                    if nat_eq(a6313, a6314) {
                        f285(12u64, 1u64, 1u64, 2u64, 0u64, 1u64, 1u64)
                    } else {
                        let a6315 = v0;
                        let a6316 = if v3 {
                            5u64
                        } else {
                            4u64
                        };
                        if nat_eq(a6315, a6316) {
                            f285(11u64, 2u64, 1065353216u64, 1u64, 1u64, 1u64, 1u64)
                        } else {
                            Adt17::C0
                        }
                    }
                }
            }
        }
    }
}

pub fn f294(v0: bool, v1: u64, v2: u64, v3: Adt21, v4: Adt20, v5: Adt20) -> R<Adt17> {
    let v6: u64 = if v0 {
        4u64
    } else {
        2u64
    };
    let v7: u64 = {
        let a6320 = 1u64;
        let a6321 = {
            let a6318 = match v4.clone() {
                Adt20::C0(h6317, _, _, _, _, _, _, _) => {
                    h6317
                }
            };
            let a6319 = v6;
            nat_mul(a6318, a6319)?
        };
        nat_add(a6320, a6321)?
    };
    let a6322 = v1;
    let a6323 = 0u64;
    if nat_eq(a6322, a6323) {
        Ok(f283(1u64))
    } else {
        let a6324 = v1;
        let a6325 = v7;
        if nat_lt(a6324, a6325) {
            let v8: u64 = {
                let a6326 = v1;
                let a6327 = 1u64;
                nat_sub(a6326, a6327)
            };
            let v9: u64 = {
                let a6334 = {
                    let a6329 = f297(v2, v3, v4.clone())?;
                    let a6330 = match v4.clone() {
                        Adt20::C0(h6328, _, _, _, _, _, _, _) => {
                            h6328
                        }
                    };
                    nat_mul(a6329, a6330)?
                };
                let a6335 = {
                    let a6331 = v8;
                    let a6332 = v6;
                    let a6333 = 0u64;
                    nat_quot(a6331, a6332, a6333)
                };
                nat_add(a6334, a6335)?
            };
            let v10: u64 = f284(v9, v4)?;
            let v11: u64 = f284(v9, v5)?;
            let v12: u64 = {
                let a6336 = v8;
                let a6337 = v6;
                let a6338 = 0u64;
                nat_rem(a6336, a6337, a6338)
            };
            if v0 {
                let a6339 = v12;
                let a6340 = 0u64;
                if nat_eq(a6339, a6340) {
                    Ok(f285(9u64, 0u64, v10, 1u64, 3u64, 3u64, v11))
                } else {
                    let a6341 = v12;
                    let a6342 = 1u64;
                    if nat_eq(a6341, a6342) {
                        Ok(f285(10u64, 3u64, v11, 3u64, v11, 1u64, 2u64))
                    } else {
                        let a6343 = v12;
                        let a6344 = 2u64;
                        if nat_eq(a6343, a6344) {
                            Ok(f285(1u64, 1u64, 2u64, 2u64, 0u64, 1u64, 2u64))
                        } else {
                            Ok(f285(0u64, 1u64, 1u64, 1u64, 2u64, 1u64, 1u64))
                        }
                    }
                }
            } else {
                let a6345 = v12;
                let a6346 = 0u64;
                if nat_eq(a6345, a6346) {
                    Ok(f285(1u64, 0u64, v10, 2u64, 0u64, 1u64, 2u64))
                } else {
                    Ok(f285(0u64, 1u64, 1u64, 1u64, 2u64, 1u64, 1u64))
                }
            }
        } else {
            let a6347 = v1;
            let a6348 = v7;
            if nat_eq(a6347, a6348) {
                Ok(f285(0u64, 1u64, 0u64, 1u64, 1u64, 1u64, 0u64))
            } else {
                Ok(Adt17::C0)
            }
        }
    }
}

pub fn f295(v0: u64, v1: u64) -> Adt17 {
    let a6349 = v0;
    let a6350 = 0u64;
    if nat_eq(a6349, a6350) {
        f285(22u64, 2u64, v1, 2u64, 0u64, 1u64, 2u64)
    } else {
        let a6351 = v0;
        let a6352 = 1u64;
        if nat_eq(a6351, a6352) {
            f285(21u64, 1u64, 0u64, 1u64, 2u64, 1u64, 3u64)
        } else {
            let a6353 = v0;
            let a6354 = 2u64;
            if nat_eq(a6353, a6354) {
                f285(2u64, 1u64, 3u64, 2u64, 0u64, 1u64, 3u64)
            } else {
                let a6355 = v0;
                let a6356 = 3u64;
                if nat_eq(a6355, a6356) {
                    f283(0u64)
                } else {
                    Adt17::C0
                }
            }
        }
    }
}

pub fn f296(v0: u64, v1: u64, v2: u64) -> Adt17 {
    let a6357 = v0;
    let a6358 = 0u64;
    if nat_eq(a6357, a6358) {
        f285(22u64, 2u64, v1, 2u64, 0u64, 1u64, 2u64)
    } else {
        let a6359 = v0;
        let a6360 = 1u64;
        if nat_eq(a6359, a6360) {
            f285(21u64, 1u64, 0u64, 1u64, 2u64, 1u64, 2u64)
        } else {
            let a6361 = v0;
            let a6362 = 2u64;
            if nat_eq(a6361, a6362) {
                f285(2u64, 1u64, 2u64, 2u64, 0u64, 1u64, 2u64)
            } else {
                let a6363 = v0;
                let a6364 = 3u64;
                if nat_eq(a6363, a6364) {
                    f285(4u64, 1u64, 2u64, 2u64, v2, 1u64, 1u64)
                } else {
                    let a6365 = v0;
                    let a6366 = 4u64;
                    if nat_eq(a6365, a6366) {
                        f285(12u64, 1u64, 1u64, 2u64, 0u64, 1u64, 1u64)
                    } else {
                        let a6367 = v0;
                        let a6368 = 5u64;
                        if nat_eq(a6367, a6368) {
                            f285(11u64, 2u64, 1065353216u64, 1u64, 1u64, 1u64, 1u64)
                        } else {
                            Adt17::C0
                        }
                    }
                }
            }
        }
    }
}

pub fn f297(v0: u64, v1: Adt21, v2: Adt20) -> R<u64> {
    let a6379 = {
        let a6377 = {
            let a6374 = {
                let a6371 = match v1.clone() {
                    Adt21::C0(_, _, h6369, _) => {
                        h6369
                    }
                };
                let a6372 = match v2.clone() {
                    Adt20::C0(_, _, h6370, _, _, _, _, _) => {
                        h6370
                    }
                };
                nat_mul(a6371, a6372)?
            };
            let a6375 = match v1 {
                Adt21::C0(h6373, _, _, _) => {
                    h6373
                }
            };
            nat_add(a6374, a6375)?
        };
        let a6378 = match v2 {
            Adt20::C0(_, h6376, _, _, _, _, _, _) => {
                h6376
            }
        };
        nat_mul(a6377, a6378)?
    };
    let a6380 = v0;
    nat_add(a6379, a6380)
}

pub fn f298(v0: u64) -> u64 {
    if f196(v0) {
        let m6381 = f187(v0);
        match m6381 {
            Adt18::C0 => {
                0u64
            }
            Adt18::C1(v1) => {
                let a6383 = 1u64;
                let a6384 = match v1.clone() {
                    Adt19::C0(_, h6382, _) => {
                        h6382
                    }
                };
                if if nat_lt(a6383, a6384) {
                    true
                } else {
                    let a6385 = v0;
                    let a6386 = 43u64;
                    nat_eq(a6385, a6386)
                } {
                    match v1 {
                        Adt19::C0(_, h6387, _) => {
                            h6387
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

pub fn f299(v0: u64, v1: u64, v2: u64, v3: Adt20, v4: Adt20) -> R<Adt17> {
    let v5: u64 = {
        let a6395 = {
            let a6393 = {
                let a6389 = v1;
                let a6390 = match v3.clone() {
                    Adt20::C0(h6388, _, _, _, _, _, _, _) => {
                        h6388
                    }
                };
                nat_mul(a6389, a6390)?
            };
            let a6394 = {
                let a6391 = v2;
                let a6392 = 4u64;
                nat_mul(a6391, a6392)?
            };
            nat_add(a6393, a6394)?
        };
        let a6396 = f302(v0);
        nat_add(a6395, a6396)?
    };
    let v6: u64 = f284(v5, v3)?;
    let v7: u64 = f284(v5, v4)?;
    let a6397 = v0;
    let a6398 = 0u64;
    if if if if nat_eq(a6397, a6398) {
        true
    } else {
        let a6399 = v0;
        let a6400 = 2u64;
        nat_eq(a6399, a6400)
    } {
        true
    } else {
        let a6401 = v0;
        let a6402 = 5u64;
        nat_eq(a6401, a6402)
    } {
        true
    } else {
        let a6403 = v0;
        let a6404 = 7u64;
        nat_eq(a6403, a6404)
    } {
        Ok(f285(9u64, 0u64, v6, 1u64, 3u64, 3u64, v7))
    } else {
        let a6405 = v0;
        let a6406 = 1u64;
        if if if if nat_eq(a6405, a6406) {
            true
        } else {
            let a6407 = v0;
            let a6408 = 3u64;
            nat_eq(a6407, a6408)
        } {
            true
        } else {
            let a6409 = v0;
            let a6410 = 6u64;
            nat_eq(a6409, a6410)
        } {
            true
        } else {
            let a6411 = v0;
            let a6412 = 8u64;
            nat_eq(a6411, a6412)
        } {
            Ok(f285(10u64, 3u64, v7, 3u64, v7, 1u64, {
                let a6413 = v0;
                let a6414 = 1u64;
                if nat_eq(a6413, a6414) {
                    4u64
                } else {
                    let a6415 = v0;
                    let a6416 = 6u64;
                    if nat_eq(a6415, a6416) {
                        5u64
                    } else {
                        1u64
                    }
                }
            }))
        } else {
            let a6417 = v0;
            let a6418 = 4u64;
            if nat_eq(a6417, a6418) {
                Ok(f285(4u64, 1u64, 4u64, 1u64, 1u64, 1u64, 4u64))
            } else {
                let a6419 = v0;
                let a6420 = 9u64;
                if nat_eq(a6419, a6420) {
                    Ok(f285(4u64, 1u64, 5u64, 1u64, 1u64, 1u64, 5u64))
                } else {
                    let a6421 = v0;
                    let a6422 = 10u64;
                    if nat_eq(a6421, a6422) {
                        Ok(f285(4u64, 1u64, 4u64, 1u64, 5u64, 1u64, 1u64))
                    } else {
                        let a6423 = v0;
                        let a6424 = 11u64;
                        if nat_eq(a6423, a6424) {
                            Ok(f285(1u64, 1u64, 1u64, 2u64, 0u64, 1u64, 1u64))
                        } else {
                            let a6425 = v0;
                            let a6426 = 12u64;
                            if nat_eq(a6425, a6426) {
                                Ok(f285(0u64, 1u64, 0u64, 1u64, 1u64, 1u64, 0u64))
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

pub fn f300(v0: u64, v1: u64, v2: Adt20, v3: Adt20, v4: bool) -> R<Adt17> {
    let v5: u64 = f284(v1, v2)?;
    let v6: u64 = f284(v1, v3)?;
    let a6427 = v0;
    let a6428 = 0u64;
    if nat_eq(a6427, a6428) {
        Ok(f285(9u64, 0u64, v5, 1u64, 3u64, 3u64, v6))
    } else {
        let a6429 = v0;
        let a6430 = 1u64;
        if nat_eq(a6429, a6430) {
            Ok(f285(10u64, 3u64, v6, 3u64, v6, 1u64, 1u64))
        } else {
            if v4 {
                let a6431 = v0;
                let a6432 = 2u64;
                if nat_eq(a6431, a6432) {
                    Ok(f285(1u64, 1u64, 1u64, 2u64, 0u64, 1u64, 1u64))
                } else {
                    let a6433 = v0;
                    let a6434 = 3u64;
                    if nat_eq(a6433, a6434) {
                        Ok(f285(0u64, 1u64, 0u64, 1u64, 1u64, 1u64, 0u64))
                    } else {
                        Ok(Adt17::C0)
                    }
                }
            } else {
                let a6435 = v0;
                let a6436 = 2u64;
                if nat_eq(a6435, a6436) {
                    Ok(f285(4u64, 1u64, 0u64, 1u64, 1u64, 1u64, 0u64))
                } else {
                    Ok(Adt17::C0)
                }
            }
        }
    }
}

pub fn f301(v0: u64) -> R<u64> {
    let a6437 = 4u64;
    let a6438 = f277(v0);
    nat_mul(a6437, a6438)
}

pub fn f302(v0: u64) -> u64 {
    let a6439 = v0;
    let a6440 = 5u64;
    if nat_lt(a6439, a6440) {
        let a6441 = v0;
        let a6442 = 2u64;
        if nat_lt(a6441, a6442) {
            0u64
        } else {
            1u64
        }
    } else {
        let a6443 = v0;
        let a6444 = 7u64;
        if nat_lt(a6443, a6444) {
            2u64
        } else {
            3u64
        }
    }
}
