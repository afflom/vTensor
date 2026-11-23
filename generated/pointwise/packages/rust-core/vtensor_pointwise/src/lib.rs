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
    C0,
    C1(Adt17),
}

#[derive(Clone)]
pub enum Adt17 {
    C0(u64, u64, u64),
}

#[derive(Clone)]
pub enum Adt18 {
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
                        let a6 = 53u64;
                        if if nat_eq(a5, a6) {
                            true
                        } else {
                            let a7 = v1;
                            let a8 = 54u64;
                            nat_eq(a7, a8)
                        } {
                            Ok(Adt9::C0(f2({
                                let a9 = v1;
                                let a10 = 54u64;
                                nat_eq(a9, a10)
                            }, v2, v7, v4, v6)?))
                        } else {
                            let a11 = v1;
                            let a12 = 78u64;
                            if nat_eq(a11, a12) {
                                Ok(Adt9::C0(f3(v2, v7, v4, v5, v6)?))
                            } else {
                                let a13 = v1;
                                let a14 = 23u64;
                                if if if nat_eq(a13, a14) {
                                    true
                                } else {
                                    let a15 = v1;
                                    let a16 = 26u64;
                                    nat_eq(a15, a16)
                                } {
                                    true
                                } else {
                                    let a17 = v1;
                                    let a18 = 25u64;
                                    nat_eq(a17, a18)
                                } {
                                    Ok(Adt9::C0(f4(v1, v2, v7, v4, v6)?))
                                } else {
                                    let a19 = v1;
                                    let a20 = 24u64;
                                    if nat_eq(a19, a20) {
                                        Ok(Adt9::C0(f5(v2, v7, v6)?))
                                    } else {
                                        let a21 = v1;
                                        let a22 = 27u64;
                                        if nat_eq(a21, a22) {
                                            Ok(Adt9::C0(f6(v2, v7, v6)?))
                                        } else {
                                            let a23 = v1;
                                            let a24 = 3u64;
                                            if nat_eq(a23, a24) {
                                                Ok(Adt9::C0(f7(v7, v4, v5, v6)?))
                                            } else {
                                                f8(v1, v2, v7, v4, v6)
                                            }
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
            f10(v13, v14, v15, v16, v17)
        }
        Adt0::C2(v18) => {
            f11(v18)
        }
        Adt0::C3(v19, v20, v21, v22, v23, v24, v25, v26, v27, v28, v29, v30, v31) => {
            Ok(f12(v19, v20, v21, v22, v23, v24, v25, v26, v27, v28, v29, v30, v31))
        }
        Adt0::C4(v8, v9, v10, v11) => {
            f9(v8, v9, v10, v11)
        }
        Adt0::C5(v12) => {
            Ok(Adt9::C9({
                let a25 = 3240099840u64;
                let a26 = v12;
                if if nat_le(a25, a26) {
                    let a27 = v12;
                    let a28 = 4286578688u64;
                    nat_le(a27, a28)
                } else {
                    false
                } {
                    Some(0u64)
                } else {
                    let a29 = 1092616192u64;
                    let a30 = v12;
                    if if nat_le(a29, a30) {
                        let a31 = v12;
                        let a32 = 2139095040u64;
                        nat_le(a31, a32)
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
    }
}

pub fn f1(v0: Adt1, v1: Adt2) -> R<Adt10> {
    let v2: u64 = f13(v1.clone(), 1u64)?;
    let a34 = match v1.clone() {
        Adt2::C0(h33, _, _, _, _, _, _, _, _, _, _) => {
            h33
        }
    };
    let a35 = 0u64;
    if if if if if if if if nat_eq(a34, a35) {
        let a37 = match v1.clone() {
            Adt2::C0(_, _, _, _, _, h36, _, _, _, _, _) => {
                h36
            }
        };
        let a38 = 4u64;
        nat_eq(a37, a38)
    } else {
        false
    } {
        f14(match v0.clone() {
            Adt1::C0(h39, _, _, _, _, _, _, _, _, _, _, _) => {
                h39
            }
        })
    } else {
        false
    } {
        f14(match v0.clone() {
            Adt1::C0(_, h40, _, _, _, _, _, _, _, _, _, _) => {
                h40
            }
        })
    } else {
        false
    } {
        let a42 = 0u64;
        let a43 = match v0.clone() {
            Adt1::C0(_, _, h41, _, _, _, _, _, _, _, _, _) => {
                h41
            }
        };
        nat_lt(a42, a43)
    } else {
        false
    } {
        let a45 = match v0.clone() {
            Adt1::C0(_, _, h44, _, _, _, _, _, _, _, _, _) => {
                h44
            }
        };
        let a46 = 2139095040u64;
        nat_lt(a45, a46)
    } else {
        false
    } {
        let a47 = v2;
        let a48 = 9223372036854775808u64;
        nat_lt(a47, a48)
    } else {
        false
    } {
        let a50 = f15(v2)?;
        let a51 = match v1.clone() {
            Adt2::C0(_, _, _, _, _, _, _, _, _, h49, _) => {
                h49
            }
        };
        nat_le(a50, a51)
    } else {
        false
    } {
        Ok(Adt10::C4(f16(v2)?, 4u64, f17(v1.clone(), 1u64), f17(v1, 1u64), Adt4::C23(match v0.clone() {
            Adt1::C0(h52, _, _, _, _, _, _, _, _, _, _, _) => {
                h52
            }
        }, match v0.clone() {
            Adt1::C0(_, h53, _, _, _, _, _, _, _, _, _, _) => {
                h53
            }
        }, match v0 {
            Adt1::C0(_, _, h54, _, _, _, _, _, _, _, _, _) => {
                h54
            }
        }, v2)))
    } else {
        Ok(Adt10::C1)
    }
}

pub fn f2(v0: bool, v1: Adt1, v2: Adt2, v3: Option<Adt2>, v4: Adt2) -> R<Adt10> {
    let m55 = v3;
    match m55 {
        None => {
            Ok(Adt10::C1)
        }
        Some(v5) => {
            let v6: Adt6 = Adt6::C0(match v2.clone() {
                Adt2::C0(_, h56, _, _, _, _, _, _, _, _, _) => {
                    h56
                }
            }, match v1.clone() {
                Adt1::C0(h57, _, _, _, _, _, _, _, _, _, _, _) => {
                    h57
                }
            }, if v0 {
                match v1.clone() {
                    Adt1::C0(_, _, _, h58, _, _, _, _, _, _, _, _) => {
                        h58
                    }
                }
            } else {
                match v1.clone() {
                    Adt1::C0(_, _, h59, _, _, _, _, _, _, _, _, _) => {
                        h59
                    }
                }
            }, if v0 {
                match v1.clone() {
                    Adt1::C0(_, _, _, _, _, _, h60, _, _, _, _, _) => {
                        h60
                    }
                }
            } else {
                match v1.clone() {
                    Adt1::C0(_, _, _, _, h61, _, _, _, _, _, _, _) => {
                        h61
                    }
                }
            });
            let v7: Adt6 = Adt6::C0(match v2.clone() {
                Adt2::C0(_, _, h62, _, _, _, _, _, _, _, _) => {
                    h62
                }
            }, match v1.clone() {
                Adt1::C0(_, h63, _, _, _, _, _, _, _, _, _, _) => {
                    h63
                }
            }, if v0 {
                match v1.clone() {
                    Adt1::C0(_, _, _, _, h64, _, _, _, _, _, _, _) => {
                        h64
                    }
                }
            } else {
                match v1.clone() {
                    Adt1::C0(_, _, _, h65, _, _, _, _, _, _, _, _) => {
                        h65
                    }
                }
            }, if v0 {
                match v1.clone() {
                    Adt1::C0(_, _, _, _, _, _, _, h66, _, _, _, _) => {
                        h66
                    }
                }
            } else {
                match v1.clone() {
                    Adt1::C0(_, _, _, _, _, h67, _, _, _, _, _, _) => {
                        h67
                    }
                }
            });
            let v8: Adt6 = if v0 {
                Adt6::C0(match v2.clone() {
                    Adt2::C0(_, _, _, h68, _, _, _, _, _, _, _) => {
                        h68
                    }
                }, match v1.clone() {
                    Adt1::C0(_, _, h69, _, _, _, _, _, _, _, _, _) => {
                        h69
                    }
                }, match v1.clone() {
                    Adt1::C0(_, _, _, _, _, h70, _, _, _, _, _, _) => {
                        h70
                    }
                }, match v1.clone() {
                    Adt1::C0(_, _, _, _, _, _, _, _, h71, _, _, _) => {
                        h71
                    }
                })
            } else {
                Adt6::C0(1u64, 1u64, 0u64, 1u64)
            };
            let v9: u64 = f18(v6.clone(), match v5.clone() {
                Adt2::C0(_, h72, _, _, _, _, _, _, _, _, _) => {
                    h72
                }
            })?;
            let v10: u64 = f18(v7.clone(), match v5.clone() {
                Adt2::C0(_, _, h73, _, _, _, _, _, _, _, _) => {
                    h73
                }
            })?;
            let v11: u64 = if v0 {
                f18(v8.clone(), match v5.clone() {
                    Adt2::C0(_, _, _, h74, _, _, _, _, _, _, _) => {
                        h74
                    }
                })?
            } else {
                1u64
            };
            let v12: u64 = if v0 {
                match v1.clone() {
                    Adt1::C0(_, _, _, _, _, _, _, _, _, h75, _, _) => {
                        h75
                    }
                }
            } else {
                match v5.clone() {
                    Adt2::C0(_, _, _, h76, _, _, _, _, _, _, _) => {
                        h76
                    }
                }
            };
            let v13: u64 = if v0 {
                match v1.clone() {
                    Adt1::C0(_, _, _, _, _, _, _, _, _, _, h77, _) => {
                        h77
                    }
                }
            } else {
                match v5.clone() {
                    Adt2::C0(_, _, _, _, h78, _, _, _, _, _, _) => {
                        h78
                    }
                }
            };
            let v14: u64 = if v0 {
                match v1 {
                    Adt1::C0(_, _, _, _, _, _, _, _, _, _, _, h79) => {
                        h79
                    }
                }
            } else {
                match v2.clone() {
                    Adt2::C0(_, _, _, _, h80, _, _, _, _, _, _) => {
                        h80
                    }
                }
            };
            let v15: u64 = {
                let a85 = {
                    let a81 = v9;
                    let a82 = v10;
                    nat_mul(a81, a82)?
                };
                let a86 = {
                    let a83 = v11;
                    let a84 = v13;
                    nat_mul(a83, a84)?
                };
                nat_mul(a85, a86)?
            };
            let v16: u64 = {
                let a94 = {
                    let a89 = match v6.clone() {
                        Adt6::C0(h87, _, _, _) => {
                            h87
                        }
                    };
                    let a90 = match v7.clone() {
                        Adt6::C0(h88, _, _, _) => {
                            h88
                        }
                    };
                    nat_mul(a89, a90)?
                };
                let a95 = {
                    let a92 = match v8.clone() {
                        Adt6::C0(h91, _, _, _) => {
                            h91
                        }
                    };
                    let a93 = v12;
                    nat_mul(a92, a93)?
                };
                nat_mul(a94, a95)?
            };
            let v17: u64 = {
                let a96 = v15;
                let a97 = 32u64;
                if nat_lt(a96, a97) {
                    v15
                } else {
                    32u64
                }
            };
            let a99 = match v2.clone() {
                Adt2::C0(h98, _, _, _, _, _, _, _, _, _, _) => {
                    h98
                }
            };
            let a100 = 0u64;
            if if if if if if if if if if if nat_eq(a99, a100) {
                true
            } else {
                let a102 = match v2.clone() {
                    Adt2::C0(h101, _, _, _, _, _, _, _, _, _, _) => {
                        h101
                    }
                };
                let a103 = 1u64;
                nat_eq(a102, a103)
            } {
                let a105 = match v5.clone() {
                    Adt2::C0(h104, _, _, _, _, _, _, _, _, _, _) => {
                        h104
                    }
                };
                let a106 = 0u64;
                nat_eq(a105, a106)
            } else {
                false
            } {
                let a108 = match v4.clone() {
                    Adt2::C0(h107, _, _, _, _, _, _, _, _, _, _) => {
                        h107
                    }
                };
                let a109 = 0u64;
                nat_eq(a108, a109)
            } else {
                false
            } {
                let a111 = match v4.clone() {
                    Adt2::C0(_, h110, _, _, _, _, _, _, _, _, _) => {
                        h110
                    }
                };
                let a112 = v9;
                nat_eq(a111, a112)
            } else {
                false
            } {
                let a114 = match v4.clone() {
                    Adt2::C0(_, _, h113, _, _, _, _, _, _, _, _) => {
                        h113
                    }
                };
                let a115 = v10;
                nat_eq(a114, a115)
            } else {
                false
            } {
                if v0 {
                    let a116 = 0u64;
                    let a117 = v12;
                    if if if if if if if nat_lt(a116, a117) {
                        let a118 = v12;
                        let a119 = 2147483648u64;
                        nat_lt(a118, a119)
                    } else {
                        false
                    } {
                        let a120 = v14;
                        let a121 = 2147483648u64;
                        nat_lt(a120, a121)
                    } else {
                        false
                    } {
                        let a122 = v13;
                        let a123 = 2147483648u64;
                        nat_lt(a122, a123)
                    } else {
                        false
                    } {
                        let a127 = match v2.clone() {
                            Adt2::C0(_, _, _, _, h124, _, _, _, _, _, _) => {
                                h124
                            }
                        };
                        let a128 = {
                            let a125 = v12;
                            let a126 = v14;
                            nat_mul(a125, a126)?
                        };
                        nat_eq(a127, a128)
                    } else {
                        false
                    } {
                        let a132 = match v5.clone() {
                            Adt2::C0(_, _, _, _, h129, _, _, _, _, _, _) => {
                                h129
                            }
                        };
                        let a133 = {
                            let a130 = v12;
                            let a131 = v13;
                            nat_mul(a130, a131)?
                        };
                        nat_eq(a132, a133)
                    } else {
                        false
                    } {
                        let a135 = match v4.clone() {
                            Adt2::C0(_, _, _, h134, _, _, _, _, _, _, _) => {
                                h134
                            }
                        };
                        let a136 = v11;
                        nat_eq(a135, a136)
                    } else {
                        false
                    } {
                        let a140 = match v4.clone() {
                            Adt2::C0(_, _, _, _, h137, _, _, _, _, _, _) => {
                                h137
                            }
                        };
                        let a141 = {
                            let a138 = v14;
                            let a139 = v13;
                            nat_mul(a138, a139)?
                        };
                        nat_eq(a140, a141)
                    } else {
                        false
                    }
                } else {
                    let a143 = match v2.clone() {
                        Adt2::C0(_, _, _, h142, _, _, _, _, _, _, _) => {
                            h142
                        }
                    };
                    let a144 = v12;
                    if if if nat_eq(a143, a144) {
                        let a146 = match v4.clone() {
                            Adt2::C0(_, _, _, h145, _, _, _, _, _, _, _) => {
                                h145
                            }
                        };
                        let a147 = v14;
                        nat_eq(a146, a147)
                    } else {
                        false
                    } {
                        let a149 = match v4.clone() {
                            Adt2::C0(_, _, _, _, h148, _, _, _, _, _, _) => {
                                h148
                            }
                        };
                        let a150 = v13;
                        nat_eq(a149, a150)
                    } else {
                        false
                    } {
                        let a151 = 0u64;
                        let a152 = v16;
                        nat_lt(a151, a152)
                    } else {
                        false
                    }
                }
            } else {
                false
            } {
                let a153 = 0u64;
                let a154 = v14;
                nat_lt(a153, a154)
            } else {
                false
            } {
                let a155 = 0u64;
                let a156 = v15;
                nat_lt(a155, a156)
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
                    let a157 = v16;
                    let a158 = v14;
                    nat_mul(a157, a158)?
                }, v17, {
                    let a159 = v17;
                    let a160 = v16;
                    nat_mul(a159, a160)?
                }, {
                    let a161 = v17;
                    let a162 = v14;
                    nat_mul(a161, a162)?
                }, match v2.clone() {
                    Adt2::C0(h163, _, _, _, _, _, _, _, _, _, _) => {
                        h163
                    }
                }, f17(v2, 1u64), f17(v5, 1u64), f17(v4, 1u64), v6, v7, v8, if v0 {
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

pub fn f3(v0: Adt1, v1: Adt2, v2: Option<Adt2>, v3: Option<Adt2>, v4: Adt2) -> R<Adt10> {
    let m164 = v2;
    match m164 {
        None => {
            Ok(Adt10::C1)
        }
        Some(v5) => {
            let m165 = v3;
            match m165 {
                None => {
                    Ok(Adt10::C1)
                }
                Some(v6) => {
                    let a167 = match v1.clone() {
                        Adt2::C0(h166, _, _, _, _, _, _, _, _, _, _) => {
                            h166
                        }
                    };
                    let a168 = 0u64;
                    if if if if if if if if if if if if if if if nat_eq(a167, a168) {
                        let a170 = match v5.clone() {
                            Adt2::C0(h169, _, _, _, _, _, _, _, _, _, _) => {
                                h169
                            }
                        };
                        let a171 = 0u64;
                        nat_eq(a170, a171)
                    } else {
                        false
                    } {
                        let a173 = match v6.clone() {
                            Adt2::C0(h172, _, _, _, _, _, _, _, _, _, _) => {
                                h172
                            }
                        };
                        let a174 = 0u64;
                        nat_eq(a173, a174)
                    } else {
                        false
                    } {
                        let a176 = match v4.clone() {
                            Adt2::C0(h175, _, _, _, _, _, _, _, _, _, _) => {
                                h175
                            }
                        };
                        let a177 = 0u64;
                        nat_eq(a176, a177)
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
                        f21(v1.clone())?
                    } else {
                        false
                    } {
                        f21(v5.clone())?
                    } else {
                        false
                    } {
                        f21(v6)?
                    } else {
                        false
                    } {
                        f21(v4.clone())?
                    } else {
                        false
                    } {
                        let a183 = match v1.clone() {
                            Adt2::C0(_, h178, _, _, _, _, _, _, _, _, _) => {
                                h178
                            }
                        };
                        let a184 = {
                            let a181 = match v5.clone() {
                                Adt2::C0(_, h179, _, _, _, _, _, _, _, _, _) => {
                                    h179
                                }
                            };
                            let a182 = match v5.clone() {
                                Adt2::C0(_, h180, _, _, _, _, _, _, _, _, _) => {
                                    h180
                                }
                            };
                            nat_mul(a181, a182)?
                        };
                        nat_eq(a183, a184)
                    } else {
                        false
                    } {
                        let a190 = match v1.clone() {
                            Adt2::C0(_, _, h185, _, _, _, _, _, _, _, _) => {
                                h185
                            }
                        };
                        let a191 = {
                            let a188 = match v5.clone() {
                                Adt2::C0(_, _, h186, _, _, _, _, _, _, _, _) => {
                                    h186
                                }
                            };
                            let a189 = match v5.clone() {
                                Adt2::C0(_, _, _, h187, _, _, _, _, _, _, _) => {
                                    h187
                                }
                            };
                            nat_mul(a188, a189)?
                        };
                        nat_eq(a190, a191)
                    } else {
                        false
                    } {
                        let a194 = match v1.clone() {
                            Adt2::C0(_, _, _, h192, _, _, _, _, _, _, _) => {
                                h192
                            }
                        };
                        let a195 = match v5.clone() {
                            Adt2::C0(_, _, _, _, h193, _, _, _, _, _, _) => {
                                h193
                            }
                        };
                        nat_eq(a194, a195)
                    } else {
                        false
                    } {
                        let a197 = match v0.clone() {
                            Adt1::C0(h196, _, _, _, _, _, _, _, _, _, _, _) => {
                                h196
                            }
                        };
                        let a198 = 4294967296u64;
                        nat_lt(a197, a198)
                    } else {
                        false
                    } {
                        let a200 = match v0.clone() {
                            Adt1::C0(h199, _, _, _, _, _, _, _, _, _, _, _) => {
                                h199
                            }
                        };
                        let a201 = 0u64;
                        if nat_eq(a200, a201) {
                            if f19(v1.clone(), v4.clone())? {
                                true
                            } else {
                                let a204 = match v1.clone() {
                                    Adt2::C0(_, _, _, _, _, _, _, _, _, _, h202) => {
                                        h202
                                    }
                                };
                                let a205 = match v4.clone() {
                                    Adt2::C0(_, _, _, _, _, _, _, _, _, _, h203) => {
                                        h203
                                    }
                                };
                                nat_eq(a204, a205)
                            }
                        } else {
                            let a208 = match v1.clone() {
                                Adt2::C0(_, _, _, _, _, _, _, _, _, _, h206) => {
                                    h206
                                }
                            };
                            let a209 = match v4.clone() {
                                Adt2::C0(_, _, _, _, _, _, _, _, _, _, h207) => {
                                    h207
                                }
                            };
                            nat_eq(a208, a209)
                        }
                    } else {
                        false
                    } {
                        let v7: bool = {
                            let a211 = match v4.clone() {
                                Adt2::C0(_, h210, _, _, _, _, _, _, _, _, _) => {
                                    h210
                                }
                            };
                            let a212 = 0u64;
                            if if if nat_eq(a211, a212) {
                                true
                            } else {
                                let a214 = match v4.clone() {
                                    Adt2::C0(_, _, h213, _, _, _, _, _, _, _, _) => {
                                        h213
                                    }
                                };
                                let a215 = 0u64;
                                nat_eq(a214, a215)
                            } {
                                true
                            } else {
                                let a217 = match v4.clone() {
                                    Adt2::C0(_, _, _, h216, _, _, _, _, _, _, _) => {
                                        h216
                                    }
                                };
                                let a218 = 0u64;
                                nat_eq(a217, a218)
                            } {
                                true
                            } else {
                                let a220 = match v4.clone() {
                                    Adt2::C0(_, _, _, _, h219, _, _, _, _, _, _) => {
                                        h219
                                    }
                                };
                                let a221 = 0u64;
                                nat_eq(a220, a221)
                            }
                        };
                        let v8: u64 = match v5.clone() {
                            Adt2::C0(_, h222, _, _, _, _, _, _, _, _, _) => {
                                h222
                            }
                        };
                        let v9: u64 = if v7 {
                            0u64
                        } else {
                            let a224 = match v0 {
                                Adt1::C0(h223, _, _, _, _, _, _, _, _, _, _, _) => {
                                    h223
                                }
                            };
                            let a225 = 0u64;
                            if nat_eq(a224, a225) {
                                f13(v1.clone(), 1u64)?
                            } else {
                                0u64
                            }
                        };
                        let v10: u64 = if v7 {
                            0u64
                        } else {
                            f22(v8, match v5.clone() {
                                Adt2::C0(_, _, h226, _, _, _, _, _, _, _, _) => {
                                    h226
                                }
                            }, match v5.clone() {
                                Adt2::C0(_, _, _, h227, _, _, _, _, _, _, _) => {
                                    h227
                                }
                            }, match v5 {
                                Adt2::C0(_, _, _, _, h228, _, _, _, _, _, _) => {
                                    h228
                                }
                            })?
                        };
                        Ok(Adt10::C4(f23(v9, v8, v10)?, 4u64, f17(v1, 1u64), f17(v4, 1u64), Adt4::C25(v9, v8, v10)))
                    } else {
                        Ok(Adt10::C1)
                    }
                }
            }
        }
    }
}

pub fn f4(v0: u64, v1: Adt1, v2: Adt2, v3: Option<Adt2>, v4: Adt2) -> R<Adt10> {
    let v5: Adt14 = f24(v2.clone());
    let v6: Adt14 = f24(v4.clone());
    let v8: Adt15 = {
        let m229 = v3;
        match m229 {
            None => {
                f25(v0, match v1.clone() {
                    Adt1::C0(h230, _, _, _, _, _, _, _, _, _, _, _) => {
                        h230
                    }
                }, match v1 {
                    Adt1::C0(_, h231, _, _, _, _, _, _, _, _, _, _) => {
                        h231
                    }
                }, false, v5.clone(), v5, v6)?
            }
            Some(v7) => {
                f25(v0, match v1.clone() {
                    Adt1::C0(h232, _, _, _, _, _, _, _, _, _, _, _) => {
                        h232
                    }
                }, match v1 {
                    Adt1::C0(_, h233, _, _, _, _, _, _, _, _, _, _) => {
                        h233
                    }
                }, true, v5, f24(v7), v6)?
            }
        }
    };
    let m234 = v8;
    match m234 {
        Adt15::C0 => {
            Ok(Adt10::C1)
        }
        Adt15::C1(v9, v10, v11, v12, v13, v14) => {
            Ok(Adt10::C4(v9, v10, f17(v2, 1u64), f17(v4, 1u64), Adt4::C26(v11, v14, v12, v13)))
        }
    }
}

pub fn f5(v0: Adt1, v1: Adt2, v2: Adt2) -> R<Adt10> {
    let v3: u64 = match v1.clone() {
        Adt2::C0(h235, _, _, _, _, _, _, _, _, _, _) => {
            h235
        }
    };
    let v4: u64 = f26(v3);
    let a237 = match v1.clone() {
        Adt2::C0(h236, _, _, _, _, _, _, _, _, _, _) => {
            h236
        }
    };
    let a238 = 0u64;
    if if if if if if if if nat_eq(a237, a238) {
        true
    } else {
        let a240 = match v1.clone() {
            Adt2::C0(h239, _, _, _, _, _, _, _, _, _, _) => {
                h239
            }
        };
        let a241 = 30u64;
        nat_eq(a240, a241)
    } {
        let a244 = match v1.clone() {
            Adt2::C0(h242, _, _, _, _, _, _, _, _, _, _) => {
                h242
            }
        };
        let a245 = match v2.clone() {
            Adt2::C0(h243, _, _, _, _, _, _, _, _, _, _) => {
                h243
            }
        };
        nat_eq(a244, a245)
    } else {
        false
    } {
        f20(v1.clone(), v2.clone())
    } else {
        false
    } {
        f27(match v0.clone() {
            Adt1::C0(h246, _, _, _, _, _, _, _, _, _, _, _) => {
                h246
            }
        })
    } else {
        false
    } {
        f28(v1.clone(), v4)
    } else {
        false
    } {
        f28(v2.clone(), v4)
    } else {
        false
    } {
        if f19(v1.clone(), v2.clone())? {
            true
        } else {
            f29(v1.clone(), v2.clone())
        }
    } else {
        false
    } {
        let v5: u64 = if f30(match v1.clone() {
            Adt2::C0(_, h247, _, _, _, _, _, _, _, _, _) => {
                h247
            }
        }, match v1.clone() {
            Adt2::C0(_, _, h248, _, _, _, _, _, _, _, _) => {
                h248
            }
        }, match v1.clone() {
            Adt2::C0(_, _, _, h249, _, _, _, _, _, _, _) => {
                h249
            }
        }, match v1.clone() {
            Adt2::C0(_, _, _, _, h250, _, _, _, _, _, _) => {
                h250
            }
        }) {
            0u64
        } else {
            let a256 = match v1.clone() {
                Adt2::C0(_, _, h251, _, _, _, _, _, _, _, _) => {
                    h251
                }
            };
            let a257 = {
                let a254 = match v1.clone() {
                    Adt2::C0(_, _, _, h252, _, _, _, _, _, _, _) => {
                        h252
                    }
                };
                let a255 = match v1.clone() {
                    Adt2::C0(_, _, _, _, h253, _, _, _, _, _, _) => {
                        h253
                    }
                };
                nat_mul(a254, a255)?
            };
            nat_mul(a256, a257)?
        };
        let v6: u64 = {
            let a258 = v5;
            let a259 = 0u64;
            if if nat_eq(a258, a259) {
                true
            } else {
                f31(v2.clone(), v4)?
            } {
                0u64
            } else {
                let a261 = match v2.clone() {
                    Adt2::C0(_, _, _, _, _, _, _, _, _, h260, _) => {
                        h260
                    }
                };
                let a262 = v4;
                let a263 = 0u64;
                nat_quot(a261, a262, a263)
            }
        };
        Ok(Adt10::C4(f32(v3, match v1.clone() {
            Adt2::C0(_, h264, _, _, _, _, _, _, _, _, _) => {
                h264
            }
        }, v5, v6)?, v4, f17(v1.clone(), 1u64), f17(v2, 1u64), Adt4::C26(v3, match v0 {
            Adt1::C0(h265, _, _, _, _, _, _, _, _, _, _, _) => {
                h265
            }
        }, v5, f33(match v1 {
            Adt2::C0(_, h266, _, _, _, _, _, _, _, _, _) => {
                h266
            }
        }, v5, v6)?)))
    } else {
        Ok(Adt10::C1)
    }
}

pub fn f6(v0: Adt1, v1: Adt2, v2: Adt2) -> R<Adt10> {
    let a268 = match v1.clone() {
        Adt2::C0(h267, _, _, _, _, _, _, _, _, _, _) => {
            h267
        }
    };
    let a269 = 0u64;
    if if if if if nat_eq(a268, a269) {
        let a271 = match v2.clone() {
            Adt2::C0(h270, _, _, _, _, _, _, _, _, _, _) => {
                h270
            }
        };
        let a272 = 0u64;
        nat_eq(a271, a272)
    } else {
        false
    } {
        f20(v1.clone(), v2.clone())
    } else {
        false
    } {
        f34(match v0.clone() {
            Adt1::C0(h273, _, _, _, _, _, _, _, _, _, _, _) => {
                h273
            }
        })
    } else {
        false
    } {
        if f19(v1.clone(), v2.clone())? {
            true
        } else {
            f29(v1.clone(), v2.clone())
        }
    } else {
        false
    } {
        let v3: Adt3 = f17(v1, 1u64);
        let v4: u64 = {
            let a279 = match v3.clone() {
                Adt3::C0(_, h274, _, _, _, _, _, _) => {
                    h274
                }
            };
            let a280 = {
                let a277 = match v3.clone() {
                    Adt3::C0(_, _, h275, _, _, _, _, _) => {
                        h275
                    }
                };
                let a278 = match v3.clone() {
                    Adt3::C0(_, _, _, h276, _, _, _, _) => {
                        h276
                    }
                };
                nat_mul(a277, a278)?
            };
            nat_mul(a279, a280)?
        };
        Ok(Adt10::C4(f35(match v3.clone() {
            Adt3::C0(h281, _, _, _, _, _, _, _) => {
                h281
            }
        }, v4)?, 4u64, v3, f17(v2, 1u64), Adt4::C24(match v0 {
            Adt1::C0(h282, _, _, _, _, _, _, _, _, _, _, _) => {
                h282
            }
        })))
    } else {
        Ok(Adt10::C1)
    }
}

pub fn f7(v0: Adt2, v1: Option<Adt2>, v2: Option<Adt2>, v3: Adt2) -> R<Adt10> {
    let m283 = v1;
    match m283 {
        None => {
            Ok(Adt10::C1)
        }
        Some(v4) => {
            let m284 = v2;
            match m284 {
                None => {
                    Ok(Adt10::C1)
                }
                Some(v5) => {
                    let a286 = match v0.clone() {
                        Adt2::C0(h285, _, _, _, _, _, _, _, _, _, _) => {
                            h285
                        }
                    };
                    let a287 = 0u64;
                    if if if if if if if if if if if if if if if if if nat_eq(a286, a287) {
                        let a289 = match v4.clone() {
                            Adt2::C0(h288, _, _, _, _, _, _, _, _, _, _) => {
                                h288
                            }
                        };
                        let a290 = 0u64;
                        nat_eq(a289, a290)
                    } else {
                        false
                    } {
                        let a292 = match v5.clone() {
                            Adt2::C0(h291, _, _, _, _, _, _, _, _, _, _) => {
                                h291
                            }
                        };
                        let a293 = 26u64;
                        nat_eq(a292, a293)
                    } else {
                        false
                    } {
                        let a295 = match v3.clone() {
                            Adt2::C0(h294, _, _, _, _, _, _, _, _, _, _) => {
                                h294
                            }
                        };
                        let a296 = 0u64;
                        nat_eq(a295, a296)
                    } else {
                        false
                    } {
                        f20(v0.clone(), v3.clone())
                    } else {
                        false
                    } {
                        let a299 = match v0.clone() {
                            Adt2::C0(_, h297, _, _, _, _, _, _, _, _, _) => {
                                h297
                            }
                        };
                        let a300 = match v4.clone() {
                            Adt2::C0(_, h298, _, _, _, _, _, _, _, _, _) => {
                                h298
                            }
                        };
                        nat_eq(a299, a300)
                    } else {
                        false
                    } {
                        let a303 = match v0.clone() {
                            Adt2::C0(_, _, h301, _, _, _, _, _, _, _, _) => {
                                h301
                            }
                        };
                        let a304 = match v5.clone() {
                            Adt2::C0(_, h302, _, _, _, _, _, _, _, _, _) => {
                                h302
                            }
                        };
                        nat_eq(a303, a304)
                    } else {
                        false
                    } {
                        let a307 = match v0.clone() {
                            Adt2::C0(_, _, _, h305, _, _, _, _, _, _, _) => {
                                h305
                            }
                        };
                        let a308 = match v5.clone() {
                            Adt2::C0(_, _, h306, _, _, _, _, _, _, _, _) => {
                                h306
                            }
                        };
                        nat_eq(a307, a308)
                    } else {
                        false
                    } {
                        let a310 = 0u64;
                        let a311 = match v5.clone() {
                            Adt2::C0(_, _, _, h309, _, _, _, _, _, _, _) => {
                                h309
                            }
                        };
                        nat_lt(a310, a311)
                    } else {
                        false
                    } {
                        let a313 = 0u64;
                        let a314 = match v5.clone() {
                            Adt2::C0(_, _, _, _, h312, _, _, _, _, _, _) => {
                                h312
                            }
                        };
                        nat_lt(a313, a314)
                    } else {
                        false
                    } {
                        let a316 = 4u64;
                        let a317 = match v0.clone() {
                            Adt2::C0(_, _, _, _, _, h315, _, _, _, _, _) => {
                                h315
                            }
                        };
                        nat_le(a316, a317)
                    } else {
                        false
                    } {
                        let a319 = 4u64;
                        let a320 = match v4.clone() {
                            Adt2::C0(_, _, _, _, _, h318, _, _, _, _, _) => {
                                h318
                            }
                        };
                        nat_le(a319, a320)
                    } else {
                        false
                    } {
                        let a322 = 4u64;
                        let a323 = match v5.clone() {
                            Adt2::C0(_, _, _, _, _, h321, _, _, _, _, _) => {
                                h321
                            }
                        };
                        nat_le(a322, a323)
                    } else {
                        false
                    } {
                        let a325 = 4u64;
                        let a326 = match v3.clone() {
                            Adt2::C0(_, _, _, _, _, h324, _, _, _, _, _) => {
                                h324
                            }
                        };
                        nat_le(a325, a326)
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
                        Ok(Adt10::C4(f36(match v0.clone() {
                            Adt2::C0(_, h327, _, _, _, _, _, _, _, _, _) => {
                                h327
                            }
                        }, match v0.clone() {
                            Adt2::C0(_, _, h328, _, _, _, _, _, _, _, _) => {
                                h328
                            }
                        }, match v0.clone() {
                            Adt2::C0(_, _, _, h329, _, _, _, _, _, _, _) => {
                                h329
                            }
                        }, match v0.clone() {
                            Adt2::C0(_, _, _, _, h330, _, _, _, _, _, _) => {
                                h330
                            }
                        })?, 4u64, f17(v0, 1u64), f17(v3, 1u64), Adt4::C22(f17(v4, 1u64), f17(v5, 1u64))))
                    } else {
                        Ok(Adt10::C1)
                    }
                }
            }
        }
    }
}

pub fn f8(v0: u64, v1: Adt1, v2: Adt2, v3: Option<Adt2>, v4: Adt2) -> R<Adt9> {
    let v5: u64 = match v1.clone() {
        Adt1::C0(h331, _, _, _, _, _, _, _, _, _, _, _) => {
            h331
        }
    };
    Ok(Adt9::C0({
        let a332 = v0;
        let a333 = 52u64;
        if nat_eq(a332, a333) {
            f37(v1, v2, v4)?
        } else {
            let a334 = v0;
            let a335 = 73u64;
            if nat_eq(a334, a335) {
                f38(v2, v3, v4)?
            } else {
                let a336 = v0;
                let a337 = 57u64;
                if if nat_eq(a336, a337) {
                    true
                } else {
                    let a338 = v0;
                    let a339 = 58u64;
                    nat_eq(a338, a339)
                } {
                    let m340 = v3;
                    match m340 {
                        None => {
                            f39({
                                let a341 = v0;
                                let a342 = 58u64;
                                nat_eq(a341, a342)
                            }, v1, v2, v4)?
                        }
                        Some(_) => {
                            Adt10::C1
                        }
                    }
                } else {
                    let a343 = v0;
                    let a344 = 49u64;
                    if nat_eq(a343, a344) {
                        f40(v1, v2, v3, v4)?
                    } else {
                        let a345 = v0;
                        let a346 = 51u64;
                        if nat_eq(a345, a346) {
                            f41(v1, v2, v3, v4)?
                        } else {
                            let a347 = v0;
                            let a348 = 39u64;
                            if nat_eq(a347, a348) {
                                f42(v2, v3, v4)?
                            } else {
                                let a349 = v0;
                                let a350 = 77u64;
                                if nat_eq(a349, a350) {
                                    f43(v2, v4)?
                                } else {
                                    let a351 = v0;
                                    let a352 = 40u64;
                                    if nat_eq(a351, a352) {
                                        f44(v2, v3, v4)?
                                    } else {
                                        if f45(v0) {
                                            f46(v0, v2, {
                                                let m353 = v3;
                                                match m353 {
                                                    None => {
                                                        false
                                                    }
                                                    Some(_) => {
                                                        true
                                                    }
                                                }
                                            }, v4)?
                                        } else {
                                            let a354 = v0;
                                            let a355 = 19u64;
                                            if nat_eq(a354, a355) {
                                                f47(v2, v4)?
                                            } else {
                                                let a356 = v0;
                                                let a357 = 21u64;
                                                if nat_eq(a356, a357) {
                                                    f48(v5, v2, v3, v4)?
                                                } else {
                                                    let a358 = v0;
                                                    let a359 = 61u64;
                                                    if if if nat_eq(a358, a359) {
                                                        true
                                                    } else {
                                                        let a360 = v0;
                                                        let a361 = 62u64;
                                                        nat_eq(a360, a361)
                                                    } {
                                                        true
                                                    } else {
                                                        let a362 = v0;
                                                        let a363 = 63u64;
                                                        nat_eq(a362, a363)
                                                    } {
                                                        f49(v0, v1, v2, v4)?
                                                    } else {
                                                        let a364 = v0;
                                                        let a365 = 4u64;
                                                        if if nat_eq(a364, a365) {
                                                            true
                                                        } else {
                                                            let a366 = v0;
                                                            let a367 = 32u64;
                                                            nat_eq(a366, a367)
                                                        } {
                                                            f50(v0, v1, v2, v3, v4)?
                                                        } else {
                                                            let a368 = v0;
                                                            let a369 = 20u64;
                                                            if nat_eq(a368, a369) {
                                                                f51(v2, v4)?
                                                            } else {
                                                                let a370 = v0;
                                                                let a371 = 42u64;
                                                                if nat_eq(a370, a371) {
                                                                    f52(4u64, v2, v4)?
                                                                } else {
                                                                    let a372 = v0;
                                                                    let a373 = 69u64;
                                                                    if nat_eq(a372, a373) {
                                                                        let a374 = v5;
                                                                        let a375 = 4u64;
                                                                        if nat_lt(a374, a375) {
                                                                            f52(v5, v2, v4)?
                                                                        } else {
                                                                            Adt10::C1
                                                                        }
                                                                    } else {
                                                                        let a376 = v0;
                                                                        let a377 = 70u64;
                                                                        if nat_eq(a376, a377) {
                                                                            f53(v5, v2, v4)?
                                                                        } else {
                                                                            let a378 = v0;
                                                                            let a379 = 75u64;
                                                                            if if nat_eq(a378, a379) {
                                                                                true
                                                                            } else {
                                                                                let a380 = v0;
                                                                                let a381 = 76u64;
                                                                                nat_eq(a380, a381)
                                                                            } {
                                                                                f54({
                                                                                    let a382 = v0;
                                                                                    let a383 = 76u64;
                                                                                    nat_eq(a382, a383)
                                                                                }, v1, v2, v4)?
                                                                            } else {
                                                                                let a384 = v0;
                                                                                let a385 = 31u64;
                                                                                if if if nat_eq(a384, a385) {
                                                                                    true
                                                                                } else {
                                                                                    let a386 = v0;
                                                                                    let a387 = 47u64;
                                                                                    nat_eq(a386, a387)
                                                                                } {
                                                                                    true
                                                                                } else {
                                                                                    let a388 = v0;
                                                                                    let a389 = 68u64;
                                                                                    nat_eq(a388, a389)
                                                                                } {
                                                                                    f55(v0, v1, v2, v4)?
                                                                                } else {
                                                                                    let a390 = v0;
                                                                                    let a391 = 15u64;
                                                                                    if nat_eq(a390, a391) {
                                                                                        f56(v2, v4)?
                                                                                    } else {
                                                                                        let a392 = v0;
                                                                                        let a393 = 17u64;
                                                                                        if nat_eq(a392, a393) {
                                                                                            f57(v2, v4)?
                                                                                        } else {
                                                                                            let a394 = v0;
                                                                                            let a395 = 18u64;
                                                                                            if nat_eq(a394, a395) {
                                                                                                f58(v2, v3, v4)?
                                                                                            } else {
                                                                                                let a396 = v0;
                                                                                                let a397 = 13u64;
                                                                                                if if if nat_eq(a396, a397) {
                                                                                                    true
                                                                                                } else {
                                                                                                    let a398 = v0;
                                                                                                    let a399 = 14u64;
                                                                                                    nat_eq(a398, a399)
                                                                                                } {
                                                                                                    true
                                                                                                } else {
                                                                                                    let a400 = v0;
                                                                                                    let a401 = 16u64;
                                                                                                    nat_eq(a400, a401)
                                                                                                } {
                                                                                                    f59(v0, v2, v4)?
                                                                                                } else {
                                                                                                    let m402 = f60(v0, v5)?;
                                                                                                    match m402 {
                                                                                                        None => {
                                                                                                            f61(v0, v5, v2, v3, v4)?
                                                                                                        }
                                                                                                        Some(v8) => {
                                                                                                            f62(v8, v2, v4)?
                                                                                                        }
                                                                                                    }
                                                                                                }
                                                                                            }
                                                                                        }
                                                                                    }
                                                                                }
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
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

pub fn f9(v0: Adt8, v1: u64, v2: u64, v3: u64) -> R<Adt9> {
    let a403 = v1;
    let a404 = 0u64;
    if nat_eq(a403, a404) {
        let a406 = v3;
        let a407 = match v0.clone() {
            Adt8::C0(_, _, _, h405, _, _, _, _, _, _, _, _, _, _, _) => {
                h405
            }
        };
        if nat_lt(a406, a407) {
            let v4: Adt3 = match v0.clone() {
                Adt8::C0(_, _, _, _, _, _, _, _, h408, _, _, _, _, _, _) => {
                    h408
                }
            };
            let v5: u64 = {
                let a410 = v3;
                let a411 = match v0.clone() {
                    Adt8::C0(_, _, h409, _, _, _, _, _, _, _, _, _, _, _, _) => {
                        h409
                    }
                };
                let a412 = 0u64;
                nat_quot(a410, a411, a412)
            };
            Ok(Adt9::C8(Some(f63(v4.clone(), f64(v5, v4.clone(), 0u64), f64(v5, v4.clone(), 1u64), f64(v5, v4, 2u64), f65(v0.clone(), v5, {
                let a414 = v3;
                let a415 = match v0 {
                    Adt8::C0(_, _, h413, _, _, _, _, _, _, _, _, _, _, _, _) => {
                        h413
                    }
                };
                let a416 = 0u64;
                nat_rem(a414, a415, a416)
            })?)?)))
        } else {
            Ok(Adt9::C0(Adt10::C1))
        }
    } else {
        let v6: Adt3 = match v0.clone() {
            Adt8::C0(_, _, _, _, _, _, _, _, _, _, h417, _, _, _, _) => {
                h417
            }
        };
        let v7: u64 = {
            let a418 = v1;
            let a419 = 1u64;
            if nat_eq(a418, a419) {
                match v0.clone() {
                    Adt8::C0(_, h420, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                        h420
                    }
                }
            } else {
                match v0.clone() {
                    Adt8::C0(_, _, h421, _, _, _, _, _, _, _, _, _, _, _, _) => {
                        h421
                    }
                }
            }
        };
        let v8: u64 = {
            let a425 = v2;
            let a426 = {
                let a422 = v3;
                let a423 = v7;
                let a424 = 0u64;
                nat_quot(a422, a423, a424)
            };
            nat_add(a425, a426)?
        };
        let v9: u64 = {
            let a427 = v3;
            let a428 = v7;
            let a429 = 0u64;
            nat_rem(a427, a428, a429)
        };
        let v10: u64 = {
            let a431 = v8;
            let a432 = match v6.clone() {
                Adt3::C0(h430, _, _, _, _, _, _, _) => {
                    h430
                }
            };
            let a433 = 0u64;
            nat_rem(a431, a432, a433)
        };
        let v11: u64 = {
            let a435 = v8;
            let a436 = match v6.clone() {
                Adt3::C0(h434, _, _, _, _, _, _, _) => {
                    h434
                }
            };
            let a437 = 0u64;
            nat_quot(a435, a436, a437)
        };
        let v12: u64 = {
            let a439 = v11;
            let a440 = match v6.clone() {
                Adt3::C0(_, h438, _, _, _, _, _, _) => {
                    h438
                }
            };
            let a441 = 0u64;
            nat_rem(a439, a440, a441)
        };
        let v13: u64 = {
            let a443 = v11;
            let a444 = match v6.clone() {
                Adt3::C0(_, h442, _, _, _, _, _, _) => {
                    h442
                }
            };
            let a445 = 0u64;
            nat_quot(a443, a444, a445)
        };
        let a447 = v8;
        let a448 = match v0.clone() {
            Adt8::C0(h446, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h446
            }
        };
        if nat_lt(a447, a448) {
            let a449 = v1;
            let a450 = 1u64;
            if nat_eq(a449, a450) {
                let v14: Adt3 = match v0.clone() {
                    Adt8::C0(_, _, _, _, _, _, _, _, h451, _, _, _, _, _, _) => {
                        h451
                    }
                };
                let v15: Adt3 = match v0.clone() {
                    Adt8::C0(_, _, _, _, _, _, _, _, _, h452, _, _, _, _, _) => {
                        h452
                    }
                };
                let m455 = f66(match v0.clone() {
                    Adt8::C0(_, _, _, _, _, _, _, _, _, _, _, h453, _, _, _) => {
                        h453
                    }
                }, match v15.clone() {
                    Adt3::C0(h454, _, _, _, _, _, _, _) => {
                        h454
                    }
                }, v10, f64(v9, v14.clone(), 0u64))?;
                match m455 {
                    None => {
                        Ok(Adt9::C8(None::<u64>))
                    }
                    Some(v16) => {
                        let m458 = f66(match v0.clone() {
                            Adt8::C0(_, _, _, _, _, _, _, _, _, _, _, _, h456, _, _) => {
                                h456
                            }
                        }, match v15 {
                            Adt3::C0(_, h457, _, _, _, _, _, _) => {
                                h457
                            }
                        }, v12, f64(v9, v14, 1u64))?;
                        match m458 {
                            None => {
                                Ok(Adt9::C8(None::<u64>))
                            }
                            Some(v17) => {
                                f67(v0, v9, v13, v16, v17)
                            }
                        }
                    }
                }
            } else {
                let a459 = v1;
                let a460 = 2u64;
                if nat_eq(a459, a460) {
                    Ok(Adt9::C8(Some({
                        let a462 = match v0.clone() {
                            Adt8::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, h461) => {
                                h461
                            }
                        };
                        let a463 = 0u64;
                        if nat_eq(a462, a463) {
                            f63(v6, v10, v12, v9, v13)?
                        } else {
                            f63(v6.clone(), v10, v12, {
                                let a465 = v13;
                                let a466 = match v6.clone() {
                                    Adt3::C0(_, _, h464, _, _, _, _, _) => {
                                        h464
                                    }
                                };
                                let a467 = 0u64;
                                nat_rem(a465, a466, a467)
                            }, {
                                let a475 = {
                                    let a473 = {
                                        let a469 = v13;
                                        let a470 = match v6 {
                                            Adt3::C0(_, _, h468, _, _, _, _, _) => {
                                                h468
                                            }
                                        };
                                        let a471 = 0u64;
                                        nat_quot(a469, a470, a471)
                                    };
                                    let a474 = match v0 {
                                        Adt8::C0(_, _, h472, _, _, _, _, _, _, _, _, _, _, _, _) => {
                                            h472
                                        }
                                    };
                                    nat_mul(a473, a474)?
                                };
                                let a476 = v9;
                                nat_add(a475, a476)?
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

pub fn f10(v0: u64, v1: Adt3, v2: Adt3, v3: Adt4, v4: u64) -> R<Adt9> {
    Ok(f68(v3.clone(), {
        let m477 = v3.clone();
        match m477 {
            Adt4::C0 => {
                f69(v0, v1, v2, v3)?
            }
            Adt4::C1 => {
                f69(v0, v1, v2, v3)?
            }
            Adt4::C2(_, _) => {
                f69(v0, v1, v2, v3)?
            }
            Adt4::C3(_, _, _, _, _) => {
                f69(v0, v1, v2, v3)?
            }
            Adt4::C4(_) => {
                f69(v0, v1, v2, v3)?
            }
            Adt4::C5(_, _, _, _) => {
                f69(v0, v1, v2, v3)?
            }
            Adt4::C6(_, _, _) => {
                f69(v0, v1, v2, v3)?
            }
            Adt4::C7(_) => {
                f69(v0, v1, v2, v3)?
            }
            Adt4::C8(_) => {
                f69(v0, v1, v2, v3)?
            }
            Adt4::C9(_, _) => {
                f69(v0, v1, v2, v3)?
            }
            Adt4::C10(_, _, _) => {
                f69(v0, v1, v2, v3)?
            }
            Adt4::C11(_, _) => {
                f69(v0, v1, v2, v3)?
            }
            Adt4::C12(_, _, _) => {
                f69(v0, v1, v2, v3)?
            }
            Adt4::C13(_, _, _, _) => {
                f69(v0, v1, v2, v3)?
            }
            Adt4::C14(_, _, _) => {
                f69(v0, v1, v2, v3)?
            }
            Adt4::C15 => {
                f69(v0, v1, v2, v3)?
            }
            Adt4::C16(_, _) => {
                f69(v0, v1, v2, v3)?
            }
            Adt4::C17 => {
                f70(v0, v1, v2, v4)?
            }
            Adt4::C18(_, _) => {
                f69(v0, v1, v2, v3)?
            }
            Adt4::C19(_, v44) => {
                f71(v0, v1, v2, v44, v4)?
            }
            Adt4::C20(v45, v46) => {
                f72(v45, v0, v1, v2, v46, v4)?
            }
            Adt4::C21 => {
                f69(v0, v1, v2, v3)?
            }
            Adt4::C22(v47, v48) => {
                f73(v0, v1, v2, v47, v48, v4)?
            }
            Adt4::C23(v62, v63, v64, v65) => {
                f79(v0, v65, v62, v63, v64, v4)?
            }
            Adt4::C24(_) => {
                Adt9::C0(Adt10::C5(65u64))
            }
            Adt4::C25(_, _, _) => {
                f69(v0, v1, v2, v3)?
            }
            Adt4::C26(v57, v58, v59, v60) => {
                let a478 = v57;
                let a479 = 64u64;
                if if if nat_eq(a478, a479) {
                    true
                } else {
                    let a480 = v57;
                    let a481 = 65u64;
                    nat_eq(a480, a481)
                } {
                    true
                } else {
                    let a482 = v57;
                    let a483 = 66u64;
                    nat_eq(a482, a483)
                } {
                    Adt9::C0(Adt10::C5(64u64))
                } else {
                    let a484 = v57;
                    let a485 = 0u64;
                    if if nat_eq(a484, a485) {
                        true
                    } else {
                        let a486 = v57;
                        let a487 = 30u64;
                        nat_eq(a486, a487)
                    } {
                        f78(v0, v1, v2, v57, v58, v59, v60, v4)?
                    } else {
                        Adt9::C0(Adt10::C1)
                    }
                }
            }
            Adt4::C27(v56) => {
                f77(v0, v1, v2, v56)?
            }
            Adt4::C28(v52) => {
                f74(v0, v1, v2, v52, v4)?
            }
            Adt4::C29(v53) => {
                f75(v0, v1, v2, v53)?
            }
            Adt4::C30(v54, _) => {
                f76(v0, v1, v2, v54)?
            }
        }
    }))
}

pub fn f11(v0: Adt4) -> R<Adt9> {
    let m488 = v0;
    match m488 {
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
        Adt4::C25(v45, _, v47) => {
            Ok(Adt9::C6({
                let a493 = {
                    let a489 = 0u64;
                    let a490 = v45;
                    if nat_lt(a489, a490) {
                        1u64
                    } else {
                        0u64
                    }
                };
                let a494 = {
                    let a491 = 0u64;
                    let a492 = v47;
                    if nat_lt(a491, a492) {
                        6u64
                    } else {
                        0u64
                    }
                };
                nat_add(a493, a494)?
            }, 0u64, 0u64, false))
        }
        Adt4::C26(v53, _, _, _) => {
            Ok(Adt9::C6({
                let a495 = v53;
                let a496 = 66u64;
                if nat_eq(a495, a496) {
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
    }
}

pub fn f12(v0: bool, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64, v7: u64, v8: u64, v9: u64, v10: u64, v11: u64, v12: u64) -> Adt9 {
    let a497 = v1;
    let a498 = 5u64;
    if if if if nat_lt(a497, a498) {
        let a499 = v5;
        let a500 = 5u64;
        nat_lt(a499, a500)
    } else {
        false
    } {
        let a501 = v7;
        let a502 = 5u64;
        nat_lt(a501, a502)
    } else {
        false
    } {
        let a503 = v9;
        let a504 = 2u64;
        if nat_lt(a503, a504) {
            true
        } else {
            let a505 = v9;
            let a506 = 3u64;
            nat_eq(a505, a506)
        }
    } else {
        false
    } {
        Adt9::C7(f80(v0, v1, v2, v3, v4), f81(v5, v6), f81(v7, v8), f82({
            let a507 = v9;
            let a508 = 0u64;
            if nat_eq(a507, a508) {
                1u64
            } else {
                v9
            }
        }, v10), v11, v12)
    } else {
        Adt9::C0(Adt10::C1)
    }
}

pub fn f13(v0: Adt2, v1: u64) -> R<u64> {
    let a520 = {
        let a514 = {
            let a510 = match v0.clone() {
                Adt2::C0(_, h509, _, _, _, _, _, _, _, _, _) => {
                    h509
                }
            };
            let a511 = v1;
            let a512 = 0u64;
            nat_quot(a510, a511, a512)
        };
        let a515 = match v0.clone() {
            Adt2::C0(_, _, h513, _, _, _, _, _, _, _, _) => {
                h513
            }
        };
        nat_mul(a514, a515)?
    };
    let a521 = {
        let a518 = match v0.clone() {
            Adt2::C0(_, _, _, h516, _, _, _, _, _, _, _) => {
                h516
            }
        };
        let a519 = match v0 {
            Adt2::C0(_, _, _, _, h517, _, _, _, _, _, _) => {
                h517
            }
        };
        nat_mul(a518, a519)?
    };
    nat_mul(a520, a521)
}

pub fn f14(v0: u64) -> bool {
    let a522 = v0;
    let a523 = 4294967296u64;
    if nat_lt(a522, a523) {
        let a527 = {
            let a524 = v0;
            let a525 = 2147483648u64;
            let a526 = 0u64;
            nat_rem(a524, a525, a526)
        };
        let a528 = 2139095040u64;
        nat_lt(a527, a528)
    } else {
        false
    }
}

pub fn f15(v0: u64) -> R<u64> {
    let a529 = 4u64;
    let a530 = v0;
    nat_mul(a529, a530)
}

pub fn f16(v0: u64) -> R<u64> {
    let a531 = v0;
    let a532 = 0u64;
    if nat_eq(a531, a532) {
        Ok(0u64)
    } else {
        let a535 = 4u64;
        let a536 = {
            let a533 = 3u64;
            let a534 = v0;
            nat_mul(a533, a534)?
        };
        nat_add(a535, a536)
    }
}

pub fn f17(v0: Adt2, v1: u64) -> Adt3 {
    Adt3::C0({
        let a538 = match v0.clone() {
            Adt2::C0(_, h537, _, _, _, _, _, _, _, _, _) => {
                h537
            }
        };
        let a539 = v1;
        let a540 = 0u64;
        nat_quot(a538, a539, a540)
    }, match v0.clone() {
        Adt2::C0(_, _, h541, _, _, _, _, _, _, _, _) => {
            h541
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, h542, _, _, _, _, _, _, _) => {
            h542
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, h543, _, _, _, _, _, _) => {
            h543
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, _, h544, _, _, _, _, _) => {
            h544
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, _, _, h545, _, _, _, _) => {
            h545
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, _, _, _, h546, _, _, _) => {
            h546
        }
    }, match v0 {
        Adt2::C0(_, _, _, _, _, _, _, _, h547, _, _) => {
            h547
        }
    })
}

pub fn f18(v0: Adt6, v1: u64) -> R<u64> {
    let a549 = match v0.clone() {
        Adt6::C0(h548, _, _, _) => {
            h548
        }
    };
    let a550 = 0u64;
    if nat_eq(a549, a550) {
        f83(Adt6::C0(1u64, match v0.clone() {
            Adt6::C0(_, h551, _, _) => {
                h551
            }
        }, match v0.clone() {
            Adt6::C0(_, _, h552, _) => {
                h552
            }
        }, match v0.clone() {
            Adt6::C0(_, _, _, h553) => {
                h553
            }
        }), {
            let a555 = v1;
            let a556 = match v0 {
                Adt6::C0(_, _, _, h554) => {
                    h554
                }
            };
            nat_add(a555, a556)?
        })
    } else {
        f83(v0, v1)
    }
}

pub fn f19(v0: Adt2, v1: Adt2) -> R<bool> {
    let a558 = match v0.clone() {
        Adt2::C0(_, _, _, _, _, _, _, _, _, h557, _) => {
            h557
        }
    };
    let a559 = 0u64;
    if nat_eq(a558, a559) {
        Ok(true)
    } else {
        let a561 = match v1.clone() {
            Adt2::C0(_, _, _, _, _, _, _, _, _, h560, _) => {
                h560
            }
        };
        let a562 = 0u64;
        if nat_eq(a561, a562) {
            Ok(true)
        } else {
            let a564 = match v0.clone() {
                Adt2::C0(_, _, _, _, _, _, _, _, _, _, h563) => {
                    h563
                }
            };
            let a565 = 0u64;
            if nat_eq(a564, a565) {
                Ok(true)
            } else {
                let a567 = match v1.clone() {
                    Adt2::C0(_, _, _, _, _, _, _, _, _, _, h566) => {
                        h566
                    }
                };
                let a568 = 0u64;
                if nat_eq(a567, a568) {
                    Ok(true)
                } else {
                    let a574 = {
                        let a571 = match v0.clone() {
                            Adt2::C0(_, _, _, _, _, _, _, _, _, _, h569) => {
                                h569
                            }
                        };
                        let a572 = match v0.clone() {
                            Adt2::C0(_, _, _, _, _, _, _, _, _, h570, _) => {
                                h570
                            }
                        };
                        nat_add(a571, a572)?
                    };
                    let a575 = match v1.clone() {
                        Adt2::C0(_, _, _, _, _, _, _, _, _, _, h573) => {
                            h573
                        }
                    };
                    if nat_le(a574, a575) {
                        Ok(true)
                    } else {
                        let a581 = {
                            let a578 = match v1.clone() {
                                Adt2::C0(_, _, _, _, _, _, _, _, _, _, h576) => {
                                    h576
                                }
                            };
                            let a579 = match v1 {
                                Adt2::C0(_, _, _, _, _, _, _, _, _, h577, _) => {
                                    h577
                                }
                            };
                            nat_add(a578, a579)?
                        };
                        let a582 = match v0 {
                            Adt2::C0(_, _, _, _, _, _, _, _, _, _, h580) => {
                                h580
                            }
                        };
                        Ok(nat_le(a581, a582))
                    }
                }
            }
        }
    }
}

pub fn f20(v0: Adt2, v1: Adt2) -> bool {
    let a585 = match v0.clone() {
        Adt2::C0(_, h583, _, _, _, _, _, _, _, _, _) => {
            h583
        }
    };
    let a586 = match v1.clone() {
        Adt2::C0(_, h584, _, _, _, _, _, _, _, _, _) => {
            h584
        }
    };
    if if if nat_eq(a585, a586) {
        let a589 = match v0.clone() {
            Adt2::C0(_, _, h587, _, _, _, _, _, _, _, _) => {
                h587
            }
        };
        let a590 = match v1.clone() {
            Adt2::C0(_, _, h588, _, _, _, _, _, _, _, _) => {
                h588
            }
        };
        nat_eq(a589, a590)
    } else {
        false
    } {
        let a593 = match v0.clone() {
            Adt2::C0(_, _, _, h591, _, _, _, _, _, _, _) => {
                h591
            }
        };
        let a594 = match v1.clone() {
            Adt2::C0(_, _, _, h592, _, _, _, _, _, _, _) => {
                h592
            }
        };
        nat_eq(a593, a594)
    } else {
        false
    } {
        let a597 = match v0 {
            Adt2::C0(_, _, _, _, h595, _, _, _, _, _, _) => {
                h595
            }
        };
        let a598 = match v1 {
            Adt2::C0(_, _, _, _, h596, _, _, _, _, _, _) => {
                h596
            }
        };
        nat_eq(a597, a598)
    } else {
        false
    }
}

pub fn f21(v0: Adt2) -> R<bool> {
    let a600 = match v0.clone() {
        Adt2::C0(_, h599, _, _, _, _, _, _, _, _, _) => {
            h599
        }
    };
    let a601 = 1u64;
    if if if if nat_eq(a600, a601) {
        true
    } else {
        let a603 = match v0.clone() {
            Adt2::C0(_, _, _, _, _, h602, _, _, _, _, _) => {
                h602
            }
        };
        let a604 = 4u64;
        nat_eq(a603, a604)
    } {
        let a606 = match v0.clone() {
            Adt2::C0(_, _, h605, _, _, _, _, _, _, _, _) => {
                h605
            }
        };
        let a607 = 1u64;
        if nat_eq(a606, a607) {
            true
        } else {
            let a612 = match v0.clone() {
                Adt2::C0(_, _, _, _, _, _, h608, _, _, _, _) => {
                    h608
                }
            };
            let a613 = {
                let a610 = match v0.clone() {
                    Adt2::C0(_, h609, _, _, _, _, _, _, _, _, _) => {
                        h609
                    }
                };
                let a611 = 4u64;
                nat_mul(a610, a611)?
            };
            nat_eq(a612, a613)
        }
    } else {
        false
    } {
        let a615 = match v0.clone() {
            Adt2::C0(_, _, _, h614, _, _, _, _, _, _, _) => {
                h614
            }
        };
        let a616 = 1u64;
        if nat_eq(a615, a616) {
            true
        } else {
            let a624 = match v0.clone() {
                Adt2::C0(_, _, _, _, _, _, _, h617, _, _, _) => {
                    h617
                }
            };
            let a625 = {
                let a622 = {
                    let a620 = match v0.clone() {
                        Adt2::C0(_, h618, _, _, _, _, _, _, _, _, _) => {
                            h618
                        }
                    };
                    let a621 = match v0.clone() {
                        Adt2::C0(_, _, h619, _, _, _, _, _, _, _, _) => {
                            h619
                        }
                    };
                    nat_mul(a620, a621)?
                };
                let a623 = 4u64;
                nat_mul(a622, a623)?
            };
            nat_eq(a624, a625)
        }
    } else {
        false
    } {
        let a627 = match v0.clone() {
            Adt2::C0(_, _, _, _, h626, _, _, _, _, _, _) => {
                h626
            }
        };
        let a628 = 1u64;
        if nat_eq(a627, a628) {
            Ok(true)
        } else {
            let a639 = match v0.clone() {
                Adt2::C0(_, _, _, _, _, _, _, _, h629, _, _) => {
                    h629
                }
            };
            let a640 = {
                let a637 = {
                    let a635 = {
                        let a632 = match v0.clone() {
                            Adt2::C0(_, h630, _, _, _, _, _, _, _, _, _) => {
                                h630
                            }
                        };
                        let a633 = match v0.clone() {
                            Adt2::C0(_, _, h631, _, _, _, _, _, _, _, _) => {
                                h631
                            }
                        };
                        nat_mul(a632, a633)?
                    };
                    let a636 = match v0 {
                        Adt2::C0(_, _, _, h634, _, _, _, _, _, _, _) => {
                            h634
                        }
                    };
                    nat_mul(a635, a636)?
                };
                let a638 = 4u64;
                nat_mul(a637, a638)?
            };
            Ok(nat_eq(a639, a640))
        }
    } else {
        Ok(false)
    }
}

pub fn f22(v0: u64, v1: u64, v2: u64, v3: u64) -> R<u64> {
    let a645 = {
        let a641 = v0;
        let a642 = v1;
        nat_mul(a641, a642)?
    };
    let a646 = {
        let a643 = v2;
        let a644 = f84(v3);
        nat_mul(a643, a644)?
    };
    nat_mul(a645, a646)
}

pub fn f23(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let a649 = v0;
    let a650 = {
        let a647 = f85(v1)?;
        let a648 = v2;
        nat_mul(a647, a648)?
    };
    nat_add(a649, a650)
}

pub fn f24(v0: Adt2) -> Adt14 {
    Adt14::C0(match v0.clone() {
        Adt2::C0(h651, _, _, _, _, _, _, _, _, _, _) => {
            h651
        }
    }, match v0.clone() {
        Adt2::C0(_, h652, _, _, _, _, _, _, _, _, _) => {
            h652
        }
    }, match v0.clone() {
        Adt2::C0(_, _, h653, _, _, _, _, _, _, _, _) => {
            h653
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, h654, _, _, _, _, _, _, _) => {
            h654
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, h655, _, _, _, _, _, _) => {
            h655
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, _, h656, _, _, _, _, _) => {
            h656
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, _, _, h657, _, _, _, _) => {
            h657
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, _, _, _, h658, _, _, _) => {
            h658
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, _, _, _, _, h659, _, _) => {
            h659
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, _, _, _, _, _, h660, _) => {
            h660
        }
    }, match v0 {
        Adt2::C0(_, _, _, _, _, _, _, _, _, _, h661) => {
            h661
        }
    })
}

pub fn f25(v0: u64, v1: u64, v2: u64, v3: bool, v4: Adt14, v5: Adt14, v6: Adt14) -> R<Adt15> {
    let a662 = v0;
    let a663 = 23u64;
    if nat_eq(a662, a663) {
        f86(v1, v4, v6)
    } else {
        let a664 = v0;
        let a665 = 26u64;
        if nat_eq(a664, a665) {
            f87(v1, v2, v4, v6)
        } else {
            let a666 = v0;
            let a667 = 25u64;
            if nat_eq(a666, a667) {
                f88(v1, v3, v4, v5, v6)
            } else {
                Ok(Adt15::C0)
            }
        }
    }
}

pub fn f26(v0: u64) -> u64 {
    let a668 = v0;
    let a669 = 30u64;
    if nat_eq(a668, a669) {
        2u64
    } else {
        4u64
    }
}

pub fn f27(v0: u64) -> bool {
    let a670 = v0;
    let a671 = 2139095040u64;
    if nat_le(a670, a671) {
        true
    } else {
        let a672 = v0;
        let a673 = 2147483648u64;
        nat_eq(a672, a673)
    }
}

pub fn f28(v0: Adt2, v1: u64) -> bool {
    let a675 = match v0.clone() {
        Adt2::C0(_, _, _, _, _, h674, _, _, _, _, _) => {
            h674
        }
    };
    let a676 = v1;
    if if nat_eq(a675, a676) {
        let a681 = {
            let a678 = match v0.clone() {
                Adt2::C0(_, _, _, _, _, _, _, _, _, _, h677) => {
                    h677
                }
            };
            let a679 = v1;
            let a680 = 0u64;
            nat_rem(a678, a679, a680)
        };
        let a682 = 0u64;
        nat_eq(a681, a682)
    } else {
        false
    } {
        f89(match v0.clone() {
            Adt2::C0(_, _, h683, _, _, _, _, _, _, _, _) => {
                h683
            }
        }, match v0.clone() {
            Adt2::C0(_, _, _, h684, _, _, _, _, _, _, _) => {
                h684
            }
        }, match v0.clone() {
            Adt2::C0(_, _, _, _, h685, _, _, _, _, _, _) => {
                h685
            }
        }, match v0.clone() {
            Adt2::C0(_, _, _, _, _, _, h686, _, _, _, _) => {
                h686
            }
        }, match v0.clone() {
            Adt2::C0(_, _, _, _, _, _, _, h687, _, _, _) => {
                h687
            }
        }, match v0 {
            Adt2::C0(_, _, _, _, _, _, _, _, h688, _, _) => {
                h688
            }
        }, v1)
    } else {
        false
    }
}

pub fn f29(v0: Adt2, v1: Adt2) -> bool {
    if if if if if if f20(v0.clone(), v1.clone()) {
        let a691 = match v0.clone() {
            Adt2::C0(h689, _, _, _, _, _, _, _, _, _, _) => {
                h689
            }
        };
        let a692 = match v1.clone() {
            Adt2::C0(h690, _, _, _, _, _, _, _, _, _, _) => {
                h690
            }
        };
        nat_eq(a691, a692)
    } else {
        false
    } {
        let a695 = match v0.clone() {
            Adt2::C0(_, _, _, _, _, _, _, _, _, _, h693) => {
                h693
            }
        };
        let a696 = match v1.clone() {
            Adt2::C0(_, _, _, _, _, _, _, _, _, _, h694) => {
                h694
            }
        };
        nat_eq(a695, a696)
    } else {
        false
    } {
        let a699 = match v0.clone() {
            Adt2::C0(_, _, _, _, _, h697, _, _, _, _, _) => {
                h697
            }
        };
        let a700 = match v1.clone() {
            Adt2::C0(_, _, _, _, _, h698, _, _, _, _, _) => {
                h698
            }
        };
        nat_eq(a699, a700)
    } else {
        false
    } {
        let a703 = match v0.clone() {
            Adt2::C0(_, _, _, _, _, _, h701, _, _, _, _) => {
                h701
            }
        };
        let a704 = match v1.clone() {
            Adt2::C0(_, _, _, _, _, _, h702, _, _, _, _) => {
                h702
            }
        };
        nat_eq(a703, a704)
    } else {
        false
    } {
        let a707 = match v0.clone() {
            Adt2::C0(_, _, _, _, _, _, _, h705, _, _, _) => {
                h705
            }
        };
        let a708 = match v1.clone() {
            Adt2::C0(_, _, _, _, _, _, _, h706, _, _, _) => {
                h706
            }
        };
        nat_eq(a707, a708)
    } else {
        false
    } {
        let a711 = match v0 {
            Adt2::C0(_, _, _, _, _, _, _, _, h709, _, _) => {
                h709
            }
        };
        let a712 = match v1 {
            Adt2::C0(_, _, _, _, _, _, _, _, h710, _, _) => {
                h710
            }
        };
        nat_eq(a711, a712)
    } else {
        false
    }
}

pub fn f30(v0: u64, v1: u64, v2: u64, v3: u64) -> bool {
    let a713 = v0;
    let a714 = 0u64;
    if if if nat_eq(a713, a714) {
        true
    } else {
        let a715 = v1;
        let a716 = 0u64;
        nat_eq(a715, a716)
    } {
        true
    } else {
        let a717 = v2;
        let a718 = 0u64;
        nat_eq(a717, a718)
    } {
        true
    } else {
        let a719 = v3;
        let a720 = 0u64;
        nat_eq(a719, a720)
    }
}

pub fn f31(v0: Adt2, v1: u64) -> R<bool> {
    f90({
        let a722 = match v0.clone() {
            Adt2::C0(_, h721, _, _, _, _, _, _, _, _, _) => {
                h721
            }
        };
        let a723 = v1;
        nat_mul(a722, a723)?
    }, match v0.clone() {
        Adt2::C0(_, _, h724, _, _, _, _, _, _, _, _) => {
            h724
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, h725, _, _, _, _, _, _, _) => {
            h725
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, h726, _, _, _, _, _, _) => {
            h726
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, _, _, h727, _, _, _, _) => {
            h727
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, _, _, _, h728, _, _, _) => {
            h728
        }
    }, match v0 {
        Adt2::C0(_, _, _, _, _, _, _, _, h729, _, _) => {
            h729
        }
    })
}

pub fn f32(v0: u64, v1: u64, v2: u64, v3: u64) -> R<u64> {
    let a730 = v1;
    let a731 = 0u64;
    if if nat_eq(a730, a731) {
        true
    } else {
        let a732 = v2;
        let a733 = 0u64;
        nat_eq(a732, a733)
    } {
        Ok(0u64)
    } else {
        let a740 = {
            let a734 = f91(v1, v2, v3)?;
            let a735 = f92(v0, v1, v2)?;
            nat_add(a734, a735)?
        };
        let a741 = {
            let a738 = f93(v0);
            let a739 = {
                let a736 = v1;
                let a737 = v2;
                nat_mul(a736, a737)?
            };
            nat_mul(a738, a739)?
        };
        nat_add(a740, a741)
    }
}

pub fn f33(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let a742 = v0;
    let a743 = 0u64;
    if if nat_eq(a742, a743) {
        true
    } else {
        let a744 = v1;
        let a745 = 0u64;
        nat_eq(a744, a745)
    } {
        Ok(3u64)
    } else {
        let a748 = {
            let a746 = 3u64;
            let a747 = v1;
            nat_add(a746, a747)?
        };
        let a749 = v2;
        nat_add(a748, a749)
    }
}

pub fn f34(v0: u64) -> bool {
    let a750 = v0;
    let a751 = 2139095040u64;
    if nat_le(a750, a751) {
        true
    } else {
        let a752 = v0;
        let a753 = 2147483648u64;
        nat_eq(a752, a753)
    }
}

pub fn f35(v0: u64, v1: u64) -> R<u64> {
    let a754 = v0;
    let a755 = 0u64;
    if nat_eq(a754, a755) {
        Ok(0u64)
    } else {
        let a756 = v1;
        let a757 = f94(v0)?;
        nat_mul(a756, a757)
    }
}

pub fn f36(v0: u64, v1: u64, v2: u64, v3: u64) -> R<u64> {
    let a758 = v0;
    let a759 = 0u64;
    if if if if nat_eq(a758, a759) {
        true
    } else {
        let a760 = v1;
        let a761 = 0u64;
        nat_eq(a760, a761)
    } {
        true
    } else {
        let a762 = v2;
        let a763 = 0u64;
        nat_eq(a762, a763)
    } {
        true
    } else {
        let a764 = v3;
        let a765 = 0u64;
        nat_eq(a764, a765)
    } {
        Ok(0u64)
    } else {
        let a778 = {
            let a768 = 2u64;
            let a769 = {
                let a766 = v1;
                let a767 = v2;
                nat_mul(a766, a767)?
            };
            nat_mul(a768, a769)?
        };
        let a779 = {
            let a776 = {
                let a772 = {
                    let a770 = v1;
                    let a771 = v2;
                    nat_mul(a770, a771)?
                };
                let a773 = v3;
                nat_mul(a772, a773)?
            };
            let a777 = {
                let a774 = 1u64;
                let a775 = v0;
                nat_add(a774, a775)?
            };
            nat_mul(a776, a777)?
        };
        nat_add(a778, a779)
    }
}

pub fn f37(v0: Adt1, v1: Adt2, v2: Adt2) -> R<Adt10> {
    let v3: u64 = match v0.clone() {
        Adt1::C0(h780, _, _, _, _, _, _, _, _, _, _, _) => {
            h780
        }
    };
    let v4: u64 = match v0.clone() {
        Adt1::C0(_, h781, _, _, _, _, _, _, _, _, _, _) => {
            h781
        }
    };
    let v5: u64 = match v0 {
        Adt1::C0(_, _, h782, _, _, _, _, _, _, _, _, _) => {
            h782
        }
    };
    let v6: u64 = {
        let a784 = match v1.clone() {
            Adt2::C0(_, h783, _, _, _, _, _, _, _, _, _) => {
                h783
            }
        };
        let a785 = v4;
        let a786 = 0u64;
        nat_quot(a784, a785, a786)
    };
    let v7: u64 = {
        let a788 = match v1.clone() {
            Adt2::C0(h787, _, _, _, _, _, _, _, _, _, _) => {
                h787
            }
        };
        let a789 = 0u64;
        if nat_eq(a788, a789) {
            4u64
        } else {
            2u64
        }
    };
    if if if if if if if if if if if if if if if f95(match v1.clone() {
        Adt2::C0(h790, _, _, _, _, _, _, _, _, _, _) => {
            h790
        }
    }) {
        let a793 = match v1.clone() {
            Adt2::C0(h791, _, _, _, _, _, _, _, _, _, _) => {
                h791
            }
        };
        let a794 = match v2.clone() {
            Adt2::C0(h792, _, _, _, _, _, _, _, _, _, _) => {
                h792
            }
        };
        nat_eq(a793, a794)
    } else {
        false
    } {
        let a795 = 0u64;
        let a796 = v3;
        nat_lt(a795, a796)
    } else {
        false
    } {
        let a797 = v3;
        let a798 = 2147483648u64;
        nat_lt(a797, a798)
    } else {
        false
    } {
        let a799 = 0u64;
        let a800 = v4;
        nat_lt(a799, a800)
    } else {
        false
    } {
        let a801 = v4;
        let a802 = 2147483648u64;
        nat_lt(a801, a802)
    } else {
        false
    } {
        let a803 = v5;
        let a804 = 2147483648u64;
        nat_lt(a803, a804)
    } else {
        false
    } {
        let a805 = 0u64;
        let a806 = v6;
        nat_lt(a805, a806)
    } else {
        false
    } {
        let a808 = 0u64;
        let a809 = match v2.clone() {
            Adt2::C0(_, h807, _, _, _, _, _, _, _, _, _) => {
                h807
            }
        };
        nat_lt(a808, a809)
    } else {
        false
    } {
        let a813 = match v1.clone() {
            Adt2::C0(_, h810, _, _, _, _, _, _, _, _, _) => {
                h810
            }
        };
        let a814 = {
            let a811 = v6;
            let a812 = v4;
            nat_mul(a811, a812)?
        };
        nat_eq(a813, a814)
    } else {
        false
    } {
        let a816 = match v2.clone() {
            Adt2::C0(_, _, h815, _, _, _, _, _, _, _, _) => {
                h815
            }
        };
        let a817 = v4;
        nat_eq(a816, a817)
    } else {
        false
    } {
        let a830 = {
            let a823 = {
                let a821 = match v2.clone() {
                    Adt2::C0(_, h818, _, _, _, _, _, _, _, _, _) => {
                        h818
                    }
                };
                let a822 = {
                    let a819 = v5;
                    let a820 = 2u64;
                    nat_mul(a819, a820)?
                };
                nat_add(a821, a822)?
            };
            let a824 = v3;
            nat_add(a823, a824)?
        };
        let a831 = {
            let a828 = {
                let a826 = match v1.clone() {
                    Adt2::C0(_, _, h825, _, _, _, _, _, _, _, _) => {
                        h825
                    }
                };
                let a827 = v3;
                nat_mul(a826, a827)?
            };
            let a829 = v6;
            nat_add(a828, a829)?
        };
        nat_eq(a830, a831)
    } else {
        false
    } {
        f96(v1.clone(), v7)?
    } else {
        false
    } {
        f96(v2.clone(), v7)?
    } else {
        false
    } {
        f19(v1.clone(), v2.clone())?
    } else {
        false
    } {
        let v8: Adt6 = Adt6::C0(v6, v3, v5, 1u64);
        Ok(Adt10::C4({
            let a832 = f13(v2.clone(), 1u64)?;
            let a833 = f97(v8.clone())?;
            nat_mul(a832, a833)?
        }, v7, f17(v1.clone(), 1u64), f17(v2, 1u64), Adt4::C30(v8, match v1 {
            Adt2::C0(h834, _, _, _, _, _, _, _, _, _, _) => {
                h834
            }
        })))
    } else {
        Ok(Adt10::C1)
    }
}

pub fn f38(v0: Adt2, v1: Option<Adt2>, v2: Adt2) -> R<Adt10> {
    let m835 = v1;
    match m835 {
        None => {
            Ok(Adt10::C1)
        }
        Some(v3) => {
            let a837 = match v0.clone() {
                Adt2::C0(h836, _, _, _, _, _, _, _, _, _, _) => {
                    h836
                }
            };
            let a838 = 0u64;
            if if if if if if if if if if if if if if if if if if if nat_eq(a837, a838) {
                let a840 = match v3.clone() {
                    Adt2::C0(h839, _, _, _, _, _, _, _, _, _, _) => {
                        h839
                    }
                };
                let a841 = 0u64;
                nat_eq(a840, a841)
            } else {
                false
            } {
                let a843 = match v2.clone() {
                    Adt2::C0(h842, _, _, _, _, _, _, _, _, _, _) => {
                        h842
                    }
                };
                let a844 = 0u64;
                nat_eq(a843, a844)
            } else {
                false
            } {
                let a846 = match v0.clone() {
                    Adt2::C0(_, _, _, _, h845, _, _, _, _, _, _) => {
                        h845
                    }
                };
                let a847 = 1u64;
                nat_eq(a846, a847)
            } else {
                false
            } {
                let a849 = match v3.clone() {
                    Adt2::C0(_, _, _, h848, _, _, _, _, _, _, _) => {
                        h848
                    }
                };
                let a850 = 1u64;
                nat_eq(a849, a850)
            } else {
                false
            } {
                let a852 = match v3.clone() {
                    Adt2::C0(_, _, _, _, h851, _, _, _, _, _, _) => {
                        h851
                    }
                };
                let a853 = 1u64;
                nat_eq(a852, a853)
            } else {
                false
            } {
                let a856 = match v0.clone() {
                    Adt2::C0(_, _, h854, _, _, _, _, _, _, _, _) => {
                        h854
                    }
                };
                let a857 = match v3.clone() {
                    Adt2::C0(_, _, h855, _, _, _, _, _, _, _, _) => {
                        h855
                    }
                };
                nat_eq(a856, a857)
            } else {
                false
            } {
                let a862 = match v3.clone() {
                    Adt2::C0(_, h858, _, _, _, _, _, _, _, _, _) => {
                        h858
                    }
                };
                let a863 = {
                    let a860 = match v0.clone() {
                        Adt2::C0(_, h859, _, _, _, _, _, _, _, _, _) => {
                            h859
                        }
                    };
                    let a861 = 1u64;
                    nat_add(a860, a861)?
                };
                nat_le(a862, a863)
            } else {
                false
            } {
                let a866 = match v2.clone() {
                    Adt2::C0(_, h864, _, _, _, _, _, _, _, _, _) => {
                        h864
                    }
                };
                let a867 = match v0.clone() {
                    Adt2::C0(_, _, h865, _, _, _, _, _, _, _, _) => {
                        h865
                    }
                };
                nat_eq(a866, a867)
            } else {
                false
            } {
                let a875 = match v2.clone() {
                    Adt2::C0(_, _, h868, _, _, _, _, _, _, _, _) => {
                        h868
                    }
                };
                let a876 = {
                    let a873 = {
                        let a870 = match v0.clone() {
                            Adt2::C0(_, h869, _, _, _, _, _, _, _, _, _) => {
                                h869
                            }
                        };
                        let a871 = 1u64;
                        nat_add(a870, a871)?
                    };
                    let a874 = match v3.clone() {
                        Adt2::C0(_, h872, _, _, _, _, _, _, _, _, _) => {
                            h872
                        }
                    };
                    nat_sub(a873, a874)
                };
                nat_eq(a875, a876)
            } else {
                false
            } {
                let a879 = match v2.clone() {
                    Adt2::C0(_, _, _, h877, _, _, _, _, _, _, _) => {
                        h877
                    }
                };
                let a880 = match v0.clone() {
                    Adt2::C0(_, _, _, h878, _, _, _, _, _, _, _) => {
                        h878
                    }
                };
                nat_eq(a879, a880)
            } else {
                false
            } {
                let a882 = match v2.clone() {
                    Adt2::C0(_, _, _, _, h881, _, _, _, _, _, _) => {
                        h881
                    }
                };
                let a883 = 1u64;
                nat_eq(a882, a883)
            } else {
                false
            } {
                f21(v2.clone())?
            } else {
                false
            } {
                let a885 = match v0.clone() {
                    Adt2::C0(_, _, _, _, _, h884, _, _, _, _, _) => {
                        h884
                    }
                };
                let a886 = 4u64;
                nat_eq(a885, a886)
            } else {
                false
            } {
                let a888 = match v3.clone() {
                    Adt2::C0(_, _, _, _, _, h887, _, _, _, _, _) => {
                        h887
                    }
                };
                let a889 = 4u64;
                nat_eq(a888, a889)
            } else {
                false
            } {
                let a894 = match v0.clone() {
                    Adt2::C0(_, _, _, _, _, _, h890, _, _, _, _) => {
                        h890
                    }
                };
                let a895 = {
                    let a892 = match v0.clone() {
                        Adt2::C0(_, h891, _, _, _, _, _, _, _, _, _) => {
                            h891
                        }
                    };
                    let a893 = 4u64;
                    nat_mul(a892, a893)?
                };
                nat_eq(a894, a895)
            } else {
                false
            } {
                let a900 = match v3.clone() {
                    Adt2::C0(_, _, _, _, _, _, h896, _, _, _, _) => {
                        h896
                    }
                };
                let a901 = {
                    let a898 = match v3.clone() {
                        Adt2::C0(_, h897, _, _, _, _, _, _, _, _, _) => {
                            h897
                        }
                    };
                    let a899 = 4u64;
                    nat_mul(a898, a899)?
                };
                nat_eq(a900, a901)
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
                    let a903 = f13(v2.clone(), 1u64)?;
                    let a904 = f98(match v3.clone() {
                        Adt2::C0(_, h902, _, _, _, _, _, _, _, _, _) => {
                            h902
                        }
                    })?;
                    nat_mul(a903, a904)?
                }, 4u64, f17(v0, 1u64), f17(v2, 1u64), Adt4::C29(f17(v3, 1u64))))
            } else {
                Ok(Adt10::C1)
            }
        }
    }
}

pub fn f39(v0: bool, v1: Adt1, v2: Adt2, v3: Adt2) -> R<Adt10> {
    let v4: Adt6 = Adt6::C0(match v1.clone() {
        Adt1::C0(_, h905, _, _, _, _, _, _, _, _, _, _) => {
            h905
        }
    }, if v0 {
        match v1.clone() {
            Adt1::C0(_, _, _, h906, _, _, _, _, _, _, _, _) => {
                h906
            }
        }
    } else {
        match v1.clone() {
            Adt1::C0(_, _, h907, _, _, _, _, _, _, _, _, _) => {
                h907
            }
        }
    }, if v0 {
        match v1.clone() {
            Adt1::C0(_, _, _, _, _, h908, _, _, _, _, _, _) => {
                h908
            }
        }
    } else {
        match v1.clone() {
            Adt1::C0(_, _, _, h909, _, _, _, _, _, _, _, _) => {
                h909
            }
        }
    }, 1u64);
    let v5: Adt6 = if v0 {
        Adt6::C0(match v1.clone() {
            Adt1::C0(_, _, h910, _, _, _, _, _, _, _, _, _) => {
                h910
            }
        }, match v1.clone() {
            Adt1::C0(_, _, _, _, h911, _, _, _, _, _, _, _) => {
                h911
            }
        }, match v1.clone() {
            Adt1::C0(_, _, _, _, _, _, h912, _, _, _, _, _) => {
                h912
            }
        }, 1u64)
    } else {
        Adt6::C0(1u64, 1u64, 0u64, 1u64)
    };
    let a914 = match v1.clone() {
        Adt1::C0(h913, _, _, _, _, _, _, _, _, _, _, _) => {
            h913
        }
    };
    let a915 = 1u64;
    if if if if if if if if if if if nat_le(a914, a915) {
        let a917 = match v2.clone() {
            Adt2::C0(h916, _, _, _, _, _, _, _, _, _, _) => {
                h916
            }
        };
        let a918 = 1u64;
        nat_le(a917, a918)
    } else {
        false
    } {
        let a920 = match v3.clone() {
            Adt2::C0(h919, _, _, _, _, _, _, _, _, _, _) => {
                h919
            }
        };
        let a921 = 0u64;
        nat_eq(a920, a921)
    } else {
        false
    } {
        let a926 = match v2.clone() {
            Adt2::C0(_, _, _, _, _, h922, _, _, _, _, _) => {
                h922
            }
        };
        let a927 = {
            let a924 = match v2.clone() {
                Adt2::C0(h923, _, _, _, _, _, _, _, _, _, _) => {
                    h923
                }
            };
            let a925 = 0u64;
            if nat_eq(a924, a925) {
                4u64
            } else {
                2u64
            }
        };
        nat_eq(a926, a927)
    } else {
        false
    } {
        f21(v3.clone())?
    } else {
        false
    } {
        f19(v2.clone(), v3.clone())?
    } else {
        false
    } {
        f99(v4.clone(), match v2.clone() {
            Adt2::C0(_, h928, _, _, _, _, _, _, _, _, _) => {
                h928
            }
        }, match v3.clone() {
            Adt2::C0(_, h929, _, _, _, _, _, _, _, _, _) => {
                h929
            }
        })?
    } else {
        false
    } {
        if v0 {
            f99(v5.clone(), match v2.clone() {
                Adt2::C0(_, _, h930, _, _, _, _, _, _, _, _) => {
                    h930
                }
            }, match v3.clone() {
                Adt2::C0(_, _, h931, _, _, _, _, _, _, _, _) => {
                    h931
                }
            })?
        } else {
            let a934 = match v2.clone() {
                Adt2::C0(_, _, h932, _, _, _, _, _, _, _, _) => {
                    h932
                }
            };
            let a935 = match v3.clone() {
                Adt2::C0(_, _, h933, _, _, _, _, _, _, _, _) => {
                    h933
                }
            };
            nat_eq(a934, a935)
        }
    } else {
        false
    } {
        let a938 = match v2.clone() {
            Adt2::C0(_, _, _, h936, _, _, _, _, _, _, _) => {
                h936
            }
        };
        let a939 = match v3.clone() {
            Adt2::C0(_, _, _, h937, _, _, _, _, _, _, _) => {
                h937
            }
        };
        nat_eq(a938, a939)
    } else {
        false
    } {
        let a942 = match v2.clone() {
            Adt2::C0(_, _, _, _, h940, _, _, _, _, _, _) => {
                h940
            }
        };
        let a943 = match v3.clone() {
            Adt2::C0(_, _, _, _, h941, _, _, _, _, _, _) => {
                h941
            }
        };
        nat_eq(a942, a943)
    } else {
        false
    } {
        let a948 = {
            let a946 = match v4.clone() {
                Adt6::C0(h944, _, _, _) => {
                    h944
                }
            };
            let a947 = match v5.clone() {
                Adt6::C0(h945, _, _, _) => {
                    h945
                }
            };
            nat_mul(a946, a947)?
        };
        let a949 = 2147483648u64;
        nat_lt(a948, a949)
    } else {
        false
    } {
        let v6: Adt7 = Adt7::C0(v4, v5, v0, {
            let a951 = match v1 {
                Adt1::C0(h950, _, _, _, _, _, _, _, _, _, _, _) => {
                    h950
                }
            };
            let a952 = 0u64;
            nat_eq(a951, a952)
        }, match v2.clone() {
            Adt2::C0(h953, _, _, _, _, _, _, _, _, _, _) => {
                h953
            }
        });
        Ok(Adt10::C4({
            let a954 = f13(v3.clone(), 1u64)?;
            let a955 = f100(v6.clone())?;
            nat_mul(a954, a955)?
        }, 4u64, f17(v2, 1u64), f17(v3, 1u64), Adt4::C28(v6)))
    } else {
        Ok(Adt10::C1)
    }
}

pub fn f40(v0: Adt1, v1: Adt2, v2: Option<Adt2>, v3: Adt2) -> R<Adt10> {
    let m956 = v2;
    match m956 {
        None => {
            Ok(Adt10::C1)
        }
        Some(v4) => {
            let v5: bool = {
                let a958 = match v0.clone() {
                    Adt1::C0(_, _, _, _, _, _, h957, _, _, _, _, _) => {
                        h957
                    }
                };
                let a959 = 1u64;
                nat_eq(a958, a959)
            };
            let v6: Adt6 = Adt6::C0(match v1.clone() {
                Adt2::C0(_, h960, _, _, _, _, _, _, _, _, _) => {
                    h960
                }
            }, match v0.clone() {
                Adt1::C0(h961, _, _, _, _, _, _, _, _, _, _, _) => {
                    h961
                }
            }, match v0.clone() {
                Adt1::C0(_, _, h962, _, _, _, _, _, _, _, _, _) => {
                    h962
                }
            }, match v0.clone() {
                Adt1::C0(_, _, _, _, h963, _, _, _, _, _, _, _) => {
                    h963
                }
            });
            let v7: Adt6 = if v5 {
                Adt6::C0(match v1.clone() {
                    Adt2::C0(_, _, h964, _, _, _, _, _, _, _, _) => {
                        h964
                    }
                }, match v0.clone() {
                    Adt1::C0(_, h965, _, _, _, _, _, _, _, _, _, _) => {
                        h965
                    }
                }, match v0.clone() {
                    Adt1::C0(_, _, _, h966, _, _, _, _, _, _, _, _) => {
                        h966
                    }
                }, match v0.clone() {
                    Adt1::C0(_, _, _, _, _, h967, _, _, _, _, _, _) => {
                        h967
                    }
                })
            } else {
                Adt6::C0(1u64, 1u64, 0u64, 1u64)
            };
            let v8: Adt6 = Adt6::C0(1u64, 1u64, 0u64, 1u64);
            let v9: u64 = if v5 {
                match v4.clone() {
                    Adt2::C0(_, _, _, h968, _, _, _, _, _, _, _) => {
                        h968
                    }
                }
            } else {
                match v4.clone() {
                    Adt2::C0(_, _, h969, _, _, _, _, _, _, _, _) => {
                        h969
                    }
                }
            };
            let v10: u64 = if v5 {
                match v4.clone() {
                    Adt2::C0(_, _, _, _, h970, _, _, _, _, _, _) => {
                        h970
                    }
                }
            } else {
                match v4.clone() {
                    Adt2::C0(_, _, _, h971, _, _, _, _, _, _, _) => {
                        h971
                    }
                }
            };
            let v11: u64 = if v5 {
                match v4.clone() {
                    Adt2::C0(_, _, h972, _, _, _, _, _, _, _, _) => {
                        h972
                    }
                }
            } else {
                1u64
            };
            let v12: u64 = match v4.clone() {
                Adt2::C0(h973, _, _, _, _, _, _, _, _, _, _) => {
                    h973
                }
            };
            let v13: u64 = match v3.clone() {
                Adt2::C0(h974, _, _, _, _, _, _, _, _, _, _) => {
                    h974
                }
            };
            let v14: u64 = {
                let a975 = v12;
                let a976 = 0u64;
                if nat_eq(a975, a976) {
                    4u64
                } else {
                    2u64
                }
            };
            let v15: u64 = {
                let a978 = match v4.clone() {
                    Adt2::C0(_, h977, _, _, _, _, _, _, _, _, _) => {
                        h977
                    }
                };
                let a979 = v14;
                nat_mul(a978, a979)?
            };
            let v16: u64 = if v5 {
                match v4.clone() {
                    Adt2::C0(_, _, _, _, _, _, _, h980, _, _, _) => {
                        h980
                    }
                }
            } else {
                match v4.clone() {
                    Adt2::C0(_, _, _, _, _, _, h981, _, _, _, _) => {
                        h981
                    }
                }
            };
            let v17: u64 = if v5 {
                match v4.clone() {
                    Adt2::C0(_, _, _, _, _, _, _, _, h982, _, _) => {
                        h982
                    }
                }
            } else {
                match v4.clone() {
                    Adt2::C0(_, _, _, _, _, _, _, h983, _, _, _) => {
                        h983
                    }
                }
            };
            let a985 = match v0 {
                Adt1::C0(_, _, _, _, _, _, h984, _, _, _, _, _) => {
                    h984
                }
            };
            let a986 = 1u64;
            if if if if if if if if if if nat_le(a985, a986) {
                let a987 = v12;
                let a988 = 0u64;
                if nat_eq(a987, a988) {
                    true
                } else {
                    let a989 = v12;
                    let a990 = 1u64;
                    if nat_eq(a989, a990) {
                        let a991 = v13;
                        let a992 = 1u64;
                        nat_eq(a991, a992)
                    } else {
                        false
                    }
                }
            } else {
                false
            } {
                let a993 = v13;
                let a994 = 0u64;
                if nat_eq(a993, a994) {
                    true
                } else {
                    let a995 = v13;
                    let a996 = 1u64;
                    nat_eq(a995, a996)
                }
            } else {
                false
            } {
                let a998 = match v4.clone() {
                    Adt2::C0(_, _, _, _, _, h997, _, _, _, _, _) => {
                        h997
                    }
                };
                let a999 = v14;
                nat_eq(a998, a999)
            } else {
                false
            } {
                let a1007 = match v3.clone() {
                    Adt2::C0(_, h1000, _, _, _, _, _, _, _, _, _) => {
                        h1000
                    }
                };
                let a1008 = {
                    let a1005 = v9;
                    let a1006 = {
                        let a1003 = match v6.clone() {
                            Adt6::C0(h1001, _, _, _) => {
                                h1001
                            }
                        };
                        let a1004 = match v7.clone() {
                            Adt6::C0(h1002, _, _, _) => {
                                h1002
                            }
                        };
                        nat_mul(a1003, a1004)?
                    };
                    nat_mul(a1005, a1006)?
                };
                nat_eq(a1007, a1008)
            } else {
                false
            } {
                let a1011 = match v3.clone() {
                    Adt2::C0(_, _, h1009, _, _, _, _, _, _, _, _) => {
                        h1009
                    }
                };
                let a1012 = f83(v6.clone(), match v4.clone() {
                    Adt2::C0(_, h1010, _, _, _, _, _, _, _, _, _) => {
                        h1010
                    }
                })?;
                nat_eq(a1011, a1012)
            } else {
                false
            } {
                let a1014 = 0u64;
                let a1015 = match v3.clone() {
                    Adt2::C0(_, _, h1013, _, _, _, _, _, _, _, _) => {
                        h1013
                    }
                };
                nat_lt(a1014, a1015)
            } else {
                false
            } {
                if v5 {
                    let a1017 = match v1 {
                        Adt2::C0(_, _, _, h1016, _, _, _, _, _, _, _) => {
                            h1016
                        }
                    };
                    let a1018 = v9;
                    if if if nat_eq(a1017, a1018) {
                        let a1020 = match v3.clone() {
                            Adt2::C0(_, _, _, h1019, _, _, _, _, _, _, _) => {
                                h1019
                            }
                        };
                        let a1021 = f83(v7.clone(), v11)?;
                        nat_eq(a1020, a1021)
                    } else {
                        false
                    } {
                        let a1023 = match v3.clone() {
                            Adt2::C0(_, _, _, _, h1022, _, _, _, _, _, _) => {
                                h1022
                            }
                        };
                        let a1024 = v10;
                        nat_eq(a1023, a1024)
                    } else {
                        false
                    } {
                        let a1026 = 0u64;
                        let a1027 = match v3.clone() {
                            Adt2::C0(_, _, _, h1025, _, _, _, _, _, _, _) => {
                                h1025
                            }
                        };
                        nat_lt(a1026, a1027)
                    } else {
                        false
                    }
                } else {
                    let a1029 = match v1 {
                        Adt2::C0(_, _, h1028, _, _, _, _, _, _, _, _) => {
                            h1028
                        }
                    };
                    let a1030 = v9;
                    if if if nat_eq(a1029, a1030) {
                        let a1032 = match v4.clone() {
                            Adt2::C0(_, _, _, _, h1031, _, _, _, _, _, _) => {
                                h1031
                            }
                        };
                        let a1033 = 1u64;
                        nat_eq(a1032, a1033)
                    } else {
                        false
                    } {
                        let a1035 = match v3.clone() {
                            Adt2::C0(_, _, _, h1034, _, _, _, _, _, _, _) => {
                                h1034
                            }
                        };
                        let a1036 = v10;
                        nat_eq(a1035, a1036)
                    } else {
                        false
                    } {
                        let a1038 = match v3.clone() {
                            Adt2::C0(_, _, _, _, h1037, _, _, _, _, _, _) => {
                                h1037
                            }
                        };
                        let a1039 = 1u64;
                        nat_eq(a1038, a1039)
                    } else {
                        false
                    }
                }
            } else {
                false
            } {
                if f30(match v4.clone() {
                    Adt2::C0(_, h1040, _, _, _, _, _, _, _, _, _) => {
                        h1040
                    }
                }, v11, v9, v10) {
                    true
                } else {
                    let a1060 = {
                        let a1057 = {
                            let a1055 = {
                                let a1049 = {
                                    let a1043 = {
                                        let a1041 = v10;
                                        let a1042 = 1u64;
                                        nat_sub(a1041, a1042)
                                    };
                                    let a1044 = v17;
                                    nat_mul(a1043, a1044)?
                                };
                                let a1050 = {
                                    let a1047 = {
                                        let a1045 = v9;
                                        let a1046 = 1u64;
                                        nat_sub(a1045, a1046)
                                    };
                                    let a1048 = v16;
                                    nat_mul(a1047, a1048)?
                                };
                                nat_add(a1049, a1050)?
                            };
                            let a1056 = {
                                let a1053 = {
                                    let a1051 = v11;
                                    let a1052 = 1u64;
                                    nat_sub(a1051, a1052)
                                };
                                let a1054 = v15;
                                nat_mul(a1053, a1054)?
                            };
                            nat_add(a1055, a1056)?
                        };
                        let a1058 = v15;
                        nat_add(a1057, a1058)?
                    };
                    let a1061 = match v4.clone() {
                        Adt2::C0(_, _, _, _, _, _, _, _, _, h1059, _) => {
                            h1059
                        }
                    };
                    nat_le(a1060, a1061)
                }
            } else {
                false
            } {
                f19(v4.clone(), v3.clone())?
            } else {
                false
            } {
                Ok(Adt10::C4(f13(v3.clone(), 1u64)?, {
                    let a1062 = v13;
                    let a1063 = 0u64;
                    if nat_eq(a1062, a1063) {
                        4u64
                    } else {
                        2u64
                    }
                }, Adt3::C0(match v4 {
                    Adt2::C0(_, h1064, _, _, _, _, _, _, _, _, _) => {
                        h1064
                    }
                }, v11, 1u64, v10, v14, v15, 0u64, 0u64), if v5 {
                    f17(v3, 1u64)
                } else {
                    Adt3::C0(match v3.clone() {
                        Adt2::C0(_, h1065, _, _, _, _, _, _, _, _, _) => {
                            h1065
                        }
                    }, match v3.clone() {
                        Adt2::C0(_, _, h1066, _, _, _, _, _, _, _, _) => {
                            h1066
                        }
                    }, 1u64, match v3.clone() {
                        Adt2::C0(_, _, _, h1067, _, _, _, _, _, _, _) => {
                            h1067
                        }
                    }, match v3.clone() {
                        Adt2::C0(_, _, _, _, _, h1068, _, _, _, _, _) => {
                            h1068
                        }
                    }, match v3.clone() {
                        Adt2::C0(_, _, _, _, _, _, h1069, _, _, _, _) => {
                            h1069
                        }
                    }, 0u64, match v3 {
                        Adt2::C0(_, _, _, _, _, _, _, h1070, _, _, _) => {
                            h1070
                        }
                    })
                }, Adt4::C27(Adt5::C0(v6, v7, v8, v16, v17, v12, 1u64, v13))))
            } else {
                Ok(Adt10::C1)
            }
        }
    }
}

pub fn f41(v0: Adt1, v1: Adt2, v2: Option<Adt2>, v3: Adt2) -> R<Adt10> {
    let m1071 = v2;
    match m1071 {
        None => {
            Ok(Adt10::C1)
        }
        Some(v4) => {
            let v5: Adt6 = Adt6::C0(match v1.clone() {
                Adt2::C0(_, h1072, _, _, _, _, _, _, _, _, _) => {
                    h1072
                }
            }, match v0.clone() {
                Adt1::C0(h1073, _, _, _, _, _, _, _, _, _, _, _) => {
                    h1073
                }
            }, match v0.clone() {
                Adt1::C0(_, _, _, h1074, _, _, _, _, _, _, _, _) => {
                    h1074
                }
            }, match v0.clone() {
                Adt1::C0(_, _, _, _, _, _, h1075, _, _, _, _, _) => {
                    h1075
                }
            });
            let v6: Adt6 = Adt6::C0(match v1.clone() {
                Adt2::C0(_, _, h1076, _, _, _, _, _, _, _, _) => {
                    h1076
                }
            }, match v0.clone() {
                Adt1::C0(_, h1077, _, _, _, _, _, _, _, _, _, _) => {
                    h1077
                }
            }, match v0.clone() {
                Adt1::C0(_, _, _, _, h1078, _, _, _, _, _, _, _) => {
                    h1078
                }
            }, match v0.clone() {
                Adt1::C0(_, _, _, _, _, _, _, h1079, _, _, _, _) => {
                    h1079
                }
            });
            let v7: Adt6 = Adt6::C0(match v1 {
                Adt2::C0(_, _, _, h1080, _, _, _, _, _, _, _) => {
                    h1080
                }
            }, match v0.clone() {
                Adt1::C0(_, _, h1081, _, _, _, _, _, _, _, _, _) => {
                    h1081
                }
            }, match v0.clone() {
                Adt1::C0(_, _, _, _, _, h1082, _, _, _, _, _, _) => {
                    h1082
                }
            }, match v0.clone() {
                Adt1::C0(_, _, _, _, _, _, _, _, h1083, _, _, _) => {
                    h1083
                }
            });
            let v8: u64 = match v0 {
                Adt1::C0(_, _, _, _, _, _, _, _, _, h1084, _, _) => {
                    h1084
                }
            };
            let v9: u64 = f83(v7.clone(), match v4.clone() {
                Adt2::C0(_, _, _, h1085, _, _, _, _, _, _, _) => {
                    h1085
                }
            })?;
            let v10: u64 = match v3.clone() {
                Adt2::C0(h1086, _, _, _, _, _, _, _, _, _, _) => {
                    h1086
                }
            };
            let v11: u64 = {
                let a1094 = v8;
                let a1095 = {
                    let a1092 = match v5.clone() {
                        Adt6::C0(h1087, _, _, _) => {
                            h1087
                        }
                    };
                    let a1093 = {
                        let a1090 = match v6.clone() {
                            Adt6::C0(h1088, _, _, _) => {
                                h1088
                            }
                        };
                        let a1091 = match v7.clone() {
                            Adt6::C0(h1089, _, _, _) => {
                                h1089
                            }
                        };
                        nat_mul(a1090, a1091)?
                    };
                    nat_mul(a1092, a1093)?
                };
                nat_mul(a1094, a1095)?
            };
            let a1097 = match v4.clone() {
                Adt2::C0(h1096, _, _, _, _, _, _, _, _, _, _) => {
                    h1096
                }
            };
            let a1098 = 0u64;
            if if if if if if if if if if if if if nat_eq(a1097, a1098) {
                let a1099 = v10;
                let a1100 = 0u64;
                if nat_eq(a1099, a1100) {
                    true
                } else {
                    let a1101 = v10;
                    let a1102 = 1u64;
                    nat_eq(a1101, a1102)
                }
            } else {
                false
            } {
                let a1103 = 0u64;
                let a1104 = v8;
                nat_lt(a1103, a1104)
            } else {
                false
            } {
                let a1105 = v8;
                let a1106 = 2147483648u64;
                nat_lt(a1105, a1106)
            } else {
                false
            } {
                let a1108 = match v4.clone() {
                    Adt2::C0(_, _, _, _, _, h1107, _, _, _, _, _) => {
                        h1107
                    }
                };
                let a1109 = 4u64;
                nat_eq(a1108, a1109)
            } else {
                false
            } {
                let a1110 = 0u64;
                let a1111 = v9;
                nat_lt(a1110, a1111)
            } else {
                false
            } {
                let a1113 = 0u64;
                let a1114 = match v3.clone() {
                    Adt2::C0(_, _, h1112, _, _, _, _, _, _, _, _) => {
                        h1112
                    }
                };
                nat_lt(a1113, a1114)
            } else {
                false
            } {
                let a1116 = 0u64;
                let a1117 = match v3.clone() {
                    Adt2::C0(_, _, _, h1115, _, _, _, _, _, _, _) => {
                        h1115
                    }
                };
                nat_lt(a1116, a1117)
            } else {
                false
            } {
                let a1119 = match v3.clone() {
                    Adt2::C0(_, h1118, _, _, _, _, _, _, _, _, _) => {
                        h1118
                    }
                };
                let a1120 = v11;
                nat_eq(a1119, a1120)
            } else {
                false
            } {
                let a1123 = match v3.clone() {
                    Adt2::C0(_, _, h1121, _, _, _, _, _, _, _, _) => {
                        h1121
                    }
                };
                let a1124 = f83(v5.clone(), match v4.clone() {
                    Adt2::C0(_, h1122, _, _, _, _, _, _, _, _, _) => {
                        h1122
                    }
                })?;
                nat_eq(a1123, a1124)
            } else {
                false
            } {
                let a1127 = match v3.clone() {
                    Adt2::C0(_, _, _, h1125, _, _, _, _, _, _, _) => {
                        h1125
                    }
                };
                let a1128 = f83(v6.clone(), match v4.clone() {
                    Adt2::C0(_, _, h1126, _, _, _, _, _, _, _, _) => {
                        h1126
                    }
                })?;
                nat_eq(a1127, a1128)
            } else {
                false
            } {
                let a1136 = match v3.clone() {
                    Adt2::C0(_, _, _, _, h1129, _, _, _, _, _, _) => {
                        h1129
                    }
                };
                let a1137 = {
                    let a1134 = v9;
                    let a1135 = {
                        let a1131 = match v4.clone() {
                            Adt2::C0(_, _, _, _, h1130, _, _, _, _, _, _) => {
                                h1130
                            }
                        };
                        let a1132 = v8;
                        let a1133 = 0u64;
                        nat_quot(a1131, a1132, a1133)
                    };
                    nat_mul(a1134, a1135)?
                };
                nat_eq(a1136, a1137)
            } else {
                false
            } {
                f19(v4.clone(), v3.clone())?
            } else {
                false
            } {
                Ok(Adt10::C4(f13(v3.clone(), 1u64)?, {
                    let a1138 = v10;
                    let a1139 = 0u64;
                    if nat_eq(a1138, a1139) {
                        4u64
                    } else {
                        2u64
                    }
                }, f17(v4.clone(), 1u64), f17(v3, 1u64), Adt4::C27(Adt5::C0(v5, v6, v7, match v4.clone() {
                    Adt2::C0(_, _, _, _, _, _, _, _, h1140, _, _) => {
                        h1140
                    }
                }, {
                    let a1142 = v8;
                    let a1143 = match v4 {
                        Adt2::C0(_, _, _, _, _, _, _, _, h1141, _, _) => {
                            h1141
                        }
                    };
                    nat_mul(a1142, a1143)?
                }, 0u64, v9, v10))))
            } else {
                Ok(Adt10::C1)
            }
        }
    }
}

pub fn f42(v0: Adt2, v1: Option<Adt2>, v2: Adt2) -> R<Adt10> {
    let m1144 = v1;
    match m1144 {
        None => {
            Ok(Adt10::C1)
        }
        Some(v3) => {
            f101(v0, v3, v2)
        }
    }
}

pub fn f43(v0: Adt2, v1: Adt2) -> R<Adt10> {
    let a1146 = match v0.clone() {
        Adt2::C0(h1145, _, _, _, _, _, _, _, _, _, _) => {
            h1145
        }
    };
    let a1147 = 1u64;
    if if nat_eq(a1146, a1147) {
        true
    } else {
        let a1149 = match v0.clone() {
            Adt2::C0(h1148, _, _, _, _, _, _, _, _, _, _) => {
                h1148
            }
        };
        let a1150 = 30u64;
        nat_eq(a1149, a1150)
    } {
        let a1152 = match v1.clone() {
            Adt2::C0(h1151, _, _, _, _, _, _, _, _, _, _) => {
                h1151
            }
        };
        let a1153 = 1u64;
        if if if if if if if if if if if nat_eq(a1152, a1153) {
            let a1156 = match v0.clone() {
                Adt2::C0(_, h1154, _, _, _, _, _, _, _, _, _) => {
                    h1154
                }
            };
            let a1157 = match v1.clone() {
                Adt2::C0(_, h1155, _, _, _, _, _, _, _, _, _) => {
                    h1155
                }
            };
            nat_eq(a1156, a1157)
        } else {
            false
        } {
            let a1159 = 0u64;
            let a1160 = match v1.clone() {
                Adt2::C0(_, _, h1158, _, _, _, _, _, _, _, _) => {
                    h1158
                }
            };
            nat_lt(a1159, a1160)
        } else {
            false
        } {
            let a1163 = match v1.clone() {
                Adt2::C0(_, _, h1161, _, _, _, _, _, _, _, _) => {
                    h1161
                }
            };
            let a1164 = match v1.clone() {
                Adt2::C0(_, _, _, h1162, _, _, _, _, _, _, _) => {
                    h1162
                }
            };
            nat_eq(a1163, a1164)
        } else {
            false
        } {
            let a1166 = match v1.clone() {
                Adt2::C0(_, _, _, _, h1165, _, _, _, _, _, _) => {
                    h1165
                }
            };
            let a1167 = 1u64;
            nat_eq(a1166, a1167)
        } else {
            false
        } {
            let a1175 = match v0.clone() {
                Adt2::C0(_, _, h1168, _, _, _, _, _, _, _, _) => {
                    h1168
                }
            };
            let a1176 = {
                let a1173 = {
                    let a1171 = match v1.clone() {
                        Adt2::C0(_, _, h1169, _, _, _, _, _, _, _, _) => {
                            h1169
                        }
                    };
                    let a1172 = match v1.clone() {
                        Adt2::C0(_, _, h1170, _, _, _, _, _, _, _, _) => {
                            h1170
                        }
                    };
                    nat_add(a1171, a1172)?
                };
                let a1174 = 1u64;
                nat_sub(a1173, a1174)
            };
            nat_eq(a1175, a1176)
        } else {
            false
        } {
            let a1178 = 0u64;
            let a1179 = match v0.clone() {
                Adt2::C0(_, _, _, h1177, _, _, _, _, _, _, _) => {
                    h1177
                }
            };
            nat_lt(a1178, a1179)
        } else {
            false
        } {
            let a1181 = 0u64;
            let a1182 = match v0.clone() {
                Adt2::C0(_, _, _, _, h1180, _, _, _, _, _, _) => {
                    h1180
                }
            };
            nat_lt(a1181, a1182)
        } else {
            false
        } {
            let a1184 = 2u64;
            let a1185 = match v0.clone() {
                Adt2::C0(_, _, _, _, _, h1183, _, _, _, _, _) => {
                    h1183
                }
            };
            nat_le(a1184, a1185)
        } else {
            false
        } {
            let a1187 = 2u64;
            let a1188 = match v1.clone() {
                Adt2::C0(_, _, _, _, _, h1186, _, _, _, _, _) => {
                    h1186
                }
            };
            nat_le(a1187, a1188)
        } else {
            false
        } {
            f19(v0.clone(), v1.clone())?
        } else {
            false
        } {
            Ok(Adt10::C4(f13(v1.clone(), 1u64)?, 2u64, f17(v0, 1u64), f17(v1, 1u64), Adt4::C21))
        } else {
            Ok(Adt10::C1)
        }
    } else {
        Ok(Adt10::C0)
    }
}

pub fn f44(v0: Adt2, v1: Option<Adt2>, v2: Adt2) -> R<Adt10> {
    let m1189 = v1;
    match m1189 {
        None => {
            Ok(Adt10::C1)
        }
        Some(v3) => {
            f102(v0, v3, v2)
        }
    }
}

pub fn f45(v0: u64) -> bool {
    let a1190 = v0;
    let a1191 = 1u64;
    if if nat_eq(a1190, a1191) {
        true
    } else {
        let a1192 = v0;
        let a1193 = 33u64;
        nat_eq(a1192, a1193)
    } {
        true
    } else {
        let a1194 = v0;
        let a1195 = 34u64;
        nat_eq(a1194, a1195)
    }
}

pub fn f46(v0: u64, v1: Adt2, v2: bool, v3: Adt2) -> R<Adt10> {
    let a1198 = match v1.clone() {
        Adt2::C0(h1196, _, _, _, _, _, _, _, _, _, _) => {
            h1196
        }
    };
    let a1199 = match v3.clone() {
        Adt2::C0(h1197, _, _, _, _, _, _, _, _, _, _) => {
            h1197
        }
    };
    if nat_eq(a1198, a1199) {
        f103(v0, v1, v2, v3)
    } else {
        if if if if f95(match v1.clone() {
            Adt2::C0(h1200, _, _, _, _, _, _, _, _, _, _) => {
                h1200
            }
        }) {
            f95(match v3.clone() {
                Adt2::C0(h1201, _, _, _, _, _, _, _, _, _, _) => {
                    h1201
                }
            })
        } else {
            false
        } {
            let a1202 = v0;
            let a1203 = 33u64;
            if nat_eq(a1202, a1203) {
                v2
            } else {
                true
            }
        } else {
            false
        } {
            let a1204 = f13(v1.clone(), 1u64)?;
            let a1205 = f13(v3.clone(), 1u64)?;
            nat_eq(a1204, a1205)
        } else {
            false
        } {
            let m1207 = f104(match v3.clone() {
                Adt2::C0(h1206, _, _, _, _, _, _, _, _, _, _) => {
                    h1206
                }
            });
            match m1207 {
                Adt16::C0 => {
                    Ok(Adt10::C0)
                }
                Adt16::C1(v4) => {
                    Ok(Adt10::C4(f13(v3.clone(), 1u64)?, match v4 {
                        Adt17::C0(_, _, h1208) => {
                            h1208
                        }
                    }, f17(v1.clone(), 1u64), f17(v3.clone(), 1u64), Adt4::C11(match v1 {
                        Adt2::C0(h1209, _, _, _, _, _, _, _, _, _, _) => {
                            h1209
                        }
                    }, match v3 {
                        Adt2::C0(h1210, _, _, _, _, _, _, _, _, _, _) => {
                            h1210
                        }
                    })))
                }
            }
        } else {
            Ok(Adt10::C1)
        }
    }
}

pub fn f47(v0: Adt2, v1: Adt2) -> R<Adt10> {
    let a1212 = match v0.clone() {
        Adt2::C0(h1211, _, _, _, _, _, _, _, _, _, _) => {
            h1211
        }
    };
    let a1213 = 0u64;
    if if if if nat_eq(a1212, a1213) {
        true
    } else {
        let a1215 = match v0.clone() {
            Adt2::C0(h1214, _, _, _, _, _, _, _, _, _, _) => {
                h1214
            }
        };
        let a1216 = 1u64;
        nat_eq(a1215, a1216)
    } {
        true
    } else {
        let a1218 = match v0.clone() {
            Adt2::C0(h1217, _, _, _, _, _, _, _, _, _, _) => {
                h1217
            }
        };
        let a1219 = 25u64;
        if nat_eq(a1218, a1219) {
            true
        } else {
            let a1221 = match v0.clone() {
                Adt2::C0(h1220, _, _, _, _, _, _, _, _, _, _) => {
                    h1220
                }
            };
            let a1222 = 26u64;
            nat_eq(a1221, a1222)
        }
    } {
        true
    } else {
        let a1224 = match v0.clone() {
            Adt2::C0(h1223, _, _, _, _, _, _, _, _, _, _) => {
                h1223
            }
        };
        let a1225 = 30u64;
        nat_eq(a1224, a1225)
    } {
        let a1228 = match v0.clone() {
            Adt2::C0(h1226, _, _, _, _, _, _, _, _, _, _) => {
                h1226
            }
        };
        let a1229 = match v1.clone() {
            Adt2::C0(h1227, _, _, _, _, _, _, _, _, _, _) => {
                h1227
            }
        };
        if if nat_eq(a1228, a1229) {
            f105(match v1.clone() {
                Adt2::C0(_, h1230, _, _, _, _, _, _, _, _, _) => {
                    h1230
                }
            }, match v1.clone() {
                Adt2::C0(_, _, h1231, _, _, _, _, _, _, _, _) => {
                    h1231
                }
            }, match v1.clone() {
                Adt2::C0(_, _, _, h1232, _, _, _, _, _, _, _) => {
                    h1232
                }
            }, match v1.clone() {
                Adt2::C0(_, _, _, _, h1233, _, _, _, _, _, _) => {
                    h1233
                }
            }, match v0.clone() {
                Adt2::C0(_, h1234, _, _, _, _, _, _, _, _, _) => {
                    h1234
                }
            }, match v0.clone() {
                Adt2::C0(_, _, h1235, _, _, _, _, _, _, _, _) => {
                    h1235
                }
            }, match v0.clone() {
                Adt2::C0(_, _, _, h1236, _, _, _, _, _, _, _) => {
                    h1236
                }
            }, match v0.clone() {
                Adt2::C0(_, _, _, _, h1237, _, _, _, _, _, _) => {
                    h1237
                }
            })
        } else {
            false
        } {
            let m1239 = f104(match v0.clone() {
                Adt2::C0(h1238, _, _, _, _, _, _, _, _, _, _) => {
                    h1238
                }
            });
            match m1239 {
                Adt16::C0 => {
                    Ok(Adt10::C0)
                }
                Adt16::C1(v2) => {
                    Ok(Adt10::C4(f13(v1.clone(), 1u64)?, match v2 {
                        Adt17::C0(_, _, h1240) => {
                            h1240
                        }
                    }, f17(v0, 1u64), f17(v1, 1u64), Adt4::C1))
                }
            }
        } else {
            Ok(Adt10::C1)
        }
    } else {
        Ok(Adt10::C0)
    }
}

pub fn f48(v0: u64, v1: Adt2, v2: Option<Adt2>, v3: Adt2) -> R<Adt10> {
    let m1241 = v2;
    match m1241 {
        None => {
            Ok(Adt10::C1)
        }
        Some(v4) => {
            let a1242 = v0;
            let a1243 = 4u64;
            if if if nat_lt(a1242, a1243) {
                let a1246 = match v1.clone() {
                    Adt2::C0(h1244, _, _, _, _, _, _, _, _, _, _) => {
                        h1244
                    }
                };
                let a1247 = match v4.clone() {
                    Adt2::C0(h1245, _, _, _, _, _, _, _, _, _, _) => {
                        h1245
                    }
                };
                if nat_eq(a1246, a1247) {
                    let a1250 = match v1.clone() {
                        Adt2::C0(h1248, _, _, _, _, _, _, _, _, _, _) => {
                            h1248
                        }
                    };
                    let a1251 = match v3.clone() {
                        Adt2::C0(h1249, _, _, _, _, _, _, _, _, _, _) => {
                            h1249
                        }
                    };
                    nat_eq(a1250, a1251)
                } else {
                    false
                }
            } else {
                false
            } {
                if if if f106(v0, 0u64, match v1.clone() {
                    Adt2::C0(_, h1252, _, _, _, _, _, _, _, _, _) => {
                        h1252
                    }
                }, match v4.clone() {
                    Adt2::C0(_, h1253, _, _, _, _, _, _, _, _, _) => {
                        h1253
                    }
                }, match v3.clone() {
                    Adt2::C0(_, h1254, _, _, _, _, _, _, _, _, _) => {
                        h1254
                    }
                })? {
                    f106(v0, 1u64, match v1.clone() {
                        Adt2::C0(_, _, h1255, _, _, _, _, _, _, _, _) => {
                            h1255
                        }
                    }, match v4.clone() {
                        Adt2::C0(_, _, h1256, _, _, _, _, _, _, _, _) => {
                            h1256
                        }
                    }, match v3.clone() {
                        Adt2::C0(_, _, h1257, _, _, _, _, _, _, _, _) => {
                            h1257
                        }
                    })?
                } else {
                    false
                } {
                    f106(v0, 2u64, match v1.clone() {
                        Adt2::C0(_, _, _, h1258, _, _, _, _, _, _, _) => {
                            h1258
                        }
                    }, match v4.clone() {
                        Adt2::C0(_, _, _, h1259, _, _, _, _, _, _, _) => {
                            h1259
                        }
                    }, match v3.clone() {
                        Adt2::C0(_, _, _, h1260, _, _, _, _, _, _, _) => {
                            h1260
                        }
                    })?
                } else {
                    false
                } {
                    f106(v0, 3u64, match v1.clone() {
                        Adt2::C0(_, _, _, _, h1261, _, _, _, _, _, _) => {
                            h1261
                        }
                    }, match v4.clone() {
                        Adt2::C0(_, _, _, _, h1262, _, _, _, _, _, _) => {
                            h1262
                        }
                    }, match v3.clone() {
                        Adt2::C0(_, _, _, _, h1263, _, _, _, _, _, _) => {
                            h1263
                        }
                    })?
                } else {
                    false
                }
            } else {
                false
            } {
                let m1265 = f104(match v1.clone() {
                    Adt2::C0(h1264, _, _, _, _, _, _, _, _, _, _) => {
                        h1264
                    }
                });
                match m1265 {
                    Adt16::C0 => {
                        Ok(Adt10::C0)
                    }
                    Adt16::C1(v5) => {
                        let v6: u64 = match v5.clone() {
                            Adt17::C0(_, h1266, _) => {
                                h1266
                            }
                        };
                        let a1271 = {
                            let a1268 = match v1.clone() {
                                Adt2::C0(_, h1267, _, _, _, _, _, _, _, _, _) => {
                                    h1267
                                }
                            };
                            let a1269 = v6;
                            let a1270 = 1u64;
                            nat_rem(a1268, a1269, a1270)
                        };
                        let a1272 = 0u64;
                        if if if nat_eq(a1271, a1272) {
                            let a1277 = {
                                let a1274 = match v4.clone() {
                                    Adt2::C0(_, h1273, _, _, _, _, _, _, _, _, _) => {
                                        h1273
                                    }
                                };
                                let a1275 = v6;
                                let a1276 = 1u64;
                                nat_rem(a1274, a1275, a1276)
                            };
                            let a1278 = 0u64;
                            nat_eq(a1277, a1278)
                        } else {
                            false
                        } {
                            let a1283 = {
                                let a1280 = match v3.clone() {
                                    Adt2::C0(_, h1279, _, _, _, _, _, _, _, _, _) => {
                                        h1279
                                    }
                                };
                                let a1281 = v6;
                                let a1282 = 1u64;
                                nat_rem(a1280, a1281, a1282)
                            };
                            let a1284 = 0u64;
                            nat_eq(a1283, a1284)
                        } else {
                            false
                        } {
                            Ok(Adt10::C4(f13(v3.clone(), v6)?, match v5 {
                                Adt17::C0(_, _, h1285) => {
                                    h1285
                                }
                            }, f17(v1, v6), f17(v3, v6), Adt4::C2(v0, f17(v4, v6))))
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

pub fn f49(v0: u64, v1: Adt1, v2: Adt2, v3: Adt2) -> R<Adt10> {
    let a1287 = match v2.clone() {
        Adt2::C0(h1286, _, _, _, _, _, _, _, _, _, _) => {
            h1286
        }
    };
    let a1288 = 0u64;
    if if nat_eq(a1287, a1288) {
        let a1290 = match v3.clone() {
            Adt2::C0(h1289, _, _, _, _, _, _, _, _, _, _) => {
                h1289
            }
        };
        let a1291 = 0u64;
        nat_eq(a1290, a1291)
    } else {
        false
    } {
        let a1292 = v0;
        let a1293 = 61u64;
        if nat_eq(a1292, a1293) {
            let v4: bool = {
                let a1295 = 0u64;
                let a1296 = match v1.clone() {
                    Adt1::C0(_, _, _, _, _, _, _, _, h1294, _, _, _) => {
                        h1294
                    }
                };
                nat_lt(a1295, a1296)
            };
            if if if if if if f107(match v2.clone() {
                Adt2::C0(_, h1297, _, _, _, _, _, _, _, _, _) => {
                    h1297
                }
            }, match v1.clone() {
                Adt1::C0(h1298, _, _, _, _, _, _, _, _, _, _, _) => {
                    h1298
                }
            }, match v1.clone() {
                Adt1::C0(_, h1299, _, _, _, _, _, _, _, _, _, _) => {
                    h1299
                }
            }, match v3.clone() {
                Adt2::C0(_, h1300, _, _, _, _, _, _, _, _, _) => {
                    h1300
                }
            })? {
                f107(match v2.clone() {
                    Adt2::C0(_, _, h1301, _, _, _, _, _, _, _, _) => {
                        h1301
                    }
                }, match v1.clone() {
                    Adt1::C0(_, _, h1302, _, _, _, _, _, _, _, _, _) => {
                        h1302
                    }
                }, match v1.clone() {
                    Adt1::C0(_, _, _, h1303, _, _, _, _, _, _, _, _) => {
                        h1303
                    }
                }, match v3.clone() {
                    Adt2::C0(_, _, h1304, _, _, _, _, _, _, _, _) => {
                        h1304
                    }
                })?
            } else {
                false
            } {
                f107(match v2.clone() {
                    Adt2::C0(_, _, _, h1305, _, _, _, _, _, _, _) => {
                        h1305
                    }
                }, match v1.clone() {
                    Adt1::C0(_, _, _, _, h1306, _, _, _, _, _, _, _) => {
                        h1306
                    }
                }, match v1.clone() {
                    Adt1::C0(_, _, _, _, _, h1307, _, _, _, _, _, _) => {
                        h1307
                    }
                }, match v3.clone() {
                    Adt2::C0(_, _, _, h1308, _, _, _, _, _, _, _) => {
                        h1308
                    }
                })?
            } else {
                false
            } {
                f107(match v2.clone() {
                    Adt2::C0(_, _, _, _, h1309, _, _, _, _, _, _) => {
                        h1309
                    }
                }, match v1.clone() {
                    Adt1::C0(_, _, _, _, _, _, h1310, _, _, _, _, _) => {
                        h1310
                    }
                }, match v1.clone() {
                    Adt1::C0(_, _, _, _, _, _, _, h1311, _, _, _, _) => {
                        h1311
                    }
                }, match v3.clone() {
                    Adt2::C0(_, _, _, _, h1312, _, _, _, _, _, _) => {
                        h1312
                    }
                })?
            } else {
                false
            } {
                let a1314 = match v1.clone() {
                    Adt1::C0(_, _, _, _, _, _, _, _, h1313, _, _, _) => {
                        h1313
                    }
                };
                let a1315 = 4294967295u64;
                nat_le(a1314, a1315)
            } else {
                false
            } {
                if v4 {
                    !f30(match v2.clone() {
                        Adt2::C0(_, h1316, _, _, _, _, _, _, _, _, _) => {
                            h1316
                        }
                    }, match v2.clone() {
                        Adt2::C0(_, _, h1317, _, _, _, _, _, _, _, _) => {
                            h1317
                        }
                    }, match v2.clone() {
                        Adt2::C0(_, _, _, h1318, _, _, _, _, _, _, _) => {
                            h1318
                        }
                    }, match v2.clone() {
                        Adt2::C0(_, _, _, _, h1319, _, _, _, _, _, _) => {
                            h1319
                        }
                    })
                } else {
                    true
                }
            } else {
                false
            } {
                Ok(Adt10::C4(f13(v3.clone(), 1u64)?, 4u64, f17(v2, 1u64), f17(v3, 1u64), Adt4::C3(match v1.clone() {
                    Adt1::C0(h1320, _, _, _, _, _, _, _, _, _, _, _) => {
                        h1320
                    }
                }, match v1.clone() {
                    Adt1::C0(_, _, h1321, _, _, _, _, _, _, _, _, _) => {
                        h1321
                    }
                }, match v1.clone() {
                    Adt1::C0(_, _, _, _, h1322, _, _, _, _, _, _, _) => {
                        h1322
                    }
                }, match v1 {
                    Adt1::C0(_, _, _, _, _, _, h1323, _, _, _, _, _) => {
                        h1323
                    }
                }, v4)))
            } else {
                Ok(Adt10::C1)
            }
        } else {
            let a1324 = v0;
            let a1325 = 62u64;
            if nat_eq(a1324, a1325) {
                let a1327 = match v1.clone() {
                    Adt1::C0(h1326, _, _, _, _, _, _, _, _, _, _, _) => {
                        h1326
                    }
                };
                let a1328 = 2147483648u64;
                if if if if if nat_lt(a1327, a1328) {
                    let a1330 = match v1.clone() {
                        Adt1::C0(_, h1329, _, _, _, _, _, _, _, _, _, _) => {
                            h1329
                        }
                    };
                    let a1331 = 2147483648u64;
                    nat_lt(a1330, a1331)
                } else {
                    false
                } {
                    let a1334 = match v1.clone() {
                        Adt1::C0(h1332, _, _, _, _, _, _, _, _, _, _, _) => {
                            h1332
                        }
                    };
                    let a1335 = match v2.clone() {
                        Adt2::C0(_, h1333, _, _, _, _, _, _, _, _, _) => {
                            h1333
                        }
                    };
                    nat_lt(a1334, a1335)
                } else {
                    false
                } {
                    let a1338 = match v1.clone() {
                        Adt1::C0(_, h1336, _, _, _, _, _, _, _, _, _, _) => {
                            h1336
                        }
                    };
                    let a1339 = match v2.clone() {
                        Adt2::C0(_, h1337, _, _, _, _, _, _, _, _, _) => {
                            h1337
                        }
                    };
                    nat_lt(a1338, a1339)
                } else {
                    false
                } {
                    let a1342 = match v2.clone() {
                        Adt2::C0(_, _, h1340, _, _, _, _, _, _, _, _) => {
                            h1340
                        }
                    };
                    let a1343 = match v3.clone() {
                        Adt2::C0(_, _, h1341, _, _, _, _, _, _, _, _) => {
                            h1341
                        }
                    };
                    if if if nat_eq(a1342, a1343) {
                        let a1346 = match v2.clone() {
                            Adt2::C0(_, _, _, h1344, _, _, _, _, _, _, _) => {
                                h1344
                            }
                        };
                        let a1347 = match v3.clone() {
                            Adt2::C0(_, _, _, h1345, _, _, _, _, _, _, _) => {
                                h1345
                            }
                        };
                        nat_eq(a1346, a1347)
                    } else {
                        false
                    } {
                        let a1350 = match v2.clone() {
                            Adt2::C0(_, _, _, _, h1348, _, _, _, _, _, _) => {
                                h1348
                            }
                        };
                        let a1351 = match v3.clone() {
                            Adt2::C0(_, _, _, _, h1349, _, _, _, _, _, _) => {
                                h1349
                            }
                        };
                        nat_eq(a1350, a1351)
                    } else {
                        false
                    } {
                        let a1360 = match v3.clone() {
                            Adt2::C0(_, h1352, _, _, _, _, _, _, _, _, _) => {
                                h1352
                            }
                        };
                        let a1361 = {
                            let a1358 = {
                                let a1355 = match v2.clone() {
                                    Adt2::C0(_, h1353, _, _, _, _, _, _, _, _, _) => {
                                        h1353
                                    }
                                };
                                let a1356 = match v1.clone() {
                                    Adt1::C0(h1354, _, _, _, _, _, _, _, _, _, _, _) => {
                                        h1354
                                    }
                                };
                                nat_add(a1355, a1356)?
                            };
                            let a1359 = match v1.clone() {
                                Adt1::C0(_, h1357, _, _, _, _, _, _, _, _, _, _) => {
                                    h1357
                                }
                            };
                            nat_add(a1358, a1359)?
                        };
                        nat_eq(a1360, a1361)
                    } else {
                        false
                    }
                } else {
                    false
                } {
                    Ok(Adt10::C4(f13(v3.clone(), 1u64)?, 4u64, f17(v2, 1u64), f17(v3, 1u64), Adt4::C4(match v1 {
                        Adt1::C0(h1362, _, _, _, _, _, _, _, _, _, _, _) => {
                            h1362
                        }
                    })))
                } else {
                    Ok(Adt10::C1)
                }
            } else {
                let a1363 = v0;
                let a1364 = 63u64;
                if nat_eq(a1363, a1364) {
                    if if if if if f108(match v2.clone() {
                        Adt2::C0(_, h1365, _, _, _, _, _, _, _, _, _) => {
                            h1365
                        }
                    }, match v1.clone() {
                        Adt1::C0(h1366, _, _, _, _, _, _, _, _, _, _, _) => {
                            h1366
                        }
                    })? {
                        f108(match v2.clone() {
                            Adt2::C0(_, _, h1367, _, _, _, _, _, _, _, _) => {
                                h1367
                            }
                        }, match v1.clone() {
                            Adt1::C0(_, h1368, _, _, _, _, _, _, _, _, _, _) => {
                                h1368
                            }
                        })?
                    } else {
                        false
                    } {
                        f108(match v2.clone() {
                            Adt2::C0(_, _, _, h1369, _, _, _, _, _, _, _) => {
                                h1369
                            }
                        }, match v1.clone() {
                            Adt1::C0(_, _, h1370, _, _, _, _, _, _, _, _, _) => {
                                h1370
                            }
                        })?
                    } else {
                        false
                    } {
                        f108(match v2.clone() {
                            Adt2::C0(_, _, _, _, h1371, _, _, _, _, _, _) => {
                                h1371
                            }
                        }, match v1.clone() {
                            Adt1::C0(_, _, _, h1372, _, _, _, _, _, _, _, _) => {
                                h1372
                            }
                        })?
                    } else {
                        false
                    } {
                        let a1375 = match v2.clone() {
                            Adt2::C0(_, h1373, _, _, _, _, _, _, _, _, _) => {
                                h1373
                            }
                        };
                        let a1376 = match v3.clone() {
                            Adt2::C0(_, h1374, _, _, _, _, _, _, _, _, _) => {
                                h1374
                            }
                        };
                        if if if nat_eq(a1375, a1376) {
                            let a1379 = match v2.clone() {
                                Adt2::C0(_, _, h1377, _, _, _, _, _, _, _, _) => {
                                    h1377
                                }
                            };
                            let a1380 = match v3.clone() {
                                Adt2::C0(_, _, h1378, _, _, _, _, _, _, _, _) => {
                                    h1378
                                }
                            };
                            nat_eq(a1379, a1380)
                        } else {
                            false
                        } {
                            let a1383 = match v2.clone() {
                                Adt2::C0(_, _, _, h1381, _, _, _, _, _, _, _) => {
                                    h1381
                                }
                            };
                            let a1384 = match v3.clone() {
                                Adt2::C0(_, _, _, h1382, _, _, _, _, _, _, _) => {
                                    h1382
                                }
                            };
                            nat_eq(a1383, a1384)
                        } else {
                            false
                        } {
                            let a1387 = match v2.clone() {
                                Adt2::C0(_, _, _, _, h1385, _, _, _, _, _, _) => {
                                    h1385
                                }
                            };
                            let a1388 = match v3.clone() {
                                Adt2::C0(_, _, _, _, h1386, _, _, _, _, _, _) => {
                                    h1386
                                }
                            };
                            nat_eq(a1387, a1388)
                        } else {
                            false
                        }
                    } else {
                        false
                    } {
                        Ok(Adt10::C4(f13(v3.clone(), 1u64)?, 4u64, f17(v2, 1u64), f17(v3, 1u64), Adt4::C5(match v1.clone() {
                            Adt1::C0(h1389, _, _, _, _, _, _, _, _, _, _, _) => {
                                h1389
                            }
                        }, match v1.clone() {
                            Adt1::C0(_, h1390, _, _, _, _, _, _, _, _, _, _) => {
                                h1390
                            }
                        }, match v1.clone() {
                            Adt1::C0(_, _, h1391, _, _, _, _, _, _, _, _, _) => {
                                h1391
                            }
                        }, match v1 {
                            Adt1::C0(_, _, _, h1392, _, _, _, _, _, _, _, _) => {
                                h1392
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

pub fn f50(v0: u64, v1: Adt1, v2: Adt2, v3: Option<Adt2>, v4: Adt2) -> R<Adt10> {
    let m1393 = v3;
    match m1393 {
        None => {
            Ok(Adt10::C1)
        }
        Some(v5) => {
            let v6: bool = {
                let a1394 = v0;
                let a1395 = 4u64;
                nat_eq(a1394, a1395)
            };
            let v7: bool = {
                let a1397 = 0u64;
                let a1398 = match v1.clone() {
                    Adt1::C0(_, _, _, _, h1396, _, _, _, _, _, _, _) => {
                        h1396
                    }
                };
                nat_lt(a1397, a1398)
            };
            let v8: u64 = f13(v5.clone(), 1u64)?;
            let v9: Adt3 = Adt3::C0(match v5.clone() {
                Adt2::C0(_, h1399, _, _, _, _, _, _, _, _, _) => {
                    h1399
                }
            }, match v5.clone() {
                Adt2::C0(_, _, h1400, _, _, _, _, _, _, _, _) => {
                    h1400
                }
            }, match v5.clone() {
                Adt2::C0(_, _, _, h1401, _, _, _, _, _, _, _) => {
                    h1401
                }
            }, match v5.clone() {
                Adt2::C0(_, _, _, _, h1402, _, _, _, _, _, _) => {
                    h1402
                }
            }, 4u64, match v1.clone() {
                Adt1::C0(h1403, _, _, _, _, _, _, _, _, _, _, _) => {
                    h1403
                }
            }, match v1.clone() {
                Adt1::C0(_, h1404, _, _, _, _, _, _, _, _, _, _) => {
                    h1404
                }
            }, match v1.clone() {
                Adt1::C0(_, _, h1405, _, _, _, _, _, _, _, _, _) => {
                    h1405
                }
            });
            if if if if if if if if if if if if v6 {
                let a1407 = match v2.clone() {
                    Adt2::C0(h1406, _, _, _, _, _, _, _, _, _, _) => {
                        h1406
                    }
                };
                let a1408 = 0u64;
                if nat_eq(a1407, a1408) {
                    let a1410 = match v5.clone() {
                        Adt2::C0(h1409, _, _, _, _, _, _, _, _, _, _) => {
                            h1409
                        }
                    };
                    let a1411 = 0u64;
                    nat_eq(a1410, a1411)
                } else {
                    false
                }
            } else {
                let a1413 = match v2.clone() {
                    Adt2::C0(h1412, _, _, _, _, _, _, _, _, _, _) => {
                        h1412
                    }
                };
                let a1414 = 0u64;
                if if nat_eq(a1413, a1414) {
                    true
                } else {
                    let a1416 = match v2.clone() {
                        Adt2::C0(h1415, _, _, _, _, _, _, _, _, _, _) => {
                            h1415
                        }
                    };
                    let a1417 = 26u64;
                    nat_eq(a1416, a1417)
                } {
                    let a1419 = match v5.clone() {
                        Adt2::C0(h1418, _, _, _, _, _, _, _, _, _, _) => {
                            h1418
                        }
                    };
                    let a1420 = 0u64;
                    if nat_eq(a1419, a1420) {
                        true
                    } else {
                        let a1422 = match v5.clone() {
                            Adt2::C0(h1421, _, _, _, _, _, _, _, _, _, _) => {
                                h1421
                            }
                        };
                        let a1423 = 26u64;
                        nat_eq(a1422, a1423)
                    }
                } else {
                    false
                }
            } {
                let a1426 = match v2.clone() {
                    Adt2::C0(h1424, _, _, _, _, _, _, _, _, _, _) => {
                        h1424
                    }
                };
                let a1427 = match v4.clone() {
                    Adt2::C0(h1425, _, _, _, _, _, _, _, _, _, _) => {
                        h1425
                    }
                };
                nat_eq(a1426, a1427)
            } else {
                false
            } {
                f20(v2.clone(), v4.clone())
            } else {
                false
            } {
                f21(v2.clone())?
            } else {
                false
            } {
                f21(v4.clone())?
            } else {
                false
            } {
                let a1429 = match v5.clone() {
                    Adt2::C0(_, _, _, _, _, h1428, _, _, _, _, _) => {
                        h1428
                    }
                };
                let a1430 = 4u64;
                nat_eq(a1429, a1430)
            } else {
                false
            } {
                let a1431 = v8;
                let a1432 = f13(v2.clone(), 1u64)?;
                nat_le(a1431, a1432)
            } else {
                false
            } {
                let a1434 = match v1.clone() {
                    Adt1::C0(_, _, _, h1433, _, _, _, _, _, _, _, _) => {
                        h1433
                    }
                };
                let a1435 = 1073741824u64;
                nat_lt(a1434, a1435)
            } else {
                false
            } {
                let a1437 = match v1.clone() {
                    Adt1::C0(h1436, _, _, _, _, _, _, _, _, _, _, _) => {
                        h1436
                    }
                };
                let a1438 = 4294967295u64;
                if if if if nat_le(a1437, a1438) {
                    let a1440 = match v1.clone() {
                        Adt1::C0(_, h1439, _, _, _, _, _, _, _, _, _, _) => {
                            h1439
                        }
                    };
                    let a1441 = 4294967295u64;
                    nat_le(a1440, a1441)
                } else {
                    false
                } {
                    let a1443 = match v1.clone() {
                        Adt1::C0(_, _, h1442, _, _, _, _, _, _, _, _, _) => {
                            h1442
                        }
                    };
                    let a1444 = 4294967295u64;
                    nat_le(a1443, a1444)
                } else {
                    false
                } {
                    let a1446 = match v1.clone() {
                        Adt1::C0(_, _, _, h1445, _, _, _, _, _, _, _, _) => {
                            h1445
                        }
                    };
                    let a1447 = 4294967295u64;
                    nat_le(a1446, a1447)
                } else {
                    false
                } {
                    let a1449 = match v1.clone() {
                        Adt1::C0(_, _, _, _, h1448, _, _, _, _, _, _, _) => {
                            h1448
                        }
                    };
                    let a1450 = 4294967295u64;
                    nat_le(a1449, a1450)
                } else {
                    false
                }
            } else {
                false
            } {
                if v7 {
                    let a1453 = match v2.clone() {
                        Adt2::C0(_, _, _, _, _, _, _, _, _, _, h1451) => {
                            h1451
                        }
                    };
                    let a1454 = match v4.clone() {
                        Adt2::C0(_, _, _, _, _, _, _, _, _, _, h1452) => {
                            h1452
                        }
                    };
                    nat_eq(a1453, a1454)
                } else {
                    true
                }
            } else {
                false
            } {
                f109(v9.clone(), match v1.clone() {
                    Adt1::C0(_, _, _, h1455, _, _, _, _, _, _, _, _) => {
                        h1455
                    }
                }, match v4 {
                    Adt2::C0(_, _, _, _, _, _, _, _, _, h1456, _) => {
                        h1456
                    }
                }, v8, v6)?
            } else {
                false
            } {
                let v10: u64 = if v7 {
                    0u64
                } else {
                    f13(v2, 1u64)?
                };
                Ok(Adt10::C4({
                    let a1457 = v10;
                    let a1458 = v8;
                    nat_add(a1457, a1458)?
                }, 4u64, f17(v5, 1u64), v9, Adt4::C6(v10, match v1 {
                    Adt1::C0(_, _, _, h1459, _, _, _, _, _, _, _, _) => {
                        h1459
                    }
                }, v6)))
            } else {
                Ok(Adt10::C1)
            }
        }
    }
}

pub fn f51(v0: Adt2, v1: Adt2) -> R<Adt10> {
    let a1461 = match v0.clone() {
        Adt2::C0(h1460, _, _, _, _, _, _, _, _, _, _) => {
            h1460
        }
    };
    let a1462 = 0u64;
    if if if nat_eq(a1461, a1462) {
        let a1464 = match v1.clone() {
            Adt2::C0(h1463, _, _, _, _, _, _, _, _, _, _) => {
                h1463
            }
        };
        let a1465 = 0u64;
        nat_eq(a1464, a1465)
    } else {
        false
    } {
        f105(match v0.clone() {
            Adt2::C0(_, h1466, _, _, _, _, _, _, _, _, _) => {
                h1466
            }
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
        }, match v1.clone() {
            Adt2::C0(_, h1470, _, _, _, _, _, _, _, _, _) => {
                h1470
            }
        }, match v1.clone() {
            Adt2::C0(_, _, h1471, _, _, _, _, _, _, _, _) => {
                h1471
            }
        }, match v1.clone() {
            Adt2::C0(_, _, _, h1472, _, _, _, _, _, _, _) => {
                h1472
            }
        }, match v1.clone() {
            Adt2::C0(_, _, _, _, h1473, _, _, _, _, _, _) => {
                h1473
            }
        })
    } else {
        false
    } {
        let v2: u64 = f13(v1.clone(), 1u64)?;
        Ok(Adt10::C4({
            let a1474 = v2;
            let a1475 = f13(v0.clone(), 1u64)?;
            nat_add(a1474, a1475)?
        }, 4u64, f17(v0, 1u64), f17(v1, 1u64), Adt4::C7(v2)))
    } else {
        Ok(Adt10::C1)
    }
}

pub fn f52(v0: u64, v1: Adt2, v2: Adt2) -> R<Adt10> {
    let a1477 = match v1.clone() {
        Adt2::C0(h1476, _, _, _, _, _, _, _, _, _, _) => {
            h1476
        }
    };
    let a1478 = 0u64;
    if if if nat_eq(a1477, a1478) {
        true
    } else {
        let a1480 = match v1.clone() {
            Adt2::C0(h1479, _, _, _, _, _, _, _, _, _, _) => {
                h1479
            }
        };
        let a1481 = 1u64;
        nat_eq(a1480, a1481)
    } {
        true
    } else {
        let a1483 = match v1.clone() {
            Adt2::C0(h1482, _, _, _, _, _, _, _, _, _, _) => {
                h1482
            }
        };
        let a1484 = 30u64;
        nat_eq(a1483, a1484)
    } {
        let a1487 = match v1.clone() {
            Adt2::C0(h1485, _, _, _, _, _, _, _, _, _, _) => {
                h1485
            }
        };
        let a1488 = match v2.clone() {
            Adt2::C0(h1486, _, _, _, _, _, _, _, _, _, _) => {
                h1486
            }
        };
        if if if if if nat_eq(a1487, a1488) {
            let a1491 = match v1.clone() {
                Adt2::C0(_, h1489, _, _, _, _, _, _, _, _, _) => {
                    h1489
                }
            };
            let a1492 = match v2.clone() {
                Adt2::C0(_, h1490, _, _, _, _, _, _, _, _, _) => {
                    h1490
                }
            };
            nat_eq(a1491, a1492)
        } else {
            false
        } {
            let a1495 = match v1.clone() {
                Adt2::C0(_, _, _, h1493, _, _, _, _, _, _, _) => {
                    h1493
                }
            };
            let a1496 = match v2.clone() {
                Adt2::C0(_, _, _, h1494, _, _, _, _, _, _, _) => {
                    h1494
                }
            };
            nat_eq(a1495, a1496)
        } else {
            false
        } {
            let a1499 = match v1.clone() {
                Adt2::C0(_, _, _, _, h1497, _, _, _, _, _, _) => {
                    h1497
                }
            };
            let a1500 = match v2.clone() {
                Adt2::C0(_, _, _, _, h1498, _, _, _, _, _, _) => {
                    h1498
                }
            };
            nat_eq(a1499, a1500)
        } else {
            false
        } {
            let a1501 = v0;
            let a1502 = 4u64;
            if nat_eq(a1501, a1502) {
                let a1504 = match v1.clone() {
                    Adt2::C0(_, _, h1503, _, _, _, _, _, _, _, _) => {
                        h1503
                    }
                };
                let a1505 = 1u64;
                if nat_eq(a1504, a1505) {
                    let a1508 = match v1.clone() {
                        Adt2::C0(_, h1506, _, _, _, _, _, _, _, _, _) => {
                            h1506
                        }
                    };
                    let a1509 = match v2.clone() {
                        Adt2::C0(_, _, h1507, _, _, _, _, _, _, _, _) => {
                            h1507
                        }
                    };
                    nat_eq(a1508, a1509)
                } else {
                    false
                }
            } else {
                let a1510 = v0;
                let a1511 = 4u64;
                if if nat_lt(a1510, a1511) {
                    let a1514 = match v1.clone() {
                        Adt2::C0(_, h1512, _, _, _, _, _, _, _, _, _) => {
                            h1512
                        }
                    };
                    let a1515 = match v1.clone() {
                        Adt2::C0(_, _, h1513, _, _, _, _, _, _, _, _) => {
                            h1513
                        }
                    };
                    nat_eq(a1514, a1515)
                } else {
                    false
                } {
                    let a1518 = match v1.clone() {
                        Adt2::C0(_, _, h1516, _, _, _, _, _, _, _, _) => {
                            h1516
                        }
                    };
                    let a1519 = match v2.clone() {
                        Adt2::C0(_, _, h1517, _, _, _, _, _, _, _, _) => {
                            h1517
                        }
                    };
                    nat_eq(a1518, a1519)
                } else {
                    false
                }
            }
        } else {
            false
        } {
            let m1521 = f104(match v1.clone() {
                Adt2::C0(h1520, _, _, _, _, _, _, _, _, _, _) => {
                    h1520
                }
            });
            match m1521 {
                Adt16::C0 => {
                    Ok(Adt10::C0)
                }
                Adt16::C1(v3) => {
                    Ok(Adt10::C4(f13(v2.clone(), 1u64)?, match v3 {
                        Adt17::C0(_, _, h1522) => {
                            h1522
                        }
                    }, f17(v1, 1u64), f17(v2, 1u64), Adt4::C8(v0)))
                }
            }
        } else {
            Ok(Adt10::C1)
        }
    } else {
        Ok(Adt10::C0)
    }
}

pub fn f53(v0: u64, v1: Adt2, v2: Adt2) -> R<Adt10> {
    if if if f110(match v1.clone() {
        Adt2::C0(h1523, _, _, _, _, _, _, _, _, _, _) => {
            h1523
        }
    }, v0) {
        let a1526 = match v1.clone() {
            Adt2::C0(h1524, _, _, _, _, _, _, _, _, _, _) => {
                h1524
            }
        };
        let a1527 = match v2.clone() {
            Adt2::C0(h1525, _, _, _, _, _, _, _, _, _, _) => {
                h1525
            }
        };
        nat_eq(a1526, a1527)
    } else {
        false
    } {
        let a1530 = match v1.clone() {
            Adt2::C0(_, h1528, _, _, _, _, _, _, _, _, _) => {
                h1528
            }
        };
        let a1531 = match v2.clone() {
            Adt2::C0(_, h1529, _, _, _, _, _, _, _, _, _) => {
                h1529
            }
        };
        if if if nat_eq(a1530, a1531) {
            let a1534 = match v1.clone() {
                Adt2::C0(_, _, h1532, _, _, _, _, _, _, _, _) => {
                    h1532
                }
            };
            let a1535 = match v2.clone() {
                Adt2::C0(_, _, h1533, _, _, _, _, _, _, _, _) => {
                    h1533
                }
            };
            nat_eq(a1534, a1535)
        } else {
            false
        } {
            let a1538 = match v1.clone() {
                Adt2::C0(_, _, _, h1536, _, _, _, _, _, _, _) => {
                    h1536
                }
            };
            let a1539 = match v2.clone() {
                Adt2::C0(_, _, _, h1537, _, _, _, _, _, _, _) => {
                    h1537
                }
            };
            nat_eq(a1538, a1539)
        } else {
            false
        } {
            let a1542 = match v1.clone() {
                Adt2::C0(_, _, _, _, h1540, _, _, _, _, _, _) => {
                    h1540
                }
            };
            let a1543 = match v2.clone() {
                Adt2::C0(_, _, _, _, h1541, _, _, _, _, _, _) => {
                    h1541
                }
            };
            nat_eq(a1542, a1543)
        } else {
            false
        }
    } else {
        false
    } {
        let m1545 = f104(match v1.clone() {
            Adt2::C0(h1544, _, _, _, _, _, _, _, _, _, _) => {
                h1544
            }
        });
        match m1545 {
            Adt16::C0 => {
                Ok(Adt10::C0)
            }
            Adt16::C1(v3) => {
                Ok(Adt10::C4(f13(v2.clone(), 1u64)?, match v3 {
                    Adt17::C0(_, _, h1546) => {
                        h1546
                    }
                }, f17(v1.clone(), 1u64), f17(v2, 1u64), Adt4::C9(match v1 {
                    Adt2::C0(h1547, _, _, _, _, _, _, _, _, _, _) => {
                        h1547
                    }
                }, v0)))
            }
        }
    } else {
        Ok(Adt10::C1)
    }
}

pub fn f54(v0: bool, v1: Adt1, v2: Adt2, v3: Adt2) -> R<Adt10> {
    let v4: u64 = if v0 {
        match v1.clone() {
            Adt1::C0(h1548, _, _, _, _, _, _, _, _, _, _, _) => {
                h1548
            }
        }
    } else {
        match v1.clone() {
            Adt1::C0(_, _, h1549, _, _, _, _, _, _, _, _, _) => {
                h1549
            }
        }
    };
    let a1550 = 0u64;
    let a1551 = v4;
    if if nat_lt(a1550, a1551) {
        let a1552 = v4;
        let a1553 = 2147483648u64;
        nat_lt(a1552, a1553)
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
        let v7: u64 = f111(match v5.clone() {
            Adt2::C0(_, _, h1554, _, _, _, _, _, _, _, _) => {
                h1554
            }
        }, v4)?;
        let v8: u64 = f111(match v5.clone() {
            Adt2::C0(_, _, _, h1555, _, _, _, _, _, _, _) => {
                h1555
            }
        }, v4)?;
        let a1557 = match v5.clone() {
            Adt2::C0(h1556, _, _, _, _, _, _, _, _, _, _) => {
                h1556
            }
        };
        let a1558 = 0u64;
        if if if if if nat_eq(a1557, a1558) {
            let a1560 = match v6.clone() {
                Adt2::C0(h1559, _, _, _, _, _, _, _, _, _, _) => {
                    h1559
                }
            };
            let a1561 = 0u64;
            nat_eq(a1560, a1561)
        } else {
            false
        } {
            let a1563 = match v5.clone() {
                Adt2::C0(_, _, _, _, h1562, _, _, _, _, _, _) => {
                    h1562
                }
            };
            let a1564 = 1u64;
            nat_eq(a1563, a1564)
        } else {
            false
        } {
            let a1567 = match v5.clone() {
                Adt2::C0(_, h1565, _, _, _, _, _, _, _, _, _) => {
                    h1565
                }
            };
            let a1568 = match v6.clone() {
                Adt2::C0(_, h1566, _, _, _, _, _, _, _, _, _) => {
                    h1566
                }
            };
            nat_eq(a1567, a1568)
        } else {
            false
        } {
            if v0 {
                let a1570 = match v5.clone() {
                    Adt2::C0(_, _, h1569, _, _, _, _, _, _, _, _) => {
                        h1569
                    }
                };
                let a1571 = 2147483648u64;
                if if if nat_lt(a1570, a1571) {
                    let a1573 = match v5.clone() {
                        Adt2::C0(_, _, _, h1572, _, _, _, _, _, _, _) => {
                            h1572
                        }
                    };
                    let a1574 = 2147483648u64;
                    nat_lt(a1573, a1574)
                } else {
                    false
                } {
                    let a1577 = {
                        let a1575 = v7;
                        let a1576 = v8;
                        nat_mul(a1575, a1576)?
                    };
                    let a1578 = 2147483648u64;
                    nat_le(a1577, a1578)
                } else {
                    false
                } {
                    f112(v5, v6, v4, v7)?
                } else {
                    false
                }
            } else {
                let a1580 = match v1.clone() {
                    Adt1::C0(h1579, _, _, _, _, _, _, _, _, _, _, _) => {
                        h1579
                    }
                };
                let a1581 = v7;
                if if if if nat_eq(a1580, a1581) {
                    let a1583 = match v1 {
                        Adt1::C0(_, h1582, _, _, _, _, _, _, _, _, _, _) => {
                            h1582
                        }
                    };
                    let a1584 = v8;
                    nat_eq(a1583, a1584)
                } else {
                    false
                } {
                    f113(v7, v8, v4)?
                } else {
                    false
                } {
                    let a1586 = match v6.clone() {
                        Adt2::C0(_, _, h1585, _, _, _, _, _, _, _, _) => {
                            h1585
                        }
                    };
                    let a1587 = v4;
                    if nat_eq(a1586, a1587) {
                        let a1589 = match v6.clone() {
                            Adt2::C0(_, _, _, h1588, _, _, _, _, _, _, _) => {
                                h1588
                            }
                        };
                        let a1590 = v4;
                        nat_eq(a1589, a1590)
                    } else {
                        false
                    }
                } else {
                    false
                } {
                    let a1594 = match v6 {
                        Adt2::C0(_, _, _, _, h1591, _, _, _, _, _, _) => {
                            h1591
                        }
                    };
                    let a1595 = {
                        let a1592 = v7;
                        let a1593 = v8;
                        nat_mul(a1592, a1593)?
                    };
                    nat_eq(a1594, a1595)
                } else {
                    false
                }
            }
        } else {
            false
        } {
            Ok(Adt10::C4(f13(v3.clone(), 1u64)?, 4u64, f17(v2, 1u64), f17(v3, 1u64), Adt4::C12(v0, v4, v7)))
        } else {
            Ok(Adt10::C1)
        }
    } else {
        Ok(Adt10::C1)
    }
}

pub fn f55(v0: u64, v1: Adt1, v2: Adt2, v3: Adt2) -> R<Adt10> {
    let a1597 = match v2.clone() {
        Adt2::C0(h1596, _, _, _, _, _, _, _, _, _, _) => {
            h1596
        }
    };
    let a1598 = 0u64;
    if if if if if nat_eq(a1597, a1598) {
        true
    } else {
        let a1600 = match v2.clone() {
            Adt2::C0(h1599, _, _, _, _, _, _, _, _, _, _) => {
                h1599
            }
        };
        let a1601 = 1u64;
        nat_eq(a1600, a1601)
    } {
        true
    } else {
        let a1603 = match v2.clone() {
            Adt2::C0(h1602, _, _, _, _, _, _, _, _, _, _) => {
                h1602
            }
        };
        let a1604 = 30u64;
        nat_eq(a1603, a1604)
    } {
        let a1607 = match v2.clone() {
            Adt2::C0(h1605, _, _, _, _, _, _, _, _, _, _) => {
                h1605
            }
        };
        let a1608 = match v3.clone() {
            Adt2::C0(h1606, _, _, _, _, _, _, _, _, _, _) => {
                h1606
            }
        };
        nat_eq(a1607, a1608)
    } else {
        false
    } {
        if if f20(v2.clone(), v3.clone()) {
            let a1610 = match v1.clone() {
                Adt1::C0(h1609, _, _, _, _, _, _, _, _, _, _, _) => {
                    h1609
                }
            };
            let a1611 = 4294967296u64;
            nat_lt(a1610, a1611)
        } else {
            false
        } {
            let a1613 = match v1.clone() {
                Adt1::C0(_, h1612, _, _, _, _, _, _, _, _, _, _) => {
                    h1612
                }
            };
            let a1614 = 4294967296u64;
            nat_lt(a1613, a1614)
        } else {
            false
        }
    } else {
        false
    } {
        Ok(Adt10::C4(f13(v3.clone(), 1u64)?, {
            let a1616 = match v2.clone() {
                Adt2::C0(h1615, _, _, _, _, _, _, _, _, _, _) => {
                    h1615
                }
            };
            let a1617 = 0u64;
            if nat_eq(a1616, a1617) {
                4u64
            } else {
                2u64
            }
        }, f17(v2.clone(), 1u64), f17(v3, 1u64), Adt4::C13({
            let a1618 = v0;
            let a1619 = 31u64;
            if nat_eq(a1618, a1619) {
                0u64
            } else {
                let a1620 = v0;
                let a1621 = 47u64;
                if nat_eq(a1620, a1621) {
                    1u64
                } else {
                    2u64
                }
            }
        }, match v2 {
            Adt2::C0(h1622, _, _, _, _, _, _, _, _, _, _) => {
                h1622
            }
        }, match v1.clone() {
            Adt1::C0(h1623, _, _, _, _, _, _, _, _, _, _, _) => {
                h1623
            }
        }, match v1 {
            Adt1::C0(_, h1624, _, _, _, _, _, _, _, _, _, _) => {
                h1624
            }
        })))
    } else {
        Ok(Adt10::C1)
    }
}

pub fn f56(v0: Adt2, v1: Adt2) -> R<Adt10> {
    let a1626 = match v0.clone() {
        Adt2::C0(h1625, _, _, _, _, _, _, _, _, _, _) => {
            h1625
        }
    };
    let a1627 = 0u64;
    if if if nat_eq(a1626, a1627) {
        let a1629 = match v1.clone() {
            Adt2::C0(h1628, _, _, _, _, _, _, _, _, _, _) => {
                h1628
            }
        };
        let a1630 = 0u64;
        nat_eq(a1629, a1630)
    } else {
        false
    } {
        f20(v0.clone(), v1.clone())
    } else {
        false
    } {
        Ok(Adt10::C4(f13(v1.clone(), 1u64)?, 4u64, f17(v0, 1u64), f17(v1, 1u64), Adt4::C15))
    } else {
        Ok(Adt10::C1)
    }
}

pub fn f57(v0: Adt2, v1: Adt2) -> R<Adt10> {
    let a1632 = match v0.clone() {
        Adt2::C0(h1631, _, _, _, _, _, _, _, _, _, _) => {
            h1631
        }
    };
    let a1633 = 0u64;
    if if nat_eq(a1632, a1633) {
        let a1635 = match v1.clone() {
            Adt2::C0(h1634, _, _, _, _, _, _, _, _, _, _) => {
                h1634
            }
        };
        let a1636 = 26u64;
        nat_eq(a1635, a1636)
    } else {
        false
    } {
        let a1638 = match v0.clone() {
            Adt2::C0(_, _, _, h1637, _, _, _, _, _, _, _) => {
                h1637
            }
        };
        let a1639 = 1u64;
        if if if if if if if nat_eq(a1638, a1639) {
            let a1641 = match v0.clone() {
                Adt2::C0(_, _, _, _, h1640, _, _, _, _, _, _) => {
                    h1640
                }
            };
            let a1642 = 1u64;
            nat_eq(a1641, a1642)
        } else {
            false
        } {
            let a1644 = match v0.clone() {
                Adt2::C0(_, h1643, _, _, _, _, _, _, _, _, _) => {
                    h1643
                }
            };
            let a1645 = 2147483647u64;
            nat_le(a1644, a1645)
        } else {
            false
        } {
            let a1648 = match v1.clone() {
                Adt2::C0(_, h1646, _, _, _, _, _, _, _, _, _) => {
                    h1646
                }
            };
            let a1649 = match v0.clone() {
                Adt2::C0(_, _, h1647, _, _, _, _, _, _, _, _) => {
                    h1647
                }
            };
            nat_eq(a1648, a1649)
        } else {
            false
        } {
            let a1651 = match v1.clone() {
                Adt2::C0(_, _, h1650, _, _, _, _, _, _, _, _) => {
                    h1650
                }
            };
            let a1652 = 1u64;
            nat_eq(a1651, a1652)
        } else {
            false
        } {
            let a1654 = match v1.clone() {
                Adt2::C0(_, _, _, h1653, _, _, _, _, _, _, _) => {
                    h1653
                }
            };
            let a1655 = 1u64;
            nat_eq(a1654, a1655)
        } else {
            false
        } {
            let a1657 = match v1.clone() {
                Adt2::C0(_, _, _, _, h1656, _, _, _, _, _, _) => {
                    h1656
                }
            };
            let a1658 = 1u64;
            nat_eq(a1657, a1658)
        } else {
            false
        } {
            Ok(Adt10::C4({
                let a1661 = match v0.clone() {
                    Adt2::C0(_, _, h1659, _, _, _, _, _, _, _, _) => {
                        h1659
                    }
                };
                let a1662 = f114(match v0.clone() {
                    Adt2::C0(_, h1660, _, _, _, _, _, _, _, _, _) => {
                        h1660
                    }
                })?;
                nat_mul(a1661, a1662)?
            }, 4u64, f17(v0, 1u64), f17(v1, 1u64), Adt4::C17))
        } else {
            Ok(Adt10::C1)
        }
    } else {
        Ok(Adt10::C0)
    }
}

pub fn f58(v0: Adt2, v1: Option<Adt2>, v2: Adt2) -> R<Adt10> {
    let m1663 = v1;
    match m1663 {
        None => {
            Ok(Adt10::C1)
        }
        Some(v3) => {
            let v4: u64 = f115(f17(v0.clone(), 1u64))?;
            let a1665 = match v0.clone() {
                Adt2::C0(h1664, _, _, _, _, _, _, _, _, _, _) => {
                    h1664
                }
            };
            let a1666 = 26u64;
            if if if nat_eq(a1665, a1666) {
                let a1668 = match v3.clone() {
                    Adt2::C0(h1667, _, _, _, _, _, _, _, _, _, _) => {
                        h1667
                    }
                };
                let a1669 = 26u64;
                nat_eq(a1668, a1669)
            } else {
                false
            } {
                let a1671 = match v2.clone() {
                    Adt2::C0(h1670, _, _, _, _, _, _, _, _, _, _) => {
                        h1670
                    }
                };
                let a1672 = 27u64;
                nat_eq(a1671, a1672)
            } else {
                false
            } {
                if if if if if if f20(v0.clone(), v3.clone()) {
                    let a1674 = match v2.clone() {
                        Adt2::C0(_, h1673, _, _, _, _, _, _, _, _, _) => {
                            h1673
                        }
                    };
                    let a1675 = 1u64;
                    nat_eq(a1674, a1675)
                } else {
                    false
                } {
                    let a1677 = match v2.clone() {
                        Adt2::C0(_, _, h1676, _, _, _, _, _, _, _, _) => {
                            h1676
                        }
                    };
                    let a1678 = 1u64;
                    nat_eq(a1677, a1678)
                } else {
                    false
                } {
                    let a1680 = match v2.clone() {
                        Adt2::C0(_, _, _, h1679, _, _, _, _, _, _, _) => {
                            h1679
                        }
                    };
                    let a1681 = 1u64;
                    nat_eq(a1680, a1681)
                } else {
                    false
                } {
                    let a1683 = match v2.clone() {
                        Adt2::C0(_, _, _, _, h1682, _, _, _, _, _, _) => {
                            h1682
                        }
                    };
                    let a1684 = 1u64;
                    nat_eq(a1683, a1684)
                } else {
                    false
                } {
                    let a1685 = v4;
                    let a1686 = 9223372036854775808u64;
                    nat_lt(a1685, a1686)
                } else {
                    false
                } {
                    Ok(Adt10::C4({
                        let a1689 = 2u64;
                        let a1690 = {
                            let a1687 = v4;
                            let a1688 = 2u64;
                            nat_mul(a1687, a1688)?
                        };
                        nat_add(a1689, a1690)?
                    }, 8u64, f17(v0, 1u64), f17(v2, 1u64), Adt4::C18(v4, f17(v3, 1u64))))
                } else {
                    Ok(Adt10::C1)
                }
            } else {
                Ok(Adt10::C0)
            }
        }
    }
}

pub fn f59(v0: u64, v1: Adt2, v2: Adt2) -> R<Adt10> {
    let v3: u64 = match v1.clone() {
        Adt2::C0(h1691, _, _, _, _, _, _, _, _, _, _) => {
            h1691
        }
    };
    let a1692 = v3;
    let a1693 = 0u64;
    if if nat_eq(a1692, a1693) {
        true
    } else {
        let a1694 = v0;
        let a1695 = 13u64;
        if nat_eq(a1694, a1695) {
            let a1696 = v3;
            let a1697 = 1u64;
            if nat_eq(a1696, a1697) {
                true
            } else {
                let a1698 = v3;
                let a1699 = 30u64;
                nat_eq(a1698, a1699)
            }
        } else {
            false
        }
    } {
        let a1701 = v3;
        let a1702 = match v2.clone() {
            Adt2::C0(h1700, _, _, _, _, _, _, _, _, _, _) => {
                h1700
            }
        };
        if if nat_eq(a1701, a1702) {
            let a1703 = v0;
            let a1704 = 13u64;
            if nat_eq(a1703, a1704) {
                let a1706 = match v2.clone() {
                    Adt2::C0(_, h1705, _, _, _, _, _, _, _, _, _) => {
                        h1705
                    }
                };
                let a1707 = 1u64;
                if if if nat_eq(a1706, a1707) {
                    let a1709 = match v2.clone() {
                        Adt2::C0(_, _, h1708, _, _, _, _, _, _, _, _) => {
                            h1708
                        }
                    };
                    let a1710 = 1u64;
                    nat_eq(a1709, a1710)
                } else {
                    false
                } {
                    let a1712 = match v2.clone() {
                        Adt2::C0(_, _, _, h1711, _, _, _, _, _, _, _) => {
                            h1711
                        }
                    };
                    let a1713 = 1u64;
                    nat_eq(a1712, a1713)
                } else {
                    false
                } {
                    let a1715 = match v2.clone() {
                        Adt2::C0(_, _, _, _, h1714, _, _, _, _, _, _) => {
                            h1714
                        }
                    };
                    let a1716 = 1u64;
                    nat_eq(a1715, a1716)
                } else {
                    false
                }
            } else {
                let a1718 = match v2.clone() {
                    Adt2::C0(_, h1717, _, _, _, _, _, _, _, _, _) => {
                        h1717
                    }
                };
                let a1719 = 1u64;
                if if if nat_eq(a1718, a1719) {
                    let a1722 = match v1.clone() {
                        Adt2::C0(_, _, h1720, _, _, _, _, _, _, _, _) => {
                            h1720
                        }
                    };
                    let a1723 = match v2.clone() {
                        Adt2::C0(_, _, h1721, _, _, _, _, _, _, _, _) => {
                            h1721
                        }
                    };
                    nat_eq(a1722, a1723)
                } else {
                    false
                } {
                    let a1726 = match v1.clone() {
                        Adt2::C0(_, _, _, h1724, _, _, _, _, _, _, _) => {
                            h1724
                        }
                    };
                    let a1727 = match v2.clone() {
                        Adt2::C0(_, _, _, h1725, _, _, _, _, _, _, _) => {
                            h1725
                        }
                    };
                    nat_eq(a1726, a1727)
                } else {
                    false
                } {
                    let a1730 = match v1.clone() {
                        Adt2::C0(_, _, _, _, h1728, _, _, _, _, _, _) => {
                            h1728
                        }
                    };
                    let a1731 = match v2.clone() {
                        Adt2::C0(_, _, _, _, h1729, _, _, _, _, _, _) => {
                            h1729
                        }
                    };
                    nat_eq(a1730, a1731)
                } else {
                    false
                }
            }
        } else {
            false
        } {
            let v4: Adt3 = f17(v1, 1u64);
            Ok(Adt10::C4(f116(v4.clone(), v0, v3)?, {
                let a1732 = v3;
                let a1733 = 0u64;
                if nat_eq(a1732, a1733) {
                    4u64
                } else {
                    2u64
                }
            }, v4, f17(v2, 1u64), Adt4::C16(v0, v3)))
        } else {
            Ok(Adt10::C1)
        }
    } else {
        Ok(Adt10::C0)
    }
}

pub fn f60(v0: u64, v1: u64) -> R<Option<u64>> {
    let a1734 = v0;
    let a1735 = 89u64;
    if if nat_eq(a1734, a1735) {
        let a1736 = 8u64;
        let a1737 = v1;
        if nat_le(a1736, a1737) {
            let a1738 = v1;
            let a1739 = 9u64;
            nat_le(a1738, a1739)
        } else {
            false
        }
    } else {
        false
    } {
        Ok(Some({
            let a1740 = v1;
            let a1741 = 12u64;
            nat_add(a1740, a1741)?
        }))
    } else {
        let a1742 = v0;
        let a1743 = 89u64;
        if if nat_eq(a1742, a1743) {
            let a1744 = v1;
            let a1745 = 4u64;
            nat_eq(a1744, a1745)
        } else {
            false
        } {
            Ok(Some(19u64))
        } else {
            let a1746 = 10u64;
            let a1747 = v0;
            if if nat_le(a1746, a1747) {
                let a1748 = v0;
                let a1749 = 12u64;
                nat_le(a1748, a1749)
            } else {
                false
            } {
                Ok(Some({
                    let a1750 = v0;
                    let a1751 = 3u64;
                    nat_add(a1750, a1751)?
                }))
            } else {
                let a1752 = v0;
                let a1753 = 89u64;
                if if nat_eq(a1752, a1753) {
                    let a1754 = v1;
                    let a1755 = 7u64;
                    nat_eq(a1754, a1755)
                } else {
                    false
                } {
                    Ok(Some(17u64))
                } else {
                    let a1756 = v0;
                    let a1757 = 89u64;
                    if if nat_eq(a1756, a1757) {
                        let a1758 = v1;
                        let a1759 = 10u64;
                        nat_eq(a1758, a1759)
                    } else {
                        false
                    } {
                        Ok(Some(18u64))
                    } else {
                        let a1760 = v0;
                        let a1761 = 89u64;
                        if if nat_eq(a1760, a1761) {
                            let a1762 = v1;
                            let a1763 = 13u64;
                            nat_eq(a1762, a1763)
                        } else {
                            false
                        } {
                            Ok(Some(16u64))
                        } else {
                            let a1764 = v0;
                            let a1765 = 8u64;
                            if nat_eq(a1764, a1765) {
                                Ok(Some(5u64))
                            } else {
                                let a1766 = v0;
                                let a1767 = 9u64;
                                if nat_eq(a1766, a1767) {
                                    Ok(Some(6u64))
                                } else {
                                    let a1768 = v0;
                                    let a1769 = 89u64;
                                    if nat_eq(a1768, a1769) {
                                        let a1770 = v1;
                                        let a1771 = 0u64;
                                        if nat_eq(a1770, a1771) {
                                            Ok(Some(0u64))
                                        } else {
                                            let a1772 = v1;
                                            let a1773 = 1u64;
                                            if nat_eq(a1772, a1773) {
                                                Ok(Some(1u64))
                                            } else {
                                                let a1774 = v1;
                                                let a1775 = 2u64;
                                                if nat_eq(a1774, a1775) {
                                                    Ok(Some(2u64))
                                                } else {
                                                    let a1776 = v1;
                                                    let a1777 = 3u64;
                                                    if nat_eq(a1776, a1777) {
                                                        Ok(Some(3u64))
                                                    } else {
                                                        let a1778 = v1;
                                                        let a1779 = 6u64;
                                                        if nat_eq(a1778, a1779) {
                                                            Ok(Some(4u64))
                                                        } else {
                                                            let a1780 = v1;
                                                            let a1781 = 12u64;
                                                            if nat_eq(a1780, a1781) {
                                                                Ok(Some(7u64))
                                                            } else {
                                                                let a1782 = v1;
                                                                let a1783 = 11u64;
                                                                if nat_eq(a1782, a1783) {
                                                                    Ok(Some(8u64))
                                                                } else {
                                                                    let a1784 = v1;
                                                                    let a1785 = 18u64;
                                                                    if nat_eq(a1784, a1785) {
                                                                        Ok(Some(9u64))
                                                                    } else {
                                                                        let a1786 = v1;
                                                                        let a1787 = 19u64;
                                                                        if nat_eq(a1786, a1787) {
                                                                            Ok(Some(10u64))
                                                                        } else {
                                                                            let a1788 = v1;
                                                                            let a1789 = 20u64;
                                                                            if nat_eq(a1788, a1789) {
                                                                                Ok(Some(11u64))
                                                                            } else {
                                                                                let a1790 = v1;
                                                                                let a1791 = 21u64;
                                                                                if nat_eq(a1790, a1791) {
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

pub fn f61(v0: u64, v1: u64, v2: Adt2, v3: Option<Adt2>, v4: Adt2) -> R<Adt10> {
    let m1792 = f117(v0, v1, v2.clone(), v3.clone(), v4.clone())?;
    match m1792 {
        Adt10::C0 => {
            Ok(Adt10::C0)
        }
        Adt10::C1 => {
            let a1793 = v0;
            let a1794 = 2u64;
            if if if if nat_eq(a1793, a1794) {
                true
            } else {
                let a1795 = v0;
                let a1796 = 5u64;
                nat_eq(a1795, a1796)
            } {
                true
            } else {
                let a1797 = v0;
                let a1798 = 6u64;
                nat_eq(a1797, a1798)
            } {
                true
            } else {
                let a1799 = v0;
                let a1800 = 7u64;
                nat_eq(a1799, a1800)
            } {
                Ok(Adt10::C5(67u64))
            } else {
                Ok(Adt10::C1)
            }
        }
        Adt10::C2(v7, v8, _, _, _, _, v13) => {
            if v8 {
                Ok(Adt10::C4(v13, 4u64, f17(v2.clone(), 1u64), f17(v4, 1u64), Adt4::C10(v7, v8, {
                    let m1801 = v3;
                    match m1801 {
                        None => {
                            f17(v2, 1u64)
                        }
                        Some(v14) => {
                            f17(v14, 1u64)
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

pub fn f62(v0: u64, v1: Adt2, v2: Adt2) -> R<Adt10> {
    let a1803 = match v1.clone() {
        Adt2::C0(h1802, _, _, _, _, _, _, _, _, _, _) => {
            h1802
        }
    };
    let a1804 = 0u64;
    if if if if if nat_eq(a1803, a1804) {
        true
    } else {
        let a1806 = match v1.clone() {
            Adt2::C0(h1805, _, _, _, _, _, _, _, _, _, _) => {
                h1805
            }
        };
        let a1807 = 1u64;
        nat_eq(a1806, a1807)
    } {
        true
    } else {
        let a1809 = match v1.clone() {
            Adt2::C0(h1808, _, _, _, _, _, _, _, _, _, _) => {
                h1808
            }
        };
        let a1810 = 30u64;
        nat_eq(a1809, a1810)
    } {
        let a1813 = match v1.clone() {
            Adt2::C0(h1811, _, _, _, _, _, _, _, _, _, _) => {
                h1811
            }
        };
        let a1814 = match v2.clone() {
            Adt2::C0(h1812, _, _, _, _, _, _, _, _, _, _) => {
                h1812
            }
        };
        if nat_eq(a1813, a1814) {
            true
        } else {
            let a1816 = match v2.clone() {
                Adt2::C0(h1815, _, _, _, _, _, _, _, _, _, _) => {
                    h1815
                }
            };
            let a1817 = 0u64;
            nat_eq(a1816, a1817)
        }
    } else {
        false
    } {
        f20(v1.clone(), v2.clone())
    } else {
        false
    } {
        Ok(Adt10::C4(f13(v2.clone(), 1u64)?, {
            let a1819 = match v2.clone() {
                Adt2::C0(h1818, _, _, _, _, _, _, _, _, _, _) => {
                    h1818
                }
            };
            let a1820 = 0u64;
            if nat_eq(a1819, a1820) {
                4u64
            } else {
                2u64
            }
        }, f17(v1.clone(), 1u64), f17(v2.clone(), 1u64), Adt4::C14(v0, match v1 {
            Adt2::C0(h1821, _, _, _, _, _, _, _, _, _, _) => {
                h1821
            }
        }, match v2 {
            Adt2::C0(h1822, _, _, _, _, _, _, _, _, _, _) => {
                h1822
            }
        })))
    } else {
        Ok(Adt10::C1)
    }
}

pub fn f63(v0: Adt3, v1: u64, v2: u64, v3: u64, v4: u64) -> R<u64> {
    f118(v1, v2, v3, v4, match v0.clone() {
        Adt3::C0(_, _, _, _, h1823, _, _, _) => {
            h1823
        }
    }, match v0.clone() {
        Adt3::C0(_, _, _, _, _, h1824, _, _) => {
            h1824
        }
    }, match v0.clone() {
        Adt3::C0(_, _, _, _, _, _, h1825, _) => {
            h1825
        }
    }, match v0 {
        Adt3::C0(_, _, _, _, _, _, _, h1826) => {
            h1826
        }
    })
}

pub fn f64(v0: u64, v1: Adt3, v2: u64) -> u64 {
    let v3: u64 = {
        let a1828 = v0;
        let a1829 = match v1.clone() {
            Adt3::C0(h1827, _, _, _, _, _, _, _) => {
                h1827
            }
        };
        let a1830 = 0u64;
        nat_quot(a1828, a1829, a1830)
    };
    let v4: u64 = {
        let a1832 = v3;
        let a1833 = match v1.clone() {
            Adt3::C0(_, h1831, _, _, _, _, _, _) => {
                h1831
            }
        };
        let a1834 = 0u64;
        nat_quot(a1832, a1833, a1834)
    };
    let a1835 = v2;
    let a1836 = 0u64;
    if nat_eq(a1835, a1836) {
        let a1838 = v0;
        let a1839 = match v1 {
            Adt3::C0(h1837, _, _, _, _, _, _, _) => {
                h1837
            }
        };
        let a1840 = 0u64;
        nat_rem(a1838, a1839, a1840)
    } else {
        let a1841 = v2;
        let a1842 = 1u64;
        if nat_eq(a1841, a1842) {
            let a1844 = v3;
            let a1845 = match v1 {
                Adt3::C0(_, h1843, _, _, _, _, _, _) => {
                    h1843
                }
            };
            let a1846 = 0u64;
            nat_rem(a1844, a1845, a1846)
        } else {
            let a1847 = v2;
            let a1848 = 2u64;
            if nat_eq(a1847, a1848) {
                let a1850 = v4;
                let a1851 = match v1 {
                    Adt3::C0(_, _, h1849, _, _, _, _, _) => {
                        h1849
                    }
                };
                let a1852 = 0u64;
                nat_rem(a1850, a1851, a1852)
            } else {
                let a1854 = v4;
                let a1855 = match v1 {
                    Adt3::C0(_, _, h1853, _, _, _, _, _) => {
                        h1853
                    }
                };
                let a1856 = 0u64;
                nat_quot(a1854, a1855, a1856)
            }
        }
    }
}

pub fn f65(v0: Adt8, v1: u64, v2: u64) -> R<u64> {
    let a1858 = match v0.clone() {
        Adt8::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, h1857) => {
            h1857
        }
    };
    let a1859 = 0u64;
    if nat_eq(a1858, a1859) {
        Ok(v2)
    } else {
        let a1864 = {
            let a1861 = v2;
            let a1862 = match v0.clone() {
                Adt8::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, h1860) => {
                    h1860
                }
            };
            nat_mul(a1861, a1862)?
        };
        let a1865 = f64(v1, match v0 {
            Adt8::C0(_, _, _, _, _, _, _, _, h1863, _, _, _, _, _, _) => {
                h1863
            }
        }, 3u64);
        nat_add(a1864, a1865)
    }
}

pub fn f66(v0: Adt6, v1: u64, v2: u64, v3: u64) -> R<Option<u64>> {
    let v4: u64 = {
        let a1875 = {
            let a1872 = {
                let a1867 = v2;
                let a1868 = match v0.clone() {
                    Adt6::C0(_, h1866, _, _) => {
                        h1866
                    }
                };
                nat_mul(a1867, a1868)?
            };
            let a1873 = {
                let a1870 = v3;
                let a1871 = match v0.clone() {
                    Adt6::C0(_, _, _, h1869) => {
                        h1869
                    }
                };
                nat_mul(a1870, a1871)?
            };
            nat_add(a1872, a1873)?
        };
        let a1876 = f119(match v0.clone() {
            Adt6::C0(_, _, h1874, _) => {
                h1874
            }
        });
        nat_add(a1875, a1876)?
    };
    let v5: u64 = f120(match v0 {
        Adt6::C0(_, _, h1877, _) => {
            h1877
        }
    });
    let a1878 = v5;
    let a1879 = v4;
    if if nat_le(a1878, a1879) {
        let a1882 = {
            let a1880 = v4;
            let a1881 = v5;
            nat_sub(a1880, a1881)
        };
        let a1883 = v1;
        nat_lt(a1882, a1883)
    } else {
        false
    } {
        Ok(Some({
            let a1884 = v4;
            let a1885 = v5;
            nat_sub(a1884, a1885)
        }))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f67(v0: Adt8, v1: u64, v2: u64, v3: u64, v4: u64) -> R<Adt9> {
    let v5: Adt3 = match v0.clone() {
        Adt8::C0(_, _, _, _, _, _, _, _, _, h1886, _, _, _, _, _) => {
            h1886
        }
    };
    let v6: Adt3 = match v0.clone() {
        Adt8::C0(_, _, _, _, _, _, _, _, h1887, _, _, _, _, _, _) => {
            h1887
        }
    };
    let v7: u64 = f64(v1, v6.clone(), 2u64);
    let a1889 = match v0.clone() {
        Adt8::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, h1888) => {
            h1888
        }
    };
    let a1890 = 0u64;
    if nat_eq(a1889, a1890) {
        Ok(Adt9::C8(Some(f63(v5, v3, v4, v7, v2)?)))
    } else {
        let v8: u64 = {
            let m1893 = match v0.clone() {
                Adt8::C0(_, _, _, _, _, _, _, _, _, _, h1892, _, _, _, _) => {
                    h1892
                }
            };
            match m1893 {
                Adt3::C0(_, _, h1891, _, _, _, _, _) => {
                    h1891
                }
            }
        };
        let m1899 = f66(match v0.clone() {
            Adt8::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, h1894, _) => {
                h1894
            }
        }, match v5.clone() {
            Adt3::C0(_, _, h1895, _, _, _, _, _) => {
                h1895
            }
        }, {
            let a1896 = v2;
            let a1897 = v8;
            let a1898 = 0u64;
            nat_rem(a1896, a1897, a1898)
        }, v7)?;
        match m1899 {
            None => {
                Ok(Adt9::C8(None::<u64>))
            }
            Some(v9) => {
                let v10: u64 = {
                    let a1906 = {
                        let a1904 = {
                            let a1900 = v2;
                            let a1901 = v8;
                            let a1902 = 0u64;
                            nat_quot(a1900, a1901, a1902)
                        };
                        let a1905 = match v0 {
                            Adt8::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, h1903) => {
                                h1903
                            }
                        };
                        nat_mul(a1904, a1905)?
                    };
                    let a1907 = f64(v1, v6, 3u64);
                    nat_add(a1906, a1907)?
                };
                Ok(Adt9::C8(Some(f63(v5, v3, v4, v9, v10)?)))
            }
        }
    }
}

pub fn f68(v0: Adt4, v1: Adt9) -> Adt9 {
    let m1908 = v1;
    match m1908 {
        Adt9::C0(v4) => {
            Adt9::C0(v4)
        }
        Adt9::C1(v5, v6, v7) => {
            Adt9::C1(v5, v6, v7)
        }
        Adt9::C2(v8, v9, v10, v11, v12, v13) => {
            Adt9::C2(v8, v9, v10, v11, v12, v13)
        }
        Adt9::C3(v14, v15, v16, v17) => {
            f121(f122(v0.clone(), v14, v15), v14, v15, 0u64, v16, 2u64, f123(v0), 3u64, v17)
        }
        Adt9::C4(v18, v19, v20, v21, v22, v23, v24) => {
            let v25: u64 = f124(v0.clone(), v18);
            f121(f125(v0, v18), v25, v25, v19, v20, v21, v22, v23, v24)
        }
        Adt9::C5(v26, v27, v28, v29, v30, v31) => {
            Adt9::C5(v26, v27, v28, v29, v30, v31)
        }
        Adt9::C6(v32, v33, v34, v35) => {
            Adt9::C6(v32, v33, v34, v35)
        }
        Adt9::C7(v36, v37, v38, v39, v40, v41) => {
            Adt9::C7(v36, v37, v38, v39, v40, v41)
        }
        Adt9::C8(v2) => {
            Adt9::C8(v2)
        }
        Adt9::C9(v3) => {
            Adt9::C9(v3)
        }
    }
}

pub fn f69(v0: u64, v1: Adt3, v2: Adt3, v3: Adt4) -> R<Adt9> {
    let m1909 = v3;
    match m1909 {
        Adt4::C0 => {
            Ok(Adt9::C1(false, f126(v0, v1)?, f126(v0, v2)?))
        }
        Adt4::C1 => {
            Ok(Adt9::C1(false, f128(v0, v1, v2.clone())?, f126(v0, v2)?))
        }
        Adt4::C2(v6, v7) => {
            f129(v0, v1, v7, v2, v6)
        }
        Adt4::C3(v8, v9, v10, v11, v12) => {
            f130(v0, v1, v2, v8, v9, v10, v11, v12)
        }
        Adt4::C4(v13) => {
            f131(v0, v1, v2, v13)
        }
        Adt4::C5(v14, v15, v16, v17) => {
            f130(v0, v1, v2, v14, v15, v16, v17, true)
        }
        Adt4::C6(v18, v19, v20) => {
            f132(v0, v1, v2, v18, v19, v20)
        }
        Adt4::C7(v21) => {
            f133(v0, v1, v2, v21)
        }
        Adt4::C8(v22) => {
            f134(v22, v0, v1, v2)
        }
        Adt4::C9(_, _) => {
            Ok(Adt9::C1(true, 0u64, f126(v0, v2)?))
        }
        Adt4::C10(v25, v26, v27) => {
            f135(v0, v1, v2, v25, v26, v27)
        }
        Adt4::C11(v28, v29) => {
            Ok(Adt9::C3(v28, v29, f126(v0, v1)?, f126(v0, v2)?))
        }
        Adt4::C12(v30, v31, v32) => {
            f136(v0, v1, v2, v30, v31, v32)
        }
        Adt4::C13(_, v34, _, _) => {
            Ok(Adt9::C3(v34, v34, f126(v0, v1)?, f126(v0, v2)?))
        }
        Adt4::C14(_, v38, v39) => {
            Ok(Adt9::C3(v38, v39, f126(v0, v1)?, f126(v0, v2)?))
        }
        Adt4::C15 => {
            f137(v0, v1, v2)
        }
        Adt4::C16(v40, v41) => {
            f138(v0, v1, v2, v40, v41)
        }
        Adt4::C17 => {
            Ok(Adt9::C0(Adt10::C1))
        }
        Adt4::C18(v42, v43) => {
            f139(v0, v1, v43, v42)
        }
        Adt4::C19(_, _) => {
            Ok(Adt9::C0(Adt10::C1))
        }
        Adt4::C20(_, _) => {
            Ok(Adt9::C0(Adt10::C1))
        }
        Adt4::C21 => {
            Ok(Adt9::C1(false, f126(f127(v0, match v2.clone() {
                Adt3::C0(h1910, _, _, _, _, _, _, _) => {
                    h1910
                }
            }, match v2.clone() {
                Adt3::C0(_, h1911, _, _, _, _, _, _) => {
                    h1911
                }
            })?, v1)?, f126(v0, v2)?))
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
        Adt4::C25(v48, v49, v50) => {
            f140(v0, v48, v49, v50)
        }
        Adt4::C26(v56, _, _, _) => {
            let a1912 = v56;
            let a1913 = 64u64;
            if if if nat_eq(a1912, a1913) {
                true
            } else {
                let a1914 = v56;
                let a1915 = 65u64;
                nat_eq(a1914, a1915)
            } {
                true
            } else {
                let a1916 = v56;
                let a1917 = 66u64;
                nat_eq(a1916, a1917)
            } {
                Ok(Adt9::C0(Adt10::C5(64u64)))
            } else {
                Ok(Adt9::C0(Adt10::C1))
            }
        }
        Adt4::C27(v55) => {
            f77(v0, v1, v2, v55)
        }
        Adt4::C28(v51) => {
            f74(v0, v1, v2, v51, 0u64)
        }
        Adt4::C29(v52) => {
            f75(v0, v1, v2, v52)
        }
        Adt4::C30(v53, _) => {
            f76(v0, v1, v2, v53)
        }
    }
}

pub fn f70(v0: u64, v1: Adt3, v2: Adt3, v3: u64) -> R<Adt9> {
    let v4: u64 = f114(match v1.clone() {
        Adt3::C0(h1918, _, _, _, _, _, _, _) => {
            h1918
        }
    })?;
    let v5: u64 = {
        let a1919 = v0;
        let a1920 = v4;
        let a1921 = 0u64;
        nat_quot(a1919, a1920, a1921)
    };
    let v6: u64 = {
        let a1922 = v0;
        let a1923 = v4;
        let a1924 = 0u64;
        nat_rem(a1922, a1923, a1924)
    };
    let a1925 = v6;
    let a1926 = 0u64;
    if nat_eq(a1925, a1926) {
        Ok(Adt9::C4(19u64, 2u64, 4286578688u64, 2u64, 0u64, 1u64, 0u64))
    } else {
        let a1927 = v6;
        let a1928 = 1u64;
        if nat_eq(a1927, a1928) {
            Ok(Adt9::C4(19u64, 2u64, 0u64, 2u64, 0u64, 1u64, 1u64))
        } else {
            let a1931 = {
                let a1929 = v6;
                let a1930 = 1u64;
                nat_add(a1929, a1930)?
            };
            let a1932 = v4;
            if nat_eq(a1931, a1932) {
                Ok(Adt9::C4(19u64, 1u64, 1u64, 2u64, 0u64, 3u64, f126(v5, v2)?))
            } else {
                let v7: u64 = {
                    let a1933 = v6;
                    let a1934 = 2u64;
                    nat_sub(a1933, a1934)
                };
                let v8: u64 = {
                    let a1935 = v7;
                    let a1936 = 3u64;
                    let a1937 = 0u64;
                    nat_quot(a1935, a1936, a1937)
                };
                let v9: u64 = {
                    let a1938 = v7;
                    let a1939 = 3u64;
                    let a1940 = 0u64;
                    nat_rem(a1938, a1939, a1940)
                };
                let v10: u64 = f126({
                    let a1944 = {
                        let a1942 = v5;
                        let a1943 = match v1.clone() {
                            Adt3::C0(h1941, _, _, _, _, _, _, _) => {
                                h1941
                            }
                        };
                        nat_mul(a1942, a1943)?
                    };
                    let a1945 = v8;
                    nat_add(a1944, a1945)?
                }, v1)?;
                let a1946 = v9;
                let a1947 = 0u64;
                if nat_eq(a1946, a1947) {
                    Ok(Adt9::C4(16u64, 1u64, 0u64, 0u64, v10, 1u64, 2u64))
                } else {
                    let a1948 = v9;
                    let a1949 = 1u64;
                    if nat_eq(a1948, a1949) {
                        let a1950 = v3;
                        let a1951 = 2u64;
                        if nat_eq(a1950, a1951) {
                            Ok(Adt9::C4(19u64, 1u64, 0u64, 2u64, 0u64, 1u64, 0u64))
                        } else {
                            Ok(Adt9::C4(19u64, 0u64, v10, 2u64, 0u64, 1u64, 0u64))
                        }
                    } else {
                        let a1952 = v3;
                        let a1953 = 2u64;
                        if if nat_eq(a1952, a1953) {
                            true
                        } else {
                            let a1954 = v3;
                            let a1955 = 3u64;
                            nat_eq(a1954, a1955)
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

pub fn f71(v0: u64, v1: Adt3, v2: Adt3, v3: Adt3, v4: u64) -> R<Adt9> {
    let v5: u64 = {
        let a1957 = 2u64;
        let a1958 = match v1.clone() {
            Adt3::C0(_, h1956, _, _, _, _, _, _) => {
                h1956
            }
        };
        nat_mul(a1957, a1958)?
    };
    let a1959 = v0;
    let a1960 = v5;
    if nat_lt(a1959, a1960) {
        let a1964 = {
            let a1961 = v0;
            let a1962 = 2u64;
            let a1963 = 0u64;
            nat_rem(a1961, a1962, a1963)
        };
        let a1965 = 0u64;
        if nat_eq(a1964, a1965) {
            Ok(Adt9::C4(19u64, 4u64, f126({
                let a1966 = v0;
                let a1967 = 2u64;
                let a1968 = 0u64;
                nat_quot(a1966, a1967, a1968)
            }, v3)?, 2u64, 0u64, 1u64, 2u64))
        } else {
            let a1969 = v4;
            let a1970 = 2147483648u64;
            if if nat_lt(a1969, a1970) {
                let a1972 = v4;
                let a1973 = match v2 {
                    Adt3::C0(_, h1971, _, _, _, _, _, _) => {
                        h1971
                    }
                };
                nat_lt(a1972, a1973)
            } else {
                false
            } {
                Ok(Adt9::C4(19u64, 2u64, 0u64, 2u64, 0u64, 1u64, 0u64))
            } else {
                Ok(Adt9::C0(Adt10::C1))
            }
        }
    } else {
        let v6: u64 = {
            let a1976 = match v2.clone() {
                Adt3::C0(h1974, _, _, _, _, _, _, _) => {
                    h1974
                }
            };
            let a1977 = match v2.clone() {
                Adt3::C0(_, h1975, _, _, _, _, _, _) => {
                    h1975
                }
            };
            nat_mul(a1976, a1977)?
        };
        let a1980 = {
            let a1978 = v0;
            let a1979 = v5;
            nat_sub(a1978, a1979)
        };
        let a1981 = v6;
        if nat_lt(a1980, a1981) {
            Ok(Adt9::C4(19u64, 2u64, 0u64, 2u64, 0u64, 3u64, f126({
                let a1982 = v0;
                let a1983 = v5;
                nat_sub(a1982, a1983)
            }, v2)?))
        } else {
            let v7: u64 = {
                let a1986 = {
                    let a1984 = v0;
                    let a1985 = v5;
                    nat_sub(a1984, a1985)
                };
                let a1987 = v6;
                nat_sub(a1986, a1987)
            };
            let v8: u64 = {
                let a1991 = 1u64;
                let a1992 = {
                    let a1989 = 2u64;
                    let a1990 = match v1.clone() {
                        Adt3::C0(h1988, _, _, _, _, _, _, _) => {
                            h1988
                        }
                    };
                    nat_mul(a1989, a1990)?
                };
                nat_add(a1991, a1992)?
            };
            let v9: u64 = {
                let a1993 = v7;
                let a1994 = v8;
                let a1995 = 0u64;
                nat_quot(a1993, a1994, a1995)
            };
            let v10: u64 = {
                let a1996 = v7;
                let a1997 = v8;
                let a1998 = 0u64;
                nat_rem(a1996, a1997, a1998)
            };
            let a1999 = v10;
            let a2000 = 0u64;
            if nat_eq(a1999, a2000) {
                Ok(Adt9::C4(19u64, 4u64, f126(v9, v3)?, 2u64, 0u64, 1u64, 2u64))
            } else {
                let a2004 = {
                    let a2001 = v10;
                    let a2002 = 2u64;
                    let a2003 = 0u64;
                    nat_rem(a2001, a2002, a2003)
                };
                let a2005 = 1u64;
                if nat_eq(a2004, a2005) {
                    Ok(Adt9::C4(6u64, 0u64, f126({
                        let a2014 = {
                            let a2007 = v9;
                            let a2008 = match v1.clone() {
                                Adt3::C0(h2006, _, _, _, _, _, _, _) => {
                                    h2006
                                }
                            };
                            nat_mul(a2007, a2008)?
                        };
                        let a2015 = {
                            let a2011 = {
                                let a2009 = v10;
                                let a2010 = 1u64;
                                nat_sub(a2009, a2010)
                            };
                            let a2012 = 2u64;
                            let a2013 = 0u64;
                            nat_quot(a2011, a2012, a2013)
                        };
                        nat_add(a2014, a2015)?
                    }, v1)?, 2u64, 0u64, 1u64, 0u64))
                } else {
                    Ok(Adt9::C4(4u64, 3u64, f126({
                        let a2024 = {
                            let a2017 = v4;
                            let a2018 = match v1.clone() {
                                Adt3::C0(h2016, _, _, _, _, _, _, _) => {
                                    h2016
                                }
                            };
                            nat_mul(a2017, a2018)?
                        };
                        let a2025 = {
                            let a2021 = {
                                let a2019 = v10;
                                let a2020 = 1u64;
                                nat_sub(a2019, a2020)
                            };
                            let a2022 = 2u64;
                            let a2023 = 0u64;
                            nat_quot(a2021, a2022, a2023)
                        };
                        nat_add(a2024, a2025)?
                    }, v2.clone())?, 1u64, 0u64, 3u64, f126({
                        let a2034 = {
                            let a2027 = v4;
                            let a2028 = match v1 {
                                Adt3::C0(h2026, _, _, _, _, _, _, _) => {
                                    h2026
                                }
                            };
                            nat_mul(a2027, a2028)?
                        };
                        let a2035 = {
                            let a2031 = {
                                let a2029 = v10;
                                let a2030 = 1u64;
                                nat_sub(a2029, a2030)
                            };
                            let a2032 = 2u64;
                            let a2033 = 0u64;
                            nat_quot(a2031, a2032, a2033)
                        };
                        nat_add(a2034, a2035)?
                    }, v2)?))
                }
            }
        }
    }
}

pub fn f72(v0: u64, v1: u64, v2: Adt3, v3: Adt3, v4: Adt3, v5: u64) -> R<Adt9> {
    let a2036 = v1;
    let a2037 = f141(v2.clone(), v4.clone())?;
    if nat_lt(a2036, a2037) {
        let a2040 = v1;
        let a2041 = {
            let a2038 = 2u64;
            let a2039 = f115(v4.clone())?;
            nat_mul(a2038, a2039)?
        };
        if nat_lt(a2040, a2041) {
            let a2045 = {
                let a2042 = v1;
                let a2043 = 2u64;
                let a2044 = 0u64;
                nat_rem(a2042, a2043, a2044)
            };
            let a2046 = 0u64;
            if nat_eq(a2045, a2046) {
                f142({
                    let a2047 = v1;
                    let a2048 = 2u64;
                    let a2049 = 0u64;
                    nat_quot(a2047, a2048, a2049)
                }, v4)
            } else {
                if f143(match v2 {
                    Adt3::C0(_, h2050, _, _, _, _, _, _) => {
                        h2050
                    }
                }, v5) {
                    Ok(Adt9::C4(19u64, 1u64, 2u64, 2u64, 0u64, 1u64, 2u64))
                } else {
                    Ok(Adt9::C0(Adt10::C1))
                }
            }
        } else {
            let v6: u64 = {
                let a2053 = v1;
                let a2054 = {
                    let a2051 = 2u64;
                    let a2052 = f115(v4.clone())?;
                    nat_mul(a2051, a2052)?
                };
                nat_sub(a2053, a2054)
            };
            let v7: u64 = {
                let a2058 = v6;
                let a2059 = {
                    let a2056 = 1u64;
                    let a2057 = match v2.clone() {
                        Adt3::C0(h2055, _, _, _, _, _, _, _) => {
                            h2055
                        }
                    };
                    nat_add(a2056, a2057)?
                };
                let a2060 = 0u64;
                nat_quot(a2058, a2059, a2060)
            };
            let v8: u64 = {
                let a2064 = v6;
                let a2065 = {
                    let a2062 = 1u64;
                    let a2063 = match v2.clone() {
                        Adt3::C0(h2061, _, _, _, _, _, _, _) => {
                            h2061
                        }
                    };
                    nat_add(a2062, a2063)?
                };
                let a2066 = 0u64;
                nat_rem(a2064, a2065, a2066)
            };
            let a2067 = v8;
            let a2068 = 0u64;
            if nat_eq(a2067, a2068) {
                f142(v7, v4)
            } else {
                f144(v0, v7, {
                    let a2069 = v8;
                    let a2070 = 1u64;
                    nat_sub(a2069, a2070)
                }, v2, v3, v4, v5)
            }
        }
    } else {
        Ok(Adt9::C0(Adt10::C1))
    }
}

pub fn f73(v0: u64, v1: Adt3, v2: Adt3, v3: Adt3, v4: Adt3, v5: u64) -> R<Adt9> {
    let v6: u64 = {
        let a2073 = match v4.clone() {
            Adt3::C0(h2071, _, _, _, _, _, _, _) => {
                h2071
            }
        };
        let a2074 = match v4.clone() {
            Adt3::C0(_, h2072, _, _, _, _, _, _) => {
                h2072
            }
        };
        nat_mul(a2073, a2074)?
    };
    let v7: u64 = {
        let a2075 = 2u64;
        let a2076 = v6;
        nat_mul(a2075, a2076)?
    };
    let v8: u64 = f145(match v3.clone() {
        Adt3::C0(_, h2077, _, _, _, _, _, _) => {
            h2077
        }
    }, match v3.clone() {
        Adt3::C0(_, _, h2078, _, _, _, _, _) => {
            h2078
        }
    }, match v3.clone() {
        Adt3::C0(_, _, _, h2079, _, _, _, _) => {
            h2079
        }
    });
    let a2080 = v0;
    let a2081 = v7;
    if nat_lt(a2080, a2081) {
        let a2085 = {
            let a2082 = v0;
            let a2083 = 2u64;
            let a2084 = 0u64;
            nat_rem(a2082, a2083, a2084)
        };
        let a2086 = 0u64;
        if nat_eq(a2085, a2086) {
            Ok(Adt9::C4(19u64, 5u64, f126({
                let a2087 = v0;
                let a2088 = 2u64;
                let a2089 = 0u64;
                nat_quot(a2087, a2088, a2089)
            }, v4)?, 2u64, 0u64, 1u64, 2u64))
        } else {
            if f146(v5, v8) {
                Ok(Adt9::C4(19u64, 1u64, 2u64, 2u64, 0u64, 1u64, 2u64))
            } else {
                Ok(Adt9::C0(Adt10::C1))
            }
        }
    } else {
        let v9: u64 = {
            let a2090 = v0;
            let a2091 = v7;
            nat_sub(a2090, a2091)
        };
        let v10: u64 = {
            let a2095 = v9;
            let a2096 = {
                let a2093 = 1u64;
                let a2094 = match v1.clone() {
                    Adt3::C0(h2092, _, _, _, _, _, _, _) => {
                        h2092
                    }
                };
                nat_add(a2093, a2094)?
            };
            let a2097 = 0u64;
            nat_quot(a2095, a2096, a2097)
        };
        let v11: u64 = {
            let a2101 = v9;
            let a2102 = {
                let a2099 = 1u64;
                let a2100 = match v1.clone() {
                    Adt3::C0(h2098, _, _, _, _, _, _, _) => {
                        h2098
                    }
                };
                nat_add(a2099, a2100)?
            };
            let a2103 = 0u64;
            nat_rem(a2101, a2102, a2103)
        };
        let a2104 = v11;
        let a2105 = 0u64;
        if nat_eq(a2104, a2105) {
            Ok(Adt9::C4(19u64, 5u64, f126({
                let a2106 = v10;
                let a2107 = v6;
                let a2108 = 0u64;
                nat_rem(a2106, a2107, a2108)
            }, v4)?, 2u64, 0u64, 1u64, 2u64))
        } else {
            if f146(v5, v8) {
                let v12: u64 = {
                    let a2114 = {
                        let a2110 = v10;
                        let a2111 = match v1.clone() {
                            Adt3::C0(h2109, _, _, _, _, _, _, _) => {
                                h2109
                            }
                        };
                        nat_mul(a2110, a2111)?
                    };
                    let a2115 = {
                        let a2112 = v11;
                        let a2113 = 1u64;
                        nat_sub(a2112, a2113)
                    };
                    nat_add(a2114, a2115)?
                };
                let a2116 = v5;
                let a2117 = 4294967295u64;
                if nat_eq(a2116, a2117) {
                    Ok(Adt9::C1(false, f126(v12, v1)?, f126(v12, v2)?))
                } else {
                    Ok(Adt9::C4(8u64, 0u64, f126(v12, v1)?, 4u64, f126({
                        let a2123 = {
                            let a2119 = v5;
                            let a2120 = match v3.clone() {
                                Adt3::C0(h2118, _, _, _, _, _, _, _) => {
                                    h2118
                                }
                            };
                            nat_mul(a2119, a2120)?
                        };
                        let a2124 = {
                            let a2121 = v11;
                            let a2122 = 1u64;
                            nat_sub(a2121, a2122)
                        };
                        nat_add(a2123, a2124)?
                    }, v3)?, 3u64, f126(v12, v2)?))
                }
            } else {
                Ok(Adt9::C0(Adt10::C1))
            }
        }
    }
}

pub fn f74(v0: u64, v1: Adt3, v2: Adt3, v3: Adt7, v4: u64) -> R<Adt9> {
    let v5: u64 = f100(v3.clone())?;
    let v6: u64 = {
        let a2125 = v0;
        let a2126 = v5;
        let a2127 = 0u64;
        nat_quot(a2125, a2126, a2127)
    };
    let v7: u64 = {
        let a2128 = v0;
        let a2129 = v5;
        let a2130 = 0u64;
        nat_rem(a2128, a2129, a2130)
    };
    let v8: u64 = f147(v3.clone())?;
    let a2131 = v7;
    let a2132 = 0u64;
    if nat_eq(a2131, a2132) {
        Ok(Adt9::C4(19u64, 2u64, if match v3 {
            Adt7::C0(_, _, _, h2133, _) => {
                h2133
            }
        } {
            4286578687u64
        } else {
            0u64
        }, 2u64, 0u64, 1u64, 0u64))
    } else {
        let a2137 = v5;
        let a2138 = {
            let a2135 = v7;
            let a2136 = if match v3.clone() {
                Adt7::C0(_, _, _, h2134, _) => {
                    h2134
                }
            } {
                1u64
            } else {
                2u64
            };
            nat_add(a2135, a2136)?
        };
        if nat_le(a2137, a2138) {
            if match v3.clone() {
                Adt7::C0(_, _, _, h2139, _) => {
                    h2139
                }
            } {
                Ok(Adt9::C4(19u64, 1u64, 0u64, 2u64, 0u64, 3u64, f126(v6, v2)?))
            } else {
                let a2142 = {
                    let a2140 = v7;
                    let a2141 = 2u64;
                    nat_add(a2140, a2141)?
                };
                let a2143 = v5;
                if nat_eq(a2142, a2143) {
                    let v9: u64 = if match v3.clone() {
                        Adt7::C0(_, _, h2144, _, _) => {
                            h2144
                        }
                    } {
                        let a2151 = {
                            let m2147 = match v3.clone() {
                                Adt7::C0(h2146, _, _, _, _) => {
                                    h2146
                                }
                            };
                            match m2147 {
                                Adt6::C0(h2145, _, _, _) => {
                                    h2145
                                }
                            }
                        };
                        let a2152 = {
                            let m2150 = match v3 {
                                Adt7::C0(_, h2149, _, _, _) => {
                                    h2149
                                }
                            };
                            match m2150 {
                                Adt6::C0(h2148, _, _, _) => {
                                    h2148
                                }
                            }
                        };
                        nat_mul(a2151, a2152)?
                    } else {
                        f148(match v3 {
                            Adt7::C0(h2153, _, _, _, _) => {
                                h2153
                            }
                        }, match v1 {
                            Adt3::C0(h2154, _, _, _, _, _, _, _) => {
                                h2154
                            }
                        }, f64(v6, v2, 0u64))?
                    };
                    Ok(Adt9::C4(3u64, 2u64, {
                        let a2155 = v9;
                        let a2156 = 0u64;
                        if nat_eq(a2155, a2156) {
                            1u64
                        } else {
                            v9
                        }
                    }, 2u64, 0u64, 1u64, 1u64))
                } else {
                    Ok(Adt9::C4(5u64, 1u64, 0u64, 1u64, 1u64, 3u64, f126(v6, v2)?))
                }
            }
        } else {
            let v10: u64 = {
                let a2159 = {
                    let a2157 = v7;
                    let a2158 = 1u64;
                    nat_sub(a2157, a2158)
                };
                let a2160 = v8;
                let a2161 = 0u64;
                nat_rem(a2159, a2160, a2161)
            };
            let m2167 = f149(v3.clone(), v1, v2, v6, {
                let a2164 = {
                    let a2162 = v7;
                    let a2163 = 1u64;
                    nat_sub(a2162, a2163)
                };
                let a2165 = v8;
                let a2166 = 0u64;
                nat_quot(a2164, a2165, a2166)
            })?;
            match m2167 {
                None => {
                    Ok(Adt9::C4(19u64, 1u64, 0u64, 2u64, 0u64, 1u64, 0u64))
                }
                Some(v11) => {
                    let a2169 = match v3.clone() {
                        Adt7::C0(_, _, _, _, h2168) => {
                            h2168
                        }
                    };
                    let a2170 = 1u64;
                    if if nat_eq(a2169, a2170) {
                        let a2171 = v10;
                        let a2172 = 0u64;
                        nat_eq(a2171, a2172)
                    } else {
                        false
                    } {
                        Ok(Adt9::C4(6u64, 0u64, v11, 2u64, 0u64, 1u64, 1u64))
                    } else {
                        if match v3.clone() {
                            Adt7::C0(_, _, _, h2173, _) => {
                                h2173
                            }
                        } {
                            let a2176 = {
                                let a2174 = v10;
                                let a2175 = 1u64;
                                nat_add(a2174, a2175)?
                            };
                            let a2177 = v8;
                            if nat_eq(a2176, a2177) {
                                let a2178 = v4;
                                let a2179 = 2u64;
                                if nat_eq(a2178, a2179) {
                                    Ok(Adt9::C4(19u64, 1u64, 0u64, 2u64, 0u64, 1u64, 0u64))
                                } else {
                                    Ok(Adt9::C4(19u64, {
                                        let a2181 = match v3.clone() {
                                            Adt7::C0(_, _, _, _, h2180) => {
                                                h2180
                                            }
                                        };
                                        let a2182 = 1u64;
                                        if nat_eq(a2181, a2182) {
                                            1u64
                                        } else {
                                            0u64
                                        }
                                    }, {
                                        let a2184 = match v3 {
                                            Adt7::C0(_, _, _, _, h2183) => {
                                                h2183
                                            }
                                        };
                                        let a2185 = 1u64;
                                        if nat_eq(a2184, a2185) {
                                            1u64
                                        } else {
                                            v11
                                        }
                                    }, 2u64, 0u64, 1u64, 0u64))
                                }
                            } else {
                                Ok(Adt9::C4(16u64, 1u64, 0u64, {
                                    let a2187 = match v3.clone() {
                                        Adt7::C0(_, _, _, _, h2186) => {
                                            h2186
                                        }
                                    };
                                    let a2188 = 1u64;
                                    if nat_eq(a2187, a2188) {
                                        1u64
                                    } else {
                                        0u64
                                    }
                                }, {
                                    let a2190 = match v3 {
                                        Adt7::C0(_, _, _, _, h2189) => {
                                            h2189
                                        }
                                    };
                                    let a2191 = 1u64;
                                    if nat_eq(a2190, a2191) {
                                        1u64
                                    } else {
                                        v11
                                    }
                                }, 1u64, 2u64))
                            }
                        } else {
                            Ok(Adt9::C4(4u64, 1u64, 0u64, {
                                let a2193 = match v3.clone() {
                                    Adt7::C0(_, _, _, _, h2192) => {
                                        h2192
                                    }
                                };
                                let a2194 = 1u64;
                                if nat_eq(a2193, a2194) {
                                    1u64
                                } else {
                                    0u64
                                }
                            }, {
                                let a2196 = match v3 {
                                    Adt7::C0(_, _, _, _, h2195) => {
                                        h2195
                                    }
                                };
                                let a2197 = 1u64;
                                if nat_eq(a2196, a2197) {
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

pub fn f75(v0: u64, v1: Adt3, v2: Adt3, v3: Adt3) -> R<Adt9> {
    let v4: u64 = f98(match v3.clone() {
        Adt3::C0(h2198, _, _, _, _, _, _, _) => {
            h2198
        }
    })?;
    let v5: u64 = {
        let a2199 = v0;
        let a2200 = v4;
        let a2201 = 0u64;
        nat_rem(a2199, a2200, a2201)
    };
    let v6: u64 = {
        let a2202 = v0;
        let a2203 = v4;
        let a2204 = 0u64;
        nat_quot(a2202, a2203, a2204)
    };
    let a2205 = v5;
    let a2206 = 0u64;
    if nat_eq(a2205, a2206) {
        Ok(Adt9::C4(19u64, 2u64, 0u64, 2u64, 0u64, 1u64, 0u64))
    } else {
        let a2209 = {
            let a2207 = v5;
            let a2208 = 1u64;
            nat_add(a2207, a2208)?
        };
        let a2210 = v4;
        if nat_eq(a2209, a2210) {
            Ok(Adt9::C4(19u64, 1u64, 0u64, 2u64, 0u64, 3u64, f126(v6, v2)?))
        } else {
            let a2214 = {
                let a2211 = v5;
                let a2212 = 2u64;
                let a2213 = 0u64;
                nat_rem(a2211, a2212, a2213)
            };
            let a2215 = 0u64;
            if nat_eq(a2214, a2215) {
                Ok(Adt9::C4(4u64, 1u64, 0u64, 1u64, 1u64, 1u64, 0u64))
            } else {
                let v7: u64 = {
                    let a2218 = {
                        let a2216 = v5;
                        let a2217 = 1u64;
                        nat_sub(a2216, a2217)
                    };
                    let a2219 = 2u64;
                    let a2220 = 0u64;
                    nat_quot(a2218, a2219, a2220)
                };
                let v8: u64 = f64(v6, v2.clone(), 0u64);
                let v9: u64 = f64(v6, v2.clone(), 1u64);
                let v10: u64 = f64(v6, v2, 2u64);
                Ok(Adt9::C4(10u64, 0u64, f63(v1, {
                    let a2221 = v9;
                    let a2222 = v7;
                    nat_add(a2221, a2222)?
                }, v8, v10, 0u64)?, 4u64, f63(v3, v7, v8, 0u64, 0u64)?, 1u64, 1u64))
            }
        }
    }
}

pub fn f76(v0: u64, v1: Adt3, v2: Adt3, v3: Adt6) -> R<Adt9> {
    let v4: u64 = f97(v3.clone())?;
    let v5: u64 = {
        let a2223 = v0;
        let a2224 = v4;
        let a2225 = 0u64;
        nat_rem(a2223, a2224, a2225)
    };
    let v6: u64 = {
        let a2226 = v0;
        let a2227 = v4;
        let a2228 = 0u64;
        nat_quot(a2226, a2227, a2228)
    };
    let a2229 = v5;
    let a2230 = 0u64;
    if nat_eq(a2229, a2230) {
        Ok(Adt9::C4(19u64, 2u64, 0u64, 2u64, 0u64, 1u64, 0u64))
    } else {
        let a2233 = {
            let a2231 = v5;
            let a2232 = 1u64;
            nat_add(a2231, a2232)?
        };
        let a2234 = v4;
        if nat_eq(a2233, a2234) {
            Ok(Adt9::C4(7u64, 1u64, 0u64, 2u64, 0u64, 3u64, f126(v6, v2)?))
        } else {
            let a2238 = {
                let a2235 = v5;
                let a2236 = 2u64;
                let a2237 = 0u64;
                nat_rem(a2235, a2236, a2237)
            };
            let a2239 = 0u64;
            if nat_eq(a2238, a2239) {
                Ok(Adt9::C4(4u64, 1u64, 0u64, 1u64, 1u64, 1u64, 0u64))
            } else {
                let v7: u64 = {
                    let a2241 = f64(v6, v2.clone(), 0u64);
                    let a2242 = match v3.clone() {
                        Adt6::C0(_, _, h2240, _) => {
                            h2240
                        }
                    };
                    nat_add(a2241, a2242)?
                };
                let v8: u64 = {
                    let a2250 = {
                        let a2247 = {
                            let a2244 = v7;
                            let a2245 = match v3.clone() {
                                Adt6::C0(_, h2243, _, _) => {
                                    h2243
                                }
                            };
                            nat_add(a2244, a2245)?
                        };
                        let a2248 = match v3.clone() {
                            Adt6::C0(h2246, _, _, _) => {
                                h2246
                            }
                        };
                        nat_sub(a2247, a2248)
                    };
                    let a2251 = match v3.clone() {
                        Adt6::C0(_, h2249, _, _) => {
                            h2249
                        }
                    };
                    let a2252 = 0u64;
                    nat_quot(a2250, a2251, a2252)
                };
                let v9: u64 = {
                    let a2258 = v8;
                    let a2259 = {
                        let a2255 = {
                            let a2253 = v5;
                            let a2254 = 1u64;
                            nat_sub(a2253, a2254)
                        };
                        let a2256 = 2u64;
                        let a2257 = 0u64;
                        nat_quot(a2255, a2256, a2257)
                    };
                    nat_add(a2258, a2259)?
                };
                let v10: u64 = {
                    let a2261 = v9;
                    let a2262 = match v3.clone() {
                        Adt6::C0(_, h2260, _, _) => {
                            h2260
                        }
                    };
                    nat_mul(a2261, a2262)?
                };
                let v11: u64 = {
                    let a2263 = v7;
                    let a2264 = v10;
                    nat_sub(a2263, a2264)
                };
                let a2266 = v9;
                let a2267 = match v1.clone() {
                    Adt3::C0(_, h2265, _, _, _, _, _, _) => {
                        h2265
                    }
                };
                if if if nat_lt(a2266, a2267) {
                    let a2268 = v10;
                    let a2269 = v7;
                    nat_le(a2268, a2269)
                } else {
                    false
                } {
                    let a2271 = v11;
                    let a2272 = match v3.clone() {
                        Adt6::C0(h2270, _, _, _) => {
                            h2270
                        }
                    };
                    nat_lt(a2271, a2272)
                } else {
                    false
                } {
                    Ok(Adt9::C4(6u64, 0u64, f63(v1, {
                        let a2276 = {
                            let a2274 = f64(v6, v2, 1u64);
                            let a2275 = match v3 {
                                Adt6::C0(h2273, _, _, _) => {
                                    h2273
                                }
                            };
                            nat_mul(a2274, a2275)?
                        };
                        let a2277 = v11;
                        nat_add(a2276, a2277)?
                    }, v9, 0u64, 0u64)?, 2u64, 0u64, 1u64, 1u64))
                } else {
                    Ok(Adt9::C4(19u64, 2u64, 0u64, 2u64, 0u64, 1u64, 1u64))
                }
            }
        }
    }
}

pub fn f77(v0: u64, v1: Adt3, v2: Adt3, v3: Adt5) -> R<Adt9> {
    let v4: u64 = f64(v0, v2.clone(), 0u64);
    let v5: u64 = f64(v0, v2.clone(), 3u64);
    let v6: u64 = {
        let a2279 = v5;
        let a2280 = match v3.clone() {
            Adt5::C0(_, _, _, _, _, _, h2278, _) => {
                h2278
            }
        };
        let a2281 = 0u64;
        nat_rem(a2279, a2280, a2281)
    };
    let v7: u64 = {
        let a2283 = v5;
        let a2284 = match v3.clone() {
            Adt5::C0(_, _, _, _, _, _, h2282, _) => {
                h2282
            }
        };
        let a2285 = 0u64;
        nat_quot(a2283, a2284, a2285)
    };
    let v8: Option<u64> = f66(match v3.clone() {
        Adt5::C0(h2286, _, _, _, _, _, _, _) => {
            h2286
        }
    }, match v1.clone() {
        Adt3::C0(h2287, _, _, _, _, _, _, _) => {
            h2287
        }
    }, f64(v0, v2.clone(), 1u64), {
        let a2291 = v4;
        let a2292 = {
            let m2290 = match v3.clone() {
                Adt5::C0(h2289, _, _, _, _, _, _, _) => {
                    h2289
                }
            };
            match m2290 {
                Adt6::C0(h2288, _, _, _) => {
                    h2288
                }
            }
        };
        let a2293 = 0u64;
        nat_rem(a2291, a2292, a2293)
    })?;
    let v9: u64 = {
        let a2297 = v4;
        let a2298 = {
            let m2296 = match v3.clone() {
                Adt5::C0(h2295, _, _, _, _, _, _, _) => {
                    h2295
                }
            };
            match m2296 {
                Adt6::C0(h2294, _, _, _) => {
                    h2294
                }
            }
        };
        let a2299 = 0u64;
        nat_quot(a2297, a2298, a2299)
    };
    let v10: Option<u64> = f66(match v3.clone() {
        Adt5::C0(_, h2300, _, _, _, _, _, _) => {
            h2300
        }
    }, match v1.clone() {
        Adt3::C0(_, h2301, _, _, _, _, _, _) => {
            h2301
        }
    }, f64(v0, v2.clone(), 2u64), {
        let a2305 = v9;
        let a2306 = {
            let m2304 = match v3.clone() {
                Adt5::C0(_, h2303, _, _, _, _, _, _) => {
                    h2303
                }
            };
            match m2304 {
                Adt6::C0(h2302, _, _, _) => {
                    h2302
                }
            }
        };
        let a2307 = 0u64;
        nat_rem(a2305, a2306, a2307)
    })?;
    let v11: u64 = {
        let a2311 = v9;
        let a2312 = {
            let m2310 = match v3.clone() {
                Adt5::C0(_, h2309, _, _, _, _, _, _) => {
                    h2309
                }
            };
            match m2310 {
                Adt6::C0(h2308, _, _, _) => {
                    h2308
                }
            }
        };
        let a2313 = 0u64;
        nat_quot(a2311, a2312, a2313)
    };
    let v12: Option<u64> = f66(match v3.clone() {
        Adt5::C0(_, _, h2314, _, _, _, _, _) => {
            h2314
        }
    }, match v1.clone() {
        Adt3::C0(_, _, h2315, _, _, _, _, _) => {
            h2315
        }
    }, v6, {
        let a2319 = v11;
        let a2320 = {
            let m2318 = match v3.clone() {
                Adt5::C0(_, _, h2317, _, _, _, _, _) => {
                    h2317
                }
            };
            match m2318 {
                Adt6::C0(h2316, _, _, _) => {
                    h2316
                }
            }
        };
        let a2321 = 0u64;
        nat_rem(a2319, a2320, a2321)
    })?;
    let v13: u64 = {
        let a2325 = v11;
        let a2326 = {
            let m2324 = match v3.clone() {
                Adt5::C0(_, _, h2323, _, _, _, _, _) => {
                    h2323
                }
            };
            match m2324 {
                Adt6::C0(h2322, _, _, _) => {
                    h2322
                }
            }
        };
        let a2327 = 0u64;
        nat_quot(a2325, a2326, a2327)
    };
    let v17: Adt12 = {
        let m2328 = v8;
        match m2328 {
            None => {
                Adt12::C2(0u64)
            }
            Some(v14) => {
                let m2329 = v10;
                match m2329 {
                    None => {
                        Adt12::C2(0u64)
                    }
                    Some(v15) => {
                        let m2330 = v12;
                        match m2330 {
                            None => {
                                Adt12::C2(0u64)
                            }
                            Some(v16) => {
                                Adt12::C0(1u64, {
                                    let a2339 = {
                                        let a2334 = f63(v1, v14, v15, v16, 0u64)?;
                                        let a2335 = {
                                            let a2332 = v7;
                                            let a2333 = match v3.clone() {
                                                Adt5::C0(_, _, _, _, h2331, _, _, _) => {
                                                    h2331
                                                }
                                            };
                                            nat_mul(a2332, a2333)?
                                        };
                                        nat_add(a2334, a2335)?
                                    };
                                    let a2340 = {
                                        let a2337 = v13;
                                        let a2338 = match v3.clone() {
                                            Adt5::C0(_, _, _, h2336, _, _, _, _) => {
                                                h2336
                                            }
                                        };
                                        nat_mul(a2337, a2338)?
                                    };
                                    nat_add(a2339, a2340)?
                                })
                            }
                        }
                    }
                }
            }
        }
    };
    Ok(Adt9::C7(Adt11::C2(match v3.clone() {
        Adt5::C0(_, _, _, _, _, h2341, _, _) => {
            h2341
        }
    }, match v3.clone() {
        Adt5::C0(_, _, _, _, _, _, _, h2342) => {
            h2342
        }
    }, true), v17, Adt12::C2(0u64), Adt13::C1(f126(v0, v2)?), {
        let a2344 = match v3.clone() {
            Adt5::C0(_, _, _, _, _, h2343, _, _) => {
                h2343
            }
        };
        let a2345 = 0u64;
        if nat_eq(a2344, a2345) {
            4u64
        } else {
            2u64
        }
    }, {
        let a2347 = match v3 {
            Adt5::C0(_, _, _, _, _, _, _, h2346) => {
                h2346
            }
        };
        let a2348 = 0u64;
        if nat_eq(a2347, a2348) {
            4u64
        } else {
            2u64
        }
    }))
}

pub fn f78(v0: u64, v1: Adt3, v2: Adt3, v3: u64, v4: u64, v5: u64, v6: u64, v7: u64) -> R<Adt9> {
    let v8: u64 = {
        let a2351 = v6;
        let a2352 = {
            let a2349 = 3u64;
            let a2350 = v5;
            nat_add(a2349, a2350)?
        };
        nat_sub(a2351, a2352)
    };
    let a2354 = v0;
    let a2355 = f32(v3, match v1.clone() {
        Adt3::C0(h2353, _, _, _, _, _, _, _) => {
            h2353
        }
    }, v5, v8)?;
    if nat_lt(a2354, a2355) {
        let v9: u64 = f91(match v1.clone() {
            Adt3::C0(h2356, _, _, _, _, _, _, _) => {
                h2356
            }
        }, v5, v8)?;
        let v10: u64 = f92(v3, match v1.clone() {
            Adt3::C0(h2357, _, _, _, _, _, _, _) => {
                h2357
            }
        }, v5)?;
        let a2358 = v0;
        let a2359 = v9;
        if nat_lt(a2358, a2359) {
            f150(v0, v2, f26(v3), {
                let a2360 = 3u64;
                let a2361 = v5;
                nat_add(a2360, a2361)?
            }, v7)
        } else {
            let a2364 = v0;
            let a2365 = {
                let a2362 = v9;
                let a2363 = v10;
                nat_add(a2362, a2363)?
            };
            if nat_lt(a2364, a2365) {
                let v11: u64 = {
                    let a2366 = v0;
                    let a2367 = v9;
                    nat_sub(a2366, a2367)
                };
                let v12: u64 = f151(v3, match v1.clone() {
                    Adt3::C0(h2368, _, _, _, _, _, _, _) => {
                        h2368
                    }
                })?;
                f152({
                    let a2369 = v11;
                    let a2370 = v12;
                    let a2371 = 0u64;
                    nat_rem(a2369, a2370, a2371)
                }, {
                    let a2372 = v11;
                    let a2373 = v12;
                    let a2374 = 0u64;
                    nat_quot(a2372, a2373, a2374)
                }, v1, v3, v4, v7)
            } else {
                f153({
                    let a2377 = v0;
                    let a2378 = {
                        let a2375 = v9;
                        let a2376 = v10;
                        nat_add(a2375, a2376)?
                    };
                    nat_sub(a2377, a2378)
                }, v1, v2, v3)
            }
        }
    } else {
        Ok(Adt9::C0(Adt10::C1))
    }
}

pub fn f79(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64) -> R<Adt9> {
    let a2379 = v0;
    let a2380 = f16(v1)?;
    if nat_lt(a2379, a2380) {
        let a2381 = v0;
        let a2382 = 4u64;
        if nat_lt(a2381, a2382) {
            let a2383 = v0;
            let a2384 = 0u64;
            if nat_eq(a2383, a2384) {
                Ok(Adt9::C4(16u64, 2u64, v2, 2u64, v3, 1u64, 2u64))
            } else {
                let a2385 = v0;
                let a2386 = 1u64;
                if nat_eq(a2385, a2386) {
                    let a2387 = v5;
                    let a2388 = 0u64;
                    if nat_eq(a2387, a2388) {
                        Ok(Adt9::C4(9u64, 2u64, v3, 2u64, v2, 1u64, 2u64))
                    } else {
                        Ok(Adt9::C0(Adt10::C1))
                    }
                } else {
                    let a2389 = v0;
                    let a2390 = 2u64;
                    if nat_eq(a2389, a2390) {
                        Ok(Adt9::C4(11u64, 1u64, 2u64, 2u64, v4, 1u64, 2u64))
                    } else {
                        Ok(Adt9::C4(20u64, 1u64, 2u64, 2u64, 10u64, 1u64, 2u64))
                    }
                }
            }
        } else {
            let a2391 = v0;
            let a2392 = 4u64;
            if if nat_eq(a2391, a2392) {
                let a2393 = f154(v5)?;
                let a2394 = v1;
                !nat_eq(a2393, a2394)
            } else {
                false
            } {
                Ok(Adt9::C0(Adt10::C1))
            } else {
                let v6: u64 = {
                    let a2397 = {
                        let a2395 = v0;
                        let a2396 = 4u64;
                        nat_sub(a2395, a2396)
                    };
                    let a2398 = 3u64;
                    let a2399 = 0u64;
                    nat_quot(a2397, a2398, a2399)
                };
                let v7: u64 = {
                    let a2402 = {
                        let a2400 = v0;
                        let a2401 = 4u64;
                        nat_sub(a2400, a2401)
                    };
                    let a2403 = 3u64;
                    let a2404 = 0u64;
                    nat_rem(a2402, a2403, a2404)
                };
                let a2405 = v7;
                let a2406 = 0u64;
                if nat_eq(a2405, a2406) {
                    Ok(Adt9::C4(3u64, 2u64, v6, 2u64, 0u64, 1u64, 0u64))
                } else {
                    let a2407 = v7;
                    let a2408 = 1u64;
                    if nat_eq(a2407, a2408) {
                        Ok(Adt9::C4(10u64, 2u64, v4, 1u64, 0u64, 1u64, 0u64))
                    } else {
                        Ok(Adt9::C4(8u64, 2u64, v2, 1u64, 0u64, 3u64, f15(v6)?))
                    }
                }
            }
        }
    } else {
        Ok(Adt9::C0(Adt10::C1))
    }
}

pub fn f80(v0: bool, v1: u64, v2: u64, v3: u64, v4: u64) -> Adt11 {
    let a2409 = v1;
    let a2410 = 0u64;
    if nat_eq(a2409, a2410) {
        if v0 {
            Adt11::C2(v3, v4, false)
        } else {
            Adt11::C0(v2)
        }
    } else {
        let a2411 = v1;
        let a2412 = 1u64;
        if nat_eq(a2411, a2412) {
            Adt11::C1(v2)
        } else {
            let a2413 = v1;
            let a2414 = 2u64;
            if nat_eq(a2413, a2414) {
                Adt11::C5(v2)
            } else {
                let a2415 = v1;
                let a2416 = 3u64;
                if nat_eq(a2415, a2416) {
                    Adt11::C3(v2, v2, false)
                } else {
                    let a2417 = v1;
                    let a2418 = 4u64;
                    if nat_eq(a2417, a2418) {
                        Adt11::C2(v3, v4, false)
                    } else {
                        Adt11::C6
                    }
                }
            }
        }
    }
}

pub fn f81(v0: u64, v1: u64) -> Adt12 {
    let a2419 = v0;
    let a2420 = 0u64;
    if nat_eq(a2419, a2420) {
        Adt12::C0(0u64, v1)
    } else {
        let a2421 = v0;
        let a2422 = 1u64;
        if nat_eq(a2421, a2422) {
            Adt12::C1(v1)
        } else {
            let a2423 = v0;
            let a2424 = 2u64;
            if nat_eq(a2423, a2424) {
                Adt12::C2(v1)
            } else {
                let a2425 = v0;
                let a2426 = 3u64;
                if nat_eq(a2425, a2426) {
                    Adt12::C3(v1)
                } else {
                    let a2427 = v0;
                    let a2428 = 13u64;
                    if nat_lt(a2427, a2428) {
                        Adt12::C0({
                            let a2429 = v0;
                            let a2430 = 3u64;
                            nat_sub(a2429, a2430)
                        }, v1)
                    } else {
                        Adt12::C4
                    }
                }
            }
        }
    }
}

pub fn f82(v0: u64, v1: u64) -> Adt13 {
    let a2431 = v0;
    let a2432 = 1u64;
    if nat_eq(a2431, a2432) {
        Adt13::C0(v1)
    } else {
        let a2433 = v0;
        let a2434 = 3u64;
        if if nat_eq(a2433, a2434) {
            true
        } else {
            let a2435 = v0;
            let a2436 = 6u64;
            nat_eq(a2435, a2436)
        } {
            Adt13::C1(v1)
        } else {
            Adt13::C2
        }
    }
}

pub fn f83(v0: Adt6, v1: u64) -> R<u64> {
    let a2438 = 0u64;
    let a2439 = match v0.clone() {
        Adt6::C0(h2437, _, _, _) => {
            h2437
        }
    };
    if if if if if nat_lt(a2438, a2439) {
        let a2441 = 0u64;
        let a2442 = match v0.clone() {
            Adt6::C0(_, h2440, _, _) => {
                h2440
            }
        };
        nat_lt(a2441, a2442)
    } else {
        false
    } {
        let a2444 = 0u64;
        let a2445 = match v0.clone() {
            Adt6::C0(_, _, _, h2443) => {
                h2443
            }
        };
        nat_lt(a2444, a2445)
    } else {
        false
    } {
        let a2447 = match v0.clone() {
            Adt6::C0(_, h2446, _, _) => {
                h2446
            }
        };
        let a2448 = 2147483648u64;
        nat_lt(a2447, a2448)
    } else {
        false
    } {
        let a2450 = match v0.clone() {
            Adt6::C0(_, _, _, h2449) => {
                h2449
            }
        };
        let a2451 = 2147483648u64;
        nat_lt(a2450, a2451)
    } else {
        false
    } {
        let v2: u64 = {
            let a2455 = v1;
            let a2456 = {
                let a2453 = 2u64;
                let a2454 = f120(match v0.clone() {
                    Adt6::C0(_, _, h2452, _) => {
                        h2452
                    }
                });
                nat_mul(a2453, a2454)?
            };
            nat_add(a2455, a2456)?
        };
        let v3: u64 = {
            let a2468 = {
                let a2463 = 1u64;
                let a2464 = {
                    let a2461 = match v0.clone() {
                        Adt6::C0(_, _, _, h2457) => {
                            h2457
                        }
                    };
                    let a2462 = {
                        let a2459 = match v0.clone() {
                            Adt6::C0(h2458, _, _, _) => {
                                h2458
                            }
                        };
                        let a2460 = 1u64;
                        nat_sub(a2459, a2460)
                    };
                    nat_mul(a2461, a2462)?
                };
                nat_add(a2463, a2464)?
            };
            let a2469 = {
                let a2466 = 2u64;
                let a2467 = f119(match v0.clone() {
                    Adt6::C0(_, _, h2465, _) => {
                        h2465
                    }
                });
                nat_mul(a2466, a2467)?
            };
            nat_add(a2468, a2469)?
        };
        let a2470 = v3;
        let a2471 = v2;
        if nat_le(a2470, a2471) {
            let a2478 = 1u64;
            let a2479 = {
                let a2475 = {
                    let a2472 = v2;
                    let a2473 = v3;
                    nat_sub(a2472, a2473)
                };
                let a2476 = match v0 {
                    Adt6::C0(_, h2474, _, _) => {
                        h2474
                    }
                };
                let a2477 = 0u64;
                nat_quot(a2475, a2476, a2477)
            };
            nat_add(a2478, a2479)
        } else {
            let a2483 = {
                let a2480 = v3;
                let a2481 = v2;
                nat_sub(a2480, a2481)
            };
            let a2484 = match v0 {
                Adt6::C0(_, h2482, _, _) => {
                    h2482
                }
            };
            if nat_lt(a2483, a2484) {
                Ok(1u64)
            } else {
                Ok(0u64)
            }
        }
    } else {
        Ok(0u64)
    }
}

pub fn f84(v0: u64) -> u64 {
    let v1: u64 = {
        let a2485 = v0;
        let a2486 = 4294967296u64;
        let a2487 = 0u64;
        nat_rem(a2485, a2486, a2487)
    };
    let a2488 = v1;
    let a2489 = 2147483648u64;
    if nat_lt(a2488, a2489) {
        v1
    } else {
        0u64
    }
}

pub fn f85(v0: u64) -> R<u64> {
    let a2492 = 2u64;
    let a2493 = {
        let a2490 = 2u64;
        let a2491 = v0;
        nat_mul(a2490, a2491)?
    };
    nat_add(a2492, a2493)
}

pub fn f86(v0: u64, v1: Adt14, v2: Adt14) -> R<Adt15> {
    let a2495 = match v1.clone() {
        Adt14::C0(h2494, _, _, _, _, _, _, _, _, _, _) => {
            h2494
        }
    };
    let a2496 = 0u64;
    if if if if nat_eq(a2495, a2496) {
        let a2498 = match v2.clone() {
            Adt14::C0(h2497, _, _, _, _, _, _, _, _, _, _) => {
                h2497
            }
        };
        let a2499 = 0u64;
        nat_eq(a2498, a2499)
    } else {
        false
    } {
        f155(v1.clone(), v2.clone())
    } else {
        false
    } {
        if f156(match v1.clone() {
            Adt14::C0(_, h2500, _, _, _, _, _, _, _, _, _) => {
                h2500
            }
        }, match v1.clone() {
            Adt14::C0(_, _, h2501, _, _, _, _, _, _, _, _) => {
                h2501
            }
        }, match v1.clone() {
            Adt14::C0(_, _, _, h2502, _, _, _, _, _, _, _) => {
                h2502
            }
        }, match v1.clone() {
            Adt14::C0(_, _, _, _, h2503, _, _, _, _, _, _) => {
                h2503
            }
        }) {
            true
        } else {
            if if if if f27(v0) {
                f157(v1.clone(), v2.clone())
            } else {
                false
            } {
                f158(v1.clone())
            } else {
                false
            } {
                f158(v2.clone())
            } else {
                false
            } {
                if f159(v1.clone(), v2.clone())? {
                    true
                } else {
                    f160(v1.clone(), v2.clone())
                }
            } else {
                false
            }
        }
    } else {
        false
    } {
        let v3: u64 = if f156(match v1.clone() {
            Adt14::C0(_, h2504, _, _, _, _, _, _, _, _, _) => {
                h2504
            }
        }, match v1.clone() {
            Adt14::C0(_, _, h2505, _, _, _, _, _, _, _, _) => {
                h2505
            }
        }, match v1.clone() {
            Adt14::C0(_, _, _, h2506, _, _, _, _, _, _, _) => {
                h2506
            }
        }, match v1.clone() {
            Adt14::C0(_, _, _, _, h2507, _, _, _, _, _, _) => {
                h2507
            }
        }) {
            0u64
        } else {
            let a2513 = match v1.clone() {
                Adt14::C0(_, _, h2508, _, _, _, _, _, _, _, _) => {
                    h2508
                }
            };
            let a2514 = {
                let a2511 = match v1.clone() {
                    Adt14::C0(_, _, _, h2509, _, _, _, _, _, _, _) => {
                        h2509
                    }
                };
                let a2512 = match v1.clone() {
                    Adt14::C0(_, _, _, _, h2510, _, _, _, _, _, _) => {
                        h2510
                    }
                };
                nat_mul(a2511, a2512)?
            };
            nat_mul(a2513, a2514)?
        };
        let v4: u64 = {
            let a2515 = v3;
            let a2516 = 0u64;
            if if nat_eq(a2515, a2516) {
                true
            } else {
                f161(v2.clone())?
            } {
                0u64
            } else {
                let a2518 = match v2.clone() {
                    Adt14::C0(_, _, _, _, _, _, _, _, _, h2517, _) => {
                        h2517
                    }
                };
                let a2519 = 4u64;
                let a2520 = 0u64;
                nat_quot(a2518, a2519, a2520)
            }
        };
        Ok(Adt15::C1(f162(f163(match v1.clone() {
            Adt14::C0(_, _, _, _, _, h2521, _, _, _, _, _) => {
                h2521
            }
        }, match v2 {
            Adt14::C0(_, _, _, _, _, h2522, _, _, _, _, _) => {
                h2522
            }
        }), match v1 {
            Adt14::C0(_, h2523, _, _, _, _, _, _, _, _, _) => {
                h2523
            }
        }, v3, v4)?, 4u64, 64u64, v3, f164(v4)?, v0))
    } else {
        Ok(Adt15::C0)
    }
}

pub fn f87(v0: u64, v1: u64, v2: Adt14, v3: Adt14) -> R<Adt15> {
    let v4: Adt14 = f165(v3.clone());
    let a2525 = match v2.clone() {
        Adt14::C0(h2524, _, _, _, _, _, _, _, _, _, _) => {
            h2524
        }
    };
    let a2526 = 0u64;
    if if if if nat_eq(a2525, a2526) {
        let a2528 = match v3.clone() {
            Adt14::C0(h2527, _, _, _, _, _, _, _, _, _, _) => {
                h2527
            }
        };
        let a2529 = 0u64;
        nat_eq(a2528, a2529)
    } else {
        false
    } {
        f155(v2.clone(), v3)
    } else {
        false
    } {
        if f156(match v2.clone() {
            Adt14::C0(_, h2530, _, _, _, _, _, _, _, _, _) => {
                h2530
            }
        }, match v2.clone() {
            Adt14::C0(_, _, h2531, _, _, _, _, _, _, _, _) => {
                h2531
            }
        }, match v2.clone() {
            Adt14::C0(_, _, _, h2532, _, _, _, _, _, _, _) => {
                h2532
            }
        }, match v2.clone() {
            Adt14::C0(_, _, _, _, h2533, _, _, _, _, _, _) => {
                h2533
            }
        }) {
            true
        } else {
            let a2535 = match v2.clone() {
                Adt14::C0(_, _, _, _, _, h2534, _, _, _, _, _) => {
                    h2534
                }
            };
            let a2536 = 4u64;
            if if if if if nat_eq(a2535, a2536) {
                f166(match v2.clone() {
                    Adt14::C0(_, h2537, _, _, _, _, _, _, _, _, _) => {
                        h2537
                    }
                }, match v2.clone() {
                    Adt14::C0(_, _, h2538, _, _, _, _, _, _, _, _) => {
                        h2538
                    }
                }, match v2.clone() {
                    Adt14::C0(_, _, _, h2539, _, _, _, _, _, _, _) => {
                        h2539
                    }
                }, v0)?
            } else {
                false
            } {
                f158(v2.clone())
            } else {
                false
            } {
                f158(v4.clone())
            } else {
                false
            } {
                let a2541 = f167(v4.clone())?;
                let a2542 = match v4.clone() {
                    Adt14::C0(_, _, _, _, _, _, _, _, _, h2540, _) => {
                        h2540
                    }
                };
                nat_le(a2541, a2542)
            } else {
                false
            } {
                if f159(v2.clone(), v4.clone())? {
                    true
                } else {
                    f160(v2.clone(), v4.clone())
                }
            } else {
                false
            }
        }
    } else {
        false
    } {
        let v5: u64 = if if f156(match v2.clone() {
            Adt14::C0(_, h2543, _, _, _, _, _, _, _, _, _) => {
                h2543
            }
        }, match v2.clone() {
            Adt14::C0(_, _, h2544, _, _, _, _, _, _, _, _) => {
                h2544
            }
        }, match v2.clone() {
            Adt14::C0(_, _, _, h2545, _, _, _, _, _, _, _) => {
                h2545
            }
        }, match v2.clone() {
            Adt14::C0(_, _, _, _, h2546, _, _, _, _, _, _) => {
                h2546
            }
        }) {
            true
        } else {
            f161(v4.clone())?
        } {
            0u64
        } else {
            let a2548 = match v4 {
                Adt14::C0(_, _, _, _, _, _, _, _, _, h2547, _) => {
                    h2547
                }
            };
            let a2549 = 4u64;
            let a2550 = 0u64;
            nat_quot(a2548, a2549, a2550)
        };
        Ok(Adt15::C1(f168(match v2.clone() {
            Adt14::C0(_, h2551, _, _, _, _, _, _, _, _, _) => {
                h2551
            }
        }, match v2.clone() {
            Adt14::C0(_, _, h2552, _, _, _, _, _, _, _, _) => {
                h2552
            }
        }, match v2.clone() {
            Adt14::C0(_, _, _, h2553, _, _, _, _, _, _, _) => {
                h2553
            }
        }, match v2 {
            Adt14::C0(_, _, _, _, h2554, _, _, _, _, _, _) => {
                h2554
            }
        }, v0, v5)?, 4u64, 65u64, v0, f164(v5)?, v1))
    } else {
        Ok(Adt15::C0)
    }
}

pub fn f88(v0: u64, v1: bool, v2: Adt14, v3: Adt14, v4: Adt14) -> R<Adt15> {
    let v5: Adt14 = f165(v4.clone());
    if if if if if if if v1 {
        let a2556 = match v2.clone() {
            Adt14::C0(h2555, _, _, _, _, _, _, _, _, _, _) => {
                h2555
            }
        };
        let a2557 = 0u64;
        nat_eq(a2556, a2557)
    } else {
        false
    } {
        let a2559 = match v3.clone() {
            Adt14::C0(h2558, _, _, _, _, _, _, _, _, _, _) => {
                h2558
            }
        };
        let a2560 = 0u64;
        nat_eq(a2559, a2560)
    } else {
        false
    } {
        let a2562 = match v4.clone() {
            Adt14::C0(h2561, _, _, _, _, _, _, _, _, _, _) => {
                h2561
            }
        };
        let a2563 = 0u64;
        nat_eq(a2562, a2563)
    } else {
        false
    } {
        f155(v2.clone(), v3.clone())
    } else {
        false
    } {
        f155(v2.clone(), v4)
    } else {
        false
    } {
        if f156(match v2.clone() {
            Adt14::C0(_, h2564, _, _, _, _, _, _, _, _, _) => {
                h2564
            }
        }, match v2.clone() {
            Adt14::C0(_, _, h2565, _, _, _, _, _, _, _, _) => {
                h2565
            }
        }, match v2.clone() {
            Adt14::C0(_, _, _, h2566, _, _, _, _, _, _, _) => {
                h2566
            }
        }, match v2.clone() {
            Adt14::C0(_, _, _, _, h2567, _, _, _, _, _, _) => {
                h2567
            }
        }) {
            true
        } else {
            let a2569 = match v2.clone() {
                Adt14::C0(_, _, _, _, _, h2568, _, _, _, _, _) => {
                    h2568
                }
            };
            let a2570 = 4u64;
            if if if if if if if nat_eq(a2569, a2570) {
                let a2572 = match v3.clone() {
                    Adt14::C0(_, _, _, _, _, h2571, _, _, _, _, _) => {
                        h2571
                    }
                };
                let a2573 = 4u64;
                nat_eq(a2572, a2573)
            } else {
                false
            } {
                f158(v2.clone())
            } else {
                false
            } {
                f158(v3.clone())
            } else {
                false
            } {
                f158(v5.clone())
            } else {
                false
            } {
                let a2575 = f167(v5.clone())?;
                let a2576 = match v5.clone() {
                    Adt14::C0(_, _, _, _, _, _, _, _, _, h2574, _) => {
                        h2574
                    }
                };
                nat_le(a2575, a2576)
            } else {
                false
            } {
                if f159(v2.clone(), v5.clone())? {
                    true
                } else {
                    f160(v2.clone(), v5.clone())
                }
            } else {
                false
            } {
                if f159(v3.clone(), v5.clone())? {
                    true
                } else {
                    f160(v3, v5.clone())
                }
            } else {
                false
            }
        }
    } else {
        false
    } {
        let v6: u64 = if f156(match v2.clone() {
            Adt14::C0(_, h2577, _, _, _, _, _, _, _, _, _) => {
                h2577
            }
        }, match v2.clone() {
            Adt14::C0(_, _, h2578, _, _, _, _, _, _, _, _) => {
                h2578
            }
        }, match v2.clone() {
            Adt14::C0(_, _, _, h2579, _, _, _, _, _, _, _) => {
                h2579
            }
        }, match v2.clone() {
            Adt14::C0(_, _, _, _, h2580, _, _, _, _, _, _) => {
                h2580
            }
        }) {
            0u64
        } else {
            let a2586 = match v2.clone() {
                Adt14::C0(_, _, h2581, _, _, _, _, _, _, _, _) => {
                    h2581
                }
            };
            let a2587 = {
                let a2584 = match v2.clone() {
                    Adt14::C0(_, _, _, h2582, _, _, _, _, _, _, _) => {
                        h2582
                    }
                };
                let a2585 = match v2.clone() {
                    Adt14::C0(_, _, _, _, h2583, _, _, _, _, _, _) => {
                        h2583
                    }
                };
                nat_mul(a2584, a2585)?
            };
            nat_mul(a2586, a2587)?
        };
        let v7: u64 = {
            let a2588 = v6;
            let a2589 = 0u64;
            if if nat_eq(a2588, a2589) {
                true
            } else {
                f161(v5.clone())?
            } {
                0u64
            } else {
                let a2591 = match v5 {
                    Adt14::C0(_, _, _, _, _, _, _, _, _, h2590, _) => {
                        h2590
                    }
                };
                let a2592 = 4u64;
                let a2593 = 0u64;
                nat_quot(a2591, a2592, a2593)
            }
        };
        Ok(Adt15::C1(f169(match v2 {
            Adt14::C0(_, h2594, _, _, _, _, _, _, _, _, _) => {
                h2594
            }
        }, v6, v7)?, 4u64, 66u64, v6, f164(v7)?, v0))
    } else {
        Ok(Adt15::C0)
    }
}

pub fn f89(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64) -> bool {
    if if f170(v0, v3, v6) {
        f170(v1, v4, v6)
    } else {
        false
    } {
        f170(v2, v5, v6)
    } else {
        false
    }
}

pub fn f90(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64) -> R<bool> {
    if if if if if f171(v0, v1, v4, v2, v5, v3, v6)? {
        true
    } else {
        f171(v0, v1, v4, v3, v6, v2, v5)?
    } {
        true
    } else {
        f171(v0, v2, v5, v1, v4, v3, v6)?
    } {
        true
    } else {
        f171(v0, v2, v5, v3, v6, v1, v4)?
    } {
        true
    } else {
        f171(v0, v3, v6, v1, v4, v2, v5)?
    } {
        Ok(true)
    } else {
        f171(v0, v3, v6, v2, v5, v1, v4)
    }
}

pub fn f91(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let a2595 = v2;
    let a2596 = 0u64;
    if nat_eq(a2595, a2596) {
        Ok(0u64)
    } else {
        let a2599 = 2u64;
        let a2600 = {
            let a2597 = v0;
            let a2598 = v1;
            nat_mul(a2597, a2598)?
        };
        nat_mul(a2599, a2600)
    }
}

pub fn f92(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let a2601 = v2;
    let a2602 = f151(v0, v1)?;
    nat_mul(a2601, a2602)
}

pub fn f93(v0: u64) -> u64 {
    let a2603 = v0;
    let a2604 = 30u64;
    if nat_eq(a2603, a2604) {
        3u64
    } else {
        1u64
    }
}

pub fn f94(v0: u64) -> R<u64> {
    let a2607 = 5u64;
    let a2608 = {
        let a2605 = 4u64;
        let a2606 = v0;
        nat_mul(a2605, a2606)?
    };
    nat_add(a2607, a2608)
}

pub fn f95(v0: u64) -> bool {
    let a2609 = v0;
    let a2610 = 0u64;
    if if nat_eq(a2609, a2610) {
        true
    } else {
        let a2611 = v0;
        let a2612 = 1u64;
        nat_eq(a2611, a2612)
    } {
        true
    } else {
        let a2613 = v0;
        let a2614 = 30u64;
        nat_eq(a2613, a2614)
    }
}

pub fn f96(v0: Adt2, v1: u64) -> R<bool> {
    let a2616 = match v0.clone() {
        Adt2::C0(_, _, _, h2615, _, _, _, _, _, _, _) => {
            h2615
        }
    };
    let a2617 = 1u64;
    if if if nat_eq(a2616, a2617) {
        let a2619 = match v0.clone() {
            Adt2::C0(_, _, _, _, h2618, _, _, _, _, _, _) => {
                h2618
            }
        };
        let a2620 = 1u64;
        nat_eq(a2619, a2620)
    } else {
        false
    } {
        let a2622 = match v0.clone() {
            Adt2::C0(_, _, _, _, _, h2621, _, _, _, _, _) => {
                h2621
            }
        };
        let a2623 = v1;
        nat_eq(a2622, a2623)
    } else {
        false
    } {
        let a2625 = match v0.clone() {
            Adt2::C0(_, _, h2624, _, _, _, _, _, _, _, _) => {
                h2624
            }
        };
        let a2626 = 1u64;
        if nat_eq(a2625, a2626) {
            Ok(true)
        } else {
            let a2631 = match v0.clone() {
                Adt2::C0(_, _, _, _, _, _, h2627, _, _, _, _) => {
                    h2627
                }
            };
            let a2632 = {
                let a2629 = match v0 {
                    Adt2::C0(_, h2628, _, _, _, _, _, _, _, _, _) => {
                        h2628
                    }
                };
                let a2630 = v1;
                nat_mul(a2629, a2630)?
            };
            Ok(nat_eq(a2631, a2632))
        }
    } else {
        Ok(false)
    }
}

pub fn f97(v0: Adt6) -> R<u64> {
    let a2645 = 2u64;
    let a2646 = {
        let a2643 = 2u64;
        let a2644 = {
            let a2640 = {
                let a2637 = match v0.clone() {
                    Adt6::C0(h2633, _, _, _) => {
                        h2633
                    }
                };
                let a2638 = {
                    let a2635 = match v0.clone() {
                        Adt6::C0(_, h2634, _, _) => {
                            h2634
                        }
                    };
                    let a2636 = 1u64;
                    nat_sub(a2635, a2636)
                };
                nat_add(a2637, a2638)?
            };
            let a2641 = match v0 {
                Adt6::C0(_, h2639, _, _) => {
                    h2639
                }
            };
            let a2642 = 0u64;
            nat_quot(a2640, a2641, a2642)
        };
        nat_mul(a2643, a2644)?
    };
    nat_add(a2645, a2646)
}

pub fn f98(v0: u64) -> R<u64> {
    let a2649 = 2u64;
    let a2650 = {
        let a2647 = v0;
        let a2648 = 2u64;
        nat_mul(a2647, a2648)?
    };
    nat_add(a2649, a2650)
}

pub fn f99(v0: Adt6, v1: u64, v2: u64) -> R<bool> {
    let v3: u64 = {
        let a2654 = {
            let a2651 = v2;
            let a2652 = 1u64;
            nat_sub(a2651, a2652)
        };
        let a2655 = match v0.clone() {
            Adt6::C0(_, h2653, _, _) => {
                h2653
            }
        };
        nat_mul(a2654, a2655)?
    };
    let a2657 = 0u64;
    let a2658 = match v0.clone() {
        Adt6::C0(h2656, _, _, _) => {
            h2656
        }
    };
    if if if if if if if if nat_lt(a2657, a2658) {
        let a2660 = match v0.clone() {
            Adt6::C0(h2659, _, _, _) => {
                h2659
            }
        };
        let a2661 = 2147483648u64;
        nat_lt(a2660, a2661)
    } else {
        false
    } {
        let a2663 = 0u64;
        let a2664 = match v0.clone() {
            Adt6::C0(_, h2662, _, _) => {
                h2662
            }
        };
        nat_lt(a2663, a2664)
    } else {
        false
    } {
        let a2666 = match v0.clone() {
            Adt6::C0(_, h2665, _, _) => {
                h2665
            }
        };
        let a2667 = 2147483648u64;
        nat_lt(a2666, a2667)
    } else {
        false
    } {
        let a2668 = v1;
        let a2669 = 2147483648u64;
        nat_lt(a2668, a2669)
    } else {
        false
    } {
        let a2670 = 0u64;
        let a2671 = v2;
        nat_lt(a2670, a2671)
    } else {
        false
    } {
        let a2672 = v3;
        let a2673 = 2147483648u64;
        nat_lt(a2672, a2673)
    } else {
        false
    } {
        let a2685 = {
            let a2680 = {
                let a2677 = v3;
                let a2678 = {
                    let a2675 = match v0.clone() {
                        Adt6::C0(h2674, _, _, _) => {
                            h2674
                        }
                    };
                    let a2676 = 1u64;
                    nat_sub(a2675, a2676)
                };
                nat_add(a2677, a2678)?
            };
            let a2681 = f119(match v0.clone() {
                Adt6::C0(_, _, h2679, _) => {
                    h2679
                }
            });
            nat_add(a2680, a2681)?
        };
        let a2686 = {
            let a2683 = 2147483647u64;
            let a2684 = f120(match v0.clone() {
                Adt6::C0(_, _, h2682, _) => {
                    h2682
                }
            });
            nat_add(a2683, a2684)?
        };
        nat_le(a2685, a2686)
    } else {
        false
    } {
        let a2688 = match v0 {
            Adt6::C0(_, _, h2687, _) => {
                h2687
            }
        };
        let a2689 = 2147483648u64;
        Ok(!nat_eq(a2688, a2689))
    } else {
        Ok(false)
    }
}

pub fn f100(v0: Adt7) -> R<u64> {
    let a2701 = if match v0.clone() {
        Adt7::C0(_, _, _, h2690, _) => {
            h2690
        }
    } {
        2u64
    } else {
        3u64
    };
    let a2702 = {
        let a2699 = {
            let a2697 = {
                let m2693 = match v0.clone() {
                    Adt7::C0(h2692, _, _, _, _) => {
                        h2692
                    }
                };
                match m2693 {
                    Adt6::C0(h2691, _, _, _) => {
                        h2691
                    }
                }
            };
            let a2698 = {
                let m2696 = match v0.clone() {
                    Adt7::C0(_, h2695, _, _, _) => {
                        h2695
                    }
                };
                match m2696 {
                    Adt6::C0(h2694, _, _, _) => {
                        h2694
                    }
                }
            };
            nat_mul(a2697, a2698)?
        };
        let a2700 = f147(v0)?;
        nat_mul(a2699, a2700)?
    };
    nat_add(a2701, a2702)
}

pub fn f101(v0: Adt2, v1: Adt2, v2: Adt2) -> R<Adt10> {
    if f172(match v0.clone() {
        Adt2::C0(h2703, _, _, _, _, _, _, _, _, _, _) => {
            h2703
        }
    }, match v0.clone() {
        Adt2::C0(_, h2704, _, _, _, _, _, _, _, _, _) => {
            h2704
        }
    }, match v0.clone() {
        Adt2::C0(_, _, h2705, _, _, _, _, _, _, _, _) => {
            h2705
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, h2706, _, _, _, _, _, _, _) => {
            h2706
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, h2707, _, _, _, _, _, _) => {
            h2707
        }
    }, match v1.clone() {
        Adt2::C0(h2708, _, _, _, _, _, _, _, _, _, _) => {
            h2708
        }
    }, match v1.clone() {
        Adt2::C0(_, h2709, _, _, _, _, _, _, _, _, _) => {
            h2709
        }
    }, match v1.clone() {
        Adt2::C0(_, _, h2710, _, _, _, _, _, _, _, _) => {
            h2710
        }
    }, match v1.clone() {
        Adt2::C0(_, _, _, h2711, _, _, _, _, _, _, _) => {
            h2711
        }
    }, match v1.clone() {
        Adt2::C0(_, _, _, _, h2712, _, _, _, _, _, _) => {
            h2712
        }
    }, match v2.clone() {
        Adt2::C0(h2713, _, _, _, _, _, _, _, _, _, _) => {
            h2713
        }
    }, match v2.clone() {
        Adt2::C0(_, h2714, _, _, _, _, _, _, _, _, _) => {
            h2714
        }
    }, match v2.clone() {
        Adt2::C0(_, _, h2715, _, _, _, _, _, _, _, _) => {
            h2715
        }
    }, match v2.clone() {
        Adt2::C0(_, _, _, h2716, _, _, _, _, _, _, _) => {
            h2716
        }
    }, match v2.clone() {
        Adt2::C0(_, _, _, _, h2717, _, _, _, _, _, _) => {
            h2717
        }
    }) {
        let m2719 = f104(match v0.clone() {
            Adt2::C0(h2718, _, _, _, _, _, _, _, _, _, _) => {
                h2718
            }
        });
        match m2719 {
            Adt16::C0 => {
                Ok(Adt10::C0)
            }
            Adt16::C1(v3) => {
                let a2725 = {
                    let a2722 = match v0.clone() {
                        Adt2::C0(_, h2720, _, _, _, _, _, _, _, _, _) => {
                            h2720
                        }
                    };
                    let a2723 = match v3.clone() {
                        Adt17::C0(_, h2721, _) => {
                            h2721
                        }
                    };
                    let a2724 = 0u64;
                    nat_rem(a2722, a2723, a2724)
                };
                let a2726 = 0u64;
                if if nat_eq(a2725, a2726) {
                    if f173(match v0.clone() {
                        Adt2::C0(_, h2727, _, _, _, _, _, _, _, _, _) => {
                            h2727
                        }
                    }, match v1.clone() {
                        Adt2::C0(_, h2728, _, _, _, _, _, _, _, _, _) => {
                            h2728
                        }
                    }, match v1.clone() {
                        Adt2::C0(_, _, h2729, _, _, _, _, _, _, _, _) => {
                            h2729
                        }
                    }, match v1.clone() {
                        Adt2::C0(_, _, _, h2730, _, _, _, _, _, _, _) => {
                            h2730
                        }
                    }) {
                        true
                    } else {
                        let a2732 = 4u64;
                        let a2733 = match v2.clone() {
                            Adt2::C0(_, _, _, _, _, h2731, _, _, _, _, _) => {
                                h2731
                            }
                        };
                        if if nat_le(a2732, a2733) {
                            let a2735 = match v3.clone() {
                                Adt17::C0(_, h2734, _) => {
                                    h2734
                                }
                            };
                            let a2736 = 1u64;
                            if nat_eq(a2735, a2736) {
                                true
                            } else {
                                let a2739 = match v3.clone() {
                                    Adt17::C0(_, _, h2737) => {
                                        h2737
                                    }
                                };
                                let a2740 = match v0.clone() {
                                    Adt2::C0(_, _, _, _, _, h2738, _, _, _, _, _) => {
                                        h2738
                                    }
                                };
                                nat_le(a2739, a2740)
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
                    let v4: Adt3 = f17(v0.clone(), match v3 {
                        Adt17::C0(_, h2741, _) => {
                            h2741
                        }
                    });
                    Ok(Adt10::C4(if f173(match v0.clone() {
                        Adt2::C0(_, h2742, _, _, _, _, _, _, _, _, _) => {
                            h2742
                        }
                    }, match v1.clone() {
                        Adt2::C0(_, h2743, _, _, _, _, _, _, _, _, _) => {
                            h2743
                        }
                    }, match v1.clone() {
                        Adt2::C0(_, _, h2744, _, _, _, _, _, _, _, _) => {
                            h2744
                        }
                    }, match v1.clone() {
                        Adt2::C0(_, _, _, h2745, _, _, _, _, _, _, _) => {
                            h2745
                        }
                    }) {
                        0u64
                    } else {
                        f141(v4.clone(), f17(v1.clone(), 1u64))?
                    }, 4u64, v4, f17(v2, 1u64), Adt4::C20(match v0 {
                        Adt2::C0(h2746, _, _, _, _, _, _, _, _, _, _) => {
                            h2746
                        }
                    }, f17(v1, 1u64))))
                } else {
                    Ok(Adt10::C1)
                }
            }
        }
    } else {
        Ok(Adt10::C1)
    }
}

pub fn f102(v0: Adt2, v1: Adt2, v2: Adt2) -> R<Adt10> {
    if if if if if if if if if if if if f95(match v0.clone() {
        Adt2::C0(h2747, _, _, _, _, _, _, _, _, _, _) => {
            h2747
        }
    }) {
        let a2749 = match v1.clone() {
            Adt2::C0(h2748, _, _, _, _, _, _, _, _, _, _) => {
                h2748
            }
        };
        let a2750 = 26u64;
        nat_eq(a2749, a2750)
    } else {
        false
    } {
        let a2752 = match v2.clone() {
            Adt2::C0(h2751, _, _, _, _, _, _, _, _, _, _) => {
                h2751
            }
        };
        let a2753 = 0u64;
        nat_eq(a2752, a2753)
    } else {
        false
    } {
        let a2756 = match v0.clone() {
            Adt2::C0(_, h2754, _, _, _, _, _, _, _, _, _) => {
                h2754
            }
        };
        let a2757 = match v2.clone() {
            Adt2::C0(_, h2755, _, _, _, _, _, _, _, _, _) => {
                h2755
            }
        };
        nat_eq(a2756, a2757)
    } else {
        false
    } {
        let a2760 = match v1.clone() {
            Adt2::C0(_, h2758, _, _, _, _, _, _, _, _, _) => {
                h2758
            }
        };
        let a2761 = match v0.clone() {
            Adt2::C0(_, _, h2759, _, _, _, _, _, _, _, _) => {
                h2759
            }
        };
        nat_le(a2760, a2761)
    } else {
        false
    } {
        let a2763 = match v0.clone() {
            Adt2::C0(_, _, _, h2762, _, _, _, _, _, _, _) => {
                h2762
            }
        };
        let a2764 = 1u64;
        nat_eq(a2763, a2764)
    } else {
        false
    } {
        let a2766 = match v0.clone() {
            Adt2::C0(_, _, _, _, h2765, _, _, _, _, _, _) => {
                h2765
            }
        };
        let a2767 = 1u64;
        nat_eq(a2766, a2767)
    } else {
        false
    } {
        let a2769 = match v2.clone() {
            Adt2::C0(_, _, _, h2768, _, _, _, _, _, _, _) => {
                h2768
            }
        };
        let a2770 = 1u64;
        nat_eq(a2769, a2770)
    } else {
        false
    } {
        let a2772 = match v2.clone() {
            Adt2::C0(_, _, _, _, h2771, _, _, _, _, _, _) => {
                h2771
            }
        };
        let a2773 = 1u64;
        nat_eq(a2772, a2773)
    } else {
        false
    } {
        let a2775 = match v1.clone() {
            Adt2::C0(_, _, h2774, _, _, _, _, _, _, _, _) => {
                h2774
            }
        };
        let a2776 = 1u64;
        nat_eq(a2775, a2776)
    } else {
        false
    } {
        let a2778 = match v1.clone() {
            Adt2::C0(_, _, _, h2777, _, _, _, _, _, _, _) => {
                h2777
            }
        };
        let a2779 = 1u64;
        nat_eq(a2778, a2779)
    } else {
        false
    } {
        let a2781 = match v1.clone() {
            Adt2::C0(_, _, _, _, h2780, _, _, _, _, _, _) => {
                h2780
            }
        };
        let a2782 = 1u64;
        nat_eq(a2781, a2782)
    } else {
        false
    } {
        if f30(match v2.clone() {
            Adt2::C0(_, h2783, _, _, _, _, _, _, _, _, _) => {
                h2783
            }
        }, match v2.clone() {
            Adt2::C0(_, _, h2784, _, _, _, _, _, _, _, _) => {
                h2784
            }
        }, match v2.clone() {
            Adt2::C0(_, _, _, h2785, _, _, _, _, _, _, _) => {
                h2785
            }
        }, match v2.clone() {
            Adt2::C0(_, _, _, _, h2786, _, _, _, _, _, _) => {
                h2786
            }
        }) {
            Ok(Adt10::C4(0u64, 4u64, f17(v0, 1u64), f17(v2, 1u64), Adt4::C9(0u64, 0u64)))
        } else {
            let a2788 = match v1.clone() {
                Adt2::C0(_, h2787, _, _, _, _, _, _, _, _, _) => {
                    h2787
                }
            };
            let a2789 = 0u64;
            if nat_eq(a2788, a2789) {
                Ok(Adt10::C4(f13(v2.clone(), 1u64)?, 4u64, f17(v0, 1u64), f17(v2, 1u64), Adt4::C9(0u64, 0u64)))
            } else {
                let a2791 = match v0.clone() {
                    Adt2::C0(_, h2790, _, _, _, _, _, _, _, _, _) => {
                        h2790
                    }
                };
                let a2792 = 2147483647u64;
                if if if if if if nat_le(a2791, a2792) {
                    let a2794 = match v1.clone() {
                        Adt2::C0(_, h2793, _, _, _, _, _, _, _, _, _) => {
                            h2793
                        }
                    };
                    let a2795 = 2147483647u64;
                    nat_le(a2794, a2795)
                } else {
                    false
                } {
                    let a2797 = 4u64;
                    let a2798 = match v2.clone() {
                        Adt2::C0(_, _, _, _, _, h2796, _, _, _, _, _) => {
                            h2796
                        }
                    };
                    nat_le(a2797, a2798)
                } else {
                    false
                } {
                    let a2804 = {
                        let a2801 = match v2.clone() {
                            Adt2::C0(_, h2799, _, _, _, _, _, _, _, _, _) => {
                                h2799
                            }
                        };
                        let a2802 = match v2.clone() {
                            Adt2::C0(_, _, _, _, _, h2800, _, _, _, _, _) => {
                                h2800
                            }
                        };
                        nat_mul(a2801, a2802)?
                    };
                    let a2805 = match v2.clone() {
                        Adt2::C0(_, _, _, _, _, _, h2803, _, _, _, _) => {
                            h2803
                        }
                    };
                    nat_le(a2804, a2805)
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
                        let a2819 = {
                            let a2809 = {
                                let a2807 = 2u64;
                                let a2808 = match v1.clone() {
                                    Adt2::C0(_, h2806, _, _, _, _, _, _, _, _, _) => {
                                        h2806
                                    }
                                };
                                nat_mul(a2807, a2808)?
                            };
                            let a2810 = f13(v2.clone(), 1u64)?;
                            nat_add(a2809, a2810)?
                        };
                        let a2820 = {
                            let a2817 = match v1.clone() {
                                Adt2::C0(_, h2811, _, _, _, _, _, _, _, _, _) => {
                                    h2811
                                }
                            };
                            let a2818 = {
                                let a2815 = 1u64;
                                let a2816 = {
                                    let a2813 = 2u64;
                                    let a2814 = match v0.clone() {
                                        Adt2::C0(_, h2812, _, _, _, _, _, _, _, _, _) => {
                                            h2812
                                        }
                                    };
                                    nat_mul(a2813, a2814)?
                                };
                                nat_add(a2815, a2816)?
                            };
                            nat_mul(a2817, a2818)?
                        };
                        nat_add(a2819, a2820)?
                    }, 4u64, Adt3::C0(match v0.clone() {
                        Adt2::C0(_, h2821, _, _, _, _, _, _, _, _, _) => {
                            h2821
                        }
                    }, match v1.clone() {
                        Adt2::C0(_, h2822, _, _, _, _, _, _, _, _, _) => {
                            h2822
                        }
                    }, 1u64, 1u64, match v0.clone() {
                        Adt2::C0(_, _, _, _, _, h2823, _, _, _, _, _) => {
                            h2823
                        }
                    }, match v0.clone() {
                        Adt2::C0(_, _, _, _, _, _, h2824, _, _, _, _) => {
                            h2824
                        }
                    }, match v0.clone() {
                        Adt2::C0(_, _, _, _, _, _, _, h2825, _, _, _) => {
                            h2825
                        }
                    }, match v0.clone() {
                        Adt2::C0(_, _, _, _, _, _, _, _, h2826, _, _) => {
                            h2826
                        }
                    }), f17(v2, 1u64), Adt4::C19(match v0 {
                        Adt2::C0(h2827, _, _, _, _, _, _, _, _, _, _) => {
                            h2827
                        }
                    }, f17(v1, 1u64))))
                } else {
                    Ok(Adt10::C1)
                }
            }
        }
    } else {
        Ok(Adt10::C1)
    }
}

pub fn f103(v0: u64, v1: Adt2, v2: bool, v3: Adt2) -> R<Adt10> {
    let a2830 = match v1.clone() {
        Adt2::C0(h2828, _, _, _, _, _, _, _, _, _, _) => {
            h2828
        }
    };
    let a2831 = match v3.clone() {
        Adt2::C0(h2829, _, _, _, _, _, _, _, _, _, _) => {
            h2829
        }
    };
    if if nat_eq(a2830, a2831) {
        let a2832 = v0;
        let a2833 = 33u64;
        if nat_eq(a2832, a2833) {
            v2
        } else {
            true
        }
    } else {
        false
    } {
        let m2835 = f104(match v1.clone() {
            Adt2::C0(h2834, _, _, _, _, _, _, _, _, _, _) => {
                h2834
            }
        });
        match m2835 {
            Adt16::C0 => {
                Ok(Adt10::C0)
            }
            Adt16::C1(v4) => {
                let v5: u64 = match v4.clone() {
                    Adt17::C0(_, h2836, _) => {
                        h2836
                    }
                };
                let v6: u64 = f13(v1.clone(), v5)?;
                let a2841 = {
                    let a2838 = match v1.clone() {
                        Adt2::C0(_, h2837, _, _, _, _, _, _, _, _, _) => {
                            h2837
                        }
                    };
                    let a2839 = v5;
                    let a2840 = 1u64;
                    nat_rem(a2838, a2839, a2840)
                };
                let a2842 = 0u64;
                if if if nat_eq(a2841, a2842) {
                    let a2847 = {
                        let a2844 = match v3.clone() {
                            Adt2::C0(_, h2843, _, _, _, _, _, _, _, _, _) => {
                                h2843
                            }
                        };
                        let a2845 = v5;
                        let a2846 = 1u64;
                        nat_rem(a2844, a2845, a2846)
                    };
                    let a2848 = 0u64;
                    nat_eq(a2847, a2848)
                } else {
                    false
                } {
                    let a2849 = v6;
                    let a2850 = f13(v3.clone(), v5)?;
                    nat_eq(a2849, a2850)
                } else {
                    false
                } {
                    Ok(Adt10::C4(v6, match v4 {
                        Adt17::C0(_, _, h2851) => {
                            h2851
                        }
                    }, f17(v1, v5), f17(v3, v5), Adt4::C0))
                } else {
                    Ok(Adt10::C1)
                }
            }
        }
    } else {
        Ok(Adt10::C1)
    }
}

pub fn f104(v0: u64) -> Adt16 {
    let a2852 = v0;
    let a2853 = 0u64;
    if nat_eq(a2852, a2853) {
        Adt16::C1(Adt17::C0(0u64, 1u64, 4u64))
    } else {
        let a2854 = v0;
        let a2855 = 1u64;
        if nat_eq(a2854, a2855) {
            Adt16::C1(Adt17::C0(1u64, 1u64, 2u64))
        } else {
            let a2856 = v0;
            let a2857 = 2u64;
            if nat_eq(a2856, a2857) {
                Adt16::C1(Adt17::C0(2u64, 32u64, 18u64))
            } else {
                let a2858 = v0;
                let a2859 = 3u64;
                if nat_eq(a2858, a2859) {
                    Adt16::C1(Adt17::C0(3u64, 32u64, 20u64))
                } else {
                    let a2860 = v0;
                    let a2861 = 6u64;
                    if nat_eq(a2860, a2861) {
                        Adt16::C1(Adt17::C0(6u64, 32u64, 22u64))
                    } else {
                        let a2862 = v0;
                        let a2863 = 7u64;
                        if nat_eq(a2862, a2863) {
                            Adt16::C1(Adt17::C0(7u64, 32u64, 24u64))
                        } else {
                            let a2864 = v0;
                            let a2865 = 8u64;
                            if nat_eq(a2864, a2865) {
                                Adt16::C1(Adt17::C0(8u64, 32u64, 34u64))
                            } else {
                                let a2866 = v0;
                                let a2867 = 9u64;
                                if nat_eq(a2866, a2867) {
                                    Adt16::C1(Adt17::C0(9u64, 32u64, 36u64))
                                } else {
                                    let a2868 = v0;
                                    let a2869 = 10u64;
                                    if nat_eq(a2868, a2869) {
                                        Adt16::C1(Adt17::C0(10u64, 256u64, 84u64))
                                    } else {
                                        let a2870 = v0;
                                        let a2871 = 11u64;
                                        if nat_eq(a2870, a2871) {
                                            Adt16::C1(Adt17::C0(11u64, 256u64, 110u64))
                                        } else {
                                            let a2872 = v0;
                                            let a2873 = 12u64;
                                            if nat_eq(a2872, a2873) {
                                                Adt16::C1(Adt17::C0(12u64, 256u64, 144u64))
                                            } else {
                                                let a2874 = v0;
                                                let a2875 = 13u64;
                                                if nat_eq(a2874, a2875) {
                                                    Adt16::C1(Adt17::C0(13u64, 256u64, 176u64))
                                                } else {
                                                    let a2876 = v0;
                                                    let a2877 = 14u64;
                                                    if nat_eq(a2876, a2877) {
                                                        Adt16::C1(Adt17::C0(14u64, 256u64, 210u64))
                                                    } else {
                                                        let a2878 = v0;
                                                        let a2879 = 15u64;
                                                        if nat_eq(a2878, a2879) {
                                                            Adt16::C1(Adt17::C0(15u64, 256u64, 292u64))
                                                        } else {
                                                            let a2880 = v0;
                                                            let a2881 = 16u64;
                                                            if nat_eq(a2880, a2881) {
                                                                Adt16::C1(Adt17::C0(16u64, 256u64, 66u64))
                                                            } else {
                                                                let a2882 = v0;
                                                                let a2883 = 17u64;
                                                                if nat_eq(a2882, a2883) {
                                                                    Adt16::C1(Adt17::C0(17u64, 256u64, 74u64))
                                                                } else {
                                                                    let a2884 = v0;
                                                                    let a2885 = 18u64;
                                                                    if nat_eq(a2884, a2885) {
                                                                        Adt16::C1(Adt17::C0(18u64, 256u64, 98u64))
                                                                    } else {
                                                                        let a2886 = v0;
                                                                        let a2887 = 19u64;
                                                                        if nat_eq(a2886, a2887) {
                                                                            Adt16::C1(Adt17::C0(19u64, 256u64, 50u64))
                                                                        } else {
                                                                            let a2888 = v0;
                                                                            let a2889 = 20u64;
                                                                            if nat_eq(a2888, a2889) {
                                                                                Adt16::C1(Adt17::C0(20u64, 32u64, 18u64))
                                                                            } else {
                                                                                let a2890 = v0;
                                                                                let a2891 = 21u64;
                                                                                if nat_eq(a2890, a2891) {
                                                                                    Adt16::C1(Adt17::C0(21u64, 256u64, 110u64))
                                                                                } else {
                                                                                    let a2892 = v0;
                                                                                    let a2893 = 22u64;
                                                                                    if nat_eq(a2892, a2893) {
                                                                                        Adt16::C1(Adt17::C0(22u64, 256u64, 82u64))
                                                                                    } else {
                                                                                        let a2894 = v0;
                                                                                        let a2895 = 23u64;
                                                                                        if nat_eq(a2894, a2895) {
                                                                                            Adt16::C1(Adt17::C0(23u64, 256u64, 136u64))
                                                                                        } else {
                                                                                            let a2896 = v0;
                                                                                            let a2897 = 24u64;
                                                                                            if nat_eq(a2896, a2897) {
                                                                                                Adt16::C1(Adt17::C0(24u64, 1u64, 1u64))
                                                                                            } else {
                                                                                                let a2898 = v0;
                                                                                                let a2899 = 25u64;
                                                                                                if nat_eq(a2898, a2899) {
                                                                                                    Adt16::C1(Adt17::C0(25u64, 1u64, 2u64))
                                                                                                } else {
                                                                                                    let a2900 = v0;
                                                                                                    let a2901 = 26u64;
                                                                                                    if nat_eq(a2900, a2901) {
                                                                                                        Adt16::C1(Adt17::C0(26u64, 1u64, 4u64))
                                                                                                    } else {
                                                                                                        let a2902 = v0;
                                                                                                        let a2903 = 27u64;
                                                                                                        if nat_eq(a2902, a2903) {
                                                                                                            Adt16::C1(Adt17::C0(27u64, 1u64, 8u64))
                                                                                                        } else {
                                                                                                            let a2904 = v0;
                                                                                                            let a2905 = 28u64;
                                                                                                            if nat_eq(a2904, a2905) {
                                                                                                                Adt16::C1(Adt17::C0(28u64, 1u64, 8u64))
                                                                                                            } else {
                                                                                                                let a2906 = v0;
                                                                                                                let a2907 = 29u64;
                                                                                                                if nat_eq(a2906, a2907) {
                                                                                                                    Adt16::C1(Adt17::C0(29u64, 256u64, 56u64))
                                                                                                                } else {
                                                                                                                    let a2908 = v0;
                                                                                                                    let a2909 = 30u64;
                                                                                                                    if nat_eq(a2908, a2909) {
                                                                                                                        Adt16::C1(Adt17::C0(30u64, 1u64, 2u64))
                                                                                                                    } else {
                                                                                                                        let a2910 = v0;
                                                                                                                        let a2911 = 34u64;
                                                                                                                        if nat_eq(a2910, a2911) {
                                                                                                                            Adt16::C1(Adt17::C0(34u64, 256u64, 54u64))
                                                                                                                        } else {
                                                                                                                            let a2912 = v0;
                                                                                                                            let a2913 = 35u64;
                                                                                                                            if nat_eq(a2912, a2913) {
                                                                                                                                Adt16::C1(Adt17::C0(35u64, 256u64, 66u64))
                                                                                                                            } else {
                                                                                                                                let a2914 = v0;
                                                                                                                                let a2915 = 39u64;
                                                                                                                                if nat_eq(a2914, a2915) {
                                                                                                                                    Adt16::C1(Adt17::C0(39u64, 32u64, 17u64))
                                                                                                                                } else {
                                                                                                                                    let a2916 = v0;
                                                                                                                                    let a2917 = 40u64;
                                                                                                                                    if nat_eq(a2916, a2917) {
                                                                                                                                        Adt16::C1(Adt17::C0(40u64, 64u64, 36u64))
                                                                                                                                    } else {
                                                                                                                                        let a2918 = v0;
                                                                                                                                        let a2919 = 41u64;
                                                                                                                                        if nat_eq(a2918, a2919) {
                                                                                                                                            Adt16::C1(Adt17::C0(41u64, 128u64, 18u64))
                                                                                                                                        } else {
                                                                                                                                            let a2920 = v0;
                                                                                                                                            let a2921 = 42u64;
                                                                                                                                            if nat_eq(a2920, a2921) {
                                                                                                                                                Adt16::C1(Adt17::C0(42u64, 64u64, 18u64))
                                                                                                                                            } else {
                                                                                                                                                let a2922 = v0;
                                                                                                                                                let a2923 = 43u64;
                                                                                                                                                if nat_eq(a2922, a2923) {
                                                                                                                                                    Adt16::C1(Adt17::C0(43u64, 1u64, 1u64))
                                                                                                                                                } else {
                                                                                                                                                    Adt16::C0
                                                                                                                                                }
                                                                                                                                            }
                                                                                                                                        }
                                                                                                                                    }
                                                                                                                                }
                                                                                                                            }
                                                                                                                        }
                                                                                                                    }
                                                                                                                }
                                                                                                            }
                                                                                                        }
                                                                                                    }
                                                                                                }
                                                                                            }
                                                                                        }
                                                                                    }
                                                                                }
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn f105(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64, v7: u64) -> bool {
    if f30(v4, v5, v6, v7) {
        f30(v0, v1, v2, v3)
    } else {
        let a2927 = {
            let a2924 = v0;
            let a2925 = v4;
            let a2926 = 1u64;
            nat_rem(a2924, a2925, a2926)
        };
        let a2928 = 0u64;
        if if if nat_eq(a2927, a2928) {
            let a2932 = {
                let a2929 = v1;
                let a2930 = v5;
                let a2931 = 1u64;
                nat_rem(a2929, a2930, a2931)
            };
            let a2933 = 0u64;
            nat_eq(a2932, a2933)
        } else {
            false
        } {
            let a2937 = {
                let a2934 = v2;
                let a2935 = v6;
                let a2936 = 1u64;
                nat_rem(a2934, a2935, a2936)
            };
            let a2938 = 0u64;
            nat_eq(a2937, a2938)
        } else {
            false
        } {
            let a2942 = {
                let a2939 = v3;
                let a2940 = v7;
                let a2941 = 1u64;
                nat_rem(a2939, a2940, a2941)
            };
            let a2943 = 0u64;
            nat_eq(a2942, a2943)
        } else {
            false
        }
    }
}

pub fn f106(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64) -> R<bool> {
    let a2944 = v0;
    let a2945 = v1;
    if nat_eq(a2944, a2945) {
        let a2948 = v4;
        let a2949 = {
            let a2946 = v2;
            let a2947 = v3;
            nat_add(a2946, a2947)?
        };
        Ok(nat_eq(a2948, a2949))
    } else {
        let a2950 = v2;
        let a2951 = v3;
        if nat_eq(a2950, a2951) {
            let a2952 = v4;
            let a2953 = v2;
            Ok(nat_eq(a2952, a2953))
        } else {
            Ok(false)
        }
    }
}

pub fn f107(v0: u64, v1: u64, v2: u64, v3: u64) -> R<bool> {
    let v4: u64 = {
        let a2956 = {
            let a2954 = v0;
            let a2955 = f120(v1);
            nat_add(a2954, a2955)?
        };
        let a2957 = f120(v2);
        nat_add(a2956, a2957)?
    };
    let v5: u64 = {
        let a2958 = f119(v1);
        let a2959 = f119(v2);
        nat_add(a2958, a2959)?
    };
    let a2960 = v1;
    let a2961 = 4294967295u64;
    if if nat_le(a2960, a2961) {
        let a2962 = v2;
        let a2963 = 4294967295u64;
        nat_le(a2962, a2963)
    } else {
        false
    } {
        let a2964 = v5;
        let a2965 = v4;
        if nat_le(a2964, a2965) {
            let a2968 = v3;
            let a2969 = {
                let a2966 = v4;
                let a2967 = v5;
                nat_sub(a2966, a2967)
            };
            Ok(nat_eq(a2968, a2969))
        } else {
            Ok(false)
        }
    } else {
        Ok(false)
    }
}

pub fn f108(v0: u64, v1: u64) -> R<bool> {
    let a2970 = v1;
    let a2971 = 4294967295u64;
    if nat_le(a2970, a2971) {
        let a2974 = {
            let a2972 = f120(v1);
            let a2973 = f119(v1);
            nat_add(a2972, a2973)?
        };
        let a2975 = v0;
        Ok(nat_lt(a2974, a2975))
    } else {
        Ok(false)
    }
}

pub fn f109(v0: Adt3, v1: u64, v2: u64, v3: u64, v4: bool) -> R<bool> {
    let v5: u64 = f174(v0.clone(), false)?;
    let v6: u64 = f174(v0, true)?;
    let a2976 = v6;
    let a2977 = v1;
    if if nat_le(a2976, a2977) {
        let a2984 = {
            let a2982 = {
                let a2978 = v1;
                let a2979 = v5;
                nat_add(a2978, a2979)?
            };
            let a2983 = {
                let a2980 = v3;
                let a2981 = 0u64;
                if nat_eq(a2980, a2981) {
                    0u64
                } else {
                    4u64
                }
            };
            nat_add(a2982, a2983)?
        };
        let a2985 = v2;
        nat_le(a2984, a2985)
    } else {
        false
    } {
        if v4 {
            let a2990 = {
                let a2988 = {
                    let a2986 = v1;
                    let a2987 = v5;
                    nat_add(a2986, a2987)?
                };
                let a2989 = v6;
                nat_sub(a2988, a2989)
            };
            let a2991 = v2;
            Ok(nat_lt(a2990, a2991))
        } else {
            let a2996 = {
                let a2994 = {
                    let a2992 = v1;
                    let a2993 = v5;
                    nat_add(a2992, a2993)?
                };
                let a2995 = v6;
                nat_sub(a2994, a2995)
            };
            let a2997 = v2;
            Ok(nat_le(a2996, a2997))
        }
    } else {
        Ok(false)
    }
}

pub fn f110(v0: u64, v1: u64) -> bool {
    let a2998 = v0;
    let a2999 = 0u64;
    if if nat_eq(a2998, a2999) {
        true
    } else {
        let a3000 = v0;
        let a3001 = 1u64;
        nat_eq(a3000, a3001)
    } {
        let a3002 = v1;
        let a3003 = 4294967296u64;
        nat_lt(a3002, a3003)
    } else {
        false
    }
}

pub fn f111(v0: u64, v1: u64) -> R<u64> {
    let a3008 = {
        let a3006 = v0;
        let a3007 = {
            let a3004 = v1;
            let a3005 = 1u64;
            nat_sub(a3004, a3005)
        };
        nat_add(a3006, a3007)?
    };
    let a3009 = v1;
    let a3010 = 0u64;
    Ok(nat_quot(a3008, a3009, a3010))
}

pub fn f112(v0: Adt2, v1: Adt2, v2: u64, v3: u64) -> R<bool> {
    if f30(match v0.clone() {
        Adt2::C0(_, h3011, _, _, _, _, _, _, _, _, _) => {
            h3011
        }
    }, match v0.clone() {
        Adt2::C0(_, _, h3012, _, _, _, _, _, _, _, _) => {
            h3012
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, h3013, _, _, _, _, _, _, _) => {
            h3013
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, h3014, _, _, _, _, _, _) => {
            h3014
        }
    }) {
        Ok(true)
    } else {
        let v4: u64 = {
            let a3017 = match v1.clone() {
                Adt2::C0(_, h3015, _, _, _, _, _, _, _, _, _) => {
                    h3015
                }
            };
            let a3018 = match v1.clone() {
                Adt2::C0(_, _, h3016, _, _, _, _, _, _, _, _) => {
                    h3016
                }
            };
            nat_mul(a3017, a3018)?
        };
        let v5: u64 = {
            let a3020 = v4;
            let a3021 = match v1.clone() {
                Adt2::C0(_, _, _, h3019, _, _, _, _, _, _, _) => {
                    h3019
                }
            };
            nat_mul(a3020, a3021)?
        };
        let a3034 = {
            let a3032 = {
                let a3023 = match v0.clone() {
                    Adt2::C0(_, h3022, _, _, _, _, _, _, _, _, _) => {
                        h3022
                    }
                };
                let a3024 = 1u64;
                nat_sub(a3023, a3024)
            };
            let a3033 = {
                let a3030 = f175(match v0.clone() {
                    Adt2::C0(_, _, h3025, _, _, _, _, _, _, _, _) => {
                        h3025
                    }
                }, v2, v5, match v1.clone() {
                    Adt2::C0(_, h3026, _, _, _, _, _, _, _, _, _) => {
                        h3026
                    }
                })?;
                let a3031 = f175(match v0 {
                    Adt2::C0(_, _, _, h3027, _, _, _, _, _, _, _) => {
                        h3027
                    }
                }, v2, {
                    let a3028 = v3;
                    let a3029 = v5;
                    nat_mul(a3028, a3029)?
                }, v4)?;
                nat_add(a3030, a3031)?
            };
            nat_add(a3032, a3033)?
        };
        let a3035 = f13(v1, 1u64)?;
        Ok(nat_lt(a3034, a3035))
    }
}

pub fn f113(v0: u64, v1: u64, v2: u64) -> R<bool> {
    let a3036 = v0;
    let a3037 = 2147483648u64;
    if if if if nat_lt(a3036, a3037) {
        let a3038 = v1;
        let a3039 = 2147483648u64;
        nat_lt(a3038, a3039)
    } else {
        false
    } {
        let a3042 = {
            let a3040 = v0;
            let a3041 = v1;
            nat_mul(a3040, a3041)?
        };
        let a3043 = 2147483648u64;
        nat_lt(a3042, a3043)
    } else {
        false
    } {
        let a3048 = {
            let a3046 = {
                let a3044 = v0;
                let a3045 = 1u64;
                nat_sub(a3044, a3045)
            };
            let a3047 = v2;
            nat_mul(a3046, a3047)?
        };
        let a3049 = 2147483648u64;
        nat_lt(a3048, a3049)
    } else {
        false
    } {
        let a3054 = {
            let a3052 = {
                let a3050 = v1;
                let a3051 = 1u64;
                nat_sub(a3050, a3051)
            };
            let a3053 = v2;
            nat_mul(a3052, a3053)?
        };
        let a3055 = 2147483648u64;
        Ok(nat_lt(a3054, a3055))
    } else {
        Ok(false)
    }
}

pub fn f114(v0: u64) -> R<u64> {
    let a3058 = 3u64;
    let a3059 = {
        let a3056 = v0;
        let a3057 = 3u64;
        nat_mul(a3056, a3057)?
    };
    nat_add(a3058, a3059)
}

pub fn f115(v0: Adt3) -> R<u64> {
    if f30(match v0.clone() {
        Adt3::C0(h3060, _, _, _, _, _, _, _) => {
            h3060
        }
    }, match v0.clone() {
        Adt3::C0(_, h3061, _, _, _, _, _, _) => {
            h3061
        }
    }, match v0.clone() {
        Adt3::C0(_, _, h3062, _, _, _, _, _) => {
            h3062
        }
    }, match v0.clone() {
        Adt3::C0(_, _, _, h3063, _, _, _, _) => {
            h3063
        }
    }) {
        Ok(0u64)
    } else {
        let a3072 = match v0.clone() {
            Adt3::C0(h3064, _, _, _, _, _, _, _) => {
                h3064
            }
        };
        let a3073 = {
            let a3070 = match v0.clone() {
                Adt3::C0(_, h3065, _, _, _, _, _, _) => {
                    h3065
                }
            };
            let a3071 = {
                let a3068 = match v0.clone() {
                    Adt3::C0(_, _, h3066, _, _, _, _, _) => {
                        h3066
                    }
                };
                let a3069 = match v0 {
                    Adt3::C0(_, _, _, h3067, _, _, _, _) => {
                        h3067
                    }
                };
                nat_mul(a3068, a3069)?
            };
            nat_mul(a3070, a3071)?
        };
        nat_mul(a3072, a3073)
    }
}

pub fn f116(v0: Adt3, v1: u64, v2: u64) -> R<u64> {
    let a3074 = v1;
    let a3075 = 13u64;
    if if nat_eq(a3074, a3075) {
        f30(match v0.clone() {
            Adt3::C0(h3076, _, _, _, _, _, _, _) => {
                h3076
            }
        }, match v0.clone() {
            Adt3::C0(_, h3077, _, _, _, _, _, _) => {
                h3077
            }
        }, match v0.clone() {
            Adt3::C0(_, _, h3078, _, _, _, _, _) => {
                h3078
            }
        }, match v0.clone() {
            Adt3::C0(_, _, _, h3079, _, _, _, _) => {
                h3079
            }
        })
    } else {
        false
    } {
        Ok(2u64)
    } else {
        let v3: u64 = {
            let a3085 = match v0.clone() {
                Adt3::C0(_, h3080, _, _, _, _, _, _) => {
                    h3080
                }
            };
            let a3086 = {
                let a3083 = match v0.clone() {
                    Adt3::C0(_, _, h3081, _, _, _, _, _) => {
                        h3081
                    }
                };
                let a3084 = match v0.clone() {
                    Adt3::C0(_, _, _, h3082, _, _, _, _) => {
                        h3082
                    }
                };
                nat_mul(a3083, a3084)?
            };
            nat_mul(a3085, a3086)?
        };
        let a3087 = v1;
        let a3088 = 13u64;
        if nat_eq(a3087, a3088) {
            let a3097 = 2u64;
            let a3098 = if f176(v0.clone(), v2) {
                let a3090 = v3;
                let a3091 = f177(match v0 {
                    Adt3::C0(h3089, _, _, _, _, _, _, _) => {
                        h3089
                    }
                }, false)?;
                nat_mul(a3090, a3091)?
            } else {
                let a3095 = v3;
                let a3096 = {
                    let a3093 = match v0 {
                        Adt3::C0(h3092, _, _, _, _, _, _, _) => {
                            h3092
                        }
                    };
                    let a3094 = 2u64;
                    nat_mul(a3093, a3094)?
                };
                nat_mul(a3095, a3096)?
            };
            nat_add(a3097, a3098)
        } else {
            let a3102 = v3;
            let a3103 = f177(match v0 {
                Adt3::C0(h3099, _, _, _, _, _, _, _) => {
                    h3099
                }
            }, {
                let a3100 = v1;
                let a3101 = 16u64;
                nat_eq(a3100, a3101)
            })?;
            nat_mul(a3102, a3103)
        }
    }
}

pub fn f117(v0: u64, v1: u64, v2: Adt2, v3: Option<Adt2>, v4: Adt2) -> R<Adt10> {
    let a3104 = v0;
    let a3105 = 30u64;
    if if nat_eq(a3104, a3105) {
        let a3108 = match v2.clone() {
            Adt2::C0(_, _, _, _, _, _, h3106, _, _, _, _) => {
                h3106
            }
        };
        let a3109 = match v2.clone() {
            Adt2::C0(_, _, _, _, _, h3107, _, _, _, _, _) => {
                h3107
            }
        };
        nat_lt(a3108, a3109)
    } else {
        false
    } {
        Ok(Adt10::C1)
    } else {
        let m3110 = v3;
        match m3110 {
            None => {
                f178(v0, v1, match v2.clone() {
                    Adt2::C0(h3111, _, _, _, _, _, _, _, _, _, _) => {
                        h3111
                    }
                }, 0u64, match v4.clone() {
                    Adt2::C0(h3112, _, _, _, _, _, _, _, _, _, _) => {
                        h3112
                    }
                }, false, match v2.clone() {
                    Adt2::C0(_, h3113, _, _, _, _, _, _, _, _, _) => {
                        h3113
                    }
                }, match v2.clone() {
                    Adt2::C0(_, _, h3114, _, _, _, _, _, _, _, _) => {
                        h3114
                    }
                }, match v2.clone() {
                    Adt2::C0(_, _, _, h3115, _, _, _, _, _, _, _) => {
                        h3115
                    }
                }, match v2 {
                    Adt2::C0(_, _, _, _, h3116, _, _, _, _, _, _) => {
                        h3116
                    }
                }, 1u64, 1u64, 1u64, 1u64, match v4.clone() {
                    Adt2::C0(_, h3117, _, _, _, _, _, _, _, _, _) => {
                        h3117
                    }
                }, match v4.clone() {
                    Adt2::C0(_, _, h3118, _, _, _, _, _, _, _, _) => {
                        h3118
                    }
                }, match v4.clone() {
                    Adt2::C0(_, _, _, h3119, _, _, _, _, _, _, _) => {
                        h3119
                    }
                }, match v4 {
                    Adt2::C0(_, _, _, _, h3120, _, _, _, _, _, _) => {
                        h3120
                    }
                })
            }
            Some(v5) => {
                f178(v0, v1, match v2.clone() {
                    Adt2::C0(h3121, _, _, _, _, _, _, _, _, _, _) => {
                        h3121
                    }
                }, match v5.clone() {
                    Adt2::C0(h3122, _, _, _, _, _, _, _, _, _, _) => {
                        h3122
                    }
                }, match v4.clone() {
                    Adt2::C0(h3123, _, _, _, _, _, _, _, _, _, _) => {
                        h3123
                    }
                }, true, match v2.clone() {
                    Adt2::C0(_, h3124, _, _, _, _, _, _, _, _, _) => {
                        h3124
                    }
                }, match v2.clone() {
                    Adt2::C0(_, _, h3125, _, _, _, _, _, _, _, _) => {
                        h3125
                    }
                }, match v2.clone() {
                    Adt2::C0(_, _, _, h3126, _, _, _, _, _, _, _) => {
                        h3126
                    }
                }, match v2 {
                    Adt2::C0(_, _, _, _, h3127, _, _, _, _, _, _) => {
                        h3127
                    }
                }, match v5.clone() {
                    Adt2::C0(_, h3128, _, _, _, _, _, _, _, _, _) => {
                        h3128
                    }
                }, match v5.clone() {
                    Adt2::C0(_, _, h3129, _, _, _, _, _, _, _, _) => {
                        h3129
                    }
                }, match v5.clone() {
                    Adt2::C0(_, _, _, h3130, _, _, _, _, _, _, _) => {
                        h3130
                    }
                }, match v5 {
                    Adt2::C0(_, _, _, _, h3131, _, _, _, _, _, _) => {
                        h3131
                    }
                }, match v4.clone() {
                    Adt2::C0(_, h3132, _, _, _, _, _, _, _, _, _) => {
                        h3132
                    }
                }, match v4.clone() {
                    Adt2::C0(_, _, h3133, _, _, _, _, _, _, _, _) => {
                        h3133
                    }
                }, match v4.clone() {
                    Adt2::C0(_, _, _, h3134, _, _, _, _, _, _, _) => {
                        h3134
                    }
                }, match v4 {
                    Adt2::C0(_, _, _, _, h3135, _, _, _, _, _, _) => {
                        h3135
                    }
                })
            }
        }
    }
}

pub fn f118(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64, v7: u64) -> R<u64> {
    let a3148 = {
        let a3144 = {
            let a3140 = {
                let a3136 = v0;
                let a3137 = v4;
                nat_mul(a3136, a3137)?
            };
            let a3141 = {
                let a3138 = v1;
                let a3139 = v5;
                nat_mul(a3138, a3139)?
            };
            nat_add(a3140, a3141)?
        };
        let a3145 = {
            let a3142 = v2;
            let a3143 = v6;
            nat_mul(a3142, a3143)?
        };
        nat_add(a3144, a3145)?
    };
    let a3149 = {
        let a3146 = v3;
        let a3147 = v7;
        nat_mul(a3146, a3147)?
    };
    nat_add(a3148, a3149)
}

pub fn f119(v0: u64) -> u64 {
    let a3150 = v0;
    let a3151 = 2147483648u64;
    if nat_lt(a3150, a3151) {
        0u64
    } else {
        let a3152 = 4294967296u64;
        let a3153 = v0;
        nat_sub(a3152, a3153)
    }
}

pub fn f120(v0: u64) -> u64 {
    let a3154 = v0;
    let a3155 = 2147483648u64;
    if nat_lt(a3154, a3155) {
        v0
    } else {
        0u64
    }
}

pub fn f121(v0: Adt11, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64, v7: u64, v8: u64) -> Adt9 {
    let v9: u64 = {
        let a3156 = v7;
        let a3157 = 6u64;
        if nat_eq(a3156, a3157) {
            27u64
        } else {
            v2
        }
    };
    Adt9::C7(v0, f81(v3, v4), f81(v5, v6), f82(v7, v8), f179(v1), f179(v9))
}

pub fn f122(v0: Adt4, v1: u64, v2: u64) -> Adt11 {
    let m3158 = v0;
    match m3158 {
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
        Adt4::C13(v30, v31, v32, v33) => {
            Adt11::C4(v30, v31, v32, v33)
        }
        Adt4::C14(v34, _, _) => {
            let a3159 = 13u64;
            let a3160 = v34;
            if nat_le(a3159, a3160) {
                Adt11::C7(v34, v1, v2)
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
        Adt4::C27(v54) => {
            Adt11::C2(match v54.clone() {
                Adt5::C0(_, _, _, _, _, h3161, _, _) => {
                    h3161
                }
            }, match v54 {
                Adt5::C0(_, _, _, _, _, _, _, h3162) => {
                    h3162
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
    }
}

pub fn f123(v0: Adt4) -> u64 {
    let m3163 = v0;
    match m3163 {
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
        Adt4::C14(v32, _, _) => {
            v32
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
    }
}

pub fn f124(v0: Adt4, v1: u64) -> u64 {
    let m3164 = v0;
    match m3164 {
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
        Adt4::C16(_, v37) => {
            v37
        }
        Adt4::C17 => {
            0u64
        }
        Adt4::C18(_, _) => {
            0u64
        }
        Adt4::C19(v40, _) => {
            let a3165 = v1;
            let a3166 = 6u64;
            if nat_eq(a3165, a3166) {
                v40
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
        Adt4::C26(v54, _, _, _) => {
            let a3175 = v1;
            let a3176 = 6u64;
            if if if nat_eq(a3175, a3176) {
                true
            } else {
                let a3177 = v1;
                let a3178 = 7u64;
                nat_eq(a3177, a3178)
            } {
                let a3179 = v54;
                let a3180 = 30u64;
                nat_eq(a3179, a3180)
            } else {
                false
            } {
                30u64
            } else {
                0u64
            }
        }
        Adt4::C27(v53) => {
            match v53 {
                Adt5::C0(_, _, _, _, _, _, _, h3174) => {
                    h3174
                }
            }
        }
        Adt4::C28(v49) => {
            let a3167 = v1;
            let a3168 = 6u64;
            if nat_eq(a3167, a3168) {
                match v49 {
                    Adt7::C0(_, _, _, _, h3169) => {
                        h3169
                    }
                }
            } else {
                0u64
            }
        }
        Adt4::C29(_) => {
            0u64
        }
        Adt4::C30(_, v52) => {
            let a3170 = v1;
            let a3171 = 6u64;
            if if nat_eq(a3170, a3171) {
                true
            } else {
                let a3172 = v1;
                let a3173 = 7u64;
                nat_eq(a3172, a3173)
            } {
                v52
            } else {
                0u64
            }
        }
    }
}

pub fn f125(v0: Adt4, v1: u64) -> Adt11 {
    let a3181 = v1;
    let a3182 = 4u64;
    if nat_lt(a3181, a3182) {
        Adt11::C0(v1)
    } else {
        let a3183 = v1;
        let a3184 = 4u64;
        if nat_eq(a3183, a3184) {
            Adt11::C1(0u64)
        } else {
            let a3185 = v1;
            let a3186 = 5u64;
            if nat_eq(a3185, a3186) {
                Adt11::C1(3u64)
            } else {
                let a3187 = v1;
                let a3188 = 6u64;
                if nat_eq(a3187, a3188) {
                    Adt11::C2(f124(v0, v1), 0u64, false)
                } else {
                    let a3189 = v1;
                    let a3190 = 7u64;
                    if nat_eq(a3189, a3190) {
                        Adt11::C2(0u64, f124(v0, v1), false)
                    } else {
                        let a3191 = v1;
                        let a3192 = 16u64;
                        if nat_lt(a3191, a3192) {
                            Adt11::C1({
                                let a3193 = v1;
                                let a3194 = 8u64;
                                nat_sub(a3193, a3194)
                            })
                        } else {
                            let a3195 = v1;
                            let a3196 = 20u64;
                            if nat_lt(a3195, a3196) {
                                Adt11::C5({
                                    let a3197 = v1;
                                    let a3198 = 16u64;
                                    nat_sub(a3197, a3198)
                                })
                            } else {
                                let a3199 = v1;
                                let a3200 = 20u64;
                                if nat_eq(a3199, a3200) {
                                    Adt11::C3(0u64, 0u64, false)
                                } else {
                                    let a3201 = v1;
                                    let a3202 = 21u64;
                                    if nat_eq(a3201, a3202) {
                                        Adt11::C0(4u64)
                                    } else {
                                        let a3203 = v1;
                                        let a3204 = 22u64;
                                        if nat_eq(a3203, a3204) {
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

pub fn f126(v0: u64, v1: Adt3) -> R<u64> {
    let v2: u64 = {
        let a3206 = v0;
        let a3207 = match v1.clone() {
            Adt3::C0(h3205, _, _, _, _, _, _, _) => {
                h3205
            }
        };
        let a3208 = 0u64;
        nat_quot(a3206, a3207, a3208)
    };
    let v3: u64 = {
        let a3210 = v2;
        let a3211 = match v1.clone() {
            Adt3::C0(_, h3209, _, _, _, _, _, _) => {
                h3209
            }
        };
        let a3212 = 0u64;
        nat_quot(a3210, a3211, a3212)
    };
    f118({
        let a3214 = v0;
        let a3215 = match v1.clone() {
            Adt3::C0(h3213, _, _, _, _, _, _, _) => {
                h3213
            }
        };
        let a3216 = 0u64;
        nat_rem(a3214, a3215, a3216)
    }, {
        let a3218 = v2;
        let a3219 = match v1.clone() {
            Adt3::C0(_, h3217, _, _, _, _, _, _) => {
                h3217
            }
        };
        let a3220 = 0u64;
        nat_rem(a3218, a3219, a3220)
    }, {
        let a3222 = v3;
        let a3223 = match v1.clone() {
            Adt3::C0(_, _, h3221, _, _, _, _, _) => {
                h3221
            }
        };
        let a3224 = 0u64;
        nat_rem(a3222, a3223, a3224)
    }, {
        let a3226 = v3;
        let a3227 = match v1.clone() {
            Adt3::C0(_, _, h3225, _, _, _, _, _) => {
                h3225
            }
        };
        let a3228 = 0u64;
        nat_quot(a3226, a3227, a3228)
    }, match v1.clone() {
        Adt3::C0(_, _, _, _, h3229, _, _, _) => {
            h3229
        }
    }, match v1.clone() {
        Adt3::C0(_, _, _, _, _, h3230, _, _) => {
            h3230
        }
    }, match v1.clone() {
        Adt3::C0(_, _, _, _, _, _, h3231, _) => {
            h3231
        }
    }, match v1 {
        Adt3::C0(_, _, _, _, _, _, _, h3232) => {
            h3232
        }
    })
}

pub fn f127(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let a3250 = {
        let a3245 = f180(v2, {
            let a3236 = {
                let a3233 = v0;
                let a3234 = v1;
                let a3235 = 0u64;
                nat_quot(a3233, a3234, a3235)
            };
            let a3237 = v2;
            let a3238 = 0u64;
            nat_rem(a3236, a3237, a3238)
        }, {
            let a3242 = {
                let a3239 = v0;
                let a3240 = v1;
                let a3241 = 0u64;
                nat_quot(a3239, a3240, a3241)
            };
            let a3243 = v2;
            let a3244 = 0u64;
            nat_quot(a3242, a3243, a3244)
        })?;
        let a3246 = v1;
        nat_mul(a3245, a3246)?
    };
    let a3251 = {
        let a3247 = v0;
        let a3248 = v1;
        let a3249 = 0u64;
        nat_rem(a3247, a3248, a3249)
    };
    nat_add(a3250, a3251)
}

pub fn f128(v0: u64, v1: Adt3, v2: Adt3) -> R<u64> {
    let v3: u64 = {
        let a3253 = v0;
        let a3254 = match v2.clone() {
            Adt3::C0(h3252, _, _, _, _, _, _, _) => {
                h3252
            }
        };
        let a3255 = 0u64;
        nat_quot(a3253, a3254, a3255)
    };
    let v4: u64 = {
        let a3257 = v3;
        let a3258 = match v2.clone() {
            Adt3::C0(_, h3256, _, _, _, _, _, _) => {
                h3256
            }
        };
        let a3259 = 0u64;
        nat_quot(a3257, a3258, a3259)
    };
    f181({
        let a3261 = v0;
        let a3262 = match v2.clone() {
            Adt3::C0(h3260, _, _, _, _, _, _, _) => {
                h3260
            }
        };
        let a3263 = 0u64;
        nat_rem(a3261, a3262, a3263)
    }, {
        let a3265 = v3;
        let a3266 = match v2.clone() {
            Adt3::C0(_, h3264, _, _, _, _, _, _) => {
                h3264
            }
        };
        let a3267 = 0u64;
        nat_rem(a3265, a3266, a3267)
    }, {
        let a3269 = v4;
        let a3270 = match v2.clone() {
            Adt3::C0(_, _, h3268, _, _, _, _, _) => {
                h3268
            }
        };
        let a3271 = 0u64;
        nat_rem(a3269, a3270, a3271)
    }, {
        let a3273 = v4;
        let a3274 = match v2 {
            Adt3::C0(_, _, h3272, _, _, _, _, _) => {
                h3272
            }
        };
        let a3275 = 0u64;
        nat_quot(a3273, a3274, a3275)
    }, match v1.clone() {
        Adt3::C0(h3276, _, _, _, _, _, _, _) => {
            h3276
        }
    }, match v1.clone() {
        Adt3::C0(_, h3277, _, _, _, _, _, _) => {
            h3277
        }
    }, match v1.clone() {
        Adt3::C0(_, _, h3278, _, _, _, _, _) => {
            h3278
        }
    }, match v1.clone() {
        Adt3::C0(_, _, _, h3279, _, _, _, _) => {
            h3279
        }
    }, match v1.clone() {
        Adt3::C0(_, _, _, _, h3280, _, _, _) => {
            h3280
        }
    }, match v1.clone() {
        Adt3::C0(_, _, _, _, _, h3281, _, _) => {
            h3281
        }
    }, match v1.clone() {
        Adt3::C0(_, _, _, _, _, _, h3282, _) => {
            h3282
        }
    }, match v1 {
        Adt3::C0(_, _, _, _, _, _, _, h3283) => {
            h3283
        }
    })
}

pub fn f129(v0: u64, v1: Adt3, v2: Adt3, v3: Adt3, v4: u64) -> R<Adt9> {
    let v5: u64 = {
        let a3285 = v0;
        let a3286 = match v3.clone() {
            Adt3::C0(h3284, _, _, _, _, _, _, _) => {
                h3284
            }
        };
        let a3287 = 0u64;
        nat_quot(a3285, a3286, a3287)
    };
    let v6: u64 = {
        let a3289 = v5;
        let a3290 = match v3.clone() {
            Adt3::C0(_, h3288, _, _, _, _, _, _) => {
                h3288
            }
        };
        let a3291 = 0u64;
        nat_quot(a3289, a3290, a3291)
    };
    let v7: u64 = {
        let a3293 = v0;
        let a3294 = match v3.clone() {
            Adt3::C0(h3292, _, _, _, _, _, _, _) => {
                h3292
            }
        };
        let a3295 = 0u64;
        nat_rem(a3293, a3294, a3295)
    };
    let v8: u64 = {
        let a3297 = v5;
        let a3298 = match v3.clone() {
            Adt3::C0(_, h3296, _, _, _, _, _, _) => {
                h3296
            }
        };
        let a3299 = 0u64;
        nat_rem(a3297, a3298, a3299)
    };
    let v9: u64 = {
        let a3301 = v6;
        let a3302 = match v3.clone() {
            Adt3::C0(_, _, h3300, _, _, _, _, _) => {
                h3300
            }
        };
        let a3303 = 0u64;
        nat_rem(a3301, a3302, a3303)
    };
    let v10: u64 = {
        let a3305 = v6;
        let a3306 = match v3.clone() {
            Adt3::C0(_, _, h3304, _, _, _, _, _) => {
                h3304
            }
        };
        let a3307 = 0u64;
        nat_quot(a3305, a3306, a3307)
    };
    let v11: u64 = {
        let a3308 = v4;
        let a3309 = 0u64;
        if nat_eq(a3308, a3309) {
            match v1.clone() {
                Adt3::C0(h3310, _, _, _, _, _, _, _) => {
                    h3310
                }
            }
        } else {
            let a3311 = v4;
            let a3312 = 1u64;
            if nat_eq(a3311, a3312) {
                match v1.clone() {
                    Adt3::C0(_, h3313, _, _, _, _, _, _) => {
                        h3313
                    }
                }
            } else {
                let a3314 = v4;
                let a3315 = 2u64;
                if nat_eq(a3314, a3315) {
                    match v1.clone() {
                        Adt3::C0(_, _, h3316, _, _, _, _, _) => {
                            h3316
                        }
                    }
                } else {
                    match v1.clone() {
                        Adt3::C0(_, _, _, h3317, _, _, _, _) => {
                            h3317
                        }
                    }
                }
            }
        }
    };
    let v12: bool = {
        let a3324 = v11;
        let a3325 = {
            let a3318 = v4;
            let a3319 = 0u64;
            if nat_eq(a3318, a3319) {
                v7
            } else {
                let a3320 = v4;
                let a3321 = 1u64;
                if nat_eq(a3320, a3321) {
                    v8
                } else {
                    let a3322 = v4;
                    let a3323 = 2u64;
                    if nat_eq(a3322, a3323) {
                        v9
                    } else {
                        v10
                    }
                }
            }
        };
        nat_le(a3324, a3325)
    };
    let v13: Adt3 = if v12 {
        v2
    } else {
        v1
    };
    Ok(Adt9::C1(v12, f118(if if v12 {
        let a3326 = v4;
        let a3327 = 0u64;
        nat_eq(a3326, a3327)
    } else {
        false
    } {
        let a3328 = v7;
        let a3329 = v11;
        nat_sub(a3328, a3329)
    } else {
        v7
    }, if if v12 {
        let a3330 = v4;
        let a3331 = 1u64;
        nat_eq(a3330, a3331)
    } else {
        false
    } {
        let a3332 = v8;
        let a3333 = v11;
        nat_sub(a3332, a3333)
    } else {
        v8
    }, if if v12 {
        let a3334 = v4;
        let a3335 = 2u64;
        nat_eq(a3334, a3335)
    } else {
        false
    } {
        let a3336 = v9;
        let a3337 = v11;
        nat_sub(a3336, a3337)
    } else {
        v9
    }, if if v12 {
        let a3338 = v4;
        let a3339 = 3u64;
        nat_eq(a3338, a3339)
    } else {
        false
    } {
        let a3340 = v10;
        let a3341 = v11;
        nat_sub(a3340, a3341)
    } else {
        v10
    }, match v13.clone() {
        Adt3::C0(_, _, _, _, h3342, _, _, _) => {
            h3342
        }
    }, match v13.clone() {
        Adt3::C0(_, _, _, _, _, h3343, _, _) => {
            h3343
        }
    }, match v13.clone() {
        Adt3::C0(_, _, _, _, _, _, h3344, _) => {
            h3344
        }
    }, match v13 {
        Adt3::C0(_, _, _, _, _, _, _, h3345) => {
            h3345
        }
    })?, f126(v0, v3)?))
}

pub fn f130(v0: u64, v1: Adt3, v2: Adt3, v3: u64, v4: u64, v5: u64, v6: u64, v7: bool) -> R<Adt9> {
    let v8: u64 = f64(v0, v2.clone(), 0u64);
    let v9: u64 = f64(v0, v2.clone(), 1u64);
    let v10: u64 = f64(v0, v2.clone(), 2u64);
    let v11: u64 = f64(v0, v2.clone(), 3u64);
    if v7 {
        Ok(Adt9::C1(false, f63(v1.clone(), f182(v8, v3, match v1.clone() {
            Adt3::C0(h3346, _, _, _, _, _, _, _) => {
                h3346
            }
        })?, f182(v9, v4, match v1.clone() {
            Adt3::C0(_, h3347, _, _, _, _, _, _) => {
                h3347
            }
        })?, f182(v10, v5, match v1.clone() {
            Adt3::C0(_, _, h3348, _, _, _, _, _) => {
                h3348
            }
        })?, f182(v11, v6, match v1 {
            Adt3::C0(_, _, _, h3349, _, _, _, _) => {
                h3349
            }
        })?)?, f126(v0, v2)?))
    } else {
        if if if if f183(v8, v3, match v1.clone() {
            Adt3::C0(h3350, _, _, _, _, _, _, _) => {
                h3350
            }
        })? {
            f183(v9, v4, match v1.clone() {
                Adt3::C0(_, h3351, _, _, _, _, _, _) => {
                    h3351
                }
            })?
        } else {
            false
        } {
            f183(v10, v5, match v1.clone() {
                Adt3::C0(_, _, h3352, _, _, _, _, _) => {
                    h3352
                }
            })?
        } else {
            false
        } {
            f183(v11, v6, match v1.clone() {
                Adt3::C0(_, _, _, h3353, _, _, _, _) => {
                    h3353
                }
            })?
        } else {
            false
        } {
            Ok(Adt9::C1(false, f63(v1, f184(v8, v3)?, f184(v9, v4)?, f184(v10, v5)?, f184(v11, v6)?)?, f126(v0, v2)?))
        } else {
            Ok(Adt9::C1(true, 0u64, f126(v0, v2)?))
        }
    }
}

pub fn f131(v0: u64, v1: Adt3, v2: Adt3, v3: u64) -> R<Adt9> {
    let v4: u64 = f64(v0, v2.clone(), 0u64);
    let v5: u64 = f64(v0, v2.clone(), 1u64);
    let v6: u64 = f64(v0, v2.clone(), 2u64);
    let v7: u64 = f64(v0, v2.clone(), 3u64);
    Ok(Adt9::C1(false, f63(v1.clone(), f185(v4, v3, match v1 {
        Adt3::C0(h3354, _, _, _, _, _, _, _) => {
            h3354
        }
    })?, v5, v6, v7)?, f126(v0, v2)?))
}

pub fn f132(v0: u64, v1: Adt3, v2: Adt3, v3: u64, v4: u64, v5: bool) -> R<Adt9> {
    let a3355 = v0;
    let a3356 = v3;
    if nat_lt(a3355, a3356) {
        Ok(Adt9::C1(false, {
            let a3357 = v0;
            let a3358 = 4u64;
            nat_mul(a3357, a3358)?
        }, {
            let a3359 = v0;
            let a3360 = 4u64;
            nat_mul(a3359, a3360)?
        }))
    } else {
        let v6: u64 = {
            let a3361 = v0;
            let a3362 = v3;
            nat_sub(a3361, a3362)
        };
        let v7: u64 = f126(v6, v1)?;
        let v8: u64 = f186(v6, v2, v4)?;
        if v5 {
            Ok(Adt9::C4(8u64, 0u64, v8, 4u64, v7, 3u64, v8))
        } else {
            Ok(Adt9::C1(true, v7, v8))
        }
    }
}

pub fn f133(v0: u64, v1: Adt3, v2: Adt3, v3: u64) -> R<Adt9> {
    let a3363 = v0;
    let a3364 = v3;
    if nat_lt(a3363, a3364) {
        Ok(Adt9::C1(true, 0u64, f126(v0, v2)?))
    } else {
        let v4: u64 = {
            let a3365 = v0;
            let a3366 = v3;
            nat_sub(a3365, a3366)
        };
        let v5: u64 = f128(v4, v2, v1.clone())?;
        Ok(Adt9::C4(8u64, 3u64, v5, 0u64, f126(v4, v1)?, 3u64, v5))
    }
}

pub fn f134(v0: u64, v1: u64, v2: Adt3, v3: Adt3) -> R<Adt9> {
    let v4: u64 = {
        let a3368 = v1;
        let a3369 = match v3.clone() {
            Adt3::C0(h3367, _, _, _, _, _, _, _) => {
                h3367
            }
        };
        let a3370 = 0u64;
        nat_rem(a3368, a3369, a3370)
    };
    let v5: u64 = {
        let a3376 = {
            let a3372 = v1;
            let a3373 = match v3.clone() {
                Adt3::C0(h3371, _, _, _, _, _, _, _) => {
                    h3371
                }
            };
            let a3374 = 0u64;
            nat_quot(a3372, a3373, a3374)
        };
        let a3377 = match v3.clone() {
            Adt3::C0(_, h3375, _, _, _, _, _, _) => {
                h3375
            }
        };
        let a3378 = 0u64;
        nat_rem(a3376, a3377, a3378)
    };
    let v6: bool = f187(v0, v4, v5);
    Ok(Adt9::C1(!v6, if v6 {
        f128(v1, v2, v3.clone())?
    } else {
        0u64
    }, f126(v1, v3)?))
}

pub fn f135(v0: u64, v1: Adt3, v2: Adt3, v3: u64, v4: bool, v5: Adt3) -> R<Adt9> {
    Ok(Adt9::C4({
        let a3379 = v3;
        let a3380 = 8u64;
        nat_add(a3379, a3380)?
    }, 0u64, f126(v0, v1)?, if v4 {
        5u64
    } else {
        4u64
    }, if v4 {
        0u64
    } else {
        f128(v0, v5, v2.clone())?
    }, 3u64, f126(v0, v2)?))
}

pub fn f136(v0: u64, v1: Adt3, v2: Adt3, v3: bool, v4: u64, v5: u64) -> R<Adt9> {
    let v6: u64 = {
        let a3382 = v0;
        let a3383 = match v2.clone() {
            Adt3::C0(h3381, _, _, _, _, _, _, _) => {
                h3381
            }
        };
        let a3384 = 0u64;
        nat_rem(a3382, a3383, a3384)
    };
    let v7: u64 = {
        let a3386 = v0;
        let a3387 = match v2.clone() {
            Adt3::C0(h3385, _, _, _, _, _, _, _) => {
                h3385
            }
        };
        let a3388 = 0u64;
        nat_quot(a3386, a3387, a3388)
    };
    let v8: u64 = {
        let a3390 = v7;
        let a3391 = match v2.clone() {
            Adt3::C0(_, h3389, _, _, _, _, _, _) => {
                h3389
            }
        };
        let a3392 = 0u64;
        nat_rem(a3390, a3391, a3392)
    };
    let v9: u64 = {
        let a3394 = v7;
        let a3395 = match v2.clone() {
            Adt3::C0(_, h3393, _, _, _, _, _, _) => {
                h3393
            }
        };
        let a3396 = 0u64;
        nat_quot(a3394, a3395, a3396)
    };
    let v10: u64 = {
        let a3398 = v9;
        let a3399 = match v2.clone() {
            Adt3::C0(_, _, h3397, _, _, _, _, _) => {
                h3397
            }
        };
        let a3400 = 0u64;
        nat_rem(a3398, a3399, a3400)
    };
    let v11: u64 = {
        let a3402 = v9;
        let a3403 = match v2.clone() {
            Adt3::C0(_, _, h3401, _, _, _, _, _) => {
                h3401
            }
        };
        let a3404 = 0u64;
        nat_quot(a3402, a3403, a3404)
    };
    let v12: u64 = f126(v0, v2)?;
    if v3 {
        let v13: u64 = {
            let a3413 = {
                let a3408 = {
                    let a3405 = v10;
                    let a3406 = v4;
                    let a3407 = 0u64;
                    nat_quot(a3405, a3406, a3407)
                };
                let a3409 = v5;
                nat_mul(a3408, a3409)?
            };
            let a3414 = {
                let a3410 = v8;
                let a3411 = v4;
                let a3412 = 0u64;
                nat_quot(a3410, a3411, a3412)
            };
            nat_add(a3413, a3414)?
        };
        let v14: u64 = {
            let a3434 = v6;
            let a3435 = {
                let a3432 = match v1.clone() {
                    Adt3::C0(h3415, _, _, _, _, _, _, _) => {
                        h3415
                    }
                };
                let a3433 = {
                    let a3430 = {
                        let a3416 = v8;
                        let a3417 = v4;
                        let a3418 = 0u64;
                        nat_rem(a3416, a3417, a3418)
                    };
                    let a3431 = {
                        let a3428 = match v1.clone() {
                            Adt3::C0(_, h3419, _, _, _, _, _, _) => {
                                h3419
                            }
                        };
                        let a3429 = {
                            let a3426 = {
                                let a3420 = v10;
                                let a3421 = v4;
                                let a3422 = 0u64;
                                nat_rem(a3420, a3421, a3422)
                            };
                            let a3427 = {
                                let a3424 = match v1.clone() {
                                    Adt3::C0(_, _, h3423, _, _, _, _, _) => {
                                        h3423
                                    }
                                };
                                let a3425 = v13;
                                nat_mul(a3424, a3425)?
                            };
                            nat_add(a3426, a3427)?
                        };
                        nat_mul(a3428, a3429)?
                    };
                    nat_add(a3430, a3431)?
                };
                nat_mul(a3432, a3433)?
            };
            nat_add(a3434, a3435)?
        };
        Ok(Adt9::C1(false, f126(v14, v1)?, v12))
    } else {
        let v15: u64 = {
            let a3441 = {
                let a3439 = {
                    let a3436 = v11;
                    let a3437 = v5;
                    let a3438 = 0u64;
                    nat_rem(a3436, a3437, a3438)
                };
                let a3440 = v4;
                nat_mul(a3439, a3440)?
            };
            let a3442 = v8;
            nat_add(a3441, a3442)?
        };
        let v16: u64 = {
            let a3448 = {
                let a3446 = {
                    let a3443 = v11;
                    let a3444 = v5;
                    let a3445 = 0u64;
                    nat_quot(a3443, a3444, a3445)
                };
                let a3447 = v4;
                nat_mul(a3446, a3447)?
            };
            let a3449 = v10;
            nat_add(a3448, a3449)?
        };
        let a3451 = v15;
        let a3452 = match v1.clone() {
            Adt3::C0(_, h3450, _, _, _, _, _, _) => {
                h3450
            }
        };
        if if nat_lt(a3451, a3452) {
            let a3454 = v16;
            let a3455 = match v1.clone() {
                Adt3::C0(_, _, h3453, _, _, _, _, _) => {
                    h3453
                }
            };
            nat_lt(a3454, a3455)
        } else {
            false
        } {
            Ok(Adt9::C1(false, f126({
                let a3464 = v6;
                let a3465 = {
                    let a3462 = match v1.clone() {
                        Adt3::C0(h3456, _, _, _, _, _, _, _) => {
                            h3456
                        }
                    };
                    let a3463 = {
                        let a3460 = v15;
                        let a3461 = {
                            let a3458 = match v1.clone() {
                                Adt3::C0(_, h3457, _, _, _, _, _, _) => {
                                    h3457
                                }
                            };
                            let a3459 = v16;
                            nat_mul(a3458, a3459)?
                        };
                        nat_add(a3460, a3461)?
                    };
                    nat_mul(a3462, a3463)?
                };
                nat_add(a3464, a3465)?
            }, v1)?, v12))
        } else {
            Ok(Adt9::C1(true, 0u64, v12))
        }
    }
}

pub fn f137(v0: u64, v1: Adt3, v2: Adt3) -> R<Adt9> {
    let v3: u64 = f126(v0, v2.clone())?;
    let v4: u64 = f126(v0, v1.clone())?;
    let a3470 = {
        let a3467 = v0;
        let a3468 = match v1 {
            Adt3::C0(h3466, _, _, _, _, _, _, _) => {
                h3466
            }
        };
        let a3469 = 0u64;
        nat_rem(a3467, a3468, a3469)
    };
    let a3471 = 0u64;
    if nat_eq(a3470, a3471) {
        Ok(Adt9::C1(false, v4, v3))
    } else {
        Ok(Adt9::C4(8u64, 3u64, f126({
            let a3472 = v0;
            let a3473 = 1u64;
            nat_sub(a3472, a3473)
        }, v2)?, 0u64, v4, 3u64, v3))
    }
}

pub fn f138(v0: u64, v1: Adt3, v2: Adt3, v3: u64, v4: u64) -> R<Adt9> {
    let a3474 = v3;
    let a3475 = 13u64;
    if nat_eq(a3474, a3475) {
        f188(v0, v1, v4)
    } else {
        let v5: u64 = f177(match v1.clone() {
            Adt3::C0(h3476, _, _, _, _, _, _, _) => {
                h3476
            }
        }, {
            let a3477 = v3;
            let a3478 = 16u64;
            nat_eq(a3477, a3478)
        })?;
        let v6: u64 = {
            let a3479 = v0;
            let a3480 = v5;
            let a3481 = 0u64;
            nat_quot(a3479, a3480, a3481)
        };
        let v7: u64 = {
            let a3482 = v0;
            let a3483 = v5;
            let a3484 = 0u64;
            nat_rem(a3482, a3483, a3484)
        };
        let v8: u64 = {
            let a3488 = 1u64;
            let a3489 = {
                let a3486 = match v1.clone() {
                    Adt3::C0(h3485, _, _, _, _, _, _, _) => {
                        h3485
                    }
                };
                let a3487 = 2u64;
                nat_mul(a3486, a3487)?
            };
            nat_add(a3488, a3489)?
        };
        let a3490 = v7;
        let a3491 = v8;
        if nat_lt(a3490, a3491) {
            f189(v7, v6, v1, 0u64)
        } else {
            let v9: u64 = f126(v6, v2)?;
            let a3492 = v3;
            let a3493 = 14u64;
            if nat_eq(a3492, a3493) {
                Ok(Adt9::C4(2u64, 1u64, 1u64, 2u64, 0u64, 3u64, v9))
            } else {
                let a3494 = v7;
                let a3495 = v8;
                if nat_eq(a3494, a3495) {
                    Ok(Adt9::C4(2u64, 1u64, 1u64, 2u64, 0u64, 1u64, 1u64))
                } else {
                    let a3498 = v7;
                    let a3499 = {
                        let a3496 = v8;
                        let a3497 = 1u64;
                        nat_add(a3496, a3497)?
                    };
                    if nat_eq(a3498, a3499) {
                        Ok(Adt9::C4(3u64, 2u64, match v1 {
                            Adt3::C0(h3500, _, _, _, _, _, _, _) => {
                                h3500
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

pub fn f139(v0: u64, v1: Adt3, v2: Adt3, v3: u64) -> R<Adt9> {
    let a3501 = v0;
    let a3502 = 0u64;
    if nat_eq(a3501, a3502) {
        Ok(Adt9::C4(19u64, 2u64, 0u64, 2u64, 0u64, 1u64, 0u64))
    } else {
        let a3509 = {
            let a3503 = v0;
            let a3504 = 1u64;
            nat_add(a3503, a3504)?
        };
        let a3510 = {
            let a3507 = 2u64;
            let a3508 = {
                let a3505 = v3;
                let a3506 = 2u64;
                nat_mul(a3505, a3506)?
            };
            nat_add(a3507, a3508)?
        };
        if nat_eq(a3509, a3510) {
            Ok(Adt9::C4(19u64, 1u64, 0u64, 2u64, 0u64, 6u64, 0u64))
        } else {
            let v4: u64 = {
                let a3511 = v0;
                let a3512 = 1u64;
                nat_sub(a3511, a3512)
            };
            let v5: u64 = {
                let a3513 = v4;
                let a3514 = 2u64;
                let a3515 = 0u64;
                nat_quot(a3513, a3514, a3515)
            };
            let a3519 = {
                let a3516 = v4;
                let a3517 = 2u64;
                let a3518 = 0u64;
                nat_rem(a3516, a3517, a3518)
            };
            let a3520 = 0u64;
            if nat_eq(a3519, a3520) {
                Ok(Adt9::C4(17u64, 0u64, f126(v5, v1)?, 4u64, f126(v5, v2)?, 1u64, 2u64))
            } else {
                Ok(Adt9::C4(18u64, 1u64, 0u64, 1u64, 2u64, 1u64, 0u64))
            }
        }
    }
}

pub fn f140(v0: u64, v1: u64, v2: u64, v3: u64) -> R<Adt9> {
    let a3521 = v0;
    let a3522 = f23(v1, v2, v3)?;
    if nat_lt(a3521, a3522) {
        let a3523 = v0;
        let a3524 = v1;
        if nat_lt(a3523, a3524) {
            Ok(Adt9::C1(false, {
                let a3525 = 4u64;
                let a3526 = v0;
                nat_mul(a3525, a3526)?
            }, {
                let a3527 = 4u64;
                let a3528 = v0;
                nat_mul(a3527, a3528)?
            }))
        } else {
            let v4: u64 = {
                let a3529 = v0;
                let a3530 = v1;
                nat_sub(a3529, a3530)
            };
            let v5: u64 = f190(v4, v2)?;
            let a3531 = v5;
            let a3532 = 2u64;
            if nat_lt(a3531, a3532) {
                Ok(Adt9::C4(6u64, {
                    let a3533 = v5;
                    let a3534 = 0u64;
                    if nat_eq(a3533, a3534) {
                        4u64
                    } else {
                        5u64
                    }
                }, f191(v4, v2)?, 2u64, 0u64, 1u64, v5))
            } else {
                let v6: u64 = f192(v4, v2)?;
                Ok(Adt9::C4(8u64, 3u64, v6, 1u64, if f193(v4, v2)? {
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

pub fn f141(v0: Adt3, v1: Adt3) -> R<u64> {
    let a3542 = {
        let a3535 = 2u64;
        let a3536 = f115(v1.clone())?;
        nat_mul(a3535, a3536)?
    };
    let a3543 = {
        let a3540 = f115(v1)?;
        let a3541 = {
            let a3538 = 1u64;
            let a3539 = match v0 {
                Adt3::C0(h3537, _, _, _, _, _, _, _) => {
                    h3537
                }
            };
            nat_add(a3538, a3539)?
        };
        nat_mul(a3540, a3541)?
    };
    nat_add(a3542, a3543)
}

pub fn f142(v0: u64, v1: Adt3) -> R<Adt9> {
    Ok(Adt9::C4(19u64, 4u64, f126(v0, v1)?, 2u64, 0u64, 1u64, 2u64))
}

pub fn f143(v0: u64, v1: u64) -> bool {
    let a3544 = v1;
    let a3545 = 2147483647u64;
    if nat_le(a3544, a3545) {
        let a3546 = v1;
        let a3547 = v0;
        nat_lt(a3546, a3547)
    } else {
        false
    }
}

pub fn f144(v0: u64, v1: u64, v2: u64, v3: Adt3, v4: Adt3, v5: Adt3, v6: u64) -> R<Adt9> {
    let m3548 = f104(v0);
    match m3548 {
        Adt16::C0 => {
            Ok(Adt9::C0(Adt10::C1))
        }
        Adt16::C1(v7) => {
            let v8: u64 = {
                let a3550 = v2;
                let a3551 = match v7.clone() {
                    Adt17::C0(_, h3549, _) => {
                        h3549
                    }
                };
                nat_mul(a3550, a3551)?
            };
            let m3564 = f194(v0, v8, v6, v1, match v5.clone() {
                Adt3::C0(h3552, _, _, _, _, _, _, _) => {
                    h3552
                }
            }, match v5.clone() {
                Adt3::C0(_, h3553, _, _, _, _, _, _) => {
                    h3553
                }
            }, match v5 {
                Adt3::C0(_, _, h3554, _, _, _, _, _) => {
                    h3554
                }
            }, {
                let a3557 = match v3.clone() {
                    Adt3::C0(h3555, _, _, _, _, _, _, _) => {
                        h3555
                    }
                };
                let a3558 = match v7.clone() {
                    Adt17::C0(_, h3556, _) => {
                        h3556
                    }
                };
                nat_mul(a3557, a3558)?
            }, match v3.clone() {
                Adt3::C0(_, h3559, _, _, _, _, _, _) => {
                    h3559
                }
            }, match v3.clone() {
                Adt3::C0(_, _, _, _, h3560, _, _, _) => {
                    h3560
                }
            }, match v3.clone() {
                Adt3::C0(_, _, _, _, _, h3561, _, _) => {
                    h3561
                }
            }, match v3.clone() {
                Adt3::C0(_, _, _, _, _, _, h3562, _) => {
                    h3562
                }
            }, match v3 {
                Adt3::C0(_, _, _, _, _, _, _, h3563) => {
                    h3563
                }
            })?;
            match m3564 {
                None => {
                    Ok(Adt9::C0(Adt10::C1))
                }
                Some(v9) => {
                    let a3565 = v0;
                    let a3566 = 0u64;
                    if if nat_eq(a3565, a3566) {
                        true
                    } else {
                        let a3567 = v0;
                        let a3568 = 26u64;
                        nat_eq(a3567, a3568)
                    } {
                        Ok(Adt9::C1(false, v9, f126({
                            let a3572 = {
                                let a3570 = v1;
                                let a3571 = match v4.clone() {
                                    Adt3::C0(h3569, _, _, _, _, _, _, _) => {
                                        h3569
                                    }
                                };
                                nat_mul(a3570, a3571)?
                            };
                            let a3573 = v8;
                            nat_add(a3572, a3573)?
                        }, v4)?))
                    } else {
                        Ok(Adt9::C5(v0, v9, match v7.clone() {
                            Adt17::C0(_, _, h3574) => {
                                h3574
                            }
                        }, f126({
                            let a3578 = {
                                let a3576 = v1;
                                let a3577 = match v4.clone() {
                                    Adt3::C0(h3575, _, _, _, _, _, _, _) => {
                                        h3575
                                    }
                                };
                                nat_mul(a3576, a3577)?
                            };
                            let a3579 = v8;
                            nat_add(a3578, a3579)?
                        }, v4.clone())?, {
                            let a3586 = {
                                let a3584 = {
                                    let a3581 = match v7 {
                                        Adt17::C0(_, h3580, _) => {
                                            h3580
                                        }
                                    };
                                    let a3582 = 1u64;
                                    nat_sub(a3581, a3582)
                                };
                                let a3585 = match v4.clone() {
                                    Adt3::C0(_, _, _, _, h3583, _, _, _) => {
                                        h3583
                                    }
                                };
                                nat_mul(a3584, a3585)?
                            };
                            let a3587 = 4u64;
                            nat_add(a3586, a3587)?
                        }, match v4 {
                            Adt3::C0(_, _, _, _, h3588, _, _, _) => {
                                h3588
                            }
                        }))
                    }
                }
            }
        }
    }
}

pub fn f145(v0: u64, v1: u64, v2: u64) -> u64 {
    let a3589 = v1;
    let a3590 = 0u64;
    if if nat_eq(a3589, a3590) {
        true
    } else {
        let a3591 = v2;
        let a3592 = 0u64;
        nat_eq(a3591, a3592)
    } {
        0u64
    } else {
        v0
    }
}

pub fn f146(v0: u64, v1: u64) -> bool {
    let a3593 = v0;
    let a3594 = 4294967295u64;
    if nat_eq(a3593, a3594) {
        true
    } else {
        let a3595 = v0;
        let a3596 = 2147483648u64;
        if nat_lt(a3595, a3596) {
            let a3597 = v0;
            let a3598 = v1;
            nat_lt(a3597, a3598)
        } else {
            false
        }
    }
}

pub fn f147(v0: Adt7) -> R<u64> {
    let a3603 = if match v0.clone() {
        Adt7::C0(_, _, _, h3599, _) => {
            h3599
        }
    } {
        2u64
    } else {
        1u64
    };
    let a3604 = {
        let a3601 = match v0 {
            Adt7::C0(_, _, _, _, h3600) => {
                h3600
            }
        };
        let a3602 = 1u64;
        if nat_eq(a3601, a3602) {
            1u64
        } else {
            0u64
        }
    };
    nat_add(a3603, a3604)
}

pub fn f148(v0: Adt6, v1: u64, v2: u64) -> R<u64> {
    let v3: u64 = {
        let a3609 = {
            let a3606 = v2;
            let a3607 = match v0.clone() {
                Adt6::C0(_, h3605, _, _) => {
                    h3605
                }
            };
            nat_mul(a3606, a3607)?
        };
        let a3610 = f119(match v0.clone() {
            Adt6::C0(_, _, h3608, _) => {
                h3608
            }
        });
        nat_add(a3609, a3610)?
    };
    let v4: u64 = f120(match v0.clone() {
        Adt6::C0(_, _, h3611, _) => {
            h3611
        }
    });
    let v5: u64 = {
        let a3615 = {
            let a3613 = v3;
            let a3614 = match v0 {
                Adt6::C0(h3612, _, _, _) => {
                    h3612
                }
            };
            nat_add(a3613, a3614)?
        };
        let a3616 = v4;
        nat_sub(a3615, a3616)
    };
    let a3621 = {
        let a3617 = v5;
        let a3618 = v1;
        if nat_lt(a3617, a3618) {
            v5
        } else {
            v1
        }
    };
    let a3622 = {
        let a3619 = v3;
        let a3620 = v4;
        nat_sub(a3619, a3620)
    };
    Ok(nat_sub(a3621, a3622))
}

pub fn f149(v0: Adt7, v1: Adt3, v2: Adt3, v3: u64, v4: u64) -> R<Option<u64>> {
    let m3631 = f66(match v0.clone() {
        Adt7::C0(h3623, _, _, _, _) => {
            h3623
        }
    }, match v1.clone() {
        Adt3::C0(h3624, _, _, _, _, _, _, _) => {
            h3624
        }
    }, f64(v3, v2.clone(), 0u64), {
        let a3628 = v4;
        let a3629 = {
            let m3627 = match v0.clone() {
                Adt7::C0(h3626, _, _, _, _) => {
                    h3626
                }
            };
            match m3627 {
                Adt6::C0(h3625, _, _, _) => {
                    h3625
                }
            }
        };
        let a3630 = 0u64;
        nat_rem(a3628, a3629, a3630)
    })?;
    match m3631 {
        None => {
            Ok(None::<u64>)
        }
        Some(v5) => {
            let m3640 = f66(match v0.clone() {
                Adt7::C0(_, h3632, _, _, _) => {
                    h3632
                }
            }, match v1.clone() {
                Adt3::C0(_, h3633, _, _, _, _, _, _) => {
                    h3633
                }
            }, f64(v3, v2.clone(), 1u64), {
                let a3637 = v4;
                let a3638 = {
                    let m3636 = match v0 {
                        Adt7::C0(h3635, _, _, _, _) => {
                            h3635
                        }
                    };
                    match m3636 {
                        Adt6::C0(h3634, _, _, _) => {
                            h3634
                        }
                    }
                };
                let a3639 = 0u64;
                nat_quot(a3637, a3638, a3639)
            })?;
            match m3640 {
                None => {
                    Ok(None::<u64>)
                }
                Some(v6) => {
                    Ok(Some(f63(v1, v5, v6, f64(v3, v2.clone(), 2u64), f64(v3, v2, 3u64))?))
                }
            }
        }
    }
}

pub fn f150(v0: u64, v1: Adt3, v2: u64, v3: u64, v4: u64) -> R<Adt9> {
    let v5: u64 = {
        let a3647 = v3;
        let a3648 = {
            let a3644 = f126({
                let a3641 = v0;
                let a3642 = 2u64;
                let a3643 = 0u64;
                nat_quot(a3641, a3642, a3643)
            }, v1)?;
            let a3645 = v2;
            let a3646 = 0u64;
            nat_quot(a3644, a3645, a3646)
        };
        nat_add(a3647, a3648)?
    };
    let a3652 = {
        let a3649 = v0;
        let a3650 = 2u64;
        let a3651 = 0u64;
        nat_rem(a3649, a3650, a3651)
    };
    let a3653 = 0u64;
    if nat_eq(a3652, a3653) {
        Ok(Adt9::C4(19u64, 1u64, v5, 2u64, 0u64, 1u64, 2u64))
    } else {
        let a3654 = v4;
        let a3655 = 0u64;
        if nat_eq(a3654, a3655) {
            Ok(Adt9::C4(19u64, 2u64, 1u64, 2u64, 0u64, 1u64, v5))
        } else {
            Ok(Adt9::C0(Adt10::C1))
        }
    }
}

pub fn f151(v0: u64, v1: u64) -> R<u64> {
    let a3658 = 8u64;
    let a3659 = {
        let a3656 = f195(v0);
        let a3657 = v1;
        nat_mul(a3656, a3657)?
    };
    nat_add(a3658, a3659)
}

pub fn f152(v0: u64, v1: u64, v2: Adt3, v3: u64, v4: u64, v5: u64) -> R<Adt9> {
    let v6: u64 = match v2.clone() {
        Adt3::C0(h3660, _, _, _, _, _, _, _) => {
            h3660
        }
    };
    let v7: u64 = f195(v3);
    let v8: u64 = {
        let a3663 = 1u64;
        let a3664 = {
            let a3661 = v6;
            let a3662 = v7;
            nat_mul(a3661, a3662)?
        };
        nat_add(a3663, a3664)?
    };
    let a3665 = v0;
    let a3666 = 0u64;
    if nat_eq(a3665, a3666) {
        Ok(f196(0u64))
    } else {
        let a3667 = v0;
        let a3668 = v8;
        if nat_lt(a3667, a3668) {
            let v9: u64 = {
                let a3669 = v0;
                let a3670 = 1u64;
                nat_sub(a3669, a3670)
            };
            let v10: u64 = {
                let a3671 = v9;
                let a3672 = v7;
                let a3673 = 0u64;
                nat_rem(a3671, a3672, a3673)
            };
            let v11: u64 = f126({
                let a3679 = {
                    let a3674 = v1;
                    let a3675 = v6;
                    nat_mul(a3674, a3675)?
                };
                let a3680 = {
                    let a3676 = v9;
                    let a3677 = v7;
                    let a3678 = 0u64;
                    nat_quot(a3676, a3677, a3678)
                };
                nat_add(a3679, a3680)?
            }, v2)?;
            let a3681 = v3;
            let a3682 = 30u64;
            if nat_eq(a3681, a3682) {
                let a3683 = v10;
                let a3684 = 0u64;
                if nat_eq(a3683, a3684) {
                    Ok(Adt9::C4(6u64, 0u64, v11, 2u64, 0u64, 1u64, 1u64))
                } else {
                    let a3685 = v10;
                    let a3686 = 1u64;
                    if nat_eq(a3685, a3686) {
                        Ok(Adt9::C4(10u64, 1u64, 1u64, 1u64, 1u64, 1u64, 1u64))
                    } else {
                        let a3687 = v10;
                        let a3688 = 2u64;
                        if nat_eq(a3687, a3688) {
                            Ok(Adt9::C4(1u64, 1u64, 1u64, 2u64, 0u64, 1u64, 1u64))
                        } else {
                            Ok(Adt9::C4(0u64, 1u64, 0u64, 1u64, 1u64, 1u64, 0u64))
                        }
                    }
                }
            } else {
                let a3689 = v10;
                let a3690 = 0u64;
                if nat_eq(a3689, a3690) {
                    Ok(Adt9::C4(10u64, 0u64, v11, 0u64, v11, 1u64, 1u64))
                } else {
                    let a3691 = v10;
                    let a3692 = 1u64;
                    if nat_eq(a3691, a3692) {
                        Ok(Adt9::C4(1u64, 1u64, 1u64, 2u64, 0u64, 1u64, 1u64))
                    } else {
                        Ok(Adt9::C4(0u64, 1u64, 0u64, 1u64, 1u64, 1u64, 0u64))
                    }
                }
            }
        } else {
            let v12: u64 = {
                let a3693 = v0;
                let a3694 = v8;
                nat_sub(a3693, a3694)
            };
            let a3695 = v12;
            let a3696 = 0u64;
            if nat_eq(a3695, a3696) {
                Ok(Adt9::C4(22u64, 2u64, v6, 2u64, 0u64, 1u64, 1u64))
            } else {
                let a3697 = v12;
                let a3698 = 1u64;
                if nat_eq(a3697, a3698) {
                    Ok(Adt9::C4(21u64, 1u64, 0u64, 1u64, 1u64, 1u64, 1u64))
                } else {
                    let a3699 = v12;
                    let a3700 = 2u64;
                    if nat_eq(a3699, a3700) {
                        Ok(Adt9::C4(2u64, 1u64, 1u64, 2u64, 0u64, 1u64, 1u64))
                    } else {
                        let a3701 = v12;
                        let a3702 = 3u64;
                        if nat_eq(a3701, a3702) {
                            Ok(Adt9::C4(4u64, 1u64, 1u64, 2u64, v4, 1u64, 1u64))
                        } else {
                            let a3703 = v12;
                            let a3704 = 4u64;
                            if nat_eq(a3703, a3704) {
                                Ok(Adt9::C4(12u64, 1u64, 1u64, 2u64, 0u64, 1u64, 1u64))
                            } else {
                                let a3705 = v12;
                                let a3706 = 5u64;
                                if nat_eq(a3705, a3706) {
                                    Ok(Adt9::C4(11u64, 2u64, 1065353216u64, 1u64, 1u64, 1u64, 2u64))
                                } else {
                                    let a3707 = v12;
                                    let a3708 = 6u64;
                                    if nat_eq(a3707, a3708) {
                                        if f197(v5) {
                                            Ok(Adt9::C4(19u64, 1u64, 2u64, 2u64, 0u64, 1u64, {
                                                let a3709 = 3u64;
                                                let a3710 = v1;
                                                nat_add(a3709, a3710)?
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

pub fn f153(v0: u64, v1: Adt3, v2: Adt3, v3: u64) -> R<Adt9> {
    let v4: u64 = {
        let a3711 = v0;
        let a3712 = f93(v3);
        let a3713 = 0u64;
        nat_quot(a3711, a3712, a3713)
    };
    let v5: u64 = {
        let a3718 = 3u64;
        let a3719 = {
            let a3715 = v4;
            let a3716 = match v1.clone() {
                Adt3::C0(h3714, _, _, _, _, _, _, _) => {
                    h3714
                }
            };
            let a3717 = 0u64;
            nat_quot(a3715, a3716, a3717)
        };
        nat_add(a3718, a3719)?
    };
    let v6: u64 = f126(v4, v1)?;
    let v7: u64 = f126(v4, v2)?;
    let a3720 = v3;
    let a3721 = 30u64;
    if nat_eq(a3720, a3721) {
        let a3725 = {
            let a3722 = v0;
            let a3723 = 3u64;
            let a3724 = 0u64;
            nat_rem(a3722, a3723, a3724)
        };
        let a3726 = 0u64;
        if nat_eq(a3725, a3726) {
            Ok(Adt9::C4(6u64, 0u64, v6, 2u64, 0u64, 1u64, 1u64))
        } else {
            let a3730 = {
                let a3727 = v0;
                let a3728 = 3u64;
                let a3729 = 0u64;
                nat_rem(a3727, a3728, a3729)
            };
            let a3731 = 1u64;
            if nat_eq(a3730, a3731) {
                Ok(Adt9::C4(10u64, 1u64, 1u64, 1u64, v5, 1u64, 1u64))
            } else {
                Ok(Adt9::C4(7u64, 1u64, 1u64, 2u64, 0u64, 3u64, v7))
            }
        }
    } else {
        Ok(Adt9::C4(10u64, 0u64, v6, 1u64, v5, 3u64, v7))
    }
}

pub fn f154(v0: u64) -> R<u64> {
    let v1: u64 = {
        let a3732 = v0;
        let a3733 = 8388608u64;
        let a3734 = 0u64;
        nat_quot(a3732, a3733, a3734)
    };
    let a3735 = v0;
    let a3736 = 2147483648u64;
    if if if nat_lt(a3735, a3736) {
        let a3737 = 127u64;
        let a3738 = v1;
        nat_le(a3737, a3738)
    } else {
        false
    } {
        let a3739 = v1;
        let a3740 = 190u64;
        nat_lt(a3739, a3740)
    } else {
        false
    } {
        let v2: u64 = {
            let a3744 = 8388608u64;
            let a3745 = {
                let a3741 = v0;
                let a3742 = 8388608u64;
                let a3743 = 0u64;
                nat_rem(a3741, a3742, a3743)
            };
            nat_add(a3744, a3745)?
        };
        let a3746 = v1;
        let a3747 = 150u64;
        if nat_lt(a3746, a3747) {
            let v3: u64 = f198({
                let a3748 = 150u64;
                let a3749 = v1;
                nat_sub(a3748, a3749)
            });
            let a3753 = {
                let a3750 = v2;
                let a3751 = v3;
                let a3752 = 0u64;
                nat_rem(a3750, a3751, a3752)
            };
            let a3754 = 0u64;
            if nat_eq(a3753, a3754) {
                let a3755 = v2;
                let a3756 = v3;
                let a3757 = 0u64;
                Ok(nat_quot(a3755, a3756, a3757))
            } else {
                Ok(0u64)
            }
        } else {
            let a3760 = f198({
                let a3758 = v1;
                let a3759 = 150u64;
                nat_sub(a3758, a3759)
            });
            let a3761 = v2;
            nat_mul(a3760, a3761)
        }
    } else {
        Ok(0u64)
    }
}

pub fn f155(v0: Adt14, v1: Adt14) -> bool {
    let a3764 = match v0.clone() {
        Adt14::C0(_, h3762, _, _, _, _, _, _, _, _, _) => {
            h3762
        }
    };
    let a3765 = match v1.clone() {
        Adt14::C0(_, h3763, _, _, _, _, _, _, _, _, _) => {
            h3763
        }
    };
    if if if nat_eq(a3764, a3765) {
        let a3768 = match v0.clone() {
            Adt14::C0(_, _, h3766, _, _, _, _, _, _, _, _) => {
                h3766
            }
        };
        let a3769 = match v1.clone() {
            Adt14::C0(_, _, h3767, _, _, _, _, _, _, _, _) => {
                h3767
            }
        };
        nat_eq(a3768, a3769)
    } else {
        false
    } {
        let a3772 = match v0.clone() {
            Adt14::C0(_, _, _, h3770, _, _, _, _, _, _, _) => {
                h3770
            }
        };
        let a3773 = match v1.clone() {
            Adt14::C0(_, _, _, h3771, _, _, _, _, _, _, _) => {
                h3771
            }
        };
        nat_eq(a3772, a3773)
    } else {
        false
    } {
        let a3776 = match v0 {
            Adt14::C0(_, _, _, _, h3774, _, _, _, _, _, _) => {
                h3774
            }
        };
        let a3777 = match v1 {
            Adt14::C0(_, _, _, _, h3775, _, _, _, _, _, _) => {
                h3775
            }
        };
        nat_eq(a3776, a3777)
    } else {
        false
    }
}

pub fn f156(v0: u64, v1: u64, v2: u64, v3: u64) -> bool {
    let a3778 = v0;
    let a3779 = 0u64;
    if if if nat_eq(a3778, a3779) {
        true
    } else {
        let a3780 = v1;
        let a3781 = 0u64;
        nat_eq(a3780, a3781)
    } {
        true
    } else {
        let a3782 = v2;
        let a3783 = 0u64;
        nat_eq(a3782, a3783)
    } {
        true
    } else {
        let a3784 = v3;
        let a3785 = 0u64;
        nat_eq(a3784, a3785)
    }
}

pub fn f157(v0: Adt14, v1: Adt14) -> bool {
    if f163(match v0.clone() {
        Adt14::C0(_, _, _, _, _, h3786, _, _, _, _, _) => {
            h3786
        }
    }, match v1 {
        Adt14::C0(_, _, _, _, _, h3787, _, _, _, _, _) => {
            h3787
        }
    }) {
        let a3789 = match v0 {
            Adt14::C0(_, h3788, _, _, _, _, _, _, _, _, _) => {
                h3788
            }
        };
        let a3790 = 2147483647u64;
        nat_le(a3789, a3790)
    } else {
        true
    }
}

pub fn f158(v0: Adt14) -> bool {
    let a3795 = {
        let a3792 = match v0.clone() {
            Adt14::C0(_, _, _, _, _, _, _, _, _, _, h3791) => {
                h3791
            }
        };
        let a3793 = 4u64;
        let a3794 = 0u64;
        nat_rem(a3792, a3793, a3794)
    };
    let a3796 = 0u64;
    if if nat_eq(a3795, a3796) {
        f199(match v0.clone() {
            Adt14::C0(_, h3797, _, _, _, _, _, _, _, _, _) => {
                h3797
            }
        }, match v0.clone() {
            Adt14::C0(_, _, _, _, _, h3798, _, _, _, _, _) => {
                h3798
            }
        })
    } else {
        false
    } {
        f89(match v0.clone() {
            Adt14::C0(_, _, h3799, _, _, _, _, _, _, _, _) => {
                h3799
            }
        }, match v0.clone() {
            Adt14::C0(_, _, _, h3800, _, _, _, _, _, _, _) => {
                h3800
            }
        }, match v0.clone() {
            Adt14::C0(_, _, _, _, h3801, _, _, _, _, _, _) => {
                h3801
            }
        }, match v0.clone() {
            Adt14::C0(_, _, _, _, _, _, h3802, _, _, _, _) => {
                h3802
            }
        }, match v0.clone() {
            Adt14::C0(_, _, _, _, _, _, _, h3803, _, _, _) => {
                h3803
            }
        }, match v0 {
            Adt14::C0(_, _, _, _, _, _, _, _, h3804, _, _) => {
                h3804
            }
        }, 4u64)
    } else {
        false
    }
}

pub fn f159(v0: Adt14, v1: Adt14) -> R<bool> {
    let a3806 = match v0.clone() {
        Adt14::C0(_, _, _, _, _, _, _, _, _, h3805, _) => {
            h3805
        }
    };
    let a3807 = 0u64;
    if nat_eq(a3806, a3807) {
        Ok(true)
    } else {
        let a3809 = match v1.clone() {
            Adt14::C0(_, _, _, _, _, _, _, _, _, h3808, _) => {
                h3808
            }
        };
        let a3810 = 0u64;
        if nat_eq(a3809, a3810) {
            Ok(true)
        } else {
            let a3812 = match v0.clone() {
                Adt14::C0(_, _, _, _, _, _, _, _, _, _, h3811) => {
                    h3811
                }
            };
            let a3813 = 0u64;
            if nat_eq(a3812, a3813) {
                Ok(true)
            } else {
                let a3815 = match v1.clone() {
                    Adt14::C0(_, _, _, _, _, _, _, _, _, _, h3814) => {
                        h3814
                    }
                };
                let a3816 = 0u64;
                if nat_eq(a3815, a3816) {
                    Ok(true)
                } else {
                    let a3822 = {
                        let a3819 = match v0.clone() {
                            Adt14::C0(_, _, _, _, _, _, _, _, _, _, h3817) => {
                                h3817
                            }
                        };
                        let a3820 = match v0.clone() {
                            Adt14::C0(_, _, _, _, _, _, _, _, _, h3818, _) => {
                                h3818
                            }
                        };
                        nat_add(a3819, a3820)?
                    };
                    let a3823 = match v1.clone() {
                        Adt14::C0(_, _, _, _, _, _, _, _, _, _, h3821) => {
                            h3821
                        }
                    };
                    if nat_le(a3822, a3823) {
                        Ok(true)
                    } else {
                        let a3829 = {
                            let a3826 = match v1.clone() {
                                Adt14::C0(_, _, _, _, _, _, _, _, _, _, h3824) => {
                                    h3824
                                }
                            };
                            let a3827 = match v1 {
                                Adt14::C0(_, _, _, _, _, _, _, _, _, h3825, _) => {
                                    h3825
                                }
                            };
                            nat_add(a3826, a3827)?
                        };
                        let a3830 = match v0 {
                            Adt14::C0(_, _, _, _, _, _, _, _, _, _, h3828) => {
                                h3828
                            }
                        };
                        Ok(nat_le(a3829, a3830))
                    }
                }
            }
        }
    }
}

pub fn f160(v0: Adt14, v1: Adt14) -> bool {
    if if if if if if f155(v0.clone(), v1.clone()) {
        let a3833 = match v0.clone() {
            Adt14::C0(h3831, _, _, _, _, _, _, _, _, _, _) => {
                h3831
            }
        };
        let a3834 = match v1.clone() {
            Adt14::C0(h3832, _, _, _, _, _, _, _, _, _, _) => {
                h3832
            }
        };
        nat_eq(a3833, a3834)
    } else {
        false
    } {
        let a3837 = match v0.clone() {
            Adt14::C0(_, _, _, _, _, _, _, _, _, _, h3835) => {
                h3835
            }
        };
        let a3838 = match v1.clone() {
            Adt14::C0(_, _, _, _, _, _, _, _, _, _, h3836) => {
                h3836
            }
        };
        nat_eq(a3837, a3838)
    } else {
        false
    } {
        let a3841 = match v0.clone() {
            Adt14::C0(_, _, _, _, _, h3839, _, _, _, _, _) => {
                h3839
            }
        };
        let a3842 = match v1.clone() {
            Adt14::C0(_, _, _, _, _, h3840, _, _, _, _, _) => {
                h3840
            }
        };
        nat_eq(a3841, a3842)
    } else {
        false
    } {
        let a3845 = match v0.clone() {
            Adt14::C0(_, _, _, _, _, _, h3843, _, _, _, _) => {
                h3843
            }
        };
        let a3846 = match v1.clone() {
            Adt14::C0(_, _, _, _, _, _, h3844, _, _, _, _) => {
                h3844
            }
        };
        nat_eq(a3845, a3846)
    } else {
        false
    } {
        let a3849 = match v0.clone() {
            Adt14::C0(_, _, _, _, _, _, _, h3847, _, _, _) => {
                h3847
            }
        };
        let a3850 = match v1.clone() {
            Adt14::C0(_, _, _, _, _, _, _, h3848, _, _, _) => {
                h3848
            }
        };
        nat_eq(a3849, a3850)
    } else {
        false
    } {
        let a3853 = match v0 {
            Adt14::C0(_, _, _, _, _, _, _, _, h3851, _, _) => {
                h3851
            }
        };
        let a3854 = match v1 {
            Adt14::C0(_, _, _, _, _, _, _, _, h3852, _, _) => {
                h3852
            }
        };
        nat_eq(a3853, a3854)
    } else {
        false
    }
}

pub fn f161(v0: Adt14) -> R<bool> {
    if f200(match v0.clone() {
        Adt14::C0(_, h3855, _, _, _, _, _, _, _, _, _) => {
            h3855
        }
    }, match v0.clone() {
        Adt14::C0(_, _, _, _, _, h3856, _, _, _, _, _) => {
            h3856
        }
    }) {
        f90(f201(match v0.clone() {
            Adt14::C0(_, h3857, _, _, _, _, _, _, _, _, _) => {
                h3857
            }
        }, match v0.clone() {
            Adt14::C0(_, _, _, _, _, h3858, _, _, _, _, _) => {
                h3858
            }
        })?, match v0.clone() {
            Adt14::C0(_, _, h3859, _, _, _, _, _, _, _, _) => {
                h3859
            }
        }, match v0.clone() {
            Adt14::C0(_, _, _, h3860, _, _, _, _, _, _, _) => {
                h3860
            }
        }, match v0.clone() {
            Adt14::C0(_, _, _, _, h3861, _, _, _, _, _, _) => {
                h3861
            }
        }, match v0.clone() {
            Adt14::C0(_, _, _, _, _, _, h3862, _, _, _, _) => {
                h3862
            }
        }, match v0.clone() {
            Adt14::C0(_, _, _, _, _, _, _, h3863, _, _, _) => {
                h3863
            }
        }, match v0 {
            Adt14::C0(_, _, _, _, _, _, _, _, h3864, _, _) => {
                h3864
            }
        })
    } else {
        Ok(false)
    }
}

pub fn f162(v0: bool, v1: u64, v2: u64, v3: u64) -> R<u64> {
    let a3865 = v1;
    let a3866 = 0u64;
    if if nat_eq(a3865, a3866) {
        true
    } else {
        let a3867 = v2;
        let a3868 = 0u64;
        nat_eq(a3867, a3868)
    } {
        Ok(0u64)
    } else {
        let a3871 = f91(v1, v2, v3)?;
        let a3872 = {
            let a3869 = v2;
            let a3870 = f202(v0, v1)?;
            nat_mul(a3869, a3870)?
        };
        nat_add(a3871, a3872)
    }
}

pub fn f163(v0: u64, v1: u64) -> bool {
    let a3873 = v0;
    let a3874 = 4u64;
    if nat_eq(a3873, a3874) {
        let a3875 = v1;
        let a3876 = 4u64;
        nat_eq(a3875, a3876)
    } else {
        false
    }
}

pub fn f164(v0: u64) -> R<u64> {
    let a3877 = 6u64;
    let a3878 = v0;
    nat_add(a3877, a3878)
}

pub fn f165(v0: Adt14) -> Adt14 {
    Adt14::C0(match v0.clone() {
        Adt14::C0(h3879, _, _, _, _, _, _, _, _, _, _) => {
            h3879
        }
    }, match v0.clone() {
        Adt14::C0(_, h3880, _, _, _, _, _, _, _, _, _) => {
            h3880
        }
    }, match v0.clone() {
        Adt14::C0(_, _, h3881, _, _, _, _, _, _, _, _) => {
            h3881
        }
    }, match v0.clone() {
        Adt14::C0(_, _, _, h3882, _, _, _, _, _, _, _) => {
            h3882
        }
    }, match v0.clone() {
        Adt14::C0(_, _, _, _, h3883, _, _, _, _, _, _) => {
            h3883
        }
    }, 4u64, match v0.clone() {
        Adt14::C0(_, _, _, _, _, _, h3884, _, _, _, _) => {
            h3884
        }
    }, match v0.clone() {
        Adt14::C0(_, _, _, _, _, _, _, h3885, _, _, _) => {
            h3885
        }
    }, match v0.clone() {
        Adt14::C0(_, _, _, _, _, _, _, _, h3886, _, _) => {
            h3886
        }
    }, match v0.clone() {
        Adt14::C0(_, _, _, _, _, _, _, _, _, h3887, _) => {
            h3887
        }
    }, match v0 {
        Adt14::C0(_, _, _, _, _, _, _, _, _, _, h3888) => {
            h3888
        }
    })
}

pub fn f166(v0: u64, v1: u64, v2: u64, v3: u64) -> R<bool> {
    let a3889 = v0;
    let a3890 = 2147483647u64;
    if if if nat_le(a3889, a3890) {
        let a3891 = 0u64;
        let a3892 = v3;
        if nat_lt(a3891, a3892) {
            let a3893 = v3;
            let a3894 = 2147483647u64;
            nat_le(a3893, a3894)
        } else {
            false
        }
    } else {
        false
    } {
        let a3899 = {
            let a3897 = {
                let a3895 = v2;
                let a3896 = v3;
                nat_add(a3895, a3896)?
            };
            let a3898 = 1u64;
            nat_sub(a3897, a3898)
        };
        let a3900 = 2147483647u64;
        nat_le(a3899, a3900)
    } else {
        false
    } {
        let a3905 = {
            let a3903 = {
                let a3901 = v0;
                let a3902 = v1;
                nat_mul(a3901, a3902)?
            };
            let a3904 = f203(v2, v3)?;
            nat_mul(a3903, a3904)?
        };
        let a3906 = 9223372036854775807u64;
        Ok(nat_le(a3905, a3906))
    } else {
        Ok(false)
    }
}

pub fn f167(v0: Adt14) -> R<u64> {
    let a3937 = {
        let a3929 = {
            let a3921 = {
                let a3913 = 4u64;
                let a3914 = {
                    let a3911 = {
                        let a3908 = match v0.clone() {
                            Adt14::C0(_, h3907, _, _, _, _, _, _, _, _, _) => {
                                h3907
                            }
                        };
                        let a3909 = 1u64;
                        nat_sub(a3908, a3909)
                    };
                    let a3912 = match v0.clone() {
                        Adt14::C0(_, _, _, _, _, h3910, _, _, _, _, _) => {
                            h3910
                        }
                    };
                    nat_mul(a3911, a3912)?
                };
                nat_add(a3913, a3914)?
            };
            let a3922 = {
                let a3919 = {
                    let a3916 = match v0.clone() {
                        Adt14::C0(_, _, h3915, _, _, _, _, _, _, _, _) => {
                            h3915
                        }
                    };
                    let a3917 = 1u64;
                    nat_sub(a3916, a3917)
                };
                let a3920 = match v0.clone() {
                    Adt14::C0(_, _, _, _, _, _, h3918, _, _, _, _) => {
                        h3918
                    }
                };
                nat_mul(a3919, a3920)?
            };
            nat_add(a3921, a3922)?
        };
        let a3930 = {
            let a3927 = {
                let a3924 = match v0.clone() {
                    Adt14::C0(_, _, _, h3923, _, _, _, _, _, _, _) => {
                        h3923
                    }
                };
                let a3925 = 1u64;
                nat_sub(a3924, a3925)
            };
            let a3928 = match v0.clone() {
                Adt14::C0(_, _, _, _, _, _, _, h3926, _, _, _) => {
                    h3926
                }
            };
            nat_mul(a3927, a3928)?
        };
        nat_add(a3929, a3930)?
    };
    let a3938 = {
        let a3935 = {
            let a3932 = match v0.clone() {
                Adt14::C0(_, _, _, _, h3931, _, _, _, _, _, _) => {
                    h3931
                }
            };
            let a3933 = 1u64;
            nat_sub(a3932, a3933)
        };
        let a3936 = match v0 {
            Adt14::C0(_, _, _, _, _, _, _, _, h3934, _, _) => {
                h3934
            }
        };
        nat_mul(a3935, a3936)?
    };
    nat_add(a3937, a3938)
}

pub fn f168(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64) -> R<u64> {
    let a3939 = v0;
    let a3940 = 0u64;
    if if if if nat_eq(a3939, a3940) {
        true
    } else {
        let a3941 = v1;
        let a3942 = 0u64;
        nat_eq(a3941, a3942)
    } {
        true
    } else {
        let a3943 = v2;
        let a3944 = 0u64;
        nat_eq(a3943, a3944)
    } {
        true
    } else {
        let a3945 = v3;
        let a3946 = 0u64;
        nat_eq(a3945, a3946)
    } {
        Ok(0u64)
    } else {
        let a3947 = f204(v0, v1, v2, v3, v5)?;
        let a3948 = f205(v0, v1, v2, v3, v4)?;
        nat_add(a3947, a3948)
    }
}

pub fn f169(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let a3949 = v0;
    let a3950 = 0u64;
    if if nat_eq(a3949, a3950) {
        true
    } else {
        let a3951 = v1;
        let a3952 = 0u64;
        nat_eq(a3951, a3952)
    } {
        Ok(0u64)
    } else {
        let a3955 = f91(v0, v1, v2)?;
        let a3956 = {
            let a3953 = v1;
            let a3954 = f206(v0)?;
            nat_mul(a3953, a3954)?
        };
        nat_add(a3955, a3956)
    }
}

pub fn f170(v0: u64, v1: u64, v2: u64) -> bool {
    let a3957 = v0;
    let a3958 = 1u64;
    if nat_le(a3957, a3958) {
        true
    } else {
        let a3962 = {
            let a3959 = v1;
            let a3960 = v2;
            let a3961 = 1u64;
            nat_rem(a3959, a3960, a3961)
        };
        let a3963 = 0u64;
        nat_eq(a3962, a3963)
    }
}

pub fn f171(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64) -> R<bool> {
    let a3964 = v1;
    let a3965 = 1u64;
    if if if nat_le(a3964, a3965) {
        true
    } else {
        let a3966 = v0;
        let a3967 = v2;
        nat_le(a3966, a3967)
    } {
        let a3968 = v3;
        let a3969 = 1u64;
        if nat_le(a3968, a3969) {
            true
        } else {
            let a3976 = {
                let a3974 = v0;
                let a3975 = {
                    let a3972 = {
                        let a3970 = v1;
                        let a3971 = 1u64;
                        nat_sub(a3970, a3971)
                    };
                    let a3973 = v2;
                    nat_mul(a3972, a3973)?
                };
                nat_add(a3974, a3975)?
            };
            let a3977 = v4;
            nat_le(a3976, a3977)
        }
    } else {
        false
    } {
        let a3978 = v5;
        let a3979 = 1u64;
        if nat_le(a3978, a3979) {
            Ok(true)
        } else {
            let a3992 = {
                let a3990 = {
                    let a3984 = v0;
                    let a3985 = {
                        let a3982 = {
                            let a3980 = v1;
                            let a3981 = 1u64;
                            nat_sub(a3980, a3981)
                        };
                        let a3983 = v2;
                        nat_mul(a3982, a3983)?
                    };
                    nat_add(a3984, a3985)?
                };
                let a3991 = {
                    let a3988 = {
                        let a3986 = v3;
                        let a3987 = 1u64;
                        nat_sub(a3986, a3987)
                    };
                    let a3989 = v4;
                    nat_mul(a3988, a3989)?
                };
                nat_add(a3990, a3991)?
            };
            let a3993 = v6;
            Ok(nat_le(a3992, a3993))
        }
    } else {
        Ok(false)
    }
}

pub fn f172(v0: u64, v1: u64, _: u64, v3: u64, v4: u64, v5: u64, v6: u64, v7: u64, v8: u64, v9: u64, v10: u64, v11: u64, v12: u64, v13: u64, v14: u64) -> bool {
    if if if if if if if if if f207(v0, v1, v6, v7, v8) {
        let a3994 = v5;
        let a3995 = 26u64;
        nat_eq(a3994, a3995)
    } else {
        false
    } {
        let a3996 = v3;
        let a3997 = v7;
        nat_eq(a3996, a3997)
    } else {
        false
    } {
        let a3998 = v4;
        let a3999 = v8;
        nat_eq(a3998, a3999)
    } else {
        false
    } {
        let a4000 = v9;
        let a4001 = 1u64;
        nat_eq(a4000, a4001)
    } else {
        false
    } {
        let a4002 = v10;
        let a4003 = f208(v0);
        nat_eq(a4002, a4003)
    } else {
        false
    } {
        let a4004 = v11;
        let a4005 = v1;
        nat_eq(a4004, a4005)
    } else {
        false
    } {
        let a4006 = v12;
        let a4007 = v6;
        nat_eq(a4006, a4007)
    } else {
        false
    } {
        let a4008 = v13;
        let a4009 = v7;
        nat_eq(a4008, a4009)
    } else {
        false
    } {
        let a4010 = v14;
        let a4011 = v8;
        nat_eq(a4010, a4011)
    } else {
        false
    }
}

pub fn f173(v0: u64, v1: u64, v2: u64, v3: u64) -> bool {
    let a4012 = v0;
    let a4013 = 0u64;
    if if if nat_eq(a4012, a4013) {
        true
    } else {
        let a4014 = v1;
        let a4015 = 0u64;
        nat_eq(a4014, a4015)
    } {
        true
    } else {
        let a4016 = v2;
        let a4017 = 0u64;
        nat_eq(a4016, a4017)
    } {
        true
    } else {
        let a4018 = v3;
        let a4019 = 0u64;
        nat_eq(a4018, a4019)
    }
}

pub fn f174(v0: Adt3, v1: bool) -> R<u64> {
    f209(v0.clone(), {
        let a4021 = match v0.clone() {
            Adt3::C0(h4020, _, _, _, _, _, _, _) => {
                h4020
            }
        };
        let a4022 = 1u64;
        nat_sub(a4021, a4022)
    }, {
        let a4024 = match v0.clone() {
            Adt3::C0(_, h4023, _, _, _, _, _, _) => {
                h4023
            }
        };
        let a4025 = 1u64;
        nat_sub(a4024, a4025)
    }, {
        let a4027 = match v0.clone() {
            Adt3::C0(_, _, h4026, _, _, _, _, _) => {
                h4026
            }
        };
        let a4028 = 1u64;
        nat_sub(a4027, a4028)
    }, {
        let a4030 = match v0 {
            Adt3::C0(_, _, _, h4029, _, _, _, _) => {
                h4029
            }
        };
        let a4031 = 1u64;
        nat_sub(a4030, a4031)
    }, v1)
}

pub fn f175(v0: u64, v1: u64, v2: u64, v3: u64) -> R<u64> {
    let a4032 = v0;
    let a4033 = 0u64;
    if nat_eq(a4032, a4033) {
        Ok(0u64)
    } else {
        let v4: u64 = {
            let a4034 = v0;
            let a4035 = 1u64;
            nat_sub(a4034, a4035)
        };
        let v5: u64 = {
            let a4036 = v4;
            let a4037 = v1;
            let a4038 = 0u64;
            nat_quot(a4036, a4037, a4038)
        };
        let v6: u64 = {
            let a4046 = {
                let a4039 = v5;
                let a4040 = v2;
                nat_mul(a4039, a4040)?
            };
            let a4047 = {
                let a4044 = {
                    let a4041 = v4;
                    let a4042 = v1;
                    let a4043 = 0u64;
                    nat_rem(a4041, a4042, a4043)
                };
                let a4045 = v3;
                nat_mul(a4044, a4045)?
            };
            nat_add(a4046, a4047)?
        };
        let a4048 = v5;
        let a4049 = 0u64;
        if nat_eq(a4048, a4049) {
            Ok(v6)
        } else {
            let v7: u64 = {
                let a4058 = {
                    let a4052 = {
                        let a4050 = v5;
                        let a4051 = 1u64;
                        nat_sub(a4050, a4051)
                    };
                    let a4053 = v2;
                    nat_mul(a4052, a4053)?
                };
                let a4059 = {
                    let a4056 = {
                        let a4054 = v1;
                        let a4055 = 1u64;
                        nat_sub(a4054, a4055)
                    };
                    let a4057 = v3;
                    nat_mul(a4056, a4057)?
                };
                nat_add(a4058, a4059)?
            };
            let a4060 = v6;
            let a4061 = v7;
            if nat_lt(a4060, a4061) {
                Ok(v7)
            } else {
                Ok(v6)
            }
        }
    }
}

pub fn f176(v0: Adt3, v1: u64) -> bool {
    let a4062 = v1;
    let a4063 = 0u64;
    if !nat_eq(a4062, a4063) {
        true
    } else {
        let a4065 = match v0 {
            Adt3::C0(_, _, _, _, h4064, _, _, _) => {
                h4064
            }
        };
        let a4066 = 4u64;
        nat_eq(a4065, a4066)
    }
}

pub fn f177(v0: u64, v1: bool) -> R<u64> {
    let a4069 = {
        let a4067 = v0;
        let a4068 = 2u64;
        nat_mul(a4067, a4068)?
    };
    let a4070 = if v1 {
        4u64
    } else {
        2u64
    };
    nat_add(a4069, a4070)
}

pub fn f178(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: bool, v6: u64, v7: u64, v8: u64, v9: u64, v10: u64, v11: u64, v12: u64, v13: u64, v14: u64, v15: u64, v16: u64, v17: u64) -> R<Adt10> {
    let m4071 = f210(v0, v1);
    match m4071 {
        None => {
            if v5 {
                let m4072 = f211(v0, v2, v3, v4, v6, v7, v8, v9, v10, v11, v12, v13, v14, v15, v16, v17)?;
                match m4072 {
                    Adt18::C0 => {
                        Ok(Adt10::C0)
                    }
                    Adt18::C1 => {
                        Ok(Adt10::C1)
                    }
                    Adt18::C2(v18, v19, v20, v21, v22, v23, v24, v25, v26, v27) => {
                        Ok(Adt10::C3(v18, v19, v20, v21, v22, v23, v24, v25, v26, v27))
                    }
                }
            } else {
                Ok(Adt10::C1)
            }
        }
        Some(v28) => {
            let v29: bool = {
                let a4073 = 4u64;
                let a4074 = v28;
                nat_le(a4073, a4074)
            };
            let a4075 = v2;
            let a4076 = 0u64;
            if if if if if nat_eq(a4075, a4076) {
                let a4077 = v4;
                let a4078 = 0u64;
                nat_eq(a4077, a4078)
            } else {
                false
            } {
                let a4079 = v6;
                let a4080 = v14;
                if nat_eq(a4079, a4080) {
                    let a4081 = v7;
                    let a4082 = v15;
                    nat_eq(a4081, a4082)
                } else {
                    false
                }
            } else {
                false
            } {
                let a4083 = v8;
                let a4084 = v16;
                if nat_eq(a4083, a4084) {
                    let a4085 = v9;
                    let a4086 = v17;
                    nat_eq(a4085, a4086)
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
                        let a4087 = v3;
                        let a4088 = 0u64;
                        nat_eq(a4087, a4088)
                    } else {
                        false
                    } {
                        f105(v6, v7, v8, v9, v10, v11, v12, v13)
                    } else {
                        false
                    }
                }
            } else {
                false
            } {
                Ok(Adt10::C2(v28, v29, v6, v7, v8, v9, {
                    let a4093 = {
                        let a4089 = v6;
                        let a4090 = v7;
                        nat_mul(a4089, a4090)?
                    };
                    let a4094 = {
                        let a4091 = v8;
                        let a4092 = v9;
                        nat_mul(a4091, a4092)?
                    };
                    nat_mul(a4093, a4094)?
                }))
            } else {
                Ok(Adt10::C1)
            }
        }
    }
}

pub fn f179(v0: u64) -> u64 {
    let m4095 = f104(v0);
    match m4095 {
        Adt16::C0 => {
            0u64
        }
        Adt16::C1(v1) => {
            let a4097 = match v1.clone() {
                Adt17::C0(_, h4096, _) => {
                    h4096
                }
            };
            let a4098 = 1u64;
            if if nat_eq(a4097, a4098) {
                let a4100 = match v1.clone() {
                    Adt17::C0(_, _, h4099) => {
                        h4099
                    }
                };
                let a4101 = 1u64;
                if if if nat_eq(a4100, a4101) {
                    true
                } else {
                    let a4103 = match v1.clone() {
                        Adt17::C0(_, _, h4102) => {
                            h4102
                        }
                    };
                    let a4104 = 2u64;
                    nat_eq(a4103, a4104)
                } {
                    true
                } else {
                    let a4106 = match v1.clone() {
                        Adt17::C0(_, _, h4105) => {
                            h4105
                        }
                    };
                    let a4107 = 4u64;
                    nat_eq(a4106, a4107)
                } {
                    true
                } else {
                    let a4109 = match v1.clone() {
                        Adt17::C0(_, _, h4108) => {
                            h4108
                        }
                    };
                    let a4110 = 8u64;
                    nat_eq(a4109, a4110)
                }
            } else {
                false
            } {
                match v1 {
                    Adt17::C0(_, _, h4111) => {
                        h4111
                    }
                }
            } else {
                0u64
            }
        }
    }
}

pub fn f180(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let a4116 = {
        let a4114 = {
            let a4112 = v0;
            let a4113 = v1;
            nat_sub(a4112, a4113)
        };
        let a4115 = 1u64;
        nat_sub(a4114, a4115)
    };
    let a4117 = v2;
    nat_add(a4116, a4117)
}

pub fn f181(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64, v7: u64, v8: u64, v9: u64, v10: u64, v11: u64) -> R<u64> {
    f118({
        let a4118 = v0;
        let a4119 = v4;
        let a4120 = 0u64;
        nat_rem(a4118, a4119, a4120)
    }, {
        let a4121 = v1;
        let a4122 = v5;
        let a4123 = 0u64;
        nat_rem(a4121, a4122, a4123)
    }, {
        let a4124 = v2;
        let a4125 = v6;
        let a4126 = 0u64;
        nat_rem(a4124, a4125, a4126)
    }, {
        let a4127 = v3;
        let a4128 = v7;
        let a4129 = 0u64;
        nat_rem(a4127, a4128, a4129)
    }, v8, v9, v10, v11)
}

pub fn f182(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let a4130 = v1;
    let a4131 = 2147483648u64;
    if nat_lt(a4130, a4131) {
        let a4142 = {
            let a4140 = {
                let a4135 = {
                    let a4132 = v0;
                    let a4133 = v2;
                    let a4134 = 0u64;
                    nat_rem(a4132, a4133, a4134)
                };
                let a4136 = v2;
                nat_add(a4135, a4136)?
            };
            let a4141 = {
                let a4137 = v1;
                let a4138 = v2;
                let a4139 = 0u64;
                nat_rem(a4137, a4138, a4139)
            };
            nat_sub(a4140, a4141)
        };
        let a4143 = v2;
        let a4144 = 0u64;
        Ok(nat_rem(a4142, a4143, a4144))
    } else {
        let a4147 = {
            let a4145 = v0;
            let a4146 = f119(v1);
            nat_add(a4145, a4146)?
        };
        let a4148 = v2;
        let a4149 = 0u64;
        Ok(nat_rem(a4147, a4148, a4149))
    }
}

pub fn f183(v0: u64, v1: u64, v2: u64) -> R<bool> {
    let a4152 = f120(v1);
    let a4153 = {
        let a4150 = v0;
        let a4151 = f119(v1);
        nat_add(a4150, a4151)?
    };
    if nat_le(a4152, a4153) {
        let a4154 = f184(v0, v1)?;
        let a4155 = v2;
        Ok(nat_lt(a4154, a4155))
    } else {
        Ok(false)
    }
}

pub fn f184(v0: u64, v1: u64) -> R<u64> {
    let a4158 = {
        let a4156 = v0;
        let a4157 = f119(v1);
        nat_add(a4156, a4157)?
    };
    let a4159 = f120(v1);
    Ok(nat_sub(a4158, a4159))
}

pub fn f185(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let a4160 = v0;
    let a4161 = v1;
    if nat_lt(a4160, a4161) {
        let a4162 = v1;
        let a4163 = v0;
        Ok(nat_sub(a4162, a4163))
    } else {
        let a4166 = v0;
        let a4167 = {
            let a4164 = v1;
            let a4165 = v2;
            nat_add(a4164, a4165)?
        };
        if nat_lt(a4166, a4167) {
            let a4168 = v0;
            let a4169 = v1;
            Ok(nat_sub(a4168, a4169))
        } else {
            let a4176 = {
                let a4172 = 2u64;
                let a4173 = {
                    let a4170 = v2;
                    let a4171 = 1u64;
                    nat_sub(a4170, a4171)
                };
                nat_mul(a4172, a4173)?
            };
            let a4177 = {
                let a4174 = v0;
                let a4175 = v1;
                nat_sub(a4174, a4175)
            };
            Ok(nat_sub(a4176, a4177))
        }
    }
}

pub fn f186(v0: u64, v1: Adt3, v2: u64) -> R<u64> {
    let a4180 = {
        let a4178 = v2;
        let a4179 = f209(v1.clone(), f64(v0, v1.clone(), 0u64), f64(v0, v1.clone(), 1u64), f64(v0, v1.clone(), 2u64), f64(v0, v1.clone(), 3u64), false)?;
        nat_add(a4178, a4179)?
    };
    let a4181 = f209(v1.clone(), f64(v0, v1.clone(), 0u64), f64(v0, v1.clone(), 1u64), f64(v0, v1.clone(), 2u64), f64(v0, v1, 3u64), true)?;
    Ok(nat_sub(a4180, a4181))
}

pub fn f187(v0: u64, v1: u64, v2: u64) -> bool {
    let a4182 = v0;
    let a4183 = 0u64;
    if nat_eq(a4182, a4183) {
        let a4184 = v2;
        let a4185 = v1;
        nat_le(a4184, a4185)
    } else {
        let a4186 = v0;
        let a4187 = 1u64;
        if nat_eq(a4186, a4187) {
            let a4188 = v2;
            let a4189 = v1;
            nat_lt(a4188, a4189)
        } else {
            let a4190 = v0;
            let a4191 = 2u64;
            if nat_eq(a4190, a4191) {
                let a4192 = v1;
                let a4193 = v2;
                nat_le(a4192, a4193)
            } else {
                let a4194 = v0;
                let a4195 = 3u64;
                if nat_eq(a4194, a4195) {
                    let a4196 = v1;
                    let a4197 = v2;
                    nat_lt(a4196, a4197)
                } else {
                    let a4198 = v1;
                    let a4199 = v2;
                    nat_eq(a4198, a4199)
                }
            }
        }
    }
}

pub fn f188(v0: u64, v1: Adt3, v2: u64) -> R<Adt9> {
    let a4200 = v0;
    let a4201 = 0u64;
    if nat_eq(a4200, a4201) {
        Ok(f196(0u64))
    } else {
        let a4204 = {
            let a4202 = v0;
            let a4203 = 1u64;
            nat_add(a4202, a4203)?
        };
        let a4205 = f116(v1.clone(), 13u64, v2)?;
        if nat_eq(a4204, a4205) {
            Ok(Adt9::C4({
                let a4206 = v2;
                let a4207 = 0u64;
                if nat_eq(a4206, a4207) {
                    2u64
                } else {
                    7u64
                }
            }, 1u64, 0u64, 2u64, 0u64, 3u64, 0u64))
        } else {
            let v3: u64 = {
                let a4208 = v0;
                let a4209 = 1u64;
                nat_sub(a4208, a4209)
            };
            if f176(v1.clone(), v2) {
                let v4: u64 = f177(match v1.clone() {
                    Adt3::C0(h4210, _, _, _, _, _, _, _) => {
                        h4210
                    }
                }, false)?;
                let v5: u64 = {
                    let a4211 = v3;
                    let a4212 = v4;
                    let a4213 = 0u64;
                    nat_quot(a4211, a4212, a4213)
                };
                let v6: u64 = {
                    let a4214 = v3;
                    let a4215 = v4;
                    let a4216 = 0u64;
                    nat_rem(a4214, a4215, a4216)
                };
                let a4219 = {
                    let a4217 = v6;
                    let a4218 = 1u64;
                    nat_add(a4217, a4218)?
                };
                let a4220 = v4;
                if nat_eq(a4219, a4220) {
                    Ok(Adt9::C4({
                        let a4221 = v2;
                        let a4222 = 0u64;
                        if nat_eq(a4221, a4222) {
                            0u64
                        } else {
                            4u64
                        }
                    }, 1u64, 0u64, 1u64, 1u64, 1u64, 0u64))
                } else {
                    f189(v6, v5, v1, v2)
                }
            } else {
                f212({
                    let a4223 = v3;
                    let a4224 = 2u64;
                    let a4225 = 0u64;
                    nat_quot(a4223, a4224, a4225)
                }, {
                    let a4226 = v3;
                    let a4227 = 2u64;
                    let a4228 = 0u64;
                    nat_rem(a4226, a4227, a4228)
                }, v1, v2, 0u64)
            }
        }
    }
}

pub fn f189(v0: u64, v1: u64, v2: Adt3, v3: u64) -> R<Adt9> {
    let a4229 = v0;
    let a4230 = 0u64;
    if nat_eq(a4229, a4230) {
        Ok(f196(1u64))
    } else {
        f212({
            let a4239 = {
                let a4232 = v1;
                let a4233 = match v2.clone() {
                    Adt3::C0(h4231, _, _, _, _, _, _, _) => {
                        h4231
                    }
                };
                nat_mul(a4232, a4233)?
            };
            let a4240 = {
                let a4236 = {
                    let a4234 = v0;
                    let a4235 = 1u64;
                    nat_sub(a4234, a4235)
                };
                let a4237 = 2u64;
                let a4238 = 0u64;
                nat_quot(a4236, a4237, a4238)
            };
            nat_add(a4239, a4240)?
        }, {
            let a4243 = {
                let a4241 = v0;
                let a4242 = 1u64;
                nat_sub(a4241, a4242)
            };
            let a4244 = 2u64;
            let a4245 = 0u64;
            nat_rem(a4243, a4244, a4245)
        }, v2, v3, 1u64)
    }
}

pub fn f190(v0: u64, v1: u64) -> R<u64> {
    let a4246 = v0;
    let a4247 = f85(v1)?;
    let a4248 = 0u64;
    Ok(nat_rem(a4246, a4247, a4248))
}

pub fn f191(v0: u64, v1: u64) -> R<u64> {
    let a4249 = 4u64;
    let a4250 = f213(v0, v1)?;
    nat_mul(a4249, a4250)
}

pub fn f192(v0: u64, v1: u64) -> R<u64> {
    let a4251 = 4u64;
    let a4252 = f214(v0, v1)?;
    nat_mul(a4251, a4252)
}

pub fn f193(v0: u64, v1: u64) -> R<bool> {
    let a4258 = {
        let a4255 = {
            let a4253 = f190(v0, v1)?;
            let a4254 = 2u64;
            nat_sub(a4253, a4254)
        };
        let a4256 = 2u64;
        let a4257 = 0u64;
        nat_rem(a4255, a4256, a4257)
    };
    let a4259 = 0u64;
    Ok(nat_eq(a4258, a4259))
}

pub fn f194(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64, v7: u64, v8: u64, v9: u64, v10: u64, v11: u64, v12: u64) -> R<Option<u64>> {
    let m4260 = f104(v0);
    match m4260 {
        Adt16::C0 => {
            Ok(None::<u64>)
        }
        Adt16::C1(v13) => {
            if if if if if f215(v0) {
                let a4261 = v1;
                let a4262 = v7;
                nat_lt(a4261, a4262)
            } else {
                false
            } {
                f143(v8, v2)
            } else {
                false
            } {
                let a4263 = v3;
                let a4264 = f216(v4, v5, v6)?;
                nat_lt(a4263, a4264)
            } else {
                false
            } {
                let a4269 = {
                    let a4266 = v7;
                    let a4267 = match v13.clone() {
                        Adt17::C0(_, h4265, _) => {
                            h4265
                        }
                    };
                    let a4268 = 0u64;
                    nat_rem(a4266, a4267, a4268)
                };
                let a4270 = 0u64;
                nat_eq(a4269, a4270)
            } else {
                false
            } {
                Ok(Some({
                    let a4299 = {
                        let a4289 = {
                            let a4279 = {
                                let a4275 = {
                                    let a4272 = v1;
                                    let a4273 = match v13 {
                                        Adt17::C0(_, h4271, _) => {
                                            h4271
                                        }
                                    };
                                    let a4274 = 0u64;
                                    nat_quot(a4272, a4273, a4274)
                                };
                                let a4276 = v9;
                                nat_mul(a4275, a4276)?
                            };
                            let a4280 = {
                                let a4277 = v2;
                                let a4278 = v10;
                                nat_mul(a4277, a4278)?
                            };
                            nat_add(a4279, a4280)?
                        };
                        let a4290 = {
                            let a4287 = {
                                let a4284 = {
                                    let a4281 = v3;
                                    let a4282 = v4;
                                    let a4283 = 0u64;
                                    nat_quot(a4281, a4282, a4283)
                                };
                                let a4285 = v5;
                                let a4286 = 0u64;
                                nat_rem(a4284, a4285, a4286)
                            };
                            let a4288 = v11;
                            nat_mul(a4287, a4288)?
                        };
                        nat_add(a4289, a4290)?
                    };
                    let a4300 = {
                        let a4297 = {
                            let a4294 = {
                                let a4291 = v3;
                                let a4292 = v4;
                                let a4293 = 0u64;
                                nat_quot(a4291, a4292, a4293)
                            };
                            let a4295 = v5;
                            let a4296 = 0u64;
                            nat_quot(a4294, a4295, a4296)
                        };
                        let a4298 = v12;
                        nat_mul(a4297, a4298)?
                    };
                    nat_add(a4299, a4300)?
                }))
            } else {
                Ok(None::<u64>)
            }
        }
    }
}

pub fn f195(v0: u64) -> u64 {
    let a4301 = v0;
    let a4302 = 30u64;
    if nat_eq(a4301, a4302) {
        4u64
    } else {
        3u64
    }
}

pub fn f196(v0: u64) -> Adt9 {
    Adt9::C4(0u64, 2u64, 0u64, 2u64, 0u64, 1u64, v0)
}

pub fn f197(v0: u64) -> bool {
    let a4303 = 0u64;
    let a4304 = v0;
    if nat_lt(a4303, a4304) {
        let a4305 = v0;
        let a4306 = 2139095040u64;
        nat_le(a4305, a4306)
    } else {
        false
    }
}

pub fn f198(v0: u64) -> u64 {
    let a4307 = v0;
    let a4308 = 40u64;
    if nat_lt(a4307, a4308) {
        let a4309 = v0;
        let a4310 = 20u64;
        if nat_lt(a4309, a4310) {
            let a4311 = v0;
            let a4312 = 10u64;
            if nat_lt(a4311, a4312) {
                let a4313 = v0;
                let a4314 = 5u64;
                if nat_lt(a4313, a4314) {
                    let a4315 = v0;
                    let a4316 = 2u64;
                    if nat_lt(a4315, a4316) {
                        let a4317 = v0;
                        let a4318 = 1u64;
                        if nat_lt(a4317, a4318) {
                            1u64
                        } else {
                            2u64
                        }
                    } else {
                        let a4319 = v0;
                        let a4320 = 3u64;
                        if nat_lt(a4319, a4320) {
                            4u64
                        } else {
                            let a4321 = v0;
                            let a4322 = 4u64;
                            if nat_lt(a4321, a4322) {
                                8u64
                            } else {
                                16u64
                            }
                        }
                    }
                } else {
                    let a4323 = v0;
                    let a4324 = 7u64;
                    if nat_lt(a4323, a4324) {
                        let a4325 = v0;
                        let a4326 = 6u64;
                        if nat_lt(a4325, a4326) {
                            32u64
                        } else {
                            64u64
                        }
                    } else {
                        let a4327 = v0;
                        let a4328 = 8u64;
                        if nat_lt(a4327, a4328) {
                            128u64
                        } else {
                            let a4329 = v0;
                            let a4330 = 9u64;
                            if nat_lt(a4329, a4330) {
                                256u64
                            } else {
                                512u64
                            }
                        }
                    }
                }
            } else {
                let a4331 = v0;
                let a4332 = 15u64;
                if nat_lt(a4331, a4332) {
                    let a4333 = v0;
                    let a4334 = 12u64;
                    if nat_lt(a4333, a4334) {
                        let a4335 = v0;
                        let a4336 = 11u64;
                        if nat_lt(a4335, a4336) {
                            1024u64
                        } else {
                            2048u64
                        }
                    } else {
                        let a4337 = v0;
                        let a4338 = 13u64;
                        if nat_lt(a4337, a4338) {
                            4096u64
                        } else {
                            let a4339 = v0;
                            let a4340 = 14u64;
                            if nat_lt(a4339, a4340) {
                                8192u64
                            } else {
                                16384u64
                            }
                        }
                    }
                } else {
                    let a4341 = v0;
                    let a4342 = 17u64;
                    if nat_lt(a4341, a4342) {
                        let a4343 = v0;
                        let a4344 = 16u64;
                        if nat_lt(a4343, a4344) {
                            32768u64
                        } else {
                            65536u64
                        }
                    } else {
                        let a4345 = v0;
                        let a4346 = 18u64;
                        if nat_lt(a4345, a4346) {
                            131072u64
                        } else {
                            let a4347 = v0;
                            let a4348 = 19u64;
                            if nat_lt(a4347, a4348) {
                                262144u64
                            } else {
                                524288u64
                            }
                        }
                    }
                }
            }
        } else {
            let a4349 = v0;
            let a4350 = 30u64;
            if nat_lt(a4349, a4350) {
                let a4351 = v0;
                let a4352 = 25u64;
                if nat_lt(a4351, a4352) {
                    let a4353 = v0;
                    let a4354 = 22u64;
                    if nat_lt(a4353, a4354) {
                        let a4355 = v0;
                        let a4356 = 21u64;
                        if nat_lt(a4355, a4356) {
                            1048576u64
                        } else {
                            2097152u64
                        }
                    } else {
                        let a4357 = v0;
                        let a4358 = 23u64;
                        if nat_lt(a4357, a4358) {
                            4194304u64
                        } else {
                            let a4359 = v0;
                            let a4360 = 24u64;
                            if nat_lt(a4359, a4360) {
                                8388608u64
                            } else {
                                16777216u64
                            }
                        }
                    }
                } else {
                    let a4361 = v0;
                    let a4362 = 27u64;
                    if nat_lt(a4361, a4362) {
                        let a4363 = v0;
                        let a4364 = 26u64;
                        if nat_lt(a4363, a4364) {
                            33554432u64
                        } else {
                            67108864u64
                        }
                    } else {
                        let a4365 = v0;
                        let a4366 = 28u64;
                        if nat_lt(a4365, a4366) {
                            134217728u64
                        } else {
                            let a4367 = v0;
                            let a4368 = 29u64;
                            if nat_lt(a4367, a4368) {
                                268435456u64
                            } else {
                                536870912u64
                            }
                        }
                    }
                }
            } else {
                let a4369 = v0;
                let a4370 = 35u64;
                if nat_lt(a4369, a4370) {
                    let a4371 = v0;
                    let a4372 = 32u64;
                    if nat_lt(a4371, a4372) {
                        let a4373 = v0;
                        let a4374 = 31u64;
                        if nat_lt(a4373, a4374) {
                            1073741824u64
                        } else {
                            2147483648u64
                        }
                    } else {
                        let a4375 = v0;
                        let a4376 = 33u64;
                        if nat_lt(a4375, a4376) {
                            4294967296u64
                        } else {
                            let a4377 = v0;
                            let a4378 = 34u64;
                            if nat_lt(a4377, a4378) {
                                8589934592u64
                            } else {
                                17179869184u64
                            }
                        }
                    }
                } else {
                    let a4379 = v0;
                    let a4380 = 37u64;
                    if nat_lt(a4379, a4380) {
                        let a4381 = v0;
                        let a4382 = 36u64;
                        if nat_lt(a4381, a4382) {
                            34359738368u64
                        } else {
                            68719476736u64
                        }
                    } else {
                        let a4383 = v0;
                        let a4384 = 38u64;
                        if nat_lt(a4383, a4384) {
                            137438953472u64
                        } else {
                            let a4385 = v0;
                            let a4386 = 39u64;
                            if nat_lt(a4385, a4386) {
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

pub fn f199(v0: u64, v1: u64) -> bool {
    let a4387 = v0;
    let a4388 = 1u64;
    if nat_le(a4387, a4388) {
        true
    } else {
        let a4392 = {
            let a4389 = v1;
            let a4390 = 4u64;
            let a4391 = 0u64;
            nat_rem(a4389, a4390, a4391)
        };
        let a4393 = 0u64;
        nat_eq(a4392, a4393)
    }
}

pub fn f200(v0: u64, v1: u64) -> bool {
    let a4394 = v0;
    let a4395 = 1u64;
    if nat_le(a4394, a4395) {
        true
    } else {
        let a4396 = 4u64;
        let a4397 = v1;
        nat_le(a4396, a4397)
    }
}

pub fn f201(v0: u64, v1: u64) -> R<u64> {
    let a4398 = v0;
    let a4399 = 0u64;
    if nat_eq(a4398, a4399) {
        Ok(0u64)
    } else {
        let a4404 = 4u64;
        let a4405 = {
            let a4402 = {
                let a4400 = v0;
                let a4401 = 1u64;
                nat_sub(a4400, a4401)
            };
            let a4403 = v1;
            nat_mul(a4402, a4403)?
        };
        nat_add(a4404, a4405)
    }
}

pub fn f202(v0: bool, v1: u64) -> R<u64> {
    let a4410 = {
        let a4408 = {
            let a4406 = f217(v0, v1)?;
            let a4407 = f218(v0, v1)?;
            nat_add(a4406, a4407)?
        };
        let a4409 = f219(v0);
        nat_add(a4408, a4409)?
    };
    let a4411 = v1;
    nat_add(a4410, a4411)
}

pub fn f203(v0: u64, v1: u64) -> R<u64> {
    let v2: u64 = f220(v0, v1)?;
    let v3: u64 = {
        let a4416 = {
            let a4414 = {
                let a4412 = v1;
                let a4413 = 1u64;
                nat_sub(a4412, a4413)
            };
            let a4415 = v2;
            nat_mul(a4414, a4415)?
        };
        let a4417 = v0;
        nat_sub(a4416, a4417)
    };
    let a4418 = v2;
    let a4419 = v3;
    if nat_lt(a4418, a4419) {
        Ok(v3)
    } else {
        Ok(v2)
    }
}

pub fn f204(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64) -> R<u64> {
    let a4420 = v4;
    let a4421 = 0u64;
    if nat_eq(a4420, a4421) {
        Ok(0u64)
    } else {
        let a4428 = 2u64;
        let a4429 = {
            let a4426 = {
                let a4422 = v0;
                let a4423 = v1;
                nat_mul(a4422, a4423)?
            };
            let a4427 = {
                let a4424 = v2;
                let a4425 = v3;
                nat_mul(a4424, a4425)?
            };
            nat_mul(a4426, a4427)?
        };
        nat_mul(a4428, a4429)
    }
}

pub fn f205(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64) -> R<u64> {
    let a4434 = v3;
    let a4435 = {
        let a4432 = {
            let a4430 = f221(v2, v4)?;
            let a4431 = f222(v0, v1, f220(v2, v4)?)?;
            nat_mul(a4430, a4431)?
        };
        let a4433 = f222(v0, v1, f223(v2, v4)?)?;
        nat_add(a4432, a4433)?
    };
    nat_mul(a4434, a4435)
}

pub fn f206(v0: u64) -> R<u64> {
    let a4438 = 13u64;
    let a4439 = {
        let a4436 = 9u64;
        let a4437 = v0;
        nat_mul(a4436, a4437)?
    };
    nat_add(a4438, a4439)
}

pub fn f207(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64) -> bool {
    if f173(v1, v2, v3, v4) {
        f224(v0)
    } else {
        f215(v0)
    }
}

pub fn f208(v0: u64) -> u64 {
    let a4440 = v0;
    let a4441 = 26u64;
    if nat_eq(a4440, a4441) {
        26u64
    } else {
        0u64
    }
}

pub fn f209(v0: Adt3, v1: u64, v2: u64, v3: u64, v4: u64, v5: bool) -> R<u64> {
    let a4462 = {
        let a4456 = {
            let a4450 = {
                let a4444 = v1;
                let a4445 = if v5 {
                    f119(match v0.clone() {
                        Adt3::C0(_, _, _, _, h4442, _, _, _) => {
                            h4442
                        }
                    })
                } else {
                    f120(match v0.clone() {
                        Adt3::C0(_, _, _, _, h4443, _, _, _) => {
                            h4443
                        }
                    })
                };
                nat_mul(a4444, a4445)?
            };
            let a4451 = {
                let a4448 = v2;
                let a4449 = if v5 {
                    f119(match v0.clone() {
                        Adt3::C0(_, _, _, _, _, h4446, _, _) => {
                            h4446
                        }
                    })
                } else {
                    f120(match v0.clone() {
                        Adt3::C0(_, _, _, _, _, h4447, _, _) => {
                            h4447
                        }
                    })
                };
                nat_mul(a4448, a4449)?
            };
            nat_add(a4450, a4451)?
        };
        let a4457 = {
            let a4454 = v3;
            let a4455 = if v5 {
                f119(match v0.clone() {
                    Adt3::C0(_, _, _, _, _, _, h4452, _) => {
                        h4452
                    }
                })
            } else {
                f120(match v0.clone() {
                    Adt3::C0(_, _, _, _, _, _, h4453, _) => {
                        h4453
                    }
                })
            };
            nat_mul(a4454, a4455)?
        };
        nat_add(a4456, a4457)?
    };
    let a4463 = {
        let a4460 = v4;
        let a4461 = if v5 {
            f119(match v0 {
                Adt3::C0(_, _, _, _, _, _, _, h4458) => {
                    h4458
                }
            })
        } else {
            f120(match v0 {
                Adt3::C0(_, _, _, _, _, _, _, h4459) => {
                    h4459
                }
            })
        };
        nat_mul(a4460, a4461)?
    };
    nat_add(a4462, a4463)
}

pub fn f210(v0: u64, v1: u64) -> Option<u64> {
    let a4464 = v0;
    let a4465 = 2u64;
    if nat_eq(a4464, a4465) {
        Some(0u64)
    } else {
        let a4466 = v0;
        let a4467 = 5u64;
        if nat_eq(a4466, a4467) {
            Some(1u64)
        } else {
            let a4468 = v0;
            let a4469 = 6u64;
            if nat_eq(a4468, a4469) {
                Some(2u64)
            } else {
                let a4470 = v0;
                let a4471 = 7u64;
                if nat_eq(a4470, a4471) {
                    Some(3u64)
                } else {
                    let a4472 = v0;
                    let a4473 = 9u64;
                    if nat_eq(a4472, a4473) {
                        Some(4u64)
                    } else {
                        let a4474 = v0;
                        let a4475 = 8u64;
                        if nat_eq(a4474, a4475) {
                            Some(5u64)
                        } else {
                            let a4476 = v0;
                            let a4477 = 89u64;
                            if nat_eq(a4476, a4477) {
                                let a4478 = v1;
                                let a4479 = 2u64;
                                if nat_eq(a4478, a4479) {
                                    Some(6u64)
                                } else {
                                    let a4480 = v1;
                                    let a4481 = 0u64;
                                    if nat_eq(a4480, a4481) {
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

pub fn f211(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64, v7: u64, v8: u64, v9: u64, v10: u64, v11: u64, v12: u64, v13: u64, v14: u64, v15: u64) -> R<Adt18> {
    let a4482 = v0;
    let a4483 = 30u64;
    if nat_eq(a4482, a4483) {
        let a4484 = v2;
        let a4485 = 0u64;
        if if if nat_eq(a4484, a4485) {
            true
        } else {
            let a4486 = v2;
            let a4487 = 1u64;
            nat_eq(a4486, a4487)
        } {
            let a4488 = v1;
            let a4489 = 0u64;
            if nat_eq(a4488, a4489) {
                true
            } else {
                f225(v1, v4)
            }
        } else {
            false
        } {
            f226(28u64, 0u64, 0u64, v3, v5, v4, v6, v7, v9, v8, v10, v11, v12, v13, v14, v15)
        } else {
            Ok(Adt18::C0)
        }
    } else {
        if if if if f227(v1) {
            f227(v2)
        } else {
            false
        } {
            let a4490 = v1;
            let a4491 = 0u64;
            if if nat_eq(a4490, a4491) {
                true
            } else {
                let a4492 = v2;
                let a4493 = 0u64;
                nat_eq(a4492, a4493)
            } {
                true
            } else {
                let a4494 = v1;
                let a4495 = v2;
                nat_eq(a4494, a4495)
            }
        } else {
            false
        } {
            true
        } else {
            let a4496 = v1;
            let a4497 = 43u64;
            if nat_eq(a4496, a4497) {
                f227(v2)
            } else {
                false
            }
        } {
            f226(v0, 0u64, 0u64, v3, v4, v5, v6, v7, v8, v9, v10, v11, v12, v13, v14, v15)
        } else {
            Ok(Adt18::C0)
        }
    }
}

pub fn f212(v0: u64, v1: u64, v2: Adt3, v3: u64, v4: u64) -> R<Adt9> {
    let a4498 = v1;
    let a4499 = 0u64;
    if nat_eq(a4498, a4499) {
        Ok(Adt9::C4({
            let a4500 = v3;
            let a4501 = 0u64;
            if nat_eq(a4500, a4501) {
                1u64
            } else {
                6u64
            }
        }, 0u64, f126(v0, v2)?, 2u64, 0u64, 1u64, 2u64))
    } else {
        Ok(Adt9::C4({
            let a4502 = v3;
            let a4503 = 0u64;
            if nat_eq(a4502, a4503) {
                0u64
            } else {
                4u64
            }
        }, 1u64, v4, 1u64, 2u64, 1u64, v4))
    }
}

pub fn f213(v0: u64, v1: u64) -> R<u64> {
    let a4504 = v0;
    let a4505 = f85(v1)?;
    let a4506 = 0u64;
    Ok(nat_quot(a4504, a4505, a4506))
}

pub fn f214(v0: u64, v1: u64) -> R<u64> {
    let v2: u64 = f213(v0, v1)?;
    let v3: u64 = {
        let a4509 = {
            let a4507 = f190(v0, v1)?;
            let a4508 = 2u64;
            nat_sub(a4507, a4508)
        };
        let a4510 = 2u64;
        let a4511 = 0u64;
        nat_quot(a4509, a4510, a4511)
    };
    let v4: u64 = {
        let a4512 = v2;
        let a4513 = v1;
        nat_mul(a4512, a4513)?
    };
    if f193(v0, v1)? {
        let a4514 = v4;
        let a4515 = v3;
        nat_add(a4514, a4515)
    } else {
        let a4527 = {
            let a4523 = v4;
            let a4524 = {
                let a4521 = {
                    let a4516 = v1;
                    let a4517 = 1u64;
                    nat_sub(a4516, a4517)
                };
                let a4522 = {
                    let a4518 = v2;
                    let a4519 = v1;
                    let a4520 = 0u64;
                    nat_rem(a4518, a4519, a4520)
                };
                nat_mul(a4521, a4522)?
            };
            nat_sub(a4523, a4524)
        };
        let a4528 = {
            let a4525 = v3;
            let a4526 = v1;
            nat_mul(a4525, a4526)?
        };
        nat_add(a4527, a4528)
    }
}

pub fn f215(v0: u64) -> bool {
    let a4529 = v0;
    let a4530 = 0u64;
    if if if if if if if if if if if if if if if if if if if if if if if if if if if if if nat_eq(a4529, a4530) {
        true
    } else {
        let a4531 = v0;
        let a4532 = 1u64;
        nat_eq(a4531, a4532)
    } {
        true
    } else {
        let a4533 = v0;
        let a4534 = 2u64;
        nat_eq(a4533, a4534)
    } {
        true
    } else {
        let a4535 = v0;
        let a4536 = 3u64;
        nat_eq(a4535, a4536)
    } {
        true
    } else {
        let a4537 = v0;
        let a4538 = 6u64;
        nat_eq(a4537, a4538)
    } {
        true
    } else {
        let a4539 = v0;
        let a4540 = 7u64;
        nat_eq(a4539, a4540)
    } {
        true
    } else {
        let a4541 = v0;
        let a4542 = 8u64;
        nat_eq(a4541, a4542)
    } {
        true
    } else {
        let a4543 = v0;
        let a4544 = 10u64;
        nat_eq(a4543, a4544)
    } {
        true
    } else {
        let a4545 = v0;
        let a4546 = 11u64;
        nat_eq(a4545, a4546)
    } {
        true
    } else {
        let a4547 = v0;
        let a4548 = 12u64;
        nat_eq(a4547, a4548)
    } {
        true
    } else {
        let a4549 = v0;
        let a4550 = 13u64;
        nat_eq(a4549, a4550)
    } {
        true
    } else {
        let a4551 = v0;
        let a4552 = 14u64;
        nat_eq(a4551, a4552)
    } {
        true
    } else {
        let a4553 = v0;
        let a4554 = 16u64;
        nat_eq(a4553, a4554)
    } {
        true
    } else {
        let a4555 = v0;
        let a4556 = 17u64;
        nat_eq(a4555, a4556)
    } {
        true
    } else {
        let a4557 = v0;
        let a4558 = 18u64;
        nat_eq(a4557, a4558)
    } {
        true
    } else {
        let a4559 = v0;
        let a4560 = 19u64;
        nat_eq(a4559, a4560)
    } {
        true
    } else {
        let a4561 = v0;
        let a4562 = 20u64;
        nat_eq(a4561, a4562)
    } {
        true
    } else {
        let a4563 = v0;
        let a4564 = 21u64;
        nat_eq(a4563, a4564)
    } {
        true
    } else {
        let a4565 = v0;
        let a4566 = 22u64;
        nat_eq(a4565, a4566)
    } {
        true
    } else {
        let a4567 = v0;
        let a4568 = 23u64;
        nat_eq(a4567, a4568)
    } {
        true
    } else {
        let a4569 = v0;
        let a4570 = 26u64;
        nat_eq(a4569, a4570)
    } {
        true
    } else {
        let a4571 = v0;
        let a4572 = 29u64;
        nat_eq(a4571, a4572)
    } {
        true
    } else {
        let a4573 = v0;
        let a4574 = 30u64;
        nat_eq(a4573, a4574)
    } {
        true
    } else {
        let a4575 = v0;
        let a4576 = 34u64;
        nat_eq(a4575, a4576)
    } {
        true
    } else {
        let a4577 = v0;
        let a4578 = 35u64;
        nat_eq(a4577, a4578)
    } {
        true
    } else {
        let a4579 = v0;
        let a4580 = 39u64;
        nat_eq(a4579, a4580)
    } {
        true
    } else {
        let a4581 = v0;
        let a4582 = 40u64;
        nat_eq(a4581, a4582)
    } {
        true
    } else {
        let a4583 = v0;
        let a4584 = 41u64;
        nat_eq(a4583, a4584)
    } {
        true
    } else {
        let a4585 = v0;
        let a4586 = 42u64;
        nat_eq(a4585, a4586)
    } {
        true
    } else {
        let a4587 = v0;
        let a4588 = 43u64;
        nat_eq(a4587, a4588)
    }
}

pub fn f216(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let a4589 = v0;
    let a4590 = 0u64;
    if if if nat_eq(a4589, a4590) {
        true
    } else {
        let a4591 = v1;
        let a4592 = 0u64;
        nat_eq(a4591, a4592)
    } {
        true
    } else {
        let a4593 = v2;
        let a4594 = 0u64;
        nat_eq(a4593, a4594)
    } {
        Ok(0u64)
    } else {
        let a4597 = {
            let a4595 = v0;
            let a4596 = v1;
            nat_mul(a4595, a4596)?
        };
        let a4598 = v2;
        nat_mul(a4597, a4598)
    }
}

pub fn f217(v0: bool, v1: u64) -> R<u64> {
    let a4601 = if v0 {
        5u64
    } else {
        4u64
    };
    let a4602 = {
        let a4599 = f228(v0);
        let a4600 = v1;
        nat_mul(a4599, a4600)?
    };
    nat_add(a4601, a4602)
}

pub fn f218(v0: bool, v1: u64) -> R<u64> {
    if v0 {
        let a4610 = {
            let a4603 = 13u64;
            let a4604 = f229(v1);
            nat_mul(a4603, a4604)?
        };
        let a4611 = {
            let a4608 = 4u64;
            let a4609 = {
                let a4605 = v1;
                let a4606 = 4u64;
                let a4607 = 0u64;
                nat_rem(a4605, a4606, a4607)
            };
            nat_mul(a4608, a4609)?
        };
        nat_add(a4610, a4611)
    } else {
        let a4612 = 3u64;
        let a4613 = v1;
        nat_mul(a4612, a4613)
    }
}

pub fn f219(v0: bool) -> u64 {
    if v0 {
        6u64
    } else {
        5u64
    }
}

pub fn f220(v0: u64, v1: u64) -> R<u64> {
    let a4618 = {
        let a4616 = {
            let a4614 = v0;
            let a4615 = v1;
            nat_add(a4614, a4615)?
        };
        let a4617 = 1u64;
        nat_sub(a4616, a4617)
    };
    let a4619 = v1;
    let a4620 = 0u64;
    Ok(nat_quot(a4618, a4619, a4620))
}

pub fn f221(v0: u64, v1: u64) -> R<u64> {
    let a4621 = v0;
    let a4622 = f220(v0, v1)?;
    let a4623 = 0u64;
    Ok(nat_quot(a4621, a4622, a4623))
}

pub fn f222(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let a4624 = v2;
    let a4625 = 0u64;
    if nat_eq(a4624, a4625) {
        Ok(0u64)
    } else {
        let a4630 = 11u64;
        let a4631 = {
            let a4628 = {
                let a4626 = v1;
                let a4627 = v2;
                nat_mul(a4626, a4627)?
            };
            let a4629 = f230(v0)?;
            nat_mul(a4628, a4629)?
        };
        nat_add(a4630, a4631)
    }
}

pub fn f223(v0: u64, v1: u64) -> R<u64> {
    let a4632 = v0;
    let a4633 = f220(v0, v1)?;
    let a4634 = 0u64;
    Ok(nat_rem(a4632, a4633, a4634))
}

pub fn f224(v0: u64) -> bool {
    let m4635 = f104(v0);
    match m4635 {
        Adt16::C0 => {
            false
        }
        Adt16::C1(_) => {
            true
        }
    }
}

pub fn f225(v0: u64, v1: u64) -> bool {
    let a4636 = v0;
    let a4637 = 1u64;
    if nat_eq(a4636, a4637) {
        true
    } else {
        let v2: u64 = f231(v0);
        let a4638 = 0u64;
        let a4639 = v2;
        if nat_lt(a4638, a4639) {
            let a4643 = {
                let a4640 = v1;
                let a4641 = v2;
                let a4642 = 1u64;
                nat_rem(a4640, a4641, a4642)
            };
            let a4644 = 0u64;
            nat_eq(a4643, a4644)
        } else {
            false
        }
    }
}

pub fn f226(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64, v7: u64, v8: u64, v9: u64, v10: u64, v11: u64, v12: u64, v13: u64, v14: u64, v15: u64) -> R<Adt18> {
    let a4645 = v0;
    let a4646 = 28u64;
    if if if if nat_eq(a4645, a4646) {
        let a4647 = v1;
        let a4648 = 0u64;
        nat_eq(a4647, a4648)
    } else {
        false
    } {
        let a4649 = v2;
        let a4650 = 0u64;
        nat_eq(a4649, a4650)
    } else {
        false
    } {
        let a4651 = v3;
        let a4652 = 0u64;
        nat_eq(a4651, a4652)
    } else {
        false
    } {
        let a4653 = v4;
        let a4654 = v8;
        if if if if if if if if nat_eq(a4653, a4654) {
            let a4655 = v5;
            let a4656 = v12;
            nat_eq(a4655, a4656)
        } else {
            false
        } {
            let a4657 = v9;
            let a4658 = v13;
            nat_eq(a4657, a4658)
        } else {
            false
        } {
            let a4659 = v10;
            let a4660 = v14;
            nat_eq(a4659, a4660)
        } else {
            false
        } {
            let a4661 = v11;
            let a4662 = v15;
            nat_eq(a4661, a4662)
        } else {
            false
        } {
            let a4663 = 0u64;
            let a4664 = v6;
            nat_lt(a4663, a4664)
        } else {
            false
        } {
            let a4665 = 0u64;
            let a4666 = v7;
            nat_lt(a4665, a4666)
        } else {
            false
        } {
            let a4670 = {
                let a4667 = v10;
                let a4668 = v6;
                let a4669 = 1u64;
                nat_rem(a4667, a4668, a4669)
            };
            let a4671 = 0u64;
            if nat_eq(a4670, a4671) {
                let a4675 = {
                    let a4672 = v11;
                    let a4673 = v7;
                    let a4674 = 1u64;
                    nat_rem(a4672, a4673, a4674)
                };
                let a4676 = 0u64;
                nat_eq(a4675, a4676)
            } else {
                false
            }
        } else {
            false
        } {
            Ok(Adt18::C2(v5, v4, v9, v10, v11, {
                let a4677 = v10;
                let a4678 = v6;
                let a4679 = 0u64;
                nat_quot(a4677, a4678, a4679)
            }, {
                let a4680 = v11;
                let a4681 = v7;
                let a4682 = 0u64;
                nat_quot(a4680, a4681, a4682)
            }, {
                let a4683 = v5;
                let a4684 = v4;
                nat_mul(a4683, a4684)?
            }, {
                let a4685 = v4;
                let a4686 = v9;
                nat_mul(a4685, a4686)?
            }, {
                let a4687 = v5;
                let a4688 = v9;
                nat_mul(a4687, a4688)?
            }))
        } else {
            Ok(Adt18::C1)
        }
    } else {
        Ok(Adt18::C0)
    }
}

pub fn f227(v0: u64) -> bool {
    let a4689 = v0;
    let a4690 = 0u64;
    if if nat_eq(a4689, a4690) {
        true
    } else {
        let a4691 = v0;
        let a4692 = 1u64;
        nat_eq(a4691, a4692)
    } {
        true
    } else {
        let a4693 = v0;
        let a4694 = 30u64;
        nat_eq(a4693, a4694)
    }
}

pub fn f228(v0: bool) -> u64 {
    if v0 {
        2u64
    } else {
        1u64
    }
}

pub fn f229(v0: u64) -> u64 {
    let a4695 = v0;
    let a4696 = 4u64;
    let a4697 = 0u64;
    nat_quot(a4695, a4696, a4697)
}

pub fn f230(v0: u64) -> R<u64> {
    let a4700 = {
        let a4698 = 7u64;
        let a4699 = v0;
        nat_mul(a4698, a4699)?
    };
    let a4701 = 4u64;
    nat_add(a4700, a4701)
}

pub fn f231(v0: u64) -> u64 {
    if f215(v0) {
        let m4702 = f104(v0);
        match m4702 {
            Adt16::C0 => {
                0u64
            }
            Adt16::C1(v1) => {
                let a4704 = 1u64;
                let a4705 = match v1.clone() {
                    Adt17::C0(_, h4703, _) => {
                        h4703
                    }
                };
                if if nat_lt(a4704, a4705) {
                    true
                } else {
                    let a4706 = v0;
                    let a4707 = 43u64;
                    nat_eq(a4706, a4707)
                } {
                    match v1 {
                        Adt17::C0(_, h4708, _) => {
                            h4708
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

pub fn run(p0: Adt0) -> R<Adt9> {
    f0(p0)
}
