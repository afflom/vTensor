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
    C0(u64, Adt1, Option<Adt2>, Option<Adt2>, Option<Adt2>, Option<Adt2>, Option<Adt2>, Option<Adt2>, Option<Adt2>, bool, u64, bool, Adt2),
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
    C47(bool, u64),
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
        Adt0::C0(v1, v2, v3, v4, v5, v6, v7, v8, v9, v10, v11, v12, v13) => {
            let a2 = v1;
            let a3 = 79u64;
            if if nat_eq(a2, a3) {
                true
            } else {
                let a4 = v1;
                let a5 = 81u64;
                nat_eq(a4, a5)
            } {
                Ok(Adt9::C0(f1({
                    let a6 = v1;
                    let a7 = 81u64;
                    nat_eq(a6, a7)
                }, v3, v4, v5, v6, v7, v8, v9, v10, v13)?))
            } else {
                let v14: bool = if if f2(v8) {
                    true
                } else {
                    f2(v9)
                } {
                    true
                } else {
                    v10
                };
                let a8 = v1;
                let a9 = 80u64;
                if nat_eq(a8, a9) {
                    Ok(Adt9::C0(f3(match v2 {
                        Adt1::C0(h10, _, _, _, _, _, _, _, _, _, _, _) => {
                            h10
                        }
                    }, v3, v4, v5, v6, v7, v14, v13)?))
                } else {
                    let a11 = v1;
                    let a12 = 82u64;
                    if if if if if nat_eq(a11, a12) {
                        true
                    } else {
                        let a13 = v1;
                        let a14 = 50u64;
                        nat_eq(a13, a14)
                    } {
                        true
                    } else {
                        let a15 = v1;
                        let a16 = 59u64;
                        nat_eq(a15, a16)
                    } {
                        true
                    } else {
                        let a17 = v1;
                        let a18 = 65u64;
                        nat_eq(a17, a18)
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
                            v14
                        }
                    } else {
                        false
                    } {
                        Ok(Adt9::C0(Adt10::C1))
                    } else {
                        let a19 = v1;
                        let a20 = 94u64;
                        if if nat_eq(a19, a20) {
                            true
                        } else {
                            let a21 = v1;
                            let a22 = 95u64;
                            nat_eq(a21, a22)
                        } {
                            Ok(Adt9::C0(f4({
                                let a23 = v1;
                                let a24 = 95u64;
                                nat_eq(a23, a24)
                            }, v3, v4, v5, {
                                let m25 = v6;
                                match m25 {
                                    None => {
                                        let m26 = v7;
                                        match m26 {
                                            None => {
                                                v14
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
                            }, v13)?))
                        } else {
                            let a27 = v1;
                            let a28 = 96u64;
                            if if nat_eq(a27, a28) {
                                true
                            } else {
                                let a29 = v1;
                                let a30 = 97u64;
                                nat_eq(a29, a30)
                            } {
                                Ok(Adt9::C0(f5({
                                    let a31 = v1;
                                    let a32 = 96u64;
                                    nat_eq(a31, a32)
                                }, v11, v12, v3, v4, v5, v6, v7, v14, v13)?))
                            } else {
                                let v18: bool = {
                                    let m33 = v7;
                                    match m33 {
                                        None => {
                                            v14
                                        }
                                        Some(_) => {
                                            true
                                        }
                                    }
                                };
                                let a34 = v1;
                                let a35 = 85u64;
                                if nat_eq(a34, a35) {
                                    Ok(Adt9::C0(f6(v2, v3, v4, v5, {
                                        let m36 = v6;
                                        match m36 {
                                            None => {
                                                v18
                                            }
                                            Some(_) => {
                                                true
                                            }
                                        }
                                    }, v13)?))
                                } else {
                                    let a37 = v1;
                                    let a38 = 87u64;
                                    if nat_eq(a37, a38) {
                                        Ok(Adt9::C0(f7(v3, v4, v5, v6, v18, v13)?))
                                    } else {
                                        let a39 = v1;
                                        let a40 = 64u64;
                                        if nat_eq(a39, a40) {
                                            Ok(Adt9::C0(f8(v2, v13)?))
                                        } else {
                                            let m41 = v3;
                                            match m41 {
                                                None => {
                                                    Ok(Adt9::C0(Adt10::C1))
                                                }
                                                Some(v20) => {
                                                    let a42 = v1;
                                                    let a43 = 48u64;
                                                    if if nat_eq(a42, a43) {
                                                        true
                                                    } else {
                                                        let a44 = v1;
                                                        let a45 = 56u64;
                                                        nat_eq(a44, a45)
                                                    } {
                                                        Ok(Adt9::C0(f9({
                                                            let a46 = v1;
                                                            let a47 = 56u64;
                                                            nat_eq(a46, a47)
                                                        }, v2, v20, v4, v13)?))
                                                    } else {
                                                        let a48 = v1;
                                                        let a49 = 53u64;
                                                        if if nat_eq(a48, a49) {
                                                            true
                                                        } else {
                                                            let a50 = v1;
                                                            let a51 = 54u64;
                                                            nat_eq(a50, a51)
                                                        } {
                                                            Ok(Adt9::C0(f10({
                                                                let a52 = v1;
                                                                let a53 = 54u64;
                                                                nat_eq(a52, a53)
                                                            }, v2, v20, v4, v13)?))
                                                        } else {
                                                            let a54 = v1;
                                                            let a55 = 78u64;
                                                            if nat_eq(a54, a55) {
                                                                Ok(Adt9::C0(f11(v2, v20, v4, v5, v13)?))
                                                            } else {
                                                                let a56 = v1;
                                                                let a57 = 23u64;
                                                                if if if nat_eq(a56, a57) {
                                                                    true
                                                                } else {
                                                                    let a58 = v1;
                                                                    let a59 = 26u64;
                                                                    nat_eq(a58, a59)
                                                                } {
                                                                    true
                                                                } else {
                                                                    let a60 = v1;
                                                                    let a61 = 25u64;
                                                                    nat_eq(a60, a61)
                                                                } {
                                                                    Ok(Adt9::C0(f12(v1, v2, v20, v4, v13)?))
                                                                } else {
                                                                    let a62 = v1;
                                                                    let a63 = 24u64;
                                                                    if nat_eq(a62, a63) {
                                                                        Ok(Adt9::C0(f13(v2, v20, v13)?))
                                                                    } else {
                                                                        let a64 = v1;
                                                                        let a65 = 27u64;
                                                                        if nat_eq(a64, a65) {
                                                                            Ok(Adt9::C0(f14(v2, v20, v13)?))
                                                                        } else {
                                                                            let a66 = v1;
                                                                            let a67 = 3u64;
                                                                            if nat_eq(a66, a67) {
                                                                                Ok(Adt9::C0(f15(v20, v4, v5, v13)?))
                                                                            } else {
                                                                                f16(v1, v2, v20, v4, v13)
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        Adt0::C1(v26, v27, v28, v29, v30) => {
            f19(v26, v27, v28, v29, v30)
        }
        Adt0::C2(v41) => {
            f21(v41)
        }
        Adt0::C3(v42, v43, v44, v45, v46, v47, v48, v49, v50, v51, v52, v53, v54) => {
            Ok(f22(v42, v43, v44, v45, v46, v47, v48, v49, v50, v51, v52, v53, v54))
        }
        Adt0::C4(v21, v22, v23, v24) => {
            let a69 = 0u64;
            let a70 = match v21.clone() {
                Adt8::C0(_, _, _, _, _, _, _, h68, _, _, _, _, _, _, _) => {
                    h68
                }
            };
            if nat_lt(a69, a70) {
                f17(v21, v22, v23, v24)
            } else {
                f18(v21, v22, v23, v24)
            }
        }
        Adt0::C5(v25) => {
            Ok(Adt9::C9({
                let a71 = 3240099840u64;
                let a72 = v25;
                if if nat_le(a71, a72) {
                    let a73 = v25;
                    let a74 = 4286578688u64;
                    nat_le(a73, a74)
                } else {
                    false
                } {
                    Some(0u64)
                } else {
                    let a75 = 1092616192u64;
                    let a76 = v25;
                    if if nat_le(a75, a76) {
                        let a77 = v25;
                        let a78 = 2139095040u64;
                        nat_le(a77, a78)
                    } else {
                        false
                    } {
                        Some(v25)
                    } else {
                        None::<u64>
                    }
                }
            }))
        }
        Adt0::C6(v31, v32, v33, v34, v35, v36, v37, v38, v39, v40) => {
            f20(v31, v32, v33, v34, v35, true, v36, v37, v38, v39, v40)
        }
    }
}

pub fn f1(v0: bool, v1: Option<Adt2>, v2: Option<Adt2>, v3: Option<Adt2>, v4: Option<Adt2>, v5: Option<Adt2>, v6: Option<Adt2>, v7: Option<Adt2>, v8: bool, v9: Adt2) -> R<Adt10> {
    if v8 {
        Ok(Adt10::C1)
    } else {
        if v0 {
            let m79 = v1;
            match m79 {
                None => {
                    Ok(Adt10::C1)
                }
                Some(v10) => {
                    let m80 = v2;
                    match m80 {
                        None => {
                            Ok(Adt10::C1)
                        }
                        Some(v11) => {
                            let m81 = v3;
                            match m81 {
                                None => {
                                    Ok(Adt10::C1)
                                }
                                Some(v12) => {
                                    let m82 = v4;
                                    match m82 {
                                        None => {
                                            Ok(Adt10::C1)
                                        }
                                        Some(v13) => {
                                            let m83 = v5;
                                            match m83 {
                                                None => {
                                                    Ok(Adt10::C1)
                                                }
                                                Some(v14) => {
                                                    let m84 = v6;
                                                    match m84 {
                                                        None => {
                                                            Ok(Adt10::C1)
                                                        }
                                                        Some(v15) => {
                                                            let m85 = v7;
                                                            match m85 {
                                                                None => {
                                                                    Ok(Adt10::C1)
                                                                }
                                                                Some(v16) => {
                                                                    let v17: u64 = {
                                                                        let a88 = match v12.clone() {
                                                                            Adt2::C0(_, h86, _, _, _, _, _, _, _, _, _) => {
                                                                                h86
                                                                            }
                                                                        };
                                                                        let a89 = match v12.clone() {
                                                                            Adt2::C0(_, _, h87, _, _, _, _, _, _, _, _) => {
                                                                                h87
                                                                            }
                                                                        };
                                                                        nat_mul(a88, a89)?
                                                                    };
                                                                    let v18: u64 = {
                                                                        let a91 = v17;
                                                                        let a92 = match v12.clone() {
                                                                            Adt2::C0(_, _, _, h90, _, _, _, _, _, _, _) => {
                                                                                h90
                                                                            }
                                                                        };
                                                                        nat_mul(a91, a92)?
                                                                    };
                                                                    if if if if if f23(v10, v12.clone(), v9.clone(), v18)? {
                                                                        f23(v11, v12.clone(), v9.clone(), v18)?
                                                                    } else {
                                                                        false
                                                                    } {
                                                                        f23(v13, v12.clone(), v9.clone(), v18)?
                                                                    } else {
                                                                        false
                                                                    } {
                                                                        f23(v14, v12.clone(), v9.clone(), v18)?
                                                                    } else {
                                                                        false
                                                                    } {
                                                                        f23(v15, v12.clone(), v9.clone(), v18)?
                                                                    } else {
                                                                        false
                                                                    } {
                                                                        f24(2u64, 0u64, v12, v16, v9)
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
                }
            }
        } else {
            let m93 = v7;
            match m93 {
                None => {
                    let m94 = v1;
                    match m94 {
                        None => {
                            Ok(Adt10::C1)
                        }
                        Some(v19) => {
                            let m95 = v2;
                            match m95 {
                                None => {
                                    Ok(Adt10::C1)
                                }
                                Some(v20) => {
                                    let m96 = v3;
                                    match m96 {
                                        None => {
                                            Ok(Adt10::C1)
                                        }
                                        Some(v21) => {
                                            let m97 = v4;
                                            match m97 {
                                                None => {
                                                    Ok(Adt10::C1)
                                                }
                                                Some(v22) => {
                                                    let m98 = v5;
                                                    match m98 {
                                                        None => {
                                                            Ok(Adt10::C1)
                                                        }
                                                        Some(v23) => {
                                                            let m99 = v6;
                                                            match m99 {
                                                                None => {
                                                                    Ok(Adt10::C1)
                                                                }
                                                                Some(v24) => {
                                                                    let v25: u64 = {
                                                                        let a102 = match v19.clone() {
                                                                            Adt2::C0(_, h100, _, _, _, _, _, _, _, _, _) => {
                                                                                h100
                                                                            }
                                                                        };
                                                                        let a103 = match v19.clone() {
                                                                            Adt2::C0(_, _, h101, _, _, _, _, _, _, _, _) => {
                                                                                h101
                                                                            }
                                                                        };
                                                                        nat_mul(a102, a103)?
                                                                    };
                                                                    let v26: u64 = {
                                                                        let a105 = v25;
                                                                        let a106 = match v19.clone() {
                                                                            Adt2::C0(_, _, _, h104, _, _, _, _, _, _, _) => {
                                                                                h104
                                                                            }
                                                                        };
                                                                        nat_mul(a105, a106)?
                                                                    };
                                                                    if if if if f23(v20, v19.clone(), v9.clone(), v26)? {
                                                                        f23(v21, v19.clone(), v9.clone(), v26)?
                                                                    } else {
                                                                        false
                                                                    } {
                                                                        f23(v23, v19.clone(), v9.clone(), v26)?
                                                                    } else {
                                                                        false
                                                                    } {
                                                                        f25(v22, v9.clone(), v25)?
                                                                    } else {
                                                                        false
                                                                    } {
                                                                        f24(0u64, 0u64, v19, v24, v9)
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
                }
                Some(_) => {
                    Ok(Adt10::C1)
                }
            }
        }
    }
}

pub fn f2(v0: Option<Adt2>) -> bool {
    let m107 = v0;
    match m107 {
        None => {
            false
        }
        Some(_) => {
            true
        }
    }
}

pub fn f3(v0: u64, v1: Option<Adt2>, v2: Option<Adt2>, v3: Option<Adt2>, v4: Option<Adt2>, v5: Option<Adt2>, v6: bool, v7: Adt2) -> R<Adt10> {
    if v6 {
        Ok(Adt10::C1)
    } else {
        let m108 = v1;
        match m108 {
            None => {
                Ok(Adt10::C1)
            }
            Some(v8) => {
                let m109 = v2;
                match m109 {
                    None => {
                        Ok(Adt10::C1)
                    }
                    Some(v9) => {
                        let m110 = v3;
                        match m110 {
                            None => {
                                Ok(Adt10::C1)
                            }
                            Some(v10) => {
                                let m111 = v4;
                                match m111 {
                                    None => {
                                        Ok(Adt10::C1)
                                    }
                                    Some(v11) => {
                                        let m112 = v5;
                                        match m112 {
                                            None => {
                                                Ok(Adt10::C1)
                                            }
                                            Some(v12) => {
                                                let v13: u64 = {
                                                    let a115 = match v8.clone() {
                                                        Adt2::C0(_, h113, _, _, _, _, _, _, _, _, _) => {
                                                            h113
                                                        }
                                                    };
                                                    let a116 = match v8.clone() {
                                                        Adt2::C0(_, _, h114, _, _, _, _, _, _, _, _) => {
                                                            h114
                                                        }
                                                    };
                                                    nat_mul(a115, a116)?
                                                };
                                                let v14: u64 = {
                                                    let a118 = v13;
                                                    let a119 = match v8.clone() {
                                                        Adt2::C0(_, _, _, h117, _, _, _, _, _, _, _) => {
                                                            h117
                                                        }
                                                    };
                                                    nat_mul(a118, a119)?
                                                };
                                                if if if f23(v9, v8.clone(), v7.clone(), v14)? {
                                                    f23(v10, v8.clone(), v7.clone(), v14)?
                                                } else {
                                                    false
                                                } {
                                                    f23(v11, v8.clone(), v7.clone(), v14)?
                                                } else {
                                                    false
                                                } {
                                                    f24(1u64, v0, v8, v12, v7)
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

pub fn f4(v0: bool, v1: Option<Adt2>, v2: Option<Adt2>, v3: Option<Adt2>, v4: bool, v5: Adt2) -> R<Adt10> {
    let v7: Adt2 = {
        let m120 = v1.clone();
        match m120 {
            None => {
                v5.clone()
            }
            Some(v6) => {
                v6
            }
        }
    };
    let v10: Adt2 = if v0 {
        let m121 = v2.clone();
        match m121 {
            None => {
                v5.clone()
            }
            Some(v8) => {
                v8
            }
        }
    } else {
        let m122 = v1.clone();
        match m122 {
            None => {
                v5.clone()
            }
            Some(v9) => {
                v9
            }
        }
    };
    let v13: Adt2 = if v0 {
        let m123 = v3.clone();
        match m123 {
            None => {
                v5.clone()
            }
            Some(v11) => {
                v11
            }
        }
    } else {
        let m124 = v2.clone();
        match m124 {
            None => {
                v5.clone()
            }
            Some(v12) => {
                v12
            }
        }
    };
    let v14: u64 = {
        let a130 = {
            let a127 = match v10.clone() {
                Adt2::C0(_, _, h125, _, _, _, _, _, _, _, _) => {
                    h125
                }
            };
            let a128 = match v10.clone() {
                Adt2::C0(_, _, _, h126, _, _, _, _, _, _, _) => {
                    h126
                }
            };
            nat_mul(a127, a128)?
        };
        let a131 = match v10.clone() {
            Adt2::C0(_, _, _, _, h129, _, _, _, _, _, _) => {
                h129
            }
        };
        nat_mul(a130, a131)?
    };
    let m132 = v1;
    if if if if if if if if if if match m132 {
        None => {
            false
        }
        Some(_) => {
            true
        }
    } {
        let m133 = v2;
        match m133 {
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
            let m134 = v3;
            match m134 {
                None => {
                    false
                }
                Some(_) => {
                    true
                }
            }
        } else {
            let m135 = v3;
            !match m135 {
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
        let a136 = 0u64;
        let a137 = v14;
        nat_lt(a136, a137)
    } else {
        false
    } {
        let a139 = 0u64;
        let a140 = match v10.clone() {
            Adt2::C0(_, h138, _, _, _, _, _, _, _, _, _) => {
                h138
            }
        };
        nat_lt(a139, a140)
    } else {
        false
    } {
        f26(v10.clone(), v13.clone())
    } else {
        false
    } {
        f27(v10.clone(), v14, v0)?
    } else {
        false
    } {
        f27(v13.clone(), v14, v0)?
    } else {
        false
    } {
        if v0 {
            let a142 = match v7.clone() {
                Adt2::C0(h141, _, _, _, _, _, _, _, _, _, _) => {
                    h141
                }
            };
            let a143 = 0u64;
            if if if if nat_eq(a142, a143) {
                let a144 = f28(v7.clone(), 1u64)?;
                let a145 = 1u64;
                nat_eq(a144, a145)
            } else {
                false
            } {
                let a147 = 4u64;
                let a148 = match v7 {
                    Adt2::C0(_, _, _, _, _, _, _, _, _, h146, _) => {
                        h146
                    }
                };
                nat_le(a147, a148)
            } else {
                false
            } {
                f26(v10.clone(), v5.clone())
            } else {
                false
            } {
                f27(v5.clone(), v14, true)?
            } else {
                false
            }
        } else {
            let a150 = match v5.clone() {
                Adt2::C0(h149, _, _, _, _, _, _, _, _, _, _) => {
                    h149
                }
            };
            let a151 = 0u64;
            if if nat_eq(a150, a151) {
                let a152 = f28(v5.clone(), 1u64)?;
                let a153 = 1u64;
                nat_eq(a152, a153)
            } else {
                false
            } {
                let a155 = 4u64;
                let a156 = match v5.clone() {
                    Adt2::C0(_, _, _, _, _, _, _, _, _, h154, _) => {
                        h154
                    }
                };
                nat_le(a155, a156)
            } else {
                false
            }
        }
    } else {
        false
    } {
        Ok(Adt10::C4({
            let a162 = {
                let a160 = 2u64;
                let a161 = {
                    let a158 = v14;
                    let a159 = f29(v0, match v10.clone() {
                        Adt2::C0(_, h157, _, _, _, _, _, _, _, _, _) => {
                            h157
                        }
                    })?;
                    nat_mul(a158, a159)?
                };
                nat_add(a160, a161)?
            };
            let a163 = if v0 {
                0u64
            } else {
                1u64
            };
            nat_add(a162, a163)?
        }, 4u64, f30(v10, 1u64), f30(v5, 1u64), Adt4::C40(v0, match v13 {
            Adt2::C0(_, _, _, _, _, _, h164, _, _, _, _) => {
                h164
            }
        }, v14)))
    } else {
        Ok(Adt10::C1)
    }
}

pub fn f5(v0: bool, v1: u64, v2: bool, v3: Option<Adt2>, v4: Option<Adt2>, v5: Option<Adt2>, v6: Option<Adt2>, v7: Option<Adt2>, v8: bool, v9: Adt2) -> R<Adt10> {
    let v11: Adt2 = {
        let m165 = v3.clone();
        match m165 {
            None => {
                v9.clone()
            }
            Some(v10) => {
                v10
            }
        }
    };
    let v13: Adt2 = {
        let m166 = v4.clone();
        match m166 {
            None => {
                v9.clone()
            }
            Some(v12) => {
                v12
            }
        }
    };
    let v15: Adt2 = {
        let m167 = v5.clone();
        match m167 {
            None => {
                v9.clone()
            }
            Some(v14) => {
                v14
            }
        }
    };
    let v17: Adt2 = {
        let m168 = v6.clone();
        match m168 {
            None => {
                v9.clone()
            }
            Some(v16) => {
                v16
            }
        }
    };
    let v20: Adt2 = if v0 {
        let m169 = v7.clone();
        match m169 {
            None => {
                v9.clone()
            }
            Some(v18) => {
                v18
            }
        }
    } else {
        let m170 = v5.clone();
        match m170 {
            None => {
                v9.clone()
            }
            Some(v19) => {
                v19
            }
        }
    };
    let m171 = v3;
    if if if if if if if if if if if match m171 {
        None => {
            false
        }
        Some(_) => {
            true
        }
    } {
        let m172 = v4;
        match m172 {
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
        let m173 = v5;
        match m173 {
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
            let m174 = v6;
            if match m174 {
                None => {
                    false
                }
                Some(_) => {
                    true
                }
            } {
                let m175 = v7;
                match m175 {
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
            let m176 = v6;
            if !match m176 {
                None => {
                    false
                }
                Some(_) => {
                    true
                }
            } {
                let m177 = v7;
                !match m177 {
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
            let a184 = {
                let a181 = {
                    let a178 = v1;
                    let a179 = 4u64;
                    let a180 = 0u64;
                    nat_quot(a178, a179, a180)
                };
                let a182 = 2u64;
                let a183 = 0u64;
                nat_rem(a181, a182, a183)
            };
            let a185 = 1u64;
            nat_eq(a184, a185)
        } else {
            false
        } {
            let a187 = match v11.clone() {
                Adt2::C0(h186, _, _, _, _, _, _, _, _, _, _) => {
                    h186
                }
            };
            let a188 = 0u64;
            nat_eq(a187, a188)
        } else {
            false
        } {
            let a190 = match v11.clone() {
                Adt2::C0(_, _, _, _, _, h189, _, _, _, _, _) => {
                    h189
                }
            };
            let a191 = 4u64;
            nat_eq(a190, a191)
        } else {
            false
        } {
            f31(v11.clone(), f28(v11.clone(), 1u64)?)?
        } else {
            false
        } {
            f32(v11.clone(), v9.clone())?
        } else {
            false
        } {
            let a193 = match v11.clone() {
                Adt2::C0(_, h192, _, _, _, _, _, _, _, _, _) => {
                    h192
                }
            };
            let a194 = 1u64;
            if nat_le(a193, a194) {
                true
            } else {
                let a197 = match v11.clone() {
                    Adt2::C0(_, _, _, _, _, h195, _, _, _, _, _) => {
                        h195
                    }
                };
                let a198 = match v9.clone() {
                    Adt2::C0(_, _, _, _, _, h196, _, _, _, _, _) => {
                        h196
                    }
                };
                nat_eq(a197, a198)
            }
        } else {
            false
        } {
            let a200 = match v11.clone() {
                Adt2::C0(_, _, h199, _, _, _, _, _, _, _, _) => {
                    h199
                }
            };
            let a201 = 1u64;
            if nat_le(a200, a201) {
                true
            } else {
                let a204 = match v11.clone() {
                    Adt2::C0(_, _, _, _, _, _, h202, _, _, _, _) => {
                        h202
                    }
                };
                let a205 = match v9.clone() {
                    Adt2::C0(_, _, _, _, _, _, h203, _, _, _, _) => {
                        h203
                    }
                };
                nat_eq(a204, a205)
            }
        } else {
            false
        } {
            let a207 = match v11.clone() {
                Adt2::C0(_, _, _, h206, _, _, _, _, _, _, _) => {
                    h206
                }
            };
            let a208 = 1u64;
            if nat_le(a207, a208) {
                true
            } else {
                let a211 = match v11.clone() {
                    Adt2::C0(_, _, _, _, _, _, _, h209, _, _, _) => {
                        h209
                    }
                };
                let a212 = match v9.clone() {
                    Adt2::C0(_, _, _, _, _, _, _, h210, _, _, _) => {
                        h210
                    }
                };
                nat_eq(a211, a212)
            }
        } else {
            false
        } {
            let a214 = match v11.clone() {
                Adt2::C0(_, _, _, _, h213, _, _, _, _, _, _) => {
                    h213
                }
            };
            let a215 = 1u64;
            if nat_le(a214, a215) {
                true
            } else {
                let a218 = match v11.clone() {
                    Adt2::C0(_, _, _, _, _, _, _, _, h216, _, _) => {
                        h216
                    }
                };
                let a219 = match v9.clone() {
                    Adt2::C0(_, _, _, _, _, _, _, _, h217, _, _) => {
                        h217
                    }
                };
                nat_eq(a218, a219)
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
        let a221 = match v20.clone() {
            Adt2::C0(h220, _, _, _, _, _, _, _, _, _, _) => {
                h220
            }
        };
        let a222 = 0u64;
        nat_eq(a221, a222)
    } else {
        false
    } {
        let a223 = f28(v20.clone(), 1u64)?;
        let a224 = if v0 {
            7u64
        } else {
            2u64
        };
        nat_eq(a223, a224)
    } else {
        false
    } {
        let a226 = if v0 {
            28u64
        } else {
            8u64
        };
        let a227 = match v20 {
            Adt2::C0(_, _, _, _, _, _, _, _, _, h225, _) => {
                h225
            }
        };
        nat_le(a226, a227)
    } else {
        false
    } {
        Ok(Adt10::C4({
            let a230 = f33(v0);
            let a231 = {
                let a228 = f28(v11.clone(), 1u64)?;
                let a229 = f34(v0);
                nat_mul(a228, a229)?
            };
            nat_add(a230, a231)?
        }, 4u64, f30(v11, 1u64), f30(v9, 1u64), Adt4::C39(v0)))
    } else {
        Ok(Adt10::C1)
    }
}

pub fn f6(v0: Adt1, v1: Option<Adt2>, v2: Option<Adt2>, v3: Option<Adt2>, v4: bool, v5: Adt2) -> R<Adt10> {
    let v7: Adt2 = {
        let m232 = v1.clone();
        match m232 {
            None => {
                v5.clone()
            }
            Some(v6) => {
                v6
            }
        }
    };
    let v9: Adt2 = {
        let m233 = v2.clone();
        match m233 {
            None => {
                v5.clone()
            }
            Some(v8) => {
                v8
            }
        }
    };
    let v11: Adt2 = {
        let m234 = v3.clone();
        match m234 {
            None => {
                v5.clone()
            }
            Some(v10) => {
                v10
            }
        }
    };
    let v12: u64 = match v0.clone() {
        Adt1::C0(_, h235, _, _, _, _, _, _, _, _, _, _) => {
            h235
        }
    };
    let m236 = v1;
    if if if if if if if if if if if if if if if if if if if if if if if if if match m236 {
        None => {
            false
        }
        Some(_) => {
            true
        }
    } {
        let m237 = v2;
        match m237 {
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
        let m238 = v3;
        match m238 {
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
        let a240 = match v7.clone() {
            Adt2::C0(h239, _, _, _, _, _, _, _, _, _, _) => {
                h239
            }
        };
        let a241 = 0u64;
        if if nat_eq(a240, a241) {
            f31(v7.clone(), f28(v7.clone(), 1u64)?)?
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
        let a243 = match v9.clone() {
            Adt2::C0(h242, _, _, _, _, _, _, _, _, _, _) => {
                h242
            }
        };
        let a244 = 0u64;
        if if nat_eq(a243, a244) {
            f31(v9.clone(), f28(v9.clone(), 1u64)?)?
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
        let a246 = match v11.clone() {
            Adt2::C0(h245, _, _, _, _, _, _, _, _, _, _) => {
                h245
            }
        };
        let a247 = 0u64;
        if if nat_eq(a246, a247) {
            f31(v11.clone(), f28(v11.clone(), 1u64)?)?
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
        let a249 = match v7.clone() {
            Adt2::C0(_, h248, _, _, _, _, _, _, _, _, _) => {
                h248
            }
        };
        let a250 = 24u64;
        nat_eq(a249, a250)
    } else {
        false
    } {
        let a252 = match v7.clone() {
            Adt2::C0(_, _, _, h251, _, _, _, _, _, _, _) => {
                h251
            }
        };
        let a253 = 1u64;
        nat_eq(a252, a253)
    } else {
        false
    } {
        let a255 = match v7.clone() {
            Adt2::C0(_, _, _, _, h254, _, _, _, _, _, _) => {
                h254
            }
        };
        let a256 = 1u64;
        nat_eq(a255, a256)
    } else {
        false
    } {
        let a258 = match v11.clone() {
            Adt2::C0(_, h257, _, _, _, _, _, _, _, _, _) => {
                h257
            }
        };
        let a259 = 24u64;
        nat_eq(a258, a259)
    } else {
        false
    } {
        let a261 = match v11.clone() {
            Adt2::C0(_, _, h260, _, _, _, _, _, _, _, _) => {
                h260
            }
        };
        let a262 = 1u64;
        nat_eq(a261, a262)
    } else {
        false
    } {
        let a264 = match v11.clone() {
            Adt2::C0(_, _, _, h263, _, _, _, _, _, _, _) => {
                h263
            }
        };
        let a265 = 1u64;
        nat_eq(a264, a265)
    } else {
        false
    } {
        let a267 = match v11.clone() {
            Adt2::C0(_, _, _, _, h266, _, _, _, _, _, _) => {
                h266
            }
        };
        let a268 = 1u64;
        nat_eq(a267, a268)
    } else {
        false
    } {
        let a270 = match v9.clone() {
            Adt2::C0(_, _, h269, _, _, _, _, _, _, _, _) => {
                h269
            }
        };
        let a271 = 1u64;
        nat_eq(a270, a271)
    } else {
        false
    } {
        let a273 = match v9.clone() {
            Adt2::C0(_, _, _, h272, _, _, _, _, _, _, _) => {
                h272
            }
        };
        let a274 = 1u64;
        nat_eq(a273, a274)
    } else {
        false
    } {
        let a276 = match v9.clone() {
            Adt2::C0(_, _, _, _, h275, _, _, _, _, _, _) => {
                h275
            }
        };
        let a277 = 1u64;
        nat_eq(a276, a277)
    } else {
        false
    } {
        let a279 = 3u64;
        let a280 = match v9.clone() {
            Adt2::C0(_, h278, _, _, _, _, _, _, _, _, _) => {
                h278
            }
        };
        nat_le(a279, a280)
    } else {
        false
    } {
        let a282 = match v5.clone() {
            Adt2::C0(_, h281, _, _, _, _, _, _, _, _, _) => {
                h281
            }
        };
        let a283 = 4u64;
        nat_eq(a282, a283)
    } else {
        false
    } {
        let a285 = match v5.clone() {
            Adt2::C0(_, _, h284, _, _, _, _, _, _, _, _) => {
                h284
            }
        };
        let a286 = 4u64;
        nat_eq(a285, a286)
    } else {
        false
    } {
        let a289 = match v5.clone() {
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
        let a292 = match v5.clone() {
            Adt2::C0(_, _, _, _, h291, _, _, _, _, _, _) => {
                h291
            }
        };
        let a293 = 1u64;
        nat_eq(a292, a293)
    } else {
        false
    } {
        f36(v5.clone())?
    } else {
        false
    } {
        let a294 = 0u64;
        let a295 = v12;
        nat_lt(a294, a295)
    } else {
        false
    } {
        let a296 = v12;
        let a297 = 2147483648u64;
        nat_lt(a296, a297)
    } else {
        false
    } {
        Ok(Adt10::C4({
            let a299 = match v7.clone() {
                Adt2::C0(_, _, h298, _, _, _, _, _, _, _, _) => {
                    h298
                }
            };
            let a300 = f37(v12)?;
            nat_mul(a299, a300)?
        }, 4u64, f30(v7, 1u64), f30(v5, 1u64), Adt4::C38(f30(v9, 1u64), f30(v11, 1u64), match v0 {
            Adt1::C0(h301, _, _, _, _, _, _, _, _, _, _, _) => {
                h301
            }
        }, v12)))
    } else {
        Ok(Adt10::C1)
    }
}

pub fn f7(v0: Option<Adt2>, v1: Option<Adt2>, v2: Option<Adt2>, v3: Option<Adt2>, v4: bool, v5: Adt2) -> R<Adt10> {
    let v7: Adt2 = {
        let m302 = v0.clone();
        match m302 {
            None => {
                v5.clone()
            }
            Some(v6) => {
                v6
            }
        }
    };
    let v9: Adt2 = {
        let m303 = v1.clone();
        match m303 {
            None => {
                v5.clone()
            }
            Some(v8) => {
                v8
            }
        }
    };
    let v11: Adt2 = {
        let m304 = v2.clone();
        match m304 {
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
        let m305 = v0;
        match m305 {
            None => {
                false
            }
            Some(_) => {
                let m306 = v1;
                match m306 {
                    None => {
                        false
                    }
                    Some(_) => {
                        let m307 = v2;
                        match m307 {
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
        let a309 = match v7.clone() {
            Adt2::C0(h308, _, _, _, _, _, _, _, _, _, _) => {
                h308
            }
        };
        let a310 = 0u64;
        if if if if if if if if if if if if if if nat_eq(a309, a310) {
            let a312 = match v7.clone() {
                Adt2::C0(_, _, _, _, h311, _, _, _, _, _, _) => {
                    h311
                }
            };
            let a313 = 1u64;
            nat_eq(a312, a313)
        } else {
            false
        } {
            f31(v7.clone(), f28(v7.clone(), 1u64)?)?
        } else {
            false
        } {
            f35(v7.clone(), v5.clone())?
        } else {
            false
        } {
            let a315 = match v9.clone() {
                Adt2::C0(h314, _, _, _, _, _, _, _, _, _, _) => {
                    h314
                }
            };
            let a316 = 0u64;
            if if if nat_eq(a315, a316) {
                let a318 = match v9.clone() {
                    Adt2::C0(_, _, _, _, h317, _, _, _, _, _, _) => {
                        h317
                    }
                };
                let a319 = 1u64;
                nat_eq(a318, a319)
            } else {
                false
            } {
                f31(v9.clone(), f28(v9.clone(), 1u64)?)?
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
            let a321 = match v11.clone() {
                Adt2::C0(h320, _, _, _, _, _, _, _, _, _, _) => {
                    h320
                }
            };
            let a322 = 0u64;
            if if if nat_eq(a321, a322) {
                let a324 = match v11.clone() {
                    Adt2::C0(_, _, _, _, h323, _, _, _, _, _, _) => {
                        h323
                    }
                };
                let a325 = 1u64;
                nat_eq(a324, a325)
            } else {
                false
            } {
                f31(v11.clone(), f28(v11.clone(), 1u64)?)?
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
            let a327 = match v7.clone() {
                Adt2::C0(_, _, _, h326, _, _, _, _, _, _, _) => {
                    h326
                }
            };
            let a328 = 1u64;
            nat_eq(a327, a328)
        } else {
            false
        } {
            let a330 = match v11.clone() {
                Adt2::C0(_, _, _, h329, _, _, _, _, _, _, _) => {
                    h329
                }
            };
            let a331 = 1u64;
            nat_eq(a330, a331)
        } else {
            false
        } {
            let a334 = match v9.clone() {
                Adt2::C0(_, h332, _, _, _, _, _, _, _, _, _) => {
                    h332
                }
            };
            let a335 = match v7.clone() {
                Adt2::C0(_, h333, _, _, _, _, _, _, _, _, _) => {
                    h333
                }
            };
            nat_eq(a334, a335)
        } else {
            false
        } {
            let a338 = match v9.clone() {
                Adt2::C0(_, _, _, h336, _, _, _, _, _, _, _) => {
                    h336
                }
            };
            let a339 = match v7.clone() {
                Adt2::C0(_, _, h337, _, _, _, _, _, _, _, _) => {
                    h337
                }
            };
            nat_eq(a338, a339)
        } else {
            false
        } {
            let a342 = match v11.clone() {
                Adt2::C0(_, h340, _, _, _, _, _, _, _, _, _) => {
                    h340
                }
            };
            let a343 = match v9.clone() {
                Adt2::C0(_, _, h341, _, _, _, _, _, _, _, _) => {
                    h341
                }
            };
            nat_eq(a342, a343)
        } else {
            false
        } {
            let a346 = match v11.clone() {
                Adt2::C0(_, _, h344, _, _, _, _, _, _, _, _) => {
                    h344
                }
            };
            let a347 = match v7.clone() {
                Adt2::C0(_, _, h345, _, _, _, _, _, _, _, _) => {
                    h345
                }
            };
            nat_eq(a346, a347)
        } else {
            false
        } {
            let a349 = 0u64;
            let a350 = match v9.clone() {
                Adt2::C0(_, _, h348, _, _, _, _, _, _, _, _) => {
                    h348
                }
            };
            if nat_lt(a349, a350) {
                f26(v9.clone(), v5.clone())
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
            let m351 = v3.clone();
            match m351 {
                None => {
                    true
                }
                Some(v15) => {
                    let a353 = match v15.clone() {
                        Adt2::C0(h352, _, _, _, _, _, _, _, _, _, _) => {
                            h352
                        }
                    };
                    let a354 = 0u64;
                    if if if if if if nat_eq(a353, a354) {
                        let a356 = match v15.clone() {
                            Adt2::C0(_, _, _, _, h355, _, _, _, _, _, _) => {
                                h355
                            }
                        };
                        let a357 = 1u64;
                        nat_eq(a356, a357)
                    } else {
                        false
                    } {
                        f31(v15.clone(), f28(v15.clone(), 1u64)?)?
                    } else {
                        false
                    } {
                        f35(v15.clone(), v5.clone())?
                    } else {
                        false
                    } {
                        let a360 = match v15.clone() {
                            Adt2::C0(_, h358, _, _, _, _, _, _, _, _, _) => {
                                h358
                            }
                        };
                        let a361 = match v9.clone() {
                            Adt2::C0(_, _, h359, _, _, _, _, _, _, _, _) => {
                                h359
                            }
                        };
                        nat_eq(a360, a361)
                    } else {
                        false
                    } {
                        let a364 = match v15.clone() {
                            Adt2::C0(_, _, h362, _, _, _, _, _, _, _, _) => {
                                h362
                            }
                        };
                        let a365 = match v9.clone() {
                            Adt2::C0(_, _, h363, _, _, _, _, _, _, _, _) => {
                                h363
                            }
                        };
                        nat_eq(a364, a365)
                    } else {
                        false
                    } {
                        let a368 = match v15 {
                            Adt2::C0(_, _, _, h366, _, _, _, _, _, _, _) => {
                                h366
                            }
                        };
                        let a369 = match v7.clone() {
                            Adt2::C0(_, _, h367, _, _, _, _, _, _, _, _) => {
                                h367
                            }
                        };
                        nat_eq(a368, a369)
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
            let m370 = v3;
            match m370 {
                None => {
                    None::<Adt3>
                }
                Some(v16) => {
                    Some(f30(v16, 1u64))
                }
            }
        };
        Ok(Adt10::C4({
            let a372 = f28(v5.clone(), 1u64)?;
            let a373 = f38(match v9.clone() {
                Adt2::C0(_, _, h371, _, _, _, _, _, _, _, _) => {
                    h371
                }
            }, v17.clone())?;
            nat_mul(a372, a373)?
        }, 4u64, f30(v7, 1u64), f30(v5, 1u64), Adt4::C37(f30(v9, 1u64), f30(v11, 1u64), v17)))
    } else {
        Ok(Adt10::C1)
    }
}

pub fn f8(v0: Adt1, v1: Adt2) -> R<Adt10> {
    let v2: u64 = f28(v1.clone(), 1u64)?;
    let a375 = match v1.clone() {
        Adt2::C0(h374, _, _, _, _, _, _, _, _, _, _) => {
            h374
        }
    };
    let a376 = 0u64;
    if if if if if if if if nat_eq(a375, a376) {
        let a378 = match v1.clone() {
            Adt2::C0(_, _, _, _, _, h377, _, _, _, _, _) => {
                h377
            }
        };
        let a379 = 4u64;
        nat_eq(a378, a379)
    } else {
        false
    } {
        f39(match v0.clone() {
            Adt1::C0(h380, _, _, _, _, _, _, _, _, _, _, _) => {
                h380
            }
        })
    } else {
        false
    } {
        f39(match v0.clone() {
            Adt1::C0(_, h381, _, _, _, _, _, _, _, _, _, _) => {
                h381
            }
        })
    } else {
        false
    } {
        let a383 = 0u64;
        let a384 = match v0.clone() {
            Adt1::C0(_, _, h382, _, _, _, _, _, _, _, _, _) => {
                h382
            }
        };
        nat_lt(a383, a384)
    } else {
        false
    } {
        let a386 = match v0.clone() {
            Adt1::C0(_, _, h385, _, _, _, _, _, _, _, _, _) => {
                h385
            }
        };
        let a387 = 2139095040u64;
        nat_lt(a386, a387)
    } else {
        false
    } {
        let a388 = v2;
        let a389 = 9223372036854775808u64;
        nat_lt(a388, a389)
    } else {
        false
    } {
        let a391 = f40(v2)?;
        let a392 = match v1.clone() {
            Adt2::C0(_, _, _, _, _, _, _, _, _, h390, _) => {
                h390
            }
        };
        nat_le(a391, a392)
    } else {
        false
    } {
        Ok(Adt10::C4(f41(v2)?, 4u64, f30(v1.clone(), 1u64), f30(v1, 1u64), Adt4::C23(match v0.clone() {
            Adt1::C0(h393, _, _, _, _, _, _, _, _, _, _, _) => {
                h393
            }
        }, match v0.clone() {
            Adt1::C0(_, h394, _, _, _, _, _, _, _, _, _, _) => {
                h394
            }
        }, match v0 {
            Adt1::C0(_, _, h395, _, _, _, _, _, _, _, _, _) => {
                h395
            }
        }, v2)))
    } else {
        Ok(Adt10::C1)
    }
}

pub fn f9(v0: bool, v1: Adt1, v2: Adt2, v3: Option<Adt2>, v4: Adt2) -> R<Adt10> {
    let m396 = v3;
    match m396 {
        None => {
            Ok(Adt10::C1)
        }
        Some(v5) => {
            let v6: Adt3 = f42(v0, v2.clone());
            let v7: Adt3 = f42(v0, v5.clone());
            let v8: Adt3 = f42(v0, v4.clone());
            let v9: u64 = match v7.clone() {
                Adt3::C0(h397, _, _, _, _, _, _, _) => {
                    h397
                }
            };
            let v10: u64 = match v7.clone() {
                Adt3::C0(_, h398, _, _, _, _, _, _) => {
                    h398
                }
            };
            let v11: u64 = match v7.clone() {
                Adt3::C0(_, _, h399, _, _, _, _, _) => {
                    h399
                }
            };
            let v12: u64 = match v7.clone() {
                Adt3::C0(_, _, _, h400, _, _, _, _) => {
                    h400
                }
            };
            let v13: u64 = match v6.clone() {
                Adt3::C0(h401, _, _, _, _, _, _, _) => {
                    h401
                }
            };
            let v14: u64 = match v6.clone() {
                Adt3::C0(_, h402, _, _, _, _, _, _) => {
                    h402
                }
            };
            let v15: u64 = match v6.clone() {
                Adt3::C0(_, _, h403, _, _, _, _, _) => {
                    h403
                }
            };
            let v16: u64 = match v1.clone() {
                Adt1::C0(h404, _, _, _, _, _, _, _, _, _, _, _) => {
                    h404
                }
            };
            let v17: u64 = {
                let a409 = v13;
                let a410 = {
                    let a407 = {
                        let a405 = v9;
                        let a406 = 1u64;
                        nat_sub(a405, a406)
                    };
                    let a408 = v16;
                    nat_mul(a407, a408)?
                };
                nat_add(a409, a410)?
            };
            let v18: u64 = {
                let a415 = v14;
                let a416 = {
                    let a413 = {
                        let a411 = v10;
                        let a412 = 1u64;
                        nat_sub(a411, a412)
                    };
                    let a414 = v16;
                    nat_mul(a413, a414)?
                };
                nat_add(a415, a416)?
            };
            let v19: u64 = {
                let a419 = {
                    let a417 = v9;
                    let a418 = v10;
                    nat_mul(a417, a418)?
                };
                let a420 = v12;
                nat_mul(a419, a420)?
            };
            let v20: u64 = {
                let a423 = {
                    let a421 = v13;
                    let a422 = v14;
                    nat_mul(a421, a422)?
                };
                let a424 = v15;
                nat_mul(a423, a424)?
            };
            let v21: u64 = {
                let a425 = v19;
                let a426 = 32u64;
                if nat_lt(a425, a426) {
                    v19
                } else {
                    32u64
                }
            };
            let a428 = match v2.clone() {
                Adt2::C0(h427, _, _, _, _, _, _, _, _, _, _) => {
                    h427
                }
            };
            let a429 = 0u64;
            if if if if if if if if if if if if if if if if if if if if if if if if nat_eq(a428, a429) {
                true
            } else {
                let a431 = match v2.clone() {
                    Adt2::C0(h430, _, _, _, _, _, _, _, _, _, _) => {
                        h430
                    }
                };
                let a432 = 1u64;
                nat_eq(a431, a432)
            } {
                let a434 = match v5.clone() {
                    Adt2::C0(h433, _, _, _, _, _, _, _, _, _, _) => {
                        h433
                    }
                };
                let a435 = 0u64;
                nat_eq(a434, a435)
            } else {
                false
            } {
                let a437 = match v4.clone() {
                    Adt2::C0(h436, _, _, _, _, _, _, _, _, _, _) => {
                        h436
                    }
                };
                let a438 = 0u64;
                nat_eq(a437, a438)
            } else {
                false
            } {
                let a439 = v16;
                let a440 = 2147483648u64;
                nat_lt(a439, a440)
            } else {
                false
            } {
                let a441 = 0u64;
                let a442 = v9;
                nat_lt(a441, a442)
            } else {
                false
            } {
                let a443 = 0u64;
                let a444 = v10;
                nat_lt(a443, a444)
            } else {
                false
            } {
                let a445 = 0u64;
                let a446 = v11;
                nat_lt(a445, a446)
            } else {
                false
            } {
                let a447 = 0u64;
                let a448 = v12;
                nat_lt(a447, a448)
            } else {
                false
            } {
                let a449 = 0u64;
                let a450 = v13;
                nat_lt(a449, a450)
            } else {
                false
            } {
                let a451 = 0u64;
                let a452 = v14;
                nat_lt(a451, a452)
            } else {
                false
            } {
                let a453 = 0u64;
                let a454 = v15;
                nat_lt(a453, a454)
            } else {
                false
            } {
                let a455 = 0u64;
                let a456 = v16;
                nat_lt(a455, a456)
            } else {
                false
            } {
                let a458 = match v6.clone() {
                    Adt3::C0(_, _, _, h457, _, _, _, _) => {
                        h457
                    }
                };
                let a459 = v11;
                nat_eq(a458, a459)
            } else {
                false
            } {
                let a461 = match v8.clone() {
                    Adt3::C0(h460, _, _, _, _, _, _, _) => {
                        h460
                    }
                };
                let a462 = v17;
                nat_eq(a461, a462)
            } else {
                false
            } {
                let a464 = match v8.clone() {
                    Adt3::C0(_, h463, _, _, _, _, _, _) => {
                        h463
                    }
                };
                let a465 = v18;
                nat_eq(a464, a465)
            } else {
                false
            } {
                let a467 = match v8.clone() {
                    Adt3::C0(_, _, h466, _, _, _, _, _) => {
                        h466
                    }
                };
                let a468 = v15;
                nat_eq(a467, a468)
            } else {
                false
            } {
                let a470 = match v8.clone() {
                    Adt3::C0(_, _, _, h469, _, _, _, _) => {
                        h469
                    }
                };
                let a471 = v12;
                nat_eq(a470, a471)
            } else {
                false
            } {
                f43(v4.clone())?
            } else {
                false
            } {
                let a476 = match v2.clone() {
                    Adt2::C0(_, _, _, _, _, h472, _, _, _, _, _) => {
                        h472
                    }
                };
                let a477 = {
                    let a474 = match v2.clone() {
                        Adt2::C0(h473, _, _, _, _, _, _, _, _, _, _) => {
                            h473
                        }
                    };
                    let a475 = 1u64;
                    if nat_eq(a474, a475) {
                        2u64
                    } else {
                        4u64
                    }
                };
                nat_eq(a476, a477)
            } else {
                false
            } {
                let a479 = match v5.clone() {
                    Adt2::C0(_, _, _, _, _, h478, _, _, _, _, _) => {
                        h478
                    }
                };
                let a480 = 4u64;
                nat_eq(a479, a480)
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
                    let a485 = match v2.clone() {
                        Adt2::C0(_, _, _, _, _, _, h481, _, _, _, _) => {
                            h481
                        }
                    };
                    let a486 = {
                        let a483 = v13;
                        let a484 = match v2 {
                            Adt2::C0(_, _, _, _, _, h482, _, _, _, _, _) => {
                                h482
                            }
                        };
                        nat_mul(a483, a484)?
                    };
                    nat_eq(a485, a486)
                } else {
                    let a488 = match v1.clone() {
                        Adt1::C0(_, h487, _, _, _, _, _, _, _, _, _, _) => {
                            h487
                        }
                    };
                    let a489 = 0u64;
                    if if if if if if nat_eq(a488, a489) {
                        let a491 = match v1 {
                            Adt1::C0(_, _, h490, _, _, _, _, _, _, _, _, _) => {
                                h490
                            }
                        };
                        let a492 = 1u64;
                        nat_eq(a491, a492)
                    } else {
                        false
                    } {
                        let a494 = match v2 {
                            Adt2::C0(_, _, _, _, h493, _, _, _, _, _, _) => {
                                h493
                            }
                        };
                        let a495 = 1u64;
                        nat_eq(a494, a495)
                    } else {
                        false
                    } {
                        let a497 = match v5.clone() {
                            Adt2::C0(_, _, _, h496, _, _, _, _, _, _, _) => {
                                h496
                            }
                        };
                        let a498 = 1u64;
                        nat_eq(a497, a498)
                    } else {
                        false
                    } {
                        let a500 = match v5 {
                            Adt2::C0(_, _, _, _, h499, _, _, _, _, _, _) => {
                                h499
                            }
                        };
                        let a501 = 1u64;
                        nat_eq(a500, a501)
                    } else {
                        false
                    } {
                        let a503 = match v4.clone() {
                            Adt2::C0(_, _, _, h502, _, _, _, _, _, _, _) => {
                                h502
                            }
                        };
                        let a504 = 1u64;
                        nat_eq(a503, a504)
                    } else {
                        false
                    } {
                        let a506 = match v4 {
                            Adt2::C0(_, _, _, _, h505, _, _, _, _, _, _) => {
                                h505
                            }
                        };
                        let a507 = 1u64;
                        nat_eq(a506, a507)
                    } else {
                        false
                    }
                }
            } else {
                false
            } {
                Ok(Adt10::C6(Adt8::C0(v19, v11, v20, {
                    let a508 = v11;
                    let a509 = v20;
                    nat_mul(a508, a509)?
                }, v21, {
                    let a510 = v21;
                    let a511 = v11;
                    nat_mul(a510, a511)?
                }, {
                    let a512 = v21;
                    let a513 = v20;
                    nat_mul(a512, a513)?
                }, {
                    let a518 = {
                        let a514 = v17;
                        let a515 = v18;
                        nat_mul(a514, a515)?
                    };
                    let a519 = {
                        let a516 = v15;
                        let a517 = v12;
                        nat_mul(a516, a517)?
                    };
                    nat_mul(a518, a519)?
                }, v6, v7, v8, Adt6::C0(v13, v16, 0u64, 1u64), Adt6::C0(v14, v16, 0u64, 1u64), Adt6::C0(1u64, 1u64, 0u64, 1u64), 0u64)))
            } else {
                Ok(Adt10::C1)
            }
        }
    }
}

pub fn f10(v0: bool, v1: Adt1, v2: Adt2, v3: Option<Adt2>, v4: Adt2) -> R<Adt10> {
    let m520 = v3;
    match m520 {
        None => {
            Ok(Adt10::C1)
        }
        Some(v5) => {
            let v6: Adt6 = Adt6::C0(match v2.clone() {
                Adt2::C0(_, h521, _, _, _, _, _, _, _, _, _) => {
                    h521
                }
            }, match v1.clone() {
                Adt1::C0(h522, _, _, _, _, _, _, _, _, _, _, _) => {
                    h522
                }
            }, if v0 {
                match v1.clone() {
                    Adt1::C0(_, _, _, h523, _, _, _, _, _, _, _, _) => {
                        h523
                    }
                }
            } else {
                match v1.clone() {
                    Adt1::C0(_, _, h524, _, _, _, _, _, _, _, _, _) => {
                        h524
                    }
                }
            }, if v0 {
                match v1.clone() {
                    Adt1::C0(_, _, _, _, _, _, h525, _, _, _, _, _) => {
                        h525
                    }
                }
            } else {
                match v1.clone() {
                    Adt1::C0(_, _, _, _, h526, _, _, _, _, _, _, _) => {
                        h526
                    }
                }
            });
            let v7: Adt6 = Adt6::C0(match v2.clone() {
                Adt2::C0(_, _, h527, _, _, _, _, _, _, _, _) => {
                    h527
                }
            }, match v1.clone() {
                Adt1::C0(_, h528, _, _, _, _, _, _, _, _, _, _) => {
                    h528
                }
            }, if v0 {
                match v1.clone() {
                    Adt1::C0(_, _, _, _, h529, _, _, _, _, _, _, _) => {
                        h529
                    }
                }
            } else {
                match v1.clone() {
                    Adt1::C0(_, _, _, h530, _, _, _, _, _, _, _, _) => {
                        h530
                    }
                }
            }, if v0 {
                match v1.clone() {
                    Adt1::C0(_, _, _, _, _, _, _, h531, _, _, _, _) => {
                        h531
                    }
                }
            } else {
                match v1.clone() {
                    Adt1::C0(_, _, _, _, _, h532, _, _, _, _, _, _) => {
                        h532
                    }
                }
            });
            let v8: Adt6 = if v0 {
                Adt6::C0(match v2.clone() {
                    Adt2::C0(_, _, _, h533, _, _, _, _, _, _, _) => {
                        h533
                    }
                }, match v1.clone() {
                    Adt1::C0(_, _, h534, _, _, _, _, _, _, _, _, _) => {
                        h534
                    }
                }, match v1.clone() {
                    Adt1::C0(_, _, _, _, _, h535, _, _, _, _, _, _) => {
                        h535
                    }
                }, match v1.clone() {
                    Adt1::C0(_, _, _, _, _, _, _, _, h536, _, _, _) => {
                        h536
                    }
                })
            } else {
                Adt6::C0(1u64, 1u64, 0u64, 1u64)
            };
            let v9: u64 = f44(v6.clone(), match v5.clone() {
                Adt2::C0(_, h537, _, _, _, _, _, _, _, _, _) => {
                    h537
                }
            })?;
            let v10: u64 = f44(v7.clone(), match v5.clone() {
                Adt2::C0(_, _, h538, _, _, _, _, _, _, _, _) => {
                    h538
                }
            })?;
            let v11: u64 = if v0 {
                f44(v8.clone(), match v5.clone() {
                    Adt2::C0(_, _, _, h539, _, _, _, _, _, _, _) => {
                        h539
                    }
                })?
            } else {
                1u64
            };
            let v12: u64 = if v0 {
                match v1.clone() {
                    Adt1::C0(_, _, _, _, _, _, _, _, _, h540, _, _) => {
                        h540
                    }
                }
            } else {
                match v5.clone() {
                    Adt2::C0(_, _, _, h541, _, _, _, _, _, _, _) => {
                        h541
                    }
                }
            };
            let v13: u64 = if v0 {
                match v1.clone() {
                    Adt1::C0(_, _, _, _, _, _, _, _, _, _, h542, _) => {
                        h542
                    }
                }
            } else {
                match v5.clone() {
                    Adt2::C0(_, _, _, _, h543, _, _, _, _, _, _) => {
                        h543
                    }
                }
            };
            let v14: u64 = if v0 {
                match v1 {
                    Adt1::C0(_, _, _, _, _, _, _, _, _, _, _, h544) => {
                        h544
                    }
                }
            } else {
                match v2.clone() {
                    Adt2::C0(_, _, _, _, h545, _, _, _, _, _, _) => {
                        h545
                    }
                }
            };
            let v15: u64 = {
                let a550 = {
                    let a546 = v9;
                    let a547 = v10;
                    nat_mul(a546, a547)?
                };
                let a551 = {
                    let a548 = v11;
                    let a549 = v13;
                    nat_mul(a548, a549)?
                };
                nat_mul(a550, a551)?
            };
            let v16: u64 = {
                let a559 = {
                    let a554 = match v6.clone() {
                        Adt6::C0(h552, _, _, _) => {
                            h552
                        }
                    };
                    let a555 = match v7.clone() {
                        Adt6::C0(h553, _, _, _) => {
                            h553
                        }
                    };
                    nat_mul(a554, a555)?
                };
                let a560 = {
                    let a557 = match v8.clone() {
                        Adt6::C0(h556, _, _, _) => {
                            h556
                        }
                    };
                    let a558 = v12;
                    nat_mul(a557, a558)?
                };
                nat_mul(a559, a560)?
            };
            let v17: u64 = {
                let a561 = v15;
                let a562 = 32u64;
                if nat_lt(a561, a562) {
                    v15
                } else {
                    32u64
                }
            };
            let a564 = match v2.clone() {
                Adt2::C0(h563, _, _, _, _, _, _, _, _, _, _) => {
                    h563
                }
            };
            let a565 = 0u64;
            if if if if if if if if if if if nat_eq(a564, a565) {
                true
            } else {
                let a567 = match v2.clone() {
                    Adt2::C0(h566, _, _, _, _, _, _, _, _, _, _) => {
                        h566
                    }
                };
                let a568 = 1u64;
                nat_eq(a567, a568)
            } {
                let a570 = match v5.clone() {
                    Adt2::C0(h569, _, _, _, _, _, _, _, _, _, _) => {
                        h569
                    }
                };
                let a571 = 0u64;
                nat_eq(a570, a571)
            } else {
                false
            } {
                let a573 = match v4.clone() {
                    Adt2::C0(h572, _, _, _, _, _, _, _, _, _, _) => {
                        h572
                    }
                };
                let a574 = 0u64;
                nat_eq(a573, a574)
            } else {
                false
            } {
                let a576 = match v4.clone() {
                    Adt2::C0(_, h575, _, _, _, _, _, _, _, _, _) => {
                        h575
                    }
                };
                let a577 = v9;
                nat_eq(a576, a577)
            } else {
                false
            } {
                let a579 = match v4.clone() {
                    Adt2::C0(_, _, h578, _, _, _, _, _, _, _, _) => {
                        h578
                    }
                };
                let a580 = v10;
                nat_eq(a579, a580)
            } else {
                false
            } {
                if v0 {
                    let a581 = 0u64;
                    let a582 = v12;
                    if if if if if if if nat_lt(a581, a582) {
                        let a583 = v12;
                        let a584 = 2147483648u64;
                        nat_lt(a583, a584)
                    } else {
                        false
                    } {
                        let a585 = v14;
                        let a586 = 2147483648u64;
                        nat_lt(a585, a586)
                    } else {
                        false
                    } {
                        let a587 = v13;
                        let a588 = 2147483648u64;
                        nat_lt(a587, a588)
                    } else {
                        false
                    } {
                        let a592 = match v2.clone() {
                            Adt2::C0(_, _, _, _, h589, _, _, _, _, _, _) => {
                                h589
                            }
                        };
                        let a593 = {
                            let a590 = v12;
                            let a591 = v14;
                            nat_mul(a590, a591)?
                        };
                        nat_eq(a592, a593)
                    } else {
                        false
                    } {
                        let a597 = match v5.clone() {
                            Adt2::C0(_, _, _, _, h594, _, _, _, _, _, _) => {
                                h594
                            }
                        };
                        let a598 = {
                            let a595 = v12;
                            let a596 = v13;
                            nat_mul(a595, a596)?
                        };
                        nat_eq(a597, a598)
                    } else {
                        false
                    } {
                        let a600 = match v4.clone() {
                            Adt2::C0(_, _, _, h599, _, _, _, _, _, _, _) => {
                                h599
                            }
                        };
                        let a601 = v11;
                        nat_eq(a600, a601)
                    } else {
                        false
                    } {
                        let a605 = match v4.clone() {
                            Adt2::C0(_, _, _, _, h602, _, _, _, _, _, _) => {
                                h602
                            }
                        };
                        let a606 = {
                            let a603 = v14;
                            let a604 = v13;
                            nat_mul(a603, a604)?
                        };
                        nat_eq(a605, a606)
                    } else {
                        false
                    }
                } else {
                    let a608 = match v2.clone() {
                        Adt2::C0(_, _, _, h607, _, _, _, _, _, _, _) => {
                            h607
                        }
                    };
                    let a609 = v12;
                    if if if nat_eq(a608, a609) {
                        let a611 = match v4.clone() {
                            Adt2::C0(_, _, _, h610, _, _, _, _, _, _, _) => {
                                h610
                            }
                        };
                        let a612 = v14;
                        nat_eq(a611, a612)
                    } else {
                        false
                    } {
                        let a614 = match v4.clone() {
                            Adt2::C0(_, _, _, _, h613, _, _, _, _, _, _) => {
                                h613
                            }
                        };
                        let a615 = v13;
                        nat_eq(a614, a615)
                    } else {
                        false
                    } {
                        let a616 = 0u64;
                        let a617 = v16;
                        nat_lt(a616, a617)
                    } else {
                        false
                    }
                }
            } else {
                false
            } {
                let a618 = 0u64;
                let a619 = v14;
                nat_lt(a618, a619)
            } else {
                false
            } {
                let a620 = 0u64;
                let a621 = v15;
                nat_lt(a620, a621)
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
                    let a622 = v16;
                    let a623 = v14;
                    nat_mul(a622, a623)?
                }, v17, {
                    let a624 = v17;
                    let a625 = v16;
                    nat_mul(a624, a625)?
                }, {
                    let a626 = v17;
                    let a627 = v14;
                    nat_mul(a626, a627)?
                }, 0u64, f30(v2, 1u64), f30(v5, 1u64), f30(v4, 1u64), v6, v7, v8, if v0 {
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

pub fn f11(v0: Adt1, v1: Adt2, v2: Option<Adt2>, v3: Option<Adt2>, v4: Adt2) -> R<Adt10> {
    let m628 = v2;
    match m628 {
        None => {
            Ok(Adt10::C1)
        }
        Some(v5) => {
            let m629 = v3;
            match m629 {
                None => {
                    Ok(Adt10::C1)
                }
                Some(v6) => {
                    let a631 = match v1.clone() {
                        Adt2::C0(h630, _, _, _, _, _, _, _, _, _, _) => {
                            h630
                        }
                    };
                    let a632 = 0u64;
                    if if if if if if if if if if if if if if if nat_eq(a631, a632) {
                        let a634 = match v5.clone() {
                            Adt2::C0(h633, _, _, _, _, _, _, _, _, _, _) => {
                                h633
                            }
                        };
                        let a635 = 0u64;
                        nat_eq(a634, a635)
                    } else {
                        false
                    } {
                        let a637 = match v6.clone() {
                            Adt2::C0(h636, _, _, _, _, _, _, _, _, _, _) => {
                                h636
                            }
                        };
                        let a638 = 0u64;
                        nat_eq(a637, a638)
                    } else {
                        false
                    } {
                        let a640 = match v4.clone() {
                            Adt2::C0(h639, _, _, _, _, _, _, _, _, _, _) => {
                                h639
                            }
                        };
                        let a641 = 0u64;
                        nat_eq(a640, a641)
                    } else {
                        false
                    } {
                        f26(v5.clone(), v6.clone())
                    } else {
                        false
                    } {
                        f26(v1.clone(), v4.clone())
                    } else {
                        false
                    } {
                        f43(v1.clone())?
                    } else {
                        false
                    } {
                        f43(v5.clone())?
                    } else {
                        false
                    } {
                        f43(v6)?
                    } else {
                        false
                    } {
                        f43(v4.clone())?
                    } else {
                        false
                    } {
                        let a647 = match v1.clone() {
                            Adt2::C0(_, h642, _, _, _, _, _, _, _, _, _) => {
                                h642
                            }
                        };
                        let a648 = {
                            let a645 = match v5.clone() {
                                Adt2::C0(_, h643, _, _, _, _, _, _, _, _, _) => {
                                    h643
                                }
                            };
                            let a646 = match v5.clone() {
                                Adt2::C0(_, h644, _, _, _, _, _, _, _, _, _) => {
                                    h644
                                }
                            };
                            nat_mul(a645, a646)?
                        };
                        nat_eq(a647, a648)
                    } else {
                        false
                    } {
                        let a654 = match v1.clone() {
                            Adt2::C0(_, _, h649, _, _, _, _, _, _, _, _) => {
                                h649
                            }
                        };
                        let a655 = {
                            let a652 = match v5.clone() {
                                Adt2::C0(_, _, h650, _, _, _, _, _, _, _, _) => {
                                    h650
                                }
                            };
                            let a653 = match v5.clone() {
                                Adt2::C0(_, _, _, h651, _, _, _, _, _, _, _) => {
                                    h651
                                }
                            };
                            nat_mul(a652, a653)?
                        };
                        nat_eq(a654, a655)
                    } else {
                        false
                    } {
                        let a658 = match v1.clone() {
                            Adt2::C0(_, _, _, h656, _, _, _, _, _, _, _) => {
                                h656
                            }
                        };
                        let a659 = match v5.clone() {
                            Adt2::C0(_, _, _, _, h657, _, _, _, _, _, _) => {
                                h657
                            }
                        };
                        nat_eq(a658, a659)
                    } else {
                        false
                    } {
                        let a661 = match v0.clone() {
                            Adt1::C0(h660, _, _, _, _, _, _, _, _, _, _, _) => {
                                h660
                            }
                        };
                        let a662 = 4294967296u64;
                        nat_lt(a661, a662)
                    } else {
                        false
                    } {
                        let a664 = match v0.clone() {
                            Adt1::C0(h663, _, _, _, _, _, _, _, _, _, _, _) => {
                                h663
                            }
                        };
                        let a665 = 0u64;
                        if nat_eq(a664, a665) {
                            if f35(v1.clone(), v4.clone())? {
                                true
                            } else {
                                let a668 = match v1.clone() {
                                    Adt2::C0(_, _, _, _, _, _, _, _, _, _, h666) => {
                                        h666
                                    }
                                };
                                let a669 = match v4.clone() {
                                    Adt2::C0(_, _, _, _, _, _, _, _, _, _, h667) => {
                                        h667
                                    }
                                };
                                nat_eq(a668, a669)
                            }
                        } else {
                            let a672 = match v1.clone() {
                                Adt2::C0(_, _, _, _, _, _, _, _, _, _, h670) => {
                                    h670
                                }
                            };
                            let a673 = match v4.clone() {
                                Adt2::C0(_, _, _, _, _, _, _, _, _, _, h671) => {
                                    h671
                                }
                            };
                            nat_eq(a672, a673)
                        }
                    } else {
                        false
                    } {
                        let v7: bool = {
                            let a675 = match v4.clone() {
                                Adt2::C0(_, h674, _, _, _, _, _, _, _, _, _) => {
                                    h674
                                }
                            };
                            let a676 = 0u64;
                            if if if nat_eq(a675, a676) {
                                true
                            } else {
                                let a678 = match v4.clone() {
                                    Adt2::C0(_, _, h677, _, _, _, _, _, _, _, _) => {
                                        h677
                                    }
                                };
                                let a679 = 0u64;
                                nat_eq(a678, a679)
                            } {
                                true
                            } else {
                                let a681 = match v4.clone() {
                                    Adt2::C0(_, _, _, h680, _, _, _, _, _, _, _) => {
                                        h680
                                    }
                                };
                                let a682 = 0u64;
                                nat_eq(a681, a682)
                            } {
                                true
                            } else {
                                let a684 = match v4.clone() {
                                    Adt2::C0(_, _, _, _, h683, _, _, _, _, _, _) => {
                                        h683
                                    }
                                };
                                let a685 = 0u64;
                                nat_eq(a684, a685)
                            }
                        };
                        let v8: u64 = match v5.clone() {
                            Adt2::C0(_, h686, _, _, _, _, _, _, _, _, _) => {
                                h686
                            }
                        };
                        let v9: u64 = if v7 {
                            0u64
                        } else {
                            let a688 = match v0 {
                                Adt1::C0(h687, _, _, _, _, _, _, _, _, _, _, _) => {
                                    h687
                                }
                            };
                            let a689 = 0u64;
                            if nat_eq(a688, a689) {
                                f28(v1.clone(), 1u64)?
                            } else {
                                0u64
                            }
                        };
                        let v10: u64 = if v7 {
                            0u64
                        } else {
                            f45(v8, match v5.clone() {
                                Adt2::C0(_, _, h690, _, _, _, _, _, _, _, _) => {
                                    h690
                                }
                            }, match v5.clone() {
                                Adt2::C0(_, _, _, h691, _, _, _, _, _, _, _) => {
                                    h691
                                }
                            }, match v5 {
                                Adt2::C0(_, _, _, _, h692, _, _, _, _, _, _) => {
                                    h692
                                }
                            })?
                        };
                        Ok(Adt10::C4(f46(v9, v8, v10)?, 4u64, f30(v1, 1u64), f30(v4, 1u64), Adt4::C25(v9, v8, v10)))
                    } else {
                        Ok(Adt10::C1)
                    }
                }
            }
        }
    }
}

pub fn f12(v0: u64, v1: Adt1, v2: Adt2, v3: Option<Adt2>, v4: Adt2) -> R<Adt10> {
    let v5: Adt14 = f47(v2.clone());
    let v6: Adt14 = f47(v4.clone());
    let v8: Adt15 = {
        let m693 = v3;
        match m693 {
            None => {
                f48(v0, match v1.clone() {
                    Adt1::C0(h694, _, _, _, _, _, _, _, _, _, _, _) => {
                        h694
                    }
                }, match v1 {
                    Adt1::C0(_, h695, _, _, _, _, _, _, _, _, _, _) => {
                        h695
                    }
                }, false, v5.clone(), v5, v6)?
            }
            Some(v7) => {
                f48(v0, match v1.clone() {
                    Adt1::C0(h696, _, _, _, _, _, _, _, _, _, _, _) => {
                        h696
                    }
                }, match v1 {
                    Adt1::C0(_, h697, _, _, _, _, _, _, _, _, _, _) => {
                        h697
                    }
                }, true, v5, f47(v7), v6)?
            }
        }
    };
    let m698 = v8;
    match m698 {
        Adt15::C0 => {
            Ok(Adt10::C1)
        }
        Adt15::C1(v9, v10, v11, v12, v13, v14) => {
            Ok(Adt10::C4(v9, v10, f30(v2, 1u64), f30(v4, 1u64), Adt4::C26(v11, v14, v12, v13)))
        }
    }
}

pub fn f13(v0: Adt1, v1: Adt2, v2: Adt2) -> R<Adt10> {
    let v3: u64 = match v1.clone() {
        Adt2::C0(h699, _, _, _, _, _, _, _, _, _, _) => {
            h699
        }
    };
    let v4: u64 = f49(v3);
    let a701 = match v1.clone() {
        Adt2::C0(h700, _, _, _, _, _, _, _, _, _, _) => {
            h700
        }
    };
    let a702 = 0u64;
    if if if if if if if if nat_eq(a701, a702) {
        true
    } else {
        let a704 = match v1.clone() {
            Adt2::C0(h703, _, _, _, _, _, _, _, _, _, _) => {
                h703
            }
        };
        let a705 = 30u64;
        nat_eq(a704, a705)
    } {
        let a708 = match v1.clone() {
            Adt2::C0(h706, _, _, _, _, _, _, _, _, _, _) => {
                h706
            }
        };
        let a709 = match v2.clone() {
            Adt2::C0(h707, _, _, _, _, _, _, _, _, _, _) => {
                h707
            }
        };
        nat_eq(a708, a709)
    } else {
        false
    } {
        f26(v1.clone(), v2.clone())
    } else {
        false
    } {
        f50(match v0.clone() {
            Adt1::C0(h710, _, _, _, _, _, _, _, _, _, _, _) => {
                h710
            }
        })
    } else {
        false
    } {
        f51(v1.clone(), v4)
    } else {
        false
    } {
        f51(v2.clone(), v4)
    } else {
        false
    } {
        if f35(v1.clone(), v2.clone())? {
            true
        } else {
            f52(v1.clone(), v2.clone())
        }
    } else {
        false
    } {
        let v5: u64 = if f53(match v1.clone() {
            Adt2::C0(_, h711, _, _, _, _, _, _, _, _, _) => {
                h711
            }
        }, match v1.clone() {
            Adt2::C0(_, _, h712, _, _, _, _, _, _, _, _) => {
                h712
            }
        }, match v1.clone() {
            Adt2::C0(_, _, _, h713, _, _, _, _, _, _, _) => {
                h713
            }
        }, match v1.clone() {
            Adt2::C0(_, _, _, _, h714, _, _, _, _, _, _) => {
                h714
            }
        }) {
            0u64
        } else {
            let a720 = match v1.clone() {
                Adt2::C0(_, _, h715, _, _, _, _, _, _, _, _) => {
                    h715
                }
            };
            let a721 = {
                let a718 = match v1.clone() {
                    Adt2::C0(_, _, _, h716, _, _, _, _, _, _, _) => {
                        h716
                    }
                };
                let a719 = match v1.clone() {
                    Adt2::C0(_, _, _, _, h717, _, _, _, _, _, _) => {
                        h717
                    }
                };
                nat_mul(a718, a719)?
            };
            nat_mul(a720, a721)?
        };
        let v6: u64 = {
            let a722 = v5;
            let a723 = 0u64;
            if if nat_eq(a722, a723) {
                true
            } else {
                f54(v2.clone(), v4)?
            } {
                0u64
            } else {
                let a725 = match v2.clone() {
                    Adt2::C0(_, _, _, _, _, _, _, _, _, h724, _) => {
                        h724
                    }
                };
                let a726 = v4;
                let a727 = 0u64;
                nat_quot(a725, a726, a727)
            }
        };
        Ok(Adt10::C4(f55(v3, match v1.clone() {
            Adt2::C0(_, h728, _, _, _, _, _, _, _, _, _) => {
                h728
            }
        }, v5, v6)?, v4, f30(v1.clone(), 1u64), f30(v2, 1u64), Adt4::C26(v3, match v0 {
            Adt1::C0(h729, _, _, _, _, _, _, _, _, _, _, _) => {
                h729
            }
        }, v5, f56(match v1 {
            Adt2::C0(_, h730, _, _, _, _, _, _, _, _, _) => {
                h730
            }
        }, v5, v6)?)))
    } else {
        Ok(Adt10::C1)
    }
}

pub fn f14(v0: Adt1, v1: Adt2, v2: Adt2) -> R<Adt10> {
    let a732 = match v1.clone() {
        Adt2::C0(h731, _, _, _, _, _, _, _, _, _, _) => {
            h731
        }
    };
    let a733 = 0u64;
    if if if if if nat_eq(a732, a733) {
        let a735 = match v2.clone() {
            Adt2::C0(h734, _, _, _, _, _, _, _, _, _, _) => {
                h734
            }
        };
        let a736 = 0u64;
        nat_eq(a735, a736)
    } else {
        false
    } {
        f26(v1.clone(), v2.clone())
    } else {
        false
    } {
        f57(match v0.clone() {
            Adt1::C0(h737, _, _, _, _, _, _, _, _, _, _, _) => {
                h737
            }
        })
    } else {
        false
    } {
        if f35(v1.clone(), v2.clone())? {
            true
        } else {
            f52(v1.clone(), v2.clone())
        }
    } else {
        false
    } {
        let v3: Adt3 = f30(v1, 1u64);
        let v4: u64 = {
            let a743 = match v3.clone() {
                Adt3::C0(_, h738, _, _, _, _, _, _) => {
                    h738
                }
            };
            let a744 = {
                let a741 = match v3.clone() {
                    Adt3::C0(_, _, h739, _, _, _, _, _) => {
                        h739
                    }
                };
                let a742 = match v3.clone() {
                    Adt3::C0(_, _, _, h740, _, _, _, _) => {
                        h740
                    }
                };
                nat_mul(a741, a742)?
            };
            nat_mul(a743, a744)?
        };
        Ok(Adt10::C4(f58(match v3.clone() {
            Adt3::C0(h745, _, _, _, _, _, _, _) => {
                h745
            }
        }, v4)?, 4u64, v3, f30(v2, 1u64), Adt4::C24(match v0 {
            Adt1::C0(h746, _, _, _, _, _, _, _, _, _, _, _) => {
                h746
            }
        })))
    } else {
        Ok(Adt10::C1)
    }
}

pub fn f15(v0: Adt2, v1: Option<Adt2>, v2: Option<Adt2>, v3: Adt2) -> R<Adt10> {
    let m747 = v1;
    match m747 {
        None => {
            Ok(Adt10::C1)
        }
        Some(v4) => {
            let m748 = v2;
            match m748 {
                None => {
                    Ok(Adt10::C1)
                }
                Some(v5) => {
                    let a750 = match v0.clone() {
                        Adt2::C0(h749, _, _, _, _, _, _, _, _, _, _) => {
                            h749
                        }
                    };
                    let a751 = 0u64;
                    if if if if if if if if if if if if if if if if if nat_eq(a750, a751) {
                        let a753 = match v4.clone() {
                            Adt2::C0(h752, _, _, _, _, _, _, _, _, _, _) => {
                                h752
                            }
                        };
                        let a754 = 0u64;
                        nat_eq(a753, a754)
                    } else {
                        false
                    } {
                        let a756 = match v5.clone() {
                            Adt2::C0(h755, _, _, _, _, _, _, _, _, _, _) => {
                                h755
                            }
                        };
                        let a757 = 26u64;
                        nat_eq(a756, a757)
                    } else {
                        false
                    } {
                        let a759 = match v3.clone() {
                            Adt2::C0(h758, _, _, _, _, _, _, _, _, _, _) => {
                                h758
                            }
                        };
                        let a760 = 0u64;
                        nat_eq(a759, a760)
                    } else {
                        false
                    } {
                        f26(v0.clone(), v3.clone())
                    } else {
                        false
                    } {
                        let a763 = match v0.clone() {
                            Adt2::C0(_, h761, _, _, _, _, _, _, _, _, _) => {
                                h761
                            }
                        };
                        let a764 = match v4.clone() {
                            Adt2::C0(_, h762, _, _, _, _, _, _, _, _, _) => {
                                h762
                            }
                        };
                        nat_eq(a763, a764)
                    } else {
                        false
                    } {
                        let a767 = match v0.clone() {
                            Adt2::C0(_, _, h765, _, _, _, _, _, _, _, _) => {
                                h765
                            }
                        };
                        let a768 = match v5.clone() {
                            Adt2::C0(_, h766, _, _, _, _, _, _, _, _, _) => {
                                h766
                            }
                        };
                        nat_eq(a767, a768)
                    } else {
                        false
                    } {
                        let a771 = match v0.clone() {
                            Adt2::C0(_, _, _, h769, _, _, _, _, _, _, _) => {
                                h769
                            }
                        };
                        let a772 = match v5.clone() {
                            Adt2::C0(_, _, h770, _, _, _, _, _, _, _, _) => {
                                h770
                            }
                        };
                        nat_eq(a771, a772)
                    } else {
                        false
                    } {
                        let a774 = 0u64;
                        let a775 = match v5.clone() {
                            Adt2::C0(_, _, _, h773, _, _, _, _, _, _, _) => {
                                h773
                            }
                        };
                        nat_lt(a774, a775)
                    } else {
                        false
                    } {
                        let a777 = 0u64;
                        let a778 = match v5.clone() {
                            Adt2::C0(_, _, _, _, h776, _, _, _, _, _, _) => {
                                h776
                            }
                        };
                        nat_lt(a777, a778)
                    } else {
                        false
                    } {
                        let a780 = 4u64;
                        let a781 = match v0.clone() {
                            Adt2::C0(_, _, _, _, _, h779, _, _, _, _, _) => {
                                h779
                            }
                        };
                        nat_le(a780, a781)
                    } else {
                        false
                    } {
                        let a783 = 4u64;
                        let a784 = match v4.clone() {
                            Adt2::C0(_, _, _, _, _, h782, _, _, _, _, _) => {
                                h782
                            }
                        };
                        nat_le(a783, a784)
                    } else {
                        false
                    } {
                        let a786 = 4u64;
                        let a787 = match v5.clone() {
                            Adt2::C0(_, _, _, _, _, h785, _, _, _, _, _) => {
                                h785
                            }
                        };
                        nat_le(a786, a787)
                    } else {
                        false
                    } {
                        let a789 = 4u64;
                        let a790 = match v3.clone() {
                            Adt2::C0(_, _, _, _, _, h788, _, _, _, _, _) => {
                                h788
                            }
                        };
                        nat_le(a789, a790)
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
                        Ok(Adt10::C4(f59(match v0.clone() {
                            Adt2::C0(_, h791, _, _, _, _, _, _, _, _, _) => {
                                h791
                            }
                        }, match v0.clone() {
                            Adt2::C0(_, _, h792, _, _, _, _, _, _, _, _) => {
                                h792
                            }
                        }, match v0.clone() {
                            Adt2::C0(_, _, _, h793, _, _, _, _, _, _, _) => {
                                h793
                            }
                        }, match v0.clone() {
                            Adt2::C0(_, _, _, _, h794, _, _, _, _, _, _) => {
                                h794
                            }
                        })?, 4u64, f30(v0, 1u64), f30(v3, 1u64), Adt4::C22(f30(v4, 1u64), f30(v5, 1u64))))
                    } else {
                        Ok(Adt10::C1)
                    }
                }
            }
        }
    }
}

pub fn f16(v0: u64, v1: Adt1, v2: Adt2, v3: Option<Adt2>, v4: Adt2) -> R<Adt9> {
    let a795 = v0;
    let a796 = 28u64;
    if if if nat_eq(a795, a796) {
        true
    } else {
        let a797 = v0;
        let a798 = 29u64;
        nat_eq(a797, a798)
    } {
        let a800 = 0u64;
        let a801 = match v1.clone() {
            Adt1::C0(h799, _, _, _, _, _, _, _, _, _, _, _) => {
                h799
            }
        };
        if nat_lt(a800, a801) {
            true
        } else {
            let a803 = 0u64;
            let a804 = match v1.clone() {
                Adt1::C0(_, _, _, h802, _, _, _, _, _, _, _, _) => {
                    h802
                }
            };
            nat_lt(a803, a804)
        }
    } else {
        false
    } {
        let m808 = f60(match v2.clone() {
            Adt2::C0(h805, _, _, _, _, _, _, _, _, _, _) => {
                h805
            }
        }, match v1.clone() {
            Adt1::C0(h806, _, _, _, _, _, _, _, _, _, _, _) => {
                h806
            }
        }, match v1.clone() {
            Adt1::C0(_, _, _, h807, _, _, _, _, _, _, _, _) => {
                h807
            }
        });
        match m808 {
            None => {
                Ok(Adt9::C0(Adt10::C8))
            }
            Some(v5) => {
                if v5 {
                    Ok(Adt9::C0(Adt10::C7))
                } else {
                    f61(v0, v1, v2, v3, v4)
                }
            }
        }
    } else {
        f61(v0, v1, v2, v3, v4)
    }
}

pub fn f17(v0: Adt8, v1: u64, v2: u64, v3: u64) -> R<Adt9> {
    let v4: u64 = match v0.clone() {
        Adt8::C0(_, h809, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h809
        }
    };
    let v5: u64 = match v0.clone() {
        Adt8::C0(_, _, h810, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h810
        }
    };
    let v6: u64 = {
        let a811 = v3;
        let a812 = v5;
        let a813 = 0u64;
        nat_rem(a811, a812, a813)
    };
    let a814 = v1;
    let a815 = 0u64;
    if nat_eq(a814, a815) {
        let a817 = v3;
        let a818 = match v0.clone() {
            Adt8::C0(_, _, _, h816, _, _, _, _, _, _, _, _, _, _, _) => {
                h816
            }
        };
        if nat_lt(a817, a818) {
            Ok(Adt9::C8(Some(f62(match v0.clone() {
                Adt8::C0(_, _, _, _, _, _, _, _, h819, _, _, _, _, _, _) => {
                    h819
                }
            }, {
                let a823 = v6;
                let a824 = {
                    let m822 = match v0.clone() {
                        Adt8::C0(_, _, _, _, _, _, _, _, h821, _, _, _, _, _, _) => {
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
                nat_rem(a823, a824, a825)
            }, {
                let a835 = {
                    let a829 = v6;
                    let a830 = {
                        let m828 = match v0.clone() {
                            Adt8::C0(_, _, _, _, _, _, _, _, h827, _, _, _, _, _, _) => {
                                h827
                            }
                        };
                        match m828 {
                            Adt3::C0(h826, _, _, _, _, _, _, _) => {
                                h826
                            }
                        }
                    };
                    let a831 = 0u64;
                    nat_quot(a829, a830, a831)
                };
                let a836 = {
                    let m834 = match v0.clone() {
                        Adt8::C0(_, _, _, _, _, _, _, _, h833, _, _, _, _, _, _) => {
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
                nat_rem(a835, a836, a837)
            }, {
                let a847 = {
                    let a841 = v6;
                    let a842 = {
                        let m840 = match v0.clone() {
                            Adt8::C0(_, _, _, _, _, _, _, _, h839, _, _, _, _, _, _) => {
                                h839
                            }
                        };
                        match m840 {
                            Adt3::C0(h838, _, _, _, _, _, _, _) => {
                                h838
                            }
                        }
                    };
                    let a843 = 0u64;
                    nat_quot(a841, a842, a843)
                };
                let a848 = {
                    let m846 = match v0 {
                        Adt8::C0(_, _, _, _, _, _, _, _, h845, _, _, _, _, _, _) => {
                            h845
                        }
                    };
                    match m846 {
                        Adt3::C0(_, h844, _, _, _, _, _, _) => {
                            h844
                        }
                    }
                };
                let a849 = 0u64;
                nat_quot(a847, a848, a849)
            }, {
                let a850 = v3;
                let a851 = v5;
                let a852 = 0u64;
                nat_quot(a850, a851, a852)
            })?)))
        } else {
            Ok(Adt9::C0(Adt10::C1))
        }
    } else {
        let v7: u64 = {
            let a858 = v2;
            let a859 = {
                let a855 = v3;
                let a856 = {
                    let a853 = v1;
                    let a854 = 1u64;
                    if nat_eq(a853, a854) {
                        v4
                    } else {
                        v5
                    }
                };
                let a857 = 0u64;
                nat_quot(a855, a856, a857)
            };
            nat_add(a858, a859)?
        };
        let a861 = v7;
        let a862 = match v0.clone() {
            Adt8::C0(h860, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h860
            }
        };
        if nat_lt(a861, a862) {
            let v8: u64 = {
                let a866 = v7;
                let a867 = {
                    let m865 = match v0.clone() {
                        Adt8::C0(_, _, _, _, _, _, _, _, _, h864, _, _, _, _, _) => {
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
                nat_rem(a866, a867, a868)
            };
            let v9: u64 = {
                let a872 = v7;
                let a873 = {
                    let m871 = match v0.clone() {
                        Adt8::C0(_, _, _, _, _, _, _, _, _, h870, _, _, _, _, _) => {
                            h870
                        }
                    };
                    match m871 {
                        Adt3::C0(h869, _, _, _, _, _, _, _) => {
                            h869
                        }
                    }
                };
                let a874 = 0u64;
                nat_quot(a872, a873, a874)
            };
            let v10: u64 = {
                let a878 = v9;
                let a879 = {
                    let m877 = match v0.clone() {
                        Adt8::C0(_, _, _, _, _, _, _, _, _, h876, _, _, _, _, _) => {
                            h876
                        }
                    };
                    match m877 {
                        Adt3::C0(_, h875, _, _, _, _, _, _) => {
                            h875
                        }
                    }
                };
                let a880 = 0u64;
                nat_rem(a878, a879, a880)
            };
            let v11: u64 = {
                let a884 = v9;
                let a885 = {
                    let m883 = match v0.clone() {
                        Adt8::C0(_, _, _, _, _, _, _, _, _, h882, _, _, _, _, _) => {
                            h882
                        }
                    };
                    match m883 {
                        Adt3::C0(_, h881, _, _, _, _, _, _) => {
                            h881
                        }
                    }
                };
                let a886 = 0u64;
                nat_quot(a884, a885, a886)
            };
            let a887 = v1;
            let a888 = 1u64;
            if nat_eq(a887, a888) {
                Ok(Adt9::C8(Some(f62(match v0 {
                    Adt8::C0(_, _, _, _, _, _, _, _, _, h889, _, _, _, _, _) => {
                        h889
                    }
                }, v8, v10, {
                    let a890 = v3;
                    let a891 = v4;
                    let a892 = 0u64;
                    nat_rem(a890, a891, a892)
                }, v11)?)))
            } else {
                let a893 = v1;
                let a894 = 2u64;
                if nat_eq(a893, a894) {
                    Ok(Adt9::C8(Some({
                        let a940 = f62(match v0.clone() {
                            Adt8::C0(_, _, _, _, _, _, _, _, _, _, h895, _, _, _, _) => {
                                h895
                            }
                        }, {
                            let a907 = {
                                let a899 = v8;
                                let a900 = {
                                    let m898 = match v0.clone() {
                                        Adt8::C0(_, _, _, _, _, _, _, _, _, _, _, h897, _, _, _) => {
                                            h897
                                        }
                                    };
                                    match m898 {
                                        Adt6::C0(_, h896, _, _) => {
                                            h896
                                        }
                                    }
                                };
                                nat_mul(a899, a900)?
                            };
                            let a908 = {
                                let a904 = v6;
                                let a905 = {
                                    let m903 = match v0.clone() {
                                        Adt8::C0(_, _, _, _, _, _, _, _, h902, _, _, _, _, _, _) => {
                                            h902
                                        }
                                    };
                                    match m903 {
                                        Adt3::C0(h901, _, _, _, _, _, _, _) => {
                                            h901
                                        }
                                    }
                                };
                                let a906 = 0u64;
                                nat_rem(a904, a905, a906)
                            };
                            nat_add(a907, a908)?
                        }, {
                            let a926 = {
                                let a912 = v10;
                                let a913 = {
                                    let m911 = match v0.clone() {
                                        Adt8::C0(_, _, _, _, _, _, _, _, _, _, _, h910, _, _, _) => {
                                            h910
                                        }
                                    };
                                    match m911 {
                                        Adt6::C0(_, h909, _, _) => {
                                            h909
                                        }
                                    }
                                };
                                nat_mul(a912, a913)?
                            };
                            let a927 = {
                                let a923 = {
                                    let a917 = v6;
                                    let a918 = {
                                        let m916 = match v0.clone() {
                                            Adt8::C0(_, _, _, _, _, _, _, _, h915, _, _, _, _, _, _) => {
                                                h915
                                            }
                                        };
                                        match m916 {
                                            Adt3::C0(h914, _, _, _, _, _, _, _) => {
                                                h914
                                            }
                                        }
                                    };
                                    let a919 = 0u64;
                                    nat_quot(a917, a918, a919)
                                };
                                let a924 = {
                                    let m922 = match v0.clone() {
                                        Adt8::C0(_, _, _, _, _, _, _, _, h921, _, _, _, _, _, _) => {
                                            h921
                                        }
                                    };
                                    match m922 {
                                        Adt3::C0(_, h920, _, _, _, _, _, _) => {
                                            h920
                                        }
                                    }
                                };
                                let a925 = 0u64;
                                nat_rem(a923, a924, a925)
                            };
                            nat_add(a926, a927)?
                        }, {
                            let a937 = {
                                let a931 = v6;
                                let a932 = {
                                    let m930 = match v0.clone() {
                                        Adt8::C0(_, _, _, _, _, _, _, _, h929, _, _, _, _, _, _) => {
                                            h929
                                        }
                                    };
                                    match m930 {
                                        Adt3::C0(h928, _, _, _, _, _, _, _) => {
                                            h928
                                        }
                                    }
                                };
                                let a933 = 0u64;
                                nat_quot(a931, a932, a933)
                            };
                            let a938 = {
                                let m936 = match v0 {
                                    Adt8::C0(_, _, _, _, _, _, _, _, h935, _, _, _, _, _, _) => {
                                        h935
                                    }
                                };
                                match m936 {
                                    Adt3::C0(_, h934, _, _, _, _, _, _) => {
                                        h934
                                    }
                                }
                            };
                            let a939 = 0u64;
                            nat_quot(a937, a938, a939)
                        }, v11)?;
                        let a941 = 4u64;
                        let a942 = 0u64;
                        nat_quot(a940, a941, a942)
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

pub fn f18(v0: Adt8, v1: u64, v2: u64, v3: u64) -> R<Adt9> {
    let a943 = v1;
    let a944 = 0u64;
    if nat_eq(a943, a944) {
        let a946 = v3;
        let a947 = match v0.clone() {
            Adt8::C0(_, _, _, h945, _, _, _, _, _, _, _, _, _, _, _) => {
                h945
            }
        };
        if nat_lt(a946, a947) {
            let v4: Adt3 = match v0.clone() {
                Adt8::C0(_, _, _, _, _, _, _, _, h948, _, _, _, _, _, _) => {
                    h948
                }
            };
            let v5: u64 = {
                let a950 = v3;
                let a951 = match v0.clone() {
                    Adt8::C0(_, _, h949, _, _, _, _, _, _, _, _, _, _, _, _) => {
                        h949
                    }
                };
                let a952 = 0u64;
                nat_quot(a950, a951, a952)
            };
            Ok(Adt9::C8(Some(f62(v4.clone(), f63(v5, v4.clone(), 0u64), f63(v5, v4.clone(), 1u64), f63(v5, v4, 2u64), f64(v0.clone(), v5, {
                let a954 = v3;
                let a955 = match v0 {
                    Adt8::C0(_, _, h953, _, _, _, _, _, _, _, _, _, _, _, _) => {
                        h953
                    }
                };
                let a956 = 0u64;
                nat_rem(a954, a955, a956)
            })?)?)))
        } else {
            Ok(Adt9::C0(Adt10::C1))
        }
    } else {
        let v6: Adt3 = match v0.clone() {
            Adt8::C0(_, _, _, _, _, _, _, _, _, _, h957, _, _, _, _) => {
                h957
            }
        };
        let v7: u64 = {
            let a958 = v1;
            let a959 = 1u64;
            if nat_eq(a958, a959) {
                match v0.clone() {
                    Adt8::C0(_, h960, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                        h960
                    }
                }
            } else {
                match v0.clone() {
                    Adt8::C0(_, _, h961, _, _, _, _, _, _, _, _, _, _, _, _) => {
                        h961
                    }
                }
            }
        };
        let v8: u64 = {
            let a965 = v2;
            let a966 = {
                let a962 = v3;
                let a963 = v7;
                let a964 = 0u64;
                nat_quot(a962, a963, a964)
            };
            nat_add(a965, a966)?
        };
        let v9: u64 = {
            let a967 = v3;
            let a968 = v7;
            let a969 = 0u64;
            nat_rem(a967, a968, a969)
        };
        let v10: u64 = {
            let a971 = v8;
            let a972 = match v6.clone() {
                Adt3::C0(h970, _, _, _, _, _, _, _) => {
                    h970
                }
            };
            let a973 = 0u64;
            nat_rem(a971, a972, a973)
        };
        let v11: u64 = {
            let a975 = v8;
            let a976 = match v6.clone() {
                Adt3::C0(h974, _, _, _, _, _, _, _) => {
                    h974
                }
            };
            let a977 = 0u64;
            nat_quot(a975, a976, a977)
        };
        let v12: u64 = {
            let a979 = v11;
            let a980 = match v6.clone() {
                Adt3::C0(_, h978, _, _, _, _, _, _) => {
                    h978
                }
            };
            let a981 = 0u64;
            nat_rem(a979, a980, a981)
        };
        let v13: u64 = {
            let a983 = v11;
            let a984 = match v6.clone() {
                Adt3::C0(_, h982, _, _, _, _, _, _) => {
                    h982
                }
            };
            let a985 = 0u64;
            nat_quot(a983, a984, a985)
        };
        let a987 = v8;
        let a988 = match v0.clone() {
            Adt8::C0(h986, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h986
            }
        };
        if nat_lt(a987, a988) {
            let a989 = v1;
            let a990 = 1u64;
            if nat_eq(a989, a990) {
                let v14: Adt3 = match v0.clone() {
                    Adt8::C0(_, _, _, _, _, _, _, _, h991, _, _, _, _, _, _) => {
                        h991
                    }
                };
                let v15: Adt3 = match v0.clone() {
                    Adt8::C0(_, _, _, _, _, _, _, _, _, h992, _, _, _, _, _) => {
                        h992
                    }
                };
                let m995 = f65(match v0.clone() {
                    Adt8::C0(_, _, _, _, _, _, _, _, _, _, _, h993, _, _, _) => {
                        h993
                    }
                }, match v15.clone() {
                    Adt3::C0(h994, _, _, _, _, _, _, _) => {
                        h994
                    }
                }, v10, f63(v9, v14.clone(), 0u64))?;
                match m995 {
                    None => {
                        Ok(Adt9::C8(None::<u64>))
                    }
                    Some(v16) => {
                        let m998 = f65(match v0.clone() {
                            Adt8::C0(_, _, _, _, _, _, _, _, _, _, _, _, h996, _, _) => {
                                h996
                            }
                        }, match v15 {
                            Adt3::C0(_, h997, _, _, _, _, _, _) => {
                                h997
                            }
                        }, v12, f63(v9, v14, 1u64))?;
                        match m998 {
                            None => {
                                Ok(Adt9::C8(None::<u64>))
                            }
                            Some(v17) => {
                                f66(v0, v9, v13, v16, v17)
                            }
                        }
                    }
                }
            } else {
                let a999 = v1;
                let a1000 = 2u64;
                if nat_eq(a999, a1000) {
                    Ok(Adt9::C8(Some({
                        let a1002 = match v0.clone() {
                            Adt8::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, h1001) => {
                                h1001
                            }
                        };
                        let a1003 = 0u64;
                        if nat_eq(a1002, a1003) {
                            f62(v6, v10, v12, v9, v13)?
                        } else {
                            f62(v6.clone(), v10, v12, {
                                let a1005 = v13;
                                let a1006 = match v6.clone() {
                                    Adt3::C0(_, _, h1004, _, _, _, _, _) => {
                                        h1004
                                    }
                                };
                                let a1007 = 0u64;
                                nat_rem(a1005, a1006, a1007)
                            }, {
                                let a1015 = {
                                    let a1013 = {
                                        let a1009 = v13;
                                        let a1010 = match v6 {
                                            Adt3::C0(_, _, h1008, _, _, _, _, _) => {
                                                h1008
                                            }
                                        };
                                        let a1011 = 0u64;
                                        nat_quot(a1009, a1010, a1011)
                                    };
                                    let a1014 = match v0 {
                                        Adt8::C0(_, _, h1012, _, _, _, _, _, _, _, _, _, _, _, _) => {
                                            h1012
                                        }
                                    };
                                    nat_mul(a1013, a1014)?
                                };
                                let a1016 = v9;
                                nat_add(a1015, a1016)?
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

pub fn f19(v0: u64, v1: Adt3, v2: Adt3, v3: Adt4, v4: u64) -> R<Adt9> {
    f20(v0, v1, v2, v3, v4, false, 0u64, 0u64, 0u64, 0u64, 0u64)
}

pub fn f20(v0: u64, v1: Adt3, v2: Adt3, v3: Adt4, v4: u64, v5: bool, v6: u64, v7: u64, v8: u64, v9: u64, v10: u64) -> R<Adt9> {
    Ok(f67(v3.clone(), {
        let m1017 = v3.clone();
        match m1017 {
            Adt4::C0 => {
                f68(v0, v1, v2, v3)?
            }
            Adt4::C1 => {
                f68(v0, v1, v2, v3)?
            }
            Adt4::C2(_, _) => {
                f68(v0, v1, v2, v3)?
            }
            Adt4::C3(_, _, _, _, _) => {
                f68(v0, v1, v2, v3)?
            }
            Adt4::C4(_) => {
                f68(v0, v1, v2, v3)?
            }
            Adt4::C5(_, _, _, _) => {
                f68(v0, v1, v2, v3)?
            }
            Adt4::C6(_, _, _) => {
                f68(v0, v1, v2, v3)?
            }
            Adt4::C7(_) => {
                f68(v0, v1, v2, v3)?
            }
            Adt4::C8(_) => {
                f68(v0, v1, v2, v3)?
            }
            Adt4::C9(_, _) => {
                f68(v0, v1, v2, v3)?
            }
            Adt4::C10(_, _, _) => {
                f68(v0, v1, v2, v3)?
            }
            Adt4::C11(_, _) => {
                f68(v0, v1, v2, v3)?
            }
            Adt4::C12(_, _, _) => {
                f68(v0, v1, v2, v3)?
            }
            Adt4::C13(_, _, _, _) => {
                f68(v0, v1, v2, v3)?
            }
            Adt4::C14(_, _, _) => {
                f68(v0, v1, v2, v3)?
            }
            Adt4::C15 => {
                f68(v0, v1, v2, v3)?
            }
            Adt4::C16(_, _) => {
                f68(v0, v1, v2, v3)?
            }
            Adt4::C17 => {
                f86(v0, v1, v2, v4)?
            }
            Adt4::C18(_, _) => {
                f68(v0, v1, v2, v3)?
            }
            Adt4::C19(v85, v86) => {
                f87(v85, v0, v1, v2, v86, v4)?
            }
            Adt4::C20(v87, v88) => {
                f88(v87, v0, v1, v2, v88, v4)?
            }
            Adt4::C21 => {
                f68(v0, v1, v2, v3)?
            }
            Adt4::C22(v89, v90) => {
                f89(v0, v1, v2, v89, v90, v4)?
            }
            Adt4::C23(v104, v105, v106, v107) => {
                f96(v0, v107, v104, v105, v106, v4)?
            }
            Adt4::C24(v103) => {
                if v5 {
                    f94(Adt16::C0(v103), v0, v1, v2, v4, v6, v7, v8, v9, v10)?
                } else {
                    Adt9::C0(Adt10::C5(65u64))
                }
            }
            Adt4::C25(_, _, _) => {
                f68(v0, v1, v2, v3)?
            }
            Adt4::C26(v99, v100, v101, v102) => {
                let a1019 = v99;
                let a1020 = 64u64;
                if if if nat_eq(a1019, a1020) {
                    true
                } else {
                    let a1021 = v99;
                    let a1022 = 65u64;
                    nat_eq(a1021, a1022)
                } {
                    true
                } else {
                    let a1023 = v99;
                    let a1024 = 66u64;
                    nat_eq(a1023, a1024)
                } {
                    if v5 {
                        f94(Adt16::C1(v99, v100, v101, v102), v0, v1, v2, v4, v6, v7, v8, v9, v10)?
                    } else {
                        Adt9::C0(Adt10::C5(64u64))
                    }
                } else {
                    let a1025 = v99;
                    let a1026 = 0u64;
                    if if nat_eq(a1025, a1026) {
                        true
                    } else {
                        let a1027 = v99;
                        let a1028 = 30u64;
                        nat_eq(a1027, a1028)
                    } {
                        f95(v0, v1, v2, v99, v100, v101, v102, v4)?
                    } else {
                        Adt9::C0(Adt10::C1)
                    }
                }
            }
            Adt4::C27(v98) => {
                f93(v0, v1, v2, v98)?
            }
            Adt4::C28(v94) => {
                f90(v0, v1, v2, v94, v4)?
            }
            Adt4::C29(v95) => {
                f91(v0, v1, v2, v95)?
            }
            Adt4::C30(v96, _) => {
                f92(v0, v1, v2, v96)?
            }
            Adt4::C31(v33, v34) => {
                f69(v0, v1, v2, v33, v34)?
            }
            Adt4::C32(v35) => {
                f70(v0, match v1 {
                    Adt3::C0(h1018, _, _, _, _, _, _, _) => {
                        h1018
                    }
                }, v35)?
            }
            Adt4::C33(v36) => {
                f71(v0, v1, v36, v2)?
            }
            Adt4::C34(v65, v66, v67, v68) => {
                f85(v0, v1, v65, v2, v66, v67, v68)?
            }
            Adt4::C35(v64) => {
                f84(v0, v64, v1, v2)?
            }
            Adt4::C36(v37, v38, v39) => {
                f72(v0, v1, v37, v2, v38, v39)?
            }
            Adt4::C37(v40, v41, v42) => {
                f73(v0, v1, v40, v41, v42, v2)?
            }
            Adt4::C38(v43, v44, v45, v46) => {
                f74(v0, v1, v43, v44, v2, v45, v46, v4)?
            }
            Adt4::C39(v47) => {
                f75(v0, v1, v47)?
            }
            Adt4::C40(v48, v49, v50) => {
                f76(v0, v1, v2, v48, v49, v50, v4)?
            }
            Adt4::C41 => {
                f83(v0, v1, v2, v4)?
            }
            Adt4::C42(v58, v59) => {
                f81(v0, v1, v2, v58, v59)?
            }
            Adt4::C43(v56, v57) => {
                f80(v0, v1, v2, v56, v57, v4)?
            }
            Adt4::C44(v55) => {
                f79(v0, v2, v55)?
            }
            Adt4::C45(v60, v61, v62, v63) => {
                f82(v0, v1, v2, v60, v61, v62, v63, v4)?
            }
            Adt4::C46(v53, v54) => {
                f78(v0, v1, v53, v54)?
            }
            Adt4::C47(v51, v52) => {
                f77(v0, v1, v51, v52)?
            }
        }
    }))
}

pub fn f21(v0: Adt4) -> R<Adt9> {
    let m1029 = v0;
    match m1029 {
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
        Adt4::C25(v82, _, v84) => {
            Ok(Adt9::C6({
                let a1035 = {
                    let a1031 = 0u64;
                    let a1032 = v82;
                    if nat_lt(a1031, a1032) {
                        1u64
                    } else {
                        0u64
                    }
                };
                let a1036 = {
                    let a1033 = 0u64;
                    let a1034 = v84;
                    if nat_lt(a1033, a1034) {
                        6u64
                    } else {
                        0u64
                    }
                };
                nat_add(a1035, a1036)?
            }, 0u64, 0u64, false, 3u64))
        }
        Adt4::C26(v90, _, _, v93) => {
            Ok(Adt9::C6({
                let a1037 = v90;
                let a1038 = 66u64;
                if nat_eq(a1037, a1038) {
                    3u64
                } else {
                    1u64
                }
            }, 0u64, 0u64, false, v93))
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
        Adt4::C35(v55) => {
            Ok(Adt9::C6(1u64, 0u64, 0u64, false, v55))
        }
        Adt4::C36(_, _, _) => {
            Ok(Adt9::C6(3u64, 0u64, 0u64, false, 3u64))
        }
        Adt4::C37(_, _, v32) => {
            Ok(Adt9::C6({
                let m1030 = v32;
                match m1030 {
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
        Adt4::C47(v42, _) => {
            Ok(Adt9::C6(if v42 {
                127u64
            } else {
                63u64
            }, 0u64, 0u64, false, 3u64))
        }
    }
}

pub fn f22(v0: bool, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64, v7: u64, v8: u64, v9: u64, v10: u64, v11: u64, v12: u64) -> Adt9 {
    let a1039 = v1;
    let a1040 = 5u64;
    if if if if if nat_lt(a1039, a1040) {
        true
    } else {
        f97(v0, v1, v2, v3, v4)
    } {
        let a1041 = v5;
        let a1042 = 5u64;
        nat_lt(a1041, a1042)
    } else {
        false
    } {
        let a1043 = v7;
        let a1044 = 5u64;
        nat_lt(a1043, a1044)
    } else {
        false
    } {
        let a1045 = v9;
        let a1046 = 2u64;
        if nat_lt(a1045, a1046) {
            true
        } else {
            let a1047 = v9;
            let a1048 = 3u64;
            nat_eq(a1047, a1048)
        }
    } else {
        false
    } {
        Adt9::C7(f98(v0, v1, v2, v3, v4), f99(v5, v6), f99(v7, v8), f100({
            let a1049 = v9;
            let a1050 = 0u64;
            if nat_eq(a1049, a1050) {
                1u64
            } else {
                v9
            }
        }, v10), v11, v12)
    } else {
        Adt9::C0(Adt10::C1)
    }
}

pub fn f23(v0: Adt2, v1: Adt2, v2: Adt2, v3: u64) -> R<bool> {
    let a1053 = match v0.clone() {
        Adt2::C0(_, h1051, _, _, _, _, _, _, _, _, _) => {
            h1051
        }
    };
    let a1054 = match v1.clone() {
        Adt2::C0(_, h1052, _, _, _, _, _, _, _, _, _) => {
            h1052
        }
    };
    if if if nat_eq(a1053, a1054) {
        let a1057 = match v0.clone() {
            Adt2::C0(_, _, h1055, _, _, _, _, _, _, _, _) => {
                h1055
            }
        };
        let a1058 = match v1.clone() {
            Adt2::C0(_, _, h1056, _, _, _, _, _, _, _, _) => {
                h1056
            }
        };
        nat_eq(a1057, a1058)
    } else {
        false
    } {
        let a1061 = match v0.clone() {
            Adt2::C0(_, _, _, h1059, _, _, _, _, _, _, _) => {
                h1059
            }
        };
        let a1062 = match v1 {
            Adt2::C0(_, _, _, h1060, _, _, _, _, _, _, _) => {
                h1060
            }
        };
        nat_eq(a1061, a1062)
    } else {
        false
    } {
        f25(v0, v2, v3)
    } else {
        Ok(false)
    }
}

pub fn f24(v0: u64, v1: u64, v2: Adt2, v3: Adt2, v4: Adt2) -> R<Adt10> {
    let v5: u64 = match v2.clone() {
        Adt2::C0(_, h1063, _, _, _, _, _, _, _, _, _) => {
            h1063
        }
    };
    let v6: u64 = match v2.clone() {
        Adt2::C0(_, _, h1064, _, _, _, _, _, _, _, _) => {
            h1064
        }
    };
    let v7: u64 = match v2.clone() {
        Adt2::C0(_, _, _, h1065, _, _, _, _, _, _, _) => {
            h1065
        }
    };
    let v8: u64 = match v3.clone() {
        Adt2::C0(_, _, h1066, _, _, _, _, _, _, _, _) => {
            h1066
        }
    };
    let v9: u64 = {
        let a1067 = v5;
        let a1068 = v6;
        nat_mul(a1067, a1068)?
    };
    let v10: u64 = {
        let a1069 = v9;
        let a1070 = v7;
        nat_mul(a1069, a1070)?
    };
    let v11: u64 = {
        let a1073 = {
            let a1071 = v9;
            let a1072 = v5;
            nat_mul(a1071, a1072)?
        };
        let a1074 = v8;
        nat_mul(a1073, a1074)?
    };
    let a1075 = 0u64;
    let a1076 = v6;
    if if if if if if if if if if if if if if nat_lt(a1075, a1076) {
        let a1077 = 0u64;
        let a1078 = v7;
        nat_lt(a1077, a1078)
    } else {
        false
    } {
        let a1079 = 0u64;
        let a1080 = v8;
        nat_lt(a1079, a1080)
    } else {
        false
    } {
        let a1084 = {
            let a1081 = v7;
            let a1082 = v8;
            let a1083 = 0u64;
            nat_rem(a1081, a1082, a1083)
        };
        let a1085 = 0u64;
        nat_eq(a1084, a1085)
    } else {
        false
    } {
        let a1087 = match v4.clone() {
            Adt2::C0(h1086, _, _, _, _, _, _, _, _, _, _) => {
                h1086
            }
        };
        let a1088 = 0u64;
        nat_eq(a1087, a1088)
    } else {
        false
    } {
        f43(v4.clone())?
    } else {
        false
    } {
        let a1090 = match v4.clone() {
            Adt2::C0(_, h1089, _, _, _, _, _, _, _, _, _) => {
                h1089
            }
        };
        let a1091 = v9;
        nat_eq(a1090, a1091)
    } else {
        false
    } {
        let a1097 = match v4.clone() {
            Adt2::C0(_, _, h1092, _, _, _, _, _, _, _, _) => {
                h1092
            }
        };
        let a1098 = {
            let a1095 = v7;
            let a1096 = {
                let a1093 = v5;
                let a1094 = v8;
                nat_mul(a1093, a1094)?
            };
            nat_add(a1095, a1096)?
        };
        nat_eq(a1097, a1098)
    } else {
        false
    } {
        let a1100 = match v4.clone() {
            Adt2::C0(_, _, _, h1099, _, _, _, _, _, _, _) => {
                h1099
            }
        };
        let a1101 = 1u64;
        nat_eq(a1100, a1101)
    } else {
        false
    } {
        let a1103 = match v4.clone() {
            Adt2::C0(_, _, _, _, h1102, _, _, _, _, _, _) => {
                h1102
            }
        };
        let a1104 = 1u64;
        nat_eq(a1103, a1104)
    } else {
        false
    } {
        let a1105 = f28(v3.clone(), 1u64)?;
        let a1106 = v11;
        nat_eq(a1105, a1106)
    } else {
        false
    } {
        f25(v2.clone(), v4.clone(), v10)?
    } else {
        false
    } {
        f25(v3, v4.clone(), v11)?
    } else {
        false
    } {
        let a1112 = {
            let a1109 = 4u64;
            let a1110 = {
                let a1107 = v10;
                let a1108 = v11;
                nat_add(a1107, a1108)?
            };
            nat_mul(a1109, a1110)?
        };
        let a1113 = match v4.clone() {
            Adt2::C0(_, _, _, _, _, _, _, _, _, h1111, _) => {
                h1111
            }
        };
        nat_le(a1112, a1113)
    } else {
        false
    } {
        let a1114 = v0;
        let a1115 = 1u64;
        if nat_eq(a1114, a1115) {
            Ok(Adt10::C4(f101(v5, v10)?, 4u64, f30(v2, 1u64), f30(v4, 1u64), Adt4::C46(v1, v8)))
        } else {
            Ok(Adt10::C4(f102({
                let a1116 = v0;
                let a1117 = 2u64;
                nat_eq(a1116, a1117)
            }, v5, v10)?, 4u64, f30(v2, 1u64), f30(v4, 1u64), Adt4::C47({
                let a1118 = v0;
                let a1119 = 2u64;
                nat_eq(a1118, a1119)
            }, v8)))
        }
    } else {
        Ok(Adt10::C1)
    }
}

pub fn f25(v0: Adt2, v1: Adt2, v2: u64) -> R<bool> {
    let a1121 = match v0.clone() {
        Adt2::C0(h1120, _, _, _, _, _, _, _, _, _, _) => {
            h1120
        }
    };
    let a1122 = 0u64;
    if if if nat_eq(a1121, a1122) {
        f43(v0.clone())?
    } else {
        false
    } {
        let a1126 = {
            let a1123 = 4u64;
            let a1124 = v2;
            nat_mul(a1123, a1124)?
        };
        let a1127 = match v0.clone() {
            Adt2::C0(_, _, _, _, _, _, _, _, _, h1125, _) => {
                h1125
            }
        };
        nat_le(a1126, a1127)
    } else {
        false
    } {
        f35(v0, v1)
    } else {
        Ok(false)
    }
}

pub fn f26(v0: Adt2, v1: Adt2) -> bool {
    let a1130 = match v0.clone() {
        Adt2::C0(_, h1128, _, _, _, _, _, _, _, _, _) => {
            h1128
        }
    };
    let a1131 = match v1.clone() {
        Adt2::C0(_, h1129, _, _, _, _, _, _, _, _, _) => {
            h1129
        }
    };
    if if if nat_eq(a1130, a1131) {
        let a1134 = match v0.clone() {
            Adt2::C0(_, _, h1132, _, _, _, _, _, _, _, _) => {
                h1132
            }
        };
        let a1135 = match v1.clone() {
            Adt2::C0(_, _, h1133, _, _, _, _, _, _, _, _) => {
                h1133
            }
        };
        nat_eq(a1134, a1135)
    } else {
        false
    } {
        let a1138 = match v0.clone() {
            Adt2::C0(_, _, _, h1136, _, _, _, _, _, _, _) => {
                h1136
            }
        };
        let a1139 = match v1.clone() {
            Adt2::C0(_, _, _, h1137, _, _, _, _, _, _, _) => {
                h1137
            }
        };
        nat_eq(a1138, a1139)
    } else {
        false
    } {
        let a1142 = match v0 {
            Adt2::C0(_, _, _, _, h1140, _, _, _, _, _, _) => {
                h1140
            }
        };
        let a1143 = match v1 {
            Adt2::C0(_, _, _, _, h1141, _, _, _, _, _, _) => {
                h1141
            }
        };
        nat_eq(a1142, a1143)
    } else {
        false
    }
}

pub fn f27(v0: Adt2, v1: u64, v2: bool) -> R<bool> {
    let a1145 = match v0.clone() {
        Adt2::C0(h1144, _, _, _, _, _, _, _, _, _, _) => {
            h1144
        }
    };
    let a1146 = 0u64;
    if if if nat_eq(a1145, a1146) {
        let a1148 = match v0.clone() {
            Adt2::C0(_, _, _, _, _, h1147, _, _, _, _, _) => {
                h1147
            }
        };
        let a1149 = 4u64;
        nat_eq(a1148, a1149)
    } else {
        false
    } {
        if v2 {
            f43(v0.clone())?
        } else {
            true
        }
    } else {
        false
    } {
        let a1161 = {
            let a1158 = {
                let a1153 = {
                    let a1150 = v1;
                    let a1151 = 1u64;
                    nat_sub(a1150, a1151)
                };
                let a1154 = match v0.clone() {
                    Adt2::C0(_, _, _, _, _, _, h1152, _, _, _, _) => {
                        h1152
                    }
                };
                nat_mul(a1153, a1154)?
            };
            let a1159 = {
                let a1156 = 4u64;
                let a1157 = match v0.clone() {
                    Adt2::C0(_, h1155, _, _, _, _, _, _, _, _, _) => {
                        h1155
                    }
                };
                nat_mul(a1156, a1157)?
            };
            nat_add(a1158, a1159)?
        };
        let a1162 = match v0 {
            Adt2::C0(_, _, _, _, _, _, _, _, _, h1160, _) => {
                h1160
            }
        };
        Ok(nat_le(a1161, a1162))
    } else {
        Ok(false)
    }
}

pub fn f28(v0: Adt2, v1: u64) -> R<u64> {
    let a1174 = {
        let a1168 = {
            let a1164 = match v0.clone() {
                Adt2::C0(_, h1163, _, _, _, _, _, _, _, _, _) => {
                    h1163
                }
            };
            let a1165 = v1;
            let a1166 = 0u64;
            nat_quot(a1164, a1165, a1166)
        };
        let a1169 = match v0.clone() {
            Adt2::C0(_, _, h1167, _, _, _, _, _, _, _, _) => {
                h1167
            }
        };
        nat_mul(a1168, a1169)?
    };
    let a1175 = {
        let a1172 = match v0.clone() {
            Adt2::C0(_, _, _, h1170, _, _, _, _, _, _, _) => {
                h1170
            }
        };
        let a1173 = match v0 {
            Adt2::C0(_, _, _, _, h1171, _, _, _, _, _, _) => {
                h1171
            }
        };
        nat_mul(a1172, a1173)?
    };
    nat_mul(a1174, a1175)
}

pub fn f29(v0: bool, v1: u64) -> R<u64> {
    let a1178 = if v0 {
        2u64
    } else {
        4u64
    };
    let a1179 = {
        let a1176 = if v0 {
            9u64
        } else {
            11u64
        };
        let a1177 = v1;
        nat_mul(a1176, a1177)?
    };
    nat_add(a1178, a1179)
}

pub fn f30(v0: Adt2, v1: u64) -> Adt3 {
    Adt3::C0({
        let a1181 = match v0.clone() {
            Adt2::C0(_, h1180, _, _, _, _, _, _, _, _, _) => {
                h1180
            }
        };
        let a1182 = v1;
        let a1183 = 0u64;
        nat_quot(a1181, a1182, a1183)
    }, match v0.clone() {
        Adt2::C0(_, _, h1184, _, _, _, _, _, _, _, _) => {
            h1184
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, h1185, _, _, _, _, _, _, _) => {
            h1185
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, h1186, _, _, _, _, _, _) => {
            h1186
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, _, h1187, _, _, _, _, _) => {
            h1187
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, _, _, h1188, _, _, _, _) => {
            h1188
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, _, _, _, h1189, _, _, _) => {
            h1189
        }
    }, match v0 {
        Adt2::C0(_, _, _, _, _, _, _, _, h1190, _, _) => {
            h1190
        }
    })
}

pub fn f31(v0: Adt2, v1: u64) -> R<bool> {
    let a1191 = v1;
    let a1192 = 0u64;
    if nat_eq(a1191, a1192) {
        Ok(true)
    } else {
        let a1198 = {
            let a1195 = f103({
                let a1193 = v1;
                let a1194 = 1u64;
                nat_sub(a1193, a1194)
            }, f30(v0.clone(), 1u64))?;
            let a1196 = 4u64;
            nat_add(a1195, a1196)?
        };
        let a1199 = match v0 {
            Adt2::C0(_, _, _, _, _, _, _, _, _, h1197, _) => {
                h1197
            }
        };
        Ok(nat_le(a1198, a1199))
    }
}

pub fn f32(v0: Adt2, v1: Adt2) -> R<bool> {
    let a1201 = match v1.clone() {
        Adt2::C0(h1200, _, _, _, _, _, _, _, _, _, _) => {
            h1200
        }
    };
    let a1202 = 0u64;
    if if nat_eq(a1201, a1202) {
        f26(v0.clone(), v1.clone())
    } else {
        false
    } {
        let a1203 = f28(v0.clone(), 1u64)?;
        let a1204 = 0u64;
        if nat_eq(a1203, a1204) {
            Ok(true)
        } else {
            let a1210 = {
                let a1207 = f103({
                    let a1205 = f28(v0.clone(), 1u64)?;
                    let a1206 = 1u64;
                    nat_sub(a1205, a1206)
                }, f30(v0, 1u64))?;
                let a1208 = 4u64;
                nat_add(a1207, a1208)?
            };
            let a1211 = match v1 {
                Adt2::C0(_, _, _, _, _, _, _, _, _, h1209, _) => {
                    h1209
                }
            };
            Ok(nat_le(a1210, a1211))
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
    let a1213 = match v0.clone() {
        Adt2::C0(_, _, _, _, _, _, _, _, _, h1212, _) => {
            h1212
        }
    };
    let a1214 = 0u64;
    if nat_eq(a1213, a1214) {
        Ok(true)
    } else {
        let a1216 = match v1.clone() {
            Adt2::C0(_, _, _, _, _, _, _, _, _, h1215, _) => {
                h1215
            }
        };
        let a1217 = 0u64;
        if nat_eq(a1216, a1217) {
            Ok(true)
        } else {
            let a1219 = match v0.clone() {
                Adt2::C0(_, _, _, _, _, _, _, _, _, _, h1218) => {
                    h1218
                }
            };
            let a1220 = 0u64;
            if nat_eq(a1219, a1220) {
                Ok(true)
            } else {
                let a1222 = match v1.clone() {
                    Adt2::C0(_, _, _, _, _, _, _, _, _, _, h1221) => {
                        h1221
                    }
                };
                let a1223 = 0u64;
                if nat_eq(a1222, a1223) {
                    Ok(true)
                } else {
                    let a1229 = {
                        let a1226 = match v0.clone() {
                            Adt2::C0(_, _, _, _, _, _, _, _, _, _, h1224) => {
                                h1224
                            }
                        };
                        let a1227 = match v0.clone() {
                            Adt2::C0(_, _, _, _, _, _, _, _, _, h1225, _) => {
                                h1225
                            }
                        };
                        nat_add(a1226, a1227)?
                    };
                    let a1230 = match v1.clone() {
                        Adt2::C0(_, _, _, _, _, _, _, _, _, _, h1228) => {
                            h1228
                        }
                    };
                    if nat_le(a1229, a1230) {
                        Ok(true)
                    } else {
                        let a1236 = {
                            let a1233 = match v1.clone() {
                                Adt2::C0(_, _, _, _, _, _, _, _, _, _, h1231) => {
                                    h1231
                                }
                            };
                            let a1234 = match v1 {
                                Adt2::C0(_, _, _, _, _, _, _, _, _, h1232, _) => {
                                    h1232
                                }
                            };
                            nat_add(a1233, a1234)?
                        };
                        let a1237 = match v0 {
                            Adt2::C0(_, _, _, _, _, _, _, _, _, _, h1235) => {
                                h1235
                            }
                        };
                        Ok(nat_le(a1236, a1237))
                    }
                }
            }
        }
    }
}

pub fn f36(v0: Adt2) -> R<bool> {
    let v1: u64 = {
        let a1243 = match v0.clone() {
            Adt2::C0(_, _, h1238, _, _, _, _, _, _, _, _) => {
                h1238
            }
        };
        let a1244 = {
            let a1241 = match v0.clone() {
                Adt2::C0(_, _, _, h1239, _, _, _, _, _, _, _) => {
                    h1239
                }
            };
            let a1242 = match v0.clone() {
                Adt2::C0(_, _, _, _, h1240, _, _, _, _, _, _) => {
                    h1240
                }
            };
            nat_mul(a1241, a1242)?
        };
        nat_mul(a1243, a1244)?
    };
    let v2: u64 = f28(v0.clone(), 1u64)?;
    let a1246 = match v0.clone() {
        Adt2::C0(h1245, _, _, _, _, _, _, _, _, _, _) => {
            h1245
        }
    };
    let a1247 = 0u64;
    if if if if if if if nat_eq(a1246, a1247) {
        let a1249 = match v0.clone() {
            Adt2::C0(_, h1248, _, _, _, _, _, _, _, _, _) => {
                h1248
            }
        };
        let a1250 = 2147483647u64;
        nat_le(a1249, a1250)
    } else {
        false
    } {
        let a1251 = v1;
        let a1252 = 2147483647u64;
        nat_le(a1251, a1252)
    } else {
        false
    } {
        let a1254 = match v0.clone() {
            Adt2::C0(_, h1253, _, _, _, _, _, _, _, _, _) => {
                h1253
            }
        };
        let a1255 = 1u64;
        if nat_eq(a1254, a1255) {
            true
        } else {
            let a1257 = match v0.clone() {
                Adt2::C0(_, _, _, _, _, h1256, _, _, _, _, _) => {
                    h1256
                }
            };
            let a1258 = 4u64;
            nat_eq(a1257, a1258)
        }
    } else {
        false
    } {
        let a1259 = v1;
        let a1260 = 1u64;
        if nat_le(a1259, a1260) {
            true
        } else {
            let a1265 = {
                let a1262 = match v0.clone() {
                    Adt2::C0(_, h1261, _, _, _, _, _, _, _, _, _) => {
                        h1261
                    }
                };
                let a1263 = 4u64;
                nat_mul(a1262, a1263)?
            };
            let a1266 = match v0.clone() {
                Adt2::C0(_, _, _, _, _, _, h1264, _, _, _, _) => {
                    h1264
                }
            };
            nat_le(a1265, a1266)
        }
    } else {
        false
    } {
        let a1268 = match v0.clone() {
            Adt2::C0(_, _, _, h1267, _, _, _, _, _, _, _) => {
                h1267
            }
        };
        let a1269 = 1u64;
        if nat_eq(a1268, a1269) {
            true
        } else {
            let a1275 = match v0.clone() {
                Adt2::C0(_, _, _, _, _, _, _, h1270, _, _, _) => {
                    h1270
                }
            };
            let a1276 = {
                let a1273 = match v0.clone() {
                    Adt2::C0(_, _, h1271, _, _, _, _, _, _, _, _) => {
                        h1271
                    }
                };
                let a1274 = match v0.clone() {
                    Adt2::C0(_, _, _, _, _, _, h1272, _, _, _, _) => {
                        h1272
                    }
                };
                nat_mul(a1273, a1274)?
            };
            nat_eq(a1275, a1276)
        }
    } else {
        false
    } {
        let a1278 = match v0.clone() {
            Adt2::C0(_, _, _, _, h1277, _, _, _, _, _, _) => {
                h1277
            }
        };
        let a1279 = 1u64;
        if nat_eq(a1278, a1279) {
            true
        } else {
            let a1288 = match v0.clone() {
                Adt2::C0(_, _, _, _, _, _, _, _, h1280, _, _) => {
                    h1280
                }
            };
            let a1289 = {
                let a1286 = match v0.clone() {
                    Adt2::C0(_, _, _, h1281, _, _, _, _, _, _, _) => {
                        h1281
                    }
                };
                let a1287 = {
                    let a1284 = match v0.clone() {
                        Adt2::C0(_, _, h1282, _, _, _, _, _, _, _, _) => {
                            h1282
                        }
                    };
                    let a1285 = match v0.clone() {
                        Adt2::C0(_, _, _, _, _, _, h1283, _, _, _, _) => {
                            h1283
                        }
                    };
                    nat_mul(a1284, a1285)?
                };
                nat_mul(a1286, a1287)?
            };
            nat_eq(a1288, a1289)
        }
    } else {
        false
    } {
        let a1290 = v2;
        let a1291 = 0u64;
        if nat_eq(a1290, a1291) {
            Ok(true)
        } else {
            let a1297 = {
                let a1294 = f103({
                    let a1292 = v2;
                    let a1293 = 1u64;
                    nat_sub(a1292, a1293)
                }, f30(v0.clone(), 1u64))?;
                let a1295 = 4u64;
                nat_add(a1294, a1295)?
            };
            let a1298 = match v0 {
                Adt2::C0(_, _, _, _, _, _, _, _, _, h1296, _) => {
                    h1296
                }
            };
            Ok(nat_le(a1297, a1298))
        }
    } else {
        Ok(false)
    }
}

pub fn f37(v0: u64) -> R<u64> {
    let a1305 = 148u64;
    let a1306 = {
        let a1303 = 36u64;
        let a1304 = {
            let a1301 = {
                let a1299 = 2u64;
                let a1300 = v0;
                nat_mul(a1299, a1300)?
            };
            let a1302 = 1u64;
            nat_sub(a1301, a1302)
        };
        nat_mul(a1303, a1304)?
    };
    nat_add(a1305, a1306)
}

pub fn f38(v0: u64, v1: Option<Adt3>) -> R<u64> {
    let m1307 = v1;
    match m1307 {
        None => {
            Ok(2u64)
        }
        Some(_) => {
            let a1310 = 1u64;
            let a1311 = {
                let a1308 = 2u64;
                let a1309 = v0;
                nat_mul(a1308, a1309)?
            };
            nat_add(a1310, a1311)
        }
    }
}

pub fn f39(v0: u64) -> bool {
    let a1312 = v0;
    let a1313 = 4294967296u64;
    if nat_lt(a1312, a1313) {
        let a1317 = {
            let a1314 = v0;
            let a1315 = 2147483648u64;
            let a1316 = 0u64;
            nat_rem(a1314, a1315, a1316)
        };
        let a1318 = 2139095040u64;
        nat_lt(a1317, a1318)
    } else {
        false
    }
}

pub fn f40(v0: u64) -> R<u64> {
    let a1319 = 4u64;
    let a1320 = v0;
    nat_mul(a1319, a1320)
}

pub fn f41(v0: u64) -> R<u64> {
    let a1321 = v0;
    let a1322 = 0u64;
    if nat_eq(a1321, a1322) {
        Ok(0u64)
    } else {
        let a1325 = 4u64;
        let a1326 = {
            let a1323 = 3u64;
            let a1324 = v0;
            nat_mul(a1323, a1324)?
        };
        nat_add(a1325, a1326)
    }
}

pub fn f42(v0: bool, v1: Adt2) -> Adt3 {
    if v0 {
        f30(v1, 1u64)
    } else {
        Adt3::C0(match v1.clone() {
            Adt2::C0(_, h1327, _, _, _, _, _, _, _, _, _) => {
                h1327
            }
        }, 1u64, match v1.clone() {
            Adt2::C0(_, _, h1328, _, _, _, _, _, _, _, _) => {
                h1328
            }
        }, match v1.clone() {
            Adt2::C0(_, _, _, h1329, _, _, _, _, _, _, _) => {
                h1329
            }
        }, match v1.clone() {
            Adt2::C0(_, _, _, _, _, h1330, _, _, _, _, _) => {
                h1330
            }
        }, 0u64, match v1.clone() {
            Adt2::C0(_, _, _, _, _, _, h1331, _, _, _, _) => {
                h1331
            }
        }, match v1 {
            Adt2::C0(_, _, _, _, _, _, _, h1332, _, _, _) => {
                h1332
            }
        })
    }
}

pub fn f43(v0: Adt2) -> R<bool> {
    let a1334 = match v0.clone() {
        Adt2::C0(_, h1333, _, _, _, _, _, _, _, _, _) => {
            h1333
        }
    };
    let a1335 = 1u64;
    if if if if nat_eq(a1334, a1335) {
        true
    } else {
        let a1337 = match v0.clone() {
            Adt2::C0(_, _, _, _, _, h1336, _, _, _, _, _) => {
                h1336
            }
        };
        let a1338 = 4u64;
        nat_eq(a1337, a1338)
    } {
        let a1340 = match v0.clone() {
            Adt2::C0(_, _, h1339, _, _, _, _, _, _, _, _) => {
                h1339
            }
        };
        let a1341 = 1u64;
        if nat_eq(a1340, a1341) {
            true
        } else {
            let a1346 = match v0.clone() {
                Adt2::C0(_, _, _, _, _, _, h1342, _, _, _, _) => {
                    h1342
                }
            };
            let a1347 = {
                let a1344 = match v0.clone() {
                    Adt2::C0(_, h1343, _, _, _, _, _, _, _, _, _) => {
                        h1343
                    }
                };
                let a1345 = 4u64;
                nat_mul(a1344, a1345)?
            };
            nat_eq(a1346, a1347)
        }
    } else {
        false
    } {
        let a1349 = match v0.clone() {
            Adt2::C0(_, _, _, h1348, _, _, _, _, _, _, _) => {
                h1348
            }
        };
        let a1350 = 1u64;
        if nat_eq(a1349, a1350) {
            true
        } else {
            let a1358 = match v0.clone() {
                Adt2::C0(_, _, _, _, _, _, _, h1351, _, _, _) => {
                    h1351
                }
            };
            let a1359 = {
                let a1356 = {
                    let a1354 = match v0.clone() {
                        Adt2::C0(_, h1352, _, _, _, _, _, _, _, _, _) => {
                            h1352
                        }
                    };
                    let a1355 = match v0.clone() {
                        Adt2::C0(_, _, h1353, _, _, _, _, _, _, _, _) => {
                            h1353
                        }
                    };
                    nat_mul(a1354, a1355)?
                };
                let a1357 = 4u64;
                nat_mul(a1356, a1357)?
            };
            nat_eq(a1358, a1359)
        }
    } else {
        false
    } {
        let a1361 = match v0.clone() {
            Adt2::C0(_, _, _, _, h1360, _, _, _, _, _, _) => {
                h1360
            }
        };
        let a1362 = 1u64;
        if nat_eq(a1361, a1362) {
            Ok(true)
        } else {
            let a1373 = match v0.clone() {
                Adt2::C0(_, _, _, _, _, _, _, _, h1363, _, _) => {
                    h1363
                }
            };
            let a1374 = {
                let a1371 = {
                    let a1369 = {
                        let a1366 = match v0.clone() {
                            Adt2::C0(_, h1364, _, _, _, _, _, _, _, _, _) => {
                                h1364
                            }
                        };
                        let a1367 = match v0.clone() {
                            Adt2::C0(_, _, h1365, _, _, _, _, _, _, _, _) => {
                                h1365
                            }
                        };
                        nat_mul(a1366, a1367)?
                    };
                    let a1370 = match v0 {
                        Adt2::C0(_, _, _, h1368, _, _, _, _, _, _, _) => {
                            h1368
                        }
                    };
                    nat_mul(a1369, a1370)?
                };
                let a1372 = 4u64;
                nat_mul(a1371, a1372)?
            };
            Ok(nat_eq(a1373, a1374))
        }
    } else {
        Ok(false)
    }
}

pub fn f44(v0: Adt6, v1: u64) -> R<u64> {
    let a1376 = match v0.clone() {
        Adt6::C0(h1375, _, _, _) => {
            h1375
        }
    };
    let a1377 = 0u64;
    if nat_eq(a1376, a1377) {
        f104(Adt6::C0(1u64, match v0.clone() {
            Adt6::C0(_, h1378, _, _) => {
                h1378
            }
        }, match v0.clone() {
            Adt6::C0(_, _, h1379, _) => {
                h1379
            }
        }, match v0.clone() {
            Adt6::C0(_, _, _, h1380) => {
                h1380
            }
        }), {
            let a1382 = v1;
            let a1383 = match v0 {
                Adt6::C0(_, _, _, h1381) => {
                    h1381
                }
            };
            nat_add(a1382, a1383)?
        })
    } else {
        f104(v0, v1)
    }
}

pub fn f45(v0: u64, v1: u64, v2: u64, v3: u64) -> R<u64> {
    let a1388 = {
        let a1384 = v0;
        let a1385 = v1;
        nat_mul(a1384, a1385)?
    };
    let a1389 = {
        let a1386 = v2;
        let a1387 = f105(v3);
        nat_mul(a1386, a1387)?
    };
    nat_mul(a1388, a1389)
}

pub fn f46(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let a1392 = v0;
    let a1393 = {
        let a1390 = f106(v1)?;
        let a1391 = v2;
        nat_mul(a1390, a1391)?
    };
    nat_add(a1392, a1393)
}

pub fn f47(v0: Adt2) -> Adt14 {
    Adt14::C0(match v0.clone() {
        Adt2::C0(h1394, _, _, _, _, _, _, _, _, _, _) => {
            h1394
        }
    }, match v0.clone() {
        Adt2::C0(_, h1395, _, _, _, _, _, _, _, _, _) => {
            h1395
        }
    }, match v0.clone() {
        Adt2::C0(_, _, h1396, _, _, _, _, _, _, _, _) => {
            h1396
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, h1397, _, _, _, _, _, _, _) => {
            h1397
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, h1398, _, _, _, _, _, _) => {
            h1398
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, _, h1399, _, _, _, _, _) => {
            h1399
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, _, _, h1400, _, _, _, _) => {
            h1400
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, _, _, _, h1401, _, _, _) => {
            h1401
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, _, _, _, _, h1402, _, _) => {
            h1402
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, _, _, _, _, _, h1403, _) => {
            h1403
        }
    }, match v0 {
        Adt2::C0(_, _, _, _, _, _, _, _, _, _, h1404) => {
            h1404
        }
    })
}

pub fn f48(v0: u64, v1: u64, v2: u64, v3: bool, v4: Adt14, v5: Adt14, v6: Adt14) -> R<Adt15> {
    let a1405 = v0;
    let a1406 = 23u64;
    if nat_eq(a1405, a1406) {
        f107(v1, v4, v6)
    } else {
        let a1407 = v0;
        let a1408 = 26u64;
        if nat_eq(a1407, a1408) {
            f108(v1, v2, v4, v6)
        } else {
            let a1409 = v0;
            let a1410 = 25u64;
            if nat_eq(a1409, a1410) {
                f109(v1, v3, v4, v5, v6)
            } else {
                Ok(Adt15::C0)
            }
        }
    }
}

pub fn f49(v0: u64) -> u64 {
    let a1411 = v0;
    let a1412 = 30u64;
    if nat_eq(a1411, a1412) {
        2u64
    } else {
        4u64
    }
}

pub fn f50(v0: u64) -> bool {
    let a1413 = v0;
    let a1414 = 2139095040u64;
    if nat_le(a1413, a1414) {
        true
    } else {
        let a1415 = v0;
        let a1416 = 2147483648u64;
        nat_eq(a1415, a1416)
    }
}

pub fn f51(v0: Adt2, v1: u64) -> bool {
    let a1418 = match v0.clone() {
        Adt2::C0(_, _, _, _, _, h1417, _, _, _, _, _) => {
            h1417
        }
    };
    let a1419 = v1;
    if if nat_eq(a1418, a1419) {
        let a1424 = {
            let a1421 = match v0.clone() {
                Adt2::C0(_, _, _, _, _, _, _, _, _, _, h1420) => {
                    h1420
                }
            };
            let a1422 = v1;
            let a1423 = 0u64;
            nat_rem(a1421, a1422, a1423)
        };
        let a1425 = 0u64;
        nat_eq(a1424, a1425)
    } else {
        false
    } {
        f110(match v0.clone() {
            Adt2::C0(_, _, h1426, _, _, _, _, _, _, _, _) => {
                h1426
            }
        }, match v0.clone() {
            Adt2::C0(_, _, _, h1427, _, _, _, _, _, _, _) => {
                h1427
            }
        }, match v0.clone() {
            Adt2::C0(_, _, _, _, h1428, _, _, _, _, _, _) => {
                h1428
            }
        }, match v0.clone() {
            Adt2::C0(_, _, _, _, _, _, h1429, _, _, _, _) => {
                h1429
            }
        }, match v0.clone() {
            Adt2::C0(_, _, _, _, _, _, _, h1430, _, _, _) => {
                h1430
            }
        }, match v0 {
            Adt2::C0(_, _, _, _, _, _, _, _, h1431, _, _) => {
                h1431
            }
        }, v1)
    } else {
        false
    }
}

pub fn f52(v0: Adt2, v1: Adt2) -> bool {
    if if if if if if f26(v0.clone(), v1.clone()) {
        let a1434 = match v0.clone() {
            Adt2::C0(h1432, _, _, _, _, _, _, _, _, _, _) => {
                h1432
            }
        };
        let a1435 = match v1.clone() {
            Adt2::C0(h1433, _, _, _, _, _, _, _, _, _, _) => {
                h1433
            }
        };
        nat_eq(a1434, a1435)
    } else {
        false
    } {
        let a1438 = match v0.clone() {
            Adt2::C0(_, _, _, _, _, _, _, _, _, _, h1436) => {
                h1436
            }
        };
        let a1439 = match v1.clone() {
            Adt2::C0(_, _, _, _, _, _, _, _, _, _, h1437) => {
                h1437
            }
        };
        nat_eq(a1438, a1439)
    } else {
        false
    } {
        let a1442 = match v0.clone() {
            Adt2::C0(_, _, _, _, _, h1440, _, _, _, _, _) => {
                h1440
            }
        };
        let a1443 = match v1.clone() {
            Adt2::C0(_, _, _, _, _, h1441, _, _, _, _, _) => {
                h1441
            }
        };
        nat_eq(a1442, a1443)
    } else {
        false
    } {
        let a1446 = match v0.clone() {
            Adt2::C0(_, _, _, _, _, _, h1444, _, _, _, _) => {
                h1444
            }
        };
        let a1447 = match v1.clone() {
            Adt2::C0(_, _, _, _, _, _, h1445, _, _, _, _) => {
                h1445
            }
        };
        nat_eq(a1446, a1447)
    } else {
        false
    } {
        let a1450 = match v0.clone() {
            Adt2::C0(_, _, _, _, _, _, _, h1448, _, _, _) => {
                h1448
            }
        };
        let a1451 = match v1.clone() {
            Adt2::C0(_, _, _, _, _, _, _, h1449, _, _, _) => {
                h1449
            }
        };
        nat_eq(a1450, a1451)
    } else {
        false
    } {
        let a1454 = match v0 {
            Adt2::C0(_, _, _, _, _, _, _, _, h1452, _, _) => {
                h1452
            }
        };
        let a1455 = match v1 {
            Adt2::C0(_, _, _, _, _, _, _, _, h1453, _, _) => {
                h1453
            }
        };
        nat_eq(a1454, a1455)
    } else {
        false
    }
}

pub fn f53(v0: u64, v1: u64, v2: u64, v3: u64) -> bool {
    let a1456 = v0;
    let a1457 = 0u64;
    if if if nat_eq(a1456, a1457) {
        true
    } else {
        let a1458 = v1;
        let a1459 = 0u64;
        nat_eq(a1458, a1459)
    } {
        true
    } else {
        let a1460 = v2;
        let a1461 = 0u64;
        nat_eq(a1460, a1461)
    } {
        true
    } else {
        let a1462 = v3;
        let a1463 = 0u64;
        nat_eq(a1462, a1463)
    }
}

pub fn f54(v0: Adt2, v1: u64) -> R<bool> {
    f111({
        let a1465 = match v0.clone() {
            Adt2::C0(_, h1464, _, _, _, _, _, _, _, _, _) => {
                h1464
            }
        };
        let a1466 = v1;
        nat_mul(a1465, a1466)?
    }, match v0.clone() {
        Adt2::C0(_, _, h1467, _, _, _, _, _, _, _, _) => {
            h1467
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, h1468, _, _, _, _, _, _, _) => {
            h1468
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, h1469, _, _, _, _, _, _) => {
            h1469
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, _, _, h1470, _, _, _, _) => {
            h1470
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, _, _, _, h1471, _, _, _) => {
            h1471
        }
    }, match v0 {
        Adt2::C0(_, _, _, _, _, _, _, _, h1472, _, _) => {
            h1472
        }
    })
}

pub fn f55(v0: u64, v1: u64, v2: u64, v3: u64) -> R<u64> {
    let a1473 = v1;
    let a1474 = 0u64;
    if if nat_eq(a1473, a1474) {
        true
    } else {
        let a1475 = v2;
        let a1476 = 0u64;
        nat_eq(a1475, a1476)
    } {
        Ok(0u64)
    } else {
        let a1483 = {
            let a1477 = f112(v1, v2, v3)?;
            let a1478 = f113(v0, v1, v2)?;
            nat_add(a1477, a1478)?
        };
        let a1484 = {
            let a1481 = f114(v0);
            let a1482 = {
                let a1479 = v1;
                let a1480 = v2;
                nat_mul(a1479, a1480)?
            };
            nat_mul(a1481, a1482)?
        };
        nat_add(a1483, a1484)
    }
}

pub fn f56(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let a1485 = v0;
    let a1486 = 0u64;
    if if nat_eq(a1485, a1486) {
        true
    } else {
        let a1487 = v1;
        let a1488 = 0u64;
        nat_eq(a1487, a1488)
    } {
        Ok(3u64)
    } else {
        let a1491 = {
            let a1489 = 3u64;
            let a1490 = v1;
            nat_add(a1489, a1490)?
        };
        let a1492 = v2;
        nat_add(a1491, a1492)
    }
}

pub fn f57(v0: u64) -> bool {
    let a1493 = v0;
    let a1494 = 2139095040u64;
    if nat_le(a1493, a1494) {
        true
    } else {
        let a1495 = v0;
        let a1496 = 2147483648u64;
        nat_eq(a1495, a1496)
    }
}

pub fn f58(v0: u64, v1: u64) -> R<u64> {
    let a1497 = v0;
    let a1498 = 0u64;
    if nat_eq(a1497, a1498) {
        Ok(0u64)
    } else {
        let a1499 = v1;
        let a1500 = f115(v0)?;
        nat_mul(a1499, a1500)
    }
}

pub fn f59(v0: u64, v1: u64, v2: u64, v3: u64) -> R<u64> {
    let a1501 = v0;
    let a1502 = 0u64;
    if if if if nat_eq(a1501, a1502) {
        true
    } else {
        let a1503 = v1;
        let a1504 = 0u64;
        nat_eq(a1503, a1504)
    } {
        true
    } else {
        let a1505 = v2;
        let a1506 = 0u64;
        nat_eq(a1505, a1506)
    } {
        true
    } else {
        let a1507 = v3;
        let a1508 = 0u64;
        nat_eq(a1507, a1508)
    } {
        Ok(0u64)
    } else {
        let a1521 = {
            let a1511 = 2u64;
            let a1512 = {
                let a1509 = v1;
                let a1510 = v2;
                nat_mul(a1509, a1510)?
            };
            nat_mul(a1511, a1512)?
        };
        let a1522 = {
            let a1519 = {
                let a1515 = {
                    let a1513 = v1;
                    let a1514 = v2;
                    nat_mul(a1513, a1514)?
                };
                let a1516 = v3;
                nat_mul(a1515, a1516)?
            };
            let a1520 = {
                let a1517 = 1u64;
                let a1518 = v0;
                nat_add(a1517, a1518)?
            };
            nat_mul(a1519, a1520)?
        };
        nat_add(a1521, a1522)
    }
}

pub fn f60(v0: u64, v1: u64, v2: u64) -> Option<bool> {
    let v3: bool = {
        let a1523 = v1;
        let a1524 = 0u64;
        if if if nat_eq(a1523, a1524) {
            true
        } else {
            let a1525 = v1;
            let a1526 = 10u64;
            nat_eq(a1525, a1526)
        } {
            true
        } else {
            let a1527 = v1;
            let a1528 = 15u64;
            nat_eq(a1527, a1528)
        } {
            true
        } else {
            let a1529 = v1;
            let a1530 = 20u64;
            nat_eq(a1529, a1530)
        }
    };
    let v4: bool = {
        let a1531 = v2;
        let a1532 = 0u64;
        if if if if if nat_eq(a1531, a1532) {
            true
        } else {
            let a1533 = v2;
            let a1534 = 10u64;
            nat_eq(a1533, a1534)
        } {
            true
        } else {
            let a1535 = v2;
            let a1536 = 15u64;
            nat_eq(a1535, a1536)
        } {
            true
        } else {
            let a1537 = v2;
            let a1538 = 20u64;
            nat_eq(a1537, a1538)
        } {
            true
        } else {
            let a1539 = v2;
            let a1540 = 30u64;
            nat_eq(a1539, a1540)
        } {
            true
        } else {
            let a1541 = v2;
            let a1542 = 40u64;
            nat_eq(a1541, a1542)
        }
    };
    if if v3 {
        v4
    } else {
        false
    } {
        Some({
            let a1543 = v0;
            let a1544 = 30u64;
            if if nat_eq(a1543, a1544) {
                let a1545 = v2;
                let a1546 = 10u64;
                nat_eq(a1545, a1546)
            } else {
                false
            } {
                true
            } else {
                if f116(v0) {
                    let a1547 = v2;
                    let a1548 = 10u64;
                    if if nat_eq(a1547, a1548) {
                        true
                    } else {
                        let a1549 = v2;
                        let a1550 = 15u64;
                        nat_eq(a1549, a1550)
                    } {
                        true
                    } else {
                        let a1551 = v2;
                        let a1552 = 20u64;
                        nat_eq(a1551, a1552)
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

pub fn f61(v0: u64, v1: Adt1, v2: Adt2, v3: Option<Adt2>, v4: Adt2) -> R<Adt9> {
    let v5: u64 = match v1.clone() {
        Adt1::C0(h1553, _, _, _, _, _, _, _, _, _, _, _) => {
            h1553
        }
    };
    Ok(Adt9::C0({
        let a1554 = v0;
        let a1555 = 65u64;
        if nat_eq(a1554, a1555) {
            let m1556 = v3;
            match m1556 {
                None => {
                    f117(v1, v2, v4)?
                }
                Some(_) => {
                    Adt10::C1
                }
            }
        } else {
            let a1557 = v0;
            let a1558 = 59u64;
            if nat_eq(a1557, a1558) {
                let m1559 = v3;
                match m1559 {
                    None => {
                        Adt10::C1
                    }
                    Some(v7) => {
                        f118(v1, v2, v7, v4)?
                    }
                }
            } else {
                let a1560 = v0;
                let a1561 = 50u64;
                if nat_eq(a1560, a1561) {
                    let m1562 = v3;
                    match m1562 {
                        None => {
                            Adt10::C1
                        }
                        Some(v8) => {
                            f119(v1, v2, v8, v4)?
                        }
                    }
                } else {
                    let a1563 = v0;
                    let a1564 = 82u64;
                    if nat_eq(a1563, a1564) {
                        let m1565 = v3;
                        match m1565 {
                            None => {
                                Adt10::C1
                            }
                            Some(v9) => {
                                f120(v2, v9, v4)?
                            }
                        }
                    } else {
                        let a1566 = v0;
                        let a1567 = 86u64;
                        if nat_eq(a1566, a1567) {
                            let m1568 = v3;
                            match m1568 {
                                None => {
                                    Adt10::C1
                                }
                                Some(v10) => {
                                    f121(v1, v2, v10, v4)?
                                }
                            }
                        } else {
                            let a1569 = v0;
                            let a1570 = 55u64;
                            if nat_eq(a1569, a1570) {
                                let m1571 = v3;
                                match m1571 {
                                    None => {
                                        Adt10::C1
                                    }
                                    Some(v11) => {
                                        f122(v1, v2, v11, v4)?
                                    }
                                }
                            } else {
                                let a1572 = v0;
                                let a1573 = 22u64;
                                if nat_eq(a1572, a1573) {
                                    let m1574 = v3;
                                    match m1574 {
                                        None => {
                                            Adt10::C1
                                        }
                                        Some(v12) => {
                                            f123(v2, v12, v4)?
                                        }
                                    }
                                } else {
                                    let a1575 = v0;
                                    let a1576 = 44u64;
                                    if nat_eq(a1575, a1576) {
                                        f124(v1, v2, v3, v4)?
                                    } else {
                                        let a1577 = v0;
                                        let a1578 = 52u64;
                                        if nat_eq(a1577, a1578) {
                                            f125(v1, v2, v4)?
                                        } else {
                                            let a1579 = v0;
                                            let a1580 = 73u64;
                                            if nat_eq(a1579, a1580) {
                                                f126(v2, v3, v4)?
                                            } else {
                                                let a1581 = v0;
                                                let a1582 = 57u64;
                                                if if nat_eq(a1581, a1582) {
                                                    true
                                                } else {
                                                    let a1583 = v0;
                                                    let a1584 = 58u64;
                                                    nat_eq(a1583, a1584)
                                                } {
                                                    let m1585 = v3;
                                                    match m1585 {
                                                        None => {
                                                            f127(false, {
                                                                let a1586 = v0;
                                                                let a1587 = 58u64;
                                                                nat_eq(a1586, a1587)
                                                            }, v1, v2, v4)?
                                                        }
                                                        Some(_) => {
                                                            Adt10::C1
                                                        }
                                                    }
                                                } else {
                                                    let a1588 = v0;
                                                    let a1589 = 49u64;
                                                    if nat_eq(a1588, a1589) {
                                                        f128(false, v1, v2, v3, v4)?
                                                    } else {
                                                        let a1590 = v0;
                                                        let a1591 = 51u64;
                                                        if nat_eq(a1590, a1591) {
                                                            f129(v1, v2, v3, v4)?
                                                        } else {
                                                            let a1592 = v0;
                                                            let a1593 = 39u64;
                                                            if nat_eq(a1592, a1593) {
                                                                f130(v2, v3, v4)?
                                                            } else {
                                                                let a1594 = v0;
                                                                let a1595 = 77u64;
                                                                if nat_eq(a1594, a1595) {
                                                                    f131(v2, v4)?
                                                                } else {
                                                                    let a1596 = v0;
                                                                    let a1597 = 40u64;
                                                                    if nat_eq(a1596, a1597) {
                                                                        f132(v2, v3, v4)?
                                                                    } else {
                                                                        if f133(v0) {
                                                                            let a1598 = v0;
                                                                            let a1599 = 33u64;
                                                                            if if if nat_eq(a1598, a1599) {
                                                                                let a1601 = match v2.clone() {
                                                                                    Adt2::C0(h1600, _, _, _, _, _, _, _, _, _, _) => {
                                                                                        h1600
                                                                                    }
                                                                                };
                                                                                let a1602 = 0u64;
                                                                                nat_eq(a1601, a1602)
                                                                            } else {
                                                                                false
                                                                            } {
                                                                                let a1604 = match v4.clone() {
                                                                                    Adt2::C0(h1603, _, _, _, _, _, _, _, _, _, _) => {
                                                                                        h1603
                                                                                    }
                                                                                };
                                                                                let a1605 = 26u64;
                                                                                nat_eq(a1604, a1605)
                                                                            } else {
                                                                                false
                                                                            } {
                                                                                let m1606 = v3;
                                                                                match m1606 {
                                                                                    None => {
                                                                                        Adt10::C1
                                                                                    }
                                                                                    Some(v14) => {
                                                                                        f134(v2, v14, v4)?
                                                                                    }
                                                                                }
                                                                            } else {
                                                                                let a1607 = v0;
                                                                                let a1608 = 33u64;
                                                                                if if if nat_eq(a1607, a1608) {
                                                                                    let a1610 = match v4.clone() {
                                                                                        Adt2::C0(h1609, _, _, _, _, _, _, _, _, _, _) => {
                                                                                            h1609
                                                                                        }
                                                                                    };
                                                                                    let a1611 = 0u64;
                                                                                    nat_eq(a1610, a1611)
                                                                                } else {
                                                                                    false
                                                                                } {
                                                                                    f135(match v2.clone() {
                                                                                        Adt2::C0(h1612, _, _, _, _, _, _, _, _, _, _) => {
                                                                                            h1612
                                                                                        }
                                                                                    })
                                                                                } else {
                                                                                    false
                                                                                } {
                                                                                    f136(v2, v3, v4)?
                                                                                } else {
                                                                                    f137(v0, v2, {
                                                                                        let m1613 = v3;
                                                                                        match m1613 {
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
                                                                            let a1614 = v0;
                                                                            let a1615 = 19u64;
                                                                            if nat_eq(a1614, a1615) {
                                                                                f138(v2, v4)?
                                                                            } else {
                                                                                let a1616 = v0;
                                                                                let a1617 = 21u64;
                                                                                if nat_eq(a1616, a1617) {
                                                                                    f139(v5, v2, v3, v4)?
                                                                                } else {
                                                                                    let a1618 = v0;
                                                                                    let a1619 = 61u64;
                                                                                    if if if nat_eq(a1618, a1619) {
                                                                                        true
                                                                                    } else {
                                                                                        let a1620 = v0;
                                                                                        let a1621 = 62u64;
                                                                                        nat_eq(a1620, a1621)
                                                                                    } {
                                                                                        true
                                                                                    } else {
                                                                                        let a1622 = v0;
                                                                                        let a1623 = 63u64;
                                                                                        nat_eq(a1622, a1623)
                                                                                    } {
                                                                                        f140(v0, v1, v2, v4)?
                                                                                    } else {
                                                                                        let a1624 = v0;
                                                                                        let a1625 = 4u64;
                                                                                        if if nat_eq(a1624, a1625) {
                                                                                            true
                                                                                        } else {
                                                                                            let a1626 = v0;
                                                                                            let a1627 = 32u64;
                                                                                            nat_eq(a1626, a1627)
                                                                                        } {
                                                                                            f141(v0, v1, v2, v3, v4)?
                                                                                        } else {
                                                                                            let a1628 = v0;
                                                                                            let a1629 = 20u64;
                                                                                            if nat_eq(a1628, a1629) {
                                                                                                f142(v2, v4)?
                                                                                            } else {
                                                                                                let a1630 = v0;
                                                                                                let a1631 = 42u64;
                                                                                                if nat_eq(a1630, a1631) {
                                                                                                    f143(4u64, v2, v4)?
                                                                                                } else {
                                                                                                    let a1632 = v0;
                                                                                                    let a1633 = 69u64;
                                                                                                    if nat_eq(a1632, a1633) {
                                                                                                        let a1634 = v5;
                                                                                                        let a1635 = 4u64;
                                                                                                        if nat_lt(a1634, a1635) {
                                                                                                            f143(v5, v2, v4)?
                                                                                                        } else {
                                                                                                            Adt10::C1
                                                                                                        }
                                                                                                    } else {
                                                                                                        let a1636 = v0;
                                                                                                        let a1637 = 70u64;
                                                                                                        if nat_eq(a1636, a1637) {
                                                                                                            f144(v5, v2, v4)?
                                                                                                        } else {
                                                                                                            let a1638 = v0;
                                                                                                            let a1639 = 75u64;
                                                                                                            if if nat_eq(a1638, a1639) {
                                                                                                                true
                                                                                                            } else {
                                                                                                                let a1640 = v0;
                                                                                                                let a1641 = 76u64;
                                                                                                                nat_eq(a1640, a1641)
                                                                                                            } {
                                                                                                                f145({
                                                                                                                    let a1642 = v0;
                                                                                                                    let a1643 = 76u64;
                                                                                                                    nat_eq(a1642, a1643)
                                                                                                                }, v1, v2, v4)?
                                                                                                            } else {
                                                                                                                let a1644 = v0;
                                                                                                                let a1645 = 31u64;
                                                                                                                if if if nat_eq(a1644, a1645) {
                                                                                                                    true
                                                                                                                } else {
                                                                                                                    let a1646 = v0;
                                                                                                                    let a1647 = 47u64;
                                                                                                                    nat_eq(a1646, a1647)
                                                                                                                } {
                                                                                                                    true
                                                                                                                } else {
                                                                                                                    let a1648 = v0;
                                                                                                                    let a1649 = 68u64;
                                                                                                                    nat_eq(a1648, a1649)
                                                                                                                } {
                                                                                                                    f146(v0, v1, v2, v4)?
                                                                                                                } else {
                                                                                                                    let a1650 = v0;
                                                                                                                    let a1651 = 15u64;
                                                                                                                    if nat_eq(a1650, a1651) {
                                                                                                                        f147(v2, v4)?
                                                                                                                    } else {
                                                                                                                        let a1652 = v0;
                                                                                                                        let a1653 = 17u64;
                                                                                                                        if nat_eq(a1652, a1653) {
                                                                                                                            f148(v2, v4)?
                                                                                                                        } else {
                                                                                                                            let a1654 = v0;
                                                                                                                            let a1655 = 18u64;
                                                                                                                            if nat_eq(a1654, a1655) {
                                                                                                                                f149(v2, v3, v4)?
                                                                                                                            } else {
                                                                                                                                let a1656 = v0;
                                                                                                                                let a1657 = 13u64;
                                                                                                                                if if if nat_eq(a1656, a1657) {
                                                                                                                                    true
                                                                                                                                } else {
                                                                                                                                    let a1658 = v0;
                                                                                                                                    let a1659 = 14u64;
                                                                                                                                    nat_eq(a1658, a1659)
                                                                                                                                } {
                                                                                                                                    true
                                                                                                                                } else {
                                                                                                                                    let a1660 = v0;
                                                                                                                                    let a1661 = 16u64;
                                                                                                                                    nat_eq(a1660, a1661)
                                                                                                                                } {
                                                                                                                                    f150(v0, v2, v4)?
                                                                                                                                } else {
                                                                                                                                    let m1662 = f151(v0, v5)?;
                                                                                                                                    match m1662 {
                                                                                                                                        None => {
                                                                                                                                            f152(v0, v5, v2, v3, v4)?
                                                                                                                                        }
                                                                                                                                        Some(v16) => {
                                                                                                                                            f153(v16, v1, v2, v4)?
                                                                                                                                        }
                                                                                                                                    }
                                                                                                                                }
                                                                                                                            }
                                                                                                                        }
                                                                                                                    }
                                                                                                                }
                                                                                                            }
                                                                                                        }
                                                                                                    }
                                                                                                }
                                                                                            }
                                                                                        }
                                                                                    }
                                                                                }
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
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

pub fn f62(v0: Adt3, v1: u64, v2: u64, v3: u64, v4: u64) -> R<u64> {
    f154(v1, v2, v3, v4, match v0.clone() {
        Adt3::C0(_, _, _, _, h1663, _, _, _) => {
            h1663
        }
    }, match v0.clone() {
        Adt3::C0(_, _, _, _, _, h1664, _, _) => {
            h1664
        }
    }, match v0.clone() {
        Adt3::C0(_, _, _, _, _, _, h1665, _) => {
            h1665
        }
    }, match v0 {
        Adt3::C0(_, _, _, _, _, _, _, h1666) => {
            h1666
        }
    })
}

pub fn f63(v0: u64, v1: Adt3, v2: u64) -> u64 {
    let v3: u64 = {
        let a1668 = v0;
        let a1669 = match v1.clone() {
            Adt3::C0(h1667, _, _, _, _, _, _, _) => {
                h1667
            }
        };
        let a1670 = 0u64;
        nat_quot(a1668, a1669, a1670)
    };
    let v4: u64 = {
        let a1672 = v3;
        let a1673 = match v1.clone() {
            Adt3::C0(_, h1671, _, _, _, _, _, _) => {
                h1671
            }
        };
        let a1674 = 0u64;
        nat_quot(a1672, a1673, a1674)
    };
    let a1675 = v2;
    let a1676 = 0u64;
    if nat_eq(a1675, a1676) {
        let a1678 = v0;
        let a1679 = match v1 {
            Adt3::C0(h1677, _, _, _, _, _, _, _) => {
                h1677
            }
        };
        let a1680 = 0u64;
        nat_rem(a1678, a1679, a1680)
    } else {
        let a1681 = v2;
        let a1682 = 1u64;
        if nat_eq(a1681, a1682) {
            let a1684 = v3;
            let a1685 = match v1 {
                Adt3::C0(_, h1683, _, _, _, _, _, _) => {
                    h1683
                }
            };
            let a1686 = 0u64;
            nat_rem(a1684, a1685, a1686)
        } else {
            let a1687 = v2;
            let a1688 = 2u64;
            if nat_eq(a1687, a1688) {
                let a1690 = v4;
                let a1691 = match v1 {
                    Adt3::C0(_, _, h1689, _, _, _, _, _) => {
                        h1689
                    }
                };
                let a1692 = 0u64;
                nat_rem(a1690, a1691, a1692)
            } else {
                let a1694 = v4;
                let a1695 = match v1 {
                    Adt3::C0(_, _, h1693, _, _, _, _, _) => {
                        h1693
                    }
                };
                let a1696 = 0u64;
                nat_quot(a1694, a1695, a1696)
            }
        }
    }
}

pub fn f64(v0: Adt8, v1: u64, v2: u64) -> R<u64> {
    let a1698 = match v0.clone() {
        Adt8::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, h1697) => {
            h1697
        }
    };
    let a1699 = 0u64;
    if nat_eq(a1698, a1699) {
        Ok(v2)
    } else {
        let a1704 = {
            let a1701 = v2;
            let a1702 = match v0.clone() {
                Adt8::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, h1700) => {
                    h1700
                }
            };
            nat_mul(a1701, a1702)?
        };
        let a1705 = f63(v1, match v0 {
            Adt8::C0(_, _, _, _, _, _, _, _, h1703, _, _, _, _, _, _) => {
                h1703
            }
        }, 3u64);
        nat_add(a1704, a1705)
    }
}

pub fn f65(v0: Adt6, v1: u64, v2: u64, v3: u64) -> R<Option<u64>> {
    let v4: u64 = {
        let a1715 = {
            let a1712 = {
                let a1707 = v2;
                let a1708 = match v0.clone() {
                    Adt6::C0(_, h1706, _, _) => {
                        h1706
                    }
                };
                nat_mul(a1707, a1708)?
            };
            let a1713 = {
                let a1710 = v3;
                let a1711 = match v0.clone() {
                    Adt6::C0(_, _, _, h1709) => {
                        h1709
                    }
                };
                nat_mul(a1710, a1711)?
            };
            nat_add(a1712, a1713)?
        };
        let a1716 = f155(match v0.clone() {
            Adt6::C0(_, _, h1714, _) => {
                h1714
            }
        });
        nat_add(a1715, a1716)?
    };
    let v5: u64 = f156(match v0 {
        Adt6::C0(_, _, h1717, _) => {
            h1717
        }
    });
    let a1718 = v5;
    let a1719 = v4;
    if if nat_le(a1718, a1719) {
        let a1722 = {
            let a1720 = v4;
            let a1721 = v5;
            nat_sub(a1720, a1721)
        };
        let a1723 = v1;
        nat_lt(a1722, a1723)
    } else {
        false
    } {
        Ok(Some({
            let a1724 = v4;
            let a1725 = v5;
            nat_sub(a1724, a1725)
        }))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f66(v0: Adt8, v1: u64, v2: u64, v3: u64, v4: u64) -> R<Adt9> {
    let v5: Adt3 = match v0.clone() {
        Adt8::C0(_, _, _, _, _, _, _, _, _, h1726, _, _, _, _, _) => {
            h1726
        }
    };
    let v6: Adt3 = match v0.clone() {
        Adt8::C0(_, _, _, _, _, _, _, _, h1727, _, _, _, _, _, _) => {
            h1727
        }
    };
    let v7: u64 = f63(v1, v6.clone(), 2u64);
    let a1729 = match v0.clone() {
        Adt8::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, h1728) => {
            h1728
        }
    };
    let a1730 = 0u64;
    if nat_eq(a1729, a1730) {
        Ok(Adt9::C8(Some(f62(v5, v3, v4, v7, v2)?)))
    } else {
        let v8: u64 = {
            let m1733 = match v0.clone() {
                Adt8::C0(_, _, _, _, _, _, _, _, _, _, h1732, _, _, _, _) => {
                    h1732
                }
            };
            match m1733 {
                Adt3::C0(_, _, h1731, _, _, _, _, _) => {
                    h1731
                }
            }
        };
        let m1739 = f65(match v0.clone() {
            Adt8::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, h1734, _) => {
                h1734
            }
        }, match v5.clone() {
            Adt3::C0(_, _, h1735, _, _, _, _, _) => {
                h1735
            }
        }, {
            let a1736 = v2;
            let a1737 = v8;
            let a1738 = 0u64;
            nat_rem(a1736, a1737, a1738)
        }, v7)?;
        match m1739 {
            None => {
                Ok(Adt9::C8(None::<u64>))
            }
            Some(v9) => {
                let v10: u64 = {
                    let a1746 = {
                        let a1744 = {
                            let a1740 = v2;
                            let a1741 = v8;
                            let a1742 = 0u64;
                            nat_quot(a1740, a1741, a1742)
                        };
                        let a1745 = match v0 {
                            Adt8::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, h1743) => {
                                h1743
                            }
                        };
                        nat_mul(a1744, a1745)?
                    };
                    let a1747 = f63(v1, v6, 3u64);
                    nat_add(a1746, a1747)?
                };
                Ok(Adt9::C8(Some(f62(v5, v3, v4, v9, v10)?)))
            }
        }
    }
}

pub fn f67(v0: Adt4, v1: Adt9) -> Adt9 {
    let m1748 = v1;
    match m1748 {
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
            f157(f158(v0.clone(), v20, v21), v20, v21, 0u64, v22, 2u64, f159(v0), 3u64, v23)
        }
        Adt9::C4(v24, v25, v26, v27, v28, v29, v30) => {
            let v31: u64 = f160(v0.clone(), v24);
            f157(f161(v0, v24), v31, v31, v25, v26, v27, v28, v29, v30)
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

pub fn f68(v0: u64, v1: Adt3, v2: Adt3, v3: Adt4) -> R<Adt9> {
    let m1749 = v3;
    match m1749 {
        Adt4::C0 => {
            Ok(Adt9::C1(false, f103(v0, v1)?, f103(v0, v2)?))
        }
        Adt4::C1 => {
            Ok(Adt9::C1(false, f163(v0, v1, v2.clone())?, f103(v0, v2)?))
        }
        Adt4::C2(v6, v7) => {
            f164(v0, v1, v7, v2, v6)
        }
        Adt4::C3(v8, v9, v10, v11, v12) => {
            f165(v0, v1, v2, v8, v9, v10, v11, v12)
        }
        Adt4::C4(v13) => {
            f166(v0, v1, v2, v13)
        }
        Adt4::C5(v14, v15, v16, v17) => {
            f165(v0, v1, v2, v14, v15, v16, v17, true)
        }
        Adt4::C6(v18, v19, v20) => {
            f167(v0, v1, v2, v18, v19, v20)
        }
        Adt4::C7(v21) => {
            f168(v0, v1, v2, v21)
        }
        Adt4::C8(v22) => {
            f169(v22, v0, v1, v2)
        }
        Adt4::C9(_, _) => {
            Ok(Adt9::C1(true, 0u64, f103(v0, v2)?))
        }
        Adt4::C10(v25, v26, v27) => {
            f170(v0, v1, v2, v25, v26, v27)
        }
        Adt4::C11(v64, v65) => {
            f171(v0, v1, v2, v64, v65)
        }
        Adt4::C12(v66, v67, v68) => {
            f172(v0, v1, v2, v66, v67, v68)
        }
        Adt4::C13(_, v70, _, _) => {
            Ok(Adt9::C3(v70, v70, f103(v0, v1)?, f103(v0, v2)?))
        }
        Adt4::C14(v73, v74, v75) => {
            let a1752 = v73;
            let a1753 = 23u64;
            if nat_eq(a1752, a1753) {
                f173(v0, v1, v2, v74, v75)
            } else {
                Ok(Adt9::C3(v74, v75, f103(v0, v1)?, f103(v0, v2)?))
            }
        }
        Adt4::C15 => {
            f174(v0, v1, v2)
        }
        Adt4::C16(v76, v77) => {
            f175(v0, v1, v2, v76, v77)
        }
        Adt4::C17 => {
            Ok(Adt9::C0(Adt10::C1))
        }
        Adt4::C18(v78, v79) => {
            f176(v0, v1, v79, v78)
        }
        Adt4::C19(_, _) => {
            Ok(Adt9::C0(Adt10::C1))
        }
        Adt4::C20(_, _) => {
            Ok(Adt9::C0(Adt10::C1))
        }
        Adt4::C21 => {
            Ok(Adt9::C1(false, f103(f162(v0, match v2.clone() {
                Adt3::C0(h1750, _, _, _, _, _, _, _) => {
                    h1750
                }
            }, match v2.clone() {
                Adt3::C0(_, h1751, _, _, _, _, _, _) => {
                    h1751
                }
            })?, v1)?, f103(v0, v2)?))
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
        Adt4::C25(v84, v85, v86) => {
            f177(v0, v84, v85, v86)
        }
        Adt4::C26(v92, _, _, _) => {
            let a1754 = v92;
            let a1755 = 64u64;
            if if if nat_eq(a1754, a1755) {
                true
            } else {
                let a1756 = v92;
                let a1757 = 65u64;
                nat_eq(a1756, a1757)
            } {
                true
            } else {
                let a1758 = v92;
                let a1759 = 66u64;
                nat_eq(a1758, a1759)
            } {
                Ok(Adt9::C0(Adt10::C5(64u64)))
            } else {
                Ok(Adt9::C0(Adt10::C1))
            }
        }
        Adt4::C27(v91) => {
            f93(v0, v1, v2, v91)
        }
        Adt4::C28(v87) => {
            f90(v0, v1, v2, v87, 0u64)
        }
        Adt4::C29(v88) => {
            f91(v0, v1, v2, v88)
        }
        Adt4::C30(v89, _) => {
            f92(v0, v1, v2, v89)
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
        Adt4::C47(_, _) => {
            Ok(Adt9::C0(Adt10::C1))
        }
    }
}

pub fn f69(v0: u64, v1: Adt3, v2: Adt3, v3: u64, v4: u64) -> R<Adt9> {
    let v5: u64 = f178(v3, v4);
    let v6: u64 = {
        let a1760 = v0;
        let a1761 = v5;
        let a1762 = 0u64;
        nat_quot(a1760, a1761, a1762)
    };
    let a1763 = v5;
    let a1764 = 1u64;
    if nat_eq(a1763, a1764) {
        Ok(f157(f179(v3, v4), v3, v4, 0u64, f103(v6, v1)?, 2u64, 0u64, 3u64, f103(v6, v2)?))
    } else {
        let a1768 = {
            let a1765 = v0;
            let a1766 = 2u64;
            let a1767 = 0u64;
            nat_rem(a1765, a1766, a1767)
        };
        let a1769 = 0u64;
        if nat_eq(a1768, a1769) {
            Ok(f157(f179(v3, 0u64), v3, 0u64, 0u64, f103(v6, v1)?, 2u64, 0u64, 1u64, 0u64))
        } else {
            Ok(f157(f179(0u64, v4), 0u64, v4, 1u64, 0u64, 2u64, 0u64, 3u64, f103(v6, v2)?))
        }
    }
}

pub fn f70(v0: u64, v1: u64, v2: u64) -> R<Adt9> {
    let v3: u64 = f180(v1)?;
    let v4: u64 = {
        let a1770 = v0;
        let a1771 = v3;
        let a1772 = 0u64;
        nat_rem(a1770, a1771, a1772)
    };
    let v5: u64 = {
        let a1775 = {
            let a1773 = v1;
            let a1774 = 3u64;
            nat_mul(a1773, a1774)?
        };
        let a1776 = 1u64;
        nat_add(a1775, a1776)?
    };
    let a1777 = v4;
    let a1778 = 0u64;
    if nat_eq(a1777, a1778) {
        Ok(Adt9::C7(Adt11::C0(0u64), Adt12::C2(0u64), Adt12::C2(0u64), Adt13::C0(0u64), 8u64, 8u64))
    } else {
        let a1779 = v4;
        let a1780 = v5;
        if nat_eq(a1779, a1780) {
            Ok(Adt9::C7(Adt11::C0(2u64), Adt12::C1(0u64), Adt12::C2(0u64), Adt13::C0(0u64), 8u64, 8u64))
        } else {
            let v6: u64 = {
                let a1785 = v4;
                let a1786 = {
                    let a1781 = v4;
                    let a1782 = v5;
                    if nat_lt(a1781, a1782) {
                        1u64
                    } else {
                        let a1783 = v5;
                        let a1784 = 1u64;
                        nat_add(a1783, a1784)?
                    }
                };
                nat_sub(a1785, a1786)
            };
            let v7: u64 = {
                let a1787 = v6;
                let a1788 = 3u64;
                let a1789 = 0u64;
                nat_rem(a1787, a1788, a1789)
            };
            let v8: u64 = {
                let a1800 = 4u64;
                let a1801 = {
                    let a1798 = {
                        let a1793 = {
                            let a1790 = v0;
                            let a1791 = v3;
                            let a1792 = 0u64;
                            nat_quot(a1790, a1791, a1792)
                        };
                        let a1794 = v1;
                        nat_mul(a1793, a1794)?
                    };
                    let a1799 = {
                        let a1795 = v6;
                        let a1796 = 3u64;
                        let a1797 = 0u64;
                        nat_quot(a1795, a1796, a1797)
                    };
                    nat_add(a1798, a1799)?
                };
                nat_mul(a1800, a1801)?
            };
            let a1802 = v4;
            let a1803 = v5;
            if nat_lt(a1802, a1803) {
                let a1804 = v7;
                let a1805 = 0u64;
                if nat_eq(a1804, a1805) {
                    Ok(Adt9::C7(Adt11::C1(2u64), Adt12::C0(1u64, v8), Adt12::C0(0u64, v8), Adt13::C0(1u64), 4u64, 4u64))
                } else {
                    let a1806 = v7;
                    let a1807 = 1u64;
                    if nat_eq(a1806, a1807) {
                        Ok(Adt9::C7(Adt11::C0(1u64), Adt12::C1(1u64), Adt12::C2(0u64), Adt13::C0(1u64), 4u64, 4u64))
                    } else {
                        Ok(Adt9::C7(Adt11::C0(0u64), Adt12::C1(0u64), Adt12::C1(1u64), Adt13::C0(0u64), 8u64, 8u64))
                    }
                }
            } else {
                let a1808 = v7;
                let a1809 = 0u64;
                if nat_eq(a1808, a1809) {
                    Ok(Adt9::C7(Adt11::C1(1u64), Adt12::C0(0u64, v8), Adt12::C1(0u64), Adt13::C0(1u64), 4u64, 4u64))
                } else {
                    let a1810 = v7;
                    let a1811 = 1u64;
                    if nat_eq(a1810, a1811) {
                        Ok(Adt9::C7(Adt11::C1(2u64), Adt12::C2(v2), Adt12::C1(1u64), Adt13::C0(1u64), 4u64, 4u64))
                    } else {
                        Ok(Adt9::C7(Adt11::C1(2u64), Adt12::C1(1u64), Adt12::C0(1u64, v8), Adt13::C1(v8), 4u64, 4u64))
                    }
                }
            }
        }
    }
}

pub fn f71(v0: u64, v1: Adt3, v2: Adt3, v3: Adt3) -> R<Adt9> {
    let v4: u64 = {
        let a1812 = v0;
        let a1813 = 6u64;
        let a1814 = 0u64;
        nat_quot(a1812, a1813, a1814)
    };
    let v5: u64 = {
        let a1815 = v0;
        let a1816 = 6u64;
        let a1817 = 0u64;
        nat_rem(a1815, a1816, a1817)
    };
    let a1818 = v5;
    let a1819 = 0u64;
    if nat_eq(a1818, a1819) {
        Ok(Adt9::C7(Adt11::C7(17u64, 0u64, 0u64), Adt12::C0(1u64, f103(v4, v2)?), Adt12::C2(0u64), Adt13::C0(0u64), 4u64, 4u64))
    } else {
        let a1820 = v5;
        let a1821 = 1u64;
        if nat_eq(a1820, a1821) {
            Ok(Adt9::C7(Adt11::C1(1u64), Adt12::C2(1065353216u64), Adt12::C1(0u64), Adt13::C0(1u64), 4u64, 4u64))
        } else {
            let a1822 = v5;
            let a1823 = 2u64;
            if nat_eq(a1822, a1823) {
                Ok(Adt9::C7(Adt11::C1(2u64), Adt12::C0(1u64, f103(v4, v2)?), Adt12::C1(1u64), Adt13::C0(1u64), 4u64, 4u64))
            } else {
                let a1824 = v5;
                let a1825 = 3u64;
                if nat_eq(a1824, a1825) {
                    Ok(Adt9::C7(Adt11::C1(0u64), Adt12::C2(1065353216u64), Adt12::C1(1u64), Adt13::C0(1u64), 4u64, 4u64))
                } else {
                    let a1826 = v5;
                    let a1827 = 4u64;
                    if nat_eq(a1826, a1827) {
                        Ok(Adt9::C7(Adt11::C1(2u64), Adt12::C0(0u64, f103(v4, v1)?), Adt12::C1(0u64), Adt13::C0(0u64), 4u64, 4u64))
                    } else {
                        Ok(Adt9::C7(Adt11::C1(2u64), Adt12::C1(0u64), Adt12::C1(1u64), Adt13::C1(f103(v4, v3)?), 4u64, 4u64))
                    }
                }
            }
        }
    }
}

pub fn f72(v0: u64, v1: Adt3, v2: Adt3, v3: Adt3, v4: bool, v5: u64) -> R<Adt9> {
    let v6: u64 = if v4 {
        3u64
    } else {
        2u64
    };
    let v7: u64 = f181(match v1.clone() {
        Adt3::C0(_, h1828, _, _, _, _, _, _) => {
            h1828
        }
    }, v4)?;
    let v8: u64 = {
        let a1829 = v0;
        let a1830 = v7;
        let a1831 = 0u64;
        nat_rem(a1829, a1830, a1831)
    };
    let v9: u64 = {
        let a1832 = v0;
        let a1833 = v7;
        let a1834 = 0u64;
        nat_quot(a1832, a1833, a1834)
    };
    let a1837 = {
        let a1835 = v8;
        let a1836 = 1u64;
        nat_add(a1835, a1836)?
    };
    let a1838 = v7;
    if nat_eq(a1837, a1838) {
        Ok(Adt9::C7(Adt11::C1(2u64), Adt12::C2(v5), Adt12::C1(0u64), Adt13::C1(f103(v9, v3)?), 4u64, 4u64))
    } else {
        let v10: u64 = {
            let a1839 = v8;
            let a1840 = v6;
            let a1841 = 0u64;
            nat_quot(a1839, a1840, a1841)
        };
        let v11: u64 = {
            let a1842 = v8;
            let a1843 = v6;
            let a1844 = 0u64;
            nat_rem(a1842, a1843, a1844)
        };
        let v12: u64 = {
            let a1846 = v9;
            let a1847 = match v1.clone() {
                Adt3::C0(h1845, _, _, _, _, _, _, _) => {
                    h1845
                }
            };
            let a1848 = 0u64;
            nat_rem(a1846, a1847, a1848)
        };
        let v13: u64 = {
            let a1850 = v9;
            let a1851 = match v1.clone() {
                Adt3::C0(h1849, _, _, _, _, _, _, _) => {
                    h1849
                }
            };
            let a1852 = 0u64;
            nat_quot(a1850, a1851, a1852)
        };
        let v14: u64 = if v4 {
            let a1864 = {
                let a1859 = {
                    let a1854 = match v2.clone() {
                        Adt3::C0(_, _, _, _, h1853, _, _, _) => {
                            h1853
                        }
                    };
                    let a1855 = v12;
                    nat_mul(a1854, a1855)?
                };
                let a1860 = {
                    let a1857 = match v2.clone() {
                        Adt3::C0(_, _, _, _, _, h1856, _, _) => {
                            h1856
                        }
                    };
                    let a1858 = v10;
                    nat_mul(a1857, a1858)?
                };
                nat_add(a1859, a1860)?
            };
            let a1865 = {
                let a1862 = match v2 {
                    Adt3::C0(_, _, _, _, _, _, h1861, _) => {
                        h1861
                    }
                };
                let a1863 = v13;
                nat_mul(a1862, a1863)?
            };
            nat_add(a1864, a1865)?
        } else {
            let a1872 = {
                let a1867 = match v2.clone() {
                    Adt3::C0(_, _, _, _, h1866, _, _, _) => {
                        h1866
                    }
                };
                let a1868 = v10;
                nat_mul(a1867, a1868)?
            };
            let a1873 = {
                let a1870 = match v2 {
                    Adt3::C0(_, _, _, _, _, h1869, _, _) => {
                        h1869
                    }
                };
                let a1871 = v13;
                nat_mul(a1870, a1871)?
            };
            nat_add(a1872, a1873)?
        };
        if if v4 {
            let a1874 = v11;
            let a1875 = 0u64;
            nat_eq(a1874, a1875)
        } else {
            false
        } {
            Ok(Adt9::C7(Adt11::C7(17u64, 0u64, 0u64), Adt12::C0(1u64, v14), Adt12::C2(0u64), Adt13::C0(1u64), 4u64, 4u64))
        } else {
            let a1878 = {
                let a1876 = v11;
                let a1877 = 1u64;
                nat_add(a1876, a1877)?
            };
            let a1879 = v6;
            if nat_eq(a1878, a1879) {
                Ok(Adt9::C7(Adt11::C1(0u64), {
                    let a1880 = v10;
                    let a1881 = 0u64;
                    if nat_eq(a1880, a1881) {
                        Adt12::C2(0u64)
                    } else {
                        Adt12::C1(0u64)
                    }
                }, Adt12::C1(1u64), Adt13::C0(0u64), 4u64, 4u64))
            } else {
                Ok(Adt9::C7(Adt11::C1(2u64), Adt12::C0(0u64, {
                    let a1893 = {
                        let a1888 = {
                            let a1883 = match v1.clone() {
                                Adt3::C0(_, _, _, _, h1882, _, _, _) => {
                                    h1882
                                }
                            };
                            let a1884 = v12;
                            nat_mul(a1883, a1884)?
                        };
                        let a1889 = {
                            let a1886 = match v1.clone() {
                                Adt3::C0(_, _, _, _, _, h1885, _, _) => {
                                    h1885
                                }
                            };
                            let a1887 = v10;
                            nat_mul(a1886, a1887)?
                        };
                        nat_add(a1888, a1889)?
                    };
                    let a1894 = {
                        let a1891 = match v1 {
                            Adt3::C0(_, _, _, _, _, _, h1890, _) => {
                                h1890
                            }
                        };
                        let a1892 = v13;
                        nat_mul(a1891, a1892)?
                    };
                    nat_add(a1893, a1894)?
                }), if v4 {
                    Adt12::C1(1u64)
                } else {
                    Adt12::C0(1u64, v14)
                }, Adt13::C0(1u64), 4u64, 4u64))
            }
        }
    }
}

pub fn f73(v0: u64, v1: Adt3, v2: Adt3, v3: Adt3, v4: Option<Adt3>, v5: Adt3) -> R<Adt9> {
    let v6: u64 = f38(match v2.clone() {
        Adt3::C0(_, h1895, _, _, _, _, _, _) => {
            h1895
        }
    }, v4.clone())?;
    let v7: u64 = {
        let a1896 = v0;
        let a1897 = v6;
        let a1898 = 0u64;
        nat_rem(a1896, a1897, a1898)
    };
    let v8: u64 = {
        let a1899 = v0;
        let a1900 = v6;
        let a1901 = 0u64;
        nat_quot(a1899, a1900, a1901)
    };
    let v9: u64 = {
        let a1903 = v8;
        let a1904 = match v5.clone() {
            Adt3::C0(h1902, _, _, _, _, _, _, _) => {
                h1902
            }
        };
        let a1905 = 0u64;
        nat_rem(a1903, a1904, a1905)
    };
    let v10: u64 = {
        let a1907 = v8;
        let a1908 = match v5.clone() {
            Adt3::C0(h1906, _, _, _, _, _, _, _) => {
                h1906
            }
        };
        let a1909 = 0u64;
        nat_quot(a1907, a1908, a1909)
    };
    let v11: u64 = {
        let a1911 = v10;
        let a1912 = match v2.clone() {
            Adt3::C0(_, h1910, _, _, _, _, _, _) => {
                h1910
            }
        };
        let a1913 = 0u64;
        nat_rem(a1911, a1912, a1913)
    };
    let v12: u64 = {
        let a1915 = v10;
        let a1916 = match v2.clone() {
            Adt3::C0(_, h1914, _, _, _, _, _, _) => {
                h1914
            }
        };
        let a1917 = 0u64;
        nat_quot(a1915, a1916, a1917)
    };
    let a1918 = v7;
    let a1919 = 0u64;
    if nat_eq(a1918, a1919) {
        Ok(Adt9::C7(Adt11::C1(2u64), Adt12::C0(0u64, {
            let a1926 = {
                let a1921 = match v1.clone() {
                    Adt3::C0(_, _, _, _, h1920, _, _, _) => {
                        h1920
                    }
                };
                let a1922 = v9;
                nat_mul(a1921, a1922)?
            };
            let a1927 = {
                let a1924 = match v1 {
                    Adt3::C0(_, _, _, _, _, h1923, _, _) => {
                        h1923
                    }
                };
                let a1925 = v12;
                nat_mul(a1924, a1925)?
            };
            nat_add(a1926, a1927)?
        }), Adt12::C0(2u64, {
            let a1934 = {
                let a1929 = match v3.clone() {
                    Adt3::C0(_, _, _, _, h1928, _, _, _) => {
                        h1928
                    }
                };
                let a1930 = v11;
                nat_mul(a1929, a1930)?
            };
            let a1935 = {
                let a1932 = match v3 {
                    Adt3::C0(_, _, _, _, _, h1931, _, _) => {
                        h1931
                    }
                };
                let a1933 = v12;
                nat_mul(a1932, a1933)?
            };
            nat_add(a1934, a1935)?
        }), Adt13::C0(0u64), 4u64, 4u64))
    } else {
        let m1936 = v4;
        match m1936 {
            None => {
                Ok(Adt9::C7(Adt11::C1(0u64), Adt12::C1(0u64), Adt12::C0(1u64, {
                    let a1948 = {
                        let a1943 = {
                            let a1938 = match v2.clone() {
                                Adt3::C0(_, _, _, _, h1937, _, _, _) => {
                                    h1937
                                }
                            };
                            let a1939 = v9;
                            nat_mul(a1938, a1939)?
                        };
                        let a1944 = {
                            let a1941 = match v2.clone() {
                                Adt3::C0(_, _, _, _, _, h1940, _, _) => {
                                    h1940
                                }
                            };
                            let a1942 = v11;
                            nat_mul(a1941, a1942)?
                        };
                        nat_add(a1943, a1944)?
                    };
                    let a1949 = {
                        let a1946 = match v2 {
                            Adt3::C0(_, _, _, _, _, _, h1945, _) => {
                                h1945
                            }
                        };
                        let a1947 = v12;
                        nat_mul(a1946, a1947)?
                    };
                    nat_add(a1948, a1949)?
                }), Adt13::C1(f103(v8, v5)?), 4u64, 4u64))
            }
            Some(v13) => {
                let v14: u64 = {
                    let a1950 = v7;
                    let a1951 = 1u64;
                    nat_sub(a1950, a1951)
                };
                let v15: u64 = {
                    let a1952 = v14;
                    let a1953 = 2u64;
                    let a1954 = 0u64;
                    nat_quot(a1952, a1953, a1954)
                };
                let a1958 = {
                    let a1955 = v14;
                    let a1956 = 2u64;
                    let a1957 = 0u64;
                    nat_rem(a1955, a1956, a1957)
                };
                let a1959 = 0u64;
                if nat_eq(a1958, a1959) {
                    Ok(Adt9::C7(Adt11::C1(2u64), Adt12::C0(1u64, {
                        let a1971 = {
                            let a1966 = {
                                let a1961 = match v2.clone() {
                                    Adt3::C0(_, _, _, _, h1960, _, _, _) => {
                                        h1960
                                    }
                                };
                                let a1962 = v9;
                                nat_mul(a1961, a1962)?
                            };
                            let a1967 = {
                                let a1964 = match v2.clone() {
                                    Adt3::C0(_, _, _, _, _, h1963, _, _) => {
                                        h1963
                                    }
                                };
                                let a1965 = v15;
                                nat_mul(a1964, a1965)?
                            };
                            nat_add(a1966, a1967)?
                        };
                        let a1972 = {
                            let a1969 = match v2 {
                                Adt3::C0(_, _, _, _, _, _, h1968, _) => {
                                    h1968
                                }
                            };
                            let a1970 = v12;
                            nat_mul(a1969, a1970)?
                        };
                        nat_add(a1971, a1972)?
                    }), Adt12::C0(3u64, {
                        let a1984 = {
                            let a1979 = {
                                let a1974 = match v13.clone() {
                                    Adt3::C0(_, _, _, _, h1973, _, _, _) => {
                                        h1973
                                    }
                                };
                                let a1975 = v11;
                                nat_mul(a1974, a1975)?
                            };
                            let a1980 = {
                                let a1977 = match v13.clone() {
                                    Adt3::C0(_, _, _, _, _, h1976, _, _) => {
                                        h1976
                                    }
                                };
                                let a1978 = v15;
                                nat_mul(a1977, a1978)?
                            };
                            nat_add(a1979, a1980)?
                        };
                        let a1985 = {
                            let a1982 = match v13 {
                                Adt3::C0(_, _, _, _, _, _, h1981, _) => {
                                    h1981
                                }
                            };
                            let a1983 = v12;
                            nat_mul(a1982, a1983)?
                        };
                        nat_add(a1984, a1985)?
                    }), Adt13::C0(1u64), 4u64, 4u64))
                } else {
                    Ok(Adt9::C7(Adt11::C1(0u64), Adt12::C1(0u64), Adt12::C1(1u64), {
                        let a1988 = {
                            let a1986 = v7;
                            let a1987 = 1u64;
                            nat_add(a1986, a1987)?
                        };
                        let a1989 = v6;
                        if nat_eq(a1988, a1989) {
                            Adt13::C1(f103(v8, v5)?)
                        } else {
                            Adt13::C0(0u64)
                        }
                    }, 4u64, 4u64))
                }
            }
        }
    }
}

pub fn f74(v0: u64, v1: Adt3, v2: Adt3, v3: Adt3, v4: Adt3, v5: u64, v6: u64, v7: u64) -> R<Adt9> {
    let v8: u64 = f37(v6)?;
    let v9: u64 = {
        let a1990 = v0;
        let a1991 = v8;
        let a1992 = 0u64;
        nat_rem(a1990, a1991, a1992)
    };
    let v10: u64 = {
        let a1993 = v0;
        let a1994 = v8;
        let a1995 = 0u64;
        nat_quot(a1993, a1994, a1995)
    };
    let a1996 = v9;
    let a1997 = 148u64;
    if nat_lt(a1996, a1997) {
        f182({
            let a1998 = v9;
            let a1999 = 37u64;
            let a2000 = 0u64;
            nat_rem(a1998, a1999, a2000)
        }, {
            let a2001 = v9;
            let a2002 = 37u64;
            let a2003 = 0u64;
            nat_quot(a2001, a2002, a2003)
        }, v10, v1, v2, v3, v4, v5, v7)
    } else {
        f183({
            let a2004 = v9;
            let a2005 = 148u64;
            nat_sub(a2004, a2005)
        }, v10, v4, v5)
    }
}

pub fn f75(v0: u64, v1: Adt3, v2: bool) -> R<Adt9> {
    let v3: u64 = f33(v2);
    let a2006 = v0;
    let a2007 = v3;
    if nat_lt(a2006, a2007) {
        let a2008 = v0;
        let a2009 = if v2 {
            7u64
        } else {
            2u64
        };
        if nat_lt(a2008, a2009) {
            Ok(Adt9::C7(Adt11::C5(3u64), Adt12::C0(if v2 {
                4u64
            } else {
                2u64
            }, {
                let a2010 = 4u64;
                let a2011 = v0;
                nat_mul(a2010, a2011)?
            }), Adt12::C2(0u64), Adt13::C0(v0), 4u64, 4u64))
        } else {
            let a2012 = v0;
            let a2013 = if v2 {
                7u64
            } else {
                2u64
            };
            if nat_eq(a2012, a2013) {
                Ok(Adt9::C7(Adt11::C1(2u64), Adt12::C1(0u64), Adt12::C1(if v2 {
                    4u64
                } else {
                    1u64
                }), Adt13::C0(7u64), 4u64, 4u64))
            } else {
                let a2016 = v0;
                let a2017 = {
                    let a2014 = if v2 {
                        7u64
                    } else {
                        2u64
                    };
                    let a2015 = 1u64;
                    nat_add(a2014, a2015)?
                };
                if nat_eq(a2016, a2017) {
                    Ok(Adt9::C7(Adt11::C1(1u64), Adt12::C2(1065353216u64), Adt12::C1(7u64), Adt13::C0(7u64), 4u64, 4u64))
                } else {
                    let a2018 = v0;
                    let a2019 = 9u64;
                    if nat_eq(a2018, a2019) {
                        Ok(Adt9::C7(Adt11::C1(1u64), Adt12::C2(1065353216u64), Adt12::C1(1u64), Adt13::C0(8u64), 4u64, 4u64))
                    } else {
                        Ok(Adt9::C7(Adt11::C1(1u64), Adt12::C2(1065353216u64), Adt12::C1(2u64), Adt13::C0(9u64), 4u64, 4u64))
                    }
                }
            }
        }
    } else {
        let v4: u64 = {
            let a2020 = v0;
            let a2021 = v3;
            nat_sub(a2020, a2021)
        };
        let v5: u64 = f34(v2);
        let v6: u64 = {
            let a2022 = v4;
            let a2023 = v5;
            let a2024 = 0u64;
            nat_rem(a2022, a2023, a2024)
        };
        let v7: u64 = f103({
            let a2025 = v4;
            let a2026 = v5;
            let a2027 = 0u64;
            nat_quot(a2025, a2026, a2027)
        }, v1)?;
        if v2 {
            let a2028 = v6;
            let a2029 = 7u64;
            if nat_lt(a2028, a2029) {
                let a2030 = v6;
                let a2031 = 3u64;
                if nat_lt(a2030, a2031) {
                    let a2032 = v6;
                    let a2033 = 1u64;
                    if nat_lt(a2032, a2033) {
                        Ok(Adt9::C7(Adt11::C1(2u64), Adt12::C0(2u64, v7), Adt12::C1(1u64), Adt13::C0(10u64), 4u64, 4u64))
                    } else {
                        let a2034 = v6;
                        let a2035 = 2u64;
                        if nat_lt(a2034, a2035) {
                            Ok(Adt9::C7(Adt11::C1(2u64), Adt12::C0(1u64, v7), Adt12::C1(8u64), Adt13::C0(11u64), 4u64, 4u64))
                        } else {
                            Ok(Adt9::C7(Adt11::C1(0u64), Adt12::C1(10u64), Adt12::C1(11u64), Adt13::C3(2u64, v7), 4u64, 4u64))
                        }
                    }
                } else {
                    let a2036 = v6;
                    let a2037 = 5u64;
                    if nat_lt(a2036, a2037) {
                        let a2038 = v6;
                        let a2039 = 4u64;
                        if nat_lt(a2038, a2039) {
                            Ok(Adt9::C7(Adt11::C1(2u64), Adt12::C0(3u64, v7), Adt12::C1(2u64), Adt13::C0(10u64), 4u64, 4u64))
                        } else {
                            Ok(Adt9::C7(Adt11::C1(2u64), Adt12::C0(1u64, v7), Adt12::C0(1u64, v7), Adt13::C0(11u64), 4u64, 4u64))
                        }
                    } else {
                        let a2040 = v6;
                        let a2041 = 6u64;
                        if nat_lt(a2040, a2041) {
                            Ok(Adt9::C7(Adt11::C1(2u64), Adt12::C1(11u64), Adt12::C1(9u64), Adt13::C0(11u64), 4u64, 4u64))
                        } else {
                            Ok(Adt9::C7(Adt11::C1(0u64), Adt12::C1(10u64), Adt12::C1(11u64), Adt13::C3(3u64, v7), 4u64, 4u64))
                        }
                    }
                }
            } else {
                let a2042 = v6;
                let a2043 = 11u64;
                if nat_lt(a2042, a2043) {
                    let a2044 = v6;
                    let a2045 = 9u64;
                    if nat_lt(a2044, a2045) {
                        let a2046 = v6;
                        let a2047 = 8u64;
                        if nat_lt(a2046, a2047) {
                            Ok(Adt9::C7(Adt11::C1(2u64), Adt12::C0(2u64, v7), Adt12::C1(5u64), Adt13::C0(10u64), 4u64, 4u64))
                        } else {
                            Ok(Adt9::C7(Adt11::C1(2u64), Adt12::C0(3u64, v7), Adt12::C1(6u64), Adt13::C0(11u64), 4u64, 4u64))
                        }
                    } else {
                        let a2048 = v6;
                        let a2049 = 10u64;
                        if nat_lt(a2048, a2049) {
                            Ok(Adt9::C7(Adt11::C1(4u64), Adt12::C1(11u64), Adt12::C2(0u64), Adt13::C0(11u64), 4u64, 4u64))
                        } else {
                            Ok(Adt9::C7(Adt11::C1(0u64), Adt12::C1(11u64), Adt12::C1(3u64), Adt13::C0(11u64), 4u64, 4u64))
                        }
                    }
                } else {
                    let a2050 = v6;
                    let a2051 = 13u64;
                    if nat_lt(a2050, a2051) {
                        let a2052 = v6;
                        let a2053 = 12u64;
                        if nat_lt(a2052, a2053) {
                            Ok(Adt9::C7(Adt11::C1(2u64), Adt12::C1(0u64), Adt12::C1(10u64), Adt13::C0(10u64), 4u64, 4u64))
                        } else {
                            Ok(Adt9::C7(Adt11::C1(3u64), Adt12::C1(10u64), Adt12::C1(11u64), Adt13::C0(10u64), 4u64, 4u64))
                        }
                    } else {
                        let a2054 = v6;
                        let a2055 = 14u64;
                        if nat_lt(a2054, a2055) {
                            Ok(Adt9::C7(Adt11::C1(2u64), Adt12::C0(0u64, v7), Adt12::C1(7u64), Adt13::C0(11u64), 4u64, 4u64))
                        } else {
                            Ok(Adt9::C7(Adt11::C1(1u64), Adt12::C1(11u64), Adt12::C1(10u64), Adt13::C1(v7), 4u64, 4u64))
                        }
                    }
                }
            }
        } else {
            let a2056 = v6;
            let a2057 = 1u64;
            if nat_lt(a2056, a2057) {
                Ok(Adt9::C7(Adt11::C1(2u64), Adt12::C0(0u64, v7), Adt12::C1(7u64), Adt13::C0(10u64), 4u64, 4u64))
            } else {
                let a2058 = v6;
                let a2059 = 2u64;
                if nat_lt(a2058, a2059) {
                    Ok(Adt9::C7(Adt11::C1(2u64), Adt12::C1(0u64), Adt12::C0(1u64, v7), Adt13::C0(11u64), 4u64, 4u64))
                } else {
                    Ok(Adt9::C7(Adt11::C1(1u64), Adt12::C1(10u64), Adt12::C1(11u64), Adt13::C1(v7), 4u64, 4u64))
                }
            }
        }
    }
}

pub fn f76(v0: u64, v1: Adt3, v2: Adt3, v3: bool, v4: u64, v5: u64, v6: u64) -> R<Adt9> {
    let v7: u64 = match v1.clone() {
        Adt3::C0(h2060, _, _, _, _, _, _, _) => {
            h2060
        }
    };
    let v8: u64 = f29(v3, v7)?;
    let v9: u64 = {
        let a2061 = v0;
        let a2062 = 2u64;
        nat_sub(a2061, a2062)
    };
    let v10: u64 = {
        let a2063 = v9;
        let a2064 = v8;
        let a2065 = 0u64;
        nat_quot(a2063, a2064, a2065)
    };
    let v11: u64 = {
        let a2066 = v9;
        let a2067 = v8;
        let a2068 = 0u64;
        nat_rem(a2066, a2067, a2068)
    };
    let a2069 = v0;
    let a2070 = 0u64;
    if nat_eq(a2069, a2070) {
        Ok(Adt9::C7(Adt11::C0(3u64), Adt12::C2(v5), Adt12::C2(0u64), Adt13::C0(6u64), 4u64, 4u64))
    } else {
        let a2071 = v0;
        let a2072 = 1u64;
        if nat_eq(a2071, a2072) {
            Ok(Adt9::C7(Adt11::C1(3u64), if v3 {
                Adt12::C0(0u64, 0u64)
            } else {
                Adt12::C2(3212836864u64)
            }, Adt12::C1(6u64), Adt13::C0(6u64), 4u64, 4u64))
        } else {
            let a2073 = v10;
            let a2074 = v5;
            if nat_eq(a2073, a2074) {
                Ok(Adt9::C7(Adt11::C1(2u64), Adt12::C1(5u64), Adt12::C1(6u64), Adt13::C1(0u64), 4u64, 4u64))
            } else {
                let a2077 = v11;
                let a2078 = {
                    let a2075 = 2u64;
                    let a2076 = v7;
                    nat_mul(a2075, a2076)?
                };
                if nat_lt(a2077, a2078) {
                    let v12: u64 = {
                        let a2079 = v11;
                        let a2080 = 2u64;
                        let a2081 = 0u64;
                        nat_quot(a2079, a2080, a2081)
                    };
                    let a2085 = {
                        let a2082 = v11;
                        let a2083 = 2u64;
                        let a2084 = 0u64;
                        nat_rem(a2082, a2083, a2084)
                    };
                    let a2086 = 0u64;
                    if nat_eq(a2085, a2086) {
                        Ok(Adt9::C7(Adt11::C5(0u64), {
                            let a2087 = v12;
                            let a2088 = 0u64;
                            if nat_eq(a2087, a2088) {
                                Adt12::C2(4286578688u64)
                            } else {
                                Adt12::C1(0u64)
                            }
                        }, Adt12::C0(if v3 {
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
                                let a2093 = v12;
                                nat_mul(a2092, a2093)?
                            };
                            nat_add(a2094, a2095)?
                        }), Adt13::C0(2u64), 4u64, 4u64))
                    } else {
                        Ok(Adt9::C7(Adt11::C5(3u64), {
                            let a2096 = v6;
                            let a2097 = 2u64;
                            if nat_eq(a2096, a2097) {
                                let a2098 = v12;
                                let a2099 = 0u64;
                                if nat_eq(a2098, a2099) {
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
                                    let a2105 = {
                                        let a2101 = v10;
                                        let a2102 = match v1 {
                                            Adt3::C0(_, _, _, _, _, h2100, _, _) => {
                                                h2100
                                            }
                                        };
                                        nat_mul(a2101, a2102)?
                                    };
                                    let a2106 = {
                                        let a2103 = 4u64;
                                        let a2104 = v12;
                                        nat_mul(a2103, a2104)?
                                    };
                                    nat_add(a2105, a2106)?
                                })
                            }
                        }, Adt12::C2(0u64), Adt13::C0(0u64), 4u64, 4u64))
                    }
                } else {
                    let a2109 = v11;
                    let a2110 = {
                        let a2107 = 6u64;
                        let a2108 = v7;
                        nat_mul(a2107, a2108)?
                    };
                    if nat_lt(a2109, a2110) {
                        let v13: u64 = {
                            let a2113 = v11;
                            let a2114 = {
                                let a2111 = 2u64;
                                let a2112 = v7;
                                nat_mul(a2111, a2112)?
                            };
                            nat_sub(a2113, a2114)
                        };
                        let v14: u64 = {
                            let a2115 = v13;
                            let a2116 = 4u64;
                            let a2117 = 0u64;
                            nat_quot(a2115, a2116, a2117)
                        };
                        let a2121 = {
                            let a2118 = v13;
                            let a2119 = 4u64;
                            let a2120 = 0u64;
                            nat_rem(a2118, a2119, a2120)
                        };
                        let a2122 = 2u64;
                        if nat_lt(a2121, a2122) {
                            let a2126 = {
                                let a2123 = v13;
                                let a2124 = 4u64;
                                let a2125 = 0u64;
                                nat_rem(a2123, a2124, a2125)
                            };
                            let a2127 = 1u64;
                            if nat_lt(a2126, a2127) {
                                Ok(Adt9::C7(Adt11::C1(1u64), Adt12::C0(if v3 {
                                    1u64
                                } else {
                                    0u64
                                }, {
                                    let a2133 = {
                                        let a2129 = v10;
                                        let a2130 = match v1 {
                                            Adt3::C0(_, _, _, _, _, h2128, _, _) => {
                                                h2128
                                            }
                                        };
                                        nat_mul(a2129, a2130)?
                                    };
                                    let a2134 = {
                                        let a2131 = 4u64;
                                        let a2132 = v14;
                                        nat_mul(a2131, a2132)?
                                    };
                                    nat_add(a2133, a2134)?
                                }), Adt12::C1(0u64), Adt13::C0(3u64), 4u64, 4u64))
                            } else {
                                Ok(Adt9::C7(Adt11::C7(16u64, 0u64, 0u64), Adt12::C1(3u64), Adt12::C2(0u64), if v3 {
                                    Adt13::C1({
                                        let a2140 = {
                                            let a2136 = v10;
                                            let a2137 = match v2 {
                                                Adt3::C0(_, _, _, _, _, h2135, _, _) => {
                                                    h2135
                                                }
                                            };
                                            nat_mul(a2136, a2137)?
                                        };
                                        let a2141 = {
                                            let a2138 = 4u64;
                                            let a2139 = v14;
                                            nat_mul(a2138, a2139)?
                                        };
                                        nat_add(a2140, a2141)?
                                    })
                                } else {
                                    Adt13::C0(3u64)
                                }, 4u64, 4u64))
                            }
                        } else {
                            let a2145 = {
                                let a2142 = v13;
                                let a2143 = 4u64;
                                let a2144 = 0u64;
                                nat_rem(a2142, a2143, a2144)
                            };
                            let a2146 = 3u64;
                            if nat_lt(a2145, a2146) {
                                Ok(Adt9::C7(Adt11::C0(1u64), if v3 {
                                    Adt12::C3({
                                        let a2152 = {
                                            let a2148 = v10;
                                            let a2149 = match v2 {
                                                Adt3::C0(_, _, _, _, _, h2147, _, _) => {
                                                    h2147
                                                }
                                            };
                                            nat_mul(a2148, a2149)?
                                        };
                                        let a2153 = {
                                            let a2150 = 4u64;
                                            let a2151 = v14;
                                            nat_mul(a2150, a2151)?
                                        };
                                        nat_add(a2152, a2153)?
                                    })
                                } else {
                                    Adt12::C1(3u64)
                                }, Adt12::C2(0u64), Adt13::C0(3u64), 4u64, 4u64))
                            } else {
                                Ok(Adt9::C7(Adt11::C0(0u64), {
                                    let a2154 = v14;
                                    let a2155 = 0u64;
                                    if nat_eq(a2154, a2155) {
                                        Adt12::C2(0u64)
                                    } else {
                                        Adt12::C1(1u64)
                                    }
                                }, Adt12::C1(3u64), Adt13::C0(1u64), 4u64, 4u64))
                            }
                        }
                    } else {
                        let a2160 = v11;
                        let a2161 = {
                            let a2158 = {
                                let a2156 = 6u64;
                                let a2157 = v7;
                                nat_mul(a2156, a2157)?
                            };
                            let a2159 = 2u64;
                            nat_add(a2158, a2159)?
                        };
                        if nat_lt(a2160, a2161) {
                            let a2164 = v11;
                            let a2165 = {
                                let a2162 = 6u64;
                                let a2163 = v7;
                                nat_mul(a2162, a2163)?
                            };
                            if nat_eq(a2164, a2165) {
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
                                    let a2170 = v11;
                                    let a2171 = {
                                        let a2168 = {
                                            let a2166 = 6u64;
                                            let a2167 = v7;
                                            nat_mul(a2166, a2167)?
                                        };
                                        let a2169 = 2u64;
                                        nat_add(a2168, a2169)?
                                    };
                                    nat_sub(a2170, a2171)
                                };
                                let v16: u64 = {
                                    let a2172 = v15;
                                    let a2173 = 3u64;
                                    let a2174 = 0u64;
                                    nat_quot(a2172, a2173, a2174)
                                };
                                let a2178 = {
                                    let a2175 = v15;
                                    let a2176 = 3u64;
                                    let a2177 = 0u64;
                                    nat_rem(a2175, a2176, a2177)
                                };
                                let a2179 = 1u64;
                                if nat_lt(a2178, a2179) {
                                    Ok(Adt9::C7(Adt11::C1(2u64), Adt12::C3({
                                        let a2185 = {
                                            let a2181 = v10;
                                            let a2182 = match v2 {
                                                Adt3::C0(_, _, _, _, _, h2180, _, _) => {
                                                    h2180
                                                }
                                            };
                                            nat_mul(a2181, a2182)?
                                        };
                                        let a2186 = {
                                            let a2183 = 4u64;
                                            let a2184 = v16;
                                            nat_mul(a2183, a2184)?
                                        };
                                        nat_add(a2185, a2186)?
                                    }), Adt12::C1(1u64), Adt13::C0(3u64), 4u64, 4u64))
                                } else {
                                    let a2190 = {
                                        let a2187 = v15;
                                        let a2188 = 3u64;
                                        let a2189 = 0u64;
                                        nat_rem(a2187, a2188, a2189)
                                    };
                                    let a2191 = 2u64;
                                    if nat_lt(a2190, a2191) {
                                        Ok(Adt9::C7(Adt11::C1(1u64), Adt12::C1(3u64), Adt12::C0(if v3 {
                                            2u64
                                        } else {
                                            1u64
                                        }, {
                                            let a2196 = {
                                                let a2192 = v10;
                                                let a2193 = v4;
                                                nat_mul(a2192, a2193)?
                                            };
                                            let a2197 = {
                                                let a2194 = 4u64;
                                                let a2195 = v16;
                                                nat_mul(a2194, a2195)?
                                            };
                                            nat_add(a2196, a2197)?
                                        }), Adt13::C0(3u64), 4u64, 4u64))
                                    } else {
                                        Ok(Adt9::C7(Adt11::C1(2u64), Adt12::C1(3u64), Adt12::C1(6u64), Adt13::C1({
                                            let a2203 = {
                                                let a2199 = v10;
                                                let a2200 = match v2 {
                                                    Adt3::C0(_, _, _, _, _, h2198, _, _) => {
                                                        h2198
                                                    }
                                                };
                                                nat_mul(a2199, a2200)?
                                            };
                                            let a2204 = {
                                                let a2201 = 4u64;
                                                let a2202 = v16;
                                                nat_mul(a2201, a2202)?
                                            };
                                            nat_add(a2203, a2204)?
                                        }), 4u64, 4u64))
                                    }
                                }
                            } else {
                                let a2209 = v11;
                                let a2210 = {
                                    let a2207 = {
                                        let a2205 = 11u64;
                                        let a2206 = v7;
                                        nat_mul(a2205, a2206)?
                                    };
                                    let a2208 = 2u64;
                                    nat_add(a2207, a2208)?
                                };
                                if nat_lt(a2209, a2210) {
                                    let v17: u64 = {
                                        let a2215 = v11;
                                        let a2216 = {
                                            let a2213 = {
                                                let a2211 = 6u64;
                                                let a2212 = v7;
                                                nat_mul(a2211, a2212)?
                                            };
                                            let a2214 = 2u64;
                                            nat_add(a2213, a2214)?
                                        };
                                        nat_sub(a2215, a2216)
                                    };
                                    let v18: u64 = {
                                        let a2217 = v17;
                                        let a2218 = 5u64;
                                        let a2219 = 0u64;
                                        nat_quot(a2217, a2218, a2219)
                                    };
                                    let a2223 = {
                                        let a2220 = v17;
                                        let a2221 = 5u64;
                                        let a2222 = 0u64;
                                        nat_rem(a2220, a2221, a2222)
                                    };
                                    let a2224 = 2u64;
                                    if nat_lt(a2223, a2224) {
                                        let a2228 = {
                                            let a2225 = v17;
                                            let a2226 = 5u64;
                                            let a2227 = 0u64;
                                            nat_rem(a2225, a2226, a2227)
                                        };
                                        let a2229 = 1u64;
                                        if nat_lt(a2228, a2229) {
                                            Ok(Adt9::C7(Adt11::C1(1u64), Adt12::C0(if v3 {
                                                1u64
                                            } else {
                                                0u64
                                            }, {
                                                let a2235 = {
                                                    let a2231 = v10;
                                                    let a2232 = match v1 {
                                                        Adt3::C0(_, _, _, _, _, h2230, _, _) => {
                                                            h2230
                                                        }
                                                    };
                                                    nat_mul(a2231, a2232)?
                                                };
                                                let a2236 = {
                                                    let a2233 = 4u64;
                                                    let a2234 = v18;
                                                    nat_mul(a2233, a2234)?
                                                };
                                                nat_add(a2235, a2236)?
                                            }), Adt12::C1(0u64), Adt13::C0(3u64), 4u64, 4u64))
                                        } else {
                                            Ok(Adt9::C7(Adt11::C1(1u64), Adt12::C1(3u64), Adt12::C1(1u64), Adt13::C0(3u64), 4u64, 4u64))
                                        }
                                    } else {
                                        let a2240 = {
                                            let a2237 = v17;
                                            let a2238 = 5u64;
                                            let a2239 = 0u64;
                                            nat_rem(a2237, a2238, a2239)
                                        };
                                        let a2241 = 3u64;
                                        if nat_lt(a2240, a2241) {
                                            Ok(Adt9::C7(Adt11::C1(2u64), Adt12::C1(3u64), Adt12::C0(if v3 {
                                                2u64
                                            } else {
                                                1u64
                                            }, {
                                                let a2246 = {
                                                    let a2242 = v10;
                                                    let a2243 = v4;
                                                    nat_mul(a2242, a2243)?
                                                };
                                                let a2247 = {
                                                    let a2244 = 4u64;
                                                    let a2245 = v18;
                                                    nat_mul(a2244, a2245)?
                                                };
                                                nat_add(a2246, a2247)?
                                            }), Adt13::C0(3u64), 4u64, 4u64))
                                        } else {
                                            let a2251 = {
                                                let a2248 = v17;
                                                let a2249 = 5u64;
                                                let a2250 = 0u64;
                                                nat_rem(a2248, a2249, a2250)
                                            };
                                            let a2252 = 4u64;
                                            if nat_lt(a2251, a2252) {
                                                Ok(Adt9::C7(Adt11::C0(1u64), Adt12::C1(3u64), Adt12::C2(0u64), Adt13::C0(3u64), 4u64, 4u64))
                                            } else {
                                                Ok(Adt9::C7(Adt11::C0(0u64), {
                                                    let a2253 = v18;
                                                    let a2254 = 0u64;
                                                    if nat_eq(a2253, a2254) {
                                                        Adt12::C2(0u64)
                                                    } else {
                                                        Adt12::C1(4u64)
                                                    }
                                                }, Adt12::C1(3u64), Adt13::C0(4u64), 4u64, 4u64))
                                            }
                                        }
                                    }
                                } else {
                                    let a2259 = v11;
                                    let a2260 = {
                                        let a2257 = {
                                            let a2255 = 11u64;
                                            let a2256 = v7;
                                            nat_mul(a2255, a2256)?
                                        };
                                        let a2258 = 2u64;
                                        nat_add(a2257, a2258)?
                                    };
                                    if nat_eq(a2259, a2260) {
                                        Ok(Adt9::C7(Adt11::C0(2u64), Adt12::C1(4u64), Adt12::C2(0u64), Adt13::C0(4u64), 4u64, 4u64))
                                    } else {
                                        Ok(Adt9::C7(Adt11::C1(0u64), {
                                            let a2261 = v10;
                                            let a2262 = 0u64;
                                            if nat_eq(a2261, a2262) {
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

pub fn f77(v0: u64, v1: Adt3, v2: bool, v3: u64) -> R<Adt9> {
    let v4: u64 = match v1.clone() {
        Adt3::C0(h2263, _, _, _, _, _, _, _) => {
            h2263
        }
    };
    let v5: u64 = {
        let a2265 = v4;
        let a2266 = match v1.clone() {
            Adt3::C0(_, h2264, _, _, _, _, _, _) => {
                h2264
            }
        };
        nat_mul(a2265, a2266)?
    };
    let v6: u64 = {
        let a2268 = v5;
        let a2269 = match v1.clone() {
            Adt3::C0(_, _, h2267, _, _, _, _, _) => {
                h2267
            }
        };
        nat_mul(a2268, a2269)?
    };
    let a2270 = v0;
    let a2271 = v6;
    if nat_lt(a2270, a2271) {
        Ok(Adt9::C7(Adt11::C5(3u64), Adt12::C2(0u64), Adt12::C2(0u64), Adt13::C1({
            let a2272 = 4u64;
            let a2273 = v0;
            nat_mul(a2272, a2273)?
        }), 4u64, 4u64))
    } else {
        let v7: u64 = {
            let a2274 = v0;
            let a2275 = v6;
            nat_sub(a2274, a2275)
        };
        let v8: u64 = if v2 {
            let a2278 = 1u64;
            let a2279 = {
                let a2276 = 9u64;
                let a2277 = v4;
                nat_mul(a2276, a2277)?
            };
            nat_add(a2278, a2279)?
        } else {
            let a2280 = 7u64;
            let a2281 = v4;
            nat_mul(a2280, a2281)?
        };
        let v9: u64 = {
            let a2282 = v7;
            let a2283 = v8;
            let a2284 = 0u64;
            nat_quot(a2282, a2283, a2284)
        };
        let v10: u64 = {
            let a2285 = v7;
            let a2286 = v8;
            let a2287 = 0u64;
            nat_rem(a2285, a2286, a2287)
        };
        let v11: u64 = {
            let a2288 = v9;
            let a2289 = v5;
            let a2290 = 0u64;
            nat_quot(a2288, a2289, a2290)
        };
        let v12: u64 = {
            let a2291 = v9;
            let a2292 = v5;
            let a2293 = 0u64;
            nat_rem(a2291, a2292, a2293)
        };
        let v13: u64 = {
            let a2295 = match v1 {
                Adt3::C0(_, _, h2294, _, _, _, _, _) => {
                    h2294
                }
            };
            let a2296 = v3;
            let a2297 = 0u64;
            nat_quot(a2295, a2296, a2297)
        };
        let v14: u64 = {
            let a2298 = 4u64;
            let a2299 = v9;
            nat_mul(a2298, a2299)?
        };
        if v2 {
            let a2300 = v10;
            let a2301 = 0u64;
            if nat_eq(a2300, a2301) {
                Ok(Adt9::C7(Adt11::C5(3u64), Adt12::C2(0u64), Adt12::C2(0u64), Adt13::C0(0u64), 4u64, 4u64))
            } else {
                let a2304 = v10;
                let a2305 = {
                    let a2302 = 2u64;
                    let a2303 = v4;
                    nat_mul(a2302, a2303)?
                };
                if nat_le(a2304, a2305) {
                    let v15: u64 = {
                        let a2308 = {
                            let a2306 = v10;
                            let a2307 = 1u64;
                            nat_sub(a2306, a2307)
                        };
                        let a2309 = 2u64;
                        let a2310 = 0u64;
                        nat_quot(a2308, a2309, a2310)
                    };
                    let v16: u64 = {
                        let a2318 = 4u64;
                        let a2319 = {
                            let a2316 = {
                                let a2314 = v9;
                                let a2315 = {
                                    let a2311 = v12;
                                    let a2312 = v4;
                                    let a2313 = 0u64;
                                    nat_rem(a2311, a2312, a2313)
                                };
                                nat_sub(a2314, a2315)
                            };
                            let a2317 = v15;
                            nat_add(a2316, a2317)?
                        };
                        nat_mul(a2318, a2319)?
                    };
                    let v17: u64 = {
                        let a2331 = 4u64;
                        let a2332 = {
                            let a2329 = {
                                let a2327 = {
                                    let a2325 = {
                                        let a2323 = {
                                            let a2320 = v11;
                                            let a2321 = v13;
                                            let a2322 = 0u64;
                                            nat_quot(a2320, a2321, a2322)
                                        };
                                        let a2324 = v5;
                                        nat_mul(a2323, a2324)?
                                    };
                                    let a2326 = v12;
                                    nat_add(a2325, a2326)?
                                };
                                let a2328 = v4;
                                nat_mul(a2327, a2328)?
                            };
                            let a2330 = v15;
                            nat_add(a2329, a2330)?
                        };
                        nat_mul(a2331, a2332)?
                    };
                    let a2336 = {
                        let a2333 = v10;
                        let a2334 = 2u64;
                        let a2335 = 0u64;
                        nat_rem(a2333, a2334, a2335)
                    };
                    let a2337 = 1u64;
                    if nat_eq(a2336, a2337) {
                        Ok(Adt9::C7(Adt11::C1(2u64), Adt12::C0(4u64, v16), {
                            let a2341 = {
                                let a2338 = v11;
                                let a2339 = v13;
                                let a2340 = 0u64;
                                nat_rem(a2338, a2339, a2340)
                            };
                            let a2342 = 0u64;
                            if nat_eq(a2341, a2342) {
                                Adt12::C0(6u64, v17)
                            } else {
                                Adt12::C3({
                                    let a2345 = {
                                        let a2343 = 4u64;
                                        let a2344 = v6;
                                        nat_mul(a2343, a2344)?
                                    };
                                    let a2346 = v17;
                                    nat_add(a2345, a2346)?
                                })
                            }
                        }, Adt13::C0(1u64), 4u64, 4u64))
                    } else {
                        Ok(Adt9::C7(Adt11::C1(0u64), Adt12::C1(0u64), Adt12::C1(1u64), Adt13::C0(0u64), 4u64, 4u64))
                    }
                } else {
                    let v18: u64 = {
                        let a2353 = {
                            let a2351 = v10;
                            let a2352 = {
                                let a2349 = 1u64;
                                let a2350 = {
                                    let a2347 = 2u64;
                                    let a2348 = v4;
                                    nat_mul(a2347, a2348)?
                                };
                                nat_add(a2349, a2350)?
                            };
                            nat_sub(a2351, a2352)
                        };
                        let a2354 = 7u64;
                        let a2355 = 0u64;
                        nat_quot(a2353, a2354, a2355)
                    };
                    let v19: u64 = {
                        let a2362 = {
                            let a2360 = v10;
                            let a2361 = {
                                let a2358 = 1u64;
                                let a2359 = {
                                    let a2356 = 2u64;
                                    let a2357 = v4;
                                    nat_mul(a2356, a2357)?
                                };
                                nat_add(a2358, a2359)?
                            };
                            nat_sub(a2360, a2361)
                        };
                        let a2363 = 7u64;
                        let a2364 = 0u64;
                        nat_rem(a2362, a2363, a2364)
                    };
                    let v20: u64 = {
                        let a2372 = 4u64;
                        let a2373 = {
                            let a2370 = {
                                let a2368 = v9;
                                let a2369 = {
                                    let a2365 = v12;
                                    let a2366 = v4;
                                    let a2367 = 0u64;
                                    nat_rem(a2365, a2366, a2367)
                                };
                                nat_sub(a2368, a2369)
                            };
                            let a2371 = v18;
                            nat_add(a2370, a2371)?
                        };
                        nat_mul(a2372, a2373)?
                    };
                    let v21: u64 = {
                        let a2385 = 4u64;
                        let a2386 = {
                            let a2383 = {
                                let a2381 = {
                                    let a2379 = {
                                        let a2377 = {
                                            let a2374 = v11;
                                            let a2375 = v13;
                                            let a2376 = 0u64;
                                            nat_quot(a2374, a2375, a2376)
                                        };
                                        let a2378 = v5;
                                        nat_mul(a2377, a2378)?
                                    };
                                    let a2380 = v12;
                                    nat_add(a2379, a2380)?
                                };
                                let a2382 = v4;
                                nat_mul(a2381, a2382)?
                            };
                            let a2384 = v18;
                            nat_add(a2383, a2384)?
                        };
                        nat_mul(a2385, a2386)?
                    };
                    let v22: u64 = {
                        let a2389 = {
                            let a2387 = 4u64;
                            let a2388 = v6;
                            nat_mul(a2387, a2388)?
                        };
                        let a2390 = v21;
                        nat_add(a2389, a2390)?
                    };
                    let a2391 = v19;
                    let a2392 = 0u64;
                    if nat_eq(a2391, a2392) {
                        Ok(Adt9::C7(Adt11::C1(2u64), Adt12::C0(3u64, v14), Adt12::C0(2u64, v20), Adt13::C0(1u64), 4u64, 4u64))
                    } else {
                        let a2393 = v19;
                        let a2394 = 1u64;
                        if nat_eq(a2393, a2394) {
                            Ok(Adt9::C7(Adt11::C1(2u64), {
                                let a2398 = {
                                    let a2395 = v11;
                                    let a2396 = v13;
                                    let a2397 = 0u64;
                                    nat_rem(a2395, a2396, a2397)
                                };
                                let a2399 = 0u64;
                                if nat_eq(a2398, a2399) {
                                    Adt12::C0(6u64, v21)
                                } else {
                                    Adt12::C3(v22)
                                }
                            }, Adt12::C0(1u64, v20), Adt13::C0(2u64), 4u64, 4u64))
                        } else {
                            let a2400 = v19;
                            let a2401 = 2u64;
                            if nat_eq(a2400, a2401) {
                                Ok(Adt9::C7(Adt11::C1(0u64), Adt12::C1(2u64), Adt12::C1(1u64), Adt13::C0(2u64), 4u64, 4u64))
                            } else {
                                let a2402 = v19;
                                let a2403 = 3u64;
                                if nat_eq(a2402, a2403) {
                                    Ok(Adt9::C7(Adt11::C1(2u64), Adt12::C1(0u64), Adt12::C0(5u64, v20), Adt13::C0(1u64), 4u64, 4u64))
                                } else {
                                    let a2404 = v19;
                                    let a2405 = 4u64;
                                    if nat_eq(a2404, a2405) {
                                        Ok(Adt9::C7(Adt11::C1(0u64), Adt12::C1(2u64), Adt12::C1(1u64), Adt13::C1(v22), 4u64, 4u64))
                                    } else {
                                        let a2406 = v19;
                                        let a2407 = 5u64;
                                        if nat_eq(a2406, a2407) {
                                            Ok(Adt9::C7(Adt11::C1(2u64), Adt12::C3(v22), Adt12::C0(0u64, v20), Adt13::C0(1u64), 4u64, 4u64))
                                        } else {
                                            Ok(Adt9::C7(Adt11::C1(0u64), Adt12::C3(v14), Adt12::C1(1u64), Adt13::C1(v14), 4u64, 4u64))
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        } else {
            let v23: u64 = {
                let a2408 = v10;
                let a2409 = 7u64;
                let a2410 = 0u64;
                nat_quot(a2408, a2409, a2410)
            };
            let v24: u64 = {
                let a2411 = v10;
                let a2412 = 7u64;
                let a2413 = 0u64;
                nat_rem(a2411, a2412, a2413)
            };
            let v25: u64 = {
                let a2421 = 4u64;
                let a2422 = {
                    let a2419 = {
                        let a2417 = v9;
                        let a2418 = {
                            let a2414 = v12;
                            let a2415 = v4;
                            let a2416 = 0u64;
                            nat_rem(a2414, a2415, a2416)
                        };
                        nat_sub(a2417, a2418)
                    };
                    let a2420 = v23;
                    nat_add(a2419, a2420)?
                };
                nat_mul(a2421, a2422)?
            };
            let v26: u64 = {
                let a2434 = 4u64;
                let a2435 = {
                    let a2432 = {
                        let a2430 = {
                            let a2428 = {
                                let a2426 = {
                                    let a2423 = v11;
                                    let a2424 = v13;
                                    let a2425 = 0u64;
                                    nat_quot(a2423, a2424, a2425)
                                };
                                let a2427 = v5;
                                nat_mul(a2426, a2427)?
                            };
                            let a2429 = v12;
                            nat_add(a2428, a2429)?
                        };
                        let a2431 = v4;
                        nat_mul(a2430, a2431)?
                    };
                    let a2433 = v23;
                    nat_add(a2432, a2433)?
                };
                nat_mul(a2434, a2435)?
            };
            let v27: u64 = {
                let a2438 = {
                    let a2436 = 4u64;
                    let a2437 = v6;
                    nat_mul(a2436, a2437)?
                };
                let a2439 = v26;
                nat_add(a2438, a2439)?
            };
            let a2440 = v24;
            let a2441 = 0u64;
            if nat_eq(a2440, a2441) {
                Ok(Adt9::C7(Adt11::C1(2u64), Adt12::C0(1u64, v25), Adt12::C0(0u64, v14), Adt13::C0(0u64), 4u64, 4u64))
            } else {
                let a2442 = v24;
                let a2443 = 1u64;
                if nat_eq(a2442, a2443) {
                    Ok(Adt9::C7(Adt11::C1(2u64), Adt12::C1(0u64), Adt12::C0(3u64, {
                        let a2444 = 4u64;
                        let a2445 = v12;
                        nat_mul(a2444, a2445)?
                    }), Adt13::C0(1u64), 4u64, 4u64))
                } else {
                    let a2446 = v24;
                    let a2447 = 2u64;
                    if nat_eq(a2446, a2447) {
                        Ok(Adt9::C7(Adt11::C1(0u64), Adt12::C1(1u64), {
                            let a2451 = {
                                let a2448 = v11;
                                let a2449 = v13;
                                let a2450 = 0u64;
                                nat_rem(a2448, a2449, a2450)
                            };
                            let a2452 = 0u64;
                            if nat_eq(a2451, a2452) {
                                Adt12::C0(5u64, v26)
                            } else {
                                Adt12::C3(v27)
                            }
                        }, Adt13::C0(1u64), 4u64, 4u64))
                    } else {
                        let a2453 = v24;
                        let a2454 = 3u64;
                        if nat_eq(a2453, a2454) {
                            Ok(Adt9::C7(Adt11::C1(2u64), Adt12::C1(1u64), Adt12::C0(2u64, v14), Adt13::C0(1u64), 4u64, 4u64))
                        } else {
                            let a2455 = v24;
                            let a2456 = 4u64;
                            if nat_eq(a2455, a2456) {
                                Ok(Adt9::C7(Adt11::C1(0u64), Adt12::C3(v25), Adt12::C1(1u64), Adt13::C1(v25), 4u64, 4u64))
                            } else {
                                let a2457 = v24;
                                let a2458 = 5u64;
                                if nat_eq(a2457, a2458) {
                                    Ok(Adt9::C7(Adt11::C1(2u64), {
                                        let a2462 = {
                                            let a2459 = v11;
                                            let a2460 = v13;
                                            let a2461 = 0u64;
                                            nat_rem(a2459, a2460, a2461)
                                        };
                                        let a2463 = 0u64;
                                        if nat_eq(a2462, a2463) {
                                            Adt12::C0(5u64, v26)
                                        } else {
                                            Adt12::C3(v27)
                                        }
                                    }, Adt12::C0(4u64, v14), Adt13::C0(1u64), 4u64, 4u64))
                                } else {
                                    Ok(Adt9::C7(Adt11::C1(0u64), Adt12::C1(1u64), Adt12::C1(0u64), Adt13::C1(v27), 4u64, 4u64))
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn f78(v0: u64, v1: Adt3, v2: u64, v3: u64) -> R<Adt9> {
    let v4: u64 = match v1.clone() {
        Adt3::C0(h2464, _, _, _, _, _, _, _) => {
            h2464
        }
    };
    let v5: u64 = {
        let a2466 = v4;
        let a2467 = match v1.clone() {
            Adt3::C0(_, h2465, _, _, _, _, _, _) => {
                h2465
            }
        };
        nat_mul(a2466, a2467)?
    };
    let v6: u64 = {
        let a2469 = v5;
        let a2470 = match v1.clone() {
            Adt3::C0(_, _, h2468, _, _, _, _, _) => {
                h2468
            }
        };
        nat_mul(a2469, a2470)?
    };
    let a2471 = v0;
    let a2472 = v6;
    if nat_lt(a2471, a2472) {
        Ok(Adt9::C7(Adt11::C5(3u64), Adt12::C2(0u64), Adt12::C2(0u64), Adt13::C1({
            let a2473 = 4u64;
            let a2474 = v0;
            nat_mul(a2473, a2474)?
        }), 4u64, 4u64))
    } else {
        let v7: u64 = {
            let a2475 = v0;
            let a2476 = v6;
            nat_sub(a2475, a2476)
        };
        let v8: u64 = {
            let a2479 = 1u64;
            let a2480 = {
                let a2477 = 5u64;
                let a2478 = v4;
                nat_mul(a2477, a2478)?
            };
            nat_add(a2479, a2480)?
        };
        let v9: u64 = {
            let a2481 = v7;
            let a2482 = v8;
            let a2483 = 0u64;
            nat_quot(a2481, a2482, a2483)
        };
        let v10: u64 = {
            let a2484 = v7;
            let a2485 = v8;
            let a2486 = 0u64;
            nat_rem(a2484, a2485, a2486)
        };
        let a2487 = v10;
        let a2488 = 0u64;
        if nat_eq(a2487, a2488) {
            Ok(Adt9::C7(Adt11::C1(2u64), Adt12::C0(2u64, {
                let a2489 = 4u64;
                let a2490 = v9;
                nat_mul(a2489, a2490)?
            }), Adt12::C2(v2), Adt13::C0(0u64), 4u64, 4u64))
        } else {
            let v11: u64 = {
                let a2491 = v10;
                let a2492 = 1u64;
                nat_sub(a2491, a2492)
            };
            let v12: u64 = {
                let a2493 = v11;
                let a2494 = 5u64;
                let a2495 = 0u64;
                nat_quot(a2493, a2494, a2495)
            };
            let v13: u64 = {
                let a2496 = v11;
                let a2497 = 5u64;
                let a2498 = 0u64;
                nat_rem(a2496, a2497, a2498)
            };
            let v14: u64 = {
                let a2499 = v9;
                let a2500 = v5;
                let a2501 = 0u64;
                nat_quot(a2499, a2500, a2501)
            };
            let v15: u64 = {
                let a2502 = v9;
                let a2503 = v5;
                let a2504 = 0u64;
                nat_rem(a2502, a2503, a2504)
            };
            let v16: u64 = {
                let a2506 = match v1 {
                    Adt3::C0(_, _, h2505, _, _, _, _, _) => {
                        h2505
                    }
                };
                let a2507 = v3;
                let a2508 = 0u64;
                nat_quot(a2506, a2507, a2508)
            };
            let v17: u64 = {
                let a2516 = 4u64;
                let a2517 = {
                    let a2514 = {
                        let a2512 = v9;
                        let a2513 = {
                            let a2509 = v15;
                            let a2510 = v4;
                            let a2511 = 0u64;
                            nat_rem(a2509, a2510, a2511)
                        };
                        nat_sub(a2512, a2513)
                    };
                    let a2515 = v12;
                    nat_add(a2514, a2515)?
                };
                nat_mul(a2516, a2517)?
            };
            let v18: u64 = {
                let a2529 = 4u64;
                let a2530 = {
                    let a2527 = {
                        let a2525 = {
                            let a2523 = {
                                let a2521 = {
                                    let a2518 = v14;
                                    let a2519 = v16;
                                    let a2520 = 0u64;
                                    nat_quot(a2518, a2519, a2520)
                                };
                                let a2522 = v5;
                                nat_mul(a2521, a2522)?
                            };
                            let a2524 = v15;
                            nat_add(a2523, a2524)?
                        };
                        let a2526 = v4;
                        nat_mul(a2525, a2526)?
                    };
                    let a2528 = v12;
                    nat_add(a2527, a2528)?
                };
                nat_mul(a2529, a2530)?
            };
            let v19: u64 = {
                let a2533 = {
                    let a2531 = 4u64;
                    let a2532 = v6;
                    nat_mul(a2531, a2532)?
                };
                let a2534 = v18;
                nat_add(a2533, a2534)?
            };
            let a2535 = v13;
            let a2536 = 0u64;
            if nat_eq(a2535, a2536) {
                Ok(Adt9::C7(Adt11::C1(2u64), Adt12::C0(1u64, v17), Adt12::C0(0u64, {
                    let a2537 = 4u64;
                    let a2538 = v9;
                    nat_mul(a2537, a2538)?
                }), Adt13::C0(1u64), 4u64, 4u64))
            } else {
                let a2539 = v13;
                let a2540 = 1u64;
                if nat_eq(a2539, a2540) {
                    Ok(Adt9::C7(Adt11::C1(2u64), {
                        let a2544 = {
                            let a2541 = v14;
                            let a2542 = v16;
                            let a2543 = 0u64;
                            nat_rem(a2541, a2542, a2543)
                        };
                        let a2545 = 0u64;
                        if nat_eq(a2544, a2545) {
                            Adt12::C0(4u64, v18)
                        } else {
                            Adt12::C3(v19)
                        }
                    }, Adt12::C0(3u64, {
                        let a2546 = 4u64;
                        let a2547 = v9;
                        nat_mul(a2546, a2547)?
                    }), Adt13::C0(2u64), 4u64, 4u64))
                } else {
                    let a2548 = v13;
                    let a2549 = 2u64;
                    if nat_eq(a2548, a2549) {
                        Ok(Adt9::C7(Adt11::C1(0u64), Adt12::C1(2u64), Adt12::C1(1u64), Adt13::C1(v19), 4u64, 4u64))
                    } else {
                        let a2550 = v13;
                        let a2551 = 3u64;
                        if nat_eq(a2550, a2551) {
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

pub fn f79(v0: u64, v1: Adt3, v2: u64) -> R<Adt9> {
    let v3: u64 = match v1.clone() {
        Adt3::C0(h2552, _, _, _, _, _, _, _) => {
            h2552
        }
    };
    let v4: u64 = match v1 {
        Adt3::C0(_, h2553, _, _, _, _, _, _) => {
            h2553
        }
    };
    let v5: u64 = {
        let a2554 = v3;
        let a2555 = 2u64;
        let a2556 = 0u64;
        nat_quot(a2554, a2555, a2556)
    };
    let v6: u64 = {
        let a2559 = 5u64;
        let a2560 = {
            let a2557 = 3u64;
            let a2558 = v4;
            nat_mul(a2557, a2558)?
        };
        nat_add(a2559, a2560)?
    };
    let v7: u64 = {
        let a2561 = v5;
        let a2562 = 0u64;
        if nat_eq(a2561, a2562) {
            0u64
        } else {
            let a2565 = 7u64;
            let a2566 = {
                let a2563 = v5;
                let a2564 = v6;
                nat_mul(a2563, a2564)?
            };
            nat_add(a2565, a2566)?
        }
    };
    let a2567 = v7;
    let a2568 = v0;
    if nat_le(a2567, a2568) {
        Ok(Adt9::C7(Adt11::C5(3u64), Adt12::C2(0u64), Adt12::C2(0u64), Adt13::C1({
            let a2577 = 4u64;
            let a2578 = {
                let a2575 = {
                    let a2571 = {
                        let a2569 = v0;
                        let a2570 = v7;
                        nat_sub(a2569, a2570)
                    };
                    let a2572 = v3;
                    nat_mul(a2571, a2572)?
                };
                let a2576 = {
                    let a2573 = 2u64;
                    let a2574 = v5;
                    nat_mul(a2573, a2574)?
                };
                nat_add(a2575, a2576)?
            };
            nat_mul(a2577, a2578)?
        }), 4u64, 4u64))
    } else {
        let a2579 = v0;
        let a2580 = 7u64;
        if nat_lt(a2579, a2580) {
            let a2581 = v0;
            let a2582 = 0u64;
            if nat_eq(a2581, a2582) {
                Ok(Adt9::C7(Adt11::C0(5u64), Adt12::C2({
                    let a2583 = f156(v2);
                    let a2584 = f155(v2);
                    nat_add(a2583, a2584)?
                }), Adt12::C2(0u64), Adt13::C0(0u64), 4u64, 4u64))
            } else {
                let a2585 = v0;
                let a2586 = 1u64;
                if nat_eq(a2585, a2586) {
                    Ok(Adt9::C7(Adt11::C0(2u64), Adt12::C1(0u64), Adt12::C2(0u64), Adt13::C0(0u64), 4u64, 4u64))
                } else {
                    let a2587 = v0;
                    let a2588 = 2u64;
                    if nat_eq(a2587, a2588) {
                        let a2589 = v2;
                        let a2590 = 2147483648u64;
                        if nat_lt(a2589, a2590) {
                            Ok(Adt9::C7(Adt11::C5(3u64), Adt12::C1(0u64), Adt12::C2(0u64), Adt13::C0(0u64), 4u64, 4u64))
                        } else {
                            Ok(Adt9::C7(Adt11::C1(6u64), Adt12::C1(0u64), Adt12::C2(0u64), Adt13::C0(0u64), 4u64, 4u64))
                        }
                    } else {
                        let a2591 = v0;
                        let a2592 = 3u64;
                        if nat_eq(a2591, a2592) {
                            Ok(Adt9::C7(Adt11::C7(13u64, 0u64, 0u64), Adt12::C1(0u64), Adt12::C2(0u64), Adt13::C0(0u64), 4u64, 4u64))
                        } else {
                            let a2593 = v0;
                            let a2594 = 4u64;
                            if nat_eq(a2593, a2594) {
                                Ok(Adt9::C7(Adt11::C1(6u64), Adt12::C1(0u64), Adt12::C2(0u64), Adt13::C0(0u64), 4u64, 4u64))
                            } else {
                                let a2595 = v0;
                                let a2596 = 5u64;
                                if nat_eq(a2595, a2596) {
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
                let a2597 = v0;
                let a2598 = 7u64;
                nat_sub(a2597, a2598)
            };
            let v9: u64 = {
                let a2599 = v8;
                let a2600 = v6;
                let a2601 = 0u64;
                nat_quot(a2599, a2600, a2601)
            };
            let v10: u64 = {
                let a2602 = v8;
                let a2603 = v6;
                let a2604 = 0u64;
                nat_rem(a2602, a2603, a2604)
            };
            let a2605 = v10;
            let a2606 = 5u64;
            if nat_lt(a2605, a2606) {
                let a2607 = v10;
                let a2608 = 0u64;
                if nat_eq(a2607, a2608) {
                    Ok(Adt9::C7(Adt11::C0(5u64), Adt12::C2(v9), Adt12::C2(0u64), Adt13::C0(3u64), 4u64, 4u64))
                } else {
                    let a2609 = v10;
                    let a2610 = 1u64;
                    if nat_eq(a2609, a2610) {
                        Ok(Adt9::C7(Adt11::C0(2u64), Adt12::C1(3u64), Adt12::C2(0u64), Adt13::C0(3u64), 4u64, 4u64))
                    } else {
                        let a2611 = v10;
                        let a2612 = 2u64;
                        if nat_eq(a2611, a2612) {
                            Ok(Adt9::C7(Adt11::C1(2u64), Adt12::C1(0u64), Adt12::C1(3u64), Adt13::C0(3u64), 4u64, 4u64))
                        } else {
                            let a2613 = v10;
                            let a2614 = 3u64;
                            if nat_eq(a2613, a2614) {
                                Ok(Adt9::C7(Adt11::C1(3u64), Adt12::C1(3u64), Adt12::C1(1u64), Adt13::C0(3u64), 4u64, 4u64))
                            } else {
                                Ok(Adt9::C7(Adt11::C7(16u64, 0u64, 0u64), Adt12::C1(3u64), Adt12::C2(0u64), Adt13::C0(3u64), 4u64, 4u64))
                            }
                        }
                    }
                }
            } else {
                let v11: u64 = {
                    let a2615 = v10;
                    let a2616 = 5u64;
                    nat_sub(a2615, a2616)
                };
                let v12: u64 = {
                    let a2617 = v11;
                    let a2618 = 3u64;
                    let a2619 = 0u64;
                    nat_quot(a2617, a2618, a2619)
                };
                let v13: u64 = {
                    let a2620 = v11;
                    let a2621 = 3u64;
                    let a2622 = 0u64;
                    nat_rem(a2620, a2621, a2622)
                };
                let a2623 = v13;
                let a2624 = 0u64;
                if nat_eq(a2623, a2624) {
                    Ok(Adt9::C7(Adt11::C1(2u64), Adt12::C0(0u64, {
                        let a2625 = 4u64;
                        let a2626 = v12;
                        nat_mul(a2625, a2626)?
                    }), Adt12::C1(3u64), Adt13::C0(4u64), 4u64, 4u64))
                } else {
                    let a2627 = v13;
                    let a2628 = 1u64;
                    if nat_eq(a2627, a2628) {
                        Ok(Adt9::C7(Adt11::C7(15u64, 0u64, 0u64), Adt12::C1(4u64), Adt12::C2(0u64), Adt13::C1({
                            let a2633 = 4u64;
                            let a2634 = {
                                let a2631 = {
                                    let a2629 = v12;
                                    let a2630 = v3;
                                    nat_mul(a2629, a2630)?
                                };
                                let a2632 = v9;
                                nat_add(a2631, a2632)?
                            };
                            nat_mul(a2633, a2634)?
                        }), 4u64, 4u64))
                    } else {
                        Ok(Adt9::C7(Adt11::C7(14u64, 0u64, 0u64), Adt12::C1(4u64), Adt12::C2(0u64), Adt13::C1({
                            let a2641 = 4u64;
                            let a2642 = {
                                let a2639 = {
                                    let a2635 = v12;
                                    let a2636 = v3;
                                    nat_mul(a2635, a2636)?
                                };
                                let a2640 = {
                                    let a2637 = v5;
                                    let a2638 = v9;
                                    nat_add(a2637, a2638)?
                                };
                                nat_add(a2639, a2640)?
                            };
                            nat_mul(a2641, a2642)?
                        }), 4u64, 4u64))
                    }
                }
            }
        }
    }
}

pub fn f80(v0: u64, v1: Adt3, v2: Adt3, v3: Adt7, v4: u64, v5: u64) -> R<Adt9> {
    let a2643 = v0;
    let a2644 = v4;
    if nat_lt(a2643, a2644) {
        Ok(Adt9::C7(Adt11::C5(3u64), Adt12::C2(0u64), Adt12::C2(0u64), Adt13::C1(f103(v0, v2)?), 4u64, 4u64))
    } else {
        let v6: u64 = {
            let a2651 = {
                let m2647 = match v3.clone() {
                    Adt7::C0(h2646, _, _, _, _) => {
                        h2646
                    }
                };
                match m2647 {
                    Adt6::C0(h2645, _, _, _) => {
                        h2645
                    }
                }
            };
            let a2652 = {
                let m2650 = match v3.clone() {
                    Adt7::C0(_, h2649, _, _, _) => {
                        h2649
                    }
                };
                match m2650 {
                    Adt6::C0(h2648, _, _, _) => {
                        h2648
                    }
                }
            };
            nat_mul(a2651, a2652)?
        };
        let v7: u64 = f184(match v3.clone() {
            Adt7::C0(_, _, _, h2653, _) => {
                h2653
            }
        }, v6)?;
        let v8: u64 = {
            let a2654 = v0;
            let a2655 = v4;
            nat_sub(a2654, a2655)
        };
        let v9: u64 = {
            let a2656 = v8;
            let a2657 = v7;
            let a2658 = 0u64;
            nat_quot(a2656, a2657, a2658)
        };
        let v10: u64 = {
            let a2659 = v8;
            let a2660 = v7;
            let a2661 = 0u64;
            nat_rem(a2659, a2660, a2661)
        };
        if match v3.clone() {
            Adt7::C0(_, _, _, h2662, _) => {
                h2662
            }
        } {
            let a2663 = v10;
            let a2664 = 0u64;
            if nat_eq(a2663, a2664) {
                Ok(Adt9::C7(Adt11::C5(3u64), Adt12::C2(4286578687u64), Adt12::C2(0u64), Adt13::C0(0u64), 4u64, 4u64))
            } else {
                let a2665 = v10;
                let a2666 = 1u64;
                if nat_eq(a2665, a2666) {
                    Ok(Adt9::C7(Adt11::C5(3u64), Adt12::C2(0u64), Adt12::C2(0u64), Adt13::C0(1u64), 4u64, 4u64))
                } else {
                    let a2669 = {
                        let a2667 = v10;
                        let a2668 = 2u64;
                        nat_add(a2667, a2668)?
                    };
                    let a2670 = v7;
                    if nat_eq(a2669, a2670) {
                        Ok(Adt9::C7(Adt11::C5(3u64), Adt12::C1(1u64), Adt12::C2(0u64), Adt13::C0(2u64), 4u64, 4u64))
                    } else {
                        let a2673 = {
                            let a2671 = v10;
                            let a2672 = 1u64;
                            nat_add(a2671, a2672)?
                        };
                        let a2674 = v7;
                        if nat_eq(a2673, a2674) {
                            let a2675 = v5;
                            let a2676 = 0u64;
                            if nat_eq(a2675, a2676) {
                                Ok(Adt9::C7(Adt11::C5(3u64), Adt12::C1(0u64), Adt12::C2(0u64), Adt13::C0(0u64), 4u64, 4u64))
                            } else {
                                Ok(Adt9::C7(Adt11::C1(0u64), Adt12::C3({
                                    let a2677 = v5;
                                    let a2678 = 1u64;
                                    nat_sub(a2677, a2678)
                                }), Adt12::C0(0u64, f103(v9, v1)?), Adt13::C1({
                                    let a2679 = v5;
                                    let a2680 = 1u64;
                                    nat_sub(a2679, a2680)
                                }), 4u64, 4u64))
                            }
                        } else {
                            let v11: u64 = {
                                let a2681 = v10;
                                let a2682 = 2u64;
                                nat_sub(a2681, a2682)
                            };
                            let v12: u64 = {
                                let a2683 = v11;
                                let a2684 = 3u64;
                                let a2685 = 0u64;
                                nat_rem(a2683, a2684, a2685)
                            };
                            let m2689 = f185(v3, v2, v1, v9, {
                                let a2686 = v11;
                                let a2687 = 3u64;
                                let a2688 = 0u64;
                                nat_quot(a2686, a2687, a2688)
                            })?;
                            match m2689 {
                                None => {
                                    Ok(Adt9::C7(Adt11::C5(3u64), Adt12::C1(0u64), Adt12::C2(0u64), Adt13::C0(0u64), 4u64, 4u64))
                                }
                                Some(v13) => {
                                    let a2690 = v12;
                                    let a2691 = 0u64;
                                    if nat_eq(a2690, a2691) {
                                        Ok(Adt9::C7(Adt11::C5(0u64), Adt12::C0(1u64, v13), Adt12::C1(0u64), Adt13::C0(2u64), 4u64, 4u64))
                                    } else {
                                        let a2692 = 2u64;
                                        let a2693 = v5;
                                        if nat_le(a2692, a2693) {
                                            let a2694 = v12;
                                            let a2695 = 1u64;
                                            if nat_eq(a2694, a2695) {
                                                Ok(Adt9::C7(Adt11::C5(3u64), Adt12::C0(1u64, v13), Adt12::C2(0u64), Adt13::C0(0u64), 4u64, 4u64))
                                            } else {
                                                Ok(Adt9::C7(Adt11::C5(3u64), Adt12::C2({
                                                    let a2696 = v13;
                                                    let a2697 = 1u64;
                                                    nat_add(a2696, a2697)?
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
            let a2698 = v10;
            let a2699 = 0u64;
            if nat_eq(a2698, a2699) {
                Ok(Adt9::C7(Adt11::C0(5u64), Adt12::C2(v6), Adt12::C2(0u64), Adt13::C0(0u64), 4u64, 4u64))
            } else {
                let a2700 = v10;
                let a2701 = 1u64;
                if nat_eq(a2700, a2701) {
                    Ok(Adt9::C7(Adt11::C0(2u64), Adt12::C1(0u64), Adt12::C2(0u64), Adt13::C0(0u64), 4u64, 4u64))
                } else {
                    let a2702 = v10;
                    let a2703 = 2u64;
                    if nat_eq(a2702, a2703) {
                        Ok(Adt9::C7(Adt11::C1(3u64), Adt12::C0(0u64, f103(v9, v1)?), Adt12::C1(0u64), Adt13::C0(0u64), 4u64, 4u64))
                    } else {
                        let m2706 = f185(v3, v2, v1, v9, {
                            let a2704 = v10;
                            let a2705 = 3u64;
                            nat_sub(a2704, a2705)
                        })?;
                        match m2706 {
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

pub fn f81(v0: u64, v1: Adt3, v2: Adt3, v3: Adt6, v4: Adt6) -> R<Adt9> {
    let v5: u64 = {
        let a2709 = match v3.clone() {
            Adt6::C0(h2707, _, _, _) => {
                h2707
            }
        };
        let a2710 = match v4.clone() {
            Adt6::C0(h2708, _, _, _) => {
                h2708
            }
        };
        nat_mul(a2709, a2710)?
    };
    let v6: u64 = {
        let a2711 = v5;
        let a2712 = 2u64;
        nat_add(a2711, a2712)?
    };
    let v7: u64 = {
        let a2713 = v0;
        let a2714 = v6;
        let a2715 = 0u64;
        nat_quot(a2713, a2714, a2715)
    };
    let v8: u64 = {
        let a2716 = v0;
        let a2717 = v6;
        let a2718 = 0u64;
        nat_rem(a2716, a2717, a2718)
    };
    let a2719 = v8;
    let a2720 = 0u64;
    if nat_eq(a2719, a2720) {
        Ok(Adt9::C7(Adt11::C5(3u64), Adt12::C2(0u64), Adt12::C2(0u64), Adt13::C0(0u64), 4u64, 4u64))
    } else {
        let a2723 = {
            let a2721 = v8;
            let a2722 = 1u64;
            nat_add(a2721, a2722)?
        };
        let a2724 = v6;
        if nat_eq(a2723, a2724) {
            Ok(Adt9::C7(Adt11::C5(3u64), Adt12::C1(0u64), Adt12::C2(0u64), Adt13::C1(f103(v7, v2)?), 4u64, 4u64))
        } else {
            let v9: u64 = {
                let a2725 = v8;
                let a2726 = 1u64;
                nat_sub(a2725, a2726)
            };
            let m2732 = f186(v3.clone(), match v1.clone() {
                Adt3::C0(_, h2727, _, _, _, _, _, _) => {
                    h2727
                }
            }, f63(v7, v2.clone(), 0u64), {
                let a2729 = v9;
                let a2730 = match v3.clone() {
                    Adt6::C0(h2728, _, _, _) => {
                        h2728
                    }
                };
                let a2731 = 0u64;
                nat_rem(a2729, a2730, a2731)
            })?;
            match m2732 {
                None => {
                    Ok(Adt9::C7(Adt11::C5(3u64), Adt12::C1(0u64), Adt12::C2(0u64), Adt13::C0(0u64), 4u64, 4u64))
                }
                Some(v10) => {
                    let m2738 = f186(v4, match v1.clone() {
                        Adt3::C0(_, _, h2733, _, _, _, _, _) => {
                            h2733
                        }
                    }, f63(v7, v2.clone(), 1u64), {
                        let a2735 = v9;
                        let a2736 = match v3 {
                            Adt6::C0(h2734, _, _, _) => {
                                h2734
                            }
                        };
                        let a2737 = 0u64;
                        nat_quot(a2735, a2736, a2737)
                    })?;
                    match m2738 {
                        None => {
                            Ok(Adt9::C7(Adt11::C5(3u64), Adt12::C1(0u64), Adt12::C2(0u64), Adt13::C0(0u64), 4u64, 4u64))
                        }
                        Some(v11) => {
                            Ok(Adt9::C7(Adt11::C1(0u64), Adt12::C1(0u64), Adt12::C0(0u64, f62(v1, {
                                let a2741 = {
                                    let a2739 = f63(v7, v2.clone(), 2u64);
                                    let a2740 = v5;
                                    nat_mul(a2739, a2740)?
                                };
                                let a2742 = v9;
                                nat_add(a2741, a2742)?
                            }, v10, v11, f63(v7, v2, 3u64))?), Adt13::C0(0u64), 4u64, 4u64))
                        }
                    }
                }
            }
        }
    }
}

pub fn f82(v0: u64, v1: Adt3, v2: Adt3, v3: bool, v4: u64, v5: u64, v6: Adt1, v7: u64) -> R<Adt9> {
    let v8: u64 = f187(v3, v5)?;
    let v9: u64 = {
        let a2743 = v0;
        let a2744 = v8;
        let a2745 = 0u64;
        nat_rem(a2743, a2744, a2745)
    };
    let v10: u64 = {
        let a2746 = v0;
        let a2747 = v8;
        let a2748 = 0u64;
        nat_quot(a2746, a2747, a2748)
    };
    let v11: Adt13 = Adt13::C1(f103(v10, v2)?);
    let v12: Adt13 = {
        let a2749 = v5;
        let a2750 = 0u64;
        if nat_eq(a2749, a2750) {
            v11.clone()
        } else {
            Adt13::C0(0u64)
        }
    };
    let a2751 = v9;
    let a2752 = 0u64;
    if nat_eq(a2751, a2752) {
        Ok(Adt9::C7(Adt11::C2(v4, 0u64, true), Adt12::C0(0u64, f103(v10, v1)?), Adt12::C2(0u64), Adt13::C0(0u64), f188(v4), 4u64))
    } else {
        let a2753 = v9;
        let a2754 = 1u64;
        if nat_eq(a2753, a2754) {
            Ok(Adt9::C7(Adt11::C5(0u64), Adt12::C1(0u64), Adt12::C2(0u64), Adt13::C0(2u64), 4u64, 4u64))
        } else {
            let a2755 = v9;
            let a2756 = if v3 {
                8u64
            } else {
                3u64
            };
            if nat_eq(a2755, a2756) {
                Ok(Adt9::C7(Adt11::C2(0u64, v5, true), Adt12::C1(0u64), Adt12::C2(0u64), v11, 4u64, f188(v5)))
            } else {
                if v3 {
                    Ok(f189(v9, v7, v6, v12))
                } else {
                    Ok(Adt9::C7({
                        let a2757 = v7;
                        let a2758 = 2u64;
                        if nat_eq(a2757, a2758) {
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

pub fn f83(v0: u64, v1: Adt3, v2: Adt3, v3: u64) -> R<Adt9> {
    let v4: u64 = match v1 {
        Adt3::C0(h2759, _, _, _, _, _, _, _) => {
            h2759
        }
    };
    let v5: u64 = match v2 {
        Adt3::C0(h2760, _, _, _, _, _, _, _) => {
            h2760
        }
    };
    let v6: u64 = f190(v4)?;
    let v7: u64 = {
        let a2761 = v0;
        let a2762 = v6;
        let a2763 = 0u64;
        nat_quot(a2761, a2762, a2763)
    };
    let v8: u64 = {
        let a2764 = v7;
        let a2765 = v5;
        let a2766 = 0u64;
        nat_quot(a2764, a2765, a2766)
    };
    let v9: u64 = {
        let a2767 = v7;
        let a2768 = v5;
        let a2769 = 0u64;
        nat_rem(a2767, a2768, a2769)
    };
    let v10: u64 = {
        let a2770 = v3;
        let a2771 = v4;
        let a2772 = 0u64;
        nat_rem(a2770, a2771, a2772)
    };
    let v11: u64 = {
        let a2777 = 4u64;
        let a2778 = {
            let a2775 = v8;
            let a2776 = {
                let a2773 = v4;
                let a2774 = v4;
                nat_mul(a2773, a2774)?
            };
            nat_mul(a2775, a2776)?
        };
        nat_mul(a2777, a2778)?
    };
    let v12: u64 = {
        let a2783 = 4u64;
        let a2784 = {
            let a2781 = v8;
            let a2782 = {
                let a2779 = v4;
                let a2780 = v5;
                nat_mul(a2779, a2780)?
            };
            nat_mul(a2781, a2782)?
        };
        nat_mul(a2783, a2784)?
    };
    let v13: u64 = {
        let a2788 = {
            let a2785 = v0;
            let a2786 = v6;
            let a2787 = 0u64;
            nat_rem(a2785, a2786, a2787)
        };
        let a2789 = f190(v10)?;
        nat_sub(a2788, a2789)
    };
    let a2792 = v13;
    let a2793 = {
        let a2790 = v10;
        let a2791 = 2u64;
        nat_mul(a2790, a2791)?
    };
    if nat_lt(a2792, a2793) {
        let v14: u64 = {
            let a2794 = v13;
            let a2795 = 2u64;
            let a2796 = 0u64;
            nat_quot(a2794, a2795, a2796)
        };
        let a2800 = {
            let a2797 = v13;
            let a2798 = 2u64;
            let a2799 = 0u64;
            nat_rem(a2797, a2798, a2799)
        };
        let a2801 = 0u64;
        if nat_eq(a2800, a2801) {
            Ok(Adt9::C7(Adt11::C1(2u64), Adt12::C0(0u64, {
                let a2808 = v11;
                let a2809 = {
                    let a2806 = 4u64;
                    let a2807 = {
                        let a2804 = {
                            let a2802 = v10;
                            let a2803 = v4;
                            nat_mul(a2802, a2803)?
                        };
                        let a2805 = v14;
                        nat_add(a2804, a2805)?
                    };
                    nat_mul(a2806, a2807)?
                };
                nat_add(a2808, a2809)?
            }), Adt12::C3({
                let a2816 = v12;
                let a2817 = {
                    let a2814 = 4u64;
                    let a2815 = {
                        let a2812 = {
                            let a2810 = v14;
                            let a2811 = v5;
                            nat_mul(a2810, a2811)?
                        };
                        let a2813 = v9;
                        nat_add(a2812, a2813)?
                    };
                    nat_mul(a2814, a2815)?
                };
                nat_add(a2816, a2817)?
            }), Adt13::C0(1u64), 4u64, 4u64))
        } else {
            Ok(Adt9::C7(Adt11::C1(0u64), {
                let a2818 = v14;
                let a2819 = 0u64;
                if nat_eq(a2818, a2819) {
                    Adt12::C2(0u64)
                } else {
                    Adt12::C1(0u64)
                }
            }, Adt12::C1(1u64), Adt13::C0(0u64), 4u64, 4u64))
        }
    } else {
        let a2822 = v13;
        let a2823 = {
            let a2820 = v10;
            let a2821 = 2u64;
            nat_mul(a2820, a2821)?
        };
        if nat_eq(a2822, a2823) {
            Ok(Adt9::C7(Adt11::C1(1u64), Adt12::C0(1u64, {
                let a2830 = v12;
                let a2831 = {
                    let a2828 = 4u64;
                    let a2829 = {
                        let a2826 = {
                            let a2824 = v10;
                            let a2825 = v5;
                            nat_mul(a2824, a2825)?
                        };
                        let a2827 = v9;
                        nat_add(a2826, a2827)?
                    };
                    nat_mul(a2828, a2829)?
                };
                nat_add(a2830, a2831)?
            }), {
                let a2832 = v10;
                let a2833 = 0u64;
                if nat_eq(a2832, a2833) {
                    Adt12::C2(0u64)
                } else {
                    Adt12::C1(0u64)
                }
            }, Adt13::C0(0u64), 4u64, 4u64))
        } else {
            let a2838 = v13;
            let a2839 = {
                let a2836 = {
                    let a2834 = v10;
                    let a2835 = 2u64;
                    nat_mul(a2834, a2835)?
                };
                let a2837 = 1u64;
                nat_add(a2836, a2837)?
            };
            if nat_eq(a2838, a2839) {
                Ok(Adt9::C7(Adt11::C1(3u64), Adt12::C1(0u64), Adt12::C0(0u64, {
                    let a2846 = v11;
                    let a2847 = {
                        let a2844 = 4u64;
                        let a2845 = {
                            let a2842 = {
                                let a2840 = v10;
                                let a2841 = v4;
                                nat_mul(a2840, a2841)?
                            };
                            let a2843 = v10;
                            nat_add(a2842, a2843)?
                        };
                        nat_mul(a2844, a2845)?
                    };
                    nat_add(a2846, a2847)?
                }), Adt13::C1({
                    let a2854 = v12;
                    let a2855 = {
                        let a2852 = 4u64;
                        let a2853 = {
                            let a2850 = {
                                let a2848 = v10;
                                let a2849 = v5;
                                nat_mul(a2848, a2849)?
                            };
                            let a2851 = v9;
                            nat_add(a2850, a2851)?
                        };
                        nat_mul(a2852, a2853)?
                    };
                    nat_add(a2854, a2855)?
                }), 4u64, 4u64))
            } else {
                Ok(Adt9::C7(Adt11::C5(3u64), Adt12::C2({
                    let a2858 = {
                        let a2856 = v10;
                        let a2857 = 1u64;
                        nat_add(a2856, a2857)?
                    };
                    let a2859 = v4;
                    if nat_eq(a2858, a2859) {
                        0u64
                    } else {
                        let a2860 = v10;
                        let a2861 = 1u64;
                        nat_add(a2860, a2861)?
                    }
                }), Adt12::C2(0u64), Adt13::C0(2u64), 4u64, 4u64))
            }
        }
    }
}

pub fn f84(v0: u64, v1: u64, v2: Adt3, v3: Adt3) -> R<Adt9> {
    let a2862 = v0;
    let a2863 = v1;
    if nat_lt(a2862, a2863) {
        Ok(Adt9::C7(Adt11::C9, Adt12::C0(0u64, f103(v0, v2)?), Adt12::C2(0u64), Adt13::C0(v0), 4u64, 4u64))
    } else {
        let v4: u64 = {
            let a2864 = v0;
            let a2865 = v1;
            nat_sub(a2864, a2865)
        };
        Ok(Adt9::C7(Adt11::C5(3u64), Adt12::C1(v4), Adt12::C2(0u64), Adt13::C1(f103(v4, v3)?), 4u64, 4u64))
    }
}

pub fn f85(v0: u64, v1: Adt3, v2: Adt3, v3: Adt3, v4: Adt6, v5: Adt6, v6: u64) -> R<Adt9> {
    let v7: u64 = {
        let a2872 = {
            let a2870 = 3u64;
            let a2871 = {
                let a2868 = match v4.clone() {
                    Adt6::C0(h2866, _, _, _) => {
                        h2866
                    }
                };
                let a2869 = match v5.clone() {
                    Adt6::C0(h2867, _, _, _) => {
                        h2867
                    }
                };
                nat_mul(a2868, a2869)?
            };
            nat_mul(a2870, a2871)?
        };
        let a2873 = 2u64;
        nat_add(a2872, a2873)?
    };
    let v8: u64 = {
        let a2874 = v0;
        let a2875 = v7;
        let a2876 = 0u64;
        nat_quot(a2874, a2875, a2876)
    };
    let v9: u64 = {
        let a2877 = v0;
        let a2878 = v7;
        let a2879 = 0u64;
        nat_rem(a2877, a2878, a2879)
    };
    let a2880 = v9;
    let a2881 = 0u64;
    if nat_eq(a2880, a2881) {
        Ok(Adt9::C7(Adt11::C2(0u64, 0u64, false), Adt12::C2(0u64), Adt12::C2(0u64), Adt13::C0(0u64), 4u64, 4u64))
    } else {
        let a2884 = {
            let a2882 = v9;
            let a2883 = 1u64;
            nat_add(a2882, a2883)?
        };
        let a2885 = v7;
        if nat_eq(a2884, a2885) {
            Ok(Adt9::C7(Adt11::C2(0u64, 0u64, false), Adt12::C1(0u64), Adt12::C2(0u64), Adt13::C1(f103(v8, v3)?), 4u64, 4u64))
        } else {
            let v10: u64 = {
                let a2886 = v9;
                let a2887 = 1u64;
                nat_sub(a2886, a2887)
            };
            let v11: u64 = {
                let a2888 = v10;
                let a2889 = 3u64;
                let a2890 = 0u64;
                nat_quot(a2888, a2889, a2890)
            };
            let v12: u64 = {
                let a2892 = v11;
                let a2893 = match v4.clone() {
                    Adt6::C0(h2891, _, _, _) => {
                        h2891
                    }
                };
                let a2894 = 0u64;
                nat_rem(a2892, a2893, a2894)
            };
            let v13: u64 = {
                let a2896 = v11;
                let a2897 = match v4.clone() {
                    Adt6::C0(h2895, _, _, _) => {
                        h2895
                    }
                };
                let a2898 = 0u64;
                nat_quot(a2896, a2897, a2898)
            };
            let v14: u64 = {
                let a2899 = v10;
                let a2900 = 3u64;
                let a2901 = 0u64;
                nat_rem(a2899, a2900, a2901)
            };
            let v15: u64 = f63(v8, v3.clone(), 2u64);
            let m2903 = f65(v4, match v2.clone() {
                Adt3::C0(h2902, _, _, _, _, _, _, _) => {
                    h2902
                }
            }, f63(v8, v3.clone(), 0u64), v12)?;
            match m2903 {
                None => {
                    Ok(Adt9::C7(Adt11::C2(0u64, 0u64, false), Adt12::C1(0u64), Adt12::C2(0u64), Adt13::C0(0u64), 4u64, 4u64))
                }
                Some(v16) => {
                    let m2905 = f65(v5, match v2.clone() {
                        Adt3::C0(_, h2904, _, _, _, _, _, _) => {
                            h2904
                        }
                    }, f63(v8, v3.clone(), 1u64), v13)?;
                    match m2905 {
                        None => {
                            Ok(Adt9::C7(Adt11::C2(0u64, 0u64, false), Adt12::C1(0u64), Adt12::C2(0u64), Adt13::C0(0u64), 4u64, 4u64))
                        }
                        Some(v17) => {
                            let a2906 = v14;
                            let a2907 = 0u64;
                            if nat_eq(a2906, a2907) {
                                Ok(Adt9::C7(Adt11::C2(v6, 0u64, false), Adt12::C0(0u64, f62(v1, v12, v13, 0u64, v15)?), Adt12::C2(0u64), Adt13::C0(1u64), f188(v6), 4u64))
                            } else {
                                let a2908 = v14;
                                let a2909 = 1u64;
                                if nat_eq(a2908, a2909) {
                                    Ok(Adt9::C7(Adt11::C1(2u64), Adt12::C1(1u64), Adt12::C0(1u64, f62(v2, v16, v17, v15, f63(v8, v3, 3u64))?), Adt13::C0(1u64), 4u64, 4u64))
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

pub fn f86(v0: u64, v1: Adt3, v2: Adt3, v3: u64) -> R<Adt9> {
    let v4: u64 = f191(match v1.clone() {
        Adt3::C0(h2910, _, _, _, _, _, _, _) => {
            h2910
        }
    })?;
    let v5: u64 = {
        let a2911 = v0;
        let a2912 = v4;
        let a2913 = 0u64;
        nat_quot(a2911, a2912, a2913)
    };
    let v6: u64 = {
        let a2914 = v0;
        let a2915 = v4;
        let a2916 = 0u64;
        nat_rem(a2914, a2915, a2916)
    };
    let a2917 = v6;
    let a2918 = 0u64;
    if nat_eq(a2917, a2918) {
        Ok(Adt9::C4(19u64, 2u64, 4286578688u64, 2u64, 0u64, 1u64, 0u64))
    } else {
        let a2919 = v6;
        let a2920 = 1u64;
        if nat_eq(a2919, a2920) {
            Ok(Adt9::C4(19u64, 2u64, 0u64, 2u64, 0u64, 1u64, 1u64))
        } else {
            let a2923 = {
                let a2921 = v6;
                let a2922 = 1u64;
                nat_add(a2921, a2922)?
            };
            let a2924 = v4;
            if nat_eq(a2923, a2924) {
                Ok(Adt9::C4(19u64, 1u64, 1u64, 2u64, 0u64, 3u64, f103(v5, v2)?))
            } else {
                let v7: u64 = {
                    let a2925 = v6;
                    let a2926 = 2u64;
                    nat_sub(a2925, a2926)
                };
                let v8: u64 = {
                    let a2927 = v7;
                    let a2928 = 3u64;
                    let a2929 = 0u64;
                    nat_quot(a2927, a2928, a2929)
                };
                let v9: u64 = {
                    let a2930 = v7;
                    let a2931 = 3u64;
                    let a2932 = 0u64;
                    nat_rem(a2930, a2931, a2932)
                };
                let v10: u64 = f103({
                    let a2936 = {
                        let a2934 = v5;
                        let a2935 = match v1.clone() {
                            Adt3::C0(h2933, _, _, _, _, _, _, _) => {
                                h2933
                            }
                        };
                        nat_mul(a2934, a2935)?
                    };
                    let a2937 = v8;
                    nat_add(a2936, a2937)?
                }, v1)?;
                let a2938 = v9;
                let a2939 = 0u64;
                if nat_eq(a2938, a2939) {
                    Ok(Adt9::C4(16u64, 1u64, 0u64, 0u64, v10, 1u64, 2u64))
                } else {
                    let a2940 = v9;
                    let a2941 = 1u64;
                    if nat_eq(a2940, a2941) {
                        let a2942 = v3;
                        let a2943 = 2u64;
                        if nat_eq(a2942, a2943) {
                            Ok(Adt9::C4(19u64, 1u64, 0u64, 2u64, 0u64, 1u64, 0u64))
                        } else {
                            Ok(Adt9::C4(19u64, 0u64, v10, 2u64, 0u64, 1u64, 0u64))
                        }
                    } else {
                        let a2944 = v3;
                        let a2945 = 2u64;
                        if if nat_eq(a2944, a2945) {
                            true
                        } else {
                            let a2946 = v3;
                            let a2947 = 3u64;
                            nat_eq(a2946, a2947)
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

pub fn f87(v0: u64, v1: u64, v2: Adt3, v3: Adt3, v4: Adt3, v5: u64) -> R<Adt9> {
    let v6: u64 = {
        let a2949 = 2u64;
        let a2950 = match v2.clone() {
            Adt3::C0(_, h2948, _, _, _, _, _, _) => {
                h2948
            }
        };
        nat_mul(a2949, a2950)?
    };
    let a2951 = v1;
    let a2952 = v6;
    if nat_lt(a2951, a2952) {
        let a2956 = {
            let a2953 = v1;
            let a2954 = 2u64;
            let a2955 = 0u64;
            nat_rem(a2953, a2954, a2955)
        };
        let a2957 = 0u64;
        if nat_eq(a2956, a2957) {
            Ok(Adt9::C4(19u64, 4u64, f103({
                let a2958 = v1;
                let a2959 = 2u64;
                let a2960 = 0u64;
                nat_quot(a2958, a2959, a2960)
            }, v4)?, 2u64, 0u64, 1u64, 2u64))
        } else {
            let a2961 = v5;
            let a2962 = 2147483648u64;
            if if nat_lt(a2961, a2962) {
                let a2964 = v5;
                let a2965 = match v3 {
                    Adt3::C0(_, h2963, _, _, _, _, _, _) => {
                        h2963
                    }
                };
                nat_lt(a2964, a2965)
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
            let a2968 = match v3.clone() {
                Adt3::C0(h2966, _, _, _, _, _, _, _) => {
                    h2966
                }
            };
            let a2969 = match v3.clone() {
                Adt3::C0(_, h2967, _, _, _, _, _, _) => {
                    h2967
                }
            };
            nat_mul(a2968, a2969)?
        };
        let a2972 = {
            let a2970 = v1;
            let a2971 = v6;
            nat_sub(a2970, a2971)
        };
        let a2973 = v7;
        if nat_lt(a2972, a2973) {
            Ok(Adt9::C4(19u64, 2u64, 0u64, 2u64, 0u64, 3u64, f103({
                let a2974 = v1;
                let a2975 = v6;
                nat_sub(a2974, a2975)
            }, v3)?))
        } else {
            let v8: u64 = {
                let a2978 = {
                    let a2976 = v1;
                    let a2977 = v6;
                    nat_sub(a2976, a2977)
                };
                let a2979 = v7;
                nat_sub(a2978, a2979)
            };
            let v9: u64 = {
                let a2981 = 1u64;
                let a2982 = f192(v0, match v2.clone() {
                    Adt3::C0(h2980, _, _, _, _, _, _, _) => {
                        h2980
                    }
                })?;
                nat_add(a2981, a2982)?
            };
            let v10: u64 = {
                let a2983 = v8;
                let a2984 = v9;
                let a2985 = 0u64;
                nat_quot(a2983, a2984, a2985)
            };
            let v11: u64 = {
                let a2986 = v8;
                let a2987 = v9;
                let a2988 = 0u64;
                nat_rem(a2986, a2987, a2988)
            };
            let a2989 = v11;
            let a2990 = 0u64;
            if nat_eq(a2989, a2990) {
                Ok(Adt9::C4(19u64, 4u64, f103(v10, v4)?, 2u64, 0u64, 1u64, 2u64))
            } else {
                if f193(v0) {
                    let a2994 = {
                        let a2991 = v11;
                        let a2992 = 2u64;
                        let a2993 = 0u64;
                        nat_rem(a2991, a2992, a2993)
                    };
                    let a2995 = 1u64;
                    if nat_eq(a2994, a2995) {
                        Ok(Adt9::C4(6u64, 0u64, f103({
                            let a3004 = {
                                let a2997 = v10;
                                let a2998 = match v2.clone() {
                                    Adt3::C0(h2996, _, _, _, _, _, _, _) => {
                                        h2996
                                    }
                                };
                                nat_mul(a2997, a2998)?
                            };
                            let a3005 = {
                                let a3001 = {
                                    let a2999 = v11;
                                    let a3000 = 1u64;
                                    nat_sub(a2999, a3000)
                                };
                                let a3002 = 2u64;
                                let a3003 = 0u64;
                                nat_quot(a3001, a3002, a3003)
                            };
                            nat_add(a3004, a3005)?
                        }, v2)?, 2u64, 0u64, 1u64, 0u64))
                    } else {
                        Ok(Adt9::C4(4u64, 3u64, f103({
                            let a3014 = {
                                let a3007 = v5;
                                let a3008 = match v2.clone() {
                                    Adt3::C0(h3006, _, _, _, _, _, _, _) => {
                                        h3006
                                    }
                                };
                                nat_mul(a3007, a3008)?
                            };
                            let a3015 = {
                                let a3011 = {
                                    let a3009 = v11;
                                    let a3010 = 1u64;
                                    nat_sub(a3009, a3010)
                                };
                                let a3012 = 2u64;
                                let a3013 = 0u64;
                                nat_quot(a3011, a3012, a3013)
                            };
                            nat_add(a3014, a3015)?
                        }, v3.clone())?, 1u64, 0u64, 3u64, f103({
                            let a3024 = {
                                let a3017 = v5;
                                let a3018 = match v2 {
                                    Adt3::C0(h3016, _, _, _, _, _, _, _) => {
                                        h3016
                                    }
                                };
                                nat_mul(a3017, a3018)?
                            };
                            let a3025 = {
                                let a3021 = {
                                    let a3019 = v11;
                                    let a3020 = 1u64;
                                    nat_sub(a3019, a3020)
                                };
                                let a3022 = 2u64;
                                let a3023 = 0u64;
                                nat_quot(a3021, a3022, a3023)
                            };
                            nat_add(a3024, a3025)?
                        }, v3)?))
                    }
                } else {
                    f194(v0, v10, {
                        let a3026 = v11;
                        let a3027 = 1u64;
                        nat_sub(a3026, a3027)
                    }, v5, v2, v3)
                }
            }
        }
    }
}

pub fn f88(v0: u64, v1: u64, v2: Adt3, v3: Adt3, v4: Adt3, v5: u64) -> R<Adt9> {
    let a3028 = v1;
    let a3029 = f195(v2.clone(), v4.clone())?;
    if nat_lt(a3028, a3029) {
        let a3032 = v1;
        let a3033 = {
            let a3030 = 2u64;
            let a3031 = f196(v4.clone())?;
            nat_mul(a3030, a3031)?
        };
        if nat_lt(a3032, a3033) {
            let a3037 = {
                let a3034 = v1;
                let a3035 = 2u64;
                let a3036 = 0u64;
                nat_rem(a3034, a3035, a3036)
            };
            let a3038 = 0u64;
            if nat_eq(a3037, a3038) {
                f197({
                    let a3039 = v1;
                    let a3040 = 2u64;
                    let a3041 = 0u64;
                    nat_quot(a3039, a3040, a3041)
                }, v4)
            } else {
                if f198(match v2 {
                    Adt3::C0(_, h3042, _, _, _, _, _, _) => {
                        h3042
                    }
                }, v5) {
                    Ok(Adt9::C4(19u64, 1u64, 2u64, 2u64, 0u64, 1u64, 2u64))
                } else {
                    Ok(Adt9::C0(Adt10::C1))
                }
            }
        } else {
            let v6: u64 = {
                let a3045 = v1;
                let a3046 = {
                    let a3043 = 2u64;
                    let a3044 = f196(v4.clone())?;
                    nat_mul(a3043, a3044)?
                };
                nat_sub(a3045, a3046)
            };
            let v7: u64 = {
                let a3050 = v6;
                let a3051 = {
                    let a3048 = 1u64;
                    let a3049 = match v2.clone() {
                        Adt3::C0(h3047, _, _, _, _, _, _, _) => {
                            h3047
                        }
                    };
                    nat_add(a3048, a3049)?
                };
                let a3052 = 0u64;
                nat_quot(a3050, a3051, a3052)
            };
            let v8: u64 = {
                let a3056 = v6;
                let a3057 = {
                    let a3054 = 1u64;
                    let a3055 = match v2.clone() {
                        Adt3::C0(h3053, _, _, _, _, _, _, _) => {
                            h3053
                        }
                    };
                    nat_add(a3054, a3055)?
                };
                let a3058 = 0u64;
                nat_rem(a3056, a3057, a3058)
            };
            let a3059 = v8;
            let a3060 = 0u64;
            if nat_eq(a3059, a3060) {
                f197(v7, v4)
            } else {
                f199(v0, v7, {
                    let a3061 = v8;
                    let a3062 = 1u64;
                    nat_sub(a3061, a3062)
                }, v2, v3, v4, v5)
            }
        }
    } else {
        Ok(Adt9::C0(Adt10::C1))
    }
}

pub fn f89(v0: u64, v1: Adt3, v2: Adt3, v3: Adt3, v4: Adt3, v5: u64) -> R<Adt9> {
    let v6: u64 = {
        let a3065 = match v4.clone() {
            Adt3::C0(h3063, _, _, _, _, _, _, _) => {
                h3063
            }
        };
        let a3066 = match v4.clone() {
            Adt3::C0(_, h3064, _, _, _, _, _, _) => {
                h3064
            }
        };
        nat_mul(a3065, a3066)?
    };
    let v7: u64 = {
        let a3067 = 2u64;
        let a3068 = v6;
        nat_mul(a3067, a3068)?
    };
    let v8: u64 = f200(match v3.clone() {
        Adt3::C0(_, h3069, _, _, _, _, _, _) => {
            h3069
        }
    }, match v3.clone() {
        Adt3::C0(_, _, h3070, _, _, _, _, _) => {
            h3070
        }
    }, match v3.clone() {
        Adt3::C0(_, _, _, h3071, _, _, _, _) => {
            h3071
        }
    });
    let a3072 = v0;
    let a3073 = v7;
    if nat_lt(a3072, a3073) {
        let a3077 = {
            let a3074 = v0;
            let a3075 = 2u64;
            let a3076 = 0u64;
            nat_rem(a3074, a3075, a3076)
        };
        let a3078 = 0u64;
        if nat_eq(a3077, a3078) {
            Ok(Adt9::C4(19u64, 5u64, f103({
                let a3079 = v0;
                let a3080 = 2u64;
                let a3081 = 0u64;
                nat_quot(a3079, a3080, a3081)
            }, v4)?, 2u64, 0u64, 1u64, 2u64))
        } else {
            if f201(v5, v8) {
                Ok(Adt9::C4(19u64, 1u64, 2u64, 2u64, 0u64, 1u64, 2u64))
            } else {
                Ok(Adt9::C0(Adt10::C1))
            }
        }
    } else {
        let v9: u64 = {
            let a3082 = v0;
            let a3083 = v7;
            nat_sub(a3082, a3083)
        };
        let v10: u64 = {
            let a3087 = v9;
            let a3088 = {
                let a3085 = 1u64;
                let a3086 = match v1.clone() {
                    Adt3::C0(h3084, _, _, _, _, _, _, _) => {
                        h3084
                    }
                };
                nat_add(a3085, a3086)?
            };
            let a3089 = 0u64;
            nat_quot(a3087, a3088, a3089)
        };
        let v11: u64 = {
            let a3093 = v9;
            let a3094 = {
                let a3091 = 1u64;
                let a3092 = match v1.clone() {
                    Adt3::C0(h3090, _, _, _, _, _, _, _) => {
                        h3090
                    }
                };
                nat_add(a3091, a3092)?
            };
            let a3095 = 0u64;
            nat_rem(a3093, a3094, a3095)
        };
        let a3096 = v11;
        let a3097 = 0u64;
        if nat_eq(a3096, a3097) {
            Ok(Adt9::C4(19u64, 5u64, f103({
                let a3098 = v10;
                let a3099 = v6;
                let a3100 = 0u64;
                nat_rem(a3098, a3099, a3100)
            }, v4)?, 2u64, 0u64, 1u64, 2u64))
        } else {
            if f201(v5, v8) {
                let v12: u64 = {
                    let a3106 = {
                        let a3102 = v10;
                        let a3103 = match v1.clone() {
                            Adt3::C0(h3101, _, _, _, _, _, _, _) => {
                                h3101
                            }
                        };
                        nat_mul(a3102, a3103)?
                    };
                    let a3107 = {
                        let a3104 = v11;
                        let a3105 = 1u64;
                        nat_sub(a3104, a3105)
                    };
                    nat_add(a3106, a3107)?
                };
                let a3108 = v5;
                let a3109 = 4294967295u64;
                if nat_eq(a3108, a3109) {
                    Ok(Adt9::C1(false, f103(v12, v1)?, f103(v12, v2)?))
                } else {
                    Ok(Adt9::C4(8u64, 0u64, f103(v12, v1)?, 4u64, f103({
                        let a3115 = {
                            let a3111 = v5;
                            let a3112 = match v3.clone() {
                                Adt3::C0(h3110, _, _, _, _, _, _, _) => {
                                    h3110
                                }
                            };
                            nat_mul(a3111, a3112)?
                        };
                        let a3116 = {
                            let a3113 = v11;
                            let a3114 = 1u64;
                            nat_sub(a3113, a3114)
                        };
                        nat_add(a3115, a3116)?
                    }, v3)?, 3u64, f103(v12, v2)?))
                }
            } else {
                Ok(Adt9::C0(Adt10::C1))
            }
        }
    }
}

pub fn f90(v0: u64, v1: Adt3, v2: Adt3, v3: Adt7, v4: u64) -> R<Adt9> {
    let v5: u64 = f202(v3.clone())?;
    let v6: u64 = {
        let a3117 = v0;
        let a3118 = v5;
        let a3119 = 0u64;
        nat_quot(a3117, a3118, a3119)
    };
    let v7: u64 = {
        let a3120 = v0;
        let a3121 = v5;
        let a3122 = 0u64;
        nat_rem(a3120, a3121, a3122)
    };
    let v8: u64 = f203(v3.clone())?;
    let a3123 = v7;
    let a3124 = 0u64;
    if nat_eq(a3123, a3124) {
        Ok(Adt9::C4(19u64, 2u64, if match v3 {
            Adt7::C0(_, _, _, h3125, _) => {
                h3125
            }
        } {
            4286578687u64
        } else {
            0u64
        }, 2u64, 0u64, 1u64, 0u64))
    } else {
        let a3129 = v5;
        let a3130 = {
            let a3127 = v7;
            let a3128 = if match v3.clone() {
                Adt7::C0(_, _, _, h3126, _) => {
                    h3126
                }
            } {
                1u64
            } else {
                2u64
            };
            nat_add(a3127, a3128)?
        };
        if nat_le(a3129, a3130) {
            if match v3.clone() {
                Adt7::C0(_, _, _, h3131, _) => {
                    h3131
                }
            } {
                Ok(Adt9::C4(19u64, 1u64, 0u64, 2u64, 0u64, 3u64, f103(v6, v2)?))
            } else {
                let a3134 = {
                    let a3132 = v7;
                    let a3133 = 2u64;
                    nat_add(a3132, a3133)?
                };
                let a3135 = v5;
                if nat_eq(a3134, a3135) {
                    let v9: u64 = if match v3.clone() {
                        Adt7::C0(_, _, h3136, _, _) => {
                            h3136
                        }
                    } {
                        let a3143 = {
                            let m3139 = match v3.clone() {
                                Adt7::C0(h3138, _, _, _, _) => {
                                    h3138
                                }
                            };
                            match m3139 {
                                Adt6::C0(h3137, _, _, _) => {
                                    h3137
                                }
                            }
                        };
                        let a3144 = {
                            let m3142 = match v3 {
                                Adt7::C0(_, h3141, _, _, _) => {
                                    h3141
                                }
                            };
                            match m3142 {
                                Adt6::C0(h3140, _, _, _) => {
                                    h3140
                                }
                            }
                        };
                        nat_mul(a3143, a3144)?
                    } else {
                        f204(match v3 {
                            Adt7::C0(h3145, _, _, _, _) => {
                                h3145
                            }
                        }, match v1 {
                            Adt3::C0(h3146, _, _, _, _, _, _, _) => {
                                h3146
                            }
                        }, f63(v6, v2, 0u64))?
                    };
                    Ok(Adt9::C4(3u64, 2u64, {
                        let a3147 = v9;
                        let a3148 = 0u64;
                        if nat_eq(a3147, a3148) {
                            1u64
                        } else {
                            v9
                        }
                    }, 2u64, 0u64, 1u64, 1u64))
                } else {
                    Ok(Adt9::C4(5u64, 1u64, 0u64, 1u64, 1u64, 3u64, f103(v6, v2)?))
                }
            }
        } else {
            let v10: u64 = {
                let a3151 = {
                    let a3149 = v7;
                    let a3150 = 1u64;
                    nat_sub(a3149, a3150)
                };
                let a3152 = v8;
                let a3153 = 0u64;
                nat_rem(a3151, a3152, a3153)
            };
            let m3159 = f185(v3.clone(), v1, v2, v6, {
                let a3156 = {
                    let a3154 = v7;
                    let a3155 = 1u64;
                    nat_sub(a3154, a3155)
                };
                let a3157 = v8;
                let a3158 = 0u64;
                nat_quot(a3156, a3157, a3158)
            })?;
            match m3159 {
                None => {
                    Ok(Adt9::C4(19u64, 1u64, 0u64, 2u64, 0u64, 1u64, 0u64))
                }
                Some(v11) => {
                    let a3161 = match v3.clone() {
                        Adt7::C0(_, _, _, _, h3160) => {
                            h3160
                        }
                    };
                    let a3162 = 1u64;
                    if if nat_eq(a3161, a3162) {
                        let a3163 = v10;
                        let a3164 = 0u64;
                        nat_eq(a3163, a3164)
                    } else {
                        false
                    } {
                        Ok(Adt9::C4(6u64, 0u64, v11, 2u64, 0u64, 1u64, 1u64))
                    } else {
                        if match v3.clone() {
                            Adt7::C0(_, _, _, h3165, _) => {
                                h3165
                            }
                        } {
                            let a3168 = {
                                let a3166 = v10;
                                let a3167 = 1u64;
                                nat_add(a3166, a3167)?
                            };
                            let a3169 = v8;
                            if nat_eq(a3168, a3169) {
                                let a3170 = v4;
                                let a3171 = 2u64;
                                if nat_eq(a3170, a3171) {
                                    Ok(Adt9::C4(19u64, 1u64, 0u64, 2u64, 0u64, 1u64, 0u64))
                                } else {
                                    Ok(Adt9::C4(19u64, {
                                        let a3173 = match v3.clone() {
                                            Adt7::C0(_, _, _, _, h3172) => {
                                                h3172
                                            }
                                        };
                                        let a3174 = 1u64;
                                        if nat_eq(a3173, a3174) {
                                            1u64
                                        } else {
                                            0u64
                                        }
                                    }, {
                                        let a3176 = match v3 {
                                            Adt7::C0(_, _, _, _, h3175) => {
                                                h3175
                                            }
                                        };
                                        let a3177 = 1u64;
                                        if nat_eq(a3176, a3177) {
                                            1u64
                                        } else {
                                            v11
                                        }
                                    }, 2u64, 0u64, 1u64, 0u64))
                                }
                            } else {
                                Ok(Adt9::C4(16u64, 1u64, 0u64, {
                                    let a3179 = match v3.clone() {
                                        Adt7::C0(_, _, _, _, h3178) => {
                                            h3178
                                        }
                                    };
                                    let a3180 = 1u64;
                                    if nat_eq(a3179, a3180) {
                                        1u64
                                    } else {
                                        0u64
                                    }
                                }, {
                                    let a3182 = match v3 {
                                        Adt7::C0(_, _, _, _, h3181) => {
                                            h3181
                                        }
                                    };
                                    let a3183 = 1u64;
                                    if nat_eq(a3182, a3183) {
                                        1u64
                                    } else {
                                        v11
                                    }
                                }, 1u64, 2u64))
                            }
                        } else {
                            Ok(Adt9::C4(4u64, 1u64, 0u64, {
                                let a3185 = match v3.clone() {
                                    Adt7::C0(_, _, _, _, h3184) => {
                                        h3184
                                    }
                                };
                                let a3186 = 1u64;
                                if nat_eq(a3185, a3186) {
                                    1u64
                                } else {
                                    0u64
                                }
                            }, {
                                let a3188 = match v3 {
                                    Adt7::C0(_, _, _, _, h3187) => {
                                        h3187
                                    }
                                };
                                let a3189 = 1u64;
                                if nat_eq(a3188, a3189) {
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

pub fn f91(v0: u64, v1: Adt3, v2: Adt3, v3: Adt3) -> R<Adt9> {
    let v4: u64 = f205(match v3.clone() {
        Adt3::C0(h3190, _, _, _, _, _, _, _) => {
            h3190
        }
    })?;
    let v5: u64 = {
        let a3191 = v0;
        let a3192 = v4;
        let a3193 = 0u64;
        nat_rem(a3191, a3192, a3193)
    };
    let v6: u64 = {
        let a3194 = v0;
        let a3195 = v4;
        let a3196 = 0u64;
        nat_quot(a3194, a3195, a3196)
    };
    let a3197 = v5;
    let a3198 = 0u64;
    if nat_eq(a3197, a3198) {
        Ok(Adt9::C4(19u64, 2u64, 0u64, 2u64, 0u64, 1u64, 0u64))
    } else {
        let a3201 = {
            let a3199 = v5;
            let a3200 = 1u64;
            nat_add(a3199, a3200)?
        };
        let a3202 = v4;
        if nat_eq(a3201, a3202) {
            Ok(Adt9::C4(19u64, 1u64, 0u64, 2u64, 0u64, 3u64, f103(v6, v2)?))
        } else {
            let a3206 = {
                let a3203 = v5;
                let a3204 = 2u64;
                let a3205 = 0u64;
                nat_rem(a3203, a3204, a3205)
            };
            let a3207 = 0u64;
            if nat_eq(a3206, a3207) {
                Ok(Adt9::C4(4u64, 1u64, 0u64, 1u64, 1u64, 1u64, 0u64))
            } else {
                let v7: u64 = {
                    let a3210 = {
                        let a3208 = v5;
                        let a3209 = 1u64;
                        nat_sub(a3208, a3209)
                    };
                    let a3211 = 2u64;
                    let a3212 = 0u64;
                    nat_quot(a3210, a3211, a3212)
                };
                let v8: u64 = f63(v6, v2.clone(), 0u64);
                let v9: u64 = f63(v6, v2.clone(), 1u64);
                let v10: u64 = f63(v6, v2, 2u64);
                Ok(Adt9::C4(10u64, 0u64, f62(v1, {
                    let a3213 = v9;
                    let a3214 = v7;
                    nat_add(a3213, a3214)?
                }, v8, v10, 0u64)?, 4u64, f62(v3, v7, v8, 0u64, 0u64)?, 1u64, 1u64))
            }
        }
    }
}

pub fn f92(v0: u64, v1: Adt3, v2: Adt3, v3: Adt6) -> R<Adt9> {
    let v4: u64 = f206(v3.clone())?;
    let v5: u64 = {
        let a3215 = v0;
        let a3216 = v4;
        let a3217 = 0u64;
        nat_rem(a3215, a3216, a3217)
    };
    let v6: u64 = {
        let a3218 = v0;
        let a3219 = v4;
        let a3220 = 0u64;
        nat_quot(a3218, a3219, a3220)
    };
    let a3221 = v5;
    let a3222 = 0u64;
    if nat_eq(a3221, a3222) {
        Ok(Adt9::C4(19u64, 2u64, 0u64, 2u64, 0u64, 1u64, 0u64))
    } else {
        let a3225 = {
            let a3223 = v5;
            let a3224 = 1u64;
            nat_add(a3223, a3224)?
        };
        let a3226 = v4;
        if nat_eq(a3225, a3226) {
            Ok(Adt9::C4(7u64, 1u64, 0u64, 2u64, 0u64, 3u64, f103(v6, v2)?))
        } else {
            let a3230 = {
                let a3227 = v5;
                let a3228 = 2u64;
                let a3229 = 0u64;
                nat_rem(a3227, a3228, a3229)
            };
            let a3231 = 0u64;
            if nat_eq(a3230, a3231) {
                Ok(Adt9::C4(4u64, 1u64, 0u64, 1u64, 1u64, 1u64, 0u64))
            } else {
                let v7: u64 = {
                    let a3233 = f63(v6, v2.clone(), 0u64);
                    let a3234 = match v3.clone() {
                        Adt6::C0(_, _, h3232, _) => {
                            h3232
                        }
                    };
                    nat_add(a3233, a3234)?
                };
                let v8: u64 = {
                    let a3242 = {
                        let a3239 = {
                            let a3236 = v7;
                            let a3237 = match v3.clone() {
                                Adt6::C0(_, h3235, _, _) => {
                                    h3235
                                }
                            };
                            nat_add(a3236, a3237)?
                        };
                        let a3240 = match v3.clone() {
                            Adt6::C0(h3238, _, _, _) => {
                                h3238
                            }
                        };
                        nat_sub(a3239, a3240)
                    };
                    let a3243 = match v3.clone() {
                        Adt6::C0(_, h3241, _, _) => {
                            h3241
                        }
                    };
                    let a3244 = 0u64;
                    nat_quot(a3242, a3243, a3244)
                };
                let v9: u64 = {
                    let a3250 = v8;
                    let a3251 = {
                        let a3247 = {
                            let a3245 = v5;
                            let a3246 = 1u64;
                            nat_sub(a3245, a3246)
                        };
                        let a3248 = 2u64;
                        let a3249 = 0u64;
                        nat_quot(a3247, a3248, a3249)
                    };
                    nat_add(a3250, a3251)?
                };
                let v10: u64 = {
                    let a3253 = v9;
                    let a3254 = match v3.clone() {
                        Adt6::C0(_, h3252, _, _) => {
                            h3252
                        }
                    };
                    nat_mul(a3253, a3254)?
                };
                let v11: u64 = {
                    let a3255 = v7;
                    let a3256 = v10;
                    nat_sub(a3255, a3256)
                };
                let a3258 = v9;
                let a3259 = match v1.clone() {
                    Adt3::C0(_, h3257, _, _, _, _, _, _) => {
                        h3257
                    }
                };
                if if if nat_lt(a3258, a3259) {
                    let a3260 = v10;
                    let a3261 = v7;
                    nat_le(a3260, a3261)
                } else {
                    false
                } {
                    let a3263 = v11;
                    let a3264 = match v3.clone() {
                        Adt6::C0(h3262, _, _, _) => {
                            h3262
                        }
                    };
                    nat_lt(a3263, a3264)
                } else {
                    false
                } {
                    Ok(Adt9::C4(6u64, 0u64, f62(v1, {
                        let a3268 = {
                            let a3266 = f63(v6, v2, 1u64);
                            let a3267 = match v3 {
                                Adt6::C0(h3265, _, _, _) => {
                                    h3265
                                }
                            };
                            nat_mul(a3266, a3267)?
                        };
                        let a3269 = v11;
                        nat_add(a3268, a3269)?
                    }, v9, 0u64, 0u64)?, 2u64, 0u64, 1u64, 1u64))
                } else {
                    Ok(Adt9::C4(19u64, 2u64, 0u64, 2u64, 0u64, 1u64, 1u64))
                }
            }
        }
    }
}

pub fn f93(v0: u64, v1: Adt3, v2: Adt3, v3: Adt5) -> R<Adt9> {
    let v4: u64 = f63(v0, v2.clone(), 0u64);
    let v5: u64 = f63(v0, v2.clone(), 3u64);
    let v6: u64 = {
        let a3271 = v5;
        let a3272 = match v3.clone() {
            Adt5::C0(_, _, _, _, _, _, h3270, _) => {
                h3270
            }
        };
        let a3273 = 0u64;
        nat_rem(a3271, a3272, a3273)
    };
    let v7: u64 = {
        let a3275 = v5;
        let a3276 = match v3.clone() {
            Adt5::C0(_, _, _, _, _, _, h3274, _) => {
                h3274
            }
        };
        let a3277 = 0u64;
        nat_quot(a3275, a3276, a3277)
    };
    let v8: Option<u64> = f65(match v3.clone() {
        Adt5::C0(h3278, _, _, _, _, _, _, _) => {
            h3278
        }
    }, match v1.clone() {
        Adt3::C0(h3279, _, _, _, _, _, _, _) => {
            h3279
        }
    }, f63(v0, v2.clone(), 1u64), {
        let a3283 = v4;
        let a3284 = {
            let m3282 = match v3.clone() {
                Adt5::C0(h3281, _, _, _, _, _, _, _) => {
                    h3281
                }
            };
            match m3282 {
                Adt6::C0(h3280, _, _, _) => {
                    h3280
                }
            }
        };
        let a3285 = 0u64;
        nat_rem(a3283, a3284, a3285)
    })?;
    let v9: u64 = {
        let a3289 = v4;
        let a3290 = {
            let m3288 = match v3.clone() {
                Adt5::C0(h3287, _, _, _, _, _, _, _) => {
                    h3287
                }
            };
            match m3288 {
                Adt6::C0(h3286, _, _, _) => {
                    h3286
                }
            }
        };
        let a3291 = 0u64;
        nat_quot(a3289, a3290, a3291)
    };
    let v10: Option<u64> = f65(match v3.clone() {
        Adt5::C0(_, h3292, _, _, _, _, _, _) => {
            h3292
        }
    }, match v1.clone() {
        Adt3::C0(_, h3293, _, _, _, _, _, _) => {
            h3293
        }
    }, f63(v0, v2.clone(), 2u64), {
        let a3297 = v9;
        let a3298 = {
            let m3296 = match v3.clone() {
                Adt5::C0(_, h3295, _, _, _, _, _, _) => {
                    h3295
                }
            };
            match m3296 {
                Adt6::C0(h3294, _, _, _) => {
                    h3294
                }
            }
        };
        let a3299 = 0u64;
        nat_rem(a3297, a3298, a3299)
    })?;
    let v11: u64 = {
        let a3303 = v9;
        let a3304 = {
            let m3302 = match v3.clone() {
                Adt5::C0(_, h3301, _, _, _, _, _, _) => {
                    h3301
                }
            };
            match m3302 {
                Adt6::C0(h3300, _, _, _) => {
                    h3300
                }
            }
        };
        let a3305 = 0u64;
        nat_quot(a3303, a3304, a3305)
    };
    let v12: Option<u64> = f65(match v3.clone() {
        Adt5::C0(_, _, h3306, _, _, _, _, _) => {
            h3306
        }
    }, match v1.clone() {
        Adt3::C0(_, _, h3307, _, _, _, _, _) => {
            h3307
        }
    }, v6, {
        let a3311 = v11;
        let a3312 = {
            let m3310 = match v3.clone() {
                Adt5::C0(_, _, h3309, _, _, _, _, _) => {
                    h3309
                }
            };
            match m3310 {
                Adt6::C0(h3308, _, _, _) => {
                    h3308
                }
            }
        };
        let a3313 = 0u64;
        nat_rem(a3311, a3312, a3313)
    })?;
    let v13: u64 = {
        let a3317 = v11;
        let a3318 = {
            let m3316 = match v3.clone() {
                Adt5::C0(_, _, h3315, _, _, _, _, _) => {
                    h3315
                }
            };
            match m3316 {
                Adt6::C0(h3314, _, _, _) => {
                    h3314
                }
            }
        };
        let a3319 = 0u64;
        nat_quot(a3317, a3318, a3319)
    };
    let v17: Adt12 = {
        let m3320 = v8;
        match m3320 {
            None => {
                Adt12::C2(0u64)
            }
            Some(v14) => {
                let m3321 = v10;
                match m3321 {
                    None => {
                        Adt12::C2(0u64)
                    }
                    Some(v15) => {
                        let m3322 = v12;
                        match m3322 {
                            None => {
                                Adt12::C2(0u64)
                            }
                            Some(v16) => {
                                Adt12::C0(1u64, {
                                    let a3331 = {
                                        let a3326 = f62(v1, v14, v15, v16, 0u64)?;
                                        let a3327 = {
                                            let a3324 = v7;
                                            let a3325 = match v3.clone() {
                                                Adt5::C0(_, _, _, _, h3323, _, _, _) => {
                                                    h3323
                                                }
                                            };
                                            nat_mul(a3324, a3325)?
                                        };
                                        nat_add(a3326, a3327)?
                                    };
                                    let a3332 = {
                                        let a3329 = v13;
                                        let a3330 = match v3.clone() {
                                            Adt5::C0(_, _, _, h3328, _, _, _, _) => {
                                                h3328
                                            }
                                        };
                                        nat_mul(a3329, a3330)?
                                    };
                                    nat_add(a3331, a3332)?
                                })
                            }
                        }
                    }
                }
            }
        }
    };
    Ok(Adt9::C7(Adt11::C2(match v3.clone() {
        Adt5::C0(_, _, _, _, _, h3333, _, _) => {
            h3333
        }
    }, match v3.clone() {
        Adt5::C0(_, _, _, _, _, _, _, h3334) => {
            h3334
        }
    }, true), v17, Adt12::C2(0u64), Adt13::C1(f103(v0, v2)?), {
        let a3336 = match v3.clone() {
            Adt5::C0(_, _, _, _, _, h3335, _, _) => {
                h3335
            }
        };
        let a3337 = 0u64;
        if nat_eq(a3336, a3337) {
            4u64
        } else {
            2u64
        }
    }, {
        let a3339 = match v3 {
            Adt5::C0(_, _, _, _, _, _, _, h3338) => {
                h3338
            }
        };
        let a3340 = 0u64;
        if nat_eq(a3339, a3340) {
            4u64
        } else {
            2u64
        }
    }))
}

pub fn f94(v0: Adt16, v1: u64, v2: Adt3, v3: Adt3, v4: u64, v5: u64, v6: u64, v7: u64, v8: u64, v9: u64) -> R<Adt9> {
    let m3353 = f207(v0, v1, match v2.clone() {
        Adt3::C0(h3341, _, _, _, _, _, _, _) => {
            h3341
        }
    }, match v2.clone() {
        Adt3::C0(_, h3342, _, _, _, _, _, _) => {
            h3342
        }
    }, match v2.clone() {
        Adt3::C0(_, _, h3343, _, _, _, _, _) => {
            h3343
        }
    }, match v2.clone() {
        Adt3::C0(_, _, _, h3344, _, _, _, _) => {
            h3344
        }
    }, match v2.clone() {
        Adt3::C0(_, _, _, _, h3345, _, _, _) => {
            h3345
        }
    }, match v2.clone() {
        Adt3::C0(_, _, _, _, _, h3346, _, _) => {
            h3346
        }
    }, match v2.clone() {
        Adt3::C0(_, _, _, _, _, _, h3347, _) => {
            h3347
        }
    }, match v2 {
        Adt3::C0(_, _, _, _, _, _, _, h3348) => {
            h3348
        }
    }, v6, v7, v8, v9, match v3.clone() {
        Adt3::C0(_, _, _, _, h3349, _, _, _) => {
            h3349
        }
    }, match v3.clone() {
        Adt3::C0(_, _, _, _, _, h3350, _, _) => {
            h3350
        }
    }, match v3.clone() {
        Adt3::C0(_, _, _, _, _, _, h3351, _) => {
            h3351
        }
    }, match v3 {
        Adt3::C0(_, _, _, _, _, _, _, h3352) => {
            h3352
        }
    }, v4)?;
    match m3353 {
        Adt17::C0 => {
            Ok(Adt9::C0(Adt10::C1))
        }
        Adt17::C1(v10, v11, v12, v13, v14, v15, v16, v17) => {
            Ok(f22(false, v10, v11, 0u64, 0u64, v12, v13, v14, v15, v16, v17, v5, v5))
        }
    }
}

pub fn f95(v0: u64, v1: Adt3, v2: Adt3, v3: u64, v4: u64, v5: u64, v6: u64, v7: u64) -> R<Adt9> {
    let v8: u64 = {
        let a3356 = v6;
        let a3357 = {
            let a3354 = 3u64;
            let a3355 = v5;
            nat_add(a3354, a3355)?
        };
        nat_sub(a3356, a3357)
    };
    let a3359 = v0;
    let a3360 = f55(v3, match v1.clone() {
        Adt3::C0(h3358, _, _, _, _, _, _, _) => {
            h3358
        }
    }, v5, v8)?;
    if nat_lt(a3359, a3360) {
        let v9: u64 = f112(match v1.clone() {
            Adt3::C0(h3361, _, _, _, _, _, _, _) => {
                h3361
            }
        }, v5, v8)?;
        let v10: u64 = f113(v3, match v1.clone() {
            Adt3::C0(h3362, _, _, _, _, _, _, _) => {
                h3362
            }
        }, v5)?;
        let a3363 = v0;
        let a3364 = v9;
        if nat_lt(a3363, a3364) {
            f208(v0, v2, f49(v3), {
                let a3365 = 3u64;
                let a3366 = v5;
                nat_add(a3365, a3366)?
            }, v7)
        } else {
            let a3369 = v0;
            let a3370 = {
                let a3367 = v9;
                let a3368 = v10;
                nat_add(a3367, a3368)?
            };
            if nat_lt(a3369, a3370) {
                let v11: u64 = {
                    let a3371 = v0;
                    let a3372 = v9;
                    nat_sub(a3371, a3372)
                };
                let v12: u64 = f209(v3, match v1.clone() {
                    Adt3::C0(h3373, _, _, _, _, _, _, _) => {
                        h3373
                    }
                })?;
                f210({
                    let a3374 = v11;
                    let a3375 = v12;
                    let a3376 = 0u64;
                    nat_rem(a3374, a3375, a3376)
                }, {
                    let a3377 = v11;
                    let a3378 = v12;
                    let a3379 = 0u64;
                    nat_quot(a3377, a3378, a3379)
                }, v1, v3, v4, v7)
            } else {
                f211({
                    let a3382 = v0;
                    let a3383 = {
                        let a3380 = v9;
                        let a3381 = v10;
                        nat_add(a3380, a3381)?
                    };
                    nat_sub(a3382, a3383)
                }, v1, v2, v3)
            }
        }
    } else {
        Ok(Adt9::C0(Adt10::C1))
    }
}

pub fn f96(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64) -> R<Adt9> {
    let a3384 = v0;
    let a3385 = f41(v1)?;
    if nat_lt(a3384, a3385) {
        let a3386 = v0;
        let a3387 = 4u64;
        if nat_lt(a3386, a3387) {
            let a3388 = v0;
            let a3389 = 0u64;
            if nat_eq(a3388, a3389) {
                Ok(Adt9::C4(16u64, 2u64, v2, 2u64, v3, 1u64, 2u64))
            } else {
                let a3390 = v0;
                let a3391 = 1u64;
                if nat_eq(a3390, a3391) {
                    let a3392 = v5;
                    let a3393 = 0u64;
                    if nat_eq(a3392, a3393) {
                        Ok(Adt9::C4(9u64, 2u64, v3, 2u64, v2, 1u64, 2u64))
                    } else {
                        Ok(Adt9::C0(Adt10::C1))
                    }
                } else {
                    let a3394 = v0;
                    let a3395 = 2u64;
                    if nat_eq(a3394, a3395) {
                        Ok(Adt9::C4(11u64, 1u64, 2u64, 2u64, v4, 1u64, 2u64))
                    } else {
                        Ok(Adt9::C4(20u64, 1u64, 2u64, 2u64, 10u64, 1u64, 2u64))
                    }
                }
            }
        } else {
            let a3396 = v0;
            let a3397 = 4u64;
            if if nat_eq(a3396, a3397) {
                let a3398 = f212(v5)?;
                let a3399 = v1;
                !nat_eq(a3398, a3399)
            } else {
                false
            } {
                Ok(Adt9::C0(Adt10::C1))
            } else {
                let v6: u64 = {
                    let a3402 = {
                        let a3400 = v0;
                        let a3401 = 4u64;
                        nat_sub(a3400, a3401)
                    };
                    let a3403 = 3u64;
                    let a3404 = 0u64;
                    nat_quot(a3402, a3403, a3404)
                };
                let v7: u64 = {
                    let a3407 = {
                        let a3405 = v0;
                        let a3406 = 4u64;
                        nat_sub(a3405, a3406)
                    };
                    let a3408 = 3u64;
                    let a3409 = 0u64;
                    nat_rem(a3407, a3408, a3409)
                };
                let a3410 = v7;
                let a3411 = 0u64;
                if nat_eq(a3410, a3411) {
                    Ok(Adt9::C4(3u64, 2u64, v6, 2u64, 0u64, 1u64, 0u64))
                } else {
                    let a3412 = v7;
                    let a3413 = 1u64;
                    if nat_eq(a3412, a3413) {
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

pub fn f97(v0: bool, v1: u64, v2: u64, v3: u64, v4: u64) -> bool {
    if if if if v0 {
        let a3414 = v1;
        let a3415 = 5u64;
        nat_eq(a3414, a3415)
    } else {
        false
    } {
        let a3416 = v2;
        let a3417 = 16u64;
        nat_eq(a3416, a3417)
    } else {
        false
    } {
        let a3418 = v3;
        let a3419 = 0u64;
        nat_eq(a3418, a3419)
    } else {
        false
    } {
        let a3420 = v4;
        let a3421 = 0u64;
        nat_eq(a3420, a3421)
    } else {
        false
    }
}

pub fn f98(v0: bool, v1: u64, v2: u64, v3: u64, v4: u64) -> Adt11 {
    let a3422 = v1;
    let a3423 = 0u64;
    if nat_eq(a3422, a3423) {
        if v0 {
            let a3424 = v3;
            let a3425 = 0u64;
            if if if nat_eq(a3424, a3425) {
                let a3426 = v4;
                let a3427 = 43u64;
                nat_eq(a3426, a3427)
            } else {
                false
            } {
                true
            } else {
                let a3428 = v3;
                let a3429 = 43u64;
                if nat_eq(a3428, a3429) {
                    let a3430 = v4;
                    let a3431 = 0u64;
                    nat_eq(a3430, a3431)
                } else {
                    false
                }
            } {
                f179(v3, v4)
            } else {
                Adt11::C2(v3, v4, false)
            }
        } else {
            Adt11::C0(v2)
        }
    } else {
        let a3432 = v1;
        let a3433 = 1u64;
        if nat_eq(a3432, a3433) {
            Adt11::C1(v2)
        } else {
            let a3434 = v1;
            let a3435 = 2u64;
            if nat_eq(a3434, a3435) {
                Adt11::C5(v2)
            } else {
                let a3436 = v1;
                let a3437 = 3u64;
                if nat_eq(a3436, a3437) {
                    Adt11::C3(v2, v2, false)
                } else {
                    let a3438 = v1;
                    let a3439 = 4u64;
                    if nat_eq(a3438, a3439) {
                        Adt11::C2(v3, v4, false)
                    } else {
                        if f97(v0, v1, v2, v3, v4) {
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

pub fn f99(v0: u64, v1: u64) -> Adt12 {
    let a3440 = v0;
    let a3441 = 0u64;
    if nat_eq(a3440, a3441) {
        Adt12::C0(0u64, v1)
    } else {
        let a3442 = v0;
        let a3443 = 1u64;
        if nat_eq(a3442, a3443) {
            Adt12::C1(v1)
        } else {
            let a3444 = v0;
            let a3445 = 2u64;
            if nat_eq(a3444, a3445) {
                Adt12::C2(v1)
            } else {
                let a3446 = v0;
                let a3447 = 3u64;
                if nat_eq(a3446, a3447) {
                    Adt12::C3(v1)
                } else {
                    let a3448 = v0;
                    let a3449 = 13u64;
                    if nat_lt(a3448, a3449) {
                        Adt12::C0({
                            let a3450 = v0;
                            let a3451 = 3u64;
                            nat_sub(a3450, a3451)
                        }, v1)
                    } else {
                        Adt12::C4
                    }
                }
            }
        }
    }
}

pub fn f100(v0: u64, v1: u64) -> Adt13 {
    let a3452 = v0;
    let a3453 = 1u64;
    if nat_eq(a3452, a3453) {
        Adt13::C0(v1)
    } else {
        let a3454 = v0;
        let a3455 = 3u64;
        if if nat_eq(a3454, a3455) {
            true
        } else {
            let a3456 = v0;
            let a3457 = 6u64;
            nat_eq(a3456, a3457)
        } {
            Adt13::C1(v1)
        } else {
            Adt13::C2
        }
    }
}

pub fn f101(v0: u64, v1: u64) -> R<u64> {
    let a3464 = v1;
    let a3465 = {
        let a3462 = v1;
        let a3463 = {
            let a3460 = 1u64;
            let a3461 = {
                let a3458 = 5u64;
                let a3459 = v0;
                nat_mul(a3458, a3459)?
            };
            nat_add(a3460, a3461)?
        };
        nat_mul(a3462, a3463)?
    };
    nat_add(a3464, a3465)
}

pub fn f102(v0: bool, v1: u64, v2: u64) -> R<u64> {
    let a3474 = v2;
    let a3475 = {
        let a3472 = v2;
        let a3473 = if v0 {
            let a3468 = 1u64;
            let a3469 = {
                let a3466 = 9u64;
                let a3467 = v1;
                nat_mul(a3466, a3467)?
            };
            nat_add(a3468, a3469)?
        } else {
            let a3470 = 7u64;
            let a3471 = v1;
            nat_mul(a3470, a3471)?
        };
        nat_mul(a3472, a3473)?
    };
    nat_add(a3474, a3475)
}

pub fn f103(v0: u64, v1: Adt3) -> R<u64> {
    let v2: u64 = {
        let a3477 = v0;
        let a3478 = match v1.clone() {
            Adt3::C0(h3476, _, _, _, _, _, _, _) => {
                h3476
            }
        };
        let a3479 = 0u64;
        nat_quot(a3477, a3478, a3479)
    };
    let v3: u64 = {
        let a3481 = v2;
        let a3482 = match v1.clone() {
            Adt3::C0(_, h3480, _, _, _, _, _, _) => {
                h3480
            }
        };
        let a3483 = 0u64;
        nat_quot(a3481, a3482, a3483)
    };
    f154({
        let a3485 = v0;
        let a3486 = match v1.clone() {
            Adt3::C0(h3484, _, _, _, _, _, _, _) => {
                h3484
            }
        };
        let a3487 = 0u64;
        nat_rem(a3485, a3486, a3487)
    }, {
        let a3489 = v2;
        let a3490 = match v1.clone() {
            Adt3::C0(_, h3488, _, _, _, _, _, _) => {
                h3488
            }
        };
        let a3491 = 0u64;
        nat_rem(a3489, a3490, a3491)
    }, {
        let a3493 = v3;
        let a3494 = match v1.clone() {
            Adt3::C0(_, _, h3492, _, _, _, _, _) => {
                h3492
            }
        };
        let a3495 = 0u64;
        nat_rem(a3493, a3494, a3495)
    }, {
        let a3497 = v3;
        let a3498 = match v1.clone() {
            Adt3::C0(_, _, h3496, _, _, _, _, _) => {
                h3496
            }
        };
        let a3499 = 0u64;
        nat_quot(a3497, a3498, a3499)
    }, match v1.clone() {
        Adt3::C0(_, _, _, _, h3500, _, _, _) => {
            h3500
        }
    }, match v1.clone() {
        Adt3::C0(_, _, _, _, _, h3501, _, _) => {
            h3501
        }
    }, match v1.clone() {
        Adt3::C0(_, _, _, _, _, _, h3502, _) => {
            h3502
        }
    }, match v1 {
        Adt3::C0(_, _, _, _, _, _, _, h3503) => {
            h3503
        }
    })
}

pub fn f104(v0: Adt6, v1: u64) -> R<u64> {
    let a3505 = 0u64;
    let a3506 = match v0.clone() {
        Adt6::C0(h3504, _, _, _) => {
            h3504
        }
    };
    if if if if if nat_lt(a3505, a3506) {
        let a3508 = 0u64;
        let a3509 = match v0.clone() {
            Adt6::C0(_, h3507, _, _) => {
                h3507
            }
        };
        nat_lt(a3508, a3509)
    } else {
        false
    } {
        let a3511 = 0u64;
        let a3512 = match v0.clone() {
            Adt6::C0(_, _, _, h3510) => {
                h3510
            }
        };
        nat_lt(a3511, a3512)
    } else {
        false
    } {
        let a3514 = match v0.clone() {
            Adt6::C0(_, h3513, _, _) => {
                h3513
            }
        };
        let a3515 = 2147483648u64;
        nat_lt(a3514, a3515)
    } else {
        false
    } {
        let a3517 = match v0.clone() {
            Adt6::C0(_, _, _, h3516) => {
                h3516
            }
        };
        let a3518 = 2147483648u64;
        nat_lt(a3517, a3518)
    } else {
        false
    } {
        let v2: u64 = {
            let a3522 = v1;
            let a3523 = {
                let a3520 = 2u64;
                let a3521 = f156(match v0.clone() {
                    Adt6::C0(_, _, h3519, _) => {
                        h3519
                    }
                });
                nat_mul(a3520, a3521)?
            };
            nat_add(a3522, a3523)?
        };
        let v3: u64 = {
            let a3535 = {
                let a3530 = 1u64;
                let a3531 = {
                    let a3528 = match v0.clone() {
                        Adt6::C0(_, _, _, h3524) => {
                            h3524
                        }
                    };
                    let a3529 = {
                        let a3526 = match v0.clone() {
                            Adt6::C0(h3525, _, _, _) => {
                                h3525
                            }
                        };
                        let a3527 = 1u64;
                        nat_sub(a3526, a3527)
                    };
                    nat_mul(a3528, a3529)?
                };
                nat_add(a3530, a3531)?
            };
            let a3536 = {
                let a3533 = 2u64;
                let a3534 = f155(match v0.clone() {
                    Adt6::C0(_, _, h3532, _) => {
                        h3532
                    }
                });
                nat_mul(a3533, a3534)?
            };
            nat_add(a3535, a3536)?
        };
        let a3537 = v3;
        let a3538 = v2;
        if nat_le(a3537, a3538) {
            let a3545 = 1u64;
            let a3546 = {
                let a3542 = {
                    let a3539 = v2;
                    let a3540 = v3;
                    nat_sub(a3539, a3540)
                };
                let a3543 = match v0 {
                    Adt6::C0(_, h3541, _, _) => {
                        h3541
                    }
                };
                let a3544 = 0u64;
                nat_quot(a3542, a3543, a3544)
            };
            nat_add(a3545, a3546)
        } else {
            let a3550 = {
                let a3547 = v3;
                let a3548 = v2;
                nat_sub(a3547, a3548)
            };
            let a3551 = match v0 {
                Adt6::C0(_, h3549, _, _) => {
                    h3549
                }
            };
            if nat_lt(a3550, a3551) {
                Ok(1u64)
            } else {
                Ok(0u64)
            }
        }
    } else {
        Ok(0u64)
    }
}

pub fn f105(v0: u64) -> u64 {
    let v1: u64 = {
        let a3552 = v0;
        let a3553 = 4294967296u64;
        let a3554 = 0u64;
        nat_rem(a3552, a3553, a3554)
    };
    let a3555 = v1;
    let a3556 = 2147483648u64;
    if nat_lt(a3555, a3556) {
        v1
    } else {
        0u64
    }
}

pub fn f106(v0: u64) -> R<u64> {
    let a3559 = 2u64;
    let a3560 = {
        let a3557 = 2u64;
        let a3558 = v0;
        nat_mul(a3557, a3558)?
    };
    nat_add(a3559, a3560)
}

pub fn f107(v0: u64, v1: Adt14, v2: Adt14) -> R<Adt15> {
    let a3562 = match v1.clone() {
        Adt14::C0(h3561, _, _, _, _, _, _, _, _, _, _) => {
            h3561
        }
    };
    let a3563 = 0u64;
    if if if if nat_eq(a3562, a3563) {
        let a3565 = match v2.clone() {
            Adt14::C0(h3564, _, _, _, _, _, _, _, _, _, _) => {
                h3564
            }
        };
        let a3566 = 0u64;
        nat_eq(a3565, a3566)
    } else {
        false
    } {
        f213(v1.clone(), v2.clone())
    } else {
        false
    } {
        if f214(match v1.clone() {
            Adt14::C0(_, h3567, _, _, _, _, _, _, _, _, _) => {
                h3567
            }
        }, match v1.clone() {
            Adt14::C0(_, _, h3568, _, _, _, _, _, _, _, _) => {
                h3568
            }
        }, match v1.clone() {
            Adt14::C0(_, _, _, h3569, _, _, _, _, _, _, _) => {
                h3569
            }
        }, match v1.clone() {
            Adt14::C0(_, _, _, _, h3570, _, _, _, _, _, _) => {
                h3570
            }
        }) {
            true
        } else {
            if if if if f50(v0) {
                f215(v1.clone(), v2.clone())
            } else {
                false
            } {
                f216(v1.clone())
            } else {
                false
            } {
                f216(v2.clone())
            } else {
                false
            } {
                if f217(v1.clone(), v2.clone())? {
                    true
                } else {
                    f218(v1.clone(), v2.clone())
                }
            } else {
                false
            }
        }
    } else {
        false
    } {
        let v3: u64 = if f214(match v1.clone() {
            Adt14::C0(_, h3571, _, _, _, _, _, _, _, _, _) => {
                h3571
            }
        }, match v1.clone() {
            Adt14::C0(_, _, h3572, _, _, _, _, _, _, _, _) => {
                h3572
            }
        }, match v1.clone() {
            Adt14::C0(_, _, _, h3573, _, _, _, _, _, _, _) => {
                h3573
            }
        }, match v1.clone() {
            Adt14::C0(_, _, _, _, h3574, _, _, _, _, _, _) => {
                h3574
            }
        }) {
            0u64
        } else {
            let a3580 = match v1.clone() {
                Adt14::C0(_, _, h3575, _, _, _, _, _, _, _, _) => {
                    h3575
                }
            };
            let a3581 = {
                let a3578 = match v1.clone() {
                    Adt14::C0(_, _, _, h3576, _, _, _, _, _, _, _) => {
                        h3576
                    }
                };
                let a3579 = match v1.clone() {
                    Adt14::C0(_, _, _, _, h3577, _, _, _, _, _, _) => {
                        h3577
                    }
                };
                nat_mul(a3578, a3579)?
            };
            nat_mul(a3580, a3581)?
        };
        let v4: u64 = {
            let a3582 = v3;
            let a3583 = 0u64;
            if if nat_eq(a3582, a3583) {
                true
            } else {
                f219(v2.clone())?
            } {
                0u64
            } else {
                let a3585 = match v2.clone() {
                    Adt14::C0(_, _, _, _, _, _, _, _, _, h3584, _) => {
                        h3584
                    }
                };
                let a3586 = 4u64;
                let a3587 = 0u64;
                nat_quot(a3585, a3586, a3587)
            }
        };
        Ok(Adt15::C1(f220(f221(match v1.clone() {
            Adt14::C0(_, _, _, _, _, h3588, _, _, _, _, _) => {
                h3588
            }
        }, match v2 {
            Adt14::C0(_, _, _, _, _, h3589, _, _, _, _, _) => {
                h3589
            }
        }), match v1 {
            Adt14::C0(_, h3590, _, _, _, _, _, _, _, _, _) => {
                h3590
            }
        }, v3, v4)?, 4u64, 64u64, v3, f222(v4)?, v0))
    } else {
        Ok(Adt15::C0)
    }
}

pub fn f108(v0: u64, v1: u64, v2: Adt14, v3: Adt14) -> R<Adt15> {
    let v4: Adt14 = f223(v3.clone());
    let a3592 = match v2.clone() {
        Adt14::C0(h3591, _, _, _, _, _, _, _, _, _, _) => {
            h3591
        }
    };
    let a3593 = 0u64;
    if if if if nat_eq(a3592, a3593) {
        let a3595 = match v3.clone() {
            Adt14::C0(h3594, _, _, _, _, _, _, _, _, _, _) => {
                h3594
            }
        };
        let a3596 = 0u64;
        nat_eq(a3595, a3596)
    } else {
        false
    } {
        f213(v2.clone(), v3)
    } else {
        false
    } {
        if f214(match v2.clone() {
            Adt14::C0(_, h3597, _, _, _, _, _, _, _, _, _) => {
                h3597
            }
        }, match v2.clone() {
            Adt14::C0(_, _, h3598, _, _, _, _, _, _, _, _) => {
                h3598
            }
        }, match v2.clone() {
            Adt14::C0(_, _, _, h3599, _, _, _, _, _, _, _) => {
                h3599
            }
        }, match v2.clone() {
            Adt14::C0(_, _, _, _, h3600, _, _, _, _, _, _) => {
                h3600
            }
        }) {
            true
        } else {
            let a3602 = match v2.clone() {
                Adt14::C0(_, _, _, _, _, h3601, _, _, _, _, _) => {
                    h3601
                }
            };
            let a3603 = 4u64;
            if if if if if nat_eq(a3602, a3603) {
                f224(match v2.clone() {
                    Adt14::C0(_, h3604, _, _, _, _, _, _, _, _, _) => {
                        h3604
                    }
                }, match v2.clone() {
                    Adt14::C0(_, _, h3605, _, _, _, _, _, _, _, _) => {
                        h3605
                    }
                }, match v2.clone() {
                    Adt14::C0(_, _, _, h3606, _, _, _, _, _, _, _) => {
                        h3606
                    }
                }, v0)?
            } else {
                false
            } {
                f216(v2.clone())
            } else {
                false
            } {
                f216(v4.clone())
            } else {
                false
            } {
                let a3608 = f225(v4.clone())?;
                let a3609 = match v4.clone() {
                    Adt14::C0(_, _, _, _, _, _, _, _, _, h3607, _) => {
                        h3607
                    }
                };
                nat_le(a3608, a3609)
            } else {
                false
            } {
                if f217(v2.clone(), v4.clone())? {
                    true
                } else {
                    f218(v2.clone(), v4.clone())
                }
            } else {
                false
            }
        }
    } else {
        false
    } {
        let v5: u64 = if if f214(match v2.clone() {
            Adt14::C0(_, h3610, _, _, _, _, _, _, _, _, _) => {
                h3610
            }
        }, match v2.clone() {
            Adt14::C0(_, _, h3611, _, _, _, _, _, _, _, _) => {
                h3611
            }
        }, match v2.clone() {
            Adt14::C0(_, _, _, h3612, _, _, _, _, _, _, _) => {
                h3612
            }
        }, match v2.clone() {
            Adt14::C0(_, _, _, _, h3613, _, _, _, _, _, _) => {
                h3613
            }
        }) {
            true
        } else {
            f219(v4.clone())?
        } {
            0u64
        } else {
            let a3615 = match v4 {
                Adt14::C0(_, _, _, _, _, _, _, _, _, h3614, _) => {
                    h3614
                }
            };
            let a3616 = 4u64;
            let a3617 = 0u64;
            nat_quot(a3615, a3616, a3617)
        };
        Ok(Adt15::C1(f226(match v2.clone() {
            Adt14::C0(_, h3618, _, _, _, _, _, _, _, _, _) => {
                h3618
            }
        }, match v2.clone() {
            Adt14::C0(_, _, h3619, _, _, _, _, _, _, _, _) => {
                h3619
            }
        }, match v2.clone() {
            Adt14::C0(_, _, _, h3620, _, _, _, _, _, _, _) => {
                h3620
            }
        }, match v2 {
            Adt14::C0(_, _, _, _, h3621, _, _, _, _, _, _) => {
                h3621
            }
        }, v0, v5)?, 4u64, 65u64, v0, f222(v5)?, v1))
    } else {
        Ok(Adt15::C0)
    }
}

pub fn f109(v0: u64, v1: bool, v2: Adt14, v3: Adt14, v4: Adt14) -> R<Adt15> {
    let v5: Adt14 = f223(v4.clone());
    if if if if if if if v1 {
        let a3623 = match v2.clone() {
            Adt14::C0(h3622, _, _, _, _, _, _, _, _, _, _) => {
                h3622
            }
        };
        let a3624 = 0u64;
        nat_eq(a3623, a3624)
    } else {
        false
    } {
        let a3626 = match v3.clone() {
            Adt14::C0(h3625, _, _, _, _, _, _, _, _, _, _) => {
                h3625
            }
        };
        let a3627 = 0u64;
        nat_eq(a3626, a3627)
    } else {
        false
    } {
        let a3629 = match v4.clone() {
            Adt14::C0(h3628, _, _, _, _, _, _, _, _, _, _) => {
                h3628
            }
        };
        let a3630 = 0u64;
        nat_eq(a3629, a3630)
    } else {
        false
    } {
        f213(v2.clone(), v3.clone())
    } else {
        false
    } {
        f213(v2.clone(), v4)
    } else {
        false
    } {
        if f214(match v2.clone() {
            Adt14::C0(_, h3631, _, _, _, _, _, _, _, _, _) => {
                h3631
            }
        }, match v2.clone() {
            Adt14::C0(_, _, h3632, _, _, _, _, _, _, _, _) => {
                h3632
            }
        }, match v2.clone() {
            Adt14::C0(_, _, _, h3633, _, _, _, _, _, _, _) => {
                h3633
            }
        }, match v2.clone() {
            Adt14::C0(_, _, _, _, h3634, _, _, _, _, _, _) => {
                h3634
            }
        }) {
            true
        } else {
            let a3636 = match v2.clone() {
                Adt14::C0(_, _, _, _, _, h3635, _, _, _, _, _) => {
                    h3635
                }
            };
            let a3637 = 4u64;
            if if if if if if if nat_eq(a3636, a3637) {
                let a3639 = match v3.clone() {
                    Adt14::C0(_, _, _, _, _, h3638, _, _, _, _, _) => {
                        h3638
                    }
                };
                let a3640 = 4u64;
                nat_eq(a3639, a3640)
            } else {
                false
            } {
                f216(v2.clone())
            } else {
                false
            } {
                f216(v3.clone())
            } else {
                false
            } {
                f216(v5.clone())
            } else {
                false
            } {
                let a3642 = f225(v5.clone())?;
                let a3643 = match v5.clone() {
                    Adt14::C0(_, _, _, _, _, _, _, _, _, h3641, _) => {
                        h3641
                    }
                };
                nat_le(a3642, a3643)
            } else {
                false
            } {
                if f217(v2.clone(), v5.clone())? {
                    true
                } else {
                    f218(v2.clone(), v5.clone())
                }
            } else {
                false
            } {
                if f217(v3.clone(), v5.clone())? {
                    true
                } else {
                    f218(v3, v5.clone())
                }
            } else {
                false
            }
        }
    } else {
        false
    } {
        let v6: u64 = if f214(match v2.clone() {
            Adt14::C0(_, h3644, _, _, _, _, _, _, _, _, _) => {
                h3644
            }
        }, match v2.clone() {
            Adt14::C0(_, _, h3645, _, _, _, _, _, _, _, _) => {
                h3645
            }
        }, match v2.clone() {
            Adt14::C0(_, _, _, h3646, _, _, _, _, _, _, _) => {
                h3646
            }
        }, match v2.clone() {
            Adt14::C0(_, _, _, _, h3647, _, _, _, _, _, _) => {
                h3647
            }
        }) {
            0u64
        } else {
            let a3653 = match v2.clone() {
                Adt14::C0(_, _, h3648, _, _, _, _, _, _, _, _) => {
                    h3648
                }
            };
            let a3654 = {
                let a3651 = match v2.clone() {
                    Adt14::C0(_, _, _, h3649, _, _, _, _, _, _, _) => {
                        h3649
                    }
                };
                let a3652 = match v2.clone() {
                    Adt14::C0(_, _, _, _, h3650, _, _, _, _, _, _) => {
                        h3650
                    }
                };
                nat_mul(a3651, a3652)?
            };
            nat_mul(a3653, a3654)?
        };
        let v7: u64 = {
            let a3655 = v6;
            let a3656 = 0u64;
            if if nat_eq(a3655, a3656) {
                true
            } else {
                f219(v5.clone())?
            } {
                0u64
            } else {
                let a3658 = match v5 {
                    Adt14::C0(_, _, _, _, _, _, _, _, _, h3657, _) => {
                        h3657
                    }
                };
                let a3659 = 4u64;
                let a3660 = 0u64;
                nat_quot(a3658, a3659, a3660)
            }
        };
        Ok(Adt15::C1(f227(match v2 {
            Adt14::C0(_, h3661, _, _, _, _, _, _, _, _, _) => {
                h3661
            }
        }, v6, v7)?, 4u64, 66u64, v6, f222(v7)?, v0))
    } else {
        Ok(Adt15::C0)
    }
}

pub fn f110(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64) -> bool {
    if if f228(v0, v3, v6) {
        f228(v1, v4, v6)
    } else {
        false
    } {
        f228(v2, v5, v6)
    } else {
        false
    }
}

pub fn f111(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64) -> R<bool> {
    if if if if if f229(v0, v1, v4, v2, v5, v3, v6)? {
        true
    } else {
        f229(v0, v1, v4, v3, v6, v2, v5)?
    } {
        true
    } else {
        f229(v0, v2, v5, v1, v4, v3, v6)?
    } {
        true
    } else {
        f229(v0, v2, v5, v3, v6, v1, v4)?
    } {
        true
    } else {
        f229(v0, v3, v6, v1, v4, v2, v5)?
    } {
        Ok(true)
    } else {
        f229(v0, v3, v6, v2, v5, v1, v4)
    }
}

pub fn f112(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let a3662 = v2;
    let a3663 = 0u64;
    if nat_eq(a3662, a3663) {
        Ok(0u64)
    } else {
        let a3666 = 2u64;
        let a3667 = {
            let a3664 = v0;
            let a3665 = v1;
            nat_mul(a3664, a3665)?
        };
        nat_mul(a3666, a3667)
    }
}

pub fn f113(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let a3668 = v2;
    let a3669 = f209(v0, v1)?;
    nat_mul(a3668, a3669)
}

pub fn f114(v0: u64) -> u64 {
    let a3670 = v0;
    let a3671 = 30u64;
    if nat_eq(a3670, a3671) {
        3u64
    } else {
        1u64
    }
}

pub fn f115(v0: u64) -> R<u64> {
    let a3674 = 5u64;
    let a3675 = {
        let a3672 = 4u64;
        let a3673 = v0;
        nat_mul(a3672, a3673)?
    };
    nat_add(a3674, a3675)
}

pub fn f116(v0: u64) -> bool {
    let m3676 = f230(v0);
    match m3676 {
        Adt18::C0 => {
            false
        }
        Adt18::C1(v1) => {
            let a3678 = 1u64;
            let a3679 = match v1 {
                Adt19::C0(_, h3677, _) => {
                    h3677
                }
            };
            nat_lt(a3678, a3679)
        }
    }
}

pub fn f117(v0: Adt1, v1: Adt2, v2: Adt2) -> R<Adt10> {
    let v3: u64 = match v0.clone() {
        Adt1::C0(h3680, _, _, _, _, _, _, _, _, _, _, _) => {
            h3680
        }
    };
    let v4: u64 = match v0 {
        Adt1::C0(_, h3681, _, _, _, _, _, _, _, _, _, _) => {
            h3681
        }
    };
    let v5: u64 = match v1.clone() {
        Adt2::C0(_, h3682, _, _, _, _, _, _, _, _, _) => {
            h3682
        }
    };
    let a3684 = match v1.clone() {
        Adt2::C0(h3683, _, _, _, _, _, _, _, _, _, _) => {
            h3683
        }
    };
    let a3685 = 0u64;
    if if if if if if if if if if if if if nat_eq(a3684, a3685) {
        let a3687 = match v2.clone() {
            Adt2::C0(h3686, _, _, _, _, _, _, _, _, _, _) => {
                h3686
            }
        };
        let a3688 = 0u64;
        nat_eq(a3687, a3688)
    } else {
        false
    } {
        let a3690 = match v1.clone() {
            Adt2::C0(_, _, _, _, _, h3689, _, _, _, _, _) => {
                h3689
            }
        };
        let a3691 = 4u64;
        nat_eq(a3690, a3691)
    } else {
        false
    } {
        let a3693 = match v2.clone() {
            Adt2::C0(_, h3692, _, _, _, _, _, _, _, _, _) => {
                h3692
            }
        };
        let a3694 = v3;
        nat_eq(a3693, a3694)
    } else {
        false
    } {
        let a3696 = match v2.clone() {
            Adt2::C0(_, _, h3695, _, _, _, _, _, _, _, _) => {
                h3695
            }
        };
        let a3697 = v5;
        nat_eq(a3696, a3697)
    } else {
        false
    } {
        let a3699 = match v2.clone() {
            Adt2::C0(_, _, _, h3698, _, _, _, _, _, _, _) => {
                h3698
            }
        };
        let a3700 = 1u64;
        nat_eq(a3699, a3700)
    } else {
        false
    } {
        let a3702 = match v2.clone() {
            Adt2::C0(_, _, _, _, h3701, _, _, _, _, _, _) => {
                h3701
            }
        };
        let a3703 = 1u64;
        nat_eq(a3702, a3703)
    } else {
        false
    } {
        let a3704 = v3;
        let a3705 = 2147483648u64;
        nat_lt(a3704, a3705)
    } else {
        false
    } {
        let a3706 = v4;
        let a3707 = 4294967296u64;
        nat_lt(a3706, a3707)
    } else {
        false
    } {
        f43(v2.clone())?
    } else {
        false
    } {
        let a3711 = {
            let a3708 = v5;
            let a3709 = 4u64;
            nat_mul(a3708, a3709)?
        };
        let a3712 = match v1.clone() {
            Adt2::C0(_, _, _, _, _, _, _, _, _, h3710, _) => {
                h3710
            }
        };
        nat_le(a3711, a3712)
    } else {
        false
    } {
        f31(v2.clone(), f28(v2.clone(), 1u64)?)?
    } else {
        false
    } {
        f35(v1.clone(), v2.clone())?
    } else {
        false
    } {
        Ok(Adt10::C4({
            let a3713 = v5;
            let a3714 = 0u64;
            if nat_eq(a3713, a3714) {
                0u64
            } else {
                f231(v3, v5)?
            }
        }, 4u64, f30(v1, 1u64), f30(v2, 1u64), Adt4::C44(v4)))
    } else {
        Ok(Adt10::C1)
    }
}

pub fn f118(v0: Adt1, v1: Adt2, v2: Adt2, v3: Adt2) -> R<Adt10> {
    if if if if if if if if f26(v2.clone(), v3.clone()) {
        let a3716 = match v3.clone() {
            Adt2::C0(h3715, _, _, _, _, _, _, _, _, _, _) => {
                h3715
            }
        };
        let a3717 = 0u64;
        nat_eq(a3716, a3717)
    } else {
        false
    } {
        f43(v3.clone())?
    } else {
        false
    } {
        f31(v3.clone(), f28(v3.clone(), 1u64)?)?
    } else {
        false
    } {
        f31(v1.clone(), f28(v1.clone(), 1u64)?)?
    } else {
        false
    } {
        f35(v2.clone(), v3.clone())?
    } else {
        false
    } {
        let a3719 = match v2.clone() {
            Adt2::C0(h3718, _, _, _, _, _, _, _, _, _, _) => {
                h3718
            }
        };
        let a3720 = 1u64;
        nat_le(a3719, a3720)
    } else {
        false
    } {
        let a3722 = match v0.clone() {
            Adt1::C0(h3721, _, _, _, _, _, _, _, _, _, _, _) => {
                h3721
            }
        };
        let a3723 = 0u64;
        if nat_eq(a3722, a3723) {
            let a3725 = match v2.clone() {
                Adt2::C0(h3724, _, _, _, _, _, _, _, _, _, _) => {
                    h3724
                }
            };
            let a3726 = 0u64;
            if if nat_eq(a3725, a3726) {
                f43(v2.clone())?
            } else {
                false
            } {
                f31(v2.clone(), f28(v2, 1u64)?)?
            } else {
                false
            }
        } else {
            true
        }
    } else {
        false
    } {
        f127(true, true, v0, v3, v1)
    } else {
        Ok(Adt10::C1)
    }
}

pub fn f119(v0: Adt1, v1: Adt2, v2: Adt2, v3: Adt2) -> R<Adt10> {
    let a3728 = match v1.clone() {
        Adt2::C0(h3727, _, _, _, _, _, _, _, _, _, _) => {
            h3727
        }
    };
    let a3729 = 0u64;
    if if if if if if if nat_eq(a3728, a3729) {
        let a3731 = match v3.clone() {
            Adt2::C0(h3730, _, _, _, _, _, _, _, _, _, _) => {
                h3730
            }
        };
        let a3732 = 0u64;
        nat_eq(a3731, a3732)
    } else {
        false
    } {
        let a3734 = match v2.clone() {
            Adt2::C0(h3733, _, _, _, _, _, _, _, _, _, _) => {
                h3733
            }
        };
        let a3735 = 1u64;
        nat_le(a3734, a3735)
    } else {
        false
    } {
        f43(v1.clone())?
    } else {
        false
    } {
        f31(v1.clone(), f28(v1.clone(), 1u64)?)?
    } else {
        false
    } {
        f31(v3.clone(), f28(v3.clone(), 1u64)?)?
    } else {
        false
    } {
        let a3737 = match v0.clone() {
            Adt1::C0(_, _, _, _, _, _, h3736, _, _, _, _, _) => {
                h3736
            }
        };
        let a3738 = 0u64;
        if nat_eq(a3737, a3738) {
            true
        } else {
            let a3740 = match v3.clone() {
                Adt2::C0(_, _, h3739, _, _, _, _, _, _, _, _) => {
                    h3739
                }
            };
            let a3741 = 1u64;
            if nat_le(a3740, a3741) {
                true
            } else {
                let a3746 = match v3.clone() {
                    Adt2::C0(_, _, _, _, _, _, h3742, _, _, _, _) => {
                        h3742
                    }
                };
                let a3747 = {
                    let a3744 = 4u64;
                    let a3745 = match v3.clone() {
                        Adt2::C0(_, h3743, _, _, _, _, _, _, _, _, _) => {
                            h3743
                        }
                    };
                    nat_mul(a3744, a3745)?
                };
                nat_eq(a3746, a3747)
            }
        }
    } else {
        false
    } {
        f128(true, v0, v2, Some(v3), v1)
    } else {
        Ok(Adt10::C1)
    }
}

pub fn f120(v0: Adt2, v1: Adt2, v2: Adt2) -> R<Adt10> {
    let a3749 = match v0.clone() {
        Adt2::C0(h3748, _, _, _, _, _, _, _, _, _, _) => {
            h3748
        }
    };
    let a3750 = 0u64;
    if if if if if if if if if if if if nat_eq(a3749, a3750) {
        f43(v0.clone())?
    } else {
        false
    } {
        f31(v0.clone(), f28(v0.clone(), 1u64)?)?
    } else {
        false
    } {
        let a3752 = match v1.clone() {
            Adt2::C0(h3751, _, _, _, _, _, _, _, _, _, _) => {
                h3751
            }
        };
        let a3753 = 0u64;
        if if nat_eq(a3752, a3753) {
            f43(v1.clone())?
        } else {
            false
        } {
            f31(v1.clone(), f28(v1.clone(), 1u64)?)?
        } else {
            false
        }
    } else {
        false
    } {
        let a3755 = match v2.clone() {
            Adt2::C0(h3754, _, _, _, _, _, _, _, _, _, _) => {
                h3754
            }
        };
        let a3756 = 0u64;
        if if nat_eq(a3755, a3756) {
            f43(v2.clone())?
        } else {
            false
        } {
            f31(v2.clone(), f28(v2.clone(), 1u64)?)?
        } else {
            false
        }
    } else {
        false
    } {
        let a3759 = match v0.clone() {
            Adt2::C0(_, h3757, _, _, _, _, _, _, _, _, _) => {
                h3757
            }
        };
        let a3760 = match v0.clone() {
            Adt2::C0(_, _, h3758, _, _, _, _, _, _, _, _) => {
                h3758
            }
        };
        nat_eq(a3759, a3760)
    } else {
        false
    } {
        let a3763 = match v0.clone() {
            Adt2::C0(_, _, h3761, _, _, _, _, _, _, _, _) => {
                h3761
            }
        };
        let a3764 = match v1.clone() {
            Adt2::C0(_, _, h3762, _, _, _, _, _, _, _, _) => {
                h3762
            }
        };
        nat_eq(a3763, a3764)
    } else {
        false
    } {
        let a3767 = match v0.clone() {
            Adt2::C0(_, _, _, h3765, _, _, _, _, _, _, _) => {
                h3765
            }
        };
        let a3768 = match v1.clone() {
            Adt2::C0(_, _, _, h3766, _, _, _, _, _, _, _) => {
                h3766
            }
        };
        nat_eq(a3767, a3768)
    } else {
        false
    } {
        let a3771 = match v0.clone() {
            Adt2::C0(_, _, _, _, h3769, _, _, _, _, _, _) => {
                h3769
            }
        };
        let a3772 = match v1.clone() {
            Adt2::C0(_, _, _, _, h3770, _, _, _, _, _, _) => {
                h3770
            }
        };
        nat_eq(a3771, a3772)
    } else {
        false
    } {
        f26(v1.clone(), v2.clone())
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
            let a3781 = {
                let a3778 = {
                    let a3775 = match v1.clone() {
                        Adt2::C0(_, _, _, h3773, _, _, _, _, _, _, _) => {
                            h3773
                        }
                    };
                    let a3776 = match v1.clone() {
                        Adt2::C0(_, _, _, _, h3774, _, _, _, _, _, _) => {
                            h3774
                        }
                    };
                    nat_mul(a3775, a3776)?
                };
                let a3779 = match v1 {
                    Adt2::C0(_, h3777, _, _, _, _, _, _, _, _, _) => {
                        h3777
                    }
                };
                nat_mul(a3778, a3779)?
            };
            let a3782 = f190(match v0.clone() {
                Adt2::C0(_, h3780, _, _, _, _, _, _, _, _, _) => {
                    h3780
                }
            })?;
            nat_mul(a3781, a3782)?
        }, 4u64, f30(v0, 1u64), f30(v2, 1u64), Adt4::C41))
    } else {
        Ok(Adt10::C1)
    }
}

pub fn f121(v0: Adt1, v1: Adt2, v2: Adt2, v3: Adt2) -> R<Adt10> {
    let v4: bool = {
        let a3784 = 0u64;
        let a3785 = match v0.clone() {
            Adt1::C0(_, h3783, _, _, _, _, _, _, _, _, _, _) => {
                h3783
            }
        };
        nat_lt(a3784, a3785)
    };
    let a3787 = match v1.clone() {
        Adt2::C0(h3786, _, _, _, _, _, _, _, _, _, _) => {
            h3786
        }
    };
    let a3788 = 0u64;
    if if if if if if if if if if if if if if if if nat_eq(a3787, a3788) {
        let a3790 = match v2.clone() {
            Adt2::C0(h3789, _, _, _, _, _, _, _, _, _, _) => {
                h3789
            }
        };
        let a3791 = 0u64;
        nat_eq(a3790, a3791)
    } else {
        false
    } {
        let a3793 = match v3.clone() {
            Adt2::C0(h3792, _, _, _, _, _, _, _, _, _, _) => {
                h3792
            }
        };
        let a3794 = 0u64;
        nat_eq(a3793, a3794)
    } else {
        false
    } {
        let a3796 = match v1.clone() {
            Adt2::C0(_, _, _, _, h3795, _, _, _, _, _, _) => {
                h3795
            }
        };
        let a3797 = 1u64;
        nat_eq(a3796, a3797)
    } else {
        false
    } {
        let a3799 = match v2.clone() {
            Adt2::C0(_, _, _, _, h3798, _, _, _, _, _, _) => {
                h3798
            }
        };
        let a3800 = 1u64;
        nat_eq(a3799, a3800)
    } else {
        false
    } {
        let a3802 = match v3.clone() {
            Adt2::C0(_, _, _, _, h3801, _, _, _, _, _, _) => {
                h3801
            }
        };
        let a3803 = 1u64;
        nat_eq(a3802, a3803)
    } else {
        false
    } {
        let a3805 = 0u64;
        let a3806 = match v1.clone() {
            Adt2::C0(_, _, h3804, _, _, _, _, _, _, _, _) => {
                h3804
            }
        };
        nat_lt(a3805, a3806)
    } else {
        false
    } {
        let a3808 = match v3.clone() {
            Adt2::C0(_, _, _, h3807, _, _, _, _, _, _, _) => {
                h3807
            }
        };
        let a3809 = 1u64;
        nat_eq(a3808, a3809)
    } else {
        false
    } {
        let a3812 = match v3.clone() {
            Adt2::C0(_, h3810, _, _, _, _, _, _, _, _, _) => {
                h3810
            }
        };
        let a3813 = match v1.clone() {
            Adt2::C0(_, h3811, _, _, _, _, _, _, _, _, _) => {
                h3811
            }
        };
        nat_eq(a3812, a3813)
    } else {
        false
    } {
        let a3816 = match v3.clone() {
            Adt2::C0(_, _, h3814, _, _, _, _, _, _, _, _) => {
                h3814
            }
        };
        let a3817 = match v1.clone() {
            Adt2::C0(_, _, _, h3815, _, _, _, _, _, _, _) => {
                h3815
            }
        };
        nat_eq(a3816, a3817)
    } else {
        false
    } {
        if v4 {
            f26(v1.clone(), v2.clone())
        } else {
            let a3820 = match v2.clone() {
                Adt2::C0(_, h3818, _, _, _, _, _, _, _, _, _) => {
                    h3818
                }
            };
            let a3821 = match v1.clone() {
                Adt2::C0(_, _, h3819, _, _, _, _, _, _, _, _) => {
                    h3819
                }
            };
            if if nat_eq(a3820, a3821) {
                let a3824 = match v2.clone() {
                    Adt2::C0(_, _, h3822, _, _, _, _, _, _, _, _) => {
                        h3822
                    }
                };
                let a3825 = match v1.clone() {
                    Adt2::C0(_, _, _, h3823, _, _, _, _, _, _, _) => {
                        h3823
                    }
                };
                nat_eq(a3824, a3825)
            } else {
                false
            } {
                let a3827 = match v2.clone() {
                    Adt2::C0(_, _, _, h3826, _, _, _, _, _, _, _) => {
                        h3826
                    }
                };
                let a3828 = 1u64;
                nat_eq(a3827, a3828)
            } else {
                false
            }
        }
    } else {
        false
    } {
        f31(v1.clone(), f28(v1.clone(), 1u64)?)?
    } else {
        false
    } {
        f31(v2.clone(), f28(v2.clone(), 1u64)?)?
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
            let a3830 = f28(v3.clone(), 1u64)?;
            let a3831 = f181(match v1.clone() {
                Adt2::C0(_, _, h3829, _, _, _, _, _, _, _, _) => {
                    h3829
                }
            }, v4)?;
            nat_mul(a3830, a3831)?
        }, 4u64, f30(v1, 1u64), f30(v3, 1u64), Adt4::C36(f30(v2, 1u64), v4, match v0 {
            Adt1::C0(h3832, _, _, _, _, _, _, _, _, _, _, _) => {
                h3832
            }
        })))
    } else {
        Ok(Adt10::C1)
    }
}

pub fn f122(v0: Adt1, v1: Adt2, v2: Adt2, v3: Adt2) -> R<Adt10> {
    let v4: Adt6 = Adt6::C0(match v1.clone() {
        Adt2::C0(_, h3833, _, _, _, _, _, _, _, _, _) => {
            h3833
        }
    }, match v0.clone() {
        Adt1::C0(h3834, _, _, _, _, _, _, _, _, _, _, _) => {
            h3834
        }
    }, match v0.clone() {
        Adt1::C0(_, _, h3835, _, _, _, _, _, _, _, _, _) => {
            h3835
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, _, h3836, _, _, _, _, _, _, _) => {
            h3836
        }
    });
    let v5: Adt6 = Adt6::C0(match v1.clone() {
        Adt2::C0(_, _, h3837, _, _, _, _, _, _, _, _) => {
            h3837
        }
    }, match v0.clone() {
        Adt1::C0(_, h3838, _, _, _, _, _, _, _, _, _, _) => {
            h3838
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, h3839, _, _, _, _, _, _, _, _) => {
            h3839
        }
    }, match v0 {
        Adt1::C0(_, _, _, _, _, h3840, _, _, _, _, _, _) => {
            h3840
        }
    });
    let v6: u64 = match v1.clone() {
        Adt2::C0(h3841, _, _, _, _, _, _, _, _, _, _) => {
            h3841
        }
    };
    let v7: bool = {
        let a3844 = match v2.clone() {
            Adt2::C0(_, _, _, _, _, _, _, h3842, _, _, _) => {
                h3842
            }
        };
        let a3845 = match v2.clone() {
            Adt2::C0(_, _, _, _, _, h3843, _, _, _, _, _) => {
                h3843
            }
        };
        nat_lt(a3844, a3845)
    };
    let a3846 = v6;
    let a3847 = 0u64;
    if if if if if if if if if if if if if if if nat_eq(a3846, a3847) {
        true
    } else {
        let a3848 = v6;
        let a3849 = 1u64;
        nat_eq(a3848, a3849)
    } {
        let a3851 = match v2.clone() {
            Adt2::C0(h3850, _, _, _, _, _, _, _, _, _, _) => {
                h3850
            }
        };
        let a3852 = 0u64;
        nat_eq(a3851, a3852)
    } else {
        false
    } {
        let a3854 = match v3.clone() {
            Adt2::C0(h3853, _, _, _, _, _, _, _, _, _, _) => {
                h3853
            }
        };
        let a3855 = 0u64;
        nat_eq(a3854, a3855)
    } else {
        false
    } {
        let a3857 = match v1.clone() {
            Adt2::C0(_, _, _, h3856, _, _, _, _, _, _, _) => {
                h3856
            }
        };
        let a3858 = 1u64;
        nat_eq(a3857, a3858)
    } else {
        false
    } {
        let a3861 = match v1.clone() {
            Adt2::C0(_, _, _, _, h3859, _, _, _, _, _, _) => {
                h3859
            }
        };
        let a3862 = match v2.clone() {
            Adt2::C0(_, _, _, h3860, _, _, _, _, _, _, _) => {
                h3860
            }
        };
        nat_eq(a3861, a3862)
    } else {
        false
    } {
        let a3865 = match v3.clone() {
            Adt2::C0(_, _, _, h3863, _, _, _, _, _, _, _) => {
                h3863
            }
        };
        let a3866 = match v2.clone() {
            Adt2::C0(_, _, _, h3864, _, _, _, _, _, _, _) => {
                h3864
            }
        };
        nat_eq(a3865, a3866)
    } else {
        false
    } {
        let a3869 = match v3.clone() {
            Adt2::C0(_, _, _, _, h3867, _, _, _, _, _, _) => {
                h3867
            }
        };
        let a3870 = match v2.clone() {
            Adt2::C0(_, _, _, _, h3868, _, _, _, _, _, _) => {
                h3868
            }
        };
        nat_eq(a3869, a3870)
    } else {
        false
    } {
        let a3872 = 0u64;
        let a3873 = match v4.clone() {
            Adt6::C0(h3871, _, _, _) => {
                h3871
            }
        };
        if if if if if nat_lt(a3872, a3873) {
            let a3875 = 0u64;
            let a3876 = match v4.clone() {
                Adt6::C0(_, h3874, _, _) => {
                    h3874
                }
            };
            nat_lt(a3875, a3876)
        } else {
            false
        } {
            let a3878 = match v4.clone() {
                Adt6::C0(_, h3877, _, _) => {
                    h3877
                }
            };
            let a3879 = 2147483648u64;
            nat_lt(a3878, a3879)
        } else {
            false
        } {
            let a3881 = 0u64;
            let a3882 = match v4.clone() {
                Adt6::C0(_, _, _, h3880) => {
                    h3880
                }
            };
            nat_lt(a3881, a3882)
        } else {
            false
        } {
            let a3884 = match v4.clone() {
                Adt6::C0(_, _, _, h3883) => {
                    h3883
                }
            };
            let a3885 = 2147483648u64;
            nat_lt(a3884, a3885)
        } else {
            false
        } {
            let a3888 = match v3.clone() {
                Adt2::C0(_, h3886, _, _, _, _, _, _, _, _, _) => {
                    h3886
                }
            };
            let a3889 = f104(v4.clone(), match v2.clone() {
                Adt2::C0(_, h3887, _, _, _, _, _, _, _, _, _) => {
                    h3887
                }
            })?;
            nat_eq(a3888, a3889)
        } else {
            false
        }
    } else {
        false
    } {
        let a3891 = 0u64;
        let a3892 = match v5.clone() {
            Adt6::C0(h3890, _, _, _) => {
                h3890
            }
        };
        if if if if if nat_lt(a3891, a3892) {
            let a3894 = 0u64;
            let a3895 = match v5.clone() {
                Adt6::C0(_, h3893, _, _) => {
                    h3893
                }
            };
            nat_lt(a3894, a3895)
        } else {
            false
        } {
            let a3897 = match v5.clone() {
                Adt6::C0(_, h3896, _, _) => {
                    h3896
                }
            };
            let a3898 = 2147483648u64;
            nat_lt(a3897, a3898)
        } else {
            false
        } {
            let a3900 = 0u64;
            let a3901 = match v5.clone() {
                Adt6::C0(_, _, _, h3899) => {
                    h3899
                }
            };
            nat_lt(a3900, a3901)
        } else {
            false
        } {
            let a3903 = match v5.clone() {
                Adt6::C0(_, _, _, h3902) => {
                    h3902
                }
            };
            let a3904 = 2147483648u64;
            nat_lt(a3903, a3904)
        } else {
            false
        } {
            let a3907 = match v3.clone() {
                Adt2::C0(_, _, h3905, _, _, _, _, _, _, _, _) => {
                    h3905
                }
            };
            let a3908 = f104(v5.clone(), match v2.clone() {
                Adt2::C0(_, _, h3906, _, _, _, _, _, _, _, _) => {
                    h3906
                }
            })?;
            nat_eq(a3907, a3908)
        } else {
            false
        }
    } else {
        false
    } {
        f232(v2.clone(), 4u64, v7, false)?
    } else {
        false
    } {
        f232(v3.clone(), 4u64, v7, false)?
    } else {
        false
    } {
        f232(v1.clone(), {
            let a3909 = v6;
            let a3910 = 0u64;
            if nat_eq(a3909, a3910) {
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
            let a3919 = {
                let a3917 = {
                    let a3915 = 3u64;
                    let a3916 = {
                        let a3913 = match v4.clone() {
                            Adt6::C0(h3911, _, _, _) => {
                                h3911
                            }
                        };
                        let a3914 = match v5.clone() {
                            Adt6::C0(h3912, _, _, _) => {
                                h3912
                            }
                        };
                        nat_mul(a3913, a3914)?
                    };
                    nat_mul(a3915, a3916)?
                };
                let a3918 = 2u64;
                nat_add(a3917, a3918)?
            };
            let a3920 = f28(v3.clone(), 1u64)?;
            nat_mul(a3919, a3920)?
        }, 4u64, f30(v1, 1u64), f30(v3, 1u64), Adt4::C34(f30(v2, 1u64), v4, v5, v6)))
    } else {
        Ok(Adt10::C1)
    }
}

pub fn f123(v0: Adt2, v1: Adt2, v2: Adt2) -> R<Adt10> {
    if if if if if if if f26(v0.clone(), v1.clone()) {
        f26(v0.clone(), v2.clone())
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
            f52(v0.clone(), v2.clone())
        }
    } else {
        false
    } {
        if f35(v1.clone(), v2.clone())? {
            true
        } else {
            f52(v1.clone(), v2.clone())
        }
    } else {
        false
    } {
        Ok(Adt10::C4({
            let a3921 = 6u64;
            let a3922 = f28(v0.clone(), 1u64)?;
            nat_mul(a3921, a3922)?
        }, 2u64, f30(v0, 1u64), f30(v2, 1u64), Adt4::C33(f30(v1, 1u64))))
    } else {
        Ok(Adt10::C1)
    }
}

pub fn f124(v0: Adt1, v1: Adt2, v2: Option<Adt2>, v3: Adt2) -> R<Adt10> {
    let m3923 = v2;
    match m3923 {
        None => {
            Ok(Adt10::C1)
        }
        Some(v4) => {
            let v5: u64 = match v1.clone() {
                Adt2::C0(_, h3924, _, _, _, _, _, _, _, _, _) => {
                    h3924
                }
            };
            let v6: u64 = {
                let a3930 = match v1.clone() {
                    Adt2::C0(_, _, h3925, _, _, _, _, _, _, _, _) => {
                        h3925
                    }
                };
                let a3931 = {
                    let a3928 = match v1.clone() {
                        Adt2::C0(_, _, _, h3926, _, _, _, _, _, _, _) => {
                            h3926
                        }
                    };
                    let a3929 = match v1.clone() {
                        Adt2::C0(_, _, _, _, h3927, _, _, _, _, _, _) => {
                            h3927
                        }
                    };
                    nat_mul(a3928, a3929)?
                };
                nat_mul(a3930, a3931)?
            };
            if if if if if if if if if if f26(v1.clone(), v4.clone()) {
                f26(v1.clone(), v3.clone())
            } else {
                false
            } {
                f233(v1.clone(), v6)?
            } else {
                false
            } {
                f233(v4.clone(), v6)?
            } else {
                false
            } {
                f233(v3.clone(), v6)?
            } else {
                false
            } {
                if f35(v1.clone(), v3.clone())? {
                    true
                } else {
                    f52(v1.clone(), v3.clone())
                }
            } else {
                false
            } {
                if f35(v4.clone(), v3.clone())? {
                    true
                } else {
                    f52(v4, v3.clone())
                }
            } else {
                false
            } {
                let a3932 = v5;
                let a3933 = 2147483647u64;
                nat_le(a3932, a3933)
            } else {
                false
            } {
                let a3934 = v6;
                let a3935 = 2147483647u64;
                nat_le(a3934, a3935)
            } else {
                false
            } {
                let a3937 = match v0.clone() {
                    Adt1::C0(h3936, _, _, _, _, _, _, _, _, _, _, _) => {
                        h3936
                    }
                };
                let a3938 = 4294967295u64;
                if nat_le(a3937, a3938) {
                    let a3940 = match v0.clone() {
                        Adt1::C0(_, h3939, _, _, _, _, _, _, _, _, _, _) => {
                            h3939
                        }
                    };
                    let a3941 = 4294967295u64;
                    nat_le(a3940, a3941)
                } else {
                    false
                }
            } else {
                false
            } {
                Ok(Adt10::C4({
                    let a3942 = v5;
                    let a3943 = 0u64;
                    if nat_eq(a3942, a3943) {
                        0u64
                    } else {
                        let a3944 = v6;
                        let a3945 = f180(v5)?;
                        nat_mul(a3944, a3945)?
                    }
                }, 4u64, f30(v1, 1u64), f30(v3, 1u64), Adt4::C32(match v0 {
                    Adt1::C0(h3946, _, _, _, _, _, _, _, _, _, _, _) => {
                        h3946
                    }
                })))
            } else {
                Ok(Adt10::C1)
            }
        }
    }
}

pub fn f125(v0: Adt1, v1: Adt2, v2: Adt2) -> R<Adt10> {
    let v3: u64 = match v0.clone() {
        Adt1::C0(h3947, _, _, _, _, _, _, _, _, _, _, _) => {
            h3947
        }
    };
    let v4: u64 = match v0.clone() {
        Adt1::C0(_, h3948, _, _, _, _, _, _, _, _, _, _) => {
            h3948
        }
    };
    let v5: u64 = match v0 {
        Adt1::C0(_, _, h3949, _, _, _, _, _, _, _, _, _) => {
            h3949
        }
    };
    let v6: u64 = {
        let a3951 = match v1.clone() {
            Adt2::C0(_, h3950, _, _, _, _, _, _, _, _, _) => {
                h3950
            }
        };
        let a3952 = v4;
        let a3953 = 0u64;
        nat_quot(a3951, a3952, a3953)
    };
    let v7: u64 = {
        let a3955 = match v1.clone() {
            Adt2::C0(h3954, _, _, _, _, _, _, _, _, _, _) => {
                h3954
            }
        };
        let a3956 = 0u64;
        if nat_eq(a3955, a3956) {
            4u64
        } else {
            2u64
        }
    };
    if if if if if if if if if if if if if if if f193(match v1.clone() {
        Adt2::C0(h3957, _, _, _, _, _, _, _, _, _, _) => {
            h3957
        }
    }) {
        let a3960 = match v1.clone() {
            Adt2::C0(h3958, _, _, _, _, _, _, _, _, _, _) => {
                h3958
            }
        };
        let a3961 = match v2.clone() {
            Adt2::C0(h3959, _, _, _, _, _, _, _, _, _, _) => {
                h3959
            }
        };
        nat_eq(a3960, a3961)
    } else {
        false
    } {
        let a3962 = 0u64;
        let a3963 = v3;
        nat_lt(a3962, a3963)
    } else {
        false
    } {
        let a3964 = v3;
        let a3965 = 2147483648u64;
        nat_lt(a3964, a3965)
    } else {
        false
    } {
        let a3966 = 0u64;
        let a3967 = v4;
        nat_lt(a3966, a3967)
    } else {
        false
    } {
        let a3968 = v4;
        let a3969 = 2147483648u64;
        nat_lt(a3968, a3969)
    } else {
        false
    } {
        let a3970 = v5;
        let a3971 = 2147483648u64;
        nat_lt(a3970, a3971)
    } else {
        false
    } {
        let a3972 = 0u64;
        let a3973 = v6;
        nat_lt(a3972, a3973)
    } else {
        false
    } {
        let a3975 = 0u64;
        let a3976 = match v2.clone() {
            Adt2::C0(_, h3974, _, _, _, _, _, _, _, _, _) => {
                h3974
            }
        };
        nat_lt(a3975, a3976)
    } else {
        false
    } {
        let a3980 = match v1.clone() {
            Adt2::C0(_, h3977, _, _, _, _, _, _, _, _, _) => {
                h3977
            }
        };
        let a3981 = {
            let a3978 = v6;
            let a3979 = v4;
            nat_mul(a3978, a3979)?
        };
        nat_eq(a3980, a3981)
    } else {
        false
    } {
        let a3983 = match v2.clone() {
            Adt2::C0(_, _, h3982, _, _, _, _, _, _, _, _) => {
                h3982
            }
        };
        let a3984 = v4;
        nat_eq(a3983, a3984)
    } else {
        false
    } {
        let a3997 = {
            let a3990 = {
                let a3988 = match v2.clone() {
                    Adt2::C0(_, h3985, _, _, _, _, _, _, _, _, _) => {
                        h3985
                    }
                };
                let a3989 = {
                    let a3986 = v5;
                    let a3987 = 2u64;
                    nat_mul(a3986, a3987)?
                };
                nat_add(a3988, a3989)?
            };
            let a3991 = v3;
            nat_add(a3990, a3991)?
        };
        let a3998 = {
            let a3995 = {
                let a3993 = match v1.clone() {
                    Adt2::C0(_, _, h3992, _, _, _, _, _, _, _, _) => {
                        h3992
                    }
                };
                let a3994 = v3;
                nat_mul(a3993, a3994)?
            };
            let a3996 = v6;
            nat_add(a3995, a3996)?
        };
        nat_eq(a3997, a3998)
    } else {
        false
    } {
        f234(v1.clone(), v7)?
    } else {
        false
    } {
        f234(v2.clone(), v7)?
    } else {
        false
    } {
        f35(v1.clone(), v2.clone())?
    } else {
        false
    } {
        let v8: Adt6 = Adt6::C0(v6, v3, v5, 1u64);
        Ok(Adt10::C4({
            let a3999 = f28(v2.clone(), 1u64)?;
            let a4000 = f206(v8.clone())?;
            nat_mul(a3999, a4000)?
        }, v7, f30(v1.clone(), 1u64), f30(v2, 1u64), Adt4::C30(v8, match v1 {
            Adt2::C0(h4001, _, _, _, _, _, _, _, _, _, _) => {
                h4001
            }
        })))
    } else {
        Ok(Adt10::C1)
    }
}

pub fn f126(v0: Adt2, v1: Option<Adt2>, v2: Adt2) -> R<Adt10> {
    let m4002 = v1;
    match m4002 {
        None => {
            Ok(Adt10::C1)
        }
        Some(v3) => {
            let a4004 = match v0.clone() {
                Adt2::C0(h4003, _, _, _, _, _, _, _, _, _, _) => {
                    h4003
                }
            };
            let a4005 = 0u64;
            if if if if if if if if if if if if if if if if if if if nat_eq(a4004, a4005) {
                let a4007 = match v3.clone() {
                    Adt2::C0(h4006, _, _, _, _, _, _, _, _, _, _) => {
                        h4006
                    }
                };
                let a4008 = 0u64;
                nat_eq(a4007, a4008)
            } else {
                false
            } {
                let a4010 = match v2.clone() {
                    Adt2::C0(h4009, _, _, _, _, _, _, _, _, _, _) => {
                        h4009
                    }
                };
                let a4011 = 0u64;
                nat_eq(a4010, a4011)
            } else {
                false
            } {
                let a4013 = match v0.clone() {
                    Adt2::C0(_, _, _, _, h4012, _, _, _, _, _, _) => {
                        h4012
                    }
                };
                let a4014 = 1u64;
                nat_eq(a4013, a4014)
            } else {
                false
            } {
                let a4016 = match v3.clone() {
                    Adt2::C0(_, _, _, h4015, _, _, _, _, _, _, _) => {
                        h4015
                    }
                };
                let a4017 = 1u64;
                nat_eq(a4016, a4017)
            } else {
                false
            } {
                let a4019 = match v3.clone() {
                    Adt2::C0(_, _, _, _, h4018, _, _, _, _, _, _) => {
                        h4018
                    }
                };
                let a4020 = 1u64;
                nat_eq(a4019, a4020)
            } else {
                false
            } {
                let a4023 = match v0.clone() {
                    Adt2::C0(_, _, h4021, _, _, _, _, _, _, _, _) => {
                        h4021
                    }
                };
                let a4024 = match v3.clone() {
                    Adt2::C0(_, _, h4022, _, _, _, _, _, _, _, _) => {
                        h4022
                    }
                };
                nat_eq(a4023, a4024)
            } else {
                false
            } {
                let a4029 = match v3.clone() {
                    Adt2::C0(_, h4025, _, _, _, _, _, _, _, _, _) => {
                        h4025
                    }
                };
                let a4030 = {
                    let a4027 = match v0.clone() {
                        Adt2::C0(_, h4026, _, _, _, _, _, _, _, _, _) => {
                            h4026
                        }
                    };
                    let a4028 = 1u64;
                    nat_add(a4027, a4028)?
                };
                nat_le(a4029, a4030)
            } else {
                false
            } {
                let a4033 = match v2.clone() {
                    Adt2::C0(_, h4031, _, _, _, _, _, _, _, _, _) => {
                        h4031
                    }
                };
                let a4034 = match v0.clone() {
                    Adt2::C0(_, _, h4032, _, _, _, _, _, _, _, _) => {
                        h4032
                    }
                };
                nat_eq(a4033, a4034)
            } else {
                false
            } {
                let a4042 = match v2.clone() {
                    Adt2::C0(_, _, h4035, _, _, _, _, _, _, _, _) => {
                        h4035
                    }
                };
                let a4043 = {
                    let a4040 = {
                        let a4037 = match v0.clone() {
                            Adt2::C0(_, h4036, _, _, _, _, _, _, _, _, _) => {
                                h4036
                            }
                        };
                        let a4038 = 1u64;
                        nat_add(a4037, a4038)?
                    };
                    let a4041 = match v3.clone() {
                        Adt2::C0(_, h4039, _, _, _, _, _, _, _, _, _) => {
                            h4039
                        }
                    };
                    nat_sub(a4040, a4041)
                };
                nat_eq(a4042, a4043)
            } else {
                false
            } {
                let a4046 = match v2.clone() {
                    Adt2::C0(_, _, _, h4044, _, _, _, _, _, _, _) => {
                        h4044
                    }
                };
                let a4047 = match v0.clone() {
                    Adt2::C0(_, _, _, h4045, _, _, _, _, _, _, _) => {
                        h4045
                    }
                };
                nat_eq(a4046, a4047)
            } else {
                false
            } {
                let a4049 = match v2.clone() {
                    Adt2::C0(_, _, _, _, h4048, _, _, _, _, _, _) => {
                        h4048
                    }
                };
                let a4050 = 1u64;
                nat_eq(a4049, a4050)
            } else {
                false
            } {
                f43(v2.clone())?
            } else {
                false
            } {
                let a4052 = match v0.clone() {
                    Adt2::C0(_, _, _, _, _, h4051, _, _, _, _, _) => {
                        h4051
                    }
                };
                let a4053 = 4u64;
                nat_eq(a4052, a4053)
            } else {
                false
            } {
                let a4055 = match v3.clone() {
                    Adt2::C0(_, _, _, _, _, h4054, _, _, _, _, _) => {
                        h4054
                    }
                };
                let a4056 = 4u64;
                nat_eq(a4055, a4056)
            } else {
                false
            } {
                let a4061 = match v0.clone() {
                    Adt2::C0(_, _, _, _, _, _, h4057, _, _, _, _) => {
                        h4057
                    }
                };
                let a4062 = {
                    let a4059 = match v0.clone() {
                        Adt2::C0(_, h4058, _, _, _, _, _, _, _, _, _) => {
                            h4058
                        }
                    };
                    let a4060 = 4u64;
                    nat_mul(a4059, a4060)?
                };
                nat_eq(a4061, a4062)
            } else {
                false
            } {
                let a4067 = match v3.clone() {
                    Adt2::C0(_, _, _, _, _, _, h4063, _, _, _, _) => {
                        h4063
                    }
                };
                let a4068 = {
                    let a4065 = match v3.clone() {
                        Adt2::C0(_, h4064, _, _, _, _, _, _, _, _, _) => {
                            h4064
                        }
                    };
                    let a4066 = 4u64;
                    nat_mul(a4065, a4066)?
                };
                nat_eq(a4067, a4068)
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
                    let a4070 = f28(v2.clone(), 1u64)?;
                    let a4071 = f205(match v3.clone() {
                        Adt2::C0(_, h4069, _, _, _, _, _, _, _, _, _) => {
                            h4069
                        }
                    })?;
                    nat_mul(a4070, a4071)?
                }, 4u64, f30(v0, 1u64), f30(v2, 1u64), Adt4::C29(f30(v3, 1u64))))
            } else {
                Ok(Adt10::C1)
            }
        }
    }
}

pub fn f127(v0: bool, v1: bool, v2: Adt1, v3: Adt2, v4: Adt2) -> R<Adt10> {
    let v5: Adt6 = Adt6::C0(match v2.clone() {
        Adt1::C0(_, h4072, _, _, _, _, _, _, _, _, _, _) => {
            h4072
        }
    }, if v1 {
        match v2.clone() {
            Adt1::C0(_, _, _, h4073, _, _, _, _, _, _, _, _) => {
                h4073
            }
        }
    } else {
        match v2.clone() {
            Adt1::C0(_, _, h4074, _, _, _, _, _, _, _, _, _) => {
                h4074
            }
        }
    }, if v1 {
        match v2.clone() {
            Adt1::C0(_, _, _, _, _, h4075, _, _, _, _, _, _) => {
                h4075
            }
        }
    } else {
        match v2.clone() {
            Adt1::C0(_, _, _, h4076, _, _, _, _, _, _, _, _) => {
                h4076
            }
        }
    }, 1u64);
    let v6: Adt6 = if v1 {
        Adt6::C0(match v2.clone() {
            Adt1::C0(_, _, h4077, _, _, _, _, _, _, _, _, _) => {
                h4077
            }
        }, match v2.clone() {
            Adt1::C0(_, _, _, _, h4078, _, _, _, _, _, _, _) => {
                h4078
            }
        }, match v2.clone() {
            Adt1::C0(_, _, _, _, _, _, h4079, _, _, _, _, _) => {
                h4079
            }
        }, 1u64)
    } else {
        Adt6::C0(1u64, 1u64, 0u64, 1u64)
    };
    let a4081 = match v2.clone() {
        Adt1::C0(h4080, _, _, _, _, _, _, _, _, _, _, _) => {
            h4080
        }
    };
    let a4082 = 1u64;
    if if if if if if if if if if if nat_le(a4081, a4082) {
        let a4084 = match v3.clone() {
            Adt2::C0(h4083, _, _, _, _, _, _, _, _, _, _) => {
                h4083
            }
        };
        let a4085 = 1u64;
        nat_le(a4084, a4085)
    } else {
        false
    } {
        let a4087 = match v4.clone() {
            Adt2::C0(h4086, _, _, _, _, _, _, _, _, _, _) => {
                h4086
            }
        };
        let a4088 = 0u64;
        nat_eq(a4087, a4088)
    } else {
        false
    } {
        let a4093 = match v3.clone() {
            Adt2::C0(_, _, _, _, _, h4089, _, _, _, _, _) => {
                h4089
            }
        };
        let a4094 = {
            let a4091 = match v3.clone() {
                Adt2::C0(h4090, _, _, _, _, _, _, _, _, _, _) => {
                    h4090
                }
            };
            let a4092 = 0u64;
            if nat_eq(a4091, a4092) {
                4u64
            } else {
                2u64
            }
        };
        nat_eq(a4093, a4094)
    } else {
        false
    } {
        f43(v4.clone())?
    } else {
        false
    } {
        f35(v3.clone(), v4.clone())?
    } else {
        false
    } {
        f235(v5.clone(), match v3.clone() {
            Adt2::C0(_, h4095, _, _, _, _, _, _, _, _, _) => {
                h4095
            }
        }, match v4.clone() {
            Adt2::C0(_, h4096, _, _, _, _, _, _, _, _, _) => {
                h4096
            }
        })?
    } else {
        false
    } {
        if v1 {
            f235(v6.clone(), match v3.clone() {
                Adt2::C0(_, _, h4097, _, _, _, _, _, _, _, _) => {
                    h4097
                }
            }, match v4.clone() {
                Adt2::C0(_, _, h4098, _, _, _, _, _, _, _, _) => {
                    h4098
                }
            })?
        } else {
            let a4101 = match v3.clone() {
                Adt2::C0(_, _, h4099, _, _, _, _, _, _, _, _) => {
                    h4099
                }
            };
            let a4102 = match v4.clone() {
                Adt2::C0(_, _, h4100, _, _, _, _, _, _, _, _) => {
                    h4100
                }
            };
            nat_eq(a4101, a4102)
        }
    } else {
        false
    } {
        let a4105 = match v3.clone() {
            Adt2::C0(_, _, _, h4103, _, _, _, _, _, _, _) => {
                h4103
            }
        };
        let a4106 = match v4.clone() {
            Adt2::C0(_, _, _, h4104, _, _, _, _, _, _, _) => {
                h4104
            }
        };
        nat_eq(a4105, a4106)
    } else {
        false
    } {
        let a4109 = match v3.clone() {
            Adt2::C0(_, _, _, _, h4107, _, _, _, _, _, _) => {
                h4107
            }
        };
        let a4110 = match v4.clone() {
            Adt2::C0(_, _, _, _, h4108, _, _, _, _, _, _) => {
                h4108
            }
        };
        nat_eq(a4109, a4110)
    } else {
        false
    } {
        let a4115 = {
            let a4113 = match v5.clone() {
                Adt6::C0(h4111, _, _, _) => {
                    h4111
                }
            };
            let a4114 = match v6.clone() {
                Adt6::C0(h4112, _, _, _) => {
                    h4112
                }
            };
            nat_mul(a4113, a4114)?
        };
        let a4116 = 2147483648u64;
        nat_lt(a4115, a4116)
    } else {
        false
    } {
        let v7: Adt7 = Adt7::C0(v5, v6, v1, {
            let a4118 = match v2 {
                Adt1::C0(h4117, _, _, _, _, _, _, _, _, _, _, _) => {
                    h4117
                }
            };
            let a4119 = 0u64;
            nat_eq(a4118, a4119)
        }, match v3.clone() {
            Adt2::C0(h4120, _, _, _, _, _, _, _, _, _, _) => {
                h4120
            }
        });
        if v0 {
            Ok(Adt10::C4({
                let a4132 = f28(v3.clone(), 1u64)?;
                let a4133 = {
                    let a4130 = f28(v4.clone(), 1u64)?;
                    let a4131 = f184(match v7.clone() {
                        Adt7::C0(_, _, _, h4121, _) => {
                            h4121
                        }
                    }, {
                        let a4128 = {
                            let m4124 = match v7.clone() {
                                Adt7::C0(h4123, _, _, _, _) => {
                                    h4123
                                }
                            };
                            match m4124 {
                                Adt6::C0(h4122, _, _, _) => {
                                    h4122
                                }
                            }
                        };
                        let a4129 = {
                            let m4127 = match v7.clone() {
                                Adt7::C0(_, h4126, _, _, _) => {
                                    h4126
                                }
                            };
                            match m4127 {
                                Adt6::C0(h4125, _, _, _) => {
                                    h4125
                                }
                            }
                        };
                        nat_mul(a4128, a4129)?
                    })?;
                    nat_mul(a4130, a4131)?
                };
                nat_add(a4132, a4133)?
            }, 4u64, f30(v4, 1u64), f30(v3.clone(), 1u64), Adt4::C43(v7, f28(v3, 1u64)?)))
        } else {
            Ok(Adt10::C4({
                let a4134 = f28(v4.clone(), 1u64)?;
                let a4135 = f202(v7.clone())?;
                nat_mul(a4134, a4135)?
            }, 4u64, f30(v3, 1u64), f30(v4, 1u64), Adt4::C28(v7)))
        }
    } else {
        Ok(Adt10::C1)
    }
}

pub fn f128(v0: bool, v1: Adt1, v2: Adt2, v3: Option<Adt2>, v4: Adt2) -> R<Adt10> {
    let m4136 = v3;
    match m4136 {
        None => {
            Ok(Adt10::C1)
        }
        Some(v5) => {
            let v6: bool = {
                let a4138 = match v1.clone() {
                    Adt1::C0(_, _, _, _, _, _, h4137, _, _, _, _, _) => {
                        h4137
                    }
                };
                let a4139 = 1u64;
                nat_eq(a4138, a4139)
            };
            let v7: Adt6 = Adt6::C0(match v2.clone() {
                Adt2::C0(_, h4140, _, _, _, _, _, _, _, _, _) => {
                    h4140
                }
            }, match v1.clone() {
                Adt1::C0(h4141, _, _, _, _, _, _, _, _, _, _, _) => {
                    h4141
                }
            }, match v1.clone() {
                Adt1::C0(_, _, h4142, _, _, _, _, _, _, _, _, _) => {
                    h4142
                }
            }, match v1.clone() {
                Adt1::C0(_, _, _, _, h4143, _, _, _, _, _, _, _) => {
                    h4143
                }
            });
            let v8: Adt6 = if v6 {
                Adt6::C0(match v2.clone() {
                    Adt2::C0(_, _, h4144, _, _, _, _, _, _, _, _) => {
                        h4144
                    }
                }, match v1.clone() {
                    Adt1::C0(_, h4145, _, _, _, _, _, _, _, _, _, _) => {
                        h4145
                    }
                }, match v1.clone() {
                    Adt1::C0(_, _, _, h4146, _, _, _, _, _, _, _, _) => {
                        h4146
                    }
                }, match v1.clone() {
                    Adt1::C0(_, _, _, _, _, h4147, _, _, _, _, _, _) => {
                        h4147
                    }
                })
            } else {
                Adt6::C0(1u64, 1u64, 0u64, 1u64)
            };
            let v9: Adt6 = Adt6::C0(1u64, 1u64, 0u64, 1u64);
            let v10: u64 = if v6 {
                match v5.clone() {
                    Adt2::C0(_, _, _, h4148, _, _, _, _, _, _, _) => {
                        h4148
                    }
                }
            } else {
                match v5.clone() {
                    Adt2::C0(_, _, h4149, _, _, _, _, _, _, _, _) => {
                        h4149
                    }
                }
            };
            let v11: u64 = if v6 {
                match v5.clone() {
                    Adt2::C0(_, _, _, _, h4150, _, _, _, _, _, _) => {
                        h4150
                    }
                }
            } else {
                match v5.clone() {
                    Adt2::C0(_, _, _, h4151, _, _, _, _, _, _, _) => {
                        h4151
                    }
                }
            };
            let v12: u64 = if v6 {
                match v5.clone() {
                    Adt2::C0(_, _, h4152, _, _, _, _, _, _, _, _) => {
                        h4152
                    }
                }
            } else {
                1u64
            };
            let v13: u64 = match v5.clone() {
                Adt2::C0(h4153, _, _, _, _, _, _, _, _, _, _) => {
                    h4153
                }
            };
            let v14: u64 = match v4.clone() {
                Adt2::C0(h4154, _, _, _, _, _, _, _, _, _, _) => {
                    h4154
                }
            };
            let v15: u64 = {
                let a4155 = v13;
                let a4156 = 0u64;
                if nat_eq(a4155, a4156) {
                    4u64
                } else {
                    2u64
                }
            };
            let v16: u64 = {
                let a4158 = match v5.clone() {
                    Adt2::C0(_, h4157, _, _, _, _, _, _, _, _, _) => {
                        h4157
                    }
                };
                let a4159 = v15;
                nat_mul(a4158, a4159)?
            };
            let v17: u64 = if v6 {
                match v5.clone() {
                    Adt2::C0(_, _, _, _, _, _, _, h4160, _, _, _) => {
                        h4160
                    }
                }
            } else {
                match v5.clone() {
                    Adt2::C0(_, _, _, _, _, _, h4161, _, _, _, _) => {
                        h4161
                    }
                }
            };
            let v18: u64 = if v6 {
                match v5.clone() {
                    Adt2::C0(_, _, _, _, _, _, _, _, h4162, _, _) => {
                        h4162
                    }
                }
            } else {
                match v5.clone() {
                    Adt2::C0(_, _, _, _, _, _, _, h4163, _, _, _) => {
                        h4163
                    }
                }
            };
            let a4165 = match v1 {
                Adt1::C0(_, _, _, _, _, _, h4164, _, _, _, _, _) => {
                    h4164
                }
            };
            let a4166 = 1u64;
            if if if if if if if if if if nat_le(a4165, a4166) {
                let a4167 = v13;
                let a4168 = 0u64;
                if nat_eq(a4167, a4168) {
                    true
                } else {
                    let a4169 = v13;
                    let a4170 = 1u64;
                    if nat_eq(a4169, a4170) {
                        let a4171 = v14;
                        let a4172 = 1u64;
                        nat_eq(a4171, a4172)
                    } else {
                        false
                    }
                }
            } else {
                false
            } {
                let a4173 = v14;
                let a4174 = 0u64;
                if nat_eq(a4173, a4174) {
                    true
                } else {
                    let a4175 = v14;
                    let a4176 = 1u64;
                    nat_eq(a4175, a4176)
                }
            } else {
                false
            } {
                let a4178 = match v5.clone() {
                    Adt2::C0(_, _, _, _, _, h4177, _, _, _, _, _) => {
                        h4177
                    }
                };
                let a4179 = v15;
                nat_eq(a4178, a4179)
            } else {
                false
            } {
                let a4187 = match v4.clone() {
                    Adt2::C0(_, h4180, _, _, _, _, _, _, _, _, _) => {
                        h4180
                    }
                };
                let a4188 = {
                    let a4185 = v10;
                    let a4186 = {
                        let a4183 = match v7.clone() {
                            Adt6::C0(h4181, _, _, _) => {
                                h4181
                            }
                        };
                        let a4184 = match v8.clone() {
                            Adt6::C0(h4182, _, _, _) => {
                                h4182
                            }
                        };
                        nat_mul(a4183, a4184)?
                    };
                    nat_mul(a4185, a4186)?
                };
                nat_eq(a4187, a4188)
            } else {
                false
            } {
                let a4191 = match v4.clone() {
                    Adt2::C0(_, _, h4189, _, _, _, _, _, _, _, _) => {
                        h4189
                    }
                };
                let a4192 = f104(v7.clone(), match v5.clone() {
                    Adt2::C0(_, h4190, _, _, _, _, _, _, _, _, _) => {
                        h4190
                    }
                })?;
                nat_eq(a4191, a4192)
            } else {
                false
            } {
                let a4194 = 0u64;
                let a4195 = match v4.clone() {
                    Adt2::C0(_, _, h4193, _, _, _, _, _, _, _, _) => {
                        h4193
                    }
                };
                nat_lt(a4194, a4195)
            } else {
                false
            } {
                if v6 {
                    let a4197 = match v2 {
                        Adt2::C0(_, _, _, h4196, _, _, _, _, _, _, _) => {
                            h4196
                        }
                    };
                    let a4198 = v10;
                    if if if nat_eq(a4197, a4198) {
                        let a4200 = match v4.clone() {
                            Adt2::C0(_, _, _, h4199, _, _, _, _, _, _, _) => {
                                h4199
                            }
                        };
                        let a4201 = f104(v8.clone(), v12)?;
                        nat_eq(a4200, a4201)
                    } else {
                        false
                    } {
                        let a4203 = match v4.clone() {
                            Adt2::C0(_, _, _, _, h4202, _, _, _, _, _, _) => {
                                h4202
                            }
                        };
                        let a4204 = v11;
                        nat_eq(a4203, a4204)
                    } else {
                        false
                    } {
                        let a4206 = 0u64;
                        let a4207 = match v4.clone() {
                            Adt2::C0(_, _, _, h4205, _, _, _, _, _, _, _) => {
                                h4205
                            }
                        };
                        nat_lt(a4206, a4207)
                    } else {
                        false
                    }
                } else {
                    let a4209 = match v2 {
                        Adt2::C0(_, _, h4208, _, _, _, _, _, _, _, _) => {
                            h4208
                        }
                    };
                    let a4210 = v10;
                    if if if nat_eq(a4209, a4210) {
                        let a4212 = match v5.clone() {
                            Adt2::C0(_, _, _, _, h4211, _, _, _, _, _, _) => {
                                h4211
                            }
                        };
                        let a4213 = 1u64;
                        nat_eq(a4212, a4213)
                    } else {
                        false
                    } {
                        let a4215 = match v4.clone() {
                            Adt2::C0(_, _, _, h4214, _, _, _, _, _, _, _) => {
                                h4214
                            }
                        };
                        let a4216 = v11;
                        nat_eq(a4215, a4216)
                    } else {
                        false
                    } {
                        let a4218 = match v4.clone() {
                            Adt2::C0(_, _, _, _, h4217, _, _, _, _, _, _) => {
                                h4217
                            }
                        };
                        let a4219 = 1u64;
                        nat_eq(a4218, a4219)
                    } else {
                        false
                    }
                }
            } else {
                false
            } {
                if f53(match v5.clone() {
                    Adt2::C0(_, h4220, _, _, _, _, _, _, _, _, _) => {
                        h4220
                    }
                }, v12, v10, v11) {
                    true
                } else {
                    let a4240 = {
                        let a4237 = {
                            let a4235 = {
                                let a4229 = {
                                    let a4223 = {
                                        let a4221 = v11;
                                        let a4222 = 1u64;
                                        nat_sub(a4221, a4222)
                                    };
                                    let a4224 = v18;
                                    nat_mul(a4223, a4224)?
                                };
                                let a4230 = {
                                    let a4227 = {
                                        let a4225 = v10;
                                        let a4226 = 1u64;
                                        nat_sub(a4225, a4226)
                                    };
                                    let a4228 = v17;
                                    nat_mul(a4227, a4228)?
                                };
                                nat_add(a4229, a4230)?
                            };
                            let a4236 = {
                                let a4233 = {
                                    let a4231 = v12;
                                    let a4232 = 1u64;
                                    nat_sub(a4231, a4232)
                                };
                                let a4234 = v16;
                                nat_mul(a4233, a4234)?
                            };
                            nat_add(a4235, a4236)?
                        };
                        let a4238 = v16;
                        nat_add(a4237, a4238)?
                    };
                    let a4241 = match v5.clone() {
                        Adt2::C0(_, _, _, _, _, _, _, _, _, h4239, _) => {
                            h4239
                        }
                    };
                    nat_le(a4240, a4241)
                }
            } else {
                false
            } {
                f35(v5.clone(), v4.clone())?
            } else {
                false
            } {
                let v19: Adt3 = if v6 {
                    f30(v4.clone(), 1u64)
                } else {
                    Adt3::C0(match v4.clone() {
                        Adt2::C0(_, h4242, _, _, _, _, _, _, _, _, _) => {
                            h4242
                        }
                    }, match v4.clone() {
                        Adt2::C0(_, _, h4243, _, _, _, _, _, _, _, _) => {
                            h4243
                        }
                    }, 1u64, match v4.clone() {
                        Adt2::C0(_, _, _, h4244, _, _, _, _, _, _, _) => {
                            h4244
                        }
                    }, match v4.clone() {
                        Adt2::C0(_, _, _, _, _, h4245, _, _, _, _, _) => {
                            h4245
                        }
                    }, match v4.clone() {
                        Adt2::C0(_, _, _, _, _, _, h4246, _, _, _, _) => {
                            h4246
                        }
                    }, 0u64, match v4.clone() {
                        Adt2::C0(_, _, _, _, _, _, _, h4247, _, _, _) => {
                            h4247
                        }
                    })
                };
                if v0 {
                    Ok(Adt10::C4({
                        let a4254 = f28(v5.clone(), 1u64)?;
                        let a4255 = {
                            let a4252 = {
                                let a4250 = match v7.clone() {
                                    Adt6::C0(h4248, _, _, _) => {
                                        h4248
                                    }
                                };
                                let a4251 = match v8.clone() {
                                    Adt6::C0(h4249, _, _, _) => {
                                        h4249
                                    }
                                };
                                nat_mul(a4250, a4251)?
                            };
                            let a4253 = 2u64;
                            nat_add(a4252, a4253)?
                        };
                        nat_mul(a4254, a4255)?
                    }, 4u64, v19, f42(v6, v5), Adt4::C42(v7, v8)))
                } else {
                    Ok(Adt10::C4(f28(v4, 1u64)?, {
                        let a4256 = v14;
                        let a4257 = 0u64;
                        if nat_eq(a4256, a4257) {
                            4u64
                        } else {
                            2u64
                        }
                    }, Adt3::C0(match v5 {
                        Adt2::C0(_, h4258, _, _, _, _, _, _, _, _, _) => {
                            h4258
                        }
                    }, v12, 1u64, v11, v15, v16, 0u64, 0u64), v19, Adt4::C27(Adt5::C0(v7, v8, v9, v17, v18, v13, 1u64, v14))))
                }
            } else {
                Ok(Adt10::C1)
            }
        }
    }
}

pub fn f129(v0: Adt1, v1: Adt2, v2: Option<Adt2>, v3: Adt2) -> R<Adt10> {
    let m4259 = v2;
    match m4259 {
        None => {
            Ok(Adt10::C1)
        }
        Some(v4) => {
            let v5: Adt6 = Adt6::C0(match v1.clone() {
                Adt2::C0(_, h4260, _, _, _, _, _, _, _, _, _) => {
                    h4260
                }
            }, match v0.clone() {
                Adt1::C0(h4261, _, _, _, _, _, _, _, _, _, _, _) => {
                    h4261
                }
            }, match v0.clone() {
                Adt1::C0(_, _, _, h4262, _, _, _, _, _, _, _, _) => {
                    h4262
                }
            }, match v0.clone() {
                Adt1::C0(_, _, _, _, _, _, h4263, _, _, _, _, _) => {
                    h4263
                }
            });
            let v6: Adt6 = Adt6::C0(match v1.clone() {
                Adt2::C0(_, _, h4264, _, _, _, _, _, _, _, _) => {
                    h4264
                }
            }, match v0.clone() {
                Adt1::C0(_, h4265, _, _, _, _, _, _, _, _, _, _) => {
                    h4265
                }
            }, match v0.clone() {
                Adt1::C0(_, _, _, _, h4266, _, _, _, _, _, _, _) => {
                    h4266
                }
            }, match v0.clone() {
                Adt1::C0(_, _, _, _, _, _, _, h4267, _, _, _, _) => {
                    h4267
                }
            });
            let v7: Adt6 = Adt6::C0(match v1 {
                Adt2::C0(_, _, _, h4268, _, _, _, _, _, _, _) => {
                    h4268
                }
            }, match v0.clone() {
                Adt1::C0(_, _, h4269, _, _, _, _, _, _, _, _, _) => {
                    h4269
                }
            }, match v0.clone() {
                Adt1::C0(_, _, _, _, _, h4270, _, _, _, _, _, _) => {
                    h4270
                }
            }, match v0.clone() {
                Adt1::C0(_, _, _, _, _, _, _, _, h4271, _, _, _) => {
                    h4271
                }
            });
            let v8: u64 = match v0 {
                Adt1::C0(_, _, _, _, _, _, _, _, _, h4272, _, _) => {
                    h4272
                }
            };
            let v9: u64 = f104(v7.clone(), match v4.clone() {
                Adt2::C0(_, _, _, h4273, _, _, _, _, _, _, _) => {
                    h4273
                }
            })?;
            let v10: u64 = match v3.clone() {
                Adt2::C0(h4274, _, _, _, _, _, _, _, _, _, _) => {
                    h4274
                }
            };
            let v11: u64 = {
                let a4282 = v8;
                let a4283 = {
                    let a4280 = match v5.clone() {
                        Adt6::C0(h4275, _, _, _) => {
                            h4275
                        }
                    };
                    let a4281 = {
                        let a4278 = match v6.clone() {
                            Adt6::C0(h4276, _, _, _) => {
                                h4276
                            }
                        };
                        let a4279 = match v7.clone() {
                            Adt6::C0(h4277, _, _, _) => {
                                h4277
                            }
                        };
                        nat_mul(a4278, a4279)?
                    };
                    nat_mul(a4280, a4281)?
                };
                nat_mul(a4282, a4283)?
            };
            let a4285 = match v4.clone() {
                Adt2::C0(h4284, _, _, _, _, _, _, _, _, _, _) => {
                    h4284
                }
            };
            let a4286 = 0u64;
            if if if if if if if if if if if if if nat_eq(a4285, a4286) {
                let a4287 = v10;
                let a4288 = 0u64;
                if nat_eq(a4287, a4288) {
                    true
                } else {
                    let a4289 = v10;
                    let a4290 = 1u64;
                    nat_eq(a4289, a4290)
                }
            } else {
                false
            } {
                let a4291 = 0u64;
                let a4292 = v8;
                nat_lt(a4291, a4292)
            } else {
                false
            } {
                let a4293 = v8;
                let a4294 = 2147483648u64;
                nat_lt(a4293, a4294)
            } else {
                false
            } {
                let a4296 = match v4.clone() {
                    Adt2::C0(_, _, _, _, _, h4295, _, _, _, _, _) => {
                        h4295
                    }
                };
                let a4297 = 4u64;
                nat_eq(a4296, a4297)
            } else {
                false
            } {
                let a4298 = 0u64;
                let a4299 = v9;
                nat_lt(a4298, a4299)
            } else {
                false
            } {
                let a4301 = 0u64;
                let a4302 = match v3.clone() {
                    Adt2::C0(_, _, h4300, _, _, _, _, _, _, _, _) => {
                        h4300
                    }
                };
                nat_lt(a4301, a4302)
            } else {
                false
            } {
                let a4304 = 0u64;
                let a4305 = match v3.clone() {
                    Adt2::C0(_, _, _, h4303, _, _, _, _, _, _, _) => {
                        h4303
                    }
                };
                nat_lt(a4304, a4305)
            } else {
                false
            } {
                let a4307 = match v3.clone() {
                    Adt2::C0(_, h4306, _, _, _, _, _, _, _, _, _) => {
                        h4306
                    }
                };
                let a4308 = v11;
                nat_eq(a4307, a4308)
            } else {
                false
            } {
                let a4311 = match v3.clone() {
                    Adt2::C0(_, _, h4309, _, _, _, _, _, _, _, _) => {
                        h4309
                    }
                };
                let a4312 = f104(v5.clone(), match v4.clone() {
                    Adt2::C0(_, h4310, _, _, _, _, _, _, _, _, _) => {
                        h4310
                    }
                })?;
                nat_eq(a4311, a4312)
            } else {
                false
            } {
                let a4315 = match v3.clone() {
                    Adt2::C0(_, _, _, h4313, _, _, _, _, _, _, _) => {
                        h4313
                    }
                };
                let a4316 = f104(v6.clone(), match v4.clone() {
                    Adt2::C0(_, _, h4314, _, _, _, _, _, _, _, _) => {
                        h4314
                    }
                })?;
                nat_eq(a4315, a4316)
            } else {
                false
            } {
                let a4324 = match v3.clone() {
                    Adt2::C0(_, _, _, _, h4317, _, _, _, _, _, _) => {
                        h4317
                    }
                };
                let a4325 = {
                    let a4322 = v9;
                    let a4323 = {
                        let a4319 = match v4.clone() {
                            Adt2::C0(_, _, _, _, h4318, _, _, _, _, _, _) => {
                                h4318
                            }
                        };
                        let a4320 = v8;
                        let a4321 = 0u64;
                        nat_quot(a4319, a4320, a4321)
                    };
                    nat_mul(a4322, a4323)?
                };
                nat_eq(a4324, a4325)
            } else {
                false
            } {
                f35(v4.clone(), v3.clone())?
            } else {
                false
            } {
                Ok(Adt10::C4(f28(v3.clone(), 1u64)?, {
                    let a4326 = v10;
                    let a4327 = 0u64;
                    if nat_eq(a4326, a4327) {
                        4u64
                    } else {
                        2u64
                    }
                }, f30(v4.clone(), 1u64), f30(v3, 1u64), Adt4::C27(Adt5::C0(v5, v6, v7, match v4.clone() {
                    Adt2::C0(_, _, _, _, _, _, _, _, h4328, _, _) => {
                        h4328
                    }
                }, {
                    let a4330 = v8;
                    let a4331 = match v4 {
                        Adt2::C0(_, _, _, _, _, _, _, _, h4329, _, _) => {
                            h4329
                        }
                    };
                    nat_mul(a4330, a4331)?
                }, 0u64, v9, v10))))
            } else {
                Ok(Adt10::C1)
            }
        }
    }
}

pub fn f130(v0: Adt2, v1: Option<Adt2>, v2: Adt2) -> R<Adt10> {
    let m4332 = v1;
    match m4332 {
        None => {
            Ok(Adt10::C1)
        }
        Some(v3) => {
            f236(v0, v3, v2)
        }
    }
}

pub fn f131(v0: Adt2, v1: Adt2) -> R<Adt10> {
    let a4334 = match v0.clone() {
        Adt2::C0(h4333, _, _, _, _, _, _, _, _, _, _) => {
            h4333
        }
    };
    let a4335 = 1u64;
    if if nat_eq(a4334, a4335) {
        true
    } else {
        let a4337 = match v0.clone() {
            Adt2::C0(h4336, _, _, _, _, _, _, _, _, _, _) => {
                h4336
            }
        };
        let a4338 = 30u64;
        nat_eq(a4337, a4338)
    } {
        let a4340 = match v1.clone() {
            Adt2::C0(h4339, _, _, _, _, _, _, _, _, _, _) => {
                h4339
            }
        };
        let a4341 = 1u64;
        if if if if if if if if if if if nat_eq(a4340, a4341) {
            let a4344 = match v0.clone() {
                Adt2::C0(_, h4342, _, _, _, _, _, _, _, _, _) => {
                    h4342
                }
            };
            let a4345 = match v1.clone() {
                Adt2::C0(_, h4343, _, _, _, _, _, _, _, _, _) => {
                    h4343
                }
            };
            nat_eq(a4344, a4345)
        } else {
            false
        } {
            let a4347 = 0u64;
            let a4348 = match v1.clone() {
                Adt2::C0(_, _, h4346, _, _, _, _, _, _, _, _) => {
                    h4346
                }
            };
            nat_lt(a4347, a4348)
        } else {
            false
        } {
            let a4351 = match v1.clone() {
                Adt2::C0(_, _, h4349, _, _, _, _, _, _, _, _) => {
                    h4349
                }
            };
            let a4352 = match v1.clone() {
                Adt2::C0(_, _, _, h4350, _, _, _, _, _, _, _) => {
                    h4350
                }
            };
            nat_eq(a4351, a4352)
        } else {
            false
        } {
            let a4354 = match v1.clone() {
                Adt2::C0(_, _, _, _, h4353, _, _, _, _, _, _) => {
                    h4353
                }
            };
            let a4355 = 1u64;
            nat_eq(a4354, a4355)
        } else {
            false
        } {
            let a4363 = match v0.clone() {
                Adt2::C0(_, _, h4356, _, _, _, _, _, _, _, _) => {
                    h4356
                }
            };
            let a4364 = {
                let a4361 = {
                    let a4359 = match v1.clone() {
                        Adt2::C0(_, _, h4357, _, _, _, _, _, _, _, _) => {
                            h4357
                        }
                    };
                    let a4360 = match v1.clone() {
                        Adt2::C0(_, _, h4358, _, _, _, _, _, _, _, _) => {
                            h4358
                        }
                    };
                    nat_add(a4359, a4360)?
                };
                let a4362 = 1u64;
                nat_sub(a4361, a4362)
            };
            nat_eq(a4363, a4364)
        } else {
            false
        } {
            let a4366 = 0u64;
            let a4367 = match v0.clone() {
                Adt2::C0(_, _, _, h4365, _, _, _, _, _, _, _) => {
                    h4365
                }
            };
            nat_lt(a4366, a4367)
        } else {
            false
        } {
            let a4369 = 0u64;
            let a4370 = match v0.clone() {
                Adt2::C0(_, _, _, _, h4368, _, _, _, _, _, _) => {
                    h4368
                }
            };
            nat_lt(a4369, a4370)
        } else {
            false
        } {
            let a4372 = 2u64;
            let a4373 = match v0.clone() {
                Adt2::C0(_, _, _, _, _, h4371, _, _, _, _, _) => {
                    h4371
                }
            };
            nat_le(a4372, a4373)
        } else {
            false
        } {
            let a4375 = 2u64;
            let a4376 = match v1.clone() {
                Adt2::C0(_, _, _, _, _, h4374, _, _, _, _, _) => {
                    h4374
                }
            };
            nat_le(a4375, a4376)
        } else {
            false
        } {
            f35(v0.clone(), v1.clone())?
        } else {
            false
        } {
            Ok(Adt10::C4(f28(v1.clone(), 1u64)?, 2u64, f30(v0, 1u64), f30(v1, 1u64), Adt4::C21))
        } else {
            Ok(Adt10::C1)
        }
    } else {
        Ok(Adt10::C0)
    }
}

pub fn f132(v0: Adt2, v1: Option<Adt2>, v2: Adt2) -> R<Adt10> {
    let m4377 = v1;
    match m4377 {
        None => {
            Ok(Adt10::C1)
        }
        Some(v3) => {
            f237(v0, v3, v2)
        }
    }
}

pub fn f133(v0: u64) -> bool {
    let a4378 = v0;
    let a4379 = 1u64;
    if if nat_eq(a4378, a4379) {
        true
    } else {
        let a4380 = v0;
        let a4381 = 33u64;
        nat_eq(a4380, a4381)
    } {
        true
    } else {
        let a4382 = v0;
        let a4383 = 34u64;
        nat_eq(a4382, a4383)
    }
}

pub fn f134(v0: Adt2, v1: Adt2, v2: Adt2) -> R<Adt10> {
    let v3: u64 = f28(v0.clone(), 1u64)?;
    if if if if if f52(v1, v2.clone()) {
        let a4384 = v3;
        let a4385 = f28(v2.clone(), 1u64)?;
        nat_eq(a4384, a4385)
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
        let a4386 = v3;
        let a4387 = 0u64;
        if nat_eq(a4386, a4387) {
            true
        } else {
            let a4389 = 4u64;
            let a4390 = match v2.clone() {
                Adt2::C0(_, _, _, _, _, h4388, _, _, _, _, _) => {
                    h4388
                }
            };
            nat_le(a4389, a4390)
        }
    } else {
        false
    } {
        Ok(Adt10::C4({
            let a4391 = 2u64;
            let a4392 = v3;
            nat_mul(a4391, a4392)?
        }, 4u64, f30(v0, 1u64), f30(v2, 1u64), Adt4::C35(v3)))
    } else {
        Ok(Adt10::C1)
    }
}

pub fn f135(v0: u64) -> bool {
    if f238(v0) {
        !f239(v0)
    } else {
        false
    }
}

pub fn f136(v0: Adt2, v1: Option<Adt2>, v2: Adt2) -> R<Adt10> {
    let m4393 = v1;
    match m4393 {
        None => {
            Ok(Adt10::C1)
        }
        Some(v3) => {
            let m4395 = f230(match v0.clone() {
                Adt2::C0(h4394, _, _, _, _, _, _, _, _, _, _) => {
                    h4394
                }
            });
            match m4395 {
                Adt18::C0 => {
                    Ok(Adt10::C0)
                }
                Adt18::C1(v4) => {
                    let v5: u64 = match v4.clone() {
                        Adt19::C0(_, h4396, _) => {
                            h4396
                        }
                    };
                    if if if if if if f52(v3, v2.clone()) {
                        f35(v0.clone(), v2.clone())?
                    } else {
                        false
                    } {
                        let a4397 = f28(v0.clone(), 1u64)?;
                        let a4398 = f28(v2.clone(), 1u64)?;
                        nat_eq(a4397, a4398)
                    } else {
                        false
                    } {
                        let a4403 = {
                            let a4400 = match v0.clone() {
                                Adt2::C0(_, h4399, _, _, _, _, _, _, _, _, _) => {
                                    h4399
                                }
                            };
                            let a4401 = v5;
                            let a4402 = 1u64;
                            nat_rem(a4400, a4401, a4402)
                        };
                        let a4404 = 0u64;
                        if nat_eq(a4403, a4404) {
                            let a4407 = match v4 {
                                Adt19::C0(_, _, h4405) => {
                                    h4405
                                }
                            };
                            let a4408 = match v0.clone() {
                                Adt2::C0(_, _, _, _, _, h4406, _, _, _, _, _) => {
                                    h4406
                                }
                            };
                            nat_le(a4407, a4408)
                        } else {
                            false
                        }
                    } else {
                        false
                    } {
                        let a4409 = v5;
                        let a4410 = 1u64;
                        if nat_eq(a4409, a4410) {
                            let a4412 = 4u64;
                            let a4413 = match v2.clone() {
                                Adt2::C0(_, _, _, _, _, h4411, _, _, _, _, _) => {
                                    h4411
                                }
                            };
                            nat_le(a4412, a4413)
                        } else {
                            let a4415 = match v2.clone() {
                                Adt2::C0(_, _, _, _, _, h4414, _, _, _, _, _) => {
                                    h4414
                                }
                            };
                            let a4416 = 4u64;
                            nat_eq(a4415, a4416)
                        }
                    } else {
                        false
                    } {
                        let a4421 = {
                            let a4418 = match v2.clone() {
                                Adt2::C0(_, h4417, _, _, _, _, _, _, _, _, _) => {
                                    h4417
                                }
                            };
                            let a4419 = v5;
                            let a4420 = 1u64;
                            nat_rem(a4418, a4419, a4420)
                        };
                        let a4422 = 0u64;
                        if nat_eq(a4421, a4422) {
                            true
                        } else {
                            f43(v2.clone())?
                        }
                    } else {
                        false
                    } {
                        Ok(Adt10::C4(f28(v0.clone(), v5)?, 4u64, f30(v0.clone(), v5), f30(v2, 1u64), Adt4::C11(match v0 {
                            Adt2::C0(h4423, _, _, _, _, _, _, _, _, _, _) => {
                                h4423
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

pub fn f137(v0: u64, v1: Adt2, v2: bool, v3: Adt2) -> R<Adt10> {
    let a4426 = match v1.clone() {
        Adt2::C0(h4424, _, _, _, _, _, _, _, _, _, _) => {
            h4424
        }
    };
    let a4427 = match v3.clone() {
        Adt2::C0(h4425, _, _, _, _, _, _, _, _, _, _) => {
            h4425
        }
    };
    if nat_eq(a4426, a4427) {
        f240(v0, v1, v2, v3)
    } else {
        if if if if f239(match v1.clone() {
            Adt2::C0(h4428, _, _, _, _, _, _, _, _, _, _) => {
                h4428
            }
        }) {
            f239(match v3.clone() {
                Adt2::C0(h4429, _, _, _, _, _, _, _, _, _, _) => {
                    h4429
                }
            })
        } else {
            false
        } {
            let a4430 = v0;
            let a4431 = 33u64;
            if nat_eq(a4430, a4431) {
                v2
            } else {
                true
            }
        } else {
            false
        } {
            let a4432 = f28(v1.clone(), 1u64)?;
            let a4433 = f28(v3.clone(), 1u64)?;
            nat_eq(a4432, a4433)
        } else {
            false
        } {
            let m4435 = f230(match v3.clone() {
                Adt2::C0(h4434, _, _, _, _, _, _, _, _, _, _) => {
                    h4434
                }
            });
            match m4435 {
                Adt18::C0 => {
                    Ok(Adt10::C0)
                }
                Adt18::C1(v4) => {
                    let v5: u64 = match v1.clone() {
                        Adt2::C0(h4436, _, _, _, _, _, _, _, _, _, _) => {
                            h4436
                        }
                    };
                    let v6: u64 = match v3.clone() {
                        Adt2::C0(h4437, _, _, _, _, _, _, _, _, _, _) => {
                            h4437
                        }
                    };
                    let v7: bool = {
                        let a4438 = v5;
                        let a4439 = 43u64;
                        if nat_eq(a4438, a4439) {
                            true
                        } else {
                            let a4440 = v6;
                            let a4441 = 43u64;
                            nat_eq(a4440, a4441)
                        }
                    };
                    Ok(Adt10::C4({
                        let a4442 = f28(v3.clone(), 1u64)?;
                        let a4443 = if v7 {
                            f178(v5, v6)
                        } else {
                            1u64
                        };
                        nat_mul(a4442, a4443)?
                    }, match v4 {
                        Adt19::C0(_, _, h4444) => {
                            h4444
                        }
                    }, f30(v1, 1u64), f30(v3, 1u64), if v7 {
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

pub fn f138(v0: Adt2, v1: Adt2) -> R<Adt10> {
    let a4446 = match v0.clone() {
        Adt2::C0(h4445, _, _, _, _, _, _, _, _, _, _) => {
            h4445
        }
    };
    let a4447 = 0u64;
    if if if if nat_eq(a4446, a4447) {
        true
    } else {
        let a4449 = match v0.clone() {
            Adt2::C0(h4448, _, _, _, _, _, _, _, _, _, _) => {
                h4448
            }
        };
        let a4450 = 1u64;
        nat_eq(a4449, a4450)
    } {
        true
    } else {
        let a4452 = match v0.clone() {
            Adt2::C0(h4451, _, _, _, _, _, _, _, _, _, _) => {
                h4451
            }
        };
        let a4453 = 25u64;
        if nat_eq(a4452, a4453) {
            true
        } else {
            let a4455 = match v0.clone() {
                Adt2::C0(h4454, _, _, _, _, _, _, _, _, _, _) => {
                    h4454
                }
            };
            let a4456 = 26u64;
            nat_eq(a4455, a4456)
        }
    } {
        true
    } else {
        let a4458 = match v0.clone() {
            Adt2::C0(h4457, _, _, _, _, _, _, _, _, _, _) => {
                h4457
            }
        };
        let a4459 = 30u64;
        nat_eq(a4458, a4459)
    } {
        let a4462 = match v0.clone() {
            Adt2::C0(h4460, _, _, _, _, _, _, _, _, _, _) => {
                h4460
            }
        };
        let a4463 = match v1.clone() {
            Adt2::C0(h4461, _, _, _, _, _, _, _, _, _, _) => {
                h4461
            }
        };
        if if nat_eq(a4462, a4463) {
            f241(match v1.clone() {
                Adt2::C0(_, h4464, _, _, _, _, _, _, _, _, _) => {
                    h4464
                }
            }, match v1.clone() {
                Adt2::C0(_, _, h4465, _, _, _, _, _, _, _, _) => {
                    h4465
                }
            }, match v1.clone() {
                Adt2::C0(_, _, _, h4466, _, _, _, _, _, _, _) => {
                    h4466
                }
            }, match v1.clone() {
                Adt2::C0(_, _, _, _, h4467, _, _, _, _, _, _) => {
                    h4467
                }
            }, match v0.clone() {
                Adt2::C0(_, h4468, _, _, _, _, _, _, _, _, _) => {
                    h4468
                }
            }, match v0.clone() {
                Adt2::C0(_, _, h4469, _, _, _, _, _, _, _, _) => {
                    h4469
                }
            }, match v0.clone() {
                Adt2::C0(_, _, _, h4470, _, _, _, _, _, _, _) => {
                    h4470
                }
            }, match v0.clone() {
                Adt2::C0(_, _, _, _, h4471, _, _, _, _, _, _) => {
                    h4471
                }
            })
        } else {
            false
        } {
            let m4473 = f230(match v0.clone() {
                Adt2::C0(h4472, _, _, _, _, _, _, _, _, _, _) => {
                    h4472
                }
            });
            match m4473 {
                Adt18::C0 => {
                    Ok(Adt10::C0)
                }
                Adt18::C1(v2) => {
                    Ok(Adt10::C4(f28(v1.clone(), 1u64)?, match v2 {
                        Adt19::C0(_, _, h4474) => {
                            h4474
                        }
                    }, f30(v0, 1u64), f30(v1, 1u64), Adt4::C1))
                }
            }
        } else {
            Ok(Adt10::C1)
        }
    } else {
        Ok(Adt10::C0)
    }
}

pub fn f139(v0: u64, v1: Adt2, v2: Option<Adt2>, v3: Adt2) -> R<Adt10> {
    let m4475 = v2;
    match m4475 {
        None => {
            Ok(Adt10::C1)
        }
        Some(v4) => {
            let a4476 = v0;
            let a4477 = 4u64;
            if if if nat_lt(a4476, a4477) {
                let a4480 = match v1.clone() {
                    Adt2::C0(h4478, _, _, _, _, _, _, _, _, _, _) => {
                        h4478
                    }
                };
                let a4481 = match v4.clone() {
                    Adt2::C0(h4479, _, _, _, _, _, _, _, _, _, _) => {
                        h4479
                    }
                };
                if nat_eq(a4480, a4481) {
                    let a4484 = match v1.clone() {
                        Adt2::C0(h4482, _, _, _, _, _, _, _, _, _, _) => {
                            h4482
                        }
                    };
                    let a4485 = match v3.clone() {
                        Adt2::C0(h4483, _, _, _, _, _, _, _, _, _, _) => {
                            h4483
                        }
                    };
                    nat_eq(a4484, a4485)
                } else {
                    false
                }
            } else {
                false
            } {
                if if if f242(v0, 0u64, match v1.clone() {
                    Adt2::C0(_, h4486, _, _, _, _, _, _, _, _, _) => {
                        h4486
                    }
                }, match v4.clone() {
                    Adt2::C0(_, h4487, _, _, _, _, _, _, _, _, _) => {
                        h4487
                    }
                }, match v3.clone() {
                    Adt2::C0(_, h4488, _, _, _, _, _, _, _, _, _) => {
                        h4488
                    }
                })? {
                    f242(v0, 1u64, match v1.clone() {
                        Adt2::C0(_, _, h4489, _, _, _, _, _, _, _, _) => {
                            h4489
                        }
                    }, match v4.clone() {
                        Adt2::C0(_, _, h4490, _, _, _, _, _, _, _, _) => {
                            h4490
                        }
                    }, match v3.clone() {
                        Adt2::C0(_, _, h4491, _, _, _, _, _, _, _, _) => {
                            h4491
                        }
                    })?
                } else {
                    false
                } {
                    f242(v0, 2u64, match v1.clone() {
                        Adt2::C0(_, _, _, h4492, _, _, _, _, _, _, _) => {
                            h4492
                        }
                    }, match v4.clone() {
                        Adt2::C0(_, _, _, h4493, _, _, _, _, _, _, _) => {
                            h4493
                        }
                    }, match v3.clone() {
                        Adt2::C0(_, _, _, h4494, _, _, _, _, _, _, _) => {
                            h4494
                        }
                    })?
                } else {
                    false
                } {
                    f242(v0, 3u64, match v1.clone() {
                        Adt2::C0(_, _, _, _, h4495, _, _, _, _, _, _) => {
                            h4495
                        }
                    }, match v4.clone() {
                        Adt2::C0(_, _, _, _, h4496, _, _, _, _, _, _) => {
                            h4496
                        }
                    }, match v3.clone() {
                        Adt2::C0(_, _, _, _, h4497, _, _, _, _, _, _) => {
                            h4497
                        }
                    })?
                } else {
                    false
                }
            } else {
                false
            } {
                let m4499 = f230(match v1.clone() {
                    Adt2::C0(h4498, _, _, _, _, _, _, _, _, _, _) => {
                        h4498
                    }
                });
                match m4499 {
                    Adt18::C0 => {
                        Ok(Adt10::C0)
                    }
                    Adt18::C1(v5) => {
                        let v6: u64 = match v5.clone() {
                            Adt19::C0(_, h4500, _) => {
                                h4500
                            }
                        };
                        let a4505 = {
                            let a4502 = match v1.clone() {
                                Adt2::C0(_, h4501, _, _, _, _, _, _, _, _, _) => {
                                    h4501
                                }
                            };
                            let a4503 = v6;
                            let a4504 = 1u64;
                            nat_rem(a4502, a4503, a4504)
                        };
                        let a4506 = 0u64;
                        if if if nat_eq(a4505, a4506) {
                            let a4511 = {
                                let a4508 = match v4.clone() {
                                    Adt2::C0(_, h4507, _, _, _, _, _, _, _, _, _) => {
                                        h4507
                                    }
                                };
                                let a4509 = v6;
                                let a4510 = 1u64;
                                nat_rem(a4508, a4509, a4510)
                            };
                            let a4512 = 0u64;
                            nat_eq(a4511, a4512)
                        } else {
                            false
                        } {
                            let a4517 = {
                                let a4514 = match v3.clone() {
                                    Adt2::C0(_, h4513, _, _, _, _, _, _, _, _, _) => {
                                        h4513
                                    }
                                };
                                let a4515 = v6;
                                let a4516 = 1u64;
                                nat_rem(a4514, a4515, a4516)
                            };
                            let a4518 = 0u64;
                            nat_eq(a4517, a4518)
                        } else {
                            false
                        } {
                            Ok(Adt10::C4(f28(v3.clone(), v6)?, match v5 {
                                Adt19::C0(_, _, h4519) => {
                                    h4519
                                }
                            }, f30(v1, v6), f30(v3, v6), Adt4::C2(v0, f30(v4, v6))))
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

pub fn f140(v0: u64, v1: Adt1, v2: Adt2, v3: Adt2) -> R<Adt10> {
    let a4521 = match v2.clone() {
        Adt2::C0(h4520, _, _, _, _, _, _, _, _, _, _) => {
            h4520
        }
    };
    let a4522 = 0u64;
    if if nat_eq(a4521, a4522) {
        let a4524 = match v3.clone() {
            Adt2::C0(h4523, _, _, _, _, _, _, _, _, _, _) => {
                h4523
            }
        };
        let a4525 = 0u64;
        nat_eq(a4524, a4525)
    } else {
        false
    } {
        let a4526 = v0;
        let a4527 = 61u64;
        if nat_eq(a4526, a4527) {
            let v4: bool = {
                let a4529 = 0u64;
                let a4530 = match v1.clone() {
                    Adt1::C0(_, _, _, _, _, _, _, _, h4528, _, _, _) => {
                        h4528
                    }
                };
                nat_lt(a4529, a4530)
            };
            if if if if if if f243(match v2.clone() {
                Adt2::C0(_, h4531, _, _, _, _, _, _, _, _, _) => {
                    h4531
                }
            }, match v1.clone() {
                Adt1::C0(h4532, _, _, _, _, _, _, _, _, _, _, _) => {
                    h4532
                }
            }, match v1.clone() {
                Adt1::C0(_, h4533, _, _, _, _, _, _, _, _, _, _) => {
                    h4533
                }
            }, match v3.clone() {
                Adt2::C0(_, h4534, _, _, _, _, _, _, _, _, _) => {
                    h4534
                }
            })? {
                f243(match v2.clone() {
                    Adt2::C0(_, _, h4535, _, _, _, _, _, _, _, _) => {
                        h4535
                    }
                }, match v1.clone() {
                    Adt1::C0(_, _, h4536, _, _, _, _, _, _, _, _, _) => {
                        h4536
                    }
                }, match v1.clone() {
                    Adt1::C0(_, _, _, h4537, _, _, _, _, _, _, _, _) => {
                        h4537
                    }
                }, match v3.clone() {
                    Adt2::C0(_, _, h4538, _, _, _, _, _, _, _, _) => {
                        h4538
                    }
                })?
            } else {
                false
            } {
                f243(match v2.clone() {
                    Adt2::C0(_, _, _, h4539, _, _, _, _, _, _, _) => {
                        h4539
                    }
                }, match v1.clone() {
                    Adt1::C0(_, _, _, _, h4540, _, _, _, _, _, _, _) => {
                        h4540
                    }
                }, match v1.clone() {
                    Adt1::C0(_, _, _, _, _, h4541, _, _, _, _, _, _) => {
                        h4541
                    }
                }, match v3.clone() {
                    Adt2::C0(_, _, _, h4542, _, _, _, _, _, _, _) => {
                        h4542
                    }
                })?
            } else {
                false
            } {
                f243(match v2.clone() {
                    Adt2::C0(_, _, _, _, h4543, _, _, _, _, _, _) => {
                        h4543
                    }
                }, match v1.clone() {
                    Adt1::C0(_, _, _, _, _, _, h4544, _, _, _, _, _) => {
                        h4544
                    }
                }, match v1.clone() {
                    Adt1::C0(_, _, _, _, _, _, _, h4545, _, _, _, _) => {
                        h4545
                    }
                }, match v3.clone() {
                    Adt2::C0(_, _, _, _, h4546, _, _, _, _, _, _) => {
                        h4546
                    }
                })?
            } else {
                false
            } {
                let a4548 = match v1.clone() {
                    Adt1::C0(_, _, _, _, _, _, _, _, h4547, _, _, _) => {
                        h4547
                    }
                };
                let a4549 = 4294967295u64;
                nat_le(a4548, a4549)
            } else {
                false
            } {
                if v4 {
                    !f53(match v2.clone() {
                        Adt2::C0(_, h4550, _, _, _, _, _, _, _, _, _) => {
                            h4550
                        }
                    }, match v2.clone() {
                        Adt2::C0(_, _, h4551, _, _, _, _, _, _, _, _) => {
                            h4551
                        }
                    }, match v2.clone() {
                        Adt2::C0(_, _, _, h4552, _, _, _, _, _, _, _) => {
                            h4552
                        }
                    }, match v2.clone() {
                        Adt2::C0(_, _, _, _, h4553, _, _, _, _, _, _) => {
                            h4553
                        }
                    })
                } else {
                    true
                }
            } else {
                false
            } {
                Ok(Adt10::C4(f28(v3.clone(), 1u64)?, 4u64, f30(v2, 1u64), f30(v3, 1u64), Adt4::C3(match v1.clone() {
                    Adt1::C0(h4554, _, _, _, _, _, _, _, _, _, _, _) => {
                        h4554
                    }
                }, match v1.clone() {
                    Adt1::C0(_, _, h4555, _, _, _, _, _, _, _, _, _) => {
                        h4555
                    }
                }, match v1.clone() {
                    Adt1::C0(_, _, _, _, h4556, _, _, _, _, _, _, _) => {
                        h4556
                    }
                }, match v1 {
                    Adt1::C0(_, _, _, _, _, _, h4557, _, _, _, _, _) => {
                        h4557
                    }
                }, v4)))
            } else {
                Ok(Adt10::C1)
            }
        } else {
            let a4558 = v0;
            let a4559 = 62u64;
            if nat_eq(a4558, a4559) {
                let a4561 = match v1.clone() {
                    Adt1::C0(h4560, _, _, _, _, _, _, _, _, _, _, _) => {
                        h4560
                    }
                };
                let a4562 = 2147483648u64;
                if if if if if nat_lt(a4561, a4562) {
                    let a4564 = match v1.clone() {
                        Adt1::C0(_, h4563, _, _, _, _, _, _, _, _, _, _) => {
                            h4563
                        }
                    };
                    let a4565 = 2147483648u64;
                    nat_lt(a4564, a4565)
                } else {
                    false
                } {
                    let a4568 = match v1.clone() {
                        Adt1::C0(h4566, _, _, _, _, _, _, _, _, _, _, _) => {
                            h4566
                        }
                    };
                    let a4569 = match v2.clone() {
                        Adt2::C0(_, h4567, _, _, _, _, _, _, _, _, _) => {
                            h4567
                        }
                    };
                    nat_lt(a4568, a4569)
                } else {
                    false
                } {
                    let a4572 = match v1.clone() {
                        Adt1::C0(_, h4570, _, _, _, _, _, _, _, _, _, _) => {
                            h4570
                        }
                    };
                    let a4573 = match v2.clone() {
                        Adt2::C0(_, h4571, _, _, _, _, _, _, _, _, _) => {
                            h4571
                        }
                    };
                    nat_lt(a4572, a4573)
                } else {
                    false
                } {
                    let a4576 = match v2.clone() {
                        Adt2::C0(_, _, h4574, _, _, _, _, _, _, _, _) => {
                            h4574
                        }
                    };
                    let a4577 = match v3.clone() {
                        Adt2::C0(_, _, h4575, _, _, _, _, _, _, _, _) => {
                            h4575
                        }
                    };
                    if if if nat_eq(a4576, a4577) {
                        let a4580 = match v2.clone() {
                            Adt2::C0(_, _, _, h4578, _, _, _, _, _, _, _) => {
                                h4578
                            }
                        };
                        let a4581 = match v3.clone() {
                            Adt2::C0(_, _, _, h4579, _, _, _, _, _, _, _) => {
                                h4579
                            }
                        };
                        nat_eq(a4580, a4581)
                    } else {
                        false
                    } {
                        let a4584 = match v2.clone() {
                            Adt2::C0(_, _, _, _, h4582, _, _, _, _, _, _) => {
                                h4582
                            }
                        };
                        let a4585 = match v3.clone() {
                            Adt2::C0(_, _, _, _, h4583, _, _, _, _, _, _) => {
                                h4583
                            }
                        };
                        nat_eq(a4584, a4585)
                    } else {
                        false
                    } {
                        let a4594 = match v3.clone() {
                            Adt2::C0(_, h4586, _, _, _, _, _, _, _, _, _) => {
                                h4586
                            }
                        };
                        let a4595 = {
                            let a4592 = {
                                let a4589 = match v2.clone() {
                                    Adt2::C0(_, h4587, _, _, _, _, _, _, _, _, _) => {
                                        h4587
                                    }
                                };
                                let a4590 = match v1.clone() {
                                    Adt1::C0(h4588, _, _, _, _, _, _, _, _, _, _, _) => {
                                        h4588
                                    }
                                };
                                nat_add(a4589, a4590)?
                            };
                            let a4593 = match v1.clone() {
                                Adt1::C0(_, h4591, _, _, _, _, _, _, _, _, _, _) => {
                                    h4591
                                }
                            };
                            nat_add(a4592, a4593)?
                        };
                        nat_eq(a4594, a4595)
                    } else {
                        false
                    }
                } else {
                    false
                } {
                    Ok(Adt10::C4(f28(v3.clone(), 1u64)?, 4u64, f30(v2, 1u64), f30(v3, 1u64), Adt4::C4(match v1 {
                        Adt1::C0(h4596, _, _, _, _, _, _, _, _, _, _, _) => {
                            h4596
                        }
                    })))
                } else {
                    Ok(Adt10::C1)
                }
            } else {
                let a4597 = v0;
                let a4598 = 63u64;
                if nat_eq(a4597, a4598) {
                    if if if if if f244(match v2.clone() {
                        Adt2::C0(_, h4599, _, _, _, _, _, _, _, _, _) => {
                            h4599
                        }
                    }, match v1.clone() {
                        Adt1::C0(h4600, _, _, _, _, _, _, _, _, _, _, _) => {
                            h4600
                        }
                    })? {
                        f244(match v2.clone() {
                            Adt2::C0(_, _, h4601, _, _, _, _, _, _, _, _) => {
                                h4601
                            }
                        }, match v1.clone() {
                            Adt1::C0(_, h4602, _, _, _, _, _, _, _, _, _, _) => {
                                h4602
                            }
                        })?
                    } else {
                        false
                    } {
                        f244(match v2.clone() {
                            Adt2::C0(_, _, _, h4603, _, _, _, _, _, _, _) => {
                                h4603
                            }
                        }, match v1.clone() {
                            Adt1::C0(_, _, h4604, _, _, _, _, _, _, _, _, _) => {
                                h4604
                            }
                        })?
                    } else {
                        false
                    } {
                        f244(match v2.clone() {
                            Adt2::C0(_, _, _, _, h4605, _, _, _, _, _, _) => {
                                h4605
                            }
                        }, match v1.clone() {
                            Adt1::C0(_, _, _, h4606, _, _, _, _, _, _, _, _) => {
                                h4606
                            }
                        })?
                    } else {
                        false
                    } {
                        let a4609 = match v2.clone() {
                            Adt2::C0(_, h4607, _, _, _, _, _, _, _, _, _) => {
                                h4607
                            }
                        };
                        let a4610 = match v3.clone() {
                            Adt2::C0(_, h4608, _, _, _, _, _, _, _, _, _) => {
                                h4608
                            }
                        };
                        if if if nat_eq(a4609, a4610) {
                            let a4613 = match v2.clone() {
                                Adt2::C0(_, _, h4611, _, _, _, _, _, _, _, _) => {
                                    h4611
                                }
                            };
                            let a4614 = match v3.clone() {
                                Adt2::C0(_, _, h4612, _, _, _, _, _, _, _, _) => {
                                    h4612
                                }
                            };
                            nat_eq(a4613, a4614)
                        } else {
                            false
                        } {
                            let a4617 = match v2.clone() {
                                Adt2::C0(_, _, _, h4615, _, _, _, _, _, _, _) => {
                                    h4615
                                }
                            };
                            let a4618 = match v3.clone() {
                                Adt2::C0(_, _, _, h4616, _, _, _, _, _, _, _) => {
                                    h4616
                                }
                            };
                            nat_eq(a4617, a4618)
                        } else {
                            false
                        } {
                            let a4621 = match v2.clone() {
                                Adt2::C0(_, _, _, _, h4619, _, _, _, _, _, _) => {
                                    h4619
                                }
                            };
                            let a4622 = match v3.clone() {
                                Adt2::C0(_, _, _, _, h4620, _, _, _, _, _, _) => {
                                    h4620
                                }
                            };
                            nat_eq(a4621, a4622)
                        } else {
                            false
                        }
                    } else {
                        false
                    } {
                        Ok(Adt10::C4(f28(v3.clone(), 1u64)?, 4u64, f30(v2, 1u64), f30(v3, 1u64), Adt4::C5(match v1.clone() {
                            Adt1::C0(h4623, _, _, _, _, _, _, _, _, _, _, _) => {
                                h4623
                            }
                        }, match v1.clone() {
                            Adt1::C0(_, h4624, _, _, _, _, _, _, _, _, _, _) => {
                                h4624
                            }
                        }, match v1.clone() {
                            Adt1::C0(_, _, h4625, _, _, _, _, _, _, _, _, _) => {
                                h4625
                            }
                        }, match v1 {
                            Adt1::C0(_, _, _, h4626, _, _, _, _, _, _, _, _) => {
                                h4626
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

pub fn f141(v0: u64, v1: Adt1, v2: Adt2, v3: Option<Adt2>, v4: Adt2) -> R<Adt10> {
    let m4627 = v3;
    match m4627 {
        None => {
            Ok(Adt10::C1)
        }
        Some(v5) => {
            let v6: bool = {
                let a4628 = v0;
                let a4629 = 4u64;
                nat_eq(a4628, a4629)
            };
            let v7: bool = {
                let a4631 = 0u64;
                let a4632 = match v1.clone() {
                    Adt1::C0(_, _, _, _, h4630, _, _, _, _, _, _, _) => {
                        h4630
                    }
                };
                nat_lt(a4631, a4632)
            };
            let v8: u64 = f28(v5.clone(), 1u64)?;
            let v9: Adt3 = Adt3::C0(match v5.clone() {
                Adt2::C0(_, h4633, _, _, _, _, _, _, _, _, _) => {
                    h4633
                }
            }, match v5.clone() {
                Adt2::C0(_, _, h4634, _, _, _, _, _, _, _, _) => {
                    h4634
                }
            }, match v5.clone() {
                Adt2::C0(_, _, _, h4635, _, _, _, _, _, _, _) => {
                    h4635
                }
            }, match v5.clone() {
                Adt2::C0(_, _, _, _, h4636, _, _, _, _, _, _) => {
                    h4636
                }
            }, 4u64, match v1.clone() {
                Adt1::C0(h4637, _, _, _, _, _, _, _, _, _, _, _) => {
                    h4637
                }
            }, match v1.clone() {
                Adt1::C0(_, h4638, _, _, _, _, _, _, _, _, _, _) => {
                    h4638
                }
            }, match v1.clone() {
                Adt1::C0(_, _, h4639, _, _, _, _, _, _, _, _, _) => {
                    h4639
                }
            });
            if if if if if if if if if if if if v6 {
                let a4641 = match v2.clone() {
                    Adt2::C0(h4640, _, _, _, _, _, _, _, _, _, _) => {
                        h4640
                    }
                };
                let a4642 = 0u64;
                if nat_eq(a4641, a4642) {
                    let a4644 = match v5.clone() {
                        Adt2::C0(h4643, _, _, _, _, _, _, _, _, _, _) => {
                            h4643
                        }
                    };
                    let a4645 = 0u64;
                    nat_eq(a4644, a4645)
                } else {
                    false
                }
            } else {
                let a4647 = match v2.clone() {
                    Adt2::C0(h4646, _, _, _, _, _, _, _, _, _, _) => {
                        h4646
                    }
                };
                let a4648 = 0u64;
                if if nat_eq(a4647, a4648) {
                    true
                } else {
                    let a4650 = match v2.clone() {
                        Adt2::C0(h4649, _, _, _, _, _, _, _, _, _, _) => {
                            h4649
                        }
                    };
                    let a4651 = 26u64;
                    nat_eq(a4650, a4651)
                } {
                    let a4653 = match v5.clone() {
                        Adt2::C0(h4652, _, _, _, _, _, _, _, _, _, _) => {
                            h4652
                        }
                    };
                    let a4654 = 0u64;
                    if nat_eq(a4653, a4654) {
                        true
                    } else {
                        let a4656 = match v5.clone() {
                            Adt2::C0(h4655, _, _, _, _, _, _, _, _, _, _) => {
                                h4655
                            }
                        };
                        let a4657 = 26u64;
                        nat_eq(a4656, a4657)
                    }
                } else {
                    false
                }
            } {
                let a4660 = match v2.clone() {
                    Adt2::C0(h4658, _, _, _, _, _, _, _, _, _, _) => {
                        h4658
                    }
                };
                let a4661 = match v4.clone() {
                    Adt2::C0(h4659, _, _, _, _, _, _, _, _, _, _) => {
                        h4659
                    }
                };
                nat_eq(a4660, a4661)
            } else {
                false
            } {
                f26(v2.clone(), v4.clone())
            } else {
                false
            } {
                f43(v2.clone())?
            } else {
                false
            } {
                f43(v4.clone())?
            } else {
                false
            } {
                let a4663 = match v5.clone() {
                    Adt2::C0(_, _, _, _, _, h4662, _, _, _, _, _) => {
                        h4662
                    }
                };
                let a4664 = 4u64;
                nat_eq(a4663, a4664)
            } else {
                false
            } {
                let a4665 = v8;
                let a4666 = f28(v2.clone(), 1u64)?;
                nat_le(a4665, a4666)
            } else {
                false
            } {
                let a4668 = match v1.clone() {
                    Adt1::C0(_, _, _, h4667, _, _, _, _, _, _, _, _) => {
                        h4667
                    }
                };
                let a4669 = 1073741824u64;
                nat_lt(a4668, a4669)
            } else {
                false
            } {
                let a4671 = match v1.clone() {
                    Adt1::C0(h4670, _, _, _, _, _, _, _, _, _, _, _) => {
                        h4670
                    }
                };
                let a4672 = 4294967295u64;
                if if if if nat_le(a4671, a4672) {
                    let a4674 = match v1.clone() {
                        Adt1::C0(_, h4673, _, _, _, _, _, _, _, _, _, _) => {
                            h4673
                        }
                    };
                    let a4675 = 4294967295u64;
                    nat_le(a4674, a4675)
                } else {
                    false
                } {
                    let a4677 = match v1.clone() {
                        Adt1::C0(_, _, h4676, _, _, _, _, _, _, _, _, _) => {
                            h4676
                        }
                    };
                    let a4678 = 4294967295u64;
                    nat_le(a4677, a4678)
                } else {
                    false
                } {
                    let a4680 = match v1.clone() {
                        Adt1::C0(_, _, _, h4679, _, _, _, _, _, _, _, _) => {
                            h4679
                        }
                    };
                    let a4681 = 4294967295u64;
                    nat_le(a4680, a4681)
                } else {
                    false
                } {
                    let a4683 = match v1.clone() {
                        Adt1::C0(_, _, _, _, h4682, _, _, _, _, _, _, _) => {
                            h4682
                        }
                    };
                    let a4684 = 4294967295u64;
                    nat_le(a4683, a4684)
                } else {
                    false
                }
            } else {
                false
            } {
                if v7 {
                    let a4687 = match v2.clone() {
                        Adt2::C0(_, _, _, _, _, _, _, _, _, _, h4685) => {
                            h4685
                        }
                    };
                    let a4688 = match v4.clone() {
                        Adt2::C0(_, _, _, _, _, _, _, _, _, _, h4686) => {
                            h4686
                        }
                    };
                    nat_eq(a4687, a4688)
                } else {
                    true
                }
            } else {
                false
            } {
                f245(v9.clone(), match v1.clone() {
                    Adt1::C0(_, _, _, h4689, _, _, _, _, _, _, _, _) => {
                        h4689
                    }
                }, match v4 {
                    Adt2::C0(_, _, _, _, _, _, _, _, _, h4690, _) => {
                        h4690
                    }
                }, v8, v6)?
            } else {
                false
            } {
                let v10: u64 = if v7 {
                    0u64
                } else {
                    f28(v2, 1u64)?
                };
                Ok(Adt10::C4({
                    let a4691 = v10;
                    let a4692 = v8;
                    nat_add(a4691, a4692)?
                }, 4u64, f30(v5, 1u64), v9, Adt4::C6(v10, match v1 {
                    Adt1::C0(_, _, _, h4693, _, _, _, _, _, _, _, _) => {
                        h4693
                    }
                }, v6)))
            } else {
                Ok(Adt10::C1)
            }
        }
    }
}

pub fn f142(v0: Adt2, v1: Adt2) -> R<Adt10> {
    let a4695 = match v0.clone() {
        Adt2::C0(h4694, _, _, _, _, _, _, _, _, _, _) => {
            h4694
        }
    };
    let a4696 = 0u64;
    if if if nat_eq(a4695, a4696) {
        let a4698 = match v1.clone() {
            Adt2::C0(h4697, _, _, _, _, _, _, _, _, _, _) => {
                h4697
            }
        };
        let a4699 = 0u64;
        nat_eq(a4698, a4699)
    } else {
        false
    } {
        f241(match v0.clone() {
            Adt2::C0(_, h4700, _, _, _, _, _, _, _, _, _) => {
                h4700
            }
        }, match v0.clone() {
            Adt2::C0(_, _, h4701, _, _, _, _, _, _, _, _) => {
                h4701
            }
        }, match v0.clone() {
            Adt2::C0(_, _, _, h4702, _, _, _, _, _, _, _) => {
                h4702
            }
        }, match v0.clone() {
            Adt2::C0(_, _, _, _, h4703, _, _, _, _, _, _) => {
                h4703
            }
        }, match v1.clone() {
            Adt2::C0(_, h4704, _, _, _, _, _, _, _, _, _) => {
                h4704
            }
        }, match v1.clone() {
            Adt2::C0(_, _, h4705, _, _, _, _, _, _, _, _) => {
                h4705
            }
        }, match v1.clone() {
            Adt2::C0(_, _, _, h4706, _, _, _, _, _, _, _) => {
                h4706
            }
        }, match v1.clone() {
            Adt2::C0(_, _, _, _, h4707, _, _, _, _, _, _) => {
                h4707
            }
        })
    } else {
        false
    } {
        let v2: u64 = f28(v1.clone(), 1u64)?;
        Ok(Adt10::C4({
            let a4708 = v2;
            let a4709 = f28(v0.clone(), 1u64)?;
            nat_add(a4708, a4709)?
        }, 4u64, f30(v0, 1u64), f30(v1, 1u64), Adt4::C7(v2)))
    } else {
        Ok(Adt10::C1)
    }
}

pub fn f143(v0: u64, v1: Adt2, v2: Adt2) -> R<Adt10> {
    let a4711 = match v1.clone() {
        Adt2::C0(h4710, _, _, _, _, _, _, _, _, _, _) => {
            h4710
        }
    };
    let a4712 = 0u64;
    if if if nat_eq(a4711, a4712) {
        true
    } else {
        let a4714 = match v1.clone() {
            Adt2::C0(h4713, _, _, _, _, _, _, _, _, _, _) => {
                h4713
            }
        };
        let a4715 = 1u64;
        nat_eq(a4714, a4715)
    } {
        true
    } else {
        let a4717 = match v1.clone() {
            Adt2::C0(h4716, _, _, _, _, _, _, _, _, _, _) => {
                h4716
            }
        };
        let a4718 = 30u64;
        nat_eq(a4717, a4718)
    } {
        let a4721 = match v1.clone() {
            Adt2::C0(h4719, _, _, _, _, _, _, _, _, _, _) => {
                h4719
            }
        };
        let a4722 = match v2.clone() {
            Adt2::C0(h4720, _, _, _, _, _, _, _, _, _, _) => {
                h4720
            }
        };
        if if if if if nat_eq(a4721, a4722) {
            let a4725 = match v1.clone() {
                Adt2::C0(_, h4723, _, _, _, _, _, _, _, _, _) => {
                    h4723
                }
            };
            let a4726 = match v2.clone() {
                Adt2::C0(_, h4724, _, _, _, _, _, _, _, _, _) => {
                    h4724
                }
            };
            nat_eq(a4725, a4726)
        } else {
            false
        } {
            let a4729 = match v1.clone() {
                Adt2::C0(_, _, _, h4727, _, _, _, _, _, _, _) => {
                    h4727
                }
            };
            let a4730 = match v2.clone() {
                Adt2::C0(_, _, _, h4728, _, _, _, _, _, _, _) => {
                    h4728
                }
            };
            nat_eq(a4729, a4730)
        } else {
            false
        } {
            let a4733 = match v1.clone() {
                Adt2::C0(_, _, _, _, h4731, _, _, _, _, _, _) => {
                    h4731
                }
            };
            let a4734 = match v2.clone() {
                Adt2::C0(_, _, _, _, h4732, _, _, _, _, _, _) => {
                    h4732
                }
            };
            nat_eq(a4733, a4734)
        } else {
            false
        } {
            let a4735 = v0;
            let a4736 = 4u64;
            if nat_eq(a4735, a4736) {
                let a4738 = match v1.clone() {
                    Adt2::C0(_, _, h4737, _, _, _, _, _, _, _, _) => {
                        h4737
                    }
                };
                let a4739 = 1u64;
                if nat_eq(a4738, a4739) {
                    let a4742 = match v1.clone() {
                        Adt2::C0(_, h4740, _, _, _, _, _, _, _, _, _) => {
                            h4740
                        }
                    };
                    let a4743 = match v2.clone() {
                        Adt2::C0(_, _, h4741, _, _, _, _, _, _, _, _) => {
                            h4741
                        }
                    };
                    nat_eq(a4742, a4743)
                } else {
                    false
                }
            } else {
                let a4744 = v0;
                let a4745 = 4u64;
                if if nat_lt(a4744, a4745) {
                    let a4748 = match v1.clone() {
                        Adt2::C0(_, h4746, _, _, _, _, _, _, _, _, _) => {
                            h4746
                        }
                    };
                    let a4749 = match v1.clone() {
                        Adt2::C0(_, _, h4747, _, _, _, _, _, _, _, _) => {
                            h4747
                        }
                    };
                    nat_eq(a4748, a4749)
                } else {
                    false
                } {
                    let a4752 = match v1.clone() {
                        Adt2::C0(_, _, h4750, _, _, _, _, _, _, _, _) => {
                            h4750
                        }
                    };
                    let a4753 = match v2.clone() {
                        Adt2::C0(_, _, h4751, _, _, _, _, _, _, _, _) => {
                            h4751
                        }
                    };
                    nat_eq(a4752, a4753)
                } else {
                    false
                }
            }
        } else {
            false
        } {
            let m4755 = f230(match v1.clone() {
                Adt2::C0(h4754, _, _, _, _, _, _, _, _, _, _) => {
                    h4754
                }
            });
            match m4755 {
                Adt18::C0 => {
                    Ok(Adt10::C0)
                }
                Adt18::C1(v3) => {
                    Ok(Adt10::C4(f28(v2.clone(), 1u64)?, match v3 {
                        Adt19::C0(_, _, h4756) => {
                            h4756
                        }
                    }, f30(v1, 1u64), f30(v2, 1u64), Adt4::C8(v0)))
                }
            }
        } else {
            Ok(Adt10::C1)
        }
    } else {
        Ok(Adt10::C0)
    }
}

pub fn f144(v0: u64, v1: Adt2, v2: Adt2) -> R<Adt10> {
    if if if f246(match v1.clone() {
        Adt2::C0(h4757, _, _, _, _, _, _, _, _, _, _) => {
            h4757
        }
    }, v0) {
        let a4760 = match v1.clone() {
            Adt2::C0(h4758, _, _, _, _, _, _, _, _, _, _) => {
                h4758
            }
        };
        let a4761 = match v2.clone() {
            Adt2::C0(h4759, _, _, _, _, _, _, _, _, _, _) => {
                h4759
            }
        };
        nat_eq(a4760, a4761)
    } else {
        false
    } {
        let a4764 = match v1.clone() {
            Adt2::C0(_, h4762, _, _, _, _, _, _, _, _, _) => {
                h4762
            }
        };
        let a4765 = match v2.clone() {
            Adt2::C0(_, h4763, _, _, _, _, _, _, _, _, _) => {
                h4763
            }
        };
        if if if nat_eq(a4764, a4765) {
            let a4768 = match v1.clone() {
                Adt2::C0(_, _, h4766, _, _, _, _, _, _, _, _) => {
                    h4766
                }
            };
            let a4769 = match v2.clone() {
                Adt2::C0(_, _, h4767, _, _, _, _, _, _, _, _) => {
                    h4767
                }
            };
            nat_eq(a4768, a4769)
        } else {
            false
        } {
            let a4772 = match v1.clone() {
                Adt2::C0(_, _, _, h4770, _, _, _, _, _, _, _) => {
                    h4770
                }
            };
            let a4773 = match v2.clone() {
                Adt2::C0(_, _, _, h4771, _, _, _, _, _, _, _) => {
                    h4771
                }
            };
            nat_eq(a4772, a4773)
        } else {
            false
        } {
            let a4776 = match v1.clone() {
                Adt2::C0(_, _, _, _, h4774, _, _, _, _, _, _) => {
                    h4774
                }
            };
            let a4777 = match v2.clone() {
                Adt2::C0(_, _, _, _, h4775, _, _, _, _, _, _) => {
                    h4775
                }
            };
            nat_eq(a4776, a4777)
        } else {
            false
        }
    } else {
        false
    } {
        let m4779 = f230(match v1.clone() {
            Adt2::C0(h4778, _, _, _, _, _, _, _, _, _, _) => {
                h4778
            }
        });
        match m4779 {
            Adt18::C0 => {
                Ok(Adt10::C0)
            }
            Adt18::C1(v3) => {
                Ok(Adt10::C4(f28(v2.clone(), 1u64)?, match v3 {
                    Adt19::C0(_, _, h4780) => {
                        h4780
                    }
                }, f30(v1.clone(), 1u64), f30(v2, 1u64), Adt4::C9(match v1 {
                    Adt2::C0(h4781, _, _, _, _, _, _, _, _, _, _) => {
                        h4781
                    }
                }, v0)))
            }
        }
    } else {
        Ok(Adt10::C1)
    }
}

pub fn f145(v0: bool, v1: Adt1, v2: Adt2, v3: Adt2) -> R<Adt10> {
    let v4: u64 = if v0 {
        match v1.clone() {
            Adt1::C0(h4782, _, _, _, _, _, _, _, _, _, _, _) => {
                h4782
            }
        }
    } else {
        match v1.clone() {
            Adt1::C0(_, _, h4783, _, _, _, _, _, _, _, _, _) => {
                h4783
            }
        }
    };
    let a4784 = 0u64;
    let a4785 = v4;
    if if nat_lt(a4784, a4785) {
        let a4786 = v4;
        let a4787 = 2147483648u64;
        nat_lt(a4786, a4787)
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
        let v7: u64 = f247(match v5.clone() {
            Adt2::C0(_, _, h4788, _, _, _, _, _, _, _, _) => {
                h4788
            }
        }, v4)?;
        let v8: u64 = f247(match v5.clone() {
            Adt2::C0(_, _, _, h4789, _, _, _, _, _, _, _) => {
                h4789
            }
        }, v4)?;
        let a4791 = match v5.clone() {
            Adt2::C0(h4790, _, _, _, _, _, _, _, _, _, _) => {
                h4790
            }
        };
        let a4792 = 0u64;
        if if if if if nat_eq(a4791, a4792) {
            let a4794 = match v6.clone() {
                Adt2::C0(h4793, _, _, _, _, _, _, _, _, _, _) => {
                    h4793
                }
            };
            let a4795 = 0u64;
            nat_eq(a4794, a4795)
        } else {
            false
        } {
            let a4797 = match v5.clone() {
                Adt2::C0(_, _, _, _, h4796, _, _, _, _, _, _) => {
                    h4796
                }
            };
            let a4798 = 1u64;
            nat_eq(a4797, a4798)
        } else {
            false
        } {
            let a4801 = match v5.clone() {
                Adt2::C0(_, h4799, _, _, _, _, _, _, _, _, _) => {
                    h4799
                }
            };
            let a4802 = match v6.clone() {
                Adt2::C0(_, h4800, _, _, _, _, _, _, _, _, _) => {
                    h4800
                }
            };
            nat_eq(a4801, a4802)
        } else {
            false
        } {
            if v0 {
                let a4804 = match v5.clone() {
                    Adt2::C0(_, _, h4803, _, _, _, _, _, _, _, _) => {
                        h4803
                    }
                };
                let a4805 = 2147483648u64;
                if if if nat_lt(a4804, a4805) {
                    let a4807 = match v5.clone() {
                        Adt2::C0(_, _, _, h4806, _, _, _, _, _, _, _) => {
                            h4806
                        }
                    };
                    let a4808 = 2147483648u64;
                    nat_lt(a4807, a4808)
                } else {
                    false
                } {
                    let a4811 = {
                        let a4809 = v7;
                        let a4810 = v8;
                        nat_mul(a4809, a4810)?
                    };
                    let a4812 = 2147483648u64;
                    nat_le(a4811, a4812)
                } else {
                    false
                } {
                    f248(v5, v6, v4, v7)?
                } else {
                    false
                }
            } else {
                let a4814 = match v1.clone() {
                    Adt1::C0(h4813, _, _, _, _, _, _, _, _, _, _, _) => {
                        h4813
                    }
                };
                let a4815 = v7;
                if if if if nat_eq(a4814, a4815) {
                    let a4817 = match v1 {
                        Adt1::C0(_, h4816, _, _, _, _, _, _, _, _, _, _) => {
                            h4816
                        }
                    };
                    let a4818 = v8;
                    nat_eq(a4817, a4818)
                } else {
                    false
                } {
                    f249(v7, v8, v4)?
                } else {
                    false
                } {
                    let a4820 = match v6.clone() {
                        Adt2::C0(_, _, h4819, _, _, _, _, _, _, _, _) => {
                            h4819
                        }
                    };
                    let a4821 = v4;
                    if nat_eq(a4820, a4821) {
                        let a4823 = match v6.clone() {
                            Adt2::C0(_, _, _, h4822, _, _, _, _, _, _, _) => {
                                h4822
                            }
                        };
                        let a4824 = v4;
                        nat_eq(a4823, a4824)
                    } else {
                        false
                    }
                } else {
                    false
                } {
                    let a4828 = match v6 {
                        Adt2::C0(_, _, _, _, h4825, _, _, _, _, _, _) => {
                            h4825
                        }
                    };
                    let a4829 = {
                        let a4826 = v7;
                        let a4827 = v8;
                        nat_mul(a4826, a4827)?
                    };
                    nat_eq(a4828, a4829)
                } else {
                    false
                }
            }
        } else {
            false
        } {
            Ok(Adt10::C4(f28(v3.clone(), 1u64)?, 4u64, f30(v2, 1u64), f30(v3, 1u64), Adt4::C12(v0, v4, v7)))
        } else {
            Ok(Adt10::C1)
        }
    } else {
        Ok(Adt10::C1)
    }
}

pub fn f146(v0: u64, v1: Adt1, v2: Adt2, v3: Adt2) -> R<Adt10> {
    let a4831 = match v2.clone() {
        Adt2::C0(h4830, _, _, _, _, _, _, _, _, _, _) => {
            h4830
        }
    };
    let a4832 = 0u64;
    if if if if if nat_eq(a4831, a4832) {
        true
    } else {
        let a4834 = match v2.clone() {
            Adt2::C0(h4833, _, _, _, _, _, _, _, _, _, _) => {
                h4833
            }
        };
        let a4835 = 1u64;
        nat_eq(a4834, a4835)
    } {
        true
    } else {
        let a4837 = match v2.clone() {
            Adt2::C0(h4836, _, _, _, _, _, _, _, _, _, _) => {
                h4836
            }
        };
        let a4838 = 30u64;
        nat_eq(a4837, a4838)
    } {
        let a4841 = match v2.clone() {
            Adt2::C0(h4839, _, _, _, _, _, _, _, _, _, _) => {
                h4839
            }
        };
        let a4842 = match v3.clone() {
            Adt2::C0(h4840, _, _, _, _, _, _, _, _, _, _) => {
                h4840
            }
        };
        nat_eq(a4841, a4842)
    } else {
        false
    } {
        if if f26(v2.clone(), v3.clone()) {
            let a4844 = match v1.clone() {
                Adt1::C0(h4843, _, _, _, _, _, _, _, _, _, _, _) => {
                    h4843
                }
            };
            let a4845 = 4294967296u64;
            nat_lt(a4844, a4845)
        } else {
            false
        } {
            let a4847 = match v1.clone() {
                Adt1::C0(_, h4846, _, _, _, _, _, _, _, _, _, _) => {
                    h4846
                }
            };
            let a4848 = 4294967296u64;
            nat_lt(a4847, a4848)
        } else {
            false
        }
    } else {
        false
    } {
        Ok(Adt10::C4(f28(v3.clone(), 1u64)?, {
            let a4850 = match v2.clone() {
                Adt2::C0(h4849, _, _, _, _, _, _, _, _, _, _) => {
                    h4849
                }
            };
            let a4851 = 0u64;
            if nat_eq(a4850, a4851) {
                4u64
            } else {
                2u64
            }
        }, f30(v2.clone(), 1u64), f30(v3, 1u64), Adt4::C13({
            let a4852 = v0;
            let a4853 = 31u64;
            if nat_eq(a4852, a4853) {
                0u64
            } else {
                let a4854 = v0;
                let a4855 = 47u64;
                if nat_eq(a4854, a4855) {
                    1u64
                } else {
                    2u64
                }
            }
        }, match v2 {
            Adt2::C0(h4856, _, _, _, _, _, _, _, _, _, _) => {
                h4856
            }
        }, match v1.clone() {
            Adt1::C0(h4857, _, _, _, _, _, _, _, _, _, _, _) => {
                h4857
            }
        }, match v1 {
            Adt1::C0(_, h4858, _, _, _, _, _, _, _, _, _, _) => {
                h4858
            }
        })))
    } else {
        Ok(Adt10::C1)
    }
}

pub fn f147(v0: Adt2, v1: Adt2) -> R<Adt10> {
    let a4860 = match v0.clone() {
        Adt2::C0(h4859, _, _, _, _, _, _, _, _, _, _) => {
            h4859
        }
    };
    let a4861 = 0u64;
    if if if nat_eq(a4860, a4861) {
        let a4863 = match v1.clone() {
            Adt2::C0(h4862, _, _, _, _, _, _, _, _, _, _) => {
                h4862
            }
        };
        let a4864 = 0u64;
        nat_eq(a4863, a4864)
    } else {
        false
    } {
        f26(v0.clone(), v1.clone())
    } else {
        false
    } {
        Ok(Adt10::C4(f28(v1.clone(), 1u64)?, 4u64, f30(v0, 1u64), f30(v1, 1u64), Adt4::C15))
    } else {
        Ok(Adt10::C1)
    }
}

pub fn f148(v0: Adt2, v1: Adt2) -> R<Adt10> {
    let a4866 = match v0.clone() {
        Adt2::C0(h4865, _, _, _, _, _, _, _, _, _, _) => {
            h4865
        }
    };
    let a4867 = 0u64;
    if if nat_eq(a4866, a4867) {
        let a4869 = match v1.clone() {
            Adt2::C0(h4868, _, _, _, _, _, _, _, _, _, _) => {
                h4868
            }
        };
        let a4870 = 26u64;
        nat_eq(a4869, a4870)
    } else {
        false
    } {
        let a4872 = match v0.clone() {
            Adt2::C0(_, _, _, h4871, _, _, _, _, _, _, _) => {
                h4871
            }
        };
        let a4873 = 1u64;
        if if if if if if if nat_eq(a4872, a4873) {
            let a4875 = match v0.clone() {
                Adt2::C0(_, _, _, _, h4874, _, _, _, _, _, _) => {
                    h4874
                }
            };
            let a4876 = 1u64;
            nat_eq(a4875, a4876)
        } else {
            false
        } {
            let a4878 = match v0.clone() {
                Adt2::C0(_, h4877, _, _, _, _, _, _, _, _, _) => {
                    h4877
                }
            };
            let a4879 = 2147483647u64;
            nat_le(a4878, a4879)
        } else {
            false
        } {
            let a4882 = match v1.clone() {
                Adt2::C0(_, h4880, _, _, _, _, _, _, _, _, _) => {
                    h4880
                }
            };
            let a4883 = match v0.clone() {
                Adt2::C0(_, _, h4881, _, _, _, _, _, _, _, _) => {
                    h4881
                }
            };
            nat_eq(a4882, a4883)
        } else {
            false
        } {
            let a4885 = match v1.clone() {
                Adt2::C0(_, _, h4884, _, _, _, _, _, _, _, _) => {
                    h4884
                }
            };
            let a4886 = 1u64;
            nat_eq(a4885, a4886)
        } else {
            false
        } {
            let a4888 = match v1.clone() {
                Adt2::C0(_, _, _, h4887, _, _, _, _, _, _, _) => {
                    h4887
                }
            };
            let a4889 = 1u64;
            nat_eq(a4888, a4889)
        } else {
            false
        } {
            let a4891 = match v1.clone() {
                Adt2::C0(_, _, _, _, h4890, _, _, _, _, _, _) => {
                    h4890
                }
            };
            let a4892 = 1u64;
            nat_eq(a4891, a4892)
        } else {
            false
        } {
            Ok(Adt10::C4({
                let a4895 = match v0.clone() {
                    Adt2::C0(_, _, h4893, _, _, _, _, _, _, _, _) => {
                        h4893
                    }
                };
                let a4896 = f191(match v0.clone() {
                    Adt2::C0(_, h4894, _, _, _, _, _, _, _, _, _) => {
                        h4894
                    }
                })?;
                nat_mul(a4895, a4896)?
            }, 4u64, f30(v0, 1u64), f30(v1, 1u64), Adt4::C17))
        } else {
            Ok(Adt10::C1)
        }
    } else {
        Ok(Adt10::C0)
    }
}

pub fn f149(v0: Adt2, v1: Option<Adt2>, v2: Adt2) -> R<Adt10> {
    let m4897 = v1;
    match m4897 {
        None => {
            Ok(Adt10::C1)
        }
        Some(v3) => {
            let v4: u64 = f196(f30(v0.clone(), 1u64))?;
            let a4899 = match v0.clone() {
                Adt2::C0(h4898, _, _, _, _, _, _, _, _, _, _) => {
                    h4898
                }
            };
            let a4900 = 26u64;
            if if if nat_eq(a4899, a4900) {
                let a4902 = match v3.clone() {
                    Adt2::C0(h4901, _, _, _, _, _, _, _, _, _, _) => {
                        h4901
                    }
                };
                let a4903 = 26u64;
                nat_eq(a4902, a4903)
            } else {
                false
            } {
                let a4905 = match v2.clone() {
                    Adt2::C0(h4904, _, _, _, _, _, _, _, _, _, _) => {
                        h4904
                    }
                };
                let a4906 = 27u64;
                nat_eq(a4905, a4906)
            } else {
                false
            } {
                if if if if if if f26(v0.clone(), v3.clone()) {
                    let a4908 = match v2.clone() {
                        Adt2::C0(_, h4907, _, _, _, _, _, _, _, _, _) => {
                            h4907
                        }
                    };
                    let a4909 = 1u64;
                    nat_eq(a4908, a4909)
                } else {
                    false
                } {
                    let a4911 = match v2.clone() {
                        Adt2::C0(_, _, h4910, _, _, _, _, _, _, _, _) => {
                            h4910
                        }
                    };
                    let a4912 = 1u64;
                    nat_eq(a4911, a4912)
                } else {
                    false
                } {
                    let a4914 = match v2.clone() {
                        Adt2::C0(_, _, _, h4913, _, _, _, _, _, _, _) => {
                            h4913
                        }
                    };
                    let a4915 = 1u64;
                    nat_eq(a4914, a4915)
                } else {
                    false
                } {
                    let a4917 = match v2.clone() {
                        Adt2::C0(_, _, _, _, h4916, _, _, _, _, _, _) => {
                            h4916
                        }
                    };
                    let a4918 = 1u64;
                    nat_eq(a4917, a4918)
                } else {
                    false
                } {
                    let a4919 = v4;
                    let a4920 = 9223372036854775808u64;
                    nat_lt(a4919, a4920)
                } else {
                    false
                } {
                    Ok(Adt10::C4({
                        let a4923 = 2u64;
                        let a4924 = {
                            let a4921 = v4;
                            let a4922 = 2u64;
                            nat_mul(a4921, a4922)?
                        };
                        nat_add(a4923, a4924)?
                    }, 8u64, f30(v0, 1u64), f30(v2, 1u64), Adt4::C18(v4, f30(v3, 1u64))))
                } else {
                    Ok(Adt10::C1)
                }
            } else {
                Ok(Adt10::C0)
            }
        }
    }
}

pub fn f150(v0: u64, v1: Adt2, v2: Adt2) -> R<Adt10> {
    let v3: u64 = match v1.clone() {
        Adt2::C0(h4925, _, _, _, _, _, _, _, _, _, _) => {
            h4925
        }
    };
    let a4926 = v3;
    let a4927 = 0u64;
    if if nat_eq(a4926, a4927) {
        true
    } else {
        let a4928 = v0;
        let a4929 = 13u64;
        if nat_eq(a4928, a4929) {
            let a4930 = v3;
            let a4931 = 1u64;
            if nat_eq(a4930, a4931) {
                true
            } else {
                let a4932 = v3;
                let a4933 = 30u64;
                nat_eq(a4932, a4933)
            }
        } else {
            false
        }
    } {
        let a4935 = v3;
        let a4936 = match v2.clone() {
            Adt2::C0(h4934, _, _, _, _, _, _, _, _, _, _) => {
                h4934
            }
        };
        if if nat_eq(a4935, a4936) {
            let a4937 = v0;
            let a4938 = 13u64;
            if nat_eq(a4937, a4938) {
                let a4940 = match v2.clone() {
                    Adt2::C0(_, h4939, _, _, _, _, _, _, _, _, _) => {
                        h4939
                    }
                };
                let a4941 = 1u64;
                if if if nat_eq(a4940, a4941) {
                    let a4943 = match v2.clone() {
                        Adt2::C0(_, _, h4942, _, _, _, _, _, _, _, _) => {
                            h4942
                        }
                    };
                    let a4944 = 1u64;
                    nat_eq(a4943, a4944)
                } else {
                    false
                } {
                    let a4946 = match v2.clone() {
                        Adt2::C0(_, _, _, h4945, _, _, _, _, _, _, _) => {
                            h4945
                        }
                    };
                    let a4947 = 1u64;
                    nat_eq(a4946, a4947)
                } else {
                    false
                } {
                    let a4949 = match v2.clone() {
                        Adt2::C0(_, _, _, _, h4948, _, _, _, _, _, _) => {
                            h4948
                        }
                    };
                    let a4950 = 1u64;
                    nat_eq(a4949, a4950)
                } else {
                    false
                }
            } else {
                let a4952 = match v2.clone() {
                    Adt2::C0(_, h4951, _, _, _, _, _, _, _, _, _) => {
                        h4951
                    }
                };
                let a4953 = 1u64;
                if if if nat_eq(a4952, a4953) {
                    let a4956 = match v1.clone() {
                        Adt2::C0(_, _, h4954, _, _, _, _, _, _, _, _) => {
                            h4954
                        }
                    };
                    let a4957 = match v2.clone() {
                        Adt2::C0(_, _, h4955, _, _, _, _, _, _, _, _) => {
                            h4955
                        }
                    };
                    nat_eq(a4956, a4957)
                } else {
                    false
                } {
                    let a4960 = match v1.clone() {
                        Adt2::C0(_, _, _, h4958, _, _, _, _, _, _, _) => {
                            h4958
                        }
                    };
                    let a4961 = match v2.clone() {
                        Adt2::C0(_, _, _, h4959, _, _, _, _, _, _, _) => {
                            h4959
                        }
                    };
                    nat_eq(a4960, a4961)
                } else {
                    false
                } {
                    let a4964 = match v1.clone() {
                        Adt2::C0(_, _, _, _, h4962, _, _, _, _, _, _) => {
                            h4962
                        }
                    };
                    let a4965 = match v2.clone() {
                        Adt2::C0(_, _, _, _, h4963, _, _, _, _, _, _) => {
                            h4963
                        }
                    };
                    nat_eq(a4964, a4965)
                } else {
                    false
                }
            }
        } else {
            false
        } {
            let v4: Adt3 = f30(v1, 1u64);
            Ok(Adt10::C4(f250(v4.clone(), v0, v3)?, {
                let a4966 = v3;
                let a4967 = 0u64;
                if nat_eq(a4966, a4967) {
                    4u64
                } else {
                    2u64
                }
            }, v4, f30(v2, 1u64), Adt4::C16(v0, v3)))
        } else {
            Ok(Adt10::C1)
        }
    } else {
        Ok(Adt10::C0)
    }
}

pub fn f151(v0: u64, v1: u64) -> R<Option<u64>> {
    let a4968 = v0;
    let a4969 = 89u64;
    if if nat_eq(a4968, a4969) {
        let a4970 = v1;
        let a4971 = 5u64;
        nat_eq(a4970, a4971)
    } else {
        false
    } {
        Ok(Some(25u64))
    } else {
        let a4972 = v0;
        let a4973 = 89u64;
        if if nat_eq(a4972, a4973) {
            let a4974 = v1;
            let a4975 = 17u64;
            nat_eq(a4974, a4975)
        } else {
            false
        } {
            Ok(Some(26u64))
        } else {
            let a4976 = v0;
            let a4977 = 89u64;
            if if nat_eq(a4976, a4977) {
                let a4978 = v1;
                let a4979 = 16u64;
                nat_eq(a4978, a4979)
            } else {
                false
            } {
                Ok(Some(24u64))
            } else {
                let a4980 = v0;
                let a4981 = 89u64;
                if if nat_eq(a4980, a4981) {
                    let a4982 = v1;
                    let a4983 = 14u64;
                    nat_eq(a4982, a4983)
                } else {
                    false
                } {
                    Ok(Some(23u64))
                } else {
                    let a4984 = v0;
                    let a4985 = 89u64;
                    if if nat_eq(a4984, a4985) {
                        let a4986 = v1;
                        let a4987 = 15u64;
                        nat_eq(a4986, a4987)
                    } else {
                        false
                    } {
                        Ok(Some(22u64))
                    } else {
                        let a4988 = v0;
                        let a4989 = 89u64;
                        if if nat_eq(a4988, a4989) {
                            let a4990 = 8u64;
                            let a4991 = v1;
                            if nat_le(a4990, a4991) {
                                let a4992 = v1;
                                let a4993 = 9u64;
                                nat_le(a4992, a4993)
                            } else {
                                false
                            }
                        } else {
                            false
                        } {
                            Ok(Some({
                                let a4994 = v1;
                                let a4995 = 12u64;
                                nat_add(a4994, a4995)?
                            }))
                        } else {
                            let a4996 = v0;
                            let a4997 = 89u64;
                            if if nat_eq(a4996, a4997) {
                                let a4998 = v1;
                                let a4999 = 4u64;
                                nat_eq(a4998, a4999)
                            } else {
                                false
                            } {
                                Ok(Some(19u64))
                            } else {
                                let a5000 = 10u64;
                                let a5001 = v0;
                                if if nat_le(a5000, a5001) {
                                    let a5002 = v0;
                                    let a5003 = 12u64;
                                    nat_le(a5002, a5003)
                                } else {
                                    false
                                } {
                                    Ok(Some({
                                        let a5004 = v0;
                                        let a5005 = 3u64;
                                        nat_add(a5004, a5005)?
                                    }))
                                } else {
                                    let a5006 = v0;
                                    let a5007 = 89u64;
                                    if if nat_eq(a5006, a5007) {
                                        let a5008 = v1;
                                        let a5009 = 7u64;
                                        nat_eq(a5008, a5009)
                                    } else {
                                        false
                                    } {
                                        Ok(Some(17u64))
                                    } else {
                                        let a5010 = v0;
                                        let a5011 = 89u64;
                                        if if nat_eq(a5010, a5011) {
                                            let a5012 = v1;
                                            let a5013 = 10u64;
                                            nat_eq(a5012, a5013)
                                        } else {
                                            false
                                        } {
                                            Ok(Some(18u64))
                                        } else {
                                            let a5014 = v0;
                                            let a5015 = 89u64;
                                            if if nat_eq(a5014, a5015) {
                                                let a5016 = v1;
                                                let a5017 = 13u64;
                                                nat_eq(a5016, a5017)
                                            } else {
                                                false
                                            } {
                                                Ok(Some(16u64))
                                            } else {
                                                let a5018 = v0;
                                                let a5019 = 8u64;
                                                if nat_eq(a5018, a5019) {
                                                    Ok(Some(5u64))
                                                } else {
                                                    let a5020 = v0;
                                                    let a5021 = 9u64;
                                                    if nat_eq(a5020, a5021) {
                                                        Ok(Some(6u64))
                                                    } else {
                                                        let a5022 = v0;
                                                        let a5023 = 89u64;
                                                        if nat_eq(a5022, a5023) {
                                                            let a5024 = v1;
                                                            let a5025 = 0u64;
                                                            if nat_eq(a5024, a5025) {
                                                                Ok(Some(0u64))
                                                            } else {
                                                                let a5026 = v1;
                                                                let a5027 = 1u64;
                                                                if nat_eq(a5026, a5027) {
                                                                    Ok(Some(1u64))
                                                                } else {
                                                                    let a5028 = v1;
                                                                    let a5029 = 2u64;
                                                                    if nat_eq(a5028, a5029) {
                                                                        Ok(Some(2u64))
                                                                    } else {
                                                                        let a5030 = v1;
                                                                        let a5031 = 3u64;
                                                                        if nat_eq(a5030, a5031) {
                                                                            Ok(Some(3u64))
                                                                        } else {
                                                                            let a5032 = v1;
                                                                            let a5033 = 6u64;
                                                                            if nat_eq(a5032, a5033) {
                                                                                Ok(Some(4u64))
                                                                            } else {
                                                                                let a5034 = v1;
                                                                                let a5035 = 12u64;
                                                                                if nat_eq(a5034, a5035) {
                                                                                    Ok(Some(7u64))
                                                                                } else {
                                                                                    let a5036 = v1;
                                                                                    let a5037 = 11u64;
                                                                                    if nat_eq(a5036, a5037) {
                                                                                        Ok(Some(8u64))
                                                                                    } else {
                                                                                        let a5038 = v1;
                                                                                        let a5039 = 18u64;
                                                                                        if nat_eq(a5038, a5039) {
                                                                                            Ok(Some(9u64))
                                                                                        } else {
                                                                                            let a5040 = v1;
                                                                                            let a5041 = 19u64;
                                                                                            if nat_eq(a5040, a5041) {
                                                                                                Ok(Some(10u64))
                                                                                            } else {
                                                                                                let a5042 = v1;
                                                                                                let a5043 = 20u64;
                                                                                                if nat_eq(a5042, a5043) {
                                                                                                    Ok(Some(11u64))
                                                                                                } else {
                                                                                                    let a5044 = v1;
                                                                                                    let a5045 = 21u64;
                                                                                                    if nat_eq(a5044, a5045) {
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

pub fn f152(v0: u64, v1: u64, v2: Adt2, v3: Option<Adt2>, v4: Adt2) -> R<Adt10> {
    let m5046 = f251(v0, v1, v2.clone(), v3.clone(), v4.clone())?;
    match m5046 {
        Adt10::C0 => {
            Ok(Adt10::C0)
        }
        Adt10::C1 => {
            let a5047 = v0;
            let a5048 = 2u64;
            if if if if nat_eq(a5047, a5048) {
                true
            } else {
                let a5049 = v0;
                let a5050 = 5u64;
                nat_eq(a5049, a5050)
            } {
                true
            } else {
                let a5051 = v0;
                let a5052 = 6u64;
                nat_eq(a5051, a5052)
            } {
                true
            } else {
                let a5053 = v0;
                let a5054 = 7u64;
                nat_eq(a5053, a5054)
            } {
                Ok(Adt10::C5(67u64))
            } else {
                Ok(Adt10::C1)
            }
        }
        Adt10::C2(v7, v8, _, _, _, _, v13) => {
            if v8 {
                Ok(Adt10::C4(v13, 4u64, f30(v2.clone(), 1u64), f30(v4, 1u64), Adt4::C10(v7, v8, {
                    let m5055 = v3;
                    match m5055 {
                        None => {
                            f30(v2, 1u64)
                        }
                        Some(v14) => {
                            f30(v14, 1u64)
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

pub fn f153(v0: u64, v1: Adt1, v2: Adt2, v3: Adt2) -> R<Adt10> {
    let a5057 = match v2.clone() {
        Adt2::C0(h5056, _, _, _, _, _, _, _, _, _, _) => {
            h5056
        }
    };
    let a5058 = 0u64;
    if if if if if nat_eq(a5057, a5058) {
        true
    } else {
        let a5060 = match v2.clone() {
            Adt2::C0(h5059, _, _, _, _, _, _, _, _, _, _) => {
                h5059
            }
        };
        let a5061 = 1u64;
        nat_eq(a5060, a5061)
    } {
        true
    } else {
        let a5063 = match v2.clone() {
            Adt2::C0(h5062, _, _, _, _, _, _, _, _, _, _) => {
                h5062
            }
        };
        let a5064 = 30u64;
        nat_eq(a5063, a5064)
    } {
        let a5067 = match v2.clone() {
            Adt2::C0(h5065, _, _, _, _, _, _, _, _, _, _) => {
                h5065
            }
        };
        let a5068 = match v3.clone() {
            Adt2::C0(h5066, _, _, _, _, _, _, _, _, _, _) => {
                h5066
            }
        };
        if nat_eq(a5067, a5068) {
            true
        } else {
            let a5070 = match v3.clone() {
                Adt2::C0(h5069, _, _, _, _, _, _, _, _, _, _) => {
                    h5069
                }
            };
            let a5071 = 0u64;
            nat_eq(a5070, a5071)
        }
    } else {
        false
    } {
        f26(v2.clone(), v3.clone())
    } else {
        false
    } {
        Ok(Adt10::C4({
            let a5078 = f28(v3.clone(), 1u64)?;
            let a5079 = {
                let a5072 = 25u64;
                let a5073 = v0;
                if nat_le(a5072, a5073) {
                    f187({
                        let a5074 = v0;
                        let a5075 = 26u64;
                        nat_eq(a5074, a5075)
                    }, match v3.clone() {
                        Adt2::C0(h5076, _, _, _, _, _, _, _, _, _, _) => {
                            h5076
                        }
                    })?
                } else {
                    f252(v0, match v3.clone() {
                        Adt2::C0(h5077, _, _, _, _, _, _, _, _, _, _) => {
                            h5077
                        }
                    })
                }
            };
            nat_mul(a5078, a5079)?
        }, {
            let a5081 = match v3.clone() {
                Adt2::C0(h5080, _, _, _, _, _, _, _, _, _, _) => {
                    h5080
                }
            };
            let a5082 = 0u64;
            if nat_eq(a5081, a5082) {
                4u64
            } else {
                2u64
            }
        }, f30(v2.clone(), 1u64), f30(v3.clone(), 1u64), {
            let a5083 = 25u64;
            let a5084 = v0;
            if nat_le(a5083, a5084) {
                Adt4::C45({
                    let a5085 = v0;
                    let a5086 = 26u64;
                    nat_eq(a5085, a5086)
                }, match v2 {
                    Adt2::C0(h5087, _, _, _, _, _, _, _, _, _, _) => {
                        h5087
                    }
                }, match v3 {
                    Adt2::C0(h5088, _, _, _, _, _, _, _, _, _, _) => {
                        h5088
                    }
                }, v1)
            } else {
                Adt4::C14(v0, match v2 {
                    Adt2::C0(h5089, _, _, _, _, _, _, _, _, _, _) => {
                        h5089
                    }
                }, match v3 {
                    Adt2::C0(h5090, _, _, _, _, _, _, _, _, _, _) => {
                        h5090
                    }
                })
            }
        }))
    } else {
        Ok(Adt10::C1)
    }
}

pub fn f154(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64, v7: u64) -> R<u64> {
    let a5103 = {
        let a5099 = {
            let a5095 = {
                let a5091 = v0;
                let a5092 = v4;
                nat_mul(a5091, a5092)?
            };
            let a5096 = {
                let a5093 = v1;
                let a5094 = v5;
                nat_mul(a5093, a5094)?
            };
            nat_add(a5095, a5096)?
        };
        let a5100 = {
            let a5097 = v2;
            let a5098 = v6;
            nat_mul(a5097, a5098)?
        };
        nat_add(a5099, a5100)?
    };
    let a5104 = {
        let a5101 = v3;
        let a5102 = v7;
        nat_mul(a5101, a5102)?
    };
    nat_add(a5103, a5104)
}

pub fn f155(v0: u64) -> u64 {
    let a5105 = v0;
    let a5106 = 2147483648u64;
    if nat_lt(a5105, a5106) {
        0u64
    } else {
        let a5107 = 4294967296u64;
        let a5108 = v0;
        nat_sub(a5107, a5108)
    }
}

pub fn f156(v0: u64) -> u64 {
    let a5109 = v0;
    let a5110 = 2147483648u64;
    if nat_lt(a5109, a5110) {
        v0
    } else {
        0u64
    }
}

pub fn f157(v0: Adt11, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64, v7: u64, v8: u64) -> Adt9 {
    let v9: u64 = {
        let a5111 = v7;
        let a5112 = 6u64;
        if nat_eq(a5111, a5112) {
            27u64
        } else {
            v2
        }
    };
    Adt9::C7(v0, f99(v3, v4), f99(v5, v6), f100(v7, v8), f188(v1), f188(v9))
}

pub fn f158(v0: Adt4, v1: u64, v2: u64) -> Adt11 {
    let m5113 = v0;
    match m5113 {
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
        Adt4::C13(v66, v67, v68, v69) => {
            Adt11::C4(v66, v67, v68, v69)
        }
        Adt4::C14(v70, _, _) => {
            let a5114 = 13u64;
            let a5115 = v70;
            if nat_le(a5114, a5115) {
                Adt11::C7(v70, v1, v2)
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
        Adt4::C27(v90) => {
            Adt11::C2(match v90.clone() {
                Adt5::C0(_, _, _, _, _, h5116, _, _) => {
                    h5116
                }
            }, match v90 {
                Adt5::C0(_, _, _, _, _, _, _, h5117) => {
                    h5117
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
        Adt4::C47(_, _) => {
            Adt11::C6
        }
    }
}

pub fn f159(v0: Adt4) -> u64 {
    let m5118 = v0;
    match m5118 {
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
        Adt4::C14(v68, _, _) => {
            v68
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
        Adt4::C47(_, _) => {
            0u64
        }
    }
}

pub fn f160(v0: Adt4, v1: u64) -> u64 {
    let m5119 = v0;
    match m5119 {
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
        Adt4::C16(_, v73) => {
            v73
        }
        Adt4::C17 => {
            0u64
        }
        Adt4::C18(_, _) => {
            0u64
        }
        Adt4::C19(v76, _) => {
            let a5120 = v1;
            let a5121 = 6u64;
            if nat_eq(a5120, a5121) {
                v76
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
        Adt4::C26(v90, _, _, _) => {
            let a5130 = v1;
            let a5131 = 6u64;
            if if if nat_eq(a5130, a5131) {
                true
            } else {
                let a5132 = v1;
                let a5133 = 7u64;
                nat_eq(a5132, a5133)
            } {
                let a5134 = v90;
                let a5135 = 30u64;
                nat_eq(a5134, a5135)
            } else {
                false
            } {
                30u64
            } else {
                0u64
            }
        }
        Adt4::C27(v89) => {
            match v89 {
                Adt5::C0(_, _, _, _, _, _, _, h5129) => {
                    h5129
                }
            }
        }
        Adt4::C28(v85) => {
            let a5122 = v1;
            let a5123 = 6u64;
            if nat_eq(a5122, a5123) {
                match v85 {
                    Adt7::C0(_, _, _, _, h5124) => {
                        h5124
                    }
                }
            } else {
                0u64
            }
        }
        Adt4::C29(_) => {
            0u64
        }
        Adt4::C30(_, v88) => {
            let a5125 = v1;
            let a5126 = 6u64;
            if if nat_eq(a5125, a5126) {
                true
            } else {
                let a5127 = v1;
                let a5128 = 7u64;
                nat_eq(a5127, a5128)
            } {
                v88
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
        Adt4::C47(_, _) => {
            0u64
        }
    }
}

pub fn f161(v0: Adt4, v1: u64) -> Adt11 {
    let a5136 = v1;
    let a5137 = 4u64;
    if nat_lt(a5136, a5137) {
        Adt11::C0(v1)
    } else {
        let a5138 = v1;
        let a5139 = 4u64;
        if nat_eq(a5138, a5139) {
            Adt11::C1(0u64)
        } else {
            let a5140 = v1;
            let a5141 = 5u64;
            if nat_eq(a5140, a5141) {
                Adt11::C1(3u64)
            } else {
                let a5142 = v1;
                let a5143 = 6u64;
                if nat_eq(a5142, a5143) {
                    Adt11::C2(f160(v0, v1), 0u64, false)
                } else {
                    let a5144 = v1;
                    let a5145 = 7u64;
                    if nat_eq(a5144, a5145) {
                        Adt11::C2(0u64, f160(v0, v1), false)
                    } else {
                        let a5146 = v1;
                        let a5147 = 16u64;
                        if nat_lt(a5146, a5147) {
                            Adt11::C1({
                                let a5148 = v1;
                                let a5149 = 8u64;
                                nat_sub(a5148, a5149)
                            })
                        } else {
                            let a5150 = v1;
                            let a5151 = 20u64;
                            if nat_lt(a5150, a5151) {
                                Adt11::C5({
                                    let a5152 = v1;
                                    let a5153 = 16u64;
                                    nat_sub(a5152, a5153)
                                })
                            } else {
                                let a5154 = v1;
                                let a5155 = 20u64;
                                if nat_eq(a5154, a5155) {
                                    Adt11::C3(0u64, 0u64, false)
                                } else {
                                    let a5156 = v1;
                                    let a5157 = 21u64;
                                    if nat_eq(a5156, a5157) {
                                        Adt11::C0(4u64)
                                    } else {
                                        let a5158 = v1;
                                        let a5159 = 22u64;
                                        if nat_eq(a5158, a5159) {
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

pub fn f162(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let a5177 = {
        let a5172 = f253(v2, {
            let a5163 = {
                let a5160 = v0;
                let a5161 = v1;
                let a5162 = 0u64;
                nat_quot(a5160, a5161, a5162)
            };
            let a5164 = v2;
            let a5165 = 0u64;
            nat_rem(a5163, a5164, a5165)
        }, {
            let a5169 = {
                let a5166 = v0;
                let a5167 = v1;
                let a5168 = 0u64;
                nat_quot(a5166, a5167, a5168)
            };
            let a5170 = v2;
            let a5171 = 0u64;
            nat_quot(a5169, a5170, a5171)
        })?;
        let a5173 = v1;
        nat_mul(a5172, a5173)?
    };
    let a5178 = {
        let a5174 = v0;
        let a5175 = v1;
        let a5176 = 0u64;
        nat_rem(a5174, a5175, a5176)
    };
    nat_add(a5177, a5178)
}

pub fn f163(v0: u64, v1: Adt3, v2: Adt3) -> R<u64> {
    let v3: u64 = {
        let a5180 = v0;
        let a5181 = match v2.clone() {
            Adt3::C0(h5179, _, _, _, _, _, _, _) => {
                h5179
            }
        };
        let a5182 = 0u64;
        nat_quot(a5180, a5181, a5182)
    };
    let v4: u64 = {
        let a5184 = v3;
        let a5185 = match v2.clone() {
            Adt3::C0(_, h5183, _, _, _, _, _, _) => {
                h5183
            }
        };
        let a5186 = 0u64;
        nat_quot(a5184, a5185, a5186)
    };
    f254({
        let a5188 = v0;
        let a5189 = match v2.clone() {
            Adt3::C0(h5187, _, _, _, _, _, _, _) => {
                h5187
            }
        };
        let a5190 = 0u64;
        nat_rem(a5188, a5189, a5190)
    }, {
        let a5192 = v3;
        let a5193 = match v2.clone() {
            Adt3::C0(_, h5191, _, _, _, _, _, _) => {
                h5191
            }
        };
        let a5194 = 0u64;
        nat_rem(a5192, a5193, a5194)
    }, {
        let a5196 = v4;
        let a5197 = match v2.clone() {
            Adt3::C0(_, _, h5195, _, _, _, _, _) => {
                h5195
            }
        };
        let a5198 = 0u64;
        nat_rem(a5196, a5197, a5198)
    }, {
        let a5200 = v4;
        let a5201 = match v2 {
            Adt3::C0(_, _, h5199, _, _, _, _, _) => {
                h5199
            }
        };
        let a5202 = 0u64;
        nat_quot(a5200, a5201, a5202)
    }, match v1.clone() {
        Adt3::C0(h5203, _, _, _, _, _, _, _) => {
            h5203
        }
    }, match v1.clone() {
        Adt3::C0(_, h5204, _, _, _, _, _, _) => {
            h5204
        }
    }, match v1.clone() {
        Adt3::C0(_, _, h5205, _, _, _, _, _) => {
            h5205
        }
    }, match v1.clone() {
        Adt3::C0(_, _, _, h5206, _, _, _, _) => {
            h5206
        }
    }, match v1.clone() {
        Adt3::C0(_, _, _, _, h5207, _, _, _) => {
            h5207
        }
    }, match v1.clone() {
        Adt3::C0(_, _, _, _, _, h5208, _, _) => {
            h5208
        }
    }, match v1.clone() {
        Adt3::C0(_, _, _, _, _, _, h5209, _) => {
            h5209
        }
    }, match v1 {
        Adt3::C0(_, _, _, _, _, _, _, h5210) => {
            h5210
        }
    })
}

pub fn f164(v0: u64, v1: Adt3, v2: Adt3, v3: Adt3, v4: u64) -> R<Adt9> {
    let v5: u64 = {
        let a5212 = v0;
        let a5213 = match v3.clone() {
            Adt3::C0(h5211, _, _, _, _, _, _, _) => {
                h5211
            }
        };
        let a5214 = 0u64;
        nat_quot(a5212, a5213, a5214)
    };
    let v6: u64 = {
        let a5216 = v5;
        let a5217 = match v3.clone() {
            Adt3::C0(_, h5215, _, _, _, _, _, _) => {
                h5215
            }
        };
        let a5218 = 0u64;
        nat_quot(a5216, a5217, a5218)
    };
    let v7: u64 = {
        let a5220 = v0;
        let a5221 = match v3.clone() {
            Adt3::C0(h5219, _, _, _, _, _, _, _) => {
                h5219
            }
        };
        let a5222 = 0u64;
        nat_rem(a5220, a5221, a5222)
    };
    let v8: u64 = {
        let a5224 = v5;
        let a5225 = match v3.clone() {
            Adt3::C0(_, h5223, _, _, _, _, _, _) => {
                h5223
            }
        };
        let a5226 = 0u64;
        nat_rem(a5224, a5225, a5226)
    };
    let v9: u64 = {
        let a5228 = v6;
        let a5229 = match v3.clone() {
            Adt3::C0(_, _, h5227, _, _, _, _, _) => {
                h5227
            }
        };
        let a5230 = 0u64;
        nat_rem(a5228, a5229, a5230)
    };
    let v10: u64 = {
        let a5232 = v6;
        let a5233 = match v3.clone() {
            Adt3::C0(_, _, h5231, _, _, _, _, _) => {
                h5231
            }
        };
        let a5234 = 0u64;
        nat_quot(a5232, a5233, a5234)
    };
    let v11: u64 = {
        let a5235 = v4;
        let a5236 = 0u64;
        if nat_eq(a5235, a5236) {
            match v1.clone() {
                Adt3::C0(h5237, _, _, _, _, _, _, _) => {
                    h5237
                }
            }
        } else {
            let a5238 = v4;
            let a5239 = 1u64;
            if nat_eq(a5238, a5239) {
                match v1.clone() {
                    Adt3::C0(_, h5240, _, _, _, _, _, _) => {
                        h5240
                    }
                }
            } else {
                let a5241 = v4;
                let a5242 = 2u64;
                if nat_eq(a5241, a5242) {
                    match v1.clone() {
                        Adt3::C0(_, _, h5243, _, _, _, _, _) => {
                            h5243
                        }
                    }
                } else {
                    match v1.clone() {
                        Adt3::C0(_, _, _, h5244, _, _, _, _) => {
                            h5244
                        }
                    }
                }
            }
        }
    };
    let v12: bool = {
        let a5251 = v11;
        let a5252 = {
            let a5245 = v4;
            let a5246 = 0u64;
            if nat_eq(a5245, a5246) {
                v7
            } else {
                let a5247 = v4;
                let a5248 = 1u64;
                if nat_eq(a5247, a5248) {
                    v8
                } else {
                    let a5249 = v4;
                    let a5250 = 2u64;
                    if nat_eq(a5249, a5250) {
                        v9
                    } else {
                        v10
                    }
                }
            }
        };
        nat_le(a5251, a5252)
    };
    let v13: Adt3 = if v12 {
        v2
    } else {
        v1
    };
    Ok(Adt9::C1(v12, f154(if if v12 {
        let a5253 = v4;
        let a5254 = 0u64;
        nat_eq(a5253, a5254)
    } else {
        false
    } {
        let a5255 = v7;
        let a5256 = v11;
        nat_sub(a5255, a5256)
    } else {
        v7
    }, if if v12 {
        let a5257 = v4;
        let a5258 = 1u64;
        nat_eq(a5257, a5258)
    } else {
        false
    } {
        let a5259 = v8;
        let a5260 = v11;
        nat_sub(a5259, a5260)
    } else {
        v8
    }, if if v12 {
        let a5261 = v4;
        let a5262 = 2u64;
        nat_eq(a5261, a5262)
    } else {
        false
    } {
        let a5263 = v9;
        let a5264 = v11;
        nat_sub(a5263, a5264)
    } else {
        v9
    }, if if v12 {
        let a5265 = v4;
        let a5266 = 3u64;
        nat_eq(a5265, a5266)
    } else {
        false
    } {
        let a5267 = v10;
        let a5268 = v11;
        nat_sub(a5267, a5268)
    } else {
        v10
    }, match v13.clone() {
        Adt3::C0(_, _, _, _, h5269, _, _, _) => {
            h5269
        }
    }, match v13.clone() {
        Adt3::C0(_, _, _, _, _, h5270, _, _) => {
            h5270
        }
    }, match v13.clone() {
        Adt3::C0(_, _, _, _, _, _, h5271, _) => {
            h5271
        }
    }, match v13 {
        Adt3::C0(_, _, _, _, _, _, _, h5272) => {
            h5272
        }
    })?, f103(v0, v3)?))
}

pub fn f165(v0: u64, v1: Adt3, v2: Adt3, v3: u64, v4: u64, v5: u64, v6: u64, v7: bool) -> R<Adt9> {
    let v8: u64 = f63(v0, v2.clone(), 0u64);
    let v9: u64 = f63(v0, v2.clone(), 1u64);
    let v10: u64 = f63(v0, v2.clone(), 2u64);
    let v11: u64 = f63(v0, v2.clone(), 3u64);
    if v7 {
        Ok(Adt9::C1(false, f62(v1.clone(), f255(v8, v3, match v1.clone() {
            Adt3::C0(h5273, _, _, _, _, _, _, _) => {
                h5273
            }
        })?, f255(v9, v4, match v1.clone() {
            Adt3::C0(_, h5274, _, _, _, _, _, _) => {
                h5274
            }
        })?, f255(v10, v5, match v1.clone() {
            Adt3::C0(_, _, h5275, _, _, _, _, _) => {
                h5275
            }
        })?, f255(v11, v6, match v1 {
            Adt3::C0(_, _, _, h5276, _, _, _, _) => {
                h5276
            }
        })?)?, f103(v0, v2)?))
    } else {
        if if if if f256(v8, v3, match v1.clone() {
            Adt3::C0(h5277, _, _, _, _, _, _, _) => {
                h5277
            }
        })? {
            f256(v9, v4, match v1.clone() {
                Adt3::C0(_, h5278, _, _, _, _, _, _) => {
                    h5278
                }
            })?
        } else {
            false
        } {
            f256(v10, v5, match v1.clone() {
                Adt3::C0(_, _, h5279, _, _, _, _, _) => {
                    h5279
                }
            })?
        } else {
            false
        } {
            f256(v11, v6, match v1.clone() {
                Adt3::C0(_, _, _, h5280, _, _, _, _) => {
                    h5280
                }
            })?
        } else {
            false
        } {
            Ok(Adt9::C1(false, f62(v1, f257(v8, v3)?, f257(v9, v4)?, f257(v10, v5)?, f257(v11, v6)?)?, f103(v0, v2)?))
        } else {
            Ok(Adt9::C1(true, 0u64, f103(v0, v2)?))
        }
    }
}

pub fn f166(v0: u64, v1: Adt3, v2: Adt3, v3: u64) -> R<Adt9> {
    let v4: u64 = f63(v0, v2.clone(), 0u64);
    let v5: u64 = f63(v0, v2.clone(), 1u64);
    let v6: u64 = f63(v0, v2.clone(), 2u64);
    let v7: u64 = f63(v0, v2.clone(), 3u64);
    Ok(Adt9::C1(false, f62(v1.clone(), f258(v4, v3, match v1 {
        Adt3::C0(h5281, _, _, _, _, _, _, _) => {
            h5281
        }
    })?, v5, v6, v7)?, f103(v0, v2)?))
}

pub fn f167(v0: u64, v1: Adt3, v2: Adt3, v3: u64, v4: u64, v5: bool) -> R<Adt9> {
    let a5282 = v0;
    let a5283 = v3;
    if nat_lt(a5282, a5283) {
        Ok(Adt9::C1(false, {
            let a5284 = v0;
            let a5285 = 4u64;
            nat_mul(a5284, a5285)?
        }, {
            let a5286 = v0;
            let a5287 = 4u64;
            nat_mul(a5286, a5287)?
        }))
    } else {
        let v6: u64 = {
            let a5288 = v0;
            let a5289 = v3;
            nat_sub(a5288, a5289)
        };
        let v7: u64 = f103(v6, v1)?;
        let v8: u64 = f259(v6, v2, v4)?;
        if v5 {
            Ok(Adt9::C4(8u64, 0u64, v8, 4u64, v7, 3u64, v8))
        } else {
            Ok(Adt9::C1(true, v7, v8))
        }
    }
}

pub fn f168(v0: u64, v1: Adt3, v2: Adt3, v3: u64) -> R<Adt9> {
    let a5290 = v0;
    let a5291 = v3;
    if nat_lt(a5290, a5291) {
        Ok(Adt9::C1(true, 0u64, f103(v0, v2)?))
    } else {
        let v4: u64 = {
            let a5292 = v0;
            let a5293 = v3;
            nat_sub(a5292, a5293)
        };
        let v5: u64 = f163(v4, v2, v1.clone())?;
        Ok(Adt9::C4(8u64, 3u64, v5, 0u64, f103(v4, v1)?, 3u64, v5))
    }
}

pub fn f169(v0: u64, v1: u64, v2: Adt3, v3: Adt3) -> R<Adt9> {
    let v4: u64 = {
        let a5295 = v1;
        let a5296 = match v3.clone() {
            Adt3::C0(h5294, _, _, _, _, _, _, _) => {
                h5294
            }
        };
        let a5297 = 0u64;
        nat_rem(a5295, a5296, a5297)
    };
    let v5: u64 = {
        let a5303 = {
            let a5299 = v1;
            let a5300 = match v3.clone() {
                Adt3::C0(h5298, _, _, _, _, _, _, _) => {
                    h5298
                }
            };
            let a5301 = 0u64;
            nat_quot(a5299, a5300, a5301)
        };
        let a5304 = match v3.clone() {
            Adt3::C0(_, h5302, _, _, _, _, _, _) => {
                h5302
            }
        };
        let a5305 = 0u64;
        nat_rem(a5303, a5304, a5305)
    };
    let v6: bool = f260(v0, v4, v5);
    Ok(Adt9::C1(!v6, if v6 {
        f163(v1, v2, v3.clone())?
    } else {
        0u64
    }, f103(v1, v3)?))
}

pub fn f170(v0: u64, v1: Adt3, v2: Adt3, v3: u64, v4: bool, v5: Adt3) -> R<Adt9> {
    Ok(Adt9::C4({
        let a5306 = v3;
        let a5307 = 8u64;
        nat_add(a5306, a5307)?
    }, 0u64, f103(v0, v1)?, if v4 {
        5u64
    } else {
        4u64
    }, if v4 {
        0u64
    } else {
        f163(v0, v5, v2.clone())?
    }, 3u64, f103(v0, v2)?))
}

pub fn f171(v0: u64, v1: Adt3, v2: Adt3, v3: u64, v4: u64) -> R<Adt9> {
    let a5308 = v4;
    let a5309 = 0u64;
    if if nat_eq(a5308, a5309) {
        f135(v3)
    } else {
        false
    } {
        let m5310 = f230(v3);
        match m5310 {
            Adt18::C0 => {
                Ok(Adt9::C0(Adt10::C1))
            }
            Adt18::C1(v5) => {
                let v6: u64 = match v5.clone() {
                    Adt19::C0(_, h5311, _) => {
                        h5311
                    }
                };
                Ok(Adt9::C5(v3, f103(v0, v1)?, match v5 {
                    Adt19::C0(_, _, h5312) => {
                        h5312
                    }
                }, f103({
                    let a5313 = v0;
                    let a5314 = v6;
                    nat_mul(a5313, a5314)?
                }, v2.clone())?, {
                    let a5320 = {
                        let a5318 = {
                            let a5315 = v6;
                            let a5316 = 1u64;
                            nat_sub(a5315, a5316)
                        };
                        let a5319 = match v2.clone() {
                            Adt3::C0(_, _, _, _, h5317, _, _, _) => {
                                h5317
                            }
                        };
                        nat_mul(a5318, a5319)?
                    };
                    let a5321 = 4u64;
                    nat_add(a5320, a5321)?
                }, match v2 {
                    Adt3::C0(_, _, _, _, h5322, _, _, _) => {
                        h5322
                    }
                }))
            }
        }
    } else {
        Ok(Adt9::C3(v3, v4, f103(v0, v1)?, f103(v0, v2)?))
    }
}

pub fn f172(v0: u64, v1: Adt3, v2: Adt3, v3: bool, v4: u64, v5: u64) -> R<Adt9> {
    let v6: u64 = {
        let a5324 = v0;
        let a5325 = match v2.clone() {
            Adt3::C0(h5323, _, _, _, _, _, _, _) => {
                h5323
            }
        };
        let a5326 = 0u64;
        nat_rem(a5324, a5325, a5326)
    };
    let v7: u64 = {
        let a5328 = v0;
        let a5329 = match v2.clone() {
            Adt3::C0(h5327, _, _, _, _, _, _, _) => {
                h5327
            }
        };
        let a5330 = 0u64;
        nat_quot(a5328, a5329, a5330)
    };
    let v8: u64 = {
        let a5332 = v7;
        let a5333 = match v2.clone() {
            Adt3::C0(_, h5331, _, _, _, _, _, _) => {
                h5331
            }
        };
        let a5334 = 0u64;
        nat_rem(a5332, a5333, a5334)
    };
    let v9: u64 = {
        let a5336 = v7;
        let a5337 = match v2.clone() {
            Adt3::C0(_, h5335, _, _, _, _, _, _) => {
                h5335
            }
        };
        let a5338 = 0u64;
        nat_quot(a5336, a5337, a5338)
    };
    let v10: u64 = {
        let a5340 = v9;
        let a5341 = match v2.clone() {
            Adt3::C0(_, _, h5339, _, _, _, _, _) => {
                h5339
            }
        };
        let a5342 = 0u64;
        nat_rem(a5340, a5341, a5342)
    };
    let v11: u64 = {
        let a5344 = v9;
        let a5345 = match v2.clone() {
            Adt3::C0(_, _, h5343, _, _, _, _, _) => {
                h5343
            }
        };
        let a5346 = 0u64;
        nat_quot(a5344, a5345, a5346)
    };
    let v12: u64 = f103(v0, v2)?;
    if v3 {
        let v13: u64 = {
            let a5355 = {
                let a5350 = {
                    let a5347 = v10;
                    let a5348 = v4;
                    let a5349 = 0u64;
                    nat_quot(a5347, a5348, a5349)
                };
                let a5351 = v5;
                nat_mul(a5350, a5351)?
            };
            let a5356 = {
                let a5352 = v8;
                let a5353 = v4;
                let a5354 = 0u64;
                nat_quot(a5352, a5353, a5354)
            };
            nat_add(a5355, a5356)?
        };
        let v14: u64 = {
            let a5376 = v6;
            let a5377 = {
                let a5374 = match v1.clone() {
                    Adt3::C0(h5357, _, _, _, _, _, _, _) => {
                        h5357
                    }
                };
                let a5375 = {
                    let a5372 = {
                        let a5358 = v8;
                        let a5359 = v4;
                        let a5360 = 0u64;
                        nat_rem(a5358, a5359, a5360)
                    };
                    let a5373 = {
                        let a5370 = match v1.clone() {
                            Adt3::C0(_, h5361, _, _, _, _, _, _) => {
                                h5361
                            }
                        };
                        let a5371 = {
                            let a5368 = {
                                let a5362 = v10;
                                let a5363 = v4;
                                let a5364 = 0u64;
                                nat_rem(a5362, a5363, a5364)
                            };
                            let a5369 = {
                                let a5366 = match v1.clone() {
                                    Adt3::C0(_, _, h5365, _, _, _, _, _) => {
                                        h5365
                                    }
                                };
                                let a5367 = v13;
                                nat_mul(a5366, a5367)?
                            };
                            nat_add(a5368, a5369)?
                        };
                        nat_mul(a5370, a5371)?
                    };
                    nat_add(a5372, a5373)?
                };
                nat_mul(a5374, a5375)?
            };
            nat_add(a5376, a5377)?
        };
        Ok(Adt9::C1(false, f103(v14, v1)?, v12))
    } else {
        let v15: u64 = {
            let a5383 = {
                let a5381 = {
                    let a5378 = v11;
                    let a5379 = v5;
                    let a5380 = 0u64;
                    nat_rem(a5378, a5379, a5380)
                };
                let a5382 = v4;
                nat_mul(a5381, a5382)?
            };
            let a5384 = v8;
            nat_add(a5383, a5384)?
        };
        let v16: u64 = {
            let a5390 = {
                let a5388 = {
                    let a5385 = v11;
                    let a5386 = v5;
                    let a5387 = 0u64;
                    nat_quot(a5385, a5386, a5387)
                };
                let a5389 = v4;
                nat_mul(a5388, a5389)?
            };
            let a5391 = v10;
            nat_add(a5390, a5391)?
        };
        let a5393 = v15;
        let a5394 = match v1.clone() {
            Adt3::C0(_, h5392, _, _, _, _, _, _) => {
                h5392
            }
        };
        if if nat_lt(a5393, a5394) {
            let a5396 = v16;
            let a5397 = match v1.clone() {
                Adt3::C0(_, _, h5395, _, _, _, _, _) => {
                    h5395
                }
            };
            nat_lt(a5396, a5397)
        } else {
            false
        } {
            Ok(Adt9::C1(false, f103({
                let a5406 = v6;
                let a5407 = {
                    let a5404 = match v1.clone() {
                        Adt3::C0(h5398, _, _, _, _, _, _, _) => {
                            h5398
                        }
                    };
                    let a5405 = {
                        let a5402 = v15;
                        let a5403 = {
                            let a5400 = match v1.clone() {
                                Adt3::C0(_, h5399, _, _, _, _, _, _) => {
                                    h5399
                                }
                            };
                            let a5401 = v16;
                            nat_mul(a5400, a5401)?
                        };
                        nat_add(a5402, a5403)?
                    };
                    nat_mul(a5404, a5405)?
                };
                nat_add(a5406, a5407)?
            }, v1)?, v12))
        } else {
            Ok(Adt9::C1(true, 0u64, v12))
        }
    }
}

pub fn f173(v0: u64, v1: Adt3, v2: Adt3, v3: u64, v4: u64) -> R<Adt9> {
    let v5: u64 = f252(23u64, v4);
    let v6: u64 = {
        let a5408 = v0;
        let a5409 = v5;
        let a5410 = 0u64;
        nat_rem(a5408, a5409, a5410)
    };
    let v7: u64 = {
        let a5411 = v0;
        let a5412 = v5;
        let a5413 = 0u64;
        nat_quot(a5411, a5412, a5413)
    };
    let v8: Adt13 = Adt13::C1(f103(v7, v2)?);
    let a5414 = v6;
    let a5415 = 0u64;
    if nat_eq(a5414, a5415) {
        Ok(Adt9::C7(Adt11::C7(16u64, v3, 0u64), Adt12::C0(0u64, f103(v7, v1)?), Adt12::C2(0u64), Adt13::C0(0u64), f188(v3), 4u64))
    } else {
        let a5416 = v6;
        let a5417 = 1u64;
        if nat_eq(a5416, a5417) {
            Ok(Adt9::C7(Adt11::C1(1u64), Adt12::C1(0u64), Adt12::C2(1065353216u64), {
                let a5418 = v4;
                let a5419 = 0u64;
                if nat_eq(a5418, a5419) {
                    v8
                } else {
                    Adt13::C0(0u64)
                }
            }, 4u64, 4u64))
        } else {
            Ok(Adt9::C7(Adt11::C2(0u64, v4, true), Adt12::C1(0u64), Adt12::C2(0u64), v8, 4u64, f188(v4)))
        }
    }
}

pub fn f174(v0: u64, v1: Adt3, v2: Adt3) -> R<Adt9> {
    let v3: u64 = f103(v0, v2.clone())?;
    let v4: u64 = f103(v0, v1.clone())?;
    let a5424 = {
        let a5421 = v0;
        let a5422 = match v1 {
            Adt3::C0(h5420, _, _, _, _, _, _, _) => {
                h5420
            }
        };
        let a5423 = 0u64;
        nat_rem(a5421, a5422, a5423)
    };
    let a5425 = 0u64;
    if nat_eq(a5424, a5425) {
        Ok(Adt9::C1(false, v4, v3))
    } else {
        Ok(Adt9::C4(8u64, 3u64, f103({
            let a5426 = v0;
            let a5427 = 1u64;
            nat_sub(a5426, a5427)
        }, v2)?, 0u64, v4, 3u64, v3))
    }
}

pub fn f175(v0: u64, v1: Adt3, v2: Adt3, v3: u64, v4: u64) -> R<Adt9> {
    let a5428 = v3;
    let a5429 = 13u64;
    if nat_eq(a5428, a5429) {
        f261(v0, v1, v4)
    } else {
        let v5: u64 = f262(match v1.clone() {
            Adt3::C0(h5430, _, _, _, _, _, _, _) => {
                h5430
            }
        }, {
            let a5431 = v3;
            let a5432 = 16u64;
            nat_eq(a5431, a5432)
        })?;
        let v6: u64 = {
            let a5433 = v0;
            let a5434 = v5;
            let a5435 = 0u64;
            nat_quot(a5433, a5434, a5435)
        };
        let v7: u64 = {
            let a5436 = v0;
            let a5437 = v5;
            let a5438 = 0u64;
            nat_rem(a5436, a5437, a5438)
        };
        let v8: u64 = {
            let a5442 = 1u64;
            let a5443 = {
                let a5440 = match v1.clone() {
                    Adt3::C0(h5439, _, _, _, _, _, _, _) => {
                        h5439
                    }
                };
                let a5441 = 2u64;
                nat_mul(a5440, a5441)?
            };
            nat_add(a5442, a5443)?
        };
        let a5444 = v7;
        let a5445 = v8;
        if nat_lt(a5444, a5445) {
            f263(v7, v6, v1, 0u64)
        } else {
            let v9: u64 = f103(v6, v2)?;
            let a5446 = v3;
            let a5447 = 14u64;
            if nat_eq(a5446, a5447) {
                Ok(Adt9::C4(2u64, 1u64, 1u64, 2u64, 0u64, 3u64, v9))
            } else {
                let a5448 = v7;
                let a5449 = v8;
                if nat_eq(a5448, a5449) {
                    Ok(Adt9::C4(2u64, 1u64, 1u64, 2u64, 0u64, 1u64, 1u64))
                } else {
                    let a5452 = v7;
                    let a5453 = {
                        let a5450 = v8;
                        let a5451 = 1u64;
                        nat_add(a5450, a5451)?
                    };
                    if nat_eq(a5452, a5453) {
                        Ok(Adt9::C4(3u64, 2u64, match v1 {
                            Adt3::C0(h5454, _, _, _, _, _, _, _) => {
                                h5454
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

pub fn f176(v0: u64, v1: Adt3, v2: Adt3, v3: u64) -> R<Adt9> {
    let a5455 = v0;
    let a5456 = 0u64;
    if nat_eq(a5455, a5456) {
        Ok(Adt9::C4(19u64, 2u64, 0u64, 2u64, 0u64, 1u64, 0u64))
    } else {
        let a5463 = {
            let a5457 = v0;
            let a5458 = 1u64;
            nat_add(a5457, a5458)?
        };
        let a5464 = {
            let a5461 = 2u64;
            let a5462 = {
                let a5459 = v3;
                let a5460 = 2u64;
                nat_mul(a5459, a5460)?
            };
            nat_add(a5461, a5462)?
        };
        if nat_eq(a5463, a5464) {
            Ok(Adt9::C4(19u64, 1u64, 0u64, 2u64, 0u64, 6u64, 0u64))
        } else {
            let v4: u64 = {
                let a5465 = v0;
                let a5466 = 1u64;
                nat_sub(a5465, a5466)
            };
            let v5: u64 = {
                let a5467 = v4;
                let a5468 = 2u64;
                let a5469 = 0u64;
                nat_quot(a5467, a5468, a5469)
            };
            let a5473 = {
                let a5470 = v4;
                let a5471 = 2u64;
                let a5472 = 0u64;
                nat_rem(a5470, a5471, a5472)
            };
            let a5474 = 0u64;
            if nat_eq(a5473, a5474) {
                Ok(Adt9::C4(17u64, 0u64, f103(v5, v1)?, 4u64, f103(v5, v2)?, 1u64, 2u64))
            } else {
                Ok(Adt9::C4(18u64, 1u64, 0u64, 1u64, 2u64, 1u64, 0u64))
            }
        }
    }
}

pub fn f177(v0: u64, v1: u64, v2: u64, v3: u64) -> R<Adt9> {
    let a5475 = v0;
    let a5476 = f46(v1, v2, v3)?;
    if nat_lt(a5475, a5476) {
        let a5477 = v0;
        let a5478 = v1;
        if nat_lt(a5477, a5478) {
            Ok(Adt9::C1(false, {
                let a5479 = 4u64;
                let a5480 = v0;
                nat_mul(a5479, a5480)?
            }, {
                let a5481 = 4u64;
                let a5482 = v0;
                nat_mul(a5481, a5482)?
            }))
        } else {
            let v4: u64 = {
                let a5483 = v0;
                let a5484 = v1;
                nat_sub(a5483, a5484)
            };
            let v5: u64 = f264(v4, v2)?;
            let a5485 = v5;
            let a5486 = 2u64;
            if nat_lt(a5485, a5486) {
                Ok(Adt9::C4(6u64, {
                    let a5487 = v5;
                    let a5488 = 0u64;
                    if nat_eq(a5487, a5488) {
                        4u64
                    } else {
                        5u64
                    }
                }, f265(v4, v2)?, 2u64, 0u64, 1u64, v5))
            } else {
                let v6: u64 = f266(v4, v2)?;
                Ok(Adt9::C4(8u64, 3u64, v6, 1u64, if f267(v4, v2)? {
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

pub fn f178(v0: u64, v1: u64) -> u64 {
    let a5489 = v0;
    let a5490 = 0u64;
    if if nat_eq(a5489, a5490) {
        true
    } else {
        let a5491 = v1;
        let a5492 = 0u64;
        nat_eq(a5491, a5492)
    } {
        1u64
    } else {
        2u64
    }
}

pub fn f179(v0: u64, v1: u64) -> Adt11 {
    let a5493 = v0;
    let a5494 = 43u64;
    if nat_eq(a5493, a5494) {
        Adt11::C8(3u64)
    } else {
        let a5495 = v1;
        let a5496 = 43u64;
        if nat_eq(a5495, a5496) {
            Adt11::C8(4u64)
        } else {
            Adt11::C2(v0, v1, true)
        }
    }
}

pub fn f180(v0: u64) -> R<u64> {
    let a5499 = 2u64;
    let a5500 = {
        let a5497 = 6u64;
        let a5498 = v0;
        nat_mul(a5497, a5498)?
    };
    nat_add(a5499, a5500)
}

pub fn f181(v0: u64, v1: bool) -> R<u64> {
    let a5503 = {
        let a5501 = v0;
        let a5502 = if v1 {
            3u64
        } else {
            2u64
        };
        nat_mul(a5501, a5502)?
    };
    let a5504 = 1u64;
    nat_add(a5503, a5504)
}

pub fn f182(v0: u64, v1: u64, v2: u64, v3: Adt3, v4: Adt3, v5: Adt3, v6: Adt3, v7: u64, v8: u64) -> R<Adt9> {
    let a5505 = v0;
    let a5506 = 16u64;
    if nat_lt(a5505, a5506) {
        let v9: u64 = {
            let a5507 = v0;
            let a5508 = 4u64;
            let a5509 = 0u64;
            nat_quot(a5507, a5508, a5509)
        };
        let v10: u64 = {
            let a5514 = {
                let a5512 = 8u64;
                let a5513 = {
                    let a5510 = 4u64;
                    let a5511 = v1;
                    nat_mul(a5510, a5511)?
                };
                nat_add(a5512, a5513)?
            };
            let a5515 = v9;
            nat_add(a5514, a5515)?
        };
        let v11: u64 = f62(v6, v9, v1, v2, 0u64)?;
        let a5519 = {
            let a5516 = v0;
            let a5517 = 4u64;
            let a5518 = 0u64;
            nat_rem(a5516, a5517, a5518)
        };
        let a5520 = 0u64;
        if nat_eq(a5519, a5520) {
            Ok(Adt9::C7(Adt11::C1(2u64), Adt12::C0(0u64, f62(v3, v10, v2, 0u64, 0u64)?), Adt12::C0(1u64, f62(v4, 2u64, 0u64, 0u64, 0u64)?), Adt13::C0(1u64), 4u64, 4u64))
        } else {
            let a5524 = {
                let a5521 = v0;
                let a5522 = 4u64;
                let a5523 = 0u64;
                nat_rem(a5521, a5522, a5523)
            };
            let a5525 = 1u64;
            if nat_eq(a5524, a5525) {
                Ok(Adt9::C7(Adt11::C1(0u64), Adt12::C1(1u64), Adt12::C0(2u64, f62(v5, v10, 0u64, 0u64, 0u64)?), Adt13::C1(v11), 4u64, 4u64))
            } else {
                let a5529 = {
                    let a5526 = v0;
                    let a5527 = 4u64;
                    let a5528 = 0u64;
                    nat_rem(a5526, a5527, a5528)
                };
                let a5530 = 2u64;
                if nat_eq(a5529, a5530) {
                    Ok(Adt9::C7(Adt11::C5(0u64), {
                        let a5531 = v9;
                        let a5532 = 0u64;
                        if nat_eq(a5531, a5532) {
                            Adt12::C2(4286578688u64)
                        } else {
                            Adt12::C1(0u64)
                        }
                    }, Adt12::C3(v11), Adt13::C0(2u64), 4u64, 4u64))
                } else {
                    Ok(Adt9::C7(Adt11::C5(3u64), {
                        let a5533 = v8;
                        let a5534 = 2u64;
                        if nat_eq(a5533, a5534) {
                            let a5535 = v9;
                            let a5536 = 0u64;
                            if nat_eq(a5535, a5536) {
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
        let a5537 = v0;
        let a5538 = 28u64;
        if nat_lt(a5537, a5538) {
            let v12: u64 = {
                let a5539 = v0;
                let a5540 = 16u64;
                nat_sub(a5539, a5540)
            };
            let v13: u64 = {
                let a5541 = v12;
                let a5542 = 3u64;
                let a5543 = 0u64;
                nat_quot(a5541, a5542, a5543)
            };
            let v14: u64 = f62(v6, v13, v1, v2, 0u64)?;
            let a5547 = {
                let a5544 = v12;
                let a5545 = 3u64;
                let a5546 = 0u64;
                nat_rem(a5544, a5545, a5546)
            };
            let a5548 = 0u64;
            if nat_eq(a5547, a5548) {
                Ok(Adt9::C7(Adt11::C1(1u64), Adt12::C3(v14), Adt12::C1(0u64), Adt13::C0(2u64), 4u64, 4u64))
            } else {
                let a5552 = {
                    let a5549 = v12;
                    let a5550 = 3u64;
                    let a5551 = 0u64;
                    nat_rem(a5549, a5550, a5551)
                };
                let a5553 = 1u64;
                if nat_eq(a5552, a5553) {
                    Ok(Adt9::C7(Adt11::C7(16u64, 0u64, 0u64), Adt12::C1(2u64), Adt12::C2(0u64), Adt13::C1(v14), 4u64, 4u64))
                } else {
                    Ok(Adt9::C7(Adt11::C1(0u64), {
                        let a5554 = v13;
                        let a5555 = 0u64;
                        if nat_eq(a5554, a5555) {
                            Adt12::C2(0u64)
                        } else {
                            Adt12::C1(1u64)
                        }
                    }, Adt12::C3(v14), Adt13::C0(1u64), 4u64, 4u64))
                }
            }
        } else {
            let a5556 = v0;
            let a5557 = 28u64;
            if nat_eq(a5556, a5557) {
                Ok(Adt9::C7(Adt11::C1(3u64), Adt12::C2(1065353216u64), Adt12::C1(1u64), Adt13::C0(1u64), 4u64, 4u64))
            } else {
                let v15: u64 = {
                    let a5558 = v0;
                    let a5559 = 29u64;
                    nat_sub(a5558, a5559)
                };
                let v16: u64 = {
                    let a5560 = v15;
                    let a5561 = 2u64;
                    let a5562 = 0u64;
                    nat_quot(a5560, a5561, a5562)
                };
                let v17: u64 = f62(v6, v16, v1, v2, 0u64)?;
                let a5566 = {
                    let a5563 = v15;
                    let a5564 = 2u64;
                    let a5565 = 0u64;
                    nat_rem(a5563, a5564, a5565)
                };
                let a5567 = 0u64;
                if nat_eq(a5566, a5567) {
                    Ok(Adt9::C7(Adt11::C1(2u64), Adt12::C3(v17), Adt12::C1(1u64), Adt13::C0(2u64), 4u64, 4u64))
                } else {
                    Ok(Adt9::C7(Adt11::C1(0u64), Adt12::C1(2u64), Adt12::C2(v7), Adt13::C1(v17), 4u64, 4u64))
                }
            }
        }
    }
}

pub fn f183(v0: u64, v1: u64, v2: Adt3, v3: u64) -> R<Adt9> {
    let v4: u64 = {
        let a5568 = v0;
        let a5569 = 9u64;
        let a5570 = 0u64;
        nat_rem(a5568, a5569, a5570)
    };
    let v5: u64 = {
        let a5574 = {
            let a5571 = v0;
            let a5572 = 9u64;
            let a5573 = 0u64;
            nat_quot(a5571, a5572, a5573)
        };
        let a5575 = 4u64;
        let a5576 = 0u64;
        nat_rem(a5574, a5575, a5576)
    };
    let v6: bool = {
        let a5583 = {
            let a5580 = {
                let a5577 = v0;
                let a5578 = 36u64;
                let a5579 = 0u64;
                nat_quot(a5577, a5578, a5579)
            };
            let a5581 = 2u64;
            let a5582 = 0u64;
            nat_rem(a5580, a5581, a5582)
        };
        let a5584 = 0u64;
        nat_eq(a5583, a5584)
    };
    let v7: u64 = {
        let a5585 = v4;
        let a5586 = 4u64;
        if nat_lt(a5585, a5586) {
            v4
        } else {
            let a5587 = v4;
            let a5588 = 5u64;
            nat_sub(a5587, a5588)
        }
    };
    let v8: u64 = f62(v2, if v6 {
        v5
    } else {
        v7
    }, if v6 {
        v7
    } else {
        v5
    }, v1, 0u64)?;
    let a5589 = v4;
    let a5590 = 4u64;
    if nat_lt(a5589, a5590) {
        Ok(Adt9::C7(Adt11::C1(0u64), {
            let a5591 = v4;
            let a5592 = 0u64;
            if nat_eq(a5591, a5592) {
                Adt12::C2(v3)
            } else {
                Adt12::C1(1u64)
            }
        }, Adt12::C3(v8), Adt13::C0(1u64), 4u64, 4u64))
    } else {
        let a5593 = v4;
        let a5594 = 4u64;
        if nat_eq(a5593, a5594) {
            Ok(Adt9::C7(Adt11::C1(3u64), Adt12::C2(1065353216u64), Adt12::C1(1u64), Adt13::C0(1u64), 4u64, 4u64))
        } else {
            Ok(Adt9::C7(Adt11::C1(2u64), Adt12::C3(v8), Adt12::C1(1u64), Adt13::C1(v8), 4u64, 4u64))
        }
    }
}

pub fn f184(v0: bool, v1: u64) -> R<u64> {
    if v0 {
        let a5597 = {
            let a5595 = 3u64;
            let a5596 = v1;
            nat_mul(a5595, a5596)?
        };
        let a5598 = 4u64;
        nat_add(a5597, a5598)
    } else {
        let a5599 = v1;
        let a5600 = 3u64;
        nat_add(a5599, a5600)
    }
}

pub fn f185(v0: Adt7, v1: Adt3, v2: Adt3, v3: u64, v4: u64) -> R<Option<u64>> {
    let m5609 = f65(match v0.clone() {
        Adt7::C0(h5601, _, _, _, _) => {
            h5601
        }
    }, match v1.clone() {
        Adt3::C0(h5602, _, _, _, _, _, _, _) => {
            h5602
        }
    }, f63(v3, v2.clone(), 0u64), {
        let a5606 = v4;
        let a5607 = {
            let m5605 = match v0.clone() {
                Adt7::C0(h5604, _, _, _, _) => {
                    h5604
                }
            };
            match m5605 {
                Adt6::C0(h5603, _, _, _) => {
                    h5603
                }
            }
        };
        let a5608 = 0u64;
        nat_rem(a5606, a5607, a5608)
    })?;
    match m5609 {
        None => {
            Ok(None::<u64>)
        }
        Some(v5) => {
            let m5618 = f65(match v0.clone() {
                Adt7::C0(_, h5610, _, _, _) => {
                    h5610
                }
            }, match v1.clone() {
                Adt3::C0(_, h5611, _, _, _, _, _, _) => {
                    h5611
                }
            }, f63(v3, v2.clone(), 1u64), {
                let a5615 = v4;
                let a5616 = {
                    let m5614 = match v0 {
                        Adt7::C0(h5613, _, _, _, _) => {
                            h5613
                        }
                    };
                    match m5614 {
                        Adt6::C0(h5612, _, _, _) => {
                            h5612
                        }
                    }
                };
                let a5617 = 0u64;
                nat_quot(a5615, a5616, a5617)
            })?;
            match m5618 {
                None => {
                    Ok(None::<u64>)
                }
                Some(v6) => {
                    Ok(Some(f62(v1, v5, v6, f63(v3, v2.clone(), 2u64), f63(v3, v2, 3u64))?))
                }
            }
        }
    }
}

pub fn f186(v0: Adt6, v1: u64, v2: u64, v3: u64) -> R<Option<u64>> {
    let v4: u64 = {
        let a5620 = v2;
        let a5621 = f156(match v0.clone() {
            Adt6::C0(_, _, h5619, _) => {
                h5619
            }
        });
        nat_add(a5620, a5621)?
    };
    let v5: u64 = {
        let a5626 = {
            let a5623 = v3;
            let a5624 = match v0.clone() {
                Adt6::C0(_, _, _, h5622) => {
                    h5622
                }
            };
            nat_mul(a5623, a5624)?
        };
        let a5627 = f155(match v0.clone() {
            Adt6::C0(_, _, h5625, _) => {
                h5625
            }
        });
        nat_add(a5626, a5627)?
    };
    let v6: u64 = {
        let a5628 = v4;
        let a5629 = v5;
        nat_sub(a5628, a5629)
    };
    let v7: u64 = match v0 {
        Adt6::C0(_, h5630, _, _) => {
            h5630
        }
    };
    let v8: u64 = {
        let a5631 = v6;
        let a5632 = v7;
        let a5633 = 0u64;
        nat_quot(a5631, a5632, a5633)
    };
    let a5634 = v5;
    let a5635 = v4;
    if if if nat_le(a5634, a5635) {
        let a5639 = {
            let a5636 = v6;
            let a5637 = v7;
            let a5638 = 0u64;
            nat_rem(a5636, a5637, a5638)
        };
        let a5640 = 0u64;
        nat_eq(a5639, a5640)
    } else {
        false
    } {
        let a5641 = v8;
        let a5642 = v1;
        nat_lt(a5641, a5642)
    } else {
        false
    } {
        Ok(Some(v8))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f187(v0: bool, v1: u64) -> R<u64> {
    let a5645 = if v0 {
        8u64
    } else {
        3u64
    };
    let a5646 = {
        let a5643 = v1;
        let a5644 = 0u64;
        if nat_eq(a5643, a5644) {
            0u64
        } else {
            1u64
        }
    };
    nat_add(a5645, a5646)
}

pub fn f188(v0: u64) -> u64 {
    let m5647 = f230(v0);
    match m5647 {
        Adt18::C0 => {
            0u64
        }
        Adt18::C1(v1) => {
            let a5649 = match v1.clone() {
                Adt19::C0(_, h5648, _) => {
                    h5648
                }
            };
            let a5650 = 1u64;
            if if nat_eq(a5649, a5650) {
                let a5652 = match v1.clone() {
                    Adt19::C0(_, _, h5651) => {
                        h5651
                    }
                };
                let a5653 = 1u64;
                if if if nat_eq(a5652, a5653) {
                    true
                } else {
                    let a5655 = match v1.clone() {
                        Adt19::C0(_, _, h5654) => {
                            h5654
                        }
                    };
                    let a5656 = 2u64;
                    nat_eq(a5655, a5656)
                } {
                    true
                } else {
                    let a5658 = match v1.clone() {
                        Adt19::C0(_, _, h5657) => {
                            h5657
                        }
                    };
                    let a5659 = 4u64;
                    nat_eq(a5658, a5659)
                } {
                    true
                } else {
                    let a5661 = match v1.clone() {
                        Adt19::C0(_, _, h5660) => {
                            h5660
                        }
                    };
                    let a5662 = 8u64;
                    nat_eq(a5661, a5662)
                }
            } else {
                false
            } {
                match v1 {
                    Adt19::C0(_, _, h5663) => {
                        h5663
                    }
                }
            } else {
                0u64
            }
        }
    }
}

pub fn f189(v0: u64, v1: u64, v2: Adt1, v3: Adt13) -> Adt9 {
    let a5664 = v0;
    let a5665 = 2u64;
    if nat_eq(a5664, a5665) {
        let a5666 = v1;
        let a5667 = 2u64;
        if nat_eq(a5666, a5667) {
            Adt9::C7(Adt11::C5(3u64), Adt12::C2(5u64), Adt12::C2(0u64), Adt13::C0(2u64), 4u64, 4u64)
        } else {
            Adt9::C7(Adt11::C5(0u64), Adt12::C1(0u64), Adt12::C2(match v2 {
                Adt1::C0(_, _, _, _, h5668, _, _, _, _, _, _, _) => {
                    h5668
                }
            }), Adt13::C0(2u64), 4u64, 4u64)
        }
    } else {
        let a5669 = v0;
        let a5670 = 3u64;
        if nat_eq(a5669, a5670) {
            let a5671 = v1;
            let a5672 = 5u64;
            if nat_eq(a5671, a5672) {
                Adt9::C7(Adt11::C1(2u64), Adt12::C2(match v2 {
                    Adt1::C0(_, _, h5673, _, _, _, _, _, _, _, _, _) => {
                        h5673
                    }
                }), Adt12::C1(0u64), Adt13::C0(1u64), 4u64, 4u64)
            } else {
                Adt9::C7(Adt11::C3(0u64, 0u64, false), {
                    let a5674 = v1;
                    let a5675 = 0u64;
                    if if if nat_eq(a5674, a5675) {
                        true
                    } else {
                        let a5676 = v1;
                        let a5677 = 3u64;
                        nat_eq(a5676, a5677)
                    } {
                        true
                    } else {
                        let a5678 = v1;
                        let a5679 = 1u64;
                        if nat_eq(a5678, a5679) {
                            let a5681 = match v2.clone() {
                                Adt1::C0(_, _, _, _, h5680, _, _, _, _, _, _, _) => {
                                    h5680
                                }
                            };
                            let a5682 = 2147483648u64;
                            nat_lt(a5681, a5682)
                        } else {
                            false
                        }
                    } {
                        Adt12::C1(0u64)
                    } else {
                        Adt12::C2(match v2 {
                            Adt1::C0(_, _, _, _, h5683, _, _, _, _, _, _, _) => {
                                h5683
                            }
                        })
                    }
                }, Adt12::C2(13u64), Adt13::C0(1u64), 4u64, 4u64)
            }
        } else {
            let a5684 = v0;
            let a5685 = 4u64;
            if nat_eq(a5684, a5685) {
                Adt9::C7({
                    let a5686 = v1;
                    let a5687 = 5u64;
                    if nat_eq(a5686, a5687) {
                        Adt11::C1(2u64)
                    } else {
                        Adt11::C1(1u64)
                    }
                }, Adt12::C1(1u64), Adt12::C1(0u64), Adt13::C0(1u64), 4u64, 4u64)
            } else {
                let a5688 = v0;
                let a5689 = 5u64;
                if nat_eq(a5688, a5689) {
                    let a5690 = v1;
                    let a5691 = 5u64;
                    if nat_eq(a5690, a5691) {
                        Adt9::C7(Adt11::C5(3u64), Adt12::C1(1u64), Adt12::C2(0u64), Adt13::C0(1u64), 4u64, 4u64)
                    } else {
                        Adt9::C7(Adt11::C1(2u64), Adt12::C1(1u64), Adt12::C2(match v2 {
                            Adt1::C0(_, h5692, _, _, _, _, _, _, _, _, _, _) => {
                                h5692
                            }
                        }), Adt13::C0(1u64), 4u64, 4u64)
                    }
                } else {
                    let a5693 = v0;
                    let a5694 = 6u64;
                    if nat_eq(a5693, a5694) {
                        Adt9::C7(Adt11::C1(2u64), Adt12::C2(match v2 {
                            Adt1::C0(_, _, _, h5695, _, _, _, _, _, _, _, _) => {
                                h5695
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

pub fn f190(v0: u64) -> R<u64> {
    let a5698 = v0;
    let a5699 = {
        let a5696 = v0;
        let a5697 = 2u64;
        nat_add(a5696, a5697)?
    };
    nat_mul(a5698, a5699)
}

pub fn f191(v0: u64) -> R<u64> {
    let a5702 = 3u64;
    let a5703 = {
        let a5700 = v0;
        let a5701 = 3u64;
        nat_mul(a5700, a5701)?
    };
    nat_add(a5702, a5703)
}

pub fn f192(v0: u64, v1: u64) -> R<u64> {
    if f193(v0) {
        let a5704 = 2u64;
        let a5705 = v1;
        nat_mul(a5704, a5705)
    } else {
        let a5706 = v1;
        let a5707 = f268(v0);
        let a5708 = 0u64;
        Ok(nat_quot(a5706, a5707, a5708))
    }
}

pub fn f193(v0: u64) -> bool {
    let a5709 = v0;
    let a5710 = 0u64;
    if if nat_eq(a5709, a5710) {
        true
    } else {
        let a5711 = v0;
        let a5712 = 1u64;
        nat_eq(a5711, a5712)
    } {
        true
    } else {
        let a5713 = v0;
        let a5714 = 30u64;
        nat_eq(a5713, a5714)
    }
}

pub fn f194(v0: u64, v1: u64, v2: u64, v3: u64, v4: Adt3, v5: Adt3) -> R<Adt9> {
    let m5715 = f230(v0);
    match m5715 {
        Adt18::C0 => {
            Ok(Adt9::C0(Adt10::C1))
        }
        Adt18::C1(v6) => {
            Ok(Adt9::C10(v0, f154(v2, v1, 0u64, 0u64, match v4.clone() {
                Adt3::C0(_, _, _, _, h5716, _, _, _) => {
                    h5716
                }
            }, match v4.clone() {
                Adt3::C0(_, _, _, _, _, h5717, _, _) => {
                    h5717
                }
            }, match v4.clone() {
                Adt3::C0(_, _, _, _, _, _, h5718, _) => {
                    h5718
                }
            }, match v4 {
                Adt3::C0(_, _, _, _, _, _, _, h5719) => {
                    h5719
                }
            })?, match v6.clone() {
                Adt19::C0(_, _, h5720) => {
                    h5720
                }
            }, f154({
                let a5722 = v2;
                let a5723 = match v6.clone() {
                    Adt19::C0(_, h5721, _) => {
                        h5721
                    }
                };
                nat_mul(a5722, a5723)?
            }, v3, 0u64, 0u64, match v5.clone() {
                Adt3::C0(_, _, _, _, h5724, _, _, _) => {
                    h5724
                }
            }, match v5.clone() {
                Adt3::C0(_, _, _, _, _, h5725, _, _) => {
                    h5725
                }
            }, match v5.clone() {
                Adt3::C0(_, _, _, _, _, _, h5726, _) => {
                    h5726
                }
            }, match v5.clone() {
                Adt3::C0(_, _, _, _, _, _, _, h5727) => {
                    h5727
                }
            })?, match v6 {
                Adt19::C0(_, h5728, _) => {
                    h5728
                }
            }, match v5 {
                Adt3::C0(_, _, _, _, h5729, _, _, _) => {
                    h5729
                }
            }))
        }
    }
}

pub fn f195(v0: Adt3, v1: Adt3) -> R<u64> {
    let a5737 = {
        let a5730 = 2u64;
        let a5731 = f196(v1.clone())?;
        nat_mul(a5730, a5731)?
    };
    let a5738 = {
        let a5735 = f196(v1)?;
        let a5736 = {
            let a5733 = 1u64;
            let a5734 = match v0 {
                Adt3::C0(h5732, _, _, _, _, _, _, _) => {
                    h5732
                }
            };
            nat_add(a5733, a5734)?
        };
        nat_mul(a5735, a5736)?
    };
    nat_add(a5737, a5738)
}

pub fn f196(v0: Adt3) -> R<u64> {
    if f53(match v0.clone() {
        Adt3::C0(h5739, _, _, _, _, _, _, _) => {
            h5739
        }
    }, match v0.clone() {
        Adt3::C0(_, h5740, _, _, _, _, _, _) => {
            h5740
        }
    }, match v0.clone() {
        Adt3::C0(_, _, h5741, _, _, _, _, _) => {
            h5741
        }
    }, match v0.clone() {
        Adt3::C0(_, _, _, h5742, _, _, _, _) => {
            h5742
        }
    }) {
        Ok(0u64)
    } else {
        let a5751 = match v0.clone() {
            Adt3::C0(h5743, _, _, _, _, _, _, _) => {
                h5743
            }
        };
        let a5752 = {
            let a5749 = match v0.clone() {
                Adt3::C0(_, h5744, _, _, _, _, _, _) => {
                    h5744
                }
            };
            let a5750 = {
                let a5747 = match v0.clone() {
                    Adt3::C0(_, _, h5745, _, _, _, _, _) => {
                        h5745
                    }
                };
                let a5748 = match v0 {
                    Adt3::C0(_, _, _, h5746, _, _, _, _) => {
                        h5746
                    }
                };
                nat_mul(a5747, a5748)?
            };
            nat_mul(a5749, a5750)?
        };
        nat_mul(a5751, a5752)
    }
}

pub fn f197(v0: u64, v1: Adt3) -> R<Adt9> {
    Ok(Adt9::C4(19u64, 4u64, f103(v0, v1)?, 2u64, 0u64, 1u64, 2u64))
}

pub fn f198(v0: u64, v1: u64) -> bool {
    let a5753 = v1;
    let a5754 = 2147483647u64;
    if nat_le(a5753, a5754) {
        let a5755 = v1;
        let a5756 = v0;
        nat_lt(a5755, a5756)
    } else {
        false
    }
}

pub fn f199(v0: u64, v1: u64, v2: u64, v3: Adt3, v4: Adt3, v5: Adt3, v6: u64) -> R<Adt9> {
    let m5757 = f230(v0);
    match m5757 {
        Adt18::C0 => {
            Ok(Adt9::C0(Adt10::C1))
        }
        Adt18::C1(v7) => {
            let v8: u64 = {
                let a5759 = v2;
                let a5760 = match v7.clone() {
                    Adt19::C0(_, h5758, _) => {
                        h5758
                    }
                };
                nat_mul(a5759, a5760)?
            };
            let m5773 = f269(v0, v8, v6, v1, match v5.clone() {
                Adt3::C0(h5761, _, _, _, _, _, _, _) => {
                    h5761
                }
            }, match v5.clone() {
                Adt3::C0(_, h5762, _, _, _, _, _, _) => {
                    h5762
                }
            }, match v5 {
                Adt3::C0(_, _, h5763, _, _, _, _, _) => {
                    h5763
                }
            }, {
                let a5766 = match v3.clone() {
                    Adt3::C0(h5764, _, _, _, _, _, _, _) => {
                        h5764
                    }
                };
                let a5767 = match v7.clone() {
                    Adt19::C0(_, h5765, _) => {
                        h5765
                    }
                };
                nat_mul(a5766, a5767)?
            }, match v3.clone() {
                Adt3::C0(_, h5768, _, _, _, _, _, _) => {
                    h5768
                }
            }, match v3.clone() {
                Adt3::C0(_, _, _, _, h5769, _, _, _) => {
                    h5769
                }
            }, match v3.clone() {
                Adt3::C0(_, _, _, _, _, h5770, _, _) => {
                    h5770
                }
            }, match v3.clone() {
                Adt3::C0(_, _, _, _, _, _, h5771, _) => {
                    h5771
                }
            }, match v3 {
                Adt3::C0(_, _, _, _, _, _, _, h5772) => {
                    h5772
                }
            })?;
            match m5773 {
                None => {
                    Ok(Adt9::C0(Adt10::C1))
                }
                Some(v9) => {
                    let a5774 = v0;
                    let a5775 = 0u64;
                    if if nat_eq(a5774, a5775) {
                        true
                    } else {
                        let a5776 = v0;
                        let a5777 = 26u64;
                        nat_eq(a5776, a5777)
                    } {
                        Ok(Adt9::C1(false, v9, f103({
                            let a5781 = {
                                let a5779 = v1;
                                let a5780 = match v4.clone() {
                                    Adt3::C0(h5778, _, _, _, _, _, _, _) => {
                                        h5778
                                    }
                                };
                                nat_mul(a5779, a5780)?
                            };
                            let a5782 = v8;
                            nat_add(a5781, a5782)?
                        }, v4)?))
                    } else {
                        Ok(Adt9::C5(v0, v9, match v7.clone() {
                            Adt19::C0(_, _, h5783) => {
                                h5783
                            }
                        }, f103({
                            let a5787 = {
                                let a5785 = v1;
                                let a5786 = match v4.clone() {
                                    Adt3::C0(h5784, _, _, _, _, _, _, _) => {
                                        h5784
                                    }
                                };
                                nat_mul(a5785, a5786)?
                            };
                            let a5788 = v8;
                            nat_add(a5787, a5788)?
                        }, v4.clone())?, {
                            let a5795 = {
                                let a5793 = {
                                    let a5790 = match v7 {
                                        Adt19::C0(_, h5789, _) => {
                                            h5789
                                        }
                                    };
                                    let a5791 = 1u64;
                                    nat_sub(a5790, a5791)
                                };
                                let a5794 = match v4.clone() {
                                    Adt3::C0(_, _, _, _, h5792, _, _, _) => {
                                        h5792
                                    }
                                };
                                nat_mul(a5793, a5794)?
                            };
                            let a5796 = 4u64;
                            nat_add(a5795, a5796)?
                        }, match v4 {
                            Adt3::C0(_, _, _, _, h5797, _, _, _) => {
                                h5797
                            }
                        }))
                    }
                }
            }
        }
    }
}

pub fn f200(v0: u64, v1: u64, v2: u64) -> u64 {
    let a5798 = v1;
    let a5799 = 0u64;
    if if nat_eq(a5798, a5799) {
        true
    } else {
        let a5800 = v2;
        let a5801 = 0u64;
        nat_eq(a5800, a5801)
    } {
        0u64
    } else {
        v0
    }
}

pub fn f201(v0: u64, v1: u64) -> bool {
    let a5802 = v0;
    let a5803 = 4294967295u64;
    if nat_eq(a5802, a5803) {
        true
    } else {
        let a5804 = v0;
        let a5805 = 2147483648u64;
        if nat_lt(a5804, a5805) {
            let a5806 = v0;
            let a5807 = v1;
            nat_lt(a5806, a5807)
        } else {
            false
        }
    }
}

pub fn f202(v0: Adt7) -> R<u64> {
    let a5819 = if match v0.clone() {
        Adt7::C0(_, _, _, h5808, _) => {
            h5808
        }
    } {
        2u64
    } else {
        3u64
    };
    let a5820 = {
        let a5817 = {
            let a5815 = {
                let m5811 = match v0.clone() {
                    Adt7::C0(h5810, _, _, _, _) => {
                        h5810
                    }
                };
                match m5811 {
                    Adt6::C0(h5809, _, _, _) => {
                        h5809
                    }
                }
            };
            let a5816 = {
                let m5814 = match v0.clone() {
                    Adt7::C0(_, h5813, _, _, _) => {
                        h5813
                    }
                };
                match m5814 {
                    Adt6::C0(h5812, _, _, _) => {
                        h5812
                    }
                }
            };
            nat_mul(a5815, a5816)?
        };
        let a5818 = f203(v0)?;
        nat_mul(a5817, a5818)?
    };
    nat_add(a5819, a5820)
}

pub fn f203(v0: Adt7) -> R<u64> {
    let a5825 = if match v0.clone() {
        Adt7::C0(_, _, _, h5821, _) => {
            h5821
        }
    } {
        2u64
    } else {
        1u64
    };
    let a5826 = {
        let a5823 = match v0 {
            Adt7::C0(_, _, _, _, h5822) => {
                h5822
            }
        };
        let a5824 = 1u64;
        if nat_eq(a5823, a5824) {
            1u64
        } else {
            0u64
        }
    };
    nat_add(a5825, a5826)
}

pub fn f204(v0: Adt6, v1: u64, v2: u64) -> R<u64> {
    let v3: u64 = {
        let a5831 = {
            let a5828 = v2;
            let a5829 = match v0.clone() {
                Adt6::C0(_, h5827, _, _) => {
                    h5827
                }
            };
            nat_mul(a5828, a5829)?
        };
        let a5832 = f155(match v0.clone() {
            Adt6::C0(_, _, h5830, _) => {
                h5830
            }
        });
        nat_add(a5831, a5832)?
    };
    let v4: u64 = f156(match v0.clone() {
        Adt6::C0(_, _, h5833, _) => {
            h5833
        }
    });
    let v5: u64 = {
        let a5837 = {
            let a5835 = v3;
            let a5836 = match v0 {
                Adt6::C0(h5834, _, _, _) => {
                    h5834
                }
            };
            nat_add(a5835, a5836)?
        };
        let a5838 = v4;
        nat_sub(a5837, a5838)
    };
    let a5843 = {
        let a5839 = v5;
        let a5840 = v1;
        if nat_lt(a5839, a5840) {
            v5
        } else {
            v1
        }
    };
    let a5844 = {
        let a5841 = v3;
        let a5842 = v4;
        nat_sub(a5841, a5842)
    };
    Ok(nat_sub(a5843, a5844))
}

pub fn f205(v0: u64) -> R<u64> {
    let a5847 = 2u64;
    let a5848 = {
        let a5845 = v0;
        let a5846 = 2u64;
        nat_mul(a5845, a5846)?
    };
    nat_add(a5847, a5848)
}

pub fn f206(v0: Adt6) -> R<u64> {
    let a5861 = 2u64;
    let a5862 = {
        let a5859 = 2u64;
        let a5860 = {
            let a5856 = {
                let a5853 = match v0.clone() {
                    Adt6::C0(h5849, _, _, _) => {
                        h5849
                    }
                };
                let a5854 = {
                    let a5851 = match v0.clone() {
                        Adt6::C0(_, h5850, _, _) => {
                            h5850
                        }
                    };
                    let a5852 = 1u64;
                    nat_sub(a5851, a5852)
                };
                nat_add(a5853, a5854)?
            };
            let a5857 = match v0 {
                Adt6::C0(_, h5855, _, _) => {
                    h5855
                }
            };
            let a5858 = 0u64;
            nat_quot(a5856, a5857, a5858)
        };
        nat_mul(a5859, a5860)?
    };
    nat_add(a5861, a5862)
}

pub fn f207(v0: Adt16, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64, v7: u64, v8: u64, v9: u64, v10: u64, v11: u64, v12: u64, v13: u64, v14: u64, v15: u64, v16: u64, v17: u64, v18: u64) -> R<Adt17> {
    let m5863 = v0;
    match m5863 {
        Adt16::C0(v19) => {
            f270(v1, v2, v3, v4, v5, v6, v7, v8, v9, v14, v15, v16, v17, v19, v18)
        }
        Adt16::C1(v20, v21, v22, v23) => {
            let a5864 = v20;
            let a5865 = 66u64;
            if nat_eq(a5864, a5865) {
                f271(v1, v2, v3, v4, v5, v6, v7, v8, v9, v10, v11, v12, v13, v14, v15, v16, v17, v21, v22, v23, v18)
            } else {
                f272(v20, v1, v2, v3, v4, v5, v6, v7, v8, v9, v14, v15, v16, v17, v21, v22, v23, v18)
            }
        }
    }
}

pub fn f208(v0: u64, v1: Adt3, v2: u64, v3: u64, v4: u64) -> R<Adt9> {
    let v5: u64 = {
        let a5872 = v3;
        let a5873 = {
            let a5869 = f103({
                let a5866 = v0;
                let a5867 = 2u64;
                let a5868 = 0u64;
                nat_quot(a5866, a5867, a5868)
            }, v1)?;
            let a5870 = v2;
            let a5871 = 0u64;
            nat_quot(a5869, a5870, a5871)
        };
        nat_add(a5872, a5873)?
    };
    let a5877 = {
        let a5874 = v0;
        let a5875 = 2u64;
        let a5876 = 0u64;
        nat_rem(a5874, a5875, a5876)
    };
    let a5878 = 0u64;
    if nat_eq(a5877, a5878) {
        Ok(Adt9::C4(19u64, 1u64, v5, 2u64, 0u64, 1u64, 2u64))
    } else {
        let a5879 = v4;
        let a5880 = 0u64;
        if nat_eq(a5879, a5880) {
            Ok(Adt9::C4(19u64, 2u64, 1u64, 2u64, 0u64, 1u64, v5))
        } else {
            Ok(Adt9::C0(Adt10::C1))
        }
    }
}

pub fn f209(v0: u64, v1: u64) -> R<u64> {
    let a5883 = 8u64;
    let a5884 = {
        let a5881 = f273(v0);
        let a5882 = v1;
        nat_mul(a5881, a5882)?
    };
    nat_add(a5883, a5884)
}

pub fn f210(v0: u64, v1: u64, v2: Adt3, v3: u64, v4: u64, v5: u64) -> R<Adt9> {
    let v6: u64 = match v2.clone() {
        Adt3::C0(h5885, _, _, _, _, _, _, _) => {
            h5885
        }
    };
    let v7: u64 = f273(v3);
    let v8: u64 = {
        let a5888 = 1u64;
        let a5889 = {
            let a5886 = v6;
            let a5887 = v7;
            nat_mul(a5886, a5887)?
        };
        nat_add(a5888, a5889)?
    };
    let a5890 = v0;
    let a5891 = 0u64;
    if nat_eq(a5890, a5891) {
        Ok(f274(0u64))
    } else {
        let a5892 = v0;
        let a5893 = v8;
        if nat_lt(a5892, a5893) {
            let v9: u64 = {
                let a5894 = v0;
                let a5895 = 1u64;
                nat_sub(a5894, a5895)
            };
            let v10: u64 = {
                let a5896 = v9;
                let a5897 = v7;
                let a5898 = 0u64;
                nat_rem(a5896, a5897, a5898)
            };
            let v11: u64 = f103({
                let a5904 = {
                    let a5899 = v1;
                    let a5900 = v6;
                    nat_mul(a5899, a5900)?
                };
                let a5905 = {
                    let a5901 = v9;
                    let a5902 = v7;
                    let a5903 = 0u64;
                    nat_quot(a5901, a5902, a5903)
                };
                nat_add(a5904, a5905)?
            }, v2)?;
            let a5906 = v3;
            let a5907 = 30u64;
            if nat_eq(a5906, a5907) {
                let a5908 = v10;
                let a5909 = 0u64;
                if nat_eq(a5908, a5909) {
                    Ok(Adt9::C4(6u64, 0u64, v11, 2u64, 0u64, 1u64, 1u64))
                } else {
                    let a5910 = v10;
                    let a5911 = 1u64;
                    if nat_eq(a5910, a5911) {
                        Ok(Adt9::C4(10u64, 1u64, 1u64, 1u64, 1u64, 1u64, 1u64))
                    } else {
                        let a5912 = v10;
                        let a5913 = 2u64;
                        if nat_eq(a5912, a5913) {
                            Ok(Adt9::C4(1u64, 1u64, 1u64, 2u64, 0u64, 1u64, 1u64))
                        } else {
                            Ok(Adt9::C4(0u64, 1u64, 0u64, 1u64, 1u64, 1u64, 0u64))
                        }
                    }
                }
            } else {
                let a5914 = v10;
                let a5915 = 0u64;
                if nat_eq(a5914, a5915) {
                    Ok(Adt9::C4(10u64, 0u64, v11, 0u64, v11, 1u64, 1u64))
                } else {
                    let a5916 = v10;
                    let a5917 = 1u64;
                    if nat_eq(a5916, a5917) {
                        Ok(Adt9::C4(1u64, 1u64, 1u64, 2u64, 0u64, 1u64, 1u64))
                    } else {
                        Ok(Adt9::C4(0u64, 1u64, 0u64, 1u64, 1u64, 1u64, 0u64))
                    }
                }
            }
        } else {
            let v12: u64 = {
                let a5918 = v0;
                let a5919 = v8;
                nat_sub(a5918, a5919)
            };
            let a5920 = v12;
            let a5921 = 0u64;
            if nat_eq(a5920, a5921) {
                Ok(Adt9::C4(22u64, 2u64, v6, 2u64, 0u64, 1u64, 1u64))
            } else {
                let a5922 = v12;
                let a5923 = 1u64;
                if nat_eq(a5922, a5923) {
                    Ok(Adt9::C4(21u64, 1u64, 0u64, 1u64, 1u64, 1u64, 1u64))
                } else {
                    let a5924 = v12;
                    let a5925 = 2u64;
                    if nat_eq(a5924, a5925) {
                        Ok(Adt9::C4(2u64, 1u64, 1u64, 2u64, 0u64, 1u64, 1u64))
                    } else {
                        let a5926 = v12;
                        let a5927 = 3u64;
                        if nat_eq(a5926, a5927) {
                            Ok(Adt9::C4(4u64, 1u64, 1u64, 2u64, v4, 1u64, 1u64))
                        } else {
                            let a5928 = v12;
                            let a5929 = 4u64;
                            if nat_eq(a5928, a5929) {
                                Ok(Adt9::C4(12u64, 1u64, 1u64, 2u64, 0u64, 1u64, 1u64))
                            } else {
                                let a5930 = v12;
                                let a5931 = 5u64;
                                if nat_eq(a5930, a5931) {
                                    Ok(Adt9::C4(11u64, 2u64, 1065353216u64, 1u64, 1u64, 1u64, 2u64))
                                } else {
                                    let a5932 = v12;
                                    let a5933 = 6u64;
                                    if nat_eq(a5932, a5933) {
                                        if f275(v5) {
                                            Ok(Adt9::C4(19u64, 1u64, 2u64, 2u64, 0u64, 1u64, {
                                                let a5934 = 3u64;
                                                let a5935 = v1;
                                                nat_add(a5934, a5935)?
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

pub fn f211(v0: u64, v1: Adt3, v2: Adt3, v3: u64) -> R<Adt9> {
    let v4: u64 = {
        let a5936 = v0;
        let a5937 = f114(v3);
        let a5938 = 0u64;
        nat_quot(a5936, a5937, a5938)
    };
    let v5: u64 = {
        let a5943 = 3u64;
        let a5944 = {
            let a5940 = v4;
            let a5941 = match v1.clone() {
                Adt3::C0(h5939, _, _, _, _, _, _, _) => {
                    h5939
                }
            };
            let a5942 = 0u64;
            nat_quot(a5940, a5941, a5942)
        };
        nat_add(a5943, a5944)?
    };
    let v6: u64 = f103(v4, v1)?;
    let v7: u64 = f103(v4, v2)?;
    let a5945 = v3;
    let a5946 = 30u64;
    if nat_eq(a5945, a5946) {
        let a5950 = {
            let a5947 = v0;
            let a5948 = 3u64;
            let a5949 = 0u64;
            nat_rem(a5947, a5948, a5949)
        };
        let a5951 = 0u64;
        if nat_eq(a5950, a5951) {
            Ok(Adt9::C4(6u64, 0u64, v6, 2u64, 0u64, 1u64, 1u64))
        } else {
            let a5955 = {
                let a5952 = v0;
                let a5953 = 3u64;
                let a5954 = 0u64;
                nat_rem(a5952, a5953, a5954)
            };
            let a5956 = 1u64;
            if nat_eq(a5955, a5956) {
                Ok(Adt9::C4(10u64, 1u64, 1u64, 1u64, v5, 1u64, 1u64))
            } else {
                Ok(Adt9::C4(7u64, 1u64, 1u64, 2u64, 0u64, 3u64, v7))
            }
        }
    } else {
        Ok(Adt9::C4(10u64, 0u64, v6, 1u64, v5, 3u64, v7))
    }
}

pub fn f212(v0: u64) -> R<u64> {
    let v1: u64 = {
        let a5957 = v0;
        let a5958 = 8388608u64;
        let a5959 = 0u64;
        nat_quot(a5957, a5958, a5959)
    };
    let a5960 = v0;
    let a5961 = 2147483648u64;
    if if if nat_lt(a5960, a5961) {
        let a5962 = 127u64;
        let a5963 = v1;
        nat_le(a5962, a5963)
    } else {
        false
    } {
        let a5964 = v1;
        let a5965 = 190u64;
        nat_lt(a5964, a5965)
    } else {
        false
    } {
        let v2: u64 = {
            let a5969 = 8388608u64;
            let a5970 = {
                let a5966 = v0;
                let a5967 = 8388608u64;
                let a5968 = 0u64;
                nat_rem(a5966, a5967, a5968)
            };
            nat_add(a5969, a5970)?
        };
        let a5971 = v1;
        let a5972 = 150u64;
        if nat_lt(a5971, a5972) {
            let v3: u64 = f276({
                let a5973 = 150u64;
                let a5974 = v1;
                nat_sub(a5973, a5974)
            });
            let a5978 = {
                let a5975 = v2;
                let a5976 = v3;
                let a5977 = 0u64;
                nat_rem(a5975, a5976, a5977)
            };
            let a5979 = 0u64;
            if nat_eq(a5978, a5979) {
                let a5980 = v2;
                let a5981 = v3;
                let a5982 = 0u64;
                Ok(nat_quot(a5980, a5981, a5982))
            } else {
                Ok(0u64)
            }
        } else {
            let a5985 = f276({
                let a5983 = v1;
                let a5984 = 150u64;
                nat_sub(a5983, a5984)
            });
            let a5986 = v2;
            nat_mul(a5985, a5986)
        }
    } else {
        Ok(0u64)
    }
}

pub fn f213(v0: Adt14, v1: Adt14) -> bool {
    let a5989 = match v0.clone() {
        Adt14::C0(_, h5987, _, _, _, _, _, _, _, _, _) => {
            h5987
        }
    };
    let a5990 = match v1.clone() {
        Adt14::C0(_, h5988, _, _, _, _, _, _, _, _, _) => {
            h5988
        }
    };
    if if if nat_eq(a5989, a5990) {
        let a5993 = match v0.clone() {
            Adt14::C0(_, _, h5991, _, _, _, _, _, _, _, _) => {
                h5991
            }
        };
        let a5994 = match v1.clone() {
            Adt14::C0(_, _, h5992, _, _, _, _, _, _, _, _) => {
                h5992
            }
        };
        nat_eq(a5993, a5994)
    } else {
        false
    } {
        let a5997 = match v0.clone() {
            Adt14::C0(_, _, _, h5995, _, _, _, _, _, _, _) => {
                h5995
            }
        };
        let a5998 = match v1.clone() {
            Adt14::C0(_, _, _, h5996, _, _, _, _, _, _, _) => {
                h5996
            }
        };
        nat_eq(a5997, a5998)
    } else {
        false
    } {
        let a6001 = match v0 {
            Adt14::C0(_, _, _, _, h5999, _, _, _, _, _, _) => {
                h5999
            }
        };
        let a6002 = match v1 {
            Adt14::C0(_, _, _, _, h6000, _, _, _, _, _, _) => {
                h6000
            }
        };
        nat_eq(a6001, a6002)
    } else {
        false
    }
}

pub fn f214(v0: u64, v1: u64, v2: u64, v3: u64) -> bool {
    let a6003 = v0;
    let a6004 = 0u64;
    if if if nat_eq(a6003, a6004) {
        true
    } else {
        let a6005 = v1;
        let a6006 = 0u64;
        nat_eq(a6005, a6006)
    } {
        true
    } else {
        let a6007 = v2;
        let a6008 = 0u64;
        nat_eq(a6007, a6008)
    } {
        true
    } else {
        let a6009 = v3;
        let a6010 = 0u64;
        nat_eq(a6009, a6010)
    }
}

pub fn f215(v0: Adt14, v1: Adt14) -> bool {
    if f221(match v0.clone() {
        Adt14::C0(_, _, _, _, _, h6011, _, _, _, _, _) => {
            h6011
        }
    }, match v1 {
        Adt14::C0(_, _, _, _, _, h6012, _, _, _, _, _) => {
            h6012
        }
    }) {
        let a6014 = match v0 {
            Adt14::C0(_, h6013, _, _, _, _, _, _, _, _, _) => {
                h6013
            }
        };
        let a6015 = 2147483647u64;
        nat_le(a6014, a6015)
    } else {
        true
    }
}

pub fn f216(v0: Adt14) -> bool {
    let a6020 = {
        let a6017 = match v0.clone() {
            Adt14::C0(_, _, _, _, _, _, _, _, _, _, h6016) => {
                h6016
            }
        };
        let a6018 = 4u64;
        let a6019 = 0u64;
        nat_rem(a6017, a6018, a6019)
    };
    let a6021 = 0u64;
    if if nat_eq(a6020, a6021) {
        f277(match v0.clone() {
            Adt14::C0(_, h6022, _, _, _, _, _, _, _, _, _) => {
                h6022
            }
        }, match v0.clone() {
            Adt14::C0(_, _, _, _, _, h6023, _, _, _, _, _) => {
                h6023
            }
        })
    } else {
        false
    } {
        f110(match v0.clone() {
            Adt14::C0(_, _, h6024, _, _, _, _, _, _, _, _) => {
                h6024
            }
        }, match v0.clone() {
            Adt14::C0(_, _, _, h6025, _, _, _, _, _, _, _) => {
                h6025
            }
        }, match v0.clone() {
            Adt14::C0(_, _, _, _, h6026, _, _, _, _, _, _) => {
                h6026
            }
        }, match v0.clone() {
            Adt14::C0(_, _, _, _, _, _, h6027, _, _, _, _) => {
                h6027
            }
        }, match v0.clone() {
            Adt14::C0(_, _, _, _, _, _, _, h6028, _, _, _) => {
                h6028
            }
        }, match v0 {
            Adt14::C0(_, _, _, _, _, _, _, _, h6029, _, _) => {
                h6029
            }
        }, 4u64)
    } else {
        false
    }
}

pub fn f217(v0: Adt14, v1: Adt14) -> R<bool> {
    let a6031 = match v0.clone() {
        Adt14::C0(_, _, _, _, _, _, _, _, _, h6030, _) => {
            h6030
        }
    };
    let a6032 = 0u64;
    if nat_eq(a6031, a6032) {
        Ok(true)
    } else {
        let a6034 = match v1.clone() {
            Adt14::C0(_, _, _, _, _, _, _, _, _, h6033, _) => {
                h6033
            }
        };
        let a6035 = 0u64;
        if nat_eq(a6034, a6035) {
            Ok(true)
        } else {
            let a6037 = match v0.clone() {
                Adt14::C0(_, _, _, _, _, _, _, _, _, _, h6036) => {
                    h6036
                }
            };
            let a6038 = 0u64;
            if nat_eq(a6037, a6038) {
                Ok(true)
            } else {
                let a6040 = match v1.clone() {
                    Adt14::C0(_, _, _, _, _, _, _, _, _, _, h6039) => {
                        h6039
                    }
                };
                let a6041 = 0u64;
                if nat_eq(a6040, a6041) {
                    Ok(true)
                } else {
                    let a6047 = {
                        let a6044 = match v0.clone() {
                            Adt14::C0(_, _, _, _, _, _, _, _, _, _, h6042) => {
                                h6042
                            }
                        };
                        let a6045 = match v0.clone() {
                            Adt14::C0(_, _, _, _, _, _, _, _, _, h6043, _) => {
                                h6043
                            }
                        };
                        nat_add(a6044, a6045)?
                    };
                    let a6048 = match v1.clone() {
                        Adt14::C0(_, _, _, _, _, _, _, _, _, _, h6046) => {
                            h6046
                        }
                    };
                    if nat_le(a6047, a6048) {
                        Ok(true)
                    } else {
                        let a6054 = {
                            let a6051 = match v1.clone() {
                                Adt14::C0(_, _, _, _, _, _, _, _, _, _, h6049) => {
                                    h6049
                                }
                            };
                            let a6052 = match v1 {
                                Adt14::C0(_, _, _, _, _, _, _, _, _, h6050, _) => {
                                    h6050
                                }
                            };
                            nat_add(a6051, a6052)?
                        };
                        let a6055 = match v0 {
                            Adt14::C0(_, _, _, _, _, _, _, _, _, _, h6053) => {
                                h6053
                            }
                        };
                        Ok(nat_le(a6054, a6055))
                    }
                }
            }
        }
    }
}

pub fn f218(v0: Adt14, v1: Adt14) -> bool {
    if if if if if if f213(v0.clone(), v1.clone()) {
        let a6058 = match v0.clone() {
            Adt14::C0(h6056, _, _, _, _, _, _, _, _, _, _) => {
                h6056
            }
        };
        let a6059 = match v1.clone() {
            Adt14::C0(h6057, _, _, _, _, _, _, _, _, _, _) => {
                h6057
            }
        };
        nat_eq(a6058, a6059)
    } else {
        false
    } {
        let a6062 = match v0.clone() {
            Adt14::C0(_, _, _, _, _, _, _, _, _, _, h6060) => {
                h6060
            }
        };
        let a6063 = match v1.clone() {
            Adt14::C0(_, _, _, _, _, _, _, _, _, _, h6061) => {
                h6061
            }
        };
        nat_eq(a6062, a6063)
    } else {
        false
    } {
        let a6066 = match v0.clone() {
            Adt14::C0(_, _, _, _, _, h6064, _, _, _, _, _) => {
                h6064
            }
        };
        let a6067 = match v1.clone() {
            Adt14::C0(_, _, _, _, _, h6065, _, _, _, _, _) => {
                h6065
            }
        };
        nat_eq(a6066, a6067)
    } else {
        false
    } {
        let a6070 = match v0.clone() {
            Adt14::C0(_, _, _, _, _, _, h6068, _, _, _, _) => {
                h6068
            }
        };
        let a6071 = match v1.clone() {
            Adt14::C0(_, _, _, _, _, _, h6069, _, _, _, _) => {
                h6069
            }
        };
        nat_eq(a6070, a6071)
    } else {
        false
    } {
        let a6074 = match v0.clone() {
            Adt14::C0(_, _, _, _, _, _, _, h6072, _, _, _) => {
                h6072
            }
        };
        let a6075 = match v1.clone() {
            Adt14::C0(_, _, _, _, _, _, _, h6073, _, _, _) => {
                h6073
            }
        };
        nat_eq(a6074, a6075)
    } else {
        false
    } {
        let a6078 = match v0 {
            Adt14::C0(_, _, _, _, _, _, _, _, h6076, _, _) => {
                h6076
            }
        };
        let a6079 = match v1 {
            Adt14::C0(_, _, _, _, _, _, _, _, h6077, _, _) => {
                h6077
            }
        };
        nat_eq(a6078, a6079)
    } else {
        false
    }
}

pub fn f219(v0: Adt14) -> R<bool> {
    if f278(match v0.clone() {
        Adt14::C0(_, h6080, _, _, _, _, _, _, _, _, _) => {
            h6080
        }
    }, match v0.clone() {
        Adt14::C0(_, _, _, _, _, h6081, _, _, _, _, _) => {
            h6081
        }
    }) {
        f111(f279(match v0.clone() {
            Adt14::C0(_, h6082, _, _, _, _, _, _, _, _, _) => {
                h6082
            }
        }, match v0.clone() {
            Adt14::C0(_, _, _, _, _, h6083, _, _, _, _, _) => {
                h6083
            }
        })?, match v0.clone() {
            Adt14::C0(_, _, h6084, _, _, _, _, _, _, _, _) => {
                h6084
            }
        }, match v0.clone() {
            Adt14::C0(_, _, _, h6085, _, _, _, _, _, _, _) => {
                h6085
            }
        }, match v0.clone() {
            Adt14::C0(_, _, _, _, h6086, _, _, _, _, _, _) => {
                h6086
            }
        }, match v0.clone() {
            Adt14::C0(_, _, _, _, _, _, h6087, _, _, _, _) => {
                h6087
            }
        }, match v0.clone() {
            Adt14::C0(_, _, _, _, _, _, _, h6088, _, _, _) => {
                h6088
            }
        }, match v0 {
            Adt14::C0(_, _, _, _, _, _, _, _, h6089, _, _) => {
                h6089
            }
        })
    } else {
        Ok(false)
    }
}

pub fn f220(v0: bool, v1: u64, v2: u64, v3: u64) -> R<u64> {
    let a6090 = v1;
    let a6091 = 0u64;
    if if nat_eq(a6090, a6091) {
        true
    } else {
        let a6092 = v2;
        let a6093 = 0u64;
        nat_eq(a6092, a6093)
    } {
        Ok(0u64)
    } else {
        let a6096 = f112(v1, v2, v3)?;
        let a6097 = {
            let a6094 = v2;
            let a6095 = f280(v0, v1)?;
            nat_mul(a6094, a6095)?
        };
        nat_add(a6096, a6097)
    }
}

pub fn f221(v0: u64, v1: u64) -> bool {
    let a6098 = v0;
    let a6099 = 4u64;
    if nat_eq(a6098, a6099) {
        let a6100 = v1;
        let a6101 = 4u64;
        nat_eq(a6100, a6101)
    } else {
        false
    }
}

pub fn f222(v0: u64) -> R<u64> {
    let a6102 = 6u64;
    let a6103 = v0;
    nat_add(a6102, a6103)
}

pub fn f223(v0: Adt14) -> Adt14 {
    Adt14::C0(match v0.clone() {
        Adt14::C0(h6104, _, _, _, _, _, _, _, _, _, _) => {
            h6104
        }
    }, match v0.clone() {
        Adt14::C0(_, h6105, _, _, _, _, _, _, _, _, _) => {
            h6105
        }
    }, match v0.clone() {
        Adt14::C0(_, _, h6106, _, _, _, _, _, _, _, _) => {
            h6106
        }
    }, match v0.clone() {
        Adt14::C0(_, _, _, h6107, _, _, _, _, _, _, _) => {
            h6107
        }
    }, match v0.clone() {
        Adt14::C0(_, _, _, _, h6108, _, _, _, _, _, _) => {
            h6108
        }
    }, 4u64, match v0.clone() {
        Adt14::C0(_, _, _, _, _, _, h6109, _, _, _, _) => {
            h6109
        }
    }, match v0.clone() {
        Adt14::C0(_, _, _, _, _, _, _, h6110, _, _, _) => {
            h6110
        }
    }, match v0.clone() {
        Adt14::C0(_, _, _, _, _, _, _, _, h6111, _, _) => {
            h6111
        }
    }, match v0.clone() {
        Adt14::C0(_, _, _, _, _, _, _, _, _, h6112, _) => {
            h6112
        }
    }, match v0 {
        Adt14::C0(_, _, _, _, _, _, _, _, _, _, h6113) => {
            h6113
        }
    })
}

pub fn f224(v0: u64, v1: u64, v2: u64, v3: u64) -> R<bool> {
    let a6114 = v0;
    let a6115 = 2147483647u64;
    if if if nat_le(a6114, a6115) {
        let a6116 = 0u64;
        let a6117 = v3;
        if nat_lt(a6116, a6117) {
            let a6118 = v3;
            let a6119 = 2147483647u64;
            nat_le(a6118, a6119)
        } else {
            false
        }
    } else {
        false
    } {
        let a6124 = {
            let a6122 = {
                let a6120 = v2;
                let a6121 = v3;
                nat_add(a6120, a6121)?
            };
            let a6123 = 1u64;
            nat_sub(a6122, a6123)
        };
        let a6125 = 2147483647u64;
        nat_le(a6124, a6125)
    } else {
        false
    } {
        let a6130 = {
            let a6128 = {
                let a6126 = v0;
                let a6127 = v1;
                nat_mul(a6126, a6127)?
            };
            let a6129 = f281(v2, v3)?;
            nat_mul(a6128, a6129)?
        };
        let a6131 = 9223372036854775807u64;
        Ok(nat_le(a6130, a6131))
    } else {
        Ok(false)
    }
}

pub fn f225(v0: Adt14) -> R<u64> {
    let a6162 = {
        let a6154 = {
            let a6146 = {
                let a6138 = 4u64;
                let a6139 = {
                    let a6136 = {
                        let a6133 = match v0.clone() {
                            Adt14::C0(_, h6132, _, _, _, _, _, _, _, _, _) => {
                                h6132
                            }
                        };
                        let a6134 = 1u64;
                        nat_sub(a6133, a6134)
                    };
                    let a6137 = match v0.clone() {
                        Adt14::C0(_, _, _, _, _, h6135, _, _, _, _, _) => {
                            h6135
                        }
                    };
                    nat_mul(a6136, a6137)?
                };
                nat_add(a6138, a6139)?
            };
            let a6147 = {
                let a6144 = {
                    let a6141 = match v0.clone() {
                        Adt14::C0(_, _, h6140, _, _, _, _, _, _, _, _) => {
                            h6140
                        }
                    };
                    let a6142 = 1u64;
                    nat_sub(a6141, a6142)
                };
                let a6145 = match v0.clone() {
                    Adt14::C0(_, _, _, _, _, _, h6143, _, _, _, _) => {
                        h6143
                    }
                };
                nat_mul(a6144, a6145)?
            };
            nat_add(a6146, a6147)?
        };
        let a6155 = {
            let a6152 = {
                let a6149 = match v0.clone() {
                    Adt14::C0(_, _, _, h6148, _, _, _, _, _, _, _) => {
                        h6148
                    }
                };
                let a6150 = 1u64;
                nat_sub(a6149, a6150)
            };
            let a6153 = match v0.clone() {
                Adt14::C0(_, _, _, _, _, _, _, h6151, _, _, _) => {
                    h6151
                }
            };
            nat_mul(a6152, a6153)?
        };
        nat_add(a6154, a6155)?
    };
    let a6163 = {
        let a6160 = {
            let a6157 = match v0.clone() {
                Adt14::C0(_, _, _, _, h6156, _, _, _, _, _, _) => {
                    h6156
                }
            };
            let a6158 = 1u64;
            nat_sub(a6157, a6158)
        };
        let a6161 = match v0 {
            Adt14::C0(_, _, _, _, _, _, _, _, h6159, _, _) => {
                h6159
            }
        };
        nat_mul(a6160, a6161)?
    };
    nat_add(a6162, a6163)
}

pub fn f226(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64) -> R<u64> {
    let a6164 = v0;
    let a6165 = 0u64;
    if if if if nat_eq(a6164, a6165) {
        true
    } else {
        let a6166 = v1;
        let a6167 = 0u64;
        nat_eq(a6166, a6167)
    } {
        true
    } else {
        let a6168 = v2;
        let a6169 = 0u64;
        nat_eq(a6168, a6169)
    } {
        true
    } else {
        let a6170 = v3;
        let a6171 = 0u64;
        nat_eq(a6170, a6171)
    } {
        Ok(0u64)
    } else {
        let a6172 = f282(v0, v1, v2, v3, v5)?;
        let a6173 = f283(v0, v1, v2, v3, v4)?;
        nat_add(a6172, a6173)
    }
}

pub fn f227(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let a6174 = v0;
    let a6175 = 0u64;
    if if nat_eq(a6174, a6175) {
        true
    } else {
        let a6176 = v1;
        let a6177 = 0u64;
        nat_eq(a6176, a6177)
    } {
        Ok(0u64)
    } else {
        let a6180 = f112(v0, v1, v2)?;
        let a6181 = {
            let a6178 = v1;
            let a6179 = f284(v0)?;
            nat_mul(a6178, a6179)?
        };
        nat_add(a6180, a6181)
    }
}

pub fn f228(v0: u64, v1: u64, v2: u64) -> bool {
    let a6182 = v0;
    let a6183 = 1u64;
    if nat_le(a6182, a6183) {
        true
    } else {
        let a6187 = {
            let a6184 = v1;
            let a6185 = v2;
            let a6186 = 1u64;
            nat_rem(a6184, a6185, a6186)
        };
        let a6188 = 0u64;
        nat_eq(a6187, a6188)
    }
}

pub fn f229(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64) -> R<bool> {
    let a6189 = v1;
    let a6190 = 1u64;
    if if if nat_le(a6189, a6190) {
        true
    } else {
        let a6191 = v0;
        let a6192 = v2;
        nat_le(a6191, a6192)
    } {
        let a6193 = v3;
        let a6194 = 1u64;
        if nat_le(a6193, a6194) {
            true
        } else {
            let a6201 = {
                let a6199 = v0;
                let a6200 = {
                    let a6197 = {
                        let a6195 = v1;
                        let a6196 = 1u64;
                        nat_sub(a6195, a6196)
                    };
                    let a6198 = v2;
                    nat_mul(a6197, a6198)?
                };
                nat_add(a6199, a6200)?
            };
            let a6202 = v4;
            nat_le(a6201, a6202)
        }
    } else {
        false
    } {
        let a6203 = v5;
        let a6204 = 1u64;
        if nat_le(a6203, a6204) {
            Ok(true)
        } else {
            let a6217 = {
                let a6215 = {
                    let a6209 = v0;
                    let a6210 = {
                        let a6207 = {
                            let a6205 = v1;
                            let a6206 = 1u64;
                            nat_sub(a6205, a6206)
                        };
                        let a6208 = v2;
                        nat_mul(a6207, a6208)?
                    };
                    nat_add(a6209, a6210)?
                };
                let a6216 = {
                    let a6213 = {
                        let a6211 = v3;
                        let a6212 = 1u64;
                        nat_sub(a6211, a6212)
                    };
                    let a6214 = v4;
                    nat_mul(a6213, a6214)?
                };
                nat_add(a6215, a6216)?
            };
            let a6218 = v6;
            Ok(nat_le(a6217, a6218))
        }
    } else {
        Ok(false)
    }
}

pub fn f230(v0: u64) -> Adt18 {
    let a6219 = v0;
    let a6220 = 0u64;
    if nat_eq(a6219, a6220) {
        Adt18::C1(Adt19::C0(0u64, 1u64, 4u64))
    } else {
        let a6221 = v0;
        let a6222 = 1u64;
        if nat_eq(a6221, a6222) {
            Adt18::C1(Adt19::C0(1u64, 1u64, 2u64))
        } else {
            let a6223 = v0;
            let a6224 = 2u64;
            if nat_eq(a6223, a6224) {
                Adt18::C1(Adt19::C0(2u64, 32u64, 18u64))
            } else {
                let a6225 = v0;
                let a6226 = 3u64;
                if nat_eq(a6225, a6226) {
                    Adt18::C1(Adt19::C0(3u64, 32u64, 20u64))
                } else {
                    let a6227 = v0;
                    let a6228 = 6u64;
                    if nat_eq(a6227, a6228) {
                        Adt18::C1(Adt19::C0(6u64, 32u64, 22u64))
                    } else {
                        let a6229 = v0;
                        let a6230 = 7u64;
                        if nat_eq(a6229, a6230) {
                            Adt18::C1(Adt19::C0(7u64, 32u64, 24u64))
                        } else {
                            let a6231 = v0;
                            let a6232 = 8u64;
                            if nat_eq(a6231, a6232) {
                                Adt18::C1(Adt19::C0(8u64, 32u64, 34u64))
                            } else {
                                let a6233 = v0;
                                let a6234 = 9u64;
                                if nat_eq(a6233, a6234) {
                                    Adt18::C1(Adt19::C0(9u64, 32u64, 36u64))
                                } else {
                                    let a6235 = v0;
                                    let a6236 = 10u64;
                                    if nat_eq(a6235, a6236) {
                                        Adt18::C1(Adt19::C0(10u64, 256u64, 84u64))
                                    } else {
                                        let a6237 = v0;
                                        let a6238 = 11u64;
                                        if nat_eq(a6237, a6238) {
                                            Adt18::C1(Adt19::C0(11u64, 256u64, 110u64))
                                        } else {
                                            let a6239 = v0;
                                            let a6240 = 12u64;
                                            if nat_eq(a6239, a6240) {
                                                Adt18::C1(Adt19::C0(12u64, 256u64, 144u64))
                                            } else {
                                                let a6241 = v0;
                                                let a6242 = 13u64;
                                                if nat_eq(a6241, a6242) {
                                                    Adt18::C1(Adt19::C0(13u64, 256u64, 176u64))
                                                } else {
                                                    let a6243 = v0;
                                                    let a6244 = 14u64;
                                                    if nat_eq(a6243, a6244) {
                                                        Adt18::C1(Adt19::C0(14u64, 256u64, 210u64))
                                                    } else {
                                                        let a6245 = v0;
                                                        let a6246 = 15u64;
                                                        if nat_eq(a6245, a6246) {
                                                            Adt18::C1(Adt19::C0(15u64, 256u64, 292u64))
                                                        } else {
                                                            let a6247 = v0;
                                                            let a6248 = 16u64;
                                                            if nat_eq(a6247, a6248) {
                                                                Adt18::C1(Adt19::C0(16u64, 256u64, 66u64))
                                                            } else {
                                                                let a6249 = v0;
                                                                let a6250 = 17u64;
                                                                if nat_eq(a6249, a6250) {
                                                                    Adt18::C1(Adt19::C0(17u64, 256u64, 74u64))
                                                                } else {
                                                                    let a6251 = v0;
                                                                    let a6252 = 18u64;
                                                                    if nat_eq(a6251, a6252) {
                                                                        Adt18::C1(Adt19::C0(18u64, 256u64, 98u64))
                                                                    } else {
                                                                        let a6253 = v0;
                                                                        let a6254 = 19u64;
                                                                        if nat_eq(a6253, a6254) {
                                                                            Adt18::C1(Adt19::C0(19u64, 256u64, 50u64))
                                                                        } else {
                                                                            let a6255 = v0;
                                                                            let a6256 = 20u64;
                                                                            if nat_eq(a6255, a6256) {
                                                                                Adt18::C1(Adt19::C0(20u64, 32u64, 18u64))
                                                                            } else {
                                                                                let a6257 = v0;
                                                                                let a6258 = 21u64;
                                                                                if nat_eq(a6257, a6258) {
                                                                                    Adt18::C1(Adt19::C0(21u64, 256u64, 110u64))
                                                                                } else {
                                                                                    let a6259 = v0;
                                                                                    let a6260 = 22u64;
                                                                                    if nat_eq(a6259, a6260) {
                                                                                        Adt18::C1(Adt19::C0(22u64, 256u64, 82u64))
                                                                                    } else {
                                                                                        let a6261 = v0;
                                                                                        let a6262 = 23u64;
                                                                                        if nat_eq(a6261, a6262) {
                                                                                            Adt18::C1(Adt19::C0(23u64, 256u64, 136u64))
                                                                                        } else {
                                                                                            let a6263 = v0;
                                                                                            let a6264 = 24u64;
                                                                                            if nat_eq(a6263, a6264) {
                                                                                                Adt18::C1(Adt19::C0(24u64, 1u64, 1u64))
                                                                                            } else {
                                                                                                let a6265 = v0;
                                                                                                let a6266 = 25u64;
                                                                                                if nat_eq(a6265, a6266) {
                                                                                                    Adt18::C1(Adt19::C0(25u64, 1u64, 2u64))
                                                                                                } else {
                                                                                                    let a6267 = v0;
                                                                                                    let a6268 = 26u64;
                                                                                                    if nat_eq(a6267, a6268) {
                                                                                                        Adt18::C1(Adt19::C0(26u64, 1u64, 4u64))
                                                                                                    } else {
                                                                                                        let a6269 = v0;
                                                                                                        let a6270 = 27u64;
                                                                                                        if nat_eq(a6269, a6270) {
                                                                                                            Adt18::C1(Adt19::C0(27u64, 1u64, 8u64))
                                                                                                        } else {
                                                                                                            let a6271 = v0;
                                                                                                            let a6272 = 28u64;
                                                                                                            if nat_eq(a6271, a6272) {
                                                                                                                Adt18::C1(Adt19::C0(28u64, 1u64, 8u64))
                                                                                                            } else {
                                                                                                                let a6273 = v0;
                                                                                                                let a6274 = 29u64;
                                                                                                                if nat_eq(a6273, a6274) {
                                                                                                                    Adt18::C1(Adt19::C0(29u64, 256u64, 56u64))
                                                                                                                } else {
                                                                                                                    let a6275 = v0;
                                                                                                                    let a6276 = 30u64;
                                                                                                                    if nat_eq(a6275, a6276) {
                                                                                                                        Adt18::C1(Adt19::C0(30u64, 1u64, 2u64))
                                                                                                                    } else {
                                                                                                                        let a6277 = v0;
                                                                                                                        let a6278 = 34u64;
                                                                                                                        if nat_eq(a6277, a6278) {
                                                                                                                            Adt18::C1(Adt19::C0(34u64, 256u64, 54u64))
                                                                                                                        } else {
                                                                                                                            let a6279 = v0;
                                                                                                                            let a6280 = 35u64;
                                                                                                                            if nat_eq(a6279, a6280) {
                                                                                                                                Adt18::C1(Adt19::C0(35u64, 256u64, 66u64))
                                                                                                                            } else {
                                                                                                                                let a6281 = v0;
                                                                                                                                let a6282 = 39u64;
                                                                                                                                if nat_eq(a6281, a6282) {
                                                                                                                                    Adt18::C1(Adt19::C0(39u64, 32u64, 17u64))
                                                                                                                                } else {
                                                                                                                                    let a6283 = v0;
                                                                                                                                    let a6284 = 40u64;
                                                                                                                                    if nat_eq(a6283, a6284) {
                                                                                                                                        Adt18::C1(Adt19::C0(40u64, 64u64, 36u64))
                                                                                                                                    } else {
                                                                                                                                        let a6285 = v0;
                                                                                                                                        let a6286 = 41u64;
                                                                                                                                        if nat_eq(a6285, a6286) {
                                                                                                                                            Adt18::C1(Adt19::C0(41u64, 128u64, 18u64))
                                                                                                                                        } else {
                                                                                                                                            let a6287 = v0;
                                                                                                                                            let a6288 = 42u64;
                                                                                                                                            if nat_eq(a6287, a6288) {
                                                                                                                                                Adt18::C1(Adt19::C0(42u64, 64u64, 18u64))
                                                                                                                                            } else {
                                                                                                                                                let a6289 = v0;
                                                                                                                                                let a6290 = 43u64;
                                                                                                                                                if nat_eq(a6289, a6290) {
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

pub fn f231(v0: u64, v1: u64) -> R<u64> {
    let v2: u64 = {
        let a6291 = v0;
        let a6292 = 2u64;
        let a6293 = 0u64;
        nat_quot(a6291, a6292, a6293)
    };
    let a6309 = {
        let a6294 = v2;
        let a6295 = 0u64;
        if nat_eq(a6294, a6295) {
            0u64
        } else {
            let a6302 = 7u64;
            let a6303 = {
                let a6300 = v2;
                let a6301 = {
                    let a6298 = 5u64;
                    let a6299 = {
                        let a6296 = 3u64;
                        let a6297 = v1;
                        nat_mul(a6296, a6297)?
                    };
                    nat_add(a6298, a6299)?
                };
                nat_mul(a6300, a6301)?
            };
            nat_add(a6302, a6303)?
        }
    };
    let a6310 = {
        let a6307 = {
            let a6304 = v0;
            let a6305 = 2u64;
            let a6306 = 0u64;
            nat_rem(a6304, a6305, a6306)
        };
        let a6308 = 1u64;
        if nat_eq(a6307, a6308) {
            v1
        } else {
            0u64
        }
    };
    nat_add(a6309, a6310)
}

pub fn f232(v0: Adt2, v1: u64, v2: bool, v3: bool) -> R<bool> {
    if v2 {
        if v3 {
            let a6312 = match v0.clone() {
                Adt2::C0(_, _, _, _, h6311, _, _, _, _, _, _) => {
                    h6311
                }
            };
            let a6313 = 1u64;
            if if if if if nat_le(a6312, a6313) {
                true
            } else {
                let a6315 = match v0.clone() {
                    Adt2::C0(_, _, _, _, _, _, _, _, h6314, _, _) => {
                        h6314
                    }
                };
                let a6316 = v1;
                nat_eq(a6315, a6316)
            } {
                let a6318 = match v0.clone() {
                    Adt2::C0(_, h6317, _, _, _, _, _, _, _, _, _) => {
                        h6317
                    }
                };
                let a6319 = 1u64;
                if nat_le(a6318, a6319) {
                    true
                } else {
                    let a6324 = match v0.clone() {
                        Adt2::C0(_, _, _, _, _, h6320, _, _, _, _, _) => {
                            h6320
                        }
                    };
                    let a6325 = {
                        let a6322 = v1;
                        let a6323 = match v0.clone() {
                            Adt2::C0(_, _, _, _, h6321, _, _, _, _, _, _) => {
                                h6321
                            }
                        };
                        nat_mul(a6322, a6323)?
                    };
                    nat_eq(a6324, a6325)
                }
            } else {
                false
            } {
                let a6327 = match v0.clone() {
                    Adt2::C0(_, _, h6326, _, _, _, _, _, _, _, _) => {
                        h6326
                    }
                };
                let a6328 = 1u64;
                if nat_le(a6327, a6328) {
                    true
                } else {
                    let a6336 = match v0.clone() {
                        Adt2::C0(_, _, _, _, _, _, h6329, _, _, _, _) => {
                            h6329
                        }
                    };
                    let a6337 = {
                        let a6334 = {
                            let a6331 = v1;
                            let a6332 = match v0.clone() {
                                Adt2::C0(_, _, _, _, h6330, _, _, _, _, _, _) => {
                                    h6330
                                }
                            };
                            nat_mul(a6331, a6332)?
                        };
                        let a6335 = match v0.clone() {
                            Adt2::C0(_, h6333, _, _, _, _, _, _, _, _, _) => {
                                h6333
                            }
                        };
                        nat_mul(a6334, a6335)?
                    };
                    nat_eq(a6336, a6337)
                }
            } else {
                false
            } {
                let a6339 = match v0.clone() {
                    Adt2::C0(_, _, _, h6338, _, _, _, _, _, _, _) => {
                        h6338
                    }
                };
                let a6340 = 1u64;
                if nat_le(a6339, a6340) {
                    true
                } else {
                    let a6351 = match v0.clone() {
                        Adt2::C0(_, _, _, _, _, _, _, h6341, _, _, _) => {
                            h6341
                        }
                    };
                    let a6352 = {
                        let a6349 = {
                            let a6346 = {
                                let a6343 = v1;
                                let a6344 = match v0.clone() {
                                    Adt2::C0(_, _, _, _, h6342, _, _, _, _, _, _) => {
                                        h6342
                                    }
                                };
                                nat_mul(a6343, a6344)?
                            };
                            let a6347 = match v0.clone() {
                                Adt2::C0(_, h6345, _, _, _, _, _, _, _, _, _) => {
                                    h6345
                                }
                            };
                            nat_mul(a6346, a6347)?
                        };
                        let a6350 = match v0.clone() {
                            Adt2::C0(_, _, h6348, _, _, _, _, _, _, _, _) => {
                                h6348
                            }
                        };
                        nat_mul(a6349, a6350)?
                    };
                    nat_eq(a6351, a6352)
                }
            } else {
                false
            } {
                let a6366 = {
                    let a6363 = {
                        let a6360 = {
                            let a6357 = {
                                let a6354 = v1;
                                let a6355 = match v0.clone() {
                                    Adt2::C0(_, _, _, _, h6353, _, _, _, _, _, _) => {
                                        h6353
                                    }
                                };
                                nat_mul(a6354, a6355)?
                            };
                            let a6358 = match v0.clone() {
                                Adt2::C0(_, h6356, _, _, _, _, _, _, _, _, _) => {
                                    h6356
                                }
                            };
                            nat_mul(a6357, a6358)?
                        };
                        let a6361 = match v0.clone() {
                            Adt2::C0(_, _, h6359, _, _, _, _, _, _, _, _) => {
                                h6359
                            }
                        };
                        nat_mul(a6360, a6361)?
                    };
                    let a6364 = match v0.clone() {
                        Adt2::C0(_, _, _, h6362, _, _, _, _, _, _, _) => {
                            h6362
                        }
                    };
                    nat_mul(a6363, a6364)?
                };
                let a6367 = match v0 {
                    Adt2::C0(_, _, _, _, _, _, _, _, _, h6365, _) => {
                        h6365
                    }
                };
                Ok(nat_le(a6366, a6367))
            } else {
                Ok(false)
            }
        } else {
            let a6369 = match v0.clone() {
                Adt2::C0(_, _, _, h6368, _, _, _, _, _, _, _) => {
                    h6368
                }
            };
            let a6370 = 1u64;
            if if if if if nat_le(a6369, a6370) {
                true
            } else {
                let a6372 = match v0.clone() {
                    Adt2::C0(_, _, _, _, _, _, _, h6371, _, _, _) => {
                        h6371
                    }
                };
                let a6373 = v1;
                nat_eq(a6372, a6373)
            } {
                let a6375 = match v0.clone() {
                    Adt2::C0(_, h6374, _, _, _, _, _, _, _, _, _) => {
                        h6374
                    }
                };
                let a6376 = 1u64;
                if nat_le(a6375, a6376) {
                    true
                } else {
                    let a6381 = match v0.clone() {
                        Adt2::C0(_, _, _, _, _, h6377, _, _, _, _, _) => {
                            h6377
                        }
                    };
                    let a6382 = {
                        let a6379 = v1;
                        let a6380 = match v0.clone() {
                            Adt2::C0(_, _, _, h6378, _, _, _, _, _, _, _) => {
                                h6378
                            }
                        };
                        nat_mul(a6379, a6380)?
                    };
                    nat_eq(a6381, a6382)
                }
            } else {
                false
            } {
                let a6384 = match v0.clone() {
                    Adt2::C0(_, _, h6383, _, _, _, _, _, _, _, _) => {
                        h6383
                    }
                };
                let a6385 = 1u64;
                if nat_le(a6384, a6385) {
                    true
                } else {
                    let a6393 = match v0.clone() {
                        Adt2::C0(_, _, _, _, _, _, h6386, _, _, _, _) => {
                            h6386
                        }
                    };
                    let a6394 = {
                        let a6391 = {
                            let a6388 = v1;
                            let a6389 = match v0.clone() {
                                Adt2::C0(_, _, _, h6387, _, _, _, _, _, _, _) => {
                                    h6387
                                }
                            };
                            nat_mul(a6388, a6389)?
                        };
                        let a6392 = match v0.clone() {
                            Adt2::C0(_, h6390, _, _, _, _, _, _, _, _, _) => {
                                h6390
                            }
                        };
                        nat_mul(a6391, a6392)?
                    };
                    nat_eq(a6393, a6394)
                }
            } else {
                false
            } {
                let a6396 = match v0.clone() {
                    Adt2::C0(_, _, _, _, h6395, _, _, _, _, _, _) => {
                        h6395
                    }
                };
                let a6397 = 1u64;
                if nat_le(a6396, a6397) {
                    true
                } else {
                    let a6408 = match v0.clone() {
                        Adt2::C0(_, _, _, _, _, _, _, _, h6398, _, _) => {
                            h6398
                        }
                    };
                    let a6409 = {
                        let a6406 = {
                            let a6403 = {
                                let a6400 = v1;
                                let a6401 = match v0.clone() {
                                    Adt2::C0(_, _, _, h6399, _, _, _, _, _, _, _) => {
                                        h6399
                                    }
                                };
                                nat_mul(a6400, a6401)?
                            };
                            let a6404 = match v0.clone() {
                                Adt2::C0(_, h6402, _, _, _, _, _, _, _, _, _) => {
                                    h6402
                                }
                            };
                            nat_mul(a6403, a6404)?
                        };
                        let a6407 = match v0.clone() {
                            Adt2::C0(_, _, h6405, _, _, _, _, _, _, _, _) => {
                                h6405
                            }
                        };
                        nat_mul(a6406, a6407)?
                    };
                    nat_eq(a6408, a6409)
                }
            } else {
                false
            } {
                let a6423 = {
                    let a6420 = {
                        let a6417 = {
                            let a6414 = {
                                let a6411 = v1;
                                let a6412 = match v0.clone() {
                                    Adt2::C0(_, _, _, h6410, _, _, _, _, _, _, _) => {
                                        h6410
                                    }
                                };
                                nat_mul(a6411, a6412)?
                            };
                            let a6415 = match v0.clone() {
                                Adt2::C0(_, h6413, _, _, _, _, _, _, _, _, _) => {
                                    h6413
                                }
                            };
                            nat_mul(a6414, a6415)?
                        };
                        let a6418 = match v0.clone() {
                            Adt2::C0(_, _, h6416, _, _, _, _, _, _, _, _) => {
                                h6416
                            }
                        };
                        nat_mul(a6417, a6418)?
                    };
                    let a6421 = match v0.clone() {
                        Adt2::C0(_, _, _, _, h6419, _, _, _, _, _, _) => {
                            h6419
                        }
                    };
                    nat_mul(a6420, a6421)?
                };
                let a6424 = match v0 {
                    Adt2::C0(_, _, _, _, _, _, _, _, _, h6422, _) => {
                        h6422
                    }
                };
                Ok(nat_le(a6423, a6424))
            } else {
                Ok(false)
            }
        }
    } else {
        let a6426 = match v0.clone() {
            Adt2::C0(_, h6425, _, _, _, _, _, _, _, _, _) => {
                h6425
            }
        };
        let a6427 = 1u64;
        if if if if if nat_le(a6426, a6427) {
            true
        } else {
            let a6429 = match v0.clone() {
                Adt2::C0(_, _, _, _, _, h6428, _, _, _, _, _) => {
                    h6428
                }
            };
            let a6430 = v1;
            nat_eq(a6429, a6430)
        } {
            let a6432 = match v0.clone() {
                Adt2::C0(_, _, h6431, _, _, _, _, _, _, _, _) => {
                    h6431
                }
            };
            let a6433 = 1u64;
            if nat_le(a6432, a6433) {
                true
            } else {
                let a6438 = match v0.clone() {
                    Adt2::C0(_, _, _, _, _, _, h6434, _, _, _, _) => {
                        h6434
                    }
                };
                let a6439 = {
                    let a6436 = v1;
                    let a6437 = match v0.clone() {
                        Adt2::C0(_, h6435, _, _, _, _, _, _, _, _, _) => {
                            h6435
                        }
                    };
                    nat_mul(a6436, a6437)?
                };
                nat_eq(a6438, a6439)
            }
        } else {
            false
        } {
            let a6441 = match v0.clone() {
                Adt2::C0(_, _, _, h6440, _, _, _, _, _, _, _) => {
                    h6440
                }
            };
            let a6442 = 1u64;
            if nat_le(a6441, a6442) {
                true
            } else {
                let a6450 = match v0.clone() {
                    Adt2::C0(_, _, _, _, _, _, _, h6443, _, _, _) => {
                        h6443
                    }
                };
                let a6451 = {
                    let a6448 = {
                        let a6445 = v1;
                        let a6446 = match v0.clone() {
                            Adt2::C0(_, h6444, _, _, _, _, _, _, _, _, _) => {
                                h6444
                            }
                        };
                        nat_mul(a6445, a6446)?
                    };
                    let a6449 = match v0.clone() {
                        Adt2::C0(_, _, h6447, _, _, _, _, _, _, _, _) => {
                            h6447
                        }
                    };
                    nat_mul(a6448, a6449)?
                };
                nat_eq(a6450, a6451)
            }
        } else {
            false
        } {
            let a6453 = match v0.clone() {
                Adt2::C0(_, _, _, _, h6452, _, _, _, _, _, _) => {
                    h6452
                }
            };
            let a6454 = 1u64;
            if nat_le(a6453, a6454) {
                true
            } else {
                let a6465 = match v0.clone() {
                    Adt2::C0(_, _, _, _, _, _, _, _, h6455, _, _) => {
                        h6455
                    }
                };
                let a6466 = {
                    let a6463 = {
                        let a6460 = {
                            let a6457 = v1;
                            let a6458 = match v0.clone() {
                                Adt2::C0(_, h6456, _, _, _, _, _, _, _, _, _) => {
                                    h6456
                                }
                            };
                            nat_mul(a6457, a6458)?
                        };
                        let a6461 = match v0.clone() {
                            Adt2::C0(_, _, h6459, _, _, _, _, _, _, _, _) => {
                                h6459
                            }
                        };
                        nat_mul(a6460, a6461)?
                    };
                    let a6464 = match v0.clone() {
                        Adt2::C0(_, _, _, h6462, _, _, _, _, _, _, _) => {
                            h6462
                        }
                    };
                    nat_mul(a6463, a6464)?
                };
                nat_eq(a6465, a6466)
            }
        } else {
            false
        } {
            let a6480 = {
                let a6477 = {
                    let a6474 = {
                        let a6471 = {
                            let a6468 = v1;
                            let a6469 = match v0.clone() {
                                Adt2::C0(_, h6467, _, _, _, _, _, _, _, _, _) => {
                                    h6467
                                }
                            };
                            nat_mul(a6468, a6469)?
                        };
                        let a6472 = match v0.clone() {
                            Adt2::C0(_, _, h6470, _, _, _, _, _, _, _, _) => {
                                h6470
                            }
                        };
                        nat_mul(a6471, a6472)?
                    };
                    let a6475 = match v0.clone() {
                        Adt2::C0(_, _, _, h6473, _, _, _, _, _, _, _) => {
                            h6473
                        }
                    };
                    nat_mul(a6474, a6475)?
                };
                let a6478 = match v0.clone() {
                    Adt2::C0(_, _, _, _, h6476, _, _, _, _, _, _) => {
                        h6476
                    }
                };
                nat_mul(a6477, a6478)?
            };
            let a6481 = match v0 {
                Adt2::C0(_, _, _, _, _, _, _, _, _, h6479, _) => {
                    h6479
                }
            };
            Ok(nat_le(a6480, a6481))
        } else {
            Ok(false)
        }
    }
}

pub fn f233(v0: Adt2, v1: u64) -> R<bool> {
    let a6483 = match v0.clone() {
        Adt2::C0(h6482, _, _, _, _, _, _, _, _, _, _) => {
            h6482
        }
    };
    let a6484 = 0u64;
    if if nat_eq(a6483, a6484) {
        f43(v0.clone())?
    } else {
        false
    } {
        let a6485 = v1;
        let a6486 = 1u64;
        if nat_le(a6485, a6486) {
            Ok(true)
        } else {
            let a6491 = match v0.clone() {
                Adt2::C0(_, _, _, _, _, _, h6487, _, _, _, _) => {
                    h6487
                }
            };
            let a6492 = {
                let a6489 = 4u64;
                let a6490 = match v0 {
                    Adt2::C0(_, h6488, _, _, _, _, _, _, _, _, _) => {
                        h6488
                    }
                };
                nat_mul(a6489, a6490)?
            };
            Ok(nat_eq(a6491, a6492))
        }
    } else {
        Ok(false)
    }
}

pub fn f234(v0: Adt2, v1: u64) -> R<bool> {
    let a6494 = match v0.clone() {
        Adt2::C0(_, _, _, h6493, _, _, _, _, _, _, _) => {
            h6493
        }
    };
    let a6495 = 1u64;
    if if if nat_eq(a6494, a6495) {
        let a6497 = match v0.clone() {
            Adt2::C0(_, _, _, _, h6496, _, _, _, _, _, _) => {
                h6496
            }
        };
        let a6498 = 1u64;
        nat_eq(a6497, a6498)
    } else {
        false
    } {
        let a6500 = match v0.clone() {
            Adt2::C0(_, _, _, _, _, h6499, _, _, _, _, _) => {
                h6499
            }
        };
        let a6501 = v1;
        nat_eq(a6500, a6501)
    } else {
        false
    } {
        let a6503 = match v0.clone() {
            Adt2::C0(_, _, h6502, _, _, _, _, _, _, _, _) => {
                h6502
            }
        };
        let a6504 = 1u64;
        if nat_eq(a6503, a6504) {
            Ok(true)
        } else {
            let a6509 = match v0.clone() {
                Adt2::C0(_, _, _, _, _, _, h6505, _, _, _, _) => {
                    h6505
                }
            };
            let a6510 = {
                let a6507 = match v0 {
                    Adt2::C0(_, h6506, _, _, _, _, _, _, _, _, _) => {
                        h6506
                    }
                };
                let a6508 = v1;
                nat_mul(a6507, a6508)?
            };
            Ok(nat_eq(a6509, a6510))
        }
    } else {
        Ok(false)
    }
}

pub fn f235(v0: Adt6, v1: u64, v2: u64) -> R<bool> {
    let v3: u64 = {
        let a6514 = {
            let a6511 = v2;
            let a6512 = 1u64;
            nat_sub(a6511, a6512)
        };
        let a6515 = match v0.clone() {
            Adt6::C0(_, h6513, _, _) => {
                h6513
            }
        };
        nat_mul(a6514, a6515)?
    };
    let a6517 = 0u64;
    let a6518 = match v0.clone() {
        Adt6::C0(h6516, _, _, _) => {
            h6516
        }
    };
    if if if if if if if if nat_lt(a6517, a6518) {
        let a6520 = match v0.clone() {
            Adt6::C0(h6519, _, _, _) => {
                h6519
            }
        };
        let a6521 = 2147483648u64;
        nat_lt(a6520, a6521)
    } else {
        false
    } {
        let a6523 = 0u64;
        let a6524 = match v0.clone() {
            Adt6::C0(_, h6522, _, _) => {
                h6522
            }
        };
        nat_lt(a6523, a6524)
    } else {
        false
    } {
        let a6526 = match v0.clone() {
            Adt6::C0(_, h6525, _, _) => {
                h6525
            }
        };
        let a6527 = 2147483648u64;
        nat_lt(a6526, a6527)
    } else {
        false
    } {
        let a6528 = v1;
        let a6529 = 2147483648u64;
        nat_lt(a6528, a6529)
    } else {
        false
    } {
        let a6530 = 0u64;
        let a6531 = v2;
        nat_lt(a6530, a6531)
    } else {
        false
    } {
        let a6532 = v3;
        let a6533 = 2147483648u64;
        nat_lt(a6532, a6533)
    } else {
        false
    } {
        let a6545 = {
            let a6540 = {
                let a6537 = v3;
                let a6538 = {
                    let a6535 = match v0.clone() {
                        Adt6::C0(h6534, _, _, _) => {
                            h6534
                        }
                    };
                    let a6536 = 1u64;
                    nat_sub(a6535, a6536)
                };
                nat_add(a6537, a6538)?
            };
            let a6541 = f155(match v0.clone() {
                Adt6::C0(_, _, h6539, _) => {
                    h6539
                }
            });
            nat_add(a6540, a6541)?
        };
        let a6546 = {
            let a6543 = 2147483647u64;
            let a6544 = f156(match v0.clone() {
                Adt6::C0(_, _, h6542, _) => {
                    h6542
                }
            });
            nat_add(a6543, a6544)?
        };
        nat_le(a6545, a6546)
    } else {
        false
    } {
        let a6548 = match v0 {
            Adt6::C0(_, _, h6547, _) => {
                h6547
            }
        };
        let a6549 = 2147483648u64;
        Ok(!nat_eq(a6548, a6549))
    } else {
        Ok(false)
    }
}

pub fn f236(v0: Adt2, v1: Adt2, v2: Adt2) -> R<Adt10> {
    if f285(match v0.clone() {
        Adt2::C0(h6550, _, _, _, _, _, _, _, _, _, _) => {
            h6550
        }
    }, match v0.clone() {
        Adt2::C0(_, h6551, _, _, _, _, _, _, _, _, _) => {
            h6551
        }
    }, match v0.clone() {
        Adt2::C0(_, _, h6552, _, _, _, _, _, _, _, _) => {
            h6552
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, h6553, _, _, _, _, _, _, _) => {
            h6553
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, h6554, _, _, _, _, _, _) => {
            h6554
        }
    }, match v1.clone() {
        Adt2::C0(h6555, _, _, _, _, _, _, _, _, _, _) => {
            h6555
        }
    }, match v1.clone() {
        Adt2::C0(_, h6556, _, _, _, _, _, _, _, _, _) => {
            h6556
        }
    }, match v1.clone() {
        Adt2::C0(_, _, h6557, _, _, _, _, _, _, _, _) => {
            h6557
        }
    }, match v1.clone() {
        Adt2::C0(_, _, _, h6558, _, _, _, _, _, _, _) => {
            h6558
        }
    }, match v1.clone() {
        Adt2::C0(_, _, _, _, h6559, _, _, _, _, _, _) => {
            h6559
        }
    }, match v2.clone() {
        Adt2::C0(h6560, _, _, _, _, _, _, _, _, _, _) => {
            h6560
        }
    }, match v2.clone() {
        Adt2::C0(_, h6561, _, _, _, _, _, _, _, _, _) => {
            h6561
        }
    }, match v2.clone() {
        Adt2::C0(_, _, h6562, _, _, _, _, _, _, _, _) => {
            h6562
        }
    }, match v2.clone() {
        Adt2::C0(_, _, _, h6563, _, _, _, _, _, _, _) => {
            h6563
        }
    }, match v2.clone() {
        Adt2::C0(_, _, _, _, h6564, _, _, _, _, _, _) => {
            h6564
        }
    }) {
        let m6566 = f230(match v0.clone() {
            Adt2::C0(h6565, _, _, _, _, _, _, _, _, _, _) => {
                h6565
            }
        });
        match m6566 {
            Adt18::C0 => {
                Ok(Adt10::C0)
            }
            Adt18::C1(v3) => {
                let a6572 = {
                    let a6569 = match v0.clone() {
                        Adt2::C0(_, h6567, _, _, _, _, _, _, _, _, _) => {
                            h6567
                        }
                    };
                    let a6570 = match v3.clone() {
                        Adt19::C0(_, h6568, _) => {
                            h6568
                        }
                    };
                    let a6571 = 0u64;
                    nat_rem(a6569, a6570, a6571)
                };
                let a6573 = 0u64;
                if if nat_eq(a6572, a6573) {
                    if f286(match v0.clone() {
                        Adt2::C0(_, h6574, _, _, _, _, _, _, _, _, _) => {
                            h6574
                        }
                    }, match v1.clone() {
                        Adt2::C0(_, h6575, _, _, _, _, _, _, _, _, _) => {
                            h6575
                        }
                    }, match v1.clone() {
                        Adt2::C0(_, _, h6576, _, _, _, _, _, _, _, _) => {
                            h6576
                        }
                    }, match v1.clone() {
                        Adt2::C0(_, _, _, h6577, _, _, _, _, _, _, _) => {
                            h6577
                        }
                    }) {
                        true
                    } else {
                        let a6579 = 4u64;
                        let a6580 = match v2.clone() {
                            Adt2::C0(_, _, _, _, _, h6578, _, _, _, _, _) => {
                                h6578
                            }
                        };
                        if if nat_le(a6579, a6580) {
                            let a6582 = match v3.clone() {
                                Adt19::C0(_, h6581, _) => {
                                    h6581
                                }
                            };
                            let a6583 = 1u64;
                            if nat_eq(a6582, a6583) {
                                true
                            } else {
                                let a6586 = match v3.clone() {
                                    Adt19::C0(_, _, h6584) => {
                                        h6584
                                    }
                                };
                                let a6587 = match v0.clone() {
                                    Adt2::C0(_, _, _, _, _, h6585, _, _, _, _, _) => {
                                        h6585
                                    }
                                };
                                nat_le(a6586, a6587)
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
                    let v4: Adt3 = f30(v0.clone(), match v3 {
                        Adt19::C0(_, h6588, _) => {
                            h6588
                        }
                    });
                    Ok(Adt10::C4(if f286(match v0.clone() {
                        Adt2::C0(_, h6589, _, _, _, _, _, _, _, _, _) => {
                            h6589
                        }
                    }, match v1.clone() {
                        Adt2::C0(_, h6590, _, _, _, _, _, _, _, _, _) => {
                            h6590
                        }
                    }, match v1.clone() {
                        Adt2::C0(_, _, h6591, _, _, _, _, _, _, _, _) => {
                            h6591
                        }
                    }, match v1.clone() {
                        Adt2::C0(_, _, _, h6592, _, _, _, _, _, _, _) => {
                            h6592
                        }
                    }) {
                        0u64
                    } else {
                        f195(v4.clone(), f30(v1.clone(), 1u64))?
                    }, 4u64, v4, f30(v2, 1u64), Adt4::C20(match v0 {
                        Adt2::C0(h6593, _, _, _, _, _, _, _, _, _, _) => {
                            h6593
                        }
                    }, f30(v1, 1u64))))
                } else {
                    Ok(Adt10::C1)
                }
            }
        }
    } else {
        Ok(Adt10::C1)
    }
}

pub fn f237(v0: Adt2, v1: Adt2, v2: Adt2) -> R<Adt10> {
    if if if if if if if if if if if if if f238(match v0.clone() {
        Adt2::C0(h6594, _, _, _, _, _, _, _, _, _, _) => {
            h6594
        }
    }) {
        let a6600 = {
            let a6597 = match v0.clone() {
                Adt2::C0(_, h6595, _, _, _, _, _, _, _, _, _) => {
                    h6595
                }
            };
            let a6598 = f268(match v0.clone() {
                Adt2::C0(h6596, _, _, _, _, _, _, _, _, _, _) => {
                    h6596
                }
            });
            let a6599 = 1u64;
            nat_rem(a6597, a6598, a6599)
        };
        let a6601 = 0u64;
        nat_eq(a6600, a6601)
    } else {
        false
    } {
        let a6603 = match v1.clone() {
            Adt2::C0(h6602, _, _, _, _, _, _, _, _, _, _) => {
                h6602
            }
        };
        let a6604 = 26u64;
        nat_eq(a6603, a6604)
    } else {
        false
    } {
        let a6606 = match v2.clone() {
            Adt2::C0(h6605, _, _, _, _, _, _, _, _, _, _) => {
                h6605
            }
        };
        let a6607 = 0u64;
        nat_eq(a6606, a6607)
    } else {
        false
    } {
        let a6610 = match v0.clone() {
            Adt2::C0(_, h6608, _, _, _, _, _, _, _, _, _) => {
                h6608
            }
        };
        let a6611 = match v2.clone() {
            Adt2::C0(_, h6609, _, _, _, _, _, _, _, _, _) => {
                h6609
            }
        };
        nat_eq(a6610, a6611)
    } else {
        false
    } {
        let a6614 = match v1.clone() {
            Adt2::C0(_, h6612, _, _, _, _, _, _, _, _, _) => {
                h6612
            }
        };
        let a6615 = match v0.clone() {
            Adt2::C0(_, _, h6613, _, _, _, _, _, _, _, _) => {
                h6613
            }
        };
        nat_le(a6614, a6615)
    } else {
        false
    } {
        let a6617 = match v0.clone() {
            Adt2::C0(_, _, _, h6616, _, _, _, _, _, _, _) => {
                h6616
            }
        };
        let a6618 = 1u64;
        nat_eq(a6617, a6618)
    } else {
        false
    } {
        let a6620 = match v0.clone() {
            Adt2::C0(_, _, _, _, h6619, _, _, _, _, _, _) => {
                h6619
            }
        };
        let a6621 = 1u64;
        nat_eq(a6620, a6621)
    } else {
        false
    } {
        let a6623 = match v2.clone() {
            Adt2::C0(_, _, _, h6622, _, _, _, _, _, _, _) => {
                h6622
            }
        };
        let a6624 = 1u64;
        nat_eq(a6623, a6624)
    } else {
        false
    } {
        let a6626 = match v2.clone() {
            Adt2::C0(_, _, _, _, h6625, _, _, _, _, _, _) => {
                h6625
            }
        };
        let a6627 = 1u64;
        nat_eq(a6626, a6627)
    } else {
        false
    } {
        let a6629 = match v1.clone() {
            Adt2::C0(_, _, h6628, _, _, _, _, _, _, _, _) => {
                h6628
            }
        };
        let a6630 = 1u64;
        nat_eq(a6629, a6630)
    } else {
        false
    } {
        let a6632 = match v1.clone() {
            Adt2::C0(_, _, _, h6631, _, _, _, _, _, _, _) => {
                h6631
            }
        };
        let a6633 = 1u64;
        nat_eq(a6632, a6633)
    } else {
        false
    } {
        let a6635 = match v1.clone() {
            Adt2::C0(_, _, _, _, h6634, _, _, _, _, _, _) => {
                h6634
            }
        };
        let a6636 = 1u64;
        nat_eq(a6635, a6636)
    } else {
        false
    } {
        if f53(match v2.clone() {
            Adt2::C0(_, h6637, _, _, _, _, _, _, _, _, _) => {
                h6637
            }
        }, match v2.clone() {
            Adt2::C0(_, _, h6638, _, _, _, _, _, _, _, _) => {
                h6638
            }
        }, match v2.clone() {
            Adt2::C0(_, _, _, h6639, _, _, _, _, _, _, _) => {
                h6639
            }
        }, match v2.clone() {
            Adt2::C0(_, _, _, _, h6640, _, _, _, _, _, _) => {
                h6640
            }
        }) {
            Ok(Adt10::C4(0u64, 4u64, f30(v0, 1u64), f30(v2, 1u64), Adt4::C9(0u64, 0u64)))
        } else {
            let a6642 = match v1.clone() {
                Adt2::C0(_, h6641, _, _, _, _, _, _, _, _, _) => {
                    h6641
                }
            };
            let a6643 = 0u64;
            if nat_eq(a6642, a6643) {
                Ok(Adt10::C4(f28(v2.clone(), 1u64)?, 4u64, f30(v0, 1u64), f30(v2, 1u64), Adt4::C9(0u64, 0u64)))
            } else {
                let a6645 = match v0.clone() {
                    Adt2::C0(_, h6644, _, _, _, _, _, _, _, _, _) => {
                        h6644
                    }
                };
                let a6646 = 2147483647u64;
                if if if if if if nat_le(a6645, a6646) {
                    let a6648 = match v1.clone() {
                        Adt2::C0(_, h6647, _, _, _, _, _, _, _, _, _) => {
                            h6647
                        }
                    };
                    let a6649 = 2147483647u64;
                    nat_le(a6648, a6649)
                } else {
                    false
                } {
                    let a6651 = 4u64;
                    let a6652 = match v2.clone() {
                        Adt2::C0(_, _, _, _, _, h6650, _, _, _, _, _) => {
                            h6650
                        }
                    };
                    nat_le(a6651, a6652)
                } else {
                    false
                } {
                    let a6658 = {
                        let a6655 = match v2.clone() {
                            Adt2::C0(_, h6653, _, _, _, _, _, _, _, _, _) => {
                                h6653
                            }
                        };
                        let a6656 = match v2.clone() {
                            Adt2::C0(_, _, _, _, _, h6654, _, _, _, _, _) => {
                                h6654
                            }
                        };
                        nat_mul(a6655, a6656)?
                    };
                    let a6659 = match v2.clone() {
                        Adt2::C0(_, _, _, _, _, _, h6657, _, _, _, _) => {
                            h6657
                        }
                    };
                    nat_le(a6658, a6659)
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
                        let a6672 = {
                            let a6663 = {
                                let a6661 = 2u64;
                                let a6662 = match v1.clone() {
                                    Adt2::C0(_, h6660, _, _, _, _, _, _, _, _, _) => {
                                        h6660
                                    }
                                };
                                nat_mul(a6661, a6662)?
                            };
                            let a6664 = f28(v2.clone(), 1u64)?;
                            nat_add(a6663, a6664)?
                        };
                        let a6673 = {
                            let a6670 = match v1.clone() {
                                Adt2::C0(_, h6665, _, _, _, _, _, _, _, _, _) => {
                                    h6665
                                }
                            };
                            let a6671 = {
                                let a6668 = 1u64;
                                let a6669 = f192(match v0.clone() {
                                    Adt2::C0(h6666, _, _, _, _, _, _, _, _, _, _) => {
                                        h6666
                                    }
                                }, match v0.clone() {
                                    Adt2::C0(_, h6667, _, _, _, _, _, _, _, _, _) => {
                                        h6667
                                    }
                                })?;
                                nat_add(a6668, a6669)?
                            };
                            nat_mul(a6670, a6671)?
                        };
                        nat_add(a6672, a6673)?
                    }, 4u64, Adt3::C0(match v0.clone() {
                        Adt2::C0(_, h6674, _, _, _, _, _, _, _, _, _) => {
                            h6674
                        }
                    }, match v1.clone() {
                        Adt2::C0(_, h6675, _, _, _, _, _, _, _, _, _) => {
                            h6675
                        }
                    }, 1u64, 1u64, match v0.clone() {
                        Adt2::C0(_, _, _, _, _, h6676, _, _, _, _, _) => {
                            h6676
                        }
                    }, match v0.clone() {
                        Adt2::C0(_, _, _, _, _, _, h6677, _, _, _, _) => {
                            h6677
                        }
                    }, match v0.clone() {
                        Adt2::C0(_, _, _, _, _, _, _, h6678, _, _, _) => {
                            h6678
                        }
                    }, match v0.clone() {
                        Adt2::C0(_, _, _, _, _, _, _, _, h6679, _, _) => {
                            h6679
                        }
                    }), f30(v2, 1u64), Adt4::C19(match v0 {
                        Adt2::C0(h6680, _, _, _, _, _, _, _, _, _, _) => {
                            h6680
                        }
                    }, f30(v1, 1u64))))
                } else {
                    Ok(Adt10::C1)
                }
            }
        }
    } else {
        Ok(Adt10::C1)
    }
}

pub fn f238(v0: u64) -> bool {
    let a6681 = v0;
    let a6682 = 0u64;
    if if if if if if if if if if if if if if if if if if if if if if if if if if if if if nat_eq(a6681, a6682) {
        true
    } else {
        let a6683 = v0;
        let a6684 = 1u64;
        nat_eq(a6683, a6684)
    } {
        true
    } else {
        let a6685 = v0;
        let a6686 = 2u64;
        nat_eq(a6685, a6686)
    } {
        true
    } else {
        let a6687 = v0;
        let a6688 = 3u64;
        nat_eq(a6687, a6688)
    } {
        true
    } else {
        let a6689 = v0;
        let a6690 = 6u64;
        nat_eq(a6689, a6690)
    } {
        true
    } else {
        let a6691 = v0;
        let a6692 = 7u64;
        nat_eq(a6691, a6692)
    } {
        true
    } else {
        let a6693 = v0;
        let a6694 = 8u64;
        nat_eq(a6693, a6694)
    } {
        true
    } else {
        let a6695 = v0;
        let a6696 = 10u64;
        nat_eq(a6695, a6696)
    } {
        true
    } else {
        let a6697 = v0;
        let a6698 = 11u64;
        nat_eq(a6697, a6698)
    } {
        true
    } else {
        let a6699 = v0;
        let a6700 = 12u64;
        nat_eq(a6699, a6700)
    } {
        true
    } else {
        let a6701 = v0;
        let a6702 = 13u64;
        nat_eq(a6701, a6702)
    } {
        true
    } else {
        let a6703 = v0;
        let a6704 = 14u64;
        nat_eq(a6703, a6704)
    } {
        true
    } else {
        let a6705 = v0;
        let a6706 = 16u64;
        nat_eq(a6705, a6706)
    } {
        true
    } else {
        let a6707 = v0;
        let a6708 = 17u64;
        nat_eq(a6707, a6708)
    } {
        true
    } else {
        let a6709 = v0;
        let a6710 = 18u64;
        nat_eq(a6709, a6710)
    } {
        true
    } else {
        let a6711 = v0;
        let a6712 = 19u64;
        nat_eq(a6711, a6712)
    } {
        true
    } else {
        let a6713 = v0;
        let a6714 = 20u64;
        nat_eq(a6713, a6714)
    } {
        true
    } else {
        let a6715 = v0;
        let a6716 = 21u64;
        nat_eq(a6715, a6716)
    } {
        true
    } else {
        let a6717 = v0;
        let a6718 = 22u64;
        nat_eq(a6717, a6718)
    } {
        true
    } else {
        let a6719 = v0;
        let a6720 = 23u64;
        nat_eq(a6719, a6720)
    } {
        true
    } else {
        let a6721 = v0;
        let a6722 = 26u64;
        nat_eq(a6721, a6722)
    } {
        true
    } else {
        let a6723 = v0;
        let a6724 = 29u64;
        nat_eq(a6723, a6724)
    } {
        true
    } else {
        let a6725 = v0;
        let a6726 = 30u64;
        nat_eq(a6725, a6726)
    } {
        true
    } else {
        let a6727 = v0;
        let a6728 = 34u64;
        nat_eq(a6727, a6728)
    } {
        true
    } else {
        let a6729 = v0;
        let a6730 = 35u64;
        nat_eq(a6729, a6730)
    } {
        true
    } else {
        let a6731 = v0;
        let a6732 = 39u64;
        nat_eq(a6731, a6732)
    } {
        true
    } else {
        let a6733 = v0;
        let a6734 = 40u64;
        nat_eq(a6733, a6734)
    } {
        true
    } else {
        let a6735 = v0;
        let a6736 = 41u64;
        nat_eq(a6735, a6736)
    } {
        true
    } else {
        let a6737 = v0;
        let a6738 = 42u64;
        nat_eq(a6737, a6738)
    } {
        true
    } else {
        let a6739 = v0;
        let a6740 = 43u64;
        nat_eq(a6739, a6740)
    }
}

pub fn f239(v0: u64) -> bool {
    if f193(v0) {
        true
    } else {
        let a6741 = v0;
        let a6742 = 43u64;
        nat_eq(a6741, a6742)
    }
}

pub fn f240(v0: u64, v1: Adt2, v2: bool, v3: Adt2) -> R<Adt10> {
    let a6745 = match v1.clone() {
        Adt2::C0(h6743, _, _, _, _, _, _, _, _, _, _) => {
            h6743
        }
    };
    let a6746 = match v3.clone() {
        Adt2::C0(h6744, _, _, _, _, _, _, _, _, _, _) => {
            h6744
        }
    };
    if if nat_eq(a6745, a6746) {
        let a6747 = v0;
        let a6748 = 33u64;
        if nat_eq(a6747, a6748) {
            v2
        } else {
            true
        }
    } else {
        false
    } {
        let m6750 = f230(match v1.clone() {
            Adt2::C0(h6749, _, _, _, _, _, _, _, _, _, _) => {
                h6749
            }
        });
        match m6750 {
            Adt18::C0 => {
                Ok(Adt10::C0)
            }
            Adt18::C1(v4) => {
                let v5: u64 = match v4.clone() {
                    Adt19::C0(_, h6751, _) => {
                        h6751
                    }
                };
                let v6: u64 = f28(v1.clone(), v5)?;
                let a6756 = {
                    let a6753 = match v1.clone() {
                        Adt2::C0(_, h6752, _, _, _, _, _, _, _, _, _) => {
                            h6752
                        }
                    };
                    let a6754 = v5;
                    let a6755 = 1u64;
                    nat_rem(a6753, a6754, a6755)
                };
                let a6757 = 0u64;
                if if if nat_eq(a6756, a6757) {
                    let a6762 = {
                        let a6759 = match v3.clone() {
                            Adt2::C0(_, h6758, _, _, _, _, _, _, _, _, _) => {
                                h6758
                            }
                        };
                        let a6760 = v5;
                        let a6761 = 1u64;
                        nat_rem(a6759, a6760, a6761)
                    };
                    let a6763 = 0u64;
                    nat_eq(a6762, a6763)
                } else {
                    false
                } {
                    let a6764 = v6;
                    let a6765 = f28(v3.clone(), v5)?;
                    nat_eq(a6764, a6765)
                } else {
                    false
                } {
                    Ok(Adt10::C4(v6, match v4 {
                        Adt19::C0(_, _, h6766) => {
                            h6766
                        }
                    }, f30(v1, v5), f30(v3, v5), Adt4::C0))
                } else {
                    Ok(Adt10::C1)
                }
            }
        }
    } else {
        Ok(Adt10::C1)
    }
}

pub fn f241(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64, v7: u64) -> bool {
    if f53(v4, v5, v6, v7) {
        f53(v0, v1, v2, v3)
    } else {
        let a6770 = {
            let a6767 = v0;
            let a6768 = v4;
            let a6769 = 1u64;
            nat_rem(a6767, a6768, a6769)
        };
        let a6771 = 0u64;
        if if if nat_eq(a6770, a6771) {
            let a6775 = {
                let a6772 = v1;
                let a6773 = v5;
                let a6774 = 1u64;
                nat_rem(a6772, a6773, a6774)
            };
            let a6776 = 0u64;
            nat_eq(a6775, a6776)
        } else {
            false
        } {
            let a6780 = {
                let a6777 = v2;
                let a6778 = v6;
                let a6779 = 1u64;
                nat_rem(a6777, a6778, a6779)
            };
            let a6781 = 0u64;
            nat_eq(a6780, a6781)
        } else {
            false
        } {
            let a6785 = {
                let a6782 = v3;
                let a6783 = v7;
                let a6784 = 1u64;
                nat_rem(a6782, a6783, a6784)
            };
            let a6786 = 0u64;
            nat_eq(a6785, a6786)
        } else {
            false
        }
    }
}

pub fn f242(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64) -> R<bool> {
    let a6787 = v0;
    let a6788 = v1;
    if nat_eq(a6787, a6788) {
        let a6791 = v4;
        let a6792 = {
            let a6789 = v2;
            let a6790 = v3;
            nat_add(a6789, a6790)?
        };
        Ok(nat_eq(a6791, a6792))
    } else {
        let a6793 = v2;
        let a6794 = v3;
        if nat_eq(a6793, a6794) {
            let a6795 = v4;
            let a6796 = v2;
            Ok(nat_eq(a6795, a6796))
        } else {
            Ok(false)
        }
    }
}

pub fn f243(v0: u64, v1: u64, v2: u64, v3: u64) -> R<bool> {
    let v4: u64 = {
        let a6799 = {
            let a6797 = v0;
            let a6798 = f156(v1);
            nat_add(a6797, a6798)?
        };
        let a6800 = f156(v2);
        nat_add(a6799, a6800)?
    };
    let v5: u64 = {
        let a6801 = f155(v1);
        let a6802 = f155(v2);
        nat_add(a6801, a6802)?
    };
    let a6803 = v1;
    let a6804 = 4294967295u64;
    if if nat_le(a6803, a6804) {
        let a6805 = v2;
        let a6806 = 4294967295u64;
        nat_le(a6805, a6806)
    } else {
        false
    } {
        let a6807 = v5;
        let a6808 = v4;
        if nat_le(a6807, a6808) {
            let a6811 = v3;
            let a6812 = {
                let a6809 = v4;
                let a6810 = v5;
                nat_sub(a6809, a6810)
            };
            Ok(nat_eq(a6811, a6812))
        } else {
            Ok(false)
        }
    } else {
        Ok(false)
    }
}

pub fn f244(v0: u64, v1: u64) -> R<bool> {
    let a6813 = v1;
    let a6814 = 4294967295u64;
    if nat_le(a6813, a6814) {
        let a6817 = {
            let a6815 = f156(v1);
            let a6816 = f155(v1);
            nat_add(a6815, a6816)?
        };
        let a6818 = v0;
        Ok(nat_lt(a6817, a6818))
    } else {
        Ok(false)
    }
}

pub fn f245(v0: Adt3, v1: u64, v2: u64, v3: u64, v4: bool) -> R<bool> {
    let v5: u64 = f287(v0.clone(), false)?;
    let v6: u64 = f287(v0, true)?;
    let a6819 = v6;
    let a6820 = v1;
    if if nat_le(a6819, a6820) {
        let a6827 = {
            let a6825 = {
                let a6821 = v1;
                let a6822 = v5;
                nat_add(a6821, a6822)?
            };
            let a6826 = {
                let a6823 = v3;
                let a6824 = 0u64;
                if nat_eq(a6823, a6824) {
                    0u64
                } else {
                    4u64
                }
            };
            nat_add(a6825, a6826)?
        };
        let a6828 = v2;
        nat_le(a6827, a6828)
    } else {
        false
    } {
        if v4 {
            let a6833 = {
                let a6831 = {
                    let a6829 = v1;
                    let a6830 = v5;
                    nat_add(a6829, a6830)?
                };
                let a6832 = v6;
                nat_sub(a6831, a6832)
            };
            let a6834 = v2;
            Ok(nat_lt(a6833, a6834))
        } else {
            let a6839 = {
                let a6837 = {
                    let a6835 = v1;
                    let a6836 = v5;
                    nat_add(a6835, a6836)?
                };
                let a6838 = v6;
                nat_sub(a6837, a6838)
            };
            let a6840 = v2;
            Ok(nat_le(a6839, a6840))
        }
    } else {
        Ok(false)
    }
}

pub fn f246(v0: u64, v1: u64) -> bool {
    let a6841 = v0;
    let a6842 = 0u64;
    if if nat_eq(a6841, a6842) {
        true
    } else {
        let a6843 = v0;
        let a6844 = 1u64;
        nat_eq(a6843, a6844)
    } {
        let a6845 = v1;
        let a6846 = 4294967296u64;
        nat_lt(a6845, a6846)
    } else {
        false
    }
}

pub fn f247(v0: u64, v1: u64) -> R<u64> {
    let a6851 = {
        let a6849 = v0;
        let a6850 = {
            let a6847 = v1;
            let a6848 = 1u64;
            nat_sub(a6847, a6848)
        };
        nat_add(a6849, a6850)?
    };
    let a6852 = v1;
    let a6853 = 0u64;
    Ok(nat_quot(a6851, a6852, a6853))
}

pub fn f248(v0: Adt2, v1: Adt2, v2: u64, v3: u64) -> R<bool> {
    if f53(match v0.clone() {
        Adt2::C0(_, h6854, _, _, _, _, _, _, _, _, _) => {
            h6854
        }
    }, match v0.clone() {
        Adt2::C0(_, _, h6855, _, _, _, _, _, _, _, _) => {
            h6855
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, h6856, _, _, _, _, _, _, _) => {
            h6856
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, h6857, _, _, _, _, _, _) => {
            h6857
        }
    }) {
        Ok(true)
    } else {
        let v4: u64 = {
            let a6860 = match v1.clone() {
                Adt2::C0(_, h6858, _, _, _, _, _, _, _, _, _) => {
                    h6858
                }
            };
            let a6861 = match v1.clone() {
                Adt2::C0(_, _, h6859, _, _, _, _, _, _, _, _) => {
                    h6859
                }
            };
            nat_mul(a6860, a6861)?
        };
        let v5: u64 = {
            let a6863 = v4;
            let a6864 = match v1.clone() {
                Adt2::C0(_, _, _, h6862, _, _, _, _, _, _, _) => {
                    h6862
                }
            };
            nat_mul(a6863, a6864)?
        };
        let a6877 = {
            let a6875 = {
                let a6866 = match v0.clone() {
                    Adt2::C0(_, h6865, _, _, _, _, _, _, _, _, _) => {
                        h6865
                    }
                };
                let a6867 = 1u64;
                nat_sub(a6866, a6867)
            };
            let a6876 = {
                let a6873 = f288(match v0.clone() {
                    Adt2::C0(_, _, h6868, _, _, _, _, _, _, _, _) => {
                        h6868
                    }
                }, v2, v5, match v1.clone() {
                    Adt2::C0(_, h6869, _, _, _, _, _, _, _, _, _) => {
                        h6869
                    }
                })?;
                let a6874 = f288(match v0 {
                    Adt2::C0(_, _, _, h6870, _, _, _, _, _, _, _) => {
                        h6870
                    }
                }, v2, {
                    let a6871 = v3;
                    let a6872 = v5;
                    nat_mul(a6871, a6872)?
                }, v4)?;
                nat_add(a6873, a6874)?
            };
            nat_add(a6875, a6876)?
        };
        let a6878 = f28(v1, 1u64)?;
        Ok(nat_lt(a6877, a6878))
    }
}

pub fn f249(v0: u64, v1: u64, v2: u64) -> R<bool> {
    let a6879 = v0;
    let a6880 = 2147483648u64;
    if if if if nat_lt(a6879, a6880) {
        let a6881 = v1;
        let a6882 = 2147483648u64;
        nat_lt(a6881, a6882)
    } else {
        false
    } {
        let a6885 = {
            let a6883 = v0;
            let a6884 = v1;
            nat_mul(a6883, a6884)?
        };
        let a6886 = 2147483648u64;
        nat_lt(a6885, a6886)
    } else {
        false
    } {
        let a6891 = {
            let a6889 = {
                let a6887 = v0;
                let a6888 = 1u64;
                nat_sub(a6887, a6888)
            };
            let a6890 = v2;
            nat_mul(a6889, a6890)?
        };
        let a6892 = 2147483648u64;
        nat_lt(a6891, a6892)
    } else {
        false
    } {
        let a6897 = {
            let a6895 = {
                let a6893 = v1;
                let a6894 = 1u64;
                nat_sub(a6893, a6894)
            };
            let a6896 = v2;
            nat_mul(a6895, a6896)?
        };
        let a6898 = 2147483648u64;
        Ok(nat_lt(a6897, a6898))
    } else {
        Ok(false)
    }
}

pub fn f250(v0: Adt3, v1: u64, v2: u64) -> R<u64> {
    let a6899 = v1;
    let a6900 = 13u64;
    if if nat_eq(a6899, a6900) {
        f53(match v0.clone() {
            Adt3::C0(h6901, _, _, _, _, _, _, _) => {
                h6901
            }
        }, match v0.clone() {
            Adt3::C0(_, h6902, _, _, _, _, _, _) => {
                h6902
            }
        }, match v0.clone() {
            Adt3::C0(_, _, h6903, _, _, _, _, _) => {
                h6903
            }
        }, match v0.clone() {
            Adt3::C0(_, _, _, h6904, _, _, _, _) => {
                h6904
            }
        })
    } else {
        false
    } {
        Ok(2u64)
    } else {
        let v3: u64 = {
            let a6910 = match v0.clone() {
                Adt3::C0(_, h6905, _, _, _, _, _, _) => {
                    h6905
                }
            };
            let a6911 = {
                let a6908 = match v0.clone() {
                    Adt3::C0(_, _, h6906, _, _, _, _, _) => {
                        h6906
                    }
                };
                let a6909 = match v0.clone() {
                    Adt3::C0(_, _, _, h6907, _, _, _, _) => {
                        h6907
                    }
                };
                nat_mul(a6908, a6909)?
            };
            nat_mul(a6910, a6911)?
        };
        let a6912 = v1;
        let a6913 = 13u64;
        if nat_eq(a6912, a6913) {
            let a6922 = 2u64;
            let a6923 = if f289(v0.clone(), v2) {
                let a6915 = v3;
                let a6916 = f262(match v0 {
                    Adt3::C0(h6914, _, _, _, _, _, _, _) => {
                        h6914
                    }
                }, false)?;
                nat_mul(a6915, a6916)?
            } else {
                let a6920 = v3;
                let a6921 = {
                    let a6918 = match v0 {
                        Adt3::C0(h6917, _, _, _, _, _, _, _) => {
                            h6917
                        }
                    };
                    let a6919 = 2u64;
                    nat_mul(a6918, a6919)?
                };
                nat_mul(a6920, a6921)?
            };
            nat_add(a6922, a6923)
        } else {
            let a6927 = v3;
            let a6928 = f262(match v0 {
                Adt3::C0(h6924, _, _, _, _, _, _, _) => {
                    h6924
                }
            }, {
                let a6925 = v1;
                let a6926 = 16u64;
                nat_eq(a6925, a6926)
            })?;
            nat_mul(a6927, a6928)
        }
    }
}

pub fn f251(v0: u64, v1: u64, v2: Adt2, v3: Option<Adt2>, v4: Adt2) -> R<Adt10> {
    let a6929 = v0;
    let a6930 = 30u64;
    if if nat_eq(a6929, a6930) {
        let a6933 = match v2.clone() {
            Adt2::C0(_, _, _, _, _, _, h6931, _, _, _, _) => {
                h6931
            }
        };
        let a6934 = match v2.clone() {
            Adt2::C0(_, _, _, _, _, h6932, _, _, _, _, _) => {
                h6932
            }
        };
        nat_lt(a6933, a6934)
    } else {
        false
    } {
        Ok(Adt10::C1)
    } else {
        let m6935 = v3;
        match m6935 {
            None => {
                f290(v0, v1, match v2.clone() {
                    Adt2::C0(h6936, _, _, _, _, _, _, _, _, _, _) => {
                        h6936
                    }
                }, 0u64, match v4.clone() {
                    Adt2::C0(h6937, _, _, _, _, _, _, _, _, _, _) => {
                        h6937
                    }
                }, false, match v2.clone() {
                    Adt2::C0(_, h6938, _, _, _, _, _, _, _, _, _) => {
                        h6938
                    }
                }, match v2.clone() {
                    Adt2::C0(_, _, h6939, _, _, _, _, _, _, _, _) => {
                        h6939
                    }
                }, match v2.clone() {
                    Adt2::C0(_, _, _, h6940, _, _, _, _, _, _, _) => {
                        h6940
                    }
                }, match v2 {
                    Adt2::C0(_, _, _, _, h6941, _, _, _, _, _, _) => {
                        h6941
                    }
                }, 1u64, 1u64, 1u64, 1u64, match v4.clone() {
                    Adt2::C0(_, h6942, _, _, _, _, _, _, _, _, _) => {
                        h6942
                    }
                }, match v4.clone() {
                    Adt2::C0(_, _, h6943, _, _, _, _, _, _, _, _) => {
                        h6943
                    }
                }, match v4.clone() {
                    Adt2::C0(_, _, _, h6944, _, _, _, _, _, _, _) => {
                        h6944
                    }
                }, match v4 {
                    Adt2::C0(_, _, _, _, h6945, _, _, _, _, _, _) => {
                        h6945
                    }
                })
            }
            Some(v5) => {
                f290(v0, v1, match v2.clone() {
                    Adt2::C0(h6946, _, _, _, _, _, _, _, _, _, _) => {
                        h6946
                    }
                }, match v5.clone() {
                    Adt2::C0(h6947, _, _, _, _, _, _, _, _, _, _) => {
                        h6947
                    }
                }, match v4.clone() {
                    Adt2::C0(h6948, _, _, _, _, _, _, _, _, _, _) => {
                        h6948
                    }
                }, true, match v2.clone() {
                    Adt2::C0(_, h6949, _, _, _, _, _, _, _, _, _) => {
                        h6949
                    }
                }, match v2.clone() {
                    Adt2::C0(_, _, h6950, _, _, _, _, _, _, _, _) => {
                        h6950
                    }
                }, match v2.clone() {
                    Adt2::C0(_, _, _, h6951, _, _, _, _, _, _, _) => {
                        h6951
                    }
                }, match v2 {
                    Adt2::C0(_, _, _, _, h6952, _, _, _, _, _, _) => {
                        h6952
                    }
                }, match v5.clone() {
                    Adt2::C0(_, h6953, _, _, _, _, _, _, _, _, _) => {
                        h6953
                    }
                }, match v5.clone() {
                    Adt2::C0(_, _, h6954, _, _, _, _, _, _, _, _) => {
                        h6954
                    }
                }, match v5.clone() {
                    Adt2::C0(_, _, _, h6955, _, _, _, _, _, _, _) => {
                        h6955
                    }
                }, match v5 {
                    Adt2::C0(_, _, _, _, h6956, _, _, _, _, _, _) => {
                        h6956
                    }
                }, match v4.clone() {
                    Adt2::C0(_, h6957, _, _, _, _, _, _, _, _, _) => {
                        h6957
                    }
                }, match v4.clone() {
                    Adt2::C0(_, _, h6958, _, _, _, _, _, _, _, _) => {
                        h6958
                    }
                }, match v4.clone() {
                    Adt2::C0(_, _, _, h6959, _, _, _, _, _, _, _) => {
                        h6959
                    }
                }, match v4 {
                    Adt2::C0(_, _, _, _, h6960, _, _, _, _, _, _) => {
                        h6960
                    }
                })
            }
        }
    }
}

pub fn f252(v0: u64, v1: u64) -> u64 {
    let a6961 = v0;
    let a6962 = 23u64;
    if nat_eq(a6961, a6962) {
        let a6963 = v1;
        let a6964 = 0u64;
        if nat_eq(a6963, a6964) {
            2u64
        } else {
            3u64
        }
    } else {
        1u64
    }
}

pub fn f253(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let a6969 = {
        let a6967 = {
            let a6965 = v0;
            let a6966 = v1;
            nat_sub(a6965, a6966)
        };
        let a6968 = 1u64;
        nat_sub(a6967, a6968)
    };
    let a6970 = v2;
    nat_add(a6969, a6970)
}

pub fn f254(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64, v7: u64, v8: u64, v9: u64, v10: u64, v11: u64) -> R<u64> {
    f154({
        let a6971 = v0;
        let a6972 = v4;
        let a6973 = 0u64;
        nat_rem(a6971, a6972, a6973)
    }, {
        let a6974 = v1;
        let a6975 = v5;
        let a6976 = 0u64;
        nat_rem(a6974, a6975, a6976)
    }, {
        let a6977 = v2;
        let a6978 = v6;
        let a6979 = 0u64;
        nat_rem(a6977, a6978, a6979)
    }, {
        let a6980 = v3;
        let a6981 = v7;
        let a6982 = 0u64;
        nat_rem(a6980, a6981, a6982)
    }, v8, v9, v10, v11)
}

pub fn f255(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let a6983 = v1;
    let a6984 = 2147483648u64;
    if nat_lt(a6983, a6984) {
        let a6995 = {
            let a6993 = {
                let a6988 = {
                    let a6985 = v0;
                    let a6986 = v2;
                    let a6987 = 0u64;
                    nat_rem(a6985, a6986, a6987)
                };
                let a6989 = v2;
                nat_add(a6988, a6989)?
            };
            let a6994 = {
                let a6990 = v1;
                let a6991 = v2;
                let a6992 = 0u64;
                nat_rem(a6990, a6991, a6992)
            };
            nat_sub(a6993, a6994)
        };
        let a6996 = v2;
        let a6997 = 0u64;
        Ok(nat_rem(a6995, a6996, a6997))
    } else {
        let a7000 = {
            let a6998 = v0;
            let a6999 = f155(v1);
            nat_add(a6998, a6999)?
        };
        let a7001 = v2;
        let a7002 = 0u64;
        Ok(nat_rem(a7000, a7001, a7002))
    }
}

pub fn f256(v0: u64, v1: u64, v2: u64) -> R<bool> {
    let a7005 = f156(v1);
    let a7006 = {
        let a7003 = v0;
        let a7004 = f155(v1);
        nat_add(a7003, a7004)?
    };
    if nat_le(a7005, a7006) {
        let a7007 = f257(v0, v1)?;
        let a7008 = v2;
        Ok(nat_lt(a7007, a7008))
    } else {
        Ok(false)
    }
}

pub fn f257(v0: u64, v1: u64) -> R<u64> {
    let a7011 = {
        let a7009 = v0;
        let a7010 = f155(v1);
        nat_add(a7009, a7010)?
    };
    let a7012 = f156(v1);
    Ok(nat_sub(a7011, a7012))
}

pub fn f258(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let a7013 = v0;
    let a7014 = v1;
    if nat_lt(a7013, a7014) {
        let a7015 = v1;
        let a7016 = v0;
        Ok(nat_sub(a7015, a7016))
    } else {
        let a7019 = v0;
        let a7020 = {
            let a7017 = v1;
            let a7018 = v2;
            nat_add(a7017, a7018)?
        };
        if nat_lt(a7019, a7020) {
            let a7021 = v0;
            let a7022 = v1;
            Ok(nat_sub(a7021, a7022))
        } else {
            let a7029 = {
                let a7025 = 2u64;
                let a7026 = {
                    let a7023 = v2;
                    let a7024 = 1u64;
                    nat_sub(a7023, a7024)
                };
                nat_mul(a7025, a7026)?
            };
            let a7030 = {
                let a7027 = v0;
                let a7028 = v1;
                nat_sub(a7027, a7028)
            };
            Ok(nat_sub(a7029, a7030))
        }
    }
}

pub fn f259(v0: u64, v1: Adt3, v2: u64) -> R<u64> {
    let a7033 = {
        let a7031 = v2;
        let a7032 = f291(v1.clone(), f63(v0, v1.clone(), 0u64), f63(v0, v1.clone(), 1u64), f63(v0, v1.clone(), 2u64), f63(v0, v1.clone(), 3u64), false)?;
        nat_add(a7031, a7032)?
    };
    let a7034 = f291(v1.clone(), f63(v0, v1.clone(), 0u64), f63(v0, v1.clone(), 1u64), f63(v0, v1.clone(), 2u64), f63(v0, v1, 3u64), true)?;
    Ok(nat_sub(a7033, a7034))
}

pub fn f260(v0: u64, v1: u64, v2: u64) -> bool {
    let a7035 = v0;
    let a7036 = 0u64;
    if nat_eq(a7035, a7036) {
        let a7037 = v2;
        let a7038 = v1;
        nat_le(a7037, a7038)
    } else {
        let a7039 = v0;
        let a7040 = 1u64;
        if nat_eq(a7039, a7040) {
            let a7041 = v2;
            let a7042 = v1;
            nat_lt(a7041, a7042)
        } else {
            let a7043 = v0;
            let a7044 = 2u64;
            if nat_eq(a7043, a7044) {
                let a7045 = v1;
                let a7046 = v2;
                nat_le(a7045, a7046)
            } else {
                let a7047 = v0;
                let a7048 = 3u64;
                if nat_eq(a7047, a7048) {
                    let a7049 = v1;
                    let a7050 = v2;
                    nat_lt(a7049, a7050)
                } else {
                    let a7051 = v1;
                    let a7052 = v2;
                    nat_eq(a7051, a7052)
                }
            }
        }
    }
}

pub fn f261(v0: u64, v1: Adt3, v2: u64) -> R<Adt9> {
    let a7053 = v0;
    let a7054 = 0u64;
    if nat_eq(a7053, a7054) {
        Ok(f274(0u64))
    } else {
        let a7057 = {
            let a7055 = v0;
            let a7056 = 1u64;
            nat_add(a7055, a7056)?
        };
        let a7058 = f250(v1.clone(), 13u64, v2)?;
        if nat_eq(a7057, a7058) {
            Ok(Adt9::C4({
                let a7059 = v2;
                let a7060 = 0u64;
                if nat_eq(a7059, a7060) {
                    2u64
                } else {
                    7u64
                }
            }, 1u64, 0u64, 2u64, 0u64, 3u64, 0u64))
        } else {
            let v3: u64 = {
                let a7061 = v0;
                let a7062 = 1u64;
                nat_sub(a7061, a7062)
            };
            if f289(v1.clone(), v2) {
                let v4: u64 = f262(match v1.clone() {
                    Adt3::C0(h7063, _, _, _, _, _, _, _) => {
                        h7063
                    }
                }, false)?;
                let v5: u64 = {
                    let a7064 = v3;
                    let a7065 = v4;
                    let a7066 = 0u64;
                    nat_quot(a7064, a7065, a7066)
                };
                let v6: u64 = {
                    let a7067 = v3;
                    let a7068 = v4;
                    let a7069 = 0u64;
                    nat_rem(a7067, a7068, a7069)
                };
                let a7072 = {
                    let a7070 = v6;
                    let a7071 = 1u64;
                    nat_add(a7070, a7071)?
                };
                let a7073 = v4;
                if nat_eq(a7072, a7073) {
                    Ok(Adt9::C4({
                        let a7074 = v2;
                        let a7075 = 0u64;
                        if nat_eq(a7074, a7075) {
                            0u64
                        } else {
                            4u64
                        }
                    }, 1u64, 0u64, 1u64, 1u64, 1u64, 0u64))
                } else {
                    f263(v6, v5, v1, v2)
                }
            } else {
                f292({
                    let a7076 = v3;
                    let a7077 = 2u64;
                    let a7078 = 0u64;
                    nat_quot(a7076, a7077, a7078)
                }, {
                    let a7079 = v3;
                    let a7080 = 2u64;
                    let a7081 = 0u64;
                    nat_rem(a7079, a7080, a7081)
                }, v1, v2, 0u64)
            }
        }
    }
}

pub fn f262(v0: u64, v1: bool) -> R<u64> {
    let a7084 = {
        let a7082 = v0;
        let a7083 = 2u64;
        nat_mul(a7082, a7083)?
    };
    let a7085 = if v1 {
        4u64
    } else {
        2u64
    };
    nat_add(a7084, a7085)
}

pub fn f263(v0: u64, v1: u64, v2: Adt3, v3: u64) -> R<Adt9> {
    let a7086 = v0;
    let a7087 = 0u64;
    if nat_eq(a7086, a7087) {
        Ok(f274(1u64))
    } else {
        f292({
            let a7096 = {
                let a7089 = v1;
                let a7090 = match v2.clone() {
                    Adt3::C0(h7088, _, _, _, _, _, _, _) => {
                        h7088
                    }
                };
                nat_mul(a7089, a7090)?
            };
            let a7097 = {
                let a7093 = {
                    let a7091 = v0;
                    let a7092 = 1u64;
                    nat_sub(a7091, a7092)
                };
                let a7094 = 2u64;
                let a7095 = 0u64;
                nat_quot(a7093, a7094, a7095)
            };
            nat_add(a7096, a7097)?
        }, {
            let a7100 = {
                let a7098 = v0;
                let a7099 = 1u64;
                nat_sub(a7098, a7099)
            };
            let a7101 = 2u64;
            let a7102 = 0u64;
            nat_rem(a7100, a7101, a7102)
        }, v2, v3, 1u64)
    }
}

pub fn f264(v0: u64, v1: u64) -> R<u64> {
    let a7103 = v0;
    let a7104 = f106(v1)?;
    let a7105 = 0u64;
    Ok(nat_rem(a7103, a7104, a7105))
}

pub fn f265(v0: u64, v1: u64) -> R<u64> {
    let a7106 = 4u64;
    let a7107 = f293(v0, v1)?;
    nat_mul(a7106, a7107)
}

pub fn f266(v0: u64, v1: u64) -> R<u64> {
    let a7108 = 4u64;
    let a7109 = f294(v0, v1)?;
    nat_mul(a7108, a7109)
}

pub fn f267(v0: u64, v1: u64) -> R<bool> {
    let a7115 = {
        let a7112 = {
            let a7110 = f264(v0, v1)?;
            let a7111 = 2u64;
            nat_sub(a7110, a7111)
        };
        let a7113 = 2u64;
        let a7114 = 0u64;
        nat_rem(a7112, a7113, a7114)
    };
    let a7116 = 0u64;
    Ok(nat_eq(a7115, a7116))
}

pub fn f268(v0: u64) -> u64 {
    let m7117 = f230(v0);
    match m7117 {
        Adt18::C0 => {
            0u64
        }
        Adt18::C1(v1) => {
            match v1 {
                Adt19::C0(_, h7118, _) => {
                    h7118
                }
            }
        }
    }
}

pub fn f269(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64, v7: u64, v8: u64, v9: u64, v10: u64, v11: u64, v12: u64) -> R<Option<u64>> {
    let m7119 = f230(v0);
    match m7119 {
        Adt18::C0 => {
            Ok(None::<u64>)
        }
        Adt18::C1(v13) => {
            if if if if if f238(v0) {
                let a7120 = v1;
                let a7121 = v7;
                nat_lt(a7120, a7121)
            } else {
                false
            } {
                f198(v8, v2)
            } else {
                false
            } {
                let a7122 = v3;
                let a7123 = f295(v4, v5, v6)?;
                nat_lt(a7122, a7123)
            } else {
                false
            } {
                let a7128 = {
                    let a7125 = v7;
                    let a7126 = match v13.clone() {
                        Adt19::C0(_, h7124, _) => {
                            h7124
                        }
                    };
                    let a7127 = 0u64;
                    nat_rem(a7125, a7126, a7127)
                };
                let a7129 = 0u64;
                nat_eq(a7128, a7129)
            } else {
                false
            } {
                Ok(Some({
                    let a7158 = {
                        let a7148 = {
                            let a7138 = {
                                let a7134 = {
                                    let a7131 = v1;
                                    let a7132 = match v13 {
                                        Adt19::C0(_, h7130, _) => {
                                            h7130
                                        }
                                    };
                                    let a7133 = 0u64;
                                    nat_quot(a7131, a7132, a7133)
                                };
                                let a7135 = v9;
                                nat_mul(a7134, a7135)?
                            };
                            let a7139 = {
                                let a7136 = v2;
                                let a7137 = v10;
                                nat_mul(a7136, a7137)?
                            };
                            nat_add(a7138, a7139)?
                        };
                        let a7149 = {
                            let a7146 = {
                                let a7143 = {
                                    let a7140 = v3;
                                    let a7141 = v4;
                                    let a7142 = 0u64;
                                    nat_quot(a7140, a7141, a7142)
                                };
                                let a7144 = v5;
                                let a7145 = 0u64;
                                nat_rem(a7143, a7144, a7145)
                            };
                            let a7147 = v11;
                            nat_mul(a7146, a7147)?
                        };
                        nat_add(a7148, a7149)?
                    };
                    let a7159 = {
                        let a7156 = {
                            let a7153 = {
                                let a7150 = v3;
                                let a7151 = v4;
                                let a7152 = 0u64;
                                nat_quot(a7150, a7151, a7152)
                            };
                            let a7154 = v5;
                            let a7155 = 0u64;
                            nat_quot(a7153, a7154, a7155)
                        };
                        let a7157 = v12;
                        nat_mul(a7156, a7157)?
                    };
                    nat_add(a7158, a7159)?
                }))
            } else {
                Ok(None::<u64>)
            }
        }
    }
}

pub fn f270(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64, v7: u64, v8: u64, v9: u64, v10: u64, v11: u64, v12: u64, v13: u64, v14: u64) -> R<Adt17> {
    f296(v0, Adt20::C0(v1, v2, v3, v4, v5, v6, v7, v8), Adt20::C0(v1, v2, v3, v4, v9, v10, v11, v12), v13, v14)
}

pub fn f271(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64, v7: u64, v8: u64, v9: u64, v10: u64, v11: u64, v12: u64, v13: u64, v14: u64, v15: u64, v16: u64, v17: u64, v18: u64, v19: u64, v20: u64) -> R<Adt17> {
    f297(v0, Adt20::C0(v1, v2, v3, v4, v5, v6, v7, v8), Adt20::C0(v1, v2, v3, v4, v9, v10, v11, v12), Adt20::C0(v1, v2, v3, v4, v13, v14, v15, v16), v17, v18, v19, v20)
}

pub fn f272(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64, v7: u64, v8: u64, v9: u64, v10: u64, v11: u64, v12: u64, v13: u64, v14: u64, v15: u64, v16: u64, v17: u64) -> R<Adt17> {
    let a7160 = v0;
    let a7161 = 64u64;
    if nat_eq(a7160, a7161) {
        f298(v1, Adt20::C0(v2, v3, v4, v5, v6, v7, v8, v9), Adt20::C0(v2, v3, v4, v5, v10, v11, v12, v13), v14, v15, v16, v17)
    } else {
        let a7162 = v0;
        let a7163 = 65u64;
        if nat_eq(a7162, a7163) {
            f299(v1, Adt20::C0(v2, v3, v4, v5, v6, v7, v8, v9), Adt20::C0(v2, v3, v4, v5, v10, v11, v12, v13), v14, v15, v16, v17)
        } else {
            Ok(Adt17::C0)
        }
    }
}

pub fn f273(v0: u64) -> u64 {
    let a7164 = v0;
    let a7165 = 30u64;
    if nat_eq(a7164, a7165) {
        4u64
    } else {
        3u64
    }
}

pub fn f274(v0: u64) -> Adt9 {
    Adt9::C4(0u64, 2u64, 0u64, 2u64, 0u64, 1u64, v0)
}

pub fn f275(v0: u64) -> bool {
    let a7166 = 0u64;
    let a7167 = v0;
    if nat_lt(a7166, a7167) {
        let a7168 = v0;
        let a7169 = 2139095040u64;
        nat_le(a7168, a7169)
    } else {
        false
    }
}

pub fn f276(v0: u64) -> u64 {
    let a7170 = v0;
    let a7171 = 40u64;
    if nat_lt(a7170, a7171) {
        let a7172 = v0;
        let a7173 = 20u64;
        if nat_lt(a7172, a7173) {
            let a7174 = v0;
            let a7175 = 10u64;
            if nat_lt(a7174, a7175) {
                let a7176 = v0;
                let a7177 = 5u64;
                if nat_lt(a7176, a7177) {
                    let a7178 = v0;
                    let a7179 = 2u64;
                    if nat_lt(a7178, a7179) {
                        let a7180 = v0;
                        let a7181 = 1u64;
                        if nat_lt(a7180, a7181) {
                            1u64
                        } else {
                            2u64
                        }
                    } else {
                        let a7182 = v0;
                        let a7183 = 3u64;
                        if nat_lt(a7182, a7183) {
                            4u64
                        } else {
                            let a7184 = v0;
                            let a7185 = 4u64;
                            if nat_lt(a7184, a7185) {
                                8u64
                            } else {
                                16u64
                            }
                        }
                    }
                } else {
                    let a7186 = v0;
                    let a7187 = 7u64;
                    if nat_lt(a7186, a7187) {
                        let a7188 = v0;
                        let a7189 = 6u64;
                        if nat_lt(a7188, a7189) {
                            32u64
                        } else {
                            64u64
                        }
                    } else {
                        let a7190 = v0;
                        let a7191 = 8u64;
                        if nat_lt(a7190, a7191) {
                            128u64
                        } else {
                            let a7192 = v0;
                            let a7193 = 9u64;
                            if nat_lt(a7192, a7193) {
                                256u64
                            } else {
                                512u64
                            }
                        }
                    }
                }
            } else {
                let a7194 = v0;
                let a7195 = 15u64;
                if nat_lt(a7194, a7195) {
                    let a7196 = v0;
                    let a7197 = 12u64;
                    if nat_lt(a7196, a7197) {
                        let a7198 = v0;
                        let a7199 = 11u64;
                        if nat_lt(a7198, a7199) {
                            1024u64
                        } else {
                            2048u64
                        }
                    } else {
                        let a7200 = v0;
                        let a7201 = 13u64;
                        if nat_lt(a7200, a7201) {
                            4096u64
                        } else {
                            let a7202 = v0;
                            let a7203 = 14u64;
                            if nat_lt(a7202, a7203) {
                                8192u64
                            } else {
                                16384u64
                            }
                        }
                    }
                } else {
                    let a7204 = v0;
                    let a7205 = 17u64;
                    if nat_lt(a7204, a7205) {
                        let a7206 = v0;
                        let a7207 = 16u64;
                        if nat_lt(a7206, a7207) {
                            32768u64
                        } else {
                            65536u64
                        }
                    } else {
                        let a7208 = v0;
                        let a7209 = 18u64;
                        if nat_lt(a7208, a7209) {
                            131072u64
                        } else {
                            let a7210 = v0;
                            let a7211 = 19u64;
                            if nat_lt(a7210, a7211) {
                                262144u64
                            } else {
                                524288u64
                            }
                        }
                    }
                }
            }
        } else {
            let a7212 = v0;
            let a7213 = 30u64;
            if nat_lt(a7212, a7213) {
                let a7214 = v0;
                let a7215 = 25u64;
                if nat_lt(a7214, a7215) {
                    let a7216 = v0;
                    let a7217 = 22u64;
                    if nat_lt(a7216, a7217) {
                        let a7218 = v0;
                        let a7219 = 21u64;
                        if nat_lt(a7218, a7219) {
                            1048576u64
                        } else {
                            2097152u64
                        }
                    } else {
                        let a7220 = v0;
                        let a7221 = 23u64;
                        if nat_lt(a7220, a7221) {
                            4194304u64
                        } else {
                            let a7222 = v0;
                            let a7223 = 24u64;
                            if nat_lt(a7222, a7223) {
                                8388608u64
                            } else {
                                16777216u64
                            }
                        }
                    }
                } else {
                    let a7224 = v0;
                    let a7225 = 27u64;
                    if nat_lt(a7224, a7225) {
                        let a7226 = v0;
                        let a7227 = 26u64;
                        if nat_lt(a7226, a7227) {
                            33554432u64
                        } else {
                            67108864u64
                        }
                    } else {
                        let a7228 = v0;
                        let a7229 = 28u64;
                        if nat_lt(a7228, a7229) {
                            134217728u64
                        } else {
                            let a7230 = v0;
                            let a7231 = 29u64;
                            if nat_lt(a7230, a7231) {
                                268435456u64
                            } else {
                                536870912u64
                            }
                        }
                    }
                }
            } else {
                let a7232 = v0;
                let a7233 = 35u64;
                if nat_lt(a7232, a7233) {
                    let a7234 = v0;
                    let a7235 = 32u64;
                    if nat_lt(a7234, a7235) {
                        let a7236 = v0;
                        let a7237 = 31u64;
                        if nat_lt(a7236, a7237) {
                            1073741824u64
                        } else {
                            2147483648u64
                        }
                    } else {
                        let a7238 = v0;
                        let a7239 = 33u64;
                        if nat_lt(a7238, a7239) {
                            4294967296u64
                        } else {
                            let a7240 = v0;
                            let a7241 = 34u64;
                            if nat_lt(a7240, a7241) {
                                8589934592u64
                            } else {
                                17179869184u64
                            }
                        }
                    }
                } else {
                    let a7242 = v0;
                    let a7243 = 37u64;
                    if nat_lt(a7242, a7243) {
                        let a7244 = v0;
                        let a7245 = 36u64;
                        if nat_lt(a7244, a7245) {
                            34359738368u64
                        } else {
                            68719476736u64
                        }
                    } else {
                        let a7246 = v0;
                        let a7247 = 38u64;
                        if nat_lt(a7246, a7247) {
                            137438953472u64
                        } else {
                            let a7248 = v0;
                            let a7249 = 39u64;
                            if nat_lt(a7248, a7249) {
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

pub fn f277(v0: u64, v1: u64) -> bool {
    let a7250 = v0;
    let a7251 = 1u64;
    if nat_le(a7250, a7251) {
        true
    } else {
        let a7255 = {
            let a7252 = v1;
            let a7253 = 4u64;
            let a7254 = 0u64;
            nat_rem(a7252, a7253, a7254)
        };
        let a7256 = 0u64;
        nat_eq(a7255, a7256)
    }
}

pub fn f278(v0: u64, v1: u64) -> bool {
    let a7257 = v0;
    let a7258 = 1u64;
    if nat_le(a7257, a7258) {
        true
    } else {
        let a7259 = 4u64;
        let a7260 = v1;
        nat_le(a7259, a7260)
    }
}

pub fn f279(v0: u64, v1: u64) -> R<u64> {
    let a7261 = v0;
    let a7262 = 0u64;
    if nat_eq(a7261, a7262) {
        Ok(0u64)
    } else {
        let a7267 = 4u64;
        let a7268 = {
            let a7265 = {
                let a7263 = v0;
                let a7264 = 1u64;
                nat_sub(a7263, a7264)
            };
            let a7266 = v1;
            nat_mul(a7265, a7266)?
        };
        nat_add(a7267, a7268)
    }
}

pub fn f280(v0: bool, v1: u64) -> R<u64> {
    let a7273 = {
        let a7271 = {
            let a7269 = f300(v0, v1)?;
            let a7270 = f301(v0, v1)?;
            nat_add(a7269, a7270)?
        };
        let a7272 = f302(v0);
        nat_add(a7271, a7272)?
    };
    let a7274 = v1;
    nat_add(a7273, a7274)
}

pub fn f281(v0: u64, v1: u64) -> R<u64> {
    let v2: u64 = f303(v0, v1)?;
    let v3: u64 = {
        let a7279 = {
            let a7277 = {
                let a7275 = v1;
                let a7276 = 1u64;
                nat_sub(a7275, a7276)
            };
            let a7278 = v2;
            nat_mul(a7277, a7278)?
        };
        let a7280 = v0;
        nat_sub(a7279, a7280)
    };
    let a7281 = v2;
    let a7282 = v3;
    if nat_lt(a7281, a7282) {
        Ok(v3)
    } else {
        Ok(v2)
    }
}

pub fn f282(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64) -> R<u64> {
    let a7283 = v4;
    let a7284 = 0u64;
    if nat_eq(a7283, a7284) {
        Ok(0u64)
    } else {
        let a7291 = 2u64;
        let a7292 = {
            let a7289 = {
                let a7285 = v0;
                let a7286 = v1;
                nat_mul(a7285, a7286)?
            };
            let a7290 = {
                let a7287 = v2;
                let a7288 = v3;
                nat_mul(a7287, a7288)?
            };
            nat_mul(a7289, a7290)?
        };
        nat_mul(a7291, a7292)
    }
}

pub fn f283(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64) -> R<u64> {
    let a7297 = v3;
    let a7298 = {
        let a7295 = {
            let a7293 = f304(v2, v4)?;
            let a7294 = f305(v0, v1, f303(v2, v4)?)?;
            nat_mul(a7293, a7294)?
        };
        let a7296 = f305(v0, v1, f306(v2, v4)?)?;
        nat_add(a7295, a7296)?
    };
    nat_mul(a7297, a7298)
}

pub fn f284(v0: u64) -> R<u64> {
    let a7301 = 13u64;
    let a7302 = {
        let a7299 = 9u64;
        let a7300 = v0;
        nat_mul(a7299, a7300)?
    };
    nat_add(a7301, a7302)
}

pub fn f285(v0: u64, v1: u64, _: u64, v3: u64, v4: u64, v5: u64, v6: u64, v7: u64, v8: u64, v9: u64, v10: u64, v11: u64, v12: u64, v13: u64, v14: u64) -> bool {
    if if if if if if if if if f307(v0, v1, v6, v7, v8) {
        let a7303 = v5;
        let a7304 = 26u64;
        nat_eq(a7303, a7304)
    } else {
        false
    } {
        let a7305 = v3;
        let a7306 = v7;
        nat_eq(a7305, a7306)
    } else {
        false
    } {
        let a7307 = v4;
        let a7308 = v8;
        nat_eq(a7307, a7308)
    } else {
        false
    } {
        let a7309 = v9;
        let a7310 = 1u64;
        nat_eq(a7309, a7310)
    } else {
        false
    } {
        let a7311 = v10;
        let a7312 = f308(v0);
        nat_eq(a7311, a7312)
    } else {
        false
    } {
        let a7313 = v11;
        let a7314 = v1;
        nat_eq(a7313, a7314)
    } else {
        false
    } {
        let a7315 = v12;
        let a7316 = v6;
        nat_eq(a7315, a7316)
    } else {
        false
    } {
        let a7317 = v13;
        let a7318 = v7;
        nat_eq(a7317, a7318)
    } else {
        false
    } {
        let a7319 = v14;
        let a7320 = v8;
        nat_eq(a7319, a7320)
    } else {
        false
    }
}

pub fn f286(v0: u64, v1: u64, v2: u64, v3: u64) -> bool {
    let a7321 = v0;
    let a7322 = 0u64;
    if if if nat_eq(a7321, a7322) {
        true
    } else {
        let a7323 = v1;
        let a7324 = 0u64;
        nat_eq(a7323, a7324)
    } {
        true
    } else {
        let a7325 = v2;
        let a7326 = 0u64;
        nat_eq(a7325, a7326)
    } {
        true
    } else {
        let a7327 = v3;
        let a7328 = 0u64;
        nat_eq(a7327, a7328)
    }
}

pub fn f287(v0: Adt3, v1: bool) -> R<u64> {
    f291(v0.clone(), {
        let a7330 = match v0.clone() {
            Adt3::C0(h7329, _, _, _, _, _, _, _) => {
                h7329
            }
        };
        let a7331 = 1u64;
        nat_sub(a7330, a7331)
    }, {
        let a7333 = match v0.clone() {
            Adt3::C0(_, h7332, _, _, _, _, _, _) => {
                h7332
            }
        };
        let a7334 = 1u64;
        nat_sub(a7333, a7334)
    }, {
        let a7336 = match v0.clone() {
            Adt3::C0(_, _, h7335, _, _, _, _, _) => {
                h7335
            }
        };
        let a7337 = 1u64;
        nat_sub(a7336, a7337)
    }, {
        let a7339 = match v0 {
            Adt3::C0(_, _, _, h7338, _, _, _, _) => {
                h7338
            }
        };
        let a7340 = 1u64;
        nat_sub(a7339, a7340)
    }, v1)
}

pub fn f288(v0: u64, v1: u64, v2: u64, v3: u64) -> R<u64> {
    let a7341 = v0;
    let a7342 = 0u64;
    if nat_eq(a7341, a7342) {
        Ok(0u64)
    } else {
        let v4: u64 = {
            let a7343 = v0;
            let a7344 = 1u64;
            nat_sub(a7343, a7344)
        };
        let v5: u64 = {
            let a7345 = v4;
            let a7346 = v1;
            let a7347 = 0u64;
            nat_quot(a7345, a7346, a7347)
        };
        let v6: u64 = {
            let a7355 = {
                let a7348 = v5;
                let a7349 = v2;
                nat_mul(a7348, a7349)?
            };
            let a7356 = {
                let a7353 = {
                    let a7350 = v4;
                    let a7351 = v1;
                    let a7352 = 0u64;
                    nat_rem(a7350, a7351, a7352)
                };
                let a7354 = v3;
                nat_mul(a7353, a7354)?
            };
            nat_add(a7355, a7356)?
        };
        let a7357 = v5;
        let a7358 = 0u64;
        if nat_eq(a7357, a7358) {
            Ok(v6)
        } else {
            let v7: u64 = {
                let a7367 = {
                    let a7361 = {
                        let a7359 = v5;
                        let a7360 = 1u64;
                        nat_sub(a7359, a7360)
                    };
                    let a7362 = v2;
                    nat_mul(a7361, a7362)?
                };
                let a7368 = {
                    let a7365 = {
                        let a7363 = v1;
                        let a7364 = 1u64;
                        nat_sub(a7363, a7364)
                    };
                    let a7366 = v3;
                    nat_mul(a7365, a7366)?
                };
                nat_add(a7367, a7368)?
            };
            let a7369 = v6;
            let a7370 = v7;
            if nat_lt(a7369, a7370) {
                Ok(v7)
            } else {
                Ok(v6)
            }
        }
    }
}

pub fn f289(v0: Adt3, v1: u64) -> bool {
    let a7371 = v1;
    let a7372 = 0u64;
    if !nat_eq(a7371, a7372) {
        true
    } else {
        let a7374 = match v0 {
            Adt3::C0(_, _, _, _, h7373, _, _, _) => {
                h7373
            }
        };
        let a7375 = 4u64;
        nat_eq(a7374, a7375)
    }
}

pub fn f290(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: bool, v6: u64, v7: u64, v8: u64, v9: u64, v10: u64, v11: u64, v12: u64, v13: u64, v14: u64, v15: u64, v16: u64, v17: u64) -> R<Adt10> {
    let m7376 = f309(v0, v1);
    match m7376 {
        None => {
            if v5 {
                let m7377 = f310(v0, v2, v3, v4, v6, v7, v8, v9, v10, v11, v12, v13, v14, v15, v16, v17)?;
                match m7377 {
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
                let a7378 = 4u64;
                let a7379 = v28;
                nat_le(a7378, a7379)
            };
            let a7380 = v2;
            let a7381 = 0u64;
            if if if if if nat_eq(a7380, a7381) {
                let a7382 = v4;
                let a7383 = 0u64;
                nat_eq(a7382, a7383)
            } else {
                false
            } {
                let a7384 = v6;
                let a7385 = v14;
                if nat_eq(a7384, a7385) {
                    let a7386 = v7;
                    let a7387 = v15;
                    nat_eq(a7386, a7387)
                } else {
                    false
                }
            } else {
                false
            } {
                let a7388 = v8;
                let a7389 = v16;
                if nat_eq(a7388, a7389) {
                    let a7390 = v9;
                    let a7391 = v17;
                    nat_eq(a7390, a7391)
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
                        let a7392 = v3;
                        let a7393 = 0u64;
                        nat_eq(a7392, a7393)
                    } else {
                        false
                    } {
                        f241(v6, v7, v8, v9, v10, v11, v12, v13)
                    } else {
                        false
                    }
                }
            } else {
                false
            } {
                Ok(Adt10::C2(v28, v29, v6, v7, v8, v9, {
                    let a7398 = {
                        let a7394 = v6;
                        let a7395 = v7;
                        nat_mul(a7394, a7395)?
                    };
                    let a7399 = {
                        let a7396 = v8;
                        let a7397 = v9;
                        nat_mul(a7396, a7397)?
                    };
                    nat_mul(a7398, a7399)?
                }))
            } else {
                Ok(Adt10::C1)
            }
        }
    }
}

pub fn f291(v0: Adt3, v1: u64, v2: u64, v3: u64, v4: u64, v5: bool) -> R<u64> {
    let a7420 = {
        let a7414 = {
            let a7408 = {
                let a7402 = v1;
                let a7403 = if v5 {
                    f155(match v0.clone() {
                        Adt3::C0(_, _, _, _, h7400, _, _, _) => {
                            h7400
                        }
                    })
                } else {
                    f156(match v0.clone() {
                        Adt3::C0(_, _, _, _, h7401, _, _, _) => {
                            h7401
                        }
                    })
                };
                nat_mul(a7402, a7403)?
            };
            let a7409 = {
                let a7406 = v2;
                let a7407 = if v5 {
                    f155(match v0.clone() {
                        Adt3::C0(_, _, _, _, _, h7404, _, _) => {
                            h7404
                        }
                    })
                } else {
                    f156(match v0.clone() {
                        Adt3::C0(_, _, _, _, _, h7405, _, _) => {
                            h7405
                        }
                    })
                };
                nat_mul(a7406, a7407)?
            };
            nat_add(a7408, a7409)?
        };
        let a7415 = {
            let a7412 = v3;
            let a7413 = if v5 {
                f155(match v0.clone() {
                    Adt3::C0(_, _, _, _, _, _, h7410, _) => {
                        h7410
                    }
                })
            } else {
                f156(match v0.clone() {
                    Adt3::C0(_, _, _, _, _, _, h7411, _) => {
                        h7411
                    }
                })
            };
            nat_mul(a7412, a7413)?
        };
        nat_add(a7414, a7415)?
    };
    let a7421 = {
        let a7418 = v4;
        let a7419 = if v5 {
            f155(match v0 {
                Adt3::C0(_, _, _, _, _, _, _, h7416) => {
                    h7416
                }
            })
        } else {
            f156(match v0 {
                Adt3::C0(_, _, _, _, _, _, _, h7417) => {
                    h7417
                }
            })
        };
        nat_mul(a7418, a7419)?
    };
    nat_add(a7420, a7421)
}

pub fn f292(v0: u64, v1: u64, v2: Adt3, v3: u64, v4: u64) -> R<Adt9> {
    let a7422 = v1;
    let a7423 = 0u64;
    if nat_eq(a7422, a7423) {
        Ok(Adt9::C4({
            let a7424 = v3;
            let a7425 = 0u64;
            if nat_eq(a7424, a7425) {
                1u64
            } else {
                6u64
            }
        }, 0u64, f103(v0, v2)?, 2u64, 0u64, 1u64, 2u64))
    } else {
        Ok(Adt9::C4({
            let a7426 = v3;
            let a7427 = 0u64;
            if nat_eq(a7426, a7427) {
                0u64
            } else {
                4u64
            }
        }, 1u64, v4, 1u64, 2u64, 1u64, v4))
    }
}

pub fn f293(v0: u64, v1: u64) -> R<u64> {
    let a7428 = v0;
    let a7429 = f106(v1)?;
    let a7430 = 0u64;
    Ok(nat_quot(a7428, a7429, a7430))
}

pub fn f294(v0: u64, v1: u64) -> R<u64> {
    let v2: u64 = f293(v0, v1)?;
    let v3: u64 = {
        let a7433 = {
            let a7431 = f264(v0, v1)?;
            let a7432 = 2u64;
            nat_sub(a7431, a7432)
        };
        let a7434 = 2u64;
        let a7435 = 0u64;
        nat_quot(a7433, a7434, a7435)
    };
    let v4: u64 = {
        let a7436 = v2;
        let a7437 = v1;
        nat_mul(a7436, a7437)?
    };
    if f267(v0, v1)? {
        let a7438 = v4;
        let a7439 = v3;
        nat_add(a7438, a7439)
    } else {
        let a7451 = {
            let a7447 = v4;
            let a7448 = {
                let a7445 = {
                    let a7440 = v1;
                    let a7441 = 1u64;
                    nat_sub(a7440, a7441)
                };
                let a7446 = {
                    let a7442 = v2;
                    let a7443 = v1;
                    let a7444 = 0u64;
                    nat_rem(a7442, a7443, a7444)
                };
                nat_mul(a7445, a7446)?
            };
            nat_sub(a7447, a7448)
        };
        let a7452 = {
            let a7449 = v3;
            let a7450 = v1;
            nat_mul(a7449, a7450)?
        };
        nat_add(a7451, a7452)
    }
}

pub fn f295(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let a7453 = v0;
    let a7454 = 0u64;
    if if if nat_eq(a7453, a7454) {
        true
    } else {
        let a7455 = v1;
        let a7456 = 0u64;
        nat_eq(a7455, a7456)
    } {
        true
    } else {
        let a7457 = v2;
        let a7458 = 0u64;
        nat_eq(a7457, a7458)
    } {
        Ok(0u64)
    } else {
        let a7461 = {
            let a7459 = v0;
            let a7460 = v1;
            nat_mul(a7459, a7460)?
        };
        let a7462 = v2;
        nat_mul(a7461, a7462)
    }
}

pub fn f296(v0: u64, v1: Adt20, v2: Adt20, v3: u64, v4: u64) -> R<Adt17> {
    let v5: u64 = f115(match v1.clone() {
        Adt20::C0(h7463, _, _, _, _, _, _, _) => {
            h7463
        }
    })?;
    let v6: u64 = {
        let a7469 = match v1.clone() {
            Adt20::C0(_, h7464, _, _, _, _, _, _) => {
                h7464
            }
        };
        let a7470 = {
            let a7467 = match v1.clone() {
                Adt20::C0(_, _, h7465, _, _, _, _, _) => {
                    h7465
                }
            };
            let a7468 = match v1.clone() {
                Adt20::C0(_, _, _, h7466, _, _, _, _) => {
                    h7466
                }
            };
            nat_mul(a7467, a7468)?
        };
        nat_mul(a7469, a7470)?
    };
    let a7472 = v0;
    let a7473 = f58(match v1.clone() {
        Adt20::C0(h7471, _, _, _, _, _, _, _) => {
            h7471
        }
    }, v6)?;
    if nat_lt(a7472, a7473) {
        f311({
            let a7474 = v0;
            let a7475 = v5;
            let a7476 = 0u64;
            nat_rem(a7474, a7475, a7476)
        }, {
            let a7477 = v0;
            let a7478 = v5;
            let a7479 = 0u64;
            nat_quot(a7477, a7478, a7479)
        }, v1, v2, v3, v4)
    } else {
        Ok(Adt17::C0)
    }
}

pub fn f297(v0: u64, v1: Adt20, v2: Adt20, v3: Adt20, v4: u64, v5: u64, v6: u64, v7: u64) -> R<Adt17> {
    let v8: u64 = {
        let a7480 = v6;
        let a7481 = 6u64;
        nat_sub(a7480, a7481)
    };
    let v9: Adt20 = f312(v3);
    let a7483 = v0;
    let a7484 = f227(match v1.clone() {
        Adt20::C0(h7482, _, _, _, _, _, _, _) => {
            h7482
        }
    }, v5, v8)?;
    if nat_lt(a7483, a7484) {
        let v10: u64 = f112(match v1.clone() {
            Adt20::C0(h7485, _, _, _, _, _, _, _) => {
                h7485
            }
        }, v5, v8)?;
        let a7486 = v0;
        let a7487 = v10;
        if nat_lt(a7486, a7487) {
            f313(v0, v9, 4u64, 6u64, v7)
        } else {
            let v11: u64 = {
                let a7488 = v0;
                let a7489 = v10;
                nat_sub(a7488, a7489)
            };
            let v12: u64 = f284(match v1.clone() {
                Adt20::C0(h7490, _, _, _, _, _, _, _) => {
                    h7490
                }
            })?;
            f314({
                let a7491 = v11;
                let a7492 = v12;
                let a7493 = 0u64;
                nat_rem(a7491, a7492, a7493)
            }, {
                let a7494 = v11;
                let a7495 = v12;
                let a7496 = 0u64;
                nat_quot(a7494, a7495, a7496)
            }, v1, v2, v9, v4)
        }
    } else {
        Ok(Adt17::C0)
    }
}

pub fn f298(v0: u64, v1: Adt20, v2: Adt20, v3: u64, v4: u64, v5: u64, v6: u64) -> R<Adt17> {
    let v7: u64 = {
        let a7497 = v5;
        let a7498 = 6u64;
        nat_sub(a7497, a7498)
    };
    let v8: bool = f221(match v1.clone() {
        Adt20::C0(_, _, _, _, h7499, _, _, _) => {
            h7499
        }
    }, match v2.clone() {
        Adt20::C0(_, _, _, _, h7500, _, _, _) => {
            h7500
        }
    });
    let a7502 = v0;
    let a7503 = f220(v8, match v1.clone() {
        Adt20::C0(h7501, _, _, _, _, _, _, _) => {
            h7501
        }
    }, v4, v7)?;
    if nat_lt(a7502, a7503) {
        let v9: u64 = f112(match v1.clone() {
            Adt20::C0(h7504, _, _, _, _, _, _, _) => {
                h7504
            }
        }, v4, v7)?;
        let a7505 = v0;
        let a7506 = v9;
        if nat_lt(a7505, a7506) {
            f313(v0, v2, 4u64, 6u64, v6)
        } else {
            let v10: u64 = {
                let a7507 = v0;
                let a7508 = v9;
                nat_sub(a7507, a7508)
            };
            let v11: u64 = f280(v8, match v1.clone() {
                Adt20::C0(h7509, _, _, _, _, _, _, _) => {
                    h7509
                }
            })?;
            f315({
                let a7510 = v10;
                let a7511 = v11;
                let a7512 = 0u64;
                nat_rem(a7510, a7511, a7512)
            }, {
                let a7513 = v10;
                let a7514 = v11;
                let a7515 = 0u64;
                nat_quot(a7513, a7514, a7515)
            }, v1, v2, v3, v8)
        }
    } else {
        Ok(Adt17::C0)
    }
}

pub fn f299(v0: u64, v1: Adt20, v2: Adt20, v3: u64, v4: u64, v5: u64, v6: u64) -> R<Adt17> {
    let v7: u64 = {
        let a7516 = v5;
        let a7517 = 6u64;
        nat_sub(a7516, a7517)
    };
    let v8: Adt20 = f312(v2);
    let v9: u64 = f282(match v1.clone() {
        Adt20::C0(h7518, _, _, _, _, _, _, _) => {
            h7518
        }
    }, match v1.clone() {
        Adt20::C0(_, h7519, _, _, _, _, _, _) => {
            h7519
        }
    }, match v1.clone() {
        Adt20::C0(_, _, h7520, _, _, _, _, _) => {
            h7520
        }
    }, match v1.clone() {
        Adt20::C0(_, _, _, h7521, _, _, _, _) => {
            h7521
        }
    }, v7)?;
    let a7526 = v0;
    let a7527 = f226(match v1.clone() {
        Adt20::C0(h7522, _, _, _, _, _, _, _) => {
            h7522
        }
    }, match v1.clone() {
        Adt20::C0(_, h7523, _, _, _, _, _, _) => {
            h7523
        }
    }, match v1.clone() {
        Adt20::C0(_, _, h7524, _, _, _, _, _) => {
            h7524
        }
    }, match v1.clone() {
        Adt20::C0(_, _, _, h7525, _, _, _, _) => {
            h7525
        }
    }, v4, v7)?;
    if nat_lt(a7526, a7527) {
        let a7528 = v0;
        let a7529 = v9;
        if nat_lt(a7528, a7529) {
            f313(v0, v8, 4u64, 6u64, v6)
        } else {
            let v10: Adt21 = f316({
                let a7530 = v0;
                let a7531 = v9;
                nat_sub(a7530, a7531)
            }, match v1.clone() {
                Adt20::C0(h7532, _, _, _, _, _, _, _) => {
                    h7532
                }
            }, match v1.clone() {
                Adt20::C0(_, h7533, _, _, _, _, _, _) => {
                    h7533
                }
            }, match v1.clone() {
                Adt20::C0(_, _, h7534, _, _, _, _, _) => {
                    h7534
                }
            }, match v1.clone() {
                Adt20::C0(_, _, _, h7535, _, _, _, _) => {
                    h7535
                }
            }, v4)?;
            f317(v10, v1, v8, v3)
        }
    } else {
        Ok(Adt17::C0)
    }
}

pub fn f300(v0: bool, v1: u64) -> R<u64> {
    let a7538 = if v0 {
        5u64
    } else {
        4u64
    };
    let a7539 = {
        let a7536 = f318(v0);
        let a7537 = v1;
        nat_mul(a7536, a7537)?
    };
    nat_add(a7538, a7539)
}

pub fn f301(v0: bool, v1: u64) -> R<u64> {
    if v0 {
        let a7547 = {
            let a7540 = 13u64;
            let a7541 = f319(v1);
            nat_mul(a7540, a7541)?
        };
        let a7548 = {
            let a7545 = 4u64;
            let a7546 = {
                let a7542 = v1;
                let a7543 = 4u64;
                let a7544 = 0u64;
                nat_rem(a7542, a7543, a7544)
            };
            nat_mul(a7545, a7546)?
        };
        nat_add(a7547, a7548)
    } else {
        let a7549 = 3u64;
        let a7550 = v1;
        nat_mul(a7549, a7550)
    }
}

pub fn f302(v0: bool) -> u64 {
    if v0 {
        6u64
    } else {
        5u64
    }
}

pub fn f303(v0: u64, v1: u64) -> R<u64> {
    let a7555 = {
        let a7553 = {
            let a7551 = v0;
            let a7552 = v1;
            nat_add(a7551, a7552)?
        };
        let a7554 = 1u64;
        nat_sub(a7553, a7554)
    };
    let a7556 = v1;
    let a7557 = 0u64;
    Ok(nat_quot(a7555, a7556, a7557))
}

pub fn f304(v0: u64, v1: u64) -> R<u64> {
    let a7558 = v0;
    let a7559 = f303(v0, v1)?;
    let a7560 = 0u64;
    Ok(nat_quot(a7558, a7559, a7560))
}

pub fn f305(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let a7561 = v2;
    let a7562 = 0u64;
    if nat_eq(a7561, a7562) {
        Ok(0u64)
    } else {
        let a7567 = 11u64;
        let a7568 = {
            let a7565 = {
                let a7563 = v1;
                let a7564 = v2;
                nat_mul(a7563, a7564)?
            };
            let a7566 = f320(v0)?;
            nat_mul(a7565, a7566)?
        };
        nat_add(a7567, a7568)
    }
}

pub fn f306(v0: u64, v1: u64) -> R<u64> {
    let a7569 = v0;
    let a7570 = f303(v0, v1)?;
    let a7571 = 0u64;
    Ok(nat_rem(a7569, a7570, a7571))
}

pub fn f307(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64) -> bool {
    if f286(v1, v2, v3, v4) {
        f321(v0)
    } else {
        f238(v0)
    }
}

pub fn f308(v0: u64) -> u64 {
    let a7572 = v0;
    let a7573 = 26u64;
    if nat_eq(a7572, a7573) {
        26u64
    } else {
        0u64
    }
}

pub fn f309(v0: u64, v1: u64) -> Option<u64> {
    let a7574 = v0;
    let a7575 = 2u64;
    if nat_eq(a7574, a7575) {
        Some(0u64)
    } else {
        let a7576 = v0;
        let a7577 = 5u64;
        if nat_eq(a7576, a7577) {
            Some(1u64)
        } else {
            let a7578 = v0;
            let a7579 = 6u64;
            if nat_eq(a7578, a7579) {
                Some(2u64)
            } else {
                let a7580 = v0;
                let a7581 = 7u64;
                if nat_eq(a7580, a7581) {
                    Some(3u64)
                } else {
                    let a7582 = v0;
                    let a7583 = 9u64;
                    if nat_eq(a7582, a7583) {
                        Some(4u64)
                    } else {
                        let a7584 = v0;
                        let a7585 = 8u64;
                        if nat_eq(a7584, a7585) {
                            Some(5u64)
                        } else {
                            let a7586 = v0;
                            let a7587 = 89u64;
                            if nat_eq(a7586, a7587) {
                                let a7588 = v1;
                                let a7589 = 2u64;
                                if nat_eq(a7588, a7589) {
                                    Some(6u64)
                                } else {
                                    let a7590 = v1;
                                    let a7591 = 0u64;
                                    if nat_eq(a7590, a7591) {
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

pub fn f310(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64, v7: u64, v8: u64, v9: u64, v10: u64, v11: u64, v12: u64, v13: u64, v14: u64, v15: u64) -> R<Adt22> {
    let a7592 = v0;
    let a7593 = 30u64;
    if nat_eq(a7592, a7593) {
        let a7594 = v2;
        let a7595 = 0u64;
        if if if nat_eq(a7594, a7595) {
            true
        } else {
            let a7596 = v2;
            let a7597 = 1u64;
            nat_eq(a7596, a7597)
        } {
            let a7598 = v1;
            let a7599 = 0u64;
            if nat_eq(a7598, a7599) {
                true
            } else {
                f322(v1, v4)
            }
        } else {
            false
        } {
            f323(28u64, 0u64, 0u64, v3, v5, v4, v6, v7, v9, v8, v10, v11, v12, v13, v14, v15)
        } else {
            Ok(Adt22::C0)
        }
    } else {
        if if if if f324(v1) {
            f324(v2)
        } else {
            false
        } {
            let a7600 = v1;
            let a7601 = 0u64;
            if if nat_eq(a7600, a7601) {
                true
            } else {
                let a7602 = v2;
                let a7603 = 0u64;
                nat_eq(a7602, a7603)
            } {
                true
            } else {
                let a7604 = v1;
                let a7605 = v2;
                nat_eq(a7604, a7605)
            }
        } else {
            false
        } {
            true
        } else {
            let a7606 = v1;
            let a7607 = 43u64;
            if nat_eq(a7606, a7607) {
                f324(v2)
            } else {
                false
            }
        } {
            f323(v0, 0u64, 0u64, v3, v4, v5, v6, v7, v8, v9, v10, v11, v12, v13, v14, v15)
        } else {
            Ok(Adt22::C0)
        }
    }
}

pub fn f311(v0: u64, v1: u64, v2: Adt20, v3: Adt20, v4: u64, v5: u64) -> R<Adt17> {
    let v6: u64 = match v2.clone() {
        Adt20::C0(h7608, _, _, _, _, _, _, _) => {
            h7608
        }
    };
    let v7: u64 = {
        let a7611 = 1u64;
        let a7612 = {
            let a7609 = 3u64;
            let a7610 = v6;
            nat_mul(a7609, a7610)?
        };
        nat_add(a7611, a7612)?
    };
    let a7613 = v0;
    let a7614 = 0u64;
    if nat_eq(a7613, a7614) {
        Ok(f325(0u64))
    } else {
        let a7615 = v0;
        let a7616 = v7;
        if nat_lt(a7615, a7616) {
            let v8: u64 = {
                let a7617 = v0;
                let a7618 = 1u64;
                nat_sub(a7617, a7618)
            };
            let v9: u64 = {
                let a7624 = {
                    let a7619 = v1;
                    let a7620 = v6;
                    nat_mul(a7619, a7620)?
                };
                let a7625 = {
                    let a7621 = v8;
                    let a7622 = 3u64;
                    let a7623 = 0u64;
                    nat_quot(a7621, a7622, a7623)
                };
                nat_add(a7624, a7625)?
            };
            let v10: u64 = f326(v9, v2)?;
            let v11: u64 = {
                let a7626 = v8;
                let a7627 = 3u64;
                let a7628 = 0u64;
                nat_rem(a7626, a7627, a7628)
            };
            let a7629 = v11;
            let a7630 = 0u64;
            if nat_eq(a7629, a7630) {
                Ok(f327(10u64, 0u64, v10, 0u64, v10, 1u64, 1u64))
            } else {
                let a7631 = v11;
                let a7632 = 1u64;
                if nat_eq(a7631, a7632) {
                    Ok(f327(1u64, 1u64, 1u64, 2u64, 0u64, 1u64, 1u64))
                } else {
                    Ok(f327(0u64, 1u64, 0u64, 1u64, 1u64, 1u64, 0u64))
                }
            }
        } else {
            let a7633 = v0;
            let a7634 = v7;
            if nat_eq(a7633, a7634) {
                Ok(f327(2u64, 1u64, 0u64, 2u64, 0u64, 1u64, 1u64))
            } else {
                let a7637 = v0;
                let a7638 = {
                    let a7635 = v7;
                    let a7636 = 1u64;
                    nat_add(a7635, a7636)?
                };
                if nat_eq(a7637, a7638) {
                    Ok(f327(12u64, 1u64, 1u64, 2u64, 0u64, 1u64, 1u64))
                } else {
                    let a7641 = v0;
                    let a7642 = {
                        let a7639 = v7;
                        let a7640 = 2u64;
                        nat_add(a7639, a7640)?
                    };
                    if nat_eq(a7641, a7642) {
                        Ok(f327(16u64, 1u64, 1u64, 2u64, v4, 1u64, 2u64))
                    } else {
                        let a7645 = v0;
                        let a7646 = {
                            let a7643 = v7;
                            let a7644 = 3u64;
                            nat_add(a7643, a7644)?
                        };
                        if nat_eq(a7645, a7646) {
                            let a7647 = v5;
                            let a7648 = 1u64;
                            if if nat_eq(a7647, a7648) {
                                true
                            } else {
                                let a7649 = v5;
                                let a7650 = 2u64;
                                nat_eq(a7649, a7650)
                            } {
                                Ok(f327(11u64, 2u64, 1065353216u64, 1u64, 1u64, 1u64, 2u64))
                            } else {
                                Ok(f327(11u64, 2u64, 1065353216u64, 2u64, v4, 1u64, 2u64))
                            }
                        } else {
                            let v12: u64 = {
                                let a7657 = {
                                    let a7651 = v1;
                                    let a7652 = v6;
                                    nat_mul(a7651, a7652)?
                                };
                                let a7658 = {
                                    let a7655 = v0;
                                    let a7656 = {
                                        let a7653 = v7;
                                        let a7654 = 4u64;
                                        nat_add(a7653, a7654)?
                                    };
                                    nat_sub(a7655, a7656)
                                };
                                nat_add(a7657, a7658)?
                            };
                            Ok(f327(10u64, 0u64, f326(v12, v2)?, 1u64, 2u64, 3u64, f326(v12, v3)?))
                        }
                    }
                }
            }
        }
    }
}

pub fn f312(v0: Adt20) -> Adt20 {
    Adt20::C0(match v0.clone() {
        Adt20::C0(h7659, _, _, _, _, _, _, _) => {
            h7659
        }
    }, match v0.clone() {
        Adt20::C0(_, h7660, _, _, _, _, _, _) => {
            h7660
        }
    }, match v0.clone() {
        Adt20::C0(_, _, h7661, _, _, _, _, _) => {
            h7661
        }
    }, match v0.clone() {
        Adt20::C0(_, _, _, h7662, _, _, _, _) => {
            h7662
        }
    }, 4u64, match v0.clone() {
        Adt20::C0(_, _, _, _, _, h7663, _, _) => {
            h7663
        }
    }, match v0.clone() {
        Adt20::C0(_, _, _, _, _, _, h7664, _) => {
            h7664
        }
    }, match v0 {
        Adt20::C0(_, _, _, _, _, _, _, h7665) => {
            h7665
        }
    })
}

pub fn f313(v0: u64, v1: Adt20, v2: u64, v3: u64, v4: u64) -> R<Adt17> {
    let v5: u64 = {
        let a7672 = v3;
        let a7673 = {
            let a7669 = f326({
                let a7666 = v0;
                let a7667 = 2u64;
                let a7668 = 0u64;
                nat_quot(a7666, a7667, a7668)
            }, v1)?;
            let a7670 = v2;
            let a7671 = 0u64;
            nat_quot(a7669, a7670, a7671)
        };
        nat_add(a7672, a7673)?
    };
    let a7677 = {
        let a7674 = v0;
        let a7675 = 2u64;
        let a7676 = 0u64;
        nat_rem(a7674, a7675, a7676)
    };
    let a7678 = 0u64;
    if nat_eq(a7677, a7678) {
        Ok(f327(19u64, 1u64, v5, 2u64, 0u64, 1u64, 2u64))
    } else {
        let a7679 = v4;
        let a7680 = 0u64;
        if nat_eq(a7679, a7680) {
            Ok(f327(19u64, 2u64, 1u64, 2u64, 0u64, 1u64, v5))
        } else {
            Ok(Adt17::C0)
        }
    }
}

pub fn f314(v0: u64, v1: u64, v2: Adt20, v3: Adt20, v4: Adt20, v5: u64) -> R<Adt17> {
    let v6: u64 = f328(match v2.clone() {
        Adt20::C0(h7681, _, _, _, _, _, _, _) => {
            h7681
        }
    })?;
    let v7: u64 = f329(match v2.clone() {
        Adt20::C0(h7682, _, _, _, _, _, _, _) => {
            h7682
        }
    })?;
    let a7683 = v0;
    let a7684 = 2u64;
    if nat_lt(a7683, a7684) {
        Ok(f325(v0))
    } else {
        let a7685 = v0;
        let a7686 = v6;
        if nat_lt(a7685, a7686) {
            f330({
                let a7687 = v0;
                let a7688 = 2u64;
                nat_sub(a7687, a7688)
            }, v1, v2, v3)
        } else {
            let a7689 = v0;
            let a7690 = v7;
            if nat_lt(a7689, a7690) {
                Ok(f331({
                    let a7691 = v0;
                    let a7692 = v6;
                    nat_sub(a7691, a7692)
                }, match v2 {
                    Adt20::C0(h7693, _, _, _, _, _, _, _) => {
                        h7693
                    }
                }, v5))
            } else {
                let a7695 = v0;
                let a7696 = f284(match v2.clone() {
                    Adt20::C0(h7694, _, _, _, _, _, _, _) => {
                        h7694
                    }
                })?;
                if nat_lt(a7695, a7696) {
                    f332({
                        let a7697 = v0;
                        let a7698 = v7;
                        nat_sub(a7697, a7698)
                    }, v1, v2, v3, v4)
                } else {
                    Ok(Adt17::C0)
                }
            }
        }
    }
}

pub fn f315(v0: u64, v1: u64, v2: Adt20, v3: Adt20, v4: u64, v5: bool) -> R<Adt17> {
    let v6: u64 = f300(v5, match v2.clone() {
        Adt20::C0(h7699, _, _, _, _, _, _, _) => {
            h7699
        }
    })?;
    let v7: u64 = {
        let a7701 = v6;
        let a7702 = f301(v5, match v2.clone() {
            Adt20::C0(h7700, _, _, _, _, _, _, _) => {
                h7700
            }
        })?;
        nat_add(a7701, a7702)?
    };
    let v8: u64 = {
        let a7703 = v7;
        let a7704 = f302(v5);
        nat_add(a7703, a7704)?
    };
    let a7705 = v0;
    let a7706 = v6;
    if nat_lt(a7705, a7706) {
        f333(v0, v1, v2, v5)
    } else {
        let a7707 = v0;
        let a7708 = v7;
        if nat_lt(a7707, a7708) {
            f334({
                let a7709 = v0;
                let a7710 = v6;
                nat_sub(a7709, a7710)
            }, v1, v2, v3, v5)
        } else {
            let a7711 = v0;
            let a7712 = v8;
            if nat_lt(a7711, a7712) {
                Ok(f335({
                    let a7713 = v0;
                    let a7714 = v7;
                    nat_sub(a7713, a7714)
                }, match v2 {
                    Adt20::C0(h7715, _, _, _, _, _, _, _) => {
                        h7715
                    }
                }, v4, v5))
            } else {
                let a7719 = v0;
                let a7720 = {
                    let a7717 = v8;
                    let a7718 = match v2.clone() {
                        Adt20::C0(h7716, _, _, _, _, _, _, _) => {
                            h7716
                        }
                    };
                    nat_add(a7717, a7718)?
                };
                if nat_lt(a7719, a7720) {
                    let v9: u64 = f326({
                        let a7726 = {
                            let a7722 = v1;
                            let a7723 = match v2 {
                                Adt20::C0(h7721, _, _, _, _, _, _, _) => {
                                    h7721
                                }
                            };
                            nat_mul(a7722, a7723)?
                        };
                        let a7727 = {
                            let a7724 = v0;
                            let a7725 = v8;
                            nat_sub(a7724, a7725)
                        };
                        nat_add(a7726, a7727)?
                    }, v3)?;
                    Ok(f327(10u64, 3u64, v9, 1u64, 1u64, 3u64, v9))
                } else {
                    Ok(Adt17::C0)
                }
            }
        }
    }
}

pub fn f316(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64) -> R<Adt21> {
    let v6: u64 = f303(v3, v5)?;
    let v7: u64 = f305(v1, v2, v6)?;
    let v8: u64 = {
        let a7730 = {
            let a7728 = f304(v3, v5)?;
            let a7729 = v4;
            nat_mul(a7728, a7729)?
        };
        let a7731 = v7;
        nat_mul(a7730, a7731)?
    };
    let a7732 = v0;
    let a7733 = v8;
    if nat_lt(a7732, a7733) {
        let v9: u64 = {
            let a7734 = v0;
            let a7735 = v7;
            let a7736 = 0u64;
            nat_quot(a7734, a7735, a7736)
        };
        Ok(Adt21::C0({
            let a7740 = {
                let a7737 = v9;
                let a7738 = v4;
                let a7739 = 0u64;
                nat_quot(a7737, a7738, a7739)
            };
            let a7741 = v6;
            nat_mul(a7740, a7741)?
        }, v6, {
            let a7742 = v9;
            let a7743 = v4;
            let a7744 = 0u64;
            nat_rem(a7742, a7743, a7744)
        }, {
            let a7745 = v0;
            let a7746 = v7;
            let a7747 = 0u64;
            nat_rem(a7745, a7746, a7747)
        }))
    } else {
        let v10: u64 = {
            let a7748 = v0;
            let a7749 = v8;
            nat_sub(a7748, a7749)
        };
        let v11: u64 = f305(v1, v2, f306(v3, v5)?)?;
        Ok(Adt21::C0({
            let a7750 = f304(v3, v5)?;
            let a7751 = v6;
            nat_mul(a7750, a7751)?
        }, f306(v3, v5)?, {
            let a7752 = v10;
            let a7753 = v11;
            let a7754 = 0u64;
            nat_quot(a7752, a7753, a7754)
        }, {
            let a7755 = v10;
            let a7756 = v11;
            let a7757 = 0u64;
            nat_rem(a7755, a7756, a7757)
        }))
    }
}

pub fn f317(v0: Adt21, v1: Adt20, v2: Adt20, v3: u64) -> R<Adt17> {
    let v4: u64 = match v0.clone() {
        Adt21::C0(_, _, _, h7758) => {
            h7758
        }
    };
    let v5: u64 = match v1.clone() {
        Adt20::C0(h7759, _, _, _, _, _, _, _) => {
            h7759
        }
    };
    let v6: u64 = {
        let a7762 = match v0.clone() {
            Adt21::C0(_, h7760, _, _) => {
                h7760
            }
        };
        let a7763 = match v1.clone() {
            Adt20::C0(_, h7761, _, _, _, _, _, _) => {
                h7761
            }
        };
        nat_mul(a7762, a7763)?
    };
    let v7: u64 = {
        let a7764 = v6;
        let a7765 = v5;
        nat_mul(a7764, a7765)?
    };
    let v8: u64 = {
        let a7768 = {
            let a7766 = 2u64;
            let a7767 = v5;
            nat_mul(a7766, a7767)?
        };
        let a7769 = 2u64;
        nat_add(a7768, a7769)?
    };
    let v9: u64 = {
        let a7772 = {
            let a7770 = 4u64;
            let a7771 = v5;
            nat_mul(a7770, a7771)?
        };
        let a7773 = 2u64;
        nat_add(a7772, a7773)?
    };
    let v10: u64 = {
        let a7776 = 1u64;
        let a7777 = {
            let a7774 = v6;
            let a7775 = v8;
            nat_mul(a7774, a7775)?
        };
        nat_add(a7776, a7777)?
    };
    let v11: u64 = {
        let a7778 = v10;
        let a7779 = 4u64;
        nat_add(a7778, a7779)?
    };
    let v12: u64 = {
        let a7782 = v11;
        let a7783 = {
            let a7780 = v6;
            let a7781 = v9;
            nat_mul(a7780, a7781)?
        };
        nat_add(a7782, a7783)?
    };
    let v13: u64 = {
        let a7784 = v12;
        let a7785 = 6u64;
        nat_add(a7784, a7785)?
    };
    let a7786 = v4;
    let a7787 = 0u64;
    if nat_eq(a7786, a7787) {
        Ok(f325(0u64))
    } else {
        let a7788 = v4;
        let a7789 = v10;
        if nat_lt(a7788, a7789) {
            let v14: u64 = {
                let a7790 = v4;
                let a7791 = 1u64;
                nat_sub(a7790, a7791)
            };
            f336(false, {
                let a7792 = v14;
                let a7793 = v8;
                let a7794 = 0u64;
                nat_rem(a7792, a7793, a7794)
            }, {
                let a7795 = v14;
                let a7796 = v8;
                let a7797 = 0u64;
                nat_quot(a7795, a7796, a7797)
            }, v0, v1, v2)
        } else {
            let a7798 = v4;
            let a7799 = v11;
            if nat_lt(a7798, a7799) {
                Ok(f337({
                    let a7800 = v4;
                    let a7801 = v10;
                    nat_sub(a7800, a7801)
                }, v7))
            } else {
                let a7802 = v4;
                let a7803 = v12;
                if nat_lt(a7802, a7803) {
                    let v15: u64 = {
                        let a7804 = v4;
                        let a7805 = v11;
                        nat_sub(a7804, a7805)
                    };
                    f336(true, {
                        let a7806 = v15;
                        let a7807 = v9;
                        let a7808 = 0u64;
                        nat_rem(a7806, a7807, a7808)
                    }, {
                        let a7809 = v15;
                        let a7810 = v9;
                        let a7811 = 0u64;
                        nat_quot(a7809, a7810, a7811)
                    }, v0, v1, v2)
                } else {
                    let a7812 = v4;
                    let a7813 = v13;
                    if nat_lt(a7812, a7813) {
                        Ok(f338({
                            let a7814 = v4;
                            let a7815 = v12;
                            nat_sub(a7814, a7815)
                        }, v7, v3))
                    } else {
                        let a7818 = v4;
                        let a7819 = {
                            let a7816 = v13;
                            let a7817 = v7;
                            nat_add(a7816, a7817)?
                        };
                        if nat_lt(a7818, a7819) {
                            let v16: u64 = {
                                let a7820 = v4;
                                let a7821 = v13;
                                nat_sub(a7820, a7821)
                            };
                            let v17: u64 = {
                                let a7822 = v16;
                                let a7823 = v5;
                                let a7824 = 0u64;
                                nat_quot(a7822, a7823, a7824)
                            };
                            let v18: u64 = {
                                let a7830 = {
                                    let a7825 = f339(v17, v0, v1)?;
                                    let a7826 = v5;
                                    nat_mul(a7825, a7826)?
                                };
                                let a7831 = {
                                    let a7827 = v16;
                                    let a7828 = v5;
                                    let a7829 = 0u64;
                                    nat_rem(a7827, a7828, a7829)
                                };
                                nat_add(a7830, a7831)?
                            };
                            let v19: u64 = f326(v18, v2)?;
                            Ok(f327(10u64, 3u64, v19, 1u64, 1u64, 3u64, v19))
                        } else {
                            Ok(Adt17::C0)
                        }
                    }
                }
            }
        }
    }
}

pub fn f318(v0: bool) -> u64 {
    if v0 {
        2u64
    } else {
        1u64
    }
}

pub fn f319(v0: u64) -> u64 {
    let a7832 = v0;
    let a7833 = 4u64;
    let a7834 = 0u64;
    nat_quot(a7832, a7833, a7834)
}

pub fn f320(v0: u64) -> R<u64> {
    let a7837 = {
        let a7835 = 7u64;
        let a7836 = v0;
        nat_mul(a7835, a7836)?
    };
    let a7838 = 4u64;
    nat_add(a7837, a7838)
}

pub fn f321(v0: u64) -> bool {
    let m7839 = f230(v0);
    match m7839 {
        Adt18::C0 => {
            false
        }
        Adt18::C1(_) => {
            true
        }
    }
}

pub fn f322(v0: u64, v1: u64) -> bool {
    let a7840 = v0;
    let a7841 = 1u64;
    if nat_eq(a7840, a7841) {
        true
    } else {
        let v2: u64 = f340(v0);
        let a7842 = 0u64;
        let a7843 = v2;
        if nat_lt(a7842, a7843) {
            let a7847 = {
                let a7844 = v1;
                let a7845 = v2;
                let a7846 = 1u64;
                nat_rem(a7844, a7845, a7846)
            };
            let a7848 = 0u64;
            nat_eq(a7847, a7848)
        } else {
            false
        }
    }
}

pub fn f323(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64, v7: u64, v8: u64, v9: u64, v10: u64, v11: u64, v12: u64, v13: u64, v14: u64, v15: u64) -> R<Adt22> {
    let a7849 = v0;
    let a7850 = 28u64;
    if if if if nat_eq(a7849, a7850) {
        let a7851 = v1;
        let a7852 = 0u64;
        nat_eq(a7851, a7852)
    } else {
        false
    } {
        let a7853 = v2;
        let a7854 = 0u64;
        nat_eq(a7853, a7854)
    } else {
        false
    } {
        let a7855 = v3;
        let a7856 = 0u64;
        nat_eq(a7855, a7856)
    } else {
        false
    } {
        let a7857 = v4;
        let a7858 = v8;
        if if if if if if if if nat_eq(a7857, a7858) {
            let a7859 = v5;
            let a7860 = v12;
            nat_eq(a7859, a7860)
        } else {
            false
        } {
            let a7861 = v9;
            let a7862 = v13;
            nat_eq(a7861, a7862)
        } else {
            false
        } {
            let a7863 = v10;
            let a7864 = v14;
            nat_eq(a7863, a7864)
        } else {
            false
        } {
            let a7865 = v11;
            let a7866 = v15;
            nat_eq(a7865, a7866)
        } else {
            false
        } {
            let a7867 = 0u64;
            let a7868 = v6;
            nat_lt(a7867, a7868)
        } else {
            false
        } {
            let a7869 = 0u64;
            let a7870 = v7;
            nat_lt(a7869, a7870)
        } else {
            false
        } {
            let a7874 = {
                let a7871 = v10;
                let a7872 = v6;
                let a7873 = 1u64;
                nat_rem(a7871, a7872, a7873)
            };
            let a7875 = 0u64;
            if nat_eq(a7874, a7875) {
                let a7879 = {
                    let a7876 = v11;
                    let a7877 = v7;
                    let a7878 = 1u64;
                    nat_rem(a7876, a7877, a7878)
                };
                let a7880 = 0u64;
                nat_eq(a7879, a7880)
            } else {
                false
            }
        } else {
            false
        } {
            Ok(Adt22::C2(v5, v4, v9, v10, v11, {
                let a7881 = v10;
                let a7882 = v6;
                let a7883 = 0u64;
                nat_quot(a7881, a7882, a7883)
            }, {
                let a7884 = v11;
                let a7885 = v7;
                let a7886 = 0u64;
                nat_quot(a7884, a7885, a7886)
            }, {
                let a7887 = v5;
                let a7888 = v4;
                nat_mul(a7887, a7888)?
            }, {
                let a7889 = v4;
                let a7890 = v9;
                nat_mul(a7889, a7890)?
            }, {
                let a7891 = v5;
                let a7892 = v9;
                nat_mul(a7891, a7892)?
            }))
        } else {
            Ok(Adt22::C1)
        }
    } else {
        Ok(Adt22::C0)
    }
}

pub fn f324(v0: u64) -> bool {
    let a7893 = v0;
    let a7894 = 0u64;
    if if nat_eq(a7893, a7894) {
        true
    } else {
        let a7895 = v0;
        let a7896 = 1u64;
        nat_eq(a7895, a7896)
    } {
        true
    } else {
        let a7897 = v0;
        let a7898 = 30u64;
        nat_eq(a7897, a7898)
    }
}

pub fn f325(v0: u64) -> Adt17 {
    f327(0u64, 2u64, 0u64, 2u64, 0u64, 1u64, v0)
}

pub fn f326(v0: u64, v1: Adt20) -> R<u64> {
    let v2: u64 = {
        let a7900 = v0;
        let a7901 = match v1.clone() {
            Adt20::C0(h7899, _, _, _, _, _, _, _) => {
                h7899
            }
        };
        let a7902 = 0u64;
        nat_quot(a7900, a7901, a7902)
    };
    let v3: u64 = {
        let a7904 = v2;
        let a7905 = match v1.clone() {
            Adt20::C0(_, h7903, _, _, _, _, _, _) => {
                h7903
            }
        };
        let a7906 = 0u64;
        nat_quot(a7904, a7905, a7906)
    };
    f154({
        let a7908 = v0;
        let a7909 = match v1.clone() {
            Adt20::C0(h7907, _, _, _, _, _, _, _) => {
                h7907
            }
        };
        let a7910 = 0u64;
        nat_rem(a7908, a7909, a7910)
    }, {
        let a7912 = v2;
        let a7913 = match v1.clone() {
            Adt20::C0(_, h7911, _, _, _, _, _, _) => {
                h7911
            }
        };
        let a7914 = 0u64;
        nat_rem(a7912, a7913, a7914)
    }, {
        let a7916 = v3;
        let a7917 = match v1.clone() {
            Adt20::C0(_, _, h7915, _, _, _, _, _) => {
                h7915
            }
        };
        let a7918 = 0u64;
        nat_rem(a7916, a7917, a7918)
    }, {
        let a7920 = v3;
        let a7921 = match v1.clone() {
            Adt20::C0(_, _, h7919, _, _, _, _, _) => {
                h7919
            }
        };
        let a7922 = 0u64;
        nat_quot(a7920, a7921, a7922)
    }, match v1.clone() {
        Adt20::C0(_, _, _, _, h7923, _, _, _) => {
            h7923
        }
    }, match v1.clone() {
        Adt20::C0(_, _, _, _, _, h7924, _, _) => {
            h7924
        }
    }, match v1.clone() {
        Adt20::C0(_, _, _, _, _, _, h7925, _) => {
            h7925
        }
    }, match v1 {
        Adt20::C0(_, _, _, _, _, _, _, h7926) => {
            h7926
        }
    })
}

pub fn f327(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64) -> Adt17 {
    let a7927 = v0;
    let a7928 = 4u64;
    if if if nat_le(a7927, a7928) {
        true
    } else {
        let a7929 = 9u64;
        let a7930 = v0;
        if nat_le(a7929, a7930) {
            let a7931 = v0;
            let a7932 = 12u64;
            nat_le(a7931, a7932)
        } else {
            false
        }
    } {
        true
    } else {
        let a7933 = v0;
        let a7934 = 16u64;
        if if nat_eq(a7933, a7934) {
            true
        } else {
            let a7935 = v0;
            let a7936 = 19u64;
            nat_eq(a7935, a7936)
        } {
            true
        } else {
            let a7937 = v0;
            let a7938 = 21u64;
            if nat_eq(a7937, a7938) {
                true
            } else {
                let a7939 = v0;
                let a7940 = 22u64;
                nat_eq(a7939, a7940)
            }
        }
    } {
        Adt17::C1({
            let a7941 = v0;
            let a7942 = 4u64;
            if if nat_lt(a7941, a7942) {
                true
            } else {
                let a7943 = 21u64;
                let a7944 = v0;
                nat_le(a7943, a7944)
            } {
                0u64
            } else {
                let a7945 = v0;
                let a7946 = 16u64;
                if if nat_eq(a7945, a7946) {
                    true
                } else {
                    let a7947 = v0;
                    let a7948 = 19u64;
                    nat_eq(a7947, a7948)
                } {
                    2u64
                } else {
                    1u64
                }
            }
        }, {
            let a7949 = v0;
            let a7950 = 4u64;
            if nat_lt(a7949, a7950) {
                v0
            } else {
                let a7951 = v0;
                let a7952 = 4u64;
                if if nat_eq(a7951, a7952) {
                    true
                } else {
                    let a7953 = v0;
                    let a7954 = 16u64;
                    nat_eq(a7953, a7954)
                } {
                    0u64
                } else {
                    let a7955 = v0;
                    let a7956 = 19u64;
                    if nat_eq(a7955, a7956) {
                        3u64
                    } else {
                        let a7957 = 21u64;
                        let a7958 = v0;
                        if nat_le(a7957, a7958) {
                            let a7959 = v0;
                            let a7960 = 17u64;
                            nat_sub(a7959, a7960)
                        } else {
                            let a7961 = v0;
                            let a7962 = 8u64;
                            nat_sub(a7961, a7962)
                        }
                    }
                }
            }
        }, v1, v2, v3, v4, v5, v6)
    } else {
        Adt17::C0
    }
}

pub fn f328(v0: u64) -> R<u64> {
    let a7965 = 2u64;
    let a7966 = {
        let a7963 = 6u64;
        let a7964 = v0;
        nat_mul(a7963, a7964)?
    };
    nat_add(a7965, a7966)
}

pub fn f329(v0: u64) -> R<u64> {
    let a7967 = f328(v0)?;
    let a7968 = 11u64;
    nat_add(a7967, a7968)
}

pub fn f330(v0: u64, v1: u64, v2: Adt20, v3: Adt20) -> R<Adt17> {
    let v4: u64 = {
        let a7969 = v0;
        let a7970 = 6u64;
        let a7971 = 0u64;
        nat_rem(a7969, a7970, a7971)
    };
    let v5: u64 = {
        let a7978 = {
            let a7973 = v1;
            let a7974 = match v2.clone() {
                Adt20::C0(h7972, _, _, _, _, _, _, _) => {
                    h7972
                }
            };
            nat_mul(a7973, a7974)?
        };
        let a7979 = {
            let a7975 = v0;
            let a7976 = 6u64;
            let a7977 = 0u64;
            nat_quot(a7975, a7976, a7977)
        };
        nat_add(a7978, a7979)?
    };
    let v6: u64 = f326(v5, v3)?;
    let v7: u64 = f326(v5, v2)?;
    let a7980 = v4;
    let a7981 = 0u64;
    if nat_eq(a7980, a7981) {
        Ok(f327(10u64, 4u64, v6, 4u64, v6, 1u64, 2u64))
    } else {
        let a7982 = v4;
        let a7983 = 1u64;
        if nat_eq(a7982, a7983) {
            Ok(f327(1u64, 1u64, 2u64, 2u64, 0u64, 1u64, 2u64))
        } else {
            let a7984 = v4;
            let a7985 = 2u64;
            if nat_eq(a7984, a7985) {
                Ok(f327(0u64, 1u64, 0u64, 1u64, 2u64, 1u64, 0u64))
            } else {
                let a7986 = v4;
                let a7987 = 3u64;
                if nat_eq(a7986, a7987) {
                    Ok(f327(10u64, 4u64, v6, 0u64, v7, 1u64, 2u64))
                } else {
                    let a7988 = v4;
                    let a7989 = 4u64;
                    if nat_eq(a7988, a7989) {
                        Ok(f327(1u64, 1u64, 2u64, 2u64, 0u64, 1u64, 2u64))
                    } else {
                        Ok(f327(0u64, 1u64, 1u64, 1u64, 2u64, 1u64, 1u64))
                    }
                }
            }
        }
    }
}

pub fn f331(v0: u64, v1: u64, v2: u64) -> Adt17 {
    let a7990 = v0;
    let a7991 = 0u64;
    if nat_eq(a7990, a7991) {
        f327(2u64, 1u64, 0u64, 2u64, 0u64, 1u64, 3u64)
    } else {
        let a7992 = v0;
        let a7993 = 1u64;
        if nat_eq(a7992, a7993) {
            f327(3u64, 2u64, v1, 2u64, 0u64, 1u64, 2u64)
        } else {
            let a7994 = v0;
            let a7995 = 2u64;
            if nat_eq(a7994, a7995) {
                f327(11u64, 1u64, 3u64, 1u64, 2u64, 1u64, 4u64)
            } else {
                let a7996 = v0;
                let a7997 = 3u64;
                if nat_eq(a7996, a7997) {
                    f327(4u64, 1u64, 4u64, 2u64, v2, 1u64, 4u64)
                } else {
                    let a7998 = v0;
                    let a7999 = 4u64;
                    if nat_eq(a7998, a7999) {
                        f327(12u64, 1u64, 4u64, 2u64, 0u64, 1u64, 4u64)
                    } else {
                        let a8000 = v0;
                        let a8001 = 5u64;
                        if nat_eq(a8000, a8001) {
                            f327(11u64, 2u64, 1065353216u64, 1u64, 4u64, 1u64, 4u64)
                        } else {
                            let a8002 = v0;
                            let a8003 = 6u64;
                            if nat_eq(a8002, a8003) {
                                f327(10u64, 2u64, v2, 1u64, 2u64, 1u64, 5u64)
                            } else {
                                let a8004 = v0;
                                let a8005 = 7u64;
                                if nat_eq(a8004, a8005) {
                                    f327(4u64, 1u64, 3u64, 1u64, 5u64, 1u64, 5u64)
                                } else {
                                    let a8006 = v0;
                                    let a8007 = 8u64;
                                    if nat_eq(a8006, a8007) {
                                        f327(2u64, 1u64, 1u64, 2u64, 0u64, 1u64, 2u64)
                                    } else {
                                        let a8008 = v0;
                                        let a8009 = 9u64;
                                        if nat_eq(a8008, a8009) {
                                            Adt17::C1(3u64, 0u64, 1u64, 2u64, 2u64, 2u64, 1u64, 2u64)
                                        } else {
                                            let a8010 = v0;
                                            let a8011 = 10u64;
                                            if nat_eq(a8010, a8011) {
                                                f327(11u64, 1u64, 2u64, 1u64, 5u64, 1u64, 5u64)
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

pub fn f332(v0: u64, v1: u64, v2: Adt20, v3: Adt20, v4: Adt20) -> R<Adt17> {
    let v5: u64 = {
        let a8012 = v0;
        let a8013 = 3u64;
        let a8014 = 0u64;
        nat_rem(a8012, a8013, a8014)
    };
    let v6: u64 = {
        let a8021 = {
            let a8016 = v1;
            let a8017 = match v2.clone() {
                Adt20::C0(h8015, _, _, _, _, _, _, _) => {
                    h8015
                }
            };
            nat_mul(a8016, a8017)?
        };
        let a8022 = {
            let a8018 = v0;
            let a8019 = 3u64;
            let a8020 = 0u64;
            nat_quot(a8018, a8019, a8020)
        };
        nat_add(a8021, a8022)?
    };
    let v7: u64 = f326(v6, v3)?;
    let v8: u64 = f326(v6, v2)?;
    let v9: u64 = f326(v6, v4)?;
    let a8023 = v5;
    let a8024 = 0u64;
    if nat_eq(a8023, a8024) {
        Ok(f327(10u64, 4u64, v7, 1u64, 5u64, 1u64, 2u64))
    } else {
        let a8025 = v5;
        let a8026 = 1u64;
        if nat_eq(a8025, a8026) {
            Ok(f327(4u64, 0u64, v8, 1u64, 2u64, 1u64, 2u64))
        } else {
            Ok(f327(10u64, 1u64, 2u64, 1u64, 4u64, 3u64, v9))
        }
    }
}

pub fn f333(v0: u64, v1: u64, v2: Adt20, v3: bool) -> R<Adt17> {
    let v4: u64 = match v2.clone() {
        Adt20::C0(h8027, _, _, _, _, _, _, _) => {
            h8027
        }
    };
    let v5: u64 = {
        let a8030 = 1u64;
        let a8031 = {
            let a8028 = v4;
            let a8029 = f318(v3);
            nat_mul(a8028, a8029)?
        };
        nat_add(a8030, a8031)?
    };
    let a8032 = v0;
    let a8033 = 0u64;
    if nat_eq(a8032, a8033) {
        Ok(f325(0u64))
    } else {
        let a8034 = v0;
        let a8035 = v5;
        if nat_lt(a8034, a8035) {
            let v6: u64 = {
                let a8036 = v0;
                let a8037 = 1u64;
                nat_sub(a8036, a8037)
            };
            let v7: u64 = f326({
                let a8043 = {
                    let a8038 = v1;
                    let a8039 = v4;
                    nat_mul(a8038, a8039)?
                };
                let a8044 = {
                    let a8040 = v6;
                    let a8041 = f318(v3);
                    let a8042 = 0u64;
                    nat_quot(a8040, a8041, a8042)
                };
                nat_add(a8043, a8044)?
            }, v2)?;
            if v3 {
                let a8048 = {
                    let a8045 = v6;
                    let a8046 = 2u64;
                    let a8047 = 0u64;
                    nat_rem(a8045, a8046, a8047)
                };
                let a8049 = 0u64;
                if nat_eq(a8048, a8049) {
                    Ok(f327(1u64, 0u64, v7, 2u64, 0u64, 1u64, 1u64))
                } else {
                    Ok(f327(0u64, 1u64, 0u64, 1u64, 1u64, 1u64, 0u64))
                }
            } else {
                Ok(f327(4u64, 1u64, 0u64, 0u64, v7, 1u64, 0u64))
            }
        } else {
            let v8: u64 = {
                let a8050 = v0;
                let a8051 = v5;
                nat_sub(a8050, a8051)
            };
            let a8052 = v8;
            let a8053 = 0u64;
            if nat_eq(a8052, a8053) {
                Ok(f327(3u64, 2u64, v4, 2u64, 0u64, 1u64, 1u64))
            } else {
                if if v3 {
                    let a8054 = v8;
                    let a8055 = 1u64;
                    nat_eq(a8054, a8055)
                } else {
                    false
                } {
                    Ok(f327(2u64, 1u64, 0u64, 2u64, 0u64, 1u64, 0u64))
                } else {
                    let a8056 = v8;
                    let a8057 = if v3 {
                        2u64
                    } else {
                        1u64
                    };
                    if nat_eq(a8056, a8057) {
                        Ok(f327(11u64, 1u64, 0u64, 1u64, 1u64, 1u64, 3u64))
                    } else {
                        let a8058 = v8;
                        let a8059 = if v3 {
                            3u64
                        } else {
                            2u64
                        };
                        if nat_eq(a8058, a8059) {
                            Ok(f325(0u64))
                        } else {
                            Ok(Adt17::C0)
                        }
                    }
                }
            }
        }
    }
}

pub fn f334(v0: u64, v1: u64, v2: Adt20, v3: Adt20, v4: bool) -> R<Adt17> {
    let v5: u64 = if v4 {
        let a8061 = 13u64;
        let a8062 = f319(match v2.clone() {
            Adt20::C0(h8060, _, _, _, _, _, _, _) => {
                h8060
            }
        });
        nat_mul(a8061, a8062)?
    } else {
        0u64
    };
    let a8063 = v0;
    let a8064 = v5;
    if nat_lt(a8063, a8064) {
        f341({
            let a8065 = v0;
            let a8066 = 13u64;
            let a8067 = 0u64;
            nat_rem(a8065, a8066, a8067)
        }, v1, {
            let a8068 = v0;
            let a8069 = 13u64;
            let a8070 = 0u64;
            nat_quot(a8068, a8069, a8070)
        }, v2, v3)
    } else {
        let v6: u64 = {
            let a8071 = v0;
            let a8072 = v5;
            nat_sub(a8071, a8072)
        };
        let v7: u64 = if v4 {
            4u64
        } else {
            3u64
        };
        f342({
            let a8073 = v6;
            let a8074 = v7;
            let a8075 = 0u64;
            nat_rem(a8073, a8074, a8075)
        }, {
            let a8085 = {
                let a8080 = {
                    let a8077 = v1;
                    let a8078 = match v2.clone() {
                        Adt20::C0(h8076, _, _, _, _, _, _, _) => {
                            h8076
                        }
                    };
                    nat_mul(a8077, a8078)?
                };
                let a8081 = if v4 {
                    f343(match v2.clone() {
                        Adt20::C0(h8079, _, _, _, _, _, _, _) => {
                            h8079
                        }
                    })?
                } else {
                    0u64
                };
                nat_add(a8080, a8081)?
            };
            let a8086 = {
                let a8082 = v6;
                let a8083 = v7;
                let a8084 = 0u64;
                nat_quot(a8082, a8083, a8084)
            };
            nat_add(a8085, a8086)?
        }, v2, v3, v4)
    }
}

pub fn f335(v0: u64, v1: u64, v2: u64, v3: bool) -> Adt17 {
    let a8087 = v0;
    let a8088 = 0u64;
    if nat_eq(a8087, a8088) {
        f327(if v3 {
            22u64
        } else {
            3u64
        }, 2u64, v1, 2u64, 0u64, 1u64, 1u64)
    } else {
        let a8089 = v0;
        let a8090 = 1u64;
        if nat_eq(a8089, a8090) {
            f327(if v3 {
                21u64
            } else {
                11u64
            }, 1u64, 0u64, 1u64, 1u64, 1u64, 1u64)
        } else {
            if if v3 {
                let a8091 = v0;
                let a8092 = 2u64;
                nat_eq(a8091, a8092)
            } else {
                false
            } {
                f327(2u64, 1u64, 1u64, 2u64, 0u64, 1u64, 1u64)
            } else {
                let a8093 = v0;
                let a8094 = if v3 {
                    3u64
                } else {
                    2u64
                };
                if nat_eq(a8093, a8094) {
                    f327(4u64, 1u64, 1u64, 2u64, v2, 1u64, 1u64)
                } else {
                    let a8095 = v0;
                    let a8096 = if v3 {
                        4u64
                    } else {
                        3u64
                    };
                    if nat_eq(a8095, a8096) {
                        f327(12u64, 1u64, 1u64, 2u64, 0u64, 1u64, 1u64)
                    } else {
                        let a8097 = v0;
                        let a8098 = if v3 {
                            5u64
                        } else {
                            4u64
                        };
                        if nat_eq(a8097, a8098) {
                            f327(11u64, 2u64, 1065353216u64, 1u64, 1u64, 1u64, 1u64)
                        } else {
                            Adt17::C0
                        }
                    }
                }
            }
        }
    }
}

pub fn f336(v0: bool, v1: u64, v2: u64, v3: Adt21, v4: Adt20, v5: Adt20) -> R<Adt17> {
    let v6: u64 = if v0 {
        4u64
    } else {
        2u64
    };
    let v7: u64 = {
        let a8102 = 1u64;
        let a8103 = {
            let a8100 = match v4.clone() {
                Adt20::C0(h8099, _, _, _, _, _, _, _) => {
                    h8099
                }
            };
            let a8101 = v6;
            nat_mul(a8100, a8101)?
        };
        nat_add(a8102, a8103)?
    };
    let a8104 = v1;
    let a8105 = 0u64;
    if nat_eq(a8104, a8105) {
        Ok(f325(1u64))
    } else {
        let a8106 = v1;
        let a8107 = v7;
        if nat_lt(a8106, a8107) {
            let v8: u64 = {
                let a8108 = v1;
                let a8109 = 1u64;
                nat_sub(a8108, a8109)
            };
            let v9: u64 = {
                let a8116 = {
                    let a8111 = f339(v2, v3, v4.clone())?;
                    let a8112 = match v4.clone() {
                        Adt20::C0(h8110, _, _, _, _, _, _, _) => {
                            h8110
                        }
                    };
                    nat_mul(a8111, a8112)?
                };
                let a8117 = {
                    let a8113 = v8;
                    let a8114 = v6;
                    let a8115 = 0u64;
                    nat_quot(a8113, a8114, a8115)
                };
                nat_add(a8116, a8117)?
            };
            let v10: u64 = f326(v9, v4)?;
            let v11: u64 = f326(v9, v5)?;
            let v12: u64 = {
                let a8118 = v8;
                let a8119 = v6;
                let a8120 = 0u64;
                nat_rem(a8118, a8119, a8120)
            };
            if v0 {
                let a8121 = v12;
                let a8122 = 0u64;
                if nat_eq(a8121, a8122) {
                    Ok(f327(9u64, 0u64, v10, 1u64, 3u64, 3u64, v11))
                } else {
                    let a8123 = v12;
                    let a8124 = 1u64;
                    if nat_eq(a8123, a8124) {
                        Ok(f327(10u64, 3u64, v11, 3u64, v11, 1u64, 2u64))
                    } else {
                        let a8125 = v12;
                        let a8126 = 2u64;
                        if nat_eq(a8125, a8126) {
                            Ok(f327(1u64, 1u64, 2u64, 2u64, 0u64, 1u64, 2u64))
                        } else {
                            Ok(f327(0u64, 1u64, 1u64, 1u64, 2u64, 1u64, 1u64))
                        }
                    }
                }
            } else {
                let a8127 = v12;
                let a8128 = 0u64;
                if nat_eq(a8127, a8128) {
                    Ok(f327(1u64, 0u64, v10, 2u64, 0u64, 1u64, 2u64))
                } else {
                    Ok(f327(0u64, 1u64, 1u64, 1u64, 2u64, 1u64, 1u64))
                }
            }
        } else {
            let a8129 = v1;
            let a8130 = v7;
            if nat_eq(a8129, a8130) {
                Ok(f327(0u64, 1u64, 0u64, 1u64, 1u64, 1u64, 0u64))
            } else {
                Ok(Adt17::C0)
            }
        }
    }
}

pub fn f337(v0: u64, v1: u64) -> Adt17 {
    let a8131 = v0;
    let a8132 = 0u64;
    if nat_eq(a8131, a8132) {
        f327(22u64, 2u64, v1, 2u64, 0u64, 1u64, 2u64)
    } else {
        let a8133 = v0;
        let a8134 = 1u64;
        if nat_eq(a8133, a8134) {
            f327(21u64, 1u64, 0u64, 1u64, 2u64, 1u64, 3u64)
        } else {
            let a8135 = v0;
            let a8136 = 2u64;
            if nat_eq(a8135, a8136) {
                f327(2u64, 1u64, 3u64, 2u64, 0u64, 1u64, 3u64)
            } else {
                let a8137 = v0;
                let a8138 = 3u64;
                if nat_eq(a8137, a8138) {
                    f325(0u64)
                } else {
                    Adt17::C0
                }
            }
        }
    }
}

pub fn f338(v0: u64, v1: u64, v2: u64) -> Adt17 {
    let a8139 = v0;
    let a8140 = 0u64;
    if nat_eq(a8139, a8140) {
        f327(22u64, 2u64, v1, 2u64, 0u64, 1u64, 2u64)
    } else {
        let a8141 = v0;
        let a8142 = 1u64;
        if nat_eq(a8141, a8142) {
            f327(21u64, 1u64, 0u64, 1u64, 2u64, 1u64, 2u64)
        } else {
            let a8143 = v0;
            let a8144 = 2u64;
            if nat_eq(a8143, a8144) {
                f327(2u64, 1u64, 2u64, 2u64, 0u64, 1u64, 2u64)
            } else {
                let a8145 = v0;
                let a8146 = 3u64;
                if nat_eq(a8145, a8146) {
                    f327(4u64, 1u64, 2u64, 2u64, v2, 1u64, 1u64)
                } else {
                    let a8147 = v0;
                    let a8148 = 4u64;
                    if nat_eq(a8147, a8148) {
                        f327(12u64, 1u64, 1u64, 2u64, 0u64, 1u64, 1u64)
                    } else {
                        let a8149 = v0;
                        let a8150 = 5u64;
                        if nat_eq(a8149, a8150) {
                            f327(11u64, 2u64, 1065353216u64, 1u64, 1u64, 1u64, 1u64)
                        } else {
                            Adt17::C0
                        }
                    }
                }
            }
        }
    }
}

pub fn f339(v0: u64, v1: Adt21, v2: Adt20) -> R<u64> {
    let a8161 = {
        let a8159 = {
            let a8156 = {
                let a8153 = match v1.clone() {
                    Adt21::C0(_, _, h8151, _) => {
                        h8151
                    }
                };
                let a8154 = match v2.clone() {
                    Adt20::C0(_, _, h8152, _, _, _, _, _) => {
                        h8152
                    }
                };
                nat_mul(a8153, a8154)?
            };
            let a8157 = match v1 {
                Adt21::C0(h8155, _, _, _) => {
                    h8155
                }
            };
            nat_add(a8156, a8157)?
        };
        let a8160 = match v2 {
            Adt20::C0(_, h8158, _, _, _, _, _, _) => {
                h8158
            }
        };
        nat_mul(a8159, a8160)?
    };
    let a8162 = v0;
    nat_add(a8161, a8162)
}

pub fn f340(v0: u64) -> u64 {
    if f238(v0) {
        let m8163 = f230(v0);
        match m8163 {
            Adt18::C0 => {
                0u64
            }
            Adt18::C1(v1) => {
                let a8165 = 1u64;
                let a8166 = match v1.clone() {
                    Adt19::C0(_, h8164, _) => {
                        h8164
                    }
                };
                if if nat_lt(a8165, a8166) {
                    true
                } else {
                    let a8167 = v0;
                    let a8168 = 43u64;
                    nat_eq(a8167, a8168)
                } {
                    match v1 {
                        Adt19::C0(_, h8169, _) => {
                            h8169
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

pub fn f341(v0: u64, v1: u64, v2: u64, v3: Adt20, v4: Adt20) -> R<Adt17> {
    let v5: u64 = {
        let a8177 = {
            let a8175 = {
                let a8171 = v1;
                let a8172 = match v3.clone() {
                    Adt20::C0(h8170, _, _, _, _, _, _, _) => {
                        h8170
                    }
                };
                nat_mul(a8171, a8172)?
            };
            let a8176 = {
                let a8173 = v2;
                let a8174 = 4u64;
                nat_mul(a8173, a8174)?
            };
            nat_add(a8175, a8176)?
        };
        let a8178 = f344(v0);
        nat_add(a8177, a8178)?
    };
    let v6: u64 = f326(v5, v3)?;
    let v7: u64 = f326(v5, v4)?;
    let a8179 = v0;
    let a8180 = 0u64;
    if if if if nat_eq(a8179, a8180) {
        true
    } else {
        let a8181 = v0;
        let a8182 = 2u64;
        nat_eq(a8181, a8182)
    } {
        true
    } else {
        let a8183 = v0;
        let a8184 = 5u64;
        nat_eq(a8183, a8184)
    } {
        true
    } else {
        let a8185 = v0;
        let a8186 = 7u64;
        nat_eq(a8185, a8186)
    } {
        Ok(f327(9u64, 0u64, v6, 1u64, 3u64, 3u64, v7))
    } else {
        let a8187 = v0;
        let a8188 = 1u64;
        if if if if nat_eq(a8187, a8188) {
            true
        } else {
            let a8189 = v0;
            let a8190 = 3u64;
            nat_eq(a8189, a8190)
        } {
            true
        } else {
            let a8191 = v0;
            let a8192 = 6u64;
            nat_eq(a8191, a8192)
        } {
            true
        } else {
            let a8193 = v0;
            let a8194 = 8u64;
            nat_eq(a8193, a8194)
        } {
            Ok(f327(10u64, 3u64, v7, 3u64, v7, 1u64, {
                let a8195 = v0;
                let a8196 = 1u64;
                if nat_eq(a8195, a8196) {
                    4u64
                } else {
                    let a8197 = v0;
                    let a8198 = 6u64;
                    if nat_eq(a8197, a8198) {
                        5u64
                    } else {
                        1u64
                    }
                }
            }))
        } else {
            let a8199 = v0;
            let a8200 = 4u64;
            if nat_eq(a8199, a8200) {
                Ok(f327(4u64, 1u64, 4u64, 1u64, 1u64, 1u64, 4u64))
            } else {
                let a8201 = v0;
                let a8202 = 9u64;
                if nat_eq(a8201, a8202) {
                    Ok(f327(4u64, 1u64, 5u64, 1u64, 1u64, 1u64, 5u64))
                } else {
                    let a8203 = v0;
                    let a8204 = 10u64;
                    if nat_eq(a8203, a8204) {
                        Ok(f327(4u64, 1u64, 4u64, 1u64, 5u64, 1u64, 1u64))
                    } else {
                        let a8205 = v0;
                        let a8206 = 11u64;
                        if nat_eq(a8205, a8206) {
                            Ok(f327(1u64, 1u64, 1u64, 2u64, 0u64, 1u64, 1u64))
                        } else {
                            let a8207 = v0;
                            let a8208 = 12u64;
                            if nat_eq(a8207, a8208) {
                                Ok(f327(0u64, 1u64, 0u64, 1u64, 1u64, 1u64, 0u64))
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

pub fn f342(v0: u64, v1: u64, v2: Adt20, v3: Adt20, v4: bool) -> R<Adt17> {
    let v5: u64 = f326(v1, v2)?;
    let v6: u64 = f326(v1, v3)?;
    let a8209 = v0;
    let a8210 = 0u64;
    if nat_eq(a8209, a8210) {
        Ok(f327(9u64, 0u64, v5, 1u64, 3u64, 3u64, v6))
    } else {
        let a8211 = v0;
        let a8212 = 1u64;
        if nat_eq(a8211, a8212) {
            Ok(f327(10u64, 3u64, v6, 3u64, v6, 1u64, 1u64))
        } else {
            if v4 {
                let a8213 = v0;
                let a8214 = 2u64;
                if nat_eq(a8213, a8214) {
                    Ok(f327(1u64, 1u64, 1u64, 2u64, 0u64, 1u64, 1u64))
                } else {
                    let a8215 = v0;
                    let a8216 = 3u64;
                    if nat_eq(a8215, a8216) {
                        Ok(f327(0u64, 1u64, 0u64, 1u64, 1u64, 1u64, 0u64))
                    } else {
                        Ok(Adt17::C0)
                    }
                }
            } else {
                let a8217 = v0;
                let a8218 = 2u64;
                if nat_eq(a8217, a8218) {
                    Ok(f327(4u64, 1u64, 0u64, 1u64, 1u64, 1u64, 0u64))
                } else {
                    Ok(Adt17::C0)
                }
            }
        }
    }
}

pub fn f343(v0: u64) -> R<u64> {
    let a8219 = 4u64;
    let a8220 = f319(v0);
    nat_mul(a8219, a8220)
}

pub fn f344(v0: u64) -> u64 {
    let a8221 = v0;
    let a8222 = 5u64;
    if nat_lt(a8221, a8222) {
        let a8223 = v0;
        let a8224 = 2u64;
        if nat_lt(a8223, a8224) {
            0u64
        } else {
            1u64
        }
    } else {
        let a8225 = v0;
        let a8226 = 7u64;
        if nat_lt(a8225, a8226) {
            2u64
        } else {
            3u64
        }
    }
}
