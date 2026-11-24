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
    C48(bool, u64, u64),
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
            let a3 = 60u64;
            if nat_eq(a2, a3) {
                Ok(Adt9::C0({
                    let m4 = v3;
                    match m4 {
                        None => {
                            Adt10::C1
                        }
                        Some(v14) => {
                            if if f1(v4) {
                                true
                            } else {
                                if f1(v5) {
                                    true
                                } else {
                                    if f1(v6) {
                                        true
                                    } else {
                                        if f1(v7) {
                                            true
                                        } else {
                                            if f1(v8) {
                                                true
                                            } else {
                                                if f1(v9) {
                                                    true
                                                } else {
                                                    v10
                                                }
                                            }
                                        }
                                    }
                                }
                            } {
                                Adt10::C1
                            } else {
                                f2(v2, v14, v13)?
                            }
                        }
                    }
                }))
            } else {
                let a5 = v1;
                let a6 = 79u64;
                if if nat_eq(a5, a6) {
                    true
                } else {
                    let a7 = v1;
                    let a8 = 81u64;
                    nat_eq(a7, a8)
                } {
                    Ok(Adt9::C0(f3({
                        let a9 = v1;
                        let a10 = 81u64;
                        nat_eq(a9, a10)
                    }, v3, v4, v5, v6, v7, v8, v9, v10, v13)?))
                } else {
                    let v15: bool = if if f1(v8) {
                        true
                    } else {
                        f1(v9)
                    } {
                        true
                    } else {
                        v10
                    };
                    let a11 = v1;
                    let a12 = 80u64;
                    if nat_eq(a11, a12) {
                        Ok(Adt9::C0(f4(match v2 {
                            Adt1::C0(h13, _, _, _, _, _, _, _, _, _, _, _) => {
                                h13
                            }
                        }, v3, v4, v5, v6, v7, v15, v13)?))
                    } else {
                        let a14 = v1;
                        let a15 = 82u64;
                        if if if if if nat_eq(a14, a15) {
                            true
                        } else {
                            let a16 = v1;
                            let a17 = 50u64;
                            nat_eq(a16, a17)
                        } {
                            true
                        } else {
                            let a18 = v1;
                            let a19 = 59u64;
                            nat_eq(a18, a19)
                        } {
                            true
                        } else {
                            let a20 = v1;
                            let a21 = 65u64;
                            nat_eq(a20, a21)
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
                                v15
                            }
                        } else {
                            false
                        } {
                            Ok(Adt9::C0(Adt10::C1))
                        } else {
                            let a22 = v1;
                            let a23 = 94u64;
                            if if nat_eq(a22, a23) {
                                true
                            } else {
                                let a24 = v1;
                                let a25 = 95u64;
                                nat_eq(a24, a25)
                            } {
                                Ok(Adt9::C0(f5({
                                    let a26 = v1;
                                    let a27 = 95u64;
                                    nat_eq(a26, a27)
                                }, v3, v4, v5, {
                                    let m28 = v6;
                                    match m28 {
                                        None => {
                                            let m29 = v7;
                                            match m29 {
                                                None => {
                                                    v15
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
                                let a30 = v1;
                                let a31 = 96u64;
                                if if nat_eq(a30, a31) {
                                    true
                                } else {
                                    let a32 = v1;
                                    let a33 = 97u64;
                                    nat_eq(a32, a33)
                                } {
                                    Ok(Adt9::C0(f6({
                                        let a34 = v1;
                                        let a35 = 96u64;
                                        nat_eq(a34, a35)
                                    }, v11, v12, v3, v4, v5, v6, v7, v15, v13)?))
                                } else {
                                    let v19: bool = {
                                        let m36 = v7;
                                        match m36 {
                                            None => {
                                                v15
                                            }
                                            Some(_) => {
                                                true
                                            }
                                        }
                                    };
                                    let a37 = v1;
                                    let a38 = 85u64;
                                    if nat_eq(a37, a38) {
                                        Ok(Adt9::C0(f7(v2, v3, v4, v5, {
                                            let m39 = v6;
                                            match m39 {
                                                None => {
                                                    v19
                                                }
                                                Some(_) => {
                                                    true
                                                }
                                            }
                                        }, v13)?))
                                    } else {
                                        let a40 = v1;
                                        let a41 = 87u64;
                                        if nat_eq(a40, a41) {
                                            Ok(Adt9::C0(f8(v3, v4, v5, v6, v19, v13)?))
                                        } else {
                                            let a42 = v1;
                                            let a43 = 64u64;
                                            if nat_eq(a42, a43) {
                                                Ok(Adt9::C0(f9(v2, v13)?))
                                            } else {
                                                let m44 = v3;
                                                match m44 {
                                                    None => {
                                                        Ok(Adt9::C0(Adt10::C1))
                                                    }
                                                    Some(v21) => {
                                                        let a45 = v1;
                                                        let a46 = 48u64;
                                                        if if nat_eq(a45, a46) {
                                                            true
                                                        } else {
                                                            let a47 = v1;
                                                            let a48 = 56u64;
                                                            nat_eq(a47, a48)
                                                        } {
                                                            Ok(Adt9::C0(f10({
                                                                let a49 = v1;
                                                                let a50 = 56u64;
                                                                nat_eq(a49, a50)
                                                            }, v2, v21, v4, v13)?))
                                                        } else {
                                                            let a51 = v1;
                                                            let a52 = 53u64;
                                                            if if nat_eq(a51, a52) {
                                                                true
                                                            } else {
                                                                let a53 = v1;
                                                                let a54 = 54u64;
                                                                nat_eq(a53, a54)
                                                            } {
                                                                Ok(Adt9::C0(f11({
                                                                    let a55 = v1;
                                                                    let a56 = 54u64;
                                                                    nat_eq(a55, a56)
                                                                }, v2, v21, v4, v13)?))
                                                            } else {
                                                                let a57 = v1;
                                                                let a58 = 78u64;
                                                                if nat_eq(a57, a58) {
                                                                    Ok(Adt9::C0(f12(v2, v21, v4, v5, v13)?))
                                                                } else {
                                                                    let a59 = v1;
                                                                    let a60 = 23u64;
                                                                    if if if nat_eq(a59, a60) {
                                                                        true
                                                                    } else {
                                                                        let a61 = v1;
                                                                        let a62 = 26u64;
                                                                        nat_eq(a61, a62)
                                                                    } {
                                                                        true
                                                                    } else {
                                                                        let a63 = v1;
                                                                        let a64 = 25u64;
                                                                        nat_eq(a63, a64)
                                                                    } {
                                                                        Ok(Adt9::C0(f13(v1, v2, v21, v4, v13)?))
                                                                    } else {
                                                                        let a65 = v1;
                                                                        let a66 = 24u64;
                                                                        if nat_eq(a65, a66) {
                                                                            Ok(Adt9::C0(f14(v2, v21, v13)?))
                                                                        } else {
                                                                            let a67 = v1;
                                                                            let a68 = 27u64;
                                                                            if nat_eq(a67, a68) {
                                                                                Ok(Adt9::C0(f15(v2, v21, v13)?))
                                                                            } else {
                                                                                let a69 = v1;
                                                                                let a70 = 3u64;
                                                                                if nat_eq(a69, a70) {
                                                                                    Ok(Adt9::C0(f16(v21, v4, v5, v13)?))
                                                                                } else {
                                                                                    f17(v1, v2, v21, v4, v13)
                                                                                }
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        Adt0::C1(v27, v28, v29, v30, v31) => {
            f20(v27, v28, v29, v30, v31)
        }
        Adt0::C2(v42) => {
            f22(v42)
        }
        Adt0::C3(v43, v44, v45, v46, v47, v48, v49, v50, v51, v52, v53, v54, v55) => {
            Ok(f23(v43, v44, v45, v46, v47, v48, v49, v50, v51, v52, v53, v54, v55))
        }
        Adt0::C4(v22, v23, v24, v25) => {
            let a72 = 0u64;
            let a73 = match v22.clone() {
                Adt8::C0(_, _, _, _, _, _, _, h71, _, _, _, _, _, _, _) => {
                    h71
                }
            };
            if nat_lt(a72, a73) {
                f18(v22, v23, v24, v25)
            } else {
                f19(v22, v23, v24, v25)
            }
        }
        Adt0::C5(v26) => {
            Ok(Adt9::C9({
                let a74 = 3240099840u64;
                let a75 = v26;
                if if nat_le(a74, a75) {
                    let a76 = v26;
                    let a77 = 4286578688u64;
                    nat_le(a76, a77)
                } else {
                    false
                } {
                    Some(0u64)
                } else {
                    let a78 = 1092616192u64;
                    let a79 = v26;
                    if if nat_le(a78, a79) {
                        let a80 = v26;
                        let a81 = 2139095040u64;
                        nat_le(a80, a81)
                    } else {
                        false
                    } {
                        Some(v26)
                    } else {
                        None::<u64>
                    }
                }
            }))
        }
        Adt0::C6(v32, v33, v34, v35, v36, v37, v38, v39, v40, v41) => {
            f21(v32, v33, v34, v35, v36, true, v37, v38, v39, v40, v41)
        }
    }
}

pub fn f1(v0: Option<Adt2>) -> bool {
    let m82 = v0;
    match m82 {
        None => {
            false
        }
        Some(_) => {
            true
        }
    }
}

pub fn f2(v0: Adt1, v1: Adt2, v2: Adt2) -> R<Adt10> {
    let v3: u64 = match v0 {
        Adt1::C0(h83, _, _, _, _, _, _, _, _, _, _, _) => {
            h83
        }
    };
    let v4: u64 = {
        let a92 = {
            let a89 = {
                let a86 = match v2.clone() {
                    Adt2::C0(_, h84, _, _, _, _, _, _, _, _, _) => {
                        h84
                    }
                };
                let a87 = match v2.clone() {
                    Adt2::C0(_, _, h85, _, _, _, _, _, _, _, _) => {
                        h85
                    }
                };
                nat_add(a86, a87)?
            };
            let a90 = match v2.clone() {
                Adt2::C0(_, _, _, h88, _, _, _, _, _, _, _) => {
                    h88
                }
            };
            nat_add(a89, a90)?
        };
        let a93 = match v2.clone() {
            Adt2::C0(_, _, _, _, h91, _, _, _, _, _, _) => {
                h91
            }
        };
        nat_add(a92, a93)?
    };
    let v5: u64 = {
        let a106 = {
            let a102 = {
                let a98 = f24(match v1.clone() {
                    Adt2::C0(_, h94, _, _, _, _, _, _, _, _, _) => {
                        h94
                    }
                }, match v2.clone() {
                    Adt2::C0(_, h95, _, _, _, _, _, _, _, _, _) => {
                        h95
                    }
                })?;
                let a99 = f24(match v1.clone() {
                    Adt2::C0(_, _, h96, _, _, _, _, _, _, _, _) => {
                        h96
                    }
                }, match v2.clone() {
                    Adt2::C0(_, _, h97, _, _, _, _, _, _, _, _) => {
                        h97
                    }
                })?;
                nat_add(a98, a99)?
            };
            let a103 = f24(match v1.clone() {
                Adt2::C0(_, _, _, h100, _, _, _, _, _, _, _) => {
                    h100
                }
            }, match v2.clone() {
                Adt2::C0(_, _, _, h101, _, _, _, _, _, _, _) => {
                    h101
                }
            })?;
            nat_add(a102, a103)?
        };
        let a107 = f24(match v1.clone() {
            Adt2::C0(_, _, _, _, h104, _, _, _, _, _, _) => {
                h104
            }
        }, match v2.clone() {
            Adt2::C0(_, _, _, _, h105, _, _, _, _, _, _) => {
                h105
            }
        })?;
        nat_add(a106, a107)?
    };
    let a109 = match v1.clone() {
        Adt2::C0(h108, _, _, _, _, _, _, _, _, _, _) => {
            h108
        }
    };
    let a110 = 0u64;
    if if if if if if if if nat_eq(a109, a110) {
        let a112 = match v2.clone() {
            Adt2::C0(h111, _, _, _, _, _, _, _, _, _, _) => {
                h111
            }
        };
        let a113 = 0u64;
        nat_eq(a112, a113)
    } else {
        false
    } {
        let a114 = 0u64;
        let a115 = f25(v1.clone(), 1u64)?;
        nat_lt(a114, a115)
    } else {
        false
    } {
        let a116 = 0u64;
        let a117 = f25(v2.clone(), 1u64)?;
        nat_lt(a116, a117)
    } else {
        false
    } {
        f26(v1.clone(), f25(v1.clone(), 1u64)?)?
    } else {
        false
    } {
        f26(v2.clone(), f25(v2.clone(), 1u64)?)?
    } else {
        false
    } {
        f27(v1.clone(), v2.clone())?
    } else {
        false
    } {
        let a121 = {
            let a118 = v3;
            let a119 = 256u64;
            let a120 = 0u64;
            nat_rem(a118, a119, a120)
        };
        let a122 = 0u64;
        if nat_eq(a121, a122) {
            let a123 = v3;
            let a124 = 512u64;
            nat_lt(a123, a124)
        } else {
            false
        }
    } else {
        false
    } {
        Ok(Adt10::C4({
            let a136 = v5;
            let a137 = {
                let a134 = {
                    let a126 = f25(v2.clone(), 1u64)?;
                    let a127 = match v2.clone() {
                        Adt2::C0(_, h125, _, _, _, _, _, _, _, _, _) => {
                            h125
                        }
                    };
                    let a128 = 0u64;
                    nat_quot(a126, a127, a128)
                };
                let a135 = {
                    let a132 = 2u64;
                    let a133 = {
                        let a130 = 2u64;
                        let a131 = match v2.clone() {
                            Adt2::C0(_, h129, _, _, _, _, _, _, _, _, _) => {
                                h129
                            }
                        };
                        nat_mul(a130, a131)?
                    };
                    nat_add(a132, a133)?
                };
                nat_mul(a134, a135)?
            };
            nat_add(a136, a137)?
        }, 4u64, f28(v1, 1u64), f28(v2, 1u64), Adt4::C48({
            let a144 = {
                let a141 = {
                    let a138 = v3;
                    let a139 = 256u64;
                    let a140 = 0u64;
                    nat_quot(a138, a139, a140)
                };
                let a142 = 2u64;
                let a143 = 0u64;
                nat_rem(a141, a142, a143)
            };
            let a145 = 1u64;
            nat_eq(a144, a145)
        }, v4, v5)))
    } else {
        Ok(Adt10::C1)
    }
}

pub fn f3(v0: bool, v1: Option<Adt2>, v2: Option<Adt2>, v3: Option<Adt2>, v4: Option<Adt2>, v5: Option<Adt2>, v6: Option<Adt2>, v7: Option<Adt2>, v8: bool, v9: Adt2) -> R<Adt10> {
    if v8 {
        Ok(Adt10::C1)
    } else {
        if v0 {
            let m146 = v1;
            match m146 {
                None => {
                    Ok(Adt10::C1)
                }
                Some(v10) => {
                    let m147 = v2;
                    match m147 {
                        None => {
                            Ok(Adt10::C1)
                        }
                        Some(v11) => {
                            let m148 = v3;
                            match m148 {
                                None => {
                                    Ok(Adt10::C1)
                                }
                                Some(v12) => {
                                    let m149 = v4;
                                    match m149 {
                                        None => {
                                            Ok(Adt10::C1)
                                        }
                                        Some(v13) => {
                                            let m150 = v5;
                                            match m150 {
                                                None => {
                                                    Ok(Adt10::C1)
                                                }
                                                Some(v14) => {
                                                    let m151 = v6;
                                                    match m151 {
                                                        None => {
                                                            Ok(Adt10::C1)
                                                        }
                                                        Some(v15) => {
                                                            let m152 = v7;
                                                            match m152 {
                                                                None => {
                                                                    Ok(Adt10::C1)
                                                                }
                                                                Some(v16) => {
                                                                    let v17: u64 = {
                                                                        let a155 = match v12.clone() {
                                                                            Adt2::C0(_, h153, _, _, _, _, _, _, _, _, _) => {
                                                                                h153
                                                                            }
                                                                        };
                                                                        let a156 = match v12.clone() {
                                                                            Adt2::C0(_, _, h154, _, _, _, _, _, _, _, _) => {
                                                                                h154
                                                                            }
                                                                        };
                                                                        nat_mul(a155, a156)?
                                                                    };
                                                                    let v18: u64 = {
                                                                        let a158 = v17;
                                                                        let a159 = match v12.clone() {
                                                                            Adt2::C0(_, _, _, h157, _, _, _, _, _, _, _) => {
                                                                                h157
                                                                            }
                                                                        };
                                                                        nat_mul(a158, a159)?
                                                                    };
                                                                    if if if if if f29(v10, v12.clone(), v9.clone(), v18)? {
                                                                        f29(v11, v12.clone(), v9.clone(), v18)?
                                                                    } else {
                                                                        false
                                                                    } {
                                                                        f29(v13, v12.clone(), v9.clone(), v18)?
                                                                    } else {
                                                                        false
                                                                    } {
                                                                        f29(v14, v12.clone(), v9.clone(), v18)?
                                                                    } else {
                                                                        false
                                                                    } {
                                                                        f29(v15, v12.clone(), v9.clone(), v18)?
                                                                    } else {
                                                                        false
                                                                    } {
                                                                        f30(2u64, 0u64, v12, v16, v9)
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
            let m160 = v7;
            match m160 {
                None => {
                    let m161 = v1;
                    match m161 {
                        None => {
                            Ok(Adt10::C1)
                        }
                        Some(v19) => {
                            let m162 = v2;
                            match m162 {
                                None => {
                                    Ok(Adt10::C1)
                                }
                                Some(v20) => {
                                    let m163 = v3;
                                    match m163 {
                                        None => {
                                            Ok(Adt10::C1)
                                        }
                                        Some(v21) => {
                                            let m164 = v4;
                                            match m164 {
                                                None => {
                                                    Ok(Adt10::C1)
                                                }
                                                Some(v22) => {
                                                    let m165 = v5;
                                                    match m165 {
                                                        None => {
                                                            Ok(Adt10::C1)
                                                        }
                                                        Some(v23) => {
                                                            let m166 = v6;
                                                            match m166 {
                                                                None => {
                                                                    Ok(Adt10::C1)
                                                                }
                                                                Some(v24) => {
                                                                    let v25: u64 = {
                                                                        let a169 = match v19.clone() {
                                                                            Adt2::C0(_, h167, _, _, _, _, _, _, _, _, _) => {
                                                                                h167
                                                                            }
                                                                        };
                                                                        let a170 = match v19.clone() {
                                                                            Adt2::C0(_, _, h168, _, _, _, _, _, _, _, _) => {
                                                                                h168
                                                                            }
                                                                        };
                                                                        nat_mul(a169, a170)?
                                                                    };
                                                                    let v26: u64 = {
                                                                        let a172 = v25;
                                                                        let a173 = match v19.clone() {
                                                                            Adt2::C0(_, _, _, h171, _, _, _, _, _, _, _) => {
                                                                                h171
                                                                            }
                                                                        };
                                                                        nat_mul(a172, a173)?
                                                                    };
                                                                    if if if if f29(v20, v19.clone(), v9.clone(), v26)? {
                                                                        f29(v21, v19.clone(), v9.clone(), v26)?
                                                                    } else {
                                                                        false
                                                                    } {
                                                                        f29(v23, v19.clone(), v9.clone(), v26)?
                                                                    } else {
                                                                        false
                                                                    } {
                                                                        f31(v22, v9.clone(), v25)?
                                                                    } else {
                                                                        false
                                                                    } {
                                                                        f30(0u64, 0u64, v19, v24, v9)
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

pub fn f4(v0: u64, v1: Option<Adt2>, v2: Option<Adt2>, v3: Option<Adt2>, v4: Option<Adt2>, v5: Option<Adt2>, v6: bool, v7: Adt2) -> R<Adt10> {
    if v6 {
        Ok(Adt10::C1)
    } else {
        let m174 = v1;
        match m174 {
            None => {
                Ok(Adt10::C1)
            }
            Some(v8) => {
                let m175 = v2;
                match m175 {
                    None => {
                        Ok(Adt10::C1)
                    }
                    Some(v9) => {
                        let m176 = v3;
                        match m176 {
                            None => {
                                Ok(Adt10::C1)
                            }
                            Some(v10) => {
                                let m177 = v4;
                                match m177 {
                                    None => {
                                        Ok(Adt10::C1)
                                    }
                                    Some(v11) => {
                                        let m178 = v5;
                                        match m178 {
                                            None => {
                                                Ok(Adt10::C1)
                                            }
                                            Some(v12) => {
                                                let v13: u64 = {
                                                    let a181 = match v8.clone() {
                                                        Adt2::C0(_, h179, _, _, _, _, _, _, _, _, _) => {
                                                            h179
                                                        }
                                                    };
                                                    let a182 = match v8.clone() {
                                                        Adt2::C0(_, _, h180, _, _, _, _, _, _, _, _) => {
                                                            h180
                                                        }
                                                    };
                                                    nat_mul(a181, a182)?
                                                };
                                                let v14: u64 = {
                                                    let a184 = v13;
                                                    let a185 = match v8.clone() {
                                                        Adt2::C0(_, _, _, h183, _, _, _, _, _, _, _) => {
                                                            h183
                                                        }
                                                    };
                                                    nat_mul(a184, a185)?
                                                };
                                                if if if f29(v9, v8.clone(), v7.clone(), v14)? {
                                                    f29(v10, v8.clone(), v7.clone(), v14)?
                                                } else {
                                                    false
                                                } {
                                                    f29(v11, v8.clone(), v7.clone(), v14)?
                                                } else {
                                                    false
                                                } {
                                                    f30(1u64, v0, v8, v12, v7)
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

pub fn f5(v0: bool, v1: Option<Adt2>, v2: Option<Adt2>, v3: Option<Adt2>, v4: bool, v5: Adt2) -> R<Adt10> {
    let v7: Adt2 = {
        let m186 = v1.clone();
        match m186 {
            None => {
                v5.clone()
            }
            Some(v6) => {
                v6
            }
        }
    };
    let v10: Adt2 = if v0 {
        let m187 = v2.clone();
        match m187 {
            None => {
                v5.clone()
            }
            Some(v8) => {
                v8
            }
        }
    } else {
        let m188 = v1.clone();
        match m188 {
            None => {
                v5.clone()
            }
            Some(v9) => {
                v9
            }
        }
    };
    let v13: Adt2 = if v0 {
        let m189 = v3.clone();
        match m189 {
            None => {
                v5.clone()
            }
            Some(v11) => {
                v11
            }
        }
    } else {
        let m190 = v2.clone();
        match m190 {
            None => {
                v5.clone()
            }
            Some(v12) => {
                v12
            }
        }
    };
    let v14: u64 = {
        let a196 = {
            let a193 = match v10.clone() {
                Adt2::C0(_, _, h191, _, _, _, _, _, _, _, _) => {
                    h191
                }
            };
            let a194 = match v10.clone() {
                Adt2::C0(_, _, _, h192, _, _, _, _, _, _, _) => {
                    h192
                }
            };
            nat_mul(a193, a194)?
        };
        let a197 = match v10.clone() {
            Adt2::C0(_, _, _, _, h195, _, _, _, _, _, _) => {
                h195
            }
        };
        nat_mul(a196, a197)?
    };
    let m198 = v1;
    if if if if if if if if if if match m198 {
        None => {
            false
        }
        Some(_) => {
            true
        }
    } {
        let m199 = v2;
        match m199 {
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
            let m200 = v3;
            match m200 {
                None => {
                    false
                }
                Some(_) => {
                    true
                }
            }
        } else {
            let m201 = v3;
            !match m201 {
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
        let a202 = 0u64;
        let a203 = v14;
        nat_lt(a202, a203)
    } else {
        false
    } {
        let a205 = 0u64;
        let a206 = match v10.clone() {
            Adt2::C0(_, h204, _, _, _, _, _, _, _, _, _) => {
                h204
            }
        };
        nat_lt(a205, a206)
    } else {
        false
    } {
        f32(v10.clone(), v13.clone())
    } else {
        false
    } {
        f33(v10.clone(), v14, v0)?
    } else {
        false
    } {
        f33(v13.clone(), v14, v0)?
    } else {
        false
    } {
        if v0 {
            let a208 = match v7.clone() {
                Adt2::C0(h207, _, _, _, _, _, _, _, _, _, _) => {
                    h207
                }
            };
            let a209 = 0u64;
            if if if if nat_eq(a208, a209) {
                let a210 = f25(v7.clone(), 1u64)?;
                let a211 = 1u64;
                nat_eq(a210, a211)
            } else {
                false
            } {
                let a213 = 4u64;
                let a214 = match v7 {
                    Adt2::C0(_, _, _, _, _, _, _, _, _, h212, _) => {
                        h212
                    }
                };
                nat_le(a213, a214)
            } else {
                false
            } {
                f32(v10.clone(), v5.clone())
            } else {
                false
            } {
                f33(v5.clone(), v14, true)?
            } else {
                false
            }
        } else {
            let a216 = match v5.clone() {
                Adt2::C0(h215, _, _, _, _, _, _, _, _, _, _) => {
                    h215
                }
            };
            let a217 = 0u64;
            if if nat_eq(a216, a217) {
                let a218 = f25(v5.clone(), 1u64)?;
                let a219 = 1u64;
                nat_eq(a218, a219)
            } else {
                false
            } {
                let a221 = 4u64;
                let a222 = match v5.clone() {
                    Adt2::C0(_, _, _, _, _, _, _, _, _, h220, _) => {
                        h220
                    }
                };
                nat_le(a221, a222)
            } else {
                false
            }
        }
    } else {
        false
    } {
        Ok(Adt10::C4({
            let a228 = {
                let a226 = 2u64;
                let a227 = {
                    let a224 = v14;
                    let a225 = f34(v0, match v10.clone() {
                        Adt2::C0(_, h223, _, _, _, _, _, _, _, _, _) => {
                            h223
                        }
                    })?;
                    nat_mul(a224, a225)?
                };
                nat_add(a226, a227)?
            };
            let a229 = if v0 {
                0u64
            } else {
                1u64
            };
            nat_add(a228, a229)?
        }, 4u64, f28(v10, 1u64), f28(v5, 1u64), Adt4::C40(v0, match v13 {
            Adt2::C0(_, _, _, _, _, _, h230, _, _, _, _) => {
                h230
            }
        }, v14)))
    } else {
        Ok(Adt10::C1)
    }
}

pub fn f6(v0: bool, v1: u64, v2: bool, v3: Option<Adt2>, v4: Option<Adt2>, v5: Option<Adt2>, v6: Option<Adt2>, v7: Option<Adt2>, v8: bool, v9: Adt2) -> R<Adt10> {
    let v11: Adt2 = {
        let m231 = v3.clone();
        match m231 {
            None => {
                v9.clone()
            }
            Some(v10) => {
                v10
            }
        }
    };
    let v13: Adt2 = {
        let m232 = v4.clone();
        match m232 {
            None => {
                v9.clone()
            }
            Some(v12) => {
                v12
            }
        }
    };
    let v15: Adt2 = {
        let m233 = v5.clone();
        match m233 {
            None => {
                v9.clone()
            }
            Some(v14) => {
                v14
            }
        }
    };
    let v17: Adt2 = {
        let m234 = v6.clone();
        match m234 {
            None => {
                v9.clone()
            }
            Some(v16) => {
                v16
            }
        }
    };
    let v20: Adt2 = if v0 {
        let m235 = v7.clone();
        match m235 {
            None => {
                v9.clone()
            }
            Some(v18) => {
                v18
            }
        }
    } else {
        let m236 = v5.clone();
        match m236 {
            None => {
                v9.clone()
            }
            Some(v19) => {
                v19
            }
        }
    };
    let m237 = v3;
    if if if if if if if if if if if match m237 {
        None => {
            false
        }
        Some(_) => {
            true
        }
    } {
        let m238 = v4;
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
        let m239 = v5;
        match m239 {
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
            let m240 = v6;
            if match m240 {
                None => {
                    false
                }
                Some(_) => {
                    true
                }
            } {
                let m241 = v7;
                match m241 {
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
            let m242 = v6;
            if !match m242 {
                None => {
                    false
                }
                Some(_) => {
                    true
                }
            } {
                let m243 = v7;
                !match m243 {
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
            let a250 = {
                let a247 = {
                    let a244 = v1;
                    let a245 = 4u64;
                    let a246 = 0u64;
                    nat_quot(a244, a245, a246)
                };
                let a248 = 2u64;
                let a249 = 0u64;
                nat_rem(a247, a248, a249)
            };
            let a251 = 1u64;
            nat_eq(a250, a251)
        } else {
            false
        } {
            let a253 = match v11.clone() {
                Adt2::C0(h252, _, _, _, _, _, _, _, _, _, _) => {
                    h252
                }
            };
            let a254 = 0u64;
            nat_eq(a253, a254)
        } else {
            false
        } {
            let a256 = match v11.clone() {
                Adt2::C0(_, _, _, _, _, h255, _, _, _, _, _) => {
                    h255
                }
            };
            let a257 = 4u64;
            nat_eq(a256, a257)
        } else {
            false
        } {
            f26(v11.clone(), f25(v11.clone(), 1u64)?)?
        } else {
            false
        } {
            f35(v11.clone(), v9.clone())?
        } else {
            false
        } {
            let a259 = match v11.clone() {
                Adt2::C0(_, h258, _, _, _, _, _, _, _, _, _) => {
                    h258
                }
            };
            let a260 = 1u64;
            if nat_le(a259, a260) {
                true
            } else {
                let a263 = match v11.clone() {
                    Adt2::C0(_, _, _, _, _, h261, _, _, _, _, _) => {
                        h261
                    }
                };
                let a264 = match v9.clone() {
                    Adt2::C0(_, _, _, _, _, h262, _, _, _, _, _) => {
                        h262
                    }
                };
                nat_eq(a263, a264)
            }
        } else {
            false
        } {
            let a266 = match v11.clone() {
                Adt2::C0(_, _, h265, _, _, _, _, _, _, _, _) => {
                    h265
                }
            };
            let a267 = 1u64;
            if nat_le(a266, a267) {
                true
            } else {
                let a270 = match v11.clone() {
                    Adt2::C0(_, _, _, _, _, _, h268, _, _, _, _) => {
                        h268
                    }
                };
                let a271 = match v9.clone() {
                    Adt2::C0(_, _, _, _, _, _, h269, _, _, _, _) => {
                        h269
                    }
                };
                nat_eq(a270, a271)
            }
        } else {
            false
        } {
            let a273 = match v11.clone() {
                Adt2::C0(_, _, _, h272, _, _, _, _, _, _, _) => {
                    h272
                }
            };
            let a274 = 1u64;
            if nat_le(a273, a274) {
                true
            } else {
                let a277 = match v11.clone() {
                    Adt2::C0(_, _, _, _, _, _, _, h275, _, _, _) => {
                        h275
                    }
                };
                let a278 = match v9.clone() {
                    Adt2::C0(_, _, _, _, _, _, _, h276, _, _, _) => {
                        h276
                    }
                };
                nat_eq(a277, a278)
            }
        } else {
            false
        } {
            let a280 = match v11.clone() {
                Adt2::C0(_, _, _, _, h279, _, _, _, _, _, _) => {
                    h279
                }
            };
            let a281 = 1u64;
            if nat_le(a280, a281) {
                true
            } else {
                let a284 = match v11.clone() {
                    Adt2::C0(_, _, _, _, _, _, _, _, h282, _, _) => {
                        h282
                    }
                };
                let a285 = match v9.clone() {
                    Adt2::C0(_, _, _, _, _, _, _, _, h283, _, _) => {
                        h283
                    }
                };
                nat_eq(a284, a285)
            }
        } else {
            false
        }
    } else {
        false
    } {
        f35(v11.clone(), v13)?
    } else {
        false
    } {
        if v0 {
            if f35(v11.clone(), v15)? {
                f35(v11.clone(), v17)?
            } else {
                false
            }
        } else {
            true
        }
    } else {
        false
    } {
        let a287 = match v20.clone() {
            Adt2::C0(h286, _, _, _, _, _, _, _, _, _, _) => {
                h286
            }
        };
        let a288 = 0u64;
        nat_eq(a287, a288)
    } else {
        false
    } {
        let a289 = f25(v20.clone(), 1u64)?;
        let a290 = if v0 {
            7u64
        } else {
            2u64
        };
        nat_eq(a289, a290)
    } else {
        false
    } {
        let a292 = if v0 {
            28u64
        } else {
            8u64
        };
        let a293 = match v20 {
            Adt2::C0(_, _, _, _, _, _, _, _, _, h291, _) => {
                h291
            }
        };
        nat_le(a292, a293)
    } else {
        false
    } {
        Ok(Adt10::C4({
            let a296 = f36(v0);
            let a297 = {
                let a294 = f25(v11.clone(), 1u64)?;
                let a295 = f37(v0);
                nat_mul(a294, a295)?
            };
            nat_add(a296, a297)?
        }, 4u64, f28(v11, 1u64), f28(v9, 1u64), Adt4::C39(v0)))
    } else {
        Ok(Adt10::C1)
    }
}

pub fn f7(v0: Adt1, v1: Option<Adt2>, v2: Option<Adt2>, v3: Option<Adt2>, v4: bool, v5: Adt2) -> R<Adt10> {
    let v7: Adt2 = {
        let m298 = v1.clone();
        match m298 {
            None => {
                v5.clone()
            }
            Some(v6) => {
                v6
            }
        }
    };
    let v9: Adt2 = {
        let m299 = v2.clone();
        match m299 {
            None => {
                v5.clone()
            }
            Some(v8) => {
                v8
            }
        }
    };
    let v11: Adt2 = {
        let m300 = v3.clone();
        match m300 {
            None => {
                v5.clone()
            }
            Some(v10) => {
                v10
            }
        }
    };
    let v12: u64 = match v0.clone() {
        Adt1::C0(_, h301, _, _, _, _, _, _, _, _, _, _) => {
            h301
        }
    };
    let m302 = v1;
    if if if if if if if if if if if if if if if if if if if if if if if if if match m302 {
        None => {
            false
        }
        Some(_) => {
            true
        }
    } {
        let m303 = v2;
        match m303 {
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
        let m304 = v3;
        match m304 {
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
        let a306 = match v7.clone() {
            Adt2::C0(h305, _, _, _, _, _, _, _, _, _, _) => {
                h305
            }
        };
        let a307 = 0u64;
        if if nat_eq(a306, a307) {
            f26(v7.clone(), f25(v7.clone(), 1u64)?)?
        } else {
            false
        } {
            f27(v7.clone(), v5.clone())?
        } else {
            false
        }
    } else {
        false
    } {
        let a309 = match v9.clone() {
            Adt2::C0(h308, _, _, _, _, _, _, _, _, _, _) => {
                h308
            }
        };
        let a310 = 0u64;
        if if nat_eq(a309, a310) {
            f26(v9.clone(), f25(v9.clone(), 1u64)?)?
        } else {
            false
        } {
            f27(v9.clone(), v5.clone())?
        } else {
            false
        }
    } else {
        false
    } {
        let a312 = match v11.clone() {
            Adt2::C0(h311, _, _, _, _, _, _, _, _, _, _) => {
                h311
            }
        };
        let a313 = 0u64;
        if if nat_eq(a312, a313) {
            f26(v11.clone(), f25(v11.clone(), 1u64)?)?
        } else {
            false
        } {
            f27(v11.clone(), v5.clone())?
        } else {
            false
        }
    } else {
        false
    } {
        let a315 = match v7.clone() {
            Adt2::C0(_, h314, _, _, _, _, _, _, _, _, _) => {
                h314
            }
        };
        let a316 = 24u64;
        nat_eq(a315, a316)
    } else {
        false
    } {
        let a318 = match v7.clone() {
            Adt2::C0(_, _, _, h317, _, _, _, _, _, _, _) => {
                h317
            }
        };
        let a319 = 1u64;
        nat_eq(a318, a319)
    } else {
        false
    } {
        let a321 = match v7.clone() {
            Adt2::C0(_, _, _, _, h320, _, _, _, _, _, _) => {
                h320
            }
        };
        let a322 = 1u64;
        nat_eq(a321, a322)
    } else {
        false
    } {
        let a324 = match v11.clone() {
            Adt2::C0(_, h323, _, _, _, _, _, _, _, _, _) => {
                h323
            }
        };
        let a325 = 24u64;
        nat_eq(a324, a325)
    } else {
        false
    } {
        let a327 = match v11.clone() {
            Adt2::C0(_, _, h326, _, _, _, _, _, _, _, _) => {
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
        let a333 = match v11.clone() {
            Adt2::C0(_, _, _, _, h332, _, _, _, _, _, _) => {
                h332
            }
        };
        let a334 = 1u64;
        nat_eq(a333, a334)
    } else {
        false
    } {
        let a336 = match v9.clone() {
            Adt2::C0(_, _, h335, _, _, _, _, _, _, _, _) => {
                h335
            }
        };
        let a337 = 1u64;
        nat_eq(a336, a337)
    } else {
        false
    } {
        let a339 = match v9.clone() {
            Adt2::C0(_, _, _, h338, _, _, _, _, _, _, _) => {
                h338
            }
        };
        let a340 = 1u64;
        nat_eq(a339, a340)
    } else {
        false
    } {
        let a342 = match v9.clone() {
            Adt2::C0(_, _, _, _, h341, _, _, _, _, _, _) => {
                h341
            }
        };
        let a343 = 1u64;
        nat_eq(a342, a343)
    } else {
        false
    } {
        let a345 = 3u64;
        let a346 = match v9.clone() {
            Adt2::C0(_, h344, _, _, _, _, _, _, _, _, _) => {
                h344
            }
        };
        nat_le(a345, a346)
    } else {
        false
    } {
        let a348 = match v5.clone() {
            Adt2::C0(_, h347, _, _, _, _, _, _, _, _, _) => {
                h347
            }
        };
        let a349 = 4u64;
        nat_eq(a348, a349)
    } else {
        false
    } {
        let a351 = match v5.clone() {
            Adt2::C0(_, _, h350, _, _, _, _, _, _, _, _) => {
                h350
            }
        };
        let a352 = 4u64;
        nat_eq(a351, a352)
    } else {
        false
    } {
        let a355 = match v5.clone() {
            Adt2::C0(_, _, _, h353, _, _, _, _, _, _, _) => {
                h353
            }
        };
        let a356 = match v7.clone() {
            Adt2::C0(_, _, h354, _, _, _, _, _, _, _, _) => {
                h354
            }
        };
        nat_eq(a355, a356)
    } else {
        false
    } {
        let a358 = match v5.clone() {
            Adt2::C0(_, _, _, _, h357, _, _, _, _, _, _) => {
                h357
            }
        };
        let a359 = 1u64;
        nat_eq(a358, a359)
    } else {
        false
    } {
        f38(v5.clone())?
    } else {
        false
    } {
        let a360 = 0u64;
        let a361 = v12;
        nat_lt(a360, a361)
    } else {
        false
    } {
        let a362 = v12;
        let a363 = 2147483648u64;
        nat_lt(a362, a363)
    } else {
        false
    } {
        Ok(Adt10::C4({
            let a365 = match v7.clone() {
                Adt2::C0(_, _, h364, _, _, _, _, _, _, _, _) => {
                    h364
                }
            };
            let a366 = f39(v12)?;
            nat_mul(a365, a366)?
        }, 4u64, f28(v7, 1u64), f28(v5, 1u64), Adt4::C38(f28(v9, 1u64), f28(v11, 1u64), match v0 {
            Adt1::C0(h367, _, _, _, _, _, _, _, _, _, _, _) => {
                h367
            }
        }, v12)))
    } else {
        Ok(Adt10::C1)
    }
}

pub fn f8(v0: Option<Adt2>, v1: Option<Adt2>, v2: Option<Adt2>, v3: Option<Adt2>, v4: bool, v5: Adt2) -> R<Adt10> {
    let v7: Adt2 = {
        let m368 = v0.clone();
        match m368 {
            None => {
                v5.clone()
            }
            Some(v6) => {
                v6
            }
        }
    };
    let v9: Adt2 = {
        let m369 = v1.clone();
        match m369 {
            None => {
                v5.clone()
            }
            Some(v8) => {
                v8
            }
        }
    };
    let v11: Adt2 = {
        let m370 = v2.clone();
        match m370 {
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
        let m371 = v0;
        match m371 {
            None => {
                false
            }
            Some(_) => {
                let m372 = v1;
                match m372 {
                    None => {
                        false
                    }
                    Some(_) => {
                        let m373 = v2;
                        match m373 {
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
        let a375 = match v7.clone() {
            Adt2::C0(h374, _, _, _, _, _, _, _, _, _, _) => {
                h374
            }
        };
        let a376 = 0u64;
        if if if if if if if if if if if if if if nat_eq(a375, a376) {
            let a378 = match v7.clone() {
                Adt2::C0(_, _, _, _, h377, _, _, _, _, _, _) => {
                    h377
                }
            };
            let a379 = 1u64;
            nat_eq(a378, a379)
        } else {
            false
        } {
            f26(v7.clone(), f25(v7.clone(), 1u64)?)?
        } else {
            false
        } {
            f27(v7.clone(), v5.clone())?
        } else {
            false
        } {
            let a381 = match v9.clone() {
                Adt2::C0(h380, _, _, _, _, _, _, _, _, _, _) => {
                    h380
                }
            };
            let a382 = 0u64;
            if if if nat_eq(a381, a382) {
                let a384 = match v9.clone() {
                    Adt2::C0(_, _, _, _, h383, _, _, _, _, _, _) => {
                        h383
                    }
                };
                let a385 = 1u64;
                nat_eq(a384, a385)
            } else {
                false
            } {
                f26(v9.clone(), f25(v9.clone(), 1u64)?)?
            } else {
                false
            } {
                f27(v9.clone(), v5.clone())?
            } else {
                false
            }
        } else {
            false
        } {
            let a387 = match v11.clone() {
                Adt2::C0(h386, _, _, _, _, _, _, _, _, _, _) => {
                    h386
                }
            };
            let a388 = 0u64;
            if if if nat_eq(a387, a388) {
                let a390 = match v11.clone() {
                    Adt2::C0(_, _, _, _, h389, _, _, _, _, _, _) => {
                        h389
                    }
                };
                let a391 = 1u64;
                nat_eq(a390, a391)
            } else {
                false
            } {
                f26(v11.clone(), f25(v11.clone(), 1u64)?)?
            } else {
                false
            } {
                f27(v11.clone(), v5.clone())?
            } else {
                false
            }
        } else {
            false
        } {
            let a393 = match v7.clone() {
                Adt2::C0(_, _, _, h392, _, _, _, _, _, _, _) => {
                    h392
                }
            };
            let a394 = 1u64;
            nat_eq(a393, a394)
        } else {
            false
        } {
            let a396 = match v11.clone() {
                Adt2::C0(_, _, _, h395, _, _, _, _, _, _, _) => {
                    h395
                }
            };
            let a397 = 1u64;
            nat_eq(a396, a397)
        } else {
            false
        } {
            let a400 = match v9.clone() {
                Adt2::C0(_, h398, _, _, _, _, _, _, _, _, _) => {
                    h398
                }
            };
            let a401 = match v7.clone() {
                Adt2::C0(_, h399, _, _, _, _, _, _, _, _, _) => {
                    h399
                }
            };
            nat_eq(a400, a401)
        } else {
            false
        } {
            let a404 = match v9.clone() {
                Adt2::C0(_, _, _, h402, _, _, _, _, _, _, _) => {
                    h402
                }
            };
            let a405 = match v7.clone() {
                Adt2::C0(_, _, h403, _, _, _, _, _, _, _, _) => {
                    h403
                }
            };
            nat_eq(a404, a405)
        } else {
            false
        } {
            let a408 = match v11.clone() {
                Adt2::C0(_, h406, _, _, _, _, _, _, _, _, _) => {
                    h406
                }
            };
            let a409 = match v9.clone() {
                Adt2::C0(_, _, h407, _, _, _, _, _, _, _, _) => {
                    h407
                }
            };
            nat_eq(a408, a409)
        } else {
            false
        } {
            let a412 = match v11.clone() {
                Adt2::C0(_, _, h410, _, _, _, _, _, _, _, _) => {
                    h410
                }
            };
            let a413 = match v7.clone() {
                Adt2::C0(_, _, h411, _, _, _, _, _, _, _, _) => {
                    h411
                }
            };
            nat_eq(a412, a413)
        } else {
            false
        } {
            let a415 = 0u64;
            let a416 = match v9.clone() {
                Adt2::C0(_, _, h414, _, _, _, _, _, _, _, _) => {
                    h414
                }
            };
            if nat_lt(a415, a416) {
                f32(v9.clone(), v5.clone())
            } else {
                false
            }
        } else {
            false
        } {
            f38(v5.clone())?
        } else {
            false
        } {
            let m417 = v3.clone();
            match m417 {
                None => {
                    true
                }
                Some(v15) => {
                    let a419 = match v15.clone() {
                        Adt2::C0(h418, _, _, _, _, _, _, _, _, _, _) => {
                            h418
                        }
                    };
                    let a420 = 0u64;
                    if if if if if if nat_eq(a419, a420) {
                        let a422 = match v15.clone() {
                            Adt2::C0(_, _, _, _, h421, _, _, _, _, _, _) => {
                                h421
                            }
                        };
                        let a423 = 1u64;
                        nat_eq(a422, a423)
                    } else {
                        false
                    } {
                        f26(v15.clone(), f25(v15.clone(), 1u64)?)?
                    } else {
                        false
                    } {
                        f27(v15.clone(), v5.clone())?
                    } else {
                        false
                    } {
                        let a426 = match v15.clone() {
                            Adt2::C0(_, h424, _, _, _, _, _, _, _, _, _) => {
                                h424
                            }
                        };
                        let a427 = match v9.clone() {
                            Adt2::C0(_, _, h425, _, _, _, _, _, _, _, _) => {
                                h425
                            }
                        };
                        nat_eq(a426, a427)
                    } else {
                        false
                    } {
                        let a430 = match v15.clone() {
                            Adt2::C0(_, _, h428, _, _, _, _, _, _, _, _) => {
                                h428
                            }
                        };
                        let a431 = match v9.clone() {
                            Adt2::C0(_, _, h429, _, _, _, _, _, _, _, _) => {
                                h429
                            }
                        };
                        nat_eq(a430, a431)
                    } else {
                        false
                    } {
                        let a434 = match v15 {
                            Adt2::C0(_, _, _, h432, _, _, _, _, _, _, _) => {
                                h432
                            }
                        };
                        let a435 = match v7.clone() {
                            Adt2::C0(_, _, h433, _, _, _, _, _, _, _, _) => {
                                h433
                            }
                        };
                        nat_eq(a434, a435)
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
            let m436 = v3;
            match m436 {
                None => {
                    None::<Adt3>
                }
                Some(v16) => {
                    Some(f28(v16, 1u64))
                }
            }
        };
        Ok(Adt10::C4({
            let a438 = f25(v5.clone(), 1u64)?;
            let a439 = f40(match v9.clone() {
                Adt2::C0(_, _, h437, _, _, _, _, _, _, _, _) => {
                    h437
                }
            }, v17.clone())?;
            nat_mul(a438, a439)?
        }, 4u64, f28(v7, 1u64), f28(v5, 1u64), Adt4::C37(f28(v9, 1u64), f28(v11, 1u64), v17)))
    } else {
        Ok(Adt10::C1)
    }
}

pub fn f9(v0: Adt1, v1: Adt2) -> R<Adt10> {
    let v2: u64 = f25(v1.clone(), 1u64)?;
    let a441 = match v1.clone() {
        Adt2::C0(h440, _, _, _, _, _, _, _, _, _, _) => {
            h440
        }
    };
    let a442 = 0u64;
    if if if if if if if if nat_eq(a441, a442) {
        let a444 = match v1.clone() {
            Adt2::C0(_, _, _, _, _, h443, _, _, _, _, _) => {
                h443
            }
        };
        let a445 = 4u64;
        nat_eq(a444, a445)
    } else {
        false
    } {
        f41(match v0.clone() {
            Adt1::C0(h446, _, _, _, _, _, _, _, _, _, _, _) => {
                h446
            }
        })
    } else {
        false
    } {
        f41(match v0.clone() {
            Adt1::C0(_, h447, _, _, _, _, _, _, _, _, _, _) => {
                h447
            }
        })
    } else {
        false
    } {
        let a449 = 0u64;
        let a450 = match v0.clone() {
            Adt1::C0(_, _, h448, _, _, _, _, _, _, _, _, _) => {
                h448
            }
        };
        nat_lt(a449, a450)
    } else {
        false
    } {
        let a452 = match v0.clone() {
            Adt1::C0(_, _, h451, _, _, _, _, _, _, _, _, _) => {
                h451
            }
        };
        let a453 = 2139095040u64;
        nat_lt(a452, a453)
    } else {
        false
    } {
        let a454 = v2;
        let a455 = 9223372036854775808u64;
        nat_lt(a454, a455)
    } else {
        false
    } {
        let a457 = f42(v2)?;
        let a458 = match v1.clone() {
            Adt2::C0(_, _, _, _, _, _, _, _, _, h456, _) => {
                h456
            }
        };
        nat_le(a457, a458)
    } else {
        false
    } {
        Ok(Adt10::C4(f43(v2)?, 4u64, f28(v1.clone(), 1u64), f28(v1, 1u64), Adt4::C23(match v0.clone() {
            Adt1::C0(h459, _, _, _, _, _, _, _, _, _, _, _) => {
                h459
            }
        }, match v0.clone() {
            Adt1::C0(_, h460, _, _, _, _, _, _, _, _, _, _) => {
                h460
            }
        }, match v0 {
            Adt1::C0(_, _, h461, _, _, _, _, _, _, _, _, _) => {
                h461
            }
        }, v2)))
    } else {
        Ok(Adt10::C1)
    }
}

pub fn f10(v0: bool, v1: Adt1, v2: Adt2, v3: Option<Adt2>, v4: Adt2) -> R<Adt10> {
    let m462 = v3;
    match m462 {
        None => {
            Ok(Adt10::C1)
        }
        Some(v5) => {
            let v6: Adt3 = f44(v0, v2.clone());
            let v7: Adt3 = f44(v0, v5.clone());
            let v8: Adt3 = f44(v0, v4.clone());
            let v9: u64 = match v7.clone() {
                Adt3::C0(h463, _, _, _, _, _, _, _) => {
                    h463
                }
            };
            let v10: u64 = match v7.clone() {
                Adt3::C0(_, h464, _, _, _, _, _, _) => {
                    h464
                }
            };
            let v11: u64 = match v7.clone() {
                Adt3::C0(_, _, h465, _, _, _, _, _) => {
                    h465
                }
            };
            let v12: u64 = match v7.clone() {
                Adt3::C0(_, _, _, h466, _, _, _, _) => {
                    h466
                }
            };
            let v13: u64 = match v6.clone() {
                Adt3::C0(h467, _, _, _, _, _, _, _) => {
                    h467
                }
            };
            let v14: u64 = match v6.clone() {
                Adt3::C0(_, h468, _, _, _, _, _, _) => {
                    h468
                }
            };
            let v15: u64 = match v6.clone() {
                Adt3::C0(_, _, h469, _, _, _, _, _) => {
                    h469
                }
            };
            let v16: u64 = match v1.clone() {
                Adt1::C0(h470, _, _, _, _, _, _, _, _, _, _, _) => {
                    h470
                }
            };
            let v17: u64 = {
                let a475 = v13;
                let a476 = {
                    let a473 = {
                        let a471 = v9;
                        let a472 = 1u64;
                        nat_sub(a471, a472)
                    };
                    let a474 = v16;
                    nat_mul(a473, a474)?
                };
                nat_add(a475, a476)?
            };
            let v18: u64 = {
                let a481 = v14;
                let a482 = {
                    let a479 = {
                        let a477 = v10;
                        let a478 = 1u64;
                        nat_sub(a477, a478)
                    };
                    let a480 = v16;
                    nat_mul(a479, a480)?
                };
                nat_add(a481, a482)?
            };
            let v19: u64 = {
                let a485 = {
                    let a483 = v9;
                    let a484 = v10;
                    nat_mul(a483, a484)?
                };
                let a486 = v12;
                nat_mul(a485, a486)?
            };
            let v20: u64 = {
                let a489 = {
                    let a487 = v13;
                    let a488 = v14;
                    nat_mul(a487, a488)?
                };
                let a490 = v15;
                nat_mul(a489, a490)?
            };
            let v21: u64 = {
                let a491 = v19;
                let a492 = 32u64;
                if nat_lt(a491, a492) {
                    v19
                } else {
                    32u64
                }
            };
            let a494 = match v2.clone() {
                Adt2::C0(h493, _, _, _, _, _, _, _, _, _, _) => {
                    h493
                }
            };
            let a495 = 0u64;
            if if if if if if if if if if if if if if if if if if if if if if if if nat_eq(a494, a495) {
                true
            } else {
                let a497 = match v2.clone() {
                    Adt2::C0(h496, _, _, _, _, _, _, _, _, _, _) => {
                        h496
                    }
                };
                let a498 = 1u64;
                nat_eq(a497, a498)
            } {
                let a500 = match v5.clone() {
                    Adt2::C0(h499, _, _, _, _, _, _, _, _, _, _) => {
                        h499
                    }
                };
                let a501 = 0u64;
                nat_eq(a500, a501)
            } else {
                false
            } {
                let a503 = match v4.clone() {
                    Adt2::C0(h502, _, _, _, _, _, _, _, _, _, _) => {
                        h502
                    }
                };
                let a504 = 0u64;
                nat_eq(a503, a504)
            } else {
                false
            } {
                let a505 = v16;
                let a506 = 2147483648u64;
                nat_lt(a505, a506)
            } else {
                false
            } {
                let a507 = 0u64;
                let a508 = v9;
                nat_lt(a507, a508)
            } else {
                false
            } {
                let a509 = 0u64;
                let a510 = v10;
                nat_lt(a509, a510)
            } else {
                false
            } {
                let a511 = 0u64;
                let a512 = v11;
                nat_lt(a511, a512)
            } else {
                false
            } {
                let a513 = 0u64;
                let a514 = v12;
                nat_lt(a513, a514)
            } else {
                false
            } {
                let a515 = 0u64;
                let a516 = v13;
                nat_lt(a515, a516)
            } else {
                false
            } {
                let a517 = 0u64;
                let a518 = v14;
                nat_lt(a517, a518)
            } else {
                false
            } {
                let a519 = 0u64;
                let a520 = v15;
                nat_lt(a519, a520)
            } else {
                false
            } {
                let a521 = 0u64;
                let a522 = v16;
                nat_lt(a521, a522)
            } else {
                false
            } {
                let a524 = match v6.clone() {
                    Adt3::C0(_, _, _, h523, _, _, _, _) => {
                        h523
                    }
                };
                let a525 = v11;
                nat_eq(a524, a525)
            } else {
                false
            } {
                let a527 = match v8.clone() {
                    Adt3::C0(h526, _, _, _, _, _, _, _) => {
                        h526
                    }
                };
                let a528 = v17;
                nat_eq(a527, a528)
            } else {
                false
            } {
                let a530 = match v8.clone() {
                    Adt3::C0(_, h529, _, _, _, _, _, _) => {
                        h529
                    }
                };
                let a531 = v18;
                nat_eq(a530, a531)
            } else {
                false
            } {
                let a533 = match v8.clone() {
                    Adt3::C0(_, _, h532, _, _, _, _, _) => {
                        h532
                    }
                };
                let a534 = v15;
                nat_eq(a533, a534)
            } else {
                false
            } {
                let a536 = match v8.clone() {
                    Adt3::C0(_, _, _, h535, _, _, _, _) => {
                        h535
                    }
                };
                let a537 = v12;
                nat_eq(a536, a537)
            } else {
                false
            } {
                f45(v4.clone())?
            } else {
                false
            } {
                let a542 = match v2.clone() {
                    Adt2::C0(_, _, _, _, _, h538, _, _, _, _, _) => {
                        h538
                    }
                };
                let a543 = {
                    let a540 = match v2.clone() {
                        Adt2::C0(h539, _, _, _, _, _, _, _, _, _, _) => {
                            h539
                        }
                    };
                    let a541 = 1u64;
                    if nat_eq(a540, a541) {
                        2u64
                    } else {
                        4u64
                    }
                };
                nat_eq(a542, a543)
            } else {
                false
            } {
                let a545 = match v5.clone() {
                    Adt2::C0(_, _, _, _, _, h544, _, _, _, _, _) => {
                        h544
                    }
                };
                let a546 = 4u64;
                nat_eq(a545, a546)
            } else {
                false
            } {
                f27(v2.clone(), v4.clone())?
            } else {
                false
            } {
                f27(v5.clone(), v4.clone())?
            } else {
                false
            } {
                if v0 {
                    let a551 = match v2.clone() {
                        Adt2::C0(_, _, _, _, _, _, h547, _, _, _, _) => {
                            h547
                        }
                    };
                    let a552 = {
                        let a549 = v13;
                        let a550 = match v2 {
                            Adt2::C0(_, _, _, _, _, h548, _, _, _, _, _) => {
                                h548
                            }
                        };
                        nat_mul(a549, a550)?
                    };
                    nat_eq(a551, a552)
                } else {
                    let a554 = match v1.clone() {
                        Adt1::C0(_, h553, _, _, _, _, _, _, _, _, _, _) => {
                            h553
                        }
                    };
                    let a555 = 0u64;
                    if if if if if if nat_eq(a554, a555) {
                        let a557 = match v1 {
                            Adt1::C0(_, _, h556, _, _, _, _, _, _, _, _, _) => {
                                h556
                            }
                        };
                        let a558 = 1u64;
                        nat_eq(a557, a558)
                    } else {
                        false
                    } {
                        let a560 = match v2 {
                            Adt2::C0(_, _, _, _, h559, _, _, _, _, _, _) => {
                                h559
                            }
                        };
                        let a561 = 1u64;
                        nat_eq(a560, a561)
                    } else {
                        false
                    } {
                        let a563 = match v5.clone() {
                            Adt2::C0(_, _, _, h562, _, _, _, _, _, _, _) => {
                                h562
                            }
                        };
                        let a564 = 1u64;
                        nat_eq(a563, a564)
                    } else {
                        false
                    } {
                        let a566 = match v5 {
                            Adt2::C0(_, _, _, _, h565, _, _, _, _, _, _) => {
                                h565
                            }
                        };
                        let a567 = 1u64;
                        nat_eq(a566, a567)
                    } else {
                        false
                    } {
                        let a569 = match v4.clone() {
                            Adt2::C0(_, _, _, h568, _, _, _, _, _, _, _) => {
                                h568
                            }
                        };
                        let a570 = 1u64;
                        nat_eq(a569, a570)
                    } else {
                        false
                    } {
                        let a572 = match v4 {
                            Adt2::C0(_, _, _, _, h571, _, _, _, _, _, _) => {
                                h571
                            }
                        };
                        let a573 = 1u64;
                        nat_eq(a572, a573)
                    } else {
                        false
                    }
                }
            } else {
                false
            } {
                Ok(Adt10::C6(Adt8::C0(v19, v11, v20, {
                    let a574 = v11;
                    let a575 = v20;
                    nat_mul(a574, a575)?
                }, v21, {
                    let a576 = v21;
                    let a577 = v11;
                    nat_mul(a576, a577)?
                }, {
                    let a578 = v21;
                    let a579 = v20;
                    nat_mul(a578, a579)?
                }, {
                    let a584 = {
                        let a580 = v17;
                        let a581 = v18;
                        nat_mul(a580, a581)?
                    };
                    let a585 = {
                        let a582 = v15;
                        let a583 = v12;
                        nat_mul(a582, a583)?
                    };
                    nat_mul(a584, a585)?
                }, v6, v7, v8, Adt6::C0(v13, v16, 0u64, 1u64), Adt6::C0(v14, v16, 0u64, 1u64), Adt6::C0(1u64, 1u64, 0u64, 1u64), 0u64)))
            } else {
                Ok(Adt10::C1)
            }
        }
    }
}

pub fn f11(v0: bool, v1: Adt1, v2: Adt2, v3: Option<Adt2>, v4: Adt2) -> R<Adt10> {
    let m586 = v3;
    match m586 {
        None => {
            Ok(Adt10::C1)
        }
        Some(v5) => {
            let v6: Adt6 = Adt6::C0(match v2.clone() {
                Adt2::C0(_, h587, _, _, _, _, _, _, _, _, _) => {
                    h587
                }
            }, match v1.clone() {
                Adt1::C0(h588, _, _, _, _, _, _, _, _, _, _, _) => {
                    h588
                }
            }, if v0 {
                match v1.clone() {
                    Adt1::C0(_, _, _, h589, _, _, _, _, _, _, _, _) => {
                        h589
                    }
                }
            } else {
                match v1.clone() {
                    Adt1::C0(_, _, h590, _, _, _, _, _, _, _, _, _) => {
                        h590
                    }
                }
            }, if v0 {
                match v1.clone() {
                    Adt1::C0(_, _, _, _, _, _, h591, _, _, _, _, _) => {
                        h591
                    }
                }
            } else {
                match v1.clone() {
                    Adt1::C0(_, _, _, _, h592, _, _, _, _, _, _, _) => {
                        h592
                    }
                }
            });
            let v7: Adt6 = Adt6::C0(match v2.clone() {
                Adt2::C0(_, _, h593, _, _, _, _, _, _, _, _) => {
                    h593
                }
            }, match v1.clone() {
                Adt1::C0(_, h594, _, _, _, _, _, _, _, _, _, _) => {
                    h594
                }
            }, if v0 {
                match v1.clone() {
                    Adt1::C0(_, _, _, _, h595, _, _, _, _, _, _, _) => {
                        h595
                    }
                }
            } else {
                match v1.clone() {
                    Adt1::C0(_, _, _, h596, _, _, _, _, _, _, _, _) => {
                        h596
                    }
                }
            }, if v0 {
                match v1.clone() {
                    Adt1::C0(_, _, _, _, _, _, _, h597, _, _, _, _) => {
                        h597
                    }
                }
            } else {
                match v1.clone() {
                    Adt1::C0(_, _, _, _, _, h598, _, _, _, _, _, _) => {
                        h598
                    }
                }
            });
            let v8: Adt6 = if v0 {
                Adt6::C0(match v2.clone() {
                    Adt2::C0(_, _, _, h599, _, _, _, _, _, _, _) => {
                        h599
                    }
                }, match v1.clone() {
                    Adt1::C0(_, _, h600, _, _, _, _, _, _, _, _, _) => {
                        h600
                    }
                }, match v1.clone() {
                    Adt1::C0(_, _, _, _, _, h601, _, _, _, _, _, _) => {
                        h601
                    }
                }, match v1.clone() {
                    Adt1::C0(_, _, _, _, _, _, _, _, h602, _, _, _) => {
                        h602
                    }
                })
            } else {
                Adt6::C0(1u64, 1u64, 0u64, 1u64)
            };
            let v9: u64 = f46(v6.clone(), match v5.clone() {
                Adt2::C0(_, h603, _, _, _, _, _, _, _, _, _) => {
                    h603
                }
            })?;
            let v10: u64 = f46(v7.clone(), match v5.clone() {
                Adt2::C0(_, _, h604, _, _, _, _, _, _, _, _) => {
                    h604
                }
            })?;
            let v11: u64 = if v0 {
                f46(v8.clone(), match v5.clone() {
                    Adt2::C0(_, _, _, h605, _, _, _, _, _, _, _) => {
                        h605
                    }
                })?
            } else {
                1u64
            };
            let v12: u64 = if v0 {
                match v1.clone() {
                    Adt1::C0(_, _, _, _, _, _, _, _, _, h606, _, _) => {
                        h606
                    }
                }
            } else {
                match v5.clone() {
                    Adt2::C0(_, _, _, h607, _, _, _, _, _, _, _) => {
                        h607
                    }
                }
            };
            let v13: u64 = if v0 {
                match v1.clone() {
                    Adt1::C0(_, _, _, _, _, _, _, _, _, _, h608, _) => {
                        h608
                    }
                }
            } else {
                match v5.clone() {
                    Adt2::C0(_, _, _, _, h609, _, _, _, _, _, _) => {
                        h609
                    }
                }
            };
            let v14: u64 = if v0 {
                match v1 {
                    Adt1::C0(_, _, _, _, _, _, _, _, _, _, _, h610) => {
                        h610
                    }
                }
            } else {
                match v2.clone() {
                    Adt2::C0(_, _, _, _, h611, _, _, _, _, _, _) => {
                        h611
                    }
                }
            };
            let v15: u64 = {
                let a616 = {
                    let a612 = v9;
                    let a613 = v10;
                    nat_mul(a612, a613)?
                };
                let a617 = {
                    let a614 = v11;
                    let a615 = v13;
                    nat_mul(a614, a615)?
                };
                nat_mul(a616, a617)?
            };
            let v16: u64 = {
                let a625 = {
                    let a620 = match v6.clone() {
                        Adt6::C0(h618, _, _, _) => {
                            h618
                        }
                    };
                    let a621 = match v7.clone() {
                        Adt6::C0(h619, _, _, _) => {
                            h619
                        }
                    };
                    nat_mul(a620, a621)?
                };
                let a626 = {
                    let a623 = match v8.clone() {
                        Adt6::C0(h622, _, _, _) => {
                            h622
                        }
                    };
                    let a624 = v12;
                    nat_mul(a623, a624)?
                };
                nat_mul(a625, a626)?
            };
            let v17: u64 = {
                let a627 = v15;
                let a628 = 32u64;
                if nat_lt(a627, a628) {
                    v15
                } else {
                    32u64
                }
            };
            let a630 = match v2.clone() {
                Adt2::C0(h629, _, _, _, _, _, _, _, _, _, _) => {
                    h629
                }
            };
            let a631 = 0u64;
            if if if if if if if if if if if nat_eq(a630, a631) {
                true
            } else {
                let a633 = match v2.clone() {
                    Adt2::C0(h632, _, _, _, _, _, _, _, _, _, _) => {
                        h632
                    }
                };
                let a634 = 1u64;
                nat_eq(a633, a634)
            } {
                let a636 = match v5.clone() {
                    Adt2::C0(h635, _, _, _, _, _, _, _, _, _, _) => {
                        h635
                    }
                };
                let a637 = 0u64;
                nat_eq(a636, a637)
            } else {
                false
            } {
                let a639 = match v4.clone() {
                    Adt2::C0(h638, _, _, _, _, _, _, _, _, _, _) => {
                        h638
                    }
                };
                let a640 = 0u64;
                nat_eq(a639, a640)
            } else {
                false
            } {
                let a642 = match v4.clone() {
                    Adt2::C0(_, h641, _, _, _, _, _, _, _, _, _) => {
                        h641
                    }
                };
                let a643 = v9;
                nat_eq(a642, a643)
            } else {
                false
            } {
                let a645 = match v4.clone() {
                    Adt2::C0(_, _, h644, _, _, _, _, _, _, _, _) => {
                        h644
                    }
                };
                let a646 = v10;
                nat_eq(a645, a646)
            } else {
                false
            } {
                if v0 {
                    let a647 = 0u64;
                    let a648 = v12;
                    if if if if if if if nat_lt(a647, a648) {
                        let a649 = v12;
                        let a650 = 2147483648u64;
                        nat_lt(a649, a650)
                    } else {
                        false
                    } {
                        let a651 = v14;
                        let a652 = 2147483648u64;
                        nat_lt(a651, a652)
                    } else {
                        false
                    } {
                        let a653 = v13;
                        let a654 = 2147483648u64;
                        nat_lt(a653, a654)
                    } else {
                        false
                    } {
                        let a658 = match v2.clone() {
                            Adt2::C0(_, _, _, _, h655, _, _, _, _, _, _) => {
                                h655
                            }
                        };
                        let a659 = {
                            let a656 = v12;
                            let a657 = v14;
                            nat_mul(a656, a657)?
                        };
                        nat_eq(a658, a659)
                    } else {
                        false
                    } {
                        let a663 = match v5.clone() {
                            Adt2::C0(_, _, _, _, h660, _, _, _, _, _, _) => {
                                h660
                            }
                        };
                        let a664 = {
                            let a661 = v12;
                            let a662 = v13;
                            nat_mul(a661, a662)?
                        };
                        nat_eq(a663, a664)
                    } else {
                        false
                    } {
                        let a666 = match v4.clone() {
                            Adt2::C0(_, _, _, h665, _, _, _, _, _, _, _) => {
                                h665
                            }
                        };
                        let a667 = v11;
                        nat_eq(a666, a667)
                    } else {
                        false
                    } {
                        let a671 = match v4.clone() {
                            Adt2::C0(_, _, _, _, h668, _, _, _, _, _, _) => {
                                h668
                            }
                        };
                        let a672 = {
                            let a669 = v14;
                            let a670 = v13;
                            nat_mul(a669, a670)?
                        };
                        nat_eq(a671, a672)
                    } else {
                        false
                    }
                } else {
                    let a674 = match v2.clone() {
                        Adt2::C0(_, _, _, h673, _, _, _, _, _, _, _) => {
                            h673
                        }
                    };
                    let a675 = v12;
                    if if if nat_eq(a674, a675) {
                        let a677 = match v4.clone() {
                            Adt2::C0(_, _, _, h676, _, _, _, _, _, _, _) => {
                                h676
                            }
                        };
                        let a678 = v14;
                        nat_eq(a677, a678)
                    } else {
                        false
                    } {
                        let a680 = match v4.clone() {
                            Adt2::C0(_, _, _, _, h679, _, _, _, _, _, _) => {
                                h679
                            }
                        };
                        let a681 = v13;
                        nat_eq(a680, a681)
                    } else {
                        false
                    } {
                        let a682 = 0u64;
                        let a683 = v16;
                        nat_lt(a682, a683)
                    } else {
                        false
                    }
                }
            } else {
                false
            } {
                let a684 = 0u64;
                let a685 = v14;
                nat_lt(a684, a685)
            } else {
                false
            } {
                let a686 = 0u64;
                let a687 = v15;
                nat_lt(a686, a687)
            } else {
                false
            } {
                f27(v2.clone(), v4.clone())?
            } else {
                false
            } {
                f27(v5.clone(), v4.clone())?
            } else {
                false
            } {
                Ok(Adt10::C6(Adt8::C0(v15, v16, v14, {
                    let a688 = v16;
                    let a689 = v14;
                    nat_mul(a688, a689)?
                }, v17, {
                    let a690 = v17;
                    let a691 = v16;
                    nat_mul(a690, a691)?
                }, {
                    let a692 = v17;
                    let a693 = v14;
                    nat_mul(a692, a693)?
                }, 0u64, f28(v2, 1u64), f28(v5, 1u64), f28(v4, 1u64), v6, v7, v8, if v0 {
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

pub fn f12(v0: Adt1, v1: Adt2, v2: Option<Adt2>, v3: Option<Adt2>, v4: Adt2) -> R<Adt10> {
    let m694 = v2;
    match m694 {
        None => {
            Ok(Adt10::C1)
        }
        Some(v5) => {
            let m695 = v3;
            match m695 {
                None => {
                    Ok(Adt10::C1)
                }
                Some(v6) => {
                    let a697 = match v1.clone() {
                        Adt2::C0(h696, _, _, _, _, _, _, _, _, _, _) => {
                            h696
                        }
                    };
                    let a698 = 0u64;
                    if if if if if if if if if if if if if if if nat_eq(a697, a698) {
                        let a700 = match v5.clone() {
                            Adt2::C0(h699, _, _, _, _, _, _, _, _, _, _) => {
                                h699
                            }
                        };
                        let a701 = 0u64;
                        nat_eq(a700, a701)
                    } else {
                        false
                    } {
                        let a703 = match v6.clone() {
                            Adt2::C0(h702, _, _, _, _, _, _, _, _, _, _) => {
                                h702
                            }
                        };
                        let a704 = 0u64;
                        nat_eq(a703, a704)
                    } else {
                        false
                    } {
                        let a706 = match v4.clone() {
                            Adt2::C0(h705, _, _, _, _, _, _, _, _, _, _) => {
                                h705
                            }
                        };
                        let a707 = 0u64;
                        nat_eq(a706, a707)
                    } else {
                        false
                    } {
                        f32(v5.clone(), v6.clone())
                    } else {
                        false
                    } {
                        f32(v1.clone(), v4.clone())
                    } else {
                        false
                    } {
                        f45(v1.clone())?
                    } else {
                        false
                    } {
                        f45(v5.clone())?
                    } else {
                        false
                    } {
                        f45(v6)?
                    } else {
                        false
                    } {
                        f45(v4.clone())?
                    } else {
                        false
                    } {
                        let a713 = match v1.clone() {
                            Adt2::C0(_, h708, _, _, _, _, _, _, _, _, _) => {
                                h708
                            }
                        };
                        let a714 = {
                            let a711 = match v5.clone() {
                                Adt2::C0(_, h709, _, _, _, _, _, _, _, _, _) => {
                                    h709
                                }
                            };
                            let a712 = match v5.clone() {
                                Adt2::C0(_, h710, _, _, _, _, _, _, _, _, _) => {
                                    h710
                                }
                            };
                            nat_mul(a711, a712)?
                        };
                        nat_eq(a713, a714)
                    } else {
                        false
                    } {
                        let a720 = match v1.clone() {
                            Adt2::C0(_, _, h715, _, _, _, _, _, _, _, _) => {
                                h715
                            }
                        };
                        let a721 = {
                            let a718 = match v5.clone() {
                                Adt2::C0(_, _, h716, _, _, _, _, _, _, _, _) => {
                                    h716
                                }
                            };
                            let a719 = match v5.clone() {
                                Adt2::C0(_, _, _, h717, _, _, _, _, _, _, _) => {
                                    h717
                                }
                            };
                            nat_mul(a718, a719)?
                        };
                        nat_eq(a720, a721)
                    } else {
                        false
                    } {
                        let a724 = match v1.clone() {
                            Adt2::C0(_, _, _, h722, _, _, _, _, _, _, _) => {
                                h722
                            }
                        };
                        let a725 = match v5.clone() {
                            Adt2::C0(_, _, _, _, h723, _, _, _, _, _, _) => {
                                h723
                            }
                        };
                        nat_eq(a724, a725)
                    } else {
                        false
                    } {
                        let a727 = match v0.clone() {
                            Adt1::C0(h726, _, _, _, _, _, _, _, _, _, _, _) => {
                                h726
                            }
                        };
                        let a728 = 4294967296u64;
                        nat_lt(a727, a728)
                    } else {
                        false
                    } {
                        let a730 = match v0.clone() {
                            Adt1::C0(h729, _, _, _, _, _, _, _, _, _, _, _) => {
                                h729
                            }
                        };
                        let a731 = 0u64;
                        if nat_eq(a730, a731) {
                            if f27(v1.clone(), v4.clone())? {
                                true
                            } else {
                                let a734 = match v1.clone() {
                                    Adt2::C0(_, _, _, _, _, _, _, _, _, _, h732) => {
                                        h732
                                    }
                                };
                                let a735 = match v4.clone() {
                                    Adt2::C0(_, _, _, _, _, _, _, _, _, _, h733) => {
                                        h733
                                    }
                                };
                                nat_eq(a734, a735)
                            }
                        } else {
                            let a738 = match v1.clone() {
                                Adt2::C0(_, _, _, _, _, _, _, _, _, _, h736) => {
                                    h736
                                }
                            };
                            let a739 = match v4.clone() {
                                Adt2::C0(_, _, _, _, _, _, _, _, _, _, h737) => {
                                    h737
                                }
                            };
                            nat_eq(a738, a739)
                        }
                    } else {
                        false
                    } {
                        let v7: bool = {
                            let a741 = match v4.clone() {
                                Adt2::C0(_, h740, _, _, _, _, _, _, _, _, _) => {
                                    h740
                                }
                            };
                            let a742 = 0u64;
                            if if if nat_eq(a741, a742) {
                                true
                            } else {
                                let a744 = match v4.clone() {
                                    Adt2::C0(_, _, h743, _, _, _, _, _, _, _, _) => {
                                        h743
                                    }
                                };
                                let a745 = 0u64;
                                nat_eq(a744, a745)
                            } {
                                true
                            } else {
                                let a747 = match v4.clone() {
                                    Adt2::C0(_, _, _, h746, _, _, _, _, _, _, _) => {
                                        h746
                                    }
                                };
                                let a748 = 0u64;
                                nat_eq(a747, a748)
                            } {
                                true
                            } else {
                                let a750 = match v4.clone() {
                                    Adt2::C0(_, _, _, _, h749, _, _, _, _, _, _) => {
                                        h749
                                    }
                                };
                                let a751 = 0u64;
                                nat_eq(a750, a751)
                            }
                        };
                        let v8: u64 = match v5.clone() {
                            Adt2::C0(_, h752, _, _, _, _, _, _, _, _, _) => {
                                h752
                            }
                        };
                        let v9: u64 = if v7 {
                            0u64
                        } else {
                            let a754 = match v0 {
                                Adt1::C0(h753, _, _, _, _, _, _, _, _, _, _, _) => {
                                    h753
                                }
                            };
                            let a755 = 0u64;
                            if nat_eq(a754, a755) {
                                f25(v1.clone(), 1u64)?
                            } else {
                                0u64
                            }
                        };
                        let v10: u64 = if v7 {
                            0u64
                        } else {
                            f47(v8, match v5.clone() {
                                Adt2::C0(_, _, h756, _, _, _, _, _, _, _, _) => {
                                    h756
                                }
                            }, match v5.clone() {
                                Adt2::C0(_, _, _, h757, _, _, _, _, _, _, _) => {
                                    h757
                                }
                            }, match v5 {
                                Adt2::C0(_, _, _, _, h758, _, _, _, _, _, _) => {
                                    h758
                                }
                            })?
                        };
                        Ok(Adt10::C4(f48(v9, v8, v10)?, 4u64, f28(v1, 1u64), f28(v4, 1u64), Adt4::C25(v9, v8, v10)))
                    } else {
                        Ok(Adt10::C1)
                    }
                }
            }
        }
    }
}

pub fn f13(v0: u64, v1: Adt1, v2: Adt2, v3: Option<Adt2>, v4: Adt2) -> R<Adt10> {
    let v5: Adt14 = f49(v2.clone());
    let v6: Adt14 = f49(v4.clone());
    let v8: Adt15 = {
        let m759 = v3;
        match m759 {
            None => {
                f50(v0, match v1.clone() {
                    Adt1::C0(h760, _, _, _, _, _, _, _, _, _, _, _) => {
                        h760
                    }
                }, match v1 {
                    Adt1::C0(_, h761, _, _, _, _, _, _, _, _, _, _) => {
                        h761
                    }
                }, false, v5.clone(), v5, v6)?
            }
            Some(v7) => {
                f50(v0, match v1.clone() {
                    Adt1::C0(h762, _, _, _, _, _, _, _, _, _, _, _) => {
                        h762
                    }
                }, match v1 {
                    Adt1::C0(_, h763, _, _, _, _, _, _, _, _, _, _) => {
                        h763
                    }
                }, true, v5, f49(v7), v6)?
            }
        }
    };
    let m764 = v8;
    match m764 {
        Adt15::C0 => {
            Ok(Adt10::C1)
        }
        Adt15::C1(v9, v10, v11, v12, v13, v14) => {
            Ok(Adt10::C4(v9, v10, f28(v2, 1u64), f28(v4, 1u64), Adt4::C26(v11, v14, v12, v13)))
        }
    }
}

pub fn f14(v0: Adt1, v1: Adt2, v2: Adt2) -> R<Adt10> {
    let v3: u64 = match v1.clone() {
        Adt2::C0(h765, _, _, _, _, _, _, _, _, _, _) => {
            h765
        }
    };
    let v4: u64 = f51(v3);
    let a767 = match v1.clone() {
        Adt2::C0(h766, _, _, _, _, _, _, _, _, _, _) => {
            h766
        }
    };
    let a768 = 0u64;
    if if if if if if if if nat_eq(a767, a768) {
        true
    } else {
        let a770 = match v1.clone() {
            Adt2::C0(h769, _, _, _, _, _, _, _, _, _, _) => {
                h769
            }
        };
        let a771 = 30u64;
        nat_eq(a770, a771)
    } {
        let a774 = match v1.clone() {
            Adt2::C0(h772, _, _, _, _, _, _, _, _, _, _) => {
                h772
            }
        };
        let a775 = match v2.clone() {
            Adt2::C0(h773, _, _, _, _, _, _, _, _, _, _) => {
                h773
            }
        };
        nat_eq(a774, a775)
    } else {
        false
    } {
        f32(v1.clone(), v2.clone())
    } else {
        false
    } {
        f52(match v0.clone() {
            Adt1::C0(h776, _, _, _, _, _, _, _, _, _, _, _) => {
                h776
            }
        })
    } else {
        false
    } {
        f53(v1.clone(), v4)
    } else {
        false
    } {
        f53(v2.clone(), v4)
    } else {
        false
    } {
        if f27(v1.clone(), v2.clone())? {
            true
        } else {
            f54(v1.clone(), v2.clone())
        }
    } else {
        false
    } {
        let v5: u64 = if f55(match v1.clone() {
            Adt2::C0(_, h777, _, _, _, _, _, _, _, _, _) => {
                h777
            }
        }, match v1.clone() {
            Adt2::C0(_, _, h778, _, _, _, _, _, _, _, _) => {
                h778
            }
        }, match v1.clone() {
            Adt2::C0(_, _, _, h779, _, _, _, _, _, _, _) => {
                h779
            }
        }, match v1.clone() {
            Adt2::C0(_, _, _, _, h780, _, _, _, _, _, _) => {
                h780
            }
        }) {
            0u64
        } else {
            let a786 = match v1.clone() {
                Adt2::C0(_, _, h781, _, _, _, _, _, _, _, _) => {
                    h781
                }
            };
            let a787 = {
                let a784 = match v1.clone() {
                    Adt2::C0(_, _, _, h782, _, _, _, _, _, _, _) => {
                        h782
                    }
                };
                let a785 = match v1.clone() {
                    Adt2::C0(_, _, _, _, h783, _, _, _, _, _, _) => {
                        h783
                    }
                };
                nat_mul(a784, a785)?
            };
            nat_mul(a786, a787)?
        };
        let v6: u64 = {
            let a788 = v5;
            let a789 = 0u64;
            if if nat_eq(a788, a789) {
                true
            } else {
                f56(v2.clone(), v4)?
            } {
                0u64
            } else {
                let a791 = match v2.clone() {
                    Adt2::C0(_, _, _, _, _, _, _, _, _, h790, _) => {
                        h790
                    }
                };
                let a792 = v4;
                let a793 = 0u64;
                nat_quot(a791, a792, a793)
            }
        };
        Ok(Adt10::C4(f57(v3, match v1.clone() {
            Adt2::C0(_, h794, _, _, _, _, _, _, _, _, _) => {
                h794
            }
        }, v5, v6)?, v4, f28(v1.clone(), 1u64), f28(v2, 1u64), Adt4::C26(v3, match v0 {
            Adt1::C0(h795, _, _, _, _, _, _, _, _, _, _, _) => {
                h795
            }
        }, v5, f58(match v1 {
            Adt2::C0(_, h796, _, _, _, _, _, _, _, _, _) => {
                h796
            }
        }, v5, v6)?)))
    } else {
        Ok(Adt10::C1)
    }
}

pub fn f15(v0: Adt1, v1: Adt2, v2: Adt2) -> R<Adt10> {
    let a798 = match v1.clone() {
        Adt2::C0(h797, _, _, _, _, _, _, _, _, _, _) => {
            h797
        }
    };
    let a799 = 0u64;
    if if if if if nat_eq(a798, a799) {
        let a801 = match v2.clone() {
            Adt2::C0(h800, _, _, _, _, _, _, _, _, _, _) => {
                h800
            }
        };
        let a802 = 0u64;
        nat_eq(a801, a802)
    } else {
        false
    } {
        f32(v1.clone(), v2.clone())
    } else {
        false
    } {
        f59(match v0.clone() {
            Adt1::C0(h803, _, _, _, _, _, _, _, _, _, _, _) => {
                h803
            }
        })
    } else {
        false
    } {
        if f27(v1.clone(), v2.clone())? {
            true
        } else {
            f54(v1.clone(), v2.clone())
        }
    } else {
        false
    } {
        let v3: Adt3 = f28(v1, 1u64);
        let v4: u64 = {
            let a809 = match v3.clone() {
                Adt3::C0(_, h804, _, _, _, _, _, _) => {
                    h804
                }
            };
            let a810 = {
                let a807 = match v3.clone() {
                    Adt3::C0(_, _, h805, _, _, _, _, _) => {
                        h805
                    }
                };
                let a808 = match v3.clone() {
                    Adt3::C0(_, _, _, h806, _, _, _, _) => {
                        h806
                    }
                };
                nat_mul(a807, a808)?
            };
            nat_mul(a809, a810)?
        };
        Ok(Adt10::C4(f60(match v3.clone() {
            Adt3::C0(h811, _, _, _, _, _, _, _) => {
                h811
            }
        }, v4)?, 4u64, v3, f28(v2, 1u64), Adt4::C24(match v0 {
            Adt1::C0(h812, _, _, _, _, _, _, _, _, _, _, _) => {
                h812
            }
        })))
    } else {
        Ok(Adt10::C1)
    }
}

pub fn f16(v0: Adt2, v1: Option<Adt2>, v2: Option<Adt2>, v3: Adt2) -> R<Adt10> {
    let m813 = v1;
    match m813 {
        None => {
            Ok(Adt10::C1)
        }
        Some(v4) => {
            let m814 = v2;
            match m814 {
                None => {
                    Ok(Adt10::C1)
                }
                Some(v5) => {
                    let a816 = match v0.clone() {
                        Adt2::C0(h815, _, _, _, _, _, _, _, _, _, _) => {
                            h815
                        }
                    };
                    let a817 = 0u64;
                    if if if if if if if if if if if if if if if if if nat_eq(a816, a817) {
                        let a819 = match v4.clone() {
                            Adt2::C0(h818, _, _, _, _, _, _, _, _, _, _) => {
                                h818
                            }
                        };
                        let a820 = 0u64;
                        nat_eq(a819, a820)
                    } else {
                        false
                    } {
                        let a822 = match v5.clone() {
                            Adt2::C0(h821, _, _, _, _, _, _, _, _, _, _) => {
                                h821
                            }
                        };
                        let a823 = 26u64;
                        nat_eq(a822, a823)
                    } else {
                        false
                    } {
                        let a825 = match v3.clone() {
                            Adt2::C0(h824, _, _, _, _, _, _, _, _, _, _) => {
                                h824
                            }
                        };
                        let a826 = 0u64;
                        nat_eq(a825, a826)
                    } else {
                        false
                    } {
                        f32(v0.clone(), v3.clone())
                    } else {
                        false
                    } {
                        let a829 = match v0.clone() {
                            Adt2::C0(_, h827, _, _, _, _, _, _, _, _, _) => {
                                h827
                            }
                        };
                        let a830 = match v4.clone() {
                            Adt2::C0(_, h828, _, _, _, _, _, _, _, _, _) => {
                                h828
                            }
                        };
                        nat_eq(a829, a830)
                    } else {
                        false
                    } {
                        let a833 = match v0.clone() {
                            Adt2::C0(_, _, h831, _, _, _, _, _, _, _, _) => {
                                h831
                            }
                        };
                        let a834 = match v5.clone() {
                            Adt2::C0(_, h832, _, _, _, _, _, _, _, _, _) => {
                                h832
                            }
                        };
                        nat_eq(a833, a834)
                    } else {
                        false
                    } {
                        let a837 = match v0.clone() {
                            Adt2::C0(_, _, _, h835, _, _, _, _, _, _, _) => {
                                h835
                            }
                        };
                        let a838 = match v5.clone() {
                            Adt2::C0(_, _, h836, _, _, _, _, _, _, _, _) => {
                                h836
                            }
                        };
                        nat_eq(a837, a838)
                    } else {
                        false
                    } {
                        let a840 = 0u64;
                        let a841 = match v5.clone() {
                            Adt2::C0(_, _, _, h839, _, _, _, _, _, _, _) => {
                                h839
                            }
                        };
                        nat_lt(a840, a841)
                    } else {
                        false
                    } {
                        let a843 = 0u64;
                        let a844 = match v5.clone() {
                            Adt2::C0(_, _, _, _, h842, _, _, _, _, _, _) => {
                                h842
                            }
                        };
                        nat_lt(a843, a844)
                    } else {
                        false
                    } {
                        let a846 = 4u64;
                        let a847 = match v0.clone() {
                            Adt2::C0(_, _, _, _, _, h845, _, _, _, _, _) => {
                                h845
                            }
                        };
                        nat_le(a846, a847)
                    } else {
                        false
                    } {
                        let a849 = 4u64;
                        let a850 = match v4.clone() {
                            Adt2::C0(_, _, _, _, _, h848, _, _, _, _, _) => {
                                h848
                            }
                        };
                        nat_le(a849, a850)
                    } else {
                        false
                    } {
                        let a852 = 4u64;
                        let a853 = match v5.clone() {
                            Adt2::C0(_, _, _, _, _, h851, _, _, _, _, _) => {
                                h851
                            }
                        };
                        nat_le(a852, a853)
                    } else {
                        false
                    } {
                        let a855 = 4u64;
                        let a856 = match v3.clone() {
                            Adt2::C0(_, _, _, _, _, h854, _, _, _, _, _) => {
                                h854
                            }
                        };
                        nat_le(a855, a856)
                    } else {
                        false
                    } {
                        f27(v0.clone(), v3.clone())?
                    } else {
                        false
                    } {
                        f27(v4.clone(), v3.clone())?
                    } else {
                        false
                    } {
                        f27(v5.clone(), v3.clone())?
                    } else {
                        false
                    } {
                        Ok(Adt10::C4(f61(match v0.clone() {
                            Adt2::C0(_, h857, _, _, _, _, _, _, _, _, _) => {
                                h857
                            }
                        }, match v0.clone() {
                            Adt2::C0(_, _, h858, _, _, _, _, _, _, _, _) => {
                                h858
                            }
                        }, match v0.clone() {
                            Adt2::C0(_, _, _, h859, _, _, _, _, _, _, _) => {
                                h859
                            }
                        }, match v0.clone() {
                            Adt2::C0(_, _, _, _, h860, _, _, _, _, _, _) => {
                                h860
                            }
                        })?, 4u64, f28(v0, 1u64), f28(v3, 1u64), Adt4::C22(f28(v4, 1u64), f28(v5, 1u64))))
                    } else {
                        Ok(Adt10::C1)
                    }
                }
            }
        }
    }
}

pub fn f17(v0: u64, v1: Adt1, v2: Adt2, v3: Option<Adt2>, v4: Adt2) -> R<Adt9> {
    let a861 = v0;
    let a862 = 28u64;
    if if if nat_eq(a861, a862) {
        true
    } else {
        let a863 = v0;
        let a864 = 29u64;
        nat_eq(a863, a864)
    } {
        let a866 = 0u64;
        let a867 = match v1.clone() {
            Adt1::C0(h865, _, _, _, _, _, _, _, _, _, _, _) => {
                h865
            }
        };
        if nat_lt(a866, a867) {
            true
        } else {
            let a869 = 0u64;
            let a870 = match v1.clone() {
                Adt1::C0(_, _, _, h868, _, _, _, _, _, _, _, _) => {
                    h868
                }
            };
            nat_lt(a869, a870)
        }
    } else {
        false
    } {
        let m874 = f62(match v2.clone() {
            Adt2::C0(h871, _, _, _, _, _, _, _, _, _, _) => {
                h871
            }
        }, match v1.clone() {
            Adt1::C0(h872, _, _, _, _, _, _, _, _, _, _, _) => {
                h872
            }
        }, match v1.clone() {
            Adt1::C0(_, _, _, h873, _, _, _, _, _, _, _, _) => {
                h873
            }
        });
        match m874 {
            None => {
                Ok(Adt9::C0(Adt10::C8))
            }
            Some(v5) => {
                if v5 {
                    Ok(Adt9::C0(Adt10::C7))
                } else {
                    f63(v0, v1, v2, v3, v4)
                }
            }
        }
    } else {
        f63(v0, v1, v2, v3, v4)
    }
}

pub fn f18(v0: Adt8, v1: u64, v2: u64, v3: u64) -> R<Adt9> {
    let v4: u64 = match v0.clone() {
        Adt8::C0(_, h875, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h875
        }
    };
    let v5: u64 = match v0.clone() {
        Adt8::C0(_, _, h876, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h876
        }
    };
    let v6: u64 = {
        let a877 = v3;
        let a878 = v5;
        let a879 = 0u64;
        nat_rem(a877, a878, a879)
    };
    let a880 = v1;
    let a881 = 0u64;
    if nat_eq(a880, a881) {
        let a883 = v3;
        let a884 = match v0.clone() {
            Adt8::C0(_, _, _, h882, _, _, _, _, _, _, _, _, _, _, _) => {
                h882
            }
        };
        if nat_lt(a883, a884) {
            Ok(Adt9::C8(Some(f64(match v0.clone() {
                Adt8::C0(_, _, _, _, _, _, _, _, h885, _, _, _, _, _, _) => {
                    h885
                }
            }, {
                let a889 = v6;
                let a890 = {
                    let m888 = match v0.clone() {
                        Adt8::C0(_, _, _, _, _, _, _, _, h887, _, _, _, _, _, _) => {
                            h887
                        }
                    };
                    match m888 {
                        Adt3::C0(h886, _, _, _, _, _, _, _) => {
                            h886
                        }
                    }
                };
                let a891 = 0u64;
                nat_rem(a889, a890, a891)
            }, {
                let a901 = {
                    let a895 = v6;
                    let a896 = {
                        let m894 = match v0.clone() {
                            Adt8::C0(_, _, _, _, _, _, _, _, h893, _, _, _, _, _, _) => {
                                h893
                            }
                        };
                        match m894 {
                            Adt3::C0(h892, _, _, _, _, _, _, _) => {
                                h892
                            }
                        }
                    };
                    let a897 = 0u64;
                    nat_quot(a895, a896, a897)
                };
                let a902 = {
                    let m900 = match v0.clone() {
                        Adt8::C0(_, _, _, _, _, _, _, _, h899, _, _, _, _, _, _) => {
                            h899
                        }
                    };
                    match m900 {
                        Adt3::C0(_, h898, _, _, _, _, _, _) => {
                            h898
                        }
                    }
                };
                let a903 = 0u64;
                nat_rem(a901, a902, a903)
            }, {
                let a913 = {
                    let a907 = v6;
                    let a908 = {
                        let m906 = match v0.clone() {
                            Adt8::C0(_, _, _, _, _, _, _, _, h905, _, _, _, _, _, _) => {
                                h905
                            }
                        };
                        match m906 {
                            Adt3::C0(h904, _, _, _, _, _, _, _) => {
                                h904
                            }
                        }
                    };
                    let a909 = 0u64;
                    nat_quot(a907, a908, a909)
                };
                let a914 = {
                    let m912 = match v0 {
                        Adt8::C0(_, _, _, _, _, _, _, _, h911, _, _, _, _, _, _) => {
                            h911
                        }
                    };
                    match m912 {
                        Adt3::C0(_, h910, _, _, _, _, _, _) => {
                            h910
                        }
                    }
                };
                let a915 = 0u64;
                nat_quot(a913, a914, a915)
            }, {
                let a916 = v3;
                let a917 = v5;
                let a918 = 0u64;
                nat_quot(a916, a917, a918)
            })?)))
        } else {
            Ok(Adt9::C0(Adt10::C1))
        }
    } else {
        let v7: u64 = {
            let a924 = v2;
            let a925 = {
                let a921 = v3;
                let a922 = {
                    let a919 = v1;
                    let a920 = 1u64;
                    if nat_eq(a919, a920) {
                        v4
                    } else {
                        v5
                    }
                };
                let a923 = 0u64;
                nat_quot(a921, a922, a923)
            };
            nat_add(a924, a925)?
        };
        let a927 = v7;
        let a928 = match v0.clone() {
            Adt8::C0(h926, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h926
            }
        };
        if nat_lt(a927, a928) {
            let v8: u64 = {
                let a932 = v7;
                let a933 = {
                    let m931 = match v0.clone() {
                        Adt8::C0(_, _, _, _, _, _, _, _, _, h930, _, _, _, _, _) => {
                            h930
                        }
                    };
                    match m931 {
                        Adt3::C0(h929, _, _, _, _, _, _, _) => {
                            h929
                        }
                    }
                };
                let a934 = 0u64;
                nat_rem(a932, a933, a934)
            };
            let v9: u64 = {
                let a938 = v7;
                let a939 = {
                    let m937 = match v0.clone() {
                        Adt8::C0(_, _, _, _, _, _, _, _, _, h936, _, _, _, _, _) => {
                            h936
                        }
                    };
                    match m937 {
                        Adt3::C0(h935, _, _, _, _, _, _, _) => {
                            h935
                        }
                    }
                };
                let a940 = 0u64;
                nat_quot(a938, a939, a940)
            };
            let v10: u64 = {
                let a944 = v9;
                let a945 = {
                    let m943 = match v0.clone() {
                        Adt8::C0(_, _, _, _, _, _, _, _, _, h942, _, _, _, _, _) => {
                            h942
                        }
                    };
                    match m943 {
                        Adt3::C0(_, h941, _, _, _, _, _, _) => {
                            h941
                        }
                    }
                };
                let a946 = 0u64;
                nat_rem(a944, a945, a946)
            };
            let v11: u64 = {
                let a950 = v9;
                let a951 = {
                    let m949 = match v0.clone() {
                        Adt8::C0(_, _, _, _, _, _, _, _, _, h948, _, _, _, _, _) => {
                            h948
                        }
                    };
                    match m949 {
                        Adt3::C0(_, h947, _, _, _, _, _, _) => {
                            h947
                        }
                    }
                };
                let a952 = 0u64;
                nat_quot(a950, a951, a952)
            };
            let a953 = v1;
            let a954 = 1u64;
            if nat_eq(a953, a954) {
                Ok(Adt9::C8(Some(f64(match v0 {
                    Adt8::C0(_, _, _, _, _, _, _, _, _, h955, _, _, _, _, _) => {
                        h955
                    }
                }, v8, v10, {
                    let a956 = v3;
                    let a957 = v4;
                    let a958 = 0u64;
                    nat_rem(a956, a957, a958)
                }, v11)?)))
            } else {
                let a959 = v1;
                let a960 = 2u64;
                if nat_eq(a959, a960) {
                    Ok(Adt9::C8(Some({
                        let a1006 = f64(match v0.clone() {
                            Adt8::C0(_, _, _, _, _, _, _, _, _, _, h961, _, _, _, _) => {
                                h961
                            }
                        }, {
                            let a973 = {
                                let a965 = v8;
                                let a966 = {
                                    let m964 = match v0.clone() {
                                        Adt8::C0(_, _, _, _, _, _, _, _, _, _, _, h963, _, _, _) => {
                                            h963
                                        }
                                    };
                                    match m964 {
                                        Adt6::C0(_, h962, _, _) => {
                                            h962
                                        }
                                    }
                                };
                                nat_mul(a965, a966)?
                            };
                            let a974 = {
                                let a970 = v6;
                                let a971 = {
                                    let m969 = match v0.clone() {
                                        Adt8::C0(_, _, _, _, _, _, _, _, h968, _, _, _, _, _, _) => {
                                            h968
                                        }
                                    };
                                    match m969 {
                                        Adt3::C0(h967, _, _, _, _, _, _, _) => {
                                            h967
                                        }
                                    }
                                };
                                let a972 = 0u64;
                                nat_rem(a970, a971, a972)
                            };
                            nat_add(a973, a974)?
                        }, {
                            let a992 = {
                                let a978 = v10;
                                let a979 = {
                                    let m977 = match v0.clone() {
                                        Adt8::C0(_, _, _, _, _, _, _, _, _, _, _, h976, _, _, _) => {
                                            h976
                                        }
                                    };
                                    match m977 {
                                        Adt6::C0(_, h975, _, _) => {
                                            h975
                                        }
                                    }
                                };
                                nat_mul(a978, a979)?
                            };
                            let a993 = {
                                let a989 = {
                                    let a983 = v6;
                                    let a984 = {
                                        let m982 = match v0.clone() {
                                            Adt8::C0(_, _, _, _, _, _, _, _, h981, _, _, _, _, _, _) => {
                                                h981
                                            }
                                        };
                                        match m982 {
                                            Adt3::C0(h980, _, _, _, _, _, _, _) => {
                                                h980
                                            }
                                        }
                                    };
                                    let a985 = 0u64;
                                    nat_quot(a983, a984, a985)
                                };
                                let a990 = {
                                    let m988 = match v0.clone() {
                                        Adt8::C0(_, _, _, _, _, _, _, _, h987, _, _, _, _, _, _) => {
                                            h987
                                        }
                                    };
                                    match m988 {
                                        Adt3::C0(_, h986, _, _, _, _, _, _) => {
                                            h986
                                        }
                                    }
                                };
                                let a991 = 0u64;
                                nat_rem(a989, a990, a991)
                            };
                            nat_add(a992, a993)?
                        }, {
                            let a1003 = {
                                let a997 = v6;
                                let a998 = {
                                    let m996 = match v0.clone() {
                                        Adt8::C0(_, _, _, _, _, _, _, _, h995, _, _, _, _, _, _) => {
                                            h995
                                        }
                                    };
                                    match m996 {
                                        Adt3::C0(h994, _, _, _, _, _, _, _) => {
                                            h994
                                        }
                                    }
                                };
                                let a999 = 0u64;
                                nat_quot(a997, a998, a999)
                            };
                            let a1004 = {
                                let m1002 = match v0 {
                                    Adt8::C0(_, _, _, _, _, _, _, _, h1001, _, _, _, _, _, _) => {
                                        h1001
                                    }
                                };
                                match m1002 {
                                    Adt3::C0(_, h1000, _, _, _, _, _, _) => {
                                        h1000
                                    }
                                }
                            };
                            let a1005 = 0u64;
                            nat_quot(a1003, a1004, a1005)
                        }, v11)?;
                        let a1007 = 4u64;
                        let a1008 = 0u64;
                        nat_quot(a1006, a1007, a1008)
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

pub fn f19(v0: Adt8, v1: u64, v2: u64, v3: u64) -> R<Adt9> {
    let a1009 = v1;
    let a1010 = 0u64;
    if nat_eq(a1009, a1010) {
        let a1012 = v3;
        let a1013 = match v0.clone() {
            Adt8::C0(_, _, _, h1011, _, _, _, _, _, _, _, _, _, _, _) => {
                h1011
            }
        };
        if nat_lt(a1012, a1013) {
            let v4: Adt3 = match v0.clone() {
                Adt8::C0(_, _, _, _, _, _, _, _, h1014, _, _, _, _, _, _) => {
                    h1014
                }
            };
            let v5: u64 = {
                let a1016 = v3;
                let a1017 = match v0.clone() {
                    Adt8::C0(_, _, h1015, _, _, _, _, _, _, _, _, _, _, _, _) => {
                        h1015
                    }
                };
                let a1018 = 0u64;
                nat_quot(a1016, a1017, a1018)
            };
            Ok(Adt9::C8(Some(f64(v4.clone(), f65(v5, v4.clone(), 0u64), f65(v5, v4.clone(), 1u64), f65(v5, v4, 2u64), f66(v0.clone(), v5, {
                let a1020 = v3;
                let a1021 = match v0 {
                    Adt8::C0(_, _, h1019, _, _, _, _, _, _, _, _, _, _, _, _) => {
                        h1019
                    }
                };
                let a1022 = 0u64;
                nat_rem(a1020, a1021, a1022)
            })?)?)))
        } else {
            Ok(Adt9::C0(Adt10::C1))
        }
    } else {
        let v6: Adt3 = match v0.clone() {
            Adt8::C0(_, _, _, _, _, _, _, _, _, _, h1023, _, _, _, _) => {
                h1023
            }
        };
        let v7: u64 = {
            let a1024 = v1;
            let a1025 = 1u64;
            if nat_eq(a1024, a1025) {
                match v0.clone() {
                    Adt8::C0(_, h1026, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                        h1026
                    }
                }
            } else {
                match v0.clone() {
                    Adt8::C0(_, _, h1027, _, _, _, _, _, _, _, _, _, _, _, _) => {
                        h1027
                    }
                }
            }
        };
        let v8: u64 = {
            let a1031 = v2;
            let a1032 = {
                let a1028 = v3;
                let a1029 = v7;
                let a1030 = 0u64;
                nat_quot(a1028, a1029, a1030)
            };
            nat_add(a1031, a1032)?
        };
        let v9: u64 = {
            let a1033 = v3;
            let a1034 = v7;
            let a1035 = 0u64;
            nat_rem(a1033, a1034, a1035)
        };
        let v10: u64 = {
            let a1037 = v8;
            let a1038 = match v6.clone() {
                Adt3::C0(h1036, _, _, _, _, _, _, _) => {
                    h1036
                }
            };
            let a1039 = 0u64;
            nat_rem(a1037, a1038, a1039)
        };
        let v11: u64 = {
            let a1041 = v8;
            let a1042 = match v6.clone() {
                Adt3::C0(h1040, _, _, _, _, _, _, _) => {
                    h1040
                }
            };
            let a1043 = 0u64;
            nat_quot(a1041, a1042, a1043)
        };
        let v12: u64 = {
            let a1045 = v11;
            let a1046 = match v6.clone() {
                Adt3::C0(_, h1044, _, _, _, _, _, _) => {
                    h1044
                }
            };
            let a1047 = 0u64;
            nat_rem(a1045, a1046, a1047)
        };
        let v13: u64 = {
            let a1049 = v11;
            let a1050 = match v6.clone() {
                Adt3::C0(_, h1048, _, _, _, _, _, _) => {
                    h1048
                }
            };
            let a1051 = 0u64;
            nat_quot(a1049, a1050, a1051)
        };
        let a1053 = v8;
        let a1054 = match v0.clone() {
            Adt8::C0(h1052, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h1052
            }
        };
        if nat_lt(a1053, a1054) {
            let a1055 = v1;
            let a1056 = 1u64;
            if nat_eq(a1055, a1056) {
                let v14: Adt3 = match v0.clone() {
                    Adt8::C0(_, _, _, _, _, _, _, _, h1057, _, _, _, _, _, _) => {
                        h1057
                    }
                };
                let v15: Adt3 = match v0.clone() {
                    Adt8::C0(_, _, _, _, _, _, _, _, _, h1058, _, _, _, _, _) => {
                        h1058
                    }
                };
                let m1061 = f67(match v0.clone() {
                    Adt8::C0(_, _, _, _, _, _, _, _, _, _, _, h1059, _, _, _) => {
                        h1059
                    }
                }, match v15.clone() {
                    Adt3::C0(h1060, _, _, _, _, _, _, _) => {
                        h1060
                    }
                }, v10, f65(v9, v14.clone(), 0u64))?;
                match m1061 {
                    None => {
                        Ok(Adt9::C8(None::<u64>))
                    }
                    Some(v16) => {
                        let m1064 = f67(match v0.clone() {
                            Adt8::C0(_, _, _, _, _, _, _, _, _, _, _, _, h1062, _, _) => {
                                h1062
                            }
                        }, match v15 {
                            Adt3::C0(_, h1063, _, _, _, _, _, _) => {
                                h1063
                            }
                        }, v12, f65(v9, v14, 1u64))?;
                        match m1064 {
                            None => {
                                Ok(Adt9::C8(None::<u64>))
                            }
                            Some(v17) => {
                                f68(v0, v9, v13, v16, v17)
                            }
                        }
                    }
                }
            } else {
                let a1065 = v1;
                let a1066 = 2u64;
                if nat_eq(a1065, a1066) {
                    Ok(Adt9::C8(Some({
                        let a1068 = match v0.clone() {
                            Adt8::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, h1067) => {
                                h1067
                            }
                        };
                        let a1069 = 0u64;
                        if nat_eq(a1068, a1069) {
                            f64(v6, v10, v12, v9, v13)?
                        } else {
                            f64(v6.clone(), v10, v12, {
                                let a1071 = v13;
                                let a1072 = match v6.clone() {
                                    Adt3::C0(_, _, h1070, _, _, _, _, _) => {
                                        h1070
                                    }
                                };
                                let a1073 = 0u64;
                                nat_rem(a1071, a1072, a1073)
                            }, {
                                let a1081 = {
                                    let a1079 = {
                                        let a1075 = v13;
                                        let a1076 = match v6 {
                                            Adt3::C0(_, _, h1074, _, _, _, _, _) => {
                                                h1074
                                            }
                                        };
                                        let a1077 = 0u64;
                                        nat_quot(a1075, a1076, a1077)
                                    };
                                    let a1080 = match v0 {
                                        Adt8::C0(_, _, h1078, _, _, _, _, _, _, _, _, _, _, _, _) => {
                                            h1078
                                        }
                                    };
                                    nat_mul(a1079, a1080)?
                                };
                                let a1082 = v9;
                                nat_add(a1081, a1082)?
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

pub fn f20(v0: u64, v1: Adt3, v2: Adt3, v3: Adt4, v4: u64) -> R<Adt9> {
    f21(v0, v1, v2, v3, v4, false, 0u64, 0u64, 0u64, 0u64, 0u64)
}

pub fn f21(v0: u64, v1: Adt3, v2: Adt3, v3: Adt4, v4: u64, v5: bool, v6: u64, v7: u64, v8: u64, v9: u64, v10: u64) -> R<Adt9> {
    Ok(f69(v3.clone(), {
        let m1083 = v3.clone();
        match m1083 {
            Adt4::C0 => {
                f70(v0, v1, v2, v3)?
            }
            Adt4::C1 => {
                f70(v0, v1, v2, v3)?
            }
            Adt4::C2(_, _) => {
                f70(v0, v1, v2, v3)?
            }
            Adt4::C3(_, _, _, _, _) => {
                f70(v0, v1, v2, v3)?
            }
            Adt4::C4(_) => {
                f70(v0, v1, v2, v3)?
            }
            Adt4::C5(_, _, _, _) => {
                f70(v0, v1, v2, v3)?
            }
            Adt4::C6(_, _, _) => {
                f70(v0, v1, v2, v3)?
            }
            Adt4::C7(_) => {
                f70(v0, v1, v2, v3)?
            }
            Adt4::C8(_) => {
                f70(v0, v1, v2, v3)?
            }
            Adt4::C9(_, _) => {
                f70(v0, v1, v2, v3)?
            }
            Adt4::C10(_, _, _) => {
                f70(v0, v1, v2, v3)?
            }
            Adt4::C11(_, _) => {
                f70(v0, v1, v2, v3)?
            }
            Adt4::C12(_, _, _) => {
                f70(v0, v1, v2, v3)?
            }
            Adt4::C13(_, _, _, _) => {
                f70(v0, v1, v2, v3)?
            }
            Adt4::C14(_, _, _) => {
                f70(v0, v1, v2, v3)?
            }
            Adt4::C15 => {
                f70(v0, v1, v2, v3)?
            }
            Adt4::C16(_, _) => {
                f70(v0, v1, v2, v3)?
            }
            Adt4::C17 => {
                f89(v0, v1, v2, v4)?
            }
            Adt4::C18(_, _) => {
                f70(v0, v1, v2, v3)?
            }
            Adt4::C19(v88, v89) => {
                f90(v88, v0, v1, v2, v89, v4)?
            }
            Adt4::C20(v90, v91) => {
                f91(v90, v0, v1, v2, v91, v4)?
            }
            Adt4::C21 => {
                f70(v0, v1, v2, v3)?
            }
            Adt4::C22(v92, v93) => {
                f92(v0, v1, v2, v92, v93, v4)?
            }
            Adt4::C23(v107, v108, v109, v110) => {
                f99(v0, v110, v107, v108, v109, v4)?
            }
            Adt4::C24(v106) => {
                if v5 {
                    f97(Adt16::C0(v106), v0, v1, v2, v4, v6, v7, v8, v9, v10)?
                } else {
                    Adt9::C0(Adt10::C5(65u64))
                }
            }
            Adt4::C25(_, _, _) => {
                f70(v0, v1, v2, v3)?
            }
            Adt4::C26(v102, v103, v104, v105) => {
                let a1085 = v102;
                let a1086 = 64u64;
                if if if nat_eq(a1085, a1086) {
                    true
                } else {
                    let a1087 = v102;
                    let a1088 = 65u64;
                    nat_eq(a1087, a1088)
                } {
                    true
                } else {
                    let a1089 = v102;
                    let a1090 = 66u64;
                    nat_eq(a1089, a1090)
                } {
                    if v5 {
                        f97(Adt16::C1(v102, v103, v104, v105), v0, v1, v2, v4, v6, v7, v8, v9, v10)?
                    } else {
                        Adt9::C0(Adt10::C5(64u64))
                    }
                } else {
                    let a1091 = v102;
                    let a1092 = 0u64;
                    if if nat_eq(a1091, a1092) {
                        true
                    } else {
                        let a1093 = v102;
                        let a1094 = 30u64;
                        nat_eq(a1093, a1094)
                    } {
                        f98(v0, v1, v2, v102, v103, v104, v105, v4)?
                    } else {
                        Adt9::C0(Adt10::C1)
                    }
                }
            }
            Adt4::C27(v101) => {
                f96(v0, v1, v2, v101)?
            }
            Adt4::C28(v97) => {
                f93(v0, v1, v2, v97, v4)?
            }
            Adt4::C29(v98) => {
                f94(v0, v1, v2, v98)?
            }
            Adt4::C30(v99, _) => {
                f95(v0, v1, v2, v99)?
            }
            Adt4::C31(v33, v34) => {
                f71(v0, v1, v2, v33, v34)?
            }
            Adt4::C32(v35) => {
                f72(v0, match v1 {
                    Adt3::C0(h1084, _, _, _, _, _, _, _) => {
                        h1084
                    }
                }, v35)?
            }
            Adt4::C33(v36) => {
                f73(v0, v1, v36, v2)?
            }
            Adt4::C34(v68, v69, v70, v71) => {
                f88(v0, v1, v68, v2, v69, v70, v71)?
            }
            Adt4::C35(v67) => {
                f87(v0, v67, v1, v2)?
            }
            Adt4::C36(v37, v38, v39) => {
                f74(v0, v1, v37, v2, v38, v39)?
            }
            Adt4::C37(v40, v41, v42) => {
                f75(v0, v1, v40, v41, v42, v2)?
            }
            Adt4::C38(v43, v44, v45, v46) => {
                f76(v0, v1, v43, v44, v2, v45, v46, v4)?
            }
            Adt4::C39(v47) => {
                f77(v0, v1, v47)?
            }
            Adt4::C40(v48, v49, v50) => {
                f78(v0, v1, v2, v48, v49, v50, v4)?
            }
            Adt4::C41 => {
                f86(v0, v1, v2, v4)?
            }
            Adt4::C42(v61, v62) => {
                f84(v0, v1, v2, v61, v62)?
            }
            Adt4::C43(v59, v60) => {
                f83(v0, v1, v2, v59, v60, v4)?
            }
            Adt4::C44(v58) => {
                f82(v0, v2, v58)?
            }
            Adt4::C45(v63, v64, v65, v66) => {
                f85(v0, v1, v2, v63, v64, v65, v66, v4)?
            }
            Adt4::C46(v56, v57) => {
                f81(v0, v1, v56, v57)?
            }
            Adt4::C47(v54, v55) => {
                f80(v0, v1, v54, v55)?
            }
            Adt4::C48(v51, _, v53) => {
                f79(v0, v1, v2, v51, v53, v4)?
            }
        }
    }))
}

pub fn f22(v0: Adt4) -> R<Adt9> {
    let m1095 = v0;
    match m1095 {
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
        Adt4::C25(v85, _, v87) => {
            Ok(Adt9::C6({
                let a1103 = {
                    let a1099 = 0u64;
                    let a1100 = v85;
                    if nat_lt(a1099, a1100) {
                        1u64
                    } else {
                        0u64
                    }
                };
                let a1104 = {
                    let a1101 = 0u64;
                    let a1102 = v87;
                    if nat_lt(a1101, a1102) {
                        6u64
                    } else {
                        0u64
                    }
                };
                nat_add(a1103, a1104)?
            }, 0u64, 0u64, false, 3u64))
        }
        Adt4::C26(v93, _, _, v96) => {
            Ok(Adt9::C6({
                let a1105 = v93;
                let a1106 = 66u64;
                if nat_eq(a1105, a1106) {
                    3u64
                } else {
                    1u64
                }
            }, 0u64, 0u64, false, v96))
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
        Adt4::C35(v58) => {
            Ok(Adt9::C6(1u64, 0u64, 0u64, false, v58))
        }
        Adt4::C36(_, _, _) => {
            Ok(Adt9::C6(3u64, 0u64, 0u64, false, 3u64))
        }
        Adt4::C37(_, _, v32) => {
            Ok(Adt9::C6({
                let m1096 = v32;
                match m1096 {
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
        Adt4::C47(v45, _) => {
            Ok(Adt9::C6(if v45 {
                127u64
            } else {
                63u64
            }, 0u64, 0u64, false, 3u64))
        }
        Adt4::C48(_, v43, _) => {
            Ok(Adt9::C6(1u64, 0u64, 0u64, false, {
                let a1097 = 4u64;
                let a1098 = v43;
                nat_add(a1097, a1098)?
            }))
        }
    }
}

pub fn f23(v0: bool, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64, v7: u64, v8: u64, v9: u64, v10: u64, v11: u64, v12: u64) -> Adt9 {
    let a1107 = v1;
    let a1108 = 5u64;
    if if if if if nat_lt(a1107, a1108) {
        true
    } else {
        f100(v0, v1, v2, v3, v4)
    } {
        let a1109 = v5;
        let a1110 = 5u64;
        nat_lt(a1109, a1110)
    } else {
        false
    } {
        let a1111 = v7;
        let a1112 = 5u64;
        nat_lt(a1111, a1112)
    } else {
        false
    } {
        let a1113 = v9;
        let a1114 = 2u64;
        if nat_lt(a1113, a1114) {
            true
        } else {
            let a1115 = v9;
            let a1116 = 3u64;
            nat_eq(a1115, a1116)
        }
    } else {
        false
    } {
        Adt9::C7(f101(v0, v1, v2, v3, v4), f102(v5, v6), f102(v7, v8), f103({
            let a1117 = v9;
            let a1118 = 0u64;
            if nat_eq(a1117, a1118) {
                1u64
            } else {
                v9
            }
        }, v10), v11, v12)
    } else {
        Adt9::C0(Adt10::C1)
    }
}

pub fn f24(v0: u64, v1: u64) -> R<u64> {
    let a1127 = 3u64;
    let a1128 = {
        let a1125 = v1;
        let a1126 = {
            let a1119 = v0;
            let a1120 = 2147483521u64;
            if nat_lt(a1119, a1120) {
                4u64
            } else {
                let a1123 = 2u64;
                let a1124 = {
                    let a1121 = 4u64;
                    let a1122 = f104(v0);
                    nat_mul(a1121, a1122)?
                };
                nat_add(a1123, a1124)?
            }
        };
        nat_mul(a1125, a1126)?
    };
    nat_add(a1127, a1128)
}

pub fn f25(v0: Adt2, v1: u64) -> R<u64> {
    let a1140 = {
        let a1134 = {
            let a1130 = match v0.clone() {
                Adt2::C0(_, h1129, _, _, _, _, _, _, _, _, _) => {
                    h1129
                }
            };
            let a1131 = v1;
            let a1132 = 0u64;
            nat_quot(a1130, a1131, a1132)
        };
        let a1135 = match v0.clone() {
            Adt2::C0(_, _, h1133, _, _, _, _, _, _, _, _) => {
                h1133
            }
        };
        nat_mul(a1134, a1135)?
    };
    let a1141 = {
        let a1138 = match v0.clone() {
            Adt2::C0(_, _, _, h1136, _, _, _, _, _, _, _) => {
                h1136
            }
        };
        let a1139 = match v0 {
            Adt2::C0(_, _, _, _, h1137, _, _, _, _, _, _) => {
                h1137
            }
        };
        nat_mul(a1138, a1139)?
    };
    nat_mul(a1140, a1141)
}

pub fn f26(v0: Adt2, v1: u64) -> R<bool> {
    let a1142 = v1;
    let a1143 = 0u64;
    if nat_eq(a1142, a1143) {
        Ok(true)
    } else {
        let a1149 = {
            let a1146 = f105({
                let a1144 = v1;
                let a1145 = 1u64;
                nat_sub(a1144, a1145)
            }, f28(v0.clone(), 1u64))?;
            let a1147 = 4u64;
            nat_add(a1146, a1147)?
        };
        let a1150 = match v0 {
            Adt2::C0(_, _, _, _, _, _, _, _, _, h1148, _) => {
                h1148
            }
        };
        Ok(nat_le(a1149, a1150))
    }
}

pub fn f27(v0: Adt2, v1: Adt2) -> R<bool> {
    let a1152 = match v0.clone() {
        Adt2::C0(_, _, _, _, _, _, _, _, _, h1151, _) => {
            h1151
        }
    };
    let a1153 = 0u64;
    if nat_eq(a1152, a1153) {
        Ok(true)
    } else {
        let a1155 = match v1.clone() {
            Adt2::C0(_, _, _, _, _, _, _, _, _, h1154, _) => {
                h1154
            }
        };
        let a1156 = 0u64;
        if nat_eq(a1155, a1156) {
            Ok(true)
        } else {
            let a1158 = match v0.clone() {
                Adt2::C0(_, _, _, _, _, _, _, _, _, _, h1157) => {
                    h1157
                }
            };
            let a1159 = 0u64;
            if nat_eq(a1158, a1159) {
                Ok(true)
            } else {
                let a1161 = match v1.clone() {
                    Adt2::C0(_, _, _, _, _, _, _, _, _, _, h1160) => {
                        h1160
                    }
                };
                let a1162 = 0u64;
                if nat_eq(a1161, a1162) {
                    Ok(true)
                } else {
                    let a1168 = {
                        let a1165 = match v0.clone() {
                            Adt2::C0(_, _, _, _, _, _, _, _, _, _, h1163) => {
                                h1163
                            }
                        };
                        let a1166 = match v0.clone() {
                            Adt2::C0(_, _, _, _, _, _, _, _, _, h1164, _) => {
                                h1164
                            }
                        };
                        nat_add(a1165, a1166)?
                    };
                    let a1169 = match v1.clone() {
                        Adt2::C0(_, _, _, _, _, _, _, _, _, _, h1167) => {
                            h1167
                        }
                    };
                    if nat_le(a1168, a1169) {
                        Ok(true)
                    } else {
                        let a1175 = {
                            let a1172 = match v1.clone() {
                                Adt2::C0(_, _, _, _, _, _, _, _, _, _, h1170) => {
                                    h1170
                                }
                            };
                            let a1173 = match v1 {
                                Adt2::C0(_, _, _, _, _, _, _, _, _, h1171, _) => {
                                    h1171
                                }
                            };
                            nat_add(a1172, a1173)?
                        };
                        let a1176 = match v0 {
                            Adt2::C0(_, _, _, _, _, _, _, _, _, _, h1174) => {
                                h1174
                            }
                        };
                        Ok(nat_le(a1175, a1176))
                    }
                }
            }
        }
    }
}

pub fn f28(v0: Adt2, v1: u64) -> Adt3 {
    Adt3::C0({
        let a1178 = match v0.clone() {
            Adt2::C0(_, h1177, _, _, _, _, _, _, _, _, _) => {
                h1177
            }
        };
        let a1179 = v1;
        let a1180 = 0u64;
        nat_quot(a1178, a1179, a1180)
    }, match v0.clone() {
        Adt2::C0(_, _, h1181, _, _, _, _, _, _, _, _) => {
            h1181
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, h1182, _, _, _, _, _, _, _) => {
            h1182
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, h1183, _, _, _, _, _, _) => {
            h1183
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, _, h1184, _, _, _, _, _) => {
            h1184
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, _, _, h1185, _, _, _, _) => {
            h1185
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, _, _, _, h1186, _, _, _) => {
            h1186
        }
    }, match v0 {
        Adt2::C0(_, _, _, _, _, _, _, _, h1187, _, _) => {
            h1187
        }
    })
}

pub fn f29(v0: Adt2, v1: Adt2, v2: Adt2, v3: u64) -> R<bool> {
    let a1190 = match v0.clone() {
        Adt2::C0(_, h1188, _, _, _, _, _, _, _, _, _) => {
            h1188
        }
    };
    let a1191 = match v1.clone() {
        Adt2::C0(_, h1189, _, _, _, _, _, _, _, _, _) => {
            h1189
        }
    };
    if if if nat_eq(a1190, a1191) {
        let a1194 = match v0.clone() {
            Adt2::C0(_, _, h1192, _, _, _, _, _, _, _, _) => {
                h1192
            }
        };
        let a1195 = match v1.clone() {
            Adt2::C0(_, _, h1193, _, _, _, _, _, _, _, _) => {
                h1193
            }
        };
        nat_eq(a1194, a1195)
    } else {
        false
    } {
        let a1198 = match v0.clone() {
            Adt2::C0(_, _, _, h1196, _, _, _, _, _, _, _) => {
                h1196
            }
        };
        let a1199 = match v1 {
            Adt2::C0(_, _, _, h1197, _, _, _, _, _, _, _) => {
                h1197
            }
        };
        nat_eq(a1198, a1199)
    } else {
        false
    } {
        f31(v0, v2, v3)
    } else {
        Ok(false)
    }
}

pub fn f30(v0: u64, v1: u64, v2: Adt2, v3: Adt2, v4: Adt2) -> R<Adt10> {
    let v5: u64 = match v2.clone() {
        Adt2::C0(_, h1200, _, _, _, _, _, _, _, _, _) => {
            h1200
        }
    };
    let v6: u64 = match v2.clone() {
        Adt2::C0(_, _, h1201, _, _, _, _, _, _, _, _) => {
            h1201
        }
    };
    let v7: u64 = match v2.clone() {
        Adt2::C0(_, _, _, h1202, _, _, _, _, _, _, _) => {
            h1202
        }
    };
    let v8: u64 = match v3.clone() {
        Adt2::C0(_, _, h1203, _, _, _, _, _, _, _, _) => {
            h1203
        }
    };
    let v9: u64 = {
        let a1204 = v5;
        let a1205 = v6;
        nat_mul(a1204, a1205)?
    };
    let v10: u64 = {
        let a1206 = v9;
        let a1207 = v7;
        nat_mul(a1206, a1207)?
    };
    let v11: u64 = {
        let a1210 = {
            let a1208 = v9;
            let a1209 = v5;
            nat_mul(a1208, a1209)?
        };
        let a1211 = v8;
        nat_mul(a1210, a1211)?
    };
    let a1212 = 0u64;
    let a1213 = v6;
    if if if if if if if if if if if if if if nat_lt(a1212, a1213) {
        let a1214 = 0u64;
        let a1215 = v7;
        nat_lt(a1214, a1215)
    } else {
        false
    } {
        let a1216 = 0u64;
        let a1217 = v8;
        nat_lt(a1216, a1217)
    } else {
        false
    } {
        let a1221 = {
            let a1218 = v7;
            let a1219 = v8;
            let a1220 = 0u64;
            nat_rem(a1218, a1219, a1220)
        };
        let a1222 = 0u64;
        nat_eq(a1221, a1222)
    } else {
        false
    } {
        let a1224 = match v4.clone() {
            Adt2::C0(h1223, _, _, _, _, _, _, _, _, _, _) => {
                h1223
            }
        };
        let a1225 = 0u64;
        nat_eq(a1224, a1225)
    } else {
        false
    } {
        f45(v4.clone())?
    } else {
        false
    } {
        let a1227 = match v4.clone() {
            Adt2::C0(_, h1226, _, _, _, _, _, _, _, _, _) => {
                h1226
            }
        };
        let a1228 = v9;
        nat_eq(a1227, a1228)
    } else {
        false
    } {
        let a1234 = match v4.clone() {
            Adt2::C0(_, _, h1229, _, _, _, _, _, _, _, _) => {
                h1229
            }
        };
        let a1235 = {
            let a1232 = v7;
            let a1233 = {
                let a1230 = v5;
                let a1231 = v8;
                nat_mul(a1230, a1231)?
            };
            nat_add(a1232, a1233)?
        };
        nat_eq(a1234, a1235)
    } else {
        false
    } {
        let a1237 = match v4.clone() {
            Adt2::C0(_, _, _, h1236, _, _, _, _, _, _, _) => {
                h1236
            }
        };
        let a1238 = 1u64;
        nat_eq(a1237, a1238)
    } else {
        false
    } {
        let a1240 = match v4.clone() {
            Adt2::C0(_, _, _, _, h1239, _, _, _, _, _, _) => {
                h1239
            }
        };
        let a1241 = 1u64;
        nat_eq(a1240, a1241)
    } else {
        false
    } {
        let a1242 = f25(v3.clone(), 1u64)?;
        let a1243 = v11;
        nat_eq(a1242, a1243)
    } else {
        false
    } {
        f31(v2.clone(), v4.clone(), v10)?
    } else {
        false
    } {
        f31(v3, v4.clone(), v11)?
    } else {
        false
    } {
        let a1249 = {
            let a1246 = 4u64;
            let a1247 = {
                let a1244 = v10;
                let a1245 = v11;
                nat_add(a1244, a1245)?
            };
            nat_mul(a1246, a1247)?
        };
        let a1250 = match v4.clone() {
            Adt2::C0(_, _, _, _, _, _, _, _, _, h1248, _) => {
                h1248
            }
        };
        nat_le(a1249, a1250)
    } else {
        false
    } {
        let a1251 = v0;
        let a1252 = 1u64;
        if nat_eq(a1251, a1252) {
            Ok(Adt10::C4(f106(v5, v10)?, 4u64, f28(v2, 1u64), f28(v4, 1u64), Adt4::C46(v1, v8)))
        } else {
            Ok(Adt10::C4(f107({
                let a1253 = v0;
                let a1254 = 2u64;
                nat_eq(a1253, a1254)
            }, v5, v10)?, 4u64, f28(v2, 1u64), f28(v4, 1u64), Adt4::C47({
                let a1255 = v0;
                let a1256 = 2u64;
                nat_eq(a1255, a1256)
            }, v8)))
        }
    } else {
        Ok(Adt10::C1)
    }
}

pub fn f31(v0: Adt2, v1: Adt2, v2: u64) -> R<bool> {
    let a1258 = match v0.clone() {
        Adt2::C0(h1257, _, _, _, _, _, _, _, _, _, _) => {
            h1257
        }
    };
    let a1259 = 0u64;
    if if if nat_eq(a1258, a1259) {
        f45(v0.clone())?
    } else {
        false
    } {
        let a1263 = {
            let a1260 = 4u64;
            let a1261 = v2;
            nat_mul(a1260, a1261)?
        };
        let a1264 = match v0.clone() {
            Adt2::C0(_, _, _, _, _, _, _, _, _, h1262, _) => {
                h1262
            }
        };
        nat_le(a1263, a1264)
    } else {
        false
    } {
        f27(v0, v1)
    } else {
        Ok(false)
    }
}

pub fn f32(v0: Adt2, v1: Adt2) -> bool {
    let a1267 = match v0.clone() {
        Adt2::C0(_, h1265, _, _, _, _, _, _, _, _, _) => {
            h1265
        }
    };
    let a1268 = match v1.clone() {
        Adt2::C0(_, h1266, _, _, _, _, _, _, _, _, _) => {
            h1266
        }
    };
    if if if nat_eq(a1267, a1268) {
        let a1271 = match v0.clone() {
            Adt2::C0(_, _, h1269, _, _, _, _, _, _, _, _) => {
                h1269
            }
        };
        let a1272 = match v1.clone() {
            Adt2::C0(_, _, h1270, _, _, _, _, _, _, _, _) => {
                h1270
            }
        };
        nat_eq(a1271, a1272)
    } else {
        false
    } {
        let a1275 = match v0.clone() {
            Adt2::C0(_, _, _, h1273, _, _, _, _, _, _, _) => {
                h1273
            }
        };
        let a1276 = match v1.clone() {
            Adt2::C0(_, _, _, h1274, _, _, _, _, _, _, _) => {
                h1274
            }
        };
        nat_eq(a1275, a1276)
    } else {
        false
    } {
        let a1279 = match v0 {
            Adt2::C0(_, _, _, _, h1277, _, _, _, _, _, _) => {
                h1277
            }
        };
        let a1280 = match v1 {
            Adt2::C0(_, _, _, _, h1278, _, _, _, _, _, _) => {
                h1278
            }
        };
        nat_eq(a1279, a1280)
    } else {
        false
    }
}

pub fn f33(v0: Adt2, v1: u64, v2: bool) -> R<bool> {
    let a1282 = match v0.clone() {
        Adt2::C0(h1281, _, _, _, _, _, _, _, _, _, _) => {
            h1281
        }
    };
    let a1283 = 0u64;
    if if if nat_eq(a1282, a1283) {
        let a1285 = match v0.clone() {
            Adt2::C0(_, _, _, _, _, h1284, _, _, _, _, _) => {
                h1284
            }
        };
        let a1286 = 4u64;
        nat_eq(a1285, a1286)
    } else {
        false
    } {
        if v2 {
            f45(v0.clone())?
        } else {
            true
        }
    } else {
        false
    } {
        let a1298 = {
            let a1295 = {
                let a1290 = {
                    let a1287 = v1;
                    let a1288 = 1u64;
                    nat_sub(a1287, a1288)
                };
                let a1291 = match v0.clone() {
                    Adt2::C0(_, _, _, _, _, _, h1289, _, _, _, _) => {
                        h1289
                    }
                };
                nat_mul(a1290, a1291)?
            };
            let a1296 = {
                let a1293 = 4u64;
                let a1294 = match v0.clone() {
                    Adt2::C0(_, h1292, _, _, _, _, _, _, _, _, _) => {
                        h1292
                    }
                };
                nat_mul(a1293, a1294)?
            };
            nat_add(a1295, a1296)?
        };
        let a1299 = match v0 {
            Adt2::C0(_, _, _, _, _, _, _, _, _, h1297, _) => {
                h1297
            }
        };
        Ok(nat_le(a1298, a1299))
    } else {
        Ok(false)
    }
}

pub fn f34(v0: bool, v1: u64) -> R<u64> {
    let a1302 = if v0 {
        2u64
    } else {
        4u64
    };
    let a1303 = {
        let a1300 = if v0 {
            9u64
        } else {
            11u64
        };
        let a1301 = v1;
        nat_mul(a1300, a1301)?
    };
    nat_add(a1302, a1303)
}

pub fn f35(v0: Adt2, v1: Adt2) -> R<bool> {
    let a1305 = match v1.clone() {
        Adt2::C0(h1304, _, _, _, _, _, _, _, _, _, _) => {
            h1304
        }
    };
    let a1306 = 0u64;
    if if nat_eq(a1305, a1306) {
        f32(v0.clone(), v1.clone())
    } else {
        false
    } {
        let a1307 = f25(v0.clone(), 1u64)?;
        let a1308 = 0u64;
        if nat_eq(a1307, a1308) {
            Ok(true)
        } else {
            let a1314 = {
                let a1311 = f105({
                    let a1309 = f25(v0.clone(), 1u64)?;
                    let a1310 = 1u64;
                    nat_sub(a1309, a1310)
                }, f28(v0, 1u64))?;
                let a1312 = 4u64;
                nat_add(a1311, a1312)?
            };
            let a1315 = match v1 {
                Adt2::C0(_, _, _, _, _, _, _, _, _, h1313, _) => {
                    h1313
                }
            };
            Ok(nat_le(a1314, a1315))
        }
    } else {
        Ok(false)
    }
}

pub fn f36(v0: bool) -> u64 {
    if v0 {
        11u64
    } else {
        4u64
    }
}

pub fn f37(v0: bool) -> u64 {
    if v0 {
        15u64
    } else {
        3u64
    }
}

pub fn f38(v0: Adt2) -> R<bool> {
    let v1: u64 = {
        let a1321 = match v0.clone() {
            Adt2::C0(_, _, h1316, _, _, _, _, _, _, _, _) => {
                h1316
            }
        };
        let a1322 = {
            let a1319 = match v0.clone() {
                Adt2::C0(_, _, _, h1317, _, _, _, _, _, _, _) => {
                    h1317
                }
            };
            let a1320 = match v0.clone() {
                Adt2::C0(_, _, _, _, h1318, _, _, _, _, _, _) => {
                    h1318
                }
            };
            nat_mul(a1319, a1320)?
        };
        nat_mul(a1321, a1322)?
    };
    let v2: u64 = f25(v0.clone(), 1u64)?;
    let a1324 = match v0.clone() {
        Adt2::C0(h1323, _, _, _, _, _, _, _, _, _, _) => {
            h1323
        }
    };
    let a1325 = 0u64;
    if if if if if if if nat_eq(a1324, a1325) {
        let a1327 = match v0.clone() {
            Adt2::C0(_, h1326, _, _, _, _, _, _, _, _, _) => {
                h1326
            }
        };
        let a1328 = 2147483647u64;
        nat_le(a1327, a1328)
    } else {
        false
    } {
        let a1329 = v1;
        let a1330 = 2147483647u64;
        nat_le(a1329, a1330)
    } else {
        false
    } {
        let a1332 = match v0.clone() {
            Adt2::C0(_, h1331, _, _, _, _, _, _, _, _, _) => {
                h1331
            }
        };
        let a1333 = 1u64;
        if nat_eq(a1332, a1333) {
            true
        } else {
            let a1335 = match v0.clone() {
                Adt2::C0(_, _, _, _, _, h1334, _, _, _, _, _) => {
                    h1334
                }
            };
            let a1336 = 4u64;
            nat_eq(a1335, a1336)
        }
    } else {
        false
    } {
        let a1337 = v1;
        let a1338 = 1u64;
        if nat_le(a1337, a1338) {
            true
        } else {
            let a1343 = {
                let a1340 = match v0.clone() {
                    Adt2::C0(_, h1339, _, _, _, _, _, _, _, _, _) => {
                        h1339
                    }
                };
                let a1341 = 4u64;
                nat_mul(a1340, a1341)?
            };
            let a1344 = match v0.clone() {
                Adt2::C0(_, _, _, _, _, _, h1342, _, _, _, _) => {
                    h1342
                }
            };
            nat_le(a1343, a1344)
        }
    } else {
        false
    } {
        let a1346 = match v0.clone() {
            Adt2::C0(_, _, _, h1345, _, _, _, _, _, _, _) => {
                h1345
            }
        };
        let a1347 = 1u64;
        if nat_eq(a1346, a1347) {
            true
        } else {
            let a1353 = match v0.clone() {
                Adt2::C0(_, _, _, _, _, _, _, h1348, _, _, _) => {
                    h1348
                }
            };
            let a1354 = {
                let a1351 = match v0.clone() {
                    Adt2::C0(_, _, h1349, _, _, _, _, _, _, _, _) => {
                        h1349
                    }
                };
                let a1352 = match v0.clone() {
                    Adt2::C0(_, _, _, _, _, _, h1350, _, _, _, _) => {
                        h1350
                    }
                };
                nat_mul(a1351, a1352)?
            };
            nat_eq(a1353, a1354)
        }
    } else {
        false
    } {
        let a1356 = match v0.clone() {
            Adt2::C0(_, _, _, _, h1355, _, _, _, _, _, _) => {
                h1355
            }
        };
        let a1357 = 1u64;
        if nat_eq(a1356, a1357) {
            true
        } else {
            let a1366 = match v0.clone() {
                Adt2::C0(_, _, _, _, _, _, _, _, h1358, _, _) => {
                    h1358
                }
            };
            let a1367 = {
                let a1364 = match v0.clone() {
                    Adt2::C0(_, _, _, h1359, _, _, _, _, _, _, _) => {
                        h1359
                    }
                };
                let a1365 = {
                    let a1362 = match v0.clone() {
                        Adt2::C0(_, _, h1360, _, _, _, _, _, _, _, _) => {
                            h1360
                        }
                    };
                    let a1363 = match v0.clone() {
                        Adt2::C0(_, _, _, _, _, _, h1361, _, _, _, _) => {
                            h1361
                        }
                    };
                    nat_mul(a1362, a1363)?
                };
                nat_mul(a1364, a1365)?
            };
            nat_eq(a1366, a1367)
        }
    } else {
        false
    } {
        let a1368 = v2;
        let a1369 = 0u64;
        if nat_eq(a1368, a1369) {
            Ok(true)
        } else {
            let a1375 = {
                let a1372 = f105({
                    let a1370 = v2;
                    let a1371 = 1u64;
                    nat_sub(a1370, a1371)
                }, f28(v0.clone(), 1u64))?;
                let a1373 = 4u64;
                nat_add(a1372, a1373)?
            };
            let a1376 = match v0 {
                Adt2::C0(_, _, _, _, _, _, _, _, _, h1374, _) => {
                    h1374
                }
            };
            Ok(nat_le(a1375, a1376))
        }
    } else {
        Ok(false)
    }
}

pub fn f39(v0: u64) -> R<u64> {
    let a1383 = 148u64;
    let a1384 = {
        let a1381 = 36u64;
        let a1382 = {
            let a1379 = {
                let a1377 = 2u64;
                let a1378 = v0;
                nat_mul(a1377, a1378)?
            };
            let a1380 = 1u64;
            nat_sub(a1379, a1380)
        };
        nat_mul(a1381, a1382)?
    };
    nat_add(a1383, a1384)
}

pub fn f40(v0: u64, v1: Option<Adt3>) -> R<u64> {
    let m1385 = v1;
    match m1385 {
        None => {
            Ok(2u64)
        }
        Some(_) => {
            let a1388 = 1u64;
            let a1389 = {
                let a1386 = 2u64;
                let a1387 = v0;
                nat_mul(a1386, a1387)?
            };
            nat_add(a1388, a1389)
        }
    }
}

pub fn f41(v0: u64) -> bool {
    let a1390 = v0;
    let a1391 = 4294967296u64;
    if nat_lt(a1390, a1391) {
        let a1395 = {
            let a1392 = v0;
            let a1393 = 2147483648u64;
            let a1394 = 0u64;
            nat_rem(a1392, a1393, a1394)
        };
        let a1396 = 2139095040u64;
        nat_lt(a1395, a1396)
    } else {
        false
    }
}

pub fn f42(v0: u64) -> R<u64> {
    let a1397 = 4u64;
    let a1398 = v0;
    nat_mul(a1397, a1398)
}

pub fn f43(v0: u64) -> R<u64> {
    let a1399 = v0;
    let a1400 = 0u64;
    if nat_eq(a1399, a1400) {
        Ok(0u64)
    } else {
        let a1403 = 4u64;
        let a1404 = {
            let a1401 = 3u64;
            let a1402 = v0;
            nat_mul(a1401, a1402)?
        };
        nat_add(a1403, a1404)
    }
}

pub fn f44(v0: bool, v1: Adt2) -> Adt3 {
    if v0 {
        f28(v1, 1u64)
    } else {
        Adt3::C0(match v1.clone() {
            Adt2::C0(_, h1405, _, _, _, _, _, _, _, _, _) => {
                h1405
            }
        }, 1u64, match v1.clone() {
            Adt2::C0(_, _, h1406, _, _, _, _, _, _, _, _) => {
                h1406
            }
        }, match v1.clone() {
            Adt2::C0(_, _, _, h1407, _, _, _, _, _, _, _) => {
                h1407
            }
        }, match v1.clone() {
            Adt2::C0(_, _, _, _, _, h1408, _, _, _, _, _) => {
                h1408
            }
        }, 0u64, match v1.clone() {
            Adt2::C0(_, _, _, _, _, _, h1409, _, _, _, _) => {
                h1409
            }
        }, match v1 {
            Adt2::C0(_, _, _, _, _, _, _, h1410, _, _, _) => {
                h1410
            }
        })
    }
}

pub fn f45(v0: Adt2) -> R<bool> {
    let a1412 = match v0.clone() {
        Adt2::C0(_, h1411, _, _, _, _, _, _, _, _, _) => {
            h1411
        }
    };
    let a1413 = 1u64;
    if if if if nat_eq(a1412, a1413) {
        true
    } else {
        let a1415 = match v0.clone() {
            Adt2::C0(_, _, _, _, _, h1414, _, _, _, _, _) => {
                h1414
            }
        };
        let a1416 = 4u64;
        nat_eq(a1415, a1416)
    } {
        let a1418 = match v0.clone() {
            Adt2::C0(_, _, h1417, _, _, _, _, _, _, _, _) => {
                h1417
            }
        };
        let a1419 = 1u64;
        if nat_eq(a1418, a1419) {
            true
        } else {
            let a1424 = match v0.clone() {
                Adt2::C0(_, _, _, _, _, _, h1420, _, _, _, _) => {
                    h1420
                }
            };
            let a1425 = {
                let a1422 = match v0.clone() {
                    Adt2::C0(_, h1421, _, _, _, _, _, _, _, _, _) => {
                        h1421
                    }
                };
                let a1423 = 4u64;
                nat_mul(a1422, a1423)?
            };
            nat_eq(a1424, a1425)
        }
    } else {
        false
    } {
        let a1427 = match v0.clone() {
            Adt2::C0(_, _, _, h1426, _, _, _, _, _, _, _) => {
                h1426
            }
        };
        let a1428 = 1u64;
        if nat_eq(a1427, a1428) {
            true
        } else {
            let a1436 = match v0.clone() {
                Adt2::C0(_, _, _, _, _, _, _, h1429, _, _, _) => {
                    h1429
                }
            };
            let a1437 = {
                let a1434 = {
                    let a1432 = match v0.clone() {
                        Adt2::C0(_, h1430, _, _, _, _, _, _, _, _, _) => {
                            h1430
                        }
                    };
                    let a1433 = match v0.clone() {
                        Adt2::C0(_, _, h1431, _, _, _, _, _, _, _, _) => {
                            h1431
                        }
                    };
                    nat_mul(a1432, a1433)?
                };
                let a1435 = 4u64;
                nat_mul(a1434, a1435)?
            };
            nat_eq(a1436, a1437)
        }
    } else {
        false
    } {
        let a1439 = match v0.clone() {
            Adt2::C0(_, _, _, _, h1438, _, _, _, _, _, _) => {
                h1438
            }
        };
        let a1440 = 1u64;
        if nat_eq(a1439, a1440) {
            Ok(true)
        } else {
            let a1451 = match v0.clone() {
                Adt2::C0(_, _, _, _, _, _, _, _, h1441, _, _) => {
                    h1441
                }
            };
            let a1452 = {
                let a1449 = {
                    let a1447 = {
                        let a1444 = match v0.clone() {
                            Adt2::C0(_, h1442, _, _, _, _, _, _, _, _, _) => {
                                h1442
                            }
                        };
                        let a1445 = match v0.clone() {
                            Adt2::C0(_, _, h1443, _, _, _, _, _, _, _, _) => {
                                h1443
                            }
                        };
                        nat_mul(a1444, a1445)?
                    };
                    let a1448 = match v0 {
                        Adt2::C0(_, _, _, h1446, _, _, _, _, _, _, _) => {
                            h1446
                        }
                    };
                    nat_mul(a1447, a1448)?
                };
                let a1450 = 4u64;
                nat_mul(a1449, a1450)?
            };
            Ok(nat_eq(a1451, a1452))
        }
    } else {
        Ok(false)
    }
}

pub fn f46(v0: Adt6, v1: u64) -> R<u64> {
    let a1454 = match v0.clone() {
        Adt6::C0(h1453, _, _, _) => {
            h1453
        }
    };
    let a1455 = 0u64;
    if nat_eq(a1454, a1455) {
        f108(Adt6::C0(1u64, match v0.clone() {
            Adt6::C0(_, h1456, _, _) => {
                h1456
            }
        }, match v0.clone() {
            Adt6::C0(_, _, h1457, _) => {
                h1457
            }
        }, match v0.clone() {
            Adt6::C0(_, _, _, h1458) => {
                h1458
            }
        }), {
            let a1460 = v1;
            let a1461 = match v0 {
                Adt6::C0(_, _, _, h1459) => {
                    h1459
                }
            };
            nat_add(a1460, a1461)?
        })
    } else {
        f108(v0, v1)
    }
}

pub fn f47(v0: u64, v1: u64, v2: u64, v3: u64) -> R<u64> {
    let a1466 = {
        let a1462 = v0;
        let a1463 = v1;
        nat_mul(a1462, a1463)?
    };
    let a1467 = {
        let a1464 = v2;
        let a1465 = f109(v3);
        nat_mul(a1464, a1465)?
    };
    nat_mul(a1466, a1467)
}

pub fn f48(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let a1470 = v0;
    let a1471 = {
        let a1468 = f110(v1)?;
        let a1469 = v2;
        nat_mul(a1468, a1469)?
    };
    nat_add(a1470, a1471)
}

pub fn f49(v0: Adt2) -> Adt14 {
    Adt14::C0(match v0.clone() {
        Adt2::C0(h1472, _, _, _, _, _, _, _, _, _, _) => {
            h1472
        }
    }, match v0.clone() {
        Adt2::C0(_, h1473, _, _, _, _, _, _, _, _, _) => {
            h1473
        }
    }, match v0.clone() {
        Adt2::C0(_, _, h1474, _, _, _, _, _, _, _, _) => {
            h1474
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, h1475, _, _, _, _, _, _, _) => {
            h1475
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, h1476, _, _, _, _, _, _) => {
            h1476
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, _, h1477, _, _, _, _, _) => {
            h1477
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, _, _, h1478, _, _, _, _) => {
            h1478
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, _, _, _, h1479, _, _, _) => {
            h1479
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, _, _, _, _, h1480, _, _) => {
            h1480
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, _, _, _, _, _, h1481, _) => {
            h1481
        }
    }, match v0 {
        Adt2::C0(_, _, _, _, _, _, _, _, _, _, h1482) => {
            h1482
        }
    })
}

pub fn f50(v0: u64, v1: u64, v2: u64, v3: bool, v4: Adt14, v5: Adt14, v6: Adt14) -> R<Adt15> {
    let a1483 = v0;
    let a1484 = 23u64;
    if nat_eq(a1483, a1484) {
        f111(v1, v4, v6)
    } else {
        let a1485 = v0;
        let a1486 = 26u64;
        if nat_eq(a1485, a1486) {
            f112(v1, v2, v4, v6)
        } else {
            let a1487 = v0;
            let a1488 = 25u64;
            if nat_eq(a1487, a1488) {
                f113(v1, v3, v4, v5, v6)
            } else {
                Ok(Adt15::C0)
            }
        }
    }
}

pub fn f51(v0: u64) -> u64 {
    let a1489 = v0;
    let a1490 = 30u64;
    if nat_eq(a1489, a1490) {
        2u64
    } else {
        4u64
    }
}

pub fn f52(v0: u64) -> bool {
    let a1491 = v0;
    let a1492 = 2139095040u64;
    if nat_le(a1491, a1492) {
        true
    } else {
        let a1493 = v0;
        let a1494 = 2147483648u64;
        nat_eq(a1493, a1494)
    }
}

pub fn f53(v0: Adt2, v1: u64) -> bool {
    let a1496 = match v0.clone() {
        Adt2::C0(_, _, _, _, _, h1495, _, _, _, _, _) => {
            h1495
        }
    };
    let a1497 = v1;
    if if nat_eq(a1496, a1497) {
        let a1502 = {
            let a1499 = match v0.clone() {
                Adt2::C0(_, _, _, _, _, _, _, _, _, _, h1498) => {
                    h1498
                }
            };
            let a1500 = v1;
            let a1501 = 0u64;
            nat_rem(a1499, a1500, a1501)
        };
        let a1503 = 0u64;
        nat_eq(a1502, a1503)
    } else {
        false
    } {
        f114(match v0.clone() {
            Adt2::C0(_, _, h1504, _, _, _, _, _, _, _, _) => {
                h1504
            }
        }, match v0.clone() {
            Adt2::C0(_, _, _, h1505, _, _, _, _, _, _, _) => {
                h1505
            }
        }, match v0.clone() {
            Adt2::C0(_, _, _, _, h1506, _, _, _, _, _, _) => {
                h1506
            }
        }, match v0.clone() {
            Adt2::C0(_, _, _, _, _, _, h1507, _, _, _, _) => {
                h1507
            }
        }, match v0.clone() {
            Adt2::C0(_, _, _, _, _, _, _, h1508, _, _, _) => {
                h1508
            }
        }, match v0 {
            Adt2::C0(_, _, _, _, _, _, _, _, h1509, _, _) => {
                h1509
            }
        }, v1)
    } else {
        false
    }
}

pub fn f54(v0: Adt2, v1: Adt2) -> bool {
    if if if if if if f32(v0.clone(), v1.clone()) {
        let a1512 = match v0.clone() {
            Adt2::C0(h1510, _, _, _, _, _, _, _, _, _, _) => {
                h1510
            }
        };
        let a1513 = match v1.clone() {
            Adt2::C0(h1511, _, _, _, _, _, _, _, _, _, _) => {
                h1511
            }
        };
        nat_eq(a1512, a1513)
    } else {
        false
    } {
        let a1516 = match v0.clone() {
            Adt2::C0(_, _, _, _, _, _, _, _, _, _, h1514) => {
                h1514
            }
        };
        let a1517 = match v1.clone() {
            Adt2::C0(_, _, _, _, _, _, _, _, _, _, h1515) => {
                h1515
            }
        };
        nat_eq(a1516, a1517)
    } else {
        false
    } {
        let a1520 = match v0.clone() {
            Adt2::C0(_, _, _, _, _, h1518, _, _, _, _, _) => {
                h1518
            }
        };
        let a1521 = match v1.clone() {
            Adt2::C0(_, _, _, _, _, h1519, _, _, _, _, _) => {
                h1519
            }
        };
        nat_eq(a1520, a1521)
    } else {
        false
    } {
        let a1524 = match v0.clone() {
            Adt2::C0(_, _, _, _, _, _, h1522, _, _, _, _) => {
                h1522
            }
        };
        let a1525 = match v1.clone() {
            Adt2::C0(_, _, _, _, _, _, h1523, _, _, _, _) => {
                h1523
            }
        };
        nat_eq(a1524, a1525)
    } else {
        false
    } {
        let a1528 = match v0.clone() {
            Adt2::C0(_, _, _, _, _, _, _, h1526, _, _, _) => {
                h1526
            }
        };
        let a1529 = match v1.clone() {
            Adt2::C0(_, _, _, _, _, _, _, h1527, _, _, _) => {
                h1527
            }
        };
        nat_eq(a1528, a1529)
    } else {
        false
    } {
        let a1532 = match v0 {
            Adt2::C0(_, _, _, _, _, _, _, _, h1530, _, _) => {
                h1530
            }
        };
        let a1533 = match v1 {
            Adt2::C0(_, _, _, _, _, _, _, _, h1531, _, _) => {
                h1531
            }
        };
        nat_eq(a1532, a1533)
    } else {
        false
    }
}

pub fn f55(v0: u64, v1: u64, v2: u64, v3: u64) -> bool {
    let a1534 = v0;
    let a1535 = 0u64;
    if if if nat_eq(a1534, a1535) {
        true
    } else {
        let a1536 = v1;
        let a1537 = 0u64;
        nat_eq(a1536, a1537)
    } {
        true
    } else {
        let a1538 = v2;
        let a1539 = 0u64;
        nat_eq(a1538, a1539)
    } {
        true
    } else {
        let a1540 = v3;
        let a1541 = 0u64;
        nat_eq(a1540, a1541)
    }
}

pub fn f56(v0: Adt2, v1: u64) -> R<bool> {
    f115({
        let a1543 = match v0.clone() {
            Adt2::C0(_, h1542, _, _, _, _, _, _, _, _, _) => {
                h1542
            }
        };
        let a1544 = v1;
        nat_mul(a1543, a1544)?
    }, match v0.clone() {
        Adt2::C0(_, _, h1545, _, _, _, _, _, _, _, _) => {
            h1545
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, h1546, _, _, _, _, _, _, _) => {
            h1546
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, h1547, _, _, _, _, _, _) => {
            h1547
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, _, _, h1548, _, _, _, _) => {
            h1548
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, _, _, _, h1549, _, _, _) => {
            h1549
        }
    }, match v0 {
        Adt2::C0(_, _, _, _, _, _, _, _, h1550, _, _) => {
            h1550
        }
    })
}

pub fn f57(v0: u64, v1: u64, v2: u64, v3: u64) -> R<u64> {
    let a1551 = v1;
    let a1552 = 0u64;
    if if nat_eq(a1551, a1552) {
        true
    } else {
        let a1553 = v2;
        let a1554 = 0u64;
        nat_eq(a1553, a1554)
    } {
        Ok(0u64)
    } else {
        let a1561 = {
            let a1555 = f116(v1, v2, v3)?;
            let a1556 = f117(v0, v1, v2)?;
            nat_add(a1555, a1556)?
        };
        let a1562 = {
            let a1559 = f118(v0);
            let a1560 = {
                let a1557 = v1;
                let a1558 = v2;
                nat_mul(a1557, a1558)?
            };
            nat_mul(a1559, a1560)?
        };
        nat_add(a1561, a1562)
    }
}

pub fn f58(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let a1563 = v0;
    let a1564 = 0u64;
    if if nat_eq(a1563, a1564) {
        true
    } else {
        let a1565 = v1;
        let a1566 = 0u64;
        nat_eq(a1565, a1566)
    } {
        Ok(3u64)
    } else {
        let a1569 = {
            let a1567 = 3u64;
            let a1568 = v1;
            nat_add(a1567, a1568)?
        };
        let a1570 = v2;
        nat_add(a1569, a1570)
    }
}

pub fn f59(v0: u64) -> bool {
    let a1571 = v0;
    let a1572 = 2139095040u64;
    if nat_le(a1571, a1572) {
        true
    } else {
        let a1573 = v0;
        let a1574 = 2147483648u64;
        nat_eq(a1573, a1574)
    }
}

pub fn f60(v0: u64, v1: u64) -> R<u64> {
    let a1575 = v0;
    let a1576 = 0u64;
    if nat_eq(a1575, a1576) {
        Ok(0u64)
    } else {
        let a1577 = v1;
        let a1578 = f119(v0)?;
        nat_mul(a1577, a1578)
    }
}

pub fn f61(v0: u64, v1: u64, v2: u64, v3: u64) -> R<u64> {
    let a1579 = v0;
    let a1580 = 0u64;
    if if if if nat_eq(a1579, a1580) {
        true
    } else {
        let a1581 = v1;
        let a1582 = 0u64;
        nat_eq(a1581, a1582)
    } {
        true
    } else {
        let a1583 = v2;
        let a1584 = 0u64;
        nat_eq(a1583, a1584)
    } {
        true
    } else {
        let a1585 = v3;
        let a1586 = 0u64;
        nat_eq(a1585, a1586)
    } {
        Ok(0u64)
    } else {
        let a1599 = {
            let a1589 = 2u64;
            let a1590 = {
                let a1587 = v1;
                let a1588 = v2;
                nat_mul(a1587, a1588)?
            };
            nat_mul(a1589, a1590)?
        };
        let a1600 = {
            let a1597 = {
                let a1593 = {
                    let a1591 = v1;
                    let a1592 = v2;
                    nat_mul(a1591, a1592)?
                };
                let a1594 = v3;
                nat_mul(a1593, a1594)?
            };
            let a1598 = {
                let a1595 = 1u64;
                let a1596 = v0;
                nat_add(a1595, a1596)?
            };
            nat_mul(a1597, a1598)?
        };
        nat_add(a1599, a1600)
    }
}

pub fn f62(v0: u64, v1: u64, v2: u64) -> Option<bool> {
    let v3: bool = {
        let a1601 = v1;
        let a1602 = 0u64;
        if if if nat_eq(a1601, a1602) {
            true
        } else {
            let a1603 = v1;
            let a1604 = 10u64;
            nat_eq(a1603, a1604)
        } {
            true
        } else {
            let a1605 = v1;
            let a1606 = 15u64;
            nat_eq(a1605, a1606)
        } {
            true
        } else {
            let a1607 = v1;
            let a1608 = 20u64;
            nat_eq(a1607, a1608)
        }
    };
    let v4: bool = {
        let a1609 = v2;
        let a1610 = 0u64;
        if if if if if nat_eq(a1609, a1610) {
            true
        } else {
            let a1611 = v2;
            let a1612 = 10u64;
            nat_eq(a1611, a1612)
        } {
            true
        } else {
            let a1613 = v2;
            let a1614 = 15u64;
            nat_eq(a1613, a1614)
        } {
            true
        } else {
            let a1615 = v2;
            let a1616 = 20u64;
            nat_eq(a1615, a1616)
        } {
            true
        } else {
            let a1617 = v2;
            let a1618 = 30u64;
            nat_eq(a1617, a1618)
        } {
            true
        } else {
            let a1619 = v2;
            let a1620 = 40u64;
            nat_eq(a1619, a1620)
        }
    };
    if if v3 {
        v4
    } else {
        false
    } {
        Some({
            let a1621 = v0;
            let a1622 = 30u64;
            if if nat_eq(a1621, a1622) {
                let a1623 = v2;
                let a1624 = 10u64;
                nat_eq(a1623, a1624)
            } else {
                false
            } {
                true
            } else {
                if f120(v0) {
                    let a1625 = v2;
                    let a1626 = 10u64;
                    if if nat_eq(a1625, a1626) {
                        true
                    } else {
                        let a1627 = v2;
                        let a1628 = 15u64;
                        nat_eq(a1627, a1628)
                    } {
                        true
                    } else {
                        let a1629 = v2;
                        let a1630 = 20u64;
                        nat_eq(a1629, a1630)
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

pub fn f63(v0: u64, v1: Adt1, v2: Adt2, v3: Option<Adt2>, v4: Adt2) -> R<Adt9> {
    let v5: u64 = match v1.clone() {
        Adt1::C0(h1631, _, _, _, _, _, _, _, _, _, _, _) => {
            h1631
        }
    };
    Ok(Adt9::C0({
        let a1632 = v0;
        let a1633 = 65u64;
        if nat_eq(a1632, a1633) {
            let m1634 = v3;
            match m1634 {
                None => {
                    f121(v1, v2, v4)?
                }
                Some(_) => {
                    Adt10::C1
                }
            }
        } else {
            let a1635 = v0;
            let a1636 = 59u64;
            if nat_eq(a1635, a1636) {
                let m1637 = v3;
                match m1637 {
                    None => {
                        Adt10::C1
                    }
                    Some(v7) => {
                        f122(v1, v2, v7, v4)?
                    }
                }
            } else {
                let a1638 = v0;
                let a1639 = 50u64;
                if nat_eq(a1638, a1639) {
                    let m1640 = v3;
                    match m1640 {
                        None => {
                            Adt10::C1
                        }
                        Some(v8) => {
                            f123(v1, v2, v8, v4)?
                        }
                    }
                } else {
                    let a1641 = v0;
                    let a1642 = 82u64;
                    if nat_eq(a1641, a1642) {
                        let m1643 = v3;
                        match m1643 {
                            None => {
                                Adt10::C1
                            }
                            Some(v9) => {
                                f124(v2, v9, v4)?
                            }
                        }
                    } else {
                        let a1644 = v0;
                        let a1645 = 86u64;
                        if nat_eq(a1644, a1645) {
                            let m1646 = v3;
                            match m1646 {
                                None => {
                                    Adt10::C1
                                }
                                Some(v10) => {
                                    f125(v1, v2, v10, v4)?
                                }
                            }
                        } else {
                            let a1647 = v0;
                            let a1648 = 55u64;
                            if nat_eq(a1647, a1648) {
                                let m1649 = v3;
                                match m1649 {
                                    None => {
                                        Adt10::C1
                                    }
                                    Some(v11) => {
                                        f126(v1, v2, v11, v4)?
                                    }
                                }
                            } else {
                                let a1650 = v0;
                                let a1651 = 22u64;
                                if nat_eq(a1650, a1651) {
                                    let m1652 = v3;
                                    match m1652 {
                                        None => {
                                            Adt10::C1
                                        }
                                        Some(v12) => {
                                            f127(v2, v12, v4)?
                                        }
                                    }
                                } else {
                                    let a1653 = v0;
                                    let a1654 = 44u64;
                                    if nat_eq(a1653, a1654) {
                                        f128(v1, v2, v3, v4)?
                                    } else {
                                        let a1655 = v0;
                                        let a1656 = 52u64;
                                        if nat_eq(a1655, a1656) {
                                            f129(v1, v2, v4)?
                                        } else {
                                            let a1657 = v0;
                                            let a1658 = 73u64;
                                            if nat_eq(a1657, a1658) {
                                                f130(v2, v3, v4)?
                                            } else {
                                                let a1659 = v0;
                                                let a1660 = 57u64;
                                                if if nat_eq(a1659, a1660) {
                                                    true
                                                } else {
                                                    let a1661 = v0;
                                                    let a1662 = 58u64;
                                                    nat_eq(a1661, a1662)
                                                } {
                                                    let m1663 = v3;
                                                    match m1663 {
                                                        None => {
                                                            f131(false, {
                                                                let a1664 = v0;
                                                                let a1665 = 58u64;
                                                                nat_eq(a1664, a1665)
                                                            }, v1, v2, v4)?
                                                        }
                                                        Some(_) => {
                                                            Adt10::C1
                                                        }
                                                    }
                                                } else {
                                                    let a1666 = v0;
                                                    let a1667 = 49u64;
                                                    if nat_eq(a1666, a1667) {
                                                        f132(false, v1, v2, v3, v4)?
                                                    } else {
                                                        let a1668 = v0;
                                                        let a1669 = 51u64;
                                                        if nat_eq(a1668, a1669) {
                                                            f133(v1, v2, v3, v4)?
                                                        } else {
                                                            let a1670 = v0;
                                                            let a1671 = 39u64;
                                                            if nat_eq(a1670, a1671) {
                                                                f134(v2, v3, v4)?
                                                            } else {
                                                                let a1672 = v0;
                                                                let a1673 = 77u64;
                                                                if nat_eq(a1672, a1673) {
                                                                    f135(v2, v4)?
                                                                } else {
                                                                    let a1674 = v0;
                                                                    let a1675 = 40u64;
                                                                    if nat_eq(a1674, a1675) {
                                                                        f136(v2, v3, v4)?
                                                                    } else {
                                                                        if f137(v0) {
                                                                            let a1676 = v0;
                                                                            let a1677 = 33u64;
                                                                            if if if nat_eq(a1676, a1677) {
                                                                                let a1679 = match v2.clone() {
                                                                                    Adt2::C0(h1678, _, _, _, _, _, _, _, _, _, _) => {
                                                                                        h1678
                                                                                    }
                                                                                };
                                                                                let a1680 = 0u64;
                                                                                nat_eq(a1679, a1680)
                                                                            } else {
                                                                                false
                                                                            } {
                                                                                let a1682 = match v4.clone() {
                                                                                    Adt2::C0(h1681, _, _, _, _, _, _, _, _, _, _) => {
                                                                                        h1681
                                                                                    }
                                                                                };
                                                                                let a1683 = 26u64;
                                                                                nat_eq(a1682, a1683)
                                                                            } else {
                                                                                false
                                                                            } {
                                                                                let m1684 = v3;
                                                                                match m1684 {
                                                                                    None => {
                                                                                        Adt10::C1
                                                                                    }
                                                                                    Some(v14) => {
                                                                                        f138(v2, v14, v4)?
                                                                                    }
                                                                                }
                                                                            } else {
                                                                                let a1685 = v0;
                                                                                let a1686 = 33u64;
                                                                                if if if nat_eq(a1685, a1686) {
                                                                                    let a1688 = match v4.clone() {
                                                                                        Adt2::C0(h1687, _, _, _, _, _, _, _, _, _, _) => {
                                                                                            h1687
                                                                                        }
                                                                                    };
                                                                                    let a1689 = 0u64;
                                                                                    nat_eq(a1688, a1689)
                                                                                } else {
                                                                                    false
                                                                                } {
                                                                                    f139(match v2.clone() {
                                                                                        Adt2::C0(h1690, _, _, _, _, _, _, _, _, _, _) => {
                                                                                            h1690
                                                                                        }
                                                                                    })
                                                                                } else {
                                                                                    false
                                                                                } {
                                                                                    f140(v2, v3, v4)?
                                                                                } else {
                                                                                    f141(v0, v2, {
                                                                                        let m1691 = v3;
                                                                                        match m1691 {
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
                                                                            let a1692 = v0;
                                                                            let a1693 = 19u64;
                                                                            if nat_eq(a1692, a1693) {
                                                                                f142(v2, v4)?
                                                                            } else {
                                                                                let a1694 = v0;
                                                                                let a1695 = 21u64;
                                                                                if nat_eq(a1694, a1695) {
                                                                                    f143(v5, v2, v3, v4)?
                                                                                } else {
                                                                                    let a1696 = v0;
                                                                                    let a1697 = 61u64;
                                                                                    if if if nat_eq(a1696, a1697) {
                                                                                        true
                                                                                    } else {
                                                                                        let a1698 = v0;
                                                                                        let a1699 = 62u64;
                                                                                        nat_eq(a1698, a1699)
                                                                                    } {
                                                                                        true
                                                                                    } else {
                                                                                        let a1700 = v0;
                                                                                        let a1701 = 63u64;
                                                                                        nat_eq(a1700, a1701)
                                                                                    } {
                                                                                        f144(v0, v1, v2, v4)?
                                                                                    } else {
                                                                                        let a1702 = v0;
                                                                                        let a1703 = 4u64;
                                                                                        if if nat_eq(a1702, a1703) {
                                                                                            true
                                                                                        } else {
                                                                                            let a1704 = v0;
                                                                                            let a1705 = 32u64;
                                                                                            nat_eq(a1704, a1705)
                                                                                        } {
                                                                                            f145(v0, v1, v2, v3, v4)?
                                                                                        } else {
                                                                                            let a1706 = v0;
                                                                                            let a1707 = 20u64;
                                                                                            if nat_eq(a1706, a1707) {
                                                                                                f146(v2, v4)?
                                                                                            } else {
                                                                                                let a1708 = v0;
                                                                                                let a1709 = 42u64;
                                                                                                if nat_eq(a1708, a1709) {
                                                                                                    f147(4u64, v2, v4)?
                                                                                                } else {
                                                                                                    let a1710 = v0;
                                                                                                    let a1711 = 69u64;
                                                                                                    if nat_eq(a1710, a1711) {
                                                                                                        let a1712 = v5;
                                                                                                        let a1713 = 4u64;
                                                                                                        if nat_lt(a1712, a1713) {
                                                                                                            f147(v5, v2, v4)?
                                                                                                        } else {
                                                                                                            Adt10::C1
                                                                                                        }
                                                                                                    } else {
                                                                                                        let a1714 = v0;
                                                                                                        let a1715 = 70u64;
                                                                                                        if nat_eq(a1714, a1715) {
                                                                                                            f148(v5, v2, v4)?
                                                                                                        } else {
                                                                                                            let a1716 = v0;
                                                                                                            let a1717 = 75u64;
                                                                                                            if if nat_eq(a1716, a1717) {
                                                                                                                true
                                                                                                            } else {
                                                                                                                let a1718 = v0;
                                                                                                                let a1719 = 76u64;
                                                                                                                nat_eq(a1718, a1719)
                                                                                                            } {
                                                                                                                f149({
                                                                                                                    let a1720 = v0;
                                                                                                                    let a1721 = 76u64;
                                                                                                                    nat_eq(a1720, a1721)
                                                                                                                }, v1, v2, v4)?
                                                                                                            } else {
                                                                                                                let a1722 = v0;
                                                                                                                let a1723 = 31u64;
                                                                                                                if if if nat_eq(a1722, a1723) {
                                                                                                                    true
                                                                                                                } else {
                                                                                                                    let a1724 = v0;
                                                                                                                    let a1725 = 47u64;
                                                                                                                    nat_eq(a1724, a1725)
                                                                                                                } {
                                                                                                                    true
                                                                                                                } else {
                                                                                                                    let a1726 = v0;
                                                                                                                    let a1727 = 68u64;
                                                                                                                    nat_eq(a1726, a1727)
                                                                                                                } {
                                                                                                                    f150(v0, v1, v2, v4)?
                                                                                                                } else {
                                                                                                                    let a1728 = v0;
                                                                                                                    let a1729 = 15u64;
                                                                                                                    if nat_eq(a1728, a1729) {
                                                                                                                        f151(v2, v4)?
                                                                                                                    } else {
                                                                                                                        let a1730 = v0;
                                                                                                                        let a1731 = 17u64;
                                                                                                                        if nat_eq(a1730, a1731) {
                                                                                                                            f152(v2, v4)?
                                                                                                                        } else {
                                                                                                                            let a1732 = v0;
                                                                                                                            let a1733 = 18u64;
                                                                                                                            if nat_eq(a1732, a1733) {
                                                                                                                                f153(v2, v3, v4)?
                                                                                                                            } else {
                                                                                                                                let a1734 = v0;
                                                                                                                                let a1735 = 13u64;
                                                                                                                                if if if nat_eq(a1734, a1735) {
                                                                                                                                    true
                                                                                                                                } else {
                                                                                                                                    let a1736 = v0;
                                                                                                                                    let a1737 = 14u64;
                                                                                                                                    nat_eq(a1736, a1737)
                                                                                                                                } {
                                                                                                                                    true
                                                                                                                                } else {
                                                                                                                                    let a1738 = v0;
                                                                                                                                    let a1739 = 16u64;
                                                                                                                                    nat_eq(a1738, a1739)
                                                                                                                                } {
                                                                                                                                    f154(v0, v2, v4)?
                                                                                                                                } else {
                                                                                                                                    let m1740 = f155(v0, v5)?;
                                                                                                                                    match m1740 {
                                                                                                                                        None => {
                                                                                                                                            f156(v0, v5, v2, v3, v4)?
                                                                                                                                        }
                                                                                                                                        Some(v16) => {
                                                                                                                                            f157(v16, v1, v2, v4)?
                                                                                                                                        }
                                                                                                                                    }
                                                                                                                                }
                                                                                                                            }
                                                                                                                        }
                                                                                                                    }
                                                                                                                }
                                                                                                            }
                                                                                                        }
                                                                                                    }
                                                                                                }
                                                                                            }
                                                                                        }
                                                                                    }
                                                                                }
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
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

pub fn f64(v0: Adt3, v1: u64, v2: u64, v3: u64, v4: u64) -> R<u64> {
    f158(v1, v2, v3, v4, match v0.clone() {
        Adt3::C0(_, _, _, _, h1741, _, _, _) => {
            h1741
        }
    }, match v0.clone() {
        Adt3::C0(_, _, _, _, _, h1742, _, _) => {
            h1742
        }
    }, match v0.clone() {
        Adt3::C0(_, _, _, _, _, _, h1743, _) => {
            h1743
        }
    }, match v0 {
        Adt3::C0(_, _, _, _, _, _, _, h1744) => {
            h1744
        }
    })
}

pub fn f65(v0: u64, v1: Adt3, v2: u64) -> u64 {
    let v3: u64 = {
        let a1746 = v0;
        let a1747 = match v1.clone() {
            Adt3::C0(h1745, _, _, _, _, _, _, _) => {
                h1745
            }
        };
        let a1748 = 0u64;
        nat_quot(a1746, a1747, a1748)
    };
    let v4: u64 = {
        let a1750 = v3;
        let a1751 = match v1.clone() {
            Adt3::C0(_, h1749, _, _, _, _, _, _) => {
                h1749
            }
        };
        let a1752 = 0u64;
        nat_quot(a1750, a1751, a1752)
    };
    let a1753 = v2;
    let a1754 = 0u64;
    if nat_eq(a1753, a1754) {
        let a1756 = v0;
        let a1757 = match v1 {
            Adt3::C0(h1755, _, _, _, _, _, _, _) => {
                h1755
            }
        };
        let a1758 = 0u64;
        nat_rem(a1756, a1757, a1758)
    } else {
        let a1759 = v2;
        let a1760 = 1u64;
        if nat_eq(a1759, a1760) {
            let a1762 = v3;
            let a1763 = match v1 {
                Adt3::C0(_, h1761, _, _, _, _, _, _) => {
                    h1761
                }
            };
            let a1764 = 0u64;
            nat_rem(a1762, a1763, a1764)
        } else {
            let a1765 = v2;
            let a1766 = 2u64;
            if nat_eq(a1765, a1766) {
                let a1768 = v4;
                let a1769 = match v1 {
                    Adt3::C0(_, _, h1767, _, _, _, _, _) => {
                        h1767
                    }
                };
                let a1770 = 0u64;
                nat_rem(a1768, a1769, a1770)
            } else {
                let a1772 = v4;
                let a1773 = match v1 {
                    Adt3::C0(_, _, h1771, _, _, _, _, _) => {
                        h1771
                    }
                };
                let a1774 = 0u64;
                nat_quot(a1772, a1773, a1774)
            }
        }
    }
}

pub fn f66(v0: Adt8, v1: u64, v2: u64) -> R<u64> {
    let a1776 = match v0.clone() {
        Adt8::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, h1775) => {
            h1775
        }
    };
    let a1777 = 0u64;
    if nat_eq(a1776, a1777) {
        Ok(v2)
    } else {
        let a1782 = {
            let a1779 = v2;
            let a1780 = match v0.clone() {
                Adt8::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, h1778) => {
                    h1778
                }
            };
            nat_mul(a1779, a1780)?
        };
        let a1783 = f65(v1, match v0 {
            Adt8::C0(_, _, _, _, _, _, _, _, h1781, _, _, _, _, _, _) => {
                h1781
            }
        }, 3u64);
        nat_add(a1782, a1783)
    }
}

pub fn f67(v0: Adt6, v1: u64, v2: u64, v3: u64) -> R<Option<u64>> {
    let v4: u64 = {
        let a1793 = {
            let a1790 = {
                let a1785 = v2;
                let a1786 = match v0.clone() {
                    Adt6::C0(_, h1784, _, _) => {
                        h1784
                    }
                };
                nat_mul(a1785, a1786)?
            };
            let a1791 = {
                let a1788 = v3;
                let a1789 = match v0.clone() {
                    Adt6::C0(_, _, _, h1787) => {
                        h1787
                    }
                };
                nat_mul(a1788, a1789)?
            };
            nat_add(a1790, a1791)?
        };
        let a1794 = f159(match v0.clone() {
            Adt6::C0(_, _, h1792, _) => {
                h1792
            }
        });
        nat_add(a1793, a1794)?
    };
    let v5: u64 = f160(match v0 {
        Adt6::C0(_, _, h1795, _) => {
            h1795
        }
    });
    let a1796 = v5;
    let a1797 = v4;
    if if nat_le(a1796, a1797) {
        let a1800 = {
            let a1798 = v4;
            let a1799 = v5;
            nat_sub(a1798, a1799)
        };
        let a1801 = v1;
        nat_lt(a1800, a1801)
    } else {
        false
    } {
        Ok(Some({
            let a1802 = v4;
            let a1803 = v5;
            nat_sub(a1802, a1803)
        }))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f68(v0: Adt8, v1: u64, v2: u64, v3: u64, v4: u64) -> R<Adt9> {
    let v5: Adt3 = match v0.clone() {
        Adt8::C0(_, _, _, _, _, _, _, _, _, h1804, _, _, _, _, _) => {
            h1804
        }
    };
    let v6: Adt3 = match v0.clone() {
        Adt8::C0(_, _, _, _, _, _, _, _, h1805, _, _, _, _, _, _) => {
            h1805
        }
    };
    let v7: u64 = f65(v1, v6.clone(), 2u64);
    let a1807 = match v0.clone() {
        Adt8::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, h1806) => {
            h1806
        }
    };
    let a1808 = 0u64;
    if nat_eq(a1807, a1808) {
        Ok(Adt9::C8(Some(f64(v5, v3, v4, v7, v2)?)))
    } else {
        let v8: u64 = {
            let m1811 = match v0.clone() {
                Adt8::C0(_, _, _, _, _, _, _, _, _, _, h1810, _, _, _, _) => {
                    h1810
                }
            };
            match m1811 {
                Adt3::C0(_, _, h1809, _, _, _, _, _) => {
                    h1809
                }
            }
        };
        let m1817 = f67(match v0.clone() {
            Adt8::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, h1812, _) => {
                h1812
            }
        }, match v5.clone() {
            Adt3::C0(_, _, h1813, _, _, _, _, _) => {
                h1813
            }
        }, {
            let a1814 = v2;
            let a1815 = v8;
            let a1816 = 0u64;
            nat_rem(a1814, a1815, a1816)
        }, v7)?;
        match m1817 {
            None => {
                Ok(Adt9::C8(None::<u64>))
            }
            Some(v9) => {
                let v10: u64 = {
                    let a1824 = {
                        let a1822 = {
                            let a1818 = v2;
                            let a1819 = v8;
                            let a1820 = 0u64;
                            nat_quot(a1818, a1819, a1820)
                        };
                        let a1823 = match v0 {
                            Adt8::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, h1821) => {
                                h1821
                            }
                        };
                        nat_mul(a1822, a1823)?
                    };
                    let a1825 = f65(v1, v6, 3u64);
                    nat_add(a1824, a1825)?
                };
                Ok(Adt9::C8(Some(f64(v5, v3, v4, v9, v10)?)))
            }
        }
    }
}

pub fn f69(v0: Adt4, v1: Adt9) -> Adt9 {
    let m1826 = v1;
    match m1826 {
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
            f161(f162(v0.clone(), v20, v21), v20, v21, 0u64, v22, 2u64, f163(v0), 3u64, v23)
        }
        Adt9::C4(v24, v25, v26, v27, v28, v29, v30) => {
            let v31: u64 = f164(v0.clone(), v24);
            f161(f165(v0, v24), v31, v31, v25, v26, v27, v28, v29, v30)
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

pub fn f70(v0: u64, v1: Adt3, v2: Adt3, v3: Adt4) -> R<Adt9> {
    let m1827 = v3;
    match m1827 {
        Adt4::C0 => {
            Ok(Adt9::C1(false, f105(v0, v1)?, f105(v0, v2)?))
        }
        Adt4::C1 => {
            Ok(Adt9::C1(false, f167(v0, v1, v2.clone())?, f105(v0, v2)?))
        }
        Adt4::C2(v6, v7) => {
            f168(v0, v1, v7, v2, v6)
        }
        Adt4::C3(v8, v9, v10, v11, v12) => {
            f169(v0, v1, v2, v8, v9, v10, v11, v12)
        }
        Adt4::C4(v13) => {
            f170(v0, v1, v2, v13)
        }
        Adt4::C5(v14, v15, v16, v17) => {
            f169(v0, v1, v2, v14, v15, v16, v17, true)
        }
        Adt4::C6(v18, v19, v20) => {
            f171(v0, v1, v2, v18, v19, v20)
        }
        Adt4::C7(v21) => {
            f172(v0, v1, v2, v21)
        }
        Adt4::C8(v22) => {
            f173(v22, v0, v1, v2)
        }
        Adt4::C9(_, _) => {
            Ok(Adt9::C1(true, 0u64, f105(v0, v2)?))
        }
        Adt4::C10(v25, v26, v27) => {
            f174(v0, v1, v2, v25, v26, v27)
        }
        Adt4::C11(v67, v68) => {
            f175(v0, v1, v2, v67, v68)
        }
        Adt4::C12(v69, v70, v71) => {
            f176(v0, v1, v2, v69, v70, v71)
        }
        Adt4::C13(_, v73, _, _) => {
            Ok(Adt9::C3(v73, v73, f105(v0, v1)?, f105(v0, v2)?))
        }
        Adt4::C14(v76, v77, v78) => {
            let a1830 = v76;
            let a1831 = 23u64;
            if nat_eq(a1830, a1831) {
                f177(v0, v1, v2, v77, v78)
            } else {
                Ok(Adt9::C3(v77, v78, f105(v0, v1)?, f105(v0, v2)?))
            }
        }
        Adt4::C15 => {
            f178(v0, v1, v2)
        }
        Adt4::C16(v79, v80) => {
            f179(v0, v1, v2, v79, v80)
        }
        Adt4::C17 => {
            Ok(Adt9::C0(Adt10::C1))
        }
        Adt4::C18(v81, v82) => {
            f180(v0, v1, v82, v81)
        }
        Adt4::C19(_, _) => {
            Ok(Adt9::C0(Adt10::C1))
        }
        Adt4::C20(_, _) => {
            Ok(Adt9::C0(Adt10::C1))
        }
        Adt4::C21 => {
            Ok(Adt9::C1(false, f105(f166(v0, match v2.clone() {
                Adt3::C0(h1828, _, _, _, _, _, _, _) => {
                    h1828
                }
            }, match v2.clone() {
                Adt3::C0(_, h1829, _, _, _, _, _, _) => {
                    h1829
                }
            })?, v1)?, f105(v0, v2)?))
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
        Adt4::C25(v87, v88, v89) => {
            f181(v0, v87, v88, v89)
        }
        Adt4::C26(v95, _, _, _) => {
            let a1832 = v95;
            let a1833 = 64u64;
            if if if nat_eq(a1832, a1833) {
                true
            } else {
                let a1834 = v95;
                let a1835 = 65u64;
                nat_eq(a1834, a1835)
            } {
                true
            } else {
                let a1836 = v95;
                let a1837 = 66u64;
                nat_eq(a1836, a1837)
            } {
                Ok(Adt9::C0(Adt10::C5(64u64)))
            } else {
                Ok(Adt9::C0(Adt10::C1))
            }
        }
        Adt4::C27(v94) => {
            f96(v0, v1, v2, v94)
        }
        Adt4::C28(v90) => {
            f93(v0, v1, v2, v90, 0u64)
        }
        Adt4::C29(v91) => {
            f94(v0, v1, v2, v91)
        }
        Adt4::C30(v92, _) => {
            f95(v0, v1, v2, v92)
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
        Adt4::C48(_, _, _) => {
            Ok(Adt9::C0(Adt10::C1))
        }
    }
}

pub fn f71(v0: u64, v1: Adt3, v2: Adt3, v3: u64, v4: u64) -> R<Adt9> {
    let v5: u64 = f182(v3, v4);
    let v6: u64 = {
        let a1838 = v0;
        let a1839 = v5;
        let a1840 = 0u64;
        nat_quot(a1838, a1839, a1840)
    };
    let a1841 = v5;
    let a1842 = 1u64;
    if nat_eq(a1841, a1842) {
        Ok(f161(f183(v3, v4), v3, v4, 0u64, f105(v6, v1)?, 2u64, 0u64, 3u64, f105(v6, v2)?))
    } else {
        let a1846 = {
            let a1843 = v0;
            let a1844 = 2u64;
            let a1845 = 0u64;
            nat_rem(a1843, a1844, a1845)
        };
        let a1847 = 0u64;
        if nat_eq(a1846, a1847) {
            Ok(f161(f183(v3, 0u64), v3, 0u64, 0u64, f105(v6, v1)?, 2u64, 0u64, 1u64, 0u64))
        } else {
            Ok(f161(f183(0u64, v4), 0u64, v4, 1u64, 0u64, 2u64, 0u64, 3u64, f105(v6, v2)?))
        }
    }
}

pub fn f72(v0: u64, v1: u64, v2: u64) -> R<Adt9> {
    let v3: u64 = f184(v1)?;
    let v4: u64 = {
        let a1848 = v0;
        let a1849 = v3;
        let a1850 = 0u64;
        nat_rem(a1848, a1849, a1850)
    };
    let v5: u64 = {
        let a1853 = {
            let a1851 = v1;
            let a1852 = 3u64;
            nat_mul(a1851, a1852)?
        };
        let a1854 = 1u64;
        nat_add(a1853, a1854)?
    };
    let a1855 = v4;
    let a1856 = 0u64;
    if nat_eq(a1855, a1856) {
        Ok(Adt9::C7(Adt11::C0(0u64), Adt12::C2(0u64), Adt12::C2(0u64), Adt13::C0(0u64), 8u64, 8u64))
    } else {
        let a1857 = v4;
        let a1858 = v5;
        if nat_eq(a1857, a1858) {
            Ok(Adt9::C7(Adt11::C0(2u64), Adt12::C1(0u64), Adt12::C2(0u64), Adt13::C0(0u64), 8u64, 8u64))
        } else {
            let v6: u64 = {
                let a1863 = v4;
                let a1864 = {
                    let a1859 = v4;
                    let a1860 = v5;
                    if nat_lt(a1859, a1860) {
                        1u64
                    } else {
                        let a1861 = v5;
                        let a1862 = 1u64;
                        nat_add(a1861, a1862)?
                    }
                };
                nat_sub(a1863, a1864)
            };
            let v7: u64 = {
                let a1865 = v6;
                let a1866 = 3u64;
                let a1867 = 0u64;
                nat_rem(a1865, a1866, a1867)
            };
            let v8: u64 = {
                let a1878 = 4u64;
                let a1879 = {
                    let a1876 = {
                        let a1871 = {
                            let a1868 = v0;
                            let a1869 = v3;
                            let a1870 = 0u64;
                            nat_quot(a1868, a1869, a1870)
                        };
                        let a1872 = v1;
                        nat_mul(a1871, a1872)?
                    };
                    let a1877 = {
                        let a1873 = v6;
                        let a1874 = 3u64;
                        let a1875 = 0u64;
                        nat_quot(a1873, a1874, a1875)
                    };
                    nat_add(a1876, a1877)?
                };
                nat_mul(a1878, a1879)?
            };
            let a1880 = v4;
            let a1881 = v5;
            if nat_lt(a1880, a1881) {
                let a1882 = v7;
                let a1883 = 0u64;
                if nat_eq(a1882, a1883) {
                    Ok(Adt9::C7(Adt11::C1(2u64), Adt12::C0(1u64, v8), Adt12::C0(0u64, v8), Adt13::C0(1u64), 4u64, 4u64))
                } else {
                    let a1884 = v7;
                    let a1885 = 1u64;
                    if nat_eq(a1884, a1885) {
                        Ok(Adt9::C7(Adt11::C0(1u64), Adt12::C1(1u64), Adt12::C2(0u64), Adt13::C0(1u64), 4u64, 4u64))
                    } else {
                        Ok(Adt9::C7(Adt11::C0(0u64), Adt12::C1(0u64), Adt12::C1(1u64), Adt13::C0(0u64), 8u64, 8u64))
                    }
                }
            } else {
                let a1886 = v7;
                let a1887 = 0u64;
                if nat_eq(a1886, a1887) {
                    Ok(Adt9::C7(Adt11::C1(1u64), Adt12::C0(0u64, v8), Adt12::C1(0u64), Adt13::C0(1u64), 4u64, 4u64))
                } else {
                    let a1888 = v7;
                    let a1889 = 1u64;
                    if nat_eq(a1888, a1889) {
                        Ok(Adt9::C7(Adt11::C1(2u64), Adt12::C2(v2), Adt12::C1(1u64), Adt13::C0(1u64), 4u64, 4u64))
                    } else {
                        Ok(Adt9::C7(Adt11::C1(2u64), Adt12::C1(1u64), Adt12::C0(1u64, v8), Adt13::C1(v8), 4u64, 4u64))
                    }
                }
            }
        }
    }
}

pub fn f73(v0: u64, v1: Adt3, v2: Adt3, v3: Adt3) -> R<Adt9> {
    let v4: u64 = {
        let a1890 = v0;
        let a1891 = 6u64;
        let a1892 = 0u64;
        nat_quot(a1890, a1891, a1892)
    };
    let v5: u64 = {
        let a1893 = v0;
        let a1894 = 6u64;
        let a1895 = 0u64;
        nat_rem(a1893, a1894, a1895)
    };
    let a1896 = v5;
    let a1897 = 0u64;
    if nat_eq(a1896, a1897) {
        Ok(Adt9::C7(Adt11::C7(17u64, 0u64, 0u64), Adt12::C0(1u64, f105(v4, v2)?), Adt12::C2(0u64), Adt13::C0(0u64), 4u64, 4u64))
    } else {
        let a1898 = v5;
        let a1899 = 1u64;
        if nat_eq(a1898, a1899) {
            Ok(Adt9::C7(Adt11::C1(1u64), Adt12::C2(1065353216u64), Adt12::C1(0u64), Adt13::C0(1u64), 4u64, 4u64))
        } else {
            let a1900 = v5;
            let a1901 = 2u64;
            if nat_eq(a1900, a1901) {
                Ok(Adt9::C7(Adt11::C1(2u64), Adt12::C0(1u64, f105(v4, v2)?), Adt12::C1(1u64), Adt13::C0(1u64), 4u64, 4u64))
            } else {
                let a1902 = v5;
                let a1903 = 3u64;
                if nat_eq(a1902, a1903) {
                    Ok(Adt9::C7(Adt11::C1(0u64), Adt12::C2(1065353216u64), Adt12::C1(1u64), Adt13::C0(1u64), 4u64, 4u64))
                } else {
                    let a1904 = v5;
                    let a1905 = 4u64;
                    if nat_eq(a1904, a1905) {
                        Ok(Adt9::C7(Adt11::C1(2u64), Adt12::C0(0u64, f105(v4, v1)?), Adt12::C1(0u64), Adt13::C0(0u64), 4u64, 4u64))
                    } else {
                        Ok(Adt9::C7(Adt11::C1(2u64), Adt12::C1(0u64), Adt12::C1(1u64), Adt13::C1(f105(v4, v3)?), 4u64, 4u64))
                    }
                }
            }
        }
    }
}

pub fn f74(v0: u64, v1: Adt3, v2: Adt3, v3: Adt3, v4: bool, v5: u64) -> R<Adt9> {
    let v6: u64 = if v4 {
        3u64
    } else {
        2u64
    };
    let v7: u64 = f185(match v1.clone() {
        Adt3::C0(_, h1906, _, _, _, _, _, _) => {
            h1906
        }
    }, v4)?;
    let v8: u64 = {
        let a1907 = v0;
        let a1908 = v7;
        let a1909 = 0u64;
        nat_rem(a1907, a1908, a1909)
    };
    let v9: u64 = {
        let a1910 = v0;
        let a1911 = v7;
        let a1912 = 0u64;
        nat_quot(a1910, a1911, a1912)
    };
    let a1915 = {
        let a1913 = v8;
        let a1914 = 1u64;
        nat_add(a1913, a1914)?
    };
    let a1916 = v7;
    if nat_eq(a1915, a1916) {
        Ok(Adt9::C7(Adt11::C1(2u64), Adt12::C2(v5), Adt12::C1(0u64), Adt13::C1(f105(v9, v3)?), 4u64, 4u64))
    } else {
        let v10: u64 = {
            let a1917 = v8;
            let a1918 = v6;
            let a1919 = 0u64;
            nat_quot(a1917, a1918, a1919)
        };
        let v11: u64 = {
            let a1920 = v8;
            let a1921 = v6;
            let a1922 = 0u64;
            nat_rem(a1920, a1921, a1922)
        };
        let v12: u64 = {
            let a1924 = v9;
            let a1925 = match v1.clone() {
                Adt3::C0(h1923, _, _, _, _, _, _, _) => {
                    h1923
                }
            };
            let a1926 = 0u64;
            nat_rem(a1924, a1925, a1926)
        };
        let v13: u64 = {
            let a1928 = v9;
            let a1929 = match v1.clone() {
                Adt3::C0(h1927, _, _, _, _, _, _, _) => {
                    h1927
                }
            };
            let a1930 = 0u64;
            nat_quot(a1928, a1929, a1930)
        };
        let v14: u64 = if v4 {
            let a1942 = {
                let a1937 = {
                    let a1932 = match v2.clone() {
                        Adt3::C0(_, _, _, _, h1931, _, _, _) => {
                            h1931
                        }
                    };
                    let a1933 = v12;
                    nat_mul(a1932, a1933)?
                };
                let a1938 = {
                    let a1935 = match v2.clone() {
                        Adt3::C0(_, _, _, _, _, h1934, _, _) => {
                            h1934
                        }
                    };
                    let a1936 = v10;
                    nat_mul(a1935, a1936)?
                };
                nat_add(a1937, a1938)?
            };
            let a1943 = {
                let a1940 = match v2 {
                    Adt3::C0(_, _, _, _, _, _, h1939, _) => {
                        h1939
                    }
                };
                let a1941 = v13;
                nat_mul(a1940, a1941)?
            };
            nat_add(a1942, a1943)?
        } else {
            let a1950 = {
                let a1945 = match v2.clone() {
                    Adt3::C0(_, _, _, _, h1944, _, _, _) => {
                        h1944
                    }
                };
                let a1946 = v10;
                nat_mul(a1945, a1946)?
            };
            let a1951 = {
                let a1948 = match v2 {
                    Adt3::C0(_, _, _, _, _, h1947, _, _) => {
                        h1947
                    }
                };
                let a1949 = v13;
                nat_mul(a1948, a1949)?
            };
            nat_add(a1950, a1951)?
        };
        if if v4 {
            let a1952 = v11;
            let a1953 = 0u64;
            nat_eq(a1952, a1953)
        } else {
            false
        } {
            Ok(Adt9::C7(Adt11::C7(17u64, 0u64, 0u64), Adt12::C0(1u64, v14), Adt12::C2(0u64), Adt13::C0(1u64), 4u64, 4u64))
        } else {
            let a1956 = {
                let a1954 = v11;
                let a1955 = 1u64;
                nat_add(a1954, a1955)?
            };
            let a1957 = v6;
            if nat_eq(a1956, a1957) {
                Ok(Adt9::C7(Adt11::C1(0u64), {
                    let a1958 = v10;
                    let a1959 = 0u64;
                    if nat_eq(a1958, a1959) {
                        Adt12::C2(0u64)
                    } else {
                        Adt12::C1(0u64)
                    }
                }, Adt12::C1(1u64), Adt13::C0(0u64), 4u64, 4u64))
            } else {
                Ok(Adt9::C7(Adt11::C1(2u64), Adt12::C0(0u64, {
                    let a1971 = {
                        let a1966 = {
                            let a1961 = match v1.clone() {
                                Adt3::C0(_, _, _, _, h1960, _, _, _) => {
                                    h1960
                                }
                            };
                            let a1962 = v12;
                            nat_mul(a1961, a1962)?
                        };
                        let a1967 = {
                            let a1964 = match v1.clone() {
                                Adt3::C0(_, _, _, _, _, h1963, _, _) => {
                                    h1963
                                }
                            };
                            let a1965 = v10;
                            nat_mul(a1964, a1965)?
                        };
                        nat_add(a1966, a1967)?
                    };
                    let a1972 = {
                        let a1969 = match v1 {
                            Adt3::C0(_, _, _, _, _, _, h1968, _) => {
                                h1968
                            }
                        };
                        let a1970 = v13;
                        nat_mul(a1969, a1970)?
                    };
                    nat_add(a1971, a1972)?
                }), if v4 {
                    Adt12::C1(1u64)
                } else {
                    Adt12::C0(1u64, v14)
                }, Adt13::C0(1u64), 4u64, 4u64))
            }
        }
    }
}

pub fn f75(v0: u64, v1: Adt3, v2: Adt3, v3: Adt3, v4: Option<Adt3>, v5: Adt3) -> R<Adt9> {
    let v6: u64 = f40(match v2.clone() {
        Adt3::C0(_, h1973, _, _, _, _, _, _) => {
            h1973
        }
    }, v4.clone())?;
    let v7: u64 = {
        let a1974 = v0;
        let a1975 = v6;
        let a1976 = 0u64;
        nat_rem(a1974, a1975, a1976)
    };
    let v8: u64 = {
        let a1977 = v0;
        let a1978 = v6;
        let a1979 = 0u64;
        nat_quot(a1977, a1978, a1979)
    };
    let v9: u64 = {
        let a1981 = v8;
        let a1982 = match v5.clone() {
            Adt3::C0(h1980, _, _, _, _, _, _, _) => {
                h1980
            }
        };
        let a1983 = 0u64;
        nat_rem(a1981, a1982, a1983)
    };
    let v10: u64 = {
        let a1985 = v8;
        let a1986 = match v5.clone() {
            Adt3::C0(h1984, _, _, _, _, _, _, _) => {
                h1984
            }
        };
        let a1987 = 0u64;
        nat_quot(a1985, a1986, a1987)
    };
    let v11: u64 = {
        let a1989 = v10;
        let a1990 = match v2.clone() {
            Adt3::C0(_, h1988, _, _, _, _, _, _) => {
                h1988
            }
        };
        let a1991 = 0u64;
        nat_rem(a1989, a1990, a1991)
    };
    let v12: u64 = {
        let a1993 = v10;
        let a1994 = match v2.clone() {
            Adt3::C0(_, h1992, _, _, _, _, _, _) => {
                h1992
            }
        };
        let a1995 = 0u64;
        nat_quot(a1993, a1994, a1995)
    };
    let a1996 = v7;
    let a1997 = 0u64;
    if nat_eq(a1996, a1997) {
        Ok(Adt9::C7(Adt11::C1(2u64), Adt12::C0(0u64, {
            let a2004 = {
                let a1999 = match v1.clone() {
                    Adt3::C0(_, _, _, _, h1998, _, _, _) => {
                        h1998
                    }
                };
                let a2000 = v9;
                nat_mul(a1999, a2000)?
            };
            let a2005 = {
                let a2002 = match v1 {
                    Adt3::C0(_, _, _, _, _, h2001, _, _) => {
                        h2001
                    }
                };
                let a2003 = v12;
                nat_mul(a2002, a2003)?
            };
            nat_add(a2004, a2005)?
        }), Adt12::C0(2u64, {
            let a2012 = {
                let a2007 = match v3.clone() {
                    Adt3::C0(_, _, _, _, h2006, _, _, _) => {
                        h2006
                    }
                };
                let a2008 = v11;
                nat_mul(a2007, a2008)?
            };
            let a2013 = {
                let a2010 = match v3 {
                    Adt3::C0(_, _, _, _, _, h2009, _, _) => {
                        h2009
                    }
                };
                let a2011 = v12;
                nat_mul(a2010, a2011)?
            };
            nat_add(a2012, a2013)?
        }), Adt13::C0(0u64), 4u64, 4u64))
    } else {
        let m2014 = v4;
        match m2014 {
            None => {
                Ok(Adt9::C7(Adt11::C1(0u64), Adt12::C1(0u64), Adt12::C0(1u64, {
                    let a2026 = {
                        let a2021 = {
                            let a2016 = match v2.clone() {
                                Adt3::C0(_, _, _, _, h2015, _, _, _) => {
                                    h2015
                                }
                            };
                            let a2017 = v9;
                            nat_mul(a2016, a2017)?
                        };
                        let a2022 = {
                            let a2019 = match v2.clone() {
                                Adt3::C0(_, _, _, _, _, h2018, _, _) => {
                                    h2018
                                }
                            };
                            let a2020 = v11;
                            nat_mul(a2019, a2020)?
                        };
                        nat_add(a2021, a2022)?
                    };
                    let a2027 = {
                        let a2024 = match v2 {
                            Adt3::C0(_, _, _, _, _, _, h2023, _) => {
                                h2023
                            }
                        };
                        let a2025 = v12;
                        nat_mul(a2024, a2025)?
                    };
                    nat_add(a2026, a2027)?
                }), Adt13::C1(f105(v8, v5)?), 4u64, 4u64))
            }
            Some(v13) => {
                let v14: u64 = {
                    let a2028 = v7;
                    let a2029 = 1u64;
                    nat_sub(a2028, a2029)
                };
                let v15: u64 = {
                    let a2030 = v14;
                    let a2031 = 2u64;
                    let a2032 = 0u64;
                    nat_quot(a2030, a2031, a2032)
                };
                let a2036 = {
                    let a2033 = v14;
                    let a2034 = 2u64;
                    let a2035 = 0u64;
                    nat_rem(a2033, a2034, a2035)
                };
                let a2037 = 0u64;
                if nat_eq(a2036, a2037) {
                    Ok(Adt9::C7(Adt11::C1(2u64), Adt12::C0(1u64, {
                        let a2049 = {
                            let a2044 = {
                                let a2039 = match v2.clone() {
                                    Adt3::C0(_, _, _, _, h2038, _, _, _) => {
                                        h2038
                                    }
                                };
                                let a2040 = v9;
                                nat_mul(a2039, a2040)?
                            };
                            let a2045 = {
                                let a2042 = match v2.clone() {
                                    Adt3::C0(_, _, _, _, _, h2041, _, _) => {
                                        h2041
                                    }
                                };
                                let a2043 = v15;
                                nat_mul(a2042, a2043)?
                            };
                            nat_add(a2044, a2045)?
                        };
                        let a2050 = {
                            let a2047 = match v2 {
                                Adt3::C0(_, _, _, _, _, _, h2046, _) => {
                                    h2046
                                }
                            };
                            let a2048 = v12;
                            nat_mul(a2047, a2048)?
                        };
                        nat_add(a2049, a2050)?
                    }), Adt12::C0(3u64, {
                        let a2062 = {
                            let a2057 = {
                                let a2052 = match v13.clone() {
                                    Adt3::C0(_, _, _, _, h2051, _, _, _) => {
                                        h2051
                                    }
                                };
                                let a2053 = v11;
                                nat_mul(a2052, a2053)?
                            };
                            let a2058 = {
                                let a2055 = match v13.clone() {
                                    Adt3::C0(_, _, _, _, _, h2054, _, _) => {
                                        h2054
                                    }
                                };
                                let a2056 = v15;
                                nat_mul(a2055, a2056)?
                            };
                            nat_add(a2057, a2058)?
                        };
                        let a2063 = {
                            let a2060 = match v13 {
                                Adt3::C0(_, _, _, _, _, _, h2059, _) => {
                                    h2059
                                }
                            };
                            let a2061 = v12;
                            nat_mul(a2060, a2061)?
                        };
                        nat_add(a2062, a2063)?
                    }), Adt13::C0(1u64), 4u64, 4u64))
                } else {
                    Ok(Adt9::C7(Adt11::C1(0u64), Adt12::C1(0u64), Adt12::C1(1u64), {
                        let a2066 = {
                            let a2064 = v7;
                            let a2065 = 1u64;
                            nat_add(a2064, a2065)?
                        };
                        let a2067 = v6;
                        if nat_eq(a2066, a2067) {
                            Adt13::C1(f105(v8, v5)?)
                        } else {
                            Adt13::C0(0u64)
                        }
                    }, 4u64, 4u64))
                }
            }
        }
    }
}

pub fn f76(v0: u64, v1: Adt3, v2: Adt3, v3: Adt3, v4: Adt3, v5: u64, v6: u64, v7: u64) -> R<Adt9> {
    let v8: u64 = f39(v6)?;
    let v9: u64 = {
        let a2068 = v0;
        let a2069 = v8;
        let a2070 = 0u64;
        nat_rem(a2068, a2069, a2070)
    };
    let v10: u64 = {
        let a2071 = v0;
        let a2072 = v8;
        let a2073 = 0u64;
        nat_quot(a2071, a2072, a2073)
    };
    let a2074 = v9;
    let a2075 = 148u64;
    if nat_lt(a2074, a2075) {
        f186({
            let a2076 = v9;
            let a2077 = 37u64;
            let a2078 = 0u64;
            nat_rem(a2076, a2077, a2078)
        }, {
            let a2079 = v9;
            let a2080 = 37u64;
            let a2081 = 0u64;
            nat_quot(a2079, a2080, a2081)
        }, v10, v1, v2, v3, v4, v5, v7)
    } else {
        f187({
            let a2082 = v9;
            let a2083 = 148u64;
            nat_sub(a2082, a2083)
        }, v10, v4, v5)
    }
}

pub fn f77(v0: u64, v1: Adt3, v2: bool) -> R<Adt9> {
    let v3: u64 = f36(v2);
    let a2084 = v0;
    let a2085 = v3;
    if nat_lt(a2084, a2085) {
        let a2086 = v0;
        let a2087 = if v2 {
            7u64
        } else {
            2u64
        };
        if nat_lt(a2086, a2087) {
            Ok(Adt9::C7(Adt11::C5(3u64), Adt12::C0(if v2 {
                4u64
            } else {
                2u64
            }, {
                let a2088 = 4u64;
                let a2089 = v0;
                nat_mul(a2088, a2089)?
            }), Adt12::C2(0u64), Adt13::C0(v0), 4u64, 4u64))
        } else {
            let a2090 = v0;
            let a2091 = if v2 {
                7u64
            } else {
                2u64
            };
            if nat_eq(a2090, a2091) {
                Ok(Adt9::C7(Adt11::C1(2u64), Adt12::C1(0u64), Adt12::C1(if v2 {
                    4u64
                } else {
                    1u64
                }), Adt13::C0(7u64), 4u64, 4u64))
            } else {
                let a2094 = v0;
                let a2095 = {
                    let a2092 = if v2 {
                        7u64
                    } else {
                        2u64
                    };
                    let a2093 = 1u64;
                    nat_add(a2092, a2093)?
                };
                if nat_eq(a2094, a2095) {
                    Ok(Adt9::C7(Adt11::C1(1u64), Adt12::C2(1065353216u64), Adt12::C1(7u64), Adt13::C0(7u64), 4u64, 4u64))
                } else {
                    let a2096 = v0;
                    let a2097 = 9u64;
                    if nat_eq(a2096, a2097) {
                        Ok(Adt9::C7(Adt11::C1(1u64), Adt12::C2(1065353216u64), Adt12::C1(1u64), Adt13::C0(8u64), 4u64, 4u64))
                    } else {
                        Ok(Adt9::C7(Adt11::C1(1u64), Adt12::C2(1065353216u64), Adt12::C1(2u64), Adt13::C0(9u64), 4u64, 4u64))
                    }
                }
            }
        }
    } else {
        let v4: u64 = {
            let a2098 = v0;
            let a2099 = v3;
            nat_sub(a2098, a2099)
        };
        let v5: u64 = f37(v2);
        let v6: u64 = {
            let a2100 = v4;
            let a2101 = v5;
            let a2102 = 0u64;
            nat_rem(a2100, a2101, a2102)
        };
        let v7: u64 = f105({
            let a2103 = v4;
            let a2104 = v5;
            let a2105 = 0u64;
            nat_quot(a2103, a2104, a2105)
        }, v1)?;
        if v2 {
            let a2106 = v6;
            let a2107 = 7u64;
            if nat_lt(a2106, a2107) {
                let a2108 = v6;
                let a2109 = 3u64;
                if nat_lt(a2108, a2109) {
                    let a2110 = v6;
                    let a2111 = 1u64;
                    if nat_lt(a2110, a2111) {
                        Ok(Adt9::C7(Adt11::C1(2u64), Adt12::C0(2u64, v7), Adt12::C1(1u64), Adt13::C0(10u64), 4u64, 4u64))
                    } else {
                        let a2112 = v6;
                        let a2113 = 2u64;
                        if nat_lt(a2112, a2113) {
                            Ok(Adt9::C7(Adt11::C1(2u64), Adt12::C0(1u64, v7), Adt12::C1(8u64), Adt13::C0(11u64), 4u64, 4u64))
                        } else {
                            Ok(Adt9::C7(Adt11::C1(0u64), Adt12::C1(10u64), Adt12::C1(11u64), Adt13::C3(2u64, v7), 4u64, 4u64))
                        }
                    }
                } else {
                    let a2114 = v6;
                    let a2115 = 5u64;
                    if nat_lt(a2114, a2115) {
                        let a2116 = v6;
                        let a2117 = 4u64;
                        if nat_lt(a2116, a2117) {
                            Ok(Adt9::C7(Adt11::C1(2u64), Adt12::C0(3u64, v7), Adt12::C1(2u64), Adt13::C0(10u64), 4u64, 4u64))
                        } else {
                            Ok(Adt9::C7(Adt11::C1(2u64), Adt12::C0(1u64, v7), Adt12::C0(1u64, v7), Adt13::C0(11u64), 4u64, 4u64))
                        }
                    } else {
                        let a2118 = v6;
                        let a2119 = 6u64;
                        if nat_lt(a2118, a2119) {
                            Ok(Adt9::C7(Adt11::C1(2u64), Adt12::C1(11u64), Adt12::C1(9u64), Adt13::C0(11u64), 4u64, 4u64))
                        } else {
                            Ok(Adt9::C7(Adt11::C1(0u64), Adt12::C1(10u64), Adt12::C1(11u64), Adt13::C3(3u64, v7), 4u64, 4u64))
                        }
                    }
                }
            } else {
                let a2120 = v6;
                let a2121 = 11u64;
                if nat_lt(a2120, a2121) {
                    let a2122 = v6;
                    let a2123 = 9u64;
                    if nat_lt(a2122, a2123) {
                        let a2124 = v6;
                        let a2125 = 8u64;
                        if nat_lt(a2124, a2125) {
                            Ok(Adt9::C7(Adt11::C1(2u64), Adt12::C0(2u64, v7), Adt12::C1(5u64), Adt13::C0(10u64), 4u64, 4u64))
                        } else {
                            Ok(Adt9::C7(Adt11::C1(2u64), Adt12::C0(3u64, v7), Adt12::C1(6u64), Adt13::C0(11u64), 4u64, 4u64))
                        }
                    } else {
                        let a2126 = v6;
                        let a2127 = 10u64;
                        if nat_lt(a2126, a2127) {
                            Ok(Adt9::C7(Adt11::C1(4u64), Adt12::C1(11u64), Adt12::C2(0u64), Adt13::C0(11u64), 4u64, 4u64))
                        } else {
                            Ok(Adt9::C7(Adt11::C1(0u64), Adt12::C1(11u64), Adt12::C1(3u64), Adt13::C0(11u64), 4u64, 4u64))
                        }
                    }
                } else {
                    let a2128 = v6;
                    let a2129 = 13u64;
                    if nat_lt(a2128, a2129) {
                        let a2130 = v6;
                        let a2131 = 12u64;
                        if nat_lt(a2130, a2131) {
                            Ok(Adt9::C7(Adt11::C1(2u64), Adt12::C1(0u64), Adt12::C1(10u64), Adt13::C0(10u64), 4u64, 4u64))
                        } else {
                            Ok(Adt9::C7(Adt11::C1(3u64), Adt12::C1(10u64), Adt12::C1(11u64), Adt13::C0(10u64), 4u64, 4u64))
                        }
                    } else {
                        let a2132 = v6;
                        let a2133 = 14u64;
                        if nat_lt(a2132, a2133) {
                            Ok(Adt9::C7(Adt11::C1(2u64), Adt12::C0(0u64, v7), Adt12::C1(7u64), Adt13::C0(11u64), 4u64, 4u64))
                        } else {
                            Ok(Adt9::C7(Adt11::C1(1u64), Adt12::C1(11u64), Adt12::C1(10u64), Adt13::C1(v7), 4u64, 4u64))
                        }
                    }
                }
            }
        } else {
            let a2134 = v6;
            let a2135 = 1u64;
            if nat_lt(a2134, a2135) {
                Ok(Adt9::C7(Adt11::C1(2u64), Adt12::C0(0u64, v7), Adt12::C1(7u64), Adt13::C0(10u64), 4u64, 4u64))
            } else {
                let a2136 = v6;
                let a2137 = 2u64;
                if nat_lt(a2136, a2137) {
                    Ok(Adt9::C7(Adt11::C1(2u64), Adt12::C1(0u64), Adt12::C0(1u64, v7), Adt13::C0(11u64), 4u64, 4u64))
                } else {
                    Ok(Adt9::C7(Adt11::C1(1u64), Adt12::C1(10u64), Adt12::C1(11u64), Adt13::C1(v7), 4u64, 4u64))
                }
            }
        }
    }
}

pub fn f78(v0: u64, v1: Adt3, v2: Adt3, v3: bool, v4: u64, v5: u64, v6: u64) -> R<Adt9> {
    let v7: u64 = match v1.clone() {
        Adt3::C0(h2138, _, _, _, _, _, _, _) => {
            h2138
        }
    };
    let v8: u64 = f34(v3, v7)?;
    let v9: u64 = {
        let a2139 = v0;
        let a2140 = 2u64;
        nat_sub(a2139, a2140)
    };
    let v10: u64 = {
        let a2141 = v9;
        let a2142 = v8;
        let a2143 = 0u64;
        nat_quot(a2141, a2142, a2143)
    };
    let v11: u64 = {
        let a2144 = v9;
        let a2145 = v8;
        let a2146 = 0u64;
        nat_rem(a2144, a2145, a2146)
    };
    let a2147 = v0;
    let a2148 = 0u64;
    if nat_eq(a2147, a2148) {
        Ok(Adt9::C7(Adt11::C0(3u64), Adt12::C2(v5), Adt12::C2(0u64), Adt13::C0(6u64), 4u64, 4u64))
    } else {
        let a2149 = v0;
        let a2150 = 1u64;
        if nat_eq(a2149, a2150) {
            Ok(Adt9::C7(Adt11::C1(3u64), if v3 {
                Adt12::C0(0u64, 0u64)
            } else {
                Adt12::C2(3212836864u64)
            }, Adt12::C1(6u64), Adt13::C0(6u64), 4u64, 4u64))
        } else {
            let a2151 = v10;
            let a2152 = v5;
            if nat_eq(a2151, a2152) {
                Ok(Adt9::C7(Adt11::C1(2u64), Adt12::C1(5u64), Adt12::C1(6u64), Adt13::C1(0u64), 4u64, 4u64))
            } else {
                let a2155 = v11;
                let a2156 = {
                    let a2153 = 2u64;
                    let a2154 = v7;
                    nat_mul(a2153, a2154)?
                };
                if nat_lt(a2155, a2156) {
                    let v12: u64 = {
                        let a2157 = v11;
                        let a2158 = 2u64;
                        let a2159 = 0u64;
                        nat_quot(a2157, a2158, a2159)
                    };
                    let a2163 = {
                        let a2160 = v11;
                        let a2161 = 2u64;
                        let a2162 = 0u64;
                        nat_rem(a2160, a2161, a2162)
                    };
                    let a2164 = 0u64;
                    if nat_eq(a2163, a2164) {
                        Ok(Adt9::C7(Adt11::C5(0u64), {
                            let a2165 = v12;
                            let a2166 = 0u64;
                            if nat_eq(a2165, a2166) {
                                Adt12::C2(4286578688u64)
                            } else {
                                Adt12::C1(0u64)
                            }
                        }, Adt12::C0(if v3 {
                            1u64
                        } else {
                            0u64
                        }, {
                            let a2172 = {
                                let a2168 = v10;
                                let a2169 = match v1 {
                                    Adt3::C0(_, _, _, _, _, h2167, _, _) => {
                                        h2167
                                    }
                                };
                                nat_mul(a2168, a2169)?
                            };
                            let a2173 = {
                                let a2170 = 4u64;
                                let a2171 = v12;
                                nat_mul(a2170, a2171)?
                            };
                            nat_add(a2172, a2173)?
                        }), Adt13::C0(2u64), 4u64, 4u64))
                    } else {
                        Ok(Adt9::C7(Adt11::C5(3u64), {
                            let a2174 = v6;
                            let a2175 = 2u64;
                            if nat_eq(a2174, a2175) {
                                let a2176 = v12;
                                let a2177 = 0u64;
                                if nat_eq(a2176, a2177) {
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
                                    let a2183 = {
                                        let a2179 = v10;
                                        let a2180 = match v1 {
                                            Adt3::C0(_, _, _, _, _, h2178, _, _) => {
                                                h2178
                                            }
                                        };
                                        nat_mul(a2179, a2180)?
                                    };
                                    let a2184 = {
                                        let a2181 = 4u64;
                                        let a2182 = v12;
                                        nat_mul(a2181, a2182)?
                                    };
                                    nat_add(a2183, a2184)?
                                })
                            }
                        }, Adt12::C2(0u64), Adt13::C0(0u64), 4u64, 4u64))
                    }
                } else {
                    let a2187 = v11;
                    let a2188 = {
                        let a2185 = 6u64;
                        let a2186 = v7;
                        nat_mul(a2185, a2186)?
                    };
                    if nat_lt(a2187, a2188) {
                        let v13: u64 = {
                            let a2191 = v11;
                            let a2192 = {
                                let a2189 = 2u64;
                                let a2190 = v7;
                                nat_mul(a2189, a2190)?
                            };
                            nat_sub(a2191, a2192)
                        };
                        let v14: u64 = {
                            let a2193 = v13;
                            let a2194 = 4u64;
                            let a2195 = 0u64;
                            nat_quot(a2193, a2194, a2195)
                        };
                        let a2199 = {
                            let a2196 = v13;
                            let a2197 = 4u64;
                            let a2198 = 0u64;
                            nat_rem(a2196, a2197, a2198)
                        };
                        let a2200 = 2u64;
                        if nat_lt(a2199, a2200) {
                            let a2204 = {
                                let a2201 = v13;
                                let a2202 = 4u64;
                                let a2203 = 0u64;
                                nat_rem(a2201, a2202, a2203)
                            };
                            let a2205 = 1u64;
                            if nat_lt(a2204, a2205) {
                                Ok(Adt9::C7(Adt11::C1(1u64), Adt12::C0(if v3 {
                                    1u64
                                } else {
                                    0u64
                                }, {
                                    let a2211 = {
                                        let a2207 = v10;
                                        let a2208 = match v1 {
                                            Adt3::C0(_, _, _, _, _, h2206, _, _) => {
                                                h2206
                                            }
                                        };
                                        nat_mul(a2207, a2208)?
                                    };
                                    let a2212 = {
                                        let a2209 = 4u64;
                                        let a2210 = v14;
                                        nat_mul(a2209, a2210)?
                                    };
                                    nat_add(a2211, a2212)?
                                }), Adt12::C1(0u64), Adt13::C0(3u64), 4u64, 4u64))
                            } else {
                                Ok(Adt9::C7(Adt11::C7(16u64, 0u64, 0u64), Adt12::C1(3u64), Adt12::C2(0u64), if v3 {
                                    Adt13::C1({
                                        let a2218 = {
                                            let a2214 = v10;
                                            let a2215 = match v2 {
                                                Adt3::C0(_, _, _, _, _, h2213, _, _) => {
                                                    h2213
                                                }
                                            };
                                            nat_mul(a2214, a2215)?
                                        };
                                        let a2219 = {
                                            let a2216 = 4u64;
                                            let a2217 = v14;
                                            nat_mul(a2216, a2217)?
                                        };
                                        nat_add(a2218, a2219)?
                                    })
                                } else {
                                    Adt13::C0(3u64)
                                }, 4u64, 4u64))
                            }
                        } else {
                            let a2223 = {
                                let a2220 = v13;
                                let a2221 = 4u64;
                                let a2222 = 0u64;
                                nat_rem(a2220, a2221, a2222)
                            };
                            let a2224 = 3u64;
                            if nat_lt(a2223, a2224) {
                                Ok(Adt9::C7(Adt11::C0(1u64), if v3 {
                                    Adt12::C3({
                                        let a2230 = {
                                            let a2226 = v10;
                                            let a2227 = match v2 {
                                                Adt3::C0(_, _, _, _, _, h2225, _, _) => {
                                                    h2225
                                                }
                                            };
                                            nat_mul(a2226, a2227)?
                                        };
                                        let a2231 = {
                                            let a2228 = 4u64;
                                            let a2229 = v14;
                                            nat_mul(a2228, a2229)?
                                        };
                                        nat_add(a2230, a2231)?
                                    })
                                } else {
                                    Adt12::C1(3u64)
                                }, Adt12::C2(0u64), Adt13::C0(3u64), 4u64, 4u64))
                            } else {
                                Ok(Adt9::C7(Adt11::C0(0u64), {
                                    let a2232 = v14;
                                    let a2233 = 0u64;
                                    if nat_eq(a2232, a2233) {
                                        Adt12::C2(0u64)
                                    } else {
                                        Adt12::C1(1u64)
                                    }
                                }, Adt12::C1(3u64), Adt13::C0(1u64), 4u64, 4u64))
                            }
                        }
                    } else {
                        let a2238 = v11;
                        let a2239 = {
                            let a2236 = {
                                let a2234 = 6u64;
                                let a2235 = v7;
                                nat_mul(a2234, a2235)?
                            };
                            let a2237 = 2u64;
                            nat_add(a2236, a2237)?
                        };
                        if nat_lt(a2238, a2239) {
                            let a2242 = v11;
                            let a2243 = {
                                let a2240 = 6u64;
                                let a2241 = v7;
                                nat_mul(a2240, a2241)?
                            };
                            if nat_eq(a2242, a2243) {
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
                                    let a2248 = v11;
                                    let a2249 = {
                                        let a2246 = {
                                            let a2244 = 6u64;
                                            let a2245 = v7;
                                            nat_mul(a2244, a2245)?
                                        };
                                        let a2247 = 2u64;
                                        nat_add(a2246, a2247)?
                                    };
                                    nat_sub(a2248, a2249)
                                };
                                let v16: u64 = {
                                    let a2250 = v15;
                                    let a2251 = 3u64;
                                    let a2252 = 0u64;
                                    nat_quot(a2250, a2251, a2252)
                                };
                                let a2256 = {
                                    let a2253 = v15;
                                    let a2254 = 3u64;
                                    let a2255 = 0u64;
                                    nat_rem(a2253, a2254, a2255)
                                };
                                let a2257 = 1u64;
                                if nat_lt(a2256, a2257) {
                                    Ok(Adt9::C7(Adt11::C1(2u64), Adt12::C3({
                                        let a2263 = {
                                            let a2259 = v10;
                                            let a2260 = match v2 {
                                                Adt3::C0(_, _, _, _, _, h2258, _, _) => {
                                                    h2258
                                                }
                                            };
                                            nat_mul(a2259, a2260)?
                                        };
                                        let a2264 = {
                                            let a2261 = 4u64;
                                            let a2262 = v16;
                                            nat_mul(a2261, a2262)?
                                        };
                                        nat_add(a2263, a2264)?
                                    }), Adt12::C1(1u64), Adt13::C0(3u64), 4u64, 4u64))
                                } else {
                                    let a2268 = {
                                        let a2265 = v15;
                                        let a2266 = 3u64;
                                        let a2267 = 0u64;
                                        nat_rem(a2265, a2266, a2267)
                                    };
                                    let a2269 = 2u64;
                                    if nat_lt(a2268, a2269) {
                                        Ok(Adt9::C7(Adt11::C1(1u64), Adt12::C1(3u64), Adt12::C0(if v3 {
                                            2u64
                                        } else {
                                            1u64
                                        }, {
                                            let a2274 = {
                                                let a2270 = v10;
                                                let a2271 = v4;
                                                nat_mul(a2270, a2271)?
                                            };
                                            let a2275 = {
                                                let a2272 = 4u64;
                                                let a2273 = v16;
                                                nat_mul(a2272, a2273)?
                                            };
                                            nat_add(a2274, a2275)?
                                        }), Adt13::C0(3u64), 4u64, 4u64))
                                    } else {
                                        Ok(Adt9::C7(Adt11::C1(2u64), Adt12::C1(3u64), Adt12::C1(6u64), Adt13::C1({
                                            let a2281 = {
                                                let a2277 = v10;
                                                let a2278 = match v2 {
                                                    Adt3::C0(_, _, _, _, _, h2276, _, _) => {
                                                        h2276
                                                    }
                                                };
                                                nat_mul(a2277, a2278)?
                                            };
                                            let a2282 = {
                                                let a2279 = 4u64;
                                                let a2280 = v16;
                                                nat_mul(a2279, a2280)?
                                            };
                                            nat_add(a2281, a2282)?
                                        }), 4u64, 4u64))
                                    }
                                }
                            } else {
                                let a2287 = v11;
                                let a2288 = {
                                    let a2285 = {
                                        let a2283 = 11u64;
                                        let a2284 = v7;
                                        nat_mul(a2283, a2284)?
                                    };
                                    let a2286 = 2u64;
                                    nat_add(a2285, a2286)?
                                };
                                if nat_lt(a2287, a2288) {
                                    let v17: u64 = {
                                        let a2293 = v11;
                                        let a2294 = {
                                            let a2291 = {
                                                let a2289 = 6u64;
                                                let a2290 = v7;
                                                nat_mul(a2289, a2290)?
                                            };
                                            let a2292 = 2u64;
                                            nat_add(a2291, a2292)?
                                        };
                                        nat_sub(a2293, a2294)
                                    };
                                    let v18: u64 = {
                                        let a2295 = v17;
                                        let a2296 = 5u64;
                                        let a2297 = 0u64;
                                        nat_quot(a2295, a2296, a2297)
                                    };
                                    let a2301 = {
                                        let a2298 = v17;
                                        let a2299 = 5u64;
                                        let a2300 = 0u64;
                                        nat_rem(a2298, a2299, a2300)
                                    };
                                    let a2302 = 2u64;
                                    if nat_lt(a2301, a2302) {
                                        let a2306 = {
                                            let a2303 = v17;
                                            let a2304 = 5u64;
                                            let a2305 = 0u64;
                                            nat_rem(a2303, a2304, a2305)
                                        };
                                        let a2307 = 1u64;
                                        if nat_lt(a2306, a2307) {
                                            Ok(Adt9::C7(Adt11::C1(1u64), Adt12::C0(if v3 {
                                                1u64
                                            } else {
                                                0u64
                                            }, {
                                                let a2313 = {
                                                    let a2309 = v10;
                                                    let a2310 = match v1 {
                                                        Adt3::C0(_, _, _, _, _, h2308, _, _) => {
                                                            h2308
                                                        }
                                                    };
                                                    nat_mul(a2309, a2310)?
                                                };
                                                let a2314 = {
                                                    let a2311 = 4u64;
                                                    let a2312 = v18;
                                                    nat_mul(a2311, a2312)?
                                                };
                                                nat_add(a2313, a2314)?
                                            }), Adt12::C1(0u64), Adt13::C0(3u64), 4u64, 4u64))
                                        } else {
                                            Ok(Adt9::C7(Adt11::C1(1u64), Adt12::C1(3u64), Adt12::C1(1u64), Adt13::C0(3u64), 4u64, 4u64))
                                        }
                                    } else {
                                        let a2318 = {
                                            let a2315 = v17;
                                            let a2316 = 5u64;
                                            let a2317 = 0u64;
                                            nat_rem(a2315, a2316, a2317)
                                        };
                                        let a2319 = 3u64;
                                        if nat_lt(a2318, a2319) {
                                            Ok(Adt9::C7(Adt11::C1(2u64), Adt12::C1(3u64), Adt12::C0(if v3 {
                                                2u64
                                            } else {
                                                1u64
                                            }, {
                                                let a2324 = {
                                                    let a2320 = v10;
                                                    let a2321 = v4;
                                                    nat_mul(a2320, a2321)?
                                                };
                                                let a2325 = {
                                                    let a2322 = 4u64;
                                                    let a2323 = v18;
                                                    nat_mul(a2322, a2323)?
                                                };
                                                nat_add(a2324, a2325)?
                                            }), Adt13::C0(3u64), 4u64, 4u64))
                                        } else {
                                            let a2329 = {
                                                let a2326 = v17;
                                                let a2327 = 5u64;
                                                let a2328 = 0u64;
                                                nat_rem(a2326, a2327, a2328)
                                            };
                                            let a2330 = 4u64;
                                            if nat_lt(a2329, a2330) {
                                                Ok(Adt9::C7(Adt11::C0(1u64), Adt12::C1(3u64), Adt12::C2(0u64), Adt13::C0(3u64), 4u64, 4u64))
                                            } else {
                                                Ok(Adt9::C7(Adt11::C0(0u64), {
                                                    let a2331 = v18;
                                                    let a2332 = 0u64;
                                                    if nat_eq(a2331, a2332) {
                                                        Adt12::C2(0u64)
                                                    } else {
                                                        Adt12::C1(4u64)
                                                    }
                                                }, Adt12::C1(3u64), Adt13::C0(4u64), 4u64, 4u64))
                                            }
                                        }
                                    }
                                } else {
                                    let a2337 = v11;
                                    let a2338 = {
                                        let a2335 = {
                                            let a2333 = 11u64;
                                            let a2334 = v7;
                                            nat_mul(a2333, a2334)?
                                        };
                                        let a2336 = 2u64;
                                        nat_add(a2335, a2336)?
                                    };
                                    if nat_eq(a2337, a2338) {
                                        Ok(Adt9::C7(Adt11::C0(2u64), Adt12::C1(4u64), Adt12::C2(0u64), Adt13::C0(4u64), 4u64, 4u64))
                                    } else {
                                        Ok(Adt9::C7(Adt11::C1(0u64), {
                                            let a2339 = v10;
                                            let a2340 = 0u64;
                                            if nat_eq(a2339, a2340) {
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

pub fn f79(v0: u64, v1: Adt3, v2: Adt3, v3: bool, v4: u64, v5: u64) -> R<Adt9> {
    let a2341 = v0;
    let a2342 = v4;
    if nat_lt(a2341, a2342) {
        let v6: u64 = f24(match v1.clone() {
            Adt3::C0(h2343, _, _, _, _, _, _, _) => {
                h2343
            }
        }, match v2.clone() {
            Adt3::C0(h2344, _, _, _, _, _, _, _) => {
                h2344
            }
        })?;
        let a2345 = v0;
        let a2346 = v6;
        if nat_lt(a2345, a2346) {
            f188(v0, 4u64, match v1.clone() {
                Adt3::C0(h2347, _, _, _, _, _, _, _) => {
                    h2347
                }
            }, match v2 {
                Adt3::C0(h2348, _, _, _, _, _, _, _) => {
                    h2348
                }
            }, match v1 {
                Adt3::C0(_, _, _, _, h2349, _, _, _) => {
                    h2349
                }
            }, v3, v5)
        } else {
            let v7: u64 = {
                let a2350 = v0;
                let a2351 = v6;
                nat_sub(a2350, a2351)
            };
            let v8: u64 = f24(match v1.clone() {
                Adt3::C0(_, h2352, _, _, _, _, _, _) => {
                    h2352
                }
            }, match v2.clone() {
                Adt3::C0(_, h2353, _, _, _, _, _, _) => {
                    h2353
                }
            })?;
            let a2354 = v7;
            let a2355 = v8;
            if nat_lt(a2354, a2355) {
                f188(v7, {
                    let a2357 = 4u64;
                    let a2358 = match v2.clone() {
                        Adt3::C0(h2356, _, _, _, _, _, _, _) => {
                            h2356
                        }
                    };
                    nat_add(a2357, a2358)?
                }, match v1.clone() {
                    Adt3::C0(_, h2359, _, _, _, _, _, _) => {
                        h2359
                    }
                }, match v2 {
                    Adt3::C0(_, h2360, _, _, _, _, _, _) => {
                        h2360
                    }
                }, match v1 {
                    Adt3::C0(_, _, _, _, _, h2361, _, _) => {
                        h2361
                    }
                }, v3, v5)
            } else {
                let v9: u64 = {
                    let a2362 = v7;
                    let a2363 = v8;
                    nat_sub(a2362, a2363)
                };
                let v10: u64 = f24(match v1.clone() {
                    Adt3::C0(_, _, h2364, _, _, _, _, _) => {
                        h2364
                    }
                }, match v2.clone() {
                    Adt3::C0(_, _, h2365, _, _, _, _, _) => {
                        h2365
                    }
                })?;
                let a2366 = v9;
                let a2367 = v10;
                if nat_lt(a2366, a2367) {
                    f188(v9, {
                        let a2372 = {
                            let a2369 = 4u64;
                            let a2370 = match v2.clone() {
                                Adt3::C0(h2368, _, _, _, _, _, _, _) => {
                                    h2368
                                }
                            };
                            nat_add(a2369, a2370)?
                        };
                        let a2373 = match v2.clone() {
                            Adt3::C0(_, h2371, _, _, _, _, _, _) => {
                                h2371
                            }
                        };
                        nat_add(a2372, a2373)?
                    }, match v1.clone() {
                        Adt3::C0(_, _, h2374, _, _, _, _, _) => {
                            h2374
                        }
                    }, match v2 {
                        Adt3::C0(_, _, h2375, _, _, _, _, _) => {
                            h2375
                        }
                    }, match v1 {
                        Adt3::C0(_, _, _, _, _, _, h2376, _) => {
                            h2376
                        }
                    }, false, v5)
                } else {
                    let v11: u64 = {
                        let a2377 = v9;
                        let a2378 = v10;
                        nat_sub(a2377, a2378)
                    };
                    f188(v11, {
                        let a2386 = {
                            let a2383 = {
                                let a2380 = 4u64;
                                let a2381 = match v2.clone() {
                                    Adt3::C0(h2379, _, _, _, _, _, _, _) => {
                                        h2379
                                    }
                                };
                                nat_add(a2380, a2381)?
                            };
                            let a2384 = match v2.clone() {
                                Adt3::C0(_, h2382, _, _, _, _, _, _) => {
                                    h2382
                                }
                            };
                            nat_add(a2383, a2384)?
                        };
                        let a2387 = match v2.clone() {
                            Adt3::C0(_, _, h2385, _, _, _, _, _) => {
                                h2385
                            }
                        };
                        nat_add(a2386, a2387)?
                    }, match v1.clone() {
                        Adt3::C0(_, _, _, h2388, _, _, _, _) => {
                            h2388
                        }
                    }, match v2 {
                        Adt3::C0(_, _, _, h2389, _, _, _, _) => {
                            h2389
                        }
                    }, match v1 {
                        Adt3::C0(_, _, _, _, _, _, _, h2390) => {
                            h2390
                        }
                    }, false, v5)
                }
            }
        }
    } else {
        let v12: u64 = match v2.clone() {
            Adt3::C0(h2391, _, _, _, _, _, _, _) => {
                h2391
            }
        };
        let v13: u64 = {
            let a2394 = 2u64;
            let a2395 = {
                let a2392 = 2u64;
                let a2393 = v12;
                nat_mul(a2392, a2393)?
            };
            nat_add(a2394, a2395)?
        };
        let v14: u64 = {
            let a2396 = v0;
            let a2397 = v4;
            nat_sub(a2396, a2397)
        };
        let v15: u64 = {
            let a2398 = v14;
            let a2399 = v13;
            let a2400 = 0u64;
            nat_quot(a2398, a2399, a2400)
        };
        let v16: u64 = {
            let a2401 = v14;
            let a2402 = v13;
            let a2403 = 0u64;
            nat_rem(a2401, a2402, a2403)
        };
        let v17: u64 = {
            let a2405 = v15;
            let a2406 = match v2.clone() {
                Adt3::C0(_, h2404, _, _, _, _, _, _) => {
                    h2404
                }
            };
            let a2407 = 0u64;
            nat_quot(a2405, a2406, a2407)
        };
        let v18: u64 = {
            let a2415 = {
                let a2409 = 4u64;
                let a2410 = match v2.clone() {
                    Adt3::C0(h2408, _, _, _, _, _, _, _) => {
                        h2408
                    }
                };
                nat_add(a2409, a2410)?
            };
            let a2416 = {
                let a2412 = v15;
                let a2413 = match v2.clone() {
                    Adt3::C0(_, h2411, _, _, _, _, _, _) => {
                        h2411
                    }
                };
                let a2414 = 0u64;
                nat_rem(a2412, a2413, a2414)
            };
            nat_add(a2415, a2416)?
        };
        let v19: u64 = {
            let a2427 = {
                let a2421 = {
                    let a2418 = 4u64;
                    let a2419 = match v2.clone() {
                        Adt3::C0(h2417, _, _, _, _, _, _, _) => {
                            h2417
                        }
                    };
                    nat_add(a2418, a2419)?
                };
                let a2422 = match v2.clone() {
                    Adt3::C0(_, h2420, _, _, _, _, _, _) => {
                        h2420
                    }
                };
                nat_add(a2421, a2422)?
            };
            let a2428 = {
                let a2424 = v17;
                let a2425 = match v2.clone() {
                    Adt3::C0(_, _, h2423, _, _, _, _, _) => {
                        h2423
                    }
                };
                let a2426 = 0u64;
                nat_rem(a2424, a2425, a2426)
            };
            nat_add(a2427, a2428)?
        };
        let v20: u64 = {
            let a2442 = {
                let a2436 = {
                    let a2433 = {
                        let a2430 = 4u64;
                        let a2431 = match v2.clone() {
                            Adt3::C0(h2429, _, _, _, _, _, _, _) => {
                                h2429
                            }
                        };
                        nat_add(a2430, a2431)?
                    };
                    let a2434 = match v2.clone() {
                        Adt3::C0(_, h2432, _, _, _, _, _, _) => {
                            h2432
                        }
                    };
                    nat_add(a2433, a2434)?
                };
                let a2437 = match v2.clone() {
                    Adt3::C0(_, _, h2435, _, _, _, _, _) => {
                        h2435
                    }
                };
                nat_add(a2436, a2437)?
            };
            let a2443 = {
                let a2439 = v17;
                let a2440 = match v2.clone() {
                    Adt3::C0(_, _, h2438, _, _, _, _, _) => {
                        h2438
                    }
                };
                let a2441 = 0u64;
                nat_quot(a2439, a2440, a2441)
            };
            nat_add(a2442, a2443)?
        };
        let v21: u64 = {
            let a2446 = {
                let a2444 = v16;
                let a2445 = 2u64;
                nat_sub(a2444, a2445)
            };
            let a2447 = 2u64;
            let a2448 = 0u64;
            nat_quot(a2446, a2447, a2448)
        };
        let a2449 = v16;
        let a2450 = 0u64;
        if nat_eq(a2449, a2450) {
            Ok(Adt9::C7(Adt11::C5(2u64), Adt12::C1(v18), Adt12::C1(v19), Adt13::C0(0u64), 4u64, 4u64))
        } else {
            let a2451 = v16;
            let a2452 = 1u64;
            if nat_eq(a2451, a2452) {
                Ok(Adt9::C7(Adt11::C5(2u64), Adt12::C1(0u64), Adt12::C1(v20), Adt13::C0(1u64), 4u64, 4u64))
            } else {
                let a2456 = {
                    let a2453 = v16;
                    let a2454 = 2u64;
                    let a2455 = 0u64;
                    nat_rem(a2453, a2454, a2455)
                };
                let a2457 = 0u64;
                if nat_eq(a2456, a2457) {
                    Ok(Adt9::C7(Adt11::C5(2u64), Adt12::C1({
                        let a2458 = 4u64;
                        let a2459 = v21;
                        nat_add(a2458, a2459)?
                    }), Adt12::C1(1u64), Adt13::C0(2u64), 4u64, 4u64))
                } else {
                    Ok(Adt9::C1(false, v5, f105({
                        let a2462 = {
                            let a2460 = v15;
                            let a2461 = v12;
                            nat_mul(a2460, a2461)?
                        };
                        let a2463 = v21;
                        nat_add(a2462, a2463)?
                    }, v2)?))
                }
            }
        }
    }
}

pub fn f80(v0: u64, v1: Adt3, v2: bool, v3: u64) -> R<Adt9> {
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
        let v8: u64 = if v2 {
            let a2479 = 1u64;
            let a2480 = {
                let a2477 = 9u64;
                let a2478 = v4;
                nat_mul(a2477, a2478)?
            };
            nat_add(a2479, a2480)?
        } else {
            let a2481 = 7u64;
            let a2482 = v4;
            nat_mul(a2481, a2482)?
        };
        let v9: u64 = {
            let a2483 = v7;
            let a2484 = v8;
            let a2485 = 0u64;
            nat_quot(a2483, a2484, a2485)
        };
        let v10: u64 = {
            let a2486 = v7;
            let a2487 = v8;
            let a2488 = 0u64;
            nat_rem(a2486, a2487, a2488)
        };
        let v11: u64 = {
            let a2489 = v9;
            let a2490 = v5;
            let a2491 = 0u64;
            nat_quot(a2489, a2490, a2491)
        };
        let v12: u64 = {
            let a2492 = v9;
            let a2493 = v5;
            let a2494 = 0u64;
            nat_rem(a2492, a2493, a2494)
        };
        let v13: u64 = {
            let a2496 = match v1 {
                Adt3::C0(_, _, h2495, _, _, _, _, _) => {
                    h2495
                }
            };
            let a2497 = v3;
            let a2498 = 0u64;
            nat_quot(a2496, a2497, a2498)
        };
        let v14: u64 = {
            let a2499 = 4u64;
            let a2500 = v9;
            nat_mul(a2499, a2500)?
        };
        if v2 {
            let a2501 = v10;
            let a2502 = 0u64;
            if nat_eq(a2501, a2502) {
                Ok(Adt9::C7(Adt11::C5(3u64), Adt12::C2(0u64), Adt12::C2(0u64), Adt13::C0(0u64), 4u64, 4u64))
            } else {
                let a2505 = v10;
                let a2506 = {
                    let a2503 = 2u64;
                    let a2504 = v4;
                    nat_mul(a2503, a2504)?
                };
                if nat_le(a2505, a2506) {
                    let v15: u64 = {
                        let a2509 = {
                            let a2507 = v10;
                            let a2508 = 1u64;
                            nat_sub(a2507, a2508)
                        };
                        let a2510 = 2u64;
                        let a2511 = 0u64;
                        nat_quot(a2509, a2510, a2511)
                    };
                    let v16: u64 = {
                        let a2519 = 4u64;
                        let a2520 = {
                            let a2517 = {
                                let a2515 = v9;
                                let a2516 = {
                                    let a2512 = v12;
                                    let a2513 = v4;
                                    let a2514 = 0u64;
                                    nat_rem(a2512, a2513, a2514)
                                };
                                nat_sub(a2515, a2516)
                            };
                            let a2518 = v15;
                            nat_add(a2517, a2518)?
                        };
                        nat_mul(a2519, a2520)?
                    };
                    let v17: u64 = {
                        let a2532 = 4u64;
                        let a2533 = {
                            let a2530 = {
                                let a2528 = {
                                    let a2526 = {
                                        let a2524 = {
                                            let a2521 = v11;
                                            let a2522 = v13;
                                            let a2523 = 0u64;
                                            nat_quot(a2521, a2522, a2523)
                                        };
                                        let a2525 = v5;
                                        nat_mul(a2524, a2525)?
                                    };
                                    let a2527 = v12;
                                    nat_add(a2526, a2527)?
                                };
                                let a2529 = v4;
                                nat_mul(a2528, a2529)?
                            };
                            let a2531 = v15;
                            nat_add(a2530, a2531)?
                        };
                        nat_mul(a2532, a2533)?
                    };
                    let a2537 = {
                        let a2534 = v10;
                        let a2535 = 2u64;
                        let a2536 = 0u64;
                        nat_rem(a2534, a2535, a2536)
                    };
                    let a2538 = 1u64;
                    if nat_eq(a2537, a2538) {
                        Ok(Adt9::C7(Adt11::C1(2u64), Adt12::C0(4u64, v16), {
                            let a2542 = {
                                let a2539 = v11;
                                let a2540 = v13;
                                let a2541 = 0u64;
                                nat_rem(a2539, a2540, a2541)
                            };
                            let a2543 = 0u64;
                            if nat_eq(a2542, a2543) {
                                Adt12::C0(6u64, v17)
                            } else {
                                Adt12::C3({
                                    let a2546 = {
                                        let a2544 = 4u64;
                                        let a2545 = v6;
                                        nat_mul(a2544, a2545)?
                                    };
                                    let a2547 = v17;
                                    nat_add(a2546, a2547)?
                                })
                            }
                        }, Adt13::C0(1u64), 4u64, 4u64))
                    } else {
                        Ok(Adt9::C7(Adt11::C1(0u64), Adt12::C1(0u64), Adt12::C1(1u64), Adt13::C0(0u64), 4u64, 4u64))
                    }
                } else {
                    let v18: u64 = {
                        let a2554 = {
                            let a2552 = v10;
                            let a2553 = {
                                let a2550 = 1u64;
                                let a2551 = {
                                    let a2548 = 2u64;
                                    let a2549 = v4;
                                    nat_mul(a2548, a2549)?
                                };
                                nat_add(a2550, a2551)?
                            };
                            nat_sub(a2552, a2553)
                        };
                        let a2555 = 7u64;
                        let a2556 = 0u64;
                        nat_quot(a2554, a2555, a2556)
                    };
                    let v19: u64 = {
                        let a2563 = {
                            let a2561 = v10;
                            let a2562 = {
                                let a2559 = 1u64;
                                let a2560 = {
                                    let a2557 = 2u64;
                                    let a2558 = v4;
                                    nat_mul(a2557, a2558)?
                                };
                                nat_add(a2559, a2560)?
                            };
                            nat_sub(a2561, a2562)
                        };
                        let a2564 = 7u64;
                        let a2565 = 0u64;
                        nat_rem(a2563, a2564, a2565)
                    };
                    let v20: u64 = {
                        let a2573 = 4u64;
                        let a2574 = {
                            let a2571 = {
                                let a2569 = v9;
                                let a2570 = {
                                    let a2566 = v12;
                                    let a2567 = v4;
                                    let a2568 = 0u64;
                                    nat_rem(a2566, a2567, a2568)
                                };
                                nat_sub(a2569, a2570)
                            };
                            let a2572 = v18;
                            nat_add(a2571, a2572)?
                        };
                        nat_mul(a2573, a2574)?
                    };
                    let v21: u64 = {
                        let a2586 = 4u64;
                        let a2587 = {
                            let a2584 = {
                                let a2582 = {
                                    let a2580 = {
                                        let a2578 = {
                                            let a2575 = v11;
                                            let a2576 = v13;
                                            let a2577 = 0u64;
                                            nat_quot(a2575, a2576, a2577)
                                        };
                                        let a2579 = v5;
                                        nat_mul(a2578, a2579)?
                                    };
                                    let a2581 = v12;
                                    nat_add(a2580, a2581)?
                                };
                                let a2583 = v4;
                                nat_mul(a2582, a2583)?
                            };
                            let a2585 = v18;
                            nat_add(a2584, a2585)?
                        };
                        nat_mul(a2586, a2587)?
                    };
                    let v22: u64 = {
                        let a2590 = {
                            let a2588 = 4u64;
                            let a2589 = v6;
                            nat_mul(a2588, a2589)?
                        };
                        let a2591 = v21;
                        nat_add(a2590, a2591)?
                    };
                    let a2592 = v19;
                    let a2593 = 0u64;
                    if nat_eq(a2592, a2593) {
                        Ok(Adt9::C7(Adt11::C1(2u64), Adt12::C0(3u64, v14), Adt12::C0(2u64, v20), Adt13::C0(1u64), 4u64, 4u64))
                    } else {
                        let a2594 = v19;
                        let a2595 = 1u64;
                        if nat_eq(a2594, a2595) {
                            Ok(Adt9::C7(Adt11::C1(2u64), {
                                let a2599 = {
                                    let a2596 = v11;
                                    let a2597 = v13;
                                    let a2598 = 0u64;
                                    nat_rem(a2596, a2597, a2598)
                                };
                                let a2600 = 0u64;
                                if nat_eq(a2599, a2600) {
                                    Adt12::C0(6u64, v21)
                                } else {
                                    Adt12::C3(v22)
                                }
                            }, Adt12::C0(1u64, v20), Adt13::C0(2u64), 4u64, 4u64))
                        } else {
                            let a2601 = v19;
                            let a2602 = 2u64;
                            if nat_eq(a2601, a2602) {
                                Ok(Adt9::C7(Adt11::C1(0u64), Adt12::C1(2u64), Adt12::C1(1u64), Adt13::C0(2u64), 4u64, 4u64))
                            } else {
                                let a2603 = v19;
                                let a2604 = 3u64;
                                if nat_eq(a2603, a2604) {
                                    Ok(Adt9::C7(Adt11::C1(2u64), Adt12::C1(0u64), Adt12::C0(5u64, v20), Adt13::C0(1u64), 4u64, 4u64))
                                } else {
                                    let a2605 = v19;
                                    let a2606 = 4u64;
                                    if nat_eq(a2605, a2606) {
                                        Ok(Adt9::C7(Adt11::C1(0u64), Adt12::C1(2u64), Adt12::C1(1u64), Adt13::C1(v22), 4u64, 4u64))
                                    } else {
                                        let a2607 = v19;
                                        let a2608 = 5u64;
                                        if nat_eq(a2607, a2608) {
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
                let a2609 = v10;
                let a2610 = 7u64;
                let a2611 = 0u64;
                nat_quot(a2609, a2610, a2611)
            };
            let v24: u64 = {
                let a2612 = v10;
                let a2613 = 7u64;
                let a2614 = 0u64;
                nat_rem(a2612, a2613, a2614)
            };
            let v25: u64 = {
                let a2622 = 4u64;
                let a2623 = {
                    let a2620 = {
                        let a2618 = v9;
                        let a2619 = {
                            let a2615 = v12;
                            let a2616 = v4;
                            let a2617 = 0u64;
                            nat_rem(a2615, a2616, a2617)
                        };
                        nat_sub(a2618, a2619)
                    };
                    let a2621 = v23;
                    nat_add(a2620, a2621)?
                };
                nat_mul(a2622, a2623)?
            };
            let v26: u64 = {
                let a2635 = 4u64;
                let a2636 = {
                    let a2633 = {
                        let a2631 = {
                            let a2629 = {
                                let a2627 = {
                                    let a2624 = v11;
                                    let a2625 = v13;
                                    let a2626 = 0u64;
                                    nat_quot(a2624, a2625, a2626)
                                };
                                let a2628 = v5;
                                nat_mul(a2627, a2628)?
                            };
                            let a2630 = v12;
                            nat_add(a2629, a2630)?
                        };
                        let a2632 = v4;
                        nat_mul(a2631, a2632)?
                    };
                    let a2634 = v23;
                    nat_add(a2633, a2634)?
                };
                nat_mul(a2635, a2636)?
            };
            let v27: u64 = {
                let a2639 = {
                    let a2637 = 4u64;
                    let a2638 = v6;
                    nat_mul(a2637, a2638)?
                };
                let a2640 = v26;
                nat_add(a2639, a2640)?
            };
            let a2641 = v24;
            let a2642 = 0u64;
            if nat_eq(a2641, a2642) {
                Ok(Adt9::C7(Adt11::C1(2u64), Adt12::C0(1u64, v25), Adt12::C0(0u64, v14), Adt13::C0(0u64), 4u64, 4u64))
            } else {
                let a2643 = v24;
                let a2644 = 1u64;
                if nat_eq(a2643, a2644) {
                    Ok(Adt9::C7(Adt11::C1(2u64), Adt12::C1(0u64), Adt12::C0(3u64, {
                        let a2645 = 4u64;
                        let a2646 = v12;
                        nat_mul(a2645, a2646)?
                    }), Adt13::C0(1u64), 4u64, 4u64))
                } else {
                    let a2647 = v24;
                    let a2648 = 2u64;
                    if nat_eq(a2647, a2648) {
                        Ok(Adt9::C7(Adt11::C1(0u64), Adt12::C1(1u64), {
                            let a2652 = {
                                let a2649 = v11;
                                let a2650 = v13;
                                let a2651 = 0u64;
                                nat_rem(a2649, a2650, a2651)
                            };
                            let a2653 = 0u64;
                            if nat_eq(a2652, a2653) {
                                Adt12::C0(5u64, v26)
                            } else {
                                Adt12::C3(v27)
                            }
                        }, Adt13::C0(1u64), 4u64, 4u64))
                    } else {
                        let a2654 = v24;
                        let a2655 = 3u64;
                        if nat_eq(a2654, a2655) {
                            Ok(Adt9::C7(Adt11::C1(2u64), Adt12::C1(1u64), Adt12::C0(2u64, v14), Adt13::C0(1u64), 4u64, 4u64))
                        } else {
                            let a2656 = v24;
                            let a2657 = 4u64;
                            if nat_eq(a2656, a2657) {
                                Ok(Adt9::C7(Adt11::C1(0u64), Adt12::C3(v25), Adt12::C1(1u64), Adt13::C1(v25), 4u64, 4u64))
                            } else {
                                let a2658 = v24;
                                let a2659 = 5u64;
                                if nat_eq(a2658, a2659) {
                                    Ok(Adt9::C7(Adt11::C1(2u64), {
                                        let a2663 = {
                                            let a2660 = v11;
                                            let a2661 = v13;
                                            let a2662 = 0u64;
                                            nat_rem(a2660, a2661, a2662)
                                        };
                                        let a2664 = 0u64;
                                        if nat_eq(a2663, a2664) {
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

pub fn f81(v0: u64, v1: Adt3, v2: u64, v3: u64) -> R<Adt9> {
    let v4: u64 = match v1.clone() {
        Adt3::C0(h2665, _, _, _, _, _, _, _) => {
            h2665
        }
    };
    let v5: u64 = {
        let a2667 = v4;
        let a2668 = match v1.clone() {
            Adt3::C0(_, h2666, _, _, _, _, _, _) => {
                h2666
            }
        };
        nat_mul(a2667, a2668)?
    };
    let v6: u64 = {
        let a2670 = v5;
        let a2671 = match v1.clone() {
            Adt3::C0(_, _, h2669, _, _, _, _, _) => {
                h2669
            }
        };
        nat_mul(a2670, a2671)?
    };
    let a2672 = v0;
    let a2673 = v6;
    if nat_lt(a2672, a2673) {
        Ok(Adt9::C7(Adt11::C5(3u64), Adt12::C2(0u64), Adt12::C2(0u64), Adt13::C1({
            let a2674 = 4u64;
            let a2675 = v0;
            nat_mul(a2674, a2675)?
        }), 4u64, 4u64))
    } else {
        let v7: u64 = {
            let a2676 = v0;
            let a2677 = v6;
            nat_sub(a2676, a2677)
        };
        let v8: u64 = {
            let a2680 = 1u64;
            let a2681 = {
                let a2678 = 5u64;
                let a2679 = v4;
                nat_mul(a2678, a2679)?
            };
            nat_add(a2680, a2681)?
        };
        let v9: u64 = {
            let a2682 = v7;
            let a2683 = v8;
            let a2684 = 0u64;
            nat_quot(a2682, a2683, a2684)
        };
        let v10: u64 = {
            let a2685 = v7;
            let a2686 = v8;
            let a2687 = 0u64;
            nat_rem(a2685, a2686, a2687)
        };
        let a2688 = v10;
        let a2689 = 0u64;
        if nat_eq(a2688, a2689) {
            Ok(Adt9::C7(Adt11::C1(2u64), Adt12::C0(2u64, {
                let a2690 = 4u64;
                let a2691 = v9;
                nat_mul(a2690, a2691)?
            }), Adt12::C2(v2), Adt13::C0(0u64), 4u64, 4u64))
        } else {
            let v11: u64 = {
                let a2692 = v10;
                let a2693 = 1u64;
                nat_sub(a2692, a2693)
            };
            let v12: u64 = {
                let a2694 = v11;
                let a2695 = 5u64;
                let a2696 = 0u64;
                nat_quot(a2694, a2695, a2696)
            };
            let v13: u64 = {
                let a2697 = v11;
                let a2698 = 5u64;
                let a2699 = 0u64;
                nat_rem(a2697, a2698, a2699)
            };
            let v14: u64 = {
                let a2700 = v9;
                let a2701 = v5;
                let a2702 = 0u64;
                nat_quot(a2700, a2701, a2702)
            };
            let v15: u64 = {
                let a2703 = v9;
                let a2704 = v5;
                let a2705 = 0u64;
                nat_rem(a2703, a2704, a2705)
            };
            let v16: u64 = {
                let a2707 = match v1 {
                    Adt3::C0(_, _, h2706, _, _, _, _, _) => {
                        h2706
                    }
                };
                let a2708 = v3;
                let a2709 = 0u64;
                nat_quot(a2707, a2708, a2709)
            };
            let v17: u64 = {
                let a2717 = 4u64;
                let a2718 = {
                    let a2715 = {
                        let a2713 = v9;
                        let a2714 = {
                            let a2710 = v15;
                            let a2711 = v4;
                            let a2712 = 0u64;
                            nat_rem(a2710, a2711, a2712)
                        };
                        nat_sub(a2713, a2714)
                    };
                    let a2716 = v12;
                    nat_add(a2715, a2716)?
                };
                nat_mul(a2717, a2718)?
            };
            let v18: u64 = {
                let a2730 = 4u64;
                let a2731 = {
                    let a2728 = {
                        let a2726 = {
                            let a2724 = {
                                let a2722 = {
                                    let a2719 = v14;
                                    let a2720 = v16;
                                    let a2721 = 0u64;
                                    nat_quot(a2719, a2720, a2721)
                                };
                                let a2723 = v5;
                                nat_mul(a2722, a2723)?
                            };
                            let a2725 = v15;
                            nat_add(a2724, a2725)?
                        };
                        let a2727 = v4;
                        nat_mul(a2726, a2727)?
                    };
                    let a2729 = v12;
                    nat_add(a2728, a2729)?
                };
                nat_mul(a2730, a2731)?
            };
            let v19: u64 = {
                let a2734 = {
                    let a2732 = 4u64;
                    let a2733 = v6;
                    nat_mul(a2732, a2733)?
                };
                let a2735 = v18;
                nat_add(a2734, a2735)?
            };
            let a2736 = v13;
            let a2737 = 0u64;
            if nat_eq(a2736, a2737) {
                Ok(Adt9::C7(Adt11::C1(2u64), Adt12::C0(1u64, v17), Adt12::C0(0u64, {
                    let a2738 = 4u64;
                    let a2739 = v9;
                    nat_mul(a2738, a2739)?
                }), Adt13::C0(1u64), 4u64, 4u64))
            } else {
                let a2740 = v13;
                let a2741 = 1u64;
                if nat_eq(a2740, a2741) {
                    Ok(Adt9::C7(Adt11::C1(2u64), {
                        let a2745 = {
                            let a2742 = v14;
                            let a2743 = v16;
                            let a2744 = 0u64;
                            nat_rem(a2742, a2743, a2744)
                        };
                        let a2746 = 0u64;
                        if nat_eq(a2745, a2746) {
                            Adt12::C0(4u64, v18)
                        } else {
                            Adt12::C3(v19)
                        }
                    }, Adt12::C0(3u64, {
                        let a2747 = 4u64;
                        let a2748 = v9;
                        nat_mul(a2747, a2748)?
                    }), Adt13::C0(2u64), 4u64, 4u64))
                } else {
                    let a2749 = v13;
                    let a2750 = 2u64;
                    if nat_eq(a2749, a2750) {
                        Ok(Adt9::C7(Adt11::C1(0u64), Adt12::C1(2u64), Adt12::C1(1u64), Adt13::C1(v19), 4u64, 4u64))
                    } else {
                        let a2751 = v13;
                        let a2752 = 3u64;
                        if nat_eq(a2751, a2752) {
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

pub fn f82(v0: u64, v1: Adt3, v2: u64) -> R<Adt9> {
    let v3: u64 = match v1.clone() {
        Adt3::C0(h2753, _, _, _, _, _, _, _) => {
            h2753
        }
    };
    let v4: u64 = match v1 {
        Adt3::C0(_, h2754, _, _, _, _, _, _) => {
            h2754
        }
    };
    let v5: u64 = {
        let a2755 = v3;
        let a2756 = 2u64;
        let a2757 = 0u64;
        nat_quot(a2755, a2756, a2757)
    };
    let v6: u64 = {
        let a2760 = 5u64;
        let a2761 = {
            let a2758 = 3u64;
            let a2759 = v4;
            nat_mul(a2758, a2759)?
        };
        nat_add(a2760, a2761)?
    };
    let v7: u64 = {
        let a2762 = v5;
        let a2763 = 0u64;
        if nat_eq(a2762, a2763) {
            0u64
        } else {
            let a2766 = 7u64;
            let a2767 = {
                let a2764 = v5;
                let a2765 = v6;
                nat_mul(a2764, a2765)?
            };
            nat_add(a2766, a2767)?
        }
    };
    let a2768 = v7;
    let a2769 = v0;
    if nat_le(a2768, a2769) {
        Ok(Adt9::C7(Adt11::C5(3u64), Adt12::C2(0u64), Adt12::C2(0u64), Adt13::C1({
            let a2778 = 4u64;
            let a2779 = {
                let a2776 = {
                    let a2772 = {
                        let a2770 = v0;
                        let a2771 = v7;
                        nat_sub(a2770, a2771)
                    };
                    let a2773 = v3;
                    nat_mul(a2772, a2773)?
                };
                let a2777 = {
                    let a2774 = 2u64;
                    let a2775 = v5;
                    nat_mul(a2774, a2775)?
                };
                nat_add(a2776, a2777)?
            };
            nat_mul(a2778, a2779)?
        }), 4u64, 4u64))
    } else {
        let a2780 = v0;
        let a2781 = 7u64;
        if nat_lt(a2780, a2781) {
            let a2782 = v0;
            let a2783 = 0u64;
            if nat_eq(a2782, a2783) {
                Ok(Adt9::C7(Adt11::C0(5u64), Adt12::C2({
                    let a2784 = f160(v2);
                    let a2785 = f159(v2);
                    nat_add(a2784, a2785)?
                }), Adt12::C2(0u64), Adt13::C0(0u64), 4u64, 4u64))
            } else {
                let a2786 = v0;
                let a2787 = 1u64;
                if nat_eq(a2786, a2787) {
                    Ok(Adt9::C7(Adt11::C0(2u64), Adt12::C1(0u64), Adt12::C2(0u64), Adt13::C0(0u64), 4u64, 4u64))
                } else {
                    let a2788 = v0;
                    let a2789 = 2u64;
                    if nat_eq(a2788, a2789) {
                        let a2790 = v2;
                        let a2791 = 2147483648u64;
                        if nat_lt(a2790, a2791) {
                            Ok(Adt9::C7(Adt11::C5(3u64), Adt12::C1(0u64), Adt12::C2(0u64), Adt13::C0(0u64), 4u64, 4u64))
                        } else {
                            Ok(Adt9::C7(Adt11::C1(6u64), Adt12::C1(0u64), Adt12::C2(0u64), Adt13::C0(0u64), 4u64, 4u64))
                        }
                    } else {
                        let a2792 = v0;
                        let a2793 = 3u64;
                        if nat_eq(a2792, a2793) {
                            Ok(Adt9::C7(Adt11::C7(13u64, 0u64, 0u64), Adt12::C1(0u64), Adt12::C2(0u64), Adt13::C0(0u64), 4u64, 4u64))
                        } else {
                            let a2794 = v0;
                            let a2795 = 4u64;
                            if nat_eq(a2794, a2795) {
                                Ok(Adt9::C7(Adt11::C1(6u64), Adt12::C1(0u64), Adt12::C2(0u64), Adt13::C0(0u64), 4u64, 4u64))
                            } else {
                                let a2796 = v0;
                                let a2797 = 5u64;
                                if nat_eq(a2796, a2797) {
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
                let a2798 = v0;
                let a2799 = 7u64;
                nat_sub(a2798, a2799)
            };
            let v9: u64 = {
                let a2800 = v8;
                let a2801 = v6;
                let a2802 = 0u64;
                nat_quot(a2800, a2801, a2802)
            };
            let v10: u64 = {
                let a2803 = v8;
                let a2804 = v6;
                let a2805 = 0u64;
                nat_rem(a2803, a2804, a2805)
            };
            let a2806 = v10;
            let a2807 = 5u64;
            if nat_lt(a2806, a2807) {
                let a2808 = v10;
                let a2809 = 0u64;
                if nat_eq(a2808, a2809) {
                    Ok(Adt9::C7(Adt11::C0(5u64), Adt12::C2(v9), Adt12::C2(0u64), Adt13::C0(3u64), 4u64, 4u64))
                } else {
                    let a2810 = v10;
                    let a2811 = 1u64;
                    if nat_eq(a2810, a2811) {
                        Ok(Adt9::C7(Adt11::C0(2u64), Adt12::C1(3u64), Adt12::C2(0u64), Adt13::C0(3u64), 4u64, 4u64))
                    } else {
                        let a2812 = v10;
                        let a2813 = 2u64;
                        if nat_eq(a2812, a2813) {
                            Ok(Adt9::C7(Adt11::C1(2u64), Adt12::C1(0u64), Adt12::C1(3u64), Adt13::C0(3u64), 4u64, 4u64))
                        } else {
                            let a2814 = v10;
                            let a2815 = 3u64;
                            if nat_eq(a2814, a2815) {
                                Ok(Adt9::C7(Adt11::C1(3u64), Adt12::C1(3u64), Adt12::C1(1u64), Adt13::C0(3u64), 4u64, 4u64))
                            } else {
                                Ok(Adt9::C7(Adt11::C7(16u64, 0u64, 0u64), Adt12::C1(3u64), Adt12::C2(0u64), Adt13::C0(3u64), 4u64, 4u64))
                            }
                        }
                    }
                }
            } else {
                let v11: u64 = {
                    let a2816 = v10;
                    let a2817 = 5u64;
                    nat_sub(a2816, a2817)
                };
                let v12: u64 = {
                    let a2818 = v11;
                    let a2819 = 3u64;
                    let a2820 = 0u64;
                    nat_quot(a2818, a2819, a2820)
                };
                let v13: u64 = {
                    let a2821 = v11;
                    let a2822 = 3u64;
                    let a2823 = 0u64;
                    nat_rem(a2821, a2822, a2823)
                };
                let a2824 = v13;
                let a2825 = 0u64;
                if nat_eq(a2824, a2825) {
                    Ok(Adt9::C7(Adt11::C1(2u64), Adt12::C0(0u64, {
                        let a2826 = 4u64;
                        let a2827 = v12;
                        nat_mul(a2826, a2827)?
                    }), Adt12::C1(3u64), Adt13::C0(4u64), 4u64, 4u64))
                } else {
                    let a2828 = v13;
                    let a2829 = 1u64;
                    if nat_eq(a2828, a2829) {
                        Ok(Adt9::C7(Adt11::C7(15u64, 0u64, 0u64), Adt12::C1(4u64), Adt12::C2(0u64), Adt13::C1({
                            let a2834 = 4u64;
                            let a2835 = {
                                let a2832 = {
                                    let a2830 = v12;
                                    let a2831 = v3;
                                    nat_mul(a2830, a2831)?
                                };
                                let a2833 = v9;
                                nat_add(a2832, a2833)?
                            };
                            nat_mul(a2834, a2835)?
                        }), 4u64, 4u64))
                    } else {
                        Ok(Adt9::C7(Adt11::C7(14u64, 0u64, 0u64), Adt12::C1(4u64), Adt12::C2(0u64), Adt13::C1({
                            let a2842 = 4u64;
                            let a2843 = {
                                let a2840 = {
                                    let a2836 = v12;
                                    let a2837 = v3;
                                    nat_mul(a2836, a2837)?
                                };
                                let a2841 = {
                                    let a2838 = v5;
                                    let a2839 = v9;
                                    nat_add(a2838, a2839)?
                                };
                                nat_add(a2840, a2841)?
                            };
                            nat_mul(a2842, a2843)?
                        }), 4u64, 4u64))
                    }
                }
            }
        }
    }
}

pub fn f83(v0: u64, v1: Adt3, v2: Adt3, v3: Adt7, v4: u64, v5: u64) -> R<Adt9> {
    let a2844 = v0;
    let a2845 = v4;
    if nat_lt(a2844, a2845) {
        Ok(Adt9::C7(Adt11::C5(3u64), Adt12::C2(0u64), Adt12::C2(0u64), Adt13::C1(f105(v0, v2)?), 4u64, 4u64))
    } else {
        let v6: u64 = {
            let a2852 = {
                let m2848 = match v3.clone() {
                    Adt7::C0(h2847, _, _, _, _) => {
                        h2847
                    }
                };
                match m2848 {
                    Adt6::C0(h2846, _, _, _) => {
                        h2846
                    }
                }
            };
            let a2853 = {
                let m2851 = match v3.clone() {
                    Adt7::C0(_, h2850, _, _, _) => {
                        h2850
                    }
                };
                match m2851 {
                    Adt6::C0(h2849, _, _, _) => {
                        h2849
                    }
                }
            };
            nat_mul(a2852, a2853)?
        };
        let v7: u64 = f189(match v3.clone() {
            Adt7::C0(_, _, _, h2854, _) => {
                h2854
            }
        }, v6)?;
        let v8: u64 = {
            let a2855 = v0;
            let a2856 = v4;
            nat_sub(a2855, a2856)
        };
        let v9: u64 = {
            let a2857 = v8;
            let a2858 = v7;
            let a2859 = 0u64;
            nat_quot(a2857, a2858, a2859)
        };
        let v10: u64 = {
            let a2860 = v8;
            let a2861 = v7;
            let a2862 = 0u64;
            nat_rem(a2860, a2861, a2862)
        };
        if match v3.clone() {
            Adt7::C0(_, _, _, h2863, _) => {
                h2863
            }
        } {
            let a2864 = v10;
            let a2865 = 0u64;
            if nat_eq(a2864, a2865) {
                Ok(Adt9::C7(Adt11::C5(3u64), Adt12::C2(4286578687u64), Adt12::C2(0u64), Adt13::C0(0u64), 4u64, 4u64))
            } else {
                let a2866 = v10;
                let a2867 = 1u64;
                if nat_eq(a2866, a2867) {
                    Ok(Adt9::C7(Adt11::C5(3u64), Adt12::C2(0u64), Adt12::C2(0u64), Adt13::C0(1u64), 4u64, 4u64))
                } else {
                    let a2870 = {
                        let a2868 = v10;
                        let a2869 = 2u64;
                        nat_add(a2868, a2869)?
                    };
                    let a2871 = v7;
                    if nat_eq(a2870, a2871) {
                        Ok(Adt9::C7(Adt11::C5(3u64), Adt12::C1(1u64), Adt12::C2(0u64), Adt13::C0(2u64), 4u64, 4u64))
                    } else {
                        let a2874 = {
                            let a2872 = v10;
                            let a2873 = 1u64;
                            nat_add(a2872, a2873)?
                        };
                        let a2875 = v7;
                        if nat_eq(a2874, a2875) {
                            let a2876 = v5;
                            let a2877 = 0u64;
                            if nat_eq(a2876, a2877) {
                                Ok(Adt9::C7(Adt11::C5(3u64), Adt12::C1(0u64), Adt12::C2(0u64), Adt13::C0(0u64), 4u64, 4u64))
                            } else {
                                Ok(Adt9::C7(Adt11::C1(0u64), Adt12::C3({
                                    let a2878 = v5;
                                    let a2879 = 1u64;
                                    nat_sub(a2878, a2879)
                                }), Adt12::C0(0u64, f105(v9, v1)?), Adt13::C1({
                                    let a2880 = v5;
                                    let a2881 = 1u64;
                                    nat_sub(a2880, a2881)
                                }), 4u64, 4u64))
                            }
                        } else {
                            let v11: u64 = {
                                let a2882 = v10;
                                let a2883 = 2u64;
                                nat_sub(a2882, a2883)
                            };
                            let v12: u64 = {
                                let a2884 = v11;
                                let a2885 = 3u64;
                                let a2886 = 0u64;
                                nat_rem(a2884, a2885, a2886)
                            };
                            let m2890 = f190(v3, v2, v1, v9, {
                                let a2887 = v11;
                                let a2888 = 3u64;
                                let a2889 = 0u64;
                                nat_quot(a2887, a2888, a2889)
                            })?;
                            match m2890 {
                                None => {
                                    Ok(Adt9::C7(Adt11::C5(3u64), Adt12::C1(0u64), Adt12::C2(0u64), Adt13::C0(0u64), 4u64, 4u64))
                                }
                                Some(v13) => {
                                    let a2891 = v12;
                                    let a2892 = 0u64;
                                    if nat_eq(a2891, a2892) {
                                        Ok(Adt9::C7(Adt11::C5(0u64), Adt12::C0(1u64, v13), Adt12::C1(0u64), Adt13::C0(2u64), 4u64, 4u64))
                                    } else {
                                        let a2893 = 2u64;
                                        let a2894 = v5;
                                        if nat_le(a2893, a2894) {
                                            let a2895 = v12;
                                            let a2896 = 1u64;
                                            if nat_eq(a2895, a2896) {
                                                Ok(Adt9::C7(Adt11::C5(3u64), Adt12::C0(1u64, v13), Adt12::C2(0u64), Adt13::C0(0u64), 4u64, 4u64))
                                            } else {
                                                Ok(Adt9::C7(Adt11::C5(3u64), Adt12::C2({
                                                    let a2897 = v13;
                                                    let a2898 = 1u64;
                                                    nat_add(a2897, a2898)?
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
            let a2899 = v10;
            let a2900 = 0u64;
            if nat_eq(a2899, a2900) {
                Ok(Adt9::C7(Adt11::C0(5u64), Adt12::C2(v6), Adt12::C2(0u64), Adt13::C0(0u64), 4u64, 4u64))
            } else {
                let a2901 = v10;
                let a2902 = 1u64;
                if nat_eq(a2901, a2902) {
                    Ok(Adt9::C7(Adt11::C0(2u64), Adt12::C1(0u64), Adt12::C2(0u64), Adt13::C0(0u64), 4u64, 4u64))
                } else {
                    let a2903 = v10;
                    let a2904 = 2u64;
                    if nat_eq(a2903, a2904) {
                        Ok(Adt9::C7(Adt11::C1(3u64), Adt12::C0(0u64, f105(v9, v1)?), Adt12::C1(0u64), Adt13::C0(0u64), 4u64, 4u64))
                    } else {
                        let m2907 = f190(v3, v2, v1, v9, {
                            let a2905 = v10;
                            let a2906 = 3u64;
                            nat_sub(a2905, a2906)
                        })?;
                        match m2907 {
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

pub fn f84(v0: u64, v1: Adt3, v2: Adt3, v3: Adt6, v4: Adt6) -> R<Adt9> {
    let v5: u64 = {
        let a2910 = match v3.clone() {
            Adt6::C0(h2908, _, _, _) => {
                h2908
            }
        };
        let a2911 = match v4.clone() {
            Adt6::C0(h2909, _, _, _) => {
                h2909
            }
        };
        nat_mul(a2910, a2911)?
    };
    let v6: u64 = {
        let a2912 = v5;
        let a2913 = 2u64;
        nat_add(a2912, a2913)?
    };
    let v7: u64 = {
        let a2914 = v0;
        let a2915 = v6;
        let a2916 = 0u64;
        nat_quot(a2914, a2915, a2916)
    };
    let v8: u64 = {
        let a2917 = v0;
        let a2918 = v6;
        let a2919 = 0u64;
        nat_rem(a2917, a2918, a2919)
    };
    let a2920 = v8;
    let a2921 = 0u64;
    if nat_eq(a2920, a2921) {
        Ok(Adt9::C7(Adt11::C5(3u64), Adt12::C2(0u64), Adt12::C2(0u64), Adt13::C0(0u64), 4u64, 4u64))
    } else {
        let a2924 = {
            let a2922 = v8;
            let a2923 = 1u64;
            nat_add(a2922, a2923)?
        };
        let a2925 = v6;
        if nat_eq(a2924, a2925) {
            Ok(Adt9::C7(Adt11::C5(3u64), Adt12::C1(0u64), Adt12::C2(0u64), Adt13::C1(f105(v7, v2)?), 4u64, 4u64))
        } else {
            let v9: u64 = {
                let a2926 = v8;
                let a2927 = 1u64;
                nat_sub(a2926, a2927)
            };
            let m2933 = f191(v3.clone(), match v1.clone() {
                Adt3::C0(_, h2928, _, _, _, _, _, _) => {
                    h2928
                }
            }, f65(v7, v2.clone(), 0u64), {
                let a2930 = v9;
                let a2931 = match v3.clone() {
                    Adt6::C0(h2929, _, _, _) => {
                        h2929
                    }
                };
                let a2932 = 0u64;
                nat_rem(a2930, a2931, a2932)
            })?;
            match m2933 {
                None => {
                    Ok(Adt9::C7(Adt11::C5(3u64), Adt12::C1(0u64), Adt12::C2(0u64), Adt13::C0(0u64), 4u64, 4u64))
                }
                Some(v10) => {
                    let m2939 = f191(v4, match v1.clone() {
                        Adt3::C0(_, _, h2934, _, _, _, _, _) => {
                            h2934
                        }
                    }, f65(v7, v2.clone(), 1u64), {
                        let a2936 = v9;
                        let a2937 = match v3 {
                            Adt6::C0(h2935, _, _, _) => {
                                h2935
                            }
                        };
                        let a2938 = 0u64;
                        nat_quot(a2936, a2937, a2938)
                    })?;
                    match m2939 {
                        None => {
                            Ok(Adt9::C7(Adt11::C5(3u64), Adt12::C1(0u64), Adt12::C2(0u64), Adt13::C0(0u64), 4u64, 4u64))
                        }
                        Some(v11) => {
                            Ok(Adt9::C7(Adt11::C1(0u64), Adt12::C1(0u64), Adt12::C0(0u64, f64(v1, {
                                let a2942 = {
                                    let a2940 = f65(v7, v2.clone(), 2u64);
                                    let a2941 = v5;
                                    nat_mul(a2940, a2941)?
                                };
                                let a2943 = v9;
                                nat_add(a2942, a2943)?
                            }, v10, v11, f65(v7, v2, 3u64))?), Adt13::C0(0u64), 4u64, 4u64))
                        }
                    }
                }
            }
        }
    }
}

pub fn f85(v0: u64, v1: Adt3, v2: Adt3, v3: bool, v4: u64, v5: u64, v6: Adt1, v7: u64) -> R<Adt9> {
    let v8: u64 = f192(v3, v5)?;
    let v9: u64 = {
        let a2944 = v0;
        let a2945 = v8;
        let a2946 = 0u64;
        nat_rem(a2944, a2945, a2946)
    };
    let v10: u64 = {
        let a2947 = v0;
        let a2948 = v8;
        let a2949 = 0u64;
        nat_quot(a2947, a2948, a2949)
    };
    let v11: Adt13 = Adt13::C1(f105(v10, v2)?);
    let v12: Adt13 = {
        let a2950 = v5;
        let a2951 = 0u64;
        if nat_eq(a2950, a2951) {
            v11.clone()
        } else {
            Adt13::C0(0u64)
        }
    };
    let a2952 = v9;
    let a2953 = 0u64;
    if nat_eq(a2952, a2953) {
        Ok(Adt9::C7(Adt11::C2(v4, 0u64, true), Adt12::C0(0u64, f105(v10, v1)?), Adt12::C2(0u64), Adt13::C0(0u64), f193(v4), 4u64))
    } else {
        let a2954 = v9;
        let a2955 = 1u64;
        if nat_eq(a2954, a2955) {
            Ok(Adt9::C7(Adt11::C5(0u64), Adt12::C1(0u64), Adt12::C2(0u64), Adt13::C0(2u64), 4u64, 4u64))
        } else {
            let a2956 = v9;
            let a2957 = if v3 {
                8u64
            } else {
                3u64
            };
            if nat_eq(a2956, a2957) {
                Ok(Adt9::C7(Adt11::C2(0u64, v5, true), Adt12::C1(0u64), Adt12::C2(0u64), v11, 4u64, f193(v5)))
            } else {
                if v3 {
                    Ok(f194(v9, v7, v6, v12))
                } else {
                    Ok(Adt9::C7({
                        let a2958 = v7;
                        let a2959 = 2u64;
                        if nat_eq(a2958, a2959) {
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

pub fn f86(v0: u64, v1: Adt3, v2: Adt3, v3: u64) -> R<Adt9> {
    let v4: u64 = match v1 {
        Adt3::C0(h2960, _, _, _, _, _, _, _) => {
            h2960
        }
    };
    let v5: u64 = match v2 {
        Adt3::C0(h2961, _, _, _, _, _, _, _) => {
            h2961
        }
    };
    let v6: u64 = f195(v4)?;
    let v7: u64 = {
        let a2962 = v0;
        let a2963 = v6;
        let a2964 = 0u64;
        nat_quot(a2962, a2963, a2964)
    };
    let v8: u64 = {
        let a2965 = v7;
        let a2966 = v5;
        let a2967 = 0u64;
        nat_quot(a2965, a2966, a2967)
    };
    let v9: u64 = {
        let a2968 = v7;
        let a2969 = v5;
        let a2970 = 0u64;
        nat_rem(a2968, a2969, a2970)
    };
    let v10: u64 = {
        let a2971 = v3;
        let a2972 = v4;
        let a2973 = 0u64;
        nat_rem(a2971, a2972, a2973)
    };
    let v11: u64 = {
        let a2978 = 4u64;
        let a2979 = {
            let a2976 = v8;
            let a2977 = {
                let a2974 = v4;
                let a2975 = v4;
                nat_mul(a2974, a2975)?
            };
            nat_mul(a2976, a2977)?
        };
        nat_mul(a2978, a2979)?
    };
    let v12: u64 = {
        let a2984 = 4u64;
        let a2985 = {
            let a2982 = v8;
            let a2983 = {
                let a2980 = v4;
                let a2981 = v5;
                nat_mul(a2980, a2981)?
            };
            nat_mul(a2982, a2983)?
        };
        nat_mul(a2984, a2985)?
    };
    let v13: u64 = {
        let a2989 = {
            let a2986 = v0;
            let a2987 = v6;
            let a2988 = 0u64;
            nat_rem(a2986, a2987, a2988)
        };
        let a2990 = f195(v10)?;
        nat_sub(a2989, a2990)
    };
    let a2993 = v13;
    let a2994 = {
        let a2991 = v10;
        let a2992 = 2u64;
        nat_mul(a2991, a2992)?
    };
    if nat_lt(a2993, a2994) {
        let v14: u64 = {
            let a2995 = v13;
            let a2996 = 2u64;
            let a2997 = 0u64;
            nat_quot(a2995, a2996, a2997)
        };
        let a3001 = {
            let a2998 = v13;
            let a2999 = 2u64;
            let a3000 = 0u64;
            nat_rem(a2998, a2999, a3000)
        };
        let a3002 = 0u64;
        if nat_eq(a3001, a3002) {
            Ok(Adt9::C7(Adt11::C1(2u64), Adt12::C0(0u64, {
                let a3009 = v11;
                let a3010 = {
                    let a3007 = 4u64;
                    let a3008 = {
                        let a3005 = {
                            let a3003 = v10;
                            let a3004 = v4;
                            nat_mul(a3003, a3004)?
                        };
                        let a3006 = v14;
                        nat_add(a3005, a3006)?
                    };
                    nat_mul(a3007, a3008)?
                };
                nat_add(a3009, a3010)?
            }), Adt12::C3({
                let a3017 = v12;
                let a3018 = {
                    let a3015 = 4u64;
                    let a3016 = {
                        let a3013 = {
                            let a3011 = v14;
                            let a3012 = v5;
                            nat_mul(a3011, a3012)?
                        };
                        let a3014 = v9;
                        nat_add(a3013, a3014)?
                    };
                    nat_mul(a3015, a3016)?
                };
                nat_add(a3017, a3018)?
            }), Adt13::C0(1u64), 4u64, 4u64))
        } else {
            Ok(Adt9::C7(Adt11::C1(0u64), {
                let a3019 = v14;
                let a3020 = 0u64;
                if nat_eq(a3019, a3020) {
                    Adt12::C2(0u64)
                } else {
                    Adt12::C1(0u64)
                }
            }, Adt12::C1(1u64), Adt13::C0(0u64), 4u64, 4u64))
        }
    } else {
        let a3023 = v13;
        let a3024 = {
            let a3021 = v10;
            let a3022 = 2u64;
            nat_mul(a3021, a3022)?
        };
        if nat_eq(a3023, a3024) {
            Ok(Adt9::C7(Adt11::C1(1u64), Adt12::C0(1u64, {
                let a3031 = v12;
                let a3032 = {
                    let a3029 = 4u64;
                    let a3030 = {
                        let a3027 = {
                            let a3025 = v10;
                            let a3026 = v5;
                            nat_mul(a3025, a3026)?
                        };
                        let a3028 = v9;
                        nat_add(a3027, a3028)?
                    };
                    nat_mul(a3029, a3030)?
                };
                nat_add(a3031, a3032)?
            }), {
                let a3033 = v10;
                let a3034 = 0u64;
                if nat_eq(a3033, a3034) {
                    Adt12::C2(0u64)
                } else {
                    Adt12::C1(0u64)
                }
            }, Adt13::C0(0u64), 4u64, 4u64))
        } else {
            let a3039 = v13;
            let a3040 = {
                let a3037 = {
                    let a3035 = v10;
                    let a3036 = 2u64;
                    nat_mul(a3035, a3036)?
                };
                let a3038 = 1u64;
                nat_add(a3037, a3038)?
            };
            if nat_eq(a3039, a3040) {
                Ok(Adt9::C7(Adt11::C1(3u64), Adt12::C1(0u64), Adt12::C0(0u64, {
                    let a3047 = v11;
                    let a3048 = {
                        let a3045 = 4u64;
                        let a3046 = {
                            let a3043 = {
                                let a3041 = v10;
                                let a3042 = v4;
                                nat_mul(a3041, a3042)?
                            };
                            let a3044 = v10;
                            nat_add(a3043, a3044)?
                        };
                        nat_mul(a3045, a3046)?
                    };
                    nat_add(a3047, a3048)?
                }), Adt13::C1({
                    let a3055 = v12;
                    let a3056 = {
                        let a3053 = 4u64;
                        let a3054 = {
                            let a3051 = {
                                let a3049 = v10;
                                let a3050 = v5;
                                nat_mul(a3049, a3050)?
                            };
                            let a3052 = v9;
                            nat_add(a3051, a3052)?
                        };
                        nat_mul(a3053, a3054)?
                    };
                    nat_add(a3055, a3056)?
                }), 4u64, 4u64))
            } else {
                Ok(Adt9::C7(Adt11::C5(3u64), Adt12::C2({
                    let a3059 = {
                        let a3057 = v10;
                        let a3058 = 1u64;
                        nat_add(a3057, a3058)?
                    };
                    let a3060 = v4;
                    if nat_eq(a3059, a3060) {
                        0u64
                    } else {
                        let a3061 = v10;
                        let a3062 = 1u64;
                        nat_add(a3061, a3062)?
                    }
                }), Adt12::C2(0u64), Adt13::C0(2u64), 4u64, 4u64))
            }
        }
    }
}

pub fn f87(v0: u64, v1: u64, v2: Adt3, v3: Adt3) -> R<Adt9> {
    let a3063 = v0;
    let a3064 = v1;
    if nat_lt(a3063, a3064) {
        Ok(Adt9::C7(Adt11::C9, Adt12::C0(0u64, f105(v0, v2)?), Adt12::C2(0u64), Adt13::C0(v0), 4u64, 4u64))
    } else {
        let v4: u64 = {
            let a3065 = v0;
            let a3066 = v1;
            nat_sub(a3065, a3066)
        };
        Ok(Adt9::C7(Adt11::C5(3u64), Adt12::C1(v4), Adt12::C2(0u64), Adt13::C1(f105(v4, v3)?), 4u64, 4u64))
    }
}

pub fn f88(v0: u64, v1: Adt3, v2: Adt3, v3: Adt3, v4: Adt6, v5: Adt6, v6: u64) -> R<Adt9> {
    let v7: u64 = {
        let a3073 = {
            let a3071 = 3u64;
            let a3072 = {
                let a3069 = match v4.clone() {
                    Adt6::C0(h3067, _, _, _) => {
                        h3067
                    }
                };
                let a3070 = match v5.clone() {
                    Adt6::C0(h3068, _, _, _) => {
                        h3068
                    }
                };
                nat_mul(a3069, a3070)?
            };
            nat_mul(a3071, a3072)?
        };
        let a3074 = 2u64;
        nat_add(a3073, a3074)?
    };
    let v8: u64 = {
        let a3075 = v0;
        let a3076 = v7;
        let a3077 = 0u64;
        nat_quot(a3075, a3076, a3077)
    };
    let v9: u64 = {
        let a3078 = v0;
        let a3079 = v7;
        let a3080 = 0u64;
        nat_rem(a3078, a3079, a3080)
    };
    let a3081 = v9;
    let a3082 = 0u64;
    if nat_eq(a3081, a3082) {
        Ok(Adt9::C7(Adt11::C2(0u64, 0u64, false), Adt12::C2(0u64), Adt12::C2(0u64), Adt13::C0(0u64), 4u64, 4u64))
    } else {
        let a3085 = {
            let a3083 = v9;
            let a3084 = 1u64;
            nat_add(a3083, a3084)?
        };
        let a3086 = v7;
        if nat_eq(a3085, a3086) {
            Ok(Adt9::C7(Adt11::C2(0u64, 0u64, false), Adt12::C1(0u64), Adt12::C2(0u64), Adt13::C1(f105(v8, v3)?), 4u64, 4u64))
        } else {
            let v10: u64 = {
                let a3087 = v9;
                let a3088 = 1u64;
                nat_sub(a3087, a3088)
            };
            let v11: u64 = {
                let a3089 = v10;
                let a3090 = 3u64;
                let a3091 = 0u64;
                nat_quot(a3089, a3090, a3091)
            };
            let v12: u64 = {
                let a3093 = v11;
                let a3094 = match v4.clone() {
                    Adt6::C0(h3092, _, _, _) => {
                        h3092
                    }
                };
                let a3095 = 0u64;
                nat_rem(a3093, a3094, a3095)
            };
            let v13: u64 = {
                let a3097 = v11;
                let a3098 = match v4.clone() {
                    Adt6::C0(h3096, _, _, _) => {
                        h3096
                    }
                };
                let a3099 = 0u64;
                nat_quot(a3097, a3098, a3099)
            };
            let v14: u64 = {
                let a3100 = v10;
                let a3101 = 3u64;
                let a3102 = 0u64;
                nat_rem(a3100, a3101, a3102)
            };
            let v15: u64 = f65(v8, v3.clone(), 2u64);
            let m3104 = f67(v4, match v2.clone() {
                Adt3::C0(h3103, _, _, _, _, _, _, _) => {
                    h3103
                }
            }, f65(v8, v3.clone(), 0u64), v12)?;
            match m3104 {
                None => {
                    Ok(Adt9::C7(Adt11::C2(0u64, 0u64, false), Adt12::C1(0u64), Adt12::C2(0u64), Adt13::C0(0u64), 4u64, 4u64))
                }
                Some(v16) => {
                    let m3106 = f67(v5, match v2.clone() {
                        Adt3::C0(_, h3105, _, _, _, _, _, _) => {
                            h3105
                        }
                    }, f65(v8, v3.clone(), 1u64), v13)?;
                    match m3106 {
                        None => {
                            Ok(Adt9::C7(Adt11::C2(0u64, 0u64, false), Adt12::C1(0u64), Adt12::C2(0u64), Adt13::C0(0u64), 4u64, 4u64))
                        }
                        Some(v17) => {
                            let a3107 = v14;
                            let a3108 = 0u64;
                            if nat_eq(a3107, a3108) {
                                Ok(Adt9::C7(Adt11::C2(v6, 0u64, false), Adt12::C0(0u64, f64(v1, v12, v13, 0u64, v15)?), Adt12::C2(0u64), Adt13::C0(1u64), f193(v6), 4u64))
                            } else {
                                let a3109 = v14;
                                let a3110 = 1u64;
                                if nat_eq(a3109, a3110) {
                                    Ok(Adt9::C7(Adt11::C1(2u64), Adt12::C1(1u64), Adt12::C0(1u64, f64(v2, v16, v17, v15, f65(v8, v3, 3u64))?), Adt13::C0(1u64), 4u64, 4u64))
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

pub fn f89(v0: u64, v1: Adt3, v2: Adt3, v3: u64) -> R<Adt9> {
    let v4: u64 = f196(match v1.clone() {
        Adt3::C0(h3111, _, _, _, _, _, _, _) => {
            h3111
        }
    })?;
    let v5: u64 = {
        let a3112 = v0;
        let a3113 = v4;
        let a3114 = 0u64;
        nat_quot(a3112, a3113, a3114)
    };
    let v6: u64 = {
        let a3115 = v0;
        let a3116 = v4;
        let a3117 = 0u64;
        nat_rem(a3115, a3116, a3117)
    };
    let a3118 = v6;
    let a3119 = 0u64;
    if nat_eq(a3118, a3119) {
        Ok(Adt9::C4(19u64, 2u64, 4286578688u64, 2u64, 0u64, 1u64, 0u64))
    } else {
        let a3120 = v6;
        let a3121 = 1u64;
        if nat_eq(a3120, a3121) {
            Ok(Adt9::C4(19u64, 2u64, 0u64, 2u64, 0u64, 1u64, 1u64))
        } else {
            let a3124 = {
                let a3122 = v6;
                let a3123 = 1u64;
                nat_add(a3122, a3123)?
            };
            let a3125 = v4;
            if nat_eq(a3124, a3125) {
                Ok(Adt9::C4(19u64, 1u64, 1u64, 2u64, 0u64, 3u64, f105(v5, v2)?))
            } else {
                let v7: u64 = {
                    let a3126 = v6;
                    let a3127 = 2u64;
                    nat_sub(a3126, a3127)
                };
                let v8: u64 = {
                    let a3128 = v7;
                    let a3129 = 3u64;
                    let a3130 = 0u64;
                    nat_quot(a3128, a3129, a3130)
                };
                let v9: u64 = {
                    let a3131 = v7;
                    let a3132 = 3u64;
                    let a3133 = 0u64;
                    nat_rem(a3131, a3132, a3133)
                };
                let v10: u64 = f105({
                    let a3137 = {
                        let a3135 = v5;
                        let a3136 = match v1.clone() {
                            Adt3::C0(h3134, _, _, _, _, _, _, _) => {
                                h3134
                            }
                        };
                        nat_mul(a3135, a3136)?
                    };
                    let a3138 = v8;
                    nat_add(a3137, a3138)?
                }, v1)?;
                let a3139 = v9;
                let a3140 = 0u64;
                if nat_eq(a3139, a3140) {
                    Ok(Adt9::C4(16u64, 1u64, 0u64, 0u64, v10, 1u64, 2u64))
                } else {
                    let a3141 = v9;
                    let a3142 = 1u64;
                    if nat_eq(a3141, a3142) {
                        let a3143 = v3;
                        let a3144 = 2u64;
                        if nat_eq(a3143, a3144) {
                            Ok(Adt9::C4(19u64, 1u64, 0u64, 2u64, 0u64, 1u64, 0u64))
                        } else {
                            Ok(Adt9::C4(19u64, 0u64, v10, 2u64, 0u64, 1u64, 0u64))
                        }
                    } else {
                        let a3145 = v3;
                        let a3146 = 2u64;
                        if if nat_eq(a3145, a3146) {
                            true
                        } else {
                            let a3147 = v3;
                            let a3148 = 3u64;
                            nat_eq(a3147, a3148)
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

pub fn f90(v0: u64, v1: u64, v2: Adt3, v3: Adt3, v4: Adt3, v5: u64) -> R<Adt9> {
    let v6: u64 = {
        let a3150 = 2u64;
        let a3151 = match v2.clone() {
            Adt3::C0(_, h3149, _, _, _, _, _, _) => {
                h3149
            }
        };
        nat_mul(a3150, a3151)?
    };
    let a3152 = v1;
    let a3153 = v6;
    if nat_lt(a3152, a3153) {
        let a3157 = {
            let a3154 = v1;
            let a3155 = 2u64;
            let a3156 = 0u64;
            nat_rem(a3154, a3155, a3156)
        };
        let a3158 = 0u64;
        if nat_eq(a3157, a3158) {
            Ok(Adt9::C4(19u64, 4u64, f105({
                let a3159 = v1;
                let a3160 = 2u64;
                let a3161 = 0u64;
                nat_quot(a3159, a3160, a3161)
            }, v4)?, 2u64, 0u64, 1u64, 2u64))
        } else {
            let a3162 = v5;
            let a3163 = 2147483648u64;
            if if nat_lt(a3162, a3163) {
                let a3165 = v5;
                let a3166 = match v3 {
                    Adt3::C0(_, h3164, _, _, _, _, _, _) => {
                        h3164
                    }
                };
                nat_lt(a3165, a3166)
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
            let a3169 = match v3.clone() {
                Adt3::C0(h3167, _, _, _, _, _, _, _) => {
                    h3167
                }
            };
            let a3170 = match v3.clone() {
                Adt3::C0(_, h3168, _, _, _, _, _, _) => {
                    h3168
                }
            };
            nat_mul(a3169, a3170)?
        };
        let a3173 = {
            let a3171 = v1;
            let a3172 = v6;
            nat_sub(a3171, a3172)
        };
        let a3174 = v7;
        if nat_lt(a3173, a3174) {
            Ok(Adt9::C4(19u64, 2u64, 0u64, 2u64, 0u64, 3u64, f105({
                let a3175 = v1;
                let a3176 = v6;
                nat_sub(a3175, a3176)
            }, v3)?))
        } else {
            let v8: u64 = {
                let a3179 = {
                    let a3177 = v1;
                    let a3178 = v6;
                    nat_sub(a3177, a3178)
                };
                let a3180 = v7;
                nat_sub(a3179, a3180)
            };
            let v9: u64 = {
                let a3182 = 1u64;
                let a3183 = f197(v0, match v2.clone() {
                    Adt3::C0(h3181, _, _, _, _, _, _, _) => {
                        h3181
                    }
                })?;
                nat_add(a3182, a3183)?
            };
            let v10: u64 = {
                let a3184 = v8;
                let a3185 = v9;
                let a3186 = 0u64;
                nat_quot(a3184, a3185, a3186)
            };
            let v11: u64 = {
                let a3187 = v8;
                let a3188 = v9;
                let a3189 = 0u64;
                nat_rem(a3187, a3188, a3189)
            };
            let a3190 = v11;
            let a3191 = 0u64;
            if nat_eq(a3190, a3191) {
                Ok(Adt9::C4(19u64, 4u64, f105(v10, v4)?, 2u64, 0u64, 1u64, 2u64))
            } else {
                if f198(v0) {
                    let a3195 = {
                        let a3192 = v11;
                        let a3193 = 2u64;
                        let a3194 = 0u64;
                        nat_rem(a3192, a3193, a3194)
                    };
                    let a3196 = 1u64;
                    if nat_eq(a3195, a3196) {
                        Ok(Adt9::C4(6u64, 0u64, f105({
                            let a3205 = {
                                let a3198 = v10;
                                let a3199 = match v2.clone() {
                                    Adt3::C0(h3197, _, _, _, _, _, _, _) => {
                                        h3197
                                    }
                                };
                                nat_mul(a3198, a3199)?
                            };
                            let a3206 = {
                                let a3202 = {
                                    let a3200 = v11;
                                    let a3201 = 1u64;
                                    nat_sub(a3200, a3201)
                                };
                                let a3203 = 2u64;
                                let a3204 = 0u64;
                                nat_quot(a3202, a3203, a3204)
                            };
                            nat_add(a3205, a3206)?
                        }, v2)?, 2u64, 0u64, 1u64, 0u64))
                    } else {
                        Ok(Adt9::C4(4u64, 3u64, f105({
                            let a3215 = {
                                let a3208 = v5;
                                let a3209 = match v2.clone() {
                                    Adt3::C0(h3207, _, _, _, _, _, _, _) => {
                                        h3207
                                    }
                                };
                                nat_mul(a3208, a3209)?
                            };
                            let a3216 = {
                                let a3212 = {
                                    let a3210 = v11;
                                    let a3211 = 1u64;
                                    nat_sub(a3210, a3211)
                                };
                                let a3213 = 2u64;
                                let a3214 = 0u64;
                                nat_quot(a3212, a3213, a3214)
                            };
                            nat_add(a3215, a3216)?
                        }, v3.clone())?, 1u64, 0u64, 3u64, f105({
                            let a3225 = {
                                let a3218 = v5;
                                let a3219 = match v2 {
                                    Adt3::C0(h3217, _, _, _, _, _, _, _) => {
                                        h3217
                                    }
                                };
                                nat_mul(a3218, a3219)?
                            };
                            let a3226 = {
                                let a3222 = {
                                    let a3220 = v11;
                                    let a3221 = 1u64;
                                    nat_sub(a3220, a3221)
                                };
                                let a3223 = 2u64;
                                let a3224 = 0u64;
                                nat_quot(a3222, a3223, a3224)
                            };
                            nat_add(a3225, a3226)?
                        }, v3)?))
                    }
                } else {
                    f199(v0, v10, {
                        let a3227 = v11;
                        let a3228 = 1u64;
                        nat_sub(a3227, a3228)
                    }, v5, v2, v3)
                }
            }
        }
    }
}

pub fn f91(v0: u64, v1: u64, v2: Adt3, v3: Adt3, v4: Adt3, v5: u64) -> R<Adt9> {
    let a3229 = v1;
    let a3230 = f200(v2.clone(), v4.clone())?;
    if nat_lt(a3229, a3230) {
        let a3233 = v1;
        let a3234 = {
            let a3231 = 2u64;
            let a3232 = f201(v4.clone())?;
            nat_mul(a3231, a3232)?
        };
        if nat_lt(a3233, a3234) {
            let a3238 = {
                let a3235 = v1;
                let a3236 = 2u64;
                let a3237 = 0u64;
                nat_rem(a3235, a3236, a3237)
            };
            let a3239 = 0u64;
            if nat_eq(a3238, a3239) {
                f202({
                    let a3240 = v1;
                    let a3241 = 2u64;
                    let a3242 = 0u64;
                    nat_quot(a3240, a3241, a3242)
                }, v4)
            } else {
                if f203(match v2 {
                    Adt3::C0(_, h3243, _, _, _, _, _, _) => {
                        h3243
                    }
                }, v5) {
                    Ok(Adt9::C4(19u64, 1u64, 2u64, 2u64, 0u64, 1u64, 2u64))
                } else {
                    Ok(Adt9::C0(Adt10::C1))
                }
            }
        } else {
            let v6: u64 = {
                let a3246 = v1;
                let a3247 = {
                    let a3244 = 2u64;
                    let a3245 = f201(v4.clone())?;
                    nat_mul(a3244, a3245)?
                };
                nat_sub(a3246, a3247)
            };
            let v7: u64 = {
                let a3251 = v6;
                let a3252 = {
                    let a3249 = 1u64;
                    let a3250 = match v2.clone() {
                        Adt3::C0(h3248, _, _, _, _, _, _, _) => {
                            h3248
                        }
                    };
                    nat_add(a3249, a3250)?
                };
                let a3253 = 0u64;
                nat_quot(a3251, a3252, a3253)
            };
            let v8: u64 = {
                let a3257 = v6;
                let a3258 = {
                    let a3255 = 1u64;
                    let a3256 = match v2.clone() {
                        Adt3::C0(h3254, _, _, _, _, _, _, _) => {
                            h3254
                        }
                    };
                    nat_add(a3255, a3256)?
                };
                let a3259 = 0u64;
                nat_rem(a3257, a3258, a3259)
            };
            let a3260 = v8;
            let a3261 = 0u64;
            if nat_eq(a3260, a3261) {
                f202(v7, v4)
            } else {
                f204(v0, v7, {
                    let a3262 = v8;
                    let a3263 = 1u64;
                    nat_sub(a3262, a3263)
                }, v2, v3, v4, v5)
            }
        }
    } else {
        Ok(Adt9::C0(Adt10::C1))
    }
}

pub fn f92(v0: u64, v1: Adt3, v2: Adt3, v3: Adt3, v4: Adt3, v5: u64) -> R<Adt9> {
    let v6: u64 = {
        let a3266 = match v4.clone() {
            Adt3::C0(h3264, _, _, _, _, _, _, _) => {
                h3264
            }
        };
        let a3267 = match v4.clone() {
            Adt3::C0(_, h3265, _, _, _, _, _, _) => {
                h3265
            }
        };
        nat_mul(a3266, a3267)?
    };
    let v7: u64 = {
        let a3268 = 2u64;
        let a3269 = v6;
        nat_mul(a3268, a3269)?
    };
    let v8: u64 = f205(match v3.clone() {
        Adt3::C0(_, h3270, _, _, _, _, _, _) => {
            h3270
        }
    }, match v3.clone() {
        Adt3::C0(_, _, h3271, _, _, _, _, _) => {
            h3271
        }
    }, match v3.clone() {
        Adt3::C0(_, _, _, h3272, _, _, _, _) => {
            h3272
        }
    });
    let a3273 = v0;
    let a3274 = v7;
    if nat_lt(a3273, a3274) {
        let a3278 = {
            let a3275 = v0;
            let a3276 = 2u64;
            let a3277 = 0u64;
            nat_rem(a3275, a3276, a3277)
        };
        let a3279 = 0u64;
        if nat_eq(a3278, a3279) {
            Ok(Adt9::C4(19u64, 5u64, f105({
                let a3280 = v0;
                let a3281 = 2u64;
                let a3282 = 0u64;
                nat_quot(a3280, a3281, a3282)
            }, v4)?, 2u64, 0u64, 1u64, 2u64))
        } else {
            if f206(v5, v8) {
                Ok(Adt9::C4(19u64, 1u64, 2u64, 2u64, 0u64, 1u64, 2u64))
            } else {
                Ok(Adt9::C0(Adt10::C1))
            }
        }
    } else {
        let v9: u64 = {
            let a3283 = v0;
            let a3284 = v7;
            nat_sub(a3283, a3284)
        };
        let v10: u64 = {
            let a3288 = v9;
            let a3289 = {
                let a3286 = 1u64;
                let a3287 = match v1.clone() {
                    Adt3::C0(h3285, _, _, _, _, _, _, _) => {
                        h3285
                    }
                };
                nat_add(a3286, a3287)?
            };
            let a3290 = 0u64;
            nat_quot(a3288, a3289, a3290)
        };
        let v11: u64 = {
            let a3294 = v9;
            let a3295 = {
                let a3292 = 1u64;
                let a3293 = match v1.clone() {
                    Adt3::C0(h3291, _, _, _, _, _, _, _) => {
                        h3291
                    }
                };
                nat_add(a3292, a3293)?
            };
            let a3296 = 0u64;
            nat_rem(a3294, a3295, a3296)
        };
        let a3297 = v11;
        let a3298 = 0u64;
        if nat_eq(a3297, a3298) {
            Ok(Adt9::C4(19u64, 5u64, f105({
                let a3299 = v10;
                let a3300 = v6;
                let a3301 = 0u64;
                nat_rem(a3299, a3300, a3301)
            }, v4)?, 2u64, 0u64, 1u64, 2u64))
        } else {
            if f206(v5, v8) {
                let v12: u64 = {
                    let a3307 = {
                        let a3303 = v10;
                        let a3304 = match v1.clone() {
                            Adt3::C0(h3302, _, _, _, _, _, _, _) => {
                                h3302
                            }
                        };
                        nat_mul(a3303, a3304)?
                    };
                    let a3308 = {
                        let a3305 = v11;
                        let a3306 = 1u64;
                        nat_sub(a3305, a3306)
                    };
                    nat_add(a3307, a3308)?
                };
                let a3309 = v5;
                let a3310 = 4294967295u64;
                if nat_eq(a3309, a3310) {
                    Ok(Adt9::C1(false, f105(v12, v1)?, f105(v12, v2)?))
                } else {
                    Ok(Adt9::C4(8u64, 0u64, f105(v12, v1)?, 4u64, f105({
                        let a3316 = {
                            let a3312 = v5;
                            let a3313 = match v3.clone() {
                                Adt3::C0(h3311, _, _, _, _, _, _, _) => {
                                    h3311
                                }
                            };
                            nat_mul(a3312, a3313)?
                        };
                        let a3317 = {
                            let a3314 = v11;
                            let a3315 = 1u64;
                            nat_sub(a3314, a3315)
                        };
                        nat_add(a3316, a3317)?
                    }, v3)?, 3u64, f105(v12, v2)?))
                }
            } else {
                Ok(Adt9::C0(Adt10::C1))
            }
        }
    }
}

pub fn f93(v0: u64, v1: Adt3, v2: Adt3, v3: Adt7, v4: u64) -> R<Adt9> {
    let v5: u64 = f207(v3.clone())?;
    let v6: u64 = {
        let a3318 = v0;
        let a3319 = v5;
        let a3320 = 0u64;
        nat_quot(a3318, a3319, a3320)
    };
    let v7: u64 = {
        let a3321 = v0;
        let a3322 = v5;
        let a3323 = 0u64;
        nat_rem(a3321, a3322, a3323)
    };
    let v8: u64 = f208(v3.clone())?;
    let a3324 = v7;
    let a3325 = 0u64;
    if nat_eq(a3324, a3325) {
        Ok(Adt9::C4(19u64, 2u64, if match v3 {
            Adt7::C0(_, _, _, h3326, _) => {
                h3326
            }
        } {
            4286578687u64
        } else {
            0u64
        }, 2u64, 0u64, 1u64, 0u64))
    } else {
        let a3330 = v5;
        let a3331 = {
            let a3328 = v7;
            let a3329 = if match v3.clone() {
                Adt7::C0(_, _, _, h3327, _) => {
                    h3327
                }
            } {
                1u64
            } else {
                2u64
            };
            nat_add(a3328, a3329)?
        };
        if nat_le(a3330, a3331) {
            if match v3.clone() {
                Adt7::C0(_, _, _, h3332, _) => {
                    h3332
                }
            } {
                Ok(Adt9::C4(19u64, 1u64, 0u64, 2u64, 0u64, 3u64, f105(v6, v2)?))
            } else {
                let a3335 = {
                    let a3333 = v7;
                    let a3334 = 2u64;
                    nat_add(a3333, a3334)?
                };
                let a3336 = v5;
                if nat_eq(a3335, a3336) {
                    let v9: u64 = if match v3.clone() {
                        Adt7::C0(_, _, h3337, _, _) => {
                            h3337
                        }
                    } {
                        let a3344 = {
                            let m3340 = match v3.clone() {
                                Adt7::C0(h3339, _, _, _, _) => {
                                    h3339
                                }
                            };
                            match m3340 {
                                Adt6::C0(h3338, _, _, _) => {
                                    h3338
                                }
                            }
                        };
                        let a3345 = {
                            let m3343 = match v3 {
                                Adt7::C0(_, h3342, _, _, _) => {
                                    h3342
                                }
                            };
                            match m3343 {
                                Adt6::C0(h3341, _, _, _) => {
                                    h3341
                                }
                            }
                        };
                        nat_mul(a3344, a3345)?
                    } else {
                        f209(match v3 {
                            Adt7::C0(h3346, _, _, _, _) => {
                                h3346
                            }
                        }, match v1 {
                            Adt3::C0(h3347, _, _, _, _, _, _, _) => {
                                h3347
                            }
                        }, f65(v6, v2, 0u64))?
                    };
                    Ok(Adt9::C4(3u64, 2u64, {
                        let a3348 = v9;
                        let a3349 = 0u64;
                        if nat_eq(a3348, a3349) {
                            1u64
                        } else {
                            v9
                        }
                    }, 2u64, 0u64, 1u64, 1u64))
                } else {
                    Ok(Adt9::C4(5u64, 1u64, 0u64, 1u64, 1u64, 3u64, f105(v6, v2)?))
                }
            }
        } else {
            let v10: u64 = {
                let a3352 = {
                    let a3350 = v7;
                    let a3351 = 1u64;
                    nat_sub(a3350, a3351)
                };
                let a3353 = v8;
                let a3354 = 0u64;
                nat_rem(a3352, a3353, a3354)
            };
            let m3360 = f190(v3.clone(), v1, v2, v6, {
                let a3357 = {
                    let a3355 = v7;
                    let a3356 = 1u64;
                    nat_sub(a3355, a3356)
                };
                let a3358 = v8;
                let a3359 = 0u64;
                nat_quot(a3357, a3358, a3359)
            })?;
            match m3360 {
                None => {
                    Ok(Adt9::C4(19u64, 1u64, 0u64, 2u64, 0u64, 1u64, 0u64))
                }
                Some(v11) => {
                    let a3362 = match v3.clone() {
                        Adt7::C0(_, _, _, _, h3361) => {
                            h3361
                        }
                    };
                    let a3363 = 1u64;
                    if if nat_eq(a3362, a3363) {
                        let a3364 = v10;
                        let a3365 = 0u64;
                        nat_eq(a3364, a3365)
                    } else {
                        false
                    } {
                        Ok(Adt9::C4(6u64, 0u64, v11, 2u64, 0u64, 1u64, 1u64))
                    } else {
                        if match v3.clone() {
                            Adt7::C0(_, _, _, h3366, _) => {
                                h3366
                            }
                        } {
                            let a3369 = {
                                let a3367 = v10;
                                let a3368 = 1u64;
                                nat_add(a3367, a3368)?
                            };
                            let a3370 = v8;
                            if nat_eq(a3369, a3370) {
                                let a3371 = v4;
                                let a3372 = 2u64;
                                if nat_eq(a3371, a3372) {
                                    Ok(Adt9::C4(19u64, 1u64, 0u64, 2u64, 0u64, 1u64, 0u64))
                                } else {
                                    Ok(Adt9::C4(19u64, {
                                        let a3374 = match v3.clone() {
                                            Adt7::C0(_, _, _, _, h3373) => {
                                                h3373
                                            }
                                        };
                                        let a3375 = 1u64;
                                        if nat_eq(a3374, a3375) {
                                            1u64
                                        } else {
                                            0u64
                                        }
                                    }, {
                                        let a3377 = match v3 {
                                            Adt7::C0(_, _, _, _, h3376) => {
                                                h3376
                                            }
                                        };
                                        let a3378 = 1u64;
                                        if nat_eq(a3377, a3378) {
                                            1u64
                                        } else {
                                            v11
                                        }
                                    }, 2u64, 0u64, 1u64, 0u64))
                                }
                            } else {
                                Ok(Adt9::C4(16u64, 1u64, 0u64, {
                                    let a3380 = match v3.clone() {
                                        Adt7::C0(_, _, _, _, h3379) => {
                                            h3379
                                        }
                                    };
                                    let a3381 = 1u64;
                                    if nat_eq(a3380, a3381) {
                                        1u64
                                    } else {
                                        0u64
                                    }
                                }, {
                                    let a3383 = match v3 {
                                        Adt7::C0(_, _, _, _, h3382) => {
                                            h3382
                                        }
                                    };
                                    let a3384 = 1u64;
                                    if nat_eq(a3383, a3384) {
                                        1u64
                                    } else {
                                        v11
                                    }
                                }, 1u64, 2u64))
                            }
                        } else {
                            Ok(Adt9::C4(4u64, 1u64, 0u64, {
                                let a3386 = match v3.clone() {
                                    Adt7::C0(_, _, _, _, h3385) => {
                                        h3385
                                    }
                                };
                                let a3387 = 1u64;
                                if nat_eq(a3386, a3387) {
                                    1u64
                                } else {
                                    0u64
                                }
                            }, {
                                let a3389 = match v3 {
                                    Adt7::C0(_, _, _, _, h3388) => {
                                        h3388
                                    }
                                };
                                let a3390 = 1u64;
                                if nat_eq(a3389, a3390) {
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

pub fn f94(v0: u64, v1: Adt3, v2: Adt3, v3: Adt3) -> R<Adt9> {
    let v4: u64 = f210(match v3.clone() {
        Adt3::C0(h3391, _, _, _, _, _, _, _) => {
            h3391
        }
    })?;
    let v5: u64 = {
        let a3392 = v0;
        let a3393 = v4;
        let a3394 = 0u64;
        nat_rem(a3392, a3393, a3394)
    };
    let v6: u64 = {
        let a3395 = v0;
        let a3396 = v4;
        let a3397 = 0u64;
        nat_quot(a3395, a3396, a3397)
    };
    let a3398 = v5;
    let a3399 = 0u64;
    if nat_eq(a3398, a3399) {
        Ok(Adt9::C4(19u64, 2u64, 0u64, 2u64, 0u64, 1u64, 0u64))
    } else {
        let a3402 = {
            let a3400 = v5;
            let a3401 = 1u64;
            nat_add(a3400, a3401)?
        };
        let a3403 = v4;
        if nat_eq(a3402, a3403) {
            Ok(Adt9::C4(19u64, 1u64, 0u64, 2u64, 0u64, 3u64, f105(v6, v2)?))
        } else {
            let a3407 = {
                let a3404 = v5;
                let a3405 = 2u64;
                let a3406 = 0u64;
                nat_rem(a3404, a3405, a3406)
            };
            let a3408 = 0u64;
            if nat_eq(a3407, a3408) {
                Ok(Adt9::C4(4u64, 1u64, 0u64, 1u64, 1u64, 1u64, 0u64))
            } else {
                let v7: u64 = {
                    let a3411 = {
                        let a3409 = v5;
                        let a3410 = 1u64;
                        nat_sub(a3409, a3410)
                    };
                    let a3412 = 2u64;
                    let a3413 = 0u64;
                    nat_quot(a3411, a3412, a3413)
                };
                let v8: u64 = f65(v6, v2.clone(), 0u64);
                let v9: u64 = f65(v6, v2.clone(), 1u64);
                let v10: u64 = f65(v6, v2, 2u64);
                Ok(Adt9::C4(10u64, 0u64, f64(v1, {
                    let a3414 = v9;
                    let a3415 = v7;
                    nat_add(a3414, a3415)?
                }, v8, v10, 0u64)?, 4u64, f64(v3, v7, v8, 0u64, 0u64)?, 1u64, 1u64))
            }
        }
    }
}

pub fn f95(v0: u64, v1: Adt3, v2: Adt3, v3: Adt6) -> R<Adt9> {
    let v4: u64 = f211(v3.clone())?;
    let v5: u64 = {
        let a3416 = v0;
        let a3417 = v4;
        let a3418 = 0u64;
        nat_rem(a3416, a3417, a3418)
    };
    let v6: u64 = {
        let a3419 = v0;
        let a3420 = v4;
        let a3421 = 0u64;
        nat_quot(a3419, a3420, a3421)
    };
    let a3422 = v5;
    let a3423 = 0u64;
    if nat_eq(a3422, a3423) {
        Ok(Adt9::C4(19u64, 2u64, 0u64, 2u64, 0u64, 1u64, 0u64))
    } else {
        let a3426 = {
            let a3424 = v5;
            let a3425 = 1u64;
            nat_add(a3424, a3425)?
        };
        let a3427 = v4;
        if nat_eq(a3426, a3427) {
            Ok(Adt9::C4(7u64, 1u64, 0u64, 2u64, 0u64, 3u64, f105(v6, v2)?))
        } else {
            let a3431 = {
                let a3428 = v5;
                let a3429 = 2u64;
                let a3430 = 0u64;
                nat_rem(a3428, a3429, a3430)
            };
            let a3432 = 0u64;
            if nat_eq(a3431, a3432) {
                Ok(Adt9::C4(4u64, 1u64, 0u64, 1u64, 1u64, 1u64, 0u64))
            } else {
                let v7: u64 = {
                    let a3434 = f65(v6, v2.clone(), 0u64);
                    let a3435 = match v3.clone() {
                        Adt6::C0(_, _, h3433, _) => {
                            h3433
                        }
                    };
                    nat_add(a3434, a3435)?
                };
                let v8: u64 = {
                    let a3443 = {
                        let a3440 = {
                            let a3437 = v7;
                            let a3438 = match v3.clone() {
                                Adt6::C0(_, h3436, _, _) => {
                                    h3436
                                }
                            };
                            nat_add(a3437, a3438)?
                        };
                        let a3441 = match v3.clone() {
                            Adt6::C0(h3439, _, _, _) => {
                                h3439
                            }
                        };
                        nat_sub(a3440, a3441)
                    };
                    let a3444 = match v3.clone() {
                        Adt6::C0(_, h3442, _, _) => {
                            h3442
                        }
                    };
                    let a3445 = 0u64;
                    nat_quot(a3443, a3444, a3445)
                };
                let v9: u64 = {
                    let a3451 = v8;
                    let a3452 = {
                        let a3448 = {
                            let a3446 = v5;
                            let a3447 = 1u64;
                            nat_sub(a3446, a3447)
                        };
                        let a3449 = 2u64;
                        let a3450 = 0u64;
                        nat_quot(a3448, a3449, a3450)
                    };
                    nat_add(a3451, a3452)?
                };
                let v10: u64 = {
                    let a3454 = v9;
                    let a3455 = match v3.clone() {
                        Adt6::C0(_, h3453, _, _) => {
                            h3453
                        }
                    };
                    nat_mul(a3454, a3455)?
                };
                let v11: u64 = {
                    let a3456 = v7;
                    let a3457 = v10;
                    nat_sub(a3456, a3457)
                };
                let a3459 = v9;
                let a3460 = match v1.clone() {
                    Adt3::C0(_, h3458, _, _, _, _, _, _) => {
                        h3458
                    }
                };
                if if if nat_lt(a3459, a3460) {
                    let a3461 = v10;
                    let a3462 = v7;
                    nat_le(a3461, a3462)
                } else {
                    false
                } {
                    let a3464 = v11;
                    let a3465 = match v3.clone() {
                        Adt6::C0(h3463, _, _, _) => {
                            h3463
                        }
                    };
                    nat_lt(a3464, a3465)
                } else {
                    false
                } {
                    Ok(Adt9::C4(6u64, 0u64, f64(v1, {
                        let a3469 = {
                            let a3467 = f65(v6, v2, 1u64);
                            let a3468 = match v3 {
                                Adt6::C0(h3466, _, _, _) => {
                                    h3466
                                }
                            };
                            nat_mul(a3467, a3468)?
                        };
                        let a3470 = v11;
                        nat_add(a3469, a3470)?
                    }, v9, 0u64, 0u64)?, 2u64, 0u64, 1u64, 1u64))
                } else {
                    Ok(Adt9::C4(19u64, 2u64, 0u64, 2u64, 0u64, 1u64, 1u64))
                }
            }
        }
    }
}

pub fn f96(v0: u64, v1: Adt3, v2: Adt3, v3: Adt5) -> R<Adt9> {
    let v4: u64 = f65(v0, v2.clone(), 0u64);
    let v5: u64 = f65(v0, v2.clone(), 3u64);
    let v6: u64 = {
        let a3472 = v5;
        let a3473 = match v3.clone() {
            Adt5::C0(_, _, _, _, _, _, h3471, _) => {
                h3471
            }
        };
        let a3474 = 0u64;
        nat_rem(a3472, a3473, a3474)
    };
    let v7: u64 = {
        let a3476 = v5;
        let a3477 = match v3.clone() {
            Adt5::C0(_, _, _, _, _, _, h3475, _) => {
                h3475
            }
        };
        let a3478 = 0u64;
        nat_quot(a3476, a3477, a3478)
    };
    let v8: Option<u64> = f67(match v3.clone() {
        Adt5::C0(h3479, _, _, _, _, _, _, _) => {
            h3479
        }
    }, match v1.clone() {
        Adt3::C0(h3480, _, _, _, _, _, _, _) => {
            h3480
        }
    }, f65(v0, v2.clone(), 1u64), {
        let a3484 = v4;
        let a3485 = {
            let m3483 = match v3.clone() {
                Adt5::C0(h3482, _, _, _, _, _, _, _) => {
                    h3482
                }
            };
            match m3483 {
                Adt6::C0(h3481, _, _, _) => {
                    h3481
                }
            }
        };
        let a3486 = 0u64;
        nat_rem(a3484, a3485, a3486)
    })?;
    let v9: u64 = {
        let a3490 = v4;
        let a3491 = {
            let m3489 = match v3.clone() {
                Adt5::C0(h3488, _, _, _, _, _, _, _) => {
                    h3488
                }
            };
            match m3489 {
                Adt6::C0(h3487, _, _, _) => {
                    h3487
                }
            }
        };
        let a3492 = 0u64;
        nat_quot(a3490, a3491, a3492)
    };
    let v10: Option<u64> = f67(match v3.clone() {
        Adt5::C0(_, h3493, _, _, _, _, _, _) => {
            h3493
        }
    }, match v1.clone() {
        Adt3::C0(_, h3494, _, _, _, _, _, _) => {
            h3494
        }
    }, f65(v0, v2.clone(), 2u64), {
        let a3498 = v9;
        let a3499 = {
            let m3497 = match v3.clone() {
                Adt5::C0(_, h3496, _, _, _, _, _, _) => {
                    h3496
                }
            };
            match m3497 {
                Adt6::C0(h3495, _, _, _) => {
                    h3495
                }
            }
        };
        let a3500 = 0u64;
        nat_rem(a3498, a3499, a3500)
    })?;
    let v11: u64 = {
        let a3504 = v9;
        let a3505 = {
            let m3503 = match v3.clone() {
                Adt5::C0(_, h3502, _, _, _, _, _, _) => {
                    h3502
                }
            };
            match m3503 {
                Adt6::C0(h3501, _, _, _) => {
                    h3501
                }
            }
        };
        let a3506 = 0u64;
        nat_quot(a3504, a3505, a3506)
    };
    let v12: Option<u64> = f67(match v3.clone() {
        Adt5::C0(_, _, h3507, _, _, _, _, _) => {
            h3507
        }
    }, match v1.clone() {
        Adt3::C0(_, _, h3508, _, _, _, _, _) => {
            h3508
        }
    }, v6, {
        let a3512 = v11;
        let a3513 = {
            let m3511 = match v3.clone() {
                Adt5::C0(_, _, h3510, _, _, _, _, _) => {
                    h3510
                }
            };
            match m3511 {
                Adt6::C0(h3509, _, _, _) => {
                    h3509
                }
            }
        };
        let a3514 = 0u64;
        nat_rem(a3512, a3513, a3514)
    })?;
    let v13: u64 = {
        let a3518 = v11;
        let a3519 = {
            let m3517 = match v3.clone() {
                Adt5::C0(_, _, h3516, _, _, _, _, _) => {
                    h3516
                }
            };
            match m3517 {
                Adt6::C0(h3515, _, _, _) => {
                    h3515
                }
            }
        };
        let a3520 = 0u64;
        nat_quot(a3518, a3519, a3520)
    };
    let v17: Adt12 = {
        let m3521 = v8;
        match m3521 {
            None => {
                Adt12::C2(0u64)
            }
            Some(v14) => {
                let m3522 = v10;
                match m3522 {
                    None => {
                        Adt12::C2(0u64)
                    }
                    Some(v15) => {
                        let m3523 = v12;
                        match m3523 {
                            None => {
                                Adt12::C2(0u64)
                            }
                            Some(v16) => {
                                Adt12::C0(1u64, {
                                    let a3532 = {
                                        let a3527 = f64(v1, v14, v15, v16, 0u64)?;
                                        let a3528 = {
                                            let a3525 = v7;
                                            let a3526 = match v3.clone() {
                                                Adt5::C0(_, _, _, _, h3524, _, _, _) => {
                                                    h3524
                                                }
                                            };
                                            nat_mul(a3525, a3526)?
                                        };
                                        nat_add(a3527, a3528)?
                                    };
                                    let a3533 = {
                                        let a3530 = v13;
                                        let a3531 = match v3.clone() {
                                            Adt5::C0(_, _, _, h3529, _, _, _, _) => {
                                                h3529
                                            }
                                        };
                                        nat_mul(a3530, a3531)?
                                    };
                                    nat_add(a3532, a3533)?
                                })
                            }
                        }
                    }
                }
            }
        }
    };
    Ok(Adt9::C7(Adt11::C2(match v3.clone() {
        Adt5::C0(_, _, _, _, _, h3534, _, _) => {
            h3534
        }
    }, match v3.clone() {
        Adt5::C0(_, _, _, _, _, _, _, h3535) => {
            h3535
        }
    }, true), v17, Adt12::C2(0u64), Adt13::C1(f105(v0, v2)?), {
        let a3537 = match v3.clone() {
            Adt5::C0(_, _, _, _, _, h3536, _, _) => {
                h3536
            }
        };
        let a3538 = 0u64;
        if nat_eq(a3537, a3538) {
            4u64
        } else {
            2u64
        }
    }, {
        let a3540 = match v3 {
            Adt5::C0(_, _, _, _, _, _, _, h3539) => {
                h3539
            }
        };
        let a3541 = 0u64;
        if nat_eq(a3540, a3541) {
            4u64
        } else {
            2u64
        }
    }))
}

pub fn f97(v0: Adt16, v1: u64, v2: Adt3, v3: Adt3, v4: u64, v5: u64, v6: u64, v7: u64, v8: u64, v9: u64) -> R<Adt9> {
    let m3554 = f212(v0, v1, match v2.clone() {
        Adt3::C0(h3542, _, _, _, _, _, _, _) => {
            h3542
        }
    }, match v2.clone() {
        Adt3::C0(_, h3543, _, _, _, _, _, _) => {
            h3543
        }
    }, match v2.clone() {
        Adt3::C0(_, _, h3544, _, _, _, _, _) => {
            h3544
        }
    }, match v2.clone() {
        Adt3::C0(_, _, _, h3545, _, _, _, _) => {
            h3545
        }
    }, match v2.clone() {
        Adt3::C0(_, _, _, _, h3546, _, _, _) => {
            h3546
        }
    }, match v2.clone() {
        Adt3::C0(_, _, _, _, _, h3547, _, _) => {
            h3547
        }
    }, match v2.clone() {
        Adt3::C0(_, _, _, _, _, _, h3548, _) => {
            h3548
        }
    }, match v2 {
        Adt3::C0(_, _, _, _, _, _, _, h3549) => {
            h3549
        }
    }, v6, v7, v8, v9, match v3.clone() {
        Adt3::C0(_, _, _, _, h3550, _, _, _) => {
            h3550
        }
    }, match v3.clone() {
        Adt3::C0(_, _, _, _, _, h3551, _, _) => {
            h3551
        }
    }, match v3.clone() {
        Adt3::C0(_, _, _, _, _, _, h3552, _) => {
            h3552
        }
    }, match v3 {
        Adt3::C0(_, _, _, _, _, _, _, h3553) => {
            h3553
        }
    }, v4)?;
    match m3554 {
        Adt17::C0 => {
            Ok(Adt9::C0(Adt10::C1))
        }
        Adt17::C1(v10, v11, v12, v13, v14, v15, v16, v17) => {
            Ok(f23(false, v10, v11, 0u64, 0u64, v12, v13, v14, v15, v16, v17, v5, v5))
        }
    }
}

pub fn f98(v0: u64, v1: Adt3, v2: Adt3, v3: u64, v4: u64, v5: u64, v6: u64, v7: u64) -> R<Adt9> {
    let v8: u64 = {
        let a3557 = v6;
        let a3558 = {
            let a3555 = 3u64;
            let a3556 = v5;
            nat_add(a3555, a3556)?
        };
        nat_sub(a3557, a3558)
    };
    let a3560 = v0;
    let a3561 = f57(v3, match v1.clone() {
        Adt3::C0(h3559, _, _, _, _, _, _, _) => {
            h3559
        }
    }, v5, v8)?;
    if nat_lt(a3560, a3561) {
        let v9: u64 = f116(match v1.clone() {
            Adt3::C0(h3562, _, _, _, _, _, _, _) => {
                h3562
            }
        }, v5, v8)?;
        let v10: u64 = f117(v3, match v1.clone() {
            Adt3::C0(h3563, _, _, _, _, _, _, _) => {
                h3563
            }
        }, v5)?;
        let a3564 = v0;
        let a3565 = v9;
        if nat_lt(a3564, a3565) {
            f213(v0, v2, f51(v3), {
                let a3566 = 3u64;
                let a3567 = v5;
                nat_add(a3566, a3567)?
            }, v7)
        } else {
            let a3570 = v0;
            let a3571 = {
                let a3568 = v9;
                let a3569 = v10;
                nat_add(a3568, a3569)?
            };
            if nat_lt(a3570, a3571) {
                let v11: u64 = {
                    let a3572 = v0;
                    let a3573 = v9;
                    nat_sub(a3572, a3573)
                };
                let v12: u64 = f214(v3, match v1.clone() {
                    Adt3::C0(h3574, _, _, _, _, _, _, _) => {
                        h3574
                    }
                })?;
                f215({
                    let a3575 = v11;
                    let a3576 = v12;
                    let a3577 = 0u64;
                    nat_rem(a3575, a3576, a3577)
                }, {
                    let a3578 = v11;
                    let a3579 = v12;
                    let a3580 = 0u64;
                    nat_quot(a3578, a3579, a3580)
                }, v1, v3, v4, v7)
            } else {
                f216({
                    let a3583 = v0;
                    let a3584 = {
                        let a3581 = v9;
                        let a3582 = v10;
                        nat_add(a3581, a3582)?
                    };
                    nat_sub(a3583, a3584)
                }, v1, v2, v3)
            }
        }
    } else {
        Ok(Adt9::C0(Adt10::C1))
    }
}

pub fn f99(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64) -> R<Adt9> {
    let a3585 = v0;
    let a3586 = f43(v1)?;
    if nat_lt(a3585, a3586) {
        let a3587 = v0;
        let a3588 = 4u64;
        if nat_lt(a3587, a3588) {
            let a3589 = v0;
            let a3590 = 0u64;
            if nat_eq(a3589, a3590) {
                Ok(Adt9::C4(16u64, 2u64, v2, 2u64, v3, 1u64, 2u64))
            } else {
                let a3591 = v0;
                let a3592 = 1u64;
                if nat_eq(a3591, a3592) {
                    let a3593 = v5;
                    let a3594 = 0u64;
                    if nat_eq(a3593, a3594) {
                        Ok(Adt9::C4(9u64, 2u64, v3, 2u64, v2, 1u64, 2u64))
                    } else {
                        Ok(Adt9::C0(Adt10::C1))
                    }
                } else {
                    let a3595 = v0;
                    let a3596 = 2u64;
                    if nat_eq(a3595, a3596) {
                        Ok(Adt9::C4(11u64, 1u64, 2u64, 2u64, v4, 1u64, 2u64))
                    } else {
                        Ok(Adt9::C4(20u64, 1u64, 2u64, 2u64, 10u64, 1u64, 2u64))
                    }
                }
            }
        } else {
            let a3597 = v0;
            let a3598 = 4u64;
            if if nat_eq(a3597, a3598) {
                let a3599 = f217(v5)?;
                let a3600 = v1;
                !nat_eq(a3599, a3600)
            } else {
                false
            } {
                Ok(Adt9::C0(Adt10::C1))
            } else {
                let v6: u64 = {
                    let a3603 = {
                        let a3601 = v0;
                        let a3602 = 4u64;
                        nat_sub(a3601, a3602)
                    };
                    let a3604 = 3u64;
                    let a3605 = 0u64;
                    nat_quot(a3603, a3604, a3605)
                };
                let v7: u64 = {
                    let a3608 = {
                        let a3606 = v0;
                        let a3607 = 4u64;
                        nat_sub(a3606, a3607)
                    };
                    let a3609 = 3u64;
                    let a3610 = 0u64;
                    nat_rem(a3608, a3609, a3610)
                };
                let a3611 = v7;
                let a3612 = 0u64;
                if nat_eq(a3611, a3612) {
                    Ok(Adt9::C4(3u64, 2u64, v6, 2u64, 0u64, 1u64, 0u64))
                } else {
                    let a3613 = v7;
                    let a3614 = 1u64;
                    if nat_eq(a3613, a3614) {
                        Ok(Adt9::C4(10u64, 2u64, v4, 1u64, 0u64, 1u64, 0u64))
                    } else {
                        Ok(Adt9::C4(8u64, 2u64, v2, 1u64, 0u64, 3u64, f42(v6)?))
                    }
                }
            }
        }
    } else {
        Ok(Adt9::C0(Adt10::C1))
    }
}

pub fn f100(v0: bool, v1: u64, v2: u64, v3: u64, v4: u64) -> bool {
    if if if if v0 {
        let a3615 = v1;
        let a3616 = 5u64;
        nat_eq(a3615, a3616)
    } else {
        false
    } {
        let a3617 = v2;
        let a3618 = 16u64;
        nat_eq(a3617, a3618)
    } else {
        false
    } {
        let a3619 = v3;
        let a3620 = 0u64;
        nat_eq(a3619, a3620)
    } else {
        false
    } {
        let a3621 = v4;
        let a3622 = 0u64;
        nat_eq(a3621, a3622)
    } else {
        false
    }
}

pub fn f101(v0: bool, v1: u64, v2: u64, v3: u64, v4: u64) -> Adt11 {
    let a3623 = v1;
    let a3624 = 0u64;
    if nat_eq(a3623, a3624) {
        if v0 {
            let a3625 = v3;
            let a3626 = 0u64;
            if if if nat_eq(a3625, a3626) {
                let a3627 = v4;
                let a3628 = 43u64;
                nat_eq(a3627, a3628)
            } else {
                false
            } {
                true
            } else {
                let a3629 = v3;
                let a3630 = 43u64;
                if nat_eq(a3629, a3630) {
                    let a3631 = v4;
                    let a3632 = 0u64;
                    nat_eq(a3631, a3632)
                } else {
                    false
                }
            } {
                f183(v3, v4)
            } else {
                Adt11::C2(v3, v4, false)
            }
        } else {
            Adt11::C0(v2)
        }
    } else {
        let a3633 = v1;
        let a3634 = 1u64;
        if nat_eq(a3633, a3634) {
            Adt11::C1(v2)
        } else {
            let a3635 = v1;
            let a3636 = 2u64;
            if nat_eq(a3635, a3636) {
                Adt11::C5(v2)
            } else {
                let a3637 = v1;
                let a3638 = 3u64;
                if nat_eq(a3637, a3638) {
                    Adt11::C3(v2, v2, false)
                } else {
                    let a3639 = v1;
                    let a3640 = 4u64;
                    if nat_eq(a3639, a3640) {
                        Adt11::C2(v3, v4, false)
                    } else {
                        if f100(v0, v1, v2, v3, v4) {
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

pub fn f102(v0: u64, v1: u64) -> Adt12 {
    let a3641 = v0;
    let a3642 = 0u64;
    if nat_eq(a3641, a3642) {
        Adt12::C0(0u64, v1)
    } else {
        let a3643 = v0;
        let a3644 = 1u64;
        if nat_eq(a3643, a3644) {
            Adt12::C1(v1)
        } else {
            let a3645 = v0;
            let a3646 = 2u64;
            if nat_eq(a3645, a3646) {
                Adt12::C2(v1)
            } else {
                let a3647 = v0;
                let a3648 = 3u64;
                if nat_eq(a3647, a3648) {
                    Adt12::C3(v1)
                } else {
                    let a3649 = v0;
                    let a3650 = 13u64;
                    if nat_lt(a3649, a3650) {
                        Adt12::C0({
                            let a3651 = v0;
                            let a3652 = 3u64;
                            nat_sub(a3651, a3652)
                        }, v1)
                    } else {
                        Adt12::C4
                    }
                }
            }
        }
    }
}

pub fn f103(v0: u64, v1: u64) -> Adt13 {
    let a3653 = v0;
    let a3654 = 1u64;
    if nat_eq(a3653, a3654) {
        Adt13::C0(v1)
    } else {
        let a3655 = v0;
        let a3656 = 3u64;
        if if nat_eq(a3655, a3656) {
            true
        } else {
            let a3657 = v0;
            let a3658 = 6u64;
            nat_eq(a3657, a3658)
        } {
            Adt13::C1(v1)
        } else {
            Adt13::C2
        }
    }
}

pub fn f104(v0: u64) -> u64 {
    let a3659 = v0;
    let a3660 = 2147483521u64;
    if nat_lt(a3659, a3660) {
        1u64
    } else {
        let a3661 = v0;
        let a3662 = 549755813761u64;
        if nat_lt(a3661, a3662) {
            2u64
        } else {
            let a3663 = v0;
            let a3664 = 140737488355201u64;
            if nat_lt(a3663, a3664) {
                3u64
            } else {
                let a3665 = v0;
                let a3666 = 36028797018963841u64;
                if nat_lt(a3665, a3666) {
                    4u64
                } else {
                    let a3667 = v0;
                    let a3668 = 9223372036854775681u64;
                    if nat_lt(a3667, a3668) {
                        5u64
                    } else {
                        6u64
                    }
                }
            }
        }
    }
}

pub fn f105(v0: u64, v1: Adt3) -> R<u64> {
    let v2: u64 = {
        let a3670 = v0;
        let a3671 = match v1.clone() {
            Adt3::C0(h3669, _, _, _, _, _, _, _) => {
                h3669
            }
        };
        let a3672 = 0u64;
        nat_quot(a3670, a3671, a3672)
    };
    let v3: u64 = {
        let a3674 = v2;
        let a3675 = match v1.clone() {
            Adt3::C0(_, h3673, _, _, _, _, _, _) => {
                h3673
            }
        };
        let a3676 = 0u64;
        nat_quot(a3674, a3675, a3676)
    };
    f158({
        let a3678 = v0;
        let a3679 = match v1.clone() {
            Adt3::C0(h3677, _, _, _, _, _, _, _) => {
                h3677
            }
        };
        let a3680 = 0u64;
        nat_rem(a3678, a3679, a3680)
    }, {
        let a3682 = v2;
        let a3683 = match v1.clone() {
            Adt3::C0(_, h3681, _, _, _, _, _, _) => {
                h3681
            }
        };
        let a3684 = 0u64;
        nat_rem(a3682, a3683, a3684)
    }, {
        let a3686 = v3;
        let a3687 = match v1.clone() {
            Adt3::C0(_, _, h3685, _, _, _, _, _) => {
                h3685
            }
        };
        let a3688 = 0u64;
        nat_rem(a3686, a3687, a3688)
    }, {
        let a3690 = v3;
        let a3691 = match v1.clone() {
            Adt3::C0(_, _, h3689, _, _, _, _, _) => {
                h3689
            }
        };
        let a3692 = 0u64;
        nat_quot(a3690, a3691, a3692)
    }, match v1.clone() {
        Adt3::C0(_, _, _, _, h3693, _, _, _) => {
            h3693
        }
    }, match v1.clone() {
        Adt3::C0(_, _, _, _, _, h3694, _, _) => {
            h3694
        }
    }, match v1.clone() {
        Adt3::C0(_, _, _, _, _, _, h3695, _) => {
            h3695
        }
    }, match v1 {
        Adt3::C0(_, _, _, _, _, _, _, h3696) => {
            h3696
        }
    })
}

pub fn f106(v0: u64, v1: u64) -> R<u64> {
    let a3703 = v1;
    let a3704 = {
        let a3701 = v1;
        let a3702 = {
            let a3699 = 1u64;
            let a3700 = {
                let a3697 = 5u64;
                let a3698 = v0;
                nat_mul(a3697, a3698)?
            };
            nat_add(a3699, a3700)?
        };
        nat_mul(a3701, a3702)?
    };
    nat_add(a3703, a3704)
}

pub fn f107(v0: bool, v1: u64, v2: u64) -> R<u64> {
    let a3713 = v2;
    let a3714 = {
        let a3711 = v2;
        let a3712 = if v0 {
            let a3707 = 1u64;
            let a3708 = {
                let a3705 = 9u64;
                let a3706 = v1;
                nat_mul(a3705, a3706)?
            };
            nat_add(a3707, a3708)?
        } else {
            let a3709 = 7u64;
            let a3710 = v1;
            nat_mul(a3709, a3710)?
        };
        nat_mul(a3711, a3712)?
    };
    nat_add(a3713, a3714)
}

pub fn f108(v0: Adt6, v1: u64) -> R<u64> {
    let a3716 = 0u64;
    let a3717 = match v0.clone() {
        Adt6::C0(h3715, _, _, _) => {
            h3715
        }
    };
    if if if if if nat_lt(a3716, a3717) {
        let a3719 = 0u64;
        let a3720 = match v0.clone() {
            Adt6::C0(_, h3718, _, _) => {
                h3718
            }
        };
        nat_lt(a3719, a3720)
    } else {
        false
    } {
        let a3722 = 0u64;
        let a3723 = match v0.clone() {
            Adt6::C0(_, _, _, h3721) => {
                h3721
            }
        };
        nat_lt(a3722, a3723)
    } else {
        false
    } {
        let a3725 = match v0.clone() {
            Adt6::C0(_, h3724, _, _) => {
                h3724
            }
        };
        let a3726 = 2147483648u64;
        nat_lt(a3725, a3726)
    } else {
        false
    } {
        let a3728 = match v0.clone() {
            Adt6::C0(_, _, _, h3727) => {
                h3727
            }
        };
        let a3729 = 2147483648u64;
        nat_lt(a3728, a3729)
    } else {
        false
    } {
        let v2: u64 = {
            let a3733 = v1;
            let a3734 = {
                let a3731 = 2u64;
                let a3732 = f160(match v0.clone() {
                    Adt6::C0(_, _, h3730, _) => {
                        h3730
                    }
                });
                nat_mul(a3731, a3732)?
            };
            nat_add(a3733, a3734)?
        };
        let v3: u64 = {
            let a3746 = {
                let a3741 = 1u64;
                let a3742 = {
                    let a3739 = match v0.clone() {
                        Adt6::C0(_, _, _, h3735) => {
                            h3735
                        }
                    };
                    let a3740 = {
                        let a3737 = match v0.clone() {
                            Adt6::C0(h3736, _, _, _) => {
                                h3736
                            }
                        };
                        let a3738 = 1u64;
                        nat_sub(a3737, a3738)
                    };
                    nat_mul(a3739, a3740)?
                };
                nat_add(a3741, a3742)?
            };
            let a3747 = {
                let a3744 = 2u64;
                let a3745 = f159(match v0.clone() {
                    Adt6::C0(_, _, h3743, _) => {
                        h3743
                    }
                });
                nat_mul(a3744, a3745)?
            };
            nat_add(a3746, a3747)?
        };
        let a3748 = v3;
        let a3749 = v2;
        if nat_le(a3748, a3749) {
            let a3756 = 1u64;
            let a3757 = {
                let a3753 = {
                    let a3750 = v2;
                    let a3751 = v3;
                    nat_sub(a3750, a3751)
                };
                let a3754 = match v0 {
                    Adt6::C0(_, h3752, _, _) => {
                        h3752
                    }
                };
                let a3755 = 0u64;
                nat_quot(a3753, a3754, a3755)
            };
            nat_add(a3756, a3757)
        } else {
            let a3761 = {
                let a3758 = v3;
                let a3759 = v2;
                nat_sub(a3758, a3759)
            };
            let a3762 = match v0 {
                Adt6::C0(_, h3760, _, _) => {
                    h3760
                }
            };
            if nat_lt(a3761, a3762) {
                Ok(1u64)
            } else {
                Ok(0u64)
            }
        }
    } else {
        Ok(0u64)
    }
}

pub fn f109(v0: u64) -> u64 {
    let v1: u64 = {
        let a3763 = v0;
        let a3764 = 4294967296u64;
        let a3765 = 0u64;
        nat_rem(a3763, a3764, a3765)
    };
    let a3766 = v1;
    let a3767 = 2147483648u64;
    if nat_lt(a3766, a3767) {
        v1
    } else {
        0u64
    }
}

pub fn f110(v0: u64) -> R<u64> {
    let a3770 = 2u64;
    let a3771 = {
        let a3768 = 2u64;
        let a3769 = v0;
        nat_mul(a3768, a3769)?
    };
    nat_add(a3770, a3771)
}

pub fn f111(v0: u64, v1: Adt14, v2: Adt14) -> R<Adt15> {
    let a3773 = match v1.clone() {
        Adt14::C0(h3772, _, _, _, _, _, _, _, _, _, _) => {
            h3772
        }
    };
    let a3774 = 0u64;
    if if if if nat_eq(a3773, a3774) {
        let a3776 = match v2.clone() {
            Adt14::C0(h3775, _, _, _, _, _, _, _, _, _, _) => {
                h3775
            }
        };
        let a3777 = 0u64;
        nat_eq(a3776, a3777)
    } else {
        false
    } {
        f218(v1.clone(), v2.clone())
    } else {
        false
    } {
        if f219(match v1.clone() {
            Adt14::C0(_, h3778, _, _, _, _, _, _, _, _, _) => {
                h3778
            }
        }, match v1.clone() {
            Adt14::C0(_, _, h3779, _, _, _, _, _, _, _, _) => {
                h3779
            }
        }, match v1.clone() {
            Adt14::C0(_, _, _, h3780, _, _, _, _, _, _, _) => {
                h3780
            }
        }, match v1.clone() {
            Adt14::C0(_, _, _, _, h3781, _, _, _, _, _, _) => {
                h3781
            }
        }) {
            true
        } else {
            if if if if f52(v0) {
                f220(v1.clone(), v2.clone())
            } else {
                false
            } {
                f221(v1.clone())
            } else {
                false
            } {
                f221(v2.clone())
            } else {
                false
            } {
                if f222(v1.clone(), v2.clone())? {
                    true
                } else {
                    f223(v1.clone(), v2.clone())
                }
            } else {
                false
            }
        }
    } else {
        false
    } {
        let v3: u64 = if f219(match v1.clone() {
            Adt14::C0(_, h3782, _, _, _, _, _, _, _, _, _) => {
                h3782
            }
        }, match v1.clone() {
            Adt14::C0(_, _, h3783, _, _, _, _, _, _, _, _) => {
                h3783
            }
        }, match v1.clone() {
            Adt14::C0(_, _, _, h3784, _, _, _, _, _, _, _) => {
                h3784
            }
        }, match v1.clone() {
            Adt14::C0(_, _, _, _, h3785, _, _, _, _, _, _) => {
                h3785
            }
        }) {
            0u64
        } else {
            let a3791 = match v1.clone() {
                Adt14::C0(_, _, h3786, _, _, _, _, _, _, _, _) => {
                    h3786
                }
            };
            let a3792 = {
                let a3789 = match v1.clone() {
                    Adt14::C0(_, _, _, h3787, _, _, _, _, _, _, _) => {
                        h3787
                    }
                };
                let a3790 = match v1.clone() {
                    Adt14::C0(_, _, _, _, h3788, _, _, _, _, _, _) => {
                        h3788
                    }
                };
                nat_mul(a3789, a3790)?
            };
            nat_mul(a3791, a3792)?
        };
        let v4: u64 = {
            let a3793 = v3;
            let a3794 = 0u64;
            if if nat_eq(a3793, a3794) {
                true
            } else {
                f224(v2.clone())?
            } {
                0u64
            } else {
                let a3796 = match v2.clone() {
                    Adt14::C0(_, _, _, _, _, _, _, _, _, h3795, _) => {
                        h3795
                    }
                };
                let a3797 = 4u64;
                let a3798 = 0u64;
                nat_quot(a3796, a3797, a3798)
            }
        };
        Ok(Adt15::C1(f225(f226(match v1.clone() {
            Adt14::C0(_, _, _, _, _, h3799, _, _, _, _, _) => {
                h3799
            }
        }, match v2 {
            Adt14::C0(_, _, _, _, _, h3800, _, _, _, _, _) => {
                h3800
            }
        }), match v1 {
            Adt14::C0(_, h3801, _, _, _, _, _, _, _, _, _) => {
                h3801
            }
        }, v3, v4)?, 4u64, 64u64, v3, f227(v4)?, v0))
    } else {
        Ok(Adt15::C0)
    }
}

pub fn f112(v0: u64, v1: u64, v2: Adt14, v3: Adt14) -> R<Adt15> {
    let v4: Adt14 = f228(v3.clone());
    let a3803 = match v2.clone() {
        Adt14::C0(h3802, _, _, _, _, _, _, _, _, _, _) => {
            h3802
        }
    };
    let a3804 = 0u64;
    if if if if nat_eq(a3803, a3804) {
        let a3806 = match v3.clone() {
            Adt14::C0(h3805, _, _, _, _, _, _, _, _, _, _) => {
                h3805
            }
        };
        let a3807 = 0u64;
        nat_eq(a3806, a3807)
    } else {
        false
    } {
        f218(v2.clone(), v3)
    } else {
        false
    } {
        if f219(match v2.clone() {
            Adt14::C0(_, h3808, _, _, _, _, _, _, _, _, _) => {
                h3808
            }
        }, match v2.clone() {
            Adt14::C0(_, _, h3809, _, _, _, _, _, _, _, _) => {
                h3809
            }
        }, match v2.clone() {
            Adt14::C0(_, _, _, h3810, _, _, _, _, _, _, _) => {
                h3810
            }
        }, match v2.clone() {
            Adt14::C0(_, _, _, _, h3811, _, _, _, _, _, _) => {
                h3811
            }
        }) {
            true
        } else {
            let a3813 = match v2.clone() {
                Adt14::C0(_, _, _, _, _, h3812, _, _, _, _, _) => {
                    h3812
                }
            };
            let a3814 = 4u64;
            if if if if if nat_eq(a3813, a3814) {
                f229(match v2.clone() {
                    Adt14::C0(_, h3815, _, _, _, _, _, _, _, _, _) => {
                        h3815
                    }
                }, match v2.clone() {
                    Adt14::C0(_, _, h3816, _, _, _, _, _, _, _, _) => {
                        h3816
                    }
                }, match v2.clone() {
                    Adt14::C0(_, _, _, h3817, _, _, _, _, _, _, _) => {
                        h3817
                    }
                }, v0)?
            } else {
                false
            } {
                f221(v2.clone())
            } else {
                false
            } {
                f221(v4.clone())
            } else {
                false
            } {
                let a3819 = f230(v4.clone())?;
                let a3820 = match v4.clone() {
                    Adt14::C0(_, _, _, _, _, _, _, _, _, h3818, _) => {
                        h3818
                    }
                };
                nat_le(a3819, a3820)
            } else {
                false
            } {
                if f222(v2.clone(), v4.clone())? {
                    true
                } else {
                    f223(v2.clone(), v4.clone())
                }
            } else {
                false
            }
        }
    } else {
        false
    } {
        let v5: u64 = if if f219(match v2.clone() {
            Adt14::C0(_, h3821, _, _, _, _, _, _, _, _, _) => {
                h3821
            }
        }, match v2.clone() {
            Adt14::C0(_, _, h3822, _, _, _, _, _, _, _, _) => {
                h3822
            }
        }, match v2.clone() {
            Adt14::C0(_, _, _, h3823, _, _, _, _, _, _, _) => {
                h3823
            }
        }, match v2.clone() {
            Adt14::C0(_, _, _, _, h3824, _, _, _, _, _, _) => {
                h3824
            }
        }) {
            true
        } else {
            f224(v4.clone())?
        } {
            0u64
        } else {
            let a3826 = match v4 {
                Adt14::C0(_, _, _, _, _, _, _, _, _, h3825, _) => {
                    h3825
                }
            };
            let a3827 = 4u64;
            let a3828 = 0u64;
            nat_quot(a3826, a3827, a3828)
        };
        Ok(Adt15::C1(f231(match v2.clone() {
            Adt14::C0(_, h3829, _, _, _, _, _, _, _, _, _) => {
                h3829
            }
        }, match v2.clone() {
            Adt14::C0(_, _, h3830, _, _, _, _, _, _, _, _) => {
                h3830
            }
        }, match v2.clone() {
            Adt14::C0(_, _, _, h3831, _, _, _, _, _, _, _) => {
                h3831
            }
        }, match v2 {
            Adt14::C0(_, _, _, _, h3832, _, _, _, _, _, _) => {
                h3832
            }
        }, v0, v5)?, 4u64, 65u64, v0, f227(v5)?, v1))
    } else {
        Ok(Adt15::C0)
    }
}

pub fn f113(v0: u64, v1: bool, v2: Adt14, v3: Adt14, v4: Adt14) -> R<Adt15> {
    let v5: Adt14 = f228(v4.clone());
    if if if if if if if v1 {
        let a3834 = match v2.clone() {
            Adt14::C0(h3833, _, _, _, _, _, _, _, _, _, _) => {
                h3833
            }
        };
        let a3835 = 0u64;
        nat_eq(a3834, a3835)
    } else {
        false
    } {
        let a3837 = match v3.clone() {
            Adt14::C0(h3836, _, _, _, _, _, _, _, _, _, _) => {
                h3836
            }
        };
        let a3838 = 0u64;
        nat_eq(a3837, a3838)
    } else {
        false
    } {
        let a3840 = match v4.clone() {
            Adt14::C0(h3839, _, _, _, _, _, _, _, _, _, _) => {
                h3839
            }
        };
        let a3841 = 0u64;
        nat_eq(a3840, a3841)
    } else {
        false
    } {
        f218(v2.clone(), v3.clone())
    } else {
        false
    } {
        f218(v2.clone(), v4)
    } else {
        false
    } {
        if f219(match v2.clone() {
            Adt14::C0(_, h3842, _, _, _, _, _, _, _, _, _) => {
                h3842
            }
        }, match v2.clone() {
            Adt14::C0(_, _, h3843, _, _, _, _, _, _, _, _) => {
                h3843
            }
        }, match v2.clone() {
            Adt14::C0(_, _, _, h3844, _, _, _, _, _, _, _) => {
                h3844
            }
        }, match v2.clone() {
            Adt14::C0(_, _, _, _, h3845, _, _, _, _, _, _) => {
                h3845
            }
        }) {
            true
        } else {
            let a3847 = match v2.clone() {
                Adt14::C0(_, _, _, _, _, h3846, _, _, _, _, _) => {
                    h3846
                }
            };
            let a3848 = 4u64;
            if if if if if if if nat_eq(a3847, a3848) {
                let a3850 = match v3.clone() {
                    Adt14::C0(_, _, _, _, _, h3849, _, _, _, _, _) => {
                        h3849
                    }
                };
                let a3851 = 4u64;
                nat_eq(a3850, a3851)
            } else {
                false
            } {
                f221(v2.clone())
            } else {
                false
            } {
                f221(v3.clone())
            } else {
                false
            } {
                f221(v5.clone())
            } else {
                false
            } {
                let a3853 = f230(v5.clone())?;
                let a3854 = match v5.clone() {
                    Adt14::C0(_, _, _, _, _, _, _, _, _, h3852, _) => {
                        h3852
                    }
                };
                nat_le(a3853, a3854)
            } else {
                false
            } {
                if f222(v2.clone(), v5.clone())? {
                    true
                } else {
                    f223(v2.clone(), v5.clone())
                }
            } else {
                false
            } {
                if f222(v3.clone(), v5.clone())? {
                    true
                } else {
                    f223(v3, v5.clone())
                }
            } else {
                false
            }
        }
    } else {
        false
    } {
        let v6: u64 = if f219(match v2.clone() {
            Adt14::C0(_, h3855, _, _, _, _, _, _, _, _, _) => {
                h3855
            }
        }, match v2.clone() {
            Adt14::C0(_, _, h3856, _, _, _, _, _, _, _, _) => {
                h3856
            }
        }, match v2.clone() {
            Adt14::C0(_, _, _, h3857, _, _, _, _, _, _, _) => {
                h3857
            }
        }, match v2.clone() {
            Adt14::C0(_, _, _, _, h3858, _, _, _, _, _, _) => {
                h3858
            }
        }) {
            0u64
        } else {
            let a3864 = match v2.clone() {
                Adt14::C0(_, _, h3859, _, _, _, _, _, _, _, _) => {
                    h3859
                }
            };
            let a3865 = {
                let a3862 = match v2.clone() {
                    Adt14::C0(_, _, _, h3860, _, _, _, _, _, _, _) => {
                        h3860
                    }
                };
                let a3863 = match v2.clone() {
                    Adt14::C0(_, _, _, _, h3861, _, _, _, _, _, _) => {
                        h3861
                    }
                };
                nat_mul(a3862, a3863)?
            };
            nat_mul(a3864, a3865)?
        };
        let v7: u64 = {
            let a3866 = v6;
            let a3867 = 0u64;
            if if nat_eq(a3866, a3867) {
                true
            } else {
                f224(v5.clone())?
            } {
                0u64
            } else {
                let a3869 = match v5 {
                    Adt14::C0(_, _, _, _, _, _, _, _, _, h3868, _) => {
                        h3868
                    }
                };
                let a3870 = 4u64;
                let a3871 = 0u64;
                nat_quot(a3869, a3870, a3871)
            }
        };
        Ok(Adt15::C1(f232(match v2 {
            Adt14::C0(_, h3872, _, _, _, _, _, _, _, _, _) => {
                h3872
            }
        }, v6, v7)?, 4u64, 66u64, v6, f227(v7)?, v0))
    } else {
        Ok(Adt15::C0)
    }
}

pub fn f114(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64) -> bool {
    if if f233(v0, v3, v6) {
        f233(v1, v4, v6)
    } else {
        false
    } {
        f233(v2, v5, v6)
    } else {
        false
    }
}

pub fn f115(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64) -> R<bool> {
    if if if if if f234(v0, v1, v4, v2, v5, v3, v6)? {
        true
    } else {
        f234(v0, v1, v4, v3, v6, v2, v5)?
    } {
        true
    } else {
        f234(v0, v2, v5, v1, v4, v3, v6)?
    } {
        true
    } else {
        f234(v0, v2, v5, v3, v6, v1, v4)?
    } {
        true
    } else {
        f234(v0, v3, v6, v1, v4, v2, v5)?
    } {
        Ok(true)
    } else {
        f234(v0, v3, v6, v2, v5, v1, v4)
    }
}

pub fn f116(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let a3873 = v2;
    let a3874 = 0u64;
    if nat_eq(a3873, a3874) {
        Ok(0u64)
    } else {
        let a3877 = 2u64;
        let a3878 = {
            let a3875 = v0;
            let a3876 = v1;
            nat_mul(a3875, a3876)?
        };
        nat_mul(a3877, a3878)
    }
}

pub fn f117(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let a3879 = v2;
    let a3880 = f214(v0, v1)?;
    nat_mul(a3879, a3880)
}

pub fn f118(v0: u64) -> u64 {
    let a3881 = v0;
    let a3882 = 30u64;
    if nat_eq(a3881, a3882) {
        3u64
    } else {
        1u64
    }
}

pub fn f119(v0: u64) -> R<u64> {
    let a3885 = 5u64;
    let a3886 = {
        let a3883 = 4u64;
        let a3884 = v0;
        nat_mul(a3883, a3884)?
    };
    nat_add(a3885, a3886)
}

pub fn f120(v0: u64) -> bool {
    let m3887 = f235(v0);
    match m3887 {
        Adt18::C0 => {
            false
        }
        Adt18::C1(v1) => {
            let a3889 = 1u64;
            let a3890 = match v1 {
                Adt19::C0(_, h3888, _) => {
                    h3888
                }
            };
            nat_lt(a3889, a3890)
        }
    }
}

pub fn f121(v0: Adt1, v1: Adt2, v2: Adt2) -> R<Adt10> {
    let v3: u64 = match v0.clone() {
        Adt1::C0(h3891, _, _, _, _, _, _, _, _, _, _, _) => {
            h3891
        }
    };
    let v4: u64 = match v0 {
        Adt1::C0(_, h3892, _, _, _, _, _, _, _, _, _, _) => {
            h3892
        }
    };
    let v5: u64 = match v1.clone() {
        Adt2::C0(_, h3893, _, _, _, _, _, _, _, _, _) => {
            h3893
        }
    };
    let a3895 = match v1.clone() {
        Adt2::C0(h3894, _, _, _, _, _, _, _, _, _, _) => {
            h3894
        }
    };
    let a3896 = 0u64;
    if if if if if if if if if if if if if nat_eq(a3895, a3896) {
        let a3898 = match v2.clone() {
            Adt2::C0(h3897, _, _, _, _, _, _, _, _, _, _) => {
                h3897
            }
        };
        let a3899 = 0u64;
        nat_eq(a3898, a3899)
    } else {
        false
    } {
        let a3901 = match v1.clone() {
            Adt2::C0(_, _, _, _, _, h3900, _, _, _, _, _) => {
                h3900
            }
        };
        let a3902 = 4u64;
        nat_eq(a3901, a3902)
    } else {
        false
    } {
        let a3904 = match v2.clone() {
            Adt2::C0(_, h3903, _, _, _, _, _, _, _, _, _) => {
                h3903
            }
        };
        let a3905 = v3;
        nat_eq(a3904, a3905)
    } else {
        false
    } {
        let a3907 = match v2.clone() {
            Adt2::C0(_, _, h3906, _, _, _, _, _, _, _, _) => {
                h3906
            }
        };
        let a3908 = v5;
        nat_eq(a3907, a3908)
    } else {
        false
    } {
        let a3910 = match v2.clone() {
            Adt2::C0(_, _, _, h3909, _, _, _, _, _, _, _) => {
                h3909
            }
        };
        let a3911 = 1u64;
        nat_eq(a3910, a3911)
    } else {
        false
    } {
        let a3913 = match v2.clone() {
            Adt2::C0(_, _, _, _, h3912, _, _, _, _, _, _) => {
                h3912
            }
        };
        let a3914 = 1u64;
        nat_eq(a3913, a3914)
    } else {
        false
    } {
        let a3915 = v3;
        let a3916 = 2147483648u64;
        nat_lt(a3915, a3916)
    } else {
        false
    } {
        let a3917 = v4;
        let a3918 = 4294967296u64;
        nat_lt(a3917, a3918)
    } else {
        false
    } {
        f45(v2.clone())?
    } else {
        false
    } {
        let a3922 = {
            let a3919 = v5;
            let a3920 = 4u64;
            nat_mul(a3919, a3920)?
        };
        let a3923 = match v1.clone() {
            Adt2::C0(_, _, _, _, _, _, _, _, _, h3921, _) => {
                h3921
            }
        };
        nat_le(a3922, a3923)
    } else {
        false
    } {
        f26(v2.clone(), f25(v2.clone(), 1u64)?)?
    } else {
        false
    } {
        f27(v1.clone(), v2.clone())?
    } else {
        false
    } {
        Ok(Adt10::C4({
            let a3924 = v5;
            let a3925 = 0u64;
            if nat_eq(a3924, a3925) {
                0u64
            } else {
                f236(v3, v5)?
            }
        }, 4u64, f28(v1, 1u64), f28(v2, 1u64), Adt4::C44(v4)))
    } else {
        Ok(Adt10::C1)
    }
}

pub fn f122(v0: Adt1, v1: Adt2, v2: Adt2, v3: Adt2) -> R<Adt10> {
    if if if if if if if if f32(v2.clone(), v3.clone()) {
        let a3927 = match v3.clone() {
            Adt2::C0(h3926, _, _, _, _, _, _, _, _, _, _) => {
                h3926
            }
        };
        let a3928 = 0u64;
        nat_eq(a3927, a3928)
    } else {
        false
    } {
        f45(v3.clone())?
    } else {
        false
    } {
        f26(v3.clone(), f25(v3.clone(), 1u64)?)?
    } else {
        false
    } {
        f26(v1.clone(), f25(v1.clone(), 1u64)?)?
    } else {
        false
    } {
        f27(v2.clone(), v3.clone())?
    } else {
        false
    } {
        let a3930 = match v2.clone() {
            Adt2::C0(h3929, _, _, _, _, _, _, _, _, _, _) => {
                h3929
            }
        };
        let a3931 = 1u64;
        nat_le(a3930, a3931)
    } else {
        false
    } {
        let a3933 = match v0.clone() {
            Adt1::C0(h3932, _, _, _, _, _, _, _, _, _, _, _) => {
                h3932
            }
        };
        let a3934 = 0u64;
        if nat_eq(a3933, a3934) {
            let a3936 = match v2.clone() {
                Adt2::C0(h3935, _, _, _, _, _, _, _, _, _, _) => {
                    h3935
                }
            };
            let a3937 = 0u64;
            if if nat_eq(a3936, a3937) {
                f45(v2.clone())?
            } else {
                false
            } {
                f26(v2.clone(), f25(v2, 1u64)?)?
            } else {
                false
            }
        } else {
            true
        }
    } else {
        false
    } {
        f131(true, true, v0, v3, v1)
    } else {
        Ok(Adt10::C1)
    }
}

pub fn f123(v0: Adt1, v1: Adt2, v2: Adt2, v3: Adt2) -> R<Adt10> {
    let a3939 = match v1.clone() {
        Adt2::C0(h3938, _, _, _, _, _, _, _, _, _, _) => {
            h3938
        }
    };
    let a3940 = 0u64;
    if if if if if if if nat_eq(a3939, a3940) {
        let a3942 = match v3.clone() {
            Adt2::C0(h3941, _, _, _, _, _, _, _, _, _, _) => {
                h3941
            }
        };
        let a3943 = 0u64;
        nat_eq(a3942, a3943)
    } else {
        false
    } {
        let a3945 = match v2.clone() {
            Adt2::C0(h3944, _, _, _, _, _, _, _, _, _, _) => {
                h3944
            }
        };
        let a3946 = 1u64;
        nat_le(a3945, a3946)
    } else {
        false
    } {
        f45(v1.clone())?
    } else {
        false
    } {
        f26(v1.clone(), f25(v1.clone(), 1u64)?)?
    } else {
        false
    } {
        f26(v3.clone(), f25(v3.clone(), 1u64)?)?
    } else {
        false
    } {
        let a3948 = match v0.clone() {
            Adt1::C0(_, _, _, _, _, _, h3947, _, _, _, _, _) => {
                h3947
            }
        };
        let a3949 = 0u64;
        if nat_eq(a3948, a3949) {
            true
        } else {
            let a3951 = match v3.clone() {
                Adt2::C0(_, _, h3950, _, _, _, _, _, _, _, _) => {
                    h3950
                }
            };
            let a3952 = 1u64;
            if nat_le(a3951, a3952) {
                true
            } else {
                let a3957 = match v3.clone() {
                    Adt2::C0(_, _, _, _, _, _, h3953, _, _, _, _) => {
                        h3953
                    }
                };
                let a3958 = {
                    let a3955 = 4u64;
                    let a3956 = match v3.clone() {
                        Adt2::C0(_, h3954, _, _, _, _, _, _, _, _, _) => {
                            h3954
                        }
                    };
                    nat_mul(a3955, a3956)?
                };
                nat_eq(a3957, a3958)
            }
        }
    } else {
        false
    } {
        f132(true, v0, v2, Some(v3), v1)
    } else {
        Ok(Adt10::C1)
    }
}

pub fn f124(v0: Adt2, v1: Adt2, v2: Adt2) -> R<Adt10> {
    let a3960 = match v0.clone() {
        Adt2::C0(h3959, _, _, _, _, _, _, _, _, _, _) => {
            h3959
        }
    };
    let a3961 = 0u64;
    if if if if if if if if if if if if nat_eq(a3960, a3961) {
        f45(v0.clone())?
    } else {
        false
    } {
        f26(v0.clone(), f25(v0.clone(), 1u64)?)?
    } else {
        false
    } {
        let a3963 = match v1.clone() {
            Adt2::C0(h3962, _, _, _, _, _, _, _, _, _, _) => {
                h3962
            }
        };
        let a3964 = 0u64;
        if if nat_eq(a3963, a3964) {
            f45(v1.clone())?
        } else {
            false
        } {
            f26(v1.clone(), f25(v1.clone(), 1u64)?)?
        } else {
            false
        }
    } else {
        false
    } {
        let a3966 = match v2.clone() {
            Adt2::C0(h3965, _, _, _, _, _, _, _, _, _, _) => {
                h3965
            }
        };
        let a3967 = 0u64;
        if if nat_eq(a3966, a3967) {
            f45(v2.clone())?
        } else {
            false
        } {
            f26(v2.clone(), f25(v2.clone(), 1u64)?)?
        } else {
            false
        }
    } else {
        false
    } {
        let a3970 = match v0.clone() {
            Adt2::C0(_, h3968, _, _, _, _, _, _, _, _, _) => {
                h3968
            }
        };
        let a3971 = match v0.clone() {
            Adt2::C0(_, _, h3969, _, _, _, _, _, _, _, _) => {
                h3969
            }
        };
        nat_eq(a3970, a3971)
    } else {
        false
    } {
        let a3974 = match v0.clone() {
            Adt2::C0(_, _, h3972, _, _, _, _, _, _, _, _) => {
                h3972
            }
        };
        let a3975 = match v1.clone() {
            Adt2::C0(_, _, h3973, _, _, _, _, _, _, _, _) => {
                h3973
            }
        };
        nat_eq(a3974, a3975)
    } else {
        false
    } {
        let a3978 = match v0.clone() {
            Adt2::C0(_, _, _, h3976, _, _, _, _, _, _, _) => {
                h3976
            }
        };
        let a3979 = match v1.clone() {
            Adt2::C0(_, _, _, h3977, _, _, _, _, _, _, _) => {
                h3977
            }
        };
        nat_eq(a3978, a3979)
    } else {
        false
    } {
        let a3982 = match v0.clone() {
            Adt2::C0(_, _, _, _, h3980, _, _, _, _, _, _) => {
                h3980
            }
        };
        let a3983 = match v1.clone() {
            Adt2::C0(_, _, _, _, h3981, _, _, _, _, _, _) => {
                h3981
            }
        };
        nat_eq(a3982, a3983)
    } else {
        false
    } {
        f32(v1.clone(), v2.clone())
    } else {
        false
    } {
        f27(v0.clone(), v2.clone())?
    } else {
        false
    } {
        f27(v1.clone(), v2.clone())?
    } else {
        false
    } {
        Ok(Adt10::C4({
            let a3992 = {
                let a3989 = {
                    let a3986 = match v1.clone() {
                        Adt2::C0(_, _, _, h3984, _, _, _, _, _, _, _) => {
                            h3984
                        }
                    };
                    let a3987 = match v1.clone() {
                        Adt2::C0(_, _, _, _, h3985, _, _, _, _, _, _) => {
                            h3985
                        }
                    };
                    nat_mul(a3986, a3987)?
                };
                let a3990 = match v1 {
                    Adt2::C0(_, h3988, _, _, _, _, _, _, _, _, _) => {
                        h3988
                    }
                };
                nat_mul(a3989, a3990)?
            };
            let a3993 = f195(match v0.clone() {
                Adt2::C0(_, h3991, _, _, _, _, _, _, _, _, _) => {
                    h3991
                }
            })?;
            nat_mul(a3992, a3993)?
        }, 4u64, f28(v0, 1u64), f28(v2, 1u64), Adt4::C41))
    } else {
        Ok(Adt10::C1)
    }
}

pub fn f125(v0: Adt1, v1: Adt2, v2: Adt2, v3: Adt2) -> R<Adt10> {
    let v4: bool = {
        let a3995 = 0u64;
        let a3996 = match v0.clone() {
            Adt1::C0(_, h3994, _, _, _, _, _, _, _, _, _, _) => {
                h3994
            }
        };
        nat_lt(a3995, a3996)
    };
    let a3998 = match v1.clone() {
        Adt2::C0(h3997, _, _, _, _, _, _, _, _, _, _) => {
            h3997
        }
    };
    let a3999 = 0u64;
    if if if if if if if if if if if if if if if if nat_eq(a3998, a3999) {
        let a4001 = match v2.clone() {
            Adt2::C0(h4000, _, _, _, _, _, _, _, _, _, _) => {
                h4000
            }
        };
        let a4002 = 0u64;
        nat_eq(a4001, a4002)
    } else {
        false
    } {
        let a4004 = match v3.clone() {
            Adt2::C0(h4003, _, _, _, _, _, _, _, _, _, _) => {
                h4003
            }
        };
        let a4005 = 0u64;
        nat_eq(a4004, a4005)
    } else {
        false
    } {
        let a4007 = match v1.clone() {
            Adt2::C0(_, _, _, _, h4006, _, _, _, _, _, _) => {
                h4006
            }
        };
        let a4008 = 1u64;
        nat_eq(a4007, a4008)
    } else {
        false
    } {
        let a4010 = match v2.clone() {
            Adt2::C0(_, _, _, _, h4009, _, _, _, _, _, _) => {
                h4009
            }
        };
        let a4011 = 1u64;
        nat_eq(a4010, a4011)
    } else {
        false
    } {
        let a4013 = match v3.clone() {
            Adt2::C0(_, _, _, _, h4012, _, _, _, _, _, _) => {
                h4012
            }
        };
        let a4014 = 1u64;
        nat_eq(a4013, a4014)
    } else {
        false
    } {
        let a4016 = 0u64;
        let a4017 = match v1.clone() {
            Adt2::C0(_, _, h4015, _, _, _, _, _, _, _, _) => {
                h4015
            }
        };
        nat_lt(a4016, a4017)
    } else {
        false
    } {
        let a4019 = match v3.clone() {
            Adt2::C0(_, _, _, h4018, _, _, _, _, _, _, _) => {
                h4018
            }
        };
        let a4020 = 1u64;
        nat_eq(a4019, a4020)
    } else {
        false
    } {
        let a4023 = match v3.clone() {
            Adt2::C0(_, h4021, _, _, _, _, _, _, _, _, _) => {
                h4021
            }
        };
        let a4024 = match v1.clone() {
            Adt2::C0(_, h4022, _, _, _, _, _, _, _, _, _) => {
                h4022
            }
        };
        nat_eq(a4023, a4024)
    } else {
        false
    } {
        let a4027 = match v3.clone() {
            Adt2::C0(_, _, h4025, _, _, _, _, _, _, _, _) => {
                h4025
            }
        };
        let a4028 = match v1.clone() {
            Adt2::C0(_, _, _, h4026, _, _, _, _, _, _, _) => {
                h4026
            }
        };
        nat_eq(a4027, a4028)
    } else {
        false
    } {
        if v4 {
            f32(v1.clone(), v2.clone())
        } else {
            let a4031 = match v2.clone() {
                Adt2::C0(_, h4029, _, _, _, _, _, _, _, _, _) => {
                    h4029
                }
            };
            let a4032 = match v1.clone() {
                Adt2::C0(_, _, h4030, _, _, _, _, _, _, _, _) => {
                    h4030
                }
            };
            if if nat_eq(a4031, a4032) {
                let a4035 = match v2.clone() {
                    Adt2::C0(_, _, h4033, _, _, _, _, _, _, _, _) => {
                        h4033
                    }
                };
                let a4036 = match v1.clone() {
                    Adt2::C0(_, _, _, h4034, _, _, _, _, _, _, _) => {
                        h4034
                    }
                };
                nat_eq(a4035, a4036)
            } else {
                false
            } {
                let a4038 = match v2.clone() {
                    Adt2::C0(_, _, _, h4037, _, _, _, _, _, _, _) => {
                        h4037
                    }
                };
                let a4039 = 1u64;
                nat_eq(a4038, a4039)
            } else {
                false
            }
        }
    } else {
        false
    } {
        f26(v1.clone(), f25(v1.clone(), 1u64)?)?
    } else {
        false
    } {
        f26(v2.clone(), f25(v2.clone(), 1u64)?)?
    } else {
        false
    } {
        f38(v3.clone())?
    } else {
        false
    } {
        f27(v1.clone(), v3.clone())?
    } else {
        false
    } {
        f27(v2.clone(), v3.clone())?
    } else {
        false
    } {
        Ok(Adt10::C4({
            let a4041 = f25(v3.clone(), 1u64)?;
            let a4042 = f185(match v1.clone() {
                Adt2::C0(_, _, h4040, _, _, _, _, _, _, _, _) => {
                    h4040
                }
            }, v4)?;
            nat_mul(a4041, a4042)?
        }, 4u64, f28(v1, 1u64), f28(v3, 1u64), Adt4::C36(f28(v2, 1u64), v4, match v0 {
            Adt1::C0(h4043, _, _, _, _, _, _, _, _, _, _, _) => {
                h4043
            }
        })))
    } else {
        Ok(Adt10::C1)
    }
}

pub fn f126(v0: Adt1, v1: Adt2, v2: Adt2, v3: Adt2) -> R<Adt10> {
    let v4: Adt6 = Adt6::C0(match v1.clone() {
        Adt2::C0(_, h4044, _, _, _, _, _, _, _, _, _) => {
            h4044
        }
    }, match v0.clone() {
        Adt1::C0(h4045, _, _, _, _, _, _, _, _, _, _, _) => {
            h4045
        }
    }, match v0.clone() {
        Adt1::C0(_, _, h4046, _, _, _, _, _, _, _, _, _) => {
            h4046
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, _, h4047, _, _, _, _, _, _, _) => {
            h4047
        }
    });
    let v5: Adt6 = Adt6::C0(match v1.clone() {
        Adt2::C0(_, _, h4048, _, _, _, _, _, _, _, _) => {
            h4048
        }
    }, match v0.clone() {
        Adt1::C0(_, h4049, _, _, _, _, _, _, _, _, _, _) => {
            h4049
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, h4050, _, _, _, _, _, _, _, _) => {
            h4050
        }
    }, match v0 {
        Adt1::C0(_, _, _, _, _, h4051, _, _, _, _, _, _) => {
            h4051
        }
    });
    let v6: u64 = match v1.clone() {
        Adt2::C0(h4052, _, _, _, _, _, _, _, _, _, _) => {
            h4052
        }
    };
    let v7: bool = {
        let a4055 = match v2.clone() {
            Adt2::C0(_, _, _, _, _, _, _, h4053, _, _, _) => {
                h4053
            }
        };
        let a4056 = match v2.clone() {
            Adt2::C0(_, _, _, _, _, h4054, _, _, _, _, _) => {
                h4054
            }
        };
        nat_lt(a4055, a4056)
    };
    let a4057 = v6;
    let a4058 = 0u64;
    if if if if if if if if if if if if if if if nat_eq(a4057, a4058) {
        true
    } else {
        let a4059 = v6;
        let a4060 = 1u64;
        nat_eq(a4059, a4060)
    } {
        let a4062 = match v2.clone() {
            Adt2::C0(h4061, _, _, _, _, _, _, _, _, _, _) => {
                h4061
            }
        };
        let a4063 = 0u64;
        nat_eq(a4062, a4063)
    } else {
        false
    } {
        let a4065 = match v3.clone() {
            Adt2::C0(h4064, _, _, _, _, _, _, _, _, _, _) => {
                h4064
            }
        };
        let a4066 = 0u64;
        nat_eq(a4065, a4066)
    } else {
        false
    } {
        let a4068 = match v1.clone() {
            Adt2::C0(_, _, _, h4067, _, _, _, _, _, _, _) => {
                h4067
            }
        };
        let a4069 = 1u64;
        nat_eq(a4068, a4069)
    } else {
        false
    } {
        let a4072 = match v1.clone() {
            Adt2::C0(_, _, _, _, h4070, _, _, _, _, _, _) => {
                h4070
            }
        };
        let a4073 = match v2.clone() {
            Adt2::C0(_, _, _, h4071, _, _, _, _, _, _, _) => {
                h4071
            }
        };
        nat_eq(a4072, a4073)
    } else {
        false
    } {
        let a4076 = match v3.clone() {
            Adt2::C0(_, _, _, h4074, _, _, _, _, _, _, _) => {
                h4074
            }
        };
        let a4077 = match v2.clone() {
            Adt2::C0(_, _, _, h4075, _, _, _, _, _, _, _) => {
                h4075
            }
        };
        nat_eq(a4076, a4077)
    } else {
        false
    } {
        let a4080 = match v3.clone() {
            Adt2::C0(_, _, _, _, h4078, _, _, _, _, _, _) => {
                h4078
            }
        };
        let a4081 = match v2.clone() {
            Adt2::C0(_, _, _, _, h4079, _, _, _, _, _, _) => {
                h4079
            }
        };
        nat_eq(a4080, a4081)
    } else {
        false
    } {
        let a4083 = 0u64;
        let a4084 = match v4.clone() {
            Adt6::C0(h4082, _, _, _) => {
                h4082
            }
        };
        if if if if if nat_lt(a4083, a4084) {
            let a4086 = 0u64;
            let a4087 = match v4.clone() {
                Adt6::C0(_, h4085, _, _) => {
                    h4085
                }
            };
            nat_lt(a4086, a4087)
        } else {
            false
        } {
            let a4089 = match v4.clone() {
                Adt6::C0(_, h4088, _, _) => {
                    h4088
                }
            };
            let a4090 = 2147483648u64;
            nat_lt(a4089, a4090)
        } else {
            false
        } {
            let a4092 = 0u64;
            let a4093 = match v4.clone() {
                Adt6::C0(_, _, _, h4091) => {
                    h4091
                }
            };
            nat_lt(a4092, a4093)
        } else {
            false
        } {
            let a4095 = match v4.clone() {
                Adt6::C0(_, _, _, h4094) => {
                    h4094
                }
            };
            let a4096 = 2147483648u64;
            nat_lt(a4095, a4096)
        } else {
            false
        } {
            let a4099 = match v3.clone() {
                Adt2::C0(_, h4097, _, _, _, _, _, _, _, _, _) => {
                    h4097
                }
            };
            let a4100 = f108(v4.clone(), match v2.clone() {
                Adt2::C0(_, h4098, _, _, _, _, _, _, _, _, _) => {
                    h4098
                }
            })?;
            nat_eq(a4099, a4100)
        } else {
            false
        }
    } else {
        false
    } {
        let a4102 = 0u64;
        let a4103 = match v5.clone() {
            Adt6::C0(h4101, _, _, _) => {
                h4101
            }
        };
        if if if if if nat_lt(a4102, a4103) {
            let a4105 = 0u64;
            let a4106 = match v5.clone() {
                Adt6::C0(_, h4104, _, _) => {
                    h4104
                }
            };
            nat_lt(a4105, a4106)
        } else {
            false
        } {
            let a4108 = match v5.clone() {
                Adt6::C0(_, h4107, _, _) => {
                    h4107
                }
            };
            let a4109 = 2147483648u64;
            nat_lt(a4108, a4109)
        } else {
            false
        } {
            let a4111 = 0u64;
            let a4112 = match v5.clone() {
                Adt6::C0(_, _, _, h4110) => {
                    h4110
                }
            };
            nat_lt(a4111, a4112)
        } else {
            false
        } {
            let a4114 = match v5.clone() {
                Adt6::C0(_, _, _, h4113) => {
                    h4113
                }
            };
            let a4115 = 2147483648u64;
            nat_lt(a4114, a4115)
        } else {
            false
        } {
            let a4118 = match v3.clone() {
                Adt2::C0(_, _, h4116, _, _, _, _, _, _, _, _) => {
                    h4116
                }
            };
            let a4119 = f108(v5.clone(), match v2.clone() {
                Adt2::C0(_, _, h4117, _, _, _, _, _, _, _, _) => {
                    h4117
                }
            })?;
            nat_eq(a4118, a4119)
        } else {
            false
        }
    } else {
        false
    } {
        f237(v2.clone(), 4u64, v7, false)?
    } else {
        false
    } {
        f237(v3.clone(), 4u64, v7, false)?
    } else {
        false
    } {
        f237(v1.clone(), {
            let a4120 = v6;
            let a4121 = 0u64;
            if nat_eq(a4120, a4121) {
                4u64
            } else {
                2u64
            }
        }, v7, true)?
    } else {
        false
    } {
        f27(v1.clone(), v3.clone())?
    } else {
        false
    } {
        f27(v2.clone(), v3.clone())?
    } else {
        false
    } {
        Ok(Adt10::C4({
            let a4130 = {
                let a4128 = {
                    let a4126 = 3u64;
                    let a4127 = {
                        let a4124 = match v4.clone() {
                            Adt6::C0(h4122, _, _, _) => {
                                h4122
                            }
                        };
                        let a4125 = match v5.clone() {
                            Adt6::C0(h4123, _, _, _) => {
                                h4123
                            }
                        };
                        nat_mul(a4124, a4125)?
                    };
                    nat_mul(a4126, a4127)?
                };
                let a4129 = 2u64;
                nat_add(a4128, a4129)?
            };
            let a4131 = f25(v3.clone(), 1u64)?;
            nat_mul(a4130, a4131)?
        }, 4u64, f28(v1, 1u64), f28(v3, 1u64), Adt4::C34(f28(v2, 1u64), v4, v5, v6)))
    } else {
        Ok(Adt10::C1)
    }
}

pub fn f127(v0: Adt2, v1: Adt2, v2: Adt2) -> R<Adt10> {
    if if if if if if if f32(v0.clone(), v1.clone()) {
        f32(v0.clone(), v2.clone())
    } else {
        false
    } {
        f38(v0.clone())?
    } else {
        false
    } {
        f38(v1.clone())?
    } else {
        false
    } {
        f38(v2.clone())?
    } else {
        false
    } {
        if f27(v0.clone(), v2.clone())? {
            true
        } else {
            f54(v0.clone(), v2.clone())
        }
    } else {
        false
    } {
        if f27(v1.clone(), v2.clone())? {
            true
        } else {
            f54(v1.clone(), v2.clone())
        }
    } else {
        false
    } {
        Ok(Adt10::C4({
            let a4132 = 6u64;
            let a4133 = f25(v0.clone(), 1u64)?;
            nat_mul(a4132, a4133)?
        }, 2u64, f28(v0, 1u64), f28(v2, 1u64), Adt4::C33(f28(v1, 1u64))))
    } else {
        Ok(Adt10::C1)
    }
}

pub fn f128(v0: Adt1, v1: Adt2, v2: Option<Adt2>, v3: Adt2) -> R<Adt10> {
    let m4134 = v2;
    match m4134 {
        None => {
            Ok(Adt10::C1)
        }
        Some(v4) => {
            let v5: u64 = match v1.clone() {
                Adt2::C0(_, h4135, _, _, _, _, _, _, _, _, _) => {
                    h4135
                }
            };
            let v6: u64 = {
                let a4141 = match v1.clone() {
                    Adt2::C0(_, _, h4136, _, _, _, _, _, _, _, _) => {
                        h4136
                    }
                };
                let a4142 = {
                    let a4139 = match v1.clone() {
                        Adt2::C0(_, _, _, h4137, _, _, _, _, _, _, _) => {
                            h4137
                        }
                    };
                    let a4140 = match v1.clone() {
                        Adt2::C0(_, _, _, _, h4138, _, _, _, _, _, _) => {
                            h4138
                        }
                    };
                    nat_mul(a4139, a4140)?
                };
                nat_mul(a4141, a4142)?
            };
            if if if if if if if if if if f32(v1.clone(), v4.clone()) {
                f32(v1.clone(), v3.clone())
            } else {
                false
            } {
                f238(v1.clone(), v6)?
            } else {
                false
            } {
                f238(v4.clone(), v6)?
            } else {
                false
            } {
                f238(v3.clone(), v6)?
            } else {
                false
            } {
                if f27(v1.clone(), v3.clone())? {
                    true
                } else {
                    f54(v1.clone(), v3.clone())
                }
            } else {
                false
            } {
                if f27(v4.clone(), v3.clone())? {
                    true
                } else {
                    f54(v4, v3.clone())
                }
            } else {
                false
            } {
                let a4143 = v5;
                let a4144 = 2147483647u64;
                nat_le(a4143, a4144)
            } else {
                false
            } {
                let a4145 = v6;
                let a4146 = 2147483647u64;
                nat_le(a4145, a4146)
            } else {
                false
            } {
                let a4148 = match v0.clone() {
                    Adt1::C0(h4147, _, _, _, _, _, _, _, _, _, _, _) => {
                        h4147
                    }
                };
                let a4149 = 4294967295u64;
                if nat_le(a4148, a4149) {
                    let a4151 = match v0.clone() {
                        Adt1::C0(_, h4150, _, _, _, _, _, _, _, _, _, _) => {
                            h4150
                        }
                    };
                    let a4152 = 4294967295u64;
                    nat_le(a4151, a4152)
                } else {
                    false
                }
            } else {
                false
            } {
                Ok(Adt10::C4({
                    let a4153 = v5;
                    let a4154 = 0u64;
                    if nat_eq(a4153, a4154) {
                        0u64
                    } else {
                        let a4155 = v6;
                        let a4156 = f184(v5)?;
                        nat_mul(a4155, a4156)?
                    }
                }, 4u64, f28(v1, 1u64), f28(v3, 1u64), Adt4::C32(match v0 {
                    Adt1::C0(h4157, _, _, _, _, _, _, _, _, _, _, _) => {
                        h4157
                    }
                })))
            } else {
                Ok(Adt10::C1)
            }
        }
    }
}

pub fn f129(v0: Adt1, v1: Adt2, v2: Adt2) -> R<Adt10> {
    let v3: u64 = match v0.clone() {
        Adt1::C0(h4158, _, _, _, _, _, _, _, _, _, _, _) => {
            h4158
        }
    };
    let v4: u64 = match v0.clone() {
        Adt1::C0(_, h4159, _, _, _, _, _, _, _, _, _, _) => {
            h4159
        }
    };
    let v5: u64 = match v0 {
        Adt1::C0(_, _, h4160, _, _, _, _, _, _, _, _, _) => {
            h4160
        }
    };
    let v6: u64 = {
        let a4162 = match v1.clone() {
            Adt2::C0(_, h4161, _, _, _, _, _, _, _, _, _) => {
                h4161
            }
        };
        let a4163 = v4;
        let a4164 = 0u64;
        nat_quot(a4162, a4163, a4164)
    };
    let v7: u64 = {
        let a4166 = match v1.clone() {
            Adt2::C0(h4165, _, _, _, _, _, _, _, _, _, _) => {
                h4165
            }
        };
        let a4167 = 0u64;
        if nat_eq(a4166, a4167) {
            4u64
        } else {
            2u64
        }
    };
    if if if if if if if if if if if if if if if f198(match v1.clone() {
        Adt2::C0(h4168, _, _, _, _, _, _, _, _, _, _) => {
            h4168
        }
    }) {
        let a4171 = match v1.clone() {
            Adt2::C0(h4169, _, _, _, _, _, _, _, _, _, _) => {
                h4169
            }
        };
        let a4172 = match v2.clone() {
            Adt2::C0(h4170, _, _, _, _, _, _, _, _, _, _) => {
                h4170
            }
        };
        nat_eq(a4171, a4172)
    } else {
        false
    } {
        let a4173 = 0u64;
        let a4174 = v3;
        nat_lt(a4173, a4174)
    } else {
        false
    } {
        let a4175 = v3;
        let a4176 = 2147483648u64;
        nat_lt(a4175, a4176)
    } else {
        false
    } {
        let a4177 = 0u64;
        let a4178 = v4;
        nat_lt(a4177, a4178)
    } else {
        false
    } {
        let a4179 = v4;
        let a4180 = 2147483648u64;
        nat_lt(a4179, a4180)
    } else {
        false
    } {
        let a4181 = v5;
        let a4182 = 2147483648u64;
        nat_lt(a4181, a4182)
    } else {
        false
    } {
        let a4183 = 0u64;
        let a4184 = v6;
        nat_lt(a4183, a4184)
    } else {
        false
    } {
        let a4186 = 0u64;
        let a4187 = match v2.clone() {
            Adt2::C0(_, h4185, _, _, _, _, _, _, _, _, _) => {
                h4185
            }
        };
        nat_lt(a4186, a4187)
    } else {
        false
    } {
        let a4191 = match v1.clone() {
            Adt2::C0(_, h4188, _, _, _, _, _, _, _, _, _) => {
                h4188
            }
        };
        let a4192 = {
            let a4189 = v6;
            let a4190 = v4;
            nat_mul(a4189, a4190)?
        };
        nat_eq(a4191, a4192)
    } else {
        false
    } {
        let a4194 = match v2.clone() {
            Adt2::C0(_, _, h4193, _, _, _, _, _, _, _, _) => {
                h4193
            }
        };
        let a4195 = v4;
        nat_eq(a4194, a4195)
    } else {
        false
    } {
        let a4208 = {
            let a4201 = {
                let a4199 = match v2.clone() {
                    Adt2::C0(_, h4196, _, _, _, _, _, _, _, _, _) => {
                        h4196
                    }
                };
                let a4200 = {
                    let a4197 = v5;
                    let a4198 = 2u64;
                    nat_mul(a4197, a4198)?
                };
                nat_add(a4199, a4200)?
            };
            let a4202 = v3;
            nat_add(a4201, a4202)?
        };
        let a4209 = {
            let a4206 = {
                let a4204 = match v1.clone() {
                    Adt2::C0(_, _, h4203, _, _, _, _, _, _, _, _) => {
                        h4203
                    }
                };
                let a4205 = v3;
                nat_mul(a4204, a4205)?
            };
            let a4207 = v6;
            nat_add(a4206, a4207)?
        };
        nat_eq(a4208, a4209)
    } else {
        false
    } {
        f239(v1.clone(), v7)?
    } else {
        false
    } {
        f239(v2.clone(), v7)?
    } else {
        false
    } {
        f27(v1.clone(), v2.clone())?
    } else {
        false
    } {
        let v8: Adt6 = Adt6::C0(v6, v3, v5, 1u64);
        Ok(Adt10::C4({
            let a4210 = f25(v2.clone(), 1u64)?;
            let a4211 = f211(v8.clone())?;
            nat_mul(a4210, a4211)?
        }, v7, f28(v1.clone(), 1u64), f28(v2, 1u64), Adt4::C30(v8, match v1 {
            Adt2::C0(h4212, _, _, _, _, _, _, _, _, _, _) => {
                h4212
            }
        })))
    } else {
        Ok(Adt10::C1)
    }
}

pub fn f130(v0: Adt2, v1: Option<Adt2>, v2: Adt2) -> R<Adt10> {
    let m4213 = v1;
    match m4213 {
        None => {
            Ok(Adt10::C1)
        }
        Some(v3) => {
            let a4215 = match v0.clone() {
                Adt2::C0(h4214, _, _, _, _, _, _, _, _, _, _) => {
                    h4214
                }
            };
            let a4216 = 0u64;
            if if if if if if if if if if if if if if if if if if if nat_eq(a4215, a4216) {
                let a4218 = match v3.clone() {
                    Adt2::C0(h4217, _, _, _, _, _, _, _, _, _, _) => {
                        h4217
                    }
                };
                let a4219 = 0u64;
                nat_eq(a4218, a4219)
            } else {
                false
            } {
                let a4221 = match v2.clone() {
                    Adt2::C0(h4220, _, _, _, _, _, _, _, _, _, _) => {
                        h4220
                    }
                };
                let a4222 = 0u64;
                nat_eq(a4221, a4222)
            } else {
                false
            } {
                let a4224 = match v0.clone() {
                    Adt2::C0(_, _, _, _, h4223, _, _, _, _, _, _) => {
                        h4223
                    }
                };
                let a4225 = 1u64;
                nat_eq(a4224, a4225)
            } else {
                false
            } {
                let a4227 = match v3.clone() {
                    Adt2::C0(_, _, _, h4226, _, _, _, _, _, _, _) => {
                        h4226
                    }
                };
                let a4228 = 1u64;
                nat_eq(a4227, a4228)
            } else {
                false
            } {
                let a4230 = match v3.clone() {
                    Adt2::C0(_, _, _, _, h4229, _, _, _, _, _, _) => {
                        h4229
                    }
                };
                let a4231 = 1u64;
                nat_eq(a4230, a4231)
            } else {
                false
            } {
                let a4234 = match v0.clone() {
                    Adt2::C0(_, _, h4232, _, _, _, _, _, _, _, _) => {
                        h4232
                    }
                };
                let a4235 = match v3.clone() {
                    Adt2::C0(_, _, h4233, _, _, _, _, _, _, _, _) => {
                        h4233
                    }
                };
                nat_eq(a4234, a4235)
            } else {
                false
            } {
                let a4240 = match v3.clone() {
                    Adt2::C0(_, h4236, _, _, _, _, _, _, _, _, _) => {
                        h4236
                    }
                };
                let a4241 = {
                    let a4238 = match v0.clone() {
                        Adt2::C0(_, h4237, _, _, _, _, _, _, _, _, _) => {
                            h4237
                        }
                    };
                    let a4239 = 1u64;
                    nat_add(a4238, a4239)?
                };
                nat_le(a4240, a4241)
            } else {
                false
            } {
                let a4244 = match v2.clone() {
                    Adt2::C0(_, h4242, _, _, _, _, _, _, _, _, _) => {
                        h4242
                    }
                };
                let a4245 = match v0.clone() {
                    Adt2::C0(_, _, h4243, _, _, _, _, _, _, _, _) => {
                        h4243
                    }
                };
                nat_eq(a4244, a4245)
            } else {
                false
            } {
                let a4253 = match v2.clone() {
                    Adt2::C0(_, _, h4246, _, _, _, _, _, _, _, _) => {
                        h4246
                    }
                };
                let a4254 = {
                    let a4251 = {
                        let a4248 = match v0.clone() {
                            Adt2::C0(_, h4247, _, _, _, _, _, _, _, _, _) => {
                                h4247
                            }
                        };
                        let a4249 = 1u64;
                        nat_add(a4248, a4249)?
                    };
                    let a4252 = match v3.clone() {
                        Adt2::C0(_, h4250, _, _, _, _, _, _, _, _, _) => {
                            h4250
                        }
                    };
                    nat_sub(a4251, a4252)
                };
                nat_eq(a4253, a4254)
            } else {
                false
            } {
                let a4257 = match v2.clone() {
                    Adt2::C0(_, _, _, h4255, _, _, _, _, _, _, _) => {
                        h4255
                    }
                };
                let a4258 = match v0.clone() {
                    Adt2::C0(_, _, _, h4256, _, _, _, _, _, _, _) => {
                        h4256
                    }
                };
                nat_eq(a4257, a4258)
            } else {
                false
            } {
                let a4260 = match v2.clone() {
                    Adt2::C0(_, _, _, _, h4259, _, _, _, _, _, _) => {
                        h4259
                    }
                };
                let a4261 = 1u64;
                nat_eq(a4260, a4261)
            } else {
                false
            } {
                f45(v2.clone())?
            } else {
                false
            } {
                let a4263 = match v0.clone() {
                    Adt2::C0(_, _, _, _, _, h4262, _, _, _, _, _) => {
                        h4262
                    }
                };
                let a4264 = 4u64;
                nat_eq(a4263, a4264)
            } else {
                false
            } {
                let a4266 = match v3.clone() {
                    Adt2::C0(_, _, _, _, _, h4265, _, _, _, _, _) => {
                        h4265
                    }
                };
                let a4267 = 4u64;
                nat_eq(a4266, a4267)
            } else {
                false
            } {
                let a4272 = match v0.clone() {
                    Adt2::C0(_, _, _, _, _, _, h4268, _, _, _, _) => {
                        h4268
                    }
                };
                let a4273 = {
                    let a4270 = match v0.clone() {
                        Adt2::C0(_, h4269, _, _, _, _, _, _, _, _, _) => {
                            h4269
                        }
                    };
                    let a4271 = 4u64;
                    nat_mul(a4270, a4271)?
                };
                nat_eq(a4272, a4273)
            } else {
                false
            } {
                let a4278 = match v3.clone() {
                    Adt2::C0(_, _, _, _, _, _, h4274, _, _, _, _) => {
                        h4274
                    }
                };
                let a4279 = {
                    let a4276 = match v3.clone() {
                        Adt2::C0(_, h4275, _, _, _, _, _, _, _, _, _) => {
                            h4275
                        }
                    };
                    let a4277 = 4u64;
                    nat_mul(a4276, a4277)?
                };
                nat_eq(a4278, a4279)
            } else {
                false
            } {
                f27(v0.clone(), v2.clone())?
            } else {
                false
            } {
                f27(v3.clone(), v2.clone())?
            } else {
                false
            } {
                Ok(Adt10::C4({
                    let a4281 = f25(v2.clone(), 1u64)?;
                    let a4282 = f210(match v3.clone() {
                        Adt2::C0(_, h4280, _, _, _, _, _, _, _, _, _) => {
                            h4280
                        }
                    })?;
                    nat_mul(a4281, a4282)?
                }, 4u64, f28(v0, 1u64), f28(v2, 1u64), Adt4::C29(f28(v3, 1u64))))
            } else {
                Ok(Adt10::C1)
            }
        }
    }
}

pub fn f131(v0: bool, v1: bool, v2: Adt1, v3: Adt2, v4: Adt2) -> R<Adt10> {
    let v5: Adt6 = Adt6::C0(match v2.clone() {
        Adt1::C0(_, h4283, _, _, _, _, _, _, _, _, _, _) => {
            h4283
        }
    }, if v1 {
        match v2.clone() {
            Adt1::C0(_, _, _, h4284, _, _, _, _, _, _, _, _) => {
                h4284
            }
        }
    } else {
        match v2.clone() {
            Adt1::C0(_, _, h4285, _, _, _, _, _, _, _, _, _) => {
                h4285
            }
        }
    }, if v1 {
        match v2.clone() {
            Adt1::C0(_, _, _, _, _, h4286, _, _, _, _, _, _) => {
                h4286
            }
        }
    } else {
        match v2.clone() {
            Adt1::C0(_, _, _, h4287, _, _, _, _, _, _, _, _) => {
                h4287
            }
        }
    }, 1u64);
    let v6: Adt6 = if v1 {
        Adt6::C0(match v2.clone() {
            Adt1::C0(_, _, h4288, _, _, _, _, _, _, _, _, _) => {
                h4288
            }
        }, match v2.clone() {
            Adt1::C0(_, _, _, _, h4289, _, _, _, _, _, _, _) => {
                h4289
            }
        }, match v2.clone() {
            Adt1::C0(_, _, _, _, _, _, h4290, _, _, _, _, _) => {
                h4290
            }
        }, 1u64)
    } else {
        Adt6::C0(1u64, 1u64, 0u64, 1u64)
    };
    let a4292 = match v2.clone() {
        Adt1::C0(h4291, _, _, _, _, _, _, _, _, _, _, _) => {
            h4291
        }
    };
    let a4293 = 1u64;
    if if if if if if if if if if if nat_le(a4292, a4293) {
        let a4295 = match v3.clone() {
            Adt2::C0(h4294, _, _, _, _, _, _, _, _, _, _) => {
                h4294
            }
        };
        let a4296 = 1u64;
        nat_le(a4295, a4296)
    } else {
        false
    } {
        let a4298 = match v4.clone() {
            Adt2::C0(h4297, _, _, _, _, _, _, _, _, _, _) => {
                h4297
            }
        };
        let a4299 = 0u64;
        nat_eq(a4298, a4299)
    } else {
        false
    } {
        let a4304 = match v3.clone() {
            Adt2::C0(_, _, _, _, _, h4300, _, _, _, _, _) => {
                h4300
            }
        };
        let a4305 = {
            let a4302 = match v3.clone() {
                Adt2::C0(h4301, _, _, _, _, _, _, _, _, _, _) => {
                    h4301
                }
            };
            let a4303 = 0u64;
            if nat_eq(a4302, a4303) {
                4u64
            } else {
                2u64
            }
        };
        nat_eq(a4304, a4305)
    } else {
        false
    } {
        f45(v4.clone())?
    } else {
        false
    } {
        f27(v3.clone(), v4.clone())?
    } else {
        false
    } {
        f240(v5.clone(), match v3.clone() {
            Adt2::C0(_, h4306, _, _, _, _, _, _, _, _, _) => {
                h4306
            }
        }, match v4.clone() {
            Adt2::C0(_, h4307, _, _, _, _, _, _, _, _, _) => {
                h4307
            }
        })?
    } else {
        false
    } {
        if v1 {
            f240(v6.clone(), match v3.clone() {
                Adt2::C0(_, _, h4308, _, _, _, _, _, _, _, _) => {
                    h4308
                }
            }, match v4.clone() {
                Adt2::C0(_, _, h4309, _, _, _, _, _, _, _, _) => {
                    h4309
                }
            })?
        } else {
            let a4312 = match v3.clone() {
                Adt2::C0(_, _, h4310, _, _, _, _, _, _, _, _) => {
                    h4310
                }
            };
            let a4313 = match v4.clone() {
                Adt2::C0(_, _, h4311, _, _, _, _, _, _, _, _) => {
                    h4311
                }
            };
            nat_eq(a4312, a4313)
        }
    } else {
        false
    } {
        let a4316 = match v3.clone() {
            Adt2::C0(_, _, _, h4314, _, _, _, _, _, _, _) => {
                h4314
            }
        };
        let a4317 = match v4.clone() {
            Adt2::C0(_, _, _, h4315, _, _, _, _, _, _, _) => {
                h4315
            }
        };
        nat_eq(a4316, a4317)
    } else {
        false
    } {
        let a4320 = match v3.clone() {
            Adt2::C0(_, _, _, _, h4318, _, _, _, _, _, _) => {
                h4318
            }
        };
        let a4321 = match v4.clone() {
            Adt2::C0(_, _, _, _, h4319, _, _, _, _, _, _) => {
                h4319
            }
        };
        nat_eq(a4320, a4321)
    } else {
        false
    } {
        let a4326 = {
            let a4324 = match v5.clone() {
                Adt6::C0(h4322, _, _, _) => {
                    h4322
                }
            };
            let a4325 = match v6.clone() {
                Adt6::C0(h4323, _, _, _) => {
                    h4323
                }
            };
            nat_mul(a4324, a4325)?
        };
        let a4327 = 2147483648u64;
        nat_lt(a4326, a4327)
    } else {
        false
    } {
        let v7: Adt7 = Adt7::C0(v5, v6, v1, {
            let a4329 = match v2 {
                Adt1::C0(h4328, _, _, _, _, _, _, _, _, _, _, _) => {
                    h4328
                }
            };
            let a4330 = 0u64;
            nat_eq(a4329, a4330)
        }, match v3.clone() {
            Adt2::C0(h4331, _, _, _, _, _, _, _, _, _, _) => {
                h4331
            }
        });
        if v0 {
            Ok(Adt10::C4({
                let a4343 = f25(v3.clone(), 1u64)?;
                let a4344 = {
                    let a4341 = f25(v4.clone(), 1u64)?;
                    let a4342 = f189(match v7.clone() {
                        Adt7::C0(_, _, _, h4332, _) => {
                            h4332
                        }
                    }, {
                        let a4339 = {
                            let m4335 = match v7.clone() {
                                Adt7::C0(h4334, _, _, _, _) => {
                                    h4334
                                }
                            };
                            match m4335 {
                                Adt6::C0(h4333, _, _, _) => {
                                    h4333
                                }
                            }
                        };
                        let a4340 = {
                            let m4338 = match v7.clone() {
                                Adt7::C0(_, h4337, _, _, _) => {
                                    h4337
                                }
                            };
                            match m4338 {
                                Adt6::C0(h4336, _, _, _) => {
                                    h4336
                                }
                            }
                        };
                        nat_mul(a4339, a4340)?
                    })?;
                    nat_mul(a4341, a4342)?
                };
                nat_add(a4343, a4344)?
            }, 4u64, f28(v4, 1u64), f28(v3.clone(), 1u64), Adt4::C43(v7, f25(v3, 1u64)?)))
        } else {
            Ok(Adt10::C4({
                let a4345 = f25(v4.clone(), 1u64)?;
                let a4346 = f207(v7.clone())?;
                nat_mul(a4345, a4346)?
            }, 4u64, f28(v3, 1u64), f28(v4, 1u64), Adt4::C28(v7)))
        }
    } else {
        Ok(Adt10::C1)
    }
}

pub fn f132(v0: bool, v1: Adt1, v2: Adt2, v3: Option<Adt2>, v4: Adt2) -> R<Adt10> {
    let m4347 = v3;
    match m4347 {
        None => {
            Ok(Adt10::C1)
        }
        Some(v5) => {
            let v6: bool = {
                let a4349 = match v1.clone() {
                    Adt1::C0(_, _, _, _, _, _, h4348, _, _, _, _, _) => {
                        h4348
                    }
                };
                let a4350 = 1u64;
                nat_eq(a4349, a4350)
            };
            let v7: Adt6 = Adt6::C0(match v2.clone() {
                Adt2::C0(_, h4351, _, _, _, _, _, _, _, _, _) => {
                    h4351
                }
            }, match v1.clone() {
                Adt1::C0(h4352, _, _, _, _, _, _, _, _, _, _, _) => {
                    h4352
                }
            }, match v1.clone() {
                Adt1::C0(_, _, h4353, _, _, _, _, _, _, _, _, _) => {
                    h4353
                }
            }, match v1.clone() {
                Adt1::C0(_, _, _, _, h4354, _, _, _, _, _, _, _) => {
                    h4354
                }
            });
            let v8: Adt6 = if v6 {
                Adt6::C0(match v2.clone() {
                    Adt2::C0(_, _, h4355, _, _, _, _, _, _, _, _) => {
                        h4355
                    }
                }, match v1.clone() {
                    Adt1::C0(_, h4356, _, _, _, _, _, _, _, _, _, _) => {
                        h4356
                    }
                }, match v1.clone() {
                    Adt1::C0(_, _, _, h4357, _, _, _, _, _, _, _, _) => {
                        h4357
                    }
                }, match v1.clone() {
                    Adt1::C0(_, _, _, _, _, h4358, _, _, _, _, _, _) => {
                        h4358
                    }
                })
            } else {
                Adt6::C0(1u64, 1u64, 0u64, 1u64)
            };
            let v9: Adt6 = Adt6::C0(1u64, 1u64, 0u64, 1u64);
            let v10: u64 = if v6 {
                match v5.clone() {
                    Adt2::C0(_, _, _, h4359, _, _, _, _, _, _, _) => {
                        h4359
                    }
                }
            } else {
                match v5.clone() {
                    Adt2::C0(_, _, h4360, _, _, _, _, _, _, _, _) => {
                        h4360
                    }
                }
            };
            let v11: u64 = if v6 {
                match v5.clone() {
                    Adt2::C0(_, _, _, _, h4361, _, _, _, _, _, _) => {
                        h4361
                    }
                }
            } else {
                match v5.clone() {
                    Adt2::C0(_, _, _, h4362, _, _, _, _, _, _, _) => {
                        h4362
                    }
                }
            };
            let v12: u64 = if v6 {
                match v5.clone() {
                    Adt2::C0(_, _, h4363, _, _, _, _, _, _, _, _) => {
                        h4363
                    }
                }
            } else {
                1u64
            };
            let v13: u64 = match v5.clone() {
                Adt2::C0(h4364, _, _, _, _, _, _, _, _, _, _) => {
                    h4364
                }
            };
            let v14: u64 = match v4.clone() {
                Adt2::C0(h4365, _, _, _, _, _, _, _, _, _, _) => {
                    h4365
                }
            };
            let v15: u64 = {
                let a4366 = v13;
                let a4367 = 0u64;
                if nat_eq(a4366, a4367) {
                    4u64
                } else {
                    2u64
                }
            };
            let v16: u64 = {
                let a4369 = match v5.clone() {
                    Adt2::C0(_, h4368, _, _, _, _, _, _, _, _, _) => {
                        h4368
                    }
                };
                let a4370 = v15;
                nat_mul(a4369, a4370)?
            };
            let v17: u64 = if v6 {
                match v5.clone() {
                    Adt2::C0(_, _, _, _, _, _, _, h4371, _, _, _) => {
                        h4371
                    }
                }
            } else {
                match v5.clone() {
                    Adt2::C0(_, _, _, _, _, _, h4372, _, _, _, _) => {
                        h4372
                    }
                }
            };
            let v18: u64 = if v6 {
                match v5.clone() {
                    Adt2::C0(_, _, _, _, _, _, _, _, h4373, _, _) => {
                        h4373
                    }
                }
            } else {
                match v5.clone() {
                    Adt2::C0(_, _, _, _, _, _, _, h4374, _, _, _) => {
                        h4374
                    }
                }
            };
            let a4376 = match v1 {
                Adt1::C0(_, _, _, _, _, _, h4375, _, _, _, _, _) => {
                    h4375
                }
            };
            let a4377 = 1u64;
            if if if if if if if if if if nat_le(a4376, a4377) {
                let a4378 = v13;
                let a4379 = 0u64;
                if nat_eq(a4378, a4379) {
                    true
                } else {
                    let a4380 = v13;
                    let a4381 = 1u64;
                    if nat_eq(a4380, a4381) {
                        let a4382 = v14;
                        let a4383 = 1u64;
                        nat_eq(a4382, a4383)
                    } else {
                        false
                    }
                }
            } else {
                false
            } {
                let a4384 = v14;
                let a4385 = 0u64;
                if nat_eq(a4384, a4385) {
                    true
                } else {
                    let a4386 = v14;
                    let a4387 = 1u64;
                    nat_eq(a4386, a4387)
                }
            } else {
                false
            } {
                let a4389 = match v5.clone() {
                    Adt2::C0(_, _, _, _, _, h4388, _, _, _, _, _) => {
                        h4388
                    }
                };
                let a4390 = v15;
                nat_eq(a4389, a4390)
            } else {
                false
            } {
                let a4398 = match v4.clone() {
                    Adt2::C0(_, h4391, _, _, _, _, _, _, _, _, _) => {
                        h4391
                    }
                };
                let a4399 = {
                    let a4396 = v10;
                    let a4397 = {
                        let a4394 = match v7.clone() {
                            Adt6::C0(h4392, _, _, _) => {
                                h4392
                            }
                        };
                        let a4395 = match v8.clone() {
                            Adt6::C0(h4393, _, _, _) => {
                                h4393
                            }
                        };
                        nat_mul(a4394, a4395)?
                    };
                    nat_mul(a4396, a4397)?
                };
                nat_eq(a4398, a4399)
            } else {
                false
            } {
                let a4402 = match v4.clone() {
                    Adt2::C0(_, _, h4400, _, _, _, _, _, _, _, _) => {
                        h4400
                    }
                };
                let a4403 = f108(v7.clone(), match v5.clone() {
                    Adt2::C0(_, h4401, _, _, _, _, _, _, _, _, _) => {
                        h4401
                    }
                })?;
                nat_eq(a4402, a4403)
            } else {
                false
            } {
                let a4405 = 0u64;
                let a4406 = match v4.clone() {
                    Adt2::C0(_, _, h4404, _, _, _, _, _, _, _, _) => {
                        h4404
                    }
                };
                nat_lt(a4405, a4406)
            } else {
                false
            } {
                if v6 {
                    let a4408 = match v2 {
                        Adt2::C0(_, _, _, h4407, _, _, _, _, _, _, _) => {
                            h4407
                        }
                    };
                    let a4409 = v10;
                    if if if nat_eq(a4408, a4409) {
                        let a4411 = match v4.clone() {
                            Adt2::C0(_, _, _, h4410, _, _, _, _, _, _, _) => {
                                h4410
                            }
                        };
                        let a4412 = f108(v8.clone(), v12)?;
                        nat_eq(a4411, a4412)
                    } else {
                        false
                    } {
                        let a4414 = match v4.clone() {
                            Adt2::C0(_, _, _, _, h4413, _, _, _, _, _, _) => {
                                h4413
                            }
                        };
                        let a4415 = v11;
                        nat_eq(a4414, a4415)
                    } else {
                        false
                    } {
                        let a4417 = 0u64;
                        let a4418 = match v4.clone() {
                            Adt2::C0(_, _, _, h4416, _, _, _, _, _, _, _) => {
                                h4416
                            }
                        };
                        nat_lt(a4417, a4418)
                    } else {
                        false
                    }
                } else {
                    let a4420 = match v2 {
                        Adt2::C0(_, _, h4419, _, _, _, _, _, _, _, _) => {
                            h4419
                        }
                    };
                    let a4421 = v10;
                    if if if nat_eq(a4420, a4421) {
                        let a4423 = match v5.clone() {
                            Adt2::C0(_, _, _, _, h4422, _, _, _, _, _, _) => {
                                h4422
                            }
                        };
                        let a4424 = 1u64;
                        nat_eq(a4423, a4424)
                    } else {
                        false
                    } {
                        let a4426 = match v4.clone() {
                            Adt2::C0(_, _, _, h4425, _, _, _, _, _, _, _) => {
                                h4425
                            }
                        };
                        let a4427 = v11;
                        nat_eq(a4426, a4427)
                    } else {
                        false
                    } {
                        let a4429 = match v4.clone() {
                            Adt2::C0(_, _, _, _, h4428, _, _, _, _, _, _) => {
                                h4428
                            }
                        };
                        let a4430 = 1u64;
                        nat_eq(a4429, a4430)
                    } else {
                        false
                    }
                }
            } else {
                false
            } {
                if f55(match v5.clone() {
                    Adt2::C0(_, h4431, _, _, _, _, _, _, _, _, _) => {
                        h4431
                    }
                }, v12, v10, v11) {
                    true
                } else {
                    let a4451 = {
                        let a4448 = {
                            let a4446 = {
                                let a4440 = {
                                    let a4434 = {
                                        let a4432 = v11;
                                        let a4433 = 1u64;
                                        nat_sub(a4432, a4433)
                                    };
                                    let a4435 = v18;
                                    nat_mul(a4434, a4435)?
                                };
                                let a4441 = {
                                    let a4438 = {
                                        let a4436 = v10;
                                        let a4437 = 1u64;
                                        nat_sub(a4436, a4437)
                                    };
                                    let a4439 = v17;
                                    nat_mul(a4438, a4439)?
                                };
                                nat_add(a4440, a4441)?
                            };
                            let a4447 = {
                                let a4444 = {
                                    let a4442 = v12;
                                    let a4443 = 1u64;
                                    nat_sub(a4442, a4443)
                                };
                                let a4445 = v16;
                                nat_mul(a4444, a4445)?
                            };
                            nat_add(a4446, a4447)?
                        };
                        let a4449 = v16;
                        nat_add(a4448, a4449)?
                    };
                    let a4452 = match v5.clone() {
                        Adt2::C0(_, _, _, _, _, _, _, _, _, h4450, _) => {
                            h4450
                        }
                    };
                    nat_le(a4451, a4452)
                }
            } else {
                false
            } {
                f27(v5.clone(), v4.clone())?
            } else {
                false
            } {
                let v19: Adt3 = if v6 {
                    f28(v4.clone(), 1u64)
                } else {
                    Adt3::C0(match v4.clone() {
                        Adt2::C0(_, h4453, _, _, _, _, _, _, _, _, _) => {
                            h4453
                        }
                    }, match v4.clone() {
                        Adt2::C0(_, _, h4454, _, _, _, _, _, _, _, _) => {
                            h4454
                        }
                    }, 1u64, match v4.clone() {
                        Adt2::C0(_, _, _, h4455, _, _, _, _, _, _, _) => {
                            h4455
                        }
                    }, match v4.clone() {
                        Adt2::C0(_, _, _, _, _, h4456, _, _, _, _, _) => {
                            h4456
                        }
                    }, match v4.clone() {
                        Adt2::C0(_, _, _, _, _, _, h4457, _, _, _, _) => {
                            h4457
                        }
                    }, 0u64, match v4.clone() {
                        Adt2::C0(_, _, _, _, _, _, _, h4458, _, _, _) => {
                            h4458
                        }
                    })
                };
                if v0 {
                    Ok(Adt10::C4({
                        let a4465 = f25(v5.clone(), 1u64)?;
                        let a4466 = {
                            let a4463 = {
                                let a4461 = match v7.clone() {
                                    Adt6::C0(h4459, _, _, _) => {
                                        h4459
                                    }
                                };
                                let a4462 = match v8.clone() {
                                    Adt6::C0(h4460, _, _, _) => {
                                        h4460
                                    }
                                };
                                nat_mul(a4461, a4462)?
                            };
                            let a4464 = 2u64;
                            nat_add(a4463, a4464)?
                        };
                        nat_mul(a4465, a4466)?
                    }, 4u64, v19, f44(v6, v5), Adt4::C42(v7, v8)))
                } else {
                    Ok(Adt10::C4(f25(v4, 1u64)?, {
                        let a4467 = v14;
                        let a4468 = 0u64;
                        if nat_eq(a4467, a4468) {
                            4u64
                        } else {
                            2u64
                        }
                    }, Adt3::C0(match v5 {
                        Adt2::C0(_, h4469, _, _, _, _, _, _, _, _, _) => {
                            h4469
                        }
                    }, v12, 1u64, v11, v15, v16, 0u64, 0u64), v19, Adt4::C27(Adt5::C0(v7, v8, v9, v17, v18, v13, 1u64, v14))))
                }
            } else {
                Ok(Adt10::C1)
            }
        }
    }
}

pub fn f133(v0: Adt1, v1: Adt2, v2: Option<Adt2>, v3: Adt2) -> R<Adt10> {
    let m4470 = v2;
    match m4470 {
        None => {
            Ok(Adt10::C1)
        }
        Some(v4) => {
            let v5: Adt6 = Adt6::C0(match v1.clone() {
                Adt2::C0(_, h4471, _, _, _, _, _, _, _, _, _) => {
                    h4471
                }
            }, match v0.clone() {
                Adt1::C0(h4472, _, _, _, _, _, _, _, _, _, _, _) => {
                    h4472
                }
            }, match v0.clone() {
                Adt1::C0(_, _, _, h4473, _, _, _, _, _, _, _, _) => {
                    h4473
                }
            }, match v0.clone() {
                Adt1::C0(_, _, _, _, _, _, h4474, _, _, _, _, _) => {
                    h4474
                }
            });
            let v6: Adt6 = Adt6::C0(match v1.clone() {
                Adt2::C0(_, _, h4475, _, _, _, _, _, _, _, _) => {
                    h4475
                }
            }, match v0.clone() {
                Adt1::C0(_, h4476, _, _, _, _, _, _, _, _, _, _) => {
                    h4476
                }
            }, match v0.clone() {
                Adt1::C0(_, _, _, _, h4477, _, _, _, _, _, _, _) => {
                    h4477
                }
            }, match v0.clone() {
                Adt1::C0(_, _, _, _, _, _, _, h4478, _, _, _, _) => {
                    h4478
                }
            });
            let v7: Adt6 = Adt6::C0(match v1 {
                Adt2::C0(_, _, _, h4479, _, _, _, _, _, _, _) => {
                    h4479
                }
            }, match v0.clone() {
                Adt1::C0(_, _, h4480, _, _, _, _, _, _, _, _, _) => {
                    h4480
                }
            }, match v0.clone() {
                Adt1::C0(_, _, _, _, _, h4481, _, _, _, _, _, _) => {
                    h4481
                }
            }, match v0.clone() {
                Adt1::C0(_, _, _, _, _, _, _, _, h4482, _, _, _) => {
                    h4482
                }
            });
            let v8: u64 = match v0 {
                Adt1::C0(_, _, _, _, _, _, _, _, _, h4483, _, _) => {
                    h4483
                }
            };
            let v9: u64 = f108(v7.clone(), match v4.clone() {
                Adt2::C0(_, _, _, h4484, _, _, _, _, _, _, _) => {
                    h4484
                }
            })?;
            let v10: u64 = match v3.clone() {
                Adt2::C0(h4485, _, _, _, _, _, _, _, _, _, _) => {
                    h4485
                }
            };
            let v11: u64 = {
                let a4493 = v8;
                let a4494 = {
                    let a4491 = match v5.clone() {
                        Adt6::C0(h4486, _, _, _) => {
                            h4486
                        }
                    };
                    let a4492 = {
                        let a4489 = match v6.clone() {
                            Adt6::C0(h4487, _, _, _) => {
                                h4487
                            }
                        };
                        let a4490 = match v7.clone() {
                            Adt6::C0(h4488, _, _, _) => {
                                h4488
                            }
                        };
                        nat_mul(a4489, a4490)?
                    };
                    nat_mul(a4491, a4492)?
                };
                nat_mul(a4493, a4494)?
            };
            let a4496 = match v4.clone() {
                Adt2::C0(h4495, _, _, _, _, _, _, _, _, _, _) => {
                    h4495
                }
            };
            let a4497 = 0u64;
            if if if if if if if if if if if if if nat_eq(a4496, a4497) {
                let a4498 = v10;
                let a4499 = 0u64;
                if nat_eq(a4498, a4499) {
                    true
                } else {
                    let a4500 = v10;
                    let a4501 = 1u64;
                    nat_eq(a4500, a4501)
                }
            } else {
                false
            } {
                let a4502 = 0u64;
                let a4503 = v8;
                nat_lt(a4502, a4503)
            } else {
                false
            } {
                let a4504 = v8;
                let a4505 = 2147483648u64;
                nat_lt(a4504, a4505)
            } else {
                false
            } {
                let a4507 = match v4.clone() {
                    Adt2::C0(_, _, _, _, _, h4506, _, _, _, _, _) => {
                        h4506
                    }
                };
                let a4508 = 4u64;
                nat_eq(a4507, a4508)
            } else {
                false
            } {
                let a4509 = 0u64;
                let a4510 = v9;
                nat_lt(a4509, a4510)
            } else {
                false
            } {
                let a4512 = 0u64;
                let a4513 = match v3.clone() {
                    Adt2::C0(_, _, h4511, _, _, _, _, _, _, _, _) => {
                        h4511
                    }
                };
                nat_lt(a4512, a4513)
            } else {
                false
            } {
                let a4515 = 0u64;
                let a4516 = match v3.clone() {
                    Adt2::C0(_, _, _, h4514, _, _, _, _, _, _, _) => {
                        h4514
                    }
                };
                nat_lt(a4515, a4516)
            } else {
                false
            } {
                let a4518 = match v3.clone() {
                    Adt2::C0(_, h4517, _, _, _, _, _, _, _, _, _) => {
                        h4517
                    }
                };
                let a4519 = v11;
                nat_eq(a4518, a4519)
            } else {
                false
            } {
                let a4522 = match v3.clone() {
                    Adt2::C0(_, _, h4520, _, _, _, _, _, _, _, _) => {
                        h4520
                    }
                };
                let a4523 = f108(v5.clone(), match v4.clone() {
                    Adt2::C0(_, h4521, _, _, _, _, _, _, _, _, _) => {
                        h4521
                    }
                })?;
                nat_eq(a4522, a4523)
            } else {
                false
            } {
                let a4526 = match v3.clone() {
                    Adt2::C0(_, _, _, h4524, _, _, _, _, _, _, _) => {
                        h4524
                    }
                };
                let a4527 = f108(v6.clone(), match v4.clone() {
                    Adt2::C0(_, _, h4525, _, _, _, _, _, _, _, _) => {
                        h4525
                    }
                })?;
                nat_eq(a4526, a4527)
            } else {
                false
            } {
                let a4535 = match v3.clone() {
                    Adt2::C0(_, _, _, _, h4528, _, _, _, _, _, _) => {
                        h4528
                    }
                };
                let a4536 = {
                    let a4533 = v9;
                    let a4534 = {
                        let a4530 = match v4.clone() {
                            Adt2::C0(_, _, _, _, h4529, _, _, _, _, _, _) => {
                                h4529
                            }
                        };
                        let a4531 = v8;
                        let a4532 = 0u64;
                        nat_quot(a4530, a4531, a4532)
                    };
                    nat_mul(a4533, a4534)?
                };
                nat_eq(a4535, a4536)
            } else {
                false
            } {
                f27(v4.clone(), v3.clone())?
            } else {
                false
            } {
                Ok(Adt10::C4(f25(v3.clone(), 1u64)?, {
                    let a4537 = v10;
                    let a4538 = 0u64;
                    if nat_eq(a4537, a4538) {
                        4u64
                    } else {
                        2u64
                    }
                }, f28(v4.clone(), 1u64), f28(v3, 1u64), Adt4::C27(Adt5::C0(v5, v6, v7, match v4.clone() {
                    Adt2::C0(_, _, _, _, _, _, _, _, h4539, _, _) => {
                        h4539
                    }
                }, {
                    let a4541 = v8;
                    let a4542 = match v4 {
                        Adt2::C0(_, _, _, _, _, _, _, _, h4540, _, _) => {
                            h4540
                        }
                    };
                    nat_mul(a4541, a4542)?
                }, 0u64, v9, v10))))
            } else {
                Ok(Adt10::C1)
            }
        }
    }
}

pub fn f134(v0: Adt2, v1: Option<Adt2>, v2: Adt2) -> R<Adt10> {
    let m4543 = v1;
    match m4543 {
        None => {
            Ok(Adt10::C1)
        }
        Some(v3) => {
            f241(v0, v3, v2)
        }
    }
}

pub fn f135(v0: Adt2, v1: Adt2) -> R<Adt10> {
    let a4545 = match v0.clone() {
        Adt2::C0(h4544, _, _, _, _, _, _, _, _, _, _) => {
            h4544
        }
    };
    let a4546 = 1u64;
    if if nat_eq(a4545, a4546) {
        true
    } else {
        let a4548 = match v0.clone() {
            Adt2::C0(h4547, _, _, _, _, _, _, _, _, _, _) => {
                h4547
            }
        };
        let a4549 = 30u64;
        nat_eq(a4548, a4549)
    } {
        let a4551 = match v1.clone() {
            Adt2::C0(h4550, _, _, _, _, _, _, _, _, _, _) => {
                h4550
            }
        };
        let a4552 = 1u64;
        if if if if if if if if if if if nat_eq(a4551, a4552) {
            let a4555 = match v0.clone() {
                Adt2::C0(_, h4553, _, _, _, _, _, _, _, _, _) => {
                    h4553
                }
            };
            let a4556 = match v1.clone() {
                Adt2::C0(_, h4554, _, _, _, _, _, _, _, _, _) => {
                    h4554
                }
            };
            nat_eq(a4555, a4556)
        } else {
            false
        } {
            let a4558 = 0u64;
            let a4559 = match v1.clone() {
                Adt2::C0(_, _, h4557, _, _, _, _, _, _, _, _) => {
                    h4557
                }
            };
            nat_lt(a4558, a4559)
        } else {
            false
        } {
            let a4562 = match v1.clone() {
                Adt2::C0(_, _, h4560, _, _, _, _, _, _, _, _) => {
                    h4560
                }
            };
            let a4563 = match v1.clone() {
                Adt2::C0(_, _, _, h4561, _, _, _, _, _, _, _) => {
                    h4561
                }
            };
            nat_eq(a4562, a4563)
        } else {
            false
        } {
            let a4565 = match v1.clone() {
                Adt2::C0(_, _, _, _, h4564, _, _, _, _, _, _) => {
                    h4564
                }
            };
            let a4566 = 1u64;
            nat_eq(a4565, a4566)
        } else {
            false
        } {
            let a4574 = match v0.clone() {
                Adt2::C0(_, _, h4567, _, _, _, _, _, _, _, _) => {
                    h4567
                }
            };
            let a4575 = {
                let a4572 = {
                    let a4570 = match v1.clone() {
                        Adt2::C0(_, _, h4568, _, _, _, _, _, _, _, _) => {
                            h4568
                        }
                    };
                    let a4571 = match v1.clone() {
                        Adt2::C0(_, _, h4569, _, _, _, _, _, _, _, _) => {
                            h4569
                        }
                    };
                    nat_add(a4570, a4571)?
                };
                let a4573 = 1u64;
                nat_sub(a4572, a4573)
            };
            nat_eq(a4574, a4575)
        } else {
            false
        } {
            let a4577 = 0u64;
            let a4578 = match v0.clone() {
                Adt2::C0(_, _, _, h4576, _, _, _, _, _, _, _) => {
                    h4576
                }
            };
            nat_lt(a4577, a4578)
        } else {
            false
        } {
            let a4580 = 0u64;
            let a4581 = match v0.clone() {
                Adt2::C0(_, _, _, _, h4579, _, _, _, _, _, _) => {
                    h4579
                }
            };
            nat_lt(a4580, a4581)
        } else {
            false
        } {
            let a4583 = 2u64;
            let a4584 = match v0.clone() {
                Adt2::C0(_, _, _, _, _, h4582, _, _, _, _, _) => {
                    h4582
                }
            };
            nat_le(a4583, a4584)
        } else {
            false
        } {
            let a4586 = 2u64;
            let a4587 = match v1.clone() {
                Adt2::C0(_, _, _, _, _, h4585, _, _, _, _, _) => {
                    h4585
                }
            };
            nat_le(a4586, a4587)
        } else {
            false
        } {
            f27(v0.clone(), v1.clone())?
        } else {
            false
        } {
            Ok(Adt10::C4(f25(v1.clone(), 1u64)?, 2u64, f28(v0, 1u64), f28(v1, 1u64), Adt4::C21))
        } else {
            Ok(Adt10::C1)
        }
    } else {
        Ok(Adt10::C0)
    }
}

pub fn f136(v0: Adt2, v1: Option<Adt2>, v2: Adt2) -> R<Adt10> {
    let m4588 = v1;
    match m4588 {
        None => {
            Ok(Adt10::C1)
        }
        Some(v3) => {
            f242(v0, v3, v2)
        }
    }
}

pub fn f137(v0: u64) -> bool {
    let a4589 = v0;
    let a4590 = 1u64;
    if if nat_eq(a4589, a4590) {
        true
    } else {
        let a4591 = v0;
        let a4592 = 33u64;
        nat_eq(a4591, a4592)
    } {
        true
    } else {
        let a4593 = v0;
        let a4594 = 34u64;
        nat_eq(a4593, a4594)
    }
}

pub fn f138(v0: Adt2, v1: Adt2, v2: Adt2) -> R<Adt10> {
    let v3: u64 = f25(v0.clone(), 1u64)?;
    if if if if if f54(v1, v2.clone()) {
        let a4595 = v3;
        let a4596 = f25(v2.clone(), 1u64)?;
        nat_eq(a4595, a4596)
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
        let a4597 = v3;
        let a4598 = 0u64;
        if nat_eq(a4597, a4598) {
            true
        } else {
            let a4600 = 4u64;
            let a4601 = match v2.clone() {
                Adt2::C0(_, _, _, _, _, h4599, _, _, _, _, _) => {
                    h4599
                }
            };
            nat_le(a4600, a4601)
        }
    } else {
        false
    } {
        Ok(Adt10::C4({
            let a4602 = 2u64;
            let a4603 = v3;
            nat_mul(a4602, a4603)?
        }, 4u64, f28(v0, 1u64), f28(v2, 1u64), Adt4::C35(v3)))
    } else {
        Ok(Adt10::C1)
    }
}

pub fn f139(v0: u64) -> bool {
    if f243(v0) {
        !f244(v0)
    } else {
        false
    }
}

pub fn f140(v0: Adt2, v1: Option<Adt2>, v2: Adt2) -> R<Adt10> {
    let m4604 = v1;
    match m4604 {
        None => {
            Ok(Adt10::C1)
        }
        Some(v3) => {
            let m4606 = f235(match v0.clone() {
                Adt2::C0(h4605, _, _, _, _, _, _, _, _, _, _) => {
                    h4605
                }
            });
            match m4606 {
                Adt18::C0 => {
                    Ok(Adt10::C0)
                }
                Adt18::C1(v4) => {
                    let v5: u64 = match v4.clone() {
                        Adt19::C0(_, h4607, _) => {
                            h4607
                        }
                    };
                    if if if if if if f54(v3, v2.clone()) {
                        f27(v0.clone(), v2.clone())?
                    } else {
                        false
                    } {
                        let a4608 = f25(v0.clone(), 1u64)?;
                        let a4609 = f25(v2.clone(), 1u64)?;
                        nat_eq(a4608, a4609)
                    } else {
                        false
                    } {
                        let a4614 = {
                            let a4611 = match v0.clone() {
                                Adt2::C0(_, h4610, _, _, _, _, _, _, _, _, _) => {
                                    h4610
                                }
                            };
                            let a4612 = v5;
                            let a4613 = 1u64;
                            nat_rem(a4611, a4612, a4613)
                        };
                        let a4615 = 0u64;
                        if nat_eq(a4614, a4615) {
                            let a4618 = match v4 {
                                Adt19::C0(_, _, h4616) => {
                                    h4616
                                }
                            };
                            let a4619 = match v0.clone() {
                                Adt2::C0(_, _, _, _, _, h4617, _, _, _, _, _) => {
                                    h4617
                                }
                            };
                            nat_le(a4618, a4619)
                        } else {
                            false
                        }
                    } else {
                        false
                    } {
                        let a4620 = v5;
                        let a4621 = 1u64;
                        if nat_eq(a4620, a4621) {
                            let a4623 = 4u64;
                            let a4624 = match v2.clone() {
                                Adt2::C0(_, _, _, _, _, h4622, _, _, _, _, _) => {
                                    h4622
                                }
                            };
                            nat_le(a4623, a4624)
                        } else {
                            let a4626 = match v2.clone() {
                                Adt2::C0(_, _, _, _, _, h4625, _, _, _, _, _) => {
                                    h4625
                                }
                            };
                            let a4627 = 4u64;
                            nat_eq(a4626, a4627)
                        }
                    } else {
                        false
                    } {
                        let a4632 = {
                            let a4629 = match v2.clone() {
                                Adt2::C0(_, h4628, _, _, _, _, _, _, _, _, _) => {
                                    h4628
                                }
                            };
                            let a4630 = v5;
                            let a4631 = 1u64;
                            nat_rem(a4629, a4630, a4631)
                        };
                        let a4633 = 0u64;
                        if nat_eq(a4632, a4633) {
                            true
                        } else {
                            f45(v2.clone())?
                        }
                    } else {
                        false
                    } {
                        Ok(Adt10::C4(f25(v0.clone(), v5)?, 4u64, f28(v0.clone(), v5), f28(v2, 1u64), Adt4::C11(match v0 {
                            Adt2::C0(h4634, _, _, _, _, _, _, _, _, _, _) => {
                                h4634
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

pub fn f141(v0: u64, v1: Adt2, v2: bool, v3: Adt2) -> R<Adt10> {
    let a4637 = match v1.clone() {
        Adt2::C0(h4635, _, _, _, _, _, _, _, _, _, _) => {
            h4635
        }
    };
    let a4638 = match v3.clone() {
        Adt2::C0(h4636, _, _, _, _, _, _, _, _, _, _) => {
            h4636
        }
    };
    if nat_eq(a4637, a4638) {
        f245(v0, v1, v2, v3)
    } else {
        if if if if f244(match v1.clone() {
            Adt2::C0(h4639, _, _, _, _, _, _, _, _, _, _) => {
                h4639
            }
        }) {
            f244(match v3.clone() {
                Adt2::C0(h4640, _, _, _, _, _, _, _, _, _, _) => {
                    h4640
                }
            })
        } else {
            false
        } {
            let a4641 = v0;
            let a4642 = 33u64;
            if nat_eq(a4641, a4642) {
                v2
            } else {
                true
            }
        } else {
            false
        } {
            let a4643 = f25(v1.clone(), 1u64)?;
            let a4644 = f25(v3.clone(), 1u64)?;
            nat_eq(a4643, a4644)
        } else {
            false
        } {
            let m4646 = f235(match v3.clone() {
                Adt2::C0(h4645, _, _, _, _, _, _, _, _, _, _) => {
                    h4645
                }
            });
            match m4646 {
                Adt18::C0 => {
                    Ok(Adt10::C0)
                }
                Adt18::C1(v4) => {
                    let v5: u64 = match v1.clone() {
                        Adt2::C0(h4647, _, _, _, _, _, _, _, _, _, _) => {
                            h4647
                        }
                    };
                    let v6: u64 = match v3.clone() {
                        Adt2::C0(h4648, _, _, _, _, _, _, _, _, _, _) => {
                            h4648
                        }
                    };
                    let v7: bool = {
                        let a4649 = v5;
                        let a4650 = 43u64;
                        if nat_eq(a4649, a4650) {
                            true
                        } else {
                            let a4651 = v6;
                            let a4652 = 43u64;
                            nat_eq(a4651, a4652)
                        }
                    };
                    Ok(Adt10::C4({
                        let a4653 = f25(v3.clone(), 1u64)?;
                        let a4654 = if v7 {
                            f182(v5, v6)
                        } else {
                            1u64
                        };
                        nat_mul(a4653, a4654)?
                    }, match v4 {
                        Adt19::C0(_, _, h4655) => {
                            h4655
                        }
                    }, f28(v1, 1u64), f28(v3, 1u64), if v7 {
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

pub fn f142(v0: Adt2, v1: Adt2) -> R<Adt10> {
    let a4657 = match v0.clone() {
        Adt2::C0(h4656, _, _, _, _, _, _, _, _, _, _) => {
            h4656
        }
    };
    let a4658 = 0u64;
    if if if if nat_eq(a4657, a4658) {
        true
    } else {
        let a4660 = match v0.clone() {
            Adt2::C0(h4659, _, _, _, _, _, _, _, _, _, _) => {
                h4659
            }
        };
        let a4661 = 1u64;
        nat_eq(a4660, a4661)
    } {
        true
    } else {
        let a4663 = match v0.clone() {
            Adt2::C0(h4662, _, _, _, _, _, _, _, _, _, _) => {
                h4662
            }
        };
        let a4664 = 25u64;
        if nat_eq(a4663, a4664) {
            true
        } else {
            let a4666 = match v0.clone() {
                Adt2::C0(h4665, _, _, _, _, _, _, _, _, _, _) => {
                    h4665
                }
            };
            let a4667 = 26u64;
            nat_eq(a4666, a4667)
        }
    } {
        true
    } else {
        let a4669 = match v0.clone() {
            Adt2::C0(h4668, _, _, _, _, _, _, _, _, _, _) => {
                h4668
            }
        };
        let a4670 = 30u64;
        nat_eq(a4669, a4670)
    } {
        let a4673 = match v0.clone() {
            Adt2::C0(h4671, _, _, _, _, _, _, _, _, _, _) => {
                h4671
            }
        };
        let a4674 = match v1.clone() {
            Adt2::C0(h4672, _, _, _, _, _, _, _, _, _, _) => {
                h4672
            }
        };
        if if nat_eq(a4673, a4674) {
            f246(match v1.clone() {
                Adt2::C0(_, h4675, _, _, _, _, _, _, _, _, _) => {
                    h4675
                }
            }, match v1.clone() {
                Adt2::C0(_, _, h4676, _, _, _, _, _, _, _, _) => {
                    h4676
                }
            }, match v1.clone() {
                Adt2::C0(_, _, _, h4677, _, _, _, _, _, _, _) => {
                    h4677
                }
            }, match v1.clone() {
                Adt2::C0(_, _, _, _, h4678, _, _, _, _, _, _) => {
                    h4678
                }
            }, match v0.clone() {
                Adt2::C0(_, h4679, _, _, _, _, _, _, _, _, _) => {
                    h4679
                }
            }, match v0.clone() {
                Adt2::C0(_, _, h4680, _, _, _, _, _, _, _, _) => {
                    h4680
                }
            }, match v0.clone() {
                Adt2::C0(_, _, _, h4681, _, _, _, _, _, _, _) => {
                    h4681
                }
            }, match v0.clone() {
                Adt2::C0(_, _, _, _, h4682, _, _, _, _, _, _) => {
                    h4682
                }
            })
        } else {
            false
        } {
            let m4684 = f235(match v0.clone() {
                Adt2::C0(h4683, _, _, _, _, _, _, _, _, _, _) => {
                    h4683
                }
            });
            match m4684 {
                Adt18::C0 => {
                    Ok(Adt10::C0)
                }
                Adt18::C1(v2) => {
                    Ok(Adt10::C4(f25(v1.clone(), 1u64)?, match v2 {
                        Adt19::C0(_, _, h4685) => {
                            h4685
                        }
                    }, f28(v0, 1u64), f28(v1, 1u64), Adt4::C1))
                }
            }
        } else {
            Ok(Adt10::C1)
        }
    } else {
        Ok(Adt10::C0)
    }
}

pub fn f143(v0: u64, v1: Adt2, v2: Option<Adt2>, v3: Adt2) -> R<Adt10> {
    let m4686 = v2;
    match m4686 {
        None => {
            Ok(Adt10::C1)
        }
        Some(v4) => {
            let a4687 = v0;
            let a4688 = 4u64;
            if if if nat_lt(a4687, a4688) {
                let a4691 = match v1.clone() {
                    Adt2::C0(h4689, _, _, _, _, _, _, _, _, _, _) => {
                        h4689
                    }
                };
                let a4692 = match v4.clone() {
                    Adt2::C0(h4690, _, _, _, _, _, _, _, _, _, _) => {
                        h4690
                    }
                };
                if nat_eq(a4691, a4692) {
                    let a4695 = match v1.clone() {
                        Adt2::C0(h4693, _, _, _, _, _, _, _, _, _, _) => {
                            h4693
                        }
                    };
                    let a4696 = match v3.clone() {
                        Adt2::C0(h4694, _, _, _, _, _, _, _, _, _, _) => {
                            h4694
                        }
                    };
                    nat_eq(a4695, a4696)
                } else {
                    false
                }
            } else {
                false
            } {
                if if if f247(v0, 0u64, match v1.clone() {
                    Adt2::C0(_, h4697, _, _, _, _, _, _, _, _, _) => {
                        h4697
                    }
                }, match v4.clone() {
                    Adt2::C0(_, h4698, _, _, _, _, _, _, _, _, _) => {
                        h4698
                    }
                }, match v3.clone() {
                    Adt2::C0(_, h4699, _, _, _, _, _, _, _, _, _) => {
                        h4699
                    }
                })? {
                    f247(v0, 1u64, match v1.clone() {
                        Adt2::C0(_, _, h4700, _, _, _, _, _, _, _, _) => {
                            h4700
                        }
                    }, match v4.clone() {
                        Adt2::C0(_, _, h4701, _, _, _, _, _, _, _, _) => {
                            h4701
                        }
                    }, match v3.clone() {
                        Adt2::C0(_, _, h4702, _, _, _, _, _, _, _, _) => {
                            h4702
                        }
                    })?
                } else {
                    false
                } {
                    f247(v0, 2u64, match v1.clone() {
                        Adt2::C0(_, _, _, h4703, _, _, _, _, _, _, _) => {
                            h4703
                        }
                    }, match v4.clone() {
                        Adt2::C0(_, _, _, h4704, _, _, _, _, _, _, _) => {
                            h4704
                        }
                    }, match v3.clone() {
                        Adt2::C0(_, _, _, h4705, _, _, _, _, _, _, _) => {
                            h4705
                        }
                    })?
                } else {
                    false
                } {
                    f247(v0, 3u64, match v1.clone() {
                        Adt2::C0(_, _, _, _, h4706, _, _, _, _, _, _) => {
                            h4706
                        }
                    }, match v4.clone() {
                        Adt2::C0(_, _, _, _, h4707, _, _, _, _, _, _) => {
                            h4707
                        }
                    }, match v3.clone() {
                        Adt2::C0(_, _, _, _, h4708, _, _, _, _, _, _) => {
                            h4708
                        }
                    })?
                } else {
                    false
                }
            } else {
                false
            } {
                let m4710 = f235(match v1.clone() {
                    Adt2::C0(h4709, _, _, _, _, _, _, _, _, _, _) => {
                        h4709
                    }
                });
                match m4710 {
                    Adt18::C0 => {
                        Ok(Adt10::C0)
                    }
                    Adt18::C1(v5) => {
                        let v6: u64 = match v5.clone() {
                            Adt19::C0(_, h4711, _) => {
                                h4711
                            }
                        };
                        let a4716 = {
                            let a4713 = match v1.clone() {
                                Adt2::C0(_, h4712, _, _, _, _, _, _, _, _, _) => {
                                    h4712
                                }
                            };
                            let a4714 = v6;
                            let a4715 = 1u64;
                            nat_rem(a4713, a4714, a4715)
                        };
                        let a4717 = 0u64;
                        if if if nat_eq(a4716, a4717) {
                            let a4722 = {
                                let a4719 = match v4.clone() {
                                    Adt2::C0(_, h4718, _, _, _, _, _, _, _, _, _) => {
                                        h4718
                                    }
                                };
                                let a4720 = v6;
                                let a4721 = 1u64;
                                nat_rem(a4719, a4720, a4721)
                            };
                            let a4723 = 0u64;
                            nat_eq(a4722, a4723)
                        } else {
                            false
                        } {
                            let a4728 = {
                                let a4725 = match v3.clone() {
                                    Adt2::C0(_, h4724, _, _, _, _, _, _, _, _, _) => {
                                        h4724
                                    }
                                };
                                let a4726 = v6;
                                let a4727 = 1u64;
                                nat_rem(a4725, a4726, a4727)
                            };
                            let a4729 = 0u64;
                            nat_eq(a4728, a4729)
                        } else {
                            false
                        } {
                            Ok(Adt10::C4(f25(v3.clone(), v6)?, match v5 {
                                Adt19::C0(_, _, h4730) => {
                                    h4730
                                }
                            }, f28(v1, v6), f28(v3, v6), Adt4::C2(v0, f28(v4, v6))))
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

pub fn f144(v0: u64, v1: Adt1, v2: Adt2, v3: Adt2) -> R<Adt10> {
    let a4732 = match v2.clone() {
        Adt2::C0(h4731, _, _, _, _, _, _, _, _, _, _) => {
            h4731
        }
    };
    let a4733 = 0u64;
    if if nat_eq(a4732, a4733) {
        let a4735 = match v3.clone() {
            Adt2::C0(h4734, _, _, _, _, _, _, _, _, _, _) => {
                h4734
            }
        };
        let a4736 = 0u64;
        nat_eq(a4735, a4736)
    } else {
        false
    } {
        let a4737 = v0;
        let a4738 = 61u64;
        if nat_eq(a4737, a4738) {
            let v4: bool = {
                let a4740 = 0u64;
                let a4741 = match v1.clone() {
                    Adt1::C0(_, _, _, _, _, _, _, _, h4739, _, _, _) => {
                        h4739
                    }
                };
                nat_lt(a4740, a4741)
            };
            if if if if if if f248(match v2.clone() {
                Adt2::C0(_, h4742, _, _, _, _, _, _, _, _, _) => {
                    h4742
                }
            }, match v1.clone() {
                Adt1::C0(h4743, _, _, _, _, _, _, _, _, _, _, _) => {
                    h4743
                }
            }, match v1.clone() {
                Adt1::C0(_, h4744, _, _, _, _, _, _, _, _, _, _) => {
                    h4744
                }
            }, match v3.clone() {
                Adt2::C0(_, h4745, _, _, _, _, _, _, _, _, _) => {
                    h4745
                }
            })? {
                f248(match v2.clone() {
                    Adt2::C0(_, _, h4746, _, _, _, _, _, _, _, _) => {
                        h4746
                    }
                }, match v1.clone() {
                    Adt1::C0(_, _, h4747, _, _, _, _, _, _, _, _, _) => {
                        h4747
                    }
                }, match v1.clone() {
                    Adt1::C0(_, _, _, h4748, _, _, _, _, _, _, _, _) => {
                        h4748
                    }
                }, match v3.clone() {
                    Adt2::C0(_, _, h4749, _, _, _, _, _, _, _, _) => {
                        h4749
                    }
                })?
            } else {
                false
            } {
                f248(match v2.clone() {
                    Adt2::C0(_, _, _, h4750, _, _, _, _, _, _, _) => {
                        h4750
                    }
                }, match v1.clone() {
                    Adt1::C0(_, _, _, _, h4751, _, _, _, _, _, _, _) => {
                        h4751
                    }
                }, match v1.clone() {
                    Adt1::C0(_, _, _, _, _, h4752, _, _, _, _, _, _) => {
                        h4752
                    }
                }, match v3.clone() {
                    Adt2::C0(_, _, _, h4753, _, _, _, _, _, _, _) => {
                        h4753
                    }
                })?
            } else {
                false
            } {
                f248(match v2.clone() {
                    Adt2::C0(_, _, _, _, h4754, _, _, _, _, _, _) => {
                        h4754
                    }
                }, match v1.clone() {
                    Adt1::C0(_, _, _, _, _, _, h4755, _, _, _, _, _) => {
                        h4755
                    }
                }, match v1.clone() {
                    Adt1::C0(_, _, _, _, _, _, _, h4756, _, _, _, _) => {
                        h4756
                    }
                }, match v3.clone() {
                    Adt2::C0(_, _, _, _, h4757, _, _, _, _, _, _) => {
                        h4757
                    }
                })?
            } else {
                false
            } {
                let a4759 = match v1.clone() {
                    Adt1::C0(_, _, _, _, _, _, _, _, h4758, _, _, _) => {
                        h4758
                    }
                };
                let a4760 = 4294967295u64;
                nat_le(a4759, a4760)
            } else {
                false
            } {
                if v4 {
                    !f55(match v2.clone() {
                        Adt2::C0(_, h4761, _, _, _, _, _, _, _, _, _) => {
                            h4761
                        }
                    }, match v2.clone() {
                        Adt2::C0(_, _, h4762, _, _, _, _, _, _, _, _) => {
                            h4762
                        }
                    }, match v2.clone() {
                        Adt2::C0(_, _, _, h4763, _, _, _, _, _, _, _) => {
                            h4763
                        }
                    }, match v2.clone() {
                        Adt2::C0(_, _, _, _, h4764, _, _, _, _, _, _) => {
                            h4764
                        }
                    })
                } else {
                    true
                }
            } else {
                false
            } {
                Ok(Adt10::C4(f25(v3.clone(), 1u64)?, 4u64, f28(v2, 1u64), f28(v3, 1u64), Adt4::C3(match v1.clone() {
                    Adt1::C0(h4765, _, _, _, _, _, _, _, _, _, _, _) => {
                        h4765
                    }
                }, match v1.clone() {
                    Adt1::C0(_, _, h4766, _, _, _, _, _, _, _, _, _) => {
                        h4766
                    }
                }, match v1.clone() {
                    Adt1::C0(_, _, _, _, h4767, _, _, _, _, _, _, _) => {
                        h4767
                    }
                }, match v1 {
                    Adt1::C0(_, _, _, _, _, _, h4768, _, _, _, _, _) => {
                        h4768
                    }
                }, v4)))
            } else {
                Ok(Adt10::C1)
            }
        } else {
            let a4769 = v0;
            let a4770 = 62u64;
            if nat_eq(a4769, a4770) {
                let a4772 = match v1.clone() {
                    Adt1::C0(h4771, _, _, _, _, _, _, _, _, _, _, _) => {
                        h4771
                    }
                };
                let a4773 = 2147483648u64;
                if if if if if nat_lt(a4772, a4773) {
                    let a4775 = match v1.clone() {
                        Adt1::C0(_, h4774, _, _, _, _, _, _, _, _, _, _) => {
                            h4774
                        }
                    };
                    let a4776 = 2147483648u64;
                    nat_lt(a4775, a4776)
                } else {
                    false
                } {
                    let a4779 = match v1.clone() {
                        Adt1::C0(h4777, _, _, _, _, _, _, _, _, _, _, _) => {
                            h4777
                        }
                    };
                    let a4780 = match v2.clone() {
                        Adt2::C0(_, h4778, _, _, _, _, _, _, _, _, _) => {
                            h4778
                        }
                    };
                    nat_lt(a4779, a4780)
                } else {
                    false
                } {
                    let a4783 = match v1.clone() {
                        Adt1::C0(_, h4781, _, _, _, _, _, _, _, _, _, _) => {
                            h4781
                        }
                    };
                    let a4784 = match v2.clone() {
                        Adt2::C0(_, h4782, _, _, _, _, _, _, _, _, _) => {
                            h4782
                        }
                    };
                    nat_lt(a4783, a4784)
                } else {
                    false
                } {
                    let a4787 = match v2.clone() {
                        Adt2::C0(_, _, h4785, _, _, _, _, _, _, _, _) => {
                            h4785
                        }
                    };
                    let a4788 = match v3.clone() {
                        Adt2::C0(_, _, h4786, _, _, _, _, _, _, _, _) => {
                            h4786
                        }
                    };
                    if if if nat_eq(a4787, a4788) {
                        let a4791 = match v2.clone() {
                            Adt2::C0(_, _, _, h4789, _, _, _, _, _, _, _) => {
                                h4789
                            }
                        };
                        let a4792 = match v3.clone() {
                            Adt2::C0(_, _, _, h4790, _, _, _, _, _, _, _) => {
                                h4790
                            }
                        };
                        nat_eq(a4791, a4792)
                    } else {
                        false
                    } {
                        let a4795 = match v2.clone() {
                            Adt2::C0(_, _, _, _, h4793, _, _, _, _, _, _) => {
                                h4793
                            }
                        };
                        let a4796 = match v3.clone() {
                            Adt2::C0(_, _, _, _, h4794, _, _, _, _, _, _) => {
                                h4794
                            }
                        };
                        nat_eq(a4795, a4796)
                    } else {
                        false
                    } {
                        let a4805 = match v3.clone() {
                            Adt2::C0(_, h4797, _, _, _, _, _, _, _, _, _) => {
                                h4797
                            }
                        };
                        let a4806 = {
                            let a4803 = {
                                let a4800 = match v2.clone() {
                                    Adt2::C0(_, h4798, _, _, _, _, _, _, _, _, _) => {
                                        h4798
                                    }
                                };
                                let a4801 = match v1.clone() {
                                    Adt1::C0(h4799, _, _, _, _, _, _, _, _, _, _, _) => {
                                        h4799
                                    }
                                };
                                nat_add(a4800, a4801)?
                            };
                            let a4804 = match v1.clone() {
                                Adt1::C0(_, h4802, _, _, _, _, _, _, _, _, _, _) => {
                                    h4802
                                }
                            };
                            nat_add(a4803, a4804)?
                        };
                        nat_eq(a4805, a4806)
                    } else {
                        false
                    }
                } else {
                    false
                } {
                    Ok(Adt10::C4(f25(v3.clone(), 1u64)?, 4u64, f28(v2, 1u64), f28(v3, 1u64), Adt4::C4(match v1 {
                        Adt1::C0(h4807, _, _, _, _, _, _, _, _, _, _, _) => {
                            h4807
                        }
                    })))
                } else {
                    Ok(Adt10::C1)
                }
            } else {
                let a4808 = v0;
                let a4809 = 63u64;
                if nat_eq(a4808, a4809) {
                    if if if if if f249(match v2.clone() {
                        Adt2::C0(_, h4810, _, _, _, _, _, _, _, _, _) => {
                            h4810
                        }
                    }, match v1.clone() {
                        Adt1::C0(h4811, _, _, _, _, _, _, _, _, _, _, _) => {
                            h4811
                        }
                    })? {
                        f249(match v2.clone() {
                            Adt2::C0(_, _, h4812, _, _, _, _, _, _, _, _) => {
                                h4812
                            }
                        }, match v1.clone() {
                            Adt1::C0(_, h4813, _, _, _, _, _, _, _, _, _, _) => {
                                h4813
                            }
                        })?
                    } else {
                        false
                    } {
                        f249(match v2.clone() {
                            Adt2::C0(_, _, _, h4814, _, _, _, _, _, _, _) => {
                                h4814
                            }
                        }, match v1.clone() {
                            Adt1::C0(_, _, h4815, _, _, _, _, _, _, _, _, _) => {
                                h4815
                            }
                        })?
                    } else {
                        false
                    } {
                        f249(match v2.clone() {
                            Adt2::C0(_, _, _, _, h4816, _, _, _, _, _, _) => {
                                h4816
                            }
                        }, match v1.clone() {
                            Adt1::C0(_, _, _, h4817, _, _, _, _, _, _, _, _) => {
                                h4817
                            }
                        })?
                    } else {
                        false
                    } {
                        let a4820 = match v2.clone() {
                            Adt2::C0(_, h4818, _, _, _, _, _, _, _, _, _) => {
                                h4818
                            }
                        };
                        let a4821 = match v3.clone() {
                            Adt2::C0(_, h4819, _, _, _, _, _, _, _, _, _) => {
                                h4819
                            }
                        };
                        if if if nat_eq(a4820, a4821) {
                            let a4824 = match v2.clone() {
                                Adt2::C0(_, _, h4822, _, _, _, _, _, _, _, _) => {
                                    h4822
                                }
                            };
                            let a4825 = match v3.clone() {
                                Adt2::C0(_, _, h4823, _, _, _, _, _, _, _, _) => {
                                    h4823
                                }
                            };
                            nat_eq(a4824, a4825)
                        } else {
                            false
                        } {
                            let a4828 = match v2.clone() {
                                Adt2::C0(_, _, _, h4826, _, _, _, _, _, _, _) => {
                                    h4826
                                }
                            };
                            let a4829 = match v3.clone() {
                                Adt2::C0(_, _, _, h4827, _, _, _, _, _, _, _) => {
                                    h4827
                                }
                            };
                            nat_eq(a4828, a4829)
                        } else {
                            false
                        } {
                            let a4832 = match v2.clone() {
                                Adt2::C0(_, _, _, _, h4830, _, _, _, _, _, _) => {
                                    h4830
                                }
                            };
                            let a4833 = match v3.clone() {
                                Adt2::C0(_, _, _, _, h4831, _, _, _, _, _, _) => {
                                    h4831
                                }
                            };
                            nat_eq(a4832, a4833)
                        } else {
                            false
                        }
                    } else {
                        false
                    } {
                        Ok(Adt10::C4(f25(v3.clone(), 1u64)?, 4u64, f28(v2, 1u64), f28(v3, 1u64), Adt4::C5(match v1.clone() {
                            Adt1::C0(h4834, _, _, _, _, _, _, _, _, _, _, _) => {
                                h4834
                            }
                        }, match v1.clone() {
                            Adt1::C0(_, h4835, _, _, _, _, _, _, _, _, _, _) => {
                                h4835
                            }
                        }, match v1.clone() {
                            Adt1::C0(_, _, h4836, _, _, _, _, _, _, _, _, _) => {
                                h4836
                            }
                        }, match v1 {
                            Adt1::C0(_, _, _, h4837, _, _, _, _, _, _, _, _) => {
                                h4837
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

pub fn f145(v0: u64, v1: Adt1, v2: Adt2, v3: Option<Adt2>, v4: Adt2) -> R<Adt10> {
    let m4838 = v3;
    match m4838 {
        None => {
            Ok(Adt10::C1)
        }
        Some(v5) => {
            let v6: bool = {
                let a4839 = v0;
                let a4840 = 4u64;
                nat_eq(a4839, a4840)
            };
            let v7: bool = {
                let a4842 = 0u64;
                let a4843 = match v1.clone() {
                    Adt1::C0(_, _, _, _, h4841, _, _, _, _, _, _, _) => {
                        h4841
                    }
                };
                nat_lt(a4842, a4843)
            };
            let v8: u64 = f25(v5.clone(), 1u64)?;
            let v9: Adt3 = Adt3::C0(match v5.clone() {
                Adt2::C0(_, h4844, _, _, _, _, _, _, _, _, _) => {
                    h4844
                }
            }, match v5.clone() {
                Adt2::C0(_, _, h4845, _, _, _, _, _, _, _, _) => {
                    h4845
                }
            }, match v5.clone() {
                Adt2::C0(_, _, _, h4846, _, _, _, _, _, _, _) => {
                    h4846
                }
            }, match v5.clone() {
                Adt2::C0(_, _, _, _, h4847, _, _, _, _, _, _) => {
                    h4847
                }
            }, 4u64, match v1.clone() {
                Adt1::C0(h4848, _, _, _, _, _, _, _, _, _, _, _) => {
                    h4848
                }
            }, match v1.clone() {
                Adt1::C0(_, h4849, _, _, _, _, _, _, _, _, _, _) => {
                    h4849
                }
            }, match v1.clone() {
                Adt1::C0(_, _, h4850, _, _, _, _, _, _, _, _, _) => {
                    h4850
                }
            });
            if if if if if if if if if if if if v6 {
                let a4852 = match v2.clone() {
                    Adt2::C0(h4851, _, _, _, _, _, _, _, _, _, _) => {
                        h4851
                    }
                };
                let a4853 = 0u64;
                if nat_eq(a4852, a4853) {
                    let a4855 = match v5.clone() {
                        Adt2::C0(h4854, _, _, _, _, _, _, _, _, _, _) => {
                            h4854
                        }
                    };
                    let a4856 = 0u64;
                    nat_eq(a4855, a4856)
                } else {
                    false
                }
            } else {
                let a4858 = match v2.clone() {
                    Adt2::C0(h4857, _, _, _, _, _, _, _, _, _, _) => {
                        h4857
                    }
                };
                let a4859 = 0u64;
                if if nat_eq(a4858, a4859) {
                    true
                } else {
                    let a4861 = match v2.clone() {
                        Adt2::C0(h4860, _, _, _, _, _, _, _, _, _, _) => {
                            h4860
                        }
                    };
                    let a4862 = 26u64;
                    nat_eq(a4861, a4862)
                } {
                    let a4864 = match v5.clone() {
                        Adt2::C0(h4863, _, _, _, _, _, _, _, _, _, _) => {
                            h4863
                        }
                    };
                    let a4865 = 0u64;
                    if nat_eq(a4864, a4865) {
                        true
                    } else {
                        let a4867 = match v5.clone() {
                            Adt2::C0(h4866, _, _, _, _, _, _, _, _, _, _) => {
                                h4866
                            }
                        };
                        let a4868 = 26u64;
                        nat_eq(a4867, a4868)
                    }
                } else {
                    false
                }
            } {
                let a4871 = match v2.clone() {
                    Adt2::C0(h4869, _, _, _, _, _, _, _, _, _, _) => {
                        h4869
                    }
                };
                let a4872 = match v4.clone() {
                    Adt2::C0(h4870, _, _, _, _, _, _, _, _, _, _) => {
                        h4870
                    }
                };
                nat_eq(a4871, a4872)
            } else {
                false
            } {
                f32(v2.clone(), v4.clone())
            } else {
                false
            } {
                f45(v2.clone())?
            } else {
                false
            } {
                f45(v4.clone())?
            } else {
                false
            } {
                let a4874 = match v5.clone() {
                    Adt2::C0(_, _, _, _, _, h4873, _, _, _, _, _) => {
                        h4873
                    }
                };
                let a4875 = 4u64;
                nat_eq(a4874, a4875)
            } else {
                false
            } {
                let a4876 = v8;
                let a4877 = f25(v2.clone(), 1u64)?;
                nat_le(a4876, a4877)
            } else {
                false
            } {
                let a4879 = match v1.clone() {
                    Adt1::C0(_, _, _, h4878, _, _, _, _, _, _, _, _) => {
                        h4878
                    }
                };
                let a4880 = 1073741824u64;
                nat_lt(a4879, a4880)
            } else {
                false
            } {
                let a4882 = match v1.clone() {
                    Adt1::C0(h4881, _, _, _, _, _, _, _, _, _, _, _) => {
                        h4881
                    }
                };
                let a4883 = 4294967295u64;
                if if if if nat_le(a4882, a4883) {
                    let a4885 = match v1.clone() {
                        Adt1::C0(_, h4884, _, _, _, _, _, _, _, _, _, _) => {
                            h4884
                        }
                    };
                    let a4886 = 4294967295u64;
                    nat_le(a4885, a4886)
                } else {
                    false
                } {
                    let a4888 = match v1.clone() {
                        Adt1::C0(_, _, h4887, _, _, _, _, _, _, _, _, _) => {
                            h4887
                        }
                    };
                    let a4889 = 4294967295u64;
                    nat_le(a4888, a4889)
                } else {
                    false
                } {
                    let a4891 = match v1.clone() {
                        Adt1::C0(_, _, _, h4890, _, _, _, _, _, _, _, _) => {
                            h4890
                        }
                    };
                    let a4892 = 4294967295u64;
                    nat_le(a4891, a4892)
                } else {
                    false
                } {
                    let a4894 = match v1.clone() {
                        Adt1::C0(_, _, _, _, h4893, _, _, _, _, _, _, _) => {
                            h4893
                        }
                    };
                    let a4895 = 4294967295u64;
                    nat_le(a4894, a4895)
                } else {
                    false
                }
            } else {
                false
            } {
                if v7 {
                    let a4898 = match v2.clone() {
                        Adt2::C0(_, _, _, _, _, _, _, _, _, _, h4896) => {
                            h4896
                        }
                    };
                    let a4899 = match v4.clone() {
                        Adt2::C0(_, _, _, _, _, _, _, _, _, _, h4897) => {
                            h4897
                        }
                    };
                    nat_eq(a4898, a4899)
                } else {
                    true
                }
            } else {
                false
            } {
                f250(v9.clone(), match v1.clone() {
                    Adt1::C0(_, _, _, h4900, _, _, _, _, _, _, _, _) => {
                        h4900
                    }
                }, match v4 {
                    Adt2::C0(_, _, _, _, _, _, _, _, _, h4901, _) => {
                        h4901
                    }
                }, v8, v6)?
            } else {
                false
            } {
                let v10: u64 = if v7 {
                    0u64
                } else {
                    f25(v2, 1u64)?
                };
                Ok(Adt10::C4({
                    let a4902 = v10;
                    let a4903 = v8;
                    nat_add(a4902, a4903)?
                }, 4u64, f28(v5, 1u64), v9, Adt4::C6(v10, match v1 {
                    Adt1::C0(_, _, _, h4904, _, _, _, _, _, _, _, _) => {
                        h4904
                    }
                }, v6)))
            } else {
                Ok(Adt10::C1)
            }
        }
    }
}

pub fn f146(v0: Adt2, v1: Adt2) -> R<Adt10> {
    let a4906 = match v0.clone() {
        Adt2::C0(h4905, _, _, _, _, _, _, _, _, _, _) => {
            h4905
        }
    };
    let a4907 = 0u64;
    if if if nat_eq(a4906, a4907) {
        let a4909 = match v1.clone() {
            Adt2::C0(h4908, _, _, _, _, _, _, _, _, _, _) => {
                h4908
            }
        };
        let a4910 = 0u64;
        nat_eq(a4909, a4910)
    } else {
        false
    } {
        f246(match v0.clone() {
            Adt2::C0(_, h4911, _, _, _, _, _, _, _, _, _) => {
                h4911
            }
        }, match v0.clone() {
            Adt2::C0(_, _, h4912, _, _, _, _, _, _, _, _) => {
                h4912
            }
        }, match v0.clone() {
            Adt2::C0(_, _, _, h4913, _, _, _, _, _, _, _) => {
                h4913
            }
        }, match v0.clone() {
            Adt2::C0(_, _, _, _, h4914, _, _, _, _, _, _) => {
                h4914
            }
        }, match v1.clone() {
            Adt2::C0(_, h4915, _, _, _, _, _, _, _, _, _) => {
                h4915
            }
        }, match v1.clone() {
            Adt2::C0(_, _, h4916, _, _, _, _, _, _, _, _) => {
                h4916
            }
        }, match v1.clone() {
            Adt2::C0(_, _, _, h4917, _, _, _, _, _, _, _) => {
                h4917
            }
        }, match v1.clone() {
            Adt2::C0(_, _, _, _, h4918, _, _, _, _, _, _) => {
                h4918
            }
        })
    } else {
        false
    } {
        let v2: u64 = f25(v1.clone(), 1u64)?;
        Ok(Adt10::C4({
            let a4919 = v2;
            let a4920 = f25(v0.clone(), 1u64)?;
            nat_add(a4919, a4920)?
        }, 4u64, f28(v0, 1u64), f28(v1, 1u64), Adt4::C7(v2)))
    } else {
        Ok(Adt10::C1)
    }
}

pub fn f147(v0: u64, v1: Adt2, v2: Adt2) -> R<Adt10> {
    let a4922 = match v1.clone() {
        Adt2::C0(h4921, _, _, _, _, _, _, _, _, _, _) => {
            h4921
        }
    };
    let a4923 = 0u64;
    if if if nat_eq(a4922, a4923) {
        true
    } else {
        let a4925 = match v1.clone() {
            Adt2::C0(h4924, _, _, _, _, _, _, _, _, _, _) => {
                h4924
            }
        };
        let a4926 = 1u64;
        nat_eq(a4925, a4926)
    } {
        true
    } else {
        let a4928 = match v1.clone() {
            Adt2::C0(h4927, _, _, _, _, _, _, _, _, _, _) => {
                h4927
            }
        };
        let a4929 = 30u64;
        nat_eq(a4928, a4929)
    } {
        let a4932 = match v1.clone() {
            Adt2::C0(h4930, _, _, _, _, _, _, _, _, _, _) => {
                h4930
            }
        };
        let a4933 = match v2.clone() {
            Adt2::C0(h4931, _, _, _, _, _, _, _, _, _, _) => {
                h4931
            }
        };
        if if if if if nat_eq(a4932, a4933) {
            let a4936 = match v1.clone() {
                Adt2::C0(_, h4934, _, _, _, _, _, _, _, _, _) => {
                    h4934
                }
            };
            let a4937 = match v2.clone() {
                Adt2::C0(_, h4935, _, _, _, _, _, _, _, _, _) => {
                    h4935
                }
            };
            nat_eq(a4936, a4937)
        } else {
            false
        } {
            let a4940 = match v1.clone() {
                Adt2::C0(_, _, _, h4938, _, _, _, _, _, _, _) => {
                    h4938
                }
            };
            let a4941 = match v2.clone() {
                Adt2::C0(_, _, _, h4939, _, _, _, _, _, _, _) => {
                    h4939
                }
            };
            nat_eq(a4940, a4941)
        } else {
            false
        } {
            let a4944 = match v1.clone() {
                Adt2::C0(_, _, _, _, h4942, _, _, _, _, _, _) => {
                    h4942
                }
            };
            let a4945 = match v2.clone() {
                Adt2::C0(_, _, _, _, h4943, _, _, _, _, _, _) => {
                    h4943
                }
            };
            nat_eq(a4944, a4945)
        } else {
            false
        } {
            let a4946 = v0;
            let a4947 = 4u64;
            if nat_eq(a4946, a4947) {
                let a4949 = match v1.clone() {
                    Adt2::C0(_, _, h4948, _, _, _, _, _, _, _, _) => {
                        h4948
                    }
                };
                let a4950 = 1u64;
                if nat_eq(a4949, a4950) {
                    let a4953 = match v1.clone() {
                        Adt2::C0(_, h4951, _, _, _, _, _, _, _, _, _) => {
                            h4951
                        }
                    };
                    let a4954 = match v2.clone() {
                        Adt2::C0(_, _, h4952, _, _, _, _, _, _, _, _) => {
                            h4952
                        }
                    };
                    nat_eq(a4953, a4954)
                } else {
                    false
                }
            } else {
                let a4955 = v0;
                let a4956 = 4u64;
                if if nat_lt(a4955, a4956) {
                    let a4959 = match v1.clone() {
                        Adt2::C0(_, h4957, _, _, _, _, _, _, _, _, _) => {
                            h4957
                        }
                    };
                    let a4960 = match v1.clone() {
                        Adt2::C0(_, _, h4958, _, _, _, _, _, _, _, _) => {
                            h4958
                        }
                    };
                    nat_eq(a4959, a4960)
                } else {
                    false
                } {
                    let a4963 = match v1.clone() {
                        Adt2::C0(_, _, h4961, _, _, _, _, _, _, _, _) => {
                            h4961
                        }
                    };
                    let a4964 = match v2.clone() {
                        Adt2::C0(_, _, h4962, _, _, _, _, _, _, _, _) => {
                            h4962
                        }
                    };
                    nat_eq(a4963, a4964)
                } else {
                    false
                }
            }
        } else {
            false
        } {
            let m4966 = f235(match v1.clone() {
                Adt2::C0(h4965, _, _, _, _, _, _, _, _, _, _) => {
                    h4965
                }
            });
            match m4966 {
                Adt18::C0 => {
                    Ok(Adt10::C0)
                }
                Adt18::C1(v3) => {
                    Ok(Adt10::C4(f25(v2.clone(), 1u64)?, match v3 {
                        Adt19::C0(_, _, h4967) => {
                            h4967
                        }
                    }, f28(v1, 1u64), f28(v2, 1u64), Adt4::C8(v0)))
                }
            }
        } else {
            Ok(Adt10::C1)
        }
    } else {
        Ok(Adt10::C0)
    }
}

pub fn f148(v0: u64, v1: Adt2, v2: Adt2) -> R<Adt10> {
    if if if f251(match v1.clone() {
        Adt2::C0(h4968, _, _, _, _, _, _, _, _, _, _) => {
            h4968
        }
    }, v0) {
        let a4971 = match v1.clone() {
            Adt2::C0(h4969, _, _, _, _, _, _, _, _, _, _) => {
                h4969
            }
        };
        let a4972 = match v2.clone() {
            Adt2::C0(h4970, _, _, _, _, _, _, _, _, _, _) => {
                h4970
            }
        };
        nat_eq(a4971, a4972)
    } else {
        false
    } {
        let a4975 = match v1.clone() {
            Adt2::C0(_, h4973, _, _, _, _, _, _, _, _, _) => {
                h4973
            }
        };
        let a4976 = match v2.clone() {
            Adt2::C0(_, h4974, _, _, _, _, _, _, _, _, _) => {
                h4974
            }
        };
        if if if nat_eq(a4975, a4976) {
            let a4979 = match v1.clone() {
                Adt2::C0(_, _, h4977, _, _, _, _, _, _, _, _) => {
                    h4977
                }
            };
            let a4980 = match v2.clone() {
                Adt2::C0(_, _, h4978, _, _, _, _, _, _, _, _) => {
                    h4978
                }
            };
            nat_eq(a4979, a4980)
        } else {
            false
        } {
            let a4983 = match v1.clone() {
                Adt2::C0(_, _, _, h4981, _, _, _, _, _, _, _) => {
                    h4981
                }
            };
            let a4984 = match v2.clone() {
                Adt2::C0(_, _, _, h4982, _, _, _, _, _, _, _) => {
                    h4982
                }
            };
            nat_eq(a4983, a4984)
        } else {
            false
        } {
            let a4987 = match v1.clone() {
                Adt2::C0(_, _, _, _, h4985, _, _, _, _, _, _) => {
                    h4985
                }
            };
            let a4988 = match v2.clone() {
                Adt2::C0(_, _, _, _, h4986, _, _, _, _, _, _) => {
                    h4986
                }
            };
            nat_eq(a4987, a4988)
        } else {
            false
        }
    } else {
        false
    } {
        let m4990 = f235(match v1.clone() {
            Adt2::C0(h4989, _, _, _, _, _, _, _, _, _, _) => {
                h4989
            }
        });
        match m4990 {
            Adt18::C0 => {
                Ok(Adt10::C0)
            }
            Adt18::C1(v3) => {
                Ok(Adt10::C4(f25(v2.clone(), 1u64)?, match v3 {
                    Adt19::C0(_, _, h4991) => {
                        h4991
                    }
                }, f28(v1.clone(), 1u64), f28(v2, 1u64), Adt4::C9(match v1 {
                    Adt2::C0(h4992, _, _, _, _, _, _, _, _, _, _) => {
                        h4992
                    }
                }, v0)))
            }
        }
    } else {
        Ok(Adt10::C1)
    }
}

pub fn f149(v0: bool, v1: Adt1, v2: Adt2, v3: Adt2) -> R<Adt10> {
    let v4: u64 = if v0 {
        match v1.clone() {
            Adt1::C0(h4993, _, _, _, _, _, _, _, _, _, _, _) => {
                h4993
            }
        }
    } else {
        match v1.clone() {
            Adt1::C0(_, _, h4994, _, _, _, _, _, _, _, _, _) => {
                h4994
            }
        }
    };
    let a4995 = 0u64;
    let a4996 = v4;
    if if nat_lt(a4995, a4996) {
        let a4997 = v4;
        let a4998 = 2147483648u64;
        nat_lt(a4997, a4998)
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
        let v7: u64 = f252(match v5.clone() {
            Adt2::C0(_, _, h4999, _, _, _, _, _, _, _, _) => {
                h4999
            }
        }, v4)?;
        let v8: u64 = f252(match v5.clone() {
            Adt2::C0(_, _, _, h5000, _, _, _, _, _, _, _) => {
                h5000
            }
        }, v4)?;
        let a5002 = match v5.clone() {
            Adt2::C0(h5001, _, _, _, _, _, _, _, _, _, _) => {
                h5001
            }
        };
        let a5003 = 0u64;
        if if if if if nat_eq(a5002, a5003) {
            let a5005 = match v6.clone() {
                Adt2::C0(h5004, _, _, _, _, _, _, _, _, _, _) => {
                    h5004
                }
            };
            let a5006 = 0u64;
            nat_eq(a5005, a5006)
        } else {
            false
        } {
            let a5008 = match v5.clone() {
                Adt2::C0(_, _, _, _, h5007, _, _, _, _, _, _) => {
                    h5007
                }
            };
            let a5009 = 1u64;
            nat_eq(a5008, a5009)
        } else {
            false
        } {
            let a5012 = match v5.clone() {
                Adt2::C0(_, h5010, _, _, _, _, _, _, _, _, _) => {
                    h5010
                }
            };
            let a5013 = match v6.clone() {
                Adt2::C0(_, h5011, _, _, _, _, _, _, _, _, _) => {
                    h5011
                }
            };
            nat_eq(a5012, a5013)
        } else {
            false
        } {
            if v0 {
                let a5015 = match v5.clone() {
                    Adt2::C0(_, _, h5014, _, _, _, _, _, _, _, _) => {
                        h5014
                    }
                };
                let a5016 = 2147483648u64;
                if if if nat_lt(a5015, a5016) {
                    let a5018 = match v5.clone() {
                        Adt2::C0(_, _, _, h5017, _, _, _, _, _, _, _) => {
                            h5017
                        }
                    };
                    let a5019 = 2147483648u64;
                    nat_lt(a5018, a5019)
                } else {
                    false
                } {
                    let a5022 = {
                        let a5020 = v7;
                        let a5021 = v8;
                        nat_mul(a5020, a5021)?
                    };
                    let a5023 = 2147483648u64;
                    nat_le(a5022, a5023)
                } else {
                    false
                } {
                    f253(v5, v6, v4, v7)?
                } else {
                    false
                }
            } else {
                let a5025 = match v1.clone() {
                    Adt1::C0(h5024, _, _, _, _, _, _, _, _, _, _, _) => {
                        h5024
                    }
                };
                let a5026 = v7;
                if if if if nat_eq(a5025, a5026) {
                    let a5028 = match v1 {
                        Adt1::C0(_, h5027, _, _, _, _, _, _, _, _, _, _) => {
                            h5027
                        }
                    };
                    let a5029 = v8;
                    nat_eq(a5028, a5029)
                } else {
                    false
                } {
                    f254(v7, v8, v4)?
                } else {
                    false
                } {
                    let a5031 = match v6.clone() {
                        Adt2::C0(_, _, h5030, _, _, _, _, _, _, _, _) => {
                            h5030
                        }
                    };
                    let a5032 = v4;
                    if nat_eq(a5031, a5032) {
                        let a5034 = match v6.clone() {
                            Adt2::C0(_, _, _, h5033, _, _, _, _, _, _, _) => {
                                h5033
                            }
                        };
                        let a5035 = v4;
                        nat_eq(a5034, a5035)
                    } else {
                        false
                    }
                } else {
                    false
                } {
                    let a5039 = match v6 {
                        Adt2::C0(_, _, _, _, h5036, _, _, _, _, _, _) => {
                            h5036
                        }
                    };
                    let a5040 = {
                        let a5037 = v7;
                        let a5038 = v8;
                        nat_mul(a5037, a5038)?
                    };
                    nat_eq(a5039, a5040)
                } else {
                    false
                }
            }
        } else {
            false
        } {
            Ok(Adt10::C4(f25(v3.clone(), 1u64)?, 4u64, f28(v2, 1u64), f28(v3, 1u64), Adt4::C12(v0, v4, v7)))
        } else {
            Ok(Adt10::C1)
        }
    } else {
        Ok(Adt10::C1)
    }
}

pub fn f150(v0: u64, v1: Adt1, v2: Adt2, v3: Adt2) -> R<Adt10> {
    let a5042 = match v2.clone() {
        Adt2::C0(h5041, _, _, _, _, _, _, _, _, _, _) => {
            h5041
        }
    };
    let a5043 = 0u64;
    if if if if if nat_eq(a5042, a5043) {
        true
    } else {
        let a5045 = match v2.clone() {
            Adt2::C0(h5044, _, _, _, _, _, _, _, _, _, _) => {
                h5044
            }
        };
        let a5046 = 1u64;
        nat_eq(a5045, a5046)
    } {
        true
    } else {
        let a5048 = match v2.clone() {
            Adt2::C0(h5047, _, _, _, _, _, _, _, _, _, _) => {
                h5047
            }
        };
        let a5049 = 30u64;
        nat_eq(a5048, a5049)
    } {
        let a5052 = match v2.clone() {
            Adt2::C0(h5050, _, _, _, _, _, _, _, _, _, _) => {
                h5050
            }
        };
        let a5053 = match v3.clone() {
            Adt2::C0(h5051, _, _, _, _, _, _, _, _, _, _) => {
                h5051
            }
        };
        nat_eq(a5052, a5053)
    } else {
        false
    } {
        if if f32(v2.clone(), v3.clone()) {
            let a5055 = match v1.clone() {
                Adt1::C0(h5054, _, _, _, _, _, _, _, _, _, _, _) => {
                    h5054
                }
            };
            let a5056 = 4294967296u64;
            nat_lt(a5055, a5056)
        } else {
            false
        } {
            let a5058 = match v1.clone() {
                Adt1::C0(_, h5057, _, _, _, _, _, _, _, _, _, _) => {
                    h5057
                }
            };
            let a5059 = 4294967296u64;
            nat_lt(a5058, a5059)
        } else {
            false
        }
    } else {
        false
    } {
        Ok(Adt10::C4(f25(v3.clone(), 1u64)?, {
            let a5061 = match v2.clone() {
                Adt2::C0(h5060, _, _, _, _, _, _, _, _, _, _) => {
                    h5060
                }
            };
            let a5062 = 0u64;
            if nat_eq(a5061, a5062) {
                4u64
            } else {
                2u64
            }
        }, f28(v2.clone(), 1u64), f28(v3, 1u64), Adt4::C13({
            let a5063 = v0;
            let a5064 = 31u64;
            if nat_eq(a5063, a5064) {
                0u64
            } else {
                let a5065 = v0;
                let a5066 = 47u64;
                if nat_eq(a5065, a5066) {
                    1u64
                } else {
                    2u64
                }
            }
        }, match v2 {
            Adt2::C0(h5067, _, _, _, _, _, _, _, _, _, _) => {
                h5067
            }
        }, match v1.clone() {
            Adt1::C0(h5068, _, _, _, _, _, _, _, _, _, _, _) => {
                h5068
            }
        }, match v1 {
            Adt1::C0(_, h5069, _, _, _, _, _, _, _, _, _, _) => {
                h5069
            }
        })))
    } else {
        Ok(Adt10::C1)
    }
}

pub fn f151(v0: Adt2, v1: Adt2) -> R<Adt10> {
    let a5071 = match v0.clone() {
        Adt2::C0(h5070, _, _, _, _, _, _, _, _, _, _) => {
            h5070
        }
    };
    let a5072 = 0u64;
    if if if nat_eq(a5071, a5072) {
        let a5074 = match v1.clone() {
            Adt2::C0(h5073, _, _, _, _, _, _, _, _, _, _) => {
                h5073
            }
        };
        let a5075 = 0u64;
        nat_eq(a5074, a5075)
    } else {
        false
    } {
        f32(v0.clone(), v1.clone())
    } else {
        false
    } {
        Ok(Adt10::C4(f25(v1.clone(), 1u64)?, 4u64, f28(v0, 1u64), f28(v1, 1u64), Adt4::C15))
    } else {
        Ok(Adt10::C1)
    }
}

pub fn f152(v0: Adt2, v1: Adt2) -> R<Adt10> {
    let a5077 = match v0.clone() {
        Adt2::C0(h5076, _, _, _, _, _, _, _, _, _, _) => {
            h5076
        }
    };
    let a5078 = 0u64;
    if if nat_eq(a5077, a5078) {
        let a5080 = match v1.clone() {
            Adt2::C0(h5079, _, _, _, _, _, _, _, _, _, _) => {
                h5079
            }
        };
        let a5081 = 26u64;
        nat_eq(a5080, a5081)
    } else {
        false
    } {
        let a5083 = match v0.clone() {
            Adt2::C0(_, _, _, h5082, _, _, _, _, _, _, _) => {
                h5082
            }
        };
        let a5084 = 1u64;
        if if if if if if if nat_eq(a5083, a5084) {
            let a5086 = match v0.clone() {
                Adt2::C0(_, _, _, _, h5085, _, _, _, _, _, _) => {
                    h5085
                }
            };
            let a5087 = 1u64;
            nat_eq(a5086, a5087)
        } else {
            false
        } {
            let a5089 = match v0.clone() {
                Adt2::C0(_, h5088, _, _, _, _, _, _, _, _, _) => {
                    h5088
                }
            };
            let a5090 = 2147483647u64;
            nat_le(a5089, a5090)
        } else {
            false
        } {
            let a5093 = match v1.clone() {
                Adt2::C0(_, h5091, _, _, _, _, _, _, _, _, _) => {
                    h5091
                }
            };
            let a5094 = match v0.clone() {
                Adt2::C0(_, _, h5092, _, _, _, _, _, _, _, _) => {
                    h5092
                }
            };
            nat_eq(a5093, a5094)
        } else {
            false
        } {
            let a5096 = match v1.clone() {
                Adt2::C0(_, _, h5095, _, _, _, _, _, _, _, _) => {
                    h5095
                }
            };
            let a5097 = 1u64;
            nat_eq(a5096, a5097)
        } else {
            false
        } {
            let a5099 = match v1.clone() {
                Adt2::C0(_, _, _, h5098, _, _, _, _, _, _, _) => {
                    h5098
                }
            };
            let a5100 = 1u64;
            nat_eq(a5099, a5100)
        } else {
            false
        } {
            let a5102 = match v1.clone() {
                Adt2::C0(_, _, _, _, h5101, _, _, _, _, _, _) => {
                    h5101
                }
            };
            let a5103 = 1u64;
            nat_eq(a5102, a5103)
        } else {
            false
        } {
            Ok(Adt10::C4({
                let a5106 = match v0.clone() {
                    Adt2::C0(_, _, h5104, _, _, _, _, _, _, _, _) => {
                        h5104
                    }
                };
                let a5107 = f196(match v0.clone() {
                    Adt2::C0(_, h5105, _, _, _, _, _, _, _, _, _) => {
                        h5105
                    }
                })?;
                nat_mul(a5106, a5107)?
            }, 4u64, f28(v0, 1u64), f28(v1, 1u64), Adt4::C17))
        } else {
            Ok(Adt10::C1)
        }
    } else {
        Ok(Adt10::C0)
    }
}

pub fn f153(v0: Adt2, v1: Option<Adt2>, v2: Adt2) -> R<Adt10> {
    let m5108 = v1;
    match m5108 {
        None => {
            Ok(Adt10::C1)
        }
        Some(v3) => {
            let v4: u64 = f201(f28(v0.clone(), 1u64))?;
            let a5110 = match v0.clone() {
                Adt2::C0(h5109, _, _, _, _, _, _, _, _, _, _) => {
                    h5109
                }
            };
            let a5111 = 26u64;
            if if if nat_eq(a5110, a5111) {
                let a5113 = match v3.clone() {
                    Adt2::C0(h5112, _, _, _, _, _, _, _, _, _, _) => {
                        h5112
                    }
                };
                let a5114 = 26u64;
                nat_eq(a5113, a5114)
            } else {
                false
            } {
                let a5116 = match v2.clone() {
                    Adt2::C0(h5115, _, _, _, _, _, _, _, _, _, _) => {
                        h5115
                    }
                };
                let a5117 = 27u64;
                nat_eq(a5116, a5117)
            } else {
                false
            } {
                if if if if if if f32(v0.clone(), v3.clone()) {
                    let a5119 = match v2.clone() {
                        Adt2::C0(_, h5118, _, _, _, _, _, _, _, _, _) => {
                            h5118
                        }
                    };
                    let a5120 = 1u64;
                    nat_eq(a5119, a5120)
                } else {
                    false
                } {
                    let a5122 = match v2.clone() {
                        Adt2::C0(_, _, h5121, _, _, _, _, _, _, _, _) => {
                            h5121
                        }
                    };
                    let a5123 = 1u64;
                    nat_eq(a5122, a5123)
                } else {
                    false
                } {
                    let a5125 = match v2.clone() {
                        Adt2::C0(_, _, _, h5124, _, _, _, _, _, _, _) => {
                            h5124
                        }
                    };
                    let a5126 = 1u64;
                    nat_eq(a5125, a5126)
                } else {
                    false
                } {
                    let a5128 = match v2.clone() {
                        Adt2::C0(_, _, _, _, h5127, _, _, _, _, _, _) => {
                            h5127
                        }
                    };
                    let a5129 = 1u64;
                    nat_eq(a5128, a5129)
                } else {
                    false
                } {
                    let a5130 = v4;
                    let a5131 = 9223372036854775808u64;
                    nat_lt(a5130, a5131)
                } else {
                    false
                } {
                    Ok(Adt10::C4({
                        let a5134 = 2u64;
                        let a5135 = {
                            let a5132 = v4;
                            let a5133 = 2u64;
                            nat_mul(a5132, a5133)?
                        };
                        nat_add(a5134, a5135)?
                    }, 8u64, f28(v0, 1u64), f28(v2, 1u64), Adt4::C18(v4, f28(v3, 1u64))))
                } else {
                    Ok(Adt10::C1)
                }
            } else {
                Ok(Adt10::C0)
            }
        }
    }
}

pub fn f154(v0: u64, v1: Adt2, v2: Adt2) -> R<Adt10> {
    let v3: u64 = match v1.clone() {
        Adt2::C0(h5136, _, _, _, _, _, _, _, _, _, _) => {
            h5136
        }
    };
    let a5137 = v3;
    let a5138 = 0u64;
    if if nat_eq(a5137, a5138) {
        true
    } else {
        let a5139 = v0;
        let a5140 = 13u64;
        if nat_eq(a5139, a5140) {
            let a5141 = v3;
            let a5142 = 1u64;
            if nat_eq(a5141, a5142) {
                true
            } else {
                let a5143 = v3;
                let a5144 = 30u64;
                nat_eq(a5143, a5144)
            }
        } else {
            false
        }
    } {
        let a5146 = v3;
        let a5147 = match v2.clone() {
            Adt2::C0(h5145, _, _, _, _, _, _, _, _, _, _) => {
                h5145
            }
        };
        if if nat_eq(a5146, a5147) {
            let a5148 = v0;
            let a5149 = 13u64;
            if nat_eq(a5148, a5149) {
                let a5151 = match v2.clone() {
                    Adt2::C0(_, h5150, _, _, _, _, _, _, _, _, _) => {
                        h5150
                    }
                };
                let a5152 = 1u64;
                if if if nat_eq(a5151, a5152) {
                    let a5154 = match v2.clone() {
                        Adt2::C0(_, _, h5153, _, _, _, _, _, _, _, _) => {
                            h5153
                        }
                    };
                    let a5155 = 1u64;
                    nat_eq(a5154, a5155)
                } else {
                    false
                } {
                    let a5157 = match v2.clone() {
                        Adt2::C0(_, _, _, h5156, _, _, _, _, _, _, _) => {
                            h5156
                        }
                    };
                    let a5158 = 1u64;
                    nat_eq(a5157, a5158)
                } else {
                    false
                } {
                    let a5160 = match v2.clone() {
                        Adt2::C0(_, _, _, _, h5159, _, _, _, _, _, _) => {
                            h5159
                        }
                    };
                    let a5161 = 1u64;
                    nat_eq(a5160, a5161)
                } else {
                    false
                }
            } else {
                let a5163 = match v2.clone() {
                    Adt2::C0(_, h5162, _, _, _, _, _, _, _, _, _) => {
                        h5162
                    }
                };
                let a5164 = 1u64;
                if if if nat_eq(a5163, a5164) {
                    let a5167 = match v1.clone() {
                        Adt2::C0(_, _, h5165, _, _, _, _, _, _, _, _) => {
                            h5165
                        }
                    };
                    let a5168 = match v2.clone() {
                        Adt2::C0(_, _, h5166, _, _, _, _, _, _, _, _) => {
                            h5166
                        }
                    };
                    nat_eq(a5167, a5168)
                } else {
                    false
                } {
                    let a5171 = match v1.clone() {
                        Adt2::C0(_, _, _, h5169, _, _, _, _, _, _, _) => {
                            h5169
                        }
                    };
                    let a5172 = match v2.clone() {
                        Adt2::C0(_, _, _, h5170, _, _, _, _, _, _, _) => {
                            h5170
                        }
                    };
                    nat_eq(a5171, a5172)
                } else {
                    false
                } {
                    let a5175 = match v1.clone() {
                        Adt2::C0(_, _, _, _, h5173, _, _, _, _, _, _) => {
                            h5173
                        }
                    };
                    let a5176 = match v2.clone() {
                        Adt2::C0(_, _, _, _, h5174, _, _, _, _, _, _) => {
                            h5174
                        }
                    };
                    nat_eq(a5175, a5176)
                } else {
                    false
                }
            }
        } else {
            false
        } {
            let v4: Adt3 = f28(v1, 1u64);
            Ok(Adt10::C4(f255(v4.clone(), v0, v3)?, {
                let a5177 = v3;
                let a5178 = 0u64;
                if nat_eq(a5177, a5178) {
                    4u64
                } else {
                    2u64
                }
            }, v4, f28(v2, 1u64), Adt4::C16(v0, v3)))
        } else {
            Ok(Adt10::C1)
        }
    } else {
        Ok(Adt10::C0)
    }
}

pub fn f155(v0: u64, v1: u64) -> R<Option<u64>> {
    let a5179 = v0;
    let a5180 = 89u64;
    if if nat_eq(a5179, a5180) {
        let a5181 = v1;
        let a5182 = 5u64;
        nat_eq(a5181, a5182)
    } else {
        false
    } {
        Ok(Some(25u64))
    } else {
        let a5183 = v0;
        let a5184 = 89u64;
        if if nat_eq(a5183, a5184) {
            let a5185 = v1;
            let a5186 = 17u64;
            nat_eq(a5185, a5186)
        } else {
            false
        } {
            Ok(Some(26u64))
        } else {
            let a5187 = v0;
            let a5188 = 89u64;
            if if nat_eq(a5187, a5188) {
                let a5189 = v1;
                let a5190 = 16u64;
                nat_eq(a5189, a5190)
            } else {
                false
            } {
                Ok(Some(24u64))
            } else {
                let a5191 = v0;
                let a5192 = 89u64;
                if if nat_eq(a5191, a5192) {
                    let a5193 = v1;
                    let a5194 = 14u64;
                    nat_eq(a5193, a5194)
                } else {
                    false
                } {
                    Ok(Some(23u64))
                } else {
                    let a5195 = v0;
                    let a5196 = 89u64;
                    if if nat_eq(a5195, a5196) {
                        let a5197 = v1;
                        let a5198 = 15u64;
                        nat_eq(a5197, a5198)
                    } else {
                        false
                    } {
                        Ok(Some(22u64))
                    } else {
                        let a5199 = v0;
                        let a5200 = 89u64;
                        if if nat_eq(a5199, a5200) {
                            let a5201 = 8u64;
                            let a5202 = v1;
                            if nat_le(a5201, a5202) {
                                let a5203 = v1;
                                let a5204 = 9u64;
                                nat_le(a5203, a5204)
                            } else {
                                false
                            }
                        } else {
                            false
                        } {
                            Ok(Some({
                                let a5205 = v1;
                                let a5206 = 12u64;
                                nat_add(a5205, a5206)?
                            }))
                        } else {
                            let a5207 = v0;
                            let a5208 = 89u64;
                            if if nat_eq(a5207, a5208) {
                                let a5209 = v1;
                                let a5210 = 4u64;
                                nat_eq(a5209, a5210)
                            } else {
                                false
                            } {
                                Ok(Some(19u64))
                            } else {
                                let a5211 = 10u64;
                                let a5212 = v0;
                                if if nat_le(a5211, a5212) {
                                    let a5213 = v0;
                                    let a5214 = 12u64;
                                    nat_le(a5213, a5214)
                                } else {
                                    false
                                } {
                                    Ok(Some({
                                        let a5215 = v0;
                                        let a5216 = 3u64;
                                        nat_add(a5215, a5216)?
                                    }))
                                } else {
                                    let a5217 = v0;
                                    let a5218 = 89u64;
                                    if if nat_eq(a5217, a5218) {
                                        let a5219 = v1;
                                        let a5220 = 7u64;
                                        nat_eq(a5219, a5220)
                                    } else {
                                        false
                                    } {
                                        Ok(Some(17u64))
                                    } else {
                                        let a5221 = v0;
                                        let a5222 = 89u64;
                                        if if nat_eq(a5221, a5222) {
                                            let a5223 = v1;
                                            let a5224 = 10u64;
                                            nat_eq(a5223, a5224)
                                        } else {
                                            false
                                        } {
                                            Ok(Some(18u64))
                                        } else {
                                            let a5225 = v0;
                                            let a5226 = 89u64;
                                            if if nat_eq(a5225, a5226) {
                                                let a5227 = v1;
                                                let a5228 = 13u64;
                                                nat_eq(a5227, a5228)
                                            } else {
                                                false
                                            } {
                                                Ok(Some(16u64))
                                            } else {
                                                let a5229 = v0;
                                                let a5230 = 8u64;
                                                if nat_eq(a5229, a5230) {
                                                    Ok(Some(5u64))
                                                } else {
                                                    let a5231 = v0;
                                                    let a5232 = 9u64;
                                                    if nat_eq(a5231, a5232) {
                                                        Ok(Some(6u64))
                                                    } else {
                                                        let a5233 = v0;
                                                        let a5234 = 89u64;
                                                        if nat_eq(a5233, a5234) {
                                                            let a5235 = v1;
                                                            let a5236 = 0u64;
                                                            if nat_eq(a5235, a5236) {
                                                                Ok(Some(0u64))
                                                            } else {
                                                                let a5237 = v1;
                                                                let a5238 = 1u64;
                                                                if nat_eq(a5237, a5238) {
                                                                    Ok(Some(1u64))
                                                                } else {
                                                                    let a5239 = v1;
                                                                    let a5240 = 2u64;
                                                                    if nat_eq(a5239, a5240) {
                                                                        Ok(Some(2u64))
                                                                    } else {
                                                                        let a5241 = v1;
                                                                        let a5242 = 3u64;
                                                                        if nat_eq(a5241, a5242) {
                                                                            Ok(Some(3u64))
                                                                        } else {
                                                                            let a5243 = v1;
                                                                            let a5244 = 6u64;
                                                                            if nat_eq(a5243, a5244) {
                                                                                Ok(Some(4u64))
                                                                            } else {
                                                                                let a5245 = v1;
                                                                                let a5246 = 12u64;
                                                                                if nat_eq(a5245, a5246) {
                                                                                    Ok(Some(7u64))
                                                                                } else {
                                                                                    let a5247 = v1;
                                                                                    let a5248 = 11u64;
                                                                                    if nat_eq(a5247, a5248) {
                                                                                        Ok(Some(8u64))
                                                                                    } else {
                                                                                        let a5249 = v1;
                                                                                        let a5250 = 18u64;
                                                                                        if nat_eq(a5249, a5250) {
                                                                                            Ok(Some(9u64))
                                                                                        } else {
                                                                                            let a5251 = v1;
                                                                                            let a5252 = 19u64;
                                                                                            if nat_eq(a5251, a5252) {
                                                                                                Ok(Some(10u64))
                                                                                            } else {
                                                                                                let a5253 = v1;
                                                                                                let a5254 = 20u64;
                                                                                                if nat_eq(a5253, a5254) {
                                                                                                    Ok(Some(11u64))
                                                                                                } else {
                                                                                                    let a5255 = v1;
                                                                                                    let a5256 = 21u64;
                                                                                                    if nat_eq(a5255, a5256) {
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

pub fn f156(v0: u64, v1: u64, v2: Adt2, v3: Option<Adt2>, v4: Adt2) -> R<Adt10> {
    let m5257 = f256(v0, v1, v2.clone(), v3.clone(), v4.clone())?;
    match m5257 {
        Adt10::C0 => {
            Ok(Adt10::C0)
        }
        Adt10::C1 => {
            let a5258 = v0;
            let a5259 = 2u64;
            if if if if nat_eq(a5258, a5259) {
                true
            } else {
                let a5260 = v0;
                let a5261 = 5u64;
                nat_eq(a5260, a5261)
            } {
                true
            } else {
                let a5262 = v0;
                let a5263 = 6u64;
                nat_eq(a5262, a5263)
            } {
                true
            } else {
                let a5264 = v0;
                let a5265 = 7u64;
                nat_eq(a5264, a5265)
            } {
                Ok(Adt10::C5(67u64))
            } else {
                Ok(Adt10::C1)
            }
        }
        Adt10::C2(v7, v8, _, _, _, _, v13) => {
            if v8 {
                Ok(Adt10::C4(v13, 4u64, f28(v2.clone(), 1u64), f28(v4, 1u64), Adt4::C10(v7, v8, {
                    let m5266 = v3;
                    match m5266 {
                        None => {
                            f28(v2, 1u64)
                        }
                        Some(v14) => {
                            f28(v14, 1u64)
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

pub fn f157(v0: u64, v1: Adt1, v2: Adt2, v3: Adt2) -> R<Adt10> {
    let a5268 = match v2.clone() {
        Adt2::C0(h5267, _, _, _, _, _, _, _, _, _, _) => {
            h5267
        }
    };
    let a5269 = 0u64;
    if if if if if nat_eq(a5268, a5269) {
        true
    } else {
        let a5271 = match v2.clone() {
            Adt2::C0(h5270, _, _, _, _, _, _, _, _, _, _) => {
                h5270
            }
        };
        let a5272 = 1u64;
        nat_eq(a5271, a5272)
    } {
        true
    } else {
        let a5274 = match v2.clone() {
            Adt2::C0(h5273, _, _, _, _, _, _, _, _, _, _) => {
                h5273
            }
        };
        let a5275 = 30u64;
        nat_eq(a5274, a5275)
    } {
        let a5278 = match v2.clone() {
            Adt2::C0(h5276, _, _, _, _, _, _, _, _, _, _) => {
                h5276
            }
        };
        let a5279 = match v3.clone() {
            Adt2::C0(h5277, _, _, _, _, _, _, _, _, _, _) => {
                h5277
            }
        };
        if nat_eq(a5278, a5279) {
            true
        } else {
            let a5281 = match v3.clone() {
                Adt2::C0(h5280, _, _, _, _, _, _, _, _, _, _) => {
                    h5280
                }
            };
            let a5282 = 0u64;
            nat_eq(a5281, a5282)
        }
    } else {
        false
    } {
        f32(v2.clone(), v3.clone())
    } else {
        false
    } {
        Ok(Adt10::C4({
            let a5289 = f25(v3.clone(), 1u64)?;
            let a5290 = {
                let a5283 = 25u64;
                let a5284 = v0;
                if nat_le(a5283, a5284) {
                    f192({
                        let a5285 = v0;
                        let a5286 = 26u64;
                        nat_eq(a5285, a5286)
                    }, match v3.clone() {
                        Adt2::C0(h5287, _, _, _, _, _, _, _, _, _, _) => {
                            h5287
                        }
                    })?
                } else {
                    f257(v0, match v3.clone() {
                        Adt2::C0(h5288, _, _, _, _, _, _, _, _, _, _) => {
                            h5288
                        }
                    })
                }
            };
            nat_mul(a5289, a5290)?
        }, {
            let a5292 = match v3.clone() {
                Adt2::C0(h5291, _, _, _, _, _, _, _, _, _, _) => {
                    h5291
                }
            };
            let a5293 = 0u64;
            if nat_eq(a5292, a5293) {
                4u64
            } else {
                2u64
            }
        }, f28(v2.clone(), 1u64), f28(v3.clone(), 1u64), {
            let a5294 = 25u64;
            let a5295 = v0;
            if nat_le(a5294, a5295) {
                Adt4::C45({
                    let a5296 = v0;
                    let a5297 = 26u64;
                    nat_eq(a5296, a5297)
                }, match v2 {
                    Adt2::C0(h5298, _, _, _, _, _, _, _, _, _, _) => {
                        h5298
                    }
                }, match v3 {
                    Adt2::C0(h5299, _, _, _, _, _, _, _, _, _, _) => {
                        h5299
                    }
                }, v1)
            } else {
                Adt4::C14(v0, match v2 {
                    Adt2::C0(h5300, _, _, _, _, _, _, _, _, _, _) => {
                        h5300
                    }
                }, match v3 {
                    Adt2::C0(h5301, _, _, _, _, _, _, _, _, _, _) => {
                        h5301
                    }
                })
            }
        }))
    } else {
        Ok(Adt10::C1)
    }
}

pub fn f158(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64, v7: u64) -> R<u64> {
    let a5314 = {
        let a5310 = {
            let a5306 = {
                let a5302 = v0;
                let a5303 = v4;
                nat_mul(a5302, a5303)?
            };
            let a5307 = {
                let a5304 = v1;
                let a5305 = v5;
                nat_mul(a5304, a5305)?
            };
            nat_add(a5306, a5307)?
        };
        let a5311 = {
            let a5308 = v2;
            let a5309 = v6;
            nat_mul(a5308, a5309)?
        };
        nat_add(a5310, a5311)?
    };
    let a5315 = {
        let a5312 = v3;
        let a5313 = v7;
        nat_mul(a5312, a5313)?
    };
    nat_add(a5314, a5315)
}

pub fn f159(v0: u64) -> u64 {
    let a5316 = v0;
    let a5317 = 2147483648u64;
    if nat_lt(a5316, a5317) {
        0u64
    } else {
        let a5318 = 4294967296u64;
        let a5319 = v0;
        nat_sub(a5318, a5319)
    }
}

pub fn f160(v0: u64) -> u64 {
    let a5320 = v0;
    let a5321 = 2147483648u64;
    if nat_lt(a5320, a5321) {
        v0
    } else {
        0u64
    }
}

pub fn f161(v0: Adt11, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64, v7: u64, v8: u64) -> Adt9 {
    let v9: u64 = {
        let a5322 = v7;
        let a5323 = 6u64;
        if nat_eq(a5322, a5323) {
            27u64
        } else {
            v2
        }
    };
    Adt9::C7(v0, f102(v3, v4), f102(v5, v6), f103(v7, v8), f193(v1), f193(v9))
}

pub fn f162(v0: Adt4, v1: u64, v2: u64) -> Adt11 {
    let m5324 = v0;
    match m5324 {
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
        Adt4::C13(v69, v70, v71, v72) => {
            Adt11::C4(v69, v70, v71, v72)
        }
        Adt4::C14(v73, _, _) => {
            let a5325 = 13u64;
            let a5326 = v73;
            if nat_le(a5325, a5326) {
                Adt11::C7(v73, v1, v2)
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
        Adt4::C27(v93) => {
            Adt11::C2(match v93.clone() {
                Adt5::C0(_, _, _, _, _, h5327, _, _) => {
                    h5327
                }
            }, match v93 {
                Adt5::C0(_, _, _, _, _, _, _, h5328) => {
                    h5328
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
        Adt4::C48(_, _, _) => {
            Adt11::C6
        }
    }
}

pub fn f163(v0: Adt4) -> u64 {
    let m5329 = v0;
    match m5329 {
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
        Adt4::C14(v71, _, _) => {
            v71
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
        Adt4::C48(_, _, _) => {
            0u64
        }
    }
}

pub fn f164(v0: Adt4, v1: u64) -> u64 {
    let m5330 = v0;
    match m5330 {
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
        Adt4::C16(_, v76) => {
            v76
        }
        Adt4::C17 => {
            0u64
        }
        Adt4::C18(_, _) => {
            0u64
        }
        Adt4::C19(v79, _) => {
            let a5331 = v1;
            let a5332 = 6u64;
            if nat_eq(a5331, a5332) {
                v79
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
        Adt4::C26(v93, _, _, _) => {
            let a5341 = v1;
            let a5342 = 6u64;
            if if if nat_eq(a5341, a5342) {
                true
            } else {
                let a5343 = v1;
                let a5344 = 7u64;
                nat_eq(a5343, a5344)
            } {
                let a5345 = v93;
                let a5346 = 30u64;
                nat_eq(a5345, a5346)
            } else {
                false
            } {
                30u64
            } else {
                0u64
            }
        }
        Adt4::C27(v92) => {
            match v92 {
                Adt5::C0(_, _, _, _, _, _, _, h5340) => {
                    h5340
                }
            }
        }
        Adt4::C28(v88) => {
            let a5333 = v1;
            let a5334 = 6u64;
            if nat_eq(a5333, a5334) {
                match v88 {
                    Adt7::C0(_, _, _, _, h5335) => {
                        h5335
                    }
                }
            } else {
                0u64
            }
        }
        Adt4::C29(_) => {
            0u64
        }
        Adt4::C30(_, v91) => {
            let a5336 = v1;
            let a5337 = 6u64;
            if if nat_eq(a5336, a5337) {
                true
            } else {
                let a5338 = v1;
                let a5339 = 7u64;
                nat_eq(a5338, a5339)
            } {
                v91
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
        Adt4::C48(_, _, _) => {
            0u64
        }
    }
}

pub fn f165(v0: Adt4, v1: u64) -> Adt11 {
    let a5347 = v1;
    let a5348 = 4u64;
    if nat_lt(a5347, a5348) {
        Adt11::C0(v1)
    } else {
        let a5349 = v1;
        let a5350 = 4u64;
        if nat_eq(a5349, a5350) {
            Adt11::C1(0u64)
        } else {
            let a5351 = v1;
            let a5352 = 5u64;
            if nat_eq(a5351, a5352) {
                Adt11::C1(3u64)
            } else {
                let a5353 = v1;
                let a5354 = 6u64;
                if nat_eq(a5353, a5354) {
                    Adt11::C2(f164(v0, v1), 0u64, false)
                } else {
                    let a5355 = v1;
                    let a5356 = 7u64;
                    if nat_eq(a5355, a5356) {
                        Adt11::C2(0u64, f164(v0, v1), false)
                    } else {
                        let a5357 = v1;
                        let a5358 = 16u64;
                        if nat_lt(a5357, a5358) {
                            Adt11::C1({
                                let a5359 = v1;
                                let a5360 = 8u64;
                                nat_sub(a5359, a5360)
                            })
                        } else {
                            let a5361 = v1;
                            let a5362 = 20u64;
                            if nat_lt(a5361, a5362) {
                                Adt11::C5({
                                    let a5363 = v1;
                                    let a5364 = 16u64;
                                    nat_sub(a5363, a5364)
                                })
                            } else {
                                let a5365 = v1;
                                let a5366 = 20u64;
                                if nat_eq(a5365, a5366) {
                                    Adt11::C3(0u64, 0u64, false)
                                } else {
                                    let a5367 = v1;
                                    let a5368 = 21u64;
                                    if nat_eq(a5367, a5368) {
                                        Adt11::C0(4u64)
                                    } else {
                                        let a5369 = v1;
                                        let a5370 = 22u64;
                                        if nat_eq(a5369, a5370) {
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

pub fn f166(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let a5388 = {
        let a5383 = f258(v2, {
            let a5374 = {
                let a5371 = v0;
                let a5372 = v1;
                let a5373 = 0u64;
                nat_quot(a5371, a5372, a5373)
            };
            let a5375 = v2;
            let a5376 = 0u64;
            nat_rem(a5374, a5375, a5376)
        }, {
            let a5380 = {
                let a5377 = v0;
                let a5378 = v1;
                let a5379 = 0u64;
                nat_quot(a5377, a5378, a5379)
            };
            let a5381 = v2;
            let a5382 = 0u64;
            nat_quot(a5380, a5381, a5382)
        })?;
        let a5384 = v1;
        nat_mul(a5383, a5384)?
    };
    let a5389 = {
        let a5385 = v0;
        let a5386 = v1;
        let a5387 = 0u64;
        nat_rem(a5385, a5386, a5387)
    };
    nat_add(a5388, a5389)
}

pub fn f167(v0: u64, v1: Adt3, v2: Adt3) -> R<u64> {
    let v3: u64 = {
        let a5391 = v0;
        let a5392 = match v2.clone() {
            Adt3::C0(h5390, _, _, _, _, _, _, _) => {
                h5390
            }
        };
        let a5393 = 0u64;
        nat_quot(a5391, a5392, a5393)
    };
    let v4: u64 = {
        let a5395 = v3;
        let a5396 = match v2.clone() {
            Adt3::C0(_, h5394, _, _, _, _, _, _) => {
                h5394
            }
        };
        let a5397 = 0u64;
        nat_quot(a5395, a5396, a5397)
    };
    f259({
        let a5399 = v0;
        let a5400 = match v2.clone() {
            Adt3::C0(h5398, _, _, _, _, _, _, _) => {
                h5398
            }
        };
        let a5401 = 0u64;
        nat_rem(a5399, a5400, a5401)
    }, {
        let a5403 = v3;
        let a5404 = match v2.clone() {
            Adt3::C0(_, h5402, _, _, _, _, _, _) => {
                h5402
            }
        };
        let a5405 = 0u64;
        nat_rem(a5403, a5404, a5405)
    }, {
        let a5407 = v4;
        let a5408 = match v2.clone() {
            Adt3::C0(_, _, h5406, _, _, _, _, _) => {
                h5406
            }
        };
        let a5409 = 0u64;
        nat_rem(a5407, a5408, a5409)
    }, {
        let a5411 = v4;
        let a5412 = match v2 {
            Adt3::C0(_, _, h5410, _, _, _, _, _) => {
                h5410
            }
        };
        let a5413 = 0u64;
        nat_quot(a5411, a5412, a5413)
    }, match v1.clone() {
        Adt3::C0(h5414, _, _, _, _, _, _, _) => {
            h5414
        }
    }, match v1.clone() {
        Adt3::C0(_, h5415, _, _, _, _, _, _) => {
            h5415
        }
    }, match v1.clone() {
        Adt3::C0(_, _, h5416, _, _, _, _, _) => {
            h5416
        }
    }, match v1.clone() {
        Adt3::C0(_, _, _, h5417, _, _, _, _) => {
            h5417
        }
    }, match v1.clone() {
        Adt3::C0(_, _, _, _, h5418, _, _, _) => {
            h5418
        }
    }, match v1.clone() {
        Adt3::C0(_, _, _, _, _, h5419, _, _) => {
            h5419
        }
    }, match v1.clone() {
        Adt3::C0(_, _, _, _, _, _, h5420, _) => {
            h5420
        }
    }, match v1 {
        Adt3::C0(_, _, _, _, _, _, _, h5421) => {
            h5421
        }
    })
}

pub fn f168(v0: u64, v1: Adt3, v2: Adt3, v3: Adt3, v4: u64) -> R<Adt9> {
    let v5: u64 = {
        let a5423 = v0;
        let a5424 = match v3.clone() {
            Adt3::C0(h5422, _, _, _, _, _, _, _) => {
                h5422
            }
        };
        let a5425 = 0u64;
        nat_quot(a5423, a5424, a5425)
    };
    let v6: u64 = {
        let a5427 = v5;
        let a5428 = match v3.clone() {
            Adt3::C0(_, h5426, _, _, _, _, _, _) => {
                h5426
            }
        };
        let a5429 = 0u64;
        nat_quot(a5427, a5428, a5429)
    };
    let v7: u64 = {
        let a5431 = v0;
        let a5432 = match v3.clone() {
            Adt3::C0(h5430, _, _, _, _, _, _, _) => {
                h5430
            }
        };
        let a5433 = 0u64;
        nat_rem(a5431, a5432, a5433)
    };
    let v8: u64 = {
        let a5435 = v5;
        let a5436 = match v3.clone() {
            Adt3::C0(_, h5434, _, _, _, _, _, _) => {
                h5434
            }
        };
        let a5437 = 0u64;
        nat_rem(a5435, a5436, a5437)
    };
    let v9: u64 = {
        let a5439 = v6;
        let a5440 = match v3.clone() {
            Adt3::C0(_, _, h5438, _, _, _, _, _) => {
                h5438
            }
        };
        let a5441 = 0u64;
        nat_rem(a5439, a5440, a5441)
    };
    let v10: u64 = {
        let a5443 = v6;
        let a5444 = match v3.clone() {
            Adt3::C0(_, _, h5442, _, _, _, _, _) => {
                h5442
            }
        };
        let a5445 = 0u64;
        nat_quot(a5443, a5444, a5445)
    };
    let v11: u64 = {
        let a5446 = v4;
        let a5447 = 0u64;
        if nat_eq(a5446, a5447) {
            match v1.clone() {
                Adt3::C0(h5448, _, _, _, _, _, _, _) => {
                    h5448
                }
            }
        } else {
            let a5449 = v4;
            let a5450 = 1u64;
            if nat_eq(a5449, a5450) {
                match v1.clone() {
                    Adt3::C0(_, h5451, _, _, _, _, _, _) => {
                        h5451
                    }
                }
            } else {
                let a5452 = v4;
                let a5453 = 2u64;
                if nat_eq(a5452, a5453) {
                    match v1.clone() {
                        Adt3::C0(_, _, h5454, _, _, _, _, _) => {
                            h5454
                        }
                    }
                } else {
                    match v1.clone() {
                        Adt3::C0(_, _, _, h5455, _, _, _, _) => {
                            h5455
                        }
                    }
                }
            }
        }
    };
    let v12: bool = {
        let a5462 = v11;
        let a5463 = {
            let a5456 = v4;
            let a5457 = 0u64;
            if nat_eq(a5456, a5457) {
                v7
            } else {
                let a5458 = v4;
                let a5459 = 1u64;
                if nat_eq(a5458, a5459) {
                    v8
                } else {
                    let a5460 = v4;
                    let a5461 = 2u64;
                    if nat_eq(a5460, a5461) {
                        v9
                    } else {
                        v10
                    }
                }
            }
        };
        nat_le(a5462, a5463)
    };
    let v13: Adt3 = if v12 {
        v2
    } else {
        v1
    };
    Ok(Adt9::C1(v12, f158(if if v12 {
        let a5464 = v4;
        let a5465 = 0u64;
        nat_eq(a5464, a5465)
    } else {
        false
    } {
        let a5466 = v7;
        let a5467 = v11;
        nat_sub(a5466, a5467)
    } else {
        v7
    }, if if v12 {
        let a5468 = v4;
        let a5469 = 1u64;
        nat_eq(a5468, a5469)
    } else {
        false
    } {
        let a5470 = v8;
        let a5471 = v11;
        nat_sub(a5470, a5471)
    } else {
        v8
    }, if if v12 {
        let a5472 = v4;
        let a5473 = 2u64;
        nat_eq(a5472, a5473)
    } else {
        false
    } {
        let a5474 = v9;
        let a5475 = v11;
        nat_sub(a5474, a5475)
    } else {
        v9
    }, if if v12 {
        let a5476 = v4;
        let a5477 = 3u64;
        nat_eq(a5476, a5477)
    } else {
        false
    } {
        let a5478 = v10;
        let a5479 = v11;
        nat_sub(a5478, a5479)
    } else {
        v10
    }, match v13.clone() {
        Adt3::C0(_, _, _, _, h5480, _, _, _) => {
            h5480
        }
    }, match v13.clone() {
        Adt3::C0(_, _, _, _, _, h5481, _, _) => {
            h5481
        }
    }, match v13.clone() {
        Adt3::C0(_, _, _, _, _, _, h5482, _) => {
            h5482
        }
    }, match v13 {
        Adt3::C0(_, _, _, _, _, _, _, h5483) => {
            h5483
        }
    })?, f105(v0, v3)?))
}

pub fn f169(v0: u64, v1: Adt3, v2: Adt3, v3: u64, v4: u64, v5: u64, v6: u64, v7: bool) -> R<Adt9> {
    let v8: u64 = f65(v0, v2.clone(), 0u64);
    let v9: u64 = f65(v0, v2.clone(), 1u64);
    let v10: u64 = f65(v0, v2.clone(), 2u64);
    let v11: u64 = f65(v0, v2.clone(), 3u64);
    if v7 {
        Ok(Adt9::C1(false, f64(v1.clone(), f260(v8, v3, match v1.clone() {
            Adt3::C0(h5484, _, _, _, _, _, _, _) => {
                h5484
            }
        })?, f260(v9, v4, match v1.clone() {
            Adt3::C0(_, h5485, _, _, _, _, _, _) => {
                h5485
            }
        })?, f260(v10, v5, match v1.clone() {
            Adt3::C0(_, _, h5486, _, _, _, _, _) => {
                h5486
            }
        })?, f260(v11, v6, match v1 {
            Adt3::C0(_, _, _, h5487, _, _, _, _) => {
                h5487
            }
        })?)?, f105(v0, v2)?))
    } else {
        if if if if f261(v8, v3, match v1.clone() {
            Adt3::C0(h5488, _, _, _, _, _, _, _) => {
                h5488
            }
        })? {
            f261(v9, v4, match v1.clone() {
                Adt3::C0(_, h5489, _, _, _, _, _, _) => {
                    h5489
                }
            })?
        } else {
            false
        } {
            f261(v10, v5, match v1.clone() {
                Adt3::C0(_, _, h5490, _, _, _, _, _) => {
                    h5490
                }
            })?
        } else {
            false
        } {
            f261(v11, v6, match v1.clone() {
                Adt3::C0(_, _, _, h5491, _, _, _, _) => {
                    h5491
                }
            })?
        } else {
            false
        } {
            Ok(Adt9::C1(false, f64(v1, f262(v8, v3)?, f262(v9, v4)?, f262(v10, v5)?, f262(v11, v6)?)?, f105(v0, v2)?))
        } else {
            Ok(Adt9::C1(true, 0u64, f105(v0, v2)?))
        }
    }
}

pub fn f170(v0: u64, v1: Adt3, v2: Adt3, v3: u64) -> R<Adt9> {
    let v4: u64 = f65(v0, v2.clone(), 0u64);
    let v5: u64 = f65(v0, v2.clone(), 1u64);
    let v6: u64 = f65(v0, v2.clone(), 2u64);
    let v7: u64 = f65(v0, v2.clone(), 3u64);
    Ok(Adt9::C1(false, f64(v1.clone(), f263(v4, v3, match v1 {
        Adt3::C0(h5492, _, _, _, _, _, _, _) => {
            h5492
        }
    })?, v5, v6, v7)?, f105(v0, v2)?))
}

pub fn f171(v0: u64, v1: Adt3, v2: Adt3, v3: u64, v4: u64, v5: bool) -> R<Adt9> {
    let a5493 = v0;
    let a5494 = v3;
    if nat_lt(a5493, a5494) {
        Ok(Adt9::C1(false, {
            let a5495 = v0;
            let a5496 = 4u64;
            nat_mul(a5495, a5496)?
        }, {
            let a5497 = v0;
            let a5498 = 4u64;
            nat_mul(a5497, a5498)?
        }))
    } else {
        let v6: u64 = {
            let a5499 = v0;
            let a5500 = v3;
            nat_sub(a5499, a5500)
        };
        let v7: u64 = f105(v6, v1)?;
        let v8: u64 = f264(v6, v2, v4)?;
        if v5 {
            Ok(Adt9::C4(8u64, 0u64, v8, 4u64, v7, 3u64, v8))
        } else {
            Ok(Adt9::C1(true, v7, v8))
        }
    }
}

pub fn f172(v0: u64, v1: Adt3, v2: Adt3, v3: u64) -> R<Adt9> {
    let a5501 = v0;
    let a5502 = v3;
    if nat_lt(a5501, a5502) {
        Ok(Adt9::C1(true, 0u64, f105(v0, v2)?))
    } else {
        let v4: u64 = {
            let a5503 = v0;
            let a5504 = v3;
            nat_sub(a5503, a5504)
        };
        let v5: u64 = f167(v4, v2, v1.clone())?;
        Ok(Adt9::C4(8u64, 3u64, v5, 0u64, f105(v4, v1)?, 3u64, v5))
    }
}

pub fn f173(v0: u64, v1: u64, v2: Adt3, v3: Adt3) -> R<Adt9> {
    let v4: u64 = {
        let a5506 = v1;
        let a5507 = match v3.clone() {
            Adt3::C0(h5505, _, _, _, _, _, _, _) => {
                h5505
            }
        };
        let a5508 = 0u64;
        nat_rem(a5506, a5507, a5508)
    };
    let v5: u64 = {
        let a5514 = {
            let a5510 = v1;
            let a5511 = match v3.clone() {
                Adt3::C0(h5509, _, _, _, _, _, _, _) => {
                    h5509
                }
            };
            let a5512 = 0u64;
            nat_quot(a5510, a5511, a5512)
        };
        let a5515 = match v3.clone() {
            Adt3::C0(_, h5513, _, _, _, _, _, _) => {
                h5513
            }
        };
        let a5516 = 0u64;
        nat_rem(a5514, a5515, a5516)
    };
    let v6: bool = f265(v0, v4, v5);
    Ok(Adt9::C1(!v6, if v6 {
        f167(v1, v2, v3.clone())?
    } else {
        0u64
    }, f105(v1, v3)?))
}

pub fn f174(v0: u64, v1: Adt3, v2: Adt3, v3: u64, v4: bool, v5: Adt3) -> R<Adt9> {
    Ok(Adt9::C4({
        let a5517 = v3;
        let a5518 = 8u64;
        nat_add(a5517, a5518)?
    }, 0u64, f105(v0, v1)?, if v4 {
        5u64
    } else {
        4u64
    }, if v4 {
        0u64
    } else {
        f167(v0, v5, v2.clone())?
    }, 3u64, f105(v0, v2)?))
}

pub fn f175(v0: u64, v1: Adt3, v2: Adt3, v3: u64, v4: u64) -> R<Adt9> {
    let a5519 = v4;
    let a5520 = 0u64;
    if if nat_eq(a5519, a5520) {
        f139(v3)
    } else {
        false
    } {
        let m5521 = f235(v3);
        match m5521 {
            Adt18::C0 => {
                Ok(Adt9::C0(Adt10::C1))
            }
            Adt18::C1(v5) => {
                let v6: u64 = match v5.clone() {
                    Adt19::C0(_, h5522, _) => {
                        h5522
                    }
                };
                Ok(Adt9::C5(v3, f105(v0, v1)?, match v5 {
                    Adt19::C0(_, _, h5523) => {
                        h5523
                    }
                }, f105({
                    let a5524 = v0;
                    let a5525 = v6;
                    nat_mul(a5524, a5525)?
                }, v2.clone())?, {
                    let a5531 = {
                        let a5529 = {
                            let a5526 = v6;
                            let a5527 = 1u64;
                            nat_sub(a5526, a5527)
                        };
                        let a5530 = match v2.clone() {
                            Adt3::C0(_, _, _, _, h5528, _, _, _) => {
                                h5528
                            }
                        };
                        nat_mul(a5529, a5530)?
                    };
                    let a5532 = 4u64;
                    nat_add(a5531, a5532)?
                }, match v2 {
                    Adt3::C0(_, _, _, _, h5533, _, _, _) => {
                        h5533
                    }
                }))
            }
        }
    } else {
        Ok(Adt9::C3(v3, v4, f105(v0, v1)?, f105(v0, v2)?))
    }
}

pub fn f176(v0: u64, v1: Adt3, v2: Adt3, v3: bool, v4: u64, v5: u64) -> R<Adt9> {
    let v6: u64 = {
        let a5535 = v0;
        let a5536 = match v2.clone() {
            Adt3::C0(h5534, _, _, _, _, _, _, _) => {
                h5534
            }
        };
        let a5537 = 0u64;
        nat_rem(a5535, a5536, a5537)
    };
    let v7: u64 = {
        let a5539 = v0;
        let a5540 = match v2.clone() {
            Adt3::C0(h5538, _, _, _, _, _, _, _) => {
                h5538
            }
        };
        let a5541 = 0u64;
        nat_quot(a5539, a5540, a5541)
    };
    let v8: u64 = {
        let a5543 = v7;
        let a5544 = match v2.clone() {
            Adt3::C0(_, h5542, _, _, _, _, _, _) => {
                h5542
            }
        };
        let a5545 = 0u64;
        nat_rem(a5543, a5544, a5545)
    };
    let v9: u64 = {
        let a5547 = v7;
        let a5548 = match v2.clone() {
            Adt3::C0(_, h5546, _, _, _, _, _, _) => {
                h5546
            }
        };
        let a5549 = 0u64;
        nat_quot(a5547, a5548, a5549)
    };
    let v10: u64 = {
        let a5551 = v9;
        let a5552 = match v2.clone() {
            Adt3::C0(_, _, h5550, _, _, _, _, _) => {
                h5550
            }
        };
        let a5553 = 0u64;
        nat_rem(a5551, a5552, a5553)
    };
    let v11: u64 = {
        let a5555 = v9;
        let a5556 = match v2.clone() {
            Adt3::C0(_, _, h5554, _, _, _, _, _) => {
                h5554
            }
        };
        let a5557 = 0u64;
        nat_quot(a5555, a5556, a5557)
    };
    let v12: u64 = f105(v0, v2)?;
    if v3 {
        let v13: u64 = {
            let a5566 = {
                let a5561 = {
                    let a5558 = v10;
                    let a5559 = v4;
                    let a5560 = 0u64;
                    nat_quot(a5558, a5559, a5560)
                };
                let a5562 = v5;
                nat_mul(a5561, a5562)?
            };
            let a5567 = {
                let a5563 = v8;
                let a5564 = v4;
                let a5565 = 0u64;
                nat_quot(a5563, a5564, a5565)
            };
            nat_add(a5566, a5567)?
        };
        let v14: u64 = {
            let a5587 = v6;
            let a5588 = {
                let a5585 = match v1.clone() {
                    Adt3::C0(h5568, _, _, _, _, _, _, _) => {
                        h5568
                    }
                };
                let a5586 = {
                    let a5583 = {
                        let a5569 = v8;
                        let a5570 = v4;
                        let a5571 = 0u64;
                        nat_rem(a5569, a5570, a5571)
                    };
                    let a5584 = {
                        let a5581 = match v1.clone() {
                            Adt3::C0(_, h5572, _, _, _, _, _, _) => {
                                h5572
                            }
                        };
                        let a5582 = {
                            let a5579 = {
                                let a5573 = v10;
                                let a5574 = v4;
                                let a5575 = 0u64;
                                nat_rem(a5573, a5574, a5575)
                            };
                            let a5580 = {
                                let a5577 = match v1.clone() {
                                    Adt3::C0(_, _, h5576, _, _, _, _, _) => {
                                        h5576
                                    }
                                };
                                let a5578 = v13;
                                nat_mul(a5577, a5578)?
                            };
                            nat_add(a5579, a5580)?
                        };
                        nat_mul(a5581, a5582)?
                    };
                    nat_add(a5583, a5584)?
                };
                nat_mul(a5585, a5586)?
            };
            nat_add(a5587, a5588)?
        };
        Ok(Adt9::C1(false, f105(v14, v1)?, v12))
    } else {
        let v15: u64 = {
            let a5594 = {
                let a5592 = {
                    let a5589 = v11;
                    let a5590 = v5;
                    let a5591 = 0u64;
                    nat_rem(a5589, a5590, a5591)
                };
                let a5593 = v4;
                nat_mul(a5592, a5593)?
            };
            let a5595 = v8;
            nat_add(a5594, a5595)?
        };
        let v16: u64 = {
            let a5601 = {
                let a5599 = {
                    let a5596 = v11;
                    let a5597 = v5;
                    let a5598 = 0u64;
                    nat_quot(a5596, a5597, a5598)
                };
                let a5600 = v4;
                nat_mul(a5599, a5600)?
            };
            let a5602 = v10;
            nat_add(a5601, a5602)?
        };
        let a5604 = v15;
        let a5605 = match v1.clone() {
            Adt3::C0(_, h5603, _, _, _, _, _, _) => {
                h5603
            }
        };
        if if nat_lt(a5604, a5605) {
            let a5607 = v16;
            let a5608 = match v1.clone() {
                Adt3::C0(_, _, h5606, _, _, _, _, _) => {
                    h5606
                }
            };
            nat_lt(a5607, a5608)
        } else {
            false
        } {
            Ok(Adt9::C1(false, f105({
                let a5617 = v6;
                let a5618 = {
                    let a5615 = match v1.clone() {
                        Adt3::C0(h5609, _, _, _, _, _, _, _) => {
                            h5609
                        }
                    };
                    let a5616 = {
                        let a5613 = v15;
                        let a5614 = {
                            let a5611 = match v1.clone() {
                                Adt3::C0(_, h5610, _, _, _, _, _, _) => {
                                    h5610
                                }
                            };
                            let a5612 = v16;
                            nat_mul(a5611, a5612)?
                        };
                        nat_add(a5613, a5614)?
                    };
                    nat_mul(a5615, a5616)?
                };
                nat_add(a5617, a5618)?
            }, v1)?, v12))
        } else {
            Ok(Adt9::C1(true, 0u64, v12))
        }
    }
}

pub fn f177(v0: u64, v1: Adt3, v2: Adt3, v3: u64, v4: u64) -> R<Adt9> {
    let v5: u64 = f257(23u64, v4);
    let v6: u64 = {
        let a5619 = v0;
        let a5620 = v5;
        let a5621 = 0u64;
        nat_rem(a5619, a5620, a5621)
    };
    let v7: u64 = {
        let a5622 = v0;
        let a5623 = v5;
        let a5624 = 0u64;
        nat_quot(a5622, a5623, a5624)
    };
    let v8: Adt13 = Adt13::C1(f105(v7, v2)?);
    let a5625 = v6;
    let a5626 = 0u64;
    if nat_eq(a5625, a5626) {
        Ok(Adt9::C7(Adt11::C7(16u64, v3, 0u64), Adt12::C0(0u64, f105(v7, v1)?), Adt12::C2(0u64), Adt13::C0(0u64), f193(v3), 4u64))
    } else {
        let a5627 = v6;
        let a5628 = 1u64;
        if nat_eq(a5627, a5628) {
            Ok(Adt9::C7(Adt11::C1(1u64), Adt12::C1(0u64), Adt12::C2(1065353216u64), {
                let a5629 = v4;
                let a5630 = 0u64;
                if nat_eq(a5629, a5630) {
                    v8
                } else {
                    Adt13::C0(0u64)
                }
            }, 4u64, 4u64))
        } else {
            Ok(Adt9::C7(Adt11::C2(0u64, v4, true), Adt12::C1(0u64), Adt12::C2(0u64), v8, 4u64, f193(v4)))
        }
    }
}

pub fn f178(v0: u64, v1: Adt3, v2: Adt3) -> R<Adt9> {
    let v3: u64 = f105(v0, v2.clone())?;
    let v4: u64 = f105(v0, v1.clone())?;
    let a5635 = {
        let a5632 = v0;
        let a5633 = match v1 {
            Adt3::C0(h5631, _, _, _, _, _, _, _) => {
                h5631
            }
        };
        let a5634 = 0u64;
        nat_rem(a5632, a5633, a5634)
    };
    let a5636 = 0u64;
    if nat_eq(a5635, a5636) {
        Ok(Adt9::C1(false, v4, v3))
    } else {
        Ok(Adt9::C4(8u64, 3u64, f105({
            let a5637 = v0;
            let a5638 = 1u64;
            nat_sub(a5637, a5638)
        }, v2)?, 0u64, v4, 3u64, v3))
    }
}

pub fn f179(v0: u64, v1: Adt3, v2: Adt3, v3: u64, v4: u64) -> R<Adt9> {
    let a5639 = v3;
    let a5640 = 13u64;
    if nat_eq(a5639, a5640) {
        f266(v0, v1, v4)
    } else {
        let v5: u64 = f267(match v1.clone() {
            Adt3::C0(h5641, _, _, _, _, _, _, _) => {
                h5641
            }
        }, {
            let a5642 = v3;
            let a5643 = 16u64;
            nat_eq(a5642, a5643)
        })?;
        let v6: u64 = {
            let a5644 = v0;
            let a5645 = v5;
            let a5646 = 0u64;
            nat_quot(a5644, a5645, a5646)
        };
        let v7: u64 = {
            let a5647 = v0;
            let a5648 = v5;
            let a5649 = 0u64;
            nat_rem(a5647, a5648, a5649)
        };
        let v8: u64 = {
            let a5653 = 1u64;
            let a5654 = {
                let a5651 = match v1.clone() {
                    Adt3::C0(h5650, _, _, _, _, _, _, _) => {
                        h5650
                    }
                };
                let a5652 = 2u64;
                nat_mul(a5651, a5652)?
            };
            nat_add(a5653, a5654)?
        };
        let a5655 = v7;
        let a5656 = v8;
        if nat_lt(a5655, a5656) {
            f268(v7, v6, v1, 0u64)
        } else {
            let v9: u64 = f105(v6, v2)?;
            let a5657 = v3;
            let a5658 = 14u64;
            if nat_eq(a5657, a5658) {
                Ok(Adt9::C4(2u64, 1u64, 1u64, 2u64, 0u64, 3u64, v9))
            } else {
                let a5659 = v7;
                let a5660 = v8;
                if nat_eq(a5659, a5660) {
                    Ok(Adt9::C4(2u64, 1u64, 1u64, 2u64, 0u64, 1u64, 1u64))
                } else {
                    let a5663 = v7;
                    let a5664 = {
                        let a5661 = v8;
                        let a5662 = 1u64;
                        nat_add(a5661, a5662)?
                    };
                    if nat_eq(a5663, a5664) {
                        Ok(Adt9::C4(3u64, 2u64, match v1 {
                            Adt3::C0(h5665, _, _, _, _, _, _, _) => {
                                h5665
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

pub fn f180(v0: u64, v1: Adt3, v2: Adt3, v3: u64) -> R<Adt9> {
    let a5666 = v0;
    let a5667 = 0u64;
    if nat_eq(a5666, a5667) {
        Ok(Adt9::C4(19u64, 2u64, 0u64, 2u64, 0u64, 1u64, 0u64))
    } else {
        let a5674 = {
            let a5668 = v0;
            let a5669 = 1u64;
            nat_add(a5668, a5669)?
        };
        let a5675 = {
            let a5672 = 2u64;
            let a5673 = {
                let a5670 = v3;
                let a5671 = 2u64;
                nat_mul(a5670, a5671)?
            };
            nat_add(a5672, a5673)?
        };
        if nat_eq(a5674, a5675) {
            Ok(Adt9::C4(19u64, 1u64, 0u64, 2u64, 0u64, 6u64, 0u64))
        } else {
            let v4: u64 = {
                let a5676 = v0;
                let a5677 = 1u64;
                nat_sub(a5676, a5677)
            };
            let v5: u64 = {
                let a5678 = v4;
                let a5679 = 2u64;
                let a5680 = 0u64;
                nat_quot(a5678, a5679, a5680)
            };
            let a5684 = {
                let a5681 = v4;
                let a5682 = 2u64;
                let a5683 = 0u64;
                nat_rem(a5681, a5682, a5683)
            };
            let a5685 = 0u64;
            if nat_eq(a5684, a5685) {
                Ok(Adt9::C4(17u64, 0u64, f105(v5, v1)?, 4u64, f105(v5, v2)?, 1u64, 2u64))
            } else {
                Ok(Adt9::C4(18u64, 1u64, 0u64, 1u64, 2u64, 1u64, 0u64))
            }
        }
    }
}

pub fn f181(v0: u64, v1: u64, v2: u64, v3: u64) -> R<Adt9> {
    let a5686 = v0;
    let a5687 = f48(v1, v2, v3)?;
    if nat_lt(a5686, a5687) {
        let a5688 = v0;
        let a5689 = v1;
        if nat_lt(a5688, a5689) {
            Ok(Adt9::C1(false, {
                let a5690 = 4u64;
                let a5691 = v0;
                nat_mul(a5690, a5691)?
            }, {
                let a5692 = 4u64;
                let a5693 = v0;
                nat_mul(a5692, a5693)?
            }))
        } else {
            let v4: u64 = {
                let a5694 = v0;
                let a5695 = v1;
                nat_sub(a5694, a5695)
            };
            let v5: u64 = f269(v4, v2)?;
            let a5696 = v5;
            let a5697 = 2u64;
            if nat_lt(a5696, a5697) {
                Ok(Adt9::C4(6u64, {
                    let a5698 = v5;
                    let a5699 = 0u64;
                    if nat_eq(a5698, a5699) {
                        4u64
                    } else {
                        5u64
                    }
                }, f270(v4, v2)?, 2u64, 0u64, 1u64, v5))
            } else {
                let v6: u64 = f271(v4, v2)?;
                Ok(Adt9::C4(8u64, 3u64, v6, 1u64, if f272(v4, v2)? {
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

pub fn f182(v0: u64, v1: u64) -> u64 {
    let a5700 = v0;
    let a5701 = 0u64;
    if if nat_eq(a5700, a5701) {
        true
    } else {
        let a5702 = v1;
        let a5703 = 0u64;
        nat_eq(a5702, a5703)
    } {
        1u64
    } else {
        2u64
    }
}

pub fn f183(v0: u64, v1: u64) -> Adt11 {
    let a5704 = v0;
    let a5705 = 43u64;
    if nat_eq(a5704, a5705) {
        Adt11::C8(3u64)
    } else {
        let a5706 = v1;
        let a5707 = 43u64;
        if nat_eq(a5706, a5707) {
            Adt11::C8(4u64)
        } else {
            Adt11::C2(v0, v1, true)
        }
    }
}

pub fn f184(v0: u64) -> R<u64> {
    let a5710 = 2u64;
    let a5711 = {
        let a5708 = 6u64;
        let a5709 = v0;
        nat_mul(a5708, a5709)?
    };
    nat_add(a5710, a5711)
}

pub fn f185(v0: u64, v1: bool) -> R<u64> {
    let a5714 = {
        let a5712 = v0;
        let a5713 = if v1 {
            3u64
        } else {
            2u64
        };
        nat_mul(a5712, a5713)?
    };
    let a5715 = 1u64;
    nat_add(a5714, a5715)
}

pub fn f186(v0: u64, v1: u64, v2: u64, v3: Adt3, v4: Adt3, v5: Adt3, v6: Adt3, v7: u64, v8: u64) -> R<Adt9> {
    let a5716 = v0;
    let a5717 = 16u64;
    if nat_lt(a5716, a5717) {
        let v9: u64 = {
            let a5718 = v0;
            let a5719 = 4u64;
            let a5720 = 0u64;
            nat_quot(a5718, a5719, a5720)
        };
        let v10: u64 = {
            let a5725 = {
                let a5723 = 8u64;
                let a5724 = {
                    let a5721 = 4u64;
                    let a5722 = v1;
                    nat_mul(a5721, a5722)?
                };
                nat_add(a5723, a5724)?
            };
            let a5726 = v9;
            nat_add(a5725, a5726)?
        };
        let v11: u64 = f64(v6, v9, v1, v2, 0u64)?;
        let a5730 = {
            let a5727 = v0;
            let a5728 = 4u64;
            let a5729 = 0u64;
            nat_rem(a5727, a5728, a5729)
        };
        let a5731 = 0u64;
        if nat_eq(a5730, a5731) {
            Ok(Adt9::C7(Adt11::C1(2u64), Adt12::C0(0u64, f64(v3, v10, v2, 0u64, 0u64)?), Adt12::C0(1u64, f64(v4, 2u64, 0u64, 0u64, 0u64)?), Adt13::C0(1u64), 4u64, 4u64))
        } else {
            let a5735 = {
                let a5732 = v0;
                let a5733 = 4u64;
                let a5734 = 0u64;
                nat_rem(a5732, a5733, a5734)
            };
            let a5736 = 1u64;
            if nat_eq(a5735, a5736) {
                Ok(Adt9::C7(Adt11::C1(0u64), Adt12::C1(1u64), Adt12::C0(2u64, f64(v5, v10, 0u64, 0u64, 0u64)?), Adt13::C1(v11), 4u64, 4u64))
            } else {
                let a5740 = {
                    let a5737 = v0;
                    let a5738 = 4u64;
                    let a5739 = 0u64;
                    nat_rem(a5737, a5738, a5739)
                };
                let a5741 = 2u64;
                if nat_eq(a5740, a5741) {
                    Ok(Adt9::C7(Adt11::C5(0u64), {
                        let a5742 = v9;
                        let a5743 = 0u64;
                        if nat_eq(a5742, a5743) {
                            Adt12::C2(4286578688u64)
                        } else {
                            Adt12::C1(0u64)
                        }
                    }, Adt12::C3(v11), Adt13::C0(2u64), 4u64, 4u64))
                } else {
                    Ok(Adt9::C7(Adt11::C5(3u64), {
                        let a5744 = v8;
                        let a5745 = 2u64;
                        if nat_eq(a5744, a5745) {
                            let a5746 = v9;
                            let a5747 = 0u64;
                            if nat_eq(a5746, a5747) {
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
        let a5748 = v0;
        let a5749 = 28u64;
        if nat_lt(a5748, a5749) {
            let v12: u64 = {
                let a5750 = v0;
                let a5751 = 16u64;
                nat_sub(a5750, a5751)
            };
            let v13: u64 = {
                let a5752 = v12;
                let a5753 = 3u64;
                let a5754 = 0u64;
                nat_quot(a5752, a5753, a5754)
            };
            let v14: u64 = f64(v6, v13, v1, v2, 0u64)?;
            let a5758 = {
                let a5755 = v12;
                let a5756 = 3u64;
                let a5757 = 0u64;
                nat_rem(a5755, a5756, a5757)
            };
            let a5759 = 0u64;
            if nat_eq(a5758, a5759) {
                Ok(Adt9::C7(Adt11::C1(1u64), Adt12::C3(v14), Adt12::C1(0u64), Adt13::C0(2u64), 4u64, 4u64))
            } else {
                let a5763 = {
                    let a5760 = v12;
                    let a5761 = 3u64;
                    let a5762 = 0u64;
                    nat_rem(a5760, a5761, a5762)
                };
                let a5764 = 1u64;
                if nat_eq(a5763, a5764) {
                    Ok(Adt9::C7(Adt11::C7(16u64, 0u64, 0u64), Adt12::C1(2u64), Adt12::C2(0u64), Adt13::C1(v14), 4u64, 4u64))
                } else {
                    Ok(Adt9::C7(Adt11::C1(0u64), {
                        let a5765 = v13;
                        let a5766 = 0u64;
                        if nat_eq(a5765, a5766) {
                            Adt12::C2(0u64)
                        } else {
                            Adt12::C1(1u64)
                        }
                    }, Adt12::C3(v14), Adt13::C0(1u64), 4u64, 4u64))
                }
            }
        } else {
            let a5767 = v0;
            let a5768 = 28u64;
            if nat_eq(a5767, a5768) {
                Ok(Adt9::C7(Adt11::C1(3u64), Adt12::C2(1065353216u64), Adt12::C1(1u64), Adt13::C0(1u64), 4u64, 4u64))
            } else {
                let v15: u64 = {
                    let a5769 = v0;
                    let a5770 = 29u64;
                    nat_sub(a5769, a5770)
                };
                let v16: u64 = {
                    let a5771 = v15;
                    let a5772 = 2u64;
                    let a5773 = 0u64;
                    nat_quot(a5771, a5772, a5773)
                };
                let v17: u64 = f64(v6, v16, v1, v2, 0u64)?;
                let a5777 = {
                    let a5774 = v15;
                    let a5775 = 2u64;
                    let a5776 = 0u64;
                    nat_rem(a5774, a5775, a5776)
                };
                let a5778 = 0u64;
                if nat_eq(a5777, a5778) {
                    Ok(Adt9::C7(Adt11::C1(2u64), Adt12::C3(v17), Adt12::C1(1u64), Adt13::C0(2u64), 4u64, 4u64))
                } else {
                    Ok(Adt9::C7(Adt11::C1(0u64), Adt12::C1(2u64), Adt12::C2(v7), Adt13::C1(v17), 4u64, 4u64))
                }
            }
        }
    }
}

pub fn f187(v0: u64, v1: u64, v2: Adt3, v3: u64) -> R<Adt9> {
    let v4: u64 = {
        let a5779 = v0;
        let a5780 = 9u64;
        let a5781 = 0u64;
        nat_rem(a5779, a5780, a5781)
    };
    let v5: u64 = {
        let a5785 = {
            let a5782 = v0;
            let a5783 = 9u64;
            let a5784 = 0u64;
            nat_quot(a5782, a5783, a5784)
        };
        let a5786 = 4u64;
        let a5787 = 0u64;
        nat_rem(a5785, a5786, a5787)
    };
    let v6: bool = {
        let a5794 = {
            let a5791 = {
                let a5788 = v0;
                let a5789 = 36u64;
                let a5790 = 0u64;
                nat_quot(a5788, a5789, a5790)
            };
            let a5792 = 2u64;
            let a5793 = 0u64;
            nat_rem(a5791, a5792, a5793)
        };
        let a5795 = 0u64;
        nat_eq(a5794, a5795)
    };
    let v7: u64 = {
        let a5796 = v4;
        let a5797 = 4u64;
        if nat_lt(a5796, a5797) {
            v4
        } else {
            let a5798 = v4;
            let a5799 = 5u64;
            nat_sub(a5798, a5799)
        }
    };
    let v8: u64 = f64(v2, if v6 {
        v5
    } else {
        v7
    }, if v6 {
        v7
    } else {
        v5
    }, v1, 0u64)?;
    let a5800 = v4;
    let a5801 = 4u64;
    if nat_lt(a5800, a5801) {
        Ok(Adt9::C7(Adt11::C1(0u64), {
            let a5802 = v4;
            let a5803 = 0u64;
            if nat_eq(a5802, a5803) {
                Adt12::C2(v3)
            } else {
                Adt12::C1(1u64)
            }
        }, Adt12::C3(v8), Adt13::C0(1u64), 4u64, 4u64))
    } else {
        let a5804 = v4;
        let a5805 = 4u64;
        if nat_eq(a5804, a5805) {
            Ok(Adt9::C7(Adt11::C1(3u64), Adt12::C2(1065353216u64), Adt12::C1(1u64), Adt13::C0(1u64), 4u64, 4u64))
        } else {
            Ok(Adt9::C7(Adt11::C1(2u64), Adt12::C3(v8), Adt12::C1(1u64), Adt13::C1(v8), 4u64, 4u64))
        }
    }
}

pub fn f188(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: bool, v6: u64) -> R<Adt9> {
    let a5806 = v0;
    let a5807 = 3u64;
    if nat_lt(a5806, a5807) {
        let v7: bool = if if v5 {
            let a5808 = 1u64;
            let a5809 = v2;
            nat_lt(a5808, a5809)
        } else {
            false
        } {
            let a5810 = 1u64;
            let a5811 = v3;
            nat_lt(a5810, a5811)
        } else {
            false
        };
        let a5812 = v0;
        let a5813 = 0u64;
        if nat_eq(a5812, a5813) {
            Ok(Adt9::C7(Adt11::C0(3u64), Adt12::C2(if v7 {
                let a5814 = v3;
                let a5815 = 1u64;
                nat_sub(a5814, a5815)
            } else {
                v3
            }), Adt12::C2(0u64), Adt13::C0(0u64), 4u64, 4u64))
        } else {
            let a5816 = v0;
            let a5817 = 1u64;
            if nat_eq(a5816, a5817) {
                Ok(Adt9::C7(Adt11::C0(3u64), Adt12::C2(if v7 {
                    let a5818 = v2;
                    let a5819 = 1u64;
                    nat_sub(a5818, a5819)
                } else {
                    v2
                }), Adt12::C2(0u64), Adt13::C0(1u64), 4u64, 4u64))
            } else {
                Ok(Adt9::C7(Adt11::C1(3u64), Adt12::C1(0u64), Adt12::C1(1u64), Adt13::C0(3u64), 4u64, 4u64))
            }
        }
    } else {
        let v8: u64 = {
            let a5820 = v2;
            let a5821 = 2147483521u64;
            if nat_lt(a5820, a5821) {
                4u64
            } else {
                let a5824 = 2u64;
                let a5825 = {
                    let a5822 = 4u64;
                    let a5823 = f104(v2);
                    nat_mul(a5822, a5823)?
                };
                nat_add(a5824, a5825)?
            }
        };
        let v9: u64 = {
            let a5826 = v0;
            let a5827 = 3u64;
            nat_sub(a5826, a5827)
        };
        let v10: u64 = {
            let a5828 = v9;
            let a5829 = v8;
            let a5830 = 0u64;
            nat_quot(a5828, a5829, a5830)
        };
        let v11: u64 = {
            let a5831 = v9;
            let a5832 = v8;
            let a5833 = 0u64;
            nat_rem(a5831, a5832, a5833)
        };
        let v12: u64 = {
            let a5834 = v1;
            let a5835 = v10;
            nat_add(a5834, a5835)?
        };
        let a5836 = v2;
        let a5837 = 2147483521u64;
        if nat_lt(a5836, a5837) {
            let a5838 = v11;
            let a5839 = 0u64;
            if nat_eq(a5838, a5839) {
                Ok(Adt9::C7(Adt11::C0(3u64), Adt12::C2(v10), Adt12::C2(0u64), Adt13::C0(0u64), 4u64, 4u64))
            } else {
                let a5840 = v11;
                let a5841 = 1u64;
                if nat_eq(a5840, a5841) {
                    Ok(Adt9::C7(Adt11::C1(3u64), Adt12::C1(0u64), Adt12::C1(3u64), Adt13::C0(2u64), 4u64, 4u64))
                } else {
                    let a5842 = v11;
                    let a5843 = 2u64;
                    if nat_eq(a5842, a5843) {
                        Ok(Adt9::C7(Adt11::C9, Adt12::C1(2u64), Adt12::C2(0u64), Adt13::C0(2u64), 4u64, 4u64))
                    } else {
                        Ok(Adt9::C7(Adt11::C5(3u64), Adt12::C2({
                            let a5848 = {
                                let a5844 = v6;
                                let a5845 = v2;
                                if nat_lt(a5844, a5845) {
                                    v6
                                } else {
                                    let a5846 = v2;
                                    let a5847 = 1u64;
                                    nat_sub(a5846, a5847)
                                }
                            };
                            let a5849 = v4;
                            nat_mul(a5848, a5849)?
                        }), Adt12::C2(0u64), Adt13::C0(v12), 4u64, 4u64))
                    }
                }
            }
        } else {
            let a5850 = v11;
            let a5851 = 0u64;
            if nat_eq(a5850, a5851) {
                Ok(Adt9::C7(Adt11::C0(3u64), Adt12::C2(v10), Adt12::C2(0u64), Adt13::C0(0u64), 4u64, 4u64))
            } else {
                let a5852 = v11;
                let a5853 = 1u64;
                if nat_eq(a5852, a5853) {
                    Ok(Adt9::C7(Adt11::C1(3u64), Adt12::C1(0u64), Adt12::C1(3u64), Adt13::C0(1u64), 4u64, 4u64))
                } else {
                    let v13: u64 = {
                        let a5856 = {
                            let a5854 = v11;
                            let a5855 = 2u64;
                            nat_sub(a5854, a5855)
                        };
                        let a5857 = 4u64;
                        let a5858 = 0u64;
                        nat_quot(a5856, a5857, a5858)
                    };
                    let v14: u64 = {
                        let a5861 = {
                            let a5859 = v11;
                            let a5860 = 2u64;
                            nat_sub(a5859, a5860)
                        };
                        let a5862 = 4u64;
                        let a5863 = 0u64;
                        nat_rem(a5861, a5862, a5863)
                    };
                    let a5864 = v14;
                    let a5865 = 0u64;
                    if nat_eq(a5864, a5865) {
                        Ok(Adt9::C7(Adt11::C5(3u64), Adt12::C1(1u64), Adt12::C2(0u64), Adt13::C0(2u64), 4u64, 4u64))
                    } else {
                        let a5866 = v14;
                        let a5867 = 1u64;
                        if nat_eq(a5866, a5867) {
                            let a5878 = {
                                let a5868 = v13;
                                let a5869 = 0u64;
                                if nat_eq(a5868, a5869) {
                                    0u64
                                } else {
                                    let a5870 = v13;
                                    let a5871 = 1u64;
                                    if nat_eq(a5870, a5871) {
                                        1325400064u64
                                    } else {
                                        let a5872 = v13;
                                        let a5873 = 2u64;
                                        if nat_eq(a5872, a5873) {
                                            1392508928u64
                                        } else {
                                            let a5874 = v13;
                                            let a5875 = 3u64;
                                            if nat_eq(a5874, a5875) {
                                                1459617792u64
                                            } else {
                                                let a5876 = v13;
                                                let a5877 = 4u64;
                                                if nat_eq(a5876, a5877) {
                                                    1526726656u64
                                                } else {
                                                    1593835520u64
                                                }
                                            }
                                        }
                                    }
                                }
                            };
                            let a5879 = v6;
                            if if nat_le(a5878, a5879) {
                                let a5890 = v6;
                                let a5891 = {
                                    let a5880 = v13;
                                    let a5881 = 0u64;
                                    if nat_eq(a5880, a5881) {
                                        1325400064u64
                                    } else {
                                        let a5882 = v13;
                                        let a5883 = 1u64;
                                        if nat_eq(a5882, a5883) {
                                            1392508928u64
                                        } else {
                                            let a5884 = v13;
                                            let a5885 = 2u64;
                                            if nat_eq(a5884, a5885) {
                                                1459617792u64
                                            } else {
                                                let a5886 = v13;
                                                let a5887 = 3u64;
                                                if nat_eq(a5886, a5887) {
                                                    1526726656u64
                                                } else {
                                                    let a5888 = v13;
                                                    let a5889 = 4u64;
                                                    if nat_eq(a5888, a5889) {
                                                        1593835520u64
                                                    } else {
                                                        1660944384u64
                                                    }
                                                }
                                            }
                                        }
                                    }
                                };
                                nat_lt(a5890, a5891)
                            } else {
                                false
                            } {
                                Ok(Adt9::C7(Adt11::C1(3u64), Adt12::C1(1u64), Adt12::C2({
                                    let a5892 = v13;
                                    let a5893 = 0u64;
                                    if nat_eq(a5892, a5893) {
                                        1065353216u64
                                    } else {
                                        let a5894 = v13;
                                        let a5895 = 1u64;
                                        if nat_eq(a5894, a5895) {
                                            1132462080u64
                                        } else {
                                            let a5896 = v13;
                                            let a5897 = 2u64;
                                            if nat_eq(a5896, a5897) {
                                                1199570944u64
                                            } else {
                                                let a5898 = v13;
                                                let a5899 = 3u64;
                                                if nat_eq(a5898, a5899) {
                                                    1266679808u64
                                                } else {
                                                    let a5900 = v13;
                                                    let a5901 = 4u64;
                                                    if nat_eq(a5900, a5901) {
                                                        1333788672u64
                                                    } else {
                                                        1400897536u64
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }), Adt13::C0(0u64), 4u64, 4u64))
                            } else {
                                Ok(Adt9::C7(Adt11::C5(3u64), Adt12::C2(3212836864u64), Adt12::C2(0u64), Adt13::C0(0u64), 4u64, 4u64))
                            }
                        } else {
                            let a5902 = v14;
                            let a5903 = 2u64;
                            if nat_eq(a5902, a5903) {
                                Ok(Adt9::C7(Adt11::C9, Adt12::C1(0u64), Adt12::C2(0u64), Adt13::C0(2u64), 4u64, 4u64))
                            } else {
                                let a5904 = v6;
                                let a5905 = 4294967295u64;
                                if nat_eq(a5904, a5905) {
                                    Ok(Adt9::C7(Adt11::C5(3u64), Adt12::C1(0u64), Adt12::C2(0u64), Adt13::C0(0u64), 4u64, 4u64))
                                } else {
                                    Ok(Adt9::C7(Adt11::C5(3u64), Adt12::C2({
                                        let a5934 = {
                                            let a5918 = {
                                                let a5916 = v6;
                                                let a5917 = {
                                                    let a5906 = v13;
                                                    let a5907 = 0u64;
                                                    if nat_eq(a5906, a5907) {
                                                        1u64
                                                    } else {
                                                        let a5908 = v13;
                                                        let a5909 = 1u64;
                                                        if nat_eq(a5908, a5909) {
                                                            256u64
                                                        } else {
                                                            let a5910 = v13;
                                                            let a5911 = 2u64;
                                                            if nat_eq(a5910, a5911) {
                                                                65536u64
                                                            } else {
                                                                let a5912 = v13;
                                                                let a5913 = 3u64;
                                                                if nat_eq(a5912, a5913) {
                                                                    16777216u64
                                                                } else {
                                                                    let a5914 = v13;
                                                                    let a5915 = 4u64;
                                                                    if nat_eq(a5914, a5915) {
                                                                        4294967296u64
                                                                    } else {
                                                                        1099511627776u64
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                };
                                                nat_mul(a5916, a5917)?
                                            };
                                            let a5919 = v2;
                                            if nat_lt(a5918, a5919) {
                                                let a5930 = v6;
                                                let a5931 = {
                                                    let a5920 = v13;
                                                    let a5921 = 0u64;
                                                    if nat_eq(a5920, a5921) {
                                                        1u64
                                                    } else {
                                                        let a5922 = v13;
                                                        let a5923 = 1u64;
                                                        if nat_eq(a5922, a5923) {
                                                            256u64
                                                        } else {
                                                            let a5924 = v13;
                                                            let a5925 = 2u64;
                                                            if nat_eq(a5924, a5925) {
                                                                65536u64
                                                            } else {
                                                                let a5926 = v13;
                                                                let a5927 = 3u64;
                                                                if nat_eq(a5926, a5927) {
                                                                    16777216u64
                                                                } else {
                                                                    let a5928 = v13;
                                                                    let a5929 = 4u64;
                                                                    if nat_eq(a5928, a5929) {
                                                                        4294967296u64
                                                                    } else {
                                                                        1099511627776u64
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                };
                                                nat_mul(a5930, a5931)?
                                            } else {
                                                let a5932 = v2;
                                                let a5933 = 1u64;
                                                nat_sub(a5932, a5933)
                                            }
                                        };
                                        let a5935 = v4;
                                        nat_mul(a5934, a5935)?
                                    }), Adt12::C2(0u64), Adt13::C0(v12), 4u64, 4u64))
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn f189(v0: bool, v1: u64) -> R<u64> {
    if v0 {
        let a5938 = {
            let a5936 = 3u64;
            let a5937 = v1;
            nat_mul(a5936, a5937)?
        };
        let a5939 = 4u64;
        nat_add(a5938, a5939)
    } else {
        let a5940 = v1;
        let a5941 = 3u64;
        nat_add(a5940, a5941)
    }
}

pub fn f190(v0: Adt7, v1: Adt3, v2: Adt3, v3: u64, v4: u64) -> R<Option<u64>> {
    let m5950 = f67(match v0.clone() {
        Adt7::C0(h5942, _, _, _, _) => {
            h5942
        }
    }, match v1.clone() {
        Adt3::C0(h5943, _, _, _, _, _, _, _) => {
            h5943
        }
    }, f65(v3, v2.clone(), 0u64), {
        let a5947 = v4;
        let a5948 = {
            let m5946 = match v0.clone() {
                Adt7::C0(h5945, _, _, _, _) => {
                    h5945
                }
            };
            match m5946 {
                Adt6::C0(h5944, _, _, _) => {
                    h5944
                }
            }
        };
        let a5949 = 0u64;
        nat_rem(a5947, a5948, a5949)
    })?;
    match m5950 {
        None => {
            Ok(None::<u64>)
        }
        Some(v5) => {
            let m5959 = f67(match v0.clone() {
                Adt7::C0(_, h5951, _, _, _) => {
                    h5951
                }
            }, match v1.clone() {
                Adt3::C0(_, h5952, _, _, _, _, _, _) => {
                    h5952
                }
            }, f65(v3, v2.clone(), 1u64), {
                let a5956 = v4;
                let a5957 = {
                    let m5955 = match v0 {
                        Adt7::C0(h5954, _, _, _, _) => {
                            h5954
                        }
                    };
                    match m5955 {
                        Adt6::C0(h5953, _, _, _) => {
                            h5953
                        }
                    }
                };
                let a5958 = 0u64;
                nat_quot(a5956, a5957, a5958)
            })?;
            match m5959 {
                None => {
                    Ok(None::<u64>)
                }
                Some(v6) => {
                    Ok(Some(f64(v1, v5, v6, f65(v3, v2.clone(), 2u64), f65(v3, v2, 3u64))?))
                }
            }
        }
    }
}

pub fn f191(v0: Adt6, v1: u64, v2: u64, v3: u64) -> R<Option<u64>> {
    let v4: u64 = {
        let a5961 = v2;
        let a5962 = f160(match v0.clone() {
            Adt6::C0(_, _, h5960, _) => {
                h5960
            }
        });
        nat_add(a5961, a5962)?
    };
    let v5: u64 = {
        let a5967 = {
            let a5964 = v3;
            let a5965 = match v0.clone() {
                Adt6::C0(_, _, _, h5963) => {
                    h5963
                }
            };
            nat_mul(a5964, a5965)?
        };
        let a5968 = f159(match v0.clone() {
            Adt6::C0(_, _, h5966, _) => {
                h5966
            }
        });
        nat_add(a5967, a5968)?
    };
    let v6: u64 = {
        let a5969 = v4;
        let a5970 = v5;
        nat_sub(a5969, a5970)
    };
    let v7: u64 = match v0 {
        Adt6::C0(_, h5971, _, _) => {
            h5971
        }
    };
    let v8: u64 = {
        let a5972 = v6;
        let a5973 = v7;
        let a5974 = 0u64;
        nat_quot(a5972, a5973, a5974)
    };
    let a5975 = v5;
    let a5976 = v4;
    if if if nat_le(a5975, a5976) {
        let a5980 = {
            let a5977 = v6;
            let a5978 = v7;
            let a5979 = 0u64;
            nat_rem(a5977, a5978, a5979)
        };
        let a5981 = 0u64;
        nat_eq(a5980, a5981)
    } else {
        false
    } {
        let a5982 = v8;
        let a5983 = v1;
        nat_lt(a5982, a5983)
    } else {
        false
    } {
        Ok(Some(v8))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f192(v0: bool, v1: u64) -> R<u64> {
    let a5986 = if v0 {
        8u64
    } else {
        3u64
    };
    let a5987 = {
        let a5984 = v1;
        let a5985 = 0u64;
        if nat_eq(a5984, a5985) {
            0u64
        } else {
            1u64
        }
    };
    nat_add(a5986, a5987)
}

pub fn f193(v0: u64) -> u64 {
    let m5988 = f235(v0);
    match m5988 {
        Adt18::C0 => {
            0u64
        }
        Adt18::C1(v1) => {
            let a5990 = match v1.clone() {
                Adt19::C0(_, h5989, _) => {
                    h5989
                }
            };
            let a5991 = 1u64;
            if if nat_eq(a5990, a5991) {
                let a5993 = match v1.clone() {
                    Adt19::C0(_, _, h5992) => {
                        h5992
                    }
                };
                let a5994 = 1u64;
                if if if nat_eq(a5993, a5994) {
                    true
                } else {
                    let a5996 = match v1.clone() {
                        Adt19::C0(_, _, h5995) => {
                            h5995
                        }
                    };
                    let a5997 = 2u64;
                    nat_eq(a5996, a5997)
                } {
                    true
                } else {
                    let a5999 = match v1.clone() {
                        Adt19::C0(_, _, h5998) => {
                            h5998
                        }
                    };
                    let a6000 = 4u64;
                    nat_eq(a5999, a6000)
                } {
                    true
                } else {
                    let a6002 = match v1.clone() {
                        Adt19::C0(_, _, h6001) => {
                            h6001
                        }
                    };
                    let a6003 = 8u64;
                    nat_eq(a6002, a6003)
                }
            } else {
                false
            } {
                match v1 {
                    Adt19::C0(_, _, h6004) => {
                        h6004
                    }
                }
            } else {
                0u64
            }
        }
    }
}

pub fn f194(v0: u64, v1: u64, v2: Adt1, v3: Adt13) -> Adt9 {
    let a6005 = v0;
    let a6006 = 2u64;
    if nat_eq(a6005, a6006) {
        let a6007 = v1;
        let a6008 = 2u64;
        if nat_eq(a6007, a6008) {
            Adt9::C7(Adt11::C5(3u64), Adt12::C2(5u64), Adt12::C2(0u64), Adt13::C0(2u64), 4u64, 4u64)
        } else {
            Adt9::C7(Adt11::C5(0u64), Adt12::C1(0u64), Adt12::C2(match v2 {
                Adt1::C0(_, _, _, _, h6009, _, _, _, _, _, _, _) => {
                    h6009
                }
            }), Adt13::C0(2u64), 4u64, 4u64)
        }
    } else {
        let a6010 = v0;
        let a6011 = 3u64;
        if nat_eq(a6010, a6011) {
            let a6012 = v1;
            let a6013 = 5u64;
            if nat_eq(a6012, a6013) {
                Adt9::C7(Adt11::C1(2u64), Adt12::C2(match v2 {
                    Adt1::C0(_, _, h6014, _, _, _, _, _, _, _, _, _) => {
                        h6014
                    }
                }), Adt12::C1(0u64), Adt13::C0(1u64), 4u64, 4u64)
            } else {
                Adt9::C7(Adt11::C3(0u64, 0u64, false), {
                    let a6015 = v1;
                    let a6016 = 0u64;
                    if if if nat_eq(a6015, a6016) {
                        true
                    } else {
                        let a6017 = v1;
                        let a6018 = 3u64;
                        nat_eq(a6017, a6018)
                    } {
                        true
                    } else {
                        let a6019 = v1;
                        let a6020 = 1u64;
                        if nat_eq(a6019, a6020) {
                            let a6022 = match v2.clone() {
                                Adt1::C0(_, _, _, _, h6021, _, _, _, _, _, _, _) => {
                                    h6021
                                }
                            };
                            let a6023 = 2147483648u64;
                            nat_lt(a6022, a6023)
                        } else {
                            false
                        }
                    } {
                        Adt12::C1(0u64)
                    } else {
                        Adt12::C2(match v2 {
                            Adt1::C0(_, _, _, _, h6024, _, _, _, _, _, _, _) => {
                                h6024
                            }
                        })
                    }
                }, Adt12::C2(13u64), Adt13::C0(1u64), 4u64, 4u64)
            }
        } else {
            let a6025 = v0;
            let a6026 = 4u64;
            if nat_eq(a6025, a6026) {
                Adt9::C7({
                    let a6027 = v1;
                    let a6028 = 5u64;
                    if nat_eq(a6027, a6028) {
                        Adt11::C1(2u64)
                    } else {
                        Adt11::C1(1u64)
                    }
                }, Adt12::C1(1u64), Adt12::C1(0u64), Adt13::C0(1u64), 4u64, 4u64)
            } else {
                let a6029 = v0;
                let a6030 = 5u64;
                if nat_eq(a6029, a6030) {
                    let a6031 = v1;
                    let a6032 = 5u64;
                    if nat_eq(a6031, a6032) {
                        Adt9::C7(Adt11::C5(3u64), Adt12::C1(1u64), Adt12::C2(0u64), Adt13::C0(1u64), 4u64, 4u64)
                    } else {
                        Adt9::C7(Adt11::C1(2u64), Adt12::C1(1u64), Adt12::C2(match v2 {
                            Adt1::C0(_, h6033, _, _, _, _, _, _, _, _, _, _) => {
                                h6033
                            }
                        }), Adt13::C0(1u64), 4u64, 4u64)
                    }
                } else {
                    let a6034 = v0;
                    let a6035 = 6u64;
                    if nat_eq(a6034, a6035) {
                        Adt9::C7(Adt11::C1(2u64), Adt12::C2(match v2 {
                            Adt1::C0(_, _, _, h6036, _, _, _, _, _, _, _, _) => {
                                h6036
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

pub fn f195(v0: u64) -> R<u64> {
    let a6039 = v0;
    let a6040 = {
        let a6037 = v0;
        let a6038 = 2u64;
        nat_add(a6037, a6038)?
    };
    nat_mul(a6039, a6040)
}

pub fn f196(v0: u64) -> R<u64> {
    let a6043 = 3u64;
    let a6044 = {
        let a6041 = v0;
        let a6042 = 3u64;
        nat_mul(a6041, a6042)?
    };
    nat_add(a6043, a6044)
}

pub fn f197(v0: u64, v1: u64) -> R<u64> {
    if f198(v0) {
        let a6045 = 2u64;
        let a6046 = v1;
        nat_mul(a6045, a6046)
    } else {
        let a6047 = v1;
        let a6048 = f273(v0);
        let a6049 = 0u64;
        Ok(nat_quot(a6047, a6048, a6049))
    }
}

pub fn f198(v0: u64) -> bool {
    let a6050 = v0;
    let a6051 = 0u64;
    if if nat_eq(a6050, a6051) {
        true
    } else {
        let a6052 = v0;
        let a6053 = 1u64;
        nat_eq(a6052, a6053)
    } {
        true
    } else {
        let a6054 = v0;
        let a6055 = 30u64;
        nat_eq(a6054, a6055)
    }
}

pub fn f199(v0: u64, v1: u64, v2: u64, v3: u64, v4: Adt3, v5: Adt3) -> R<Adt9> {
    let m6056 = f235(v0);
    match m6056 {
        Adt18::C0 => {
            Ok(Adt9::C0(Adt10::C1))
        }
        Adt18::C1(v6) => {
            Ok(Adt9::C10(v0, f158(v2, v1, 0u64, 0u64, match v4.clone() {
                Adt3::C0(_, _, _, _, h6057, _, _, _) => {
                    h6057
                }
            }, match v4.clone() {
                Adt3::C0(_, _, _, _, _, h6058, _, _) => {
                    h6058
                }
            }, match v4.clone() {
                Adt3::C0(_, _, _, _, _, _, h6059, _) => {
                    h6059
                }
            }, match v4 {
                Adt3::C0(_, _, _, _, _, _, _, h6060) => {
                    h6060
                }
            })?, match v6.clone() {
                Adt19::C0(_, _, h6061) => {
                    h6061
                }
            }, f158({
                let a6063 = v2;
                let a6064 = match v6.clone() {
                    Adt19::C0(_, h6062, _) => {
                        h6062
                    }
                };
                nat_mul(a6063, a6064)?
            }, v3, 0u64, 0u64, match v5.clone() {
                Adt3::C0(_, _, _, _, h6065, _, _, _) => {
                    h6065
                }
            }, match v5.clone() {
                Adt3::C0(_, _, _, _, _, h6066, _, _) => {
                    h6066
                }
            }, match v5.clone() {
                Adt3::C0(_, _, _, _, _, _, h6067, _) => {
                    h6067
                }
            }, match v5.clone() {
                Adt3::C0(_, _, _, _, _, _, _, h6068) => {
                    h6068
                }
            })?, match v6 {
                Adt19::C0(_, h6069, _) => {
                    h6069
                }
            }, match v5 {
                Adt3::C0(_, _, _, _, h6070, _, _, _) => {
                    h6070
                }
            }))
        }
    }
}

pub fn f200(v0: Adt3, v1: Adt3) -> R<u64> {
    let a6078 = {
        let a6071 = 2u64;
        let a6072 = f201(v1.clone())?;
        nat_mul(a6071, a6072)?
    };
    let a6079 = {
        let a6076 = f201(v1)?;
        let a6077 = {
            let a6074 = 1u64;
            let a6075 = match v0 {
                Adt3::C0(h6073, _, _, _, _, _, _, _) => {
                    h6073
                }
            };
            nat_add(a6074, a6075)?
        };
        nat_mul(a6076, a6077)?
    };
    nat_add(a6078, a6079)
}

pub fn f201(v0: Adt3) -> R<u64> {
    if f55(match v0.clone() {
        Adt3::C0(h6080, _, _, _, _, _, _, _) => {
            h6080
        }
    }, match v0.clone() {
        Adt3::C0(_, h6081, _, _, _, _, _, _) => {
            h6081
        }
    }, match v0.clone() {
        Adt3::C0(_, _, h6082, _, _, _, _, _) => {
            h6082
        }
    }, match v0.clone() {
        Adt3::C0(_, _, _, h6083, _, _, _, _) => {
            h6083
        }
    }) {
        Ok(0u64)
    } else {
        let a6092 = match v0.clone() {
            Adt3::C0(h6084, _, _, _, _, _, _, _) => {
                h6084
            }
        };
        let a6093 = {
            let a6090 = match v0.clone() {
                Adt3::C0(_, h6085, _, _, _, _, _, _) => {
                    h6085
                }
            };
            let a6091 = {
                let a6088 = match v0.clone() {
                    Adt3::C0(_, _, h6086, _, _, _, _, _) => {
                        h6086
                    }
                };
                let a6089 = match v0 {
                    Adt3::C0(_, _, _, h6087, _, _, _, _) => {
                        h6087
                    }
                };
                nat_mul(a6088, a6089)?
            };
            nat_mul(a6090, a6091)?
        };
        nat_mul(a6092, a6093)
    }
}

pub fn f202(v0: u64, v1: Adt3) -> R<Adt9> {
    Ok(Adt9::C4(19u64, 4u64, f105(v0, v1)?, 2u64, 0u64, 1u64, 2u64))
}

pub fn f203(v0: u64, v1: u64) -> bool {
    let a6094 = v1;
    let a6095 = 2147483647u64;
    if nat_le(a6094, a6095) {
        let a6096 = v1;
        let a6097 = v0;
        nat_lt(a6096, a6097)
    } else {
        false
    }
}

pub fn f204(v0: u64, v1: u64, v2: u64, v3: Adt3, v4: Adt3, v5: Adt3, v6: u64) -> R<Adt9> {
    let m6098 = f235(v0);
    match m6098 {
        Adt18::C0 => {
            Ok(Adt9::C0(Adt10::C1))
        }
        Adt18::C1(v7) => {
            let v8: u64 = {
                let a6100 = v2;
                let a6101 = match v7.clone() {
                    Adt19::C0(_, h6099, _) => {
                        h6099
                    }
                };
                nat_mul(a6100, a6101)?
            };
            let m6114 = f274(v0, v8, v6, v1, match v5.clone() {
                Adt3::C0(h6102, _, _, _, _, _, _, _) => {
                    h6102
                }
            }, match v5.clone() {
                Adt3::C0(_, h6103, _, _, _, _, _, _) => {
                    h6103
                }
            }, match v5 {
                Adt3::C0(_, _, h6104, _, _, _, _, _) => {
                    h6104
                }
            }, {
                let a6107 = match v3.clone() {
                    Adt3::C0(h6105, _, _, _, _, _, _, _) => {
                        h6105
                    }
                };
                let a6108 = match v7.clone() {
                    Adt19::C0(_, h6106, _) => {
                        h6106
                    }
                };
                nat_mul(a6107, a6108)?
            }, match v3.clone() {
                Adt3::C0(_, h6109, _, _, _, _, _, _) => {
                    h6109
                }
            }, match v3.clone() {
                Adt3::C0(_, _, _, _, h6110, _, _, _) => {
                    h6110
                }
            }, match v3.clone() {
                Adt3::C0(_, _, _, _, _, h6111, _, _) => {
                    h6111
                }
            }, match v3.clone() {
                Adt3::C0(_, _, _, _, _, _, h6112, _) => {
                    h6112
                }
            }, match v3 {
                Adt3::C0(_, _, _, _, _, _, _, h6113) => {
                    h6113
                }
            })?;
            match m6114 {
                None => {
                    Ok(Adt9::C0(Adt10::C1))
                }
                Some(v9) => {
                    let a6115 = v0;
                    let a6116 = 0u64;
                    if if nat_eq(a6115, a6116) {
                        true
                    } else {
                        let a6117 = v0;
                        let a6118 = 26u64;
                        nat_eq(a6117, a6118)
                    } {
                        Ok(Adt9::C1(false, v9, f105({
                            let a6122 = {
                                let a6120 = v1;
                                let a6121 = match v4.clone() {
                                    Adt3::C0(h6119, _, _, _, _, _, _, _) => {
                                        h6119
                                    }
                                };
                                nat_mul(a6120, a6121)?
                            };
                            let a6123 = v8;
                            nat_add(a6122, a6123)?
                        }, v4)?))
                    } else {
                        Ok(Adt9::C5(v0, v9, match v7.clone() {
                            Adt19::C0(_, _, h6124) => {
                                h6124
                            }
                        }, f105({
                            let a6128 = {
                                let a6126 = v1;
                                let a6127 = match v4.clone() {
                                    Adt3::C0(h6125, _, _, _, _, _, _, _) => {
                                        h6125
                                    }
                                };
                                nat_mul(a6126, a6127)?
                            };
                            let a6129 = v8;
                            nat_add(a6128, a6129)?
                        }, v4.clone())?, {
                            let a6136 = {
                                let a6134 = {
                                    let a6131 = match v7 {
                                        Adt19::C0(_, h6130, _) => {
                                            h6130
                                        }
                                    };
                                    let a6132 = 1u64;
                                    nat_sub(a6131, a6132)
                                };
                                let a6135 = match v4.clone() {
                                    Adt3::C0(_, _, _, _, h6133, _, _, _) => {
                                        h6133
                                    }
                                };
                                nat_mul(a6134, a6135)?
                            };
                            let a6137 = 4u64;
                            nat_add(a6136, a6137)?
                        }, match v4 {
                            Adt3::C0(_, _, _, _, h6138, _, _, _) => {
                                h6138
                            }
                        }))
                    }
                }
            }
        }
    }
}

pub fn f205(v0: u64, v1: u64, v2: u64) -> u64 {
    let a6139 = v1;
    let a6140 = 0u64;
    if if nat_eq(a6139, a6140) {
        true
    } else {
        let a6141 = v2;
        let a6142 = 0u64;
        nat_eq(a6141, a6142)
    } {
        0u64
    } else {
        v0
    }
}

pub fn f206(v0: u64, v1: u64) -> bool {
    let a6143 = v0;
    let a6144 = 4294967295u64;
    if nat_eq(a6143, a6144) {
        true
    } else {
        let a6145 = v0;
        let a6146 = 2147483648u64;
        if nat_lt(a6145, a6146) {
            let a6147 = v0;
            let a6148 = v1;
            nat_lt(a6147, a6148)
        } else {
            false
        }
    }
}

pub fn f207(v0: Adt7) -> R<u64> {
    let a6160 = if match v0.clone() {
        Adt7::C0(_, _, _, h6149, _) => {
            h6149
        }
    } {
        2u64
    } else {
        3u64
    };
    let a6161 = {
        let a6158 = {
            let a6156 = {
                let m6152 = match v0.clone() {
                    Adt7::C0(h6151, _, _, _, _) => {
                        h6151
                    }
                };
                match m6152 {
                    Adt6::C0(h6150, _, _, _) => {
                        h6150
                    }
                }
            };
            let a6157 = {
                let m6155 = match v0.clone() {
                    Adt7::C0(_, h6154, _, _, _) => {
                        h6154
                    }
                };
                match m6155 {
                    Adt6::C0(h6153, _, _, _) => {
                        h6153
                    }
                }
            };
            nat_mul(a6156, a6157)?
        };
        let a6159 = f208(v0)?;
        nat_mul(a6158, a6159)?
    };
    nat_add(a6160, a6161)
}

pub fn f208(v0: Adt7) -> R<u64> {
    let a6166 = if match v0.clone() {
        Adt7::C0(_, _, _, h6162, _) => {
            h6162
        }
    } {
        2u64
    } else {
        1u64
    };
    let a6167 = {
        let a6164 = match v0 {
            Adt7::C0(_, _, _, _, h6163) => {
                h6163
            }
        };
        let a6165 = 1u64;
        if nat_eq(a6164, a6165) {
            1u64
        } else {
            0u64
        }
    };
    nat_add(a6166, a6167)
}

pub fn f209(v0: Adt6, v1: u64, v2: u64) -> R<u64> {
    let v3: u64 = {
        let a6172 = {
            let a6169 = v2;
            let a6170 = match v0.clone() {
                Adt6::C0(_, h6168, _, _) => {
                    h6168
                }
            };
            nat_mul(a6169, a6170)?
        };
        let a6173 = f159(match v0.clone() {
            Adt6::C0(_, _, h6171, _) => {
                h6171
            }
        });
        nat_add(a6172, a6173)?
    };
    let v4: u64 = f160(match v0.clone() {
        Adt6::C0(_, _, h6174, _) => {
            h6174
        }
    });
    let v5: u64 = {
        let a6178 = {
            let a6176 = v3;
            let a6177 = match v0 {
                Adt6::C0(h6175, _, _, _) => {
                    h6175
                }
            };
            nat_add(a6176, a6177)?
        };
        let a6179 = v4;
        nat_sub(a6178, a6179)
    };
    let a6184 = {
        let a6180 = v5;
        let a6181 = v1;
        if nat_lt(a6180, a6181) {
            v5
        } else {
            v1
        }
    };
    let a6185 = {
        let a6182 = v3;
        let a6183 = v4;
        nat_sub(a6182, a6183)
    };
    Ok(nat_sub(a6184, a6185))
}

pub fn f210(v0: u64) -> R<u64> {
    let a6188 = 2u64;
    let a6189 = {
        let a6186 = v0;
        let a6187 = 2u64;
        nat_mul(a6186, a6187)?
    };
    nat_add(a6188, a6189)
}

pub fn f211(v0: Adt6) -> R<u64> {
    let a6202 = 2u64;
    let a6203 = {
        let a6200 = 2u64;
        let a6201 = {
            let a6197 = {
                let a6194 = match v0.clone() {
                    Adt6::C0(h6190, _, _, _) => {
                        h6190
                    }
                };
                let a6195 = {
                    let a6192 = match v0.clone() {
                        Adt6::C0(_, h6191, _, _) => {
                            h6191
                        }
                    };
                    let a6193 = 1u64;
                    nat_sub(a6192, a6193)
                };
                nat_add(a6194, a6195)?
            };
            let a6198 = match v0 {
                Adt6::C0(_, h6196, _, _) => {
                    h6196
                }
            };
            let a6199 = 0u64;
            nat_quot(a6197, a6198, a6199)
        };
        nat_mul(a6200, a6201)?
    };
    nat_add(a6202, a6203)
}

pub fn f212(v0: Adt16, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64, v7: u64, v8: u64, v9: u64, v10: u64, v11: u64, v12: u64, v13: u64, v14: u64, v15: u64, v16: u64, v17: u64, v18: u64) -> R<Adt17> {
    let m6204 = v0;
    match m6204 {
        Adt16::C0(v19) => {
            f275(v1, v2, v3, v4, v5, v6, v7, v8, v9, v14, v15, v16, v17, v19, v18)
        }
        Adt16::C1(v20, v21, v22, v23) => {
            let a6205 = v20;
            let a6206 = 66u64;
            if nat_eq(a6205, a6206) {
                f276(v1, v2, v3, v4, v5, v6, v7, v8, v9, v10, v11, v12, v13, v14, v15, v16, v17, v21, v22, v23, v18)
            } else {
                f277(v20, v1, v2, v3, v4, v5, v6, v7, v8, v9, v14, v15, v16, v17, v21, v22, v23, v18)
            }
        }
    }
}

pub fn f213(v0: u64, v1: Adt3, v2: u64, v3: u64, v4: u64) -> R<Adt9> {
    let v5: u64 = {
        let a6213 = v3;
        let a6214 = {
            let a6210 = f105({
                let a6207 = v0;
                let a6208 = 2u64;
                let a6209 = 0u64;
                nat_quot(a6207, a6208, a6209)
            }, v1)?;
            let a6211 = v2;
            let a6212 = 0u64;
            nat_quot(a6210, a6211, a6212)
        };
        nat_add(a6213, a6214)?
    };
    let a6218 = {
        let a6215 = v0;
        let a6216 = 2u64;
        let a6217 = 0u64;
        nat_rem(a6215, a6216, a6217)
    };
    let a6219 = 0u64;
    if nat_eq(a6218, a6219) {
        Ok(Adt9::C4(19u64, 1u64, v5, 2u64, 0u64, 1u64, 2u64))
    } else {
        let a6220 = v4;
        let a6221 = 0u64;
        if nat_eq(a6220, a6221) {
            Ok(Adt9::C4(19u64, 2u64, 1u64, 2u64, 0u64, 1u64, v5))
        } else {
            Ok(Adt9::C0(Adt10::C1))
        }
    }
}

pub fn f214(v0: u64, v1: u64) -> R<u64> {
    let a6224 = 8u64;
    let a6225 = {
        let a6222 = f278(v0);
        let a6223 = v1;
        nat_mul(a6222, a6223)?
    };
    nat_add(a6224, a6225)
}

pub fn f215(v0: u64, v1: u64, v2: Adt3, v3: u64, v4: u64, v5: u64) -> R<Adt9> {
    let v6: u64 = match v2.clone() {
        Adt3::C0(h6226, _, _, _, _, _, _, _) => {
            h6226
        }
    };
    let v7: u64 = f278(v3);
    let v8: u64 = {
        let a6229 = 1u64;
        let a6230 = {
            let a6227 = v6;
            let a6228 = v7;
            nat_mul(a6227, a6228)?
        };
        nat_add(a6229, a6230)?
    };
    let a6231 = v0;
    let a6232 = 0u64;
    if nat_eq(a6231, a6232) {
        Ok(f279(0u64))
    } else {
        let a6233 = v0;
        let a6234 = v8;
        if nat_lt(a6233, a6234) {
            let v9: u64 = {
                let a6235 = v0;
                let a6236 = 1u64;
                nat_sub(a6235, a6236)
            };
            let v10: u64 = {
                let a6237 = v9;
                let a6238 = v7;
                let a6239 = 0u64;
                nat_rem(a6237, a6238, a6239)
            };
            let v11: u64 = f105({
                let a6245 = {
                    let a6240 = v1;
                    let a6241 = v6;
                    nat_mul(a6240, a6241)?
                };
                let a6246 = {
                    let a6242 = v9;
                    let a6243 = v7;
                    let a6244 = 0u64;
                    nat_quot(a6242, a6243, a6244)
                };
                nat_add(a6245, a6246)?
            }, v2)?;
            let a6247 = v3;
            let a6248 = 30u64;
            if nat_eq(a6247, a6248) {
                let a6249 = v10;
                let a6250 = 0u64;
                if nat_eq(a6249, a6250) {
                    Ok(Adt9::C4(6u64, 0u64, v11, 2u64, 0u64, 1u64, 1u64))
                } else {
                    let a6251 = v10;
                    let a6252 = 1u64;
                    if nat_eq(a6251, a6252) {
                        Ok(Adt9::C4(10u64, 1u64, 1u64, 1u64, 1u64, 1u64, 1u64))
                    } else {
                        let a6253 = v10;
                        let a6254 = 2u64;
                        if nat_eq(a6253, a6254) {
                            Ok(Adt9::C4(1u64, 1u64, 1u64, 2u64, 0u64, 1u64, 1u64))
                        } else {
                            Ok(Adt9::C4(0u64, 1u64, 0u64, 1u64, 1u64, 1u64, 0u64))
                        }
                    }
                }
            } else {
                let a6255 = v10;
                let a6256 = 0u64;
                if nat_eq(a6255, a6256) {
                    Ok(Adt9::C4(10u64, 0u64, v11, 0u64, v11, 1u64, 1u64))
                } else {
                    let a6257 = v10;
                    let a6258 = 1u64;
                    if nat_eq(a6257, a6258) {
                        Ok(Adt9::C4(1u64, 1u64, 1u64, 2u64, 0u64, 1u64, 1u64))
                    } else {
                        Ok(Adt9::C4(0u64, 1u64, 0u64, 1u64, 1u64, 1u64, 0u64))
                    }
                }
            }
        } else {
            let v12: u64 = {
                let a6259 = v0;
                let a6260 = v8;
                nat_sub(a6259, a6260)
            };
            let a6261 = v12;
            let a6262 = 0u64;
            if nat_eq(a6261, a6262) {
                Ok(Adt9::C4(22u64, 2u64, v6, 2u64, 0u64, 1u64, 1u64))
            } else {
                let a6263 = v12;
                let a6264 = 1u64;
                if nat_eq(a6263, a6264) {
                    Ok(Adt9::C4(21u64, 1u64, 0u64, 1u64, 1u64, 1u64, 1u64))
                } else {
                    let a6265 = v12;
                    let a6266 = 2u64;
                    if nat_eq(a6265, a6266) {
                        Ok(Adt9::C4(2u64, 1u64, 1u64, 2u64, 0u64, 1u64, 1u64))
                    } else {
                        let a6267 = v12;
                        let a6268 = 3u64;
                        if nat_eq(a6267, a6268) {
                            Ok(Adt9::C4(4u64, 1u64, 1u64, 2u64, v4, 1u64, 1u64))
                        } else {
                            let a6269 = v12;
                            let a6270 = 4u64;
                            if nat_eq(a6269, a6270) {
                                Ok(Adt9::C4(12u64, 1u64, 1u64, 2u64, 0u64, 1u64, 1u64))
                            } else {
                                let a6271 = v12;
                                let a6272 = 5u64;
                                if nat_eq(a6271, a6272) {
                                    Ok(Adt9::C4(11u64, 2u64, 1065353216u64, 1u64, 1u64, 1u64, 2u64))
                                } else {
                                    let a6273 = v12;
                                    let a6274 = 6u64;
                                    if nat_eq(a6273, a6274) {
                                        if f280(v5) {
                                            Ok(Adt9::C4(19u64, 1u64, 2u64, 2u64, 0u64, 1u64, {
                                                let a6275 = 3u64;
                                                let a6276 = v1;
                                                nat_add(a6275, a6276)?
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

pub fn f216(v0: u64, v1: Adt3, v2: Adt3, v3: u64) -> R<Adt9> {
    let v4: u64 = {
        let a6277 = v0;
        let a6278 = f118(v3);
        let a6279 = 0u64;
        nat_quot(a6277, a6278, a6279)
    };
    let v5: u64 = {
        let a6284 = 3u64;
        let a6285 = {
            let a6281 = v4;
            let a6282 = match v1.clone() {
                Adt3::C0(h6280, _, _, _, _, _, _, _) => {
                    h6280
                }
            };
            let a6283 = 0u64;
            nat_quot(a6281, a6282, a6283)
        };
        nat_add(a6284, a6285)?
    };
    let v6: u64 = f105(v4, v1)?;
    let v7: u64 = f105(v4, v2)?;
    let a6286 = v3;
    let a6287 = 30u64;
    if nat_eq(a6286, a6287) {
        let a6291 = {
            let a6288 = v0;
            let a6289 = 3u64;
            let a6290 = 0u64;
            nat_rem(a6288, a6289, a6290)
        };
        let a6292 = 0u64;
        if nat_eq(a6291, a6292) {
            Ok(Adt9::C4(6u64, 0u64, v6, 2u64, 0u64, 1u64, 1u64))
        } else {
            let a6296 = {
                let a6293 = v0;
                let a6294 = 3u64;
                let a6295 = 0u64;
                nat_rem(a6293, a6294, a6295)
            };
            let a6297 = 1u64;
            if nat_eq(a6296, a6297) {
                Ok(Adt9::C4(10u64, 1u64, 1u64, 1u64, v5, 1u64, 1u64))
            } else {
                Ok(Adt9::C4(7u64, 1u64, 1u64, 2u64, 0u64, 3u64, v7))
            }
        }
    } else {
        Ok(Adt9::C4(10u64, 0u64, v6, 1u64, v5, 3u64, v7))
    }
}

pub fn f217(v0: u64) -> R<u64> {
    let v1: u64 = {
        let a6298 = v0;
        let a6299 = 8388608u64;
        let a6300 = 0u64;
        nat_quot(a6298, a6299, a6300)
    };
    let a6301 = v0;
    let a6302 = 2147483648u64;
    if if if nat_lt(a6301, a6302) {
        let a6303 = 127u64;
        let a6304 = v1;
        nat_le(a6303, a6304)
    } else {
        false
    } {
        let a6305 = v1;
        let a6306 = 190u64;
        nat_lt(a6305, a6306)
    } else {
        false
    } {
        let v2: u64 = {
            let a6310 = 8388608u64;
            let a6311 = {
                let a6307 = v0;
                let a6308 = 8388608u64;
                let a6309 = 0u64;
                nat_rem(a6307, a6308, a6309)
            };
            nat_add(a6310, a6311)?
        };
        let a6312 = v1;
        let a6313 = 150u64;
        if nat_lt(a6312, a6313) {
            let v3: u64 = f281({
                let a6314 = 150u64;
                let a6315 = v1;
                nat_sub(a6314, a6315)
            });
            let a6319 = {
                let a6316 = v2;
                let a6317 = v3;
                let a6318 = 0u64;
                nat_rem(a6316, a6317, a6318)
            };
            let a6320 = 0u64;
            if nat_eq(a6319, a6320) {
                let a6321 = v2;
                let a6322 = v3;
                let a6323 = 0u64;
                Ok(nat_quot(a6321, a6322, a6323))
            } else {
                Ok(0u64)
            }
        } else {
            let a6326 = f281({
                let a6324 = v1;
                let a6325 = 150u64;
                nat_sub(a6324, a6325)
            });
            let a6327 = v2;
            nat_mul(a6326, a6327)
        }
    } else {
        Ok(0u64)
    }
}

pub fn f218(v0: Adt14, v1: Adt14) -> bool {
    let a6330 = match v0.clone() {
        Adt14::C0(_, h6328, _, _, _, _, _, _, _, _, _) => {
            h6328
        }
    };
    let a6331 = match v1.clone() {
        Adt14::C0(_, h6329, _, _, _, _, _, _, _, _, _) => {
            h6329
        }
    };
    if if if nat_eq(a6330, a6331) {
        let a6334 = match v0.clone() {
            Adt14::C0(_, _, h6332, _, _, _, _, _, _, _, _) => {
                h6332
            }
        };
        let a6335 = match v1.clone() {
            Adt14::C0(_, _, h6333, _, _, _, _, _, _, _, _) => {
                h6333
            }
        };
        nat_eq(a6334, a6335)
    } else {
        false
    } {
        let a6338 = match v0.clone() {
            Adt14::C0(_, _, _, h6336, _, _, _, _, _, _, _) => {
                h6336
            }
        };
        let a6339 = match v1.clone() {
            Adt14::C0(_, _, _, h6337, _, _, _, _, _, _, _) => {
                h6337
            }
        };
        nat_eq(a6338, a6339)
    } else {
        false
    } {
        let a6342 = match v0 {
            Adt14::C0(_, _, _, _, h6340, _, _, _, _, _, _) => {
                h6340
            }
        };
        let a6343 = match v1 {
            Adt14::C0(_, _, _, _, h6341, _, _, _, _, _, _) => {
                h6341
            }
        };
        nat_eq(a6342, a6343)
    } else {
        false
    }
}

pub fn f219(v0: u64, v1: u64, v2: u64, v3: u64) -> bool {
    let a6344 = v0;
    let a6345 = 0u64;
    if if if nat_eq(a6344, a6345) {
        true
    } else {
        let a6346 = v1;
        let a6347 = 0u64;
        nat_eq(a6346, a6347)
    } {
        true
    } else {
        let a6348 = v2;
        let a6349 = 0u64;
        nat_eq(a6348, a6349)
    } {
        true
    } else {
        let a6350 = v3;
        let a6351 = 0u64;
        nat_eq(a6350, a6351)
    }
}

pub fn f220(v0: Adt14, v1: Adt14) -> bool {
    if f226(match v0.clone() {
        Adt14::C0(_, _, _, _, _, h6352, _, _, _, _, _) => {
            h6352
        }
    }, match v1 {
        Adt14::C0(_, _, _, _, _, h6353, _, _, _, _, _) => {
            h6353
        }
    }) {
        let a6355 = match v0 {
            Adt14::C0(_, h6354, _, _, _, _, _, _, _, _, _) => {
                h6354
            }
        };
        let a6356 = 2147483647u64;
        nat_le(a6355, a6356)
    } else {
        true
    }
}

pub fn f221(v0: Adt14) -> bool {
    let a6361 = {
        let a6358 = match v0.clone() {
            Adt14::C0(_, _, _, _, _, _, _, _, _, _, h6357) => {
                h6357
            }
        };
        let a6359 = 4u64;
        let a6360 = 0u64;
        nat_rem(a6358, a6359, a6360)
    };
    let a6362 = 0u64;
    if if nat_eq(a6361, a6362) {
        f282(match v0.clone() {
            Adt14::C0(_, h6363, _, _, _, _, _, _, _, _, _) => {
                h6363
            }
        }, match v0.clone() {
            Adt14::C0(_, _, _, _, _, h6364, _, _, _, _, _) => {
                h6364
            }
        })
    } else {
        false
    } {
        f114(match v0.clone() {
            Adt14::C0(_, _, h6365, _, _, _, _, _, _, _, _) => {
                h6365
            }
        }, match v0.clone() {
            Adt14::C0(_, _, _, h6366, _, _, _, _, _, _, _) => {
                h6366
            }
        }, match v0.clone() {
            Adt14::C0(_, _, _, _, h6367, _, _, _, _, _, _) => {
                h6367
            }
        }, match v0.clone() {
            Adt14::C0(_, _, _, _, _, _, h6368, _, _, _, _) => {
                h6368
            }
        }, match v0.clone() {
            Adt14::C0(_, _, _, _, _, _, _, h6369, _, _, _) => {
                h6369
            }
        }, match v0 {
            Adt14::C0(_, _, _, _, _, _, _, _, h6370, _, _) => {
                h6370
            }
        }, 4u64)
    } else {
        false
    }
}

pub fn f222(v0: Adt14, v1: Adt14) -> R<bool> {
    let a6372 = match v0.clone() {
        Adt14::C0(_, _, _, _, _, _, _, _, _, h6371, _) => {
            h6371
        }
    };
    let a6373 = 0u64;
    if nat_eq(a6372, a6373) {
        Ok(true)
    } else {
        let a6375 = match v1.clone() {
            Adt14::C0(_, _, _, _, _, _, _, _, _, h6374, _) => {
                h6374
            }
        };
        let a6376 = 0u64;
        if nat_eq(a6375, a6376) {
            Ok(true)
        } else {
            let a6378 = match v0.clone() {
                Adt14::C0(_, _, _, _, _, _, _, _, _, _, h6377) => {
                    h6377
                }
            };
            let a6379 = 0u64;
            if nat_eq(a6378, a6379) {
                Ok(true)
            } else {
                let a6381 = match v1.clone() {
                    Adt14::C0(_, _, _, _, _, _, _, _, _, _, h6380) => {
                        h6380
                    }
                };
                let a6382 = 0u64;
                if nat_eq(a6381, a6382) {
                    Ok(true)
                } else {
                    let a6388 = {
                        let a6385 = match v0.clone() {
                            Adt14::C0(_, _, _, _, _, _, _, _, _, _, h6383) => {
                                h6383
                            }
                        };
                        let a6386 = match v0.clone() {
                            Adt14::C0(_, _, _, _, _, _, _, _, _, h6384, _) => {
                                h6384
                            }
                        };
                        nat_add(a6385, a6386)?
                    };
                    let a6389 = match v1.clone() {
                        Adt14::C0(_, _, _, _, _, _, _, _, _, _, h6387) => {
                            h6387
                        }
                    };
                    if nat_le(a6388, a6389) {
                        Ok(true)
                    } else {
                        let a6395 = {
                            let a6392 = match v1.clone() {
                                Adt14::C0(_, _, _, _, _, _, _, _, _, _, h6390) => {
                                    h6390
                                }
                            };
                            let a6393 = match v1 {
                                Adt14::C0(_, _, _, _, _, _, _, _, _, h6391, _) => {
                                    h6391
                                }
                            };
                            nat_add(a6392, a6393)?
                        };
                        let a6396 = match v0 {
                            Adt14::C0(_, _, _, _, _, _, _, _, _, _, h6394) => {
                                h6394
                            }
                        };
                        Ok(nat_le(a6395, a6396))
                    }
                }
            }
        }
    }
}

pub fn f223(v0: Adt14, v1: Adt14) -> bool {
    if if if if if if f218(v0.clone(), v1.clone()) {
        let a6399 = match v0.clone() {
            Adt14::C0(h6397, _, _, _, _, _, _, _, _, _, _) => {
                h6397
            }
        };
        let a6400 = match v1.clone() {
            Adt14::C0(h6398, _, _, _, _, _, _, _, _, _, _) => {
                h6398
            }
        };
        nat_eq(a6399, a6400)
    } else {
        false
    } {
        let a6403 = match v0.clone() {
            Adt14::C0(_, _, _, _, _, _, _, _, _, _, h6401) => {
                h6401
            }
        };
        let a6404 = match v1.clone() {
            Adt14::C0(_, _, _, _, _, _, _, _, _, _, h6402) => {
                h6402
            }
        };
        nat_eq(a6403, a6404)
    } else {
        false
    } {
        let a6407 = match v0.clone() {
            Adt14::C0(_, _, _, _, _, h6405, _, _, _, _, _) => {
                h6405
            }
        };
        let a6408 = match v1.clone() {
            Adt14::C0(_, _, _, _, _, h6406, _, _, _, _, _) => {
                h6406
            }
        };
        nat_eq(a6407, a6408)
    } else {
        false
    } {
        let a6411 = match v0.clone() {
            Adt14::C0(_, _, _, _, _, _, h6409, _, _, _, _) => {
                h6409
            }
        };
        let a6412 = match v1.clone() {
            Adt14::C0(_, _, _, _, _, _, h6410, _, _, _, _) => {
                h6410
            }
        };
        nat_eq(a6411, a6412)
    } else {
        false
    } {
        let a6415 = match v0.clone() {
            Adt14::C0(_, _, _, _, _, _, _, h6413, _, _, _) => {
                h6413
            }
        };
        let a6416 = match v1.clone() {
            Adt14::C0(_, _, _, _, _, _, _, h6414, _, _, _) => {
                h6414
            }
        };
        nat_eq(a6415, a6416)
    } else {
        false
    } {
        let a6419 = match v0 {
            Adt14::C0(_, _, _, _, _, _, _, _, h6417, _, _) => {
                h6417
            }
        };
        let a6420 = match v1 {
            Adt14::C0(_, _, _, _, _, _, _, _, h6418, _, _) => {
                h6418
            }
        };
        nat_eq(a6419, a6420)
    } else {
        false
    }
}

pub fn f224(v0: Adt14) -> R<bool> {
    if f283(match v0.clone() {
        Adt14::C0(_, h6421, _, _, _, _, _, _, _, _, _) => {
            h6421
        }
    }, match v0.clone() {
        Adt14::C0(_, _, _, _, _, h6422, _, _, _, _, _) => {
            h6422
        }
    }) {
        f115(f284(match v0.clone() {
            Adt14::C0(_, h6423, _, _, _, _, _, _, _, _, _) => {
                h6423
            }
        }, match v0.clone() {
            Adt14::C0(_, _, _, _, _, h6424, _, _, _, _, _) => {
                h6424
            }
        })?, match v0.clone() {
            Adt14::C0(_, _, h6425, _, _, _, _, _, _, _, _) => {
                h6425
            }
        }, match v0.clone() {
            Adt14::C0(_, _, _, h6426, _, _, _, _, _, _, _) => {
                h6426
            }
        }, match v0.clone() {
            Adt14::C0(_, _, _, _, h6427, _, _, _, _, _, _) => {
                h6427
            }
        }, match v0.clone() {
            Adt14::C0(_, _, _, _, _, _, h6428, _, _, _, _) => {
                h6428
            }
        }, match v0.clone() {
            Adt14::C0(_, _, _, _, _, _, _, h6429, _, _, _) => {
                h6429
            }
        }, match v0 {
            Adt14::C0(_, _, _, _, _, _, _, _, h6430, _, _) => {
                h6430
            }
        })
    } else {
        Ok(false)
    }
}

pub fn f225(v0: bool, v1: u64, v2: u64, v3: u64) -> R<u64> {
    let a6431 = v1;
    let a6432 = 0u64;
    if if nat_eq(a6431, a6432) {
        true
    } else {
        let a6433 = v2;
        let a6434 = 0u64;
        nat_eq(a6433, a6434)
    } {
        Ok(0u64)
    } else {
        let a6437 = f116(v1, v2, v3)?;
        let a6438 = {
            let a6435 = v2;
            let a6436 = f285(v0, v1)?;
            nat_mul(a6435, a6436)?
        };
        nat_add(a6437, a6438)
    }
}

pub fn f226(v0: u64, v1: u64) -> bool {
    let a6439 = v0;
    let a6440 = 4u64;
    if nat_eq(a6439, a6440) {
        let a6441 = v1;
        let a6442 = 4u64;
        nat_eq(a6441, a6442)
    } else {
        false
    }
}

pub fn f227(v0: u64) -> R<u64> {
    let a6443 = 6u64;
    let a6444 = v0;
    nat_add(a6443, a6444)
}

pub fn f228(v0: Adt14) -> Adt14 {
    Adt14::C0(match v0.clone() {
        Adt14::C0(h6445, _, _, _, _, _, _, _, _, _, _) => {
            h6445
        }
    }, match v0.clone() {
        Adt14::C0(_, h6446, _, _, _, _, _, _, _, _, _) => {
            h6446
        }
    }, match v0.clone() {
        Adt14::C0(_, _, h6447, _, _, _, _, _, _, _, _) => {
            h6447
        }
    }, match v0.clone() {
        Adt14::C0(_, _, _, h6448, _, _, _, _, _, _, _) => {
            h6448
        }
    }, match v0.clone() {
        Adt14::C0(_, _, _, _, h6449, _, _, _, _, _, _) => {
            h6449
        }
    }, 4u64, match v0.clone() {
        Adt14::C0(_, _, _, _, _, _, h6450, _, _, _, _) => {
            h6450
        }
    }, match v0.clone() {
        Adt14::C0(_, _, _, _, _, _, _, h6451, _, _, _) => {
            h6451
        }
    }, match v0.clone() {
        Adt14::C0(_, _, _, _, _, _, _, _, h6452, _, _) => {
            h6452
        }
    }, match v0.clone() {
        Adt14::C0(_, _, _, _, _, _, _, _, _, h6453, _) => {
            h6453
        }
    }, match v0 {
        Adt14::C0(_, _, _, _, _, _, _, _, _, _, h6454) => {
            h6454
        }
    })
}

pub fn f229(v0: u64, v1: u64, v2: u64, v3: u64) -> R<bool> {
    let a6455 = v0;
    let a6456 = 2147483647u64;
    if if if nat_le(a6455, a6456) {
        let a6457 = 0u64;
        let a6458 = v3;
        if nat_lt(a6457, a6458) {
            let a6459 = v3;
            let a6460 = 2147483647u64;
            nat_le(a6459, a6460)
        } else {
            false
        }
    } else {
        false
    } {
        let a6465 = {
            let a6463 = {
                let a6461 = v2;
                let a6462 = v3;
                nat_add(a6461, a6462)?
            };
            let a6464 = 1u64;
            nat_sub(a6463, a6464)
        };
        let a6466 = 2147483647u64;
        nat_le(a6465, a6466)
    } else {
        false
    } {
        let a6471 = {
            let a6469 = {
                let a6467 = v0;
                let a6468 = v1;
                nat_mul(a6467, a6468)?
            };
            let a6470 = f286(v2, v3)?;
            nat_mul(a6469, a6470)?
        };
        let a6472 = 9223372036854775807u64;
        Ok(nat_le(a6471, a6472))
    } else {
        Ok(false)
    }
}

pub fn f230(v0: Adt14) -> R<u64> {
    let a6503 = {
        let a6495 = {
            let a6487 = {
                let a6479 = 4u64;
                let a6480 = {
                    let a6477 = {
                        let a6474 = match v0.clone() {
                            Adt14::C0(_, h6473, _, _, _, _, _, _, _, _, _) => {
                                h6473
                            }
                        };
                        let a6475 = 1u64;
                        nat_sub(a6474, a6475)
                    };
                    let a6478 = match v0.clone() {
                        Adt14::C0(_, _, _, _, _, h6476, _, _, _, _, _) => {
                            h6476
                        }
                    };
                    nat_mul(a6477, a6478)?
                };
                nat_add(a6479, a6480)?
            };
            let a6488 = {
                let a6485 = {
                    let a6482 = match v0.clone() {
                        Adt14::C0(_, _, h6481, _, _, _, _, _, _, _, _) => {
                            h6481
                        }
                    };
                    let a6483 = 1u64;
                    nat_sub(a6482, a6483)
                };
                let a6486 = match v0.clone() {
                    Adt14::C0(_, _, _, _, _, _, h6484, _, _, _, _) => {
                        h6484
                    }
                };
                nat_mul(a6485, a6486)?
            };
            nat_add(a6487, a6488)?
        };
        let a6496 = {
            let a6493 = {
                let a6490 = match v0.clone() {
                    Adt14::C0(_, _, _, h6489, _, _, _, _, _, _, _) => {
                        h6489
                    }
                };
                let a6491 = 1u64;
                nat_sub(a6490, a6491)
            };
            let a6494 = match v0.clone() {
                Adt14::C0(_, _, _, _, _, _, _, h6492, _, _, _) => {
                    h6492
                }
            };
            nat_mul(a6493, a6494)?
        };
        nat_add(a6495, a6496)?
    };
    let a6504 = {
        let a6501 = {
            let a6498 = match v0.clone() {
                Adt14::C0(_, _, _, _, h6497, _, _, _, _, _, _) => {
                    h6497
                }
            };
            let a6499 = 1u64;
            nat_sub(a6498, a6499)
        };
        let a6502 = match v0 {
            Adt14::C0(_, _, _, _, _, _, _, _, h6500, _, _) => {
                h6500
            }
        };
        nat_mul(a6501, a6502)?
    };
    nat_add(a6503, a6504)
}

pub fn f231(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64) -> R<u64> {
    let a6505 = v0;
    let a6506 = 0u64;
    if if if if nat_eq(a6505, a6506) {
        true
    } else {
        let a6507 = v1;
        let a6508 = 0u64;
        nat_eq(a6507, a6508)
    } {
        true
    } else {
        let a6509 = v2;
        let a6510 = 0u64;
        nat_eq(a6509, a6510)
    } {
        true
    } else {
        let a6511 = v3;
        let a6512 = 0u64;
        nat_eq(a6511, a6512)
    } {
        Ok(0u64)
    } else {
        let a6513 = f287(v0, v1, v2, v3, v5)?;
        let a6514 = f288(v0, v1, v2, v3, v4)?;
        nat_add(a6513, a6514)
    }
}

pub fn f232(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let a6515 = v0;
    let a6516 = 0u64;
    if if nat_eq(a6515, a6516) {
        true
    } else {
        let a6517 = v1;
        let a6518 = 0u64;
        nat_eq(a6517, a6518)
    } {
        Ok(0u64)
    } else {
        let a6521 = f116(v0, v1, v2)?;
        let a6522 = {
            let a6519 = v1;
            let a6520 = f289(v0)?;
            nat_mul(a6519, a6520)?
        };
        nat_add(a6521, a6522)
    }
}

pub fn f233(v0: u64, v1: u64, v2: u64) -> bool {
    let a6523 = v0;
    let a6524 = 1u64;
    if nat_le(a6523, a6524) {
        true
    } else {
        let a6528 = {
            let a6525 = v1;
            let a6526 = v2;
            let a6527 = 1u64;
            nat_rem(a6525, a6526, a6527)
        };
        let a6529 = 0u64;
        nat_eq(a6528, a6529)
    }
}

pub fn f234(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64) -> R<bool> {
    let a6530 = v1;
    let a6531 = 1u64;
    if if if nat_le(a6530, a6531) {
        true
    } else {
        let a6532 = v0;
        let a6533 = v2;
        nat_le(a6532, a6533)
    } {
        let a6534 = v3;
        let a6535 = 1u64;
        if nat_le(a6534, a6535) {
            true
        } else {
            let a6542 = {
                let a6540 = v0;
                let a6541 = {
                    let a6538 = {
                        let a6536 = v1;
                        let a6537 = 1u64;
                        nat_sub(a6536, a6537)
                    };
                    let a6539 = v2;
                    nat_mul(a6538, a6539)?
                };
                nat_add(a6540, a6541)?
            };
            let a6543 = v4;
            nat_le(a6542, a6543)
        }
    } else {
        false
    } {
        let a6544 = v5;
        let a6545 = 1u64;
        if nat_le(a6544, a6545) {
            Ok(true)
        } else {
            let a6558 = {
                let a6556 = {
                    let a6550 = v0;
                    let a6551 = {
                        let a6548 = {
                            let a6546 = v1;
                            let a6547 = 1u64;
                            nat_sub(a6546, a6547)
                        };
                        let a6549 = v2;
                        nat_mul(a6548, a6549)?
                    };
                    nat_add(a6550, a6551)?
                };
                let a6557 = {
                    let a6554 = {
                        let a6552 = v3;
                        let a6553 = 1u64;
                        nat_sub(a6552, a6553)
                    };
                    let a6555 = v4;
                    nat_mul(a6554, a6555)?
                };
                nat_add(a6556, a6557)?
            };
            let a6559 = v6;
            Ok(nat_le(a6558, a6559))
        }
    } else {
        Ok(false)
    }
}

pub fn f235(v0: u64) -> Adt18 {
    let a6560 = v0;
    let a6561 = 0u64;
    if nat_eq(a6560, a6561) {
        Adt18::C1(Adt19::C0(0u64, 1u64, 4u64))
    } else {
        let a6562 = v0;
        let a6563 = 1u64;
        if nat_eq(a6562, a6563) {
            Adt18::C1(Adt19::C0(1u64, 1u64, 2u64))
        } else {
            let a6564 = v0;
            let a6565 = 2u64;
            if nat_eq(a6564, a6565) {
                Adt18::C1(Adt19::C0(2u64, 32u64, 18u64))
            } else {
                let a6566 = v0;
                let a6567 = 3u64;
                if nat_eq(a6566, a6567) {
                    Adt18::C1(Adt19::C0(3u64, 32u64, 20u64))
                } else {
                    let a6568 = v0;
                    let a6569 = 6u64;
                    if nat_eq(a6568, a6569) {
                        Adt18::C1(Adt19::C0(6u64, 32u64, 22u64))
                    } else {
                        let a6570 = v0;
                        let a6571 = 7u64;
                        if nat_eq(a6570, a6571) {
                            Adt18::C1(Adt19::C0(7u64, 32u64, 24u64))
                        } else {
                            let a6572 = v0;
                            let a6573 = 8u64;
                            if nat_eq(a6572, a6573) {
                                Adt18::C1(Adt19::C0(8u64, 32u64, 34u64))
                            } else {
                                let a6574 = v0;
                                let a6575 = 9u64;
                                if nat_eq(a6574, a6575) {
                                    Adt18::C1(Adt19::C0(9u64, 32u64, 36u64))
                                } else {
                                    let a6576 = v0;
                                    let a6577 = 10u64;
                                    if nat_eq(a6576, a6577) {
                                        Adt18::C1(Adt19::C0(10u64, 256u64, 84u64))
                                    } else {
                                        let a6578 = v0;
                                        let a6579 = 11u64;
                                        if nat_eq(a6578, a6579) {
                                            Adt18::C1(Adt19::C0(11u64, 256u64, 110u64))
                                        } else {
                                            let a6580 = v0;
                                            let a6581 = 12u64;
                                            if nat_eq(a6580, a6581) {
                                                Adt18::C1(Adt19::C0(12u64, 256u64, 144u64))
                                            } else {
                                                let a6582 = v0;
                                                let a6583 = 13u64;
                                                if nat_eq(a6582, a6583) {
                                                    Adt18::C1(Adt19::C0(13u64, 256u64, 176u64))
                                                } else {
                                                    let a6584 = v0;
                                                    let a6585 = 14u64;
                                                    if nat_eq(a6584, a6585) {
                                                        Adt18::C1(Adt19::C0(14u64, 256u64, 210u64))
                                                    } else {
                                                        let a6586 = v0;
                                                        let a6587 = 15u64;
                                                        if nat_eq(a6586, a6587) {
                                                            Adt18::C1(Adt19::C0(15u64, 256u64, 292u64))
                                                        } else {
                                                            let a6588 = v0;
                                                            let a6589 = 16u64;
                                                            if nat_eq(a6588, a6589) {
                                                                Adt18::C1(Adt19::C0(16u64, 256u64, 66u64))
                                                            } else {
                                                                let a6590 = v0;
                                                                let a6591 = 17u64;
                                                                if nat_eq(a6590, a6591) {
                                                                    Adt18::C1(Adt19::C0(17u64, 256u64, 74u64))
                                                                } else {
                                                                    let a6592 = v0;
                                                                    let a6593 = 18u64;
                                                                    if nat_eq(a6592, a6593) {
                                                                        Adt18::C1(Adt19::C0(18u64, 256u64, 98u64))
                                                                    } else {
                                                                        let a6594 = v0;
                                                                        let a6595 = 19u64;
                                                                        if nat_eq(a6594, a6595) {
                                                                            Adt18::C1(Adt19::C0(19u64, 256u64, 50u64))
                                                                        } else {
                                                                            let a6596 = v0;
                                                                            let a6597 = 20u64;
                                                                            if nat_eq(a6596, a6597) {
                                                                                Adt18::C1(Adt19::C0(20u64, 32u64, 18u64))
                                                                            } else {
                                                                                let a6598 = v0;
                                                                                let a6599 = 21u64;
                                                                                if nat_eq(a6598, a6599) {
                                                                                    Adt18::C1(Adt19::C0(21u64, 256u64, 110u64))
                                                                                } else {
                                                                                    let a6600 = v0;
                                                                                    let a6601 = 22u64;
                                                                                    if nat_eq(a6600, a6601) {
                                                                                        Adt18::C1(Adt19::C0(22u64, 256u64, 82u64))
                                                                                    } else {
                                                                                        let a6602 = v0;
                                                                                        let a6603 = 23u64;
                                                                                        if nat_eq(a6602, a6603) {
                                                                                            Adt18::C1(Adt19::C0(23u64, 256u64, 136u64))
                                                                                        } else {
                                                                                            let a6604 = v0;
                                                                                            let a6605 = 24u64;
                                                                                            if nat_eq(a6604, a6605) {
                                                                                                Adt18::C1(Adt19::C0(24u64, 1u64, 1u64))
                                                                                            } else {
                                                                                                let a6606 = v0;
                                                                                                let a6607 = 25u64;
                                                                                                if nat_eq(a6606, a6607) {
                                                                                                    Adt18::C1(Adt19::C0(25u64, 1u64, 2u64))
                                                                                                } else {
                                                                                                    let a6608 = v0;
                                                                                                    let a6609 = 26u64;
                                                                                                    if nat_eq(a6608, a6609) {
                                                                                                        Adt18::C1(Adt19::C0(26u64, 1u64, 4u64))
                                                                                                    } else {
                                                                                                        let a6610 = v0;
                                                                                                        let a6611 = 27u64;
                                                                                                        if nat_eq(a6610, a6611) {
                                                                                                            Adt18::C1(Adt19::C0(27u64, 1u64, 8u64))
                                                                                                        } else {
                                                                                                            let a6612 = v0;
                                                                                                            let a6613 = 28u64;
                                                                                                            if nat_eq(a6612, a6613) {
                                                                                                                Adt18::C1(Adt19::C0(28u64, 1u64, 8u64))
                                                                                                            } else {
                                                                                                                let a6614 = v0;
                                                                                                                let a6615 = 29u64;
                                                                                                                if nat_eq(a6614, a6615) {
                                                                                                                    Adt18::C1(Adt19::C0(29u64, 256u64, 56u64))
                                                                                                                } else {
                                                                                                                    let a6616 = v0;
                                                                                                                    let a6617 = 30u64;
                                                                                                                    if nat_eq(a6616, a6617) {
                                                                                                                        Adt18::C1(Adt19::C0(30u64, 1u64, 2u64))
                                                                                                                    } else {
                                                                                                                        let a6618 = v0;
                                                                                                                        let a6619 = 34u64;
                                                                                                                        if nat_eq(a6618, a6619) {
                                                                                                                            Adt18::C1(Adt19::C0(34u64, 256u64, 54u64))
                                                                                                                        } else {
                                                                                                                            let a6620 = v0;
                                                                                                                            let a6621 = 35u64;
                                                                                                                            if nat_eq(a6620, a6621) {
                                                                                                                                Adt18::C1(Adt19::C0(35u64, 256u64, 66u64))
                                                                                                                            } else {
                                                                                                                                let a6622 = v0;
                                                                                                                                let a6623 = 39u64;
                                                                                                                                if nat_eq(a6622, a6623) {
                                                                                                                                    Adt18::C1(Adt19::C0(39u64, 32u64, 17u64))
                                                                                                                                } else {
                                                                                                                                    let a6624 = v0;
                                                                                                                                    let a6625 = 40u64;
                                                                                                                                    if nat_eq(a6624, a6625) {
                                                                                                                                        Adt18::C1(Adt19::C0(40u64, 64u64, 36u64))
                                                                                                                                    } else {
                                                                                                                                        let a6626 = v0;
                                                                                                                                        let a6627 = 41u64;
                                                                                                                                        if nat_eq(a6626, a6627) {
                                                                                                                                            Adt18::C1(Adt19::C0(41u64, 128u64, 18u64))
                                                                                                                                        } else {
                                                                                                                                            let a6628 = v0;
                                                                                                                                            let a6629 = 42u64;
                                                                                                                                            if nat_eq(a6628, a6629) {
                                                                                                                                                Adt18::C1(Adt19::C0(42u64, 64u64, 18u64))
                                                                                                                                            } else {
                                                                                                                                                let a6630 = v0;
                                                                                                                                                let a6631 = 43u64;
                                                                                                                                                if nat_eq(a6630, a6631) {
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

pub fn f236(v0: u64, v1: u64) -> R<u64> {
    let v2: u64 = {
        let a6632 = v0;
        let a6633 = 2u64;
        let a6634 = 0u64;
        nat_quot(a6632, a6633, a6634)
    };
    let a6650 = {
        let a6635 = v2;
        let a6636 = 0u64;
        if nat_eq(a6635, a6636) {
            0u64
        } else {
            let a6643 = 7u64;
            let a6644 = {
                let a6641 = v2;
                let a6642 = {
                    let a6639 = 5u64;
                    let a6640 = {
                        let a6637 = 3u64;
                        let a6638 = v1;
                        nat_mul(a6637, a6638)?
                    };
                    nat_add(a6639, a6640)?
                };
                nat_mul(a6641, a6642)?
            };
            nat_add(a6643, a6644)?
        }
    };
    let a6651 = {
        let a6648 = {
            let a6645 = v0;
            let a6646 = 2u64;
            let a6647 = 0u64;
            nat_rem(a6645, a6646, a6647)
        };
        let a6649 = 1u64;
        if nat_eq(a6648, a6649) {
            v1
        } else {
            0u64
        }
    };
    nat_add(a6650, a6651)
}

pub fn f237(v0: Adt2, v1: u64, v2: bool, v3: bool) -> R<bool> {
    if v2 {
        if v3 {
            let a6653 = match v0.clone() {
                Adt2::C0(_, _, _, _, h6652, _, _, _, _, _, _) => {
                    h6652
                }
            };
            let a6654 = 1u64;
            if if if if if nat_le(a6653, a6654) {
                true
            } else {
                let a6656 = match v0.clone() {
                    Adt2::C0(_, _, _, _, _, _, _, _, h6655, _, _) => {
                        h6655
                    }
                };
                let a6657 = v1;
                nat_eq(a6656, a6657)
            } {
                let a6659 = match v0.clone() {
                    Adt2::C0(_, h6658, _, _, _, _, _, _, _, _, _) => {
                        h6658
                    }
                };
                let a6660 = 1u64;
                if nat_le(a6659, a6660) {
                    true
                } else {
                    let a6665 = match v0.clone() {
                        Adt2::C0(_, _, _, _, _, h6661, _, _, _, _, _) => {
                            h6661
                        }
                    };
                    let a6666 = {
                        let a6663 = v1;
                        let a6664 = match v0.clone() {
                            Adt2::C0(_, _, _, _, h6662, _, _, _, _, _, _) => {
                                h6662
                            }
                        };
                        nat_mul(a6663, a6664)?
                    };
                    nat_eq(a6665, a6666)
                }
            } else {
                false
            } {
                let a6668 = match v0.clone() {
                    Adt2::C0(_, _, h6667, _, _, _, _, _, _, _, _) => {
                        h6667
                    }
                };
                let a6669 = 1u64;
                if nat_le(a6668, a6669) {
                    true
                } else {
                    let a6677 = match v0.clone() {
                        Adt2::C0(_, _, _, _, _, _, h6670, _, _, _, _) => {
                            h6670
                        }
                    };
                    let a6678 = {
                        let a6675 = {
                            let a6672 = v1;
                            let a6673 = match v0.clone() {
                                Adt2::C0(_, _, _, _, h6671, _, _, _, _, _, _) => {
                                    h6671
                                }
                            };
                            nat_mul(a6672, a6673)?
                        };
                        let a6676 = match v0.clone() {
                            Adt2::C0(_, h6674, _, _, _, _, _, _, _, _, _) => {
                                h6674
                            }
                        };
                        nat_mul(a6675, a6676)?
                    };
                    nat_eq(a6677, a6678)
                }
            } else {
                false
            } {
                let a6680 = match v0.clone() {
                    Adt2::C0(_, _, _, h6679, _, _, _, _, _, _, _) => {
                        h6679
                    }
                };
                let a6681 = 1u64;
                if nat_le(a6680, a6681) {
                    true
                } else {
                    let a6692 = match v0.clone() {
                        Adt2::C0(_, _, _, _, _, _, _, h6682, _, _, _) => {
                            h6682
                        }
                    };
                    let a6693 = {
                        let a6690 = {
                            let a6687 = {
                                let a6684 = v1;
                                let a6685 = match v0.clone() {
                                    Adt2::C0(_, _, _, _, h6683, _, _, _, _, _, _) => {
                                        h6683
                                    }
                                };
                                nat_mul(a6684, a6685)?
                            };
                            let a6688 = match v0.clone() {
                                Adt2::C0(_, h6686, _, _, _, _, _, _, _, _, _) => {
                                    h6686
                                }
                            };
                            nat_mul(a6687, a6688)?
                        };
                        let a6691 = match v0.clone() {
                            Adt2::C0(_, _, h6689, _, _, _, _, _, _, _, _) => {
                                h6689
                            }
                        };
                        nat_mul(a6690, a6691)?
                    };
                    nat_eq(a6692, a6693)
                }
            } else {
                false
            } {
                let a6707 = {
                    let a6704 = {
                        let a6701 = {
                            let a6698 = {
                                let a6695 = v1;
                                let a6696 = match v0.clone() {
                                    Adt2::C0(_, _, _, _, h6694, _, _, _, _, _, _) => {
                                        h6694
                                    }
                                };
                                nat_mul(a6695, a6696)?
                            };
                            let a6699 = match v0.clone() {
                                Adt2::C0(_, h6697, _, _, _, _, _, _, _, _, _) => {
                                    h6697
                                }
                            };
                            nat_mul(a6698, a6699)?
                        };
                        let a6702 = match v0.clone() {
                            Adt2::C0(_, _, h6700, _, _, _, _, _, _, _, _) => {
                                h6700
                            }
                        };
                        nat_mul(a6701, a6702)?
                    };
                    let a6705 = match v0.clone() {
                        Adt2::C0(_, _, _, h6703, _, _, _, _, _, _, _) => {
                            h6703
                        }
                    };
                    nat_mul(a6704, a6705)?
                };
                let a6708 = match v0 {
                    Adt2::C0(_, _, _, _, _, _, _, _, _, h6706, _) => {
                        h6706
                    }
                };
                Ok(nat_le(a6707, a6708))
            } else {
                Ok(false)
            }
        } else {
            let a6710 = match v0.clone() {
                Adt2::C0(_, _, _, h6709, _, _, _, _, _, _, _) => {
                    h6709
                }
            };
            let a6711 = 1u64;
            if if if if if nat_le(a6710, a6711) {
                true
            } else {
                let a6713 = match v0.clone() {
                    Adt2::C0(_, _, _, _, _, _, _, h6712, _, _, _) => {
                        h6712
                    }
                };
                let a6714 = v1;
                nat_eq(a6713, a6714)
            } {
                let a6716 = match v0.clone() {
                    Adt2::C0(_, h6715, _, _, _, _, _, _, _, _, _) => {
                        h6715
                    }
                };
                let a6717 = 1u64;
                if nat_le(a6716, a6717) {
                    true
                } else {
                    let a6722 = match v0.clone() {
                        Adt2::C0(_, _, _, _, _, h6718, _, _, _, _, _) => {
                            h6718
                        }
                    };
                    let a6723 = {
                        let a6720 = v1;
                        let a6721 = match v0.clone() {
                            Adt2::C0(_, _, _, h6719, _, _, _, _, _, _, _) => {
                                h6719
                            }
                        };
                        nat_mul(a6720, a6721)?
                    };
                    nat_eq(a6722, a6723)
                }
            } else {
                false
            } {
                let a6725 = match v0.clone() {
                    Adt2::C0(_, _, h6724, _, _, _, _, _, _, _, _) => {
                        h6724
                    }
                };
                let a6726 = 1u64;
                if nat_le(a6725, a6726) {
                    true
                } else {
                    let a6734 = match v0.clone() {
                        Adt2::C0(_, _, _, _, _, _, h6727, _, _, _, _) => {
                            h6727
                        }
                    };
                    let a6735 = {
                        let a6732 = {
                            let a6729 = v1;
                            let a6730 = match v0.clone() {
                                Adt2::C0(_, _, _, h6728, _, _, _, _, _, _, _) => {
                                    h6728
                                }
                            };
                            nat_mul(a6729, a6730)?
                        };
                        let a6733 = match v0.clone() {
                            Adt2::C0(_, h6731, _, _, _, _, _, _, _, _, _) => {
                                h6731
                            }
                        };
                        nat_mul(a6732, a6733)?
                    };
                    nat_eq(a6734, a6735)
                }
            } else {
                false
            } {
                let a6737 = match v0.clone() {
                    Adt2::C0(_, _, _, _, h6736, _, _, _, _, _, _) => {
                        h6736
                    }
                };
                let a6738 = 1u64;
                if nat_le(a6737, a6738) {
                    true
                } else {
                    let a6749 = match v0.clone() {
                        Adt2::C0(_, _, _, _, _, _, _, _, h6739, _, _) => {
                            h6739
                        }
                    };
                    let a6750 = {
                        let a6747 = {
                            let a6744 = {
                                let a6741 = v1;
                                let a6742 = match v0.clone() {
                                    Adt2::C0(_, _, _, h6740, _, _, _, _, _, _, _) => {
                                        h6740
                                    }
                                };
                                nat_mul(a6741, a6742)?
                            };
                            let a6745 = match v0.clone() {
                                Adt2::C0(_, h6743, _, _, _, _, _, _, _, _, _) => {
                                    h6743
                                }
                            };
                            nat_mul(a6744, a6745)?
                        };
                        let a6748 = match v0.clone() {
                            Adt2::C0(_, _, h6746, _, _, _, _, _, _, _, _) => {
                                h6746
                            }
                        };
                        nat_mul(a6747, a6748)?
                    };
                    nat_eq(a6749, a6750)
                }
            } else {
                false
            } {
                let a6764 = {
                    let a6761 = {
                        let a6758 = {
                            let a6755 = {
                                let a6752 = v1;
                                let a6753 = match v0.clone() {
                                    Adt2::C0(_, _, _, h6751, _, _, _, _, _, _, _) => {
                                        h6751
                                    }
                                };
                                nat_mul(a6752, a6753)?
                            };
                            let a6756 = match v0.clone() {
                                Adt2::C0(_, h6754, _, _, _, _, _, _, _, _, _) => {
                                    h6754
                                }
                            };
                            nat_mul(a6755, a6756)?
                        };
                        let a6759 = match v0.clone() {
                            Adt2::C0(_, _, h6757, _, _, _, _, _, _, _, _) => {
                                h6757
                            }
                        };
                        nat_mul(a6758, a6759)?
                    };
                    let a6762 = match v0.clone() {
                        Adt2::C0(_, _, _, _, h6760, _, _, _, _, _, _) => {
                            h6760
                        }
                    };
                    nat_mul(a6761, a6762)?
                };
                let a6765 = match v0 {
                    Adt2::C0(_, _, _, _, _, _, _, _, _, h6763, _) => {
                        h6763
                    }
                };
                Ok(nat_le(a6764, a6765))
            } else {
                Ok(false)
            }
        }
    } else {
        let a6767 = match v0.clone() {
            Adt2::C0(_, h6766, _, _, _, _, _, _, _, _, _) => {
                h6766
            }
        };
        let a6768 = 1u64;
        if if if if if nat_le(a6767, a6768) {
            true
        } else {
            let a6770 = match v0.clone() {
                Adt2::C0(_, _, _, _, _, h6769, _, _, _, _, _) => {
                    h6769
                }
            };
            let a6771 = v1;
            nat_eq(a6770, a6771)
        } {
            let a6773 = match v0.clone() {
                Adt2::C0(_, _, h6772, _, _, _, _, _, _, _, _) => {
                    h6772
                }
            };
            let a6774 = 1u64;
            if nat_le(a6773, a6774) {
                true
            } else {
                let a6779 = match v0.clone() {
                    Adt2::C0(_, _, _, _, _, _, h6775, _, _, _, _) => {
                        h6775
                    }
                };
                let a6780 = {
                    let a6777 = v1;
                    let a6778 = match v0.clone() {
                        Adt2::C0(_, h6776, _, _, _, _, _, _, _, _, _) => {
                            h6776
                        }
                    };
                    nat_mul(a6777, a6778)?
                };
                nat_eq(a6779, a6780)
            }
        } else {
            false
        } {
            let a6782 = match v0.clone() {
                Adt2::C0(_, _, _, h6781, _, _, _, _, _, _, _) => {
                    h6781
                }
            };
            let a6783 = 1u64;
            if nat_le(a6782, a6783) {
                true
            } else {
                let a6791 = match v0.clone() {
                    Adt2::C0(_, _, _, _, _, _, _, h6784, _, _, _) => {
                        h6784
                    }
                };
                let a6792 = {
                    let a6789 = {
                        let a6786 = v1;
                        let a6787 = match v0.clone() {
                            Adt2::C0(_, h6785, _, _, _, _, _, _, _, _, _) => {
                                h6785
                            }
                        };
                        nat_mul(a6786, a6787)?
                    };
                    let a6790 = match v0.clone() {
                        Adt2::C0(_, _, h6788, _, _, _, _, _, _, _, _) => {
                            h6788
                        }
                    };
                    nat_mul(a6789, a6790)?
                };
                nat_eq(a6791, a6792)
            }
        } else {
            false
        } {
            let a6794 = match v0.clone() {
                Adt2::C0(_, _, _, _, h6793, _, _, _, _, _, _) => {
                    h6793
                }
            };
            let a6795 = 1u64;
            if nat_le(a6794, a6795) {
                true
            } else {
                let a6806 = match v0.clone() {
                    Adt2::C0(_, _, _, _, _, _, _, _, h6796, _, _) => {
                        h6796
                    }
                };
                let a6807 = {
                    let a6804 = {
                        let a6801 = {
                            let a6798 = v1;
                            let a6799 = match v0.clone() {
                                Adt2::C0(_, h6797, _, _, _, _, _, _, _, _, _) => {
                                    h6797
                                }
                            };
                            nat_mul(a6798, a6799)?
                        };
                        let a6802 = match v0.clone() {
                            Adt2::C0(_, _, h6800, _, _, _, _, _, _, _, _) => {
                                h6800
                            }
                        };
                        nat_mul(a6801, a6802)?
                    };
                    let a6805 = match v0.clone() {
                        Adt2::C0(_, _, _, h6803, _, _, _, _, _, _, _) => {
                            h6803
                        }
                    };
                    nat_mul(a6804, a6805)?
                };
                nat_eq(a6806, a6807)
            }
        } else {
            false
        } {
            let a6821 = {
                let a6818 = {
                    let a6815 = {
                        let a6812 = {
                            let a6809 = v1;
                            let a6810 = match v0.clone() {
                                Adt2::C0(_, h6808, _, _, _, _, _, _, _, _, _) => {
                                    h6808
                                }
                            };
                            nat_mul(a6809, a6810)?
                        };
                        let a6813 = match v0.clone() {
                            Adt2::C0(_, _, h6811, _, _, _, _, _, _, _, _) => {
                                h6811
                            }
                        };
                        nat_mul(a6812, a6813)?
                    };
                    let a6816 = match v0.clone() {
                        Adt2::C0(_, _, _, h6814, _, _, _, _, _, _, _) => {
                            h6814
                        }
                    };
                    nat_mul(a6815, a6816)?
                };
                let a6819 = match v0.clone() {
                    Adt2::C0(_, _, _, _, h6817, _, _, _, _, _, _) => {
                        h6817
                    }
                };
                nat_mul(a6818, a6819)?
            };
            let a6822 = match v0 {
                Adt2::C0(_, _, _, _, _, _, _, _, _, h6820, _) => {
                    h6820
                }
            };
            Ok(nat_le(a6821, a6822))
        } else {
            Ok(false)
        }
    }
}

pub fn f238(v0: Adt2, v1: u64) -> R<bool> {
    let a6824 = match v0.clone() {
        Adt2::C0(h6823, _, _, _, _, _, _, _, _, _, _) => {
            h6823
        }
    };
    let a6825 = 0u64;
    if if nat_eq(a6824, a6825) {
        f45(v0.clone())?
    } else {
        false
    } {
        let a6826 = v1;
        let a6827 = 1u64;
        if nat_le(a6826, a6827) {
            Ok(true)
        } else {
            let a6832 = match v0.clone() {
                Adt2::C0(_, _, _, _, _, _, h6828, _, _, _, _) => {
                    h6828
                }
            };
            let a6833 = {
                let a6830 = 4u64;
                let a6831 = match v0 {
                    Adt2::C0(_, h6829, _, _, _, _, _, _, _, _, _) => {
                        h6829
                    }
                };
                nat_mul(a6830, a6831)?
            };
            Ok(nat_eq(a6832, a6833))
        }
    } else {
        Ok(false)
    }
}

pub fn f239(v0: Adt2, v1: u64) -> R<bool> {
    let a6835 = match v0.clone() {
        Adt2::C0(_, _, _, h6834, _, _, _, _, _, _, _) => {
            h6834
        }
    };
    let a6836 = 1u64;
    if if if nat_eq(a6835, a6836) {
        let a6838 = match v0.clone() {
            Adt2::C0(_, _, _, _, h6837, _, _, _, _, _, _) => {
                h6837
            }
        };
        let a6839 = 1u64;
        nat_eq(a6838, a6839)
    } else {
        false
    } {
        let a6841 = match v0.clone() {
            Adt2::C0(_, _, _, _, _, h6840, _, _, _, _, _) => {
                h6840
            }
        };
        let a6842 = v1;
        nat_eq(a6841, a6842)
    } else {
        false
    } {
        let a6844 = match v0.clone() {
            Adt2::C0(_, _, h6843, _, _, _, _, _, _, _, _) => {
                h6843
            }
        };
        let a6845 = 1u64;
        if nat_eq(a6844, a6845) {
            Ok(true)
        } else {
            let a6850 = match v0.clone() {
                Adt2::C0(_, _, _, _, _, _, h6846, _, _, _, _) => {
                    h6846
                }
            };
            let a6851 = {
                let a6848 = match v0 {
                    Adt2::C0(_, h6847, _, _, _, _, _, _, _, _, _) => {
                        h6847
                    }
                };
                let a6849 = v1;
                nat_mul(a6848, a6849)?
            };
            Ok(nat_eq(a6850, a6851))
        }
    } else {
        Ok(false)
    }
}

pub fn f240(v0: Adt6, v1: u64, v2: u64) -> R<bool> {
    let v3: u64 = {
        let a6855 = {
            let a6852 = v2;
            let a6853 = 1u64;
            nat_sub(a6852, a6853)
        };
        let a6856 = match v0.clone() {
            Adt6::C0(_, h6854, _, _) => {
                h6854
            }
        };
        nat_mul(a6855, a6856)?
    };
    let a6858 = 0u64;
    let a6859 = match v0.clone() {
        Adt6::C0(h6857, _, _, _) => {
            h6857
        }
    };
    if if if if if if if if nat_lt(a6858, a6859) {
        let a6861 = match v0.clone() {
            Adt6::C0(h6860, _, _, _) => {
                h6860
            }
        };
        let a6862 = 2147483648u64;
        nat_lt(a6861, a6862)
    } else {
        false
    } {
        let a6864 = 0u64;
        let a6865 = match v0.clone() {
            Adt6::C0(_, h6863, _, _) => {
                h6863
            }
        };
        nat_lt(a6864, a6865)
    } else {
        false
    } {
        let a6867 = match v0.clone() {
            Adt6::C0(_, h6866, _, _) => {
                h6866
            }
        };
        let a6868 = 2147483648u64;
        nat_lt(a6867, a6868)
    } else {
        false
    } {
        let a6869 = v1;
        let a6870 = 2147483648u64;
        nat_lt(a6869, a6870)
    } else {
        false
    } {
        let a6871 = 0u64;
        let a6872 = v2;
        nat_lt(a6871, a6872)
    } else {
        false
    } {
        let a6873 = v3;
        let a6874 = 2147483648u64;
        nat_lt(a6873, a6874)
    } else {
        false
    } {
        let a6886 = {
            let a6881 = {
                let a6878 = v3;
                let a6879 = {
                    let a6876 = match v0.clone() {
                        Adt6::C0(h6875, _, _, _) => {
                            h6875
                        }
                    };
                    let a6877 = 1u64;
                    nat_sub(a6876, a6877)
                };
                nat_add(a6878, a6879)?
            };
            let a6882 = f159(match v0.clone() {
                Adt6::C0(_, _, h6880, _) => {
                    h6880
                }
            });
            nat_add(a6881, a6882)?
        };
        let a6887 = {
            let a6884 = 2147483647u64;
            let a6885 = f160(match v0.clone() {
                Adt6::C0(_, _, h6883, _) => {
                    h6883
                }
            });
            nat_add(a6884, a6885)?
        };
        nat_le(a6886, a6887)
    } else {
        false
    } {
        let a6889 = match v0 {
            Adt6::C0(_, _, h6888, _) => {
                h6888
            }
        };
        let a6890 = 2147483648u64;
        Ok(!nat_eq(a6889, a6890))
    } else {
        Ok(false)
    }
}

pub fn f241(v0: Adt2, v1: Adt2, v2: Adt2) -> R<Adt10> {
    if f290(match v0.clone() {
        Adt2::C0(h6891, _, _, _, _, _, _, _, _, _, _) => {
            h6891
        }
    }, match v0.clone() {
        Adt2::C0(_, h6892, _, _, _, _, _, _, _, _, _) => {
            h6892
        }
    }, match v0.clone() {
        Adt2::C0(_, _, h6893, _, _, _, _, _, _, _, _) => {
            h6893
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, h6894, _, _, _, _, _, _, _) => {
            h6894
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, h6895, _, _, _, _, _, _) => {
            h6895
        }
    }, match v1.clone() {
        Adt2::C0(h6896, _, _, _, _, _, _, _, _, _, _) => {
            h6896
        }
    }, match v1.clone() {
        Adt2::C0(_, h6897, _, _, _, _, _, _, _, _, _) => {
            h6897
        }
    }, match v1.clone() {
        Adt2::C0(_, _, h6898, _, _, _, _, _, _, _, _) => {
            h6898
        }
    }, match v1.clone() {
        Adt2::C0(_, _, _, h6899, _, _, _, _, _, _, _) => {
            h6899
        }
    }, match v1.clone() {
        Adt2::C0(_, _, _, _, h6900, _, _, _, _, _, _) => {
            h6900
        }
    }, match v2.clone() {
        Adt2::C0(h6901, _, _, _, _, _, _, _, _, _, _) => {
            h6901
        }
    }, match v2.clone() {
        Adt2::C0(_, h6902, _, _, _, _, _, _, _, _, _) => {
            h6902
        }
    }, match v2.clone() {
        Adt2::C0(_, _, h6903, _, _, _, _, _, _, _, _) => {
            h6903
        }
    }, match v2.clone() {
        Adt2::C0(_, _, _, h6904, _, _, _, _, _, _, _) => {
            h6904
        }
    }, match v2.clone() {
        Adt2::C0(_, _, _, _, h6905, _, _, _, _, _, _) => {
            h6905
        }
    }) {
        let m6907 = f235(match v0.clone() {
            Adt2::C0(h6906, _, _, _, _, _, _, _, _, _, _) => {
                h6906
            }
        });
        match m6907 {
            Adt18::C0 => {
                Ok(Adt10::C0)
            }
            Adt18::C1(v3) => {
                let a6913 = {
                    let a6910 = match v0.clone() {
                        Adt2::C0(_, h6908, _, _, _, _, _, _, _, _, _) => {
                            h6908
                        }
                    };
                    let a6911 = match v3.clone() {
                        Adt19::C0(_, h6909, _) => {
                            h6909
                        }
                    };
                    let a6912 = 0u64;
                    nat_rem(a6910, a6911, a6912)
                };
                let a6914 = 0u64;
                if if nat_eq(a6913, a6914) {
                    if f291(match v0.clone() {
                        Adt2::C0(_, h6915, _, _, _, _, _, _, _, _, _) => {
                            h6915
                        }
                    }, match v1.clone() {
                        Adt2::C0(_, h6916, _, _, _, _, _, _, _, _, _) => {
                            h6916
                        }
                    }, match v1.clone() {
                        Adt2::C0(_, _, h6917, _, _, _, _, _, _, _, _) => {
                            h6917
                        }
                    }, match v1.clone() {
                        Adt2::C0(_, _, _, h6918, _, _, _, _, _, _, _) => {
                            h6918
                        }
                    }) {
                        true
                    } else {
                        let a6920 = 4u64;
                        let a6921 = match v2.clone() {
                            Adt2::C0(_, _, _, _, _, h6919, _, _, _, _, _) => {
                                h6919
                            }
                        };
                        if if nat_le(a6920, a6921) {
                            let a6923 = match v3.clone() {
                                Adt19::C0(_, h6922, _) => {
                                    h6922
                                }
                            };
                            let a6924 = 1u64;
                            if nat_eq(a6923, a6924) {
                                true
                            } else {
                                let a6927 = match v3.clone() {
                                    Adt19::C0(_, _, h6925) => {
                                        h6925
                                    }
                                };
                                let a6928 = match v0.clone() {
                                    Adt2::C0(_, _, _, _, _, h6926, _, _, _, _, _) => {
                                        h6926
                                    }
                                };
                                nat_le(a6927, a6928)
                            }
                        } else {
                            false
                        } {
                            if f27(v0.clone(), v2.clone())? {
                                f27(v1.clone(), v2.clone())?
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
                    let v4: Adt3 = f28(v0.clone(), match v3 {
                        Adt19::C0(_, h6929, _) => {
                            h6929
                        }
                    });
                    Ok(Adt10::C4(if f291(match v0.clone() {
                        Adt2::C0(_, h6930, _, _, _, _, _, _, _, _, _) => {
                            h6930
                        }
                    }, match v1.clone() {
                        Adt2::C0(_, h6931, _, _, _, _, _, _, _, _, _) => {
                            h6931
                        }
                    }, match v1.clone() {
                        Adt2::C0(_, _, h6932, _, _, _, _, _, _, _, _) => {
                            h6932
                        }
                    }, match v1.clone() {
                        Adt2::C0(_, _, _, h6933, _, _, _, _, _, _, _) => {
                            h6933
                        }
                    }) {
                        0u64
                    } else {
                        f200(v4.clone(), f28(v1.clone(), 1u64))?
                    }, 4u64, v4, f28(v2, 1u64), Adt4::C20(match v0 {
                        Adt2::C0(h6934, _, _, _, _, _, _, _, _, _, _) => {
                            h6934
                        }
                    }, f28(v1, 1u64))))
                } else {
                    Ok(Adt10::C1)
                }
            }
        }
    } else {
        Ok(Adt10::C1)
    }
}

pub fn f242(v0: Adt2, v1: Adt2, v2: Adt2) -> R<Adt10> {
    if if if if if if if if if if if if if f243(match v0.clone() {
        Adt2::C0(h6935, _, _, _, _, _, _, _, _, _, _) => {
            h6935
        }
    }) {
        let a6941 = {
            let a6938 = match v0.clone() {
                Adt2::C0(_, h6936, _, _, _, _, _, _, _, _, _) => {
                    h6936
                }
            };
            let a6939 = f273(match v0.clone() {
                Adt2::C0(h6937, _, _, _, _, _, _, _, _, _, _) => {
                    h6937
                }
            });
            let a6940 = 1u64;
            nat_rem(a6938, a6939, a6940)
        };
        let a6942 = 0u64;
        nat_eq(a6941, a6942)
    } else {
        false
    } {
        let a6944 = match v1.clone() {
            Adt2::C0(h6943, _, _, _, _, _, _, _, _, _, _) => {
                h6943
            }
        };
        let a6945 = 26u64;
        nat_eq(a6944, a6945)
    } else {
        false
    } {
        let a6947 = match v2.clone() {
            Adt2::C0(h6946, _, _, _, _, _, _, _, _, _, _) => {
                h6946
            }
        };
        let a6948 = 0u64;
        nat_eq(a6947, a6948)
    } else {
        false
    } {
        let a6951 = match v0.clone() {
            Adt2::C0(_, h6949, _, _, _, _, _, _, _, _, _) => {
                h6949
            }
        };
        let a6952 = match v2.clone() {
            Adt2::C0(_, h6950, _, _, _, _, _, _, _, _, _) => {
                h6950
            }
        };
        nat_eq(a6951, a6952)
    } else {
        false
    } {
        let a6955 = match v1.clone() {
            Adt2::C0(_, h6953, _, _, _, _, _, _, _, _, _) => {
                h6953
            }
        };
        let a6956 = match v0.clone() {
            Adt2::C0(_, _, h6954, _, _, _, _, _, _, _, _) => {
                h6954
            }
        };
        nat_le(a6955, a6956)
    } else {
        false
    } {
        let a6958 = match v0.clone() {
            Adt2::C0(_, _, _, h6957, _, _, _, _, _, _, _) => {
                h6957
            }
        };
        let a6959 = 1u64;
        nat_eq(a6958, a6959)
    } else {
        false
    } {
        let a6961 = match v0.clone() {
            Adt2::C0(_, _, _, _, h6960, _, _, _, _, _, _) => {
                h6960
            }
        };
        let a6962 = 1u64;
        nat_eq(a6961, a6962)
    } else {
        false
    } {
        let a6964 = match v2.clone() {
            Adt2::C0(_, _, _, h6963, _, _, _, _, _, _, _) => {
                h6963
            }
        };
        let a6965 = 1u64;
        nat_eq(a6964, a6965)
    } else {
        false
    } {
        let a6967 = match v2.clone() {
            Adt2::C0(_, _, _, _, h6966, _, _, _, _, _, _) => {
                h6966
            }
        };
        let a6968 = 1u64;
        nat_eq(a6967, a6968)
    } else {
        false
    } {
        let a6970 = match v1.clone() {
            Adt2::C0(_, _, h6969, _, _, _, _, _, _, _, _) => {
                h6969
            }
        };
        let a6971 = 1u64;
        nat_eq(a6970, a6971)
    } else {
        false
    } {
        let a6973 = match v1.clone() {
            Adt2::C0(_, _, _, h6972, _, _, _, _, _, _, _) => {
                h6972
            }
        };
        let a6974 = 1u64;
        nat_eq(a6973, a6974)
    } else {
        false
    } {
        let a6976 = match v1.clone() {
            Adt2::C0(_, _, _, _, h6975, _, _, _, _, _, _) => {
                h6975
            }
        };
        let a6977 = 1u64;
        nat_eq(a6976, a6977)
    } else {
        false
    } {
        if f55(match v2.clone() {
            Adt2::C0(_, h6978, _, _, _, _, _, _, _, _, _) => {
                h6978
            }
        }, match v2.clone() {
            Adt2::C0(_, _, h6979, _, _, _, _, _, _, _, _) => {
                h6979
            }
        }, match v2.clone() {
            Adt2::C0(_, _, _, h6980, _, _, _, _, _, _, _) => {
                h6980
            }
        }, match v2.clone() {
            Adt2::C0(_, _, _, _, h6981, _, _, _, _, _, _) => {
                h6981
            }
        }) {
            Ok(Adt10::C4(0u64, 4u64, f28(v0, 1u64), f28(v2, 1u64), Adt4::C9(0u64, 0u64)))
        } else {
            let a6983 = match v1.clone() {
                Adt2::C0(_, h6982, _, _, _, _, _, _, _, _, _) => {
                    h6982
                }
            };
            let a6984 = 0u64;
            if nat_eq(a6983, a6984) {
                Ok(Adt10::C4(f25(v2.clone(), 1u64)?, 4u64, f28(v0, 1u64), f28(v2, 1u64), Adt4::C9(0u64, 0u64)))
            } else {
                let a6986 = match v0.clone() {
                    Adt2::C0(_, h6985, _, _, _, _, _, _, _, _, _) => {
                        h6985
                    }
                };
                let a6987 = 2147483647u64;
                if if if if if if nat_le(a6986, a6987) {
                    let a6989 = match v1.clone() {
                        Adt2::C0(_, h6988, _, _, _, _, _, _, _, _, _) => {
                            h6988
                        }
                    };
                    let a6990 = 2147483647u64;
                    nat_le(a6989, a6990)
                } else {
                    false
                } {
                    let a6992 = 4u64;
                    let a6993 = match v2.clone() {
                        Adt2::C0(_, _, _, _, _, h6991, _, _, _, _, _) => {
                            h6991
                        }
                    };
                    nat_le(a6992, a6993)
                } else {
                    false
                } {
                    let a6999 = {
                        let a6996 = match v2.clone() {
                            Adt2::C0(_, h6994, _, _, _, _, _, _, _, _, _) => {
                                h6994
                            }
                        };
                        let a6997 = match v2.clone() {
                            Adt2::C0(_, _, _, _, _, h6995, _, _, _, _, _) => {
                                h6995
                            }
                        };
                        nat_mul(a6996, a6997)?
                    };
                    let a7000 = match v2.clone() {
                        Adt2::C0(_, _, _, _, _, _, h6998, _, _, _, _) => {
                            h6998
                        }
                    };
                    nat_le(a6999, a7000)
                } else {
                    false
                } {
                    f27(v0.clone(), v2.clone())?
                } else {
                    false
                } {
                    f27(v1.clone(), v2.clone())?
                } else {
                    false
                } {
                    Ok(Adt10::C4({
                        let a7013 = {
                            let a7004 = {
                                let a7002 = 2u64;
                                let a7003 = match v1.clone() {
                                    Adt2::C0(_, h7001, _, _, _, _, _, _, _, _, _) => {
                                        h7001
                                    }
                                };
                                nat_mul(a7002, a7003)?
                            };
                            let a7005 = f25(v2.clone(), 1u64)?;
                            nat_add(a7004, a7005)?
                        };
                        let a7014 = {
                            let a7011 = match v1.clone() {
                                Adt2::C0(_, h7006, _, _, _, _, _, _, _, _, _) => {
                                    h7006
                                }
                            };
                            let a7012 = {
                                let a7009 = 1u64;
                                let a7010 = f197(match v0.clone() {
                                    Adt2::C0(h7007, _, _, _, _, _, _, _, _, _, _) => {
                                        h7007
                                    }
                                }, match v0.clone() {
                                    Adt2::C0(_, h7008, _, _, _, _, _, _, _, _, _) => {
                                        h7008
                                    }
                                })?;
                                nat_add(a7009, a7010)?
                            };
                            nat_mul(a7011, a7012)?
                        };
                        nat_add(a7013, a7014)?
                    }, 4u64, Adt3::C0(match v0.clone() {
                        Adt2::C0(_, h7015, _, _, _, _, _, _, _, _, _) => {
                            h7015
                        }
                    }, match v1.clone() {
                        Adt2::C0(_, h7016, _, _, _, _, _, _, _, _, _) => {
                            h7016
                        }
                    }, 1u64, 1u64, match v0.clone() {
                        Adt2::C0(_, _, _, _, _, h7017, _, _, _, _, _) => {
                            h7017
                        }
                    }, match v0.clone() {
                        Adt2::C0(_, _, _, _, _, _, h7018, _, _, _, _) => {
                            h7018
                        }
                    }, match v0.clone() {
                        Adt2::C0(_, _, _, _, _, _, _, h7019, _, _, _) => {
                            h7019
                        }
                    }, match v0.clone() {
                        Adt2::C0(_, _, _, _, _, _, _, _, h7020, _, _) => {
                            h7020
                        }
                    }), f28(v2, 1u64), Adt4::C19(match v0 {
                        Adt2::C0(h7021, _, _, _, _, _, _, _, _, _, _) => {
                            h7021
                        }
                    }, f28(v1, 1u64))))
                } else {
                    Ok(Adt10::C1)
                }
            }
        }
    } else {
        Ok(Adt10::C1)
    }
}

pub fn f243(v0: u64) -> bool {
    let a7022 = v0;
    let a7023 = 0u64;
    if if if if if if if if if if if if if if if if if if if if if if if if if if if if if nat_eq(a7022, a7023) {
        true
    } else {
        let a7024 = v0;
        let a7025 = 1u64;
        nat_eq(a7024, a7025)
    } {
        true
    } else {
        let a7026 = v0;
        let a7027 = 2u64;
        nat_eq(a7026, a7027)
    } {
        true
    } else {
        let a7028 = v0;
        let a7029 = 3u64;
        nat_eq(a7028, a7029)
    } {
        true
    } else {
        let a7030 = v0;
        let a7031 = 6u64;
        nat_eq(a7030, a7031)
    } {
        true
    } else {
        let a7032 = v0;
        let a7033 = 7u64;
        nat_eq(a7032, a7033)
    } {
        true
    } else {
        let a7034 = v0;
        let a7035 = 8u64;
        nat_eq(a7034, a7035)
    } {
        true
    } else {
        let a7036 = v0;
        let a7037 = 10u64;
        nat_eq(a7036, a7037)
    } {
        true
    } else {
        let a7038 = v0;
        let a7039 = 11u64;
        nat_eq(a7038, a7039)
    } {
        true
    } else {
        let a7040 = v0;
        let a7041 = 12u64;
        nat_eq(a7040, a7041)
    } {
        true
    } else {
        let a7042 = v0;
        let a7043 = 13u64;
        nat_eq(a7042, a7043)
    } {
        true
    } else {
        let a7044 = v0;
        let a7045 = 14u64;
        nat_eq(a7044, a7045)
    } {
        true
    } else {
        let a7046 = v0;
        let a7047 = 16u64;
        nat_eq(a7046, a7047)
    } {
        true
    } else {
        let a7048 = v0;
        let a7049 = 17u64;
        nat_eq(a7048, a7049)
    } {
        true
    } else {
        let a7050 = v0;
        let a7051 = 18u64;
        nat_eq(a7050, a7051)
    } {
        true
    } else {
        let a7052 = v0;
        let a7053 = 19u64;
        nat_eq(a7052, a7053)
    } {
        true
    } else {
        let a7054 = v0;
        let a7055 = 20u64;
        nat_eq(a7054, a7055)
    } {
        true
    } else {
        let a7056 = v0;
        let a7057 = 21u64;
        nat_eq(a7056, a7057)
    } {
        true
    } else {
        let a7058 = v0;
        let a7059 = 22u64;
        nat_eq(a7058, a7059)
    } {
        true
    } else {
        let a7060 = v0;
        let a7061 = 23u64;
        nat_eq(a7060, a7061)
    } {
        true
    } else {
        let a7062 = v0;
        let a7063 = 26u64;
        nat_eq(a7062, a7063)
    } {
        true
    } else {
        let a7064 = v0;
        let a7065 = 29u64;
        nat_eq(a7064, a7065)
    } {
        true
    } else {
        let a7066 = v0;
        let a7067 = 30u64;
        nat_eq(a7066, a7067)
    } {
        true
    } else {
        let a7068 = v0;
        let a7069 = 34u64;
        nat_eq(a7068, a7069)
    } {
        true
    } else {
        let a7070 = v0;
        let a7071 = 35u64;
        nat_eq(a7070, a7071)
    } {
        true
    } else {
        let a7072 = v0;
        let a7073 = 39u64;
        nat_eq(a7072, a7073)
    } {
        true
    } else {
        let a7074 = v0;
        let a7075 = 40u64;
        nat_eq(a7074, a7075)
    } {
        true
    } else {
        let a7076 = v0;
        let a7077 = 41u64;
        nat_eq(a7076, a7077)
    } {
        true
    } else {
        let a7078 = v0;
        let a7079 = 42u64;
        nat_eq(a7078, a7079)
    } {
        true
    } else {
        let a7080 = v0;
        let a7081 = 43u64;
        nat_eq(a7080, a7081)
    }
}

pub fn f244(v0: u64) -> bool {
    if f198(v0) {
        true
    } else {
        let a7082 = v0;
        let a7083 = 43u64;
        nat_eq(a7082, a7083)
    }
}

pub fn f245(v0: u64, v1: Adt2, v2: bool, v3: Adt2) -> R<Adt10> {
    let a7086 = match v1.clone() {
        Adt2::C0(h7084, _, _, _, _, _, _, _, _, _, _) => {
            h7084
        }
    };
    let a7087 = match v3.clone() {
        Adt2::C0(h7085, _, _, _, _, _, _, _, _, _, _) => {
            h7085
        }
    };
    if if nat_eq(a7086, a7087) {
        let a7088 = v0;
        let a7089 = 33u64;
        if nat_eq(a7088, a7089) {
            v2
        } else {
            true
        }
    } else {
        false
    } {
        let m7091 = f235(match v1.clone() {
            Adt2::C0(h7090, _, _, _, _, _, _, _, _, _, _) => {
                h7090
            }
        });
        match m7091 {
            Adt18::C0 => {
                Ok(Adt10::C0)
            }
            Adt18::C1(v4) => {
                let v5: u64 = match v4.clone() {
                    Adt19::C0(_, h7092, _) => {
                        h7092
                    }
                };
                let v6: u64 = f25(v1.clone(), v5)?;
                let a7097 = {
                    let a7094 = match v1.clone() {
                        Adt2::C0(_, h7093, _, _, _, _, _, _, _, _, _) => {
                            h7093
                        }
                    };
                    let a7095 = v5;
                    let a7096 = 1u64;
                    nat_rem(a7094, a7095, a7096)
                };
                let a7098 = 0u64;
                if if if nat_eq(a7097, a7098) {
                    let a7103 = {
                        let a7100 = match v3.clone() {
                            Adt2::C0(_, h7099, _, _, _, _, _, _, _, _, _) => {
                                h7099
                            }
                        };
                        let a7101 = v5;
                        let a7102 = 1u64;
                        nat_rem(a7100, a7101, a7102)
                    };
                    let a7104 = 0u64;
                    nat_eq(a7103, a7104)
                } else {
                    false
                } {
                    let a7105 = v6;
                    let a7106 = f25(v3.clone(), v5)?;
                    nat_eq(a7105, a7106)
                } else {
                    false
                } {
                    Ok(Adt10::C4(v6, match v4 {
                        Adt19::C0(_, _, h7107) => {
                            h7107
                        }
                    }, f28(v1, v5), f28(v3, v5), Adt4::C0))
                } else {
                    Ok(Adt10::C1)
                }
            }
        }
    } else {
        Ok(Adt10::C1)
    }
}

pub fn f246(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64, v7: u64) -> bool {
    if f55(v4, v5, v6, v7) {
        f55(v0, v1, v2, v3)
    } else {
        let a7111 = {
            let a7108 = v0;
            let a7109 = v4;
            let a7110 = 1u64;
            nat_rem(a7108, a7109, a7110)
        };
        let a7112 = 0u64;
        if if if nat_eq(a7111, a7112) {
            let a7116 = {
                let a7113 = v1;
                let a7114 = v5;
                let a7115 = 1u64;
                nat_rem(a7113, a7114, a7115)
            };
            let a7117 = 0u64;
            nat_eq(a7116, a7117)
        } else {
            false
        } {
            let a7121 = {
                let a7118 = v2;
                let a7119 = v6;
                let a7120 = 1u64;
                nat_rem(a7118, a7119, a7120)
            };
            let a7122 = 0u64;
            nat_eq(a7121, a7122)
        } else {
            false
        } {
            let a7126 = {
                let a7123 = v3;
                let a7124 = v7;
                let a7125 = 1u64;
                nat_rem(a7123, a7124, a7125)
            };
            let a7127 = 0u64;
            nat_eq(a7126, a7127)
        } else {
            false
        }
    }
}

pub fn f247(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64) -> R<bool> {
    let a7128 = v0;
    let a7129 = v1;
    if nat_eq(a7128, a7129) {
        let a7132 = v4;
        let a7133 = {
            let a7130 = v2;
            let a7131 = v3;
            nat_add(a7130, a7131)?
        };
        Ok(nat_eq(a7132, a7133))
    } else {
        let a7134 = v2;
        let a7135 = v3;
        if nat_eq(a7134, a7135) {
            let a7136 = v4;
            let a7137 = v2;
            Ok(nat_eq(a7136, a7137))
        } else {
            Ok(false)
        }
    }
}

pub fn f248(v0: u64, v1: u64, v2: u64, v3: u64) -> R<bool> {
    let v4: u64 = {
        let a7140 = {
            let a7138 = v0;
            let a7139 = f160(v1);
            nat_add(a7138, a7139)?
        };
        let a7141 = f160(v2);
        nat_add(a7140, a7141)?
    };
    let v5: u64 = {
        let a7142 = f159(v1);
        let a7143 = f159(v2);
        nat_add(a7142, a7143)?
    };
    let a7144 = v1;
    let a7145 = 4294967295u64;
    if if nat_le(a7144, a7145) {
        let a7146 = v2;
        let a7147 = 4294967295u64;
        nat_le(a7146, a7147)
    } else {
        false
    } {
        let a7148 = v5;
        let a7149 = v4;
        if nat_le(a7148, a7149) {
            let a7152 = v3;
            let a7153 = {
                let a7150 = v4;
                let a7151 = v5;
                nat_sub(a7150, a7151)
            };
            Ok(nat_eq(a7152, a7153))
        } else {
            Ok(false)
        }
    } else {
        Ok(false)
    }
}

pub fn f249(v0: u64, v1: u64) -> R<bool> {
    let a7154 = v1;
    let a7155 = 4294967295u64;
    if nat_le(a7154, a7155) {
        let a7158 = {
            let a7156 = f160(v1);
            let a7157 = f159(v1);
            nat_add(a7156, a7157)?
        };
        let a7159 = v0;
        Ok(nat_lt(a7158, a7159))
    } else {
        Ok(false)
    }
}

pub fn f250(v0: Adt3, v1: u64, v2: u64, v3: u64, v4: bool) -> R<bool> {
    let v5: u64 = f292(v0.clone(), false)?;
    let v6: u64 = f292(v0, true)?;
    let a7160 = v6;
    let a7161 = v1;
    if if nat_le(a7160, a7161) {
        let a7168 = {
            let a7166 = {
                let a7162 = v1;
                let a7163 = v5;
                nat_add(a7162, a7163)?
            };
            let a7167 = {
                let a7164 = v3;
                let a7165 = 0u64;
                if nat_eq(a7164, a7165) {
                    0u64
                } else {
                    4u64
                }
            };
            nat_add(a7166, a7167)?
        };
        let a7169 = v2;
        nat_le(a7168, a7169)
    } else {
        false
    } {
        if v4 {
            let a7174 = {
                let a7172 = {
                    let a7170 = v1;
                    let a7171 = v5;
                    nat_add(a7170, a7171)?
                };
                let a7173 = v6;
                nat_sub(a7172, a7173)
            };
            let a7175 = v2;
            Ok(nat_lt(a7174, a7175))
        } else {
            let a7180 = {
                let a7178 = {
                    let a7176 = v1;
                    let a7177 = v5;
                    nat_add(a7176, a7177)?
                };
                let a7179 = v6;
                nat_sub(a7178, a7179)
            };
            let a7181 = v2;
            Ok(nat_le(a7180, a7181))
        }
    } else {
        Ok(false)
    }
}

pub fn f251(v0: u64, v1: u64) -> bool {
    let a7182 = v0;
    let a7183 = 0u64;
    if if nat_eq(a7182, a7183) {
        true
    } else {
        let a7184 = v0;
        let a7185 = 1u64;
        nat_eq(a7184, a7185)
    } {
        let a7186 = v1;
        let a7187 = 4294967296u64;
        nat_lt(a7186, a7187)
    } else {
        false
    }
}

pub fn f252(v0: u64, v1: u64) -> R<u64> {
    let a7192 = {
        let a7190 = v0;
        let a7191 = {
            let a7188 = v1;
            let a7189 = 1u64;
            nat_sub(a7188, a7189)
        };
        nat_add(a7190, a7191)?
    };
    let a7193 = v1;
    let a7194 = 0u64;
    Ok(nat_quot(a7192, a7193, a7194))
}

pub fn f253(v0: Adt2, v1: Adt2, v2: u64, v3: u64) -> R<bool> {
    if f55(match v0.clone() {
        Adt2::C0(_, h7195, _, _, _, _, _, _, _, _, _) => {
            h7195
        }
    }, match v0.clone() {
        Adt2::C0(_, _, h7196, _, _, _, _, _, _, _, _) => {
            h7196
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, h7197, _, _, _, _, _, _, _) => {
            h7197
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, h7198, _, _, _, _, _, _) => {
            h7198
        }
    }) {
        Ok(true)
    } else {
        let v4: u64 = {
            let a7201 = match v1.clone() {
                Adt2::C0(_, h7199, _, _, _, _, _, _, _, _, _) => {
                    h7199
                }
            };
            let a7202 = match v1.clone() {
                Adt2::C0(_, _, h7200, _, _, _, _, _, _, _, _) => {
                    h7200
                }
            };
            nat_mul(a7201, a7202)?
        };
        let v5: u64 = {
            let a7204 = v4;
            let a7205 = match v1.clone() {
                Adt2::C0(_, _, _, h7203, _, _, _, _, _, _, _) => {
                    h7203
                }
            };
            nat_mul(a7204, a7205)?
        };
        let a7218 = {
            let a7216 = {
                let a7207 = match v0.clone() {
                    Adt2::C0(_, h7206, _, _, _, _, _, _, _, _, _) => {
                        h7206
                    }
                };
                let a7208 = 1u64;
                nat_sub(a7207, a7208)
            };
            let a7217 = {
                let a7214 = f293(match v0.clone() {
                    Adt2::C0(_, _, h7209, _, _, _, _, _, _, _, _) => {
                        h7209
                    }
                }, v2, v5, match v1.clone() {
                    Adt2::C0(_, h7210, _, _, _, _, _, _, _, _, _) => {
                        h7210
                    }
                })?;
                let a7215 = f293(match v0 {
                    Adt2::C0(_, _, _, h7211, _, _, _, _, _, _, _) => {
                        h7211
                    }
                }, v2, {
                    let a7212 = v3;
                    let a7213 = v5;
                    nat_mul(a7212, a7213)?
                }, v4)?;
                nat_add(a7214, a7215)?
            };
            nat_add(a7216, a7217)?
        };
        let a7219 = f25(v1, 1u64)?;
        Ok(nat_lt(a7218, a7219))
    }
}

pub fn f254(v0: u64, v1: u64, v2: u64) -> R<bool> {
    let a7220 = v0;
    let a7221 = 2147483648u64;
    if if if if nat_lt(a7220, a7221) {
        let a7222 = v1;
        let a7223 = 2147483648u64;
        nat_lt(a7222, a7223)
    } else {
        false
    } {
        let a7226 = {
            let a7224 = v0;
            let a7225 = v1;
            nat_mul(a7224, a7225)?
        };
        let a7227 = 2147483648u64;
        nat_lt(a7226, a7227)
    } else {
        false
    } {
        let a7232 = {
            let a7230 = {
                let a7228 = v0;
                let a7229 = 1u64;
                nat_sub(a7228, a7229)
            };
            let a7231 = v2;
            nat_mul(a7230, a7231)?
        };
        let a7233 = 2147483648u64;
        nat_lt(a7232, a7233)
    } else {
        false
    } {
        let a7238 = {
            let a7236 = {
                let a7234 = v1;
                let a7235 = 1u64;
                nat_sub(a7234, a7235)
            };
            let a7237 = v2;
            nat_mul(a7236, a7237)?
        };
        let a7239 = 2147483648u64;
        Ok(nat_lt(a7238, a7239))
    } else {
        Ok(false)
    }
}

pub fn f255(v0: Adt3, v1: u64, v2: u64) -> R<u64> {
    let a7240 = v1;
    let a7241 = 13u64;
    if if nat_eq(a7240, a7241) {
        f55(match v0.clone() {
            Adt3::C0(h7242, _, _, _, _, _, _, _) => {
                h7242
            }
        }, match v0.clone() {
            Adt3::C0(_, h7243, _, _, _, _, _, _) => {
                h7243
            }
        }, match v0.clone() {
            Adt3::C0(_, _, h7244, _, _, _, _, _) => {
                h7244
            }
        }, match v0.clone() {
            Adt3::C0(_, _, _, h7245, _, _, _, _) => {
                h7245
            }
        })
    } else {
        false
    } {
        Ok(2u64)
    } else {
        let v3: u64 = {
            let a7251 = match v0.clone() {
                Adt3::C0(_, h7246, _, _, _, _, _, _) => {
                    h7246
                }
            };
            let a7252 = {
                let a7249 = match v0.clone() {
                    Adt3::C0(_, _, h7247, _, _, _, _, _) => {
                        h7247
                    }
                };
                let a7250 = match v0.clone() {
                    Adt3::C0(_, _, _, h7248, _, _, _, _) => {
                        h7248
                    }
                };
                nat_mul(a7249, a7250)?
            };
            nat_mul(a7251, a7252)?
        };
        let a7253 = v1;
        let a7254 = 13u64;
        if nat_eq(a7253, a7254) {
            let a7263 = 2u64;
            let a7264 = if f294(v0.clone(), v2) {
                let a7256 = v3;
                let a7257 = f267(match v0 {
                    Adt3::C0(h7255, _, _, _, _, _, _, _) => {
                        h7255
                    }
                }, false)?;
                nat_mul(a7256, a7257)?
            } else {
                let a7261 = v3;
                let a7262 = {
                    let a7259 = match v0 {
                        Adt3::C0(h7258, _, _, _, _, _, _, _) => {
                            h7258
                        }
                    };
                    let a7260 = 2u64;
                    nat_mul(a7259, a7260)?
                };
                nat_mul(a7261, a7262)?
            };
            nat_add(a7263, a7264)
        } else {
            let a7268 = v3;
            let a7269 = f267(match v0 {
                Adt3::C0(h7265, _, _, _, _, _, _, _) => {
                    h7265
                }
            }, {
                let a7266 = v1;
                let a7267 = 16u64;
                nat_eq(a7266, a7267)
            })?;
            nat_mul(a7268, a7269)
        }
    }
}

pub fn f256(v0: u64, v1: u64, v2: Adt2, v3: Option<Adt2>, v4: Adt2) -> R<Adt10> {
    let a7270 = v0;
    let a7271 = 30u64;
    if if nat_eq(a7270, a7271) {
        let a7274 = match v2.clone() {
            Adt2::C0(_, _, _, _, _, _, h7272, _, _, _, _) => {
                h7272
            }
        };
        let a7275 = match v2.clone() {
            Adt2::C0(_, _, _, _, _, h7273, _, _, _, _, _) => {
                h7273
            }
        };
        nat_lt(a7274, a7275)
    } else {
        false
    } {
        Ok(Adt10::C1)
    } else {
        let m7276 = v3;
        match m7276 {
            None => {
                f295(v0, v1, match v2.clone() {
                    Adt2::C0(h7277, _, _, _, _, _, _, _, _, _, _) => {
                        h7277
                    }
                }, 0u64, match v4.clone() {
                    Adt2::C0(h7278, _, _, _, _, _, _, _, _, _, _) => {
                        h7278
                    }
                }, false, match v2.clone() {
                    Adt2::C0(_, h7279, _, _, _, _, _, _, _, _, _) => {
                        h7279
                    }
                }, match v2.clone() {
                    Adt2::C0(_, _, h7280, _, _, _, _, _, _, _, _) => {
                        h7280
                    }
                }, match v2.clone() {
                    Adt2::C0(_, _, _, h7281, _, _, _, _, _, _, _) => {
                        h7281
                    }
                }, match v2 {
                    Adt2::C0(_, _, _, _, h7282, _, _, _, _, _, _) => {
                        h7282
                    }
                }, 1u64, 1u64, 1u64, 1u64, match v4.clone() {
                    Adt2::C0(_, h7283, _, _, _, _, _, _, _, _, _) => {
                        h7283
                    }
                }, match v4.clone() {
                    Adt2::C0(_, _, h7284, _, _, _, _, _, _, _, _) => {
                        h7284
                    }
                }, match v4.clone() {
                    Adt2::C0(_, _, _, h7285, _, _, _, _, _, _, _) => {
                        h7285
                    }
                }, match v4 {
                    Adt2::C0(_, _, _, _, h7286, _, _, _, _, _, _) => {
                        h7286
                    }
                })
            }
            Some(v5) => {
                f295(v0, v1, match v2.clone() {
                    Adt2::C0(h7287, _, _, _, _, _, _, _, _, _, _) => {
                        h7287
                    }
                }, match v5.clone() {
                    Adt2::C0(h7288, _, _, _, _, _, _, _, _, _, _) => {
                        h7288
                    }
                }, match v4.clone() {
                    Adt2::C0(h7289, _, _, _, _, _, _, _, _, _, _) => {
                        h7289
                    }
                }, true, match v2.clone() {
                    Adt2::C0(_, h7290, _, _, _, _, _, _, _, _, _) => {
                        h7290
                    }
                }, match v2.clone() {
                    Adt2::C0(_, _, h7291, _, _, _, _, _, _, _, _) => {
                        h7291
                    }
                }, match v2.clone() {
                    Adt2::C0(_, _, _, h7292, _, _, _, _, _, _, _) => {
                        h7292
                    }
                }, match v2 {
                    Adt2::C0(_, _, _, _, h7293, _, _, _, _, _, _) => {
                        h7293
                    }
                }, match v5.clone() {
                    Adt2::C0(_, h7294, _, _, _, _, _, _, _, _, _) => {
                        h7294
                    }
                }, match v5.clone() {
                    Adt2::C0(_, _, h7295, _, _, _, _, _, _, _, _) => {
                        h7295
                    }
                }, match v5.clone() {
                    Adt2::C0(_, _, _, h7296, _, _, _, _, _, _, _) => {
                        h7296
                    }
                }, match v5 {
                    Adt2::C0(_, _, _, _, h7297, _, _, _, _, _, _) => {
                        h7297
                    }
                }, match v4.clone() {
                    Adt2::C0(_, h7298, _, _, _, _, _, _, _, _, _) => {
                        h7298
                    }
                }, match v4.clone() {
                    Adt2::C0(_, _, h7299, _, _, _, _, _, _, _, _) => {
                        h7299
                    }
                }, match v4.clone() {
                    Adt2::C0(_, _, _, h7300, _, _, _, _, _, _, _) => {
                        h7300
                    }
                }, match v4 {
                    Adt2::C0(_, _, _, _, h7301, _, _, _, _, _, _) => {
                        h7301
                    }
                })
            }
        }
    }
}

pub fn f257(v0: u64, v1: u64) -> u64 {
    let a7302 = v0;
    let a7303 = 23u64;
    if nat_eq(a7302, a7303) {
        let a7304 = v1;
        let a7305 = 0u64;
        if nat_eq(a7304, a7305) {
            2u64
        } else {
            3u64
        }
    } else {
        1u64
    }
}

pub fn f258(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let a7310 = {
        let a7308 = {
            let a7306 = v0;
            let a7307 = v1;
            nat_sub(a7306, a7307)
        };
        let a7309 = 1u64;
        nat_sub(a7308, a7309)
    };
    let a7311 = v2;
    nat_add(a7310, a7311)
}

pub fn f259(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64, v7: u64, v8: u64, v9: u64, v10: u64, v11: u64) -> R<u64> {
    f158({
        let a7312 = v0;
        let a7313 = v4;
        let a7314 = 0u64;
        nat_rem(a7312, a7313, a7314)
    }, {
        let a7315 = v1;
        let a7316 = v5;
        let a7317 = 0u64;
        nat_rem(a7315, a7316, a7317)
    }, {
        let a7318 = v2;
        let a7319 = v6;
        let a7320 = 0u64;
        nat_rem(a7318, a7319, a7320)
    }, {
        let a7321 = v3;
        let a7322 = v7;
        let a7323 = 0u64;
        nat_rem(a7321, a7322, a7323)
    }, v8, v9, v10, v11)
}

pub fn f260(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let a7324 = v1;
    let a7325 = 2147483648u64;
    if nat_lt(a7324, a7325) {
        let a7336 = {
            let a7334 = {
                let a7329 = {
                    let a7326 = v0;
                    let a7327 = v2;
                    let a7328 = 0u64;
                    nat_rem(a7326, a7327, a7328)
                };
                let a7330 = v2;
                nat_add(a7329, a7330)?
            };
            let a7335 = {
                let a7331 = v1;
                let a7332 = v2;
                let a7333 = 0u64;
                nat_rem(a7331, a7332, a7333)
            };
            nat_sub(a7334, a7335)
        };
        let a7337 = v2;
        let a7338 = 0u64;
        Ok(nat_rem(a7336, a7337, a7338))
    } else {
        let a7341 = {
            let a7339 = v0;
            let a7340 = f159(v1);
            nat_add(a7339, a7340)?
        };
        let a7342 = v2;
        let a7343 = 0u64;
        Ok(nat_rem(a7341, a7342, a7343))
    }
}

pub fn f261(v0: u64, v1: u64, v2: u64) -> R<bool> {
    let a7346 = f160(v1);
    let a7347 = {
        let a7344 = v0;
        let a7345 = f159(v1);
        nat_add(a7344, a7345)?
    };
    if nat_le(a7346, a7347) {
        let a7348 = f262(v0, v1)?;
        let a7349 = v2;
        Ok(nat_lt(a7348, a7349))
    } else {
        Ok(false)
    }
}

pub fn f262(v0: u64, v1: u64) -> R<u64> {
    let a7352 = {
        let a7350 = v0;
        let a7351 = f159(v1);
        nat_add(a7350, a7351)?
    };
    let a7353 = f160(v1);
    Ok(nat_sub(a7352, a7353))
}

pub fn f263(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let a7354 = v0;
    let a7355 = v1;
    if nat_lt(a7354, a7355) {
        let a7356 = v1;
        let a7357 = v0;
        Ok(nat_sub(a7356, a7357))
    } else {
        let a7360 = v0;
        let a7361 = {
            let a7358 = v1;
            let a7359 = v2;
            nat_add(a7358, a7359)?
        };
        if nat_lt(a7360, a7361) {
            let a7362 = v0;
            let a7363 = v1;
            Ok(nat_sub(a7362, a7363))
        } else {
            let a7370 = {
                let a7366 = 2u64;
                let a7367 = {
                    let a7364 = v2;
                    let a7365 = 1u64;
                    nat_sub(a7364, a7365)
                };
                nat_mul(a7366, a7367)?
            };
            let a7371 = {
                let a7368 = v0;
                let a7369 = v1;
                nat_sub(a7368, a7369)
            };
            Ok(nat_sub(a7370, a7371))
        }
    }
}

pub fn f264(v0: u64, v1: Adt3, v2: u64) -> R<u64> {
    let a7374 = {
        let a7372 = v2;
        let a7373 = f296(v1.clone(), f65(v0, v1.clone(), 0u64), f65(v0, v1.clone(), 1u64), f65(v0, v1.clone(), 2u64), f65(v0, v1.clone(), 3u64), false)?;
        nat_add(a7372, a7373)?
    };
    let a7375 = f296(v1.clone(), f65(v0, v1.clone(), 0u64), f65(v0, v1.clone(), 1u64), f65(v0, v1.clone(), 2u64), f65(v0, v1, 3u64), true)?;
    Ok(nat_sub(a7374, a7375))
}

pub fn f265(v0: u64, v1: u64, v2: u64) -> bool {
    let a7376 = v0;
    let a7377 = 0u64;
    if nat_eq(a7376, a7377) {
        let a7378 = v2;
        let a7379 = v1;
        nat_le(a7378, a7379)
    } else {
        let a7380 = v0;
        let a7381 = 1u64;
        if nat_eq(a7380, a7381) {
            let a7382 = v2;
            let a7383 = v1;
            nat_lt(a7382, a7383)
        } else {
            let a7384 = v0;
            let a7385 = 2u64;
            if nat_eq(a7384, a7385) {
                let a7386 = v1;
                let a7387 = v2;
                nat_le(a7386, a7387)
            } else {
                let a7388 = v0;
                let a7389 = 3u64;
                if nat_eq(a7388, a7389) {
                    let a7390 = v1;
                    let a7391 = v2;
                    nat_lt(a7390, a7391)
                } else {
                    let a7392 = v1;
                    let a7393 = v2;
                    nat_eq(a7392, a7393)
                }
            }
        }
    }
}

pub fn f266(v0: u64, v1: Adt3, v2: u64) -> R<Adt9> {
    let a7394 = v0;
    let a7395 = 0u64;
    if nat_eq(a7394, a7395) {
        Ok(f279(0u64))
    } else {
        let a7398 = {
            let a7396 = v0;
            let a7397 = 1u64;
            nat_add(a7396, a7397)?
        };
        let a7399 = f255(v1.clone(), 13u64, v2)?;
        if nat_eq(a7398, a7399) {
            Ok(Adt9::C4({
                let a7400 = v2;
                let a7401 = 0u64;
                if nat_eq(a7400, a7401) {
                    2u64
                } else {
                    7u64
                }
            }, 1u64, 0u64, 2u64, 0u64, 3u64, 0u64))
        } else {
            let v3: u64 = {
                let a7402 = v0;
                let a7403 = 1u64;
                nat_sub(a7402, a7403)
            };
            if f294(v1.clone(), v2) {
                let v4: u64 = f267(match v1.clone() {
                    Adt3::C0(h7404, _, _, _, _, _, _, _) => {
                        h7404
                    }
                }, false)?;
                let v5: u64 = {
                    let a7405 = v3;
                    let a7406 = v4;
                    let a7407 = 0u64;
                    nat_quot(a7405, a7406, a7407)
                };
                let v6: u64 = {
                    let a7408 = v3;
                    let a7409 = v4;
                    let a7410 = 0u64;
                    nat_rem(a7408, a7409, a7410)
                };
                let a7413 = {
                    let a7411 = v6;
                    let a7412 = 1u64;
                    nat_add(a7411, a7412)?
                };
                let a7414 = v4;
                if nat_eq(a7413, a7414) {
                    Ok(Adt9::C4({
                        let a7415 = v2;
                        let a7416 = 0u64;
                        if nat_eq(a7415, a7416) {
                            0u64
                        } else {
                            4u64
                        }
                    }, 1u64, 0u64, 1u64, 1u64, 1u64, 0u64))
                } else {
                    f268(v6, v5, v1, v2)
                }
            } else {
                f297({
                    let a7417 = v3;
                    let a7418 = 2u64;
                    let a7419 = 0u64;
                    nat_quot(a7417, a7418, a7419)
                }, {
                    let a7420 = v3;
                    let a7421 = 2u64;
                    let a7422 = 0u64;
                    nat_rem(a7420, a7421, a7422)
                }, v1, v2, 0u64)
            }
        }
    }
}

pub fn f267(v0: u64, v1: bool) -> R<u64> {
    let a7425 = {
        let a7423 = v0;
        let a7424 = 2u64;
        nat_mul(a7423, a7424)?
    };
    let a7426 = if v1 {
        4u64
    } else {
        2u64
    };
    nat_add(a7425, a7426)
}

pub fn f268(v0: u64, v1: u64, v2: Adt3, v3: u64) -> R<Adt9> {
    let a7427 = v0;
    let a7428 = 0u64;
    if nat_eq(a7427, a7428) {
        Ok(f279(1u64))
    } else {
        f297({
            let a7437 = {
                let a7430 = v1;
                let a7431 = match v2.clone() {
                    Adt3::C0(h7429, _, _, _, _, _, _, _) => {
                        h7429
                    }
                };
                nat_mul(a7430, a7431)?
            };
            let a7438 = {
                let a7434 = {
                    let a7432 = v0;
                    let a7433 = 1u64;
                    nat_sub(a7432, a7433)
                };
                let a7435 = 2u64;
                let a7436 = 0u64;
                nat_quot(a7434, a7435, a7436)
            };
            nat_add(a7437, a7438)?
        }, {
            let a7441 = {
                let a7439 = v0;
                let a7440 = 1u64;
                nat_sub(a7439, a7440)
            };
            let a7442 = 2u64;
            let a7443 = 0u64;
            nat_rem(a7441, a7442, a7443)
        }, v2, v3, 1u64)
    }
}

pub fn f269(v0: u64, v1: u64) -> R<u64> {
    let a7444 = v0;
    let a7445 = f110(v1)?;
    let a7446 = 0u64;
    Ok(nat_rem(a7444, a7445, a7446))
}

pub fn f270(v0: u64, v1: u64) -> R<u64> {
    let a7447 = 4u64;
    let a7448 = f298(v0, v1)?;
    nat_mul(a7447, a7448)
}

pub fn f271(v0: u64, v1: u64) -> R<u64> {
    let a7449 = 4u64;
    let a7450 = f299(v0, v1)?;
    nat_mul(a7449, a7450)
}

pub fn f272(v0: u64, v1: u64) -> R<bool> {
    let a7456 = {
        let a7453 = {
            let a7451 = f269(v0, v1)?;
            let a7452 = 2u64;
            nat_sub(a7451, a7452)
        };
        let a7454 = 2u64;
        let a7455 = 0u64;
        nat_rem(a7453, a7454, a7455)
    };
    let a7457 = 0u64;
    Ok(nat_eq(a7456, a7457))
}

pub fn f273(v0: u64) -> u64 {
    let m7458 = f235(v0);
    match m7458 {
        Adt18::C0 => {
            0u64
        }
        Adt18::C1(v1) => {
            match v1 {
                Adt19::C0(_, h7459, _) => {
                    h7459
                }
            }
        }
    }
}

pub fn f274(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64, v7: u64, v8: u64, v9: u64, v10: u64, v11: u64, v12: u64) -> R<Option<u64>> {
    let m7460 = f235(v0);
    match m7460 {
        Adt18::C0 => {
            Ok(None::<u64>)
        }
        Adt18::C1(v13) => {
            if if if if if f243(v0) {
                let a7461 = v1;
                let a7462 = v7;
                nat_lt(a7461, a7462)
            } else {
                false
            } {
                f203(v8, v2)
            } else {
                false
            } {
                let a7463 = v3;
                let a7464 = f300(v4, v5, v6)?;
                nat_lt(a7463, a7464)
            } else {
                false
            } {
                let a7469 = {
                    let a7466 = v7;
                    let a7467 = match v13.clone() {
                        Adt19::C0(_, h7465, _) => {
                            h7465
                        }
                    };
                    let a7468 = 0u64;
                    nat_rem(a7466, a7467, a7468)
                };
                let a7470 = 0u64;
                nat_eq(a7469, a7470)
            } else {
                false
            } {
                Ok(Some({
                    let a7499 = {
                        let a7489 = {
                            let a7479 = {
                                let a7475 = {
                                    let a7472 = v1;
                                    let a7473 = match v13 {
                                        Adt19::C0(_, h7471, _) => {
                                            h7471
                                        }
                                    };
                                    let a7474 = 0u64;
                                    nat_quot(a7472, a7473, a7474)
                                };
                                let a7476 = v9;
                                nat_mul(a7475, a7476)?
                            };
                            let a7480 = {
                                let a7477 = v2;
                                let a7478 = v10;
                                nat_mul(a7477, a7478)?
                            };
                            nat_add(a7479, a7480)?
                        };
                        let a7490 = {
                            let a7487 = {
                                let a7484 = {
                                    let a7481 = v3;
                                    let a7482 = v4;
                                    let a7483 = 0u64;
                                    nat_quot(a7481, a7482, a7483)
                                };
                                let a7485 = v5;
                                let a7486 = 0u64;
                                nat_rem(a7484, a7485, a7486)
                            };
                            let a7488 = v11;
                            nat_mul(a7487, a7488)?
                        };
                        nat_add(a7489, a7490)?
                    };
                    let a7500 = {
                        let a7497 = {
                            let a7494 = {
                                let a7491 = v3;
                                let a7492 = v4;
                                let a7493 = 0u64;
                                nat_quot(a7491, a7492, a7493)
                            };
                            let a7495 = v5;
                            let a7496 = 0u64;
                            nat_quot(a7494, a7495, a7496)
                        };
                        let a7498 = v12;
                        nat_mul(a7497, a7498)?
                    };
                    nat_add(a7499, a7500)?
                }))
            } else {
                Ok(None::<u64>)
            }
        }
    }
}

pub fn f275(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64, v7: u64, v8: u64, v9: u64, v10: u64, v11: u64, v12: u64, v13: u64, v14: u64) -> R<Adt17> {
    f301(v0, Adt20::C0(v1, v2, v3, v4, v5, v6, v7, v8), Adt20::C0(v1, v2, v3, v4, v9, v10, v11, v12), v13, v14)
}

pub fn f276(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64, v7: u64, v8: u64, v9: u64, v10: u64, v11: u64, v12: u64, v13: u64, v14: u64, v15: u64, v16: u64, v17: u64, v18: u64, v19: u64, v20: u64) -> R<Adt17> {
    f302(v0, Adt20::C0(v1, v2, v3, v4, v5, v6, v7, v8), Adt20::C0(v1, v2, v3, v4, v9, v10, v11, v12), Adt20::C0(v1, v2, v3, v4, v13, v14, v15, v16), v17, v18, v19, v20)
}

pub fn f277(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64, v7: u64, v8: u64, v9: u64, v10: u64, v11: u64, v12: u64, v13: u64, v14: u64, v15: u64, v16: u64, v17: u64) -> R<Adt17> {
    let a7501 = v0;
    let a7502 = 64u64;
    if nat_eq(a7501, a7502) {
        f303(v1, Adt20::C0(v2, v3, v4, v5, v6, v7, v8, v9), Adt20::C0(v2, v3, v4, v5, v10, v11, v12, v13), v14, v15, v16, v17)
    } else {
        let a7503 = v0;
        let a7504 = 65u64;
        if nat_eq(a7503, a7504) {
            f304(v1, Adt20::C0(v2, v3, v4, v5, v6, v7, v8, v9), Adt20::C0(v2, v3, v4, v5, v10, v11, v12, v13), v14, v15, v16, v17)
        } else {
            Ok(Adt17::C0)
        }
    }
}

pub fn f278(v0: u64) -> u64 {
    let a7505 = v0;
    let a7506 = 30u64;
    if nat_eq(a7505, a7506) {
        4u64
    } else {
        3u64
    }
}

pub fn f279(v0: u64) -> Adt9 {
    Adt9::C4(0u64, 2u64, 0u64, 2u64, 0u64, 1u64, v0)
}

pub fn f280(v0: u64) -> bool {
    let a7507 = 0u64;
    let a7508 = v0;
    if nat_lt(a7507, a7508) {
        let a7509 = v0;
        let a7510 = 2139095040u64;
        nat_le(a7509, a7510)
    } else {
        false
    }
}

pub fn f281(v0: u64) -> u64 {
    let a7511 = v0;
    let a7512 = 40u64;
    if nat_lt(a7511, a7512) {
        let a7513 = v0;
        let a7514 = 20u64;
        if nat_lt(a7513, a7514) {
            let a7515 = v0;
            let a7516 = 10u64;
            if nat_lt(a7515, a7516) {
                let a7517 = v0;
                let a7518 = 5u64;
                if nat_lt(a7517, a7518) {
                    let a7519 = v0;
                    let a7520 = 2u64;
                    if nat_lt(a7519, a7520) {
                        let a7521 = v0;
                        let a7522 = 1u64;
                        if nat_lt(a7521, a7522) {
                            1u64
                        } else {
                            2u64
                        }
                    } else {
                        let a7523 = v0;
                        let a7524 = 3u64;
                        if nat_lt(a7523, a7524) {
                            4u64
                        } else {
                            let a7525 = v0;
                            let a7526 = 4u64;
                            if nat_lt(a7525, a7526) {
                                8u64
                            } else {
                                16u64
                            }
                        }
                    }
                } else {
                    let a7527 = v0;
                    let a7528 = 7u64;
                    if nat_lt(a7527, a7528) {
                        let a7529 = v0;
                        let a7530 = 6u64;
                        if nat_lt(a7529, a7530) {
                            32u64
                        } else {
                            64u64
                        }
                    } else {
                        let a7531 = v0;
                        let a7532 = 8u64;
                        if nat_lt(a7531, a7532) {
                            128u64
                        } else {
                            let a7533 = v0;
                            let a7534 = 9u64;
                            if nat_lt(a7533, a7534) {
                                256u64
                            } else {
                                512u64
                            }
                        }
                    }
                }
            } else {
                let a7535 = v0;
                let a7536 = 15u64;
                if nat_lt(a7535, a7536) {
                    let a7537 = v0;
                    let a7538 = 12u64;
                    if nat_lt(a7537, a7538) {
                        let a7539 = v0;
                        let a7540 = 11u64;
                        if nat_lt(a7539, a7540) {
                            1024u64
                        } else {
                            2048u64
                        }
                    } else {
                        let a7541 = v0;
                        let a7542 = 13u64;
                        if nat_lt(a7541, a7542) {
                            4096u64
                        } else {
                            let a7543 = v0;
                            let a7544 = 14u64;
                            if nat_lt(a7543, a7544) {
                                8192u64
                            } else {
                                16384u64
                            }
                        }
                    }
                } else {
                    let a7545 = v0;
                    let a7546 = 17u64;
                    if nat_lt(a7545, a7546) {
                        let a7547 = v0;
                        let a7548 = 16u64;
                        if nat_lt(a7547, a7548) {
                            32768u64
                        } else {
                            65536u64
                        }
                    } else {
                        let a7549 = v0;
                        let a7550 = 18u64;
                        if nat_lt(a7549, a7550) {
                            131072u64
                        } else {
                            let a7551 = v0;
                            let a7552 = 19u64;
                            if nat_lt(a7551, a7552) {
                                262144u64
                            } else {
                                524288u64
                            }
                        }
                    }
                }
            }
        } else {
            let a7553 = v0;
            let a7554 = 30u64;
            if nat_lt(a7553, a7554) {
                let a7555 = v0;
                let a7556 = 25u64;
                if nat_lt(a7555, a7556) {
                    let a7557 = v0;
                    let a7558 = 22u64;
                    if nat_lt(a7557, a7558) {
                        let a7559 = v0;
                        let a7560 = 21u64;
                        if nat_lt(a7559, a7560) {
                            1048576u64
                        } else {
                            2097152u64
                        }
                    } else {
                        let a7561 = v0;
                        let a7562 = 23u64;
                        if nat_lt(a7561, a7562) {
                            4194304u64
                        } else {
                            let a7563 = v0;
                            let a7564 = 24u64;
                            if nat_lt(a7563, a7564) {
                                8388608u64
                            } else {
                                16777216u64
                            }
                        }
                    }
                } else {
                    let a7565 = v0;
                    let a7566 = 27u64;
                    if nat_lt(a7565, a7566) {
                        let a7567 = v0;
                        let a7568 = 26u64;
                        if nat_lt(a7567, a7568) {
                            33554432u64
                        } else {
                            67108864u64
                        }
                    } else {
                        let a7569 = v0;
                        let a7570 = 28u64;
                        if nat_lt(a7569, a7570) {
                            134217728u64
                        } else {
                            let a7571 = v0;
                            let a7572 = 29u64;
                            if nat_lt(a7571, a7572) {
                                268435456u64
                            } else {
                                536870912u64
                            }
                        }
                    }
                }
            } else {
                let a7573 = v0;
                let a7574 = 35u64;
                if nat_lt(a7573, a7574) {
                    let a7575 = v0;
                    let a7576 = 32u64;
                    if nat_lt(a7575, a7576) {
                        let a7577 = v0;
                        let a7578 = 31u64;
                        if nat_lt(a7577, a7578) {
                            1073741824u64
                        } else {
                            2147483648u64
                        }
                    } else {
                        let a7579 = v0;
                        let a7580 = 33u64;
                        if nat_lt(a7579, a7580) {
                            4294967296u64
                        } else {
                            let a7581 = v0;
                            let a7582 = 34u64;
                            if nat_lt(a7581, a7582) {
                                8589934592u64
                            } else {
                                17179869184u64
                            }
                        }
                    }
                } else {
                    let a7583 = v0;
                    let a7584 = 37u64;
                    if nat_lt(a7583, a7584) {
                        let a7585 = v0;
                        let a7586 = 36u64;
                        if nat_lt(a7585, a7586) {
                            34359738368u64
                        } else {
                            68719476736u64
                        }
                    } else {
                        let a7587 = v0;
                        let a7588 = 38u64;
                        if nat_lt(a7587, a7588) {
                            137438953472u64
                        } else {
                            let a7589 = v0;
                            let a7590 = 39u64;
                            if nat_lt(a7589, a7590) {
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

pub fn f282(v0: u64, v1: u64) -> bool {
    let a7591 = v0;
    let a7592 = 1u64;
    if nat_le(a7591, a7592) {
        true
    } else {
        let a7596 = {
            let a7593 = v1;
            let a7594 = 4u64;
            let a7595 = 0u64;
            nat_rem(a7593, a7594, a7595)
        };
        let a7597 = 0u64;
        nat_eq(a7596, a7597)
    }
}

pub fn f283(v0: u64, v1: u64) -> bool {
    let a7598 = v0;
    let a7599 = 1u64;
    if nat_le(a7598, a7599) {
        true
    } else {
        let a7600 = 4u64;
        let a7601 = v1;
        nat_le(a7600, a7601)
    }
}

pub fn f284(v0: u64, v1: u64) -> R<u64> {
    let a7602 = v0;
    let a7603 = 0u64;
    if nat_eq(a7602, a7603) {
        Ok(0u64)
    } else {
        let a7608 = 4u64;
        let a7609 = {
            let a7606 = {
                let a7604 = v0;
                let a7605 = 1u64;
                nat_sub(a7604, a7605)
            };
            let a7607 = v1;
            nat_mul(a7606, a7607)?
        };
        nat_add(a7608, a7609)
    }
}

pub fn f285(v0: bool, v1: u64) -> R<u64> {
    let a7614 = {
        let a7612 = {
            let a7610 = f305(v0, v1)?;
            let a7611 = f306(v0, v1)?;
            nat_add(a7610, a7611)?
        };
        let a7613 = f307(v0);
        nat_add(a7612, a7613)?
    };
    let a7615 = v1;
    nat_add(a7614, a7615)
}

pub fn f286(v0: u64, v1: u64) -> R<u64> {
    let v2: u64 = f308(v0, v1)?;
    let v3: u64 = {
        let a7620 = {
            let a7618 = {
                let a7616 = v1;
                let a7617 = 1u64;
                nat_sub(a7616, a7617)
            };
            let a7619 = v2;
            nat_mul(a7618, a7619)?
        };
        let a7621 = v0;
        nat_sub(a7620, a7621)
    };
    let a7622 = v2;
    let a7623 = v3;
    if nat_lt(a7622, a7623) {
        Ok(v3)
    } else {
        Ok(v2)
    }
}

pub fn f287(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64) -> R<u64> {
    let a7624 = v4;
    let a7625 = 0u64;
    if nat_eq(a7624, a7625) {
        Ok(0u64)
    } else {
        let a7632 = 2u64;
        let a7633 = {
            let a7630 = {
                let a7626 = v0;
                let a7627 = v1;
                nat_mul(a7626, a7627)?
            };
            let a7631 = {
                let a7628 = v2;
                let a7629 = v3;
                nat_mul(a7628, a7629)?
            };
            nat_mul(a7630, a7631)?
        };
        nat_mul(a7632, a7633)
    }
}

pub fn f288(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64) -> R<u64> {
    let a7638 = v3;
    let a7639 = {
        let a7636 = {
            let a7634 = f309(v2, v4)?;
            let a7635 = f310(v0, v1, f308(v2, v4)?)?;
            nat_mul(a7634, a7635)?
        };
        let a7637 = f310(v0, v1, f311(v2, v4)?)?;
        nat_add(a7636, a7637)?
    };
    nat_mul(a7638, a7639)
}

pub fn f289(v0: u64) -> R<u64> {
    let a7642 = 13u64;
    let a7643 = {
        let a7640 = 9u64;
        let a7641 = v0;
        nat_mul(a7640, a7641)?
    };
    nat_add(a7642, a7643)
}

pub fn f290(v0: u64, v1: u64, _: u64, v3: u64, v4: u64, v5: u64, v6: u64, v7: u64, v8: u64, v9: u64, v10: u64, v11: u64, v12: u64, v13: u64, v14: u64) -> bool {
    if if if if if if if if if f312(v0, v1, v6, v7, v8) {
        let a7644 = v5;
        let a7645 = 26u64;
        nat_eq(a7644, a7645)
    } else {
        false
    } {
        let a7646 = v3;
        let a7647 = v7;
        nat_eq(a7646, a7647)
    } else {
        false
    } {
        let a7648 = v4;
        let a7649 = v8;
        nat_eq(a7648, a7649)
    } else {
        false
    } {
        let a7650 = v9;
        let a7651 = 1u64;
        nat_eq(a7650, a7651)
    } else {
        false
    } {
        let a7652 = v10;
        let a7653 = f313(v0);
        nat_eq(a7652, a7653)
    } else {
        false
    } {
        let a7654 = v11;
        let a7655 = v1;
        nat_eq(a7654, a7655)
    } else {
        false
    } {
        let a7656 = v12;
        let a7657 = v6;
        nat_eq(a7656, a7657)
    } else {
        false
    } {
        let a7658 = v13;
        let a7659 = v7;
        nat_eq(a7658, a7659)
    } else {
        false
    } {
        let a7660 = v14;
        let a7661 = v8;
        nat_eq(a7660, a7661)
    } else {
        false
    }
}

pub fn f291(v0: u64, v1: u64, v2: u64, v3: u64) -> bool {
    let a7662 = v0;
    let a7663 = 0u64;
    if if if nat_eq(a7662, a7663) {
        true
    } else {
        let a7664 = v1;
        let a7665 = 0u64;
        nat_eq(a7664, a7665)
    } {
        true
    } else {
        let a7666 = v2;
        let a7667 = 0u64;
        nat_eq(a7666, a7667)
    } {
        true
    } else {
        let a7668 = v3;
        let a7669 = 0u64;
        nat_eq(a7668, a7669)
    }
}

pub fn f292(v0: Adt3, v1: bool) -> R<u64> {
    f296(v0.clone(), {
        let a7671 = match v0.clone() {
            Adt3::C0(h7670, _, _, _, _, _, _, _) => {
                h7670
            }
        };
        let a7672 = 1u64;
        nat_sub(a7671, a7672)
    }, {
        let a7674 = match v0.clone() {
            Adt3::C0(_, h7673, _, _, _, _, _, _) => {
                h7673
            }
        };
        let a7675 = 1u64;
        nat_sub(a7674, a7675)
    }, {
        let a7677 = match v0.clone() {
            Adt3::C0(_, _, h7676, _, _, _, _, _) => {
                h7676
            }
        };
        let a7678 = 1u64;
        nat_sub(a7677, a7678)
    }, {
        let a7680 = match v0 {
            Adt3::C0(_, _, _, h7679, _, _, _, _) => {
                h7679
            }
        };
        let a7681 = 1u64;
        nat_sub(a7680, a7681)
    }, v1)
}

pub fn f293(v0: u64, v1: u64, v2: u64, v3: u64) -> R<u64> {
    let a7682 = v0;
    let a7683 = 0u64;
    if nat_eq(a7682, a7683) {
        Ok(0u64)
    } else {
        let v4: u64 = {
            let a7684 = v0;
            let a7685 = 1u64;
            nat_sub(a7684, a7685)
        };
        let v5: u64 = {
            let a7686 = v4;
            let a7687 = v1;
            let a7688 = 0u64;
            nat_quot(a7686, a7687, a7688)
        };
        let v6: u64 = {
            let a7696 = {
                let a7689 = v5;
                let a7690 = v2;
                nat_mul(a7689, a7690)?
            };
            let a7697 = {
                let a7694 = {
                    let a7691 = v4;
                    let a7692 = v1;
                    let a7693 = 0u64;
                    nat_rem(a7691, a7692, a7693)
                };
                let a7695 = v3;
                nat_mul(a7694, a7695)?
            };
            nat_add(a7696, a7697)?
        };
        let a7698 = v5;
        let a7699 = 0u64;
        if nat_eq(a7698, a7699) {
            Ok(v6)
        } else {
            let v7: u64 = {
                let a7708 = {
                    let a7702 = {
                        let a7700 = v5;
                        let a7701 = 1u64;
                        nat_sub(a7700, a7701)
                    };
                    let a7703 = v2;
                    nat_mul(a7702, a7703)?
                };
                let a7709 = {
                    let a7706 = {
                        let a7704 = v1;
                        let a7705 = 1u64;
                        nat_sub(a7704, a7705)
                    };
                    let a7707 = v3;
                    nat_mul(a7706, a7707)?
                };
                nat_add(a7708, a7709)?
            };
            let a7710 = v6;
            let a7711 = v7;
            if nat_lt(a7710, a7711) {
                Ok(v7)
            } else {
                Ok(v6)
            }
        }
    }
}

pub fn f294(v0: Adt3, v1: u64) -> bool {
    let a7712 = v1;
    let a7713 = 0u64;
    if !nat_eq(a7712, a7713) {
        true
    } else {
        let a7715 = match v0 {
            Adt3::C0(_, _, _, _, h7714, _, _, _) => {
                h7714
            }
        };
        let a7716 = 4u64;
        nat_eq(a7715, a7716)
    }
}

pub fn f295(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: bool, v6: u64, v7: u64, v8: u64, v9: u64, v10: u64, v11: u64, v12: u64, v13: u64, v14: u64, v15: u64, v16: u64, v17: u64) -> R<Adt10> {
    let m7717 = f314(v0, v1);
    match m7717 {
        None => {
            if v5 {
                let m7718 = f315(v0, v2, v3, v4, v6, v7, v8, v9, v10, v11, v12, v13, v14, v15, v16, v17)?;
                match m7718 {
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
                let a7719 = 4u64;
                let a7720 = v28;
                nat_le(a7719, a7720)
            };
            let a7721 = v2;
            let a7722 = 0u64;
            if if if if if nat_eq(a7721, a7722) {
                let a7723 = v4;
                let a7724 = 0u64;
                nat_eq(a7723, a7724)
            } else {
                false
            } {
                let a7725 = v6;
                let a7726 = v14;
                if nat_eq(a7725, a7726) {
                    let a7727 = v7;
                    let a7728 = v15;
                    nat_eq(a7727, a7728)
                } else {
                    false
                }
            } else {
                false
            } {
                let a7729 = v8;
                let a7730 = v16;
                if nat_eq(a7729, a7730) {
                    let a7731 = v9;
                    let a7732 = v17;
                    nat_eq(a7731, a7732)
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
                        let a7733 = v3;
                        let a7734 = 0u64;
                        nat_eq(a7733, a7734)
                    } else {
                        false
                    } {
                        f246(v6, v7, v8, v9, v10, v11, v12, v13)
                    } else {
                        false
                    }
                }
            } else {
                false
            } {
                Ok(Adt10::C2(v28, v29, v6, v7, v8, v9, {
                    let a7739 = {
                        let a7735 = v6;
                        let a7736 = v7;
                        nat_mul(a7735, a7736)?
                    };
                    let a7740 = {
                        let a7737 = v8;
                        let a7738 = v9;
                        nat_mul(a7737, a7738)?
                    };
                    nat_mul(a7739, a7740)?
                }))
            } else {
                Ok(Adt10::C1)
            }
        }
    }
}

pub fn f296(v0: Adt3, v1: u64, v2: u64, v3: u64, v4: u64, v5: bool) -> R<u64> {
    let a7761 = {
        let a7755 = {
            let a7749 = {
                let a7743 = v1;
                let a7744 = if v5 {
                    f159(match v0.clone() {
                        Adt3::C0(_, _, _, _, h7741, _, _, _) => {
                            h7741
                        }
                    })
                } else {
                    f160(match v0.clone() {
                        Adt3::C0(_, _, _, _, h7742, _, _, _) => {
                            h7742
                        }
                    })
                };
                nat_mul(a7743, a7744)?
            };
            let a7750 = {
                let a7747 = v2;
                let a7748 = if v5 {
                    f159(match v0.clone() {
                        Adt3::C0(_, _, _, _, _, h7745, _, _) => {
                            h7745
                        }
                    })
                } else {
                    f160(match v0.clone() {
                        Adt3::C0(_, _, _, _, _, h7746, _, _) => {
                            h7746
                        }
                    })
                };
                nat_mul(a7747, a7748)?
            };
            nat_add(a7749, a7750)?
        };
        let a7756 = {
            let a7753 = v3;
            let a7754 = if v5 {
                f159(match v0.clone() {
                    Adt3::C0(_, _, _, _, _, _, h7751, _) => {
                        h7751
                    }
                })
            } else {
                f160(match v0.clone() {
                    Adt3::C0(_, _, _, _, _, _, h7752, _) => {
                        h7752
                    }
                })
            };
            nat_mul(a7753, a7754)?
        };
        nat_add(a7755, a7756)?
    };
    let a7762 = {
        let a7759 = v4;
        let a7760 = if v5 {
            f159(match v0 {
                Adt3::C0(_, _, _, _, _, _, _, h7757) => {
                    h7757
                }
            })
        } else {
            f160(match v0 {
                Adt3::C0(_, _, _, _, _, _, _, h7758) => {
                    h7758
                }
            })
        };
        nat_mul(a7759, a7760)?
    };
    nat_add(a7761, a7762)
}

pub fn f297(v0: u64, v1: u64, v2: Adt3, v3: u64, v4: u64) -> R<Adt9> {
    let a7763 = v1;
    let a7764 = 0u64;
    if nat_eq(a7763, a7764) {
        Ok(Adt9::C4({
            let a7765 = v3;
            let a7766 = 0u64;
            if nat_eq(a7765, a7766) {
                1u64
            } else {
                6u64
            }
        }, 0u64, f105(v0, v2)?, 2u64, 0u64, 1u64, 2u64))
    } else {
        Ok(Adt9::C4({
            let a7767 = v3;
            let a7768 = 0u64;
            if nat_eq(a7767, a7768) {
                0u64
            } else {
                4u64
            }
        }, 1u64, v4, 1u64, 2u64, 1u64, v4))
    }
}

pub fn f298(v0: u64, v1: u64) -> R<u64> {
    let a7769 = v0;
    let a7770 = f110(v1)?;
    let a7771 = 0u64;
    Ok(nat_quot(a7769, a7770, a7771))
}

pub fn f299(v0: u64, v1: u64) -> R<u64> {
    let v2: u64 = f298(v0, v1)?;
    let v3: u64 = {
        let a7774 = {
            let a7772 = f269(v0, v1)?;
            let a7773 = 2u64;
            nat_sub(a7772, a7773)
        };
        let a7775 = 2u64;
        let a7776 = 0u64;
        nat_quot(a7774, a7775, a7776)
    };
    let v4: u64 = {
        let a7777 = v2;
        let a7778 = v1;
        nat_mul(a7777, a7778)?
    };
    if f272(v0, v1)? {
        let a7779 = v4;
        let a7780 = v3;
        nat_add(a7779, a7780)
    } else {
        let a7792 = {
            let a7788 = v4;
            let a7789 = {
                let a7786 = {
                    let a7781 = v1;
                    let a7782 = 1u64;
                    nat_sub(a7781, a7782)
                };
                let a7787 = {
                    let a7783 = v2;
                    let a7784 = v1;
                    let a7785 = 0u64;
                    nat_rem(a7783, a7784, a7785)
                };
                nat_mul(a7786, a7787)?
            };
            nat_sub(a7788, a7789)
        };
        let a7793 = {
            let a7790 = v3;
            let a7791 = v1;
            nat_mul(a7790, a7791)?
        };
        nat_add(a7792, a7793)
    }
}

pub fn f300(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let a7794 = v0;
    let a7795 = 0u64;
    if if if nat_eq(a7794, a7795) {
        true
    } else {
        let a7796 = v1;
        let a7797 = 0u64;
        nat_eq(a7796, a7797)
    } {
        true
    } else {
        let a7798 = v2;
        let a7799 = 0u64;
        nat_eq(a7798, a7799)
    } {
        Ok(0u64)
    } else {
        let a7802 = {
            let a7800 = v0;
            let a7801 = v1;
            nat_mul(a7800, a7801)?
        };
        let a7803 = v2;
        nat_mul(a7802, a7803)
    }
}

pub fn f301(v0: u64, v1: Adt20, v2: Adt20, v3: u64, v4: u64) -> R<Adt17> {
    let v5: u64 = f119(match v1.clone() {
        Adt20::C0(h7804, _, _, _, _, _, _, _) => {
            h7804
        }
    })?;
    let v6: u64 = {
        let a7810 = match v1.clone() {
            Adt20::C0(_, h7805, _, _, _, _, _, _) => {
                h7805
            }
        };
        let a7811 = {
            let a7808 = match v1.clone() {
                Adt20::C0(_, _, h7806, _, _, _, _, _) => {
                    h7806
                }
            };
            let a7809 = match v1.clone() {
                Adt20::C0(_, _, _, h7807, _, _, _, _) => {
                    h7807
                }
            };
            nat_mul(a7808, a7809)?
        };
        nat_mul(a7810, a7811)?
    };
    let a7813 = v0;
    let a7814 = f60(match v1.clone() {
        Adt20::C0(h7812, _, _, _, _, _, _, _) => {
            h7812
        }
    }, v6)?;
    if nat_lt(a7813, a7814) {
        f316({
            let a7815 = v0;
            let a7816 = v5;
            let a7817 = 0u64;
            nat_rem(a7815, a7816, a7817)
        }, {
            let a7818 = v0;
            let a7819 = v5;
            let a7820 = 0u64;
            nat_quot(a7818, a7819, a7820)
        }, v1, v2, v3, v4)
    } else {
        Ok(Adt17::C0)
    }
}

pub fn f302(v0: u64, v1: Adt20, v2: Adt20, v3: Adt20, v4: u64, v5: u64, v6: u64, v7: u64) -> R<Adt17> {
    let v8: u64 = {
        let a7821 = v6;
        let a7822 = 6u64;
        nat_sub(a7821, a7822)
    };
    let v9: Adt20 = f317(v3);
    let a7824 = v0;
    let a7825 = f232(match v1.clone() {
        Adt20::C0(h7823, _, _, _, _, _, _, _) => {
            h7823
        }
    }, v5, v8)?;
    if nat_lt(a7824, a7825) {
        let v10: u64 = f116(match v1.clone() {
            Adt20::C0(h7826, _, _, _, _, _, _, _) => {
                h7826
            }
        }, v5, v8)?;
        let a7827 = v0;
        let a7828 = v10;
        if nat_lt(a7827, a7828) {
            f318(v0, v9, 4u64, 6u64, v7)
        } else {
            let v11: u64 = {
                let a7829 = v0;
                let a7830 = v10;
                nat_sub(a7829, a7830)
            };
            let v12: u64 = f289(match v1.clone() {
                Adt20::C0(h7831, _, _, _, _, _, _, _) => {
                    h7831
                }
            })?;
            f319({
                let a7832 = v11;
                let a7833 = v12;
                let a7834 = 0u64;
                nat_rem(a7832, a7833, a7834)
            }, {
                let a7835 = v11;
                let a7836 = v12;
                let a7837 = 0u64;
                nat_quot(a7835, a7836, a7837)
            }, v1, v2, v9, v4)
        }
    } else {
        Ok(Adt17::C0)
    }
}

pub fn f303(v0: u64, v1: Adt20, v2: Adt20, v3: u64, v4: u64, v5: u64, v6: u64) -> R<Adt17> {
    let v7: u64 = {
        let a7838 = v5;
        let a7839 = 6u64;
        nat_sub(a7838, a7839)
    };
    let v8: bool = f226(match v1.clone() {
        Adt20::C0(_, _, _, _, h7840, _, _, _) => {
            h7840
        }
    }, match v2.clone() {
        Adt20::C0(_, _, _, _, h7841, _, _, _) => {
            h7841
        }
    });
    let a7843 = v0;
    let a7844 = f225(v8, match v1.clone() {
        Adt20::C0(h7842, _, _, _, _, _, _, _) => {
            h7842
        }
    }, v4, v7)?;
    if nat_lt(a7843, a7844) {
        let v9: u64 = f116(match v1.clone() {
            Adt20::C0(h7845, _, _, _, _, _, _, _) => {
                h7845
            }
        }, v4, v7)?;
        let a7846 = v0;
        let a7847 = v9;
        if nat_lt(a7846, a7847) {
            f318(v0, v2, 4u64, 6u64, v6)
        } else {
            let v10: u64 = {
                let a7848 = v0;
                let a7849 = v9;
                nat_sub(a7848, a7849)
            };
            let v11: u64 = f285(v8, match v1.clone() {
                Adt20::C0(h7850, _, _, _, _, _, _, _) => {
                    h7850
                }
            })?;
            f320({
                let a7851 = v10;
                let a7852 = v11;
                let a7853 = 0u64;
                nat_rem(a7851, a7852, a7853)
            }, {
                let a7854 = v10;
                let a7855 = v11;
                let a7856 = 0u64;
                nat_quot(a7854, a7855, a7856)
            }, v1, v2, v3, v8)
        }
    } else {
        Ok(Adt17::C0)
    }
}

pub fn f304(v0: u64, v1: Adt20, v2: Adt20, v3: u64, v4: u64, v5: u64, v6: u64) -> R<Adt17> {
    let v7: u64 = {
        let a7857 = v5;
        let a7858 = 6u64;
        nat_sub(a7857, a7858)
    };
    let v8: Adt20 = f317(v2);
    let v9: u64 = f287(match v1.clone() {
        Adt20::C0(h7859, _, _, _, _, _, _, _) => {
            h7859
        }
    }, match v1.clone() {
        Adt20::C0(_, h7860, _, _, _, _, _, _) => {
            h7860
        }
    }, match v1.clone() {
        Adt20::C0(_, _, h7861, _, _, _, _, _) => {
            h7861
        }
    }, match v1.clone() {
        Adt20::C0(_, _, _, h7862, _, _, _, _) => {
            h7862
        }
    }, v7)?;
    let a7867 = v0;
    let a7868 = f231(match v1.clone() {
        Adt20::C0(h7863, _, _, _, _, _, _, _) => {
            h7863
        }
    }, match v1.clone() {
        Adt20::C0(_, h7864, _, _, _, _, _, _) => {
            h7864
        }
    }, match v1.clone() {
        Adt20::C0(_, _, h7865, _, _, _, _, _) => {
            h7865
        }
    }, match v1.clone() {
        Adt20::C0(_, _, _, h7866, _, _, _, _) => {
            h7866
        }
    }, v4, v7)?;
    if nat_lt(a7867, a7868) {
        let a7869 = v0;
        let a7870 = v9;
        if nat_lt(a7869, a7870) {
            f318(v0, v8, 4u64, 6u64, v6)
        } else {
            let v10: Adt21 = f321({
                let a7871 = v0;
                let a7872 = v9;
                nat_sub(a7871, a7872)
            }, match v1.clone() {
                Adt20::C0(h7873, _, _, _, _, _, _, _) => {
                    h7873
                }
            }, match v1.clone() {
                Adt20::C0(_, h7874, _, _, _, _, _, _) => {
                    h7874
                }
            }, match v1.clone() {
                Adt20::C0(_, _, h7875, _, _, _, _, _) => {
                    h7875
                }
            }, match v1.clone() {
                Adt20::C0(_, _, _, h7876, _, _, _, _) => {
                    h7876
                }
            }, v4)?;
            f322(v10, v1, v8, v3)
        }
    } else {
        Ok(Adt17::C0)
    }
}

pub fn f305(v0: bool, v1: u64) -> R<u64> {
    let a7879 = if v0 {
        5u64
    } else {
        4u64
    };
    let a7880 = {
        let a7877 = f323(v0);
        let a7878 = v1;
        nat_mul(a7877, a7878)?
    };
    nat_add(a7879, a7880)
}

pub fn f306(v0: bool, v1: u64) -> R<u64> {
    if v0 {
        let a7888 = {
            let a7881 = 13u64;
            let a7882 = f324(v1);
            nat_mul(a7881, a7882)?
        };
        let a7889 = {
            let a7886 = 4u64;
            let a7887 = {
                let a7883 = v1;
                let a7884 = 4u64;
                let a7885 = 0u64;
                nat_rem(a7883, a7884, a7885)
            };
            nat_mul(a7886, a7887)?
        };
        nat_add(a7888, a7889)
    } else {
        let a7890 = 3u64;
        let a7891 = v1;
        nat_mul(a7890, a7891)
    }
}

pub fn f307(v0: bool) -> u64 {
    if v0 {
        6u64
    } else {
        5u64
    }
}

pub fn f308(v0: u64, v1: u64) -> R<u64> {
    let a7896 = {
        let a7894 = {
            let a7892 = v0;
            let a7893 = v1;
            nat_add(a7892, a7893)?
        };
        let a7895 = 1u64;
        nat_sub(a7894, a7895)
    };
    let a7897 = v1;
    let a7898 = 0u64;
    Ok(nat_quot(a7896, a7897, a7898))
}

pub fn f309(v0: u64, v1: u64) -> R<u64> {
    let a7899 = v0;
    let a7900 = f308(v0, v1)?;
    let a7901 = 0u64;
    Ok(nat_quot(a7899, a7900, a7901))
}

pub fn f310(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let a7902 = v2;
    let a7903 = 0u64;
    if nat_eq(a7902, a7903) {
        Ok(0u64)
    } else {
        let a7908 = 11u64;
        let a7909 = {
            let a7906 = {
                let a7904 = v1;
                let a7905 = v2;
                nat_mul(a7904, a7905)?
            };
            let a7907 = f325(v0)?;
            nat_mul(a7906, a7907)?
        };
        nat_add(a7908, a7909)
    }
}

pub fn f311(v0: u64, v1: u64) -> R<u64> {
    let a7910 = v0;
    let a7911 = f308(v0, v1)?;
    let a7912 = 0u64;
    Ok(nat_rem(a7910, a7911, a7912))
}

pub fn f312(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64) -> bool {
    if f291(v1, v2, v3, v4) {
        f326(v0)
    } else {
        f243(v0)
    }
}

pub fn f313(v0: u64) -> u64 {
    let a7913 = v0;
    let a7914 = 26u64;
    if nat_eq(a7913, a7914) {
        26u64
    } else {
        0u64
    }
}

pub fn f314(v0: u64, v1: u64) -> Option<u64> {
    let a7915 = v0;
    let a7916 = 2u64;
    if nat_eq(a7915, a7916) {
        Some(0u64)
    } else {
        let a7917 = v0;
        let a7918 = 5u64;
        if nat_eq(a7917, a7918) {
            Some(1u64)
        } else {
            let a7919 = v0;
            let a7920 = 6u64;
            if nat_eq(a7919, a7920) {
                Some(2u64)
            } else {
                let a7921 = v0;
                let a7922 = 7u64;
                if nat_eq(a7921, a7922) {
                    Some(3u64)
                } else {
                    let a7923 = v0;
                    let a7924 = 9u64;
                    if nat_eq(a7923, a7924) {
                        Some(4u64)
                    } else {
                        let a7925 = v0;
                        let a7926 = 8u64;
                        if nat_eq(a7925, a7926) {
                            Some(5u64)
                        } else {
                            let a7927 = v0;
                            let a7928 = 89u64;
                            if nat_eq(a7927, a7928) {
                                let a7929 = v1;
                                let a7930 = 2u64;
                                if nat_eq(a7929, a7930) {
                                    Some(6u64)
                                } else {
                                    let a7931 = v1;
                                    let a7932 = 0u64;
                                    if nat_eq(a7931, a7932) {
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

pub fn f315(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64, v7: u64, v8: u64, v9: u64, v10: u64, v11: u64, v12: u64, v13: u64, v14: u64, v15: u64) -> R<Adt22> {
    let a7933 = v0;
    let a7934 = 30u64;
    if nat_eq(a7933, a7934) {
        let a7935 = v2;
        let a7936 = 0u64;
        if if if nat_eq(a7935, a7936) {
            true
        } else {
            let a7937 = v2;
            let a7938 = 1u64;
            nat_eq(a7937, a7938)
        } {
            let a7939 = v1;
            let a7940 = 0u64;
            if nat_eq(a7939, a7940) {
                true
            } else {
                f327(v1, v4)
            }
        } else {
            false
        } {
            f328(28u64, 0u64, 0u64, v3, v5, v4, v6, v7, v9, v8, v10, v11, v12, v13, v14, v15)
        } else {
            Ok(Adt22::C0)
        }
    } else {
        if if if if f329(v1) {
            f329(v2)
        } else {
            false
        } {
            let a7941 = v1;
            let a7942 = 0u64;
            if if nat_eq(a7941, a7942) {
                true
            } else {
                let a7943 = v2;
                let a7944 = 0u64;
                nat_eq(a7943, a7944)
            } {
                true
            } else {
                let a7945 = v1;
                let a7946 = v2;
                nat_eq(a7945, a7946)
            }
        } else {
            false
        } {
            true
        } else {
            let a7947 = v1;
            let a7948 = 43u64;
            if nat_eq(a7947, a7948) {
                f329(v2)
            } else {
                false
            }
        } {
            f328(v0, 0u64, 0u64, v3, v4, v5, v6, v7, v8, v9, v10, v11, v12, v13, v14, v15)
        } else {
            Ok(Adt22::C0)
        }
    }
}

pub fn f316(v0: u64, v1: u64, v2: Adt20, v3: Adt20, v4: u64, v5: u64) -> R<Adt17> {
    let v6: u64 = match v2.clone() {
        Adt20::C0(h7949, _, _, _, _, _, _, _) => {
            h7949
        }
    };
    let v7: u64 = {
        let a7952 = 1u64;
        let a7953 = {
            let a7950 = 3u64;
            let a7951 = v6;
            nat_mul(a7950, a7951)?
        };
        nat_add(a7952, a7953)?
    };
    let a7954 = v0;
    let a7955 = 0u64;
    if nat_eq(a7954, a7955) {
        Ok(f330(0u64))
    } else {
        let a7956 = v0;
        let a7957 = v7;
        if nat_lt(a7956, a7957) {
            let v8: u64 = {
                let a7958 = v0;
                let a7959 = 1u64;
                nat_sub(a7958, a7959)
            };
            let v9: u64 = {
                let a7965 = {
                    let a7960 = v1;
                    let a7961 = v6;
                    nat_mul(a7960, a7961)?
                };
                let a7966 = {
                    let a7962 = v8;
                    let a7963 = 3u64;
                    let a7964 = 0u64;
                    nat_quot(a7962, a7963, a7964)
                };
                nat_add(a7965, a7966)?
            };
            let v10: u64 = f331(v9, v2)?;
            let v11: u64 = {
                let a7967 = v8;
                let a7968 = 3u64;
                let a7969 = 0u64;
                nat_rem(a7967, a7968, a7969)
            };
            let a7970 = v11;
            let a7971 = 0u64;
            if nat_eq(a7970, a7971) {
                Ok(f332(10u64, 0u64, v10, 0u64, v10, 1u64, 1u64))
            } else {
                let a7972 = v11;
                let a7973 = 1u64;
                if nat_eq(a7972, a7973) {
                    Ok(f332(1u64, 1u64, 1u64, 2u64, 0u64, 1u64, 1u64))
                } else {
                    Ok(f332(0u64, 1u64, 0u64, 1u64, 1u64, 1u64, 0u64))
                }
            }
        } else {
            let a7974 = v0;
            let a7975 = v7;
            if nat_eq(a7974, a7975) {
                Ok(f332(2u64, 1u64, 0u64, 2u64, 0u64, 1u64, 1u64))
            } else {
                let a7978 = v0;
                let a7979 = {
                    let a7976 = v7;
                    let a7977 = 1u64;
                    nat_add(a7976, a7977)?
                };
                if nat_eq(a7978, a7979) {
                    Ok(f332(12u64, 1u64, 1u64, 2u64, 0u64, 1u64, 1u64))
                } else {
                    let a7982 = v0;
                    let a7983 = {
                        let a7980 = v7;
                        let a7981 = 2u64;
                        nat_add(a7980, a7981)?
                    };
                    if nat_eq(a7982, a7983) {
                        Ok(f332(16u64, 1u64, 1u64, 2u64, v4, 1u64, 2u64))
                    } else {
                        let a7986 = v0;
                        let a7987 = {
                            let a7984 = v7;
                            let a7985 = 3u64;
                            nat_add(a7984, a7985)?
                        };
                        if nat_eq(a7986, a7987) {
                            let a7988 = v5;
                            let a7989 = 1u64;
                            if if nat_eq(a7988, a7989) {
                                true
                            } else {
                                let a7990 = v5;
                                let a7991 = 2u64;
                                nat_eq(a7990, a7991)
                            } {
                                Ok(f332(11u64, 2u64, 1065353216u64, 1u64, 1u64, 1u64, 2u64))
                            } else {
                                Ok(f332(11u64, 2u64, 1065353216u64, 2u64, v4, 1u64, 2u64))
                            }
                        } else {
                            let v12: u64 = {
                                let a7998 = {
                                    let a7992 = v1;
                                    let a7993 = v6;
                                    nat_mul(a7992, a7993)?
                                };
                                let a7999 = {
                                    let a7996 = v0;
                                    let a7997 = {
                                        let a7994 = v7;
                                        let a7995 = 4u64;
                                        nat_add(a7994, a7995)?
                                    };
                                    nat_sub(a7996, a7997)
                                };
                                nat_add(a7998, a7999)?
                            };
                            Ok(f332(10u64, 0u64, f331(v12, v2)?, 1u64, 2u64, 3u64, f331(v12, v3)?))
                        }
                    }
                }
            }
        }
    }
}

pub fn f317(v0: Adt20) -> Adt20 {
    Adt20::C0(match v0.clone() {
        Adt20::C0(h8000, _, _, _, _, _, _, _) => {
            h8000
        }
    }, match v0.clone() {
        Adt20::C0(_, h8001, _, _, _, _, _, _) => {
            h8001
        }
    }, match v0.clone() {
        Adt20::C0(_, _, h8002, _, _, _, _, _) => {
            h8002
        }
    }, match v0.clone() {
        Adt20::C0(_, _, _, h8003, _, _, _, _) => {
            h8003
        }
    }, 4u64, match v0.clone() {
        Adt20::C0(_, _, _, _, _, h8004, _, _) => {
            h8004
        }
    }, match v0.clone() {
        Adt20::C0(_, _, _, _, _, _, h8005, _) => {
            h8005
        }
    }, match v0 {
        Adt20::C0(_, _, _, _, _, _, _, h8006) => {
            h8006
        }
    })
}

pub fn f318(v0: u64, v1: Adt20, v2: u64, v3: u64, v4: u64) -> R<Adt17> {
    let v5: u64 = {
        let a8013 = v3;
        let a8014 = {
            let a8010 = f331({
                let a8007 = v0;
                let a8008 = 2u64;
                let a8009 = 0u64;
                nat_quot(a8007, a8008, a8009)
            }, v1)?;
            let a8011 = v2;
            let a8012 = 0u64;
            nat_quot(a8010, a8011, a8012)
        };
        nat_add(a8013, a8014)?
    };
    let a8018 = {
        let a8015 = v0;
        let a8016 = 2u64;
        let a8017 = 0u64;
        nat_rem(a8015, a8016, a8017)
    };
    let a8019 = 0u64;
    if nat_eq(a8018, a8019) {
        Ok(f332(19u64, 1u64, v5, 2u64, 0u64, 1u64, 2u64))
    } else {
        let a8020 = v4;
        let a8021 = 0u64;
        if nat_eq(a8020, a8021) {
            Ok(f332(19u64, 2u64, 1u64, 2u64, 0u64, 1u64, v5))
        } else {
            Ok(Adt17::C0)
        }
    }
}

pub fn f319(v0: u64, v1: u64, v2: Adt20, v3: Adt20, v4: Adt20, v5: u64) -> R<Adt17> {
    let v6: u64 = f333(match v2.clone() {
        Adt20::C0(h8022, _, _, _, _, _, _, _) => {
            h8022
        }
    })?;
    let v7: u64 = f334(match v2.clone() {
        Adt20::C0(h8023, _, _, _, _, _, _, _) => {
            h8023
        }
    })?;
    let a8024 = v0;
    let a8025 = 2u64;
    if nat_lt(a8024, a8025) {
        Ok(f330(v0))
    } else {
        let a8026 = v0;
        let a8027 = v6;
        if nat_lt(a8026, a8027) {
            f335({
                let a8028 = v0;
                let a8029 = 2u64;
                nat_sub(a8028, a8029)
            }, v1, v2, v3)
        } else {
            let a8030 = v0;
            let a8031 = v7;
            if nat_lt(a8030, a8031) {
                Ok(f336({
                    let a8032 = v0;
                    let a8033 = v6;
                    nat_sub(a8032, a8033)
                }, match v2 {
                    Adt20::C0(h8034, _, _, _, _, _, _, _) => {
                        h8034
                    }
                }, v5))
            } else {
                let a8036 = v0;
                let a8037 = f289(match v2.clone() {
                    Adt20::C0(h8035, _, _, _, _, _, _, _) => {
                        h8035
                    }
                })?;
                if nat_lt(a8036, a8037) {
                    f337({
                        let a8038 = v0;
                        let a8039 = v7;
                        nat_sub(a8038, a8039)
                    }, v1, v2, v3, v4)
                } else {
                    Ok(Adt17::C0)
                }
            }
        }
    }
}

pub fn f320(v0: u64, v1: u64, v2: Adt20, v3: Adt20, v4: u64, v5: bool) -> R<Adt17> {
    let v6: u64 = f305(v5, match v2.clone() {
        Adt20::C0(h8040, _, _, _, _, _, _, _) => {
            h8040
        }
    })?;
    let v7: u64 = {
        let a8042 = v6;
        let a8043 = f306(v5, match v2.clone() {
            Adt20::C0(h8041, _, _, _, _, _, _, _) => {
                h8041
            }
        })?;
        nat_add(a8042, a8043)?
    };
    let v8: u64 = {
        let a8044 = v7;
        let a8045 = f307(v5);
        nat_add(a8044, a8045)?
    };
    let a8046 = v0;
    let a8047 = v6;
    if nat_lt(a8046, a8047) {
        f338(v0, v1, v2, v5)
    } else {
        let a8048 = v0;
        let a8049 = v7;
        if nat_lt(a8048, a8049) {
            f339({
                let a8050 = v0;
                let a8051 = v6;
                nat_sub(a8050, a8051)
            }, v1, v2, v3, v5)
        } else {
            let a8052 = v0;
            let a8053 = v8;
            if nat_lt(a8052, a8053) {
                Ok(f340({
                    let a8054 = v0;
                    let a8055 = v7;
                    nat_sub(a8054, a8055)
                }, match v2 {
                    Adt20::C0(h8056, _, _, _, _, _, _, _) => {
                        h8056
                    }
                }, v4, v5))
            } else {
                let a8060 = v0;
                let a8061 = {
                    let a8058 = v8;
                    let a8059 = match v2.clone() {
                        Adt20::C0(h8057, _, _, _, _, _, _, _) => {
                            h8057
                        }
                    };
                    nat_add(a8058, a8059)?
                };
                if nat_lt(a8060, a8061) {
                    let v9: u64 = f331({
                        let a8067 = {
                            let a8063 = v1;
                            let a8064 = match v2 {
                                Adt20::C0(h8062, _, _, _, _, _, _, _) => {
                                    h8062
                                }
                            };
                            nat_mul(a8063, a8064)?
                        };
                        let a8068 = {
                            let a8065 = v0;
                            let a8066 = v8;
                            nat_sub(a8065, a8066)
                        };
                        nat_add(a8067, a8068)?
                    }, v3)?;
                    Ok(f332(10u64, 3u64, v9, 1u64, 1u64, 3u64, v9))
                } else {
                    Ok(Adt17::C0)
                }
            }
        }
    }
}

pub fn f321(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64) -> R<Adt21> {
    let v6: u64 = f308(v3, v5)?;
    let v7: u64 = f310(v1, v2, v6)?;
    let v8: u64 = {
        let a8071 = {
            let a8069 = f309(v3, v5)?;
            let a8070 = v4;
            nat_mul(a8069, a8070)?
        };
        let a8072 = v7;
        nat_mul(a8071, a8072)?
    };
    let a8073 = v0;
    let a8074 = v8;
    if nat_lt(a8073, a8074) {
        let v9: u64 = {
            let a8075 = v0;
            let a8076 = v7;
            let a8077 = 0u64;
            nat_quot(a8075, a8076, a8077)
        };
        Ok(Adt21::C0({
            let a8081 = {
                let a8078 = v9;
                let a8079 = v4;
                let a8080 = 0u64;
                nat_quot(a8078, a8079, a8080)
            };
            let a8082 = v6;
            nat_mul(a8081, a8082)?
        }, v6, {
            let a8083 = v9;
            let a8084 = v4;
            let a8085 = 0u64;
            nat_rem(a8083, a8084, a8085)
        }, {
            let a8086 = v0;
            let a8087 = v7;
            let a8088 = 0u64;
            nat_rem(a8086, a8087, a8088)
        }))
    } else {
        let v10: u64 = {
            let a8089 = v0;
            let a8090 = v8;
            nat_sub(a8089, a8090)
        };
        let v11: u64 = f310(v1, v2, f311(v3, v5)?)?;
        Ok(Adt21::C0({
            let a8091 = f309(v3, v5)?;
            let a8092 = v6;
            nat_mul(a8091, a8092)?
        }, f311(v3, v5)?, {
            let a8093 = v10;
            let a8094 = v11;
            let a8095 = 0u64;
            nat_quot(a8093, a8094, a8095)
        }, {
            let a8096 = v10;
            let a8097 = v11;
            let a8098 = 0u64;
            nat_rem(a8096, a8097, a8098)
        }))
    }
}

pub fn f322(v0: Adt21, v1: Adt20, v2: Adt20, v3: u64) -> R<Adt17> {
    let v4: u64 = match v0.clone() {
        Adt21::C0(_, _, _, h8099) => {
            h8099
        }
    };
    let v5: u64 = match v1.clone() {
        Adt20::C0(h8100, _, _, _, _, _, _, _) => {
            h8100
        }
    };
    let v6: u64 = {
        let a8103 = match v0.clone() {
            Adt21::C0(_, h8101, _, _) => {
                h8101
            }
        };
        let a8104 = match v1.clone() {
            Adt20::C0(_, h8102, _, _, _, _, _, _) => {
                h8102
            }
        };
        nat_mul(a8103, a8104)?
    };
    let v7: u64 = {
        let a8105 = v6;
        let a8106 = v5;
        nat_mul(a8105, a8106)?
    };
    let v8: u64 = {
        let a8109 = {
            let a8107 = 2u64;
            let a8108 = v5;
            nat_mul(a8107, a8108)?
        };
        let a8110 = 2u64;
        nat_add(a8109, a8110)?
    };
    let v9: u64 = {
        let a8113 = {
            let a8111 = 4u64;
            let a8112 = v5;
            nat_mul(a8111, a8112)?
        };
        let a8114 = 2u64;
        nat_add(a8113, a8114)?
    };
    let v10: u64 = {
        let a8117 = 1u64;
        let a8118 = {
            let a8115 = v6;
            let a8116 = v8;
            nat_mul(a8115, a8116)?
        };
        nat_add(a8117, a8118)?
    };
    let v11: u64 = {
        let a8119 = v10;
        let a8120 = 4u64;
        nat_add(a8119, a8120)?
    };
    let v12: u64 = {
        let a8123 = v11;
        let a8124 = {
            let a8121 = v6;
            let a8122 = v9;
            nat_mul(a8121, a8122)?
        };
        nat_add(a8123, a8124)?
    };
    let v13: u64 = {
        let a8125 = v12;
        let a8126 = 6u64;
        nat_add(a8125, a8126)?
    };
    let a8127 = v4;
    let a8128 = 0u64;
    if nat_eq(a8127, a8128) {
        Ok(f330(0u64))
    } else {
        let a8129 = v4;
        let a8130 = v10;
        if nat_lt(a8129, a8130) {
            let v14: u64 = {
                let a8131 = v4;
                let a8132 = 1u64;
                nat_sub(a8131, a8132)
            };
            f341(false, {
                let a8133 = v14;
                let a8134 = v8;
                let a8135 = 0u64;
                nat_rem(a8133, a8134, a8135)
            }, {
                let a8136 = v14;
                let a8137 = v8;
                let a8138 = 0u64;
                nat_quot(a8136, a8137, a8138)
            }, v0, v1, v2)
        } else {
            let a8139 = v4;
            let a8140 = v11;
            if nat_lt(a8139, a8140) {
                Ok(f342({
                    let a8141 = v4;
                    let a8142 = v10;
                    nat_sub(a8141, a8142)
                }, v7))
            } else {
                let a8143 = v4;
                let a8144 = v12;
                if nat_lt(a8143, a8144) {
                    let v15: u64 = {
                        let a8145 = v4;
                        let a8146 = v11;
                        nat_sub(a8145, a8146)
                    };
                    f341(true, {
                        let a8147 = v15;
                        let a8148 = v9;
                        let a8149 = 0u64;
                        nat_rem(a8147, a8148, a8149)
                    }, {
                        let a8150 = v15;
                        let a8151 = v9;
                        let a8152 = 0u64;
                        nat_quot(a8150, a8151, a8152)
                    }, v0, v1, v2)
                } else {
                    let a8153 = v4;
                    let a8154 = v13;
                    if nat_lt(a8153, a8154) {
                        Ok(f343({
                            let a8155 = v4;
                            let a8156 = v12;
                            nat_sub(a8155, a8156)
                        }, v7, v3))
                    } else {
                        let a8159 = v4;
                        let a8160 = {
                            let a8157 = v13;
                            let a8158 = v7;
                            nat_add(a8157, a8158)?
                        };
                        if nat_lt(a8159, a8160) {
                            let v16: u64 = {
                                let a8161 = v4;
                                let a8162 = v13;
                                nat_sub(a8161, a8162)
                            };
                            let v17: u64 = {
                                let a8163 = v16;
                                let a8164 = v5;
                                let a8165 = 0u64;
                                nat_quot(a8163, a8164, a8165)
                            };
                            let v18: u64 = {
                                let a8171 = {
                                    let a8166 = f344(v17, v0, v1)?;
                                    let a8167 = v5;
                                    nat_mul(a8166, a8167)?
                                };
                                let a8172 = {
                                    let a8168 = v16;
                                    let a8169 = v5;
                                    let a8170 = 0u64;
                                    nat_rem(a8168, a8169, a8170)
                                };
                                nat_add(a8171, a8172)?
                            };
                            let v19: u64 = f331(v18, v2)?;
                            Ok(f332(10u64, 3u64, v19, 1u64, 1u64, 3u64, v19))
                        } else {
                            Ok(Adt17::C0)
                        }
                    }
                }
            }
        }
    }
}

pub fn f323(v0: bool) -> u64 {
    if v0 {
        2u64
    } else {
        1u64
    }
}

pub fn f324(v0: u64) -> u64 {
    let a8173 = v0;
    let a8174 = 4u64;
    let a8175 = 0u64;
    nat_quot(a8173, a8174, a8175)
}

pub fn f325(v0: u64) -> R<u64> {
    let a8178 = {
        let a8176 = 7u64;
        let a8177 = v0;
        nat_mul(a8176, a8177)?
    };
    let a8179 = 4u64;
    nat_add(a8178, a8179)
}

pub fn f326(v0: u64) -> bool {
    let m8180 = f235(v0);
    match m8180 {
        Adt18::C0 => {
            false
        }
        Adt18::C1(_) => {
            true
        }
    }
}

pub fn f327(v0: u64, v1: u64) -> bool {
    let a8181 = v0;
    let a8182 = 1u64;
    if nat_eq(a8181, a8182) {
        true
    } else {
        let v2: u64 = f345(v0);
        let a8183 = 0u64;
        let a8184 = v2;
        if nat_lt(a8183, a8184) {
            let a8188 = {
                let a8185 = v1;
                let a8186 = v2;
                let a8187 = 1u64;
                nat_rem(a8185, a8186, a8187)
            };
            let a8189 = 0u64;
            nat_eq(a8188, a8189)
        } else {
            false
        }
    }
}

pub fn f328(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64, v7: u64, v8: u64, v9: u64, v10: u64, v11: u64, v12: u64, v13: u64, v14: u64, v15: u64) -> R<Adt22> {
    let a8190 = v0;
    let a8191 = 28u64;
    if if if if nat_eq(a8190, a8191) {
        let a8192 = v1;
        let a8193 = 0u64;
        nat_eq(a8192, a8193)
    } else {
        false
    } {
        let a8194 = v2;
        let a8195 = 0u64;
        nat_eq(a8194, a8195)
    } else {
        false
    } {
        let a8196 = v3;
        let a8197 = 0u64;
        nat_eq(a8196, a8197)
    } else {
        false
    } {
        let a8198 = v4;
        let a8199 = v8;
        if if if if if if if if nat_eq(a8198, a8199) {
            let a8200 = v5;
            let a8201 = v12;
            nat_eq(a8200, a8201)
        } else {
            false
        } {
            let a8202 = v9;
            let a8203 = v13;
            nat_eq(a8202, a8203)
        } else {
            false
        } {
            let a8204 = v10;
            let a8205 = v14;
            nat_eq(a8204, a8205)
        } else {
            false
        } {
            let a8206 = v11;
            let a8207 = v15;
            nat_eq(a8206, a8207)
        } else {
            false
        } {
            let a8208 = 0u64;
            let a8209 = v6;
            nat_lt(a8208, a8209)
        } else {
            false
        } {
            let a8210 = 0u64;
            let a8211 = v7;
            nat_lt(a8210, a8211)
        } else {
            false
        } {
            let a8215 = {
                let a8212 = v10;
                let a8213 = v6;
                let a8214 = 1u64;
                nat_rem(a8212, a8213, a8214)
            };
            let a8216 = 0u64;
            if nat_eq(a8215, a8216) {
                let a8220 = {
                    let a8217 = v11;
                    let a8218 = v7;
                    let a8219 = 1u64;
                    nat_rem(a8217, a8218, a8219)
                };
                let a8221 = 0u64;
                nat_eq(a8220, a8221)
            } else {
                false
            }
        } else {
            false
        } {
            Ok(Adt22::C2(v5, v4, v9, v10, v11, {
                let a8222 = v10;
                let a8223 = v6;
                let a8224 = 0u64;
                nat_quot(a8222, a8223, a8224)
            }, {
                let a8225 = v11;
                let a8226 = v7;
                let a8227 = 0u64;
                nat_quot(a8225, a8226, a8227)
            }, {
                let a8228 = v5;
                let a8229 = v4;
                nat_mul(a8228, a8229)?
            }, {
                let a8230 = v4;
                let a8231 = v9;
                nat_mul(a8230, a8231)?
            }, {
                let a8232 = v5;
                let a8233 = v9;
                nat_mul(a8232, a8233)?
            }))
        } else {
            Ok(Adt22::C1)
        }
    } else {
        Ok(Adt22::C0)
    }
}

pub fn f329(v0: u64) -> bool {
    let a8234 = v0;
    let a8235 = 0u64;
    if if nat_eq(a8234, a8235) {
        true
    } else {
        let a8236 = v0;
        let a8237 = 1u64;
        nat_eq(a8236, a8237)
    } {
        true
    } else {
        let a8238 = v0;
        let a8239 = 30u64;
        nat_eq(a8238, a8239)
    }
}

pub fn f330(v0: u64) -> Adt17 {
    f332(0u64, 2u64, 0u64, 2u64, 0u64, 1u64, v0)
}

pub fn f331(v0: u64, v1: Adt20) -> R<u64> {
    let v2: u64 = {
        let a8241 = v0;
        let a8242 = match v1.clone() {
            Adt20::C0(h8240, _, _, _, _, _, _, _) => {
                h8240
            }
        };
        let a8243 = 0u64;
        nat_quot(a8241, a8242, a8243)
    };
    let v3: u64 = {
        let a8245 = v2;
        let a8246 = match v1.clone() {
            Adt20::C0(_, h8244, _, _, _, _, _, _) => {
                h8244
            }
        };
        let a8247 = 0u64;
        nat_quot(a8245, a8246, a8247)
    };
    f158({
        let a8249 = v0;
        let a8250 = match v1.clone() {
            Adt20::C0(h8248, _, _, _, _, _, _, _) => {
                h8248
            }
        };
        let a8251 = 0u64;
        nat_rem(a8249, a8250, a8251)
    }, {
        let a8253 = v2;
        let a8254 = match v1.clone() {
            Adt20::C0(_, h8252, _, _, _, _, _, _) => {
                h8252
            }
        };
        let a8255 = 0u64;
        nat_rem(a8253, a8254, a8255)
    }, {
        let a8257 = v3;
        let a8258 = match v1.clone() {
            Adt20::C0(_, _, h8256, _, _, _, _, _) => {
                h8256
            }
        };
        let a8259 = 0u64;
        nat_rem(a8257, a8258, a8259)
    }, {
        let a8261 = v3;
        let a8262 = match v1.clone() {
            Adt20::C0(_, _, h8260, _, _, _, _, _) => {
                h8260
            }
        };
        let a8263 = 0u64;
        nat_quot(a8261, a8262, a8263)
    }, match v1.clone() {
        Adt20::C0(_, _, _, _, h8264, _, _, _) => {
            h8264
        }
    }, match v1.clone() {
        Adt20::C0(_, _, _, _, _, h8265, _, _) => {
            h8265
        }
    }, match v1.clone() {
        Adt20::C0(_, _, _, _, _, _, h8266, _) => {
            h8266
        }
    }, match v1 {
        Adt20::C0(_, _, _, _, _, _, _, h8267) => {
            h8267
        }
    })
}

pub fn f332(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64) -> Adt17 {
    let a8268 = v0;
    let a8269 = 4u64;
    if if if nat_le(a8268, a8269) {
        true
    } else {
        let a8270 = 9u64;
        let a8271 = v0;
        if nat_le(a8270, a8271) {
            let a8272 = v0;
            let a8273 = 12u64;
            nat_le(a8272, a8273)
        } else {
            false
        }
    } {
        true
    } else {
        let a8274 = v0;
        let a8275 = 16u64;
        if if nat_eq(a8274, a8275) {
            true
        } else {
            let a8276 = v0;
            let a8277 = 19u64;
            nat_eq(a8276, a8277)
        } {
            true
        } else {
            let a8278 = v0;
            let a8279 = 21u64;
            if nat_eq(a8278, a8279) {
                true
            } else {
                let a8280 = v0;
                let a8281 = 22u64;
                nat_eq(a8280, a8281)
            }
        }
    } {
        Adt17::C1({
            let a8282 = v0;
            let a8283 = 4u64;
            if if nat_lt(a8282, a8283) {
                true
            } else {
                let a8284 = 21u64;
                let a8285 = v0;
                nat_le(a8284, a8285)
            } {
                0u64
            } else {
                let a8286 = v0;
                let a8287 = 16u64;
                if if nat_eq(a8286, a8287) {
                    true
                } else {
                    let a8288 = v0;
                    let a8289 = 19u64;
                    nat_eq(a8288, a8289)
                } {
                    2u64
                } else {
                    1u64
                }
            }
        }, {
            let a8290 = v0;
            let a8291 = 4u64;
            if nat_lt(a8290, a8291) {
                v0
            } else {
                let a8292 = v0;
                let a8293 = 4u64;
                if if nat_eq(a8292, a8293) {
                    true
                } else {
                    let a8294 = v0;
                    let a8295 = 16u64;
                    nat_eq(a8294, a8295)
                } {
                    0u64
                } else {
                    let a8296 = v0;
                    let a8297 = 19u64;
                    if nat_eq(a8296, a8297) {
                        3u64
                    } else {
                        let a8298 = 21u64;
                        let a8299 = v0;
                        if nat_le(a8298, a8299) {
                            let a8300 = v0;
                            let a8301 = 17u64;
                            nat_sub(a8300, a8301)
                        } else {
                            let a8302 = v0;
                            let a8303 = 8u64;
                            nat_sub(a8302, a8303)
                        }
                    }
                }
            }
        }, v1, v2, v3, v4, v5, v6)
    } else {
        Adt17::C0
    }
}

pub fn f333(v0: u64) -> R<u64> {
    let a8306 = 2u64;
    let a8307 = {
        let a8304 = 6u64;
        let a8305 = v0;
        nat_mul(a8304, a8305)?
    };
    nat_add(a8306, a8307)
}

pub fn f334(v0: u64) -> R<u64> {
    let a8308 = f333(v0)?;
    let a8309 = 11u64;
    nat_add(a8308, a8309)
}

pub fn f335(v0: u64, v1: u64, v2: Adt20, v3: Adt20) -> R<Adt17> {
    let v4: u64 = {
        let a8310 = v0;
        let a8311 = 6u64;
        let a8312 = 0u64;
        nat_rem(a8310, a8311, a8312)
    };
    let v5: u64 = {
        let a8319 = {
            let a8314 = v1;
            let a8315 = match v2.clone() {
                Adt20::C0(h8313, _, _, _, _, _, _, _) => {
                    h8313
                }
            };
            nat_mul(a8314, a8315)?
        };
        let a8320 = {
            let a8316 = v0;
            let a8317 = 6u64;
            let a8318 = 0u64;
            nat_quot(a8316, a8317, a8318)
        };
        nat_add(a8319, a8320)?
    };
    let v6: u64 = f331(v5, v3)?;
    let v7: u64 = f331(v5, v2)?;
    let a8321 = v4;
    let a8322 = 0u64;
    if nat_eq(a8321, a8322) {
        Ok(f332(10u64, 4u64, v6, 4u64, v6, 1u64, 2u64))
    } else {
        let a8323 = v4;
        let a8324 = 1u64;
        if nat_eq(a8323, a8324) {
            Ok(f332(1u64, 1u64, 2u64, 2u64, 0u64, 1u64, 2u64))
        } else {
            let a8325 = v4;
            let a8326 = 2u64;
            if nat_eq(a8325, a8326) {
                Ok(f332(0u64, 1u64, 0u64, 1u64, 2u64, 1u64, 0u64))
            } else {
                let a8327 = v4;
                let a8328 = 3u64;
                if nat_eq(a8327, a8328) {
                    Ok(f332(10u64, 4u64, v6, 0u64, v7, 1u64, 2u64))
                } else {
                    let a8329 = v4;
                    let a8330 = 4u64;
                    if nat_eq(a8329, a8330) {
                        Ok(f332(1u64, 1u64, 2u64, 2u64, 0u64, 1u64, 2u64))
                    } else {
                        Ok(f332(0u64, 1u64, 1u64, 1u64, 2u64, 1u64, 1u64))
                    }
                }
            }
        }
    }
}

pub fn f336(v0: u64, v1: u64, v2: u64) -> Adt17 {
    let a8331 = v0;
    let a8332 = 0u64;
    if nat_eq(a8331, a8332) {
        f332(2u64, 1u64, 0u64, 2u64, 0u64, 1u64, 3u64)
    } else {
        let a8333 = v0;
        let a8334 = 1u64;
        if nat_eq(a8333, a8334) {
            f332(3u64, 2u64, v1, 2u64, 0u64, 1u64, 2u64)
        } else {
            let a8335 = v0;
            let a8336 = 2u64;
            if nat_eq(a8335, a8336) {
                f332(11u64, 1u64, 3u64, 1u64, 2u64, 1u64, 4u64)
            } else {
                let a8337 = v0;
                let a8338 = 3u64;
                if nat_eq(a8337, a8338) {
                    f332(4u64, 1u64, 4u64, 2u64, v2, 1u64, 4u64)
                } else {
                    let a8339 = v0;
                    let a8340 = 4u64;
                    if nat_eq(a8339, a8340) {
                        f332(12u64, 1u64, 4u64, 2u64, 0u64, 1u64, 4u64)
                    } else {
                        let a8341 = v0;
                        let a8342 = 5u64;
                        if nat_eq(a8341, a8342) {
                            f332(11u64, 2u64, 1065353216u64, 1u64, 4u64, 1u64, 4u64)
                        } else {
                            let a8343 = v0;
                            let a8344 = 6u64;
                            if nat_eq(a8343, a8344) {
                                f332(10u64, 2u64, v2, 1u64, 2u64, 1u64, 5u64)
                            } else {
                                let a8345 = v0;
                                let a8346 = 7u64;
                                if nat_eq(a8345, a8346) {
                                    f332(4u64, 1u64, 3u64, 1u64, 5u64, 1u64, 5u64)
                                } else {
                                    let a8347 = v0;
                                    let a8348 = 8u64;
                                    if nat_eq(a8347, a8348) {
                                        f332(2u64, 1u64, 1u64, 2u64, 0u64, 1u64, 2u64)
                                    } else {
                                        let a8349 = v0;
                                        let a8350 = 9u64;
                                        if nat_eq(a8349, a8350) {
                                            Adt17::C1(3u64, 0u64, 1u64, 2u64, 2u64, 2u64, 1u64, 2u64)
                                        } else {
                                            let a8351 = v0;
                                            let a8352 = 10u64;
                                            if nat_eq(a8351, a8352) {
                                                f332(11u64, 1u64, 2u64, 1u64, 5u64, 1u64, 5u64)
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

pub fn f337(v0: u64, v1: u64, v2: Adt20, v3: Adt20, v4: Adt20) -> R<Adt17> {
    let v5: u64 = {
        let a8353 = v0;
        let a8354 = 3u64;
        let a8355 = 0u64;
        nat_rem(a8353, a8354, a8355)
    };
    let v6: u64 = {
        let a8362 = {
            let a8357 = v1;
            let a8358 = match v2.clone() {
                Adt20::C0(h8356, _, _, _, _, _, _, _) => {
                    h8356
                }
            };
            nat_mul(a8357, a8358)?
        };
        let a8363 = {
            let a8359 = v0;
            let a8360 = 3u64;
            let a8361 = 0u64;
            nat_quot(a8359, a8360, a8361)
        };
        nat_add(a8362, a8363)?
    };
    let v7: u64 = f331(v6, v3)?;
    let v8: u64 = f331(v6, v2)?;
    let v9: u64 = f331(v6, v4)?;
    let a8364 = v5;
    let a8365 = 0u64;
    if nat_eq(a8364, a8365) {
        Ok(f332(10u64, 4u64, v7, 1u64, 5u64, 1u64, 2u64))
    } else {
        let a8366 = v5;
        let a8367 = 1u64;
        if nat_eq(a8366, a8367) {
            Ok(f332(4u64, 0u64, v8, 1u64, 2u64, 1u64, 2u64))
        } else {
            Ok(f332(10u64, 1u64, 2u64, 1u64, 4u64, 3u64, v9))
        }
    }
}

pub fn f338(v0: u64, v1: u64, v2: Adt20, v3: bool) -> R<Adt17> {
    let v4: u64 = match v2.clone() {
        Adt20::C0(h8368, _, _, _, _, _, _, _) => {
            h8368
        }
    };
    let v5: u64 = {
        let a8371 = 1u64;
        let a8372 = {
            let a8369 = v4;
            let a8370 = f323(v3);
            nat_mul(a8369, a8370)?
        };
        nat_add(a8371, a8372)?
    };
    let a8373 = v0;
    let a8374 = 0u64;
    if nat_eq(a8373, a8374) {
        Ok(f330(0u64))
    } else {
        let a8375 = v0;
        let a8376 = v5;
        if nat_lt(a8375, a8376) {
            let v6: u64 = {
                let a8377 = v0;
                let a8378 = 1u64;
                nat_sub(a8377, a8378)
            };
            let v7: u64 = f331({
                let a8384 = {
                    let a8379 = v1;
                    let a8380 = v4;
                    nat_mul(a8379, a8380)?
                };
                let a8385 = {
                    let a8381 = v6;
                    let a8382 = f323(v3);
                    let a8383 = 0u64;
                    nat_quot(a8381, a8382, a8383)
                };
                nat_add(a8384, a8385)?
            }, v2)?;
            if v3 {
                let a8389 = {
                    let a8386 = v6;
                    let a8387 = 2u64;
                    let a8388 = 0u64;
                    nat_rem(a8386, a8387, a8388)
                };
                let a8390 = 0u64;
                if nat_eq(a8389, a8390) {
                    Ok(f332(1u64, 0u64, v7, 2u64, 0u64, 1u64, 1u64))
                } else {
                    Ok(f332(0u64, 1u64, 0u64, 1u64, 1u64, 1u64, 0u64))
                }
            } else {
                Ok(f332(4u64, 1u64, 0u64, 0u64, v7, 1u64, 0u64))
            }
        } else {
            let v8: u64 = {
                let a8391 = v0;
                let a8392 = v5;
                nat_sub(a8391, a8392)
            };
            let a8393 = v8;
            let a8394 = 0u64;
            if nat_eq(a8393, a8394) {
                Ok(f332(3u64, 2u64, v4, 2u64, 0u64, 1u64, 1u64))
            } else {
                if if v3 {
                    let a8395 = v8;
                    let a8396 = 1u64;
                    nat_eq(a8395, a8396)
                } else {
                    false
                } {
                    Ok(f332(2u64, 1u64, 0u64, 2u64, 0u64, 1u64, 0u64))
                } else {
                    let a8397 = v8;
                    let a8398 = if v3 {
                        2u64
                    } else {
                        1u64
                    };
                    if nat_eq(a8397, a8398) {
                        Ok(f332(11u64, 1u64, 0u64, 1u64, 1u64, 1u64, 3u64))
                    } else {
                        let a8399 = v8;
                        let a8400 = if v3 {
                            3u64
                        } else {
                            2u64
                        };
                        if nat_eq(a8399, a8400) {
                            Ok(f330(0u64))
                        } else {
                            Ok(Adt17::C0)
                        }
                    }
                }
            }
        }
    }
}

pub fn f339(v0: u64, v1: u64, v2: Adt20, v3: Adt20, v4: bool) -> R<Adt17> {
    let v5: u64 = if v4 {
        let a8402 = 13u64;
        let a8403 = f324(match v2.clone() {
            Adt20::C0(h8401, _, _, _, _, _, _, _) => {
                h8401
            }
        });
        nat_mul(a8402, a8403)?
    } else {
        0u64
    };
    let a8404 = v0;
    let a8405 = v5;
    if nat_lt(a8404, a8405) {
        f346({
            let a8406 = v0;
            let a8407 = 13u64;
            let a8408 = 0u64;
            nat_rem(a8406, a8407, a8408)
        }, v1, {
            let a8409 = v0;
            let a8410 = 13u64;
            let a8411 = 0u64;
            nat_quot(a8409, a8410, a8411)
        }, v2, v3)
    } else {
        let v6: u64 = {
            let a8412 = v0;
            let a8413 = v5;
            nat_sub(a8412, a8413)
        };
        let v7: u64 = if v4 {
            4u64
        } else {
            3u64
        };
        f347({
            let a8414 = v6;
            let a8415 = v7;
            let a8416 = 0u64;
            nat_rem(a8414, a8415, a8416)
        }, {
            let a8426 = {
                let a8421 = {
                    let a8418 = v1;
                    let a8419 = match v2.clone() {
                        Adt20::C0(h8417, _, _, _, _, _, _, _) => {
                            h8417
                        }
                    };
                    nat_mul(a8418, a8419)?
                };
                let a8422 = if v4 {
                    f348(match v2.clone() {
                        Adt20::C0(h8420, _, _, _, _, _, _, _) => {
                            h8420
                        }
                    })?
                } else {
                    0u64
                };
                nat_add(a8421, a8422)?
            };
            let a8427 = {
                let a8423 = v6;
                let a8424 = v7;
                let a8425 = 0u64;
                nat_quot(a8423, a8424, a8425)
            };
            nat_add(a8426, a8427)?
        }, v2, v3, v4)
    }
}

pub fn f340(v0: u64, v1: u64, v2: u64, v3: bool) -> Adt17 {
    let a8428 = v0;
    let a8429 = 0u64;
    if nat_eq(a8428, a8429) {
        f332(if v3 {
            22u64
        } else {
            3u64
        }, 2u64, v1, 2u64, 0u64, 1u64, 1u64)
    } else {
        let a8430 = v0;
        let a8431 = 1u64;
        if nat_eq(a8430, a8431) {
            f332(if v3 {
                21u64
            } else {
                11u64
            }, 1u64, 0u64, 1u64, 1u64, 1u64, 1u64)
        } else {
            if if v3 {
                let a8432 = v0;
                let a8433 = 2u64;
                nat_eq(a8432, a8433)
            } else {
                false
            } {
                f332(2u64, 1u64, 1u64, 2u64, 0u64, 1u64, 1u64)
            } else {
                let a8434 = v0;
                let a8435 = if v3 {
                    3u64
                } else {
                    2u64
                };
                if nat_eq(a8434, a8435) {
                    f332(4u64, 1u64, 1u64, 2u64, v2, 1u64, 1u64)
                } else {
                    let a8436 = v0;
                    let a8437 = if v3 {
                        4u64
                    } else {
                        3u64
                    };
                    if nat_eq(a8436, a8437) {
                        f332(12u64, 1u64, 1u64, 2u64, 0u64, 1u64, 1u64)
                    } else {
                        let a8438 = v0;
                        let a8439 = if v3 {
                            5u64
                        } else {
                            4u64
                        };
                        if nat_eq(a8438, a8439) {
                            f332(11u64, 2u64, 1065353216u64, 1u64, 1u64, 1u64, 1u64)
                        } else {
                            Adt17::C0
                        }
                    }
                }
            }
        }
    }
}

pub fn f341(v0: bool, v1: u64, v2: u64, v3: Adt21, v4: Adt20, v5: Adt20) -> R<Adt17> {
    let v6: u64 = if v0 {
        4u64
    } else {
        2u64
    };
    let v7: u64 = {
        let a8443 = 1u64;
        let a8444 = {
            let a8441 = match v4.clone() {
                Adt20::C0(h8440, _, _, _, _, _, _, _) => {
                    h8440
                }
            };
            let a8442 = v6;
            nat_mul(a8441, a8442)?
        };
        nat_add(a8443, a8444)?
    };
    let a8445 = v1;
    let a8446 = 0u64;
    if nat_eq(a8445, a8446) {
        Ok(f330(1u64))
    } else {
        let a8447 = v1;
        let a8448 = v7;
        if nat_lt(a8447, a8448) {
            let v8: u64 = {
                let a8449 = v1;
                let a8450 = 1u64;
                nat_sub(a8449, a8450)
            };
            let v9: u64 = {
                let a8457 = {
                    let a8452 = f344(v2, v3, v4.clone())?;
                    let a8453 = match v4.clone() {
                        Adt20::C0(h8451, _, _, _, _, _, _, _) => {
                            h8451
                        }
                    };
                    nat_mul(a8452, a8453)?
                };
                let a8458 = {
                    let a8454 = v8;
                    let a8455 = v6;
                    let a8456 = 0u64;
                    nat_quot(a8454, a8455, a8456)
                };
                nat_add(a8457, a8458)?
            };
            let v10: u64 = f331(v9, v4)?;
            let v11: u64 = f331(v9, v5)?;
            let v12: u64 = {
                let a8459 = v8;
                let a8460 = v6;
                let a8461 = 0u64;
                nat_rem(a8459, a8460, a8461)
            };
            if v0 {
                let a8462 = v12;
                let a8463 = 0u64;
                if nat_eq(a8462, a8463) {
                    Ok(f332(9u64, 0u64, v10, 1u64, 3u64, 3u64, v11))
                } else {
                    let a8464 = v12;
                    let a8465 = 1u64;
                    if nat_eq(a8464, a8465) {
                        Ok(f332(10u64, 3u64, v11, 3u64, v11, 1u64, 2u64))
                    } else {
                        let a8466 = v12;
                        let a8467 = 2u64;
                        if nat_eq(a8466, a8467) {
                            Ok(f332(1u64, 1u64, 2u64, 2u64, 0u64, 1u64, 2u64))
                        } else {
                            Ok(f332(0u64, 1u64, 1u64, 1u64, 2u64, 1u64, 1u64))
                        }
                    }
                }
            } else {
                let a8468 = v12;
                let a8469 = 0u64;
                if nat_eq(a8468, a8469) {
                    Ok(f332(1u64, 0u64, v10, 2u64, 0u64, 1u64, 2u64))
                } else {
                    Ok(f332(0u64, 1u64, 1u64, 1u64, 2u64, 1u64, 1u64))
                }
            }
        } else {
            let a8470 = v1;
            let a8471 = v7;
            if nat_eq(a8470, a8471) {
                Ok(f332(0u64, 1u64, 0u64, 1u64, 1u64, 1u64, 0u64))
            } else {
                Ok(Adt17::C0)
            }
        }
    }
}

pub fn f342(v0: u64, v1: u64) -> Adt17 {
    let a8472 = v0;
    let a8473 = 0u64;
    if nat_eq(a8472, a8473) {
        f332(22u64, 2u64, v1, 2u64, 0u64, 1u64, 2u64)
    } else {
        let a8474 = v0;
        let a8475 = 1u64;
        if nat_eq(a8474, a8475) {
            f332(21u64, 1u64, 0u64, 1u64, 2u64, 1u64, 3u64)
        } else {
            let a8476 = v0;
            let a8477 = 2u64;
            if nat_eq(a8476, a8477) {
                f332(2u64, 1u64, 3u64, 2u64, 0u64, 1u64, 3u64)
            } else {
                let a8478 = v0;
                let a8479 = 3u64;
                if nat_eq(a8478, a8479) {
                    f330(0u64)
                } else {
                    Adt17::C0
                }
            }
        }
    }
}

pub fn f343(v0: u64, v1: u64, v2: u64) -> Adt17 {
    let a8480 = v0;
    let a8481 = 0u64;
    if nat_eq(a8480, a8481) {
        f332(22u64, 2u64, v1, 2u64, 0u64, 1u64, 2u64)
    } else {
        let a8482 = v0;
        let a8483 = 1u64;
        if nat_eq(a8482, a8483) {
            f332(21u64, 1u64, 0u64, 1u64, 2u64, 1u64, 2u64)
        } else {
            let a8484 = v0;
            let a8485 = 2u64;
            if nat_eq(a8484, a8485) {
                f332(2u64, 1u64, 2u64, 2u64, 0u64, 1u64, 2u64)
            } else {
                let a8486 = v0;
                let a8487 = 3u64;
                if nat_eq(a8486, a8487) {
                    f332(4u64, 1u64, 2u64, 2u64, v2, 1u64, 1u64)
                } else {
                    let a8488 = v0;
                    let a8489 = 4u64;
                    if nat_eq(a8488, a8489) {
                        f332(12u64, 1u64, 1u64, 2u64, 0u64, 1u64, 1u64)
                    } else {
                        let a8490 = v0;
                        let a8491 = 5u64;
                        if nat_eq(a8490, a8491) {
                            f332(11u64, 2u64, 1065353216u64, 1u64, 1u64, 1u64, 1u64)
                        } else {
                            Adt17::C0
                        }
                    }
                }
            }
        }
    }
}

pub fn f344(v0: u64, v1: Adt21, v2: Adt20) -> R<u64> {
    let a8502 = {
        let a8500 = {
            let a8497 = {
                let a8494 = match v1.clone() {
                    Adt21::C0(_, _, h8492, _) => {
                        h8492
                    }
                };
                let a8495 = match v2.clone() {
                    Adt20::C0(_, _, h8493, _, _, _, _, _) => {
                        h8493
                    }
                };
                nat_mul(a8494, a8495)?
            };
            let a8498 = match v1 {
                Adt21::C0(h8496, _, _, _) => {
                    h8496
                }
            };
            nat_add(a8497, a8498)?
        };
        let a8501 = match v2 {
            Adt20::C0(_, h8499, _, _, _, _, _, _) => {
                h8499
            }
        };
        nat_mul(a8500, a8501)?
    };
    let a8503 = v0;
    nat_add(a8502, a8503)
}

pub fn f345(v0: u64) -> u64 {
    if f243(v0) {
        let m8504 = f235(v0);
        match m8504 {
            Adt18::C0 => {
                0u64
            }
            Adt18::C1(v1) => {
                let a8506 = 1u64;
                let a8507 = match v1.clone() {
                    Adt19::C0(_, h8505, _) => {
                        h8505
                    }
                };
                if if nat_lt(a8506, a8507) {
                    true
                } else {
                    let a8508 = v0;
                    let a8509 = 43u64;
                    nat_eq(a8508, a8509)
                } {
                    match v1 {
                        Adt19::C0(_, h8510, _) => {
                            h8510
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

pub fn f346(v0: u64, v1: u64, v2: u64, v3: Adt20, v4: Adt20) -> R<Adt17> {
    let v5: u64 = {
        let a8518 = {
            let a8516 = {
                let a8512 = v1;
                let a8513 = match v3.clone() {
                    Adt20::C0(h8511, _, _, _, _, _, _, _) => {
                        h8511
                    }
                };
                nat_mul(a8512, a8513)?
            };
            let a8517 = {
                let a8514 = v2;
                let a8515 = 4u64;
                nat_mul(a8514, a8515)?
            };
            nat_add(a8516, a8517)?
        };
        let a8519 = f349(v0);
        nat_add(a8518, a8519)?
    };
    let v6: u64 = f331(v5, v3)?;
    let v7: u64 = f331(v5, v4)?;
    let a8520 = v0;
    let a8521 = 0u64;
    if if if if nat_eq(a8520, a8521) {
        true
    } else {
        let a8522 = v0;
        let a8523 = 2u64;
        nat_eq(a8522, a8523)
    } {
        true
    } else {
        let a8524 = v0;
        let a8525 = 5u64;
        nat_eq(a8524, a8525)
    } {
        true
    } else {
        let a8526 = v0;
        let a8527 = 7u64;
        nat_eq(a8526, a8527)
    } {
        Ok(f332(9u64, 0u64, v6, 1u64, 3u64, 3u64, v7))
    } else {
        let a8528 = v0;
        let a8529 = 1u64;
        if if if if nat_eq(a8528, a8529) {
            true
        } else {
            let a8530 = v0;
            let a8531 = 3u64;
            nat_eq(a8530, a8531)
        } {
            true
        } else {
            let a8532 = v0;
            let a8533 = 6u64;
            nat_eq(a8532, a8533)
        } {
            true
        } else {
            let a8534 = v0;
            let a8535 = 8u64;
            nat_eq(a8534, a8535)
        } {
            Ok(f332(10u64, 3u64, v7, 3u64, v7, 1u64, {
                let a8536 = v0;
                let a8537 = 1u64;
                if nat_eq(a8536, a8537) {
                    4u64
                } else {
                    let a8538 = v0;
                    let a8539 = 6u64;
                    if nat_eq(a8538, a8539) {
                        5u64
                    } else {
                        1u64
                    }
                }
            }))
        } else {
            let a8540 = v0;
            let a8541 = 4u64;
            if nat_eq(a8540, a8541) {
                Ok(f332(4u64, 1u64, 4u64, 1u64, 1u64, 1u64, 4u64))
            } else {
                let a8542 = v0;
                let a8543 = 9u64;
                if nat_eq(a8542, a8543) {
                    Ok(f332(4u64, 1u64, 5u64, 1u64, 1u64, 1u64, 5u64))
                } else {
                    let a8544 = v0;
                    let a8545 = 10u64;
                    if nat_eq(a8544, a8545) {
                        Ok(f332(4u64, 1u64, 4u64, 1u64, 5u64, 1u64, 1u64))
                    } else {
                        let a8546 = v0;
                        let a8547 = 11u64;
                        if nat_eq(a8546, a8547) {
                            Ok(f332(1u64, 1u64, 1u64, 2u64, 0u64, 1u64, 1u64))
                        } else {
                            let a8548 = v0;
                            let a8549 = 12u64;
                            if nat_eq(a8548, a8549) {
                                Ok(f332(0u64, 1u64, 0u64, 1u64, 1u64, 1u64, 0u64))
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

pub fn f347(v0: u64, v1: u64, v2: Adt20, v3: Adt20, v4: bool) -> R<Adt17> {
    let v5: u64 = f331(v1, v2)?;
    let v6: u64 = f331(v1, v3)?;
    let a8550 = v0;
    let a8551 = 0u64;
    if nat_eq(a8550, a8551) {
        Ok(f332(9u64, 0u64, v5, 1u64, 3u64, 3u64, v6))
    } else {
        let a8552 = v0;
        let a8553 = 1u64;
        if nat_eq(a8552, a8553) {
            Ok(f332(10u64, 3u64, v6, 3u64, v6, 1u64, 1u64))
        } else {
            if v4 {
                let a8554 = v0;
                let a8555 = 2u64;
                if nat_eq(a8554, a8555) {
                    Ok(f332(1u64, 1u64, 1u64, 2u64, 0u64, 1u64, 1u64))
                } else {
                    let a8556 = v0;
                    let a8557 = 3u64;
                    if nat_eq(a8556, a8557) {
                        Ok(f332(0u64, 1u64, 0u64, 1u64, 1u64, 1u64, 0u64))
                    } else {
                        Ok(Adt17::C0)
                    }
                }
            } else {
                let a8558 = v0;
                let a8559 = 2u64;
                if nat_eq(a8558, a8559) {
                    Ok(f332(4u64, 1u64, 0u64, 1u64, 1u64, 1u64, 0u64))
                } else {
                    Ok(Adt17::C0)
                }
            }
        }
    }
}

pub fn f348(v0: u64) -> R<u64> {
    let a8560 = 4u64;
    let a8561 = f324(v0);
    nat_mul(a8560, a8561)
}

pub fn f349(v0: u64) -> u64 {
    let a8562 = v0;
    let a8563 = 5u64;
    if nat_lt(a8562, a8563) {
        let a8564 = v0;
        let a8565 = 2u64;
        if nat_lt(a8564, a8565) {
            0u64
        } else {
            1u64
        }
    } else {
        let a8566 = v0;
        let a8567 = 7u64;
        if nat_lt(a8566, a8567) {
            2u64
        } else {
            3u64
        }
    }
}

pub fn run(p0: Adt0) -> R<Adt9> {
    f0(p0)
}
