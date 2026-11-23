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
                                    Ok(Adt9::C0(Adt10::C5(64u64)))
                                } else {
                                    let a19 = v1;
                                    let a20 = 24u64;
                                    if nat_eq(a19, a20) {
                                        Ok(Adt9::C0(f4(v2, v7, v6)?))
                                    } else {
                                        let a21 = v1;
                                        let a22 = 27u64;
                                        if nat_eq(a21, a22) {
                                            Ok(Adt9::C0(f5(v2, v7, v6)?))
                                        } else {
                                            let a23 = v1;
                                            let a24 = 3u64;
                                            if nat_eq(a23, a24) {
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
    let v2: u64 = f12(v1.clone(), 1u64)?;
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
        f13(match v0.clone() {
            Adt1::C0(h39, _, _, _, _, _, _, _, _, _, _, _) => {
                h39
            }
        })
    } else {
        false
    } {
        f13(match v0.clone() {
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
        let a50 = f14(v2)?;
        let a51 = match v1.clone() {
            Adt2::C0(_, _, _, _, _, _, _, _, _, h49, _) => {
                h49
            }
        };
        nat_le(a50, a51)
    } else {
        false
    } {
        Ok(Adt10::C4(f15(v2)?, 4u64, f16(v1.clone(), 1u64), f16(v1, 1u64), Adt4::C23(match v0.clone() {
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
            let v9: u64 = f17(v6.clone(), match v5.clone() {
                Adt2::C0(_, h72, _, _, _, _, _, _, _, _, _) => {
                    h72
                }
            })?;
            let v10: u64 = f17(v7.clone(), match v5.clone() {
                Adt2::C0(_, _, h73, _, _, _, _, _, _, _, _) => {
                    h73
                }
            })?;
            let v11: u64 = if v0 {
                f17(v8.clone(), match v5.clone() {
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
                f18(v2.clone(), v4.clone())?
            } else {
                false
            } {
                f18(v5.clone(), v4.clone())?
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
                }, f16(v2, 1u64), f16(v5, 1u64), f16(v4, 1u64), v6, v7, v8, if v0 {
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
                            if f18(v1.clone(), v4.clone())? {
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
                                f12(v1.clone(), 1u64)?
                            } else {
                                0u64
                            }
                        };
                        let v10: u64 = if v7 {
                            0u64
                        } else {
                            f21(v8, match v5.clone() {
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
        Adt2::C0(h229, _, _, _, _, _, _, _, _, _, _) => {
            h229
        }
    };
    let v4: u64 = f23(v3);
    let a231 = match v1.clone() {
        Adt2::C0(h230, _, _, _, _, _, _, _, _, _, _) => {
            h230
        }
    };
    let a232 = 0u64;
    if if if if if if if if nat_eq(a231, a232) {
        true
    } else {
        let a234 = match v1.clone() {
            Adt2::C0(h233, _, _, _, _, _, _, _, _, _, _) => {
                h233
            }
        };
        let a235 = 30u64;
        nat_eq(a234, a235)
    } {
        let a238 = match v1.clone() {
            Adt2::C0(h236, _, _, _, _, _, _, _, _, _, _) => {
                h236
            }
        };
        let a239 = match v2.clone() {
            Adt2::C0(h237, _, _, _, _, _, _, _, _, _, _) => {
                h237
            }
        };
        nat_eq(a238, a239)
    } else {
        false
    } {
        f19(v1.clone(), v2.clone())
    } else {
        false
    } {
        f24(match v0.clone() {
            Adt1::C0(h240, _, _, _, _, _, _, _, _, _, _, _) => {
                h240
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
            Adt2::C0(_, h241, _, _, _, _, _, _, _, _, _) => {
                h241
            }
        }, match v1.clone() {
            Adt2::C0(_, _, h242, _, _, _, _, _, _, _, _) => {
                h242
            }
        }, match v1.clone() {
            Adt2::C0(_, _, _, h243, _, _, _, _, _, _, _) => {
                h243
            }
        }, match v1.clone() {
            Adt2::C0(_, _, _, _, h244, _, _, _, _, _, _) => {
                h244
            }
        }) {
            0u64
        } else {
            let a250 = match v1.clone() {
                Adt2::C0(_, _, h245, _, _, _, _, _, _, _, _) => {
                    h245
                }
            };
            let a251 = {
                let a248 = match v1.clone() {
                    Adt2::C0(_, _, _, h246, _, _, _, _, _, _, _) => {
                        h246
                    }
                };
                let a249 = match v1.clone() {
                    Adt2::C0(_, _, _, _, h247, _, _, _, _, _, _) => {
                        h247
                    }
                };
                nat_mul(a248, a249)?
            };
            nat_mul(a250, a251)?
        };
        let v6: u64 = {
            let a252 = v5;
            let a253 = 0u64;
            if if nat_eq(a252, a253) {
                true
            } else {
                f28(v2.clone(), v4)?
            } {
                0u64
            } else {
                let a255 = match v2.clone() {
                    Adt2::C0(_, _, _, _, _, _, _, _, _, h254, _) => {
                        h254
                    }
                };
                let a256 = v4;
                let a257 = 0u64;
                nat_quot(a255, a256, a257)
            }
        };
        Ok(Adt10::C4(f29(v3, match v1.clone() {
            Adt2::C0(_, h258, _, _, _, _, _, _, _, _, _) => {
                h258
            }
        }, v5, v6)?, v4, f16(v1.clone(), 1u64), f16(v2, 1u64), Adt4::C26(v3, match v0 {
            Adt1::C0(h259, _, _, _, _, _, _, _, _, _, _, _) => {
                h259
            }
        }, v5, f30(match v1 {
            Adt2::C0(_, h260, _, _, _, _, _, _, _, _, _) => {
                h260
            }
        }, v5, v6)?)))
    } else {
        Ok(Adt10::C1)
    }
}

pub fn f5(v0: Adt1, v1: Adt2, v2: Adt2) -> R<Adt10> {
    let a262 = match v1.clone() {
        Adt2::C0(h261, _, _, _, _, _, _, _, _, _, _) => {
            h261
        }
    };
    let a263 = 0u64;
    if if if if if nat_eq(a262, a263) {
        let a265 = match v2.clone() {
            Adt2::C0(h264, _, _, _, _, _, _, _, _, _, _) => {
                h264
            }
        };
        let a266 = 0u64;
        nat_eq(a265, a266)
    } else {
        false
    } {
        f19(v1.clone(), v2.clone())
    } else {
        false
    } {
        f31(match v0.clone() {
            Adt1::C0(h267, _, _, _, _, _, _, _, _, _, _, _) => {
                h267
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
            let a273 = match v3.clone() {
                Adt3::C0(_, h268, _, _, _, _, _, _) => {
                    h268
                }
            };
            let a274 = {
                let a271 = match v3.clone() {
                    Adt3::C0(_, _, h269, _, _, _, _, _) => {
                        h269
                    }
                };
                let a272 = match v3.clone() {
                    Adt3::C0(_, _, _, h270, _, _, _, _) => {
                        h270
                    }
                };
                nat_mul(a271, a272)?
            };
            nat_mul(a273, a274)?
        };
        Ok(Adt10::C4(f32(match v3.clone() {
            Adt3::C0(h275, _, _, _, _, _, _, _) => {
                h275
            }
        }, v4)?, 4u64, v3, f16(v2, 1u64), Adt4::C24(match v0 {
            Adt1::C0(h276, _, _, _, _, _, _, _, _, _, _, _) => {
                h276
            }
        })))
    } else {
        Ok(Adt10::C1)
    }
}

pub fn f6(v0: Adt2, v1: Option<Adt2>, v2: Option<Adt2>, v3: Adt2) -> R<Adt10> {
    let m277 = v1;
    match m277 {
        None => {
            Ok(Adt10::C1)
        }
        Some(v4) => {
            let m278 = v2;
            match m278 {
                None => {
                    Ok(Adt10::C1)
                }
                Some(v5) => {
                    let a280 = match v0.clone() {
                        Adt2::C0(h279, _, _, _, _, _, _, _, _, _, _) => {
                            h279
                        }
                    };
                    let a281 = 0u64;
                    if if if if if if if if if if if if if if if if if nat_eq(a280, a281) {
                        let a283 = match v4.clone() {
                            Adt2::C0(h282, _, _, _, _, _, _, _, _, _, _) => {
                                h282
                            }
                        };
                        let a284 = 0u64;
                        nat_eq(a283, a284)
                    } else {
                        false
                    } {
                        let a286 = match v5.clone() {
                            Adt2::C0(h285, _, _, _, _, _, _, _, _, _, _) => {
                                h285
                            }
                        };
                        let a287 = 26u64;
                        nat_eq(a286, a287)
                    } else {
                        false
                    } {
                        let a289 = match v3.clone() {
                            Adt2::C0(h288, _, _, _, _, _, _, _, _, _, _) => {
                                h288
                            }
                        };
                        let a290 = 0u64;
                        nat_eq(a289, a290)
                    } else {
                        false
                    } {
                        f19(v0.clone(), v3.clone())
                    } else {
                        false
                    } {
                        let a293 = match v0.clone() {
                            Adt2::C0(_, h291, _, _, _, _, _, _, _, _, _) => {
                                h291
                            }
                        };
                        let a294 = match v4.clone() {
                            Adt2::C0(_, h292, _, _, _, _, _, _, _, _, _) => {
                                h292
                            }
                        };
                        nat_eq(a293, a294)
                    } else {
                        false
                    } {
                        let a297 = match v0.clone() {
                            Adt2::C0(_, _, h295, _, _, _, _, _, _, _, _) => {
                                h295
                            }
                        };
                        let a298 = match v5.clone() {
                            Adt2::C0(_, h296, _, _, _, _, _, _, _, _, _) => {
                                h296
                            }
                        };
                        nat_eq(a297, a298)
                    } else {
                        false
                    } {
                        let a301 = match v0.clone() {
                            Adt2::C0(_, _, _, h299, _, _, _, _, _, _, _) => {
                                h299
                            }
                        };
                        let a302 = match v5.clone() {
                            Adt2::C0(_, _, h300, _, _, _, _, _, _, _, _) => {
                                h300
                            }
                        };
                        nat_eq(a301, a302)
                    } else {
                        false
                    } {
                        let a304 = 0u64;
                        let a305 = match v5.clone() {
                            Adt2::C0(_, _, _, h303, _, _, _, _, _, _, _) => {
                                h303
                            }
                        };
                        nat_lt(a304, a305)
                    } else {
                        false
                    } {
                        let a307 = 0u64;
                        let a308 = match v5.clone() {
                            Adt2::C0(_, _, _, _, h306, _, _, _, _, _, _) => {
                                h306
                            }
                        };
                        nat_lt(a307, a308)
                    } else {
                        false
                    } {
                        let a310 = 4u64;
                        let a311 = match v0.clone() {
                            Adt2::C0(_, _, _, _, _, h309, _, _, _, _, _) => {
                                h309
                            }
                        };
                        nat_le(a310, a311)
                    } else {
                        false
                    } {
                        let a313 = 4u64;
                        let a314 = match v4.clone() {
                            Adt2::C0(_, _, _, _, _, h312, _, _, _, _, _) => {
                                h312
                            }
                        };
                        nat_le(a313, a314)
                    } else {
                        false
                    } {
                        let a316 = 4u64;
                        let a317 = match v5.clone() {
                            Adt2::C0(_, _, _, _, _, h315, _, _, _, _, _) => {
                                h315
                            }
                        };
                        nat_le(a316, a317)
                    } else {
                        false
                    } {
                        let a319 = 4u64;
                        let a320 = match v3.clone() {
                            Adt2::C0(_, _, _, _, _, h318, _, _, _, _, _) => {
                                h318
                            }
                        };
                        nat_le(a319, a320)
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
                            Adt2::C0(_, h321, _, _, _, _, _, _, _, _, _) => {
                                h321
                            }
                        }, match v0.clone() {
                            Adt2::C0(_, _, h322, _, _, _, _, _, _, _, _) => {
                                h322
                            }
                        }, match v0.clone() {
                            Adt2::C0(_, _, _, h323, _, _, _, _, _, _, _) => {
                                h323
                            }
                        }, match v0.clone() {
                            Adt2::C0(_, _, _, _, h324, _, _, _, _, _, _) => {
                                h324
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
        Adt1::C0(h325, _, _, _, _, _, _, _, _, _, _, _) => {
            h325
        }
    };
    Ok(Adt9::C0({
        let a326 = v0;
        let a327 = 57u64;
        if if nat_eq(a326, a327) {
            true
        } else {
            let a328 = v0;
            let a329 = 58u64;
            nat_eq(a328, a329)
        } {
            let m330 = v3;
            match m330 {
                None => {
                    f34({
                        let a331 = v0;
                        let a332 = 58u64;
                        nat_eq(a331, a332)
                    }, v1, v2, v4)?
                }
                Some(_) => {
                    Adt10::C1
                }
            }
        } else {
            let a333 = v0;
            let a334 = 49u64;
            if nat_eq(a333, a334) {
                f35(v1, v2, v3, v4)?
            } else {
                let a335 = v0;
                let a336 = 51u64;
                if nat_eq(a335, a336) {
                    f36(v1, v2, v3, v4)?
                } else {
                    let a337 = v0;
                    let a338 = 39u64;
                    if nat_eq(a337, a338) {
                        f37(v2, v3, v4)?
                    } else {
                        let a339 = v0;
                        let a340 = 77u64;
                        if nat_eq(a339, a340) {
                            f38(v2, v4)?
                        } else {
                            let a341 = v0;
                            let a342 = 40u64;
                            if nat_eq(a341, a342) {
                                f39(v2, v3, v4)?
                            } else {
                                if f40(v0) {
                                    f41(v0, v2, {
                                        let m343 = v3;
                                        match m343 {
                                            None => {
                                                false
                                            }
                                            Some(_) => {
                                                true
                                            }
                                        }
                                    }, v4)?
                                } else {
                                    let a344 = v0;
                                    let a345 = 19u64;
                                    if nat_eq(a344, a345) {
                                        f42(v2, v4)?
                                    } else {
                                        let a346 = v0;
                                        let a347 = 21u64;
                                        if nat_eq(a346, a347) {
                                            f43(v5, v2, v3, v4)?
                                        } else {
                                            let a348 = v0;
                                            let a349 = 61u64;
                                            if if if nat_eq(a348, a349) {
                                                true
                                            } else {
                                                let a350 = v0;
                                                let a351 = 62u64;
                                                nat_eq(a350, a351)
                                            } {
                                                true
                                            } else {
                                                let a352 = v0;
                                                let a353 = 63u64;
                                                nat_eq(a352, a353)
                                            } {
                                                f44(v0, v1, v2, v4)?
                                            } else {
                                                let a354 = v0;
                                                let a355 = 4u64;
                                                if if nat_eq(a354, a355) {
                                                    true
                                                } else {
                                                    let a356 = v0;
                                                    let a357 = 32u64;
                                                    nat_eq(a356, a357)
                                                } {
                                                    f45(v0, v1, v2, v3, v4)?
                                                } else {
                                                    let a358 = v0;
                                                    let a359 = 20u64;
                                                    if nat_eq(a358, a359) {
                                                        f46(v2, v4)?
                                                    } else {
                                                        let a360 = v0;
                                                        let a361 = 42u64;
                                                        if nat_eq(a360, a361) {
                                                            f47(4u64, v2, v4)?
                                                        } else {
                                                            let a362 = v0;
                                                            let a363 = 69u64;
                                                            if nat_eq(a362, a363) {
                                                                let a364 = v5;
                                                                let a365 = 4u64;
                                                                if nat_lt(a364, a365) {
                                                                    f47(v5, v2, v4)?
                                                                } else {
                                                                    Adt10::C1
                                                                }
                                                            } else {
                                                                let a366 = v0;
                                                                let a367 = 70u64;
                                                                if nat_eq(a366, a367) {
                                                                    f48(v5, v2, v4)?
                                                                } else {
                                                                    let a368 = v0;
                                                                    let a369 = 75u64;
                                                                    if if nat_eq(a368, a369) {
                                                                        true
                                                                    } else {
                                                                        let a370 = v0;
                                                                        let a371 = 76u64;
                                                                        nat_eq(a370, a371)
                                                                    } {
                                                                        f49({
                                                                            let a372 = v0;
                                                                            let a373 = 76u64;
                                                                            nat_eq(a372, a373)
                                                                        }, v1, v2, v4)?
                                                                    } else {
                                                                        let a374 = v0;
                                                                        let a375 = 31u64;
                                                                        if if if nat_eq(a374, a375) {
                                                                            true
                                                                        } else {
                                                                            let a376 = v0;
                                                                            let a377 = 47u64;
                                                                            nat_eq(a376, a377)
                                                                        } {
                                                                            true
                                                                        } else {
                                                                            let a378 = v0;
                                                                            let a379 = 68u64;
                                                                            nat_eq(a378, a379)
                                                                        } {
                                                                            f50(v0, v1, v2, v4)?
                                                                        } else {
                                                                            let a380 = v0;
                                                                            let a381 = 15u64;
                                                                            if nat_eq(a380, a381) {
                                                                                f51(v2, v4)?
                                                                            } else {
                                                                                let a382 = v0;
                                                                                let a383 = 17u64;
                                                                                if nat_eq(a382, a383) {
                                                                                    f52(v2, v4)?
                                                                                } else {
                                                                                    let a384 = v0;
                                                                                    let a385 = 18u64;
                                                                                    if nat_eq(a384, a385) {
                                                                                        f53(v2, v3, v4)?
                                                                                    } else {
                                                                                        let a386 = v0;
                                                                                        let a387 = 13u64;
                                                                                        if if if nat_eq(a386, a387) {
                                                                                            true
                                                                                        } else {
                                                                                            let a388 = v0;
                                                                                            let a389 = 14u64;
                                                                                            nat_eq(a388, a389)
                                                                                        } {
                                                                                            true
                                                                                        } else {
                                                                                            let a390 = v0;
                                                                                            let a391 = 16u64;
                                                                                            nat_eq(a390, a391)
                                                                                        } {
                                                                                            f54(v0, v2, v4)?
                                                                                        } else {
                                                                                            let m392 = f55(v0, v5)?;
                                                                                            match m392 {
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
    let a393 = v1;
    let a394 = 0u64;
    if nat_eq(a393, a394) {
        let a396 = v3;
        let a397 = match v0.clone() {
            Adt8::C0(_, _, _, h395, _, _, _, _, _, _, _, _, _, _, _) => {
                h395
            }
        };
        if nat_lt(a396, a397) {
            let v4: Adt3 = match v0.clone() {
                Adt8::C0(_, _, _, _, _, _, _, _, h398, _, _, _, _, _, _) => {
                    h398
                }
            };
            let v5: u64 = {
                let a400 = v3;
                let a401 = match v0.clone() {
                    Adt8::C0(_, _, h399, _, _, _, _, _, _, _, _, _, _, _, _) => {
                        h399
                    }
                };
                let a402 = 0u64;
                nat_quot(a400, a401, a402)
            };
            Ok(Adt9::C8(Some(f58(v4.clone(), f59(v5, v4.clone(), 0u64), f59(v5, v4.clone(), 1u64), f59(v5, v4, 2u64), f60(v0.clone(), v5, {
                let a404 = v3;
                let a405 = match v0 {
                    Adt8::C0(_, _, h403, _, _, _, _, _, _, _, _, _, _, _, _) => {
                        h403
                    }
                };
                let a406 = 0u64;
                nat_rem(a404, a405, a406)
            })?)?)))
        } else {
            Ok(Adt9::C0(Adt10::C1))
        }
    } else {
        let v6: Adt3 = match v0.clone() {
            Adt8::C0(_, _, _, _, _, _, _, _, _, _, h407, _, _, _, _) => {
                h407
            }
        };
        let v7: u64 = {
            let a408 = v1;
            let a409 = 1u64;
            if nat_eq(a408, a409) {
                match v0.clone() {
                    Adt8::C0(_, h410, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                        h410
                    }
                }
            } else {
                match v0.clone() {
                    Adt8::C0(_, _, h411, _, _, _, _, _, _, _, _, _, _, _, _) => {
                        h411
                    }
                }
            }
        };
        let v8: u64 = {
            let a415 = v2;
            let a416 = {
                let a412 = v3;
                let a413 = v7;
                let a414 = 0u64;
                nat_quot(a412, a413, a414)
            };
            nat_add(a415, a416)?
        };
        let v9: u64 = {
            let a417 = v3;
            let a418 = v7;
            let a419 = 0u64;
            nat_rem(a417, a418, a419)
        };
        let v10: u64 = {
            let a421 = v8;
            let a422 = match v6.clone() {
                Adt3::C0(h420, _, _, _, _, _, _, _) => {
                    h420
                }
            };
            let a423 = 0u64;
            nat_rem(a421, a422, a423)
        };
        let v11: u64 = {
            let a425 = v8;
            let a426 = match v6.clone() {
                Adt3::C0(h424, _, _, _, _, _, _, _) => {
                    h424
                }
            };
            let a427 = 0u64;
            nat_quot(a425, a426, a427)
        };
        let v12: u64 = {
            let a429 = v11;
            let a430 = match v6.clone() {
                Adt3::C0(_, h428, _, _, _, _, _, _) => {
                    h428
                }
            };
            let a431 = 0u64;
            nat_rem(a429, a430, a431)
        };
        let v13: u64 = {
            let a433 = v11;
            let a434 = match v6.clone() {
                Adt3::C0(_, h432, _, _, _, _, _, _) => {
                    h432
                }
            };
            let a435 = 0u64;
            nat_quot(a433, a434, a435)
        };
        let a437 = v8;
        let a438 = match v0.clone() {
            Adt8::C0(h436, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h436
            }
        };
        if nat_lt(a437, a438) {
            let a439 = v1;
            let a440 = 1u64;
            if nat_eq(a439, a440) {
                let v14: Adt3 = match v0.clone() {
                    Adt8::C0(_, _, _, _, _, _, _, _, h441, _, _, _, _, _, _) => {
                        h441
                    }
                };
                let v15: Adt3 = match v0.clone() {
                    Adt8::C0(_, _, _, _, _, _, _, _, _, h442, _, _, _, _, _) => {
                        h442
                    }
                };
                let m445 = f61(match v0.clone() {
                    Adt8::C0(_, _, _, _, _, _, _, _, _, _, _, h443, _, _, _) => {
                        h443
                    }
                }, match v15.clone() {
                    Adt3::C0(h444, _, _, _, _, _, _, _) => {
                        h444
                    }
                }, v10, f59(v9, v14.clone(), 0u64))?;
                match m445 {
                    None => {
                        Ok(Adt9::C8(None::<u64>))
                    }
                    Some(v16) => {
                        let m448 = f61(match v0.clone() {
                            Adt8::C0(_, _, _, _, _, _, _, _, _, _, _, _, h446, _, _) => {
                                h446
                            }
                        }, match v15 {
                            Adt3::C0(_, h447, _, _, _, _, _, _) => {
                                h447
                            }
                        }, v12, f59(v9, v14, 1u64))?;
                        match m448 {
                            None => {
                                Ok(Adt9::C8(None::<u64>))
                            }
                            Some(v17) => {
                                f62(v0, v9, v13, v16, v17)
                            }
                        }
                    }
                }
            } else {
                let a449 = v1;
                let a450 = 2u64;
                if nat_eq(a449, a450) {
                    Ok(Adt9::C8(Some({
                        let a452 = match v0.clone() {
                            Adt8::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, h451) => {
                                h451
                            }
                        };
                        let a453 = 0u64;
                        if nat_eq(a452, a453) {
                            f58(v6, v10, v12, v9, v13)?
                        } else {
                            f58(v6.clone(), v10, v12, {
                                let a455 = v13;
                                let a456 = match v6.clone() {
                                    Adt3::C0(_, _, h454, _, _, _, _, _) => {
                                        h454
                                    }
                                };
                                let a457 = 0u64;
                                nat_rem(a455, a456, a457)
                            }, {
                                let a465 = {
                                    let a463 = {
                                        let a459 = v13;
                                        let a460 = match v6 {
                                            Adt3::C0(_, _, h458, _, _, _, _, _) => {
                                                h458
                                            }
                                        };
                                        let a461 = 0u64;
                                        nat_quot(a459, a460, a461)
                                    };
                                    let a464 = match v0 {
                                        Adt8::C0(_, _, h462, _, _, _, _, _, _, _, _, _, _, _, _) => {
                                            h462
                                        }
                                    };
                                    nat_mul(a463, a464)?
                                };
                                let a466 = v9;
                                nat_add(a465, a466)?
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

pub fn f9(v0: u64, v1: Adt3, v2: Adt3, v3: Adt4, v4: u64) -> R<Adt9> {
    Ok(f63(v3.clone(), {
        let m467 = v3.clone();
        match m467 {
            Adt4::C0 => {
                f64(v0, v1, v2, v3)?
            }
            Adt4::C1 => {
                f64(v0, v1, v2, v3)?
            }
            Adt4::C2(_, _) => {
                f64(v0, v1, v2, v3)?
            }
            Adt4::C3(_, _, _, _, _) => {
                f64(v0, v1, v2, v3)?
            }
            Adt4::C4(_) => {
                f64(v0, v1, v2, v3)?
            }
            Adt4::C5(_, _, _, _) => {
                f64(v0, v1, v2, v3)?
            }
            Adt4::C6(_, _, _) => {
                f64(v0, v1, v2, v3)?
            }
            Adt4::C7(_) => {
                f64(v0, v1, v2, v3)?
            }
            Adt4::C8(_) => {
                f64(v0, v1, v2, v3)?
            }
            Adt4::C9(_, _) => {
                f64(v0, v1, v2, v3)?
            }
            Adt4::C10(_, _, _) => {
                f64(v0, v1, v2, v3)?
            }
            Adt4::C11(_, _) => {
                f64(v0, v1, v2, v3)?
            }
            Adt4::C12(_, _, _) => {
                f64(v0, v1, v2, v3)?
            }
            Adt4::C13(_, _, _, _) => {
                f64(v0, v1, v2, v3)?
            }
            Adt4::C14(_, _, _) => {
                f64(v0, v1, v2, v3)?
            }
            Adt4::C15 => {
                f64(v0, v1, v2, v3)?
            }
            Adt4::C16(_, _) => {
                f64(v0, v1, v2, v3)?
            }
            Adt4::C17 => {
                f65(v0, v1, v2, v4)?
            }
            Adt4::C18(_, _) => {
                f64(v0, v1, v2, v3)?
            }
            Adt4::C19(_, v44) => {
                f66(v0, v1, v2, v44, v4)?
            }
            Adt4::C20(v45, v46) => {
                f67(v45, v0, v1, v2, v46, v4)?
            }
            Adt4::C21 => {
                f64(v0, v1, v2, v3)?
            }
            Adt4::C22(v47, v48) => {
                f68(v0, v1, v2, v47, v48, v4)?
            }
            Adt4::C23(v59, v60, v61, v62) => {
                f72(v0, v62, v59, v60, v61, v4)?
            }
            Adt4::C24(_) => {
                Adt9::C0(Adt10::C5(65u64))
            }
            Adt4::C25(_, _, _) => {
                f64(v0, v1, v2, v3)?
            }
            Adt4::C26(v54, v55, v56, v57) => {
                let a468 = v54;
                let a469 = 64u64;
                if if if nat_eq(a468, a469) {
                    true
                } else {
                    let a470 = v54;
                    let a471 = 65u64;
                    nat_eq(a470, a471)
                } {
                    true
                } else {
                    let a472 = v54;
                    let a473 = 66u64;
                    nat_eq(a472, a473)
                } {
                    Adt9::C0(Adt10::C5(64u64))
                } else {
                    let a474 = v54;
                    let a475 = 0u64;
                    if if nat_eq(a474, a475) {
                        true
                    } else {
                        let a476 = v54;
                        let a477 = 30u64;
                        nat_eq(a476, a477)
                    } {
                        f71(v0, v1, v2, v54, v55, v56, v57, v4)?
                    } else {
                        Adt9::C0(Adt10::C1)
                    }
                }
            }
            Adt4::C27(v53) => {
                f70(v0, v1, v2, v53)?
            }
            Adt4::C28(v52) => {
                f69(v0, v1, v2, v52, v4)?
            }
        }
    }))
}

pub fn f10(v0: Adt4) -> R<Adt9> {
    let m478 = v0;
    match m478 {
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
                let a483 = {
                    let a479 = 0u64;
                    let a480 = v45;
                    if nat_lt(a479, a480) {
                        1u64
                    } else {
                        0u64
                    }
                };
                let a484 = {
                    let a481 = 0u64;
                    let a482 = v47;
                    if nat_lt(a481, a482) {
                        6u64
                    } else {
                        0u64
                    }
                };
                nat_add(a483, a484)?
            }, 0u64, 0u64, false))
        }
        Adt4::C26(v50, _, _, _) => {
            Ok(Adt9::C6({
                let a485 = v50;
                let a486 = 66u64;
                if nat_eq(a485, a486) {
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
    let a487 = v1;
    let a488 = 5u64;
    if if if if nat_lt(a487, a488) {
        let a489 = v5;
        let a490 = 5u64;
        nat_lt(a489, a490)
    } else {
        false
    } {
        let a491 = v7;
        let a492 = 5u64;
        nat_lt(a491, a492)
    } else {
        false
    } {
        let a493 = v9;
        let a494 = 2u64;
        if nat_lt(a493, a494) {
            true
        } else {
            let a495 = v9;
            let a496 = 3u64;
            nat_eq(a495, a496)
        }
    } else {
        false
    } {
        Adt9::C7(f73(v0, v1, v2, v3, v4), f74(v5, v6), f74(v7, v8), f75({
            let a497 = v9;
            let a498 = 0u64;
            if nat_eq(a497, a498) {
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
    let a510 = {
        let a504 = {
            let a500 = match v0.clone() {
                Adt2::C0(_, h499, _, _, _, _, _, _, _, _, _) => {
                    h499
                }
            };
            let a501 = v1;
            let a502 = 0u64;
            nat_quot(a500, a501, a502)
        };
        let a505 = match v0.clone() {
            Adt2::C0(_, _, h503, _, _, _, _, _, _, _, _) => {
                h503
            }
        };
        nat_mul(a504, a505)?
    };
    let a511 = {
        let a508 = match v0.clone() {
            Adt2::C0(_, _, _, h506, _, _, _, _, _, _, _) => {
                h506
            }
        };
        let a509 = match v0 {
            Adt2::C0(_, _, _, _, h507, _, _, _, _, _, _) => {
                h507
            }
        };
        nat_mul(a508, a509)?
    };
    nat_mul(a510, a511)
}

pub fn f13(v0: u64) -> bool {
    let a512 = v0;
    let a513 = 4294967296u64;
    if nat_lt(a512, a513) {
        let a517 = {
            let a514 = v0;
            let a515 = 2147483648u64;
            let a516 = 0u64;
            nat_rem(a514, a515, a516)
        };
        let a518 = 2139095040u64;
        nat_lt(a517, a518)
    } else {
        false
    }
}

pub fn f14(v0: u64) -> R<u64> {
    let a519 = 4u64;
    let a520 = v0;
    nat_mul(a519, a520)
}

pub fn f15(v0: u64) -> R<u64> {
    let a521 = v0;
    let a522 = 0u64;
    if nat_eq(a521, a522) {
        Ok(0u64)
    } else {
        let a525 = 4u64;
        let a526 = {
            let a523 = 3u64;
            let a524 = v0;
            nat_mul(a523, a524)?
        };
        nat_add(a525, a526)
    }
}

pub fn f16(v0: Adt2, v1: u64) -> Adt3 {
    Adt3::C0({
        let a528 = match v0.clone() {
            Adt2::C0(_, h527, _, _, _, _, _, _, _, _, _) => {
                h527
            }
        };
        let a529 = v1;
        let a530 = 0u64;
        nat_quot(a528, a529, a530)
    }, match v0.clone() {
        Adt2::C0(_, _, h531, _, _, _, _, _, _, _, _) => {
            h531
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, h532, _, _, _, _, _, _, _) => {
            h532
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, h533, _, _, _, _, _, _) => {
            h533
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, _, h534, _, _, _, _, _) => {
            h534
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, _, _, h535, _, _, _, _) => {
            h535
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, _, _, _, h536, _, _, _) => {
            h536
        }
    }, match v0 {
        Adt2::C0(_, _, _, _, _, _, _, _, h537, _, _) => {
            h537
        }
    })
}

pub fn f17(v0: Adt6, v1: u64) -> R<u64> {
    let a539 = match v0.clone() {
        Adt6::C0(h538, _, _, _) => {
            h538
        }
    };
    let a540 = 0u64;
    if nat_eq(a539, a540) {
        f76(Adt6::C0(1u64, match v0.clone() {
            Adt6::C0(_, h541, _, _) => {
                h541
            }
        }, match v0.clone() {
            Adt6::C0(_, _, h542, _) => {
                h542
            }
        }, match v0.clone() {
            Adt6::C0(_, _, _, h543) => {
                h543
            }
        }), {
            let a545 = v1;
            let a546 = match v0 {
                Adt6::C0(_, _, _, h544) => {
                    h544
                }
            };
            nat_add(a545, a546)?
        })
    } else {
        f76(v0, v1)
    }
}

pub fn f18(v0: Adt2, v1: Adt2) -> R<bool> {
    let a548 = match v0.clone() {
        Adt2::C0(_, _, _, _, _, _, _, _, _, h547, _) => {
            h547
        }
    };
    let a549 = 0u64;
    if nat_eq(a548, a549) {
        Ok(true)
    } else {
        let a551 = match v1.clone() {
            Adt2::C0(_, _, _, _, _, _, _, _, _, h550, _) => {
                h550
            }
        };
        let a552 = 0u64;
        if nat_eq(a551, a552) {
            Ok(true)
        } else {
            let a554 = match v0.clone() {
                Adt2::C0(_, _, _, _, _, _, _, _, _, _, h553) => {
                    h553
                }
            };
            let a555 = 0u64;
            if nat_eq(a554, a555) {
                Ok(true)
            } else {
                let a557 = match v1.clone() {
                    Adt2::C0(_, _, _, _, _, _, _, _, _, _, h556) => {
                        h556
                    }
                };
                let a558 = 0u64;
                if nat_eq(a557, a558) {
                    Ok(true)
                } else {
                    let a564 = {
                        let a561 = match v0.clone() {
                            Adt2::C0(_, _, _, _, _, _, _, _, _, _, h559) => {
                                h559
                            }
                        };
                        let a562 = match v0.clone() {
                            Adt2::C0(_, _, _, _, _, _, _, _, _, h560, _) => {
                                h560
                            }
                        };
                        nat_add(a561, a562)?
                    };
                    let a565 = match v1.clone() {
                        Adt2::C0(_, _, _, _, _, _, _, _, _, _, h563) => {
                            h563
                        }
                    };
                    if nat_le(a564, a565) {
                        Ok(true)
                    } else {
                        let a571 = {
                            let a568 = match v1.clone() {
                                Adt2::C0(_, _, _, _, _, _, _, _, _, _, h566) => {
                                    h566
                                }
                            };
                            let a569 = match v1 {
                                Adt2::C0(_, _, _, _, _, _, _, _, _, h567, _) => {
                                    h567
                                }
                            };
                            nat_add(a568, a569)?
                        };
                        let a572 = match v0 {
                            Adt2::C0(_, _, _, _, _, _, _, _, _, _, h570) => {
                                h570
                            }
                        };
                        Ok(nat_le(a571, a572))
                    }
                }
            }
        }
    }
}

pub fn f19(v0: Adt2, v1: Adt2) -> bool {
    let a575 = match v0.clone() {
        Adt2::C0(_, h573, _, _, _, _, _, _, _, _, _) => {
            h573
        }
    };
    let a576 = match v1.clone() {
        Adt2::C0(_, h574, _, _, _, _, _, _, _, _, _) => {
            h574
        }
    };
    if if if nat_eq(a575, a576) {
        let a579 = match v0.clone() {
            Adt2::C0(_, _, h577, _, _, _, _, _, _, _, _) => {
                h577
            }
        };
        let a580 = match v1.clone() {
            Adt2::C0(_, _, h578, _, _, _, _, _, _, _, _) => {
                h578
            }
        };
        nat_eq(a579, a580)
    } else {
        false
    } {
        let a583 = match v0.clone() {
            Adt2::C0(_, _, _, h581, _, _, _, _, _, _, _) => {
                h581
            }
        };
        let a584 = match v1.clone() {
            Adt2::C0(_, _, _, h582, _, _, _, _, _, _, _) => {
                h582
            }
        };
        nat_eq(a583, a584)
    } else {
        false
    } {
        let a587 = match v0 {
            Adt2::C0(_, _, _, _, h585, _, _, _, _, _, _) => {
                h585
            }
        };
        let a588 = match v1 {
            Adt2::C0(_, _, _, _, h586, _, _, _, _, _, _) => {
                h586
            }
        };
        nat_eq(a587, a588)
    } else {
        false
    }
}

pub fn f20(v0: Adt2) -> R<bool> {
    let a590 = match v0.clone() {
        Adt2::C0(_, h589, _, _, _, _, _, _, _, _, _) => {
            h589
        }
    };
    let a591 = 1u64;
    if if if if nat_eq(a590, a591) {
        true
    } else {
        let a593 = match v0.clone() {
            Adt2::C0(_, _, _, _, _, h592, _, _, _, _, _) => {
                h592
            }
        };
        let a594 = 4u64;
        nat_eq(a593, a594)
    } {
        let a596 = match v0.clone() {
            Adt2::C0(_, _, h595, _, _, _, _, _, _, _, _) => {
                h595
            }
        };
        let a597 = 1u64;
        if nat_eq(a596, a597) {
            true
        } else {
            let a602 = match v0.clone() {
                Adt2::C0(_, _, _, _, _, _, h598, _, _, _, _) => {
                    h598
                }
            };
            let a603 = {
                let a600 = match v0.clone() {
                    Adt2::C0(_, h599, _, _, _, _, _, _, _, _, _) => {
                        h599
                    }
                };
                let a601 = 4u64;
                nat_mul(a600, a601)?
            };
            nat_eq(a602, a603)
        }
    } else {
        false
    } {
        let a605 = match v0.clone() {
            Adt2::C0(_, _, _, h604, _, _, _, _, _, _, _) => {
                h604
            }
        };
        let a606 = 1u64;
        if nat_eq(a605, a606) {
            true
        } else {
            let a614 = match v0.clone() {
                Adt2::C0(_, _, _, _, _, _, _, h607, _, _, _) => {
                    h607
                }
            };
            let a615 = {
                let a612 = {
                    let a610 = match v0.clone() {
                        Adt2::C0(_, h608, _, _, _, _, _, _, _, _, _) => {
                            h608
                        }
                    };
                    let a611 = match v0.clone() {
                        Adt2::C0(_, _, h609, _, _, _, _, _, _, _, _) => {
                            h609
                        }
                    };
                    nat_mul(a610, a611)?
                };
                let a613 = 4u64;
                nat_mul(a612, a613)?
            };
            nat_eq(a614, a615)
        }
    } else {
        false
    } {
        let a617 = match v0.clone() {
            Adt2::C0(_, _, _, _, h616, _, _, _, _, _, _) => {
                h616
            }
        };
        let a618 = 1u64;
        if nat_eq(a617, a618) {
            Ok(true)
        } else {
            let a629 = match v0.clone() {
                Adt2::C0(_, _, _, _, _, _, _, _, h619, _, _) => {
                    h619
                }
            };
            let a630 = {
                let a627 = {
                    let a625 = {
                        let a622 = match v0.clone() {
                            Adt2::C0(_, h620, _, _, _, _, _, _, _, _, _) => {
                                h620
                            }
                        };
                        let a623 = match v0.clone() {
                            Adt2::C0(_, _, h621, _, _, _, _, _, _, _, _) => {
                                h621
                            }
                        };
                        nat_mul(a622, a623)?
                    };
                    let a626 = match v0 {
                        Adt2::C0(_, _, _, h624, _, _, _, _, _, _, _) => {
                            h624
                        }
                    };
                    nat_mul(a625, a626)?
                };
                let a628 = 4u64;
                nat_mul(a627, a628)?
            };
            Ok(nat_eq(a629, a630))
        }
    } else {
        Ok(false)
    }
}

pub fn f21(v0: u64, v1: u64, v2: u64, v3: u64) -> R<u64> {
    let a635 = {
        let a631 = v0;
        let a632 = v1;
        nat_mul(a631, a632)?
    };
    let a636 = {
        let a633 = v2;
        let a634 = f77(v3);
        nat_mul(a633, a634)?
    };
    nat_mul(a635, a636)
}

pub fn f22(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let a639 = v0;
    let a640 = {
        let a637 = f78(v1)?;
        let a638 = v2;
        nat_mul(a637, a638)?
    };
    nat_add(a639, a640)
}

pub fn f23(v0: u64) -> u64 {
    let a641 = v0;
    let a642 = 30u64;
    if nat_eq(a641, a642) {
        2u64
    } else {
        4u64
    }
}

pub fn f24(v0: u64) -> bool {
    let a643 = v0;
    let a644 = 2139095040u64;
    if nat_le(a643, a644) {
        true
    } else {
        let a645 = v0;
        let a646 = 2147483648u64;
        nat_eq(a645, a646)
    }
}

pub fn f25(v0: Adt2, v1: u64) -> bool {
    let a648 = match v0.clone() {
        Adt2::C0(_, _, _, _, _, h647, _, _, _, _, _) => {
            h647
        }
    };
    let a649 = v1;
    if if nat_eq(a648, a649) {
        let a654 = {
            let a651 = match v0.clone() {
                Adt2::C0(_, _, _, _, _, _, _, _, _, _, h650) => {
                    h650
                }
            };
            let a652 = v1;
            let a653 = 0u64;
            nat_rem(a651, a652, a653)
        };
        let a655 = 0u64;
        nat_eq(a654, a655)
    } else {
        false
    } {
        f79(match v0.clone() {
            Adt2::C0(_, _, h656, _, _, _, _, _, _, _, _) => {
                h656
            }
        }, match v0.clone() {
            Adt2::C0(_, _, _, h657, _, _, _, _, _, _, _) => {
                h657
            }
        }, match v0.clone() {
            Adt2::C0(_, _, _, _, h658, _, _, _, _, _, _) => {
                h658
            }
        }, match v0.clone() {
            Adt2::C0(_, _, _, _, _, _, h659, _, _, _, _) => {
                h659
            }
        }, match v0.clone() {
            Adt2::C0(_, _, _, _, _, _, _, h660, _, _, _) => {
                h660
            }
        }, match v0 {
            Adt2::C0(_, _, _, _, _, _, _, _, h661, _, _) => {
                h661
            }
        }, v1)
    } else {
        false
    }
}

pub fn f26(v0: Adt2, v1: Adt2) -> bool {
    if if if if if if f19(v0.clone(), v1.clone()) {
        let a664 = match v0.clone() {
            Adt2::C0(h662, _, _, _, _, _, _, _, _, _, _) => {
                h662
            }
        };
        let a665 = match v1.clone() {
            Adt2::C0(h663, _, _, _, _, _, _, _, _, _, _) => {
                h663
            }
        };
        nat_eq(a664, a665)
    } else {
        false
    } {
        let a668 = match v0.clone() {
            Adt2::C0(_, _, _, _, _, _, _, _, _, _, h666) => {
                h666
            }
        };
        let a669 = match v1.clone() {
            Adt2::C0(_, _, _, _, _, _, _, _, _, _, h667) => {
                h667
            }
        };
        nat_eq(a668, a669)
    } else {
        false
    } {
        let a672 = match v0.clone() {
            Adt2::C0(_, _, _, _, _, h670, _, _, _, _, _) => {
                h670
            }
        };
        let a673 = match v1.clone() {
            Adt2::C0(_, _, _, _, _, h671, _, _, _, _, _) => {
                h671
            }
        };
        nat_eq(a672, a673)
    } else {
        false
    } {
        let a676 = match v0.clone() {
            Adt2::C0(_, _, _, _, _, _, h674, _, _, _, _) => {
                h674
            }
        };
        let a677 = match v1.clone() {
            Adt2::C0(_, _, _, _, _, _, h675, _, _, _, _) => {
                h675
            }
        };
        nat_eq(a676, a677)
    } else {
        false
    } {
        let a680 = match v0.clone() {
            Adt2::C0(_, _, _, _, _, _, _, h678, _, _, _) => {
                h678
            }
        };
        let a681 = match v1.clone() {
            Adt2::C0(_, _, _, _, _, _, _, h679, _, _, _) => {
                h679
            }
        };
        nat_eq(a680, a681)
    } else {
        false
    } {
        let a684 = match v0 {
            Adt2::C0(_, _, _, _, _, _, _, _, h682, _, _) => {
                h682
            }
        };
        let a685 = match v1 {
            Adt2::C0(_, _, _, _, _, _, _, _, h683, _, _) => {
                h683
            }
        };
        nat_eq(a684, a685)
    } else {
        false
    }
}

pub fn f27(v0: u64, v1: u64, v2: u64, v3: u64) -> bool {
    let a686 = v0;
    let a687 = 0u64;
    if if if nat_eq(a686, a687) {
        true
    } else {
        let a688 = v1;
        let a689 = 0u64;
        nat_eq(a688, a689)
    } {
        true
    } else {
        let a690 = v2;
        let a691 = 0u64;
        nat_eq(a690, a691)
    } {
        true
    } else {
        let a692 = v3;
        let a693 = 0u64;
        nat_eq(a692, a693)
    }
}

pub fn f28(v0: Adt2, v1: u64) -> R<bool> {
    f80({
        let a695 = match v0.clone() {
            Adt2::C0(_, h694, _, _, _, _, _, _, _, _, _) => {
                h694
            }
        };
        let a696 = v1;
        nat_mul(a695, a696)?
    }, match v0.clone() {
        Adt2::C0(_, _, h697, _, _, _, _, _, _, _, _) => {
            h697
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, h698, _, _, _, _, _, _, _) => {
            h698
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, h699, _, _, _, _, _, _) => {
            h699
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, _, _, h700, _, _, _, _) => {
            h700
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, _, _, _, h701, _, _, _) => {
            h701
        }
    }, match v0 {
        Adt2::C0(_, _, _, _, _, _, _, _, h702, _, _) => {
            h702
        }
    })
}

pub fn f29(v0: u64, v1: u64, v2: u64, v3: u64) -> R<u64> {
    let a703 = v1;
    let a704 = 0u64;
    if if nat_eq(a703, a704) {
        true
    } else {
        let a705 = v2;
        let a706 = 0u64;
        nat_eq(a705, a706)
    } {
        Ok(0u64)
    } else {
        let a713 = {
            let a707 = f81(v1, v2, v3)?;
            let a708 = f82(v0, v1, v2)?;
            nat_add(a707, a708)?
        };
        let a714 = {
            let a711 = f83(v0);
            let a712 = {
                let a709 = v1;
                let a710 = v2;
                nat_mul(a709, a710)?
            };
            nat_mul(a711, a712)?
        };
        nat_add(a713, a714)
    }
}

pub fn f30(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let a715 = v0;
    let a716 = 0u64;
    if if nat_eq(a715, a716) {
        true
    } else {
        let a717 = v1;
        let a718 = 0u64;
        nat_eq(a717, a718)
    } {
        Ok(3u64)
    } else {
        let a721 = {
            let a719 = 3u64;
            let a720 = v1;
            nat_add(a719, a720)?
        };
        let a722 = v2;
        nat_add(a721, a722)
    }
}

pub fn f31(v0: u64) -> bool {
    let a723 = v0;
    let a724 = 2139095040u64;
    if nat_le(a723, a724) {
        true
    } else {
        let a725 = v0;
        let a726 = 2147483648u64;
        nat_eq(a725, a726)
    }
}

pub fn f32(v0: u64, v1: u64) -> R<u64> {
    let a727 = v0;
    let a728 = 0u64;
    if nat_eq(a727, a728) {
        Ok(0u64)
    } else {
        let a729 = v1;
        let a730 = f84(v0)?;
        nat_mul(a729, a730)
    }
}

pub fn f33(v0: u64, v1: u64, v2: u64, v3: u64) -> R<u64> {
    let a731 = v0;
    let a732 = 0u64;
    if if if if nat_eq(a731, a732) {
        true
    } else {
        let a733 = v1;
        let a734 = 0u64;
        nat_eq(a733, a734)
    } {
        true
    } else {
        let a735 = v2;
        let a736 = 0u64;
        nat_eq(a735, a736)
    } {
        true
    } else {
        let a737 = v3;
        let a738 = 0u64;
        nat_eq(a737, a738)
    } {
        Ok(0u64)
    } else {
        let a751 = {
            let a741 = 2u64;
            let a742 = {
                let a739 = v1;
                let a740 = v2;
                nat_mul(a739, a740)?
            };
            nat_mul(a741, a742)?
        };
        let a752 = {
            let a749 = {
                let a745 = {
                    let a743 = v1;
                    let a744 = v2;
                    nat_mul(a743, a744)?
                };
                let a746 = v3;
                nat_mul(a745, a746)?
            };
            let a750 = {
                let a747 = 1u64;
                let a748 = v0;
                nat_add(a747, a748)?
            };
            nat_mul(a749, a750)?
        };
        nat_add(a751, a752)
    }
}

pub fn f34(v0: bool, v1: Adt1, v2: Adt2, v3: Adt2) -> R<Adt10> {
    let v4: Adt6 = Adt6::C0(match v1.clone() {
        Adt1::C0(_, h753, _, _, _, _, _, _, _, _, _, _) => {
            h753
        }
    }, if v0 {
        match v1.clone() {
            Adt1::C0(_, _, _, h754, _, _, _, _, _, _, _, _) => {
                h754
            }
        }
    } else {
        match v1.clone() {
            Adt1::C0(_, _, h755, _, _, _, _, _, _, _, _, _) => {
                h755
            }
        }
    }, if v0 {
        match v1.clone() {
            Adt1::C0(_, _, _, _, _, h756, _, _, _, _, _, _) => {
                h756
            }
        }
    } else {
        match v1.clone() {
            Adt1::C0(_, _, _, h757, _, _, _, _, _, _, _, _) => {
                h757
            }
        }
    }, 1u64);
    let v5: Adt6 = if v0 {
        Adt6::C0(match v1.clone() {
            Adt1::C0(_, _, h758, _, _, _, _, _, _, _, _, _) => {
                h758
            }
        }, match v1.clone() {
            Adt1::C0(_, _, _, _, h759, _, _, _, _, _, _, _) => {
                h759
            }
        }, match v1.clone() {
            Adt1::C0(_, _, _, _, _, _, h760, _, _, _, _, _) => {
                h760
            }
        }, 1u64)
    } else {
        Adt6::C0(1u64, 1u64, 0u64, 1u64)
    };
    let a762 = match v1.clone() {
        Adt1::C0(h761, _, _, _, _, _, _, _, _, _, _, _) => {
            h761
        }
    };
    let a763 = 1u64;
    if if if if if if if if if if if nat_le(a762, a763) {
        let a765 = match v2.clone() {
            Adt2::C0(h764, _, _, _, _, _, _, _, _, _, _) => {
                h764
            }
        };
        let a766 = 1u64;
        nat_le(a765, a766)
    } else {
        false
    } {
        let a768 = match v3.clone() {
            Adt2::C0(h767, _, _, _, _, _, _, _, _, _, _) => {
                h767
            }
        };
        let a769 = 0u64;
        nat_eq(a768, a769)
    } else {
        false
    } {
        let a774 = match v2.clone() {
            Adt2::C0(_, _, _, _, _, h770, _, _, _, _, _) => {
                h770
            }
        };
        let a775 = {
            let a772 = match v2.clone() {
                Adt2::C0(h771, _, _, _, _, _, _, _, _, _, _) => {
                    h771
                }
            };
            let a773 = 0u64;
            if nat_eq(a772, a773) {
                4u64
            } else {
                2u64
            }
        };
        nat_eq(a774, a775)
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
        f85(v4.clone(), match v2.clone() {
            Adt2::C0(_, h776, _, _, _, _, _, _, _, _, _) => {
                h776
            }
        }, match v3.clone() {
            Adt2::C0(_, h777, _, _, _, _, _, _, _, _, _) => {
                h777
            }
        })?
    } else {
        false
    } {
        if v0 {
            f85(v5.clone(), match v2.clone() {
                Adt2::C0(_, _, h778, _, _, _, _, _, _, _, _) => {
                    h778
                }
            }, match v3.clone() {
                Adt2::C0(_, _, h779, _, _, _, _, _, _, _, _) => {
                    h779
                }
            })?
        } else {
            let a782 = match v2.clone() {
                Adt2::C0(_, _, h780, _, _, _, _, _, _, _, _) => {
                    h780
                }
            };
            let a783 = match v3.clone() {
                Adt2::C0(_, _, h781, _, _, _, _, _, _, _, _) => {
                    h781
                }
            };
            nat_eq(a782, a783)
        }
    } else {
        false
    } {
        let a786 = match v2.clone() {
            Adt2::C0(_, _, _, h784, _, _, _, _, _, _, _) => {
                h784
            }
        };
        let a787 = match v3.clone() {
            Adt2::C0(_, _, _, h785, _, _, _, _, _, _, _) => {
                h785
            }
        };
        nat_eq(a786, a787)
    } else {
        false
    } {
        let a790 = match v2.clone() {
            Adt2::C0(_, _, _, _, h788, _, _, _, _, _, _) => {
                h788
            }
        };
        let a791 = match v3.clone() {
            Adt2::C0(_, _, _, _, h789, _, _, _, _, _, _) => {
                h789
            }
        };
        nat_eq(a790, a791)
    } else {
        false
    } {
        let a796 = {
            let a794 = match v4.clone() {
                Adt6::C0(h792, _, _, _) => {
                    h792
                }
            };
            let a795 = match v5.clone() {
                Adt6::C0(h793, _, _, _) => {
                    h793
                }
            };
            nat_mul(a794, a795)?
        };
        let a797 = 2147483648u64;
        nat_lt(a796, a797)
    } else {
        false
    } {
        let v6: Adt7 = Adt7::C0(v4, v5, v0, {
            let a799 = match v1 {
                Adt1::C0(h798, _, _, _, _, _, _, _, _, _, _, _) => {
                    h798
                }
            };
            let a800 = 0u64;
            nat_eq(a799, a800)
        }, match v2.clone() {
            Adt2::C0(h801, _, _, _, _, _, _, _, _, _, _) => {
                h801
            }
        });
        Ok(Adt10::C4({
            let a802 = f12(v3.clone(), 1u64)?;
            let a803 = f86(v6.clone())?;
            nat_mul(a802, a803)?
        }, 4u64, f16(v2, 1u64), f16(v3, 1u64), Adt4::C28(v6)))
    } else {
        Ok(Adt10::C1)
    }
}

pub fn f35(v0: Adt1, v1: Adt2, v2: Option<Adt2>, v3: Adt2) -> R<Adt10> {
    let m804 = v2;
    match m804 {
        None => {
            Ok(Adt10::C1)
        }
        Some(v4) => {
            let v5: bool = {
                let a806 = match v0.clone() {
                    Adt1::C0(_, _, _, _, _, _, h805, _, _, _, _, _) => {
                        h805
                    }
                };
                let a807 = 1u64;
                nat_eq(a806, a807)
            };
            let v6: Adt6 = Adt6::C0(match v1.clone() {
                Adt2::C0(_, h808, _, _, _, _, _, _, _, _, _) => {
                    h808
                }
            }, match v0.clone() {
                Adt1::C0(h809, _, _, _, _, _, _, _, _, _, _, _) => {
                    h809
                }
            }, match v0.clone() {
                Adt1::C0(_, _, h810, _, _, _, _, _, _, _, _, _) => {
                    h810
                }
            }, match v0.clone() {
                Adt1::C0(_, _, _, _, h811, _, _, _, _, _, _, _) => {
                    h811
                }
            });
            let v7: Adt6 = if v5 {
                Adt6::C0(match v1.clone() {
                    Adt2::C0(_, _, h812, _, _, _, _, _, _, _, _) => {
                        h812
                    }
                }, match v0.clone() {
                    Adt1::C0(_, h813, _, _, _, _, _, _, _, _, _, _) => {
                        h813
                    }
                }, match v0.clone() {
                    Adt1::C0(_, _, _, h814, _, _, _, _, _, _, _, _) => {
                        h814
                    }
                }, match v0.clone() {
                    Adt1::C0(_, _, _, _, _, h815, _, _, _, _, _, _) => {
                        h815
                    }
                })
            } else {
                Adt6::C0(1u64, 1u64, 0u64, 1u64)
            };
            let v8: Adt6 = Adt6::C0(1u64, 1u64, 0u64, 1u64);
            let v9: u64 = if v5 {
                match v4.clone() {
                    Adt2::C0(_, _, _, h816, _, _, _, _, _, _, _) => {
                        h816
                    }
                }
            } else {
                match v4.clone() {
                    Adt2::C0(_, _, h817, _, _, _, _, _, _, _, _) => {
                        h817
                    }
                }
            };
            let v10: u64 = if v5 {
                match v4.clone() {
                    Adt2::C0(_, _, _, _, h818, _, _, _, _, _, _) => {
                        h818
                    }
                }
            } else {
                match v4.clone() {
                    Adt2::C0(_, _, _, h819, _, _, _, _, _, _, _) => {
                        h819
                    }
                }
            };
            let v11: u64 = if v5 {
                match v4.clone() {
                    Adt2::C0(_, _, h820, _, _, _, _, _, _, _, _) => {
                        h820
                    }
                }
            } else {
                1u64
            };
            let v12: u64 = match v4.clone() {
                Adt2::C0(h821, _, _, _, _, _, _, _, _, _, _) => {
                    h821
                }
            };
            let v13: u64 = match v3.clone() {
                Adt2::C0(h822, _, _, _, _, _, _, _, _, _, _) => {
                    h822
                }
            };
            let v14: u64 = {
                let a823 = v12;
                let a824 = 0u64;
                if nat_eq(a823, a824) {
                    4u64
                } else {
                    2u64
                }
            };
            let v15: u64 = {
                let a826 = match v4.clone() {
                    Adt2::C0(_, h825, _, _, _, _, _, _, _, _, _) => {
                        h825
                    }
                };
                let a827 = v14;
                nat_mul(a826, a827)?
            };
            let v16: u64 = if v5 {
                match v4.clone() {
                    Adt2::C0(_, _, _, _, _, _, _, h828, _, _, _) => {
                        h828
                    }
                }
            } else {
                match v4.clone() {
                    Adt2::C0(_, _, _, _, _, _, h829, _, _, _, _) => {
                        h829
                    }
                }
            };
            let v17: u64 = if v5 {
                match v4.clone() {
                    Adt2::C0(_, _, _, _, _, _, _, _, h830, _, _) => {
                        h830
                    }
                }
            } else {
                match v4.clone() {
                    Adt2::C0(_, _, _, _, _, _, _, h831, _, _, _) => {
                        h831
                    }
                }
            };
            let a833 = match v0 {
                Adt1::C0(_, _, _, _, _, _, h832, _, _, _, _, _) => {
                    h832
                }
            };
            let a834 = 1u64;
            if if if if if if if if if if nat_le(a833, a834) {
                let a835 = v12;
                let a836 = 0u64;
                if nat_eq(a835, a836) {
                    true
                } else {
                    let a837 = v12;
                    let a838 = 1u64;
                    if nat_eq(a837, a838) {
                        let a839 = v13;
                        let a840 = 1u64;
                        nat_eq(a839, a840)
                    } else {
                        false
                    }
                }
            } else {
                false
            } {
                let a841 = v13;
                let a842 = 0u64;
                if nat_eq(a841, a842) {
                    true
                } else {
                    let a843 = v13;
                    let a844 = 1u64;
                    nat_eq(a843, a844)
                }
            } else {
                false
            } {
                let a846 = match v4.clone() {
                    Adt2::C0(_, _, _, _, _, h845, _, _, _, _, _) => {
                        h845
                    }
                };
                let a847 = v14;
                nat_eq(a846, a847)
            } else {
                false
            } {
                let a855 = match v3.clone() {
                    Adt2::C0(_, h848, _, _, _, _, _, _, _, _, _) => {
                        h848
                    }
                };
                let a856 = {
                    let a853 = v9;
                    let a854 = {
                        let a851 = match v6.clone() {
                            Adt6::C0(h849, _, _, _) => {
                                h849
                            }
                        };
                        let a852 = match v7.clone() {
                            Adt6::C0(h850, _, _, _) => {
                                h850
                            }
                        };
                        nat_mul(a851, a852)?
                    };
                    nat_mul(a853, a854)?
                };
                nat_eq(a855, a856)
            } else {
                false
            } {
                let a859 = match v3.clone() {
                    Adt2::C0(_, _, h857, _, _, _, _, _, _, _, _) => {
                        h857
                    }
                };
                let a860 = f76(v6.clone(), match v4.clone() {
                    Adt2::C0(_, h858, _, _, _, _, _, _, _, _, _) => {
                        h858
                    }
                })?;
                nat_eq(a859, a860)
            } else {
                false
            } {
                let a862 = 0u64;
                let a863 = match v3.clone() {
                    Adt2::C0(_, _, h861, _, _, _, _, _, _, _, _) => {
                        h861
                    }
                };
                nat_lt(a862, a863)
            } else {
                false
            } {
                if v5 {
                    let a865 = match v1 {
                        Adt2::C0(_, _, _, h864, _, _, _, _, _, _, _) => {
                            h864
                        }
                    };
                    let a866 = v9;
                    if if if nat_eq(a865, a866) {
                        let a868 = match v3.clone() {
                            Adt2::C0(_, _, _, h867, _, _, _, _, _, _, _) => {
                                h867
                            }
                        };
                        let a869 = f76(v7.clone(), v11)?;
                        nat_eq(a868, a869)
                    } else {
                        false
                    } {
                        let a871 = match v3.clone() {
                            Adt2::C0(_, _, _, _, h870, _, _, _, _, _, _) => {
                                h870
                            }
                        };
                        let a872 = v10;
                        nat_eq(a871, a872)
                    } else {
                        false
                    } {
                        let a874 = 0u64;
                        let a875 = match v3.clone() {
                            Adt2::C0(_, _, _, h873, _, _, _, _, _, _, _) => {
                                h873
                            }
                        };
                        nat_lt(a874, a875)
                    } else {
                        false
                    }
                } else {
                    let a877 = match v1 {
                        Adt2::C0(_, _, h876, _, _, _, _, _, _, _, _) => {
                            h876
                        }
                    };
                    let a878 = v9;
                    if if if nat_eq(a877, a878) {
                        let a880 = match v4.clone() {
                            Adt2::C0(_, _, _, _, h879, _, _, _, _, _, _) => {
                                h879
                            }
                        };
                        let a881 = 1u64;
                        nat_eq(a880, a881)
                    } else {
                        false
                    } {
                        let a883 = match v3.clone() {
                            Adt2::C0(_, _, _, h882, _, _, _, _, _, _, _) => {
                                h882
                            }
                        };
                        let a884 = v10;
                        nat_eq(a883, a884)
                    } else {
                        false
                    } {
                        let a886 = match v3.clone() {
                            Adt2::C0(_, _, _, _, h885, _, _, _, _, _, _) => {
                                h885
                            }
                        };
                        let a887 = 1u64;
                        nat_eq(a886, a887)
                    } else {
                        false
                    }
                }
            } else {
                false
            } {
                if f27(match v4.clone() {
                    Adt2::C0(_, h888, _, _, _, _, _, _, _, _, _) => {
                        h888
                    }
                }, v11, v9, v10) {
                    true
                } else {
                    let a908 = {
                        let a905 = {
                            let a903 = {
                                let a897 = {
                                    let a891 = {
                                        let a889 = v10;
                                        let a890 = 1u64;
                                        nat_sub(a889, a890)
                                    };
                                    let a892 = v17;
                                    nat_mul(a891, a892)?
                                };
                                let a898 = {
                                    let a895 = {
                                        let a893 = v9;
                                        let a894 = 1u64;
                                        nat_sub(a893, a894)
                                    };
                                    let a896 = v16;
                                    nat_mul(a895, a896)?
                                };
                                nat_add(a897, a898)?
                            };
                            let a904 = {
                                let a901 = {
                                    let a899 = v11;
                                    let a900 = 1u64;
                                    nat_sub(a899, a900)
                                };
                                let a902 = v15;
                                nat_mul(a901, a902)?
                            };
                            nat_add(a903, a904)?
                        };
                        let a906 = v15;
                        nat_add(a905, a906)?
                    };
                    let a909 = match v4.clone() {
                        Adt2::C0(_, _, _, _, _, _, _, _, _, h907, _) => {
                            h907
                        }
                    };
                    nat_le(a908, a909)
                }
            } else {
                false
            } {
                f18(v4.clone(), v3.clone())?
            } else {
                false
            } {
                Ok(Adt10::C4(f12(v3.clone(), 1u64)?, {
                    let a910 = v13;
                    let a911 = 0u64;
                    if nat_eq(a910, a911) {
                        4u64
                    } else {
                        2u64
                    }
                }, Adt3::C0(match v4 {
                    Adt2::C0(_, h912, _, _, _, _, _, _, _, _, _) => {
                        h912
                    }
                }, v11, 1u64, v10, v14, v15, 0u64, 0u64), if v5 {
                    f16(v3, 1u64)
                } else {
                    Adt3::C0(match v3.clone() {
                        Adt2::C0(_, h913, _, _, _, _, _, _, _, _, _) => {
                            h913
                        }
                    }, match v3.clone() {
                        Adt2::C0(_, _, h914, _, _, _, _, _, _, _, _) => {
                            h914
                        }
                    }, 1u64, match v3.clone() {
                        Adt2::C0(_, _, _, h915, _, _, _, _, _, _, _) => {
                            h915
                        }
                    }, match v3.clone() {
                        Adt2::C0(_, _, _, _, _, h916, _, _, _, _, _) => {
                            h916
                        }
                    }, match v3.clone() {
                        Adt2::C0(_, _, _, _, _, _, h917, _, _, _, _) => {
                            h917
                        }
                    }, 0u64, match v3 {
                        Adt2::C0(_, _, _, _, _, _, _, h918, _, _, _) => {
                            h918
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
    let m919 = v2;
    match m919 {
        None => {
            Ok(Adt10::C1)
        }
        Some(v4) => {
            let v5: Adt6 = Adt6::C0(match v1.clone() {
                Adt2::C0(_, h920, _, _, _, _, _, _, _, _, _) => {
                    h920
                }
            }, match v0.clone() {
                Adt1::C0(h921, _, _, _, _, _, _, _, _, _, _, _) => {
                    h921
                }
            }, match v0.clone() {
                Adt1::C0(_, _, _, h922, _, _, _, _, _, _, _, _) => {
                    h922
                }
            }, match v0.clone() {
                Adt1::C0(_, _, _, _, _, _, h923, _, _, _, _, _) => {
                    h923
                }
            });
            let v6: Adt6 = Adt6::C0(match v1.clone() {
                Adt2::C0(_, _, h924, _, _, _, _, _, _, _, _) => {
                    h924
                }
            }, match v0.clone() {
                Adt1::C0(_, h925, _, _, _, _, _, _, _, _, _, _) => {
                    h925
                }
            }, match v0.clone() {
                Adt1::C0(_, _, _, _, h926, _, _, _, _, _, _, _) => {
                    h926
                }
            }, match v0.clone() {
                Adt1::C0(_, _, _, _, _, _, _, h927, _, _, _, _) => {
                    h927
                }
            });
            let v7: Adt6 = Adt6::C0(match v1 {
                Adt2::C0(_, _, _, h928, _, _, _, _, _, _, _) => {
                    h928
                }
            }, match v0.clone() {
                Adt1::C0(_, _, h929, _, _, _, _, _, _, _, _, _) => {
                    h929
                }
            }, match v0.clone() {
                Adt1::C0(_, _, _, _, _, h930, _, _, _, _, _, _) => {
                    h930
                }
            }, match v0.clone() {
                Adt1::C0(_, _, _, _, _, _, _, _, h931, _, _, _) => {
                    h931
                }
            });
            let v8: u64 = match v0 {
                Adt1::C0(_, _, _, _, _, _, _, _, _, h932, _, _) => {
                    h932
                }
            };
            let v9: u64 = f76(v7.clone(), match v4.clone() {
                Adt2::C0(_, _, _, h933, _, _, _, _, _, _, _) => {
                    h933
                }
            })?;
            let v10: u64 = match v3.clone() {
                Adt2::C0(h934, _, _, _, _, _, _, _, _, _, _) => {
                    h934
                }
            };
            let v11: u64 = {
                let a942 = v8;
                let a943 = {
                    let a940 = match v5.clone() {
                        Adt6::C0(h935, _, _, _) => {
                            h935
                        }
                    };
                    let a941 = {
                        let a938 = match v6.clone() {
                            Adt6::C0(h936, _, _, _) => {
                                h936
                            }
                        };
                        let a939 = match v7.clone() {
                            Adt6::C0(h937, _, _, _) => {
                                h937
                            }
                        };
                        nat_mul(a938, a939)?
                    };
                    nat_mul(a940, a941)?
                };
                nat_mul(a942, a943)?
            };
            let a945 = match v4.clone() {
                Adt2::C0(h944, _, _, _, _, _, _, _, _, _, _) => {
                    h944
                }
            };
            let a946 = 0u64;
            if if if if if if if if if if if if if nat_eq(a945, a946) {
                let a947 = v10;
                let a948 = 0u64;
                if nat_eq(a947, a948) {
                    true
                } else {
                    let a949 = v10;
                    let a950 = 1u64;
                    nat_eq(a949, a950)
                }
            } else {
                false
            } {
                let a951 = 0u64;
                let a952 = v8;
                nat_lt(a951, a952)
            } else {
                false
            } {
                let a953 = v8;
                let a954 = 2147483648u64;
                nat_lt(a953, a954)
            } else {
                false
            } {
                let a956 = match v4.clone() {
                    Adt2::C0(_, _, _, _, _, h955, _, _, _, _, _) => {
                        h955
                    }
                };
                let a957 = 4u64;
                nat_eq(a956, a957)
            } else {
                false
            } {
                let a958 = 0u64;
                let a959 = v9;
                nat_lt(a958, a959)
            } else {
                false
            } {
                let a961 = 0u64;
                let a962 = match v3.clone() {
                    Adt2::C0(_, _, h960, _, _, _, _, _, _, _, _) => {
                        h960
                    }
                };
                nat_lt(a961, a962)
            } else {
                false
            } {
                let a964 = 0u64;
                let a965 = match v3.clone() {
                    Adt2::C0(_, _, _, h963, _, _, _, _, _, _, _) => {
                        h963
                    }
                };
                nat_lt(a964, a965)
            } else {
                false
            } {
                let a967 = match v3.clone() {
                    Adt2::C0(_, h966, _, _, _, _, _, _, _, _, _) => {
                        h966
                    }
                };
                let a968 = v11;
                nat_eq(a967, a968)
            } else {
                false
            } {
                let a971 = match v3.clone() {
                    Adt2::C0(_, _, h969, _, _, _, _, _, _, _, _) => {
                        h969
                    }
                };
                let a972 = f76(v5.clone(), match v4.clone() {
                    Adt2::C0(_, h970, _, _, _, _, _, _, _, _, _) => {
                        h970
                    }
                })?;
                nat_eq(a971, a972)
            } else {
                false
            } {
                let a975 = match v3.clone() {
                    Adt2::C0(_, _, _, h973, _, _, _, _, _, _, _) => {
                        h973
                    }
                };
                let a976 = f76(v6.clone(), match v4.clone() {
                    Adt2::C0(_, _, h974, _, _, _, _, _, _, _, _) => {
                        h974
                    }
                })?;
                nat_eq(a975, a976)
            } else {
                false
            } {
                let a984 = match v3.clone() {
                    Adt2::C0(_, _, _, _, h977, _, _, _, _, _, _) => {
                        h977
                    }
                };
                let a985 = {
                    let a982 = v9;
                    let a983 = {
                        let a979 = match v4.clone() {
                            Adt2::C0(_, _, _, _, h978, _, _, _, _, _, _) => {
                                h978
                            }
                        };
                        let a980 = v8;
                        let a981 = 0u64;
                        nat_quot(a979, a980, a981)
                    };
                    nat_mul(a982, a983)?
                };
                nat_eq(a984, a985)
            } else {
                false
            } {
                f18(v4.clone(), v3.clone())?
            } else {
                false
            } {
                Ok(Adt10::C4(f12(v3.clone(), 1u64)?, {
                    let a986 = v10;
                    let a987 = 0u64;
                    if nat_eq(a986, a987) {
                        4u64
                    } else {
                        2u64
                    }
                }, f16(v4.clone(), 1u64), f16(v3, 1u64), Adt4::C27(Adt5::C0(v5, v6, v7, match v4.clone() {
                    Adt2::C0(_, _, _, _, _, _, _, _, h988, _, _) => {
                        h988
                    }
                }, {
                    let a990 = v8;
                    let a991 = match v4 {
                        Adt2::C0(_, _, _, _, _, _, _, _, h989, _, _) => {
                            h989
                        }
                    };
                    nat_mul(a990, a991)?
                }, 0u64, v9, v10))))
            } else {
                Ok(Adt10::C1)
            }
        }
    }
}

pub fn f37(v0: Adt2, v1: Option<Adt2>, v2: Adt2) -> R<Adt10> {
    let m992 = v1;
    match m992 {
        None => {
            Ok(Adt10::C1)
        }
        Some(v3) => {
            f87(v0, v3, v2)
        }
    }
}

pub fn f38(v0: Adt2, v1: Adt2) -> R<Adt10> {
    let a994 = match v0.clone() {
        Adt2::C0(h993, _, _, _, _, _, _, _, _, _, _) => {
            h993
        }
    };
    let a995 = 1u64;
    if if nat_eq(a994, a995) {
        true
    } else {
        let a997 = match v0.clone() {
            Adt2::C0(h996, _, _, _, _, _, _, _, _, _, _) => {
                h996
            }
        };
        let a998 = 30u64;
        nat_eq(a997, a998)
    } {
        let a1000 = match v1.clone() {
            Adt2::C0(h999, _, _, _, _, _, _, _, _, _, _) => {
                h999
            }
        };
        let a1001 = 1u64;
        if if if if if if if if if if if nat_eq(a1000, a1001) {
            let a1004 = match v0.clone() {
                Adt2::C0(_, h1002, _, _, _, _, _, _, _, _, _) => {
                    h1002
                }
            };
            let a1005 = match v1.clone() {
                Adt2::C0(_, h1003, _, _, _, _, _, _, _, _, _) => {
                    h1003
                }
            };
            nat_eq(a1004, a1005)
        } else {
            false
        } {
            let a1007 = 0u64;
            let a1008 = match v1.clone() {
                Adt2::C0(_, _, h1006, _, _, _, _, _, _, _, _) => {
                    h1006
                }
            };
            nat_lt(a1007, a1008)
        } else {
            false
        } {
            let a1011 = match v1.clone() {
                Adt2::C0(_, _, h1009, _, _, _, _, _, _, _, _) => {
                    h1009
                }
            };
            let a1012 = match v1.clone() {
                Adt2::C0(_, _, _, h1010, _, _, _, _, _, _, _) => {
                    h1010
                }
            };
            nat_eq(a1011, a1012)
        } else {
            false
        } {
            let a1014 = match v1.clone() {
                Adt2::C0(_, _, _, _, h1013, _, _, _, _, _, _) => {
                    h1013
                }
            };
            let a1015 = 1u64;
            nat_eq(a1014, a1015)
        } else {
            false
        } {
            let a1023 = match v0.clone() {
                Adt2::C0(_, _, h1016, _, _, _, _, _, _, _, _) => {
                    h1016
                }
            };
            let a1024 = {
                let a1021 = {
                    let a1019 = match v1.clone() {
                        Adt2::C0(_, _, h1017, _, _, _, _, _, _, _, _) => {
                            h1017
                        }
                    };
                    let a1020 = match v1.clone() {
                        Adt2::C0(_, _, h1018, _, _, _, _, _, _, _, _) => {
                            h1018
                        }
                    };
                    nat_add(a1019, a1020)?
                };
                let a1022 = 1u64;
                nat_sub(a1021, a1022)
            };
            nat_eq(a1023, a1024)
        } else {
            false
        } {
            let a1026 = 0u64;
            let a1027 = match v0.clone() {
                Adt2::C0(_, _, _, h1025, _, _, _, _, _, _, _) => {
                    h1025
                }
            };
            nat_lt(a1026, a1027)
        } else {
            false
        } {
            let a1029 = 0u64;
            let a1030 = match v0.clone() {
                Adt2::C0(_, _, _, _, h1028, _, _, _, _, _, _) => {
                    h1028
                }
            };
            nat_lt(a1029, a1030)
        } else {
            false
        } {
            let a1032 = 2u64;
            let a1033 = match v0.clone() {
                Adt2::C0(_, _, _, _, _, h1031, _, _, _, _, _) => {
                    h1031
                }
            };
            nat_le(a1032, a1033)
        } else {
            false
        } {
            let a1035 = 2u64;
            let a1036 = match v1.clone() {
                Adt2::C0(_, _, _, _, _, h1034, _, _, _, _, _) => {
                    h1034
                }
            };
            nat_le(a1035, a1036)
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
    let m1037 = v1;
    match m1037 {
        None => {
            Ok(Adt10::C1)
        }
        Some(v3) => {
            f88(v0, v3, v2)
        }
    }
}

pub fn f40(v0: u64) -> bool {
    let a1038 = v0;
    let a1039 = 1u64;
    if if nat_eq(a1038, a1039) {
        true
    } else {
        let a1040 = v0;
        let a1041 = 33u64;
        nat_eq(a1040, a1041)
    } {
        true
    } else {
        let a1042 = v0;
        let a1043 = 34u64;
        nat_eq(a1042, a1043)
    }
}

pub fn f41(v0: u64, v1: Adt2, v2: bool, v3: Adt2) -> R<Adt10> {
    let a1046 = match v1.clone() {
        Adt2::C0(h1044, _, _, _, _, _, _, _, _, _, _) => {
            h1044
        }
    };
    let a1047 = match v3.clone() {
        Adt2::C0(h1045, _, _, _, _, _, _, _, _, _, _) => {
            h1045
        }
    };
    if nat_eq(a1046, a1047) {
        f89(v0, v1, v2, v3)
    } else {
        if if if if f90(match v1.clone() {
            Adt2::C0(h1048, _, _, _, _, _, _, _, _, _, _) => {
                h1048
            }
        }) {
            f90(match v3.clone() {
                Adt2::C0(h1049, _, _, _, _, _, _, _, _, _, _) => {
                    h1049
                }
            })
        } else {
            false
        } {
            let a1050 = v0;
            let a1051 = 33u64;
            if nat_eq(a1050, a1051) {
                v2
            } else {
                true
            }
        } else {
            false
        } {
            let a1052 = f12(v1.clone(), 1u64)?;
            let a1053 = f12(v3.clone(), 1u64)?;
            nat_eq(a1052, a1053)
        } else {
            false
        } {
            let m1055 = f91(match v3.clone() {
                Adt2::C0(h1054, _, _, _, _, _, _, _, _, _, _) => {
                    h1054
                }
            });
            match m1055 {
                Adt14::C0 => {
                    Ok(Adt10::C0)
                }
                Adt14::C1(v4) => {
                    Ok(Adt10::C4(f12(v3.clone(), 1u64)?, match v4 {
                        Adt15::C0(_, _, h1056) => {
                            h1056
                        }
                    }, f16(v1.clone(), 1u64), f16(v3.clone(), 1u64), Adt4::C11(match v1 {
                        Adt2::C0(h1057, _, _, _, _, _, _, _, _, _, _) => {
                            h1057
                        }
                    }, match v3 {
                        Adt2::C0(h1058, _, _, _, _, _, _, _, _, _, _) => {
                            h1058
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
    let a1060 = match v0.clone() {
        Adt2::C0(h1059, _, _, _, _, _, _, _, _, _, _) => {
            h1059
        }
    };
    let a1061 = 0u64;
    if if if if nat_eq(a1060, a1061) {
        true
    } else {
        let a1063 = match v0.clone() {
            Adt2::C0(h1062, _, _, _, _, _, _, _, _, _, _) => {
                h1062
            }
        };
        let a1064 = 1u64;
        nat_eq(a1063, a1064)
    } {
        true
    } else {
        let a1066 = match v0.clone() {
            Adt2::C0(h1065, _, _, _, _, _, _, _, _, _, _) => {
                h1065
            }
        };
        let a1067 = 25u64;
        if nat_eq(a1066, a1067) {
            true
        } else {
            let a1069 = match v0.clone() {
                Adt2::C0(h1068, _, _, _, _, _, _, _, _, _, _) => {
                    h1068
                }
            };
            let a1070 = 26u64;
            nat_eq(a1069, a1070)
        }
    } {
        true
    } else {
        let a1072 = match v0.clone() {
            Adt2::C0(h1071, _, _, _, _, _, _, _, _, _, _) => {
                h1071
            }
        };
        let a1073 = 30u64;
        nat_eq(a1072, a1073)
    } {
        let a1076 = match v0.clone() {
            Adt2::C0(h1074, _, _, _, _, _, _, _, _, _, _) => {
                h1074
            }
        };
        let a1077 = match v1.clone() {
            Adt2::C0(h1075, _, _, _, _, _, _, _, _, _, _) => {
                h1075
            }
        };
        if if nat_eq(a1076, a1077) {
            f92(match v1.clone() {
                Adt2::C0(_, h1078, _, _, _, _, _, _, _, _, _) => {
                    h1078
                }
            }, match v1.clone() {
                Adt2::C0(_, _, h1079, _, _, _, _, _, _, _, _) => {
                    h1079
                }
            }, match v1.clone() {
                Adt2::C0(_, _, _, h1080, _, _, _, _, _, _, _) => {
                    h1080
                }
            }, match v1.clone() {
                Adt2::C0(_, _, _, _, h1081, _, _, _, _, _, _) => {
                    h1081
                }
            }, match v0.clone() {
                Adt2::C0(_, h1082, _, _, _, _, _, _, _, _, _) => {
                    h1082
                }
            }, match v0.clone() {
                Adt2::C0(_, _, h1083, _, _, _, _, _, _, _, _) => {
                    h1083
                }
            }, match v0.clone() {
                Adt2::C0(_, _, _, h1084, _, _, _, _, _, _, _) => {
                    h1084
                }
            }, match v0.clone() {
                Adt2::C0(_, _, _, _, h1085, _, _, _, _, _, _) => {
                    h1085
                }
            })
        } else {
            false
        } {
            let m1087 = f91(match v0.clone() {
                Adt2::C0(h1086, _, _, _, _, _, _, _, _, _, _) => {
                    h1086
                }
            });
            match m1087 {
                Adt14::C0 => {
                    Ok(Adt10::C0)
                }
                Adt14::C1(v2) => {
                    Ok(Adt10::C4(f12(v1.clone(), 1u64)?, match v2 {
                        Adt15::C0(_, _, h1088) => {
                            h1088
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
    let m1089 = v2;
    match m1089 {
        None => {
            Ok(Adt10::C1)
        }
        Some(v4) => {
            let a1090 = v0;
            let a1091 = 4u64;
            if if if nat_lt(a1090, a1091) {
                let a1094 = match v1.clone() {
                    Adt2::C0(h1092, _, _, _, _, _, _, _, _, _, _) => {
                        h1092
                    }
                };
                let a1095 = match v4.clone() {
                    Adt2::C0(h1093, _, _, _, _, _, _, _, _, _, _) => {
                        h1093
                    }
                };
                if nat_eq(a1094, a1095) {
                    let a1098 = match v1.clone() {
                        Adt2::C0(h1096, _, _, _, _, _, _, _, _, _, _) => {
                            h1096
                        }
                    };
                    let a1099 = match v3.clone() {
                        Adt2::C0(h1097, _, _, _, _, _, _, _, _, _, _) => {
                            h1097
                        }
                    };
                    nat_eq(a1098, a1099)
                } else {
                    false
                }
            } else {
                false
            } {
                if if if f93(v0, 0u64, match v1.clone() {
                    Adt2::C0(_, h1100, _, _, _, _, _, _, _, _, _) => {
                        h1100
                    }
                }, match v4.clone() {
                    Adt2::C0(_, h1101, _, _, _, _, _, _, _, _, _) => {
                        h1101
                    }
                }, match v3.clone() {
                    Adt2::C0(_, h1102, _, _, _, _, _, _, _, _, _) => {
                        h1102
                    }
                })? {
                    f93(v0, 1u64, match v1.clone() {
                        Adt2::C0(_, _, h1103, _, _, _, _, _, _, _, _) => {
                            h1103
                        }
                    }, match v4.clone() {
                        Adt2::C0(_, _, h1104, _, _, _, _, _, _, _, _) => {
                            h1104
                        }
                    }, match v3.clone() {
                        Adt2::C0(_, _, h1105, _, _, _, _, _, _, _, _) => {
                            h1105
                        }
                    })?
                } else {
                    false
                } {
                    f93(v0, 2u64, match v1.clone() {
                        Adt2::C0(_, _, _, h1106, _, _, _, _, _, _, _) => {
                            h1106
                        }
                    }, match v4.clone() {
                        Adt2::C0(_, _, _, h1107, _, _, _, _, _, _, _) => {
                            h1107
                        }
                    }, match v3.clone() {
                        Adt2::C0(_, _, _, h1108, _, _, _, _, _, _, _) => {
                            h1108
                        }
                    })?
                } else {
                    false
                } {
                    f93(v0, 3u64, match v1.clone() {
                        Adt2::C0(_, _, _, _, h1109, _, _, _, _, _, _) => {
                            h1109
                        }
                    }, match v4.clone() {
                        Adt2::C0(_, _, _, _, h1110, _, _, _, _, _, _) => {
                            h1110
                        }
                    }, match v3.clone() {
                        Adt2::C0(_, _, _, _, h1111, _, _, _, _, _, _) => {
                            h1111
                        }
                    })?
                } else {
                    false
                }
            } else {
                false
            } {
                let m1113 = f91(match v1.clone() {
                    Adt2::C0(h1112, _, _, _, _, _, _, _, _, _, _) => {
                        h1112
                    }
                });
                match m1113 {
                    Adt14::C0 => {
                        Ok(Adt10::C0)
                    }
                    Adt14::C1(v5) => {
                        let v6: u64 = match v5.clone() {
                            Adt15::C0(_, h1114, _) => {
                                h1114
                            }
                        };
                        let a1119 = {
                            let a1116 = match v1.clone() {
                                Adt2::C0(_, h1115, _, _, _, _, _, _, _, _, _) => {
                                    h1115
                                }
                            };
                            let a1117 = v6;
                            let a1118 = 1u64;
                            nat_rem(a1116, a1117, a1118)
                        };
                        let a1120 = 0u64;
                        if if if nat_eq(a1119, a1120) {
                            let a1125 = {
                                let a1122 = match v4.clone() {
                                    Adt2::C0(_, h1121, _, _, _, _, _, _, _, _, _) => {
                                        h1121
                                    }
                                };
                                let a1123 = v6;
                                let a1124 = 1u64;
                                nat_rem(a1122, a1123, a1124)
                            };
                            let a1126 = 0u64;
                            nat_eq(a1125, a1126)
                        } else {
                            false
                        } {
                            let a1131 = {
                                let a1128 = match v3.clone() {
                                    Adt2::C0(_, h1127, _, _, _, _, _, _, _, _, _) => {
                                        h1127
                                    }
                                };
                                let a1129 = v6;
                                let a1130 = 1u64;
                                nat_rem(a1128, a1129, a1130)
                            };
                            let a1132 = 0u64;
                            nat_eq(a1131, a1132)
                        } else {
                            false
                        } {
                            Ok(Adt10::C4(f12(v3.clone(), v6)?, match v5 {
                                Adt15::C0(_, _, h1133) => {
                                    h1133
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
    let a1135 = match v2.clone() {
        Adt2::C0(h1134, _, _, _, _, _, _, _, _, _, _) => {
            h1134
        }
    };
    let a1136 = 0u64;
    if if nat_eq(a1135, a1136) {
        let a1138 = match v3.clone() {
            Adt2::C0(h1137, _, _, _, _, _, _, _, _, _, _) => {
                h1137
            }
        };
        let a1139 = 0u64;
        nat_eq(a1138, a1139)
    } else {
        false
    } {
        let a1140 = v0;
        let a1141 = 61u64;
        if nat_eq(a1140, a1141) {
            let v4: bool = {
                let a1143 = 0u64;
                let a1144 = match v1.clone() {
                    Adt1::C0(_, _, _, _, _, _, _, _, h1142, _, _, _) => {
                        h1142
                    }
                };
                nat_lt(a1143, a1144)
            };
            if if if if if if f94(match v2.clone() {
                Adt2::C0(_, h1145, _, _, _, _, _, _, _, _, _) => {
                    h1145
                }
            }, match v1.clone() {
                Adt1::C0(h1146, _, _, _, _, _, _, _, _, _, _, _) => {
                    h1146
                }
            }, match v1.clone() {
                Adt1::C0(_, h1147, _, _, _, _, _, _, _, _, _, _) => {
                    h1147
                }
            }, match v3.clone() {
                Adt2::C0(_, h1148, _, _, _, _, _, _, _, _, _) => {
                    h1148
                }
            })? {
                f94(match v2.clone() {
                    Adt2::C0(_, _, h1149, _, _, _, _, _, _, _, _) => {
                        h1149
                    }
                }, match v1.clone() {
                    Adt1::C0(_, _, h1150, _, _, _, _, _, _, _, _, _) => {
                        h1150
                    }
                }, match v1.clone() {
                    Adt1::C0(_, _, _, h1151, _, _, _, _, _, _, _, _) => {
                        h1151
                    }
                }, match v3.clone() {
                    Adt2::C0(_, _, h1152, _, _, _, _, _, _, _, _) => {
                        h1152
                    }
                })?
            } else {
                false
            } {
                f94(match v2.clone() {
                    Adt2::C0(_, _, _, h1153, _, _, _, _, _, _, _) => {
                        h1153
                    }
                }, match v1.clone() {
                    Adt1::C0(_, _, _, _, h1154, _, _, _, _, _, _, _) => {
                        h1154
                    }
                }, match v1.clone() {
                    Adt1::C0(_, _, _, _, _, h1155, _, _, _, _, _, _) => {
                        h1155
                    }
                }, match v3.clone() {
                    Adt2::C0(_, _, _, h1156, _, _, _, _, _, _, _) => {
                        h1156
                    }
                })?
            } else {
                false
            } {
                f94(match v2.clone() {
                    Adt2::C0(_, _, _, _, h1157, _, _, _, _, _, _) => {
                        h1157
                    }
                }, match v1.clone() {
                    Adt1::C0(_, _, _, _, _, _, h1158, _, _, _, _, _) => {
                        h1158
                    }
                }, match v1.clone() {
                    Adt1::C0(_, _, _, _, _, _, _, h1159, _, _, _, _) => {
                        h1159
                    }
                }, match v3.clone() {
                    Adt2::C0(_, _, _, _, h1160, _, _, _, _, _, _) => {
                        h1160
                    }
                })?
            } else {
                false
            } {
                let a1162 = match v1.clone() {
                    Adt1::C0(_, _, _, _, _, _, _, _, h1161, _, _, _) => {
                        h1161
                    }
                };
                let a1163 = 4294967295u64;
                nat_le(a1162, a1163)
            } else {
                false
            } {
                if v4 {
                    !f27(match v2.clone() {
                        Adt2::C0(_, h1164, _, _, _, _, _, _, _, _, _) => {
                            h1164
                        }
                    }, match v2.clone() {
                        Adt2::C0(_, _, h1165, _, _, _, _, _, _, _, _) => {
                            h1165
                        }
                    }, match v2.clone() {
                        Adt2::C0(_, _, _, h1166, _, _, _, _, _, _, _) => {
                            h1166
                        }
                    }, match v2.clone() {
                        Adt2::C0(_, _, _, _, h1167, _, _, _, _, _, _) => {
                            h1167
                        }
                    })
                } else {
                    true
                }
            } else {
                false
            } {
                Ok(Adt10::C4(f12(v3.clone(), 1u64)?, 4u64, f16(v2, 1u64), f16(v3, 1u64), Adt4::C3(match v1.clone() {
                    Adt1::C0(h1168, _, _, _, _, _, _, _, _, _, _, _) => {
                        h1168
                    }
                }, match v1.clone() {
                    Adt1::C0(_, _, h1169, _, _, _, _, _, _, _, _, _) => {
                        h1169
                    }
                }, match v1.clone() {
                    Adt1::C0(_, _, _, _, h1170, _, _, _, _, _, _, _) => {
                        h1170
                    }
                }, match v1 {
                    Adt1::C0(_, _, _, _, _, _, h1171, _, _, _, _, _) => {
                        h1171
                    }
                }, v4)))
            } else {
                Ok(Adt10::C1)
            }
        } else {
            let a1172 = v0;
            let a1173 = 62u64;
            if nat_eq(a1172, a1173) {
                let a1175 = match v1.clone() {
                    Adt1::C0(h1174, _, _, _, _, _, _, _, _, _, _, _) => {
                        h1174
                    }
                };
                let a1176 = 2147483648u64;
                if if if if if nat_lt(a1175, a1176) {
                    let a1178 = match v1.clone() {
                        Adt1::C0(_, h1177, _, _, _, _, _, _, _, _, _, _) => {
                            h1177
                        }
                    };
                    let a1179 = 2147483648u64;
                    nat_lt(a1178, a1179)
                } else {
                    false
                } {
                    let a1182 = match v1.clone() {
                        Adt1::C0(h1180, _, _, _, _, _, _, _, _, _, _, _) => {
                            h1180
                        }
                    };
                    let a1183 = match v2.clone() {
                        Adt2::C0(_, h1181, _, _, _, _, _, _, _, _, _) => {
                            h1181
                        }
                    };
                    nat_lt(a1182, a1183)
                } else {
                    false
                } {
                    let a1186 = match v1.clone() {
                        Adt1::C0(_, h1184, _, _, _, _, _, _, _, _, _, _) => {
                            h1184
                        }
                    };
                    let a1187 = match v2.clone() {
                        Adt2::C0(_, h1185, _, _, _, _, _, _, _, _, _) => {
                            h1185
                        }
                    };
                    nat_lt(a1186, a1187)
                } else {
                    false
                } {
                    let a1190 = match v2.clone() {
                        Adt2::C0(_, _, h1188, _, _, _, _, _, _, _, _) => {
                            h1188
                        }
                    };
                    let a1191 = match v3.clone() {
                        Adt2::C0(_, _, h1189, _, _, _, _, _, _, _, _) => {
                            h1189
                        }
                    };
                    if if if nat_eq(a1190, a1191) {
                        let a1194 = match v2.clone() {
                            Adt2::C0(_, _, _, h1192, _, _, _, _, _, _, _) => {
                                h1192
                            }
                        };
                        let a1195 = match v3.clone() {
                            Adt2::C0(_, _, _, h1193, _, _, _, _, _, _, _) => {
                                h1193
                            }
                        };
                        nat_eq(a1194, a1195)
                    } else {
                        false
                    } {
                        let a1198 = match v2.clone() {
                            Adt2::C0(_, _, _, _, h1196, _, _, _, _, _, _) => {
                                h1196
                            }
                        };
                        let a1199 = match v3.clone() {
                            Adt2::C0(_, _, _, _, h1197, _, _, _, _, _, _) => {
                                h1197
                            }
                        };
                        nat_eq(a1198, a1199)
                    } else {
                        false
                    } {
                        let a1208 = match v3.clone() {
                            Adt2::C0(_, h1200, _, _, _, _, _, _, _, _, _) => {
                                h1200
                            }
                        };
                        let a1209 = {
                            let a1206 = {
                                let a1203 = match v2.clone() {
                                    Adt2::C0(_, h1201, _, _, _, _, _, _, _, _, _) => {
                                        h1201
                                    }
                                };
                                let a1204 = match v1.clone() {
                                    Adt1::C0(h1202, _, _, _, _, _, _, _, _, _, _, _) => {
                                        h1202
                                    }
                                };
                                nat_add(a1203, a1204)?
                            };
                            let a1207 = match v1.clone() {
                                Adt1::C0(_, h1205, _, _, _, _, _, _, _, _, _, _) => {
                                    h1205
                                }
                            };
                            nat_add(a1206, a1207)?
                        };
                        nat_eq(a1208, a1209)
                    } else {
                        false
                    }
                } else {
                    false
                } {
                    Ok(Adt10::C4(f12(v3.clone(), 1u64)?, 4u64, f16(v2, 1u64), f16(v3, 1u64), Adt4::C4(match v1 {
                        Adt1::C0(h1210, _, _, _, _, _, _, _, _, _, _, _) => {
                            h1210
                        }
                    })))
                } else {
                    Ok(Adt10::C1)
                }
            } else {
                let a1211 = v0;
                let a1212 = 63u64;
                if nat_eq(a1211, a1212) {
                    if if if if if f95(match v2.clone() {
                        Adt2::C0(_, h1213, _, _, _, _, _, _, _, _, _) => {
                            h1213
                        }
                    }, match v1.clone() {
                        Adt1::C0(h1214, _, _, _, _, _, _, _, _, _, _, _) => {
                            h1214
                        }
                    })? {
                        f95(match v2.clone() {
                            Adt2::C0(_, _, h1215, _, _, _, _, _, _, _, _) => {
                                h1215
                            }
                        }, match v1.clone() {
                            Adt1::C0(_, h1216, _, _, _, _, _, _, _, _, _, _) => {
                                h1216
                            }
                        })?
                    } else {
                        false
                    } {
                        f95(match v2.clone() {
                            Adt2::C0(_, _, _, h1217, _, _, _, _, _, _, _) => {
                                h1217
                            }
                        }, match v1.clone() {
                            Adt1::C0(_, _, h1218, _, _, _, _, _, _, _, _, _) => {
                                h1218
                            }
                        })?
                    } else {
                        false
                    } {
                        f95(match v2.clone() {
                            Adt2::C0(_, _, _, _, h1219, _, _, _, _, _, _) => {
                                h1219
                            }
                        }, match v1.clone() {
                            Adt1::C0(_, _, _, h1220, _, _, _, _, _, _, _, _) => {
                                h1220
                            }
                        })?
                    } else {
                        false
                    } {
                        let a1223 = match v2.clone() {
                            Adt2::C0(_, h1221, _, _, _, _, _, _, _, _, _) => {
                                h1221
                            }
                        };
                        let a1224 = match v3.clone() {
                            Adt2::C0(_, h1222, _, _, _, _, _, _, _, _, _) => {
                                h1222
                            }
                        };
                        if if if nat_eq(a1223, a1224) {
                            let a1227 = match v2.clone() {
                                Adt2::C0(_, _, h1225, _, _, _, _, _, _, _, _) => {
                                    h1225
                                }
                            };
                            let a1228 = match v3.clone() {
                                Adt2::C0(_, _, h1226, _, _, _, _, _, _, _, _) => {
                                    h1226
                                }
                            };
                            nat_eq(a1227, a1228)
                        } else {
                            false
                        } {
                            let a1231 = match v2.clone() {
                                Adt2::C0(_, _, _, h1229, _, _, _, _, _, _, _) => {
                                    h1229
                                }
                            };
                            let a1232 = match v3.clone() {
                                Adt2::C0(_, _, _, h1230, _, _, _, _, _, _, _) => {
                                    h1230
                                }
                            };
                            nat_eq(a1231, a1232)
                        } else {
                            false
                        } {
                            let a1235 = match v2.clone() {
                                Adt2::C0(_, _, _, _, h1233, _, _, _, _, _, _) => {
                                    h1233
                                }
                            };
                            let a1236 = match v3.clone() {
                                Adt2::C0(_, _, _, _, h1234, _, _, _, _, _, _) => {
                                    h1234
                                }
                            };
                            nat_eq(a1235, a1236)
                        } else {
                            false
                        }
                    } else {
                        false
                    } {
                        Ok(Adt10::C4(f12(v3.clone(), 1u64)?, 4u64, f16(v2, 1u64), f16(v3, 1u64), Adt4::C5(match v1.clone() {
                            Adt1::C0(h1237, _, _, _, _, _, _, _, _, _, _, _) => {
                                h1237
                            }
                        }, match v1.clone() {
                            Adt1::C0(_, h1238, _, _, _, _, _, _, _, _, _, _) => {
                                h1238
                            }
                        }, match v1.clone() {
                            Adt1::C0(_, _, h1239, _, _, _, _, _, _, _, _, _) => {
                                h1239
                            }
                        }, match v1 {
                            Adt1::C0(_, _, _, h1240, _, _, _, _, _, _, _, _) => {
                                h1240
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
    let m1241 = v3;
    match m1241 {
        None => {
            Ok(Adt10::C1)
        }
        Some(v5) => {
            let v6: bool = {
                let a1242 = v0;
                let a1243 = 4u64;
                nat_eq(a1242, a1243)
            };
            let v7: bool = {
                let a1245 = 0u64;
                let a1246 = match v1.clone() {
                    Adt1::C0(_, _, _, _, h1244, _, _, _, _, _, _, _) => {
                        h1244
                    }
                };
                nat_lt(a1245, a1246)
            };
            let v8: u64 = f12(v5.clone(), 1u64)?;
            let v9: Adt3 = Adt3::C0(match v5.clone() {
                Adt2::C0(_, h1247, _, _, _, _, _, _, _, _, _) => {
                    h1247
                }
            }, match v5.clone() {
                Adt2::C0(_, _, h1248, _, _, _, _, _, _, _, _) => {
                    h1248
                }
            }, match v5.clone() {
                Adt2::C0(_, _, _, h1249, _, _, _, _, _, _, _) => {
                    h1249
                }
            }, match v5.clone() {
                Adt2::C0(_, _, _, _, h1250, _, _, _, _, _, _) => {
                    h1250
                }
            }, 4u64, match v1.clone() {
                Adt1::C0(h1251, _, _, _, _, _, _, _, _, _, _, _) => {
                    h1251
                }
            }, match v1.clone() {
                Adt1::C0(_, h1252, _, _, _, _, _, _, _, _, _, _) => {
                    h1252
                }
            }, match v1.clone() {
                Adt1::C0(_, _, h1253, _, _, _, _, _, _, _, _, _) => {
                    h1253
                }
            });
            if if if if if if if if if if if if v6 {
                let a1255 = match v2.clone() {
                    Adt2::C0(h1254, _, _, _, _, _, _, _, _, _, _) => {
                        h1254
                    }
                };
                let a1256 = 0u64;
                if nat_eq(a1255, a1256) {
                    let a1258 = match v5.clone() {
                        Adt2::C0(h1257, _, _, _, _, _, _, _, _, _, _) => {
                            h1257
                        }
                    };
                    let a1259 = 0u64;
                    nat_eq(a1258, a1259)
                } else {
                    false
                }
            } else {
                let a1261 = match v2.clone() {
                    Adt2::C0(h1260, _, _, _, _, _, _, _, _, _, _) => {
                        h1260
                    }
                };
                let a1262 = 0u64;
                if if nat_eq(a1261, a1262) {
                    true
                } else {
                    let a1264 = match v2.clone() {
                        Adt2::C0(h1263, _, _, _, _, _, _, _, _, _, _) => {
                            h1263
                        }
                    };
                    let a1265 = 26u64;
                    nat_eq(a1264, a1265)
                } {
                    let a1267 = match v5.clone() {
                        Adt2::C0(h1266, _, _, _, _, _, _, _, _, _, _) => {
                            h1266
                        }
                    };
                    let a1268 = 0u64;
                    if nat_eq(a1267, a1268) {
                        true
                    } else {
                        let a1270 = match v5.clone() {
                            Adt2::C0(h1269, _, _, _, _, _, _, _, _, _, _) => {
                                h1269
                            }
                        };
                        let a1271 = 26u64;
                        nat_eq(a1270, a1271)
                    }
                } else {
                    false
                }
            } {
                let a1274 = match v2.clone() {
                    Adt2::C0(h1272, _, _, _, _, _, _, _, _, _, _) => {
                        h1272
                    }
                };
                let a1275 = match v4.clone() {
                    Adt2::C0(h1273, _, _, _, _, _, _, _, _, _, _) => {
                        h1273
                    }
                };
                nat_eq(a1274, a1275)
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
                let a1277 = match v5.clone() {
                    Adt2::C0(_, _, _, _, _, h1276, _, _, _, _, _) => {
                        h1276
                    }
                };
                let a1278 = 4u64;
                nat_eq(a1277, a1278)
            } else {
                false
            } {
                let a1279 = v8;
                let a1280 = f12(v2.clone(), 1u64)?;
                nat_le(a1279, a1280)
            } else {
                false
            } {
                let a1282 = match v1.clone() {
                    Adt1::C0(_, _, _, h1281, _, _, _, _, _, _, _, _) => {
                        h1281
                    }
                };
                let a1283 = 1073741824u64;
                nat_lt(a1282, a1283)
            } else {
                false
            } {
                let a1285 = match v1.clone() {
                    Adt1::C0(h1284, _, _, _, _, _, _, _, _, _, _, _) => {
                        h1284
                    }
                };
                let a1286 = 4294967295u64;
                if if if if nat_le(a1285, a1286) {
                    let a1288 = match v1.clone() {
                        Adt1::C0(_, h1287, _, _, _, _, _, _, _, _, _, _) => {
                            h1287
                        }
                    };
                    let a1289 = 4294967295u64;
                    nat_le(a1288, a1289)
                } else {
                    false
                } {
                    let a1291 = match v1.clone() {
                        Adt1::C0(_, _, h1290, _, _, _, _, _, _, _, _, _) => {
                            h1290
                        }
                    };
                    let a1292 = 4294967295u64;
                    nat_le(a1291, a1292)
                } else {
                    false
                } {
                    let a1294 = match v1.clone() {
                        Adt1::C0(_, _, _, h1293, _, _, _, _, _, _, _, _) => {
                            h1293
                        }
                    };
                    let a1295 = 4294967295u64;
                    nat_le(a1294, a1295)
                } else {
                    false
                } {
                    let a1297 = match v1.clone() {
                        Adt1::C0(_, _, _, _, h1296, _, _, _, _, _, _, _) => {
                            h1296
                        }
                    };
                    let a1298 = 4294967295u64;
                    nat_le(a1297, a1298)
                } else {
                    false
                }
            } else {
                false
            } {
                if v7 {
                    let a1301 = match v2.clone() {
                        Adt2::C0(_, _, _, _, _, _, _, _, _, _, h1299) => {
                            h1299
                        }
                    };
                    let a1302 = match v4.clone() {
                        Adt2::C0(_, _, _, _, _, _, _, _, _, _, h1300) => {
                            h1300
                        }
                    };
                    nat_eq(a1301, a1302)
                } else {
                    true
                }
            } else {
                false
            } {
                f96(v9.clone(), match v1.clone() {
                    Adt1::C0(_, _, _, h1303, _, _, _, _, _, _, _, _) => {
                        h1303
                    }
                }, match v4 {
                    Adt2::C0(_, _, _, _, _, _, _, _, _, h1304, _) => {
                        h1304
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
                    let a1305 = v10;
                    let a1306 = v8;
                    nat_add(a1305, a1306)?
                }, 4u64, f16(v5, 1u64), v9, Adt4::C6(v10, match v1 {
                    Adt1::C0(_, _, _, h1307, _, _, _, _, _, _, _, _) => {
                        h1307
                    }
                }, v6)))
            } else {
                Ok(Adt10::C1)
            }
        }
    }
}

pub fn f46(v0: Adt2, v1: Adt2) -> R<Adt10> {
    let a1309 = match v0.clone() {
        Adt2::C0(h1308, _, _, _, _, _, _, _, _, _, _) => {
            h1308
        }
    };
    let a1310 = 0u64;
    if if if nat_eq(a1309, a1310) {
        let a1312 = match v1.clone() {
            Adt2::C0(h1311, _, _, _, _, _, _, _, _, _, _) => {
                h1311
            }
        };
        let a1313 = 0u64;
        nat_eq(a1312, a1313)
    } else {
        false
    } {
        f92(match v0.clone() {
            Adt2::C0(_, h1314, _, _, _, _, _, _, _, _, _) => {
                h1314
            }
        }, match v0.clone() {
            Adt2::C0(_, _, h1315, _, _, _, _, _, _, _, _) => {
                h1315
            }
        }, match v0.clone() {
            Adt2::C0(_, _, _, h1316, _, _, _, _, _, _, _) => {
                h1316
            }
        }, match v0.clone() {
            Adt2::C0(_, _, _, _, h1317, _, _, _, _, _, _) => {
                h1317
            }
        }, match v1.clone() {
            Adt2::C0(_, h1318, _, _, _, _, _, _, _, _, _) => {
                h1318
            }
        }, match v1.clone() {
            Adt2::C0(_, _, h1319, _, _, _, _, _, _, _, _) => {
                h1319
            }
        }, match v1.clone() {
            Adt2::C0(_, _, _, h1320, _, _, _, _, _, _, _) => {
                h1320
            }
        }, match v1.clone() {
            Adt2::C0(_, _, _, _, h1321, _, _, _, _, _, _) => {
                h1321
            }
        })
    } else {
        false
    } {
        let v2: u64 = f12(v1.clone(), 1u64)?;
        Ok(Adt10::C4({
            let a1322 = v2;
            let a1323 = f12(v0.clone(), 1u64)?;
            nat_add(a1322, a1323)?
        }, 4u64, f16(v0, 1u64), f16(v1, 1u64), Adt4::C7(v2)))
    } else {
        Ok(Adt10::C1)
    }
}

pub fn f47(v0: u64, v1: Adt2, v2: Adt2) -> R<Adt10> {
    let a1325 = match v1.clone() {
        Adt2::C0(h1324, _, _, _, _, _, _, _, _, _, _) => {
            h1324
        }
    };
    let a1326 = 0u64;
    if if if nat_eq(a1325, a1326) {
        true
    } else {
        let a1328 = match v1.clone() {
            Adt2::C0(h1327, _, _, _, _, _, _, _, _, _, _) => {
                h1327
            }
        };
        let a1329 = 1u64;
        nat_eq(a1328, a1329)
    } {
        true
    } else {
        let a1331 = match v1.clone() {
            Adt2::C0(h1330, _, _, _, _, _, _, _, _, _, _) => {
                h1330
            }
        };
        let a1332 = 30u64;
        nat_eq(a1331, a1332)
    } {
        let a1335 = match v1.clone() {
            Adt2::C0(h1333, _, _, _, _, _, _, _, _, _, _) => {
                h1333
            }
        };
        let a1336 = match v2.clone() {
            Adt2::C0(h1334, _, _, _, _, _, _, _, _, _, _) => {
                h1334
            }
        };
        if if if if if nat_eq(a1335, a1336) {
            let a1339 = match v1.clone() {
                Adt2::C0(_, h1337, _, _, _, _, _, _, _, _, _) => {
                    h1337
                }
            };
            let a1340 = match v2.clone() {
                Adt2::C0(_, h1338, _, _, _, _, _, _, _, _, _) => {
                    h1338
                }
            };
            nat_eq(a1339, a1340)
        } else {
            false
        } {
            let a1343 = match v1.clone() {
                Adt2::C0(_, _, _, h1341, _, _, _, _, _, _, _) => {
                    h1341
                }
            };
            let a1344 = match v2.clone() {
                Adt2::C0(_, _, _, h1342, _, _, _, _, _, _, _) => {
                    h1342
                }
            };
            nat_eq(a1343, a1344)
        } else {
            false
        } {
            let a1347 = match v1.clone() {
                Adt2::C0(_, _, _, _, h1345, _, _, _, _, _, _) => {
                    h1345
                }
            };
            let a1348 = match v2.clone() {
                Adt2::C0(_, _, _, _, h1346, _, _, _, _, _, _) => {
                    h1346
                }
            };
            nat_eq(a1347, a1348)
        } else {
            false
        } {
            let a1349 = v0;
            let a1350 = 4u64;
            if nat_eq(a1349, a1350) {
                let a1352 = match v1.clone() {
                    Adt2::C0(_, _, h1351, _, _, _, _, _, _, _, _) => {
                        h1351
                    }
                };
                let a1353 = 1u64;
                if nat_eq(a1352, a1353) {
                    let a1356 = match v1.clone() {
                        Adt2::C0(_, h1354, _, _, _, _, _, _, _, _, _) => {
                            h1354
                        }
                    };
                    let a1357 = match v2.clone() {
                        Adt2::C0(_, _, h1355, _, _, _, _, _, _, _, _) => {
                            h1355
                        }
                    };
                    nat_eq(a1356, a1357)
                } else {
                    false
                }
            } else {
                let a1358 = v0;
                let a1359 = 4u64;
                if if nat_lt(a1358, a1359) {
                    let a1362 = match v1.clone() {
                        Adt2::C0(_, h1360, _, _, _, _, _, _, _, _, _) => {
                            h1360
                        }
                    };
                    let a1363 = match v1.clone() {
                        Adt2::C0(_, _, h1361, _, _, _, _, _, _, _, _) => {
                            h1361
                        }
                    };
                    nat_eq(a1362, a1363)
                } else {
                    false
                } {
                    let a1366 = match v1.clone() {
                        Adt2::C0(_, _, h1364, _, _, _, _, _, _, _, _) => {
                            h1364
                        }
                    };
                    let a1367 = match v2.clone() {
                        Adt2::C0(_, _, h1365, _, _, _, _, _, _, _, _) => {
                            h1365
                        }
                    };
                    nat_eq(a1366, a1367)
                } else {
                    false
                }
            }
        } else {
            false
        } {
            let m1369 = f91(match v1.clone() {
                Adt2::C0(h1368, _, _, _, _, _, _, _, _, _, _) => {
                    h1368
                }
            });
            match m1369 {
                Adt14::C0 => {
                    Ok(Adt10::C0)
                }
                Adt14::C1(v3) => {
                    Ok(Adt10::C4(f12(v2.clone(), 1u64)?, match v3 {
                        Adt15::C0(_, _, h1370) => {
                            h1370
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
    if if if f97(match v1.clone() {
        Adt2::C0(h1371, _, _, _, _, _, _, _, _, _, _) => {
            h1371
        }
    }, v0) {
        let a1374 = match v1.clone() {
            Adt2::C0(h1372, _, _, _, _, _, _, _, _, _, _) => {
                h1372
            }
        };
        let a1375 = match v2.clone() {
            Adt2::C0(h1373, _, _, _, _, _, _, _, _, _, _) => {
                h1373
            }
        };
        nat_eq(a1374, a1375)
    } else {
        false
    } {
        let a1378 = match v1.clone() {
            Adt2::C0(_, h1376, _, _, _, _, _, _, _, _, _) => {
                h1376
            }
        };
        let a1379 = match v2.clone() {
            Adt2::C0(_, h1377, _, _, _, _, _, _, _, _, _) => {
                h1377
            }
        };
        if if if nat_eq(a1378, a1379) {
            let a1382 = match v1.clone() {
                Adt2::C0(_, _, h1380, _, _, _, _, _, _, _, _) => {
                    h1380
                }
            };
            let a1383 = match v2.clone() {
                Adt2::C0(_, _, h1381, _, _, _, _, _, _, _, _) => {
                    h1381
                }
            };
            nat_eq(a1382, a1383)
        } else {
            false
        } {
            let a1386 = match v1.clone() {
                Adt2::C0(_, _, _, h1384, _, _, _, _, _, _, _) => {
                    h1384
                }
            };
            let a1387 = match v2.clone() {
                Adt2::C0(_, _, _, h1385, _, _, _, _, _, _, _) => {
                    h1385
                }
            };
            nat_eq(a1386, a1387)
        } else {
            false
        } {
            let a1390 = match v1.clone() {
                Adt2::C0(_, _, _, _, h1388, _, _, _, _, _, _) => {
                    h1388
                }
            };
            let a1391 = match v2.clone() {
                Adt2::C0(_, _, _, _, h1389, _, _, _, _, _, _) => {
                    h1389
                }
            };
            nat_eq(a1390, a1391)
        } else {
            false
        }
    } else {
        false
    } {
        let m1393 = f91(match v1.clone() {
            Adt2::C0(h1392, _, _, _, _, _, _, _, _, _, _) => {
                h1392
            }
        });
        match m1393 {
            Adt14::C0 => {
                Ok(Adt10::C0)
            }
            Adt14::C1(v3) => {
                Ok(Adt10::C4(f12(v2.clone(), 1u64)?, match v3 {
                    Adt15::C0(_, _, h1394) => {
                        h1394
                    }
                }, f16(v1.clone(), 1u64), f16(v2, 1u64), Adt4::C9(match v1 {
                    Adt2::C0(h1395, _, _, _, _, _, _, _, _, _, _) => {
                        h1395
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
            Adt1::C0(h1396, _, _, _, _, _, _, _, _, _, _, _) => {
                h1396
            }
        }
    } else {
        match v1.clone() {
            Adt1::C0(_, _, h1397, _, _, _, _, _, _, _, _, _) => {
                h1397
            }
        }
    };
    let a1398 = 0u64;
    let a1399 = v4;
    if if nat_lt(a1398, a1399) {
        let a1400 = v4;
        let a1401 = 2147483648u64;
        nat_lt(a1400, a1401)
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
        let v7: u64 = f98(match v5.clone() {
            Adt2::C0(_, _, h1402, _, _, _, _, _, _, _, _) => {
                h1402
            }
        }, v4)?;
        let v8: u64 = f98(match v5.clone() {
            Adt2::C0(_, _, _, h1403, _, _, _, _, _, _, _) => {
                h1403
            }
        }, v4)?;
        let a1405 = match v5.clone() {
            Adt2::C0(h1404, _, _, _, _, _, _, _, _, _, _) => {
                h1404
            }
        };
        let a1406 = 0u64;
        if if if if if nat_eq(a1405, a1406) {
            let a1408 = match v6.clone() {
                Adt2::C0(h1407, _, _, _, _, _, _, _, _, _, _) => {
                    h1407
                }
            };
            let a1409 = 0u64;
            nat_eq(a1408, a1409)
        } else {
            false
        } {
            let a1411 = match v5.clone() {
                Adt2::C0(_, _, _, _, h1410, _, _, _, _, _, _) => {
                    h1410
                }
            };
            let a1412 = 1u64;
            nat_eq(a1411, a1412)
        } else {
            false
        } {
            let a1415 = match v5.clone() {
                Adt2::C0(_, h1413, _, _, _, _, _, _, _, _, _) => {
                    h1413
                }
            };
            let a1416 = match v6.clone() {
                Adt2::C0(_, h1414, _, _, _, _, _, _, _, _, _) => {
                    h1414
                }
            };
            nat_eq(a1415, a1416)
        } else {
            false
        } {
            if v0 {
                let a1418 = match v5.clone() {
                    Adt2::C0(_, _, h1417, _, _, _, _, _, _, _, _) => {
                        h1417
                    }
                };
                let a1419 = 2147483648u64;
                if if if nat_lt(a1418, a1419) {
                    let a1421 = match v5.clone() {
                        Adt2::C0(_, _, _, h1420, _, _, _, _, _, _, _) => {
                            h1420
                        }
                    };
                    let a1422 = 2147483648u64;
                    nat_lt(a1421, a1422)
                } else {
                    false
                } {
                    let a1425 = {
                        let a1423 = v7;
                        let a1424 = v8;
                        nat_mul(a1423, a1424)?
                    };
                    let a1426 = 2147483648u64;
                    nat_le(a1425, a1426)
                } else {
                    false
                } {
                    f99(v5, v6, v4, v7)?
                } else {
                    false
                }
            } else {
                let a1428 = match v1.clone() {
                    Adt1::C0(h1427, _, _, _, _, _, _, _, _, _, _, _) => {
                        h1427
                    }
                };
                let a1429 = v7;
                if if if if nat_eq(a1428, a1429) {
                    let a1431 = match v1 {
                        Adt1::C0(_, h1430, _, _, _, _, _, _, _, _, _, _) => {
                            h1430
                        }
                    };
                    let a1432 = v8;
                    nat_eq(a1431, a1432)
                } else {
                    false
                } {
                    f100(v7, v8, v4)?
                } else {
                    false
                } {
                    let a1434 = match v6.clone() {
                        Adt2::C0(_, _, h1433, _, _, _, _, _, _, _, _) => {
                            h1433
                        }
                    };
                    let a1435 = v4;
                    if nat_eq(a1434, a1435) {
                        let a1437 = match v6.clone() {
                            Adt2::C0(_, _, _, h1436, _, _, _, _, _, _, _) => {
                                h1436
                            }
                        };
                        let a1438 = v4;
                        nat_eq(a1437, a1438)
                    } else {
                        false
                    }
                } else {
                    false
                } {
                    let a1442 = match v6 {
                        Adt2::C0(_, _, _, _, h1439, _, _, _, _, _, _) => {
                            h1439
                        }
                    };
                    let a1443 = {
                        let a1440 = v7;
                        let a1441 = v8;
                        nat_mul(a1440, a1441)?
                    };
                    nat_eq(a1442, a1443)
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
    let a1445 = match v2.clone() {
        Adt2::C0(h1444, _, _, _, _, _, _, _, _, _, _) => {
            h1444
        }
    };
    let a1446 = 0u64;
    if if if if if nat_eq(a1445, a1446) {
        true
    } else {
        let a1448 = match v2.clone() {
            Adt2::C0(h1447, _, _, _, _, _, _, _, _, _, _) => {
                h1447
            }
        };
        let a1449 = 1u64;
        nat_eq(a1448, a1449)
    } {
        true
    } else {
        let a1451 = match v2.clone() {
            Adt2::C0(h1450, _, _, _, _, _, _, _, _, _, _) => {
                h1450
            }
        };
        let a1452 = 30u64;
        nat_eq(a1451, a1452)
    } {
        let a1455 = match v2.clone() {
            Adt2::C0(h1453, _, _, _, _, _, _, _, _, _, _) => {
                h1453
            }
        };
        let a1456 = match v3.clone() {
            Adt2::C0(h1454, _, _, _, _, _, _, _, _, _, _) => {
                h1454
            }
        };
        nat_eq(a1455, a1456)
    } else {
        false
    } {
        if if f19(v2.clone(), v3.clone()) {
            let a1458 = match v1.clone() {
                Adt1::C0(h1457, _, _, _, _, _, _, _, _, _, _, _) => {
                    h1457
                }
            };
            let a1459 = 4294967296u64;
            nat_lt(a1458, a1459)
        } else {
            false
        } {
            let a1461 = match v1.clone() {
                Adt1::C0(_, h1460, _, _, _, _, _, _, _, _, _, _) => {
                    h1460
                }
            };
            let a1462 = 4294967296u64;
            nat_lt(a1461, a1462)
        } else {
            false
        }
    } else {
        false
    } {
        Ok(Adt10::C4(f12(v3.clone(), 1u64)?, {
            let a1464 = match v2.clone() {
                Adt2::C0(h1463, _, _, _, _, _, _, _, _, _, _) => {
                    h1463
                }
            };
            let a1465 = 0u64;
            if nat_eq(a1464, a1465) {
                4u64
            } else {
                2u64
            }
        }, f16(v2.clone(), 1u64), f16(v3, 1u64), Adt4::C13({
            let a1466 = v0;
            let a1467 = 31u64;
            if nat_eq(a1466, a1467) {
                0u64
            } else {
                let a1468 = v0;
                let a1469 = 47u64;
                if nat_eq(a1468, a1469) {
                    1u64
                } else {
                    2u64
                }
            }
        }, match v2 {
            Adt2::C0(h1470, _, _, _, _, _, _, _, _, _, _) => {
                h1470
            }
        }, match v1.clone() {
            Adt1::C0(h1471, _, _, _, _, _, _, _, _, _, _, _) => {
                h1471
            }
        }, match v1 {
            Adt1::C0(_, h1472, _, _, _, _, _, _, _, _, _, _) => {
                h1472
            }
        })))
    } else {
        Ok(Adt10::C1)
    }
}

pub fn f51(v0: Adt2, v1: Adt2) -> R<Adt10> {
    let a1474 = match v0.clone() {
        Adt2::C0(h1473, _, _, _, _, _, _, _, _, _, _) => {
            h1473
        }
    };
    let a1475 = 0u64;
    if if if nat_eq(a1474, a1475) {
        let a1477 = match v1.clone() {
            Adt2::C0(h1476, _, _, _, _, _, _, _, _, _, _) => {
                h1476
            }
        };
        let a1478 = 0u64;
        nat_eq(a1477, a1478)
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
    let a1480 = match v0.clone() {
        Adt2::C0(h1479, _, _, _, _, _, _, _, _, _, _) => {
            h1479
        }
    };
    let a1481 = 0u64;
    if if nat_eq(a1480, a1481) {
        let a1483 = match v1.clone() {
            Adt2::C0(h1482, _, _, _, _, _, _, _, _, _, _) => {
                h1482
            }
        };
        let a1484 = 26u64;
        nat_eq(a1483, a1484)
    } else {
        false
    } {
        let a1486 = match v0.clone() {
            Adt2::C0(_, _, _, h1485, _, _, _, _, _, _, _) => {
                h1485
            }
        };
        let a1487 = 1u64;
        if if if if if if if nat_eq(a1486, a1487) {
            let a1489 = match v0.clone() {
                Adt2::C0(_, _, _, _, h1488, _, _, _, _, _, _) => {
                    h1488
                }
            };
            let a1490 = 1u64;
            nat_eq(a1489, a1490)
        } else {
            false
        } {
            let a1492 = match v0.clone() {
                Adt2::C0(_, h1491, _, _, _, _, _, _, _, _, _) => {
                    h1491
                }
            };
            let a1493 = 2147483647u64;
            nat_le(a1492, a1493)
        } else {
            false
        } {
            let a1496 = match v1.clone() {
                Adt2::C0(_, h1494, _, _, _, _, _, _, _, _, _) => {
                    h1494
                }
            };
            let a1497 = match v0.clone() {
                Adt2::C0(_, _, h1495, _, _, _, _, _, _, _, _) => {
                    h1495
                }
            };
            nat_eq(a1496, a1497)
        } else {
            false
        } {
            let a1499 = match v1.clone() {
                Adt2::C0(_, _, h1498, _, _, _, _, _, _, _, _) => {
                    h1498
                }
            };
            let a1500 = 1u64;
            nat_eq(a1499, a1500)
        } else {
            false
        } {
            let a1502 = match v1.clone() {
                Adt2::C0(_, _, _, h1501, _, _, _, _, _, _, _) => {
                    h1501
                }
            };
            let a1503 = 1u64;
            nat_eq(a1502, a1503)
        } else {
            false
        } {
            let a1505 = match v1.clone() {
                Adt2::C0(_, _, _, _, h1504, _, _, _, _, _, _) => {
                    h1504
                }
            };
            let a1506 = 1u64;
            nat_eq(a1505, a1506)
        } else {
            false
        } {
            Ok(Adt10::C4({
                let a1509 = match v0.clone() {
                    Adt2::C0(_, _, h1507, _, _, _, _, _, _, _, _) => {
                        h1507
                    }
                };
                let a1510 = f101(match v0.clone() {
                    Adt2::C0(_, h1508, _, _, _, _, _, _, _, _, _) => {
                        h1508
                    }
                })?;
                nat_mul(a1509, a1510)?
            }, 4u64, f16(v0, 1u64), f16(v1, 1u64), Adt4::C17))
        } else {
            Ok(Adt10::C1)
        }
    } else {
        Ok(Adt10::C0)
    }
}

pub fn f53(v0: Adt2, v1: Option<Adt2>, v2: Adt2) -> R<Adt10> {
    let m1511 = v1;
    match m1511 {
        None => {
            Ok(Adt10::C1)
        }
        Some(v3) => {
            let v4: u64 = f102(f16(v0.clone(), 1u64))?;
            let a1513 = match v0.clone() {
                Adt2::C0(h1512, _, _, _, _, _, _, _, _, _, _) => {
                    h1512
                }
            };
            let a1514 = 26u64;
            if if if nat_eq(a1513, a1514) {
                let a1516 = match v3.clone() {
                    Adt2::C0(h1515, _, _, _, _, _, _, _, _, _, _) => {
                        h1515
                    }
                };
                let a1517 = 26u64;
                nat_eq(a1516, a1517)
            } else {
                false
            } {
                let a1519 = match v2.clone() {
                    Adt2::C0(h1518, _, _, _, _, _, _, _, _, _, _) => {
                        h1518
                    }
                };
                let a1520 = 27u64;
                nat_eq(a1519, a1520)
            } else {
                false
            } {
                if if if if if if f19(v0.clone(), v3.clone()) {
                    let a1522 = match v2.clone() {
                        Adt2::C0(_, h1521, _, _, _, _, _, _, _, _, _) => {
                            h1521
                        }
                    };
                    let a1523 = 1u64;
                    nat_eq(a1522, a1523)
                } else {
                    false
                } {
                    let a1525 = match v2.clone() {
                        Adt2::C0(_, _, h1524, _, _, _, _, _, _, _, _) => {
                            h1524
                        }
                    };
                    let a1526 = 1u64;
                    nat_eq(a1525, a1526)
                } else {
                    false
                } {
                    let a1528 = match v2.clone() {
                        Adt2::C0(_, _, _, h1527, _, _, _, _, _, _, _) => {
                            h1527
                        }
                    };
                    let a1529 = 1u64;
                    nat_eq(a1528, a1529)
                } else {
                    false
                } {
                    let a1531 = match v2.clone() {
                        Adt2::C0(_, _, _, _, h1530, _, _, _, _, _, _) => {
                            h1530
                        }
                    };
                    let a1532 = 1u64;
                    nat_eq(a1531, a1532)
                } else {
                    false
                } {
                    let a1533 = v4;
                    let a1534 = 9223372036854775808u64;
                    nat_lt(a1533, a1534)
                } else {
                    false
                } {
                    Ok(Adt10::C4({
                        let a1537 = 2u64;
                        let a1538 = {
                            let a1535 = v4;
                            let a1536 = 2u64;
                            nat_mul(a1535, a1536)?
                        };
                        nat_add(a1537, a1538)?
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
        Adt2::C0(h1539, _, _, _, _, _, _, _, _, _, _) => {
            h1539
        }
    };
    let a1540 = v3;
    let a1541 = 0u64;
    if if nat_eq(a1540, a1541) {
        true
    } else {
        let a1542 = v0;
        let a1543 = 13u64;
        if nat_eq(a1542, a1543) {
            let a1544 = v3;
            let a1545 = 1u64;
            if nat_eq(a1544, a1545) {
                true
            } else {
                let a1546 = v3;
                let a1547 = 30u64;
                nat_eq(a1546, a1547)
            }
        } else {
            false
        }
    } {
        let a1549 = v3;
        let a1550 = match v2.clone() {
            Adt2::C0(h1548, _, _, _, _, _, _, _, _, _, _) => {
                h1548
            }
        };
        if if nat_eq(a1549, a1550) {
            let a1551 = v0;
            let a1552 = 13u64;
            if nat_eq(a1551, a1552) {
                let a1554 = match v2.clone() {
                    Adt2::C0(_, h1553, _, _, _, _, _, _, _, _, _) => {
                        h1553
                    }
                };
                let a1555 = 1u64;
                if if if nat_eq(a1554, a1555) {
                    let a1557 = match v2.clone() {
                        Adt2::C0(_, _, h1556, _, _, _, _, _, _, _, _) => {
                            h1556
                        }
                    };
                    let a1558 = 1u64;
                    nat_eq(a1557, a1558)
                } else {
                    false
                } {
                    let a1560 = match v2.clone() {
                        Adt2::C0(_, _, _, h1559, _, _, _, _, _, _, _) => {
                            h1559
                        }
                    };
                    let a1561 = 1u64;
                    nat_eq(a1560, a1561)
                } else {
                    false
                } {
                    let a1563 = match v2.clone() {
                        Adt2::C0(_, _, _, _, h1562, _, _, _, _, _, _) => {
                            h1562
                        }
                    };
                    let a1564 = 1u64;
                    nat_eq(a1563, a1564)
                } else {
                    false
                }
            } else {
                let a1566 = match v2.clone() {
                    Adt2::C0(_, h1565, _, _, _, _, _, _, _, _, _) => {
                        h1565
                    }
                };
                let a1567 = 1u64;
                if if if nat_eq(a1566, a1567) {
                    let a1570 = match v1.clone() {
                        Adt2::C0(_, _, h1568, _, _, _, _, _, _, _, _) => {
                            h1568
                        }
                    };
                    let a1571 = match v2.clone() {
                        Adt2::C0(_, _, h1569, _, _, _, _, _, _, _, _) => {
                            h1569
                        }
                    };
                    nat_eq(a1570, a1571)
                } else {
                    false
                } {
                    let a1574 = match v1.clone() {
                        Adt2::C0(_, _, _, h1572, _, _, _, _, _, _, _) => {
                            h1572
                        }
                    };
                    let a1575 = match v2.clone() {
                        Adt2::C0(_, _, _, h1573, _, _, _, _, _, _, _) => {
                            h1573
                        }
                    };
                    nat_eq(a1574, a1575)
                } else {
                    false
                } {
                    let a1578 = match v1.clone() {
                        Adt2::C0(_, _, _, _, h1576, _, _, _, _, _, _) => {
                            h1576
                        }
                    };
                    let a1579 = match v2.clone() {
                        Adt2::C0(_, _, _, _, h1577, _, _, _, _, _, _) => {
                            h1577
                        }
                    };
                    nat_eq(a1578, a1579)
                } else {
                    false
                }
            }
        } else {
            false
        } {
            let v4: Adt3 = f16(v1, 1u64);
            Ok(Adt10::C4(f103(v4.clone(), v0, v3)?, {
                let a1580 = v3;
                let a1581 = 0u64;
                if nat_eq(a1580, a1581) {
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
    let a1582 = v0;
    let a1583 = 89u64;
    if if nat_eq(a1582, a1583) {
        let a1584 = 8u64;
        let a1585 = v1;
        if nat_le(a1584, a1585) {
            let a1586 = v1;
            let a1587 = 9u64;
            nat_le(a1586, a1587)
        } else {
            false
        }
    } else {
        false
    } {
        Ok(Some({
            let a1588 = v1;
            let a1589 = 12u64;
            nat_add(a1588, a1589)?
        }))
    } else {
        let a1590 = v0;
        let a1591 = 89u64;
        if if nat_eq(a1590, a1591) {
            let a1592 = v1;
            let a1593 = 4u64;
            nat_eq(a1592, a1593)
        } else {
            false
        } {
            Ok(Some(19u64))
        } else {
            let a1594 = 10u64;
            let a1595 = v0;
            if if nat_le(a1594, a1595) {
                let a1596 = v0;
                let a1597 = 12u64;
                nat_le(a1596, a1597)
            } else {
                false
            } {
                Ok(Some({
                    let a1598 = v0;
                    let a1599 = 3u64;
                    nat_add(a1598, a1599)?
                }))
            } else {
                let a1600 = v0;
                let a1601 = 89u64;
                if if nat_eq(a1600, a1601) {
                    let a1602 = v1;
                    let a1603 = 7u64;
                    nat_eq(a1602, a1603)
                } else {
                    false
                } {
                    Ok(Some(17u64))
                } else {
                    let a1604 = v0;
                    let a1605 = 89u64;
                    if if nat_eq(a1604, a1605) {
                        let a1606 = v1;
                        let a1607 = 10u64;
                        nat_eq(a1606, a1607)
                    } else {
                        false
                    } {
                        Ok(Some(18u64))
                    } else {
                        let a1608 = v0;
                        let a1609 = 89u64;
                        if if nat_eq(a1608, a1609) {
                            let a1610 = v1;
                            let a1611 = 13u64;
                            nat_eq(a1610, a1611)
                        } else {
                            false
                        } {
                            Ok(Some(16u64))
                        } else {
                            let a1612 = v0;
                            let a1613 = 8u64;
                            if nat_eq(a1612, a1613) {
                                Ok(Some(5u64))
                            } else {
                                let a1614 = v0;
                                let a1615 = 9u64;
                                if nat_eq(a1614, a1615) {
                                    Ok(Some(6u64))
                                } else {
                                    let a1616 = v0;
                                    let a1617 = 89u64;
                                    if nat_eq(a1616, a1617) {
                                        let a1618 = v1;
                                        let a1619 = 0u64;
                                        if nat_eq(a1618, a1619) {
                                            Ok(Some(0u64))
                                        } else {
                                            let a1620 = v1;
                                            let a1621 = 1u64;
                                            if nat_eq(a1620, a1621) {
                                                Ok(Some(1u64))
                                            } else {
                                                let a1622 = v1;
                                                let a1623 = 2u64;
                                                if nat_eq(a1622, a1623) {
                                                    Ok(Some(2u64))
                                                } else {
                                                    let a1624 = v1;
                                                    let a1625 = 3u64;
                                                    if nat_eq(a1624, a1625) {
                                                        Ok(Some(3u64))
                                                    } else {
                                                        let a1626 = v1;
                                                        let a1627 = 6u64;
                                                        if nat_eq(a1626, a1627) {
                                                            Ok(Some(4u64))
                                                        } else {
                                                            let a1628 = v1;
                                                            let a1629 = 12u64;
                                                            if nat_eq(a1628, a1629) {
                                                                Ok(Some(7u64))
                                                            } else {
                                                                let a1630 = v1;
                                                                let a1631 = 11u64;
                                                                if nat_eq(a1630, a1631) {
                                                                    Ok(Some(8u64))
                                                                } else {
                                                                    let a1632 = v1;
                                                                    let a1633 = 18u64;
                                                                    if nat_eq(a1632, a1633) {
                                                                        Ok(Some(9u64))
                                                                    } else {
                                                                        let a1634 = v1;
                                                                        let a1635 = 19u64;
                                                                        if nat_eq(a1634, a1635) {
                                                                            Ok(Some(10u64))
                                                                        } else {
                                                                            let a1636 = v1;
                                                                            let a1637 = 20u64;
                                                                            if nat_eq(a1636, a1637) {
                                                                                Ok(Some(11u64))
                                                                            } else {
                                                                                let a1638 = v1;
                                                                                let a1639 = 21u64;
                                                                                if nat_eq(a1638, a1639) {
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
    let m1640 = f104(v0, v1, v2.clone(), v3.clone(), v4.clone())?;
    match m1640 {
        Adt10::C0 => {
            Ok(Adt10::C0)
        }
        Adt10::C1 => {
            let a1641 = v0;
            let a1642 = 2u64;
            if if if if nat_eq(a1641, a1642) {
                true
            } else {
                let a1643 = v0;
                let a1644 = 5u64;
                nat_eq(a1643, a1644)
            } {
                true
            } else {
                let a1645 = v0;
                let a1646 = 6u64;
                nat_eq(a1645, a1646)
            } {
                true
            } else {
                let a1647 = v0;
                let a1648 = 7u64;
                nat_eq(a1647, a1648)
            } {
                Ok(Adt10::C5(67u64))
            } else {
                Ok(Adt10::C1)
            }
        }
        Adt10::C2(v7, v8, _, _, _, _, v13) => {
            if v8 {
                Ok(Adt10::C4(v13, 4u64, f16(v2.clone(), 1u64), f16(v4, 1u64), Adt4::C10(v7, v8, {
                    let m1649 = v3;
                    match m1649 {
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
    let a1651 = match v1.clone() {
        Adt2::C0(h1650, _, _, _, _, _, _, _, _, _, _) => {
            h1650
        }
    };
    let a1652 = 0u64;
    if if if if if nat_eq(a1651, a1652) {
        true
    } else {
        let a1654 = match v1.clone() {
            Adt2::C0(h1653, _, _, _, _, _, _, _, _, _, _) => {
                h1653
            }
        };
        let a1655 = 1u64;
        nat_eq(a1654, a1655)
    } {
        true
    } else {
        let a1657 = match v1.clone() {
            Adt2::C0(h1656, _, _, _, _, _, _, _, _, _, _) => {
                h1656
            }
        };
        let a1658 = 30u64;
        nat_eq(a1657, a1658)
    } {
        let a1661 = match v1.clone() {
            Adt2::C0(h1659, _, _, _, _, _, _, _, _, _, _) => {
                h1659
            }
        };
        let a1662 = match v2.clone() {
            Adt2::C0(h1660, _, _, _, _, _, _, _, _, _, _) => {
                h1660
            }
        };
        if nat_eq(a1661, a1662) {
            true
        } else {
            let a1664 = match v2.clone() {
                Adt2::C0(h1663, _, _, _, _, _, _, _, _, _, _) => {
                    h1663
                }
            };
            let a1665 = 0u64;
            nat_eq(a1664, a1665)
        }
    } else {
        false
    } {
        f19(v1.clone(), v2.clone())
    } else {
        false
    } {
        Ok(Adt10::C4(f12(v2.clone(), 1u64)?, {
            let a1667 = match v2.clone() {
                Adt2::C0(h1666, _, _, _, _, _, _, _, _, _, _) => {
                    h1666
                }
            };
            let a1668 = 0u64;
            if nat_eq(a1667, a1668) {
                4u64
            } else {
                2u64
            }
        }, f16(v1.clone(), 1u64), f16(v2.clone(), 1u64), Adt4::C14(v0, match v1 {
            Adt2::C0(h1669, _, _, _, _, _, _, _, _, _, _) => {
                h1669
            }
        }, match v2 {
            Adt2::C0(h1670, _, _, _, _, _, _, _, _, _, _) => {
                h1670
            }
        })))
    } else {
        Ok(Adt10::C1)
    }
}

pub fn f58(v0: Adt3, v1: u64, v2: u64, v3: u64, v4: u64) -> R<u64> {
    f105(v1, v2, v3, v4, match v0.clone() {
        Adt3::C0(_, _, _, _, h1671, _, _, _) => {
            h1671
        }
    }, match v0.clone() {
        Adt3::C0(_, _, _, _, _, h1672, _, _) => {
            h1672
        }
    }, match v0.clone() {
        Adt3::C0(_, _, _, _, _, _, h1673, _) => {
            h1673
        }
    }, match v0 {
        Adt3::C0(_, _, _, _, _, _, _, h1674) => {
            h1674
        }
    })
}

pub fn f59(v0: u64, v1: Adt3, v2: u64) -> u64 {
    let v3: u64 = {
        let a1676 = v0;
        let a1677 = match v1.clone() {
            Adt3::C0(h1675, _, _, _, _, _, _, _) => {
                h1675
            }
        };
        let a1678 = 0u64;
        nat_quot(a1676, a1677, a1678)
    };
    let v4: u64 = {
        let a1680 = v3;
        let a1681 = match v1.clone() {
            Adt3::C0(_, h1679, _, _, _, _, _, _) => {
                h1679
            }
        };
        let a1682 = 0u64;
        nat_quot(a1680, a1681, a1682)
    };
    let a1683 = v2;
    let a1684 = 0u64;
    if nat_eq(a1683, a1684) {
        let a1686 = v0;
        let a1687 = match v1 {
            Adt3::C0(h1685, _, _, _, _, _, _, _) => {
                h1685
            }
        };
        let a1688 = 0u64;
        nat_rem(a1686, a1687, a1688)
    } else {
        let a1689 = v2;
        let a1690 = 1u64;
        if nat_eq(a1689, a1690) {
            let a1692 = v3;
            let a1693 = match v1 {
                Adt3::C0(_, h1691, _, _, _, _, _, _) => {
                    h1691
                }
            };
            let a1694 = 0u64;
            nat_rem(a1692, a1693, a1694)
        } else {
            let a1695 = v2;
            let a1696 = 2u64;
            if nat_eq(a1695, a1696) {
                let a1698 = v4;
                let a1699 = match v1 {
                    Adt3::C0(_, _, h1697, _, _, _, _, _) => {
                        h1697
                    }
                };
                let a1700 = 0u64;
                nat_rem(a1698, a1699, a1700)
            } else {
                let a1702 = v4;
                let a1703 = match v1 {
                    Adt3::C0(_, _, h1701, _, _, _, _, _) => {
                        h1701
                    }
                };
                let a1704 = 0u64;
                nat_quot(a1702, a1703, a1704)
            }
        }
    }
}

pub fn f60(v0: Adt8, v1: u64, v2: u64) -> R<u64> {
    let a1706 = match v0.clone() {
        Adt8::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, h1705) => {
            h1705
        }
    };
    let a1707 = 0u64;
    if nat_eq(a1706, a1707) {
        Ok(v2)
    } else {
        let a1712 = {
            let a1709 = v2;
            let a1710 = match v0.clone() {
                Adt8::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, h1708) => {
                    h1708
                }
            };
            nat_mul(a1709, a1710)?
        };
        let a1713 = f59(v1, match v0 {
            Adt8::C0(_, _, _, _, _, _, _, _, h1711, _, _, _, _, _, _) => {
                h1711
            }
        }, 3u64);
        nat_add(a1712, a1713)
    }
}

pub fn f61(v0: Adt6, v1: u64, v2: u64, v3: u64) -> R<Option<u64>> {
    let v4: u64 = {
        let a1723 = {
            let a1720 = {
                let a1715 = v2;
                let a1716 = match v0.clone() {
                    Adt6::C0(_, h1714, _, _) => {
                        h1714
                    }
                };
                nat_mul(a1715, a1716)?
            };
            let a1721 = {
                let a1718 = v3;
                let a1719 = match v0.clone() {
                    Adt6::C0(_, _, _, h1717) => {
                        h1717
                    }
                };
                nat_mul(a1718, a1719)?
            };
            nat_add(a1720, a1721)?
        };
        let a1724 = f106(match v0.clone() {
            Adt6::C0(_, _, h1722, _) => {
                h1722
            }
        });
        nat_add(a1723, a1724)?
    };
    let v5: u64 = f107(match v0 {
        Adt6::C0(_, _, h1725, _) => {
            h1725
        }
    });
    let a1726 = v5;
    let a1727 = v4;
    if if nat_le(a1726, a1727) {
        let a1730 = {
            let a1728 = v4;
            let a1729 = v5;
            nat_sub(a1728, a1729)
        };
        let a1731 = v1;
        nat_lt(a1730, a1731)
    } else {
        false
    } {
        Ok(Some({
            let a1732 = v4;
            let a1733 = v5;
            nat_sub(a1732, a1733)
        }))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f62(v0: Adt8, v1: u64, v2: u64, v3: u64, v4: u64) -> R<Adt9> {
    let v5: Adt3 = match v0.clone() {
        Adt8::C0(_, _, _, _, _, _, _, _, _, h1734, _, _, _, _, _) => {
            h1734
        }
    };
    let v6: Adt3 = match v0.clone() {
        Adt8::C0(_, _, _, _, _, _, _, _, h1735, _, _, _, _, _, _) => {
            h1735
        }
    };
    let v7: u64 = f59(v1, v6.clone(), 2u64);
    let a1737 = match v0.clone() {
        Adt8::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, h1736) => {
            h1736
        }
    };
    let a1738 = 0u64;
    if nat_eq(a1737, a1738) {
        Ok(Adt9::C8(Some(f58(v5, v3, v4, v7, v2)?)))
    } else {
        let v8: u64 = {
            let m1741 = match v0.clone() {
                Adt8::C0(_, _, _, _, _, _, _, _, _, _, h1740, _, _, _, _) => {
                    h1740
                }
            };
            match m1741 {
                Adt3::C0(_, _, h1739, _, _, _, _, _) => {
                    h1739
                }
            }
        };
        let m1747 = f61(match v0.clone() {
            Adt8::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, h1742, _) => {
                h1742
            }
        }, match v5.clone() {
            Adt3::C0(_, _, h1743, _, _, _, _, _) => {
                h1743
            }
        }, {
            let a1744 = v2;
            let a1745 = v8;
            let a1746 = 0u64;
            nat_rem(a1744, a1745, a1746)
        }, v7)?;
        match m1747 {
            None => {
                Ok(Adt9::C8(None::<u64>))
            }
            Some(v9) => {
                let v10: u64 = {
                    let a1754 = {
                        let a1752 = {
                            let a1748 = v2;
                            let a1749 = v8;
                            let a1750 = 0u64;
                            nat_quot(a1748, a1749, a1750)
                        };
                        let a1753 = match v0 {
                            Adt8::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, h1751) => {
                                h1751
                            }
                        };
                        nat_mul(a1752, a1753)?
                    };
                    let a1755 = f59(v1, v6, 3u64);
                    nat_add(a1754, a1755)?
                };
                Ok(Adt9::C8(Some(f58(v5, v3, v4, v9, v10)?)))
            }
        }
    }
}

pub fn f63(v0: Adt4, v1: Adt9) -> Adt9 {
    let m1756 = v1;
    match m1756 {
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
            f108(f109(v0.clone(), v14, v15), v14, v15, 0u64, v16, 2u64, f110(v0), 3u64, v17)
        }
        Adt9::C4(v18, v19, v20, v21, v22, v23, v24) => {
            let v25: u64 = f111(v0.clone(), v18);
            f108(f112(v0, v18), v25, v25, v19, v20, v21, v22, v23, v24)
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

pub fn f64(v0: u64, v1: Adt3, v2: Adt3, v3: Adt4) -> R<Adt9> {
    let m1757 = v3;
    match m1757 {
        Adt4::C0 => {
            Ok(Adt9::C1(false, f113(v0, v1)?, f113(v0, v2)?))
        }
        Adt4::C1 => {
            Ok(Adt9::C1(false, f115(v0, v1, v2.clone())?, f113(v0, v2)?))
        }
        Adt4::C2(v6, v7) => {
            f116(v0, v1, v7, v2, v6)
        }
        Adt4::C3(v8, v9, v10, v11, v12) => {
            f117(v0, v1, v2, v8, v9, v10, v11, v12)
        }
        Adt4::C4(v13) => {
            f118(v0, v1, v2, v13)
        }
        Adt4::C5(v14, v15, v16, v17) => {
            f117(v0, v1, v2, v14, v15, v16, v17, true)
        }
        Adt4::C6(v18, v19, v20) => {
            f119(v0, v1, v2, v18, v19, v20)
        }
        Adt4::C7(v21) => {
            f120(v0, v1, v2, v21)
        }
        Adt4::C8(v22) => {
            f121(v22, v0, v1, v2)
        }
        Adt4::C9(_, _) => {
            Ok(Adt9::C1(true, 0u64, f113(v0, v2)?))
        }
        Adt4::C10(v25, v26, v27) => {
            f122(v0, v1, v2, v25, v26, v27)
        }
        Adt4::C11(v28, v29) => {
            Ok(Adt9::C3(v28, v29, f113(v0, v1)?, f113(v0, v2)?))
        }
        Adt4::C12(v30, v31, v32) => {
            f123(v0, v1, v2, v30, v31, v32)
        }
        Adt4::C13(_, v34, _, _) => {
            Ok(Adt9::C3(v34, v34, f113(v0, v1)?, f113(v0, v2)?))
        }
        Adt4::C14(_, v38, v39) => {
            Ok(Adt9::C3(v38, v39, f113(v0, v1)?, f113(v0, v2)?))
        }
        Adt4::C15 => {
            f124(v0, v1, v2)
        }
        Adt4::C16(v40, v41) => {
            f125(v0, v1, v2, v40, v41)
        }
        Adt4::C17 => {
            Ok(Adt9::C0(Adt10::C1))
        }
        Adt4::C18(v42, v43) => {
            f126(v0, v1, v43, v42)
        }
        Adt4::C19(_, _) => {
            Ok(Adt9::C0(Adt10::C1))
        }
        Adt4::C20(_, _) => {
            Ok(Adt9::C0(Adt10::C1))
        }
        Adt4::C21 => {
            Ok(Adt9::C1(false, f113(f114(v0, match v2.clone() {
                Adt3::C0(h1758, _, _, _, _, _, _, _) => {
                    h1758
                }
            }, match v2.clone() {
                Adt3::C0(_, h1759, _, _, _, _, _, _) => {
                    h1759
                }
            })?, v1)?, f113(v0, v2)?))
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
            f127(v0, v48, v49, v50)
        }
        Adt4::C26(v53, _, _, _) => {
            let a1760 = v53;
            let a1761 = 64u64;
            if if if nat_eq(a1760, a1761) {
                true
            } else {
                let a1762 = v53;
                let a1763 = 65u64;
                nat_eq(a1762, a1763)
            } {
                true
            } else {
                let a1764 = v53;
                let a1765 = 66u64;
                nat_eq(a1764, a1765)
            } {
                Ok(Adt9::C0(Adt10::C5(64u64)))
            } else {
                Ok(Adt9::C0(Adt10::C1))
            }
        }
        Adt4::C27(v52) => {
            f70(v0, v1, v2, v52)
        }
        Adt4::C28(v51) => {
            f69(v0, v1, v2, v51, 0u64)
        }
    }
}

pub fn f65(v0: u64, v1: Adt3, v2: Adt3, v3: u64) -> R<Adt9> {
    let v4: u64 = f101(match v1.clone() {
        Adt3::C0(h1766, _, _, _, _, _, _, _) => {
            h1766
        }
    })?;
    let v5: u64 = {
        let a1767 = v0;
        let a1768 = v4;
        let a1769 = 0u64;
        nat_quot(a1767, a1768, a1769)
    };
    let v6: u64 = {
        let a1770 = v0;
        let a1771 = v4;
        let a1772 = 0u64;
        nat_rem(a1770, a1771, a1772)
    };
    let a1773 = v6;
    let a1774 = 0u64;
    if nat_eq(a1773, a1774) {
        Ok(Adt9::C4(19u64, 2u64, 4286578688u64, 2u64, 0u64, 1u64, 0u64))
    } else {
        let a1775 = v6;
        let a1776 = 1u64;
        if nat_eq(a1775, a1776) {
            Ok(Adt9::C4(19u64, 2u64, 0u64, 2u64, 0u64, 1u64, 1u64))
        } else {
            let a1779 = {
                let a1777 = v6;
                let a1778 = 1u64;
                nat_add(a1777, a1778)?
            };
            let a1780 = v4;
            if nat_eq(a1779, a1780) {
                Ok(Adt9::C4(19u64, 1u64, 1u64, 2u64, 0u64, 3u64, f113(v5, v2)?))
            } else {
                let v7: u64 = {
                    let a1781 = v6;
                    let a1782 = 2u64;
                    nat_sub(a1781, a1782)
                };
                let v8: u64 = {
                    let a1783 = v7;
                    let a1784 = 3u64;
                    let a1785 = 0u64;
                    nat_quot(a1783, a1784, a1785)
                };
                let v9: u64 = {
                    let a1786 = v7;
                    let a1787 = 3u64;
                    let a1788 = 0u64;
                    nat_rem(a1786, a1787, a1788)
                };
                let v10: u64 = f113({
                    let a1792 = {
                        let a1790 = v5;
                        let a1791 = match v1.clone() {
                            Adt3::C0(h1789, _, _, _, _, _, _, _) => {
                                h1789
                            }
                        };
                        nat_mul(a1790, a1791)?
                    };
                    let a1793 = v8;
                    nat_add(a1792, a1793)?
                }, v1)?;
                let a1794 = v9;
                let a1795 = 0u64;
                if nat_eq(a1794, a1795) {
                    Ok(Adt9::C4(16u64, 1u64, 0u64, 0u64, v10, 1u64, 2u64))
                } else {
                    let a1796 = v9;
                    let a1797 = 1u64;
                    if nat_eq(a1796, a1797) {
                        let a1798 = v3;
                        let a1799 = 2u64;
                        if nat_eq(a1798, a1799) {
                            Ok(Adt9::C4(19u64, 1u64, 0u64, 2u64, 0u64, 1u64, 0u64))
                        } else {
                            Ok(Adt9::C4(19u64, 0u64, v10, 2u64, 0u64, 1u64, 0u64))
                        }
                    } else {
                        let a1800 = v3;
                        let a1801 = 2u64;
                        if if nat_eq(a1800, a1801) {
                            true
                        } else {
                            let a1802 = v3;
                            let a1803 = 3u64;
                            nat_eq(a1802, a1803)
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

pub fn f66(v0: u64, v1: Adt3, v2: Adt3, v3: Adt3, v4: u64) -> R<Adt9> {
    let v5: u64 = {
        let a1805 = 2u64;
        let a1806 = match v1.clone() {
            Adt3::C0(_, h1804, _, _, _, _, _, _) => {
                h1804
            }
        };
        nat_mul(a1805, a1806)?
    };
    let a1807 = v0;
    let a1808 = v5;
    if nat_lt(a1807, a1808) {
        let a1812 = {
            let a1809 = v0;
            let a1810 = 2u64;
            let a1811 = 0u64;
            nat_rem(a1809, a1810, a1811)
        };
        let a1813 = 0u64;
        if nat_eq(a1812, a1813) {
            Ok(Adt9::C4(19u64, 4u64, f113({
                let a1814 = v0;
                let a1815 = 2u64;
                let a1816 = 0u64;
                nat_quot(a1814, a1815, a1816)
            }, v3)?, 2u64, 0u64, 1u64, 2u64))
        } else {
            let a1817 = v4;
            let a1818 = 2147483648u64;
            if if nat_lt(a1817, a1818) {
                let a1820 = v4;
                let a1821 = match v2 {
                    Adt3::C0(_, h1819, _, _, _, _, _, _) => {
                        h1819
                    }
                };
                nat_lt(a1820, a1821)
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
            let a1824 = match v2.clone() {
                Adt3::C0(h1822, _, _, _, _, _, _, _) => {
                    h1822
                }
            };
            let a1825 = match v2.clone() {
                Adt3::C0(_, h1823, _, _, _, _, _, _) => {
                    h1823
                }
            };
            nat_mul(a1824, a1825)?
        };
        let a1828 = {
            let a1826 = v0;
            let a1827 = v5;
            nat_sub(a1826, a1827)
        };
        let a1829 = v6;
        if nat_lt(a1828, a1829) {
            Ok(Adt9::C4(19u64, 2u64, 0u64, 2u64, 0u64, 3u64, f113({
                let a1830 = v0;
                let a1831 = v5;
                nat_sub(a1830, a1831)
            }, v2)?))
        } else {
            let v7: u64 = {
                let a1834 = {
                    let a1832 = v0;
                    let a1833 = v5;
                    nat_sub(a1832, a1833)
                };
                let a1835 = v6;
                nat_sub(a1834, a1835)
            };
            let v8: u64 = {
                let a1839 = 1u64;
                let a1840 = {
                    let a1837 = 2u64;
                    let a1838 = match v1.clone() {
                        Adt3::C0(h1836, _, _, _, _, _, _, _) => {
                            h1836
                        }
                    };
                    nat_mul(a1837, a1838)?
                };
                nat_add(a1839, a1840)?
            };
            let v9: u64 = {
                let a1841 = v7;
                let a1842 = v8;
                let a1843 = 0u64;
                nat_quot(a1841, a1842, a1843)
            };
            let v10: u64 = {
                let a1844 = v7;
                let a1845 = v8;
                let a1846 = 0u64;
                nat_rem(a1844, a1845, a1846)
            };
            let a1847 = v10;
            let a1848 = 0u64;
            if nat_eq(a1847, a1848) {
                Ok(Adt9::C4(19u64, 4u64, f113(v9, v3)?, 2u64, 0u64, 1u64, 2u64))
            } else {
                let a1852 = {
                    let a1849 = v10;
                    let a1850 = 2u64;
                    let a1851 = 0u64;
                    nat_rem(a1849, a1850, a1851)
                };
                let a1853 = 1u64;
                if nat_eq(a1852, a1853) {
                    Ok(Adt9::C4(6u64, 0u64, f113({
                        let a1862 = {
                            let a1855 = v9;
                            let a1856 = match v1.clone() {
                                Adt3::C0(h1854, _, _, _, _, _, _, _) => {
                                    h1854
                                }
                            };
                            nat_mul(a1855, a1856)?
                        };
                        let a1863 = {
                            let a1859 = {
                                let a1857 = v10;
                                let a1858 = 1u64;
                                nat_sub(a1857, a1858)
                            };
                            let a1860 = 2u64;
                            let a1861 = 0u64;
                            nat_quot(a1859, a1860, a1861)
                        };
                        nat_add(a1862, a1863)?
                    }, v1)?, 2u64, 0u64, 1u64, 0u64))
                } else {
                    Ok(Adt9::C4(4u64, 3u64, f113({
                        let a1872 = {
                            let a1865 = v4;
                            let a1866 = match v1.clone() {
                                Adt3::C0(h1864, _, _, _, _, _, _, _) => {
                                    h1864
                                }
                            };
                            nat_mul(a1865, a1866)?
                        };
                        let a1873 = {
                            let a1869 = {
                                let a1867 = v10;
                                let a1868 = 1u64;
                                nat_sub(a1867, a1868)
                            };
                            let a1870 = 2u64;
                            let a1871 = 0u64;
                            nat_quot(a1869, a1870, a1871)
                        };
                        nat_add(a1872, a1873)?
                    }, v2.clone())?, 1u64, 0u64, 3u64, f113({
                        let a1882 = {
                            let a1875 = v4;
                            let a1876 = match v1 {
                                Adt3::C0(h1874, _, _, _, _, _, _, _) => {
                                    h1874
                                }
                            };
                            nat_mul(a1875, a1876)?
                        };
                        let a1883 = {
                            let a1879 = {
                                let a1877 = v10;
                                let a1878 = 1u64;
                                nat_sub(a1877, a1878)
                            };
                            let a1880 = 2u64;
                            let a1881 = 0u64;
                            nat_quot(a1879, a1880, a1881)
                        };
                        nat_add(a1882, a1883)?
                    }, v2)?))
                }
            }
        }
    }
}

pub fn f67(v0: u64, v1: u64, v2: Adt3, v3: Adt3, v4: Adt3, v5: u64) -> R<Adt9> {
    let a1884 = v1;
    let a1885 = f128(v2.clone(), v4.clone())?;
    if nat_lt(a1884, a1885) {
        let a1888 = v1;
        let a1889 = {
            let a1886 = 2u64;
            let a1887 = f102(v4.clone())?;
            nat_mul(a1886, a1887)?
        };
        if nat_lt(a1888, a1889) {
            let a1893 = {
                let a1890 = v1;
                let a1891 = 2u64;
                let a1892 = 0u64;
                nat_rem(a1890, a1891, a1892)
            };
            let a1894 = 0u64;
            if nat_eq(a1893, a1894) {
                f129({
                    let a1895 = v1;
                    let a1896 = 2u64;
                    let a1897 = 0u64;
                    nat_quot(a1895, a1896, a1897)
                }, v4)
            } else {
                if f130(match v2 {
                    Adt3::C0(_, h1898, _, _, _, _, _, _) => {
                        h1898
                    }
                }, v5) {
                    Ok(Adt9::C4(19u64, 1u64, 2u64, 2u64, 0u64, 1u64, 2u64))
                } else {
                    Ok(Adt9::C0(Adt10::C1))
                }
            }
        } else {
            let v6: u64 = {
                let a1901 = v1;
                let a1902 = {
                    let a1899 = 2u64;
                    let a1900 = f102(v4.clone())?;
                    nat_mul(a1899, a1900)?
                };
                nat_sub(a1901, a1902)
            };
            let v7: u64 = {
                let a1906 = v6;
                let a1907 = {
                    let a1904 = 1u64;
                    let a1905 = match v2.clone() {
                        Adt3::C0(h1903, _, _, _, _, _, _, _) => {
                            h1903
                        }
                    };
                    nat_add(a1904, a1905)?
                };
                let a1908 = 0u64;
                nat_quot(a1906, a1907, a1908)
            };
            let v8: u64 = {
                let a1912 = v6;
                let a1913 = {
                    let a1910 = 1u64;
                    let a1911 = match v2.clone() {
                        Adt3::C0(h1909, _, _, _, _, _, _, _) => {
                            h1909
                        }
                    };
                    nat_add(a1910, a1911)?
                };
                let a1914 = 0u64;
                nat_rem(a1912, a1913, a1914)
            };
            let a1915 = v8;
            let a1916 = 0u64;
            if nat_eq(a1915, a1916) {
                f129(v7, v4)
            } else {
                f131(v0, v7, {
                    let a1917 = v8;
                    let a1918 = 1u64;
                    nat_sub(a1917, a1918)
                }, v2, v3, v4, v5)
            }
        }
    } else {
        Ok(Adt9::C0(Adt10::C1))
    }
}

pub fn f68(v0: u64, v1: Adt3, v2: Adt3, v3: Adt3, v4: Adt3, v5: u64) -> R<Adt9> {
    let v6: u64 = {
        let a1921 = match v4.clone() {
            Adt3::C0(h1919, _, _, _, _, _, _, _) => {
                h1919
            }
        };
        let a1922 = match v4.clone() {
            Adt3::C0(_, h1920, _, _, _, _, _, _) => {
                h1920
            }
        };
        nat_mul(a1921, a1922)?
    };
    let v7: u64 = {
        let a1923 = 2u64;
        let a1924 = v6;
        nat_mul(a1923, a1924)?
    };
    let v8: u64 = f132(match v3.clone() {
        Adt3::C0(_, h1925, _, _, _, _, _, _) => {
            h1925
        }
    }, match v3.clone() {
        Adt3::C0(_, _, h1926, _, _, _, _, _) => {
            h1926
        }
    }, match v3.clone() {
        Adt3::C0(_, _, _, h1927, _, _, _, _) => {
            h1927
        }
    });
    let a1928 = v0;
    let a1929 = v7;
    if nat_lt(a1928, a1929) {
        let a1933 = {
            let a1930 = v0;
            let a1931 = 2u64;
            let a1932 = 0u64;
            nat_rem(a1930, a1931, a1932)
        };
        let a1934 = 0u64;
        if nat_eq(a1933, a1934) {
            Ok(Adt9::C4(19u64, 5u64, f113({
                let a1935 = v0;
                let a1936 = 2u64;
                let a1937 = 0u64;
                nat_quot(a1935, a1936, a1937)
            }, v4)?, 2u64, 0u64, 1u64, 2u64))
        } else {
            if f133(v5, v8) {
                Ok(Adt9::C4(19u64, 1u64, 2u64, 2u64, 0u64, 1u64, 2u64))
            } else {
                Ok(Adt9::C0(Adt10::C1))
            }
        }
    } else {
        let v9: u64 = {
            let a1938 = v0;
            let a1939 = v7;
            nat_sub(a1938, a1939)
        };
        let v10: u64 = {
            let a1943 = v9;
            let a1944 = {
                let a1941 = 1u64;
                let a1942 = match v1.clone() {
                    Adt3::C0(h1940, _, _, _, _, _, _, _) => {
                        h1940
                    }
                };
                nat_add(a1941, a1942)?
            };
            let a1945 = 0u64;
            nat_quot(a1943, a1944, a1945)
        };
        let v11: u64 = {
            let a1949 = v9;
            let a1950 = {
                let a1947 = 1u64;
                let a1948 = match v1.clone() {
                    Adt3::C0(h1946, _, _, _, _, _, _, _) => {
                        h1946
                    }
                };
                nat_add(a1947, a1948)?
            };
            let a1951 = 0u64;
            nat_rem(a1949, a1950, a1951)
        };
        let a1952 = v11;
        let a1953 = 0u64;
        if nat_eq(a1952, a1953) {
            Ok(Adt9::C4(19u64, 5u64, f113({
                let a1954 = v10;
                let a1955 = v6;
                let a1956 = 0u64;
                nat_rem(a1954, a1955, a1956)
            }, v4)?, 2u64, 0u64, 1u64, 2u64))
        } else {
            if f133(v5, v8) {
                let v12: u64 = {
                    let a1962 = {
                        let a1958 = v10;
                        let a1959 = match v1.clone() {
                            Adt3::C0(h1957, _, _, _, _, _, _, _) => {
                                h1957
                            }
                        };
                        nat_mul(a1958, a1959)?
                    };
                    let a1963 = {
                        let a1960 = v11;
                        let a1961 = 1u64;
                        nat_sub(a1960, a1961)
                    };
                    nat_add(a1962, a1963)?
                };
                let a1964 = v5;
                let a1965 = 4294967295u64;
                if nat_eq(a1964, a1965) {
                    Ok(Adt9::C1(false, f113(v12, v1)?, f113(v12, v2)?))
                } else {
                    Ok(Adt9::C4(8u64, 0u64, f113(v12, v1)?, 4u64, f113({
                        let a1971 = {
                            let a1967 = v5;
                            let a1968 = match v3.clone() {
                                Adt3::C0(h1966, _, _, _, _, _, _, _) => {
                                    h1966
                                }
                            };
                            nat_mul(a1967, a1968)?
                        };
                        let a1972 = {
                            let a1969 = v11;
                            let a1970 = 1u64;
                            nat_sub(a1969, a1970)
                        };
                        nat_add(a1971, a1972)?
                    }, v3)?, 3u64, f113(v12, v2)?))
                }
            } else {
                Ok(Adt9::C0(Adt10::C1))
            }
        }
    }
}

pub fn f69(v0: u64, v1: Adt3, v2: Adt3, v3: Adt7, v4: u64) -> R<Adt9> {
    let v5: u64 = f86(v3.clone())?;
    let v6: u64 = {
        let a1973 = v0;
        let a1974 = v5;
        let a1975 = 0u64;
        nat_quot(a1973, a1974, a1975)
    };
    let v7: u64 = {
        let a1976 = v0;
        let a1977 = v5;
        let a1978 = 0u64;
        nat_rem(a1976, a1977, a1978)
    };
    let v8: u64 = f134(v3.clone())?;
    let a1979 = v7;
    let a1980 = 0u64;
    if nat_eq(a1979, a1980) {
        Ok(Adt9::C4(19u64, 2u64, if match v3 {
            Adt7::C0(_, _, _, h1981, _) => {
                h1981
            }
        } {
            4286578687u64
        } else {
            0u64
        }, 2u64, 0u64, 1u64, 0u64))
    } else {
        let a1985 = v5;
        let a1986 = {
            let a1983 = v7;
            let a1984 = if match v3.clone() {
                Adt7::C0(_, _, _, h1982, _) => {
                    h1982
                }
            } {
                1u64
            } else {
                2u64
            };
            nat_add(a1983, a1984)?
        };
        if nat_le(a1985, a1986) {
            if match v3.clone() {
                Adt7::C0(_, _, _, h1987, _) => {
                    h1987
                }
            } {
                Ok(Adt9::C4(19u64, 1u64, 0u64, 2u64, 0u64, 3u64, f113(v6, v2)?))
            } else {
                let a1990 = {
                    let a1988 = v7;
                    let a1989 = 2u64;
                    nat_add(a1988, a1989)?
                };
                let a1991 = v5;
                if nat_eq(a1990, a1991) {
                    let v9: u64 = if match v3.clone() {
                        Adt7::C0(_, _, h1992, _, _) => {
                            h1992
                        }
                    } {
                        let a1999 = {
                            let m1995 = match v3.clone() {
                                Adt7::C0(h1994, _, _, _, _) => {
                                    h1994
                                }
                            };
                            match m1995 {
                                Adt6::C0(h1993, _, _, _) => {
                                    h1993
                                }
                            }
                        };
                        let a2000 = {
                            let m1998 = match v3 {
                                Adt7::C0(_, h1997, _, _, _) => {
                                    h1997
                                }
                            };
                            match m1998 {
                                Adt6::C0(h1996, _, _, _) => {
                                    h1996
                                }
                            }
                        };
                        nat_mul(a1999, a2000)?
                    } else {
                        f135(match v3 {
                            Adt7::C0(h2001, _, _, _, _) => {
                                h2001
                            }
                        }, match v1 {
                            Adt3::C0(h2002, _, _, _, _, _, _, _) => {
                                h2002
                            }
                        }, f59(v6, v2, 0u64))?
                    };
                    Ok(Adt9::C4(3u64, 2u64, {
                        let a2003 = v9;
                        let a2004 = 0u64;
                        if nat_eq(a2003, a2004) {
                            1u64
                        } else {
                            v9
                        }
                    }, 2u64, 0u64, 1u64, 1u64))
                } else {
                    Ok(Adt9::C4(5u64, 1u64, 0u64, 1u64, 1u64, 3u64, f113(v6, v2)?))
                }
            }
        } else {
            let v10: u64 = {
                let a2007 = {
                    let a2005 = v7;
                    let a2006 = 1u64;
                    nat_sub(a2005, a2006)
                };
                let a2008 = v8;
                let a2009 = 0u64;
                nat_rem(a2007, a2008, a2009)
            };
            let m2015 = f136(v3.clone(), v1, v2, v6, {
                let a2012 = {
                    let a2010 = v7;
                    let a2011 = 1u64;
                    nat_sub(a2010, a2011)
                };
                let a2013 = v8;
                let a2014 = 0u64;
                nat_quot(a2012, a2013, a2014)
            })?;
            match m2015 {
                None => {
                    Ok(Adt9::C4(19u64, 1u64, 0u64, 2u64, 0u64, 1u64, 0u64))
                }
                Some(v11) => {
                    let a2017 = match v3.clone() {
                        Adt7::C0(_, _, _, _, h2016) => {
                            h2016
                        }
                    };
                    let a2018 = 1u64;
                    if if nat_eq(a2017, a2018) {
                        let a2019 = v10;
                        let a2020 = 0u64;
                        nat_eq(a2019, a2020)
                    } else {
                        false
                    } {
                        Ok(Adt9::C4(6u64, 0u64, v11, 2u64, 0u64, 1u64, 1u64))
                    } else {
                        if match v3.clone() {
                            Adt7::C0(_, _, _, h2021, _) => {
                                h2021
                            }
                        } {
                            let a2024 = {
                                let a2022 = v10;
                                let a2023 = 1u64;
                                nat_add(a2022, a2023)?
                            };
                            let a2025 = v8;
                            if nat_eq(a2024, a2025) {
                                let a2026 = v4;
                                let a2027 = 2u64;
                                if nat_eq(a2026, a2027) {
                                    Ok(Adt9::C4(19u64, 1u64, 0u64, 2u64, 0u64, 1u64, 0u64))
                                } else {
                                    Ok(Adt9::C4(19u64, {
                                        let a2029 = match v3.clone() {
                                            Adt7::C0(_, _, _, _, h2028) => {
                                                h2028
                                            }
                                        };
                                        let a2030 = 1u64;
                                        if nat_eq(a2029, a2030) {
                                            1u64
                                        } else {
                                            0u64
                                        }
                                    }, {
                                        let a2032 = match v3 {
                                            Adt7::C0(_, _, _, _, h2031) => {
                                                h2031
                                            }
                                        };
                                        let a2033 = 1u64;
                                        if nat_eq(a2032, a2033) {
                                            1u64
                                        } else {
                                            v11
                                        }
                                    }, 2u64, 0u64, 1u64, 0u64))
                                }
                            } else {
                                Ok(Adt9::C4(16u64, 1u64, 0u64, {
                                    let a2035 = match v3.clone() {
                                        Adt7::C0(_, _, _, _, h2034) => {
                                            h2034
                                        }
                                    };
                                    let a2036 = 1u64;
                                    if nat_eq(a2035, a2036) {
                                        1u64
                                    } else {
                                        0u64
                                    }
                                }, {
                                    let a2038 = match v3 {
                                        Adt7::C0(_, _, _, _, h2037) => {
                                            h2037
                                        }
                                    };
                                    let a2039 = 1u64;
                                    if nat_eq(a2038, a2039) {
                                        1u64
                                    } else {
                                        v11
                                    }
                                }, 1u64, 2u64))
                            }
                        } else {
                            Ok(Adt9::C4(4u64, 1u64, 0u64, {
                                let a2041 = match v3.clone() {
                                    Adt7::C0(_, _, _, _, h2040) => {
                                        h2040
                                    }
                                };
                                let a2042 = 1u64;
                                if nat_eq(a2041, a2042) {
                                    1u64
                                } else {
                                    0u64
                                }
                            }, {
                                let a2044 = match v3 {
                                    Adt7::C0(_, _, _, _, h2043) => {
                                        h2043
                                    }
                                };
                                let a2045 = 1u64;
                                if nat_eq(a2044, a2045) {
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

pub fn f70(v0: u64, v1: Adt3, v2: Adt3, v3: Adt5) -> R<Adt9> {
    let v4: u64 = f59(v0, v2.clone(), 0u64);
    let v5: u64 = f59(v0, v2.clone(), 3u64);
    let v6: u64 = {
        let a2047 = v5;
        let a2048 = match v3.clone() {
            Adt5::C0(_, _, _, _, _, _, h2046, _) => {
                h2046
            }
        };
        let a2049 = 0u64;
        nat_rem(a2047, a2048, a2049)
    };
    let v7: u64 = {
        let a2051 = v5;
        let a2052 = match v3.clone() {
            Adt5::C0(_, _, _, _, _, _, h2050, _) => {
                h2050
            }
        };
        let a2053 = 0u64;
        nat_quot(a2051, a2052, a2053)
    };
    let v8: Option<u64> = f61(match v3.clone() {
        Adt5::C0(h2054, _, _, _, _, _, _, _) => {
            h2054
        }
    }, match v1.clone() {
        Adt3::C0(h2055, _, _, _, _, _, _, _) => {
            h2055
        }
    }, f59(v0, v2.clone(), 1u64), {
        let a2059 = v4;
        let a2060 = {
            let m2058 = match v3.clone() {
                Adt5::C0(h2057, _, _, _, _, _, _, _) => {
                    h2057
                }
            };
            match m2058 {
                Adt6::C0(h2056, _, _, _) => {
                    h2056
                }
            }
        };
        let a2061 = 0u64;
        nat_rem(a2059, a2060, a2061)
    })?;
    let v9: u64 = {
        let a2065 = v4;
        let a2066 = {
            let m2064 = match v3.clone() {
                Adt5::C0(h2063, _, _, _, _, _, _, _) => {
                    h2063
                }
            };
            match m2064 {
                Adt6::C0(h2062, _, _, _) => {
                    h2062
                }
            }
        };
        let a2067 = 0u64;
        nat_quot(a2065, a2066, a2067)
    };
    let v10: Option<u64> = f61(match v3.clone() {
        Adt5::C0(_, h2068, _, _, _, _, _, _) => {
            h2068
        }
    }, match v1.clone() {
        Adt3::C0(_, h2069, _, _, _, _, _, _) => {
            h2069
        }
    }, f59(v0, v2.clone(), 2u64), {
        let a2073 = v9;
        let a2074 = {
            let m2072 = match v3.clone() {
                Adt5::C0(_, h2071, _, _, _, _, _, _) => {
                    h2071
                }
            };
            match m2072 {
                Adt6::C0(h2070, _, _, _) => {
                    h2070
                }
            }
        };
        let a2075 = 0u64;
        nat_rem(a2073, a2074, a2075)
    })?;
    let v11: u64 = {
        let a2079 = v9;
        let a2080 = {
            let m2078 = match v3.clone() {
                Adt5::C0(_, h2077, _, _, _, _, _, _) => {
                    h2077
                }
            };
            match m2078 {
                Adt6::C0(h2076, _, _, _) => {
                    h2076
                }
            }
        };
        let a2081 = 0u64;
        nat_quot(a2079, a2080, a2081)
    };
    let v12: Option<u64> = f61(match v3.clone() {
        Adt5::C0(_, _, h2082, _, _, _, _, _) => {
            h2082
        }
    }, match v1.clone() {
        Adt3::C0(_, _, h2083, _, _, _, _, _) => {
            h2083
        }
    }, v6, {
        let a2087 = v11;
        let a2088 = {
            let m2086 = match v3.clone() {
                Adt5::C0(_, _, h2085, _, _, _, _, _) => {
                    h2085
                }
            };
            match m2086 {
                Adt6::C0(h2084, _, _, _) => {
                    h2084
                }
            }
        };
        let a2089 = 0u64;
        nat_rem(a2087, a2088, a2089)
    })?;
    let v13: u64 = {
        let a2093 = v11;
        let a2094 = {
            let m2092 = match v3.clone() {
                Adt5::C0(_, _, h2091, _, _, _, _, _) => {
                    h2091
                }
            };
            match m2092 {
                Adt6::C0(h2090, _, _, _) => {
                    h2090
                }
            }
        };
        let a2095 = 0u64;
        nat_quot(a2093, a2094, a2095)
    };
    let v17: Adt12 = {
        let m2096 = v8;
        match m2096 {
            None => {
                Adt12::C2(0u64)
            }
            Some(v14) => {
                let m2097 = v10;
                match m2097 {
                    None => {
                        Adt12::C2(0u64)
                    }
                    Some(v15) => {
                        let m2098 = v12;
                        match m2098 {
                            None => {
                                Adt12::C2(0u64)
                            }
                            Some(v16) => {
                                Adt12::C0(1u64, {
                                    let a2107 = {
                                        let a2102 = f58(v1, v14, v15, v16, 0u64)?;
                                        let a2103 = {
                                            let a2100 = v7;
                                            let a2101 = match v3.clone() {
                                                Adt5::C0(_, _, _, _, h2099, _, _, _) => {
                                                    h2099
                                                }
                                            };
                                            nat_mul(a2100, a2101)?
                                        };
                                        nat_add(a2102, a2103)?
                                    };
                                    let a2108 = {
                                        let a2105 = v13;
                                        let a2106 = match v3.clone() {
                                            Adt5::C0(_, _, _, h2104, _, _, _, _) => {
                                                h2104
                                            }
                                        };
                                        nat_mul(a2105, a2106)?
                                    };
                                    nat_add(a2107, a2108)?
                                })
                            }
                        }
                    }
                }
            }
        }
    };
    Ok(Adt9::C7(Adt11::C2(match v3.clone() {
        Adt5::C0(_, _, _, _, _, h2109, _, _) => {
            h2109
        }
    }, match v3.clone() {
        Adt5::C0(_, _, _, _, _, _, _, h2110) => {
            h2110
        }
    }, true), v17, Adt12::C2(0u64), Adt13::C1(f113(v0, v2)?), {
        let a2112 = match v3.clone() {
            Adt5::C0(_, _, _, _, _, h2111, _, _) => {
                h2111
            }
        };
        let a2113 = 0u64;
        if nat_eq(a2112, a2113) {
            4u64
        } else {
            2u64
        }
    }, {
        let a2115 = match v3 {
            Adt5::C0(_, _, _, _, _, _, _, h2114) => {
                h2114
            }
        };
        let a2116 = 0u64;
        if nat_eq(a2115, a2116) {
            4u64
        } else {
            2u64
        }
    }))
}

pub fn f71(v0: u64, v1: Adt3, v2: Adt3, v3: u64, v4: u64, v5: u64, v6: u64, v7: u64) -> R<Adt9> {
    let v8: u64 = {
        let a2119 = v6;
        let a2120 = {
            let a2117 = 3u64;
            let a2118 = v5;
            nat_add(a2117, a2118)?
        };
        nat_sub(a2119, a2120)
    };
    let a2122 = v0;
    let a2123 = f29(v3, match v1.clone() {
        Adt3::C0(h2121, _, _, _, _, _, _, _) => {
            h2121
        }
    }, v5, v8)?;
    if nat_lt(a2122, a2123) {
        let v9: u64 = f81(match v1.clone() {
            Adt3::C0(h2124, _, _, _, _, _, _, _) => {
                h2124
            }
        }, v5, v8)?;
        let v10: u64 = f82(v3, match v1.clone() {
            Adt3::C0(h2125, _, _, _, _, _, _, _) => {
                h2125
            }
        }, v5)?;
        let a2126 = v0;
        let a2127 = v9;
        if nat_lt(a2126, a2127) {
            f137(v0, v2, f23(v3), {
                let a2128 = 3u64;
                let a2129 = v5;
                nat_add(a2128, a2129)?
            }, v7)
        } else {
            let a2132 = v0;
            let a2133 = {
                let a2130 = v9;
                let a2131 = v10;
                nat_add(a2130, a2131)?
            };
            if nat_lt(a2132, a2133) {
                let v11: u64 = {
                    let a2134 = v0;
                    let a2135 = v9;
                    nat_sub(a2134, a2135)
                };
                let v12: u64 = f138(v3, match v1.clone() {
                    Adt3::C0(h2136, _, _, _, _, _, _, _) => {
                        h2136
                    }
                })?;
                f139({
                    let a2137 = v11;
                    let a2138 = v12;
                    let a2139 = 0u64;
                    nat_rem(a2137, a2138, a2139)
                }, {
                    let a2140 = v11;
                    let a2141 = v12;
                    let a2142 = 0u64;
                    nat_quot(a2140, a2141, a2142)
                }, v1, v3, v4, v7)
            } else {
                f140({
                    let a2145 = v0;
                    let a2146 = {
                        let a2143 = v9;
                        let a2144 = v10;
                        nat_add(a2143, a2144)?
                    };
                    nat_sub(a2145, a2146)
                }, v1, v2, v3)
            }
        }
    } else {
        Ok(Adt9::C0(Adt10::C1))
    }
}

pub fn f72(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64) -> R<Adt9> {
    let a2147 = v0;
    let a2148 = f15(v1)?;
    if nat_lt(a2147, a2148) {
        let a2149 = v0;
        let a2150 = 4u64;
        if nat_lt(a2149, a2150) {
            let a2151 = v0;
            let a2152 = 0u64;
            if nat_eq(a2151, a2152) {
                Ok(Adt9::C4(16u64, 2u64, v2, 2u64, v3, 1u64, 2u64))
            } else {
                let a2153 = v0;
                let a2154 = 1u64;
                if nat_eq(a2153, a2154) {
                    let a2155 = v5;
                    let a2156 = 0u64;
                    if nat_eq(a2155, a2156) {
                        Ok(Adt9::C4(9u64, 2u64, v3, 2u64, v2, 1u64, 2u64))
                    } else {
                        Ok(Adt9::C0(Adt10::C1))
                    }
                } else {
                    let a2157 = v0;
                    let a2158 = 2u64;
                    if nat_eq(a2157, a2158) {
                        Ok(Adt9::C4(11u64, 1u64, 2u64, 2u64, v4, 1u64, 2u64))
                    } else {
                        Ok(Adt9::C4(20u64, 1u64, 2u64, 2u64, 10u64, 1u64, 2u64))
                    }
                }
            }
        } else {
            let a2159 = v0;
            let a2160 = 4u64;
            if if nat_eq(a2159, a2160) {
                let a2161 = f141(v5)?;
                let a2162 = v1;
                !nat_eq(a2161, a2162)
            } else {
                false
            } {
                Ok(Adt9::C0(Adt10::C1))
            } else {
                let v6: u64 = {
                    let a2165 = {
                        let a2163 = v0;
                        let a2164 = 4u64;
                        nat_sub(a2163, a2164)
                    };
                    let a2166 = 3u64;
                    let a2167 = 0u64;
                    nat_quot(a2165, a2166, a2167)
                };
                let v7: u64 = {
                    let a2170 = {
                        let a2168 = v0;
                        let a2169 = 4u64;
                        nat_sub(a2168, a2169)
                    };
                    let a2171 = 3u64;
                    let a2172 = 0u64;
                    nat_rem(a2170, a2171, a2172)
                };
                let a2173 = v7;
                let a2174 = 0u64;
                if nat_eq(a2173, a2174) {
                    Ok(Adt9::C4(3u64, 2u64, v6, 2u64, 0u64, 1u64, 0u64))
                } else {
                    let a2175 = v7;
                    let a2176 = 1u64;
                    if nat_eq(a2175, a2176) {
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

pub fn f73(v0: bool, v1: u64, v2: u64, v3: u64, v4: u64) -> Adt11 {
    let a2177 = v1;
    let a2178 = 0u64;
    if nat_eq(a2177, a2178) {
        if v0 {
            Adt11::C2(v3, v4, false)
        } else {
            Adt11::C0(v2)
        }
    } else {
        let a2179 = v1;
        let a2180 = 1u64;
        if nat_eq(a2179, a2180) {
            Adt11::C1(v2)
        } else {
            let a2181 = v1;
            let a2182 = 2u64;
            if nat_eq(a2181, a2182) {
                Adt11::C5(v2)
            } else {
                let a2183 = v1;
                let a2184 = 3u64;
                if nat_eq(a2183, a2184) {
                    Adt11::C3(v2, v2, false)
                } else {
                    let a2185 = v1;
                    let a2186 = 4u64;
                    if nat_eq(a2185, a2186) {
                        Adt11::C2(v3, v4, false)
                    } else {
                        Adt11::C6
                    }
                }
            }
        }
    }
}

pub fn f74(v0: u64, v1: u64) -> Adt12 {
    let a2187 = v0;
    let a2188 = 0u64;
    if nat_eq(a2187, a2188) {
        Adt12::C0(0u64, v1)
    } else {
        let a2189 = v0;
        let a2190 = 1u64;
        if nat_eq(a2189, a2190) {
            Adt12::C1(v1)
        } else {
            let a2191 = v0;
            let a2192 = 2u64;
            if nat_eq(a2191, a2192) {
                Adt12::C2(v1)
            } else {
                let a2193 = v0;
                let a2194 = 3u64;
                if nat_eq(a2193, a2194) {
                    Adt12::C3(v1)
                } else {
                    let a2195 = v0;
                    let a2196 = 13u64;
                    if nat_lt(a2195, a2196) {
                        Adt12::C0({
                            let a2197 = v0;
                            let a2198 = 3u64;
                            nat_sub(a2197, a2198)
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
    let a2199 = v0;
    let a2200 = 1u64;
    if nat_eq(a2199, a2200) {
        Adt13::C0(v1)
    } else {
        let a2201 = v0;
        let a2202 = 3u64;
        if if nat_eq(a2201, a2202) {
            true
        } else {
            let a2203 = v0;
            let a2204 = 6u64;
            nat_eq(a2203, a2204)
        } {
            Adt13::C1(v1)
        } else {
            Adt13::C2
        }
    }
}

pub fn f76(v0: Adt6, v1: u64) -> R<u64> {
    let a2206 = 0u64;
    let a2207 = match v0.clone() {
        Adt6::C0(h2205, _, _, _) => {
            h2205
        }
    };
    if if if if if nat_lt(a2206, a2207) {
        let a2209 = 0u64;
        let a2210 = match v0.clone() {
            Adt6::C0(_, h2208, _, _) => {
                h2208
            }
        };
        nat_lt(a2209, a2210)
    } else {
        false
    } {
        let a2212 = 0u64;
        let a2213 = match v0.clone() {
            Adt6::C0(_, _, _, h2211) => {
                h2211
            }
        };
        nat_lt(a2212, a2213)
    } else {
        false
    } {
        let a2215 = match v0.clone() {
            Adt6::C0(_, h2214, _, _) => {
                h2214
            }
        };
        let a2216 = 2147483648u64;
        nat_lt(a2215, a2216)
    } else {
        false
    } {
        let a2218 = match v0.clone() {
            Adt6::C0(_, _, _, h2217) => {
                h2217
            }
        };
        let a2219 = 2147483648u64;
        nat_lt(a2218, a2219)
    } else {
        false
    } {
        let v2: u64 = {
            let a2223 = v1;
            let a2224 = {
                let a2221 = 2u64;
                let a2222 = f107(match v0.clone() {
                    Adt6::C0(_, _, h2220, _) => {
                        h2220
                    }
                });
                nat_mul(a2221, a2222)?
            };
            nat_add(a2223, a2224)?
        };
        let v3: u64 = {
            let a2236 = {
                let a2231 = 1u64;
                let a2232 = {
                    let a2229 = match v0.clone() {
                        Adt6::C0(_, _, _, h2225) => {
                            h2225
                        }
                    };
                    let a2230 = {
                        let a2227 = match v0.clone() {
                            Adt6::C0(h2226, _, _, _) => {
                                h2226
                            }
                        };
                        let a2228 = 1u64;
                        nat_sub(a2227, a2228)
                    };
                    nat_mul(a2229, a2230)?
                };
                nat_add(a2231, a2232)?
            };
            let a2237 = {
                let a2234 = 2u64;
                let a2235 = f106(match v0.clone() {
                    Adt6::C0(_, _, h2233, _) => {
                        h2233
                    }
                });
                nat_mul(a2234, a2235)?
            };
            nat_add(a2236, a2237)?
        };
        let a2238 = v3;
        let a2239 = v2;
        if nat_le(a2238, a2239) {
            let a2246 = 1u64;
            let a2247 = {
                let a2243 = {
                    let a2240 = v2;
                    let a2241 = v3;
                    nat_sub(a2240, a2241)
                };
                let a2244 = match v0 {
                    Adt6::C0(_, h2242, _, _) => {
                        h2242
                    }
                };
                let a2245 = 0u64;
                nat_quot(a2243, a2244, a2245)
            };
            nat_add(a2246, a2247)
        } else {
            let a2251 = {
                let a2248 = v3;
                let a2249 = v2;
                nat_sub(a2248, a2249)
            };
            let a2252 = match v0 {
                Adt6::C0(_, h2250, _, _) => {
                    h2250
                }
            };
            if nat_lt(a2251, a2252) {
                Ok(1u64)
            } else {
                Ok(0u64)
            }
        }
    } else {
        Ok(0u64)
    }
}

pub fn f77(v0: u64) -> u64 {
    let v1: u64 = {
        let a2253 = v0;
        let a2254 = 4294967296u64;
        let a2255 = 0u64;
        nat_rem(a2253, a2254, a2255)
    };
    let a2256 = v1;
    let a2257 = 2147483648u64;
    if nat_lt(a2256, a2257) {
        v1
    } else {
        0u64
    }
}

pub fn f78(v0: u64) -> R<u64> {
    let a2260 = 2u64;
    let a2261 = {
        let a2258 = 2u64;
        let a2259 = v0;
        nat_mul(a2258, a2259)?
    };
    nat_add(a2260, a2261)
}

pub fn f79(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64) -> bool {
    if if f142(v0, v3, v6) {
        f142(v1, v4, v6)
    } else {
        false
    } {
        f142(v2, v5, v6)
    } else {
        false
    }
}

pub fn f80(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64) -> R<bool> {
    if if if if if f143(v0, v1, v4, v2, v5, v3, v6)? {
        true
    } else {
        f143(v0, v1, v4, v3, v6, v2, v5)?
    } {
        true
    } else {
        f143(v0, v2, v5, v1, v4, v3, v6)?
    } {
        true
    } else {
        f143(v0, v2, v5, v3, v6, v1, v4)?
    } {
        true
    } else {
        f143(v0, v3, v6, v1, v4, v2, v5)?
    } {
        Ok(true)
    } else {
        f143(v0, v3, v6, v2, v5, v1, v4)
    }
}

pub fn f81(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let a2262 = v2;
    let a2263 = 0u64;
    if nat_eq(a2262, a2263) {
        Ok(0u64)
    } else {
        let a2266 = 2u64;
        let a2267 = {
            let a2264 = v0;
            let a2265 = v1;
            nat_mul(a2264, a2265)?
        };
        nat_mul(a2266, a2267)
    }
}

pub fn f82(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let a2268 = v2;
    let a2269 = f138(v0, v1)?;
    nat_mul(a2268, a2269)
}

pub fn f83(v0: u64) -> u64 {
    let a2270 = v0;
    let a2271 = 30u64;
    if nat_eq(a2270, a2271) {
        3u64
    } else {
        1u64
    }
}

pub fn f84(v0: u64) -> R<u64> {
    let a2274 = 5u64;
    let a2275 = {
        let a2272 = 4u64;
        let a2273 = v0;
        nat_mul(a2272, a2273)?
    };
    nat_add(a2274, a2275)
}

pub fn f85(v0: Adt6, v1: u64, v2: u64) -> R<bool> {
    let v3: u64 = {
        let a2279 = {
            let a2276 = v2;
            let a2277 = 1u64;
            nat_sub(a2276, a2277)
        };
        let a2280 = match v0.clone() {
            Adt6::C0(_, h2278, _, _) => {
                h2278
            }
        };
        nat_mul(a2279, a2280)?
    };
    let a2282 = 0u64;
    let a2283 = match v0.clone() {
        Adt6::C0(h2281, _, _, _) => {
            h2281
        }
    };
    if if if if if if if if nat_lt(a2282, a2283) {
        let a2285 = match v0.clone() {
            Adt6::C0(h2284, _, _, _) => {
                h2284
            }
        };
        let a2286 = 2147483648u64;
        nat_lt(a2285, a2286)
    } else {
        false
    } {
        let a2288 = 0u64;
        let a2289 = match v0.clone() {
            Adt6::C0(_, h2287, _, _) => {
                h2287
            }
        };
        nat_lt(a2288, a2289)
    } else {
        false
    } {
        let a2291 = match v0.clone() {
            Adt6::C0(_, h2290, _, _) => {
                h2290
            }
        };
        let a2292 = 2147483648u64;
        nat_lt(a2291, a2292)
    } else {
        false
    } {
        let a2293 = v1;
        let a2294 = 2147483648u64;
        nat_lt(a2293, a2294)
    } else {
        false
    } {
        let a2295 = 0u64;
        let a2296 = v2;
        nat_lt(a2295, a2296)
    } else {
        false
    } {
        let a2297 = v3;
        let a2298 = 2147483648u64;
        nat_lt(a2297, a2298)
    } else {
        false
    } {
        let a2310 = {
            let a2305 = {
                let a2302 = v3;
                let a2303 = {
                    let a2300 = match v0.clone() {
                        Adt6::C0(h2299, _, _, _) => {
                            h2299
                        }
                    };
                    let a2301 = 1u64;
                    nat_sub(a2300, a2301)
                };
                nat_add(a2302, a2303)?
            };
            let a2306 = f106(match v0.clone() {
                Adt6::C0(_, _, h2304, _) => {
                    h2304
                }
            });
            nat_add(a2305, a2306)?
        };
        let a2311 = {
            let a2308 = 2147483647u64;
            let a2309 = f107(match v0.clone() {
                Adt6::C0(_, _, h2307, _) => {
                    h2307
                }
            });
            nat_add(a2308, a2309)?
        };
        nat_le(a2310, a2311)
    } else {
        false
    } {
        let a2313 = match v0 {
            Adt6::C0(_, _, h2312, _) => {
                h2312
            }
        };
        let a2314 = 2147483648u64;
        Ok(!nat_eq(a2313, a2314))
    } else {
        Ok(false)
    }
}

pub fn f86(v0: Adt7) -> R<u64> {
    let a2326 = if match v0.clone() {
        Adt7::C0(_, _, _, h2315, _) => {
            h2315
        }
    } {
        2u64
    } else {
        3u64
    };
    let a2327 = {
        let a2324 = {
            let a2322 = {
                let m2318 = match v0.clone() {
                    Adt7::C0(h2317, _, _, _, _) => {
                        h2317
                    }
                };
                match m2318 {
                    Adt6::C0(h2316, _, _, _) => {
                        h2316
                    }
                }
            };
            let a2323 = {
                let m2321 = match v0.clone() {
                    Adt7::C0(_, h2320, _, _, _) => {
                        h2320
                    }
                };
                match m2321 {
                    Adt6::C0(h2319, _, _, _) => {
                        h2319
                    }
                }
            };
            nat_mul(a2322, a2323)?
        };
        let a2325 = f134(v0)?;
        nat_mul(a2324, a2325)?
    };
    nat_add(a2326, a2327)
}

pub fn f87(v0: Adt2, v1: Adt2, v2: Adt2) -> R<Adt10> {
    if f144(match v0.clone() {
        Adt2::C0(h2328, _, _, _, _, _, _, _, _, _, _) => {
            h2328
        }
    }, match v0.clone() {
        Adt2::C0(_, h2329, _, _, _, _, _, _, _, _, _) => {
            h2329
        }
    }, match v0.clone() {
        Adt2::C0(_, _, h2330, _, _, _, _, _, _, _, _) => {
            h2330
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, h2331, _, _, _, _, _, _, _) => {
            h2331
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, h2332, _, _, _, _, _, _) => {
            h2332
        }
    }, match v1.clone() {
        Adt2::C0(h2333, _, _, _, _, _, _, _, _, _, _) => {
            h2333
        }
    }, match v1.clone() {
        Adt2::C0(_, h2334, _, _, _, _, _, _, _, _, _) => {
            h2334
        }
    }, match v1.clone() {
        Adt2::C0(_, _, h2335, _, _, _, _, _, _, _, _) => {
            h2335
        }
    }, match v1.clone() {
        Adt2::C0(_, _, _, h2336, _, _, _, _, _, _, _) => {
            h2336
        }
    }, match v1.clone() {
        Adt2::C0(_, _, _, _, h2337, _, _, _, _, _, _) => {
            h2337
        }
    }, match v2.clone() {
        Adt2::C0(h2338, _, _, _, _, _, _, _, _, _, _) => {
            h2338
        }
    }, match v2.clone() {
        Adt2::C0(_, h2339, _, _, _, _, _, _, _, _, _) => {
            h2339
        }
    }, match v2.clone() {
        Adt2::C0(_, _, h2340, _, _, _, _, _, _, _, _) => {
            h2340
        }
    }, match v2.clone() {
        Adt2::C0(_, _, _, h2341, _, _, _, _, _, _, _) => {
            h2341
        }
    }, match v2.clone() {
        Adt2::C0(_, _, _, _, h2342, _, _, _, _, _, _) => {
            h2342
        }
    }) {
        let m2344 = f91(match v0.clone() {
            Adt2::C0(h2343, _, _, _, _, _, _, _, _, _, _) => {
                h2343
            }
        });
        match m2344 {
            Adt14::C0 => {
                Ok(Adt10::C0)
            }
            Adt14::C1(v3) => {
                let a2350 = {
                    let a2347 = match v0.clone() {
                        Adt2::C0(_, h2345, _, _, _, _, _, _, _, _, _) => {
                            h2345
                        }
                    };
                    let a2348 = match v3.clone() {
                        Adt15::C0(_, h2346, _) => {
                            h2346
                        }
                    };
                    let a2349 = 0u64;
                    nat_rem(a2347, a2348, a2349)
                };
                let a2351 = 0u64;
                if if nat_eq(a2350, a2351) {
                    if f145(match v0.clone() {
                        Adt2::C0(_, h2352, _, _, _, _, _, _, _, _, _) => {
                            h2352
                        }
                    }, match v1.clone() {
                        Adt2::C0(_, h2353, _, _, _, _, _, _, _, _, _) => {
                            h2353
                        }
                    }, match v1.clone() {
                        Adt2::C0(_, _, h2354, _, _, _, _, _, _, _, _) => {
                            h2354
                        }
                    }, match v1.clone() {
                        Adt2::C0(_, _, _, h2355, _, _, _, _, _, _, _) => {
                            h2355
                        }
                    }) {
                        true
                    } else {
                        let a2357 = 4u64;
                        let a2358 = match v2.clone() {
                            Adt2::C0(_, _, _, _, _, h2356, _, _, _, _, _) => {
                                h2356
                            }
                        };
                        if if nat_le(a2357, a2358) {
                            let a2360 = match v3.clone() {
                                Adt15::C0(_, h2359, _) => {
                                    h2359
                                }
                            };
                            let a2361 = 1u64;
                            if nat_eq(a2360, a2361) {
                                true
                            } else {
                                let a2364 = match v3.clone() {
                                    Adt15::C0(_, _, h2362) => {
                                        h2362
                                    }
                                };
                                let a2365 = match v0.clone() {
                                    Adt2::C0(_, _, _, _, _, h2363, _, _, _, _, _) => {
                                        h2363
                                    }
                                };
                                nat_le(a2364, a2365)
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
                        Adt15::C0(_, h2366, _) => {
                            h2366
                        }
                    });
                    Ok(Adt10::C4(if f145(match v0.clone() {
                        Adt2::C0(_, h2367, _, _, _, _, _, _, _, _, _) => {
                            h2367
                        }
                    }, match v1.clone() {
                        Adt2::C0(_, h2368, _, _, _, _, _, _, _, _, _) => {
                            h2368
                        }
                    }, match v1.clone() {
                        Adt2::C0(_, _, h2369, _, _, _, _, _, _, _, _) => {
                            h2369
                        }
                    }, match v1.clone() {
                        Adt2::C0(_, _, _, h2370, _, _, _, _, _, _, _) => {
                            h2370
                        }
                    }) {
                        0u64
                    } else {
                        f128(v4.clone(), f16(v1.clone(), 1u64))?
                    }, 4u64, v4, f16(v2, 1u64), Adt4::C20(match v0 {
                        Adt2::C0(h2371, _, _, _, _, _, _, _, _, _, _) => {
                            h2371
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

pub fn f88(v0: Adt2, v1: Adt2, v2: Adt2) -> R<Adt10> {
    if if if if if if if if if if if if f90(match v0.clone() {
        Adt2::C0(h2372, _, _, _, _, _, _, _, _, _, _) => {
            h2372
        }
    }) {
        let a2374 = match v1.clone() {
            Adt2::C0(h2373, _, _, _, _, _, _, _, _, _, _) => {
                h2373
            }
        };
        let a2375 = 26u64;
        nat_eq(a2374, a2375)
    } else {
        false
    } {
        let a2377 = match v2.clone() {
            Adt2::C0(h2376, _, _, _, _, _, _, _, _, _, _) => {
                h2376
            }
        };
        let a2378 = 0u64;
        nat_eq(a2377, a2378)
    } else {
        false
    } {
        let a2381 = match v0.clone() {
            Adt2::C0(_, h2379, _, _, _, _, _, _, _, _, _) => {
                h2379
            }
        };
        let a2382 = match v2.clone() {
            Adt2::C0(_, h2380, _, _, _, _, _, _, _, _, _) => {
                h2380
            }
        };
        nat_eq(a2381, a2382)
    } else {
        false
    } {
        let a2385 = match v1.clone() {
            Adt2::C0(_, h2383, _, _, _, _, _, _, _, _, _) => {
                h2383
            }
        };
        let a2386 = match v0.clone() {
            Adt2::C0(_, _, h2384, _, _, _, _, _, _, _, _) => {
                h2384
            }
        };
        nat_le(a2385, a2386)
    } else {
        false
    } {
        let a2388 = match v0.clone() {
            Adt2::C0(_, _, _, h2387, _, _, _, _, _, _, _) => {
                h2387
            }
        };
        let a2389 = 1u64;
        nat_eq(a2388, a2389)
    } else {
        false
    } {
        let a2391 = match v0.clone() {
            Adt2::C0(_, _, _, _, h2390, _, _, _, _, _, _) => {
                h2390
            }
        };
        let a2392 = 1u64;
        nat_eq(a2391, a2392)
    } else {
        false
    } {
        let a2394 = match v2.clone() {
            Adt2::C0(_, _, _, h2393, _, _, _, _, _, _, _) => {
                h2393
            }
        };
        let a2395 = 1u64;
        nat_eq(a2394, a2395)
    } else {
        false
    } {
        let a2397 = match v2.clone() {
            Adt2::C0(_, _, _, _, h2396, _, _, _, _, _, _) => {
                h2396
            }
        };
        let a2398 = 1u64;
        nat_eq(a2397, a2398)
    } else {
        false
    } {
        let a2400 = match v1.clone() {
            Adt2::C0(_, _, h2399, _, _, _, _, _, _, _, _) => {
                h2399
            }
        };
        let a2401 = 1u64;
        nat_eq(a2400, a2401)
    } else {
        false
    } {
        let a2403 = match v1.clone() {
            Adt2::C0(_, _, _, h2402, _, _, _, _, _, _, _) => {
                h2402
            }
        };
        let a2404 = 1u64;
        nat_eq(a2403, a2404)
    } else {
        false
    } {
        let a2406 = match v1.clone() {
            Adt2::C0(_, _, _, _, h2405, _, _, _, _, _, _) => {
                h2405
            }
        };
        let a2407 = 1u64;
        nat_eq(a2406, a2407)
    } else {
        false
    } {
        if f27(match v2.clone() {
            Adt2::C0(_, h2408, _, _, _, _, _, _, _, _, _) => {
                h2408
            }
        }, match v2.clone() {
            Adt2::C0(_, _, h2409, _, _, _, _, _, _, _, _) => {
                h2409
            }
        }, match v2.clone() {
            Adt2::C0(_, _, _, h2410, _, _, _, _, _, _, _) => {
                h2410
            }
        }, match v2.clone() {
            Adt2::C0(_, _, _, _, h2411, _, _, _, _, _, _) => {
                h2411
            }
        }) {
            Ok(Adt10::C4(0u64, 4u64, f16(v0, 1u64), f16(v2, 1u64), Adt4::C9(0u64, 0u64)))
        } else {
            let a2413 = match v1.clone() {
                Adt2::C0(_, h2412, _, _, _, _, _, _, _, _, _) => {
                    h2412
                }
            };
            let a2414 = 0u64;
            if nat_eq(a2413, a2414) {
                Ok(Adt10::C4(f12(v2.clone(), 1u64)?, 4u64, f16(v0, 1u64), f16(v2, 1u64), Adt4::C9(0u64, 0u64)))
            } else {
                let a2416 = match v0.clone() {
                    Adt2::C0(_, h2415, _, _, _, _, _, _, _, _, _) => {
                        h2415
                    }
                };
                let a2417 = 2147483647u64;
                if if if if if if nat_le(a2416, a2417) {
                    let a2419 = match v1.clone() {
                        Adt2::C0(_, h2418, _, _, _, _, _, _, _, _, _) => {
                            h2418
                        }
                    };
                    let a2420 = 2147483647u64;
                    nat_le(a2419, a2420)
                } else {
                    false
                } {
                    let a2422 = 4u64;
                    let a2423 = match v2.clone() {
                        Adt2::C0(_, _, _, _, _, h2421, _, _, _, _, _) => {
                            h2421
                        }
                    };
                    nat_le(a2422, a2423)
                } else {
                    false
                } {
                    let a2429 = {
                        let a2426 = match v2.clone() {
                            Adt2::C0(_, h2424, _, _, _, _, _, _, _, _, _) => {
                                h2424
                            }
                        };
                        let a2427 = match v2.clone() {
                            Adt2::C0(_, _, _, _, _, h2425, _, _, _, _, _) => {
                                h2425
                            }
                        };
                        nat_mul(a2426, a2427)?
                    };
                    let a2430 = match v2.clone() {
                        Adt2::C0(_, _, _, _, _, _, h2428, _, _, _, _) => {
                            h2428
                        }
                    };
                    nat_le(a2429, a2430)
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
                        let a2444 = {
                            let a2434 = {
                                let a2432 = 2u64;
                                let a2433 = match v1.clone() {
                                    Adt2::C0(_, h2431, _, _, _, _, _, _, _, _, _) => {
                                        h2431
                                    }
                                };
                                nat_mul(a2432, a2433)?
                            };
                            let a2435 = f12(v2.clone(), 1u64)?;
                            nat_add(a2434, a2435)?
                        };
                        let a2445 = {
                            let a2442 = match v1.clone() {
                                Adt2::C0(_, h2436, _, _, _, _, _, _, _, _, _) => {
                                    h2436
                                }
                            };
                            let a2443 = {
                                let a2440 = 1u64;
                                let a2441 = {
                                    let a2438 = 2u64;
                                    let a2439 = match v0.clone() {
                                        Adt2::C0(_, h2437, _, _, _, _, _, _, _, _, _) => {
                                            h2437
                                        }
                                    };
                                    nat_mul(a2438, a2439)?
                                };
                                nat_add(a2440, a2441)?
                            };
                            nat_mul(a2442, a2443)?
                        };
                        nat_add(a2444, a2445)?
                    }, 4u64, Adt3::C0(match v0.clone() {
                        Adt2::C0(_, h2446, _, _, _, _, _, _, _, _, _) => {
                            h2446
                        }
                    }, match v1.clone() {
                        Adt2::C0(_, h2447, _, _, _, _, _, _, _, _, _) => {
                            h2447
                        }
                    }, 1u64, 1u64, match v0.clone() {
                        Adt2::C0(_, _, _, _, _, h2448, _, _, _, _, _) => {
                            h2448
                        }
                    }, match v0.clone() {
                        Adt2::C0(_, _, _, _, _, _, h2449, _, _, _, _) => {
                            h2449
                        }
                    }, match v0.clone() {
                        Adt2::C0(_, _, _, _, _, _, _, h2450, _, _, _) => {
                            h2450
                        }
                    }, match v0.clone() {
                        Adt2::C0(_, _, _, _, _, _, _, _, h2451, _, _) => {
                            h2451
                        }
                    }), f16(v2, 1u64), Adt4::C19(match v0 {
                        Adt2::C0(h2452, _, _, _, _, _, _, _, _, _, _) => {
                            h2452
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

pub fn f89(v0: u64, v1: Adt2, v2: bool, v3: Adt2) -> R<Adt10> {
    let a2455 = match v1.clone() {
        Adt2::C0(h2453, _, _, _, _, _, _, _, _, _, _) => {
            h2453
        }
    };
    let a2456 = match v3.clone() {
        Adt2::C0(h2454, _, _, _, _, _, _, _, _, _, _) => {
            h2454
        }
    };
    if if nat_eq(a2455, a2456) {
        let a2457 = v0;
        let a2458 = 33u64;
        if nat_eq(a2457, a2458) {
            v2
        } else {
            true
        }
    } else {
        false
    } {
        let m2460 = f91(match v1.clone() {
            Adt2::C0(h2459, _, _, _, _, _, _, _, _, _, _) => {
                h2459
            }
        });
        match m2460 {
            Adt14::C0 => {
                Ok(Adt10::C0)
            }
            Adt14::C1(v4) => {
                let v5: u64 = match v4.clone() {
                    Adt15::C0(_, h2461, _) => {
                        h2461
                    }
                };
                let v6: u64 = f12(v1.clone(), v5)?;
                let a2466 = {
                    let a2463 = match v1.clone() {
                        Adt2::C0(_, h2462, _, _, _, _, _, _, _, _, _) => {
                            h2462
                        }
                    };
                    let a2464 = v5;
                    let a2465 = 1u64;
                    nat_rem(a2463, a2464, a2465)
                };
                let a2467 = 0u64;
                if if if nat_eq(a2466, a2467) {
                    let a2472 = {
                        let a2469 = match v3.clone() {
                            Adt2::C0(_, h2468, _, _, _, _, _, _, _, _, _) => {
                                h2468
                            }
                        };
                        let a2470 = v5;
                        let a2471 = 1u64;
                        nat_rem(a2469, a2470, a2471)
                    };
                    let a2473 = 0u64;
                    nat_eq(a2472, a2473)
                } else {
                    false
                } {
                    let a2474 = v6;
                    let a2475 = f12(v3.clone(), v5)?;
                    nat_eq(a2474, a2475)
                } else {
                    false
                } {
                    Ok(Adt10::C4(v6, match v4 {
                        Adt15::C0(_, _, h2476) => {
                            h2476
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

pub fn f90(v0: u64) -> bool {
    let a2477 = v0;
    let a2478 = 0u64;
    if if nat_eq(a2477, a2478) {
        true
    } else {
        let a2479 = v0;
        let a2480 = 1u64;
        nat_eq(a2479, a2480)
    } {
        true
    } else {
        let a2481 = v0;
        let a2482 = 30u64;
        nat_eq(a2481, a2482)
    }
}

pub fn f91(v0: u64) -> Adt14 {
    let a2483 = v0;
    let a2484 = 0u64;
    if nat_eq(a2483, a2484) {
        Adt14::C1(Adt15::C0(0u64, 1u64, 4u64))
    } else {
        let a2485 = v0;
        let a2486 = 1u64;
        if nat_eq(a2485, a2486) {
            Adt14::C1(Adt15::C0(1u64, 1u64, 2u64))
        } else {
            let a2487 = v0;
            let a2488 = 2u64;
            if nat_eq(a2487, a2488) {
                Adt14::C1(Adt15::C0(2u64, 32u64, 18u64))
            } else {
                let a2489 = v0;
                let a2490 = 3u64;
                if nat_eq(a2489, a2490) {
                    Adt14::C1(Adt15::C0(3u64, 32u64, 20u64))
                } else {
                    let a2491 = v0;
                    let a2492 = 6u64;
                    if nat_eq(a2491, a2492) {
                        Adt14::C1(Adt15::C0(6u64, 32u64, 22u64))
                    } else {
                        let a2493 = v0;
                        let a2494 = 7u64;
                        if nat_eq(a2493, a2494) {
                            Adt14::C1(Adt15::C0(7u64, 32u64, 24u64))
                        } else {
                            let a2495 = v0;
                            let a2496 = 8u64;
                            if nat_eq(a2495, a2496) {
                                Adt14::C1(Adt15::C0(8u64, 32u64, 34u64))
                            } else {
                                let a2497 = v0;
                                let a2498 = 9u64;
                                if nat_eq(a2497, a2498) {
                                    Adt14::C1(Adt15::C0(9u64, 32u64, 36u64))
                                } else {
                                    let a2499 = v0;
                                    let a2500 = 10u64;
                                    if nat_eq(a2499, a2500) {
                                        Adt14::C1(Adt15::C0(10u64, 256u64, 84u64))
                                    } else {
                                        let a2501 = v0;
                                        let a2502 = 11u64;
                                        if nat_eq(a2501, a2502) {
                                            Adt14::C1(Adt15::C0(11u64, 256u64, 110u64))
                                        } else {
                                            let a2503 = v0;
                                            let a2504 = 12u64;
                                            if nat_eq(a2503, a2504) {
                                                Adt14::C1(Adt15::C0(12u64, 256u64, 144u64))
                                            } else {
                                                let a2505 = v0;
                                                let a2506 = 13u64;
                                                if nat_eq(a2505, a2506) {
                                                    Adt14::C1(Adt15::C0(13u64, 256u64, 176u64))
                                                } else {
                                                    let a2507 = v0;
                                                    let a2508 = 14u64;
                                                    if nat_eq(a2507, a2508) {
                                                        Adt14::C1(Adt15::C0(14u64, 256u64, 210u64))
                                                    } else {
                                                        let a2509 = v0;
                                                        let a2510 = 15u64;
                                                        if nat_eq(a2509, a2510) {
                                                            Adt14::C1(Adt15::C0(15u64, 256u64, 292u64))
                                                        } else {
                                                            let a2511 = v0;
                                                            let a2512 = 16u64;
                                                            if nat_eq(a2511, a2512) {
                                                                Adt14::C1(Adt15::C0(16u64, 256u64, 66u64))
                                                            } else {
                                                                let a2513 = v0;
                                                                let a2514 = 17u64;
                                                                if nat_eq(a2513, a2514) {
                                                                    Adt14::C1(Adt15::C0(17u64, 256u64, 74u64))
                                                                } else {
                                                                    let a2515 = v0;
                                                                    let a2516 = 18u64;
                                                                    if nat_eq(a2515, a2516) {
                                                                        Adt14::C1(Adt15::C0(18u64, 256u64, 98u64))
                                                                    } else {
                                                                        let a2517 = v0;
                                                                        let a2518 = 19u64;
                                                                        if nat_eq(a2517, a2518) {
                                                                            Adt14::C1(Adt15::C0(19u64, 256u64, 50u64))
                                                                        } else {
                                                                            let a2519 = v0;
                                                                            let a2520 = 20u64;
                                                                            if nat_eq(a2519, a2520) {
                                                                                Adt14::C1(Adt15::C0(20u64, 32u64, 18u64))
                                                                            } else {
                                                                                let a2521 = v0;
                                                                                let a2522 = 21u64;
                                                                                if nat_eq(a2521, a2522) {
                                                                                    Adt14::C1(Adt15::C0(21u64, 256u64, 110u64))
                                                                                } else {
                                                                                    let a2523 = v0;
                                                                                    let a2524 = 22u64;
                                                                                    if nat_eq(a2523, a2524) {
                                                                                        Adt14::C1(Adt15::C0(22u64, 256u64, 82u64))
                                                                                    } else {
                                                                                        let a2525 = v0;
                                                                                        let a2526 = 23u64;
                                                                                        if nat_eq(a2525, a2526) {
                                                                                            Adt14::C1(Adt15::C0(23u64, 256u64, 136u64))
                                                                                        } else {
                                                                                            let a2527 = v0;
                                                                                            let a2528 = 24u64;
                                                                                            if nat_eq(a2527, a2528) {
                                                                                                Adt14::C1(Adt15::C0(24u64, 1u64, 1u64))
                                                                                            } else {
                                                                                                let a2529 = v0;
                                                                                                let a2530 = 25u64;
                                                                                                if nat_eq(a2529, a2530) {
                                                                                                    Adt14::C1(Adt15::C0(25u64, 1u64, 2u64))
                                                                                                } else {
                                                                                                    let a2531 = v0;
                                                                                                    let a2532 = 26u64;
                                                                                                    if nat_eq(a2531, a2532) {
                                                                                                        Adt14::C1(Adt15::C0(26u64, 1u64, 4u64))
                                                                                                    } else {
                                                                                                        let a2533 = v0;
                                                                                                        let a2534 = 27u64;
                                                                                                        if nat_eq(a2533, a2534) {
                                                                                                            Adt14::C1(Adt15::C0(27u64, 1u64, 8u64))
                                                                                                        } else {
                                                                                                            let a2535 = v0;
                                                                                                            let a2536 = 28u64;
                                                                                                            if nat_eq(a2535, a2536) {
                                                                                                                Adt14::C1(Adt15::C0(28u64, 1u64, 8u64))
                                                                                                            } else {
                                                                                                                let a2537 = v0;
                                                                                                                let a2538 = 29u64;
                                                                                                                if nat_eq(a2537, a2538) {
                                                                                                                    Adt14::C1(Adt15::C0(29u64, 256u64, 56u64))
                                                                                                                } else {
                                                                                                                    let a2539 = v0;
                                                                                                                    let a2540 = 30u64;
                                                                                                                    if nat_eq(a2539, a2540) {
                                                                                                                        Adt14::C1(Adt15::C0(30u64, 1u64, 2u64))
                                                                                                                    } else {
                                                                                                                        let a2541 = v0;
                                                                                                                        let a2542 = 34u64;
                                                                                                                        if nat_eq(a2541, a2542) {
                                                                                                                            Adt14::C1(Adt15::C0(34u64, 256u64, 54u64))
                                                                                                                        } else {
                                                                                                                            let a2543 = v0;
                                                                                                                            let a2544 = 35u64;
                                                                                                                            if nat_eq(a2543, a2544) {
                                                                                                                                Adt14::C1(Adt15::C0(35u64, 256u64, 66u64))
                                                                                                                            } else {
                                                                                                                                let a2545 = v0;
                                                                                                                                let a2546 = 39u64;
                                                                                                                                if nat_eq(a2545, a2546) {
                                                                                                                                    Adt14::C1(Adt15::C0(39u64, 32u64, 17u64))
                                                                                                                                } else {
                                                                                                                                    let a2547 = v0;
                                                                                                                                    let a2548 = 40u64;
                                                                                                                                    if nat_eq(a2547, a2548) {
                                                                                                                                        Adt14::C1(Adt15::C0(40u64, 64u64, 36u64))
                                                                                                                                    } else {
                                                                                                                                        let a2549 = v0;
                                                                                                                                        let a2550 = 41u64;
                                                                                                                                        if nat_eq(a2549, a2550) {
                                                                                                                                            Adt14::C1(Adt15::C0(41u64, 128u64, 18u64))
                                                                                                                                        } else {
                                                                                                                                            let a2551 = v0;
                                                                                                                                            let a2552 = 42u64;
                                                                                                                                            if nat_eq(a2551, a2552) {
                                                                                                                                                Adt14::C1(Adt15::C0(42u64, 64u64, 18u64))
                                                                                                                                            } else {
                                                                                                                                                let a2553 = v0;
                                                                                                                                                let a2554 = 43u64;
                                                                                                                                                if nat_eq(a2553, a2554) {
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

pub fn f92(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64, v7: u64) -> bool {
    if f27(v4, v5, v6, v7) {
        f27(v0, v1, v2, v3)
    } else {
        let a2558 = {
            let a2555 = v0;
            let a2556 = v4;
            let a2557 = 1u64;
            nat_rem(a2555, a2556, a2557)
        };
        let a2559 = 0u64;
        if if if nat_eq(a2558, a2559) {
            let a2563 = {
                let a2560 = v1;
                let a2561 = v5;
                let a2562 = 1u64;
                nat_rem(a2560, a2561, a2562)
            };
            let a2564 = 0u64;
            nat_eq(a2563, a2564)
        } else {
            false
        } {
            let a2568 = {
                let a2565 = v2;
                let a2566 = v6;
                let a2567 = 1u64;
                nat_rem(a2565, a2566, a2567)
            };
            let a2569 = 0u64;
            nat_eq(a2568, a2569)
        } else {
            false
        } {
            let a2573 = {
                let a2570 = v3;
                let a2571 = v7;
                let a2572 = 1u64;
                nat_rem(a2570, a2571, a2572)
            };
            let a2574 = 0u64;
            nat_eq(a2573, a2574)
        } else {
            false
        }
    }
}

pub fn f93(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64) -> R<bool> {
    let a2575 = v0;
    let a2576 = v1;
    if nat_eq(a2575, a2576) {
        let a2579 = v4;
        let a2580 = {
            let a2577 = v2;
            let a2578 = v3;
            nat_add(a2577, a2578)?
        };
        Ok(nat_eq(a2579, a2580))
    } else {
        let a2581 = v2;
        let a2582 = v3;
        if nat_eq(a2581, a2582) {
            let a2583 = v4;
            let a2584 = v2;
            Ok(nat_eq(a2583, a2584))
        } else {
            Ok(false)
        }
    }
}

pub fn f94(v0: u64, v1: u64, v2: u64, v3: u64) -> R<bool> {
    let v4: u64 = {
        let a2587 = {
            let a2585 = v0;
            let a2586 = f107(v1);
            nat_add(a2585, a2586)?
        };
        let a2588 = f107(v2);
        nat_add(a2587, a2588)?
    };
    let v5: u64 = {
        let a2589 = f106(v1);
        let a2590 = f106(v2);
        nat_add(a2589, a2590)?
    };
    let a2591 = v1;
    let a2592 = 4294967295u64;
    if if nat_le(a2591, a2592) {
        let a2593 = v2;
        let a2594 = 4294967295u64;
        nat_le(a2593, a2594)
    } else {
        false
    } {
        let a2595 = v5;
        let a2596 = v4;
        if nat_le(a2595, a2596) {
            let a2599 = v3;
            let a2600 = {
                let a2597 = v4;
                let a2598 = v5;
                nat_sub(a2597, a2598)
            };
            Ok(nat_eq(a2599, a2600))
        } else {
            Ok(false)
        }
    } else {
        Ok(false)
    }
}

pub fn f95(v0: u64, v1: u64) -> R<bool> {
    let a2601 = v1;
    let a2602 = 4294967295u64;
    if nat_le(a2601, a2602) {
        let a2605 = {
            let a2603 = f107(v1);
            let a2604 = f106(v1);
            nat_add(a2603, a2604)?
        };
        let a2606 = v0;
        Ok(nat_lt(a2605, a2606))
    } else {
        Ok(false)
    }
}

pub fn f96(v0: Adt3, v1: u64, v2: u64, v3: u64, v4: bool) -> R<bool> {
    let v5: u64 = f146(v0.clone(), false)?;
    let v6: u64 = f146(v0, true)?;
    let a2607 = v6;
    let a2608 = v1;
    if if nat_le(a2607, a2608) {
        let a2615 = {
            let a2613 = {
                let a2609 = v1;
                let a2610 = v5;
                nat_add(a2609, a2610)?
            };
            let a2614 = {
                let a2611 = v3;
                let a2612 = 0u64;
                if nat_eq(a2611, a2612) {
                    0u64
                } else {
                    4u64
                }
            };
            nat_add(a2613, a2614)?
        };
        let a2616 = v2;
        nat_le(a2615, a2616)
    } else {
        false
    } {
        if v4 {
            let a2621 = {
                let a2619 = {
                    let a2617 = v1;
                    let a2618 = v5;
                    nat_add(a2617, a2618)?
                };
                let a2620 = v6;
                nat_sub(a2619, a2620)
            };
            let a2622 = v2;
            Ok(nat_lt(a2621, a2622))
        } else {
            let a2627 = {
                let a2625 = {
                    let a2623 = v1;
                    let a2624 = v5;
                    nat_add(a2623, a2624)?
                };
                let a2626 = v6;
                nat_sub(a2625, a2626)
            };
            let a2628 = v2;
            Ok(nat_le(a2627, a2628))
        }
    } else {
        Ok(false)
    }
}

pub fn f97(v0: u64, v1: u64) -> bool {
    let a2629 = v0;
    let a2630 = 0u64;
    if if nat_eq(a2629, a2630) {
        true
    } else {
        let a2631 = v0;
        let a2632 = 1u64;
        nat_eq(a2631, a2632)
    } {
        let a2633 = v1;
        let a2634 = 4294967296u64;
        nat_lt(a2633, a2634)
    } else {
        false
    }
}

pub fn f98(v0: u64, v1: u64) -> R<u64> {
    let a2639 = {
        let a2637 = v0;
        let a2638 = {
            let a2635 = v1;
            let a2636 = 1u64;
            nat_sub(a2635, a2636)
        };
        nat_add(a2637, a2638)?
    };
    let a2640 = v1;
    let a2641 = 0u64;
    Ok(nat_quot(a2639, a2640, a2641))
}

pub fn f99(v0: Adt2, v1: Adt2, v2: u64, v3: u64) -> R<bool> {
    if f27(match v0.clone() {
        Adt2::C0(_, h2642, _, _, _, _, _, _, _, _, _) => {
            h2642
        }
    }, match v0.clone() {
        Adt2::C0(_, _, h2643, _, _, _, _, _, _, _, _) => {
            h2643
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, h2644, _, _, _, _, _, _, _) => {
            h2644
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, h2645, _, _, _, _, _, _) => {
            h2645
        }
    }) {
        Ok(true)
    } else {
        let v4: u64 = {
            let a2648 = match v1.clone() {
                Adt2::C0(_, h2646, _, _, _, _, _, _, _, _, _) => {
                    h2646
                }
            };
            let a2649 = match v1.clone() {
                Adt2::C0(_, _, h2647, _, _, _, _, _, _, _, _) => {
                    h2647
                }
            };
            nat_mul(a2648, a2649)?
        };
        let v5: u64 = {
            let a2651 = v4;
            let a2652 = match v1.clone() {
                Adt2::C0(_, _, _, h2650, _, _, _, _, _, _, _) => {
                    h2650
                }
            };
            nat_mul(a2651, a2652)?
        };
        let a2665 = {
            let a2663 = {
                let a2654 = match v0.clone() {
                    Adt2::C0(_, h2653, _, _, _, _, _, _, _, _, _) => {
                        h2653
                    }
                };
                let a2655 = 1u64;
                nat_sub(a2654, a2655)
            };
            let a2664 = {
                let a2661 = f147(match v0.clone() {
                    Adt2::C0(_, _, h2656, _, _, _, _, _, _, _, _) => {
                        h2656
                    }
                }, v2, v5, match v1.clone() {
                    Adt2::C0(_, h2657, _, _, _, _, _, _, _, _, _) => {
                        h2657
                    }
                })?;
                let a2662 = f147(match v0 {
                    Adt2::C0(_, _, _, h2658, _, _, _, _, _, _, _) => {
                        h2658
                    }
                }, v2, {
                    let a2659 = v3;
                    let a2660 = v5;
                    nat_mul(a2659, a2660)?
                }, v4)?;
                nat_add(a2661, a2662)?
            };
            nat_add(a2663, a2664)?
        };
        let a2666 = f12(v1, 1u64)?;
        Ok(nat_lt(a2665, a2666))
    }
}

pub fn f100(v0: u64, v1: u64, v2: u64) -> R<bool> {
    let a2667 = v0;
    let a2668 = 2147483648u64;
    if if if if nat_lt(a2667, a2668) {
        let a2669 = v1;
        let a2670 = 2147483648u64;
        nat_lt(a2669, a2670)
    } else {
        false
    } {
        let a2673 = {
            let a2671 = v0;
            let a2672 = v1;
            nat_mul(a2671, a2672)?
        };
        let a2674 = 2147483648u64;
        nat_lt(a2673, a2674)
    } else {
        false
    } {
        let a2679 = {
            let a2677 = {
                let a2675 = v0;
                let a2676 = 1u64;
                nat_sub(a2675, a2676)
            };
            let a2678 = v2;
            nat_mul(a2677, a2678)?
        };
        let a2680 = 2147483648u64;
        nat_lt(a2679, a2680)
    } else {
        false
    } {
        let a2685 = {
            let a2683 = {
                let a2681 = v1;
                let a2682 = 1u64;
                nat_sub(a2681, a2682)
            };
            let a2684 = v2;
            nat_mul(a2683, a2684)?
        };
        let a2686 = 2147483648u64;
        Ok(nat_lt(a2685, a2686))
    } else {
        Ok(false)
    }
}

pub fn f101(v0: u64) -> R<u64> {
    let a2689 = 3u64;
    let a2690 = {
        let a2687 = v0;
        let a2688 = 3u64;
        nat_mul(a2687, a2688)?
    };
    nat_add(a2689, a2690)
}

pub fn f102(v0: Adt3) -> R<u64> {
    if f27(match v0.clone() {
        Adt3::C0(h2691, _, _, _, _, _, _, _) => {
            h2691
        }
    }, match v0.clone() {
        Adt3::C0(_, h2692, _, _, _, _, _, _) => {
            h2692
        }
    }, match v0.clone() {
        Adt3::C0(_, _, h2693, _, _, _, _, _) => {
            h2693
        }
    }, match v0.clone() {
        Adt3::C0(_, _, _, h2694, _, _, _, _) => {
            h2694
        }
    }) {
        Ok(0u64)
    } else {
        let a2703 = match v0.clone() {
            Adt3::C0(h2695, _, _, _, _, _, _, _) => {
                h2695
            }
        };
        let a2704 = {
            let a2701 = match v0.clone() {
                Adt3::C0(_, h2696, _, _, _, _, _, _) => {
                    h2696
                }
            };
            let a2702 = {
                let a2699 = match v0.clone() {
                    Adt3::C0(_, _, h2697, _, _, _, _, _) => {
                        h2697
                    }
                };
                let a2700 = match v0 {
                    Adt3::C0(_, _, _, h2698, _, _, _, _) => {
                        h2698
                    }
                };
                nat_mul(a2699, a2700)?
            };
            nat_mul(a2701, a2702)?
        };
        nat_mul(a2703, a2704)
    }
}

pub fn f103(v0: Adt3, v1: u64, v2: u64) -> R<u64> {
    let a2705 = v1;
    let a2706 = 13u64;
    if if nat_eq(a2705, a2706) {
        f27(match v0.clone() {
            Adt3::C0(h2707, _, _, _, _, _, _, _) => {
                h2707
            }
        }, match v0.clone() {
            Adt3::C0(_, h2708, _, _, _, _, _, _) => {
                h2708
            }
        }, match v0.clone() {
            Adt3::C0(_, _, h2709, _, _, _, _, _) => {
                h2709
            }
        }, match v0.clone() {
            Adt3::C0(_, _, _, h2710, _, _, _, _) => {
                h2710
            }
        })
    } else {
        false
    } {
        Ok(2u64)
    } else {
        let v3: u64 = {
            let a2716 = match v0.clone() {
                Adt3::C0(_, h2711, _, _, _, _, _, _) => {
                    h2711
                }
            };
            let a2717 = {
                let a2714 = match v0.clone() {
                    Adt3::C0(_, _, h2712, _, _, _, _, _) => {
                        h2712
                    }
                };
                let a2715 = match v0.clone() {
                    Adt3::C0(_, _, _, h2713, _, _, _, _) => {
                        h2713
                    }
                };
                nat_mul(a2714, a2715)?
            };
            nat_mul(a2716, a2717)?
        };
        let a2718 = v1;
        let a2719 = 13u64;
        if nat_eq(a2718, a2719) {
            let a2728 = 2u64;
            let a2729 = if f148(v0.clone(), v2) {
                let a2721 = v3;
                let a2722 = f149(match v0 {
                    Adt3::C0(h2720, _, _, _, _, _, _, _) => {
                        h2720
                    }
                }, false)?;
                nat_mul(a2721, a2722)?
            } else {
                let a2726 = v3;
                let a2727 = {
                    let a2724 = match v0 {
                        Adt3::C0(h2723, _, _, _, _, _, _, _) => {
                            h2723
                        }
                    };
                    let a2725 = 2u64;
                    nat_mul(a2724, a2725)?
                };
                nat_mul(a2726, a2727)?
            };
            nat_add(a2728, a2729)
        } else {
            let a2733 = v3;
            let a2734 = f149(match v0 {
                Adt3::C0(h2730, _, _, _, _, _, _, _) => {
                    h2730
                }
            }, {
                let a2731 = v1;
                let a2732 = 16u64;
                nat_eq(a2731, a2732)
            })?;
            nat_mul(a2733, a2734)
        }
    }
}

pub fn f104(v0: u64, v1: u64, v2: Adt2, v3: Option<Adt2>, v4: Adt2) -> R<Adt10> {
    let a2735 = v0;
    let a2736 = 30u64;
    if if nat_eq(a2735, a2736) {
        let a2739 = match v2.clone() {
            Adt2::C0(_, _, _, _, _, _, h2737, _, _, _, _) => {
                h2737
            }
        };
        let a2740 = match v2.clone() {
            Adt2::C0(_, _, _, _, _, h2738, _, _, _, _, _) => {
                h2738
            }
        };
        nat_lt(a2739, a2740)
    } else {
        false
    } {
        Ok(Adt10::C1)
    } else {
        let m2741 = v3;
        match m2741 {
            None => {
                f150(v0, v1, match v2.clone() {
                    Adt2::C0(h2742, _, _, _, _, _, _, _, _, _, _) => {
                        h2742
                    }
                }, 0u64, match v4.clone() {
                    Adt2::C0(h2743, _, _, _, _, _, _, _, _, _, _) => {
                        h2743
                    }
                }, false, match v2.clone() {
                    Adt2::C0(_, h2744, _, _, _, _, _, _, _, _, _) => {
                        h2744
                    }
                }, match v2.clone() {
                    Adt2::C0(_, _, h2745, _, _, _, _, _, _, _, _) => {
                        h2745
                    }
                }, match v2.clone() {
                    Adt2::C0(_, _, _, h2746, _, _, _, _, _, _, _) => {
                        h2746
                    }
                }, match v2 {
                    Adt2::C0(_, _, _, _, h2747, _, _, _, _, _, _) => {
                        h2747
                    }
                }, 1u64, 1u64, 1u64, 1u64, match v4.clone() {
                    Adt2::C0(_, h2748, _, _, _, _, _, _, _, _, _) => {
                        h2748
                    }
                }, match v4.clone() {
                    Adt2::C0(_, _, h2749, _, _, _, _, _, _, _, _) => {
                        h2749
                    }
                }, match v4.clone() {
                    Adt2::C0(_, _, _, h2750, _, _, _, _, _, _, _) => {
                        h2750
                    }
                }, match v4 {
                    Adt2::C0(_, _, _, _, h2751, _, _, _, _, _, _) => {
                        h2751
                    }
                })
            }
            Some(v5) => {
                f150(v0, v1, match v2.clone() {
                    Adt2::C0(h2752, _, _, _, _, _, _, _, _, _, _) => {
                        h2752
                    }
                }, match v5.clone() {
                    Adt2::C0(h2753, _, _, _, _, _, _, _, _, _, _) => {
                        h2753
                    }
                }, match v4.clone() {
                    Adt2::C0(h2754, _, _, _, _, _, _, _, _, _, _) => {
                        h2754
                    }
                }, true, match v2.clone() {
                    Adt2::C0(_, h2755, _, _, _, _, _, _, _, _, _) => {
                        h2755
                    }
                }, match v2.clone() {
                    Adt2::C0(_, _, h2756, _, _, _, _, _, _, _, _) => {
                        h2756
                    }
                }, match v2.clone() {
                    Adt2::C0(_, _, _, h2757, _, _, _, _, _, _, _) => {
                        h2757
                    }
                }, match v2 {
                    Adt2::C0(_, _, _, _, h2758, _, _, _, _, _, _) => {
                        h2758
                    }
                }, match v5.clone() {
                    Adt2::C0(_, h2759, _, _, _, _, _, _, _, _, _) => {
                        h2759
                    }
                }, match v5.clone() {
                    Adt2::C0(_, _, h2760, _, _, _, _, _, _, _, _) => {
                        h2760
                    }
                }, match v5.clone() {
                    Adt2::C0(_, _, _, h2761, _, _, _, _, _, _, _) => {
                        h2761
                    }
                }, match v5 {
                    Adt2::C0(_, _, _, _, h2762, _, _, _, _, _, _) => {
                        h2762
                    }
                }, match v4.clone() {
                    Adt2::C0(_, h2763, _, _, _, _, _, _, _, _, _) => {
                        h2763
                    }
                }, match v4.clone() {
                    Adt2::C0(_, _, h2764, _, _, _, _, _, _, _, _) => {
                        h2764
                    }
                }, match v4.clone() {
                    Adt2::C0(_, _, _, h2765, _, _, _, _, _, _, _) => {
                        h2765
                    }
                }, match v4 {
                    Adt2::C0(_, _, _, _, h2766, _, _, _, _, _, _) => {
                        h2766
                    }
                })
            }
        }
    }
}

pub fn f105(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64, v7: u64) -> R<u64> {
    let a2779 = {
        let a2775 = {
            let a2771 = {
                let a2767 = v0;
                let a2768 = v4;
                nat_mul(a2767, a2768)?
            };
            let a2772 = {
                let a2769 = v1;
                let a2770 = v5;
                nat_mul(a2769, a2770)?
            };
            nat_add(a2771, a2772)?
        };
        let a2776 = {
            let a2773 = v2;
            let a2774 = v6;
            nat_mul(a2773, a2774)?
        };
        nat_add(a2775, a2776)?
    };
    let a2780 = {
        let a2777 = v3;
        let a2778 = v7;
        nat_mul(a2777, a2778)?
    };
    nat_add(a2779, a2780)
}

pub fn f106(v0: u64) -> u64 {
    let a2781 = v0;
    let a2782 = 2147483648u64;
    if nat_lt(a2781, a2782) {
        0u64
    } else {
        let a2783 = 4294967296u64;
        let a2784 = v0;
        nat_sub(a2783, a2784)
    }
}

pub fn f107(v0: u64) -> u64 {
    let a2785 = v0;
    let a2786 = 2147483648u64;
    if nat_lt(a2785, a2786) {
        v0
    } else {
        0u64
    }
}

pub fn f108(v0: Adt11, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64, v7: u64, v8: u64) -> Adt9 {
    let v9: u64 = {
        let a2787 = v7;
        let a2788 = 6u64;
        if nat_eq(a2787, a2788) {
            27u64
        } else {
            v2
        }
    };
    Adt9::C7(v0, f74(v3, v4), f74(v5, v6), f75(v7, v8), f151(v1), f151(v9))
}

pub fn f109(v0: Adt4, v1: u64, v2: u64) -> Adt11 {
    let m2789 = v0;
    match m2789 {
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
            let a2790 = 13u64;
            let a2791 = v34;
            if nat_le(a2790, a2791) {
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
                Adt5::C0(_, _, _, _, _, h2792, _, _) => {
                    h2792
                }
            }, match v51 {
                Adt5::C0(_, _, _, _, _, _, _, h2793) => {
                    h2793
                }
            }, true)
        }
        Adt4::C28(_) => {
            Adt11::C6
        }
    }
}

pub fn f110(v0: Adt4) -> u64 {
    let m2794 = v0;
    match m2794 {
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

pub fn f111(v0: Adt4, v1: u64) -> u64 {
    let m2795 = v0;
    match m2795 {
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
            let a2796 = v1;
            let a2797 = 6u64;
            if nat_eq(a2796, a2797) {
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
            let a2802 = v1;
            let a2803 = 6u64;
            if if if nat_eq(a2802, a2803) {
                true
            } else {
                let a2804 = v1;
                let a2805 = 7u64;
                nat_eq(a2804, a2805)
            } {
                let a2806 = v51;
                let a2807 = 30u64;
                nat_eq(a2806, a2807)
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
                Adt5::C0(_, _, _, _, _, _, _, h2801) => {
                    h2801
                }
            }
        }
        Adt4::C28(v49) => {
            let a2798 = v1;
            let a2799 = 6u64;
            if nat_eq(a2798, a2799) {
                match v49 {
                    Adt7::C0(_, _, _, _, h2800) => {
                        h2800
                    }
                }
            } else {
                0u64
            }
        }
    }
}

pub fn f112(v0: Adt4, v1: u64) -> Adt11 {
    let a2808 = v1;
    let a2809 = 4u64;
    if nat_lt(a2808, a2809) {
        Adt11::C0(v1)
    } else {
        let a2810 = v1;
        let a2811 = 4u64;
        if nat_eq(a2810, a2811) {
            Adt11::C1(0u64)
        } else {
            let a2812 = v1;
            let a2813 = 5u64;
            if nat_eq(a2812, a2813) {
                Adt11::C1(3u64)
            } else {
                let a2814 = v1;
                let a2815 = 6u64;
                if nat_eq(a2814, a2815) {
                    Adt11::C2(f111(v0, v1), 0u64, false)
                } else {
                    let a2816 = v1;
                    let a2817 = 7u64;
                    if nat_eq(a2816, a2817) {
                        Adt11::C2(0u64, f111(v0, v1), false)
                    } else {
                        let a2818 = v1;
                        let a2819 = 16u64;
                        if nat_lt(a2818, a2819) {
                            Adt11::C1({
                                let a2820 = v1;
                                let a2821 = 8u64;
                                nat_sub(a2820, a2821)
                            })
                        } else {
                            let a2822 = v1;
                            let a2823 = 20u64;
                            if nat_lt(a2822, a2823) {
                                Adt11::C5({
                                    let a2824 = v1;
                                    let a2825 = 16u64;
                                    nat_sub(a2824, a2825)
                                })
                            } else {
                                let a2826 = v1;
                                let a2827 = 20u64;
                                if nat_eq(a2826, a2827) {
                                    Adt11::C3(0u64, 0u64, false)
                                } else {
                                    let a2828 = v1;
                                    let a2829 = 21u64;
                                    if nat_eq(a2828, a2829) {
                                        Adt11::C0(4u64)
                                    } else {
                                        let a2830 = v1;
                                        let a2831 = 22u64;
                                        if nat_eq(a2830, a2831) {
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

pub fn f113(v0: u64, v1: Adt3) -> R<u64> {
    let v2: u64 = {
        let a2833 = v0;
        let a2834 = match v1.clone() {
            Adt3::C0(h2832, _, _, _, _, _, _, _) => {
                h2832
            }
        };
        let a2835 = 0u64;
        nat_quot(a2833, a2834, a2835)
    };
    let v3: u64 = {
        let a2837 = v2;
        let a2838 = match v1.clone() {
            Adt3::C0(_, h2836, _, _, _, _, _, _) => {
                h2836
            }
        };
        let a2839 = 0u64;
        nat_quot(a2837, a2838, a2839)
    };
    f105({
        let a2841 = v0;
        let a2842 = match v1.clone() {
            Adt3::C0(h2840, _, _, _, _, _, _, _) => {
                h2840
            }
        };
        let a2843 = 0u64;
        nat_rem(a2841, a2842, a2843)
    }, {
        let a2845 = v2;
        let a2846 = match v1.clone() {
            Adt3::C0(_, h2844, _, _, _, _, _, _) => {
                h2844
            }
        };
        let a2847 = 0u64;
        nat_rem(a2845, a2846, a2847)
    }, {
        let a2849 = v3;
        let a2850 = match v1.clone() {
            Adt3::C0(_, _, h2848, _, _, _, _, _) => {
                h2848
            }
        };
        let a2851 = 0u64;
        nat_rem(a2849, a2850, a2851)
    }, {
        let a2853 = v3;
        let a2854 = match v1.clone() {
            Adt3::C0(_, _, h2852, _, _, _, _, _) => {
                h2852
            }
        };
        let a2855 = 0u64;
        nat_quot(a2853, a2854, a2855)
    }, match v1.clone() {
        Adt3::C0(_, _, _, _, h2856, _, _, _) => {
            h2856
        }
    }, match v1.clone() {
        Adt3::C0(_, _, _, _, _, h2857, _, _) => {
            h2857
        }
    }, match v1.clone() {
        Adt3::C0(_, _, _, _, _, _, h2858, _) => {
            h2858
        }
    }, match v1 {
        Adt3::C0(_, _, _, _, _, _, _, h2859) => {
            h2859
        }
    })
}

pub fn f114(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let a2877 = {
        let a2872 = f152(v2, {
            let a2863 = {
                let a2860 = v0;
                let a2861 = v1;
                let a2862 = 0u64;
                nat_quot(a2860, a2861, a2862)
            };
            let a2864 = v2;
            let a2865 = 0u64;
            nat_rem(a2863, a2864, a2865)
        }, {
            let a2869 = {
                let a2866 = v0;
                let a2867 = v1;
                let a2868 = 0u64;
                nat_quot(a2866, a2867, a2868)
            };
            let a2870 = v2;
            let a2871 = 0u64;
            nat_quot(a2869, a2870, a2871)
        })?;
        let a2873 = v1;
        nat_mul(a2872, a2873)?
    };
    let a2878 = {
        let a2874 = v0;
        let a2875 = v1;
        let a2876 = 0u64;
        nat_rem(a2874, a2875, a2876)
    };
    nat_add(a2877, a2878)
}

pub fn f115(v0: u64, v1: Adt3, v2: Adt3) -> R<u64> {
    let v3: u64 = {
        let a2880 = v0;
        let a2881 = match v2.clone() {
            Adt3::C0(h2879, _, _, _, _, _, _, _) => {
                h2879
            }
        };
        let a2882 = 0u64;
        nat_quot(a2880, a2881, a2882)
    };
    let v4: u64 = {
        let a2884 = v3;
        let a2885 = match v2.clone() {
            Adt3::C0(_, h2883, _, _, _, _, _, _) => {
                h2883
            }
        };
        let a2886 = 0u64;
        nat_quot(a2884, a2885, a2886)
    };
    f153({
        let a2888 = v0;
        let a2889 = match v2.clone() {
            Adt3::C0(h2887, _, _, _, _, _, _, _) => {
                h2887
            }
        };
        let a2890 = 0u64;
        nat_rem(a2888, a2889, a2890)
    }, {
        let a2892 = v3;
        let a2893 = match v2.clone() {
            Adt3::C0(_, h2891, _, _, _, _, _, _) => {
                h2891
            }
        };
        let a2894 = 0u64;
        nat_rem(a2892, a2893, a2894)
    }, {
        let a2896 = v4;
        let a2897 = match v2.clone() {
            Adt3::C0(_, _, h2895, _, _, _, _, _) => {
                h2895
            }
        };
        let a2898 = 0u64;
        nat_rem(a2896, a2897, a2898)
    }, {
        let a2900 = v4;
        let a2901 = match v2 {
            Adt3::C0(_, _, h2899, _, _, _, _, _) => {
                h2899
            }
        };
        let a2902 = 0u64;
        nat_quot(a2900, a2901, a2902)
    }, match v1.clone() {
        Adt3::C0(h2903, _, _, _, _, _, _, _) => {
            h2903
        }
    }, match v1.clone() {
        Adt3::C0(_, h2904, _, _, _, _, _, _) => {
            h2904
        }
    }, match v1.clone() {
        Adt3::C0(_, _, h2905, _, _, _, _, _) => {
            h2905
        }
    }, match v1.clone() {
        Adt3::C0(_, _, _, h2906, _, _, _, _) => {
            h2906
        }
    }, match v1.clone() {
        Adt3::C0(_, _, _, _, h2907, _, _, _) => {
            h2907
        }
    }, match v1.clone() {
        Adt3::C0(_, _, _, _, _, h2908, _, _) => {
            h2908
        }
    }, match v1.clone() {
        Adt3::C0(_, _, _, _, _, _, h2909, _) => {
            h2909
        }
    }, match v1 {
        Adt3::C0(_, _, _, _, _, _, _, h2910) => {
            h2910
        }
    })
}

pub fn f116(v0: u64, v1: Adt3, v2: Adt3, v3: Adt3, v4: u64) -> R<Adt9> {
    let v5: u64 = {
        let a2912 = v0;
        let a2913 = match v3.clone() {
            Adt3::C0(h2911, _, _, _, _, _, _, _) => {
                h2911
            }
        };
        let a2914 = 0u64;
        nat_quot(a2912, a2913, a2914)
    };
    let v6: u64 = {
        let a2916 = v5;
        let a2917 = match v3.clone() {
            Adt3::C0(_, h2915, _, _, _, _, _, _) => {
                h2915
            }
        };
        let a2918 = 0u64;
        nat_quot(a2916, a2917, a2918)
    };
    let v7: u64 = {
        let a2920 = v0;
        let a2921 = match v3.clone() {
            Adt3::C0(h2919, _, _, _, _, _, _, _) => {
                h2919
            }
        };
        let a2922 = 0u64;
        nat_rem(a2920, a2921, a2922)
    };
    let v8: u64 = {
        let a2924 = v5;
        let a2925 = match v3.clone() {
            Adt3::C0(_, h2923, _, _, _, _, _, _) => {
                h2923
            }
        };
        let a2926 = 0u64;
        nat_rem(a2924, a2925, a2926)
    };
    let v9: u64 = {
        let a2928 = v6;
        let a2929 = match v3.clone() {
            Adt3::C0(_, _, h2927, _, _, _, _, _) => {
                h2927
            }
        };
        let a2930 = 0u64;
        nat_rem(a2928, a2929, a2930)
    };
    let v10: u64 = {
        let a2932 = v6;
        let a2933 = match v3.clone() {
            Adt3::C0(_, _, h2931, _, _, _, _, _) => {
                h2931
            }
        };
        let a2934 = 0u64;
        nat_quot(a2932, a2933, a2934)
    };
    let v11: u64 = {
        let a2935 = v4;
        let a2936 = 0u64;
        if nat_eq(a2935, a2936) {
            match v1.clone() {
                Adt3::C0(h2937, _, _, _, _, _, _, _) => {
                    h2937
                }
            }
        } else {
            let a2938 = v4;
            let a2939 = 1u64;
            if nat_eq(a2938, a2939) {
                match v1.clone() {
                    Adt3::C0(_, h2940, _, _, _, _, _, _) => {
                        h2940
                    }
                }
            } else {
                let a2941 = v4;
                let a2942 = 2u64;
                if nat_eq(a2941, a2942) {
                    match v1.clone() {
                        Adt3::C0(_, _, h2943, _, _, _, _, _) => {
                            h2943
                        }
                    }
                } else {
                    match v1.clone() {
                        Adt3::C0(_, _, _, h2944, _, _, _, _) => {
                            h2944
                        }
                    }
                }
            }
        }
    };
    let v12: bool = {
        let a2951 = v11;
        let a2952 = {
            let a2945 = v4;
            let a2946 = 0u64;
            if nat_eq(a2945, a2946) {
                v7
            } else {
                let a2947 = v4;
                let a2948 = 1u64;
                if nat_eq(a2947, a2948) {
                    v8
                } else {
                    let a2949 = v4;
                    let a2950 = 2u64;
                    if nat_eq(a2949, a2950) {
                        v9
                    } else {
                        v10
                    }
                }
            }
        };
        nat_le(a2951, a2952)
    };
    let v13: Adt3 = if v12 {
        v2
    } else {
        v1
    };
    Ok(Adt9::C1(v12, f105(if if v12 {
        let a2953 = v4;
        let a2954 = 0u64;
        nat_eq(a2953, a2954)
    } else {
        false
    } {
        let a2955 = v7;
        let a2956 = v11;
        nat_sub(a2955, a2956)
    } else {
        v7
    }, if if v12 {
        let a2957 = v4;
        let a2958 = 1u64;
        nat_eq(a2957, a2958)
    } else {
        false
    } {
        let a2959 = v8;
        let a2960 = v11;
        nat_sub(a2959, a2960)
    } else {
        v8
    }, if if v12 {
        let a2961 = v4;
        let a2962 = 2u64;
        nat_eq(a2961, a2962)
    } else {
        false
    } {
        let a2963 = v9;
        let a2964 = v11;
        nat_sub(a2963, a2964)
    } else {
        v9
    }, if if v12 {
        let a2965 = v4;
        let a2966 = 3u64;
        nat_eq(a2965, a2966)
    } else {
        false
    } {
        let a2967 = v10;
        let a2968 = v11;
        nat_sub(a2967, a2968)
    } else {
        v10
    }, match v13.clone() {
        Adt3::C0(_, _, _, _, h2969, _, _, _) => {
            h2969
        }
    }, match v13.clone() {
        Adt3::C0(_, _, _, _, _, h2970, _, _) => {
            h2970
        }
    }, match v13.clone() {
        Adt3::C0(_, _, _, _, _, _, h2971, _) => {
            h2971
        }
    }, match v13 {
        Adt3::C0(_, _, _, _, _, _, _, h2972) => {
            h2972
        }
    })?, f113(v0, v3)?))
}

pub fn f117(v0: u64, v1: Adt3, v2: Adt3, v3: u64, v4: u64, v5: u64, v6: u64, v7: bool) -> R<Adt9> {
    let v8: u64 = f59(v0, v2.clone(), 0u64);
    let v9: u64 = f59(v0, v2.clone(), 1u64);
    let v10: u64 = f59(v0, v2.clone(), 2u64);
    let v11: u64 = f59(v0, v2.clone(), 3u64);
    if v7 {
        Ok(Adt9::C1(false, f58(v1.clone(), f154(v8, v3, match v1.clone() {
            Adt3::C0(h2973, _, _, _, _, _, _, _) => {
                h2973
            }
        })?, f154(v9, v4, match v1.clone() {
            Adt3::C0(_, h2974, _, _, _, _, _, _) => {
                h2974
            }
        })?, f154(v10, v5, match v1.clone() {
            Adt3::C0(_, _, h2975, _, _, _, _, _) => {
                h2975
            }
        })?, f154(v11, v6, match v1 {
            Adt3::C0(_, _, _, h2976, _, _, _, _) => {
                h2976
            }
        })?)?, f113(v0, v2)?))
    } else {
        if if if if f155(v8, v3, match v1.clone() {
            Adt3::C0(h2977, _, _, _, _, _, _, _) => {
                h2977
            }
        })? {
            f155(v9, v4, match v1.clone() {
                Adt3::C0(_, h2978, _, _, _, _, _, _) => {
                    h2978
                }
            })?
        } else {
            false
        } {
            f155(v10, v5, match v1.clone() {
                Adt3::C0(_, _, h2979, _, _, _, _, _) => {
                    h2979
                }
            })?
        } else {
            false
        } {
            f155(v11, v6, match v1.clone() {
                Adt3::C0(_, _, _, h2980, _, _, _, _) => {
                    h2980
                }
            })?
        } else {
            false
        } {
            Ok(Adt9::C1(false, f58(v1, f156(v8, v3)?, f156(v9, v4)?, f156(v10, v5)?, f156(v11, v6)?)?, f113(v0, v2)?))
        } else {
            Ok(Adt9::C1(true, 0u64, f113(v0, v2)?))
        }
    }
}

pub fn f118(v0: u64, v1: Adt3, v2: Adt3, v3: u64) -> R<Adt9> {
    let v4: u64 = f59(v0, v2.clone(), 0u64);
    let v5: u64 = f59(v0, v2.clone(), 1u64);
    let v6: u64 = f59(v0, v2.clone(), 2u64);
    let v7: u64 = f59(v0, v2.clone(), 3u64);
    Ok(Adt9::C1(false, f58(v1.clone(), f157(v4, v3, match v1 {
        Adt3::C0(h2981, _, _, _, _, _, _, _) => {
            h2981
        }
    })?, v5, v6, v7)?, f113(v0, v2)?))
}

pub fn f119(v0: u64, v1: Adt3, v2: Adt3, v3: u64, v4: u64, v5: bool) -> R<Adt9> {
    let a2982 = v0;
    let a2983 = v3;
    if nat_lt(a2982, a2983) {
        Ok(Adt9::C1(false, {
            let a2984 = v0;
            let a2985 = 4u64;
            nat_mul(a2984, a2985)?
        }, {
            let a2986 = v0;
            let a2987 = 4u64;
            nat_mul(a2986, a2987)?
        }))
    } else {
        let v6: u64 = {
            let a2988 = v0;
            let a2989 = v3;
            nat_sub(a2988, a2989)
        };
        let v7: u64 = f113(v6, v1)?;
        let v8: u64 = f158(v6, v2, v4)?;
        if v5 {
            Ok(Adt9::C4(8u64, 0u64, v8, 4u64, v7, 3u64, v8))
        } else {
            Ok(Adt9::C1(true, v7, v8))
        }
    }
}

pub fn f120(v0: u64, v1: Adt3, v2: Adt3, v3: u64) -> R<Adt9> {
    let a2990 = v0;
    let a2991 = v3;
    if nat_lt(a2990, a2991) {
        Ok(Adt9::C1(true, 0u64, f113(v0, v2)?))
    } else {
        let v4: u64 = {
            let a2992 = v0;
            let a2993 = v3;
            nat_sub(a2992, a2993)
        };
        let v5: u64 = f115(v4, v2, v1.clone())?;
        Ok(Adt9::C4(8u64, 3u64, v5, 0u64, f113(v4, v1)?, 3u64, v5))
    }
}

pub fn f121(v0: u64, v1: u64, v2: Adt3, v3: Adt3) -> R<Adt9> {
    let v4: u64 = {
        let a2995 = v1;
        let a2996 = match v3.clone() {
            Adt3::C0(h2994, _, _, _, _, _, _, _) => {
                h2994
            }
        };
        let a2997 = 0u64;
        nat_rem(a2995, a2996, a2997)
    };
    let v5: u64 = {
        let a3003 = {
            let a2999 = v1;
            let a3000 = match v3.clone() {
                Adt3::C0(h2998, _, _, _, _, _, _, _) => {
                    h2998
                }
            };
            let a3001 = 0u64;
            nat_quot(a2999, a3000, a3001)
        };
        let a3004 = match v3.clone() {
            Adt3::C0(_, h3002, _, _, _, _, _, _) => {
                h3002
            }
        };
        let a3005 = 0u64;
        nat_rem(a3003, a3004, a3005)
    };
    let v6: bool = f159(v0, v4, v5);
    Ok(Adt9::C1(!v6, if v6 {
        f115(v1, v2, v3.clone())?
    } else {
        0u64
    }, f113(v1, v3)?))
}

pub fn f122(v0: u64, v1: Adt3, v2: Adt3, v3: u64, v4: bool, v5: Adt3) -> R<Adt9> {
    Ok(Adt9::C4({
        let a3006 = v3;
        let a3007 = 8u64;
        nat_add(a3006, a3007)?
    }, 0u64, f113(v0, v1)?, if v4 {
        5u64
    } else {
        4u64
    }, if v4 {
        0u64
    } else {
        f115(v0, v5, v2.clone())?
    }, 3u64, f113(v0, v2)?))
}

pub fn f123(v0: u64, v1: Adt3, v2: Adt3, v3: bool, v4: u64, v5: u64) -> R<Adt9> {
    let v6: u64 = {
        let a3009 = v0;
        let a3010 = match v2.clone() {
            Adt3::C0(h3008, _, _, _, _, _, _, _) => {
                h3008
            }
        };
        let a3011 = 0u64;
        nat_rem(a3009, a3010, a3011)
    };
    let v7: u64 = {
        let a3013 = v0;
        let a3014 = match v2.clone() {
            Adt3::C0(h3012, _, _, _, _, _, _, _) => {
                h3012
            }
        };
        let a3015 = 0u64;
        nat_quot(a3013, a3014, a3015)
    };
    let v8: u64 = {
        let a3017 = v7;
        let a3018 = match v2.clone() {
            Adt3::C0(_, h3016, _, _, _, _, _, _) => {
                h3016
            }
        };
        let a3019 = 0u64;
        nat_rem(a3017, a3018, a3019)
    };
    let v9: u64 = {
        let a3021 = v7;
        let a3022 = match v2.clone() {
            Adt3::C0(_, h3020, _, _, _, _, _, _) => {
                h3020
            }
        };
        let a3023 = 0u64;
        nat_quot(a3021, a3022, a3023)
    };
    let v10: u64 = {
        let a3025 = v9;
        let a3026 = match v2.clone() {
            Adt3::C0(_, _, h3024, _, _, _, _, _) => {
                h3024
            }
        };
        let a3027 = 0u64;
        nat_rem(a3025, a3026, a3027)
    };
    let v11: u64 = {
        let a3029 = v9;
        let a3030 = match v2.clone() {
            Adt3::C0(_, _, h3028, _, _, _, _, _) => {
                h3028
            }
        };
        let a3031 = 0u64;
        nat_quot(a3029, a3030, a3031)
    };
    let v12: u64 = f113(v0, v2)?;
    if v3 {
        let v13: u64 = {
            let a3040 = {
                let a3035 = {
                    let a3032 = v10;
                    let a3033 = v4;
                    let a3034 = 0u64;
                    nat_quot(a3032, a3033, a3034)
                };
                let a3036 = v5;
                nat_mul(a3035, a3036)?
            };
            let a3041 = {
                let a3037 = v8;
                let a3038 = v4;
                let a3039 = 0u64;
                nat_quot(a3037, a3038, a3039)
            };
            nat_add(a3040, a3041)?
        };
        let v14: u64 = {
            let a3061 = v6;
            let a3062 = {
                let a3059 = match v1.clone() {
                    Adt3::C0(h3042, _, _, _, _, _, _, _) => {
                        h3042
                    }
                };
                let a3060 = {
                    let a3057 = {
                        let a3043 = v8;
                        let a3044 = v4;
                        let a3045 = 0u64;
                        nat_rem(a3043, a3044, a3045)
                    };
                    let a3058 = {
                        let a3055 = match v1.clone() {
                            Adt3::C0(_, h3046, _, _, _, _, _, _) => {
                                h3046
                            }
                        };
                        let a3056 = {
                            let a3053 = {
                                let a3047 = v10;
                                let a3048 = v4;
                                let a3049 = 0u64;
                                nat_rem(a3047, a3048, a3049)
                            };
                            let a3054 = {
                                let a3051 = match v1.clone() {
                                    Adt3::C0(_, _, h3050, _, _, _, _, _) => {
                                        h3050
                                    }
                                };
                                let a3052 = v13;
                                nat_mul(a3051, a3052)?
                            };
                            nat_add(a3053, a3054)?
                        };
                        nat_mul(a3055, a3056)?
                    };
                    nat_add(a3057, a3058)?
                };
                nat_mul(a3059, a3060)?
            };
            nat_add(a3061, a3062)?
        };
        Ok(Adt9::C1(false, f113(v14, v1)?, v12))
    } else {
        let v15: u64 = {
            let a3068 = {
                let a3066 = {
                    let a3063 = v11;
                    let a3064 = v5;
                    let a3065 = 0u64;
                    nat_rem(a3063, a3064, a3065)
                };
                let a3067 = v4;
                nat_mul(a3066, a3067)?
            };
            let a3069 = v8;
            nat_add(a3068, a3069)?
        };
        let v16: u64 = {
            let a3075 = {
                let a3073 = {
                    let a3070 = v11;
                    let a3071 = v5;
                    let a3072 = 0u64;
                    nat_quot(a3070, a3071, a3072)
                };
                let a3074 = v4;
                nat_mul(a3073, a3074)?
            };
            let a3076 = v10;
            nat_add(a3075, a3076)?
        };
        let a3078 = v15;
        let a3079 = match v1.clone() {
            Adt3::C0(_, h3077, _, _, _, _, _, _) => {
                h3077
            }
        };
        if if nat_lt(a3078, a3079) {
            let a3081 = v16;
            let a3082 = match v1.clone() {
                Adt3::C0(_, _, h3080, _, _, _, _, _) => {
                    h3080
                }
            };
            nat_lt(a3081, a3082)
        } else {
            false
        } {
            Ok(Adt9::C1(false, f113({
                let a3091 = v6;
                let a3092 = {
                    let a3089 = match v1.clone() {
                        Adt3::C0(h3083, _, _, _, _, _, _, _) => {
                            h3083
                        }
                    };
                    let a3090 = {
                        let a3087 = v15;
                        let a3088 = {
                            let a3085 = match v1.clone() {
                                Adt3::C0(_, h3084, _, _, _, _, _, _) => {
                                    h3084
                                }
                            };
                            let a3086 = v16;
                            nat_mul(a3085, a3086)?
                        };
                        nat_add(a3087, a3088)?
                    };
                    nat_mul(a3089, a3090)?
                };
                nat_add(a3091, a3092)?
            }, v1)?, v12))
        } else {
            Ok(Adt9::C1(true, 0u64, v12))
        }
    }
}

pub fn f124(v0: u64, v1: Adt3, v2: Adt3) -> R<Adt9> {
    let v3: u64 = f113(v0, v2.clone())?;
    let v4: u64 = f113(v0, v1.clone())?;
    let a3097 = {
        let a3094 = v0;
        let a3095 = match v1 {
            Adt3::C0(h3093, _, _, _, _, _, _, _) => {
                h3093
            }
        };
        let a3096 = 0u64;
        nat_rem(a3094, a3095, a3096)
    };
    let a3098 = 0u64;
    if nat_eq(a3097, a3098) {
        Ok(Adt9::C1(false, v4, v3))
    } else {
        Ok(Adt9::C4(8u64, 3u64, f113({
            let a3099 = v0;
            let a3100 = 1u64;
            nat_sub(a3099, a3100)
        }, v2)?, 0u64, v4, 3u64, v3))
    }
}

pub fn f125(v0: u64, v1: Adt3, v2: Adt3, v3: u64, v4: u64) -> R<Adt9> {
    let a3101 = v3;
    let a3102 = 13u64;
    if nat_eq(a3101, a3102) {
        f160(v0, v1, v4)
    } else {
        let v5: u64 = f149(match v1.clone() {
            Adt3::C0(h3103, _, _, _, _, _, _, _) => {
                h3103
            }
        }, {
            let a3104 = v3;
            let a3105 = 16u64;
            nat_eq(a3104, a3105)
        })?;
        let v6: u64 = {
            let a3106 = v0;
            let a3107 = v5;
            let a3108 = 0u64;
            nat_quot(a3106, a3107, a3108)
        };
        let v7: u64 = {
            let a3109 = v0;
            let a3110 = v5;
            let a3111 = 0u64;
            nat_rem(a3109, a3110, a3111)
        };
        let v8: u64 = {
            let a3115 = 1u64;
            let a3116 = {
                let a3113 = match v1.clone() {
                    Adt3::C0(h3112, _, _, _, _, _, _, _) => {
                        h3112
                    }
                };
                let a3114 = 2u64;
                nat_mul(a3113, a3114)?
            };
            nat_add(a3115, a3116)?
        };
        let a3117 = v7;
        let a3118 = v8;
        if nat_lt(a3117, a3118) {
            f161(v7, v6, v1, 0u64)
        } else {
            let v9: u64 = f113(v6, v2)?;
            let a3119 = v3;
            let a3120 = 14u64;
            if nat_eq(a3119, a3120) {
                Ok(Adt9::C4(2u64, 1u64, 1u64, 2u64, 0u64, 3u64, v9))
            } else {
                let a3121 = v7;
                let a3122 = v8;
                if nat_eq(a3121, a3122) {
                    Ok(Adt9::C4(2u64, 1u64, 1u64, 2u64, 0u64, 1u64, 1u64))
                } else {
                    let a3125 = v7;
                    let a3126 = {
                        let a3123 = v8;
                        let a3124 = 1u64;
                        nat_add(a3123, a3124)?
                    };
                    if nat_eq(a3125, a3126) {
                        Ok(Adt9::C4(3u64, 2u64, match v1 {
                            Adt3::C0(h3127, _, _, _, _, _, _, _) => {
                                h3127
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

pub fn f126(v0: u64, v1: Adt3, v2: Adt3, v3: u64) -> R<Adt9> {
    let a3128 = v0;
    let a3129 = 0u64;
    if nat_eq(a3128, a3129) {
        Ok(Adt9::C4(19u64, 2u64, 0u64, 2u64, 0u64, 1u64, 0u64))
    } else {
        let a3136 = {
            let a3130 = v0;
            let a3131 = 1u64;
            nat_add(a3130, a3131)?
        };
        let a3137 = {
            let a3134 = 2u64;
            let a3135 = {
                let a3132 = v3;
                let a3133 = 2u64;
                nat_mul(a3132, a3133)?
            };
            nat_add(a3134, a3135)?
        };
        if nat_eq(a3136, a3137) {
            Ok(Adt9::C4(19u64, 1u64, 0u64, 2u64, 0u64, 6u64, 0u64))
        } else {
            let v4: u64 = {
                let a3138 = v0;
                let a3139 = 1u64;
                nat_sub(a3138, a3139)
            };
            let v5: u64 = {
                let a3140 = v4;
                let a3141 = 2u64;
                let a3142 = 0u64;
                nat_quot(a3140, a3141, a3142)
            };
            let a3146 = {
                let a3143 = v4;
                let a3144 = 2u64;
                let a3145 = 0u64;
                nat_rem(a3143, a3144, a3145)
            };
            let a3147 = 0u64;
            if nat_eq(a3146, a3147) {
                Ok(Adt9::C4(17u64, 0u64, f113(v5, v1)?, 4u64, f113(v5, v2)?, 1u64, 2u64))
            } else {
                Ok(Adt9::C4(18u64, 1u64, 0u64, 1u64, 2u64, 1u64, 0u64))
            }
        }
    }
}

pub fn f127(v0: u64, v1: u64, v2: u64, v3: u64) -> R<Adt9> {
    let a3148 = v0;
    let a3149 = f22(v1, v2, v3)?;
    if nat_lt(a3148, a3149) {
        let a3150 = v0;
        let a3151 = v1;
        if nat_lt(a3150, a3151) {
            Ok(Adt9::C1(false, {
                let a3152 = 4u64;
                let a3153 = v0;
                nat_mul(a3152, a3153)?
            }, {
                let a3154 = 4u64;
                let a3155 = v0;
                nat_mul(a3154, a3155)?
            }))
        } else {
            let v4: u64 = {
                let a3156 = v0;
                let a3157 = v1;
                nat_sub(a3156, a3157)
            };
            let v5: u64 = f162(v4, v2)?;
            let a3158 = v5;
            let a3159 = 2u64;
            if nat_lt(a3158, a3159) {
                Ok(Adt9::C4(6u64, {
                    let a3160 = v5;
                    let a3161 = 0u64;
                    if nat_eq(a3160, a3161) {
                        4u64
                    } else {
                        5u64
                    }
                }, f163(v4, v2)?, 2u64, 0u64, 1u64, v5))
            } else {
                let v6: u64 = f164(v4, v2)?;
                Ok(Adt9::C4(8u64, 3u64, v6, 1u64, if f165(v4, v2)? {
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

pub fn f128(v0: Adt3, v1: Adt3) -> R<u64> {
    let a3169 = {
        let a3162 = 2u64;
        let a3163 = f102(v1.clone())?;
        nat_mul(a3162, a3163)?
    };
    let a3170 = {
        let a3167 = f102(v1)?;
        let a3168 = {
            let a3165 = 1u64;
            let a3166 = match v0 {
                Adt3::C0(h3164, _, _, _, _, _, _, _) => {
                    h3164
                }
            };
            nat_add(a3165, a3166)?
        };
        nat_mul(a3167, a3168)?
    };
    nat_add(a3169, a3170)
}

pub fn f129(v0: u64, v1: Adt3) -> R<Adt9> {
    Ok(Adt9::C4(19u64, 4u64, f113(v0, v1)?, 2u64, 0u64, 1u64, 2u64))
}

pub fn f130(v0: u64, v1: u64) -> bool {
    let a3171 = v1;
    let a3172 = 2147483647u64;
    if nat_le(a3171, a3172) {
        let a3173 = v1;
        let a3174 = v0;
        nat_lt(a3173, a3174)
    } else {
        false
    }
}

pub fn f131(v0: u64, v1: u64, v2: u64, v3: Adt3, v4: Adt3, v5: Adt3, v6: u64) -> R<Adt9> {
    let m3175 = f91(v0);
    match m3175 {
        Adt14::C0 => {
            Ok(Adt9::C0(Adt10::C1))
        }
        Adt14::C1(v7) => {
            let v8: u64 = {
                let a3177 = v2;
                let a3178 = match v7.clone() {
                    Adt15::C0(_, h3176, _) => {
                        h3176
                    }
                };
                nat_mul(a3177, a3178)?
            };
            let m3191 = f166(v0, v8, v6, v1, match v5.clone() {
                Adt3::C0(h3179, _, _, _, _, _, _, _) => {
                    h3179
                }
            }, match v5.clone() {
                Adt3::C0(_, h3180, _, _, _, _, _, _) => {
                    h3180
                }
            }, match v5 {
                Adt3::C0(_, _, h3181, _, _, _, _, _) => {
                    h3181
                }
            }, {
                let a3184 = match v3.clone() {
                    Adt3::C0(h3182, _, _, _, _, _, _, _) => {
                        h3182
                    }
                };
                let a3185 = match v7.clone() {
                    Adt15::C0(_, h3183, _) => {
                        h3183
                    }
                };
                nat_mul(a3184, a3185)?
            }, match v3.clone() {
                Adt3::C0(_, h3186, _, _, _, _, _, _) => {
                    h3186
                }
            }, match v3.clone() {
                Adt3::C0(_, _, _, _, h3187, _, _, _) => {
                    h3187
                }
            }, match v3.clone() {
                Adt3::C0(_, _, _, _, _, h3188, _, _) => {
                    h3188
                }
            }, match v3.clone() {
                Adt3::C0(_, _, _, _, _, _, h3189, _) => {
                    h3189
                }
            }, match v3 {
                Adt3::C0(_, _, _, _, _, _, _, h3190) => {
                    h3190
                }
            })?;
            match m3191 {
                None => {
                    Ok(Adt9::C0(Adt10::C1))
                }
                Some(v9) => {
                    let a3192 = v0;
                    let a3193 = 0u64;
                    if if nat_eq(a3192, a3193) {
                        true
                    } else {
                        let a3194 = v0;
                        let a3195 = 26u64;
                        nat_eq(a3194, a3195)
                    } {
                        Ok(Adt9::C1(false, v9, f113({
                            let a3199 = {
                                let a3197 = v1;
                                let a3198 = match v4.clone() {
                                    Adt3::C0(h3196, _, _, _, _, _, _, _) => {
                                        h3196
                                    }
                                };
                                nat_mul(a3197, a3198)?
                            };
                            let a3200 = v8;
                            nat_add(a3199, a3200)?
                        }, v4)?))
                    } else {
                        Ok(Adt9::C5(v0, v9, match v7.clone() {
                            Adt15::C0(_, _, h3201) => {
                                h3201
                            }
                        }, f113({
                            let a3205 = {
                                let a3203 = v1;
                                let a3204 = match v4.clone() {
                                    Adt3::C0(h3202, _, _, _, _, _, _, _) => {
                                        h3202
                                    }
                                };
                                nat_mul(a3203, a3204)?
                            };
                            let a3206 = v8;
                            nat_add(a3205, a3206)?
                        }, v4.clone())?, {
                            let a3213 = {
                                let a3211 = {
                                    let a3208 = match v7 {
                                        Adt15::C0(_, h3207, _) => {
                                            h3207
                                        }
                                    };
                                    let a3209 = 1u64;
                                    nat_sub(a3208, a3209)
                                };
                                let a3212 = match v4.clone() {
                                    Adt3::C0(_, _, _, _, h3210, _, _, _) => {
                                        h3210
                                    }
                                };
                                nat_mul(a3211, a3212)?
                            };
                            let a3214 = 4u64;
                            nat_add(a3213, a3214)?
                        }, match v4 {
                            Adt3::C0(_, _, _, _, h3215, _, _, _) => {
                                h3215
                            }
                        }))
                    }
                }
            }
        }
    }
}

pub fn f132(v0: u64, v1: u64, v2: u64) -> u64 {
    let a3216 = v1;
    let a3217 = 0u64;
    if if nat_eq(a3216, a3217) {
        true
    } else {
        let a3218 = v2;
        let a3219 = 0u64;
        nat_eq(a3218, a3219)
    } {
        0u64
    } else {
        v0
    }
}

pub fn f133(v0: u64, v1: u64) -> bool {
    let a3220 = v0;
    let a3221 = 4294967295u64;
    if nat_eq(a3220, a3221) {
        true
    } else {
        let a3222 = v0;
        let a3223 = 2147483648u64;
        if nat_lt(a3222, a3223) {
            let a3224 = v0;
            let a3225 = v1;
            nat_lt(a3224, a3225)
        } else {
            false
        }
    }
}

pub fn f134(v0: Adt7) -> R<u64> {
    let a3230 = if match v0.clone() {
        Adt7::C0(_, _, _, h3226, _) => {
            h3226
        }
    } {
        2u64
    } else {
        1u64
    };
    let a3231 = {
        let a3228 = match v0 {
            Adt7::C0(_, _, _, _, h3227) => {
                h3227
            }
        };
        let a3229 = 1u64;
        if nat_eq(a3228, a3229) {
            1u64
        } else {
            0u64
        }
    };
    nat_add(a3230, a3231)
}

pub fn f135(v0: Adt6, v1: u64, v2: u64) -> R<u64> {
    let v3: u64 = {
        let a3236 = {
            let a3233 = v2;
            let a3234 = match v0.clone() {
                Adt6::C0(_, h3232, _, _) => {
                    h3232
                }
            };
            nat_mul(a3233, a3234)?
        };
        let a3237 = f106(match v0.clone() {
            Adt6::C0(_, _, h3235, _) => {
                h3235
            }
        });
        nat_add(a3236, a3237)?
    };
    let v4: u64 = f107(match v0.clone() {
        Adt6::C0(_, _, h3238, _) => {
            h3238
        }
    });
    let v5: u64 = {
        let a3242 = {
            let a3240 = v3;
            let a3241 = match v0 {
                Adt6::C0(h3239, _, _, _) => {
                    h3239
                }
            };
            nat_add(a3240, a3241)?
        };
        let a3243 = v4;
        nat_sub(a3242, a3243)
    };
    let a3248 = {
        let a3244 = v5;
        let a3245 = v1;
        if nat_lt(a3244, a3245) {
            v5
        } else {
            v1
        }
    };
    let a3249 = {
        let a3246 = v3;
        let a3247 = v4;
        nat_sub(a3246, a3247)
    };
    Ok(nat_sub(a3248, a3249))
}

pub fn f136(v0: Adt7, v1: Adt3, v2: Adt3, v3: u64, v4: u64) -> R<Option<u64>> {
    let m3258 = f61(match v0.clone() {
        Adt7::C0(h3250, _, _, _, _) => {
            h3250
        }
    }, match v1.clone() {
        Adt3::C0(h3251, _, _, _, _, _, _, _) => {
            h3251
        }
    }, f59(v3, v2.clone(), 0u64), {
        let a3255 = v4;
        let a3256 = {
            let m3254 = match v0.clone() {
                Adt7::C0(h3253, _, _, _, _) => {
                    h3253
                }
            };
            match m3254 {
                Adt6::C0(h3252, _, _, _) => {
                    h3252
                }
            }
        };
        let a3257 = 0u64;
        nat_rem(a3255, a3256, a3257)
    })?;
    match m3258 {
        None => {
            Ok(None::<u64>)
        }
        Some(v5) => {
            let m3267 = f61(match v0.clone() {
                Adt7::C0(_, h3259, _, _, _) => {
                    h3259
                }
            }, match v1.clone() {
                Adt3::C0(_, h3260, _, _, _, _, _, _) => {
                    h3260
                }
            }, f59(v3, v2.clone(), 1u64), {
                let a3264 = v4;
                let a3265 = {
                    let m3263 = match v0 {
                        Adt7::C0(h3262, _, _, _, _) => {
                            h3262
                        }
                    };
                    match m3263 {
                        Adt6::C0(h3261, _, _, _) => {
                            h3261
                        }
                    }
                };
                let a3266 = 0u64;
                nat_quot(a3264, a3265, a3266)
            })?;
            match m3267 {
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

pub fn f137(v0: u64, v1: Adt3, v2: u64, v3: u64, v4: u64) -> R<Adt9> {
    let v5: u64 = {
        let a3274 = v3;
        let a3275 = {
            let a3271 = f113({
                let a3268 = v0;
                let a3269 = 2u64;
                let a3270 = 0u64;
                nat_quot(a3268, a3269, a3270)
            }, v1)?;
            let a3272 = v2;
            let a3273 = 0u64;
            nat_quot(a3271, a3272, a3273)
        };
        nat_add(a3274, a3275)?
    };
    let a3279 = {
        let a3276 = v0;
        let a3277 = 2u64;
        let a3278 = 0u64;
        nat_rem(a3276, a3277, a3278)
    };
    let a3280 = 0u64;
    if nat_eq(a3279, a3280) {
        Ok(Adt9::C4(19u64, 1u64, v5, 2u64, 0u64, 1u64, 2u64))
    } else {
        let a3281 = v4;
        let a3282 = 0u64;
        if nat_eq(a3281, a3282) {
            Ok(Adt9::C4(19u64, 2u64, 1u64, 2u64, 0u64, 1u64, v5))
        } else {
            Ok(Adt9::C0(Adt10::C1))
        }
    }
}

pub fn f138(v0: u64, v1: u64) -> R<u64> {
    let a3285 = 8u64;
    let a3286 = {
        let a3283 = f167(v0);
        let a3284 = v1;
        nat_mul(a3283, a3284)?
    };
    nat_add(a3285, a3286)
}

pub fn f139(v0: u64, v1: u64, v2: Adt3, v3: u64, v4: u64, v5: u64) -> R<Adt9> {
    let v6: u64 = match v2.clone() {
        Adt3::C0(h3287, _, _, _, _, _, _, _) => {
            h3287
        }
    };
    let v7: u64 = f167(v3);
    let v8: u64 = {
        let a3290 = 1u64;
        let a3291 = {
            let a3288 = v6;
            let a3289 = v7;
            nat_mul(a3288, a3289)?
        };
        nat_add(a3290, a3291)?
    };
    let a3292 = v0;
    let a3293 = 0u64;
    if nat_eq(a3292, a3293) {
        Ok(f168(0u64))
    } else {
        let a3294 = v0;
        let a3295 = v8;
        if nat_lt(a3294, a3295) {
            let v9: u64 = {
                let a3296 = v0;
                let a3297 = 1u64;
                nat_sub(a3296, a3297)
            };
            let v10: u64 = {
                let a3298 = v9;
                let a3299 = v7;
                let a3300 = 0u64;
                nat_rem(a3298, a3299, a3300)
            };
            let v11: u64 = f113({
                let a3306 = {
                    let a3301 = v1;
                    let a3302 = v6;
                    nat_mul(a3301, a3302)?
                };
                let a3307 = {
                    let a3303 = v9;
                    let a3304 = v7;
                    let a3305 = 0u64;
                    nat_quot(a3303, a3304, a3305)
                };
                nat_add(a3306, a3307)?
            }, v2)?;
            let a3308 = v3;
            let a3309 = 30u64;
            if nat_eq(a3308, a3309) {
                let a3310 = v10;
                let a3311 = 0u64;
                if nat_eq(a3310, a3311) {
                    Ok(Adt9::C4(6u64, 0u64, v11, 2u64, 0u64, 1u64, 1u64))
                } else {
                    let a3312 = v10;
                    let a3313 = 1u64;
                    if nat_eq(a3312, a3313) {
                        Ok(Adt9::C4(10u64, 1u64, 1u64, 1u64, 1u64, 1u64, 1u64))
                    } else {
                        let a3314 = v10;
                        let a3315 = 2u64;
                        if nat_eq(a3314, a3315) {
                            Ok(Adt9::C4(1u64, 1u64, 1u64, 2u64, 0u64, 1u64, 1u64))
                        } else {
                            Ok(Adt9::C4(0u64, 1u64, 0u64, 1u64, 1u64, 1u64, 0u64))
                        }
                    }
                }
            } else {
                let a3316 = v10;
                let a3317 = 0u64;
                if nat_eq(a3316, a3317) {
                    Ok(Adt9::C4(10u64, 0u64, v11, 0u64, v11, 1u64, 1u64))
                } else {
                    let a3318 = v10;
                    let a3319 = 1u64;
                    if nat_eq(a3318, a3319) {
                        Ok(Adt9::C4(1u64, 1u64, 1u64, 2u64, 0u64, 1u64, 1u64))
                    } else {
                        Ok(Adt9::C4(0u64, 1u64, 0u64, 1u64, 1u64, 1u64, 0u64))
                    }
                }
            }
        } else {
            let v12: u64 = {
                let a3320 = v0;
                let a3321 = v8;
                nat_sub(a3320, a3321)
            };
            let a3322 = v12;
            let a3323 = 0u64;
            if nat_eq(a3322, a3323) {
                Ok(Adt9::C4(22u64, 2u64, v6, 2u64, 0u64, 1u64, 1u64))
            } else {
                let a3324 = v12;
                let a3325 = 1u64;
                if nat_eq(a3324, a3325) {
                    Ok(Adt9::C4(21u64, 1u64, 0u64, 1u64, 1u64, 1u64, 1u64))
                } else {
                    let a3326 = v12;
                    let a3327 = 2u64;
                    if nat_eq(a3326, a3327) {
                        Ok(Adt9::C4(2u64, 1u64, 1u64, 2u64, 0u64, 1u64, 1u64))
                    } else {
                        let a3328 = v12;
                        let a3329 = 3u64;
                        if nat_eq(a3328, a3329) {
                            Ok(Adt9::C4(4u64, 1u64, 1u64, 2u64, v4, 1u64, 1u64))
                        } else {
                            let a3330 = v12;
                            let a3331 = 4u64;
                            if nat_eq(a3330, a3331) {
                                Ok(Adt9::C4(12u64, 1u64, 1u64, 2u64, 0u64, 1u64, 1u64))
                            } else {
                                let a3332 = v12;
                                let a3333 = 5u64;
                                if nat_eq(a3332, a3333) {
                                    Ok(Adt9::C4(11u64, 2u64, 1065353216u64, 1u64, 1u64, 1u64, 2u64))
                                } else {
                                    let a3334 = v12;
                                    let a3335 = 6u64;
                                    if nat_eq(a3334, a3335) {
                                        if f169(v5) {
                                            Ok(Adt9::C4(19u64, 1u64, 2u64, 2u64, 0u64, 1u64, {
                                                let a3336 = 3u64;
                                                let a3337 = v1;
                                                nat_add(a3336, a3337)?
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

pub fn f140(v0: u64, v1: Adt3, v2: Adt3, v3: u64) -> R<Adt9> {
    let v4: u64 = {
        let a3338 = v0;
        let a3339 = f83(v3);
        let a3340 = 0u64;
        nat_quot(a3338, a3339, a3340)
    };
    let v5: u64 = {
        let a3345 = 3u64;
        let a3346 = {
            let a3342 = v4;
            let a3343 = match v1.clone() {
                Adt3::C0(h3341, _, _, _, _, _, _, _) => {
                    h3341
                }
            };
            let a3344 = 0u64;
            nat_quot(a3342, a3343, a3344)
        };
        nat_add(a3345, a3346)?
    };
    let v6: u64 = f113(v4, v1)?;
    let v7: u64 = f113(v4, v2)?;
    let a3347 = v3;
    let a3348 = 30u64;
    if nat_eq(a3347, a3348) {
        let a3352 = {
            let a3349 = v0;
            let a3350 = 3u64;
            let a3351 = 0u64;
            nat_rem(a3349, a3350, a3351)
        };
        let a3353 = 0u64;
        if nat_eq(a3352, a3353) {
            Ok(Adt9::C4(6u64, 0u64, v6, 2u64, 0u64, 1u64, 1u64))
        } else {
            let a3357 = {
                let a3354 = v0;
                let a3355 = 3u64;
                let a3356 = 0u64;
                nat_rem(a3354, a3355, a3356)
            };
            let a3358 = 1u64;
            if nat_eq(a3357, a3358) {
                Ok(Adt9::C4(10u64, 1u64, 1u64, 1u64, v5, 1u64, 1u64))
            } else {
                Ok(Adt9::C4(7u64, 1u64, 1u64, 2u64, 0u64, 3u64, v7))
            }
        }
    } else {
        Ok(Adt9::C4(10u64, 0u64, v6, 1u64, v5, 3u64, v7))
    }
}

pub fn f141(v0: u64) -> R<u64> {
    let v1: u64 = {
        let a3359 = v0;
        let a3360 = 8388608u64;
        let a3361 = 0u64;
        nat_quot(a3359, a3360, a3361)
    };
    let a3362 = v0;
    let a3363 = 2147483648u64;
    if if if nat_lt(a3362, a3363) {
        let a3364 = 127u64;
        let a3365 = v1;
        nat_le(a3364, a3365)
    } else {
        false
    } {
        let a3366 = v1;
        let a3367 = 190u64;
        nat_lt(a3366, a3367)
    } else {
        false
    } {
        let v2: u64 = {
            let a3371 = 8388608u64;
            let a3372 = {
                let a3368 = v0;
                let a3369 = 8388608u64;
                let a3370 = 0u64;
                nat_rem(a3368, a3369, a3370)
            };
            nat_add(a3371, a3372)?
        };
        let a3373 = v1;
        let a3374 = 150u64;
        if nat_lt(a3373, a3374) {
            let v3: u64 = f170({
                let a3375 = 150u64;
                let a3376 = v1;
                nat_sub(a3375, a3376)
            });
            let a3380 = {
                let a3377 = v2;
                let a3378 = v3;
                let a3379 = 0u64;
                nat_rem(a3377, a3378, a3379)
            };
            let a3381 = 0u64;
            if nat_eq(a3380, a3381) {
                let a3382 = v2;
                let a3383 = v3;
                let a3384 = 0u64;
                Ok(nat_quot(a3382, a3383, a3384))
            } else {
                Ok(0u64)
            }
        } else {
            let a3387 = f170({
                let a3385 = v1;
                let a3386 = 150u64;
                nat_sub(a3385, a3386)
            });
            let a3388 = v2;
            nat_mul(a3387, a3388)
        }
    } else {
        Ok(0u64)
    }
}

pub fn f142(v0: u64, v1: u64, v2: u64) -> bool {
    let a3389 = v0;
    let a3390 = 1u64;
    if nat_le(a3389, a3390) {
        true
    } else {
        let a3394 = {
            let a3391 = v1;
            let a3392 = v2;
            let a3393 = 1u64;
            nat_rem(a3391, a3392, a3393)
        };
        let a3395 = 0u64;
        nat_eq(a3394, a3395)
    }
}

pub fn f143(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64) -> R<bool> {
    let a3396 = v1;
    let a3397 = 1u64;
    if if if nat_le(a3396, a3397) {
        true
    } else {
        let a3398 = v0;
        let a3399 = v2;
        nat_le(a3398, a3399)
    } {
        let a3400 = v3;
        let a3401 = 1u64;
        if nat_le(a3400, a3401) {
            true
        } else {
            let a3408 = {
                let a3406 = v0;
                let a3407 = {
                    let a3404 = {
                        let a3402 = v1;
                        let a3403 = 1u64;
                        nat_sub(a3402, a3403)
                    };
                    let a3405 = v2;
                    nat_mul(a3404, a3405)?
                };
                nat_add(a3406, a3407)?
            };
            let a3409 = v4;
            nat_le(a3408, a3409)
        }
    } else {
        false
    } {
        let a3410 = v5;
        let a3411 = 1u64;
        if nat_le(a3410, a3411) {
            Ok(true)
        } else {
            let a3424 = {
                let a3422 = {
                    let a3416 = v0;
                    let a3417 = {
                        let a3414 = {
                            let a3412 = v1;
                            let a3413 = 1u64;
                            nat_sub(a3412, a3413)
                        };
                        let a3415 = v2;
                        nat_mul(a3414, a3415)?
                    };
                    nat_add(a3416, a3417)?
                };
                let a3423 = {
                    let a3420 = {
                        let a3418 = v3;
                        let a3419 = 1u64;
                        nat_sub(a3418, a3419)
                    };
                    let a3421 = v4;
                    nat_mul(a3420, a3421)?
                };
                nat_add(a3422, a3423)?
            };
            let a3425 = v6;
            Ok(nat_le(a3424, a3425))
        }
    } else {
        Ok(false)
    }
}

pub fn f144(v0: u64, v1: u64, _: u64, v3: u64, v4: u64, v5: u64, v6: u64, v7: u64, v8: u64, v9: u64, v10: u64, v11: u64, v12: u64, v13: u64, v14: u64) -> bool {
    if if if if if if if if if f171(v0, v1, v6, v7, v8) {
        let a3426 = v5;
        let a3427 = 26u64;
        nat_eq(a3426, a3427)
    } else {
        false
    } {
        let a3428 = v3;
        let a3429 = v7;
        nat_eq(a3428, a3429)
    } else {
        false
    } {
        let a3430 = v4;
        let a3431 = v8;
        nat_eq(a3430, a3431)
    } else {
        false
    } {
        let a3432 = v9;
        let a3433 = 1u64;
        nat_eq(a3432, a3433)
    } else {
        false
    } {
        let a3434 = v10;
        let a3435 = f172(v0);
        nat_eq(a3434, a3435)
    } else {
        false
    } {
        let a3436 = v11;
        let a3437 = v1;
        nat_eq(a3436, a3437)
    } else {
        false
    } {
        let a3438 = v12;
        let a3439 = v6;
        nat_eq(a3438, a3439)
    } else {
        false
    } {
        let a3440 = v13;
        let a3441 = v7;
        nat_eq(a3440, a3441)
    } else {
        false
    } {
        let a3442 = v14;
        let a3443 = v8;
        nat_eq(a3442, a3443)
    } else {
        false
    }
}

pub fn f145(v0: u64, v1: u64, v2: u64, v3: u64) -> bool {
    let a3444 = v0;
    let a3445 = 0u64;
    if if if nat_eq(a3444, a3445) {
        true
    } else {
        let a3446 = v1;
        let a3447 = 0u64;
        nat_eq(a3446, a3447)
    } {
        true
    } else {
        let a3448 = v2;
        let a3449 = 0u64;
        nat_eq(a3448, a3449)
    } {
        true
    } else {
        let a3450 = v3;
        let a3451 = 0u64;
        nat_eq(a3450, a3451)
    }
}

pub fn f146(v0: Adt3, v1: bool) -> R<u64> {
    f173(v0.clone(), {
        let a3453 = match v0.clone() {
            Adt3::C0(h3452, _, _, _, _, _, _, _) => {
                h3452
            }
        };
        let a3454 = 1u64;
        nat_sub(a3453, a3454)
    }, {
        let a3456 = match v0.clone() {
            Adt3::C0(_, h3455, _, _, _, _, _, _) => {
                h3455
            }
        };
        let a3457 = 1u64;
        nat_sub(a3456, a3457)
    }, {
        let a3459 = match v0.clone() {
            Adt3::C0(_, _, h3458, _, _, _, _, _) => {
                h3458
            }
        };
        let a3460 = 1u64;
        nat_sub(a3459, a3460)
    }, {
        let a3462 = match v0 {
            Adt3::C0(_, _, _, h3461, _, _, _, _) => {
                h3461
            }
        };
        let a3463 = 1u64;
        nat_sub(a3462, a3463)
    }, v1)
}

pub fn f147(v0: u64, v1: u64, v2: u64, v3: u64) -> R<u64> {
    let a3464 = v0;
    let a3465 = 0u64;
    if nat_eq(a3464, a3465) {
        Ok(0u64)
    } else {
        let v4: u64 = {
            let a3466 = v0;
            let a3467 = 1u64;
            nat_sub(a3466, a3467)
        };
        let v5: u64 = {
            let a3468 = v4;
            let a3469 = v1;
            let a3470 = 0u64;
            nat_quot(a3468, a3469, a3470)
        };
        let v6: u64 = {
            let a3478 = {
                let a3471 = v5;
                let a3472 = v2;
                nat_mul(a3471, a3472)?
            };
            let a3479 = {
                let a3476 = {
                    let a3473 = v4;
                    let a3474 = v1;
                    let a3475 = 0u64;
                    nat_rem(a3473, a3474, a3475)
                };
                let a3477 = v3;
                nat_mul(a3476, a3477)?
            };
            nat_add(a3478, a3479)?
        };
        let a3480 = v5;
        let a3481 = 0u64;
        if nat_eq(a3480, a3481) {
            Ok(v6)
        } else {
            let v7: u64 = {
                let a3490 = {
                    let a3484 = {
                        let a3482 = v5;
                        let a3483 = 1u64;
                        nat_sub(a3482, a3483)
                    };
                    let a3485 = v2;
                    nat_mul(a3484, a3485)?
                };
                let a3491 = {
                    let a3488 = {
                        let a3486 = v1;
                        let a3487 = 1u64;
                        nat_sub(a3486, a3487)
                    };
                    let a3489 = v3;
                    nat_mul(a3488, a3489)?
                };
                nat_add(a3490, a3491)?
            };
            let a3492 = v6;
            let a3493 = v7;
            if nat_lt(a3492, a3493) {
                Ok(v7)
            } else {
                Ok(v6)
            }
        }
    }
}

pub fn f148(v0: Adt3, v1: u64) -> bool {
    let a3494 = v1;
    let a3495 = 0u64;
    if !nat_eq(a3494, a3495) {
        true
    } else {
        let a3497 = match v0 {
            Adt3::C0(_, _, _, _, h3496, _, _, _) => {
                h3496
            }
        };
        let a3498 = 4u64;
        nat_eq(a3497, a3498)
    }
}

pub fn f149(v0: u64, v1: bool) -> R<u64> {
    let a3501 = {
        let a3499 = v0;
        let a3500 = 2u64;
        nat_mul(a3499, a3500)?
    };
    let a3502 = if v1 {
        4u64
    } else {
        2u64
    };
    nat_add(a3501, a3502)
}

pub fn f150(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: bool, v6: u64, v7: u64, v8: u64, v9: u64, v10: u64, v11: u64, v12: u64, v13: u64, v14: u64, v15: u64, v16: u64, v17: u64) -> R<Adt10> {
    let m3503 = f174(v0, v1);
    match m3503 {
        None => {
            if v5 {
                let m3504 = f175(v0, v2, v3, v4, v6, v7, v8, v9, v10, v11, v12, v13, v14, v15, v16, v17)?;
                match m3504 {
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
                let a3505 = 4u64;
                let a3506 = v28;
                nat_le(a3505, a3506)
            };
            let a3507 = v2;
            let a3508 = 0u64;
            if if if if if nat_eq(a3507, a3508) {
                let a3509 = v4;
                let a3510 = 0u64;
                nat_eq(a3509, a3510)
            } else {
                false
            } {
                let a3511 = v6;
                let a3512 = v14;
                if nat_eq(a3511, a3512) {
                    let a3513 = v7;
                    let a3514 = v15;
                    nat_eq(a3513, a3514)
                } else {
                    false
                }
            } else {
                false
            } {
                let a3515 = v8;
                let a3516 = v16;
                if nat_eq(a3515, a3516) {
                    let a3517 = v9;
                    let a3518 = v17;
                    nat_eq(a3517, a3518)
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
                        let a3519 = v3;
                        let a3520 = 0u64;
                        nat_eq(a3519, a3520)
                    } else {
                        false
                    } {
                        f92(v6, v7, v8, v9, v10, v11, v12, v13)
                    } else {
                        false
                    }
                }
            } else {
                false
            } {
                Ok(Adt10::C2(v28, v29, v6, v7, v8, v9, {
                    let a3525 = {
                        let a3521 = v6;
                        let a3522 = v7;
                        nat_mul(a3521, a3522)?
                    };
                    let a3526 = {
                        let a3523 = v8;
                        let a3524 = v9;
                        nat_mul(a3523, a3524)?
                    };
                    nat_mul(a3525, a3526)?
                }))
            } else {
                Ok(Adt10::C1)
            }
        }
    }
}

pub fn f151(v0: u64) -> u64 {
    let m3527 = f91(v0);
    match m3527 {
        Adt14::C0 => {
            0u64
        }
        Adt14::C1(v1) => {
            let a3529 = match v1.clone() {
                Adt15::C0(_, h3528, _) => {
                    h3528
                }
            };
            let a3530 = 1u64;
            if if nat_eq(a3529, a3530) {
                let a3532 = match v1.clone() {
                    Adt15::C0(_, _, h3531) => {
                        h3531
                    }
                };
                let a3533 = 1u64;
                if if if nat_eq(a3532, a3533) {
                    true
                } else {
                    let a3535 = match v1.clone() {
                        Adt15::C0(_, _, h3534) => {
                            h3534
                        }
                    };
                    let a3536 = 2u64;
                    nat_eq(a3535, a3536)
                } {
                    true
                } else {
                    let a3538 = match v1.clone() {
                        Adt15::C0(_, _, h3537) => {
                            h3537
                        }
                    };
                    let a3539 = 4u64;
                    nat_eq(a3538, a3539)
                } {
                    true
                } else {
                    let a3541 = match v1.clone() {
                        Adt15::C0(_, _, h3540) => {
                            h3540
                        }
                    };
                    let a3542 = 8u64;
                    nat_eq(a3541, a3542)
                }
            } else {
                false
            } {
                match v1 {
                    Adt15::C0(_, _, h3543) => {
                        h3543
                    }
                }
            } else {
                0u64
            }
        }
    }
}

pub fn f152(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let a3548 = {
        let a3546 = {
            let a3544 = v0;
            let a3545 = v1;
            nat_sub(a3544, a3545)
        };
        let a3547 = 1u64;
        nat_sub(a3546, a3547)
    };
    let a3549 = v2;
    nat_add(a3548, a3549)
}

pub fn f153(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64, v7: u64, v8: u64, v9: u64, v10: u64, v11: u64) -> R<u64> {
    f105({
        let a3550 = v0;
        let a3551 = v4;
        let a3552 = 0u64;
        nat_rem(a3550, a3551, a3552)
    }, {
        let a3553 = v1;
        let a3554 = v5;
        let a3555 = 0u64;
        nat_rem(a3553, a3554, a3555)
    }, {
        let a3556 = v2;
        let a3557 = v6;
        let a3558 = 0u64;
        nat_rem(a3556, a3557, a3558)
    }, {
        let a3559 = v3;
        let a3560 = v7;
        let a3561 = 0u64;
        nat_rem(a3559, a3560, a3561)
    }, v8, v9, v10, v11)
}

pub fn f154(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let a3562 = v1;
    let a3563 = 2147483648u64;
    if nat_lt(a3562, a3563) {
        let a3574 = {
            let a3572 = {
                let a3567 = {
                    let a3564 = v0;
                    let a3565 = v2;
                    let a3566 = 0u64;
                    nat_rem(a3564, a3565, a3566)
                };
                let a3568 = v2;
                nat_add(a3567, a3568)?
            };
            let a3573 = {
                let a3569 = v1;
                let a3570 = v2;
                let a3571 = 0u64;
                nat_rem(a3569, a3570, a3571)
            };
            nat_sub(a3572, a3573)
        };
        let a3575 = v2;
        let a3576 = 0u64;
        Ok(nat_rem(a3574, a3575, a3576))
    } else {
        let a3579 = {
            let a3577 = v0;
            let a3578 = f106(v1);
            nat_add(a3577, a3578)?
        };
        let a3580 = v2;
        let a3581 = 0u64;
        Ok(nat_rem(a3579, a3580, a3581))
    }
}

pub fn f155(v0: u64, v1: u64, v2: u64) -> R<bool> {
    let a3584 = f107(v1);
    let a3585 = {
        let a3582 = v0;
        let a3583 = f106(v1);
        nat_add(a3582, a3583)?
    };
    if nat_le(a3584, a3585) {
        let a3586 = f156(v0, v1)?;
        let a3587 = v2;
        Ok(nat_lt(a3586, a3587))
    } else {
        Ok(false)
    }
}

pub fn f156(v0: u64, v1: u64) -> R<u64> {
    let a3590 = {
        let a3588 = v0;
        let a3589 = f106(v1);
        nat_add(a3588, a3589)?
    };
    let a3591 = f107(v1);
    Ok(nat_sub(a3590, a3591))
}

pub fn f157(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let a3592 = v0;
    let a3593 = v1;
    if nat_lt(a3592, a3593) {
        let a3594 = v1;
        let a3595 = v0;
        Ok(nat_sub(a3594, a3595))
    } else {
        let a3598 = v0;
        let a3599 = {
            let a3596 = v1;
            let a3597 = v2;
            nat_add(a3596, a3597)?
        };
        if nat_lt(a3598, a3599) {
            let a3600 = v0;
            let a3601 = v1;
            Ok(nat_sub(a3600, a3601))
        } else {
            let a3608 = {
                let a3604 = 2u64;
                let a3605 = {
                    let a3602 = v2;
                    let a3603 = 1u64;
                    nat_sub(a3602, a3603)
                };
                nat_mul(a3604, a3605)?
            };
            let a3609 = {
                let a3606 = v0;
                let a3607 = v1;
                nat_sub(a3606, a3607)
            };
            Ok(nat_sub(a3608, a3609))
        }
    }
}

pub fn f158(v0: u64, v1: Adt3, v2: u64) -> R<u64> {
    let a3612 = {
        let a3610 = v2;
        let a3611 = f173(v1.clone(), f59(v0, v1.clone(), 0u64), f59(v0, v1.clone(), 1u64), f59(v0, v1.clone(), 2u64), f59(v0, v1.clone(), 3u64), false)?;
        nat_add(a3610, a3611)?
    };
    let a3613 = f173(v1.clone(), f59(v0, v1.clone(), 0u64), f59(v0, v1.clone(), 1u64), f59(v0, v1.clone(), 2u64), f59(v0, v1, 3u64), true)?;
    Ok(nat_sub(a3612, a3613))
}

pub fn f159(v0: u64, v1: u64, v2: u64) -> bool {
    let a3614 = v0;
    let a3615 = 0u64;
    if nat_eq(a3614, a3615) {
        let a3616 = v2;
        let a3617 = v1;
        nat_le(a3616, a3617)
    } else {
        let a3618 = v0;
        let a3619 = 1u64;
        if nat_eq(a3618, a3619) {
            let a3620 = v2;
            let a3621 = v1;
            nat_lt(a3620, a3621)
        } else {
            let a3622 = v0;
            let a3623 = 2u64;
            if nat_eq(a3622, a3623) {
                let a3624 = v1;
                let a3625 = v2;
                nat_le(a3624, a3625)
            } else {
                let a3626 = v0;
                let a3627 = 3u64;
                if nat_eq(a3626, a3627) {
                    let a3628 = v1;
                    let a3629 = v2;
                    nat_lt(a3628, a3629)
                } else {
                    let a3630 = v1;
                    let a3631 = v2;
                    nat_eq(a3630, a3631)
                }
            }
        }
    }
}

pub fn f160(v0: u64, v1: Adt3, v2: u64) -> R<Adt9> {
    let a3632 = v0;
    let a3633 = 0u64;
    if nat_eq(a3632, a3633) {
        Ok(f168(0u64))
    } else {
        let a3636 = {
            let a3634 = v0;
            let a3635 = 1u64;
            nat_add(a3634, a3635)?
        };
        let a3637 = f103(v1.clone(), 13u64, v2)?;
        if nat_eq(a3636, a3637) {
            Ok(Adt9::C4({
                let a3638 = v2;
                let a3639 = 0u64;
                if nat_eq(a3638, a3639) {
                    2u64
                } else {
                    7u64
                }
            }, 1u64, 0u64, 2u64, 0u64, 3u64, 0u64))
        } else {
            let v3: u64 = {
                let a3640 = v0;
                let a3641 = 1u64;
                nat_sub(a3640, a3641)
            };
            if f148(v1.clone(), v2) {
                let v4: u64 = f149(match v1.clone() {
                    Adt3::C0(h3642, _, _, _, _, _, _, _) => {
                        h3642
                    }
                }, false)?;
                let v5: u64 = {
                    let a3643 = v3;
                    let a3644 = v4;
                    let a3645 = 0u64;
                    nat_quot(a3643, a3644, a3645)
                };
                let v6: u64 = {
                    let a3646 = v3;
                    let a3647 = v4;
                    let a3648 = 0u64;
                    nat_rem(a3646, a3647, a3648)
                };
                let a3651 = {
                    let a3649 = v6;
                    let a3650 = 1u64;
                    nat_add(a3649, a3650)?
                };
                let a3652 = v4;
                if nat_eq(a3651, a3652) {
                    Ok(Adt9::C4({
                        let a3653 = v2;
                        let a3654 = 0u64;
                        if nat_eq(a3653, a3654) {
                            0u64
                        } else {
                            4u64
                        }
                    }, 1u64, 0u64, 1u64, 1u64, 1u64, 0u64))
                } else {
                    f161(v6, v5, v1, v2)
                }
            } else {
                f176({
                    let a3655 = v3;
                    let a3656 = 2u64;
                    let a3657 = 0u64;
                    nat_quot(a3655, a3656, a3657)
                }, {
                    let a3658 = v3;
                    let a3659 = 2u64;
                    let a3660 = 0u64;
                    nat_rem(a3658, a3659, a3660)
                }, v1, v2, 0u64)
            }
        }
    }
}

pub fn f161(v0: u64, v1: u64, v2: Adt3, v3: u64) -> R<Adt9> {
    let a3661 = v0;
    let a3662 = 0u64;
    if nat_eq(a3661, a3662) {
        Ok(f168(1u64))
    } else {
        f176({
            let a3671 = {
                let a3664 = v1;
                let a3665 = match v2.clone() {
                    Adt3::C0(h3663, _, _, _, _, _, _, _) => {
                        h3663
                    }
                };
                nat_mul(a3664, a3665)?
            };
            let a3672 = {
                let a3668 = {
                    let a3666 = v0;
                    let a3667 = 1u64;
                    nat_sub(a3666, a3667)
                };
                let a3669 = 2u64;
                let a3670 = 0u64;
                nat_quot(a3668, a3669, a3670)
            };
            nat_add(a3671, a3672)?
        }, {
            let a3675 = {
                let a3673 = v0;
                let a3674 = 1u64;
                nat_sub(a3673, a3674)
            };
            let a3676 = 2u64;
            let a3677 = 0u64;
            nat_rem(a3675, a3676, a3677)
        }, v2, v3, 1u64)
    }
}

pub fn f162(v0: u64, v1: u64) -> R<u64> {
    let a3678 = v0;
    let a3679 = f78(v1)?;
    let a3680 = 0u64;
    Ok(nat_rem(a3678, a3679, a3680))
}

pub fn f163(v0: u64, v1: u64) -> R<u64> {
    let a3681 = 4u64;
    let a3682 = f177(v0, v1)?;
    nat_mul(a3681, a3682)
}

pub fn f164(v0: u64, v1: u64) -> R<u64> {
    let a3683 = 4u64;
    let a3684 = f178(v0, v1)?;
    nat_mul(a3683, a3684)
}

pub fn f165(v0: u64, v1: u64) -> R<bool> {
    let a3690 = {
        let a3687 = {
            let a3685 = f162(v0, v1)?;
            let a3686 = 2u64;
            nat_sub(a3685, a3686)
        };
        let a3688 = 2u64;
        let a3689 = 0u64;
        nat_rem(a3687, a3688, a3689)
    };
    let a3691 = 0u64;
    Ok(nat_eq(a3690, a3691))
}

pub fn f166(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64, v7: u64, v8: u64, v9: u64, v10: u64, v11: u64, v12: u64) -> R<Option<u64>> {
    let m3692 = f91(v0);
    match m3692 {
        Adt14::C0 => {
            Ok(None::<u64>)
        }
        Adt14::C1(v13) => {
            if if if if if f179(v0) {
                let a3693 = v1;
                let a3694 = v7;
                nat_lt(a3693, a3694)
            } else {
                false
            } {
                f130(v8, v2)
            } else {
                false
            } {
                let a3695 = v3;
                let a3696 = f180(v4, v5, v6)?;
                nat_lt(a3695, a3696)
            } else {
                false
            } {
                let a3701 = {
                    let a3698 = v7;
                    let a3699 = match v13.clone() {
                        Adt15::C0(_, h3697, _) => {
                            h3697
                        }
                    };
                    let a3700 = 0u64;
                    nat_rem(a3698, a3699, a3700)
                };
                let a3702 = 0u64;
                nat_eq(a3701, a3702)
            } else {
                false
            } {
                Ok(Some({
                    let a3731 = {
                        let a3721 = {
                            let a3711 = {
                                let a3707 = {
                                    let a3704 = v1;
                                    let a3705 = match v13 {
                                        Adt15::C0(_, h3703, _) => {
                                            h3703
                                        }
                                    };
                                    let a3706 = 0u64;
                                    nat_quot(a3704, a3705, a3706)
                                };
                                let a3708 = v9;
                                nat_mul(a3707, a3708)?
                            };
                            let a3712 = {
                                let a3709 = v2;
                                let a3710 = v10;
                                nat_mul(a3709, a3710)?
                            };
                            nat_add(a3711, a3712)?
                        };
                        let a3722 = {
                            let a3719 = {
                                let a3716 = {
                                    let a3713 = v3;
                                    let a3714 = v4;
                                    let a3715 = 0u64;
                                    nat_quot(a3713, a3714, a3715)
                                };
                                let a3717 = v5;
                                let a3718 = 0u64;
                                nat_rem(a3716, a3717, a3718)
                            };
                            let a3720 = v11;
                            nat_mul(a3719, a3720)?
                        };
                        nat_add(a3721, a3722)?
                    };
                    let a3732 = {
                        let a3729 = {
                            let a3726 = {
                                let a3723 = v3;
                                let a3724 = v4;
                                let a3725 = 0u64;
                                nat_quot(a3723, a3724, a3725)
                            };
                            let a3727 = v5;
                            let a3728 = 0u64;
                            nat_quot(a3726, a3727, a3728)
                        };
                        let a3730 = v12;
                        nat_mul(a3729, a3730)?
                    };
                    nat_add(a3731, a3732)?
                }))
            } else {
                Ok(None::<u64>)
            }
        }
    }
}

pub fn f167(v0: u64) -> u64 {
    let a3733 = v0;
    let a3734 = 30u64;
    if nat_eq(a3733, a3734) {
        4u64
    } else {
        3u64
    }
}

pub fn f168(v0: u64) -> Adt9 {
    Adt9::C4(0u64, 2u64, 0u64, 2u64, 0u64, 1u64, v0)
}

pub fn f169(v0: u64) -> bool {
    let a3735 = 0u64;
    let a3736 = v0;
    if nat_lt(a3735, a3736) {
        let a3737 = v0;
        let a3738 = 2139095040u64;
        nat_le(a3737, a3738)
    } else {
        false
    }
}

pub fn f170(v0: u64) -> u64 {
    let a3739 = v0;
    let a3740 = 40u64;
    if nat_lt(a3739, a3740) {
        let a3741 = v0;
        let a3742 = 20u64;
        if nat_lt(a3741, a3742) {
            let a3743 = v0;
            let a3744 = 10u64;
            if nat_lt(a3743, a3744) {
                let a3745 = v0;
                let a3746 = 5u64;
                if nat_lt(a3745, a3746) {
                    let a3747 = v0;
                    let a3748 = 2u64;
                    if nat_lt(a3747, a3748) {
                        let a3749 = v0;
                        let a3750 = 1u64;
                        if nat_lt(a3749, a3750) {
                            1u64
                        } else {
                            2u64
                        }
                    } else {
                        let a3751 = v0;
                        let a3752 = 3u64;
                        if nat_lt(a3751, a3752) {
                            4u64
                        } else {
                            let a3753 = v0;
                            let a3754 = 4u64;
                            if nat_lt(a3753, a3754) {
                                8u64
                            } else {
                                16u64
                            }
                        }
                    }
                } else {
                    let a3755 = v0;
                    let a3756 = 7u64;
                    if nat_lt(a3755, a3756) {
                        let a3757 = v0;
                        let a3758 = 6u64;
                        if nat_lt(a3757, a3758) {
                            32u64
                        } else {
                            64u64
                        }
                    } else {
                        let a3759 = v0;
                        let a3760 = 8u64;
                        if nat_lt(a3759, a3760) {
                            128u64
                        } else {
                            let a3761 = v0;
                            let a3762 = 9u64;
                            if nat_lt(a3761, a3762) {
                                256u64
                            } else {
                                512u64
                            }
                        }
                    }
                }
            } else {
                let a3763 = v0;
                let a3764 = 15u64;
                if nat_lt(a3763, a3764) {
                    let a3765 = v0;
                    let a3766 = 12u64;
                    if nat_lt(a3765, a3766) {
                        let a3767 = v0;
                        let a3768 = 11u64;
                        if nat_lt(a3767, a3768) {
                            1024u64
                        } else {
                            2048u64
                        }
                    } else {
                        let a3769 = v0;
                        let a3770 = 13u64;
                        if nat_lt(a3769, a3770) {
                            4096u64
                        } else {
                            let a3771 = v0;
                            let a3772 = 14u64;
                            if nat_lt(a3771, a3772) {
                                8192u64
                            } else {
                                16384u64
                            }
                        }
                    }
                } else {
                    let a3773 = v0;
                    let a3774 = 17u64;
                    if nat_lt(a3773, a3774) {
                        let a3775 = v0;
                        let a3776 = 16u64;
                        if nat_lt(a3775, a3776) {
                            32768u64
                        } else {
                            65536u64
                        }
                    } else {
                        let a3777 = v0;
                        let a3778 = 18u64;
                        if nat_lt(a3777, a3778) {
                            131072u64
                        } else {
                            let a3779 = v0;
                            let a3780 = 19u64;
                            if nat_lt(a3779, a3780) {
                                262144u64
                            } else {
                                524288u64
                            }
                        }
                    }
                }
            }
        } else {
            let a3781 = v0;
            let a3782 = 30u64;
            if nat_lt(a3781, a3782) {
                let a3783 = v0;
                let a3784 = 25u64;
                if nat_lt(a3783, a3784) {
                    let a3785 = v0;
                    let a3786 = 22u64;
                    if nat_lt(a3785, a3786) {
                        let a3787 = v0;
                        let a3788 = 21u64;
                        if nat_lt(a3787, a3788) {
                            1048576u64
                        } else {
                            2097152u64
                        }
                    } else {
                        let a3789 = v0;
                        let a3790 = 23u64;
                        if nat_lt(a3789, a3790) {
                            4194304u64
                        } else {
                            let a3791 = v0;
                            let a3792 = 24u64;
                            if nat_lt(a3791, a3792) {
                                8388608u64
                            } else {
                                16777216u64
                            }
                        }
                    }
                } else {
                    let a3793 = v0;
                    let a3794 = 27u64;
                    if nat_lt(a3793, a3794) {
                        let a3795 = v0;
                        let a3796 = 26u64;
                        if nat_lt(a3795, a3796) {
                            33554432u64
                        } else {
                            67108864u64
                        }
                    } else {
                        let a3797 = v0;
                        let a3798 = 28u64;
                        if nat_lt(a3797, a3798) {
                            134217728u64
                        } else {
                            let a3799 = v0;
                            let a3800 = 29u64;
                            if nat_lt(a3799, a3800) {
                                268435456u64
                            } else {
                                536870912u64
                            }
                        }
                    }
                }
            } else {
                let a3801 = v0;
                let a3802 = 35u64;
                if nat_lt(a3801, a3802) {
                    let a3803 = v0;
                    let a3804 = 32u64;
                    if nat_lt(a3803, a3804) {
                        let a3805 = v0;
                        let a3806 = 31u64;
                        if nat_lt(a3805, a3806) {
                            1073741824u64
                        } else {
                            2147483648u64
                        }
                    } else {
                        let a3807 = v0;
                        let a3808 = 33u64;
                        if nat_lt(a3807, a3808) {
                            4294967296u64
                        } else {
                            let a3809 = v0;
                            let a3810 = 34u64;
                            if nat_lt(a3809, a3810) {
                                8589934592u64
                            } else {
                                17179869184u64
                            }
                        }
                    }
                } else {
                    let a3811 = v0;
                    let a3812 = 37u64;
                    if nat_lt(a3811, a3812) {
                        let a3813 = v0;
                        let a3814 = 36u64;
                        if nat_lt(a3813, a3814) {
                            34359738368u64
                        } else {
                            68719476736u64
                        }
                    } else {
                        let a3815 = v0;
                        let a3816 = 38u64;
                        if nat_lt(a3815, a3816) {
                            137438953472u64
                        } else {
                            let a3817 = v0;
                            let a3818 = 39u64;
                            if nat_lt(a3817, a3818) {
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

pub fn f171(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64) -> bool {
    if f145(v1, v2, v3, v4) {
        f181(v0)
    } else {
        f179(v0)
    }
}

pub fn f172(v0: u64) -> u64 {
    let a3819 = v0;
    let a3820 = 26u64;
    if nat_eq(a3819, a3820) {
        26u64
    } else {
        0u64
    }
}

pub fn f173(v0: Adt3, v1: u64, v2: u64, v3: u64, v4: u64, v5: bool) -> R<u64> {
    let a3841 = {
        let a3835 = {
            let a3829 = {
                let a3823 = v1;
                let a3824 = if v5 {
                    f106(match v0.clone() {
                        Adt3::C0(_, _, _, _, h3821, _, _, _) => {
                            h3821
                        }
                    })
                } else {
                    f107(match v0.clone() {
                        Adt3::C0(_, _, _, _, h3822, _, _, _) => {
                            h3822
                        }
                    })
                };
                nat_mul(a3823, a3824)?
            };
            let a3830 = {
                let a3827 = v2;
                let a3828 = if v5 {
                    f106(match v0.clone() {
                        Adt3::C0(_, _, _, _, _, h3825, _, _) => {
                            h3825
                        }
                    })
                } else {
                    f107(match v0.clone() {
                        Adt3::C0(_, _, _, _, _, h3826, _, _) => {
                            h3826
                        }
                    })
                };
                nat_mul(a3827, a3828)?
            };
            nat_add(a3829, a3830)?
        };
        let a3836 = {
            let a3833 = v3;
            let a3834 = if v5 {
                f106(match v0.clone() {
                    Adt3::C0(_, _, _, _, _, _, h3831, _) => {
                        h3831
                    }
                })
            } else {
                f107(match v0.clone() {
                    Adt3::C0(_, _, _, _, _, _, h3832, _) => {
                        h3832
                    }
                })
            };
            nat_mul(a3833, a3834)?
        };
        nat_add(a3835, a3836)?
    };
    let a3842 = {
        let a3839 = v4;
        let a3840 = if v5 {
            f106(match v0 {
                Adt3::C0(_, _, _, _, _, _, _, h3837) => {
                    h3837
                }
            })
        } else {
            f107(match v0 {
                Adt3::C0(_, _, _, _, _, _, _, h3838) => {
                    h3838
                }
            })
        };
        nat_mul(a3839, a3840)?
    };
    nat_add(a3841, a3842)
}

pub fn f174(v0: u64, v1: u64) -> Option<u64> {
    let a3843 = v0;
    let a3844 = 2u64;
    if nat_eq(a3843, a3844) {
        Some(0u64)
    } else {
        let a3845 = v0;
        let a3846 = 5u64;
        if nat_eq(a3845, a3846) {
            Some(1u64)
        } else {
            let a3847 = v0;
            let a3848 = 6u64;
            if nat_eq(a3847, a3848) {
                Some(2u64)
            } else {
                let a3849 = v0;
                let a3850 = 7u64;
                if nat_eq(a3849, a3850) {
                    Some(3u64)
                } else {
                    let a3851 = v0;
                    let a3852 = 9u64;
                    if nat_eq(a3851, a3852) {
                        Some(4u64)
                    } else {
                        let a3853 = v0;
                        let a3854 = 8u64;
                        if nat_eq(a3853, a3854) {
                            Some(5u64)
                        } else {
                            let a3855 = v0;
                            let a3856 = 89u64;
                            if nat_eq(a3855, a3856) {
                                let a3857 = v1;
                                let a3858 = 2u64;
                                if nat_eq(a3857, a3858) {
                                    Some(6u64)
                                } else {
                                    let a3859 = v1;
                                    let a3860 = 0u64;
                                    if nat_eq(a3859, a3860) {
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

pub fn f175(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64, v7: u64, v8: u64, v9: u64, v10: u64, v11: u64, v12: u64, v13: u64, v14: u64, v15: u64) -> R<Adt16> {
    let a3861 = v0;
    let a3862 = 30u64;
    if nat_eq(a3861, a3862) {
        let a3863 = v2;
        let a3864 = 0u64;
        if if if nat_eq(a3863, a3864) {
            true
        } else {
            let a3865 = v2;
            let a3866 = 1u64;
            nat_eq(a3865, a3866)
        } {
            let a3867 = v1;
            let a3868 = 0u64;
            if nat_eq(a3867, a3868) {
                true
            } else {
                f182(v1, v4)
            }
        } else {
            false
        } {
            f183(28u64, 0u64, 0u64, v3, v5, v4, v6, v7, v9, v8, v10, v11, v12, v13, v14, v15)
        } else {
            Ok(Adt16::C0)
        }
    } else {
        if if if f184(v1) {
            f184(v2)
        } else {
            false
        } {
            let a3869 = v1;
            let a3870 = 0u64;
            if if nat_eq(a3869, a3870) {
                true
            } else {
                let a3871 = v2;
                let a3872 = 0u64;
                nat_eq(a3871, a3872)
            } {
                true
            } else {
                let a3873 = v1;
                let a3874 = v2;
                nat_eq(a3873, a3874)
            }
        } else {
            false
        } {
            f183(v0, 0u64, 0u64, v3, v4, v5, v6, v7, v8, v9, v10, v11, v12, v13, v14, v15)
        } else {
            Ok(Adt16::C0)
        }
    }
}

pub fn f176(v0: u64, v1: u64, v2: Adt3, v3: u64, v4: u64) -> R<Adt9> {
    let a3875 = v1;
    let a3876 = 0u64;
    if nat_eq(a3875, a3876) {
        Ok(Adt9::C4({
            let a3877 = v3;
            let a3878 = 0u64;
            if nat_eq(a3877, a3878) {
                1u64
            } else {
                6u64
            }
        }, 0u64, f113(v0, v2)?, 2u64, 0u64, 1u64, 2u64))
    } else {
        Ok(Adt9::C4({
            let a3879 = v3;
            let a3880 = 0u64;
            if nat_eq(a3879, a3880) {
                0u64
            } else {
                4u64
            }
        }, 1u64, v4, 1u64, 2u64, 1u64, v4))
    }
}

pub fn f177(v0: u64, v1: u64) -> R<u64> {
    let a3881 = v0;
    let a3882 = f78(v1)?;
    let a3883 = 0u64;
    Ok(nat_quot(a3881, a3882, a3883))
}

pub fn f178(v0: u64, v1: u64) -> R<u64> {
    let v2: u64 = f177(v0, v1)?;
    let v3: u64 = {
        let a3886 = {
            let a3884 = f162(v0, v1)?;
            let a3885 = 2u64;
            nat_sub(a3884, a3885)
        };
        let a3887 = 2u64;
        let a3888 = 0u64;
        nat_quot(a3886, a3887, a3888)
    };
    let v4: u64 = {
        let a3889 = v2;
        let a3890 = v1;
        nat_mul(a3889, a3890)?
    };
    if f165(v0, v1)? {
        let a3891 = v4;
        let a3892 = v3;
        nat_add(a3891, a3892)
    } else {
        let a3904 = {
            let a3900 = v4;
            let a3901 = {
                let a3898 = {
                    let a3893 = v1;
                    let a3894 = 1u64;
                    nat_sub(a3893, a3894)
                };
                let a3899 = {
                    let a3895 = v2;
                    let a3896 = v1;
                    let a3897 = 0u64;
                    nat_rem(a3895, a3896, a3897)
                };
                nat_mul(a3898, a3899)?
            };
            nat_sub(a3900, a3901)
        };
        let a3905 = {
            let a3902 = v3;
            let a3903 = v1;
            nat_mul(a3902, a3903)?
        };
        nat_add(a3904, a3905)
    }
}

pub fn f179(v0: u64) -> bool {
    let a3906 = v0;
    let a3907 = 0u64;
    if if if if if if if if if if if if if if if if if if if if if if if if if if if if if nat_eq(a3906, a3907) {
        true
    } else {
        let a3908 = v0;
        let a3909 = 1u64;
        nat_eq(a3908, a3909)
    } {
        true
    } else {
        let a3910 = v0;
        let a3911 = 2u64;
        nat_eq(a3910, a3911)
    } {
        true
    } else {
        let a3912 = v0;
        let a3913 = 3u64;
        nat_eq(a3912, a3913)
    } {
        true
    } else {
        let a3914 = v0;
        let a3915 = 6u64;
        nat_eq(a3914, a3915)
    } {
        true
    } else {
        let a3916 = v0;
        let a3917 = 7u64;
        nat_eq(a3916, a3917)
    } {
        true
    } else {
        let a3918 = v0;
        let a3919 = 8u64;
        nat_eq(a3918, a3919)
    } {
        true
    } else {
        let a3920 = v0;
        let a3921 = 10u64;
        nat_eq(a3920, a3921)
    } {
        true
    } else {
        let a3922 = v0;
        let a3923 = 11u64;
        nat_eq(a3922, a3923)
    } {
        true
    } else {
        let a3924 = v0;
        let a3925 = 12u64;
        nat_eq(a3924, a3925)
    } {
        true
    } else {
        let a3926 = v0;
        let a3927 = 13u64;
        nat_eq(a3926, a3927)
    } {
        true
    } else {
        let a3928 = v0;
        let a3929 = 14u64;
        nat_eq(a3928, a3929)
    } {
        true
    } else {
        let a3930 = v0;
        let a3931 = 16u64;
        nat_eq(a3930, a3931)
    } {
        true
    } else {
        let a3932 = v0;
        let a3933 = 17u64;
        nat_eq(a3932, a3933)
    } {
        true
    } else {
        let a3934 = v0;
        let a3935 = 18u64;
        nat_eq(a3934, a3935)
    } {
        true
    } else {
        let a3936 = v0;
        let a3937 = 19u64;
        nat_eq(a3936, a3937)
    } {
        true
    } else {
        let a3938 = v0;
        let a3939 = 20u64;
        nat_eq(a3938, a3939)
    } {
        true
    } else {
        let a3940 = v0;
        let a3941 = 21u64;
        nat_eq(a3940, a3941)
    } {
        true
    } else {
        let a3942 = v0;
        let a3943 = 22u64;
        nat_eq(a3942, a3943)
    } {
        true
    } else {
        let a3944 = v0;
        let a3945 = 23u64;
        nat_eq(a3944, a3945)
    } {
        true
    } else {
        let a3946 = v0;
        let a3947 = 26u64;
        nat_eq(a3946, a3947)
    } {
        true
    } else {
        let a3948 = v0;
        let a3949 = 29u64;
        nat_eq(a3948, a3949)
    } {
        true
    } else {
        let a3950 = v0;
        let a3951 = 30u64;
        nat_eq(a3950, a3951)
    } {
        true
    } else {
        let a3952 = v0;
        let a3953 = 34u64;
        nat_eq(a3952, a3953)
    } {
        true
    } else {
        let a3954 = v0;
        let a3955 = 35u64;
        nat_eq(a3954, a3955)
    } {
        true
    } else {
        let a3956 = v0;
        let a3957 = 39u64;
        nat_eq(a3956, a3957)
    } {
        true
    } else {
        let a3958 = v0;
        let a3959 = 40u64;
        nat_eq(a3958, a3959)
    } {
        true
    } else {
        let a3960 = v0;
        let a3961 = 41u64;
        nat_eq(a3960, a3961)
    } {
        true
    } else {
        let a3962 = v0;
        let a3963 = 42u64;
        nat_eq(a3962, a3963)
    } {
        true
    } else {
        let a3964 = v0;
        let a3965 = 43u64;
        nat_eq(a3964, a3965)
    }
}

pub fn f180(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let a3966 = v0;
    let a3967 = 0u64;
    if if if nat_eq(a3966, a3967) {
        true
    } else {
        let a3968 = v1;
        let a3969 = 0u64;
        nat_eq(a3968, a3969)
    } {
        true
    } else {
        let a3970 = v2;
        let a3971 = 0u64;
        nat_eq(a3970, a3971)
    } {
        Ok(0u64)
    } else {
        let a3974 = {
            let a3972 = v0;
            let a3973 = v1;
            nat_mul(a3972, a3973)?
        };
        let a3975 = v2;
        nat_mul(a3974, a3975)
    }
}

pub fn f181(v0: u64) -> bool {
    let m3976 = f91(v0);
    match m3976 {
        Adt14::C0 => {
            false
        }
        Adt14::C1(_) => {
            true
        }
    }
}

pub fn f182(v0: u64, v1: u64) -> bool {
    let a3977 = v0;
    let a3978 = 1u64;
    if nat_eq(a3977, a3978) {
        true
    } else {
        let v2: u64 = f185(v0);
        let a3979 = 0u64;
        let a3980 = v2;
        if nat_lt(a3979, a3980) {
            let a3984 = {
                let a3981 = v1;
                let a3982 = v2;
                let a3983 = 1u64;
                nat_rem(a3981, a3982, a3983)
            };
            let a3985 = 0u64;
            nat_eq(a3984, a3985)
        } else {
            false
        }
    }
}

pub fn f183(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64, v7: u64, v8: u64, v9: u64, v10: u64, v11: u64, v12: u64, v13: u64, v14: u64, v15: u64) -> R<Adt16> {
    let a3986 = v0;
    let a3987 = 28u64;
    if if if if nat_eq(a3986, a3987) {
        let a3988 = v1;
        let a3989 = 0u64;
        nat_eq(a3988, a3989)
    } else {
        false
    } {
        let a3990 = v2;
        let a3991 = 0u64;
        nat_eq(a3990, a3991)
    } else {
        false
    } {
        let a3992 = v3;
        let a3993 = 0u64;
        nat_eq(a3992, a3993)
    } else {
        false
    } {
        let a3994 = v4;
        let a3995 = v8;
        if if if if if if if if nat_eq(a3994, a3995) {
            let a3996 = v5;
            let a3997 = v12;
            nat_eq(a3996, a3997)
        } else {
            false
        } {
            let a3998 = v9;
            let a3999 = v13;
            nat_eq(a3998, a3999)
        } else {
            false
        } {
            let a4000 = v10;
            let a4001 = v14;
            nat_eq(a4000, a4001)
        } else {
            false
        } {
            let a4002 = v11;
            let a4003 = v15;
            nat_eq(a4002, a4003)
        } else {
            false
        } {
            let a4004 = 0u64;
            let a4005 = v6;
            nat_lt(a4004, a4005)
        } else {
            false
        } {
            let a4006 = 0u64;
            let a4007 = v7;
            nat_lt(a4006, a4007)
        } else {
            false
        } {
            let a4011 = {
                let a4008 = v10;
                let a4009 = v6;
                let a4010 = 1u64;
                nat_rem(a4008, a4009, a4010)
            };
            let a4012 = 0u64;
            if nat_eq(a4011, a4012) {
                let a4016 = {
                    let a4013 = v11;
                    let a4014 = v7;
                    let a4015 = 1u64;
                    nat_rem(a4013, a4014, a4015)
                };
                let a4017 = 0u64;
                nat_eq(a4016, a4017)
            } else {
                false
            }
        } else {
            false
        } {
            Ok(Adt16::C2(v5, v4, v9, v10, v11, {
                let a4018 = v10;
                let a4019 = v6;
                let a4020 = 0u64;
                nat_quot(a4018, a4019, a4020)
            }, {
                let a4021 = v11;
                let a4022 = v7;
                let a4023 = 0u64;
                nat_quot(a4021, a4022, a4023)
            }, {
                let a4024 = v5;
                let a4025 = v4;
                nat_mul(a4024, a4025)?
            }, {
                let a4026 = v4;
                let a4027 = v9;
                nat_mul(a4026, a4027)?
            }, {
                let a4028 = v5;
                let a4029 = v9;
                nat_mul(a4028, a4029)?
            }))
        } else {
            Ok(Adt16::C1)
        }
    } else {
        Ok(Adt16::C0)
    }
}

pub fn f184(v0: u64) -> bool {
    let a4030 = v0;
    let a4031 = 0u64;
    if if nat_eq(a4030, a4031) {
        true
    } else {
        let a4032 = v0;
        let a4033 = 1u64;
        nat_eq(a4032, a4033)
    } {
        true
    } else {
        let a4034 = v0;
        let a4035 = 30u64;
        nat_eq(a4034, a4035)
    }
}

pub fn f185(v0: u64) -> u64 {
    if f179(v0) {
        let m4036 = f91(v0);
        match m4036 {
            Adt14::C0 => {
                0u64
            }
            Adt14::C1(v1) => {
                let a4038 = 1u64;
                let a4039 = match v1.clone() {
                    Adt15::C0(_, h4037, _) => {
                        h4037
                    }
                };
                if if nat_lt(a4038, a4039) {
                    true
                } else {
                    let a4040 = v0;
                    let a4041 = 43u64;
                    nat_eq(a4040, a4041)
                } {
                    match v1 {
                        Adt15::C0(_, h4042, _) => {
                            h4042
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
