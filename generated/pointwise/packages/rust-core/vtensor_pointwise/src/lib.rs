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
    C0(u64, u64, u64, u64, u64, u64, u64, u64, u64, u64),
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
    C0(u64, u64, u64, u64, u64, u64, u64, u64, Adt3, Adt3, Adt3, Adt6, Adt6),
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
    C0,
    C1(Adt15),
}

#[derive(Clone)]
pub enum Adt15 {
    C0(u64, u64, u64),
}

#[derive(Clone)]
pub enum Adt16 {
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
                        if nat_eq(a5, a6) {
                            Ok(Adt9::C0(f2(v2, v7, v4, v6)?))
                        } else {
                            let a7 = v1;
                            let a8 = 78u64;
                            if nat_eq(a7, a8) {
                                Ok(Adt9::C0(f3(v2, v7, v4, v5, v6)?))
                            } else {
                                let a9 = v1;
                                let a10 = 23u64;
                                if if if nat_eq(a9, a10) {
                                    true
                                } else {
                                    let a11 = v1;
                                    let a12 = 26u64;
                                    nat_eq(a11, a12)
                                } {
                                    true
                                } else {
                                    let a13 = v1;
                                    let a14 = 25u64;
                                    nat_eq(a13, a14)
                                } {
                                    Ok(Adt9::C0(Adt10::C5(64u64)))
                                } else {
                                    let a15 = v1;
                                    let a16 = 24u64;
                                    if nat_eq(a15, a16) {
                                        Ok(Adt9::C0(f4(v2, v7, v6)?))
                                    } else {
                                        let a17 = v1;
                                        let a18 = 27u64;
                                        if nat_eq(a17, a18) {
                                            Ok(Adt9::C0(f5(v2, v7, v6)?))
                                        } else {
                                            let a19 = v1;
                                            let a20 = 3u64;
                                            if nat_eq(a19, a20) {
                                                Ok(Adt9::C0(f6(v7, v4, v5, v6)?))
                                            } else {
                                                f7(v1, v2, v7, v4, v6)
                                            }
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
            f9(v13, v14, v15, v16, v17)
        }
        Adt0::C2(v18) => {
            f10(v18)
        }
        Adt0::C3(v19, v20, v21, v22, v23, v24, v25, v26, v27, v28, v29, v30, v31) => {
            Ok(f11(v19, v20, v21, v22, v23, v24, v25, v26, v27, v28, v29, v30, v31))
        }
        Adt0::C4(v8, v9, v10, v11) => {
            f8(v8, v9, v10, v11)
        }
        Adt0::C5(v12) => {
            Ok(Adt9::C9({
                let a21 = 3240099840u64;
                let a22 = v12;
                if if nat_le(a21, a22) {
                    let a23 = v12;
                    let a24 = 4286578688u64;
                    nat_le(a23, a24)
                } else {
                    false
                } {
                    Some(0u64)
                } else {
                    let a25 = 1092616192u64;
                    let a26 = v12;
                    if if nat_le(a25, a26) {
                        let a27 = v12;
                        let a28 = 2139095040u64;
                        nat_le(a27, a28)
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
    let v2: u64 = f12(v1.clone(), 1u64)?;
    let a30 = match v1.clone() {
        Adt2::C0(h29, _, _, _, _, _, _, _, _, _, _) => {
            h29
        }
    };
    let a31 = 0u64;
    if if if if if if if if nat_eq(a30, a31) {
        let a33 = match v1.clone() {
            Adt2::C0(_, _, _, _, _, h32, _, _, _, _, _) => {
                h32
            }
        };
        let a34 = 4u64;
        nat_eq(a33, a34)
    } else {
        false
    } {
        f13(match v0.clone() {
            Adt1::C0(h35, _, _, _, _, _, _, _, _, _) => {
                h35
            }
        })
    } else {
        false
    } {
        f13(match v0.clone() {
            Adt1::C0(_, h36, _, _, _, _, _, _, _, _) => {
                h36
            }
        })
    } else {
        false
    } {
        let a38 = 0u64;
        let a39 = match v0.clone() {
            Adt1::C0(_, _, h37, _, _, _, _, _, _, _) => {
                h37
            }
        };
        nat_lt(a38, a39)
    } else {
        false
    } {
        let a41 = match v0.clone() {
            Adt1::C0(_, _, h40, _, _, _, _, _, _, _) => {
                h40
            }
        };
        let a42 = 2139095040u64;
        nat_lt(a41, a42)
    } else {
        false
    } {
        let a43 = v2;
        let a44 = 9223372036854775808u64;
        nat_lt(a43, a44)
    } else {
        false
    } {
        let a46 = f14(v2)?;
        let a47 = match v1.clone() {
            Adt2::C0(_, _, _, _, _, _, _, _, _, h45, _) => {
                h45
            }
        };
        nat_le(a46, a47)
    } else {
        false
    } {
        Ok(Adt10::C4(f15(v2)?, 4u64, f16(v1.clone(), 1u64), f16(v1, 1u64), Adt4::C23(match v0.clone() {
            Adt1::C0(h48, _, _, _, _, _, _, _, _, _) => {
                h48
            }
        }, match v0.clone() {
            Adt1::C0(_, h49, _, _, _, _, _, _, _, _) => {
                h49
            }
        }, match v0 {
            Adt1::C0(_, _, h50, _, _, _, _, _, _, _) => {
                h50
            }
        }, v2)))
    } else {
        Ok(Adt10::C1)
    }
}

pub fn f2(v0: Adt1, v1: Adt2, v2: Option<Adt2>, v3: Adt2) -> R<Adt10> {
    let m51 = v2;
    match m51 {
        None => {
            Ok(Adt10::C1)
        }
        Some(v4) => {
            let v5: Adt6 = Adt6::C0(match v1.clone() {
                Adt2::C0(_, h52, _, _, _, _, _, _, _, _, _) => {
                    h52
                }
            }, match v0.clone() {
                Adt1::C0(h53, _, _, _, _, _, _, _, _, _) => {
                    h53
                }
            }, match v0.clone() {
                Adt1::C0(_, _, h54, _, _, _, _, _, _, _) => {
                    h54
                }
            }, match v0.clone() {
                Adt1::C0(_, _, _, _, h55, _, _, _, _, _) => {
                    h55
                }
            });
            let v6: Adt6 = Adt6::C0(match v1.clone() {
                Adt2::C0(_, _, h56, _, _, _, _, _, _, _, _) => {
                    h56
                }
            }, match v0.clone() {
                Adt1::C0(_, h57, _, _, _, _, _, _, _, _) => {
                    h57
                }
            }, match v0.clone() {
                Adt1::C0(_, _, _, h58, _, _, _, _, _, _) => {
                    h58
                }
            }, match v0 {
                Adt1::C0(_, _, _, _, _, h59, _, _, _, _) => {
                    h59
                }
            });
            let v7: u64 = f17(v5.clone(), match v4.clone() {
                Adt2::C0(_, h60, _, _, _, _, _, _, _, _, _) => {
                    h60
                }
            })?;
            let v8: u64 = f17(v6.clone(), match v4.clone() {
                Adt2::C0(_, _, h61, _, _, _, _, _, _, _, _) => {
                    h61
                }
            })?;
            let v9: u64 = {
                let a65 = {
                    let a62 = v7;
                    let a63 = v8;
                    nat_mul(a62, a63)?
                };
                let a66 = match v4.clone() {
                    Adt2::C0(_, _, _, _, h64, _, _, _, _, _, _) => {
                        h64
                    }
                };
                nat_mul(a65, a66)?
            };
            let v10: u64 = match v1.clone() {
                Adt2::C0(_, _, _, _, h67, _, _, _, _, _, _) => {
                    h67
                }
            };
            let v11: u64 = {
                let a73 = {
                    let a70 = match v5.clone() {
                        Adt6::C0(h68, _, _, _) => {
                            h68
                        }
                    };
                    let a71 = match v6.clone() {
                        Adt6::C0(h69, _, _, _) => {
                            h69
                        }
                    };
                    nat_mul(a70, a71)?
                };
                let a74 = match v1.clone() {
                    Adt2::C0(_, _, _, h72, _, _, _, _, _, _, _) => {
                        h72
                    }
                };
                nat_mul(a73, a74)?
            };
            let v12: u64 = {
                let a75 = v9;
                let a76 = 32u64;
                if nat_lt(a75, a76) {
                    v9
                } else {
                    32u64
                }
            };
            let a78 = match v1.clone() {
                Adt2::C0(h77, _, _, _, _, _, _, _, _, _, _) => {
                    h77
                }
            };
            let a79 = 0u64;
            if if if if if if if if if if if if if if nat_eq(a78, a79) {
                true
            } else {
                let a81 = match v1.clone() {
                    Adt2::C0(h80, _, _, _, _, _, _, _, _, _, _) => {
                        h80
                    }
                };
                let a82 = 1u64;
                nat_eq(a81, a82)
            } {
                let a84 = match v4.clone() {
                    Adt2::C0(h83, _, _, _, _, _, _, _, _, _, _) => {
                        h83
                    }
                };
                let a85 = 0u64;
                nat_eq(a84, a85)
            } else {
                false
            } {
                let a87 = match v3.clone() {
                    Adt2::C0(h86, _, _, _, _, _, _, _, _, _, _) => {
                        h86
                    }
                };
                let a88 = 0u64;
                nat_eq(a87, a88)
            } else {
                false
            } {
                let a91 = match v1.clone() {
                    Adt2::C0(_, _, _, h89, _, _, _, _, _, _, _) => {
                        h89
                    }
                };
                let a92 = match v4.clone() {
                    Adt2::C0(_, _, _, h90, _, _, _, _, _, _, _) => {
                        h90
                    }
                };
                nat_eq(a91, a92)
            } else {
                false
            } {
                let a94 = match v3.clone() {
                    Adt2::C0(_, h93, _, _, _, _, _, _, _, _, _) => {
                        h93
                    }
                };
                let a95 = v7;
                nat_eq(a94, a95)
            } else {
                false
            } {
                let a97 = match v3.clone() {
                    Adt2::C0(_, _, h96, _, _, _, _, _, _, _, _) => {
                        h96
                    }
                };
                let a98 = v8;
                nat_eq(a97, a98)
            } else {
                false
            } {
                let a100 = match v3.clone() {
                    Adt2::C0(_, _, _, h99, _, _, _, _, _, _, _) => {
                        h99
                    }
                };
                let a101 = v10;
                nat_eq(a100, a101)
            } else {
                false
            } {
                let a104 = match v3.clone() {
                    Adt2::C0(_, _, _, _, h102, _, _, _, _, _, _) => {
                        h102
                    }
                };
                let a105 = match v4.clone() {
                    Adt2::C0(_, _, _, _, h103, _, _, _, _, _, _) => {
                        h103
                    }
                };
                nat_eq(a104, a105)
            } else {
                false
            } {
                let a106 = 0u64;
                let a107 = v11;
                nat_lt(a106, a107)
            } else {
                false
            } {
                let a108 = 0u64;
                let a109 = v10;
                nat_lt(a108, a109)
            } else {
                false
            } {
                let a110 = 0u64;
                let a111 = v9;
                nat_lt(a110, a111)
            } else {
                false
            } {
                f18(v1.clone(), v3.clone())?
            } else {
                false
            } {
                f18(v4.clone(), v3.clone())?
            } else {
                false
            } {
                Ok(Adt10::C6(Adt8::C0(v9, v11, v10, {
                    let a112 = v11;
                    let a113 = v10;
                    nat_mul(a112, a113)?
                }, v12, {
                    let a114 = v12;
                    let a115 = v11;
                    nat_mul(a114, a115)?
                }, {
                    let a116 = v12;
                    let a117 = v10;
                    nat_mul(a116, a117)?
                }, match v1.clone() {
                    Adt2::C0(h118, _, _, _, _, _, _, _, _, _, _) => {
                        h118
                    }
                }, f16(v1, 1u64), f16(v4, 1u64), f16(v3, 1u64), v5, v6)))
            } else {
                Ok(Adt10::C1)
            }
        }
    }
}

pub fn f3(v0: Adt1, v1: Adt2, v2: Option<Adt2>, v3: Option<Adt2>, v4: Adt2) -> R<Adt10> {
    let m119 = v2;
    match m119 {
        None => {
            Ok(Adt10::C1)
        }
        Some(v5) => {
            let m120 = v3;
            match m120 {
                None => {
                    Ok(Adt10::C1)
                }
                Some(v6) => {
                    let a122 = match v1.clone() {
                        Adt2::C0(h121, _, _, _, _, _, _, _, _, _, _) => {
                            h121
                        }
                    };
                    let a123 = 0u64;
                    if if if if if if if if if if if if if if if nat_eq(a122, a123) {
                        let a125 = match v5.clone() {
                            Adt2::C0(h124, _, _, _, _, _, _, _, _, _, _) => {
                                h124
                            }
                        };
                        let a126 = 0u64;
                        nat_eq(a125, a126)
                    } else {
                        false
                    } {
                        let a128 = match v6.clone() {
                            Adt2::C0(h127, _, _, _, _, _, _, _, _, _, _) => {
                                h127
                            }
                        };
                        let a129 = 0u64;
                        nat_eq(a128, a129)
                    } else {
                        false
                    } {
                        let a131 = match v4.clone() {
                            Adt2::C0(h130, _, _, _, _, _, _, _, _, _, _) => {
                                h130
                            }
                        };
                        let a132 = 0u64;
                        nat_eq(a131, a132)
                    } else {
                        false
                    } {
                        f19(v5.clone(), v6.clone())
                    } else {
                        false
                    } {
                        f19(v1.clone(), v4.clone())
                    } else {
                        false
                    } {
                        f20(v1.clone())?
                    } else {
                        false
                    } {
                        f20(v5.clone())?
                    } else {
                        false
                    } {
                        f20(v6)?
                    } else {
                        false
                    } {
                        f20(v4.clone())?
                    } else {
                        false
                    } {
                        let a138 = match v1.clone() {
                            Adt2::C0(_, h133, _, _, _, _, _, _, _, _, _) => {
                                h133
                            }
                        };
                        let a139 = {
                            let a136 = match v5.clone() {
                                Adt2::C0(_, h134, _, _, _, _, _, _, _, _, _) => {
                                    h134
                                }
                            };
                            let a137 = match v5.clone() {
                                Adt2::C0(_, h135, _, _, _, _, _, _, _, _, _) => {
                                    h135
                                }
                            };
                            nat_mul(a136, a137)?
                        };
                        nat_eq(a138, a139)
                    } else {
                        false
                    } {
                        let a145 = match v1.clone() {
                            Adt2::C0(_, _, h140, _, _, _, _, _, _, _, _) => {
                                h140
                            }
                        };
                        let a146 = {
                            let a143 = match v5.clone() {
                                Adt2::C0(_, _, h141, _, _, _, _, _, _, _, _) => {
                                    h141
                                }
                            };
                            let a144 = match v5.clone() {
                                Adt2::C0(_, _, _, h142, _, _, _, _, _, _, _) => {
                                    h142
                                }
                            };
                            nat_mul(a143, a144)?
                        };
                        nat_eq(a145, a146)
                    } else {
                        false
                    } {
                        let a149 = match v1.clone() {
                            Adt2::C0(_, _, _, h147, _, _, _, _, _, _, _) => {
                                h147
                            }
                        };
                        let a150 = match v5.clone() {
                            Adt2::C0(_, _, _, _, h148, _, _, _, _, _, _) => {
                                h148
                            }
                        };
                        nat_eq(a149, a150)
                    } else {
                        false
                    } {
                        let a152 = match v0.clone() {
                            Adt1::C0(h151, _, _, _, _, _, _, _, _, _) => {
                                h151
                            }
                        };
                        let a153 = 4294967296u64;
                        nat_lt(a152, a153)
                    } else {
                        false
                    } {
                        let a155 = match v0.clone() {
                            Adt1::C0(h154, _, _, _, _, _, _, _, _, _) => {
                                h154
                            }
                        };
                        let a156 = 0u64;
                        if nat_eq(a155, a156) {
                            if f18(v1.clone(), v4.clone())? {
                                true
                            } else {
                                let a159 = match v1.clone() {
                                    Adt2::C0(_, _, _, _, _, _, _, _, _, _, h157) => {
                                        h157
                                    }
                                };
                                let a160 = match v4.clone() {
                                    Adt2::C0(_, _, _, _, _, _, _, _, _, _, h158) => {
                                        h158
                                    }
                                };
                                nat_eq(a159, a160)
                            }
                        } else {
                            let a163 = match v1.clone() {
                                Adt2::C0(_, _, _, _, _, _, _, _, _, _, h161) => {
                                    h161
                                }
                            };
                            let a164 = match v4.clone() {
                                Adt2::C0(_, _, _, _, _, _, _, _, _, _, h162) => {
                                    h162
                                }
                            };
                            nat_eq(a163, a164)
                        }
                    } else {
                        false
                    } {
                        let v7: bool = {
                            let a166 = match v4.clone() {
                                Adt2::C0(_, h165, _, _, _, _, _, _, _, _, _) => {
                                    h165
                                }
                            };
                            let a167 = 0u64;
                            if if if nat_eq(a166, a167) {
                                true
                            } else {
                                let a169 = match v4.clone() {
                                    Adt2::C0(_, _, h168, _, _, _, _, _, _, _, _) => {
                                        h168
                                    }
                                };
                                let a170 = 0u64;
                                nat_eq(a169, a170)
                            } {
                                true
                            } else {
                                let a172 = match v4.clone() {
                                    Adt2::C0(_, _, _, h171, _, _, _, _, _, _, _) => {
                                        h171
                                    }
                                };
                                let a173 = 0u64;
                                nat_eq(a172, a173)
                            } {
                                true
                            } else {
                                let a175 = match v4.clone() {
                                    Adt2::C0(_, _, _, _, h174, _, _, _, _, _, _) => {
                                        h174
                                    }
                                };
                                let a176 = 0u64;
                                nat_eq(a175, a176)
                            }
                        };
                        let v8: u64 = match v5.clone() {
                            Adt2::C0(_, h177, _, _, _, _, _, _, _, _, _) => {
                                h177
                            }
                        };
                        let v9: u64 = if v7 {
                            0u64
                        } else {
                            let a179 = match v0 {
                                Adt1::C0(h178, _, _, _, _, _, _, _, _, _) => {
                                    h178
                                }
                            };
                            let a180 = 0u64;
                            if nat_eq(a179, a180) {
                                f12(v1.clone(), 1u64)?
                            } else {
                                0u64
                            }
                        };
                        let v10: u64 = if v7 {
                            0u64
                        } else {
                            f21(v8, match v5.clone() {
                                Adt2::C0(_, _, h181, _, _, _, _, _, _, _, _) => {
                                    h181
                                }
                            }, match v5.clone() {
                                Adt2::C0(_, _, _, h182, _, _, _, _, _, _, _) => {
                                    h182
                                }
                            }, match v5 {
                                Adt2::C0(_, _, _, _, h183, _, _, _, _, _, _) => {
                                    h183
                                }
                            })?
                        };
                        Ok(Adt10::C4(f22(v9, v8, v10)?, 4u64, f16(v1, 1u64), f16(v4, 1u64), Adt4::C25(v9, v8, v10)))
                    } else {
                        Ok(Adt10::C1)
                    }
                }
            }
        }
    }
}

pub fn f4(v0: Adt1, v1: Adt2, v2: Adt2) -> R<Adt10> {
    let v3: u64 = match v1.clone() {
        Adt2::C0(h184, _, _, _, _, _, _, _, _, _, _) => {
            h184
        }
    };
    let v4: u64 = f23(v3);
    let a186 = match v1.clone() {
        Adt2::C0(h185, _, _, _, _, _, _, _, _, _, _) => {
            h185
        }
    };
    let a187 = 0u64;
    if if if if if if if if nat_eq(a186, a187) {
        true
    } else {
        let a189 = match v1.clone() {
            Adt2::C0(h188, _, _, _, _, _, _, _, _, _, _) => {
                h188
            }
        };
        let a190 = 30u64;
        nat_eq(a189, a190)
    } {
        let a193 = match v1.clone() {
            Adt2::C0(h191, _, _, _, _, _, _, _, _, _, _) => {
                h191
            }
        };
        let a194 = match v2.clone() {
            Adt2::C0(h192, _, _, _, _, _, _, _, _, _, _) => {
                h192
            }
        };
        nat_eq(a193, a194)
    } else {
        false
    } {
        f19(v1.clone(), v2.clone())
    } else {
        false
    } {
        f24(match v0.clone() {
            Adt1::C0(h195, _, _, _, _, _, _, _, _, _) => {
                h195
            }
        })
    } else {
        false
    } {
        f25(v1.clone(), v4)
    } else {
        false
    } {
        f25(v2.clone(), v4)
    } else {
        false
    } {
        if f18(v1.clone(), v2.clone())? {
            true
        } else {
            f26(v1.clone(), v2.clone())
        }
    } else {
        false
    } {
        let v5: u64 = if f27(match v1.clone() {
            Adt2::C0(_, h196, _, _, _, _, _, _, _, _, _) => {
                h196
            }
        }, match v1.clone() {
            Adt2::C0(_, _, h197, _, _, _, _, _, _, _, _) => {
                h197
            }
        }, match v1.clone() {
            Adt2::C0(_, _, _, h198, _, _, _, _, _, _, _) => {
                h198
            }
        }, match v1.clone() {
            Adt2::C0(_, _, _, _, h199, _, _, _, _, _, _) => {
                h199
            }
        }) {
            0u64
        } else {
            let a205 = match v1.clone() {
                Adt2::C0(_, _, h200, _, _, _, _, _, _, _, _) => {
                    h200
                }
            };
            let a206 = {
                let a203 = match v1.clone() {
                    Adt2::C0(_, _, _, h201, _, _, _, _, _, _, _) => {
                        h201
                    }
                };
                let a204 = match v1.clone() {
                    Adt2::C0(_, _, _, _, h202, _, _, _, _, _, _) => {
                        h202
                    }
                };
                nat_mul(a203, a204)?
            };
            nat_mul(a205, a206)?
        };
        let v6: u64 = {
            let a207 = v5;
            let a208 = 0u64;
            if if nat_eq(a207, a208) {
                true
            } else {
                f28(v2.clone(), v4)?
            } {
                0u64
            } else {
                let a210 = match v2.clone() {
                    Adt2::C0(_, _, _, _, _, _, _, _, _, h209, _) => {
                        h209
                    }
                };
                let a211 = v4;
                let a212 = 0u64;
                nat_quot(a210, a211, a212)
            }
        };
        Ok(Adt10::C4(f29(v3, match v1.clone() {
            Adt2::C0(_, h213, _, _, _, _, _, _, _, _, _) => {
                h213
            }
        }, v5, v6)?, v4, f16(v1.clone(), 1u64), f16(v2, 1u64), Adt4::C26(v3, match v0 {
            Adt1::C0(h214, _, _, _, _, _, _, _, _, _) => {
                h214
            }
        }, v5, f30(match v1 {
            Adt2::C0(_, h215, _, _, _, _, _, _, _, _, _) => {
                h215
            }
        }, v5, v6)?)))
    } else {
        Ok(Adt10::C1)
    }
}

pub fn f5(v0: Adt1, v1: Adt2, v2: Adt2) -> R<Adt10> {
    let a217 = match v1.clone() {
        Adt2::C0(h216, _, _, _, _, _, _, _, _, _, _) => {
            h216
        }
    };
    let a218 = 0u64;
    if if if if if nat_eq(a217, a218) {
        let a220 = match v2.clone() {
            Adt2::C0(h219, _, _, _, _, _, _, _, _, _, _) => {
                h219
            }
        };
        let a221 = 0u64;
        nat_eq(a220, a221)
    } else {
        false
    } {
        f19(v1.clone(), v2.clone())
    } else {
        false
    } {
        f31(match v0.clone() {
            Adt1::C0(h222, _, _, _, _, _, _, _, _, _) => {
                h222
            }
        })
    } else {
        false
    } {
        if f18(v1.clone(), v2.clone())? {
            true
        } else {
            f26(v1.clone(), v2.clone())
        }
    } else {
        false
    } {
        let v3: Adt3 = f16(v1, 1u64);
        let v4: u64 = {
            let a228 = match v3.clone() {
                Adt3::C0(_, h223, _, _, _, _, _, _) => {
                    h223
                }
            };
            let a229 = {
                let a226 = match v3.clone() {
                    Adt3::C0(_, _, h224, _, _, _, _, _) => {
                        h224
                    }
                };
                let a227 = match v3.clone() {
                    Adt3::C0(_, _, _, h225, _, _, _, _) => {
                        h225
                    }
                };
                nat_mul(a226, a227)?
            };
            nat_mul(a228, a229)?
        };
        Ok(Adt10::C4(f32(match v3.clone() {
            Adt3::C0(h230, _, _, _, _, _, _, _) => {
                h230
            }
        }, v4)?, 4u64, v3, f16(v2, 1u64), Adt4::C24(match v0 {
            Adt1::C0(h231, _, _, _, _, _, _, _, _, _) => {
                h231
            }
        })))
    } else {
        Ok(Adt10::C1)
    }
}

pub fn f6(v0: Adt2, v1: Option<Adt2>, v2: Option<Adt2>, v3: Adt2) -> R<Adt10> {
    let m232 = v1;
    match m232 {
        None => {
            Ok(Adt10::C1)
        }
        Some(v4) => {
            let m233 = v2;
            match m233 {
                None => {
                    Ok(Adt10::C1)
                }
                Some(v5) => {
                    let a235 = match v0.clone() {
                        Adt2::C0(h234, _, _, _, _, _, _, _, _, _, _) => {
                            h234
                        }
                    };
                    let a236 = 0u64;
                    if if if if if if if if if if if if if if if if if nat_eq(a235, a236) {
                        let a238 = match v4.clone() {
                            Adt2::C0(h237, _, _, _, _, _, _, _, _, _, _) => {
                                h237
                            }
                        };
                        let a239 = 0u64;
                        nat_eq(a238, a239)
                    } else {
                        false
                    } {
                        let a241 = match v5.clone() {
                            Adt2::C0(h240, _, _, _, _, _, _, _, _, _, _) => {
                                h240
                            }
                        };
                        let a242 = 26u64;
                        nat_eq(a241, a242)
                    } else {
                        false
                    } {
                        let a244 = match v3.clone() {
                            Adt2::C0(h243, _, _, _, _, _, _, _, _, _, _) => {
                                h243
                            }
                        };
                        let a245 = 0u64;
                        nat_eq(a244, a245)
                    } else {
                        false
                    } {
                        f19(v0.clone(), v3.clone())
                    } else {
                        false
                    } {
                        let a248 = match v0.clone() {
                            Adt2::C0(_, h246, _, _, _, _, _, _, _, _, _) => {
                                h246
                            }
                        };
                        let a249 = match v4.clone() {
                            Adt2::C0(_, h247, _, _, _, _, _, _, _, _, _) => {
                                h247
                            }
                        };
                        nat_eq(a248, a249)
                    } else {
                        false
                    } {
                        let a252 = match v0.clone() {
                            Adt2::C0(_, _, h250, _, _, _, _, _, _, _, _) => {
                                h250
                            }
                        };
                        let a253 = match v5.clone() {
                            Adt2::C0(_, h251, _, _, _, _, _, _, _, _, _) => {
                                h251
                            }
                        };
                        nat_eq(a252, a253)
                    } else {
                        false
                    } {
                        let a256 = match v0.clone() {
                            Adt2::C0(_, _, _, h254, _, _, _, _, _, _, _) => {
                                h254
                            }
                        };
                        let a257 = match v5.clone() {
                            Adt2::C0(_, _, h255, _, _, _, _, _, _, _, _) => {
                                h255
                            }
                        };
                        nat_eq(a256, a257)
                    } else {
                        false
                    } {
                        let a259 = 0u64;
                        let a260 = match v5.clone() {
                            Adt2::C0(_, _, _, h258, _, _, _, _, _, _, _) => {
                                h258
                            }
                        };
                        nat_lt(a259, a260)
                    } else {
                        false
                    } {
                        let a262 = 0u64;
                        let a263 = match v5.clone() {
                            Adt2::C0(_, _, _, _, h261, _, _, _, _, _, _) => {
                                h261
                            }
                        };
                        nat_lt(a262, a263)
                    } else {
                        false
                    } {
                        let a265 = 4u64;
                        let a266 = match v0.clone() {
                            Adt2::C0(_, _, _, _, _, h264, _, _, _, _, _) => {
                                h264
                            }
                        };
                        nat_le(a265, a266)
                    } else {
                        false
                    } {
                        let a268 = 4u64;
                        let a269 = match v4.clone() {
                            Adt2::C0(_, _, _, _, _, h267, _, _, _, _, _) => {
                                h267
                            }
                        };
                        nat_le(a268, a269)
                    } else {
                        false
                    } {
                        let a271 = 4u64;
                        let a272 = match v5.clone() {
                            Adt2::C0(_, _, _, _, _, h270, _, _, _, _, _) => {
                                h270
                            }
                        };
                        nat_le(a271, a272)
                    } else {
                        false
                    } {
                        let a274 = 4u64;
                        let a275 = match v3.clone() {
                            Adt2::C0(_, _, _, _, _, h273, _, _, _, _, _) => {
                                h273
                            }
                        };
                        nat_le(a274, a275)
                    } else {
                        false
                    } {
                        f18(v0.clone(), v3.clone())?
                    } else {
                        false
                    } {
                        f18(v4.clone(), v3.clone())?
                    } else {
                        false
                    } {
                        f18(v5.clone(), v3.clone())?
                    } else {
                        false
                    } {
                        Ok(Adt10::C4(f33(match v0.clone() {
                            Adt2::C0(_, h276, _, _, _, _, _, _, _, _, _) => {
                                h276
                            }
                        }, match v0.clone() {
                            Adt2::C0(_, _, h277, _, _, _, _, _, _, _, _) => {
                                h277
                            }
                        }, match v0.clone() {
                            Adt2::C0(_, _, _, h278, _, _, _, _, _, _, _) => {
                                h278
                            }
                        }, match v0.clone() {
                            Adt2::C0(_, _, _, _, h279, _, _, _, _, _, _) => {
                                h279
                            }
                        })?, 4u64, f16(v0, 1u64), f16(v3, 1u64), Adt4::C22(f16(v4, 1u64), f16(v5, 1u64))))
                    } else {
                        Ok(Adt10::C1)
                    }
                }
            }
        }
    }
}

pub fn f7(v0: u64, v1: Adt1, v2: Adt2, v3: Option<Adt2>, v4: Adt2) -> R<Adt9> {
    let v5: u64 = match v1.clone() {
        Adt1::C0(h280, _, _, _, _, _, _, _, _, _) => {
            h280
        }
    };
    Ok(Adt9::C0({
        let a281 = v0;
        let a282 = 57u64;
        if if nat_eq(a281, a282) {
            true
        } else {
            let a283 = v0;
            let a284 = 58u64;
            nat_eq(a283, a284)
        } {
            let m285 = v3;
            match m285 {
                None => {
                    f34({
                        let a286 = v0;
                        let a287 = 58u64;
                        nat_eq(a286, a287)
                    }, v1, v2, v4)?
                }
                Some(_) => {
                    Adt10::C1
                }
            }
        } else {
            let a288 = v0;
            let a289 = 49u64;
            if nat_eq(a288, a289) {
                f35(v1, v2, v3, v4)?
            } else {
                let a290 = v0;
                let a291 = 51u64;
                if nat_eq(a290, a291) {
                    f36(v1, v2, v3, v4)?
                } else {
                    let a292 = v0;
                    let a293 = 39u64;
                    if nat_eq(a292, a293) {
                        f37(v2, v3, v4)?
                    } else {
                        let a294 = v0;
                        let a295 = 77u64;
                        if nat_eq(a294, a295) {
                            f38(v2, v4)?
                        } else {
                            let a296 = v0;
                            let a297 = 40u64;
                            if nat_eq(a296, a297) {
                                f39(v2, v3, v4)?
                            } else {
                                if f40(v0) {
                                    f41(v0, v2, {
                                        let m298 = v3;
                                        match m298 {
                                            None => {
                                                false
                                            }
                                            Some(_) => {
                                                true
                                            }
                                        }
                                    }, v4)?
                                } else {
                                    let a299 = v0;
                                    let a300 = 19u64;
                                    if nat_eq(a299, a300) {
                                        f42(v2, v4)?
                                    } else {
                                        let a301 = v0;
                                        let a302 = 21u64;
                                        if nat_eq(a301, a302) {
                                            f43(v5, v2, v3, v4)?
                                        } else {
                                            let a303 = v0;
                                            let a304 = 61u64;
                                            if if if nat_eq(a303, a304) {
                                                true
                                            } else {
                                                let a305 = v0;
                                                let a306 = 62u64;
                                                nat_eq(a305, a306)
                                            } {
                                                true
                                            } else {
                                                let a307 = v0;
                                                let a308 = 63u64;
                                                nat_eq(a307, a308)
                                            } {
                                                f44(v0, v1, v2, v4)?
                                            } else {
                                                let a309 = v0;
                                                let a310 = 4u64;
                                                if if nat_eq(a309, a310) {
                                                    true
                                                } else {
                                                    let a311 = v0;
                                                    let a312 = 32u64;
                                                    nat_eq(a311, a312)
                                                } {
                                                    f45(v0, v1, v2, v3, v4)?
                                                } else {
                                                    let a313 = v0;
                                                    let a314 = 20u64;
                                                    if nat_eq(a313, a314) {
                                                        f46(v2, v4)?
                                                    } else {
                                                        let a315 = v0;
                                                        let a316 = 42u64;
                                                        if nat_eq(a315, a316) {
                                                            f47(4u64, v2, v4)?
                                                        } else {
                                                            let a317 = v0;
                                                            let a318 = 69u64;
                                                            if nat_eq(a317, a318) {
                                                                let a319 = v5;
                                                                let a320 = 4u64;
                                                                if nat_lt(a319, a320) {
                                                                    f47(v5, v2, v4)?
                                                                } else {
                                                                    Adt10::C1
                                                                }
                                                            } else {
                                                                let a321 = v0;
                                                                let a322 = 70u64;
                                                                if nat_eq(a321, a322) {
                                                                    f48(v5, v2, v4)?
                                                                } else {
                                                                    let a323 = v0;
                                                                    let a324 = 75u64;
                                                                    if if nat_eq(a323, a324) {
                                                                        true
                                                                    } else {
                                                                        let a325 = v0;
                                                                        let a326 = 76u64;
                                                                        nat_eq(a325, a326)
                                                                    } {
                                                                        f49({
                                                                            let a327 = v0;
                                                                            let a328 = 76u64;
                                                                            nat_eq(a327, a328)
                                                                        }, v1, v2, v4)?
                                                                    } else {
                                                                        let a329 = v0;
                                                                        let a330 = 31u64;
                                                                        if if if nat_eq(a329, a330) {
                                                                            true
                                                                        } else {
                                                                            let a331 = v0;
                                                                            let a332 = 47u64;
                                                                            nat_eq(a331, a332)
                                                                        } {
                                                                            true
                                                                        } else {
                                                                            let a333 = v0;
                                                                            let a334 = 68u64;
                                                                            nat_eq(a333, a334)
                                                                        } {
                                                                            f50(v0, v1, v2, v4)?
                                                                        } else {
                                                                            let a335 = v0;
                                                                            let a336 = 15u64;
                                                                            if nat_eq(a335, a336) {
                                                                                f51(v2, v4)?
                                                                            } else {
                                                                                let a337 = v0;
                                                                                let a338 = 17u64;
                                                                                if nat_eq(a337, a338) {
                                                                                    f52(v2, v4)?
                                                                                } else {
                                                                                    let a339 = v0;
                                                                                    let a340 = 18u64;
                                                                                    if nat_eq(a339, a340) {
                                                                                        f53(v2, v3, v4)?
                                                                                    } else {
                                                                                        let a341 = v0;
                                                                                        let a342 = 13u64;
                                                                                        if if if nat_eq(a341, a342) {
                                                                                            true
                                                                                        } else {
                                                                                            let a343 = v0;
                                                                                            let a344 = 14u64;
                                                                                            nat_eq(a343, a344)
                                                                                        } {
                                                                                            true
                                                                                        } else {
                                                                                            let a345 = v0;
                                                                                            let a346 = 16u64;
                                                                                            nat_eq(a345, a346)
                                                                                        } {
                                                                                            f54(v0, v2, v4)?
                                                                                        } else {
                                                                                            let m347 = f55(v0, v5)?;
                                                                                            match m347 {
                                                                                                None => {
                                                                                                    f56(v0, v5, v2, v3, v4)?
                                                                                                }
                                                                                                Some(v8) => {
                                                                                                    f57(v8, v2, v4)?
                                                                                                }
                                                                                            }
                                                                                        }
                                                                                    }
                                                                                }
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
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

pub fn f8(v0: Adt8, v1: u64, v2: u64, v3: u64) -> R<Adt9> {
    let a348 = v1;
    let a349 = 0u64;
    if nat_eq(a348, a349) {
        let a351 = v3;
        let a352 = match v0.clone() {
            Adt8::C0(_, _, _, h350, _, _, _, _, _, _, _, _, _) => {
                h350
            }
        };
        if nat_lt(a351, a352) {
            let v4: Adt3 = match v0.clone() {
                Adt8::C0(_, _, _, _, _, _, _, _, h353, _, _, _, _) => {
                    h353
                }
            };
            let v5: u64 = {
                let a355 = v3;
                let a356 = match v0.clone() {
                    Adt8::C0(_, _, h354, _, _, _, _, _, _, _, _, _, _) => {
                        h354
                    }
                };
                let a357 = 0u64;
                nat_quot(a355, a356, a357)
            };
            Ok(Adt9::C8(Some(f58(v4.clone(), f59(v5, v4.clone(), 0u64), f59(v5, v4.clone(), 1u64), f59(v5, v4, 2u64), {
                let a359 = v3;
                let a360 = match v0 {
                    Adt8::C0(_, _, h358, _, _, _, _, _, _, _, _, _, _) => {
                        h358
                    }
                };
                let a361 = 0u64;
                nat_rem(a359, a360, a361)
            })?)))
        } else {
            Ok(Adt9::C0(Adt10::C1))
        }
    } else {
        let v6: Adt3 = match v0.clone() {
            Adt8::C0(_, _, _, _, _, _, _, _, _, _, h362, _, _) => {
                h362
            }
        };
        let v7: u64 = {
            let a363 = v1;
            let a364 = 1u64;
            if nat_eq(a363, a364) {
                match v0.clone() {
                    Adt8::C0(_, h365, _, _, _, _, _, _, _, _, _, _, _) => {
                        h365
                    }
                }
            } else {
                match v0.clone() {
                    Adt8::C0(_, _, h366, _, _, _, _, _, _, _, _, _, _) => {
                        h366
                    }
                }
            }
        };
        let v8: u64 = {
            let a370 = v2;
            let a371 = {
                let a367 = v3;
                let a368 = v7;
                let a369 = 0u64;
                nat_quot(a367, a368, a369)
            };
            nat_add(a370, a371)?
        };
        let v9: u64 = {
            let a372 = v3;
            let a373 = v7;
            let a374 = 0u64;
            nat_rem(a372, a373, a374)
        };
        let v10: u64 = {
            let a376 = v8;
            let a377 = match v6.clone() {
                Adt3::C0(h375, _, _, _, _, _, _, _) => {
                    h375
                }
            };
            let a378 = 0u64;
            nat_rem(a376, a377, a378)
        };
        let v11: u64 = {
            let a380 = v8;
            let a381 = match v6.clone() {
                Adt3::C0(h379, _, _, _, _, _, _, _) => {
                    h379
                }
            };
            let a382 = 0u64;
            nat_quot(a380, a381, a382)
        };
        let v12: u64 = {
            let a384 = v11;
            let a385 = match v6.clone() {
                Adt3::C0(_, h383, _, _, _, _, _, _) => {
                    h383
                }
            };
            let a386 = 0u64;
            nat_rem(a384, a385, a386)
        };
        let v13: u64 = {
            let a388 = v11;
            let a389 = match v6.clone() {
                Adt3::C0(_, h387, _, _, _, _, _, _) => {
                    h387
                }
            };
            let a390 = 0u64;
            nat_quot(a388, a389, a390)
        };
        let a392 = v8;
        let a393 = match v0.clone() {
            Adt8::C0(h391, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h391
            }
        };
        if nat_lt(a392, a393) {
            let a394 = v1;
            let a395 = 1u64;
            if nat_eq(a394, a395) {
                let v14: Adt3 = match v0.clone() {
                    Adt8::C0(_, _, _, _, _, _, _, _, h396, _, _, _, _) => {
                        h396
                    }
                };
                let v15: Adt3 = match v0.clone() {
                    Adt8::C0(_, _, _, _, _, _, _, _, _, h397, _, _, _) => {
                        h397
                    }
                };
                let m400 = f60(match v0.clone() {
                    Adt8::C0(_, _, _, _, _, _, _, _, _, _, _, h398, _) => {
                        h398
                    }
                }, match v15.clone() {
                    Adt3::C0(h399, _, _, _, _, _, _, _) => {
                        h399
                    }
                }, v10, f59(v9, v14.clone(), 0u64))?;
                match m400 {
                    None => {
                        Ok(Adt9::C8(None::<u64>))
                    }
                    Some(v16) => {
                        let m403 = f60(match v0 {
                            Adt8::C0(_, _, _, _, _, _, _, _, _, _, _, _, h401) => {
                                h401
                            }
                        }, match v15.clone() {
                            Adt3::C0(_, h402, _, _, _, _, _, _) => {
                                h402
                            }
                        }, v12, f59(v9, v14.clone(), 1u64))?;
                        match m403 {
                            None => {
                                Ok(Adt9::C8(None::<u64>))
                            }
                            Some(v17) => {
                                Ok(Adt9::C8(Some(f58(v15, v16, v17, f59(v9, v14, 2u64), v13)?)))
                            }
                        }
                    }
                }
            } else {
                let a404 = v1;
                let a405 = 2u64;
                if nat_eq(a404, a405) {
                    Ok(Adt9::C8(Some(f58(v6, v10, v12, v9, v13)?)))
                } else {
                    Ok(Adt9::C0(Adt10::C1))
                }
            }
        } else {
            Ok(Adt9::C0(Adt10::C1))
        }
    }
}

pub fn f9(v0: u64, v1: Adt3, v2: Adt3, v3: Adt4, v4: u64) -> R<Adt9> {
    Ok(f61(v3.clone(), {
        let m406 = v3.clone();
        match m406 {
            Adt4::C0 => {
                f62(v0, v1, v2, v3)?
            }
            Adt4::C1 => {
                f62(v0, v1, v2, v3)?
            }
            Adt4::C2(_, _) => {
                f62(v0, v1, v2, v3)?
            }
            Adt4::C3(_, _, _, _, _) => {
                f62(v0, v1, v2, v3)?
            }
            Adt4::C4(_) => {
                f62(v0, v1, v2, v3)?
            }
            Adt4::C5(_, _, _, _) => {
                f62(v0, v1, v2, v3)?
            }
            Adt4::C6(_, _, _) => {
                f62(v0, v1, v2, v3)?
            }
            Adt4::C7(_) => {
                f62(v0, v1, v2, v3)?
            }
            Adt4::C8(_) => {
                f62(v0, v1, v2, v3)?
            }
            Adt4::C9(_, _) => {
                f62(v0, v1, v2, v3)?
            }
            Adt4::C10(_, _, _) => {
                f62(v0, v1, v2, v3)?
            }
            Adt4::C11(_, _) => {
                f62(v0, v1, v2, v3)?
            }
            Adt4::C12(_, _, _) => {
                f62(v0, v1, v2, v3)?
            }
            Adt4::C13(_, _, _, _) => {
                f62(v0, v1, v2, v3)?
            }
            Adt4::C14(_, _, _) => {
                f62(v0, v1, v2, v3)?
            }
            Adt4::C15 => {
                f62(v0, v1, v2, v3)?
            }
            Adt4::C16(_, _) => {
                f62(v0, v1, v2, v3)?
            }
            Adt4::C17 => {
                f63(v0, v1, v2, v4)?
            }
            Adt4::C18(_, _) => {
                f62(v0, v1, v2, v3)?
            }
            Adt4::C19(_, v44) => {
                f64(v0, v1, v2, v44, v4)?
            }
            Adt4::C20(v45, v46) => {
                f65(v45, v0, v1, v2, v46, v4)?
            }
            Adt4::C21 => {
                f62(v0, v1, v2, v3)?
            }
            Adt4::C22(v47, v48) => {
                f66(v0, v1, v2, v47, v48, v4)?
            }
            Adt4::C23(v59, v60, v61, v62) => {
                f70(v0, v62, v59, v60, v61, v4)?
            }
            Adt4::C24(_) => {
                Adt9::C0(Adt10::C5(65u64))
            }
            Adt4::C25(_, _, _) => {
                f62(v0, v1, v2, v3)?
            }
            Adt4::C26(v54, v55, v56, v57) => {
                let a407 = v54;
                let a408 = 64u64;
                if if if nat_eq(a407, a408) {
                    true
                } else {
                    let a409 = v54;
                    let a410 = 65u64;
                    nat_eq(a409, a410)
                } {
                    true
                } else {
                    let a411 = v54;
                    let a412 = 66u64;
                    nat_eq(a411, a412)
                } {
                    Adt9::C0(Adt10::C5(64u64))
                } else {
                    let a413 = v54;
                    let a414 = 0u64;
                    if if nat_eq(a413, a414) {
                        true
                    } else {
                        let a415 = v54;
                        let a416 = 30u64;
                        nat_eq(a415, a416)
                    } {
                        f69(v0, v1, v2, v54, v55, v56, v57, v4)?
                    } else {
                        Adt9::C0(Adt10::C1)
                    }
                }
            }
            Adt4::C27(v53) => {
                f68(v0, v1, v2, v53)?
            }
            Adt4::C28(v52) => {
                f67(v0, v1, v2, v52, v4)?
            }
        }
    }))
}

pub fn f10(v0: Adt4) -> R<Adt9> {
    let m417 = v0;
    match m417 {
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
                let a422 = {
                    let a418 = 0u64;
                    let a419 = v45;
                    if nat_lt(a418, a419) {
                        1u64
                    } else {
                        0u64
                    }
                };
                let a423 = {
                    let a420 = 0u64;
                    let a421 = v47;
                    if nat_lt(a420, a421) {
                        6u64
                    } else {
                        0u64
                    }
                };
                nat_add(a422, a423)?
            }, 0u64, 0u64, false))
        }
        Adt4::C26(v50, _, _, _) => {
            Ok(Adt9::C6({
                let a424 = v50;
                let a425 = 66u64;
                if nat_eq(a424, a425) {
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
    }
}

pub fn f11(v0: bool, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64, v7: u64, v8: u64, v9: u64, v10: u64, v11: u64, v12: u64) -> Adt9 {
    let a426 = v1;
    let a427 = 5u64;
    if if if if nat_lt(a426, a427) {
        let a428 = v5;
        let a429 = 5u64;
        nat_lt(a428, a429)
    } else {
        false
    } {
        let a430 = v7;
        let a431 = 5u64;
        nat_lt(a430, a431)
    } else {
        false
    } {
        let a432 = v9;
        let a433 = 2u64;
        if nat_lt(a432, a433) {
            true
        } else {
            let a434 = v9;
            let a435 = 3u64;
            nat_eq(a434, a435)
        }
    } else {
        false
    } {
        Adt9::C7(f71(v0, v1, v2, v3, v4), f72(v5, v6), f72(v7, v8), f73({
            let a436 = v9;
            let a437 = 0u64;
            if nat_eq(a436, a437) {
                1u64
            } else {
                v9
            }
        }, v10), v11, v12)
    } else {
        Adt9::C0(Adt10::C1)
    }
}

pub fn f12(v0: Adt2, v1: u64) -> R<u64> {
    let a449 = {
        let a443 = {
            let a439 = match v0.clone() {
                Adt2::C0(_, h438, _, _, _, _, _, _, _, _, _) => {
                    h438
                }
            };
            let a440 = v1;
            let a441 = 0u64;
            nat_quot(a439, a440, a441)
        };
        let a444 = match v0.clone() {
            Adt2::C0(_, _, h442, _, _, _, _, _, _, _, _) => {
                h442
            }
        };
        nat_mul(a443, a444)?
    };
    let a450 = {
        let a447 = match v0.clone() {
            Adt2::C0(_, _, _, h445, _, _, _, _, _, _, _) => {
                h445
            }
        };
        let a448 = match v0 {
            Adt2::C0(_, _, _, _, h446, _, _, _, _, _, _) => {
                h446
            }
        };
        nat_mul(a447, a448)?
    };
    nat_mul(a449, a450)
}

pub fn f13(v0: u64) -> bool {
    let a451 = v0;
    let a452 = 4294967296u64;
    if nat_lt(a451, a452) {
        let a456 = {
            let a453 = v0;
            let a454 = 2147483648u64;
            let a455 = 0u64;
            nat_rem(a453, a454, a455)
        };
        let a457 = 2139095040u64;
        nat_lt(a456, a457)
    } else {
        false
    }
}

pub fn f14(v0: u64) -> R<u64> {
    let a458 = 4u64;
    let a459 = v0;
    nat_mul(a458, a459)
}

pub fn f15(v0: u64) -> R<u64> {
    let a460 = v0;
    let a461 = 0u64;
    if nat_eq(a460, a461) {
        Ok(0u64)
    } else {
        let a464 = 4u64;
        let a465 = {
            let a462 = 3u64;
            let a463 = v0;
            nat_mul(a462, a463)?
        };
        nat_add(a464, a465)
    }
}

pub fn f16(v0: Adt2, v1: u64) -> Adt3 {
    Adt3::C0({
        let a467 = match v0.clone() {
            Adt2::C0(_, h466, _, _, _, _, _, _, _, _, _) => {
                h466
            }
        };
        let a468 = v1;
        let a469 = 0u64;
        nat_quot(a467, a468, a469)
    }, match v0.clone() {
        Adt2::C0(_, _, h470, _, _, _, _, _, _, _, _) => {
            h470
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, h471, _, _, _, _, _, _, _) => {
            h471
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, h472, _, _, _, _, _, _) => {
            h472
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, _, h473, _, _, _, _, _) => {
            h473
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, _, _, h474, _, _, _, _) => {
            h474
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, _, _, _, h475, _, _, _) => {
            h475
        }
    }, match v0 {
        Adt2::C0(_, _, _, _, _, _, _, _, h476, _, _) => {
            h476
        }
    })
}

pub fn f17(v0: Adt6, v1: u64) -> R<u64> {
    let a478 = 0u64;
    let a479 = match v0.clone() {
        Adt6::C0(h477, _, _, _) => {
            h477
        }
    };
    if if if if if nat_lt(a478, a479) {
        let a481 = 0u64;
        let a482 = match v0.clone() {
            Adt6::C0(_, h480, _, _) => {
                h480
            }
        };
        nat_lt(a481, a482)
    } else {
        false
    } {
        let a484 = 0u64;
        let a485 = match v0.clone() {
            Adt6::C0(_, _, _, h483) => {
                h483
            }
        };
        nat_lt(a484, a485)
    } else {
        false
    } {
        let a487 = match v0.clone() {
            Adt6::C0(_, h486, _, _) => {
                h486
            }
        };
        let a488 = 2147483648u64;
        nat_lt(a487, a488)
    } else {
        false
    } {
        let a490 = match v0.clone() {
            Adt6::C0(_, _, _, h489) => {
                h489
            }
        };
        let a491 = 2147483648u64;
        nat_lt(a490, a491)
    } else {
        false
    } {
        let v2: u64 = {
            let a495 = v1;
            let a496 = {
                let a493 = 2u64;
                let a494 = f74(match v0.clone() {
                    Adt6::C0(_, _, h492, _) => {
                        h492
                    }
                });
                nat_mul(a493, a494)?
            };
            nat_add(a495, a496)?
        };
        let v3: u64 = {
            let a508 = {
                let a503 = 1u64;
                let a504 = {
                    let a501 = match v0.clone() {
                        Adt6::C0(_, _, _, h497) => {
                            h497
                        }
                    };
                    let a502 = {
                        let a499 = match v0.clone() {
                            Adt6::C0(h498, _, _, _) => {
                                h498
                            }
                        };
                        let a500 = 1u64;
                        nat_sub(a499, a500)
                    };
                    nat_mul(a501, a502)?
                };
                nat_add(a503, a504)?
            };
            let a509 = {
                let a506 = 2u64;
                let a507 = f75(match v0.clone() {
                    Adt6::C0(_, _, h505, _) => {
                        h505
                    }
                });
                nat_mul(a506, a507)?
            };
            nat_add(a508, a509)?
        };
        let a510 = v3;
        let a511 = v2;
        if nat_le(a510, a511) {
            let a518 = 1u64;
            let a519 = {
                let a515 = {
                    let a512 = v2;
                    let a513 = v3;
                    nat_sub(a512, a513)
                };
                let a516 = match v0 {
                    Adt6::C0(_, h514, _, _) => {
                        h514
                    }
                };
                let a517 = 0u64;
                nat_quot(a515, a516, a517)
            };
            nat_add(a518, a519)
        } else {
            let a523 = {
                let a520 = v3;
                let a521 = v2;
                nat_sub(a520, a521)
            };
            let a524 = match v0 {
                Adt6::C0(_, h522, _, _) => {
                    h522
                }
            };
            if nat_lt(a523, a524) {
                Ok(1u64)
            } else {
                Ok(0u64)
            }
        }
    } else {
        Ok(0u64)
    }
}

pub fn f18(v0: Adt2, v1: Adt2) -> R<bool> {
    let a526 = match v0.clone() {
        Adt2::C0(_, _, _, _, _, _, _, _, _, h525, _) => {
            h525
        }
    };
    let a527 = 0u64;
    if nat_eq(a526, a527) {
        Ok(true)
    } else {
        let a529 = match v1.clone() {
            Adt2::C0(_, _, _, _, _, _, _, _, _, h528, _) => {
                h528
            }
        };
        let a530 = 0u64;
        if nat_eq(a529, a530) {
            Ok(true)
        } else {
            let a532 = match v0.clone() {
                Adt2::C0(_, _, _, _, _, _, _, _, _, _, h531) => {
                    h531
                }
            };
            let a533 = 0u64;
            if nat_eq(a532, a533) {
                Ok(true)
            } else {
                let a535 = match v1.clone() {
                    Adt2::C0(_, _, _, _, _, _, _, _, _, _, h534) => {
                        h534
                    }
                };
                let a536 = 0u64;
                if nat_eq(a535, a536) {
                    Ok(true)
                } else {
                    let a542 = {
                        let a539 = match v0.clone() {
                            Adt2::C0(_, _, _, _, _, _, _, _, _, _, h537) => {
                                h537
                            }
                        };
                        let a540 = match v0.clone() {
                            Adt2::C0(_, _, _, _, _, _, _, _, _, h538, _) => {
                                h538
                            }
                        };
                        nat_add(a539, a540)?
                    };
                    let a543 = match v1.clone() {
                        Adt2::C0(_, _, _, _, _, _, _, _, _, _, h541) => {
                            h541
                        }
                    };
                    if nat_le(a542, a543) {
                        Ok(true)
                    } else {
                        let a549 = {
                            let a546 = match v1.clone() {
                                Adt2::C0(_, _, _, _, _, _, _, _, _, _, h544) => {
                                    h544
                                }
                            };
                            let a547 = match v1 {
                                Adt2::C0(_, _, _, _, _, _, _, _, _, h545, _) => {
                                    h545
                                }
                            };
                            nat_add(a546, a547)?
                        };
                        let a550 = match v0 {
                            Adt2::C0(_, _, _, _, _, _, _, _, _, _, h548) => {
                                h548
                            }
                        };
                        Ok(nat_le(a549, a550))
                    }
                }
            }
        }
    }
}

pub fn f19(v0: Adt2, v1: Adt2) -> bool {
    let a553 = match v0.clone() {
        Adt2::C0(_, h551, _, _, _, _, _, _, _, _, _) => {
            h551
        }
    };
    let a554 = match v1.clone() {
        Adt2::C0(_, h552, _, _, _, _, _, _, _, _, _) => {
            h552
        }
    };
    if if if nat_eq(a553, a554) {
        let a557 = match v0.clone() {
            Adt2::C0(_, _, h555, _, _, _, _, _, _, _, _) => {
                h555
            }
        };
        let a558 = match v1.clone() {
            Adt2::C0(_, _, h556, _, _, _, _, _, _, _, _) => {
                h556
            }
        };
        nat_eq(a557, a558)
    } else {
        false
    } {
        let a561 = match v0.clone() {
            Adt2::C0(_, _, _, h559, _, _, _, _, _, _, _) => {
                h559
            }
        };
        let a562 = match v1.clone() {
            Adt2::C0(_, _, _, h560, _, _, _, _, _, _, _) => {
                h560
            }
        };
        nat_eq(a561, a562)
    } else {
        false
    } {
        let a565 = match v0 {
            Adt2::C0(_, _, _, _, h563, _, _, _, _, _, _) => {
                h563
            }
        };
        let a566 = match v1 {
            Adt2::C0(_, _, _, _, h564, _, _, _, _, _, _) => {
                h564
            }
        };
        nat_eq(a565, a566)
    } else {
        false
    }
}

pub fn f20(v0: Adt2) -> R<bool> {
    let a568 = match v0.clone() {
        Adt2::C0(_, h567, _, _, _, _, _, _, _, _, _) => {
            h567
        }
    };
    let a569 = 1u64;
    if if if if nat_eq(a568, a569) {
        true
    } else {
        let a571 = match v0.clone() {
            Adt2::C0(_, _, _, _, _, h570, _, _, _, _, _) => {
                h570
            }
        };
        let a572 = 4u64;
        nat_eq(a571, a572)
    } {
        let a574 = match v0.clone() {
            Adt2::C0(_, _, h573, _, _, _, _, _, _, _, _) => {
                h573
            }
        };
        let a575 = 1u64;
        if nat_eq(a574, a575) {
            true
        } else {
            let a580 = match v0.clone() {
                Adt2::C0(_, _, _, _, _, _, h576, _, _, _, _) => {
                    h576
                }
            };
            let a581 = {
                let a578 = match v0.clone() {
                    Adt2::C0(_, h577, _, _, _, _, _, _, _, _, _) => {
                        h577
                    }
                };
                let a579 = 4u64;
                nat_mul(a578, a579)?
            };
            nat_eq(a580, a581)
        }
    } else {
        false
    } {
        let a583 = match v0.clone() {
            Adt2::C0(_, _, _, h582, _, _, _, _, _, _, _) => {
                h582
            }
        };
        let a584 = 1u64;
        if nat_eq(a583, a584) {
            true
        } else {
            let a592 = match v0.clone() {
                Adt2::C0(_, _, _, _, _, _, _, h585, _, _, _) => {
                    h585
                }
            };
            let a593 = {
                let a590 = {
                    let a588 = match v0.clone() {
                        Adt2::C0(_, h586, _, _, _, _, _, _, _, _, _) => {
                            h586
                        }
                    };
                    let a589 = match v0.clone() {
                        Adt2::C0(_, _, h587, _, _, _, _, _, _, _, _) => {
                            h587
                        }
                    };
                    nat_mul(a588, a589)?
                };
                let a591 = 4u64;
                nat_mul(a590, a591)?
            };
            nat_eq(a592, a593)
        }
    } else {
        false
    } {
        let a595 = match v0.clone() {
            Adt2::C0(_, _, _, _, h594, _, _, _, _, _, _) => {
                h594
            }
        };
        let a596 = 1u64;
        if nat_eq(a595, a596) {
            Ok(true)
        } else {
            let a607 = match v0.clone() {
                Adt2::C0(_, _, _, _, _, _, _, _, h597, _, _) => {
                    h597
                }
            };
            let a608 = {
                let a605 = {
                    let a603 = {
                        let a600 = match v0.clone() {
                            Adt2::C0(_, h598, _, _, _, _, _, _, _, _, _) => {
                                h598
                            }
                        };
                        let a601 = match v0.clone() {
                            Adt2::C0(_, _, h599, _, _, _, _, _, _, _, _) => {
                                h599
                            }
                        };
                        nat_mul(a600, a601)?
                    };
                    let a604 = match v0 {
                        Adt2::C0(_, _, _, h602, _, _, _, _, _, _, _) => {
                            h602
                        }
                    };
                    nat_mul(a603, a604)?
                };
                let a606 = 4u64;
                nat_mul(a605, a606)?
            };
            Ok(nat_eq(a607, a608))
        }
    } else {
        Ok(false)
    }
}

pub fn f21(v0: u64, v1: u64, v2: u64, v3: u64) -> R<u64> {
    let a613 = {
        let a609 = v0;
        let a610 = v1;
        nat_mul(a609, a610)?
    };
    let a614 = {
        let a611 = v2;
        let a612 = f76(v3);
        nat_mul(a611, a612)?
    };
    nat_mul(a613, a614)
}

pub fn f22(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let a617 = v0;
    let a618 = {
        let a615 = f77(v1)?;
        let a616 = v2;
        nat_mul(a615, a616)?
    };
    nat_add(a617, a618)
}

pub fn f23(v0: u64) -> u64 {
    let a619 = v0;
    let a620 = 30u64;
    if nat_eq(a619, a620) {
        2u64
    } else {
        4u64
    }
}

pub fn f24(v0: u64) -> bool {
    let a621 = v0;
    let a622 = 2139095040u64;
    if nat_le(a621, a622) {
        true
    } else {
        let a623 = v0;
        let a624 = 2147483648u64;
        nat_eq(a623, a624)
    }
}

pub fn f25(v0: Adt2, v1: u64) -> bool {
    let a626 = match v0.clone() {
        Adt2::C0(_, _, _, _, _, h625, _, _, _, _, _) => {
            h625
        }
    };
    let a627 = v1;
    if if nat_eq(a626, a627) {
        let a632 = {
            let a629 = match v0.clone() {
                Adt2::C0(_, _, _, _, _, _, _, _, _, _, h628) => {
                    h628
                }
            };
            let a630 = v1;
            let a631 = 0u64;
            nat_rem(a629, a630, a631)
        };
        let a633 = 0u64;
        nat_eq(a632, a633)
    } else {
        false
    } {
        f78(match v0.clone() {
            Adt2::C0(_, _, h634, _, _, _, _, _, _, _, _) => {
                h634
            }
        }, match v0.clone() {
            Adt2::C0(_, _, _, h635, _, _, _, _, _, _, _) => {
                h635
            }
        }, match v0.clone() {
            Adt2::C0(_, _, _, _, h636, _, _, _, _, _, _) => {
                h636
            }
        }, match v0.clone() {
            Adt2::C0(_, _, _, _, _, _, h637, _, _, _, _) => {
                h637
            }
        }, match v0.clone() {
            Adt2::C0(_, _, _, _, _, _, _, h638, _, _, _) => {
                h638
            }
        }, match v0 {
            Adt2::C0(_, _, _, _, _, _, _, _, h639, _, _) => {
                h639
            }
        }, v1)
    } else {
        false
    }
}

pub fn f26(v0: Adt2, v1: Adt2) -> bool {
    if if if if if if f19(v0.clone(), v1.clone()) {
        let a642 = match v0.clone() {
            Adt2::C0(h640, _, _, _, _, _, _, _, _, _, _) => {
                h640
            }
        };
        let a643 = match v1.clone() {
            Adt2::C0(h641, _, _, _, _, _, _, _, _, _, _) => {
                h641
            }
        };
        nat_eq(a642, a643)
    } else {
        false
    } {
        let a646 = match v0.clone() {
            Adt2::C0(_, _, _, _, _, _, _, _, _, _, h644) => {
                h644
            }
        };
        let a647 = match v1.clone() {
            Adt2::C0(_, _, _, _, _, _, _, _, _, _, h645) => {
                h645
            }
        };
        nat_eq(a646, a647)
    } else {
        false
    } {
        let a650 = match v0.clone() {
            Adt2::C0(_, _, _, _, _, h648, _, _, _, _, _) => {
                h648
            }
        };
        let a651 = match v1.clone() {
            Adt2::C0(_, _, _, _, _, h649, _, _, _, _, _) => {
                h649
            }
        };
        nat_eq(a650, a651)
    } else {
        false
    } {
        let a654 = match v0.clone() {
            Adt2::C0(_, _, _, _, _, _, h652, _, _, _, _) => {
                h652
            }
        };
        let a655 = match v1.clone() {
            Adt2::C0(_, _, _, _, _, _, h653, _, _, _, _) => {
                h653
            }
        };
        nat_eq(a654, a655)
    } else {
        false
    } {
        let a658 = match v0.clone() {
            Adt2::C0(_, _, _, _, _, _, _, h656, _, _, _) => {
                h656
            }
        };
        let a659 = match v1.clone() {
            Adt2::C0(_, _, _, _, _, _, _, h657, _, _, _) => {
                h657
            }
        };
        nat_eq(a658, a659)
    } else {
        false
    } {
        let a662 = match v0 {
            Adt2::C0(_, _, _, _, _, _, _, _, h660, _, _) => {
                h660
            }
        };
        let a663 = match v1 {
            Adt2::C0(_, _, _, _, _, _, _, _, h661, _, _) => {
                h661
            }
        };
        nat_eq(a662, a663)
    } else {
        false
    }
}

pub fn f27(v0: u64, v1: u64, v2: u64, v3: u64) -> bool {
    let a664 = v0;
    let a665 = 0u64;
    if if if nat_eq(a664, a665) {
        true
    } else {
        let a666 = v1;
        let a667 = 0u64;
        nat_eq(a666, a667)
    } {
        true
    } else {
        let a668 = v2;
        let a669 = 0u64;
        nat_eq(a668, a669)
    } {
        true
    } else {
        let a670 = v3;
        let a671 = 0u64;
        nat_eq(a670, a671)
    }
}

pub fn f28(v0: Adt2, v1: u64) -> R<bool> {
    f79({
        let a673 = match v0.clone() {
            Adt2::C0(_, h672, _, _, _, _, _, _, _, _, _) => {
                h672
            }
        };
        let a674 = v1;
        nat_mul(a673, a674)?
    }, match v0.clone() {
        Adt2::C0(_, _, h675, _, _, _, _, _, _, _, _) => {
            h675
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, h676, _, _, _, _, _, _, _) => {
            h676
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, h677, _, _, _, _, _, _) => {
            h677
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, _, _, h678, _, _, _, _) => {
            h678
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, _, _, _, h679, _, _, _) => {
            h679
        }
    }, match v0 {
        Adt2::C0(_, _, _, _, _, _, _, _, h680, _, _) => {
            h680
        }
    })
}

pub fn f29(v0: u64, v1: u64, v2: u64, v3: u64) -> R<u64> {
    let a681 = v1;
    let a682 = 0u64;
    if if nat_eq(a681, a682) {
        true
    } else {
        let a683 = v2;
        let a684 = 0u64;
        nat_eq(a683, a684)
    } {
        Ok(0u64)
    } else {
        let a691 = {
            let a685 = f80(v1, v2, v3)?;
            let a686 = f81(v0, v1, v2)?;
            nat_add(a685, a686)?
        };
        let a692 = {
            let a689 = f82(v0);
            let a690 = {
                let a687 = v1;
                let a688 = v2;
                nat_mul(a687, a688)?
            };
            nat_mul(a689, a690)?
        };
        nat_add(a691, a692)
    }
}

pub fn f30(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let a693 = v0;
    let a694 = 0u64;
    if if nat_eq(a693, a694) {
        true
    } else {
        let a695 = v1;
        let a696 = 0u64;
        nat_eq(a695, a696)
    } {
        Ok(3u64)
    } else {
        let a699 = {
            let a697 = 3u64;
            let a698 = v1;
            nat_add(a697, a698)?
        };
        let a700 = v2;
        nat_add(a699, a700)
    }
}

pub fn f31(v0: u64) -> bool {
    let a701 = v0;
    let a702 = 2139095040u64;
    if nat_le(a701, a702) {
        true
    } else {
        let a703 = v0;
        let a704 = 2147483648u64;
        nat_eq(a703, a704)
    }
}

pub fn f32(v0: u64, v1: u64) -> R<u64> {
    let a705 = v0;
    let a706 = 0u64;
    if nat_eq(a705, a706) {
        Ok(0u64)
    } else {
        let a707 = v1;
        let a708 = f83(v0)?;
        nat_mul(a707, a708)
    }
}

pub fn f33(v0: u64, v1: u64, v2: u64, v3: u64) -> R<u64> {
    let a709 = v0;
    let a710 = 0u64;
    if if if if nat_eq(a709, a710) {
        true
    } else {
        let a711 = v1;
        let a712 = 0u64;
        nat_eq(a711, a712)
    } {
        true
    } else {
        let a713 = v2;
        let a714 = 0u64;
        nat_eq(a713, a714)
    } {
        true
    } else {
        let a715 = v3;
        let a716 = 0u64;
        nat_eq(a715, a716)
    } {
        Ok(0u64)
    } else {
        let a729 = {
            let a719 = 2u64;
            let a720 = {
                let a717 = v1;
                let a718 = v2;
                nat_mul(a717, a718)?
            };
            nat_mul(a719, a720)?
        };
        let a730 = {
            let a727 = {
                let a723 = {
                    let a721 = v1;
                    let a722 = v2;
                    nat_mul(a721, a722)?
                };
                let a724 = v3;
                nat_mul(a723, a724)?
            };
            let a728 = {
                let a725 = 1u64;
                let a726 = v0;
                nat_add(a725, a726)?
            };
            nat_mul(a727, a728)?
        };
        nat_add(a729, a730)
    }
}

pub fn f34(v0: bool, v1: Adt1, v2: Adt2, v3: Adt2) -> R<Adt10> {
    let v4: Adt6 = Adt6::C0(match v1.clone() {
        Adt1::C0(_, h731, _, _, _, _, _, _, _, _) => {
            h731
        }
    }, if v0 {
        match v1.clone() {
            Adt1::C0(_, _, _, h732, _, _, _, _, _, _) => {
                h732
            }
        }
    } else {
        match v1.clone() {
            Adt1::C0(_, _, h733, _, _, _, _, _, _, _) => {
                h733
            }
        }
    }, if v0 {
        match v1.clone() {
            Adt1::C0(_, _, _, _, _, h734, _, _, _, _) => {
                h734
            }
        }
    } else {
        match v1.clone() {
            Adt1::C0(_, _, _, h735, _, _, _, _, _, _) => {
                h735
            }
        }
    }, 1u64);
    let v5: Adt6 = if v0 {
        Adt6::C0(match v1.clone() {
            Adt1::C0(_, _, h736, _, _, _, _, _, _, _) => {
                h736
            }
        }, match v1.clone() {
            Adt1::C0(_, _, _, _, h737, _, _, _, _, _) => {
                h737
            }
        }, match v1.clone() {
            Adt1::C0(_, _, _, _, _, _, h738, _, _, _) => {
                h738
            }
        }, 1u64)
    } else {
        Adt6::C0(1u64, 1u64, 0u64, 1u64)
    };
    let a740 = match v1.clone() {
        Adt1::C0(h739, _, _, _, _, _, _, _, _, _) => {
            h739
        }
    };
    let a741 = 1u64;
    if if if if if if if if if if if nat_le(a740, a741) {
        let a743 = match v2.clone() {
            Adt2::C0(h742, _, _, _, _, _, _, _, _, _, _) => {
                h742
            }
        };
        let a744 = 1u64;
        nat_le(a743, a744)
    } else {
        false
    } {
        let a746 = match v3.clone() {
            Adt2::C0(h745, _, _, _, _, _, _, _, _, _, _) => {
                h745
            }
        };
        let a747 = 0u64;
        nat_eq(a746, a747)
    } else {
        false
    } {
        let a752 = match v2.clone() {
            Adt2::C0(_, _, _, _, _, h748, _, _, _, _, _) => {
                h748
            }
        };
        let a753 = {
            let a750 = match v2.clone() {
                Adt2::C0(h749, _, _, _, _, _, _, _, _, _, _) => {
                    h749
                }
            };
            let a751 = 0u64;
            if nat_eq(a750, a751) {
                4u64
            } else {
                2u64
            }
        };
        nat_eq(a752, a753)
    } else {
        false
    } {
        f20(v3.clone())?
    } else {
        false
    } {
        f18(v2.clone(), v3.clone())?
    } else {
        false
    } {
        f84(v4.clone(), match v2.clone() {
            Adt2::C0(_, h754, _, _, _, _, _, _, _, _, _) => {
                h754
            }
        }, match v3.clone() {
            Adt2::C0(_, h755, _, _, _, _, _, _, _, _, _) => {
                h755
            }
        })?
    } else {
        false
    } {
        if v0 {
            f84(v5.clone(), match v2.clone() {
                Adt2::C0(_, _, h756, _, _, _, _, _, _, _, _) => {
                    h756
                }
            }, match v3.clone() {
                Adt2::C0(_, _, h757, _, _, _, _, _, _, _, _) => {
                    h757
                }
            })?
        } else {
            let a760 = match v2.clone() {
                Adt2::C0(_, _, h758, _, _, _, _, _, _, _, _) => {
                    h758
                }
            };
            let a761 = match v3.clone() {
                Adt2::C0(_, _, h759, _, _, _, _, _, _, _, _) => {
                    h759
                }
            };
            nat_eq(a760, a761)
        }
    } else {
        false
    } {
        let a764 = match v2.clone() {
            Adt2::C0(_, _, _, h762, _, _, _, _, _, _, _) => {
                h762
            }
        };
        let a765 = match v3.clone() {
            Adt2::C0(_, _, _, h763, _, _, _, _, _, _, _) => {
                h763
            }
        };
        nat_eq(a764, a765)
    } else {
        false
    } {
        let a768 = match v2.clone() {
            Adt2::C0(_, _, _, _, h766, _, _, _, _, _, _) => {
                h766
            }
        };
        let a769 = match v3.clone() {
            Adt2::C0(_, _, _, _, h767, _, _, _, _, _, _) => {
                h767
            }
        };
        nat_eq(a768, a769)
    } else {
        false
    } {
        let a774 = {
            let a772 = match v4.clone() {
                Adt6::C0(h770, _, _, _) => {
                    h770
                }
            };
            let a773 = match v5.clone() {
                Adt6::C0(h771, _, _, _) => {
                    h771
                }
            };
            nat_mul(a772, a773)?
        };
        let a775 = 2147483648u64;
        nat_lt(a774, a775)
    } else {
        false
    } {
        let v6: Adt7 = Adt7::C0(v4, v5, v0, {
            let a777 = match v1 {
                Adt1::C0(h776, _, _, _, _, _, _, _, _, _) => {
                    h776
                }
            };
            let a778 = 0u64;
            nat_eq(a777, a778)
        }, match v2.clone() {
            Adt2::C0(h779, _, _, _, _, _, _, _, _, _, _) => {
                h779
            }
        });
        Ok(Adt10::C4({
            let a780 = f12(v3.clone(), 1u64)?;
            let a781 = f85(v6.clone())?;
            nat_mul(a780, a781)?
        }, 4u64, f16(v2, 1u64), f16(v3, 1u64), Adt4::C28(v6)))
    } else {
        Ok(Adt10::C1)
    }
}

pub fn f35(v0: Adt1, v1: Adt2, v2: Option<Adt2>, v3: Adt2) -> R<Adt10> {
    let m782 = v2;
    match m782 {
        None => {
            Ok(Adt10::C1)
        }
        Some(v4) => {
            let v5: bool = {
                let a784 = match v0.clone() {
                    Adt1::C0(_, _, _, _, _, _, h783, _, _, _) => {
                        h783
                    }
                };
                let a785 = 1u64;
                nat_eq(a784, a785)
            };
            let v6: Adt6 = Adt6::C0(match v1.clone() {
                Adt2::C0(_, h786, _, _, _, _, _, _, _, _, _) => {
                    h786
                }
            }, match v0.clone() {
                Adt1::C0(h787, _, _, _, _, _, _, _, _, _) => {
                    h787
                }
            }, match v0.clone() {
                Adt1::C0(_, _, h788, _, _, _, _, _, _, _) => {
                    h788
                }
            }, match v0.clone() {
                Adt1::C0(_, _, _, _, h789, _, _, _, _, _) => {
                    h789
                }
            });
            let v7: Adt6 = if v5 {
                Adt6::C0(match v1.clone() {
                    Adt2::C0(_, _, h790, _, _, _, _, _, _, _, _) => {
                        h790
                    }
                }, match v0.clone() {
                    Adt1::C0(_, h791, _, _, _, _, _, _, _, _) => {
                        h791
                    }
                }, match v0.clone() {
                    Adt1::C0(_, _, _, h792, _, _, _, _, _, _) => {
                        h792
                    }
                }, match v0.clone() {
                    Adt1::C0(_, _, _, _, _, h793, _, _, _, _) => {
                        h793
                    }
                })
            } else {
                Adt6::C0(1u64, 1u64, 0u64, 1u64)
            };
            let v8: Adt6 = Adt6::C0(1u64, 1u64, 0u64, 1u64);
            let v9: u64 = if v5 {
                match v4.clone() {
                    Adt2::C0(_, _, _, h794, _, _, _, _, _, _, _) => {
                        h794
                    }
                }
            } else {
                match v4.clone() {
                    Adt2::C0(_, _, h795, _, _, _, _, _, _, _, _) => {
                        h795
                    }
                }
            };
            let v10: u64 = if v5 {
                match v4.clone() {
                    Adt2::C0(_, _, _, _, h796, _, _, _, _, _, _) => {
                        h796
                    }
                }
            } else {
                match v4.clone() {
                    Adt2::C0(_, _, _, h797, _, _, _, _, _, _, _) => {
                        h797
                    }
                }
            };
            let v11: u64 = if v5 {
                match v4.clone() {
                    Adt2::C0(_, _, h798, _, _, _, _, _, _, _, _) => {
                        h798
                    }
                }
            } else {
                1u64
            };
            let v12: u64 = match v4.clone() {
                Adt2::C0(h799, _, _, _, _, _, _, _, _, _, _) => {
                    h799
                }
            };
            let v13: u64 = match v3.clone() {
                Adt2::C0(h800, _, _, _, _, _, _, _, _, _, _) => {
                    h800
                }
            };
            let v14: u64 = {
                let a801 = v12;
                let a802 = 0u64;
                if nat_eq(a801, a802) {
                    4u64
                } else {
                    2u64
                }
            };
            let v15: u64 = {
                let a804 = match v4.clone() {
                    Adt2::C0(_, h803, _, _, _, _, _, _, _, _, _) => {
                        h803
                    }
                };
                let a805 = v14;
                nat_mul(a804, a805)?
            };
            let v16: u64 = if v5 {
                match v4.clone() {
                    Adt2::C0(_, _, _, _, _, _, _, h806, _, _, _) => {
                        h806
                    }
                }
            } else {
                match v4.clone() {
                    Adt2::C0(_, _, _, _, _, _, h807, _, _, _, _) => {
                        h807
                    }
                }
            };
            let v17: u64 = if v5 {
                match v4.clone() {
                    Adt2::C0(_, _, _, _, _, _, _, _, h808, _, _) => {
                        h808
                    }
                }
            } else {
                match v4.clone() {
                    Adt2::C0(_, _, _, _, _, _, _, h809, _, _, _) => {
                        h809
                    }
                }
            };
            let a811 = match v0 {
                Adt1::C0(_, _, _, _, _, _, h810, _, _, _) => {
                    h810
                }
            };
            let a812 = 1u64;
            if if if if if if if if if if nat_le(a811, a812) {
                let a813 = v12;
                let a814 = 0u64;
                if nat_eq(a813, a814) {
                    true
                } else {
                    let a815 = v12;
                    let a816 = 1u64;
                    if nat_eq(a815, a816) {
                        let a817 = v13;
                        let a818 = 1u64;
                        nat_eq(a817, a818)
                    } else {
                        false
                    }
                }
            } else {
                false
            } {
                let a819 = v13;
                let a820 = 0u64;
                if nat_eq(a819, a820) {
                    true
                } else {
                    let a821 = v13;
                    let a822 = 1u64;
                    nat_eq(a821, a822)
                }
            } else {
                false
            } {
                let a824 = match v4.clone() {
                    Adt2::C0(_, _, _, _, _, h823, _, _, _, _, _) => {
                        h823
                    }
                };
                let a825 = v14;
                nat_eq(a824, a825)
            } else {
                false
            } {
                let a833 = match v3.clone() {
                    Adt2::C0(_, h826, _, _, _, _, _, _, _, _, _) => {
                        h826
                    }
                };
                let a834 = {
                    let a831 = v9;
                    let a832 = {
                        let a829 = match v6.clone() {
                            Adt6::C0(h827, _, _, _) => {
                                h827
                            }
                        };
                        let a830 = match v7.clone() {
                            Adt6::C0(h828, _, _, _) => {
                                h828
                            }
                        };
                        nat_mul(a829, a830)?
                    };
                    nat_mul(a831, a832)?
                };
                nat_eq(a833, a834)
            } else {
                false
            } {
                let a837 = match v3.clone() {
                    Adt2::C0(_, _, h835, _, _, _, _, _, _, _, _) => {
                        h835
                    }
                };
                let a838 = f17(v6.clone(), match v4.clone() {
                    Adt2::C0(_, h836, _, _, _, _, _, _, _, _, _) => {
                        h836
                    }
                })?;
                nat_eq(a837, a838)
            } else {
                false
            } {
                let a840 = 0u64;
                let a841 = match v3.clone() {
                    Adt2::C0(_, _, h839, _, _, _, _, _, _, _, _) => {
                        h839
                    }
                };
                nat_lt(a840, a841)
            } else {
                false
            } {
                if v5 {
                    let a843 = match v1 {
                        Adt2::C0(_, _, _, h842, _, _, _, _, _, _, _) => {
                            h842
                        }
                    };
                    let a844 = v9;
                    if if if nat_eq(a843, a844) {
                        let a846 = match v3.clone() {
                            Adt2::C0(_, _, _, h845, _, _, _, _, _, _, _) => {
                                h845
                            }
                        };
                        let a847 = f17(v7.clone(), v11)?;
                        nat_eq(a846, a847)
                    } else {
                        false
                    } {
                        let a849 = match v3.clone() {
                            Adt2::C0(_, _, _, _, h848, _, _, _, _, _, _) => {
                                h848
                            }
                        };
                        let a850 = v10;
                        nat_eq(a849, a850)
                    } else {
                        false
                    } {
                        let a852 = 0u64;
                        let a853 = match v3.clone() {
                            Adt2::C0(_, _, _, h851, _, _, _, _, _, _, _) => {
                                h851
                            }
                        };
                        nat_lt(a852, a853)
                    } else {
                        false
                    }
                } else {
                    let a855 = match v1 {
                        Adt2::C0(_, _, h854, _, _, _, _, _, _, _, _) => {
                            h854
                        }
                    };
                    let a856 = v9;
                    if if if nat_eq(a855, a856) {
                        let a858 = match v4.clone() {
                            Adt2::C0(_, _, _, _, h857, _, _, _, _, _, _) => {
                                h857
                            }
                        };
                        let a859 = 1u64;
                        nat_eq(a858, a859)
                    } else {
                        false
                    } {
                        let a861 = match v3.clone() {
                            Adt2::C0(_, _, _, h860, _, _, _, _, _, _, _) => {
                                h860
                            }
                        };
                        let a862 = v10;
                        nat_eq(a861, a862)
                    } else {
                        false
                    } {
                        let a864 = match v3.clone() {
                            Adt2::C0(_, _, _, _, h863, _, _, _, _, _, _) => {
                                h863
                            }
                        };
                        let a865 = 1u64;
                        nat_eq(a864, a865)
                    } else {
                        false
                    }
                }
            } else {
                false
            } {
                if f27(match v4.clone() {
                    Adt2::C0(_, h866, _, _, _, _, _, _, _, _, _) => {
                        h866
                    }
                }, v11, v9, v10) {
                    true
                } else {
                    let a886 = {
                        let a883 = {
                            let a881 = {
                                let a875 = {
                                    let a869 = {
                                        let a867 = v10;
                                        let a868 = 1u64;
                                        nat_sub(a867, a868)
                                    };
                                    let a870 = v17;
                                    nat_mul(a869, a870)?
                                };
                                let a876 = {
                                    let a873 = {
                                        let a871 = v9;
                                        let a872 = 1u64;
                                        nat_sub(a871, a872)
                                    };
                                    let a874 = v16;
                                    nat_mul(a873, a874)?
                                };
                                nat_add(a875, a876)?
                            };
                            let a882 = {
                                let a879 = {
                                    let a877 = v11;
                                    let a878 = 1u64;
                                    nat_sub(a877, a878)
                                };
                                let a880 = v15;
                                nat_mul(a879, a880)?
                            };
                            nat_add(a881, a882)?
                        };
                        let a884 = v15;
                        nat_add(a883, a884)?
                    };
                    let a887 = match v4.clone() {
                        Adt2::C0(_, _, _, _, _, _, _, _, _, h885, _) => {
                            h885
                        }
                    };
                    nat_le(a886, a887)
                }
            } else {
                false
            } {
                f18(v4.clone(), v3.clone())?
            } else {
                false
            } {
                Ok(Adt10::C4(f12(v3.clone(), 1u64)?, {
                    let a888 = v13;
                    let a889 = 0u64;
                    if nat_eq(a888, a889) {
                        4u64
                    } else {
                        2u64
                    }
                }, Adt3::C0(match v4 {
                    Adt2::C0(_, h890, _, _, _, _, _, _, _, _, _) => {
                        h890
                    }
                }, v11, 1u64, v10, v14, v15, 0u64, 0u64), if v5 {
                    f16(v3, 1u64)
                } else {
                    Adt3::C0(match v3.clone() {
                        Adt2::C0(_, h891, _, _, _, _, _, _, _, _, _) => {
                            h891
                        }
                    }, match v3.clone() {
                        Adt2::C0(_, _, h892, _, _, _, _, _, _, _, _) => {
                            h892
                        }
                    }, 1u64, match v3.clone() {
                        Adt2::C0(_, _, _, h893, _, _, _, _, _, _, _) => {
                            h893
                        }
                    }, match v3.clone() {
                        Adt2::C0(_, _, _, _, _, h894, _, _, _, _, _) => {
                            h894
                        }
                    }, match v3.clone() {
                        Adt2::C0(_, _, _, _, _, _, h895, _, _, _, _) => {
                            h895
                        }
                    }, 0u64, match v3 {
                        Adt2::C0(_, _, _, _, _, _, _, h896, _, _, _) => {
                            h896
                        }
                    })
                }, Adt4::C27(Adt5::C0(v6, v7, v8, v16, v17, v12, 1u64, v13))))
            } else {
                Ok(Adt10::C1)
            }
        }
    }
}

pub fn f36(v0: Adt1, v1: Adt2, v2: Option<Adt2>, v3: Adt2) -> R<Adt10> {
    let m897 = v2;
    match m897 {
        None => {
            Ok(Adt10::C1)
        }
        Some(v4) => {
            let v5: Adt6 = Adt6::C0(match v1.clone() {
                Adt2::C0(_, h898, _, _, _, _, _, _, _, _, _) => {
                    h898
                }
            }, match v0.clone() {
                Adt1::C0(h899, _, _, _, _, _, _, _, _, _) => {
                    h899
                }
            }, match v0.clone() {
                Adt1::C0(_, _, _, h900, _, _, _, _, _, _) => {
                    h900
                }
            }, match v0.clone() {
                Adt1::C0(_, _, _, _, _, _, h901, _, _, _) => {
                    h901
                }
            });
            let v6: Adt6 = Adt6::C0(match v1.clone() {
                Adt2::C0(_, _, h902, _, _, _, _, _, _, _, _) => {
                    h902
                }
            }, match v0.clone() {
                Adt1::C0(_, h903, _, _, _, _, _, _, _, _) => {
                    h903
                }
            }, match v0.clone() {
                Adt1::C0(_, _, _, _, h904, _, _, _, _, _) => {
                    h904
                }
            }, match v0.clone() {
                Adt1::C0(_, _, _, _, _, _, _, h905, _, _) => {
                    h905
                }
            });
            let v7: Adt6 = Adt6::C0(match v1 {
                Adt2::C0(_, _, _, h906, _, _, _, _, _, _, _) => {
                    h906
                }
            }, match v0.clone() {
                Adt1::C0(_, _, h907, _, _, _, _, _, _, _) => {
                    h907
                }
            }, match v0.clone() {
                Adt1::C0(_, _, _, _, _, h908, _, _, _, _) => {
                    h908
                }
            }, match v0.clone() {
                Adt1::C0(_, _, _, _, _, _, _, _, h909, _) => {
                    h909
                }
            });
            let v8: u64 = match v0 {
                Adt1::C0(_, _, _, _, _, _, _, _, _, h910) => {
                    h910
                }
            };
            let v9: u64 = f17(v7.clone(), match v4.clone() {
                Adt2::C0(_, _, _, h911, _, _, _, _, _, _, _) => {
                    h911
                }
            })?;
            let v10: u64 = match v3.clone() {
                Adt2::C0(h912, _, _, _, _, _, _, _, _, _, _) => {
                    h912
                }
            };
            let v11: u64 = {
                let a920 = v8;
                let a921 = {
                    let a918 = match v5.clone() {
                        Adt6::C0(h913, _, _, _) => {
                            h913
                        }
                    };
                    let a919 = {
                        let a916 = match v6.clone() {
                            Adt6::C0(h914, _, _, _) => {
                                h914
                            }
                        };
                        let a917 = match v7.clone() {
                            Adt6::C0(h915, _, _, _) => {
                                h915
                            }
                        };
                        nat_mul(a916, a917)?
                    };
                    nat_mul(a918, a919)?
                };
                nat_mul(a920, a921)?
            };
            let a923 = match v4.clone() {
                Adt2::C0(h922, _, _, _, _, _, _, _, _, _, _) => {
                    h922
                }
            };
            let a924 = 0u64;
            if if if if if if if if if if if if if nat_eq(a923, a924) {
                let a925 = v10;
                let a926 = 0u64;
                if nat_eq(a925, a926) {
                    true
                } else {
                    let a927 = v10;
                    let a928 = 1u64;
                    nat_eq(a927, a928)
                }
            } else {
                false
            } {
                let a929 = 0u64;
                let a930 = v8;
                nat_lt(a929, a930)
            } else {
                false
            } {
                let a931 = v8;
                let a932 = 2147483648u64;
                nat_lt(a931, a932)
            } else {
                false
            } {
                let a934 = match v4.clone() {
                    Adt2::C0(_, _, _, _, _, h933, _, _, _, _, _) => {
                        h933
                    }
                };
                let a935 = 4u64;
                nat_eq(a934, a935)
            } else {
                false
            } {
                let a936 = 0u64;
                let a937 = v9;
                nat_lt(a936, a937)
            } else {
                false
            } {
                let a939 = 0u64;
                let a940 = match v3.clone() {
                    Adt2::C0(_, _, h938, _, _, _, _, _, _, _, _) => {
                        h938
                    }
                };
                nat_lt(a939, a940)
            } else {
                false
            } {
                let a942 = 0u64;
                let a943 = match v3.clone() {
                    Adt2::C0(_, _, _, h941, _, _, _, _, _, _, _) => {
                        h941
                    }
                };
                nat_lt(a942, a943)
            } else {
                false
            } {
                let a945 = match v3.clone() {
                    Adt2::C0(_, h944, _, _, _, _, _, _, _, _, _) => {
                        h944
                    }
                };
                let a946 = v11;
                nat_eq(a945, a946)
            } else {
                false
            } {
                let a949 = match v3.clone() {
                    Adt2::C0(_, _, h947, _, _, _, _, _, _, _, _) => {
                        h947
                    }
                };
                let a950 = f17(v5.clone(), match v4.clone() {
                    Adt2::C0(_, h948, _, _, _, _, _, _, _, _, _) => {
                        h948
                    }
                })?;
                nat_eq(a949, a950)
            } else {
                false
            } {
                let a953 = match v3.clone() {
                    Adt2::C0(_, _, _, h951, _, _, _, _, _, _, _) => {
                        h951
                    }
                };
                let a954 = f17(v6.clone(), match v4.clone() {
                    Adt2::C0(_, _, h952, _, _, _, _, _, _, _, _) => {
                        h952
                    }
                })?;
                nat_eq(a953, a954)
            } else {
                false
            } {
                let a962 = match v3.clone() {
                    Adt2::C0(_, _, _, _, h955, _, _, _, _, _, _) => {
                        h955
                    }
                };
                let a963 = {
                    let a960 = v9;
                    let a961 = {
                        let a957 = match v4.clone() {
                            Adt2::C0(_, _, _, _, h956, _, _, _, _, _, _) => {
                                h956
                            }
                        };
                        let a958 = v8;
                        let a959 = 0u64;
                        nat_quot(a957, a958, a959)
                    };
                    nat_mul(a960, a961)?
                };
                nat_eq(a962, a963)
            } else {
                false
            } {
                f18(v4.clone(), v3.clone())?
            } else {
                false
            } {
                Ok(Adt10::C4(f12(v3.clone(), 1u64)?, {
                    let a964 = v10;
                    let a965 = 0u64;
                    if nat_eq(a964, a965) {
                        4u64
                    } else {
                        2u64
                    }
                }, f16(v4.clone(), 1u64), f16(v3, 1u64), Adt4::C27(Adt5::C0(v5, v6, v7, match v4.clone() {
                    Adt2::C0(_, _, _, _, _, _, _, _, h966, _, _) => {
                        h966
                    }
                }, {
                    let a968 = v8;
                    let a969 = match v4 {
                        Adt2::C0(_, _, _, _, _, _, _, _, h967, _, _) => {
                            h967
                        }
                    };
                    nat_mul(a968, a969)?
                }, 0u64, v9, v10))))
            } else {
                Ok(Adt10::C1)
            }
        }
    }
}

pub fn f37(v0: Adt2, v1: Option<Adt2>, v2: Adt2) -> R<Adt10> {
    let m970 = v1;
    match m970 {
        None => {
            Ok(Adt10::C1)
        }
        Some(v3) => {
            f86(v0, v3, v2)
        }
    }
}

pub fn f38(v0: Adt2, v1: Adt2) -> R<Adt10> {
    let a972 = match v0.clone() {
        Adt2::C0(h971, _, _, _, _, _, _, _, _, _, _) => {
            h971
        }
    };
    let a973 = 1u64;
    if if nat_eq(a972, a973) {
        true
    } else {
        let a975 = match v0.clone() {
            Adt2::C0(h974, _, _, _, _, _, _, _, _, _, _) => {
                h974
            }
        };
        let a976 = 30u64;
        nat_eq(a975, a976)
    } {
        let a978 = match v1.clone() {
            Adt2::C0(h977, _, _, _, _, _, _, _, _, _, _) => {
                h977
            }
        };
        let a979 = 1u64;
        if if if if if if if if if if if nat_eq(a978, a979) {
            let a982 = match v0.clone() {
                Adt2::C0(_, h980, _, _, _, _, _, _, _, _, _) => {
                    h980
                }
            };
            let a983 = match v1.clone() {
                Adt2::C0(_, h981, _, _, _, _, _, _, _, _, _) => {
                    h981
                }
            };
            nat_eq(a982, a983)
        } else {
            false
        } {
            let a985 = 0u64;
            let a986 = match v1.clone() {
                Adt2::C0(_, _, h984, _, _, _, _, _, _, _, _) => {
                    h984
                }
            };
            nat_lt(a985, a986)
        } else {
            false
        } {
            let a989 = match v1.clone() {
                Adt2::C0(_, _, h987, _, _, _, _, _, _, _, _) => {
                    h987
                }
            };
            let a990 = match v1.clone() {
                Adt2::C0(_, _, _, h988, _, _, _, _, _, _, _) => {
                    h988
                }
            };
            nat_eq(a989, a990)
        } else {
            false
        } {
            let a992 = match v1.clone() {
                Adt2::C0(_, _, _, _, h991, _, _, _, _, _, _) => {
                    h991
                }
            };
            let a993 = 1u64;
            nat_eq(a992, a993)
        } else {
            false
        } {
            let a1001 = match v0.clone() {
                Adt2::C0(_, _, h994, _, _, _, _, _, _, _, _) => {
                    h994
                }
            };
            let a1002 = {
                let a999 = {
                    let a997 = match v1.clone() {
                        Adt2::C0(_, _, h995, _, _, _, _, _, _, _, _) => {
                            h995
                        }
                    };
                    let a998 = match v1.clone() {
                        Adt2::C0(_, _, h996, _, _, _, _, _, _, _, _) => {
                            h996
                        }
                    };
                    nat_add(a997, a998)?
                };
                let a1000 = 1u64;
                nat_sub(a999, a1000)
            };
            nat_eq(a1001, a1002)
        } else {
            false
        } {
            let a1004 = 0u64;
            let a1005 = match v0.clone() {
                Adt2::C0(_, _, _, h1003, _, _, _, _, _, _, _) => {
                    h1003
                }
            };
            nat_lt(a1004, a1005)
        } else {
            false
        } {
            let a1007 = 0u64;
            let a1008 = match v0.clone() {
                Adt2::C0(_, _, _, _, h1006, _, _, _, _, _, _) => {
                    h1006
                }
            };
            nat_lt(a1007, a1008)
        } else {
            false
        } {
            let a1010 = 2u64;
            let a1011 = match v0.clone() {
                Adt2::C0(_, _, _, _, _, h1009, _, _, _, _, _) => {
                    h1009
                }
            };
            nat_le(a1010, a1011)
        } else {
            false
        } {
            let a1013 = 2u64;
            let a1014 = match v1.clone() {
                Adt2::C0(_, _, _, _, _, h1012, _, _, _, _, _) => {
                    h1012
                }
            };
            nat_le(a1013, a1014)
        } else {
            false
        } {
            f18(v0.clone(), v1.clone())?
        } else {
            false
        } {
            Ok(Adt10::C4(f12(v1.clone(), 1u64)?, 2u64, f16(v0, 1u64), f16(v1, 1u64), Adt4::C21))
        } else {
            Ok(Adt10::C1)
        }
    } else {
        Ok(Adt10::C0)
    }
}

pub fn f39(v0: Adt2, v1: Option<Adt2>, v2: Adt2) -> R<Adt10> {
    let m1015 = v1;
    match m1015 {
        None => {
            Ok(Adt10::C1)
        }
        Some(v3) => {
            f87(v0, v3, v2)
        }
    }
}

pub fn f40(v0: u64) -> bool {
    let a1016 = v0;
    let a1017 = 1u64;
    if if nat_eq(a1016, a1017) {
        true
    } else {
        let a1018 = v0;
        let a1019 = 33u64;
        nat_eq(a1018, a1019)
    } {
        true
    } else {
        let a1020 = v0;
        let a1021 = 34u64;
        nat_eq(a1020, a1021)
    }
}

pub fn f41(v0: u64, v1: Adt2, v2: bool, v3: Adt2) -> R<Adt10> {
    let a1024 = match v1.clone() {
        Adt2::C0(h1022, _, _, _, _, _, _, _, _, _, _) => {
            h1022
        }
    };
    let a1025 = match v3.clone() {
        Adt2::C0(h1023, _, _, _, _, _, _, _, _, _, _) => {
            h1023
        }
    };
    if nat_eq(a1024, a1025) {
        f88(v0, v1, v2, v3)
    } else {
        if if if if f89(match v1.clone() {
            Adt2::C0(h1026, _, _, _, _, _, _, _, _, _, _) => {
                h1026
            }
        }) {
            f89(match v3.clone() {
                Adt2::C0(h1027, _, _, _, _, _, _, _, _, _, _) => {
                    h1027
                }
            })
        } else {
            false
        } {
            let a1028 = v0;
            let a1029 = 33u64;
            if nat_eq(a1028, a1029) {
                v2
            } else {
                true
            }
        } else {
            false
        } {
            let a1030 = f12(v1.clone(), 1u64)?;
            let a1031 = f12(v3.clone(), 1u64)?;
            nat_eq(a1030, a1031)
        } else {
            false
        } {
            let m1033 = f90(match v3.clone() {
                Adt2::C0(h1032, _, _, _, _, _, _, _, _, _, _) => {
                    h1032
                }
            });
            match m1033 {
                Adt14::C0 => {
                    Ok(Adt10::C0)
                }
                Adt14::C1(v4) => {
                    Ok(Adt10::C4(f12(v3.clone(), 1u64)?, match v4 {
                        Adt15::C0(_, _, h1034) => {
                            h1034
                        }
                    }, f16(v1.clone(), 1u64), f16(v3.clone(), 1u64), Adt4::C11(match v1 {
                        Adt2::C0(h1035, _, _, _, _, _, _, _, _, _, _) => {
                            h1035
                        }
                    }, match v3 {
                        Adt2::C0(h1036, _, _, _, _, _, _, _, _, _, _) => {
                            h1036
                        }
                    })))
                }
            }
        } else {
            Ok(Adt10::C1)
        }
    }
}

pub fn f42(v0: Adt2, v1: Adt2) -> R<Adt10> {
    let a1038 = match v0.clone() {
        Adt2::C0(h1037, _, _, _, _, _, _, _, _, _, _) => {
            h1037
        }
    };
    let a1039 = 0u64;
    if if if if nat_eq(a1038, a1039) {
        true
    } else {
        let a1041 = match v0.clone() {
            Adt2::C0(h1040, _, _, _, _, _, _, _, _, _, _) => {
                h1040
            }
        };
        let a1042 = 1u64;
        nat_eq(a1041, a1042)
    } {
        true
    } else {
        let a1044 = match v0.clone() {
            Adt2::C0(h1043, _, _, _, _, _, _, _, _, _, _) => {
                h1043
            }
        };
        let a1045 = 25u64;
        if nat_eq(a1044, a1045) {
            true
        } else {
            let a1047 = match v0.clone() {
                Adt2::C0(h1046, _, _, _, _, _, _, _, _, _, _) => {
                    h1046
                }
            };
            let a1048 = 26u64;
            nat_eq(a1047, a1048)
        }
    } {
        true
    } else {
        let a1050 = match v0.clone() {
            Adt2::C0(h1049, _, _, _, _, _, _, _, _, _, _) => {
                h1049
            }
        };
        let a1051 = 30u64;
        nat_eq(a1050, a1051)
    } {
        let a1054 = match v0.clone() {
            Adt2::C0(h1052, _, _, _, _, _, _, _, _, _, _) => {
                h1052
            }
        };
        let a1055 = match v1.clone() {
            Adt2::C0(h1053, _, _, _, _, _, _, _, _, _, _) => {
                h1053
            }
        };
        if if nat_eq(a1054, a1055) {
            f91(match v1.clone() {
                Adt2::C0(_, h1056, _, _, _, _, _, _, _, _, _) => {
                    h1056
                }
            }, match v1.clone() {
                Adt2::C0(_, _, h1057, _, _, _, _, _, _, _, _) => {
                    h1057
                }
            }, match v1.clone() {
                Adt2::C0(_, _, _, h1058, _, _, _, _, _, _, _) => {
                    h1058
                }
            }, match v1.clone() {
                Adt2::C0(_, _, _, _, h1059, _, _, _, _, _, _) => {
                    h1059
                }
            }, match v0.clone() {
                Adt2::C0(_, h1060, _, _, _, _, _, _, _, _, _) => {
                    h1060
                }
            }, match v0.clone() {
                Adt2::C0(_, _, h1061, _, _, _, _, _, _, _, _) => {
                    h1061
                }
            }, match v0.clone() {
                Adt2::C0(_, _, _, h1062, _, _, _, _, _, _, _) => {
                    h1062
                }
            }, match v0.clone() {
                Adt2::C0(_, _, _, _, h1063, _, _, _, _, _, _) => {
                    h1063
                }
            })
        } else {
            false
        } {
            let m1065 = f90(match v0.clone() {
                Adt2::C0(h1064, _, _, _, _, _, _, _, _, _, _) => {
                    h1064
                }
            });
            match m1065 {
                Adt14::C0 => {
                    Ok(Adt10::C0)
                }
                Adt14::C1(v2) => {
                    Ok(Adt10::C4(f12(v1.clone(), 1u64)?, match v2 {
                        Adt15::C0(_, _, h1066) => {
                            h1066
                        }
                    }, f16(v0, 1u64), f16(v1, 1u64), Adt4::C1))
                }
            }
        } else {
            Ok(Adt10::C1)
        }
    } else {
        Ok(Adt10::C0)
    }
}

pub fn f43(v0: u64, v1: Adt2, v2: Option<Adt2>, v3: Adt2) -> R<Adt10> {
    let m1067 = v2;
    match m1067 {
        None => {
            Ok(Adt10::C1)
        }
        Some(v4) => {
            let a1068 = v0;
            let a1069 = 4u64;
            if if if nat_lt(a1068, a1069) {
                let a1072 = match v1.clone() {
                    Adt2::C0(h1070, _, _, _, _, _, _, _, _, _, _) => {
                        h1070
                    }
                };
                let a1073 = match v4.clone() {
                    Adt2::C0(h1071, _, _, _, _, _, _, _, _, _, _) => {
                        h1071
                    }
                };
                if nat_eq(a1072, a1073) {
                    let a1076 = match v1.clone() {
                        Adt2::C0(h1074, _, _, _, _, _, _, _, _, _, _) => {
                            h1074
                        }
                    };
                    let a1077 = match v3.clone() {
                        Adt2::C0(h1075, _, _, _, _, _, _, _, _, _, _) => {
                            h1075
                        }
                    };
                    nat_eq(a1076, a1077)
                } else {
                    false
                }
            } else {
                false
            } {
                if if if f92(v0, 0u64, match v1.clone() {
                    Adt2::C0(_, h1078, _, _, _, _, _, _, _, _, _) => {
                        h1078
                    }
                }, match v4.clone() {
                    Adt2::C0(_, h1079, _, _, _, _, _, _, _, _, _) => {
                        h1079
                    }
                }, match v3.clone() {
                    Adt2::C0(_, h1080, _, _, _, _, _, _, _, _, _) => {
                        h1080
                    }
                })? {
                    f92(v0, 1u64, match v1.clone() {
                        Adt2::C0(_, _, h1081, _, _, _, _, _, _, _, _) => {
                            h1081
                        }
                    }, match v4.clone() {
                        Adt2::C0(_, _, h1082, _, _, _, _, _, _, _, _) => {
                            h1082
                        }
                    }, match v3.clone() {
                        Adt2::C0(_, _, h1083, _, _, _, _, _, _, _, _) => {
                            h1083
                        }
                    })?
                } else {
                    false
                } {
                    f92(v0, 2u64, match v1.clone() {
                        Adt2::C0(_, _, _, h1084, _, _, _, _, _, _, _) => {
                            h1084
                        }
                    }, match v4.clone() {
                        Adt2::C0(_, _, _, h1085, _, _, _, _, _, _, _) => {
                            h1085
                        }
                    }, match v3.clone() {
                        Adt2::C0(_, _, _, h1086, _, _, _, _, _, _, _) => {
                            h1086
                        }
                    })?
                } else {
                    false
                } {
                    f92(v0, 3u64, match v1.clone() {
                        Adt2::C0(_, _, _, _, h1087, _, _, _, _, _, _) => {
                            h1087
                        }
                    }, match v4.clone() {
                        Adt2::C0(_, _, _, _, h1088, _, _, _, _, _, _) => {
                            h1088
                        }
                    }, match v3.clone() {
                        Adt2::C0(_, _, _, _, h1089, _, _, _, _, _, _) => {
                            h1089
                        }
                    })?
                } else {
                    false
                }
            } else {
                false
            } {
                let m1091 = f90(match v1.clone() {
                    Adt2::C0(h1090, _, _, _, _, _, _, _, _, _, _) => {
                        h1090
                    }
                });
                match m1091 {
                    Adt14::C0 => {
                        Ok(Adt10::C0)
                    }
                    Adt14::C1(v5) => {
                        let v6: u64 = match v5.clone() {
                            Adt15::C0(_, h1092, _) => {
                                h1092
                            }
                        };
                        let a1097 = {
                            let a1094 = match v1.clone() {
                                Adt2::C0(_, h1093, _, _, _, _, _, _, _, _, _) => {
                                    h1093
                                }
                            };
                            let a1095 = v6;
                            let a1096 = 1u64;
                            nat_rem(a1094, a1095, a1096)
                        };
                        let a1098 = 0u64;
                        if if if nat_eq(a1097, a1098) {
                            let a1103 = {
                                let a1100 = match v4.clone() {
                                    Adt2::C0(_, h1099, _, _, _, _, _, _, _, _, _) => {
                                        h1099
                                    }
                                };
                                let a1101 = v6;
                                let a1102 = 1u64;
                                nat_rem(a1100, a1101, a1102)
                            };
                            let a1104 = 0u64;
                            nat_eq(a1103, a1104)
                        } else {
                            false
                        } {
                            let a1109 = {
                                let a1106 = match v3.clone() {
                                    Adt2::C0(_, h1105, _, _, _, _, _, _, _, _, _) => {
                                        h1105
                                    }
                                };
                                let a1107 = v6;
                                let a1108 = 1u64;
                                nat_rem(a1106, a1107, a1108)
                            };
                            let a1110 = 0u64;
                            nat_eq(a1109, a1110)
                        } else {
                            false
                        } {
                            Ok(Adt10::C4(f12(v3.clone(), v6)?, match v5 {
                                Adt15::C0(_, _, h1111) => {
                                    h1111
                                }
                            }, f16(v1, v6), f16(v3, v6), Adt4::C2(v0, f16(v4, v6))))
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

pub fn f44(v0: u64, v1: Adt1, v2: Adt2, v3: Adt2) -> R<Adt10> {
    let a1113 = match v2.clone() {
        Adt2::C0(h1112, _, _, _, _, _, _, _, _, _, _) => {
            h1112
        }
    };
    let a1114 = 0u64;
    if if nat_eq(a1113, a1114) {
        let a1116 = match v3.clone() {
            Adt2::C0(h1115, _, _, _, _, _, _, _, _, _, _) => {
                h1115
            }
        };
        let a1117 = 0u64;
        nat_eq(a1116, a1117)
    } else {
        false
    } {
        let a1118 = v0;
        let a1119 = 61u64;
        if nat_eq(a1118, a1119) {
            let v4: bool = {
                let a1121 = 0u64;
                let a1122 = match v1.clone() {
                    Adt1::C0(_, _, _, _, _, _, _, _, h1120, _) => {
                        h1120
                    }
                };
                nat_lt(a1121, a1122)
            };
            if if if if if if f93(match v2.clone() {
                Adt2::C0(_, h1123, _, _, _, _, _, _, _, _, _) => {
                    h1123
                }
            }, match v1.clone() {
                Adt1::C0(h1124, _, _, _, _, _, _, _, _, _) => {
                    h1124
                }
            }, match v1.clone() {
                Adt1::C0(_, h1125, _, _, _, _, _, _, _, _) => {
                    h1125
                }
            }, match v3.clone() {
                Adt2::C0(_, h1126, _, _, _, _, _, _, _, _, _) => {
                    h1126
                }
            })? {
                f93(match v2.clone() {
                    Adt2::C0(_, _, h1127, _, _, _, _, _, _, _, _) => {
                        h1127
                    }
                }, match v1.clone() {
                    Adt1::C0(_, _, h1128, _, _, _, _, _, _, _) => {
                        h1128
                    }
                }, match v1.clone() {
                    Adt1::C0(_, _, _, h1129, _, _, _, _, _, _) => {
                        h1129
                    }
                }, match v3.clone() {
                    Adt2::C0(_, _, h1130, _, _, _, _, _, _, _, _) => {
                        h1130
                    }
                })?
            } else {
                false
            } {
                f93(match v2.clone() {
                    Adt2::C0(_, _, _, h1131, _, _, _, _, _, _, _) => {
                        h1131
                    }
                }, match v1.clone() {
                    Adt1::C0(_, _, _, _, h1132, _, _, _, _, _) => {
                        h1132
                    }
                }, match v1.clone() {
                    Adt1::C0(_, _, _, _, _, h1133, _, _, _, _) => {
                        h1133
                    }
                }, match v3.clone() {
                    Adt2::C0(_, _, _, h1134, _, _, _, _, _, _, _) => {
                        h1134
                    }
                })?
            } else {
                false
            } {
                f93(match v2.clone() {
                    Adt2::C0(_, _, _, _, h1135, _, _, _, _, _, _) => {
                        h1135
                    }
                }, match v1.clone() {
                    Adt1::C0(_, _, _, _, _, _, h1136, _, _, _) => {
                        h1136
                    }
                }, match v1.clone() {
                    Adt1::C0(_, _, _, _, _, _, _, h1137, _, _) => {
                        h1137
                    }
                }, match v3.clone() {
                    Adt2::C0(_, _, _, _, h1138, _, _, _, _, _, _) => {
                        h1138
                    }
                })?
            } else {
                false
            } {
                let a1140 = match v1.clone() {
                    Adt1::C0(_, _, _, _, _, _, _, _, h1139, _) => {
                        h1139
                    }
                };
                let a1141 = 4294967295u64;
                nat_le(a1140, a1141)
            } else {
                false
            } {
                if v4 {
                    !f27(match v2.clone() {
                        Adt2::C0(_, h1142, _, _, _, _, _, _, _, _, _) => {
                            h1142
                        }
                    }, match v2.clone() {
                        Adt2::C0(_, _, h1143, _, _, _, _, _, _, _, _) => {
                            h1143
                        }
                    }, match v2.clone() {
                        Adt2::C0(_, _, _, h1144, _, _, _, _, _, _, _) => {
                            h1144
                        }
                    }, match v2.clone() {
                        Adt2::C0(_, _, _, _, h1145, _, _, _, _, _, _) => {
                            h1145
                        }
                    })
                } else {
                    true
                }
            } else {
                false
            } {
                Ok(Adt10::C4(f12(v3.clone(), 1u64)?, 4u64, f16(v2, 1u64), f16(v3, 1u64), Adt4::C3(match v1.clone() {
                    Adt1::C0(h1146, _, _, _, _, _, _, _, _, _) => {
                        h1146
                    }
                }, match v1.clone() {
                    Adt1::C0(_, _, h1147, _, _, _, _, _, _, _) => {
                        h1147
                    }
                }, match v1.clone() {
                    Adt1::C0(_, _, _, _, h1148, _, _, _, _, _) => {
                        h1148
                    }
                }, match v1 {
                    Adt1::C0(_, _, _, _, _, _, h1149, _, _, _) => {
                        h1149
                    }
                }, v4)))
            } else {
                Ok(Adt10::C1)
            }
        } else {
            let a1150 = v0;
            let a1151 = 62u64;
            if nat_eq(a1150, a1151) {
                let a1153 = match v1.clone() {
                    Adt1::C0(h1152, _, _, _, _, _, _, _, _, _) => {
                        h1152
                    }
                };
                let a1154 = 2147483648u64;
                if if if if if nat_lt(a1153, a1154) {
                    let a1156 = match v1.clone() {
                        Adt1::C0(_, h1155, _, _, _, _, _, _, _, _) => {
                            h1155
                        }
                    };
                    let a1157 = 2147483648u64;
                    nat_lt(a1156, a1157)
                } else {
                    false
                } {
                    let a1160 = match v1.clone() {
                        Adt1::C0(h1158, _, _, _, _, _, _, _, _, _) => {
                            h1158
                        }
                    };
                    let a1161 = match v2.clone() {
                        Adt2::C0(_, h1159, _, _, _, _, _, _, _, _, _) => {
                            h1159
                        }
                    };
                    nat_lt(a1160, a1161)
                } else {
                    false
                } {
                    let a1164 = match v1.clone() {
                        Adt1::C0(_, h1162, _, _, _, _, _, _, _, _) => {
                            h1162
                        }
                    };
                    let a1165 = match v2.clone() {
                        Adt2::C0(_, h1163, _, _, _, _, _, _, _, _, _) => {
                            h1163
                        }
                    };
                    nat_lt(a1164, a1165)
                } else {
                    false
                } {
                    let a1168 = match v2.clone() {
                        Adt2::C0(_, _, h1166, _, _, _, _, _, _, _, _) => {
                            h1166
                        }
                    };
                    let a1169 = match v3.clone() {
                        Adt2::C0(_, _, h1167, _, _, _, _, _, _, _, _) => {
                            h1167
                        }
                    };
                    if if if nat_eq(a1168, a1169) {
                        let a1172 = match v2.clone() {
                            Adt2::C0(_, _, _, h1170, _, _, _, _, _, _, _) => {
                                h1170
                            }
                        };
                        let a1173 = match v3.clone() {
                            Adt2::C0(_, _, _, h1171, _, _, _, _, _, _, _) => {
                                h1171
                            }
                        };
                        nat_eq(a1172, a1173)
                    } else {
                        false
                    } {
                        let a1176 = match v2.clone() {
                            Adt2::C0(_, _, _, _, h1174, _, _, _, _, _, _) => {
                                h1174
                            }
                        };
                        let a1177 = match v3.clone() {
                            Adt2::C0(_, _, _, _, h1175, _, _, _, _, _, _) => {
                                h1175
                            }
                        };
                        nat_eq(a1176, a1177)
                    } else {
                        false
                    } {
                        let a1186 = match v3.clone() {
                            Adt2::C0(_, h1178, _, _, _, _, _, _, _, _, _) => {
                                h1178
                            }
                        };
                        let a1187 = {
                            let a1184 = {
                                let a1181 = match v2.clone() {
                                    Adt2::C0(_, h1179, _, _, _, _, _, _, _, _, _) => {
                                        h1179
                                    }
                                };
                                let a1182 = match v1.clone() {
                                    Adt1::C0(h1180, _, _, _, _, _, _, _, _, _) => {
                                        h1180
                                    }
                                };
                                nat_add(a1181, a1182)?
                            };
                            let a1185 = match v1.clone() {
                                Adt1::C0(_, h1183, _, _, _, _, _, _, _, _) => {
                                    h1183
                                }
                            };
                            nat_add(a1184, a1185)?
                        };
                        nat_eq(a1186, a1187)
                    } else {
                        false
                    }
                } else {
                    false
                } {
                    Ok(Adt10::C4(f12(v3.clone(), 1u64)?, 4u64, f16(v2, 1u64), f16(v3, 1u64), Adt4::C4(match v1 {
                        Adt1::C0(h1188, _, _, _, _, _, _, _, _, _) => {
                            h1188
                        }
                    })))
                } else {
                    Ok(Adt10::C1)
                }
            } else {
                let a1189 = v0;
                let a1190 = 63u64;
                if nat_eq(a1189, a1190) {
                    if if if if if f94(match v2.clone() {
                        Adt2::C0(_, h1191, _, _, _, _, _, _, _, _, _) => {
                            h1191
                        }
                    }, match v1.clone() {
                        Adt1::C0(h1192, _, _, _, _, _, _, _, _, _) => {
                            h1192
                        }
                    })? {
                        f94(match v2.clone() {
                            Adt2::C0(_, _, h1193, _, _, _, _, _, _, _, _) => {
                                h1193
                            }
                        }, match v1.clone() {
                            Adt1::C0(_, h1194, _, _, _, _, _, _, _, _) => {
                                h1194
                            }
                        })?
                    } else {
                        false
                    } {
                        f94(match v2.clone() {
                            Adt2::C0(_, _, _, h1195, _, _, _, _, _, _, _) => {
                                h1195
                            }
                        }, match v1.clone() {
                            Adt1::C0(_, _, h1196, _, _, _, _, _, _, _) => {
                                h1196
                            }
                        })?
                    } else {
                        false
                    } {
                        f94(match v2.clone() {
                            Adt2::C0(_, _, _, _, h1197, _, _, _, _, _, _) => {
                                h1197
                            }
                        }, match v1.clone() {
                            Adt1::C0(_, _, _, h1198, _, _, _, _, _, _) => {
                                h1198
                            }
                        })?
                    } else {
                        false
                    } {
                        let a1201 = match v2.clone() {
                            Adt2::C0(_, h1199, _, _, _, _, _, _, _, _, _) => {
                                h1199
                            }
                        };
                        let a1202 = match v3.clone() {
                            Adt2::C0(_, h1200, _, _, _, _, _, _, _, _, _) => {
                                h1200
                            }
                        };
                        if if if nat_eq(a1201, a1202) {
                            let a1205 = match v2.clone() {
                                Adt2::C0(_, _, h1203, _, _, _, _, _, _, _, _) => {
                                    h1203
                                }
                            };
                            let a1206 = match v3.clone() {
                                Adt2::C0(_, _, h1204, _, _, _, _, _, _, _, _) => {
                                    h1204
                                }
                            };
                            nat_eq(a1205, a1206)
                        } else {
                            false
                        } {
                            let a1209 = match v2.clone() {
                                Adt2::C0(_, _, _, h1207, _, _, _, _, _, _, _) => {
                                    h1207
                                }
                            };
                            let a1210 = match v3.clone() {
                                Adt2::C0(_, _, _, h1208, _, _, _, _, _, _, _) => {
                                    h1208
                                }
                            };
                            nat_eq(a1209, a1210)
                        } else {
                            false
                        } {
                            let a1213 = match v2.clone() {
                                Adt2::C0(_, _, _, _, h1211, _, _, _, _, _, _) => {
                                    h1211
                                }
                            };
                            let a1214 = match v3.clone() {
                                Adt2::C0(_, _, _, _, h1212, _, _, _, _, _, _) => {
                                    h1212
                                }
                            };
                            nat_eq(a1213, a1214)
                        } else {
                            false
                        }
                    } else {
                        false
                    } {
                        Ok(Adt10::C4(f12(v3.clone(), 1u64)?, 4u64, f16(v2, 1u64), f16(v3, 1u64), Adt4::C5(match v1.clone() {
                            Adt1::C0(h1215, _, _, _, _, _, _, _, _, _) => {
                                h1215
                            }
                        }, match v1.clone() {
                            Adt1::C0(_, h1216, _, _, _, _, _, _, _, _) => {
                                h1216
                            }
                        }, match v1.clone() {
                            Adt1::C0(_, _, h1217, _, _, _, _, _, _, _) => {
                                h1217
                            }
                        }, match v1 {
                            Adt1::C0(_, _, _, h1218, _, _, _, _, _, _) => {
                                h1218
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

pub fn f45(v0: u64, v1: Adt1, v2: Adt2, v3: Option<Adt2>, v4: Adt2) -> R<Adt10> {
    let m1219 = v3;
    match m1219 {
        None => {
            Ok(Adt10::C1)
        }
        Some(v5) => {
            let v6: bool = {
                let a1220 = v0;
                let a1221 = 4u64;
                nat_eq(a1220, a1221)
            };
            let v7: bool = {
                let a1223 = 0u64;
                let a1224 = match v1.clone() {
                    Adt1::C0(_, _, _, _, h1222, _, _, _, _, _) => {
                        h1222
                    }
                };
                nat_lt(a1223, a1224)
            };
            let v8: u64 = f12(v5.clone(), 1u64)?;
            let v9: Adt3 = Adt3::C0(match v5.clone() {
                Adt2::C0(_, h1225, _, _, _, _, _, _, _, _, _) => {
                    h1225
                }
            }, match v5.clone() {
                Adt2::C0(_, _, h1226, _, _, _, _, _, _, _, _) => {
                    h1226
                }
            }, match v5.clone() {
                Adt2::C0(_, _, _, h1227, _, _, _, _, _, _, _) => {
                    h1227
                }
            }, match v5.clone() {
                Adt2::C0(_, _, _, _, h1228, _, _, _, _, _, _) => {
                    h1228
                }
            }, 4u64, match v1.clone() {
                Adt1::C0(h1229, _, _, _, _, _, _, _, _, _) => {
                    h1229
                }
            }, match v1.clone() {
                Adt1::C0(_, h1230, _, _, _, _, _, _, _, _) => {
                    h1230
                }
            }, match v1.clone() {
                Adt1::C0(_, _, h1231, _, _, _, _, _, _, _) => {
                    h1231
                }
            });
            if if if if if if if if if if if if v6 {
                let a1233 = match v2.clone() {
                    Adt2::C0(h1232, _, _, _, _, _, _, _, _, _, _) => {
                        h1232
                    }
                };
                let a1234 = 0u64;
                if nat_eq(a1233, a1234) {
                    let a1236 = match v5.clone() {
                        Adt2::C0(h1235, _, _, _, _, _, _, _, _, _, _) => {
                            h1235
                        }
                    };
                    let a1237 = 0u64;
                    nat_eq(a1236, a1237)
                } else {
                    false
                }
            } else {
                let a1239 = match v2.clone() {
                    Adt2::C0(h1238, _, _, _, _, _, _, _, _, _, _) => {
                        h1238
                    }
                };
                let a1240 = 0u64;
                if if nat_eq(a1239, a1240) {
                    true
                } else {
                    let a1242 = match v2.clone() {
                        Adt2::C0(h1241, _, _, _, _, _, _, _, _, _, _) => {
                            h1241
                        }
                    };
                    let a1243 = 26u64;
                    nat_eq(a1242, a1243)
                } {
                    let a1245 = match v5.clone() {
                        Adt2::C0(h1244, _, _, _, _, _, _, _, _, _, _) => {
                            h1244
                        }
                    };
                    let a1246 = 0u64;
                    if nat_eq(a1245, a1246) {
                        true
                    } else {
                        let a1248 = match v5.clone() {
                            Adt2::C0(h1247, _, _, _, _, _, _, _, _, _, _) => {
                                h1247
                            }
                        };
                        let a1249 = 26u64;
                        nat_eq(a1248, a1249)
                    }
                } else {
                    false
                }
            } {
                let a1252 = match v2.clone() {
                    Adt2::C0(h1250, _, _, _, _, _, _, _, _, _, _) => {
                        h1250
                    }
                };
                let a1253 = match v4.clone() {
                    Adt2::C0(h1251, _, _, _, _, _, _, _, _, _, _) => {
                        h1251
                    }
                };
                nat_eq(a1252, a1253)
            } else {
                false
            } {
                f19(v2.clone(), v4.clone())
            } else {
                false
            } {
                f20(v2.clone())?
            } else {
                false
            } {
                f20(v4.clone())?
            } else {
                false
            } {
                let a1255 = match v5.clone() {
                    Adt2::C0(_, _, _, _, _, h1254, _, _, _, _, _) => {
                        h1254
                    }
                };
                let a1256 = 4u64;
                nat_eq(a1255, a1256)
            } else {
                false
            } {
                let a1257 = v8;
                let a1258 = f12(v2.clone(), 1u64)?;
                nat_le(a1257, a1258)
            } else {
                false
            } {
                let a1260 = match v1.clone() {
                    Adt1::C0(_, _, _, h1259, _, _, _, _, _, _) => {
                        h1259
                    }
                };
                let a1261 = 1073741824u64;
                nat_lt(a1260, a1261)
            } else {
                false
            } {
                let a1263 = match v1.clone() {
                    Adt1::C0(h1262, _, _, _, _, _, _, _, _, _) => {
                        h1262
                    }
                };
                let a1264 = 4294967295u64;
                if if if if nat_le(a1263, a1264) {
                    let a1266 = match v1.clone() {
                        Adt1::C0(_, h1265, _, _, _, _, _, _, _, _) => {
                            h1265
                        }
                    };
                    let a1267 = 4294967295u64;
                    nat_le(a1266, a1267)
                } else {
                    false
                } {
                    let a1269 = match v1.clone() {
                        Adt1::C0(_, _, h1268, _, _, _, _, _, _, _) => {
                            h1268
                        }
                    };
                    let a1270 = 4294967295u64;
                    nat_le(a1269, a1270)
                } else {
                    false
                } {
                    let a1272 = match v1.clone() {
                        Adt1::C0(_, _, _, h1271, _, _, _, _, _, _) => {
                            h1271
                        }
                    };
                    let a1273 = 4294967295u64;
                    nat_le(a1272, a1273)
                } else {
                    false
                } {
                    let a1275 = match v1.clone() {
                        Adt1::C0(_, _, _, _, h1274, _, _, _, _, _) => {
                            h1274
                        }
                    };
                    let a1276 = 4294967295u64;
                    nat_le(a1275, a1276)
                } else {
                    false
                }
            } else {
                false
            } {
                if v7 {
                    let a1279 = match v2.clone() {
                        Adt2::C0(_, _, _, _, _, _, _, _, _, _, h1277) => {
                            h1277
                        }
                    };
                    let a1280 = match v4.clone() {
                        Adt2::C0(_, _, _, _, _, _, _, _, _, _, h1278) => {
                            h1278
                        }
                    };
                    nat_eq(a1279, a1280)
                } else {
                    true
                }
            } else {
                false
            } {
                f95(v9.clone(), match v1.clone() {
                    Adt1::C0(_, _, _, h1281, _, _, _, _, _, _) => {
                        h1281
                    }
                }, match v4 {
                    Adt2::C0(_, _, _, _, _, _, _, _, _, h1282, _) => {
                        h1282
                    }
                }, v8, v6)?
            } else {
                false
            } {
                let v10: u64 = if v7 {
                    0u64
                } else {
                    f12(v2, 1u64)?
                };
                Ok(Adt10::C4({
                    let a1283 = v10;
                    let a1284 = v8;
                    nat_add(a1283, a1284)?
                }, 4u64, f16(v5, 1u64), v9, Adt4::C6(v10, match v1 {
                    Adt1::C0(_, _, _, h1285, _, _, _, _, _, _) => {
                        h1285
                    }
                }, v6)))
            } else {
                Ok(Adt10::C1)
            }
        }
    }
}

pub fn f46(v0: Adt2, v1: Adt2) -> R<Adt10> {
    let a1287 = match v0.clone() {
        Adt2::C0(h1286, _, _, _, _, _, _, _, _, _, _) => {
            h1286
        }
    };
    let a1288 = 0u64;
    if if if nat_eq(a1287, a1288) {
        let a1290 = match v1.clone() {
            Adt2::C0(h1289, _, _, _, _, _, _, _, _, _, _) => {
                h1289
            }
        };
        let a1291 = 0u64;
        nat_eq(a1290, a1291)
    } else {
        false
    } {
        f91(match v0.clone() {
            Adt2::C0(_, h1292, _, _, _, _, _, _, _, _, _) => {
                h1292
            }
        }, match v0.clone() {
            Adt2::C0(_, _, h1293, _, _, _, _, _, _, _, _) => {
                h1293
            }
        }, match v0.clone() {
            Adt2::C0(_, _, _, h1294, _, _, _, _, _, _, _) => {
                h1294
            }
        }, match v0.clone() {
            Adt2::C0(_, _, _, _, h1295, _, _, _, _, _, _) => {
                h1295
            }
        }, match v1.clone() {
            Adt2::C0(_, h1296, _, _, _, _, _, _, _, _, _) => {
                h1296
            }
        }, match v1.clone() {
            Adt2::C0(_, _, h1297, _, _, _, _, _, _, _, _) => {
                h1297
            }
        }, match v1.clone() {
            Adt2::C0(_, _, _, h1298, _, _, _, _, _, _, _) => {
                h1298
            }
        }, match v1.clone() {
            Adt2::C0(_, _, _, _, h1299, _, _, _, _, _, _) => {
                h1299
            }
        })
    } else {
        false
    } {
        let v2: u64 = f12(v1.clone(), 1u64)?;
        Ok(Adt10::C4({
            let a1300 = v2;
            let a1301 = f12(v0.clone(), 1u64)?;
            nat_add(a1300, a1301)?
        }, 4u64, f16(v0, 1u64), f16(v1, 1u64), Adt4::C7(v2)))
    } else {
        Ok(Adt10::C1)
    }
}

pub fn f47(v0: u64, v1: Adt2, v2: Adt2) -> R<Adt10> {
    let a1303 = match v1.clone() {
        Adt2::C0(h1302, _, _, _, _, _, _, _, _, _, _) => {
            h1302
        }
    };
    let a1304 = 0u64;
    if if if nat_eq(a1303, a1304) {
        true
    } else {
        let a1306 = match v1.clone() {
            Adt2::C0(h1305, _, _, _, _, _, _, _, _, _, _) => {
                h1305
            }
        };
        let a1307 = 1u64;
        nat_eq(a1306, a1307)
    } {
        true
    } else {
        let a1309 = match v1.clone() {
            Adt2::C0(h1308, _, _, _, _, _, _, _, _, _, _) => {
                h1308
            }
        };
        let a1310 = 30u64;
        nat_eq(a1309, a1310)
    } {
        let a1313 = match v1.clone() {
            Adt2::C0(h1311, _, _, _, _, _, _, _, _, _, _) => {
                h1311
            }
        };
        let a1314 = match v2.clone() {
            Adt2::C0(h1312, _, _, _, _, _, _, _, _, _, _) => {
                h1312
            }
        };
        if if if if if nat_eq(a1313, a1314) {
            let a1317 = match v1.clone() {
                Adt2::C0(_, h1315, _, _, _, _, _, _, _, _, _) => {
                    h1315
                }
            };
            let a1318 = match v2.clone() {
                Adt2::C0(_, h1316, _, _, _, _, _, _, _, _, _) => {
                    h1316
                }
            };
            nat_eq(a1317, a1318)
        } else {
            false
        } {
            let a1321 = match v1.clone() {
                Adt2::C0(_, _, _, h1319, _, _, _, _, _, _, _) => {
                    h1319
                }
            };
            let a1322 = match v2.clone() {
                Adt2::C0(_, _, _, h1320, _, _, _, _, _, _, _) => {
                    h1320
                }
            };
            nat_eq(a1321, a1322)
        } else {
            false
        } {
            let a1325 = match v1.clone() {
                Adt2::C0(_, _, _, _, h1323, _, _, _, _, _, _) => {
                    h1323
                }
            };
            let a1326 = match v2.clone() {
                Adt2::C0(_, _, _, _, h1324, _, _, _, _, _, _) => {
                    h1324
                }
            };
            nat_eq(a1325, a1326)
        } else {
            false
        } {
            let a1327 = v0;
            let a1328 = 4u64;
            if nat_eq(a1327, a1328) {
                let a1330 = match v1.clone() {
                    Adt2::C0(_, _, h1329, _, _, _, _, _, _, _, _) => {
                        h1329
                    }
                };
                let a1331 = 1u64;
                if nat_eq(a1330, a1331) {
                    let a1334 = match v1.clone() {
                        Adt2::C0(_, h1332, _, _, _, _, _, _, _, _, _) => {
                            h1332
                        }
                    };
                    let a1335 = match v2.clone() {
                        Adt2::C0(_, _, h1333, _, _, _, _, _, _, _, _) => {
                            h1333
                        }
                    };
                    nat_eq(a1334, a1335)
                } else {
                    false
                }
            } else {
                let a1336 = v0;
                let a1337 = 4u64;
                if if nat_lt(a1336, a1337) {
                    let a1340 = match v1.clone() {
                        Adt2::C0(_, h1338, _, _, _, _, _, _, _, _, _) => {
                            h1338
                        }
                    };
                    let a1341 = match v1.clone() {
                        Adt2::C0(_, _, h1339, _, _, _, _, _, _, _, _) => {
                            h1339
                        }
                    };
                    nat_eq(a1340, a1341)
                } else {
                    false
                } {
                    let a1344 = match v1.clone() {
                        Adt2::C0(_, _, h1342, _, _, _, _, _, _, _, _) => {
                            h1342
                        }
                    };
                    let a1345 = match v2.clone() {
                        Adt2::C0(_, _, h1343, _, _, _, _, _, _, _, _) => {
                            h1343
                        }
                    };
                    nat_eq(a1344, a1345)
                } else {
                    false
                }
            }
        } else {
            false
        } {
            let m1347 = f90(match v1.clone() {
                Adt2::C0(h1346, _, _, _, _, _, _, _, _, _, _) => {
                    h1346
                }
            });
            match m1347 {
                Adt14::C0 => {
                    Ok(Adt10::C0)
                }
                Adt14::C1(v3) => {
                    Ok(Adt10::C4(f12(v2.clone(), 1u64)?, match v3 {
                        Adt15::C0(_, _, h1348) => {
                            h1348
                        }
                    }, f16(v1, 1u64), f16(v2, 1u64), Adt4::C8(v0)))
                }
            }
        } else {
            Ok(Adt10::C1)
        }
    } else {
        Ok(Adt10::C0)
    }
}

pub fn f48(v0: u64, v1: Adt2, v2: Adt2) -> R<Adt10> {
    if if if f96(match v1.clone() {
        Adt2::C0(h1349, _, _, _, _, _, _, _, _, _, _) => {
            h1349
        }
    }, v0) {
        let a1352 = match v1.clone() {
            Adt2::C0(h1350, _, _, _, _, _, _, _, _, _, _) => {
                h1350
            }
        };
        let a1353 = match v2.clone() {
            Adt2::C0(h1351, _, _, _, _, _, _, _, _, _, _) => {
                h1351
            }
        };
        nat_eq(a1352, a1353)
    } else {
        false
    } {
        let a1356 = match v1.clone() {
            Adt2::C0(_, h1354, _, _, _, _, _, _, _, _, _) => {
                h1354
            }
        };
        let a1357 = match v2.clone() {
            Adt2::C0(_, h1355, _, _, _, _, _, _, _, _, _) => {
                h1355
            }
        };
        if if if nat_eq(a1356, a1357) {
            let a1360 = match v1.clone() {
                Adt2::C0(_, _, h1358, _, _, _, _, _, _, _, _) => {
                    h1358
                }
            };
            let a1361 = match v2.clone() {
                Adt2::C0(_, _, h1359, _, _, _, _, _, _, _, _) => {
                    h1359
                }
            };
            nat_eq(a1360, a1361)
        } else {
            false
        } {
            let a1364 = match v1.clone() {
                Adt2::C0(_, _, _, h1362, _, _, _, _, _, _, _) => {
                    h1362
                }
            };
            let a1365 = match v2.clone() {
                Adt2::C0(_, _, _, h1363, _, _, _, _, _, _, _) => {
                    h1363
                }
            };
            nat_eq(a1364, a1365)
        } else {
            false
        } {
            let a1368 = match v1.clone() {
                Adt2::C0(_, _, _, _, h1366, _, _, _, _, _, _) => {
                    h1366
                }
            };
            let a1369 = match v2.clone() {
                Adt2::C0(_, _, _, _, h1367, _, _, _, _, _, _) => {
                    h1367
                }
            };
            nat_eq(a1368, a1369)
        } else {
            false
        }
    } else {
        false
    } {
        let m1371 = f90(match v1.clone() {
            Adt2::C0(h1370, _, _, _, _, _, _, _, _, _, _) => {
                h1370
            }
        });
        match m1371 {
            Adt14::C0 => {
                Ok(Adt10::C0)
            }
            Adt14::C1(v3) => {
                Ok(Adt10::C4(f12(v2.clone(), 1u64)?, match v3 {
                    Adt15::C0(_, _, h1372) => {
                        h1372
                    }
                }, f16(v1.clone(), 1u64), f16(v2, 1u64), Adt4::C9(match v1 {
                    Adt2::C0(h1373, _, _, _, _, _, _, _, _, _, _) => {
                        h1373
                    }
                }, v0)))
            }
        }
    } else {
        Ok(Adt10::C1)
    }
}

pub fn f49(v0: bool, v1: Adt1, v2: Adt2, v3: Adt2) -> R<Adt10> {
    let v4: u64 = if v0 {
        match v1.clone() {
            Adt1::C0(h1374, _, _, _, _, _, _, _, _, _) => {
                h1374
            }
        }
    } else {
        match v1.clone() {
            Adt1::C0(_, _, h1375, _, _, _, _, _, _, _) => {
                h1375
            }
        }
    };
    let a1376 = 0u64;
    let a1377 = v4;
    if if nat_lt(a1376, a1377) {
        let a1378 = v4;
        let a1379 = 2147483648u64;
        nat_lt(a1378, a1379)
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
        let v7: u64 = f97(match v5.clone() {
            Adt2::C0(_, _, h1380, _, _, _, _, _, _, _, _) => {
                h1380
            }
        }, v4)?;
        let v8: u64 = f97(match v5.clone() {
            Adt2::C0(_, _, _, h1381, _, _, _, _, _, _, _) => {
                h1381
            }
        }, v4)?;
        let a1383 = match v5.clone() {
            Adt2::C0(h1382, _, _, _, _, _, _, _, _, _, _) => {
                h1382
            }
        };
        let a1384 = 0u64;
        if if if if if nat_eq(a1383, a1384) {
            let a1386 = match v6.clone() {
                Adt2::C0(h1385, _, _, _, _, _, _, _, _, _, _) => {
                    h1385
                }
            };
            let a1387 = 0u64;
            nat_eq(a1386, a1387)
        } else {
            false
        } {
            let a1389 = match v5.clone() {
                Adt2::C0(_, _, _, _, h1388, _, _, _, _, _, _) => {
                    h1388
                }
            };
            let a1390 = 1u64;
            nat_eq(a1389, a1390)
        } else {
            false
        } {
            let a1393 = match v5.clone() {
                Adt2::C0(_, h1391, _, _, _, _, _, _, _, _, _) => {
                    h1391
                }
            };
            let a1394 = match v6.clone() {
                Adt2::C0(_, h1392, _, _, _, _, _, _, _, _, _) => {
                    h1392
                }
            };
            nat_eq(a1393, a1394)
        } else {
            false
        } {
            if v0 {
                let a1396 = match v5.clone() {
                    Adt2::C0(_, _, h1395, _, _, _, _, _, _, _, _) => {
                        h1395
                    }
                };
                let a1397 = 2147483648u64;
                if if if nat_lt(a1396, a1397) {
                    let a1399 = match v5.clone() {
                        Adt2::C0(_, _, _, h1398, _, _, _, _, _, _, _) => {
                            h1398
                        }
                    };
                    let a1400 = 2147483648u64;
                    nat_lt(a1399, a1400)
                } else {
                    false
                } {
                    let a1403 = {
                        let a1401 = v7;
                        let a1402 = v8;
                        nat_mul(a1401, a1402)?
                    };
                    let a1404 = 2147483648u64;
                    nat_le(a1403, a1404)
                } else {
                    false
                } {
                    f98(v5, v6, v4, v7)?
                } else {
                    false
                }
            } else {
                let a1406 = match v1.clone() {
                    Adt1::C0(h1405, _, _, _, _, _, _, _, _, _) => {
                        h1405
                    }
                };
                let a1407 = v7;
                if if if if nat_eq(a1406, a1407) {
                    let a1409 = match v1 {
                        Adt1::C0(_, h1408, _, _, _, _, _, _, _, _) => {
                            h1408
                        }
                    };
                    let a1410 = v8;
                    nat_eq(a1409, a1410)
                } else {
                    false
                } {
                    f99(v7, v8, v4)?
                } else {
                    false
                } {
                    let a1412 = match v6.clone() {
                        Adt2::C0(_, _, h1411, _, _, _, _, _, _, _, _) => {
                            h1411
                        }
                    };
                    let a1413 = v4;
                    if nat_eq(a1412, a1413) {
                        let a1415 = match v6.clone() {
                            Adt2::C0(_, _, _, h1414, _, _, _, _, _, _, _) => {
                                h1414
                            }
                        };
                        let a1416 = v4;
                        nat_eq(a1415, a1416)
                    } else {
                        false
                    }
                } else {
                    false
                } {
                    let a1420 = match v6 {
                        Adt2::C0(_, _, _, _, h1417, _, _, _, _, _, _) => {
                            h1417
                        }
                    };
                    let a1421 = {
                        let a1418 = v7;
                        let a1419 = v8;
                        nat_mul(a1418, a1419)?
                    };
                    nat_eq(a1420, a1421)
                } else {
                    false
                }
            }
        } else {
            false
        } {
            Ok(Adt10::C4(f12(v3.clone(), 1u64)?, 4u64, f16(v2, 1u64), f16(v3, 1u64), Adt4::C12(v0, v4, v7)))
        } else {
            Ok(Adt10::C1)
        }
    } else {
        Ok(Adt10::C1)
    }
}

pub fn f50(v0: u64, v1: Adt1, v2: Adt2, v3: Adt2) -> R<Adt10> {
    let a1423 = match v2.clone() {
        Adt2::C0(h1422, _, _, _, _, _, _, _, _, _, _) => {
            h1422
        }
    };
    let a1424 = 0u64;
    if if if if if nat_eq(a1423, a1424) {
        true
    } else {
        let a1426 = match v2.clone() {
            Adt2::C0(h1425, _, _, _, _, _, _, _, _, _, _) => {
                h1425
            }
        };
        let a1427 = 1u64;
        nat_eq(a1426, a1427)
    } {
        true
    } else {
        let a1429 = match v2.clone() {
            Adt2::C0(h1428, _, _, _, _, _, _, _, _, _, _) => {
                h1428
            }
        };
        let a1430 = 30u64;
        nat_eq(a1429, a1430)
    } {
        let a1433 = match v2.clone() {
            Adt2::C0(h1431, _, _, _, _, _, _, _, _, _, _) => {
                h1431
            }
        };
        let a1434 = match v3.clone() {
            Adt2::C0(h1432, _, _, _, _, _, _, _, _, _, _) => {
                h1432
            }
        };
        nat_eq(a1433, a1434)
    } else {
        false
    } {
        if if f19(v2.clone(), v3.clone()) {
            let a1436 = match v1.clone() {
                Adt1::C0(h1435, _, _, _, _, _, _, _, _, _) => {
                    h1435
                }
            };
            let a1437 = 4294967296u64;
            nat_lt(a1436, a1437)
        } else {
            false
        } {
            let a1439 = match v1.clone() {
                Adt1::C0(_, h1438, _, _, _, _, _, _, _, _) => {
                    h1438
                }
            };
            let a1440 = 4294967296u64;
            nat_lt(a1439, a1440)
        } else {
            false
        }
    } else {
        false
    } {
        Ok(Adt10::C4(f12(v3.clone(), 1u64)?, {
            let a1442 = match v2.clone() {
                Adt2::C0(h1441, _, _, _, _, _, _, _, _, _, _) => {
                    h1441
                }
            };
            let a1443 = 0u64;
            if nat_eq(a1442, a1443) {
                4u64
            } else {
                2u64
            }
        }, f16(v2.clone(), 1u64), f16(v3, 1u64), Adt4::C13({
            let a1444 = v0;
            let a1445 = 31u64;
            if nat_eq(a1444, a1445) {
                0u64
            } else {
                let a1446 = v0;
                let a1447 = 47u64;
                if nat_eq(a1446, a1447) {
                    1u64
                } else {
                    2u64
                }
            }
        }, match v2 {
            Adt2::C0(h1448, _, _, _, _, _, _, _, _, _, _) => {
                h1448
            }
        }, match v1.clone() {
            Adt1::C0(h1449, _, _, _, _, _, _, _, _, _) => {
                h1449
            }
        }, match v1 {
            Adt1::C0(_, h1450, _, _, _, _, _, _, _, _) => {
                h1450
            }
        })))
    } else {
        Ok(Adt10::C1)
    }
}

pub fn f51(v0: Adt2, v1: Adt2) -> R<Adt10> {
    let a1452 = match v0.clone() {
        Adt2::C0(h1451, _, _, _, _, _, _, _, _, _, _) => {
            h1451
        }
    };
    let a1453 = 0u64;
    if if if nat_eq(a1452, a1453) {
        let a1455 = match v1.clone() {
            Adt2::C0(h1454, _, _, _, _, _, _, _, _, _, _) => {
                h1454
            }
        };
        let a1456 = 0u64;
        nat_eq(a1455, a1456)
    } else {
        false
    } {
        f19(v0.clone(), v1.clone())
    } else {
        false
    } {
        Ok(Adt10::C4(f12(v1.clone(), 1u64)?, 4u64, f16(v0, 1u64), f16(v1, 1u64), Adt4::C15))
    } else {
        Ok(Adt10::C1)
    }
}

pub fn f52(v0: Adt2, v1: Adt2) -> R<Adt10> {
    let a1458 = match v0.clone() {
        Adt2::C0(h1457, _, _, _, _, _, _, _, _, _, _) => {
            h1457
        }
    };
    let a1459 = 0u64;
    if if nat_eq(a1458, a1459) {
        let a1461 = match v1.clone() {
            Adt2::C0(h1460, _, _, _, _, _, _, _, _, _, _) => {
                h1460
            }
        };
        let a1462 = 26u64;
        nat_eq(a1461, a1462)
    } else {
        false
    } {
        let a1464 = match v0.clone() {
            Adt2::C0(_, _, _, h1463, _, _, _, _, _, _, _) => {
                h1463
            }
        };
        let a1465 = 1u64;
        if if if if if if if nat_eq(a1464, a1465) {
            let a1467 = match v0.clone() {
                Adt2::C0(_, _, _, _, h1466, _, _, _, _, _, _) => {
                    h1466
                }
            };
            let a1468 = 1u64;
            nat_eq(a1467, a1468)
        } else {
            false
        } {
            let a1470 = match v0.clone() {
                Adt2::C0(_, h1469, _, _, _, _, _, _, _, _, _) => {
                    h1469
                }
            };
            let a1471 = 2147483647u64;
            nat_le(a1470, a1471)
        } else {
            false
        } {
            let a1474 = match v1.clone() {
                Adt2::C0(_, h1472, _, _, _, _, _, _, _, _, _) => {
                    h1472
                }
            };
            let a1475 = match v0.clone() {
                Adt2::C0(_, _, h1473, _, _, _, _, _, _, _, _) => {
                    h1473
                }
            };
            nat_eq(a1474, a1475)
        } else {
            false
        } {
            let a1477 = match v1.clone() {
                Adt2::C0(_, _, h1476, _, _, _, _, _, _, _, _) => {
                    h1476
                }
            };
            let a1478 = 1u64;
            nat_eq(a1477, a1478)
        } else {
            false
        } {
            let a1480 = match v1.clone() {
                Adt2::C0(_, _, _, h1479, _, _, _, _, _, _, _) => {
                    h1479
                }
            };
            let a1481 = 1u64;
            nat_eq(a1480, a1481)
        } else {
            false
        } {
            let a1483 = match v1.clone() {
                Adt2::C0(_, _, _, _, h1482, _, _, _, _, _, _) => {
                    h1482
                }
            };
            let a1484 = 1u64;
            nat_eq(a1483, a1484)
        } else {
            false
        } {
            Ok(Adt10::C4({
                let a1487 = match v0.clone() {
                    Adt2::C0(_, _, h1485, _, _, _, _, _, _, _, _) => {
                        h1485
                    }
                };
                let a1488 = f100(match v0.clone() {
                    Adt2::C0(_, h1486, _, _, _, _, _, _, _, _, _) => {
                        h1486
                    }
                })?;
                nat_mul(a1487, a1488)?
            }, 4u64, f16(v0, 1u64), f16(v1, 1u64), Adt4::C17))
        } else {
            Ok(Adt10::C1)
        }
    } else {
        Ok(Adt10::C0)
    }
}

pub fn f53(v0: Adt2, v1: Option<Adt2>, v2: Adt2) -> R<Adt10> {
    let m1489 = v1;
    match m1489 {
        None => {
            Ok(Adt10::C1)
        }
        Some(v3) => {
            let v4: u64 = f101(f16(v0.clone(), 1u64))?;
            let a1491 = match v0.clone() {
                Adt2::C0(h1490, _, _, _, _, _, _, _, _, _, _) => {
                    h1490
                }
            };
            let a1492 = 26u64;
            if if if nat_eq(a1491, a1492) {
                let a1494 = match v3.clone() {
                    Adt2::C0(h1493, _, _, _, _, _, _, _, _, _, _) => {
                        h1493
                    }
                };
                let a1495 = 26u64;
                nat_eq(a1494, a1495)
            } else {
                false
            } {
                let a1497 = match v2.clone() {
                    Adt2::C0(h1496, _, _, _, _, _, _, _, _, _, _) => {
                        h1496
                    }
                };
                let a1498 = 27u64;
                nat_eq(a1497, a1498)
            } else {
                false
            } {
                if if if if if if f19(v0.clone(), v3.clone()) {
                    let a1500 = match v2.clone() {
                        Adt2::C0(_, h1499, _, _, _, _, _, _, _, _, _) => {
                            h1499
                        }
                    };
                    let a1501 = 1u64;
                    nat_eq(a1500, a1501)
                } else {
                    false
                } {
                    let a1503 = match v2.clone() {
                        Adt2::C0(_, _, h1502, _, _, _, _, _, _, _, _) => {
                            h1502
                        }
                    };
                    let a1504 = 1u64;
                    nat_eq(a1503, a1504)
                } else {
                    false
                } {
                    let a1506 = match v2.clone() {
                        Adt2::C0(_, _, _, h1505, _, _, _, _, _, _, _) => {
                            h1505
                        }
                    };
                    let a1507 = 1u64;
                    nat_eq(a1506, a1507)
                } else {
                    false
                } {
                    let a1509 = match v2.clone() {
                        Adt2::C0(_, _, _, _, h1508, _, _, _, _, _, _) => {
                            h1508
                        }
                    };
                    let a1510 = 1u64;
                    nat_eq(a1509, a1510)
                } else {
                    false
                } {
                    let a1511 = v4;
                    let a1512 = 9223372036854775808u64;
                    nat_lt(a1511, a1512)
                } else {
                    false
                } {
                    Ok(Adt10::C4({
                        let a1515 = 2u64;
                        let a1516 = {
                            let a1513 = v4;
                            let a1514 = 2u64;
                            nat_mul(a1513, a1514)?
                        };
                        nat_add(a1515, a1516)?
                    }, 8u64, f16(v0, 1u64), f16(v2, 1u64), Adt4::C18(v4, f16(v3, 1u64))))
                } else {
                    Ok(Adt10::C1)
                }
            } else {
                Ok(Adt10::C0)
            }
        }
    }
}

pub fn f54(v0: u64, v1: Adt2, v2: Adt2) -> R<Adt10> {
    let v3: u64 = match v1.clone() {
        Adt2::C0(h1517, _, _, _, _, _, _, _, _, _, _) => {
            h1517
        }
    };
    let a1518 = v3;
    let a1519 = 0u64;
    if if nat_eq(a1518, a1519) {
        true
    } else {
        let a1520 = v0;
        let a1521 = 13u64;
        if nat_eq(a1520, a1521) {
            let a1522 = v3;
            let a1523 = 1u64;
            if nat_eq(a1522, a1523) {
                true
            } else {
                let a1524 = v3;
                let a1525 = 30u64;
                nat_eq(a1524, a1525)
            }
        } else {
            false
        }
    } {
        let a1527 = v3;
        let a1528 = match v2.clone() {
            Adt2::C0(h1526, _, _, _, _, _, _, _, _, _, _) => {
                h1526
            }
        };
        if if nat_eq(a1527, a1528) {
            let a1529 = v0;
            let a1530 = 13u64;
            if nat_eq(a1529, a1530) {
                let a1532 = match v2.clone() {
                    Adt2::C0(_, h1531, _, _, _, _, _, _, _, _, _) => {
                        h1531
                    }
                };
                let a1533 = 1u64;
                if if if nat_eq(a1532, a1533) {
                    let a1535 = match v2.clone() {
                        Adt2::C0(_, _, h1534, _, _, _, _, _, _, _, _) => {
                            h1534
                        }
                    };
                    let a1536 = 1u64;
                    nat_eq(a1535, a1536)
                } else {
                    false
                } {
                    let a1538 = match v2.clone() {
                        Adt2::C0(_, _, _, h1537, _, _, _, _, _, _, _) => {
                            h1537
                        }
                    };
                    let a1539 = 1u64;
                    nat_eq(a1538, a1539)
                } else {
                    false
                } {
                    let a1541 = match v2.clone() {
                        Adt2::C0(_, _, _, _, h1540, _, _, _, _, _, _) => {
                            h1540
                        }
                    };
                    let a1542 = 1u64;
                    nat_eq(a1541, a1542)
                } else {
                    false
                }
            } else {
                let a1544 = match v2.clone() {
                    Adt2::C0(_, h1543, _, _, _, _, _, _, _, _, _) => {
                        h1543
                    }
                };
                let a1545 = 1u64;
                if if if nat_eq(a1544, a1545) {
                    let a1548 = match v1.clone() {
                        Adt2::C0(_, _, h1546, _, _, _, _, _, _, _, _) => {
                            h1546
                        }
                    };
                    let a1549 = match v2.clone() {
                        Adt2::C0(_, _, h1547, _, _, _, _, _, _, _, _) => {
                            h1547
                        }
                    };
                    nat_eq(a1548, a1549)
                } else {
                    false
                } {
                    let a1552 = match v1.clone() {
                        Adt2::C0(_, _, _, h1550, _, _, _, _, _, _, _) => {
                            h1550
                        }
                    };
                    let a1553 = match v2.clone() {
                        Adt2::C0(_, _, _, h1551, _, _, _, _, _, _, _) => {
                            h1551
                        }
                    };
                    nat_eq(a1552, a1553)
                } else {
                    false
                } {
                    let a1556 = match v1.clone() {
                        Adt2::C0(_, _, _, _, h1554, _, _, _, _, _, _) => {
                            h1554
                        }
                    };
                    let a1557 = match v2.clone() {
                        Adt2::C0(_, _, _, _, h1555, _, _, _, _, _, _) => {
                            h1555
                        }
                    };
                    nat_eq(a1556, a1557)
                } else {
                    false
                }
            }
        } else {
            false
        } {
            let v4: Adt3 = f16(v1, 1u64);
            Ok(Adt10::C4(f102(v4.clone(), v0, v3)?, {
                let a1558 = v3;
                let a1559 = 0u64;
                if nat_eq(a1558, a1559) {
                    4u64
                } else {
                    2u64
                }
            }, v4, f16(v2, 1u64), Adt4::C16(v0, v3)))
        } else {
            Ok(Adt10::C1)
        }
    } else {
        Ok(Adt10::C0)
    }
}

pub fn f55(v0: u64, v1: u64) -> R<Option<u64>> {
    let a1560 = v0;
    let a1561 = 89u64;
    if if nat_eq(a1560, a1561) {
        let a1562 = 8u64;
        let a1563 = v1;
        if nat_le(a1562, a1563) {
            let a1564 = v1;
            let a1565 = 9u64;
            nat_le(a1564, a1565)
        } else {
            false
        }
    } else {
        false
    } {
        Ok(Some({
            let a1566 = v1;
            let a1567 = 12u64;
            nat_add(a1566, a1567)?
        }))
    } else {
        let a1568 = v0;
        let a1569 = 89u64;
        if if nat_eq(a1568, a1569) {
            let a1570 = v1;
            let a1571 = 4u64;
            nat_eq(a1570, a1571)
        } else {
            false
        } {
            Ok(Some(19u64))
        } else {
            let a1572 = 10u64;
            let a1573 = v0;
            if if nat_le(a1572, a1573) {
                let a1574 = v0;
                let a1575 = 12u64;
                nat_le(a1574, a1575)
            } else {
                false
            } {
                Ok(Some({
                    let a1576 = v0;
                    let a1577 = 3u64;
                    nat_add(a1576, a1577)?
                }))
            } else {
                let a1578 = v0;
                let a1579 = 89u64;
                if if nat_eq(a1578, a1579) {
                    let a1580 = v1;
                    let a1581 = 7u64;
                    nat_eq(a1580, a1581)
                } else {
                    false
                } {
                    Ok(Some(17u64))
                } else {
                    let a1582 = v0;
                    let a1583 = 89u64;
                    if if nat_eq(a1582, a1583) {
                        let a1584 = v1;
                        let a1585 = 10u64;
                        nat_eq(a1584, a1585)
                    } else {
                        false
                    } {
                        Ok(Some(18u64))
                    } else {
                        let a1586 = v0;
                        let a1587 = 89u64;
                        if if nat_eq(a1586, a1587) {
                            let a1588 = v1;
                            let a1589 = 13u64;
                            nat_eq(a1588, a1589)
                        } else {
                            false
                        } {
                            Ok(Some(16u64))
                        } else {
                            let a1590 = v0;
                            let a1591 = 8u64;
                            if nat_eq(a1590, a1591) {
                                Ok(Some(5u64))
                            } else {
                                let a1592 = v0;
                                let a1593 = 9u64;
                                if nat_eq(a1592, a1593) {
                                    Ok(Some(6u64))
                                } else {
                                    let a1594 = v0;
                                    let a1595 = 89u64;
                                    if nat_eq(a1594, a1595) {
                                        let a1596 = v1;
                                        let a1597 = 0u64;
                                        if nat_eq(a1596, a1597) {
                                            Ok(Some(0u64))
                                        } else {
                                            let a1598 = v1;
                                            let a1599 = 1u64;
                                            if nat_eq(a1598, a1599) {
                                                Ok(Some(1u64))
                                            } else {
                                                let a1600 = v1;
                                                let a1601 = 2u64;
                                                if nat_eq(a1600, a1601) {
                                                    Ok(Some(2u64))
                                                } else {
                                                    let a1602 = v1;
                                                    let a1603 = 3u64;
                                                    if nat_eq(a1602, a1603) {
                                                        Ok(Some(3u64))
                                                    } else {
                                                        let a1604 = v1;
                                                        let a1605 = 6u64;
                                                        if nat_eq(a1604, a1605) {
                                                            Ok(Some(4u64))
                                                        } else {
                                                            let a1606 = v1;
                                                            let a1607 = 12u64;
                                                            if nat_eq(a1606, a1607) {
                                                                Ok(Some(7u64))
                                                            } else {
                                                                let a1608 = v1;
                                                                let a1609 = 11u64;
                                                                if nat_eq(a1608, a1609) {
                                                                    Ok(Some(8u64))
                                                                } else {
                                                                    let a1610 = v1;
                                                                    let a1611 = 18u64;
                                                                    if nat_eq(a1610, a1611) {
                                                                        Ok(Some(9u64))
                                                                    } else {
                                                                        let a1612 = v1;
                                                                        let a1613 = 19u64;
                                                                        if nat_eq(a1612, a1613) {
                                                                            Ok(Some(10u64))
                                                                        } else {
                                                                            let a1614 = v1;
                                                                            let a1615 = 20u64;
                                                                            if nat_eq(a1614, a1615) {
                                                                                Ok(Some(11u64))
                                                                            } else {
                                                                                let a1616 = v1;
                                                                                let a1617 = 21u64;
                                                                                if nat_eq(a1616, a1617) {
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

pub fn f56(v0: u64, v1: u64, v2: Adt2, v3: Option<Adt2>, v4: Adt2) -> R<Adt10> {
    let m1618 = f103(v0, v1, v2.clone(), v3.clone(), v4.clone())?;
    match m1618 {
        Adt10::C0 => {
            Ok(Adt10::C0)
        }
        Adt10::C1 => {
            let a1619 = v0;
            let a1620 = 2u64;
            if if if if nat_eq(a1619, a1620) {
                true
            } else {
                let a1621 = v0;
                let a1622 = 5u64;
                nat_eq(a1621, a1622)
            } {
                true
            } else {
                let a1623 = v0;
                let a1624 = 6u64;
                nat_eq(a1623, a1624)
            } {
                true
            } else {
                let a1625 = v0;
                let a1626 = 7u64;
                nat_eq(a1625, a1626)
            } {
                Ok(Adt10::C5(67u64))
            } else {
                Ok(Adt10::C1)
            }
        }
        Adt10::C2(v7, v8, _, _, _, _, v13) => {
            if v8 {
                Ok(Adt10::C4(v13, 4u64, f16(v2.clone(), 1u64), f16(v4, 1u64), Adt4::C10(v7, v8, {
                    let m1627 = v3;
                    match m1627 {
                        None => {
                            f16(v2, 1u64)
                        }
                        Some(v14) => {
                            f16(v14, 1u64)
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

pub fn f57(v0: u64, v1: Adt2, v2: Adt2) -> R<Adt10> {
    let a1629 = match v1.clone() {
        Adt2::C0(h1628, _, _, _, _, _, _, _, _, _, _) => {
            h1628
        }
    };
    let a1630 = 0u64;
    if if if if if nat_eq(a1629, a1630) {
        true
    } else {
        let a1632 = match v1.clone() {
            Adt2::C0(h1631, _, _, _, _, _, _, _, _, _, _) => {
                h1631
            }
        };
        let a1633 = 1u64;
        nat_eq(a1632, a1633)
    } {
        true
    } else {
        let a1635 = match v1.clone() {
            Adt2::C0(h1634, _, _, _, _, _, _, _, _, _, _) => {
                h1634
            }
        };
        let a1636 = 30u64;
        nat_eq(a1635, a1636)
    } {
        let a1639 = match v1.clone() {
            Adt2::C0(h1637, _, _, _, _, _, _, _, _, _, _) => {
                h1637
            }
        };
        let a1640 = match v2.clone() {
            Adt2::C0(h1638, _, _, _, _, _, _, _, _, _, _) => {
                h1638
            }
        };
        if nat_eq(a1639, a1640) {
            true
        } else {
            let a1642 = match v2.clone() {
                Adt2::C0(h1641, _, _, _, _, _, _, _, _, _, _) => {
                    h1641
                }
            };
            let a1643 = 0u64;
            nat_eq(a1642, a1643)
        }
    } else {
        false
    } {
        f19(v1.clone(), v2.clone())
    } else {
        false
    } {
        Ok(Adt10::C4(f12(v2.clone(), 1u64)?, {
            let a1645 = match v2.clone() {
                Adt2::C0(h1644, _, _, _, _, _, _, _, _, _, _) => {
                    h1644
                }
            };
            let a1646 = 0u64;
            if nat_eq(a1645, a1646) {
                4u64
            } else {
                2u64
            }
        }, f16(v1.clone(), 1u64), f16(v2.clone(), 1u64), Adt4::C14(v0, match v1 {
            Adt2::C0(h1647, _, _, _, _, _, _, _, _, _, _) => {
                h1647
            }
        }, match v2 {
            Adt2::C0(h1648, _, _, _, _, _, _, _, _, _, _) => {
                h1648
            }
        })))
    } else {
        Ok(Adt10::C1)
    }
}

pub fn f58(v0: Adt3, v1: u64, v2: u64, v3: u64, v4: u64) -> R<u64> {
    f104(v1, v2, v3, v4, match v0.clone() {
        Adt3::C0(_, _, _, _, h1649, _, _, _) => {
            h1649
        }
    }, match v0.clone() {
        Adt3::C0(_, _, _, _, _, h1650, _, _) => {
            h1650
        }
    }, match v0.clone() {
        Adt3::C0(_, _, _, _, _, _, h1651, _) => {
            h1651
        }
    }, match v0 {
        Adt3::C0(_, _, _, _, _, _, _, h1652) => {
            h1652
        }
    })
}

pub fn f59(v0: u64, v1: Adt3, v2: u64) -> u64 {
    let v3: u64 = {
        let a1654 = v0;
        let a1655 = match v1.clone() {
            Adt3::C0(h1653, _, _, _, _, _, _, _) => {
                h1653
            }
        };
        let a1656 = 0u64;
        nat_quot(a1654, a1655, a1656)
    };
    let v4: u64 = {
        let a1658 = v3;
        let a1659 = match v1.clone() {
            Adt3::C0(_, h1657, _, _, _, _, _, _) => {
                h1657
            }
        };
        let a1660 = 0u64;
        nat_quot(a1658, a1659, a1660)
    };
    let a1661 = v2;
    let a1662 = 0u64;
    if nat_eq(a1661, a1662) {
        let a1664 = v0;
        let a1665 = match v1 {
            Adt3::C0(h1663, _, _, _, _, _, _, _) => {
                h1663
            }
        };
        let a1666 = 0u64;
        nat_rem(a1664, a1665, a1666)
    } else {
        let a1667 = v2;
        let a1668 = 1u64;
        if nat_eq(a1667, a1668) {
            let a1670 = v3;
            let a1671 = match v1 {
                Adt3::C0(_, h1669, _, _, _, _, _, _) => {
                    h1669
                }
            };
            let a1672 = 0u64;
            nat_rem(a1670, a1671, a1672)
        } else {
            let a1673 = v2;
            let a1674 = 2u64;
            if nat_eq(a1673, a1674) {
                let a1676 = v4;
                let a1677 = match v1 {
                    Adt3::C0(_, _, h1675, _, _, _, _, _) => {
                        h1675
                    }
                };
                let a1678 = 0u64;
                nat_rem(a1676, a1677, a1678)
            } else {
                let a1680 = v4;
                let a1681 = match v1 {
                    Adt3::C0(_, _, h1679, _, _, _, _, _) => {
                        h1679
                    }
                };
                let a1682 = 0u64;
                nat_quot(a1680, a1681, a1682)
            }
        }
    }
}

pub fn f60(v0: Adt6, v1: u64, v2: u64, v3: u64) -> R<Option<u64>> {
    let v4: u64 = {
        let a1692 = {
            let a1689 = {
                let a1684 = v2;
                let a1685 = match v0.clone() {
                    Adt6::C0(_, h1683, _, _) => {
                        h1683
                    }
                };
                nat_mul(a1684, a1685)?
            };
            let a1690 = {
                let a1687 = v3;
                let a1688 = match v0.clone() {
                    Adt6::C0(_, _, _, h1686) => {
                        h1686
                    }
                };
                nat_mul(a1687, a1688)?
            };
            nat_add(a1689, a1690)?
        };
        let a1693 = f75(match v0.clone() {
            Adt6::C0(_, _, h1691, _) => {
                h1691
            }
        });
        nat_add(a1692, a1693)?
    };
    let v5: u64 = f74(match v0 {
        Adt6::C0(_, _, h1694, _) => {
            h1694
        }
    });
    let a1695 = v5;
    let a1696 = v4;
    if if nat_le(a1695, a1696) {
        let a1699 = {
            let a1697 = v4;
            let a1698 = v5;
            nat_sub(a1697, a1698)
        };
        let a1700 = v1;
        nat_lt(a1699, a1700)
    } else {
        false
    } {
        Ok(Some({
            let a1701 = v4;
            let a1702 = v5;
            nat_sub(a1701, a1702)
        }))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f61(v0: Adt4, v1: Adt9) -> Adt9 {
    let m1703 = v1;
    match m1703 {
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
            f105(f106(v0.clone(), v14, v15), v14, v15, 0u64, v16, 2u64, f107(v0), 3u64, v17)
        }
        Adt9::C4(v18, v19, v20, v21, v22, v23, v24) => {
            let v25: u64 = f108(v0.clone(), v18);
            f105(f109(v0, v18), v25, v25, v19, v20, v21, v22, v23, v24)
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

pub fn f62(v0: u64, v1: Adt3, v2: Adt3, v3: Adt4) -> R<Adt9> {
    let m1704 = v3;
    match m1704 {
        Adt4::C0 => {
            Ok(Adt9::C1(false, f110(v0, v1)?, f110(v0, v2)?))
        }
        Adt4::C1 => {
            Ok(Adt9::C1(false, f112(v0, v1, v2.clone())?, f110(v0, v2)?))
        }
        Adt4::C2(v6, v7) => {
            f113(v0, v1, v7, v2, v6)
        }
        Adt4::C3(v8, v9, v10, v11, v12) => {
            f114(v0, v1, v2, v8, v9, v10, v11, v12)
        }
        Adt4::C4(v13) => {
            f115(v0, v1, v2, v13)
        }
        Adt4::C5(v14, v15, v16, v17) => {
            f114(v0, v1, v2, v14, v15, v16, v17, true)
        }
        Adt4::C6(v18, v19, v20) => {
            f116(v0, v1, v2, v18, v19, v20)
        }
        Adt4::C7(v21) => {
            f117(v0, v1, v2, v21)
        }
        Adt4::C8(v22) => {
            f118(v22, v0, v1, v2)
        }
        Adt4::C9(_, _) => {
            Ok(Adt9::C1(true, 0u64, f110(v0, v2)?))
        }
        Adt4::C10(v25, v26, v27) => {
            f119(v0, v1, v2, v25, v26, v27)
        }
        Adt4::C11(v28, v29) => {
            Ok(Adt9::C3(v28, v29, f110(v0, v1)?, f110(v0, v2)?))
        }
        Adt4::C12(v30, v31, v32) => {
            f120(v0, v1, v2, v30, v31, v32)
        }
        Adt4::C13(_, v34, _, _) => {
            Ok(Adt9::C3(v34, v34, f110(v0, v1)?, f110(v0, v2)?))
        }
        Adt4::C14(_, v38, v39) => {
            Ok(Adt9::C3(v38, v39, f110(v0, v1)?, f110(v0, v2)?))
        }
        Adt4::C15 => {
            f121(v0, v1, v2)
        }
        Adt4::C16(v40, v41) => {
            f122(v0, v1, v2, v40, v41)
        }
        Adt4::C17 => {
            Ok(Adt9::C0(Adt10::C1))
        }
        Adt4::C18(v42, v43) => {
            f123(v0, v1, v43, v42)
        }
        Adt4::C19(_, _) => {
            Ok(Adt9::C0(Adt10::C1))
        }
        Adt4::C20(_, _) => {
            Ok(Adt9::C0(Adt10::C1))
        }
        Adt4::C21 => {
            Ok(Adt9::C1(false, f110(f111(v0, match v2.clone() {
                Adt3::C0(h1705, _, _, _, _, _, _, _) => {
                    h1705
                }
            }, match v2.clone() {
                Adt3::C0(_, h1706, _, _, _, _, _, _) => {
                    h1706
                }
            })?, v1)?, f110(v0, v2)?))
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
            f124(v0, v48, v49, v50)
        }
        Adt4::C26(v53, _, _, _) => {
            let a1707 = v53;
            let a1708 = 64u64;
            if if if nat_eq(a1707, a1708) {
                true
            } else {
                let a1709 = v53;
                let a1710 = 65u64;
                nat_eq(a1709, a1710)
            } {
                true
            } else {
                let a1711 = v53;
                let a1712 = 66u64;
                nat_eq(a1711, a1712)
            } {
                Ok(Adt9::C0(Adt10::C5(64u64)))
            } else {
                Ok(Adt9::C0(Adt10::C1))
            }
        }
        Adt4::C27(v52) => {
            f68(v0, v1, v2, v52)
        }
        Adt4::C28(v51) => {
            f67(v0, v1, v2, v51, 0u64)
        }
    }
}

pub fn f63(v0: u64, v1: Adt3, v2: Adt3, v3: u64) -> R<Adt9> {
    let v4: u64 = f100(match v1.clone() {
        Adt3::C0(h1713, _, _, _, _, _, _, _) => {
            h1713
        }
    })?;
    let v5: u64 = {
        let a1714 = v0;
        let a1715 = v4;
        let a1716 = 0u64;
        nat_quot(a1714, a1715, a1716)
    };
    let v6: u64 = {
        let a1717 = v0;
        let a1718 = v4;
        let a1719 = 0u64;
        nat_rem(a1717, a1718, a1719)
    };
    let a1720 = v6;
    let a1721 = 0u64;
    if nat_eq(a1720, a1721) {
        Ok(Adt9::C4(19u64, 2u64, 4286578688u64, 2u64, 0u64, 1u64, 0u64))
    } else {
        let a1722 = v6;
        let a1723 = 1u64;
        if nat_eq(a1722, a1723) {
            Ok(Adt9::C4(19u64, 2u64, 0u64, 2u64, 0u64, 1u64, 1u64))
        } else {
            let a1726 = {
                let a1724 = v6;
                let a1725 = 1u64;
                nat_add(a1724, a1725)?
            };
            let a1727 = v4;
            if nat_eq(a1726, a1727) {
                Ok(Adt9::C4(19u64, 1u64, 1u64, 2u64, 0u64, 3u64, f110(v5, v2)?))
            } else {
                let v7: u64 = {
                    let a1728 = v6;
                    let a1729 = 2u64;
                    nat_sub(a1728, a1729)
                };
                let v8: u64 = {
                    let a1730 = v7;
                    let a1731 = 3u64;
                    let a1732 = 0u64;
                    nat_quot(a1730, a1731, a1732)
                };
                let v9: u64 = {
                    let a1733 = v7;
                    let a1734 = 3u64;
                    let a1735 = 0u64;
                    nat_rem(a1733, a1734, a1735)
                };
                let v10: u64 = f110({
                    let a1739 = {
                        let a1737 = v5;
                        let a1738 = match v1.clone() {
                            Adt3::C0(h1736, _, _, _, _, _, _, _) => {
                                h1736
                            }
                        };
                        nat_mul(a1737, a1738)?
                    };
                    let a1740 = v8;
                    nat_add(a1739, a1740)?
                }, v1)?;
                let a1741 = v9;
                let a1742 = 0u64;
                if nat_eq(a1741, a1742) {
                    Ok(Adt9::C4(16u64, 1u64, 0u64, 0u64, v10, 1u64, 2u64))
                } else {
                    let a1743 = v9;
                    let a1744 = 1u64;
                    if nat_eq(a1743, a1744) {
                        let a1745 = v3;
                        let a1746 = 2u64;
                        if nat_eq(a1745, a1746) {
                            Ok(Adt9::C4(19u64, 1u64, 0u64, 2u64, 0u64, 1u64, 0u64))
                        } else {
                            Ok(Adt9::C4(19u64, 0u64, v10, 2u64, 0u64, 1u64, 0u64))
                        }
                    } else {
                        let a1747 = v3;
                        let a1748 = 2u64;
                        if if nat_eq(a1747, a1748) {
                            true
                        } else {
                            let a1749 = v3;
                            let a1750 = 3u64;
                            nat_eq(a1749, a1750)
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

pub fn f64(v0: u64, v1: Adt3, v2: Adt3, v3: Adt3, v4: u64) -> R<Adt9> {
    let v5: u64 = {
        let a1752 = 2u64;
        let a1753 = match v1.clone() {
            Adt3::C0(_, h1751, _, _, _, _, _, _) => {
                h1751
            }
        };
        nat_mul(a1752, a1753)?
    };
    let a1754 = v0;
    let a1755 = v5;
    if nat_lt(a1754, a1755) {
        let a1759 = {
            let a1756 = v0;
            let a1757 = 2u64;
            let a1758 = 0u64;
            nat_rem(a1756, a1757, a1758)
        };
        let a1760 = 0u64;
        if nat_eq(a1759, a1760) {
            Ok(Adt9::C4(19u64, 4u64, f110({
                let a1761 = v0;
                let a1762 = 2u64;
                let a1763 = 0u64;
                nat_quot(a1761, a1762, a1763)
            }, v3)?, 2u64, 0u64, 1u64, 2u64))
        } else {
            let a1764 = v4;
            let a1765 = 2147483648u64;
            if if nat_lt(a1764, a1765) {
                let a1767 = v4;
                let a1768 = match v2 {
                    Adt3::C0(_, h1766, _, _, _, _, _, _) => {
                        h1766
                    }
                };
                nat_lt(a1767, a1768)
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
            let a1771 = match v2.clone() {
                Adt3::C0(h1769, _, _, _, _, _, _, _) => {
                    h1769
                }
            };
            let a1772 = match v2.clone() {
                Adt3::C0(_, h1770, _, _, _, _, _, _) => {
                    h1770
                }
            };
            nat_mul(a1771, a1772)?
        };
        let a1775 = {
            let a1773 = v0;
            let a1774 = v5;
            nat_sub(a1773, a1774)
        };
        let a1776 = v6;
        if nat_lt(a1775, a1776) {
            Ok(Adt9::C4(19u64, 2u64, 0u64, 2u64, 0u64, 3u64, f110({
                let a1777 = v0;
                let a1778 = v5;
                nat_sub(a1777, a1778)
            }, v2)?))
        } else {
            let v7: u64 = {
                let a1781 = {
                    let a1779 = v0;
                    let a1780 = v5;
                    nat_sub(a1779, a1780)
                };
                let a1782 = v6;
                nat_sub(a1781, a1782)
            };
            let v8: u64 = {
                let a1786 = 1u64;
                let a1787 = {
                    let a1784 = 2u64;
                    let a1785 = match v1.clone() {
                        Adt3::C0(h1783, _, _, _, _, _, _, _) => {
                            h1783
                        }
                    };
                    nat_mul(a1784, a1785)?
                };
                nat_add(a1786, a1787)?
            };
            let v9: u64 = {
                let a1788 = v7;
                let a1789 = v8;
                let a1790 = 0u64;
                nat_quot(a1788, a1789, a1790)
            };
            let v10: u64 = {
                let a1791 = v7;
                let a1792 = v8;
                let a1793 = 0u64;
                nat_rem(a1791, a1792, a1793)
            };
            let a1794 = v10;
            let a1795 = 0u64;
            if nat_eq(a1794, a1795) {
                Ok(Adt9::C4(19u64, 4u64, f110(v9, v3)?, 2u64, 0u64, 1u64, 2u64))
            } else {
                let a1799 = {
                    let a1796 = v10;
                    let a1797 = 2u64;
                    let a1798 = 0u64;
                    nat_rem(a1796, a1797, a1798)
                };
                let a1800 = 1u64;
                if nat_eq(a1799, a1800) {
                    Ok(Adt9::C4(6u64, 0u64, f110({
                        let a1809 = {
                            let a1802 = v9;
                            let a1803 = match v1.clone() {
                                Adt3::C0(h1801, _, _, _, _, _, _, _) => {
                                    h1801
                                }
                            };
                            nat_mul(a1802, a1803)?
                        };
                        let a1810 = {
                            let a1806 = {
                                let a1804 = v10;
                                let a1805 = 1u64;
                                nat_sub(a1804, a1805)
                            };
                            let a1807 = 2u64;
                            let a1808 = 0u64;
                            nat_quot(a1806, a1807, a1808)
                        };
                        nat_add(a1809, a1810)?
                    }, v1)?, 2u64, 0u64, 1u64, 0u64))
                } else {
                    Ok(Adt9::C4(4u64, 3u64, f110({
                        let a1819 = {
                            let a1812 = v4;
                            let a1813 = match v1.clone() {
                                Adt3::C0(h1811, _, _, _, _, _, _, _) => {
                                    h1811
                                }
                            };
                            nat_mul(a1812, a1813)?
                        };
                        let a1820 = {
                            let a1816 = {
                                let a1814 = v10;
                                let a1815 = 1u64;
                                nat_sub(a1814, a1815)
                            };
                            let a1817 = 2u64;
                            let a1818 = 0u64;
                            nat_quot(a1816, a1817, a1818)
                        };
                        nat_add(a1819, a1820)?
                    }, v2.clone())?, 1u64, 0u64, 3u64, f110({
                        let a1829 = {
                            let a1822 = v4;
                            let a1823 = match v1 {
                                Adt3::C0(h1821, _, _, _, _, _, _, _) => {
                                    h1821
                                }
                            };
                            nat_mul(a1822, a1823)?
                        };
                        let a1830 = {
                            let a1826 = {
                                let a1824 = v10;
                                let a1825 = 1u64;
                                nat_sub(a1824, a1825)
                            };
                            let a1827 = 2u64;
                            let a1828 = 0u64;
                            nat_quot(a1826, a1827, a1828)
                        };
                        nat_add(a1829, a1830)?
                    }, v2)?))
                }
            }
        }
    }
}

pub fn f65(v0: u64, v1: u64, v2: Adt3, v3: Adt3, v4: Adt3, v5: u64) -> R<Adt9> {
    let a1831 = v1;
    let a1832 = f125(v2.clone(), v4.clone())?;
    if nat_lt(a1831, a1832) {
        let a1835 = v1;
        let a1836 = {
            let a1833 = 2u64;
            let a1834 = f101(v4.clone())?;
            nat_mul(a1833, a1834)?
        };
        if nat_lt(a1835, a1836) {
            let a1840 = {
                let a1837 = v1;
                let a1838 = 2u64;
                let a1839 = 0u64;
                nat_rem(a1837, a1838, a1839)
            };
            let a1841 = 0u64;
            if nat_eq(a1840, a1841) {
                f126({
                    let a1842 = v1;
                    let a1843 = 2u64;
                    let a1844 = 0u64;
                    nat_quot(a1842, a1843, a1844)
                }, v4)
            } else {
                if f127(match v2 {
                    Adt3::C0(_, h1845, _, _, _, _, _, _) => {
                        h1845
                    }
                }, v5) {
                    Ok(Adt9::C4(19u64, 1u64, 2u64, 2u64, 0u64, 1u64, 2u64))
                } else {
                    Ok(Adt9::C0(Adt10::C1))
                }
            }
        } else {
            let v6: u64 = {
                let a1848 = v1;
                let a1849 = {
                    let a1846 = 2u64;
                    let a1847 = f101(v4.clone())?;
                    nat_mul(a1846, a1847)?
                };
                nat_sub(a1848, a1849)
            };
            let v7: u64 = {
                let a1853 = v6;
                let a1854 = {
                    let a1851 = 1u64;
                    let a1852 = match v2.clone() {
                        Adt3::C0(h1850, _, _, _, _, _, _, _) => {
                            h1850
                        }
                    };
                    nat_add(a1851, a1852)?
                };
                let a1855 = 0u64;
                nat_quot(a1853, a1854, a1855)
            };
            let v8: u64 = {
                let a1859 = v6;
                let a1860 = {
                    let a1857 = 1u64;
                    let a1858 = match v2.clone() {
                        Adt3::C0(h1856, _, _, _, _, _, _, _) => {
                            h1856
                        }
                    };
                    nat_add(a1857, a1858)?
                };
                let a1861 = 0u64;
                nat_rem(a1859, a1860, a1861)
            };
            let a1862 = v8;
            let a1863 = 0u64;
            if nat_eq(a1862, a1863) {
                f126(v7, v4)
            } else {
                f128(v0, v7, {
                    let a1864 = v8;
                    let a1865 = 1u64;
                    nat_sub(a1864, a1865)
                }, v2, v3, v4, v5)
            }
        }
    } else {
        Ok(Adt9::C0(Adt10::C1))
    }
}

pub fn f66(v0: u64, v1: Adt3, v2: Adt3, v3: Adt3, v4: Adt3, v5: u64) -> R<Adt9> {
    let v6: u64 = {
        let a1868 = match v4.clone() {
            Adt3::C0(h1866, _, _, _, _, _, _, _) => {
                h1866
            }
        };
        let a1869 = match v4.clone() {
            Adt3::C0(_, h1867, _, _, _, _, _, _) => {
                h1867
            }
        };
        nat_mul(a1868, a1869)?
    };
    let v7: u64 = {
        let a1870 = 2u64;
        let a1871 = v6;
        nat_mul(a1870, a1871)?
    };
    let v8: u64 = f129(match v3.clone() {
        Adt3::C0(_, h1872, _, _, _, _, _, _) => {
            h1872
        }
    }, match v3.clone() {
        Adt3::C0(_, _, h1873, _, _, _, _, _) => {
            h1873
        }
    }, match v3.clone() {
        Adt3::C0(_, _, _, h1874, _, _, _, _) => {
            h1874
        }
    });
    let a1875 = v0;
    let a1876 = v7;
    if nat_lt(a1875, a1876) {
        let a1880 = {
            let a1877 = v0;
            let a1878 = 2u64;
            let a1879 = 0u64;
            nat_rem(a1877, a1878, a1879)
        };
        let a1881 = 0u64;
        if nat_eq(a1880, a1881) {
            Ok(Adt9::C4(19u64, 5u64, f110({
                let a1882 = v0;
                let a1883 = 2u64;
                let a1884 = 0u64;
                nat_quot(a1882, a1883, a1884)
            }, v4)?, 2u64, 0u64, 1u64, 2u64))
        } else {
            if f130(v5, v8) {
                Ok(Adt9::C4(19u64, 1u64, 2u64, 2u64, 0u64, 1u64, 2u64))
            } else {
                Ok(Adt9::C0(Adt10::C1))
            }
        }
    } else {
        let v9: u64 = {
            let a1885 = v0;
            let a1886 = v7;
            nat_sub(a1885, a1886)
        };
        let v10: u64 = {
            let a1890 = v9;
            let a1891 = {
                let a1888 = 1u64;
                let a1889 = match v1.clone() {
                    Adt3::C0(h1887, _, _, _, _, _, _, _) => {
                        h1887
                    }
                };
                nat_add(a1888, a1889)?
            };
            let a1892 = 0u64;
            nat_quot(a1890, a1891, a1892)
        };
        let v11: u64 = {
            let a1896 = v9;
            let a1897 = {
                let a1894 = 1u64;
                let a1895 = match v1.clone() {
                    Adt3::C0(h1893, _, _, _, _, _, _, _) => {
                        h1893
                    }
                };
                nat_add(a1894, a1895)?
            };
            let a1898 = 0u64;
            nat_rem(a1896, a1897, a1898)
        };
        let a1899 = v11;
        let a1900 = 0u64;
        if nat_eq(a1899, a1900) {
            Ok(Adt9::C4(19u64, 5u64, f110({
                let a1901 = v10;
                let a1902 = v6;
                let a1903 = 0u64;
                nat_rem(a1901, a1902, a1903)
            }, v4)?, 2u64, 0u64, 1u64, 2u64))
        } else {
            if f130(v5, v8) {
                let v12: u64 = {
                    let a1909 = {
                        let a1905 = v10;
                        let a1906 = match v1.clone() {
                            Adt3::C0(h1904, _, _, _, _, _, _, _) => {
                                h1904
                            }
                        };
                        nat_mul(a1905, a1906)?
                    };
                    let a1910 = {
                        let a1907 = v11;
                        let a1908 = 1u64;
                        nat_sub(a1907, a1908)
                    };
                    nat_add(a1909, a1910)?
                };
                let a1911 = v5;
                let a1912 = 4294967295u64;
                if nat_eq(a1911, a1912) {
                    Ok(Adt9::C1(false, f110(v12, v1)?, f110(v12, v2)?))
                } else {
                    Ok(Adt9::C4(8u64, 0u64, f110(v12, v1)?, 4u64, f110({
                        let a1918 = {
                            let a1914 = v5;
                            let a1915 = match v3.clone() {
                                Adt3::C0(h1913, _, _, _, _, _, _, _) => {
                                    h1913
                                }
                            };
                            nat_mul(a1914, a1915)?
                        };
                        let a1919 = {
                            let a1916 = v11;
                            let a1917 = 1u64;
                            nat_sub(a1916, a1917)
                        };
                        nat_add(a1918, a1919)?
                    }, v3)?, 3u64, f110(v12, v2)?))
                }
            } else {
                Ok(Adt9::C0(Adt10::C1))
            }
        }
    }
}

pub fn f67(v0: u64, v1: Adt3, v2: Adt3, v3: Adt7, v4: u64) -> R<Adt9> {
    let v5: u64 = f85(v3.clone())?;
    let v6: u64 = {
        let a1920 = v0;
        let a1921 = v5;
        let a1922 = 0u64;
        nat_quot(a1920, a1921, a1922)
    };
    let v7: u64 = {
        let a1923 = v0;
        let a1924 = v5;
        let a1925 = 0u64;
        nat_rem(a1923, a1924, a1925)
    };
    let v8: u64 = f131(v3.clone())?;
    let a1926 = v7;
    let a1927 = 0u64;
    if nat_eq(a1926, a1927) {
        Ok(Adt9::C4(19u64, 2u64, if match v3 {
            Adt7::C0(_, _, _, h1928, _) => {
                h1928
            }
        } {
            4286578687u64
        } else {
            0u64
        }, 2u64, 0u64, 1u64, 0u64))
    } else {
        let a1932 = v5;
        let a1933 = {
            let a1930 = v7;
            let a1931 = if match v3.clone() {
                Adt7::C0(_, _, _, h1929, _) => {
                    h1929
                }
            } {
                1u64
            } else {
                2u64
            };
            nat_add(a1930, a1931)?
        };
        if nat_le(a1932, a1933) {
            if match v3.clone() {
                Adt7::C0(_, _, _, h1934, _) => {
                    h1934
                }
            } {
                Ok(Adt9::C4(19u64, 1u64, 0u64, 2u64, 0u64, 3u64, f110(v6, v2)?))
            } else {
                let a1937 = {
                    let a1935 = v7;
                    let a1936 = 2u64;
                    nat_add(a1935, a1936)?
                };
                let a1938 = v5;
                if nat_eq(a1937, a1938) {
                    let v9: u64 = if match v3.clone() {
                        Adt7::C0(_, _, h1939, _, _) => {
                            h1939
                        }
                    } {
                        let a1946 = {
                            let m1942 = match v3.clone() {
                                Adt7::C0(h1941, _, _, _, _) => {
                                    h1941
                                }
                            };
                            match m1942 {
                                Adt6::C0(h1940, _, _, _) => {
                                    h1940
                                }
                            }
                        };
                        let a1947 = {
                            let m1945 = match v3 {
                                Adt7::C0(_, h1944, _, _, _) => {
                                    h1944
                                }
                            };
                            match m1945 {
                                Adt6::C0(h1943, _, _, _) => {
                                    h1943
                                }
                            }
                        };
                        nat_mul(a1946, a1947)?
                    } else {
                        f132(match v3 {
                            Adt7::C0(h1948, _, _, _, _) => {
                                h1948
                            }
                        }, match v1 {
                            Adt3::C0(h1949, _, _, _, _, _, _, _) => {
                                h1949
                            }
                        }, f59(v6, v2, 0u64))?
                    };
                    Ok(Adt9::C4(3u64, 2u64, {
                        let a1950 = v9;
                        let a1951 = 0u64;
                        if nat_eq(a1950, a1951) {
                            1u64
                        } else {
                            v9
                        }
                    }, 2u64, 0u64, 1u64, 1u64))
                } else {
                    Ok(Adt9::C4(5u64, 1u64, 0u64, 1u64, 1u64, 3u64, f110(v6, v2)?))
                }
            }
        } else {
            let v10: u64 = {
                let a1954 = {
                    let a1952 = v7;
                    let a1953 = 1u64;
                    nat_sub(a1952, a1953)
                };
                let a1955 = v8;
                let a1956 = 0u64;
                nat_rem(a1954, a1955, a1956)
            };
            let m1962 = f133(v3.clone(), v1, v2, v6, {
                let a1959 = {
                    let a1957 = v7;
                    let a1958 = 1u64;
                    nat_sub(a1957, a1958)
                };
                let a1960 = v8;
                let a1961 = 0u64;
                nat_quot(a1959, a1960, a1961)
            })?;
            match m1962 {
                None => {
                    Ok(Adt9::C4(19u64, 1u64, 0u64, 2u64, 0u64, 1u64, 0u64))
                }
                Some(v11) => {
                    let a1964 = match v3.clone() {
                        Adt7::C0(_, _, _, _, h1963) => {
                            h1963
                        }
                    };
                    let a1965 = 1u64;
                    if if nat_eq(a1964, a1965) {
                        let a1966 = v10;
                        let a1967 = 0u64;
                        nat_eq(a1966, a1967)
                    } else {
                        false
                    } {
                        Ok(Adt9::C4(6u64, 0u64, v11, 2u64, 0u64, 1u64, 1u64))
                    } else {
                        if match v3.clone() {
                            Adt7::C0(_, _, _, h1968, _) => {
                                h1968
                            }
                        } {
                            let a1971 = {
                                let a1969 = v10;
                                let a1970 = 1u64;
                                nat_add(a1969, a1970)?
                            };
                            let a1972 = v8;
                            if nat_eq(a1971, a1972) {
                                let a1973 = v4;
                                let a1974 = 2u64;
                                if nat_eq(a1973, a1974) {
                                    Ok(Adt9::C4(19u64, 1u64, 0u64, 2u64, 0u64, 1u64, 0u64))
                                } else {
                                    Ok(Adt9::C4(19u64, {
                                        let a1976 = match v3.clone() {
                                            Adt7::C0(_, _, _, _, h1975) => {
                                                h1975
                                            }
                                        };
                                        let a1977 = 1u64;
                                        if nat_eq(a1976, a1977) {
                                            1u64
                                        } else {
                                            0u64
                                        }
                                    }, {
                                        let a1979 = match v3 {
                                            Adt7::C0(_, _, _, _, h1978) => {
                                                h1978
                                            }
                                        };
                                        let a1980 = 1u64;
                                        if nat_eq(a1979, a1980) {
                                            1u64
                                        } else {
                                            v11
                                        }
                                    }, 2u64, 0u64, 1u64, 0u64))
                                }
                            } else {
                                Ok(Adt9::C4(16u64, 1u64, 0u64, {
                                    let a1982 = match v3.clone() {
                                        Adt7::C0(_, _, _, _, h1981) => {
                                            h1981
                                        }
                                    };
                                    let a1983 = 1u64;
                                    if nat_eq(a1982, a1983) {
                                        1u64
                                    } else {
                                        0u64
                                    }
                                }, {
                                    let a1985 = match v3 {
                                        Adt7::C0(_, _, _, _, h1984) => {
                                            h1984
                                        }
                                    };
                                    let a1986 = 1u64;
                                    if nat_eq(a1985, a1986) {
                                        1u64
                                    } else {
                                        v11
                                    }
                                }, 1u64, 2u64))
                            }
                        } else {
                            Ok(Adt9::C4(4u64, 1u64, 0u64, {
                                let a1988 = match v3.clone() {
                                    Adt7::C0(_, _, _, _, h1987) => {
                                        h1987
                                    }
                                };
                                let a1989 = 1u64;
                                if nat_eq(a1988, a1989) {
                                    1u64
                                } else {
                                    0u64
                                }
                            }, {
                                let a1991 = match v3 {
                                    Adt7::C0(_, _, _, _, h1990) => {
                                        h1990
                                    }
                                };
                                let a1992 = 1u64;
                                if nat_eq(a1991, a1992) {
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

pub fn f68(v0: u64, v1: Adt3, v2: Adt3, v3: Adt5) -> R<Adt9> {
    let v4: u64 = f59(v0, v2.clone(), 0u64);
    let v5: u64 = f59(v0, v2.clone(), 3u64);
    let v6: u64 = {
        let a1994 = v5;
        let a1995 = match v3.clone() {
            Adt5::C0(_, _, _, _, _, _, h1993, _) => {
                h1993
            }
        };
        let a1996 = 0u64;
        nat_rem(a1994, a1995, a1996)
    };
    let v7: u64 = {
        let a1998 = v5;
        let a1999 = match v3.clone() {
            Adt5::C0(_, _, _, _, _, _, h1997, _) => {
                h1997
            }
        };
        let a2000 = 0u64;
        nat_quot(a1998, a1999, a2000)
    };
    let v8: Option<u64> = f60(match v3.clone() {
        Adt5::C0(h2001, _, _, _, _, _, _, _) => {
            h2001
        }
    }, match v1.clone() {
        Adt3::C0(h2002, _, _, _, _, _, _, _) => {
            h2002
        }
    }, f59(v0, v2.clone(), 1u64), {
        let a2006 = v4;
        let a2007 = {
            let m2005 = match v3.clone() {
                Adt5::C0(h2004, _, _, _, _, _, _, _) => {
                    h2004
                }
            };
            match m2005 {
                Adt6::C0(h2003, _, _, _) => {
                    h2003
                }
            }
        };
        let a2008 = 0u64;
        nat_rem(a2006, a2007, a2008)
    })?;
    let v9: u64 = {
        let a2012 = v4;
        let a2013 = {
            let m2011 = match v3.clone() {
                Adt5::C0(h2010, _, _, _, _, _, _, _) => {
                    h2010
                }
            };
            match m2011 {
                Adt6::C0(h2009, _, _, _) => {
                    h2009
                }
            }
        };
        let a2014 = 0u64;
        nat_quot(a2012, a2013, a2014)
    };
    let v10: Option<u64> = f60(match v3.clone() {
        Adt5::C0(_, h2015, _, _, _, _, _, _) => {
            h2015
        }
    }, match v1.clone() {
        Adt3::C0(_, h2016, _, _, _, _, _, _) => {
            h2016
        }
    }, f59(v0, v2.clone(), 2u64), {
        let a2020 = v9;
        let a2021 = {
            let m2019 = match v3.clone() {
                Adt5::C0(_, h2018, _, _, _, _, _, _) => {
                    h2018
                }
            };
            match m2019 {
                Adt6::C0(h2017, _, _, _) => {
                    h2017
                }
            }
        };
        let a2022 = 0u64;
        nat_rem(a2020, a2021, a2022)
    })?;
    let v11: u64 = {
        let a2026 = v9;
        let a2027 = {
            let m2025 = match v3.clone() {
                Adt5::C0(_, h2024, _, _, _, _, _, _) => {
                    h2024
                }
            };
            match m2025 {
                Adt6::C0(h2023, _, _, _) => {
                    h2023
                }
            }
        };
        let a2028 = 0u64;
        nat_quot(a2026, a2027, a2028)
    };
    let v12: Option<u64> = f60(match v3.clone() {
        Adt5::C0(_, _, h2029, _, _, _, _, _) => {
            h2029
        }
    }, match v1.clone() {
        Adt3::C0(_, _, h2030, _, _, _, _, _) => {
            h2030
        }
    }, v6, {
        let a2034 = v11;
        let a2035 = {
            let m2033 = match v3.clone() {
                Adt5::C0(_, _, h2032, _, _, _, _, _) => {
                    h2032
                }
            };
            match m2033 {
                Adt6::C0(h2031, _, _, _) => {
                    h2031
                }
            }
        };
        let a2036 = 0u64;
        nat_rem(a2034, a2035, a2036)
    })?;
    let v13: u64 = {
        let a2040 = v11;
        let a2041 = {
            let m2039 = match v3.clone() {
                Adt5::C0(_, _, h2038, _, _, _, _, _) => {
                    h2038
                }
            };
            match m2039 {
                Adt6::C0(h2037, _, _, _) => {
                    h2037
                }
            }
        };
        let a2042 = 0u64;
        nat_quot(a2040, a2041, a2042)
    };
    let v17: Adt12 = {
        let m2043 = v8;
        match m2043 {
            None => {
                Adt12::C2(0u64)
            }
            Some(v14) => {
                let m2044 = v10;
                match m2044 {
                    None => {
                        Adt12::C2(0u64)
                    }
                    Some(v15) => {
                        let m2045 = v12;
                        match m2045 {
                            None => {
                                Adt12::C2(0u64)
                            }
                            Some(v16) => {
                                Adt12::C0(1u64, {
                                    let a2054 = {
                                        let a2049 = f58(v1, v14, v15, v16, 0u64)?;
                                        let a2050 = {
                                            let a2047 = v7;
                                            let a2048 = match v3.clone() {
                                                Adt5::C0(_, _, _, _, h2046, _, _, _) => {
                                                    h2046
                                                }
                                            };
                                            nat_mul(a2047, a2048)?
                                        };
                                        nat_add(a2049, a2050)?
                                    };
                                    let a2055 = {
                                        let a2052 = v13;
                                        let a2053 = match v3.clone() {
                                            Adt5::C0(_, _, _, h2051, _, _, _, _) => {
                                                h2051
                                            }
                                        };
                                        nat_mul(a2052, a2053)?
                                    };
                                    nat_add(a2054, a2055)?
                                })
                            }
                        }
                    }
                }
            }
        }
    };
    Ok(Adt9::C7(Adt11::C2(match v3.clone() {
        Adt5::C0(_, _, _, _, _, h2056, _, _) => {
            h2056
        }
    }, match v3.clone() {
        Adt5::C0(_, _, _, _, _, _, _, h2057) => {
            h2057
        }
    }, true), v17, Adt12::C2(0u64), Adt13::C1(f110(v0, v2)?), {
        let a2059 = match v3.clone() {
            Adt5::C0(_, _, _, _, _, h2058, _, _) => {
                h2058
            }
        };
        let a2060 = 0u64;
        if nat_eq(a2059, a2060) {
            4u64
        } else {
            2u64
        }
    }, {
        let a2062 = match v3 {
            Adt5::C0(_, _, _, _, _, _, _, h2061) => {
                h2061
            }
        };
        let a2063 = 0u64;
        if nat_eq(a2062, a2063) {
            4u64
        } else {
            2u64
        }
    }))
}

pub fn f69(v0: u64, v1: Adt3, v2: Adt3, v3: u64, v4: u64, v5: u64, v6: u64, v7: u64) -> R<Adt9> {
    let v8: u64 = {
        let a2066 = v6;
        let a2067 = {
            let a2064 = 3u64;
            let a2065 = v5;
            nat_add(a2064, a2065)?
        };
        nat_sub(a2066, a2067)
    };
    let a2069 = v0;
    let a2070 = f29(v3, match v1.clone() {
        Adt3::C0(h2068, _, _, _, _, _, _, _) => {
            h2068
        }
    }, v5, v8)?;
    if nat_lt(a2069, a2070) {
        let v9: u64 = f80(match v1.clone() {
            Adt3::C0(h2071, _, _, _, _, _, _, _) => {
                h2071
            }
        }, v5, v8)?;
        let v10: u64 = f81(v3, match v1.clone() {
            Adt3::C0(h2072, _, _, _, _, _, _, _) => {
                h2072
            }
        }, v5)?;
        let a2073 = v0;
        let a2074 = v9;
        if nat_lt(a2073, a2074) {
            f134(v0, v2, f23(v3), {
                let a2075 = 3u64;
                let a2076 = v5;
                nat_add(a2075, a2076)?
            }, v7)
        } else {
            let a2079 = v0;
            let a2080 = {
                let a2077 = v9;
                let a2078 = v10;
                nat_add(a2077, a2078)?
            };
            if nat_lt(a2079, a2080) {
                let v11: u64 = {
                    let a2081 = v0;
                    let a2082 = v9;
                    nat_sub(a2081, a2082)
                };
                let v12: u64 = f135(v3, match v1.clone() {
                    Adt3::C0(h2083, _, _, _, _, _, _, _) => {
                        h2083
                    }
                })?;
                f136({
                    let a2084 = v11;
                    let a2085 = v12;
                    let a2086 = 0u64;
                    nat_rem(a2084, a2085, a2086)
                }, {
                    let a2087 = v11;
                    let a2088 = v12;
                    let a2089 = 0u64;
                    nat_quot(a2087, a2088, a2089)
                }, v1, v3, v4, v7)
            } else {
                f137({
                    let a2092 = v0;
                    let a2093 = {
                        let a2090 = v9;
                        let a2091 = v10;
                        nat_add(a2090, a2091)?
                    };
                    nat_sub(a2092, a2093)
                }, v1, v2, v3)
            }
        }
    } else {
        Ok(Adt9::C0(Adt10::C1))
    }
}

pub fn f70(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64) -> R<Adt9> {
    let a2094 = v0;
    let a2095 = f15(v1)?;
    if nat_lt(a2094, a2095) {
        let a2096 = v0;
        let a2097 = 4u64;
        if nat_lt(a2096, a2097) {
            let a2098 = v0;
            let a2099 = 0u64;
            if nat_eq(a2098, a2099) {
                Ok(Adt9::C4(16u64, 2u64, v2, 2u64, v3, 1u64, 2u64))
            } else {
                let a2100 = v0;
                let a2101 = 1u64;
                if nat_eq(a2100, a2101) {
                    let a2102 = v5;
                    let a2103 = 0u64;
                    if nat_eq(a2102, a2103) {
                        Ok(Adt9::C4(9u64, 2u64, v3, 2u64, v2, 1u64, 2u64))
                    } else {
                        Ok(Adt9::C0(Adt10::C1))
                    }
                } else {
                    let a2104 = v0;
                    let a2105 = 2u64;
                    if nat_eq(a2104, a2105) {
                        Ok(Adt9::C4(11u64, 1u64, 2u64, 2u64, v4, 1u64, 2u64))
                    } else {
                        Ok(Adt9::C4(20u64, 1u64, 2u64, 2u64, 10u64, 1u64, 2u64))
                    }
                }
            }
        } else {
            let a2106 = v0;
            let a2107 = 4u64;
            if if nat_eq(a2106, a2107) {
                let a2108 = f138(v5)?;
                let a2109 = v1;
                !nat_eq(a2108, a2109)
            } else {
                false
            } {
                Ok(Adt9::C0(Adt10::C1))
            } else {
                let v6: u64 = {
                    let a2112 = {
                        let a2110 = v0;
                        let a2111 = 4u64;
                        nat_sub(a2110, a2111)
                    };
                    let a2113 = 3u64;
                    let a2114 = 0u64;
                    nat_quot(a2112, a2113, a2114)
                };
                let v7: u64 = {
                    let a2117 = {
                        let a2115 = v0;
                        let a2116 = 4u64;
                        nat_sub(a2115, a2116)
                    };
                    let a2118 = 3u64;
                    let a2119 = 0u64;
                    nat_rem(a2117, a2118, a2119)
                };
                let a2120 = v7;
                let a2121 = 0u64;
                if nat_eq(a2120, a2121) {
                    Ok(Adt9::C4(3u64, 2u64, v6, 2u64, 0u64, 1u64, 0u64))
                } else {
                    let a2122 = v7;
                    let a2123 = 1u64;
                    if nat_eq(a2122, a2123) {
                        Ok(Adt9::C4(10u64, 2u64, v4, 1u64, 0u64, 1u64, 0u64))
                    } else {
                        Ok(Adt9::C4(8u64, 2u64, v2, 1u64, 0u64, 3u64, f14(v6)?))
                    }
                }
            }
        }
    } else {
        Ok(Adt9::C0(Adt10::C1))
    }
}

pub fn f71(v0: bool, v1: u64, v2: u64, v3: u64, v4: u64) -> Adt11 {
    let a2124 = v1;
    let a2125 = 0u64;
    if nat_eq(a2124, a2125) {
        if v0 {
            Adt11::C2(v3, v4, false)
        } else {
            Adt11::C0(v2)
        }
    } else {
        let a2126 = v1;
        let a2127 = 1u64;
        if nat_eq(a2126, a2127) {
            Adt11::C1(v2)
        } else {
            let a2128 = v1;
            let a2129 = 2u64;
            if nat_eq(a2128, a2129) {
                Adt11::C5(v2)
            } else {
                let a2130 = v1;
                let a2131 = 3u64;
                if nat_eq(a2130, a2131) {
                    Adt11::C3(v2, v2, false)
                } else {
                    let a2132 = v1;
                    let a2133 = 4u64;
                    if nat_eq(a2132, a2133) {
                        Adt11::C2(v3, v4, false)
                    } else {
                        Adt11::C6
                    }
                }
            }
        }
    }
}

pub fn f72(v0: u64, v1: u64) -> Adt12 {
    let a2134 = v0;
    let a2135 = 0u64;
    if nat_eq(a2134, a2135) {
        Adt12::C0(0u64, v1)
    } else {
        let a2136 = v0;
        let a2137 = 1u64;
        if nat_eq(a2136, a2137) {
            Adt12::C1(v1)
        } else {
            let a2138 = v0;
            let a2139 = 2u64;
            if nat_eq(a2138, a2139) {
                Adt12::C2(v1)
            } else {
                let a2140 = v0;
                let a2141 = 3u64;
                if nat_eq(a2140, a2141) {
                    Adt12::C3(v1)
                } else {
                    let a2142 = v0;
                    let a2143 = 13u64;
                    if nat_lt(a2142, a2143) {
                        Adt12::C0({
                            let a2144 = v0;
                            let a2145 = 3u64;
                            nat_sub(a2144, a2145)
                        }, v1)
                    } else {
                        Adt12::C4
                    }
                }
            }
        }
    }
}

pub fn f73(v0: u64, v1: u64) -> Adt13 {
    let a2146 = v0;
    let a2147 = 1u64;
    if nat_eq(a2146, a2147) {
        Adt13::C0(v1)
    } else {
        let a2148 = v0;
        let a2149 = 3u64;
        if if nat_eq(a2148, a2149) {
            true
        } else {
            let a2150 = v0;
            let a2151 = 6u64;
            nat_eq(a2150, a2151)
        } {
            Adt13::C1(v1)
        } else {
            Adt13::C2
        }
    }
}

pub fn f74(v0: u64) -> u64 {
    let a2152 = v0;
    let a2153 = 2147483648u64;
    if nat_lt(a2152, a2153) {
        v0
    } else {
        0u64
    }
}

pub fn f75(v0: u64) -> u64 {
    let a2154 = v0;
    let a2155 = 2147483648u64;
    if nat_lt(a2154, a2155) {
        0u64
    } else {
        let a2156 = 4294967296u64;
        let a2157 = v0;
        nat_sub(a2156, a2157)
    }
}

pub fn f76(v0: u64) -> u64 {
    let v1: u64 = {
        let a2158 = v0;
        let a2159 = 4294967296u64;
        let a2160 = 0u64;
        nat_rem(a2158, a2159, a2160)
    };
    let a2161 = v1;
    let a2162 = 2147483648u64;
    if nat_lt(a2161, a2162) {
        v1
    } else {
        0u64
    }
}

pub fn f77(v0: u64) -> R<u64> {
    let a2165 = 2u64;
    let a2166 = {
        let a2163 = 2u64;
        let a2164 = v0;
        nat_mul(a2163, a2164)?
    };
    nat_add(a2165, a2166)
}

pub fn f78(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64) -> bool {
    if if f139(v0, v3, v6) {
        f139(v1, v4, v6)
    } else {
        false
    } {
        f139(v2, v5, v6)
    } else {
        false
    }
}

pub fn f79(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64) -> R<bool> {
    if if if if if f140(v0, v1, v4, v2, v5, v3, v6)? {
        true
    } else {
        f140(v0, v1, v4, v3, v6, v2, v5)?
    } {
        true
    } else {
        f140(v0, v2, v5, v1, v4, v3, v6)?
    } {
        true
    } else {
        f140(v0, v2, v5, v3, v6, v1, v4)?
    } {
        true
    } else {
        f140(v0, v3, v6, v1, v4, v2, v5)?
    } {
        Ok(true)
    } else {
        f140(v0, v3, v6, v2, v5, v1, v4)
    }
}

pub fn f80(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let a2167 = v2;
    let a2168 = 0u64;
    if nat_eq(a2167, a2168) {
        Ok(0u64)
    } else {
        let a2171 = 2u64;
        let a2172 = {
            let a2169 = v0;
            let a2170 = v1;
            nat_mul(a2169, a2170)?
        };
        nat_mul(a2171, a2172)
    }
}

pub fn f81(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let a2173 = v2;
    let a2174 = f135(v0, v1)?;
    nat_mul(a2173, a2174)
}

pub fn f82(v0: u64) -> u64 {
    let a2175 = v0;
    let a2176 = 30u64;
    if nat_eq(a2175, a2176) {
        3u64
    } else {
        1u64
    }
}

pub fn f83(v0: u64) -> R<u64> {
    let a2179 = 5u64;
    let a2180 = {
        let a2177 = 4u64;
        let a2178 = v0;
        nat_mul(a2177, a2178)?
    };
    nat_add(a2179, a2180)
}

pub fn f84(v0: Adt6, v1: u64, v2: u64) -> R<bool> {
    let v3: u64 = {
        let a2184 = {
            let a2181 = v2;
            let a2182 = 1u64;
            nat_sub(a2181, a2182)
        };
        let a2185 = match v0.clone() {
            Adt6::C0(_, h2183, _, _) => {
                h2183
            }
        };
        nat_mul(a2184, a2185)?
    };
    let a2187 = 0u64;
    let a2188 = match v0.clone() {
        Adt6::C0(h2186, _, _, _) => {
            h2186
        }
    };
    if if if if if if if if nat_lt(a2187, a2188) {
        let a2190 = match v0.clone() {
            Adt6::C0(h2189, _, _, _) => {
                h2189
            }
        };
        let a2191 = 2147483648u64;
        nat_lt(a2190, a2191)
    } else {
        false
    } {
        let a2193 = 0u64;
        let a2194 = match v0.clone() {
            Adt6::C0(_, h2192, _, _) => {
                h2192
            }
        };
        nat_lt(a2193, a2194)
    } else {
        false
    } {
        let a2196 = match v0.clone() {
            Adt6::C0(_, h2195, _, _) => {
                h2195
            }
        };
        let a2197 = 2147483648u64;
        nat_lt(a2196, a2197)
    } else {
        false
    } {
        let a2198 = v1;
        let a2199 = 2147483648u64;
        nat_lt(a2198, a2199)
    } else {
        false
    } {
        let a2200 = 0u64;
        let a2201 = v2;
        nat_lt(a2200, a2201)
    } else {
        false
    } {
        let a2202 = v3;
        let a2203 = 2147483648u64;
        nat_lt(a2202, a2203)
    } else {
        false
    } {
        let a2215 = {
            let a2210 = {
                let a2207 = v3;
                let a2208 = {
                    let a2205 = match v0.clone() {
                        Adt6::C0(h2204, _, _, _) => {
                            h2204
                        }
                    };
                    let a2206 = 1u64;
                    nat_sub(a2205, a2206)
                };
                nat_add(a2207, a2208)?
            };
            let a2211 = f75(match v0.clone() {
                Adt6::C0(_, _, h2209, _) => {
                    h2209
                }
            });
            nat_add(a2210, a2211)?
        };
        let a2216 = {
            let a2213 = 2147483647u64;
            let a2214 = f74(match v0.clone() {
                Adt6::C0(_, _, h2212, _) => {
                    h2212
                }
            });
            nat_add(a2213, a2214)?
        };
        nat_le(a2215, a2216)
    } else {
        false
    } {
        let a2218 = match v0 {
            Adt6::C0(_, _, h2217, _) => {
                h2217
            }
        };
        let a2219 = 2147483648u64;
        Ok(!nat_eq(a2218, a2219))
    } else {
        Ok(false)
    }
}

pub fn f85(v0: Adt7) -> R<u64> {
    let a2231 = if match v0.clone() {
        Adt7::C0(_, _, _, h2220, _) => {
            h2220
        }
    } {
        2u64
    } else {
        3u64
    };
    let a2232 = {
        let a2229 = {
            let a2227 = {
                let m2223 = match v0.clone() {
                    Adt7::C0(h2222, _, _, _, _) => {
                        h2222
                    }
                };
                match m2223 {
                    Adt6::C0(h2221, _, _, _) => {
                        h2221
                    }
                }
            };
            let a2228 = {
                let m2226 = match v0.clone() {
                    Adt7::C0(_, h2225, _, _, _) => {
                        h2225
                    }
                };
                match m2226 {
                    Adt6::C0(h2224, _, _, _) => {
                        h2224
                    }
                }
            };
            nat_mul(a2227, a2228)?
        };
        let a2230 = f131(v0)?;
        nat_mul(a2229, a2230)?
    };
    nat_add(a2231, a2232)
}

pub fn f86(v0: Adt2, v1: Adt2, v2: Adt2) -> R<Adt10> {
    if f141(match v0.clone() {
        Adt2::C0(h2233, _, _, _, _, _, _, _, _, _, _) => {
            h2233
        }
    }, match v0.clone() {
        Adt2::C0(_, h2234, _, _, _, _, _, _, _, _, _) => {
            h2234
        }
    }, match v0.clone() {
        Adt2::C0(_, _, h2235, _, _, _, _, _, _, _, _) => {
            h2235
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, h2236, _, _, _, _, _, _, _) => {
            h2236
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, h2237, _, _, _, _, _, _) => {
            h2237
        }
    }, match v1.clone() {
        Adt2::C0(h2238, _, _, _, _, _, _, _, _, _, _) => {
            h2238
        }
    }, match v1.clone() {
        Adt2::C0(_, h2239, _, _, _, _, _, _, _, _, _) => {
            h2239
        }
    }, match v1.clone() {
        Adt2::C0(_, _, h2240, _, _, _, _, _, _, _, _) => {
            h2240
        }
    }, match v1.clone() {
        Adt2::C0(_, _, _, h2241, _, _, _, _, _, _, _) => {
            h2241
        }
    }, match v1.clone() {
        Adt2::C0(_, _, _, _, h2242, _, _, _, _, _, _) => {
            h2242
        }
    }, match v2.clone() {
        Adt2::C0(h2243, _, _, _, _, _, _, _, _, _, _) => {
            h2243
        }
    }, match v2.clone() {
        Adt2::C0(_, h2244, _, _, _, _, _, _, _, _, _) => {
            h2244
        }
    }, match v2.clone() {
        Adt2::C0(_, _, h2245, _, _, _, _, _, _, _, _) => {
            h2245
        }
    }, match v2.clone() {
        Adt2::C0(_, _, _, h2246, _, _, _, _, _, _, _) => {
            h2246
        }
    }, match v2.clone() {
        Adt2::C0(_, _, _, _, h2247, _, _, _, _, _, _) => {
            h2247
        }
    }) {
        let m2249 = f90(match v0.clone() {
            Adt2::C0(h2248, _, _, _, _, _, _, _, _, _, _) => {
                h2248
            }
        });
        match m2249 {
            Adt14::C0 => {
                Ok(Adt10::C0)
            }
            Adt14::C1(v3) => {
                let a2255 = {
                    let a2252 = match v0.clone() {
                        Adt2::C0(_, h2250, _, _, _, _, _, _, _, _, _) => {
                            h2250
                        }
                    };
                    let a2253 = match v3.clone() {
                        Adt15::C0(_, h2251, _) => {
                            h2251
                        }
                    };
                    let a2254 = 0u64;
                    nat_rem(a2252, a2253, a2254)
                };
                let a2256 = 0u64;
                if if nat_eq(a2255, a2256) {
                    if f142(match v0.clone() {
                        Adt2::C0(_, h2257, _, _, _, _, _, _, _, _, _) => {
                            h2257
                        }
                    }, match v1.clone() {
                        Adt2::C0(_, h2258, _, _, _, _, _, _, _, _, _) => {
                            h2258
                        }
                    }, match v1.clone() {
                        Adt2::C0(_, _, h2259, _, _, _, _, _, _, _, _) => {
                            h2259
                        }
                    }, match v1.clone() {
                        Adt2::C0(_, _, _, h2260, _, _, _, _, _, _, _) => {
                            h2260
                        }
                    }) {
                        true
                    } else {
                        let a2262 = 4u64;
                        let a2263 = match v2.clone() {
                            Adt2::C0(_, _, _, _, _, h2261, _, _, _, _, _) => {
                                h2261
                            }
                        };
                        if if nat_le(a2262, a2263) {
                            let a2265 = match v3.clone() {
                                Adt15::C0(_, h2264, _) => {
                                    h2264
                                }
                            };
                            let a2266 = 1u64;
                            if nat_eq(a2265, a2266) {
                                true
                            } else {
                                let a2269 = match v3.clone() {
                                    Adt15::C0(_, _, h2267) => {
                                        h2267
                                    }
                                };
                                let a2270 = match v0.clone() {
                                    Adt2::C0(_, _, _, _, _, h2268, _, _, _, _, _) => {
                                        h2268
                                    }
                                };
                                nat_le(a2269, a2270)
                            }
                        } else {
                            false
                        } {
                            if f18(v0.clone(), v2.clone())? {
                                f18(v1.clone(), v2.clone())?
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
                    let v4: Adt3 = f16(v0.clone(), match v3 {
                        Adt15::C0(_, h2271, _) => {
                            h2271
                        }
                    });
                    Ok(Adt10::C4(if f142(match v0.clone() {
                        Adt2::C0(_, h2272, _, _, _, _, _, _, _, _, _) => {
                            h2272
                        }
                    }, match v1.clone() {
                        Adt2::C0(_, h2273, _, _, _, _, _, _, _, _, _) => {
                            h2273
                        }
                    }, match v1.clone() {
                        Adt2::C0(_, _, h2274, _, _, _, _, _, _, _, _) => {
                            h2274
                        }
                    }, match v1.clone() {
                        Adt2::C0(_, _, _, h2275, _, _, _, _, _, _, _) => {
                            h2275
                        }
                    }) {
                        0u64
                    } else {
                        f125(v4.clone(), f16(v1.clone(), 1u64))?
                    }, 4u64, v4, f16(v2, 1u64), Adt4::C20(match v0 {
                        Adt2::C0(h2276, _, _, _, _, _, _, _, _, _, _) => {
                            h2276
                        }
                    }, f16(v1, 1u64))))
                } else {
                    Ok(Adt10::C1)
                }
            }
        }
    } else {
        Ok(Adt10::C1)
    }
}

pub fn f87(v0: Adt2, v1: Adt2, v2: Adt2) -> R<Adt10> {
    if if if if if if if if if if if if f89(match v0.clone() {
        Adt2::C0(h2277, _, _, _, _, _, _, _, _, _, _) => {
            h2277
        }
    }) {
        let a2279 = match v1.clone() {
            Adt2::C0(h2278, _, _, _, _, _, _, _, _, _, _) => {
                h2278
            }
        };
        let a2280 = 26u64;
        nat_eq(a2279, a2280)
    } else {
        false
    } {
        let a2282 = match v2.clone() {
            Adt2::C0(h2281, _, _, _, _, _, _, _, _, _, _) => {
                h2281
            }
        };
        let a2283 = 0u64;
        nat_eq(a2282, a2283)
    } else {
        false
    } {
        let a2286 = match v0.clone() {
            Adt2::C0(_, h2284, _, _, _, _, _, _, _, _, _) => {
                h2284
            }
        };
        let a2287 = match v2.clone() {
            Adt2::C0(_, h2285, _, _, _, _, _, _, _, _, _) => {
                h2285
            }
        };
        nat_eq(a2286, a2287)
    } else {
        false
    } {
        let a2290 = match v1.clone() {
            Adt2::C0(_, h2288, _, _, _, _, _, _, _, _, _) => {
                h2288
            }
        };
        let a2291 = match v0.clone() {
            Adt2::C0(_, _, h2289, _, _, _, _, _, _, _, _) => {
                h2289
            }
        };
        nat_le(a2290, a2291)
    } else {
        false
    } {
        let a2293 = match v0.clone() {
            Adt2::C0(_, _, _, h2292, _, _, _, _, _, _, _) => {
                h2292
            }
        };
        let a2294 = 1u64;
        nat_eq(a2293, a2294)
    } else {
        false
    } {
        let a2296 = match v0.clone() {
            Adt2::C0(_, _, _, _, h2295, _, _, _, _, _, _) => {
                h2295
            }
        };
        let a2297 = 1u64;
        nat_eq(a2296, a2297)
    } else {
        false
    } {
        let a2299 = match v2.clone() {
            Adt2::C0(_, _, _, h2298, _, _, _, _, _, _, _) => {
                h2298
            }
        };
        let a2300 = 1u64;
        nat_eq(a2299, a2300)
    } else {
        false
    } {
        let a2302 = match v2.clone() {
            Adt2::C0(_, _, _, _, h2301, _, _, _, _, _, _) => {
                h2301
            }
        };
        let a2303 = 1u64;
        nat_eq(a2302, a2303)
    } else {
        false
    } {
        let a2305 = match v1.clone() {
            Adt2::C0(_, _, h2304, _, _, _, _, _, _, _, _) => {
                h2304
            }
        };
        let a2306 = 1u64;
        nat_eq(a2305, a2306)
    } else {
        false
    } {
        let a2308 = match v1.clone() {
            Adt2::C0(_, _, _, h2307, _, _, _, _, _, _, _) => {
                h2307
            }
        };
        let a2309 = 1u64;
        nat_eq(a2308, a2309)
    } else {
        false
    } {
        let a2311 = match v1.clone() {
            Adt2::C0(_, _, _, _, h2310, _, _, _, _, _, _) => {
                h2310
            }
        };
        let a2312 = 1u64;
        nat_eq(a2311, a2312)
    } else {
        false
    } {
        if f27(match v2.clone() {
            Adt2::C0(_, h2313, _, _, _, _, _, _, _, _, _) => {
                h2313
            }
        }, match v2.clone() {
            Adt2::C0(_, _, h2314, _, _, _, _, _, _, _, _) => {
                h2314
            }
        }, match v2.clone() {
            Adt2::C0(_, _, _, h2315, _, _, _, _, _, _, _) => {
                h2315
            }
        }, match v2.clone() {
            Adt2::C0(_, _, _, _, h2316, _, _, _, _, _, _) => {
                h2316
            }
        }) {
            Ok(Adt10::C4(0u64, 4u64, f16(v0, 1u64), f16(v2, 1u64), Adt4::C9(0u64, 0u64)))
        } else {
            let a2318 = match v1.clone() {
                Adt2::C0(_, h2317, _, _, _, _, _, _, _, _, _) => {
                    h2317
                }
            };
            let a2319 = 0u64;
            if nat_eq(a2318, a2319) {
                Ok(Adt10::C4(f12(v2.clone(), 1u64)?, 4u64, f16(v0, 1u64), f16(v2, 1u64), Adt4::C9(0u64, 0u64)))
            } else {
                let a2321 = match v0.clone() {
                    Adt2::C0(_, h2320, _, _, _, _, _, _, _, _, _) => {
                        h2320
                    }
                };
                let a2322 = 2147483647u64;
                if if if if if if nat_le(a2321, a2322) {
                    let a2324 = match v1.clone() {
                        Adt2::C0(_, h2323, _, _, _, _, _, _, _, _, _) => {
                            h2323
                        }
                    };
                    let a2325 = 2147483647u64;
                    nat_le(a2324, a2325)
                } else {
                    false
                } {
                    let a2327 = 4u64;
                    let a2328 = match v2.clone() {
                        Adt2::C0(_, _, _, _, _, h2326, _, _, _, _, _) => {
                            h2326
                        }
                    };
                    nat_le(a2327, a2328)
                } else {
                    false
                } {
                    let a2334 = {
                        let a2331 = match v2.clone() {
                            Adt2::C0(_, h2329, _, _, _, _, _, _, _, _, _) => {
                                h2329
                            }
                        };
                        let a2332 = match v2.clone() {
                            Adt2::C0(_, _, _, _, _, h2330, _, _, _, _, _) => {
                                h2330
                            }
                        };
                        nat_mul(a2331, a2332)?
                    };
                    let a2335 = match v2.clone() {
                        Adt2::C0(_, _, _, _, _, _, h2333, _, _, _, _) => {
                            h2333
                        }
                    };
                    nat_le(a2334, a2335)
                } else {
                    false
                } {
                    f18(v0.clone(), v2.clone())?
                } else {
                    false
                } {
                    f18(v1.clone(), v2.clone())?
                } else {
                    false
                } {
                    Ok(Adt10::C4({
                        let a2349 = {
                            let a2339 = {
                                let a2337 = 2u64;
                                let a2338 = match v1.clone() {
                                    Adt2::C0(_, h2336, _, _, _, _, _, _, _, _, _) => {
                                        h2336
                                    }
                                };
                                nat_mul(a2337, a2338)?
                            };
                            let a2340 = f12(v2.clone(), 1u64)?;
                            nat_add(a2339, a2340)?
                        };
                        let a2350 = {
                            let a2347 = match v1.clone() {
                                Adt2::C0(_, h2341, _, _, _, _, _, _, _, _, _) => {
                                    h2341
                                }
                            };
                            let a2348 = {
                                let a2345 = 1u64;
                                let a2346 = {
                                    let a2343 = 2u64;
                                    let a2344 = match v0.clone() {
                                        Adt2::C0(_, h2342, _, _, _, _, _, _, _, _, _) => {
                                            h2342
                                        }
                                    };
                                    nat_mul(a2343, a2344)?
                                };
                                nat_add(a2345, a2346)?
                            };
                            nat_mul(a2347, a2348)?
                        };
                        nat_add(a2349, a2350)?
                    }, 4u64, Adt3::C0(match v0.clone() {
                        Adt2::C0(_, h2351, _, _, _, _, _, _, _, _, _) => {
                            h2351
                        }
                    }, match v1.clone() {
                        Adt2::C0(_, h2352, _, _, _, _, _, _, _, _, _) => {
                            h2352
                        }
                    }, 1u64, 1u64, match v0.clone() {
                        Adt2::C0(_, _, _, _, _, h2353, _, _, _, _, _) => {
                            h2353
                        }
                    }, match v0.clone() {
                        Adt2::C0(_, _, _, _, _, _, h2354, _, _, _, _) => {
                            h2354
                        }
                    }, match v0.clone() {
                        Adt2::C0(_, _, _, _, _, _, _, h2355, _, _, _) => {
                            h2355
                        }
                    }, match v0.clone() {
                        Adt2::C0(_, _, _, _, _, _, _, _, h2356, _, _) => {
                            h2356
                        }
                    }), f16(v2, 1u64), Adt4::C19(match v0 {
                        Adt2::C0(h2357, _, _, _, _, _, _, _, _, _, _) => {
                            h2357
                        }
                    }, f16(v1, 1u64))))
                } else {
                    Ok(Adt10::C1)
                }
            }
        }
    } else {
        Ok(Adt10::C1)
    }
}

pub fn f88(v0: u64, v1: Adt2, v2: bool, v3: Adt2) -> R<Adt10> {
    let a2360 = match v1.clone() {
        Adt2::C0(h2358, _, _, _, _, _, _, _, _, _, _) => {
            h2358
        }
    };
    let a2361 = match v3.clone() {
        Adt2::C0(h2359, _, _, _, _, _, _, _, _, _, _) => {
            h2359
        }
    };
    if if nat_eq(a2360, a2361) {
        let a2362 = v0;
        let a2363 = 33u64;
        if nat_eq(a2362, a2363) {
            v2
        } else {
            true
        }
    } else {
        false
    } {
        let m2365 = f90(match v1.clone() {
            Adt2::C0(h2364, _, _, _, _, _, _, _, _, _, _) => {
                h2364
            }
        });
        match m2365 {
            Adt14::C0 => {
                Ok(Adt10::C0)
            }
            Adt14::C1(v4) => {
                let v5: u64 = match v4.clone() {
                    Adt15::C0(_, h2366, _) => {
                        h2366
                    }
                };
                let v6: u64 = f12(v1.clone(), v5)?;
                let a2371 = {
                    let a2368 = match v1.clone() {
                        Adt2::C0(_, h2367, _, _, _, _, _, _, _, _, _) => {
                            h2367
                        }
                    };
                    let a2369 = v5;
                    let a2370 = 1u64;
                    nat_rem(a2368, a2369, a2370)
                };
                let a2372 = 0u64;
                if if if nat_eq(a2371, a2372) {
                    let a2377 = {
                        let a2374 = match v3.clone() {
                            Adt2::C0(_, h2373, _, _, _, _, _, _, _, _, _) => {
                                h2373
                            }
                        };
                        let a2375 = v5;
                        let a2376 = 1u64;
                        nat_rem(a2374, a2375, a2376)
                    };
                    let a2378 = 0u64;
                    nat_eq(a2377, a2378)
                } else {
                    false
                } {
                    let a2379 = v6;
                    let a2380 = f12(v3.clone(), v5)?;
                    nat_eq(a2379, a2380)
                } else {
                    false
                } {
                    Ok(Adt10::C4(v6, match v4 {
                        Adt15::C0(_, _, h2381) => {
                            h2381
                        }
                    }, f16(v1, v5), f16(v3, v5), Adt4::C0))
                } else {
                    Ok(Adt10::C1)
                }
            }
        }
    } else {
        Ok(Adt10::C1)
    }
}

pub fn f89(v0: u64) -> bool {
    let a2382 = v0;
    let a2383 = 0u64;
    if if nat_eq(a2382, a2383) {
        true
    } else {
        let a2384 = v0;
        let a2385 = 1u64;
        nat_eq(a2384, a2385)
    } {
        true
    } else {
        let a2386 = v0;
        let a2387 = 30u64;
        nat_eq(a2386, a2387)
    }
}

pub fn f90(v0: u64) -> Adt14 {
    let a2388 = v0;
    let a2389 = 0u64;
    if nat_eq(a2388, a2389) {
        Adt14::C1(Adt15::C0(0u64, 1u64, 4u64))
    } else {
        let a2390 = v0;
        let a2391 = 1u64;
        if nat_eq(a2390, a2391) {
            Adt14::C1(Adt15::C0(1u64, 1u64, 2u64))
        } else {
            let a2392 = v0;
            let a2393 = 2u64;
            if nat_eq(a2392, a2393) {
                Adt14::C1(Adt15::C0(2u64, 32u64, 18u64))
            } else {
                let a2394 = v0;
                let a2395 = 3u64;
                if nat_eq(a2394, a2395) {
                    Adt14::C1(Adt15::C0(3u64, 32u64, 20u64))
                } else {
                    let a2396 = v0;
                    let a2397 = 6u64;
                    if nat_eq(a2396, a2397) {
                        Adt14::C1(Adt15::C0(6u64, 32u64, 22u64))
                    } else {
                        let a2398 = v0;
                        let a2399 = 7u64;
                        if nat_eq(a2398, a2399) {
                            Adt14::C1(Adt15::C0(7u64, 32u64, 24u64))
                        } else {
                            let a2400 = v0;
                            let a2401 = 8u64;
                            if nat_eq(a2400, a2401) {
                                Adt14::C1(Adt15::C0(8u64, 32u64, 34u64))
                            } else {
                                let a2402 = v0;
                                let a2403 = 9u64;
                                if nat_eq(a2402, a2403) {
                                    Adt14::C1(Adt15::C0(9u64, 32u64, 36u64))
                                } else {
                                    let a2404 = v0;
                                    let a2405 = 10u64;
                                    if nat_eq(a2404, a2405) {
                                        Adt14::C1(Adt15::C0(10u64, 256u64, 84u64))
                                    } else {
                                        let a2406 = v0;
                                        let a2407 = 11u64;
                                        if nat_eq(a2406, a2407) {
                                            Adt14::C1(Adt15::C0(11u64, 256u64, 110u64))
                                        } else {
                                            let a2408 = v0;
                                            let a2409 = 12u64;
                                            if nat_eq(a2408, a2409) {
                                                Adt14::C1(Adt15::C0(12u64, 256u64, 144u64))
                                            } else {
                                                let a2410 = v0;
                                                let a2411 = 13u64;
                                                if nat_eq(a2410, a2411) {
                                                    Adt14::C1(Adt15::C0(13u64, 256u64, 176u64))
                                                } else {
                                                    let a2412 = v0;
                                                    let a2413 = 14u64;
                                                    if nat_eq(a2412, a2413) {
                                                        Adt14::C1(Adt15::C0(14u64, 256u64, 210u64))
                                                    } else {
                                                        let a2414 = v0;
                                                        let a2415 = 15u64;
                                                        if nat_eq(a2414, a2415) {
                                                            Adt14::C1(Adt15::C0(15u64, 256u64, 292u64))
                                                        } else {
                                                            let a2416 = v0;
                                                            let a2417 = 16u64;
                                                            if nat_eq(a2416, a2417) {
                                                                Adt14::C1(Adt15::C0(16u64, 256u64, 66u64))
                                                            } else {
                                                                let a2418 = v0;
                                                                let a2419 = 17u64;
                                                                if nat_eq(a2418, a2419) {
                                                                    Adt14::C1(Adt15::C0(17u64, 256u64, 74u64))
                                                                } else {
                                                                    let a2420 = v0;
                                                                    let a2421 = 18u64;
                                                                    if nat_eq(a2420, a2421) {
                                                                        Adt14::C1(Adt15::C0(18u64, 256u64, 98u64))
                                                                    } else {
                                                                        let a2422 = v0;
                                                                        let a2423 = 19u64;
                                                                        if nat_eq(a2422, a2423) {
                                                                            Adt14::C1(Adt15::C0(19u64, 256u64, 50u64))
                                                                        } else {
                                                                            let a2424 = v0;
                                                                            let a2425 = 20u64;
                                                                            if nat_eq(a2424, a2425) {
                                                                                Adt14::C1(Adt15::C0(20u64, 32u64, 18u64))
                                                                            } else {
                                                                                let a2426 = v0;
                                                                                let a2427 = 21u64;
                                                                                if nat_eq(a2426, a2427) {
                                                                                    Adt14::C1(Adt15::C0(21u64, 256u64, 110u64))
                                                                                } else {
                                                                                    let a2428 = v0;
                                                                                    let a2429 = 22u64;
                                                                                    if nat_eq(a2428, a2429) {
                                                                                        Adt14::C1(Adt15::C0(22u64, 256u64, 82u64))
                                                                                    } else {
                                                                                        let a2430 = v0;
                                                                                        let a2431 = 23u64;
                                                                                        if nat_eq(a2430, a2431) {
                                                                                            Adt14::C1(Adt15::C0(23u64, 256u64, 136u64))
                                                                                        } else {
                                                                                            let a2432 = v0;
                                                                                            let a2433 = 24u64;
                                                                                            if nat_eq(a2432, a2433) {
                                                                                                Adt14::C1(Adt15::C0(24u64, 1u64, 1u64))
                                                                                            } else {
                                                                                                let a2434 = v0;
                                                                                                let a2435 = 25u64;
                                                                                                if nat_eq(a2434, a2435) {
                                                                                                    Adt14::C1(Adt15::C0(25u64, 1u64, 2u64))
                                                                                                } else {
                                                                                                    let a2436 = v0;
                                                                                                    let a2437 = 26u64;
                                                                                                    if nat_eq(a2436, a2437) {
                                                                                                        Adt14::C1(Adt15::C0(26u64, 1u64, 4u64))
                                                                                                    } else {
                                                                                                        let a2438 = v0;
                                                                                                        let a2439 = 27u64;
                                                                                                        if nat_eq(a2438, a2439) {
                                                                                                            Adt14::C1(Adt15::C0(27u64, 1u64, 8u64))
                                                                                                        } else {
                                                                                                            let a2440 = v0;
                                                                                                            let a2441 = 28u64;
                                                                                                            if nat_eq(a2440, a2441) {
                                                                                                                Adt14::C1(Adt15::C0(28u64, 1u64, 8u64))
                                                                                                            } else {
                                                                                                                let a2442 = v0;
                                                                                                                let a2443 = 29u64;
                                                                                                                if nat_eq(a2442, a2443) {
                                                                                                                    Adt14::C1(Adt15::C0(29u64, 256u64, 56u64))
                                                                                                                } else {
                                                                                                                    let a2444 = v0;
                                                                                                                    let a2445 = 30u64;
                                                                                                                    if nat_eq(a2444, a2445) {
                                                                                                                        Adt14::C1(Adt15::C0(30u64, 1u64, 2u64))
                                                                                                                    } else {
                                                                                                                        let a2446 = v0;
                                                                                                                        let a2447 = 34u64;
                                                                                                                        if nat_eq(a2446, a2447) {
                                                                                                                            Adt14::C1(Adt15::C0(34u64, 256u64, 54u64))
                                                                                                                        } else {
                                                                                                                            let a2448 = v0;
                                                                                                                            let a2449 = 35u64;
                                                                                                                            if nat_eq(a2448, a2449) {
                                                                                                                                Adt14::C1(Adt15::C0(35u64, 256u64, 66u64))
                                                                                                                            } else {
                                                                                                                                let a2450 = v0;
                                                                                                                                let a2451 = 39u64;
                                                                                                                                if nat_eq(a2450, a2451) {
                                                                                                                                    Adt14::C1(Adt15::C0(39u64, 32u64, 17u64))
                                                                                                                                } else {
                                                                                                                                    let a2452 = v0;
                                                                                                                                    let a2453 = 40u64;
                                                                                                                                    if nat_eq(a2452, a2453) {
                                                                                                                                        Adt14::C1(Adt15::C0(40u64, 64u64, 36u64))
                                                                                                                                    } else {
                                                                                                                                        let a2454 = v0;
                                                                                                                                        let a2455 = 41u64;
                                                                                                                                        if nat_eq(a2454, a2455) {
                                                                                                                                            Adt14::C1(Adt15::C0(41u64, 128u64, 18u64))
                                                                                                                                        } else {
                                                                                                                                            let a2456 = v0;
                                                                                                                                            let a2457 = 42u64;
                                                                                                                                            if nat_eq(a2456, a2457) {
                                                                                                                                                Adt14::C1(Adt15::C0(42u64, 64u64, 18u64))
                                                                                                                                            } else {
                                                                                                                                                let a2458 = v0;
                                                                                                                                                let a2459 = 43u64;
                                                                                                                                                if nat_eq(a2458, a2459) {
                                                                                                                                                    Adt14::C1(Adt15::C0(43u64, 1u64, 1u64))
                                                                                                                                                } else {
                                                                                                                                                    Adt14::C0
                                                                                                                                                }
                                                                                                                                            }
                                                                                                                                        }
                                                                                                                                    }
                                                                                                                                }
                                                                                                                            }
                                                                                                                        }
                                                                                                                    }
                                                                                                                }
                                                                                                            }
                                                                                                        }
                                                                                                    }
                                                                                                }
                                                                                            }
                                                                                        }
                                                                                    }
                                                                                }
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn f91(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64, v7: u64) -> bool {
    if f27(v4, v5, v6, v7) {
        f27(v0, v1, v2, v3)
    } else {
        let a2463 = {
            let a2460 = v0;
            let a2461 = v4;
            let a2462 = 1u64;
            nat_rem(a2460, a2461, a2462)
        };
        let a2464 = 0u64;
        if if if nat_eq(a2463, a2464) {
            let a2468 = {
                let a2465 = v1;
                let a2466 = v5;
                let a2467 = 1u64;
                nat_rem(a2465, a2466, a2467)
            };
            let a2469 = 0u64;
            nat_eq(a2468, a2469)
        } else {
            false
        } {
            let a2473 = {
                let a2470 = v2;
                let a2471 = v6;
                let a2472 = 1u64;
                nat_rem(a2470, a2471, a2472)
            };
            let a2474 = 0u64;
            nat_eq(a2473, a2474)
        } else {
            false
        } {
            let a2478 = {
                let a2475 = v3;
                let a2476 = v7;
                let a2477 = 1u64;
                nat_rem(a2475, a2476, a2477)
            };
            let a2479 = 0u64;
            nat_eq(a2478, a2479)
        } else {
            false
        }
    }
}

pub fn f92(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64) -> R<bool> {
    let a2480 = v0;
    let a2481 = v1;
    if nat_eq(a2480, a2481) {
        let a2484 = v4;
        let a2485 = {
            let a2482 = v2;
            let a2483 = v3;
            nat_add(a2482, a2483)?
        };
        Ok(nat_eq(a2484, a2485))
    } else {
        let a2486 = v2;
        let a2487 = v3;
        if nat_eq(a2486, a2487) {
            let a2488 = v4;
            let a2489 = v2;
            Ok(nat_eq(a2488, a2489))
        } else {
            Ok(false)
        }
    }
}

pub fn f93(v0: u64, v1: u64, v2: u64, v3: u64) -> R<bool> {
    let v4: u64 = {
        let a2492 = {
            let a2490 = v0;
            let a2491 = f74(v1);
            nat_add(a2490, a2491)?
        };
        let a2493 = f74(v2);
        nat_add(a2492, a2493)?
    };
    let v5: u64 = {
        let a2494 = f75(v1);
        let a2495 = f75(v2);
        nat_add(a2494, a2495)?
    };
    let a2496 = v1;
    let a2497 = 4294967295u64;
    if if nat_le(a2496, a2497) {
        let a2498 = v2;
        let a2499 = 4294967295u64;
        nat_le(a2498, a2499)
    } else {
        false
    } {
        let a2500 = v5;
        let a2501 = v4;
        if nat_le(a2500, a2501) {
            let a2504 = v3;
            let a2505 = {
                let a2502 = v4;
                let a2503 = v5;
                nat_sub(a2502, a2503)
            };
            Ok(nat_eq(a2504, a2505))
        } else {
            Ok(false)
        }
    } else {
        Ok(false)
    }
}

pub fn f94(v0: u64, v1: u64) -> R<bool> {
    let a2506 = v1;
    let a2507 = 4294967295u64;
    if nat_le(a2506, a2507) {
        let a2510 = {
            let a2508 = f74(v1);
            let a2509 = f75(v1);
            nat_add(a2508, a2509)?
        };
        let a2511 = v0;
        Ok(nat_lt(a2510, a2511))
    } else {
        Ok(false)
    }
}

pub fn f95(v0: Adt3, v1: u64, v2: u64, v3: u64, v4: bool) -> R<bool> {
    let v5: u64 = f143(v0.clone(), false)?;
    let v6: u64 = f143(v0, true)?;
    let a2512 = v6;
    let a2513 = v1;
    if if nat_le(a2512, a2513) {
        let a2520 = {
            let a2518 = {
                let a2514 = v1;
                let a2515 = v5;
                nat_add(a2514, a2515)?
            };
            let a2519 = {
                let a2516 = v3;
                let a2517 = 0u64;
                if nat_eq(a2516, a2517) {
                    0u64
                } else {
                    4u64
                }
            };
            nat_add(a2518, a2519)?
        };
        let a2521 = v2;
        nat_le(a2520, a2521)
    } else {
        false
    } {
        if v4 {
            let a2526 = {
                let a2524 = {
                    let a2522 = v1;
                    let a2523 = v5;
                    nat_add(a2522, a2523)?
                };
                let a2525 = v6;
                nat_sub(a2524, a2525)
            };
            let a2527 = v2;
            Ok(nat_lt(a2526, a2527))
        } else {
            let a2532 = {
                let a2530 = {
                    let a2528 = v1;
                    let a2529 = v5;
                    nat_add(a2528, a2529)?
                };
                let a2531 = v6;
                nat_sub(a2530, a2531)
            };
            let a2533 = v2;
            Ok(nat_le(a2532, a2533))
        }
    } else {
        Ok(false)
    }
}

pub fn f96(v0: u64, v1: u64) -> bool {
    let a2534 = v0;
    let a2535 = 0u64;
    if if nat_eq(a2534, a2535) {
        true
    } else {
        let a2536 = v0;
        let a2537 = 1u64;
        nat_eq(a2536, a2537)
    } {
        let a2538 = v1;
        let a2539 = 4294967296u64;
        nat_lt(a2538, a2539)
    } else {
        false
    }
}

pub fn f97(v0: u64, v1: u64) -> R<u64> {
    let a2544 = {
        let a2542 = v0;
        let a2543 = {
            let a2540 = v1;
            let a2541 = 1u64;
            nat_sub(a2540, a2541)
        };
        nat_add(a2542, a2543)?
    };
    let a2545 = v1;
    let a2546 = 0u64;
    Ok(nat_quot(a2544, a2545, a2546))
}

pub fn f98(v0: Adt2, v1: Adt2, v2: u64, v3: u64) -> R<bool> {
    if f27(match v0.clone() {
        Adt2::C0(_, h2547, _, _, _, _, _, _, _, _, _) => {
            h2547
        }
    }, match v0.clone() {
        Adt2::C0(_, _, h2548, _, _, _, _, _, _, _, _) => {
            h2548
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, h2549, _, _, _, _, _, _, _) => {
            h2549
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, h2550, _, _, _, _, _, _) => {
            h2550
        }
    }) {
        Ok(true)
    } else {
        let v4: u64 = {
            let a2553 = match v1.clone() {
                Adt2::C0(_, h2551, _, _, _, _, _, _, _, _, _) => {
                    h2551
                }
            };
            let a2554 = match v1.clone() {
                Adt2::C0(_, _, h2552, _, _, _, _, _, _, _, _) => {
                    h2552
                }
            };
            nat_mul(a2553, a2554)?
        };
        let v5: u64 = {
            let a2556 = v4;
            let a2557 = match v1.clone() {
                Adt2::C0(_, _, _, h2555, _, _, _, _, _, _, _) => {
                    h2555
                }
            };
            nat_mul(a2556, a2557)?
        };
        let a2570 = {
            let a2568 = {
                let a2559 = match v0.clone() {
                    Adt2::C0(_, h2558, _, _, _, _, _, _, _, _, _) => {
                        h2558
                    }
                };
                let a2560 = 1u64;
                nat_sub(a2559, a2560)
            };
            let a2569 = {
                let a2566 = f144(match v0.clone() {
                    Adt2::C0(_, _, h2561, _, _, _, _, _, _, _, _) => {
                        h2561
                    }
                }, v2, v5, match v1.clone() {
                    Adt2::C0(_, h2562, _, _, _, _, _, _, _, _, _) => {
                        h2562
                    }
                })?;
                let a2567 = f144(match v0 {
                    Adt2::C0(_, _, _, h2563, _, _, _, _, _, _, _) => {
                        h2563
                    }
                }, v2, {
                    let a2564 = v3;
                    let a2565 = v5;
                    nat_mul(a2564, a2565)?
                }, v4)?;
                nat_add(a2566, a2567)?
            };
            nat_add(a2568, a2569)?
        };
        let a2571 = f12(v1, 1u64)?;
        Ok(nat_lt(a2570, a2571))
    }
}

pub fn f99(v0: u64, v1: u64, v2: u64) -> R<bool> {
    let a2572 = v0;
    let a2573 = 2147483648u64;
    if if if if nat_lt(a2572, a2573) {
        let a2574 = v1;
        let a2575 = 2147483648u64;
        nat_lt(a2574, a2575)
    } else {
        false
    } {
        let a2578 = {
            let a2576 = v0;
            let a2577 = v1;
            nat_mul(a2576, a2577)?
        };
        let a2579 = 2147483648u64;
        nat_lt(a2578, a2579)
    } else {
        false
    } {
        let a2584 = {
            let a2582 = {
                let a2580 = v0;
                let a2581 = 1u64;
                nat_sub(a2580, a2581)
            };
            let a2583 = v2;
            nat_mul(a2582, a2583)?
        };
        let a2585 = 2147483648u64;
        nat_lt(a2584, a2585)
    } else {
        false
    } {
        let a2590 = {
            let a2588 = {
                let a2586 = v1;
                let a2587 = 1u64;
                nat_sub(a2586, a2587)
            };
            let a2589 = v2;
            nat_mul(a2588, a2589)?
        };
        let a2591 = 2147483648u64;
        Ok(nat_lt(a2590, a2591))
    } else {
        Ok(false)
    }
}

pub fn f100(v0: u64) -> R<u64> {
    let a2594 = 3u64;
    let a2595 = {
        let a2592 = v0;
        let a2593 = 3u64;
        nat_mul(a2592, a2593)?
    };
    nat_add(a2594, a2595)
}

pub fn f101(v0: Adt3) -> R<u64> {
    if f27(match v0.clone() {
        Adt3::C0(h2596, _, _, _, _, _, _, _) => {
            h2596
        }
    }, match v0.clone() {
        Adt3::C0(_, h2597, _, _, _, _, _, _) => {
            h2597
        }
    }, match v0.clone() {
        Adt3::C0(_, _, h2598, _, _, _, _, _) => {
            h2598
        }
    }, match v0.clone() {
        Adt3::C0(_, _, _, h2599, _, _, _, _) => {
            h2599
        }
    }) {
        Ok(0u64)
    } else {
        let a2608 = match v0.clone() {
            Adt3::C0(h2600, _, _, _, _, _, _, _) => {
                h2600
            }
        };
        let a2609 = {
            let a2606 = match v0.clone() {
                Adt3::C0(_, h2601, _, _, _, _, _, _) => {
                    h2601
                }
            };
            let a2607 = {
                let a2604 = match v0.clone() {
                    Adt3::C0(_, _, h2602, _, _, _, _, _) => {
                        h2602
                    }
                };
                let a2605 = match v0 {
                    Adt3::C0(_, _, _, h2603, _, _, _, _) => {
                        h2603
                    }
                };
                nat_mul(a2604, a2605)?
            };
            nat_mul(a2606, a2607)?
        };
        nat_mul(a2608, a2609)
    }
}

pub fn f102(v0: Adt3, v1: u64, v2: u64) -> R<u64> {
    let a2610 = v1;
    let a2611 = 13u64;
    if if nat_eq(a2610, a2611) {
        f27(match v0.clone() {
            Adt3::C0(h2612, _, _, _, _, _, _, _) => {
                h2612
            }
        }, match v0.clone() {
            Adt3::C0(_, h2613, _, _, _, _, _, _) => {
                h2613
            }
        }, match v0.clone() {
            Adt3::C0(_, _, h2614, _, _, _, _, _) => {
                h2614
            }
        }, match v0.clone() {
            Adt3::C0(_, _, _, h2615, _, _, _, _) => {
                h2615
            }
        })
    } else {
        false
    } {
        Ok(2u64)
    } else {
        let v3: u64 = {
            let a2621 = match v0.clone() {
                Adt3::C0(_, h2616, _, _, _, _, _, _) => {
                    h2616
                }
            };
            let a2622 = {
                let a2619 = match v0.clone() {
                    Adt3::C0(_, _, h2617, _, _, _, _, _) => {
                        h2617
                    }
                };
                let a2620 = match v0.clone() {
                    Adt3::C0(_, _, _, h2618, _, _, _, _) => {
                        h2618
                    }
                };
                nat_mul(a2619, a2620)?
            };
            nat_mul(a2621, a2622)?
        };
        let a2623 = v1;
        let a2624 = 13u64;
        if nat_eq(a2623, a2624) {
            let a2633 = 2u64;
            let a2634 = if f145(v0.clone(), v2) {
                let a2626 = v3;
                let a2627 = f146(match v0 {
                    Adt3::C0(h2625, _, _, _, _, _, _, _) => {
                        h2625
                    }
                }, false)?;
                nat_mul(a2626, a2627)?
            } else {
                let a2631 = v3;
                let a2632 = {
                    let a2629 = match v0 {
                        Adt3::C0(h2628, _, _, _, _, _, _, _) => {
                            h2628
                        }
                    };
                    let a2630 = 2u64;
                    nat_mul(a2629, a2630)?
                };
                nat_mul(a2631, a2632)?
            };
            nat_add(a2633, a2634)
        } else {
            let a2638 = v3;
            let a2639 = f146(match v0 {
                Adt3::C0(h2635, _, _, _, _, _, _, _) => {
                    h2635
                }
            }, {
                let a2636 = v1;
                let a2637 = 16u64;
                nat_eq(a2636, a2637)
            })?;
            nat_mul(a2638, a2639)
        }
    }
}

pub fn f103(v0: u64, v1: u64, v2: Adt2, v3: Option<Adt2>, v4: Adt2) -> R<Adt10> {
    let a2640 = v0;
    let a2641 = 30u64;
    if if nat_eq(a2640, a2641) {
        let a2644 = match v2.clone() {
            Adt2::C0(_, _, _, _, _, _, h2642, _, _, _, _) => {
                h2642
            }
        };
        let a2645 = match v2.clone() {
            Adt2::C0(_, _, _, _, _, h2643, _, _, _, _, _) => {
                h2643
            }
        };
        nat_lt(a2644, a2645)
    } else {
        false
    } {
        Ok(Adt10::C1)
    } else {
        let m2646 = v3;
        match m2646 {
            None => {
                f147(v0, v1, match v2.clone() {
                    Adt2::C0(h2647, _, _, _, _, _, _, _, _, _, _) => {
                        h2647
                    }
                }, 0u64, match v4.clone() {
                    Adt2::C0(h2648, _, _, _, _, _, _, _, _, _, _) => {
                        h2648
                    }
                }, false, match v2.clone() {
                    Adt2::C0(_, h2649, _, _, _, _, _, _, _, _, _) => {
                        h2649
                    }
                }, match v2.clone() {
                    Adt2::C0(_, _, h2650, _, _, _, _, _, _, _, _) => {
                        h2650
                    }
                }, match v2.clone() {
                    Adt2::C0(_, _, _, h2651, _, _, _, _, _, _, _) => {
                        h2651
                    }
                }, match v2 {
                    Adt2::C0(_, _, _, _, h2652, _, _, _, _, _, _) => {
                        h2652
                    }
                }, 1u64, 1u64, 1u64, 1u64, match v4.clone() {
                    Adt2::C0(_, h2653, _, _, _, _, _, _, _, _, _) => {
                        h2653
                    }
                }, match v4.clone() {
                    Adt2::C0(_, _, h2654, _, _, _, _, _, _, _, _) => {
                        h2654
                    }
                }, match v4.clone() {
                    Adt2::C0(_, _, _, h2655, _, _, _, _, _, _, _) => {
                        h2655
                    }
                }, match v4 {
                    Adt2::C0(_, _, _, _, h2656, _, _, _, _, _, _) => {
                        h2656
                    }
                })
            }
            Some(v5) => {
                f147(v0, v1, match v2.clone() {
                    Adt2::C0(h2657, _, _, _, _, _, _, _, _, _, _) => {
                        h2657
                    }
                }, match v5.clone() {
                    Adt2::C0(h2658, _, _, _, _, _, _, _, _, _, _) => {
                        h2658
                    }
                }, match v4.clone() {
                    Adt2::C0(h2659, _, _, _, _, _, _, _, _, _, _) => {
                        h2659
                    }
                }, true, match v2.clone() {
                    Adt2::C0(_, h2660, _, _, _, _, _, _, _, _, _) => {
                        h2660
                    }
                }, match v2.clone() {
                    Adt2::C0(_, _, h2661, _, _, _, _, _, _, _, _) => {
                        h2661
                    }
                }, match v2.clone() {
                    Adt2::C0(_, _, _, h2662, _, _, _, _, _, _, _) => {
                        h2662
                    }
                }, match v2 {
                    Adt2::C0(_, _, _, _, h2663, _, _, _, _, _, _) => {
                        h2663
                    }
                }, match v5.clone() {
                    Adt2::C0(_, h2664, _, _, _, _, _, _, _, _, _) => {
                        h2664
                    }
                }, match v5.clone() {
                    Adt2::C0(_, _, h2665, _, _, _, _, _, _, _, _) => {
                        h2665
                    }
                }, match v5.clone() {
                    Adt2::C0(_, _, _, h2666, _, _, _, _, _, _, _) => {
                        h2666
                    }
                }, match v5 {
                    Adt2::C0(_, _, _, _, h2667, _, _, _, _, _, _) => {
                        h2667
                    }
                }, match v4.clone() {
                    Adt2::C0(_, h2668, _, _, _, _, _, _, _, _, _) => {
                        h2668
                    }
                }, match v4.clone() {
                    Adt2::C0(_, _, h2669, _, _, _, _, _, _, _, _) => {
                        h2669
                    }
                }, match v4.clone() {
                    Adt2::C0(_, _, _, h2670, _, _, _, _, _, _, _) => {
                        h2670
                    }
                }, match v4 {
                    Adt2::C0(_, _, _, _, h2671, _, _, _, _, _, _) => {
                        h2671
                    }
                })
            }
        }
    }
}

pub fn f104(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64, v7: u64) -> R<u64> {
    let a2684 = {
        let a2680 = {
            let a2676 = {
                let a2672 = v0;
                let a2673 = v4;
                nat_mul(a2672, a2673)?
            };
            let a2677 = {
                let a2674 = v1;
                let a2675 = v5;
                nat_mul(a2674, a2675)?
            };
            nat_add(a2676, a2677)?
        };
        let a2681 = {
            let a2678 = v2;
            let a2679 = v6;
            nat_mul(a2678, a2679)?
        };
        nat_add(a2680, a2681)?
    };
    let a2685 = {
        let a2682 = v3;
        let a2683 = v7;
        nat_mul(a2682, a2683)?
    };
    nat_add(a2684, a2685)
}

pub fn f105(v0: Adt11, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64, v7: u64, v8: u64) -> Adt9 {
    let v9: u64 = {
        let a2686 = v7;
        let a2687 = 6u64;
        if nat_eq(a2686, a2687) {
            27u64
        } else {
            v2
        }
    };
    Adt9::C7(v0, f72(v3, v4), f72(v5, v6), f73(v7, v8), f148(v1), f148(v9))
}

pub fn f106(v0: Adt4, v1: u64, v2: u64) -> Adt11 {
    let m2688 = v0;
    match m2688 {
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
            let a2689 = 13u64;
            let a2690 = v34;
            if nat_le(a2689, a2690) {
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
        Adt4::C27(v51) => {
            Adt11::C2(match v51.clone() {
                Adt5::C0(_, _, _, _, _, h2691, _, _) => {
                    h2691
                }
            }, match v51 {
                Adt5::C0(_, _, _, _, _, _, _, h2692) => {
                    h2692
                }
            }, true)
        }
        Adt4::C28(_) => {
            Adt11::C6
        }
    }
}

pub fn f107(v0: Adt4) -> u64 {
    let m2693 = v0;
    match m2693 {
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
    }
}

pub fn f108(v0: Adt4, v1: u64) -> u64 {
    let m2694 = v0;
    match m2694 {
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
            let a2695 = v1;
            let a2696 = 6u64;
            if nat_eq(a2695, a2696) {
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
        Adt4::C26(v51, _, _, _) => {
            let a2701 = v1;
            let a2702 = 6u64;
            if if if nat_eq(a2701, a2702) {
                true
            } else {
                let a2703 = v1;
                let a2704 = 7u64;
                nat_eq(a2703, a2704)
            } {
                let a2705 = v51;
                let a2706 = 30u64;
                nat_eq(a2705, a2706)
            } else {
                false
            } {
                30u64
            } else {
                0u64
            }
        }
        Adt4::C27(v50) => {
            match v50 {
                Adt5::C0(_, _, _, _, _, _, _, h2700) => {
                    h2700
                }
            }
        }
        Adt4::C28(v49) => {
            let a2697 = v1;
            let a2698 = 6u64;
            if nat_eq(a2697, a2698) {
                match v49 {
                    Adt7::C0(_, _, _, _, h2699) => {
                        h2699
                    }
                }
            } else {
                0u64
            }
        }
    }
}

pub fn f109(v0: Adt4, v1: u64) -> Adt11 {
    let a2707 = v1;
    let a2708 = 4u64;
    if nat_lt(a2707, a2708) {
        Adt11::C0(v1)
    } else {
        let a2709 = v1;
        let a2710 = 4u64;
        if nat_eq(a2709, a2710) {
            Adt11::C1(0u64)
        } else {
            let a2711 = v1;
            let a2712 = 5u64;
            if nat_eq(a2711, a2712) {
                Adt11::C1(3u64)
            } else {
                let a2713 = v1;
                let a2714 = 6u64;
                if nat_eq(a2713, a2714) {
                    Adt11::C2(f108(v0, v1), 0u64, false)
                } else {
                    let a2715 = v1;
                    let a2716 = 7u64;
                    if nat_eq(a2715, a2716) {
                        Adt11::C2(0u64, f108(v0, v1), false)
                    } else {
                        let a2717 = v1;
                        let a2718 = 16u64;
                        if nat_lt(a2717, a2718) {
                            Adt11::C1({
                                let a2719 = v1;
                                let a2720 = 8u64;
                                nat_sub(a2719, a2720)
                            })
                        } else {
                            let a2721 = v1;
                            let a2722 = 20u64;
                            if nat_lt(a2721, a2722) {
                                Adt11::C5({
                                    let a2723 = v1;
                                    let a2724 = 16u64;
                                    nat_sub(a2723, a2724)
                                })
                            } else {
                                let a2725 = v1;
                                let a2726 = 20u64;
                                if nat_eq(a2725, a2726) {
                                    Adt11::C3(0u64, 0u64, false)
                                } else {
                                    let a2727 = v1;
                                    let a2728 = 21u64;
                                    if nat_eq(a2727, a2728) {
                                        Adt11::C0(4u64)
                                    } else {
                                        let a2729 = v1;
                                        let a2730 = 22u64;
                                        if nat_eq(a2729, a2730) {
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

pub fn f110(v0: u64, v1: Adt3) -> R<u64> {
    let v2: u64 = {
        let a2732 = v0;
        let a2733 = match v1.clone() {
            Adt3::C0(h2731, _, _, _, _, _, _, _) => {
                h2731
            }
        };
        let a2734 = 0u64;
        nat_quot(a2732, a2733, a2734)
    };
    let v3: u64 = {
        let a2736 = v2;
        let a2737 = match v1.clone() {
            Adt3::C0(_, h2735, _, _, _, _, _, _) => {
                h2735
            }
        };
        let a2738 = 0u64;
        nat_quot(a2736, a2737, a2738)
    };
    f104({
        let a2740 = v0;
        let a2741 = match v1.clone() {
            Adt3::C0(h2739, _, _, _, _, _, _, _) => {
                h2739
            }
        };
        let a2742 = 0u64;
        nat_rem(a2740, a2741, a2742)
    }, {
        let a2744 = v2;
        let a2745 = match v1.clone() {
            Adt3::C0(_, h2743, _, _, _, _, _, _) => {
                h2743
            }
        };
        let a2746 = 0u64;
        nat_rem(a2744, a2745, a2746)
    }, {
        let a2748 = v3;
        let a2749 = match v1.clone() {
            Adt3::C0(_, _, h2747, _, _, _, _, _) => {
                h2747
            }
        };
        let a2750 = 0u64;
        nat_rem(a2748, a2749, a2750)
    }, {
        let a2752 = v3;
        let a2753 = match v1.clone() {
            Adt3::C0(_, _, h2751, _, _, _, _, _) => {
                h2751
            }
        };
        let a2754 = 0u64;
        nat_quot(a2752, a2753, a2754)
    }, match v1.clone() {
        Adt3::C0(_, _, _, _, h2755, _, _, _) => {
            h2755
        }
    }, match v1.clone() {
        Adt3::C0(_, _, _, _, _, h2756, _, _) => {
            h2756
        }
    }, match v1.clone() {
        Adt3::C0(_, _, _, _, _, _, h2757, _) => {
            h2757
        }
    }, match v1 {
        Adt3::C0(_, _, _, _, _, _, _, h2758) => {
            h2758
        }
    })
}

pub fn f111(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let a2776 = {
        let a2771 = f149(v2, {
            let a2762 = {
                let a2759 = v0;
                let a2760 = v1;
                let a2761 = 0u64;
                nat_quot(a2759, a2760, a2761)
            };
            let a2763 = v2;
            let a2764 = 0u64;
            nat_rem(a2762, a2763, a2764)
        }, {
            let a2768 = {
                let a2765 = v0;
                let a2766 = v1;
                let a2767 = 0u64;
                nat_quot(a2765, a2766, a2767)
            };
            let a2769 = v2;
            let a2770 = 0u64;
            nat_quot(a2768, a2769, a2770)
        })?;
        let a2772 = v1;
        nat_mul(a2771, a2772)?
    };
    let a2777 = {
        let a2773 = v0;
        let a2774 = v1;
        let a2775 = 0u64;
        nat_rem(a2773, a2774, a2775)
    };
    nat_add(a2776, a2777)
}

pub fn f112(v0: u64, v1: Adt3, v2: Adt3) -> R<u64> {
    let v3: u64 = {
        let a2779 = v0;
        let a2780 = match v2.clone() {
            Adt3::C0(h2778, _, _, _, _, _, _, _) => {
                h2778
            }
        };
        let a2781 = 0u64;
        nat_quot(a2779, a2780, a2781)
    };
    let v4: u64 = {
        let a2783 = v3;
        let a2784 = match v2.clone() {
            Adt3::C0(_, h2782, _, _, _, _, _, _) => {
                h2782
            }
        };
        let a2785 = 0u64;
        nat_quot(a2783, a2784, a2785)
    };
    f150({
        let a2787 = v0;
        let a2788 = match v2.clone() {
            Adt3::C0(h2786, _, _, _, _, _, _, _) => {
                h2786
            }
        };
        let a2789 = 0u64;
        nat_rem(a2787, a2788, a2789)
    }, {
        let a2791 = v3;
        let a2792 = match v2.clone() {
            Adt3::C0(_, h2790, _, _, _, _, _, _) => {
                h2790
            }
        };
        let a2793 = 0u64;
        nat_rem(a2791, a2792, a2793)
    }, {
        let a2795 = v4;
        let a2796 = match v2.clone() {
            Adt3::C0(_, _, h2794, _, _, _, _, _) => {
                h2794
            }
        };
        let a2797 = 0u64;
        nat_rem(a2795, a2796, a2797)
    }, {
        let a2799 = v4;
        let a2800 = match v2 {
            Adt3::C0(_, _, h2798, _, _, _, _, _) => {
                h2798
            }
        };
        let a2801 = 0u64;
        nat_quot(a2799, a2800, a2801)
    }, match v1.clone() {
        Adt3::C0(h2802, _, _, _, _, _, _, _) => {
            h2802
        }
    }, match v1.clone() {
        Adt3::C0(_, h2803, _, _, _, _, _, _) => {
            h2803
        }
    }, match v1.clone() {
        Adt3::C0(_, _, h2804, _, _, _, _, _) => {
            h2804
        }
    }, match v1.clone() {
        Adt3::C0(_, _, _, h2805, _, _, _, _) => {
            h2805
        }
    }, match v1.clone() {
        Adt3::C0(_, _, _, _, h2806, _, _, _) => {
            h2806
        }
    }, match v1.clone() {
        Adt3::C0(_, _, _, _, _, h2807, _, _) => {
            h2807
        }
    }, match v1.clone() {
        Adt3::C0(_, _, _, _, _, _, h2808, _) => {
            h2808
        }
    }, match v1 {
        Adt3::C0(_, _, _, _, _, _, _, h2809) => {
            h2809
        }
    })
}

pub fn f113(v0: u64, v1: Adt3, v2: Adt3, v3: Adt3, v4: u64) -> R<Adt9> {
    let v5: u64 = {
        let a2811 = v0;
        let a2812 = match v3.clone() {
            Adt3::C0(h2810, _, _, _, _, _, _, _) => {
                h2810
            }
        };
        let a2813 = 0u64;
        nat_quot(a2811, a2812, a2813)
    };
    let v6: u64 = {
        let a2815 = v5;
        let a2816 = match v3.clone() {
            Adt3::C0(_, h2814, _, _, _, _, _, _) => {
                h2814
            }
        };
        let a2817 = 0u64;
        nat_quot(a2815, a2816, a2817)
    };
    let v7: u64 = {
        let a2819 = v0;
        let a2820 = match v3.clone() {
            Adt3::C0(h2818, _, _, _, _, _, _, _) => {
                h2818
            }
        };
        let a2821 = 0u64;
        nat_rem(a2819, a2820, a2821)
    };
    let v8: u64 = {
        let a2823 = v5;
        let a2824 = match v3.clone() {
            Adt3::C0(_, h2822, _, _, _, _, _, _) => {
                h2822
            }
        };
        let a2825 = 0u64;
        nat_rem(a2823, a2824, a2825)
    };
    let v9: u64 = {
        let a2827 = v6;
        let a2828 = match v3.clone() {
            Adt3::C0(_, _, h2826, _, _, _, _, _) => {
                h2826
            }
        };
        let a2829 = 0u64;
        nat_rem(a2827, a2828, a2829)
    };
    let v10: u64 = {
        let a2831 = v6;
        let a2832 = match v3.clone() {
            Adt3::C0(_, _, h2830, _, _, _, _, _) => {
                h2830
            }
        };
        let a2833 = 0u64;
        nat_quot(a2831, a2832, a2833)
    };
    let v11: u64 = {
        let a2834 = v4;
        let a2835 = 0u64;
        if nat_eq(a2834, a2835) {
            match v1.clone() {
                Adt3::C0(h2836, _, _, _, _, _, _, _) => {
                    h2836
                }
            }
        } else {
            let a2837 = v4;
            let a2838 = 1u64;
            if nat_eq(a2837, a2838) {
                match v1.clone() {
                    Adt3::C0(_, h2839, _, _, _, _, _, _) => {
                        h2839
                    }
                }
            } else {
                let a2840 = v4;
                let a2841 = 2u64;
                if nat_eq(a2840, a2841) {
                    match v1.clone() {
                        Adt3::C0(_, _, h2842, _, _, _, _, _) => {
                            h2842
                        }
                    }
                } else {
                    match v1.clone() {
                        Adt3::C0(_, _, _, h2843, _, _, _, _) => {
                            h2843
                        }
                    }
                }
            }
        }
    };
    let v12: bool = {
        let a2850 = v11;
        let a2851 = {
            let a2844 = v4;
            let a2845 = 0u64;
            if nat_eq(a2844, a2845) {
                v7
            } else {
                let a2846 = v4;
                let a2847 = 1u64;
                if nat_eq(a2846, a2847) {
                    v8
                } else {
                    let a2848 = v4;
                    let a2849 = 2u64;
                    if nat_eq(a2848, a2849) {
                        v9
                    } else {
                        v10
                    }
                }
            }
        };
        nat_le(a2850, a2851)
    };
    let v13: Adt3 = if v12 {
        v2
    } else {
        v1
    };
    Ok(Adt9::C1(v12, f104(if if v12 {
        let a2852 = v4;
        let a2853 = 0u64;
        nat_eq(a2852, a2853)
    } else {
        false
    } {
        let a2854 = v7;
        let a2855 = v11;
        nat_sub(a2854, a2855)
    } else {
        v7
    }, if if v12 {
        let a2856 = v4;
        let a2857 = 1u64;
        nat_eq(a2856, a2857)
    } else {
        false
    } {
        let a2858 = v8;
        let a2859 = v11;
        nat_sub(a2858, a2859)
    } else {
        v8
    }, if if v12 {
        let a2860 = v4;
        let a2861 = 2u64;
        nat_eq(a2860, a2861)
    } else {
        false
    } {
        let a2862 = v9;
        let a2863 = v11;
        nat_sub(a2862, a2863)
    } else {
        v9
    }, if if v12 {
        let a2864 = v4;
        let a2865 = 3u64;
        nat_eq(a2864, a2865)
    } else {
        false
    } {
        let a2866 = v10;
        let a2867 = v11;
        nat_sub(a2866, a2867)
    } else {
        v10
    }, match v13.clone() {
        Adt3::C0(_, _, _, _, h2868, _, _, _) => {
            h2868
        }
    }, match v13.clone() {
        Adt3::C0(_, _, _, _, _, h2869, _, _) => {
            h2869
        }
    }, match v13.clone() {
        Adt3::C0(_, _, _, _, _, _, h2870, _) => {
            h2870
        }
    }, match v13 {
        Adt3::C0(_, _, _, _, _, _, _, h2871) => {
            h2871
        }
    })?, f110(v0, v3)?))
}

pub fn f114(v0: u64, v1: Adt3, v2: Adt3, v3: u64, v4: u64, v5: u64, v6: u64, v7: bool) -> R<Adt9> {
    let v8: u64 = f59(v0, v2.clone(), 0u64);
    let v9: u64 = f59(v0, v2.clone(), 1u64);
    let v10: u64 = f59(v0, v2.clone(), 2u64);
    let v11: u64 = f59(v0, v2.clone(), 3u64);
    if v7 {
        Ok(Adt9::C1(false, f58(v1.clone(), f151(v8, v3, match v1.clone() {
            Adt3::C0(h2872, _, _, _, _, _, _, _) => {
                h2872
            }
        })?, f151(v9, v4, match v1.clone() {
            Adt3::C0(_, h2873, _, _, _, _, _, _) => {
                h2873
            }
        })?, f151(v10, v5, match v1.clone() {
            Adt3::C0(_, _, h2874, _, _, _, _, _) => {
                h2874
            }
        })?, f151(v11, v6, match v1 {
            Adt3::C0(_, _, _, h2875, _, _, _, _) => {
                h2875
            }
        })?)?, f110(v0, v2)?))
    } else {
        if if if if f152(v8, v3, match v1.clone() {
            Adt3::C0(h2876, _, _, _, _, _, _, _) => {
                h2876
            }
        })? {
            f152(v9, v4, match v1.clone() {
                Adt3::C0(_, h2877, _, _, _, _, _, _) => {
                    h2877
                }
            })?
        } else {
            false
        } {
            f152(v10, v5, match v1.clone() {
                Adt3::C0(_, _, h2878, _, _, _, _, _) => {
                    h2878
                }
            })?
        } else {
            false
        } {
            f152(v11, v6, match v1.clone() {
                Adt3::C0(_, _, _, h2879, _, _, _, _) => {
                    h2879
                }
            })?
        } else {
            false
        } {
            Ok(Adt9::C1(false, f58(v1, f153(v8, v3)?, f153(v9, v4)?, f153(v10, v5)?, f153(v11, v6)?)?, f110(v0, v2)?))
        } else {
            Ok(Adt9::C1(true, 0u64, f110(v0, v2)?))
        }
    }
}

pub fn f115(v0: u64, v1: Adt3, v2: Adt3, v3: u64) -> R<Adt9> {
    let v4: u64 = f59(v0, v2.clone(), 0u64);
    let v5: u64 = f59(v0, v2.clone(), 1u64);
    let v6: u64 = f59(v0, v2.clone(), 2u64);
    let v7: u64 = f59(v0, v2.clone(), 3u64);
    Ok(Adt9::C1(false, f58(v1.clone(), f154(v4, v3, match v1 {
        Adt3::C0(h2880, _, _, _, _, _, _, _) => {
            h2880
        }
    })?, v5, v6, v7)?, f110(v0, v2)?))
}

pub fn f116(v0: u64, v1: Adt3, v2: Adt3, v3: u64, v4: u64, v5: bool) -> R<Adt9> {
    let a2881 = v0;
    let a2882 = v3;
    if nat_lt(a2881, a2882) {
        Ok(Adt9::C1(false, {
            let a2883 = v0;
            let a2884 = 4u64;
            nat_mul(a2883, a2884)?
        }, {
            let a2885 = v0;
            let a2886 = 4u64;
            nat_mul(a2885, a2886)?
        }))
    } else {
        let v6: u64 = {
            let a2887 = v0;
            let a2888 = v3;
            nat_sub(a2887, a2888)
        };
        let v7: u64 = f110(v6, v1)?;
        let v8: u64 = f155(v6, v2, v4)?;
        if v5 {
            Ok(Adt9::C4(8u64, 0u64, v8, 4u64, v7, 3u64, v8))
        } else {
            Ok(Adt9::C1(true, v7, v8))
        }
    }
}

pub fn f117(v0: u64, v1: Adt3, v2: Adt3, v3: u64) -> R<Adt9> {
    let a2889 = v0;
    let a2890 = v3;
    if nat_lt(a2889, a2890) {
        Ok(Adt9::C1(true, 0u64, f110(v0, v2)?))
    } else {
        let v4: u64 = {
            let a2891 = v0;
            let a2892 = v3;
            nat_sub(a2891, a2892)
        };
        let v5: u64 = f112(v4, v2, v1.clone())?;
        Ok(Adt9::C4(8u64, 3u64, v5, 0u64, f110(v4, v1)?, 3u64, v5))
    }
}

pub fn f118(v0: u64, v1: u64, v2: Adt3, v3: Adt3) -> R<Adt9> {
    let v4: u64 = {
        let a2894 = v1;
        let a2895 = match v3.clone() {
            Adt3::C0(h2893, _, _, _, _, _, _, _) => {
                h2893
            }
        };
        let a2896 = 0u64;
        nat_rem(a2894, a2895, a2896)
    };
    let v5: u64 = {
        let a2902 = {
            let a2898 = v1;
            let a2899 = match v3.clone() {
                Adt3::C0(h2897, _, _, _, _, _, _, _) => {
                    h2897
                }
            };
            let a2900 = 0u64;
            nat_quot(a2898, a2899, a2900)
        };
        let a2903 = match v3.clone() {
            Adt3::C0(_, h2901, _, _, _, _, _, _) => {
                h2901
            }
        };
        let a2904 = 0u64;
        nat_rem(a2902, a2903, a2904)
    };
    let v6: bool = f156(v0, v4, v5);
    Ok(Adt9::C1(!v6, if v6 {
        f112(v1, v2, v3.clone())?
    } else {
        0u64
    }, f110(v1, v3)?))
}

pub fn f119(v0: u64, v1: Adt3, v2: Adt3, v3: u64, v4: bool, v5: Adt3) -> R<Adt9> {
    Ok(Adt9::C4({
        let a2905 = v3;
        let a2906 = 8u64;
        nat_add(a2905, a2906)?
    }, 0u64, f110(v0, v1)?, if v4 {
        5u64
    } else {
        4u64
    }, if v4 {
        0u64
    } else {
        f112(v0, v5, v2.clone())?
    }, 3u64, f110(v0, v2)?))
}

pub fn f120(v0: u64, v1: Adt3, v2: Adt3, v3: bool, v4: u64, v5: u64) -> R<Adt9> {
    let v6: u64 = {
        let a2908 = v0;
        let a2909 = match v2.clone() {
            Adt3::C0(h2907, _, _, _, _, _, _, _) => {
                h2907
            }
        };
        let a2910 = 0u64;
        nat_rem(a2908, a2909, a2910)
    };
    let v7: u64 = {
        let a2912 = v0;
        let a2913 = match v2.clone() {
            Adt3::C0(h2911, _, _, _, _, _, _, _) => {
                h2911
            }
        };
        let a2914 = 0u64;
        nat_quot(a2912, a2913, a2914)
    };
    let v8: u64 = {
        let a2916 = v7;
        let a2917 = match v2.clone() {
            Adt3::C0(_, h2915, _, _, _, _, _, _) => {
                h2915
            }
        };
        let a2918 = 0u64;
        nat_rem(a2916, a2917, a2918)
    };
    let v9: u64 = {
        let a2920 = v7;
        let a2921 = match v2.clone() {
            Adt3::C0(_, h2919, _, _, _, _, _, _) => {
                h2919
            }
        };
        let a2922 = 0u64;
        nat_quot(a2920, a2921, a2922)
    };
    let v10: u64 = {
        let a2924 = v9;
        let a2925 = match v2.clone() {
            Adt3::C0(_, _, h2923, _, _, _, _, _) => {
                h2923
            }
        };
        let a2926 = 0u64;
        nat_rem(a2924, a2925, a2926)
    };
    let v11: u64 = {
        let a2928 = v9;
        let a2929 = match v2.clone() {
            Adt3::C0(_, _, h2927, _, _, _, _, _) => {
                h2927
            }
        };
        let a2930 = 0u64;
        nat_quot(a2928, a2929, a2930)
    };
    let v12: u64 = f110(v0, v2)?;
    if v3 {
        let v13: u64 = {
            let a2939 = {
                let a2934 = {
                    let a2931 = v10;
                    let a2932 = v4;
                    let a2933 = 0u64;
                    nat_quot(a2931, a2932, a2933)
                };
                let a2935 = v5;
                nat_mul(a2934, a2935)?
            };
            let a2940 = {
                let a2936 = v8;
                let a2937 = v4;
                let a2938 = 0u64;
                nat_quot(a2936, a2937, a2938)
            };
            nat_add(a2939, a2940)?
        };
        let v14: u64 = {
            let a2960 = v6;
            let a2961 = {
                let a2958 = match v1.clone() {
                    Adt3::C0(h2941, _, _, _, _, _, _, _) => {
                        h2941
                    }
                };
                let a2959 = {
                    let a2956 = {
                        let a2942 = v8;
                        let a2943 = v4;
                        let a2944 = 0u64;
                        nat_rem(a2942, a2943, a2944)
                    };
                    let a2957 = {
                        let a2954 = match v1.clone() {
                            Adt3::C0(_, h2945, _, _, _, _, _, _) => {
                                h2945
                            }
                        };
                        let a2955 = {
                            let a2952 = {
                                let a2946 = v10;
                                let a2947 = v4;
                                let a2948 = 0u64;
                                nat_rem(a2946, a2947, a2948)
                            };
                            let a2953 = {
                                let a2950 = match v1.clone() {
                                    Adt3::C0(_, _, h2949, _, _, _, _, _) => {
                                        h2949
                                    }
                                };
                                let a2951 = v13;
                                nat_mul(a2950, a2951)?
                            };
                            nat_add(a2952, a2953)?
                        };
                        nat_mul(a2954, a2955)?
                    };
                    nat_add(a2956, a2957)?
                };
                nat_mul(a2958, a2959)?
            };
            nat_add(a2960, a2961)?
        };
        Ok(Adt9::C1(false, f110(v14, v1)?, v12))
    } else {
        let v15: u64 = {
            let a2967 = {
                let a2965 = {
                    let a2962 = v11;
                    let a2963 = v5;
                    let a2964 = 0u64;
                    nat_rem(a2962, a2963, a2964)
                };
                let a2966 = v4;
                nat_mul(a2965, a2966)?
            };
            let a2968 = v8;
            nat_add(a2967, a2968)?
        };
        let v16: u64 = {
            let a2974 = {
                let a2972 = {
                    let a2969 = v11;
                    let a2970 = v5;
                    let a2971 = 0u64;
                    nat_quot(a2969, a2970, a2971)
                };
                let a2973 = v4;
                nat_mul(a2972, a2973)?
            };
            let a2975 = v10;
            nat_add(a2974, a2975)?
        };
        let a2977 = v15;
        let a2978 = match v1.clone() {
            Adt3::C0(_, h2976, _, _, _, _, _, _) => {
                h2976
            }
        };
        if if nat_lt(a2977, a2978) {
            let a2980 = v16;
            let a2981 = match v1.clone() {
                Adt3::C0(_, _, h2979, _, _, _, _, _) => {
                    h2979
                }
            };
            nat_lt(a2980, a2981)
        } else {
            false
        } {
            Ok(Adt9::C1(false, f110({
                let a2990 = v6;
                let a2991 = {
                    let a2988 = match v1.clone() {
                        Adt3::C0(h2982, _, _, _, _, _, _, _) => {
                            h2982
                        }
                    };
                    let a2989 = {
                        let a2986 = v15;
                        let a2987 = {
                            let a2984 = match v1.clone() {
                                Adt3::C0(_, h2983, _, _, _, _, _, _) => {
                                    h2983
                                }
                            };
                            let a2985 = v16;
                            nat_mul(a2984, a2985)?
                        };
                        nat_add(a2986, a2987)?
                    };
                    nat_mul(a2988, a2989)?
                };
                nat_add(a2990, a2991)?
            }, v1)?, v12))
        } else {
            Ok(Adt9::C1(true, 0u64, v12))
        }
    }
}

pub fn f121(v0: u64, v1: Adt3, v2: Adt3) -> R<Adt9> {
    let v3: u64 = f110(v0, v2.clone())?;
    let v4: u64 = f110(v0, v1.clone())?;
    let a2996 = {
        let a2993 = v0;
        let a2994 = match v1 {
            Adt3::C0(h2992, _, _, _, _, _, _, _) => {
                h2992
            }
        };
        let a2995 = 0u64;
        nat_rem(a2993, a2994, a2995)
    };
    let a2997 = 0u64;
    if nat_eq(a2996, a2997) {
        Ok(Adt9::C1(false, v4, v3))
    } else {
        Ok(Adt9::C4(8u64, 3u64, f110({
            let a2998 = v0;
            let a2999 = 1u64;
            nat_sub(a2998, a2999)
        }, v2)?, 0u64, v4, 3u64, v3))
    }
}

pub fn f122(v0: u64, v1: Adt3, v2: Adt3, v3: u64, v4: u64) -> R<Adt9> {
    let a3000 = v3;
    let a3001 = 13u64;
    if nat_eq(a3000, a3001) {
        f157(v0, v1, v4)
    } else {
        let v5: u64 = f146(match v1.clone() {
            Adt3::C0(h3002, _, _, _, _, _, _, _) => {
                h3002
            }
        }, {
            let a3003 = v3;
            let a3004 = 16u64;
            nat_eq(a3003, a3004)
        })?;
        let v6: u64 = {
            let a3005 = v0;
            let a3006 = v5;
            let a3007 = 0u64;
            nat_quot(a3005, a3006, a3007)
        };
        let v7: u64 = {
            let a3008 = v0;
            let a3009 = v5;
            let a3010 = 0u64;
            nat_rem(a3008, a3009, a3010)
        };
        let v8: u64 = {
            let a3014 = 1u64;
            let a3015 = {
                let a3012 = match v1.clone() {
                    Adt3::C0(h3011, _, _, _, _, _, _, _) => {
                        h3011
                    }
                };
                let a3013 = 2u64;
                nat_mul(a3012, a3013)?
            };
            nat_add(a3014, a3015)?
        };
        let a3016 = v7;
        let a3017 = v8;
        if nat_lt(a3016, a3017) {
            f158(v7, v6, v1, 0u64)
        } else {
            let v9: u64 = f110(v6, v2)?;
            let a3018 = v3;
            let a3019 = 14u64;
            if nat_eq(a3018, a3019) {
                Ok(Adt9::C4(2u64, 1u64, 1u64, 2u64, 0u64, 3u64, v9))
            } else {
                let a3020 = v7;
                let a3021 = v8;
                if nat_eq(a3020, a3021) {
                    Ok(Adt9::C4(2u64, 1u64, 1u64, 2u64, 0u64, 1u64, 1u64))
                } else {
                    let a3024 = v7;
                    let a3025 = {
                        let a3022 = v8;
                        let a3023 = 1u64;
                        nat_add(a3022, a3023)?
                    };
                    if nat_eq(a3024, a3025) {
                        Ok(Adt9::C4(3u64, 2u64, match v1 {
                            Adt3::C0(h3026, _, _, _, _, _, _, _) => {
                                h3026
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

pub fn f123(v0: u64, v1: Adt3, v2: Adt3, v3: u64) -> R<Adt9> {
    let a3027 = v0;
    let a3028 = 0u64;
    if nat_eq(a3027, a3028) {
        Ok(Adt9::C4(19u64, 2u64, 0u64, 2u64, 0u64, 1u64, 0u64))
    } else {
        let a3035 = {
            let a3029 = v0;
            let a3030 = 1u64;
            nat_add(a3029, a3030)?
        };
        let a3036 = {
            let a3033 = 2u64;
            let a3034 = {
                let a3031 = v3;
                let a3032 = 2u64;
                nat_mul(a3031, a3032)?
            };
            nat_add(a3033, a3034)?
        };
        if nat_eq(a3035, a3036) {
            Ok(Adt9::C4(19u64, 1u64, 0u64, 2u64, 0u64, 6u64, 0u64))
        } else {
            let v4: u64 = {
                let a3037 = v0;
                let a3038 = 1u64;
                nat_sub(a3037, a3038)
            };
            let v5: u64 = {
                let a3039 = v4;
                let a3040 = 2u64;
                let a3041 = 0u64;
                nat_quot(a3039, a3040, a3041)
            };
            let a3045 = {
                let a3042 = v4;
                let a3043 = 2u64;
                let a3044 = 0u64;
                nat_rem(a3042, a3043, a3044)
            };
            let a3046 = 0u64;
            if nat_eq(a3045, a3046) {
                Ok(Adt9::C4(17u64, 0u64, f110(v5, v1)?, 4u64, f110(v5, v2)?, 1u64, 2u64))
            } else {
                Ok(Adt9::C4(18u64, 1u64, 0u64, 1u64, 2u64, 1u64, 0u64))
            }
        }
    }
}

pub fn f124(v0: u64, v1: u64, v2: u64, v3: u64) -> R<Adt9> {
    let a3047 = v0;
    let a3048 = f22(v1, v2, v3)?;
    if nat_lt(a3047, a3048) {
        let a3049 = v0;
        let a3050 = v1;
        if nat_lt(a3049, a3050) {
            Ok(Adt9::C1(false, {
                let a3051 = 4u64;
                let a3052 = v0;
                nat_mul(a3051, a3052)?
            }, {
                let a3053 = 4u64;
                let a3054 = v0;
                nat_mul(a3053, a3054)?
            }))
        } else {
            let v4: u64 = {
                let a3055 = v0;
                let a3056 = v1;
                nat_sub(a3055, a3056)
            };
            let v5: u64 = f159(v4, v2)?;
            let a3057 = v5;
            let a3058 = 2u64;
            if nat_lt(a3057, a3058) {
                Ok(Adt9::C4(6u64, {
                    let a3059 = v5;
                    let a3060 = 0u64;
                    if nat_eq(a3059, a3060) {
                        4u64
                    } else {
                        5u64
                    }
                }, f160(v4, v2)?, 2u64, 0u64, 1u64, v5))
            } else {
                let v6: u64 = f161(v4, v2)?;
                Ok(Adt9::C4(8u64, 3u64, v6, 1u64, if f162(v4, v2)? {
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

pub fn f125(v0: Adt3, v1: Adt3) -> R<u64> {
    let a3068 = {
        let a3061 = 2u64;
        let a3062 = f101(v1.clone())?;
        nat_mul(a3061, a3062)?
    };
    let a3069 = {
        let a3066 = f101(v1)?;
        let a3067 = {
            let a3064 = 1u64;
            let a3065 = match v0 {
                Adt3::C0(h3063, _, _, _, _, _, _, _) => {
                    h3063
                }
            };
            nat_add(a3064, a3065)?
        };
        nat_mul(a3066, a3067)?
    };
    nat_add(a3068, a3069)
}

pub fn f126(v0: u64, v1: Adt3) -> R<Adt9> {
    Ok(Adt9::C4(19u64, 4u64, f110(v0, v1)?, 2u64, 0u64, 1u64, 2u64))
}

pub fn f127(v0: u64, v1: u64) -> bool {
    let a3070 = v1;
    let a3071 = 2147483647u64;
    if nat_le(a3070, a3071) {
        let a3072 = v1;
        let a3073 = v0;
        nat_lt(a3072, a3073)
    } else {
        false
    }
}

pub fn f128(v0: u64, v1: u64, v2: u64, v3: Adt3, v4: Adt3, v5: Adt3, v6: u64) -> R<Adt9> {
    let m3074 = f90(v0);
    match m3074 {
        Adt14::C0 => {
            Ok(Adt9::C0(Adt10::C1))
        }
        Adt14::C1(v7) => {
            let v8: u64 = {
                let a3076 = v2;
                let a3077 = match v7.clone() {
                    Adt15::C0(_, h3075, _) => {
                        h3075
                    }
                };
                nat_mul(a3076, a3077)?
            };
            let m3090 = f163(v0, v8, v6, v1, match v5.clone() {
                Adt3::C0(h3078, _, _, _, _, _, _, _) => {
                    h3078
                }
            }, match v5.clone() {
                Adt3::C0(_, h3079, _, _, _, _, _, _) => {
                    h3079
                }
            }, match v5 {
                Adt3::C0(_, _, h3080, _, _, _, _, _) => {
                    h3080
                }
            }, {
                let a3083 = match v3.clone() {
                    Adt3::C0(h3081, _, _, _, _, _, _, _) => {
                        h3081
                    }
                };
                let a3084 = match v7.clone() {
                    Adt15::C0(_, h3082, _) => {
                        h3082
                    }
                };
                nat_mul(a3083, a3084)?
            }, match v3.clone() {
                Adt3::C0(_, h3085, _, _, _, _, _, _) => {
                    h3085
                }
            }, match v3.clone() {
                Adt3::C0(_, _, _, _, h3086, _, _, _) => {
                    h3086
                }
            }, match v3.clone() {
                Adt3::C0(_, _, _, _, _, h3087, _, _) => {
                    h3087
                }
            }, match v3.clone() {
                Adt3::C0(_, _, _, _, _, _, h3088, _) => {
                    h3088
                }
            }, match v3 {
                Adt3::C0(_, _, _, _, _, _, _, h3089) => {
                    h3089
                }
            })?;
            match m3090 {
                None => {
                    Ok(Adt9::C0(Adt10::C1))
                }
                Some(v9) => {
                    let a3091 = v0;
                    let a3092 = 0u64;
                    if if nat_eq(a3091, a3092) {
                        true
                    } else {
                        let a3093 = v0;
                        let a3094 = 26u64;
                        nat_eq(a3093, a3094)
                    } {
                        Ok(Adt9::C1(false, v9, f110({
                            let a3098 = {
                                let a3096 = v1;
                                let a3097 = match v4.clone() {
                                    Adt3::C0(h3095, _, _, _, _, _, _, _) => {
                                        h3095
                                    }
                                };
                                nat_mul(a3096, a3097)?
                            };
                            let a3099 = v8;
                            nat_add(a3098, a3099)?
                        }, v4)?))
                    } else {
                        Ok(Adt9::C5(v0, v9, match v7.clone() {
                            Adt15::C0(_, _, h3100) => {
                                h3100
                            }
                        }, f110({
                            let a3104 = {
                                let a3102 = v1;
                                let a3103 = match v4.clone() {
                                    Adt3::C0(h3101, _, _, _, _, _, _, _) => {
                                        h3101
                                    }
                                };
                                nat_mul(a3102, a3103)?
                            };
                            let a3105 = v8;
                            nat_add(a3104, a3105)?
                        }, v4.clone())?, {
                            let a3112 = {
                                let a3110 = {
                                    let a3107 = match v7 {
                                        Adt15::C0(_, h3106, _) => {
                                            h3106
                                        }
                                    };
                                    let a3108 = 1u64;
                                    nat_sub(a3107, a3108)
                                };
                                let a3111 = match v4.clone() {
                                    Adt3::C0(_, _, _, _, h3109, _, _, _) => {
                                        h3109
                                    }
                                };
                                nat_mul(a3110, a3111)?
                            };
                            let a3113 = 4u64;
                            nat_add(a3112, a3113)?
                        }, match v4 {
                            Adt3::C0(_, _, _, _, h3114, _, _, _) => {
                                h3114
                            }
                        }))
                    }
                }
            }
        }
    }
}

pub fn f129(v0: u64, v1: u64, v2: u64) -> u64 {
    let a3115 = v1;
    let a3116 = 0u64;
    if if nat_eq(a3115, a3116) {
        true
    } else {
        let a3117 = v2;
        let a3118 = 0u64;
        nat_eq(a3117, a3118)
    } {
        0u64
    } else {
        v0
    }
}

pub fn f130(v0: u64, v1: u64) -> bool {
    let a3119 = v0;
    let a3120 = 4294967295u64;
    if nat_eq(a3119, a3120) {
        true
    } else {
        let a3121 = v0;
        let a3122 = 2147483648u64;
        if nat_lt(a3121, a3122) {
            let a3123 = v0;
            let a3124 = v1;
            nat_lt(a3123, a3124)
        } else {
            false
        }
    }
}

pub fn f131(v0: Adt7) -> R<u64> {
    let a3129 = if match v0.clone() {
        Adt7::C0(_, _, _, h3125, _) => {
            h3125
        }
    } {
        2u64
    } else {
        1u64
    };
    let a3130 = {
        let a3127 = match v0 {
            Adt7::C0(_, _, _, _, h3126) => {
                h3126
            }
        };
        let a3128 = 1u64;
        if nat_eq(a3127, a3128) {
            1u64
        } else {
            0u64
        }
    };
    nat_add(a3129, a3130)
}

pub fn f132(v0: Adt6, v1: u64, v2: u64) -> R<u64> {
    let v3: u64 = {
        let a3135 = {
            let a3132 = v2;
            let a3133 = match v0.clone() {
                Adt6::C0(_, h3131, _, _) => {
                    h3131
                }
            };
            nat_mul(a3132, a3133)?
        };
        let a3136 = f75(match v0.clone() {
            Adt6::C0(_, _, h3134, _) => {
                h3134
            }
        });
        nat_add(a3135, a3136)?
    };
    let v4: u64 = f74(match v0.clone() {
        Adt6::C0(_, _, h3137, _) => {
            h3137
        }
    });
    let v5: u64 = {
        let a3141 = {
            let a3139 = v3;
            let a3140 = match v0 {
                Adt6::C0(h3138, _, _, _) => {
                    h3138
                }
            };
            nat_add(a3139, a3140)?
        };
        let a3142 = v4;
        nat_sub(a3141, a3142)
    };
    let a3147 = {
        let a3143 = v5;
        let a3144 = v1;
        if nat_lt(a3143, a3144) {
            v5
        } else {
            v1
        }
    };
    let a3148 = {
        let a3145 = v3;
        let a3146 = v4;
        nat_sub(a3145, a3146)
    };
    Ok(nat_sub(a3147, a3148))
}

pub fn f133(v0: Adt7, v1: Adt3, v2: Adt3, v3: u64, v4: u64) -> R<Option<u64>> {
    let m3157 = f60(match v0.clone() {
        Adt7::C0(h3149, _, _, _, _) => {
            h3149
        }
    }, match v1.clone() {
        Adt3::C0(h3150, _, _, _, _, _, _, _) => {
            h3150
        }
    }, f59(v3, v2.clone(), 0u64), {
        let a3154 = v4;
        let a3155 = {
            let m3153 = match v0.clone() {
                Adt7::C0(h3152, _, _, _, _) => {
                    h3152
                }
            };
            match m3153 {
                Adt6::C0(h3151, _, _, _) => {
                    h3151
                }
            }
        };
        let a3156 = 0u64;
        nat_rem(a3154, a3155, a3156)
    })?;
    match m3157 {
        None => {
            Ok(None::<u64>)
        }
        Some(v5) => {
            let m3166 = f60(match v0.clone() {
                Adt7::C0(_, h3158, _, _, _) => {
                    h3158
                }
            }, match v1.clone() {
                Adt3::C0(_, h3159, _, _, _, _, _, _) => {
                    h3159
                }
            }, f59(v3, v2.clone(), 1u64), {
                let a3163 = v4;
                let a3164 = {
                    let m3162 = match v0 {
                        Adt7::C0(h3161, _, _, _, _) => {
                            h3161
                        }
                    };
                    match m3162 {
                        Adt6::C0(h3160, _, _, _) => {
                            h3160
                        }
                    }
                };
                let a3165 = 0u64;
                nat_quot(a3163, a3164, a3165)
            })?;
            match m3166 {
                None => {
                    Ok(None::<u64>)
                }
                Some(v6) => {
                    Ok(Some(f58(v1, v5, v6, f59(v3, v2.clone(), 2u64), f59(v3, v2, 3u64))?))
                }
            }
        }
    }
}

pub fn f134(v0: u64, v1: Adt3, v2: u64, v3: u64, v4: u64) -> R<Adt9> {
    let v5: u64 = {
        let a3173 = v3;
        let a3174 = {
            let a3170 = f110({
                let a3167 = v0;
                let a3168 = 2u64;
                let a3169 = 0u64;
                nat_quot(a3167, a3168, a3169)
            }, v1)?;
            let a3171 = v2;
            let a3172 = 0u64;
            nat_quot(a3170, a3171, a3172)
        };
        nat_add(a3173, a3174)?
    };
    let a3178 = {
        let a3175 = v0;
        let a3176 = 2u64;
        let a3177 = 0u64;
        nat_rem(a3175, a3176, a3177)
    };
    let a3179 = 0u64;
    if nat_eq(a3178, a3179) {
        Ok(Adt9::C4(19u64, 1u64, v5, 2u64, 0u64, 1u64, 2u64))
    } else {
        let a3180 = v4;
        let a3181 = 0u64;
        if nat_eq(a3180, a3181) {
            Ok(Adt9::C4(19u64, 2u64, 1u64, 2u64, 0u64, 1u64, v5))
        } else {
            Ok(Adt9::C0(Adt10::C1))
        }
    }
}

pub fn f135(v0: u64, v1: u64) -> R<u64> {
    let a3184 = 8u64;
    let a3185 = {
        let a3182 = f164(v0);
        let a3183 = v1;
        nat_mul(a3182, a3183)?
    };
    nat_add(a3184, a3185)
}

pub fn f136(v0: u64, v1: u64, v2: Adt3, v3: u64, v4: u64, v5: u64) -> R<Adt9> {
    let v6: u64 = match v2.clone() {
        Adt3::C0(h3186, _, _, _, _, _, _, _) => {
            h3186
        }
    };
    let v7: u64 = f164(v3);
    let v8: u64 = {
        let a3189 = 1u64;
        let a3190 = {
            let a3187 = v6;
            let a3188 = v7;
            nat_mul(a3187, a3188)?
        };
        nat_add(a3189, a3190)?
    };
    let a3191 = v0;
    let a3192 = 0u64;
    if nat_eq(a3191, a3192) {
        Ok(f165(0u64))
    } else {
        let a3193 = v0;
        let a3194 = v8;
        if nat_lt(a3193, a3194) {
            let v9: u64 = {
                let a3195 = v0;
                let a3196 = 1u64;
                nat_sub(a3195, a3196)
            };
            let v10: u64 = {
                let a3197 = v9;
                let a3198 = v7;
                let a3199 = 0u64;
                nat_rem(a3197, a3198, a3199)
            };
            let v11: u64 = f110({
                let a3205 = {
                    let a3200 = v1;
                    let a3201 = v6;
                    nat_mul(a3200, a3201)?
                };
                let a3206 = {
                    let a3202 = v9;
                    let a3203 = v7;
                    let a3204 = 0u64;
                    nat_quot(a3202, a3203, a3204)
                };
                nat_add(a3205, a3206)?
            }, v2)?;
            let a3207 = v3;
            let a3208 = 30u64;
            if nat_eq(a3207, a3208) {
                let a3209 = v10;
                let a3210 = 0u64;
                if nat_eq(a3209, a3210) {
                    Ok(Adt9::C4(6u64, 0u64, v11, 2u64, 0u64, 1u64, 1u64))
                } else {
                    let a3211 = v10;
                    let a3212 = 1u64;
                    if nat_eq(a3211, a3212) {
                        Ok(Adt9::C4(10u64, 1u64, 1u64, 1u64, 1u64, 1u64, 1u64))
                    } else {
                        let a3213 = v10;
                        let a3214 = 2u64;
                        if nat_eq(a3213, a3214) {
                            Ok(Adt9::C4(1u64, 1u64, 1u64, 2u64, 0u64, 1u64, 1u64))
                        } else {
                            Ok(Adt9::C4(0u64, 1u64, 0u64, 1u64, 1u64, 1u64, 0u64))
                        }
                    }
                }
            } else {
                let a3215 = v10;
                let a3216 = 0u64;
                if nat_eq(a3215, a3216) {
                    Ok(Adt9::C4(10u64, 0u64, v11, 0u64, v11, 1u64, 1u64))
                } else {
                    let a3217 = v10;
                    let a3218 = 1u64;
                    if nat_eq(a3217, a3218) {
                        Ok(Adt9::C4(1u64, 1u64, 1u64, 2u64, 0u64, 1u64, 1u64))
                    } else {
                        Ok(Adt9::C4(0u64, 1u64, 0u64, 1u64, 1u64, 1u64, 0u64))
                    }
                }
            }
        } else {
            let v12: u64 = {
                let a3219 = v0;
                let a3220 = v8;
                nat_sub(a3219, a3220)
            };
            let a3221 = v12;
            let a3222 = 0u64;
            if nat_eq(a3221, a3222) {
                Ok(Adt9::C4(22u64, 2u64, v6, 2u64, 0u64, 1u64, 1u64))
            } else {
                let a3223 = v12;
                let a3224 = 1u64;
                if nat_eq(a3223, a3224) {
                    Ok(Adt9::C4(21u64, 1u64, 0u64, 1u64, 1u64, 1u64, 1u64))
                } else {
                    let a3225 = v12;
                    let a3226 = 2u64;
                    if nat_eq(a3225, a3226) {
                        Ok(Adt9::C4(2u64, 1u64, 1u64, 2u64, 0u64, 1u64, 1u64))
                    } else {
                        let a3227 = v12;
                        let a3228 = 3u64;
                        if nat_eq(a3227, a3228) {
                            Ok(Adt9::C4(4u64, 1u64, 1u64, 2u64, v4, 1u64, 1u64))
                        } else {
                            let a3229 = v12;
                            let a3230 = 4u64;
                            if nat_eq(a3229, a3230) {
                                Ok(Adt9::C4(12u64, 1u64, 1u64, 2u64, 0u64, 1u64, 1u64))
                            } else {
                                let a3231 = v12;
                                let a3232 = 5u64;
                                if nat_eq(a3231, a3232) {
                                    Ok(Adt9::C4(11u64, 2u64, 1065353216u64, 1u64, 1u64, 1u64, 2u64))
                                } else {
                                    let a3233 = v12;
                                    let a3234 = 6u64;
                                    if nat_eq(a3233, a3234) {
                                        if f166(v5) {
                                            Ok(Adt9::C4(19u64, 1u64, 2u64, 2u64, 0u64, 1u64, {
                                                let a3235 = 3u64;
                                                let a3236 = v1;
                                                nat_add(a3235, a3236)?
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

pub fn f137(v0: u64, v1: Adt3, v2: Adt3, v3: u64) -> R<Adt9> {
    let v4: u64 = {
        let a3237 = v0;
        let a3238 = f82(v3);
        let a3239 = 0u64;
        nat_quot(a3237, a3238, a3239)
    };
    let v5: u64 = {
        let a3244 = 3u64;
        let a3245 = {
            let a3241 = v4;
            let a3242 = match v1.clone() {
                Adt3::C0(h3240, _, _, _, _, _, _, _) => {
                    h3240
                }
            };
            let a3243 = 0u64;
            nat_quot(a3241, a3242, a3243)
        };
        nat_add(a3244, a3245)?
    };
    let v6: u64 = f110(v4, v1)?;
    let v7: u64 = f110(v4, v2)?;
    let a3246 = v3;
    let a3247 = 30u64;
    if nat_eq(a3246, a3247) {
        let a3251 = {
            let a3248 = v0;
            let a3249 = 3u64;
            let a3250 = 0u64;
            nat_rem(a3248, a3249, a3250)
        };
        let a3252 = 0u64;
        if nat_eq(a3251, a3252) {
            Ok(Adt9::C4(6u64, 0u64, v6, 2u64, 0u64, 1u64, 1u64))
        } else {
            let a3256 = {
                let a3253 = v0;
                let a3254 = 3u64;
                let a3255 = 0u64;
                nat_rem(a3253, a3254, a3255)
            };
            let a3257 = 1u64;
            if nat_eq(a3256, a3257) {
                Ok(Adt9::C4(10u64, 1u64, 1u64, 1u64, v5, 1u64, 1u64))
            } else {
                Ok(Adt9::C4(7u64, 1u64, 1u64, 2u64, 0u64, 3u64, v7))
            }
        }
    } else {
        Ok(Adt9::C4(10u64, 0u64, v6, 1u64, v5, 3u64, v7))
    }
}

pub fn f138(v0: u64) -> R<u64> {
    let v1: u64 = {
        let a3258 = v0;
        let a3259 = 8388608u64;
        let a3260 = 0u64;
        nat_quot(a3258, a3259, a3260)
    };
    let a3261 = v0;
    let a3262 = 2147483648u64;
    if if if nat_lt(a3261, a3262) {
        let a3263 = 127u64;
        let a3264 = v1;
        nat_le(a3263, a3264)
    } else {
        false
    } {
        let a3265 = v1;
        let a3266 = 190u64;
        nat_lt(a3265, a3266)
    } else {
        false
    } {
        let v2: u64 = {
            let a3270 = 8388608u64;
            let a3271 = {
                let a3267 = v0;
                let a3268 = 8388608u64;
                let a3269 = 0u64;
                nat_rem(a3267, a3268, a3269)
            };
            nat_add(a3270, a3271)?
        };
        let a3272 = v1;
        let a3273 = 150u64;
        if nat_lt(a3272, a3273) {
            let v3: u64 = f167({
                let a3274 = 150u64;
                let a3275 = v1;
                nat_sub(a3274, a3275)
            });
            let a3279 = {
                let a3276 = v2;
                let a3277 = v3;
                let a3278 = 0u64;
                nat_rem(a3276, a3277, a3278)
            };
            let a3280 = 0u64;
            if nat_eq(a3279, a3280) {
                let a3281 = v2;
                let a3282 = v3;
                let a3283 = 0u64;
                Ok(nat_quot(a3281, a3282, a3283))
            } else {
                Ok(0u64)
            }
        } else {
            let a3286 = f167({
                let a3284 = v1;
                let a3285 = 150u64;
                nat_sub(a3284, a3285)
            });
            let a3287 = v2;
            nat_mul(a3286, a3287)
        }
    } else {
        Ok(0u64)
    }
}

pub fn f139(v0: u64, v1: u64, v2: u64) -> bool {
    let a3288 = v0;
    let a3289 = 1u64;
    if nat_le(a3288, a3289) {
        true
    } else {
        let a3293 = {
            let a3290 = v1;
            let a3291 = v2;
            let a3292 = 1u64;
            nat_rem(a3290, a3291, a3292)
        };
        let a3294 = 0u64;
        nat_eq(a3293, a3294)
    }
}

pub fn f140(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64) -> R<bool> {
    let a3295 = v1;
    let a3296 = 1u64;
    if if if nat_le(a3295, a3296) {
        true
    } else {
        let a3297 = v0;
        let a3298 = v2;
        nat_le(a3297, a3298)
    } {
        let a3299 = v3;
        let a3300 = 1u64;
        if nat_le(a3299, a3300) {
            true
        } else {
            let a3307 = {
                let a3305 = v0;
                let a3306 = {
                    let a3303 = {
                        let a3301 = v1;
                        let a3302 = 1u64;
                        nat_sub(a3301, a3302)
                    };
                    let a3304 = v2;
                    nat_mul(a3303, a3304)?
                };
                nat_add(a3305, a3306)?
            };
            let a3308 = v4;
            nat_le(a3307, a3308)
        }
    } else {
        false
    } {
        let a3309 = v5;
        let a3310 = 1u64;
        if nat_le(a3309, a3310) {
            Ok(true)
        } else {
            let a3323 = {
                let a3321 = {
                    let a3315 = v0;
                    let a3316 = {
                        let a3313 = {
                            let a3311 = v1;
                            let a3312 = 1u64;
                            nat_sub(a3311, a3312)
                        };
                        let a3314 = v2;
                        nat_mul(a3313, a3314)?
                    };
                    nat_add(a3315, a3316)?
                };
                let a3322 = {
                    let a3319 = {
                        let a3317 = v3;
                        let a3318 = 1u64;
                        nat_sub(a3317, a3318)
                    };
                    let a3320 = v4;
                    nat_mul(a3319, a3320)?
                };
                nat_add(a3321, a3322)?
            };
            let a3324 = v6;
            Ok(nat_le(a3323, a3324))
        }
    } else {
        Ok(false)
    }
}

pub fn f141(v0: u64, v1: u64, _: u64, v3: u64, v4: u64, v5: u64, v6: u64, v7: u64, v8: u64, v9: u64, v10: u64, v11: u64, v12: u64, v13: u64, v14: u64) -> bool {
    if if if if if if if if if f168(v0, v1, v6, v7, v8) {
        let a3325 = v5;
        let a3326 = 26u64;
        nat_eq(a3325, a3326)
    } else {
        false
    } {
        let a3327 = v3;
        let a3328 = v7;
        nat_eq(a3327, a3328)
    } else {
        false
    } {
        let a3329 = v4;
        let a3330 = v8;
        nat_eq(a3329, a3330)
    } else {
        false
    } {
        let a3331 = v9;
        let a3332 = 1u64;
        nat_eq(a3331, a3332)
    } else {
        false
    } {
        let a3333 = v10;
        let a3334 = f169(v0);
        nat_eq(a3333, a3334)
    } else {
        false
    } {
        let a3335 = v11;
        let a3336 = v1;
        nat_eq(a3335, a3336)
    } else {
        false
    } {
        let a3337 = v12;
        let a3338 = v6;
        nat_eq(a3337, a3338)
    } else {
        false
    } {
        let a3339 = v13;
        let a3340 = v7;
        nat_eq(a3339, a3340)
    } else {
        false
    } {
        let a3341 = v14;
        let a3342 = v8;
        nat_eq(a3341, a3342)
    } else {
        false
    }
}

pub fn f142(v0: u64, v1: u64, v2: u64, v3: u64) -> bool {
    let a3343 = v0;
    let a3344 = 0u64;
    if if if nat_eq(a3343, a3344) {
        true
    } else {
        let a3345 = v1;
        let a3346 = 0u64;
        nat_eq(a3345, a3346)
    } {
        true
    } else {
        let a3347 = v2;
        let a3348 = 0u64;
        nat_eq(a3347, a3348)
    } {
        true
    } else {
        let a3349 = v3;
        let a3350 = 0u64;
        nat_eq(a3349, a3350)
    }
}

pub fn f143(v0: Adt3, v1: bool) -> R<u64> {
    f170(v0.clone(), {
        let a3352 = match v0.clone() {
            Adt3::C0(h3351, _, _, _, _, _, _, _) => {
                h3351
            }
        };
        let a3353 = 1u64;
        nat_sub(a3352, a3353)
    }, {
        let a3355 = match v0.clone() {
            Adt3::C0(_, h3354, _, _, _, _, _, _) => {
                h3354
            }
        };
        let a3356 = 1u64;
        nat_sub(a3355, a3356)
    }, {
        let a3358 = match v0.clone() {
            Adt3::C0(_, _, h3357, _, _, _, _, _) => {
                h3357
            }
        };
        let a3359 = 1u64;
        nat_sub(a3358, a3359)
    }, {
        let a3361 = match v0 {
            Adt3::C0(_, _, _, h3360, _, _, _, _) => {
                h3360
            }
        };
        let a3362 = 1u64;
        nat_sub(a3361, a3362)
    }, v1)
}

pub fn f144(v0: u64, v1: u64, v2: u64, v3: u64) -> R<u64> {
    let a3363 = v0;
    let a3364 = 0u64;
    if nat_eq(a3363, a3364) {
        Ok(0u64)
    } else {
        let v4: u64 = {
            let a3365 = v0;
            let a3366 = 1u64;
            nat_sub(a3365, a3366)
        };
        let v5: u64 = {
            let a3367 = v4;
            let a3368 = v1;
            let a3369 = 0u64;
            nat_quot(a3367, a3368, a3369)
        };
        let v6: u64 = {
            let a3377 = {
                let a3370 = v5;
                let a3371 = v2;
                nat_mul(a3370, a3371)?
            };
            let a3378 = {
                let a3375 = {
                    let a3372 = v4;
                    let a3373 = v1;
                    let a3374 = 0u64;
                    nat_rem(a3372, a3373, a3374)
                };
                let a3376 = v3;
                nat_mul(a3375, a3376)?
            };
            nat_add(a3377, a3378)?
        };
        let a3379 = v5;
        let a3380 = 0u64;
        if nat_eq(a3379, a3380) {
            Ok(v6)
        } else {
            let v7: u64 = {
                let a3389 = {
                    let a3383 = {
                        let a3381 = v5;
                        let a3382 = 1u64;
                        nat_sub(a3381, a3382)
                    };
                    let a3384 = v2;
                    nat_mul(a3383, a3384)?
                };
                let a3390 = {
                    let a3387 = {
                        let a3385 = v1;
                        let a3386 = 1u64;
                        nat_sub(a3385, a3386)
                    };
                    let a3388 = v3;
                    nat_mul(a3387, a3388)?
                };
                nat_add(a3389, a3390)?
            };
            let a3391 = v6;
            let a3392 = v7;
            if nat_lt(a3391, a3392) {
                Ok(v7)
            } else {
                Ok(v6)
            }
        }
    }
}

pub fn f145(v0: Adt3, v1: u64) -> bool {
    let a3393 = v1;
    let a3394 = 0u64;
    if !nat_eq(a3393, a3394) {
        true
    } else {
        let a3396 = match v0 {
            Adt3::C0(_, _, _, _, h3395, _, _, _) => {
                h3395
            }
        };
        let a3397 = 4u64;
        nat_eq(a3396, a3397)
    }
}

pub fn f146(v0: u64, v1: bool) -> R<u64> {
    let a3400 = {
        let a3398 = v0;
        let a3399 = 2u64;
        nat_mul(a3398, a3399)?
    };
    let a3401 = if v1 {
        4u64
    } else {
        2u64
    };
    nat_add(a3400, a3401)
}

pub fn f147(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: bool, v6: u64, v7: u64, v8: u64, v9: u64, v10: u64, v11: u64, v12: u64, v13: u64, v14: u64, v15: u64, v16: u64, v17: u64) -> R<Adt10> {
    let m3402 = f171(v0, v1);
    match m3402 {
        None => {
            if v5 {
                let m3403 = f172(v0, v2, v3, v4, v6, v7, v8, v9, v10, v11, v12, v13, v14, v15, v16, v17)?;
                match m3403 {
                    Adt16::C0 => {
                        Ok(Adt10::C0)
                    }
                    Adt16::C1 => {
                        Ok(Adt10::C1)
                    }
                    Adt16::C2(v18, v19, v20, v21, v22, v23, v24, v25, v26, v27) => {
                        Ok(Adt10::C3(v18, v19, v20, v21, v22, v23, v24, v25, v26, v27))
                    }
                }
            } else {
                Ok(Adt10::C1)
            }
        }
        Some(v28) => {
            let v29: bool = {
                let a3404 = 4u64;
                let a3405 = v28;
                nat_le(a3404, a3405)
            };
            let a3406 = v2;
            let a3407 = 0u64;
            if if if if if nat_eq(a3406, a3407) {
                let a3408 = v4;
                let a3409 = 0u64;
                nat_eq(a3408, a3409)
            } else {
                false
            } {
                let a3410 = v6;
                let a3411 = v14;
                if nat_eq(a3410, a3411) {
                    let a3412 = v7;
                    let a3413 = v15;
                    nat_eq(a3412, a3413)
                } else {
                    false
                }
            } else {
                false
            } {
                let a3414 = v8;
                let a3415 = v16;
                if nat_eq(a3414, a3415) {
                    let a3416 = v9;
                    let a3417 = v17;
                    nat_eq(a3416, a3417)
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
                        let a3418 = v3;
                        let a3419 = 0u64;
                        nat_eq(a3418, a3419)
                    } else {
                        false
                    } {
                        f91(v6, v7, v8, v9, v10, v11, v12, v13)
                    } else {
                        false
                    }
                }
            } else {
                false
            } {
                Ok(Adt10::C2(v28, v29, v6, v7, v8, v9, {
                    let a3424 = {
                        let a3420 = v6;
                        let a3421 = v7;
                        nat_mul(a3420, a3421)?
                    };
                    let a3425 = {
                        let a3422 = v8;
                        let a3423 = v9;
                        nat_mul(a3422, a3423)?
                    };
                    nat_mul(a3424, a3425)?
                }))
            } else {
                Ok(Adt10::C1)
            }
        }
    }
}

pub fn f148(v0: u64) -> u64 {
    let m3426 = f90(v0);
    match m3426 {
        Adt14::C0 => {
            0u64
        }
        Adt14::C1(v1) => {
            let a3428 = match v1.clone() {
                Adt15::C0(_, h3427, _) => {
                    h3427
                }
            };
            let a3429 = 1u64;
            if if nat_eq(a3428, a3429) {
                let a3431 = match v1.clone() {
                    Adt15::C0(_, _, h3430) => {
                        h3430
                    }
                };
                let a3432 = 1u64;
                if if if nat_eq(a3431, a3432) {
                    true
                } else {
                    let a3434 = match v1.clone() {
                        Adt15::C0(_, _, h3433) => {
                            h3433
                        }
                    };
                    let a3435 = 2u64;
                    nat_eq(a3434, a3435)
                } {
                    true
                } else {
                    let a3437 = match v1.clone() {
                        Adt15::C0(_, _, h3436) => {
                            h3436
                        }
                    };
                    let a3438 = 4u64;
                    nat_eq(a3437, a3438)
                } {
                    true
                } else {
                    let a3440 = match v1.clone() {
                        Adt15::C0(_, _, h3439) => {
                            h3439
                        }
                    };
                    let a3441 = 8u64;
                    nat_eq(a3440, a3441)
                }
            } else {
                false
            } {
                match v1 {
                    Adt15::C0(_, _, h3442) => {
                        h3442
                    }
                }
            } else {
                0u64
            }
        }
    }
}

pub fn f149(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let a3447 = {
        let a3445 = {
            let a3443 = v0;
            let a3444 = v1;
            nat_sub(a3443, a3444)
        };
        let a3446 = 1u64;
        nat_sub(a3445, a3446)
    };
    let a3448 = v2;
    nat_add(a3447, a3448)
}

pub fn f150(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64, v7: u64, v8: u64, v9: u64, v10: u64, v11: u64) -> R<u64> {
    f104({
        let a3449 = v0;
        let a3450 = v4;
        let a3451 = 0u64;
        nat_rem(a3449, a3450, a3451)
    }, {
        let a3452 = v1;
        let a3453 = v5;
        let a3454 = 0u64;
        nat_rem(a3452, a3453, a3454)
    }, {
        let a3455 = v2;
        let a3456 = v6;
        let a3457 = 0u64;
        nat_rem(a3455, a3456, a3457)
    }, {
        let a3458 = v3;
        let a3459 = v7;
        let a3460 = 0u64;
        nat_rem(a3458, a3459, a3460)
    }, v8, v9, v10, v11)
}

pub fn f151(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let a3461 = v1;
    let a3462 = 2147483648u64;
    if nat_lt(a3461, a3462) {
        let a3473 = {
            let a3471 = {
                let a3466 = {
                    let a3463 = v0;
                    let a3464 = v2;
                    let a3465 = 0u64;
                    nat_rem(a3463, a3464, a3465)
                };
                let a3467 = v2;
                nat_add(a3466, a3467)?
            };
            let a3472 = {
                let a3468 = v1;
                let a3469 = v2;
                let a3470 = 0u64;
                nat_rem(a3468, a3469, a3470)
            };
            nat_sub(a3471, a3472)
        };
        let a3474 = v2;
        let a3475 = 0u64;
        Ok(nat_rem(a3473, a3474, a3475))
    } else {
        let a3478 = {
            let a3476 = v0;
            let a3477 = f75(v1);
            nat_add(a3476, a3477)?
        };
        let a3479 = v2;
        let a3480 = 0u64;
        Ok(nat_rem(a3478, a3479, a3480))
    }
}

pub fn f152(v0: u64, v1: u64, v2: u64) -> R<bool> {
    let a3483 = f74(v1);
    let a3484 = {
        let a3481 = v0;
        let a3482 = f75(v1);
        nat_add(a3481, a3482)?
    };
    if nat_le(a3483, a3484) {
        let a3485 = f153(v0, v1)?;
        let a3486 = v2;
        Ok(nat_lt(a3485, a3486))
    } else {
        Ok(false)
    }
}

pub fn f153(v0: u64, v1: u64) -> R<u64> {
    let a3489 = {
        let a3487 = v0;
        let a3488 = f75(v1);
        nat_add(a3487, a3488)?
    };
    let a3490 = f74(v1);
    Ok(nat_sub(a3489, a3490))
}

pub fn f154(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let a3491 = v0;
    let a3492 = v1;
    if nat_lt(a3491, a3492) {
        let a3493 = v1;
        let a3494 = v0;
        Ok(nat_sub(a3493, a3494))
    } else {
        let a3497 = v0;
        let a3498 = {
            let a3495 = v1;
            let a3496 = v2;
            nat_add(a3495, a3496)?
        };
        if nat_lt(a3497, a3498) {
            let a3499 = v0;
            let a3500 = v1;
            Ok(nat_sub(a3499, a3500))
        } else {
            let a3507 = {
                let a3503 = 2u64;
                let a3504 = {
                    let a3501 = v2;
                    let a3502 = 1u64;
                    nat_sub(a3501, a3502)
                };
                nat_mul(a3503, a3504)?
            };
            let a3508 = {
                let a3505 = v0;
                let a3506 = v1;
                nat_sub(a3505, a3506)
            };
            Ok(nat_sub(a3507, a3508))
        }
    }
}

pub fn f155(v0: u64, v1: Adt3, v2: u64) -> R<u64> {
    let a3511 = {
        let a3509 = v2;
        let a3510 = f170(v1.clone(), f59(v0, v1.clone(), 0u64), f59(v0, v1.clone(), 1u64), f59(v0, v1.clone(), 2u64), f59(v0, v1.clone(), 3u64), false)?;
        nat_add(a3509, a3510)?
    };
    let a3512 = f170(v1.clone(), f59(v0, v1.clone(), 0u64), f59(v0, v1.clone(), 1u64), f59(v0, v1.clone(), 2u64), f59(v0, v1, 3u64), true)?;
    Ok(nat_sub(a3511, a3512))
}

pub fn f156(v0: u64, v1: u64, v2: u64) -> bool {
    let a3513 = v0;
    let a3514 = 0u64;
    if nat_eq(a3513, a3514) {
        let a3515 = v2;
        let a3516 = v1;
        nat_le(a3515, a3516)
    } else {
        let a3517 = v0;
        let a3518 = 1u64;
        if nat_eq(a3517, a3518) {
            let a3519 = v2;
            let a3520 = v1;
            nat_lt(a3519, a3520)
        } else {
            let a3521 = v0;
            let a3522 = 2u64;
            if nat_eq(a3521, a3522) {
                let a3523 = v1;
                let a3524 = v2;
                nat_le(a3523, a3524)
            } else {
                let a3525 = v0;
                let a3526 = 3u64;
                if nat_eq(a3525, a3526) {
                    let a3527 = v1;
                    let a3528 = v2;
                    nat_lt(a3527, a3528)
                } else {
                    let a3529 = v1;
                    let a3530 = v2;
                    nat_eq(a3529, a3530)
                }
            }
        }
    }
}

pub fn f157(v0: u64, v1: Adt3, v2: u64) -> R<Adt9> {
    let a3531 = v0;
    let a3532 = 0u64;
    if nat_eq(a3531, a3532) {
        Ok(f165(0u64))
    } else {
        let a3535 = {
            let a3533 = v0;
            let a3534 = 1u64;
            nat_add(a3533, a3534)?
        };
        let a3536 = f102(v1.clone(), 13u64, v2)?;
        if nat_eq(a3535, a3536) {
            Ok(Adt9::C4({
                let a3537 = v2;
                let a3538 = 0u64;
                if nat_eq(a3537, a3538) {
                    2u64
                } else {
                    7u64
                }
            }, 1u64, 0u64, 2u64, 0u64, 3u64, 0u64))
        } else {
            let v3: u64 = {
                let a3539 = v0;
                let a3540 = 1u64;
                nat_sub(a3539, a3540)
            };
            if f145(v1.clone(), v2) {
                let v4: u64 = f146(match v1.clone() {
                    Adt3::C0(h3541, _, _, _, _, _, _, _) => {
                        h3541
                    }
                }, false)?;
                let v5: u64 = {
                    let a3542 = v3;
                    let a3543 = v4;
                    let a3544 = 0u64;
                    nat_quot(a3542, a3543, a3544)
                };
                let v6: u64 = {
                    let a3545 = v3;
                    let a3546 = v4;
                    let a3547 = 0u64;
                    nat_rem(a3545, a3546, a3547)
                };
                let a3550 = {
                    let a3548 = v6;
                    let a3549 = 1u64;
                    nat_add(a3548, a3549)?
                };
                let a3551 = v4;
                if nat_eq(a3550, a3551) {
                    Ok(Adt9::C4({
                        let a3552 = v2;
                        let a3553 = 0u64;
                        if nat_eq(a3552, a3553) {
                            0u64
                        } else {
                            4u64
                        }
                    }, 1u64, 0u64, 1u64, 1u64, 1u64, 0u64))
                } else {
                    f158(v6, v5, v1, v2)
                }
            } else {
                f173({
                    let a3554 = v3;
                    let a3555 = 2u64;
                    let a3556 = 0u64;
                    nat_quot(a3554, a3555, a3556)
                }, {
                    let a3557 = v3;
                    let a3558 = 2u64;
                    let a3559 = 0u64;
                    nat_rem(a3557, a3558, a3559)
                }, v1, v2, 0u64)
            }
        }
    }
}

pub fn f158(v0: u64, v1: u64, v2: Adt3, v3: u64) -> R<Adt9> {
    let a3560 = v0;
    let a3561 = 0u64;
    if nat_eq(a3560, a3561) {
        Ok(f165(1u64))
    } else {
        f173({
            let a3570 = {
                let a3563 = v1;
                let a3564 = match v2.clone() {
                    Adt3::C0(h3562, _, _, _, _, _, _, _) => {
                        h3562
                    }
                };
                nat_mul(a3563, a3564)?
            };
            let a3571 = {
                let a3567 = {
                    let a3565 = v0;
                    let a3566 = 1u64;
                    nat_sub(a3565, a3566)
                };
                let a3568 = 2u64;
                let a3569 = 0u64;
                nat_quot(a3567, a3568, a3569)
            };
            nat_add(a3570, a3571)?
        }, {
            let a3574 = {
                let a3572 = v0;
                let a3573 = 1u64;
                nat_sub(a3572, a3573)
            };
            let a3575 = 2u64;
            let a3576 = 0u64;
            nat_rem(a3574, a3575, a3576)
        }, v2, v3, 1u64)
    }
}

pub fn f159(v0: u64, v1: u64) -> R<u64> {
    let a3577 = v0;
    let a3578 = f77(v1)?;
    let a3579 = 0u64;
    Ok(nat_rem(a3577, a3578, a3579))
}

pub fn f160(v0: u64, v1: u64) -> R<u64> {
    let a3580 = 4u64;
    let a3581 = f174(v0, v1)?;
    nat_mul(a3580, a3581)
}

pub fn f161(v0: u64, v1: u64) -> R<u64> {
    let a3582 = 4u64;
    let a3583 = f175(v0, v1)?;
    nat_mul(a3582, a3583)
}

pub fn f162(v0: u64, v1: u64) -> R<bool> {
    let a3589 = {
        let a3586 = {
            let a3584 = f159(v0, v1)?;
            let a3585 = 2u64;
            nat_sub(a3584, a3585)
        };
        let a3587 = 2u64;
        let a3588 = 0u64;
        nat_rem(a3586, a3587, a3588)
    };
    let a3590 = 0u64;
    Ok(nat_eq(a3589, a3590))
}

pub fn f163(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64, v7: u64, v8: u64, v9: u64, v10: u64, v11: u64, v12: u64) -> R<Option<u64>> {
    let m3591 = f90(v0);
    match m3591 {
        Adt14::C0 => {
            Ok(None::<u64>)
        }
        Adt14::C1(v13) => {
            if if if if if f176(v0) {
                let a3592 = v1;
                let a3593 = v7;
                nat_lt(a3592, a3593)
            } else {
                false
            } {
                f127(v8, v2)
            } else {
                false
            } {
                let a3594 = v3;
                let a3595 = f177(v4, v5, v6)?;
                nat_lt(a3594, a3595)
            } else {
                false
            } {
                let a3600 = {
                    let a3597 = v7;
                    let a3598 = match v13.clone() {
                        Adt15::C0(_, h3596, _) => {
                            h3596
                        }
                    };
                    let a3599 = 0u64;
                    nat_rem(a3597, a3598, a3599)
                };
                let a3601 = 0u64;
                nat_eq(a3600, a3601)
            } else {
                false
            } {
                Ok(Some({
                    let a3630 = {
                        let a3620 = {
                            let a3610 = {
                                let a3606 = {
                                    let a3603 = v1;
                                    let a3604 = match v13 {
                                        Adt15::C0(_, h3602, _) => {
                                            h3602
                                        }
                                    };
                                    let a3605 = 0u64;
                                    nat_quot(a3603, a3604, a3605)
                                };
                                let a3607 = v9;
                                nat_mul(a3606, a3607)?
                            };
                            let a3611 = {
                                let a3608 = v2;
                                let a3609 = v10;
                                nat_mul(a3608, a3609)?
                            };
                            nat_add(a3610, a3611)?
                        };
                        let a3621 = {
                            let a3618 = {
                                let a3615 = {
                                    let a3612 = v3;
                                    let a3613 = v4;
                                    let a3614 = 0u64;
                                    nat_quot(a3612, a3613, a3614)
                                };
                                let a3616 = v5;
                                let a3617 = 0u64;
                                nat_rem(a3615, a3616, a3617)
                            };
                            let a3619 = v11;
                            nat_mul(a3618, a3619)?
                        };
                        nat_add(a3620, a3621)?
                    };
                    let a3631 = {
                        let a3628 = {
                            let a3625 = {
                                let a3622 = v3;
                                let a3623 = v4;
                                let a3624 = 0u64;
                                nat_quot(a3622, a3623, a3624)
                            };
                            let a3626 = v5;
                            let a3627 = 0u64;
                            nat_quot(a3625, a3626, a3627)
                        };
                        let a3629 = v12;
                        nat_mul(a3628, a3629)?
                    };
                    nat_add(a3630, a3631)?
                }))
            } else {
                Ok(None::<u64>)
            }
        }
    }
}

pub fn f164(v0: u64) -> u64 {
    let a3632 = v0;
    let a3633 = 30u64;
    if nat_eq(a3632, a3633) {
        4u64
    } else {
        3u64
    }
}

pub fn f165(v0: u64) -> Adt9 {
    Adt9::C4(0u64, 2u64, 0u64, 2u64, 0u64, 1u64, v0)
}

pub fn f166(v0: u64) -> bool {
    let a3634 = 0u64;
    let a3635 = v0;
    if nat_lt(a3634, a3635) {
        let a3636 = v0;
        let a3637 = 2139095040u64;
        nat_le(a3636, a3637)
    } else {
        false
    }
}

pub fn f167(v0: u64) -> u64 {
    let a3638 = v0;
    let a3639 = 40u64;
    if nat_lt(a3638, a3639) {
        let a3640 = v0;
        let a3641 = 20u64;
        if nat_lt(a3640, a3641) {
            let a3642 = v0;
            let a3643 = 10u64;
            if nat_lt(a3642, a3643) {
                let a3644 = v0;
                let a3645 = 5u64;
                if nat_lt(a3644, a3645) {
                    let a3646 = v0;
                    let a3647 = 2u64;
                    if nat_lt(a3646, a3647) {
                        let a3648 = v0;
                        let a3649 = 1u64;
                        if nat_lt(a3648, a3649) {
                            1u64
                        } else {
                            2u64
                        }
                    } else {
                        let a3650 = v0;
                        let a3651 = 3u64;
                        if nat_lt(a3650, a3651) {
                            4u64
                        } else {
                            let a3652 = v0;
                            let a3653 = 4u64;
                            if nat_lt(a3652, a3653) {
                                8u64
                            } else {
                                16u64
                            }
                        }
                    }
                } else {
                    let a3654 = v0;
                    let a3655 = 7u64;
                    if nat_lt(a3654, a3655) {
                        let a3656 = v0;
                        let a3657 = 6u64;
                        if nat_lt(a3656, a3657) {
                            32u64
                        } else {
                            64u64
                        }
                    } else {
                        let a3658 = v0;
                        let a3659 = 8u64;
                        if nat_lt(a3658, a3659) {
                            128u64
                        } else {
                            let a3660 = v0;
                            let a3661 = 9u64;
                            if nat_lt(a3660, a3661) {
                                256u64
                            } else {
                                512u64
                            }
                        }
                    }
                }
            } else {
                let a3662 = v0;
                let a3663 = 15u64;
                if nat_lt(a3662, a3663) {
                    let a3664 = v0;
                    let a3665 = 12u64;
                    if nat_lt(a3664, a3665) {
                        let a3666 = v0;
                        let a3667 = 11u64;
                        if nat_lt(a3666, a3667) {
                            1024u64
                        } else {
                            2048u64
                        }
                    } else {
                        let a3668 = v0;
                        let a3669 = 13u64;
                        if nat_lt(a3668, a3669) {
                            4096u64
                        } else {
                            let a3670 = v0;
                            let a3671 = 14u64;
                            if nat_lt(a3670, a3671) {
                                8192u64
                            } else {
                                16384u64
                            }
                        }
                    }
                } else {
                    let a3672 = v0;
                    let a3673 = 17u64;
                    if nat_lt(a3672, a3673) {
                        let a3674 = v0;
                        let a3675 = 16u64;
                        if nat_lt(a3674, a3675) {
                            32768u64
                        } else {
                            65536u64
                        }
                    } else {
                        let a3676 = v0;
                        let a3677 = 18u64;
                        if nat_lt(a3676, a3677) {
                            131072u64
                        } else {
                            let a3678 = v0;
                            let a3679 = 19u64;
                            if nat_lt(a3678, a3679) {
                                262144u64
                            } else {
                                524288u64
                            }
                        }
                    }
                }
            }
        } else {
            let a3680 = v0;
            let a3681 = 30u64;
            if nat_lt(a3680, a3681) {
                let a3682 = v0;
                let a3683 = 25u64;
                if nat_lt(a3682, a3683) {
                    let a3684 = v0;
                    let a3685 = 22u64;
                    if nat_lt(a3684, a3685) {
                        let a3686 = v0;
                        let a3687 = 21u64;
                        if nat_lt(a3686, a3687) {
                            1048576u64
                        } else {
                            2097152u64
                        }
                    } else {
                        let a3688 = v0;
                        let a3689 = 23u64;
                        if nat_lt(a3688, a3689) {
                            4194304u64
                        } else {
                            let a3690 = v0;
                            let a3691 = 24u64;
                            if nat_lt(a3690, a3691) {
                                8388608u64
                            } else {
                                16777216u64
                            }
                        }
                    }
                } else {
                    let a3692 = v0;
                    let a3693 = 27u64;
                    if nat_lt(a3692, a3693) {
                        let a3694 = v0;
                        let a3695 = 26u64;
                        if nat_lt(a3694, a3695) {
                            33554432u64
                        } else {
                            67108864u64
                        }
                    } else {
                        let a3696 = v0;
                        let a3697 = 28u64;
                        if nat_lt(a3696, a3697) {
                            134217728u64
                        } else {
                            let a3698 = v0;
                            let a3699 = 29u64;
                            if nat_lt(a3698, a3699) {
                                268435456u64
                            } else {
                                536870912u64
                            }
                        }
                    }
                }
            } else {
                let a3700 = v0;
                let a3701 = 35u64;
                if nat_lt(a3700, a3701) {
                    let a3702 = v0;
                    let a3703 = 32u64;
                    if nat_lt(a3702, a3703) {
                        let a3704 = v0;
                        let a3705 = 31u64;
                        if nat_lt(a3704, a3705) {
                            1073741824u64
                        } else {
                            2147483648u64
                        }
                    } else {
                        let a3706 = v0;
                        let a3707 = 33u64;
                        if nat_lt(a3706, a3707) {
                            4294967296u64
                        } else {
                            let a3708 = v0;
                            let a3709 = 34u64;
                            if nat_lt(a3708, a3709) {
                                8589934592u64
                            } else {
                                17179869184u64
                            }
                        }
                    }
                } else {
                    let a3710 = v0;
                    let a3711 = 37u64;
                    if nat_lt(a3710, a3711) {
                        let a3712 = v0;
                        let a3713 = 36u64;
                        if nat_lt(a3712, a3713) {
                            34359738368u64
                        } else {
                            68719476736u64
                        }
                    } else {
                        let a3714 = v0;
                        let a3715 = 38u64;
                        if nat_lt(a3714, a3715) {
                            137438953472u64
                        } else {
                            let a3716 = v0;
                            let a3717 = 39u64;
                            if nat_lt(a3716, a3717) {
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

pub fn f168(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64) -> bool {
    if f142(v1, v2, v3, v4) {
        f178(v0)
    } else {
        f176(v0)
    }
}

pub fn f169(v0: u64) -> u64 {
    let a3718 = v0;
    let a3719 = 26u64;
    if nat_eq(a3718, a3719) {
        26u64
    } else {
        0u64
    }
}

pub fn f170(v0: Adt3, v1: u64, v2: u64, v3: u64, v4: u64, v5: bool) -> R<u64> {
    let a3740 = {
        let a3734 = {
            let a3728 = {
                let a3722 = v1;
                let a3723 = if v5 {
                    f75(match v0.clone() {
                        Adt3::C0(_, _, _, _, h3720, _, _, _) => {
                            h3720
                        }
                    })
                } else {
                    f74(match v0.clone() {
                        Adt3::C0(_, _, _, _, h3721, _, _, _) => {
                            h3721
                        }
                    })
                };
                nat_mul(a3722, a3723)?
            };
            let a3729 = {
                let a3726 = v2;
                let a3727 = if v5 {
                    f75(match v0.clone() {
                        Adt3::C0(_, _, _, _, _, h3724, _, _) => {
                            h3724
                        }
                    })
                } else {
                    f74(match v0.clone() {
                        Adt3::C0(_, _, _, _, _, h3725, _, _) => {
                            h3725
                        }
                    })
                };
                nat_mul(a3726, a3727)?
            };
            nat_add(a3728, a3729)?
        };
        let a3735 = {
            let a3732 = v3;
            let a3733 = if v5 {
                f75(match v0.clone() {
                    Adt3::C0(_, _, _, _, _, _, h3730, _) => {
                        h3730
                    }
                })
            } else {
                f74(match v0.clone() {
                    Adt3::C0(_, _, _, _, _, _, h3731, _) => {
                        h3731
                    }
                })
            };
            nat_mul(a3732, a3733)?
        };
        nat_add(a3734, a3735)?
    };
    let a3741 = {
        let a3738 = v4;
        let a3739 = if v5 {
            f75(match v0 {
                Adt3::C0(_, _, _, _, _, _, _, h3736) => {
                    h3736
                }
            })
        } else {
            f74(match v0 {
                Adt3::C0(_, _, _, _, _, _, _, h3737) => {
                    h3737
                }
            })
        };
        nat_mul(a3738, a3739)?
    };
    nat_add(a3740, a3741)
}

pub fn f171(v0: u64, v1: u64) -> Option<u64> {
    let a3742 = v0;
    let a3743 = 2u64;
    if nat_eq(a3742, a3743) {
        Some(0u64)
    } else {
        let a3744 = v0;
        let a3745 = 5u64;
        if nat_eq(a3744, a3745) {
            Some(1u64)
        } else {
            let a3746 = v0;
            let a3747 = 6u64;
            if nat_eq(a3746, a3747) {
                Some(2u64)
            } else {
                let a3748 = v0;
                let a3749 = 7u64;
                if nat_eq(a3748, a3749) {
                    Some(3u64)
                } else {
                    let a3750 = v0;
                    let a3751 = 9u64;
                    if nat_eq(a3750, a3751) {
                        Some(4u64)
                    } else {
                        let a3752 = v0;
                        let a3753 = 8u64;
                        if nat_eq(a3752, a3753) {
                            Some(5u64)
                        } else {
                            let a3754 = v0;
                            let a3755 = 89u64;
                            if nat_eq(a3754, a3755) {
                                let a3756 = v1;
                                let a3757 = 2u64;
                                if nat_eq(a3756, a3757) {
                                    Some(6u64)
                                } else {
                                    let a3758 = v1;
                                    let a3759 = 0u64;
                                    if nat_eq(a3758, a3759) {
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

pub fn f172(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64, v7: u64, v8: u64, v9: u64, v10: u64, v11: u64, v12: u64, v13: u64, v14: u64, v15: u64) -> R<Adt16> {
    let a3760 = v0;
    let a3761 = 30u64;
    if nat_eq(a3760, a3761) {
        let a3762 = v2;
        let a3763 = 0u64;
        if if if nat_eq(a3762, a3763) {
            true
        } else {
            let a3764 = v2;
            let a3765 = 1u64;
            nat_eq(a3764, a3765)
        } {
            let a3766 = v1;
            let a3767 = 0u64;
            if nat_eq(a3766, a3767) {
                true
            } else {
                f179(v1, v4)
            }
        } else {
            false
        } {
            f180(28u64, 0u64, 0u64, v3, v5, v4, v6, v7, v9, v8, v10, v11, v12, v13, v14, v15)
        } else {
            Ok(Adt16::C0)
        }
    } else {
        if if if f181(v1) {
            f181(v2)
        } else {
            false
        } {
            let a3768 = v1;
            let a3769 = 0u64;
            if if nat_eq(a3768, a3769) {
                true
            } else {
                let a3770 = v2;
                let a3771 = 0u64;
                nat_eq(a3770, a3771)
            } {
                true
            } else {
                let a3772 = v1;
                let a3773 = v2;
                nat_eq(a3772, a3773)
            }
        } else {
            false
        } {
            f180(v0, 0u64, 0u64, v3, v4, v5, v6, v7, v8, v9, v10, v11, v12, v13, v14, v15)
        } else {
            Ok(Adt16::C0)
        }
    }
}

pub fn f173(v0: u64, v1: u64, v2: Adt3, v3: u64, v4: u64) -> R<Adt9> {
    let a3774 = v1;
    let a3775 = 0u64;
    if nat_eq(a3774, a3775) {
        Ok(Adt9::C4({
            let a3776 = v3;
            let a3777 = 0u64;
            if nat_eq(a3776, a3777) {
                1u64
            } else {
                6u64
            }
        }, 0u64, f110(v0, v2)?, 2u64, 0u64, 1u64, 2u64))
    } else {
        Ok(Adt9::C4({
            let a3778 = v3;
            let a3779 = 0u64;
            if nat_eq(a3778, a3779) {
                0u64
            } else {
                4u64
            }
        }, 1u64, v4, 1u64, 2u64, 1u64, v4))
    }
}

pub fn f174(v0: u64, v1: u64) -> R<u64> {
    let a3780 = v0;
    let a3781 = f77(v1)?;
    let a3782 = 0u64;
    Ok(nat_quot(a3780, a3781, a3782))
}

pub fn f175(v0: u64, v1: u64) -> R<u64> {
    let v2: u64 = f174(v0, v1)?;
    let v3: u64 = {
        let a3785 = {
            let a3783 = f159(v0, v1)?;
            let a3784 = 2u64;
            nat_sub(a3783, a3784)
        };
        let a3786 = 2u64;
        let a3787 = 0u64;
        nat_quot(a3785, a3786, a3787)
    };
    let v4: u64 = {
        let a3788 = v2;
        let a3789 = v1;
        nat_mul(a3788, a3789)?
    };
    if f162(v0, v1)? {
        let a3790 = v4;
        let a3791 = v3;
        nat_add(a3790, a3791)
    } else {
        let a3803 = {
            let a3799 = v4;
            let a3800 = {
                let a3797 = {
                    let a3792 = v1;
                    let a3793 = 1u64;
                    nat_sub(a3792, a3793)
                };
                let a3798 = {
                    let a3794 = v2;
                    let a3795 = v1;
                    let a3796 = 0u64;
                    nat_rem(a3794, a3795, a3796)
                };
                nat_mul(a3797, a3798)?
            };
            nat_sub(a3799, a3800)
        };
        let a3804 = {
            let a3801 = v3;
            let a3802 = v1;
            nat_mul(a3801, a3802)?
        };
        nat_add(a3803, a3804)
    }
}

pub fn f176(v0: u64) -> bool {
    let a3805 = v0;
    let a3806 = 0u64;
    if if if if if if if if if if if if if if if if if if if if if if if if if if if if if nat_eq(a3805, a3806) {
        true
    } else {
        let a3807 = v0;
        let a3808 = 1u64;
        nat_eq(a3807, a3808)
    } {
        true
    } else {
        let a3809 = v0;
        let a3810 = 2u64;
        nat_eq(a3809, a3810)
    } {
        true
    } else {
        let a3811 = v0;
        let a3812 = 3u64;
        nat_eq(a3811, a3812)
    } {
        true
    } else {
        let a3813 = v0;
        let a3814 = 6u64;
        nat_eq(a3813, a3814)
    } {
        true
    } else {
        let a3815 = v0;
        let a3816 = 7u64;
        nat_eq(a3815, a3816)
    } {
        true
    } else {
        let a3817 = v0;
        let a3818 = 8u64;
        nat_eq(a3817, a3818)
    } {
        true
    } else {
        let a3819 = v0;
        let a3820 = 10u64;
        nat_eq(a3819, a3820)
    } {
        true
    } else {
        let a3821 = v0;
        let a3822 = 11u64;
        nat_eq(a3821, a3822)
    } {
        true
    } else {
        let a3823 = v0;
        let a3824 = 12u64;
        nat_eq(a3823, a3824)
    } {
        true
    } else {
        let a3825 = v0;
        let a3826 = 13u64;
        nat_eq(a3825, a3826)
    } {
        true
    } else {
        let a3827 = v0;
        let a3828 = 14u64;
        nat_eq(a3827, a3828)
    } {
        true
    } else {
        let a3829 = v0;
        let a3830 = 16u64;
        nat_eq(a3829, a3830)
    } {
        true
    } else {
        let a3831 = v0;
        let a3832 = 17u64;
        nat_eq(a3831, a3832)
    } {
        true
    } else {
        let a3833 = v0;
        let a3834 = 18u64;
        nat_eq(a3833, a3834)
    } {
        true
    } else {
        let a3835 = v0;
        let a3836 = 19u64;
        nat_eq(a3835, a3836)
    } {
        true
    } else {
        let a3837 = v0;
        let a3838 = 20u64;
        nat_eq(a3837, a3838)
    } {
        true
    } else {
        let a3839 = v0;
        let a3840 = 21u64;
        nat_eq(a3839, a3840)
    } {
        true
    } else {
        let a3841 = v0;
        let a3842 = 22u64;
        nat_eq(a3841, a3842)
    } {
        true
    } else {
        let a3843 = v0;
        let a3844 = 23u64;
        nat_eq(a3843, a3844)
    } {
        true
    } else {
        let a3845 = v0;
        let a3846 = 26u64;
        nat_eq(a3845, a3846)
    } {
        true
    } else {
        let a3847 = v0;
        let a3848 = 29u64;
        nat_eq(a3847, a3848)
    } {
        true
    } else {
        let a3849 = v0;
        let a3850 = 30u64;
        nat_eq(a3849, a3850)
    } {
        true
    } else {
        let a3851 = v0;
        let a3852 = 34u64;
        nat_eq(a3851, a3852)
    } {
        true
    } else {
        let a3853 = v0;
        let a3854 = 35u64;
        nat_eq(a3853, a3854)
    } {
        true
    } else {
        let a3855 = v0;
        let a3856 = 39u64;
        nat_eq(a3855, a3856)
    } {
        true
    } else {
        let a3857 = v0;
        let a3858 = 40u64;
        nat_eq(a3857, a3858)
    } {
        true
    } else {
        let a3859 = v0;
        let a3860 = 41u64;
        nat_eq(a3859, a3860)
    } {
        true
    } else {
        let a3861 = v0;
        let a3862 = 42u64;
        nat_eq(a3861, a3862)
    } {
        true
    } else {
        let a3863 = v0;
        let a3864 = 43u64;
        nat_eq(a3863, a3864)
    }
}

pub fn f177(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let a3865 = v0;
    let a3866 = 0u64;
    if if if nat_eq(a3865, a3866) {
        true
    } else {
        let a3867 = v1;
        let a3868 = 0u64;
        nat_eq(a3867, a3868)
    } {
        true
    } else {
        let a3869 = v2;
        let a3870 = 0u64;
        nat_eq(a3869, a3870)
    } {
        Ok(0u64)
    } else {
        let a3873 = {
            let a3871 = v0;
            let a3872 = v1;
            nat_mul(a3871, a3872)?
        };
        let a3874 = v2;
        nat_mul(a3873, a3874)
    }
}

pub fn f178(v0: u64) -> bool {
    let m3875 = f90(v0);
    match m3875 {
        Adt14::C0 => {
            false
        }
        Adt14::C1(_) => {
            true
        }
    }
}

pub fn f179(v0: u64, v1: u64) -> bool {
    let a3876 = v0;
    let a3877 = 1u64;
    if nat_eq(a3876, a3877) {
        true
    } else {
        let v2: u64 = f182(v0);
        let a3878 = 0u64;
        let a3879 = v2;
        if nat_lt(a3878, a3879) {
            let a3883 = {
                let a3880 = v1;
                let a3881 = v2;
                let a3882 = 1u64;
                nat_rem(a3880, a3881, a3882)
            };
            let a3884 = 0u64;
            nat_eq(a3883, a3884)
        } else {
            false
        }
    }
}

pub fn f180(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64, v7: u64, v8: u64, v9: u64, v10: u64, v11: u64, v12: u64, v13: u64, v14: u64, v15: u64) -> R<Adt16> {
    let a3885 = v0;
    let a3886 = 28u64;
    if if if if nat_eq(a3885, a3886) {
        let a3887 = v1;
        let a3888 = 0u64;
        nat_eq(a3887, a3888)
    } else {
        false
    } {
        let a3889 = v2;
        let a3890 = 0u64;
        nat_eq(a3889, a3890)
    } else {
        false
    } {
        let a3891 = v3;
        let a3892 = 0u64;
        nat_eq(a3891, a3892)
    } else {
        false
    } {
        let a3893 = v4;
        let a3894 = v8;
        if if if if if if if if nat_eq(a3893, a3894) {
            let a3895 = v5;
            let a3896 = v12;
            nat_eq(a3895, a3896)
        } else {
            false
        } {
            let a3897 = v9;
            let a3898 = v13;
            nat_eq(a3897, a3898)
        } else {
            false
        } {
            let a3899 = v10;
            let a3900 = v14;
            nat_eq(a3899, a3900)
        } else {
            false
        } {
            let a3901 = v11;
            let a3902 = v15;
            nat_eq(a3901, a3902)
        } else {
            false
        } {
            let a3903 = 0u64;
            let a3904 = v6;
            nat_lt(a3903, a3904)
        } else {
            false
        } {
            let a3905 = 0u64;
            let a3906 = v7;
            nat_lt(a3905, a3906)
        } else {
            false
        } {
            let a3910 = {
                let a3907 = v10;
                let a3908 = v6;
                let a3909 = 1u64;
                nat_rem(a3907, a3908, a3909)
            };
            let a3911 = 0u64;
            if nat_eq(a3910, a3911) {
                let a3915 = {
                    let a3912 = v11;
                    let a3913 = v7;
                    let a3914 = 1u64;
                    nat_rem(a3912, a3913, a3914)
                };
                let a3916 = 0u64;
                nat_eq(a3915, a3916)
            } else {
                false
            }
        } else {
            false
        } {
            Ok(Adt16::C2(v5, v4, v9, v10, v11, {
                let a3917 = v10;
                let a3918 = v6;
                let a3919 = 0u64;
                nat_quot(a3917, a3918, a3919)
            }, {
                let a3920 = v11;
                let a3921 = v7;
                let a3922 = 0u64;
                nat_quot(a3920, a3921, a3922)
            }, {
                let a3923 = v5;
                let a3924 = v4;
                nat_mul(a3923, a3924)?
            }, {
                let a3925 = v4;
                let a3926 = v9;
                nat_mul(a3925, a3926)?
            }, {
                let a3927 = v5;
                let a3928 = v9;
                nat_mul(a3927, a3928)?
            }))
        } else {
            Ok(Adt16::C1)
        }
    } else {
        Ok(Adt16::C0)
    }
}

pub fn f181(v0: u64) -> bool {
    let a3929 = v0;
    let a3930 = 0u64;
    if if nat_eq(a3929, a3930) {
        true
    } else {
        let a3931 = v0;
        let a3932 = 1u64;
        nat_eq(a3931, a3932)
    } {
        true
    } else {
        let a3933 = v0;
        let a3934 = 30u64;
        nat_eq(a3933, a3934)
    }
}

pub fn f182(v0: u64) -> u64 {
    if f176(v0) {
        let m3935 = f90(v0);
        match m3935 {
            Adt14::C0 => {
                0u64
            }
            Adt14::C1(v1) => {
                let a3937 = 1u64;
                let a3938 = match v1.clone() {
                    Adt15::C0(_, h3936, _) => {
                        h3936
                    }
                };
                if if nat_lt(a3937, a3938) {
                    true
                } else {
                    let a3939 = v0;
                    let a3940 = 43u64;
                    nat_eq(a3939, a3940)
                } {
                    match v1 {
                        Adt15::C0(_, h3941, _) => {
                            h3941
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
