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
        let a327 = 73u64;
        if nat_eq(a326, a327) {
            f34(v2, v3, v4)?
        } else {
            let a328 = v0;
            let a329 = 57u64;
            if if nat_eq(a328, a329) {
                true
            } else {
                let a330 = v0;
                let a331 = 58u64;
                nat_eq(a330, a331)
            } {
                let m332 = v3;
                match m332 {
                    None => {
                        f35({
                            let a333 = v0;
                            let a334 = 58u64;
                            nat_eq(a333, a334)
                        }, v1, v2, v4)?
                    }
                    Some(_) => {
                        Adt10::C1
                    }
                }
            } else {
                let a335 = v0;
                let a336 = 49u64;
                if nat_eq(a335, a336) {
                    f36(v1, v2, v3, v4)?
                } else {
                    let a337 = v0;
                    let a338 = 51u64;
                    if nat_eq(a337, a338) {
                        f37(v1, v2, v3, v4)?
                    } else {
                        let a339 = v0;
                        let a340 = 39u64;
                        if nat_eq(a339, a340) {
                            f38(v2, v3, v4)?
                        } else {
                            let a341 = v0;
                            let a342 = 77u64;
                            if nat_eq(a341, a342) {
                                f39(v2, v4)?
                            } else {
                                let a343 = v0;
                                let a344 = 40u64;
                                if nat_eq(a343, a344) {
                                    f40(v2, v3, v4)?
                                } else {
                                    if f41(v0) {
                                        f42(v0, v2, {
                                            let m345 = v3;
                                            match m345 {
                                                None => {
                                                    false
                                                }
                                                Some(_) => {
                                                    true
                                                }
                                            }
                                        }, v4)?
                                    } else {
                                        let a346 = v0;
                                        let a347 = 19u64;
                                        if nat_eq(a346, a347) {
                                            f43(v2, v4)?
                                        } else {
                                            let a348 = v0;
                                            let a349 = 21u64;
                                            if nat_eq(a348, a349) {
                                                f44(v5, v2, v3, v4)?
                                            } else {
                                                let a350 = v0;
                                                let a351 = 61u64;
                                                if if if nat_eq(a350, a351) {
                                                    true
                                                } else {
                                                    let a352 = v0;
                                                    let a353 = 62u64;
                                                    nat_eq(a352, a353)
                                                } {
                                                    true
                                                } else {
                                                    let a354 = v0;
                                                    let a355 = 63u64;
                                                    nat_eq(a354, a355)
                                                } {
                                                    f45(v0, v1, v2, v4)?
                                                } else {
                                                    let a356 = v0;
                                                    let a357 = 4u64;
                                                    if if nat_eq(a356, a357) {
                                                        true
                                                    } else {
                                                        let a358 = v0;
                                                        let a359 = 32u64;
                                                        nat_eq(a358, a359)
                                                    } {
                                                        f46(v0, v1, v2, v3, v4)?
                                                    } else {
                                                        let a360 = v0;
                                                        let a361 = 20u64;
                                                        if nat_eq(a360, a361) {
                                                            f47(v2, v4)?
                                                        } else {
                                                            let a362 = v0;
                                                            let a363 = 42u64;
                                                            if nat_eq(a362, a363) {
                                                                f48(4u64, v2, v4)?
                                                            } else {
                                                                let a364 = v0;
                                                                let a365 = 69u64;
                                                                if nat_eq(a364, a365) {
                                                                    let a366 = v5;
                                                                    let a367 = 4u64;
                                                                    if nat_lt(a366, a367) {
                                                                        f48(v5, v2, v4)?
                                                                    } else {
                                                                        Adt10::C1
                                                                    }
                                                                } else {
                                                                    let a368 = v0;
                                                                    let a369 = 70u64;
                                                                    if nat_eq(a368, a369) {
                                                                        f49(v5, v2, v4)?
                                                                    } else {
                                                                        let a370 = v0;
                                                                        let a371 = 75u64;
                                                                        if if nat_eq(a370, a371) {
                                                                            true
                                                                        } else {
                                                                            let a372 = v0;
                                                                            let a373 = 76u64;
                                                                            nat_eq(a372, a373)
                                                                        } {
                                                                            f50({
                                                                                let a374 = v0;
                                                                                let a375 = 76u64;
                                                                                nat_eq(a374, a375)
                                                                            }, v1, v2, v4)?
                                                                        } else {
                                                                            let a376 = v0;
                                                                            let a377 = 31u64;
                                                                            if if if nat_eq(a376, a377) {
                                                                                true
                                                                            } else {
                                                                                let a378 = v0;
                                                                                let a379 = 47u64;
                                                                                nat_eq(a378, a379)
                                                                            } {
                                                                                true
                                                                            } else {
                                                                                let a380 = v0;
                                                                                let a381 = 68u64;
                                                                                nat_eq(a380, a381)
                                                                            } {
                                                                                f51(v0, v1, v2, v4)?
                                                                            } else {
                                                                                let a382 = v0;
                                                                                let a383 = 15u64;
                                                                                if nat_eq(a382, a383) {
                                                                                    f52(v2, v4)?
                                                                                } else {
                                                                                    let a384 = v0;
                                                                                    let a385 = 17u64;
                                                                                    if nat_eq(a384, a385) {
                                                                                        f53(v2, v4)?
                                                                                    } else {
                                                                                        let a386 = v0;
                                                                                        let a387 = 18u64;
                                                                                        if nat_eq(a386, a387) {
                                                                                            f54(v2, v3, v4)?
                                                                                        } else {
                                                                                            let a388 = v0;
                                                                                            let a389 = 13u64;
                                                                                            if if if nat_eq(a388, a389) {
                                                                                                true
                                                                                            } else {
                                                                                                let a390 = v0;
                                                                                                let a391 = 14u64;
                                                                                                nat_eq(a390, a391)
                                                                                            } {
                                                                                                true
                                                                                            } else {
                                                                                                let a392 = v0;
                                                                                                let a393 = 16u64;
                                                                                                nat_eq(a392, a393)
                                                                                            } {
                                                                                                f55(v0, v2, v4)?
                                                                                            } else {
                                                                                                let m394 = f56(v0, v5)?;
                                                                                                match m394 {
                                                                                                    None => {
                                                                                                        f57(v0, v5, v2, v3, v4)?
                                                                                                    }
                                                                                                    Some(v8) => {
                                                                                                        f58(v8, v2, v4)?
                                                                                                    }
                                                                                                }
                                                                                            }
                                                                                        }
                                                                                    }
                                                                                }
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
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
    let a395 = v1;
    let a396 = 0u64;
    if nat_eq(a395, a396) {
        let a398 = v3;
        let a399 = match v0.clone() {
            Adt8::C0(_, _, _, h397, _, _, _, _, _, _, _, _, _, _, _) => {
                h397
            }
        };
        if nat_lt(a398, a399) {
            let v4: Adt3 = match v0.clone() {
                Adt8::C0(_, _, _, _, _, _, _, _, h400, _, _, _, _, _, _) => {
                    h400
                }
            };
            let v5: u64 = {
                let a402 = v3;
                let a403 = match v0.clone() {
                    Adt8::C0(_, _, h401, _, _, _, _, _, _, _, _, _, _, _, _) => {
                        h401
                    }
                };
                let a404 = 0u64;
                nat_quot(a402, a403, a404)
            };
            Ok(Adt9::C8(Some(f59(v4.clone(), f60(v5, v4.clone(), 0u64), f60(v5, v4.clone(), 1u64), f60(v5, v4, 2u64), f61(v0.clone(), v5, {
                let a406 = v3;
                let a407 = match v0 {
                    Adt8::C0(_, _, h405, _, _, _, _, _, _, _, _, _, _, _, _) => {
                        h405
                    }
                };
                let a408 = 0u64;
                nat_rem(a406, a407, a408)
            })?)?)))
        } else {
            Ok(Adt9::C0(Adt10::C1))
        }
    } else {
        let v6: Adt3 = match v0.clone() {
            Adt8::C0(_, _, _, _, _, _, _, _, _, _, h409, _, _, _, _) => {
                h409
            }
        };
        let v7: u64 = {
            let a410 = v1;
            let a411 = 1u64;
            if nat_eq(a410, a411) {
                match v0.clone() {
                    Adt8::C0(_, h412, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                        h412
                    }
                }
            } else {
                match v0.clone() {
                    Adt8::C0(_, _, h413, _, _, _, _, _, _, _, _, _, _, _, _) => {
                        h413
                    }
                }
            }
        };
        let v8: u64 = {
            let a417 = v2;
            let a418 = {
                let a414 = v3;
                let a415 = v7;
                let a416 = 0u64;
                nat_quot(a414, a415, a416)
            };
            nat_add(a417, a418)?
        };
        let v9: u64 = {
            let a419 = v3;
            let a420 = v7;
            let a421 = 0u64;
            nat_rem(a419, a420, a421)
        };
        let v10: u64 = {
            let a423 = v8;
            let a424 = match v6.clone() {
                Adt3::C0(h422, _, _, _, _, _, _, _) => {
                    h422
                }
            };
            let a425 = 0u64;
            nat_rem(a423, a424, a425)
        };
        let v11: u64 = {
            let a427 = v8;
            let a428 = match v6.clone() {
                Adt3::C0(h426, _, _, _, _, _, _, _) => {
                    h426
                }
            };
            let a429 = 0u64;
            nat_quot(a427, a428, a429)
        };
        let v12: u64 = {
            let a431 = v11;
            let a432 = match v6.clone() {
                Adt3::C0(_, h430, _, _, _, _, _, _) => {
                    h430
                }
            };
            let a433 = 0u64;
            nat_rem(a431, a432, a433)
        };
        let v13: u64 = {
            let a435 = v11;
            let a436 = match v6.clone() {
                Adt3::C0(_, h434, _, _, _, _, _, _) => {
                    h434
                }
            };
            let a437 = 0u64;
            nat_quot(a435, a436, a437)
        };
        let a439 = v8;
        let a440 = match v0.clone() {
            Adt8::C0(h438, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h438
            }
        };
        if nat_lt(a439, a440) {
            let a441 = v1;
            let a442 = 1u64;
            if nat_eq(a441, a442) {
                let v14: Adt3 = match v0.clone() {
                    Adt8::C0(_, _, _, _, _, _, _, _, h443, _, _, _, _, _, _) => {
                        h443
                    }
                };
                let v15: Adt3 = match v0.clone() {
                    Adt8::C0(_, _, _, _, _, _, _, _, _, h444, _, _, _, _, _) => {
                        h444
                    }
                };
                let m447 = f62(match v0.clone() {
                    Adt8::C0(_, _, _, _, _, _, _, _, _, _, _, h445, _, _, _) => {
                        h445
                    }
                }, match v15.clone() {
                    Adt3::C0(h446, _, _, _, _, _, _, _) => {
                        h446
                    }
                }, v10, f60(v9, v14.clone(), 0u64))?;
                match m447 {
                    None => {
                        Ok(Adt9::C8(None::<u64>))
                    }
                    Some(v16) => {
                        let m450 = f62(match v0.clone() {
                            Adt8::C0(_, _, _, _, _, _, _, _, _, _, _, _, h448, _, _) => {
                                h448
                            }
                        }, match v15 {
                            Adt3::C0(_, h449, _, _, _, _, _, _) => {
                                h449
                            }
                        }, v12, f60(v9, v14, 1u64))?;
                        match m450 {
                            None => {
                                Ok(Adt9::C8(None::<u64>))
                            }
                            Some(v17) => {
                                f63(v0, v9, v13, v16, v17)
                            }
                        }
                    }
                }
            } else {
                let a451 = v1;
                let a452 = 2u64;
                if nat_eq(a451, a452) {
                    Ok(Adt9::C8(Some({
                        let a454 = match v0.clone() {
                            Adt8::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, h453) => {
                                h453
                            }
                        };
                        let a455 = 0u64;
                        if nat_eq(a454, a455) {
                            f59(v6, v10, v12, v9, v13)?
                        } else {
                            f59(v6.clone(), v10, v12, {
                                let a457 = v13;
                                let a458 = match v6.clone() {
                                    Adt3::C0(_, _, h456, _, _, _, _, _) => {
                                        h456
                                    }
                                };
                                let a459 = 0u64;
                                nat_rem(a457, a458, a459)
                            }, {
                                let a467 = {
                                    let a465 = {
                                        let a461 = v13;
                                        let a462 = match v6 {
                                            Adt3::C0(_, _, h460, _, _, _, _, _) => {
                                                h460
                                            }
                                        };
                                        let a463 = 0u64;
                                        nat_quot(a461, a462, a463)
                                    };
                                    let a466 = match v0 {
                                        Adt8::C0(_, _, h464, _, _, _, _, _, _, _, _, _, _, _, _) => {
                                            h464
                                        }
                                    };
                                    nat_mul(a465, a466)?
                                };
                                let a468 = v9;
                                nat_add(a467, a468)?
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
    Ok(f64(v3.clone(), {
        let m469 = v3.clone();
        match m469 {
            Adt4::C0 => {
                f65(v0, v1, v2, v3)?
            }
            Adt4::C1 => {
                f65(v0, v1, v2, v3)?
            }
            Adt4::C2(_, _) => {
                f65(v0, v1, v2, v3)?
            }
            Adt4::C3(_, _, _, _, _) => {
                f65(v0, v1, v2, v3)?
            }
            Adt4::C4(_) => {
                f65(v0, v1, v2, v3)?
            }
            Adt4::C5(_, _, _, _) => {
                f65(v0, v1, v2, v3)?
            }
            Adt4::C6(_, _, _) => {
                f65(v0, v1, v2, v3)?
            }
            Adt4::C7(_) => {
                f65(v0, v1, v2, v3)?
            }
            Adt4::C8(_) => {
                f65(v0, v1, v2, v3)?
            }
            Adt4::C9(_, _) => {
                f65(v0, v1, v2, v3)?
            }
            Adt4::C10(_, _, _) => {
                f65(v0, v1, v2, v3)?
            }
            Adt4::C11(_, _) => {
                f65(v0, v1, v2, v3)?
            }
            Adt4::C12(_, _, _) => {
                f65(v0, v1, v2, v3)?
            }
            Adt4::C13(_, _, _, _) => {
                f65(v0, v1, v2, v3)?
            }
            Adt4::C14(_, _, _) => {
                f65(v0, v1, v2, v3)?
            }
            Adt4::C15 => {
                f65(v0, v1, v2, v3)?
            }
            Adt4::C16(_, _) => {
                f65(v0, v1, v2, v3)?
            }
            Adt4::C17 => {
                f66(v0, v1, v2, v4)?
            }
            Adt4::C18(_, _) => {
                f65(v0, v1, v2, v3)?
            }
            Adt4::C19(_, v44) => {
                f67(v0, v1, v2, v44, v4)?
            }
            Adt4::C20(v45, v46) => {
                f68(v45, v0, v1, v2, v46, v4)?
            }
            Adt4::C21 => {
                f65(v0, v1, v2, v3)?
            }
            Adt4::C22(v47, v48) => {
                f69(v0, v1, v2, v47, v48, v4)?
            }
            Adt4::C23(v60, v61, v62, v63) => {
                f74(v0, v63, v60, v61, v62, v4)?
            }
            Adt4::C24(_) => {
                Adt9::C0(Adt10::C5(65u64))
            }
            Adt4::C25(_, _, _) => {
                f65(v0, v1, v2, v3)?
            }
            Adt4::C26(v55, v56, v57, v58) => {
                let a470 = v55;
                let a471 = 64u64;
                if if if nat_eq(a470, a471) {
                    true
                } else {
                    let a472 = v55;
                    let a473 = 65u64;
                    nat_eq(a472, a473)
                } {
                    true
                } else {
                    let a474 = v55;
                    let a475 = 66u64;
                    nat_eq(a474, a475)
                } {
                    Adt9::C0(Adt10::C5(64u64))
                } else {
                    let a476 = v55;
                    let a477 = 0u64;
                    if if nat_eq(a476, a477) {
                        true
                    } else {
                        let a478 = v55;
                        let a479 = 30u64;
                        nat_eq(a478, a479)
                    } {
                        f73(v0, v1, v2, v55, v56, v57, v58, v4)?
                    } else {
                        Adt9::C0(Adt10::C1)
                    }
                }
            }
            Adt4::C27(v54) => {
                f72(v0, v1, v2, v54)?
            }
            Adt4::C28(v52) => {
                f70(v0, v1, v2, v52, v4)?
            }
            Adt4::C29(v53) => {
                f71(v0, v1, v2, v53)?
            }
        }
    }))
}

pub fn f10(v0: Adt4) -> R<Adt9> {
    let m480 = v0;
    match m480 {
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
                let a485 = {
                    let a481 = 0u64;
                    let a482 = v45;
                    if nat_lt(a481, a482) {
                        1u64
                    } else {
                        0u64
                    }
                };
                let a486 = {
                    let a483 = 0u64;
                    let a484 = v47;
                    if nat_lt(a483, a484) {
                        6u64
                    } else {
                        0u64
                    }
                };
                nat_add(a485, a486)?
            }, 0u64, 0u64, false))
        }
        Adt4::C26(v51, _, _, _) => {
            Ok(Adt9::C6({
                let a487 = v51;
                let a488 = 66u64;
                if nat_eq(a487, a488) {
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
    }
}

pub fn f11(v0: bool, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64, v7: u64, v8: u64, v9: u64, v10: u64, v11: u64, v12: u64) -> Adt9 {
    let a489 = v1;
    let a490 = 5u64;
    if if if if nat_lt(a489, a490) {
        let a491 = v5;
        let a492 = 5u64;
        nat_lt(a491, a492)
    } else {
        false
    } {
        let a493 = v7;
        let a494 = 5u64;
        nat_lt(a493, a494)
    } else {
        false
    } {
        let a495 = v9;
        let a496 = 2u64;
        if nat_lt(a495, a496) {
            true
        } else {
            let a497 = v9;
            let a498 = 3u64;
            nat_eq(a497, a498)
        }
    } else {
        false
    } {
        Adt9::C7(f75(v0, v1, v2, v3, v4), f76(v5, v6), f76(v7, v8), f77({
            let a499 = v9;
            let a500 = 0u64;
            if nat_eq(a499, a500) {
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
    let a512 = {
        let a506 = {
            let a502 = match v0.clone() {
                Adt2::C0(_, h501, _, _, _, _, _, _, _, _, _) => {
                    h501
                }
            };
            let a503 = v1;
            let a504 = 0u64;
            nat_quot(a502, a503, a504)
        };
        let a507 = match v0.clone() {
            Adt2::C0(_, _, h505, _, _, _, _, _, _, _, _) => {
                h505
            }
        };
        nat_mul(a506, a507)?
    };
    let a513 = {
        let a510 = match v0.clone() {
            Adt2::C0(_, _, _, h508, _, _, _, _, _, _, _) => {
                h508
            }
        };
        let a511 = match v0 {
            Adt2::C0(_, _, _, _, h509, _, _, _, _, _, _) => {
                h509
            }
        };
        nat_mul(a510, a511)?
    };
    nat_mul(a512, a513)
}

pub fn f13(v0: u64) -> bool {
    let a514 = v0;
    let a515 = 4294967296u64;
    if nat_lt(a514, a515) {
        let a519 = {
            let a516 = v0;
            let a517 = 2147483648u64;
            let a518 = 0u64;
            nat_rem(a516, a517, a518)
        };
        let a520 = 2139095040u64;
        nat_lt(a519, a520)
    } else {
        false
    }
}

pub fn f14(v0: u64) -> R<u64> {
    let a521 = 4u64;
    let a522 = v0;
    nat_mul(a521, a522)
}

pub fn f15(v0: u64) -> R<u64> {
    let a523 = v0;
    let a524 = 0u64;
    if nat_eq(a523, a524) {
        Ok(0u64)
    } else {
        let a527 = 4u64;
        let a528 = {
            let a525 = 3u64;
            let a526 = v0;
            nat_mul(a525, a526)?
        };
        nat_add(a527, a528)
    }
}

pub fn f16(v0: Adt2, v1: u64) -> Adt3 {
    Adt3::C0({
        let a530 = match v0.clone() {
            Adt2::C0(_, h529, _, _, _, _, _, _, _, _, _) => {
                h529
            }
        };
        let a531 = v1;
        let a532 = 0u64;
        nat_quot(a530, a531, a532)
    }, match v0.clone() {
        Adt2::C0(_, _, h533, _, _, _, _, _, _, _, _) => {
            h533
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, h534, _, _, _, _, _, _, _) => {
            h534
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, h535, _, _, _, _, _, _) => {
            h535
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, _, h536, _, _, _, _, _) => {
            h536
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, _, _, h537, _, _, _, _) => {
            h537
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, _, _, _, h538, _, _, _) => {
            h538
        }
    }, match v0 {
        Adt2::C0(_, _, _, _, _, _, _, _, h539, _, _) => {
            h539
        }
    })
}

pub fn f17(v0: Adt6, v1: u64) -> R<u64> {
    let a541 = match v0.clone() {
        Adt6::C0(h540, _, _, _) => {
            h540
        }
    };
    let a542 = 0u64;
    if nat_eq(a541, a542) {
        f78(Adt6::C0(1u64, match v0.clone() {
            Adt6::C0(_, h543, _, _) => {
                h543
            }
        }, match v0.clone() {
            Adt6::C0(_, _, h544, _) => {
                h544
            }
        }, match v0.clone() {
            Adt6::C0(_, _, _, h545) => {
                h545
            }
        }), {
            let a547 = v1;
            let a548 = match v0 {
                Adt6::C0(_, _, _, h546) => {
                    h546
                }
            };
            nat_add(a547, a548)?
        })
    } else {
        f78(v0, v1)
    }
}

pub fn f18(v0: Adt2, v1: Adt2) -> R<bool> {
    let a550 = match v0.clone() {
        Adt2::C0(_, _, _, _, _, _, _, _, _, h549, _) => {
            h549
        }
    };
    let a551 = 0u64;
    if nat_eq(a550, a551) {
        Ok(true)
    } else {
        let a553 = match v1.clone() {
            Adt2::C0(_, _, _, _, _, _, _, _, _, h552, _) => {
                h552
            }
        };
        let a554 = 0u64;
        if nat_eq(a553, a554) {
            Ok(true)
        } else {
            let a556 = match v0.clone() {
                Adt2::C0(_, _, _, _, _, _, _, _, _, _, h555) => {
                    h555
                }
            };
            let a557 = 0u64;
            if nat_eq(a556, a557) {
                Ok(true)
            } else {
                let a559 = match v1.clone() {
                    Adt2::C0(_, _, _, _, _, _, _, _, _, _, h558) => {
                        h558
                    }
                };
                let a560 = 0u64;
                if nat_eq(a559, a560) {
                    Ok(true)
                } else {
                    let a566 = {
                        let a563 = match v0.clone() {
                            Adt2::C0(_, _, _, _, _, _, _, _, _, _, h561) => {
                                h561
                            }
                        };
                        let a564 = match v0.clone() {
                            Adt2::C0(_, _, _, _, _, _, _, _, _, h562, _) => {
                                h562
                            }
                        };
                        nat_add(a563, a564)?
                    };
                    let a567 = match v1.clone() {
                        Adt2::C0(_, _, _, _, _, _, _, _, _, _, h565) => {
                            h565
                        }
                    };
                    if nat_le(a566, a567) {
                        Ok(true)
                    } else {
                        let a573 = {
                            let a570 = match v1.clone() {
                                Adt2::C0(_, _, _, _, _, _, _, _, _, _, h568) => {
                                    h568
                                }
                            };
                            let a571 = match v1 {
                                Adt2::C0(_, _, _, _, _, _, _, _, _, h569, _) => {
                                    h569
                                }
                            };
                            nat_add(a570, a571)?
                        };
                        let a574 = match v0 {
                            Adt2::C0(_, _, _, _, _, _, _, _, _, _, h572) => {
                                h572
                            }
                        };
                        Ok(nat_le(a573, a574))
                    }
                }
            }
        }
    }
}

pub fn f19(v0: Adt2, v1: Adt2) -> bool {
    let a577 = match v0.clone() {
        Adt2::C0(_, h575, _, _, _, _, _, _, _, _, _) => {
            h575
        }
    };
    let a578 = match v1.clone() {
        Adt2::C0(_, h576, _, _, _, _, _, _, _, _, _) => {
            h576
        }
    };
    if if if nat_eq(a577, a578) {
        let a581 = match v0.clone() {
            Adt2::C0(_, _, h579, _, _, _, _, _, _, _, _) => {
                h579
            }
        };
        let a582 = match v1.clone() {
            Adt2::C0(_, _, h580, _, _, _, _, _, _, _, _) => {
                h580
            }
        };
        nat_eq(a581, a582)
    } else {
        false
    } {
        let a585 = match v0.clone() {
            Adt2::C0(_, _, _, h583, _, _, _, _, _, _, _) => {
                h583
            }
        };
        let a586 = match v1.clone() {
            Adt2::C0(_, _, _, h584, _, _, _, _, _, _, _) => {
                h584
            }
        };
        nat_eq(a585, a586)
    } else {
        false
    } {
        let a589 = match v0 {
            Adt2::C0(_, _, _, _, h587, _, _, _, _, _, _) => {
                h587
            }
        };
        let a590 = match v1 {
            Adt2::C0(_, _, _, _, h588, _, _, _, _, _, _) => {
                h588
            }
        };
        nat_eq(a589, a590)
    } else {
        false
    }
}

pub fn f20(v0: Adt2) -> R<bool> {
    let a592 = match v0.clone() {
        Adt2::C0(_, h591, _, _, _, _, _, _, _, _, _) => {
            h591
        }
    };
    let a593 = 1u64;
    if if if if nat_eq(a592, a593) {
        true
    } else {
        let a595 = match v0.clone() {
            Adt2::C0(_, _, _, _, _, h594, _, _, _, _, _) => {
                h594
            }
        };
        let a596 = 4u64;
        nat_eq(a595, a596)
    } {
        let a598 = match v0.clone() {
            Adt2::C0(_, _, h597, _, _, _, _, _, _, _, _) => {
                h597
            }
        };
        let a599 = 1u64;
        if nat_eq(a598, a599) {
            true
        } else {
            let a604 = match v0.clone() {
                Adt2::C0(_, _, _, _, _, _, h600, _, _, _, _) => {
                    h600
                }
            };
            let a605 = {
                let a602 = match v0.clone() {
                    Adt2::C0(_, h601, _, _, _, _, _, _, _, _, _) => {
                        h601
                    }
                };
                let a603 = 4u64;
                nat_mul(a602, a603)?
            };
            nat_eq(a604, a605)
        }
    } else {
        false
    } {
        let a607 = match v0.clone() {
            Adt2::C0(_, _, _, h606, _, _, _, _, _, _, _) => {
                h606
            }
        };
        let a608 = 1u64;
        if nat_eq(a607, a608) {
            true
        } else {
            let a616 = match v0.clone() {
                Adt2::C0(_, _, _, _, _, _, _, h609, _, _, _) => {
                    h609
                }
            };
            let a617 = {
                let a614 = {
                    let a612 = match v0.clone() {
                        Adt2::C0(_, h610, _, _, _, _, _, _, _, _, _) => {
                            h610
                        }
                    };
                    let a613 = match v0.clone() {
                        Adt2::C0(_, _, h611, _, _, _, _, _, _, _, _) => {
                            h611
                        }
                    };
                    nat_mul(a612, a613)?
                };
                let a615 = 4u64;
                nat_mul(a614, a615)?
            };
            nat_eq(a616, a617)
        }
    } else {
        false
    } {
        let a619 = match v0.clone() {
            Adt2::C0(_, _, _, _, h618, _, _, _, _, _, _) => {
                h618
            }
        };
        let a620 = 1u64;
        if nat_eq(a619, a620) {
            Ok(true)
        } else {
            let a631 = match v0.clone() {
                Adt2::C0(_, _, _, _, _, _, _, _, h621, _, _) => {
                    h621
                }
            };
            let a632 = {
                let a629 = {
                    let a627 = {
                        let a624 = match v0.clone() {
                            Adt2::C0(_, h622, _, _, _, _, _, _, _, _, _) => {
                                h622
                            }
                        };
                        let a625 = match v0.clone() {
                            Adt2::C0(_, _, h623, _, _, _, _, _, _, _, _) => {
                                h623
                            }
                        };
                        nat_mul(a624, a625)?
                    };
                    let a628 = match v0 {
                        Adt2::C0(_, _, _, h626, _, _, _, _, _, _, _) => {
                            h626
                        }
                    };
                    nat_mul(a627, a628)?
                };
                let a630 = 4u64;
                nat_mul(a629, a630)?
            };
            Ok(nat_eq(a631, a632))
        }
    } else {
        Ok(false)
    }
}

pub fn f21(v0: u64, v1: u64, v2: u64, v3: u64) -> R<u64> {
    let a637 = {
        let a633 = v0;
        let a634 = v1;
        nat_mul(a633, a634)?
    };
    let a638 = {
        let a635 = v2;
        let a636 = f79(v3);
        nat_mul(a635, a636)?
    };
    nat_mul(a637, a638)
}

pub fn f22(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let a641 = v0;
    let a642 = {
        let a639 = f80(v1)?;
        let a640 = v2;
        nat_mul(a639, a640)?
    };
    nat_add(a641, a642)
}

pub fn f23(v0: u64) -> u64 {
    let a643 = v0;
    let a644 = 30u64;
    if nat_eq(a643, a644) {
        2u64
    } else {
        4u64
    }
}

pub fn f24(v0: u64) -> bool {
    let a645 = v0;
    let a646 = 2139095040u64;
    if nat_le(a645, a646) {
        true
    } else {
        let a647 = v0;
        let a648 = 2147483648u64;
        nat_eq(a647, a648)
    }
}

pub fn f25(v0: Adt2, v1: u64) -> bool {
    let a650 = match v0.clone() {
        Adt2::C0(_, _, _, _, _, h649, _, _, _, _, _) => {
            h649
        }
    };
    let a651 = v1;
    if if nat_eq(a650, a651) {
        let a656 = {
            let a653 = match v0.clone() {
                Adt2::C0(_, _, _, _, _, _, _, _, _, _, h652) => {
                    h652
                }
            };
            let a654 = v1;
            let a655 = 0u64;
            nat_rem(a653, a654, a655)
        };
        let a657 = 0u64;
        nat_eq(a656, a657)
    } else {
        false
    } {
        f81(match v0.clone() {
            Adt2::C0(_, _, h658, _, _, _, _, _, _, _, _) => {
                h658
            }
        }, match v0.clone() {
            Adt2::C0(_, _, _, h659, _, _, _, _, _, _, _) => {
                h659
            }
        }, match v0.clone() {
            Adt2::C0(_, _, _, _, h660, _, _, _, _, _, _) => {
                h660
            }
        }, match v0.clone() {
            Adt2::C0(_, _, _, _, _, _, h661, _, _, _, _) => {
                h661
            }
        }, match v0.clone() {
            Adt2::C0(_, _, _, _, _, _, _, h662, _, _, _) => {
                h662
            }
        }, match v0 {
            Adt2::C0(_, _, _, _, _, _, _, _, h663, _, _) => {
                h663
            }
        }, v1)
    } else {
        false
    }
}

pub fn f26(v0: Adt2, v1: Adt2) -> bool {
    if if if if if if f19(v0.clone(), v1.clone()) {
        let a666 = match v0.clone() {
            Adt2::C0(h664, _, _, _, _, _, _, _, _, _, _) => {
                h664
            }
        };
        let a667 = match v1.clone() {
            Adt2::C0(h665, _, _, _, _, _, _, _, _, _, _) => {
                h665
            }
        };
        nat_eq(a666, a667)
    } else {
        false
    } {
        let a670 = match v0.clone() {
            Adt2::C0(_, _, _, _, _, _, _, _, _, _, h668) => {
                h668
            }
        };
        let a671 = match v1.clone() {
            Adt2::C0(_, _, _, _, _, _, _, _, _, _, h669) => {
                h669
            }
        };
        nat_eq(a670, a671)
    } else {
        false
    } {
        let a674 = match v0.clone() {
            Adt2::C0(_, _, _, _, _, h672, _, _, _, _, _) => {
                h672
            }
        };
        let a675 = match v1.clone() {
            Adt2::C0(_, _, _, _, _, h673, _, _, _, _, _) => {
                h673
            }
        };
        nat_eq(a674, a675)
    } else {
        false
    } {
        let a678 = match v0.clone() {
            Adt2::C0(_, _, _, _, _, _, h676, _, _, _, _) => {
                h676
            }
        };
        let a679 = match v1.clone() {
            Adt2::C0(_, _, _, _, _, _, h677, _, _, _, _) => {
                h677
            }
        };
        nat_eq(a678, a679)
    } else {
        false
    } {
        let a682 = match v0.clone() {
            Adt2::C0(_, _, _, _, _, _, _, h680, _, _, _) => {
                h680
            }
        };
        let a683 = match v1.clone() {
            Adt2::C0(_, _, _, _, _, _, _, h681, _, _, _) => {
                h681
            }
        };
        nat_eq(a682, a683)
    } else {
        false
    } {
        let a686 = match v0 {
            Adt2::C0(_, _, _, _, _, _, _, _, h684, _, _) => {
                h684
            }
        };
        let a687 = match v1 {
            Adt2::C0(_, _, _, _, _, _, _, _, h685, _, _) => {
                h685
            }
        };
        nat_eq(a686, a687)
    } else {
        false
    }
}

pub fn f27(v0: u64, v1: u64, v2: u64, v3: u64) -> bool {
    let a688 = v0;
    let a689 = 0u64;
    if if if nat_eq(a688, a689) {
        true
    } else {
        let a690 = v1;
        let a691 = 0u64;
        nat_eq(a690, a691)
    } {
        true
    } else {
        let a692 = v2;
        let a693 = 0u64;
        nat_eq(a692, a693)
    } {
        true
    } else {
        let a694 = v3;
        let a695 = 0u64;
        nat_eq(a694, a695)
    }
}

pub fn f28(v0: Adt2, v1: u64) -> R<bool> {
    f82({
        let a697 = match v0.clone() {
            Adt2::C0(_, h696, _, _, _, _, _, _, _, _, _) => {
                h696
            }
        };
        let a698 = v1;
        nat_mul(a697, a698)?
    }, match v0.clone() {
        Adt2::C0(_, _, h699, _, _, _, _, _, _, _, _) => {
            h699
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, h700, _, _, _, _, _, _, _) => {
            h700
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, h701, _, _, _, _, _, _) => {
            h701
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, _, _, h702, _, _, _, _) => {
            h702
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, _, _, _, h703, _, _, _) => {
            h703
        }
    }, match v0 {
        Adt2::C0(_, _, _, _, _, _, _, _, h704, _, _) => {
            h704
        }
    })
}

pub fn f29(v0: u64, v1: u64, v2: u64, v3: u64) -> R<u64> {
    let a705 = v1;
    let a706 = 0u64;
    if if nat_eq(a705, a706) {
        true
    } else {
        let a707 = v2;
        let a708 = 0u64;
        nat_eq(a707, a708)
    } {
        Ok(0u64)
    } else {
        let a715 = {
            let a709 = f83(v1, v2, v3)?;
            let a710 = f84(v0, v1, v2)?;
            nat_add(a709, a710)?
        };
        let a716 = {
            let a713 = f85(v0);
            let a714 = {
                let a711 = v1;
                let a712 = v2;
                nat_mul(a711, a712)?
            };
            nat_mul(a713, a714)?
        };
        nat_add(a715, a716)
    }
}

pub fn f30(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let a717 = v0;
    let a718 = 0u64;
    if if nat_eq(a717, a718) {
        true
    } else {
        let a719 = v1;
        let a720 = 0u64;
        nat_eq(a719, a720)
    } {
        Ok(3u64)
    } else {
        let a723 = {
            let a721 = 3u64;
            let a722 = v1;
            nat_add(a721, a722)?
        };
        let a724 = v2;
        nat_add(a723, a724)
    }
}

pub fn f31(v0: u64) -> bool {
    let a725 = v0;
    let a726 = 2139095040u64;
    if nat_le(a725, a726) {
        true
    } else {
        let a727 = v0;
        let a728 = 2147483648u64;
        nat_eq(a727, a728)
    }
}

pub fn f32(v0: u64, v1: u64) -> R<u64> {
    let a729 = v0;
    let a730 = 0u64;
    if nat_eq(a729, a730) {
        Ok(0u64)
    } else {
        let a731 = v1;
        let a732 = f86(v0)?;
        nat_mul(a731, a732)
    }
}

pub fn f33(v0: u64, v1: u64, v2: u64, v3: u64) -> R<u64> {
    let a733 = v0;
    let a734 = 0u64;
    if if if if nat_eq(a733, a734) {
        true
    } else {
        let a735 = v1;
        let a736 = 0u64;
        nat_eq(a735, a736)
    } {
        true
    } else {
        let a737 = v2;
        let a738 = 0u64;
        nat_eq(a737, a738)
    } {
        true
    } else {
        let a739 = v3;
        let a740 = 0u64;
        nat_eq(a739, a740)
    } {
        Ok(0u64)
    } else {
        let a753 = {
            let a743 = 2u64;
            let a744 = {
                let a741 = v1;
                let a742 = v2;
                nat_mul(a741, a742)?
            };
            nat_mul(a743, a744)?
        };
        let a754 = {
            let a751 = {
                let a747 = {
                    let a745 = v1;
                    let a746 = v2;
                    nat_mul(a745, a746)?
                };
                let a748 = v3;
                nat_mul(a747, a748)?
            };
            let a752 = {
                let a749 = 1u64;
                let a750 = v0;
                nat_add(a749, a750)?
            };
            nat_mul(a751, a752)?
        };
        nat_add(a753, a754)
    }
}

pub fn f34(v0: Adt2, v1: Option<Adt2>, v2: Adt2) -> R<Adt10> {
    let m755 = v1;
    match m755 {
        None => {
            Ok(Adt10::C1)
        }
        Some(v3) => {
            let a757 = match v0.clone() {
                Adt2::C0(h756, _, _, _, _, _, _, _, _, _, _) => {
                    h756
                }
            };
            let a758 = 0u64;
            if if if if if if if if if if if if if if if if if if if nat_eq(a757, a758) {
                let a760 = match v3.clone() {
                    Adt2::C0(h759, _, _, _, _, _, _, _, _, _, _) => {
                        h759
                    }
                };
                let a761 = 0u64;
                nat_eq(a760, a761)
            } else {
                false
            } {
                let a763 = match v2.clone() {
                    Adt2::C0(h762, _, _, _, _, _, _, _, _, _, _) => {
                        h762
                    }
                };
                let a764 = 0u64;
                nat_eq(a763, a764)
            } else {
                false
            } {
                let a766 = match v0.clone() {
                    Adt2::C0(_, _, _, _, h765, _, _, _, _, _, _) => {
                        h765
                    }
                };
                let a767 = 1u64;
                nat_eq(a766, a767)
            } else {
                false
            } {
                let a769 = match v3.clone() {
                    Adt2::C0(_, _, _, h768, _, _, _, _, _, _, _) => {
                        h768
                    }
                };
                let a770 = 1u64;
                nat_eq(a769, a770)
            } else {
                false
            } {
                let a772 = match v3.clone() {
                    Adt2::C0(_, _, _, _, h771, _, _, _, _, _, _) => {
                        h771
                    }
                };
                let a773 = 1u64;
                nat_eq(a772, a773)
            } else {
                false
            } {
                let a776 = match v0.clone() {
                    Adt2::C0(_, _, h774, _, _, _, _, _, _, _, _) => {
                        h774
                    }
                };
                let a777 = match v3.clone() {
                    Adt2::C0(_, _, h775, _, _, _, _, _, _, _, _) => {
                        h775
                    }
                };
                nat_eq(a776, a777)
            } else {
                false
            } {
                let a782 = match v3.clone() {
                    Adt2::C0(_, h778, _, _, _, _, _, _, _, _, _) => {
                        h778
                    }
                };
                let a783 = {
                    let a780 = match v0.clone() {
                        Adt2::C0(_, h779, _, _, _, _, _, _, _, _, _) => {
                            h779
                        }
                    };
                    let a781 = 1u64;
                    nat_add(a780, a781)?
                };
                nat_le(a782, a783)
            } else {
                false
            } {
                let a786 = match v2.clone() {
                    Adt2::C0(_, h784, _, _, _, _, _, _, _, _, _) => {
                        h784
                    }
                };
                let a787 = match v0.clone() {
                    Adt2::C0(_, _, h785, _, _, _, _, _, _, _, _) => {
                        h785
                    }
                };
                nat_eq(a786, a787)
            } else {
                false
            } {
                let a795 = match v2.clone() {
                    Adt2::C0(_, _, h788, _, _, _, _, _, _, _, _) => {
                        h788
                    }
                };
                let a796 = {
                    let a793 = {
                        let a790 = match v0.clone() {
                            Adt2::C0(_, h789, _, _, _, _, _, _, _, _, _) => {
                                h789
                            }
                        };
                        let a791 = 1u64;
                        nat_add(a790, a791)?
                    };
                    let a794 = match v3.clone() {
                        Adt2::C0(_, h792, _, _, _, _, _, _, _, _, _) => {
                            h792
                        }
                    };
                    nat_sub(a793, a794)
                };
                nat_eq(a795, a796)
            } else {
                false
            } {
                let a799 = match v2.clone() {
                    Adt2::C0(_, _, _, h797, _, _, _, _, _, _, _) => {
                        h797
                    }
                };
                let a800 = match v0.clone() {
                    Adt2::C0(_, _, _, h798, _, _, _, _, _, _, _) => {
                        h798
                    }
                };
                nat_eq(a799, a800)
            } else {
                false
            } {
                let a802 = match v2.clone() {
                    Adt2::C0(_, _, _, _, h801, _, _, _, _, _, _) => {
                        h801
                    }
                };
                let a803 = 1u64;
                nat_eq(a802, a803)
            } else {
                false
            } {
                f20(v2.clone())?
            } else {
                false
            } {
                let a805 = match v0.clone() {
                    Adt2::C0(_, _, _, _, _, h804, _, _, _, _, _) => {
                        h804
                    }
                };
                let a806 = 4u64;
                nat_eq(a805, a806)
            } else {
                false
            } {
                let a808 = match v3.clone() {
                    Adt2::C0(_, _, _, _, _, h807, _, _, _, _, _) => {
                        h807
                    }
                };
                let a809 = 4u64;
                nat_eq(a808, a809)
            } else {
                false
            } {
                let a814 = match v0.clone() {
                    Adt2::C0(_, _, _, _, _, _, h810, _, _, _, _) => {
                        h810
                    }
                };
                let a815 = {
                    let a812 = match v0.clone() {
                        Adt2::C0(_, h811, _, _, _, _, _, _, _, _, _) => {
                            h811
                        }
                    };
                    let a813 = 4u64;
                    nat_mul(a812, a813)?
                };
                nat_eq(a814, a815)
            } else {
                false
            } {
                let a820 = match v3.clone() {
                    Adt2::C0(_, _, _, _, _, _, h816, _, _, _, _) => {
                        h816
                    }
                };
                let a821 = {
                    let a818 = match v3.clone() {
                        Adt2::C0(_, h817, _, _, _, _, _, _, _, _, _) => {
                            h817
                        }
                    };
                    let a819 = 4u64;
                    nat_mul(a818, a819)?
                };
                nat_eq(a820, a821)
            } else {
                false
            } {
                f18(v0.clone(), v2.clone())?
            } else {
                false
            } {
                f18(v3.clone(), v2.clone())?
            } else {
                false
            } {
                Ok(Adt10::C4({
                    let a823 = f12(v2.clone(), 1u64)?;
                    let a824 = f87(match v3.clone() {
                        Adt2::C0(_, h822, _, _, _, _, _, _, _, _, _) => {
                            h822
                        }
                    })?;
                    nat_mul(a823, a824)?
                }, 4u64, f16(v0, 1u64), f16(v2, 1u64), Adt4::C29(f16(v3, 1u64))))
            } else {
                Ok(Adt10::C1)
            }
        }
    }
}

pub fn f35(v0: bool, v1: Adt1, v2: Adt2, v3: Adt2) -> R<Adt10> {
    let v4: Adt6 = Adt6::C0(match v1.clone() {
        Adt1::C0(_, h825, _, _, _, _, _, _, _, _, _, _) => {
            h825
        }
    }, if v0 {
        match v1.clone() {
            Adt1::C0(_, _, _, h826, _, _, _, _, _, _, _, _) => {
                h826
            }
        }
    } else {
        match v1.clone() {
            Adt1::C0(_, _, h827, _, _, _, _, _, _, _, _, _) => {
                h827
            }
        }
    }, if v0 {
        match v1.clone() {
            Adt1::C0(_, _, _, _, _, h828, _, _, _, _, _, _) => {
                h828
            }
        }
    } else {
        match v1.clone() {
            Adt1::C0(_, _, _, h829, _, _, _, _, _, _, _, _) => {
                h829
            }
        }
    }, 1u64);
    let v5: Adt6 = if v0 {
        Adt6::C0(match v1.clone() {
            Adt1::C0(_, _, h830, _, _, _, _, _, _, _, _, _) => {
                h830
            }
        }, match v1.clone() {
            Adt1::C0(_, _, _, _, h831, _, _, _, _, _, _, _) => {
                h831
            }
        }, match v1.clone() {
            Adt1::C0(_, _, _, _, _, _, h832, _, _, _, _, _) => {
                h832
            }
        }, 1u64)
    } else {
        Adt6::C0(1u64, 1u64, 0u64, 1u64)
    };
    let a834 = match v1.clone() {
        Adt1::C0(h833, _, _, _, _, _, _, _, _, _, _, _) => {
            h833
        }
    };
    let a835 = 1u64;
    if if if if if if if if if if if nat_le(a834, a835) {
        let a837 = match v2.clone() {
            Adt2::C0(h836, _, _, _, _, _, _, _, _, _, _) => {
                h836
            }
        };
        let a838 = 1u64;
        nat_le(a837, a838)
    } else {
        false
    } {
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
        let a846 = match v2.clone() {
            Adt2::C0(_, _, _, _, _, h842, _, _, _, _, _) => {
                h842
            }
        };
        let a847 = {
            let a844 = match v2.clone() {
                Adt2::C0(h843, _, _, _, _, _, _, _, _, _, _) => {
                    h843
                }
            };
            let a845 = 0u64;
            if nat_eq(a844, a845) {
                4u64
            } else {
                2u64
            }
        };
        nat_eq(a846, a847)
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
        f88(v4.clone(), match v2.clone() {
            Adt2::C0(_, h848, _, _, _, _, _, _, _, _, _) => {
                h848
            }
        }, match v3.clone() {
            Adt2::C0(_, h849, _, _, _, _, _, _, _, _, _) => {
                h849
            }
        })?
    } else {
        false
    } {
        if v0 {
            f88(v5.clone(), match v2.clone() {
                Adt2::C0(_, _, h850, _, _, _, _, _, _, _, _) => {
                    h850
                }
            }, match v3.clone() {
                Adt2::C0(_, _, h851, _, _, _, _, _, _, _, _) => {
                    h851
                }
            })?
        } else {
            let a854 = match v2.clone() {
                Adt2::C0(_, _, h852, _, _, _, _, _, _, _, _) => {
                    h852
                }
            };
            let a855 = match v3.clone() {
                Adt2::C0(_, _, h853, _, _, _, _, _, _, _, _) => {
                    h853
                }
            };
            nat_eq(a854, a855)
        }
    } else {
        false
    } {
        let a858 = match v2.clone() {
            Adt2::C0(_, _, _, h856, _, _, _, _, _, _, _) => {
                h856
            }
        };
        let a859 = match v3.clone() {
            Adt2::C0(_, _, _, h857, _, _, _, _, _, _, _) => {
                h857
            }
        };
        nat_eq(a858, a859)
    } else {
        false
    } {
        let a862 = match v2.clone() {
            Adt2::C0(_, _, _, _, h860, _, _, _, _, _, _) => {
                h860
            }
        };
        let a863 = match v3.clone() {
            Adt2::C0(_, _, _, _, h861, _, _, _, _, _, _) => {
                h861
            }
        };
        nat_eq(a862, a863)
    } else {
        false
    } {
        let a868 = {
            let a866 = match v4.clone() {
                Adt6::C0(h864, _, _, _) => {
                    h864
                }
            };
            let a867 = match v5.clone() {
                Adt6::C0(h865, _, _, _) => {
                    h865
                }
            };
            nat_mul(a866, a867)?
        };
        let a869 = 2147483648u64;
        nat_lt(a868, a869)
    } else {
        false
    } {
        let v6: Adt7 = Adt7::C0(v4, v5, v0, {
            let a871 = match v1 {
                Adt1::C0(h870, _, _, _, _, _, _, _, _, _, _, _) => {
                    h870
                }
            };
            let a872 = 0u64;
            nat_eq(a871, a872)
        }, match v2.clone() {
            Adt2::C0(h873, _, _, _, _, _, _, _, _, _, _) => {
                h873
            }
        });
        Ok(Adt10::C4({
            let a874 = f12(v3.clone(), 1u64)?;
            let a875 = f89(v6.clone())?;
            nat_mul(a874, a875)?
        }, 4u64, f16(v2, 1u64), f16(v3, 1u64), Adt4::C28(v6)))
    } else {
        Ok(Adt10::C1)
    }
}

pub fn f36(v0: Adt1, v1: Adt2, v2: Option<Adt2>, v3: Adt2) -> R<Adt10> {
    let m876 = v2;
    match m876 {
        None => {
            Ok(Adt10::C1)
        }
        Some(v4) => {
            let v5: bool = {
                let a878 = match v0.clone() {
                    Adt1::C0(_, _, _, _, _, _, h877, _, _, _, _, _) => {
                        h877
                    }
                };
                let a879 = 1u64;
                nat_eq(a878, a879)
            };
            let v6: Adt6 = Adt6::C0(match v1.clone() {
                Adt2::C0(_, h880, _, _, _, _, _, _, _, _, _) => {
                    h880
                }
            }, match v0.clone() {
                Adt1::C0(h881, _, _, _, _, _, _, _, _, _, _, _) => {
                    h881
                }
            }, match v0.clone() {
                Adt1::C0(_, _, h882, _, _, _, _, _, _, _, _, _) => {
                    h882
                }
            }, match v0.clone() {
                Adt1::C0(_, _, _, _, h883, _, _, _, _, _, _, _) => {
                    h883
                }
            });
            let v7: Adt6 = if v5 {
                Adt6::C0(match v1.clone() {
                    Adt2::C0(_, _, h884, _, _, _, _, _, _, _, _) => {
                        h884
                    }
                }, match v0.clone() {
                    Adt1::C0(_, h885, _, _, _, _, _, _, _, _, _, _) => {
                        h885
                    }
                }, match v0.clone() {
                    Adt1::C0(_, _, _, h886, _, _, _, _, _, _, _, _) => {
                        h886
                    }
                }, match v0.clone() {
                    Adt1::C0(_, _, _, _, _, h887, _, _, _, _, _, _) => {
                        h887
                    }
                })
            } else {
                Adt6::C0(1u64, 1u64, 0u64, 1u64)
            };
            let v8: Adt6 = Adt6::C0(1u64, 1u64, 0u64, 1u64);
            let v9: u64 = if v5 {
                match v4.clone() {
                    Adt2::C0(_, _, _, h888, _, _, _, _, _, _, _) => {
                        h888
                    }
                }
            } else {
                match v4.clone() {
                    Adt2::C0(_, _, h889, _, _, _, _, _, _, _, _) => {
                        h889
                    }
                }
            };
            let v10: u64 = if v5 {
                match v4.clone() {
                    Adt2::C0(_, _, _, _, h890, _, _, _, _, _, _) => {
                        h890
                    }
                }
            } else {
                match v4.clone() {
                    Adt2::C0(_, _, _, h891, _, _, _, _, _, _, _) => {
                        h891
                    }
                }
            };
            let v11: u64 = if v5 {
                match v4.clone() {
                    Adt2::C0(_, _, h892, _, _, _, _, _, _, _, _) => {
                        h892
                    }
                }
            } else {
                1u64
            };
            let v12: u64 = match v4.clone() {
                Adt2::C0(h893, _, _, _, _, _, _, _, _, _, _) => {
                    h893
                }
            };
            let v13: u64 = match v3.clone() {
                Adt2::C0(h894, _, _, _, _, _, _, _, _, _, _) => {
                    h894
                }
            };
            let v14: u64 = {
                let a895 = v12;
                let a896 = 0u64;
                if nat_eq(a895, a896) {
                    4u64
                } else {
                    2u64
                }
            };
            let v15: u64 = {
                let a898 = match v4.clone() {
                    Adt2::C0(_, h897, _, _, _, _, _, _, _, _, _) => {
                        h897
                    }
                };
                let a899 = v14;
                nat_mul(a898, a899)?
            };
            let v16: u64 = if v5 {
                match v4.clone() {
                    Adt2::C0(_, _, _, _, _, _, _, h900, _, _, _) => {
                        h900
                    }
                }
            } else {
                match v4.clone() {
                    Adt2::C0(_, _, _, _, _, _, h901, _, _, _, _) => {
                        h901
                    }
                }
            };
            let v17: u64 = if v5 {
                match v4.clone() {
                    Adt2::C0(_, _, _, _, _, _, _, _, h902, _, _) => {
                        h902
                    }
                }
            } else {
                match v4.clone() {
                    Adt2::C0(_, _, _, _, _, _, _, h903, _, _, _) => {
                        h903
                    }
                }
            };
            let a905 = match v0 {
                Adt1::C0(_, _, _, _, _, _, h904, _, _, _, _, _) => {
                    h904
                }
            };
            let a906 = 1u64;
            if if if if if if if if if if nat_le(a905, a906) {
                let a907 = v12;
                let a908 = 0u64;
                if nat_eq(a907, a908) {
                    true
                } else {
                    let a909 = v12;
                    let a910 = 1u64;
                    if nat_eq(a909, a910) {
                        let a911 = v13;
                        let a912 = 1u64;
                        nat_eq(a911, a912)
                    } else {
                        false
                    }
                }
            } else {
                false
            } {
                let a913 = v13;
                let a914 = 0u64;
                if nat_eq(a913, a914) {
                    true
                } else {
                    let a915 = v13;
                    let a916 = 1u64;
                    nat_eq(a915, a916)
                }
            } else {
                false
            } {
                let a918 = match v4.clone() {
                    Adt2::C0(_, _, _, _, _, h917, _, _, _, _, _) => {
                        h917
                    }
                };
                let a919 = v14;
                nat_eq(a918, a919)
            } else {
                false
            } {
                let a927 = match v3.clone() {
                    Adt2::C0(_, h920, _, _, _, _, _, _, _, _, _) => {
                        h920
                    }
                };
                let a928 = {
                    let a925 = v9;
                    let a926 = {
                        let a923 = match v6.clone() {
                            Adt6::C0(h921, _, _, _) => {
                                h921
                            }
                        };
                        let a924 = match v7.clone() {
                            Adt6::C0(h922, _, _, _) => {
                                h922
                            }
                        };
                        nat_mul(a923, a924)?
                    };
                    nat_mul(a925, a926)?
                };
                nat_eq(a927, a928)
            } else {
                false
            } {
                let a931 = match v3.clone() {
                    Adt2::C0(_, _, h929, _, _, _, _, _, _, _, _) => {
                        h929
                    }
                };
                let a932 = f78(v6.clone(), match v4.clone() {
                    Adt2::C0(_, h930, _, _, _, _, _, _, _, _, _) => {
                        h930
                    }
                })?;
                nat_eq(a931, a932)
            } else {
                false
            } {
                let a934 = 0u64;
                let a935 = match v3.clone() {
                    Adt2::C0(_, _, h933, _, _, _, _, _, _, _, _) => {
                        h933
                    }
                };
                nat_lt(a934, a935)
            } else {
                false
            } {
                if v5 {
                    let a937 = match v1 {
                        Adt2::C0(_, _, _, h936, _, _, _, _, _, _, _) => {
                            h936
                        }
                    };
                    let a938 = v9;
                    if if if nat_eq(a937, a938) {
                        let a940 = match v3.clone() {
                            Adt2::C0(_, _, _, h939, _, _, _, _, _, _, _) => {
                                h939
                            }
                        };
                        let a941 = f78(v7.clone(), v11)?;
                        nat_eq(a940, a941)
                    } else {
                        false
                    } {
                        let a943 = match v3.clone() {
                            Adt2::C0(_, _, _, _, h942, _, _, _, _, _, _) => {
                                h942
                            }
                        };
                        let a944 = v10;
                        nat_eq(a943, a944)
                    } else {
                        false
                    } {
                        let a946 = 0u64;
                        let a947 = match v3.clone() {
                            Adt2::C0(_, _, _, h945, _, _, _, _, _, _, _) => {
                                h945
                            }
                        };
                        nat_lt(a946, a947)
                    } else {
                        false
                    }
                } else {
                    let a949 = match v1 {
                        Adt2::C0(_, _, h948, _, _, _, _, _, _, _, _) => {
                            h948
                        }
                    };
                    let a950 = v9;
                    if if if nat_eq(a949, a950) {
                        let a952 = match v4.clone() {
                            Adt2::C0(_, _, _, _, h951, _, _, _, _, _, _) => {
                                h951
                            }
                        };
                        let a953 = 1u64;
                        nat_eq(a952, a953)
                    } else {
                        false
                    } {
                        let a955 = match v3.clone() {
                            Adt2::C0(_, _, _, h954, _, _, _, _, _, _, _) => {
                                h954
                            }
                        };
                        let a956 = v10;
                        nat_eq(a955, a956)
                    } else {
                        false
                    } {
                        let a958 = match v3.clone() {
                            Adt2::C0(_, _, _, _, h957, _, _, _, _, _, _) => {
                                h957
                            }
                        };
                        let a959 = 1u64;
                        nat_eq(a958, a959)
                    } else {
                        false
                    }
                }
            } else {
                false
            } {
                if f27(match v4.clone() {
                    Adt2::C0(_, h960, _, _, _, _, _, _, _, _, _) => {
                        h960
                    }
                }, v11, v9, v10) {
                    true
                } else {
                    let a980 = {
                        let a977 = {
                            let a975 = {
                                let a969 = {
                                    let a963 = {
                                        let a961 = v10;
                                        let a962 = 1u64;
                                        nat_sub(a961, a962)
                                    };
                                    let a964 = v17;
                                    nat_mul(a963, a964)?
                                };
                                let a970 = {
                                    let a967 = {
                                        let a965 = v9;
                                        let a966 = 1u64;
                                        nat_sub(a965, a966)
                                    };
                                    let a968 = v16;
                                    nat_mul(a967, a968)?
                                };
                                nat_add(a969, a970)?
                            };
                            let a976 = {
                                let a973 = {
                                    let a971 = v11;
                                    let a972 = 1u64;
                                    nat_sub(a971, a972)
                                };
                                let a974 = v15;
                                nat_mul(a973, a974)?
                            };
                            nat_add(a975, a976)?
                        };
                        let a978 = v15;
                        nat_add(a977, a978)?
                    };
                    let a981 = match v4.clone() {
                        Adt2::C0(_, _, _, _, _, _, _, _, _, h979, _) => {
                            h979
                        }
                    };
                    nat_le(a980, a981)
                }
            } else {
                false
            } {
                f18(v4.clone(), v3.clone())?
            } else {
                false
            } {
                Ok(Adt10::C4(f12(v3.clone(), 1u64)?, {
                    let a982 = v13;
                    let a983 = 0u64;
                    if nat_eq(a982, a983) {
                        4u64
                    } else {
                        2u64
                    }
                }, Adt3::C0(match v4 {
                    Adt2::C0(_, h984, _, _, _, _, _, _, _, _, _) => {
                        h984
                    }
                }, v11, 1u64, v10, v14, v15, 0u64, 0u64), if v5 {
                    f16(v3, 1u64)
                } else {
                    Adt3::C0(match v3.clone() {
                        Adt2::C0(_, h985, _, _, _, _, _, _, _, _, _) => {
                            h985
                        }
                    }, match v3.clone() {
                        Adt2::C0(_, _, h986, _, _, _, _, _, _, _, _) => {
                            h986
                        }
                    }, 1u64, match v3.clone() {
                        Adt2::C0(_, _, _, h987, _, _, _, _, _, _, _) => {
                            h987
                        }
                    }, match v3.clone() {
                        Adt2::C0(_, _, _, _, _, h988, _, _, _, _, _) => {
                            h988
                        }
                    }, match v3.clone() {
                        Adt2::C0(_, _, _, _, _, _, h989, _, _, _, _) => {
                            h989
                        }
                    }, 0u64, match v3 {
                        Adt2::C0(_, _, _, _, _, _, _, h990, _, _, _) => {
                            h990
                        }
                    })
                }, Adt4::C27(Adt5::C0(v6, v7, v8, v16, v17, v12, 1u64, v13))))
            } else {
                Ok(Adt10::C1)
            }
        }
    }
}

pub fn f37(v0: Adt1, v1: Adt2, v2: Option<Adt2>, v3: Adt2) -> R<Adt10> {
    let m991 = v2;
    match m991 {
        None => {
            Ok(Adt10::C1)
        }
        Some(v4) => {
            let v5: Adt6 = Adt6::C0(match v1.clone() {
                Adt2::C0(_, h992, _, _, _, _, _, _, _, _, _) => {
                    h992
                }
            }, match v0.clone() {
                Adt1::C0(h993, _, _, _, _, _, _, _, _, _, _, _) => {
                    h993
                }
            }, match v0.clone() {
                Adt1::C0(_, _, _, h994, _, _, _, _, _, _, _, _) => {
                    h994
                }
            }, match v0.clone() {
                Adt1::C0(_, _, _, _, _, _, h995, _, _, _, _, _) => {
                    h995
                }
            });
            let v6: Adt6 = Adt6::C0(match v1.clone() {
                Adt2::C0(_, _, h996, _, _, _, _, _, _, _, _) => {
                    h996
                }
            }, match v0.clone() {
                Adt1::C0(_, h997, _, _, _, _, _, _, _, _, _, _) => {
                    h997
                }
            }, match v0.clone() {
                Adt1::C0(_, _, _, _, h998, _, _, _, _, _, _, _) => {
                    h998
                }
            }, match v0.clone() {
                Adt1::C0(_, _, _, _, _, _, _, h999, _, _, _, _) => {
                    h999
                }
            });
            let v7: Adt6 = Adt6::C0(match v1 {
                Adt2::C0(_, _, _, h1000, _, _, _, _, _, _, _) => {
                    h1000
                }
            }, match v0.clone() {
                Adt1::C0(_, _, h1001, _, _, _, _, _, _, _, _, _) => {
                    h1001
                }
            }, match v0.clone() {
                Adt1::C0(_, _, _, _, _, h1002, _, _, _, _, _, _) => {
                    h1002
                }
            }, match v0.clone() {
                Adt1::C0(_, _, _, _, _, _, _, _, h1003, _, _, _) => {
                    h1003
                }
            });
            let v8: u64 = match v0 {
                Adt1::C0(_, _, _, _, _, _, _, _, _, h1004, _, _) => {
                    h1004
                }
            };
            let v9: u64 = f78(v7.clone(), match v4.clone() {
                Adt2::C0(_, _, _, h1005, _, _, _, _, _, _, _) => {
                    h1005
                }
            })?;
            let v10: u64 = match v3.clone() {
                Adt2::C0(h1006, _, _, _, _, _, _, _, _, _, _) => {
                    h1006
                }
            };
            let v11: u64 = {
                let a1014 = v8;
                let a1015 = {
                    let a1012 = match v5.clone() {
                        Adt6::C0(h1007, _, _, _) => {
                            h1007
                        }
                    };
                    let a1013 = {
                        let a1010 = match v6.clone() {
                            Adt6::C0(h1008, _, _, _) => {
                                h1008
                            }
                        };
                        let a1011 = match v7.clone() {
                            Adt6::C0(h1009, _, _, _) => {
                                h1009
                            }
                        };
                        nat_mul(a1010, a1011)?
                    };
                    nat_mul(a1012, a1013)?
                };
                nat_mul(a1014, a1015)?
            };
            let a1017 = match v4.clone() {
                Adt2::C0(h1016, _, _, _, _, _, _, _, _, _, _) => {
                    h1016
                }
            };
            let a1018 = 0u64;
            if if if if if if if if if if if if if nat_eq(a1017, a1018) {
                let a1019 = v10;
                let a1020 = 0u64;
                if nat_eq(a1019, a1020) {
                    true
                } else {
                    let a1021 = v10;
                    let a1022 = 1u64;
                    nat_eq(a1021, a1022)
                }
            } else {
                false
            } {
                let a1023 = 0u64;
                let a1024 = v8;
                nat_lt(a1023, a1024)
            } else {
                false
            } {
                let a1025 = v8;
                let a1026 = 2147483648u64;
                nat_lt(a1025, a1026)
            } else {
                false
            } {
                let a1028 = match v4.clone() {
                    Adt2::C0(_, _, _, _, _, h1027, _, _, _, _, _) => {
                        h1027
                    }
                };
                let a1029 = 4u64;
                nat_eq(a1028, a1029)
            } else {
                false
            } {
                let a1030 = 0u64;
                let a1031 = v9;
                nat_lt(a1030, a1031)
            } else {
                false
            } {
                let a1033 = 0u64;
                let a1034 = match v3.clone() {
                    Adt2::C0(_, _, h1032, _, _, _, _, _, _, _, _) => {
                        h1032
                    }
                };
                nat_lt(a1033, a1034)
            } else {
                false
            } {
                let a1036 = 0u64;
                let a1037 = match v3.clone() {
                    Adt2::C0(_, _, _, h1035, _, _, _, _, _, _, _) => {
                        h1035
                    }
                };
                nat_lt(a1036, a1037)
            } else {
                false
            } {
                let a1039 = match v3.clone() {
                    Adt2::C0(_, h1038, _, _, _, _, _, _, _, _, _) => {
                        h1038
                    }
                };
                let a1040 = v11;
                nat_eq(a1039, a1040)
            } else {
                false
            } {
                let a1043 = match v3.clone() {
                    Adt2::C0(_, _, h1041, _, _, _, _, _, _, _, _) => {
                        h1041
                    }
                };
                let a1044 = f78(v5.clone(), match v4.clone() {
                    Adt2::C0(_, h1042, _, _, _, _, _, _, _, _, _) => {
                        h1042
                    }
                })?;
                nat_eq(a1043, a1044)
            } else {
                false
            } {
                let a1047 = match v3.clone() {
                    Adt2::C0(_, _, _, h1045, _, _, _, _, _, _, _) => {
                        h1045
                    }
                };
                let a1048 = f78(v6.clone(), match v4.clone() {
                    Adt2::C0(_, _, h1046, _, _, _, _, _, _, _, _) => {
                        h1046
                    }
                })?;
                nat_eq(a1047, a1048)
            } else {
                false
            } {
                let a1056 = match v3.clone() {
                    Adt2::C0(_, _, _, _, h1049, _, _, _, _, _, _) => {
                        h1049
                    }
                };
                let a1057 = {
                    let a1054 = v9;
                    let a1055 = {
                        let a1051 = match v4.clone() {
                            Adt2::C0(_, _, _, _, h1050, _, _, _, _, _, _) => {
                                h1050
                            }
                        };
                        let a1052 = v8;
                        let a1053 = 0u64;
                        nat_quot(a1051, a1052, a1053)
                    };
                    nat_mul(a1054, a1055)?
                };
                nat_eq(a1056, a1057)
            } else {
                false
            } {
                f18(v4.clone(), v3.clone())?
            } else {
                false
            } {
                Ok(Adt10::C4(f12(v3.clone(), 1u64)?, {
                    let a1058 = v10;
                    let a1059 = 0u64;
                    if nat_eq(a1058, a1059) {
                        4u64
                    } else {
                        2u64
                    }
                }, f16(v4.clone(), 1u64), f16(v3, 1u64), Adt4::C27(Adt5::C0(v5, v6, v7, match v4.clone() {
                    Adt2::C0(_, _, _, _, _, _, _, _, h1060, _, _) => {
                        h1060
                    }
                }, {
                    let a1062 = v8;
                    let a1063 = match v4 {
                        Adt2::C0(_, _, _, _, _, _, _, _, h1061, _, _) => {
                            h1061
                        }
                    };
                    nat_mul(a1062, a1063)?
                }, 0u64, v9, v10))))
            } else {
                Ok(Adt10::C1)
            }
        }
    }
}

pub fn f38(v0: Adt2, v1: Option<Adt2>, v2: Adt2) -> R<Adt10> {
    let m1064 = v1;
    match m1064 {
        None => {
            Ok(Adt10::C1)
        }
        Some(v3) => {
            f90(v0, v3, v2)
        }
    }
}

pub fn f39(v0: Adt2, v1: Adt2) -> R<Adt10> {
    let a1066 = match v0.clone() {
        Adt2::C0(h1065, _, _, _, _, _, _, _, _, _, _) => {
            h1065
        }
    };
    let a1067 = 1u64;
    if if nat_eq(a1066, a1067) {
        true
    } else {
        let a1069 = match v0.clone() {
            Adt2::C0(h1068, _, _, _, _, _, _, _, _, _, _) => {
                h1068
            }
        };
        let a1070 = 30u64;
        nat_eq(a1069, a1070)
    } {
        let a1072 = match v1.clone() {
            Adt2::C0(h1071, _, _, _, _, _, _, _, _, _, _) => {
                h1071
            }
        };
        let a1073 = 1u64;
        if if if if if if if if if if if nat_eq(a1072, a1073) {
            let a1076 = match v0.clone() {
                Adt2::C0(_, h1074, _, _, _, _, _, _, _, _, _) => {
                    h1074
                }
            };
            let a1077 = match v1.clone() {
                Adt2::C0(_, h1075, _, _, _, _, _, _, _, _, _) => {
                    h1075
                }
            };
            nat_eq(a1076, a1077)
        } else {
            false
        } {
            let a1079 = 0u64;
            let a1080 = match v1.clone() {
                Adt2::C0(_, _, h1078, _, _, _, _, _, _, _, _) => {
                    h1078
                }
            };
            nat_lt(a1079, a1080)
        } else {
            false
        } {
            let a1083 = match v1.clone() {
                Adt2::C0(_, _, h1081, _, _, _, _, _, _, _, _) => {
                    h1081
                }
            };
            let a1084 = match v1.clone() {
                Adt2::C0(_, _, _, h1082, _, _, _, _, _, _, _) => {
                    h1082
                }
            };
            nat_eq(a1083, a1084)
        } else {
            false
        } {
            let a1086 = match v1.clone() {
                Adt2::C0(_, _, _, _, h1085, _, _, _, _, _, _) => {
                    h1085
                }
            };
            let a1087 = 1u64;
            nat_eq(a1086, a1087)
        } else {
            false
        } {
            let a1095 = match v0.clone() {
                Adt2::C0(_, _, h1088, _, _, _, _, _, _, _, _) => {
                    h1088
                }
            };
            let a1096 = {
                let a1093 = {
                    let a1091 = match v1.clone() {
                        Adt2::C0(_, _, h1089, _, _, _, _, _, _, _, _) => {
                            h1089
                        }
                    };
                    let a1092 = match v1.clone() {
                        Adt2::C0(_, _, h1090, _, _, _, _, _, _, _, _) => {
                            h1090
                        }
                    };
                    nat_add(a1091, a1092)?
                };
                let a1094 = 1u64;
                nat_sub(a1093, a1094)
            };
            nat_eq(a1095, a1096)
        } else {
            false
        } {
            let a1098 = 0u64;
            let a1099 = match v0.clone() {
                Adt2::C0(_, _, _, h1097, _, _, _, _, _, _, _) => {
                    h1097
                }
            };
            nat_lt(a1098, a1099)
        } else {
            false
        } {
            let a1101 = 0u64;
            let a1102 = match v0.clone() {
                Adt2::C0(_, _, _, _, h1100, _, _, _, _, _, _) => {
                    h1100
                }
            };
            nat_lt(a1101, a1102)
        } else {
            false
        } {
            let a1104 = 2u64;
            let a1105 = match v0.clone() {
                Adt2::C0(_, _, _, _, _, h1103, _, _, _, _, _) => {
                    h1103
                }
            };
            nat_le(a1104, a1105)
        } else {
            false
        } {
            let a1107 = 2u64;
            let a1108 = match v1.clone() {
                Adt2::C0(_, _, _, _, _, h1106, _, _, _, _, _) => {
                    h1106
                }
            };
            nat_le(a1107, a1108)
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

pub fn f40(v0: Adt2, v1: Option<Adt2>, v2: Adt2) -> R<Adt10> {
    let m1109 = v1;
    match m1109 {
        None => {
            Ok(Adt10::C1)
        }
        Some(v3) => {
            f91(v0, v3, v2)
        }
    }
}

pub fn f41(v0: u64) -> bool {
    let a1110 = v0;
    let a1111 = 1u64;
    if if nat_eq(a1110, a1111) {
        true
    } else {
        let a1112 = v0;
        let a1113 = 33u64;
        nat_eq(a1112, a1113)
    } {
        true
    } else {
        let a1114 = v0;
        let a1115 = 34u64;
        nat_eq(a1114, a1115)
    }
}

pub fn f42(v0: u64, v1: Adt2, v2: bool, v3: Adt2) -> R<Adt10> {
    let a1118 = match v1.clone() {
        Adt2::C0(h1116, _, _, _, _, _, _, _, _, _, _) => {
            h1116
        }
    };
    let a1119 = match v3.clone() {
        Adt2::C0(h1117, _, _, _, _, _, _, _, _, _, _) => {
            h1117
        }
    };
    if nat_eq(a1118, a1119) {
        f92(v0, v1, v2, v3)
    } else {
        if if if if f93(match v1.clone() {
            Adt2::C0(h1120, _, _, _, _, _, _, _, _, _, _) => {
                h1120
            }
        }) {
            f93(match v3.clone() {
                Adt2::C0(h1121, _, _, _, _, _, _, _, _, _, _) => {
                    h1121
                }
            })
        } else {
            false
        } {
            let a1122 = v0;
            let a1123 = 33u64;
            if nat_eq(a1122, a1123) {
                v2
            } else {
                true
            }
        } else {
            false
        } {
            let a1124 = f12(v1.clone(), 1u64)?;
            let a1125 = f12(v3.clone(), 1u64)?;
            nat_eq(a1124, a1125)
        } else {
            false
        } {
            let m1127 = f94(match v3.clone() {
                Adt2::C0(h1126, _, _, _, _, _, _, _, _, _, _) => {
                    h1126
                }
            });
            match m1127 {
                Adt14::C0 => {
                    Ok(Adt10::C0)
                }
                Adt14::C1(v4) => {
                    Ok(Adt10::C4(f12(v3.clone(), 1u64)?, match v4 {
                        Adt15::C0(_, _, h1128) => {
                            h1128
                        }
                    }, f16(v1.clone(), 1u64), f16(v3.clone(), 1u64), Adt4::C11(match v1 {
                        Adt2::C0(h1129, _, _, _, _, _, _, _, _, _, _) => {
                            h1129
                        }
                    }, match v3 {
                        Adt2::C0(h1130, _, _, _, _, _, _, _, _, _, _) => {
                            h1130
                        }
                    })))
                }
            }
        } else {
            Ok(Adt10::C1)
        }
    }
}

pub fn f43(v0: Adt2, v1: Adt2) -> R<Adt10> {
    let a1132 = match v0.clone() {
        Adt2::C0(h1131, _, _, _, _, _, _, _, _, _, _) => {
            h1131
        }
    };
    let a1133 = 0u64;
    if if if if nat_eq(a1132, a1133) {
        true
    } else {
        let a1135 = match v0.clone() {
            Adt2::C0(h1134, _, _, _, _, _, _, _, _, _, _) => {
                h1134
            }
        };
        let a1136 = 1u64;
        nat_eq(a1135, a1136)
    } {
        true
    } else {
        let a1138 = match v0.clone() {
            Adt2::C0(h1137, _, _, _, _, _, _, _, _, _, _) => {
                h1137
            }
        };
        let a1139 = 25u64;
        if nat_eq(a1138, a1139) {
            true
        } else {
            let a1141 = match v0.clone() {
                Adt2::C0(h1140, _, _, _, _, _, _, _, _, _, _) => {
                    h1140
                }
            };
            let a1142 = 26u64;
            nat_eq(a1141, a1142)
        }
    } {
        true
    } else {
        let a1144 = match v0.clone() {
            Adt2::C0(h1143, _, _, _, _, _, _, _, _, _, _) => {
                h1143
            }
        };
        let a1145 = 30u64;
        nat_eq(a1144, a1145)
    } {
        let a1148 = match v0.clone() {
            Adt2::C0(h1146, _, _, _, _, _, _, _, _, _, _) => {
                h1146
            }
        };
        let a1149 = match v1.clone() {
            Adt2::C0(h1147, _, _, _, _, _, _, _, _, _, _) => {
                h1147
            }
        };
        if if nat_eq(a1148, a1149) {
            f95(match v1.clone() {
                Adt2::C0(_, h1150, _, _, _, _, _, _, _, _, _) => {
                    h1150
                }
            }, match v1.clone() {
                Adt2::C0(_, _, h1151, _, _, _, _, _, _, _, _) => {
                    h1151
                }
            }, match v1.clone() {
                Adt2::C0(_, _, _, h1152, _, _, _, _, _, _, _) => {
                    h1152
                }
            }, match v1.clone() {
                Adt2::C0(_, _, _, _, h1153, _, _, _, _, _, _) => {
                    h1153
                }
            }, match v0.clone() {
                Adt2::C0(_, h1154, _, _, _, _, _, _, _, _, _) => {
                    h1154
                }
            }, match v0.clone() {
                Adt2::C0(_, _, h1155, _, _, _, _, _, _, _, _) => {
                    h1155
                }
            }, match v0.clone() {
                Adt2::C0(_, _, _, h1156, _, _, _, _, _, _, _) => {
                    h1156
                }
            }, match v0.clone() {
                Adt2::C0(_, _, _, _, h1157, _, _, _, _, _, _) => {
                    h1157
                }
            })
        } else {
            false
        } {
            let m1159 = f94(match v0.clone() {
                Adt2::C0(h1158, _, _, _, _, _, _, _, _, _, _) => {
                    h1158
                }
            });
            match m1159 {
                Adt14::C0 => {
                    Ok(Adt10::C0)
                }
                Adt14::C1(v2) => {
                    Ok(Adt10::C4(f12(v1.clone(), 1u64)?, match v2 {
                        Adt15::C0(_, _, h1160) => {
                            h1160
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

pub fn f44(v0: u64, v1: Adt2, v2: Option<Adt2>, v3: Adt2) -> R<Adt10> {
    let m1161 = v2;
    match m1161 {
        None => {
            Ok(Adt10::C1)
        }
        Some(v4) => {
            let a1162 = v0;
            let a1163 = 4u64;
            if if if nat_lt(a1162, a1163) {
                let a1166 = match v1.clone() {
                    Adt2::C0(h1164, _, _, _, _, _, _, _, _, _, _) => {
                        h1164
                    }
                };
                let a1167 = match v4.clone() {
                    Adt2::C0(h1165, _, _, _, _, _, _, _, _, _, _) => {
                        h1165
                    }
                };
                if nat_eq(a1166, a1167) {
                    let a1170 = match v1.clone() {
                        Adt2::C0(h1168, _, _, _, _, _, _, _, _, _, _) => {
                            h1168
                        }
                    };
                    let a1171 = match v3.clone() {
                        Adt2::C0(h1169, _, _, _, _, _, _, _, _, _, _) => {
                            h1169
                        }
                    };
                    nat_eq(a1170, a1171)
                } else {
                    false
                }
            } else {
                false
            } {
                if if if f96(v0, 0u64, match v1.clone() {
                    Adt2::C0(_, h1172, _, _, _, _, _, _, _, _, _) => {
                        h1172
                    }
                }, match v4.clone() {
                    Adt2::C0(_, h1173, _, _, _, _, _, _, _, _, _) => {
                        h1173
                    }
                }, match v3.clone() {
                    Adt2::C0(_, h1174, _, _, _, _, _, _, _, _, _) => {
                        h1174
                    }
                })? {
                    f96(v0, 1u64, match v1.clone() {
                        Adt2::C0(_, _, h1175, _, _, _, _, _, _, _, _) => {
                            h1175
                        }
                    }, match v4.clone() {
                        Adt2::C0(_, _, h1176, _, _, _, _, _, _, _, _) => {
                            h1176
                        }
                    }, match v3.clone() {
                        Adt2::C0(_, _, h1177, _, _, _, _, _, _, _, _) => {
                            h1177
                        }
                    })?
                } else {
                    false
                } {
                    f96(v0, 2u64, match v1.clone() {
                        Adt2::C0(_, _, _, h1178, _, _, _, _, _, _, _) => {
                            h1178
                        }
                    }, match v4.clone() {
                        Adt2::C0(_, _, _, h1179, _, _, _, _, _, _, _) => {
                            h1179
                        }
                    }, match v3.clone() {
                        Adt2::C0(_, _, _, h1180, _, _, _, _, _, _, _) => {
                            h1180
                        }
                    })?
                } else {
                    false
                } {
                    f96(v0, 3u64, match v1.clone() {
                        Adt2::C0(_, _, _, _, h1181, _, _, _, _, _, _) => {
                            h1181
                        }
                    }, match v4.clone() {
                        Adt2::C0(_, _, _, _, h1182, _, _, _, _, _, _) => {
                            h1182
                        }
                    }, match v3.clone() {
                        Adt2::C0(_, _, _, _, h1183, _, _, _, _, _, _) => {
                            h1183
                        }
                    })?
                } else {
                    false
                }
            } else {
                false
            } {
                let m1185 = f94(match v1.clone() {
                    Adt2::C0(h1184, _, _, _, _, _, _, _, _, _, _) => {
                        h1184
                    }
                });
                match m1185 {
                    Adt14::C0 => {
                        Ok(Adt10::C0)
                    }
                    Adt14::C1(v5) => {
                        let v6: u64 = match v5.clone() {
                            Adt15::C0(_, h1186, _) => {
                                h1186
                            }
                        };
                        let a1191 = {
                            let a1188 = match v1.clone() {
                                Adt2::C0(_, h1187, _, _, _, _, _, _, _, _, _) => {
                                    h1187
                                }
                            };
                            let a1189 = v6;
                            let a1190 = 1u64;
                            nat_rem(a1188, a1189, a1190)
                        };
                        let a1192 = 0u64;
                        if if if nat_eq(a1191, a1192) {
                            let a1197 = {
                                let a1194 = match v4.clone() {
                                    Adt2::C0(_, h1193, _, _, _, _, _, _, _, _, _) => {
                                        h1193
                                    }
                                };
                                let a1195 = v6;
                                let a1196 = 1u64;
                                nat_rem(a1194, a1195, a1196)
                            };
                            let a1198 = 0u64;
                            nat_eq(a1197, a1198)
                        } else {
                            false
                        } {
                            let a1203 = {
                                let a1200 = match v3.clone() {
                                    Adt2::C0(_, h1199, _, _, _, _, _, _, _, _, _) => {
                                        h1199
                                    }
                                };
                                let a1201 = v6;
                                let a1202 = 1u64;
                                nat_rem(a1200, a1201, a1202)
                            };
                            let a1204 = 0u64;
                            nat_eq(a1203, a1204)
                        } else {
                            false
                        } {
                            Ok(Adt10::C4(f12(v3.clone(), v6)?, match v5 {
                                Adt15::C0(_, _, h1205) => {
                                    h1205
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

pub fn f45(v0: u64, v1: Adt1, v2: Adt2, v3: Adt2) -> R<Adt10> {
    let a1207 = match v2.clone() {
        Adt2::C0(h1206, _, _, _, _, _, _, _, _, _, _) => {
            h1206
        }
    };
    let a1208 = 0u64;
    if if nat_eq(a1207, a1208) {
        let a1210 = match v3.clone() {
            Adt2::C0(h1209, _, _, _, _, _, _, _, _, _, _) => {
                h1209
            }
        };
        let a1211 = 0u64;
        nat_eq(a1210, a1211)
    } else {
        false
    } {
        let a1212 = v0;
        let a1213 = 61u64;
        if nat_eq(a1212, a1213) {
            let v4: bool = {
                let a1215 = 0u64;
                let a1216 = match v1.clone() {
                    Adt1::C0(_, _, _, _, _, _, _, _, h1214, _, _, _) => {
                        h1214
                    }
                };
                nat_lt(a1215, a1216)
            };
            if if if if if if f97(match v2.clone() {
                Adt2::C0(_, h1217, _, _, _, _, _, _, _, _, _) => {
                    h1217
                }
            }, match v1.clone() {
                Adt1::C0(h1218, _, _, _, _, _, _, _, _, _, _, _) => {
                    h1218
                }
            }, match v1.clone() {
                Adt1::C0(_, h1219, _, _, _, _, _, _, _, _, _, _) => {
                    h1219
                }
            }, match v3.clone() {
                Adt2::C0(_, h1220, _, _, _, _, _, _, _, _, _) => {
                    h1220
                }
            })? {
                f97(match v2.clone() {
                    Adt2::C0(_, _, h1221, _, _, _, _, _, _, _, _) => {
                        h1221
                    }
                }, match v1.clone() {
                    Adt1::C0(_, _, h1222, _, _, _, _, _, _, _, _, _) => {
                        h1222
                    }
                }, match v1.clone() {
                    Adt1::C0(_, _, _, h1223, _, _, _, _, _, _, _, _) => {
                        h1223
                    }
                }, match v3.clone() {
                    Adt2::C0(_, _, h1224, _, _, _, _, _, _, _, _) => {
                        h1224
                    }
                })?
            } else {
                false
            } {
                f97(match v2.clone() {
                    Adt2::C0(_, _, _, h1225, _, _, _, _, _, _, _) => {
                        h1225
                    }
                }, match v1.clone() {
                    Adt1::C0(_, _, _, _, h1226, _, _, _, _, _, _, _) => {
                        h1226
                    }
                }, match v1.clone() {
                    Adt1::C0(_, _, _, _, _, h1227, _, _, _, _, _, _) => {
                        h1227
                    }
                }, match v3.clone() {
                    Adt2::C0(_, _, _, h1228, _, _, _, _, _, _, _) => {
                        h1228
                    }
                })?
            } else {
                false
            } {
                f97(match v2.clone() {
                    Adt2::C0(_, _, _, _, h1229, _, _, _, _, _, _) => {
                        h1229
                    }
                }, match v1.clone() {
                    Adt1::C0(_, _, _, _, _, _, h1230, _, _, _, _, _) => {
                        h1230
                    }
                }, match v1.clone() {
                    Adt1::C0(_, _, _, _, _, _, _, h1231, _, _, _, _) => {
                        h1231
                    }
                }, match v3.clone() {
                    Adt2::C0(_, _, _, _, h1232, _, _, _, _, _, _) => {
                        h1232
                    }
                })?
            } else {
                false
            } {
                let a1234 = match v1.clone() {
                    Adt1::C0(_, _, _, _, _, _, _, _, h1233, _, _, _) => {
                        h1233
                    }
                };
                let a1235 = 4294967295u64;
                nat_le(a1234, a1235)
            } else {
                false
            } {
                if v4 {
                    !f27(match v2.clone() {
                        Adt2::C0(_, h1236, _, _, _, _, _, _, _, _, _) => {
                            h1236
                        }
                    }, match v2.clone() {
                        Adt2::C0(_, _, h1237, _, _, _, _, _, _, _, _) => {
                            h1237
                        }
                    }, match v2.clone() {
                        Adt2::C0(_, _, _, h1238, _, _, _, _, _, _, _) => {
                            h1238
                        }
                    }, match v2.clone() {
                        Adt2::C0(_, _, _, _, h1239, _, _, _, _, _, _) => {
                            h1239
                        }
                    })
                } else {
                    true
                }
            } else {
                false
            } {
                Ok(Adt10::C4(f12(v3.clone(), 1u64)?, 4u64, f16(v2, 1u64), f16(v3, 1u64), Adt4::C3(match v1.clone() {
                    Adt1::C0(h1240, _, _, _, _, _, _, _, _, _, _, _) => {
                        h1240
                    }
                }, match v1.clone() {
                    Adt1::C0(_, _, h1241, _, _, _, _, _, _, _, _, _) => {
                        h1241
                    }
                }, match v1.clone() {
                    Adt1::C0(_, _, _, _, h1242, _, _, _, _, _, _, _) => {
                        h1242
                    }
                }, match v1 {
                    Adt1::C0(_, _, _, _, _, _, h1243, _, _, _, _, _) => {
                        h1243
                    }
                }, v4)))
            } else {
                Ok(Adt10::C1)
            }
        } else {
            let a1244 = v0;
            let a1245 = 62u64;
            if nat_eq(a1244, a1245) {
                let a1247 = match v1.clone() {
                    Adt1::C0(h1246, _, _, _, _, _, _, _, _, _, _, _) => {
                        h1246
                    }
                };
                let a1248 = 2147483648u64;
                if if if if if nat_lt(a1247, a1248) {
                    let a1250 = match v1.clone() {
                        Adt1::C0(_, h1249, _, _, _, _, _, _, _, _, _, _) => {
                            h1249
                        }
                    };
                    let a1251 = 2147483648u64;
                    nat_lt(a1250, a1251)
                } else {
                    false
                } {
                    let a1254 = match v1.clone() {
                        Adt1::C0(h1252, _, _, _, _, _, _, _, _, _, _, _) => {
                            h1252
                        }
                    };
                    let a1255 = match v2.clone() {
                        Adt2::C0(_, h1253, _, _, _, _, _, _, _, _, _) => {
                            h1253
                        }
                    };
                    nat_lt(a1254, a1255)
                } else {
                    false
                } {
                    let a1258 = match v1.clone() {
                        Adt1::C0(_, h1256, _, _, _, _, _, _, _, _, _, _) => {
                            h1256
                        }
                    };
                    let a1259 = match v2.clone() {
                        Adt2::C0(_, h1257, _, _, _, _, _, _, _, _, _) => {
                            h1257
                        }
                    };
                    nat_lt(a1258, a1259)
                } else {
                    false
                } {
                    let a1262 = match v2.clone() {
                        Adt2::C0(_, _, h1260, _, _, _, _, _, _, _, _) => {
                            h1260
                        }
                    };
                    let a1263 = match v3.clone() {
                        Adt2::C0(_, _, h1261, _, _, _, _, _, _, _, _) => {
                            h1261
                        }
                    };
                    if if if nat_eq(a1262, a1263) {
                        let a1266 = match v2.clone() {
                            Adt2::C0(_, _, _, h1264, _, _, _, _, _, _, _) => {
                                h1264
                            }
                        };
                        let a1267 = match v3.clone() {
                            Adt2::C0(_, _, _, h1265, _, _, _, _, _, _, _) => {
                                h1265
                            }
                        };
                        nat_eq(a1266, a1267)
                    } else {
                        false
                    } {
                        let a1270 = match v2.clone() {
                            Adt2::C0(_, _, _, _, h1268, _, _, _, _, _, _) => {
                                h1268
                            }
                        };
                        let a1271 = match v3.clone() {
                            Adt2::C0(_, _, _, _, h1269, _, _, _, _, _, _) => {
                                h1269
                            }
                        };
                        nat_eq(a1270, a1271)
                    } else {
                        false
                    } {
                        let a1280 = match v3.clone() {
                            Adt2::C0(_, h1272, _, _, _, _, _, _, _, _, _) => {
                                h1272
                            }
                        };
                        let a1281 = {
                            let a1278 = {
                                let a1275 = match v2.clone() {
                                    Adt2::C0(_, h1273, _, _, _, _, _, _, _, _, _) => {
                                        h1273
                                    }
                                };
                                let a1276 = match v1.clone() {
                                    Adt1::C0(h1274, _, _, _, _, _, _, _, _, _, _, _) => {
                                        h1274
                                    }
                                };
                                nat_add(a1275, a1276)?
                            };
                            let a1279 = match v1.clone() {
                                Adt1::C0(_, h1277, _, _, _, _, _, _, _, _, _, _) => {
                                    h1277
                                }
                            };
                            nat_add(a1278, a1279)?
                        };
                        nat_eq(a1280, a1281)
                    } else {
                        false
                    }
                } else {
                    false
                } {
                    Ok(Adt10::C4(f12(v3.clone(), 1u64)?, 4u64, f16(v2, 1u64), f16(v3, 1u64), Adt4::C4(match v1 {
                        Adt1::C0(h1282, _, _, _, _, _, _, _, _, _, _, _) => {
                            h1282
                        }
                    })))
                } else {
                    Ok(Adt10::C1)
                }
            } else {
                let a1283 = v0;
                let a1284 = 63u64;
                if nat_eq(a1283, a1284) {
                    if if if if if f98(match v2.clone() {
                        Adt2::C0(_, h1285, _, _, _, _, _, _, _, _, _) => {
                            h1285
                        }
                    }, match v1.clone() {
                        Adt1::C0(h1286, _, _, _, _, _, _, _, _, _, _, _) => {
                            h1286
                        }
                    })? {
                        f98(match v2.clone() {
                            Adt2::C0(_, _, h1287, _, _, _, _, _, _, _, _) => {
                                h1287
                            }
                        }, match v1.clone() {
                            Adt1::C0(_, h1288, _, _, _, _, _, _, _, _, _, _) => {
                                h1288
                            }
                        })?
                    } else {
                        false
                    } {
                        f98(match v2.clone() {
                            Adt2::C0(_, _, _, h1289, _, _, _, _, _, _, _) => {
                                h1289
                            }
                        }, match v1.clone() {
                            Adt1::C0(_, _, h1290, _, _, _, _, _, _, _, _, _) => {
                                h1290
                            }
                        })?
                    } else {
                        false
                    } {
                        f98(match v2.clone() {
                            Adt2::C0(_, _, _, _, h1291, _, _, _, _, _, _) => {
                                h1291
                            }
                        }, match v1.clone() {
                            Adt1::C0(_, _, _, h1292, _, _, _, _, _, _, _, _) => {
                                h1292
                            }
                        })?
                    } else {
                        false
                    } {
                        let a1295 = match v2.clone() {
                            Adt2::C0(_, h1293, _, _, _, _, _, _, _, _, _) => {
                                h1293
                            }
                        };
                        let a1296 = match v3.clone() {
                            Adt2::C0(_, h1294, _, _, _, _, _, _, _, _, _) => {
                                h1294
                            }
                        };
                        if if if nat_eq(a1295, a1296) {
                            let a1299 = match v2.clone() {
                                Adt2::C0(_, _, h1297, _, _, _, _, _, _, _, _) => {
                                    h1297
                                }
                            };
                            let a1300 = match v3.clone() {
                                Adt2::C0(_, _, h1298, _, _, _, _, _, _, _, _) => {
                                    h1298
                                }
                            };
                            nat_eq(a1299, a1300)
                        } else {
                            false
                        } {
                            let a1303 = match v2.clone() {
                                Adt2::C0(_, _, _, h1301, _, _, _, _, _, _, _) => {
                                    h1301
                                }
                            };
                            let a1304 = match v3.clone() {
                                Adt2::C0(_, _, _, h1302, _, _, _, _, _, _, _) => {
                                    h1302
                                }
                            };
                            nat_eq(a1303, a1304)
                        } else {
                            false
                        } {
                            let a1307 = match v2.clone() {
                                Adt2::C0(_, _, _, _, h1305, _, _, _, _, _, _) => {
                                    h1305
                                }
                            };
                            let a1308 = match v3.clone() {
                                Adt2::C0(_, _, _, _, h1306, _, _, _, _, _, _) => {
                                    h1306
                                }
                            };
                            nat_eq(a1307, a1308)
                        } else {
                            false
                        }
                    } else {
                        false
                    } {
                        Ok(Adt10::C4(f12(v3.clone(), 1u64)?, 4u64, f16(v2, 1u64), f16(v3, 1u64), Adt4::C5(match v1.clone() {
                            Adt1::C0(h1309, _, _, _, _, _, _, _, _, _, _, _) => {
                                h1309
                            }
                        }, match v1.clone() {
                            Adt1::C0(_, h1310, _, _, _, _, _, _, _, _, _, _) => {
                                h1310
                            }
                        }, match v1.clone() {
                            Adt1::C0(_, _, h1311, _, _, _, _, _, _, _, _, _) => {
                                h1311
                            }
                        }, match v1 {
                            Adt1::C0(_, _, _, h1312, _, _, _, _, _, _, _, _) => {
                                h1312
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

pub fn f46(v0: u64, v1: Adt1, v2: Adt2, v3: Option<Adt2>, v4: Adt2) -> R<Adt10> {
    let m1313 = v3;
    match m1313 {
        None => {
            Ok(Adt10::C1)
        }
        Some(v5) => {
            let v6: bool = {
                let a1314 = v0;
                let a1315 = 4u64;
                nat_eq(a1314, a1315)
            };
            let v7: bool = {
                let a1317 = 0u64;
                let a1318 = match v1.clone() {
                    Adt1::C0(_, _, _, _, h1316, _, _, _, _, _, _, _) => {
                        h1316
                    }
                };
                nat_lt(a1317, a1318)
            };
            let v8: u64 = f12(v5.clone(), 1u64)?;
            let v9: Adt3 = Adt3::C0(match v5.clone() {
                Adt2::C0(_, h1319, _, _, _, _, _, _, _, _, _) => {
                    h1319
                }
            }, match v5.clone() {
                Adt2::C0(_, _, h1320, _, _, _, _, _, _, _, _) => {
                    h1320
                }
            }, match v5.clone() {
                Adt2::C0(_, _, _, h1321, _, _, _, _, _, _, _) => {
                    h1321
                }
            }, match v5.clone() {
                Adt2::C0(_, _, _, _, h1322, _, _, _, _, _, _) => {
                    h1322
                }
            }, 4u64, match v1.clone() {
                Adt1::C0(h1323, _, _, _, _, _, _, _, _, _, _, _) => {
                    h1323
                }
            }, match v1.clone() {
                Adt1::C0(_, h1324, _, _, _, _, _, _, _, _, _, _) => {
                    h1324
                }
            }, match v1.clone() {
                Adt1::C0(_, _, h1325, _, _, _, _, _, _, _, _, _) => {
                    h1325
                }
            });
            if if if if if if if if if if if if v6 {
                let a1327 = match v2.clone() {
                    Adt2::C0(h1326, _, _, _, _, _, _, _, _, _, _) => {
                        h1326
                    }
                };
                let a1328 = 0u64;
                if nat_eq(a1327, a1328) {
                    let a1330 = match v5.clone() {
                        Adt2::C0(h1329, _, _, _, _, _, _, _, _, _, _) => {
                            h1329
                        }
                    };
                    let a1331 = 0u64;
                    nat_eq(a1330, a1331)
                } else {
                    false
                }
            } else {
                let a1333 = match v2.clone() {
                    Adt2::C0(h1332, _, _, _, _, _, _, _, _, _, _) => {
                        h1332
                    }
                };
                let a1334 = 0u64;
                if if nat_eq(a1333, a1334) {
                    true
                } else {
                    let a1336 = match v2.clone() {
                        Adt2::C0(h1335, _, _, _, _, _, _, _, _, _, _) => {
                            h1335
                        }
                    };
                    let a1337 = 26u64;
                    nat_eq(a1336, a1337)
                } {
                    let a1339 = match v5.clone() {
                        Adt2::C0(h1338, _, _, _, _, _, _, _, _, _, _) => {
                            h1338
                        }
                    };
                    let a1340 = 0u64;
                    if nat_eq(a1339, a1340) {
                        true
                    } else {
                        let a1342 = match v5.clone() {
                            Adt2::C0(h1341, _, _, _, _, _, _, _, _, _, _) => {
                                h1341
                            }
                        };
                        let a1343 = 26u64;
                        nat_eq(a1342, a1343)
                    }
                } else {
                    false
                }
            } {
                let a1346 = match v2.clone() {
                    Adt2::C0(h1344, _, _, _, _, _, _, _, _, _, _) => {
                        h1344
                    }
                };
                let a1347 = match v4.clone() {
                    Adt2::C0(h1345, _, _, _, _, _, _, _, _, _, _) => {
                        h1345
                    }
                };
                nat_eq(a1346, a1347)
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
                let a1349 = match v5.clone() {
                    Adt2::C0(_, _, _, _, _, h1348, _, _, _, _, _) => {
                        h1348
                    }
                };
                let a1350 = 4u64;
                nat_eq(a1349, a1350)
            } else {
                false
            } {
                let a1351 = v8;
                let a1352 = f12(v2.clone(), 1u64)?;
                nat_le(a1351, a1352)
            } else {
                false
            } {
                let a1354 = match v1.clone() {
                    Adt1::C0(_, _, _, h1353, _, _, _, _, _, _, _, _) => {
                        h1353
                    }
                };
                let a1355 = 1073741824u64;
                nat_lt(a1354, a1355)
            } else {
                false
            } {
                let a1357 = match v1.clone() {
                    Adt1::C0(h1356, _, _, _, _, _, _, _, _, _, _, _) => {
                        h1356
                    }
                };
                let a1358 = 4294967295u64;
                if if if if nat_le(a1357, a1358) {
                    let a1360 = match v1.clone() {
                        Adt1::C0(_, h1359, _, _, _, _, _, _, _, _, _, _) => {
                            h1359
                        }
                    };
                    let a1361 = 4294967295u64;
                    nat_le(a1360, a1361)
                } else {
                    false
                } {
                    let a1363 = match v1.clone() {
                        Adt1::C0(_, _, h1362, _, _, _, _, _, _, _, _, _) => {
                            h1362
                        }
                    };
                    let a1364 = 4294967295u64;
                    nat_le(a1363, a1364)
                } else {
                    false
                } {
                    let a1366 = match v1.clone() {
                        Adt1::C0(_, _, _, h1365, _, _, _, _, _, _, _, _) => {
                            h1365
                        }
                    };
                    let a1367 = 4294967295u64;
                    nat_le(a1366, a1367)
                } else {
                    false
                } {
                    let a1369 = match v1.clone() {
                        Adt1::C0(_, _, _, _, h1368, _, _, _, _, _, _, _) => {
                            h1368
                        }
                    };
                    let a1370 = 4294967295u64;
                    nat_le(a1369, a1370)
                } else {
                    false
                }
            } else {
                false
            } {
                if v7 {
                    let a1373 = match v2.clone() {
                        Adt2::C0(_, _, _, _, _, _, _, _, _, _, h1371) => {
                            h1371
                        }
                    };
                    let a1374 = match v4.clone() {
                        Adt2::C0(_, _, _, _, _, _, _, _, _, _, h1372) => {
                            h1372
                        }
                    };
                    nat_eq(a1373, a1374)
                } else {
                    true
                }
            } else {
                false
            } {
                f99(v9.clone(), match v1.clone() {
                    Adt1::C0(_, _, _, h1375, _, _, _, _, _, _, _, _) => {
                        h1375
                    }
                }, match v4 {
                    Adt2::C0(_, _, _, _, _, _, _, _, _, h1376, _) => {
                        h1376
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
                    let a1377 = v10;
                    let a1378 = v8;
                    nat_add(a1377, a1378)?
                }, 4u64, f16(v5, 1u64), v9, Adt4::C6(v10, match v1 {
                    Adt1::C0(_, _, _, h1379, _, _, _, _, _, _, _, _) => {
                        h1379
                    }
                }, v6)))
            } else {
                Ok(Adt10::C1)
            }
        }
    }
}

pub fn f47(v0: Adt2, v1: Adt2) -> R<Adt10> {
    let a1381 = match v0.clone() {
        Adt2::C0(h1380, _, _, _, _, _, _, _, _, _, _) => {
            h1380
        }
    };
    let a1382 = 0u64;
    if if if nat_eq(a1381, a1382) {
        let a1384 = match v1.clone() {
            Adt2::C0(h1383, _, _, _, _, _, _, _, _, _, _) => {
                h1383
            }
        };
        let a1385 = 0u64;
        nat_eq(a1384, a1385)
    } else {
        false
    } {
        f95(match v0.clone() {
            Adt2::C0(_, h1386, _, _, _, _, _, _, _, _, _) => {
                h1386
            }
        }, match v0.clone() {
            Adt2::C0(_, _, h1387, _, _, _, _, _, _, _, _) => {
                h1387
            }
        }, match v0.clone() {
            Adt2::C0(_, _, _, h1388, _, _, _, _, _, _, _) => {
                h1388
            }
        }, match v0.clone() {
            Adt2::C0(_, _, _, _, h1389, _, _, _, _, _, _) => {
                h1389
            }
        }, match v1.clone() {
            Adt2::C0(_, h1390, _, _, _, _, _, _, _, _, _) => {
                h1390
            }
        }, match v1.clone() {
            Adt2::C0(_, _, h1391, _, _, _, _, _, _, _, _) => {
                h1391
            }
        }, match v1.clone() {
            Adt2::C0(_, _, _, h1392, _, _, _, _, _, _, _) => {
                h1392
            }
        }, match v1.clone() {
            Adt2::C0(_, _, _, _, h1393, _, _, _, _, _, _) => {
                h1393
            }
        })
    } else {
        false
    } {
        let v2: u64 = f12(v1.clone(), 1u64)?;
        Ok(Adt10::C4({
            let a1394 = v2;
            let a1395 = f12(v0.clone(), 1u64)?;
            nat_add(a1394, a1395)?
        }, 4u64, f16(v0, 1u64), f16(v1, 1u64), Adt4::C7(v2)))
    } else {
        Ok(Adt10::C1)
    }
}

pub fn f48(v0: u64, v1: Adt2, v2: Adt2) -> R<Adt10> {
    let a1397 = match v1.clone() {
        Adt2::C0(h1396, _, _, _, _, _, _, _, _, _, _) => {
            h1396
        }
    };
    let a1398 = 0u64;
    if if if nat_eq(a1397, a1398) {
        true
    } else {
        let a1400 = match v1.clone() {
            Adt2::C0(h1399, _, _, _, _, _, _, _, _, _, _) => {
                h1399
            }
        };
        let a1401 = 1u64;
        nat_eq(a1400, a1401)
    } {
        true
    } else {
        let a1403 = match v1.clone() {
            Adt2::C0(h1402, _, _, _, _, _, _, _, _, _, _) => {
                h1402
            }
        };
        let a1404 = 30u64;
        nat_eq(a1403, a1404)
    } {
        let a1407 = match v1.clone() {
            Adt2::C0(h1405, _, _, _, _, _, _, _, _, _, _) => {
                h1405
            }
        };
        let a1408 = match v2.clone() {
            Adt2::C0(h1406, _, _, _, _, _, _, _, _, _, _) => {
                h1406
            }
        };
        if if if if if nat_eq(a1407, a1408) {
            let a1411 = match v1.clone() {
                Adt2::C0(_, h1409, _, _, _, _, _, _, _, _, _) => {
                    h1409
                }
            };
            let a1412 = match v2.clone() {
                Adt2::C0(_, h1410, _, _, _, _, _, _, _, _, _) => {
                    h1410
                }
            };
            nat_eq(a1411, a1412)
        } else {
            false
        } {
            let a1415 = match v1.clone() {
                Adt2::C0(_, _, _, h1413, _, _, _, _, _, _, _) => {
                    h1413
                }
            };
            let a1416 = match v2.clone() {
                Adt2::C0(_, _, _, h1414, _, _, _, _, _, _, _) => {
                    h1414
                }
            };
            nat_eq(a1415, a1416)
        } else {
            false
        } {
            let a1419 = match v1.clone() {
                Adt2::C0(_, _, _, _, h1417, _, _, _, _, _, _) => {
                    h1417
                }
            };
            let a1420 = match v2.clone() {
                Adt2::C0(_, _, _, _, h1418, _, _, _, _, _, _) => {
                    h1418
                }
            };
            nat_eq(a1419, a1420)
        } else {
            false
        } {
            let a1421 = v0;
            let a1422 = 4u64;
            if nat_eq(a1421, a1422) {
                let a1424 = match v1.clone() {
                    Adt2::C0(_, _, h1423, _, _, _, _, _, _, _, _) => {
                        h1423
                    }
                };
                let a1425 = 1u64;
                if nat_eq(a1424, a1425) {
                    let a1428 = match v1.clone() {
                        Adt2::C0(_, h1426, _, _, _, _, _, _, _, _, _) => {
                            h1426
                        }
                    };
                    let a1429 = match v2.clone() {
                        Adt2::C0(_, _, h1427, _, _, _, _, _, _, _, _) => {
                            h1427
                        }
                    };
                    nat_eq(a1428, a1429)
                } else {
                    false
                }
            } else {
                let a1430 = v0;
                let a1431 = 4u64;
                if if nat_lt(a1430, a1431) {
                    let a1434 = match v1.clone() {
                        Adt2::C0(_, h1432, _, _, _, _, _, _, _, _, _) => {
                            h1432
                        }
                    };
                    let a1435 = match v1.clone() {
                        Adt2::C0(_, _, h1433, _, _, _, _, _, _, _, _) => {
                            h1433
                        }
                    };
                    nat_eq(a1434, a1435)
                } else {
                    false
                } {
                    let a1438 = match v1.clone() {
                        Adt2::C0(_, _, h1436, _, _, _, _, _, _, _, _) => {
                            h1436
                        }
                    };
                    let a1439 = match v2.clone() {
                        Adt2::C0(_, _, h1437, _, _, _, _, _, _, _, _) => {
                            h1437
                        }
                    };
                    nat_eq(a1438, a1439)
                } else {
                    false
                }
            }
        } else {
            false
        } {
            let m1441 = f94(match v1.clone() {
                Adt2::C0(h1440, _, _, _, _, _, _, _, _, _, _) => {
                    h1440
                }
            });
            match m1441 {
                Adt14::C0 => {
                    Ok(Adt10::C0)
                }
                Adt14::C1(v3) => {
                    Ok(Adt10::C4(f12(v2.clone(), 1u64)?, match v3 {
                        Adt15::C0(_, _, h1442) => {
                            h1442
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

pub fn f49(v0: u64, v1: Adt2, v2: Adt2) -> R<Adt10> {
    if if if f100(match v1.clone() {
        Adt2::C0(h1443, _, _, _, _, _, _, _, _, _, _) => {
            h1443
        }
    }, v0) {
        let a1446 = match v1.clone() {
            Adt2::C0(h1444, _, _, _, _, _, _, _, _, _, _) => {
                h1444
            }
        };
        let a1447 = match v2.clone() {
            Adt2::C0(h1445, _, _, _, _, _, _, _, _, _, _) => {
                h1445
            }
        };
        nat_eq(a1446, a1447)
    } else {
        false
    } {
        let a1450 = match v1.clone() {
            Adt2::C0(_, h1448, _, _, _, _, _, _, _, _, _) => {
                h1448
            }
        };
        let a1451 = match v2.clone() {
            Adt2::C0(_, h1449, _, _, _, _, _, _, _, _, _) => {
                h1449
            }
        };
        if if if nat_eq(a1450, a1451) {
            let a1454 = match v1.clone() {
                Adt2::C0(_, _, h1452, _, _, _, _, _, _, _, _) => {
                    h1452
                }
            };
            let a1455 = match v2.clone() {
                Adt2::C0(_, _, h1453, _, _, _, _, _, _, _, _) => {
                    h1453
                }
            };
            nat_eq(a1454, a1455)
        } else {
            false
        } {
            let a1458 = match v1.clone() {
                Adt2::C0(_, _, _, h1456, _, _, _, _, _, _, _) => {
                    h1456
                }
            };
            let a1459 = match v2.clone() {
                Adt2::C0(_, _, _, h1457, _, _, _, _, _, _, _) => {
                    h1457
                }
            };
            nat_eq(a1458, a1459)
        } else {
            false
        } {
            let a1462 = match v1.clone() {
                Adt2::C0(_, _, _, _, h1460, _, _, _, _, _, _) => {
                    h1460
                }
            };
            let a1463 = match v2.clone() {
                Adt2::C0(_, _, _, _, h1461, _, _, _, _, _, _) => {
                    h1461
                }
            };
            nat_eq(a1462, a1463)
        } else {
            false
        }
    } else {
        false
    } {
        let m1465 = f94(match v1.clone() {
            Adt2::C0(h1464, _, _, _, _, _, _, _, _, _, _) => {
                h1464
            }
        });
        match m1465 {
            Adt14::C0 => {
                Ok(Adt10::C0)
            }
            Adt14::C1(v3) => {
                Ok(Adt10::C4(f12(v2.clone(), 1u64)?, match v3 {
                    Adt15::C0(_, _, h1466) => {
                        h1466
                    }
                }, f16(v1.clone(), 1u64), f16(v2, 1u64), Adt4::C9(match v1 {
                    Adt2::C0(h1467, _, _, _, _, _, _, _, _, _, _) => {
                        h1467
                    }
                }, v0)))
            }
        }
    } else {
        Ok(Adt10::C1)
    }
}

pub fn f50(v0: bool, v1: Adt1, v2: Adt2, v3: Adt2) -> R<Adt10> {
    let v4: u64 = if v0 {
        match v1.clone() {
            Adt1::C0(h1468, _, _, _, _, _, _, _, _, _, _, _) => {
                h1468
            }
        }
    } else {
        match v1.clone() {
            Adt1::C0(_, _, h1469, _, _, _, _, _, _, _, _, _) => {
                h1469
            }
        }
    };
    let a1470 = 0u64;
    let a1471 = v4;
    if if nat_lt(a1470, a1471) {
        let a1472 = v4;
        let a1473 = 2147483648u64;
        nat_lt(a1472, a1473)
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
        let v7: u64 = f101(match v5.clone() {
            Adt2::C0(_, _, h1474, _, _, _, _, _, _, _, _) => {
                h1474
            }
        }, v4)?;
        let v8: u64 = f101(match v5.clone() {
            Adt2::C0(_, _, _, h1475, _, _, _, _, _, _, _) => {
                h1475
            }
        }, v4)?;
        let a1477 = match v5.clone() {
            Adt2::C0(h1476, _, _, _, _, _, _, _, _, _, _) => {
                h1476
            }
        };
        let a1478 = 0u64;
        if if if if if nat_eq(a1477, a1478) {
            let a1480 = match v6.clone() {
                Adt2::C0(h1479, _, _, _, _, _, _, _, _, _, _) => {
                    h1479
                }
            };
            let a1481 = 0u64;
            nat_eq(a1480, a1481)
        } else {
            false
        } {
            let a1483 = match v5.clone() {
                Adt2::C0(_, _, _, _, h1482, _, _, _, _, _, _) => {
                    h1482
                }
            };
            let a1484 = 1u64;
            nat_eq(a1483, a1484)
        } else {
            false
        } {
            let a1487 = match v5.clone() {
                Adt2::C0(_, h1485, _, _, _, _, _, _, _, _, _) => {
                    h1485
                }
            };
            let a1488 = match v6.clone() {
                Adt2::C0(_, h1486, _, _, _, _, _, _, _, _, _) => {
                    h1486
                }
            };
            nat_eq(a1487, a1488)
        } else {
            false
        } {
            if v0 {
                let a1490 = match v5.clone() {
                    Adt2::C0(_, _, h1489, _, _, _, _, _, _, _, _) => {
                        h1489
                    }
                };
                let a1491 = 2147483648u64;
                if if if nat_lt(a1490, a1491) {
                    let a1493 = match v5.clone() {
                        Adt2::C0(_, _, _, h1492, _, _, _, _, _, _, _) => {
                            h1492
                        }
                    };
                    let a1494 = 2147483648u64;
                    nat_lt(a1493, a1494)
                } else {
                    false
                } {
                    let a1497 = {
                        let a1495 = v7;
                        let a1496 = v8;
                        nat_mul(a1495, a1496)?
                    };
                    let a1498 = 2147483648u64;
                    nat_le(a1497, a1498)
                } else {
                    false
                } {
                    f102(v5, v6, v4, v7)?
                } else {
                    false
                }
            } else {
                let a1500 = match v1.clone() {
                    Adt1::C0(h1499, _, _, _, _, _, _, _, _, _, _, _) => {
                        h1499
                    }
                };
                let a1501 = v7;
                if if if if nat_eq(a1500, a1501) {
                    let a1503 = match v1 {
                        Adt1::C0(_, h1502, _, _, _, _, _, _, _, _, _, _) => {
                            h1502
                        }
                    };
                    let a1504 = v8;
                    nat_eq(a1503, a1504)
                } else {
                    false
                } {
                    f103(v7, v8, v4)?
                } else {
                    false
                } {
                    let a1506 = match v6.clone() {
                        Adt2::C0(_, _, h1505, _, _, _, _, _, _, _, _) => {
                            h1505
                        }
                    };
                    let a1507 = v4;
                    if nat_eq(a1506, a1507) {
                        let a1509 = match v6.clone() {
                            Adt2::C0(_, _, _, h1508, _, _, _, _, _, _, _) => {
                                h1508
                            }
                        };
                        let a1510 = v4;
                        nat_eq(a1509, a1510)
                    } else {
                        false
                    }
                } else {
                    false
                } {
                    let a1514 = match v6 {
                        Adt2::C0(_, _, _, _, h1511, _, _, _, _, _, _) => {
                            h1511
                        }
                    };
                    let a1515 = {
                        let a1512 = v7;
                        let a1513 = v8;
                        nat_mul(a1512, a1513)?
                    };
                    nat_eq(a1514, a1515)
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

pub fn f51(v0: u64, v1: Adt1, v2: Adt2, v3: Adt2) -> R<Adt10> {
    let a1517 = match v2.clone() {
        Adt2::C0(h1516, _, _, _, _, _, _, _, _, _, _) => {
            h1516
        }
    };
    let a1518 = 0u64;
    if if if if if nat_eq(a1517, a1518) {
        true
    } else {
        let a1520 = match v2.clone() {
            Adt2::C0(h1519, _, _, _, _, _, _, _, _, _, _) => {
                h1519
            }
        };
        let a1521 = 1u64;
        nat_eq(a1520, a1521)
    } {
        true
    } else {
        let a1523 = match v2.clone() {
            Adt2::C0(h1522, _, _, _, _, _, _, _, _, _, _) => {
                h1522
            }
        };
        let a1524 = 30u64;
        nat_eq(a1523, a1524)
    } {
        let a1527 = match v2.clone() {
            Adt2::C0(h1525, _, _, _, _, _, _, _, _, _, _) => {
                h1525
            }
        };
        let a1528 = match v3.clone() {
            Adt2::C0(h1526, _, _, _, _, _, _, _, _, _, _) => {
                h1526
            }
        };
        nat_eq(a1527, a1528)
    } else {
        false
    } {
        if if f19(v2.clone(), v3.clone()) {
            let a1530 = match v1.clone() {
                Adt1::C0(h1529, _, _, _, _, _, _, _, _, _, _, _) => {
                    h1529
                }
            };
            let a1531 = 4294967296u64;
            nat_lt(a1530, a1531)
        } else {
            false
        } {
            let a1533 = match v1.clone() {
                Adt1::C0(_, h1532, _, _, _, _, _, _, _, _, _, _) => {
                    h1532
                }
            };
            let a1534 = 4294967296u64;
            nat_lt(a1533, a1534)
        } else {
            false
        }
    } else {
        false
    } {
        Ok(Adt10::C4(f12(v3.clone(), 1u64)?, {
            let a1536 = match v2.clone() {
                Adt2::C0(h1535, _, _, _, _, _, _, _, _, _, _) => {
                    h1535
                }
            };
            let a1537 = 0u64;
            if nat_eq(a1536, a1537) {
                4u64
            } else {
                2u64
            }
        }, f16(v2.clone(), 1u64), f16(v3, 1u64), Adt4::C13({
            let a1538 = v0;
            let a1539 = 31u64;
            if nat_eq(a1538, a1539) {
                0u64
            } else {
                let a1540 = v0;
                let a1541 = 47u64;
                if nat_eq(a1540, a1541) {
                    1u64
                } else {
                    2u64
                }
            }
        }, match v2 {
            Adt2::C0(h1542, _, _, _, _, _, _, _, _, _, _) => {
                h1542
            }
        }, match v1.clone() {
            Adt1::C0(h1543, _, _, _, _, _, _, _, _, _, _, _) => {
                h1543
            }
        }, match v1 {
            Adt1::C0(_, h1544, _, _, _, _, _, _, _, _, _, _) => {
                h1544
            }
        })))
    } else {
        Ok(Adt10::C1)
    }
}

pub fn f52(v0: Adt2, v1: Adt2) -> R<Adt10> {
    let a1546 = match v0.clone() {
        Adt2::C0(h1545, _, _, _, _, _, _, _, _, _, _) => {
            h1545
        }
    };
    let a1547 = 0u64;
    if if if nat_eq(a1546, a1547) {
        let a1549 = match v1.clone() {
            Adt2::C0(h1548, _, _, _, _, _, _, _, _, _, _) => {
                h1548
            }
        };
        let a1550 = 0u64;
        nat_eq(a1549, a1550)
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

pub fn f53(v0: Adt2, v1: Adt2) -> R<Adt10> {
    let a1552 = match v0.clone() {
        Adt2::C0(h1551, _, _, _, _, _, _, _, _, _, _) => {
            h1551
        }
    };
    let a1553 = 0u64;
    if if nat_eq(a1552, a1553) {
        let a1555 = match v1.clone() {
            Adt2::C0(h1554, _, _, _, _, _, _, _, _, _, _) => {
                h1554
            }
        };
        let a1556 = 26u64;
        nat_eq(a1555, a1556)
    } else {
        false
    } {
        let a1558 = match v0.clone() {
            Adt2::C0(_, _, _, h1557, _, _, _, _, _, _, _) => {
                h1557
            }
        };
        let a1559 = 1u64;
        if if if if if if if nat_eq(a1558, a1559) {
            let a1561 = match v0.clone() {
                Adt2::C0(_, _, _, _, h1560, _, _, _, _, _, _) => {
                    h1560
                }
            };
            let a1562 = 1u64;
            nat_eq(a1561, a1562)
        } else {
            false
        } {
            let a1564 = match v0.clone() {
                Adt2::C0(_, h1563, _, _, _, _, _, _, _, _, _) => {
                    h1563
                }
            };
            let a1565 = 2147483647u64;
            nat_le(a1564, a1565)
        } else {
            false
        } {
            let a1568 = match v1.clone() {
                Adt2::C0(_, h1566, _, _, _, _, _, _, _, _, _) => {
                    h1566
                }
            };
            let a1569 = match v0.clone() {
                Adt2::C0(_, _, h1567, _, _, _, _, _, _, _, _) => {
                    h1567
                }
            };
            nat_eq(a1568, a1569)
        } else {
            false
        } {
            let a1571 = match v1.clone() {
                Adt2::C0(_, _, h1570, _, _, _, _, _, _, _, _) => {
                    h1570
                }
            };
            let a1572 = 1u64;
            nat_eq(a1571, a1572)
        } else {
            false
        } {
            let a1574 = match v1.clone() {
                Adt2::C0(_, _, _, h1573, _, _, _, _, _, _, _) => {
                    h1573
                }
            };
            let a1575 = 1u64;
            nat_eq(a1574, a1575)
        } else {
            false
        } {
            let a1577 = match v1.clone() {
                Adt2::C0(_, _, _, _, h1576, _, _, _, _, _, _) => {
                    h1576
                }
            };
            let a1578 = 1u64;
            nat_eq(a1577, a1578)
        } else {
            false
        } {
            Ok(Adt10::C4({
                let a1581 = match v0.clone() {
                    Adt2::C0(_, _, h1579, _, _, _, _, _, _, _, _) => {
                        h1579
                    }
                };
                let a1582 = f104(match v0.clone() {
                    Adt2::C0(_, h1580, _, _, _, _, _, _, _, _, _) => {
                        h1580
                    }
                })?;
                nat_mul(a1581, a1582)?
            }, 4u64, f16(v0, 1u64), f16(v1, 1u64), Adt4::C17))
        } else {
            Ok(Adt10::C1)
        }
    } else {
        Ok(Adt10::C0)
    }
}

pub fn f54(v0: Adt2, v1: Option<Adt2>, v2: Adt2) -> R<Adt10> {
    let m1583 = v1;
    match m1583 {
        None => {
            Ok(Adt10::C1)
        }
        Some(v3) => {
            let v4: u64 = f105(f16(v0.clone(), 1u64))?;
            let a1585 = match v0.clone() {
                Adt2::C0(h1584, _, _, _, _, _, _, _, _, _, _) => {
                    h1584
                }
            };
            let a1586 = 26u64;
            if if if nat_eq(a1585, a1586) {
                let a1588 = match v3.clone() {
                    Adt2::C0(h1587, _, _, _, _, _, _, _, _, _, _) => {
                        h1587
                    }
                };
                let a1589 = 26u64;
                nat_eq(a1588, a1589)
            } else {
                false
            } {
                let a1591 = match v2.clone() {
                    Adt2::C0(h1590, _, _, _, _, _, _, _, _, _, _) => {
                        h1590
                    }
                };
                let a1592 = 27u64;
                nat_eq(a1591, a1592)
            } else {
                false
            } {
                if if if if if if f19(v0.clone(), v3.clone()) {
                    let a1594 = match v2.clone() {
                        Adt2::C0(_, h1593, _, _, _, _, _, _, _, _, _) => {
                            h1593
                        }
                    };
                    let a1595 = 1u64;
                    nat_eq(a1594, a1595)
                } else {
                    false
                } {
                    let a1597 = match v2.clone() {
                        Adt2::C0(_, _, h1596, _, _, _, _, _, _, _, _) => {
                            h1596
                        }
                    };
                    let a1598 = 1u64;
                    nat_eq(a1597, a1598)
                } else {
                    false
                } {
                    let a1600 = match v2.clone() {
                        Adt2::C0(_, _, _, h1599, _, _, _, _, _, _, _) => {
                            h1599
                        }
                    };
                    let a1601 = 1u64;
                    nat_eq(a1600, a1601)
                } else {
                    false
                } {
                    let a1603 = match v2.clone() {
                        Adt2::C0(_, _, _, _, h1602, _, _, _, _, _, _) => {
                            h1602
                        }
                    };
                    let a1604 = 1u64;
                    nat_eq(a1603, a1604)
                } else {
                    false
                } {
                    let a1605 = v4;
                    let a1606 = 9223372036854775808u64;
                    nat_lt(a1605, a1606)
                } else {
                    false
                } {
                    Ok(Adt10::C4({
                        let a1609 = 2u64;
                        let a1610 = {
                            let a1607 = v4;
                            let a1608 = 2u64;
                            nat_mul(a1607, a1608)?
                        };
                        nat_add(a1609, a1610)?
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

pub fn f55(v0: u64, v1: Adt2, v2: Adt2) -> R<Adt10> {
    let v3: u64 = match v1.clone() {
        Adt2::C0(h1611, _, _, _, _, _, _, _, _, _, _) => {
            h1611
        }
    };
    let a1612 = v3;
    let a1613 = 0u64;
    if if nat_eq(a1612, a1613) {
        true
    } else {
        let a1614 = v0;
        let a1615 = 13u64;
        if nat_eq(a1614, a1615) {
            let a1616 = v3;
            let a1617 = 1u64;
            if nat_eq(a1616, a1617) {
                true
            } else {
                let a1618 = v3;
                let a1619 = 30u64;
                nat_eq(a1618, a1619)
            }
        } else {
            false
        }
    } {
        let a1621 = v3;
        let a1622 = match v2.clone() {
            Adt2::C0(h1620, _, _, _, _, _, _, _, _, _, _) => {
                h1620
            }
        };
        if if nat_eq(a1621, a1622) {
            let a1623 = v0;
            let a1624 = 13u64;
            if nat_eq(a1623, a1624) {
                let a1626 = match v2.clone() {
                    Adt2::C0(_, h1625, _, _, _, _, _, _, _, _, _) => {
                        h1625
                    }
                };
                let a1627 = 1u64;
                if if if nat_eq(a1626, a1627) {
                    let a1629 = match v2.clone() {
                        Adt2::C0(_, _, h1628, _, _, _, _, _, _, _, _) => {
                            h1628
                        }
                    };
                    let a1630 = 1u64;
                    nat_eq(a1629, a1630)
                } else {
                    false
                } {
                    let a1632 = match v2.clone() {
                        Adt2::C0(_, _, _, h1631, _, _, _, _, _, _, _) => {
                            h1631
                        }
                    };
                    let a1633 = 1u64;
                    nat_eq(a1632, a1633)
                } else {
                    false
                } {
                    let a1635 = match v2.clone() {
                        Adt2::C0(_, _, _, _, h1634, _, _, _, _, _, _) => {
                            h1634
                        }
                    };
                    let a1636 = 1u64;
                    nat_eq(a1635, a1636)
                } else {
                    false
                }
            } else {
                let a1638 = match v2.clone() {
                    Adt2::C0(_, h1637, _, _, _, _, _, _, _, _, _) => {
                        h1637
                    }
                };
                let a1639 = 1u64;
                if if if nat_eq(a1638, a1639) {
                    let a1642 = match v1.clone() {
                        Adt2::C0(_, _, h1640, _, _, _, _, _, _, _, _) => {
                            h1640
                        }
                    };
                    let a1643 = match v2.clone() {
                        Adt2::C0(_, _, h1641, _, _, _, _, _, _, _, _) => {
                            h1641
                        }
                    };
                    nat_eq(a1642, a1643)
                } else {
                    false
                } {
                    let a1646 = match v1.clone() {
                        Adt2::C0(_, _, _, h1644, _, _, _, _, _, _, _) => {
                            h1644
                        }
                    };
                    let a1647 = match v2.clone() {
                        Adt2::C0(_, _, _, h1645, _, _, _, _, _, _, _) => {
                            h1645
                        }
                    };
                    nat_eq(a1646, a1647)
                } else {
                    false
                } {
                    let a1650 = match v1.clone() {
                        Adt2::C0(_, _, _, _, h1648, _, _, _, _, _, _) => {
                            h1648
                        }
                    };
                    let a1651 = match v2.clone() {
                        Adt2::C0(_, _, _, _, h1649, _, _, _, _, _, _) => {
                            h1649
                        }
                    };
                    nat_eq(a1650, a1651)
                } else {
                    false
                }
            }
        } else {
            false
        } {
            let v4: Adt3 = f16(v1, 1u64);
            Ok(Adt10::C4(f106(v4.clone(), v0, v3)?, {
                let a1652 = v3;
                let a1653 = 0u64;
                if nat_eq(a1652, a1653) {
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

pub fn f56(v0: u64, v1: u64) -> R<Option<u64>> {
    let a1654 = v0;
    let a1655 = 89u64;
    if if nat_eq(a1654, a1655) {
        let a1656 = 8u64;
        let a1657 = v1;
        if nat_le(a1656, a1657) {
            let a1658 = v1;
            let a1659 = 9u64;
            nat_le(a1658, a1659)
        } else {
            false
        }
    } else {
        false
    } {
        Ok(Some({
            let a1660 = v1;
            let a1661 = 12u64;
            nat_add(a1660, a1661)?
        }))
    } else {
        let a1662 = v0;
        let a1663 = 89u64;
        if if nat_eq(a1662, a1663) {
            let a1664 = v1;
            let a1665 = 4u64;
            nat_eq(a1664, a1665)
        } else {
            false
        } {
            Ok(Some(19u64))
        } else {
            let a1666 = 10u64;
            let a1667 = v0;
            if if nat_le(a1666, a1667) {
                let a1668 = v0;
                let a1669 = 12u64;
                nat_le(a1668, a1669)
            } else {
                false
            } {
                Ok(Some({
                    let a1670 = v0;
                    let a1671 = 3u64;
                    nat_add(a1670, a1671)?
                }))
            } else {
                let a1672 = v0;
                let a1673 = 89u64;
                if if nat_eq(a1672, a1673) {
                    let a1674 = v1;
                    let a1675 = 7u64;
                    nat_eq(a1674, a1675)
                } else {
                    false
                } {
                    Ok(Some(17u64))
                } else {
                    let a1676 = v0;
                    let a1677 = 89u64;
                    if if nat_eq(a1676, a1677) {
                        let a1678 = v1;
                        let a1679 = 10u64;
                        nat_eq(a1678, a1679)
                    } else {
                        false
                    } {
                        Ok(Some(18u64))
                    } else {
                        let a1680 = v0;
                        let a1681 = 89u64;
                        if if nat_eq(a1680, a1681) {
                            let a1682 = v1;
                            let a1683 = 13u64;
                            nat_eq(a1682, a1683)
                        } else {
                            false
                        } {
                            Ok(Some(16u64))
                        } else {
                            let a1684 = v0;
                            let a1685 = 8u64;
                            if nat_eq(a1684, a1685) {
                                Ok(Some(5u64))
                            } else {
                                let a1686 = v0;
                                let a1687 = 9u64;
                                if nat_eq(a1686, a1687) {
                                    Ok(Some(6u64))
                                } else {
                                    let a1688 = v0;
                                    let a1689 = 89u64;
                                    if nat_eq(a1688, a1689) {
                                        let a1690 = v1;
                                        let a1691 = 0u64;
                                        if nat_eq(a1690, a1691) {
                                            Ok(Some(0u64))
                                        } else {
                                            let a1692 = v1;
                                            let a1693 = 1u64;
                                            if nat_eq(a1692, a1693) {
                                                Ok(Some(1u64))
                                            } else {
                                                let a1694 = v1;
                                                let a1695 = 2u64;
                                                if nat_eq(a1694, a1695) {
                                                    Ok(Some(2u64))
                                                } else {
                                                    let a1696 = v1;
                                                    let a1697 = 3u64;
                                                    if nat_eq(a1696, a1697) {
                                                        Ok(Some(3u64))
                                                    } else {
                                                        let a1698 = v1;
                                                        let a1699 = 6u64;
                                                        if nat_eq(a1698, a1699) {
                                                            Ok(Some(4u64))
                                                        } else {
                                                            let a1700 = v1;
                                                            let a1701 = 12u64;
                                                            if nat_eq(a1700, a1701) {
                                                                Ok(Some(7u64))
                                                            } else {
                                                                let a1702 = v1;
                                                                let a1703 = 11u64;
                                                                if nat_eq(a1702, a1703) {
                                                                    Ok(Some(8u64))
                                                                } else {
                                                                    let a1704 = v1;
                                                                    let a1705 = 18u64;
                                                                    if nat_eq(a1704, a1705) {
                                                                        Ok(Some(9u64))
                                                                    } else {
                                                                        let a1706 = v1;
                                                                        let a1707 = 19u64;
                                                                        if nat_eq(a1706, a1707) {
                                                                            Ok(Some(10u64))
                                                                        } else {
                                                                            let a1708 = v1;
                                                                            let a1709 = 20u64;
                                                                            if nat_eq(a1708, a1709) {
                                                                                Ok(Some(11u64))
                                                                            } else {
                                                                                let a1710 = v1;
                                                                                let a1711 = 21u64;
                                                                                if nat_eq(a1710, a1711) {
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

pub fn f57(v0: u64, v1: u64, v2: Adt2, v3: Option<Adt2>, v4: Adt2) -> R<Adt10> {
    let m1712 = f107(v0, v1, v2.clone(), v3.clone(), v4.clone())?;
    match m1712 {
        Adt10::C0 => {
            Ok(Adt10::C0)
        }
        Adt10::C1 => {
            let a1713 = v0;
            let a1714 = 2u64;
            if if if if nat_eq(a1713, a1714) {
                true
            } else {
                let a1715 = v0;
                let a1716 = 5u64;
                nat_eq(a1715, a1716)
            } {
                true
            } else {
                let a1717 = v0;
                let a1718 = 6u64;
                nat_eq(a1717, a1718)
            } {
                true
            } else {
                let a1719 = v0;
                let a1720 = 7u64;
                nat_eq(a1719, a1720)
            } {
                Ok(Adt10::C5(67u64))
            } else {
                Ok(Adt10::C1)
            }
        }
        Adt10::C2(v7, v8, _, _, _, _, v13) => {
            if v8 {
                Ok(Adt10::C4(v13, 4u64, f16(v2.clone(), 1u64), f16(v4, 1u64), Adt4::C10(v7, v8, {
                    let m1721 = v3;
                    match m1721 {
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

pub fn f58(v0: u64, v1: Adt2, v2: Adt2) -> R<Adt10> {
    let a1723 = match v1.clone() {
        Adt2::C0(h1722, _, _, _, _, _, _, _, _, _, _) => {
            h1722
        }
    };
    let a1724 = 0u64;
    if if if if if nat_eq(a1723, a1724) {
        true
    } else {
        let a1726 = match v1.clone() {
            Adt2::C0(h1725, _, _, _, _, _, _, _, _, _, _) => {
                h1725
            }
        };
        let a1727 = 1u64;
        nat_eq(a1726, a1727)
    } {
        true
    } else {
        let a1729 = match v1.clone() {
            Adt2::C0(h1728, _, _, _, _, _, _, _, _, _, _) => {
                h1728
            }
        };
        let a1730 = 30u64;
        nat_eq(a1729, a1730)
    } {
        let a1733 = match v1.clone() {
            Adt2::C0(h1731, _, _, _, _, _, _, _, _, _, _) => {
                h1731
            }
        };
        let a1734 = match v2.clone() {
            Adt2::C0(h1732, _, _, _, _, _, _, _, _, _, _) => {
                h1732
            }
        };
        if nat_eq(a1733, a1734) {
            true
        } else {
            let a1736 = match v2.clone() {
                Adt2::C0(h1735, _, _, _, _, _, _, _, _, _, _) => {
                    h1735
                }
            };
            let a1737 = 0u64;
            nat_eq(a1736, a1737)
        }
    } else {
        false
    } {
        f19(v1.clone(), v2.clone())
    } else {
        false
    } {
        Ok(Adt10::C4(f12(v2.clone(), 1u64)?, {
            let a1739 = match v2.clone() {
                Adt2::C0(h1738, _, _, _, _, _, _, _, _, _, _) => {
                    h1738
                }
            };
            let a1740 = 0u64;
            if nat_eq(a1739, a1740) {
                4u64
            } else {
                2u64
            }
        }, f16(v1.clone(), 1u64), f16(v2.clone(), 1u64), Adt4::C14(v0, match v1 {
            Adt2::C0(h1741, _, _, _, _, _, _, _, _, _, _) => {
                h1741
            }
        }, match v2 {
            Adt2::C0(h1742, _, _, _, _, _, _, _, _, _, _) => {
                h1742
            }
        })))
    } else {
        Ok(Adt10::C1)
    }
}

pub fn f59(v0: Adt3, v1: u64, v2: u64, v3: u64, v4: u64) -> R<u64> {
    f108(v1, v2, v3, v4, match v0.clone() {
        Adt3::C0(_, _, _, _, h1743, _, _, _) => {
            h1743
        }
    }, match v0.clone() {
        Adt3::C0(_, _, _, _, _, h1744, _, _) => {
            h1744
        }
    }, match v0.clone() {
        Adt3::C0(_, _, _, _, _, _, h1745, _) => {
            h1745
        }
    }, match v0 {
        Adt3::C0(_, _, _, _, _, _, _, h1746) => {
            h1746
        }
    })
}

pub fn f60(v0: u64, v1: Adt3, v2: u64) -> u64 {
    let v3: u64 = {
        let a1748 = v0;
        let a1749 = match v1.clone() {
            Adt3::C0(h1747, _, _, _, _, _, _, _) => {
                h1747
            }
        };
        let a1750 = 0u64;
        nat_quot(a1748, a1749, a1750)
    };
    let v4: u64 = {
        let a1752 = v3;
        let a1753 = match v1.clone() {
            Adt3::C0(_, h1751, _, _, _, _, _, _) => {
                h1751
            }
        };
        let a1754 = 0u64;
        nat_quot(a1752, a1753, a1754)
    };
    let a1755 = v2;
    let a1756 = 0u64;
    if nat_eq(a1755, a1756) {
        let a1758 = v0;
        let a1759 = match v1 {
            Adt3::C0(h1757, _, _, _, _, _, _, _) => {
                h1757
            }
        };
        let a1760 = 0u64;
        nat_rem(a1758, a1759, a1760)
    } else {
        let a1761 = v2;
        let a1762 = 1u64;
        if nat_eq(a1761, a1762) {
            let a1764 = v3;
            let a1765 = match v1 {
                Adt3::C0(_, h1763, _, _, _, _, _, _) => {
                    h1763
                }
            };
            let a1766 = 0u64;
            nat_rem(a1764, a1765, a1766)
        } else {
            let a1767 = v2;
            let a1768 = 2u64;
            if nat_eq(a1767, a1768) {
                let a1770 = v4;
                let a1771 = match v1 {
                    Adt3::C0(_, _, h1769, _, _, _, _, _) => {
                        h1769
                    }
                };
                let a1772 = 0u64;
                nat_rem(a1770, a1771, a1772)
            } else {
                let a1774 = v4;
                let a1775 = match v1 {
                    Adt3::C0(_, _, h1773, _, _, _, _, _) => {
                        h1773
                    }
                };
                let a1776 = 0u64;
                nat_quot(a1774, a1775, a1776)
            }
        }
    }
}

pub fn f61(v0: Adt8, v1: u64, v2: u64) -> R<u64> {
    let a1778 = match v0.clone() {
        Adt8::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, h1777) => {
            h1777
        }
    };
    let a1779 = 0u64;
    if nat_eq(a1778, a1779) {
        Ok(v2)
    } else {
        let a1784 = {
            let a1781 = v2;
            let a1782 = match v0.clone() {
                Adt8::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, h1780) => {
                    h1780
                }
            };
            nat_mul(a1781, a1782)?
        };
        let a1785 = f60(v1, match v0 {
            Adt8::C0(_, _, _, _, _, _, _, _, h1783, _, _, _, _, _, _) => {
                h1783
            }
        }, 3u64);
        nat_add(a1784, a1785)
    }
}

pub fn f62(v0: Adt6, v1: u64, v2: u64, v3: u64) -> R<Option<u64>> {
    let v4: u64 = {
        let a1795 = {
            let a1792 = {
                let a1787 = v2;
                let a1788 = match v0.clone() {
                    Adt6::C0(_, h1786, _, _) => {
                        h1786
                    }
                };
                nat_mul(a1787, a1788)?
            };
            let a1793 = {
                let a1790 = v3;
                let a1791 = match v0.clone() {
                    Adt6::C0(_, _, _, h1789) => {
                        h1789
                    }
                };
                nat_mul(a1790, a1791)?
            };
            nat_add(a1792, a1793)?
        };
        let a1796 = f109(match v0.clone() {
            Adt6::C0(_, _, h1794, _) => {
                h1794
            }
        });
        nat_add(a1795, a1796)?
    };
    let v5: u64 = f110(match v0 {
        Adt6::C0(_, _, h1797, _) => {
            h1797
        }
    });
    let a1798 = v5;
    let a1799 = v4;
    if if nat_le(a1798, a1799) {
        let a1802 = {
            let a1800 = v4;
            let a1801 = v5;
            nat_sub(a1800, a1801)
        };
        let a1803 = v1;
        nat_lt(a1802, a1803)
    } else {
        false
    } {
        Ok(Some({
            let a1804 = v4;
            let a1805 = v5;
            nat_sub(a1804, a1805)
        }))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f63(v0: Adt8, v1: u64, v2: u64, v3: u64, v4: u64) -> R<Adt9> {
    let v5: Adt3 = match v0.clone() {
        Adt8::C0(_, _, _, _, _, _, _, _, _, h1806, _, _, _, _, _) => {
            h1806
        }
    };
    let v6: Adt3 = match v0.clone() {
        Adt8::C0(_, _, _, _, _, _, _, _, h1807, _, _, _, _, _, _) => {
            h1807
        }
    };
    let v7: u64 = f60(v1, v6.clone(), 2u64);
    let a1809 = match v0.clone() {
        Adt8::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, h1808) => {
            h1808
        }
    };
    let a1810 = 0u64;
    if nat_eq(a1809, a1810) {
        Ok(Adt9::C8(Some(f59(v5, v3, v4, v7, v2)?)))
    } else {
        let v8: u64 = {
            let m1813 = match v0.clone() {
                Adt8::C0(_, _, _, _, _, _, _, _, _, _, h1812, _, _, _, _) => {
                    h1812
                }
            };
            match m1813 {
                Adt3::C0(_, _, h1811, _, _, _, _, _) => {
                    h1811
                }
            }
        };
        let m1819 = f62(match v0.clone() {
            Adt8::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, h1814, _) => {
                h1814
            }
        }, match v5.clone() {
            Adt3::C0(_, _, h1815, _, _, _, _, _) => {
                h1815
            }
        }, {
            let a1816 = v2;
            let a1817 = v8;
            let a1818 = 0u64;
            nat_rem(a1816, a1817, a1818)
        }, v7)?;
        match m1819 {
            None => {
                Ok(Adt9::C8(None::<u64>))
            }
            Some(v9) => {
                let v10: u64 = {
                    let a1826 = {
                        let a1824 = {
                            let a1820 = v2;
                            let a1821 = v8;
                            let a1822 = 0u64;
                            nat_quot(a1820, a1821, a1822)
                        };
                        let a1825 = match v0 {
                            Adt8::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, h1823) => {
                                h1823
                            }
                        };
                        nat_mul(a1824, a1825)?
                    };
                    let a1827 = f60(v1, v6, 3u64);
                    nat_add(a1826, a1827)?
                };
                Ok(Adt9::C8(Some(f59(v5, v3, v4, v9, v10)?)))
            }
        }
    }
}

pub fn f64(v0: Adt4, v1: Adt9) -> Adt9 {
    let m1828 = v1;
    match m1828 {
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
            f111(f112(v0.clone(), v14, v15), v14, v15, 0u64, v16, 2u64, f113(v0), 3u64, v17)
        }
        Adt9::C4(v18, v19, v20, v21, v22, v23, v24) => {
            let v25: u64 = f114(v0.clone(), v18);
            f111(f115(v0, v18), v25, v25, v19, v20, v21, v22, v23, v24)
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

pub fn f65(v0: u64, v1: Adt3, v2: Adt3, v3: Adt4) -> R<Adt9> {
    let m1829 = v3;
    match m1829 {
        Adt4::C0 => {
            Ok(Adt9::C1(false, f116(v0, v1)?, f116(v0, v2)?))
        }
        Adt4::C1 => {
            Ok(Adt9::C1(false, f118(v0, v1, v2.clone())?, f116(v0, v2)?))
        }
        Adt4::C2(v6, v7) => {
            f119(v0, v1, v7, v2, v6)
        }
        Adt4::C3(v8, v9, v10, v11, v12) => {
            f120(v0, v1, v2, v8, v9, v10, v11, v12)
        }
        Adt4::C4(v13) => {
            f121(v0, v1, v2, v13)
        }
        Adt4::C5(v14, v15, v16, v17) => {
            f120(v0, v1, v2, v14, v15, v16, v17, true)
        }
        Adt4::C6(v18, v19, v20) => {
            f122(v0, v1, v2, v18, v19, v20)
        }
        Adt4::C7(v21) => {
            f123(v0, v1, v2, v21)
        }
        Adt4::C8(v22) => {
            f124(v22, v0, v1, v2)
        }
        Adt4::C9(_, _) => {
            Ok(Adt9::C1(true, 0u64, f116(v0, v2)?))
        }
        Adt4::C10(v25, v26, v27) => {
            f125(v0, v1, v2, v25, v26, v27)
        }
        Adt4::C11(v28, v29) => {
            Ok(Adt9::C3(v28, v29, f116(v0, v1)?, f116(v0, v2)?))
        }
        Adt4::C12(v30, v31, v32) => {
            f126(v0, v1, v2, v30, v31, v32)
        }
        Adt4::C13(_, v34, _, _) => {
            Ok(Adt9::C3(v34, v34, f116(v0, v1)?, f116(v0, v2)?))
        }
        Adt4::C14(_, v38, v39) => {
            Ok(Adt9::C3(v38, v39, f116(v0, v1)?, f116(v0, v2)?))
        }
        Adt4::C15 => {
            f127(v0, v1, v2)
        }
        Adt4::C16(v40, v41) => {
            f128(v0, v1, v2, v40, v41)
        }
        Adt4::C17 => {
            Ok(Adt9::C0(Adt10::C1))
        }
        Adt4::C18(v42, v43) => {
            f129(v0, v1, v43, v42)
        }
        Adt4::C19(_, _) => {
            Ok(Adt9::C0(Adt10::C1))
        }
        Adt4::C20(_, _) => {
            Ok(Adt9::C0(Adt10::C1))
        }
        Adt4::C21 => {
            Ok(Adt9::C1(false, f116(f117(v0, match v2.clone() {
                Adt3::C0(h1830, _, _, _, _, _, _, _) => {
                    h1830
                }
            }, match v2.clone() {
                Adt3::C0(_, h1831, _, _, _, _, _, _) => {
                    h1831
                }
            })?, v1)?, f116(v0, v2)?))
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
            f130(v0, v48, v49, v50)
        }
        Adt4::C26(v54, _, _, _) => {
            let a1832 = v54;
            let a1833 = 64u64;
            if if if nat_eq(a1832, a1833) {
                true
            } else {
                let a1834 = v54;
                let a1835 = 65u64;
                nat_eq(a1834, a1835)
            } {
                true
            } else {
                let a1836 = v54;
                let a1837 = 66u64;
                nat_eq(a1836, a1837)
            } {
                Ok(Adt9::C0(Adt10::C5(64u64)))
            } else {
                Ok(Adt9::C0(Adt10::C1))
            }
        }
        Adt4::C27(v53) => {
            f72(v0, v1, v2, v53)
        }
        Adt4::C28(v51) => {
            f70(v0, v1, v2, v51, 0u64)
        }
        Adt4::C29(v52) => {
            f71(v0, v1, v2, v52)
        }
    }
}

pub fn f66(v0: u64, v1: Adt3, v2: Adt3, v3: u64) -> R<Adt9> {
    let v4: u64 = f104(match v1.clone() {
        Adt3::C0(h1838, _, _, _, _, _, _, _) => {
            h1838
        }
    })?;
    let v5: u64 = {
        let a1839 = v0;
        let a1840 = v4;
        let a1841 = 0u64;
        nat_quot(a1839, a1840, a1841)
    };
    let v6: u64 = {
        let a1842 = v0;
        let a1843 = v4;
        let a1844 = 0u64;
        nat_rem(a1842, a1843, a1844)
    };
    let a1845 = v6;
    let a1846 = 0u64;
    if nat_eq(a1845, a1846) {
        Ok(Adt9::C4(19u64, 2u64, 4286578688u64, 2u64, 0u64, 1u64, 0u64))
    } else {
        let a1847 = v6;
        let a1848 = 1u64;
        if nat_eq(a1847, a1848) {
            Ok(Adt9::C4(19u64, 2u64, 0u64, 2u64, 0u64, 1u64, 1u64))
        } else {
            let a1851 = {
                let a1849 = v6;
                let a1850 = 1u64;
                nat_add(a1849, a1850)?
            };
            let a1852 = v4;
            if nat_eq(a1851, a1852) {
                Ok(Adt9::C4(19u64, 1u64, 1u64, 2u64, 0u64, 3u64, f116(v5, v2)?))
            } else {
                let v7: u64 = {
                    let a1853 = v6;
                    let a1854 = 2u64;
                    nat_sub(a1853, a1854)
                };
                let v8: u64 = {
                    let a1855 = v7;
                    let a1856 = 3u64;
                    let a1857 = 0u64;
                    nat_quot(a1855, a1856, a1857)
                };
                let v9: u64 = {
                    let a1858 = v7;
                    let a1859 = 3u64;
                    let a1860 = 0u64;
                    nat_rem(a1858, a1859, a1860)
                };
                let v10: u64 = f116({
                    let a1864 = {
                        let a1862 = v5;
                        let a1863 = match v1.clone() {
                            Adt3::C0(h1861, _, _, _, _, _, _, _) => {
                                h1861
                            }
                        };
                        nat_mul(a1862, a1863)?
                    };
                    let a1865 = v8;
                    nat_add(a1864, a1865)?
                }, v1)?;
                let a1866 = v9;
                let a1867 = 0u64;
                if nat_eq(a1866, a1867) {
                    Ok(Adt9::C4(16u64, 1u64, 0u64, 0u64, v10, 1u64, 2u64))
                } else {
                    let a1868 = v9;
                    let a1869 = 1u64;
                    if nat_eq(a1868, a1869) {
                        let a1870 = v3;
                        let a1871 = 2u64;
                        if nat_eq(a1870, a1871) {
                            Ok(Adt9::C4(19u64, 1u64, 0u64, 2u64, 0u64, 1u64, 0u64))
                        } else {
                            Ok(Adt9::C4(19u64, 0u64, v10, 2u64, 0u64, 1u64, 0u64))
                        }
                    } else {
                        let a1872 = v3;
                        let a1873 = 2u64;
                        if if nat_eq(a1872, a1873) {
                            true
                        } else {
                            let a1874 = v3;
                            let a1875 = 3u64;
                            nat_eq(a1874, a1875)
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

pub fn f67(v0: u64, v1: Adt3, v2: Adt3, v3: Adt3, v4: u64) -> R<Adt9> {
    let v5: u64 = {
        let a1877 = 2u64;
        let a1878 = match v1.clone() {
            Adt3::C0(_, h1876, _, _, _, _, _, _) => {
                h1876
            }
        };
        nat_mul(a1877, a1878)?
    };
    let a1879 = v0;
    let a1880 = v5;
    if nat_lt(a1879, a1880) {
        let a1884 = {
            let a1881 = v0;
            let a1882 = 2u64;
            let a1883 = 0u64;
            nat_rem(a1881, a1882, a1883)
        };
        let a1885 = 0u64;
        if nat_eq(a1884, a1885) {
            Ok(Adt9::C4(19u64, 4u64, f116({
                let a1886 = v0;
                let a1887 = 2u64;
                let a1888 = 0u64;
                nat_quot(a1886, a1887, a1888)
            }, v3)?, 2u64, 0u64, 1u64, 2u64))
        } else {
            let a1889 = v4;
            let a1890 = 2147483648u64;
            if if nat_lt(a1889, a1890) {
                let a1892 = v4;
                let a1893 = match v2 {
                    Adt3::C0(_, h1891, _, _, _, _, _, _) => {
                        h1891
                    }
                };
                nat_lt(a1892, a1893)
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
            let a1896 = match v2.clone() {
                Adt3::C0(h1894, _, _, _, _, _, _, _) => {
                    h1894
                }
            };
            let a1897 = match v2.clone() {
                Adt3::C0(_, h1895, _, _, _, _, _, _) => {
                    h1895
                }
            };
            nat_mul(a1896, a1897)?
        };
        let a1900 = {
            let a1898 = v0;
            let a1899 = v5;
            nat_sub(a1898, a1899)
        };
        let a1901 = v6;
        if nat_lt(a1900, a1901) {
            Ok(Adt9::C4(19u64, 2u64, 0u64, 2u64, 0u64, 3u64, f116({
                let a1902 = v0;
                let a1903 = v5;
                nat_sub(a1902, a1903)
            }, v2)?))
        } else {
            let v7: u64 = {
                let a1906 = {
                    let a1904 = v0;
                    let a1905 = v5;
                    nat_sub(a1904, a1905)
                };
                let a1907 = v6;
                nat_sub(a1906, a1907)
            };
            let v8: u64 = {
                let a1911 = 1u64;
                let a1912 = {
                    let a1909 = 2u64;
                    let a1910 = match v1.clone() {
                        Adt3::C0(h1908, _, _, _, _, _, _, _) => {
                            h1908
                        }
                    };
                    nat_mul(a1909, a1910)?
                };
                nat_add(a1911, a1912)?
            };
            let v9: u64 = {
                let a1913 = v7;
                let a1914 = v8;
                let a1915 = 0u64;
                nat_quot(a1913, a1914, a1915)
            };
            let v10: u64 = {
                let a1916 = v7;
                let a1917 = v8;
                let a1918 = 0u64;
                nat_rem(a1916, a1917, a1918)
            };
            let a1919 = v10;
            let a1920 = 0u64;
            if nat_eq(a1919, a1920) {
                Ok(Adt9::C4(19u64, 4u64, f116(v9, v3)?, 2u64, 0u64, 1u64, 2u64))
            } else {
                let a1924 = {
                    let a1921 = v10;
                    let a1922 = 2u64;
                    let a1923 = 0u64;
                    nat_rem(a1921, a1922, a1923)
                };
                let a1925 = 1u64;
                if nat_eq(a1924, a1925) {
                    Ok(Adt9::C4(6u64, 0u64, f116({
                        let a1934 = {
                            let a1927 = v9;
                            let a1928 = match v1.clone() {
                                Adt3::C0(h1926, _, _, _, _, _, _, _) => {
                                    h1926
                                }
                            };
                            nat_mul(a1927, a1928)?
                        };
                        let a1935 = {
                            let a1931 = {
                                let a1929 = v10;
                                let a1930 = 1u64;
                                nat_sub(a1929, a1930)
                            };
                            let a1932 = 2u64;
                            let a1933 = 0u64;
                            nat_quot(a1931, a1932, a1933)
                        };
                        nat_add(a1934, a1935)?
                    }, v1)?, 2u64, 0u64, 1u64, 0u64))
                } else {
                    Ok(Adt9::C4(4u64, 3u64, f116({
                        let a1944 = {
                            let a1937 = v4;
                            let a1938 = match v1.clone() {
                                Adt3::C0(h1936, _, _, _, _, _, _, _) => {
                                    h1936
                                }
                            };
                            nat_mul(a1937, a1938)?
                        };
                        let a1945 = {
                            let a1941 = {
                                let a1939 = v10;
                                let a1940 = 1u64;
                                nat_sub(a1939, a1940)
                            };
                            let a1942 = 2u64;
                            let a1943 = 0u64;
                            nat_quot(a1941, a1942, a1943)
                        };
                        nat_add(a1944, a1945)?
                    }, v2.clone())?, 1u64, 0u64, 3u64, f116({
                        let a1954 = {
                            let a1947 = v4;
                            let a1948 = match v1 {
                                Adt3::C0(h1946, _, _, _, _, _, _, _) => {
                                    h1946
                                }
                            };
                            nat_mul(a1947, a1948)?
                        };
                        let a1955 = {
                            let a1951 = {
                                let a1949 = v10;
                                let a1950 = 1u64;
                                nat_sub(a1949, a1950)
                            };
                            let a1952 = 2u64;
                            let a1953 = 0u64;
                            nat_quot(a1951, a1952, a1953)
                        };
                        nat_add(a1954, a1955)?
                    }, v2)?))
                }
            }
        }
    }
}

pub fn f68(v0: u64, v1: u64, v2: Adt3, v3: Adt3, v4: Adt3, v5: u64) -> R<Adt9> {
    let a1956 = v1;
    let a1957 = f131(v2.clone(), v4.clone())?;
    if nat_lt(a1956, a1957) {
        let a1960 = v1;
        let a1961 = {
            let a1958 = 2u64;
            let a1959 = f105(v4.clone())?;
            nat_mul(a1958, a1959)?
        };
        if nat_lt(a1960, a1961) {
            let a1965 = {
                let a1962 = v1;
                let a1963 = 2u64;
                let a1964 = 0u64;
                nat_rem(a1962, a1963, a1964)
            };
            let a1966 = 0u64;
            if nat_eq(a1965, a1966) {
                f132({
                    let a1967 = v1;
                    let a1968 = 2u64;
                    let a1969 = 0u64;
                    nat_quot(a1967, a1968, a1969)
                }, v4)
            } else {
                if f133(match v2 {
                    Adt3::C0(_, h1970, _, _, _, _, _, _) => {
                        h1970
                    }
                }, v5) {
                    Ok(Adt9::C4(19u64, 1u64, 2u64, 2u64, 0u64, 1u64, 2u64))
                } else {
                    Ok(Adt9::C0(Adt10::C1))
                }
            }
        } else {
            let v6: u64 = {
                let a1973 = v1;
                let a1974 = {
                    let a1971 = 2u64;
                    let a1972 = f105(v4.clone())?;
                    nat_mul(a1971, a1972)?
                };
                nat_sub(a1973, a1974)
            };
            let v7: u64 = {
                let a1978 = v6;
                let a1979 = {
                    let a1976 = 1u64;
                    let a1977 = match v2.clone() {
                        Adt3::C0(h1975, _, _, _, _, _, _, _) => {
                            h1975
                        }
                    };
                    nat_add(a1976, a1977)?
                };
                let a1980 = 0u64;
                nat_quot(a1978, a1979, a1980)
            };
            let v8: u64 = {
                let a1984 = v6;
                let a1985 = {
                    let a1982 = 1u64;
                    let a1983 = match v2.clone() {
                        Adt3::C0(h1981, _, _, _, _, _, _, _) => {
                            h1981
                        }
                    };
                    nat_add(a1982, a1983)?
                };
                let a1986 = 0u64;
                nat_rem(a1984, a1985, a1986)
            };
            let a1987 = v8;
            let a1988 = 0u64;
            if nat_eq(a1987, a1988) {
                f132(v7, v4)
            } else {
                f134(v0, v7, {
                    let a1989 = v8;
                    let a1990 = 1u64;
                    nat_sub(a1989, a1990)
                }, v2, v3, v4, v5)
            }
        }
    } else {
        Ok(Adt9::C0(Adt10::C1))
    }
}

pub fn f69(v0: u64, v1: Adt3, v2: Adt3, v3: Adt3, v4: Adt3, v5: u64) -> R<Adt9> {
    let v6: u64 = {
        let a1993 = match v4.clone() {
            Adt3::C0(h1991, _, _, _, _, _, _, _) => {
                h1991
            }
        };
        let a1994 = match v4.clone() {
            Adt3::C0(_, h1992, _, _, _, _, _, _) => {
                h1992
            }
        };
        nat_mul(a1993, a1994)?
    };
    let v7: u64 = {
        let a1995 = 2u64;
        let a1996 = v6;
        nat_mul(a1995, a1996)?
    };
    let v8: u64 = f135(match v3.clone() {
        Adt3::C0(_, h1997, _, _, _, _, _, _) => {
            h1997
        }
    }, match v3.clone() {
        Adt3::C0(_, _, h1998, _, _, _, _, _) => {
            h1998
        }
    }, match v3.clone() {
        Adt3::C0(_, _, _, h1999, _, _, _, _) => {
            h1999
        }
    });
    let a2000 = v0;
    let a2001 = v7;
    if nat_lt(a2000, a2001) {
        let a2005 = {
            let a2002 = v0;
            let a2003 = 2u64;
            let a2004 = 0u64;
            nat_rem(a2002, a2003, a2004)
        };
        let a2006 = 0u64;
        if nat_eq(a2005, a2006) {
            Ok(Adt9::C4(19u64, 5u64, f116({
                let a2007 = v0;
                let a2008 = 2u64;
                let a2009 = 0u64;
                nat_quot(a2007, a2008, a2009)
            }, v4)?, 2u64, 0u64, 1u64, 2u64))
        } else {
            if f136(v5, v8) {
                Ok(Adt9::C4(19u64, 1u64, 2u64, 2u64, 0u64, 1u64, 2u64))
            } else {
                Ok(Adt9::C0(Adt10::C1))
            }
        }
    } else {
        let v9: u64 = {
            let a2010 = v0;
            let a2011 = v7;
            nat_sub(a2010, a2011)
        };
        let v10: u64 = {
            let a2015 = v9;
            let a2016 = {
                let a2013 = 1u64;
                let a2014 = match v1.clone() {
                    Adt3::C0(h2012, _, _, _, _, _, _, _) => {
                        h2012
                    }
                };
                nat_add(a2013, a2014)?
            };
            let a2017 = 0u64;
            nat_quot(a2015, a2016, a2017)
        };
        let v11: u64 = {
            let a2021 = v9;
            let a2022 = {
                let a2019 = 1u64;
                let a2020 = match v1.clone() {
                    Adt3::C0(h2018, _, _, _, _, _, _, _) => {
                        h2018
                    }
                };
                nat_add(a2019, a2020)?
            };
            let a2023 = 0u64;
            nat_rem(a2021, a2022, a2023)
        };
        let a2024 = v11;
        let a2025 = 0u64;
        if nat_eq(a2024, a2025) {
            Ok(Adt9::C4(19u64, 5u64, f116({
                let a2026 = v10;
                let a2027 = v6;
                let a2028 = 0u64;
                nat_rem(a2026, a2027, a2028)
            }, v4)?, 2u64, 0u64, 1u64, 2u64))
        } else {
            if f136(v5, v8) {
                let v12: u64 = {
                    let a2034 = {
                        let a2030 = v10;
                        let a2031 = match v1.clone() {
                            Adt3::C0(h2029, _, _, _, _, _, _, _) => {
                                h2029
                            }
                        };
                        nat_mul(a2030, a2031)?
                    };
                    let a2035 = {
                        let a2032 = v11;
                        let a2033 = 1u64;
                        nat_sub(a2032, a2033)
                    };
                    nat_add(a2034, a2035)?
                };
                let a2036 = v5;
                let a2037 = 4294967295u64;
                if nat_eq(a2036, a2037) {
                    Ok(Adt9::C1(false, f116(v12, v1)?, f116(v12, v2)?))
                } else {
                    Ok(Adt9::C4(8u64, 0u64, f116(v12, v1)?, 4u64, f116({
                        let a2043 = {
                            let a2039 = v5;
                            let a2040 = match v3.clone() {
                                Adt3::C0(h2038, _, _, _, _, _, _, _) => {
                                    h2038
                                }
                            };
                            nat_mul(a2039, a2040)?
                        };
                        let a2044 = {
                            let a2041 = v11;
                            let a2042 = 1u64;
                            nat_sub(a2041, a2042)
                        };
                        nat_add(a2043, a2044)?
                    }, v3)?, 3u64, f116(v12, v2)?))
                }
            } else {
                Ok(Adt9::C0(Adt10::C1))
            }
        }
    }
}

pub fn f70(v0: u64, v1: Adt3, v2: Adt3, v3: Adt7, v4: u64) -> R<Adt9> {
    let v5: u64 = f89(v3.clone())?;
    let v6: u64 = {
        let a2045 = v0;
        let a2046 = v5;
        let a2047 = 0u64;
        nat_quot(a2045, a2046, a2047)
    };
    let v7: u64 = {
        let a2048 = v0;
        let a2049 = v5;
        let a2050 = 0u64;
        nat_rem(a2048, a2049, a2050)
    };
    let v8: u64 = f137(v3.clone())?;
    let a2051 = v7;
    let a2052 = 0u64;
    if nat_eq(a2051, a2052) {
        Ok(Adt9::C4(19u64, 2u64, if match v3 {
            Adt7::C0(_, _, _, h2053, _) => {
                h2053
            }
        } {
            4286578687u64
        } else {
            0u64
        }, 2u64, 0u64, 1u64, 0u64))
    } else {
        let a2057 = v5;
        let a2058 = {
            let a2055 = v7;
            let a2056 = if match v3.clone() {
                Adt7::C0(_, _, _, h2054, _) => {
                    h2054
                }
            } {
                1u64
            } else {
                2u64
            };
            nat_add(a2055, a2056)?
        };
        if nat_le(a2057, a2058) {
            if match v3.clone() {
                Adt7::C0(_, _, _, h2059, _) => {
                    h2059
                }
            } {
                Ok(Adt9::C4(19u64, 1u64, 0u64, 2u64, 0u64, 3u64, f116(v6, v2)?))
            } else {
                let a2062 = {
                    let a2060 = v7;
                    let a2061 = 2u64;
                    nat_add(a2060, a2061)?
                };
                let a2063 = v5;
                if nat_eq(a2062, a2063) {
                    let v9: u64 = if match v3.clone() {
                        Adt7::C0(_, _, h2064, _, _) => {
                            h2064
                        }
                    } {
                        let a2071 = {
                            let m2067 = match v3.clone() {
                                Adt7::C0(h2066, _, _, _, _) => {
                                    h2066
                                }
                            };
                            match m2067 {
                                Adt6::C0(h2065, _, _, _) => {
                                    h2065
                                }
                            }
                        };
                        let a2072 = {
                            let m2070 = match v3 {
                                Adt7::C0(_, h2069, _, _, _) => {
                                    h2069
                                }
                            };
                            match m2070 {
                                Adt6::C0(h2068, _, _, _) => {
                                    h2068
                                }
                            }
                        };
                        nat_mul(a2071, a2072)?
                    } else {
                        f138(match v3 {
                            Adt7::C0(h2073, _, _, _, _) => {
                                h2073
                            }
                        }, match v1 {
                            Adt3::C0(h2074, _, _, _, _, _, _, _) => {
                                h2074
                            }
                        }, f60(v6, v2, 0u64))?
                    };
                    Ok(Adt9::C4(3u64, 2u64, {
                        let a2075 = v9;
                        let a2076 = 0u64;
                        if nat_eq(a2075, a2076) {
                            1u64
                        } else {
                            v9
                        }
                    }, 2u64, 0u64, 1u64, 1u64))
                } else {
                    Ok(Adt9::C4(5u64, 1u64, 0u64, 1u64, 1u64, 3u64, f116(v6, v2)?))
                }
            }
        } else {
            let v10: u64 = {
                let a2079 = {
                    let a2077 = v7;
                    let a2078 = 1u64;
                    nat_sub(a2077, a2078)
                };
                let a2080 = v8;
                let a2081 = 0u64;
                nat_rem(a2079, a2080, a2081)
            };
            let m2087 = f139(v3.clone(), v1, v2, v6, {
                let a2084 = {
                    let a2082 = v7;
                    let a2083 = 1u64;
                    nat_sub(a2082, a2083)
                };
                let a2085 = v8;
                let a2086 = 0u64;
                nat_quot(a2084, a2085, a2086)
            })?;
            match m2087 {
                None => {
                    Ok(Adt9::C4(19u64, 1u64, 0u64, 2u64, 0u64, 1u64, 0u64))
                }
                Some(v11) => {
                    let a2089 = match v3.clone() {
                        Adt7::C0(_, _, _, _, h2088) => {
                            h2088
                        }
                    };
                    let a2090 = 1u64;
                    if if nat_eq(a2089, a2090) {
                        let a2091 = v10;
                        let a2092 = 0u64;
                        nat_eq(a2091, a2092)
                    } else {
                        false
                    } {
                        Ok(Adt9::C4(6u64, 0u64, v11, 2u64, 0u64, 1u64, 1u64))
                    } else {
                        if match v3.clone() {
                            Adt7::C0(_, _, _, h2093, _) => {
                                h2093
                            }
                        } {
                            let a2096 = {
                                let a2094 = v10;
                                let a2095 = 1u64;
                                nat_add(a2094, a2095)?
                            };
                            let a2097 = v8;
                            if nat_eq(a2096, a2097) {
                                let a2098 = v4;
                                let a2099 = 2u64;
                                if nat_eq(a2098, a2099) {
                                    Ok(Adt9::C4(19u64, 1u64, 0u64, 2u64, 0u64, 1u64, 0u64))
                                } else {
                                    Ok(Adt9::C4(19u64, {
                                        let a2101 = match v3.clone() {
                                            Adt7::C0(_, _, _, _, h2100) => {
                                                h2100
                                            }
                                        };
                                        let a2102 = 1u64;
                                        if nat_eq(a2101, a2102) {
                                            1u64
                                        } else {
                                            0u64
                                        }
                                    }, {
                                        let a2104 = match v3 {
                                            Adt7::C0(_, _, _, _, h2103) => {
                                                h2103
                                            }
                                        };
                                        let a2105 = 1u64;
                                        if nat_eq(a2104, a2105) {
                                            1u64
                                        } else {
                                            v11
                                        }
                                    }, 2u64, 0u64, 1u64, 0u64))
                                }
                            } else {
                                Ok(Adt9::C4(16u64, 1u64, 0u64, {
                                    let a2107 = match v3.clone() {
                                        Adt7::C0(_, _, _, _, h2106) => {
                                            h2106
                                        }
                                    };
                                    let a2108 = 1u64;
                                    if nat_eq(a2107, a2108) {
                                        1u64
                                    } else {
                                        0u64
                                    }
                                }, {
                                    let a2110 = match v3 {
                                        Adt7::C0(_, _, _, _, h2109) => {
                                            h2109
                                        }
                                    };
                                    let a2111 = 1u64;
                                    if nat_eq(a2110, a2111) {
                                        1u64
                                    } else {
                                        v11
                                    }
                                }, 1u64, 2u64))
                            }
                        } else {
                            Ok(Adt9::C4(4u64, 1u64, 0u64, {
                                let a2113 = match v3.clone() {
                                    Adt7::C0(_, _, _, _, h2112) => {
                                        h2112
                                    }
                                };
                                let a2114 = 1u64;
                                if nat_eq(a2113, a2114) {
                                    1u64
                                } else {
                                    0u64
                                }
                            }, {
                                let a2116 = match v3 {
                                    Adt7::C0(_, _, _, _, h2115) => {
                                        h2115
                                    }
                                };
                                let a2117 = 1u64;
                                if nat_eq(a2116, a2117) {
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

pub fn f71(v0: u64, v1: Adt3, v2: Adt3, v3: Adt3) -> R<Adt9> {
    let v4: u64 = f87(match v3.clone() {
        Adt3::C0(h2118, _, _, _, _, _, _, _) => {
            h2118
        }
    })?;
    let v5: u64 = {
        let a2119 = v0;
        let a2120 = v4;
        let a2121 = 0u64;
        nat_rem(a2119, a2120, a2121)
    };
    let v6: u64 = {
        let a2122 = v0;
        let a2123 = v4;
        let a2124 = 0u64;
        nat_quot(a2122, a2123, a2124)
    };
    let a2125 = v5;
    let a2126 = 0u64;
    if nat_eq(a2125, a2126) {
        Ok(Adt9::C4(19u64, 2u64, 0u64, 2u64, 0u64, 1u64, 0u64))
    } else {
        let a2129 = {
            let a2127 = v5;
            let a2128 = 1u64;
            nat_add(a2127, a2128)?
        };
        let a2130 = v4;
        if nat_eq(a2129, a2130) {
            Ok(Adt9::C4(19u64, 1u64, 0u64, 2u64, 0u64, 3u64, f116(v6, v2)?))
        } else {
            let a2134 = {
                let a2131 = v5;
                let a2132 = 2u64;
                let a2133 = 0u64;
                nat_rem(a2131, a2132, a2133)
            };
            let a2135 = 0u64;
            if nat_eq(a2134, a2135) {
                Ok(Adt9::C4(4u64, 1u64, 0u64, 1u64, 1u64, 1u64, 0u64))
            } else {
                let v7: u64 = {
                    let a2138 = {
                        let a2136 = v5;
                        let a2137 = 1u64;
                        nat_sub(a2136, a2137)
                    };
                    let a2139 = 2u64;
                    let a2140 = 0u64;
                    nat_quot(a2138, a2139, a2140)
                };
                let v8: u64 = f60(v6, v2.clone(), 0u64);
                let v9: u64 = f60(v6, v2.clone(), 1u64);
                let v10: u64 = f60(v6, v2, 2u64);
                Ok(Adt9::C4(10u64, 0u64, f59(v1, {
                    let a2141 = v9;
                    let a2142 = v7;
                    nat_add(a2141, a2142)?
                }, v8, v10, 0u64)?, 4u64, f59(v3, v7, v8, 0u64, 0u64)?, 1u64, 1u64))
            }
        }
    }
}

pub fn f72(v0: u64, v1: Adt3, v2: Adt3, v3: Adt5) -> R<Adt9> {
    let v4: u64 = f60(v0, v2.clone(), 0u64);
    let v5: u64 = f60(v0, v2.clone(), 3u64);
    let v6: u64 = {
        let a2144 = v5;
        let a2145 = match v3.clone() {
            Adt5::C0(_, _, _, _, _, _, h2143, _) => {
                h2143
            }
        };
        let a2146 = 0u64;
        nat_rem(a2144, a2145, a2146)
    };
    let v7: u64 = {
        let a2148 = v5;
        let a2149 = match v3.clone() {
            Adt5::C0(_, _, _, _, _, _, h2147, _) => {
                h2147
            }
        };
        let a2150 = 0u64;
        nat_quot(a2148, a2149, a2150)
    };
    let v8: Option<u64> = f62(match v3.clone() {
        Adt5::C0(h2151, _, _, _, _, _, _, _) => {
            h2151
        }
    }, match v1.clone() {
        Adt3::C0(h2152, _, _, _, _, _, _, _) => {
            h2152
        }
    }, f60(v0, v2.clone(), 1u64), {
        let a2156 = v4;
        let a2157 = {
            let m2155 = match v3.clone() {
                Adt5::C0(h2154, _, _, _, _, _, _, _) => {
                    h2154
                }
            };
            match m2155 {
                Adt6::C0(h2153, _, _, _) => {
                    h2153
                }
            }
        };
        let a2158 = 0u64;
        nat_rem(a2156, a2157, a2158)
    })?;
    let v9: u64 = {
        let a2162 = v4;
        let a2163 = {
            let m2161 = match v3.clone() {
                Adt5::C0(h2160, _, _, _, _, _, _, _) => {
                    h2160
                }
            };
            match m2161 {
                Adt6::C0(h2159, _, _, _) => {
                    h2159
                }
            }
        };
        let a2164 = 0u64;
        nat_quot(a2162, a2163, a2164)
    };
    let v10: Option<u64> = f62(match v3.clone() {
        Adt5::C0(_, h2165, _, _, _, _, _, _) => {
            h2165
        }
    }, match v1.clone() {
        Adt3::C0(_, h2166, _, _, _, _, _, _) => {
            h2166
        }
    }, f60(v0, v2.clone(), 2u64), {
        let a2170 = v9;
        let a2171 = {
            let m2169 = match v3.clone() {
                Adt5::C0(_, h2168, _, _, _, _, _, _) => {
                    h2168
                }
            };
            match m2169 {
                Adt6::C0(h2167, _, _, _) => {
                    h2167
                }
            }
        };
        let a2172 = 0u64;
        nat_rem(a2170, a2171, a2172)
    })?;
    let v11: u64 = {
        let a2176 = v9;
        let a2177 = {
            let m2175 = match v3.clone() {
                Adt5::C0(_, h2174, _, _, _, _, _, _) => {
                    h2174
                }
            };
            match m2175 {
                Adt6::C0(h2173, _, _, _) => {
                    h2173
                }
            }
        };
        let a2178 = 0u64;
        nat_quot(a2176, a2177, a2178)
    };
    let v12: Option<u64> = f62(match v3.clone() {
        Adt5::C0(_, _, h2179, _, _, _, _, _) => {
            h2179
        }
    }, match v1.clone() {
        Adt3::C0(_, _, h2180, _, _, _, _, _) => {
            h2180
        }
    }, v6, {
        let a2184 = v11;
        let a2185 = {
            let m2183 = match v3.clone() {
                Adt5::C0(_, _, h2182, _, _, _, _, _) => {
                    h2182
                }
            };
            match m2183 {
                Adt6::C0(h2181, _, _, _) => {
                    h2181
                }
            }
        };
        let a2186 = 0u64;
        nat_rem(a2184, a2185, a2186)
    })?;
    let v13: u64 = {
        let a2190 = v11;
        let a2191 = {
            let m2189 = match v3.clone() {
                Adt5::C0(_, _, h2188, _, _, _, _, _) => {
                    h2188
                }
            };
            match m2189 {
                Adt6::C0(h2187, _, _, _) => {
                    h2187
                }
            }
        };
        let a2192 = 0u64;
        nat_quot(a2190, a2191, a2192)
    };
    let v17: Adt12 = {
        let m2193 = v8;
        match m2193 {
            None => {
                Adt12::C2(0u64)
            }
            Some(v14) => {
                let m2194 = v10;
                match m2194 {
                    None => {
                        Adt12::C2(0u64)
                    }
                    Some(v15) => {
                        let m2195 = v12;
                        match m2195 {
                            None => {
                                Adt12::C2(0u64)
                            }
                            Some(v16) => {
                                Adt12::C0(1u64, {
                                    let a2204 = {
                                        let a2199 = f59(v1, v14, v15, v16, 0u64)?;
                                        let a2200 = {
                                            let a2197 = v7;
                                            let a2198 = match v3.clone() {
                                                Adt5::C0(_, _, _, _, h2196, _, _, _) => {
                                                    h2196
                                                }
                                            };
                                            nat_mul(a2197, a2198)?
                                        };
                                        nat_add(a2199, a2200)?
                                    };
                                    let a2205 = {
                                        let a2202 = v13;
                                        let a2203 = match v3.clone() {
                                            Adt5::C0(_, _, _, h2201, _, _, _, _) => {
                                                h2201
                                            }
                                        };
                                        nat_mul(a2202, a2203)?
                                    };
                                    nat_add(a2204, a2205)?
                                })
                            }
                        }
                    }
                }
            }
        }
    };
    Ok(Adt9::C7(Adt11::C2(match v3.clone() {
        Adt5::C0(_, _, _, _, _, h2206, _, _) => {
            h2206
        }
    }, match v3.clone() {
        Adt5::C0(_, _, _, _, _, _, _, h2207) => {
            h2207
        }
    }, true), v17, Adt12::C2(0u64), Adt13::C1(f116(v0, v2)?), {
        let a2209 = match v3.clone() {
            Adt5::C0(_, _, _, _, _, h2208, _, _) => {
                h2208
            }
        };
        let a2210 = 0u64;
        if nat_eq(a2209, a2210) {
            4u64
        } else {
            2u64
        }
    }, {
        let a2212 = match v3 {
            Adt5::C0(_, _, _, _, _, _, _, h2211) => {
                h2211
            }
        };
        let a2213 = 0u64;
        if nat_eq(a2212, a2213) {
            4u64
        } else {
            2u64
        }
    }))
}

pub fn f73(v0: u64, v1: Adt3, v2: Adt3, v3: u64, v4: u64, v5: u64, v6: u64, v7: u64) -> R<Adt9> {
    let v8: u64 = {
        let a2216 = v6;
        let a2217 = {
            let a2214 = 3u64;
            let a2215 = v5;
            nat_add(a2214, a2215)?
        };
        nat_sub(a2216, a2217)
    };
    let a2219 = v0;
    let a2220 = f29(v3, match v1.clone() {
        Adt3::C0(h2218, _, _, _, _, _, _, _) => {
            h2218
        }
    }, v5, v8)?;
    if nat_lt(a2219, a2220) {
        let v9: u64 = f83(match v1.clone() {
            Adt3::C0(h2221, _, _, _, _, _, _, _) => {
                h2221
            }
        }, v5, v8)?;
        let v10: u64 = f84(v3, match v1.clone() {
            Adt3::C0(h2222, _, _, _, _, _, _, _) => {
                h2222
            }
        }, v5)?;
        let a2223 = v0;
        let a2224 = v9;
        if nat_lt(a2223, a2224) {
            f140(v0, v2, f23(v3), {
                let a2225 = 3u64;
                let a2226 = v5;
                nat_add(a2225, a2226)?
            }, v7)
        } else {
            let a2229 = v0;
            let a2230 = {
                let a2227 = v9;
                let a2228 = v10;
                nat_add(a2227, a2228)?
            };
            if nat_lt(a2229, a2230) {
                let v11: u64 = {
                    let a2231 = v0;
                    let a2232 = v9;
                    nat_sub(a2231, a2232)
                };
                let v12: u64 = f141(v3, match v1.clone() {
                    Adt3::C0(h2233, _, _, _, _, _, _, _) => {
                        h2233
                    }
                })?;
                f142({
                    let a2234 = v11;
                    let a2235 = v12;
                    let a2236 = 0u64;
                    nat_rem(a2234, a2235, a2236)
                }, {
                    let a2237 = v11;
                    let a2238 = v12;
                    let a2239 = 0u64;
                    nat_quot(a2237, a2238, a2239)
                }, v1, v3, v4, v7)
            } else {
                f143({
                    let a2242 = v0;
                    let a2243 = {
                        let a2240 = v9;
                        let a2241 = v10;
                        nat_add(a2240, a2241)?
                    };
                    nat_sub(a2242, a2243)
                }, v1, v2, v3)
            }
        }
    } else {
        Ok(Adt9::C0(Adt10::C1))
    }
}

pub fn f74(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64) -> R<Adt9> {
    let a2244 = v0;
    let a2245 = f15(v1)?;
    if nat_lt(a2244, a2245) {
        let a2246 = v0;
        let a2247 = 4u64;
        if nat_lt(a2246, a2247) {
            let a2248 = v0;
            let a2249 = 0u64;
            if nat_eq(a2248, a2249) {
                Ok(Adt9::C4(16u64, 2u64, v2, 2u64, v3, 1u64, 2u64))
            } else {
                let a2250 = v0;
                let a2251 = 1u64;
                if nat_eq(a2250, a2251) {
                    let a2252 = v5;
                    let a2253 = 0u64;
                    if nat_eq(a2252, a2253) {
                        Ok(Adt9::C4(9u64, 2u64, v3, 2u64, v2, 1u64, 2u64))
                    } else {
                        Ok(Adt9::C0(Adt10::C1))
                    }
                } else {
                    let a2254 = v0;
                    let a2255 = 2u64;
                    if nat_eq(a2254, a2255) {
                        Ok(Adt9::C4(11u64, 1u64, 2u64, 2u64, v4, 1u64, 2u64))
                    } else {
                        Ok(Adt9::C4(20u64, 1u64, 2u64, 2u64, 10u64, 1u64, 2u64))
                    }
                }
            }
        } else {
            let a2256 = v0;
            let a2257 = 4u64;
            if if nat_eq(a2256, a2257) {
                let a2258 = f144(v5)?;
                let a2259 = v1;
                !nat_eq(a2258, a2259)
            } else {
                false
            } {
                Ok(Adt9::C0(Adt10::C1))
            } else {
                let v6: u64 = {
                    let a2262 = {
                        let a2260 = v0;
                        let a2261 = 4u64;
                        nat_sub(a2260, a2261)
                    };
                    let a2263 = 3u64;
                    let a2264 = 0u64;
                    nat_quot(a2262, a2263, a2264)
                };
                let v7: u64 = {
                    let a2267 = {
                        let a2265 = v0;
                        let a2266 = 4u64;
                        nat_sub(a2265, a2266)
                    };
                    let a2268 = 3u64;
                    let a2269 = 0u64;
                    nat_rem(a2267, a2268, a2269)
                };
                let a2270 = v7;
                let a2271 = 0u64;
                if nat_eq(a2270, a2271) {
                    Ok(Adt9::C4(3u64, 2u64, v6, 2u64, 0u64, 1u64, 0u64))
                } else {
                    let a2272 = v7;
                    let a2273 = 1u64;
                    if nat_eq(a2272, a2273) {
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

pub fn f75(v0: bool, v1: u64, v2: u64, v3: u64, v4: u64) -> Adt11 {
    let a2274 = v1;
    let a2275 = 0u64;
    if nat_eq(a2274, a2275) {
        if v0 {
            Adt11::C2(v3, v4, false)
        } else {
            Adt11::C0(v2)
        }
    } else {
        let a2276 = v1;
        let a2277 = 1u64;
        if nat_eq(a2276, a2277) {
            Adt11::C1(v2)
        } else {
            let a2278 = v1;
            let a2279 = 2u64;
            if nat_eq(a2278, a2279) {
                Adt11::C5(v2)
            } else {
                let a2280 = v1;
                let a2281 = 3u64;
                if nat_eq(a2280, a2281) {
                    Adt11::C3(v2, v2, false)
                } else {
                    let a2282 = v1;
                    let a2283 = 4u64;
                    if nat_eq(a2282, a2283) {
                        Adt11::C2(v3, v4, false)
                    } else {
                        Adt11::C6
                    }
                }
            }
        }
    }
}

pub fn f76(v0: u64, v1: u64) -> Adt12 {
    let a2284 = v0;
    let a2285 = 0u64;
    if nat_eq(a2284, a2285) {
        Adt12::C0(0u64, v1)
    } else {
        let a2286 = v0;
        let a2287 = 1u64;
        if nat_eq(a2286, a2287) {
            Adt12::C1(v1)
        } else {
            let a2288 = v0;
            let a2289 = 2u64;
            if nat_eq(a2288, a2289) {
                Adt12::C2(v1)
            } else {
                let a2290 = v0;
                let a2291 = 3u64;
                if nat_eq(a2290, a2291) {
                    Adt12::C3(v1)
                } else {
                    let a2292 = v0;
                    let a2293 = 13u64;
                    if nat_lt(a2292, a2293) {
                        Adt12::C0({
                            let a2294 = v0;
                            let a2295 = 3u64;
                            nat_sub(a2294, a2295)
                        }, v1)
                    } else {
                        Adt12::C4
                    }
                }
            }
        }
    }
}

pub fn f77(v0: u64, v1: u64) -> Adt13 {
    let a2296 = v0;
    let a2297 = 1u64;
    if nat_eq(a2296, a2297) {
        Adt13::C0(v1)
    } else {
        let a2298 = v0;
        let a2299 = 3u64;
        if if nat_eq(a2298, a2299) {
            true
        } else {
            let a2300 = v0;
            let a2301 = 6u64;
            nat_eq(a2300, a2301)
        } {
            Adt13::C1(v1)
        } else {
            Adt13::C2
        }
    }
}

pub fn f78(v0: Adt6, v1: u64) -> R<u64> {
    let a2303 = 0u64;
    let a2304 = match v0.clone() {
        Adt6::C0(h2302, _, _, _) => {
            h2302
        }
    };
    if if if if if nat_lt(a2303, a2304) {
        let a2306 = 0u64;
        let a2307 = match v0.clone() {
            Adt6::C0(_, h2305, _, _) => {
                h2305
            }
        };
        nat_lt(a2306, a2307)
    } else {
        false
    } {
        let a2309 = 0u64;
        let a2310 = match v0.clone() {
            Adt6::C0(_, _, _, h2308) => {
                h2308
            }
        };
        nat_lt(a2309, a2310)
    } else {
        false
    } {
        let a2312 = match v0.clone() {
            Adt6::C0(_, h2311, _, _) => {
                h2311
            }
        };
        let a2313 = 2147483648u64;
        nat_lt(a2312, a2313)
    } else {
        false
    } {
        let a2315 = match v0.clone() {
            Adt6::C0(_, _, _, h2314) => {
                h2314
            }
        };
        let a2316 = 2147483648u64;
        nat_lt(a2315, a2316)
    } else {
        false
    } {
        let v2: u64 = {
            let a2320 = v1;
            let a2321 = {
                let a2318 = 2u64;
                let a2319 = f110(match v0.clone() {
                    Adt6::C0(_, _, h2317, _) => {
                        h2317
                    }
                });
                nat_mul(a2318, a2319)?
            };
            nat_add(a2320, a2321)?
        };
        let v3: u64 = {
            let a2333 = {
                let a2328 = 1u64;
                let a2329 = {
                    let a2326 = match v0.clone() {
                        Adt6::C0(_, _, _, h2322) => {
                            h2322
                        }
                    };
                    let a2327 = {
                        let a2324 = match v0.clone() {
                            Adt6::C0(h2323, _, _, _) => {
                                h2323
                            }
                        };
                        let a2325 = 1u64;
                        nat_sub(a2324, a2325)
                    };
                    nat_mul(a2326, a2327)?
                };
                nat_add(a2328, a2329)?
            };
            let a2334 = {
                let a2331 = 2u64;
                let a2332 = f109(match v0.clone() {
                    Adt6::C0(_, _, h2330, _) => {
                        h2330
                    }
                });
                nat_mul(a2331, a2332)?
            };
            nat_add(a2333, a2334)?
        };
        let a2335 = v3;
        let a2336 = v2;
        if nat_le(a2335, a2336) {
            let a2343 = 1u64;
            let a2344 = {
                let a2340 = {
                    let a2337 = v2;
                    let a2338 = v3;
                    nat_sub(a2337, a2338)
                };
                let a2341 = match v0 {
                    Adt6::C0(_, h2339, _, _) => {
                        h2339
                    }
                };
                let a2342 = 0u64;
                nat_quot(a2340, a2341, a2342)
            };
            nat_add(a2343, a2344)
        } else {
            let a2348 = {
                let a2345 = v3;
                let a2346 = v2;
                nat_sub(a2345, a2346)
            };
            let a2349 = match v0 {
                Adt6::C0(_, h2347, _, _) => {
                    h2347
                }
            };
            if nat_lt(a2348, a2349) {
                Ok(1u64)
            } else {
                Ok(0u64)
            }
        }
    } else {
        Ok(0u64)
    }
}

pub fn f79(v0: u64) -> u64 {
    let v1: u64 = {
        let a2350 = v0;
        let a2351 = 4294967296u64;
        let a2352 = 0u64;
        nat_rem(a2350, a2351, a2352)
    };
    let a2353 = v1;
    let a2354 = 2147483648u64;
    if nat_lt(a2353, a2354) {
        v1
    } else {
        0u64
    }
}

pub fn f80(v0: u64) -> R<u64> {
    let a2357 = 2u64;
    let a2358 = {
        let a2355 = 2u64;
        let a2356 = v0;
        nat_mul(a2355, a2356)?
    };
    nat_add(a2357, a2358)
}

pub fn f81(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64) -> bool {
    if if f145(v0, v3, v6) {
        f145(v1, v4, v6)
    } else {
        false
    } {
        f145(v2, v5, v6)
    } else {
        false
    }
}

pub fn f82(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64) -> R<bool> {
    if if if if if f146(v0, v1, v4, v2, v5, v3, v6)? {
        true
    } else {
        f146(v0, v1, v4, v3, v6, v2, v5)?
    } {
        true
    } else {
        f146(v0, v2, v5, v1, v4, v3, v6)?
    } {
        true
    } else {
        f146(v0, v2, v5, v3, v6, v1, v4)?
    } {
        true
    } else {
        f146(v0, v3, v6, v1, v4, v2, v5)?
    } {
        Ok(true)
    } else {
        f146(v0, v3, v6, v2, v5, v1, v4)
    }
}

pub fn f83(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let a2359 = v2;
    let a2360 = 0u64;
    if nat_eq(a2359, a2360) {
        Ok(0u64)
    } else {
        let a2363 = 2u64;
        let a2364 = {
            let a2361 = v0;
            let a2362 = v1;
            nat_mul(a2361, a2362)?
        };
        nat_mul(a2363, a2364)
    }
}

pub fn f84(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let a2365 = v2;
    let a2366 = f141(v0, v1)?;
    nat_mul(a2365, a2366)
}

pub fn f85(v0: u64) -> u64 {
    let a2367 = v0;
    let a2368 = 30u64;
    if nat_eq(a2367, a2368) {
        3u64
    } else {
        1u64
    }
}

pub fn f86(v0: u64) -> R<u64> {
    let a2371 = 5u64;
    let a2372 = {
        let a2369 = 4u64;
        let a2370 = v0;
        nat_mul(a2369, a2370)?
    };
    nat_add(a2371, a2372)
}

pub fn f87(v0: u64) -> R<u64> {
    let a2375 = 2u64;
    let a2376 = {
        let a2373 = v0;
        let a2374 = 2u64;
        nat_mul(a2373, a2374)?
    };
    nat_add(a2375, a2376)
}

pub fn f88(v0: Adt6, v1: u64, v2: u64) -> R<bool> {
    let v3: u64 = {
        let a2380 = {
            let a2377 = v2;
            let a2378 = 1u64;
            nat_sub(a2377, a2378)
        };
        let a2381 = match v0.clone() {
            Adt6::C0(_, h2379, _, _) => {
                h2379
            }
        };
        nat_mul(a2380, a2381)?
    };
    let a2383 = 0u64;
    let a2384 = match v0.clone() {
        Adt6::C0(h2382, _, _, _) => {
            h2382
        }
    };
    if if if if if if if if nat_lt(a2383, a2384) {
        let a2386 = match v0.clone() {
            Adt6::C0(h2385, _, _, _) => {
                h2385
            }
        };
        let a2387 = 2147483648u64;
        nat_lt(a2386, a2387)
    } else {
        false
    } {
        let a2389 = 0u64;
        let a2390 = match v0.clone() {
            Adt6::C0(_, h2388, _, _) => {
                h2388
            }
        };
        nat_lt(a2389, a2390)
    } else {
        false
    } {
        let a2392 = match v0.clone() {
            Adt6::C0(_, h2391, _, _) => {
                h2391
            }
        };
        let a2393 = 2147483648u64;
        nat_lt(a2392, a2393)
    } else {
        false
    } {
        let a2394 = v1;
        let a2395 = 2147483648u64;
        nat_lt(a2394, a2395)
    } else {
        false
    } {
        let a2396 = 0u64;
        let a2397 = v2;
        nat_lt(a2396, a2397)
    } else {
        false
    } {
        let a2398 = v3;
        let a2399 = 2147483648u64;
        nat_lt(a2398, a2399)
    } else {
        false
    } {
        let a2411 = {
            let a2406 = {
                let a2403 = v3;
                let a2404 = {
                    let a2401 = match v0.clone() {
                        Adt6::C0(h2400, _, _, _) => {
                            h2400
                        }
                    };
                    let a2402 = 1u64;
                    nat_sub(a2401, a2402)
                };
                nat_add(a2403, a2404)?
            };
            let a2407 = f109(match v0.clone() {
                Adt6::C0(_, _, h2405, _) => {
                    h2405
                }
            });
            nat_add(a2406, a2407)?
        };
        let a2412 = {
            let a2409 = 2147483647u64;
            let a2410 = f110(match v0.clone() {
                Adt6::C0(_, _, h2408, _) => {
                    h2408
                }
            });
            nat_add(a2409, a2410)?
        };
        nat_le(a2411, a2412)
    } else {
        false
    } {
        let a2414 = match v0 {
            Adt6::C0(_, _, h2413, _) => {
                h2413
            }
        };
        let a2415 = 2147483648u64;
        Ok(!nat_eq(a2414, a2415))
    } else {
        Ok(false)
    }
}

pub fn f89(v0: Adt7) -> R<u64> {
    let a2427 = if match v0.clone() {
        Adt7::C0(_, _, _, h2416, _) => {
            h2416
        }
    } {
        2u64
    } else {
        3u64
    };
    let a2428 = {
        let a2425 = {
            let a2423 = {
                let m2419 = match v0.clone() {
                    Adt7::C0(h2418, _, _, _, _) => {
                        h2418
                    }
                };
                match m2419 {
                    Adt6::C0(h2417, _, _, _) => {
                        h2417
                    }
                }
            };
            let a2424 = {
                let m2422 = match v0.clone() {
                    Adt7::C0(_, h2421, _, _, _) => {
                        h2421
                    }
                };
                match m2422 {
                    Adt6::C0(h2420, _, _, _) => {
                        h2420
                    }
                }
            };
            nat_mul(a2423, a2424)?
        };
        let a2426 = f137(v0)?;
        nat_mul(a2425, a2426)?
    };
    nat_add(a2427, a2428)
}

pub fn f90(v0: Adt2, v1: Adt2, v2: Adt2) -> R<Adt10> {
    if f147(match v0.clone() {
        Adt2::C0(h2429, _, _, _, _, _, _, _, _, _, _) => {
            h2429
        }
    }, match v0.clone() {
        Adt2::C0(_, h2430, _, _, _, _, _, _, _, _, _) => {
            h2430
        }
    }, match v0.clone() {
        Adt2::C0(_, _, h2431, _, _, _, _, _, _, _, _) => {
            h2431
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, h2432, _, _, _, _, _, _, _) => {
            h2432
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, h2433, _, _, _, _, _, _) => {
            h2433
        }
    }, match v1.clone() {
        Adt2::C0(h2434, _, _, _, _, _, _, _, _, _, _) => {
            h2434
        }
    }, match v1.clone() {
        Adt2::C0(_, h2435, _, _, _, _, _, _, _, _, _) => {
            h2435
        }
    }, match v1.clone() {
        Adt2::C0(_, _, h2436, _, _, _, _, _, _, _, _) => {
            h2436
        }
    }, match v1.clone() {
        Adt2::C0(_, _, _, h2437, _, _, _, _, _, _, _) => {
            h2437
        }
    }, match v1.clone() {
        Adt2::C0(_, _, _, _, h2438, _, _, _, _, _, _) => {
            h2438
        }
    }, match v2.clone() {
        Adt2::C0(h2439, _, _, _, _, _, _, _, _, _, _) => {
            h2439
        }
    }, match v2.clone() {
        Adt2::C0(_, h2440, _, _, _, _, _, _, _, _, _) => {
            h2440
        }
    }, match v2.clone() {
        Adt2::C0(_, _, h2441, _, _, _, _, _, _, _, _) => {
            h2441
        }
    }, match v2.clone() {
        Adt2::C0(_, _, _, h2442, _, _, _, _, _, _, _) => {
            h2442
        }
    }, match v2.clone() {
        Adt2::C0(_, _, _, _, h2443, _, _, _, _, _, _) => {
            h2443
        }
    }) {
        let m2445 = f94(match v0.clone() {
            Adt2::C0(h2444, _, _, _, _, _, _, _, _, _, _) => {
                h2444
            }
        });
        match m2445 {
            Adt14::C0 => {
                Ok(Adt10::C0)
            }
            Adt14::C1(v3) => {
                let a2451 = {
                    let a2448 = match v0.clone() {
                        Adt2::C0(_, h2446, _, _, _, _, _, _, _, _, _) => {
                            h2446
                        }
                    };
                    let a2449 = match v3.clone() {
                        Adt15::C0(_, h2447, _) => {
                            h2447
                        }
                    };
                    let a2450 = 0u64;
                    nat_rem(a2448, a2449, a2450)
                };
                let a2452 = 0u64;
                if if nat_eq(a2451, a2452) {
                    if f148(match v0.clone() {
                        Adt2::C0(_, h2453, _, _, _, _, _, _, _, _, _) => {
                            h2453
                        }
                    }, match v1.clone() {
                        Adt2::C0(_, h2454, _, _, _, _, _, _, _, _, _) => {
                            h2454
                        }
                    }, match v1.clone() {
                        Adt2::C0(_, _, h2455, _, _, _, _, _, _, _, _) => {
                            h2455
                        }
                    }, match v1.clone() {
                        Adt2::C0(_, _, _, h2456, _, _, _, _, _, _, _) => {
                            h2456
                        }
                    }) {
                        true
                    } else {
                        let a2458 = 4u64;
                        let a2459 = match v2.clone() {
                            Adt2::C0(_, _, _, _, _, h2457, _, _, _, _, _) => {
                                h2457
                            }
                        };
                        if if nat_le(a2458, a2459) {
                            let a2461 = match v3.clone() {
                                Adt15::C0(_, h2460, _) => {
                                    h2460
                                }
                            };
                            let a2462 = 1u64;
                            if nat_eq(a2461, a2462) {
                                true
                            } else {
                                let a2465 = match v3.clone() {
                                    Adt15::C0(_, _, h2463) => {
                                        h2463
                                    }
                                };
                                let a2466 = match v0.clone() {
                                    Adt2::C0(_, _, _, _, _, h2464, _, _, _, _, _) => {
                                        h2464
                                    }
                                };
                                nat_le(a2465, a2466)
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
                        Adt15::C0(_, h2467, _) => {
                            h2467
                        }
                    });
                    Ok(Adt10::C4(if f148(match v0.clone() {
                        Adt2::C0(_, h2468, _, _, _, _, _, _, _, _, _) => {
                            h2468
                        }
                    }, match v1.clone() {
                        Adt2::C0(_, h2469, _, _, _, _, _, _, _, _, _) => {
                            h2469
                        }
                    }, match v1.clone() {
                        Adt2::C0(_, _, h2470, _, _, _, _, _, _, _, _) => {
                            h2470
                        }
                    }, match v1.clone() {
                        Adt2::C0(_, _, _, h2471, _, _, _, _, _, _, _) => {
                            h2471
                        }
                    }) {
                        0u64
                    } else {
                        f131(v4.clone(), f16(v1.clone(), 1u64))?
                    }, 4u64, v4, f16(v2, 1u64), Adt4::C20(match v0 {
                        Adt2::C0(h2472, _, _, _, _, _, _, _, _, _, _) => {
                            h2472
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

pub fn f91(v0: Adt2, v1: Adt2, v2: Adt2) -> R<Adt10> {
    if if if if if if if if if if if if f93(match v0.clone() {
        Adt2::C0(h2473, _, _, _, _, _, _, _, _, _, _) => {
            h2473
        }
    }) {
        let a2475 = match v1.clone() {
            Adt2::C0(h2474, _, _, _, _, _, _, _, _, _, _) => {
                h2474
            }
        };
        let a2476 = 26u64;
        nat_eq(a2475, a2476)
    } else {
        false
    } {
        let a2478 = match v2.clone() {
            Adt2::C0(h2477, _, _, _, _, _, _, _, _, _, _) => {
                h2477
            }
        };
        let a2479 = 0u64;
        nat_eq(a2478, a2479)
    } else {
        false
    } {
        let a2482 = match v0.clone() {
            Adt2::C0(_, h2480, _, _, _, _, _, _, _, _, _) => {
                h2480
            }
        };
        let a2483 = match v2.clone() {
            Adt2::C0(_, h2481, _, _, _, _, _, _, _, _, _) => {
                h2481
            }
        };
        nat_eq(a2482, a2483)
    } else {
        false
    } {
        let a2486 = match v1.clone() {
            Adt2::C0(_, h2484, _, _, _, _, _, _, _, _, _) => {
                h2484
            }
        };
        let a2487 = match v0.clone() {
            Adt2::C0(_, _, h2485, _, _, _, _, _, _, _, _) => {
                h2485
            }
        };
        nat_le(a2486, a2487)
    } else {
        false
    } {
        let a2489 = match v0.clone() {
            Adt2::C0(_, _, _, h2488, _, _, _, _, _, _, _) => {
                h2488
            }
        };
        let a2490 = 1u64;
        nat_eq(a2489, a2490)
    } else {
        false
    } {
        let a2492 = match v0.clone() {
            Adt2::C0(_, _, _, _, h2491, _, _, _, _, _, _) => {
                h2491
            }
        };
        let a2493 = 1u64;
        nat_eq(a2492, a2493)
    } else {
        false
    } {
        let a2495 = match v2.clone() {
            Adt2::C0(_, _, _, h2494, _, _, _, _, _, _, _) => {
                h2494
            }
        };
        let a2496 = 1u64;
        nat_eq(a2495, a2496)
    } else {
        false
    } {
        let a2498 = match v2.clone() {
            Adt2::C0(_, _, _, _, h2497, _, _, _, _, _, _) => {
                h2497
            }
        };
        let a2499 = 1u64;
        nat_eq(a2498, a2499)
    } else {
        false
    } {
        let a2501 = match v1.clone() {
            Adt2::C0(_, _, h2500, _, _, _, _, _, _, _, _) => {
                h2500
            }
        };
        let a2502 = 1u64;
        nat_eq(a2501, a2502)
    } else {
        false
    } {
        let a2504 = match v1.clone() {
            Adt2::C0(_, _, _, h2503, _, _, _, _, _, _, _) => {
                h2503
            }
        };
        let a2505 = 1u64;
        nat_eq(a2504, a2505)
    } else {
        false
    } {
        let a2507 = match v1.clone() {
            Adt2::C0(_, _, _, _, h2506, _, _, _, _, _, _) => {
                h2506
            }
        };
        let a2508 = 1u64;
        nat_eq(a2507, a2508)
    } else {
        false
    } {
        if f27(match v2.clone() {
            Adt2::C0(_, h2509, _, _, _, _, _, _, _, _, _) => {
                h2509
            }
        }, match v2.clone() {
            Adt2::C0(_, _, h2510, _, _, _, _, _, _, _, _) => {
                h2510
            }
        }, match v2.clone() {
            Adt2::C0(_, _, _, h2511, _, _, _, _, _, _, _) => {
                h2511
            }
        }, match v2.clone() {
            Adt2::C0(_, _, _, _, h2512, _, _, _, _, _, _) => {
                h2512
            }
        }) {
            Ok(Adt10::C4(0u64, 4u64, f16(v0, 1u64), f16(v2, 1u64), Adt4::C9(0u64, 0u64)))
        } else {
            let a2514 = match v1.clone() {
                Adt2::C0(_, h2513, _, _, _, _, _, _, _, _, _) => {
                    h2513
                }
            };
            let a2515 = 0u64;
            if nat_eq(a2514, a2515) {
                Ok(Adt10::C4(f12(v2.clone(), 1u64)?, 4u64, f16(v0, 1u64), f16(v2, 1u64), Adt4::C9(0u64, 0u64)))
            } else {
                let a2517 = match v0.clone() {
                    Adt2::C0(_, h2516, _, _, _, _, _, _, _, _, _) => {
                        h2516
                    }
                };
                let a2518 = 2147483647u64;
                if if if if if if nat_le(a2517, a2518) {
                    let a2520 = match v1.clone() {
                        Adt2::C0(_, h2519, _, _, _, _, _, _, _, _, _) => {
                            h2519
                        }
                    };
                    let a2521 = 2147483647u64;
                    nat_le(a2520, a2521)
                } else {
                    false
                } {
                    let a2523 = 4u64;
                    let a2524 = match v2.clone() {
                        Adt2::C0(_, _, _, _, _, h2522, _, _, _, _, _) => {
                            h2522
                        }
                    };
                    nat_le(a2523, a2524)
                } else {
                    false
                } {
                    let a2530 = {
                        let a2527 = match v2.clone() {
                            Adt2::C0(_, h2525, _, _, _, _, _, _, _, _, _) => {
                                h2525
                            }
                        };
                        let a2528 = match v2.clone() {
                            Adt2::C0(_, _, _, _, _, h2526, _, _, _, _, _) => {
                                h2526
                            }
                        };
                        nat_mul(a2527, a2528)?
                    };
                    let a2531 = match v2.clone() {
                        Adt2::C0(_, _, _, _, _, _, h2529, _, _, _, _) => {
                            h2529
                        }
                    };
                    nat_le(a2530, a2531)
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
                        let a2545 = {
                            let a2535 = {
                                let a2533 = 2u64;
                                let a2534 = match v1.clone() {
                                    Adt2::C0(_, h2532, _, _, _, _, _, _, _, _, _) => {
                                        h2532
                                    }
                                };
                                nat_mul(a2533, a2534)?
                            };
                            let a2536 = f12(v2.clone(), 1u64)?;
                            nat_add(a2535, a2536)?
                        };
                        let a2546 = {
                            let a2543 = match v1.clone() {
                                Adt2::C0(_, h2537, _, _, _, _, _, _, _, _, _) => {
                                    h2537
                                }
                            };
                            let a2544 = {
                                let a2541 = 1u64;
                                let a2542 = {
                                    let a2539 = 2u64;
                                    let a2540 = match v0.clone() {
                                        Adt2::C0(_, h2538, _, _, _, _, _, _, _, _, _) => {
                                            h2538
                                        }
                                    };
                                    nat_mul(a2539, a2540)?
                                };
                                nat_add(a2541, a2542)?
                            };
                            nat_mul(a2543, a2544)?
                        };
                        nat_add(a2545, a2546)?
                    }, 4u64, Adt3::C0(match v0.clone() {
                        Adt2::C0(_, h2547, _, _, _, _, _, _, _, _, _) => {
                            h2547
                        }
                    }, match v1.clone() {
                        Adt2::C0(_, h2548, _, _, _, _, _, _, _, _, _) => {
                            h2548
                        }
                    }, 1u64, 1u64, match v0.clone() {
                        Adt2::C0(_, _, _, _, _, h2549, _, _, _, _, _) => {
                            h2549
                        }
                    }, match v0.clone() {
                        Adt2::C0(_, _, _, _, _, _, h2550, _, _, _, _) => {
                            h2550
                        }
                    }, match v0.clone() {
                        Adt2::C0(_, _, _, _, _, _, _, h2551, _, _, _) => {
                            h2551
                        }
                    }, match v0.clone() {
                        Adt2::C0(_, _, _, _, _, _, _, _, h2552, _, _) => {
                            h2552
                        }
                    }), f16(v2, 1u64), Adt4::C19(match v0 {
                        Adt2::C0(h2553, _, _, _, _, _, _, _, _, _, _) => {
                            h2553
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

pub fn f92(v0: u64, v1: Adt2, v2: bool, v3: Adt2) -> R<Adt10> {
    let a2556 = match v1.clone() {
        Adt2::C0(h2554, _, _, _, _, _, _, _, _, _, _) => {
            h2554
        }
    };
    let a2557 = match v3.clone() {
        Adt2::C0(h2555, _, _, _, _, _, _, _, _, _, _) => {
            h2555
        }
    };
    if if nat_eq(a2556, a2557) {
        let a2558 = v0;
        let a2559 = 33u64;
        if nat_eq(a2558, a2559) {
            v2
        } else {
            true
        }
    } else {
        false
    } {
        let m2561 = f94(match v1.clone() {
            Adt2::C0(h2560, _, _, _, _, _, _, _, _, _, _) => {
                h2560
            }
        });
        match m2561 {
            Adt14::C0 => {
                Ok(Adt10::C0)
            }
            Adt14::C1(v4) => {
                let v5: u64 = match v4.clone() {
                    Adt15::C0(_, h2562, _) => {
                        h2562
                    }
                };
                let v6: u64 = f12(v1.clone(), v5)?;
                let a2567 = {
                    let a2564 = match v1.clone() {
                        Adt2::C0(_, h2563, _, _, _, _, _, _, _, _, _) => {
                            h2563
                        }
                    };
                    let a2565 = v5;
                    let a2566 = 1u64;
                    nat_rem(a2564, a2565, a2566)
                };
                let a2568 = 0u64;
                if if if nat_eq(a2567, a2568) {
                    let a2573 = {
                        let a2570 = match v3.clone() {
                            Adt2::C0(_, h2569, _, _, _, _, _, _, _, _, _) => {
                                h2569
                            }
                        };
                        let a2571 = v5;
                        let a2572 = 1u64;
                        nat_rem(a2570, a2571, a2572)
                    };
                    let a2574 = 0u64;
                    nat_eq(a2573, a2574)
                } else {
                    false
                } {
                    let a2575 = v6;
                    let a2576 = f12(v3.clone(), v5)?;
                    nat_eq(a2575, a2576)
                } else {
                    false
                } {
                    Ok(Adt10::C4(v6, match v4 {
                        Adt15::C0(_, _, h2577) => {
                            h2577
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

pub fn f93(v0: u64) -> bool {
    let a2578 = v0;
    let a2579 = 0u64;
    if if nat_eq(a2578, a2579) {
        true
    } else {
        let a2580 = v0;
        let a2581 = 1u64;
        nat_eq(a2580, a2581)
    } {
        true
    } else {
        let a2582 = v0;
        let a2583 = 30u64;
        nat_eq(a2582, a2583)
    }
}

pub fn f94(v0: u64) -> Adt14 {
    let a2584 = v0;
    let a2585 = 0u64;
    if nat_eq(a2584, a2585) {
        Adt14::C1(Adt15::C0(0u64, 1u64, 4u64))
    } else {
        let a2586 = v0;
        let a2587 = 1u64;
        if nat_eq(a2586, a2587) {
            Adt14::C1(Adt15::C0(1u64, 1u64, 2u64))
        } else {
            let a2588 = v0;
            let a2589 = 2u64;
            if nat_eq(a2588, a2589) {
                Adt14::C1(Adt15::C0(2u64, 32u64, 18u64))
            } else {
                let a2590 = v0;
                let a2591 = 3u64;
                if nat_eq(a2590, a2591) {
                    Adt14::C1(Adt15::C0(3u64, 32u64, 20u64))
                } else {
                    let a2592 = v0;
                    let a2593 = 6u64;
                    if nat_eq(a2592, a2593) {
                        Adt14::C1(Adt15::C0(6u64, 32u64, 22u64))
                    } else {
                        let a2594 = v0;
                        let a2595 = 7u64;
                        if nat_eq(a2594, a2595) {
                            Adt14::C1(Adt15::C0(7u64, 32u64, 24u64))
                        } else {
                            let a2596 = v0;
                            let a2597 = 8u64;
                            if nat_eq(a2596, a2597) {
                                Adt14::C1(Adt15::C0(8u64, 32u64, 34u64))
                            } else {
                                let a2598 = v0;
                                let a2599 = 9u64;
                                if nat_eq(a2598, a2599) {
                                    Adt14::C1(Adt15::C0(9u64, 32u64, 36u64))
                                } else {
                                    let a2600 = v0;
                                    let a2601 = 10u64;
                                    if nat_eq(a2600, a2601) {
                                        Adt14::C1(Adt15::C0(10u64, 256u64, 84u64))
                                    } else {
                                        let a2602 = v0;
                                        let a2603 = 11u64;
                                        if nat_eq(a2602, a2603) {
                                            Adt14::C1(Adt15::C0(11u64, 256u64, 110u64))
                                        } else {
                                            let a2604 = v0;
                                            let a2605 = 12u64;
                                            if nat_eq(a2604, a2605) {
                                                Adt14::C1(Adt15::C0(12u64, 256u64, 144u64))
                                            } else {
                                                let a2606 = v0;
                                                let a2607 = 13u64;
                                                if nat_eq(a2606, a2607) {
                                                    Adt14::C1(Adt15::C0(13u64, 256u64, 176u64))
                                                } else {
                                                    let a2608 = v0;
                                                    let a2609 = 14u64;
                                                    if nat_eq(a2608, a2609) {
                                                        Adt14::C1(Adt15::C0(14u64, 256u64, 210u64))
                                                    } else {
                                                        let a2610 = v0;
                                                        let a2611 = 15u64;
                                                        if nat_eq(a2610, a2611) {
                                                            Adt14::C1(Adt15::C0(15u64, 256u64, 292u64))
                                                        } else {
                                                            let a2612 = v0;
                                                            let a2613 = 16u64;
                                                            if nat_eq(a2612, a2613) {
                                                                Adt14::C1(Adt15::C0(16u64, 256u64, 66u64))
                                                            } else {
                                                                let a2614 = v0;
                                                                let a2615 = 17u64;
                                                                if nat_eq(a2614, a2615) {
                                                                    Adt14::C1(Adt15::C0(17u64, 256u64, 74u64))
                                                                } else {
                                                                    let a2616 = v0;
                                                                    let a2617 = 18u64;
                                                                    if nat_eq(a2616, a2617) {
                                                                        Adt14::C1(Adt15::C0(18u64, 256u64, 98u64))
                                                                    } else {
                                                                        let a2618 = v0;
                                                                        let a2619 = 19u64;
                                                                        if nat_eq(a2618, a2619) {
                                                                            Adt14::C1(Adt15::C0(19u64, 256u64, 50u64))
                                                                        } else {
                                                                            let a2620 = v0;
                                                                            let a2621 = 20u64;
                                                                            if nat_eq(a2620, a2621) {
                                                                                Adt14::C1(Adt15::C0(20u64, 32u64, 18u64))
                                                                            } else {
                                                                                let a2622 = v0;
                                                                                let a2623 = 21u64;
                                                                                if nat_eq(a2622, a2623) {
                                                                                    Adt14::C1(Adt15::C0(21u64, 256u64, 110u64))
                                                                                } else {
                                                                                    let a2624 = v0;
                                                                                    let a2625 = 22u64;
                                                                                    if nat_eq(a2624, a2625) {
                                                                                        Adt14::C1(Adt15::C0(22u64, 256u64, 82u64))
                                                                                    } else {
                                                                                        let a2626 = v0;
                                                                                        let a2627 = 23u64;
                                                                                        if nat_eq(a2626, a2627) {
                                                                                            Adt14::C1(Adt15::C0(23u64, 256u64, 136u64))
                                                                                        } else {
                                                                                            let a2628 = v0;
                                                                                            let a2629 = 24u64;
                                                                                            if nat_eq(a2628, a2629) {
                                                                                                Adt14::C1(Adt15::C0(24u64, 1u64, 1u64))
                                                                                            } else {
                                                                                                let a2630 = v0;
                                                                                                let a2631 = 25u64;
                                                                                                if nat_eq(a2630, a2631) {
                                                                                                    Adt14::C1(Adt15::C0(25u64, 1u64, 2u64))
                                                                                                } else {
                                                                                                    let a2632 = v0;
                                                                                                    let a2633 = 26u64;
                                                                                                    if nat_eq(a2632, a2633) {
                                                                                                        Adt14::C1(Adt15::C0(26u64, 1u64, 4u64))
                                                                                                    } else {
                                                                                                        let a2634 = v0;
                                                                                                        let a2635 = 27u64;
                                                                                                        if nat_eq(a2634, a2635) {
                                                                                                            Adt14::C1(Adt15::C0(27u64, 1u64, 8u64))
                                                                                                        } else {
                                                                                                            let a2636 = v0;
                                                                                                            let a2637 = 28u64;
                                                                                                            if nat_eq(a2636, a2637) {
                                                                                                                Adt14::C1(Adt15::C0(28u64, 1u64, 8u64))
                                                                                                            } else {
                                                                                                                let a2638 = v0;
                                                                                                                let a2639 = 29u64;
                                                                                                                if nat_eq(a2638, a2639) {
                                                                                                                    Adt14::C1(Adt15::C0(29u64, 256u64, 56u64))
                                                                                                                } else {
                                                                                                                    let a2640 = v0;
                                                                                                                    let a2641 = 30u64;
                                                                                                                    if nat_eq(a2640, a2641) {
                                                                                                                        Adt14::C1(Adt15::C0(30u64, 1u64, 2u64))
                                                                                                                    } else {
                                                                                                                        let a2642 = v0;
                                                                                                                        let a2643 = 34u64;
                                                                                                                        if nat_eq(a2642, a2643) {
                                                                                                                            Adt14::C1(Adt15::C0(34u64, 256u64, 54u64))
                                                                                                                        } else {
                                                                                                                            let a2644 = v0;
                                                                                                                            let a2645 = 35u64;
                                                                                                                            if nat_eq(a2644, a2645) {
                                                                                                                                Adt14::C1(Adt15::C0(35u64, 256u64, 66u64))
                                                                                                                            } else {
                                                                                                                                let a2646 = v0;
                                                                                                                                let a2647 = 39u64;
                                                                                                                                if nat_eq(a2646, a2647) {
                                                                                                                                    Adt14::C1(Adt15::C0(39u64, 32u64, 17u64))
                                                                                                                                } else {
                                                                                                                                    let a2648 = v0;
                                                                                                                                    let a2649 = 40u64;
                                                                                                                                    if nat_eq(a2648, a2649) {
                                                                                                                                        Adt14::C1(Adt15::C0(40u64, 64u64, 36u64))
                                                                                                                                    } else {
                                                                                                                                        let a2650 = v0;
                                                                                                                                        let a2651 = 41u64;
                                                                                                                                        if nat_eq(a2650, a2651) {
                                                                                                                                            Adt14::C1(Adt15::C0(41u64, 128u64, 18u64))
                                                                                                                                        } else {
                                                                                                                                            let a2652 = v0;
                                                                                                                                            let a2653 = 42u64;
                                                                                                                                            if nat_eq(a2652, a2653) {
                                                                                                                                                Adt14::C1(Adt15::C0(42u64, 64u64, 18u64))
                                                                                                                                            } else {
                                                                                                                                                let a2654 = v0;
                                                                                                                                                let a2655 = 43u64;
                                                                                                                                                if nat_eq(a2654, a2655) {
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

pub fn f95(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64, v7: u64) -> bool {
    if f27(v4, v5, v6, v7) {
        f27(v0, v1, v2, v3)
    } else {
        let a2659 = {
            let a2656 = v0;
            let a2657 = v4;
            let a2658 = 1u64;
            nat_rem(a2656, a2657, a2658)
        };
        let a2660 = 0u64;
        if if if nat_eq(a2659, a2660) {
            let a2664 = {
                let a2661 = v1;
                let a2662 = v5;
                let a2663 = 1u64;
                nat_rem(a2661, a2662, a2663)
            };
            let a2665 = 0u64;
            nat_eq(a2664, a2665)
        } else {
            false
        } {
            let a2669 = {
                let a2666 = v2;
                let a2667 = v6;
                let a2668 = 1u64;
                nat_rem(a2666, a2667, a2668)
            };
            let a2670 = 0u64;
            nat_eq(a2669, a2670)
        } else {
            false
        } {
            let a2674 = {
                let a2671 = v3;
                let a2672 = v7;
                let a2673 = 1u64;
                nat_rem(a2671, a2672, a2673)
            };
            let a2675 = 0u64;
            nat_eq(a2674, a2675)
        } else {
            false
        }
    }
}

pub fn f96(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64) -> R<bool> {
    let a2676 = v0;
    let a2677 = v1;
    if nat_eq(a2676, a2677) {
        let a2680 = v4;
        let a2681 = {
            let a2678 = v2;
            let a2679 = v3;
            nat_add(a2678, a2679)?
        };
        Ok(nat_eq(a2680, a2681))
    } else {
        let a2682 = v2;
        let a2683 = v3;
        if nat_eq(a2682, a2683) {
            let a2684 = v4;
            let a2685 = v2;
            Ok(nat_eq(a2684, a2685))
        } else {
            Ok(false)
        }
    }
}

pub fn f97(v0: u64, v1: u64, v2: u64, v3: u64) -> R<bool> {
    let v4: u64 = {
        let a2688 = {
            let a2686 = v0;
            let a2687 = f110(v1);
            nat_add(a2686, a2687)?
        };
        let a2689 = f110(v2);
        nat_add(a2688, a2689)?
    };
    let v5: u64 = {
        let a2690 = f109(v1);
        let a2691 = f109(v2);
        nat_add(a2690, a2691)?
    };
    let a2692 = v1;
    let a2693 = 4294967295u64;
    if if nat_le(a2692, a2693) {
        let a2694 = v2;
        let a2695 = 4294967295u64;
        nat_le(a2694, a2695)
    } else {
        false
    } {
        let a2696 = v5;
        let a2697 = v4;
        if nat_le(a2696, a2697) {
            let a2700 = v3;
            let a2701 = {
                let a2698 = v4;
                let a2699 = v5;
                nat_sub(a2698, a2699)
            };
            Ok(nat_eq(a2700, a2701))
        } else {
            Ok(false)
        }
    } else {
        Ok(false)
    }
}

pub fn f98(v0: u64, v1: u64) -> R<bool> {
    let a2702 = v1;
    let a2703 = 4294967295u64;
    if nat_le(a2702, a2703) {
        let a2706 = {
            let a2704 = f110(v1);
            let a2705 = f109(v1);
            nat_add(a2704, a2705)?
        };
        let a2707 = v0;
        Ok(nat_lt(a2706, a2707))
    } else {
        Ok(false)
    }
}

pub fn f99(v0: Adt3, v1: u64, v2: u64, v3: u64, v4: bool) -> R<bool> {
    let v5: u64 = f149(v0.clone(), false)?;
    let v6: u64 = f149(v0, true)?;
    let a2708 = v6;
    let a2709 = v1;
    if if nat_le(a2708, a2709) {
        let a2716 = {
            let a2714 = {
                let a2710 = v1;
                let a2711 = v5;
                nat_add(a2710, a2711)?
            };
            let a2715 = {
                let a2712 = v3;
                let a2713 = 0u64;
                if nat_eq(a2712, a2713) {
                    0u64
                } else {
                    4u64
                }
            };
            nat_add(a2714, a2715)?
        };
        let a2717 = v2;
        nat_le(a2716, a2717)
    } else {
        false
    } {
        if v4 {
            let a2722 = {
                let a2720 = {
                    let a2718 = v1;
                    let a2719 = v5;
                    nat_add(a2718, a2719)?
                };
                let a2721 = v6;
                nat_sub(a2720, a2721)
            };
            let a2723 = v2;
            Ok(nat_lt(a2722, a2723))
        } else {
            let a2728 = {
                let a2726 = {
                    let a2724 = v1;
                    let a2725 = v5;
                    nat_add(a2724, a2725)?
                };
                let a2727 = v6;
                nat_sub(a2726, a2727)
            };
            let a2729 = v2;
            Ok(nat_le(a2728, a2729))
        }
    } else {
        Ok(false)
    }
}

pub fn f100(v0: u64, v1: u64) -> bool {
    let a2730 = v0;
    let a2731 = 0u64;
    if if nat_eq(a2730, a2731) {
        true
    } else {
        let a2732 = v0;
        let a2733 = 1u64;
        nat_eq(a2732, a2733)
    } {
        let a2734 = v1;
        let a2735 = 4294967296u64;
        nat_lt(a2734, a2735)
    } else {
        false
    }
}

pub fn f101(v0: u64, v1: u64) -> R<u64> {
    let a2740 = {
        let a2738 = v0;
        let a2739 = {
            let a2736 = v1;
            let a2737 = 1u64;
            nat_sub(a2736, a2737)
        };
        nat_add(a2738, a2739)?
    };
    let a2741 = v1;
    let a2742 = 0u64;
    Ok(nat_quot(a2740, a2741, a2742))
}

pub fn f102(v0: Adt2, v1: Adt2, v2: u64, v3: u64) -> R<bool> {
    if f27(match v0.clone() {
        Adt2::C0(_, h2743, _, _, _, _, _, _, _, _, _) => {
            h2743
        }
    }, match v0.clone() {
        Adt2::C0(_, _, h2744, _, _, _, _, _, _, _, _) => {
            h2744
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, h2745, _, _, _, _, _, _, _) => {
            h2745
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, h2746, _, _, _, _, _, _) => {
            h2746
        }
    }) {
        Ok(true)
    } else {
        let v4: u64 = {
            let a2749 = match v1.clone() {
                Adt2::C0(_, h2747, _, _, _, _, _, _, _, _, _) => {
                    h2747
                }
            };
            let a2750 = match v1.clone() {
                Adt2::C0(_, _, h2748, _, _, _, _, _, _, _, _) => {
                    h2748
                }
            };
            nat_mul(a2749, a2750)?
        };
        let v5: u64 = {
            let a2752 = v4;
            let a2753 = match v1.clone() {
                Adt2::C0(_, _, _, h2751, _, _, _, _, _, _, _) => {
                    h2751
                }
            };
            nat_mul(a2752, a2753)?
        };
        let a2766 = {
            let a2764 = {
                let a2755 = match v0.clone() {
                    Adt2::C0(_, h2754, _, _, _, _, _, _, _, _, _) => {
                        h2754
                    }
                };
                let a2756 = 1u64;
                nat_sub(a2755, a2756)
            };
            let a2765 = {
                let a2762 = f150(match v0.clone() {
                    Adt2::C0(_, _, h2757, _, _, _, _, _, _, _, _) => {
                        h2757
                    }
                }, v2, v5, match v1.clone() {
                    Adt2::C0(_, h2758, _, _, _, _, _, _, _, _, _) => {
                        h2758
                    }
                })?;
                let a2763 = f150(match v0 {
                    Adt2::C0(_, _, _, h2759, _, _, _, _, _, _, _) => {
                        h2759
                    }
                }, v2, {
                    let a2760 = v3;
                    let a2761 = v5;
                    nat_mul(a2760, a2761)?
                }, v4)?;
                nat_add(a2762, a2763)?
            };
            nat_add(a2764, a2765)?
        };
        let a2767 = f12(v1, 1u64)?;
        Ok(nat_lt(a2766, a2767))
    }
}

pub fn f103(v0: u64, v1: u64, v2: u64) -> R<bool> {
    let a2768 = v0;
    let a2769 = 2147483648u64;
    if if if if nat_lt(a2768, a2769) {
        let a2770 = v1;
        let a2771 = 2147483648u64;
        nat_lt(a2770, a2771)
    } else {
        false
    } {
        let a2774 = {
            let a2772 = v0;
            let a2773 = v1;
            nat_mul(a2772, a2773)?
        };
        let a2775 = 2147483648u64;
        nat_lt(a2774, a2775)
    } else {
        false
    } {
        let a2780 = {
            let a2778 = {
                let a2776 = v0;
                let a2777 = 1u64;
                nat_sub(a2776, a2777)
            };
            let a2779 = v2;
            nat_mul(a2778, a2779)?
        };
        let a2781 = 2147483648u64;
        nat_lt(a2780, a2781)
    } else {
        false
    } {
        let a2786 = {
            let a2784 = {
                let a2782 = v1;
                let a2783 = 1u64;
                nat_sub(a2782, a2783)
            };
            let a2785 = v2;
            nat_mul(a2784, a2785)?
        };
        let a2787 = 2147483648u64;
        Ok(nat_lt(a2786, a2787))
    } else {
        Ok(false)
    }
}

pub fn f104(v0: u64) -> R<u64> {
    let a2790 = 3u64;
    let a2791 = {
        let a2788 = v0;
        let a2789 = 3u64;
        nat_mul(a2788, a2789)?
    };
    nat_add(a2790, a2791)
}

pub fn f105(v0: Adt3) -> R<u64> {
    if f27(match v0.clone() {
        Adt3::C0(h2792, _, _, _, _, _, _, _) => {
            h2792
        }
    }, match v0.clone() {
        Adt3::C0(_, h2793, _, _, _, _, _, _) => {
            h2793
        }
    }, match v0.clone() {
        Adt3::C0(_, _, h2794, _, _, _, _, _) => {
            h2794
        }
    }, match v0.clone() {
        Adt3::C0(_, _, _, h2795, _, _, _, _) => {
            h2795
        }
    }) {
        Ok(0u64)
    } else {
        let a2804 = match v0.clone() {
            Adt3::C0(h2796, _, _, _, _, _, _, _) => {
                h2796
            }
        };
        let a2805 = {
            let a2802 = match v0.clone() {
                Adt3::C0(_, h2797, _, _, _, _, _, _) => {
                    h2797
                }
            };
            let a2803 = {
                let a2800 = match v0.clone() {
                    Adt3::C0(_, _, h2798, _, _, _, _, _) => {
                        h2798
                    }
                };
                let a2801 = match v0 {
                    Adt3::C0(_, _, _, h2799, _, _, _, _) => {
                        h2799
                    }
                };
                nat_mul(a2800, a2801)?
            };
            nat_mul(a2802, a2803)?
        };
        nat_mul(a2804, a2805)
    }
}

pub fn f106(v0: Adt3, v1: u64, v2: u64) -> R<u64> {
    let a2806 = v1;
    let a2807 = 13u64;
    if if nat_eq(a2806, a2807) {
        f27(match v0.clone() {
            Adt3::C0(h2808, _, _, _, _, _, _, _) => {
                h2808
            }
        }, match v0.clone() {
            Adt3::C0(_, h2809, _, _, _, _, _, _) => {
                h2809
            }
        }, match v0.clone() {
            Adt3::C0(_, _, h2810, _, _, _, _, _) => {
                h2810
            }
        }, match v0.clone() {
            Adt3::C0(_, _, _, h2811, _, _, _, _) => {
                h2811
            }
        })
    } else {
        false
    } {
        Ok(2u64)
    } else {
        let v3: u64 = {
            let a2817 = match v0.clone() {
                Adt3::C0(_, h2812, _, _, _, _, _, _) => {
                    h2812
                }
            };
            let a2818 = {
                let a2815 = match v0.clone() {
                    Adt3::C0(_, _, h2813, _, _, _, _, _) => {
                        h2813
                    }
                };
                let a2816 = match v0.clone() {
                    Adt3::C0(_, _, _, h2814, _, _, _, _) => {
                        h2814
                    }
                };
                nat_mul(a2815, a2816)?
            };
            nat_mul(a2817, a2818)?
        };
        let a2819 = v1;
        let a2820 = 13u64;
        if nat_eq(a2819, a2820) {
            let a2829 = 2u64;
            let a2830 = if f151(v0.clone(), v2) {
                let a2822 = v3;
                let a2823 = f152(match v0 {
                    Adt3::C0(h2821, _, _, _, _, _, _, _) => {
                        h2821
                    }
                }, false)?;
                nat_mul(a2822, a2823)?
            } else {
                let a2827 = v3;
                let a2828 = {
                    let a2825 = match v0 {
                        Adt3::C0(h2824, _, _, _, _, _, _, _) => {
                            h2824
                        }
                    };
                    let a2826 = 2u64;
                    nat_mul(a2825, a2826)?
                };
                nat_mul(a2827, a2828)?
            };
            nat_add(a2829, a2830)
        } else {
            let a2834 = v3;
            let a2835 = f152(match v0 {
                Adt3::C0(h2831, _, _, _, _, _, _, _) => {
                    h2831
                }
            }, {
                let a2832 = v1;
                let a2833 = 16u64;
                nat_eq(a2832, a2833)
            })?;
            nat_mul(a2834, a2835)
        }
    }
}

pub fn f107(v0: u64, v1: u64, v2: Adt2, v3: Option<Adt2>, v4: Adt2) -> R<Adt10> {
    let a2836 = v0;
    let a2837 = 30u64;
    if if nat_eq(a2836, a2837) {
        let a2840 = match v2.clone() {
            Adt2::C0(_, _, _, _, _, _, h2838, _, _, _, _) => {
                h2838
            }
        };
        let a2841 = match v2.clone() {
            Adt2::C0(_, _, _, _, _, h2839, _, _, _, _, _) => {
                h2839
            }
        };
        nat_lt(a2840, a2841)
    } else {
        false
    } {
        Ok(Adt10::C1)
    } else {
        let m2842 = v3;
        match m2842 {
            None => {
                f153(v0, v1, match v2.clone() {
                    Adt2::C0(h2843, _, _, _, _, _, _, _, _, _, _) => {
                        h2843
                    }
                }, 0u64, match v4.clone() {
                    Adt2::C0(h2844, _, _, _, _, _, _, _, _, _, _) => {
                        h2844
                    }
                }, false, match v2.clone() {
                    Adt2::C0(_, h2845, _, _, _, _, _, _, _, _, _) => {
                        h2845
                    }
                }, match v2.clone() {
                    Adt2::C0(_, _, h2846, _, _, _, _, _, _, _, _) => {
                        h2846
                    }
                }, match v2.clone() {
                    Adt2::C0(_, _, _, h2847, _, _, _, _, _, _, _) => {
                        h2847
                    }
                }, match v2 {
                    Adt2::C0(_, _, _, _, h2848, _, _, _, _, _, _) => {
                        h2848
                    }
                }, 1u64, 1u64, 1u64, 1u64, match v4.clone() {
                    Adt2::C0(_, h2849, _, _, _, _, _, _, _, _, _) => {
                        h2849
                    }
                }, match v4.clone() {
                    Adt2::C0(_, _, h2850, _, _, _, _, _, _, _, _) => {
                        h2850
                    }
                }, match v4.clone() {
                    Adt2::C0(_, _, _, h2851, _, _, _, _, _, _, _) => {
                        h2851
                    }
                }, match v4 {
                    Adt2::C0(_, _, _, _, h2852, _, _, _, _, _, _) => {
                        h2852
                    }
                })
            }
            Some(v5) => {
                f153(v0, v1, match v2.clone() {
                    Adt2::C0(h2853, _, _, _, _, _, _, _, _, _, _) => {
                        h2853
                    }
                }, match v5.clone() {
                    Adt2::C0(h2854, _, _, _, _, _, _, _, _, _, _) => {
                        h2854
                    }
                }, match v4.clone() {
                    Adt2::C0(h2855, _, _, _, _, _, _, _, _, _, _) => {
                        h2855
                    }
                }, true, match v2.clone() {
                    Adt2::C0(_, h2856, _, _, _, _, _, _, _, _, _) => {
                        h2856
                    }
                }, match v2.clone() {
                    Adt2::C0(_, _, h2857, _, _, _, _, _, _, _, _) => {
                        h2857
                    }
                }, match v2.clone() {
                    Adt2::C0(_, _, _, h2858, _, _, _, _, _, _, _) => {
                        h2858
                    }
                }, match v2 {
                    Adt2::C0(_, _, _, _, h2859, _, _, _, _, _, _) => {
                        h2859
                    }
                }, match v5.clone() {
                    Adt2::C0(_, h2860, _, _, _, _, _, _, _, _, _) => {
                        h2860
                    }
                }, match v5.clone() {
                    Adt2::C0(_, _, h2861, _, _, _, _, _, _, _, _) => {
                        h2861
                    }
                }, match v5.clone() {
                    Adt2::C0(_, _, _, h2862, _, _, _, _, _, _, _) => {
                        h2862
                    }
                }, match v5 {
                    Adt2::C0(_, _, _, _, h2863, _, _, _, _, _, _) => {
                        h2863
                    }
                }, match v4.clone() {
                    Adt2::C0(_, h2864, _, _, _, _, _, _, _, _, _) => {
                        h2864
                    }
                }, match v4.clone() {
                    Adt2::C0(_, _, h2865, _, _, _, _, _, _, _, _) => {
                        h2865
                    }
                }, match v4.clone() {
                    Adt2::C0(_, _, _, h2866, _, _, _, _, _, _, _) => {
                        h2866
                    }
                }, match v4 {
                    Adt2::C0(_, _, _, _, h2867, _, _, _, _, _, _) => {
                        h2867
                    }
                })
            }
        }
    }
}

pub fn f108(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64, v7: u64) -> R<u64> {
    let a2880 = {
        let a2876 = {
            let a2872 = {
                let a2868 = v0;
                let a2869 = v4;
                nat_mul(a2868, a2869)?
            };
            let a2873 = {
                let a2870 = v1;
                let a2871 = v5;
                nat_mul(a2870, a2871)?
            };
            nat_add(a2872, a2873)?
        };
        let a2877 = {
            let a2874 = v2;
            let a2875 = v6;
            nat_mul(a2874, a2875)?
        };
        nat_add(a2876, a2877)?
    };
    let a2881 = {
        let a2878 = v3;
        let a2879 = v7;
        nat_mul(a2878, a2879)?
    };
    nat_add(a2880, a2881)
}

pub fn f109(v0: u64) -> u64 {
    let a2882 = v0;
    let a2883 = 2147483648u64;
    if nat_lt(a2882, a2883) {
        0u64
    } else {
        let a2884 = 4294967296u64;
        let a2885 = v0;
        nat_sub(a2884, a2885)
    }
}

pub fn f110(v0: u64) -> u64 {
    let a2886 = v0;
    let a2887 = 2147483648u64;
    if nat_lt(a2886, a2887) {
        v0
    } else {
        0u64
    }
}

pub fn f111(v0: Adt11, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64, v7: u64, v8: u64) -> Adt9 {
    let v9: u64 = {
        let a2888 = v7;
        let a2889 = 6u64;
        if nat_eq(a2888, a2889) {
            27u64
        } else {
            v2
        }
    };
    Adt9::C7(v0, f76(v3, v4), f76(v5, v6), f77(v7, v8), f154(v1), f154(v9))
}

pub fn f112(v0: Adt4, v1: u64, v2: u64) -> Adt11 {
    let m2890 = v0;
    match m2890 {
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
            let a2891 = 13u64;
            let a2892 = v34;
            if nat_le(a2891, a2892) {
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
        Adt4::C27(v52) => {
            Adt11::C2(match v52.clone() {
                Adt5::C0(_, _, _, _, _, h2893, _, _) => {
                    h2893
                }
            }, match v52 {
                Adt5::C0(_, _, _, _, _, _, _, h2894) => {
                    h2894
                }
            }, true)
        }
        Adt4::C28(_) => {
            Adt11::C6
        }
        Adt4::C29(_) => {
            Adt11::C6
        }
    }
}

pub fn f113(v0: Adt4) -> u64 {
    let m2895 = v0;
    match m2895 {
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
    }
}

pub fn f114(v0: Adt4, v1: u64) -> u64 {
    let m2896 = v0;
    match m2896 {
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
            let a2897 = v1;
            let a2898 = 6u64;
            if nat_eq(a2897, a2898) {
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
        Adt4::C26(v52, _, _, _) => {
            let a2903 = v1;
            let a2904 = 6u64;
            if if if nat_eq(a2903, a2904) {
                true
            } else {
                let a2905 = v1;
                let a2906 = 7u64;
                nat_eq(a2905, a2906)
            } {
                let a2907 = v52;
                let a2908 = 30u64;
                nat_eq(a2907, a2908)
            } else {
                false
            } {
                30u64
            } else {
                0u64
            }
        }
        Adt4::C27(v51) => {
            match v51 {
                Adt5::C0(_, _, _, _, _, _, _, h2902) => {
                    h2902
                }
            }
        }
        Adt4::C28(v49) => {
            let a2899 = v1;
            let a2900 = 6u64;
            if nat_eq(a2899, a2900) {
                match v49 {
                    Adt7::C0(_, _, _, _, h2901) => {
                        h2901
                    }
                }
            } else {
                0u64
            }
        }
        Adt4::C29(_) => {
            0u64
        }
    }
}

pub fn f115(v0: Adt4, v1: u64) -> Adt11 {
    let a2909 = v1;
    let a2910 = 4u64;
    if nat_lt(a2909, a2910) {
        Adt11::C0(v1)
    } else {
        let a2911 = v1;
        let a2912 = 4u64;
        if nat_eq(a2911, a2912) {
            Adt11::C1(0u64)
        } else {
            let a2913 = v1;
            let a2914 = 5u64;
            if nat_eq(a2913, a2914) {
                Adt11::C1(3u64)
            } else {
                let a2915 = v1;
                let a2916 = 6u64;
                if nat_eq(a2915, a2916) {
                    Adt11::C2(f114(v0, v1), 0u64, false)
                } else {
                    let a2917 = v1;
                    let a2918 = 7u64;
                    if nat_eq(a2917, a2918) {
                        Adt11::C2(0u64, f114(v0, v1), false)
                    } else {
                        let a2919 = v1;
                        let a2920 = 16u64;
                        if nat_lt(a2919, a2920) {
                            Adt11::C1({
                                let a2921 = v1;
                                let a2922 = 8u64;
                                nat_sub(a2921, a2922)
                            })
                        } else {
                            let a2923 = v1;
                            let a2924 = 20u64;
                            if nat_lt(a2923, a2924) {
                                Adt11::C5({
                                    let a2925 = v1;
                                    let a2926 = 16u64;
                                    nat_sub(a2925, a2926)
                                })
                            } else {
                                let a2927 = v1;
                                let a2928 = 20u64;
                                if nat_eq(a2927, a2928) {
                                    Adt11::C3(0u64, 0u64, false)
                                } else {
                                    let a2929 = v1;
                                    let a2930 = 21u64;
                                    if nat_eq(a2929, a2930) {
                                        Adt11::C0(4u64)
                                    } else {
                                        let a2931 = v1;
                                        let a2932 = 22u64;
                                        if nat_eq(a2931, a2932) {
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

pub fn f116(v0: u64, v1: Adt3) -> R<u64> {
    let v2: u64 = {
        let a2934 = v0;
        let a2935 = match v1.clone() {
            Adt3::C0(h2933, _, _, _, _, _, _, _) => {
                h2933
            }
        };
        let a2936 = 0u64;
        nat_quot(a2934, a2935, a2936)
    };
    let v3: u64 = {
        let a2938 = v2;
        let a2939 = match v1.clone() {
            Adt3::C0(_, h2937, _, _, _, _, _, _) => {
                h2937
            }
        };
        let a2940 = 0u64;
        nat_quot(a2938, a2939, a2940)
    };
    f108({
        let a2942 = v0;
        let a2943 = match v1.clone() {
            Adt3::C0(h2941, _, _, _, _, _, _, _) => {
                h2941
            }
        };
        let a2944 = 0u64;
        nat_rem(a2942, a2943, a2944)
    }, {
        let a2946 = v2;
        let a2947 = match v1.clone() {
            Adt3::C0(_, h2945, _, _, _, _, _, _) => {
                h2945
            }
        };
        let a2948 = 0u64;
        nat_rem(a2946, a2947, a2948)
    }, {
        let a2950 = v3;
        let a2951 = match v1.clone() {
            Adt3::C0(_, _, h2949, _, _, _, _, _) => {
                h2949
            }
        };
        let a2952 = 0u64;
        nat_rem(a2950, a2951, a2952)
    }, {
        let a2954 = v3;
        let a2955 = match v1.clone() {
            Adt3::C0(_, _, h2953, _, _, _, _, _) => {
                h2953
            }
        };
        let a2956 = 0u64;
        nat_quot(a2954, a2955, a2956)
    }, match v1.clone() {
        Adt3::C0(_, _, _, _, h2957, _, _, _) => {
            h2957
        }
    }, match v1.clone() {
        Adt3::C0(_, _, _, _, _, h2958, _, _) => {
            h2958
        }
    }, match v1.clone() {
        Adt3::C0(_, _, _, _, _, _, h2959, _) => {
            h2959
        }
    }, match v1 {
        Adt3::C0(_, _, _, _, _, _, _, h2960) => {
            h2960
        }
    })
}

pub fn f117(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let a2978 = {
        let a2973 = f155(v2, {
            let a2964 = {
                let a2961 = v0;
                let a2962 = v1;
                let a2963 = 0u64;
                nat_quot(a2961, a2962, a2963)
            };
            let a2965 = v2;
            let a2966 = 0u64;
            nat_rem(a2964, a2965, a2966)
        }, {
            let a2970 = {
                let a2967 = v0;
                let a2968 = v1;
                let a2969 = 0u64;
                nat_quot(a2967, a2968, a2969)
            };
            let a2971 = v2;
            let a2972 = 0u64;
            nat_quot(a2970, a2971, a2972)
        })?;
        let a2974 = v1;
        nat_mul(a2973, a2974)?
    };
    let a2979 = {
        let a2975 = v0;
        let a2976 = v1;
        let a2977 = 0u64;
        nat_rem(a2975, a2976, a2977)
    };
    nat_add(a2978, a2979)
}

pub fn f118(v0: u64, v1: Adt3, v2: Adt3) -> R<u64> {
    let v3: u64 = {
        let a2981 = v0;
        let a2982 = match v2.clone() {
            Adt3::C0(h2980, _, _, _, _, _, _, _) => {
                h2980
            }
        };
        let a2983 = 0u64;
        nat_quot(a2981, a2982, a2983)
    };
    let v4: u64 = {
        let a2985 = v3;
        let a2986 = match v2.clone() {
            Adt3::C0(_, h2984, _, _, _, _, _, _) => {
                h2984
            }
        };
        let a2987 = 0u64;
        nat_quot(a2985, a2986, a2987)
    };
    f156({
        let a2989 = v0;
        let a2990 = match v2.clone() {
            Adt3::C0(h2988, _, _, _, _, _, _, _) => {
                h2988
            }
        };
        let a2991 = 0u64;
        nat_rem(a2989, a2990, a2991)
    }, {
        let a2993 = v3;
        let a2994 = match v2.clone() {
            Adt3::C0(_, h2992, _, _, _, _, _, _) => {
                h2992
            }
        };
        let a2995 = 0u64;
        nat_rem(a2993, a2994, a2995)
    }, {
        let a2997 = v4;
        let a2998 = match v2.clone() {
            Adt3::C0(_, _, h2996, _, _, _, _, _) => {
                h2996
            }
        };
        let a2999 = 0u64;
        nat_rem(a2997, a2998, a2999)
    }, {
        let a3001 = v4;
        let a3002 = match v2 {
            Adt3::C0(_, _, h3000, _, _, _, _, _) => {
                h3000
            }
        };
        let a3003 = 0u64;
        nat_quot(a3001, a3002, a3003)
    }, match v1.clone() {
        Adt3::C0(h3004, _, _, _, _, _, _, _) => {
            h3004
        }
    }, match v1.clone() {
        Adt3::C0(_, h3005, _, _, _, _, _, _) => {
            h3005
        }
    }, match v1.clone() {
        Adt3::C0(_, _, h3006, _, _, _, _, _) => {
            h3006
        }
    }, match v1.clone() {
        Adt3::C0(_, _, _, h3007, _, _, _, _) => {
            h3007
        }
    }, match v1.clone() {
        Adt3::C0(_, _, _, _, h3008, _, _, _) => {
            h3008
        }
    }, match v1.clone() {
        Adt3::C0(_, _, _, _, _, h3009, _, _) => {
            h3009
        }
    }, match v1.clone() {
        Adt3::C0(_, _, _, _, _, _, h3010, _) => {
            h3010
        }
    }, match v1 {
        Adt3::C0(_, _, _, _, _, _, _, h3011) => {
            h3011
        }
    })
}

pub fn f119(v0: u64, v1: Adt3, v2: Adt3, v3: Adt3, v4: u64) -> R<Adt9> {
    let v5: u64 = {
        let a3013 = v0;
        let a3014 = match v3.clone() {
            Adt3::C0(h3012, _, _, _, _, _, _, _) => {
                h3012
            }
        };
        let a3015 = 0u64;
        nat_quot(a3013, a3014, a3015)
    };
    let v6: u64 = {
        let a3017 = v5;
        let a3018 = match v3.clone() {
            Adt3::C0(_, h3016, _, _, _, _, _, _) => {
                h3016
            }
        };
        let a3019 = 0u64;
        nat_quot(a3017, a3018, a3019)
    };
    let v7: u64 = {
        let a3021 = v0;
        let a3022 = match v3.clone() {
            Adt3::C0(h3020, _, _, _, _, _, _, _) => {
                h3020
            }
        };
        let a3023 = 0u64;
        nat_rem(a3021, a3022, a3023)
    };
    let v8: u64 = {
        let a3025 = v5;
        let a3026 = match v3.clone() {
            Adt3::C0(_, h3024, _, _, _, _, _, _) => {
                h3024
            }
        };
        let a3027 = 0u64;
        nat_rem(a3025, a3026, a3027)
    };
    let v9: u64 = {
        let a3029 = v6;
        let a3030 = match v3.clone() {
            Adt3::C0(_, _, h3028, _, _, _, _, _) => {
                h3028
            }
        };
        let a3031 = 0u64;
        nat_rem(a3029, a3030, a3031)
    };
    let v10: u64 = {
        let a3033 = v6;
        let a3034 = match v3.clone() {
            Adt3::C0(_, _, h3032, _, _, _, _, _) => {
                h3032
            }
        };
        let a3035 = 0u64;
        nat_quot(a3033, a3034, a3035)
    };
    let v11: u64 = {
        let a3036 = v4;
        let a3037 = 0u64;
        if nat_eq(a3036, a3037) {
            match v1.clone() {
                Adt3::C0(h3038, _, _, _, _, _, _, _) => {
                    h3038
                }
            }
        } else {
            let a3039 = v4;
            let a3040 = 1u64;
            if nat_eq(a3039, a3040) {
                match v1.clone() {
                    Adt3::C0(_, h3041, _, _, _, _, _, _) => {
                        h3041
                    }
                }
            } else {
                let a3042 = v4;
                let a3043 = 2u64;
                if nat_eq(a3042, a3043) {
                    match v1.clone() {
                        Adt3::C0(_, _, h3044, _, _, _, _, _) => {
                            h3044
                        }
                    }
                } else {
                    match v1.clone() {
                        Adt3::C0(_, _, _, h3045, _, _, _, _) => {
                            h3045
                        }
                    }
                }
            }
        }
    };
    let v12: bool = {
        let a3052 = v11;
        let a3053 = {
            let a3046 = v4;
            let a3047 = 0u64;
            if nat_eq(a3046, a3047) {
                v7
            } else {
                let a3048 = v4;
                let a3049 = 1u64;
                if nat_eq(a3048, a3049) {
                    v8
                } else {
                    let a3050 = v4;
                    let a3051 = 2u64;
                    if nat_eq(a3050, a3051) {
                        v9
                    } else {
                        v10
                    }
                }
            }
        };
        nat_le(a3052, a3053)
    };
    let v13: Adt3 = if v12 {
        v2
    } else {
        v1
    };
    Ok(Adt9::C1(v12, f108(if if v12 {
        let a3054 = v4;
        let a3055 = 0u64;
        nat_eq(a3054, a3055)
    } else {
        false
    } {
        let a3056 = v7;
        let a3057 = v11;
        nat_sub(a3056, a3057)
    } else {
        v7
    }, if if v12 {
        let a3058 = v4;
        let a3059 = 1u64;
        nat_eq(a3058, a3059)
    } else {
        false
    } {
        let a3060 = v8;
        let a3061 = v11;
        nat_sub(a3060, a3061)
    } else {
        v8
    }, if if v12 {
        let a3062 = v4;
        let a3063 = 2u64;
        nat_eq(a3062, a3063)
    } else {
        false
    } {
        let a3064 = v9;
        let a3065 = v11;
        nat_sub(a3064, a3065)
    } else {
        v9
    }, if if v12 {
        let a3066 = v4;
        let a3067 = 3u64;
        nat_eq(a3066, a3067)
    } else {
        false
    } {
        let a3068 = v10;
        let a3069 = v11;
        nat_sub(a3068, a3069)
    } else {
        v10
    }, match v13.clone() {
        Adt3::C0(_, _, _, _, h3070, _, _, _) => {
            h3070
        }
    }, match v13.clone() {
        Adt3::C0(_, _, _, _, _, h3071, _, _) => {
            h3071
        }
    }, match v13.clone() {
        Adt3::C0(_, _, _, _, _, _, h3072, _) => {
            h3072
        }
    }, match v13 {
        Adt3::C0(_, _, _, _, _, _, _, h3073) => {
            h3073
        }
    })?, f116(v0, v3)?))
}

pub fn f120(v0: u64, v1: Adt3, v2: Adt3, v3: u64, v4: u64, v5: u64, v6: u64, v7: bool) -> R<Adt9> {
    let v8: u64 = f60(v0, v2.clone(), 0u64);
    let v9: u64 = f60(v0, v2.clone(), 1u64);
    let v10: u64 = f60(v0, v2.clone(), 2u64);
    let v11: u64 = f60(v0, v2.clone(), 3u64);
    if v7 {
        Ok(Adt9::C1(false, f59(v1.clone(), f157(v8, v3, match v1.clone() {
            Adt3::C0(h3074, _, _, _, _, _, _, _) => {
                h3074
            }
        })?, f157(v9, v4, match v1.clone() {
            Adt3::C0(_, h3075, _, _, _, _, _, _) => {
                h3075
            }
        })?, f157(v10, v5, match v1.clone() {
            Adt3::C0(_, _, h3076, _, _, _, _, _) => {
                h3076
            }
        })?, f157(v11, v6, match v1 {
            Adt3::C0(_, _, _, h3077, _, _, _, _) => {
                h3077
            }
        })?)?, f116(v0, v2)?))
    } else {
        if if if if f158(v8, v3, match v1.clone() {
            Adt3::C0(h3078, _, _, _, _, _, _, _) => {
                h3078
            }
        })? {
            f158(v9, v4, match v1.clone() {
                Adt3::C0(_, h3079, _, _, _, _, _, _) => {
                    h3079
                }
            })?
        } else {
            false
        } {
            f158(v10, v5, match v1.clone() {
                Adt3::C0(_, _, h3080, _, _, _, _, _) => {
                    h3080
                }
            })?
        } else {
            false
        } {
            f158(v11, v6, match v1.clone() {
                Adt3::C0(_, _, _, h3081, _, _, _, _) => {
                    h3081
                }
            })?
        } else {
            false
        } {
            Ok(Adt9::C1(false, f59(v1, f159(v8, v3)?, f159(v9, v4)?, f159(v10, v5)?, f159(v11, v6)?)?, f116(v0, v2)?))
        } else {
            Ok(Adt9::C1(true, 0u64, f116(v0, v2)?))
        }
    }
}

pub fn f121(v0: u64, v1: Adt3, v2: Adt3, v3: u64) -> R<Adt9> {
    let v4: u64 = f60(v0, v2.clone(), 0u64);
    let v5: u64 = f60(v0, v2.clone(), 1u64);
    let v6: u64 = f60(v0, v2.clone(), 2u64);
    let v7: u64 = f60(v0, v2.clone(), 3u64);
    Ok(Adt9::C1(false, f59(v1.clone(), f160(v4, v3, match v1 {
        Adt3::C0(h3082, _, _, _, _, _, _, _) => {
            h3082
        }
    })?, v5, v6, v7)?, f116(v0, v2)?))
}

pub fn f122(v0: u64, v1: Adt3, v2: Adt3, v3: u64, v4: u64, v5: bool) -> R<Adt9> {
    let a3083 = v0;
    let a3084 = v3;
    if nat_lt(a3083, a3084) {
        Ok(Adt9::C1(false, {
            let a3085 = v0;
            let a3086 = 4u64;
            nat_mul(a3085, a3086)?
        }, {
            let a3087 = v0;
            let a3088 = 4u64;
            nat_mul(a3087, a3088)?
        }))
    } else {
        let v6: u64 = {
            let a3089 = v0;
            let a3090 = v3;
            nat_sub(a3089, a3090)
        };
        let v7: u64 = f116(v6, v1)?;
        let v8: u64 = f161(v6, v2, v4)?;
        if v5 {
            Ok(Adt9::C4(8u64, 0u64, v8, 4u64, v7, 3u64, v8))
        } else {
            Ok(Adt9::C1(true, v7, v8))
        }
    }
}

pub fn f123(v0: u64, v1: Adt3, v2: Adt3, v3: u64) -> R<Adt9> {
    let a3091 = v0;
    let a3092 = v3;
    if nat_lt(a3091, a3092) {
        Ok(Adt9::C1(true, 0u64, f116(v0, v2)?))
    } else {
        let v4: u64 = {
            let a3093 = v0;
            let a3094 = v3;
            nat_sub(a3093, a3094)
        };
        let v5: u64 = f118(v4, v2, v1.clone())?;
        Ok(Adt9::C4(8u64, 3u64, v5, 0u64, f116(v4, v1)?, 3u64, v5))
    }
}

pub fn f124(v0: u64, v1: u64, v2: Adt3, v3: Adt3) -> R<Adt9> {
    let v4: u64 = {
        let a3096 = v1;
        let a3097 = match v3.clone() {
            Adt3::C0(h3095, _, _, _, _, _, _, _) => {
                h3095
            }
        };
        let a3098 = 0u64;
        nat_rem(a3096, a3097, a3098)
    };
    let v5: u64 = {
        let a3104 = {
            let a3100 = v1;
            let a3101 = match v3.clone() {
                Adt3::C0(h3099, _, _, _, _, _, _, _) => {
                    h3099
                }
            };
            let a3102 = 0u64;
            nat_quot(a3100, a3101, a3102)
        };
        let a3105 = match v3.clone() {
            Adt3::C0(_, h3103, _, _, _, _, _, _) => {
                h3103
            }
        };
        let a3106 = 0u64;
        nat_rem(a3104, a3105, a3106)
    };
    let v6: bool = f162(v0, v4, v5);
    Ok(Adt9::C1(!v6, if v6 {
        f118(v1, v2, v3.clone())?
    } else {
        0u64
    }, f116(v1, v3)?))
}

pub fn f125(v0: u64, v1: Adt3, v2: Adt3, v3: u64, v4: bool, v5: Adt3) -> R<Adt9> {
    Ok(Adt9::C4({
        let a3107 = v3;
        let a3108 = 8u64;
        nat_add(a3107, a3108)?
    }, 0u64, f116(v0, v1)?, if v4 {
        5u64
    } else {
        4u64
    }, if v4 {
        0u64
    } else {
        f118(v0, v5, v2.clone())?
    }, 3u64, f116(v0, v2)?))
}

pub fn f126(v0: u64, v1: Adt3, v2: Adt3, v3: bool, v4: u64, v5: u64) -> R<Adt9> {
    let v6: u64 = {
        let a3110 = v0;
        let a3111 = match v2.clone() {
            Adt3::C0(h3109, _, _, _, _, _, _, _) => {
                h3109
            }
        };
        let a3112 = 0u64;
        nat_rem(a3110, a3111, a3112)
    };
    let v7: u64 = {
        let a3114 = v0;
        let a3115 = match v2.clone() {
            Adt3::C0(h3113, _, _, _, _, _, _, _) => {
                h3113
            }
        };
        let a3116 = 0u64;
        nat_quot(a3114, a3115, a3116)
    };
    let v8: u64 = {
        let a3118 = v7;
        let a3119 = match v2.clone() {
            Adt3::C0(_, h3117, _, _, _, _, _, _) => {
                h3117
            }
        };
        let a3120 = 0u64;
        nat_rem(a3118, a3119, a3120)
    };
    let v9: u64 = {
        let a3122 = v7;
        let a3123 = match v2.clone() {
            Adt3::C0(_, h3121, _, _, _, _, _, _) => {
                h3121
            }
        };
        let a3124 = 0u64;
        nat_quot(a3122, a3123, a3124)
    };
    let v10: u64 = {
        let a3126 = v9;
        let a3127 = match v2.clone() {
            Adt3::C0(_, _, h3125, _, _, _, _, _) => {
                h3125
            }
        };
        let a3128 = 0u64;
        nat_rem(a3126, a3127, a3128)
    };
    let v11: u64 = {
        let a3130 = v9;
        let a3131 = match v2.clone() {
            Adt3::C0(_, _, h3129, _, _, _, _, _) => {
                h3129
            }
        };
        let a3132 = 0u64;
        nat_quot(a3130, a3131, a3132)
    };
    let v12: u64 = f116(v0, v2)?;
    if v3 {
        let v13: u64 = {
            let a3141 = {
                let a3136 = {
                    let a3133 = v10;
                    let a3134 = v4;
                    let a3135 = 0u64;
                    nat_quot(a3133, a3134, a3135)
                };
                let a3137 = v5;
                nat_mul(a3136, a3137)?
            };
            let a3142 = {
                let a3138 = v8;
                let a3139 = v4;
                let a3140 = 0u64;
                nat_quot(a3138, a3139, a3140)
            };
            nat_add(a3141, a3142)?
        };
        let v14: u64 = {
            let a3162 = v6;
            let a3163 = {
                let a3160 = match v1.clone() {
                    Adt3::C0(h3143, _, _, _, _, _, _, _) => {
                        h3143
                    }
                };
                let a3161 = {
                    let a3158 = {
                        let a3144 = v8;
                        let a3145 = v4;
                        let a3146 = 0u64;
                        nat_rem(a3144, a3145, a3146)
                    };
                    let a3159 = {
                        let a3156 = match v1.clone() {
                            Adt3::C0(_, h3147, _, _, _, _, _, _) => {
                                h3147
                            }
                        };
                        let a3157 = {
                            let a3154 = {
                                let a3148 = v10;
                                let a3149 = v4;
                                let a3150 = 0u64;
                                nat_rem(a3148, a3149, a3150)
                            };
                            let a3155 = {
                                let a3152 = match v1.clone() {
                                    Adt3::C0(_, _, h3151, _, _, _, _, _) => {
                                        h3151
                                    }
                                };
                                let a3153 = v13;
                                nat_mul(a3152, a3153)?
                            };
                            nat_add(a3154, a3155)?
                        };
                        nat_mul(a3156, a3157)?
                    };
                    nat_add(a3158, a3159)?
                };
                nat_mul(a3160, a3161)?
            };
            nat_add(a3162, a3163)?
        };
        Ok(Adt9::C1(false, f116(v14, v1)?, v12))
    } else {
        let v15: u64 = {
            let a3169 = {
                let a3167 = {
                    let a3164 = v11;
                    let a3165 = v5;
                    let a3166 = 0u64;
                    nat_rem(a3164, a3165, a3166)
                };
                let a3168 = v4;
                nat_mul(a3167, a3168)?
            };
            let a3170 = v8;
            nat_add(a3169, a3170)?
        };
        let v16: u64 = {
            let a3176 = {
                let a3174 = {
                    let a3171 = v11;
                    let a3172 = v5;
                    let a3173 = 0u64;
                    nat_quot(a3171, a3172, a3173)
                };
                let a3175 = v4;
                nat_mul(a3174, a3175)?
            };
            let a3177 = v10;
            nat_add(a3176, a3177)?
        };
        let a3179 = v15;
        let a3180 = match v1.clone() {
            Adt3::C0(_, h3178, _, _, _, _, _, _) => {
                h3178
            }
        };
        if if nat_lt(a3179, a3180) {
            let a3182 = v16;
            let a3183 = match v1.clone() {
                Adt3::C0(_, _, h3181, _, _, _, _, _) => {
                    h3181
                }
            };
            nat_lt(a3182, a3183)
        } else {
            false
        } {
            Ok(Adt9::C1(false, f116({
                let a3192 = v6;
                let a3193 = {
                    let a3190 = match v1.clone() {
                        Adt3::C0(h3184, _, _, _, _, _, _, _) => {
                            h3184
                        }
                    };
                    let a3191 = {
                        let a3188 = v15;
                        let a3189 = {
                            let a3186 = match v1.clone() {
                                Adt3::C0(_, h3185, _, _, _, _, _, _) => {
                                    h3185
                                }
                            };
                            let a3187 = v16;
                            nat_mul(a3186, a3187)?
                        };
                        nat_add(a3188, a3189)?
                    };
                    nat_mul(a3190, a3191)?
                };
                nat_add(a3192, a3193)?
            }, v1)?, v12))
        } else {
            Ok(Adt9::C1(true, 0u64, v12))
        }
    }
}

pub fn f127(v0: u64, v1: Adt3, v2: Adt3) -> R<Adt9> {
    let v3: u64 = f116(v0, v2.clone())?;
    let v4: u64 = f116(v0, v1.clone())?;
    let a3198 = {
        let a3195 = v0;
        let a3196 = match v1 {
            Adt3::C0(h3194, _, _, _, _, _, _, _) => {
                h3194
            }
        };
        let a3197 = 0u64;
        nat_rem(a3195, a3196, a3197)
    };
    let a3199 = 0u64;
    if nat_eq(a3198, a3199) {
        Ok(Adt9::C1(false, v4, v3))
    } else {
        Ok(Adt9::C4(8u64, 3u64, f116({
            let a3200 = v0;
            let a3201 = 1u64;
            nat_sub(a3200, a3201)
        }, v2)?, 0u64, v4, 3u64, v3))
    }
}

pub fn f128(v0: u64, v1: Adt3, v2: Adt3, v3: u64, v4: u64) -> R<Adt9> {
    let a3202 = v3;
    let a3203 = 13u64;
    if nat_eq(a3202, a3203) {
        f163(v0, v1, v4)
    } else {
        let v5: u64 = f152(match v1.clone() {
            Adt3::C0(h3204, _, _, _, _, _, _, _) => {
                h3204
            }
        }, {
            let a3205 = v3;
            let a3206 = 16u64;
            nat_eq(a3205, a3206)
        })?;
        let v6: u64 = {
            let a3207 = v0;
            let a3208 = v5;
            let a3209 = 0u64;
            nat_quot(a3207, a3208, a3209)
        };
        let v7: u64 = {
            let a3210 = v0;
            let a3211 = v5;
            let a3212 = 0u64;
            nat_rem(a3210, a3211, a3212)
        };
        let v8: u64 = {
            let a3216 = 1u64;
            let a3217 = {
                let a3214 = match v1.clone() {
                    Adt3::C0(h3213, _, _, _, _, _, _, _) => {
                        h3213
                    }
                };
                let a3215 = 2u64;
                nat_mul(a3214, a3215)?
            };
            nat_add(a3216, a3217)?
        };
        let a3218 = v7;
        let a3219 = v8;
        if nat_lt(a3218, a3219) {
            f164(v7, v6, v1, 0u64)
        } else {
            let v9: u64 = f116(v6, v2)?;
            let a3220 = v3;
            let a3221 = 14u64;
            if nat_eq(a3220, a3221) {
                Ok(Adt9::C4(2u64, 1u64, 1u64, 2u64, 0u64, 3u64, v9))
            } else {
                let a3222 = v7;
                let a3223 = v8;
                if nat_eq(a3222, a3223) {
                    Ok(Adt9::C4(2u64, 1u64, 1u64, 2u64, 0u64, 1u64, 1u64))
                } else {
                    let a3226 = v7;
                    let a3227 = {
                        let a3224 = v8;
                        let a3225 = 1u64;
                        nat_add(a3224, a3225)?
                    };
                    if nat_eq(a3226, a3227) {
                        Ok(Adt9::C4(3u64, 2u64, match v1 {
                            Adt3::C0(h3228, _, _, _, _, _, _, _) => {
                                h3228
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

pub fn f129(v0: u64, v1: Adt3, v2: Adt3, v3: u64) -> R<Adt9> {
    let a3229 = v0;
    let a3230 = 0u64;
    if nat_eq(a3229, a3230) {
        Ok(Adt9::C4(19u64, 2u64, 0u64, 2u64, 0u64, 1u64, 0u64))
    } else {
        let a3237 = {
            let a3231 = v0;
            let a3232 = 1u64;
            nat_add(a3231, a3232)?
        };
        let a3238 = {
            let a3235 = 2u64;
            let a3236 = {
                let a3233 = v3;
                let a3234 = 2u64;
                nat_mul(a3233, a3234)?
            };
            nat_add(a3235, a3236)?
        };
        if nat_eq(a3237, a3238) {
            Ok(Adt9::C4(19u64, 1u64, 0u64, 2u64, 0u64, 6u64, 0u64))
        } else {
            let v4: u64 = {
                let a3239 = v0;
                let a3240 = 1u64;
                nat_sub(a3239, a3240)
            };
            let v5: u64 = {
                let a3241 = v4;
                let a3242 = 2u64;
                let a3243 = 0u64;
                nat_quot(a3241, a3242, a3243)
            };
            let a3247 = {
                let a3244 = v4;
                let a3245 = 2u64;
                let a3246 = 0u64;
                nat_rem(a3244, a3245, a3246)
            };
            let a3248 = 0u64;
            if nat_eq(a3247, a3248) {
                Ok(Adt9::C4(17u64, 0u64, f116(v5, v1)?, 4u64, f116(v5, v2)?, 1u64, 2u64))
            } else {
                Ok(Adt9::C4(18u64, 1u64, 0u64, 1u64, 2u64, 1u64, 0u64))
            }
        }
    }
}

pub fn f130(v0: u64, v1: u64, v2: u64, v3: u64) -> R<Adt9> {
    let a3249 = v0;
    let a3250 = f22(v1, v2, v3)?;
    if nat_lt(a3249, a3250) {
        let a3251 = v0;
        let a3252 = v1;
        if nat_lt(a3251, a3252) {
            Ok(Adt9::C1(false, {
                let a3253 = 4u64;
                let a3254 = v0;
                nat_mul(a3253, a3254)?
            }, {
                let a3255 = 4u64;
                let a3256 = v0;
                nat_mul(a3255, a3256)?
            }))
        } else {
            let v4: u64 = {
                let a3257 = v0;
                let a3258 = v1;
                nat_sub(a3257, a3258)
            };
            let v5: u64 = f165(v4, v2)?;
            let a3259 = v5;
            let a3260 = 2u64;
            if nat_lt(a3259, a3260) {
                Ok(Adt9::C4(6u64, {
                    let a3261 = v5;
                    let a3262 = 0u64;
                    if nat_eq(a3261, a3262) {
                        4u64
                    } else {
                        5u64
                    }
                }, f166(v4, v2)?, 2u64, 0u64, 1u64, v5))
            } else {
                let v6: u64 = f167(v4, v2)?;
                Ok(Adt9::C4(8u64, 3u64, v6, 1u64, if f168(v4, v2)? {
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

pub fn f131(v0: Adt3, v1: Adt3) -> R<u64> {
    let a3270 = {
        let a3263 = 2u64;
        let a3264 = f105(v1.clone())?;
        nat_mul(a3263, a3264)?
    };
    let a3271 = {
        let a3268 = f105(v1)?;
        let a3269 = {
            let a3266 = 1u64;
            let a3267 = match v0 {
                Adt3::C0(h3265, _, _, _, _, _, _, _) => {
                    h3265
                }
            };
            nat_add(a3266, a3267)?
        };
        nat_mul(a3268, a3269)?
    };
    nat_add(a3270, a3271)
}

pub fn f132(v0: u64, v1: Adt3) -> R<Adt9> {
    Ok(Adt9::C4(19u64, 4u64, f116(v0, v1)?, 2u64, 0u64, 1u64, 2u64))
}

pub fn f133(v0: u64, v1: u64) -> bool {
    let a3272 = v1;
    let a3273 = 2147483647u64;
    if nat_le(a3272, a3273) {
        let a3274 = v1;
        let a3275 = v0;
        nat_lt(a3274, a3275)
    } else {
        false
    }
}

pub fn f134(v0: u64, v1: u64, v2: u64, v3: Adt3, v4: Adt3, v5: Adt3, v6: u64) -> R<Adt9> {
    let m3276 = f94(v0);
    match m3276 {
        Adt14::C0 => {
            Ok(Adt9::C0(Adt10::C1))
        }
        Adt14::C1(v7) => {
            let v8: u64 = {
                let a3278 = v2;
                let a3279 = match v7.clone() {
                    Adt15::C0(_, h3277, _) => {
                        h3277
                    }
                };
                nat_mul(a3278, a3279)?
            };
            let m3292 = f169(v0, v8, v6, v1, match v5.clone() {
                Adt3::C0(h3280, _, _, _, _, _, _, _) => {
                    h3280
                }
            }, match v5.clone() {
                Adt3::C0(_, h3281, _, _, _, _, _, _) => {
                    h3281
                }
            }, match v5 {
                Adt3::C0(_, _, h3282, _, _, _, _, _) => {
                    h3282
                }
            }, {
                let a3285 = match v3.clone() {
                    Adt3::C0(h3283, _, _, _, _, _, _, _) => {
                        h3283
                    }
                };
                let a3286 = match v7.clone() {
                    Adt15::C0(_, h3284, _) => {
                        h3284
                    }
                };
                nat_mul(a3285, a3286)?
            }, match v3.clone() {
                Adt3::C0(_, h3287, _, _, _, _, _, _) => {
                    h3287
                }
            }, match v3.clone() {
                Adt3::C0(_, _, _, _, h3288, _, _, _) => {
                    h3288
                }
            }, match v3.clone() {
                Adt3::C0(_, _, _, _, _, h3289, _, _) => {
                    h3289
                }
            }, match v3.clone() {
                Adt3::C0(_, _, _, _, _, _, h3290, _) => {
                    h3290
                }
            }, match v3 {
                Adt3::C0(_, _, _, _, _, _, _, h3291) => {
                    h3291
                }
            })?;
            match m3292 {
                None => {
                    Ok(Adt9::C0(Adt10::C1))
                }
                Some(v9) => {
                    let a3293 = v0;
                    let a3294 = 0u64;
                    if if nat_eq(a3293, a3294) {
                        true
                    } else {
                        let a3295 = v0;
                        let a3296 = 26u64;
                        nat_eq(a3295, a3296)
                    } {
                        Ok(Adt9::C1(false, v9, f116({
                            let a3300 = {
                                let a3298 = v1;
                                let a3299 = match v4.clone() {
                                    Adt3::C0(h3297, _, _, _, _, _, _, _) => {
                                        h3297
                                    }
                                };
                                nat_mul(a3298, a3299)?
                            };
                            let a3301 = v8;
                            nat_add(a3300, a3301)?
                        }, v4)?))
                    } else {
                        Ok(Adt9::C5(v0, v9, match v7.clone() {
                            Adt15::C0(_, _, h3302) => {
                                h3302
                            }
                        }, f116({
                            let a3306 = {
                                let a3304 = v1;
                                let a3305 = match v4.clone() {
                                    Adt3::C0(h3303, _, _, _, _, _, _, _) => {
                                        h3303
                                    }
                                };
                                nat_mul(a3304, a3305)?
                            };
                            let a3307 = v8;
                            nat_add(a3306, a3307)?
                        }, v4.clone())?, {
                            let a3314 = {
                                let a3312 = {
                                    let a3309 = match v7 {
                                        Adt15::C0(_, h3308, _) => {
                                            h3308
                                        }
                                    };
                                    let a3310 = 1u64;
                                    nat_sub(a3309, a3310)
                                };
                                let a3313 = match v4.clone() {
                                    Adt3::C0(_, _, _, _, h3311, _, _, _) => {
                                        h3311
                                    }
                                };
                                nat_mul(a3312, a3313)?
                            };
                            let a3315 = 4u64;
                            nat_add(a3314, a3315)?
                        }, match v4 {
                            Adt3::C0(_, _, _, _, h3316, _, _, _) => {
                                h3316
                            }
                        }))
                    }
                }
            }
        }
    }
}

pub fn f135(v0: u64, v1: u64, v2: u64) -> u64 {
    let a3317 = v1;
    let a3318 = 0u64;
    if if nat_eq(a3317, a3318) {
        true
    } else {
        let a3319 = v2;
        let a3320 = 0u64;
        nat_eq(a3319, a3320)
    } {
        0u64
    } else {
        v0
    }
}

pub fn f136(v0: u64, v1: u64) -> bool {
    let a3321 = v0;
    let a3322 = 4294967295u64;
    if nat_eq(a3321, a3322) {
        true
    } else {
        let a3323 = v0;
        let a3324 = 2147483648u64;
        if nat_lt(a3323, a3324) {
            let a3325 = v0;
            let a3326 = v1;
            nat_lt(a3325, a3326)
        } else {
            false
        }
    }
}

pub fn f137(v0: Adt7) -> R<u64> {
    let a3331 = if match v0.clone() {
        Adt7::C0(_, _, _, h3327, _) => {
            h3327
        }
    } {
        2u64
    } else {
        1u64
    };
    let a3332 = {
        let a3329 = match v0 {
            Adt7::C0(_, _, _, _, h3328) => {
                h3328
            }
        };
        let a3330 = 1u64;
        if nat_eq(a3329, a3330) {
            1u64
        } else {
            0u64
        }
    };
    nat_add(a3331, a3332)
}

pub fn f138(v0: Adt6, v1: u64, v2: u64) -> R<u64> {
    let v3: u64 = {
        let a3337 = {
            let a3334 = v2;
            let a3335 = match v0.clone() {
                Adt6::C0(_, h3333, _, _) => {
                    h3333
                }
            };
            nat_mul(a3334, a3335)?
        };
        let a3338 = f109(match v0.clone() {
            Adt6::C0(_, _, h3336, _) => {
                h3336
            }
        });
        nat_add(a3337, a3338)?
    };
    let v4: u64 = f110(match v0.clone() {
        Adt6::C0(_, _, h3339, _) => {
            h3339
        }
    });
    let v5: u64 = {
        let a3343 = {
            let a3341 = v3;
            let a3342 = match v0 {
                Adt6::C0(h3340, _, _, _) => {
                    h3340
                }
            };
            nat_add(a3341, a3342)?
        };
        let a3344 = v4;
        nat_sub(a3343, a3344)
    };
    let a3349 = {
        let a3345 = v5;
        let a3346 = v1;
        if nat_lt(a3345, a3346) {
            v5
        } else {
            v1
        }
    };
    let a3350 = {
        let a3347 = v3;
        let a3348 = v4;
        nat_sub(a3347, a3348)
    };
    Ok(nat_sub(a3349, a3350))
}

pub fn f139(v0: Adt7, v1: Adt3, v2: Adt3, v3: u64, v4: u64) -> R<Option<u64>> {
    let m3359 = f62(match v0.clone() {
        Adt7::C0(h3351, _, _, _, _) => {
            h3351
        }
    }, match v1.clone() {
        Adt3::C0(h3352, _, _, _, _, _, _, _) => {
            h3352
        }
    }, f60(v3, v2.clone(), 0u64), {
        let a3356 = v4;
        let a3357 = {
            let m3355 = match v0.clone() {
                Adt7::C0(h3354, _, _, _, _) => {
                    h3354
                }
            };
            match m3355 {
                Adt6::C0(h3353, _, _, _) => {
                    h3353
                }
            }
        };
        let a3358 = 0u64;
        nat_rem(a3356, a3357, a3358)
    })?;
    match m3359 {
        None => {
            Ok(None::<u64>)
        }
        Some(v5) => {
            let m3368 = f62(match v0.clone() {
                Adt7::C0(_, h3360, _, _, _) => {
                    h3360
                }
            }, match v1.clone() {
                Adt3::C0(_, h3361, _, _, _, _, _, _) => {
                    h3361
                }
            }, f60(v3, v2.clone(), 1u64), {
                let a3365 = v4;
                let a3366 = {
                    let m3364 = match v0 {
                        Adt7::C0(h3363, _, _, _, _) => {
                            h3363
                        }
                    };
                    match m3364 {
                        Adt6::C0(h3362, _, _, _) => {
                            h3362
                        }
                    }
                };
                let a3367 = 0u64;
                nat_quot(a3365, a3366, a3367)
            })?;
            match m3368 {
                None => {
                    Ok(None::<u64>)
                }
                Some(v6) => {
                    Ok(Some(f59(v1, v5, v6, f60(v3, v2.clone(), 2u64), f60(v3, v2, 3u64))?))
                }
            }
        }
    }
}

pub fn f140(v0: u64, v1: Adt3, v2: u64, v3: u64, v4: u64) -> R<Adt9> {
    let v5: u64 = {
        let a3375 = v3;
        let a3376 = {
            let a3372 = f116({
                let a3369 = v0;
                let a3370 = 2u64;
                let a3371 = 0u64;
                nat_quot(a3369, a3370, a3371)
            }, v1)?;
            let a3373 = v2;
            let a3374 = 0u64;
            nat_quot(a3372, a3373, a3374)
        };
        nat_add(a3375, a3376)?
    };
    let a3380 = {
        let a3377 = v0;
        let a3378 = 2u64;
        let a3379 = 0u64;
        nat_rem(a3377, a3378, a3379)
    };
    let a3381 = 0u64;
    if nat_eq(a3380, a3381) {
        Ok(Adt9::C4(19u64, 1u64, v5, 2u64, 0u64, 1u64, 2u64))
    } else {
        let a3382 = v4;
        let a3383 = 0u64;
        if nat_eq(a3382, a3383) {
            Ok(Adt9::C4(19u64, 2u64, 1u64, 2u64, 0u64, 1u64, v5))
        } else {
            Ok(Adt9::C0(Adt10::C1))
        }
    }
}

pub fn f141(v0: u64, v1: u64) -> R<u64> {
    let a3386 = 8u64;
    let a3387 = {
        let a3384 = f170(v0);
        let a3385 = v1;
        nat_mul(a3384, a3385)?
    };
    nat_add(a3386, a3387)
}

pub fn f142(v0: u64, v1: u64, v2: Adt3, v3: u64, v4: u64, v5: u64) -> R<Adt9> {
    let v6: u64 = match v2.clone() {
        Adt3::C0(h3388, _, _, _, _, _, _, _) => {
            h3388
        }
    };
    let v7: u64 = f170(v3);
    let v8: u64 = {
        let a3391 = 1u64;
        let a3392 = {
            let a3389 = v6;
            let a3390 = v7;
            nat_mul(a3389, a3390)?
        };
        nat_add(a3391, a3392)?
    };
    let a3393 = v0;
    let a3394 = 0u64;
    if nat_eq(a3393, a3394) {
        Ok(f171(0u64))
    } else {
        let a3395 = v0;
        let a3396 = v8;
        if nat_lt(a3395, a3396) {
            let v9: u64 = {
                let a3397 = v0;
                let a3398 = 1u64;
                nat_sub(a3397, a3398)
            };
            let v10: u64 = {
                let a3399 = v9;
                let a3400 = v7;
                let a3401 = 0u64;
                nat_rem(a3399, a3400, a3401)
            };
            let v11: u64 = f116({
                let a3407 = {
                    let a3402 = v1;
                    let a3403 = v6;
                    nat_mul(a3402, a3403)?
                };
                let a3408 = {
                    let a3404 = v9;
                    let a3405 = v7;
                    let a3406 = 0u64;
                    nat_quot(a3404, a3405, a3406)
                };
                nat_add(a3407, a3408)?
            }, v2)?;
            let a3409 = v3;
            let a3410 = 30u64;
            if nat_eq(a3409, a3410) {
                let a3411 = v10;
                let a3412 = 0u64;
                if nat_eq(a3411, a3412) {
                    Ok(Adt9::C4(6u64, 0u64, v11, 2u64, 0u64, 1u64, 1u64))
                } else {
                    let a3413 = v10;
                    let a3414 = 1u64;
                    if nat_eq(a3413, a3414) {
                        Ok(Adt9::C4(10u64, 1u64, 1u64, 1u64, 1u64, 1u64, 1u64))
                    } else {
                        let a3415 = v10;
                        let a3416 = 2u64;
                        if nat_eq(a3415, a3416) {
                            Ok(Adt9::C4(1u64, 1u64, 1u64, 2u64, 0u64, 1u64, 1u64))
                        } else {
                            Ok(Adt9::C4(0u64, 1u64, 0u64, 1u64, 1u64, 1u64, 0u64))
                        }
                    }
                }
            } else {
                let a3417 = v10;
                let a3418 = 0u64;
                if nat_eq(a3417, a3418) {
                    Ok(Adt9::C4(10u64, 0u64, v11, 0u64, v11, 1u64, 1u64))
                } else {
                    let a3419 = v10;
                    let a3420 = 1u64;
                    if nat_eq(a3419, a3420) {
                        Ok(Adt9::C4(1u64, 1u64, 1u64, 2u64, 0u64, 1u64, 1u64))
                    } else {
                        Ok(Adt9::C4(0u64, 1u64, 0u64, 1u64, 1u64, 1u64, 0u64))
                    }
                }
            }
        } else {
            let v12: u64 = {
                let a3421 = v0;
                let a3422 = v8;
                nat_sub(a3421, a3422)
            };
            let a3423 = v12;
            let a3424 = 0u64;
            if nat_eq(a3423, a3424) {
                Ok(Adt9::C4(22u64, 2u64, v6, 2u64, 0u64, 1u64, 1u64))
            } else {
                let a3425 = v12;
                let a3426 = 1u64;
                if nat_eq(a3425, a3426) {
                    Ok(Adt9::C4(21u64, 1u64, 0u64, 1u64, 1u64, 1u64, 1u64))
                } else {
                    let a3427 = v12;
                    let a3428 = 2u64;
                    if nat_eq(a3427, a3428) {
                        Ok(Adt9::C4(2u64, 1u64, 1u64, 2u64, 0u64, 1u64, 1u64))
                    } else {
                        let a3429 = v12;
                        let a3430 = 3u64;
                        if nat_eq(a3429, a3430) {
                            Ok(Adt9::C4(4u64, 1u64, 1u64, 2u64, v4, 1u64, 1u64))
                        } else {
                            let a3431 = v12;
                            let a3432 = 4u64;
                            if nat_eq(a3431, a3432) {
                                Ok(Adt9::C4(12u64, 1u64, 1u64, 2u64, 0u64, 1u64, 1u64))
                            } else {
                                let a3433 = v12;
                                let a3434 = 5u64;
                                if nat_eq(a3433, a3434) {
                                    Ok(Adt9::C4(11u64, 2u64, 1065353216u64, 1u64, 1u64, 1u64, 2u64))
                                } else {
                                    let a3435 = v12;
                                    let a3436 = 6u64;
                                    if nat_eq(a3435, a3436) {
                                        if f172(v5) {
                                            Ok(Adt9::C4(19u64, 1u64, 2u64, 2u64, 0u64, 1u64, {
                                                let a3437 = 3u64;
                                                let a3438 = v1;
                                                nat_add(a3437, a3438)?
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

pub fn f143(v0: u64, v1: Adt3, v2: Adt3, v3: u64) -> R<Adt9> {
    let v4: u64 = {
        let a3439 = v0;
        let a3440 = f85(v3);
        let a3441 = 0u64;
        nat_quot(a3439, a3440, a3441)
    };
    let v5: u64 = {
        let a3446 = 3u64;
        let a3447 = {
            let a3443 = v4;
            let a3444 = match v1.clone() {
                Adt3::C0(h3442, _, _, _, _, _, _, _) => {
                    h3442
                }
            };
            let a3445 = 0u64;
            nat_quot(a3443, a3444, a3445)
        };
        nat_add(a3446, a3447)?
    };
    let v6: u64 = f116(v4, v1)?;
    let v7: u64 = f116(v4, v2)?;
    let a3448 = v3;
    let a3449 = 30u64;
    if nat_eq(a3448, a3449) {
        let a3453 = {
            let a3450 = v0;
            let a3451 = 3u64;
            let a3452 = 0u64;
            nat_rem(a3450, a3451, a3452)
        };
        let a3454 = 0u64;
        if nat_eq(a3453, a3454) {
            Ok(Adt9::C4(6u64, 0u64, v6, 2u64, 0u64, 1u64, 1u64))
        } else {
            let a3458 = {
                let a3455 = v0;
                let a3456 = 3u64;
                let a3457 = 0u64;
                nat_rem(a3455, a3456, a3457)
            };
            let a3459 = 1u64;
            if nat_eq(a3458, a3459) {
                Ok(Adt9::C4(10u64, 1u64, 1u64, 1u64, v5, 1u64, 1u64))
            } else {
                Ok(Adt9::C4(7u64, 1u64, 1u64, 2u64, 0u64, 3u64, v7))
            }
        }
    } else {
        Ok(Adt9::C4(10u64, 0u64, v6, 1u64, v5, 3u64, v7))
    }
}

pub fn f144(v0: u64) -> R<u64> {
    let v1: u64 = {
        let a3460 = v0;
        let a3461 = 8388608u64;
        let a3462 = 0u64;
        nat_quot(a3460, a3461, a3462)
    };
    let a3463 = v0;
    let a3464 = 2147483648u64;
    if if if nat_lt(a3463, a3464) {
        let a3465 = 127u64;
        let a3466 = v1;
        nat_le(a3465, a3466)
    } else {
        false
    } {
        let a3467 = v1;
        let a3468 = 190u64;
        nat_lt(a3467, a3468)
    } else {
        false
    } {
        let v2: u64 = {
            let a3472 = 8388608u64;
            let a3473 = {
                let a3469 = v0;
                let a3470 = 8388608u64;
                let a3471 = 0u64;
                nat_rem(a3469, a3470, a3471)
            };
            nat_add(a3472, a3473)?
        };
        let a3474 = v1;
        let a3475 = 150u64;
        if nat_lt(a3474, a3475) {
            let v3: u64 = f173({
                let a3476 = 150u64;
                let a3477 = v1;
                nat_sub(a3476, a3477)
            });
            let a3481 = {
                let a3478 = v2;
                let a3479 = v3;
                let a3480 = 0u64;
                nat_rem(a3478, a3479, a3480)
            };
            let a3482 = 0u64;
            if nat_eq(a3481, a3482) {
                let a3483 = v2;
                let a3484 = v3;
                let a3485 = 0u64;
                Ok(nat_quot(a3483, a3484, a3485))
            } else {
                Ok(0u64)
            }
        } else {
            let a3488 = f173({
                let a3486 = v1;
                let a3487 = 150u64;
                nat_sub(a3486, a3487)
            });
            let a3489 = v2;
            nat_mul(a3488, a3489)
        }
    } else {
        Ok(0u64)
    }
}

pub fn f145(v0: u64, v1: u64, v2: u64) -> bool {
    let a3490 = v0;
    let a3491 = 1u64;
    if nat_le(a3490, a3491) {
        true
    } else {
        let a3495 = {
            let a3492 = v1;
            let a3493 = v2;
            let a3494 = 1u64;
            nat_rem(a3492, a3493, a3494)
        };
        let a3496 = 0u64;
        nat_eq(a3495, a3496)
    }
}

pub fn f146(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64) -> R<bool> {
    let a3497 = v1;
    let a3498 = 1u64;
    if if if nat_le(a3497, a3498) {
        true
    } else {
        let a3499 = v0;
        let a3500 = v2;
        nat_le(a3499, a3500)
    } {
        let a3501 = v3;
        let a3502 = 1u64;
        if nat_le(a3501, a3502) {
            true
        } else {
            let a3509 = {
                let a3507 = v0;
                let a3508 = {
                    let a3505 = {
                        let a3503 = v1;
                        let a3504 = 1u64;
                        nat_sub(a3503, a3504)
                    };
                    let a3506 = v2;
                    nat_mul(a3505, a3506)?
                };
                nat_add(a3507, a3508)?
            };
            let a3510 = v4;
            nat_le(a3509, a3510)
        }
    } else {
        false
    } {
        let a3511 = v5;
        let a3512 = 1u64;
        if nat_le(a3511, a3512) {
            Ok(true)
        } else {
            let a3525 = {
                let a3523 = {
                    let a3517 = v0;
                    let a3518 = {
                        let a3515 = {
                            let a3513 = v1;
                            let a3514 = 1u64;
                            nat_sub(a3513, a3514)
                        };
                        let a3516 = v2;
                        nat_mul(a3515, a3516)?
                    };
                    nat_add(a3517, a3518)?
                };
                let a3524 = {
                    let a3521 = {
                        let a3519 = v3;
                        let a3520 = 1u64;
                        nat_sub(a3519, a3520)
                    };
                    let a3522 = v4;
                    nat_mul(a3521, a3522)?
                };
                nat_add(a3523, a3524)?
            };
            let a3526 = v6;
            Ok(nat_le(a3525, a3526))
        }
    } else {
        Ok(false)
    }
}

pub fn f147(v0: u64, v1: u64, _: u64, v3: u64, v4: u64, v5: u64, v6: u64, v7: u64, v8: u64, v9: u64, v10: u64, v11: u64, v12: u64, v13: u64, v14: u64) -> bool {
    if if if if if if if if if f174(v0, v1, v6, v7, v8) {
        let a3527 = v5;
        let a3528 = 26u64;
        nat_eq(a3527, a3528)
    } else {
        false
    } {
        let a3529 = v3;
        let a3530 = v7;
        nat_eq(a3529, a3530)
    } else {
        false
    } {
        let a3531 = v4;
        let a3532 = v8;
        nat_eq(a3531, a3532)
    } else {
        false
    } {
        let a3533 = v9;
        let a3534 = 1u64;
        nat_eq(a3533, a3534)
    } else {
        false
    } {
        let a3535 = v10;
        let a3536 = f175(v0);
        nat_eq(a3535, a3536)
    } else {
        false
    } {
        let a3537 = v11;
        let a3538 = v1;
        nat_eq(a3537, a3538)
    } else {
        false
    } {
        let a3539 = v12;
        let a3540 = v6;
        nat_eq(a3539, a3540)
    } else {
        false
    } {
        let a3541 = v13;
        let a3542 = v7;
        nat_eq(a3541, a3542)
    } else {
        false
    } {
        let a3543 = v14;
        let a3544 = v8;
        nat_eq(a3543, a3544)
    } else {
        false
    }
}

pub fn f148(v0: u64, v1: u64, v2: u64, v3: u64) -> bool {
    let a3545 = v0;
    let a3546 = 0u64;
    if if if nat_eq(a3545, a3546) {
        true
    } else {
        let a3547 = v1;
        let a3548 = 0u64;
        nat_eq(a3547, a3548)
    } {
        true
    } else {
        let a3549 = v2;
        let a3550 = 0u64;
        nat_eq(a3549, a3550)
    } {
        true
    } else {
        let a3551 = v3;
        let a3552 = 0u64;
        nat_eq(a3551, a3552)
    }
}

pub fn f149(v0: Adt3, v1: bool) -> R<u64> {
    f176(v0.clone(), {
        let a3554 = match v0.clone() {
            Adt3::C0(h3553, _, _, _, _, _, _, _) => {
                h3553
            }
        };
        let a3555 = 1u64;
        nat_sub(a3554, a3555)
    }, {
        let a3557 = match v0.clone() {
            Adt3::C0(_, h3556, _, _, _, _, _, _) => {
                h3556
            }
        };
        let a3558 = 1u64;
        nat_sub(a3557, a3558)
    }, {
        let a3560 = match v0.clone() {
            Adt3::C0(_, _, h3559, _, _, _, _, _) => {
                h3559
            }
        };
        let a3561 = 1u64;
        nat_sub(a3560, a3561)
    }, {
        let a3563 = match v0 {
            Adt3::C0(_, _, _, h3562, _, _, _, _) => {
                h3562
            }
        };
        let a3564 = 1u64;
        nat_sub(a3563, a3564)
    }, v1)
}

pub fn f150(v0: u64, v1: u64, v2: u64, v3: u64) -> R<u64> {
    let a3565 = v0;
    let a3566 = 0u64;
    if nat_eq(a3565, a3566) {
        Ok(0u64)
    } else {
        let v4: u64 = {
            let a3567 = v0;
            let a3568 = 1u64;
            nat_sub(a3567, a3568)
        };
        let v5: u64 = {
            let a3569 = v4;
            let a3570 = v1;
            let a3571 = 0u64;
            nat_quot(a3569, a3570, a3571)
        };
        let v6: u64 = {
            let a3579 = {
                let a3572 = v5;
                let a3573 = v2;
                nat_mul(a3572, a3573)?
            };
            let a3580 = {
                let a3577 = {
                    let a3574 = v4;
                    let a3575 = v1;
                    let a3576 = 0u64;
                    nat_rem(a3574, a3575, a3576)
                };
                let a3578 = v3;
                nat_mul(a3577, a3578)?
            };
            nat_add(a3579, a3580)?
        };
        let a3581 = v5;
        let a3582 = 0u64;
        if nat_eq(a3581, a3582) {
            Ok(v6)
        } else {
            let v7: u64 = {
                let a3591 = {
                    let a3585 = {
                        let a3583 = v5;
                        let a3584 = 1u64;
                        nat_sub(a3583, a3584)
                    };
                    let a3586 = v2;
                    nat_mul(a3585, a3586)?
                };
                let a3592 = {
                    let a3589 = {
                        let a3587 = v1;
                        let a3588 = 1u64;
                        nat_sub(a3587, a3588)
                    };
                    let a3590 = v3;
                    nat_mul(a3589, a3590)?
                };
                nat_add(a3591, a3592)?
            };
            let a3593 = v6;
            let a3594 = v7;
            if nat_lt(a3593, a3594) {
                Ok(v7)
            } else {
                Ok(v6)
            }
        }
    }
}

pub fn f151(v0: Adt3, v1: u64) -> bool {
    let a3595 = v1;
    let a3596 = 0u64;
    if !nat_eq(a3595, a3596) {
        true
    } else {
        let a3598 = match v0 {
            Adt3::C0(_, _, _, _, h3597, _, _, _) => {
                h3597
            }
        };
        let a3599 = 4u64;
        nat_eq(a3598, a3599)
    }
}

pub fn f152(v0: u64, v1: bool) -> R<u64> {
    let a3602 = {
        let a3600 = v0;
        let a3601 = 2u64;
        nat_mul(a3600, a3601)?
    };
    let a3603 = if v1 {
        4u64
    } else {
        2u64
    };
    nat_add(a3602, a3603)
}

pub fn f153(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: bool, v6: u64, v7: u64, v8: u64, v9: u64, v10: u64, v11: u64, v12: u64, v13: u64, v14: u64, v15: u64, v16: u64, v17: u64) -> R<Adt10> {
    let m3604 = f177(v0, v1);
    match m3604 {
        None => {
            if v5 {
                let m3605 = f178(v0, v2, v3, v4, v6, v7, v8, v9, v10, v11, v12, v13, v14, v15, v16, v17)?;
                match m3605 {
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
                let a3606 = 4u64;
                let a3607 = v28;
                nat_le(a3606, a3607)
            };
            let a3608 = v2;
            let a3609 = 0u64;
            if if if if if nat_eq(a3608, a3609) {
                let a3610 = v4;
                let a3611 = 0u64;
                nat_eq(a3610, a3611)
            } else {
                false
            } {
                let a3612 = v6;
                let a3613 = v14;
                if nat_eq(a3612, a3613) {
                    let a3614 = v7;
                    let a3615 = v15;
                    nat_eq(a3614, a3615)
                } else {
                    false
                }
            } else {
                false
            } {
                let a3616 = v8;
                let a3617 = v16;
                if nat_eq(a3616, a3617) {
                    let a3618 = v9;
                    let a3619 = v17;
                    nat_eq(a3618, a3619)
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
                        let a3620 = v3;
                        let a3621 = 0u64;
                        nat_eq(a3620, a3621)
                    } else {
                        false
                    } {
                        f95(v6, v7, v8, v9, v10, v11, v12, v13)
                    } else {
                        false
                    }
                }
            } else {
                false
            } {
                Ok(Adt10::C2(v28, v29, v6, v7, v8, v9, {
                    let a3626 = {
                        let a3622 = v6;
                        let a3623 = v7;
                        nat_mul(a3622, a3623)?
                    };
                    let a3627 = {
                        let a3624 = v8;
                        let a3625 = v9;
                        nat_mul(a3624, a3625)?
                    };
                    nat_mul(a3626, a3627)?
                }))
            } else {
                Ok(Adt10::C1)
            }
        }
    }
}

pub fn f154(v0: u64) -> u64 {
    let m3628 = f94(v0);
    match m3628 {
        Adt14::C0 => {
            0u64
        }
        Adt14::C1(v1) => {
            let a3630 = match v1.clone() {
                Adt15::C0(_, h3629, _) => {
                    h3629
                }
            };
            let a3631 = 1u64;
            if if nat_eq(a3630, a3631) {
                let a3633 = match v1.clone() {
                    Adt15::C0(_, _, h3632) => {
                        h3632
                    }
                };
                let a3634 = 1u64;
                if if if nat_eq(a3633, a3634) {
                    true
                } else {
                    let a3636 = match v1.clone() {
                        Adt15::C0(_, _, h3635) => {
                            h3635
                        }
                    };
                    let a3637 = 2u64;
                    nat_eq(a3636, a3637)
                } {
                    true
                } else {
                    let a3639 = match v1.clone() {
                        Adt15::C0(_, _, h3638) => {
                            h3638
                        }
                    };
                    let a3640 = 4u64;
                    nat_eq(a3639, a3640)
                } {
                    true
                } else {
                    let a3642 = match v1.clone() {
                        Adt15::C0(_, _, h3641) => {
                            h3641
                        }
                    };
                    let a3643 = 8u64;
                    nat_eq(a3642, a3643)
                }
            } else {
                false
            } {
                match v1 {
                    Adt15::C0(_, _, h3644) => {
                        h3644
                    }
                }
            } else {
                0u64
            }
        }
    }
}

pub fn f155(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let a3649 = {
        let a3647 = {
            let a3645 = v0;
            let a3646 = v1;
            nat_sub(a3645, a3646)
        };
        let a3648 = 1u64;
        nat_sub(a3647, a3648)
    };
    let a3650 = v2;
    nat_add(a3649, a3650)
}

pub fn f156(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64, v7: u64, v8: u64, v9: u64, v10: u64, v11: u64) -> R<u64> {
    f108({
        let a3651 = v0;
        let a3652 = v4;
        let a3653 = 0u64;
        nat_rem(a3651, a3652, a3653)
    }, {
        let a3654 = v1;
        let a3655 = v5;
        let a3656 = 0u64;
        nat_rem(a3654, a3655, a3656)
    }, {
        let a3657 = v2;
        let a3658 = v6;
        let a3659 = 0u64;
        nat_rem(a3657, a3658, a3659)
    }, {
        let a3660 = v3;
        let a3661 = v7;
        let a3662 = 0u64;
        nat_rem(a3660, a3661, a3662)
    }, v8, v9, v10, v11)
}

pub fn f157(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let a3663 = v1;
    let a3664 = 2147483648u64;
    if nat_lt(a3663, a3664) {
        let a3675 = {
            let a3673 = {
                let a3668 = {
                    let a3665 = v0;
                    let a3666 = v2;
                    let a3667 = 0u64;
                    nat_rem(a3665, a3666, a3667)
                };
                let a3669 = v2;
                nat_add(a3668, a3669)?
            };
            let a3674 = {
                let a3670 = v1;
                let a3671 = v2;
                let a3672 = 0u64;
                nat_rem(a3670, a3671, a3672)
            };
            nat_sub(a3673, a3674)
        };
        let a3676 = v2;
        let a3677 = 0u64;
        Ok(nat_rem(a3675, a3676, a3677))
    } else {
        let a3680 = {
            let a3678 = v0;
            let a3679 = f109(v1);
            nat_add(a3678, a3679)?
        };
        let a3681 = v2;
        let a3682 = 0u64;
        Ok(nat_rem(a3680, a3681, a3682))
    }
}

pub fn f158(v0: u64, v1: u64, v2: u64) -> R<bool> {
    let a3685 = f110(v1);
    let a3686 = {
        let a3683 = v0;
        let a3684 = f109(v1);
        nat_add(a3683, a3684)?
    };
    if nat_le(a3685, a3686) {
        let a3687 = f159(v0, v1)?;
        let a3688 = v2;
        Ok(nat_lt(a3687, a3688))
    } else {
        Ok(false)
    }
}

pub fn f159(v0: u64, v1: u64) -> R<u64> {
    let a3691 = {
        let a3689 = v0;
        let a3690 = f109(v1);
        nat_add(a3689, a3690)?
    };
    let a3692 = f110(v1);
    Ok(nat_sub(a3691, a3692))
}

pub fn f160(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let a3693 = v0;
    let a3694 = v1;
    if nat_lt(a3693, a3694) {
        let a3695 = v1;
        let a3696 = v0;
        Ok(nat_sub(a3695, a3696))
    } else {
        let a3699 = v0;
        let a3700 = {
            let a3697 = v1;
            let a3698 = v2;
            nat_add(a3697, a3698)?
        };
        if nat_lt(a3699, a3700) {
            let a3701 = v0;
            let a3702 = v1;
            Ok(nat_sub(a3701, a3702))
        } else {
            let a3709 = {
                let a3705 = 2u64;
                let a3706 = {
                    let a3703 = v2;
                    let a3704 = 1u64;
                    nat_sub(a3703, a3704)
                };
                nat_mul(a3705, a3706)?
            };
            let a3710 = {
                let a3707 = v0;
                let a3708 = v1;
                nat_sub(a3707, a3708)
            };
            Ok(nat_sub(a3709, a3710))
        }
    }
}

pub fn f161(v0: u64, v1: Adt3, v2: u64) -> R<u64> {
    let a3713 = {
        let a3711 = v2;
        let a3712 = f176(v1.clone(), f60(v0, v1.clone(), 0u64), f60(v0, v1.clone(), 1u64), f60(v0, v1.clone(), 2u64), f60(v0, v1.clone(), 3u64), false)?;
        nat_add(a3711, a3712)?
    };
    let a3714 = f176(v1.clone(), f60(v0, v1.clone(), 0u64), f60(v0, v1.clone(), 1u64), f60(v0, v1.clone(), 2u64), f60(v0, v1, 3u64), true)?;
    Ok(nat_sub(a3713, a3714))
}

pub fn f162(v0: u64, v1: u64, v2: u64) -> bool {
    let a3715 = v0;
    let a3716 = 0u64;
    if nat_eq(a3715, a3716) {
        let a3717 = v2;
        let a3718 = v1;
        nat_le(a3717, a3718)
    } else {
        let a3719 = v0;
        let a3720 = 1u64;
        if nat_eq(a3719, a3720) {
            let a3721 = v2;
            let a3722 = v1;
            nat_lt(a3721, a3722)
        } else {
            let a3723 = v0;
            let a3724 = 2u64;
            if nat_eq(a3723, a3724) {
                let a3725 = v1;
                let a3726 = v2;
                nat_le(a3725, a3726)
            } else {
                let a3727 = v0;
                let a3728 = 3u64;
                if nat_eq(a3727, a3728) {
                    let a3729 = v1;
                    let a3730 = v2;
                    nat_lt(a3729, a3730)
                } else {
                    let a3731 = v1;
                    let a3732 = v2;
                    nat_eq(a3731, a3732)
                }
            }
        }
    }
}

pub fn f163(v0: u64, v1: Adt3, v2: u64) -> R<Adt9> {
    let a3733 = v0;
    let a3734 = 0u64;
    if nat_eq(a3733, a3734) {
        Ok(f171(0u64))
    } else {
        let a3737 = {
            let a3735 = v0;
            let a3736 = 1u64;
            nat_add(a3735, a3736)?
        };
        let a3738 = f106(v1.clone(), 13u64, v2)?;
        if nat_eq(a3737, a3738) {
            Ok(Adt9::C4({
                let a3739 = v2;
                let a3740 = 0u64;
                if nat_eq(a3739, a3740) {
                    2u64
                } else {
                    7u64
                }
            }, 1u64, 0u64, 2u64, 0u64, 3u64, 0u64))
        } else {
            let v3: u64 = {
                let a3741 = v0;
                let a3742 = 1u64;
                nat_sub(a3741, a3742)
            };
            if f151(v1.clone(), v2) {
                let v4: u64 = f152(match v1.clone() {
                    Adt3::C0(h3743, _, _, _, _, _, _, _) => {
                        h3743
                    }
                }, false)?;
                let v5: u64 = {
                    let a3744 = v3;
                    let a3745 = v4;
                    let a3746 = 0u64;
                    nat_quot(a3744, a3745, a3746)
                };
                let v6: u64 = {
                    let a3747 = v3;
                    let a3748 = v4;
                    let a3749 = 0u64;
                    nat_rem(a3747, a3748, a3749)
                };
                let a3752 = {
                    let a3750 = v6;
                    let a3751 = 1u64;
                    nat_add(a3750, a3751)?
                };
                let a3753 = v4;
                if nat_eq(a3752, a3753) {
                    Ok(Adt9::C4({
                        let a3754 = v2;
                        let a3755 = 0u64;
                        if nat_eq(a3754, a3755) {
                            0u64
                        } else {
                            4u64
                        }
                    }, 1u64, 0u64, 1u64, 1u64, 1u64, 0u64))
                } else {
                    f164(v6, v5, v1, v2)
                }
            } else {
                f179({
                    let a3756 = v3;
                    let a3757 = 2u64;
                    let a3758 = 0u64;
                    nat_quot(a3756, a3757, a3758)
                }, {
                    let a3759 = v3;
                    let a3760 = 2u64;
                    let a3761 = 0u64;
                    nat_rem(a3759, a3760, a3761)
                }, v1, v2, 0u64)
            }
        }
    }
}

pub fn f164(v0: u64, v1: u64, v2: Adt3, v3: u64) -> R<Adt9> {
    let a3762 = v0;
    let a3763 = 0u64;
    if nat_eq(a3762, a3763) {
        Ok(f171(1u64))
    } else {
        f179({
            let a3772 = {
                let a3765 = v1;
                let a3766 = match v2.clone() {
                    Adt3::C0(h3764, _, _, _, _, _, _, _) => {
                        h3764
                    }
                };
                nat_mul(a3765, a3766)?
            };
            let a3773 = {
                let a3769 = {
                    let a3767 = v0;
                    let a3768 = 1u64;
                    nat_sub(a3767, a3768)
                };
                let a3770 = 2u64;
                let a3771 = 0u64;
                nat_quot(a3769, a3770, a3771)
            };
            nat_add(a3772, a3773)?
        }, {
            let a3776 = {
                let a3774 = v0;
                let a3775 = 1u64;
                nat_sub(a3774, a3775)
            };
            let a3777 = 2u64;
            let a3778 = 0u64;
            nat_rem(a3776, a3777, a3778)
        }, v2, v3, 1u64)
    }
}

pub fn f165(v0: u64, v1: u64) -> R<u64> {
    let a3779 = v0;
    let a3780 = f80(v1)?;
    let a3781 = 0u64;
    Ok(nat_rem(a3779, a3780, a3781))
}

pub fn f166(v0: u64, v1: u64) -> R<u64> {
    let a3782 = 4u64;
    let a3783 = f180(v0, v1)?;
    nat_mul(a3782, a3783)
}

pub fn f167(v0: u64, v1: u64) -> R<u64> {
    let a3784 = 4u64;
    let a3785 = f181(v0, v1)?;
    nat_mul(a3784, a3785)
}

pub fn f168(v0: u64, v1: u64) -> R<bool> {
    let a3791 = {
        let a3788 = {
            let a3786 = f165(v0, v1)?;
            let a3787 = 2u64;
            nat_sub(a3786, a3787)
        };
        let a3789 = 2u64;
        let a3790 = 0u64;
        nat_rem(a3788, a3789, a3790)
    };
    let a3792 = 0u64;
    Ok(nat_eq(a3791, a3792))
}

pub fn f169(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64, v7: u64, v8: u64, v9: u64, v10: u64, v11: u64, v12: u64) -> R<Option<u64>> {
    let m3793 = f94(v0);
    match m3793 {
        Adt14::C0 => {
            Ok(None::<u64>)
        }
        Adt14::C1(v13) => {
            if if if if if f182(v0) {
                let a3794 = v1;
                let a3795 = v7;
                nat_lt(a3794, a3795)
            } else {
                false
            } {
                f133(v8, v2)
            } else {
                false
            } {
                let a3796 = v3;
                let a3797 = f183(v4, v5, v6)?;
                nat_lt(a3796, a3797)
            } else {
                false
            } {
                let a3802 = {
                    let a3799 = v7;
                    let a3800 = match v13.clone() {
                        Adt15::C0(_, h3798, _) => {
                            h3798
                        }
                    };
                    let a3801 = 0u64;
                    nat_rem(a3799, a3800, a3801)
                };
                let a3803 = 0u64;
                nat_eq(a3802, a3803)
            } else {
                false
            } {
                Ok(Some({
                    let a3832 = {
                        let a3822 = {
                            let a3812 = {
                                let a3808 = {
                                    let a3805 = v1;
                                    let a3806 = match v13 {
                                        Adt15::C0(_, h3804, _) => {
                                            h3804
                                        }
                                    };
                                    let a3807 = 0u64;
                                    nat_quot(a3805, a3806, a3807)
                                };
                                let a3809 = v9;
                                nat_mul(a3808, a3809)?
                            };
                            let a3813 = {
                                let a3810 = v2;
                                let a3811 = v10;
                                nat_mul(a3810, a3811)?
                            };
                            nat_add(a3812, a3813)?
                        };
                        let a3823 = {
                            let a3820 = {
                                let a3817 = {
                                    let a3814 = v3;
                                    let a3815 = v4;
                                    let a3816 = 0u64;
                                    nat_quot(a3814, a3815, a3816)
                                };
                                let a3818 = v5;
                                let a3819 = 0u64;
                                nat_rem(a3817, a3818, a3819)
                            };
                            let a3821 = v11;
                            nat_mul(a3820, a3821)?
                        };
                        nat_add(a3822, a3823)?
                    };
                    let a3833 = {
                        let a3830 = {
                            let a3827 = {
                                let a3824 = v3;
                                let a3825 = v4;
                                let a3826 = 0u64;
                                nat_quot(a3824, a3825, a3826)
                            };
                            let a3828 = v5;
                            let a3829 = 0u64;
                            nat_quot(a3827, a3828, a3829)
                        };
                        let a3831 = v12;
                        nat_mul(a3830, a3831)?
                    };
                    nat_add(a3832, a3833)?
                }))
            } else {
                Ok(None::<u64>)
            }
        }
    }
}

pub fn f170(v0: u64) -> u64 {
    let a3834 = v0;
    let a3835 = 30u64;
    if nat_eq(a3834, a3835) {
        4u64
    } else {
        3u64
    }
}

pub fn f171(v0: u64) -> Adt9 {
    Adt9::C4(0u64, 2u64, 0u64, 2u64, 0u64, 1u64, v0)
}

pub fn f172(v0: u64) -> bool {
    let a3836 = 0u64;
    let a3837 = v0;
    if nat_lt(a3836, a3837) {
        let a3838 = v0;
        let a3839 = 2139095040u64;
        nat_le(a3838, a3839)
    } else {
        false
    }
}

pub fn f173(v0: u64) -> u64 {
    let a3840 = v0;
    let a3841 = 40u64;
    if nat_lt(a3840, a3841) {
        let a3842 = v0;
        let a3843 = 20u64;
        if nat_lt(a3842, a3843) {
            let a3844 = v0;
            let a3845 = 10u64;
            if nat_lt(a3844, a3845) {
                let a3846 = v0;
                let a3847 = 5u64;
                if nat_lt(a3846, a3847) {
                    let a3848 = v0;
                    let a3849 = 2u64;
                    if nat_lt(a3848, a3849) {
                        let a3850 = v0;
                        let a3851 = 1u64;
                        if nat_lt(a3850, a3851) {
                            1u64
                        } else {
                            2u64
                        }
                    } else {
                        let a3852 = v0;
                        let a3853 = 3u64;
                        if nat_lt(a3852, a3853) {
                            4u64
                        } else {
                            let a3854 = v0;
                            let a3855 = 4u64;
                            if nat_lt(a3854, a3855) {
                                8u64
                            } else {
                                16u64
                            }
                        }
                    }
                } else {
                    let a3856 = v0;
                    let a3857 = 7u64;
                    if nat_lt(a3856, a3857) {
                        let a3858 = v0;
                        let a3859 = 6u64;
                        if nat_lt(a3858, a3859) {
                            32u64
                        } else {
                            64u64
                        }
                    } else {
                        let a3860 = v0;
                        let a3861 = 8u64;
                        if nat_lt(a3860, a3861) {
                            128u64
                        } else {
                            let a3862 = v0;
                            let a3863 = 9u64;
                            if nat_lt(a3862, a3863) {
                                256u64
                            } else {
                                512u64
                            }
                        }
                    }
                }
            } else {
                let a3864 = v0;
                let a3865 = 15u64;
                if nat_lt(a3864, a3865) {
                    let a3866 = v0;
                    let a3867 = 12u64;
                    if nat_lt(a3866, a3867) {
                        let a3868 = v0;
                        let a3869 = 11u64;
                        if nat_lt(a3868, a3869) {
                            1024u64
                        } else {
                            2048u64
                        }
                    } else {
                        let a3870 = v0;
                        let a3871 = 13u64;
                        if nat_lt(a3870, a3871) {
                            4096u64
                        } else {
                            let a3872 = v0;
                            let a3873 = 14u64;
                            if nat_lt(a3872, a3873) {
                                8192u64
                            } else {
                                16384u64
                            }
                        }
                    }
                } else {
                    let a3874 = v0;
                    let a3875 = 17u64;
                    if nat_lt(a3874, a3875) {
                        let a3876 = v0;
                        let a3877 = 16u64;
                        if nat_lt(a3876, a3877) {
                            32768u64
                        } else {
                            65536u64
                        }
                    } else {
                        let a3878 = v0;
                        let a3879 = 18u64;
                        if nat_lt(a3878, a3879) {
                            131072u64
                        } else {
                            let a3880 = v0;
                            let a3881 = 19u64;
                            if nat_lt(a3880, a3881) {
                                262144u64
                            } else {
                                524288u64
                            }
                        }
                    }
                }
            }
        } else {
            let a3882 = v0;
            let a3883 = 30u64;
            if nat_lt(a3882, a3883) {
                let a3884 = v0;
                let a3885 = 25u64;
                if nat_lt(a3884, a3885) {
                    let a3886 = v0;
                    let a3887 = 22u64;
                    if nat_lt(a3886, a3887) {
                        let a3888 = v0;
                        let a3889 = 21u64;
                        if nat_lt(a3888, a3889) {
                            1048576u64
                        } else {
                            2097152u64
                        }
                    } else {
                        let a3890 = v0;
                        let a3891 = 23u64;
                        if nat_lt(a3890, a3891) {
                            4194304u64
                        } else {
                            let a3892 = v0;
                            let a3893 = 24u64;
                            if nat_lt(a3892, a3893) {
                                8388608u64
                            } else {
                                16777216u64
                            }
                        }
                    }
                } else {
                    let a3894 = v0;
                    let a3895 = 27u64;
                    if nat_lt(a3894, a3895) {
                        let a3896 = v0;
                        let a3897 = 26u64;
                        if nat_lt(a3896, a3897) {
                            33554432u64
                        } else {
                            67108864u64
                        }
                    } else {
                        let a3898 = v0;
                        let a3899 = 28u64;
                        if nat_lt(a3898, a3899) {
                            134217728u64
                        } else {
                            let a3900 = v0;
                            let a3901 = 29u64;
                            if nat_lt(a3900, a3901) {
                                268435456u64
                            } else {
                                536870912u64
                            }
                        }
                    }
                }
            } else {
                let a3902 = v0;
                let a3903 = 35u64;
                if nat_lt(a3902, a3903) {
                    let a3904 = v0;
                    let a3905 = 32u64;
                    if nat_lt(a3904, a3905) {
                        let a3906 = v0;
                        let a3907 = 31u64;
                        if nat_lt(a3906, a3907) {
                            1073741824u64
                        } else {
                            2147483648u64
                        }
                    } else {
                        let a3908 = v0;
                        let a3909 = 33u64;
                        if nat_lt(a3908, a3909) {
                            4294967296u64
                        } else {
                            let a3910 = v0;
                            let a3911 = 34u64;
                            if nat_lt(a3910, a3911) {
                                8589934592u64
                            } else {
                                17179869184u64
                            }
                        }
                    }
                } else {
                    let a3912 = v0;
                    let a3913 = 37u64;
                    if nat_lt(a3912, a3913) {
                        let a3914 = v0;
                        let a3915 = 36u64;
                        if nat_lt(a3914, a3915) {
                            34359738368u64
                        } else {
                            68719476736u64
                        }
                    } else {
                        let a3916 = v0;
                        let a3917 = 38u64;
                        if nat_lt(a3916, a3917) {
                            137438953472u64
                        } else {
                            let a3918 = v0;
                            let a3919 = 39u64;
                            if nat_lt(a3918, a3919) {
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

pub fn f174(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64) -> bool {
    if f148(v1, v2, v3, v4) {
        f184(v0)
    } else {
        f182(v0)
    }
}

pub fn f175(v0: u64) -> u64 {
    let a3920 = v0;
    let a3921 = 26u64;
    if nat_eq(a3920, a3921) {
        26u64
    } else {
        0u64
    }
}

pub fn f176(v0: Adt3, v1: u64, v2: u64, v3: u64, v4: u64, v5: bool) -> R<u64> {
    let a3942 = {
        let a3936 = {
            let a3930 = {
                let a3924 = v1;
                let a3925 = if v5 {
                    f109(match v0.clone() {
                        Adt3::C0(_, _, _, _, h3922, _, _, _) => {
                            h3922
                        }
                    })
                } else {
                    f110(match v0.clone() {
                        Adt3::C0(_, _, _, _, h3923, _, _, _) => {
                            h3923
                        }
                    })
                };
                nat_mul(a3924, a3925)?
            };
            let a3931 = {
                let a3928 = v2;
                let a3929 = if v5 {
                    f109(match v0.clone() {
                        Adt3::C0(_, _, _, _, _, h3926, _, _) => {
                            h3926
                        }
                    })
                } else {
                    f110(match v0.clone() {
                        Adt3::C0(_, _, _, _, _, h3927, _, _) => {
                            h3927
                        }
                    })
                };
                nat_mul(a3928, a3929)?
            };
            nat_add(a3930, a3931)?
        };
        let a3937 = {
            let a3934 = v3;
            let a3935 = if v5 {
                f109(match v0.clone() {
                    Adt3::C0(_, _, _, _, _, _, h3932, _) => {
                        h3932
                    }
                })
            } else {
                f110(match v0.clone() {
                    Adt3::C0(_, _, _, _, _, _, h3933, _) => {
                        h3933
                    }
                })
            };
            nat_mul(a3934, a3935)?
        };
        nat_add(a3936, a3937)?
    };
    let a3943 = {
        let a3940 = v4;
        let a3941 = if v5 {
            f109(match v0 {
                Adt3::C0(_, _, _, _, _, _, _, h3938) => {
                    h3938
                }
            })
        } else {
            f110(match v0 {
                Adt3::C0(_, _, _, _, _, _, _, h3939) => {
                    h3939
                }
            })
        };
        nat_mul(a3940, a3941)?
    };
    nat_add(a3942, a3943)
}

pub fn f177(v0: u64, v1: u64) -> Option<u64> {
    let a3944 = v0;
    let a3945 = 2u64;
    if nat_eq(a3944, a3945) {
        Some(0u64)
    } else {
        let a3946 = v0;
        let a3947 = 5u64;
        if nat_eq(a3946, a3947) {
            Some(1u64)
        } else {
            let a3948 = v0;
            let a3949 = 6u64;
            if nat_eq(a3948, a3949) {
                Some(2u64)
            } else {
                let a3950 = v0;
                let a3951 = 7u64;
                if nat_eq(a3950, a3951) {
                    Some(3u64)
                } else {
                    let a3952 = v0;
                    let a3953 = 9u64;
                    if nat_eq(a3952, a3953) {
                        Some(4u64)
                    } else {
                        let a3954 = v0;
                        let a3955 = 8u64;
                        if nat_eq(a3954, a3955) {
                            Some(5u64)
                        } else {
                            let a3956 = v0;
                            let a3957 = 89u64;
                            if nat_eq(a3956, a3957) {
                                let a3958 = v1;
                                let a3959 = 2u64;
                                if nat_eq(a3958, a3959) {
                                    Some(6u64)
                                } else {
                                    let a3960 = v1;
                                    let a3961 = 0u64;
                                    if nat_eq(a3960, a3961) {
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

pub fn f178(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64, v7: u64, v8: u64, v9: u64, v10: u64, v11: u64, v12: u64, v13: u64, v14: u64, v15: u64) -> R<Adt16> {
    let a3962 = v0;
    let a3963 = 30u64;
    if nat_eq(a3962, a3963) {
        let a3964 = v2;
        let a3965 = 0u64;
        if if if nat_eq(a3964, a3965) {
            true
        } else {
            let a3966 = v2;
            let a3967 = 1u64;
            nat_eq(a3966, a3967)
        } {
            let a3968 = v1;
            let a3969 = 0u64;
            if nat_eq(a3968, a3969) {
                true
            } else {
                f185(v1, v4)
            }
        } else {
            false
        } {
            f186(28u64, 0u64, 0u64, v3, v5, v4, v6, v7, v9, v8, v10, v11, v12, v13, v14, v15)
        } else {
            Ok(Adt16::C0)
        }
    } else {
        if if if f187(v1) {
            f187(v2)
        } else {
            false
        } {
            let a3970 = v1;
            let a3971 = 0u64;
            if if nat_eq(a3970, a3971) {
                true
            } else {
                let a3972 = v2;
                let a3973 = 0u64;
                nat_eq(a3972, a3973)
            } {
                true
            } else {
                let a3974 = v1;
                let a3975 = v2;
                nat_eq(a3974, a3975)
            }
        } else {
            false
        } {
            f186(v0, 0u64, 0u64, v3, v4, v5, v6, v7, v8, v9, v10, v11, v12, v13, v14, v15)
        } else {
            Ok(Adt16::C0)
        }
    }
}

pub fn f179(v0: u64, v1: u64, v2: Adt3, v3: u64, v4: u64) -> R<Adt9> {
    let a3976 = v1;
    let a3977 = 0u64;
    if nat_eq(a3976, a3977) {
        Ok(Adt9::C4({
            let a3978 = v3;
            let a3979 = 0u64;
            if nat_eq(a3978, a3979) {
                1u64
            } else {
                6u64
            }
        }, 0u64, f116(v0, v2)?, 2u64, 0u64, 1u64, 2u64))
    } else {
        Ok(Adt9::C4({
            let a3980 = v3;
            let a3981 = 0u64;
            if nat_eq(a3980, a3981) {
                0u64
            } else {
                4u64
            }
        }, 1u64, v4, 1u64, 2u64, 1u64, v4))
    }
}

pub fn f180(v0: u64, v1: u64) -> R<u64> {
    let a3982 = v0;
    let a3983 = f80(v1)?;
    let a3984 = 0u64;
    Ok(nat_quot(a3982, a3983, a3984))
}

pub fn f181(v0: u64, v1: u64) -> R<u64> {
    let v2: u64 = f180(v0, v1)?;
    let v3: u64 = {
        let a3987 = {
            let a3985 = f165(v0, v1)?;
            let a3986 = 2u64;
            nat_sub(a3985, a3986)
        };
        let a3988 = 2u64;
        let a3989 = 0u64;
        nat_quot(a3987, a3988, a3989)
    };
    let v4: u64 = {
        let a3990 = v2;
        let a3991 = v1;
        nat_mul(a3990, a3991)?
    };
    if f168(v0, v1)? {
        let a3992 = v4;
        let a3993 = v3;
        nat_add(a3992, a3993)
    } else {
        let a4005 = {
            let a4001 = v4;
            let a4002 = {
                let a3999 = {
                    let a3994 = v1;
                    let a3995 = 1u64;
                    nat_sub(a3994, a3995)
                };
                let a4000 = {
                    let a3996 = v2;
                    let a3997 = v1;
                    let a3998 = 0u64;
                    nat_rem(a3996, a3997, a3998)
                };
                nat_mul(a3999, a4000)?
            };
            nat_sub(a4001, a4002)
        };
        let a4006 = {
            let a4003 = v3;
            let a4004 = v1;
            nat_mul(a4003, a4004)?
        };
        nat_add(a4005, a4006)
    }
}

pub fn f182(v0: u64) -> bool {
    let a4007 = v0;
    let a4008 = 0u64;
    if if if if if if if if if if if if if if if if if if if if if if if if if if if if if nat_eq(a4007, a4008) {
        true
    } else {
        let a4009 = v0;
        let a4010 = 1u64;
        nat_eq(a4009, a4010)
    } {
        true
    } else {
        let a4011 = v0;
        let a4012 = 2u64;
        nat_eq(a4011, a4012)
    } {
        true
    } else {
        let a4013 = v0;
        let a4014 = 3u64;
        nat_eq(a4013, a4014)
    } {
        true
    } else {
        let a4015 = v0;
        let a4016 = 6u64;
        nat_eq(a4015, a4016)
    } {
        true
    } else {
        let a4017 = v0;
        let a4018 = 7u64;
        nat_eq(a4017, a4018)
    } {
        true
    } else {
        let a4019 = v0;
        let a4020 = 8u64;
        nat_eq(a4019, a4020)
    } {
        true
    } else {
        let a4021 = v0;
        let a4022 = 10u64;
        nat_eq(a4021, a4022)
    } {
        true
    } else {
        let a4023 = v0;
        let a4024 = 11u64;
        nat_eq(a4023, a4024)
    } {
        true
    } else {
        let a4025 = v0;
        let a4026 = 12u64;
        nat_eq(a4025, a4026)
    } {
        true
    } else {
        let a4027 = v0;
        let a4028 = 13u64;
        nat_eq(a4027, a4028)
    } {
        true
    } else {
        let a4029 = v0;
        let a4030 = 14u64;
        nat_eq(a4029, a4030)
    } {
        true
    } else {
        let a4031 = v0;
        let a4032 = 16u64;
        nat_eq(a4031, a4032)
    } {
        true
    } else {
        let a4033 = v0;
        let a4034 = 17u64;
        nat_eq(a4033, a4034)
    } {
        true
    } else {
        let a4035 = v0;
        let a4036 = 18u64;
        nat_eq(a4035, a4036)
    } {
        true
    } else {
        let a4037 = v0;
        let a4038 = 19u64;
        nat_eq(a4037, a4038)
    } {
        true
    } else {
        let a4039 = v0;
        let a4040 = 20u64;
        nat_eq(a4039, a4040)
    } {
        true
    } else {
        let a4041 = v0;
        let a4042 = 21u64;
        nat_eq(a4041, a4042)
    } {
        true
    } else {
        let a4043 = v0;
        let a4044 = 22u64;
        nat_eq(a4043, a4044)
    } {
        true
    } else {
        let a4045 = v0;
        let a4046 = 23u64;
        nat_eq(a4045, a4046)
    } {
        true
    } else {
        let a4047 = v0;
        let a4048 = 26u64;
        nat_eq(a4047, a4048)
    } {
        true
    } else {
        let a4049 = v0;
        let a4050 = 29u64;
        nat_eq(a4049, a4050)
    } {
        true
    } else {
        let a4051 = v0;
        let a4052 = 30u64;
        nat_eq(a4051, a4052)
    } {
        true
    } else {
        let a4053 = v0;
        let a4054 = 34u64;
        nat_eq(a4053, a4054)
    } {
        true
    } else {
        let a4055 = v0;
        let a4056 = 35u64;
        nat_eq(a4055, a4056)
    } {
        true
    } else {
        let a4057 = v0;
        let a4058 = 39u64;
        nat_eq(a4057, a4058)
    } {
        true
    } else {
        let a4059 = v0;
        let a4060 = 40u64;
        nat_eq(a4059, a4060)
    } {
        true
    } else {
        let a4061 = v0;
        let a4062 = 41u64;
        nat_eq(a4061, a4062)
    } {
        true
    } else {
        let a4063 = v0;
        let a4064 = 42u64;
        nat_eq(a4063, a4064)
    } {
        true
    } else {
        let a4065 = v0;
        let a4066 = 43u64;
        nat_eq(a4065, a4066)
    }
}

pub fn f183(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let a4067 = v0;
    let a4068 = 0u64;
    if if if nat_eq(a4067, a4068) {
        true
    } else {
        let a4069 = v1;
        let a4070 = 0u64;
        nat_eq(a4069, a4070)
    } {
        true
    } else {
        let a4071 = v2;
        let a4072 = 0u64;
        nat_eq(a4071, a4072)
    } {
        Ok(0u64)
    } else {
        let a4075 = {
            let a4073 = v0;
            let a4074 = v1;
            nat_mul(a4073, a4074)?
        };
        let a4076 = v2;
        nat_mul(a4075, a4076)
    }
}

pub fn f184(v0: u64) -> bool {
    let m4077 = f94(v0);
    match m4077 {
        Adt14::C0 => {
            false
        }
        Adt14::C1(_) => {
            true
        }
    }
}

pub fn f185(v0: u64, v1: u64) -> bool {
    let a4078 = v0;
    let a4079 = 1u64;
    if nat_eq(a4078, a4079) {
        true
    } else {
        let v2: u64 = f188(v0);
        let a4080 = 0u64;
        let a4081 = v2;
        if nat_lt(a4080, a4081) {
            let a4085 = {
                let a4082 = v1;
                let a4083 = v2;
                let a4084 = 1u64;
                nat_rem(a4082, a4083, a4084)
            };
            let a4086 = 0u64;
            nat_eq(a4085, a4086)
        } else {
            false
        }
    }
}

pub fn f186(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64, v7: u64, v8: u64, v9: u64, v10: u64, v11: u64, v12: u64, v13: u64, v14: u64, v15: u64) -> R<Adt16> {
    let a4087 = v0;
    let a4088 = 28u64;
    if if if if nat_eq(a4087, a4088) {
        let a4089 = v1;
        let a4090 = 0u64;
        nat_eq(a4089, a4090)
    } else {
        false
    } {
        let a4091 = v2;
        let a4092 = 0u64;
        nat_eq(a4091, a4092)
    } else {
        false
    } {
        let a4093 = v3;
        let a4094 = 0u64;
        nat_eq(a4093, a4094)
    } else {
        false
    } {
        let a4095 = v4;
        let a4096 = v8;
        if if if if if if if if nat_eq(a4095, a4096) {
            let a4097 = v5;
            let a4098 = v12;
            nat_eq(a4097, a4098)
        } else {
            false
        } {
            let a4099 = v9;
            let a4100 = v13;
            nat_eq(a4099, a4100)
        } else {
            false
        } {
            let a4101 = v10;
            let a4102 = v14;
            nat_eq(a4101, a4102)
        } else {
            false
        } {
            let a4103 = v11;
            let a4104 = v15;
            nat_eq(a4103, a4104)
        } else {
            false
        } {
            let a4105 = 0u64;
            let a4106 = v6;
            nat_lt(a4105, a4106)
        } else {
            false
        } {
            let a4107 = 0u64;
            let a4108 = v7;
            nat_lt(a4107, a4108)
        } else {
            false
        } {
            let a4112 = {
                let a4109 = v10;
                let a4110 = v6;
                let a4111 = 1u64;
                nat_rem(a4109, a4110, a4111)
            };
            let a4113 = 0u64;
            if nat_eq(a4112, a4113) {
                let a4117 = {
                    let a4114 = v11;
                    let a4115 = v7;
                    let a4116 = 1u64;
                    nat_rem(a4114, a4115, a4116)
                };
                let a4118 = 0u64;
                nat_eq(a4117, a4118)
            } else {
                false
            }
        } else {
            false
        } {
            Ok(Adt16::C2(v5, v4, v9, v10, v11, {
                let a4119 = v10;
                let a4120 = v6;
                let a4121 = 0u64;
                nat_quot(a4119, a4120, a4121)
            }, {
                let a4122 = v11;
                let a4123 = v7;
                let a4124 = 0u64;
                nat_quot(a4122, a4123, a4124)
            }, {
                let a4125 = v5;
                let a4126 = v4;
                nat_mul(a4125, a4126)?
            }, {
                let a4127 = v4;
                let a4128 = v9;
                nat_mul(a4127, a4128)?
            }, {
                let a4129 = v5;
                let a4130 = v9;
                nat_mul(a4129, a4130)?
            }))
        } else {
            Ok(Adt16::C1)
        }
    } else {
        Ok(Adt16::C0)
    }
}

pub fn f187(v0: u64) -> bool {
    let a4131 = v0;
    let a4132 = 0u64;
    if if nat_eq(a4131, a4132) {
        true
    } else {
        let a4133 = v0;
        let a4134 = 1u64;
        nat_eq(a4133, a4134)
    } {
        true
    } else {
        let a4135 = v0;
        let a4136 = 30u64;
        nat_eq(a4135, a4136)
    }
}

pub fn f188(v0: u64) -> u64 {
    if f182(v0) {
        let m4137 = f94(v0);
        match m4137 {
            Adt14::C0 => {
                0u64
            }
            Adt14::C1(v1) => {
                let a4139 = 1u64;
                let a4140 = match v1.clone() {
                    Adt15::C0(_, h4138, _) => {
                        h4138
                    }
                };
                if if nat_lt(a4139, a4140) {
                    true
                } else {
                    let a4141 = v0;
                    let a4142 = 43u64;
                    nat_eq(a4141, a4142)
                } {
                    match v1 {
                        Adt15::C0(_, h4143, _) => {
                            h4143
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
